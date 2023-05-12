use crate::execution::deployer_execute;
use crate::msg::{
    CustodyContractInfo, ExecuteMsg, ForwarderExecuteMsg, InstantiateMsg, MigrateMsg, QueryMsg,
    SudoMsg,
};
use crate::query::deployer_query;
use crate::state::{CONTRACT_REGISTRY, FORWARDER_CONTRACT_MAPPING, OWNER};
#[cfg(not(feature = "library"))]
use cosmwasm_std::{entry_point, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult};
use cosmwasm_std::{to_binary, Coin, CosmosMsg, ReplyOn, StdError, SubMsg, WasmMsg};
use cw2::set_contract_version;
use router_wasm_bindings::ethabi::{decode, ParamType};
use router_wasm_bindings::types::ChainType;
use router_wasm_bindings::utils::convert_address_from_bytes_to_string;
use router_wasm_bindings::{RouterMsg, RouterQuery};

// version info for migration info
const CONTRACT_NAME: &str = "router-crosschain_deployer";
const CONTRACT_VERSION: &str = "0.1.2";
pub const FORWARDER_REPLY_ID: u64 = 1;
pub const CREATE_I_SEND_REQUEST: u64 = 2;

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut<RouterQuery>,
    _env: Env,
    _info: MessageInfo,
    msg: InstantiateMsg,
) -> StdResult<Response> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
    OWNER.save(deps.storage, &msg.owner)?;
    Ok(Response::new().add_attribute("action", "hello_router_init"))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn sudo(deps: DepsMut<RouterQuery>, env: Env, msg: SudoMsg) -> StdResult<Response<RouterMsg>> {
    match msg {
        SudoMsg::HandleIReceive {
            request_sender,
            src_chain_id,
            request_identifier,
            payload,
        } => handle_sudo_request(
            deps,
            env,
            request_sender,
            src_chain_id,
            request_identifier,
            payload,
        ),
        SudoMsg::HandleIAck {
            request_identifier,
            exec_flag,
            exec_data,
            refund_amount,
        } => handle_sudo_ack(
            deps,
            env,
            request_identifier,
            exec_flag,
            exec_data,
            refund_amount,
        ),
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut<RouterQuery>,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> StdResult<Response<RouterMsg>> {
    deployer_execute(deps, env, info, msg)
}

pub fn handle_sudo_request(
    _deps: DepsMut<RouterQuery>,
    _env: Env,
    request_sender: String,
    src_chain_id: String,
    request_identifier: u64,
    _payload: Binary,
) -> StdResult<Response<RouterMsg>> {
    let res = Response::new()
        .add_attribute("sender", request_sender)
        .add_attribute("request_identifier", request_identifier.to_string())
        .add_attribute("src_chain_id", src_chain_id);
    Ok(res)
}

fn handle_sudo_ack(
    deps: DepsMut<RouterQuery>,
    _env: Env,
    request_identifier: u64,
    exec_flag: bool,
    exec_data: Binary,
    refund_amount: Coin,
) -> StdResult<Response<RouterMsg>> {
    let execution_msg: String = format!(
        "request_identifier {:?}, refund_amount {:?}, exec_flag {:?}, exec_data {:?}",
        request_identifier, refund_amount, exec_flag, exec_data
    );
    deps.api.debug(&execution_msg);

    let token = decode(
        &[
            ParamType::Uint(64),
            ParamType::FixedBytes(32),
            ParamType::FixedBytes(32),
            ParamType::Address,
        ],
        &exec_data.0,
    )
    .unwrap();
    let chain_id_int: u64 = token[0].clone().into_uint().unwrap().as_u64();
    let chain_id: String = chain_id_int.to_string();
    let digest: String = convert_address_from_bytes_to_string(
        &token[1].clone().into_fixed_bytes().unwrap(),
        ChainType::ChainTypeEvm.get_chain_code(),
    )
    .unwrap();
    let salt: String = convert_address_from_bytes_to_string(
        &token[2].clone().into_fixed_bytes().unwrap(),
        ChainType::ChainTypeEvm.get_chain_code(),
    )
    .unwrap();
    let addr: String = convert_address_from_bytes_to_string(
        &token[3].clone().into_address().unwrap().0,
        ChainType::ChainTypeEvm.get_chain_code(),
    )
    .unwrap();

    let registry_info: (bool, String, String) =
        CONTRACT_REGISTRY.load(deps.storage, (&digest, &salt, &chain_id))?;
    CONTRACT_REGISTRY.save(
        deps.storage,
        (&digest, &salt, &chain_id),
        &(true, addr.clone(), registry_info.2.clone()),
    )?;
    let forwader_addr: String =
        FORWARDER_CONTRACT_MAPPING.load(deps.storage, &request_identifier.to_string())?;
    let exec_msg: CosmosMsg<RouterMsg> = CosmosMsg::Wasm(WasmMsg::Execute {
        contract_addr: forwader_addr,
        funds: vec![],
        msg: to_binary(&ForwarderExecuteMsg::SetCustodyContracts {
            custody_contracts: vec![CustodyContractInfo {
                chain_id,
                address: addr,
            }],
        })?,
    });
    let sub_msg: SubMsg<RouterMsg> = SubMsg {
        gas_limit: None,
        id: FORWARDER_REPLY_ID,
        reply_on: ReplyOn::Never,
        msg: exec_msg,
    };
    let res = Response::new()
        .add_submessage(sub_msg)
        .add_attribute("request_identifier", request_identifier.to_string());
    Ok(res)
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn migrate(deps: DepsMut<RouterQuery>, _env: Env, _msg: MigrateMsg) -> StdResult<Response> {
    let ver = cw2::get_contract_version(deps.storage)?;
    // ensure we are migrating from an allowed contract
    if ver.contract != CONTRACT_NAME.to_string() {
        return Err(StdError::generic_err("Can only upgrade from same type").into());
    }
    // note: better to do proper semver compare, but string compare *usually* works
    if ver.version >= CONTRACT_VERSION.to_string() {
        return Err(StdError::generic_err("Cannot upgrade from a newer version").into());
    }

    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
    Ok(Response::default())
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps<RouterQuery>, env: Env, msg: QueryMsg) -> StdResult<Binary> {
    deployer_query(deps, env, msg)
}

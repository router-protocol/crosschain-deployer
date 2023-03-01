use std::time::{SystemTime, UNIX_EPOCH};

use crate::msg::{ExecuteMsg, InstantiateMsg, MigrateMsg, QueryMsg, SudoMsg};
use crate::state::{BRIDGE_CONTRACT, CONTRACT_REGISTRY, DATA, OWNER };
#[cfg(not(feature = "library"))]
use cosmwasm_std::{entry_point, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult};
use cosmwasm_std::{to_binary, Coin, Event, StdError, Uint128};
use cw2::{get_contract_version, set_contract_version};
use router_wasm_bindings::ethabi::{decode, ParamType};
use router_wasm_bindings::types::{ChainType, ContractCall, OutboundBatchRequest, OutgoingTxFee };
use router_wasm_bindings::RouterMsg;

use crate::deploy_code::deploy_code;
use crate::func_change_owner::change_owner;
use crate::func_register_deployer::register_deployer;
use crate::query::{
    fetch_bridge_address, fetch_data, fetch_deploy_state, fetch_deployer, fetch_owner,
};

// version info for migration info
const CONTRACT_NAME: &str = "deploy-erc20";
const CONTRACT_VERSION: &str = "0.1.0";
const REQUEST_TIMEOUT: u64 = 600;
pub const CREATE_OUTBOUND_REPLY_ID: u64 = 1;

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: InstantiateMsg,
) -> StdResult<Response> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
    OWNER.save(deps.storage, &msg.owner)?;
    Ok(Response::new().add_attribute("action", "hello_router_init"))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn sudo(deps: DepsMut, _env: Env, msg: SudoMsg) -> StdResult<Response<RouterMsg>> {
    match msg {
        SudoMsg::HandleInboundReq {
            sender,
            chain_type,
            source_chain_id,
            event_nonce,
            payload,
        } => handle_in_bound_request(
            deps,
            sender,
            chain_type,
            source_chain_id,
            event_nonce,
            payload,
        ),
        SudoMsg::HandleOutboundAck {
            outbound_tx_requested_by,
            destination_chain_type,
            destination_chain_id,
            outbound_batch_nonce,
            execution_code,
            execution_status,
            exec_flags,
            exec_data,
            refund_amount,
        } => handle_out_bound_ack_request(
            deps,
            outbound_tx_requested_by,
            destination_chain_type,
            destination_chain_id,
            outbound_batch_nonce,
            execution_code,
            execution_status,
            exec_flags,
            exec_data,
            refund_amount,
        ),
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: ExecuteMsg,
) -> StdResult<Response<RouterMsg>> {
    match msg {
        ExecuteMsg::ChangeOwner { address } => change_owner(deps, _info, address),
        ExecuteMsg::DeployContract {
            code,
            salt,
            constructor_args,
            chainids,
            gas_price,
            gas_limit,
            forwarder_contract
        } => deploy_code(
            deps,
            _env,
            _info,
            code,
            salt,
            constructor_args,
            chainids,
            gas_price,
            gas_limit,
            forwarder_contract
        ),
        ExecuteMsg::RegisterDeployer { address, chainid } => {
            register_deployer(deps, _info, address, chainid)
        }
        ExecuteMsg::UpdateBridgeContract { address, payload } => {
            update_bridge_contract(deps, address, payload.0)
        }
    }
}

fn handle_in_bound_request(
    deps: DepsMut,
    sender: String,
    chain_type: u32,
    src_chain_id: String,
    event_nonce: u64,
    payload: Binary,
) -> StdResult<Response<RouterMsg>> {
    let payload_string: Vec<u8> = base64::decode(payload.to_string()).unwrap();
    let string: String = String::from_utf8(payload_string).unwrap();
    let reverse_string: String = string.chars().rev().collect::<String>();
    DATA.save(deps.storage, &reverse_string)?;
    let event = Event::new("in_bound_request")
        .add_attribute("sender", sender.to_string())
        .add_attribute("chain_type", chain_type.to_string())
        .add_attribute("src_chain_id", src_chain_id.clone())
        .add_attribute("payload", reverse_string.clone());
    event_nonce.to_string();
    let bridge_address: String = fetch_bridge_address(deps.as_ref())?;
    let contract_call: ContractCall = ContractCall {
        destination_contract_address: bridge_address.into_bytes(),
        payload: reverse_string.into_bytes(),
    };
    let start = SystemTime::now();
    let since_the_epoch = start
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards");
    let current_timestamp = since_the_epoch.as_secs();
    let exp_timestamp: u64 = current_timestamp + REQUEST_TIMEOUT;
    let outbound_batch_req: OutboundBatchRequest = OutboundBatchRequest {
        destination_chain_type: ChainType::ChainTypeEvm.get_chain_code(),
        destination_chain_id: String::from("137"),
        contract_calls: vec![contract_call],
        relayer_fee: Coin {
            denom: String::from("route"),
            amount: Uint128::new(100_000u128),
        },
        outgoing_tx_fee: OutgoingTxFee {
            gas_limit: 25000000,
            gas_price: 25000000,
        },
        is_atomic: false,
        exp_timestamp,
    };
    let outbound_batch_reqs: RouterMsg = RouterMsg::OutboundBatchRequests {
        outbound_batch_requests: vec![outbound_batch_req],
    };

    let res = Response::new()
        .add_message(outbound_batch_reqs)
        .add_event(event)
        .add_attribute("sender", sender)
        .add_attribute("chain_type", chain_type.to_string())
        .add_attribute("src_chain_id", src_chain_id);
    Ok(res)
}

fn handle_out_bound_ack_request(
    deps: DepsMut,
    sender: String,
    destination_chain_type: u32,
    destination_chain_id: String,
    outbound_batch_nonce: u64,
    execution_code: u64,
    execution_status: bool,
    exec_flags: Vec<bool>,
    exec_data: Vec<Binary>,
    _refund_amt: Coin,
) -> StdResult<Response<RouterMsg>> {
    // TODO : Safety checks not applied for call failure and handling of failed requests
    let execution_msg: String = format!(
        "execution_code {:?}, execution_status {:?}, exec_flags {:?}, exec_data {:?} {:?}",
        execution_code,
        execution_status,
        exec_flags,
        exec_data,
        exec_data.len()
    );
    deps.api.debug(&execution_msg);

    let msg: String = format!("execution_code {:?}", &exec_data[1]);
    deps.api.debug(&msg);
    let decoded_payload = match decode(
        &[
            ParamType::Uint(64),
            ParamType::FixedBytes(32),
            ParamType::FixedBytes(32),
            ParamType::Address,
        ],
        &exec_data[1],
    ) {
        Ok(token_vec) => token_vec,
        Err(_) => {
            return Err(StdError::GenericErr {
                msg: String::from("err.into()"),
            })
        }
    };
    deps.api.debug("token vec created");
    let cid = decoded_payload[0].clone().into_uint().unwrap().as_u64();
    let hash_str = hex::encode(decoded_payload[1].clone().into_fixed_bytes().unwrap());
    let salt_str = hex::encode(decoded_payload[2].clone().into_fixed_bytes().unwrap());
    let addr_str = hex::encode(decoded_payload[3].clone().into_address().unwrap());

    let contract_reg_info = CONTRACT_REGISTRY.load(deps.storage, (hash_str.clone(), salt_str.clone(), cid))?;
    let forwarder_contract = contract_reg_info.2.clone();
    // TODO - Need to call Forwarder Registry 


    // Map Hash state to chainID
    deps.api.debug("hash values created");
    CONTRACT_REGISTRY.save(deps.storage, (hash_str, salt_str, cid), &(true, addr_str , forwarder_contract.clone() ))?;
    deps.api.debug("done");
    let res = Response::new()
        .add_attribute("sender", sender)
        .add_attribute("destination_chain_type", destination_chain_type.to_string())
        .add_attribute("destination_chain_id", destination_chain_id)
        .add_attribute("outbound_batch_nonce", outbound_batch_nonce.to_string());
    Ok(res)
}

fn update_bridge_contract(
    deps: DepsMut,
    address: String,
    payload: Vec<u8>,
) -> StdResult<Response<RouterMsg>> {
    BRIDGE_CONTRACT.save(deps.storage, &address)?;

    let contract_call: ContractCall = ContractCall {
        destination_contract_address: address.clone().into_bytes(),
        payload,
    };
    let start = SystemTime::now();
    let since_the_epoch = start
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards");
    let current_timestamp = since_the_epoch.as_secs();
    let exp_timestamp: u64 = current_timestamp + REQUEST_TIMEOUT;
    let outbound_batch_req: OutboundBatchRequest = OutboundBatchRequest {
        destination_chain_type: ChainType::ChainTypeEvm.get_chain_code(),
        destination_chain_id: String::from("137"),
        contract_calls: vec![contract_call],
        relayer_fee: Coin {
            denom: String::from("route"),
            amount: Uint128::new(100_000u128),
        },
        outgoing_tx_fee: OutgoingTxFee {
            gas_limit: 25000000,
            gas_price: 25000000,
        },
        is_atomic: false,
        exp_timestamp,
    };
    let outbound_batch_reqs: RouterMsg = RouterMsg::OutboundBatchRequests {
        outbound_batch_requests: vec![outbound_batch_req],
    };

    let res = Response::new()
        .add_message(outbound_batch_reqs)
        .add_attribute("bridge_address", address);
    Ok(res)
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn migrate(deps: DepsMut, _env: Env, _msg: MigrateMsg) -> StdResult<Response> {
    let ver = cw2::get_contract_version(deps.storage)?;
    // ensure we are migrating from an allowed contract
    if ver.contract != CONTRACT_NAME.to_string() {
        return Err(StdError::generic_err("Can only upgrade from same type").into());
    }
    // note: better to do proper semver compare, but string compare *usually* works
    // if ver.version >= CONTRACT_VERSION.to_string() {
    //     return Err(StdError::generic_err("Cannot upgrade from a newer version").into());
    // }

    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
    Ok(Response::default())
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::GetContractVersion {} => to_binary(&get_contract_version(deps.storage)?),
        QueryMsg::FetchData {} => to_binary(&fetch_data(deps)?),
        QueryMsg::FetchOwner {} => to_binary(&fetch_owner(deps)?),
        QueryMsg::FetchDeployer { chainid } => to_binary(&fetch_deployer(deps, chainid)?),
        QueryMsg::FetchDeployState {
            hash,
            salt,
            chainid,
        } => to_binary(&fetch_deploy_state(deps, hash, salt, chainid)?),
    }
}


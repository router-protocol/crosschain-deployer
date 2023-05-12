use cosmwasm_std::{
    BankMsg, Coin, DepsMut, Env, Event, MessageInfo, ReplyOn, Response, StdError, StdResult,
    SubMsg, Uint128,
};
use router_wasm_bindings::{
    ethabi::{encode, Token},
    types::{AckType, ChainType, RequestMetaData},
    utils::{convert_address_from_bytes_to_string, convert_address_from_string_to_bytes},
    Bytes, RouterMsg, RouterQuery,
};

use crate::{
    contract::CREATE_I_SEND_REQUEST,
    modifers::is_owner_modifier,
    msg::{ChainTypeInfo, ExecuteMsg},
    state::{
        DispatchDataStruct, CHAIN_TYPE_MAPPING, CONTRACT_REGISTRY, DEPLOYER_REGISTER, OWNER,
        TEMP_FORWARDER,
    },
};
use std::str::FromStr;

use sha3::{Digest, Keccak256};

pub fn deployer_execute(
    deps: DepsMut<RouterQuery>,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> StdResult<Response<RouterMsg>> {
    match msg {
        ExecuteMsg::SetChainTypes { chain_type_info } => {
            set_chain_types_info(deps, env, info, chain_type_info)
        }
        ExecuteMsg::ChangeOwner { address } => update_owner(deps, info, address),
        ExecuteMsg::RegisterDeployer { address, chain_id } => {
            register_deployer(deps, info, address, &chain_id)
        }
        ExecuteMsg::DeployContract {
            code,
            salt,
            constructor_args,
            chain_ids,
            gas_limits,
            gas_prices,
            forwarder_contract,
        } => deploy_code(
            deps,
            env,
            info,
            code,
            salt,
            constructor_args,
            chain_ids,
            gas_limits,
            gas_prices,
            forwarder_contract,
        ),
        ExecuteMsg::WithdrawFunds { recipient, amount } => {
            withdraw_funds(deps, &env, &info, recipient, amount)
        }
    }
}

pub fn register_deployer(
    deps: DepsMut<RouterQuery>,
    info: MessageInfo,
    factory: String,
    chainid: &str,
) -> StdResult<Response<RouterMsg>> {
    // Check - only owner can set deployer address
    let owner = OWNER.load(deps.storage)?;
    if owner != info.sender {
        return Err(StdError::generic_err(format!("Unauthorised")));
    }

    DEPLOYER_REGISTER.save(deps.storage, chainid, &factory)?;

    let res = Response::new();
    Ok(res)
}

/**
 * @notice Used to update the owner.
 * @notice Only callable by admin.
 * @param   new_owner  new owner address
*/
pub fn update_owner(
    deps: DepsMut<RouterQuery>,
    info: MessageInfo,
    new_owner: String,
) -> StdResult<Response<RouterMsg>> {
    // Check - only owner can set deployer address
    let owner = OWNER.load(deps.storage)?;
    if owner != info.sender {
        return Err(StdError::generic_err(format!("Unauthorised")));
    }

    OWNER.save(deps.storage, &new_owner)?;

    let res = Response::new();
    Ok(res)
}

/**
 * @notice Used to withdraw funds
 * @notice Only callable by Admin.
*/
pub fn withdraw_funds(
    deps: DepsMut<RouterQuery>,
    _env: &Env,
    info: &MessageInfo,
    recipient: String,
    amount: Uint128,
) -> StdResult<Response<RouterMsg>> {
    is_owner_modifier(deps.as_ref(), &info)?;

    let bank_msg = BankMsg::Send {
        to_address: recipient.into(),
        amount: vec![Coin {
            amount,
            denom: "route".to_string(),
        }],
    };

    let res = Response::new()
        .add_message(bank_msg)
        .add_attribute("action", "SetGasFactor");
    Ok(res)
}

/**
 * @notice Used to set new owner
 * @notice Only callable by Admin.
 * @param  new_owner
*/
pub fn set_owner(
    deps: DepsMut<RouterQuery>,
    _env: &Env,
    info: &MessageInfo,
    new_owner: String,
) -> StdResult<Response<RouterMsg>> {
    is_owner_modifier(deps.as_ref(), &info)?;
    deps.api.addr_validate(&new_owner)?;
    OWNER.save(deps.storage, &new_owner)?;

    let res = Response::new().add_attribute("action", "SetOwner");
    Ok(res)
}

/**
 * @notice Used to set chain type info operations of the given chain (chainId, chainType).
 * @notice Only callable by Admin.
 * @param  chain_type_info   chain infos (chain_id & chain_type)

*/
pub fn set_chain_types_info(
    deps: DepsMut<RouterQuery>,
    _env: Env,
    info: MessageInfo,
    chain_type_info: Vec<ChainTypeInfo>,
) -> StdResult<Response<RouterMsg>> {
    is_owner_modifier(deps.as_ref(), &info)?;

    for i in 0..chain_type_info.len() {
        CHAIN_TYPE_MAPPING.save(
            deps.storage,
            &chain_type_info[i].chain_id,
            &chain_type_info[i].chain_type,
        )?;
    }
    let event_name: String = String::from("SetChainTypeInfo");
    let set_chain_bytes_info_event: Event = Event::new(event_name);

    let res = Response::new()
        .add_attribute("action", "SetChainTypeInfo")
        .add_event(set_chain_bytes_info_event);
    Ok(res)
}

pub fn deploy_code(
    deps: DepsMut<RouterQuery>,
    _env: Env,
    info: MessageInfo,
    code: String,
    salt: String,
    constructor_args: Vec<String>,
    chain_ids: Vec<String>,
    gas_limits: Vec<u64>,
    gas_prices: Vec<u64>,
    forwarder_contract: String,
) -> StdResult<Response<RouterMsg>> {
    assert_eq!(
        constructor_args.len(),
        chain_ids.len(),
        "constructor_args length & chain_ids length should be equal"
    );
    assert_eq!(
        gas_limits.len(),
        chain_ids.len(),
        "gas_limits length & chain_ids length should be equal"
    );
    assert_eq!(
        gas_limits.len(),
        gas_prices.len(),
        "gas_limits length & gas_prices length should be equal"
    );
    let mut chainid_contract_calls: Vec<DispatchDataStruct> = vec![];
    let mut deploy_event: Vec<Event> = vec![];
    let mut outbound_messages: Vec<SubMsg<RouterMsg>> = vec![];

    let evm_chain_type: u64 = ChainType::ChainTypeEvm.get_chain_code();
    // Tokenise payload data
    let code_vec: Vec<u8> =
        convert_address_from_string_to_bytes(code.clone(), evm_chain_type).unwrap();
    let mut code_hasher: Keccak256 = Keccak256::new();
    code_hasher.update(code_vec.clone());
    let code_hash: Bytes = code_hasher.finalize().to_vec();
    let code_hash_str = convert_address_from_bytes_to_string(&code_hash, evm_chain_type).unwrap();
    let salt_vec: Vec<u8> =
        convert_address_from_string_to_bytes(salt.clone(), evm_chain_type).unwrap();
    let salt_str_dec = convert_address_from_bytes_to_string(&salt_vec, evm_chain_type).unwrap();
    let code_event = Event::new("code hash")
        .add_attribute("hash - ", code_hash_str.clone())
        .add_attribute("salt - ", salt_str_dec.clone())
        .add_attribute("caller - ", info.sender.to_string());
    for i in 0..chain_ids.len() {
        let uint128: Uint128 = Uint128::from_str(&chain_ids[i]).unwrap();
        let cid: u64 = uint128.u128() as u64;

        // Constructor Args
        let code_tokenized: Token;
        if constructor_args.len() != 0 {
            let mut constructor_arg_vec: Bytes =
                convert_address_from_string_to_bytes(constructor_args[i].clone(), evm_chain_type)
                    .unwrap();
            let mut code_final = code_vec.clone();
            code_final.append(&mut constructor_arg_vec);
            code_tokenized = Token::Bytes(code_final.clone().into());
        } else {
            code_tokenized = Token::Bytes(code_vec.clone().into());
        }

        let code_hash_tokenized = Token::FixedBytes(code_hash.to_vec());
        let salt_tokenized = Token::FixedBytes(salt_vec.clone().into());

        // Generate Payload
        let payload: Bytes = encode(&[code_tokenized, salt_tokenized, code_hash_tokenized]);

        let payload_str = hex::encode(payload.clone());

        // GET Factory Address
        let deployer_str: String = DEPLOYER_REGISTER
            .load(deps.storage, &chain_ids[i])
            .unwrap_or_default();

        // Map Hash state to chainID
        CONTRACT_REGISTRY.save(
            deps.storage,
            (&code_hash_str, &salt_str_dec, &chain_ids[i]),
            &(false, "pending ack".to_string(), info.sender.to_string()),
        )?;
        // Generate and add Event
        let payload_str = format!(
            "destContract:- {:?},  payloadRaw:- {:?} , Payload_str:- {:?} ",
            deployer_str.clone(),
            payload.clone(),
            payload_str,
        );

        let cid_str = format!("chainID :- {:?}", cid);
        let evt = Event::new("deploy_code_event").add_attribute(cid_str, payload_str.clone());
        deploy_event.push(evt);

        //Fetch Gas Prices
        // let router_querier: RouterQuerier = RouterQuerier::new(&deps.querier);
        // let gas_price = router_querier.gas_price(chain_types[i].clone())?;

        let new_dispatch = DispatchDataStruct {
            payload: payload.clone().to_vec(),
            dest_addr: deployer_str.clone(),
            chain_id: cid,
            chain_gas_limit: gas_limits[i],
            chain_gas_price: gas_prices[i],
        };
        chainid_contract_calls.push(new_dispatch);
    }

    for j in 0..chainid_contract_calls.len() {
        let cid = chainid_contract_calls[j].chain_id.clone();
        let contact_call_payload = chainid_contract_calls[j].payload.clone();
        let contract_addr = chainid_contract_calls[j].dest_addr.clone();
        let limit = chainid_contract_calls[j].chain_gas_limit.clone();
        let price = chainid_contract_calls[j].chain_gas_price.clone();

        let request_packet: Bytes = encode(&[
            Token::String(contract_addr),
            Token::Bytes(contact_call_payload),
        ]);
        let dest_chain_id: String = String::from(cid.to_string());

        let request_metadata: RequestMetaData = RequestMetaData {
            dest_gas_limit: limit,
            dest_gas_price: price,
            ack_gas_limit: limit,
            ack_gas_price: price,
            relayer_fee: Uint128::zero(),
            ack_type: AckType::AckOnBoth,
            is_read_call: false,
            asm_address: "".to_string(),
        };

        let i_send_request: RouterMsg = RouterMsg::CrosschainCall {
            version: 1,
            route_amount: Uint128::new(0u128),
            route_recipient: "".to_string(),
            dest_chain_id,
            request_metadata: request_metadata.get_abi_encoded_bytes(),
            request_packet,
        };
        let cross_chain_sub_msg: SubMsg<RouterMsg> = SubMsg {
            id: CREATE_I_SEND_REQUEST,
            msg: i_send_request.into(),
            gas_limit: None,
            reply_on: ReplyOn::Success,
        };
        outbound_messages.push(cross_chain_sub_msg);
    }

    // IF Outbound Message is 0 throw error
    if outbound_messages.len() == 0 {
        return Err(StdError::GenericErr {
            msg: "Outbound Message is null".to_string(),
        });
    }
    deps.api.addr_validate(&forwarder_contract)?;
    TEMP_FORWARDER.save(deps.storage, &forwarder_contract)?;
    let res = Response::new()
        .add_submessages(outbound_messages)
        .add_event(code_event)
        .add_events(deploy_event);

    Ok(res)
}

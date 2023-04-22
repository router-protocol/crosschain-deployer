use crate::state::{DispatchDataStruct, DEPLOYER_REGISTER};
use cosmwasm_std::{DepsMut, Env, Event, MessageInfo, Response, StdError, StdResult, Uint128};
use router_wasm_bindings::ethabi::{encode, Token};
use router_wasm_bindings::types::{AckType, RequestMetaData};
use router_wasm_bindings::{Bytes, RouterMsg, RouterQuerier, RouterQuery};

// use crate::query::fetch_oracle_gas_price;

use sha3::{Digest, Keccak256};

pub fn deploy_code(
    deps: DepsMut<RouterQuery>,
    _env: Env,
    info: MessageInfo,
    code: String,
    salt: String,
    constructor_args: Vec<String>,
    chainid: Vec<u64>,
    chain_types: Vec<String>,
    gas_limit: Vec<u64>,
) -> StdResult<Response<RouterMsg>> {
    // let mut req = [ 0 , 9];
    let mut chainid_contract_calls: Vec<DispatchDataStruct> = vec![];
    let mut deploy_event: Vec<Event> = vec![];
    let mut outbound_messages: Vec<RouterMsg> = vec![];

    // Tokenise payload data
    let code_str = code.replace("0x", "");
    let code_vec: Vec<u8> = match hex::decode(code_str) {
        Ok(addr) => addr,
        Err(err) => {
            deps.api.debug(&err.to_string());
            return Err(StdError::GenericErr {
                msg: err.to_string(),
            });
        }
    };

    let salt_str = salt.replace("0x", "");
    let salt_vec: Vec<u8> = match hex::decode(salt_str) {
        Ok(addr) => addr,
        Err(err) => {
            deps.api.debug(&err.to_string());
            return Err(StdError::GenericErr {
                msg: err.to_string(),
            });
        }
    };

    let mut code_hasher: Keccak256 = Keccak256::new();
    code_hasher.update(code_vec.clone());
    let code_hash = code_hasher.finalize();

    let code_hash_str = hex::encode(code_hash.clone().to_vec());
    let salt_str_dec = hex::encode(salt_vec.clone().to_vec());
    let code_event = Event::new("code hash")
        .add_attribute("hash - ", code_hash_str.clone())
        .add_attribute("salt - ", salt_str_dec.clone())
        .add_attribute("caller - ", info.sender);
    let mut gas_total: u64 = 0;
    for i in 0..chainid.len() {
        let cid = chainid[i];

        // Constructor Args
        let code_tokenized: Token;
        if constructor_args.len() != 0 {
            let constructor_arg = constructor_args[i].clone();
            let constructor_arg_str = constructor_arg.replace("0x", "");
            let mut constructor_arg_vec: Vec<u8> = match hex::decode(constructor_arg_str) {
                Ok(addr) => addr,
                Err(err) => {
                    deps.api.debug(&err.to_string());
                    return Err(StdError::GenericErr {
                        msg: err.to_string(),
                    });
                }
            };
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
            .load(deps.storage, &cid.to_string())
            .unwrap_or_default();
        let deployer_addr_str: String = deployer_str.replace("0x", "");
        let deployer_addr_vec: Vec<u8> = match hex::decode(deployer_addr_str) {
            Ok(addr) => addr,
            Err(err) => {
                deps.api.debug(&err.to_string());
                return Err(StdError::GenericErr {
                    msg: err.to_string(),
                });
            }
        };

        // Map Hash state to chainID
        // CONTRACT_REGISTRY.save(
        //     deps.storage,
        //     (
        //         code_hash_str.clone().into(),
        //         salt_str_dec.clone().into(),
        //         cid,
        //     ),
        //     &(false, "pending ack".to_string(), forwarder_contract.clone()),
        // )?;

        // Generate and add Event
        let payload_str = format!(
            "destContract:- {:?},  payloadRaw:- {:?} , Payload_str:- {:?}",
            deployer_str,
            payload.clone(),
            payload_str
        );

        let cid_str = format!("chainID :- {:?}", cid);
        let evt = Event::new("deploy_code_event").add_attribute(cid_str, payload_str.clone());
        deploy_event.push(evt);

        //Fetch Gas Prices
        let router_querier: RouterQuerier = RouterQuerier::new(&deps.querier);
        let gas_price = router_querier.gas_price(chain_types[i].clone())?;

        // Generate Factory Address

        let new_dispatch = DispatchDataStruct {
            payload: payload.clone().to_vec(),
            dest_addr: deployer_addr_vec.clone().to_vec(),
            chain_id: cid,
            chain_gas_limit: gas_limit[i],
            chain_gas_price: gas_price.gas_price,
        };
        let gas_used = gas_limit[i].clone() * gas_price.gas_price.clone();
        gas_total = gas_total + gas_used;

        chainid_contract_calls.push(new_dispatch);
    }

    for j in 0..chainid_contract_calls.len() {
        let cid = chainid_contract_calls[j].chain_id.clone();
        let contact_call_payload = chainid_contract_calls[j].payload.clone();
        let contract_addr = chainid_contract_calls[j].dest_addr.clone();
        let limit = chainid_contract_calls[j].chain_gas_limit.clone();
        let price = chainid_contract_calls[j].chain_gas_price.clone();

        let request_packet: Bytes = encode(&[
            Token::Bytes(contract_addr),
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

        let request: RouterMsg = RouterMsg::CrosschainCall {
            version: 1,
            route_amount: Uint128::new(0u128),
            route_recipient: "".to_string(),
            dest_chain_id,
            request_metadata: request_metadata.get_abi_encoded_bytes(),
            request_packet,
        };
        outbound_messages.push(request);
    }

    // IF Outbound Message is 0 throw error
    if outbound_messages.len() == 0 {
        return Err(StdError::GenericErr {
            msg: "Outbound Message is null".to_string(),
        });
    }

    let res = Response::new()
        .add_messages(outbound_messages)
        .add_event(code_event)
        .add_events(deploy_event);

    Ok(res)
}

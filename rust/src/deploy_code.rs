use crate::state::{DispatchDataStruct, DEPLOYER_REGISTER };
use cosmwasm_std::{Coin,Event, DepsMut, Env, Response, StdError, StdResult, Uint128};
use router_wasm_bindings::ethabi::{encode, Token};
use router_wasm_bindings::types::{ChainType, ContractCall, OutboundBatchRequest,OutgoingTxFee};
use router_wasm_bindings::RouterMsg;

pub fn deploy_code(
    deps: DepsMut,
    env: Env,
    code: String,
    chainid: Vec<u64>,
    gas_price: Vec<u64>,
    gas_limit: Vec<u64>
) -> StdResult<Response<RouterMsg>> {
    let mut batch_req: Vec<OutboundBatchRequest> = vec![];
    let mut chainid_contract_calls: Vec<DispatchDataStruct> = vec![];
    let mut deploy_event: Vec<Event> = vec![];

    for i in 0..chainid.len() {
        let cid = chainid[i];

        // Tokenise payload data
        let code_str = code.replace("0x", "");
        let code_vec: Vec<u8> = match hex::decode(code_str) {
            Ok(addr) => addr,
            Err(err) => {
                deps.api.debug(&err.to_string());
                return Err(StdError::GenericErr{
                    msg: err.to_string(),
                });
            }
        };
        let code_tokenized = Token::Bytes(code_vec.clone().into());

        // Generate Payload
        let payload = encode(&[ code_tokenized ]);

        let payload_str  = hex::encode(payload.clone());

        // Generate Factory Address
        let deployer_str: String  = DEPLOYER_REGISTER.load(deps.storage, &cid.to_string()).unwrap_or_default();
        let deployer_addr_str: String = deployer_str.replace("0x", "");
        let deployer_addr_vec: Vec<u8> = match hex::decode(deployer_addr_str) {
            Ok(addr) => addr,
            Err(err) => {
                deps.api.debug(&err.to_string());
                return Err(StdError::GenericErr{
                    msg: err.to_string(),
                });
            }
        };
        let payload_str = format!(
            "destContract:- {:?},  payloadRaw:- {:?} , Payload_str:- {:?}",
            deployer_str, payload.clone() , payload_str
        );
        // deps.api.debug(&payload_str);
        let cid_str = format!(
            "chainID :- {:?}",
            cid
        );
        let evt = Event::new("deploy_code_event").add_attribute(cid_str, payload_str.clone());
        deploy_event.push(evt);
        // Generate Factory Address
        let contract_call: ContractCall = ContractCall {
            destination_contract_address: deployer_addr_vec.clone(),
            payload: payload.clone().to_vec(),
        };
        let new_dispatch = DispatchDataStruct {
            payload: vec![contract_call],
            chain_id: cid,
            chain_gas_limit : gas_limit[i],
            chain_gas_price: gas_price[i]
        };

        chainid_contract_calls.push(new_dispatch);

    }


    for j in 0..chainid_contract_calls.len() {

        let cid = chainid_contract_calls[j].chain_id.clone();
        let contact_call_payload = chainid_contract_calls[j].payload.clone();
        let limit = chainid_contract_calls[j].chain_gas_limit.clone();
        let price = chainid_contract_calls[j].chain_gas_price.clone();
        let request = OutboundBatchRequest {
            destination_chain_type: ChainType::ChainTypeEvm.get_chain_code(),
            destination_chain_id: cid.to_string(),
            contract_calls: contact_call_payload,
            relayer_fee: Coin {
                denom: String::from("route"),
                amount: Uint128::new(10_000_000u128),
            },
            outgoing_tx_fee: OutgoingTxFee {
                gas_limit: limit,
                gas_price: price,
            },
            is_atomic: false,
            exp_timestamp: env.block.time.seconds() + 24 * 60 * 60,
        };
        batch_req.push(request);
    }

    // IF Batch size is 0 throw error
    if batch_req.len() == 0 {
            return Err(StdError::GenericErr{
                msg: "Batch Request is null".to_string(),
            });
    }

    let outbound_batch_reqs: RouterMsg = RouterMsg::OutboundBatchRequests {
        outbound_batch_requests: batch_req.to_vec(),
    };

    let res = Response::new()
        .add_message(outbound_batch_reqs)
        .add_events(deploy_event);

    Ok(res)
}

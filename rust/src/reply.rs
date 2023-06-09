use crate::contract::FORWARDER_REPLY_ID;
use crate::{
    contract::CREATE_I_SEND_REQUEST,
    state::{FORWARDER_CONTRACT_MAPPING, TEMP_FORWARDER},
};
#[cfg(not(feature = "library"))]
use cosmwasm_std::{entry_point, DepsMut, Env, Response, StdResult};
use cosmwasm_std::{from_binary, Reply, StdError, SubMsgResult};
use router_wasm_bindings::{types::CrosschainRequestResponse, RouterMsg, RouterQuery};

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn reply(deps: DepsMut<RouterQuery>, _env: Env, msg: Reply) -> StdResult<Response<RouterMsg>> {
    match msg.id {
        CREATE_I_SEND_REQUEST => {
            deps.api.debug(&msg.id.to_string());
            // TODO: need to handle nonce data here, Nonce handling logic depends on the use-case.
            let response: Response<RouterMsg> = Response::new();
            match msg.result {
                SubMsgResult::Ok(msg_result) => match msg_result.data {
                    Some(binary_data) => {
                        deps.api.debug("Binary Data Found");
                        let cross_chain_req_res: CrosschainRequestResponse =
                            from_binary(&binary_data).unwrap();
                        let nonce: u64 = cross_chain_req_res.request_identifier;
                        let info_str: String = format!(
                            "Binary data {:?}, response {:?}",
                            &binary_data.to_string(),
                            cross_chain_req_res
                        );
                        let forwarder_addr: String = TEMP_FORWARDER.load(deps.storage)?;
                        FORWARDER_CONTRACT_MAPPING.save(
                            deps.storage,
                            &nonce.to_string(),
                            &forwarder_addr,
                        )?;
                        deps.api.debug(&info_str);
                        return Ok(response);
                    }
                    None => deps.api.debug("No Binary Data Found"),
                },
                SubMsgResult::Err(err) => deps.api.debug(&err.to_string()),
            }
        }
        FORWARDER_REPLY_ID => {
            deps.api.debug("Inside the forwarder reply ID");
            return Ok(Response::new());
        }
        id => return Err(StdError::generic_err(format!("Unknown reply id: {}", id))),
    }
    Ok(Response::new())
}

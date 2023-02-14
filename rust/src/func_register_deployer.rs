use crate::state::{ OWNER, DEPLOYER_REGISTER};
use cosmwasm_std::{DepsMut, MessageInfo, Response, StdError, StdResult};
use router_wasm_bindings::RouterMsg;

pub fn register_deployer(
    deps: DepsMut,
    info: MessageInfo,
    factory: String,
    chainid: u64,
) -> StdResult<Response<RouterMsg>> {
    // Check - only owner can set deployer address 
    let owner = OWNER.load(deps.storage)?;
    if owner != info.sender {
        return Err(StdError::generic_err(format!("Unauthorised")));
    }
    let key = chainid.to_string();

    DEPLOYER_REGISTER.save(deps.storage, &key, &factory)?;

    let res = Response::new();
    Ok(res)
}

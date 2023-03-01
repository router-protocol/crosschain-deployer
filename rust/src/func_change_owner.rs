use crate::state::OWNER;
use cosmwasm_std::Addr;
use cosmwasm_std::{DepsMut, MessageInfo, Response, StdError, StdResult};
use router_wasm_bindings::RouterMsg;

pub fn change_owner(
    deps: DepsMut,
    info: MessageInfo,
    new_owner: Addr,
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

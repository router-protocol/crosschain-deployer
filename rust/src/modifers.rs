use cosmwasm_std::{Deps, MessageInfo, StdError, StdResult};
use router_wasm_bindings::RouterQuery;

use crate::state::OWNER;

pub fn is_owner_modifier(deps: Deps<RouterQuery>, info: &MessageInfo) -> StdResult<()> {
    let owner: String = match OWNER.load(deps.storage) {
        Ok(owner) => owner,
        Err(err) => return StdResult::Err(err),
    };
    if owner != info.sender.to_string() {
        return StdResult::Err(StdError::GenericErr {
            msg: String::from("Auth: Invalid Owner"),
        });
    }
    Ok(())
}

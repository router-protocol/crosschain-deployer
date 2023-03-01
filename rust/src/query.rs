use crate::state::{BRIDGE_CONTRACT, CONTRACT_REGISTRY, DATA, DEPLOYER_REGISTER, OWNER};
use cosmwasm_std::Addr;
use cosmwasm_std::{Deps, StdResult};

pub fn fetch_deployer(deps: Deps, chainid: u64) -> StdResult<String> {
    DEPLOYER_REGISTER.load(deps.storage, &chainid.to_string())
}

pub fn fetch_owner(deps: Deps) -> StdResult<Addr> {
    let owner = OWNER.load(deps.storage);
    Ok(owner.unwrap())
}

pub fn fetch_data(deps: Deps) -> StdResult<String> {
    return Ok(DATA.load(deps.storage)?);
}

pub fn fetch_bridge_address(deps: Deps) -> StdResult<String> {
    return Ok(BRIDGE_CONTRACT.load(deps.storage)?);
}

pub fn fetch_deploy_state(
    deps: Deps,
    code_hash: String,
    salt: String,
    chainid: u64,
) -> StdResult<(bool, String , String)> {
    CONTRACT_REGISTRY.load(deps.storage, (code_hash, salt, chainid))
}

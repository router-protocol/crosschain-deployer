use crate::state::{
    BRIDGE_CONTRACT, CHAIN_TYPE_MAPPING, CONTRACT_REGISTRY, DATA, DEPLOYER_REGISTER, OWNER,
};
use cosmwasm_std::Addr;
use cosmwasm_std::{Deps, StdResult};
use router_wasm_bindings::types::GasPriceResponse;
use router_wasm_bindings::RouterQuerier;
use router_wasm_bindings::RouterQuery;

pub fn fetch_deployer(deps: Deps<RouterQuery>, chainid: u64) -> StdResult<String> {
    DEPLOYER_REGISTER.load(deps.storage, &chainid.to_string())
}

pub fn fetch_owner(deps: Deps<RouterQuery>) -> StdResult<Addr> {
    let owner = OWNER.load(deps.storage);
    Ok(owner.unwrap())
}

pub fn fetch_data(deps: Deps<RouterQuery>) -> StdResult<String> {
    return Ok(DATA.load(deps.storage)?);
}

pub fn fetch_bridge_address(deps: Deps<RouterQuery>) -> StdResult<String> {
    return Ok(BRIDGE_CONTRACT.load(deps.storage)?);
}

pub fn fetch_deploy_state(
    deps: Deps<RouterQuery>,
    code_hash: String,
    salt: String,
    chainid: u64,
) -> StdResult<(bool, String, String)> {
    CONTRACT_REGISTRY.load(deps.storage, (code_hash, salt, chainid))
}

pub fn fetch_oracle_gas_price(
    deps: Deps<RouterQuery>,
    chain_id: String,
) -> StdResult<GasPriceResponse> {
    // let query_wrapper: QuerierWrapper = QuerierWrapper::new(&deps.querier);
    let router_querier: RouterQuerier = RouterQuerier::new(&deps.querier);
    router_querier.gas_price(chain_id)
}

pub fn fetch_chain_type(deps: Deps<RouterQuery>, chain_id: &str) -> StdResult<u64> {
    CHAIN_TYPE_MAPPING.load(deps.storage, chain_id)
}

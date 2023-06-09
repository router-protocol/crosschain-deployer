use crate::msg::QueryMsg;
use crate::state::{CHAIN_TYPE_MAPPING, CONTRACT_REGISTRY, DATA, DEPLOYER_REGISTER, OWNER};
use cosmwasm_std::{to_binary, Binary, Env};
use cosmwasm_std::{Deps, StdResult};
use cw2::get_contract_version;
use router_wasm_bindings::types::GasPriceResponse;
use router_wasm_bindings::RouterQuerier;
use router_wasm_bindings::RouterQuery;

pub fn deployer_query(deps: Deps<RouterQuery>, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::GetContractVersion {} => to_binary(&get_contract_version(deps.storage)?),
        QueryMsg::FetchData {} => to_binary(&fetch_data(deps)?),
        QueryMsg::FetchChainType { chain_id } => to_binary(&fetch_chain_type(deps, &chain_id)?),
        QueryMsg::FetchOwner {} => to_binary(&fetch_owner(deps)?),
        QueryMsg::FetchDeployer { chain_id } => to_binary(&fetch_deployer(deps, &chain_id)?),
        QueryMsg::FetchDeployState {
            code_hash,
            salt,
            chain_id,
        } => to_binary(&fetch_deploy_state(deps, &code_hash, &salt, &chain_id)?),
        QueryMsg::FetchOracleGasPrice { chain_id } => {
            to_binary(&fetch_oracle_gas_price(deps, chain_id)?)
        }
    }
}

pub fn fetch_deployer(deps: Deps<RouterQuery>, chainid: &str) -> StdResult<String> {
    DEPLOYER_REGISTER.load(deps.storage, &chainid)
}

pub fn fetch_owner(deps: Deps<RouterQuery>) -> StdResult<String> {
    OWNER.load(deps.storage)
}

pub fn fetch_data(deps: Deps<RouterQuery>) -> StdResult<String> {
    return Ok(DATA.load(deps.storage)?);
}

pub fn fetch_deploy_state(
    deps: Deps<RouterQuery>,
    code_hash: &str,
    salt: &str,
    chain_id: &str,
) -> StdResult<(bool, String, String)> {
    CONTRACT_REGISTRY.load(deps.storage, (code_hash, salt, chain_id))
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

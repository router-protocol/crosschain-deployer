use cosmwasm_std::Addr;
use cw_storage_plus::{Item, Map};
use router_wasm_bindings::types::ContractCall;

pub const DATA: Item<String> = Item::new("data_string");
pub const BRIDGE_CONTRACT: Item<String> = Item::new("BRIDGE_CONTRACT ");

// Contract Constants
pub const OWNER: Item<Addr> = Item::new("Owner ");

pub struct DispatchDataStruct {
    pub payload: Vec<ContractCall>,
    pub chain_id: u64,
    pub chain_gas_price: u64,
    pub chain_gas_limit: u64,
}
// Code Hash , Salt , chainID => State , Address

pub const CONTRACT_REGISTRY: Map<(String, String, u64), (bool, String, String)> =
    Map::new("contract_registry");

pub const DEPLOYER_REGISTER: Map<&str, String> = Map::new("deployer_registry");

pub const REQUEST_FORWARD_MAPPING: Map<(String, u32, u64), String> =
    Map::new("reqeust_forwarder_mapping");

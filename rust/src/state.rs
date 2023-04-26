use cosmwasm_std::Addr;
use cw_storage_plus::{Item, Map};
use router_wasm_bindings::Bytes;

pub const DATA: Item<String> = Item::new("data_string");

pub const BRIDGE_CONTRACT: Item<String> = Item::new("bridge_contract");

pub const CHAIN_TYPE_MAPPING: Map<&str, u64> = Map::new("chain_type_mapping");

pub const OWNER: Item<Addr> = Item::new("Owner ");

pub const DEPLOYER_REGISTER: Map<&str, String> = Map::new("deployer_registry");

pub const CONTRACT_REGISTRY: Map<(String, String, u64), (bool, String, String)> =
    Map::new("contract_registry");

pub struct DispatchDataStruct {
    pub payload: Bytes,
    pub dest_addr: String,
    pub chain_id: u64,
    pub chain_gas_price: u64,
    pub chain_gas_limit: u64,
}

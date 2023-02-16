
use cosmwasm_std::Addr;
use cw_storage_plus::{Item, Map};
use router_wasm_bindings::types::ContractCall;

pub const DATA: Item<String> = Item::new("data_string");
pub const BRIDGE_CONTRACT : Item<String> = Item::new( "BRIDGE_CONTRACT ");

// Contract Constants
pub const OWNER: Item<Addr> = Item::new("Owner ");


pub struct DispatchDataStruct {
    pub payload: Vec<ContractCall>,
    pub chain_id: u64,
    pub chain_gas_price: u64,
    pub chain_gas_limit: u64,

}

pub const DEPLOYER_REGISTER: Map<&str, String> = Map::new("erc_factory_registry");


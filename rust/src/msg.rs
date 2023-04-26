use cosmwasm_std::{Addr, Binary};
pub use router_wasm_bindings::SudoMsg;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InstantiateMsg {
    // here user can define required init variables
    pub owner: Addr,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    // here user can define other executable messages
    UpdateBridgeContract {
        address: String,
        payload: Binary,
    },
    SetChainType {
        chain_id: String,
        chain_type: u64,
    },
    ChangeOwner {
        address: Addr,
    },
    RegisterDeployer {
        address: String,
        chainid: u64,
    },
    DeployContract {
        code: String,
        salt: String,
        constructor_args: Vec<String>,
        chainids: Vec<u64>,
        gas_limit: Vec<u64>,
        gas_prices: Vec<u64>,
    },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct MigrateMsg {}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    // fetch contract version
    GetContractVersion {},
    FetchData {},
    FetchBridgeAddress {},
    FetchChainType {
        chain_id: String,
    },

    FetchOwner {},
    FetchDeployer {
        chainid: u64,
    },
    FetchDeployState {
        hash: String,
        salt: String,
        chainid: u64,
    },
    FetchOracleGasPrice {
        chain_id: String,
    },
}

use cosmwasm_std::Uint128;
pub use router_wasm_bindings::SudoMsg;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct ChainTypeInfo {
    pub chain_id: String,
    pub chain_type: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InstantiateMsg {
    pub owner: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    SetChainTypes {
        chain_type_info: Vec<ChainTypeInfo>,
    },
    ChangeOwner {
        address: String,
    },
    RegisterDeployer {
        address: String,
        chain_id: String,
    },
    DeployContract {
        code: String,
        salt: String,
        constructor_args: Vec<String>,
        chain_ids: Vec<String>,
        gas_limits: Vec<u64>,
        gas_prices: Vec<u64>,
        forwarder_contract: String,
    },
    WithdrawFunds {
        recipient: String,
        amount: Uint128,
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
    FetchChainType {
        chain_id: String,
    },

    FetchOwner {},
    FetchDeployer {
        chain_id: String,
    },
    FetchDeployState {
        code_hash: String,
        salt: String,
        chain_id: String,
    },
    FetchOracleGasPrice {
        chain_id: String,
    },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct CustodyContractInfo {
    pub address: String,
    pub chain_id: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ForwarderExecuteMsg {
    SetCustodyContracts {
        custody_contracts: Vec<CustodyContractInfo>,
    },
}

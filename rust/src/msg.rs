use cosmwasm_std::{ Binary , Addr };
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
    UpdateBridgeContract { address: String, payload: Binary },
    ChangeOwner { address: Addr },
    DeployContract {
        code: String,
        chainids: Vec<u64>,
        gas_price: Vec<u64>,
        gas_limit: Vec<u64>,
    },
    RegisterDeployer {
        address: String,
        chainid: u64,
    },


}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct MigrateMsg {}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    // fetch contract version
    GetContractVersion {}, // here user defined other query messages
    FetchData {},          // here user defined other query messages
    FetchOwner{},
    FetchDeployer {
        chainid: u64,
    },

}

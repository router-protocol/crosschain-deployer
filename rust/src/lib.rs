pub mod contract;
mod error;
pub mod msg;
pub mod state;

#[cfg(test)]
pub mod tests;

pub use crate::error::ContractError;
mod deploy_code;
mod func_register_deployer;
mod func_change_owner;
mod query;

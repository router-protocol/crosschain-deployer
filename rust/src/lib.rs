pub mod contract;
mod error;
pub mod msg;
pub mod state;

#[cfg(test)]
pub mod tests;

mod func_change_owner;
mod func_register_deployer;
mod deploy_code;
mod query;

pub use crate::error::ContractError;

#![no_std]

// All the imports necessary to execute the mods from the contract
mod contract;
mod contract_trait;
mod errors;
mod metadata;
mod storage_types;
#[cfg(test)]
mod test;
pub mod testutils;
pub use crate::contract::CarRentalContract;

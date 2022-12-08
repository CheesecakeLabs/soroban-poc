#![no_std]

mod contract;
mod contract_trait;
mod errors;
mod metadata;
mod storage_types;
#[cfg(test)]
mod test;
pub mod testutils;
pub use crate::contract::CarRentalContract;

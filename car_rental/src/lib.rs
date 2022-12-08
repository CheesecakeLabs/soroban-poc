#![no_std]

mod errors;
mod metadata;
mod storage_types;
mod contract;
mod contract_trait;
#[cfg(test)]

mod test;
pub mod testutils;
pub use crate::contract::CarRentalContract;
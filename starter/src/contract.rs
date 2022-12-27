use crate::errors::Error;
use soroban_auth::{Identifier, Signature};
use soroban_sdk::{contractimpl, panic_with_error, Env};

pub trait ContractTrait {}

pub struct Contract;
#[contractimpl]
impl ContractTrait for Contract {}

use crate::contract::StakingClient;
use core::fmt::Debug;
use soroban_sdk::{BytesN, Env};

pub struct StakingTest {
    env: Env,
    contract_id: BytesN<32>,
}

impl StakingTest {
    pub fn new(env: &Env, contract_id: &BytesN<32>) -> Self {
        Self {
            env: env.clone(),
            contract_id: contract_id.clone(),
        }
    }

    pub fn initialize(
        &self,
        staking_token_id: BytesN<32>,
        rewards_token_id: BytesN<32>,
        rate: u64,
    ) {
        StakingClient::new(&self.env, &self.contract_id).initialize(
            &staking_token_id,
            &rewards_token_id,
            &rate,
        );
    }
}
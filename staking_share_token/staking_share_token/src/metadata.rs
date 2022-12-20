use crate::storage_types::DataKey;
use soroban_sdk::{BytesN, Env};

pub fn write_staking_token(e: &Env, contract_id: BytesN<32>) {
    e.storage().set(DataKey::StakingTkn, contract_id);
}

pub fn write_rewards_token(e: &Env, contract_id: BytesN<32>) {
    e.storage().set(DataKey::RewardsTkn, contract_id);
}

pub fn write_reward_rate(e: &Env, rate: u64) {
    e.storage().set(DataKey::RewardRate, rate);
}

pub fn has_reward_rate(e: &Env) -> bool {
    let key = DataKey::RewardRate;
    e.storage().has(key)
}

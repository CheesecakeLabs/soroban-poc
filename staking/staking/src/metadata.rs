use crate::{contract::token::Identifier, storage_types::DataKey};
use soroban_sdk::{BytesN, Env};

pub fn write_staking_token(e: &Env, contract_id: BytesN<32>) {
    e.storage().set(DataKey::StakingTkn, contract_id);
}
pub fn read_staking_token(e: &Env) -> BytesN<32> {
    e.storage().get_unchecked(DataKey::StakingTkn).unwrap()
}

pub fn write_rewards_token(e: &Env, contract_id: BytesN<32>) {
    e.storage().set(DataKey::RewardsTkn, contract_id);
}

pub fn write_reward_rate(e: &Env, rate: i128) {
    e.storage().set(DataKey::RewardRate, rate);
}

pub fn has_reward_rate(e: &Env) -> bool {
    let key = DataKey::RewardRate;
    e.storage().has(key)
}
pub fn read_reward_rate(e: &Env) -> i128 {
    e.storage().get_unchecked(DataKey::RewardRate).unwrap()
}

pub fn write_total_supply(e: &Env, amount: i128) {
    e.storage().set(DataKey::TotSupply, amount);
}

pub fn read_total_supply(e: &Env) -> i128 {
    e.storage()
        .get(DataKey::TotSupply)
        .unwrap_or(Ok(0))
        .unwrap()
}

pub fn update_total_supply(e: &Env, amount: i128) {
    let total_supply = read_total_supply(e);
    write_total_supply(e, total_supply + amount);
}

pub fn read_balance(e: &Env, identifier: Identifier) -> i128 {
    e.storage()
        .get(DataKey::Balances(identifier))
        .unwrap_or(Ok(0))
        .unwrap()
}
pub fn write_balance(e: &Env, identifier: Identifier, amount: i128) {
    e.storage().set(DataKey::Balances(identifier), amount);
}

pub fn update_balance(e: &Env, identifier: Identifier, amount: i128) {
    let balance = read_balance(e, identifier.clone());
    write_balance(e, identifier, balance + amount);
}

pub fn write_account_rewards(e: &Env, identifier: Identifier, amount: i128) {
    e.storage().set(DataKey::Rewards(identifier), amount);
}

pub fn read_account_rewards(e: &Env, identifier: Identifier) -> i128 {
    e.storage()
        .get(DataKey::Rewards(identifier)).unwrap_or(Ok(0))
        .unwrap()
}

pub fn update_account_rewards(e: &Env, identifier: Identifier, amount: i128) {
    let rewards = read_account_rewards(e, identifier.clone());
    write_account_rewards(e, identifier, rewards + amount);
}

pub fn read_last_update(e: &Env) -> u64 {
    e.storage()
        .get(DataKey::LastUpdate)
        .unwrap_or(Ok(0))
        .unwrap()
}

pub fn write_last_update(e: &Env, time: u64) {
    e.storage().set(DataKey::LastUpdate, time);
}

pub fn read_reward_stored(e: &Env) -> i128 {
    e.storage().get(DataKey::RwrdStored).unwrap_or(Ok(0)).unwrap()
}

pub fn write_reward_stored(e: &Env, amount: i128) {
    e.storage().set(DataKey::RwrdStored, amount);
}

pub fn write_reward_paid(e: &Env, identifier: Identifier, amount: i128) {
    e.storage().set(DataKey::RwrdPaid(identifier), amount);
}

pub fn read_reward_paid(e: &Env, identifier: Identifier, amount: i128) {
    e.storage().get(DataKey::RwrdPaid(identifier)).unwrap_or(Ok(0)).unwrap();
}


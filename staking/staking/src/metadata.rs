use crate::storage_types::DataKey;
use soroban_auth::Identifier;
use soroban_sdk::{BytesN, Env};

pub fn write_staking_token(e: &Env, contract_id: BytesN<32>) {
    e.storage().set(DataKey::StakingTkn, contract_id);
}

pub fn write_rewards_token(e: &Env, contract_id: BytesN<32>) {
    e.storage().set(DataKey::RewardsTkn, contract_id);
}

pub fn write_reward_rate(e: &Env, rate: i128) {
    e.storage().set(DataKey::RewardRate, rate);
}

pub fn write_total_supply(e: &Env, value: i128) {
    e.storage().set(DataKey::TotSupply, value);
}

pub fn write_last_update(e: &Env, value: u64) {
    e.storage().set(DataKey::LastUpdate, value);
}

pub fn write_reward_per_token_stored(e: &Env, value: i128) {
    e.storage().set(DataKey::RwrdStored, value);
}

pub fn write_balance(e: &Env, id: Identifier, value: i128) {
    e.storage().set(DataKey::Balances(id), value);
}

pub fn write_reward(e: &Env, id: Identifier, value: i128) {
    e.storage().set(DataKey::Rewards(id), value);
}

pub fn write_reward_paid(e: &Env, id: Identifier, value: i128) {
    e.storage().set(DataKey::RwrdPaid(id), value);
}

pub fn has_reward_rate(e: &Env) -> bool {
    let key = DataKey::RewardRate;
    e.storage().has(key)
}

pub fn get_staking_token_contract_id(e: &Env) -> BytesN<32> {
    let key = DataKey::StakingTkn;
    e.storage().get_unchecked(key).unwrap()
}

pub fn get_rewards_token_contract_id(e: &Env) -> BytesN<32> {
    let key = DataKey::RewardsTkn;
    e.storage().get_unchecked(key).unwrap()
}

pub fn get_total_supply(e: &Env) -> i128 {
    let key = DataKey::TotSupply;
    e.storage().get(key).unwrap_or(Ok(0)).unwrap()
}

pub fn get_reward_per_token_stored(e: &Env) -> i128 {
    let key = DataKey::RwrdStored;
    e.storage().get(key).unwrap_or(Ok(0)).unwrap()
}

pub fn get_last_update(e: &Env) -> u64 {
    let key = DataKey::LastUpdate;
    e.storage()
        .get(key)
        .unwrap_or(Ok(e.ledger().timestamp()))
        .unwrap()
}

pub fn get_reward_rate(e: &Env) -> i128 {
    let key = DataKey::RewardRate;
    e.storage().get_unchecked(key).unwrap()
}

pub fn get_balance(e: &Env, id: Identifier) -> i128 {
    let key = DataKey::Balances(id);
    e.storage().get(key).unwrap_or(Ok(0)).unwrap()
}

pub fn get_reward(e: &Env, id: Identifier) -> i128 {
    let key = DataKey::Rewards(id);
    e.storage().get(key).unwrap_or(Ok(0)).unwrap()
}

pub fn get_reward_paid(e: &Env, id: Identifier) -> i128 {
    let key = DataKey::RwrdPaid(id);
    e.storage().get(key).unwrap_or(Ok(0)).unwrap()
}

pub fn increase_total_supply(e: &Env, value: i128) {
    write_total_supply(&e, get_total_supply(&e) + value)
}

pub fn increase_balance(e: &Env, id: Identifier, value: i128) {
    let new_balance = get_balance(&e, id.clone()) + value;
    write_balance(&e, id, new_balance);
}

pub fn decrease_total_supply(e: &Env, value: i128) {
    write_total_supply(&e, get_total_supply(&e) - value)
}

pub fn decrease_balance(e: &Env, id: Identifier, value: i128) {
    let new_balance = get_balance(&e, id.clone()) - value;
    write_balance(&e, id, new_balance);
}

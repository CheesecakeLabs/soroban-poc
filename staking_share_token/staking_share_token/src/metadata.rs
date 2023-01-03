use crate::storage_types::DataKey;
use soroban_sdk::{BytesN, Env};

pub fn write_staking_token(e: &Env, contract_id: BytesN<32>) {
    e.storage().set(DataKey::StakingTkn, contract_id);
}

pub fn write_share_token(e: &Env, contract_id: BytesN<32>) {
    e.storage().set(DataKey::ShareTkn, contract_id);
}

pub fn write_rate(e: &Env, rate: i128) {
    e.storage().set(DataKey::Rate, rate);
}

pub fn read_rate(e: &Env) -> i128 {
    e.storage().get_unchecked(DataKey::Rate).unwrap()
}

pub fn has_rate(e: &Env) -> bool {
    let key = DataKey::Rate;
    e.storage().has(key)
}

pub fn read_staking_token(e: &Env) -> BytesN<32> {
    e.storage().get_unchecked(DataKey::StakingTkn).unwrap()
}

pub fn read_share_token(e: &Env) -> BytesN<32> {
    e.storage().get_unchecked(DataKey::ShareTkn).unwrap()
}

pub fn write_total_supply(e: &Env, value: i128) {
    e.storage().set(DataKey::TotSupply, value);
}

pub fn read_total_supply(e: &Env) -> i128 {
    let key = DataKey::TotSupply;
    e.storage().get(key).unwrap_or(Ok(0)).unwrap()
}

pub fn read_total_share(e: &Env) -> i128 {
    let key = DataKey::TotShare;
    e.storage().get(key).unwrap_or(Ok(0)).unwrap()
}

pub fn write_total_share(e: &Env, value: i128) {
    e.storage().set(DataKey::TotShare, value);
}

pub fn increase_total_supply(e: &Env, value: i128) {
    write_total_supply(&e, read_total_supply(&e) + value)
}

pub fn increase_total_share(e: &Env, value: i128) {
    write_total_share(&e, read_total_share(&e) + value)
}

pub fn decrease_total_supply(e: &Env, value: i128) {
    write_total_supply(&e, read_total_supply(&e) - value)
}

pub fn decrease_total_share(e: &Env, value: i128) {
    write_total_share(&e, read_total_share(&e) - value)
}

pub fn read_last_updated(e: &Env) -> u64 {
    e.storage()
        .get(DataKey::LastUpdate)
        .unwrap_or(Ok(0))
        .unwrap()
}

pub fn write_last_updated(e: &Env, timestamp: u64) {
    e.storage().set(DataKey::LastUpdate, timestamp);
}

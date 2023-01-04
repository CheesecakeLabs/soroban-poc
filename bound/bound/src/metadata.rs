use crate::storage_types::{DataKey, State};
use soroban_auth::Identifier;
use soroban_sdk::{BytesN, Env};

// #[contracttype]
// pub enum DataKey {
//     State,      // State
//     Admin,      // Identifier
//     PaymentTkn, // BytesN<32>
//     BondTkn,   // BytesN<32>
//     FeeIntrvl,  // u32
//     FeeRate,    // i128
//     InitPrice,  // i128
//     Price,      // i128
//     InitTime,   // u64
//     EndTime,    // u64
//     Supply,     // i128
// }

pub fn write_admin(e: &Env, id: Identifier) {
    e.storage().set(DataKey::Admin, id);
}

pub fn write_payment_token(e: &Env, contract_id: BytesN<32>) {
    e.storage().set(DataKey::PaymentTkn, contract_id);
}

pub fn write_bond_token(e: &Env, contract_id: BytesN<32>) {
    e.storage().set(DataKey::BondTkn, contract_id);
}

pub fn write_fee_interval(e: &Env, interval: u32) {
    e.storage().set(DataKey::FeeIntrvl, interval);
}

pub fn write_fee_rate(e: &Env, rate: i128) {
    e.storage().set(DataKey::FeeRate, rate);
}

pub fn write_init_price(e: &Env, init_price: i128) {
    e.storage().set(DataKey::InitPrice, init_price);
}

pub fn write_price(e: &Env, price: i128) {
    e.storage().set(DataKey::Price, price);
}

pub fn write_init_time(e: &Env, init_time: u64) {
    e.storage().set(DataKey::InitTime, init_time);
}

pub fn write_end_time(e: &Env, end_time: u64) {
    e.storage().set(DataKey::EndTime, end_time);
}

pub fn write_supply(e: &Env, supply: i128) {
    e.storage().set(DataKey::Supply, supply);
}

pub fn has_admin(env: &Env) -> bool {
    env.storage().has(DataKey::Admin)
}

pub fn read_state(e: &Env) -> State {
    let key = DataKey::State;
    e.storage()
        .get(key)
        .unwrap_or(Ok(State::NoInitiatd))
        .unwrap()
}

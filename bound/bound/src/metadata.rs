use crate::{
    errors::Error,
    storage_types::{DataKey, State},
};
use soroban_auth::{Identifier, Signature};
use soroban_sdk::{panic_with_error, BytesN, Env};

pub fn write_admin(e: &Env, id: Identifier) {
    e.storage().set(DataKey::Admin, id);
}

pub fn write_state(e: &Env, state: State) {
    e.storage().set(DataKey::State, state);
}

pub fn write_payment_token(e: &Env, contract_id: BytesN<32>) {
    e.storage().set(DataKey::PaymentTkn, contract_id);
}

pub fn write_bond_token(e: &Env, contract_id: BytesN<32>) {
    e.storage().set(DataKey::BondTkn, contract_id);
}

pub fn write_fee_interval(e: &Env, interval: u64) {
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
    e.storage()
        .get(DataKey::State)
        .unwrap_or(Ok(State::NoInitiatd))
        .unwrap()
}

pub fn read_bond_token_id(e: &Env) -> BytesN<32> {
    e.storage().get_unchecked(DataKey::BondTkn).unwrap()
}

pub fn read_admin(e: &Env) -> Identifier {
    e.storage().get_unchecked(DataKey::Admin).unwrap()
}

pub fn check_admin(e: &Env, auth: &Signature) {
    let auth_id = auth.identifier(e);
    if auth_id != read_admin(e) {
        panic_with_error!(&e, Error::NotAuthorized);
    };
}

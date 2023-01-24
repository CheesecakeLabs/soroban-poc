use crate::{
    errors::Error,
    storage_types::{DataKey, InterestType, State},
};
use soroban_auth::{Identifier, Signature};
use soroban_sdk::{panic_with_error, BytesN, Env};

// Write functions
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

pub fn write_fee_type(e: &Env, fee_type: InterestType) {
    e.storage().set(DataKey::FeeType, fee_type);
}

pub fn write_user(e: &Env, user: Identifier) {
    e.storage().set(DataKey::User(user.clone()), user);
}

// Delete functions
pub fn delete_user(e: &Env, user: &Identifier) {
    e.storage().remove(DataKey::User(user.clone()));
}

// Read functions
pub fn read_init_time(e: &Env) -> u64 {
    e.storage().get(DataKey::InitTime).unwrap_or(Ok(0)).unwrap()
}

pub fn read_end_time(e: &Env) -> u64 {
    e.storage().get(DataKey::EndTime).unwrap_or(Ok(0)).unwrap()
}

pub fn read_fee_rate(e: &Env) -> i128 {
    e.storage().get(DataKey::FeeRate).unwrap_or(Ok(0)).unwrap()
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

pub fn read_price(e: &Env) -> i128 {
    e.storage().get_unchecked(DataKey::Price).unwrap()
}

pub fn read_supply(e: &Env) -> i128 {
    e.storage().get(DataKey::Supply).unwrap_or(Ok(0)).unwrap()
}

pub fn read_fee_interval(e: &Env) -> u64 {
    e.storage().get_unchecked(DataKey::FeeIntrvl).unwrap()
}

pub fn read_payment_token(e: &Env) -> BytesN<32> {
    e.storage().get_unchecked(DataKey::PaymentTkn).unwrap()
}

pub fn read_fee_type(e: &Env) -> InterestType {
    e.storage().get_unchecked(DataKey::FeeType).unwrap()
}

// Aux functions
pub fn increase_supply(e: &Env, supply: i128) {
    let old_supply = read_supply(&e);
    write_supply(e, old_supply + supply);
}

pub fn decrease_supply(e: &Env, supply: i128) {
    let old_supply = read_supply(&e);
    write_supply(e, old_supply - supply);
}

pub fn check_admin(e: &Env, auth: &Signature) {
    let auth_id = auth.identifier(e);
    if auth_id != read_admin(e) {
        panic_with_error!(&e, Error::NotAuthorized);
    };
}

pub fn check_user(e: &Env, user: &Identifier) -> bool {
    e.storage().has(DataKey::User(user.clone()))
}

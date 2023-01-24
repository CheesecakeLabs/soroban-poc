use crate::context::{DataKey, State};
use crate::errors::Error;
use soroban_auth::{Identifier, Signature};
use soroban_sdk::{panic_with_error, vec, BytesN, Env, Vec};

// Write functions
pub fn write_admin(env: &Env, admin: Identifier) {
    env.storage().set(DataKey::Admin, admin);
}

pub fn write_gain_percent(env: &Env, percentage: i128) {
    env.storage().set(DataKey::GainPctge, percentage);
}

pub fn write_token_address(env: &Env, token_address: BytesN<32>) {
    env.storage().set(DataKey::TokenAddr, token_address);
}

pub fn write_state(env: &Env, state: State) {
    env.storage().set(DataKey::State, state);
}

pub fn write_ticket_price(env: &Env, price: i128) {
    env.storage().set(DataKey::TicktPrice, price);
}

pub fn write_user(env: &Env, user: Identifier) {
    let mut users: Vec<Identifier> = env
        .storage()
        .get(DataKey::Users)
        .unwrap_or(Ok(vec![env]))
        .unwrap();

    users.push_back(user);

    env.storage().set(DataKey::Users, users)
}

// Read functions
pub fn read_state(env: &Env) -> State {
    env.storage()
        .get(DataKey::State)
        .unwrap_or(Ok(State::NotInititd))
        .unwrap()
}

pub fn read_token_address(env: &Env) -> BytesN<32> {
    env.storage().get_unchecked(DataKey::TokenAddr).unwrap()
}

pub fn read_ticket_price(env: &Env) -> i128 {
    env.storage().get_unchecked(DataKey::TicktPrice).unwrap()
}

pub fn read_admin_gain_percent(env: &Env) -> i128 {
    env.storage().get_unchecked(DataKey::GainPctge).unwrap()
}

pub fn read_admin(env: &Env) -> Identifier {
    env.storage().get_unchecked(DataKey::Admin).unwrap()
}

pub fn read_users(env: &Env) -> Vec<Identifier> {
    env.storage().get_unchecked(DataKey::Users).unwrap()
}

// Aux functions
pub fn check_admin(env: &Env, auth: &Signature) {
    let auth_id = auth.identifier(env);
    if auth_id != read_admin(env) {
        panic_with_error!(&env, Error::NotAuthorized);
    };
}

pub fn clear_users(env: &Env) {
    let users: Vec<Identifier> = vec![&env];
    env.storage().set(DataKey::Users, users);
}

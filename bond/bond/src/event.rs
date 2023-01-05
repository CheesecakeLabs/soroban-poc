use soroban_auth::Identifier;
use soroban_sdk::{symbol, BytesN, Env};

pub(crate) fn initialize(e: &Env, admin: Identifier, bond_id: BytesN<32>, initial_amount: i128) {
    let topics = (
        symbol!("initialize"),
        admin.clone(),
        bond_id,
        initial_amount,
    );
    e.events().publish(topics, admin);
}

pub(crate) fn start(e: &Env, initial_timestamp: u64) {
    let topics = (symbol!("start"),);
    e.events().publish(topics, initial_timestamp);
}

pub(crate) fn set_end(e: &Env, end_timestamp: u64) {
    let topics = (symbol!("set_end"),);
    e.events().publish(topics, end_timestamp);
}

pub(crate) fn withdraw(e: &Env, amount: i128, to: Identifier) {
    let topics = (symbol!("withdraw"), to);
    e.events().publish(topics, amount);
}

pub(crate) fn en_csh_out(e: &Env) {
    let topics = (symbol!("en_csh_out"),);
    e.events().publish(topics, ());
}

pub(crate) fn cash_out(e: &Env, amount: i128, to: Identifier) {
    let topics = (symbol!("cash_out"), to);
    e.events().publish(topics, amount);
}

pub(crate) fn buy(e: &Env, amount: i128, from: Identifier) {
    let topics = (symbol!("buy"), from);
    e.events().publish(topics, amount);
}

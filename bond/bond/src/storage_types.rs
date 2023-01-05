use soroban_sdk::contracttype;

use crate::contract::token::Identifier;

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum State {
    NoInitiatd,
    Initiated,
    Available,
    CashOutEn,
    Paused,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum InterestType {
    Simple,
    Compound,
}

#[derive(Clone)]
#[contracttype]
pub enum DataKey {
    State,            // State
    Admin,            // Identifier
    PaymentTkn,       // BytesN<32>
    BondTkn,          // BytesN<32>
    FeeIntrvl,        // u64
    FeeRate,          // i128
    FeeType,          // InterestType
    Price,            // i128
    InitTime,         // u64
    EndTime,          // u64
    Supply,           // i128
    User(Identifier), // Identifier
}

use crate::errors::Error;
use crate::metadata::{
    read_state, write_admin, write_bond_token, write_fee_rate, write_payment_token, write_price,
};
use crate::storage_types::State;
use soroban_auth::{Identifier, Signature};
use soroban_sdk::{contractimpl, panic_with_error, Bytes, BytesN, Env};
pub mod token {
    soroban_sdk::contractimport!(file = "../soroban_token_spec.wasm");
}
use token::{Client as TokenClient, TokenMetadata};

pub trait ContractTrait {
    // Start the contract and create the bond token
    fn initialize(
        e: Env,
        admin: Identifier,
        payment_token_id: BytesN<32>,
        bond_token_name: Bytes,
        bond_token_symbol: Bytes,
        bond_token_decimals: u32,
        price: i128,
        fee_rate: i128,
        fee_days_interval: u64,
        initial_amount: i128,
    );

    // maybe change this name to something like "turn available" in 10 chars
    // Turn the token available for puchases and set initial timestamp
    fn start(e: Env, initial_timestamp: u64);

    // Set the end timestamp
    fn set_end(e: Env, end_timestamp: u64);

    // Transfer tokens from the contract to the admin
    fn withdraw(e: Env, amount: i128);

    // Makes the token available for claiming
    fn liquidate(e: Env);

    // Buy tokens
    fn buy(e: Env, amount: i128);

    // Claim tokens
    fn claim(e: Env);

    // Get current price
    fn get_price(e: Env);
}

pub struct Contract;
#[contractimpl]
impl ContractTrait for Contract {
    fn initialize(
        e: Env,
        admin: Identifier,
        payment_token_id: BytesN<32>,
        bond_token_name: Bytes,
        bond_token_symbol: Bytes,
        bond_token_decimals: u32,
        price: i128,
        fee_rate: i128,
        fee_days_interval: u64,
        initial_amount: i128,
    ) {
        if read_state(&e) != State::NoInitiatd {
            panic_with_error!(&e, Error::AlreadyInitialized)
        }
        write_admin(&e, admin.clone());

        // Save Payment token address
        write_payment_token(&e, payment_token_id);
        // Create Bond token contract
        let (bond_id, bond_token) = create_bond_token(
            &e,
            &admin,
            bond_token_name,
            bond_token_symbol,
            bond_token_decimals,
        );
        // Save Bond token address
        write_bond_token(&e, bond_id);

        // TODO: Save fee interval (maybe in seconds?)

        // Save Bon token fee rate (multiplied by 100)
        write_fee_rate(&e, fee_rate);

        // Save the Bond token price (in terms of Payment token)
        write_price(&e, price);

        // Mint the initial amount
        if initial_amount > 0 {
            bond_token.mint(
                &Signature::Invoker,
                &0,
                &Identifier::Contract(e.current_contract()),
                &initial_amount,
            )
        }
    }

    fn start(e: Env, initial_timestamp: u64) {
        // check admin
        // check state == started
        // set state = available
        // set initial timestamp
    }

    fn set_end(e: Env, end_timestamp: u64) {
        // check admin
        // check state != liquidated
        // check end_timestamp > initial_timestamp
        // set end_timestamp
    }

    fn withdraw(e: Env, amount: i128) {
        // check admin
        // check state != liquidated
        // xfer amount to admin address
    }

    fn liquidate(e: Env) {
        // check admin
        // check state == available
        // check now > end_timestamp
        // calculates Amount sold * current price
        // check if the contract has this balance of payment tokens
        // set price = current price
        // set state = liquidated
    }

    fn buy(e: Env, amount: i128) {
        // check state == available
        // calculates amount * current price
        // xfer_from from invoker to contract (payment tokens)
        // xfer the amount from contract to user (bond)
        // set AmountSold = AmountSold + amount
    }

    fn claim(e: Env) {
        // check state == liquidated
        // get inkover balance (bond)
        // calculates balance * price
        // xfer this amount to invoker (payment tokens)
        // burn balance from user (bond)
    }

    fn get_price(e: Env) {
        // check state == available or liquidated
        // if price: return price
        // return current_price
    }
}

fn current_price(e: Env) {
    // time = (now - initial_timestamp) / fee_interval
    // return initial_price * (1 + fee_rate / 100) ^ time
}

fn create_bond_token(
    e: &Env,
    admin: &Identifier,
    name: Bytes,
    symbol: Bytes,
    decimals: u32,
) -> (BytesN<32>, TokenClient) {
    let id = e.register_contract_token(None);
    let token = TokenClient::new(e, &id);

    token.init(
        &admin,
        &TokenMetadata {
            name: name,
            symbol: symbol,
            decimals: decimals,
        },
    );

    (id, token)
}

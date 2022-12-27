use crate::errors::Error;
use soroban_auth::{Identifier, Signature};
use soroban_sdk::{contractimpl, panic_with_error, Bytes, BytesN, Env};

pub trait ContractTrait {
    // Start the contract and create the PREK token
    fn initialize(
        e: Env,
        admin: Identifier,
        payment_token_id: BytesN<32>,
        token_name: Bytes,
        token_symbol: Bytes,
        token_decimals: u32,
        price: i128,
        fee_rate: i128,
        fee_days_interval: u32,
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
        token_name: Bytes,
        token_symbol: Bytes,
        token_decimals: u32,
        price: i128,
        fee_rate: i128,
        fee_days_interval: u32,
        initial_amount: i128,
    ) {
        // check State == NotStarted
        // set Admin
        // create PREK contract
        // save payment token address
        // save prek token address
        // save fee interval (maybe in seconds? must change it to u64)
        // save fee rate (check how do the percent)
        // save the price
        // mint the initial amount if > 0
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
        // xfer the amount from contract to user (prek)
        // set AmountSold = AmountSold + amount
    }

    fn claim(e: Env) {
        // check state == liquidated
        // get inkover balance (prek)
        // calculates balance * price
        // xfer this amount to invoker (payment tokens)
        // burn balance from user (prek)
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

use crate::errors::Error;
use crate::metadata::{
    check_admin, read_bond_token_id, read_init_time, read_state, write_admin, write_bond_token,
    write_end_time, write_fee_interval, write_fee_rate, write_init_time, write_payment_token,
    write_price, write_state,read_payment_token
};
use crate::storage_types::State;
use soroban_auth::{Identifier, Signature};
use soroban_sdk::{contractimpl, panic_with_error, Bytes, BytesN, Env};
pub mod token {
    soroban_sdk::contractimport!(file = "../soroban_token_spec.wasm");
}
use token::{Client as TokenClient, TokenMetadata};

pub trait BondTrait {
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

    // Turn the cash out enabled
    fn en_csh_out(e: Env);

    // Buy tokens
    fn buy(e: Env, amount: i128);

    // Cash out tokens
    fn cash_out(e: Env);

    // Get current price
    fn get_price(e: Env);

    // Get Bond Token contract ID
    fn bond_id(e: Env) -> BytesN<32>;
}

pub struct Bond;
#[contractimpl]
impl BondTrait for Bond {
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

        write_state(&e, State::Initiated);
        write_admin(&e, admin.clone());

        // Save Payment token address
        write_payment_token(&e, payment_token_id);
        // Create Bond token contract
        let (bond_id, bond_token) =
            create_bond_token(&e, bond_token_name, bond_token_symbol, bond_token_decimals);
        // Save Bond token address
        write_bond_token(&e, bond_id);

        // Save fee interval
        write_fee_interval(&e, days_to_seconds(fee_days_interval));

        // Save Bond token fee rate (multiplied by 1000)
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
        check_admin(&e, &Signature::Invoker);

        if read_state(&e) != State::Initiated {
            panic_with_error!(&e, Error::NotInitialized)
        }

        write_state(&e, State::Available);
        write_init_time(&e, initial_timestamp);
    }

    fn set_end(e: Env, end_timestamp: u64) {
        check_admin(&e, &Signature::Invoker);

        if read_state(&e) == State::Liquidated {
            panic_with_error!(&e, Error::AlreadyLiquidated)
        }

        if read_init_time(&e) > end_timestamp {
            panic_with_error!(&e, Error::InvalidTimestamp)
        }

        write_end_time(&e, end_timestamp);
    }

    fn withdraw(e: Env, amount: i128) {
        check_admin(&e, &Signature::Invoker);
        // check state != liquidated
        if read_state(&e) == State::Liquidated {
            panic_with_error!(&e, Error::AlreadyLiquidated)
        }

        // xfer amount to admin address
        transfer_from_contract_to_account(
            &e,
            &read_payment_token(&e),
            &e.invoker().clone().into(),
            &amount,
        )

        
    }

    fn cash_out(e: Env) {
        // check state == liquidated
        // get inkover balance (bond)
        // calculates balance * price
        // xfer this amount to invoker (payment tokens)
        // burn balance from user (bond)
    }

    fn en_csh_out(e: Env) {
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



    fn get_price(e: Env) {
        // check state == available or liquidated
        // if price: return price
        // return current_price
    }

    fn bond_id(e: Env) -> BytesN<32> {
        read_bond_token_id(&e)
    }
}

fn current_price(e: Env) {
    // time = (now - initial_timestamp) / fee_interval
    // return initial_price * (1 + fee_rate / 100) ^ time
}

fn create_bond_token(
    e: &Env,
    name: Bytes,
    symbol: Bytes,
    decimals: u32,
) -> (BytesN<32>, TokenClient) {
    let salt = Bytes::new(e);
    let salt = e.crypto().sha256(&salt);
    let id = e.deployer().with_current_contract(salt).deploy_token();
    let token = TokenClient::new(e, &id);

    token.init(
        &Identifier::Contract(e.get_current_contract()),
        &TokenMetadata {
            name: name,
            symbol: symbol,
            decimals: decimals,
        },
    );

    (id, token)
}

fn days_to_seconds(days: u64) -> u64 {
    days * 24 * 60 * 60
}

fn transfer_from_contract_to_account(
    e: &Env,
    token_id: &BytesN<32>,
    to: &Identifier,
    amount: &i128,
) {
    let client = token::Client::new(e, token_id);
    client.xfer(&Signature::Invoker, &0, to, amount);
}

fn transfer_from_account_to_contract(
    e: &Env,
    token_id: &BytesN<32>,
    from: &Identifier,
    amount: &i128,
) {
    let client = token::Client::new(e, token_id);
    client.xfer_from(
        &Signature::Invoker,
        &0,
        from,
        &Identifier::Contract(e.current_contract()),
        &amount,
    );
}

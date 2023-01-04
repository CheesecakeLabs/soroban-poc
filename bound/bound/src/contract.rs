use crate::errors::Error;
use crate::metadata::{
    check_admin, decrease_supply, increase_supply, read_bond_token_id, read_end_time,
    read_fee_interval, read_fee_rate, read_init_time, read_payment_token, read_price, read_state,
    read_supply, write_admin, write_bond_token, write_end_time, write_fee_interval, write_fee_rate,
    write_init_time, write_payment_token, write_price, write_state,
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
    fn get_price(e: Env) -> i128;

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

        if read_state(&e) == State::CashOutEn {
            panic_with_error!(&e, Error::AlreadyCashOutEn)
        }

        if read_init_time(&e) > end_timestamp {
            panic_with_error!(&e, Error::InvalidTimestamp)
        }

        write_end_time(&e, end_timestamp);
    }

    fn withdraw(e: Env, amount: i128) {
        check_admin(&e, &Signature::Invoker);

        if read_state(&e) == State::CashOutEn {
            panic_with_error!(&e, Error::AlreadyCashOutEn)
        }

        transfer_from_contract_to_account(
            &e,
            &read_payment_token(&e),
            &e.invoker().clone().into(),
            &amount,
        )
    }

    fn cash_out(e: Env) {
        if read_state(&e) != State::CashOutEn {
            panic_with_error!(&e, Error::NotCashOutEn)
        }

        let invoker: Identifier = e.invoker().into();

        // Get the user Bond Token balance
        let bond_balance = token_balance(&e, &read_bond_token_id(&e), &invoker.clone());
        // Calculates amount of payment token
        let total_payment = bond_balance * current_price(&e);
        // Decrease supply
        decrease_supply(&e, bond_balance);
        // Transfer amount of payment tokens from contract to user
        transfer_from_contract_to_account(
            &e,
            &read_bond_token_id(&e),
            &invoker.clone(),
            &total_payment,
        );
        // Burn all the Bond tokens from user
        burn_token(
            &e,
            &read_bond_token_id(&e),
            &Signature::Invoker,
            &invoker,
            &bond_balance,
        )
    }

    fn en_csh_out(e: Env) {
        // check admin
        check_admin(&e, &Signature::Invoker);
        // check state == available
        if read_state(&e) != State::Available {
            panic_with_error!(&e, Error::NotAvailable)
        }
        // check now > end_timestamp
        if e.ledger().timestamp() < read_end_time(&e) {
            panic_with_error!(&e, Error::NotCashOutEn)
        }
        // calculates Amount sold * current price
        let amount_payment = current_price(&e) * read_supply(&e);
        // check if the contract has this balance of payment tokens
        let token_balance = token_balance(
            &e,
            &read_payment_token(&e),
            &Identifier::Contract(e.current_contract()),
        );

        if token_balance < amount_payment {
            panic_with_error!(&e, Error::NotEnoughTokens)
        }

        // set state = liquidated
        write_state(&e, State::CashOutEn);
    }

    fn buy(e: Env, amount: i128) {
        if read_state(&e) != State::Available {
            panic_with_error!(&e, Error::NotAvailable)
        }

        increase_supply(&e, amount);

        // Total will be the Bond amount multiplied by Bond price
        let total = current_price(&e) * amount;
        let invoker: Identifier = e.invoker().into();

        transfer_from_account_to_contract(&e, &read_payment_token(&e), &invoker.clone(), &total);
        transfer_from_contract_to_account(&e, &read_bond_token_id(&e), &invoker.clone(), &amount);
    }

    fn get_price(e: Env) -> i128 {
        current_price(&e)
    }

    fn bond_id(e: Env) -> BytesN<32> {
        read_bond_token_id(&e)
    }
}

fn current_price(e: &Env) -> i128 {
    let mut end_time = read_end_time(&e);
    let now = e.ledger().timestamp();

    if now < end_time {
        end_time = now;
    }

    let time = (end_time - read_init_time(&e)) / read_fee_interval(&e);
    if time == 0 {
        return read_price(&e);
    }
    let fees = 1000 + read_fee_rate(&e);
    read_price(&e) * (fees.pow(time as u32)) / 1000
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

fn token_balance(e: &Env, token_id: &BytesN<32>, id: &Identifier) -> i128 {
    let client = token::Client::new(e, token_id);
    client.balance(id)
}

fn burn_token(e: &Env, token_id: &BytesN<32>, admin: &Signature, from: &Identifier, amount: &i128) {
    let client = token::Client::new(e, token_id);
    client.burn(admin, &0, from, amount);
}

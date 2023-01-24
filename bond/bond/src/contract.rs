use crate::errors::Error;
use crate::event;
use crate::metadata::{
    check_admin, check_user, decrease_supply, delete_user, increase_supply, read_bond_token_id,
    read_end_time, read_fee_interval, read_fee_rate, read_fee_type, read_init_time,
    read_payment_token, read_price, read_state, read_supply, write_admin, write_bond_token,
    write_end_time, write_fee_interval, write_fee_rate, write_fee_type, write_init_time,
    write_payment_token, write_price, write_state, write_user,
};
use crate::storage_types::{InterestType, State};
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
        fee_type: InterestType,
        initial_amount: i128,
    );

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

    // add user to white list
    fn add_user(e: Env, user: Identifier);

    // remove user from white list
    fn rm_user(e: Env, user: Identifier);

    // Pause the contract (disable the buy function)
    fn pause(e: Env);

    // Unpause the contract (enables the buy function)
    fn unpause(e: Env);
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
        fee_type: InterestType,
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
        write_bond_token(&e, bond_id.clone());

        // Save fee interval
        write_fee_interval(&e, days_to_seconds(fee_days_interval));

        // Save Bond token fee rate (multiplied by 1000)
        write_fee_rate(&e, fee_rate);

        // Save interest type (simple ou compound)
        write_fee_type(&e, fee_type);

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

        event::initialize(&e, admin, bond_id, initial_amount);
    }

    fn start(e: Env, initial_timestamp: u64) {
        check_admin(&e, &Signature::Invoker);

        if read_state(&e) != State::Initiated {
            panic_with_error!(&e, Error::NotInitialized)
        }

        write_state(&e, State::Available);
        write_init_time(&e, initial_timestamp);
        event::start(&e, initial_timestamp);
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
        event::set_end(&e, end_timestamp);
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
        );
        event::withdraw(&e, amount, e.invoker().clone().into())
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
            &&read_payment_token(&e),
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
        );
        event::cash_out(&e, bond_balance, invoker);
    }

    fn en_csh_out(e: Env) {
        check_admin(&e, &Signature::Invoker);
        let state = read_state(&e);

        if state != State::Available && state != State::Paused {
            panic_with_error!(&e, Error::NotAvailable)
        }

        // Check if end time has passed
        if e.ledger().timestamp() < read_end_time(&e) {
            panic_with_error!(&e, Error::EndTimeNotPassed)
        }

        // Check if the contract has the amount of payment tokens to
        // pay the users
        let amount_payment = current_price(&e) * read_supply(&e);
        let contract_balance = token_balance(
            &e,
            &read_payment_token(&e),
            &Identifier::Contract(e.current_contract()),
        );

        if contract_balance < amount_payment {
            panic_with_error!(&e, Error::NotEnoughTokens)
        }

        write_state(&e, State::CashOutEn);
        event::en_csh_out(&e);
    }

    fn buy(e: Env, amount: i128) {
        if read_state(&e) != State::Available {
            panic_with_error!(&e, Error::NotAvailable)
        }
        if !check_user(&e, &(e.invoker().into())) {
            panic_with_error!(&e, Error::UserNotAllowed)
        }

        increase_supply(&e, amount);

        // Total will be the Bond amount multiplied by Bond price
        let total = current_price(&e) * amount;
        let invoker: Identifier = e.invoker().into();

        transfer_from_account_to_contract(&e, &read_payment_token(&e), &invoker.clone(), &total);
        transfer_from_contract_to_account(&e, &read_bond_token_id(&e), &invoker.clone(), &amount);
        event::buy(&e, amount, invoker);
    }

    fn get_price(e: Env) -> i128 {
        current_price(&e)
    }

    fn bond_id(e: Env) -> BytesN<32> {
        read_bond_token_id(&e)
    }

    fn add_user(e: Env, user: Identifier) {
        check_admin(&e, &Signature::Invoker);
        if check_user(&e, &user) {
            panic_with_error!(&e, Error::UserAlreadyAllowed)
        }
        write_user(&e, user);
    }

    fn rm_user(e: Env, user: Identifier) {
        check_admin(&e, &Signature::Invoker);
        if !check_user(&e, &user) {
            panic_with_error!(&e, Error::UserNotAllowed)
        }
        delete_user(&e, &user);
    }

    fn pause(e: Env) {
        check_admin(&e, &Signature::Invoker);
        if read_state(&e) != State::Available {
            panic_with_error!(&e, Error::NotAvailable)
        }
        write_state(&e, State::Paused);
    }

    fn unpause(e: Env) {
        check_admin(&e, &Signature::Invoker);
        if read_state(&e) != State::Paused {
            panic_with_error!(&e, Error::NotPaused)
        }
        write_state(&e, State::Available);
    }
}

fn current_price(e: &Env) -> i128 {
    let mut end_time = read_end_time(&e);
    let now = e.ledger().timestamp();

    // If the end date has not passed yet
    if now < end_time {
        end_time = now;
    }

    let initial_price = read_price(&e);
    // Calculates the amount of time intervals that have passed
    let time = (end_time - read_init_time(&e)) / read_fee_interval(&e);

    // If no time interval has passed, the price does not change
    if time == 0 {
        return initial_price;
    }

    let fee_type = read_fee_type(e);
    match fee_type {
        InterestType::Simple => {
            return initial_price + (initial_price * (time as i128) * read_fee_rate(&e)) / 1000;
        }
        InterestType::Compound => {
            let fees = 1000 + read_fee_rate(&e);
            return initial_price * (fees.pow(time as u32)) / 1000_i128.pow(time as u32);
        }
    }
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

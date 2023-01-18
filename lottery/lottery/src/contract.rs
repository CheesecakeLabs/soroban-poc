use crate::context::State;
use crate::errors::Error;
use crate::services::*;
use soroban_auth::{Identifier, Signature};
use soroban_sdk::{contractimpl, panic_with_error, BytesN, Env};

pub mod token {
    soroban_sdk::contractimport!(file = "../soroban_token_contract.wasm");
}

pub trait LotteryTrait {
    fn initialize(
        env: Env,
        admin: Identifier,
        token_address: BytesN<32>,
        gain_percentage: i128,
        price: i128,
    );

    fn start(env: Env);

    fn buy_ticket(env: Env);

    fn end(env: Env);
}

pub struct Lottery;

#[contractimpl]
impl LotteryTrait for Lottery {
    fn initialize(
        env: Env,
        admin: Identifier,
        token_address: BytesN<32>,
        gain_percentage: i128,
        price: i128,
    ) {
        if read_state(&env) != State::NotInititd {
            panic_with_error!(&env, Error::AlreadyInitialized);
        }

        write_state(&env, State::Initiated);
        write_admin(&env, admin);
        write_token_address(&env, token_address);
        write_gain_percent(&env, gain_percentage);
        write_ticket_price(&env, price);
    }

    fn start(env: Env) {
        check_admin(&env, &Signature::Invoker);
        if read_state(&env) != State::Initiated {
            panic_with_error!(&env, Error::NotInitialized);
        }

        write_state(&env, State::Started);
    }

    fn buy_ticket(env: Env) {
        if read_state(&env) != State::Started {
            panic_with_error!(&env, Error::NotStarted);
        }
        let invoker: Identifier = env.invoker().into();

        transfer_ticket_price_from_account_to_contract(&env, &invoker.clone());
        write_user(&env, invoker)
    }

    fn end(env: Env) {
        check_admin(&env, &Signature::Invoker);
        if read_state(&env) != State::Started {
            panic_with_error!(&env, Error::NotStarted);
        }

        let users = read_users(&env);
        let winner_index = get_random_winner_index(&env, users.len().into());

        let prize = get_contract_balance(&env);
        let admin_prize = (prize * read_admin_gain_percent(&env)) / 100;
        let user_prize = prize - admin_prize;

        let user: Identifier = users.get_unchecked(winner_index).unwrap();

        transfer_from_contract_to_account(&env, &user, &user_prize);
        transfer_from_contract_to_account(&env, &env.invoker().into(), &admin_prize);

        clear_users(&env);
        write_state(&env, State::Finished);
    }
}

fn transfer_from_contract_to_account(env: &Env, to: &Identifier, amount: &i128) {
    let token_client = token::Client::new(env, read_token_address(&env));
    token_client.xfer(&Signature::Invoker, &0, to, amount);
}

fn transfer_ticket_price_from_account_to_contract(env: &Env, from: &Identifier) {
    let ticket_price = read_ticket_price(&env);

    let token_client = token::Client::new(env, read_token_address(&env));
    token_client.xfer_from(
        &Signature::Invoker,
        &0,
        from,
        &Identifier::Contract(env.current_contract()),
        &ticket_price,
    );
}

fn get_contract_balance(env: &Env) -> i128 {
    let token_client = token::Client::new(env, read_token_address(&env));
    token_client.balance(&Identifier::Contract(env.current_contract()))
}

fn get_random_winner_index(env: &Env, users_len: i128) -> u32 {
    let timestamp: u64 = env.ledger().timestamp();
    let timestamp_raw_ptr = &timestamp as *const u64;
    let addr = timestamp_raw_ptr as i128;

    let nonce = (addr - users_len) * timestamp as i128;

    (nonce % users_len) as u32
}

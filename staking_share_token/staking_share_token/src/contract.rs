use crate::errors::Error;
use crate::metadata::{
    decrease_total_share, decrease_total_supply, has_rate, increase_total_share,
    increase_total_supply, read_last_updated, read_rate, read_share_token, read_staking_token,
    read_total_share, read_total_supply, write_last_updated, write_rate, write_share_token,
    write_staking_token,
};
use num_integer::Roots;
use soroban_sdk::{contractimpl, panic_with_error, Bytes, BytesN, Env};
pub mod token {
    soroban_sdk::contractimport!(file = "../soroban_token_spec.wasm");
}
use token::{Identifier, Signature, TokenMetadata};

pub trait StakingTrait {
    // Saves the contract IDs of Staking and Share tokens
    fn initialize(
        e: Env,
        staking_token_id: BytesN<32>,
        rate: i128,
        share_token_name: Bytes,
        share_token_symbol: Bytes,
        share_token_decimals: u32,
    );

    // Deposits tokens into this contract
    // amount: how much to stake
    fn stake(e: Env, amount: i128);

    // Withdraw all tokens from this contract
    fn withdraw(e: Env);

    // Get the total Staking tokens
    fn get_rsrvs(e: Env) -> i128;

    // Get the user balance
    // user: Identifier of the user
    fn get_stkd(e: Env, user: Identifier) -> i128;

    // Get Token Share contract ID
    fn share_id(e: Env) -> BytesN<32>;
}

pub struct Staking;

#[contractimpl]
impl StakingTrait for Staking {
    fn initialize(
        e: Env,
        staking_token_id: BytesN<32>,
        rate: i128,
        share_token_name: Bytes,
        share_token_symbol: Bytes,
        share_token_decimals: u32,
    ) {
        if has_rate(&e) {
            panic_with_error!(&e, Error::AlreadyInitialized)
        }

        let share_contract_id = create_contract(&e);
        token::Client::new(&e, share_contract_id.clone()).init(
            &Identifier::Contract(e.get_current_contract()),
            &TokenMetadata {
                name: share_token_name,
                symbol: share_token_symbol,
                decimals: share_token_decimals,
            },
        );

        write_rate(&e, rate);
        write_staking_token(&e, staking_token_id);
        write_share_token(&e, share_contract_id);
    }

    fn stake(e: Env, amount: i128) {
        refresh_total_supply(&e);

        let invoker = &e.invoker();
        let token_id = read_staking_token(&e);
        let share_amount = calculate_share_tokens_amount(&e, &amount);

        increase_total_supply(&e, amount);
        increase_total_share(&e, share_amount);

        transfer_from_account_to_contract(&e, &token_id, &invoker.clone().into(), &amount);
        mint_share_tokens(&e, &invoker.clone().into(), &share_amount)
    }

    fn withdraw(e: Env) {
        refresh_total_supply(&e);

        let invoker = &e.invoker();

        let share_amount = share_balance(&e, &invoker.clone().into());
        let staking_amount = calculate_staking_tokens_amount(&e, &share_amount);

        decrease_total_share(&e, share_amount);
        decrease_total_supply(&e, staking_amount); // HOW TO GET THE TOTAL USER TOKEN IN THE CONTRACT?

        burn_share_tokens(&e, &invoker.clone().into(), &share_amount);
        transfer_from_contract_to_account(
            &e,
            &read_staking_token(&e),
            &invoker.clone().into(),
            &(staking_amount),
        )
    }

    fn get_rsrvs(e: Env) -> i128 {
        refresh_total_supply(&e);

        read_total_supply(&e)
    }

    fn get_stkd(e: Env, user: Identifier) -> i128 {
        refresh_total_supply(&e);

        let share_amount = share_balance(&e, &user);
        calculate_staking_tokens_amount(&e, &share_amount)
    }

    fn share_id(e: Env) -> BytesN<32> {
        read_share_token(&e)
    }
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

fn transfer_from_contract_to_account(
    e: &Env,
    token_id: &BytesN<32>,
    to: &Identifier,
    amount: &i128,
) {
    let client = token::Client::new(e, token_id);
    client.xfer(&Signature::Invoker, &0, to, amount);
}

fn mint_share_tokens(e: &Env, to: &Identifier, amount: &i128) {
    let token_id = read_share_token(&e);
    let client = token::Client::new(e, token_id);
    client.mint(&Signature::Invoker, &0, to, &amount);
}

fn burn_share_tokens(e: &Env, from: &Identifier, amount: &i128) {
    let token_id = read_share_token(&e);
    let client = token::Client::new(e, token_id);
    client.burn(&Signature::Invoker, &0, from, &amount);
}

fn share_balance(e: &Env, id: &Identifier) -> i128 {
    let token_id = read_share_token(&e);
    let client = token::Client::new(e, token_id);
    client.balance(id)
}

fn calculate_share_tokens_amount(e: &Env, amount: &i128) -> i128 {
    let total_supply = read_total_supply(&e);
    if total_supply > 0 {
        let total_share = read_total_share(&e);
        (amount * total_share) / total_supply
    } else {
        get_init_share(&e, amount).sqrt()
    }
}

fn get_init_share(e: &Env, amount: &i128) -> i128 {
    amount * 10_i128.pow(get_share_decimals(&e))
}

fn get_share_decimals(e: &Env) -> u32 {
    let token_id = read_share_token(&e);
    let client = token::Client::new(e, token_id);
    client.decimals()
}

fn create_contract(e: &Env) -> BytesN<32> {
    let salt = Bytes::new(e);
    let salt = e.crypto().sha256(&salt);
    e.deployer().with_current_contract(salt).deploy_token()
}

fn calculate_staking_tokens_amount(e: &Env, share_amount: &i128) -> i128 {
    let total_supply = read_total_supply(e);
    let total_share = read_total_share(e);
    (share_amount * total_supply) / total_share
}

fn refresh_total_supply(e: &Env) {
    let time_now = e.ledger().timestamp();
    let time_diff: i128 = (time_now - read_last_updated(&e)).into();

    let new_tokens = time_diff * read_rate(&e);

    increase_total_supply(&e, new_tokens);
    write_last_updated(&e, time_now);
}

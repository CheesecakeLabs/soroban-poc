// See this contract in Solidity: https://github.com/smartcontractkit/defi-minimal/blob/main/contracts/Staking.sol

// Tips:
// Get the current timestamp: env.ledger().timestamp();
// Get the current user with: let invoker = e.invoker().into();
use crate::errors::Error;
use crate::metadata::{
    has_rate, increase_total_share, increase_total_supply, read_share_token, read_staking_token,
    read_total_share, read_total_supply, write_rate, write_share_token, write_staking_token,
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

    // Get the user balance
    // user: Identifier of the user
    fn get_rsrvs(e: Env, user: Identifier) -> i128;

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
        let invoker = &e.invoker();
        let token_id = read_staking_token(&e);
        let share_amount = calculate_share_tokens_amount(&e, &amount);

        increase_total_supply(&e, amount);
        increase_total_share(&e, share_amount);

        transfer_from_account_to_contract(&e, &token_id, &invoker.clone().into(), &amount);
        mint_share_tokens(&e, &invoker.clone().into(), &share_amount)
    }

    fn withdraw(e: Env) {}

    fn get_rsrvs(e: Env, user: Identifier) -> i128 {
        return 0;
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

fn mint_share_tokens(e: &Env, to: &Identifier, amount: &i128) {
    let token_id = read_share_token(&e);
    let client = token::Client::new(e, token_id);
    client.mint(&Signature::Invoker, &0, to, &amount);
}

fn calculate_share_tokens_amount(e: &Env, amount: &i128) -> i128 {
    let total_supply = read_total_supply(&e);
    if total_supply > 0 {
        let total_share = read_total_share(&e);
        (amount * total_share) / total_supply
    } else {
        amount.sqrt()
    }
}

fn create_contract(e: &Env) -> BytesN<32> {
    let salt = Bytes::new(e);
    let salt = e.crypto().sha256(&salt);
    e.deployer().with_current_contract(salt).deploy_token()
}

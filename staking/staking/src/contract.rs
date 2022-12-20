// See this contract in Solidity: https://github.com/smartcontractkit/defi-minimal/blob/main/contracts/Staking.sol

// Tips:
// Get the current timestamp: env.ledger().timestamp();
// Get the current user with: let invoker = e.invoker().into();

use crate::errors::Error;
use crate::metadata::{
    decrease_balance, decrease_total_supply, get_balance, get_last_update, get_reward,
    get_reward_paid, get_reward_per_token_stored, get_reward_rate, get_rewards_token_contract_id,
    get_staking_token_contract_id, get_total_supply, has_reward_rate, increase_balance,
    increase_total_supply, write_last_update, write_reward, write_reward_paid,
    write_reward_per_token_stored, write_reward_rate, write_rewards_token, write_staking_token,
};
use soroban_sdk::{contractimpl, panic_with_error, BytesN, Env};
pub mod token {
    soroban_sdk::contractimport!(file = "../soroban_token_spec.wasm");
}
use token::{Identifier, Signature};

// Contract usage:
// 1. The contract admin calls: initialize(staking_token_id, rewards_token_id, 100)
// 2. The contract must have an appropriate amount of Rewards tokens in it
// 3. User calls `token.approve(user_auth, staking_contract, 100)`
//    to allow contract to stake the needed amount of token.
// 4. User calls `stake(100)`. Contract stakes the provided
//    Staking token amount and stores it
// 5. User calls `withdraw(100)` ten seconds later and
//    receives back his 100 Staking tokens
// 6. User calls `claim_rwrd()` and receives 1000 Rewards tokens
pub trait StakingTrait {
    // Saves the contract IDs of Staking and Rewards tokens
    // staking_token_id: Staking token contract ID
    // rewards_token_id: Reward token contract ID
    // rate: Rate of Reward tokens distributed to users per second
    fn initialize(e: Env, staking_token_id: BytesN<32>, rewards_token_id: BytesN<32>, rate: i128);

    // Deposits tokens into this contract
    // amount: Amount of tokens to stake
    fn stake(e: Env, amount: i128);

    // Withdraw tokens from this contract
    // amount: Amount of tokens to withdraw
    fn withdraw(e: Env, amount: i128);

    // User claims their tokens
    fn claim_rwrd(e: Env);

    // How much reward a user has eanerd
    // user: Identifier of the user
    fn earned(e: Env, user: Identifier) -> i128;

    // Get the user balance
    // user: Identifier of the user
    fn get_staked(e: Env, user: Identifier) -> i128;

    // Get the total supply of this contract
    fn tot_supply(e: Env) -> i128;
}

pub struct Staking;
#[contractimpl]
impl StakingTrait for Staking {
    fn initialize(e: Env, staking_token_id: BytesN<32>, rewards_token_id: BytesN<32>, rate: i128) {
        if has_reward_rate(&e) {
            panic_with_error!(&e, Error::AlreadyInitialized)
        }

        write_reward_rate(&e, rate);
        write_staking_token(&e, staking_token_id);
        write_rewards_token(&e, rewards_token_id);
    }

    fn stake(e: Env, amount: i128) {
        check_amount(&e, amount);

        let invoker = &e.invoker();

        update_reward(&e, invoker.clone().into());

        increase_total_supply(&e, amount);
        increase_balance(&e, invoker.clone().into(), amount);

        let token_id = get_staking_token_contract_id(&e);
        transfer_from_account_to_contract(&e, &token_id, &invoker.clone().into(), &amount);
    }

    fn withdraw(e: Env, amount: i128) {
        check_amount(&e, amount);

        let invoker = &e.invoker();

        if amount > get_balance(&e, invoker.clone().into()) {
            panic_with_error!(&e, Error::InsufficientBalance)
        }

        update_reward(&e, invoker.clone().into());

        decrease_total_supply(&e, amount);
        decrease_balance(&e, invoker.clone().into(), amount);

        let token_id = get_staking_token_contract_id(&e);
        transfer_from_contract_to_account(&e, &token_id, &invoker.clone().into(), &amount);
    }

    fn claim_rwrd(e: Env) {
        let invoker = &e.invoker();

        let reward = get_reward(&e, invoker.clone().into());
        write_reward(&e, invoker.clone().into(), 0);

        let token_id = get_rewards_token_contract_id(&e);
        transfer_from_contract_to_account(&e, &token_id, &invoker.clone().into(), &reward);

        update_reward(&e, invoker.clone().into());
    }

    fn earned(e: Env, user: Identifier) -> i128 {
        (get_balance(&e, user.clone())
            * (get_reward_per_token_stored(&e) - get_reward_paid(&e, user.clone())))
            + get_reward(&e, user.clone())
    }

    fn get_staked(e: Env, user: Identifier) -> i128 {
        get_balance(&e, user)
    }

    fn tot_supply(e: Env) -> i128 {
        get_total_supply(&e)
    }
}

fn reward_per_token(e: &Env) -> i128 {
    let reward_per_token_stored = get_reward_per_token_stored(&e);
    let total_supply = get_total_supply(&e);

    if total_supply == 0 {
        return reward_per_token_stored;
    }

    let time_diff: i128 = (e.ledger().timestamp() - get_last_update(&e)).into();
    return reward_per_token_stored + ((time_diff * get_reward_rate(&e)) / total_supply);
}

fn update_reward(e: &Env, user: Identifier) {
    let reward_per_token = reward_per_token(&e);

    write_reward_per_token_stored(&e, reward_per_token);
    write_last_update(&e, e.ledger().timestamp());
    write_reward(&e, user.clone(), Staking::earned(e.clone(), user.clone()));
    write_reward_paid(&e, user.clone(), reward_per_token)
}

fn check_amount(e: &Env, amount: i128) {
    if amount <= 0 {
        panic_with_error!(&e, Error::AmountMustBeGreaterThanZero)
    }
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

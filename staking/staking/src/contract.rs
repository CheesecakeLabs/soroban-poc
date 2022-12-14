// See this contract in Solidity: https://github.com/smartcontractkit/defi-minimal/blob/main/contracts/Staking.sol

// Tips:
// Get the current timestamp: env.ledger().timestamp();
// Get the current user with: let invoker = e.invoker().into();

use crate::errors::Error;
use crate::metadata::{
    has_reward_rate, read_account_rewards, read_balance, read_last_update, read_reward_rate,
    read_reward_stored, read_staking_token, read_total_supply, update_account_rewards,
    update_balance, update_total_supply, write_balance, write_last_update, write_reward_rate,
    write_reward_stored, write_rewards_token, write_staking_token, write_total_supply, write_account_rewards, write_reward_paid,
};
// use soroban_auth::Identifier;
use soroban_sdk::{contractimpl, panic_with_error, BytesN, Env};
pub mod token {
    soroban_sdk::contractimport!(file = "../soroban_token_spec.wasm");
}
use token::{Identifier, Signature};

pub trait StakingTrait {
    // Saves the contract IDs of Staking and Rewards tokens
    fn initialize(e: Env, staking_token_id: BytesN<32>, rewards_token_id: BytesN<32>, rate: i128);

    // Deposits tokens into this contract
    // amount: how much to stake
    fn stake(e: Env, amount: i128);

    // Withdraw tokens from this contract
    // amount: how much to withdraw
    fn withdraw(e: Env, amount: i128);

    // User claims their tokens
    fn claim_rwrd(e: Env);

    // How much reward a user has eanerd
    // user: Identifier of the user
    fn earned(e: Env, user: Identifier) -> i128;

    // Get the user balance
    // user: Identifier of the user
    fn get_staked(e: Env, user: Identifier) -> i128;

    // update the reward
    fn up_reward(e: Env, identifier: Identifier);

    fn per_token(e: Env) -> i128;

    fn rd_supply(e: Env) -> i128;

    fn rd_balance(e: Env, identifier: Identifier) -> i128;
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
        // check if amount is valid
        if amount <= 0 {
            panic_with_error!(&e, Error::InvalidAmount)
        }
        //  get the staking token
        let token = read_staking_token(&e);

        // get the invoker
        let invoker = e.invoker();

        // call the token contract
        let client_a = token::Client::new(&e, token);

        // // transfer the tokens to this contract from the user account
        client_a.xfer_from(
            &Signature::Invoker,
            &0,
            &invoker.clone().into(),
            &Identifier::Contract(e.current_contract()),
            &amount,
        );
        //  how to check that this transfer was successful?
        // if !success {
        //     panic_with_error!(&e, Error::TransferFailed)
        // }
        // update the total supply of the contract
        update_total_supply(&e, amount);

        // update the user balance in this contract
        update_balance(&e, invoker.clone().into(), amount);

        // give the rewards
        Staking::up_reward(e, invoker.clone().into());
    }

    fn withdraw(e: Env, amount: i128) {
        if amount <= 0 {
            panic_with_error!(&e, Error::InvalidAmount)
        }
        //  get the staking token
        let token = read_staking_token(&e);

        // get the invoker
        let invoker = e.invoker();
        // call the token contract
        let client_a = token::Client::new(&e, token);
        client_a.xfer(
            &Signature::Invoker,
            &0,
            &invoker.clone().into(),
            &amount,
        );

        // get the user balance
        let balance = read_balance(&e, invoker.clone().into());
        // write the user balance less the amount
        write_balance(&e, invoker.clone().into(), balance - amount);

        // get the contract total supply
        let total_supply = read_total_supply(&e);
        write_total_supply(&e, total_supply - amount);
    }

    fn claim_rwrd(e: Env) {
        let invoker = e.invoker();
        let reward = read_account_rewards(&e, invoker.clone().into());
        write_account_rewards(&e, invoker.clone().into(),0);
        let token = read_staking_token(&e);
        let client_a = token::Client::new(&e, token);
        client_a.xfer(
            &Signature::Invoker,
            &0,
            &Signature::Invoker.identifier(&e),
            &reward,
        );
        Staking::up_reward(e, invoker.clone().into());
    }

    fn earned(e: Env, user: Identifier) -> i128 {
        let current_balance = read_balance(&e, user.clone());
        // how much they have been paid already
        let amount_paid = read_account_rewards(&e, user.clone());
        let current_reward_per_token = Staking::per_token(e);
        let past_rewards = 0; // TODO WHAT IS THIS VALUE

        let earned = (current_balance * (current_reward_per_token - amount_paid)) + past_rewards;
        return earned;
    }

    fn get_staked(e: Env, identifier: Identifier) -> i128 {
        return read_balance(&e, identifier);
    }

    // calculate the rewards per token

    fn up_reward(e: Env, identifier: Identifier) {
        write_reward_stored(&e, Staking::per_token(e.clone()));
        write_last_update(&e.clone(),  e.clone().ledger().timestamp());
        update_account_rewards(
            &e.clone(),
            identifier.clone(),
            Staking::earned(e.clone(), identifier.clone()),
        );
        write_reward_paid(&e, identifier.clone(), read_reward_stored(&e));

    }

    fn per_token(e: Env) -> i128 {
        let last_update = read_last_update(&e);
        let total_supply = read_total_supply(&e);
        let reward_rate = read_reward_rate(&e);
        let reward_stored = read_reward_stored(&e);
        if total_supply == 0 {
            return reward_stored;
        }

        return reward_stored
            + (e.ledger().timestamp() - last_update) as i128 * reward_rate / total_supply;
    }

    fn rd_supply(e: Env) -> i128 {
        let total_supply = read_total_supply(&e);
        return total_supply;
    }
    fn rd_balance(e: Env, user: Identifier) -> i128 {
        let balance = read_balance(&e, user);
        return balance;
    }
}

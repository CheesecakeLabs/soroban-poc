// See this contract in Solidity: https://github.com/smartcontractkit/defi-minimal/blob/main/contracts/Staking.sol

// Tips:
// Get the current timestamp: env.ledger().timestamp();
// Get the current user with: let invoker = e.invoker().into();

use crate::errors::Error;
use crate::metadata::{
    has_reward_rate, write_reward_rate, write_rewards_token, write_staking_token,
};
use soroban_sdk::{contractimpl, panic_with_error, BytesN, Env};
pub mod token {
    soroban_sdk::contractimport!(file = "../soroban_token_spec.wasm");
}
use token::{Identifier, Signature};

pub trait StakingTrait {
    // Saves the contract IDs of Staking and Rewards tokens
    fn initialize(e: Env, staking_token_id: BytesN<32>, rewards_token_id: BytesN<32>, rate: u64);

    // Deposits tokens into this contract
    // amount: how much to stake
    fn stake(e: Env, amount: u64);

    // Withdraw tokens from this contract
    // amount: how much to withdraw
    fn withdraw(e: Env, amount: u64);

    // User claims their tokens
    fn claim_rwrd(e: Env);

    // How much reward a user has eanerd
    // user: Identifier of the user
    fn earned(e: Env, user: Identifier) -> u64;

    // Get the user balance
    // user: Identifier of the user
    fn get_staked(e: Env, user: Identifier) -> u64;
}

pub struct Staking;

#[contractimpl]
impl StakingTrait for Staking {
    fn initialize(e: Env, staking_token_id: BytesN<32>, rewards_token_id: BytesN<32>, rate: u64) {
        if has_reward_rate(&e) {
            panic_with_error!(&e, Error::AlreadyInitialized)
        }

        write_reward_rate(&e, rate);
        write_staking_token(&e, staking_token_id);
        write_rewards_token(&e, rewards_token_id);
    }

    fn stake(e: Env, amount: u64) {}

    fn withdraw(e: Env, amount: u64) {}

    fn claim_rwrd(e: Env) {}

    fn earned(e: Env, user: Identifier) -> u64 {
        return 0;
    }

    fn get_staked(e: Env, user: Identifier) -> u64 {
        return 0;
    }
}

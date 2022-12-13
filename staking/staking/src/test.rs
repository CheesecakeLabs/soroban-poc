#![cfg(test)]
use crate::contract::{token, Staking};
use crate::testutils::StakingTest;

use soroban_auth::{Identifier, Signature};
use soroban_sdk::{testutils::Accounts, AccountId, BytesN, Env, IntoVal};
use token::{Client as TokenClient, TokenMetadata};

fn create_token_contract(
    e: &Env,
    admin: &AccountId,
    name: &str,
    symbol: &str,
    decimals: u32,
) -> (BytesN<32>, TokenClient) {
    let id = e.register_contract_token(None);
    let token = TokenClient::new(e, &id);

    token.init(
        &Identifier::Account(admin.clone()),
        &TokenMetadata {
            name: name.into_val(e),
            symbol: symbol.into_val(e),
            decimals: decimals,
        },
    );

    (id, token)
}

#[test]
fn test_success() {
    let e: Env = Default::default();

    let stk_token_admin = e.accounts().generate();
    let rwrd_token_admin = e.accounts().generate();
    let user1 = e.accounts().generate();
    let user2 = e.accounts().generate();
    let user1_id = Identifier::Account(user1.clone());
    let user2_id = Identifier::Account(user2.clone());

    let (stk_token_id, stk_token) =
        create_token_contract(&e, &stk_token_admin, &"Staking", &"STK", 8);
    let (rwrd_token_id, rwrd_token) =
        create_token_contract(&e, &rwrd_token_admin, &"Reward", &"RWRD", 8);

    let contract_id = e.register_contract(None, Staking);
    let staking = StakingTest::new(&e, &contract_id);
    let staking_id = Identifier::Contract(contract_id.clone());

    // Start the staking contract
    staking.initialize(stk_token_id, rwrd_token_id, 100);

    // Users approve the contract to transfer their Staking tokens
    stk_token
        .with_source_account(&user1)
        .approve(&Signature::Invoker, &0, &staking_id, &1000);
    stk_token
        .with_source_account(&user2)
        .approve(&Signature::Invoker, &0, &staking_id, &3000);

    // Reward token admin mint some tokens for the contract
    rwrd_token.with_source_account(&rwrd_token_admin).mint(
        &Signature::Invoker,
        &0,
        &staking_id,
        &1000000,
    );
}

#![cfg(test)]
use crate::contract::{token, Staking, StakingClient};

use soroban_auth::{Identifier, Signature};
use soroban_sdk::testutils::{Accounts, Ledger, LedgerInfo};
use soroban_sdk::{ AccountId, BytesN, Env, IntoVal};
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

fn updates_staking_contract_time(e: &Env, contract_id: BytesN<32>, time: u64) -> StakingClient {
    e.ledger().set(LedgerInfo {
        timestamp: time,
        protocol_version: 1,
        sequence_number: 10,
        network_passphrase: Default::default(),
        base_reserve: 10,
    });
    return StakingClient::new(&e, &contract_id);
}

#[test]
fn test_success() {
    let e: Env = Default::default();

    let stk_token_admin = e.accounts().generate();
    let user1 = e.accounts().generate();
    let user2 = e.accounts().generate();
    let user3 = e.accounts().generate();
    let user1_id = Identifier::Account(user1.clone());
    let user2_id = Identifier::Account(user2.clone());
    let user3_id = Identifier::Account(user3.clone());

    let (stk_token_id, stk_token) =
        create_token_contract(&e, &stk_token_admin, &"Staking", &"STK", 8);

    let contract_id = e.register_contract(None, Staking);
    let staking = updates_staking_contract_time(&e, contract_id.clone(), 0);
    let staking_id = Identifier::Contract(contract_id.clone());

    // Start the staking contract
    staking.initialize(
        &stk_token_id,
        &1,
        &"Erico".into_val(&e),
        &"ERC".into_val(&e),
        &8,
    );

    let share_token = TokenClient::new(&e, &staking.share_id());

    // Users approve the contract to transfer their Staking tokens
    stk_token
        .with_source_account(&user1)
        .approve(&Signature::Invoker, &0, &staking_id, &1000);
    stk_token
        .with_source_account(&user2)
        .approve(&Signature::Invoker, &0, &staking_id, &3000);
    stk_token
        .with_source_account(&user3)
        .approve(&Signature::Invoker, &0, &staking_id, &1000);

    // Staking token admin mint some tokens for the users
    stk_token
        .with_source_account(&stk_token_admin)
        .mint(&Signature::Invoker, &0, &user1_id, &1000);
    stk_token
        .with_source_account(&stk_token_admin)
        .mint(&Signature::Invoker, &0, &user2_id, &3000);
    stk_token
        .with_source_account(&stk_token_admin)
        .mint(&Signature::Invoker, &0, &user3_id, &1000);

    // Time 0
    // User 1 stakes 500 tokens and receive 22 share tokens
    staking.with_source_account(&user1).stake(&500);
    assert_eq!(stk_token.balance(&user1_id), 500);
    assert_eq!(share_token.balance(&user1_id), 22);

    // User 2 stakes 250 tokens and receive 11 share tokens
    staking.with_source_account(&user2).stake(&250);
    assert_eq!(stk_token.balance(&user2_id), 2750);
    assert_eq!(share_token.balance(&user2_id), 11);

    assert_eq!(staking.get_rsrvs(), 750);
    
    // Time 100
    // updating to time 100
    updates_staking_contract_time(&e, contract_id.clone(), 100);
    // User 3 makes a deposit 1000
    staking.with_source_account(&user3).stake(&1000);
    
    assert_eq!(stk_token.balance(&user3_id), 0);
    assert_eq!(share_token.balance(&user3_id), 38);
    assert_eq!(staking.get_rsrvs(), 1850);
    assert_eq!(staking.get_stkd(&user1_id), 573);
    assert_eq!(staking.get_stkd(&user2_id), 283);
    assert_eq!(staking.get_stkd(&user3_id), 1000);

    // Time 200
    updates_staking_contract_time(&e, contract_id.clone(), 200);
    // User 1 withdraw and receives 604 tokens
    staking.with_source_account(&user1).withdraw();
    assert_eq!(stk_token.balance(&user1_id), 1104);
    assert_eq!(share_token.balance(&user1_id), 0);
    assert_eq!(staking.get_stkd(&user2_id), 302);
    assert_eq!(staking.get_stkd(&user3_id), 1043);
    assert_eq!(staking.get_rsrvs(), 49);
}

#![cfg(test)]
use crate::contract::{token, Staking, StakingClient};

use soroban_auth::{Identifier, Signature};
use soroban_sdk::testutils::{Accounts, Ledger, LedgerInfo};
use soroban_sdk::{AccountId, BytesN, Env, IntoVal};
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
    let rwrd_token_admin = e.accounts().generate();
    let user1 = e.accounts().generate();
    let user2 = e.accounts().generate();
    let user3 = e.accounts().generate();
    let user1_id = Identifier::Account(user1.clone());
    let user2_id = Identifier::Account(user2.clone());
    let user3_id = Identifier::Account(user3.clone());

    let (stk_token_id, stk_token) =
        create_token_contract(&e, &stk_token_admin, &"Staking", &"STK", 8);
    let (rwrd_token_id, rwrd_token) =
        create_token_contract(&e, &rwrd_token_admin, &"Reward", &"RWRD", 8);

    let contract_id = e.register_contract(None, Staking);
    let mut staking = updates_staking_contract_time(&e, contract_id.clone(), 0);
    let staking_id = Identifier::Contract(contract_id.clone());

    // Start the staking contract
    staking.initialize(&stk_token_id, &rwrd_token_id, &100);

    // Users approve the contract to transfer their Staking tokens
    stk_token
        .with_source_account(&user1)
        .approve(&Signature::Invoker, &0, &staking_id, &1000);

    stk_token
        .with_source_account(&user2)
        .approve(&Signature::Invoker, &0, &staking_id, &3000);

    stk_token
        .with_source_account(&user3)
        .approve(&Signature::Invoker, &0, &staking_id, &2000);

    // Reward token admin mint some tokens for the contract
    rwrd_token.with_source_account(&rwrd_token_admin).mint(
        &Signature::Invoker,
        &0,
        &staking_id,
        &1000000,
    );

    // Mint some staking tokens to users
    stk_token
        .with_source_account(&stk_token_admin)
        .mint(&Signature::Invoker, &0, &user1_id, &1000);
    stk_token
        .with_source_account(&stk_token_admin)
        .mint(&Signature::Invoker, &0, &user2_id, &5000);
    stk_token
        .with_source_account(&stk_token_admin)
        .mint(&Signature::Invoker, &0, &user3_id, &2000);

    // Check the earnings
    assert_eq!(staking.earned(&user1_id), 0);
    assert_eq!(staking.earned(&user2_id), 0);
    assert_eq!(staking.earned(&user3_id), 0);

    //
    // FIRST SCENARIO:
    //
    // Time = 0
    // User 1 stakes 100 tokens
    // User 1: 100 staked, Earned: 0
    // Total supply = 100
    staking.with_source_account(&user1.clone()).stake(&100);

    // Check the earnings, staked tokens and balances
    assert_eq!(staking.earned(&user1_id), 0);
    assert_eq!(staking.get_staked(&user1_id), 100);
    assert_eq!(stk_token.balance(&user1_id), 900);
    assert_eq!(staking.tot_supply(), 100);

    // Time = 10
    // User 1: 0 staked, Earned: 100 * 100 * 10 / 100 = 1000, Withdraw: 100
    // Total supply = 0
    staking = updates_staking_contract_time(&e, contract_id.clone(), 10);

    staking.with_source_account(&user1.clone()).withdraw(&100);
    assert_eq!(staking.earned(&user1_id), 1000);
    assert_eq!(staking.get_staked(&user1_id), 0);
    assert_eq!(stk_token.balance(&user1_id), 1000);
    assert_eq!(staking.tot_supply(), 0);

    // User 1 claims his rewards
    staking.with_source_account(&user1.clone()).claim_rwrd();
    assert_eq!(staking.earned(&user1_id), 0);
    assert_eq!(rwrd_token.balance(&user1_id), 1000);

    //
    // SECOND SCENARIO:
    //
    // Time = 0
    // User 1: stakes 80 tokens, User 2 stakes 20 tokens
    // User 1: 80 staked, Earned: 0
    // User 2: 20 staked, Earned: 0
    // Total supply = 100
    staking = updates_staking_contract_time(&e, contract_id.clone(), 0);

    staking.with_source_account(&user1.clone()).stake(&80);
    staking.with_source_account(&user2.clone()).stake(&20);
    assert_eq!(staking.earned(&user1_id), 0);
    assert_eq!(staking.earned(&user2_id), 0);
    assert_eq!(staking.tot_supply(), 100);

    // Time = 3
    // User 3 stakes 100 tokens
    // User 1: 80 staked, Earned: (80 * 100 * 3) / 100 = 240
    // User 2: 20 staked, Earned: (20 * 100 * 3) / 100 = 60
    // User 3: 100 staked, Earned: 0
    // Total supply = 200
    staking = updates_staking_contract_time(&e, contract_id.clone(), 3);

    staking.with_source_account(&user3.clone()).stake(&100);
    assert_eq!(staking.earned(&user1_id), 240);
    assert_eq!(staking.earned(&user2_id), 60);
    assert_eq!(staking.earned(&user3_id), 0);

    // Time = 5
    // User 3 withdraws 100 tokens
    // User 1: 80 staked, Earned: 240 + ((80 * 100 * 2) / 200) = 320
    // User 2: 20 staked, Earned: 60 + ((20 * 100 * 2) / 200) = 80
    // User 3: 0 staked, Earned: (100 * 100 * 2) / 200 = 100
    // Total supply = 100
    staking = updates_staking_contract_time(&e, contract_id.clone(), 5);

    staking.with_source_account(&user3.clone()).withdraw(&100);

    assert_eq!(staking.earned(&user1_id), 320);
    assert_eq!(staking.earned(&user2_id), 80);
    assert_eq!(staking.earned(&user3_id), 100);
    assert_eq!(staking.tot_supply(), 100);

    // Time = 7
    // User 1 stakes 100 tokens
    // User 1: 180 staked, Earned: 320 + ((80 * 100 * 2) / 100) = 480
    // User 2: 20 staked, Earned: 80 + ((20 * 100 * 2) / 100) = 120
    // User 3: 0 staked, Earned: 100
    // Total supply = 200
    staking = updates_staking_contract_time(&e, contract_id.clone(), 7);

    staking.with_source_account(&user1.clone()).stake(&100);

    assert_eq!(staking.earned(&user1_id), 480);
    assert_eq!(staking.earned(&user2_id), 120);
    assert_eq!(staking.earned(&user3_id), 100);
    assert_eq!(staking.tot_supply(), 200);

    // Time = 11
    // User 1 withdraws 180 tokens, User 2 withdraws 20 tokens
    // User 1: 80 staked, Earned: 480 + ((180 * 100 * 4) / 200) = 840
    // User 2: 20 staked, Earned: 120 + ((20 * 100 * 4) / 200) = 160
    // User 3: 0 staked, Earned: 100
    // Total supply = 0
    staking = updates_staking_contract_time(&e, contract_id.clone(), 11);

    staking.with_source_account(&user1.clone()).withdraw(&180);
    staking.with_source_account(&user2.clone()).withdraw(&20);

    assert_eq!(staking.earned(&user1_id), 840);
    assert_eq!(staking.earned(&user2_id), 160);
    assert_eq!(staking.earned(&user3_id), 100);
    assert_eq!(staking.tot_supply(), 0);

    // Users claims their rewards
    staking.with_source_account(&user1.clone()).claim_rwrd();
    staking.with_source_account(&user2.clone()).claim_rwrd();
    staking.with_source_account(&user3.clone()).claim_rwrd();

    assert_eq!(staking.earned(&user1_id), 0);
    assert_eq!(staking.earned(&user2_id), 0);
    assert_eq!(staking.earned(&user3_id), 0);
    assert_eq!(rwrd_token.balance(&user1_id), 1840);
    assert_eq!(rwrd_token.balance(&user2_id), 160);
    assert_eq!(rwrd_token.balance(&user3_id), 100);
    assert_eq!(stk_token.balance(&user1_id), 1000);
    assert_eq!(stk_token.balance(&user2_id), 5000);
    assert_eq!(stk_token.balance(&user3_id), 2000);
}

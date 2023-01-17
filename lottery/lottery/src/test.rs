#![cfg(test)]

use crate::context::{DataKey, State};
use crate::contract::{token, Lottery, LotteryClient};
use soroban_auth::{Identifier, Signature};
use soroban_sdk::testutils::Accounts;
use soroban_sdk::{vec, AccountId, BytesN, Env, IntoVal, Vec};

fn create_token_contract(
    env: &Env,
    admin: &AccountId,
    name: &str,
    symbol: &str,
    decimal: u32,
) -> (BytesN<32>, token::Client) {
    let token_contract_id = env.register_contract_wasm(None, token::WASM);
    let token_client = token::Client::new(env, token_contract_id.clone());

    token_client.initialize(
        &Identifier::Account(admin.clone()),
        &decimal,
        &name.into_val(env),
        &symbol.into_val(env),
    );

    (token_contract_id.clone(), token_client)
}

#[test]
#[should_panic(expected = "Status(ContractError(1)")]
fn test_contract_initialize_panics_when_contract_is_initialize() {
    let env = Env::default();
    let contract_id = env.register_contract(None, Lottery);
    let client = LotteryClient::new(&env, contract_id.clone());

    let admin = env.accounts().generate();
    let admin_id = Identifier::Account(admin.clone());
    let gain_percentage = 10;
    let price = 100;

    env.as_contract(&contract_id, || {
        env.storage().set(DataKey::State, State::Initiated)
    });

    client.initialize(&admin_id, &contract_id, &gain_percentage, &price);
}

#[test]
fn test_contract() {
    let env = Env::default();
    let contract_id = env.register_contract(None, Lottery);
    let contract_identifier = Identifier::Contract(contract_id.clone());

    let client = LotteryClient::new(&env, contract_id.clone());

    let admin = env.accounts().generate();
    let admin_id = Identifier::Account(admin.clone());

    let user1 = env.accounts().generate();
    let user1_id = Identifier::Account(user1.clone());
    let user2 = env.accounts().generate();
    let user2_id = Identifier::Account(user2.clone());

    let gain_percentage = 10;
    let price = 100;

    let (token_contract_id, token_client) =
        create_token_contract(&env, &admin, "CAIOTK", "CAIO", 7);

    token_client
        .with_source_account(&admin)
        .mint(&Signature::Invoker, &0, &user1_id, &100);
    token_client
        .with_source_account(&admin)
        .mint(&Signature::Invoker, &0, &user2_id, &100);

    token_client.with_source_account(&user1).incr_allow(
        &Signature::Invoker,
        &0,
        &contract_identifier.clone(),
        &100,
    );
    token_client.with_source_account(&user2).incr_allow(
        &Signature::Invoker,
        &0,
        &contract_identifier.clone(),
        &100,
    );

    // Test initialize
    client.initialize(&admin_id, &token_contract_id, &gain_percentage, &price);

    env.as_contract(&contract_id, || {
        let storage_price: i128 = env.storage().get_unchecked(DataKey::TicktPrice).unwrap();
        assert_eq!(storage_price, price);

        let storage_gain_percentage: i128 =
            env.storage().get_unchecked(DataKey::GainPctge).unwrap();
        assert_eq!(storage_gain_percentage, gain_percentage);

        let storage_state: State = env.storage().get_unchecked(DataKey::State).unwrap();
        assert_eq!(storage_state, State::Initiated);

        let storage_token_address: BytesN<32> =
            env.storage().get_unchecked(DataKey::TokenAddr).unwrap();
        assert_eq!(storage_token_address, token_contract_id);

        let storage_admin: Identifier = env.storage().get_unchecked(DataKey::Admin).unwrap();
        assert_eq!(storage_admin, admin_id);

        let storage_users: Vec<Identifier> = env
            .storage()
            .get(DataKey::Users)
            .unwrap_or(Ok(vec![&env]))
            .unwrap();
        assert_eq!(storage_users, vec![&env]);
    });

    // Test start
    client.with_source_account(&admin).start();

    env.as_contract(&contract_id, || {
        let storage_state: State = env.storage().get_unchecked(DataKey::State).unwrap();
        assert_eq!(storage_state, State::Started);
    });

    // Test buy ticket
    client.with_source_account(&user1).buy_ticket();

    env.as_contract(&contract_id, || {
        let storage_users: Vec<Identifier> = env.storage().get_unchecked(DataKey::Users).unwrap();
        assert_eq!(storage_users, vec![&env, user1_id.clone()]);
    });

    client.with_source_account(&user2).buy_ticket();

    env.as_contract(&contract_id, || {
        let storage_users: Vec<Identifier> = env.storage().get_unchecked(DataKey::Users).unwrap();
        assert_eq!(
            storage_users,
            vec![&env, user1_id.clone(), user2_id.clone()]
        );
    });

    // Check the balances
    assert_eq!(token_client.balance(&user1_id), 0);
    assert_eq!(token_client.balance(&user2_id), 0);
    assert_eq!(token_client.balance(&admin_id), 0);

    // Test end
    client.with_source_account(&admin).end();

    env.as_contract(&contract_id, || {
        let storage_state: State = env.storage().get_unchecked(DataKey::State).unwrap();
        assert_eq!(storage_state, State::Finished);

        let storage_users: Vec<Identifier> = env
            .storage()
            .get(DataKey::Users)
            .unwrap_or(Ok(vec![&env]))
            .unwrap();
        assert_eq!(storage_users, vec![&env]);
    });

    // Check the balances
    // Total prize: 200
    // User 1 = 0
    // User 2 = 90% 200 = 180
    // Admin = 10% 200 = 20
    assert_eq!(token_client.balance(&user1_id), 0);
    assert_eq!(token_client.balance(&user2_id), 180);
    assert_eq!(token_client.balance(&admin_id), 20);
}

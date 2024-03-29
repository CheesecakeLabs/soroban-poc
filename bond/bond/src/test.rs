#![cfg(test)]
use crate::contract::{token, Bond, BondClient};
use crate::storage_types::InterestType;
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

fn updates_contract_time(e: &Env, contract_id: BytesN<32>, time: u64) -> BondClient {
    e.ledger().set(LedgerInfo {
        timestamp: time,
        protocol_version: 1,
        sequence_number: 10,
        network_passphrase: Default::default(),
        base_reserve: 10,
    });
    return BondClient::new(&e, &contract_id);
}

fn days_to_seconds(days: u64) -> u64 {
    days * 24 * 60 * 60
}

fn generate_user_account(
    e: &Env,
    token_client: &TokenClient,
    token_admin: &AccountId,
    contract_id: &Identifier,
    amount: &i128,
) -> (AccountId, Identifier) {
    let user1 = e.accounts().generate();
    let user1_id = Identifier::Account(user1.clone());

    token_client.with_source_account(&user1).approve(
        &Signature::Invoker,
        &0,
        &contract_id,
        &amount,
    );

    token_client.with_source_account(&token_admin).mint(
        &Signature::Invoker,
        &0,
        &user1_id,
        &amount,
    );
    (user1, user1_id)
}

#[test]
fn test_success_with_compound_interest() {
    let e: Env = Default::default();

    let admin = e.accounts().generate();
    let admin_id = Identifier::Account(admin.clone());
    let payment_tkn_admin = e.accounts().generate();

    let (payment_tkn_id, payment_tkn) =
        create_token_contract(&e, &payment_tkn_admin, &"USD Coin", &"USDC", 8);

    let time = 0;
    let contract_id = e.register_contract(None, Bond);
    let mut contract = updates_contract_time(&e, contract_id.clone(), time);
    let contract_identifier = Identifier::Contract(contract_id.clone());

    let (user1, user1_id) = generate_user_account(
        &e,
        &payment_tkn,
        &payment_tkn_admin,
        &contract_identifier,
        &100000,
    );

    let (user2, user2_id) = generate_user_account(
        &e,
        &payment_tkn,
        &payment_tkn_admin,
        &contract_identifier,
        &100000,
    );

    let (user3, user3_id) = generate_user_account(
        &e,
        &payment_tkn,
        &payment_tkn_admin,
        &contract_identifier,
        &100000,
    );

    payment_tkn.with_source_account(&payment_tkn_admin).mint(
        &Signature::Invoker,
        &0,
        &admin_id,
        &200000,
    );

    // Initialize the contract
    contract.initialize(
        &admin_id.clone(),
        &payment_tkn_id,
        &"Bond".into_val(&e),
        &"BND".into_val(&e),
        &8,
        &100,
        &100, // 100 / 1000 = 0.1 => 10%
        &30,
        &InterestType::Compound,
        &10000,
    );

    let bond_tkn = TokenClient::new(&e, &contract.bond_id());

    assert_eq!(bond_tkn.balance(&contract_identifier), 10000);

    // Start the contract
    contract.with_source_account(&admin).start(&0);
    // Set the end date for 10 months from now (assuming 1 month = 30 days)
    contract
        .with_source_account(&admin)
        .set_end(&days_to_seconds(10 * 30));

    // Add users to white list
    contract.with_source_account(&admin).add_user(&user1_id);
    contract.with_source_account(&admin).add_user(&user2_id);
    contract.with_source_account(&admin).add_user(&user3_id);

    // Get current price
    assert_eq!(100, contract.get_price());

    // User 1 buy 200 Bond tokens with price 100
    contract.with_source_account(&admin).pause();
    contract.with_source_account(&admin).unpause();
    contract.with_source_account(&user1).buy(&200);
    assert_eq!(payment_tkn.balance(&user1_id), 80000);

    // Update time in 1 month
    contract = updates_contract_time(&e, contract_id.clone(), days_to_seconds(1 * 30));
    assert_eq!(110, contract.get_price());

    // Update time in 2 months (since start date)
    contract = updates_contract_time(&e, contract_id.clone(), days_to_seconds(2 * 30));

    // User 2 buy 100 tokens with price 121
    contract.with_source_account(&user2).buy(&100);
    assert_eq!(payment_tkn.balance(&user2_id), 87900);
    assert_eq!(121, contract.get_price());

    // Update time in 5 months (since start date)
    contract = updates_contract_time(&e, contract_id.clone(), days_to_seconds(5 * 30));

    // User 3 buy 200 tokens with price 161
    contract.with_source_account(&user3).buy(&200);
    assert_eq!(payment_tkn.balance(&user3_id), 67800);
    assert_eq!(161, contract.get_price());

    // Update time in 12 months (since start date)
    // Price must be 259 because the end date is 10 months after the start date
    contract = updates_contract_time(&e, contract_id.clone(), days_to_seconds(12 * 30));

    // Admin withdraws 20000 payment tokens
    assert_eq!(payment_tkn.balance(&contract_identifier), 64300);
    contract.with_source_account(&admin).withdraw(&20000);
    assert_eq!(payment_tkn.balance(&contract_identifier), 44300);

    // Admin transfer to the contract the missing amount to pay the users
    // supply * price = 500 * 259 = 129500
    payment_tkn.with_source_account(&admin).xfer(
        &Signature::Invoker,
        &0,
        &contract_identifier,
        &85200,
    );

    // Enable cash out
    contract.with_source_account(&admin).en_csh_out();

    // User 1 cash out
    // Must receive 200 * 259 = 51800
    contract.with_source_account(&user1).cash_out();
    assert_eq!(payment_tkn.balance(&user1_id), 131800);

    // User 2 cash out
    // Must receive 100 * 259 = 25900
    contract.with_source_account(&user2).cash_out();
    assert_eq!(payment_tkn.balance(&user2_id), 113800);

    // User 3 cash out
    // Must receive 200 * 259 = 51800
    contract.with_source_account(&user3).cash_out();
    assert_eq!(payment_tkn.balance(&user3_id), 119600);

    // Check the contract balance
    assert_eq!(payment_tkn.balance(&contract_identifier), 0);
}

#[test]
fn test_success_with_simple_interest() {
    let e: Env = Default::default();

    let admin = e.accounts().generate();
    let admin_id = Identifier::Account(admin.clone());
    let payment_tkn_admin = e.accounts().generate();

    let (payment_tkn_id, payment_tkn) =
        create_token_contract(&e, &payment_tkn_admin, &"USD Coin", &"USDC", 8);

    let time = 0;
    let contract_id = e.register_contract(None, Bond);
    let mut contract = updates_contract_time(&e, contract_id.clone(), time);
    let contract_identifier = Identifier::Contract(contract_id.clone());

    let (user1, user1_id) = generate_user_account(
        &e,
        &payment_tkn,
        &payment_tkn_admin,
        &contract_identifier,
        &100000,
    );

    let (user2, user2_id) = generate_user_account(
        &e,
        &payment_tkn,
        &payment_tkn_admin,
        &contract_identifier,
        &100000,
    );

    let (user3, user3_id) = generate_user_account(
        &e,
        &payment_tkn,
        &payment_tkn_admin,
        &contract_identifier,
        &100000,
    );

    payment_tkn.with_source_account(&payment_tkn_admin).mint(
        &Signature::Invoker,
        &0,
        &admin_id,
        &200000,
    );

    // Initialize the contract
    contract.initialize(
        &admin_id.clone(),
        &payment_tkn_id,
        &"Bond".into_val(&e),
        &"BND".into_val(&e),
        &8,
        &100,
        &100, // 100 / 1000 = 0.1 => 10%
        &30,
        &InterestType::Simple,
        &10000,
    );

    let bond_tkn = TokenClient::new(&e, &contract.bond_id());

    assert_eq!(bond_tkn.balance(&contract_identifier), 10000);

    // Start the contract
    contract.with_source_account(&admin).start(&0);
    // Set the end date for 10 months from now (assuming 1 month = 30 days)
    contract
        .with_source_account(&admin)
        .set_end(&days_to_seconds(10 * 30));

    // Add users to white list
    contract.with_source_account(&admin).add_user(&user1_id);
    contract.with_source_account(&admin).add_user(&user2_id);
    contract.with_source_account(&admin).add_user(&user3_id);

    // Get current price
    assert_eq!(100, contract.get_price());

    // User 1 buy 200 Bond tokens with price 100
    contract.with_source_account(&user1).buy(&200);
    assert_eq!(payment_tkn.balance(&user1_id), 80000);

    // Update time in 1 month
    contract = updates_contract_time(&e, contract_id.clone(), days_to_seconds(1 * 30));
    assert_eq!(110, contract.get_price());

    // Update time in 2 months (since start date)
    contract = updates_contract_time(&e, contract_id.clone(), days_to_seconds(2 * 30));

    // User 2 buy 100 tokens with price 120
    contract.with_source_account(&user2).buy(&100);
    assert_eq!(payment_tkn.balance(&user2_id), 88000);
    assert_eq!(120, contract.get_price());

    // Update time in 5 months (since start date)
    contract = updates_contract_time(&e, contract_id.clone(), days_to_seconds(5 * 30));

    // User 3 buy 200 tokens with price 150
    contract.with_source_account(&user3).buy(&200);
    assert_eq!(payment_tkn.balance(&user3_id), 70000);
    assert_eq!(150, contract.get_price());

    // Update time in 12 months (since start date)
    // Price must be 200 because the end date is 10 months after the start date
    contract = updates_contract_time(&e, contract_id.clone(), days_to_seconds(12 * 30));

    // Admin withdraws 20000 payment tokens
    assert_eq!(payment_tkn.balance(&contract_identifier), 62000);
    contract.with_source_account(&admin).withdraw(&20000);
    assert_eq!(payment_tkn.balance(&contract_identifier), 42000);

    // Admin transfer to the contract the missing amount to pay the users
    // supply * price = 500 * 200 = 100000
    payment_tkn.with_source_account(&admin).xfer(
        &Signature::Invoker,
        &0,
        &contract_identifier,
        &58000,
    );

    // Enable cash out
    contract.with_source_account(&admin).en_csh_out();

    // User 1 cash out
    // Must receive 200 * 200 = 40000
    contract.with_source_account(&user1).cash_out();
    assert_eq!(payment_tkn.balance(&user1_id), 120000);

    // User 2 cash out
    // Must receive 100 * 200 = 20000
    contract.with_source_account(&user2).cash_out();
    assert_eq!(payment_tkn.balance(&user2_id), 108000);

    // User 3 cash out
    // Must receive 200 * 200 = 40000
    contract.with_source_account(&user3).cash_out();
    assert_eq!(payment_tkn.balance(&user3_id), 110000);

    // Check the contract balance
    assert_eq!(payment_tkn.balance(&contract_identifier), 0);
}

#[test]
#[should_panic(expected = "Status(ContractError(4))")]
fn test_set_end_when_cash_out_enabled() {
    let e: Env = Default::default();

    let admin = e.accounts().generate();
    let admin_id = Identifier::Account(admin.clone());
    let payment_tkn_admin = e.accounts().generate();

    let (payment_tkn_id, _) =
        create_token_contract(&e, &payment_tkn_admin, &"USD Coin", &"USDC", 8);

    let contract_id = e.register_contract(None, Bond);
    let contract = updates_contract_time(&e, contract_id.clone(), 20);

    // Initialize the contract
    contract.initialize(
        &admin_id.clone(),
        &payment_tkn_id,
        &"Bond".into_val(&e),
        &"BND".into_val(&e),
        &8,
        &100,
        &10,
        &30,
        &InterestType::Compound,
        &10000,
    );

    // Start the contract
    contract.with_source_account(&admin).start(&10);
    // Set end time
    contract.with_source_account(&admin).set_end(&20);
    // Enable cash out
    contract.with_source_account(&admin).en_csh_out();
    // Set end time again
    contract.with_source_account(&admin).set_end(&20);
}

#[test]
#[should_panic(expected = "Status(ContractError(5))")]
fn test_set_end_invalid_timestamp() {
    let e: Env = Default::default();

    let admin = e.accounts().generate();
    let admin_id = Identifier::Account(admin.clone());
    let payment_tkn_admin = e.accounts().generate();

    let (payment_tkn_id, _) =
        create_token_contract(&e, &payment_tkn_admin, &"USD Coin", &"USDC", 8);

    let contract_id = e.register_contract(None, Bond);
    let contract = updates_contract_time(&e, contract_id.clone(), 0);

    // Initialize the contract
    contract.initialize(
        &admin_id.clone(),
        &payment_tkn_id,
        &"Bond".into_val(&e),
        &"BND".into_val(&e),
        &8,
        &100,
        &10,
        &30,
        &InterestType::Compound,
        &10000,
    );

    // Start the contract
    contract.with_source_account(&admin).start(&10);
    // Set end time lower than the initial time
    contract.with_source_account(&admin).set_end(&5);
}

#[test]
#[should_panic(expected = "Status(ContractError(7))")]
fn test_buy_not_available_paused() {
    let e: Env = Default::default();

    let admin = e.accounts().generate();
    let admin_id = Identifier::Account(admin.clone());
    let payment_tkn_admin = e.accounts().generate();

    let user1 = e.accounts().generate();
    let user1_id = Identifier::Account(user1.clone());

    let (payment_tkn_id, payment_tkn) =
        create_token_contract(&e, &payment_tkn_admin, &"USD Coin", &"USDC", 8);

    let time = 0;
    let contract_id = e.register_contract(None, Bond);
    let contract = updates_contract_time(&e, contract_id.clone(), time);
    let contract_identifier = Identifier::Contract(contract_id.clone());

    // Users approve the contract to transfer their payment tokens
    payment_tkn.with_source_account(&user1).approve(
        &Signature::Invoker,
        &0,
        &contract_identifier,
        &100000,
    );

    // Payment token admin mint some tokens for the users
    payment_tkn.with_source_account(&payment_tkn_admin).mint(
        &Signature::Invoker,
        &0,
        &user1_id,
        &100000,
    );
    payment_tkn.with_source_account(&payment_tkn_admin).mint(
        &Signature::Invoker,
        &0,
        &admin_id,
        &200000,
    );

    // Initialize the contract
    contract.initialize(
        &admin_id.clone(),
        &payment_tkn_id,
        &"Bond".into_val(&e),
        &"BND".into_val(&e),
        &8,
        &100,
        &100, // 100 / 1000 = 0.1 => 10%
        &30,
        &InterestType::Compound,
        &10000,
    );

    // Start the contract
    contract.with_source_account(&admin).start(&0);

    // Pause the contract
    contract.with_source_account(&admin).pause();

    // try to buy with contract paused
    contract.with_source_account(&user1).buy(&200);
}

#[test]
#[should_panic(expected = "Status(ContractError(11))")]
fn test_buy_with_user_not_allowed() {
    let e: Env = Default::default();
    let admin = e.accounts().generate();
    let admin_id = Identifier::Account(admin.clone());
    let payment_tkn_admin = e.accounts().generate();

    let user1 = e.accounts().generate();

    let (payment_tkn_id, _payment_tkn) =
        create_token_contract(&e, &payment_tkn_admin, &"USD Coin", &"USDC", 8);

    let time = 0;
    let contract_id = e.register_contract(None, Bond);
    let contract = updates_contract_time(&e, contract_id.clone(), time);

    // Initialize the contract
    contract.initialize(
        &admin_id.clone(),
        &payment_tkn_id,
        &"Bond".into_val(&e),
        &"BND".into_val(&e),
        &8,
        &100,
        &100, // 100 / 1000 = 0.1 => 10%
        &30,
        &InterestType::Compound,
        &10000,
    );

    // Start the contract
    contract.with_source_account(&admin).start(&0);

    // try to buy without be allowed
    contract.with_source_account(&user1).buy(&200);
}

#[test]
#[should_panic(expected = "Status(ContractError(4))")]
fn test_withdraw_when_cash_out_enabled() {
    let e: Env = Default::default();

    let admin = e.accounts().generate();
    let admin_id = Identifier::Account(admin.clone());
    let payment_tkn_admin = e.accounts().generate();

    let (payment_tkn_id, _payment_tkn) =
        create_token_contract(&e, &payment_tkn_admin, &"USD Coin", &"USDC", 8);

    let time = 20;
    let contract_id = e.register_contract(None, Bond);
    let contract = updates_contract_time(&e, contract_id.clone(), time);

    // Initialize the contract
    contract.initialize(
        &admin_id.clone(),
        &payment_tkn_id,
        &"Bond".into_val(&e),
        &"BND".into_val(&e),
        &8,
        &100,
        &100, // 100 / 1000 = 0.1 => 10%
        &30,
        &InterestType::Compound,
        &10000,
    );

    // Start the contract
    contract.with_source_account(&admin).start(&0);
    // Set end time
    contract.with_source_account(&admin).set_end(&20);
    // Enable cash out
    contract.with_source_account(&admin).en_csh_out();
    // Try withdraw
    contract.with_source_account(&admin).withdraw(&10);
}

#[test]
#[should_panic(expected = "Status(ContractError(6))")]
fn test_cash_out_when_not_enabled() {
    let e: Env = Default::default();

    let admin = e.accounts().generate();
    let admin_id = Identifier::Account(admin.clone());
    let payment_tkn_admin = e.accounts().generate();

    let (payment_tkn_id, _payment_tkn) =
        create_token_contract(&e, &payment_tkn_admin, &"USD Coin", &"USDC", 8);

    let user1 = e.accounts().generate();

    let time = 20;
    let contract_id = e.register_contract(None, Bond);
    let contract = updates_contract_time(&e, contract_id.clone(), time);

    // Initialize the contract
    contract.initialize(
        &admin_id.clone(),
        &payment_tkn_id,
        &"Bond".into_val(&e),
        &"BND".into_val(&e),
        &8,
        &100,
        &100, // 100 / 1000 = 0.1 => 10%
        &30,
        &InterestType::Compound,
        &10000,
    );

    // Start the contract
    contract.with_source_account(&admin).start(&0);
    // Set end time
    contract.with_source_account(&admin).set_end(&20);
    // Try cash out
    contract.with_source_account(&user1).cash_out();
}

#[test]
#[should_panic(expected = "Status(ContractError(8))")]
fn test_enable_cash_out_when_not_enough_tokens() {
    let e: Env = Default::default();

    let admin = e.accounts().generate();
    let admin_id = Identifier::Account(admin.clone());
    let payment_tkn_admin = e.accounts().generate();

    let user1 = e.accounts().generate();
    let user1_id = Identifier::Account(user1.clone());

    let (payment_tkn_id, payment_tkn) =
        create_token_contract(&e, &payment_tkn_admin, &"USD Coin", &"USDC", 8);

    let time = 20;
    let contract_id = e.register_contract(None, Bond);
    let contract = updates_contract_time(&e, contract_id.clone(), time);
    let contract_identifier = Identifier::Contract(contract_id.clone());

    // Users approve the contract to transfer their payment tokens
    payment_tkn.with_source_account(&user1).approve(
        &Signature::Invoker,
        &0,
        &contract_identifier,
        &100000,
    );

    // Payment token admin mint some tokens for the users
    payment_tkn.with_source_account(&payment_tkn_admin).mint(
        &Signature::Invoker,
        &0,
        &user1_id,
        &100000,
    );
    payment_tkn.with_source_account(&payment_tkn_admin).mint(
        &Signature::Invoker,
        &0,
        &admin_id,
        &200000,
    );

    // Initialize the contract
    contract.initialize(
        &admin_id.clone(),
        &payment_tkn_id,
        &"Bond".into_val(&e),
        &"BND".into_val(&e),
        &8,
        &100,
        &100, // 100 / 1000 = 0.1 => 10%
        &30,
        &InterestType::Compound,
        &10000,
    );

    // Start the contract
    contract.with_source_account(&admin).start(&0);
    // Set end
    contract.with_source_account(&admin).set_end(&20);
    // Approve the User 1
    contract.with_source_account(&admin).add_user(&user1_id);
    // User 1 buys 200 bond tokens
    contract.with_source_account(&user1).buy(&200);
    // Admin withdraw some tokens
    contract.with_source_account(&admin).withdraw(&10000);
    // Try enable cash out
    contract.with_source_account(&admin).en_csh_out();
}

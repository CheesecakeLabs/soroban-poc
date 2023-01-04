#![cfg(test)]
use crate::contract::{token, Bond, BondClient};

use soroban_auth::Identifier;
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

#[test]
fn test_success() {
    let e: Env = Default::default();

    let admin = e.accounts().generate();
    let admin_id = Identifier::Account(admin.clone());
    let payment_tkn_admin = e.accounts().generate();

    let user1 = e.accounts().generate();
    let user2 = e.accounts().generate();
    let user3 = e.accounts().generate();
    let user1_id = Identifier::Account(user1.clone());
    let user2_id = Identifier::Account(user2.clone());
    let user3_id = Identifier::Account(user3.clone());

    let (payment_tkn_id, payment_tkn) =
        create_token_contract(&e, &payment_tkn_admin, &"USD Coin", &"USDC", 8);

    let contract_id = e.register_contract(None, Bond);
    let mut contract = updates_contract_time(&e, contract_id.clone(), 0);
    let contract_identifier = Identifier::Contract(contract_id.clone());

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
        &10000,
    );

    let bond_tkn = TokenClient::new(&e, &contract.bond_id());

    assert_eq!(bond_tkn.balance(&contract_identifier), 10000);

    // Start the contract
    contract.with_source_account(&admin).start(&0);

    // User call example
    // contract
    //     .with_source_account(&user1)
    //     .method(&Signature::Invoker);
}

// #[test]
// #[should_panic(expected = "Status(ContractError(1))")]
// fn already_initialized_error() {}

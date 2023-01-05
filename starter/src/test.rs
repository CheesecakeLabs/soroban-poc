#![cfg(test)]
use crate::contract::{Contract, ContractClient};

use soroban_auth::{Identifier, Signature};
use soroban_sdk::testutils::{Accounts, Ledger, LedgerInfo};
use soroban_sdk::{AccountId, BytesN, Env, IntoVal};

fn updates_contract_time(e: &Env, contract_id: BytesN<32>, time: u64) -> ContractClient {
    e.ledger().set(LedgerInfo {
        timestamp: time,
        protocol_version: 1,
        sequence_number: 10,
        network_passphrase: Default::default(),
        base_reserve: 10,
    });
    return ContractClient::new(&e, &contract_id);
}

#[test]
fn test_success() {
    let e: Env = Default::default();

    let admin = e.accounts().generate();

    let user1 = e.accounts().generate();
    let user2 = e.accounts().generate();
    let user3 = e.accounts().generate();
    let user1_id = Identifier::Account(user1.clone());
    let user2_id = Identifier::Account(user2.clone());
    let user3_id = Identifier::Account(user3.clone());

    let contract_id = e.register_contract(None, Contract);
    let mut contract = updates_contract_time(&e, contract_id.clone(), 0);
    let contract_identifier = Identifier::Contract(contract_id.clone());

    // Start the contract example
    // contract.initialize();

    // User call example
    // contract
    //     .with_source_account(&user1)
    //     .method(&Signature::Invoker);
}

#[test]
#[should_panic(expected = "Status(ContractError(1))")]
fn already_initialized_error() {}

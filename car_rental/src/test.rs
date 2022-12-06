#![cfg(test)]

use super::*;

use soroban_auth::Identifier;
use soroban_sdk::{symbol, testutils::Accounts, Env, IntoVal};

#[test]
// INIT Success
fn test() {
    let env: Env = Default::default();
    let contract_id = env.register_contract(None, CarRentalContract);
    let client = CarRentalContractClient::new(&env, &contract_id);
    let admin = env.accounts().generate();
    let (user_1_id, user_1_sign) = soroban_auth::testutils::ed25519::generate(&env);

    // Admin init
    client.init(&Identifier::Account(admin.clone()));

    let nonce = BigInt::from_u32(&env, 0);
    let sig = soroban_auth::testutils::ed25519::sign(
        &env,
        &user_1_sign,
        &contract_id,
        symbol!("open_req"),
        (&user_1_id, &nonce),
    );
    // Client open a validation request
    client.open_req(&sig, &nonce);

    let user_1_status = client.read_clnt(&user_1_id);
    assert_eq!(user_1_status, ClientStatus::Pending);

    // Admin denies client request
    client.with_source_account(&admin).deny_req(
        &Signature::Invoker,
        &user_1_id,
        &BigInt::zero(&env),
    );

    let user_1_status = client.read_clnt(&user_1_id);
    assert_eq!(user_1_status, ClientStatus::Declined);

    // Admin approves client request
    client.with_source_account(&admin).appr_req(
        &Signature::Invoker,
        &user_1_id,
        &BigInt::zero(&env),
    );

    let user_1_status = client.read_clnt(&user_1_id);
    assert_eq!(user_1_status, ClientStatus::Approved);

    let car_data = CarDataKey {
        model: "Gol quadrado".into_val(&env),
        horse: 80,
        color: "Vermelho".into_val(&env),
    };
    // Admin adds a new car
    client.with_source_account(&admin).add_car(
        &Signature::Invoker,
        &BigInt::zero(&env),
        &"IYD8J01".into_val(&env),
        &car_data,
    );

    assert_eq!(client.read_car(&"IYD8J01".into_val(&env)), car_data);
}

// INIT Failed ( has_admin = True)
#[test]
#[should_panic(expected = "Status(ContractError(4))")]
fn already_initialized() {
    let env: Env = Default::default();
    let contract_id = env.register_contract(None, CarRentalContract);
    let client = CarRentalContractClient::new(&env, &contract_id);
    let admin = env.accounts().generate();
    client.init(&Identifier::Account(admin));

    let admin_1 = env.accounts().generate();
    client.init(&Identifier::Account(admin_1));

    //  assert_eq!(has_admin, true);
    // check if admin ==
}

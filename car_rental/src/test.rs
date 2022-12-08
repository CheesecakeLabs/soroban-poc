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

    let mut nonce = 0;
    let mut sig = soroban_auth::testutils::ed25519::sign(
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
    client
        .with_source_account(&admin)
        .deny_req(&Signature::Invoker, &user_1_id, &0);

    let user_1_status = client.read_clnt(&user_1_id);
    assert_eq!(user_1_status, ClientStatus::Declined);

    // Admin approves client request
    client
        .with_source_account(&admin)
        .appr_req(&Signature::Invoker, &user_1_id, &0);

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
        &0,
        &"IYD8J01".into_val(&env),
        &car_data.model,
        &car_data.color,
        &car_data.horse,
    );

    assert_eq!(client.read_car(&"IYD8J01".into_val(&env)), car_data);
    nonce = nonce + 1;
    sig = soroban_auth::testutils::ed25519::sign(
        &env,
        &user_1_sign,
        &contract_id,
        symbol!("resrve_car"),
        (&user_1_id, &nonce),
    );

    client.resrve_car(&sig, &nonce, &"IYD8J01".into_val(&env));

    let rented_car = client.read_rent(&"IYD8J01".into_val(&env));
    assert_eq!(rented_car.status, RentedCarStatus::Reserved);

    nonce = nonce + 1;

    sig = soroban_auth::testutils::ed25519::sign(
        &env,
        &user_1_sign,
        &contract_id,
        symbol!("take_car"),
        (&user_1_id, &nonce),
    );

    client.take_car(&sig, &nonce, &"IYD8J01".into_val(&env));

    let rented_car = client.read_rent(&"IYD8J01".into_val(&env));
    assert_eq!(rented_car.status, RentedCarStatus::Rented);

    // // Admin remove a car
    // client.with_source_account(&admin).remove_car(
    //     &Signature::Invoker,
    //     &0,
    //     &"IYD8J01".into_val(&env),
    // );

    // Drop car
    nonce = nonce + 1;
    sig = soroban_auth::testutils::ed25519::sign(
        &env,
        &user_1_sign,
        &contract_id,
        symbol!("drop_car"),
        (&user_1_id, &nonce),
    );

    client.drop_car(&sig, &nonce, &"IYD8J01".into_val(&env));
    let rented_car = client.read_rent(&"IYD8J01".into_val(&env));
    assert_eq!(rented_car.status, RentedCarStatus::DropReview);

    // Deny drop request
    client.with_source_account(&admin).deny_drop(
        &Signature::Invoker,
        &0,
        &"IYD8J01".into_val(&env),
    );
    let rented_car = client.read_rent(&"IYD8J01".into_val(&env));
    assert_eq!(rented_car.status, RentedCarStatus::DropReviewDenied);

    // Drop car
    nonce = nonce + 1;
    sig = soroban_auth::testutils::ed25519::sign(
        &env,
        &user_1_sign,
        &contract_id,
        symbol!("drop_car"),
        (&user_1_id, &nonce),
    );

    client.drop_car(&sig, &nonce, &"IYD8J01".into_val(&env));
    let rented_car = client.read_rent(&"IYD8J01".into_val(&env));
    assert_eq!(rented_car.status, RentedCarStatus::DropReview);

    // Accept drop request
    client.with_source_account(&admin).accpt_drop(
        &Signature::Invoker,
        &0,
        &"IYD8J01".into_val(&env),
    );
    assert_eq!(client.has_rent(&"IYD8J01".into_val(&env)), false);
}

// INIT Failed (has_admin = True)
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

#[test]
#[should_panic(expected = "Status(ContractError(5))")]
fn add_car_fails_already_exists() {
    let env: Env = Default::default();
    let contract_id = env.register_contract(None, CarRentalContract);
    let client = CarRentalContractClient::new(&env, &contract_id);
    let admin = env.accounts().generate();
    client.init(&Identifier::Account(admin.clone()));
    let car_data = CarDataKey {
        model: "Gol quadrado".into_val(&env),
        horse: 80,
        color: "Vermelho".into_val(&env),
    };
    client.with_source_account(&(admin.clone())).add_car(
        &Signature::Invoker,
        &0,
        &"IYD8J01".into_val(&env),
        &car_data.model,
        &car_data.color,
        &car_data.horse,
    );
    assert_eq!(client.read_car(&"IYD8J01".into_val(&env)), car_data);
    let car_data = CarDataKey {
        model: "Gol quadrado".into_val(&env),
        horse: 80,
        color: "Vermelho".into_val(&env),
    };
    client.with_source_account(&admin).add_car(
        &Signature::Invoker,
        &0,
        &"IYD8J01".into_val(&env),
        &car_data.model,
        &car_data.color,
        &car_data.horse,
    );
}

#[test]
#[should_panic(expected = "Status(ContractError(6))")]
fn remove_car_fails_no_exist() {
    let env: Env = Default::default();
    let contract_id = env.register_contract(None, CarRentalContract);
    let client = CarRentalContractClient::new(&env, &contract_id);
    let admin = env.accounts().generate();
    client.init(&Identifier::Account(admin.clone()));

    client.with_source_account(&admin).remove_car(
        &Signature::Invoker,
        &0,
        &"IYD8J01".into_val(&env),
    );
}

#[test]
#[should_panic(expected = "Status(ContractError(8))")]
fn take_car_fails_car_not_rented() {
    let env: Env = Default::default();
    let contract_id = env.register_contract(None, CarRentalContract);
    let client = CarRentalContractClient::new(&env, &contract_id);
    let admin = env.accounts().generate();
    client.init(&Identifier::Account(admin.clone()));

    client
        .with_source_account(&admin)
        .take_car(&Signature::Invoker, &0, &"IYD8J01".into_val(&env));
}

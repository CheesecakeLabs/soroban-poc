#![cfg(test)]
use crate::contract::CarRentalContract;
use crate::storage_types::{CarDataKey, ClientStatus, RentedCarStatus};
use crate::testutils::CarRental;
use soroban_auth::Identifier;
use soroban_sdk::{testutils::Accounts, Env, IntoVal};

#[test]
fn test_success() {
    let env: Env = Default::default();
    let contract_id = env.register_contract(None, CarRentalContract);

    let car_rental = CarRental::new(&env, &contract_id);

    let admin = env.accounts().generate();
    let (user_1_id, user_1_sign) = soroban_auth::testutils::ed25519::generate(&env);

    // Admin init the contract
    car_rental.init(&Identifier::Account(admin.clone()));

    // User 1 opens a request to validate the account
    car_rental.open_request(user_1_id.clone(), &user_1_sign);
    // Check if User 1 account status is Pending
    assert_eq!(
        car_rental.read_client(user_1_id.clone()),
        ClientStatus::Pending
    );

    // Admin denies User 1 account validation request
    car_rental.deny_request(&admin, user_1_id.clone());
    // Check if User 1 account status is Declined
    assert_eq!(
        car_rental.read_client(user_1_id.clone()),
        ClientStatus::Declined
    );

    // Admin approves User 1 account validation request
    car_rental.approve_request(&admin, user_1_id.clone());
    assert_eq!(
        car_rental.read_client(user_1_id.clone()),
        ClientStatus::Approved
    );

    // Admin add Car 1
    let car_1_data = CarDataKey {
        model: "Volkswagen Gol".into_val(&env),
        horse: 80,
        color: "Red".into_val(&env),
    };
    let car_1_plate = "IYD8J01";
    car_rental.add_car(&admin, &car_1_plate, car_1_data.clone());
    assert_eq!(car_rental.read_car(&car_1_plate), car_1_data);

    // Admin add Car 2
    let car_2_data = CarDataKey {
        model: "Honda Civic".into_val(&env),
        horse: 200,
        color: "White".into_val(&env),
    };
    let car_2_plate = "PGWN112";
    car_rental.add_car(&admin, &car_2_plate, car_2_data.clone());
    assert_eq!(car_rental.read_car(&car_2_plate), car_2_data);

    // Client reserves Car 1
    car_rental.reserve_car(user_1_id.clone(), &user_1_sign, &car_1_plate);
    let mut rented_car = car_rental.read_rent(&car_1_plate);
    assert_eq!(rented_car.status, RentedCarStatus::Reserved);
    assert_eq!(rented_car.renter, user_1_id.clone());

    // Client take the Car 1
    car_rental.take_car(user_1_id.clone(), &user_1_sign, &car_1_plate);
    rented_car = car_rental.read_rent(&car_1_plate);
    assert_eq!(rented_car.status, RentedCarStatus::Rented);
    assert_eq!(rented_car.renter, user_1_id.clone());

    // Admin remove Car 2
    car_rental.remove_car(&admin, &car_2_plate);

    // Client drop Car 1
    car_rental.drop_car(user_1_id.clone(), &user_1_sign, &car_1_plate);
    rented_car = car_rental.read_rent(&car_1_plate);
    assert_eq!(rented_car.status, RentedCarStatus::DropReview);
    assert_eq!(rented_car.renter, user_1_id.clone());

    // Admin denies drop request
    car_rental.deny_drop(&admin, &car_1_plate);
    rented_car = car_rental.read_rent(&car_1_plate);
    assert_eq!(rented_car.status, RentedCarStatus::DropReviewDenied);
    assert_eq!(rented_car.renter, user_1_id.clone());

    // Client try drop Car 1 again
    car_rental.drop_car(user_1_id.clone(), &user_1_sign, &car_1_plate);
    rented_car = car_rental.read_rent(&car_1_plate);
    assert_eq!(rented_car.status, RentedCarStatus::DropReview);
    assert_eq!(rented_car.renter, user_1_id.clone());

    // Admin accept drop request
    car_rental.accept_drop(&admin, &car_1_plate);
    assert_eq!(car_rental.has_rent(&car_1_plate), false);
}

// Init Failed (has_admin = True)
#[test]
#[should_panic(expected = "Status(ContractError(4))")]
fn already_initialized() {
    let env: Env = Default::default();
    let contract_id = env.register_contract(None, CarRentalContract);
    let car_rental = CarRental::new(&env, &contract_id);

    let admin = env.accounts().generate();
    car_rental.init(&Identifier::Account(admin.clone()));

    let admin_1 = env.accounts().generate();
    car_rental.init(&Identifier::Account(admin_1.clone()));
}

#[test]
#[should_panic(expected = "Status(ContractError(5))")]
fn add_car_fails_already_exists() {
    let env: Env = Default::default();
    let contract_id = env.register_contract(None, CarRentalContract);
    let car_rental = CarRental::new(&env, &contract_id);

    let admin = env.accounts().generate();

    car_rental.init(&Identifier::Account(admin.clone()));

    let car_data = CarDataKey {
        model: "Volkswagen Gol".into_val(&env),
        horse: 80,
        color: "Red".into_val(&env),
    };
    let car_plate = "IYD8J01";

    car_rental.add_car(&admin, &car_plate, car_data.clone());
    car_rental.add_car(&admin, &car_plate, car_data.clone());
}

#[test]
#[should_panic(expected = "Status(ContractError(6))")]
fn remove_car_fails_no_exist() {
    let env: Env = Default::default();
    let contract_id = env.register_contract(None, CarRentalContract);
    let car_rental = CarRental::new(&env, &contract_id);

    let admin = env.accounts().generate();

    car_rental.init(&Identifier::Account(admin.clone()));

    car_rental.remove_car(&admin, &"IYD8J01");
}

#[test]
#[should_panic(expected = "Status(ContractError(8))")]
fn take_car_fails_car_not_rented() {
    let env: Env = Default::default();
    let contract_id = env.register_contract(None, CarRentalContract);
    let car_rental = CarRental::new(&env, &contract_id);

    let admin = env.accounts().generate();
    let (user_1_id, user_1_sign) = soroban_auth::testutils::ed25519::generate(&env);

    car_rental.init(&Identifier::Account(admin.clone()));

    car_rental.take_car(user_1_id.clone(), &user_1_sign, &"IYD8J01");
}


#[test]
fn test_change_admin_successfully() {
    let env: Env = Default::default();
    let contract_id = env.register_contract(None, CarRentalContract);
    let car_rental = CarRental::new(&env, &contract_id);

    let admin = env.accounts().generate();
    let new_admin_1 = env.accounts().generate();
    let (new_admin_2_id, _new_admin_2_sign) = soroban_auth::testutils::ed25519::generate(&env);

    car_rental.init(&Identifier::Account(admin.clone()));
    car_rental.set_admin(&admin, Identifier::Account(new_admin_1.clone()));
    assert_eq!(
        car_rental.read_admin(),
        Identifier::Account(new_admin_1.clone())
    );
    car_rental.set_admin(&new_admin_1, new_admin_2_id.clone());
    assert_eq!(car_rental.read_admin(), new_admin_2_id.clone());
}

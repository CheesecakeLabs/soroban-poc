#![cfg(test)]

use crate::contract::CarRentalContract;
use crate::contract::CarRentalContractClient;
use crate::storage_types::{CarDataKey, ClientStatus, RentedCarDataKey, RentedCarStatus};
use core::fmt::Debug;
use soroban_auth::{Identifier, Signature};
use soroban_sdk::{symbol, testutils::Accounts, BigInt, BytesN, Env, IntoVal};

pub struct CarRental {
    env: Env,
    contract_id: BytesN<32>,
}

impl CarRental {
    pub fn new(env: &Env, contract_id: &BytesN<32>) -> Self {
        Self {
            env: env.clone(),
            contract_id: contract_id.clone(),
        }
    }

    pub fn init(&self, admin: &Identifier) {
        CarRentalContractClient::new(&self.env, &self.contract_id).init(admin);
    }

    // Admin methods (using Invoker Signature)
    fn deny_request(&self, admin: &soroban_sdk::AccountId, client_id: Identifier) {
        CarRentalContractClient::new(&self.env, &self.contract_id)
            .with_source_account(&admin)
            .deny_req(
                &Signature::Invoker,
                &client_id,
                &BigInt::from_u32(&self.env, 0),
            );
    }

    fn approve_request(&self, admin: &soroban_sdk::AccountId, client_id: Identifier) {
        CarRentalContractClient::new(&self.env, &self.contract_id)
            .with_source_account(&admin)
            .appr_req(
                &Signature::Invoker,
                &client_id,
                &BigInt::from_u32(&self.env, 0),
            );
    }

    fn add_car(&self, admin: &soroban_sdk::AccountId, plate: &str, car_data: CarDataKey) {
        CarRentalContractClient::new(&self.env, &self.contract_id)
            .with_source_account(&admin)
            .add_car(
                &Signature::Invoker,
                &BigInt::from_u32(&self.env, 0),
                &plate.into_val(&self.env),
                &car_data.model,
                &car_data.color,
                &car_data.horse,
            );
    }

    fn remove_car(&self, admin: &soroban_sdk::AccountId, plate: &str) {
        CarRentalContractClient::new(&self.env, &self.contract_id)
            .with_source_account(&admin)
            .remove_car(
                &Signature::Invoker,
                &BigInt::from_u32(&self.env, 0),
                &plate.into_val(&self.env),
            );
    }

    fn deny_drop(&self, admin: &soroban_sdk::AccountId, plate: &str) {
        CarRentalContractClient::new(&self.env, &self.contract_id)
            .with_source_account(&admin)
            .deny_drop(
                &Signature::Invoker,
                &BigInt::from_u32(&self.env, 0),
                &plate.into_val(&self.env),
            );
    }

    fn accept_drop(&self, admin: &soroban_sdk::AccountId, plate: &str) {
        CarRentalContractClient::new(&self.env, &self.contract_id)
            .with_source_account(&admin)
            .accpt_drop(
                &Signature::Invoker,
                &BigInt::from_u32(&self.env, 0),
                &plate.into_val(&self.env),
            );
    }

    // Client methods (using Ed25519 Signature)
    fn open_request(
        &self,
        user_id: Identifier,
        user_sign: &(impl soroban_auth::testutils::ed25519::Identifier
              + soroban_sdk::testutils::ed25519::Sign<
            soroban_auth::SignaturePayload,
            Signature = [u8; 64],
        > + Debug),
    ) {
        let nonce = self.nonce(&user_id);
        let sig = soroban_auth::testutils::ed25519::sign(
            &self.env,
            user_sign,
            &self.contract_id,
            symbol!("open_req"),
            (&user_id, &nonce),
        );
        CarRentalContractClient::new(&self.env, &self.contract_id).open_req(&sig, &nonce);
    }

    fn reserve_car(
        &self,
        user_id: Identifier,
        user_sign: &(impl soroban_auth::testutils::ed25519::Identifier
              + soroban_sdk::testutils::ed25519::Sign<
            soroban_auth::SignaturePayload,
            Signature = [u8; 64],
        > + Debug),
        plate: &str,
    ) {
        let nonce = self.nonce(&user_id);
        let sig = soroban_auth::testutils::ed25519::sign(
            &self.env,
            user_sign,
            &self.contract_id,
            symbol!("resrve_car"),
            (&user_id, &nonce),
        );
        CarRentalContractClient::new(&self.env, &self.contract_id).resrve_car(
            &sig,
            &nonce,
            &plate.into_val(&self.env),
        );
    }

    fn take_car(
        &self,
        user_id: Identifier,
        user_sign: &(impl soroban_auth::testutils::ed25519::Identifier
              + soroban_sdk::testutils::ed25519::Sign<
            soroban_auth::SignaturePayload,
            Signature = [u8; 64],
        > + Debug),
        plate: &str,
    ) {
        let nonce = self.nonce(&user_id);
        let sig = soroban_auth::testutils::ed25519::sign(
            &self.env,
            user_sign,
            &self.contract_id,
            symbol!("take_car"),
            (&user_id, &nonce),
        );
        CarRentalContractClient::new(&self.env, &self.contract_id).take_car(
            &sig,
            &nonce,
            &plate.into_val(&self.env),
        );
    }

    fn drop_car(
        &self,
        user_id: Identifier,
        user_sign: &(impl soroban_auth::testutils::ed25519::Identifier
              + soroban_sdk::testutils::ed25519::Sign<
            soroban_auth::SignaturePayload,
            Signature = [u8; 64],
        > + Debug),
        plate: &str,
    ) {
        let nonce = self.nonce(&user_id);
        let sig = soroban_auth::testutils::ed25519::sign(
            &self.env,
            user_sign,
            &self.contract_id,
            symbol!("drop_car"),
            (&user_id, &nonce),
        );
        CarRentalContractClient::new(&self.env, &self.contract_id).drop_car(
            &sig,
            &nonce,
            &plate.into_val(&self.env),
        );
    }

    // Public methods
    pub fn nonce(&self, id: &Identifier) -> BigInt {
        CarRentalContractClient::new(&self.env, &self.contract_id).nonce(id)
    }

    pub fn read_client(&self, id: Identifier) -> ClientStatus {
        return CarRentalContractClient::new(&self.env, &self.contract_id).read_clnt(&id);
    }

    pub fn read_car(&self, plate: &str) -> CarDataKey {
        return CarRentalContractClient::new(&self.env, &self.contract_id)
            .read_car(&plate.into_val(&self.env));
    }

    pub fn read_rent(&self, plate: &str) -> RentedCarDataKey {
        return CarRentalContractClient::new(&self.env, &self.contract_id)
            .read_rent(&plate.into_val(&self.env));
    }

    pub fn has_rent(&self, plate: &str) -> bool {
        return CarRentalContractClient::new(&self.env, &self.contract_id)
            .has_rent(&plate.into_val(&self.env));
    }
}

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

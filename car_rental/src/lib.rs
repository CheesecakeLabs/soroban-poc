#![no_std]
use soroban_auth::verify;
use soroban_auth::{Identifier, Signature};
use soroban_sdk::{
    contracterror, contractimpl, contracttype, panic_with_error, symbol, BigInt, Bytes, Env,
};

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum Error {
    NotAuthorized = 1,
    NonceShouldBeZero = 2,
    IncorrectNonce = 3,
    AlreadyInit = 4,
    CarAlreadyExists = 5,
    CarNotExists = 6,
    CarAlreadyRented = 7,
    CarIsNotRented = 8,
    CarIsNotInDropReview = 9,
    CarIsNotReserved = 10,
    ClientIsNotRenter = 11,
    CarIsNotWithRentedStatus = 12,
    ClientNotAuthorized = 13,
}

#[contracttype]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum ClientStatus {
    Pending = 0,
    Approved = 1,
    Declined = 2,
}

#[contracttype]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum RentedCarStatus {
    Reserved = 0,
    Rented = 1,
    DropReview = 2,
    DropReviewDenied = 3,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CarDataKey {
    pub model: Bytes,
    pub color: Bytes,
    pub horse: i32,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RentedCarDataKey {
    pub renter: Identifier,
    pub status: RentedCarStatus,
}

#[derive(Clone)]
#[contracttype]
pub enum DataKey {
    Nonce(Identifier),
    Client(Identifier),
    Admin,
    Car(Bytes),       // = CarDataKey
    RentedCar(Bytes), // = RentedCarDataKey
}

fn write_client(env: &Env, client: Identifier, status: ClientStatus) {
    env.data().set(DataKey::Client(client), status)
}

fn read_client(env: &Env, client: Identifier) -> ClientStatus {
    env.data().get_unchecked(DataKey::Client(client)).unwrap()
}

fn is_client_authorized(env: &Env, client: &Identifier) -> bool {
    read_client(env, client.clone()) == ClientStatus::Approved
}

fn write_car(env: &Env, plate: &Bytes, car_data: CarDataKey) {
    env.data().set(DataKey::Car(plate.clone()), car_data)
}

fn read_car(env: &Env, plate: Bytes) -> CarDataKey {
    env.data().get_unchecked(DataKey::Car(plate)).unwrap()
}

fn has_car(env: &Env, plate: &Bytes) -> bool {
    env.data().has(DataKey::Car(plate.clone()))
}

fn remove_car(env: &Env, plate: &Bytes) {
    env.data().remove(DataKey::Car(plate.clone()))
}

fn write_rented_car(env: &Env, plate: &Bytes, rented_car_data: RentedCarDataKey) {
    env.data()
        .set(DataKey::RentedCar(plate.clone()), rented_car_data)
}

fn read_rented_car(env: &Env, plate: &Bytes) -> RentedCarDataKey {
    env.data()
        .get_unchecked(DataKey::RentedCar(plate.clone()))
        .unwrap()
}

fn remove_rented_car(env: &Env, plate: &Bytes) {
    env.data().remove(DataKey::RentedCar(plate.clone()))
}

fn has_rented_car(env: &Env, plate: &Bytes) -> bool {
    env.data().has(DataKey::RentedCar(plate.clone()))
}

fn write_admin(env: &Env, admin: Identifier) {
    env.data().set(DataKey::Admin, admin)
}

fn has_admin(env: &Env) -> bool {
    env.data().has(DataKey::Admin)
}

fn read_admin(env: &Env) -> Identifier {
    env.data().get_unchecked(DataKey::Admin).unwrap()
}

fn check_admin(env: &Env, auth: &Signature) {
    let auth_id = auth.identifier(&env);
    if auth_id != read_admin(&env) {
        panic_with_error!(&env, Error::NotAuthorized)
    }
}

fn read_nonce(env: &Env, id: &Identifier) -> BigInt {
    let key = DataKey::Nonce(id.clone());
    env.data()
        .get(key)
        .unwrap_or_else(|| Ok(BigInt::zero(env)))
        .unwrap()
}

fn verify_and_consume_nonce(env: &Env, auth: &Signature, expected_nonce: &BigInt) {
    match auth {
        Signature::Invoker => {
            if BigInt::zero(&env) != expected_nonce {
                panic_with_error!(&env, Error::NonceShouldBeZero)
            }
            return;
        }
        _ => {}
    }

    let id = auth.identifier(&env);
    let key = DataKey::Nonce(id.clone());
    let nonce = read_nonce(env, &id);

    if nonce != expected_nonce {
        panic_with_error!(&env, Error::IncorrectNonce)
    }
    env.data().set(key, &nonce + 1);
}

pub trait CarRentalTrait {
    /// # Admin Flow
    fn init(env: Env, admin: Identifier);
    fn add_car(
        env: Env,
        admin: Signature,
        nonce: BigInt,
        plate: Bytes,
        model: Bytes,
        color: Bytes,
        horse: i32,
    );
    fn remove_car(env: Env, admin: Signature, nonce: BigInt, plate: Bytes);
    fn appr_req(env: Env, admin: Signature, client: Identifier, nonce: BigInt);
    fn deny_req(env: Env, admin: Signature, client: Identifier, nonce: BigInt);

    fn open_req(env: Env, client: Signature, nonce: BigInt);

    fn take_car(env: Env, client: Signature, nonce: BigInt, plate: Bytes);
    fn read_car(env: Env, plate: Bytes) -> CarDataKey;
    fn resrve_car(env: Env, client: Signature, nonce: BigInt, plate: Bytes);
    fn drop_car(env: Env, client: Signature, nonce: BigInt, plate: Bytes);
    fn accpt_drop(env: Env, admin: Signature, nonce: BigInt, plate: Bytes);
    fn deny_drop(env: Env, admin: Signature, nonce: BigInt, plate: Bytes);
    fn read_clnt(env: Env, client: Identifier) -> ClientStatus;
    fn nonce(env: Env, identifier: Identifier) -> BigInt;
    fn read_rent(env: Env, plate: Bytes) -> RentedCarDataKey;
    fn has_rent(env: Env, plate: Bytes) -> bool;
}

pub struct CarRentalContract;

#[contractimpl]
impl CarRentalTrait for CarRentalContract {
    // Admin
    fn init(env: Env, admin: Identifier) {
        if has_admin(&env) {
            panic_with_error!(&env, Error::AlreadyInit)
        }
        write_admin(&env, admin);
    }

    fn add_car(
        env: Env,
        admin: Signature,
        nonce: BigInt,
        plate: Bytes,
        model: Bytes,
        color: Bytes,
        horse: i32,
    ) {
        check_admin(&env, &admin);
        verify_and_consume_nonce(&env, &admin, &nonce);

        let admin_id = admin.identifier(&env);
        verify(&env, &admin, symbol!("add_car"), (admin_id, nonce));
        if has_car(&env, &plate) {
            panic_with_error!(&env, Error::CarAlreadyExists)
        }
        write_car(
            &env,
            &plate,
            CarDataKey {
                model: model,
                color: color,
                horse: horse,
            },
        )
    }

    fn remove_car(env: Env, admin: Signature, nonce: BigInt, plate: Bytes) {
        check_admin(&env, &admin);
        verify_and_consume_nonce(&env, &admin, &nonce);

        let admin_id = admin.identifier(&env);
        verify(&env, &admin, symbol!("remove_car"), (admin_id, nonce));

        if !has_car(&env, &plate) {
            panic_with_error!(&env, Error::CarNotExists)
        }

        if has_rented_car(&env, &plate) {
            panic_with_error!(&env, Error::CarAlreadyRented)
        }

        remove_car(&env, &plate)
    }

    fn appr_req(env: Env, admin: Signature, client: Identifier, nonce: BigInt) {
        check_admin(&env, &admin);
        verify_and_consume_nonce(&env, &admin, &nonce);

        let admin_id = admin.identifier(&env);
        verify(&env, &admin, symbol!("appr_req"), (admin_id, nonce));

        write_client(&env, client, ClientStatus::Approved)
    }

    fn deny_req(env: Env, admin: Signature, client: Identifier, nonce: BigInt) {
        check_admin(&env, &admin);
        verify_and_consume_nonce(&env, &admin, &nonce);

        let admin_id = admin.identifier(&env);
        verify(&env, &admin, symbol!("deny_req"), (admin_id, nonce));

        write_client(&env, client, ClientStatus::Declined)
    }

    fn accpt_drop(env: Env, admin: Signature, nonce: BigInt, plate: Bytes) {
        check_admin(&env, &admin);
        verify_and_consume_nonce(&env, &admin, &nonce);

        let admin_id = admin.identifier(&env);
        verify(&env, &admin, symbol!("accpt_drop"), (admin_id, nonce));

        if !has_rented_car(&env, &plate) {
            panic_with_error!(&env, Error::CarIsNotRented)
        }

        let rented_car_data = read_rented_car(&env, &plate);
        if rented_car_data.status != RentedCarStatus::DropReview {
            panic_with_error!(&env, Error::CarIsNotInDropReview)
        }

        remove_rented_car(&env, &plate)
    }

    fn deny_drop(env: Env, admin: Signature, nonce: BigInt, plate: Bytes) {
        check_admin(&env, &admin);
        verify_and_consume_nonce(&env, &admin, &nonce);

        let admin_id = admin.identifier(&env);
        verify(&env, &admin, symbol!("accpt_drop"), (admin_id, nonce));

        if !has_rented_car(&env, &plate) {
            panic_with_error!(&env, Error::CarIsNotRented)
        }

        let mut rented_car_data = read_rented_car(&env, &plate);
        if rented_car_data.status != RentedCarStatus::DropReview {
            panic_with_error!(&env, Error::CarIsNotInDropReview)
        }

        rented_car_data.status = RentedCarStatus::DropReviewDenied;
        write_rented_car(&env, &plate, rented_car_data)
    }

    fn open_req(env: Env, client: Signature, nonce: BigInt) {
        verify_and_consume_nonce(&env, &client, &nonce);

        let client_identifier = client.identifier(&env);
        verify(
            &env,
            &client,
            symbol!("open_req"),
            (&client_identifier, nonce),
        );

        write_client(&env, client_identifier, ClientStatus::Pending)
    }

    fn resrve_car(env: Env, client: Signature, nonce: BigInt, plate: Bytes) {
        verify_and_consume_nonce(&env, &client, &nonce);

        let client_identifier = client.identifier(&env);
        verify(
            &env,
            &client,
            symbol!("resrve_car"),
            (&client_identifier, nonce),
        );

        if !is_client_authorized(&env, &client_identifier) {
            panic_with_error!(&env, Error::ClientNotAuthorized)
        }
        if !has_car(&env, &plate) {
            panic_with_error!(&env, Error::CarNotExists)
        }

        if has_rented_car(&env, &plate) {
            panic_with_error!(&env, Error::CarAlreadyRented)
        }

        write_rented_car(
            &env,
            &plate,
            RentedCarDataKey {
                renter: client_identifier,
                status: RentedCarStatus::Reserved,
            },
        )
    }

    fn take_car(env: Env, client: Signature, nonce: BigInt, plate: Bytes) {
        verify_and_consume_nonce(&env, &client, &nonce);

        let client_identifier = client.identifier(&env);
        verify(
            &env,
            &client,
            symbol!("take_car"),
            (&client_identifier, nonce),
        );

        if !has_rented_car(&env, &plate) {
            panic_with_error!(&env, Error::CarIsNotRented)
        }

        let mut rented_car_data = read_rented_car(&env, &plate);
        if rented_car_data.renter != client_identifier {
            panic_with_error!(&env, Error::ClientIsNotRenter)
        }
        if rented_car_data.status != RentedCarStatus::Reserved {
            panic_with_error!(&env, Error::CarIsNotReserved)
        }

        rented_car_data.status = RentedCarStatus::Rented;
        write_rented_car(&env, &plate, rented_car_data)
    }

    fn drop_car(env: Env, client: Signature, nonce: BigInt, plate: Bytes) {
        verify_and_consume_nonce(&env, &client, &nonce);

        let client_identifier = client.identifier(&env);
        verify(
            &env,
            &client,
            symbol!("drop_car"),
            (&client_identifier, nonce),
        );

        if !has_rented_car(&env, &plate) {
            panic_with_error!(&env, Error::CarIsNotRented)
        }

        let mut rented_car_data = read_rented_car(&env, &plate);
        if rented_car_data.renter != client_identifier {
            panic_with_error!(&env, Error::ClientIsNotRenter)
        }
        match rented_car_data.status {
            RentedCarStatus::Rented | RentedCarStatus::DropReviewDenied => {
                rented_car_data.status = RentedCarStatus::DropReview;
                write_rented_car(&env, &plate, rented_car_data)
            }
            _ => panic_with_error!(&env, Error::CarIsNotWithRentedStatus),
        }
    }

    fn read_clnt(env: Env, client: Identifier) -> ClientStatus {
        // Todo Check if client exist
        read_client(&env, client)
    }

    fn nonce(env: Env, identifier: Identifier) -> BigInt {
        read_nonce(&env, &identifier)
    }

    fn read_car(env: Env, plate: Bytes) -> CarDataKey {
        // Todo check if car exist.
        read_car(&env, plate)
    }

    fn read_rent(env: Env, plate: Bytes) -> RentedCarDataKey {
        //  read rented car
        read_rented_car(&env, &plate)
    }

    fn has_rent(env: Env, plate: Bytes) -> bool {
        has_rented_car(&env, &plate)
    }
}

#[cfg(test)]
mod test;

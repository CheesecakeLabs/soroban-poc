use crate::storage_types::{CarDataKey, ClientStatus, DataKey, RentedCarDataKey};
use crate::errors::Error;
use soroban_auth::{Identifier, Signature};
use soroban_sdk::{panic_with_error, Bytes, Env};

pub fn write_client(env: &Env, client: Identifier, status: ClientStatus) {
    env.data().set(DataKey::Client(client), status)
}

pub fn read_client(env: &Env, client: Identifier) -> ClientStatus {
    env.data().get_unchecked(DataKey::Client(client)).unwrap()
}

pub fn is_client_authorized(env: &Env, client: &Identifier) -> bool {
    read_client(env, client.clone()) == ClientStatus::Approved
}

pub fn write_car(env: &Env, plate: &Bytes, car_data: CarDataKey) {
    env.data().set(DataKey::Car(plate.clone()), car_data)
}

pub fn read_car(env: &Env, plate: Bytes) -> CarDataKey {
    env.data().get_unchecked(DataKey::Car(plate)).unwrap()
}

pub fn has_car(env: &Env, plate: &Bytes) -> bool {
    env.data().has(DataKey::Car(plate.clone()))
}

pub fn remove_car(env: &Env, plate: &Bytes) {
    env.data().remove(DataKey::Car(plate.clone()))
}

pub fn write_rented_car(env: &Env, plate: &Bytes, rented_car_data: RentedCarDataKey) {
    env.data()
        .set(DataKey::RentedCar(plate.clone()), rented_car_data)
}

pub fn read_rented_car(env: &Env, plate: &Bytes) -> RentedCarDataKey {
    env.data()
        .get_unchecked(DataKey::RentedCar(plate.clone()))
        .unwrap()
}

pub fn remove_rented_car(env: &Env, plate: &Bytes) {
    env.data().remove(DataKey::RentedCar(plate.clone()))
}

pub fn has_rented_car(env: &Env, plate: &Bytes) -> bool {
    env.data().has(DataKey::RentedCar(plate.clone()))
}

pub fn write_admin(env: &Env, admin: Identifier) {
    env.data().set(DataKey::Admin, admin)
}

pub fn has_admin(env: &Env) -> bool {
    env.data().has(DataKey::Admin)
}

pub fn read_admin(env: &Env) -> Identifier {
    env.data().get_unchecked(DataKey::Admin).unwrap()
}

pub fn check_admin(env: &Env, auth: &Signature) {
    let auth_id = auth.identifier(&env);
    if auth_id != read_admin(&env) {
        panic_with_error!(&env, Error::NotAuthorized)
    }
}

pub fn read_nonce(env: &Env, id: &Identifier) -> i128 {
    let key = DataKey::Nonce(id.clone());
    env.data()
        .get(key)
        .unwrap_or_else(|| Ok(0))
        .unwrap()
}

pub fn verify_and_consume_nonce(env: &Env, auth: &Signature, expected_nonce: i128) {
    match auth {
        Signature::Invoker => {
            if 0 != expected_nonce {
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

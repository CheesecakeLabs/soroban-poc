use crate::contract_trait::CarRentalTrait;
use crate::errors::Error;
use crate::metadata::{
    check_admin, has_admin, has_car, has_rented_car, is_client_authorized, read_car, read_client,
    read_nonce, read_rented_car, remove_car, remove_rented_car, verify_and_consume_nonce,
    write_admin, write_car, write_client, write_rented_car,
};
use crate::storage_types::{CarDataKey, ClientStatus, RentedCarDataKey, RentedCarStatus};
use soroban_auth::verify;
use soroban_auth::{Identifier, Signature};
use soroban_sdk::{contractimpl, panic_with_error, symbol, BigInt, Bytes, Env};
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

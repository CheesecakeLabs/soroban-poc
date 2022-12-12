use crate::contract_trait::CarRentalTrait;
use crate::errors::Error;
use crate::metadata::{
    check_admin, has_admin, has_car, has_rented_car, is_client_authorized, read_admin, read_car,
    read_client, read_nonce, read_rented_car, remove_car, remove_rented_car,
    verify_and_consume_nonce, write_admin, write_car, write_client, write_rented_car,
};
use crate::storage_types::{CarDataKey, ClientStatus, RentedCarDataKey, RentedCarStatus};
use soroban_auth::verify;
use soroban_auth::{Identifier, Signature};
use soroban_sdk::{contractimpl, panic_with_error, symbol, BigInt, Bytes, Env};
pub struct CarRentalContract;

/*
How to use this contract to rent a car
1. call initialize to set the admin
2. admin is able to call add_car to add a car to the contract. Or remove_car to remove a previous added car
3. client open a request to be able to rent a car (open_req)
4. admin approve or decline the client request (appr_req, deny_req). Client need to be approved to reserve a car
4. client reserve a car (resrve_car) if the car exist in the contract
5. client take the car (take_car) if previous reserved by the same client
6. client drop the car (drop_car) if previous taken. Change the status of the car to DropReview
7. admin accept or deny the drop (accpt_drop, deny_drop). if deny_drop it will change the status to DropReviewDenied. Client needs to call drop_car again again
*/

#[contractimpl]
impl CarRentalTrait for CarRentalContract {
    // Initialized the contract. Set the contract Admin.
    // Executed only one time.
    // Necessary to inform the public key of the admin.
    // After initialized, only this admin can execute the functions that contains ADMIN EXPECTED.
    fn init(env: Env, admin: Identifier) {
        // Check if the contract is already initialized and admin already set.
        if has_admin(&env) {
            panic_with_error!(&env, Error::AlreadyInit)
        }
        // write admin inside the contract storage.
        write_admin(&env, admin);
    }

    // Add a new car to the contract.
    // ADMIN EXPECTED
    // Necessary to inform: the plate (str), model (str), color (str) and horse power (int) of the car and the nonce (int).
    fn add_car(
        env: Env,
        admin: Signature,
        nonce: BigInt,
        plate: Bytes,
        model: Bytes,
        color: Bytes,
        horse: i32,
    ) {
        // verify if admin informed is the same admin that initialized the contract.
        check_admin(&env, &admin);
        // consume the nonce to avoid replay call contract.
        verify_and_consume_nonce(&env, &admin, &nonce);

        // get the admin identifier based on the admin signature.
        let admin_id = admin.identifier(&env);

        // verify the signature of the admin to that specific call.
        verify(&env, &admin, symbol!("add_car"), (admin_id, nonce));

        // check if the car has already been added based on the plate.
        if has_car(&env, &plate) {
            panic_with_error!(&env, Error::CarAlreadyExists)
        }

        // write the car inside the contract storage.
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

    // Remove the car from the storage based on the plate(str).
    // ADMIN EXPECTED
    // Necessary inform the plate (str) and the nonce (int) to avoid replay call.
    fn remove_car(env: Env, admin: Signature, nonce: BigInt, plate: Bytes) {
        // verify if admin informed is the same admin that initialized the contract.
        check_admin(&env, &admin);
        // consume the nonce to avoid replay call contract.
        verify_and_consume_nonce(&env, &admin, &nonce);

        // get the admin identifier based on the admin signature.
        let admin_id = admin.identifier(&env);

        // verify the signature of the admin to that specific call.
        verify(&env, &admin, symbol!("remove_car"), (admin_id, nonce));

        // panic if car not exist in the contract storage
        if !has_car(&env, &plate) {
            panic_with_error!(&env, Error::CarNotExists)
        }

        // panic if the car is rented.
        if has_rented_car(&env, &plate) {
            panic_with_error!(&env, Error::CarAlreadyRented)
        }

        // remove the car from the contract storage. Necessary car not be rented.
        remove_car(&env, &plate)
    }

    // Approved a client request to be allowed to rent a car.
    // ADMIN EXPECTED
    // Necessary the client public key
    fn appr_req(env: Env, admin: Signature, client: Identifier, nonce: BigInt) {
        // verify if admin informed is the same admin that initialized the contract.
        check_admin(&env, &admin);
        // consume the nonce to avoid replay call contract.
        verify_and_consume_nonce(&env, &admin, &nonce);

        // get the admin identifier based on the admin signature.
        let admin_id = admin.identifier(&env);
        // verify the signature of the admin to that specific call.
        verify(&env, &admin, symbol!("appr_req"), (admin_id, nonce));

        // write the client status Approved to this specific client.
        write_client(&env, client, ClientStatus::Approved)
    }

    // Deny a client request to be allowed to rent a car.
    // ADMIN EXPECTED
    // Necessary the client public key
    fn deny_req(env: Env, admin: Signature, client: Identifier, nonce: BigInt) {
        // verify if admin informed is the same admin that initialized the contract.
        check_admin(&env, &admin);
        // consume the nonce to avoid replay call contract.
        verify_and_consume_nonce(&env, &admin, &nonce);
        // get the admin identifier based on the admin signature.
        let admin_id = admin.identifier(&env);
        // verify the signature of the admin to that specific call.
        verify(&env, &admin, symbol!("deny_req"), (admin_id, nonce));
        // write the client status Declined to this specific client.
        write_client(&env, client, ClientStatus::Declined)
    }

    // Accept the client drop car
    // ADMIN EXPECTED
    // Necessary the car plate (str), nonce (int).
    fn accpt_drop(env: Env, admin: Signature, nonce: BigInt, plate: Bytes) {
        // verify if admin informed is the same admin that initialized the contract.
        check_admin(&env, &admin);
        // consume the nonce to avoid replay call contract.
        verify_and_consume_nonce(&env, &admin, &nonce);
        // get the admin identifier based on the admin signature.
        let admin_id = admin.identifier(&env);
        // verify the signature of the admin to that specific call.
        verify(&env, &admin, symbol!("accpt_drop"), (admin_id, nonce));

        //  check if the car was rented
        if !has_rented_car(&env, &plate) {
            panic_with_error!(&env, Error::CarIsNotRented)
        }

        //  get the rended car data
        let rented_car_data = read_rented_car(&env, &plate);

        // check if the car status is in drop review. If not it is not requested to drop
        if rented_car_data.status != RentedCarStatus::DropReview {
            panic_with_error!(&env, Error::CarIsNotInDropReview)
        }

        // remove the rented car from the contract storage.
        // The car is not rented anymore by any client.
        remove_rented_car(&env, &plate)
    }

    // Deny the client drop car
    // ADMIN EXPECTED
    // Necessary the car plate (str), nonce (int).
    fn deny_drop(env: Env, admin: Signature, nonce: BigInt, plate: Bytes) {
        // verify if admin informed is the same admin that initialized the contract.
        check_admin(&env, &admin);
        // consume the nonce to avoid replay call contract.
        verify_and_consume_nonce(&env, &admin, &nonce);
        // get the admin identifier based on the admin signature.
        let admin_id = admin.identifier(&env);
        // verify the signature of the admin to that specific call.
        verify(&env, &admin, symbol!("accpt_drop"), (admin_id, nonce));

        // check if car is rented
        if !has_rented_car(&env, &plate) {
            panic_with_error!(&env, Error::CarIsNotRented)
        }

        // get the rented car data
        let mut rented_car_data = read_rented_car(&env, &plate);

        // verify if car status id DropReview. If not it is not requested to drop
        if rented_car_data.status != RentedCarStatus::DropReview {
            panic_with_error!(&env, Error::CarIsNotInDropReview)
        }

        // change the rented car status to DropReviewDenied
        rented_car_data.status = RentedCarStatus::DropReviewDenied;
        // write the rented car data to the contract storage
        write_rented_car(&env, &plate, rented_car_data)
    }
    // Change the admin for this contract
    // ADMIN EXPECTED
    // Necessary the new_admin public key, nonce (int), new admin signature (str).
    fn set_admin(env: Env, admin: Signature, nonce: BigInt, new_admin: Identifier) {
        // verify if admin informed is the same admin that initialized the contract.
        check_admin(&env, &admin);
        // consume the nonce to avoid replay call contract.
        verify_and_consume_nonce(&env, &admin, &nonce);
        // get the admin identifier based on the admin signature.
        let admin_id = admin.identifier(&env);
        // verify the signature of the admin to that specific call.
        verify(&env, &admin, symbol!("set_admin"), (admin_id, nonce));
        // write the new_admin to the contract storage
        write_admin(&env, new_admin);
    }

    // Client open request to be able to reserve and rent a caker
    // Necessary the client public key, nonce (int).
    fn open_req(env: Env, client: Signature, nonce: BigInt) {
        // consume the nonce to avoid replay call contract.
        verify_and_consume_nonce(&env, &client, &nonce);
        // get the client identifier based on the client signature.
        let client_identifier = client.identifier(&env);
        verify(
            &env,
            &client,
            symbol!("open_req"),
            (&client_identifier, nonce),
        );
        // write the client status Pending to this specific client in the storage
        write_client(&env, client_identifier, ClientStatus::Pending)
    }

    // Client reserve a car to be able to take it later
    // Necessary the client public key, nonce (int), car plate (str).
    fn resrve_car(env: Env, client: Signature, nonce: BigInt, plate: Bytes) {
        // consume the nonce to avoid replay call contract.
        verify_and_consume_nonce(&env, &client, &nonce);
        // get the client identifier based on the client signature.
        let client_identifier = client.identifier(&env);
        // verify the signature of the client to that specific call.
        verify(
            &env,
            &client,
            symbol!("resrve_car"),
            (&client_identifier, nonce),
        );

        // check if the client is authorized to reserve a car (client status approved)
        if !is_client_authorized(&env, &client_identifier) {
            panic_with_error!(&env, Error::ClientNotAuthorized)
        }
        // check if the car exists in the contract storage
        if !has_car(&env, &plate) {
            panic_with_error!(&env, Error::CarNotExists)
        }

        // check if the car is already rented
        if has_rented_car(&env, &plate) {
            panic_with_error!(&env, Error::CarAlreadyRented)
        }

        // write the rented car data to the contract storage with status Reserved
        write_rented_car(
            &env,
            &plate,
            RentedCarDataKey {
                renter: client_identifier,
                status: RentedCarStatus::Reserved,
            },
        )
    }

    // Client take a car (previous reserved)
    // Necessary the client public key, nonce (int), car plate (str).
    fn take_car(env: Env, client: Signature, nonce: BigInt, plate: Bytes) {
        // consume the nonce to avoid replay call contract.
        verify_and_consume_nonce(&env, &client, &nonce);
        // get the client identifier based on the client signature.
        let client_identifier = client.identifier(&env);
        // verify the signature of the client to that specific call.
        verify(
            &env,
            &client,
            symbol!("take_car"),
            (&client_identifier, nonce),
        );

        // check if car is reserved
        if !has_rented_car(&env, &plate) {
            panic_with_error!(&env, Error::CarIsNotRented)
        }

        // get car data from the contract storage
        let mut rented_car_data = read_rented_car(&env, &plate);
        //  check if the renter is the same client that are requesting to take the car
        if rented_car_data.renter != client_identifier {
            panic_with_error!(&env, Error::ClientIsNotRenter)
        }
        // check if the car status is Reserved. If not it is not possible to take the car
        if rented_car_data.status != RentedCarStatus::Reserved {
            panic_with_error!(&env, Error::CarIsNotReserved)
        }

        // change car status to rented
        rented_car_data.status = RentedCarStatus::Rented;
        // write the rented car data to the contract storage
        write_rented_car(&env, &plate, rented_car_data)
    }

    // Client request to drop the car
    // Necessary the client public key, nonce (int), car plate (str).
    fn drop_car(env: Env, client: Signature, nonce: BigInt, plate: Bytes) {
        // consume the nonce to avoid replay call contract.
        verify_and_consume_nonce(&env, &client, &nonce);
        // get the client identifier based on the client signature.
        let client_identifier = client.identifier(&env);
        // verify the signature of the client to that specific call.
        verify(
            &env,
            &client,
            symbol!("drop_car"),
            (&client_identifier, nonce),
        );

        // check if car is rented
        if !has_rented_car(&env, &plate) {
            panic_with_error!(&env, Error::CarIsNotRented)
        }

        // get car data from the contract storage
        let mut rented_car_data = read_rented_car(&env, &plate);
        // check if the renter is the same client that are requesting to drop the car
        if rented_car_data.renter != client_identifier {
            panic_with_error!(&env, Error::ClientIsNotRenter)
        }
        // check if the car status is Rented or DropReviewDenied. If not it is not possible to drop the car
        // client can request the drop_car if has already been denied. It's possible to retry
        match rented_car_data.status {
            RentedCarStatus::Rented | RentedCarStatus::DropReviewDenied => {
                // change car status to DropReview
                rented_car_data.status = RentedCarStatus::DropReview;
                // write the rented car data to the contract storage
                write_rented_car(&env, &plate, rented_car_data)
            }
            _ => panic_with_error!(&env, Error::CarIsNotWithRentedStatus),
        }
    }

    // return the client status
    fn read_clnt(env: Env, client: Identifier) -> ClientStatus {
        read_client(&env, client)
    }

    // return the nonce from this contract execution
    fn nonce(env: Env, identifier: Identifier) -> BigInt {
        read_nonce(&env, &identifier)
    }

    // return the car data
    fn read_car(env: Env, plate: Bytes) -> CarDataKey {
        read_car(&env, plate)
    }

    // return the car renter and the status
    fn read_rent(env: Env, plate: Bytes) -> RentedCarDataKey {
        read_rented_car(&env, &plate)
    }

    // verify if car is rented
    // return true if car is rented
    fn has_rent(env: Env, plate: Bytes) -> bool {
        has_rented_car(&env, &plate)
    }

    // return the admin identifier
    fn read_admin(env: Env) -> Identifier {
        read_admin(&env)
    }
}

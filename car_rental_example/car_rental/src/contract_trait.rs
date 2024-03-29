use crate::storage_types::{CarDataKey, ClientStatus, RentedCarDataKey};
use soroban_auth::{Identifier, Signature};
use soroban_sdk::{BigInt, Bytes, Env};

/*
This trait represents the basic structure of this contract.
It's function implementation are defined at file contract.
Every function (fn) that contains ADMIN EXPECTED means it's necessary the admin to execute it (admin signature).
*/
pub trait CarRentalTrait {
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
    fn set_admin(env: Env, admin: Signature, nonce: BigInt, new_admin: Identifier);
    fn read_admin(env: Env) -> Identifier;
}

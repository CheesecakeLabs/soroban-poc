use crate::storage_types::{CarDataKey, ClientStatus, RentedCarDataKey};
use soroban_auth::{Identifier, Signature};
use soroban_sdk::{BigInt, Bytes, Env};
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

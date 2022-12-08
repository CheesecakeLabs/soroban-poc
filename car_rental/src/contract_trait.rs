use crate::storage_types::{CarDataKey, ClientStatus, RentedCarDataKey};
use soroban_auth::{Identifier, Signature};
use soroban_sdk::{Bytes, Env};
pub trait CarRentalTrait {
    /// # Admin Flow
    fn init(env: Env, admin: Identifier);
    fn add_car(
        env: Env,
        admin: Signature,
        nonce: i128,
        plate: Bytes,
        model: Bytes,
        color: Bytes,
        horse: i32,
    );
    fn remove_car(env: Env, admin: Signature, nonce: i128, plate: Bytes);
    fn appr_req(env: Env, admin: Signature, client: Identifier, nonce: i128);
    fn deny_req(env: Env, admin: Signature, client: Identifier, nonce: i128);
    fn open_req(env: Env, client: Signature, nonce: i128);
    fn take_car(env: Env, client: Signature, nonce: i128, plate: Bytes);
    fn read_car(env: Env, plate: Bytes) -> CarDataKey;
    fn resrve_car(env: Env, client: Signature, nonce: i128, plate: Bytes);
    fn drop_car(env: Env, client: Signature, nonce: i128, plate: Bytes);
    fn accpt_drop(env: Env, admin: Signature, nonce: i128, plate: Bytes);
    fn deny_drop(env: Env, admin: Signature, nonce: i128, plate: Bytes);
    fn read_clnt(env: Env, client: Identifier) -> ClientStatus;
    fn nonce(env: Env, identifier: Identifier) -> i128;
    fn read_rent(env: Env, plate: Bytes) -> RentedCarDataKey;
    fn has_rent(env: Env, plate: Bytes) -> bool;
}

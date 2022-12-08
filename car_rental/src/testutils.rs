#![cfg(any(test, feature = "testutils"))]

use crate::contract::CarRentalContractClient;
use ed25519_dalek::Keypair;
use soroban_auth::{Ed25519Signature, Identifier, Signature, SignaturePayload, SignaturePayloadV0};
use soroban_sdk::testutils::ed25519::Sign;
use soroban_sdk::{symbol, Bytes, BytesN, Env, IntoVal};

pub struct CarRental {
    env: Env,
    contract_id: BytesN<32>,
}

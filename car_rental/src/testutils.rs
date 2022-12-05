#![cfg(any(test, feature = "testutils"))]

use crate::contract::CarRentalContractClient;
use ed25519_dalek::Keypair;
use soroban_auth::{Ed25519Signature, Identifier, Signature, SignaturePayload, SignaturePayloadV0};
use soroban_sdk::testutils::ed25519::Sign;
use soroban_sdk::{symbol, BigInt, Bytes, BytesN, Env, IntoVal};

pub struct CarRental {
    env: Env,
    contract_id: BytesN<32>,
}

impl CarRental {
    pub fn new(env: &Env, contract_id: &[u8; 32]) -> Self {
        Self {
            env: env.clone(),
            contract_id: BytesN::from_array(env, contract_id),
        }
    }

    pub fn init(&self, admin: &Identifier) {
        CarRentalContractClient::new(&self.env, &self.contract_id).initialize(&admin);
    }

    fn open_req(
        user_1_sign: impl Identifier
            + Sign<soroban_auth::SignaturePayload, Signature = [u8; 64]>
            + Debug,
        contract_id: soroban_sdk::BytesN<32>,
        user_1_id: &Identifier,
        client: &CarRentalContractClient,
    ) {
        let nonce = BigInt::from_u32(self.env, 0);
        let sig = soroban_auth::testutils::ed25519::sign(
            self.env,
            &user_1_sign,
            &contract_id,
            symbol!("open_req"),
            (user_1_id, &nonce),
        );
        client.open_req(&sig, &nonce);
    }
}

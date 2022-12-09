use crate::contract::CarRentalContractClient;
use crate::storage_types::{CarDataKey, ClientStatus, RentedCarDataKey};
use core::fmt::Debug;
use soroban_auth::{Identifier, Signature};
use soroban_sdk::{symbol, BigInt, BytesN, Env, IntoVal};

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
    pub fn deny_request(&self, admin: &soroban_sdk::AccountId, client_id: Identifier) {
        CarRentalContractClient::new(&self.env, &self.contract_id)
            .with_source_account(&admin)
            .deny_req(
                &Signature::Invoker,
                &client_id,
                &BigInt::from_u32(&self.env, 0),
            );
    }

    pub fn approve_request(&self, admin: &soroban_sdk::AccountId, client_id: Identifier) {
        CarRentalContractClient::new(&self.env, &self.contract_id)
            .with_source_account(&admin)
            .appr_req(
                &Signature::Invoker,
                &client_id,
                &BigInt::from_u32(&self.env, 0),
            );
    }

    pub fn add_car(&self, admin: &soroban_sdk::AccountId, plate: &str, car_data: CarDataKey) {
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

    pub fn remove_car(&self, admin: &soroban_sdk::AccountId, plate: &str) {
        CarRentalContractClient::new(&self.env, &self.contract_id)
            .with_source_account(&admin)
            .remove_car(
                &Signature::Invoker,
                &BigInt::from_u32(&self.env, 0),
                &plate.into_val(&self.env),
            );
    }

    pub fn deny_drop(&self, admin: &soroban_sdk::AccountId, plate: &str) {
        CarRentalContractClient::new(&self.env, &self.contract_id)
            .with_source_account(&admin)
            .deny_drop(
                &Signature::Invoker,
                &BigInt::from_u32(&self.env, 0),
                &plate.into_val(&self.env),
            );
    }

    pub fn accept_drop(&self, admin: &soroban_sdk::AccountId, plate: &str) {
        CarRentalContractClient::new(&self.env, &self.contract_id)
            .with_source_account(&admin)
            .accpt_drop(
                &Signature::Invoker,
                &BigInt::from_u32(&self.env, 0),
                &plate.into_val(&self.env),
            );
    }

    pub fn set_admin(&self, admin: &soroban_sdk::AccountId, new_admin: Identifier) {
        CarRentalContractClient::new(&self.env, &self.contract_id)
            .with_source_account(&admin)
            .set_admin(
                &Signature::Invoker,
                &BigInt::from_u32(&self.env, 0),
                &new_admin,
            );
    }

    // Client methods (using Ed25519 Signature)
    pub fn open_request(
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

    pub fn reserve_car(
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

    pub fn take_car(
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

    pub fn drop_car(
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

    pub fn read_admin(&self) -> Identifier {
        return CarRentalContractClient::new(&self.env, &self.contract_id).read_admin();
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

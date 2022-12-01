#![no_std]
use soroban_auth::verify;
use soroban_auth::{Identifier, Signature};
use soroban_sdk::{contractimpl, contracttype, symbol, BigInt, Env};

// #[contracterror]
// #[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
// #[repr(u32)]
// pub enum Error {
//     GameNotStarted = 1,
//     MaxPlayersHit = 2,
//     InvalidReveal = 3,
//     InvalidOp = 4,
//     NotRevealed = 5,
//     LimitNotReached = 6,
//     InvalidSignature = 7,
// }

#[contracttype]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum ClientStatus {
    Pending = 0,
    Approved = 1,
    Declined = 2,
}

// #[contracttype]
// #[derive(Clone, Default, Debug, Eq, PartialEq)]
// pub struct CarDataKey {
//     pub model: String,
//     pub color: String,
//     pub horse: i32,
// }

#[derive(Clone)]
#[contracttype]
pub enum DataKey {
    Nonce(Identifier),
    Client(Identifier),
    Admin,
    //Cars(Vec<CarDataKey>)
}

fn write_client(env: &Env, client: Identifier, status: ClientStatus) {
    env.data().set(DataKey::Client(client), status)
}

fn read_client(env: &Env, client: Identifier) -> ClientStatus {
    return env.data().get_unchecked(DataKey::Client(client)).unwrap();
}

fn write_admin(env: &Env, admin: Identifier) {
    env.data().set(DataKey::Admin, admin)
}

fn has_admin(env: &Env) -> bool {
    return env.data().has(DataKey::Admin);
}

fn read_admin(env: &Env) -> Identifier {
    return env.data().get_unchecked(DataKey::Admin).unwrap();
}

fn check_admin(env: &Env, auth: &Signature) {
    let auth_id = auth.identifier(&env);
    if auth_id != read_admin(&env) {
        panic!("Not authorized!")
    }
}
fn read_nonce(e: &Env, id: &Identifier) -> BigInt {
    let key = DataKey::Nonce(id.clone());
    e.data()
        .get(key)
        .unwrap_or_else(|| Ok(BigInt::zero(e)))
        .unwrap()
}

fn verify_and_consume_nonce(e: &Env, auth: &Signature, expected_nonce: &BigInt) {
    match auth {
        Signature::Invoker => {
            if BigInt::zero(&e) != expected_nonce {
                panic!("nonce should be zero for Invoker")
            }
            return;
        }
        _ => {}
    }

    let id = auth.identifier(&e);
    let key = DataKey::Nonce(id.clone());
    let nonce = read_nonce(e, &id);

    if nonce != expected_nonce {
        panic!("incorrect nonce")
    }
    e.data().set(key, &nonce + 1);
}

pub trait CarRentalTrait {
    fn init(env: Env, admin: Identifier);
    fn add_car();
    fn remove_car();
    fn appr_req(env: Env, admin: Signature, client: Identifier, nonce: BigInt);
    fn deny_req(env: Env, admin: Signature, client: Identifier, nonce: BigInt);
    fn drop_insp();
    fn open_req(env: Env, client: Signature, nonce: BigInt);
    fn take_car();
    fn drop_car();
    fn take_insp();
    fn read_clnt(env: Env, client: Identifier) -> ClientStatus;
}

pub struct CarRentalContract;

#[contractimpl]
impl CarRentalTrait for CarRentalContract {
    // Admin
    fn init(env: Env, admin: Identifier) {
        if has_admin(&env) {
            panic!("already init")
        }
        write_admin(&env, admin);
    }

    fn add_car() {}

    fn remove_car() {}

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

    fn drop_insp() {}

    // User
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

    fn take_car() {}

    fn drop_car() {}

    fn take_insp() {}

    fn read_clnt(env: Env, client: Identifier) -> ClientStatus {
        read_client(&env, client)
    }
}

#[cfg(test)]
mod test;

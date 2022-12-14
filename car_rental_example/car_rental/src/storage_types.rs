use soroban_sdk::{contracttype};
use soroban_auth::{Identifier};
use soroban_sdk::{Bytes};

// This file contains all the storage variable types.
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CarDataKey {
    pub model: Bytes,
    pub color: Bytes,
    pub horse: i32,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RentedCarDataKey {
    pub renter: Identifier,
    pub status: RentedCarStatus,
}


#[contracttype]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum ClientStatus {
    Pending = 0,
    Approved = 1,
    Declined = 2,
}

#[contracttype]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum RentedCarStatus {
    Reserved = 0,
    Rented = 1,
    DropReview = 2,
    DropReviewDenied = 3,
}


#[derive(Clone)]
#[contracttype]
pub enum DataKey {
    Nonce(Identifier),
    Client(Identifier),
    Admin,
    Car(Bytes),       // = CarDataKey
    RentedCar(Bytes), // = RentedCarDataKey
}

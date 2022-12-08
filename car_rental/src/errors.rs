use soroban_sdk::{contracterror};

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum Error {
    NotAuthorized = 1,
    NonceShouldBeZero = 2,
    IncorrectNonce = 3,
    AlreadyInit = 4,
    CarAlreadyExists = 5,
    CarNotExists = 6,
    CarAlreadyRented = 7,
    CarIsNotRented = 8,
    CarIsNotInDropReview = 9,
    CarIsNotReserved = 10,
    ClientIsNotRenter = 11,
    CarIsNotWithRentedStatus = 12,
    ClientNotAuthorized = 13,
}

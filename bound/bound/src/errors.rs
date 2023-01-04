use soroban_sdk::contracterror;

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum Error {
    AlreadyInitialized = 1,
    NotAuthorized = 2,
    NotInitialized = 3,
    AlreadyLiquidated = 4,
    InvalidTimestamp = 5,
    NotLiquidated = 6,
}

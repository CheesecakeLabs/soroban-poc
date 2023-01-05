use soroban_sdk::contracterror;

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum Error {
    AlreadyInitialized = 1,
    NotAuthorized = 2,
    NotInitialized = 3,
    AlreadyCashOutEn = 4,
    InvalidTimestamp = 5,
    NotCashOutEn = 6,
    NotAvailable = 7,
    NotEnoughTokens = 8,
    EndTimeNotPassed = 9,
    UserAlreadyAllowed = 10,
    UserNotAllowed = 11,
    NotPaused = 12,
}

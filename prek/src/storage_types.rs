use soroban_sdk::contracttype;

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum State {
    NotStarted,
    Started,
    Available,
    Liquidated,
}

#[derive(Clone)]
#[contracttype]
pub enum DataKey {
    State,      // State
    Admin,      // Identifier
    PaymentTkn, // BytesN<32>
    PrekTkn,    // BytesN<32>
    FeeIntrvl,  // u32
    FeeRate,    // i128
    InitPrice,  // i128
    Price,      // i128
    InitTime,   // u64
    EndTime,    // u64
    AmountSold, // i128
}

use soroban_sdk::contracttype;

#[derive(Clone)]
#[contracttype]
pub enum DataKey {
    StakingTkn, // BytesN<32>
    ShareTkn,   // BytesN<32>
    Rate,       // i128
    TotSupply,  // i128
    TotShare,   // i128
    LastUpdate, // u64
}

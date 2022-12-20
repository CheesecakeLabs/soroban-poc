use soroban_auth::Identifier;
use soroban_sdk::contracttype;

#[derive(Clone)]
#[contracttype]
pub enum DataKey {
    StakingTkn,           // BytesN<32>
    RewardsTkn,           // BytesN<32>
    RewardRate,           // i128
    TotSupply,            // i128
    LastUpdate,           // u64
    RwrdStored,           // i128, Reward per Token Stored
    Balances(Identifier), // i128
    Rewards(Identifier),  // i128
    RwrdPaid(Identifier), // i128, User Reward per Token Paid
}

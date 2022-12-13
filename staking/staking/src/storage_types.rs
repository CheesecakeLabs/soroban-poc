use soroban_auth::Identifier;
use soroban_sdk::contracttype;

#[derive(Clone)]
#[contracttype]
pub enum DataKey {
    StakingTkn,           // BytesN<32>
    RewardsTkn,           // BytesN<32>
    RewardRate,           // u64
    TotSupply,            // u64
    LastUpdate,           // u64
    RwrdStored,           // u64
    Balances(Identifier), // u64
    Rewards(Identifier),  // u64
    RwrdPaid(Identifier), // u64
}

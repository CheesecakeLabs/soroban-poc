use soroban_auth::Identifier;
use soroban_sdk::contracttype;

#[derive(Clone)]
#[contracttype]
pub enum DataKey {
    StakingTkn,           // BytesN<32> ok
    RewardsTkn,           // BytesN<32> ok 
    RewardRate,           // i128 ok
    TotSupply,            // i128 Total supply of staking tokens ok
    LastUpdate,           // u64 ok 
    RwrdStored,           // i128 ok
    Balances(Identifier), // i128 Some address how much they staked
    Rewards(Identifier),  // i128 Some address how much rewards each address has
    RwrdPaid(Identifier), // i128 Some address how much they claimed
}

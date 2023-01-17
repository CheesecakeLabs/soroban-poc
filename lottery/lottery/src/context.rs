use soroban_sdk::contracttype;

#[derive(Clone, Debug, PartialEq, Eq)]
#[contracttype]
pub enum State {
    NotInititd,
    Initiated,
    Started,
    Finished,
}

#[derive(Clone)]
#[contracttype]
pub enum DataKey {
    Admin,      // Identifier
    GainPctge,  // i128
    TokenAddr,  // BytesN
    State,      // enum State
    TicktPrice, // i128
    Users,      // Vec<Identifier>
}

use soroban_sdk::{contracttype, Address};

// Current version of the Stream struct
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Stream {
    pub sender: Address,
    pub receiver: Address,
    pub token: Address,
    pub amount: i128,
    pub start_time: u64,
    pub cliff_time: u64,
    pub end_time: u64,
    pub withdrawn_amount: i128,
}

// Legacy Stream struct (v1) - for migration example
// This represents an older version without cliff_time
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct LegacyStream {
    pub sender: Address,
    pub receiver: Address,
    pub token: Address,
    pub amount: i128,
    pub start_time: u64,
    pub end_time: u64,
    pub withdrawn_amount: i128,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct StreamRequest {
    pub receiver: Address,
    pub amount: i128,
    pub start_time: u64,
    pub cliff_time: u64,
    pub end_time: u64,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum DataKey {
    Stream(u64),
    StreamId,
    Admin,
    FeeBps,
    Treasury,
    IsPaused,
    ContractVersion,        // Tracks current contract version
    MigrationExecuted(u32), // Tracks which migrations have been executed
}

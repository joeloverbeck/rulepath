//! Bot skeleton for Flood Watch.

pub const RANDOM_POLICY_ID: &str = "flood_watch_random_legal_v0";
pub const LEVEL1_POLICY_ID: &str = "flood_watch_level1_public_priority_v1";

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FloodWatchBotInput {
    pub policy_id: String,
}

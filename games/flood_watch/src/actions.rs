//! Action-path skeleton for Flood Watch.

pub const ACTION_BAIL: &str = "bail";
pub const ACTION_REINFORCE: &str = "reinforce";
pub const ACTION_FORECAST: &str = "forecast";
pub const ACTION_END_TURN: &str = "end_turn";

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum FloodWatchAction {
    Bail(String),
    Reinforce(String),
    Forecast,
    EndTurn,
}

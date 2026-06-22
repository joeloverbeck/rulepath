//! Earned shared helpers for game crates.
//!
//! This crate must stay rule-agnostic. Game behavior remains in `games/*`.

pub mod board_space;
pub mod seat;
pub mod trick_taking;

pub use seat::{SeatCount, SeatCountError, SeatCountRange, SeatCountRangeError, SeatIndexError};

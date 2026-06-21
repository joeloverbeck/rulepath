//! Vow Tide game crate.
//!
//! This crate keeps all Oh Hell-family nouns local to the game module. The
//! shared engine sees only generic Rulepath contracts.

pub mod actions;
pub mod cards;
pub mod effects;
pub mod ids;
pub mod replay_support;
pub mod rules;
pub mod scoring;
pub mod setup;
pub mod state;
pub mod variants;
pub mod visibility;

pub use setup::{setup_match, SetupOptions};
pub use variants::{load_manifest, load_variants};

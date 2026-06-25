//! Per-game browser-bridge implementations.
//!
//! Each submodule holds one game's bridge helpers (replay reconstruction,
//! effect/replay-document JSON serialization, etc.). They depend on the shared
//! concern modules at the crate root (json, seats, actors, commands, ...) and on
//! their own game crate, imported un-aliased since only one game lives here.

pub(crate) mod blackglass;
pub(crate) mod briar;
pub(crate) mod column;
pub(crate) mod directional;
pub(crate) mod draughts;
pub(crate) mod event;
pub(crate) mod flood;
pub(crate) mod frontier;
pub(crate) mod high_card;
pub(crate) mod masked;
pub(crate) mod plain;
pub(crate) mod poker;
pub(crate) mod race;
pub(crate) mod river;
pub(crate) mod secret;
pub(crate) mod three;
pub(crate) mod token;
pub(crate) mod vow;

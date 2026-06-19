//! Per-game browser-bridge implementations.
//!
//! Each submodule holds one game's bridge helpers (replay reconstruction,
//! effect/replay-document JSON serialization, etc.). They depend on the shared
//! concern modules at the crate root (json, seats, actors, commands, ...) and on
//! their own game crate, imported un-aliased since only one game lives here.

pub(crate) mod race;

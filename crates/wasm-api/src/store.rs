//! Per-thread match and replay store backing the browser API.
//!
//! Holds the live `MatchRecord` / `ReplayRecord` maps and the monotonic id
//! counters, plus the borrow-checked accessor helpers the rest of the bridge
//! uses to read and mutate them. State is thread-local so each WASM instance
//! (and each test thread) gets an isolated, deterministic store.

use std::cell::Cell;
use std::cell::RefCell;
use std::collections::BTreeMap;

use crate::json::{diagnostic_string, escape_json};
use crate::{MatchRecord, ReplayRecord};

thread_local! {
    pub(crate) static MATCHES: RefCell<BTreeMap<String, MatchRecord>> =
        const { RefCell::new(BTreeMap::new()) };
    pub(crate) static REPLAYS: RefCell<BTreeMap<String, ReplayRecord>> =
        const { RefCell::new(BTreeMap::new()) };
    pub(crate) static NEXT_MATCH_ID: Cell<u64> = const { Cell::new(1) };
    pub(crate) static NEXT_REPLAY_ID: Cell<u64> = const { Cell::new(1) };
}

pub(crate) fn next_replay_id(game_id: &str) -> String {
    NEXT_REPLAY_ID.with(|next| {
        let id = next.get();
        next.set(id.saturating_add(1));
        format!("{game_id}-replay-{id}")
    })
}

pub(crate) fn next_match_id(game_id: &str) -> String {
    NEXT_MATCH_ID.with(|next| {
        let id = next.get();
        next.set(id.saturating_add(1));
        format!("{game_id}-{id}")
    })
}

pub(crate) fn with_match<T>(
    match_id: &str,
    read: impl FnOnce(&MatchRecord) -> Result<T, String>,
) -> Result<T, String> {
    MATCHES.with(|matches| {
        let matches = matches.borrow();
        let record = matches
            .get(match_id)
            .ok_or_else(|| missing_match_json(match_id))?;
        read(record)
    })
}

pub(crate) fn with_match_mut<T>(
    match_id: &str,
    update: impl FnOnce(&mut MatchRecord) -> Result<T, String>,
) -> Result<T, String> {
    MATCHES.with(|matches| {
        let mut matches = matches.borrow_mut();
        let record = matches
            .get_mut(match_id)
            .ok_or_else(|| missing_match_json(match_id))?;
        update(record)
    })
}

pub(crate) fn with_replay<T>(
    replay_id: &str,
    read: impl FnOnce(&ReplayRecord) -> Result<T, String>,
) -> Result<T, String> {
    REPLAYS.with(|replays| {
        let replays = replays.borrow();
        let record = replays
            .get(replay_id)
            .ok_or_else(|| missing_replay_json(replay_id))?;
        read(record)
    })
}

pub(crate) fn missing_match_json(match_id: &str) -> String {
    format!(
        "{{\"code\":\"unknown_match\",\"message\":\"unknown match id: {}\"}}",
        escape_json(match_id)
    )
}

pub(crate) fn missing_replay_json(replay_id: &str) -> String {
    diagnostic_string("unknown_replay", &format!("unknown replay id: {replay_id}"))
}

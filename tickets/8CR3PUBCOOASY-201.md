# 8CR3PUBCOOASY-201: C-02 Plain Tricks typed seat parser

**Status**: PENDING
**Priority**: MEDIUM
**Effort**: Small
**Engine Changes**: Yes (deterministic evidence) — `games/plain_tricks/src/ids.rs`
**Deps**: 8CR3PUBCOOASY-001

## Problem

`plain_tricks` is the only R3 game with a game-local typed seat parser. It
hand-parses the seat string instead of delegating strict grammar to the shipped
`SeatId::parse_canonical`. C-02 makes the typed parser delegate canonical
grammar to the engine, then map the extracted index through
`PlainTricksSeat::from_index`, preserving the exact accepted/rejected language
and `as_str()` output. C-02 applies only to seat IDs — `TrickCardId`,
`TrickSuit`, `TrickRank` keep their own parsers.

## Assumption Reassessment (2026-06-24)

1. `games/plain_tricks/src/ids.rs::PlainTricksSeat::{parse (line ~55),
   from_index (line ~26), as_str (line ~48)}` exist; the shipped
   `SeatId::parse_canonical` is at `crates/engine-core/src/lib.rs:58`.
2. Spec §3.5 verdict for the Plain typed parser is `migrate`; §5.4 task
   `8C-R3-201` scopes `PlainTricksSeat::{parse,from_index,as_str}` and unit
   tests; non-seat IDs are explicitly excluded.
3. Cross-crate boundary under audit: `SeatId::parse_canonical` strict grammar vs
   the game-local two-string parser — only grammar delegation moves; index→seat
   mapping (`from_index`) and the canonical `as_str()` spelling stay local.
4. FOUNDATIONS §2 motivates this: legality/parsing authority stays in Rust and
   the canonical grammar is single-sourced; TypeScript must not normalize a seat
   ID.
5. Enforcement surface: the canonical accept/reject vector set (canonical
   acceptance, missing prefix, empty index, leading zero, sign, whitespace,
   non-ASCII digit, overflow, out-of-game index) plus existing traces; outputs
   and traces stay unchanged from the 001 baseline.

## Architecture Check

1. Delegating strict grammar to `SeatId::parse_canonical` removes a duplicated
   hand parser and single-sources the canonical seat grammar; cleaner and less
   drift-prone than maintaining a game-local grammar.
2. No backwards-compatibility alias — the manual two-string parser is replaced,
   not kept alongside.
3. `engine-core` already owns `SeatId`; no mechanic noun added, no
   `game-stdlib` change.

## Verification Layers

1. Grammar conformance -> `cargo test -p plain_tricks` seat-parse unit tests
   (canonical + every malformed case rejects as characterized).
2. Output stability -> `as_str()` and existing traces byte-identical to 001
   baseline (no `seat_<n>` spelling change).
3. Index mapping -> only indices 0/1 map to seats; out-of-game index rejects.

## What to Change

### 1. Delegate canonical grammar

In `PlainTricksSeat::parse`, call `SeatId::parse_canonical`, extract index `0`
or `1`, then map through `PlainTricksSeat::from_index`. Preserve the exact
accepted/rejected semantics and the `as_str()` output. Update/extend the unit
tests to cover the full canonical accept/reject vector set.

## Files to Touch

- `games/plain_tricks/src/ids.rs` (modify)

## Out of Scope

- `TrickCardId`, `TrickSuit`, `TrickRank`, or any non-seat ID parser.
- The WASM import-alias surface (ticket 202).
- Any change to `as_str()` output spelling or existing traces.

## Acceptance Criteria

### Tests That Must Pass

1. `cargo test -p plain_tricks` (seat-parse unit tests, full accept/reject set).
2. `cargo run -p replay-check -- --game plain_tricks --all` — byte-identical to baseline.

### Invariants

1. Accepted/rejected language and `as_str()` output are unchanged from baseline.
2. Strict grammar is decided by `SeatId::parse_canonical`; mapping stays local.

## Test Plan

### New/Modified Tests

1. `games/plain_tricks/src/ids.rs` (inline unit tests) — extend the seat-parse
   accept/reject vector set to cover canonical, malformed, and out-of-game cases.

### Commands

1. `cargo test -p plain_tricks`
2. `cargo run -p replay-check -- --game plain_tricks --all`
3. A per-game test + replay-check is the correct boundary: only the Plain seat
   parser changes; output spellings are asserted unchanged.

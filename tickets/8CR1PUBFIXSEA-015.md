# 8CR1PUBFIXSEA-015: Three Marks WASM output canonical seat migration

**Status**: PENDING
**Priority**: MEDIUM
**Effort**: Small
**Engine Changes**: Yes (deterministic evidence) — `wasm-api` (`src/seats.rs`, `src/games/three.rs`, `src/tests.rs`) + one golden trace
**Deps**: 8CR1PUBFIXSEA-012

## Problem

`crates/wasm-api/src/games/three.rs::three_replay_document_json` emits legacy hyphen seat spellings in its roster, command actors, and outcome seats. Migrate this one document to canonical `seat_<n>` via the `-012` output helper — an ADR-0009 `intentional-migration` that changes exactly one golden trace (`games/three_marks/tests/golden_traces/wasm-exported.trace.json`). Old hyphen documents stay importable.

## Assumption Reassessment (2026-06-23)

1. `three_replay_document_json` (`crates/wasm-api/src/games/three.rs`) currently formats seats via `trace_three_seat` returning hyphen `seat-<n>`; the canonical helper is added by `-012`. Confirmed during reassessment.
2. Spec §3.4 and §5.5 (task `8C-R1-213`) classify this as `intentional-migration`; only this game's `wasm-exported.trace.json` may change. MSC-8C-002 owns the seat grammar; ADR-0009 governs the receipt.
3. Cross-artifact: producer `seats.rs` (helper from `-012`) → consumer `games/three.rs` document → `tests.rs` assertion → the one golden trace. Seat-grammar contract is `engine-core::SeatId`.
4. §11 determinism/no-leak motivates this ticket: the only changed bytes are public seat-ID spellings; gameplay hashes remain semantically valid and public visibility is unchanged.
5. Enforcement surface = the WASM exported document bytes and its golden trace; the migration changes only seat-ID-bearing fields, is characterized before/after, and keeps old hyphen documents importable.

## Architecture Check

1. Flipping one document via the shared canonical helper is reversible and bounded, versus a six-document flag-day rewrite (forbidden).
2. No backwards-compatibility reader is removed; `parse_seat_import` still accepts hyphen/symbolic aliases.
3. `engine-core` stays noun-free (§3); change lives in `wasm-api`; no `game-stdlib` change (§4).

## Verification Layers

1. Document emits canonical `seat_<n>` in roster/actors/outcome -> golden trace / deterministic replay check (`replay-check --game three_marks --all`) + byte digest before/after.
2. Old hyphen document still imports; new document replays -> `wasm-api` round-trip test in `tests.rs`.
3. Only this golden file changed -> `git diff --name-only` shows exactly the Three Marks WASM trace among trace files.

## What to Change

### 1. Canonicalize the document seats

In `three_replay_document_json` (and `trace_three_seat` as needed), emit canonical seat IDs for roster, command actors, and outcome seats using the `-012` helper. Touch only seat-ID-bearing fields.

### 2. Update the assertion and golden trace

Update the `crates/wasm-api/src/tests.rs` expectation and regenerate ONLY `games/three_marks/tests/golden_traces/wasm-exported.trace.json`, recording the before/after byte+hash receipt.

## Files to Touch

- `crates/wasm-api/src/seats.rs` (modify; created/extended by 8CR1PUBFIXSEA-012)
- `crates/wasm-api/src/games/three.rs` (modify)
- `crates/wasm-api/src/tests.rs` (modify)
- `games/three_marks/tests/golden_traces/wasm-exported.trace.json` (modify)

## Out of Scope

- `default_seats`, non-WASM traces, game state bytes, or any other game's WASM document.
- `seats()` / `seats_for_count()`.
- Non-seat-ID bytes of the document.

## Acceptance Criteria

### Tests That Must Pass

1. `cargo test -p wasm-api` is green with the updated `tests.rs` expectation.
2. `cargo run -p replay-check -- --game three_marks --all` passes; the new document replays, public visibility and gameplay hashes remain valid.
3. `git diff --name-only` shows no other `wasm-exported.trace.json` changed.

### Invariants

1. Exactly one golden trace (`three_marks` WASM export) changes; all other traces are byte-identical.
2. Old hyphen-spelled documents remain importable.

## Test Plan

### New/Modified Tests

1. `games/three_marks/tests/golden_traces/wasm-exported.trace.json` — canonical seat spellings (migration receipt recorded).
2. `crates/wasm-api/src/tests.rs` — updated expected document + old-import / new-replay assertions.

### Commands

1. `cargo test -p wasm-api`
2. `cargo run -p replay-check -- --game three_marks --all`
3. The per-game replay-check plus the wasm-api round-trip test is the correct boundary.

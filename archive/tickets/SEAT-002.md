# SEAT-002: Preserve "Player N" seat labels for Race to N and Directional Flip

**Status**: COMPLETED
**Priority**: HIGH
**Effort**: Small
**Engine Changes**: Yes — `crates/wasm-api` (per-game catalog seat-label override for `RegisteredGame::RaceToN` and `RegisteredGame::DirectionalFlip`)
**Deps**: SEAT-001

## Problem

Race to N and Directional Flip intentionally present seats as "Player 1" / "Player 2"
in their play areas (`apps/web/src/components/RaceBoard.tsx:111-112`,
`apps/web/src/components/DirectionalFlipBoard.tsx:414-415`). They currently take
the default catalog seat labels (`catalog.rs:255-258` selects `None` for both, so
they fall through to `catalog_seat_labels_json`). After SEAT-001 flips the default
to "Seat N", their VIEWER would read "Seat 1"/"Seat 2", diverging from the "Player N"
naming the boards use.

Per the agreed convention (Rust owns labels; default is "Seat N" but each game may
declare its own labels), Race to N and Directional Flip must supply a Rust-side
"Player N" override so the VIEWER and the (soon-to-be-Rust-driven, SEAT-005) play
area agree on "Player 1" / "Player 2".

## Assumption Reassessment (2026-06-21)

1. `crates/wasm-api/src/catalog.rs:255-258`: only `EventFrontier` overrides
   `seat_labels_json`; `RaceToN` and `DirectionalFlip` resolve to `None` → default
   generator. Confirmed.
2. `docs/UI-INTERACTION.md` §3/§10B and `docs/MULTI-SEAT-AND-SURFACE-CONTRACT.md`
   §2: seat labels are Rust-owned, IP-safe display strings; a game may declare its
   own. "Player N" is an allowed original label.
3. Shared boundary: the per-game `seat_labels` catalog JSON override seam at
   `catalog.rs:255-258` and `with_catalog_seat_metadata`'s `seat_labels_json`
   parameter (`catalog.rs:43-57`). Reuse the existing override mechanism — the same
   path EventFrontier and River Ledger (`river_catalog_seat_labels_json`) use.
6. Schema check: additive value-only — produces the same `{"seat","label"}` entry
   shape with `label` = `"Player {index+1}"`. No consumer shape change.
8. Verify the actual seat counts at implementation time: both games use
   `DEFAULT_SEAT_COUNT` (`catalog.rs:259`). Confirm `DEFAULT_SEAT_COUNT == 2` (or
   the games' real counts) before hardcoding two entries; generate labels over the
   real `0..seat_count` range, not a hardcoded pair.

## Architecture Check

1. Reusing the existing `seat_labels_json` override seam (rather than special-casing
   in TS) keeps the "Player N" decision in Rust, consistent with how EventFrontier
   and River Ledger already override. No new mechanism is invented.
2. No backwards-compatibility shim: the override is the authoritative source; no
   alias to the old TS-invented "Player N".
3. `engine-core` untouched; no `game-stdlib` change. A small label generator in
   `wasm-api` is presentation metadata, not mechanic logic.

## Verification Layers

1. Race to N catalog labels are "Player 1"/"Player 2" -> schema/serialization
   validation (wasm-api catalog test for `RegisteredGame::RaceToN`).
2. Directional Flip catalog labels are "Player 1"/"Player 2" -> schema/serialization
   validation (wasm-api catalog test for `RegisteredGame::DirectionalFlip`).
3. Override routes through the existing seam, not a new field -> codebase grep-proof
   (the `match game` at `catalog.rs:255` gains arms for both games).

## What to Change

### 1. Add a "Player N" label generator

In `crates/wasm-api/src/catalog.rs`, add a small generator
(e.g. `catalog_player_labels_json(seat_count)`) producing
`{"seat":"seat_{index}","label":"Player {index + 1}"}` over `0..seat_count`,
mirroring the structure of `catalog_seat_labels_json`.

### 2. Route Race to N and Directional Flip through it

Extend the `match game` at `catalog.rs:255-258` with arms for
`RegisteredGame::RaceToN` and `RegisteredGame::DirectionalFlip` returning
`Some(catalog_player_labels_json(DEFAULT_SEAT_COUNT))`.

## Files to Touch

- `crates/wasm-api/src/catalog.rs` (modify)
- `crates/wasm-api/src/tests.rs` (modify)

## Out of Scope

- The 1-based default itself (SEAT-001).
- TS consumption of these labels (SEAT-004/005); this ticket only ensures Rust
  emits the right strings.

## Acceptance Criteria

### Tests That Must Pass

1. `cargo test -p wasm-api` — Race to N and Directional Flip catalogs assert
   "Player 1"/"Player 2".
2. `cargo test --workspace`
3. `cargo fmt --all --check && cargo clippy --workspace --all-targets -- -D warnings`

### Invariants

1. "Player N" naming is authored in Rust, not re-derived in TS.
2. The override uses the existing `seat_labels_json` seam (no new schema field).

## Test Plan

### New/Modified Tests

1. `crates/wasm-api/src/tests.rs` — assert the Race to N and Directional Flip
   catalog `seat_labels` are `"Player 1"`/`"Player 2"`.

### Commands

1. `cargo test -p wasm-api`
2. `cargo test --workspace`

## Outcome

Completed: 2026-06-21

Changed:
- Added a Rust-side catalog player-label generator that emits `Player 1` /
  `Player 2` over the existing `{seat,label}` shape.
- Routed `RegisteredGame::RaceToN` and `RegisteredGame::DirectionalFlip`
  through the existing per-game `seat_labels_json` override seam.
- Added `wasm-api` catalog assertions for Race to N and Directional Flip and
  regenerated the public API snapshot for the intended catalog value drift.

Deviations:
- None.

Verification:
- `UPDATE_API_SNAPSHOT=1 cargo test -p wasm-api --test api_surface` passed.
- `cargo test -p wasm-api` passed.
- `cargo test --workspace` passed.
- `cargo fmt --all --check` passed.
- `cargo clippy --workspace --all-targets -- -D warnings` passed.

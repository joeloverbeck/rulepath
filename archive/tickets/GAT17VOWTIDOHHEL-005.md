# GAT17VOWTIDOHHEL-005: Crate skeleton, workspace wiring, card model, variable-seat setup, schedule, phase state

**Status**: COMPLETED
**Priority**: HIGH
**Effort**: Large
**Engine Changes**: Yes — new crate `games/vow_tide/` (`src/{lib,ids,cards,setup,state}.rs`, `Cargo.toml`, `data/{manifest,variants}.toml` stubs); modifies root `Cargo.toml`
**Deps**: 001

## Problem

Establish the `games/vow_tide` crate and its deterministic foundations: a game-local card model, variable 3–7-seat setup with stable order and diagnostics, the `K(N)` hand-size schedule (down-to-1-up), and the explicit phase/state machine. This is the structural base every later gameplay ticket builds on.

## Assumption Reassessment (2026-06-21)

1. No `games/vow_tide/` exists; root `Cargo.toml` `members` lists 16 game crates (e.g. `games/briar_circuit`) and uses resolver `"2"` — add `"games/vow_tide"`. Sibling `games/briar_circuit/src` shows the canonical file set (`cards.rs`, `ids.rs`, `setup.rs`, `state.rs`, …).
2. Spec §3.1 + Appendix B fix the state contract: `Phase::{Bidding,PlayingTrick,Terminal}`, ordered 3–7 seat vector (`seat_0…seat_6`, labels `Tide 1…Tide 7`), schedule `K=min(10,floor(51/N))` (`K=10` for 3–5, `8` for 6, `7` for 7), total hands `2K-1`. `VT-SEATS-001`/`VT-SCHEDULE-001` (ticket 001) are the rule contract.
3. Cross-artifact boundary: the `Suit`/`Rank`/`Card`/`CardId` canonical ordering and the seat-index helpers are the shared contract for serialization (011), actions (007/008), and views (010); `load_manifest`/`load_variants` signatures must match what `fixture-check` calls (`<game>::load_manifest()`), so they are stubbed here.
4. FOUNDATIONS §3 is under audit: `engine-core` gains no card/seat/suit/rank/schedule noun — these are game-local types only.

## Architecture Check

1. A single foundation ticket owning the crate skeleton + types + setup + schedule + empty-phase state gives later tickets stable modules to extend, mirroring sibling-game decomposition.
2. No shims; all-new files under a new crate dir (legitimate all-`(new)` per greenfield-crate rule).
3. `engine-core` stays noun-free; no `game-stdlib` change here (the helper is adopted by the trick ticket 008).

## Verification Layers

1. 3–7 accepted, below/above rejected with stable diagnostics → `cargo test -p vow_tide` setup unit tests (`VT_INVALID_SEAT_COUNT`).
2. Schedule math (`K`, `2K-1`, symmetry) for N=3..7 → property tests.
3. Card model canonical ordering / no duplicates → unit + serialization-order test.
4. Crate resolves in workspace → `cargo build -p vow_tide`.
5. `engine-core` noun-free → `bash scripts/boundary-check.sh`.

## What to Change

### 1. Crate skeleton + workspace

Create `games/vow_tide/Cargo.toml` (deps `engine-core`, `ai-core`, `game-stdlib`) and `src/lib.rs`; add `"games/vow_tide"` to root `Cargo.toml` `members`. Stub `data/manifest.toml` + `data/variants.toml` and the `load_manifest()`/`load_variants()` functions (finalized in 015).

### 2. Card model + ids

`cards.rs` / `ids.rs`: typed `Suit`, `Rank` (2..ace high), `Card`, stable `CardId`; canonical deterministic ordering for serialization/actions/tests.

### 3. Setup + schedule + phase state

`setup.rs`: validate 3–7 seats and stable order; derive `K` and the full down-to-1-up schedule; init match seed, dealer (`seat_0`), hand index, cumulative scores. `state.rs`: `Phase` enum + the state fields from Appendix B (seat vector, schedule, dealer, phase, active seat, per-seat hands/scores) with empty bidding/play scaffolding.

## Files to Touch

- `games/vow_tide/Cargo.toml` (new)
- `games/vow_tide/src/lib.rs` (new)
- `games/vow_tide/src/ids.rs` (new)
- `games/vow_tide/src/cards.rs` (new)
- `games/vow_tide/src/setup.rs` (new)
- `games/vow_tide/src/state.rs` (new)
- `games/vow_tide/data/manifest.toml` (new)
- `games/vow_tide/data/variants.toml` (new)
- `Cargo.toml` (modify)

## Out of Scope

- Deal/RNG/trump (006), bidding (007), trick play (008), scoring (009), visibility/effects (010), bots, WASM, web.
- Final manifest/variants/fixtures content + `fixture-check` registration (015).

## Acceptance Criteria

### Tests That Must Pass

1. `cargo test -p vow_tide` — setup/schedule/card unit + property tests pass.
2. `cargo build --workspace` — new member resolves.
3. `bash scripts/boundary-check.sh` — no kernel noun.

### Invariants

1. Every count outside `{3,4,5,6,7}` is rejected with a stable diagnostic; `seat_0` is initial dealer.
2. Schedule is `K..1..K` with one one-card hand, `2K-1` total, for every N.

## Test Plan

### New/Modified Tests

1. `games/vow_tide/tests/rules.rs` — seat-count acceptance/rejection, schedule derivation.
2. `games/vow_tide/tests/serialization.rs` — canonical card/state ordering round-trip.

### Commands

1. `cargo test -p vow_tide`
2. `cargo build --workspace && bash scripts/boundary-check.sh`
3. Narrower command rationale: per-crate test is the correct boundary; deal/trump determinism is proven in 006.

## Outcome

Completed on 2026-06-21.

- Added the `games/vow_tide` workspace crate with game-local card, seat, schedule, setup, phase/state, and inert metadata modules.
- Added flat `data/manifest.toml` and `data/variants.toml` stubs plus `load_manifest()` / `load_variants()` loaders that reject unknown or behavior-looking fields.
- Wired `games/vow_tide` into the workspace and captured the new local package in `Cargo.lock`.
- Added setup/schedule and serialization-order tests for accepted/rejected seat counts, stable `VT_INVALID_SEAT_COUNT`, `K..1..K` schedules, canonical 52-card ordering, deterministic state summaries, and metadata loading.

Verification:

- `cargo fmt --all --check` passed.
- `cargo test -p vow_tide` passed: 6 integration tests plus crate/doc test harnesses.
- `cargo build --workspace` passed.
- `bash scripts/boundary-check.sh` passed with `engine-core boundary check passed`.

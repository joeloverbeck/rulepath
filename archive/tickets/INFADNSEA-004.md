# INFADNSEA-004: Infra B — seat-keyed deterministic simulator summaries

**Status**: COMPLETED
**Priority**: MEDIUM
**Effort**: Medium
**Engine Changes**: Yes — `tools/simulate` (summary aggregation); no `engine-core`, `game-stdlib`, or `games/*` behavior change
**Deps**: None

## Problem

`tools/simulate` aggregates results with fixed two-seat scalar counters — `seat_0_wins` / `seat_1_wins` / `seat_0_n` / `seat_1_n` (`tools/simulate/src/main.rs:87-88, 267-268, 295-383`) — repeated per game. This cannot represent a 3+ seat game's standings, so the first N-seat game (Gate 15) would have no usable simulation summary. This ticket generalizes the summary to deterministic seat-keyed maps (ordered seat IDs; win/loss/draw counts keyed by seat ID; terminal-reason counts; failure sample seeds) with stable machine-readable key ordering, while keeping the existing two-seat output byte-stable via a parity check. Simulate drives games natively (not via the WASM bridge), so this is independent of the bridge work in INFADNSEA-001.

## Assumption Reassessment (2026-06-14)

1. `tools/simulate/src/main.rs` holds the per-game summary structs and aggregation loops using `seat_0_wins`/`seat_1_wins`/`seat_0_n`/`seat_1_n` (lines 87-88, 267-268, 295-383); each game has its own counter block. Simulate constructs games natively via `*setup_match` + step, not through `crates/wasm-api`.
2. Grounded in `specs/infra-a-d-n-seat-public-infrastructure.md` WB4, and `docs/MULTI-SEAT-AND-SURFACE-CONTRACT.md §13` (simulator summaries: games completed/failed, ordered seat IDs, win counts keyed by seat ID, terminal-reason counts, sample seeds, deterministic key ordering) — §13 explicitly defers this generalization to "the later Infra B spec".
3. Shared boundary under audit: the `simulate` summary output format. It is a tool-internal report, not a serialized replay/hash contract — changing it does not touch trace schema, replay, or hashes.
4. FOUNDATIONS §11 determinism: the seat-keyed map output must use deterministic (sorted/insertion-ordered by seat ID) key ordering so repeated runs of the same seeds produce byte-identical summaries; no wall-clock or hash-map iteration order enters the output.

## Architecture Check

1. A single seat-keyed map per summary is cleaner and N-extensible than per-game two-scalar blocks: one aggregation shape serves 2-seat and N-seat games, eliminating the duplicated `seat_0_*`/`seat_1_*` pattern.
2. No backwards-compat shim: the scalar counters are replaced by the map; a parity check pins the rendered two-seat output so existing behavior is provably unchanged.
3. `engine-core` untouched; no `game-stdlib` change. The seat-keyed summary lives entirely in `tools/simulate`.

## Verification Layers

1. Two-seat output unchanged -> parity check: a `simulate` run over an existing game yields the same win/loss/`n` totals as before (regression assertion on the rendered summary).
2. Seat-keyed map is deterministic -> repeated runs with identical seeds produce byte-identical key ordering (sorted-by-seat-ID assertion).
3. N>2 representable -> a synthetic 3-seat aggregation unit test populates and renders the map (no official >2-seat game exists pre-Gate-15).
4. Tool builds and runs -> `cargo run -p simulate -- --game race_to_n --games 1000`.

## What to Change

### 1. Replace scalar counters with a seat-keyed summary

Introduce a deterministic seat-keyed summary type (ordered seat IDs; win/loss/draw/tie counts keyed by seat ID; terminal-reason counts; `games_completed`/`games_failed`; failure sample seeds) replacing the per-game `seat_0_*`/`seat_1_*` scalar blocks.

### 2. Stable rendering + parity

Render the summary with deterministic key ordering; add a parity regression so the two-seat rendered totals match the pre-change output.

## Files to Touch

- `tools/simulate/src/main.rs` (modify)

## Out of Scope

- Trace-schema / replay / hash changes — the summary is a tool report, not a serialized contract (spec §3.3).
- Any game declaring >2 seats (Gate 15); N>2 here is covered by a synthetic unit fixture.
- Bridge seat-count acceptance (INFADNSEA-001) — simulate is native and independent.

## Acceptance Criteria

### Tests That Must Pass

1. Parity test: a `simulate` run over a current two-seat game yields the same win/loss/`n` totals as the pre-change scalar summary.
2. Determinism test: identical seeds produce byte-identical seat-keyed summary key ordering.
3. `cargo run -p simulate -- --game race_to_n --games 1000` — completes and emits the seat-keyed summary.

### Invariants

1. Summary key ordering is deterministic (sorted/insertion-ordered by seat ID); no nondeterministic iteration order in the output.
2. Two-seat rendered totals are byte-stable versus the pre-change output.

## Test Plan

### New/Modified Tests

1. `tools/simulate/src/main.rs` (`#[cfg(test)]`) — synthetic 3-seat aggregation + 2-seat parity + determinism tests.

### Commands

1. `cargo test -p simulate`
2. `cargo run -p simulate -- --game race_to_n --games 1000`

## Outcome

Completed: 2026-06-14

- Replaced the default `race_to_n` summary's fixed `seat_0_wins` / `seat_1_wins` output with deterministic `seat_order` and `wins_by_seat` map rendering.
- Updated existing two-seat game summary renderers to emit deterministic seat-keyed win maps, with draw/split/nonterminal maps where those outcomes apply.
- Added helper coverage for stable `BTreeMap` rendering and a synthetic three-seat count map to prove deterministic N-seat key ordering before an official N-seat game exists.
- Deviations: simulator failure handling remains fail-fast and still uses the existing detailed failure block/report; this ticket changed the successful summary result surface without changing replay, hashes, game rules, or failure-report schema.
- Verification: `cargo fmt --all --check`; `cargo test -p simulate`; `cargo run -p simulate -- --game race_to_n --games 1000`.

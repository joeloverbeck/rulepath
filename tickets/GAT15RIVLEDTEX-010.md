# GAT15RIVLEDTEX-010: Replay export/import redaction, golden-trace replay, and serialization determinism

**Status**: PENDING
**Priority**: HIGH
**Effort**: Medium
**Engine Changes**: Yes — `games/river_ledger/src/replay_support.rs`, `tests/replay.rs`, `tests/serialization.rs`, `src/lib.rs`; replay golden trace
**Deps**: GAT15RIVLEDTEX-009

## Problem

River Ledger must support deterministic replay with a viewer-scoped public export: identical seed + command stream reproduces identical traces/checkpoints/hashes, the public replay export omits hidden facts while the internal full trace stays test-authority only, view-hash helpers are stable, and serialization order is deterministic across state, views, effects, explanations, and trace exports.

## Assumption Reassessment (2026-06-14)

1. `games/poker_lite/src/replay_support.rs` + `tests/{replay,serialization}.rs` are the precedent for the viewer-scoped export split; `crates/engine-core/src/replay.rs` `ReplayRecord` (with `seats: Vec<SeatAssignment>`, `SchemaVersion(1)`, `hashes: ReplayHashSet`) is the reused trace contract.
2. `specs/...-base.md` §4.1 (`replay_support.rs`), §6 exit row 2 (replay exports prove no-leak), §7.2 (replay/checkpoint/hash + serialization classes), and §8 (schema v1 reused, no migration) fix `RL-REPLAY-*`.
3. Cross-artifact boundary under audit: replay export consumes the visibility projections + view hashes from 008 and the no-leak guarantees proven in 009; it produces the public export consumed by `replay-check` (registered in 015) and the browser export path (016/018). Trace schema v1 is reused, not migrated.
4. FOUNDATIONS §2 (behavior authority + determinism) motivates this ticket: replay, hashing, and serialization order are Rust-owned and deterministic; no wall-clock or hash-map iteration order enters canonical forms.
5. §11 determinism + no-leak enforcement surface under audit: same `(seed, command stream, version)` → identical trace/checkpoints/hashes; the public replay export omits hole/burn/deck-tail/future-community/private-diagnostic facts; the internal full trace remains test-only. Any change to replay/hash *semantics* would trip §13 (none intended — schema reused).

## Architecture Check

1. A single export-redaction layer over the deterministic engine trace keeps the public export viewer-safe by construction and replay byte-reproducible, matching the sibling replay pattern.
2. No backwards-compatibility aliasing/shims — new module; no trace-schema migration.
3. `engine-core` stays noun-free (§3); reuses the generic replay contract; no `game-stdlib` promotion (§4).

## Verification Layers

1. Same seed + command stream → identical trace/checkpoints/hashes -> `cargo test -p river_ledger --test replay` replay-hash equality.
2. Public replay export omits hidden facts; internal full trace test-only -> export no-leak test (§11).
3. Stable JSON ordering for state/views/effects/explanations/exports -> `cargo test -p river_ledger --test serialization`.
4. Public-replay-export-import golden trace -> `cargo run -p replay-check -- --game river_ledger` after GAT15RIVLEDTEX-015; equality proven here by the replay tests.

## What to Change

### 1. `games/river_ledger/src/replay_support.rs`

Golden-trace fixture helpers, public replay export/import with redaction, and view-hash helpers.

### 2. Replay + serialization tests + trace

Create `tests/replay.rs` (replay/checkpoint/hash equality, export no-leak, import round-trip) and `tests/serialization.rs` (stable ordering, no nondeterministic map order in public payloads); add the `public-replay-export-import` golden trace.

## Files to Touch

- `games/river_ledger/src/replay_support.rs` (new)
- `games/river_ledger/tests/replay.rs` (new)
- `games/river_ledger/tests/serialization.rs` (new)
- `games/river_ledger/src/lib.rs` (modify; created by 003)
- `games/river_ledger/tests/golden_traces/public-replay-export-import.trace.json` (new)

## Out of Scope

- The pairwise visibility no-leak sweep (GAT15RIVLEDTEX-009).
- Tool registration of `replay-check` (GAT15RIVLEDTEX-015) and WASM/browser export (016/018).
- Any trace-schema migration (forbidden without ADR).

## Acceptance Criteria

### Tests That Must Pass

1. `cargo test -p river_ledger --test replay` — replay/hash equality, export no-leak, import round-trip.
2. `cargo test -p river_ledger --test serialization` — stable ordering across all public payloads.
3. `cargo test -p river_ledger` passes overall.

### Invariants

1. Replay/hash/serialization are deterministic and Rust-owned (§2/§11); schema v1 unchanged (§13).
2. The public export carries no hidden fact; the internal full trace is test-only (§11).

## Test Plan

### New/Modified Tests

1. `games/river_ledger/tests/replay.rs` (new) — replay-hash equality + export no-leak + import.
2. `games/river_ledger/tests/serialization.rs` (new) — stable serialization order.
3. `games/river_ledger/tests/golden_traces/public-replay-export-import.trace.json` (new) — export/import evidence.

### Commands

1. `cargo test -p river_ledger --test replay && cargo test -p river_ledger --test serialization`
2. `cargo test -p river_ledger && bash scripts/boundary-check.sh`
3. Crate-scoped replay/serialization tests are the correct boundary; cross-tool replay validation is wired in GAT15RIVLEDTEX-015.

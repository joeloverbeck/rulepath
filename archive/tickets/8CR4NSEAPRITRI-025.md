# 8CR4NSEAPRITRI-025: River Ledger C-08 replay-command profile driver

**Status**: COMPLETED
**Priority**: MEDIUM
**Effort**: Small
**Engine Changes**: Yes (dev-only profile adapter) — `games/river_ledger/tests/replay.rs`; trace bytes/schema unchanged
**Deps**: 8CR4NSEAPRITRI-001

## Problem

River's internal-dev command traces (including selected Gate 15.1 traces) are not yet validated through the shipped `replay-command-v1` evidence-profile driver (MSC-8C, C-08). Add a virtual `replay-command-v1` profile adapter around the existing traces, validating metadata only and leaving legacy trace bytes authoritative (spec §3.9 River replay-command, §5.9).

## Assumption Reassessment (2026-06-24)

1. `crates/game-test-support/src/profiles.rs::ReplayCommandV1Driver` exists; River's command traces (base + all-in goldens) exist under `games/river_ledger/tests/golden_traces/`. Confirmed during `/reassess-spec`.
2. Spec §3.9 classifies River `replay-command-v1` as `migrate`; the driver validates profile metadata then delegates to the game/owning validator — it does not parse commands or set up games.
3. Cross-artifact: the evidence-fixture profile contract is owned by `game-test-support`; the trace bytes/schema stay the existing authority. Baseline trace digests come from `-001`.
4. §11/Evidence-Fixture-Contract motivates this ticket: the driver accepts correct metadata, rejects wrong profile/version/field set, and leaves physical trace bytes unchanged.
5. Enforcement surface = `ReplayCommandV1Driver` metadata validation over selected internal-dev command traces; no trace is rewritten merely to insert profile keys.

## Architecture Check

1. A thin virtual profile adapter is cleaner than re-asserting trace structure inline — it routes evidence through the single owned driver while the game owns replay meaning.
2. No backwards-compatibility shim is introduced; no trace byte changes. Rollback removes only the profile metadata/driver test.
3. `engine-core` stays noun-free (§3); `game-test-support` is dev-only and encodes no behavior (§4/§5).

## Verification Layers

1. Driver accepts correct metadata, rejects wrong profile/version/field set -> schema/serialization validation in the River replay tests.
2. Physical trace bytes unchanged -> golden trace byte check (`replay-check --game river_ledger --all` byte-identical to baseline).
3. Behavior delegated to the game -> codebase grep-proof (driver validates metadata only; no command parsing/setup in the adapter).

## What to Change

### 1. Add the `replay-command-v1` profile adapter

In `games/river_ledger/tests/replay.rs`, add a virtual `ReplayCommandV1Driver` adapter validating profile metadata over selected internal-dev command traces (including selected Gate 15.1 traces), delegating replay behavior to River.

## Files to Touch

- `games/river_ledger/tests/replay.rs` (modify)

## Out of Scope

- The public/seat-private export profiles (`-026`/`-027`) and side-pot domain profile (`-028`).
- Rewriting any trace to insert profile keys; changing any trace byte.
- Any betting/all-in/pot logic.

## Acceptance Criteria

### Tests That Must Pass

1. `cargo test -p river_ledger` is green, including the `replay-command-v1` driver accept/reject metadata test.
2. `cargo run -p replay-check -- --game river_ledger --all` passes with trace bytes unchanged.
3. `bash scripts/boundary-check.sh` passes.

### Invariants

1. The driver validates metadata only and delegates to River; physical trace bytes are unchanged.
2. No procedural metadata is inserted into any trace.

## Test Plan

### New/Modified Tests

1. `games/river_ledger/tests/replay.rs` — `replay-command-v1` driver metadata accept/reject coverage over selected command traces.

### Commands

1. `cargo test -p river_ledger`
2. `cargo run -p replay-check -- --game river_ledger --all`
3. The per-game replay test is the correct boundary: the profile is a dev-only evidence adapter over existing traces.

## Outcome

Completed: 2026-06-24

What changed:

1. Added a virtual `replay-command-v1` profile adapter test in River replay coverage using `ReplayCommandV1Driver`.
2. Validated accept/delegation behavior over an internal command trace and selected Gate 15.1 golden traces while asserting profile metadata is not embedded into those trace files.
3. Added reject coverage for wrong profile id, wrong profile version, unknown field, and illegal canonical-byte claim.

Deviations: None.

Verification:

1. `cargo fmt --all --check` - passed.
2. `cargo test -p river_ledger` - passed.
3. `cargo run -p replay-check -- --game river_ledger --all` - passed.
4. `bash scripts/boundary-check.sh` - passed.

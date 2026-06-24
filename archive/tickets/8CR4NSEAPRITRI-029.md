# 8CR4NSEAPRITRI-029: Briar Circuit C-08 replay-command profile driver

**Status**: COMPLETED
**Priority**: MEDIUM
**Effort**: Small
**Engine Changes**: Yes (dev-only profile adapter) — `games/briar_circuit/tests/replay.rs`; trace bytes/schema unchanged
**Deps**: 8CR4NSEAPRITRI-001

## Problem

Briar's internal command traces (pass/play) are not yet validated through the shipped `replay-command-v1` driver (MSC-8C, C-08). Add a virtual `replay-command-v1` adapter around the existing pass/play traces, validating metadata only and leaving trace bytes authoritative (spec §3.9 Briar replay-command, §5.9).

## Assumption Reassessment (2026-06-24)

1. `crates/game-test-support/src/profiles.rs::ReplayCommandV1Driver` exists; Briar's pass/play golden traces exist under `games/briar_circuit/tests/golden_traces/`. Confirmed during `/reassess-spec`.
2. Spec §3.9 classifies Briar `replay-command-v1` as `migrate`; the driver validates metadata then delegates to the game.
3. Cross-artifact: the evidence-fixture profile contract is owned by `game-test-support`; Briar's trace bytes/schema stay authority. Baseline trace digests come from `-001`.
4. §11/Evidence-Fixture-Contract motivates this ticket: the driver accepts correct metadata, rejects wrong profile/version/field set, and leaves trace bytes unchanged.
5. Enforcement surface = `ReplayCommandV1Driver` metadata validation over selected pass/play command traces; no trace rewrite.

## Architecture Check

1. A thin virtual profile adapter is cleaner than inline trace-structure asserts — it routes evidence through the owned driver while Briar owns replay meaning.
2. No backwards-compatibility shim is introduced; no trace byte changes. Rollback removes only the profile adapter.
3. `engine-core` stays noun-free (§3); `game-test-support` is dev-only (§4/§5).

## Verification Layers

1. Driver accepts correct metadata, rejects wrong profile/version/field set -> schema/serialization validation in Briar replay tests.
2. Physical trace bytes unchanged -> golden trace byte check (`replay-check --game briar_circuit --all` byte-identical to baseline).
3. Behavior delegated to the game -> codebase grep-proof (driver validates metadata only).

## What to Change

### 1. Add the `replay-command-v1` profile adapter

In `games/briar_circuit/tests/replay.rs`, add a `ReplayCommandV1Driver` adapter validating profile metadata over selected pass/play command traces, delegating replay behavior to Briar.

## Files to Touch

- `games/briar_circuit/tests/replay.rs` (modify)

## Out of Scope

- The setup-evidence (`-030`) and public/seat-private export (`-031`/`-032`) profiles; the domain profile is pilot credit.
- Rewriting any trace to insert profile keys; changing any trace byte.
- Any pass/trick/moon policy.

## Acceptance Criteria

### Tests That Must Pass

1. `cargo test -p briar_circuit` is green, including the `replay-command-v1` driver accept/reject metadata test.
2. `cargo run -p replay-check -- --game briar_circuit --all` passes with trace bytes unchanged.
3. `bash scripts/boundary-check.sh` passes.

### Invariants

1. The driver validates metadata only and delegates to Briar; physical trace bytes are unchanged.
2. No procedural metadata is inserted into any trace.

## Test Plan

### New/Modified Tests

1. `games/briar_circuit/tests/replay.rs` — `replay-command-v1` driver metadata accept/reject over selected pass/play traces.

### Commands

1. `cargo test -p briar_circuit`
2. `cargo run -p replay-check -- --game briar_circuit --all`
3. The per-game replay test is the correct boundary: the profile is a dev-only evidence adapter over existing traces.

## Outcome

Completed: 2026-06-24

What changed:

1. Added a virtual `replay-command-v1` profile adapter test for Briar pass/play evidence using `ReplayCommandV1Driver`.
2. Validated selected pass and play golden traces remain free of embedded profile metadata while delegating behavior to Briar replay snapshots.
3. Added reject coverage for wrong profile id, wrong profile version, and unknown profile fields.

Deviations: None.

Verification:

1. `cargo fmt --all --check` - passed.
2. `cargo test -p briar_circuit` - passed.
3. `cargo run -p replay-check -- --game briar_circuit --all` - passed.
4. `bash scripts/boundary-check.sh` - passed.

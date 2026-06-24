# 8CR4NSEAPRITRI-030: Briar Circuit C-08 setup-evidence profile driver

**Status**: PENDING
**Priority**: MEDIUM
**Effort**: Small
**Engine Changes**: Yes (dev-only profile adapter) — `games/briar_circuit/tests/`; fixture bytes + deal policy unchanged
**Deps**: 8CR4NSEAPRITRI-001, 8CR4NSEAPRITRI-011

## Problem

Briar's fixed-four deterministic setup metadata is not yet validated through the shipped `setup-evidence-v1` driver (MSC-8C, C-08). Add a `setup-evidence-v1` adapter over `briar_circuit_standard.fixture.json`, validating deterministic setup metadata through the Briar setup owner while keeping deal/shuffle policy as code (spec §3.9 Briar setup, §5.9).

## Assumption Reassessment (2026-06-24)

1. `crates/game-test-support/src/profiles.rs::SetupEvidenceV1Driver` exists; `games/briar_circuit/data/fixtures/briar_circuit_standard.fixture.json` exists. Confirmed during `/reassess-spec`.
2. Spec §3.9 classifies Briar `setup-evidence-v1` as `migrate`; this ticket `Deps` `-011` so the structural exact-four validation is in place when the profile enumerates the four-seat setup.
3. Cross-artifact: the setup-evidence contract is owned by `game-test-support`; Briar owns deal/shuffle policy. Baseline fixture bytes come from `-001`.
4. §5/§11 motivate this ticket: setup metadata is typed evidence only — the driver encodes no deal formula and delegates deterministic-deal validation to Briar.
5. Enforcement surface = `SetupEvidenceV1Driver` metadata validation over the fixed-four fixture; no fixture byte changes and deal/shuffle stays code.

## Architecture Check

1. A thin setup-evidence adapter delegating to the Briar setup owner is cleaner than re-deriving deal structure in the driver.
2. No backwards-compatibility shim is introduced; no fixture byte changes. Rollback removes only the adapter.
3. `engine-core` stays noun-free (§3); `game-test-support` is dev-only and encodes no behavior (§4/§5).

## Verification Layers

1. Driver accepts correct setup metadata, rejects wrong profile/version/field set -> schema/serialization validation via `SetupEvidenceV1Driver`.
2. Deterministic deal validated through the game owner -> setup/serialization test (game-owned).
3. Fixture bytes + deal policy unchanged -> `fixture-check --game briar_circuit` byte-identical + codebase grep-proof (deal/shuffle stays code).

## What to Change

### 1. Add the `setup-evidence-v1` profile adapter

In `games/briar_circuit/tests/` (setup/serialization module), add a `SetupEvidenceV1Driver` adapter over `briar_circuit_standard.fixture.json` validating fixed-four deterministic setup metadata, delegating deal/shuffle validation to Briar.

## Files to Touch

- `games/briar_circuit/tests/serialization.rs` (modify; or the setup test module)

## Out of Scope

- The replay-command (`-029`) and public/seat-private export (`-031`/`-032`) profiles; the domain profile is pilot credit.
- Encoding any deal formula in fixture/profile metadata; rewriting the fixture.
- Any deal/shuffle/pass policy change.

## Acceptance Criteria

### Tests That Must Pass

1. `cargo test -p briar_circuit` is green, including the `setup-evidence-v1` driver + game-owned deterministic-deal assertion.
2. `cargo run -p fixture-check -- --game briar_circuit` passes with fixture bytes unchanged.
3. `bash scripts/boundary-check.sh` passes.

### Invariants

1. The driver validates metadata only; deal/shuffle stays code; fixture bytes are unchanged.
2. No procedural metadata is inserted into the fixture.

## Test Plan

### New/Modified Tests

1. `games/briar_circuit/tests/serialization.rs` — `setup-evidence-v1` driver metadata + game-owned deterministic-deal assertion.

### Commands

1. `cargo test -p briar_circuit`
2. `cargo run -p fixture-check -- --game briar_circuit`
3. The per-game fixture/setup test is the correct boundary: setup evidence delegates to the game setup owner.

# UNI8CMECSCA-023: Race to N drives `replay-command-v1`

**Status**: PENDING
**Priority**: MEDIUM
**Effort**: Medium
**Engine Changes**: Yes — `games/race_to_n/tests/replay_tests.rs`, `games/race_to_n/Cargo.toml` (`[dev-dependencies]`)
**Deps**: UNI8CMECSCA-022, UNI8CMECSCA-014

## Problem

Prove the `ReplayCommandV1Driver` (UNI8CMECSCA-022) against a real game: Race to N drives the `replay-command-v1` profile using its existing replay support and the `shortest-normal` fixture. Commands, checkpoints, and hashes replay through game-owned functions; the legacy fixture stays readable; any metadata insertion is a named profile migration, not a silent edit.

## Assumption Reassessment (2026-06-22)

1. `games/race_to_n/tests/replay_tests.rs` exists and exercises command replay against `tests/golden_traces/shortest-normal.trace.json`; the `ReplayCommandV1Driver` exists after UNI8CMECSCA-022, and the flat-tree v1 surface is established by UNI8CMECSCA-014.
2. Spec §5 8C-023 fixes the boundary: commands/checkpoints/hashes replay through game-owned functions; legacy fixture stays readable; any metadata insertion is a named profile migration. The `replay-command-v1` profile is defined in `docs/EVIDENCE-FIXTURE-CONTRACT.md`.
3. Cross-artifact boundary under audit: Race's replay support + `shortest-normal` fixture and the profile driver. The driver owns profile/version validation and the deterministic command loop; setup/command-application/legality/hashing stay in Race.
4. FOUNDATIONS §2/§11: behavior stays in Race; the driver only sequences and validates. Determinism (EC-20): commands replay byte-identically through the game's own functions.
5. Deterministic replay/hash surface under audit (§11/EC-11): the legacy `shortest-normal` fixture remains readable and its hashes unchanged; any profile-metadata insertion follows the named ADR-0009 migration (before/after, version, validators, rollback), never a silent rewrite.

## Architecture Check

1. Driving the profile from Race's existing replay support proves the driver orchestrates without absorbing command/legality/hash logic.
2. No backwards-compatibility shim — the driver is adopted; the legacy fixture is read, not rewritten.
3. `engine-core`/`game-stdlib` untouched; `game-test-support` enters only as a dev-dependency.

## Verification Layers

1. Commands/checkpoints/hashes replay through game-owned functions → `cargo test -p race_to_n` (driver-based replay test).
2. Legacy `shortest-normal` fixture readable, hashes unchanged → `cargo run -p replay-check -- --game race_to_n --all`.
3. Any metadata insertion is a named migration → migration-receipt assert (if applicable) or grep-proof the fixture is unchanged.
4. `game-test-support` dev-only edge → `cargo tree --workspace -e normal --invert game-test-support`.

## What to Change

### 1. `games/race_to_n/Cargo.toml`

Add `game-test-support` under `[dev-dependencies]`.

### 2. `games/race_to_n/tests/replay_tests.rs`

Adopt `ReplayCommandV1Driver` to drive the `shortest-normal` fixture through Race's existing replay/checkpoint functions; assert command/checkpoint/hash equality.

## Files to Touch

- `games/race_to_n/Cargo.toml` (modify)
- `games/race_to_n/tests/replay_tests.rs` (modify)

## Out of Scope

- Setup/export/domain profiles (River 024, Vow 025, Briar 026) or tool dispatch (027).
- Moving command parsing/application/legality/hashing into the driver.
- Silently inserting profile metadata into the fixture.

## Acceptance Criteria

### Tests That Must Pass

1. `cargo test -p race_to_n` passes the driver-based replay against `shortest-normal`.
2. `cargo run -p replay-check -- --game race_to_n --all` passes with unchanged hashes.
3. `cargo test --workspace` passes.

### Invariants

1. Commands/checkpoints/hashes replay through game-owned functions; the driver owns only orchestration/validation.
2. The legacy fixture stays readable; any metadata change is a named migration.

## Test Plan

### New/Modified Tests

1. `games/race_to_n/tests/replay_tests.rs` — `replay-command-v1` driver replay over the existing fixture.

### Commands

1. `cargo test -p race_to_n`
2. `cargo run -p replay-check -- --game race_to_n --all`
3. The game suite plus `replay-check` are the correct boundary — the driver must replay through real game functions without perturbing the fixture.

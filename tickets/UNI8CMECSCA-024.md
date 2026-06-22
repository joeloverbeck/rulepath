# UNI8CMECSCA-024: River Ledger drives `setup-evidence-v1`

**Status**: PENDING
**Priority**: MEDIUM
**Effort**: Medium
**Engine Changes**: Yes — `games/river_ledger/tests/replay.rs` (`[dev-dependencies]` already added by UNI8CMECSCA-021)
**Deps**: UNI8CMECSCA-022, UNI8CMECSCA-021, UNI8CMECSCA-017

## Problem

Prove the `SetupEvidenceV1Driver` against River Ledger using the standard 3-seat setup fixture. Setup evidence remains non-command input; the canonical-byte authority stays `none` unless explicitly defined; and River's setup semantics and 3–6 behavior remain game-owned. Any metadata edit to the fixture is characterized separately from command replay or export surfaces — never bundled.

## Assumption Reassessment (2026-06-22)

1. `games/river_ledger/data/fixtures/river_ledger_3p_standard.fixture.json` and `data/manifest.toml` exist; `games/river_ledger/tests/replay.rs` carries replay/setup tests; `game-test-support` is already a River dev-dependency after UNI8CMECSCA-021; `SetupEvidenceV1Driver` exists after UNI8CMECSCA-022. River's deal/setup is byte-stable after UNI8CMECSCA-017.
2. Spec §4.5 + §5 8C-024 fix the boundary: characterize any metadata edit separately from command replay or export; canonical-byte authority stays `none` unless explicitly defined; River setup semantics/3–6 behavior remain game-owned. The `setup-evidence-v1` profile is defined in `docs/EVIDENCE-FIXTURE-CONTRACT.md`.
3. Cross-artifact boundary under audit: River's setup fixture/manifest and the driver. The driver owns manifest/profile/version/seat-grammar shape and deterministic invocation comparison; setup legality, options meaning, and variant semantics stay in River.
4. FOUNDATIONS §5/§2: the fixture is typed setup evidence (non-command input), not behavior; River decides setup legality. The driver invents no stable-byte claim when authority is `none`.
5. Determinism/no-leak substrate (§11/EC-20/EC-22): setup evidence is input/expected only; no selectors/triggers enter it; any metadata insertion follows a named ADR-0009 migration (separate from command/export surfaces), with the 3-seat fixture otherwise readable and unchanged.

## Architecture Check

1. Driving setup evidence through the profile driver proves the driver validates manifest/profile shape without owning setup legality.
2. No backwards-compatibility shim — the driver is adopted; the fixture is read, and any metadata change is an explicit named migration.
3. `engine-core`/`game-stdlib` untouched; River setup semantics stay local.

## Verification Layers

1. Setup evidence validates as non-command input through the driver → `cargo test -p river_ledger` (setup-evidence test).
2. River 3–6 setup behavior/semantics unchanged → `cargo run -p fixture-check -- --game river_ledger`, `cargo run -p replay-check -- --game river_ledger --all`.
3. Canonical-byte authority `none` invents no hash → driver assertion / grep-proof.
4. Any metadata edit is a separate named migration → migration-receipt assert (if applicable) or grep-proof the fixture is unchanged.

## What to Change

### 1. `games/river_ledger/tests/replay.rs`

Adopt `SetupEvidenceV1Driver` to validate the 3-seat setup fixture's manifest/profile/version/seat-grammar shape and compare deterministic invocation, delegating setup legality/options/variant semantics to River.

## Files to Touch

- `games/river_ledger/tests/replay.rs` (modify)

## Out of Scope

- Command replay or export profiles (Race 023, Vow 025) or domain evidence (Briar 026).
- Moving River setup legality/options/variant semantics into the driver.
- Inventing a canonical-byte hash when authority is `none`; silently editing the fixture.

## Acceptance Criteria

### Tests That Must Pass

1. `cargo test -p river_ledger` passes the setup-evidence driver validation.
2. `cargo run -p fixture-check -- --game river_ledger` and `cargo run -p replay-check -- --game river_ledger --all` pass.
3. `cargo test --workspace` passes.

### Invariants

1. Setup evidence is non-command input; setup legality/options/variant semantics stay in River.
2. Canonical-byte authority `none` invents no hash; any metadata edit is a separate named migration.

## Test Plan

### New/Modified Tests

1. `games/river_ledger/tests/replay.rs` — `setup-evidence-v1` driver validation over the 3-seat fixture.

### Commands

1. `cargo test -p river_ledger`
2. `cargo run -p fixture-check -- --game river_ledger`
3. The game suite plus `fixture-check` are the correct boundary — setup evidence is validated as input, not executed.

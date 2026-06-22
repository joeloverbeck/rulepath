# UNI8CMECSCA-026: Briar Circuit drives `domain-evidence-v1` (fixture-only)

**Status**: COMPLETED
**Priority**: MEDIUM
**Effort**: Medium
**Engine Changes**: Yes — `games/briar_circuit/tests/replay.rs`, `games/briar_circuit/Cargo.toml` (`[dev-dependencies]`)
**Deps**: UNI8CMECSCA-022

## Problem

Prove the `DomainEvidenceV1Driver` against a real scoring fixture: Briar Circuit drives `domain-evidence-v1` for `briar_circuit_moon.fixture.json`, and uses `briar_circuit_first_trick_exception.fixture.json` as a negative boundary check that the driver validates evidence shape but does **not** execute scoring/legality from data. The domain fixture declares visibility, validator owner, rules/data/domain version, canonical-byte authority (`none` unless explicitly defined), and migration note. Scoring and first-trick legality remain Rust code. This is a fixture-only pilot, not a full Briar retrofit.

## Outcome

Completed: 2026-06-22

Implemented the fixture-only Briar `domain-evidence-v1` pilot in
`games/briar_circuit/tests/replay.rs`. The tests wrap the legacy moon and
first-trick-exception fixtures in virtual `DomainEvidenceV1Driver` metadata,
assert the fixtures do not carry embedded profile metadata, and delegate the
actual moon scoring and first-trick all-points legality checks to Briar Rust
code. `game-test-support` is a Briar dev-dependency only. The Briar source
modules and the two fixture JSON files were not changed.

Verification:

1. `cargo fmt --all --check`
2. `cargo test -p briar_circuit`
3. `cargo run -p fixture-check -- --game briar_circuit`
4. `cargo tree --workspace -e normal --invert game-test-support`
5. `cargo tree --workspace -e normal,build --invert game-test-support`
6. `git diff -- games/briar_circuit/src games/briar_circuit/data/fixtures/briar_circuit_moon.fixture.json games/briar_circuit/data/fixtures/briar_circuit_first_trick_exception.fixture.json`
7. `bash scripts/boundary-check.sh`
8. `cargo test --workspace`

## Assumption Reassessment (2026-06-22)

1. `games/briar_circuit/data/fixtures/{briar_circuit_moon,briar_circuit_first_trick_exception}.fixture.json` and `data/manifest.toml` exist; `games/briar_circuit/tests/replay.rs` carries replay tests. `DomainEvidenceV1Driver` exists after UNI8CMECSCA-022. `game-test-support` is not yet a Briar dev-dependency.
2. Spec §4.5 + §5 8C-026 + A-08 fix the boundary: `briar_circuit_moon` is the real `domain-evidence-v1` pilot (explicit scoring evidence); `briar_circuit_first_trick_exception` is a negative policy boundary; neither promotes scoring/legality; this does not retrofit Briar replay/visibility suites. The `domain-evidence-v1` profile is defined in `docs/EVIDENCE-FIXTURE-CONTRACT.md`.
3. Cross-artifact boundary under audit: Briar's domain fixtures and the driver. The driver owns domain profile/version/visibility/validator declarations and typed input handoff; evaluator/allocator/topology/scoring/legality/no-leak policy stay in Briar.
4. FOUNDATIONS §5/§2: the domain fixture is typed evidence, not behavior; scoring and first-trick legality stay in Rust. The driver executes no rule from fixture keys (the negative fixture proves this).
5. Determinism/no-leak substrate (§11/EC-22): the driver treats artifacts as input/expected evidence only — no selectors/triggers/formulas/procedures execute; `canonical_byte_authority = none` invents no hash; the moon fixture's declared visibility is respected.

## Architecture Check

1. A real scoring fixture plus a negative boundary fixture proves the driver validates domain-evidence shape without becoming a data-driven rules engine.
2. No backwards-compatibility shim — the driver is adopted; fixtures are read, not rewritten.
3. `engine-core`/`game-stdlib` untouched; Briar scoring/trick lifecycle stays local; `game-test-support` enters only as a dev-dependency.

## Verification Layers

1. `briar_circuit_moon` validates as domain evidence with declared visibility/validator/version/byte-authority/migration-note → `cargo test -p briar_circuit` (domain-evidence driver test).
2. `briar_circuit_first_trick_exception` proves the driver validates shape but executes no scoring/legality from data → negative boundary test.
3. Briar scoring + first-trick legality remain Rust → grep-proof those modules are untouched + `cargo run -p fixture-check -- --game briar_circuit`.
4. `game-test-support` dev-only edge → `cargo tree --workspace -e normal --invert game-test-support`.

## What to Change

### 1. `games/briar_circuit/Cargo.toml`

Add `game-test-support` under `[dev-dependencies]`.

### 2. `games/briar_circuit/tests/replay.rs`

Adopt `DomainEvidenceV1Driver` for the moon fixture (declaring visibility, validator owner, rules/data/domain version, canonical-byte authority, migration note) and the first-trick-exception fixture as a negative boundary; delegate evaluator/scoring/legality to Briar.

## Files to Touch

- `games/briar_circuit/Cargo.toml` (modify)
- `games/briar_circuit/tests/replay.rs` (modify)

## Out of Scope

- Retrofitting Briar replay/visibility suites or promoting scoring/trick lifecycle (a full Briar C-11 retrofit, not 8C).
- Executing any scoring/legality from fixture data.
- Inventing a canonical-byte hash when authority is `none`.

## Acceptance Criteria

### Tests That Must Pass

1. `cargo test -p briar_circuit` passes the domain-evidence driver test and the negative boundary test.
2. `cargo run -p fixture-check -- --game briar_circuit` passes.
3. `cargo test --workspace` passes.

### Invariants

1. Scoring and first-trick legality remain Rust code; the driver executes no rule from fixture data.
2. `canonical_byte_authority = none` invents no hash; `game-test-support` is dev-only.

## Test Plan

### New/Modified Tests

1. `games/briar_circuit/tests/replay.rs` — `domain-evidence-v1` driver over the moon fixture + negative first-trick-exception boundary.

### Commands

1. `cargo test -p briar_circuit`
2. `cargo run -p fixture-check -- --game briar_circuit`
3. The game suite plus `fixture-check` are the correct boundary — domain evidence is validated as input, never executed.

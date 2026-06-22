# UNI8CMECSCA-003: Characterize every pilot byte/hash/visibility/RNG surface

**Status**: PENDING
**Priority**: HIGH
**Effort**: Large
**Engine Changes**: Yes (deterministic evidence + docs) — `reports/8c-mechanical-scaffolding-characterization.md` plus characterization tests under the pilot game crates
**Deps**: UNI8CMECSCA-002

## Problem

Every byte/hash/visibility/RNG migration in 8C is characterize-first: the current inputs, bytes, hashes, fixture profiles, seat spellings, and RNG outputs/draw-counts must be pinned **before** any helper changes them, so a later parallel-surface or intentional migration can prove "unchanged vs. parallel-new vs. intentional" against a committed baseline. This ticket produces the characterization packet `reports/8c-mechanical-scaffolding-characterization.md` and the deterministic tests that derive its values through the real Rust SUT. It changes no production code and no fixture.

## Assumption Reassessment (2026-06-22)

1. Pilot surfaces exist at the stated paths: Race flat action tree (`games/race_to_n/src/replay_support.rs`, `tests/golden_traces/shortest-normal.trace.json`), Draughts compound tree (`games/draughts_lite/src/replay_support.rs`, `tests/golden_traces/multi-jump.trace.json`), River setup/public/seat-private evidence (`games/river_ledger/src/setup.rs`, `data/fixtures/river_ledger_3p_standard.fixture.json`, `tests/golden_traces/*`), Vow export artifacts (`games/vow_tide/tests/golden_traces/*`), High Card public/seat-private artifacts (`games/high_card_duel/tests/golden_traces/*`), Briar domain fixtures (`games/briar_circuit/data/fixtures/*`), and River RNG (`next_bounded_index_unbiased` in `games/river_ledger/src/setup.rs:159`).
2. The packet schema is fixed by spec §5 Wave-0 "Characterization packet schema" (surface_id, owning path/symbol, profile+version, visibility class, versions, canonical-byte authority, exact input vector, legacy bytes hex, legacy hashes, seat spellings + alias route, legacy RNG output + `next_u64` call count, proposed new surface/version, expected classification, migration note, compatibility window, rollback boundary, validator commands).
3. Cross-artifact boundary under audit: the existing hash/serialization surfaces (`HashValue::from_stable_bytes`, `StableSerialize` in `crates/engine-core/src/replay.rs`) and the ADR-0009 fixture profiles (`docs/EVIDENCE-FIXTURE-CONTRACT.md`). The packet records authority, it does not become a second source of behavior — every value is derived/compared through the real SUT.
4. FOUNDATIONS §11 determinism + EC-10/EC-11: the baseline must be byte-exact and re-derivable. No wall-clock or nondeterministic input enters any recorded value.
5. Deterministic replay/hash & RNG enforcement surfaces under audit: golden-trace/`replay-check` hashes and `DeterministicRng` consumption. The characterization tests must pin both returned values and `next_u64` consumption without altering any existing expected value, and must not leak any seat-private fact into the (committed) report (record canary *absence*, never canary values).

## Architecture Check

1. A committed characterization baseline derived through the SUT is the only way later migration tickets can classify a surface honestly; eyeballing or hand-calculating bytes is forbidden by §7.3.
2. No backwards-compatibility shim — the packet adds tests and a report; it mutates no production code or golden.
3. `engine-core`/`game-stdlib` untouched; no mechanic noun introduced; the report lives under `reports/`.

## Verification Layers

1. Existing pilot tests still pass unchanged → `cargo test -p race_to_n -p draughts_lite -p river_ledger -p high_card_duel -p vow_tide -p briar_circuit`.
2. Recorded bytes/hashes are SUT-derived and deterministic across runs → the new characterization tests (assert pinned values).
3. RNG vectors pin returned index **and** `next_u64` draw count → River RNG characterization test.
4. No seat-private canary appears in the committed report → grep-proof on `reports/8c-mechanical-scaffolding-characterization.md`.
5. Existing replay/fixture validators stay green → `cargo run -p replay-check -- --game river_ledger --all`, `cargo run -p fixture-check -- --game river_ledger`.

## What to Change

### 1. `reports/8c-mechanical-scaffolding-characterization.md`

Author one packet record per pilot surface using the §5 schema. Expected classification for each is `unchanged` until a later ticket proposes a parallel-new-surface or intentional-migration.

### 2. Characterization tests (per pilot crate)

Add deterministic tests that emit/compare the recorded bytes, hashes, seat spellings, and RNG output/consumption through the real Rust SUT, so the report's values are test-derived, not transcribed.

## Files to Touch

- `reports/8c-mechanical-scaffolding-characterization.md` (new)
- `games/race_to_n/tests/serialization_tests.rs` (modify — add flat-tree byte/hash characterization)
- `games/draughts_lite/tests/golden_traces/multi-jump.trace.json` (modify — characterization only if a companion test references it; otherwise add a test file)
- `games/river_ledger/tests/replay.rs` (modify — add setup/public/seat-private + RNG vectors)
- `games/high_card_duel/tests/replay.rs` (modify — public/seat-private artifact characterization)
- `games/vow_tide/tests/replay.rs` (modify — export artifact characterization)
- `games/briar_circuit/tests/replay.rs` (modify — domain fixture characterization)

## Out of Scope

- Any production-code, fixture-byte, or golden-trace change (later migration tickets only).
- Implementing the new writer/encoder/sampler (UNI8CMECSCA-012/013/016).
- Declaring any legacy hash "wrong" or regenerating a golden.

## Acceptance Criteria

### Tests That Must Pass

1. `cargo test -p river_ledger -p race_to_n -p draughts_lite -p high_card_duel -p vow_tide -p briar_circuit` passes with all pre-existing assertions unchanged.
2. `cargo run -p replay-check -- --game river_ledger --all` and `cargo run -p fixture-check -- --game river_ledger` pass.
3. `test -f reports/8c-mechanical-scaffolding-characterization.md` and it has one record per named pilot surface.

### Invariants

1. Every recorded byte/hash/RNG value is produced by a test or validator, not transcribed.
2. No seat-private canary or hidden value appears in the committed report.

## Test Plan

### New/Modified Tests

1. `games/river_ledger/tests/replay.rs` — pins River setup/public/seat-private bytes + `next_bounded_index_unbiased` output and `next_u64` draw count for selected seeds.
2. `games/race_to_n/tests/serialization_tests.rs` — pins the Race flat action-tree legacy bytes/hash.
3. `games/draughts_lite/tests/replay.rs` (or a new test module) — pins the Draughts compound-tree legacy bytes/hash.

### Commands

1. `cargo test -p river_ledger characterization`
2. `cargo test --workspace`
3. The pilot-crate test suites plus `replay-check`/`fixture-check` are the correct boundary because the packet must round-trip through the real SUT and validators, not a standalone script.

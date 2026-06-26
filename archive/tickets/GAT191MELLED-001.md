# GAT191MELLED-001: MatchState base-seed + round-index fields

**Status**: COMPLETED
**Priority**: HIGH
**Effort**: Medium
**Engine Changes**: Yes — `games/meldfall_ledger` (`src/state.rs`); test `tests/serialization.rs`
**Deps**: None

## Problem

Meldfall Ledger's `MatchState` carries no match-level identity for the multi-round
loop: it holds only `{variant, seats, cumulative_scores, dealer_index, round,
terminal}` and drops the deal seed at construction. The deferred `ML-MATCH-006`
transition (GAT191MELLED-002) needs (a) the base match seed to derive each
subsequent round's deal deterministically and (b) a rounds-settled counter so
`round_score` reports the true round index (GAT191MELLED-003). This ticket adds
those two match-level fields and threads them through construction and the
canonical state summary — the foundation every later ticket builds on.

## Assumption Reassessment (2026-06-26)

1. `MatchState` today holds exactly `{variant, seats, cumulative_scores,
   dealer_index, round, terminal}` (`games/meldfall_ledger/src/state.rs:28-35`),
   and `MatchState::from_initial_setup` (state.rs:38-52) consumes `InitialSetup`
   but never copies its `seed`. No base seed or round counter exists — matching
   spec §3.1.2.
2. `InitialSetup` already retains `seed: Seed` (`games/meldfall_ledger/src/setup.rs:37`),
   populated by `setup_match` (setup.rs:66-89). The base seed is therefore already
   available to thread; only `MatchState` retention is missing. `Seed` is the
   existing `engine_core::Seed(u64)` primitive (`crates/engine-core/src/lib.rs:104`).
3. Cross-artifact boundary under audit: `stable_internal_summary`
   (state.rs:54-81) is the canonical match-state serialization string consumed by
   `games/meldfall_ledger/tests/serialization.rs:75-77`, which asserts
   `summary.starts_with("match|variant=...|dealer=0|round=round|...")`.
4. Schema/contract extension: the extended structure is `MatchState` and its
   `stable_internal_summary` projection. Adding the round index shifts the
   canonical string for **all** states including round 0, so the single consumer
   (`serialization.rs:75-77`) is updated in this same diff — an ADR-0009-governed
   Meldfall artifact movement, not test-weakening (spec §3.2 / §4 Engine-state
   row). The golden traces under `tests/golden_traces/` are declarative JSON and
   do not embed the summary, so they are unaffected.

## Architecture Check

1. Storing the base seed + rounds-settled counter on `MatchState` (the match-level
   ledger) is cleaner than re-deriving them from history or threading the seed
   through every apply call — they are match-scoped facts that both the transition
   (002) and `round_score_index` (003) read.
2. No backwards-compatibility shim: the new fields are required and set at
   construction from `InitialSetup`; no `Option`-defaulting or alias path.
3. `engine-core` is untouched (`Seed` is an existing kernel primitive); no mechanic
   noun enters the kernel (§3); no `game-stdlib` change (§4).

## Verification Layers

1. `MatchState` retains the base seed + round index -> codebase grep-proof (fields
   present on the struct) + unit assertion in the serialization test.
2. `stable_internal_summary` stays deterministic with the extended format -> the
   `serialization.rs` `starts_with` assertion (updated) plus same-state→same-string.
3. Existing declarative golden traces remain valid -> `replay-check --game
   meldfall_ledger --all` green (the summary is not embedded in those traces).

## What to Change

### 1. Add match-level fields to `MatchState`

Add the base match seed and a rounds-settled (round-index) counter to the struct
in `state.rs`. Initialize both in `from_initial_setup`: the seed from
`InitialSetup.seed`, the counter to `0` for a fresh match.

### 2. Include the round index in `stable_internal_summary`

Extend the canonical string to carry the round index at a documented position
(e.g. immediately after `dealer={}`). Keep the field ordering stable and
deterministic.

### 3. Update the serialization assertion

Update `tests/serialization.rs:75-77` to the round-index-extended prefix, and add
an assertion that a fresh match renders the round index as 0.

## Files to Touch

- `games/meldfall_ledger/src/state.rs` (modify)
- `games/meldfall_ledger/tests/serialization.rs` (modify)

## Out of Scope

- The transition operation `advance_to_next_round` and per-round deal-seed
  derivation — GAT191MELLED-002.
- The `round_score_index` correction and the `NextRoundDealt` effect — GAT191MELLED-003.
- Any apply-path wiring that increments the counter at settlement — GAT191MELLED-004
  (this ticket only adds the field and its construction-time value).

## Acceptance Criteria

### Tests That Must Pass

1. `cargo test -p meldfall_ledger serialization` — the updated summary assertion passes.
2. `cargo test -p meldfall_ledger` — full crate green.
3. `cargo run -p replay-check -- --game meldfall_ledger --all` — existing declarative traces still pass.

### Invariants

1. `MatchState` carries the base match seed and a rounds-settled counter, set
   deterministically at construction (no wall-clock / no nondeterministic input).
2. `stable_internal_summary` is deterministic and includes the round index;
   identical states produce identical strings.

## Test Plan

### New/Modified Tests

1. `games/meldfall_ledger/tests/serialization.rs` — update the `starts_with`
   assertion to the round-index-extended format and assert the round index is 0
   for a fresh match.

### Commands

1. `cargo test -p meldfall_ledger serialization`
2. `cargo test --workspace`
3. `cargo run -p replay-check -- --game meldfall_ledger --all` — confirms the
   summary-format change leaves the declarative traces valid (narrow trace-validity
   boundary; the broad regression guard is `cargo test --workspace`).

## Outcome

Completed: 2026-06-26

Implemented the match-level base seed and `rounds_settled` fields on
`MatchState`, initialized them from `InitialSetup`, and extended
`stable_internal_summary` with a deterministic `round_index` field. Updated the
serialization integration test to assert the new round-index field for fresh
matches.

Deviations: the literal `cargo test -p meldfall_ledger serialization` command
matched no test names, so the acceptance proof used the actual integration target
`cargo test -p meldfall_ledger --test serialization` in addition to the crate and
workspace suites.

Verification:

- `cargo test -p meldfall_ledger serialization` (completed but ran zero tests due
  to the filter shape)
- `cargo test -p meldfall_ledger --test serialization`
- `cargo test -p meldfall_ledger`
- `cargo run -p replay-check -- --game meldfall_ledger --all`
- `cargo fmt --all --check`
- `cargo clippy -p meldfall_ledger --all-targets -- -D warnings`
- `cargo test --workspace`

# RIVLEDSHOSEA-004: Reconcile golden traces, replay, serialization, and rule coverage

**Status**: COMPLETED
**Priority**: HIGH
**Effort**: Medium
**Engine Changes**: Yes (deterministic evidence + docs) — `games/river_ledger/tests/golden_traces/`, `games/river_ledger/tests/replay.rs`, `games/river_ledger/tests/serialization.rs`, `games/river_ledger/docs/RULE-COVERAGE.md`
**Deps**: RIVLEDSHOSEA-003

## Problem

RIVLEDSHOSEA-001..003 intentionally change serialized winner/allocation order (canonical, not button order) and public seat labels (one-based) in `TerminalOutcome::Showdown` and the V2 banner. Existing golden traces and `replay-check` therefore need controlled regeneration, and two new locked-regression fixtures must capture the seed-`10018` contradiction fix and the seed-`31` split-order fix. This ticket reconciles all deterministic evidence and maps the new tests/traces to rule IDs — without changing trace schema/version (spec §7.4 / §16.11).

## Assumption Reassessment (2026-06-18)

1. `games/river_ledger/tests/golden_traces/` exists and contains `high-card-showdown`, `pair-beats-high-card`, `split-pot-even`, `split-pot-remainder-button-order`, and `setup-3p`..`setup-6p` traces (confirmed by directory listing). The two new fixtures (`showdown-seat-label-consistency.trace.json`, `split-winner-order-vs-remainder.trace.json`) are new siblings under that existing dir.
2. `games/river_ledger/docs/RULE-COVERAGE.md` and `docs/RULES.md` carry the rule IDs `RL-SCORE-SHOWDOWN`, `RL-SCORE-SPLIT`, `RL-POT-REMAINDER-001`, `RL-SHOW-WINNER-001`, `RL-SHOW-SPLIT-001`, `RL-VIS-SHOWDOWN-001`, `RL-REPLAY-SERIAL-001`, `RL-UI-SHOWDOWN-001` verbatim (confirmed). Coverage rows map to tests/traces, not to changed rule text.
3. Shared boundary under audit: the golden-trace serialized payload contract and the replay/serialization hash surface (`docs/TESTING-REPLAY-BENCHMARKING.md`). Only field *values/order* change (winners, allocations, labels); the trace schema and version do not.
4. FOUNDATIONS §11 (deterministic replay/hash/serialization or explicitly migrated) and §13 (changing replay/hash *semantics* requires an ADR): this ticket performs an explicit, reviewed migration of trace *values*, not a schema/semantics change, so no ADR is triggered. Restated before trusting the spec.
5. Determinism + no-leak surface: every regenerated hash is reviewed individually (no bulk-accept); the new/updated traces may reveal only showdown-eligible cards authorized by `RL-VIS-SHOWDOWN-001`; folded/non-revealed private cards stay absent from every payload. The seed-`10018`/seed-`31` no-leak hashes (observer + each seat-private view) are re-run.
6. Golden-trace contract is extended additively (two new fixtures) plus in-place value migration of four existing fixtures; consumers are `replay-check`/`fixture-check` and the native replay/serialization tests, all re-run here. No new schema field.

## Architecture Check

1. Consolidating trace regeneration into one ticket lets every hash be reviewed against an intended cause, satisfying the spec's "no unexplained churn" rule far better than scattering regeneration across the logic tickets.
2. No shim: traces are regenerated through normal trace tooling, not hand-patched; no compatibility shim for the old order is retained.
3. `engine-core`'s replay recorder is reused unchanged; no kernel or `game-stdlib` change.

## Verification Layers

1. Intentional-only trace churn -> per-file hash review: each changed trace under `golden_traces/` is justified by a winner-order or label migration; unrelated churn is rejected.
2. New regression fixtures deterministic -> `cargo run -p replay-check -- --game river_ledger --all` and `cargo run -p fixture-check -- --game river_ledger` pass for both new fixtures.
3. No schema/version change -> grep-proof that the trace `version`/schema field is unchanged across regenerated files.
4. Rule coverage maps the new evidence -> `cargo run -p rule-coverage -- --game river_ledger` green with `RULE-COVERAGE.md` rows updated for the affected `RL-SCORE-*` / `RL-POT-REMAINDER-001` / `RL-UI-SHOWDOWN-001` / `RL-VIS-*` IDs.

## What to Change

### 1. New golden traces

Add `games/river_ledger/tests/golden_traces/showdown-seat-label-consistency.trace.json` (seed `10018`, 4 seats) capturing the unique-winner label coherence, and `split-winner-order-vs-remainder.trace.json` (seed `31`, button `seat_2`) capturing canonical-vs-remainder order. Generate via the standard simulate/trace tooling.

### 2. Regenerate affected existing traces

Regenerate `high-card-showdown`, `pair-beats-high-card`, `split-pot-even`, and `split-pot-remainder-button-order` only where the canonical label/order migration intentionally changes serialized output; review each hash.

### 3. Rule coverage

Update `RULE-COVERAGE.md` rows mapping the new tests/traces to `RL-SCORE-SHOWDOWN`, `RL-SCORE-SPLIT`, `RL-POT-REMAINDER-001`, `RL-SHOW-WINNER-001`, `RL-SHOW-SPLIT-001`, `RL-UI-SHOWDOWN-001`, and the relevant `RL-VIS-*`/`RL-REPLAY-SERIAL-001` rows.

## Files to Touch

- `games/river_ledger/tests/golden_traces/showdown-seat-label-consistency.trace.json` (new)
- `games/river_ledger/tests/golden_traces/split-winner-order-vs-remainder.trace.json` (new)
- `games/river_ledger/tests/golden_traces/high-card-showdown.trace.json` (modify)
- `games/river_ledger/tests/golden_traces/pair-beats-high-card.trace.json` (modify)
- `games/river_ledger/tests/golden_traces/split-pot-even.trace.json` (modify)
- `games/river_ledger/tests/golden_traces/split-pot-remainder-button-order.trace.json` (modify)
- `games/river_ledger/tests/replay.rs` (modify)
- `games/river_ledger/tests/serialization.rs` (modify)
- `games/river_ledger/docs/RULE-COVERAGE.md` (modify)

## Out of Scope

- Any production logic change (owned by RIVLEDSHOSEA-001..003).
- Changing trace schema/version or replay compatibility policy (would require an ADR — explicitly avoided).
- Active-seat / viewer / card surfaces (RIVLEDSHOSEA-005..009).
- Final `UI.md`/`RULES.md`/`specs/README.md` closeout (RIVLEDSHOSEA-010).

## Acceptance Criteria

### Tests That Must Pass

1. `cargo run -p replay-check -- --game river_ledger --all` green, including both new fixtures.
2. `cargo run -p fixture-check -- --game river_ledger` and `cargo run -p rule-coverage -- --game river_ledger` green.
3. `cargo test -p river_ledger` (replay + serialization) green; every changed trace hash has a reviewed, recorded cause.

### Invariants

1. Trace schema/version is unchanged; only winner-order/label/allocation values migrate.
2. No trace reveals a folded/non-revealed private card; showdown reveals stay within `RL-VIS-SHOWDOWN-001`.

## Test Plan

### New/Modified Tests

1. `games/river_ledger/tests/golden_traces/showdown-seat-label-consistency.trace.json` — seed-`10018` unique-winner label coherence fixture.
2. `games/river_ledger/tests/golden_traces/split-winner-order-vs-remainder.trace.json` — seed-`31`/button-`seat_2` canonical-vs-remainder fixture.
3. `games/river_ledger/tests/replay.rs`, `tests/serialization.rs` — assertions over the new fixtures and migrated payloads.

### Commands

1. `cargo test -p river_ledger`
2. `cargo run -p replay-check -- --game river_ledger --all && cargo run -p fixture-check -- --game river_ledger && cargo run -p rule-coverage -- --game river_ledger`
3. `cargo run -p simulate -- --game river_ledger --seat-count 4 --games 1 --start-seed 10018` and `--start-seed 31 --seat-count 4` to reproduce the locked regression states directly.

## Outcome

Completed: 2026-06-18

What changed:
- Added `showdown-seat-label-consistency.trace.json` for seed `10018` and `split-winner-order-vs-remainder.trace.json` for the seed-`31`/button-`seat_2` canonical-vs-remainder regression.
- Migrated the split remainder golden fixture to canonical winner/allocation order while keeping `expected_remainder_order` button-relative; reviewed high-card, pair, and even-split fixtures with unchanged schema/rules versions.
- Added replay and serialization regressions for seed `10018` public label coherence and seed `31` canonical split ordering.
- Updated `RULE-COVERAGE.md` rows for showdown winner/split, remainder, replay/hash/serialization, visibility, and Rust-authored showdown UI evidence.

Deviations:
- Existing River Ledger golden traces are structural fixtures rather than full replay logs. This ticket extended that fixture contract and added native replay/serialization assertions for semantic lock-in instead of changing trace schema.

Verification:
- `cargo run -p replay-check -- --game river_ledger --all` passed.
- `cargo run -p fixture-check -- --game river_ledger` passed.
- `cargo run -p rule-coverage -- --game river_ledger` passed.
- `cargo test -p river_ledger` passed.
- `cargo run -p simulate -- --game river_ledger --seat-count 4 --games 1 --start-seed 10018` passed and reported `seat_0` as the unique winner.
- `cargo run -p simulate -- --game river_ledger --seat-count 4 --games 1 --start-seed 31` passed and reported a three-way split for `seat_1`, `seat_2`, and `seat_3`.
- `cargo fmt --all --check` passed.
- `cargo clippy --workspace --all-targets -- -D warnings` passed.
- `cargo build --workspace` passed.
- `rg -n --pcre2 '"schema_version": (?!1\b)|"rules_version": "(?!river-ledger-rules-v1")' games/river_ledger/tests/golden_traces` returned no matches.

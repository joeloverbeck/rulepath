# GAT151RIVLED-017: Golden traces

**Status**: COMPLETED
**Priority**: HIGH
**Effort**: Large
**Engine Changes**: Yes (deterministic evidence) — new `games/river_ledger/tests/golden_traces/*.trace.json`, `tests/replay.rs`
**Deps**: GAT151RIVLED-016

## Problem

Author the minimum 22-scenario golden-trace set (spec §7.4) capturing setup, all-in betting, reopening, runout, side-pot construction/allocation, returns, ties, remainders, no-leak, and WASM-exported terminal cases. Update any shipped Gate 15 trace whose public or internal state changed under the v2 migration, documenting each intentional drift; no unexplained deletion or mass rewrite.

## Assumption Reassessment (2026-06-20)

1. Code: `games/river_ledger/tests/golden_traces/` currently holds 25 v1 traces (e.g. `split-pot-even.trace.json`, `public-observer-no-leak.trace.json`); `tests/replay.rs` replays them. The v2 hash/serialization landed in GAT151RIVLED-011, so v1 traces with changed state need explicit migration here.
2. Docs: spec §7.4 names the 22 required scenarios and the coverage spread (≥1 three-seat and one six-seat hand, three distinct live caps, a folded contributor, a returned excess, a sole-eligible pot, a pot tie, different winners across pots, an odd unit in >1 final pot). Existing traces remain unless an explicit v2 migration note explains a change.
3. Cross-artifact boundary under audit: golden traces ↔ the v2 serialization/hash (GAT151RIVLED-011) ↔ `replay-check`; expected hashes/versions are recorded per trace.
4. (§11 determinism + no-weakening) Restate: traces are deterministic evidence; each must replay identically with recorded hash/version, and any changed shipped trace carries an individually-reviewed migration note (no bulk-accept). Confirm no-leak traces expose no private card the viewer is not entitled to.

## Architecture Check

1. Co-locating the new scenario traces with the behavior they capture, validated cross-cuttingly by `replay-check`, keeps each trace tied to a reviewable expectation rather than a bulk regeneration.
2. No backwards-compatibility shims; changed v1 traces are migrated with notes, not silently overwritten.
3. No production logic changes; this is evidence authoring over the GAT151RIVLED-009/-010/-011 surfaces.

## Verification Layers

1. 22 new scenarios replay identically with recorded hashes -> golden-trace / deterministic replay-hash check.
2. Coverage spread satisfied (caps, folded money, returns, ties, odd units, sole eligibility) -> per-trace assertions.
3. Public-observer and seat-private multi-pot traces leak nothing -> no-leak trace assertions (reusing GAT151RIVLED-016 expectations).
4. Changed shipped traces carry individual migration notes -> manual review + `replay-check --all`.

## What to Change

### 1. New scenario traces

Author the 22 named traces from spec §7.4 (setup-equal/asymmetric, short blind all-in, call-all-in-below-price, exact-call exhausts, short open/raise all-in, cumulative-reopen, full-all-in-raise, cap-blocks-short-raise, three-way main+two side pots, folded-contribution retained, uncalled-return, different/tied winners, per-pot remainder button order, all-all-in runout, sole-live foldout, public-observer/seat-private multi-pot no-leak, wasm-exported side-pot terminal).

### 2. Migrate changed shipped traces

For each v1 trace whose state changed under v2, record an explicit migration note (expected hash/version, reason) rather than deleting it; extend `tests/replay.rs` to assert the new set.

## Files to Touch

- `games/river_ledger/tests/golden_traces/setup-equal-default-stacks-3p.trace.json` (new) — plus the remaining 21 §7.4 scenario traces under the same directory
- `games/river_ledger/tests/replay.rs` (modify)

## Out of Scope

- Simulation and benchmarks (GAT151RIVLED-018).
- Doc reconciliation (GAT151RIVLED-019) and closeout (GAT151RIVLED-020).

## Acceptance Criteria

### Tests That Must Pass

1. `cargo run -p replay-check -- --game river_ledger --all` — all new and migrated traces replay identically (this resolves the trace red window opened at GAT151RIVLED-011).
2. `cargo test -p river_ledger` — trace-backed replay tests pass.
3. `cargo run -p rule-coverage -- --game river_ledger` — rule rows reference the new golden traces.

### Invariants

1. Each trace replays to its recorded hash/version; no shipped trace is deleted or mass-rewritten without an individual migration note.
2. No-leak traces expose no card the viewer is not authorized to see.

## Test Plan

### New/Modified Tests

1. `games/river_ledger/tests/golden_traces/*.trace.json` — the 22 §7.4 scenarios (new) plus migrated shipped traces.
2. `games/river_ledger/tests/replay.rs` — assertions over the expanded trace set.

### Commands

1. `cargo run -p replay-check -- --game river_ledger --all`
2. `cargo test -p river_ledger`
3. `cargo run -p rule-coverage -- --game river_ledger` — `replay-check --all` is the authoritative trace boundary; coverage finalizes with GAT151RIVLED-019 docs.

## Outcome

Completed 2026-06-20. Added the 22 Gate 15.1 River Ledger golden-trace fixtures under the existing replay-check placeholder schema and added `gate_15_1_golden_trace_set_is_present_and_reviewed` so the required set, migration notes, public expectations, and coverage-spread markers are asserted in Rust. Current `replay-check` still registers River Ledger trace fixtures under `river-ledger-rules-v1`; the new v2 side-pot/all-in/no-leak evidence therefore carries explicit `migration_review` notes instead of silently changing the checker contract.

Verification passed:

1. `cargo run -p replay-check -- --game river_ledger --all`
2. `cargo test -p river_ledger`
3. `cargo run -p rule-coverage -- --game river_ledger`

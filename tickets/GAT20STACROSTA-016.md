# GAT20STACROSTA-016: Benchmarks and thresholds

**Status**: PENDING
**Priority**: MEDIUM
**Effort**: Medium
**Engine Changes**: Yes (benchmarks + deterministic evidence) — `games/starbridge_crossing/benches/{starbridge_crossing.rs,thresholds.json}`, `games/starbridge_crossing/docs/BENCHMARKS.md`
**Deps**: GAT20STACROSTA-011, GAT20STACROSTA-012

## Problem

Gate 20 must benchmark the large-board pressure: setup, move generation, jump-chain enumeration, playout, replay/serialization, and renderer-facing smoke, with variance-aware CI floors. This ticket lands the bench harness, thresholds, and `BENCHMARKS.md` (which `rule-coverage` also reads, completing its coverage).

## Assumption Reassessment (2026-06-27)

1. Bench layout mirrors siblings: `benches/<game>.rs` + `benches/thresholds.json` (confirmed `games/meldfall_ledger/benches/`). Stable operation names are pinned from spec §7 (`setup_121_spaces_{2,3,4,6}p`, `legal_actions_*`, `jump_chain_enumeration_*`, `apply_*`, `simulate_l0_*`, `serialize_*`/`replay_full_trace_6p`/`wasm_public_view_bridge_*`).
2. `BENCHMARKS.md` is the third `rule-coverage`-consumed doc; landing it here closes the partial-green window opened in GAT20STACROSTA-013.
3. Cross-artifact boundary: benches drive the rules engine (007–009), bot playout (012), and replay/serialization (011); thresholds follow the variance-aware CI floor convention (ADR 0003/0005).
4. §11 (evidence coverage) motivates this ticket: benchmarks are a required acceptance surface; thresholds must be variance-aware and keep failing thresholds visible rather than hiding large-board pressure.

## Architecture Check

1. A dedicated bench ticket with stable op names + variance-aware floors isolates performance evidence and lets CI gate large-board regressions; co-locating `BENCHMARKS.md` follows the validator-consumed-docs rule.
2. No backwards-compatibility shims.
3. No `engine-core`/`game-stdlib` change; benches are game-local.

## Verification Layers

1. Bench harness runs -> benchmark check: `cargo bench -p starbridge_crossing` (or its smoke filter) produces the named operations.
2. Thresholds present + variance-aware -> manual review of `thresholds.json` against ADR 0003/0005 convention.
3. `rule-coverage` complete -> `cargo run -p rule-coverage -- --game starbridge_crossing` is now fully green (BENCHMARKS.md present).
4. Op-name stability -> grep-proof: `thresholds.json` operation names match `BENCHMARKS.md`.

## What to Change

### 1. Author `benches/starbridge_crossing.rs`

Bench the spec §7 operation set: setup per seat count, legal-action generation (start + midgame + dense jump), jump-chain enumeration, single-step / multi-hop / blocked-pass apply, L0 playout throughput, serialize/replay/wasm-bridge.

### 2. Author `benches/thresholds.json`

Stable operation names + variance-aware CI floors.

### 3. Author `games/starbridge_crossing/docs/BENCHMARKS.md`

Document the operations, native baselines, CI floors, and the large-board UI budget.

## Files to Touch

- `games/starbridge_crossing/benches/starbridge_crossing.rs` (new)
- `games/starbridge_crossing/benches/thresholds.json` (new)
- `games/starbridge_crossing/docs/BENCHMARKS.md` (new)

## Out of Scope

- Browser smoke metrics wiring beyond documenting the budget (the e2e smoke is GAT20STACROSTA-017).
- Bench harness changes to other games.

## Acceptance Criteria

### Tests That Must Pass

1. `cargo bench -p starbridge_crossing` (or the crate's bench smoke filter)
2. `cargo run -p rule-coverage -- --game starbridge_crossing` (now fully green)
3. `bash scripts/boundary-check.sh`

### Invariants

1. Benchmark operation names are stable and recorded in both `thresholds.json` and `BENCHMARKS.md`.
2. Thresholds are variance-aware; failing thresholds stay visible (§11, ADR 0003/0005).

## Test Plan

### New/Modified Tests

1. `games/starbridge_crossing/benches/starbridge_crossing.rs` — the operation set above.
2. `games/starbridge_crossing/benches/thresholds.json` — CI floors.

### Commands

1. `cargo bench -p starbridge_crossing`
2. `cargo run -p rule-coverage -- --game starbridge_crossing`
3. The bench run + rule-coverage completion are the correct boundary; full CI benchmark gating runs in the gate-2 lane.

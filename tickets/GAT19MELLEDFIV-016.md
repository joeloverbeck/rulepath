# GAT19MELLEDFIV-016: Golden-trace consolidation, fixtures, property/serialization tests, and replay-check/fixture-check registration

**Status**: PENDING
**Priority**: HIGH
**Effort**: Large
**Engine Changes**: Yes — deterministic evidence (`games/meldfall_ledger/data/fixtures/*`, `tests/{property,serialization,replay}.rs`); `tools/{replay-check,fixture-check}` game arms
**Deps**: GAT19MELLEDFIV-010, GAT19MELLEDFIV-011, GAT19MELLEDFIV-013, GAT19MELLEDFIV-014

## Problem

Gate 19 needs its deterministic evidence consolidated: the six fixture profiles, the property tests (deck uniqueness, card-ownership conservation, no card in two zones, public counts sum to 52, legal-apply never panics, score deltas equal card-value accounting, redacted views never expose hidden cards), the serialization/hash stability tests, and the completion profile. It also registers `meldfall_ledger` in `replay-check` (per-game `trace_dir` table) and `fixture-check` (`--game` allowlist) so the golden traces authored across the pipeline tickets and the fixtures are validated end-to-end.

## Assumption Reassessment (2026-06-25)

1. `tools/replay-check/src/main.rs` carries a hardcoded per-game `trace_dir` table and `tools/fixture-check/src/main.rs` carries a `--game` allowlist (confirmed during reassessment — both require explicit registration; invocation is `replay-check -- --game meldfall_ledger --all` and `fixture-check -- --game meldfall_ledger`). Golden traces exist from the pipeline tickets (004–013); `tests/{property,serialization,replay}.rs` are stubs from GAT19MELLEDFIV-003.
2. Spec §7.2 (test taxonomy), §7.4 (golden trace minimum set), §7.5 (fixtures + completion profile) define the evidence; `games/blackglass_pact/data/fixtures/*.fixture.json` and `tests/property.rs` are the shape exemplars.
3. Cross-artifact: the trace/fixture contract (Trace Schema v1, ADR 0009) is the boundary; `replay-check`/`fixture-check` are the validators — registering the game makes the pipeline traces + fixtures CI-checked. No blanket golden regeneration.
4. FOUNDATIONS §11 determinism: property tests assert card-ownership conservation and that redacted views never expose hidden cards; serialization tests assert stable export order with no hash drift without a trace note.
5. FOUNDATIONS §11 no-leak: the "redacted views never expose hidden cards" property is a structural firewall check complementing the GAT19MELLEDFIV-013 matrix.

## Architecture Check

1. Consolidating fixtures + property/serialization tests + the two tool registrations in one ticket gives a single "evidence is green end-to-end" reviewable diff, while the per-behavior golden traces stay with their authoring tickets.
2. No backwards-compatibility shims; no blanket golden regeneration.
3. `engine-core` untouched; the tool arms are additive registration.

## Verification Layers

1. All golden traces replay deterministically -> `cargo run -p replay-check -- --game meldfall_ledger --all`.
2. Fixtures complete and conform -> `cargo run -p fixture-check -- --game meldfall_ledger`.
3. Property invariants hold (conservation, 52-count, no-leak, score accounting) + stable serialization -> `cargo test -p meldfall_ledger` (property + serialization).

## What to Change

### 1. Fixtures

`data/fixtures/`: `meldfall_ledger_2p_standard`, `_4p_standard`, `_6p_standard`, `_multi_discard_pickup`, `_layoff_any_tableau`, `_500_tie_continues` `.fixture.json`.

### 2. Property + serialization tests

`tests/property.rs` (deck uniqueness, ownership conservation, no double-zone card, public counts sum to 52, legal-apply never panics, score-delta accounting, redacted views never leak); `tests/serialization.rs` (stable export order for tableau/discard/hands/score-ledger/effects); `tests/replay.rs` (deterministic replay, viewer-scoped round trips — extends GAT19MELLEDFIV-013).

### 3. `replay-check` + `fixture-check` registration

Add `meldfall_ledger` to the `replay-check` `trace_dir` table (`games/meldfall_ledger/tests/golden_traces`) and the `fixture-check` `--game` allowlist (+ fixtures dir).

## Files to Touch

- `games/meldfall_ledger/data/fixtures/meldfall_ledger_2p_standard.fixture.json` (new)
- `games/meldfall_ledger/data/fixtures/meldfall_ledger_4p_standard.fixture.json` (new)
- `games/meldfall_ledger/data/fixtures/meldfall_ledger_6p_standard.fixture.json` (new)
- `games/meldfall_ledger/data/fixtures/meldfall_ledger_multi_discard_pickup.fixture.json` (new)
- `games/meldfall_ledger/data/fixtures/meldfall_ledger_layoff_any_tableau.fixture.json` (new)
- `games/meldfall_ledger/data/fixtures/meldfall_ledger_500_tie_continues.fixture.json` (new)
- `games/meldfall_ledger/tests/property.rs` (modify; created by GAT19MELLEDFIV-003)
- `games/meldfall_ledger/tests/serialization.rs` (modify; created by GAT19MELLEDFIV-003)
- `games/meldfall_ledger/tests/replay.rs` (modify; created by GAT19MELLEDFIV-003)
- `tools/replay-check/src/main.rs` (modify)
- `tools/fixture-check/src/main.rs` (modify)

## Out of Scope

- `RULE-COVERAGE.md` + `rule-coverage` registration (GAT19MELLEDFIV-018) and benchmarks (GAT19MELLEDFIV-017).
- WASM/web registration (GAT19MELLEDFIV-019/020/021).

## Acceptance Criteria

### Tests That Must Pass

1. `cargo run -p replay-check -- --game meldfall_ledger --all` passes on all pipeline golden traces.
2. `cargo run -p fixture-check -- --game meldfall_ledger` passes on the six fixtures.
3. `cargo test -p meldfall_ledger` property + serialization tests pass; `cargo test --workspace` passes.

### Invariants

1. Card ownership is conserved and redacted views never expose hidden cards (FOUNDATIONS §11).
2. Serialization order is stable; no hash drift without an authorized trace note (FOUNDATIONS §11; ADR 0009).

## Test Plan

### New/Modified Tests

1. `games/meldfall_ledger/tests/property.rs` — conservation / 52-count / no-leak / score-accounting properties.
2. `games/meldfall_ledger/tests/serialization.rs` + `tests/replay.rs` — stable order + deterministic viewer-scoped replay.
3. `games/meldfall_ledger/data/fixtures/*.fixture.json` — six completion fixtures.

### Commands

1. `cargo test -p meldfall_ledger`
2. `cargo run -p replay-check -- --game meldfall_ledger --all && cargo run -p fixture-check -- --game meldfall_ledger`
3. `cargo test --workspace`

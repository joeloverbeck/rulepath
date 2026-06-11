# GAT13FROCONASY-009: Native test suite and golden traces

**Status**: PENDING
**Priority**: HIGH
**Effort**: Large
**Engine Changes**: Yes — `games/frontier_control/tests/{rules,property,replay,serialization,visibility,bots}.rs` and `games/frontier_control/tests/golden_traces/*` (seventeen traces)
**Deps**: GAT13FROCONASY-006, GAT13FROCONASY-007, GAT13FROCONASY-008

## Problem

The official-game contract (FOUNDATIONS §6/§11) requires the full native evidence set: rule/property/replay/serialization/visibility/bot tests plus the committed golden traces, each carrying the perfect-information/no-randomness `not_applicable` markers. This ticket completes the suites stubbed by earlier pipeline tickets and authors the seventeen golden traces enumerated in the spec, proving determinism, no-leak visibility, bot legality, and the asymmetric/connectivity rules end to end.

## Assumption Reassessment (2026-06-11)

1. `games/flood_watch/tests/{rules,property,replay,serialization,visibility,bots}.rs` and `tests/golden_traces/` are the structural exemplars (verified present); the rules/visibility/replay/bot surfaces from GAT13FROCONASY-005–008 are in place to exercise.
2. Spec §Acceptance evidence enumerates the test coverage and the seventeen traces (`standard-garrison-win`, `standard-prospector-win`, `tie-garrison-tiebreak`, `clash-crew-into-guards`, `clash-guard-into-crews`, `stake-and-dismantle`, `supply-cut-scores-zero`, `round-scoring-breakdown`, `budget-exhaustion-auto-end`, `early-end-turn`, `muster-and-reinforce-caps`, `highlands-setup`, `wrong-faction-diagnostic`, `non-adjacent-move-diagnostic`, `stake-on-guarded-site-diagnostic`, `bot-vs-bot-full-game`, `replay-export-import`) — count and list agree (17).
3. Cross-artifact boundary under audit: the golden traces are consumed by `tools/replay-check` (registered in GAT13FROCONASY-013) and the rule-coverage map; trace filenames authored here must match the `replay-check` registration and the `RULE-COVERAGE.md` obligations.
4. FOUNDATIONS §11 acceptance invariants under audit: determinism over the command stream, no-leak viewer-equivalence, bot legality, and bounded/typed serialization are all proven here; the failing-test protocol applies — never weaken a test to get green.
5. §11/§13 enforcement surface: replay/hash determinism, no-leak negative tests, and the Trace Schema v1 §5 `not_applicable` markers are the surfaces under audit; serialization tests prove unknown-field and behavior-looking-field rejection in map data.

## Architecture Check

1. Completing all suites + traces in one ticket (vs scattering across pipeline tickets) gives a single reviewable evidence diff and lets each trace assert against the finished rules; earlier tickets carried only stubs to keep their diffs reviewable.
2. No backwards-compatibility aliasing/shims; no test is weakened to pass.
3. `engine-core`/`game-stdlib` untouched; tests are game-local.

## Verification Layers

1. Rules/diagnostics -> rule tests (deterministic setup both maps, adjacency legality, faction separation, budget, both clash directions, stake/muster/reinforce/dismantle, supply scoring, terminal, tiebreak, viewer-safe diagnostics).
2. Invariants -> property tests (edge-following, caps, unit-count conservation through documented effects, no-stall, once-per-round scoring, terminal bound, winner = comparison + tiebreak, no panics).
3. Determinism + no-leak -> replay-hash check + no-leak visibility test + serialization tests (stable summaries; unknown/behavior-looking fields rejected).
4. Golden traces -> golden trace check (all seventeen present with Trace Schema v1 §5 markers).

## What to Change

### 1. Complete the six test suites

Expand `rules.rs`, `property.rs`, `replay.rs`, `serialization.rs`, `visibility.rs`, `bots.rs` to the full coverage in the spec §Acceptance evidence.

### 2. Author the seventeen golden traces

Commit each under `tests/golden_traces/` with the perfect-information/no-randomness `not_applicable` markers.

## Files to Touch

- `games/frontier_control/tests/rules.rs` (modify)
- `games/frontier_control/tests/property.rs` (modify)
- `games/frontier_control/tests/replay.rs` (modify)
- `games/frontier_control/tests/serialization.rs` (modify)
- `games/frontier_control/tests/visibility.rs` (modify)
- `games/frontier_control/tests/bots.rs` (modify)
- `games/frontier_control/tests/golden_traces/standard-garrison-win.trace.json` (new)
- `games/frontier_control/tests/golden_traces/standard-prospector-win.trace.json` (new)
- `games/frontier_control/tests/golden_traces/tie-garrison-tiebreak.trace.json` (new)
- `games/frontier_control/tests/golden_traces/clash-crew-into-guards.trace.json` (new)
- `games/frontier_control/tests/golden_traces/clash-guard-into-crews.trace.json` (new)
- `games/frontier_control/tests/golden_traces/stake-and-dismantle.trace.json` (new)
- `games/frontier_control/tests/golden_traces/supply-cut-scores-zero.trace.json` (new)
- `games/frontier_control/tests/golden_traces/round-scoring-breakdown.trace.json` (new)
- `games/frontier_control/tests/golden_traces/budget-exhaustion-auto-end.trace.json` (new)
- `games/frontier_control/tests/golden_traces/early-end-turn.trace.json` (new)
- `games/frontier_control/tests/golden_traces/muster-and-reinforce-caps.trace.json` (new)
- `games/frontier_control/tests/golden_traces/highlands-setup.trace.json` (new)
- `games/frontier_control/tests/golden_traces/wrong-faction-diagnostic.trace.json` (new)
- `games/frontier_control/tests/golden_traces/non-adjacent-move-diagnostic.trace.json` (new)
- `games/frontier_control/tests/golden_traces/stake-on-guarded-site-diagnostic.trace.json` (new)
- `games/frontier_control/tests/golden_traces/bot-vs-bot-full-game.trace.json` (new)
- `games/frontier_control/tests/golden_traces/replay-export-import.trace.json` (new)

## Out of Scope

- Benchmarks (GAT13FROCONASY-010); bot-evidence docs (GAT13FROCONASY-011).
- Tool/WASM/CI registration (GAT13FROCONASY-012/013).

## Acceptance Criteria

### Tests That Must Pass

1. `cargo test -p frontier_control` passes (all six suites).
2. All seventeen traces are present and each carries the Trace Schema v1 §5 `not_applicable` markers.
3. `cargo clippy -p frontier_control --all-targets -- -D warnings` passes.

### Invariants

1. Determinism, no-leak viewer-equivalence, and bot legality are proven by dedicated suites (§11).
2. Serialization rejects unknown and behavior-looking fields in map data.

## Test Plan

### New/Modified Tests

1. The six `tests/*.rs` suites — full official-game coverage.
2. Seventeen `tests/golden_traces/*.trace.json` — listed above.

### Commands

1. `cargo test -p frontier_control`
2. `cargo test --workspace`
3. Crate + workspace tests are the correct boundary; `replay-check --all` validates the traces after registration in GAT13FROCONASY-013.

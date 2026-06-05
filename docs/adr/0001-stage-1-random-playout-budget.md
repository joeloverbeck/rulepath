# ADR: Stage 1 Random Playout Benchmark Budget

Status: Accepted

Date: 2026-06-05

Decision owner: joeloverbeck

Related documents:

- `docs/FOUNDATIONS.md`
- `docs/TESTING-REPLAY-BENCHMARKING.md`
- `specs/gate-2-trace-replay-benchmark-hardening.md`
- `games/race_to_n/docs/BENCHMARKS.md`
- `games/race_to_n/benches/thresholds.json`

## Context

Gate 2 requires the recorded Stage 1 `race_to_n` `random_playout` miss to be resolved rather than silently waived. The provisional doctrine target was 500,000 games/sec. The measured native custom harness executes full validated random playouts: each turn asks the Level 0 random legal bot for an action through the legal action tree, validates the command through Rust rules, applies the action, and repeats until terminal.

Current local WSL2 evidence after structured benchmark reporting:

- full `cargo bench -p race_to_n`: `random_playout` around 136,000 to 140,000 games/sec before recalibration;
- focused `cargo bench -p race_to_n -- random_playout` after removing an unused public-view projection from `RaceRandomBot::select_action`: about 108,000 games/sec;
- `legal_actions` and `bot_decision` are themselves in the low single-digit millions per second, while a complete game normally takes several validated turns.

That makes 500,000 complete validated games/sec unrealistic for the current correctness-preserving benchmark shape. Lowering the threshold only to make CI green is forbidden, so an accepted benchmark-doctrine decision is required.

## Decision

Stage 1 `race_to_n` validated random playout throughput is recalibrated from the provisional 500,000 games/sec target to a conservative required floor of 100,000 games/sec for Gate 2 benchmark gating.

The threshold applies to the native custom harness operation named `random_playout` and MUST be enforced by `tools/bench-report` against `games/race_to_n/benches/thresholds.json`.

The old 500,000 games/sec target is no longer a Gate 2 acceptance threshold for this operation. It remains useful only as an aspirational profiling reference for future harness or algorithm work.

## Alternatives considered

| Alternative | Why considered | Why accepted/rejected |
|---|---|---|
| Keep 500,000 games/sec | It was the provisional Stage 1 budget. | Rejected. The measured harness does full bot selection, validation, action application, and terminal playout; safe local cleanup did not approach the target. |
| Optimize by bypassing validation or legal action APIs | It could increase throughput. | Rejected. The benchmark is meant to measure correctness-preserving random legal simulation. |
| Replace the wall-clock harness with Criterion or Iai-Callgrind now | It could produce richer benchmark statistics. | Rejected for Gate 2 scope. The current custom harness is accepted by the spec; tool migration can be revisited if wall-clock noise becomes the blocker. |
| Recalibrate to 100,000 games/sec | It is below repeated WSL2 measurements while still hard-failing meaningful regressions. | Accepted. It is conservative for noisy local/CI contexts and remains enforced by `bench-report`. |

## Consequences

Positive consequences:

- Gate 2 benchmark gating remains hard-failing rather than report-only.
- The threshold matches the actual validated playout harness instead of an unprofiled provisional target.
- Future regressions below 100,000 games/sec fail deterministically.

Negative or risky consequences:

- The lower threshold is less ambitious than the original provisional target.
- A faster raw playout benchmark could still be useful later, but it would measure a different surface and needs a separate operation name.

Operational requirements:

- `games/race_to_n/benches/thresholds.json` records `random_playout` at 100,000 games/sec with `accepted_adr` rationale.
- `games/race_to_n/docs/BENCHMARKS.md` records the decision, command, environment, caveats, and current values.
- `cargo run -p bench-report -- --input <report> --thresholds games/race_to_n/benches/thresholds.json` remains the hard-failing gate.

## Determinism impact

This decision does not change RNG contracts, replay, serialization order, or rule behavior. The small bot-path cleanup removes an unused view projection before random legal selection; bot selection still uses the same legal action tree and deterministic seed path.

Determinism proof remains `cargo test -p race_to_n`, including replay and bot tests.

## Replay/hash impact

No command stream, state hash, effect hash, action-tree hash, public-view hash, or trace format changes are intended. Existing Trace Schema v1 golden traces remain valid.

## Visibility impact

`race_to_n` is perfect-information. Removing an unused public-view projection from the direct bot helper does not expose private data because no private data exists and `select_action_from_view` remains available for callers that already hold a view.

## Data/Rust boundary impact

The threshold file remains typed benchmark policy data. It does not define behavior. Unknown-field and behavior-key validation stay owned by fixture/static-data tooling.

## `engine-core` Contamination Risk

No `engine-core` changes are made. The benchmark decision and small bot cleanup stay in `games/race_to_n`.

## `game-stdlib` / Primitive-Pressure Impact

No shared primitive is introduced.

## UI Impact

No UI behavior changes.

## Bot Impact

`RaceRandomBot::select_action` still chooses from the Rust legal action tree with deterministic seed input. It no longer constructs an unused public view in the direct state-based helper.

## IP Impact

No IP impact.

## Benchmark Impact

The accepted Stage 1 native `random_playout` threshold is 100,000 games/sec. The benchmark report must continue to include command, OS, Rust version, build profile, engine/rules/data versions, caveats, current value, threshold, pass/fail, and rationale class.

## Migration Notes

Update benchmark doctrine references from the provisional 500,000 games/sec target to the accepted 100,000 games/sec Gate 2 threshold for `race_to_n` validated random playouts.

# GAT91SECDRACOM-010: secret_draft golden traces + replay test

**Status**: PENDING
**Priority**: HIGH
**Effort**: Large
**Engine Changes**: Yes — `games/secret_draft/tests/golden_traces/*.trace.json` and `games/secret_draft/tests/replay.rs`; no `engine-core` / `game-stdlib` change.
**Deps**: GAT91SECDRACOM-007, GAT91SECDRACOM-009

## Problem

The gate requires a committed golden-trace set that `replay-check` validates, each trace intentionally named so a failure explains the broken invariant — especially the commitment/reveal/no-leak traces that are this gate's reason to exist. The `tests/replay.rs` harness proves deterministic reproduction of state/effect/action-tree/view hashes and reveal ordering.

## Assumption Reassessment (2026-06-08)

1. `games/token_bazaar/tests/golden_traces/` holds the precedent set (12 traces incl. `shortest-normal`, `stale-diagnostic`, `bot-action`, `wasm-exported`); `games/token_bazaar/tests/replay.rs` is the harness precedent (verified). `secret_draft` adds the commitment/reveal-specific traces.
2. The replay surface (GAT91SECDRACOM-007) and breadth suite/fixture (GAT91SECDRACOM-009) are inputs. Spec §"Golden traces" + §"Acceptance evidence → Golden traces" enumerate the required 14 trace files: `shortest-normal`, `first-commit-pending`, `simultaneous-reveal-batch`, `contested-pick-fallback`, `terminal-tie-break`, `draw-after-tie-breaks`, `already-committed-diagnostic`, `unavailable-item-diagnostic`, `stale-diagnostic`, `public-observer-no-leak`, `seat-private-no-prereveal-choice`, `bot-action`, `public-replay-export-import`, `wasm-exported`.
3. Cross-artifact boundary under audit: the golden-trace schema and the replay/hash contract (`docs/TESTING-REPLAY-BENCHMARKING.md`). Traces are consumed by `tests/replay.rs` (this ticket) and `tools/replay-check` (GAT91SECDRACOM-012); the trace format and hash surfaces must match what replay-check expects.
4. §11 determinism + no-leak are the motivating invariants: restate before trusting spec — the no-leak traces (`first-commit-pending`, `public-observer-no-leak`, `seat-private-no-prereveal-choice`, `public-replay-export-import`) must demonstrate that pre-reveal export/observer/seat timelines carry pending booleans but never the chosen item ID; `simultaneous-reveal-batch` must show both IDs appearing together in stable order; `contested-pick-fallback` must remove exactly two items via deterministic priority/fallback.
5. The `wasm-exported` trace is authored here but its WASM-export path is wired in GAT91SECDRACOM-013; the trace asserts the viewer-scoped export shape and is re-exercised by the WASM smoke. No accidental trace/hash/schema migration (spec Forbidden changes).

## Architecture Check

1. Intentionally-named small traces (one invariant each) are cleaner than a few large traces: a failing `seat-private-no-prereveal-choice` immediately names the broken guarantee, which is the whole point of the gate.
2. No backwards-compatibility aliasing/shims.
3. `engine-core` stays noun-free; traces are game-local fixtures. No `game-stdlib` helper.

## Verification Layers

1. Deterministic replay -> `tests/replay.rs`: each trace reproduces state/effect/action-tree/public-and-seat-view hashes, reveal ordering, and terminal outcome.
2. Pre-reveal no-leak in exported timelines -> the no-leak traces assert pending booleans only, no item ID (cross-checked against GAT91SECDRACOM-006/007 redaction).
3. Conflict/terminal correctness -> `contested-pick-fallback` (two removals) and `terminal-tie-break` / `draw-after-tie-breaks` traces.
4. Trace-schema conformance -> traces validate against the format `tools/replay-check` consumes (proven end-to-end in GAT91SECDRACOM-012).

## What to Change

### 1. `games/secret_draft/tests/golden_traces/*.trace.json`

Author all 14 named traces per the spec list, each minimal and targeted to its invariant.

### 2. `games/secret_draft/tests/replay.rs`

Replay harness asserting deterministic reproduction of all hashes + reveal ordering + terminal outcome for every committed trace; re-derive expected counts from the trace/fixture rather than hardcoding.

## Files to Touch

- `games/secret_draft/tests/golden_traces/shortest-normal.trace.json` (new)
- `games/secret_draft/tests/golden_traces/first-commit-pending.trace.json` (new)
- `games/secret_draft/tests/golden_traces/simultaneous-reveal-batch.trace.json` (new)
- `games/secret_draft/tests/golden_traces/contested-pick-fallback.trace.json` (new)
- `games/secret_draft/tests/golden_traces/terminal-tie-break.trace.json` (new)
- `games/secret_draft/tests/golden_traces/draw-after-tie-breaks.trace.json` (new)
- `games/secret_draft/tests/golden_traces/already-committed-diagnostic.trace.json` (new)
- `games/secret_draft/tests/golden_traces/unavailable-item-diagnostic.trace.json` (new)
- `games/secret_draft/tests/golden_traces/stale-diagnostic.trace.json` (new)
- `games/secret_draft/tests/golden_traces/public-observer-no-leak.trace.json` (new)
- `games/secret_draft/tests/golden_traces/seat-private-no-prereveal-choice.trace.json` (new)
- `games/secret_draft/tests/golden_traces/bot-action.trace.json` (new)
- `games/secret_draft/tests/golden_traces/public-replay-export-import.trace.json` (new)
- `games/secret_draft/tests/golden_traces/wasm-exported.trace.json` (new)
- `games/secret_draft/tests/replay.rs` (new)

## Out of Scope

- `tools/replay-check` registration (GAT91SECDRACOM-012) — though it consumes these traces.
- WASM export wiring that the `wasm-exported` trace's path uses (GAT91SECDRACOM-013).
- Benchmarks (GAT91SECDRACOM-011).

## Acceptance Criteria

### Tests That Must Pass

1. `cargo test -p secret_draft --test replay` — all 14 traces reproduce deterministically.
2. No-leak traces contain no unrevealed committed item ID in their observer/seat/export timelines.
3. `contested-pick-fallback` removes exactly two items; `terminal-tie-break`/`draw-after-tie-breaks` exercise the ladder to a winner and to `Draw`.

### Invariants

1. Same inputs → identical hashes + reveal ordering for every trace (§11 determinism).
2. Pre-reveal exported/observer/seat timelines never carry the chosen item ID (§11 no-leak).

## Test Plan

### New/Modified Tests

1. `games/secret_draft/tests/replay.rs` + the 14 golden traces — per §What to Change.

### Commands

1. `cargo test -p secret_draft --test replay`
2. `cargo test --workspace`
3. `cargo run -p replay-check -- --game secret_draft --all` becomes runnable after GAT91SECDRACOM-012 registers the game; the `replay` test is the correct in-crate boundary here.

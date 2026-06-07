# GAT6DIRFLI-013: Golden traces & fixtures

**Status**: PENDING
**Priority**: HIGH
**Effort**: Medium
**Engine Changes**: Yes — `games/directional_flip/tests/golden_traces/*.trace.json` (golden trace corpus) and any supporting fixture refinement.
**Deps**: 009, 011

## Problem

The official-game contract requires a golden trace corpus exercising the game's key paths with byte-stable hashes and expected diagnostics/outcomes (FOUNDATIONS §6/§11, spec §8.5, §14). This ticket authors the `directional_flip` golden traces — opening move, multi-direction flip, corner capture, forced pass, double-pass terminal, full-board terminal, draw, invalid-cell diagnostics, stale/non-active diagnostics, bot action, WASM-exported, and preview-flip-set — following `docs/TRACE-SCHEMA-v1.md` strictness.

## Assumption Reassessment (2026-06-07)

1. `games/column_four/tests/golden_traces/` is the precedent corpus (e.g. `bot-action.trace.json`, `*-win.trace.json`, `draw.trace.json`, `*-diagnostic.trace.json`, `terminal-replay.trace.json`, `wasm-exported.trace.json`). Replay support (GAT6DIRFLI-009) and bots (011) exist to generate/verify these. The standard-setup fixture `games/directional_flip/data/fixtures/directional_flip_standard.fixture.json` exists from GAT6DIRFLI-004.
2. Spec §8.5 (required trace list — coverage must not silently shrink) and `docs/TRACE-SCHEMA-v1.md` (no unknown fields, no behavior-looking keys, stable IDs, expected diagnostics/outcomes/checkpoints/hashes) are authoritative. The double-pass-terminal trace is explicitly required (spec §5.1 terminal).
3. Cross-artifact boundary under audit: the traces ↔ `replay_support.rs` (009, the projection they capture) ↔ `tools/replay-check` (GAT6DIRFLI-016, which discovers and verifies them). Trace filenames/shape must match what `replay-check`'s directional-flip discovery (016) expects.
4. FOUNDATIONS §6/§11 motivate this ticket: restate before authoring — golden traces are required evidence and must be deterministic; effect/flip ordering in the trace is the §6.4 canonical order, and the preview-flip-set trace proves `DF-PREVIEW-001` end-to-end.
5. This is a deterministic replay/hash & serialization surface (FOUNDATIONS §11): confirm each trace's stored hash is reproducible (no wall-clock/RNG in canonical form) and that traces carry only viewer-safe views/effects (no hidden-information leak in the trace export — §11 no-leak firewall).

## Architecture Check

1. A trace per key path (rather than one mega-trace) keeps each reviewable and lets `replay-check` pinpoint a regression to a specific scenario — the `column_four` corpus pattern.
2. No backwards-compatibility shims; new trace fixtures.
3. `engine-core` untouched; traces are game-local data conforming to the generic trace schema (§3).

## Verification Layers

1. Trace determinism/hash -> deterministic replay-hash check (`DF-REPLAY-001`): every trace replays to its stored hash and expected outcome.
2. Schema strictness -> schema/serialization validation against `docs/TRACE-SCHEMA-v1.md`: no unknown/behavior-looking keys; stable ids.
3. Coverage completeness -> manual review against spec §8.5: each required scenario (multi-direction flip, forced pass, double-pass terminal, invalid diagnostics, bot action, preview-flip-set, WASM-exported) has a trace; no silent shrinkage.
4. Preview==apply end-to-end -> golden trace (`DF-PREVIEW-001`): the preview-flip-set trace captures matching previewed and applied flip sets.

## What to Change

### 1. Golden trace corpus

Author `games/directional_flip/tests/golden_traces/` per spec §8.5: `opening-legal-move`, `multi-direction-flip`, `corner-capture`, `forced-pass`, `double-pass-terminal`, `full-board-terminal`, `draw`, `invalid-occupied-cell`, `invalid-non-flipping-placement`, `stale-diagnostic`, `non-active-seat-diagnostic`, `bot-action`, `wasm-exported`, `preview-flip-set` (names may adjust to repo style; coverage must not shrink). Generate via the Rust replay support so stored hashes are authoritative.

## Files to Touch

- `games/directional_flip/tests/golden_traces/opening-legal-move.trace.json` (new)
- `games/directional_flip/tests/golden_traces/multi-direction-flip.trace.json` (new)
- `games/directional_flip/tests/golden_traces/corner-capture.trace.json` (new)
- `games/directional_flip/tests/golden_traces/forced-pass.trace.json` (new)
- `games/directional_flip/tests/golden_traces/double-pass-terminal.trace.json` (new)
- `games/directional_flip/tests/golden_traces/full-board-terminal.trace.json` (new)
- `games/directional_flip/tests/golden_traces/draw.trace.json` (new)
- `games/directional_flip/tests/golden_traces/invalid-occupied-cell.trace.json` (new)
- `games/directional_flip/tests/golden_traces/invalid-non-flipping-placement.trace.json` (new)
- `games/directional_flip/tests/golden_traces/stale-diagnostic.trace.json` (new)
- `games/directional_flip/tests/golden_traces/non-active-seat-diagnostic.trace.json` (new)
- `games/directional_flip/tests/golden_traces/bot-action.trace.json` (new)
- `games/directional_flip/tests/golden_traces/wasm-exported.trace.json` (new)
- `games/directional_flip/tests/golden_traces/preview-flip-set.trace.json` (new)
- `games/directional_flip/tests/replay.rs` (modify — load and assert the golden corpus)

## Out of Scope

- `tools/replay-check` registration (GAT6DIRFLI-016), which runs `--game directional_flip --all` over this corpus.
- WASM export plumbing (GAT6DIRFLI-015); the `wasm-exported` trace is captured from that surface once it exists (this ticket depends only on the Rust replay support; the wasm-exported trace may be finalized alongside 015 if it requires the wasm path).

## Acceptance Criteria

### Tests That Must Pass

1. `cargo test -p directional_flip replay` — golden trace assertions pass.
2. Each trace validates against `docs/TRACE-SCHEMA-v1.md` strictness (no unknown/behavior keys).

### Invariants

1. Every required scenario has a deterministic, hash-stable trace; coverage does not silently shrink (FOUNDATIONS §6, spec §8.5).
2. Traces leak no hidden information and use stable ids (FOUNDATIONS §11, `docs/TRACE-SCHEMA-v1.md`).

## Test Plan

### New/Modified Tests

1. `games/directional_flip/tests/replay.rs` — golden-trace load + replay-hash assertions over the corpus.

### Commands

1. `cargo test -p directional_flip replay`
2. `cargo run -p replay-check -- --game directional_flip --all` — full corpus verification (passes once GAT6DIRFLI-016 registers the game in the tool; cite as the integration gate).
3. The crate-scoped replay test is the in-ticket boundary; the tool-driven `--all` run is the cross-tool gate owned by GAT6DIRFLI-016.

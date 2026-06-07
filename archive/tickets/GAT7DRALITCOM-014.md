# GAT7DRALITCOM-014: Golden traces & standard fixture

**Status**: COMPLETED
**Priority**: HIGH
**Effort**: Medium
**Engine Changes**: Yes — `games/draughts_lite/tests/golden_traces/*.json` (native golden traces) and `games/draughts_lite/data/fixtures/draughts_lite_standard.fixture.json` (standard setup fixture).
**Deps**: 010, 012

## Problem

Golden traces make a failed replay human-readable and pin deterministic behavior; the standard fixture validates the canonical setup. This ticket authors the standard fixture and the native golden traces covering normal, forced, multi-jump, promotion, terminal, invalid-diagnostic, and bot cases, each with a purpose note and expected hashes. The WASM-exported trace is authored in GAT7DRALITCOM-016 (it is produced by the WASM export path).

## Assumption Reassessment (2026-06-07)

1. `games/draughts_lite/src/replay_support.rs` (GAT7DRALITCOM-010) supplies the hashes the traces assert, and `bots.rs` (012) produces the deterministic bot action for the bot trace. `games/directional_flip/tests/golden_traces/*.json` (14 files incl. `wasm-exported.trace.json`) and `data/fixtures/directional_flip_standard.fixture.json` are the structural/shape precedents; `action_path` is stored as a JSON list (verified, e.g. `["place/r4c4"]`).
2. The trace set is fixed by spec §R18 "Golden traces" (18 listed): shortest quiet; mandatory-capture-suppressing-quiet; single capture; multi-jump; forced-continuation branch; promotion-by-quiet; promotion-during-capture stop; terminal no-pieces; terminal no-legal-moves; stale diagnostic; non-active-seat diagnostic; occupied-destination diagnostic; non-playable-cell diagnostic; quiet-while-capture diagnostic; illegal-continuation diagnostic; path-after-promotion-stop diagnostic; bot action; WASM-exported. The standard fixture is fixed by §R18 "Standard fixture".
3. Cross-artifact boundary under audit: the traces are consumed by `tools/replay-check --all` (GAT7DRALITCOM-017) and the replay tests; multi-jump traces must make the selected path intelligible from the `action_path` alone (spec §R10). The fixture is consumed by `tools/fixture-check` (017).
4. FOUNDATIONS §6/§11 motivate this ticket: restate before coding — golden traces fail loudly on drift and carry expected hashes; replay stays deterministic. Each trace includes a plain note explaining the rule under test (spec §R10 "Replay readability").
5. No-leak + determinism enforcement surface (§11): traces and the fixture are committed artifacts that can reach a browser via replay import/export, so confirm they expose only perfect-information facts (no hidden/internal state) and that expected hashes are reproducible across runs/builds.

## Architecture Check

1. Authoring traces as committed JSON with expected hashes + purpose notes (vs. asserting behavior only in code) gives a reviewer a human-readable diff when replay drifts and pins the multi-segment `action_path` contract.
2. No backwards-compatibility shims; new fixture + trace files.
3. `engine-core` stays noun-free (§3); traces/fixtures are game-local data conforming to Trace Schema v1.

## Verification Layers

1. Standard fixture valid -> `cargo run -p fixture-check -- --game draughts_lite` (registered in 017) + fixture test: 8×8, parity, 12 men/side, no pieces off playable cells, `seat_0` active, non-terminal.
2. Trace coverage -> replay test + `cargo run -p replay-check -- --game draughts_lite --all` (in 017): every §R18 trace replays to its expected hashes; diagnostics reproduce stably.
3. Multi-jump legibility -> manual review: the multi-jump trace's `action_path` (`from/ → jump/ → jump/`) is intelligible alone (spec §R10).
4. Determinism -> deterministic replay-hash check: expected hashes are stable across runs.

## What to Change

### 1. Standard fixture

Author `data/fixtures/draughts_lite_standard.fixture.json` per spec §R18: board dimensions, parity, 12 men/side on rows 1–3 / 6–8, `seat_0` active, non-terminal, with the hash baseline if fixtures encode hashes.

### 2. Native golden traces

Author the 17 native traces listed in §R18 (all except WASM-exported) under `tests/golden_traces/`, each with a purpose note and expected state/effect/action-tree/public-view/replay hashes.

## Files to Touch

- `games/draughts_lite/data/fixtures/draughts_lite_standard.fixture.json` (new)
- `games/draughts_lite/tests/golden_traces/` (new — the 17 native `*.trace.json` files per §R18)

## Out of Scope

- The WASM-exported golden trace (GAT7DRALITCOM-016 — produced by the WASM export path).
- The `replay-check` / `fixture-check` tool registration (GAT7DRALITCOM-017; this ticket authors the data those tools validate).
- Any Trace Schema bump (forbidden; spec §R10).

## Acceptance Criteria

### Tests That Must Pass

1. `cargo test -p draughts_lite` — replay tests over the new traces pass (expected hashes match).
2. `cargo test --workspace` — no existing trace regresses.

### Invariants

1. Each trace replays to stable, deterministic hashes and carries a human-readable purpose note (FOUNDATIONS §6/§11; spec §R10).
2. Traces/fixture expose only perfect-information facts; Trace Schema v1 is retained (FOUNDATIONS §11; spec §R10).

## Test Plan

### New/Modified Tests

1. `games/draughts_lite/tests/golden_traces/*.trace.json` — the §R18 native trace set with expected hashes.
2. `games/draughts_lite/data/fixtures/draughts_lite_standard.fixture.json` — standard setup baseline.

### Commands

1. `cargo test -p draughts_lite replay`
2. `cargo test --workspace`
3. Crate + workspace tests are the correct boundary now; the `replay-check --all` and `fixture-check` CLI passes land with tool registration in GAT7DRALITCOM-017.

## Outcome

- Added the standard Draughts Lite fixture at `games/draughts_lite/data/fixtures/draughts_lite_standard.fixture.json`, including public-safe metadata, fixture kinds, Trace Schema v1 marker, standard 8x8 setup, and expected hashes.
- Added the 17 native §R18 Trace Schema v1 golden traces under `games/draughts_lite/tests/golden_traces/`, covering quiet play, mandatory capture, single and multi-jump capture, forced continuation, promotion, terminal states, diagnostics, and deterministic bot choice. The WASM-exported trace remains out of scope for GAT7DRALITCOM-016.
- Added `games/draughts_lite/tests/replay.rs` coverage that validates replay JSON path serialization, standard fixture metadata, trace purpose/migration notes, state/effect/action-tree/public-view/replay hashes, diagnostic codes and hashes, terminal outcomes, multi-jump path legibility, and the Level 1 bot trace command.

Verification passed:

1. `cargo fmt --all --check`
2. `cargo test -p draughts_lite replay`
3. `cargo test -p draughts_lite`
4. `cargo test --workspace`

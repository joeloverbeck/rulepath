# GAT20STACROSTA-011: Replay support, serialization, golden traces, and fixtures

**Status**: COMPLETED
**Priority**: HIGH
**Effort**: Large
**Engine Changes**: Yes (deterministic evidence) — `games/starbridge_crossing/src/replay_support.rs`, `tests/{replay,serialization,property}.rs`, `data/fixtures/*`, `tests/golden_traces/*`
**Deps**: GAT20STACROSTA-010

## Problem

Gate 20 requires deterministic, versioned replay/serialization and a golden-trace catalog covering setup, moves, jump chains, blocked pass, finish order, terminal, turn-limit, and all-public no-leak parity. This ticket lands `replay_support.rs`, the serialization round-trips, the fixtures, and the trace catalog that `replay-check`/`fixture-check` validate.

## Assumption Reassessment (2026-06-27)

1. `engine-core`'s replay recorder + hash/checkpoint contracts already exist (`crates/engine-core/src/lib.rs`); `replay_support.rs` adds only the public-export layer (no redaction needed — perfect information).
2. Fixtures follow the sibling naming `<game_id>_<seats>p_standard.fixture.json` (confirmed `games/meldfall_ledger/data/fixtures/`); the four setup fixtures are `starbridge_crossing_{2,3,4,6}p_standard.fixture.json`. Golden traces are `tests/golden_traces/*.trace.json` (Trace Schema v1).
3. Cross-artifact boundary: traces capture behavior from the rules engine (007–009) and visibility (010); the canonical id/enumeration ordering pinned in 005/008 is the byte-stable basis for replay/serialization hashes.
4. §2/§11 determinism motivates this ticket: identical command stream + seed + variant reproduce action/state/effect/public-view/terminal hashes; no wall-clock or hash-map iteration enters canonical forms.
5. Deterministic replay/hash & serialization enforcement surface (§11/§13): `tests/replay.rs` + `replay-check` (registered in 013) + `tests/serialization.rs`. Confirm public-replay export/import round-trips with stable hashes and that no ADR 0004 viewer-scoped redaction class applies (perfect information); any hash change later requires a named migration note (§13), but this is the first authoring.

## Architecture Check

1. Consolidating replay/serialization/traces/fixtures into one evidence ticket is correct because the trace catalog spans every module and shares the determinism harness; per-module trace authoring would fragment the no-leak parity proof.
2. No backwards-compatibility shims.
3. `engine-core` replay contract reused; no mechanic noun added; no `game-stdlib` change.

## Verification Layers

1. Replay determinism (§2/§11) -> golden trace / deterministic replay-hash check: `replay-check --game starbridge_crossing` (registered in 013) reproduces hashes.
2. Serialization round-trip -> schema/serialization validation: public view, action tree, effects, setup/options, replay export/import, fixture profiles round-trip and are versioned.
3. Trace catalog coverage -> golden traces: setup 2/3/4/6, single-step, one-hop, multi-hop direction change, stop-midway, repeat-landing-rejected, mixed step+jump, blocked pass, finish, finish-order continues, terminal full standings, turn-limit, public-observer all-public, seat-viewer parity, public replay round-trip, wasm-exported (added in 014), bot-l0 (added in 012).
4. Property determinism -> property test: one occupant per space, legal actions never land on occupied spaces, no chain repeats a landing, replay determinism over many seeds.

## What to Change

### 1. Author `src/replay_support.rs`

Public-export replay layer over the engine-core recorder (no redaction class).

### 2. Author the fixtures + golden traces

Four setup fixtures plus the rule-domain fixtures (dense-jump, blocked); the golden-trace catalog per the spec §7 minimum set.

### 3. Author `tests/{replay,serialization,property}.rs`

Replay-hash reproduction, serialization round-trips, and the property suite (extending the 005 property stub).

## Files to Touch

- `games/starbridge_crossing/src/replay_support.rs` (new)
- `games/starbridge_crossing/tests/replay.rs` (new)
- `games/starbridge_crossing/tests/serialization.rs` (new)
- `games/starbridge_crossing/tests/property.rs` (modify; created by 005)
- `games/starbridge_crossing/data/fixtures/starbridge_crossing_2p_standard.fixture.json` (new)
- `games/starbridge_crossing/data/fixtures/starbridge_crossing_3p_standard.fixture.json` (new)
- `games/starbridge_crossing/data/fixtures/starbridge_crossing_4p_standard.fixture.json` (new)
- `games/starbridge_crossing/data/fixtures/starbridge_crossing_6p_standard.fixture.json` (new)
- `games/starbridge_crossing/tests/golden_traces/` (new; trace catalog)
- `games/starbridge_crossing/src/lib.rs` (modify; created by 004 — add `pub mod replay_support;`)

## Out of Scope

- The `replay-check`/`fixture-check` tool arms — GAT20STACROSTA-013 (this ticket authors the traces/fixtures they read).
- `wasm-exported` and `bot-l0` traces — authored alongside 014 and 012 respectively.

## Acceptance Criteria

### Tests That Must Pass

1. `cargo test -p starbridge_crossing --test replay`
2. `cargo test -p starbridge_crossing --test serialization && cargo test -p starbridge_crossing --test property`
3. `cargo test -p starbridge_crossing`

### Invariants

1. Replay/hash/serialization are deterministic and versioned across `{2,3,4,6}` (§2/§11).
2. No hidden/private class appears in any export (perfect information; ADR 0004 N/A).

## Test Plan

### New/Modified Tests

1. `games/starbridge_crossing/tests/replay.rs` — command-stream + seed + variant hash reproduction.
2. `games/starbridge_crossing/tests/serialization.rs` + `tests/property.rs` — round-trips + invariant properties.
3. `games/starbridge_crossing/tests/golden_traces/*.trace.json` — the §7 minimum trace set.

### Commands

1. `cargo test -p starbridge_crossing --test replay`
2. `cargo test -p starbridge_crossing`
3. `replay-check`/`fixture-check` CLI validation lands with their registration (013); native tests are the correct boundary at this ticket.

## Outcome

Completed: 2026-06-27

What changed:

- Added `games/starbridge_crossing/src/replay_support.rs` with deterministic
  replay through the same Rust command validators and stable hashes for state,
  effects, action tree, public view, and replay summary.
- Added `games/starbridge_crossing/tests/replay.rs`,
  `tests/serialization.rs`, and extended `tests/property.rs` for replay hash
  reproduction, serialization stability, all-public view stability, fixture/
  trace receipt coverage, one-occupant-per-space, and legal-step properties.
- Added four setup fixture JSON receipts under
  `games/starbridge_crossing/data/fixtures/`.
- Added the Gate 20 golden trace receipt catalog under
  `games/starbridge_crossing/tests/golden_traces/` for setup, step, hop,
  jump-chain diagnostics, blocked pass, finish/terminal, turn limit, no-leak
  parity, and public replay round-trip coverage.
- Updated `games/starbridge_crossing/src/lib.rs` exports for replay support.

Deviations from plan:

- CLI `replay-check` and `fixture-check` registration remains deferred to
  GAT20STACROSTA-013 as scoped. The trace files here are versioned coverage
  receipts backed by native tests, not yet tool-ingested replay transcripts.

Verification:

- `cargo test -p starbridge_crossing --test replay` passed: 2 integration
  tests.
- `cargo test -p starbridge_crossing --test serialization` passed: 4
  integration tests.
- `cargo test -p starbridge_crossing --test property` passed: 3 integration
  tests.
- `cargo test -p starbridge_crossing` passed: 22 unit tests, 28 integration
  tests, 0 doctests.
- `cargo fmt --all --check` passed.
- `bash scripts/boundary-check.sh` passed (`engine-core boundary check passed`;
  `game-test-support dev-only boundary check passed`).
- `git diff --check` passed.

Unrelated worktree changes left untouched:

- `.claude/skills/spec-to-tickets/SKILL.md`

# GAT101PLATRI-014: Native tool registration, RULE-COVERAGE.md, and gate-1 native CI steps

**Status**: PENDING
**Priority**: HIGH
**Effort**: Medium
**Engine Changes**: Yes — modifies `tools/{simulate,replay-check,fixture-check,rule-coverage}/src/main.rs`, `.github/workflows/gate-1-game-smoke.yml`; new `games/plain_tricks/docs/RULE-COVERAGE.md`. No `engine-core`/`game-stdlib` change.
**Deps**: GAT101PLATRI-013

## Problem

The four tools that register all games — `simulate`, `replay-check`, `fixture-check`, `rule-coverage` — need a `plain_tricks` arm, the game needs `RULE-COVERAGE.md` (consumed by `rule-coverage`), and `gate-1-game-smoke.yml` needs per-game native steps. This wires native verification into CI.

## Assumption Reassessment (2026-06-09)

1. `tools/simulate/src/main.rs`, `tools/replay-check/src/main.rs`, `tools/fixture-check/src/main.rs`, `tools/rule-coverage/src/main.rs` each register all nine current games (`race_to_n`…`poker_lite`) and need a `plain_tricks` arm; `.github/workflows/gate-1-game-smoke.yml` has per-game simulate/replay-check/fixture-check/rule-coverage steps (poker_lite at lines ~54/82/109/136). `tools/rule-coverage` reads `RULES.md` + `RULE-COVERAGE.md` (+`BENCHMARKS.md`).
2. Spec §4 (tool registration; "verified against current registries") and §7 fix the commands: `simulate --game plain_tricks --games 1000 --start-seed 0 --action-cap 32`; `replay-check --game plain_tricks` (`--all` is the default mode); `fixture-check --game plain_tricks`; `rule-coverage --game plain_tricks`. `simulate` flags `--games/--start-seed/--action-cap` exist. **Action-cap must be ≥ 25** (a 24-play match needs headroom; `simulate` checks terminal at the iteration after the final apply — a cap of 24 false-fails), per the spec §7 note; CI uses a cap above 24.
3. Shared boundary under audit: the per-tool game-id registration surface (a `RegisteredGame`/match-arm enum per tool). `seed-reducer`/`trace-viewer` register only `race_to_n`+`directional_flip` and are NOT expected to need `plain_tricks` (poker_lite precedent). `bench-report` registration is in GAT101PLATRI-015.
4. FOUNDATIONS §6/§11 (official games carry simulations, rule coverage, replay, fixtures) is under audit — this ticket wires those native gates.
5. Enforcement surface: deterministic replay/hash (`replay-check` is the deterministic replay gate). Registering the arm must exercise the GAT101PLATRI-011 traces; no new leak/determinism path is introduced. Note `RULE-COVERAGE.md` here references `BENCHMARKS.md` which lands in GAT101PLATRI-015 — expect a partial-green `rule-coverage --game plain_tricks` window until 015 lands (flagged, not a hard Dep, since 014/015 are parallel siblings off 013).

## Architecture Check

1. Per-tool match arms + per-game CI steps (the established pattern) keep native verification uniform across games; co-locating `RULE-COVERAGE.md` with the `rule-coverage` registration ensures the tool has a valid doc to check.
2. No backwards-compatibility aliasing/shims.
3. `engine-core` untouched; no `game-stdlib` change; tools consume the public crate surface only.

## Verification Layers

1. `plain_tricks` arm present in all four tools -> codebase grep-proof on each `tools/*/src/main.rs`.
2. Simulation completes 1000 matches to terminal -> simulation/CLI run (`simulate --game plain_tricks --games 1000 --action-cap 32`).
3. Replay/fixture/rule-coverage pass -> `replay-check` / `fixture-check` / `rule-coverage` CLI runs.
4. CI wiring -> manual review of `gate-1-game-smoke.yml` per-game steps.

## What to Change

### 1. Tool registration

Add a `plain_tricks` arm to `tools/simulate`, `tools/replay-check`, `tools/fixture-check`, `tools/rule-coverage` (`main.rs`), including a `run_one_plain_tricks_game` playout driver in `simulate` using the Level 2 bot, with the action-cap ≥ 25 discipline.

### 2. `games/plain_tricks/docs/RULE-COVERAGE.md`

Author the rule-coverage matrix mapping every `RULES.md` rule ID to its covering test(s), with no silent gaps.

### 3. `.github/workflows/gate-1-game-smoke.yml`

Add `plain_tricks` simulate (cap > 24) / replay-check / fixture-check / rule-coverage steps mirroring the poker_lite steps.

## Files to Touch

- `tools/simulate/src/main.rs` (modify)
- `tools/replay-check/src/main.rs` (modify)
- `tools/fixture-check/src/main.rs` (modify)
- `tools/rule-coverage/src/main.rs` (modify)
- `games/plain_tricks/docs/RULE-COVERAGE.md` (new)
- `.github/workflows/gate-1-game-smoke.yml` (modify)

## Out of Scope

- `bench-report` registration, benchmarks, `BENCHMARKS.md`, gate-2 CI (GAT101PLATRI-015).
- `seed-reducer` / `trace-viewer` (not expected to need `plain_tricks`).
- WASM / web / e2e (GAT101PLATRI-016/017/018).

## Acceptance Criteria

### Tests That Must Pass

1. `cargo run -p simulate -- --game plain_tricks --games 1000 --start-seed 0 --action-cap 32` completes all matches to terminal.
2. `cargo run -p replay-check -- --game plain_tricks`, `cargo run -p fixture-check -- --game plain_tricks`, `cargo run -p rule-coverage -- --game plain_tricks` pass (rule-coverage fully green once GAT101PLATRI-015 lands `BENCHMARKS.md`).
3. `cargo test --workspace` passes.

### Invariants

1. Simulation never reports a false action-cap failure (cap ≥ 25 for a 24-play match) (spec §7).
2. Every `RULES.md` rule ID has covering tests in `RULE-COVERAGE.md` (FOUNDATIONS §6; no silent gaps).

## Test Plan

### New/Modified Tests

1. `games/plain_tricks/docs/RULE-COVERAGE.md` — rule-ID → test matrix consumed by `rule-coverage`.
2. `.github/workflows/gate-1-game-smoke.yml` — native CI steps (verified by running the four CLIs locally).

### Commands

1. `cargo run -p simulate -- --game plain_tricks --games 1000 --start-seed 0 --action-cap 32`
2. `cargo run -p replay-check -- --game plain_tricks && cargo run -p fixture-check -- --game plain_tricks && cargo run -p rule-coverage -- --game plain_tricks`
3. The four-tool CLI run is the correct full-pipeline boundary for native verification; benchmark CI is GAT101PLATRI-015.

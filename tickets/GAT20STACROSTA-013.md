# GAT20STACROSTA-013: Tool registration (replay-check / fixture-check / rule-coverage) + RULE-COVERAGE.md

**Status**: PENDING
**Priority**: MEDIUM
**Effort**: Medium
**Engine Changes**: Yes (tooling + deterministic evidence) — `tools/{replay-check,fixture-check,rule-coverage}/src/main.rs` arms, `games/starbridge_crossing/docs/RULE-COVERAGE.md`
**Deps**: GAT20STACROSTA-011, GAT20STACROSTA-012

## Problem

The evidence tools `replay-check`, `fixture-check`, and `rule-coverage` carry per-game `match` arms (not generic dispatch), so Starbridge must be registered in each. This ticket adds those arms and authors `RULE-COVERAGE.md`, which `rule-coverage` validates against the rule IDs in `RULES.md`.

## Assumption Reassessment (2026-06-27)

1. **Scope correction (expand-in-place):** all three tools have per-game `match game` arms — confirmed `tools/replay-check/src/main.rs:104`/`:195`, `tools/fixture-check/src/main.rs:330`/`:494`, `tools/rule-coverage/src/main.rs:34`/`:162`. The spec's "confirm generic or register if required" resolves to **register required** in all three; the spec explicitly sanctioned this (§4), so it is decomposed against the wider surface, not flagged as a spec error.
2. `rule-coverage` reads `RULES.md` + `RULE-COVERAGE.md` + `BENCHMARKS.md`; `BENCHMARKS.md` lands with the benches ticket (GAT20STACROSTA-016), so `rule-coverage --game starbridge_crossing` is **partial-green until 016** — flagged, not a defect.
3. Cross-artifact boundary: these arms consume the traces/fixtures authored in 011 and the rule IDs in `RULES.md` (001); `replay-check`/`fixture-check` validate the catalog 011 created.
4. §2/§11 determinism motivates this ticket: registering the tools makes the deterministic replay/fixture/coverage checks runnable; the tools assert (not invent) behavior — Rust remains the authority.

## Architecture Check

1. Registering each tool arm where the spec's tool list enumerates them keeps the evidence pipeline complete; co-locating `RULE-COVERAGE.md` with the `rule-coverage` arm follows the validator-consumed-docs rule (the tool has a valid doc to check at landing).
2. No backwards-compatibility shims.
3. Tools live in `tools/*` (correct crate placement); no `engine-core`/`game-stdlib` change.

## Verification Layers

1. Replay validation -> deterministic replay-hash check: `cargo run -p replay-check -- --game starbridge_crossing --all`.
2. Fixture validation -> `cargo run -p fixture-check -- --game starbridge_crossing`.
3. Rule coverage -> `cargo run -p rule-coverage -- --game starbridge_crossing` (partial-green until 016 lands `BENCHMARKS.md`).
4. Doc-tool integrity -> manual review: `RULE-COVERAGE.md` maps every `RULES.md` rule ID.

## What to Change

### 1. Add per-game arms

`tools/replay-check/src/main.rs`, `tools/fixture-check/src/main.rs`, `tools/rule-coverage/src/main.rs`: register `"starbridge_crossing"` (RegisteredGame entry + dispatch) mirroring the meldfall arms.

### 2. Author `games/starbridge_crossing/docs/RULE-COVERAGE.md`

Map each `RULES.md` rule/scoring/terminal ID to its covering test/trace.

## Files to Touch

- `tools/replay-check/src/main.rs` (modify)
- `tools/fixture-check/src/main.rs` (modify)
- `tools/rule-coverage/src/main.rs` (modify)
- `games/starbridge_crossing/docs/RULE-COVERAGE.md` (new)

## Out of Scope

- `BENCHMARKS.md` (GAT20STACROSTA-016) — `rule-coverage` stays partial-green until then.
- The `simulate` arm (GAT20STACROSTA-012) and WASM registration (GAT20STACROSTA-014).

## Acceptance Criteria

### Tests That Must Pass

1. `cargo run -p replay-check -- --game starbridge_crossing --all`
2. `cargo run -p fixture-check -- --game starbridge_crossing`
3. `cargo run -p rule-coverage -- --game starbridge_crossing` (coverage complete once 016 lands `BENCHMARKS.md`)

### Invariants

1. All three tools recognize `starbridge_crossing` and validate its evidence deterministically.
2. `RULE-COVERAGE.md` leaves no `RULES.md` rule ID unmapped.

## Test Plan

### New/Modified Tests

1. `None — tool-registration + game-local doc; verification is the CLI runs above against the 011 trace/fixture catalog.`

### Commands

1. `cargo run -p replay-check -- --game starbridge_crossing --all`
2. `cargo run -p fixture-check -- --game starbridge_crossing && cargo run -p rule-coverage -- --game starbridge_crossing`
3. CLI validation is the correct boundary — these tools are the evidence harness; no new unit test is warranted for match-arm registration.

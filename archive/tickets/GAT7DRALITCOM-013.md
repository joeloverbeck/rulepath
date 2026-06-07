# GAT7DRALITCOM-013: Rust test suite (rules, property, visibility, bots)

**Status**: COMPLETED
**Priority**: HIGH
**Effort**: Medium
**Engine Changes**: Yes — `games/draughts_lite/tests/{rules.rs,property.rs,visibility.rs,bots.rs}` (consolidated rule/property/visibility/bot test coverage).
**Deps**: 006, 007, 009, 012

## Problem

An official game is evidence-heavy (FOUNDATIONS §6/§11): it needs rule, property, visibility, and bot tests covering the rule clauses and invariants. This ticket consolidates the test suite, expanding the inline/seed tests from earlier tickets into the full `tests/` coverage — including the property tests that must NOT assume termination (draughts can cycle when draw adjudication is omitted), asserting board invariants instead.

## Assumption Reassessment (2026-06-07)

1. `games/draughts_lite/src/{rules.rs,actions.rs}` (GAT7DRALITCOM-005/006/007), `visibility.rs`/`ui.rs` (009), and `bots.rs` (012) supply the surfaces under test; `games/directional_flip/tests/{rules.rs,property.rs,visibility.rs,bots.rs}` are the structural precedents. Earlier tickets seeded inline tests; this ticket is the consolidated suite (the `directional_flip` precedent moved inline tests into `tests/` similarly).
2. The required coverage is fixed by spec §R18 "Rule tests" (dark-square play, setup, men forward-only, king any-diagonal, captures, mandatory capture, no maximum-capture, multi-jump continuation, same-piece-only, no double-capture, promotion, promotion-during-capture stop, both terminal wins, perfect-info equivalence) and §R18 "Property tests" (every tree leaf validates; validated actions apply without panic; board invariants preserved; no off-board/non-playable/overlapping pieces; piece counts change only by capture; promotion only on the king row; no segment after promotion-during-capture; bounded random sims terminate OR stop at an action cap as a nonterminal smoke result — NOT a false failure).
3. Cross-artifact boundary under audit: this suite is the evidence layer for rules/action-tree/visibility/bots; it must align with `games/draughts_lite/docs/RULES.md` (001) clause-for-clause (the rule-coverage tool in 017 maps clauses → these tests).
4. FOUNDATIONS §6/§11 motivate this ticket: restate before coding — tests/traces/sims/benchmarks/docs cover the change; property tests prove invariants rather than guaranteed termination, because asserting termination would be a false failure under the omitted draw adjudication (spec §R18).

## Architecture Check

1. A property suite that asserts board invariants under bounded random play (rather than termination) correctly models a draw-adjudication-free ruleset — it catches real bugs without flagging legal cycles as failures.
2. No backwards-compatibility shims; consolidates/expands existing seeded tests with no aliasing.
3. `engine-core` stays noun-free (§3); tests are game-local. `game-stdlib` is exercised only through the rules core if a helper was promoted (§4).

## Verification Layers

1. Rule clauses -> rule tests (`tests/rules.rs`): every §R18 rule case passes.
2. Invariants under random play -> property tests (`tests/property.rs`): board invariants hold; no panic; piece counts change only by capture; bounded sims either terminate or stop at the action cap as a nonterminal result.
3. Perfect-information -> visibility test (`tests/visibility.rs`): public == private; no leak.
4. Bot legality -> bot tests (`tests/bots.rs`): Level 0/1 selection legality + heuristic preference + deterministic seed.

## What to Change

### 1. Rule + visibility tests

In `tests/rules.rs` and `tests/visibility.rs`, cover the §R18 rule clauses and the perfect-information equivalence / no-leak assertions.

### 2. Property + bot tests

In `tests/property.rs`, assert the §R18 invariant set under bounded random play with an action cap (no termination assertion). In `tests/bots.rs`, cover Level 0/1 legality, heuristic preference, and deterministic-seed behavior.

## Files to Touch

- `games/draughts_lite/tests/rules.rs` (new)
- `games/draughts_lite/tests/property.rs` (new)
- `games/draughts_lite/tests/visibility.rs` (new)
- `games/draughts_lite/tests/bots.rs` (new)

## Out of Scope

- Golden trace / fixture files (GAT7DRALITCOM-014).
- Benchmarks (GAT7DRALITCOM-015) and the `rule-coverage` tool mapping (GAT7DRALITCOM-017).
- WASM/browser tests (GAT7DRALITCOM-016/019).

## Acceptance Criteria

### Tests That Must Pass

1. `cargo test -p draughts_lite` — full rule/property/visibility/bot suite passes.
2. `bash scripts/boundary-check.sh` — `engine-core` stays noun-free.

### Invariants

1. Property tests assert board invariants, not termination (FOUNDATIONS §6/§11; spec §R18).
2. Every documented rule clause maps to a passing test (FOUNDATIONS §6; consumed by GAT7DRALITCOM-017).

## Test Plan

### New/Modified Tests

1. `games/draughts_lite/tests/{rules,property,visibility,bots}.rs` — the consolidated suite per §R18.

### Commands

1. `cargo test -p draughts_lite`
2. `cargo test --workspace && bash scripts/boundary-check.sh`
3. Crate + workspace tests are the correct boundary; clause-to-coverage reporting is produced by `tools/rule-coverage` (GAT7DRALITCOM-017).

## Outcome

Added the consolidated Draughts Lite integration test suite across
`tests/rules.rs`, `tests/property.rs`, `tests/visibility.rs`, and `tests/bots.rs`.
The suite covers setup/playable cells, men/kings, capture restrictions,
multi-jump continuation, no maximum-capture rule, promotion and promotion-stop,
terminal wins, board invariants under bounded random legal play, perfect-info
view/no-leak checks, and Level 0/1 bot legality/determinism. The property suite
uses an action cap as a nonterminal smoke boundary and does not assert game
termination.

Verification passed:

1. `cargo test -p draughts_lite`
2. `cargo fmt --all --check`
3. `cargo test --workspace`
4. `bash scripts/boundary-check.sh`

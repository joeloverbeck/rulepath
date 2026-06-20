# GAT16BRICIRTRI-011: Bots (L0 + bounded L1), simulator dispatch, and seeded simulation

**Status**: PENDING
**Priority**: MEDIUM
**Effort**: Medium
**Engine Changes**: Yes — `games/briar_circuit/src/bots.rs`, `tools/simulate/src/main.rs` (dispatch arm)
**Deps**: 007, 008, 009

## Problem

Briar Circuit needs a required L0 random-legal bot and a bounded L1 rule-informed policy, both using only the authorized seat view, public history, legal actions, and declared bot RNG — with viewer-safe explanations. This ticket also wires the `simulate` dispatch arm so a seeded 1,000-match four-seat smoke can prove zero illegal actions, panics, impossible phases, or unauthorized bot inputs. L2 is not admitted.

## Assumption Reassessment (2026-06-20)

1. `games/briar_circuit/src/{rules,scoring,visibility}.rs` provide legal actions, outcomes, and seat-private projections after GAT16BRICIRTRI-007/008/009; `bots.rs` was stubbed in 004. `tools/simulate/src/main.rs` registers games as string-const dispatch branches (e.g. `run_plain_tricks_simulation`) and already parses `--game`/`--seat-count`/`--games`/`--start-seed`/`--action-cap` — this ticket adds the `briar_circuit` branch.
2. `specs/gate-16-briar-circuit-trick-taking.md` §4.2 (Bots row), §3.2, Appendix A `BC-BOT-001/002`, and Appendix C (L0 contract, bounded L1 priorities, L2 gate) fix the behavior; `ai-core` supplies bot infrastructure. The L1 priorities are an original bounded authored policy (not copied strategy prose).
3. Cross-artifact boundary under audit: bots consume only the seat-private projection + public history + Rust legal actions (the same legal-action API humans use); bot candidate rankings/explanations are private to authorized bot/test surfaces and never enter observer export.
4. FOUNDATIONS §8 (public bots, not research AI) and §11 are under audit: deterministic under declared inputs; uses the normal legal action API; mutates no state directly; uses no hidden information. Public v1/v2 exclude MCTS/ISMCTS/Monte Carlo/ML/RL (spec §3.3 / §9).
5. No-leak applies to bot inputs/explanations: L0 exposes only random-legal metadata, L1 only public/own-hand facts; an explanation must never say or imply an opponent's holdings. `tests/bots.rs` asserts no hidden-state-derived feature is consumed.

## Architecture Check

1. Both bots routing through the Rust legal-action API (over any direct state access) guarantees fairness and keeps bot decisions in Rust (§2/§8); the L1 policy is a documented priority vector with deterministic tie-breaks.
2. No backwards-compatibility aliasing/shims — fills the `bots.rs` stub; adds an additive `simulate` dispatch branch.
3. `engine-core` untouched (§3); no `game-stdlib` bot helper (§4); no forbidden search/sampling/learning method.

## Verification Layers

1. L0/L1 choose only legal actions; same declared inputs+seed yield the same choice -> `tests/bots.rs` (`BC-BOT-001/002`).
2. Bots consume no hidden field; explanations carry authorized facts only -> `tests/bots.rs` input-audit + no-leak check.
3. Seeded 1,000-match four-seat run: zero illegal actions/panics/impossible phases/duplicate cards/unauthorized inputs; moon + threshold/tie counters reported -> `simulate` CLI run.
4. No forbidden algorithm present -> grep-proof / manual review against §8/§9.

## What to Change

### 1. `games/briar_circuit/src/bots.rs`

L0 random-legal (uniform over the Rust legal leaf set, declared bot RNG, viewer-safe "Random legal choice from N actions" explanation) and bounded L1 (pass/void-discard/follow-suit/lead priorities per Appendix C.2, deterministic card-ID tie-breaks, viewer-safe own-feature explanations).

### 2. `tools/simulate/src/main.rs`

Add the `briar_circuit` dispatch branch requiring `--seat-count 4`, preserving seat-keyed deterministic summaries and reporting threshold/tie/moon terminal reasons.

## Files to Touch

- `games/briar_circuit/src/bots.rs` (modify; created by 004)
- `games/briar_circuit/tests/bots.rs` (new)
- `tools/simulate/src/main.rs` (modify)

## Out of Scope

- `replay-check`/`fixture-check`/`rule-coverage` registration and `ci/games.json` (GAT16BRICIRTRI-012).
- Bot-strategy/competent-player evidence docs (GAT16BRICIRTRI-017).
- Any L2 authored policy (not admitted by this gate).

## Acceptance Criteria

### Tests That Must Pass

1. `cargo test -p briar_circuit --test bots` — legality, determinism, input-audit, and explanation no-leak.
2. `cargo run -p simulate -- --game briar_circuit --seat-count 4 --games 1000 --start-seed 1600 --action-cap 4096` — zero illegal actions/panics; moon + threshold/tie counters reported.
3. `cargo test -p briar_circuit` — full crate green.

### Invariants

1. Bots use the normal legal action API and authorized views only; no hidden state (§8/§11).
2. No MCTS/ISMCTS/Monte Carlo/ML/RL; deterministic under declared inputs (§8).

## Test Plan

### New/Modified Tests

1. `games/briar_circuit/tests/bots.rs` — L0/L1 legality, determinism, input-audit, explanation no-leak.
2. `tools/simulate/src/main.rs` — `briar_circuit` seeded-summary branch (exercised by the 1,000-match run).
3. `None` additional — bot-evidence docs are authored in GAT16BRICIRTRI-017.

### Commands

1. `cargo test -p briar_circuit --test bots`
2. `cargo run -p simulate -- --game briar_circuit --seat-count 4 --games 1000 --start-seed 1600 --action-cap 4096`
3. The seeded simulation is the correct end-to-end boundary for bot legality at scale; a cap hit must emit a reproducible failure seed, not a silent draw.

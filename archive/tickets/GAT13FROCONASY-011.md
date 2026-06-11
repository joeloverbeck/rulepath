# GAT13FROCONASY-011: Bot-strategy evidence docs and per-faction balance

**Status**: COMPLETED
**Priority**: MEDIUM
**Effort**: Medium
**Engine Changes**: None — documentation only (`games/frontier_control/docs/COMPETENT-PLAYER.md`, `games/frontier_control/docs/BOT-STRATEGY-EVIDENCE-PACK.md`)
**Deps**: GAT13FROCONASY-008, GAT13FROCONASY-009

## Problem

The gate ships per-faction Level 1 bots and must document their strategy evidence and the per-faction balance result. `COMPETENT-PLAYER.md` and `BOT-STRATEGY-EVIDENCE-PACK.md` record each faction's documented priority policy, the fixtures that exercise it, and the Level-1-vs-Level-1 balance band (Assumption A5: each faction wins roughly 35–65% on the standard map). A result outside the band triggers a constants retune before public polish.

## Assumption Reassessment (2026-06-11)

1. `templates/COMPETENT-PLAYER.md` and `templates/BOT-STRATEGY-EVIDENCE-PACK.md` are the instantiation sources; the Level 1 bot policies (GAT13FROCONASY-008) and the bot tests / `bot-vs-bot-full-game.trace.json` (GAT13FROCONASY-009) are the documented evidence. This is the trailing-placement pattern for a Level 1 (not Level 2) bot's evidence pack — the docs follow the bot tests that produce the evidence.
2. Spec §Acceptance evidence balance bullet and Assumption A5 define the band and the retune trigger; the per-faction win rates are measured by Level-1-vs-Level-1 simulation across both maps.
3. Cross-artifact boundary under audit: these docs cite the bot policies in `src/bots.rs` and the balance metrics; the authoritative per-faction win-rate numbers come from `cargo run -p simulate -- --game frontier_control` once that tool arm lands (GAT13FROCONASY-013) — until then the bot-vs-bot test harness (GAT13FROCONASY-009) supplies provisional measurements, refreshed at simulate registration.
4. FOUNDATIONS §8 (public bots are explainable/fair/beatable) and §6 (evidence-heavy) are under audit: the evidence pack documents authored policy and balance, not a Level 2 search claim; a Level 2 claim would require the full evidence workflow (A9), which this gate does not make.

## Architecture Check

1. Documenting the balance band as a measured, retune-on-miss target (vs asserting balance) keeps the asymmetry honest and gives a reviewable acceptance surface; the band, not exact constants, is the requirement (A5).
2. No backwards-compatibility aliasing/shims.
3. `engine-core`/`game-stdlib` untouched; docs are game-local.

## Verification Layers

1. Balance band (§8/A5) -> simulation/CLI run (Level-1-vs-Level-1 per-faction win rates within band; recorded with the measuring command).
2. Policy-evidence fidelity -> manual review (docs match the `src/bots.rs` priority policies) + doc-link check.

## What to Change

### 1. COMPETENT-PLAYER.md

Instantiate from template; record each faction's competent-play heuristics and the balance band + retune posture.

### 2. BOT-STRATEGY-EVIDENCE-PACK.md

Instantiate from template; document each Level 1 policy, the fixtures/traces exercising it, and the per-faction win-rate evidence.

## Files to Touch

- `games/frontier_control/docs/COMPETENT-PLAYER.md` (new)
- `games/frontier_control/docs/BOT-STRATEGY-EVIDENCE-PACK.md` (new)

## Out of Scope

- Bot code (GAT13FROCONASY-008) and bot tests (GAT13FROCONASY-009).
- `simulate` registration (GAT13FROCONASY-013); a Level 2 bot claim (not made this gate).

## Acceptance Criteria

### Tests That Must Pass

1. `node scripts/check-doc-links.mjs` passes with both docs in place.
2. The documented per-faction win rates fall in the A5 band, or a recorded retune note is present.
3. Manual review confirms the docs match the implemented Level 1 policies.

### Invariants

1. The evidence documents authored Level 1 policy only — no MCTS/search/Level 2 claim (§8/A9).
2. Balance is a measured, retune-on-miss target keyed to the A5 band.

## Test Plan

### New/Modified Tests

1. `None — documentation-only ticket; verification is command-based and existing pipeline/test coverage is named in Assumption Reassessment.`

### Commands

1. `node scripts/check-doc-links.mjs`
2. `cargo run -p simulate -- --game frontier_control --games 1000` (per-faction win rates, once GAT13FROCONASY-013 registers the arm; provisional via bot tests before then)
3. Doc-link + simulation is the correct boundary; the balance band is a simulation-measured acceptance surface, not a unit test.

## Outcome

Completed on 2026-06-11.

Implemented the two ticket-owned evidence documents:

- `games/frontier_control/docs/COMPETENT-PLAYER.md`
- `games/frontier_control/docs/BOT-STRATEGY-EVIDENCE-PACK.md`

The docs instantiate the strategy/evidence templates for Frontier Control's
actual Level 1 scope. They document the Garrison and Prospector policy order
from `games/frontier_control/src/bots.rs`, cite the existing bot/property trace
evidence, keep the hidden-information boundary explicit, and state that no
Level 2, search, MCTS/ISMCTS, Monte Carlo, ML, RL, or runtime LLM bot claim is
made.

Deviation from the plan: the registered `simulate` CLI arm is not yet available
until GAT13FROCONASY-013, so the A5 balance evidence is recorded as a retune
note instead of an in-band balance claim. A temporary local probe against the
current Rust Level 1 bot APIs found standard map Garrison 16-0 and highlands
Prospectors 15-3; the docs require constants or policy retuning before public
polish if the later `simulate` measurement confirms an out-of-band standard-map
result.

Verification:

- `node scripts/check-doc-links.mjs` passed (`Checked 25 markdown files`).
- `cargo test -p frontier_control bots` passed (4 bot-focused tests across unit
  and integration targets).
- `cargo test -p frontier_control level1_bot_sequence_reaches_terminal_without_illegal_actions`
  passed.

# GAT14EVEFROEVE-013: Bot-strategy evidence docs and per-faction balance

**Status**: PENDING
**Priority**: MEDIUM
**Effort**: Medium
**Engine Changes**: None — documentation only (`games/event_frontier/docs/COMPETENT-PLAYER.md`, `games/event_frontier/docs/BOT-STRATEGY-EVIDENCE-PACK.md`)
**Deps**: GAT14EVEFROEVE-010, GAT14EVEFROEVE-011

## Problem

The gate ships per-faction Level 1 scripted bots and must document their strategy evidence and the balance result. `COMPETENT-PLAYER.md` and `BOT-STRATEGY-EVIDENCE-PACK.md` record each faction's documented decision table and site-priority lists, the fixtures that exercise them, and the Level-1-vs-Level-1 balance band (Assumption A5: each faction wins roughly 35–65% on the standard scenario) **and** the victory-type mix in which all three victory types occur in a 1,000-game simulation. A miss triggers a constants retune recorded here before public polish.

## Assumption Reassessment (2026-06-12)

1. The bots and their tests this documents exist: verified ticket 010's `EventCharterLevel1Bot`/`EventFreeholdersLevel1Bot` with the two-layer policy, and ticket 011's `tests/bots.rs` decision-table-conformance and balance evidence harness. The balance/victory-type metrics are producible from the bot test/simulation harness (the simulate CLI registration in ticket 015 provides the closeout 1,000-game run).
2. The balance target and victory-type requirement are specified: verified the spec's Assumption A5 (35–65% per faction) and the requirement that all three victory types (Charter instant, Freeholder instant, final fallback) occur in a 1,000-game simulation; misses trigger a constants retune recorded in `COMPETENT-PLAYER.md`/`BENCHMARKS.md`.
3. Cross-artifact boundary under audit: these are evidence docs for a Level 1 bot, so they trail the bot (ticket 010) and its tests (ticket 011) — the evidence is produced by the bot fixtures, so the docs follow them (not a Level-2 pre-authored policy). The documented decision tables must match the implemented `bots.rs` policy exactly.
4. FOUNDATIONS §8 (public bots competent, explainable, fair, beatable) and §6 (evidence-heavy) motivate this ticket. Restated before trusting the spec: the docs record deterministic priority policies with total-order tiebreaks and viewer-safe explanations; no MCTS/ISMCTS/ML/RL, no Level 2 claim without the full evidence-pack workflow (already this doc).

## Architecture Check

1. Trailing the bot with the evidence pack (rather than pre-authoring it) is correct for a Level 1 policy: the documented tables are transcribed from the implemented bot and verified against its conformance tests, so the docs cannot drift ahead of the code.
2. No backwards-compatibility aliasing/shims — new docs.
3. `engine-core` stays noun-free; no `game-stdlib` promotion; documentation only.

## Verification Layers

1. Decision-table fidelity -> manual review that the documented Layer-1/Layer-2 tables match `games/event_frontier/src/bots.rs` exactly; cross-check against `tests/bots.rs` conformance cases.
2. Balance evidence -> the recorded per-faction win band (35–65%) and the victory-type distribution (all three occur) cite the simulation run; a miss records the retune.
3. Doc-link integrity -> `node scripts/check-doc-links.mjs`.
4. Single-doc-pair ticket: documentation only; no cross-crate proof surface beyond the above.

## What to Change

### 1. COMPETENT-PLAYER.md

Instantiate from `templates/COMPETENT-PLAYER.md`; describe competent play for each faction, the Level 1 bot's strategy, the balance band result, and any constants retune.

### 2. BOT-STRATEGY-EVIDENCE-PACK.md

Instantiate from `templates/BOT-STRATEGY-EVIDENCE-PACK.md`; record each faction's documented decision table and site priorities, the fixtures exercising them, the per-faction win rates, and the victory-type frequencies (Charter instant / Freeholder instant / final fallback).

## Files to Touch

- `games/event_frontier/docs/COMPETENT-PLAYER.md` (new)
- `games/event_frontier/docs/BOT-STRATEGY-EVIDENCE-PACK.md` (new)

## Out of Scope

- The bot implementation (ticket 010) and bot tests (ticket 011).
- The simulate CLI registration (ticket 015) — this ticket cites the simulation metrics; the CLI lane is registered there.
- Any Level 2 bot claim or search/learning method.

## Acceptance Criteria

### Tests That Must Pass

1. `node scripts/check-doc-links.mjs` passes with both docs present.
2. The documented decision tables match `games/event_frontier/src/bots.rs` (manual cross-check against `tests/bots.rs`).
3. The recorded balance band and victory-type mix cite a reproducible simulation run.

### Invariants

1. The evidence pack documents deterministic priority policies with total-order tiebreaks; no search/learning method appears.
2. The balance result is inside Assumption A5's band or records the retune that brought it in.

## Test Plan

### New/Modified Tests

1. `None — documentation-only ticket; balance evidence is produced by ticket 011's bot/simulation harness and the closeout 1,000-game run (ticket 015 simulate CLI).`

### Commands

1. `node scripts/check-doc-links.mjs`
2. `cargo test -p event_frontier --test bots` (re-confirm the documented tables match the conformance tests)
3. The doc-link check plus a bots-test re-run is the correct boundary — the docs transcribe verified behavior; the full 1,000-game metric run is exercised at the simulate-CLI registration (ticket 015).

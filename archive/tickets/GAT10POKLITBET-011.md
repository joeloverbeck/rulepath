# GAT10POKLITBET-011: Bot-strategy evidence docs (AI, competent-player, evidence pack)

**Status**: ✅ COMPLETED
**Priority**: MEDIUM
**Effort**: Medium
**Engine Changes**: None — new per-game bot docs only (`games/poker_lite/docs/*`); no Rust/engine surface touched
**Deps**: GAT10POKLITBET-010

## Problem

The Level 2 authored policy requires its strategy documented and evidenced: `AI.md` (bot levels + policy description), `COMPETENT-PLAYER.md` (what competent play looks like), and `BOT-STRATEGY-EVIDENCE-PACK.md` (evidence the bot is competent, fair, and beatable). These docs depend on the bot's tests/fixtures existing, so they trail GAT10POKLITBET-010.

## Assumption Reassessment (2026-06-08)

1. The doc set matches `games/secret_draft/docs/` (`AI.md`, `COMPETENT-PLAYER.md`, `BOT-STRATEGY-EVIDENCE-PACK.md`) instantiated from `templates/GAME-AI.md`, `templates/COMPETENT-PLAYER.md`, `templates/BOT-STRATEGY-EVIDENCE-PACK.md` (all verified present this session). The bot whose behavior they document was authored in GAT10POKLITBET-010.
2. The spec (`specs/gate-10-poker-lite-betting-showdown.md` §4 per-game docs, §5 WB8, §8 alignment) makes `BOT-STRATEGY-EVIDENCE-PACK.md` and `COMPETENT-PLAYER.md` mandatory because the gate ships a Level 2 bot; the policy id and heuristic priorities are fixed in §C.
3. Cross-artifact boundary under audit: the evidence cited here (deterministic-under-inputs, beatable, no hidden-state) is produced by `games/poker_lite/tests/bots.rs` and the `bot-action` golden trace (GAT10POKLITBET-010). Docs reference those real fixtures; they author no behavior.
4. FOUNDATIONS §8 (public bots are competent, explainable, fair, deterministic, beatable; no MCTS/ISMCTS/ML/RL) motivates this ticket. Restated: the evidence pack documents that the §C policy meets §8, citing the bot tests as proof — not a new capability.

## Architecture Check

1. Placing bot-evidence docs after the bot (trailing, `Deps: 010`) lets them cite real test fixtures rather than aspirational behavior — the evidence pack is only meaningful once the bot tests pass. Matches the Level-1/Level-2 trailing-evidence placement.
2. No backwards-compatibility aliasing/shims — new docs.
3. `engine-core` untouched (§3); no `game-stdlib` promotion (§4); prose only.

## Verification Layers

1. Doc-link integrity -> `node scripts/check-doc-links.mjs`.
2. Evidence fidelity (claims map to real bot tests/traces) -> manual review cross-referencing `tests/bots.rs` + `bot-action.trace.json`.
3. §8 alignment (documents deterministic/fair/beatable/no-solver) -> FOUNDATIONS alignment check against §8.
4. Single-artifact-class ticket (docs only); the layers above are the applicable proof surfaces.

## What to Change

### 1. `games/poker_lite/docs/AI.md`

Instantiate from `templates/GAME-AI.md`. Describe Level 0 random-legal and Level 2 `poker-lite-crest-ledger-level2-v1`: allowed/forbidden inputs, heuristic priority order, stable tie-break, viewer-safe explanation policy.

### 2. `games/poker_lite/docs/COMPETENT-PLAYER.md`

Instantiate from `templates/COMPETENT-PLAYER.md`. Describe competent Crest Ledger play (pledge discipline, when to press/lift/match/yield given own strength and public price) the bot approximates.

### 3. `games/poker_lite/docs/BOT-STRATEGY-EVIDENCE-PACK.md`

Instantiate from `templates/BOT-STRATEGY-EVIDENCE-PACK.md`. Cite the bot tests/traces as evidence the Level 2 policy is competent, fair, deterministic under declared inputs, hidden-state-free, and beatable.

## Files to Touch

- `games/poker_lite/docs/AI.md` (new)
- `games/poker_lite/docs/COMPETENT-PLAYER.md` (new)
- `games/poker_lite/docs/BOT-STRATEGY-EVIDENCE-PACK.md` (new)

## Out of Scope

- Bot code and tests (GAT10POKLITBET-010).
- `MECHANICS.md`, `UI.md`, `PUBLIC-RELEASE-CHECKLIST.md` (GAT10POKLITBET-017), `RULE-COVERAGE.md` (012), `BENCHMARKS.md` (013), `PRIMITIVE-PRESSURE-LEDGER.md` (018).

## Acceptance Criteria

### Tests That Must Pass

1. `node scripts/check-doc-links.mjs` passes with the three new docs.
2. Every competence/fairness/beatability claim maps to a named test or trace in GAT10POKLITBET-010 — manual checklist.
3. Docs state the no-solver / no-hidden-state posture explicitly (§8).

### Invariants

1. The evidence pack cites only real, passing bot fixtures — no aspirational claims.
2. No doc implies the bot uses hidden state or a search/learning class (§8).

## Test Plan

### New/Modified Tests

1. `None — documentation-only ticket; verification is `check-doc-links` plus manual evidence-fidelity review against GAT10POKLITBET-010's bot tests/traces.`

### Commands

1. `node scripts/check-doc-links.mjs`
2. `cargo test -p poker_lite --test bots` — confirms the cited evidence fixtures pass (the docs' factual basis).
3. Narrower boundary: docs-only, so no new Rust surface; the bot-test command above is the evidence anchor.

## Outcome

Completed on 2026-06-09.

Changed:

- Added per-game AI registry, competent-player analysis, and bot strategy evidence pack for Crest Ledger.
- Cited only shipped bot tests and `bot-action.trace.json` for competence, fairness, determinism, and no-leak claims.

Deviations:

- None.

Verification:

- `node scripts/check-doc-links.mjs`
- `cargo test -p poker_lite --test bots`

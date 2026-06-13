# PHA0NEXPHAFOU-012: Templates — bot cluster multi-opponent fields

**Status**: COMPLETED
**Priority**: MEDIUM
**Effort**: Medium
**Engine Changes**: None — `templates/GAME-AI.md`, `BOT-STRATEGY-EVIDENCE-PACK.md`, `COMPETENT-PLAYER.md` edits only.
**Deps**: PHA0NEXPHAFOU-002, PHA0NEXPHAFOU-006

## Problem

The bot templates assume a single opponent. They have no multi-opponent belief/policy structure, no supported-seat-range field, no per-viewer explanation-redaction field, and no explicit "no MCTS/Monte Carlo/ML/RL" assertion — all of which an N-player bot's evidence must carry before the first 3+ seat hidden-info game.

## Assumption Reassessment (2026-06-13)

1. No code change. `templates/` holds `GAME-AI.md`, `BOT-STRATEGY-EVIDENCE-PACK.md`, `COMPETENT-PLAYER.md` (verified via `ls templates/`).
2. Docs: the doctrine these templates mirror lives in `docs/AI-BOTS.md` (the N-player imperfect-information subsection added in PHA0NEXPHAFOU-006); `docs/MULTI-SEAT-AND-SURFACE-CONTRACT.md` (PHA0NEXPHAFOU-002) is the cross-reference target.
3. Cross-artifact boundary under audit: the bot template set; shared surface = the multi-opponent policy fields that mirror the AI-BOTS subsection.
4. FOUNDATIONS principle restate: §8 (public bots exclude MCTS/ISMCTS/Monte Carlo/ML/RL) and §11 (bot explanations do not leak). The template additions are meaning-preserving.
5. Enforcement surface: §11 no-leak firewall (explanation redaction per viewer) and §8 forbidden search classes. The templates add the fields that drive per-game bot evidence; they introduce no leakage path.

## Architecture Check

1. Adding multi-opponent fields makes every N-player bot's evidence pack prove seat-view-only inputs, cleaner than rediscovering the obligation per game.
2. No backwards-compatibility aliasing/shims introduced.
3. No new bot search class is introduced; the templates assert the no-MCTS/ML/RL rule (§8). `engine-core` is untouched.

## Verification Layers

1. Each bot template's new multi-opponent fields present → manual review.
2. The "no MCTS/Monte Carlo/ML/RL" assertion field present → codebase grep-proof.
3. Per-viewer explanation-redaction field present → FOUNDATIONS alignment check (§8/§11).
4. Links resolve → `node scripts/check-doc-links.mjs`.

## What to Change

### 1. `templates/GAME-AI.md`

Add rows for the seat range supported by each bot, per-seat policy specialization, bot-vs-bot orchestration for N seats, deterministic skip/advance order, and multi-winner/split metrics.

### 2. `templates/BOT-STRATEGY-EVIDENCE-PACK.md`

Add fields for supported seat range, opponent set, own-private/public-only input list, multi-opponent priority rules, deterministic tie-breakers, explanation redaction per viewer, and a "no MCTS/Monte Carlo/ML/RL" assertion.

### 3. `templates/COMPETENT-PLAYER.md`

Add "number of opponents," "partnership/team roles," "public table inference allowed," "private inference forbidden," and "kingmaking/coalition risk" sections.

## Files to Touch

- `templates/GAME-AI.md` (modify)
- `templates/BOT-STRATEGY-EVIDENCE-PACK.md` (modify)
- `templates/COMPETENT-PLAYER.md` (modify)

## Out of Scope

- Editing `docs/AI-BOTS.md` (PHA0NEXPHAFOU-006).
- Editing `crates/ai-core` or any `games/*` bot code.
- The other template clusters (PHA0NEXPHAFOU-011 / PHA0NEXPHAFOU-013).

## Acceptance Criteria

### Tests That Must Pass

1. Each of the three bot templates carries its multi-opponent fields per What to Change.
2. `BOT-STRATEGY-EVIDENCE-PACK.md` carries the per-viewer redaction field and the explicit "no MCTS/Monte Carlo/ML/RL" assertion.
3. `node scripts/check-doc-links.mjs` passes.

### Invariants

1. The §8 no-MCTS/ML/RL assertion is present in the evidence-pack template.
2. The §11 explanation-redaction-per-viewer field is present; no template implies a bot may read another seat's private state.

## Test Plan

### New/Modified Tests

1. `None — documentation-only ticket; verification is command-based and existing pipeline/test coverage is named in Assumption Reassessment.`

### Commands

1. `node scripts/check-doc-links.mjs`
2. `grep -niE "seat range|MCTS|redaction|opponent set" templates/GAME-AI.md templates/BOT-STRATEGY-EVIDENCE-PACK.md templates/COMPETENT-PLAYER.md`
3. `bash scripts/boundary-check.sh`

## Outcome

Completed: 2026-06-13

Summary:

- Added supported-seat-range, opponent-set, N-seat orchestration, multi-winner/split metric, and per-viewer explanation-redaction fields to the bot template cluster.
- Added the explicit no-MCTS/Monte Carlo/ML/RL assertion to the Level 2 evidence-pack template.

Deviations:

- None.

Verification:

- `node scripts/check-doc-links.mjs`
- `grep -niE "seat range|MCTS|redaction|opponent set" templates/GAME-AI.md templates/BOT-STRATEGY-EVIDENCE-PACK.md templates/COMPETENT-PLAYER.md`
- `bash scripts/boundary-check.sh`

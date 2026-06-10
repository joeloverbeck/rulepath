# GAT11MASCLABLU-018: Player-rules generation and outcome-explanation registration

**Status**: PENDING
**Priority**: MEDIUM
**Effort**: Small
**Engine Changes**: Yes (presentation/tooling) — modifies `scripts/check-player-rules.mjs` (`HIDDEN_INFO_GAMES`); generates `apps/web/public/rules/masked_claims.md` + updates `apps/web/public/rules/manifest.json`; registers masked-claims outcome explanations (no Rust surface)
**Deps**: GAT11MASCLABLU-016, GAT11MASCLABLU-017

## Problem

The player-facing rules copy must be generated from `HOW-TO-PLAY.md` and kept in sync, `masked_claims` must be marked a hidden-information game, and the outcome-explanation templates/rule-ID mirrors must be registered so the deterministic guard scripts pass.

## Assumption Reassessment (2026-06-10)

1. `HOW-TO-PLAY.md` (GAT11MASCLABLU-016) is the generation source for `scripts/copy-player-rules.mjs` → `apps/web/public/rules/masked_claims.md`; `apps/web/public/rules/` already holds the other games' rules plus `manifest.json` (confirmed). `scripts/check-player-rules.mjs` carries `HIDDEN_INFO_GAMES` (confirmed line 27: `["high_card_duel", "secret_draft", "poker_lite", "plain_tricks"]`), which `masked_claims` joins.
2. Spec Deliverables: `masked_claims` added to `HIDDEN_INFO_GAMES`; the generated `masked_claims.md` in sync; outcome templates/rule-ID mirrors registered so `scripts/check-outcome-explanations.mjs` passes. That script reads `docs/UI.md` + `docs/RULES.md` per game (confirmed) and the `outcomeExplanationTemplates.ts` entries authored in GAT11MASCLABLU-017.
3. Cross-artifact boundary under audit: the player-rules generation contract (`copy-player-rules.mjs` + `check-player-rules.mjs`) and the outcome-explanation contract (`check-outcome-explanations.mjs` + `RULES.md` rule IDs + the TS templates). Both auto-discover per game, so the work here is registration + generation + verification, not script-logic edits beyond the `HIDDEN_INFO_GAMES` set.
4. FOUNDATIONS §7 (player-facing rules surface) and §11 (the generated copy and explanations carry no hidden information) are the principles under audit.

## Architecture Check

1. Generating the public rules from the single `HOW-TO-PLAY.md` source (rather than hand-maintaining a parallel copy) keeps them from drifting; the check script enforces sync deterministically.
2. No backwards-compatibility aliasing or shims introduced.
3. `engine-core` stays noun-free; this is presentation/tooling wiring only.

## Verification Layers

1. Player-rules in sync + hidden-info flag set -> `node scripts/check-player-rules.mjs`.
2. Outcome templates/rule-ID mirrors registered -> `node scripts/check-outcome-explanations.mjs`.
3. Generated rules file present -> `test -f apps/web/public/rules/masked_claims.md`.

## What to Change

### 1. Generate the player rules

Run `scripts/copy-player-rules.mjs` to produce `apps/web/public/rules/masked_claims.md` from `games/masked_claims/docs/HOW-TO-PLAY.md` and update `apps/web/public/rules/manifest.json`.

### 2. `scripts/check-player-rules.mjs`

Add `masked_claims` to the `HIDDEN_INFO_GAMES` set.

### 3. Outcome-explanation registration

Ensure the masked-claims outcome templates (from `outcomeExplanationTemplates.ts`, GAT11MASCLABLU-017) and the `RULES.md` rule-ID mirrors are registered so `scripts/check-outcome-explanations.mjs` passes.

## Files to Touch

- `apps/web/public/rules/masked_claims.md` (new, generated)
- `apps/web/public/rules/manifest.json` (modify, regenerated)
- `scripts/check-player-rules.mjs` (modify)

## Out of Scope

- `HOW-TO-PLAY.md` authoring (GAT11MASCLABLU-016) and the outcome-template TS authoring (GAT11MASCLABLU-017).
- Browser E2E smoke and catalog README reconciliation (GAT11MASCLABLU-019).

## Acceptance Criteria

### Tests That Must Pass

1. `node scripts/check-player-rules.mjs` passes with `masked_claims` in `HIDDEN_INFO_GAMES` and `apps/web/public/rules/masked_claims.md` in sync.
2. `node scripts/check-outcome-explanations.mjs` passes with the masked-claims templates and rule-ID mirrors registered.
3. `apps/web/public/rules/masked_claims.md` exists and matches `HOW-TO-PLAY.md`.

### Invariants

1. The generated rules copy is derived from `HOW-TO-PLAY.md`, not hand-maintained (FOUNDATIONS §7).
2. The player-rules copy and outcome explanations leak no hidden information (FOUNDATIONS §11).

## Test Plan

### New/Modified Tests

1. `None — generation + registration ticket; verification is the two deterministic guard scripts.`

### Commands

1. `node scripts/check-player-rules.mjs`
2. `node scripts/check-outcome-explanations.mjs`
3. The two guard scripts are the exact verification boundary; they fail closed if the copy is stale or a template is missing.

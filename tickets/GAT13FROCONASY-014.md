# GAT13FROCONASY-014: Player and mechanic docs (HOW-TO-PLAY, MECHANICS, UI, AI)

**Status**: PENDING
**Priority**: MEDIUM
**Effort**: Medium
**Engine Changes**: None — documentation only (`games/frontier_control/docs/{HOW-TO-PLAY,MECHANICS,UI,AI}.md`; generated `apps/web/public/rules/frontier_control.md`)
**Deps**: GAT13FROCONASY-001, GAT13FROCONASY-012

## Problem

Frontier Control needs its player-facing and mechanic docs: `HOW-TO-PLAY.md` (the player rules, with its "Hidden information and reveal timing" section explicitly **not applicable** since the game is perfect-information), `MECHANICS.md`, `UI.md` (carrying the outcome/victory-explanation section the outcome contract checks), and `AI.md`. The player rules are mirrored to `apps/web/public/rules/frontier_control.md` via `scripts/copy-player-rules.mjs` and validated by `scripts/check-player-rules.mjs` without a `HIDDEN_INFO_GAMES` registration.

## Assumption Reassessment (2026-06-11)

1. `templates/GAME-{HOW-TO-PLAY,MECHANICS,UI,AI}.md` are the instantiation sources; `scripts/copy-player-rules.mjs` copies `games/<id>/docs/HOW-TO-PLAY.md` → `apps/web/public/rules/<id>.md`; `scripts/check-player-rules.mjs` defines `HIDDEN_INFO_GAMES` (verified L27, a `Set` of the six hidden-info games) and requires the hidden-information section to be marked **not applicable** for games outside that set (verified L136-144). `RULES.md` (GAT13FROCONASY-001) is the rules source these summarize.
2. Spec §Documentation player-rules/outcome bullets define the pipeline and the not-applicable hidden-info marker; A4 establishes the perfect-information scope.
3. Cross-artifact boundary under audit: `check-player-rules.mjs` keys off the wasm-api catalog (so this ticket depends on GAT13FROCONASY-012's catalog entry) and the source `HOW-TO-PLAY.md`; `UI.md`'s `## Outcome / victory explanation` section is read by `scripts/check-outcome-explanations.mjs` (GAT13FROCONASY-016 registers the TS mirrors).
4. FOUNDATIONS §7 (public UI) and §11 are under audit: the player docs are presentation/help content; frontier_control is NOT added to `HIDDEN_INFO_GAMES` and its hidden-information section is marked not applicable (no leak, perfect information).

## Architecture Check

1. Generating the player-rules asset from the canonical `HOW-TO-PLAY.md` (vs hand-maintaining a second copy) keeps one source of truth, enforced by `check-player-rules.mjs`.
2. No backwards-compatibility aliasing/shims.
3. `engine-core`/`game-stdlib` untouched; docs + generated asset only.

## Verification Layers

1. Player-rules sync + hidden-info posture -> `node scripts/check-player-rules.mjs` (generated asset in sync; hidden-information section marked not applicable; no `HIDDEN_INFO_GAMES` entry).
2. Outcome-section presence -> codebase grep-proof (`UI.md` carries the `## Outcome / victory explanation` section the outcome contract reads).
3. Doc-link integrity -> `node scripts/check-doc-links.mjs`.

## What to Change

### 1. Author the four docs

Instantiate `HOW-TO-PLAY.md` (mark the hidden-information section not applicable), `MECHANICS.md`, `UI.md` (with the outcome/victory-explanation section), and `AI.md` from templates.

### 2. Generate the player-rules asset

Run `scripts/copy-player-rules.mjs` to produce `apps/web/public/rules/frontier_control.md` in sync with `HOW-TO-PLAY.md`.

## Files to Touch

- `games/frontier_control/docs/HOW-TO-PLAY.md` (new)
- `games/frontier_control/docs/MECHANICS.md` (new)
- `games/frontier_control/docs/UI.md` (new)
- `games/frontier_control/docs/AI.md` (new)
- `apps/web/public/rules/frontier_control.md` (new)

## Out of Scope

- The TypeScript outcome-explanation templates / rule-ID mirrors (GAT13FROCONASY-015/016).
- Admission + public-release docs (GAT13FROCONASY-017).
- React board (GAT13FROCONASY-015).

## Acceptance Criteria

### Tests That Must Pass

1. `node scripts/check-player-rules.mjs` passes with the generated asset in sync and the hidden-information section marked not applicable.
2. `node scripts/check-doc-links.mjs` passes.
3. `grep -n 'Outcome / victory explanation' games/frontier_control/docs/UI.md` resolves.

### Invariants

1. frontier_control is NOT in `HIDDEN_INFO_GAMES`; its hidden-information section is explicitly not applicable (perfect information).
2. The player-rules asset is generated from `HOW-TO-PLAY.md`, not hand-authored separately.

## Test Plan

### New/Modified Tests

1. `None — documentation-only ticket; verification is command-based and existing pipeline/test coverage is named in Assumption Reassessment.`

### Commands

1. `node scripts/check-player-rules.mjs`
2. `node scripts/check-doc-links.mjs`
3. The player-rules + doc-link checks are the correct boundary; the outcome-explanation TS check runs in GAT13FROCONASY-016 once the templates land.

# GAT20STACROSTA-018: Trailing game docs and outcome-explanation surface

**Status**: PENDING
**Priority**: LOW
**Effort**: Medium
**Engine Changes**: None — game-local docs (`games/starbridge_crossing/docs/{UI.md,COMPETENT-PLAYER.md,BOT-STRATEGY-EVIDENCE-PACK.md,PUBLIC-RELEASE-CHECKLIST.md,GAME-EVIDENCE.md}`)
**Deps**: GAT20STACROSTA-012, GAT20STACROSTA-015, GAT20STACROSTA-016

## Problem

The official-game doc set must be completed: `UI.md` (with the outcome/victory-explanation section `check-outcome-explanations` requires), `COMPETENT-PLAYER.md`, `BOT-STRATEGY-EVIDENCE-PACK.md` (L2/L3 not-started), `PUBLIC-RELEASE-CHECKLIST.md`, and `GAME-EVIDENCE.md` cross-linking the no-leak audit, traces, benchmarks, bot status, and scaffolding receipt. This closes the `check-outcome-explanations` red window opened at WASM registration.

## Assumption Reassessment (2026-06-27)

1. Sibling games carry short-named `UI.md`, `COMPETENT-PLAYER.md`, `BOT-STRATEGY-EVIDENCE-PACK.md`, `PUBLIC-RELEASE-CHECKLIST.md` and prefix-kept `GAME-EVIDENCE.md` (confirmed `games/meldfall_ledger/docs/`); the reassessed spec uses these names.
2. `scripts/check-outcome-explanations.mjs` requires four co-dependent surfaces: stable rule IDs in `RULES.md` (001), the "Outcome / victory explanation" section in `UI.md` (here), the `client.ts` rationale mirror (015), and `outcomeExplanationTemplates.ts` copy keys (015). The `UI.md` section is the last to land → closes the red window.
3. Cross-artifact boundary: `GAME-EVIDENCE.md` cross-links artifacts produced across 010 (no-leak audit), 011 (traces), 016 (benchmarks), 012 (bot status); `BOT-STRATEGY-EVIDENCE-PACK.md` documents L0 with L2/L3 marked not-started (spec §4 allows this for an L0-only gate).
4. §11 (evidence coverage) motivates this ticket: the official-game contract requires the full doc set; `GAME-EVIDENCE.md` classifies artifacts under the evidence-fixture contract (replay-command-v1, setup-evidence-v1, domain-evidence-v1, viewer-scoped public profile with `not_applicable` seat-private rationale).

## Architecture Check

1. Trailing the doc set after the surfaces it documents keeps cross-links accurate and lets `GAME-EVIDENCE.md` cite real artifacts; placing the `UI.md` outcome section here closes the outcome-explanation validator window.
2. No backwards-compatibility shims; docs only.
3. No `engine-core`/`game-stdlib` change; no legality asserted.

## Verification Layers

1. Outcome-explanation completeness -> `node scripts/check-outcome-explanations.mjs` (green; `UI.md` outcome section present).
2. Doc-set completeness (§11) -> manual review: every spec §4 doc filled or marked `not applicable`/`not started` with rationale.
3. Evidence cross-links -> `node scripts/check-doc-links.mjs` (GAME-EVIDENCE.md links resolve).
4. Bot-doc honesty -> manual review: `BOT-STRATEGY-EVIDENCE-PACK.md` L2/L3 marked not-started, not falsely claimed.

## What to Change

### 1. Author `UI.md` (incl. outcome/victory-explanation section), `COMPETENT-PLAYER.md`, `BOT-STRATEGY-EVIDENCE-PACK.md`, `PUBLIC-RELEASE-CHECKLIST.md`.

### 2. Author `GAME-EVIDENCE.md`

Cross-link the all-public no-leak audit, trace profile, benchmark receipts, bot status, and (forward-reference) the scaffolding receipt; record ADR 0004 not-applicable rationale and `not_applicable` hidden/private-field rows.

## Files to Touch

- `games/starbridge_crossing/docs/UI.md` (new)
- `games/starbridge_crossing/docs/COMPETENT-PLAYER.md` (new)
- `games/starbridge_crossing/docs/BOT-STRATEGY-EVIDENCE-PACK.md` (new)
- `games/starbridge_crossing/docs/PUBLIC-RELEASE-CHECKLIST.md` (new)
- `games/starbridge_crossing/docs/GAME-EVIDENCE.md` (new)

## Out of Scope

- The `ci/scaffolding-audits.json` receipt + register reconciliation — GAT20STACROSTA-019.
- The `specs/README.md` Done-flip — GAT20STACROSTA-020.

## Acceptance Criteria

### Tests That Must Pass

1. `node scripts/check-outcome-explanations.mjs`
2. `node scripts/check-doc-links.mjs`
3. `bash scripts/boundary-check.sh`

### Invariants

1. The official-game doc set is complete with explicit `not applicable`/`not started` rationale where used (§11).
2. `check-outcome-explanations` is green (red window closed).

## Test Plan

### New/Modified Tests

1. `None — documentation-only ticket; verification is command-based and existing pipeline/test coverage is named in Assumption Reassessment.`

### Commands

1. `node scripts/check-outcome-explanations.mjs`
2. `node scripts/check-doc-links.mjs && bash scripts/boundary-check.sh`
3. The outcome-explanation + doc-link checks are the correct boundary; docs ship no code.

# GAT19MELLEDFIV-023: Trailing docs, GAME-EVIDENCE, and Gate 19 closeout capstone

**Status**: PENDING
**Priority**: MEDIUM
**Effort**: Medium
**Engine Changes**: None — docs/status (`MECHANICS.md`, `PUBLIC-RELEASE-CHECKLIST.md`, `GAME-EVIDENCE.md`, `docs/SOURCES.md`, `specs/README.md`, spec Status)
**Deps**: GAT19MELLEDFIV-015, GAT19MELLEDFIV-017, GAT19MELLEDFIV-018, GAT19MELLEDFIV-021, GAT19MELLEDFIV-022

## Problem

Gate 19 closes with the remaining game-local docs (`MECHANICS.md`, `PUBLIC-RELEASE-CHECKLIST.md`, `GAME-EVIDENCE.md`), the global `docs/SOURCES.md` source summary, and the status reconciliation: flip the `specs/README.md` Gate 19 row to the spec path + `Done`, and flip the spec's own Status to `Done`. As a verification capstone it also confirms the spec's exit criteria / acceptance evidence are green end-to-end across the prior tickets. It introduces no production logic.

## Assumption Reassessment (2026-06-25)

1. `games/blackglass_pact/docs/{MECHANICS,PUBLIC-RELEASE-CHECKLIST,GAME-EVIDENCE}.md` are exemplars; `docs/SOURCES.md` (global) carries per-game source summaries; the `specs/README.md` Gate 19 row currently reads `_(seed; unwritten)_ | Not started` (confirmed during reassessment) and the spec Status is `Planned`.
2. Spec §4.2 (MECHANICS/PUBLIC-RELEASE-CHECKLIST/GAME-EVIDENCE rows), §6 (exit criteria), §7 (acceptance evidence + command suite), and §10 (documentation-updates table: `specs/README.md`, `docs/SOURCES.md`) define the closeout.
3. Cross-artifact: this ticket reconciles the `specs/README.md` index (modify the existing Gate 19 row to the spec path + `Done`) and the spec Status; `GAME-EVIDENCE.md` consolidates the completion profile, fixture profile, command receipts, no-leak matrix, and the `forward-v1` receipt path from GAT19MELLEDFIV-022.
4. FOUNDATIONS §6: `GAME-EVIDENCE.md` is the evidence consolidation; the `Done`-flip is gated on the full command suite (§7.1) passing — no `Done` before docs/traces/evidence/web/no-leak/benchmarks/CI receipts are complete (spec §9 forbidden change).

## Architecture Check

1. Splitting the closeout into the GAT19MELLEDFIV-022 governance receipt (CI evidence) and this docs+status capstone keeps the CI-consumed receipt separate from the docs/status reconciliation, and lets the `Done`-flip gate on the whole leaf set.
2. No backwards-compatibility shims.
3. `engine-core`/`game-stdlib` untouched; docs/status only.

## Verification Layers

1. Trailing docs complete and link-valid -> `node scripts/check-doc-links.mjs`; `node scripts/check-catalog-docs.mjs`.
2. Full acceptance command suite green end-to-end -> the spec §7.1 suite (`cargo test --workspace`, `simulate` 2/4/6, `replay-check`/`fixture-check`/`rule-coverage`, `cargo bench`, the node checkers, `npm` smokes).
3. `specs/README.md` Gate 19 row + spec Status read `Done` -> grep.

## What to Change

### 1. Trailing game docs

`MECHANICS.md` (local meld/tableau/zone/lay-off/cumulative-scoring model, no trick-taking reuse), `PUBLIC-RELEASE-CHECKLIST.md` (IP/source/no-leak/web/smoke/docs/benchmark/evidence receipts), `GAME-EVIDENCE.md` (completion profile, fixture profile, command receipts, export coverage, no-leak matrix, `forward-v1` receipt path).

### 2. Global source summary

`docs/SOURCES.md` Five Hundred Rummy / Rummy 500 summary, excluded house variants, neutral-name note, prior-art/strategy/UX references.

### 3. Status reconciliation (capstone)

Flip the `specs/README.md` Gate 19 row to the spec path + `Done`; flip the spec's Status field to `Done`. Confirm the spec §7.1 acceptance suite is green.

## Files to Touch

- `games/meldfall_ledger/docs/MECHANICS.md` (new)
- `games/meldfall_ledger/docs/PUBLIC-RELEASE-CHECKLIST.md` (new)
- `games/meldfall_ledger/docs/GAME-EVIDENCE.md` (new)
- `docs/SOURCES.md` (modify)
- `specs/README.md` (modify — Gate 19 row → spec path + `Done`)
- `specs/gate-19-meldfall-ledger-five-hundred-rummy.md` (modify — Status → `Done`)

## Out of Scope

- The `ci/scaffolding-audits.json` receipt + atlas/register/ledger (GAT19MELLEDFIV-022).
- Any production rule/web/WASM logic (exercised, not modified).

## Acceptance Criteria

### Tests That Must Pass

1. The spec §7.1 command suite passes end-to-end (`cargo test --workspace`; `simulate` 2/4/6; `replay-check`/`fixture-check`/`rule-coverage --game meldfall_ledger`; `cargo bench`; `check-doc-links`/`check-ci-games`/`check-catalog-docs`/`check-player-rules`/`check-scaffolding-governance`; `npm` test + `smoke:e2e`).
2. `node scripts/check-doc-links.mjs` and `node scripts/check-catalog-docs.mjs` pass with the trailing docs.
3. `specs/README.md` Gate 19 row and the spec Status both read `Done`.

### Invariants

1. `Done` is set only after the full evidence suite passes (FOUNDATIONS §6; spec §9 forbidden change).
2. No production behavior is modified by this capstone (docs/status only).

## Test Plan

### New/Modified Tests

1. `None — docs-reconciliation + verification capstone; it reconciles the named docs/status surfaces and exercises the prior tickets' acceptance suite, adding no test file.`

### Commands

1. `cargo test --workspace`
2. `node scripts/check-doc-links.mjs && node scripts/check-catalog-docs.mjs && node scripts/check-scaffolding-governance.mjs`
3. `npm --prefix apps/web run smoke:e2e`

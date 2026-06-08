# GAT8AFTROAREA-003: Update apps/web/README.md browser games and smoke layers

**Status**: PENDING
**Priority**: MEDIUM
**Effort**: Small
**Engine Changes**: Yes (presentation-only) — `apps/web/README.md` describing the TypeScript/React shell; no Rust/engine surface touched.
**Deps**: None

## Problem

`apps/web/README.md` names only `race_to_n`, `three_marks`, and `column_four` as the shell's browser games (`apps/web/README.md:3-5`), and its Shell Surface and Smoke Layers sections mention only Three Marks and Column Four board renderers. The shell actually exposes more: `directional_flip`, `draughts_lite`, and `high_card_duel` all have first-class board components and E2E smoke files (`apps/web/e2e/directional-flip.smoke.mjs`, `draughts-lite.smoke.mjs`, `high-card-duel.smoke.mjs`, all run by `smoke:e2e`). This ticket makes the web README accurately describe the supported browser games and smoke layers (spec D3 / WB3).

## Assumption Reassessment (2026-06-08)

1. `apps/web/README.md:3-5` lists only three games; the Shell Surface section (`:47-58`) names "Three Marks and Column Four first-class board renderers" and the Smoke Layers section (`:64-71`) names accessibility/no-leak smoke "for the shell, Three Marks, and Column Four." `ls apps/web/e2e/*.smoke.mjs` confirms smoke files for `column-four`, `directional-flip`, `draughts-lite`, `high-card-duel`, `three-marks`, plus `shell` and `a11y-noleak`. `package.json` `smoke:e2e` runs `three-marks`, `column-four`, `draughts-lite`, `high-card-duel` (per `apps/web/package.json:13`).
2. The set of browser-exposed games is sourced from the actual `apps/web/e2e/` smoke files and `apps/web/package.json` scripts, not from the spec narrative. `directional-flip.smoke.mjs` exists but is not yet in the `smoke:e2e` script chain — the README should describe what the shell supports without overstating CI coverage; cross-check against GAT8AFTROAREA-004 which governs the CI step list.
3. Cross-artifact boundary under audit: the web README is presentation-layer documentation; it must describe the shell as rendering Rust-provided catalog/views/actions/effects/bot turns/replay (FOUNDATIONS §2) and must not imply TypeScript owns or decides game behavior.
4. FOUNDATIONS §2 behavior authority motivates the wording: the README already states "Rust/WASM owns game behavior; TypeScript presents" (`apps/web/README.md:5-7`); preserve that framing when expanding the game list so the doc never reads as TS deciding legality.

## Architecture Check

1. Listing all browser-exposed games and aligning the Shell Surface / Smoke Layers sections to the real `e2e/` + `package.json` surface keeps the README a truthful build/serve/smoke guide — cleaner than a partial list that silently omits shipped games.
2. No backwards-compatibility shims; prose-only change.
3. `engine-core` untouched; no `game-stdlib` change; the README stays presentation-only and does not claim TypeScript behavior authority (§2/§3).

## Verification Layers

1. Web README names all browser-exposed games -> codebase grep-proof (`grep -niE "directional|draughts|high.card" apps/web/README.md`).
2. Smoke Layers section matches the real smoke set -> cross-check grep against `apps/web/package.json` `smoke:e2e` chain and `apps/web/e2e/*.smoke.mjs`.
3. Presentation-only framing preserved -> manual review (§2): the README still states Rust/WASM owns behavior; TypeScript presents.
4. Doc links resolve -> `node scripts/check-doc-links.mjs`.

## What to Change

### 1. Intro game list

Update `apps/web/README.md:3-5` to name the browser games the shell exposes — `race_to_n`, `three_marks`, `column_four`, `directional_flip`, `draughts_lite`, and `high_card_duel` — keeping the "Rust/WASM owns behavior; TypeScript presents" sentence.

### 2. Shell Surface + Smoke Layers

Update the Shell Surface section to name the additional first-class board renderers, and the Smoke Layers section so the `smoke:e2e` description matches the actual game smoke chain in `apps/web/package.json` (do not list a smoke step the script does not run; if `directional_flip` is supported in-shell but not yet in the `smoke:e2e` chain, describe it accurately rather than overstating CI coverage).

## Files to Touch

- `apps/web/README.md` (modify)

## Out of Scope

- Editing `apps/web/package.json` or any `e2e/*.smoke.mjs` (no script/test change here).
- The CI workflow game list (GAT8AFTROAREA-004).
- Any Rust/WASM or renderer code change.

## Acceptance Criteria

### Tests That Must Pass

1. `grep -niE "directional_flip|draughts_lite|high_card_duel" apps/web/README.md` shows all three present.
2. The Smoke Layers description matches the `smoke:e2e` chain in `apps/web/package.json` (manual cross-check — no smoke step claimed that the script does not run).
3. `node scripts/check-doc-links.mjs` passes.

### Invariants

1. The web README describes the shell as presentation-only over Rust/WASM behavior (FOUNDATIONS §2); no wording implies TypeScript decides legality.
2. The game/smoke lists match the actual `apps/web/e2e/` + `package.json` surface, not an aspirational set.

## Test Plan

### New/Modified Tests

1. `None — documentation-only ticket; verification is command-based and existing pipeline/test coverage is named in Assumption Reassessment.`

### Commands

1. `grep -niE "race_to_n|three_marks|column_four|directional_flip|draughts_lite|high_card_duel" apps/web/README.md` — game-list completeness proof.
2. `node scripts/check-doc-links.mjs` — full doc-link integrity pass.
3. `grep -n "smoke:e2e" apps/web/package.json` — confirm the README's smoke description matches the real script chain (the correct verification boundary, since the README must mirror this script).

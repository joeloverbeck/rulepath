# VICEXPSHASUR-009: Shared `OutcomeExplanationPanel` + static templates + accessible presentation

**Status**: PENDING
**Priority**: HIGH
**Effort**: Medium
**Engine Changes**: Yes (presentation-only) — `apps/web/src/components/OutcomeExplanationPanel.tsx` and `apps/web/src/components/outcomeExplanationTemplates.ts`; no Rust/engine, WASM, or behavior surface (TypeScript renders Rust-supplied data only).
**Deps**: VICEXPSHASUR-001

## Problem

There is no shared surface to render the per-game outcome rationales (003–008). Each board owns an ad-hoc terminal status. Build one shared `OutcomeExplanationPanel` plus a static, keyed template-constants file, with the accessible progressive-disclosure presentation the contract (001) requires — independent of content authoring, so it parallelizes with the Rust retrofits. Source: `specs/victory-explanation-shared-surface.md` §7, §11, §15.5.

## Assumption Reassessment (2026-06-09)

1. Verified current code: `apps/web/src/components/` holds `RulesPanel.tsx` (the shared-panel precedent) and `DevPanel.tsx`; no `OutcomeExplanationPanel.tsx` and no template-constants file exist. `RulesPanel.tsx` sources content via **runtime markdown fetch** (`fetch(assetUrl)` from `public/rules/<game_id>.md`, `RulesPanel.tsx:60-63`) — it is **not** a TS template-constants precedent.
2. Spec §7/§11/§15.5. The M3 precedent-divergence note (now in spec §11.1) applies: outcome copy is per-match dynamic and must interpolate Rust-projected values by `template_key`, so keyed TS constants is a deliberate *new* mechanism — not a copy of the rules-display runtime-fetch model. Do not look for a rules-display template file to mirror.
3. Cross-artifact boundary under audit: the panel consumes a presentation-shape adapter fed by the `client.ts` rationale payloads (wired in 010); templates are keyed by the Rust-supplied `template_id`/`template_key`. The panel + templates are the presentation infrastructure, parallelizable with the content authoring in 003–008.
4. FOUNDATIONS §2 restated: TypeScript is presentation-only — the panel renders supplied fields, looks up copy by the Rust-supplied template key, interpolates only viewer-safe supplied values, and owns layout/disclosure/focus/status-message/reduced-motion. It MUST NOT compute the winner or decisive cause (no `determineWinner`/`compareCards`/`findWinningLine`/`resolveTiebreak`/`scoreOutcome`). §5: templates are inert — no comparisons, tiebreak-order logic, rank ordering, selectors, seed/fixture data, YAML, or DSL.
5. §11 no-leak firewall (substrate for deferred enforcement): the panel + templates are the presentation surfaces 011's no-leak smoke will police. Confirm the design renders only Rust-supplied viewer-safe fields and creates no hidden DOM text / `aria-label` / `data-testid` / CSS-class carrying private values, and that the static templates carry no hidden/seed/fixture data. Enforcement is deferred to 002 (template forbidden-content guard) and 011 (no-leak smoke), both of which this ticket names.

## Architecture Check

1. One shared panel (spec D1) + a no-logic presentation adapter beats nine bespoke terminal panels: the data stays per-game and Rust-owned, the UI stays singular, and the keyed-template-constants mechanism fits per-match interpolation that runtime markdown-fetch cannot serve.
2. No backwards-compatibility shims: a brand-new component + template file; no existing panel is aliased.
3. `engine-core`/`game-stdlib` untouched; the panel decides no legality and computes no outcome — `apps/web` presentation only.

## Verification Layers

1. No outcome logic in TS → grep-proof the component/templates contain none of `determineWinner`/`compareCards`/`findWinningLine`/`resolveTiebreak`/`scoreOutcome` and no score/rank comparison; FOUNDATIONS §2 manual review.
2. Accessible presentation → grep/manual: the terminal summary uses `role="status"` (or equivalent), the heading is `aria-labelledby`-associated, disclosure controls expose `aria-expanded`/`aria-controls`, and the decisive cause is text (not color/icon/animation-only).
3. Inert templates → grep-proof the templates file carries no comparison-operator/tiebreak-ladder/selector/YAML tokens (§5/§11); reuses the `check-outcome-explanations.mjs` (002) forbidden-content surface.
4. Builds clean → `npm --prefix apps/web run build` (the panel compiles and type-checks against the presentation shape).
5. Reduced-motion → manual review: the panel presents all terminal facts with animation disabled.

## What to Change

### 1. Add `apps/web/src/components/OutcomeExplanationPanel.tsx`

A shared React surface accepting Rust-provided presentation data (or a game-local payload + a no-logic adapter) and rendering: outcome heading; one-sentence decisive cause; final-standing rows (one per player); expandable breakdown sections; rule references. It owns only UI mechanics — headings, layout, disclosure state, focus, `role="status"` announcement, color-independent encoding, and reduced-motion behavior — per spec §7.1–§7.4. No outcome decision logic.

### 2. Add `apps/web/src/components/outcomeExplanationTemplates.ts`

Typed static template constants keyed by the Rust-supplied `templateId`, each with: `summary` (one sentence with safe placeholders), `expandedHeading`, `requiredParams` (checked by types/tests to avoid broken copy), `allowedGameIds` (inert coverage metadata — never a runtime template selector), and optional `ruleRefLabel`. Templates carry copy only; they contain none of the forbidden content in spec §11.3.

## Files to Touch

- `apps/web/src/components/OutcomeExplanationPanel.tsx` (new)
- `apps/web/src/components/outcomeExplanationTemplates.ts` (new)

## Out of Scope

- Any Rust rationale (003–008) — the panel renders what those project.
- `client.ts` type wiring and board integration (010).
- The browser smoke + smoke registration (011).
- Any legality/scoring/outcome decision in TypeScript; any `engine-core`/`game-stdlib` change.

## Acceptance Criteria

### Tests That Must Pass

1. `npm --prefix apps/web run build` passes (component + templates type-check).
2. `grep -E 'determineWinner|compareCards|findWinningLine|resolveTiebreak|scoreOutcome' apps/web/src/components/OutcomeExplanationPanel.tsx apps/web/src/components/outcomeExplanationTemplates.ts` returns nothing (no TS outcome logic).
3. `grep -q 'role="status"' apps/web/src/components/OutcomeExplanationPanel.tsx` and the disclosure controls expose `aria-expanded`/`aria-controls`.

### Invariants

1. TypeScript renders Rust-supplied fields and interpolates safe values only; it computes no winner, score comparison, line, tiebreaker, or decisive cause (FOUNDATIONS §2).
2. Templates are inert presentation copy — no comparisons/tiebreak-order/selectors/seed data/YAML/DSL (FOUNDATIONS §5); the panel creates no hidden DOM/`data-testid` carrying private values (FOUNDATIONS §11).

## Test Plan

### New/Modified Tests

1. `apps/web/src/components/OutcomeExplanationPanel.tsx` + `outcomeExplanationTemplates.ts` — the new presentation surface; its end-to-end behavior is exercised by the 011 browser smoke once boards are wired (010).
2. `None runnable in isolation pre-integration — verification at this stage is the build + the §2/§5/§11 grep-proofs above; the e2e/no-leak/a11y assertions live in 011.`

### Commands

1. `npm --prefix apps/web run build`
2. `grep -nE 'determineWinner|compareCards|findWinningLine|resolveTiebreak|scoreOutcome' apps/web/src/components/OutcomeExplanationPanel.tsx apps/web/src/components/outcomeExplanationTemplates.ts` (expect no matches)
3. `npm --prefix apps/web run smoke:ui` is not yet a meaningful boundary (panel not board-wired until 010); the build + grep-proofs are the correct verification boundary for this presentation-infra diff.

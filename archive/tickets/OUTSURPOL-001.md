# OUTSURPOL-001: Outcome panel visual design — author the missing CSS

**Status**: ✅ COMPLETED
**Priority**: HIGH
**Effort**: Medium
**Engine Changes**: None — `apps/web/src/styles.css` and `apps/web/e2e/outcome-explanation.smoke.mjs` only (presentation styling; TypeScript renders Rust-supplied data only).
**Deps**: None (VICEXPSHASUR-001…012 are DONE and archived; this polishes the surface they delivered).

## Problem

The shared `OutcomeExplanationPanel` (VICEXPSHASUR-009/-010) ships with **zero CSS**: `apps/web/src/styles.css` (2,884 lines) contains no rule for any of its eight classes (`.outcome-explanation-panel`, `.outcome-summary`, `.outcome-standing`, `.outcome-standing-row`, `.outcome-standing-row.emphasized`, `.outcome-breakdown`, `.outcome-breakdown-section`, `.outcome-rule-refs`). At terminal, every catalog game renders the outcome section with browser defaults: no card container (it floats unboxed between the status banner and the Actions panel), standing-row headers render run together (`<strong>` + `<span>` with no gap → "Seat 0Winner"), `<dl>`s show default 40px `dd` indents, rule references render as a plain bulleted list, disclosure buttons look like generic action buttons with no expanded/collapsed affordance, and the winner row has no emphasis. Empirically confirmed at `http://127.0.0.1:4173/` for Race to 21, Three Marks, and Crest Ledger (poker_lite). This violates the polish bar the rest of the shell meets (cards, eyebrows, banner sub-sections) and FOUNDATIONS §11's "public UI must be polished and accessible" invariant restated in `archive/specs/victory-explanation-shared-surface.md` §FOUNDATIONS table.

## Assumption Reassessment (2026-06-10)

1. Verified current code: `grep -n "outcome" apps/web/src/styles.css` returns nothing; the eight classes above are emitted by `apps/web/src/components/OutcomeExplanationPanel.tsx:100-183` (panel `<section>`, summary `<div role="status">` with `.eyebrow` + `<h2>`, standing `<article>`s with `<header><strong/><span/></header><dl/>`, disclosure `<button aria-expanded aria-controls>`, footer `<span>` + `<ul><li><code>`). `.eyebrow` is already globally styled (`styles.css:91-97`); everything else is unstyled.
2. Verified design conventions in `apps/web/src/styles.css`: card pattern `border: 1px solid #c2cec8; border-radius: 8px; background: #ffffff; padding: 18px;` with `display: grid; gap: 14px;` (`.region`/`.effects`, ~lines 1761-2026); sub-section banner pattern `border: 1px solid #d7ded9; border-radius: 8px; padding: 12px; background: #f9faf7;`; section `h2` at `1rem`; green `#2b8068` (active/legal), orange `#b45f06` (terminal/winning), muted `#5a6d66`/`#63756f`. No CSS custom properties exist — hardcoded hex is the file convention; follow it.
3. Cross-artifact boundary under audit: CSS may target only the class names and ARIA attributes the panel already emits (`[aria-expanded]` is available for the chevron affordance) — no JSX change is in scope, so the component contract, the no-leak smoke surface, and `scripts/check-outcome-explanations.mjs` inputs are untouched.
4. FOUNDATIONS §2 restated: this is pure presentation; no legality, outcome, or view-projection surface moves. UI-INTERACTION.md §16 requires the decisive cause and standing to be color-independent and reduced-motion-safe; the CSS must keep both.
5. Accessibility constraints verified computationally (external research pass, 2026-06-10): `#2b8068` vs `#b45f06` have a **1.04:1** luminance ratio — they are distinguishable by hue only, so green-vs-orange MUST NOT be the only cue distinguishing any two states (WCAG 1.4.1). Both pass AA as text on white (green 4.78:1, orange 4.58:1); orange has no headroom, so no lighter orange tints for text. Borders that carry meaning (winner-row accent) need ≥3:1 vs the surface — both accents qualify; `#d7ded9`-class hairlines do not and stay decorative.
6. The existing e2e smoke `apps/web/e2e/outcome-explanation.smoke.mjs` already asserts panel structure, disclosure keyboard/pointer behavior, reduced-motion text preservation, and no-leak; it has a computed-style precedent (`assertFocusedVisible`, lines 246-261). Style assertions added here must extend that file, not weaken existing checks.

## Architecture Check

1. Authoring the missing rules in the single existing `styles.css`, reusing the established card/banner/eyebrow vocabulary, beats introducing CSS modules or a new stylesheet: the shell has exactly one stylesheet by convention, and visual consistency with `.region`/`.effects` is the stated goal.
2. No backwards-compatibility aliasing/shims: new rules for already-emitted class names only; no class renames, no dead selectors.
3. `engine-core`/`game-stdlib` untouched; no mechanic nouns enter any Rust surface; this ticket is `apps/web` presentation only.

## Verification Layers

1. Panel is styled (card container, not browser defaults) → e2e computed-style assertion: `.outcome-explanation-panel` has a non-zero border and white background; heading font-size strictly greater than body font-size.
2. Color-independence preserved (WCAG 1.4.1 / UI-INTERACTION §16) → manual review: winner emphasis = accent border + background tint **plus** the already-present result text; no green-vs-orange-only distinction anywhere in the new rules.
3. Reduced-motion preserved → existing smoke assertion (reduced-motion text equality) stays green; new rules add no animation outside a `prefers-reduced-motion`/`.reduced`-gated block.
4. No structural/JSX drift → grep-proof: `git diff --stat` touches only `apps/web/src/styles.css` and `apps/web/e2e/outcome-explanation.smoke.mjs`.
5. Full UI pipeline still green → `npm --prefix apps/web run smoke:e2e` (includes the outcome and no-leak smokes).

## What to Change

### 1. `apps/web/src/styles.css` — outcome panel rules

Add a clearly-delimited outcome section following the file's existing comment/ordering conventions:

- **Container** `.outcome-explanation-panel`: the standard card (1px `#c2cec8`, 8px radius, white, 18px padding), `display: grid` with a deliberate spacing rhythm — larger gaps between the panel's four blocks (summary / standing / breakdown / rule refs, ~20-24px) than within them (8-14px), per the group-spacing principle.
- **Summary hierarchy** `.outcome-summary`: keep `.eyebrow` as-is; give the `h2` a full type-scale jump above the shell's 1rem section headings (≈1.5rem, bold) so "who won" is the largest text at terminal; the cause sentence stays regular-weight body text directly beneath it (result—cause as one unit).
- **Standing rows** `.outcome-standing` / `.outcome-standing-row`: sub-card treatment (1px `#d7ded9`, 8px radius, `#f9faf7`, ~12px padding); `header` becomes a flex row with gap — name `<strong>` left, result `<span>` styled as a small uppercase text badge (bordered pill, muted tone) right. **`.emphasized`** (winner): solid ~3px left border in `#b45f06`, white background, bold name — emphasis is border + weight + the badge text, never color alone. Draw standings (no `.emphasized` row) must render identically to each other.
- **Key-value lists** (`dl` in standing rows and breakdown sections): reset browser defaults; two-column grid (label ≈40% muted `#5a6d66`, value ≈60% weight-600), ~4px gap within a pair, ~8px between pairs.
- **Disclosure** `.outcome-breakdown-section`: bordered group; the `button` becomes a full-width, left-aligned header row with a chevron rendered via `::before` driven by `[aria-expanded="true"]`/`[aria-expanded="false"]` (▸/▾ convention); expanded content gets inset padding. No new transition, or gate any transition behind the reduced-motion block.
- **Rule references** `.outcome-rule-refs`: quiet reference matter — small muted label, `ul` reset to inline chips, `code` in the file's existing code styling at reduced size. Visually subordinate to everything above it.
- **Reduced motion**: under `.outcome-explanation-panel.reduced` and `.reduced-motion .outcome-explanation-panel`, force `transition: none; animation: none;` (mirrors `styles.css` pattern at ~517-522).
- **Responsive**: at narrow widths the `dl` grid collapses to stacked label-over-value; standing rows stay full-width.

### 2. `apps/web/e2e/outcome-explanation.smoke.mjs` — styled-state assertions

After the existing `assertOutcomePanel` checks, add a small `assertOutcomeStyled` helper (computed-style, mirroring `assertFocusedVisible`): panel `border-width` ≠ 0px and `background-color` ≠ transparent; outcome `h2` font-size > standing-row body font-size; disclosure button `::before` content non-empty (or `width: 100%` on the button). Keep assertions coarse enough to survive palette tweaks.

## Files to Touch

- `apps/web/src/styles.css` (modify)
- `apps/web/e2e/outcome-explanation.smoke.mjs` (modify)

## Out of Scope

- Any change to `OutcomeExplanationPanel.tsx`, `outcomeExplanationTemplates.ts`, board components, `client.ts`, or any Rust/WASM surface.
- Copy/label changes (raw `seat_0`/enum tokens — OUTSURPOL-002) and disclosure/announcement behavior (OUTSURPOL-003).
- Animations or celebration effects beyond static color/weight/scale emphasis.
- Palette redesign or introduction of CSS custom properties.

## Acceptance Criteria

### Tests That Must Pass

1. `npm --prefix apps/web run build` passes (CSS parses; no TS drift).
2. `node apps/web/e2e/outcome-explanation.smoke.mjs` passes from `apps/web` (after `npm --prefix apps/web run build`), including the new styled-state assertions, the existing reduced-motion equality, and no-leak checks.
3. `npm --prefix apps/web run smoke:e2e` passes (full e2e sweep — no other game smoke regresses).

### Invariants

1. Winner/loser/draw distinction is never conveyed by color alone; every color cue co-occurs with text, weight, or border-structure already present in the DOM (WCAG 1.4.1; UI-INTERACTION §16).
2. The CSS adds no information channel: no `content` text carrying game facts beyond the chevron glyph, no visibility tricks that could hide or reveal payload data (FOUNDATIONS §11 no-leak firewall untouched).

## Test Plan

### New/Modified Tests

1. `apps/web/e2e/outcome-explanation.smoke.mjs` — add `assertOutcomeStyled` (computed-style: card border/background, heading scale step, disclosure affordance) so the "CSS exists and is wired" regression can never silently recur.

### Commands

1. `npm --prefix apps/web run build && node apps/web/e2e/outcome-explanation.smoke.mjs` (targeted; run from repo root with `apps/web/dist` built)
2. `npm --prefix apps/web run smoke:e2e` (full pipeline)
3. Manual play-first audit: reach terminal in Race to 21 (7× "Add 3"), Three Marks (win + draw fills), and Crest Ledger; compare the panel against the adjacent `.region`/`.effects` cards for visual consistency.

## Outcome

Completed: 2026-06-10

What changed:
- Added the missing shared outcome-panel CSS in `apps/web/src/styles.css`: card container, summary hierarchy, standing-row badges and winner emphasis, key-value grids, disclosure affordance, rule-reference chips, reduced-motion suppression, and narrow-width key-value stacking.
- Added computed-style assertions to `apps/web/e2e/outcome-explanation.smoke.mjs` so the outcome panel must have a loaded card style, stronger heading scale, and disclosure affordance.

Deviations from original plan:
- None. The implementation stayed within `apps/web/src/styles.css` and `apps/web/e2e/outcome-explanation.smoke.mjs`; copy, ordering, announcement behavior, Rust/WASM, schemas, traces, and hashes were untouched.

Verification:
- `npm --prefix apps/web run build` passed.
- `node apps/web/e2e/outcome-explanation.smoke.mjs` passed.
- `npm --prefix apps/web run smoke:e2e` passed.

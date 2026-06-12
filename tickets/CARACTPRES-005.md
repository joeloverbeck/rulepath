# CARACTPRES-005: Shared DeckFlowPanel, client types, and Event Frontier adoption

**Status**: PENDING
**Priority**: HIGH
**Effort**: Large
**Engine Changes**: Yes (presentation-only) — `apps/web` (wasm client types, shared deck component, Event Frontier board adoption, smoke); no Rust surface touched
**Deps**: CARACTPRES-001

## Problem

Event Frontier's deck panel is debug output: TS-derived card names (`cardLabel()` strips `ef_` — `apps/web/src/components/EventFrontierBoard.tsx:303-305`), a hardcoded "Hidden order / redacted / undrawn beyond next card" row (`:134-137`), and a count-only discard ("0 discarded", `:120`). Spec Workstream B (D3): one shared deck/pile surface — enlarged current slot, face-up next slot, designed face-down remainder, browsable discard — fed entirely by the Rust-resolved card faces and `ui` copy CARACTPRES-001 projects, with two-tier card rendering (glanceable mini-card; full summary in an accessible detail tier).

## Assumption Reassessment (2026-06-12)

1. Current board verified by full Read this session: deck section at `EventFrontierBoard.tsx:117-139`, hardcoded redaction prose at `:134-137`, TS label derivation at `:303-305,311-313`, sr-only summary at `:105-108`. The shared stylesheet is `apps/web/src/styles.css`; the per-game smoke is `apps/web/e2e/event-frontier.smoke.mjs`; the shared no-leak/a11y smoke is `apps/web/e2e/a11y-noleak.smoke.mjs` (all verified to exist).
2. Spec authority: `specs/card-and-action-presentation-shared-surfaces.md` §6 D3 (slot anatomy, count-badge-only-when-public, discard disclosure, color+icon family banding), §5 research grounding (public info at-a-glance — summary visible without hover; detail tier holds full text only), §9 exit criteria 1/7/8. `DeckFlowPanel` is the spec's indicative name (assumption A6) — keep unless a better name emerges in review.
3. Cross-artifact boundary under audit: `apps/web/src/wasm/client.ts` typed reader of the Event Frontier view. CARACTPRES-001 changed `current_card`/`next_public_card`/`discard` from `string` shapes to resolved card-face objects and added `ui` — this ticket updates the TS types to match exactly (breaking-for-TS, coordinated in-spec; no runtime consumers outside this repo).
4. FOUNDATIONS §2/§7 restated: the component renders Rust/static-supplied text only — no ID-derived labels, no TS-authored gameplay copy (the hidden-pile copy comes from the `ui` channel), no legality decisions. Event Frontier's undrawn count stays absent (`EF-VIS-002`): the face-down slot renders the designed "order hidden" treatment, count badge only for games whose view supplies one (flood_watch, via CARACTPRES-006).
5. No-leak firewall (§11): the new DOM/test-ID surfaces must not encode hidden facts — card faces render only for view-projected IDs; `data-testid`s carry slot roles, not card identities for face-down slots; the a11y/no-leak smoke extends to assert no `ef_`-prefixed or undrawn-order content appears for the face-down slot.
6. Schema/type extension audit: `client.ts` `EventFrontierPublicView` consumers are `EventFrontierBoard.tsx` and `main.tsx`'s `isEventFrontierView` guard — both updated here; no other importers (grep-verify at implementation).

## Architecture Check

1. One shared component with per-slot props (current / next / face-down / discard, optional count) beats per-game deck markup: the two event-deck games render identically-structured flows, future deck games get the surface for free (spec D6 future-binding), and the redaction treatment is designed once instead of hand-written per board.
2. No backwards-compatibility aliasing/shims: the old deck `<section>` markup is removed in the same diff; no fallback path renders TS-derived labels (the `cardLabel`/`actionLabel` helpers lose their deck callers; `actionLabel` removal belongs to CARACTPRES-007).
3. `engine-core`/`game-stdlib` untouched; mechanic-aware ("card") TypeScript is legitimate in `apps/web` (FOUNDATIONS §3 governs Rust crates; `docs/UI-INTERACTION.md` §3 ownership table).

## Verification Layers

1. No TS-derived display text remains for deck surfaces -> codebase grep-proof: `cardLabel(` has no remaining callers in the deck section; no `replace(/^ef_/`-style derivation in the component.
2. Hidden-info safety of new DOM/test-IDs -> no-leak visibility assertions in `apps/web/e2e/a11y-noleak.smoke.mjs` (face-down slot exposes no card identity; no undrawn-order facts).
3. At-a-glance public info + detail tier accessibility (disclosure semantics, color+icon banding, reduced-motion parity) -> UI smoke assertions in `apps/web/e2e/event-frontier.smoke.mjs` + manual review against `docs/UI-INTERACTION.md` §16-§17 patterns.
4. Settle-to-view and effect-driven animation doctrine unchanged -> `npm --prefix apps/web run smoke:effects` green (no new animation authority introduced).

## What to Change

### 1. Client types

Update `EventFrontierPublicView` in `apps/web/src/wasm/client.ts`: card-face object type (id, label, summary, family, accessibility label), `ui` metadata type — matching CARACTPRES-001's projection exactly.

### 2. Shared `DeckFlowPanel` component

`apps/web/src/components/DeckFlowPanel.tsx`: slots for enlarged current card, face-up next card, face-down remainder (count badge only when a public count is supplied; otherwise the designed order-hidden treatment with Rust/static-supplied copy), and a disclosure-expandable discard list. Mini-card face: label + family band (color + icon/text, never color alone) + one-line summary; full summary in an accessible tooltip/disclosure detail tier. Reduced-motion safe. Styles in `apps/web/src/styles.css`.

### 3. Event Frontier adoption

Replace `EventFrontierBoard.tsx`'s deck section and hardcoded redaction prose with `DeckFlowPanel`; update the sr-only live summary to use Rust labels; route hidden-pile copy from `view.ui`.

### 4. Smoke coverage

Extend `event-frontier.smoke.mjs` (slots render resolved labels/summaries; discard expands; detail tier keyboard-accessible) and `a11y-noleak.smoke.mjs` (no hidden facts in DOM/test-IDs).

## Files to Touch

- `apps/web/src/wasm/client.ts` (modify)
- `apps/web/src/components/DeckFlowPanel.tsx` (new)
- `apps/web/src/components/EventFrontierBoard.tsx` (modify)
- `apps/web/src/styles.css` (modify)
- `apps/web/e2e/event-frontier.smoke.mjs` (modify)
- `apps/web/e2e/a11y-noleak.smoke.mjs` (modify)

## Out of Scope

- Action panel / `ActionPathBuilder` — CARACTPRES-007 (shares `EventFrontierBoard.tsx`; different section).
- Flood Watch adoption — CARACTPRES-006.
- Catalog copy hygiene + CI guard — CARACTPRES-009.
- Any legality, ordering, or label derivation in TypeScript; any visibility-contract change.
- Effect-log or animation-scheduler redesign.

## Acceptance Criteria

### Tests That Must Pass

1. `npm --prefix apps/web run smoke:ui` and `npm --prefix apps/web run smoke:effects` green with the new panel.
2. Extended e2e assertions: current/next slots show authored label + summary at a glance; face-down slot shows the designed treatment with no count for Event Frontier; discard list expands and lists resolved labels; no `ef_` raw IDs anywhere in rendered text.
3. `npm --prefix apps/web run build` green (type-level proof the client.ts extension is consistent).

### Invariants

1. Every visible deck string originates from Rust view data or the Rust/static `ui` channel (FOUNDATIONS §2; UI-INTERACTION §5 payload table).
2. Face-down surfaces expose no card identity, order, or count facts beyond what the view projects (§11 no-leak).

## Test Plan

### New/Modified Tests

1. `apps/web/e2e/event-frontier.smoke.mjs` — deck-slot rendering, discard disclosure, detail-tier keyboard access, raw-ID absence.
2. `apps/web/e2e/a11y-noleak.smoke.mjs` — face-down DOM/test-ID no-leak assertions for event_frontier.

### Commands

1. `npm --prefix apps/web run build && npm --prefix apps/web run smoke:ui`
2. `npm --prefix apps/web run smoke:effects && npm --prefix apps/web run smoke:e2e`
3. Narrow boundary rationale: presentation-only diff; Rust gates ran at CARACTPRES-001.

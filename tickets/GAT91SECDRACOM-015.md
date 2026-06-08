# GAT91SECDRACOM-015: SecretDraftBoard + shell integration + effect log + styles

**Status**: PENDING
**Priority**: HIGH
**Effort**: Large
**Engine Changes**: Yes (presentation-only) — `apps/web/src/components/SecretDraftBoard.tsx` (new); `apps/web/src/components/{ActionControls,EffectLog,effectFeedback}.{tsx,ts}` and `apps/web/src/styles.css` (modify). No Rust/engine change.
**Deps**: GAT91SECDRACOM-014

## Problem

The game needs a polished, accessible React board that renders the visible pool, drafted collections, pending-seat status, priority marker, scores, and reveal history — driving the synchronized reveal as a grouped effect-batch with reduced-motion support, and never rendering a committed item ID before reveal in DOM, attributes, or `data-testid`. This is the play-first surface that proves "UI shows pending seats without leaking choices."

## Assumption Reassessment (2026-06-08)

1. The renderer precedents are `apps/web/src/components/TokenBazaarBoard.tsx` and `HighCardDuelBoard.tsx` (both verified present); the shared shell pieces `ActionControls.tsx`, `EffectLog.tsx`, and `effectFeedback.ts` exist and are extended per game. `apps/web/src/styles.css` holds board styles.
2. The TS bindings/catalog wiring (GAT91SECDRACOM-014) and the Rust-provided viewer-safe views/effects (via WASM) are inputs. Spec §Deliverables (Browser row) + §"WASM/browser wiring": `SecretDraftBoard.tsx` renders visible pool, drafted collections, pending seats, priority marker, scores, reveal history, safe action affordances; `ActionControls` must not use `choice-${submitted_hidden_id}` as a persistent DOM/test anchor after commit (pending UI uses seat/round anchors); `EffectLog`/`effectFeedback.ts` treat reveal as a grouped batch with reduced motion.
3. Cross-artifact boundary under audit: the effect→animation contract and the DOM/test-id no-leak surface. Animation is driven by Rust semantic effects (commit, pending, reveal batch), settling to the latest viewer-safe view; renderer diffs are diagnostics only.
4. §7 public-UI + §11 (semantic-effect animation; no-leak; legal-only UI) are the motivating principles: restate before trusting spec — animation derives from Rust effects (not guessed state diffs); illegal moves are absent/inert; no committed item ID reaches DOM text/attributes/`data-testid`/storage/logs before reveal, even for the committing seat (A6, who sees "You have committed"); reduced motion preserves event order and no-leak.
5. No-leak DOM anchors: after commit, action-control anchors switch to seat/round identifiers; pre-commit pool-choice anchors may use public item IDs (they are merely public pool choices). The negative DOM/storage/test-id assertions are authored in GAT91SECDRACOM-016.

## Architecture Check

1. A dedicated `SecretDraftBoard` consuming Rust effects/views (with reveal as a grouped batch) is cleaner than overloading an existing board: it keeps the pending/reveal interaction model explicit and the no-leak anchors auditable.
2. No backwards-compatibility aliasing/shims.
3. `engine-core` untouched; the renderer is presentation-only over Rust-provided data. No `game-stdlib` change. No TS legality (§2 preserved).

## Verification Layers

1. Effect-driven animation -> reveal renders from the grouped Rust effect batch; renderer settles to the latest viewer-safe view (manual review + e2e in GAT91SECDRACOM-016).
2. Pending-seat UI -> board shows `seat_0 committed` / `seat_1 waiting`, round, pool, safe copy without any committed ID (manual + e2e).
3. DOM/test-id no-leak -> post-commit anchors use seat/round, not the submitted item ID (grep/manual; e2e negative assertions in GAT91SECDRACOM-016).
4. Accessibility / reduced motion -> a11y review + reduced-motion path preserves order and no-leak.
5. Build/type-check -> `npm --prefix apps/web run build`.

## What to Change

### 1. `apps/web/src/components/SecretDraftBoard.tsx`

New board: visible pool (public item IDs/labels/values/threads), per-seat drafted collections, pending-seat indicators, priority marker, scores + tie-break summary, reveal history; safe action affordances from the Rust action tree; grouped reveal-batch animation; reduced-motion support; responsive/accessible layout. Pre-reveal, render only "committed/waiting", never the chosen item ID.

### 2. `apps/web/src/components/effectFeedback.ts` + `EffectLog.tsx`

Add commit and reveal-batch entries; treat reveal as a grouped batch; reduced-motion aware; effect-log text carries no pre-reveal item ID.

### 3. `apps/web/src/components/ActionControls.tsx`

Support simultaneous pending state without TS legality; post-commit anchors keyed by seat/round, not the submitted item ID.

### 4. `apps/web/src/styles.css`

Board styles consistent with the cozy premium table aesthetic (§7); no casino/debug-dominant styling.

## Files to Touch

- `apps/web/src/components/SecretDraftBoard.tsx` (new)
- `apps/web/src/components/effectFeedback.ts` (modify)
- `apps/web/src/components/EffectLog.tsx` (modify)
- `apps/web/src/components/ActionControls.tsx` (modify)
- `apps/web/src/styles.css` (modify)

## Out of Scope

- e2e smoke, a11y/no-leak assertions, gate-1 CI, and catalog README reconciliation (GAT91SECDRACOM-016).
- TS bindings/catalog wiring (GAT91SECDRACOM-014) and Rust/WASM (GAT91SECDRACOM-013).
- UI.md doc (GAT91SECDRACOM-017).

## Acceptance Criteria

### Tests That Must Pass

1. `npm --prefix apps/web run build` succeeds.
2. `npm --prefix apps/web run smoke:ui` (and `smoke:preview`) pass with the board mounted.
3. Manual review: pre-reveal DOM/attributes/`data-testid` carry no committed item ID; reveal animates from the grouped Rust effect batch; reduced motion preserves order.

### Invariants

1. Animation is driven by Rust semantic effects; renderer diffs are diagnostics only; UI is legal-only (§7/§11).
2. No committed item ID in DOM/attributes/`data-testid` before reveal, including the committing seat (§11 no-leak, A6).

## Test Plan

### New/Modified Tests

1. `None — renderer ticket; verification is build + smoke:ui/preview + manual a11y/no-leak review. Automated DOM/storage no-leak assertions are authored in GAT91SECDRACOM-016.`

### Commands

1. `npm --prefix apps/web run build`
2. `npm --prefix apps/web run smoke:ui && npm --prefix apps/web run smoke:preview`
3. Build + UI smoke is the correct boundary for the renderer; rendered-browser no-leak/a11y e2e is GAT91SECDRACOM-016.

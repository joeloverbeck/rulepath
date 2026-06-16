# RIVLEDSHOWUX-015: Compact bot "Why?" disclosure

**Status**: PENDING
**Priority**: LOW
**Effort**: Small
**Engine Changes**: Yes (presentation-only) — `apps/web/src/components/RiverLedgerBoard.tsx`, `apps/web/src/components/ModeControls.tsx`
**Deps**: RIVLEDSHOWUX-014

## Problem

The Rust `BotDecisionPublicExplanation` (RIVLEDSHOWUX-014) needs a surface: a compact, non-debug "Why?" disclosure near the latest bot action / active status that renders the one-sentence reason + public facts when the payload is present. It must not dump candidate data into the effect log, and random / no-explanation bots show no affordance. **Optional per spec Assumption A5** — droppable with no dependents.

## Assumption Reassessment (2026-06-16)

1. Verified: `RiverLedgerBoard.tsx` and `ModeControls.tsx` are the in-play surfaces near the active status; `EffectLog.tsx` / `effectFeedback.ts` are leak-prone debug surfaces the explanation must NOT be dumped into.
2. Verified against spec §6 D9 + §8 WB15 (#14) + §14 A5 (optional); the payload arrives via RIVLEDSHOWUX-014 (hence `Deps`); `RULES.md` `RL-BOT-EXPLAIN-001`.
3. Shared boundary under audit: the optional `ModeControls` mount renders the explanation only when the payload is present; the disclosure is otherwise River-Ledger-local in `RiverLedgerBoard`.
4. FOUNDATIONS §8 (viewer-safe bot explanation) + §7 (non-debug, play-first — not a candidate-dump) motivate this ticket.

## Architecture Check

1. A presence-keyed compact disclosure (vs an always-on debug panel) keeps the affordance play-first and renders nothing for random bots; reusing the Rust-authored text means TS adds no reasoning.
2. No shims; the disclosure consumes the RIVLEDSHOWUX-014 payload directly.
3. `engine-core` untouched (§3); no `game-stdlib` change (§4); presentation only; not routed through the effect log.

## Verification Layers

1. Non-random-bot action shows a compact "Why?" with the Rust reason + public facts; random bots show none -> `npm --prefix apps/web run smoke:ui`.
2. The explanation is not written to the effect log / DOM debug surfaces -> `node apps/web/e2e/a11y-noleak.smoke.mjs` (effect-log no-leak sweep).
3. Disclosure is keyboard-accessible with clean a11y labels -> `npm --prefix apps/web run smoke:ui`.

## What to Change

### 1. `apps/web/src/components/RiverLedgerBoard.tsx`

Render a compact non-debug "Why?" disclosure near the latest bot action / active status when `BotDecisionPublicExplanation` is present: one-sentence reason + public facts, keyboard-accessible, clean a11y labels.

### 2. `apps/web/src/components/ModeControls.tsx`

Optionally surface the latest bot explanation in the shared status area only when the payload exists (no mount for random/no-explanation bots).

## Files to Touch

- `apps/web/src/components/RiverLedgerBoard.tsx` (modify)
- `apps/web/src/components/ModeControls.tsx` (modify)

## Out of Scope

- The Rust explanation payload (RIVLEDSHOWUX-014).
- Proceeding at all if the bot "Why?" affordance is dropped per spec A5 (this ticket is then not-applicable; no other ticket depends on it).
- Any candidate-ranking or hidden-strength display; any effect-log routing.

## Acceptance Criteria

### Tests That Must Pass

1. `npm --prefix apps/web run smoke:ui` + `npm --prefix apps/web run build` — "Why?" disclosure renders for non-random bots, absent for random; type-checks.
2. `node apps/web/e2e/a11y-noleak.smoke.mjs` — the explanation is not dumped into the effect log or any debug surface.
3. `npm --prefix apps/web run smoke:e2e` — in-play flow unaffected.

### Invariants

1. TS renders only the Rust-authored explanation; it computes no reasoning and shows no candidate data (§8, §2).
2. The affordance is play-first and non-debug; nothing routes through the effect log (§7, §11 no-leak).

## Test Plan

### New/Modified Tests

1. `apps/web/scripts/smoke-ui.mjs` (modify, as surfaced) — "Why?" disclosure present for non-random bots, absent for random.
2. `apps/web/e2e/a11y-noleak.smoke.mjs` (modify, as surfaced) — effect-log no-leak assertion for bot explanation.

### Commands

1. `npm --prefix apps/web run smoke:ui`
2. `node apps/web/e2e/a11y-noleak.smoke.mjs`
3. `npm --prefix apps/web run smoke:e2e`

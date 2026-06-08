# GAT8HIGCAR-003: Render public-observer-projection effects in the Replay viewer

**Status**: COMPLETED
**Priority**: HIGH
**Effort**: Small
**Engine Changes**: None (presentation only) — `apps/web/src/wasm/client.ts` (`ReplayStep` type), `apps/web/src/components/ReplayViewer.tsx` (render path), and the web no-leak smoke. No Rust, no `engine-core`/`game-stdlib`/`games/*`, no schema/trace change.
**Deps**: `GAT8HIGCAR-002` (wasm-api must emit per-step `public_effects` / `redacted_command_summary` before this can render them). Relates to ADR `docs/adr/0004-hidden-info-replay-export-taxonomy.md`.

## Problem

After `GAT8HIGCAR-002`, the imported public-observer-projection replay-step JSON carries per-step `public_effects` (string array) and `redacted_command_summary`. The web Replay viewer still renders "No replay effects at this cursor." because it only reads the structured `effects` field and routes it through `feedbackForEffect`, which expects structured engine effect payloads (`entry.effect.payload.type`, `apps/web/src/components/effectFeedback.ts:14-15`). Public exports carry opaque observation strings (e.g. `hcd_cards_revealed:round=1;seat_0_card=hcd:r09:b;seat_1_card=hcd:r02:a`), which are not structured payloads and never match that switch. The viewer needs a dedicated public-effects render path.

## Assumption Reassessment (2026-06-08)

1. **Viewer reads only structured `effects`.** `ReplayViewer` (`apps/web/src/components/ReplayViewer.tsx:25`) computes `const effects = step?.effects ?? []` and (lines 113-128) renders `feedbackForEffect(entry)` for each, or the empty-state `<li>No replay effects at this cursor.</li>` (line 115) when `effects.length === 0`. For public imports `effects` is `[]` (and stays `[]` after `GAT8HIGCAR-002`, which adds a separate `public_effects` field).
2. **`feedbackForEffect` is structured-only.** It switches on `entry.effect.payload.type` (`apps/web/src/components/effectFeedback.ts:14-204`). Public effect strings have no `payload.type` and must not be forced through it — they are pre-formatted observation strings, not engine effect envelopes.
3. **Cross-artifact boundary under audit:** the public replay-step JSON contract (`crates/wasm-api/src/lib.rs` `public_replay_step_json`, extended in `GAT8HIGCAR-002`) ↔ web `ReplayStep` type (`apps/web/src/wasm/client.ts:507-515`). This ticket adds the `public_effects?: string[]` (and `redacted_command_summary?: string`) fields to `ReplayStep` and consumes them. The fields are additive/optional, so full-internal-trace replays (which omit them) are unaffected.
4. **§11 no-leak firewall (display side).** The viewer must render only the strings Rust already deemed public — it must not parse them to recover or re-display hidden card identities, and must not introduce any client-side derivation. Pass-through rendering of the already-public strings only. The existing browser no-leak smoke (`apps/web/e2e/high-card-duel.smoke.mjs`) is the guard; extend it to assert the imported/stepped DOM contains no unrevealed `hcd:r` identity.
5. **Adjacent contradiction (separate, deferred):** `PlacementSequence` (`ReplayViewer.tsx:146-166`) reads `replay.document.commands`, which a `PublicObserverReplayExport` (`apps/web/src/wasm/client.ts:480-490`, has `steps`, not `commands`) does not provide, so it renders nothing for public imports. The redacted command timeline is conveyed instead by each step's `redacted_command_summary`. Reworking `PlacementSequence` for public exports is out of scope.

No spec mismatch found.

## Architecture Check

1. **Dedicated public-effects branch is cleaner than coercion.** Rendering `step.public_effects` as plain labeled observation lines (when `step.public_export` is true) is cleaner than fabricating structured `EffectEntry` payloads from the strings to reuse `feedbackForEffect`: the strings are deliberately opaque public projections (ADR-0004), and synthesizing structured effects would re-derive behavior in the presentation layer — exactly what the Rust-owns-behavior boundary forbids. The structured path stays untouched for internal-trace replays.
2. **No backwards-compatibility aliasing/shims.** New optional fields and a new render branch; the existing structured-effects branch is unchanged.
3. **Presentation only.** No Rust, no `engine-core`/`game-stdlib` involvement; TypeScript continues to decide nothing about legality or behavior — it renders strings Rust produced.

## Verification Layers

1. Imported public replay shows its per-step plays (user-facing fix) -> UI smoke / manual review: stepping the imported document renders commit / reveal / round-scored / terminal lines instead of "No replay effects at this cursor."
2. No hidden-information reaches the DOM (§11 no-leak firewall) -> no-leak visibility test: extend `apps/web/e2e/high-card-duel.smoke.mjs` to assert the stepped public-import DOM contains no unrevealed `hcd:r` identity and no `seed`.
3. Additive, non-breaking for other replays -> codebase grep-proof: `ReplayStep.public_effects` is optional; internal-trace replays (no `public_effects`) still render via the structured `effects` branch unchanged.
4. Type/lint/build integrity -> `npm --prefix apps/web run build` (tsc) passes with the new optional fields.

## What to Change

### 1. Extend the `ReplayStep` type

In `apps/web/src/wasm/client.ts`, add to `ReplayStep` (`client.ts:507-515`): `public_effects?: string[]` and `redacted_command_summary?: string`. Keep `effects: EffectEntry["effect"][]` as-is.

### 2. Render the public-effects branch in `ReplayViewer`

In `apps/web/src/components/ReplayViewer.tsx`, in the `<ol className="replay-effects">` block (lines 113-128): when `step.public_export` is true and `step.public_effects` is non-empty, render each public effect string as an observation line (lightly humanized label, e.g. split the leading `hcd_<event>` token from the `;`-delimited fields, or render verbatim). Optionally show `step.redacted_command_summary` as the step's command line. Preserve the existing structured-`effects` branch for non-public replays. The "No replay effects at this cursor." empty state must only show when both `effects` and `public_effects` are empty (e.g. cursor 0, the initial state).

## Files to Touch

- `apps/web/src/wasm/client.ts` (modify) — `ReplayStep` optional `public_effects` / `redacted_command_summary`.
- `apps/web/src/components/ReplayViewer.tsx` (modify) — public-effects render branch.
- `apps/web/e2e/high-card-duel.smoke.mjs` (modify) — assert public-import stepping renders effects and leaks nothing.

## Out of Scope

- Reworking `PlacementSequence` to display a public-import command timeline (Assumption 5).
- Rendering a `high_card_duel` board/snapshot for public imports (depends on view reconstruction deferred in `GAT8HIGCAR-002`; `step.view` stays `null`).
- Any change to structured-effect rendering for internal-trace replays.

## Acceptance Criteria

### Tests That Must Pass

1. `npm --prefix apps/web run build` (tsc + bundle) passes with the new optional `ReplayStep` fields.
2. `apps/web/e2e/high-card-duel.smoke.mjs`: after importing the public export and stepping, the Replay viewer DOM shows public effect lines (e.g. a reveal and a round-scored line) and never the "No replay effects at this cursor." text past cursor 0.
3. The same smoke asserts the stepped DOM contains no unrevealed `hcd:r` card identity and no `seed`.

### Invariants

1. The viewer renders only the public effect strings Rust produced; it performs no client-side derivation of hidden state (FOUNDATIONS §11 no-leak firewall; ADR-0004 — TS presents only).
2. `public_effects` is additive/optional; internal-trace replays render unchanged through the structured `effects` branch.

## Test Plan

### New/Modified Tests

1. `apps/web/e2e/high-card-duel.smoke.mjs` — extend the existing public-export smoke to import, step, assert rendered public effects, and assert no hidden-identity leak in the stepped DOM.

### Commands

1. `npm --prefix apps/web run build`
2. `npm --prefix apps/web run smoke:ui`
3. `npm --prefix apps/web run smoke:wasm` (confirms the wasm surface from `GAT8HIGCAR-002` emits `public_effects` as consumed here).

## Outcome

Completed: 2026-06-08

What changed:
- `apps/web/src/wasm/client.ts` now includes optional `public_effects` and `redacted_command_summary` fields on `ReplayStep`.
- `ReplayViewer` renders imported public-observer replay observations through a dedicated public branch instead of the structured effect renderer, while leaving internal replay effects unchanged.
- Public observation text is lightly formatted for display and raw `hcd:r...` ids are not put into the DOM.
- `apps/web/e2e/high-card-duel.smoke.mjs` now imports a post-reveal public export, steps through the replay viewer, asserts commit/reveal/scoring observations render past cursor 0, and checks the replay viewer for forbidden leak terms.

Deviations from original plan:
- The display branch humanizes and redacts raw card ids from public observation strings instead of rendering them verbatim, preserving the existing browser no-leak smoke's raw-id DOM rule.

Verification results:
- `npm --prefix apps/web run build` passed.
- `npm --prefix apps/web run smoke:wasm` passed.
- `node apps/web/e2e/high-card-duel.smoke.mjs` passed.
- `npm --prefix apps/web run smoke:ui` passed.

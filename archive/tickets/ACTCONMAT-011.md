# ACTCONMAT-011: Bot "why?" audit and conditional §15 affordance

**Status**: COMPLETED
**Priority**: LOW
**Effort**: Small
**Engine Changes**: Conditional — `Yes (presentation-only)` if a viewer-safe explanation is exposed and the affordance is rendered (`apps/web` reading the existing bot explanation from the wasm bridge); `None` if the audit finds nothing viewer-safe exposed and records a named follow-up. No new bot reasoning either way.
**Deps**: None

## Problem

`docs/UI-INTERACTION.md` §15 says public mode SHOULD offer a "why?" / recent-bot-action affordance for non-random bots. Event Frontier ships scripted Level 1 policy bots (Gate 14) but the live session surfaced no explanation affordance. It is unverified what the wasm bridge exposes for these bots; this ticket audits that and either renders the §15 affordance or records the blocking gap as a named follow-up.

## Assumption Reassessment (2026-06-12)

1. EF ships scripted Level 1 policy bots (`games/event_frontier/src/bots.rs`: `EventCharterLevel1Bot`, `EventFreeholdersLevel1Bot`, policy IDs `event_frontier_charter_level1_v1` / `..._freeholders_level1_v1`). What the wasm bridge (`crates/wasm-api/src/lib.rs`) exposes as a viewer-safe bot explanation is unverified (spec A6/O13 — needs this audit before committing to rendering).
2. Spec §4.2 / D-set: implement the §15 affordance if a viewer-safe explanation is exposed, OR record the blocking gap as a named follow-up. This is a conditional/decision-gated ticket: the audit outcome decides whether UI is built.
3. Cross-artifact boundary under audit: the bot-decision explanation surface (`ai-core`/`games/*` bot → `wasm-api` projection → TS). The audit confirms whether a viewer-safe explanation crosses the boundary today.
4. FOUNDATIONS §8 (public bots are explainable, fair) / §11: any rendered explanation uses ONLY viewer-safe fields; it must not leak hidden state or candidate rankings. This ticket renders an *existing* explanation or defers — it adds no new bot reasoning (spec §11 Forbidden changes).
5. No-leak firewall surface: confirm the bot explanation exposed to the viewer contains no hidden information unavailable to the viewer's seat (§11 no-leak — "bot explanations" are a named leak vector); if nothing viewer-safe is exposed, the affordance is deferred rather than synthesized.

## Architecture Check

1. Auditing before building avoids rendering an affordance with no lawful data behind it. Rendering an existing viewer-safe explanation (if present) reuses the bridge surface rather than inventing bot reasoning in TS — which would violate §8/§2.
2. No shim: either a real affordance backed by exposed data, or an explicit deferred follow-up — no placeholder that fabricates a reason.
3. `engine-core` untouched; no new bot search class (§13). No `game-stdlib` change.

## Verification Layers

1. What the bridge exposes for the EF Level 1 bots -> codebase grep-proof / manual review of the `wasm-api` projection.
2. If rendered: the affordance shows only viewer-safe fields -> no-leak visibility test.
3. If deferred: the gap is recorded as a named follow-up -> manual review (Step 6 cross-spec follow-up).

## What to Change

### 1. Audit the bridge

Determine what `crates/wasm-api/src/lib.rs` exposes as a viewer-safe bot explanation / recent-bot-action for the EF Level 1 policy bots.

### 2a. If exposed — render the affordance

Add a minimal §15 "why?" affordance (presentation-only) that shows the concise reason + relevant visible fact from the exposed explanation.

### 2b. If not exposed — record the follow-up

Record the blocking gap as a named follow-up (what the bridge would need to expose), with no UI built this round.

## Files to Touch

- `apps/web/src/components/ActionControls.tsx` (modify; conditional — only if 2a, a small "why?" affordance) — or none if 2b
- (audit reads `crates/wasm-api/src/lib.rs` and `games/event_frontier/src/bots.rs`; no modification unless 2a requires a bridge read already present)

## Out of Scope

- Proceeding to build UI at all if the audit (2b) finds nothing viewer-safe exposed — this ticket is then a recorded follow-up, not an implementation.
- Any new bot reasoning, candidate ranking, or policy change (spec §11 Forbidden changes) — render existing explanations only.
- Bot explanations for other games (catalog-wide §15 is a separate concern).

## Acceptance Criteria

### Tests That Must Pass

1. The audit outcome is recorded (affordance rendered OR named follow-up documented).
2. If rendered: a no-leak smoke confirms the "why?" affordance exposes no hidden information (`apps/web/e2e/a11y-noleak.smoke.mjs`).
3. `npm --prefix apps/web run smoke:e2e` green (whether or not the affordance is added).

### Invariants

1. Any rendered explanation uses only viewer-safe fields the bridge already exposes (§8/§11) — no new bot reasoning.
2. Coverage is not silently skipped: the deliverable is either the affordance or a recorded follow-up.

## Test Plan

### New/Modified Tests

1. If 2a: `apps/web/e2e/a11y-noleak.smoke.mjs` — assert the affordance leaks no hidden state.
2. If 2b: no test; the follow-up is recorded in the Step 6 summary and the spec's A6 disposition.

### Commands

1. `npm --prefix apps/web run smoke:e2e`
2. `cargo test -p event_frontier` (bot legality unchanged)
3. `grep -n "explanation\|reason" crates/wasm-api/src/lib.rs` (audit surface)

## Outcome

Audit outcome: affordance rendered. `crates/wasm-api/src/lib.rs` already projects Event Frontier Level 1 bot decisions through `run_bot_turn` as `{ policy_id, policy_version, rationale, effects, view }`; `games/event_frontier/src/bots.rs` builds the rationale from public-view policy inputs, and the existing bot tests include `bot_inputs_and_explanations_do_not_expose_undrawn_deck_order`.

Implemented a presentation-only "Bot why" disclosure in the shell mode controls for Event Frontier. The web client now preserves the full bot-turn response, stores the latest viewer-safe rationale in shell state, and renders the Rust-authored rationale plus a human-readable policy tier. The raw Rust policy id remains internal and is not shown in normal-mode UI.

Verification:

1. `grep -n "explanation\|reason" crates/wasm-api/src/lib.rs` — audit command run.
2. `grep -n "rationale" crates/wasm-api/src/lib.rs` — confirmed Event Frontier bot-turn response projects `rationale` at the bridge.
3. `cargo test -p event_frontier` — passed.
4. `npm --prefix apps/web run build` — passed.
5. `node apps/web/e2e/event-frontier.smoke.mjs` — passed with the opened "Bot why" no-leak assertion.
6. `npm --prefix apps/web run smoke:e2e` — passed.

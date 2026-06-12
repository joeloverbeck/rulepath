# ACTCONMAT-005: Faction-first match-context surface

**Status**: PENDING
**Priority**: HIGH
**Effort**: Large
**Engine Changes**: Yes — `games/event_frontier/src/ui.rs` (seat/faction display labels), `games/*/src/ui.rs` (seat-label backfill audit, 13 games), `crates/wasm-api/src/lib.rs` (seat/faction label projection); plus presentation (`apps/web/src/main.tsx`, `apps/web/src/components/EventFrontierBoard.tsx`).
**Deps**: None

## Problem

Nothing in the match surface says the local player is the Charter. Setup says "SEAT 0 / Local"; the mode line says "Seat 0 is active"; resource tiles ("Funds 2", "Provisions 4") never name their owning faction; waiting states read as broken ("No legal actions available."). Charter is pinned to `seat_0` in Rust and Human-vs-bot pins seat 0 local, but the UI never states either fact. Seat vocabulary must leave normal-mode surfaces in favor of faction-first framing.

## Assumption Reassessment (2026-06-12)

1. `games/event_frontier/src/ui.rs` `UiMetadata` currently exposes table/deck/card labels only — no `seat_labels`/`faction_labels`. Charter is pinned to `seat_0` (`games/event_frontier/src/setup.rs`/`state.rs` faction_order `[Charter, Freeholders]`). `main.tsx`/`ModeControls` render "Seat 0 is active" via `seatLabel`; `EventFrontierBoard.tsx` renders resource metrics with no owner attribution.
2. Spec D5 / §4.2: per-game `ui.rs` metadata gains seat display labels ("Charter", "Freeholders"); the shell renders setup seats as "Charter — you (local)", an identity line ("You play the Charter"), active-turn status in faction terms ("Freeholders (bot) to act"), and resource tiles with owner attribution. `Seat 0`/`seat_0` disappear from normal-mode surfaces (dev panel keeps seat vocabulary).
3. Cross-artifact boundary under audit: the `ui.rs` UI-metadata channel (per game) and the `crates/wasm-api/src/lib.rs` catalog/seat projection. 13 games carry `ui.rs` (`games/*/src/ui.rs`); EF + flood_watch get real faction labels, the rest get an audit row (games without factions use existing player naming).
4. FOUNDATIONS §2: seat/faction display labels are Rust output projected through `ui.rs`; TS renders them and never derives faction identity from seat indices itself.
5. Schema extension: `UiMetadata` (per game) and the wasm catalog/seat projection gain seat/faction label fields. Consumers: the TS shell (`main.tsx`, board components). Additive — games without faction labels fall back to existing player naming; no consumer breaks. Shared files `crates/wasm-api/src/lib.rs` and `apps/web/src/main.tsx` are also touched by ACTCONMAT-009 (variant selector) — coordinate the mechanical merge.

## Architecture Check

1. Projecting faction labels through the established `ui.rs` channel (the predecessor's per-game metadata surface) keeps match identity Rust-authored and consistent with how card/deck labels already flow, rather than hardcoding faction strings in TS.
2. No shim: seat vocabulary is removed from normal-mode copy, not aliased; the dev panel retains seat IDs as the single remaining seat-vocabulary surface.
3. `engine-core` untouched (faction is a `games/*` noun, never the kernel — §3). No `game-stdlib` promotion; seat-label backfill stays per game.

## Verification Layers

1. No `Seat N`/`seat_0` in normal-mode surfaces -> codebase grep-proof + UI smoke (faction-first copy assertion).
2. Faction labels originate in Rust `ui.rs` -> schema/serialization validation of the `ui.rs` projection.
3. Identity line + owner-attributed resource tiles render -> UI smoke (`apps/web/e2e/event-frontier.smoke.mjs`).
4. Seat-label backfill audit covers all 13 `games/*/src/ui.rs` -> codebase grep-proof (one audit row per game).

## What to Change

### 1. Seat/faction labels in ui.rs

Add seat/faction display labels to `games/event_frontier/src/ui.rs` `UiMetadata`; backfill the other 12 `games/*/src/ui.rs` with seat labels or an explicit audit row (factionless games keep player naming).

### 2. wasm-api projection

Project seat/faction labels through `crates/wasm-api/src/lib.rs` so the shell reads them from the view/catalog.

### 3. Faction-first shell copy

In `main.tsx`: setup seats as "Charter — you (local)" / "Freeholders — bot"; mode/status line in faction terms ("Freeholders (bot) to act"); waiting-state copy fix (O11: replace "No legal actions available." with a faction-framed waiting message). Remove `Seat N`/`seat_0` from normal-mode copy (dev panel keeps them).

### 4. Resource-tile owner attribution

In `EventFrontierBoard.tsx`: an identity line ("You play the Charter") and resource tiles with owner attribution ("Funds — Charter (you)").

## Files to Touch

- `games/event_frontier/src/ui.rs` (modify)
- `games/*/src/ui.rs` (modify; 12 sibling games — seat-label backfill / audit row)
- `crates/wasm-api/src/lib.rs` (modify; seat/faction label projection)
- `apps/web/src/main.tsx` (modify)
- `apps/web/src/components/EventFrontierBoard.tsx` (modify)
- `apps/web/e2e/event-frontier.smoke.mjs` (modify; faction-first copy + waiting-state assertions)

## Out of Scope

- Variant selector / picker engine-string removal (ACTCONMAT-009) — though it shares `wasm-api/lib.rs` and `main.tsx`.
- TurnReportPanel narration (ACTCONMAT-006).
- Any change to seat→faction *pinning* in Rust (Charter stays `seat_0`); this ticket is display only.

## Acceptance Criteria

### Tests That Must Pass

1. `apps/web/e2e/event-frontier.smoke.mjs` asserts the UI states which faction the local viewer plays and who is acting, in faction terms, and that `Seat N` appears only in the dev panel.
2. Grep-proof: no `seat_0`/`Seat 0` in normal-mode shell/board copy (dev panel allowlisted).
3. `cargo test -p event_frontier` (ui metadata) + `npm --prefix apps/web run smoke:e2e` green.

### Invariants

1. Faction identity is Rust-authored via `ui.rs`; TS renders, never derives it from seat indices (§2).
2. Every `games/*/src/ui.rs` carries a seat-label entry or an explicit audit row (no silent skip).

## Test Plan

### New/Modified Tests

1. `games/event_frontier/tests/` (ui metadata) — seat/faction labels present and viewer-safe.
2. `apps/web/e2e/event-frontier.smoke.mjs` — faction-first copy, owner-attributed tiles, waiting-state copy.

### Commands

1. `cargo test -p event_frontier`
2. `npm --prefix apps/web run smoke:e2e`
3. `npm --prefix apps/web run smoke:ui`

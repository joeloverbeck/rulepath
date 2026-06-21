# SEAT-005: Migrate per-board play-area seat labels to the shared resolver

**Status**: COMPLETED
**Priority**: HIGH
**Effort**: Medium
**Engine Changes**: Yes — `apps/web` only (per-board `seatLabel()` helpers and seat-label call sites)
**Deps**: SEAT-003

## Problem

Each board formats play-area seat labels with its own inline logic, diverging
across games:
- 0-based "Seat N": `HighCardDuelBoard`, `PlainTricksBoard`, `SecretDraftBoard`,
  `PokerLiteBoard`, `TokenBazaarBoard`, `MaskedClaimsBoard`, `FloodWatchBoard`.
- 1-based "Player N": `RaceBoard:111-112`, `DirectionalFlipBoard:414-415`.
- 1-based "Seat N" via `slice(-1)+1`: `BriarCircuitBoard:379`.
- Regex/map-based (already Rust-driven): `DraughtsLiteBoard`, `EventFrontierBoard`,
  `RiverLedgerBoard`.

This is the duplication `docs/UI-INTERACTION.md` §3/§10B forbids (TS must display
Rust-supplied labels). Every play-area `seatLabel()` must consume the shared
resolver (SEAT-003), so each board renders the Rust-authored label (1-based "Seat N"
default, "Player N" for Race/Directional, custom for the multi-seat games).

## Assumption Reassessment (2026-06-21)

1. Boards/aux files with local seat-label logic (grep
   `function seatLabel\|const seatLabel\|VIEWER_OPTIONS\|seat === "seat_0"\|seat.slice`
   over `apps/web/src/components`): `BriarCircuitBoard`, `DirectionalFlipBoard`,
   `DraughtsLiteBoard`, `EventFrontierBoard`, `FloodWatchBoard`, `HighCardDuelBoard`,
   `MaskedClaimsBoard`, `PlainTricksBoard`, `PokerLiteBoard`, `RaceBoard`,
   `RiverLedgerBoard`, `SecretDraftBoard`, `TokenBazaarBoard`, plus
   `ModeControls.tsx` and `outcomeExplanationTemplates.ts`. Confirm and migrate
   each play-area seat-label call site.
2. `VIEWER_OPTIONS` labels are viewer surfaces handled by SEAT-004; this ticket
   covers play-area `seatLabel()` only. Where a file has both, migrate only the
   play-area helper here.
3. Shared boundary: the resolver from SEAT-003 over the `{seat,label}` contract.
   `EventFrontierBoard`/`RiverLedgerBoard`/`DraughtsLiteBoard` already consume
   Rust labels — adopt the shared resolver for them too, deleting their bespoke
   lookups for consistency (not strictly required for correctness, but removes the
   remaining duplication this series exists to eliminate).
8. Adjacent: `outcomeExplanationTemplates.ts` may embed seat labels in explanation
   text; if it formats a seat label, route it through the resolver, else leave it.

## Architecture Check

1. Replacing every per-board formatter with the shared resolver eliminates the
   divergence (0-based vs 1-based vs "Player") at its root; boards become pure
   presenters.
2. No backwards-compatibility shim: the per-board `seatLabel()` helpers are
   deleted, not kept as fallbacks.
3. `apps/web` presentation only; no Rust/legality change. Per-game naming intent
   ("Player N", custom multi-seat labels) is preserved because it now comes from
   Rust (SEAT-002 and existing overrides).

## Verification Layers

1. Every migrated board renders Rust-supplied labels -> UI smoke
   (`npm --prefix apps/web run smoke:ui`).
2. No board-local seat-label invention remains -> codebase grep-proof
   (`grep -rn 'seat === "seat_0"\|seat.slice(-1)\|"Player 1"\|"Seat 0"' apps/web/src/components`
   returns nothing in play-area helpers).
3. Type-check and build pass -> build (`npm --prefix apps/web run build`).

## What to Change

### 1. Replace each board's play-area seat-label formatter

In each listed board, replace the local `seatLabel()` (or inline ternary/slice/
regex) with `resolveSeatLabel(seat, sources)` from SEAT-003, sourcing labels from
the view payload + game catalog already available to the board.

### 2. Adopt the resolver in the already-Rust-driven boards

`EventFrontierBoard`, `RiverLedgerBoard`, `DraughtsLiteBoard` adopt the shared
resolver in place of their bespoke lookups, to remove the last of the duplication.

## Files to Touch

- `apps/web/src/components/BriarCircuitBoard.tsx` (modify)
- `apps/web/src/components/DirectionalFlipBoard.tsx` (modify)
- `apps/web/src/components/DraughtsLiteBoard.tsx` (modify)
- `apps/web/src/components/EventFrontierBoard.tsx` (modify)
- `apps/web/src/components/FloodWatchBoard.tsx` (modify)
- `apps/web/src/components/HighCardDuelBoard.tsx` (modify — play-area helper only)
- `apps/web/src/components/MaskedClaimsBoard.tsx` (modify)
- `apps/web/src/components/PlainTricksBoard.tsx` (modify)
- `apps/web/src/components/PokerLiteBoard.tsx` (modify)
- `apps/web/src/components/RaceBoard.tsx` (modify)
- `apps/web/src/components/RiverLedgerBoard.tsx` (modify)
- `apps/web/src/components/SecretDraftBoard.tsx` (modify)
- `apps/web/src/components/TokenBazaarBoard.tsx` (modify)
- `apps/web/src/components/ModeControls.tsx` (modify — if it formats a seat label)
- `apps/web/src/components/outcomeExplanationTemplates.ts` (modify — if it formats a seat label)

## Out of Scope

- VIEWER panel / viewer-mode lists (SEAT-004).
- Any Rust change.

## Acceptance Criteria

### Tests That Must Pass

1. `npm --prefix apps/web run smoke:ui`
2. `npm --prefix apps/web run smoke:effects`
3. `npm --prefix apps/web run build`

### Invariants

1. No board invents seat labels; all come from the resolver over Rust data.
2. Per-game naming intent ("Player N", custom multi-seat labels) is preserved via
   Rust-supplied labels.

## Test Plan

### New/Modified Tests

1. UI smoke extended to assert play-area labels match catalog `seat_labels` for a
   representative game per scheme (Briar "Seat 1".."Seat 4"; Race "Player 1"/"Player 2").

### Commands

1. `npm --prefix apps/web run smoke:ui`
2. `npm --prefix apps/web run build`

## Outcome

Completed: 2026-06-21

Changed:
- Routed board play-area labels through `resolveSeatLabel` across the SEAT-005
  board set, including Race, Directional Flip, Briar Circuit, Draughts Lite,
  Event Frontier, Flood Watch, High Card Duel, Masked Claims, Plain Tricks,
  Poker Lite, River Ledger, Secret Draft, and Token Bazaar.
- Migrated the additional board-local raw-seat formatters found during grep in
  Column Four, Three Marks, and Frontier Control so their play-area text also
  uses the shared resolver.
- Passed catalog seat labels from the selected game/replay catalog into boards
  whose view payloads do not carry labels, preserving Rust-authored `Player N`
  labels for Race and Directional Flip and one-based `Seat N` labels for default
  games.
- Updated mode/status and outcome-template display helpers that formatted raw
  `seat_N` tokens so they use the resolver instead of independent 0-based copy.
- Updated e2e assertions for the new one-based labels and extended
  `smoke-ui.mjs` with representative catalog assertions for Race (`Player 1/2`)
  and Briar Circuit (`Seat 1..4`).

Deviations:
- The ticket's file list omitted Column Four, Three Marks, and Frontier Control,
  but they still contained play-area `seat_N` display formatting and were
  migrated to satisfy the invariant that boards do not invent seat labels.
- A focused grep still finds `seat === "seat_0"` in non-label control/indexing
  logic and setup-only copy in `MatchSetup`; no migrated play-area label helper
  keeps the old hardcoded `Seat 0`/`Player 1` formatter.

Verification:
- `npm --prefix apps/web run build` passed.
- `npm --prefix apps/web run smoke:ui` passed.
- `npm --prefix apps/web run smoke:effects` passed.
- `npm --prefix apps/web run smoke:e2e` passed after the assertion updates.
- Focused grep over `apps/web/src/components` showed no remaining play-area
  hardcoded `Seat 0`, `Player 1`, `Player 2`, or `seat.slice(-1)` label
  formatter; remaining `seat === "seat_0"` matches are data selection, role
  suffix, or setup surfaces.

# RIVLEDSHOWUX-005: Player-facing seat-ledger display fields; remove redundant bar

**Status**: PENDING
**Priority**: MEDIUM
**Effort**: Medium
**Engine Changes**: Yes — `games/river_ledger/src/ui.rs`, `games/river_ledger/src/state.rs`, `crates/wasm-api/src/lib.rs`, `apps/web/src/wasm/client.ts`, `apps/web/src/components/RiverLedgerBoard.tsx`
**Deps**: None

## Problem

Seat panels show raw counters `Street` / `Total` / `Private` (`RiverLedgerBoard.tsx:217-219`) plus a duplicate unlabeled contribution track-bar (`:255-264`). Replace the counters with player-facing Rust-authored labels (`This round` / `Hand total` / `Hole cards: N hidden|revealed`, role/status badges) and remove the redundant bar (or relabel it value-equal with no chip/money framing).

## Assumption Reassessment (2026-06-16)

1. Verified: `RiverLedgerBoard.tsx:217-219` renders `<Metric label="Street|Total|Private" .../>` from `seat.street_contribution` / `total_contribution` / `hidden_hole_count`; the unlabeled track-bar is `:255-264` (`river-ledger-track-bar`).
2. Verified against spec §6 D4 + §8 WB5; `RULES.md` `RL-UI-LEDGER-001`, `RL-UI-NOCASINO-001` (abstract units, no chips/cash).
3. Shared boundary under audit: the seat public-view projection — the new `RiverLedgerSeatLedgerDisplay` fields are additive; the rendered values stay the same public counts.
4. FOUNDATIONS §7 (player-facing, not raw-counter diagnostic) + §10 (`RL-UI-NOCASINO-001`: abstract units, no chip/money framing) motivate this; values stay abstract.
5. Schema extension: the seat view gains additive `RiverLedgerSeatLedgerDisplay` (`round_contribution`/`hand_contribution`/`hole_card_summary` label+value+a11y, `role_badges`, `status_label`); consumer is `RiverLedgerBoard.tsx`; additive-only.

## Architecture Check

1. Rust-authored ledger labels keep player-facing copy Rust-owned and abstract-unit-safe; removing the duplicate bar reduces an unlabeled second representation of the same Rust value.
2. No shims; the raw `<Metric>` counters and the unlabeled bar are replaced, not aliased.
3. `engine-core` untouched (§3); no `game-stdlib` change (§4).

## Verification Layers

1. Seat panels read `This round` / `Hand total` / `Hole cards: N hidden`; no raw `Street`/`Total`/`Private`; no unlabeled bar -> `npm --prefix apps/web run smoke:ui`.
2. No chip/money/cash framing in seat copy -> `node scripts/check-presentation-copy.mjs` (no-casino audit).
3. Hole-card summary leaks no count of revealed private cards pre-authorization -> `games/river_ledger/tests/visibility.rs`.

## What to Change

### 1. `games/river_ledger/src/{ui,state}.rs`

Add `RiverLedgerSeatLedgerDisplay`: `round_contribution {label:"This round",value}`, `hand_contribution {label:"Hand total",value}`, `hole_card_summary {label:"Hole cards",value:"N hidden|revealed",accessibility_label}`, `role_badges`, `status_label`.

### 2. `crates/wasm-api/src/lib.rs` + `apps/web/src/wasm/client.ts`

Project the seat-ledger display additively; add the TS type.

### 3. `apps/web/src/components/RiverLedgerBoard.tsx`

Render the player-facing labels; remove the unlabeled track-bar (`:255-264`) or relabel it value-equal with no chip/money implication.

## Files to Touch

- `games/river_ledger/src/ui.rs` (modify)
- `games/river_ledger/src/state.rs` (modify)
- `crates/wasm-api/src/lib.rs` (modify)
- `apps/web/src/wasm/client.ts` (modify)
- `apps/web/src/components/RiverLedgerBoard.tsx` (modify)

## Out of Scope

- Action rows (RIVLEDSHOWUX-003); board-slot labels (RIVLEDSHOWUX-004); table recomposition (RIVLEDSHOWUX-011).

## Acceptance Criteria

### Tests That Must Pass

1. `cargo test -p river_ledger` + `cargo run -p fixture-check -- --game river_ledger` — seat-ledger display additive; hole-card summary reveal-safe.
2. `npm --prefix apps/web run smoke:ui` + `npm --prefix apps/web run build` — player-facing labels render; bar removed/relabeled; type-checks.
3. `node scripts/check-presentation-copy.mjs` — no chip/money/cash framing.

### Invariants

1. Seat-ledger labels are Rust-authored and abstract-unit-safe; no chip/money framing (§10, `RL-UI-NOCASINO-001`).
2. Hole-card summary reveals no private-card identity, only an authorized count (§11).

## Test Plan

### New/Modified Tests

1. `games/river_ledger/tests/visibility.rs` — hole-card summary count is reveal-scoped.
2. `apps/web/e2e/river-ledger.smoke.mjs` (modify, as surfaced) — seat-ledger label assertion; no unlabeled bar.

### Commands

1. `cargo test -p river_ledger`
2. `npm --prefix apps/web run smoke:ui`
3. `node scripts/check-presentation-copy.mjs`

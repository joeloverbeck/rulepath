# RIVLEDSHOWUX-004: Rust-authored board-slot placeholder view

**Status**: PENDING
**Priority**: MEDIUM
**Effort**: Small
**Engine Changes**: Yes — `games/river_ledger/src/state.rs`, `games/river_ledger/src/visibility.rs`, `games/river_ledger/src/ui.rs`, `crates/wasm-api/src/lib.rs`, `apps/web/src/wasm/client.ts`, `apps/web/src/components/RiverLedgerBoard.tsx`
**Deps**: None

## Problem

Unrevealed board slots render a bare `<strong>Pending</strong>` (`RiverLedgerBoard.tsx:113-115`) with no Rust-authored street-specific label and no accessibility text. Rust should author each slot's placeholder + accessibility label (e.g. `Flop pending` / "Unrevealed turn card. It is not available yet."); the card identity stays `null` until Rust reveals it.

## Assumption Reassessment (2026-06-16)

1. Verified: `RiverLedgerBoard.tsx:113-115` renders a single hardcoded "Pending" placeholder; no street/a11y label arrives from Rust; the hidden-board markup (`:113`) carries a `river-ledger-card hidden` class only.
2. Verified against spec §6 D3 + §8 WB4; `RULES.md` `RL-DEAL-BOARD-002`, `RL-VIS-DECKTAIL-001` (future board stays internal).
3. Shared boundary under audit: the public-view board projection (`visibility.rs` → `crates/wasm-api`); the new `RiverLedgerBoardSlotView` is additive and carries no future-card identity.
4. FOUNDATIONS §7 (CSS-safe, readable placeholder, not a clipped diagnostic) + §11 motivate this; TS renders the Rust label.
5. No-leak / determinism: confirm the slot view emits `card: null` for `reveal_state: "pending"` — no future board identity reaches DOM, `aria-label`, or `data-testid` (§11 no-leak firewall; `RL-VIS-DECKTAIL-001`).
6. Schema extension: the public view gains an additive `RiverLedgerBoardSlotView` (`slot`, `reveal_state`, `street_label`, `visual_placeholder_label`, `accessibility_label`, nullable `card`); consumer is `RiverLedgerBoard.tsx`; additive-only.

## Architecture Check

1. Rust-authored slot labels (vs a TS-hardcoded "Pending") keep all player-facing copy Rust-owned and let the placeholder be street-specific; the nullable `card` keeps the no-leak contract structural.
2. No shims; the hardcoded TS placeholder is replaced by the rendered Rust label.
3. `engine-core` untouched (§3); no `game-stdlib` change (§4).

## Verification Layers

1. Pending slots render a CSS-safe single label + accessibility text -> `npm --prefix apps/web run smoke:ui`.
2. No future-card identity in DOM / a11y / test IDs -> `node apps/web/e2e/a11y-noleak.smoke.mjs` (pending-slot no-leak assertion).
3. Board-slot projection is additive and viewer-safe -> `games/river_ledger/tests/visibility.rs`.

## What to Change

### 1. `games/river_ledger/src/{state,visibility,ui}.rs`

Add `RiverLedgerBoardSlotView` to the public board projection: `reveal_state`, `street_label`, `visual_placeholder_label` ("Pending"), `accessibility_label`, nullable `card`. No future-card identity for pending slots.

### 2. `crates/wasm-api/src/lib.rs` + `apps/web/src/wasm/client.ts`

Project the slot view additively; add the TS type.

### 3. `apps/web/src/components/RiverLedgerBoard.tsx`

Render the Rust placeholder label + accessibility text in a CSS-safe single-label layout (replace `:113-115`).

## Files to Touch

- `games/river_ledger/src/state.rs` (modify)
- `games/river_ledger/src/visibility.rs` (modify)
- `games/river_ledger/src/ui.rs` (modify)
- `crates/wasm-api/src/lib.rs` (modify)
- `apps/web/src/wasm/client.ts` (modify)
- `apps/web/src/components/RiverLedgerBoard.tsx` (modify)

## Out of Scope

- Seat-ledger labels (RIVLEDSHOWUX-005); action rows (RIVLEDSHOWUX-003); table recomposition (RIVLEDSHOWUX-011).

## Acceptance Criteria

### Tests That Must Pass

1. `cargo test -p river_ledger` + `cargo run -p fixture-check -- --game river_ledger` — board-slot view additive; no future-card identity for pending slots.
2. `npm --prefix apps/web run smoke:ui` + `npm --prefix apps/web run build` — CSS-safe placeholder renders; type-checks.
3. `node apps/web/e2e/a11y-noleak.smoke.mjs` — pending slots leak no future-card identity.

### Invariants

1. Pending slot `card` is `null`; no future board identity reaches any viewer surface (§11).
2. The placeholder label and accessibility text are Rust-authored; TS renders only (§2).

## Test Plan

### New/Modified Tests

1. `games/river_ledger/tests/visibility.rs` — pending board slot carries label + a11y text, `card: null`.
2. `apps/web/e2e/a11y-noleak.smoke.mjs` (modify, as surfaced) — pending-slot no-leak assertion.

### Commands

1. `cargo test -p river_ledger`
2. `npm --prefix apps/web run smoke:ui`
3. `cargo run -p fixture-check -- --game river_ledger`

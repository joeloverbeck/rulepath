# RIVLEDSHOWUX-001: "Seat N" label helper for clean showdown/visibility strings

**Status**: PENDING
**Priority**: HIGH
**Effort**: Medium
**Engine Changes**: Yes — `games/river_ledger/src/ids.rs`, `games/river_ledger/src/ui.rs`, `games/river_ledger/src/showdown.rs`, `games/river_ledger/src/visibility.rs`, `games/river_ledger/tests/{rules,visibility,serialization}.rs`
**Deps**: None

## Problem

River Ledger's Rust-authored terminal strings are built from `RiverLedgerSeat::as_str()`, which returns the internal id `"seat_N"` (`ids.rs:43`). So `showdown.rs:215` (`format!("{} wins with {hand}.", winner.as_str())`) and the `seat_list` helper (`showdown.rs:518-524`) literally emit `"seat_5 wins with Pair of Jacks."` into player-facing copy — masked only at runtime by a TS fallback regex. This ticket makes those strings born clean by routing them through a "Seat N" display label, with **no new label type** (the `seat_labels` "Seat N" form already exists; spec D1).

## Assumption Reassessment (2026-06-16)

1. Verified: `RiverLedgerSeat::as_str()` returns `format!("seat_{}", self.index)` (`games/river_ledger/src/ids.rs:43`); leak sites are `showdown.rs:206,215,225` + `seat_list` (`:518-524`). The catalog already projects `{"seat":"seat_N","label":"Seat N"}` via `catalog_seat_labels_json` (`crates/wasm-api/src/lib.rs:449-457`).
2. Verified against spec §6 D1 + §8 WB1 + §2 row #1; `RULES.md` `RL-UI-SHOWDOWN-001`, `RL-UI-NOLEAK-001`.
3. Shared boundary under audit: the display label is presentation-only; raw `seat_N` MUST remain the internal id and the stable serialization/hash key (`state.rs:289`, `replay_support.rs`), so the helper adds a display form without touching serialized forms.
4. FOUNDATIONS §2 (Rust authors all player-facing strings) + §11 (no raw internal id in visible text / accessibility labels) motivate this ticket; TypeScript computes no label.
5. Determinism/serialization surface: confirm the new label helper feeds only display/explanation strings; golden traces and serialized saves keep `seat_N` keys unchanged, so replay/hash stays byte-identical (§11).

## Architecture Check

1. A small `ids.rs`/`ui.rs` helper returning "Seat N" — reused by every showdown/visibility string — is cleaner than per-call-site string surgery and avoids a conflicting new `SeatDisplayLabel` shape (the existing TS type is `{seat,label}`).
2. No backwards-compatibility shims; the `as_str()`-in-copy pattern is replaced, not aliased. The TS fallback normalization stays only as legacy safety for other games.
3. `engine-core` untouched (§3); no `game-stdlib` change (§4) — the helper is `games/river_ledger`-local.

## Verification Layers

1. No `seat_\d+` in any Rust-authored visible/a11y showdown string -> unit test asserting headline / seat-list / comparison strings read "Seat N".
2. Serialization/replay unchanged -> `cargo run -p replay-check -- --game river_ledger --all` + golden-trace diff shows display strings change while serialized `seat_N` keys do not.
3. No-leak projection intact -> `games/river_ledger/tests/visibility.rs` (folded/observer strings carry "Seat N", no raw id).

## What to Change

### 1. `games/river_ledger/src/ids.rs` / `ui.rs`

Add a `seat_public_label(seat) -> String` helper returning `"Seat {index}"` (matching the existing `catalog_seat_labels_json` form). Keep `as_str()` for internal/serialization use only.

### 2. `games/river_ledger/src/showdown.rs` / `visibility.rs`

Route the headline (`:215`), `seat_list` (`:518-524`), split/folded summaries (`:206,225`), and per-seat comparison notes through `seat_public_label`, never `as_str()`, for every visible/accessibility string.

## Files to Touch

- `games/river_ledger/src/ids.rs` (modify)
- `games/river_ledger/src/ui.rs` (modify)
- `games/river_ledger/src/showdown.rs` (modify)
- `games/river_ledger/src/visibility.rs` (modify)
- `games/river_ledger/tests/rules.rs` (modify)
- `games/river_ledger/tests/visibility.rs` (modify)
- `games/river_ledger/tests/serialization.rs` (modify)

## Out of Scope

- The runtime/audit guards that fail `\bseat_\d+\b` (RIVLEDSHOWUX-002).
- The V2 showdown payload (RIVLEDSHOWUX-007).
- The status-line `Player N → Seat N` fix (RIVLEDSHOWUX-006).

## Acceptance Criteria

### Tests That Must Pass

1. `cargo test -p river_ledger` — new unit test proves no `seat_\d+` in authored showdown/visibility strings; existing tests stay green.
2. `cargo run -p replay-check -- --game river_ledger --all` — replay/hash byte-identical (serialized `seat_N` keys unchanged).
3. `cargo run -p fixture-check -- --game river_ledger` — fixtures pass.

### Invariants

1. Raw `seat_N` appears only as an internal id / serialization key, never in a visible or accessibility string (§11).
2. TypeScript authors no seat label; the string is Rust-born (§2).

## Test Plan

### New/Modified Tests

1. `games/river_ledger/tests/rules.rs` — assert showdown headline / seat-list read "Seat N", not "seat_N".
2. `games/river_ledger/tests/visibility.rs` — observer/folded projections carry "Seat N" with no raw id.
3. `games/river_ledger/tests/serialization.rs` — serialized form still keys seats by `seat_N` (display change does not alter canonical form).

### Commands

1. `cargo test -p river_ledger`
2. `cargo run -p replay-check -- --game river_ledger --all`
3. `cargo run -p fixture-check -- --game river_ledger` (the serialization/replay boundary is where a display-vs-canonical regression would surface)

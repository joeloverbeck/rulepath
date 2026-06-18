# RIVLEDSHOSEA-005: Project active-match seat labels through Rust/WASM

**Status**: COMPLETED
**Priority**: HIGH
**Effort**: Medium
**Engine Changes**: Yes — `games/river_ledger/src/ui.rs`, `src/visibility.rs`; `crates/wasm-api/src/lib.rs`; `apps/web/src/wasm/client.ts`
**Deps**: RIVLEDSHOSEA-002

## Problem

`ui_metadata()` emits `seat_labels(STANDARD_MAX_SEATS)` — six capability labels — for every catalog entry and view-metadata instance, and the shared shell consumes that capability list as the active-match list. A four-seat match therefore advertises six seats. The seam carries only catalog `seat_labels` and the current `active_seat` actor ID; there is no match-scoped active-seat-label projection. This ticket adds a Rust-owned projection of exactly the active match's seats, in authoritative order, with one-based public labels, and types it across the WASM seam (spec §8.2 / D4; reassessment finding M1: the WASM field is required and Rust-authored, not a client-side slice of catalog labels).

## Assumption Reassessment (2026-06-18)

1. `games/river_ledger/src/ui.rs::ui_metadata()` calls `seat_labels(STANDARD_MAX_SEATS)` (`STANDARD_MAX_SEATS = 6` in `src/ids.rs`; `STANDARD_MIN_SEATS = 3`; supported counts `3,4,5,6`). No `ui_metadata_for_seat_count(count)` / `active_seat_labels(state)` exists yet — both are new. Confirmed.
2. `crates/wasm-api/src/lib.rs` projects catalog seat labels via `catalog_seat_labels_json(seat_count)` and the current actor as `active_seat`; `apps/web/src/wasm/client.ts` exports `SeatDisplayLabel = { seat; label }` but the view envelope has no match-scoped active-seat-label field (only `active_seat: ViewerSeatId | null`). Confirmed. This is the M1 gap: the active list must become a new Rust-owned typed field.
3. Shared boundary under audit: the WASM view-envelope JSON contract between `crates/wasm-api` and `apps/web/src/wasm/client.ts`. End state: a typed `active_seat_labels` (or equivalently named) field carrying exactly the match's seats in Rust order, distinct from catalog `seat_labels`.
4. FOUNDATIONS §2 (Rust owns view projection; TS presentation-only): the active-seat inventory is a Rust-authored fact. TypeScript must not derive it by slicing catalog labels to the selected count — that would let TS decide which seats are active. Restated; the new field exists precisely to keep that authority in Rust.
5. No-leak surface (§11): the active-label rows contain only public seat ID + public label. They must not include private cards, non-public roles, or viewer authorization. `src/visibility.rs` is the projection site; the new rows are added to the viewer-safe public projection.
6. Schema extension: this additively extends the WASM view envelope with one new typed field. Consumers are `apps/web/src/wasm/client.ts` (type) and the shell components (RIVLEDSHOSEA-006/007). The extension is additive-only (new field; absent/empty implies "no active match → use catalog").

## Architecture Check

1. A dedicated match-scoped projection keeps catalog capability metadata (still advertising up to six seats) cleanly separate from match inventory, so neither surface has to infer the other. A new typed field is more robust than overloading `active_seat` or re-deriving from count in TS.
2. No shim: catalog `seat_labels` is not repurposed as the active list; the active list is its own field.
3. `engine-core` stays free of mechanic nouns — the projection is a `games/river_ledger` view-metadata builder feeding the generic seam; no kernel change, no `game-stdlib` promotion.

## Verification Layers

1. Active rows match the match -> `tests/serialization.rs` asserting the projection yields exactly the active seats (counts `3,4,5,6`) in `state.seats`/authoritative order with one-based labels.
2. Count validated in Rust -> `tests/rules.rs`/setup tests asserting an unsupported count cannot produce an active-seat list (the browser never obtains one).
3. No private data in label rows -> `tests/visibility.rs` asserting observer and seat-private projections carry identical active public-label rows and no private payload in those rows.
4. Seam typed + deterministic -> grep-proof of the new field in `crates/wasm-api/src/lib.rs` and `apps/web/src/wasm/client.ts`; `npm --prefix apps/web run smoke:wasm` green.

## What to Change

### 1. Rust active-seat-label projection

Add `ui_metadata_for_seat_count(count)` / `active_seat_labels(state)` (names per implementation) producing exactly the active state's seats in authoritative order with `seat_public_label`-derived labels; validate the count against supported `3,4,5,6`. Route it through `src/visibility.rs`'s viewer-safe public projection. Keep catalog capability metadata (min/default/max, supported counts, full label set) intact.

### 2. Type the seam

Add the smallest typed `active_seat_labels` field to the WASM view envelope in `crates/wasm-api/src/lib.rs`; mirror it in `apps/web/src/wasm/client.ts`. Update `WASM-CLIENT-BOUNDARY.md` only if that document's contract surface changes (closeout RIVLEDSHOSEA-010 reconciles docs).

## Files to Touch

- `games/river_ledger/src/ui.rs` (modify)
- `games/river_ledger/src/visibility.rs` (modify)
- `crates/wasm-api/src/lib.rs` (modify)
- `apps/web/src/wasm/client.ts` (modify)
- `games/river_ledger/tests/serialization.rs` (modify)
- `games/river_ledger/tests/visibility.rs` (modify)

## Out of Scope

- Consuming the active list in `SeatFrame`/`MatchSetup` (RIVLEDSHOSEA-006/007).
- The generic viewer callback and stale-selection normalization (RIVLEDSHOSEA-006).
- Any client-side derivation of the active list from catalog labels (explicitly forbidden by §2).
- Card containment (RIVLEDSHOSEA-009).

## Acceptance Criteria

### Tests That Must Pass

1. Rust serialization/visibility tests for active label rows at counts `3,4,5,6` (exact seats, authoritative order, one-based labels, no private data).
2. Setup validation still rejects every unsupported count; no active-seat list is produced for one.
3. `cargo test -p river_ledger` and `npm --prefix apps/web run smoke:wasm` green.

### Invariants

1. The active-seat-label projection is Rust-authored and contains exactly the match's seats in `state.seats` order; TypeScript performs no count-slice of catalog labels.
2. Active-label rows carry only public ID/label — no hidden information.

## Test Plan

### New/Modified Tests

1. `games/river_ledger/tests/serialization.rs` — active-label-row projection at counts `3,4,5,6`.
2. `games/river_ledger/tests/visibility.rs` — observer vs seat-private active rows identical and private-data-free.

### Commands

1. `cargo test -p river_ledger`
2. `npm --prefix apps/web ci && npm --prefix apps/web run smoke:wasm && npm --prefix apps/web run build`
3. `cargo run -p simulate -- --game river_ledger --seat-count 4 --games 1 --start-seed 1` to confirm a 4-seat match projects exactly four active rows.

## Outcome

Completed: 2026-06-18

What changed:
- Added Rust-owned `active_seat_labels(&state.seats)` projection that validates supported counts and emits exactly the active match seats in authoritative order with one-based public labels.
- Added `PublicView.active_seat_labels`, included it in stable serialization, and serialized it through the WASM River Ledger view envelope.
- Mirrored the additive `active_seat_labels` field in `apps/web/src/wasm/client.ts`.
- Added serialization and visibility coverage for counts `3..=6`, including observer/seat-private equality and private-data absence in the active-label rows.

Deviations:
- None. Catalog `ui.seat_labels` remains capability metadata for six seats; the new field is a distinct match-scoped projection.

Verification:
- `cargo test -p river_ledger active_seat_labels` passed.
- `cargo test -p wasm-api river_ledger_view_projects_terminal_rationale_template_keys` passed.
- `cargo test -p river_ledger` passed.
- `npm --prefix apps/web run smoke:wasm` passed.
- `npm --prefix apps/web run build` passed.
- `cargo run -p simulate -- --game river_ledger --seat-count 4 --games 1 --start-seed 1` passed and reported `seat_order=[seat_0,seat_1,seat_2,seat_3]`.
- `npm --prefix apps/web ci` passed; npm reported the existing single low-severity vulnerability.
- `cargo fmt --all --check` passed.
- `cargo clippy --workspace --all-targets -- -D warnings` passed.
- `cargo build --workspace` passed.

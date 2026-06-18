# RIVLEDSHOSEA-002: Unify Rust public seat labels and close the seed-10018 contradiction

**Status**: COMPLETED
**Priority**: HIGH
**Engine Changes**: Yes — `games/river_ledger/src/ui.rs`; `apps/web/src/components/RiverLedgerBoard.tsx` (presentation); `apps/web/e2e/river-ledger.smoke.mjs`
**Effort**: Medium
**Deps**: RIVLEDSHOSEA-001

## Problem

The shipped baseline carries two incompatible public seat-numbering systems. `games/river_ledger/src/ui.rs::seat_public_label(seat)` renders `Seat {index + 1}` (one-based) and is used in showdown narration, while `seat_labels(count)` in the same file emits `label: format!("Seat {index}")` (zero-based) for catalog/view metadata, and `apps/web/src/components/RiverLedgerBoard.tsx::seatLabel()` derives a label by stripping `seat_` (`Seat ${seat.replace("seat_", "")}`, also zero-based). The generic Outcome heading is built in TypeScript from internal winner IDs, while the V2 banner is authored in Rust with `seat_public_label`. Result: the same internal winner is narrated as both "Seat 0 wins" and "Seat 1 wins" (spec §2.2, reproduced by seed `10018`). This is a player-facing identity defect, not copy polish.

## Assumption Reassessment (2026-06-18)

1. `games/river_ledger/src/ui.rs`: `seat_public_label(seat: RiverLedgerSeat) -> String` returns `format!("Seat {}", seat.index() + 1)` (one-based); `seat_labels(count: u8) -> Vec<SeatDisplayLabel>` returns `label: format!("Seat {index}")` (zero-based). `RiverLedgerSeat::from_index(index: usize) -> Option<Self>` exists in `src/ids.rs` (bounded by `STANDARD_MAX_SEATS = 6`). Confirmed.
2. `apps/web/src/components/RiverLedgerBoard.tsx::seatLabel(seat)` returns `` `Seat ${seat.replace("seat_", "")}` `` — the React ID-parse path the spec forbids (§3.3, §16.3). `SeatDisplayLabel = { seat; label }` is exported from `apps/web/src/wasm/client.ts` and projected on catalog/view metadata. Confirmed.
3. Shared boundary under audit: the Rust-authored public-label contract (`seat_public_label`) versus its catalog/view projection (`seat_labels`) versus the React consumer. End state: one Rust label function feeds catalog, view, status, terminal, accessibility, and narration; React reads the projected label and never transforms an ID.
4. FOUNDATIONS §2 (Rust owns view projection; TypeScript presentation-only): deriving a public label by parsing `seat_N` in React is TS inventing presentation identity. Restated before trusting the spec; the fix moves the single label authority into Rust and makes React a pure consumer.
5. Migration consistency (spec D1): public labels stay one-based (`Seat 1`–`Seat 6`); internal IDs stay zero-based (`seat_0`–`seat_5`). The catalog `seat_labels` generator is corrected to the one-based convention so visible, accessibility, and browser assertions migrate together — no mixed zero-/one-based intermediate state.

## Architecture Check

1. Delegating `seat_labels` to `seat_public_label` gives one label authority; correcting the generator (not the narration) keeps the already-shipped one-based prose stable while removing the zero-based divergence. Cleaner than special-casing the Outcome heading.
2. No shim: the React `seat.replace("seat_", "")` path is deleted, not wrapped; a missing projected label fails to a neutral non-identity string with a dev assertion, never the raw ID.
3. `engine-core` untouched; label vocabulary stays in `games/river_ledger`.

## Verification Layers

1. One-source labels in Rust -> `ui.rs` unit test over indices `0..STANDARD_MAX_SEATS` asserting each generated `(seat_id, label)` equals `(format!("seat_{i}"), seat_public_label(RiverLedgerSeat::from_index(i)))`.
2. No React ID-to-label parsing -> codebase grep-proof that `seat.replace("seat_"` is gone from `RiverLedgerBoard.tsx`; `node scripts/check-presentation-copy.mjs` passes.
3. Seed `10018` coherence -> `tests/rules.rs` asserting internal winner `[seat_0]` is publicly **Seat 1** everywhere and closest challenger (internal `seat_2`) is **Seat 3**; browser assertion in `river-ledger.smoke.mjs` that status, Outcome, V2 banner, decisive reason, and standings agree and no string says "Seat 0 wins".

## What to Change

### 1. `ui.rs` — delegate label generation

Rewrite `seat_labels(count)` so each row's `label` comes from `seat_public_label(RiverLedgerSeat::from_index(index))` (handling the `Option`), making catalog/view labels one-based and identical to narration. Add the index-range consistency unit test.

### 2. `RiverLedgerBoard.tsx` — consume projected labels

Delete `seatLabel()`'s `seat.replace("seat_", "")` path. Build a label lookup from the projected `SeatDisplayLabel` rows and use it for status heading, generic Outcome heading/standings, private-view heading, seat-ledger headings, active-seat copy, and accessibility names. On a missing label, render a neutral non-identity fallback plus a dev assertion — never the raw ID.

### 3. Browser regression

Extend `river-ledger.smoke.mjs` with the seed-`10018` assertion that one consistent winner label appears across every surface and the live-region announcement carries no raw `seat_N` token.

## Files to Touch

- `games/river_ledger/src/ui.rs` (modify)
- `apps/web/src/components/RiverLedgerBoard.tsx` (modify)
- `apps/web/e2e/river-ledger.smoke.mjs` (modify)
- `games/river_ledger/tests/rules.rs` (modify)

## Out of Scope

- Canonical resolved-showdown assembly and invariant assertions (RIVLEDSHOSEA-003).
- Active-match seat-count scoping and the generic viewer callback (RIVLEDSHOSEA-005/006).
- Golden-trace/`RULE-COVERAGE.md` reconciliation (RIVLEDSHOSEA-004).
- Choosing zero-based public labels; the convention stays one-based (spec §16.2).

## Acceptance Criteria

### Tests That Must Pass

1. `ui.rs` label-consistency unit test over `0..STANDARD_MAX_SEATS`.
2. `tests/rules.rs` seed-`10018` case: winner `[seat_0]` is publicly **Seat 1**; closest challenger is **Seat 3**; full ledger awarded to `seat_0`.
3. `cargo test -p river_ledger` and `npm --prefix apps/web run smoke:e2e` (River Ledger smoke) green; `node scripts/check-presentation-copy.mjs` passes.

### Invariants

1. Catalog, view, status, terminal, accessibility, and narration labels for a given seat ID are byte-identical and one-based.
2. No TypeScript path derives a public seat label by parsing a `seat_N` ID.

## Test Plan

### New/Modified Tests

1. `games/river_ledger/src/ui.rs` (or `tests/rules.rs`) — all-seat label-consistency unit test.
2. `games/river_ledger/tests/rules.rs` — seed-`10018` unique-winner label coherence.
3. `apps/web/e2e/river-ledger.smoke.mjs` — cross-surface winner-label assertion, no raw `seat_N`.

### Commands

1. `cargo test -p river_ledger`
2. `npm --prefix apps/web ci && npm --prefix apps/web run build && npm --prefix apps/web run smoke:e2e`
3. `node scripts/check-presentation-copy.mjs` (narrower presentation-copy guard for the React label change).

## Outcome

Completed: 2026-06-18

What changed:
- `games/river_ledger/src/ui.rs` now derives every `seat_labels` row from the same one-based `seat_public_label()` authority used by showdown narration.
- `crates/wasm-api/src/lib.rs` now serializes River Ledger catalog seat labels from `river_ledger::ui_metadata()` instead of the generic zero-based catalog helper, so the shared seat frame and live view receive the same labels.
- `apps/web/src/components/RiverLedgerBoard.tsx` now builds a label lookup from Rust-projected `ui.seat_labels` and no longer parses `seat_N` IDs into display labels. Missing labels fall back to neutral `Seat` text with a dev-only assertion and no raw ID exposure.
- Added the seed-10018 native regression and extended `apps/web/e2e/river-ledger.smoke.mjs` so the generic heading, live outcome text, V2 showdown banner, decisive reason, and standings use the same one-based winner label and never say `Seat 0 wins`.
- Updated native visibility and wasm-api assertions to the one-based River Ledger public-label contract.

Deviations:
- The ticket named `games/river_ledger/src/ui.rs`, `RiverLedgerBoard.tsx`, `river-ledger.smoke.mjs`, and `tests/rules.rs`; satisfying the catalog/view metadata contract also required the narrow `wasm-api` River Ledger catalog serializer and existing visibility tests that locked the old catalog labels.

Verification:
- `cargo fmt --all --check` passed.
- `cargo test -p river_ledger` passed.
- `cargo test -p wasm-api` passed.
- `npm --prefix apps/web ci` passed; npm reported one existing low-severity audit item.
- `npm --prefix apps/web run build` passed.
- `node scripts/check-presentation-copy.mjs` passed.
- `node apps/web/e2e/river-ledger.smoke.mjs` passed.
- `npm --prefix apps/web run smoke:e2e` passed.
- `cargo clippy --workspace --all-targets -- -D warnings` passed.
- `cargo build --workspace` passed.

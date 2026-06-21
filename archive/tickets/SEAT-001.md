# SEAT-001: Make the default catalog seat labels 1-based

**Status**: COMPLETED
**Priority**: HIGH
**Effort**: Small
**Engine Changes**: Yes — `crates/wasm-api` (`catalog_seat_labels_json` seat-label JSON consumed by the web catalog payload and the VIEWER panel)
**Deps**: none

## Problem

The VIEWER panel in the web app shows 0-based seat buttons ("Seat 0".."Seat 3")
while the play area shows 1-based labels ("Seat 1".."Seat 4"). The VIEWER labels
come from the Rust catalog's default generator
`catalog_seat_labels_json` (`crates/wasm-api/src/catalog.rs:65-73`), which emits
`"Seat {index}"` (0-based). `docs/UI-INTERACTION.md` §10B states seat indices are
dev-panel vocabulary and normal public UI uses 1-based display names; River Ledger
already does this (`games/river_ledger/src/ui.rs:99-101`,
`seat_public_label` = `Seat {index + 1}`). The default must match that convention.

This ticket fixes the **Rust label source** (the single source of truth per
`docs/FOUNDATIONS.md` §2 and `docs/UI-INTERACTION.md` §3). As a direct side
effect, Briar Circuit's VIEWER (which uses the default,
`catalog.rs:245` `catalog_seat_labels_json(4)`) becomes "Seat 1".."Seat 4",
matching its play area.

## Assumption Reassessment (2026-06-21)

1. `catalog_seat_labels_json(seat_count)` at `crates/wasm-api/src/catalog.rs:65-73`
   emits `{"seat":"seat_{index}","label":"Seat {index}"}` for `0..seat_count` — 0-based, confirmed.
2. `docs/UI-INTERACTION.md` §3 (ownership: Rust owns UI metadata labels) and §10B
   (seat indices are dev-panel vocabulary; normal UI uses Rust-supplied display
   names) govern this; `docs/MULTI-SEAT-AND-SURFACE-CONTRACT.md` §2 lists seat
   labels as Rust-owned, IP-safe content. `docs/FOUNDATIONS.md` §2 lists
   public/private view projection as Rust-owned.
3. Shared boundary under audit: the catalog `seat_labels` JSON contract consumed
   by `apps/web` (`SeatFrame.tsx:14` `active_seat_labels?: SeatDisplayLabel[]`,
   and `game.seat_labels`). The change is value-only (label text), not a shape
   change to the `{seat,label}` schema.
4. FOUNDATIONS §2 principle restated: Rust owns public-view projection including
   the seat labels surfaced to viewers; flipping the default keeps authorship in
   Rust rather than relying on TS to re-derive a 1-based label.
6. Schema check: the `seat_labels` array shape `{"seat","label"}` is unchanged;
   only the `label` string value changes. Additive-compatible for consumers that
   read `label` verbatim. Consumers: `apps/web` SeatFrame/boards (migrated in
   SEAT-004/005) and the wasm-api catalog tests (updated here).
8. Adjacent contradiction exposed: defaulting games whose **play area** is still
   0-based (e.g. the 2-seat boards' inline ternaries) will become inconsistent
   with the now-1-based VIEWER until SEAT-005 migrates them. This is a required,
   tracked consequence — SEAT-006 is the cross-game consistency guard — not a
   separate bug. Race/Directional naming intent ("Player N") is preserved by
   SEAT-002.

## Architecture Check

1. Fixing the Rust default (vs. patching the TS VIEWER) keeps seat labels
   authored once in Rust, the doc-mandated owner; every defaulting game inherits
   the correct convention with no per-game TS code.
2. No backwards-compatibility shim: the 0-based default is replaced outright, not
   aliased; tests assert the new values.
3. `engine-core` is untouched (no mechanic nouns added); `wasm-api` is the
   Rust↔browser bridge, the correct home for catalog presentation metadata. No
   `game-stdlib` change.

## Verification Layers

1. Default label values are 1-based -> schema/serialization validation
   (wasm-api catalog JSON unit test asserting `"label":"Seat 1"` for `seat_0`).
2. Briar Circuit VIEWER labels become "Seat 1".."Seat 4" -> schema/serialization
   validation (wasm-api catalog test for `RegisteredGame::BriarCircuit`).
3. No 0-based default label string remains -> codebase grep-proof
   (`grep -rn '"Seat {index}"\|Seat {index}' crates/wasm-api/src` returns only the
   fixed `{index + 1}` form).

## What to Change

### 1. Flip the default generator to 1-based

In `crates/wasm-api/src/catalog.rs`, change `catalog_seat_labels_json` so each
entry's label is `format!("Seat {}", index + 1)` while the `seat` id stays
`seat_{index}` (0-based id, 1-based human label), mirroring
`games/river_ledger/src/ui.rs:99-101`.

### 2. Update affected catalog tests

Update every wasm-api catalog/snapshot assertion that currently expects the
0-based default label (the defaulting games, including Briar Circuit) to the
1-based value.

## Files to Touch

- `crates/wasm-api/src/catalog.rs` (modify)
- `crates/wasm-api/src/tests.rs` (modify)

## Out of Scope

- Per-game "Player N" overrides for Race to N / Directional Flip (SEAT-002).
- Any TypeScript change (SEAT-003/004/005).
- The cross-game consistency integration test (SEAT-006).

## Acceptance Criteria

### Tests That Must Pass

1. `cargo test -p wasm-api` — catalog tests assert 1-based default labels.
2. `cargo test --workspace` — no regression.
3. `cargo fmt --all --check && cargo clippy --workspace --all-targets -- -D warnings`

### Invariants

1. Seat labels remain authored in Rust; TS is not the source of the label text.
2. The `seat_labels` entry shape `{"seat","label"}` is unchanged (value-only edit).

## Test Plan

### New/Modified Tests

1. `crates/wasm-api/src/tests.rs` — assert `catalog_seat_labels_json` and the
   Briar Circuit catalog produce `"label":"Seat 1"`..`"Seat 4"` for `seat_0`..`seat_3`.

### Commands

1. `cargo test -p wasm-api`
2. `cargo test --workspace`

## Outcome

Completed: 2026-06-21

Changed:
- `catalog_seat_labels_json` now keeps stable `seat_{index}` IDs while emitting
  1-based human labels (`Seat 1`, `Seat 2`, ...).
- `wasm-api` catalog tests now pin the default generator and Briar Circuit's
  four-seat catalog labels.
- The public API snapshot was regenerated for the intended catalog value drift.

Deviations:
- The internal default-label helper was made `pub(crate)` so the crate-local
  unit test can assert its exact serialized output. It remains non-public API.

Verification:
- `UPDATE_API_SNAPSHOT=1 cargo test -p wasm-api --test api_surface` passed.
- `cargo test -p wasm-api` passed.
- `cargo test --workspace` passed.
- `cargo fmt --all --check` passed.
- `cargo clippy --workspace --all-targets -- -D warnings` passed.

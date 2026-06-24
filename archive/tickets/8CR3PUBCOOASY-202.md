# 8CR3PUBCOOASY-202: C-02 WASM seat boundary conformance

**Status**: COMPLETED
**Priority**: MEDIUM
**Effort**: Medium
**Engine Changes**: Yes (compatibility evidence) — `crates/wasm-api/src/seats.rs`
**Deps**: 8CR3PUBCOOASY-001

## Problem

Three R3 games (`flood_watch`, `frontier_control`, `event_frontier`) have no
game-local typed seat parser — they cross the import boundary through the shared
`wasm-api` alias adapter and already emit canonical `SeatId` output. Their C-02
verdict is `exception` (import-only legacy adapter retained, canonical output
authoritative). This ticket records fresh R3 conformance evidence for the shared
boundary: import aliases accepted, canonical Rust output unchanged, out-of-game
index rejected, no TypeScript normalization. No output flip is expected.

## Assumption Reassessment (2026-06-24)

1. `crates/wasm-api/src/seats.rs` defines `parse_seat_import` (line ~42),
   `parse_plain_seat` (~210), `parse_flood_seat`/`parse_frontier_seat`/
   `parse_event_frontier_seat` (~171–179), and the canonical emitters
   `plain_seats`/`flood_seats`/`frontier_seats`/`event_frontier_seats`
   (~248–284). All exist as the spec states.
2. Spec §3.5 verdicts: the three non-Plain games' seat parsing/output are
   `exception` (shared `wasm-api` import-only adapter; canonical output
   authoritative); §5.4 task `8C-R3-202` scopes this conformance audit. The
   Plain typed parser is a separate diff (201).
3. Cross-crate boundary under audit: the `wasm-api` import-alias adapter vs the
   canonical `SeatId` output contract — import accepts legacy aliases, output
   stays canonical `seat_<n>`; faction/domain IDs are not seat aliases.
4. FOUNDATIONS §2 motivates this: TypeScript must not normalize or repair a seat
   ID; legality/normalization authority stays in Rust/WASM.
5. Enforcement surface: the canonical acceptance, malformed-rejection, and
   allowed-legacy-alias vector set plus canonical output snapshots; this ticket
   is byte-neutral — it adds conformance tests/evidence, it does not flip any
   output. If a real output defect is found, stop and split a separate
   game-specific ADR-0009 seat-output migration (do not hide it here).

## Architecture Check

1. Recording fresh conformance evidence on the existing shared adapter (rather
   than synthesizing game-local parsers for the three games) is the
   narrowest-owner choice: the adapter already crosses the boundary correctly.
2. No backwards-compatibility alias is *added* — the existing import aliases are
   retained as the compatibility window; no new shim.
3. `engine-core` and `game-stdlib` are untouched; `wasm-api` owns import aliases
   and transport compatibility only, not legality.

## Verification Layers

1. Import-alias acceptance -> `cargo test -p wasm-api` seat tests (legacy alias
   accepted on import).
2. Canonical output stability -> canonical output snapshots byte-identical to
   001 baseline (no `seat_<n>` change).
3. Boundary safety -> out-of-game index rejects; no TS normalization path exists
   (grep-proof the TS shell does not repair seat IDs).

## What to Change

### 1. Add R3 conformance tests

In `crates/wasm-api/src/seats.rs` tests, assert canonical acceptance, malformed
rejection, out-of-game index rejection, allowed legacy import aliases, and
canonical output spelling for all four games' seat surfaces. No production
behavior or output spelling changes.

## Files to Touch

- `crates/wasm-api/src/seats.rs` (modify — tests/conformance only)

## Out of Scope

- Any canonical output flip or new seat normalization (if a real defect is
  found, it becomes a separate ADR-0009 migration).
- The Plain typed parser (201); faction/district/event/site/card IDs.
- Adding seat normalization to TypeScript.

## Acceptance Criteria

### Tests That Must Pass

1. `cargo test -p wasm-api` (seat conformance vectors).
2. `cargo run -p replay-check -- --game plain_tricks --all` and the other three
   games — byte-identical canonical seat output to baseline.

### Invariants

1. Import aliases remain import-only; canonical output is unchanged from baseline.
2. No TypeScript path normalizes or repairs a seat ID.

## Test Plan

### New/Modified Tests

1. `crates/wasm-api/src/seats.rs` (inline tests) — R3 seat conformance vectors
   for the four games' import-alias and canonical-output paths.

### Commands

1. `cargo test -p wasm-api`
2. `cargo run -p replay-check -- --game flood_watch --all`
3. `cargo test -p wasm-api` plus per-game replay-check is the correct boundary:
   the surface is the shared seat adapter and the four games' canonical output.

## Outcome

Completed: 2026-06-24

Added R3 conformance vectors in `crates/wasm-api/src/seats.rs` for Plain
Tricks, Flood Watch, Frontier Control, and Event Frontier. The tests cover
canonical input, retained legacy import aliases, malformed/out-of-game
rejection, and canonical output spelling. The change is test-only; production
seat parsing, canonical output, TypeScript behavior, replay/export bytes, and
game parsers were otherwise untouched.

Deviations: none.

Verification:

- `cargo test -p wasm-api` passed.
- `cargo run -p replay-check -- --game plain_tricks --all` passed.
- `cargo run -p replay-check -- --game flood_watch --all` passed.
- `cargo run -p replay-check -- --game frontier_control --all` passed.
- `cargo run -p replay-check -- --game event_frontier --all` passed.
- Targeted TypeScript grep
  `rg -n "normalizeSeat|parseSeat|parse_seat|repairSeat|canonicalSeat|seatId.*replace|replace\\([^)]*seat-" apps/web --glob '*.{ts,tsx,js,jsx}'`
  returned no matches.

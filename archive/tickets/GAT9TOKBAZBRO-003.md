# GAT9TOKBAZBRO-003: token_bazaar state + setup + variants

**Status**: ✅ COMPLETED
**Priority**: HIGH
**Effort**: Medium
**Engine Changes**: Yes — `games/token_bazaar/src/state.rs`, `src/setup.rs`, `src/variants.rs` (new); `src/lib.rs` (modify, mod declarations)
**Deps**: GAT9TOKBAZBRO-002

## Problem

The game needs a typed, serializable state and a deterministic setup before legal
actions and effects can be implemented. This ticket defines the `token_bazaar`
public state (per-seat inventories + scores, public supply, the three market
slots, the contract queue, per-seat turn counters, active seat) and the
deterministic `token_bazaar_standard` setup (supply 14 each, inventory 1 each,
score 0, the first three queue contracts filling slots, `seat_0` active), plus the
variant selection wiring.

## Assumption Reassessment (2026-06-08)

1. `games/token_bazaar/src/ids.rs` (GAT9TOKBAZBRO-002) provides the typed IDs
   this state references (seats, resources, bundles, contracts, slots). The
   sibling `games/high_card_duel/src/state.rs` + `src/setup.rs` + `src/variants.rs`
   establish the house pattern (verified present). `src/lib.rs` exists from -002
   and is modified here to declare `mod state; mod setup; mod variants;`.
2. The state shape and setup constants are fixed by
   `specs/gate-9-token-bazaar-browser-proof.md` → "Resources" (supply/inventory
   tables), "Market contracts" (the ten-contract standard queue), and "Turn
   structure" (8 turns/seat, `seat_0` starts). No constant is invented here.
3. Cross-artifact boundary under audit: the serializable state contract. This
   state is the substrate for effects (GAT9TOKBAZBRO-005), visibility
   (-006), replay/hash (-007), serialization tests (-009), and the JSON fixture
   (-009). Its field layout and iteration order must be stable.
4. FOUNDATIONS §5 (static data is typed content): setup reads typed constants
   from the manifest/variant (or game-local typed tables) — never from procedural
   data. The contract queue is a typed ordered list, not a behavior table.
5. Deterministic-serialization substrate: state collections (supply by resource,
   inventories by seat/resource, slot→contract mapping, the queue) must use
   stable ordering (sorted or insertion-ordered, not incidental hash-map order)
   so replay/hash in GAT9TOKBAZBRO-007/010 is reproducible. All state is public,
   so there is no hidden field to redact — but the no-leak harness (-009) still
   asserts no debug-only field exists.

## Architecture Check

1. Separating pure state/setup (this ticket) from legality (-004) and
   transitions/effects (-005) keeps each diff reviewable and mirrors the
   `high_card_duel` 003/006/007 split.
2. No backwards-compatibility aliasing/shims — new files.
3. `engine-core` untouched; resource/market/contract/supply types are game-local
   to `games/token_bazaar`. No `game-stdlib` helper introduced.

## Verification Layers

1. Setup produces the spec's exact initial state -> `cargo test -p token_bazaar`
   (setup unit test asserting supply 14/inventory 1/score 0/three filled slots/
   queue length 7 remaining/`seat_0` active).
2. State serializes with stable order -> serialization round-trip unit test
   (full coverage in GAT9TOKBAZBRO-009).
3. Variant selection resolves `token_bazaar_standard` -> unit test loading the
   variant and asserting the standard setup.
4. `engine-core` noun-freedom preserved -> `bash scripts/boundary-check.sh`.

## What to Change

### 1. `games/token_bazaar/src/state.rs`

Typed state: `supply: ResourceCounts`, `inventories: [ResourceCounts; 2]`,
`scores: [u32; 2]`, `slots: [Option<ContractId>; 3]`, `queue: VecDeque<ContractId>`
(or stable ordered equivalent), `fulfilled: [Vec<ContractId>; 2]`,
`turns_taken: [u8; 2]`, `active: SeatId`, plus the contract cost/points lookup
(typed table, not data formula). Provide stable serialization.

### 2. `games/token_bazaar/src/setup.rs`

`setup_match(variant, options)` building the deterministic standard state: supply
14 each, inventory 1 each, score 0, slots filled from the queue front, remaining
queue = 7 contracts, `turns_taken = [0,0]`, `active = seat_0`.

### 3. `games/token_bazaar/src/variants.rs`

Variant enum / selector exposing `token_bazaar_standard`; resolve to the setup above.

### 4. `games/token_bazaar/src/lib.rs` (modify)

Add `mod state; mod setup; mod variants;` and re-export the public types.

## Files to Touch

- `games/token_bazaar/src/state.rs` (new)
- `games/token_bazaar/src/setup.rs` (new)
- `games/token_bazaar/src/variants.rs` (new)
- `games/token_bazaar/src/lib.rs` (modify)

## Out of Scope

- Legal-action generation, validation, diagnostics (GAT9TOKBAZBRO-004).
- Effects, transitions, terminal/tie-breaks (GAT9TOKBAZBRO-005).
- Visibility projection (GAT9TOKBAZBRO-006), replay (GAT9TOKBAZBRO-007).
- The JSON fixture (GAT9TOKBAZBRO-009).

## Acceptance Criteria

### Tests That Must Pass

1. `cargo test -p token_bazaar` — setup test asserts the exact spec initial state.
2. `cargo test -p token_bazaar` — variant resolution test for `token_bazaar_standard`.
3. `cargo build -p token_bazaar` — crate still compiles with new modules.

### Invariants

1. Setup is fully deterministic (no RNG): identical variant → identical state,
   byte-for-byte serialized.
2. State collections iterate in a stable order independent of hash-map seeding.

## Test Plan

### New/Modified Tests

1. `games/token_bazaar/src/setup.rs` (unit) — initial supply/inventory/score/slots/queue/active.
2. `games/token_bazaar/src/variants.rs` (unit) — `token_bazaar_standard` resolves.
3. `games/token_bazaar/src/state.rs` (unit) — serialization round-trip stability.

### Commands

1. `cargo test -p token_bazaar`
2. `cargo build -p token_bazaar && bash scripts/boundary-check.sh`
3. Per-crate test is the correct boundary; no other crate depends on this state
   until the tool/WASM registration tickets.

## Outcome

Completed: 2026-06-08

What changed:

- Added `games/token_bazaar/src/state.rs` with public resource counts, contract
  specs, state, terminal outcome, and stable snapshot serialization.
- Added `games/token_bazaar/src/setup.rs` with deterministic
  `token_bazaar_standard` setup: 14-resource supply, 1-resource inventories,
  zero scores, first three contracts in slots, seven queued contracts,
  `seat_0` active, zero turns, no terminal outcome, and freshness 0.
- Extended `games/token_bazaar/src/variants.rs` with standard variant
  resolution.
- Updated `src/lib.rs` exports for state/setup.

Deviations from original plan:

- `src/variants.rs` already existed from GAT9TOKBAZBRO-002 to satisfy static
  manifest parse tests; this ticket extended it instead of creating it.

Verification results:

- `cargo test -p token_bazaar` passed with 10 tests.
- `cargo build -p token_bazaar` passed.
- `bash scripts/boundary-check.sh` passed.

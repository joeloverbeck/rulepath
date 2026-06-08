# GAT9TOKBAZBRO-004: Legal action tree + validation + diagnostics

**Status**: ✅ COMPLETED
**Priority**: HIGH
**Effort**: Medium
**Engine Changes**: Yes — `games/token_bazaar/src/actions.rs` (new), `src/rules.rs` (new, legality/validation half), `src/lib.rs` (modify)
**Deps**: GAT9TOKBAZBRO-003

## Problem

Rust must own all legality. This ticket builds the legal action tree for a
non-terminal active turn (collect bundles, exchanges, fulfillable slots, and the
forced pass only when nothing else is legal), the action-tree payload metadata
the browser will present (family, cost, gain, points, slot/contract ids), and the
command validation + stable diagnostics for illegal commands (insufficient cost,
empty slot, exhausted supply, non-active seat, terminal, stale token). No
affordability or legality logic may live anywhere but Rust.

## Assumption Reassessment (2026-06-08)

1. `games/token_bazaar/src/state.rs` + `setup.rs` (GAT9TOKBAZBRO-003) provide the
   state this legality reads (supply, inventories, slots, queue, active seat,
   turn counts). The sibling `games/high_card_duel/src/actions.rs` + `src/rules.rs`
   establish the action-tree + validation house pattern (verified present).
   `src/lib.rs` from -003 is modified to add `mod actions; mod rules;`.
2. The legality rules are fixed by `specs/gate-9-token-bazaar-browser-proof.md`
   → "Legal actions" (collect requires the full bundle in supply; exchange requires
   `pay != take`, ≥2 of pay in inventory, ≥1 of take in supply; fulfill requires an
   occupied slot and all required resources; forced pass only when nothing else is
   legal) and "Action-tree previews and metadata" (the recommended metadata keys).
3. Cross-artifact boundary under audit: the action-tree + command-envelope +
   diagnostic contract from `docs/ARCHITECTURE.md` / `docs/ENGINE-GAME-DATA-BOUNDARY.md`.
   The action-tree node shape, command envelope, and diagnostic codes this ticket
   emits are consumed by replay (-007), tests (-009/-010), tools (-012), and WASM
   (-013); they must conform to the engine contract, not invent a parallel shape.
4. FOUNDATIONS §2 (behavior authority) is the motivating invariant: legality,
   validation, and the legal action set are computed only in Rust. TypeScript
   later presents this tree; it must never recompute affordability or legality
   (enforced downstream in -014/-015). Validation is fail-closed and blocking:
   an invalid command yields a diagnostic and mutates no state.
5. Determinism + no-leak substrate: the action tree must be stably ordered
   (stable bundle/slot order) so its hash is reproducible across replay and WASM
   (-007/-013). All state is public, so action metadata carries no hidden field;
   per the spec, action metadata must not expose debug-only valuation data — the
   no-leak test (-009) asserts this.
6. Action-tree / diagnostic schema: this ticket emits action-tree nodes (with the
   metadata keys) and a closed set of diagnostic codes. Consumers: WASM
   action-tree export + diagnostic hashing (-013), golden traces for each invalid
   case (-010). The set is new (additive for this game), enumerated here so the
   trace and tool tickets cover every code.

## Architecture Check

1. Splitting legality/validation (this ticket) from apply/effects (-005) gives a
   clean read-only-vs-mutating boundary and a reviewable diff; it matches the
   `high_card_duel` 006/007 split.
2. No backwards-compatibility aliasing/shims — new files.
3. `engine-core` stays noun-free: the action paths use game-local segment strings
   (`collect`/`exchange`/`fulfill`/`pass` + ids); the kernel only knows the opaque
   action-tree contract, not their meaning. No `game-stdlib` helper added.

## Verification Layers

1. Legal set correctness (only legal actions appear) -> `cargo test -p token_bazaar`
   (rules tests per family + the forced-pass-only-when-stuck case).
2. Action-tree shape conforms to the engine contract + stable order/hash ->
   schema/serialization validation in tests; deterministic action-tree hash check
   under replay (-010).
3. Invalid commands reject with stable diagnostics and no mutation -> per-code
   rejection unit tests (insufficient/empty-slot/exhausted/non-active/terminal/stale).
4. No-leak: action metadata carries no debug/valuation field -> no-leak assertion
   (full suite in -009).

## What to Change

### 1. `games/token_bazaar/src/actions.rs`

Action representation + stable paths (`collect/<bundle>`, `exchange/<pay>/<take>`,
`fulfill/<slot>`, `pass`) and the legal-action-tree builder emitting, for each
legal node, the metadata: `family`, `cost`, `gain`, `slot_id`/`contract_id`,
`points`, plus accessibility labels. Stable ordering of bundles/slots.

### 2. `games/token_bazaar/src/rules.rs` (legality/validation half)

`legal_actions(state)` and `validate_command(state, command) -> Result<Validated,
Diagnostic>`: collect/exchange/fulfill/pass legality predicates and the diagnostic
codes for each illegal case. Validation never mutates state.

### 3. `games/token_bazaar/src/lib.rs` (modify)

Add `mod actions; mod rules;`; re-export the legal-action and validation surface.

## Files to Touch

- `games/token_bazaar/src/actions.rs` (new)
- `games/token_bazaar/src/rules.rs` (new)
- `games/token_bazaar/src/lib.rs` (modify)

## Out of Scope

- Applying actions, emitting effects, refill, terminal/tie-breaks (GAT9TOKBAZBRO-005).
- Visibility projection (GAT9TOKBAZBRO-006); replay (GAT9TOKBAZBRO-007).
- WASM/TS exposure (GAT9TOKBAZBRO-013/014).

## Acceptance Criteria

### Tests That Must Pass

1. `cargo test -p token_bazaar` — legal-action tests for collect/exchange/fulfill
   and the forced-pass-only-when-stuck case.
2. `cargo test -p token_bazaar` — each invalid command (insufficient cost, empty
   slot, exhausted supply, non-active seat, terminal, stale token) rejects with
   its stable diagnostic and leaves state unchanged.
3. `cargo build -p token_bazaar`.

### Invariants

1. The legal action set is computed only in Rust; affordability/legality is never
   derivable from data alone (§2).
2. Validation is fail-closed and blocking: an invalid command mutates no state and
   returns a stable diagnostic.
3. The action tree is stably ordered and its serialization is deterministic.

## Test Plan

### New/Modified Tests

1. `games/token_bazaar/src/rules.rs` (unit) — legality predicates + per-code rejection.
2. `games/token_bazaar/src/actions.rs` (unit) — action-tree node metadata + stable order.

### Commands

1. `cargo test -p token_bazaar`
2. `cargo build -p token_bazaar && bash scripts/boundary-check.sh`
3. Per-crate test is the correct boundary; cross-tool/WASM action-tree hashing is
   verified in GAT9TOKBAZBRO-010/013.

## Outcome

Completed: 2026-06-08

What changed:

- Added `games/token_bazaar/src/actions.rs` with stable flat action segments,
  actor-seat lookup, parsing, action-tree construction, and Rust-owned metadata
  for collect, exchange, fulfill, and forced pass.
- Added `games/token_bazaar/src/rules.rs` with legal action enumeration,
  validation, stable diagnostics, and fail-closed rejection for stale,
  non-active, exhausted-supply, insufficient-cost, empty-slot, terminal, and
  pass-not-forced cases.
- Updated `src/lib.rs` exports for the action and validation surfaces.

Deviations from original plan:

- None.

Verification results:

- `cargo test -p token_bazaar` passed with 18 tests.
- `cargo build -p token_bazaar` passed.
- `bash scripts/boundary-check.sh` passed.

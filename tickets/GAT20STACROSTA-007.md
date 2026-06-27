# GAT20STACROSTA-007: Single-step legal moves, validation, and step effects

**Status**: PENDING
**Priority**: MEDIUM
**Effort**: Medium
**Engine Changes**: Yes — `games/starbridge_crossing/src/{rules.rs,actions.rs,effects.rs}`, `tests/rules.rs`
**Deps**: GAT20STACROSTA-006

## Problem

A turn is exactly one move; the simplest is a single step from a peg's space to an adjacent empty space. This ticket lands Rust-owned single-step legal-move generation, validation (occupied/off-board/non-adjacent/stale/wrong-seat diagnostics), and the step semantic effect — the foundation the jump-chain enumeration (008) extends.

## Assumption Reassessment (2026-06-27)

1. Legal-action generation uses the generic `engine-core` action-tree contract (`ActionTree`/`ActionNode`/`ActionPath`, string segments) — confirmed `crates/engine-core/src/lib.rs`; the step path encoding `move/<peg-id>/step/<dest-space>` is game-local string framing.
2. Step legality reads adjacency from `src/topology.rs` (005) and occupancy/active-seat from `src/state.rs` (006); these modules exist by this ticket's `Deps`.
3. Cross-artifact boundary: `rules.rs` (legality), `actions.rs` (path encode/parse), `effects.rs` (semantic effect) are created here and extended by 008 (jump) and 009 (finish); the action-tree ordering contract (pegs in seat-local order, step leaves before jump roots) is pinned from spec Appendix A.
4. §2 (behavior authority) motivates this ticket: Rust enumerates legal steps and validates every accepted path against current state + freshness token; TypeScript never derives adjacency or legal destinations.

## Architecture Check

1. Building the step path first establishes the action-tree/effect plumbing that the jump chain reuses, keeping a single validation entry point.
2. No backwards-compatibility shims.
3. `engine-core` action-tree contract is consumed generically (no mechanic noun added to the kernel); no `game-stdlib` change.

## Verification Layers

1. Step legality (§2) -> rule test: a peg steps only to a listed-neighbor empty space; occupied/non-adjacent/off-board destinations are rejected.
2. Diagnostic safety -> rule test: wrong-seat / stale-token / post-finish step attempts produce stable diagnostics with no state mutation.
3. Step effect -> golden trace (authored in 011) + here an effect-shape unit test.
4. Action-tree ordering -> unit test: deterministic seat-local peg ordering and canonical six-direction order.

## What to Change

### 1. Author `src/rules.rs` (step legality)

Enumerate, for the active seat, each peg's adjacent empty spaces as `step` leaves; validate an accepted step path against current state and freshness token.

### 2. Author `src/actions.rs` (step encoding)

`move/<peg-id>/step/<dest-space>` encode/parse and stable ordering.

### 3. Author `src/effects.rs` (step effect)

Semantic move effect (peg id, from, to) for renderer animation.

## Files to Touch

- `games/starbridge_crossing/src/rules.rs` (new)
- `games/starbridge_crossing/src/actions.rs` (new)
- `games/starbridge_crossing/src/effects.rs` (new)
- `games/starbridge_crossing/tests/rules.rs` (modify; created by 006 — add step legality cases)
- `games/starbridge_crossing/src/lib.rs` (modify; created by 004 — add `pub mod {rules,actions,effects};`)

## Out of Scope

- Hop / multi-hop jump chains — GAT20STACROSTA-008.
- Finish, blocked pass, turn-limit — GAT20STACROSTA-009.
- Mixed step+jump rejection is asserted in 008 (where both paths coexist).

## Acceptance Criteria

### Tests That Must Pass

1. `cargo test -p starbridge_crossing --test rules`
2. `cargo test -p starbridge_crossing`
3. `bash scripts/boundary-check.sh`

### Invariants

1. A legal step never lands on an occupied or non-adjacent space (§2).
2. Every accepted step is revalidated by Rust; invalid steps mutate no state.

## Test Plan

### New/Modified Tests

1. `games/starbridge_crossing/tests/rules.rs` — step legality, occupied/non-adjacent/off-board/wrong-seat/stale diagnostics.
2. `games/starbridge_crossing/src/effects.rs` — inline step-effect shape test.

### Commands

1. `cargo test -p starbridge_crossing --test rules`
2. `cargo test -p starbridge_crossing && bash scripts/boundary-check.sh`
3. `--test rules` isolates step legality; full crate run confirms topology/state integration.

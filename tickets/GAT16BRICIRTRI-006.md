# GAT16BRICIRTRI-006: Private pass phase (select, confirm, atomic exchange, hold)

**Status**: PENDING
**Priority**: HIGH
**Effort**: Medium
**Engine Changes**: Yes — `games/briar_circuit/src/actions.rs` (pass family), `state.rs` (pass substate), pass effects/views
**Deps**: 005

## Problem

Briar Circuit's three-card pass phase is its first hidden-information interaction: each seat privately selects exactly three distinct owned cards, commitments stay private until all four seats confirm, the exchange is atomic, and hold hands skip selection entirely. Public viewers see only direction and pending/committed counts — never card identities or which seat passed what. This ticket implements the Rust-owned pass actions, atomic exchange, and identity-free public commitment status.

## Assumption Reassessment (2026-06-20)

1. `games/briar_circuit/src/setup.rs` exposes the per-hand pass direction after GAT16BRICIRTRI-005; this ticket fills the `actions.rs` pass family and the `Passing` substate stubbed in `state.rs`. The action-path/command-envelope/diagnostic contracts come from `engine-core` (`crates/engine-core/src/lib.rs`).
2. `specs/gate-16-briar-circuit-trick-taking.md` §3.1 (Pass cycle, Pass privacy/resolution rows), §4.2 (Pass actions row), Appendix A `BC-PASS-002/003/004`, Appendix B.2 (`pass/select|unselect|confirm` action families), and Appendix B.3 (`BC_PASS_*` diagnostics) fix the behavior.
3. Cross-artifact boundary under audit: the pass-selection substate is private (owner-only projection); the public commitment effect/view carries counts/status only. This is the no-leak firewall's first exercise — staged selections and provenance must never reach another seat or the observer.
4. FOUNDATIONS §11 no-leak firewall is the principle under audit: a selected card's identity stays private to its owner until later legally played, and *who passed it* is never made public. The pass-in-flight state must not leak through views, previews, diagnostics, effects, or action trees. `BC_PASS_REQUIRES_THREE`/`BC_PASS_DUPLICATE_CARD`/`BC_PASS_ALREADY_COMMITTED` are fail-closed validation, not warnings.

## Architecture Check

1. A stateful private selection set (`pass/select`/`unselect` then `pass/confirm` at exactly three) avoids a flat 286-combination action surface while keeping Rust the legality authority; the public shell receives only pending/committed status.
2. No backwards-compatibility aliasing/shims — fills the `Passing` stubs.
3. `engine-core` untouched (§3); no `game-stdlib` pass helper (§4) — pass routing stays game-local.

## Verification Layers

1. Exactly three distinct owned cards required before confirm -> `tests/rules.rs` positive/negative (`BC-PASS-002`, `BC_PASS_REQUIRES_THREE`/`BC_PASS_DUPLICATE_CARD`/`BC_CARD_NOT_OWNED`).
2. No incoming cards until all four confirm; exchange atomic -> `tests/rules.rs` + `tests/property.rs` (`BC-PASS-003`).
3. Hold hand skips selection/exchange -> `tests/rules.rs` (`BC-PASS-004`).
4. Pass-in-flight selections/provenance do not leak -> `tests/visibility.rs` pass-in-flight no-leak (pairwise) — staged commitments absent from non-owner views/effects/trees.

## What to Change

### 1. `games/briar_circuit/src/actions.rs` (pass family)

Rust-owned `pass/select/<card-id>`, `pass/unselect/<card-id>`, and `pass/confirm`; legal-set generation per state; confirm exists only at exactly three distinct owned selections; stable `BC_PASS_*` diagnostics on the apply path.

### 2. `games/briar_circuit/src/state.rs` (`Passing` substate)

Private per-seat selection set, committed-seat set, pending-seat tracking, and the atomic exchange applied once all four seats confirm; hold-hand transition straight to play.

### 3. Pass effects and views (in `effects.rs`/`visibility.rs` stubs as needed)

Public commitment effect carries count/status only; private effects deliver own selection/commit receipt and own sent/received cards after exchange (full filtering proven in GAT16BRICIRTRI-009).

## Files to Touch

- `games/briar_circuit/src/actions.rs` (modify; created by 004)
- `games/briar_circuit/src/state.rs` (modify; created by 004)
- `games/briar_circuit/src/effects.rs` (modify; created by 004)
- `games/briar_circuit/tests/rules.rs` (modify; created by 004)
- `games/briar_circuit/tests/property.rs` (modify; created by 004)
- `games/briar_circuit/tests/visibility.rs` (new)

## Out of Scope

- Full four-seat pairwise visibility matrix and effect-filter proof (GAT16BRICIRTRI-009) — this ticket adds only the pass-in-flight no-leak case.
- Trick play legality (GAT16BRICIRTRI-007).
- The play action family in `actions.rs` — added by GAT16BRICIRTRI-007 (shared file; coordinate the mechanical merge).

## Acceptance Criteria

### Tests That Must Pass

1. `cargo test -p briar_circuit --test rules` — three-distinct-cards, atomic exchange, hold-hand, and `BC_PASS_*` diagnostics.
2. `cargo test -p briar_circuit --test visibility` — pass-in-flight selections/provenance absent from non-owner surfaces.
3. `cargo test -p briar_circuit --test property` — exchange conserves cards (13 per seat after exchange; no duplicate/lost card).

### Invariants

1. Pass selections and provenance are owner-private until/unless Rust makes them public (§11 no-leak).
2. Pass validation is deterministic, fail-closed, and blocking (§11); confirm is impossible at ≠3 distinct owned cards.

## Test Plan

### New/Modified Tests

1. `games/briar_circuit/tests/visibility.rs` — pass-in-flight pairwise no-leak (canary per seat).
2. `games/briar_circuit/tests/rules.rs` — pass select/confirm/exchange/hold + malformed-choice diagnostics.
3. `games/briar_circuit/tests/property.rs` — card conservation across the exchange.

### Commands

1. `cargo test -p briar_circuit --test rules --test visibility --test property`
2. `cargo test -p briar_circuit`
3. A per-test scope is correct because the deliverable is the pass phase; the full pairwise matrix and WASM no-leak harness belong to 009/013.

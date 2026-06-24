# 8CR4NSEAPRITRI-019: River Ledger C-07 stack-lifecycle pairwise no-leak matrix

**Status**: PENDING
**Priority**: HIGH
**Effort**: Medium
**Engine Changes**: Yes (no-leak test geometry) — `games/river_ledger/tests/visibility.rs`; preserves ticket-021 pilot base
**Deps**: 8CR4NSEAPRITRI-001

## Problem

River's ticket-021 base no-leak matrix is pilot-discharged, but the selected stack/all-in lifecycle states (short blind/call/open-bet/raise all-in, cumulative reopen) are not yet wrapped in the shared pairwise no-leak geometry (MSC-8C, C-07 residual). Wrap those selected existing lifecycle states in `game-test-support`'s pairwise geometry for counts 3–6 over observer plus every seat, preserving ticket-021 pilot credit and all game-owned reveal/accounting rules (spec §3.8 River residual, §5.8).

## Assumption Reassessment (2026-06-24)

1. `games/river_ledger/tests/visibility.rs::{visibility_stack_pot_pairwise_matrix_hides_private_data_across_lifecycle_states, no_leak_snapshot, no_leak_expectation}` and the setup helpers exist; the shared harness `crates/game-test-support/src/no_leak.rs::assert_pairwise_no_leak` exists. Confirmed during `/reassess-spec`.
2. Spec §3.8 marks the ticket-021 base rows `already-discharged-by-8C-pilot` (`archive/tickets/UNI8CMECSCA-021.md`); only the residual stack/all-in lifecycle rows are `migrate`. This ticket must not rebuild the base rows.
3. Cross-artifact: the shared harness owns deterministic enumeration + structured failure reporting only; River owns hidden-data construction, phase setup, reveal timing, and accounting. The pilot base matrix baseline comes from `-001`.
4. §11 no-leak firewall motivates this ticket: a hole-card canary during the short-call/full-call/open-bet/raise all-in lifecycle must be absent for observer and non-owner seats and present only in the owning seat's authorized private view/effect/diagnostic.
5. Enforcement surface = pairwise source-seat × viewer products over the lifecycle states; canaries are generated in memory and never committed to any trace/fixture/export/snapshot/log/test ID.

## Architecture Check

1. Wrapping selected lifecycle states in the shared geometry is cleaner than ad-hoc per-state assertions — one deterministic enumerator covers source × observer × every seat, while River keeps reveal/accounting policy.
2. No backwards-compatibility shim is introduced; existing focused River visibility tests remain (the generic matrix does not subsume them). No game-specific assertion is deleted.
3. `engine-core` stays noun-free (§3); `game-test-support` is dev-only and decides nothing about secrecy (§4 mechanical-scaffolding lane).

## Verification Layers

1. Hole-card canary hidden from observer/non-owner across the lifecycle states (counts 3–6) -> no-leak visibility test via `assert_pairwise_no_leak`.
2. Pot layers / contribution totals / all-in status remain public accounting where current projection permits -> no-leak test asserting absence-of-card, not secrecy of public accounting.
3. Ticket-021 base rows preserved -> codebase grep-proof (existing base assertions still present and passing).

## What to Change

### 1. Wrap selected stack/all-in lifecycle states in shared geometry

In `games/river_ledger/tests/visibility.rs`, wrap the selected short-blind/short-call/open-bet/raise/cumulative-reopen states in `assert_pairwise_no_leak` over source seat × observer/all seats for counts 3–6, asserting hole-card absence for non-owners and presence only in the owner's authorized surfaces. Keep the existing ticket-021 base assertions.

## Files to Touch

- `games/river_ledger/tests/visibility.rs` (modify)

## Out of Scope

- Rebuilding any ticket-021 base no-leak row (pilot credit).
- The runout/multipot export matrix (`-020`).
- Any reveal, allocation, or accounting policy change; treating public pot accounting as secret.

## Acceptance Criteria

### Tests That Must Pass

1. `cargo test -p river_ledger` is green, including the new lifecycle pairwise no-leak matrix and the retained ticket-021 base assertions.
2. `cargo run -p replay-check -- --game river_ledger --all` passes (no production behavior changed).
3. `bash scripts/boundary-check.sh` passes.

### Invariants

1. No hole card reaches observer or a non-owning seat across the lifecycle states; the owner sees only its authorized surfaces.
2. No canary token appears in any committed trace, fixture, export, snapshot, log, or test identifier.

## Test Plan

### New/Modified Tests

1. `games/river_ledger/tests/visibility.rs` — add a lifecycle pairwise no-leak matrix over short/call/open-bet/raise/cumulative-reopen for counts 3–6.

### Commands

1. `cargo test -p river_ledger`
2. `cargo run -p replay-check -- --game river_ledger --all`
3. The per-game visibility test is the correct boundary: no-leak is a game-local projection property exercised through the shared harness.

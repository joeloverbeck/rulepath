# 8CR4NSEAPRITRI-023: Vow Tide C-07 hand/stock pairwise no-leak matrix (3–7)

**Status**: COMPLETED
**Priority**: HIGH
**Effort**: Medium
**Engine Changes**: Yes (no-leak test geometry) — `games/vow_tide/tests/visibility.rs`; own-hand/stock rules unchanged
**Deps**: 8CR4NSEAPRITRI-001, 8CR4NSEAPRITRI-012

## Problem

Vow's private hands and hidden stock are tested by an existing exhaustive seat-pair check, but not yet routed through the shared pairwise geometry for every count 3–7 × source seat × observer/every declared seat over view/action/diagnostic/effect (MSC-8C, C-07). Convert/extend to the shared geometry while retaining the existing exhaustive assertions, pinning own-hand-only and stock-never-visible (spec §3.8 Vow, §5.8).

## Assumption Reassessment (2026-06-24)

1. `games/vow_tide/tests/visibility.rs::{exhaustive_seat_pair_no_leak_for_three_to_seven_players, install_canary_hands_and_stock}` exist; the shared harness `assert_pairwise_no_leak` exists. Confirmed during `/reassess-spec`.
2. Spec §3.8 classifies the hand/stock matrix as `migrate`; this ticket `Deps` `-012` so the centralized 3–7 range enumerates the same counts the matrix runs.
3. Cross-artifact: Vow's hand/stock projection is game-owned; the shared harness owns enumeration/reporting only. The existing exhaustive assertions stay (rollback removes only the shared adapter). Baseline projections come from `-001`.
4. §11 no-leak firewall motivates this ticket: a private hand card is present only in its own projected hand or authorized bot input and absent elsewhere; a hidden stock card is absent on every viewer-scoped surface for all viewers.
5. Enforcement surface = count 3–7 × source seat × observer/every declared seat over view/action-tree/diagnostic/effect; the `install_canary_hands_and_stock` canaries are in-memory only, never committed.

## Architecture Check

1. Routing the exhaustive check through the shared geometry gives uniform structured failure context (game/count/source/viewer/surface/canary) while preserving the existing assertions.
2. No backwards-compatibility shim is introduced; no existing exhaustive assertion is deleted (the shared adapter is additive).
3. `engine-core` stays noun-free (§3); `game-test-support` is dev-only and decides no secrecy (§4 mechanical-scaffolding lane).

## Verification Layers

1. Private hand card present only for owner; absent for observer/non-owner across 3–7 -> no-leak visibility test via `assert_pairwise_no_leak`.
2. Hidden stock card absent on every viewer-scoped surface -> no-leak test over view/action/diagnostic/effect.
3. Existing exhaustive assertions retained -> codebase grep-proof (`exhaustive_seat_pair_no_leak_for_three_to_seven_players` still present and passing).

## What to Change

### 1. Convert/extend to shared pairwise geometry

In `games/vow_tide/tests/visibility.rs`, wrap the hand/stock no-leak coverage in `assert_pairwise_no_leak` for every count 3–7 × source seat × observer/every declared seat over view/action/diagnostic/effect, reusing `install_canary_hands_and_stock`. Keep the existing exhaustive assertions.

## Files to Touch

- `games/vow_tide/tests/visibility.rs` (modify)

## Out of Scope

- The bid/trick/export/bot matrix (`-024`).
- Any hand-schedule, deal, trump, or projection policy change (game-local).
- Writing any canary into a committed artifact.

## Acceptance Criteria

### Tests That Must Pass

1. `cargo test -p vow_tide` is green, including the shared-geometry hand/stock matrix for 3–7 and the retained exhaustive assertions.
2. `cargo run -p replay-check -- --game vow_tide --all` passes (no production behavior changed).
3. `bash scripts/boundary-check.sh` passes.

### Invariants

1. No private hand card reaches a non-owner; no hidden stock card reaches any viewer; owner sees only its own hand.
2. No canary token appears in any committed trace, fixture, export, snapshot, log, or test identifier.

## Test Plan

### New/Modified Tests

1. `games/vow_tide/tests/visibility.rs` — shared-geometry hand/stock no-leak matrix across counts 3–7 × source seat × observer/every seat.

### Commands

1. `cargo test -p vow_tide`
2. `cargo run -p replay-check -- --game vow_tide --all`
3. The per-game visibility test is the correct boundary: hand/stock no-leak is a game-local projection property.

## Outcome

Completed: 2026-06-24

What changed:

1. Added a shared pairwise no-leak matrix for Vow hand and stock canaries across seat counts 3 through 7, each source seat, observer, and every declared seat viewer.
2. Covered view, action tree, diagnostic, and effect surfaces while retaining the existing exhaustive seat-pair no-leak assertions.
3. Kept hand, stock, deal, trump, and projection policy unchanged; the change is test-only.

Deviations: None.

Verification:

1. `cargo fmt --all --check` - passed.
2. `cargo test -p vow_tide` - passed.
3. `cargo run -p replay-check -- --game vow_tide --all` - passed.
4. `bash scripts/boundary-check.sh` - passed.

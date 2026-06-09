# GAT101PLATRI-010: Property tests

**Status**: PENDING
**Priority**: HIGH
**Effort**: Medium
**Engine Changes**: Yes — `games/plain_tricks/tests/property.rs`. No production code.
**Deps**: GAT101PLATRI-009

## Problem

`docs/OFFICIAL-GAME-CONTRACT.md` and FOUNDATIONS §6/§11 require property tests for every official game. This ticket adds the `plain_tricks` property suite asserting determinism, bounded termination, the follow-suit legality invariant, point conservation, and no-leak in projections across randomized seeds and command streams.

## Assumption Reassessment (2026-06-09)

1. `games/plain_tricks/src/{setup,rules,actions,visibility}.rs` (GAT101PLATRI-005/006/007/009) are implemented; `tests/property.rs` does not yet exist. Mirror `games/poker_lite/tests/property.rs` style.
2. Spec §7 "Property tests" fixes the invariants: deterministic replay from seed + command stream; exactly 24 plays to terminal; legal tree never offers off-suit while holding the led suit; full hand offered when leading or void; trick winner is one of the two played seats; totals always equal 12 at terminal; no hidden id in public-facing projections.
3. Shared boundary under audit: none new — this ticket exercises the existing crate surface end-to-end via randomized inputs. (Single-crate test ticket.)
4. FOUNDATIONS §11 universal acceptance invariants are restated here as executable properties before trusting narrative claims — determinism, bounded termination, no-leak, legal-only.
5. Enforcement surface: deterministic replay/hash and the no-leak firewall (§11) are asserted as properties (byte-identical replay from seed; no hidden id in projections) across the randomized space, complementing the example-based tests in GAT101PLATRI-007/009.

## Architecture Check

1. Property tests over randomized seeds/streams (vs. only example tests) catch invariant violations example tests miss, especially the follow-suit and point-conservation invariants.
2. No backwards-compatibility aliasing/shims; tests only.
3. `engine-core` untouched; no `game-stdlib` change.

## Verification Layers

1. Deterministic replay from seed + command stream -> deterministic replay-hash property.
2. Bounded termination (exactly 24 plays) + point conservation (totals = 12) -> termination/accounting property.
3. Follow-suit legality (no off-suit offered while holding led suit; full hand when leading/void) -> legal-tree property.
4. No hidden id in public-facing projections -> no-leak property.

## What to Change

### 1. `games/plain_tricks/tests/property.rs`

Implement properties for: deterministic replay; exactly-24-plays termination; follow-suit legal-tree invariant; full-hand-when-leading/void; trick-winner-is-a-played-seat; totals-equal-12; no-hidden-id-in-projection. Drive randomized seeds and legal command streams through the real crate API.

## Files to Touch

- `games/plain_tricks/tests/property.rs` (new)

## Out of Scope

- Golden traces and replay export/import (GAT101PLATRI-011).
- Rule example tests (GAT101PLATRI-006/007) and the no-leak example suite (GAT101PLATRI-009) — properties complement, not replace, them.

## Acceptance Criteria

### Tests That Must Pass

1. `cargo test -p plain_tricks --test property` passes all properties across the randomized space.
2. The exactly-24-plays and totals-equal-12 properties hold for every generated match.
3. The no-hidden-id property holds for every generated projection.

### Invariants

1. Identical seed + command stream yields identical state/hash (FOUNDATIONS §2/§11).
2. The legal tree never offers an off-suit card while the actor holds the led suit (FOUNDATIONS §2; spec property).

## Test Plan

### New/Modified Tests

1. `games/plain_tricks/tests/property.rs` — the full property suite named above.

### Commands

1. `cargo test -p plain_tricks --test property`
2. `cargo test -p plain_tricks`
3. The per-crate property test is the correct boundary; it needs no external tool.

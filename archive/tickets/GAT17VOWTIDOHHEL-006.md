# GAT17VOWTIDOHHEL-006: Deterministic deal, per-hand RNG, turn-up trump, hidden stock

**Status**: COMPLETED
**Priority**: HIGH
**Effort**: Medium
**Engine Changes**: Yes — modifies `games/vow_tide/src/{setup,state}.rs`; new `games/vow_tide/src/cards.rs` deal helpers (or `setup.rs`); new setup golden traces under `games/vow_tide/tests/golden_traces/`
**Deps**: 005

## Problem

Each hand must be dealt deterministically: a per-hand RNG derived from match seed + hand index, single-card clockwise deal from left of dealer, a public turn-up trump indicator, and a face-down identity-private hidden stock — with full card conservation. This is the determinism substrate for replay, no-leak, and bots.

## Assumption Reassessment (2026-06-21)

1. `games/vow_tide/src/setup.rs`/`state.rs` (ticket 005) own the seat vector, schedule, dealer, and match seed; this ticket adds the deal/trump/stock fields and the RNG derivation. `engine-core` supplies the deterministic RNG contract (sibling games derive per-hand streams from match seed; see `games/briar_circuit/src/setup.rs` precedent).
2. Spec §3.1 + `VT-DEAL-001`/`VT-TRUMP-001` fix: deal one card at a time clockwise from left of dealer; the next undealt card is a public non-playable trump indicator; remaining undealt cards are hidden stock; the hand-size cap guarantees an indicator exists every hand (no no-trump fallback).
3. Cross-artifact boundary: the hidden-stock order and per-hand seed are private/internal facts — they must never enter a view/effect (enforced in 010) and must be reconstructible from seed+commands for replay (011). The seed-derivation rule is the determinism contract under audit.
4. FOUNDATIONS §2/§11 (deterministic RNG, replay/hash) is the principle under audit: no wall-clock/`std::time` enters canonical forms; identical seed+hand-index produce identical deals.
5. Replay/hash + no-leak enforcement surface: the hidden stock is a §11 no-leak datum; this ticket keeps it in internal state only and introduces no projection. Card conservation (dealt + trump + stock = 52, no duplicates) is the determinism guard.

## Architecture Check

1. Centralizing per-hand RNG derivation in one documented versioned rule keeps every hand reproducible and isolates the determinism boundary, cleaner than ad-hoc shuffling per call site.
2. No shims; extends the 005 state.
3. `engine-core` untouched (uses its RNG contract); no `game-stdlib` change.

## Verification Layers

1. Same seed+hand → identical deal; different seed → different deal → property tests.
2. Card conservation (52, no dup, dealt+trump+stock) → property test.
3. Turn-up trump is public + non-playable; stock stays hidden → setup golden trace + assertion (no-leak proven fully in 010).
4. Deterministic deal across N=3..7 → per-seat-count fixtures/traces.

## What to Change

### 1. Per-hand RNG + deal

Add the documented seed-derivation (match seed + hand index, versioned) and the single-card clockwise deal beginning left of dealer until each seat holds the scheduled hand size.

### 2. Trump + hidden stock

Turn up the next undealt card as the public trump indicator (suit = trump, not in any hand, unplayable); keep remaining undealt cards as face-down identity-private stock. Add conservation counters.

### 3. Setup golden traces

Author `setup-{3,4,5,6,7}p-schedule-and-deal.trace.json` and `deterministic-turn-up-trump-and-hidden-tail.trace.json` capturing deal/trump determinism.

## Files to Touch

- `games/vow_tide/src/setup.rs` (modify)
- `games/vow_tide/src/state.rs` (modify)
- `games/vow_tide/src/cards.rs` (modify)
- `games/vow_tide/tests/golden_traces/setup-3p-schedule-and-deal.trace.json` (new)
- `games/vow_tide/tests/golden_traces/setup-7p-schedule-and-deal.trace.json` (new)
- `games/vow_tide/tests/golden_traces/deterministic-turn-up-trump-and-hidden-tail.trace.json` (new)

## Out of Scope

- Bidding (007), trick play (008), scoring (009).
- View projection / no-leak harness (010) and replay export (011) — this ticket keeps stock internal only.

## Acceptance Criteria

### Tests That Must Pass

1. `cargo test -p vow_tide` — deal/RNG/conservation property tests pass.
2. `cargo run -p replay-check -- --game vow_tide` (or native trace assertion) — setup traces deterministic once the tool arm lands (016).

### Invariants

1. Identical (seed, hand index) reproduce the identical deal, trump, and stock order; no nondeterministic input enters canonical forms.
2. Cards are conserved: dealt + trump indicator + hidden stock = 52, no duplicates.

## Test Plan

### New/Modified Tests

1. `games/vow_tide/tests/property.rs` — same-seed/different-seed determinism + conservation across N=3..7.
2. `games/vow_tide/tests/golden_traces/setup-*.trace.json` — deal/trump determinism fixtures.

### Commands

1. `cargo test -p vow_tide`
2. `cargo test -p vow_tide --test property`
3. Narrower command rationale: native property/trace tests are the determinism boundary; full `replay-check --all` runs once the tool arm (016) registers vow_tide.

## Outcome

Completed on 2026-06-21.

- Added documented v1 per-hand seed derivation, deterministic shuffling, and single-card clockwise deal order from left of dealer.
- Extended Vow Tide state with dealt private hands, public trump indicator, hidden stock, and deal order while keeping stock/internal card identity out of any viewer projection surface.
- Added conservation and determinism tests across 3–7 seats, including same seed/hand reproduction, different-seed divergence, seed partitioning by hand index, trump non-playability, and complete 52-card conservation.
- Added native golden trace fixtures for 3-seat setup, 7-seat setup, and deterministic turn-up trump/hidden tail; `golden_traces.rs` regenerates and compares them byte-for-byte.

Verification:

- `cargo fmt --all --check` passed.
- `cargo test -p vow_tide` passed: 11 integration tests plus crate/doc test harnesses.
- `cargo test -p vow_tide --test property` passed: 4 property tests.
- `replay-check --game vow_tide` was not run because the ticket explicitly defers tool registration to 016; native trace assertion covered this ticket's deterministic trace boundary.

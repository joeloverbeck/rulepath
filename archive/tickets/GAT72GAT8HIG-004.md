# GAT72GAT8HIG-004: State, setup, deterministic shuffle/deal + unbiased bounded-index RNG helper

**Status**: COMPLETED
**Priority**: HIGH
**Effort**: Medium
**Engine Changes**: Yes — `games/high_card_duel/src/{ids.rs,state.rs,setup.rs}`; a game-local unbiased bounded-index RNG helper
**Deps**: GAT72GAT8HIG-003

## Problem

Gate 8 is the chance proof. It needs a Rust-owned deterministic shuffle/deal
from the match `Seed` that builds the 24-card local deck in canonical order,
shuffles it unbiasedly, deals three private cards per seat, and stores the
remaining deck order internally only. The shuffle must not silently rely on the
modulo-reduced `DeterministicRng::next_index` (biased for a finite deck).

## Assumption Reassessment (2026-06-07)

1. Verified the RNG surface: `crates/engine-core/src/rng.rs:10-15` —
   `DeterministicRng::next_index(upper_bound)` is implemented as
   `next_u64() % upper_bound`, i.e. modulo reduction (biased). `next_u64()` is
   the unbiased primitive (`rng.rs:31`).
2. Verified against the spec: §4.2.3 mandates a named/versioned shuffle, same
   seed ⇒ identical deck/deal, and **preferred** option = a game-local
   `next_bounded_index_unbiased(upper_bound)` using rejection sampling over
   `next_u64`, with tests. The engine-core `next_index_unbiased` alternative is
   explicitly an ADR-gated path and is NOT taken here (preferred option avoids
   §13).
3. Cross-artifact boundary under audit: the deterministic-RNG contract
   (`engine-core` `Seed` / `DeterministicRng`) consumed game-locally — the helper
   lives in `games/high_card_duel`, never in `engine-core` (no card vocabulary in
   the kernel; the helper is generic-shaped but kept local per the spec).
4. FOUNDATIONS principle under audit (§11 determinism): same seed + same variant
   must yield identical deck/deal/internal trace; the shuffle is the canonical
   RNG-consuming step and must be deterministic and unbiased.
5. Enforcement surface named: deterministic replay/hash (§11/§13). The shuffle
   feeds the internal trace that GAT72GAT8HIG-009/012 replay-hash; confirm the
   helper introduces no wall-clock/`std::time` seeding and no nondeterministic
   iteration. The deck order is stored internally only — no hidden-info leak path
   is opened here (the no-leak firewall is enforced by the views ticket 008).

## Architecture Check

1. A game-local rejection-sampling helper is cleaner than promoting an RNG
   utility to `engine-core`: it proves unbiased shuffle without a kernel change
   or an ADR (spec §4.2.3 preferred), and keeps the chance proof self-contained.
2. No backwards-compatibility shims — `next_index`'s modulo path is untouched
   (used by other games under their current docs/tests); this ticket does not
   silently re-route it.
3. `engine-core` stays noun-free and unchanged; the helper is game-local. No
   `game-stdlib` promotion (cards are first use, §4).

## Verification Layers

1. Deterministic deal -> golden trace / deterministic replay-hash check: same seed + variant ⇒ byte-identical internal deck/deal in a unit test.
2. Unbiased shuffle -> manual review + targeted test: rejection-sampling helper rejects the biased high-residue band; a statistical/structural test documents the bound (per spec §4.2.3 item 7).
3. Deck secrecy substrate -> schema/serialization validation: remaining deck order lives in internal state only; public/observer projection (built later) has no field carrying it.
4. RNG contract -> FOUNDATIONS alignment check: no `std::time`/wall-clock seeding; seed flows from the engine `Seed` contract (§2/§11).

## What to Change

### 1. `ids.rs` + `state.rs`

Card identity (`hcd:rNN:a|b`, rank 1–12), seat hands, committed cards, revealed
history, internal deck order, round/phase/score/lead-seat state.

### 2. Unbiased bounded-index helper

Game-local `next_bounded_index_unbiased(rng, upper_bound)` via rejection
sampling over `DeterministicRng::next_u64`, with a documented name/version.

### 3. `setup.rs`

`HCD-SETUP-001..005`: build canonical 24-card deck, shuffle with the helper,
deal three private cards alternating `seat_0`/`seat_1`, init round 1 / score 0-0
/ phase `lead_commit` / lead `seat_0`, store remaining deck internally.

## Files to Touch

- `games/high_card_duel/src/ids.rs` (modify — fill stub)
- `games/high_card_duel/src/state.rs` (modify — fill stub)
- `games/high_card_duel/src/setup.rs` (modify — fill stub)

## Out of Scope

- Legal actions / transitions / views / effects (later tickets).
- Any change to `engine-core` `rng.rs` (the engine `next_index_unbiased` option is ADR-gated and not taken).

## Acceptance Criteria

### Tests That Must Pass

1. `same_seed_same_initial_deal_internal` — identical deck/deal across two setups with the same seed/variant.
2. `different_seeds_can_change_initial_deal` — different seeds can produce different deals.
3. `shuffle_uses_unbiased_bounded_index_or_documented_helper` — the helper is exercised and its bound documented/tested.
4. `setup_deals_private_hands_and_hides_deck` — three cards per seat; remaining deck order held internally.

### Invariants

1. Setup is deterministic under (seed, variant); no nondeterministic input enters the deal.
2. No duplicate cards across deck/hands; conservation holds at setup.

## Test Plan

### New/Modified Tests

1. `games/high_card_duel/tests/rules.rs` — setup/shuffle determinism + hand-size + deck-hidden assertions (added here, extended by later rules tickets).

### Commands

1. `cargo test -p high_card_duel --test rules setup`
2. `cargo test -p high_card_duel`
3. The per-test filter is the correct boundary now; full visibility/property proofs land with tickets 008/011.

## Outcome

Completed: 2026-06-07

What changed:

- Added game-local `CardId`, `Sigil`, canonical 24-card deck construction, and stable card IDs `hcd:rNN:a|b`.
- Added High Card Duel state containers for seats, phase, score, private hands, face-down commitments, revealed history, internal remaining deck order, and freshness.
- Added deterministic `setup_match` using the engine `Seed` contract, canonical deck construction, `hcd-shuffle-v1` Fisher-Yates shuffle, and alternating three-card private deal.
- Added game-local `next_bounded_index_unbiased` rejection sampling over `DeterministicRng::next_u64`; `engine-core::DeterministicRng::next_index` remains unchanged.
- Added integration setup tests in `games/high_card_duel/tests/rules.rs`.

Deviations from original plan:

- None.

Verification results:

- `cargo fmt --all --check` passed.
- `cargo test -p high_card_duel --test rules setup` passed.
- `cargo test -p high_card_duel` passed.

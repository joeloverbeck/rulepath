# RIVLEDSHOSEA-001: Preserve canonical winner order; make button order remainder-only

**Status**: PENDING
**Priority**: HIGH
**Effort**: Medium
**Engine Changes**: Yes — `games/river_ledger` (`src/pot.rs`, `src/showdown.rs`, `tests/property.rs`, `tests/rules.rs`)
**Deps**: None

## Problem

At the shipped baseline, `allocate_single_pot()` reorders the canonical winner set into button order via `winners_in_button_order()` and stores that order in `PotAllocation.winners`; `resolve_showdown()` then writes `TerminalOutcome::Showdown.winners = allocation.winners`, and `explain_showdown()` / `showdown_presentation_v2()` consume `&allocation`. Button order is the legitimate `RL-POT-REMAINDER-001` policy for assigning indivisible extra units — but it is being used as a second definition of the *semantic* winner set and the "primary" narrated winner. This cannot change a singleton winner, but for tied winners it makes narration order, list order, standings, accessibility copy, and serialized allocation order depend on a payout-specific order (spec §2.4). This ticket separates the two concepts at their source: canonical semantic order versus remainder order.

## Assumption Reassessment (2026-06-18)

1. `games/river_ledger/src/pot.rs::allocate_single_pot(pot_total, winners, button, seat_count) -> PotAllocation` calls `winners_in_button_order(winners, button, seat_count)` (pot.rs) and stores the result in `PotAllocation.winners`; `PotAllocation` (pot.rs) carries `winners`, `shares: Vec<PotShare>`, `remainder`, `remainder_order`. `PotShare` is `{ seat, amount }` in `src/state.rs`. Confirmed against current code.
2. `games/river_ledger/src/showdown.rs::resolve_showdown()` builds `winners = winning_seats(&evaluations)` (canonical evaluation/stable-seat order), computes `headline`/`decisive_comparison`/`comparison_basis` from `&winners`, but emits `TerminalOutcome::Showdown { winners: allocation.winners, allocations: allocation.shares, .. }` and passes `&allocation` to `explain_showdown` / `showdown_presentation_v2`. Spec §2.4 / §7.1-B describe this exact split; confirmed.
3. Shared boundary under audit: the `PotAllocation` → `TerminalOutcome::Showdown` carrier and the serialized `winners` / `allocations` arrays (read by replay/serialization tests and the WASM view projection). The semantic winner order must equal `state.seats`/evaluation order; remainder order stays button-relative.
4. FOUNDATIONS §11 (deterministic, conservation-respecting outcomes): canonical winners are nonempty/unique, allocations sum to `pot_total`, and the serialized winner array is deterministic and order-stable. Restated before trusting the spec narrative.
5. Determinism/serialization surface: this changes the order of `TerminalOutcome::Showdown.winners` and `allocations` for tied outcomes from button order to canonical order. That is an intentional deterministic-output change; it introduces no nondeterministic input and no hidden-information path. Golden-trace/replay reconciliation is owned by RIVLEDSHOSEA-004 (see Out of Scope) — this ticket changes only logic + native unit/property tests.
6. `RL-POT-REMAINDER-001` (RULES.md) is unchanged: extra indivisible units still follow button order among tied winners. Only the *representation* (remainder-only, not the semantic set) changes.
7. Adjacent contradiction: because serialized winner/allocation order changes for tied cases, existing golden traces (`split-pot-remainder-button-order.trace.json`, `split-pot-even.trace.json`) and `replay-check` will go red between this ticket and RIVLEDSHOSEA-004. Classified as a *required consequence* owned by RIVLEDSHOSEA-004, not a separate bug.

## Architecture Check

1. Representing button order solely as `remainder_order`/remainder recipients (not as `PotAllocation.winners`) makes the two concepts non-aliasable: downstream code can no longer accidentally read payout order as semantic order. Cleaner than threading a "which order is this" flag through every consumer.
2. No backwards-compatibility shim: the canonical winner order becomes the single source; the button-ordered vector is not retained under an alias.
3. `engine-core` stays free of mechanic nouns — all pot/winner/seat logic remains in `games/river_ledger`; no `game-stdlib` promotion is proposed.

## Verification Layers

1. Canonical winner order preserved through allocation -> `pot.rs` unit test asserting `allocation.winners == input canonical winners` for a tied set with a non-trivial button.
2. Remainder follows button order -> `pot.rs` unit test asserting remainder recipients/`remainder_order` follow `winners_in_button_order` while shares serialize in canonical order.
3. Conservation + uniqueness + determinism -> `tests/property.rs` property over generated winner subsets, seat counts `3..=6`, button positions, pot totals: winner-set equality, `sum(shares) == pot_total`, unique winners, deterministic ordering, remainder policy.
4. Seed `31` regression -> `tests/rules.rs` asserting canonical winners `[seat_1, seat_2, seat_3]`, remainder order `[seat_2, seat_3, seat_1]`, correct per-seat shares.

## What to Change

### 1. `pot.rs` — make button order remainder-only

Keep `PotAllocation.winners` in canonical input order (do not overwrite with `winners_in_button_order`). Use button order only to compute `remainder_order`/remainder recipients. Emit `PotShare` entries in canonical winner order while each amount is computed from the remainder-recipient set/order. Pot conservation is unchanged.

### 2. `showdown.rs` — name and route canonical order

Ensure `resolve_showdown()` passes canonical winner order to `TerminalOutcome::Showdown.winners` and to every narration/presentation builder. Name the `winning_seats()` result `canonical_winners` at call sites so review can distinguish it from payout order. (Full single-source consolidation and invariant assertions land in RIVLEDSHOSEA-003; this ticket fixes the order semantics and adds the seed-`31` evidence.)

### 3. Tests

Add the seed-`31`/button-`seat_2` regression and the allocation property described in Verification Layers.

## Files to Touch

- `games/river_ledger/src/pot.rs` (modify)
- `games/river_ledger/src/showdown.rs` (modify)
- `games/river_ledger/tests/property.rs` (modify)
- `games/river_ledger/tests/rules.rs` (modify)

## Out of Scope

- Golden-trace regeneration, `replay-check`/`fixture-check` reconciliation, and `RULE-COVERAGE.md` rows (owned by RIVLEDSHOSEA-004; `replay-check` may be red until then).
- Single-source `ResolvedShowdown` consolidation and invariant assertions (RIVLEDSHOSEA-003).
- Public seat-label unification (RIVLEDSHOSEA-002).
- Changing `RL-POT-REMAINDER-001` button-order remainder policy.
- Any TypeScript change.

## Acceptance Criteria

### Tests That Must Pass

1. New `pot.rs` tests: canonical winner order preserved; remainder recipients follow button order; shares serialize in canonical order; total conserved.
2. New `tests/rules.rs` seed-`31` case: canonical winners `[seat_1, seat_2, seat_3]`, remainder order `[seat_2, seat_3, seat_1]`, correct shares.
3. `cargo test -p river_ledger` (targeted; golden-trace/replay assertions excluded — see Out of Scope).

### Invariants

1. `PotAllocation.winners` equals the canonical (evaluation/stable-seat) winner order; button order appears only in `remainder_order`/remainder recipients.
2. `sum(shares) == pot_total`; winners are unique and nonempty; no non-winner holds a positive share.

## Test Plan

### New/Modified Tests

1. `games/river_ledger/tests/rules.rs` — seed-`31`/button-`seat_2` split regression locking canonical-vs-remainder order.
2. `games/river_ledger/tests/property.rs` — allocation property over winner subsets, seat counts `3..=6`, button positions, pot totals.

### Commands

1. `cargo test -p river_ledger pot` and `cargo test -p river_ledger -- split` (targeted unit/property/regression).
2. `cargo fmt --all --check && cargo clippy --workspace --all-targets -- -D warnings && cargo build --workspace`
3. Full `cargo run -p replay-check -- --game river_ledger --all` is intentionally deferred to RIVLEDSHOSEA-004 (it reconciles the traces this change reorders), so it is not a gate for this ticket.

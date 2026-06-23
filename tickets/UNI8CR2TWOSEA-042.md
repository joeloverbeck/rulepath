# UNI8CR2TWOSEA-042: High Card Duel — unbiased bounded-index adoption

**Status**: PENDING
**Priority**: HIGH
**Effort**: Medium
**Engine Changes**: Yes (deterministic evidence) — `games/high_card_duel/src/setup.rs`; replaces the local sampler call with `engine-core` `DeterministicRng::next_index_unbiased_v1`
**Deps**: 013

## Problem

Spec §3.10 / task `8C-R2-701`: HCD's local `next_bounded_index_unbiased` is algorithmically the shipped v1 sampler. R2 replaces only the local call with `DeterministicRng::next_index_unbiased_v1`, altering no loop bounds, shuffle order, deal order, seed handling, or game policy. Any fixed-vector / draw-count divergence stops the migration. Shares `setup.rs` with `-013` (hence `Deps: 013`).

## Assumption Reassessment (2026-06-23)

1. `games/high_card_duel/src/setup.rs::{shuffle_deck,next_bounded_index_unbiased}` exist (confirmed ~lines 65/75); `engine-core::DeterministicRng::next_index_unbiased_v1` exists (`crates/engine-core/src/rng.rs:31`).
2. Spec §3.10/§9: replace only the local call; expected ADR-0009 class is `unchanged`; any observed fixed-vector/draw-count divergence blocks this game's migration; Secret Draft is the explicit C-09 N/A.
3. Cross-crate boundary under audit: `engine-core::DeterministicRng::next_index_unbiased_v1` — the generic unbiased sampler; loop bounds and seed handling stay game-local.
4. Determinism / no-leak: returned indices, rejection draw counts, the full shuffled deck/deal vectors, and all downstream private effect/view and state/replay/export hashes must stay byte-identical to the `-001` baseline (§11); the migration is byte-neutral, not a behavior change.

## Architecture Check

1. Calling the shared unbiased sampler removes a duplicated local rejection loop while leaving loop bounds and seed handling in game code — earns no new `game-stdlib`/kernel surface (the API already ships).
2. No backwards-compat alias; the local helper call is replaced (the local `fn` is removed if fully superseded, restored on rollback).
3. `engine-core` stays noun-free; no `game-stdlib` change.

## Verification Layers

1. Identical returned indices + rejection draw counts -> deterministic replay-hash check + fixed-word vectors (`cargo test -p high_card_duel`).
2. Full shuffle/deal vectors and state/effect/view/replay/export hashes unchanged -> `replay-check --game high_card_duel --all`.
3. `DeterministicRng::next_index_unbiased_v1` adoption -> codebase grep-proof in `setup.rs`.

## What to Change

### 1. Adopt the shared unbiased sampler

Replace the local `next_bounded_index_unbiased` call site in `setup.rs` (used by `shuffle_deck`) with `DeterministicRng::next_index_unbiased_v1`, keeping loop bounds, deal/shuffle order, and seed handling unchanged.

## Files to Touch

- `games/high_card_duel/src/setup.rs` (modify; serialized after `-013`)

## Out of Scope

- Any loop-bound, shuffle/deal-order, seed-meaning, or game-policy change.
- The C-03 structural-count migration (`-013`).

## Acceptance Criteria

### Tests That Must Pass

1. `cargo test -p high_card_duel` green, with fixed-word and rejection-count vectors.
2. `cargo run -p replay-check -- --game high_card_duel --all` — shuffle/deal vectors and all downstream hashes byte-identical to baseline.

### Invariants

1. Returned indices, rejection draw counts, and shuffle/deal vectors are byte-identical to baseline.
2. No RNG algorithm, loop order, or seed meaning changes.

## Test Plan

### New/Modified Tests

1. `games/high_card_duel/src/setup.rs` (or `tests/`) — fixed-word vector + rejection-count + shuffle/deal-equality assertions.

### Commands

1. `cargo test -p high_card_duel`
2. `cargo run -p replay-check -- --game high_card_duel --all`

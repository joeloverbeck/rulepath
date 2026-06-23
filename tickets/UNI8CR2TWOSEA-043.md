# UNI8CR2TWOSEA-043: Poker Lite — unbiased bounded-index adoption

**Status**: PENDING
**Priority**: HIGH
**Effort**: Medium
**Engine Changes**: Yes (deterministic evidence) — `games/poker_lite/src/setup.rs`; replaces the local rejection sampler call with `engine-core` `DeterministicRng::next_index_unbiased_v1`
**Deps**: 015

## Problem

Spec §3.10 / task `8C-R2-702`: Poker Lite has a local rejection sampler in `setup.rs`. R2 replaces only the local call with `DeterministicRng::next_index_unbiased_v1`, altering no loop bounds, shuffle order, deal order, seed handling, or game policy — including private hands and showdown/yield traces. Any fixed-vector / draw-count divergence stops the migration. Shares `setup.rs` with `-015` (hence `Deps: 015`).

## Assumption Reassessment (2026-06-23)

1. `games/poker_lite/src/setup.rs::{shuffle_deck,next_bounded_index_unbiased}` exist (confirmed ~lines 55/70); `engine-core::DeterministicRng::next_index_unbiased_v1` exists (`rng.rs:31`).
2. Spec §3.10/§9: replace only the local call; expected ADR-0009 class is `unchanged`; any fixed-vector/draw-count divergence blocks this game's migration.
3. Cross-crate boundary under audit: `engine-core::DeterministicRng::next_index_unbiased_v1` — the generic unbiased sampler; loop bounds and seed handling stay game-local.
4. Determinism / no-leak: returned indices, rejection draw counts, full shuffle/deal vectors including private hands, and all downstream state/effect/view/replay/export hashes (incl. showdown/yield traces) must stay byte-identical to the `-001` baseline (§11); byte-neutral, not a behavior change.

## Architecture Check

1. Calling the shared unbiased sampler removes a duplicated local rejection loop while leaving loop bounds and seed handling in game code — no new shared surface earned.
2. No backwards-compat alias; the local helper call is replaced (local `fn` removed if fully superseded, restored on rollback).
3. `engine-core` stays noun-free; no `game-stdlib` change.

## Verification Layers

1. Identical returned indices + rejection draw counts -> deterministic replay-hash check + fixed-word vectors (`cargo test -p poker_lite`).
2. Full shuffle/deal vectors (private hands) and showdown/yield traces unchanged -> `replay-check --game poker_lite --all`.
3. `DeterministicRng::next_index_unbiased_v1` adoption -> codebase grep-proof in `setup.rs`.

## What to Change

### 1. Adopt the shared unbiased sampler

Replace the local rejection-sampler call site in `setup.rs` with `DeterministicRng::next_index_unbiased_v1`, keeping loop bounds, deal/shuffle order, and seed handling unchanged.

## Files to Touch

- `games/poker_lite/src/setup.rs` (modify; serialized after `-015`)

## Out of Scope

- Any loop-bound, shuffle/deal-order, seed-meaning, or game-policy change.
- The C-03 structural-count migration (`-015`).

## Acceptance Criteria

### Tests That Must Pass

1. `cargo test -p poker_lite` green, with fixed-word and rejection-count vectors.
2. `cargo run -p replay-check -- --game poker_lite --all` — shuffle/deal/private-hand vectors and showdown/yield hashes byte-identical to baseline.

### Invariants

1. Returned indices, rejection draw counts, and shuffle/deal vectors are byte-identical to baseline.
2. No RNG algorithm, loop order, or seed meaning changes.

## Test Plan

### New/Modified Tests

1. `games/poker_lite/src/setup.rs` (or `tests/`) — fixed-word vector + rejection-count + shuffle/deal-equality assertions.

### Commands

1. `cargo test -p poker_lite`
2. `cargo run -p replay-check -- --game poker_lite --all`

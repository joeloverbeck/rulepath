# UNI8CR2TWOSEA-044: Masked Claims — unbiased bounded-index adoption

**Status**: COMPLETED
**Priority**: HIGH
**Effort**: Medium
**Engine Changes**: Yes (deterministic evidence) — `games/masked_claims/src/setup.rs`; replaces the local rejection sampler call with `engine-core` `DeterministicRng::next_index_unbiased_v1`
**Deps**: 016

## Problem

Spec §3.10 / task `8C-R2-703`: Masked Claims has a local rejection sampler in `setup.rs`. R2 replaces only the local call with `DeterministicRng::next_index_unbiased_v1`, altering no loop bounds, shuffle order, deal order, seed handling, or game policy — including hands/reserve, pending claim, and export redaction. Any fixed-vector / draw-count divergence stops the migration. Shares `setup.rs` with `-016` (hence `Deps: 016`).

## Assumption Reassessment (2026-06-23)

1. `games/masked_claims/src/setup.rs::{shuffle_masks,next_bounded_index_unbiased}` exist (confirmed ~lines 71/86); `engine-core::DeterministicRng::next_index_unbiased_v1` exists (`rng.rs:31`).
2. Spec §3.10/§9: replace only the local call; expected ADR-0009 class is `unchanged`; any fixed-vector/draw-count divergence blocks this game's migration.
3. Cross-crate boundary under audit: `engine-core::DeterministicRng::next_index_unbiased_v1` — the generic unbiased sampler; loop bounds and seed handling stay game-local.
4. Determinism / no-leak: returned indices, rejection draw counts, full shuffle/deal vectors including hands/reserve, and all downstream state/effect/view/replay/export hashes (incl. pending-claim and export redaction) must stay byte-identical to the `-001` baseline (§11); byte-neutral, not a behavior change.

## Architecture Check

1. Calling the shared unbiased sampler removes a duplicated local rejection loop while leaving loop bounds and seed handling in game code — no new shared surface earned.
2. No backwards-compat alias; the local helper call is replaced (local `fn` removed if fully superseded, restored on rollback).
3. `engine-core` stays noun-free; no `game-stdlib` change.

## Verification Layers

1. Identical returned indices + rejection draw counts -> deterministic replay-hash check + fixed-word vectors (`cargo test -p masked_claims`).
2. Full shuffle vectors (hands/reserve), pending-claim and export-redaction hashes unchanged -> `replay-check --game masked_claims --all`.
3. `DeterministicRng::next_index_unbiased_v1` adoption -> codebase grep-proof in `setup.rs`.

## What to Change

### 1. Adopt the shared unbiased sampler

Replace the local rejection-sampler call site in `setup.rs` (used by `shuffle_masks`) with `DeterministicRng::next_index_unbiased_v1`, keeping loop bounds, deal/shuffle order, and seed handling unchanged.

## Files to Touch

- `games/masked_claims/src/setup.rs` (modify; serialized after `-016`)

## Out of Scope

- Any loop-bound, shuffle/deal-order, seed-meaning, or game-policy change.
- The C-03 structural-count migration (`-016`).

## Acceptance Criteria

### Tests That Must Pass

1. `cargo test -p masked_claims` green, with fixed-word and rejection-count vectors.
2. `cargo run -p replay-check -- --game masked_claims --all` — shuffle/hand/reserve vectors and pending-claim/export hashes byte-identical to baseline.

### Invariants

1. Returned indices, rejection draw counts, and shuffle/deal vectors are byte-identical to baseline.
2. No RNG algorithm, loop order, or seed meaning changes.

## Test Plan

### New/Modified Tests

1. `games/masked_claims/src/setup.rs` (or `tests/`) — fixed-word vector + rejection-count + shuffle-equality assertions.

### Commands

1. `cargo test -p masked_claims`
2. `cargo run -p replay-check -- --game masked_claims --all`

## Outcome

Completed: 2026-06-23

Masked Claims now calls `DeterministicRng::next_index_unbiased_v1(index + 1)`
directly from `games/masked_claims/src/setup.rs::shuffle_masks`. The
duplicated local `next_bounded_index_unbiased` helper was removed. Existing
setup tests were retargeted to the shared sampler and now assert both the fixed
high-residue vector and draw counts: bound `3` still rejects once then returns
index `1`, and zero-bound sampling returns `None` without drawing.

No loop bounds, shuffle/deal order, seed meaning, hands/reserve policy,
pending-claim behavior, export redaction, or replay artifacts were changed.
ADR-0009 classification remains `unchanged`.

Verification:

- `cargo test -p masked_claims bounded_index_rejects_high_residue_band -- --nocapture` passed.
- `cargo test -p masked_claims bounded_index_rejects_empty_bound -- --nocapture` passed.
- `rg -n "next_bounded_index_unbiased|next_index_unbiased_v1" games/masked_claims/src/setup.rs games/masked_claims/tests` shows only `next_index_unbiased_v1` in the shuffle call and setup tests.
- `cargo fmt --all --check` passed.
- `cargo test -p masked_claims` passed.
- `cargo run -p replay-check -- --game masked_claims --all` passed; all Masked Claims traces were accepted through the current replay-check not-applicable baseline path.

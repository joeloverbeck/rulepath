# 8CR3PUBCOOASY-701: C-09 Plain Tricks unbiased-index migration

**Status**: COMPLETED
**Priority**: HIGH
**Effort**: Small
**Engine Changes**: Yes (deterministic evidence) — `games/plain_tricks/src/setup.rs`
**Deps**: 8CR3PUBCOOASY-301

## Problem

`plain_tricks` has a local rejection-sampling bounded-index helper
(`next_bounded_index_unbiased`) used by `shuffle_deck`. C-09 replaces only the
local bounded-index call with the shipped
`DeterministicRng::next_index_unbiased_v1`, after proving exact RNG-word,
rejection-count, shuffle/deal, and downstream-hash identity. Seed handling, RNG
consumption, rejection behavior, and shuffle/deal order are preserved exactly.

## Assumption Reassessment (2026-06-24)

1. `games/plain_tricks/src/setup.rs::{shuffle_deck (line ~78),
   next_bounded_index_unbiased (line ~101)}` exist; the shipped
   `DeterministicRng::next_index_unbiased_v1` is at
   `crates/engine-core/src/rng.rs:31`, with a
   `next_index_unbiased_v1_matches_existing_local_algorithm` test confirming
   semantic identity to the local algorithm.
2. Spec §3.10 / §5.14 task `8C-R3-701` scopes replacing the local sampler call
   only, after exact vector/draw-count proof; expected ADR-0009 classification
   `unchanged`. Serialized after 301 (same `setup.rs`; the C-03 setup baseline
   is frozen first per §11.2).
3. Cross-crate boundary under audit: `DeterministicRng::next_index_unbiased_v1`
   vs the local rejection sampler — only the bounded-index call moves; shuffle/
   deal construction and game policy stay local.
4. FOUNDATIONS §11/§13 (determinism): the migration must preserve RNG word
   consumption, rejection counts, and downstream hashes exactly — a single
   vector/draw-count/hash mismatch blocks this task and restores the local helper.
5. Enforcement surface: fixed-word shuffle/deal vectors, rejection paths, full
   deck order, hands/tail, private deal effects/views, and state/replay/export
   hashes; all byte-identical to the 001 baseline.

## Architecture Check

1. Replacing the local sampler with the shipped engine helper removes a
   duplicated rejection-sampling algorithm while preserving exact output;
   cleaner than maintaining a per-game copy.
2. No backwards-compatibility alias — the local helper and its single call path
   are removed (restored only on an identity failure).
3. `engine-core` already owns the unbiased-index contract; no mechanic noun
   added, no `game-stdlib` change. This does not centralize shuffle/deal.

## Verification Layers

1. RNG identity -> `cargo test -p plain_tricks` setup/shuffle vector tests
   (fixed RNG words, rejection draws, full deck order identical to baseline).
2. Downstream hash/visibility neutrality -> `replay-check --game plain_tricks
   --all` + serialization tests byte-identical to baseline (hands/tail, private
   deal effects/views, state/replay/export hashes unchanged).
3. Draw-count identity -> rejection draw counts equal the baseline (no extra or
   fewer RNG words consumed).

## What to Change

### 1. Adopt `next_index_unbiased_v1`

In `games/plain_tricks/src/setup.rs`, replace the local
`next_bounded_index_unbiased` call inside `shuffle_deck` with
`rng.next_index_unbiased_v1(...)`. Remove the now-unused local helper. Preserve
seed handling, RNG consumption, rejection behavior, and shuffle/deal order.

## Files to Touch

- `games/plain_tricks/src/setup.rs` (modify; serialized after 301)

## Out of Scope

- Centralizing shuffle/deal/deck construction or any game rule.
- Changing seeds, word consumption, rejection behavior, or loop bounds.
- Introducing a new RNG algorithm.

## Acceptance Criteria

### Tests That Must Pass

1. `cargo test -p plain_tricks` (shuffle/deal vector + draw-count tests).
2. `cargo run -p replay-check -- --game plain_tricks --all` — byte-identical to baseline.
3. `cargo run -p fixture-check -- --game plain_tricks`.

### Invariants

1. RNG words, rejection counts, full deck/deal order, and all downstream hashes
   are identical to baseline.
2. On any identity mismatch, the local helper is restored (task blocked).

## Test Plan

### New/Modified Tests

1. `None — behavior-neutral adoption; the existing shuffle/deal vector tests, replay/serialization suites, and the 001 baseline are the regression guard (a mismatch blocks the task).`

### Commands

1. `cargo test -p plain_tricks`
2. `cargo run -p replay-check -- --game plain_tricks --all`
3. A per-game test + replay-check is the correct boundary: RNG identity and
   downstream hashes are the surfaces this migration must preserve.

## Outcome

- Replaced Plain Tricks' local `next_bounded_index_unbiased` call inside
  `shuffle_deck` with `DeterministicRng::next_index_unbiased_v1` and removed
  the now-unused local helper.
- Retargeted the existing setup rejection-vector tests to the shared RNG
  method, preserving zero-bound and high-residue rejection coverage. Shuffle,
  deal, replay, fixture, and export surfaces remained byte-identical under the
  existing gates.
- Verification passed:
  - `cargo test -p plain_tricks`
  - `cargo run -p replay-check -- --game plain_tricks --all`
  - `cargo run -p fixture-check -- --game plain_tricks`
  - `git diff --check`

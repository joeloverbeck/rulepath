# 8CR3PUBCOOASY-703: C-09 Event Frontier unbiased-index migration

**Status**: COMPLETED
**Priority**: HIGH
**Effort**: Small
**Engine Changes**: Yes (deterministic evidence) — `games/event_frontier/src/setup.rs`
**Deps**: 8CR3PUBCOOASY-308

## Problem

`event_frontier` has a local rejection-sampling bounded-index helper
(`next_bounded_index_unbiased`) used by `shuffle_epoch` (and `build_seeded_deck`).
C-09 replaces only the local bounded-index call with
`DeterministicRng::next_index_unbiased_v1`, after proving exact RNG-word,
rejection-count, per-epoch deck order, current/next window, and downstream-hash
identity. `frontier_control` is RNG-free (C-09 N/A — recorded by 802).

## Assumption Reassessment (2026-06-24)

1. `games/event_frontier/src/setup.rs::{build_seeded_deck (line ~115),
   shuffle_epoch (line ~147), next_bounded_index_unbiased (line ~266)}` exist;
   the shipped `DeterministicRng::next_index_unbiased_v1` is at
   `crates/engine-core/src/rng.rs:31` (with the matches-local-algorithm test).
2. Spec §3.10 / §5.14 task `8C-R3-703` scopes replacing the local sampler call
   only, preserving epoch shuffle/deck order, current/next window, seed and
   rejection counts; expected ADR-0009 classification `unchanged`. Serialized
   after 308 (the last Event C-03 setup predicate; setup baseline frozen first).
3. Cross-crate boundary under audit: `next_index_unbiased_v1` vs the local
   rejection sampler — only the bounded-index call moves; epoch/deck
   construction and the current/next reveal stay local.
4. FOUNDATIONS §11/§13 (determinism): preserve RNG word consumption, rejection
   counts, per-epoch order, and downstream hashes exactly — a single mismatch
   blocks this task and restores the local helper. The hidden deeper tail must
   not leak.
5. Enforcement surface: fixed-word epoch-shuffle vectors, rejection paths,
   per-epoch order, current/next/deeper tail, event visibility, and replay/export
   hashes; all byte-identical to the 001 baseline.

## Architecture Check

1. Replacing the local sampler with the shipped engine helper removes a
   duplicated rejection-sampling algorithm while preserving exact output.
2. No backwards-compatibility alias — the local helper and its single call path
   are removed (restored only on an identity failure).
3. `engine-core` already owns the unbiased-index contract; no mechanic noun
   added, no `game-stdlib` change. This does not centralize epoch partitioning.

## Verification Layers

1. RNG identity -> `cargo test -p event_frontier` epoch-shuffle vector tests
   (fixed RNG words, rejection draws, per-epoch deck order identical to baseline).
2. Downstream hash/visibility neutrality -> `replay-check --game event_frontier
   --all` + serialization tests byte-identical to baseline (current/next/deeper
   tail, event visibility, replay/export hashes unchanged).
3. Draw-count identity / no-leak -> rejection draw counts equal the baseline;
   the hidden deeper tail remains absent from public surfaces.

## What to Change

### 1. Adopt `next_index_unbiased_v1`

In `games/event_frontier/src/setup.rs`, replace the local
`next_bounded_index_unbiased` call inside `shuffle_epoch` (and any deck-build
call site) with `rng.next_index_unbiased_v1(...)`. Remove the now-unused local
helper. Preserve seed handling, RNG consumption, rejection behavior, and
per-epoch order.

## Files to Touch

- `games/event_frontier/src/setup.rs` (modify; serialized after 308)

## Out of Scope

- Centralizing epoch partitioning / deck construction or any game rule.
- Changing seeds, word consumption, rejection behavior, or loop bounds.
- Introducing a new RNG algorithm; Frontier Control's C-09 N/A (recorded by 802).

## Acceptance Criteria

### Tests That Must Pass

1. `cargo test -p event_frontier` (epoch-shuffle vector + draw-count tests).
2. `cargo run -p replay-check -- --game event_frontier --all` — byte-identical to baseline.
3. `cargo run -p fixture-check -- --game event_frontier`.

### Invariants

1. RNG words, rejection counts, per-epoch order, current/next window, and all
   downstream hashes are identical to baseline.
2. On any identity mismatch, the local helper is restored (task blocked); the
   hidden deeper tail never leaks.

## Test Plan

### New/Modified Tests

1. `None — behavior-neutral adoption; the existing epoch-shuffle vector tests, replay/serialization suites, and the 001 baseline are the regression guard (a mismatch blocks the task).`

### Commands

1. `cargo test -p event_frontier`
2. `cargo run -p replay-check -- --game event_frontier --all`
3. A per-game test + replay-check is the correct boundary: RNG identity and
   downstream hashes are the surfaces this migration must preserve.

## Outcome

- Replaced Event Frontier's local `next_bounded_index_unbiased` calls inside
  `shuffle_epoch` and the non-Reckoning first-card swap in `build_seeded_deck`
  with `DeterministicRng::next_index_unbiased_v1`, then removed the now-unused
  local helper.
- Existing epoch/deck-order, no-leak, replay, fixture, and export gates
  preserved the per-epoch order, current/next window, deeper-tail privacy, and
  downstream hashes.
- Verification passed:
  - `cargo test -p event_frontier`
  - `cargo run -p replay-check -- --game event_frontier --all`
  - `cargo run -p fixture-check -- --game event_frontier`
  - `git diff --check`

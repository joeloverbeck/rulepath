# 8CR3PUBCOOASY-702: C-09 Flood Watch unbiased-index migration

**Status**: COMPLETED
**Priority**: HIGH
**Effort**: Small
**Engine Changes**: Yes (deterministic evidence) — `games/flood_watch/src/setup.rs`
**Deps**: 8CR3PUBCOOASY-304

## Problem

`flood_watch` has a local rejection-sampling bounded-index helper
(`next_bounded_index_unbiased`) used by `shuffle_event_deck`. C-09 replaces only
the local bounded-index call with `DeterministicRng::next_index_unbiased_v1`,
after proving exact RNG-word, rejection-count, event-deck order, forecast, and
downstream-hash identity. Seed handling and RNG consumption are preserved.

## Assumption Reassessment (2026-06-24)

1. `games/flood_watch/src/setup.rs::{shuffle_event_deck (line ~75),
   next_bounded_index_unbiased (line ~114)}` exist; the shipped
   `DeterministicRng::next_index_unbiased_v1` is at
   `crates/engine-core/src/rng.rs:31` (with the matches-local-algorithm test).
2. Spec §3.10 / §5.14 task `8C-R3-702` scopes replacing the local sampler call
   only, preserving event-deck order, forecast, draws, seed and rejection
   counts; expected ADR-0009 classification `unchanged`. Serialized after 304
   (the last Flood C-03 setup predicate; setup baseline frozen first).
3. Cross-crate boundary under audit: `next_index_unbiased_v1` vs the local
   rejection sampler — only the bounded-index call moves; event-deck/forecast
   construction stays local.
4. FOUNDATIONS §11/§13 (determinism): preserve RNG word consumption, rejection
   counts, event-deck order, and downstream hashes exactly — a single mismatch
   blocks this task and restores the local helper.
5. Enforcement surface: fixed-word event-deck vectors, rejection paths, event-
   deck order, forecast, draw sequence, and public view/effects/replay/export
   hashes; all byte-identical to the 001 baseline.

## Architecture Check

1. Replacing the local sampler with the shipped engine helper removes a
   duplicated rejection-sampling algorithm while preserving exact output.
2. No backwards-compatibility alias — the local helper and its single call path
   are removed (restored only on an identity failure).
3. `engine-core` already owns the unbiased-index contract; no mechanic noun
   added, no `game-stdlib` change. This does not centralize the shuffle.

## Verification Layers

1. RNG identity -> `cargo test -p flood_watch` event-deck shuffle vector tests
   (fixed RNG words, rejection draws, event-deck order identical to baseline).
2. Downstream hash/visibility neutrality -> `replay-check --game flood_watch
   --all` + serialization tests byte-identical to baseline (forecast, public
   view/effects/replay/export hashes unchanged).
3. Draw-count identity -> rejection draw counts equal the baseline.

## What to Change

### 1. Adopt `next_index_unbiased_v1`

In `games/flood_watch/src/setup.rs`, replace the local
`next_bounded_index_unbiased` call inside `shuffle_event_deck` with
`rng.next_index_unbiased_v1(...)`. Remove the now-unused local helper. Preserve
seed handling, RNG consumption, rejection behavior, and event-deck order.

## Files to Touch

- `games/flood_watch/src/setup.rs` (modify; serialized after 304)

## Out of Scope

- Centralizing the event-deck shuffle/forecast or any game rule.
- Changing seeds, word consumption, rejection behavior, or loop bounds.
- Introducing a new RNG algorithm.

## Acceptance Criteria

### Tests That Must Pass

1. `cargo test -p flood_watch` (event-deck shuffle vector + draw-count tests).
2. `cargo run -p replay-check -- --game flood_watch --all` — byte-identical to baseline.
3. `cargo run -p fixture-check -- --game flood_watch`.

### Invariants

1. RNG words, rejection counts, event-deck/forecast order, and all downstream
   hashes are identical to baseline.
2. On any identity mismatch, the local helper is restored (task blocked).

## Test Plan

### New/Modified Tests

1. `None — behavior-neutral adoption; the existing event-deck shuffle vector tests, replay/serialization suites, and the 001 baseline are the regression guard (a mismatch blocks the task).`

### Commands

1. `cargo test -p flood_watch`
2. `cargo run -p replay-check -- --game flood_watch --all`
3. A per-game test + replay-check is the correct boundary: RNG identity and
   downstream hashes are the surfaces this migration must preserve.

## Outcome

- Replaced Flood Watch's local `next_bounded_index_unbiased` call inside
  `shuffle_event_deck` with `DeterministicRng::next_index_unbiased_v1` and
  removed the now-unused local helper.
- Retargeted the existing setup rejection-vector test to the shared RNG method,
  preserving high-residue rejection coverage. Event deck, forecast, replay,
  fixture, and export surfaces remained byte-identical under the existing
  gates.
- Verification passed:
  - `cargo test -p flood_watch`
  - `cargo run -p replay-check -- --game flood_watch --all`
  - `cargo run -p fixture-check -- --game flood_watch`
  - `git diff --check`

# UNI8CMECSCA-016: Document legacy `next_index`; add `next_index_unbiased_v1`

**Status**: COMPLETED
**Priority**: MEDIUM
**Effort**: Medium
**Engine Changes**: Yes — `crates/engine-core/src/rng.rs`
**Deps**: UNI8CMECSCA-003

## Problem

`DeterministicRng::next_index` reduces a `u64` modulo the bound, which is biased; the unbiased rejection-sampling variant is currently re-implemented locally in three games. This ticket documents `next_index` as legacy modulo semantics (kept byte-identical) and adds an explicitly named `next_index_unbiased_v1` whose accepted-zone/rejection behavior matches the existing local implementation (C-09). Deterministic vectors pin both returned indices and consumed random words, including at least one rejected draw. The legacy method's implementation and consumption are unchanged.

## Assumption Reassessment (2026-06-22)

1. `crates/engine-core/src/rng.rs` defines `trait DeterministicRng` with `fn next_u64(&mut self) -> u64` and `fn next_index(&mut self, upper_bound: usize) -> Option<usize>` implemented as `(next_u64() % upper_bound) ...` (modulo); no `next_index_unbiased*` exists today (confirmed by grep at the reassessed commit).
2. The unbiased algorithm to standardize is the existing `next_bounded_index_unbiased` in `games/river_ledger/src/setup.rs:159` (identical copies in `games/plain_tricks/src/setup.rs:101`, `games/poker_lite/src/setup.rs:70`): `None` for zero; `accepted_zone = range - (range % upper)` with `range = u64::MAX as u128 + 1`; redraw `next_u64()` while `>= accepted_zone`; return `(value % upper)`. Register entry `MSC-8C-009` homes the sampler primitive in `engine-core`.
3. Cross-artifact boundary under audit: the deterministic RNG contract (`crates/engine-core/src/rng.rs`, `docs/ARCHITECTURE.md`/`docs/TESTING-REPLAY-BENCHMARKING.md` determinism law). The new method is additive; `next_index` is untouched.
4. FOUNDATIONS §2/§11 determinism: both methods are deterministic for a fixed stream; the unbiased method's extra `next_u64` draws on rejection are characterized, not hidden. Shuffle/deal policy stays local (no shared shuffle helper — spec §9.12).
5. Deterministic replay/hash + RNG-consumption surface under audit (§11/§13/EC-12/EC-13): `next_index` returns the same values and consumes the same words as before (forbidden to mutate in place); `next_index_unbiased_v1` pins returned index and `next_u64` count for zero / power-of-two / non-power-of-two / large bounds and at least one rejected draw. No hidden information is involved.

## Architecture Check

1. The random-word→index mapping is noun-free kernel infrastructure; standardizing the already-repeated unbiased method in `engine-core` removes three local copies without touching deal/shuffle policy.
2. No backwards-compatibility shim — `next_index` keeps its exact semantics (renamed in docs only, not in code behavior); the unbiased method is a new explicit name.
3. `engine-core` gains only a bounded-index sampler (allowed deterministic-RNG surface); no mechanic noun.

## Verification Layers

1. `next_index` returns identical values and consumes identical words as before → existing RNG vectors unchanged (EV-RNG-LEGACY).
2. `next_index_unbiased_v1` matches the characterized local algorithm, including a rejected draw → deterministic vectors (EV-RNG-V1).
3. Zero / power-of-two / non-power-of-two / large-bound cases covered → unit tests.
4. Rustdoc states legacy modulo vs unbiased-v1 consumption consequences → grep-proof.

## What to Change

### 1. `crates/engine-core/src/rng.rs`

Add rustdoc to `next_index` naming it legacy modulo semantics (no code change). Add `fn next_index_unbiased_v1(&mut self, upper_bound: usize) -> Option<usize>` with the characterized accepted-zone/rejection algorithm and consumption docs.

### 2. Tests

Deterministic vectors pinning returned indices and `next_u64` counts for both methods, including rejection cases.

## Files to Touch

- `crates/engine-core/src/rng.rs` (modify)

## Out of Scope

- Migrating any game to the unbiased method (River is UNI8CMECSCA-017; no other game migrates in 8C).
- Any change to `next_index`'s behavior or consumption.
- A shared shuffle/deal/permutation helper.

## Acceptance Criteria

### Tests That Must Pass

1. `cargo test -p engine-core` passes, including legacy and unbiased vectors.
2. `next_index_unbiased_v1` vectors include at least one rejected draw and pin `next_u64` consumption.
3. `bash scripts/boundary-check.sh` and `cargo build --workspace` pass.

### Invariants

1. `next_index` returns the same values and consumes the same words as before.
2. The unbiased method matches the existing local algorithm bit-for-bit on selected seeds/bounds.

## Test Plan

### New/Modified Tests

1. `crates/engine-core/src/rng.rs` (inline `#[cfg(test)]`) — legacy `next_index` vectors (unchanged) + `next_index_unbiased_v1` vectors with rejection.

### Commands

1. `cargo test -p engine-core`
2. `bash scripts/boundary-check.sh`
3. The engine-core suite is the correct boundary — River adopts the method in UNI8CMECSCA-017.

## Outcome

Completed: 2026-06-22

What changed:
- Documented `DeterministicRng::next_index` as the legacy modulo bounded-index
  sampler that consumes exactly one `next_u64` word for every nonzero bound.
- Added `DeterministicRng::next_index_unbiased_v1`, using the characterized
  accepted-zone rejection-sampling algorithm with `u128` range arithmetic.
- Added engine-core vector tests for legacy modulo draw counts, unbiased zero
  bounds, power-of-two and non-power-of-two accepted draws, a rejected
  `u64::MAX` draw for bound 3, and equivalence with the existing local
  rejection-sampling algorithm.
- Flipped `MSC-8C-009` in `docs/MECHANICAL-SCAFFOLDING-REGISTER.md` from
  `candidate` to `accepted` for the sampler primitive.

Deviations:
- None. No game caller was migrated and `next_index` behavior/consumption stayed
  unchanged.

Verification:
- `cargo fmt --all --check`
- `cargo test -p engine-core`
- `bash scripts/boundary-check.sh`
- `cargo build --workspace`
- `rg -n 'legacy modulo|next_index_unbiased_v1|accepted zone|Rejections consume|one \`next_u64\` word' crates/engine-core/src/rng.rs`

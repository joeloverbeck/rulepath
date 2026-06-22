# UNI8CMECSCA-017: River Ledger adopts `next_index_unbiased_v1`

**Status**: COMPLETED
**Priority**: MEDIUM
**Effort**: Medium
**Engine Changes**: Yes — `games/river_ledger/src/setup.rs`
**Deps**: UNI8CMECSCA-016

## Problem

River Ledger has a local `next_bounded_index_unbiased` helper byte-equivalent to the new kernel `next_index_unbiased_v1` (UNI8CMECSCA-016). Replace River's local helper with the kernel method — River is the only 8C RNG adoption — proving byte-for-byte identical deal/setup/replay/visibility output and equal `next_u64` consumption across all selected seeds and supported seat counts. Local shuffle/deal order stays in River; no other game migrates.

## Assumption Reassessment (2026-06-22)

1. `games/river_ledger/src/setup.rs` defines and calls `next_bounded_index_unbiased` (line 159; called at line 147 for the shuffle) with the exact accepted-zone/rejection algorithm now standardized as `DeterministicRng::next_index_unbiased_v1` (UNI8CMECSCA-016). The UNI8CMECSCA-003 packet pins River's RNG output and `next_u64` draw counts.
2. Spec §5 8C-017 + A-09 fix the boundary: River is the only 8C RNG adoption because its local sampler is semantically and byte-consumption equivalent; Briar/Vow modulo callers remain for a future explicit migration. Byte-identical deal/setup/replay/visibility for all selected seeds and counts; equal `next_u64` consumption demonstrated; local shuffle/deal order remains in River.
3. Cross-artifact boundary under audit: River's `setup.rs` shuffle path and the kernel RNG method, plus River's golden traces/fixtures/visibility suite. The extraction removes the local copy; it does not change the call order.
4. FOUNDATIONS §2/§11 determinism: identical inputs+versions produce identical output; the adoption is a pure refactor of the sampler call, not a behavior change.
5. Deterministic replay/hash + RNG-consumption surface under audit (§11/EC-13): byte/trace identity to the former local implementation is the acceptance bar; the no-leak/visibility suite must stay green (the deal feeds seat-private hands). Shuffle/deal policy is not moved to shared code.

## Architecture Check

1. Replacing the duplicated local sampler with the registered kernel method (`MSC-8C-009`) removes one of three copies while preserving exact consumption — the narrowest behavior-preserving change.
2. No backwards-compatibility shim — the local helper is deleted, not aliased; River calls the kernel method directly.
3. `engine-core` untouched (method already landed); River's shuffle/deal order stays local (§9.12 no shared shuffle helper).

## Verification Layers

1. River deal/setup/replay output byte-identical to baseline across counts 3–6 → `cargo run -p replay-check -- --game river_ledger --all`, `cargo run -p fixture-check -- --game river_ledger`.
2. Equal `next_u64` consumption to the former local helper → River RNG equivalence test (EV-RNG-V1).
3. Seat-private no-leak/visibility unchanged → `cargo test -p river_ledger` (visibility suite).
4. Local `next_bounded_index_unbiased` removed → grep-proof it no longer exists in `games/river_ledger/src/setup.rs`.

## What to Change

### 1. `games/river_ledger/src/setup.rs`

Remove the local `next_bounded_index_unbiased` and call `rng.next_index_unbiased_v1(..)` at the existing shuffle site, preserving the exact call order and bounds.

### 2. Tests

River local-vs-shared equivalence vectors (returned indices + `next_u64` count) for selected seeds.

## Files to Touch

- `games/river_ledger/src/setup.rs` (modify)

## Out of Scope

- Migrating Briar/Vow or any other modulo caller (future explicit migration).
- Moving River's shuffle/deal order into shared code.
- Any change to River's deal sequence or seat-private projection.

## Acceptance Criteria

### Tests That Must Pass

1. `cargo run -p replay-check -- --game river_ledger --all` and `cargo run -p fixture-check -- --game river_ledger` pass with unchanged hashes for counts 3–6.
2. `cargo test -p river_ledger` passes (deal/setup + visibility), with an equivalence test pinning identical `next_u64` consumption.
3. `cargo test --workspace` passes.

### Invariants

1. River deal/setup/replay/visibility output is byte-identical to the former local implementation.
2. No local unbiased sampler remains in River; shuffle/deal order is unchanged.

## Test Plan

### New/Modified Tests

1. `games/river_ledger/src/setup.rs` (inline `#[cfg(test)]`) — local-vs-shared equivalence (index + consumption) for selected seeds/counts.

### Commands

1. `cargo run -p replay-check -- --game river_ledger --all && cargo run -p fixture-check -- --game river_ledger`
2. `cargo test -p river_ledger`
3. `replay-check`/`fixture-check` plus the equivalence test are the correct boundary — byte/consumption identity is the acceptance bar.

## Outcome

Completed: 2026-06-22

What changed:
- Replaced River Ledger's shuffle call to the local unbiased bounded-index
  helper with `rng.next_index_unbiased_v1(index + 1)`.
- Removed River Ledger's local `next_bounded_index_unbiased` helper.
- Updated River setup tests to pin the shared method's rejected-draw behavior,
  zero-bound draw count, and returned-index/draw-count equivalence against a
  test-local copy of the removed algorithm for selected bounds.
- Updated `MSC-8C-009` acceptance evidence with River's local-vs-shared
  equivalence and no-artifact-drift proof.

Deviations:
- The ticket listed only `games/river_ledger/src/setup.rs`; updating
  `docs/MECHANICAL-SCAFFOLDING-REGISTER.md` was needed to record the completed
  River adoption evidence.
- No other game RNG caller, shuffle order, deal order, fixture, golden trace,
  setup bytes, replay hash, or visibility output was changed.

Verification:
- `cargo fmt --all --check`
- `cargo run -p replay-check -- --game river_ledger --all`
- `cargo run -p fixture-check -- --game river_ledger`
- `cargo test -p river_ledger`
- `git diff --quiet -- games/river_ledger/tests/golden_traces games/river_ledger/data`
- `cargo test --workspace`
- `rg -n "next_bounded_index_unbiased" games/river_ledger/src/setup.rs` returned no matches.

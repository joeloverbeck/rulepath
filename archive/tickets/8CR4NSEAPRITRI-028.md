# 8CR4NSEAPRITRI-028: River Ledger C-08 side-pot domain-evidence profile driver

**Status**: COMPLETED
**Priority**: MEDIUM
**Effort**: Small
**Engine Changes**: Yes (dev-only profile adapter) — `games/river_ledger/tests/`; allocation/explanation owned by River
**Deps**: 8CR4NSEAPRITRI-001

## Problem

River's all-in/side-pot allocation evidence is not yet validated through the shipped `domain-evidence-v1` driver (MSC-8C, C-08). Add a virtual `domain-evidence-v1` adapter around selected game-owned side-pot allocation evidence, delegating allocation/explanation legality to River with `canonical_byte_authority = none` unless an existing validator owns bytes (spec §3.9 River domain, §5.9, §3.9 minimum selections).

## Assumption Reassessment (2026-06-24)

1. `crates/game-test-support/src/profiles.rs::DomainEvidenceV1Driver` exists; the River traces `three-way-main-two-side-pots`, `uncalled-return`, and the per-pot remainder-order evidence exist. Confirmed during `/reassess-spec`.
2. Spec §3.9 classifies River `domain-evidence-v1` as `migrate` with `canonical_byte_authority = none` unless the existing validator owns exact bytes; minimum selections are three-way-main-two-side-pots, uncalled-return, and per-pot-remainder-button-order.
3. Cross-artifact: the domain-evidence contract is owned by `game-test-support`; River owns the allocator/explanation. Baseline evidence comes from `-001`.
4. §5/§11 motivate this ticket: the profile metadata is test/evidence scaffolding only — it encodes no selector/formula and delegates allocation/explanation validation to River Rust.
5. Enforcement surface = `DomainEvidenceV1Driver` virtual metadata over the selected allocation evidence; no canonical byte authority is declared and no fixture is rewritten.

## Architecture Check

1. Virtual domain metadata delegating to the game allocator is cleaner than re-implementing allocation checks in the driver — the driver validates metadata, River owns side-pot semantics.
2. No backwards-compatibility shim is introduced; no fixture/trace byte changes. Rollback removes only the metadata adapter.
3. `engine-core` stays noun-free (§3); `game-test-support` is dev-only and encodes no behavior (§4/§5).

## Verification Layers

1. Driver accepts correct metadata, rejects wrong profile/version/field set -> schema/serialization validation via `DomainEvidenceV1Driver`.
2. Allocation + explanation assertions delegated to River -> rule test over the game allocator (game-owned).
3. No canonical byte authority claimed / no fixture rewrite -> codebase grep-proof (`canonical_byte_authority = none`; fixture bytes unchanged).

## What to Change

### 1. Add the `domain-evidence-v1` profile adapter

In `games/river_ledger/tests/` (fixture/replay/rule modules), add a `DomainEvidenceV1Driver` virtual-metadata adapter over the three-way-main-two-side-pots, uncalled-return, and per-pot-remainder-button-order evidence, with game-owned allocator/explanation assertions and `canonical_byte_authority = none`.

## Files to Touch

- `games/river_ledger/tests/serialization.rs` (modify; or the fixture/rule test module that owns allocation evidence)

## Out of Scope

- The replay-command/public/seat-private export profiles (`-025`/`-026`/`-027`).
- Declaring a new canonical byte authority or rewriting any fixture/trace.
- Any allocation, remainder-order, or showdown policy change.

## Acceptance Criteria

### Tests That Must Pass

1. `cargo test -p river_ledger` is green, including the `domain-evidence-v1` driver + game-owned allocator/explanation assertions.
2. `cargo run -p fixture-check -- --game river_ledger` and `cargo run -p replay-check -- --game river_ledger --all` pass with bytes unchanged.
3. `bash scripts/boundary-check.sh` passes.

### Invariants

1. The driver validates metadata only; allocation/explanation legality stays in River; `canonical_byte_authority = none`.
2. No fixture/trace byte changes.

## Test Plan

### New/Modified Tests

1. `games/river_ledger/tests/serialization.rs` (or the allocation-evidence module) — `domain-evidence-v1` virtual metadata + game-owned allocation/explanation assertions.

### Commands

1. `cargo test -p river_ledger`
2. `cargo run -p fixture-check -- --game river_ledger`
3. The per-game fixture/rule test is the correct boundary: domain evidence delegates to the game allocator.

## Outcome

Completed: 2026-06-24

What changed:

1. Added a virtual `domain-evidence-v1` profile adapter test for River side-pot evidence using `DomainEvidenceV1Driver`.
2. Delegated side-pot, uncalled-return, and per-pot remainder assertions to River-owned state/pot logic while checking selected evidence fixtures remain profile-metadata-free.
3. Added reject coverage for wrong validator owner, wrong profile version, and illegal canonical-byte claim with `canonical_byte_authority = none`.

Deviations: None.

Verification:

1. `cargo fmt --all --check` - passed.
2. `cargo test -p river_ledger` - passed.
3. `cargo run -p fixture-check -- --game river_ledger` - passed.
4. `cargo run -p replay-check -- --game river_ledger --all` - passed.
5. `bash scripts/boundary-check.sh` - passed.

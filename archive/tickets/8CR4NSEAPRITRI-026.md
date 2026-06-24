# 8CR4NSEAPRITRI-026: River Ledger C-08 public-export profile driver

**Status**: COMPLETED
**Priority**: MEDIUM
**Effort**: Small
**Engine Changes**: Yes (dev-only profile adapter) — `games/river_ledger/tests/`; export JSON/hash authority unchanged
**Deps**: 8CR4NSEAPRITRI-001, 8CR4NSEAPRITRI-020

## Problem

River's observer public export (including a multipot terminal path) is not yet validated through the shipped `public-export-v1` driver with round-trip and no-leak evidence (MSC-8C, C-08). Add a `public-export-v1` adapter over `export_public_replay`, leaving existing export JSON/hash authority unchanged (spec §3.9 River public-export, §5.9).

## Assumption Reassessment (2026-06-24)

1. `crates/game-test-support/src/profiles.rs::PublicExportV1Driver` exists; `games/river_ledger/src/replay_support.rs::export_public_replay` and the `public-replay-export-import.trace.json` plus a multipot terminal export exist. Confirmed during `/reassess-spec`.
2. Spec §3.9 classifies River `public-export-v1` as `migrate`; this ticket `Deps` `-020` so the runout/multipot no-leak geometry exists before the export profile closeout (spec §11.3: viewer geometry precedes export profile).
3. Cross-artifact: the public-export evidence contract is owned by `game-test-support`; River's export JSON/hash stays authority. Baseline export bytes come from `-001`.
4. §11 no-leak firewall motivates this ticket: the observer public export round-trips and carries no hidden hole/board card, including the multipot terminal path.
5. Enforcement surface = `PublicExportV1Driver` round-trip + no-leak over the observer export; no export encoding flip and no byte rewrite.

## Architecture Check

1. A thin public-export profile adapter is cleaner than ad-hoc round-trip asserts — it routes evidence through the owned driver while River owns export bytes.
2. No backwards-compatibility shim is introduced; no export byte changes. Rollback removes only the driver adapter.
3. `engine-core` stays noun-free (§3); `game-test-support` is dev-only (§4/§5).

## Verification Layers

1. Observer export round-trips and rejects wrong metadata -> schema/serialization validation via `PublicExportV1Driver`.
2. No hidden card in the public export incl. multipot terminal path -> no-leak visibility test over the observer export.
3. Export JSON/hash unchanged -> golden trace byte check (`replay-check --game river_ledger --all` byte-identical to baseline).

## What to Change

### 1. Add the `public-export-v1` profile adapter

In `games/river_ledger/tests/replay.rs`, add a `PublicExportV1Driver` adapter over `export_public_replay` providing observer round-trip + no-leak evidence including a multipot terminal export, delegating export meaning to River.

## Files to Touch

- `games/river_ledger/tests/replay.rs` (modify)

## Out of Scope

- The replay-command (`-025`), seat-private export (`-027`), and domain (`-028`) profiles.
- Any export encoding flip or byte rewrite.
- Any reveal/allocation policy change.

## Acceptance Criteria

### Tests That Must Pass

1. `cargo test -p river_ledger` is green, including the `public-export-v1` round-trip + no-leak test.
2. `cargo run -p replay-check -- --game river_ledger --all` passes with export bytes unchanged.
3. `bash scripts/boundary-check.sh` passes.

### Invariants

1. The observer public export round-trips with no hidden card; export JSON/hash is unchanged.
2. The driver validates metadata only and delegates to River.

## Test Plan

### New/Modified Tests

1. `games/river_ledger/tests/replay.rs` — `public-export-v1` observer round-trip + no-leak over the export incl. multipot terminal path.

### Commands

1. `cargo test -p river_ledger`
2. `cargo run -p replay-check -- --game river_ledger --all`
3. The per-game replay test is the correct boundary: the profile is a dev-only evidence adapter over the existing export.

## Outcome

Completed: 2026-06-24

What changed:

1. Added a virtual `public-export-v1` profile adapter test over River observer exports using `PublicExportV1Driver`.
2. Covered observer export round-trip and hidden-card absence for a normal export path and a multipot path while leaving existing export JSON/hash authority unchanged.
3. Added reject coverage for wrong validator owner, wrong visibility, and unknown profile fields.

Deviations: None.

Verification:

1. `cargo fmt --all --check` - passed.
2. `cargo test -p river_ledger` - passed.
3. `cargo run -p replay-check -- --game river_ledger --all` - passed.
4. `bash scripts/boundary-check.sh` - passed.

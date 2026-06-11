# GAT11MASCLABLU-012: Benchmarks, thresholds, and BENCHMARKS.md

**Status**: ✅ COMPLETED
**Priority**: MEDIUM
**Effort**: Medium
**Engine Changes**: Yes — new `games/masked_claims/benches/{masked_claims.rs,thresholds.json}`, new `games/masked_claims/docs/BENCHMARKS.md`; modifies `games/masked_claims/Cargo.toml` (`[[bench]]` table)
**Deps**: GAT11MASCLABLU-010

## Problem

The official-game contract requires benchmarks with threshold floors and a `BENCHMARKS.md` doc. Masked Claims must bench the claim/reaction legal-action generation, validate/apply for claim and both responses, challenge resolution, view projection (pending and post-reveal), replay/hash, public export, and both Level 1 bot decisions, with non-heroic smoke floors and a named calibration follow-up.

## Assumption Reassessment (2026-06-10)

1. The pipeline (GAT11MASCLABLU-004–009) provides the operations to bench. The `[[bench]]` table is added to `games/masked_claims/Cargo.toml` here (deferred from GAT11MASCLABLU-003 to avoid a dangling bench path); the shape is `name = "masked_claims"`, `path = "benches/masked_claims.rs"`, `harness = false`, modeled on `games/plain_tricks/Cargo.toml` (confirmed).
2. Spec §"Benchmark operations" lists the twelve identities (`legal_actions_claim_phase`, `legal_actions_reaction_window`, `validate_claim`, `apply_claim_open_window`, `apply_accept_resolution`, `apply_challenge_resolve_reveal`, `project_public_view_pending_reaction`, `project_public_view_after_reveal`, `state_hash_terminal`, `public_export_timeline`, `level1_bot_claim_decision`, `level1_bot_response_decision`). Thresholds start as smoke floors with a calibration follow-up under ADR 0002/0003/0005 (all confirmed present under `docs/adr/`). `BENCHMARKS.md` instantiates from `templates/GAME-BENCHMARKS.md`.
3. Cross-artifact boundary under audit: `BENCHMARKS.md` is consumed by `tools/rule-coverage` (validates rules/coverage/benchmarks docs) and `thresholds.json` by `tools/bench-report`. Co-locating the doc with the benches here means the `rule-coverage` registration ticket (GAT11MASCLABLU-015) depends on this ticket for a fully-green coverage report.
4. ADR 0002 (CI benchmark gating lanes), ADR 0003 (calibrated thresholds), and ADR 0005 (variance-aware floors) are the principles under audit — smoke floors now, PR smoke non-gating, calibration named in the handoff.

## Architecture Check

1. Co-locating `BENCHMARKS.md` with the benches (the tool-validated-docs rule) keeps the doc and the threshold file in one reviewable diff and avoids a validator reading a doc that doesn't yet exist.
2. No backwards-compatibility aliasing or shims introduced.
3. `engine-core` stays noun-free; benches are game-local.

## Verification Layers

1. The twelve-identity benchmark list runs -> `cargo bench -p masked_claims`.
2. Threshold smoke floors enforced where calibrated -> `tools/bench-report` (registered in GAT11MASCLABLU-015).
3. `BENCHMARKS.md` consistent with the bench identities -> `tools/rule-coverage` (registered in GAT11MASCLABLU-015) + manual review.

## What to Change

### 1. `games/masked_claims/benches/masked_claims.rs`

Criterion benches for the twelve identity operations above.

### 2. `games/masked_claims/benches/thresholds.json`

Non-heroic smoke floors for the benched operations.

### 3. `games/masked_claims/Cargo.toml`

Add the `[[bench]]` table (`name`, `path`, `harness = false`).

### 4. `games/masked_claims/docs/BENCHMARKS.md`

Instantiate from `templates/GAME-BENCHMARKS.md`; document the identity list, the smoke-floor posture, and the named calibration follow-up under ADR 0002/0003/0005.

## Files to Touch

- `games/masked_claims/benches/masked_claims.rs` (new)
- `games/masked_claims/benches/thresholds.json` (new)
- `games/masked_claims/Cargo.toml` (modify)
- `games/masked_claims/docs/BENCHMARKS.md` (new)

## Out of Scope

- `bench-report` / `rule-coverage` tool registration (GAT11MASCLABLU-015).
- Threshold calibration tuning (named follow-up, not this gate's blocker).

## Acceptance Criteria

### Tests That Must Pass

1. `cargo bench -p masked_claims` runs the twelve-identity benchmark list.
2. `games/masked_claims/Cargo.toml` `[[bench]]` resolves to `benches/masked_claims.rs`.
3. `BENCHMARKS.md` enumerates the benched operations and the calibration follow-up.

### Invariants

1. Benchmarks are smoke floors now with calibration named (ADR 0002/0003/0005); PR smoke stays non-gating.
2. `engine-core` gains no mechanic noun.

## Test Plan

### New/Modified Tests

1. `games/masked_claims/benches/masked_claims.rs` — the twelve benchmark identities.
2. `games/masked_claims/benches/thresholds.json` — smoke floors.

### Commands

1. `cargo bench -p masked_claims -- --warm-up-time 1 --measurement-time 1` (smoke run).
2. `cargo build -p masked_claims --benches`
3. A smoke bench run is the correct boundary; `bench-report` threshold enforcement is wired in GAT11MASCLABLU-015.

## Outcome

Completed: 2026-06-11

What changed:

- Added `games/masked_claims/benches/masked_claims.rs` with the twelve required benchmark identities and native JSON report output.
- Added `games/masked_claims/benches/thresholds.json` with non-gating smoke floors for every benchmarked operation.
- Registered the bench target in `games/masked_claims/Cargo.toml`.
- Added `games/masked_claims/docs/BENCHMARKS.md` documenting the operations, smoke-floor posture, and ADR 0002/0003/0005 calibration follow-up.

Deviations from original plan:

- None. Threshold enforcement by `bench-report` remains for GAT11MASCLABLU-015 as scoped.

Verification:

- `cargo bench -p masked_claims -- --warm-up-time 1 --measurement-time 1` passed and ran all twelve operations.
- `cargo build -p masked_claims --benches` passed.
- `cargo test -p masked_claims` passed.
- `cargo clippy -p masked_claims --all-targets -- -D warnings` passed.
- `cargo fmt --all --check` passed.

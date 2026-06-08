# GAT91SECDRACOM-011: secret_draft benchmarks + thresholds + BENCHMARKS.md + gate-2 CI

**Status**: ✅ COMPLETED
**Priority**: MEDIUM
**Effort**: Medium
**Engine Changes**: Yes — `games/secret_draft/benches/secret_draft.rs`, `benches/thresholds.json`, `games/secret_draft/docs/BENCHMARKS.md`; `.github/workflows/gate-2-benchmarks.yml` (modify). No `engine-core` / `game-stdlib` change.
**Deps**: GAT91SECDRACOM-007, GAT91SECDRACOM-008

## Problem

The official-game contract requires native benchmarks with a threshold file and a `BENCHMARKS.md`, plus a gate-2 CI bench-smoke + threshold-report step. Initial thresholds are non-heroic smoke floors with a named calibration follow-up, per ADR 0002/0003/0005.

## Assumption Reassessment (2026-06-08)

1. `games/token_bazaar/benches/{token_bazaar.rs,thresholds.json}` and `games/token_bazaar/docs/BENCHMARKS.md` are the precedent (verified). `.github/workflows/gate-2-benchmarks.yml` registers token_bazaar via a `bench smoke` step (`cargo bench -p token_bazaar -- legal_actions`, lines 44–45) and a full bench + `bench-report --thresholds games/token_bazaar/benches/thresholds.json` step (lines 90–91); `secret_draft` follows the same two-step shape.
2. The benchable operations (GAT91SECDRACOM-004/005/006/007/008) are inputs. Spec §"Benchmark operations" + §Deliverables (Benchmarks row) name the identities: `legal_actions_initial_pool`, `legal_actions_after_one_commit`, `validate_commit`, `apply_first_commit`, `apply_second_commit_resolve_reveal`, `project_public_view_pending`, `project_public_view_after_reveal`, `state_hash_terminal`, `public_export_timeline`, `level1_bot_decision`.
3. Cross-artifact boundary under audit: the `thresholds.json` schema consumed by `tools/bench-report` and the gate-2 workflow. `BENCHMARKS.md` is a tool-validated doc co-located with the benches (per decomposition §Tool-validated docs); it documents the bench identities + threshold methodology.
4. ADR 0002/0003/0005 are the governing decisions: restate before trusting spec — thresholds start as variance-aware smoke floors (non-gating PR smoke), with stable floors calibrated after enough runs; the implementation handoff names the calibration follow-up. Do not optimize until evidence points to a real issue.
5. Schema extension: `gate-2-benchmarks.yml` is extended additively (new steps for a new game); `thresholds.json` is a new per-game file, not a shared-schema change.

## Architecture Check

1. Smoke-floor thresholds with a named calibration follow-up are cleaner than guessed tight thresholds: they avoid flaky CI while still catching gross regressions, per the accepted benchmark ADRs.
2. No backwards-compatibility aliasing/shims.
3. `engine-core` stays noun-free; benches are game-local. No `game-stdlib` helper.

## Verification Layers

1. Benches run -> `cargo bench -p secret_draft -- <identity>` executes each named operation.
2. Threshold report -> `cargo run -p bench-report -- --input <report> --thresholds games/secret_draft/benches/thresholds.json` passes at smoke floors.
3. CI registration -> gate-2 workflow has the bench-smoke + threshold steps for `secret_draft` (grep-proof).
4. Doc-link integrity -> `node scripts/check-doc-links.mjs` with `BENCHMARKS.md` present.

## What to Change

### 1. `games/secret_draft/benches/secret_draft.rs`

Criterion benches for all ten named identities, exercising legal-action generation, validate/apply, reveal resolution, project-view (pending + after-reveal), state hash, public export, and Level 1 bot decision.

### 2. `games/secret_draft/benches/thresholds.json`

Smoke-floor thresholds for each bench identity (variance-aware, non-heroic) per ADR 0003/0005.

### 3. `games/secret_draft/docs/BENCHMARKS.md`

Instantiate from `templates/GAME-BENCHMARKS.md`: bench identities, threshold methodology, and the named calibration follow-up.

### 4. `.github/workflows/gate-2-benchmarks.yml`

Add a `secret_draft bench smoke` step (`cargo bench -p secret_draft -- legal_actions`) and a full bench + `bench-report --thresholds games/secret_draft/benches/thresholds.json` step, mirroring the token_bazaar steps.

## Files to Touch

- `games/secret_draft/benches/secret_draft.rs` (new)
- `games/secret_draft/benches/thresholds.json` (new)
- `games/secret_draft/docs/BENCHMARKS.md` (new)
- `.github/workflows/gate-2-benchmarks.yml` (modify)

## Out of Scope

- Threshold calibration to tight floors (named follow-up; smoke floors only here).
- gate-1 CI / tool registration (GAT91SECDRACOM-012/016).
- Other per-game docs (GAT91SECDRACOM-017).

## Acceptance Criteria

### Tests That Must Pass

1. `cargo bench -p secret_draft -- legal_actions` runs (and each named identity executes).
2. `cargo run -p bench-report -- --input <report> --thresholds games/secret_draft/benches/thresholds.json` passes at smoke floors.
3. `node scripts/check-doc-links.mjs` passes with `BENCHMARKS.md`.

### Invariants

1. Thresholds are smoke floors with a named calibration follow-up; PR smoke is non-gating (ADR 0002/0003/0005).
2. Benches add no `engine-core` noun and no `game-stdlib` helper (§3/§4).

## Test Plan

### New/Modified Tests

1. `games/secret_draft/benches/secret_draft.rs` — the ten named bench identities.

### Commands

1. `cargo bench -p secret_draft -- legal_actions`
2. `cargo bench -p secret_draft | tee /tmp/secret_draft-bench.txt && cargo run -p bench-report -- --input /tmp/secret_draft-bench.txt --thresholds games/secret_draft/benches/thresholds.json`
3. A bench-smoke filter is the correct boundary; full calibration is a named post-gate follow-up, not a blocking CI gate.

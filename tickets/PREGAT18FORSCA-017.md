# PREGAT18FORSCA-017: ci/scaffolding-audits.json receipt + 17-game legacy bootstrap

**Status**: PENDING
**Priority**: MEDIUM
**Effort**: Medium
**Engine Changes**: Yes (CI evidence receipt) — `ci/scaffolding-audits.json` (new)
**Deps**: PREGAT18FORSCA-007

## Problem

The Gate 1 governance checker needs a finite, reviewed evidence receipt to validate against `ci/games.json` and the register. This ticket adds `ci/scaffolding-audits.json` (`schema_version: 1`), bootstrapped so the frozen 17-game corpus uses `coverage: "legacy-8c-covered"` pointers to the existing 8C/R1–R4 evidence — a static-data receipt with no selectors/formulas/triggers and unknown-field rejection by default.

## Assumption Reassessment (2026-06-25)

1. `ci/games.json` lists exactly 17 games (`race_to_n` … `vow_tide`), each an object with `id` / `sim_flags` / `e2e`. `ci/` currently holds only `games.json`; `ci/scaffolding-audits.json` is absent (new). Verified this session.
2. The `legacy-8c-covered` pointers resolve to real evidence: `archive/specs/unit-8c-*`, `archive/specs/8c-r1..r4-*`, and the register's existing `MSC-8C-001` / `UNI8CMECSCA-005/006` receipt entries (`docs/MECHANICAL-SCAFFOLDING-REGISTER.md`). Verified this session.
3. The spec (D18, plan §6.3, §6.4) requires: `schema_version: 1`; the frozen 17-game `legacy-8c-covered` bootstrap; the legacy set frozen so no post-8F game can claim it; future games must use `coverage: "forward-v1"`; no selectors/formulas/triggers; unknown fields rejected.
4. FOUNDATIONS §5 (static data is not behavior) and §11 (unknown fields rejected; behavior-looking fields blocked) under audit: the receipt is finite reviewed metadata — it selects no behavior and is not loaded by any game/WASM/browser path. Reassess A6: if it trends toward behavior, escalate before landing; the unknown-field-rejection + no-selectors design keeps it static.
5. Enforcement surface named: the receipt is the static data the Gate 1 checker (PREGAT18FORSCA-018) validates (set-equality vs `ci/games.json`, path/ID resolution). It introduces no hidden-information leak and no nondeterminism — it is a sorted, reviewed JSON record; every byte/hash/visibility migration field says `none` or cites ADR 0009.

## Architecture Check

1. A static JSON receipt that rejects unknown fields and selects no behavior is the minimal tractable evidence surface — it avoids a runtime registry or DSL (which §5/§12 forbid).
2. No backwards-compatibility shim — `schema_version: 1` is the initial version; the legacy set is frozen by construction.
3. `engine-core` is untouched; the receipt is CI evidence metadata, not loaded by any game/WASM path — `boundary-check.sh` still gates the kernel.

## Verification Layers

1. Schema validity → `node -e "JSON.parse(require('fs').readFileSync('ci/scaffolding-audits.json','utf8'))"` parses; `schema_version` is `1`.
2. Set-equality → the receipt's game-id set equals `ci/games.json`'s 17 ids (manual + later checker).
3. Pointer resolution → every `legacy-8c-covered` pointer resolves to a committed evidence path/register ID (grep-proof).
4. Static-data discipline (§5/§11) → grep-proof the receipt carries no selectors/conditions/triggers/formulas and every migration field says `none` or cites ADR 0009.

## What to Change

### 1. ci/scaffolding-audits.json

Author the receipt (`schema_version: 1`) with one entry per the 17 `ci/games.json` games, each `coverage: "legacy-8c-covered"` pointing to its 8C/R-wave evidence; the legacy set frozen; future-game entries reserved for `coverage: "forward-v1"`; behavior-free fields only; unknown fields rejected by the consuming checker.

## Files to Touch

- `ci/scaffolding-audits.json` (new)

## Out of Scope

- The checker that reads the receipt (PREGAT18FORSCA-018) and its tests (PREGAT18FORSCA-019).
- Adding any `forward-v1` entry for a future game (Gate 18 is not pre-audited here).
- Any selector/condition/trigger/DSL or runtime registry.

## Acceptance Criteria

### Tests That Must Pass

1. `node -e "JSON.parse(require('fs').readFileSync('ci/scaffolding-audits.json','utf8'))"` succeeds; `schema_version` is `1`.
2. The receipt's id set equals the 17 ids in `ci/games.json`.
3. `git diff --check` clean; `bash scripts/boundary-check.sh` passes (receipt not loaded by any kernel/WASM path).

### Invariants

1. The receipt is static, finite, reviewed metadata with no selectors/conditions/triggers.
2. The 17-game legacy set is frozen; future games must use `forward-v1`.

## Test Plan

### New/Modified Tests

1. `None — static evidence-receipt data; no test file authored here. It is validated by `node -e` JSON-parse at this landing and exercised by the checker + fixture suite in PREGAT18FORSCA-018/019.`

### Commands

1. `node -e "const a=JSON.parse(require('fs').readFileSync('ci/scaffolding-audits.json','utf8')); if(a.schema_version!==1) throw new Error('bad version')"`
2. `node scripts/check-ci-games.mjs` (the 17-game source of truth the receipt mirrors)
3. `bash scripts/boundary-check.sh`

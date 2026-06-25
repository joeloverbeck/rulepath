# PREGAT18FORSCA-018: scripts/check-scaffolding-governance.mjs validator

**Status**: COMPLETED
**Priority**: HIGH
**Effort**: Large
**Engine Changes**: Yes (tooling/audit) — `scripts/check-scaffolding-governance.mjs` (new)
**Deps**: PREGAT18FORSCA-002, PREGAT18FORSCA-007, PREGAT18FORSCA-017

## Problem

The forward obligation needs a deterministic, fail-closed CI guard. This ticket implements `scripts/check-scaffolding-governance.mjs`: it validates `ci/scaffolding-audits.json` (schema, set-equality vs `ci/games.json` + real `games/` dirs, path/ID resolution, register-freshness, prior-game-scheduling, migration-authority) and runs a high-confidence known-promoted-shape fingerprint layer, with a compact success summary and no env/label/comment bypass.

## Assumption Reassessment (2026-06-25)

1. The repo runs Node CI checks of the form `node scripts/check-*.mjs` (e.g. `scripts/check-ci-games.mjs`, `scripts/check-doc-links.mjs`, `scripts/check-catalog-docs.mjs`), all present. Node is `v24.16.0`. `scripts/check-scaffolding-governance.mjs` is absent (new). Verified this session.
2. The receipt `ci/scaffolding-audits.json` (PREGAT18FORSCA-017) and the register forward cadence (PREGAT18FORSCA-007) define the data + decision-state vocabulary this checker validates; `ci/games.json` (17 games) is the game-set source of truth.
3. The spec (D19, plan §6.5, §6.6, §6.9) requires: schema / set-equality vs `ci/games.json` + real `games/` dirs / path-ID / register-freshness / prior-game-scheduling / migration-authority / known-shape fingerprint checks; compact success summary; no env/label bypass.
4. FOUNDATIONS §11 (validation is fail-closed and blocking; unknown fields rejected; behavior-looking fields blocked) under audit: the checker must exit non-zero on any violation, reject unknown receipt fields by default, and have no override path; warnings vs blockers are distinguished, with all governance failures blocking.
5. Enforcement surface: this IS the §11 fail-closed validation surface. It reads only static evidence (the receipt, `ci/games.json`, register markdown, `games/` dir listing) — it touches no game behavior, no hidden state, and is deterministic (no wall-clock, no network); the fingerprint layer excludes Non-Promotion List behavior from automatic classification to avoid false positives.

## Architecture Check

1. A single repository-wide Node checker (run once in Gate 1, conceptually adjacent to `boundary-check.sh` / `check-doc-links.mjs` / `check-catalog-docs.mjs`) is the tractable design the spec mandates — it proves the obligation ran without attempting generic clone detection or "CI decides architecture" (explicitly rejected).
2. No backwards-compatibility shim — no bypass flag, env var, branch label, or comment directive; a failing check is non-overridable in v1/v2.
3. `engine-core` is untouched; the checker is a dev/CI tool reading static evidence, not loaded by any game/WASM path.

## Verification Layers

1. Fail-closed behavior (§11) → fixture run: a missing game / stale path / unknown id / unqueued prior site / invalid exception / forbidden legacy claim each makes the checker exit non-zero (covered by PREGAT18FORSCA-019).
2. Green on real repo → `node scripts/check-scaffolding-governance.mjs` exits 0 against the committed receipt (17-game legacy bootstrap).
3. No-bypass invariant → grep-proof the script reads no env var / branch label / comment directive as an override.
4. Determinism → the checker uses no wall-clock/network input; identical repo state yields identical output.

## What to Change

### 1. Receipt + set-equality + path/ID validation

Parse `ci/scaffolding-audits.json` (reject unknown fields); assert its id set equals `ci/games.json` and the real `games/` directories; resolve every declared source/evidence path and register ID against committed files.

### 2. Register-freshness + prior-game-scheduling + migration-authority

Validate register-freshness (every cited register ID exists), prior-game-scheduling (a declared prior match has a named tracker unit or a valid no-unit disposition), and migration-authority (every byte/hash/visibility field says `none` or cites ADR 0009).

### 3. Known-shape fingerprint layer + summary + no-bypass

Add high-confidence promoted-shape fingerprints (excluding Non-Promotion List behavior); emit a compact success summary; ensure no env/label/comment bypass.

## Files to Touch

- `scripts/check-scaffolding-governance.mjs` (new)

## Out of Scope

- The Node test suite + fixtures (PREGAT18FORSCA-019).
- Wiring the step into Gate 1 (PREGAT18FORSCA-020).
- Generic clone detection / AST-wide semantic-equivalence inference (explicitly rejected).

## Acceptance Criteria

### Tests That Must Pass

1. `node scripts/check-scaffolding-governance.mjs` exits 0 against the committed repo (receipt + 17-game bootstrap).
2. The checker exits non-zero when fed a receipt with an unknown field, a missing game, or an unresolved path/ID (exercised by PREGAT18FORSCA-019).
3. `grep -nE "process.env|--force|allowlist|skip" scripts/check-scaffolding-governance.mjs` shows no override/bypass path.

### Invariants

1. The check is deterministic, fail-closed, blocking, and non-overridable.
2. Unknown receipt fields are rejected; Non-Promotion List behavior is excluded from automatic fingerprint classification.

## Test Plan

### New/Modified Tests

1. `None — checker logic only; its Node test suite + synthetic fixtures land in PREGAT18FORSCA-019. Verification at this landing is the green run against the committed repo.`

### Commands

1. `node scripts/check-scaffolding-governance.mjs` (green against the real repo)
2. `node scripts/check-ci-games.mjs` (confirms the 17-game set the checker validates against)
3. `grep -nE "process.env|branch|label|comment" scripts/check-scaffolding-governance.mjs` (no-bypass audit)

## Outcome

Completed. Added `scripts/check-scaffolding-governance.mjs`, a deterministic
repository-wide checker for `ci/scaffolding-audits.json`. The checker rejects
unknown fields, verifies schema version 1, freezes `legacy_8c_games` to the
Unit 8C historical set, enforces set equality with `ci/games.json` and real
`games/` directories, resolves evidence paths and MSC IDs, validates
disposition/register requirements, prior-game scheduling, migration authority,
behavior-looking receipt terms, and a narrow known-signal source scan for normal
`game-test-support` dependency edges. It prints a compact success summary and
has no environment, branch, label, comment, force, allowlist, or skip override.

Verification:

- `node scripts/check-scaffolding-governance.mjs`
- `node scripts/check-ci-games.mjs`
- `grep -nE "process.env|--force|allowlist|skip" scripts/check-scaffolding-governance.mjs` returned no matches.
- `grep -nE "process.env|branch|label|comment" scripts/check-scaffolding-governance.mjs` returned no matches.
- `git diff --check`

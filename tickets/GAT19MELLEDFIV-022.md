# GAT19MELLEDFIV-022: forward-v1 governance closeout — scaffolding receipt, mechanic atlas, register, and primitive-pressure ledger

**Status**: PENDING
**Priority**: HIGH
**Effort**: Medium
**Engine Changes**: Yes (CI evidence receipt) — `ci/scaffolding-audits.json`; `docs/MECHANIC-ATLAS.md`, `docs/MECHANICAL-SCAFFOLDING-REGISTER.md`, `games/meldfall_ledger/docs/PRIMITIVE-PRESSURE-LEDGER.md`
**Deps**: GAT19MELLEDFIV-002, GAT19MELLEDFIV-006, GAT19MELLEDFIV-007, GAT19MELLEDFIV-008, GAT19MELLEDFIV-009, GAT19MELLEDFIV-011

## Problem

The forward-v1 audit authored pre-code (GAT19MELLEDFIV-002) needs its post-build governance receipt and reconciliation: the `ci/scaffolding-audits.json` `forward-v1` entry (validated by `check-scaffolding-governance.mjs`), the `PRIMITIVE-PRESSURE-LEDGER.md` first-use entries (`ML-PP-001`…`ML-PP-006`), the `MECHANIC-ATLAS.md` first-use local-only rows + §10B note (and §10A kept empty), and the `MECHANICAL-SCAFFOLDING-REGISTER.md` `no-new-scaffolding` / no-prior-game-retrofit record.

## Assumption Reassessment (2026-06-25)

1. `scripts/check-scaffolding-governance.mjs` validates `ci/scaffolding-audits.json` (confirmed during reassessment: `coverage` ∈ {legacy-8c-covered, forward-v1}; `disposition` includes `no-new-scaffolding`; signal decisions ∈ {legacy-reviewed, reused, exception, not-present}; the `blackglass_pact` entry uses `forward-v1` + `no-new-scaffolding`). `MECHANIC-ATLAS.md` §10A is empty and §10B exists.
2. Spec §4.4 (intended receipt JSON, `disposition: "no-new-scaffolding"`), §8.2 (first-use posture), §10 (atlas/register/ledger updates), Appendix C (audit matrix), and Appendix D (`ML-PP-001`…`ML-PP-006`) define the content. The first-use entries were emitted by the behavior tickets (006/007/008/009/011).
3. Cross-artifact: the scaffolding-governance lane (`ci/scaffolding-audits.json` + register) and the mechanic atlas are the boundaries; this ticket reconciles them with the as-built game without smuggling behavior into the scaffolding lane.
4. FOUNDATIONS §4/§11: every first-use shape stays `local-only` (no promotion); the register records `no-new-scaffolding`; §10A stays empty (no open promotion debt). MSC-8C-010 keeps meld/lay-off/pickup/scoring/bot/visibility game-owned.
5. FOUNDATIONS §11 / §12 enforcement surface: the `forward-v1` CI receipt is the universal-acceptance-invariant artifact; its absence is a §12 stop condition. `check-scaffolding-governance.mjs` is the fail-closed validator.

## Architecture Check

1. Reconciling the receipt + atlas + register + ledger in one closeout keeps the governance evidence consistent with the shipped code and gives a single `check-scaffolding-governance` green diff.
2. No backwards-compatibility shims.
3. `engine-core` untouched; no `game-stdlib` promotion; the scaffolding register gains no behavior.

## Verification Layers

1. `ci/scaffolding-audits.json` `meldfall_ledger` `forward-v1` entry is schema-valid -> `node scripts/check-scaffolding-governance.mjs`.
2. `PRIMITIVE-PRESSURE-LEDGER.md` carries `ML-PP-001`…`ML-PP-006`; atlas first-use rows + §10B note present, §10A empty -> grep + `node scripts/check-doc-links.mjs`.
3. No first-use shape promoted; no behavior in the register -> FOUNDATIONS §4/§11 alignment check.

## What to Change

### 1. `ci/scaffolding-audits.json`

Add the `meldfall_ledger` `forward-v1` entry per spec §4.4 (`disposition: "no-new-scaffolding"`, `register_entries_reviewed` MSC-8C-001…010, `known_signal_dispositions`, `compatibility` all `none`, `prior_matching_games: []`, `follow_on_unit: null`).

### 2. `PRIMITIVE-PRESSURE-LEDGER.md` + `MECHANIC-ATLAS.md`

Author `ML-PP-001`…`ML-PP-006` (meld validation; public tableau/zone; draw/discard multi-card pickup; lay-off-any; cumulative-500 scoring; deterministic-shuffle/private-hand §10B-reviewed). Add the atlas first-use local-only rows + the §10B note (deterministic-shuffle/private-hand with redacted exports, no staged reveal, no trigger fires); keep §10A empty.

### 3. `MECHANICAL-SCAFFOLDING-REGISTER.md`

Record the Gate 19 `forward-v1` reuse-first audit: `no-new-scaffolding`, no prior-game retrofit.

## Files to Touch

- `ci/scaffolding-audits.json` (modify)
- `docs/MECHANIC-ATLAS.md` (modify)
- `docs/MECHANICAL-SCAFFOLDING-REGISTER.md` (modify)
- `games/meldfall_ledger/docs/PRIMITIVE-PRESSURE-LEDGER.md` (new)

## Out of Scope

- Trailing game docs + the `specs/README.md` `Done`-flip (GAT19MELLEDFIV-023).
- Any `game-stdlib` promotion (none earned).

## Acceptance Criteria

### Tests That Must Pass

1. `node scripts/check-scaffolding-governance.mjs` passes with the new `meldfall_ledger` `forward-v1` receipt.
2. `PRIMITIVE-PRESSURE-LEDGER.md` carries `ML-PP-001`…`ML-PP-006`; atlas §10A is empty and the §10B note is present.
3. `node scripts/check-doc-links.mjs` passes.

### Invariants

1. Every first-use shape is `local-only`; no `game-stdlib` promotion; §10A has no open debt (FOUNDATIONS §4/§11).
2. The scaffolding register carries no meld/lay-off/pickup/scoring/bot/visibility behavior (MSC-8C-010).

## Test Plan

### New/Modified Tests

1. `ci/scaffolding-audits.json` — the `forward-v1` receipt (validated by `check-scaffolding-governance.mjs`).
2. `games/meldfall_ledger/docs/PRIMITIVE-PRESSURE-LEDGER.md` — `ML-PP-001`…`ML-PP-006`.

### Commands

1. `node scripts/check-scaffolding-governance.mjs`
2. `node scripts/check-doc-links.mjs`
3. `for id in 001 002 003 004 005 006; do grep -q "ML-PP-$id" games/meldfall_ledger/docs/PRIMITIVE-PRESSURE-LEDGER.md || echo "MISSING ML-PP-$id"; done`

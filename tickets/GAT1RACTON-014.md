# GAT1RACTON-014: Docs finalize — RULE-COVERAGE close, AI.md, mechanic-atlas confirm

**Status**: PENDING
**Priority**: MEDIUM
**Effort**: Small
**Engine Changes**: Yes (docs) — finalize `games/race_to_n/docs/RULE-COVERAGE.md` and `AI.md`; confirm the `docs/MECHANIC-ATLAS.md` `race_to_n` row stays `local-only`.
**Deps**: GAT1RACTON-006, GAT1RACTON-007, GAT1RACTON-008

## Problem

Per-game docs must have no silent gaps once behavior exists (FOUNDATIONS §6;
OFFICIAL-GAME-CONTRACT §6/§7). This ticket closes `RULE-COVERAGE.md` (no `open`
rows), finalizes `AI.md` from the implemented bot, and confirms the mechanic-atlas
`race_to_n` row stays `local-only` (first use; no extraction). It is the
documentation-completeness deliverable of WB8, separate from CI wiring (013) and
the exit-criteria capstone (015).

## Assumption Reassessment (2026-06-05)

1. `games/race_to_n/docs/RULE-COVERAGE.md` and `AI.md` exist in draft form
   (RULE-COVERAGE skeleton from GAT1RACTON-001; AI.md draft from GAT1RACTON-007).
   This ticket finalizes them against implemented behavior + tests (005/006/007/
   008). `MECHANICS.md`/`RULES.md`/`SOURCES.md` were authored in 001;
   `BENCHMARKS.md`/`UI.md` in 010/012.
2. `docs/MECHANIC-ATLAS.md:166` carries `tiny numeric turn race | race_to_n |
   local-only | Keep local; proves plumbing only. | None.` (verified). The spec
   §9 directs confirming/keeping this row as `local-only` (first use) — no edit
   expected unless the implemented mechanic diverged from the inventory.
3. Cross-artifact boundary under audit: `RULE-COVERAGE.md` rows must reference the
   stable `RULES.md` IDs (001) and the rule tests that close them (005/008); each
   `covered`/`covered-by-trace` row must map to a real test/trace. `AI.md`
   describes the implemented Level 0 bot (007).
4. FOUNDATIONS §6 (no demo-shell: docs complete) and §4 (`game-stdlib` earned;
   first use stays local-only) motivate this ticket. The atlas confirmation
   asserts no unearned promotion (MECHANIC-ATLAS §11 stage-advancement check).
5. Third-use / mechanic-pressure surface: this ticket touches the §4 primitive-
   pressure gate by confirming `race_to_n` is first-use `local-only` — no ledger
   entry required (the hard gate is the *third* use). Confirm `game-stdlib` stays
   empty and no helper was promoted (FOUNDATIONS §4; MECHANIC-ATLAS §11). No
   replay/hash/leak surface touched (docs-only).
6. Schema/contract: this edits markdown docs only. The atlas row is a typed table
   row; confirming it `local-only` is a no-op edit unless divergence is found
   (then update the row + note why).

## Architecture Check

1. Closing rule coverage and finalizing bot docs after implementation (not before)
   matches the requirements-first workflow's tail (OGC §3) and ensures docs match
   code (OGC §12). Doing this in one docs-finalize ticket (vs scattering across
   impl tickets) gives one atomic "docs are complete" review surface.
2. No backwards-compatibility shims — doc content updates.
3. `engine-core`/`game-stdlib` untouched; the atlas confirmation explicitly keeps
   `game-stdlib` empty (§4).

## Verification Layers

1. No open coverage rows -> codebase grep-proof (`grep -n 'open' RULE-COVERAGE.md`
   returns no status-`open` rows; OGC §6 no silent gaps).
2. Coverage rows map to tests -> manual review (each `covered`/`covered-by-trace`
   row names a real rule test / golden trace from 005/008).
3. Atlas row correctness -> codebase grep-proof (`docs/MECHANIC-ATLAS.md`
   `race_to_n` row reads `local-only`; `game-stdlib` empty).
4. Bot docs match implementation -> manual review (`AI.md` describes the Level 0
   random legal bot as built in 007).

## What to Change

### 1. Close `games/race_to_n/docs/RULE-COVERAGE.md`

Set each row to its final status (`covered` / `covered-by-trace` /
`not-applicable` / `intentionally-deferred` with gate) referencing the rule tests
(005) and golden traces (008). No `open` rows.

### 2. Finalize `games/race_to_n/docs/AI.md`

Document the implemented Level 0 random legal bot: policy (uniform random over
legal paths), determinism, legality, limits (AI-BOTS; OGC §9).

### 3. Confirm `docs/MECHANIC-ATLAS.md` row

Verify the `tiny numeric turn race` → `race_to_n` row stays `local-only`; update
only if the implemented mechanic diverged from `MECHANICS.md` (with a note).

## Files to Touch

- `games/race_to_n/docs/RULE-COVERAGE.md` (modify)
- `games/race_to_n/docs/AI.md` (modify)
- `docs/MECHANIC-ATLAS.md` (modify) — confirm row; edit only if divergent

## Out of Scope

- CI wiring (GAT1RACTON-013).
- The `specs/README.md` index flip + exit-criteria verification (GAT1RACTON-015).
- Authoring SOURCES/RULES/MECHANICS/BENCHMARKS/UI (001/010/012).
- Any `game-stdlib` promotion (forbidden — first use is local-only, §4).

## Acceptance Criteria

### Tests That Must Pass

1. `grep -nE '\bopen\b' games/race_to_n/docs/RULE-COVERAGE.md` — no status-`open` rows remain.
2. `node scripts/check-doc-links.mjs` — doc links resolve.
3. Manual review: every `RULE-COVERAGE.md` row maps to a real rule test or golden trace; `AI.md` matches the implemented bot.

### Invariants

1. Rule coverage has no silent gaps and no `open` rows (OGC §6; FOUNDATIONS §6).
2. `race_to_n` mechanic stays `local-only`; `game-stdlib` remains empty (FOUNDATIONS §4).

## Test Plan

### New/Modified Tests

1. `None — documentation-only ticket; verification is grep/doc-link commands plus the rule tests/golden traces (005/008) named in Assumption Reassessment.`

### Commands

1. `grep -nE '\bopen\b' games/race_to_n/docs/RULE-COVERAGE.md`
2. `node scripts/check-doc-links.mjs`
3. `grep -n 'race_to_n' docs/MECHANIC-ATLAS.md` — confirm the row reads `local-only` (atlas-confirmation boundary).

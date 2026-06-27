# GAT20STACROSTA-001: Starbridge Crossing rules + IP/source docs

**Status**: COMPLETED
**Priority**: HIGH
**Effort**: Medium
**Engine Changes**: None — game-local docs (`games/starbridge_crossing/docs/RULES.md`, `SOURCES.md`) + `docs/SOURCES.md` index row
**Deps**: none

## Problem

Gate 20 admits **Starbridge Crossing** (Star Halma / Chinese Checkers family) as a public, perfect-information board game. Per `docs/OFFICIAL-GAME-CONTRACT.md` §3, original rules prose and source/IP notes precede implementation, and `docs/FOUNDATIONS.md` §10 requires neutral naming, original prose, and a source/IP receipt before any public file ships. This ticket front-loads those docs so every later ticket builds against a pinned rules contract.

## Assumption Reassessment (2026-06-27)

1. Sibling games author short-named docs `RULES.md` / `SOURCES.md` (not `GAME-`-prefixed): confirmed `games/meldfall_ledger/docs/RULES.md` and `games/vow_tide/docs/SOURCES.md`; the `GAME-` prefix is templates-only. The reassessed spec (§4) already uses short names.
2. `docs/SOURCES.md` carries a per-game index table (`| Game | Source note | Status |`) linking each game's own `SOURCES.md`; the global row is added here because this ticket creates the game `SOURCES.md` it points at.
3. Cross-artifact boundary: the rules prose pinned here is the contract `tools/rule-coverage` (GAT20STACROSTA-013), `HOW-TO-PLAY.md` (014), and the outcome-explanation surfaces (015/018) all consume; the variant id `starbridge_crossing_classic_star_v1`, rules version `starbridge-crossing-rules-v1`, seat set `{2,3,4,6}`, 10 pegs, and stop-anywhere hop chain are pinned from spec §1/Appendix A.
4. §10 IP conservatism motivates this ticket: "Starbridge Crossing" is an original coinage; `Chinese Checkers`/`Star Halma`/`Stern-Halma`/`Halma` stay rules-family/source-history labels in `SOURCES.md`, never the product title. No rulebook prose, diagrams, board/peg art, or trade dress is copied; sources are summarized in original prose with consulted dates.

## Architecture Check

1. Front-loading rules/source prose gives every downstream ticket a stable, citable contract instead of re-deriving rules per ticket; it matches the OGC §3 ordering used by every prior game gate.
2. No backwards-compatibility shims; these are new docs.
3. Docs only — no `engine-core` mechanic noun, no `game-stdlib` change. Behavior remains to be implemented in Rust by later tickets; this ticket asserts no legality.

## Verification Layers

1. Neutral-naming / IP invariant (§10) -> manual review: product title is "Starbridge Crossing"; family labels appear only as source-history references.
2. Rules-contract completeness -> manual review against spec §3 Scope + Appendix A (step, hop, hop-chain, cycle guard, blocked pass, finish, turn-limit all stated).
3. Cross-artifact index integrity -> `node scripts/check-doc-links.mjs` (the new `docs/SOURCES.md` row links a file this ticket creates).

## What to Change

### 1. Author `games/starbridge_crossing/docs/RULES.md`

Original Rulepath rules prose pinning: 121-space six-pointed star; seats `{2,3,4,6}` (default 2); 10 pegs per seat in its home point; one move per turn = one step OR one hop chain (never mixed); hop over one adjacent occupied space into the empty space beyond, jumped peg stays; chain may change direction and stop after any hop; no landing revisited within a turn; forced `pass_blocked` when no legal move; finish = all 10 pegs in opposite home; continuing finish-order ranks; `turn_limit` fallback. Include stable scoring/terminal rule IDs for the outcome-explanation surface.

### 2. Author `games/starbridge_crossing/docs/SOURCES.md`

Summarize the spec Appendix A sources (`[SRC-WIKI]`, `[SRC-ACM]`, `[SRC-BELL]`, `[SRC-ENV]`, `[SRC-HEX]`, accessibility refs) in original prose with consulted dates (2026-06-27); record naming/IP rationale and the Stern-Halma/Halma lineage as source history only.

### 3. Add the `docs/SOURCES.md` index row

Add `| \`starbridge_crossing\` | \`games/starbridge_crossing/docs/SOURCES.md\` | completed for Gate 20; … |`.

## Files to Touch

- `games/starbridge_crossing/docs/RULES.md` (new)
- `games/starbridge_crossing/docs/SOURCES.md` (new)
- `docs/SOURCES.md` (modify)

## Out of Scope

- Any Rust code, crate skeleton, or data files (later tickets).
- `HOW-TO-PLAY.md` / `MECHANICS.md` / `UI.md` / `AI.md` (later doc tickets).
- Partnership variant, square Halma, 15-peg variant — recorded as out-of-scope/future only.

## Acceptance Criteria

### Tests That Must Pass

1. `node scripts/check-doc-links.mjs` — the new `docs/SOURCES.md` link resolves.
2. `grep -L "Chinese Checkers" games/starbridge_crossing/docs/RULES.md` — family label is not used as the product name in rules prose (manual confirm title is "Starbridge Crossing").
3. `test -f games/starbridge_crossing/docs/RULES.md && test -f games/starbridge_crossing/docs/SOURCES.md`

### Invariants

1. Rules prose is original; no copied rulebook text or diagrams (§10).
2. Rules contract matches spec §3 + Appendix A; later tickets cite these IDs without contradiction.

## Test Plan

### New/Modified Tests

1. `None — documentation-only ticket; verification is command-based and existing pipeline/test coverage is named in Assumption Reassessment.`

### Commands

1. `node scripts/check-doc-links.mjs`
2. `node scripts/check-doc-links.mjs && bash scripts/boundary-check.sh`
3. Narrower command is correct: this ticket ships only markdown; doc-link integrity + boundary noun check are the relevant gates, no cargo build is affected.

## Outcome

Completed: 2026-06-27

What changed:

- Added `games/starbridge_crossing/docs/RULES.md` with original Rulepath rules
  prose, stable `SC-*` rule IDs, supported seat set `{2,3,4,6}`, 121-space
  topology contract, step/hop-chain rules, blocked-pass behavior, finish-order
  standings, deterministic turn-limit fallback, all-public visibility, replay,
  bot, UI, diagnostics, ambiguity, and out-of-scope variant sections.
- Added `games/starbridge_crossing/docs/SOURCES.md` with consulted source IDs,
  consulted dates, variant pinning, naming/IP rationale, ambiguity log, asset
  provenance, human-release-review posture, and rule-source-to-rule-ID mapping.
- Added the `starbridge_crossing` row to `docs/SOURCES.md`, scoped as completed
  for Gate 20 source/IP intake rather than full Gate 20 completion.

Deviations from plan:

- Kept the detailed `Chinese Checkers` family label in `SOURCES.md` and the
  repo-level source index, but omitted that phrase from `RULES.md` so the
  ticket's `grep -L "Chinese Checkers" games/starbridge_crossing/docs/RULES.md`
  acceptance check proves the product-name guard.

Verification:

- `node scripts/check-doc-links.mjs` passed (`Checked 31 markdown files`).
- `bash scripts/boundary-check.sh` passed (`engine-core boundary check passed`;
  `game-test-support dev-only boundary check passed`).
- `grep -L "Chinese Checkers" games/starbridge_crossing/docs/RULES.md` printed
  `games/starbridge_crossing/docs/RULES.md`, confirming the phrase is absent
  from `RULES.md`.
- `test -f games/starbridge_crossing/docs/RULES.md && test -f games/starbridge_crossing/docs/SOURCES.md` passed.

Unrelated worktree changes left untouched:

- `.claude/skills/spec-to-tickets/SKILL.md`

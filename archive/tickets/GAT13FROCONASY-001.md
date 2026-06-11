# GAT13FROCONASY-001: Rules and IP source docs for Frontier Control

**Status**: COMPLETED
**Priority**: HIGH
**Effort**: Medium
**Engine Changes**: None — documentation only (`games/frontier_control/docs/RULES.md`, `games/frontier_control/docs/SOURCES.md`)
**Deps**: None

## Problem

`docs/OFFICIAL-GAME-CONTRACT.md` §3 requires original rules prose to precede implementation. Frontier Control needs an authoritative `RULES.md` — carrying stable rule IDs for scoring, terminal, and the Garrison tiebreak that the outcome-explanation contract (`scripts/check-outcome-explanations.mjs`) and `tools/rule-coverage` later bind to — plus a `SOURCES.md` recording the consulted-mechanics-only posture and the trade-dress avoidance register (spec §Implementation reference IP-risk register). Authoring these first anchors house terminology (Garrison/Prospectors, sites, edges, guards, crews, stakes, forts, clash, supply) before any code commits to it.

## Assumption Reassessment (2026-06-11)

1. `templates/GAME-RULES.md` and `templates/GAME-SOURCES.md` exist and are the instantiation source; `games/flood_watch/docs/RULES.md` and `SOURCES.md` are the freshest exemplars for voice/structure (verified present).
2. The spec (`specs/gate-13-frontier-control-asymmetric-area-control-proof.md`) §Proposed original rules and §IP-risk register supply the rules content and the named hazards (Root foremost; `clearing`/woodland theming and `Warden` avoided per A2); display/faction/site names are A2 placeholders maintainers may rename before implementation.
3. Cross-artifact boundary under audit: the **stable rule-ID set** in `RULES.md` (`## Scoring and accounting`, `## Terminal conditions`) is the contract surface `scripts/check-outcome-explanations.mjs` and `tools/rule-coverage` consume; rule IDs authored here must match the outcome-template `ruleRefLabel` mirrors registered in GAT13FROCONASY-015/016 and the rule-coverage map in GAT13FROCONASY-013.
4. FOUNDATIONS §10 IP conservatism motivates this ticket: original rules prose, original component IDs, no rulebook prose or trade dress copied from Root/Risk/El Grande/etc.; `SOURCES.md` records the rules-vs-expression boundary (U.S. Copyright Office games circular and *DaVinci Editrice v. ZiKo Games*, mirrored from Gate 11/12 specs).

## Architecture Check

1. Front-loading rules prose (OGC §3) means every downstream ticket cites one authoritative terminology + rule-ID source instead of inventing names mid-implementation; the alternative (deriving rules from code) inverts the contract and risks unstated rules.
2. No backwards-compatibility aliasing/shims — these are new files for a new game.
3. No `engine-core` or `game-stdlib` surface touched; docs are game-local under `games/frontier_control/docs/`.

## Verification Layers

1. Original-prose / IP-conservatism invariant -> manual review (IP-conservatism audit against the spec's named hazards; no copied prose, names, or trade dress).
2. Rule-ID contract invariant -> codebase grep-proof (every scoring/terminal/tiebreak rule ID present under the required `RULES.md` headings, to be consumed by `check-outcome-explanations.mjs` and `rule-coverage`).
3. Doc-link integrity -> `node scripts/check-doc-links.mjs` (links from RULES/SOURCES resolve).

## What to Change

### 1. Author `games/frontier_control/docs/RULES.md`

Instantiate from `templates/GAME-RULES.md`. Cover: components (seats, factions, seven sites, ten edges, guards/crews/stakes/forts, stake values), setup (deterministic from variant), turn flow (two-action budget, disjoint faction action sets, `end_turn` no-stall), asymmetric clash resolution, round scoring (Garrison fort-holding; Prospector supply-connected stakes), terminal (eight rounds, higher score, Garrison tiebreak). Assign stable rule IDs under `## Scoring and accounting` and `## Terminal conditions` (e.g. `FC-SCORE-GARRISON-FORT`, `FC-SCORE-PROSPECTOR-SUPPLY`, `FC-TERM-SCORE-COMPARE`, `FC-TERM-GARRISON-TIEBREAK`).

### 2. Author `games/frontier_control/docs/SOURCES.md`

Instantiate from `templates/GAME-SOURCES.md`. Record: no external research pass ran (A10); the consulted-mechanics-only posture; the trade-dress avoidance register (Root/Risk/El Grande/Small World/Kemet/Blood Rage/AGoT/Twilight Struggle named hazards); why every name/label is original; asset/font status; the rules-vs-expression citation.

## Files to Touch

- `games/frontier_control/docs/RULES.md` (new)
- `games/frontier_control/docs/SOURCES.md` (new)

## Out of Scope

- Any Rust code, crate skeleton, or static data (GAT13FROCONASY-003+).
- The remaining eleven per-game docs (their own tickets).
- Registering rule-ID mirrors in TypeScript outcome templates (GAT13FROCONASY-015/016).

## Acceptance Criteria

### Tests That Must Pass

1. `node scripts/check-doc-links.mjs` passes with the two new docs in place.
2. `grep -E 'FC-(SCORE|TERM)' games/frontier_control/docs/RULES.md` returns the scoring/terminal/tiebreak rule IDs.
3. Manual IP-conservatism review confirms no copied prose/names/trade dress and original placeholders only.

### Invariants

1. Rules prose and component names are original (§10); no named-hazard vocabulary appears as a public-facing label.
2. Every rule ID later consumed by the outcome-explanation contract and rule-coverage exists under the required `RULES.md` headings.

## Test Plan

### New/Modified Tests

1. `None — documentation-only ticket; verification is command-based and existing pipeline/test coverage is named in Assumption Reassessment.`

### Commands

1. `node scripts/check-doc-links.mjs`
2. `grep -nE 'FC-(SCORE|TERM)' games/frontier_control/docs/RULES.md`
3. Doc-link + grep are the correct boundary: no Rust exists yet, so rule-coverage/outcome-explanation enforcement is deferred to the tickets that author those consumers.

## Outcome

Completed: 2026-06-11

What changed:
- Added `games/frontier_control/docs/RULES.md` with original Rulepath rules prose for the standard and highlands variants, including stable `FC-SCORE-*` and `FC-TERM-*` rule IDs for later rule coverage and outcome-explanation wiring.
- Added `games/frontier_control/docs/SOURCES.md` with the project-authority source posture, original naming rationale, trade-dress avoidance register, asset/font status, and rules-vs-expression notes.

Deviations from the plan:
- None. This ticket remained documentation-only and did not touch Rust, static data, WASM, or web code.

Verification:
- `node scripts/check-doc-links.mjs` passed (`Checked 25 markdown files`).
- `grep -nE 'FC-(SCORE|TERM)' games/frontier_control/docs/RULES.md` returned the expected scoring, terminal, tiebreak, and outcome-traceability rule IDs.
- Manual IP-conservatism review: the docs use original Rulepath prose and labels, avoid the named area-control/asymmetric-faction hazard vocabulary as public-facing source identity, and introduce no copied prose, assets, screenshots, scans, fonts, or trade dress.

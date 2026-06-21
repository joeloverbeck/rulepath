# GAT16BRICIRTRI-002: Requirements-admission receipt, player rules, and coverage skeleton

**Status**: COMPLETED
**Priority**: MEDIUM
**Effort**: Medium
**Engine Changes**: None (docs — `games/briar_circuit/docs/GAME-IMPLEMENTATION-ADMISSION.md`, `HOW-TO-PLAY.md`, `RULE-COVERAGE.md`)
**Deps**: 001

## Problem

`docs/OFFICIAL-GAME-CONTRACT.md` requires a requirements-first admission receipt before gameplay code, a player-facing original rules summary, and a rule-coverage map keyed to the `BC-*` IDs. This ticket authors those three docs so later implementation tickets and tooling have a coverage contract to fill in. `HOW-TO-PLAY.md` must carry a hidden-information section because Briar Circuit is a hidden-hand game (the `flood_watch`/`event_frontier` precedent for `scripts/check-player-rules.mjs`'s `HIDDEN_INFO_GAMES` requirement).

## Assumption Reassessment (2026-06-20)

1. `games/briar_circuit/docs/RULES.md` and the `BC-*` IDs exist after GAT16BRICIRTRI-001; this ticket maps each ID to its planned proof surface in `RULE-COVERAGE.md` and does not redefine IDs. `templates/GAME-IMPLEMENTATION-ADMISSION.md` and `templates/GAME-HOW-TO-PLAY.md` are the structural templates.
2. `specs/gate-16-briar-circuit-trick-taking.md` §4.3 fixes the doc requirements; §3.1's private-information row and §7.5 no-leak taxonomy fix the hidden-information content for `HOW-TO-PLAY.md`.
3. Cross-artifact boundary under audit: `RULE-COVERAGE.md` is the contract `tools/rule-coverage` validates once the `BC-` prefix is registered (GAT16BRICIRTRI-012); this ticket authors its initial state, finalized after the modules and tests land. `HOW-TO-PLAY.md` is the source `scripts/copy-player-rules.mjs` generates `apps/web/public/rules/briar_circuit.md` from at GAT16BRICIRTRI-013.
4. This ticket builds inputs to two FOUNDATIONS enforcement surfaces a later ticket activates — `tools/rule-coverage` fail-closed coverage (§6 evidence) and `scripts/check-player-rules.mjs` (the hidden-information section gate). The docs introduce no leakage path: `HOW-TO-PLAY.md` describes which facts are private without disclosing any seat's cards; the player-rules generation + `HIDDEN_INFO_GAMES` registration are enforced at GAT16BRICIRTRI-013.

## Architecture Check

1. Authoring the admission receipt and coverage skeleton before code keeps the requirements-first OGC order; the coverage map drives later test/trace tickets rather than being reverse-engineered at closeout.
2. No backwards-compatibility aliasing/shims — new game-local docs only.
3. `engine-core` untouched (§3); no `game-stdlib` change (§4) — docs-only.

## Verification Layers

1. Every `BC-*` ID in `RULES.md` appears in `RULE-COVERAGE.md` -> grep cross-check of the two files.
2. `HOW-TO-PLAY.md` carries the required sections incl. hidden information -> `node scripts/check-player-rules.mjs` dry-run intent (full enforcement at 013) + manual section check.
3. Admission receipt completeness vs template -> manual review against `templates/GAME-IMPLEMENTATION-ADMISSION.md`.

## What to Change

### 1. `games/briar_circuit/docs/GAME-IMPLEMENTATION-ADMISSION.md`

Requirements-first admission receipt: seat declaration (fixed 4), surface budgets (spec §7.7), original-prose stance, source-divergence summary, and the requirements checklist completed before behavior implementation begins.

### 2. `games/briar_circuit/docs/HOW-TO-PLAY.md`

Player-facing original summary matching the Rust behavior contract, including the hidden-information section (private hands, private pass selections/provenance, deck order) required for `HIDDEN_INFO_GAMES`.

### 3. `games/briar_circuit/docs/RULE-COVERAGE.md`

Initial map of every `BC-*` rule ID to its planned unit/rule/property/trace/simulation/replay/serialization/visibility/UI evidence (finalized in GAT16BRICIRTRI-012 once modules and tests exist).

## Files to Touch

- `games/briar_circuit/docs/GAME-IMPLEMENTATION-ADMISSION.md` (new)
- `games/briar_circuit/docs/HOW-TO-PLAY.md` (new)
- `games/briar_circuit/docs/RULE-COVERAGE.md` (new)

## Out of Scope

- Generating `apps/web/public/rules/briar_circuit.md` and `HIDDEN_INFO_GAMES` registration (GAT16BRICIRTRI-013).
- Finalizing coverage proof links and the `BC-` prefix validator (GAT16BRICIRTRI-012).
- Trailing docs MECHANICS/UI/AI/COMPETENT-PLAYER/BOT-STRATEGY-EVIDENCE-PACK/PUBLIC-RELEASE-CHECKLIST (GAT16BRICIRTRI-017).

## Acceptance Criteria

### Tests That Must Pass

1. `comm -23 <(grep -oE 'BC-[A-Z]+-[0-9]+' games/briar_circuit/docs/RULES.md | sort -u) <(grep -oE 'BC-[A-Z]+-[0-9]+' games/briar_circuit/docs/RULE-COVERAGE.md | sort -u)` — empty (every rule ID is mapped).
2. `node scripts/check-doc-links.mjs` — passes.
3. Manual check: `HOW-TO-PLAY.md` hidden-information section names private hands, pass selections/provenance, and deck order.

### Invariants

1. Every authored `BC-*` rule has a coverage row before code claims to satisfy it (§6 evidence-heavy).
2. Player rules describe only public-safe facts; no seat's private holdings are disclosed (§11 no-leak).

## Test Plan

### New/Modified Tests

1. `None — documentation-only ticket; verification is command-based and existing pipeline/test coverage is named in Assumption Reassessment.`

### Commands

1. `comm -23 <(grep -oE 'BC-[A-Z]+-[0-9]+' games/briar_circuit/docs/RULES.md | sort -u) <(grep -oE 'BC-[A-Z]+-[0-9]+' games/briar_circuit/docs/RULE-COVERAGE.md | sort -u)`
2. `node scripts/check-doc-links.mjs`
3. A narrower command is correct here because `tools/rule-coverage` enforcement and player-rules generation require code and the catalog const that land later (012/013).

## Outcome

Completed: 2026-06-21

What changed:

- Added `games/briar_circuit/docs/GAME-IMPLEMENTATION-ADMISSION.md` with the Gate 16 admission receipt, constraints, primitive-pressure blocker, no-leak risk review, UI/bot/benchmark expectations, and explicit decision to admit with constraints.
- Added `games/briar_circuit/docs/HOW-TO-PLAY.md` with original player-facing rules prose, supported seats, pass/play flow, scoring, outcome explanation facts, and hidden-information/reveal timing for private hands, pass selections/provenance, and deck order.
- Added `games/briar_circuit/docs/RULE-COVERAGE.md` as the initial `BC-*` coverage skeleton with planned implementation/evidence surfaces and open status rows for later tickets.

Deviations from plan:

- None. `node scripts/check-player-rules.mjs` was also run as an early guard even though full Briar Circuit catalog enforcement lands later.

Verification:

- `comm -23 <(grep -oE 'BC-[A-Z]+-[0-9]+' games/briar_circuit/docs/RULES.md | sort -u) <(grep -oE 'BC-[A-Z]+-[0-9]+' games/briar_circuit/docs/RULE-COVERAGE.md | sort -u)` produced no output.
- `node scripts/check-doc-links.mjs` passed (`Checked 27 markdown files`).
- `node scripts/check-player-rules.mjs` passed (`player-rules check passed — 15 catalog games validated`).
- Manual hidden-information section check confirmed the player doc names private hands, pass selection/provenance privacy, and deck order/future-deal redaction.

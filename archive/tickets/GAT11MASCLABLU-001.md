# GAT11MASCLABLU-001: Author Masked Claims rules and IP source docs

**Status**: ✅ COMPLETED
**Priority**: HIGH
**Effort**: Medium
**Engine Changes**: None — documentation only (new `games/masked_claims/docs/RULES.md`, `games/masked_claims/docs/SOURCES.md`; no Rust/engine surface)
**Deps**: None

## Problem

Per `docs/OFFICIAL-GAME-CONTRACT.md` §3, original rules prose precedes implementation: the implemented behavior must be anchored to an authored ruleset, not retrofitted afterward. Gate 11 needs `RULES.md` (the canonical Masked Claims rules with stable rule IDs the outcome-explanation contract keys off) and `SOURCES.md` (the IP-conservatism record: consulted mechanics-level prior art, what was and was not used, originality/naming review) in place before the crate skeleton and rules logic land.

## Assumption Reassessment (2026-06-10)

1. `games/masked_claims/` does not yet exist (confirmed absent); this ticket creates `games/masked_claims/docs/`. The structural model is `games/plain_tricks/docs/RULES.md` and `games/plain_tricks/docs/SOURCES.md`, instantiated from `templates/GAME-RULES.md` and `templates/GAME-SOURCES.md` (both confirmed present under `templates/`).
2. The rules content is the spec's Implementation reference §"Proposed original rules: `Masked Claims`" (components, setup, two-phase turn flow, conditional resolution, terminal + tie-break ladder, anti-degeneracy constants); the IP content is the spec's §"Source notes and originality guidance" (seven consulted sources) plus `docs/IP-POLICY.md` (confirmed present).
3. Cross-artifact boundary under audit: `RULES.md` is consumed by `tools/rule-coverage` (maps rules-doc obligations to tests/traces) and by `scripts/check-outcome-explanations.mjs` (reads `docs/RULES.md` per game for rule-ID mirrors). The stable rule-ID set — including scoring and end-state IDs — is the contract those consumers bind to and must be defined here.
4. FOUNDATIONS §10 (IP conservatism) motivates this ticket: all names, grade labels (`Plain`/`Trimmed`/`Gilded`/`Jeweled`/`Master`), prose, and visuals must be original; only mechanics-level prior art (claim/challenge loops, graded penalties, ordered claim spaces) is drawn on. There are no named roles to imitate.

## Architecture Check

1. Front-loading rules prose keeps implementation honest to an authored contract (OGC §3) instead of reverse-documenting code, and gives the rule-ID set a single authoritative origin before any consumer references it.
2. No backwards-compatibility aliasing or shims introduced.
3. `engine-core` is untouched; no `game-stdlib` change. All claim/challenge/mask/grade nouns live in the game's docs.

## Verification Layers

1. Rules-doc obligations map to stable rule IDs -> `tools/rule-coverage` consumes `RULES.md` (verified once the tool is registered in GAT11MASCLABLU-015) + manual review now.
2. Originality / IP conservatism -> manual review against `docs/IP-POLICY.md` (names, labels, prose, asset/font status).
3. Doc-link integrity across the new docs -> `node scripts/check-doc-links.mjs`.
4. Single-artifact-class ticket (docs); the rule-coverage proof in layer 1 is deferred to GAT11MASCLABLU-015 by design (the tool is not yet registered for this game).

## What to Change

### 1. `games/masked_claims/docs/RULES.md`

Instantiate from `templates/GAME-RULES.md`. Cover components (fifteen mask tiles, three per grade 1–5; pedestal; per-seat hand; reserve; veiled/exposed galleries), setup, the eight-turn two-phase flow (claim phase, reaction window, conditional resolution), accept vs honest-challenge vs exposed-lie scoring, terminal after turn 8, and the five-step tie-break ladder. Assign stable rule IDs covering legality, resolution, scoring, and end-state obligations for the outcome-explanation contract.

### 2. `games/masked_claims/docs/SOURCES.md`

Instantiate from `templates/GAME-SOURCES.md`. Record the seven consulted sources (with dates), what each shaped (proof vocabulary, anti-degeneracy analysis, implementation pattern) and what was explicitly not copied, the originality rationale for the name and grade labels, and asset/font status, per `docs/IP-POLICY.md`.

## Files to Touch

- `games/masked_claims/docs/RULES.md` (new)
- `games/masked_claims/docs/SOURCES.md` (new)

## Out of Scope

- Any Rust implementation (`rules.rs`, `setup.rs`, etc. — later tickets).
- The other eleven per-game docs (GAT11MASCLABLU-002/012/013/015/016/020).
- Player-facing `apps/web/public/rules/masked_claims.md` copy, generated from `HOW-TO-PLAY.md` (GAT11MASCLABLU-018).

## Acceptance Criteria

### Tests That Must Pass

1. `node scripts/check-doc-links.mjs` passes with the two new docs.
2. `RULES.md` carries stable rule IDs (including scoring/end IDs) that the outcome-explanation contract can mirror.
3. Manual IP review confirms original names, labels, and prose; `SOURCES.md` records the consulted-mechanics-only posture.

### Invariants

1. All rules prose, names, and labels are original Rulepath text (FOUNDATIONS §10).
2. Rule IDs are stable identifiers usable by `rule-coverage` and `check-outcome-explanations`.

## Test Plan

### New/Modified Tests

1. `games/masked_claims/docs/RULES.md` — canonical rules + stable rule IDs.
2. `games/masked_claims/docs/SOURCES.md` — IP/originality record.

### Commands

1. `node scripts/check-doc-links.mjs`
2. `cargo run -p rule-coverage -- --game masked_claims` — deferred verification boundary: passes only after the tool is registered (GAT11MASCLABLU-015) and rules-doc obligations have backing tests/traces; named here as the eventual proof surface for layer 1.
3. Narrower command is correct now because no Rust/test pipeline exists for this game yet; doc-link integrity is the runnable boundary at this stage.

## Outcome

Completed: 2026-06-11

What changed:

- Added `games/masked_claims/docs/RULES.md` as the original Rulepath rules contract for `masked_claims_standard`, including stable `MC-*` rule IDs for components, setup, phases, legal actions, restrictions, scoring, terminal tiebreaks, visibility, replay, bots, variants, deviations, and ambiguities.
- Added `games/masked_claims/docs/SOURCES.md` as the IP/source note for the original game, recording consulted project and external sources, what was used, what was not copied, naming/trade-dress rationale, asset/font status, public/private boundaries, and rule-ID cross-references.

Deviations from original plan:

- None. This ticket remained documentation-only and introduced no Rust, engine, tool, or web behavior.

Verification:

- `node scripts/check-doc-links.mjs` passed.
- Manual IP review posture recorded in `SOURCES.md`: names, labels, prose, and current asset/font status are original or not applicable; no source prose/assets are copied.

# GAT91SECDRACOM-001: Veiled Draft RULES.md + SOURCES.md

**Status**: PENDING
**Priority**: HIGH
**Effort**: Medium
**Engine Changes**: None — authors `games/secret_draft/docs/RULES.md` and `games/secret_draft/docs/SOURCES.md` only (front-loaded per `docs/OFFICIAL-GAME-CONTRACT.md` §3).
**Deps**: none

## Problem

Gate 9.1 admits a new official game, `secret_draft` (public name `Veiled Draft`), the focused proof for the two ROADMAP §11 lines Gate 9 deferred — *simultaneous choices remain hidden until reveal* and *UI shows pending seats without leaking choices*. Per `docs/OFFICIAL-GAME-CONTRACT.md` §3, original rules prose and source notes must exist before implementation so every later ticket implements against committed, IP-clean rules rather than inventing them mid-code.

## Assumption Reassessment (2026-06-08)

1. The sibling game-doc shape is real: `games/token_bazaar/docs/` and `games/high_card_duel/docs/` each carry the canonical 11-doc set including `RULES.md` and `SOURCES.md`; templates exist at `templates/GAME-RULES.md` and `templates/GAME-SOURCES.md`. This ticket instantiates the two front-loaded docs only; the other nine are split across GAT91SECDRACOM-011/012/017.
2. The spec (`specs/gate-9-1-secret-draft-commitment-reveal.md` §Implementation reference → "Proposed original rules: Veiled Draft" and "Source notes and originality guidance") defines the rules surface (two seats, twelve tiles, six rounds, simultaneous hidden pick, deterministic conflict fallback, public scoring, tie-break ladder) and the consulted prior art (BoardGameGeek simultaneous-action-selection + open-drafting, OpenSpiel, GamesRadar) with original-content obligations. These are the authoritative inputs.
3. Cross-artifact boundary under audit: `RULES.md` is consumed downstream by `tools/rule-coverage` (it maps rules-doc obligations to tests/traces — see GAT91SECDRACOM-012) and by the rule/golden-trace tickets. Rule identifiers (item IDs, threads, values, conflict fallback, scoring categories, tie-break ladder) named here become the contract those tickets implement, so they must be stable and complete now.
4. §10 IP-conservatism is the motivating principle: public files MUST use original rules prose and original names; prefer neutral IDs where trademark/trade-dress risk exists. Restate before trusting spec narrative — the game name `Veiled Draft`, tile labels (`Ember One`…`Grove Four`), and rules prose must be original and IP-reviewed; consulted sources shape only mechanic vocabulary, never copied content (spec A2, lines on source notes).

## Architecture Check

1. Front-loading rules+sources is cleaner than deriving rules from code: it gives `rule-coverage` and golden-trace tickets a fixed obligation list, and forces the IP review before any label ships in a bundle.
2. No backwards-compatibility aliasing/shims — these are new files.
3. `engine-core` untouched (no mechanic nouns enter the kernel); no `game-stdlib` change. All drafting/commitment/reveal/pool/tile/scoring nouns live only in the doc prose for `games/secret_draft`.

## Verification Layers

1. Rules completeness (every obligation later tickets must cover is named) -> manual review against spec §Implementation reference + `docs/OFFICIAL-GAME-CONTRACT.md` §3.
2. IP originality (name, labels, prose original; sources credited not copied) -> manual review / IP-conservatism audit (§10).
3. Doc-link integrity -> `node scripts/check-doc-links.mjs`.
4. Single-artifact-pair ticket: no cross-crate invariant mapping applies beyond the rule-coverage contract named in Assumption Reassessment item 3.

## What to Change

### 1. `games/secret_draft/docs/RULES.md`

Instantiate from `templates/GAME-RULES.md`. Cover, in original prose: components (two seats; twelve visible tiles with `item_id` / `thread` ∈ {ember, tide, grove} / `value` ∈ {1,2,3,4} / original label; public round marker 1..=6; alternating priority seat; per-seat public drafted collection; per-seat hidden commitment slot; public score summary); setup; the six-round commit→pending→reveal flow; conflict resolution and drafting-with-removal (different items → each keeps choice; same item → priority seat takes contested item, other seat takes lowest stable-order remaining item, both removed); scoring (base = sum of values; +3 per complete ember/tide/grove set, each tile in ≤1 set; +2 high-thread bonus once per thread for ≥3 same-thread tiles; +1 conflict-discipline bonus at terminal); terminal after round 6; the public tie-break ladder (score → set count → highest single tile value → distinct threads → fewer priority-won contested items → Draw). State that all formulas live in typed Rust and static data carries only IDs/labels/values.

### 2. `games/secret_draft/docs/SOURCES.md`

Instantiate from `templates/GAME-SOURCES.md`. Record consulted date, each consulted source (BoardGameGeek "Simultaneous Action Selection" + "Open Drafting"; Lanctot et al. OpenSpiel arXiv:1908.09453; OpenSpiel docs/repo; GamesRadar board-game-types), what each was used for (generic mechanic vocabulary / imperfect-information modeling context), what was NOT copied (rules, names, labels, UI, assets, trade dress), why the game name and tile labels are original, and asset/font status. Note the rename obligation if any label reads trademark-forward.

## Files to Touch

- `games/secret_draft/docs/RULES.md` (new)
- `games/secret_draft/docs/SOURCES.md` (new)

## Out of Scope

- The other nine per-game docs (MECHANICS/UI/AI/ADMISSION/PUBLIC-RELEASE/COMPETENT-PLAYER/BOT-STRATEGY → GAT91SECDRACOM-017; RULE-COVERAGE → 012; BENCHMARKS → 011).
- Any Rust, data, or web code — this ticket is rules+sources prose only.
- Final tie-break-value or tile-count tuning beyond the spec's defaults (spec A3/A5 allow maintainer tuning, but defaults are committed here).

## Acceptance Criteria

### Tests That Must Pass

1. `node scripts/check-doc-links.mjs` passes with the two new docs present.
2. Manual review confirms RULES.md names every obligation (components, setup, round flow, conflict fallback, scoring, terminal, tie-break ladder) that GAT91SECDRACOM-005/009/010/012 must implement/verify.
3. Manual IP audit confirms name, labels, and prose are original and sources are credited without copied content.

### Invariants

1. Static data carries no formulas — RULES.md states scoring/conflict/tie-breaks live in typed Rust (§5).
2. No mechanic noun is introduced anywhere outside `games/secret_draft` prose (§3).

## Test Plan

### New/Modified Tests

1. `None — documentation-only ticket; verification is command-based (check-doc-links) plus manual rules-completeness and IP audit. Downstream test/trace coverage is authored in GAT91SECDRACOM-009/010/012.`

### Commands

1. `node scripts/check-doc-links.mjs`
2. `ls games/secret_draft/docs/RULES.md games/secret_draft/docs/SOURCES.md`
3. A narrower command is correct here: no crate exists yet, so `cargo`/simulation verification is not applicable until GAT91SECDRACOM-002 creates the crate skeleton.

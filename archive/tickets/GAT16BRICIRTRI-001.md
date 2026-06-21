# GAT16BRICIRTRI-001: Briar Circuit rules summary and IP source notes

**Status**: COMPLETED
**Priority**: HIGH
**Effort**: Medium
**Engine Changes**: None (docs — `games/briar_circuit/docs/RULES.md`, `games/briar_circuit/docs/SOURCES.md`, `docs/SOURCES.md`)
**Deps**: None

## Problem

Gate 16 admits **Briar Circuit** (classic four-player Hearts rules family) as the first official fixed-four-seat trick-taking penalty game. `docs/OFFICIAL-GAME-CONTRACT.md` §3 front-loads original rules prose and source notes before implementation, and `docs/FOUNDATIONS.md` §10 requires original IP. This ticket authors the original `RULES.md` with the stable `BC-*` rule-ID set (spec Appendix A) and the per-game/global source notes every later ticket cites.

## Assumption Reassessment (2026-06-20)

1. No `games/briar_circuit/` crate exists yet (`Cargo.toml` workspace members list 15 games, none `briar_circuit`), and `tools/rule-coverage/src/main.rs` registers no `BC-` prefix — this ticket defines the rule-ID families the prefix validator gains in GAT16BRICIRTRI-012.
2. `specs/gate-16-briar-circuit-trick-taking.md` §3.1 (locked variant), §10.2 (central source note), and Appendix A (the `BC-SETUP/DEAL/PASS/PLAY/TRICK/SCORE/MATCH/VIS/REPLAY/BOT/UI/OUTCOME-*` ID set) fix this content; the doc set mirrors the `games/plain_tricks/docs/` 13-file convention.
3. Cross-artifact boundary under audit: the `BC-*` rule IDs authored here are the contract consumed by `RULE-COVERAGE.md` (002/012), every module's rule hooks, and `tools/rule-coverage`. `docs/SOURCES.md` is extended additively (Pagat/Bicycle Hearts + OpenSpiel prior-art + W3C accessibility notes), not duplicated.
4. FOUNDATIONS §10 (IP conservatism) motivates this ticket: rules prose, the neutral "Briar Circuit" name, and all presentation are original; external Hearts/OpenSpiel/W3C references verify only public-domain rules-family facts and license no copied prose, tables, or trade dress (spec §1 naming rationale, §10.2).

## Architecture Check

1. Authoring rules + IDs before code makes the `BC-*` IDs the single source of truth that coverage, traces, and UI reference, mirroring the OGC §3 rules-first order proven across prior game gates.
2. No backwards-compatibility aliasing/shims — new game-local docs; `docs/SOURCES.md` is extended additively.
3. `engine-core` is untouched (§3); no `game-stdlib` change (§4) — docs-only. Card/suit/rank/trick/pass/moon vocabulary stays game-local prose.

## Verification Layers

1. Rule-ID families complete and well-formed (`BC-SETUP/DEAL/PASS/PLAY/TRICK/SCORE/MATCH/VIS/REPLAY/BOT/UI/OUTCOME-*`) -> grep-proof of `BC-` IDs in `RULES.md` against spec Appendix A.
2. Doc-link integrity across the new docs -> `node scripts/check-doc-links.mjs`.
3. IP conservatism (original prose, no copied Hearts text or card-brand trade dress) -> manual review against FOUNDATIONS §10 and the spec's source-use limits.

## What to Change

### 1. `games/briar_circuit/docs/RULES.md`

Original Rulepath rules summary with stable `BC-*` IDs covering: exactly four seats; 52-card deck; deterministic deal; the left/right/across/hold pass cycle; 2♣ opening; follow-suit; the exact legality order (spec §3.1); first-trick point restriction + no-alternative exception; hearts-broken lead rules; trick comparator/capture; point values; hand scoring; fixed add-26 shoot-the-moon; 100 threshold; low-tie continuation; visibility; replay; bots; outcome. Mirror Appendix A's minimum rule-identity set.

### 2. `games/briar_circuit/docs/SOURCES.md`

Per-game source notes, source-use limits, the deliberate rule choices (strict first-trick rule, Q♠ does not break hearts, fixed add-26 moon, 100 threshold, low-tie continuation, no shoot-the-sun), variant/naming rationale, and consulted external references (Pagat, Bicycle, OpenSpiel prior-art only, W3C) per spec §10.2 / Appendix E.

### 3. `docs/SOURCES.md`

Add the global Briar Circuit / Hearts rules-family source note per spec §10.2.

## Files to Touch

- `games/briar_circuit/docs/RULES.md` (new)
- `games/briar_circuit/docs/SOURCES.md` (new)
- `docs/SOURCES.md` (modify)

## Out of Scope

- Any Rust code or crate scaffold (GAT16BRICIRTRI-004).
- Admission receipt, player-facing rules, and the initial coverage map (GAT16BRICIRTRI-002).
- Registering the `BC-*` prefix in `tools/rule-coverage` (GAT16BRICIRTRI-012).

## Acceptance Criteria

### Tests That Must Pass

1. `grep -E 'BC-(SETUP|DEAL|PASS|PLAY|TRICK|SCORE|MATCH|VIS|REPLAY|BOT|UI|OUTCOME)-' games/briar_circuit/docs/RULES.md` — every planned family present.
2. `node scripts/check-doc-links.mjs` — passes with the new docs linked.
3. Manual IP review confirms no copied prose, card-brand trade dress, or casino/real-money language.

### Invariants

1. Rules behavior is described, never encoded as data (§5); IDs are stable and original (§10).
2. External sources verify rules facts only; product identity stays original (§10).

## Test Plan

### New/Modified Tests

1. `None — documentation-only ticket; verification is command-based and existing pipeline/test coverage is named in Assumption Reassessment.`

### Commands

1. `grep -nE 'BC-[A-Z]+-' games/briar_circuit/docs/RULES.md`
2. `node scripts/check-doc-links.mjs`
3. A narrower command is correct here because the deliverable is prose + IDs; rule-coverage enforcement is exercised once code and the `BC-` prefix validator exist (GAT16BRICIRTRI-012).

## Outcome

Completed: 2026-06-21

What changed:

- Added `games/briar_circuit/docs/RULES.md` with original Briar Circuit rules prose, stable `BC-*` rule IDs, fixed-four-seat Hearts-family variant decisions, Rust-owned legality/scoring/visibility/replay/bot boundaries, and out-of-scope/forbidden-rule notes.
- Added `games/briar_circuit/docs/SOURCES.md` with consulted project/rules/accessibility references, source-use limits, naming rationale, variant/ambiguity decisions, asset/IP review posture, and rule-source-to-rule-ID cross-reference.
- Added the Briar Circuit source-note row to `docs/SOURCES.md`.

Deviations from plan:

- None. This ticket stayed docs-only and introduced no Rust crate, tooling, web, or data files beyond the scoped source/rules documentation.

Verification:

- `grep -nE 'BC-[A-Z]+-' games/briar_circuit/docs/RULES.md` passed; the planned `BC-SETUP`, `BC-DEAL`, `BC-PASS`, `BC-PLAY`, `BC-TRICK`, `BC-SCORE`, `BC-MATCH`, `BC-VIS`, `BC-REPLAY`, `BC-BOT`, `BC-UI`, and `BC-OUTCOME` families are present.
- `node scripts/check-doc-links.mjs` passed (`Checked 27 markdown files`).
- Manual IP/source review completed against `docs/IP-POLICY.md`: the new docs use original Rulepath prose, cite external Hearts-family sources as facts only, introduce no copied prose/assets/trade dress, and keep the public name as Briar Circuit.

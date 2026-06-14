# GAT15RIVLEDTEX-001: River Ledger rules summary and IP source notes

**Status**: PENDING
**Priority**: HIGH
**Effort**: Medium
**Engine Changes**: None (docs — `games/river_ledger/docs/RULES.md`, `games/river_ledger/docs/SOURCES.md`, `docs/SOURCES.md`)
**Deps**: None

## Problem

Gate 15 admits River Ledger (Texas Hold'Em rules family) as the first official 3–6-seat hidden-information betting game. `docs/OFFICIAL-GAME-CONTRACT.md` §3 front-loads original rules prose and source notes before implementation, and `docs/FOUNDATIONS.md` §10 requires original IP. This ticket authors the original `RULES.md` with the stable `RL-*` rule-ID set and the per-game/global source notes that every later ticket cites.

## Assumption Reassessment (2026-06-14)

1. No `games/river_ledger/` crate exists yet (`Cargo.toml` workspace members list 14 games, none `river_ledger`), and `tools/rule-coverage/src/main.rs` `is_rule_id` registers no `RL-` prefix — this ticket defines the rule-ID families the prefix validator gains in GAT15RIVLEDTEX-015.
2. `specs/gate-15-river-ledger-texas-holdem-base.md` §4.2/§10.2 fixes the `RULES.md`/`SOURCES.md` content and the global `docs/SOURCES.md` Hold'Em note; `docs/SOURCES.md` already carries Pagat Hold'Em/ranking placeholder rows (lines ~290) to expand, not duplicate.
3. Cross-artifact boundary under audit: the `RL-*` rule IDs authored here are the contract consumed by `RULE-COVERAGE.md` (002/015), every module's rule hooks, and `tools/rule-coverage`; the doc set mirrors the `games/poker_lite/docs/` 13-file convention.
4. FOUNDATIONS §10 (IP conservatism) motivates this ticket: rules prose, naming, and the "River Ledger" product identity are original; external Hold'Em/ranking references verify only public-domain rules-family facts and license no copied prose, tables, or casino trade dress.

## Architecture Check

1. Authoring rules + IDs before code makes the `RL-*` IDs the single source of truth that coverage, traces, and UI reference, mirroring the OGC §3 rules-first order proven across prior game gates.
2. No backwards-compatibility aliasing/shims — new game-local docs; `docs/SOURCES.md` is extended additively.
3. `engine-core` is untouched (§3); no `game-stdlib` change (§4) — docs-only.

## Verification Layers

1. Rule-ID families complete and well-formed (`RL-SETUP/DEAL/BET/STREET/EVAL/SHOW/POT/VIS/BOT/UI/REPLAY-*`) -> grep-proof of `RL-` IDs in `RULES.md`.
2. Doc-link integrity across the new docs -> `node scripts/check-doc-links.mjs`.
3. IP conservatism (original prose, no casino/copied text) -> manual review against FOUNDATIONS §10 and the spec's source-use limits.

## What to Change

### 1. `games/river_ledger/docs/RULES.md`

Original Rulepath rules summary with stable `RL-*` IDs covering seat range (3–6), setup/blinds/button, fixed-limit capped-raise betting, streets (preflop/flop/turn/river/showdown), the one-bet-plus-three-raises cap, the abstract contribution ledger, the seven-card evaluator, showdown + split/remainder, visibility, replay, bots, and the out-of-scope all-in/side-pots note.

### 2. `games/river_ledger/docs/SOURCES.md`

Per-game source notes, source-use limits, variant/naming rationale, and the exact consulted external references (Pagat Hold'Em, Pagat ranking, Fournier, OpenSpiel, boardgame.io) per spec §3.1 source posture.

### 3. `docs/SOURCES.md`

Add the global Texas Hold'Em rules-family / River Ledger source note per spec §10.2.

## Files to Touch

- `games/river_ledger/docs/RULES.md` (new)
- `games/river_ledger/docs/SOURCES.md` (new)
- `docs/SOURCES.md` (modify)

## Out of Scope

- Any Rust code or crate scaffold (GAT15RIVLEDTEX-003).
- The remaining game docs (admission/mechanics/coverage in 002; trailing docs in 019).
- Registering the `RL-*` prefix in `tools/rule-coverage` (GAT15RIVLEDTEX-015).

## Acceptance Criteria

### Tests That Must Pass

1. `node scripts/check-doc-links.mjs` — passes with the new docs linked.
2. `grep -E 'RL-(SETUP|DEAL|BET|STREET|EVAL|SHOW|POT|VIS|BOT|UI|REPLAY)-' games/river_ledger/docs/RULES.md` — every planned family present.
3. Manual IP review confirms no copied prose, casino trade dress, or chip/cash language.

### Invariants

1. Rules behavior is described, never encoded as data (§5); IDs are stable and original (§10).
2. External sources verify rules facts only; product identity stays original (§10).

## Test Plan

### New/Modified Tests

1. `None — documentation-only ticket; verification is command-based and existing pipeline/test coverage is named in Assumption Reassessment.`

### Commands

1. `grep -nE 'RL-[A-Z]+-' games/river_ledger/docs/RULES.md`
2. `node scripts/check-doc-links.mjs`
3. A narrower command is correct here because the deliverable is prose + IDs; rule-coverage enforcement is exercised once code and the prefix validator exist (GAT15RIVLEDTEX-015).

# GAT7DRALITCOM-001: Draughts Lite rules research & IP source docs

**Status**: COMPLETED
**Priority**: HIGH
**Effort**: Medium
**Engine Changes**: None — documentation only (`games/draughts_lite/docs/RULES.md`, `games/draughts_lite/docs/SOURCES.md`).
**Deps**: None

## Problem

Per `docs/OFFICIAL-GAME-CONTRACT.md` §3, original rules prose and source notes precede implementation. Gate 7 admits Draughts Lite — a deliberately small English draughts / American checkers subset — and every later ticket (rules core, action tree, tests, docs) depends on a single authoritative statement of the rules contract and its IP provenance. This ticket writes `RULES.md` (the playable rules) and `SOURCES.md` (adopted rules, omitted adjudication, IP policy), so downstream tickets implement against fixed prose rather than re-deriving rules ad hoc.

## Assumption Reassessment (2026-06-07)

1. No `games/draughts_lite/` directory exists yet (`test -e games/draughts_lite` → absent); these are the first files under the new game tree. `games/directional_flip/docs/RULES.md` and `games/directional_flip/docs/SOURCES.md` are the structural precedents, and the doc filenames map from `templates/GAME-RULES.md` / `templates/GAME-SOURCES.md` (verified present).
2. The rules contract is fixed by the spec `specs/gate-7-draughts-lite-compound-action-tree.md` §R8 (board/parity, setup, men/king movement, capture, mandatory capture, mandatory continuation, promotion, promotion-during-capture stop, terminal outcomes) and §R5 (research rationale); `docs/ROADMAP.md` §9 names "promotion if scoped" and forbids "full chess exception load".
3. Cross-artifact boundary under audit: `RULES.md` is consumed by `tools/rule-coverage` (GAT7DRALITCOM-017) and is the reference for the rule-test suite (013) and golden-trace notes (014). The rules version string `draughts_lite-rules-v1` (spec §R8) is the contract token threaded through traces and WASM registration.
4. FOUNDATIONS §10 (IP conservatism) motivates `SOURCES.md`: restate before writing — public files use original rules prose, cite the WCDF English rules as the source of the adopted ruleset, copy no rulebook prose beyond short attributed quotation, and ship no proprietary assets, engine data, or opening books. The spec §R20 / §Sources enumerate the citations (WCDF, Schaeffer *Checkers Is Solved* for the no-strong-engine rationale).

## Architecture Check

1. Front-loading rules prose (vs. inferring rules from code later) is the OGC §3 contract — it gives every implementation ticket one citable source and prevents rule drift between the Rust code, traces, and docs.
2. No backwards-compatibility shims; these are new files.
3. `engine-core` is untouched (§3); these are game-local docs. No mechanic noun enters the kernel. `game-stdlib` is untouched (§4).

## Verification Layers

1. Rules completeness -> manual review: `RULES.md` states mandatory capture, mandatory continuation, no maximum-capture rule, promotion, promotion-during-capture stop, both terminal-win conditions, and the omitted draw/tournament adjudication (spec §R8).
2. IP conservatism -> manual review / IP-conservatism audit: `SOURCES.md` cites WCDF English rules, lists adopted vs. omitted rules, and contains no copied rulebook prose or proprietary data (FOUNDATIONS §10).
3. Doc-link integrity -> `node scripts/check-doc-links.mjs` (any intra-doc links resolve).

## What to Change

### 1. `RULES.md`

Author from `templates/GAME-RULES.md`: identity (`draughts_lite`, `draughts_lite_standard`, `draughts_lite-rules-v1`), 8×8 board with dark-square parity (`row + column` odd), 12-men setup per side (rows 1–3 / 6–8), men forward-only diagonal move/capture, kings any-diagonal one-square move/capture, mandatory capture, mandatory same-piece continuation, no maximum-capture rule, promotion on the far king row, promotion-during-capture ends the move immediately, terminal wins (opponent has no pieces or no legal move; stalemate is a loss), and an explicit statement that draw/tournament adjudication is omitted in Gate 7.

### 2. `SOURCES.md`

Author from `templates/GAME-SOURCES.md`: cite the WCDF English draughts/checkers rules as the adopted ruleset, list which rules are adopted and which adjudication details are intentionally omitted, cite Schaeffer et al. *Checkers Is Solved* for the rationale that excludes strong-engine claims, and satisfy the IP policy (original prose, no proprietary assets/engine data/opening book).

## Files to Touch

- `games/draughts_lite/docs/RULES.md` (new)
- `games/draughts_lite/docs/SOURCES.md` (new)

## Out of Scope

- Any Rust code, crate scaffold, or `Cargo.toml` wiring (GAT7DRALITCOM-004).
- `MECHANICS.md`, `AI.md`, and the remaining doc package (GAT7DRALITCOM-021 and validator-co-located docs).
- The primitive-pressure decision (GAT7DRALITCOM-002).

## Acceptance Criteria

### Tests That Must Pass

1. `node scripts/check-doc-links.mjs` — passes (no broken links introduced).
2. Manual review confirms `RULES.md` covers every rule clause in spec §R8 and `SOURCES.md` covers adopted/omitted rules + IP policy.

### Invariants

1. Public rules prose is original and IP-clean; no copied rulebook prose beyond short attributed quotation (FOUNDATIONS §10).
2. The omitted draw/tournament adjudication is stated plainly so no later ticket assumes it exists (spec §R8 terminal-outcomes).

## Test Plan

### New/Modified Tests

1. `None — documentation-only ticket; verification is command-based (`check-doc-links.mjs`) and rule coverage is enforced later by GAT7DRALITCOM-017 (`tools/rule-coverage`).`

### Commands

1. `node scripts/check-doc-links.mjs`
2. `node scripts/check-doc-links.mjs && bash scripts/boundary-check.sh`
3. Doc-link + boundary checks are the correct boundary for a docs-only ticket; rule-clause-to-test mapping is verified when `RULE-COVERAGE.md` and the rule-coverage tool land (GAT7DRALITCOM-017).

## Outcome

Completed: 2026-06-07

What changed:
- Added `games/draughts_lite/docs/RULES.md` with the `draughts_lite_standard` rules contract, stable rule IDs, mandatory capture, mandatory same-piece continuation, no maximum-capture rule, promotion-during-capture stop, terminal wins, visibility, replay, bot, UI, and out-of-scope variant notes.
- Added `games/draughts_lite/docs/SOURCES.md` with consulted sources, adopted versus omitted rules, ambiguity resolutions, IP/trade-dress notes, public/private content boundary, and rule-source cross-references.

Deviations from original plan:
- None.

Verification:
- `node scripts/check-doc-links.mjs` passed (`Checked 22 markdown files`).
- `bash scripts/boundary-check.sh` passed (`engine-core boundary check passed`).
- Manual review confirmed `RULES.md` covers the Gate 7 rules clauses and `SOURCES.md` records adopted/omitted rules plus IP policy without copied source prose.

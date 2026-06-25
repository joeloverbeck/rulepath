# GAT19MELLEDFIV-001: Meldfall Ledger rules prose and source/IP notes

**Status**: PENDING
**Priority**: HIGH
**Effort**: Small
**Engine Changes**: None — game-local docs (`games/meldfall_ledger/docs/RULES.md`, `SOURCES.md`)
**Deps**: None

## Problem

Gate 19 ships **Meldfall Ledger**, a neutral presentation of the Five Hundred Rummy / Rummy 500 family. Per `docs/OFFICIAL-GAME-CONTRACT.md` §3, original rules prose must precede implementation, and per FOUNDATIONS §10 the public files must use original prose under a neutral name with public-domain source evidence. This ticket front-loads `RULES.md` (original rules) and `SOURCES.md` (research, variant pinning, neutral-name/IP note) so downstream tickets (scoring, outcome explanations, rule-coverage) can cite stable rule IDs.

## Assumption Reassessment (2026-06-25)

1. House-style exemplar `games/blackglass_pact/docs/RULES.md` and `SOURCES.md` exist and define the rules-prose + stable rule-ID convention; `games/blackglass_pact/docs/` carries no `docs/README.md`, so this game-local doc set follows that 14-doc shape (confirmed during reassessment).
2. Spec `specs/gate-19-meldfall-ledger-five-hundred-rummy.md` §4.2 (RULES.md / SOURCES.md content rows) and Appendix A (external rules sources: Pagat, Bicycle, Rummy Rulebook; variant decision matrix) define required content; the variant is pinned `classic_500_single_deck_v1`.
3. Cross-artifact: `RULES.md` scoring/terminal rule IDs are later consumed by `scripts/check-outcome-explanations.mjs` and `tools/rule-coverage`; this ticket authors stable IDs (e.g. `score-tabled`, `score-inhand-penalty`, `match-target-500`, `match-tie-continue`) so those validators have a fixed target. No tool registration here.
4. FOUNDATIONS §10 IP conservatism: prose must be original (no copied rulebook text), the public name is the neutral "Meldfall Ledger", and the common name stays in source notes only. Human IP/legal review remains a separate release gate per `docs/IP-POLICY.md`.

## Architecture Check

1. Authoring rules-first (before any code) keeps the spec's exit-criteria, scoring IDs, and rule-coverage matrix anchored to a single source of truth, avoiding retrofitting prose to match code.
2. No backwards-compatibility shims — these are new files.
3. `engine-core` is untouched (docs only); no mechanic nouns enter the kernel and no `game-stdlib` change is implied.

## Verification Layers

1. Stable rule IDs present and unique -> codebase grep-proof against `RULES.md`.
2. Original prose, neutral name, no trade-dress -> manual IP-conservatism review (FOUNDATIONS §10).
3. Internal doc links resolve -> `node scripts/check-doc-links.mjs`.

## What to Change

### 1. `RULES.md`

Author original prose for: single 52-card deck, variable 2–6 seats (13 cards for 2, 7 for 3–6), one face-up initial discard, draw from stock or discard pile, melds (sets 3–4 same rank; runs 3+ same suit; ace low/high no-wrap), multi-card discard-pile pickup with immediate-use commitment, lay-off onto any public meld with score-credit to the playing seat, going out (with/without final discard), stock-exhaustion settlement, per-card scoring (ace 15, K/Q/J/10 = 10, 2–9 pip), cumulative 500-point target with unique-winner tie continuation. Assign stable rule IDs, including scoring/terminal IDs.

### 2. `SOURCES.md`

Source summary for the Five Hundred Rummy / Rummy 500 family, the pinned single-deck variant, deliberate deviations (no jokers, no two-deck shoe, no opening minimum, top-discard immediate-use), the neutral-name rationale, and prior-art/strategy/UX references (per spec Appendix A).

## Files to Touch

- `games/meldfall_ledger/docs/RULES.md` (new)
- `games/meldfall_ledger/docs/SOURCES.md` (new)

## Out of Scope

- `HOW-TO-PLAY.md` and the generated player-rules markdown (GAT19MELLEDFIV-019).
- `MECHANICS.md` / `UI.md` / `AI.md` / `RULE-COVERAGE.md` (later tickets).
- Any Rust implementation or rule IDs wired into code.

## Acceptance Criteria

### Tests That Must Pass

1. `node scripts/check-doc-links.mjs` passes with the new docs present.
2. Each scoring/terminal rule ID referenced by later tickets exists exactly once in `RULES.md` (grep).
3. `SOURCES.md` carries the neutral-name rationale and excluded-variant notes.

### Invariants

1. All rules prose is original; the common game name appears only in `SOURCES.md`, never as the public display name.
2. Rule IDs are stable, unique tokens later validators can grep against.

## Test Plan

### New/Modified Tests

1. `None — documentation-only ticket; verification is command-based (`check-doc-links`) and IP review is manual.`

### Commands

1. `node scripts/check-doc-links.mjs`
2. `grep -nE "^(###|- ).*(score-|match-)" games/meldfall_ledger/docs/RULES.md` (confirm stable scoring/terminal IDs exist)
3. A narrower command is correct here: no crate exists yet, so `cargo` verification belongs to GAT19MELLEDFIV-003 onward.

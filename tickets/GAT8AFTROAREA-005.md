# GAT8AFTROAREA-005: Add Gate 9 candidate-placement source notes to docs/SOURCES.md

**Status**: PENDING
**Priority**: LOW
**Effort**: Small
**Engine Changes**: None — `docs/SOURCES.md` prose only.
**Deps**: GAT8AFTROAREA-002

## Problem

If the candidate-placement note added to `progress.md` (GAT8AFTROAREA-002) cites external research for the Gate 9 routing rationale — the BoardGameGeek "Market"/"Contracts" mechanic vocabulary and W3C/WAI accessibility guidance for economy UI — those references belong in `docs/SOURCES.md` under the source-use rules, not inline in a progress doc. This ticket adds the missing provenance (spec D7 / WB7). It is **CONDITIONAL**: it is worked only if the candidate-placement note actually cites external research; if the note carries no external citations, the spec's own §External-references section already records the provenance and this ticket closes not-applicable.

## Assumption Reassessment (2026-06-08)

1. `docs/SOURCES.md` already contains an `### OpenSpiel` entry (`docs/SOURCES.md:84`, under "AI, bots, and simulations"), a `## Blackjack placement audit references` section (`:121`), and a `## Web UI, rendering, and accessibility` section (`:140`). So OpenSpiel and (likely) the W3C/WAI accessibility refs are already present; the genuinely-new provenance is the BGG "Market"/"Contracts" mechanic-vocabulary links. Confirm which of the spec's three external references (OpenSpiel / BGG market+contracts / W3C use-of-color + grid) are missing before adding — add only the absent ones.
2. The reference set is sourced from the spec's §External references (OpenSpiel intro + arXiv; BGG `boardgamemechanic/2900/market` and `/2912/contracts`; W3C `WCAG22/Understanding/use-of-color` and `ARIA/apg/patterns/grid`). The candidate-placement note's actual citations (GAT8AFTROAREA-002) determine what this ticket adds.
3. Cross-artifact boundary under audit: `docs/SOURCES.md` is the provenance ledger governed by its own "Source-use rules" (`:17`) — vocabulary/rationale only, no copied rules prose. This ticket adds reference entries consistent with that section; it does not alter the candidate-placement note itself.
4. FOUNDATIONS §10 (IP conservatism) motivates routing the references here: source notes must record that BGG "Market"/"Contracts" are used as *vocabulary only* (no commercial game rules copied), matching the spec's §External-references caveat. Restate that before adding so the entries cannot read as licensing copied content.

## Architecture Check

1. Keeping Gate 9 candidate provenance in `docs/SOURCES.md` (not inline in `progress.md`) preserves the single provenance ledger and its source-use discipline — cleaner than scattering URLs across status docs.
2. No backwards-compatibility shims; additive prose entries only.
3. `engine-core` untouched; no `game-stdlib` change; entries record vocabulary only, no behavior or rules prose (§5/§10).

## Verification Layers

1. New BGG (and any other absent) references are present and labeled vocabulary-only -> codebase grep-proof (`grep -niE "market|contracts" docs/SOURCES.md`).
2. No duplicate of an already-present reference (OpenSpiel) is introduced -> manual review against `docs/SOURCES.md:84`.
3. Doc links resolve -> `node scripts/check-doc-links.mjs`.

## What to Change

### 1. Add absent Gate 9 candidate references

Under the appropriate existing `docs/SOURCES.md` sections, add the candidate-placement references the note in GAT8AFTROAREA-002 cites that are not already present — at minimum the BGG "Market" and "Contracts" mechanic-vocabulary links, each with a one-line note that they are used as vocabulary only (no copied commercial rules), per the spec's §External-references caveat. Do not duplicate the existing OpenSpiel entry; add the W3C accessibility refs only if not already under "Web UI, rendering, and accessibility."

## Files to Touch

- `docs/SOURCES.md` (modify)

## Out of Scope

- The candidate-placement note itself (GAT8AFTROAREA-002).
- Proceeding at all if GAT8AFTROAREA-002's candidate note cites no external research — this ticket is then not-applicable; the spec's §External-references section already records the provenance.
- Any game-specific source notes (those live per-game under `games/*/docs/SOURCES.md`).

## Acceptance Criteria

### Tests That Must Pass

1. `grep -niE "market|contracts" docs/SOURCES.md` shows the BGG mechanic-vocabulary references (when the ticket is applicable).
2. No duplicate OpenSpiel entry is added (manual review against `docs/SOURCES.md:84`).
3. `node scripts/check-doc-links.mjs` passes.

### Invariants

1. `docs/SOURCES.md` records Gate 9 candidate provenance as vocabulary/rationale only — no copied commercial rules (FOUNDATIONS §10).
2. References added here match the citations actually used by the candidate-placement note (no orphan provenance).

## Test Plan

### New/Modified Tests

1. `None — documentation-only, conditional ticket; verification is command-based and existing source-ledger structure is named in Assumption Reassessment.`

### Commands

1. `grep -niE "market|contracts|use-of-color|grid" docs/SOURCES.md` — targeted proof of the added references.
2. `node scripts/check-doc-links.mjs` — full doc-link integrity pass.
3. A narrow grep is the correct boundary because the change is additive prose; no Rust/test pipeline is affected.

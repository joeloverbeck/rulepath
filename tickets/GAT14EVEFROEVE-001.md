# GAT14EVEFROEVE-001: Rules and IP source docs for Event Frontier

**Status**: PENDING
**Priority**: HIGH
**Effort**: Medium
**Engine Changes**: None — documentation only (`games/event_frontier/docs/RULES.md`, `games/event_frontier/docs/SOURCES.md`)
**Deps**: None

## Problem

`docs/OFFICIAL-GAME-CONTRACT.md` §3 requires original rules prose and source notes to precede implementation, so the implemented Rust matches a written, stable-rule-ID'd specification rather than the reverse. Event Frontier is the Gate 14 capstone: an event-deck-driven, two-faction frontier game with eligibility/initiative, compound operations, edict rule-exceptions, periodic Reckonings, and asymmetric victory. `RULES.md` must define every rule with a stable ID (consumed later by `RULE-COVERAGE.md` and `tools/rule-coverage`), and `SOURCES.md` must record the consulted-mechanics-only posture and the trade-dress avoidance register, because this gate ran an external research pass (spec Assumption A10) over commercial designs whose vocabulary and presentation are protected.

## Assumption Reassessment (2026-06-12)

1. The thirteen-doc template set exists and `RULES.md`/`SOURCES.md` instantiate from `templates/GAME-RULES.md` and `templates/GAME-SOURCES.md`; verified both templates exist (`templates/GAME-RULES.md`, `templates/GAME-SOURCES.md`) and that `games/flood_watch/docs/RULES.md` + `SOURCES.md` are the closest hidden-info precedents and `games/frontier_control/docs/RULES.md` the closest graph/faction precedent.
2. The proposed rules, constants, scenarios, victory conditions, edict list, and IP-risk register live in `specs/gate-14-event-frontier-event-complexity-capstone.md` (Implementation reference: "Proposed original rules", "IP-risk register", "External research notes"); this ticket transcribes them into stable-ID rules prose, it does not invent new rules.
3. Cross-artifact boundary under audit: the stable rule IDs authored here are the contract that `RULE-COVERAGE.md` (ticket 015) maps to tests and `tools/rule-coverage` validates; rule-ID naming must be greppable and stable from first authoring (eligibility-table, edict, Reckoning-order, victory, tiebreak IDs per spec Work-breakdown item 12).
4. FOUNDATIONS §10 IP conservatism motivates `SOURCES.md`: public files must use original rules prose; researched commercial designs (GMT COIN series, Twilight Struggle/CDG, El Grande, Root) inform mechanics and engineering patterns only. Restated before trusting the spec: mechanics are unprotected; names, prose, art direction, and presentation must be original, and `docs/IP-POLICY.md` §11/§13 naming review precedes implementation.

## Architecture Check

1. Authoring rules prose with stable IDs first is cleaner than back-deriving docs from code: it fixes the contract `tools/rule-coverage` checks and prevents rule-ID churn once tests cite them.
2. No backwards-compatibility aliasing/shims — these are new documents.
3. `engine-core` is untouched (no mechanic nouns enter the kernel); no `game-stdlib` promotion is implied — this is `games/event_frontier`-local documentation.

## Verification Layers

1. Rules completeness (every mechanic has a stable ID) -> manual review against the spec's "Proposed original rules" section plus a grep that each ID class (eligibility/edict/Reckoning/victory/tiebreak) is present.
2. IP conservatism (no protected vocabulary/trade dress) -> manual IP-conservatism audit against the spec's IP-risk register; grep `RULES.md`/`SOURCES.md` for the forbidden tokens (`Propaganda`, `Coup`, `DEFCON`, `Castillo`, `clearing`).
3. Doc-link integrity -> `node scripts/check-doc-links.mjs`.
4. Single-artifact-pair ticket: the two docs share one concern (rules + their provenance); no cross-crate proof surface applies beyond the above.

## What to Change

### 1. Author `games/event_frontier/docs/RULES.md`

Instantiate from `templates/GAME-RULES.md`. Cover, each with a stable rule ID: components (six sites, eight trails, agents/depots/settlers/caches, funds/provisions); the 21-card deck (18 events incl. 4 edicts + 3 Reckonings) in three epochs of seven with the Reckoning-never-first constraint; the per-card sequence (public current/next card, first/second-eligible determination, the eligibility constraint table, pass income, ineligibility-for-next-card, no-eligible-faction discard); operations (Charter `survey`/`fortify`/`writ`, Freeholder `trek`/`cache`/`rally`; cost 1 resource per site; ops value bound); the fourteen events and four edicts (`Toll Roads`, `Survey Ban`, `Requisition`, `Long Season`) with their typed effects and expiry; the Reckoning pipeline (victory check → site scoring → income → reset) and the both-met rule; asymmetric instant victory (Charter ≥4-site majority, Freeholder ≥8 caches) and the final cumulative-score fallback with the Freeholder tiebreak. Mark the single hidden surface (undrawn deck order) in the hidden-information section.

### 2. Author `games/event_frontier/docs/SOURCES.md`

Instantiate from `templates/GAME-SOURCES.md`. Record the consulted-mechanics posture from the spec's "External research notes" (COIN eligibility, CDG event-vs-ops, El Grande/COIN periodic scoring, MtG-style layered modifiers as engineering prior art), with consulted dates, what was and was not used, and the originality rationale. Include the trade-dress avoidance register naming the protected vocabulary clusters to avoid.

## Files to Touch

- `games/event_frontier/docs/RULES.md` (new)
- `games/event_frontier/docs/SOURCES.md` (new)

## Out of Scope

- Any Rust code, data files, or crate scaffolding (ticket 003 onward).
- The primitive-pressure ledger and atlas updates (ticket 002).
- `RULE-COVERAGE.md`, `BENCHMARKS.md`, and the remaining per-game docs (later tickets).
- Renaming the game or factions: names are original placeholders pending IP review (spec Assumption A2); author with the placeholder names and flag rename points.

## Acceptance Criteria

### Tests That Must Pass

1. `node scripts/check-doc-links.mjs` passes with the two new docs present.
2. `grep -nE "rule[-_ ]?id|RULE-" games/event_frontier/docs/RULES.md` shows stable IDs covering the eligibility table, edicts, Reckoning order, victory conditions, and tiebreak.
3. `grep -niE "propaganda|coup|defcon|castillo|caballero|clearing" games/event_frontier/docs/RULES.md games/event_frontier/docs/SOURCES.md` returns no protected-vocabulary hits.

### Invariants

1. Every mechanic the spec's "Proposed original rules" defines has exactly one stable rule ID in `RULES.md`.
2. `SOURCES.md` records mechanics-only consultation with an explicit originality rationale; no rulebook prose, card text, or trade dress is reproduced.

## Test Plan

### New/Modified Tests

1. `None — documentation-only ticket; verification is command-based (doc-link check + greps) and downstream rule-coverage (ticket 015) will bind the rule IDs authored here.`

### Commands

1. `node scripts/check-doc-links.mjs`
2. `grep -nE "rule[-_ ]?id|RULE-" games/event_frontier/docs/RULES.md`
3. A narrower full-build command is not the correct boundary here — this is prose with no compilable surface; doc-link integrity plus the rule-ID/IP greps are the verification boundary.

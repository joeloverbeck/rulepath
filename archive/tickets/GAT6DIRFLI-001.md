# GAT6DIRFLI-001: Directional Flip rules research & IP source docs

**Status**: COMPLETED
**Priority**: HIGH
**Effort**: Medium
**Engine Changes**: None (documentation-only — `games/directional_flip/docs/RULES.md`, `games/directional_flip/docs/SOURCES.md`)
**Deps**: None

## Problem

Gate 6 admits the official game `directional_flip` (an 8×8 directional-flipping game in the Othello/Reversi mechanic tradition). Per `docs/OFFICIAL-GAME-CONTRACT.md` §3 and `docs/FOUNDATIONS.md` §10, original Rulepath rules prose and IP/source notes must precede implementation: the rule model the rest of the gate codes against, and the IP-conservatism rationale (neutral naming, no copied prose/trade dress), have to exist and be reviewable first. This ticket produces those two documents and the initial rule-id seed; it writes no code.

## Assumption Reassessment (2026-06-07)

1. The sibling precedent `games/column_four/docs/RULES.md` and `games/column_four/docs/SOURCES.md` exist and define the house structure (rules prose + source-to-rule xref + IP rationale); this ticket mirrors that structure for `directional_flip`. The target directory `games/directional_flip/docs/` does not exist yet and is created by this ticket.
2. The spec `specs/gate-6-directional-flip.md` §4 (external research), §6 (rules model), §6.2 (setup: first seat `r4c5`/`r5c4`, second seat `r4c4`/`r5c5`, opening moves `r3c4`/`r4c3`/`r5c6`/`r6c5`), and §21 (rule-id seed `DF-*`) are authoritative for content; the cell-id convention `r1c1…r8c8` (row 1 top) matches the established repo convention in `games/three_marks/src/ids.rs` (enum `R1C1` → `"r1c1"`).
3. Cross-artifact boundary under audit: the rule prose authored here is the contract the rules-core (GAT6DIRFLI-005), tests (012), golden traces (013), and rule-coverage doc (016) all cite by rule id. Rule ids seeded here (`DF-SETUP-001`, `DF-LEGAL-001`, …) must remain stable once downstream tickets reference them.
4. FOUNDATIONS §10 IP conservatism motivates this ticket: public games must use original prose and neutral IDs/names where trademark/trade-dress risk exists. Restating the principle before drafting — `SOURCES.md` must record that no Othello-branded prose, diagrams, palette, or trade dress is copied, and that "Directional Flip" / `directional_flip` is a neutral original name. The WOF/MegaHouse trademark caution in spec §4 is the reason.

## Architecture Check

1. Front-loading rules + source docs (rather than reverse-documenting after code) keeps the rule ids stable for every downstream citation and prevents IP review from becoming an afterthought at release time — the failure mode FOUNDATIONS §10 and the public-release checklist guard against.
2. No backwards-compatibility aliasing/shims introduced; these are new documents.
3. `engine-core` is untouched (no mechanic nouns enter the kernel); `game-stdlib` is untouched (the extraction decision is GAT6DIRFLI-002, not this ticket).

## Verification Layers

1. Original-prose / IP-conservatism invariant -> manual review (IP-conservatism audit): `SOURCES.md` states the no-copy policy and neutral-naming rationale; no rulebook prose is reproduced.
2. Rule-model fidelity to spec -> manual review against `specs/gate-6-directional-flip.md` §6: setup cells, legality (bracketing), all-direction flips, forced pass, terminal, scoring/draw are all described.
3. Rule-id stability -> codebase grep-proof: every `DF-*` id in spec §21 appears in `RULES.md`/`RULE-COVERAGE` seed so downstream tickets can cite stable ids.

## What to Change

### 1. `games/directional_flip/docs/RULES.md`

Author original Rulepath rules prose for `directional_flip_standard`: 8×8 board, two seats (use repository seat conventions, not color names as authority), standard four-disc center setup, first seat acts first, legal placement requires bracketing one or more contiguous opposing discs in a direct line, applying a placement flips all bracketed discs in every qualifying direction, forced pass when (and only when) no legal placement exists, terminal when neither seat can move (proven by a double forced pass) or the board admits no continuation, higher final disc count wins, equal counts draw. Tie each rule statement to a `DF-*` rule id from spec §21.

### 2. `games/directional_flip/docs/SOURCES.md`

Record consulted sources (spec §22.1 external-research table), a source-to-rule-id cross-reference, ambiguity decisions, and the IP/trademark rationale: neutral naming, original prose, asset/font status, and an explicit statement that no Othello-branded prose/visuals/trade dress is copied. Follow the `templates/GAME-SOURCES.md` structure and the `games/column_four/docs/SOURCES.md` house style.

## Files to Touch

- `games/directional_flip/docs/RULES.md` (new)
- `games/directional_flip/docs/SOURCES.md` (new)

## Out of Scope

- Any Rust code, crate skeleton, or workspace wiring (GAT6DIRFLI-004).
- The mechanic inventory, primitive-pressure comparison, and ledger decision (GAT6DIRFLI-002).
- `RULE-COVERAGE.md` matrix (GAT6DIRFLI-016) and trailing MECHANICS/UI/AI docs (GAT6DIRFLI-020).
- Bot strategy docs (GAT6DIRFLI-010).

## Acceptance Criteria

### Tests That Must Pass

1. `test -f games/directional_flip/docs/RULES.md && test -f games/directional_flip/docs/SOURCES.md` — both docs exist.
2. `grep -c 'DF-' games/directional_flip/docs/RULES.md` — rule prose carries the seeded rule ids.
3. Manual review: `SOURCES.md` contains the explicit no-Othello-branding / original-prose statement and neutral-naming rationale.

### Invariants

1. No copied rulebook prose, diagrams, or trade dress appears in either document (FOUNDATIONS §10).
2. Every rule statement maps to a stable `DF-*` id that downstream tickets can cite.

## Test Plan

### New/Modified Tests

1. `None — documentation-only ticket; verification is command-based and existing pipeline/test coverage is named in Assumption Reassessment.`

### Commands

1. `test -f games/directional_flip/docs/RULES.md && test -f games/directional_flip/docs/SOURCES.md`
2. `node scripts/check-doc-links.mjs` — doc link integrity across the repo after adding the new docs.
3. A narrower command is correct here because no game-id is registered yet (the `tools/rule-coverage --game directional_flip` check belongs to GAT6DIRFLI-016, after the crate and `RULE-COVERAGE.md` exist).

## Outcome

Completed: 2026-06-07

What changed:

- Added `games/directional_flip/docs/RULES.md` with original Rulepath rules prose for `directional_flip_standard`, including the seeded `DF-*` rule ids from `specs/gate-6-directional-flip.md`.
- Added `games/directional_flip/docs/SOURCES.md` with consulted sources, ambiguity decisions, source-to-rule-id cross-reference, neutral naming rationale, asset/font status, and explicit no-Othello-branding / original-prose / no-trade-dress statements.

Deviations from original plan:

- None. This ticket remained documentation-only; no Rust, workspace, `engine-core`, or `game-stdlib` files were changed.

Verification results:

- `test -f games/directional_flip/docs/RULES.md && test -f games/directional_flip/docs/SOURCES.md` passed.
- `grep -c 'DF-' games/directional_flip/docs/RULES.md` returned `57`.
- Seed coverage check for every `DF-*` id listed in spec section 21 found no missing ids in `RULES.md`.
- `node scripts/check-doc-links.mjs` passed and reported `Checked 21 markdown files`.
- Manual grep confirmed `SOURCES.md` contains the explicit original-prose, neutral naming, no-Othello-branding, and no-trade-dress rationale.

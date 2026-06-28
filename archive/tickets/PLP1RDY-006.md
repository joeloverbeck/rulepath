# PLP1RDY-006: Typed event-card boundary + mechanic/scaffolding safety + templates

**Status**: COMPLETED
**Priority**: HIGH
**Effort**: Large
**Engine Changes**: None — boundary/mechanic docs + templates (`docs/ENGINE-GAME-DATA-BOUNDARY.md`, `docs/MECHANIC-ATLAS.md`, `docs/MECHANICAL-SCAFFOLDING-REGISTER.md`, `templates/GAME-MECHANICS.md`, `templates/GAME-RULE-COVERAGE.md`, `templates/GAME-EVENT-COVERAGE.md`, `templates/PRIMITIVE-PRESSURE-LEDGER.md`)
**Deps**: PLP1RDY-002

## Problem

ADR 0011 authorizes a constrained typed Rust event-card mechanism; the boundary
doc, mechanic atlas, scaffolding register, and the affected templates must
operationalize it so COIN-scale event decks are documented as typed Rust behavior
— never YAML/DSL/untyped effect rows. The spec bundles this as WB-3 (report
`A-04`, `A-06`, `A-07`, `B-04`, `B-05`, `B-15`), including a new
`templates/GAME-EVENT-COVERAGE.md`. It lands only after ADR 0011 is accepted.

## Assumption Reassessment (2026-06-28)

1. Targets verified present: `docs/ENGINE-GAME-DATA-BOUNDARY.md`,
   `docs/MECHANIC-ATLAS.md`, `docs/MECHANICAL-SCAFFOLDING-REGISTER.md`,
   `templates/GAME-MECHANICS.md`, `templates/GAME-RULE-COVERAGE.md`,
   `templates/PRIMITIVE-PRESSURE-LEDGER.md`. The new
   `templates/GAME-EVENT-COVERAGE.md` does not yet exist (confirmed; it is the
   `B-05` split deliverable).
2. Spec source: `specs/private-lane-foundation-readiness.md` WB-3 +
   §Exit-criteria item 3 (typed-registry/effect-trait pattern documented; YAML/
   JSON/TOML/RON/table-row selectors/conditions/effect formulas explicitly banned).
3. Cross-artifact boundary under audit: ADR 0011 (PLP1RDY-002) is the authorizing
   supersession for the ENGINE-GAME-DATA-BOUNDARY typed-content/behavior line;
   this ticket gates on its `Status: Accepted`. No code in this unit — the
   registry/effect trait lives in the private crate (out of scope).
4. FOUNDATIONS principle under audit (§5 static data is typed content, not
   behavior / §2 behavior authority): event payload routing and faction routing
   are **behavior**, not scaffolding (report `A-07`) — the scaffolding register
   anti-examples must keep them out of the behavior-free mechanical-scaffolding
   lane (§4 / ADR 0008).
5. §5/§11 no-DSL boundary touched: the new `GAME-EVENT-COVERAGE.md` matrix and
   the `GAME-RULE-COVERAGE.md` split document *typed* effect coverage; they must
   reject unknown fields by default and carry no behavior-looking fields
   (selectors/conditions/triggers), matching the §11 "Unknown fields … rejected"
   and "Behavior-looking fields are blocked" invariants. The templates introduce
   no leakage or nondeterminism path; enforcement stays with the private crate's
   typed Rust and the existing boundary gate.

## Architecture Check

1. Documenting the typed-registry pattern + banning untyped effect rows is the
   §5-clean realization of an event-card mechanism; the alternative (a coverage
   matrix that encodes conditions) would smuggle behavior into static docs — the
   register anti-examples foreclose it.
2. No backwards-compatibility shim: the `GAME-RULE-COVERAGE.md` split adds a
   parallel `GAME-EVENT-COVERAGE.md` rather than overloading the rule matrix.
3. `engine-core` stays noun-free (§3): COIN nouns (`card`, `deck`, `event`) are
   documented as private-game-crate / `games/*` vocabulary, never kernel nouns;
   `game-stdlib` promotion stays unearned (§4).

## Verification Layers

1. Typed-registry/effect-trait pattern documented + no-DSL ban explicit ->
   codebase grep-proof in `docs/ENGINE-GAME-DATA-BOUNDARY.md` (bans
   YAML/JSON/TOML/RON/table-row selectors/conditions/effect formulas).
2. Event/faction routing classified as behavior, not scaffolding -> grep the
   anti-example in `docs/MECHANICAL-SCAFFOLDING-REGISTER.md`.
3. New template exists + templates carry no behavior-looking fields ->
   `test -f templates/GAME-EVENT-COVERAGE.md` + FOUNDATIONS alignment check (§5/§11).
4. Acceptance precondition -> grep `^Status: Accepted` in `docs/adr/0011-*.md`.

## What to Change

### 1. `docs/ENGINE-GAME-DATA-BOUNDARY.md`

Add the typed Rust card-effect registries section (typed content = card identity/
deck order/inert metadata; behavior = Rust functions/match arms/traits); ban
untyped effect rows (report `A-04`).

### 2. `docs/MECHANIC-ATLAS.md` + `docs/MECHANICAL-SCAFFOLDING-REGISTER.md`

Add COIN-scale mechanic categories + private-stress accounting (`A-06`) and the
COIN behavior-vs-scaffolding anti-examples (`A-07`).

### 3. Templates

`templates/GAME-MECHANICS.md` COIN categories (`B-04`); `templates/GAME-RULE-COVERAGE.md`
split into rule + event-effect matrices and add new `templates/GAME-EVENT-COVERAGE.md`
(`B-05`); `templates/PRIMITIVE-PRESSURE-LEDGER.md` private-stress type (`B-15`).

## Files to Touch

- `docs/ENGINE-GAME-DATA-BOUNDARY.md` (modify)
- `docs/MECHANIC-ATLAS.md` (modify)
- `docs/MECHANICAL-SCAFFOLDING-REGISTER.md` (modify)
- `templates/GAME-MECHANICS.md` (modify)
- `templates/GAME-RULE-COVERAGE.md` (modify)
- `templates/GAME-EVENT-COVERAGE.md` (new)
- `templates/PRIMITIVE-PRESSURE-LEDGER.md` (modify)

## Out of Scope

- Any private game crate, typed registry, or effect-trait code (private repo).
- FOUNDATIONS §5 constitution text (carried in PLP1RDY-004's §13 note + ADR 0011).
- The bot/AI templates (PLP1RDY-008) and the scaling templates (PLP1RDY-009).

## Acceptance Criteria

### Tests That Must Pass

1. `grep -qiE 'typed (rust )?(card-effect )?registr' docs/ENGINE-GAME-DATA-BOUNDARY.md` and the no-untyped-effect-row ban greps.
2. `test -f templates/GAME-EVENT-COVERAGE.md` — the split template exists.
3. `grep -q '^Status: Accepted' docs/adr/0011-*.md && node scripts/check-doc-links.mjs && bash scripts/boundary-check.sh`.

### Invariants

1. Selectors/conditions/triggers/overrides remain Rust; no YAML/DSL/untyped
   effect rows are sanctioned (§5/§11).
2. Templates reject unknown fields by default and carry no behavior-looking fields.

## Test Plan

### New/Modified Tests

1. `None — boundary/mechanic docs + templates; verification is command-based (pattern/ban greps + new-template existence + boundary gate) and the no-DSL invariant set is named in Assumption Reassessment.`

### Commands

1. `grep -niE 'no (yaml|dsl)|untyped effect|selectors?|conditions?' docs/ENGINE-GAME-DATA-BOUNDARY.md`
2. `node scripts/check-doc-links.mjs && bash scripts/boundary-check.sh`
3. A narrower command suffices: docs/templates only, so pattern greps + the doc-link and boundary gates are the correct verification boundary.

## Outcome

Completed: 2026-06-28

Operationalized ADR 0011 across the boundary, mechanic, scaffolding, and
template surfaces:

- `docs/ENGINE-GAME-DATA-BOUNDARY.md` now defines the typed Rust card-effect
  registry pattern and explicitly bans YAML/JSON/TOML/RON/CSV/table-row or
  markdown-matrix selectors, conditions, triggers, effect formulas, target
  filters, rule overrides, legality, mutation, visibility decisions, and bot
  tactics.
- `docs/MECHANIC-ATLAS.md` now includes COIN-scale/private large-event review
  categories and records that private-stress evidence is not hidden promotion
  authority.
- `docs/MECHANICAL-SCAFFOLDING-REGISTER.md` now rejects event-card dispatch,
  faction eligibility, operation/special-activity coupling, propaganda/upkeep,
  persistent event expiry, and faction bot priorities as behavior rather than
  scaffolding.
- `templates/GAME-MECHANICS.md`, `templates/GAME-RULE-COVERAGE.md`, and
  `templates/PRIMITIVE-PRESSURE-LEDGER.md` now carry the private-stress/event
  coverage guidance.
- Added `templates/GAME-EVENT-COVERAGE.md` as a placeholder-only event-effect
  coverage matrix that records evidence without becoming executable behavior or
  copying private licensed text.

Deviations from plan: none. This ticket changed only docs/templates. It did not
add a private game crate, Rust registry, effect trait, CI, catalog, trace,
fixture, replay, hash, RNG, benchmark, or web source change. Existing unrelated
`.claude/skills/*` worktree changes were left untouched and unstaged.

Verification:

- `grep -niE 'typed (rust )?(card-effect )?registr|no (yaml|dsl)|untyped effect|selectors?|conditions?|effect formulas|table rows' docs/ENGINE-GAME-DATA-BOUNDARY.md`
  passed and confirmed the typed-registry pattern plus no-untyped-effect-row
  boundary.
- `grep -niE 'event-card dispatch|faction eligibility|operation and special-activity|propaganda|persistent event expiry|faction bot priorities|not scaffolding|stays behavioral' docs/MECHANICAL-SCAFFOLDING-REGISTER.md`
  passed, confirming the behavior-vs-scaffolding anti-examples.
- `test -f templates/GAME-EVENT-COVERAGE.md` passed, and focused template greps
  confirmed the event matrix is evidence-only with unknown-field/no-selector
  guardrails.
- `grep -nE '^Status: Accepted' docs/adr/0011-*.md` passed (`Status:
  Accepted`).
- `node scripts/check-doc-links.mjs` passed (`Checked 34 markdown files`).
- `bash scripts/boundary-check.sh` passed (`engine-core boundary check passed`;
  `game-test-support dev-only boundary check passed`).

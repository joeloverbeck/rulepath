# PLP1RDY-002: Author and accept ADR 0011 — Constrained Typed Rust Event-Card Mechanism

**Status**: COMPLETED
**Priority**: HIGH
**Effort**: Medium
**Engine Changes**: None — new governance doc (`docs/adr/0011-constrained-typed-rust-event-card-mechanism.md`)
**Deps**: None

## Problem

A COIN-scale private game needs an event-card mechanism, but FOUNDATIONS §5
forbids data-driven rule behavior and bans YAML/DSL without an ADR (§13 trigger:
"introducing selectors, expressions, rule-like data, or DSL work"). The spec
(WB-1b) requires an **accepted** ADR authorizing a *constrained typed Rust*
event-card mechanism — typed static content for card identity/deck order/inert
display metadata, but every condition/selector/trigger/effect implemented as
Rust behavior — before the ENGINE-GAME-DATA-BOUNDARY typed-registry section
(PLP1RDY-006) lands. This ticket authors and accepts that ADR.

## Assumption Reassessment (2026-06-28)

1. ADR template + numbering: `docs/adr/ADR-TEMPLATE.md` exists; `0011` is the
   next integer after `0010` (PLP1RDY-001) — this ticket has no `Deps` because
   the three ADRs are authored independently and only the *integer* ordering is
   shared; if 001 has not yet created `0010`, `0011` is still correct and stable.
2. Verbatim Decision is `archive/specs/private-lane-foundation-readiness.md` §5.2; the
   spec confines the mechanism to the private crate ("game-local/private until
   public-safe evidence justifies any public helper").
3. Cross-artifact boundary under audit: the ADR amends FOUNDATIONS §5
   static-data/no-DSL section and the `docs/ENGINE-GAME-DATA-BOUNDARY.md`
   typed-content/behavior line (consumed by PLP1RDY-006), plus
   `docs/MECHANIC-ATLAS.md` private event-pressure notes. Confirmed those docs
   exist and currently carry the §5 typed-content-not-behavior contract.
4. FOUNDATIONS principle under audit (§5 static data is typed content, not
   behavior / §13 rule-like-data trigger): the ADR must explicitly ban YAML,
   script, untyped JSON/TOML effect rows, and any declarative behavior language,
   keeping selectors/conditions/triggers/overrides as Rust functions/match
   arms/traits. It does not introduce a DSL — it forecloses one — so it does not
   cross the §12 "DSL without ADR" stop; it is the ADR that authorizes the typed
   pattern.

## Architecture Check

1. A typed Rust registry + match/trait effects is the §5-clean realization: it
   keeps Rust the behavior authority (§2) while permitting typed inert content.
   The alternative (untyped effect rows) would cross the §12 procedural-static-data
   stop — the ADR records why it is rejected.
2. No backwards-compatibility shim: the ADR authorizes a forward pattern; no
   legacy data format is aliased.
3. `engine-core` stays noun-free (§3): the mechanism is game-local in the private
   crate; `game-stdlib` promotion (§4) is deferred until public-safe evidence.

## Verification Layers

1. ADR exists with correct ID/status -> codebase grep-proof (`test -f` + `grep
   '^Status: Accepted'`).
2. Decision matches verbatim §5.2 -> manual review against the spec.
3. No-DSL / typed-content boundary preserved -> FOUNDATIONS alignment check (§5,
   §11 "No YAML or DSL appears without ADR", §13 rule-like-data trigger).

## What to Change

### 1. Author `docs/adr/0011-constrained-typed-rust-event-card-mechanism.md`

From `docs/adr/ADR-TEMPLATE.md`, full section set with all impact sections,
`Status: Accepted`, date 2026-06-28, ID `0011`. Decision text is the verbatim
block from spec §5.2. The ADR names the amended FOUNDATIONS §5,
ENGINE-GAME-DATA-BOUNDARY, and MECHANIC-ATLAS lines and flags the constitution
supersession; the Data/Rust-boundary impact section explicitly bans YAML, script,
untyped JSON/TOML/RON effect rows, and declarative behavior languages.

### 2. Acceptance

Record `Status: Accepted` (Assumption A3). PLP1RDY-006 gates on this status.

## Files to Touch

- `docs/adr/0011-constrained-typed-rust-event-card-mechanism.md` (new)

## Out of Scope

- The ENGINE-GAME-DATA-BOUNDARY typed-registry doc section (PLP1RDY-006).
- Any private game crate, registry code, or effect trait (private repo, later).
- ADR 0010 and ADR 0012.

## Acceptance Criteria

### Tests That Must Pass

1. `test -f docs/adr/0011-constrained-typed-rust-event-card-mechanism.md && grep -q '^Status: Accepted' docs/adr/0011-constrained-typed-rust-event-card-mechanism.md`
2. `grep -qiE 'no (yaml|dsl)|bans? .*(yaml|dsl)' docs/adr/0011-constrained-typed-rust-event-card-mechanism.md` — the no-DSL ban is explicit.
3. `node scripts/check-doc-links.mjs` — no broken links introduced.

### Invariants

1. The ADR keeps Rust the behavior authority (§2) and bans declarative/DSL
   behavior data (§5).
2. The mechanism stays game-local/private until public-safe evidence (§4).

## Test Plan

### New/Modified Tests

1. `None — governance/ADR doc; verification is command-based (doc-link gate + status/ban grep) and existing pipeline coverage is named in Assumption Reassessment.`

### Commands

1. `grep -nE '^Status: Accepted' docs/adr/0011-constrained-typed-rust-event-card-mechanism.md`
2. `node scripts/check-doc-links.mjs`
3. A narrower command suffices: no code ships, so doc-link integrity + status/no-DSL-ban greps are the correct verification boundary.

## Outcome

Completed: 2026-06-28

Added `docs/adr/0011-constrained-typed-rust-event-card-mechanism.md` as an
accepted ADR using the repository ADR template. The ADR authorizes only a
game-local private typed Rust event-card mechanism: card identity, deck order,
inert display metadata, and non-behavioral parameters may be typed static
content, while every condition, selector, trigger, override, target choice,
legality check, transition, visibility decision, diagnostic, and semantic
effect remains Rust behavior. The ADR explicitly bans YAML, scripts, untyped
JSON/TOML/RON effect rows, and declarative behavior languages, and it defers
any public helper to later public-safe mechanic-atlas evidence.

Deviations from plan: none. No private game crate, Rust source, web, CI,
catalog, fixture, trace, replay, hash, RNG, or benchmark file changed.

Verification:

- `grep -nE '^Status: Accepted' docs/adr/0011-constrained-typed-rust-event-card-mechanism.md`
  passed (`3:Status: Accepted`).
- `grep -niE 'no (yaml|dsl)|bans? .*(yaml|dsl)|YAML|DSL' docs/adr/0011-constrained-typed-rust-event-card-mechanism.md`
  passed and confirmed the no-YAML/no-DSL boundary is explicit.
- `node scripts/check-doc-links.mjs` passed (`Checked 33 markdown files`).

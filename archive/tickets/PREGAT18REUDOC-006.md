# PREGAT18REUDOC-006: Author docs/MECHANICAL-SCAFFOLDING-REGISTER.md

**Status**: COMPLETED
**Priority**: HIGH
**Effort**: Medium
**Engine Changes**: None — docs-only (new `docs/MECHANICAL-SCAFFOLDING-REGISTER.md`, `docs/README.md`)
**Deps**: 004

## Problem

ADR 0008 authorizes a mechanical-scaffolding reuse lane but needs a decision register — the scaffolding analogue of the behavioral primitive-pressure ledger — to record per-entry classifications and, critically, the explicit non-promotion list that keeps behavioral mechanics from being relabeled as plumbing. Without it, the lane has no auditable home.

## Assumption Reassessment (2026-06-22)

1. Verified ADR 0008 (ticket 004) defines this register's governance and names it as an allowed-home/evidence surface; the register is governed by ADR 0008. Hence `Deps: 004` and the acceptance precondition.
2. Verified `docs/MECHANICAL-SCAFFOLDING-REGISTER.md` does not yet exist (confirmed absent via `/reassess-spec` this session; spec D3 / WB5).
3. Cross-artifact boundary under audit: the register's authority-map line lands in `docs/README.md` (shared file with tickets 001 and 007; mechanical merge, sequential).
4. FOUNDATIONS §4 motivates this: the register is the scaffolding decision ledger *parallel* to the behavioral primitive-pressure ledger. Restating the invariant — scaffolding stays behavior-free; the explicit non-promotion list keeps deal / reveal / projection / betting / pot / trick-lifecycle / teams / graph / accounting / reaction / scoring **behavioral**, never scaffolding.
5. Touches the §4 third-use hard-gate boundary: confirm the register does not relabel behavior as plumbing (the non-promotion list enforces this) and introduces no leak or nondeterminism path (it is a decision ledger, not code).

## Architecture Check

1. A dedicated register governed by ADR 0008 cleanly separates scaffolding decisions from the behavioral primitive-pressure ledger, instead of overloading the atlas ledger with non-behavioral rows.
2. No backwards-compatibility shims; the register records decisions, it extracts nothing (Part C unit owns code).
3. `engine-core` stays noun-free (§3); the non-promotion list preserves the behavioral earning rule (§4).

## Verification Layers

1. Register exists with per-entry fields (semantic-risk classification, production-vs-test home, affected hashes, visibility impact, exact duplicate sites, migration set, rejection rationale) and the explicit non-promotion list -> codebase grep-proof.
2. Register is governed by an `Accepted` ADR 0008 -> grep (`^Status: Accepted` on `docs/adr/0008-*.md`) — acceptance precondition.
3. The README authority-map line is added and all links resolve -> `node scripts/check-doc-links.mjs`.
4. The non-promotion list keeps the named mechanic shapes behavioral -> FOUNDATIONS alignment check (§4) + grep that each listed shape is marked "stays behavioral".

## What to Change

### 1. Author the register

Create `docs/MECHANICAL-SCAFFOLDING-REGISTER.md` with the per-entry field set above and the explicit non-promotion list (deal/reveal/projection/betting/pot/trick-lifecycle/teams/graph/accounting/reaction/scoring stay behavioral).

### 2. Index it

Add the register's authority-map line to `docs/README.md` at its correct layer.

## Files to Touch

- `docs/MECHANICAL-SCAFFOLDING-REGISTER.md` (new)
- `docs/README.md` (modify; authority map structured by ticket 001)

## Out of Scope

- Any actual scaffolding extraction or register entry for in-flight code (Part C successor unit).
- The `FOUNDATIONS.md` §4 lane definition (ticket 008).

## Acceptance Criteria

### Tests That Must Pass

1. `test -f docs/MECHANICAL-SCAFFOLDING-REGISTER.md` and it carries the field set + non-promotion list.
2. `grep -nE "stays behavioral|non-promotion" docs/MECHANICAL-SCAFFOLDING-REGISTER.md` returns the non-promotion list.
3. `node scripts/check-doc-links.mjs` passes; `grep -n "MECHANICAL-SCAFFOLDING-REGISTER" docs/README.md` returns the authority-map line.

### Invariants

1. The register governs scaffolding only; no behavioral mechanic shape is reclassified as plumbing.
2. The register is governed by an accepted ADR 0008.

## Test Plan

### New/Modified Tests

1. `None — documentation-only ticket; verification is command-based (presence, non-promotion-list grep, link check) and the §4 boundary is a FOUNDATIONS alignment check named in Assumption Reassessment.`

### Commands

1. `grep -niE "non-promotion|stays behavioral|semantic.risk|migration set" docs/MECHANICAL-SCAFFOLDING-REGISTER.md`
2. `node scripts/check-doc-links.mjs`
3. The grep + link check is the correct boundary; the register is a docs deliverable with no code surface.

## Outcome

Completed: 2026-06-22

Created `docs/MECHANICAL-SCAFFOLDING-REGISTER.md` as the ADR 0008-governed
decision register for behavior-free mechanical scaffolding. The register defines
required per-entry fields including `semantic.risk`, production-vs-test home,
affected hashes, visibility impact, duplicate sites, migration set, acceptance
evidence, rejection rationale, and next review trigger. It also records an
explicit non-promotion list that keeps deal/reveal/projection/betting/pot/trick
lifecycle/teams/graph/accounting/reaction/scoring shapes behavioral.

Updated `docs/README.md` to add the register to the ordered authority map.

Deviations: while editing the same authority surface, updated the ADR status
index to include ADR 0008 and ADR 0009, both already accepted by prior tickets;
leaving them absent would have made the index stale.

Verification:

- `grep -niE "non-promotion|stays behavioral|semantic.risk|migration set" docs/MECHANICAL-SCAFFOLDING-REGISTER.md`
  returned the entry fields and non-promotion list.
- `grep -n "MECHANICAL-SCAFFOLDING-REGISTER" docs/README.md` returned the
  authority-map line.
- `grep -nE "^Status: Accepted" docs/adr/0008-mechanical-scaffolding-governance.md`
  confirmed the governing ADR is accepted.
- `node scripts/check-doc-links.mjs` passed (`Checked 30 markdown files`).

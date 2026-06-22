# PREGAT18REUDOC-004: Author + accept ADR 0008 — Mechanical Scaffolding Governance

**Status**: PENDING
**Priority**: HIGH
**Effort**: Large
**Engine Changes**: None — docs-only (new `docs/adr/0008-mechanical-scaffolding-governance.md`)
**Deps**: 002

## Problem

The 17-game corpus repeatedly rebuilds behavior-free plumbing around the generic kernel (effect-envelope constructors, seat-ID parse/format, action-tree encoding), but the mechanic-atlas third-use gate was designed to govern *behavioral* mechanics, not plumbing. There is no lawful reuse lane for typed, behavior-free infrastructure. ADR 0008 must define that lane — without lowering the behavioral hard gate — so the foundation/area-doc amendments (tickets 008/009/014/016) and the eventual Part C code extraction have an accepted authority to cite.

## Assumption Reassessment (2026-06-22)

1. Verified `docs/FOUNDATIONS.md` §4 carries only the behavioral third-use hard gate (lines 83–89) with no "mechanical scaffolding" lane, and §11/§12 carry no scaffolding mention (confirmed via `/reassess-spec` this session; spec §Assumptions A9).
2. Verified the exact amended-section targets exist: `ENGINE-GAME-DATA-BOUNDARY.md` §13 (game-stdlib promotion boundary), `MECHANIC-ATLAS.md` §§4–8 (behavioral gate), `UI-INTERACTION.md` §10A, and `ARCHITECTURE.md` ownership table — per spec D1 / WB3.
3. Cross-artifact boundary under audit: ADR 0008 names exact amended sections across five docs (`FOUNDATIONS` §4/§11/§12, `ENGINE-GAME-DATA-BOUNDARY` §13, `MECHANIC-ATLAS` §§4–8, `ARCHITECTURE`, `UI-INTERACTION` §10A); those edits land in tickets 008/009 (and the UI/discipline portions of 014/016), each gated on this ADR's acceptance.
4. FOUNDATIONS §4 + §13 motivate this ADR: adding a reuse category beside the behavioral third-use gate amends §4 and trips the §13 ADR triggers ("promoting mechanics outside the normal primitive-pressure path", "changing `engine-core` vocabulary/responsibilities"). Restating the invariant: the behavioral third-use hard gate stays **word-for-word effective**; the new lane targets only behavior-free plumbing on nouns already in §3's allowed kernel vocabulary.
5. Touches the §4 third-use hard gate and the §11/§12 invariants: confirm the ADR does not weaken the behavioral gate (it adds a parallel lane and preserves the gate verbatim) and that the scaffolding category is behavior-neutral, deterministic, and leak-safe by its own decision rule (no §11 no-leak or determinism path is opened).

## Architecture Check

1. An **accepted ADR** (not an editorial doc edit) is the only lawful way to amend §4/§11/§12 (FOUNDATIONS L3: "supersede only by accepted ADR"); ADR-first is cleaner and law-compliant versus quietly editing the constitution.
2. No backwards-compatibility shims; the ADR authorizes the lane but implements no extraction (Part C successor unit owns code).
3. `engine-core` stays free of mechanic nouns — the lane explicitly *excludes* mechanic nouns (§3); the `game-stdlib` behavioral earning rule (§4) is preserved unchanged.

## Verification Layers

1. ADR 0008 exists, built from the revised `ADR-TEMPLATE.md`, and names all five amended sections -> codebase grep-proof.
2. The behavioral third-use wording is preserved word-for-word -> manual diff of the ADR's quoted gate against `FOUNDATIONS.md` §4.
3. ADR reaches `Status: Accepted` before any gated downstream ticket lands -> grep (`^Status: Accepted`) — acceptance precondition; **human sign-off pause**.
4. ADR links resolve -> `node scripts/check-doc-links.mjs`.
5. Scaffolding category is behavior-neutral / leak-safe / deterministic (no §11 violation) -> FOUNDATIONS alignment check against §4/§11/§12.

## What to Change

### 1. Author the ADR

Create `docs/adr/0008-mechanical-scaffolding-governance.md` (built from the revised template) defining: the mechanical-scaffolding category and its exclusions; allowed homes (`engine-core` contract ergonomics, `game-stdlib`, a future dev-only `game-test-support`, `wasm-api` adapters); evidence fields; the category decision rule (review at second exact duplication; hard decision before a third copy; second-use promotion only when semantic identity is proven AND the API is noun-free/game-layer-typed, behavior-neutral, deterministic, leak-safe, and migration-complete); the mechanic-atlas interlock; and the exact amended foundation sections. Author with `Status: Proposed`.

### 2. Acceptance (human pause)

A maintainer reviews and flips `Status` to `Accepted` (Phase-0 precedent: the realignment spec authored ADR 0007, a maintainer accepted it). Downstream tickets do not land until accepted.

## Files to Touch

- `docs/adr/0008-mechanical-scaffolding-governance.md` (new)

## Out of Scope

- The foundation/area-doc edits the ADR gates (tickets 008/009/014/016).
- Implementing any scaffolding extraction, new crate, or harness (Part C successor unit).
- Weakening, rewording, or relaxing the behavioral third-use hard gate.

## Acceptance Criteria

### Tests That Must Pass

1. `test -f docs/adr/0008-mechanical-scaffolding-governance.md` and it names all five amended sections.
2. `grep -nE "^Status: Accepted" docs/adr/0008-mechanical-scaffolding-governance.md` (after the human acceptance step).
3. `node scripts/check-doc-links.mjs` passes.

### Invariants

1. The behavioral third-use hard gate (§4) remains word-for-word effective.
2. No mechanic noun enters `engine-core`; the lane targets only already-allowed kernel vocabulary (§3).

## Test Plan

### New/Modified Tests

1. `None — documentation-only ticket; verification is command-based (ADR presence, Status grep, link check) and the behavioral-gate preservation is a manual diff named in Assumption Reassessment.`

### Commands

1. `grep -nE "^Status:|FOUNDATIONS|§4|§11|§12|§13|§10A" docs/adr/0008-mechanical-scaffolding-governance.md`
2. `node scripts/check-doc-links.mjs`
3. The grep + manual `FOUNDATIONS.md` §4 diff is the correct boundary; there is no code surface to test.

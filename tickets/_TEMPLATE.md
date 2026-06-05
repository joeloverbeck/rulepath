# <PREFIX-NNN>: <Ticket title>

**Status**: PENDING
**Priority**: <LOW|MEDIUM|HIGH>
**Effort**: <Small|Medium|Large>
**Engine Changes**: <None|Yes — list affected or newly-introduced crates/modules (`engine-core`, `game-stdlib`, `games/*`), schemas, traces, docs, skills, or code surfaces>
**Deps**: <ticket/spec dependencies that currently exist>

## Problem

<What user-facing or architecture problem this solves>

## Assumption Reassessment (<YYYY-MM-DD>)

<!-- Items 1-3 always required. Items 4+ are a menu; include only those matching this ticket's scope and renumber surviving items sequentially starting from 4. Lists like 1, 2, 3, 14 are malformed output. -->

1. <Assumption checked against current code (`engine-core` / `game-stdlib` / `games/*`) and skills, with exact file/symbol references>
2. <Assumption checked against current specs/docs, with exact file reference>
3. <If this is a cross-crate or cross-artifact ticket: name the exact shared boundary, contract, or schema under audit before implementation>
4. <If a FOUNDATIONS principle or §11 acceptance invariant motivates this ticket: restate the intended principle/invariant before trusting the spec narrative>
5. <If this ticket touches the third-use mechanic hard gate (§4), fail-closed acceptance invariants (§11), or deterministic replay/hash & serialization surfaces: name the exact enforcement surface and confirm the change does not leak hidden information (§11 no-leak firewall) or break deterministic replay/hash (§11/§13)>
6. <If this ticket extends an existing schema or contract (action tree, command/effect envelope, public/private view, golden trace, checkpoint, serialized save, static-data manifest entry): name the schema, the consumers of that schema, and whether the extension is additive-only or breaking>
7. <If this ticket renames or removes a public symbol, mechanic, acceptance invariant, doc-governed contract, or schema field: grep repo-wide (the code tree — `engine-core` / `game-stdlib` / `games/*` —, `docs/`, `specs/`, `templates/`, `.claude/skills/`) and cite the blast radius per area>
8. <If reassessment exposes adjacent contradictions: classify them as required consequences of this ticket, separate bugs, or future cleanup that must become its own ticket>
9. <Mismatch + correction (if any)>

## Architecture Check

1. <Why this approach is cleaner/more robust than alternatives>
2. <No backwards-compatibility aliasing/shims introduced>
3. <Confirm `engine-core` stays free of mechanic nouns (§3) and `game-stdlib` changes are earned via the mechanic atlas (§4)>

## Verification Layers

1. <Invariant> -> <codebase grep-proof | schema/serialization validation | golden trace / deterministic replay-hash check | no-leak visibility test | bot legality check | simulation/CLI run | benchmark check | FOUNDATIONS alignment check | manual review>
2. <Invariant> -> <verification surface>
3. <If cross-crate or cross-artifact: map each distinct invariant to its own distinct proof surface; do not collapse>
4. <If single-layer ticket, state why additional layer mapping is not applicable>

## What to Change

### 1. <Change area>

<Details>

### 2. <Change area>

<Details>

## Files to Touch

- `<path>` (<new|modify>)

## Out of Scope

- <explicit non-goals>

## Acceptance Criteria

### Tests That Must Pass

1. <specific behavior test or validation command>
2. <specific behavior test or validation command>
3. <full-pipeline verification command>

### Invariants

1. <must-always-hold architectural invariant>
2. <must-always-hold data-contract invariant>

## Test Plan

### New/Modified Tests

1. `<path/to/test-or-trace>` — <short rationale>
2. `<path/to/test-or-trace>` — <short rationale>
3. `None — documentation-only ticket; verification is command-based and existing pipeline/test coverage is named in Assumption Reassessment.` <use this instead when no tests change>

### Commands

1. `<targeted verification command>`
2. `<full-pipeline verification command (e.g. cargo test / cargo bench / simulation CLI)>`
3. `<explain why a narrower command is the correct verification boundary, if applicable>`

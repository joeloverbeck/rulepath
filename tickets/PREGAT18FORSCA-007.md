# PREGAT18FORSCA-007: REGISTER first-use-safe candidate + forward cadence sections

**Status**: PENDING
**Priority**: MEDIUM
**Effort**: Medium
**Engine Changes**: None — governance/area-doc edit (`docs/MECHANICAL-SCAFFOLDING-REGISTER.md`)
**Deps**: PREGAT18FORSCA-002

## Problem

The mechanical-scaffolding register records 8C candidates and retrofit evidence, but has no standing forward per-game cadence and no automatic prior-game refactor trigger, and its `candidate` decision state is not described as first-use-safe. This ticket broadens the `candidate` state, adds the Forward Per-Game Maintenance Cadence and Automatic Prior-Game Refactor Trigger sections, updates the Current Entries intro, and extends the Review Checklist — the register sections that the contract, agent law, and templates cross-reference.

## Assumption Reassessment (2026-06-25)

1. `docs/MECHANICAL-SCAFFOLDING-REGISTER.md` carries the `candidate` decision state (Decision States table, L45), "Non-Promotion List" (L52), "Current Entries" (L76, citing 8C `MSC-8C-001` / `UNI8CMECSCA-005/006` receipts), and "Review Checklist" (L978). No "Forward Per-Game Maintenance Cadence" or "Automatic Prior-Game Refactor Trigger" section exists yet. Verified this session.
2. The spec (D7, plan §5.7) requires: a first-use-safe `candidate` definition; the Forward Per-Game Maintenance Cadence + Automatic Prior-Game Refactor Trigger sections after the Non-Promotion List and before Current Entries; a Current Entries intro paragraph; Review Checklist additions.
3. Shared contract under audit: the register's section spine and decision-state vocabulary (`candidate` / `local-only` / `rejected` / `deferred` / `accepted`) that PREGAT18FORSCA-017's receipt and PREGAT18FORSCA-018's checker consume.
4. FOUNDATIONS §11 (mechanical-scaffolding invariant L204–205, promotion-adoption + promotion-debt invariants) under audit: the cadence/trigger sections add observability of the queue-or-dispose duty without changing the existing pre-third-copy or promotion-debt blocking rules.

## Architecture Check

1. Placing the cadence/trigger sections between the Non-Promotion List and Current Entries keeps the normative rules above the data; broadening `candidate` to first-use-safe is the minimal change that lets first use register without implying promotion.
2. No backwards-compatibility shim — existing entries and decision states are preserved; the additions are normative sections and a clarified definition.
3. `engine-core`/`game-stdlib` discipline untouched; the register stays the home for behavior-free scaffolding only, redirecting behavior-bearing shapes to the mechanic atlas.

## Verification Layers

1. Section additions → grep-proof the two new sections sit between the Non-Promotion List and Current Entries.
2. Decision-state consistency → grep-proof `candidate`/`local-only`/`rejected`/`deferred` are defined consistently with the receipt schema PREGAT18FORSCA-017 will consume.
3. Doc-link integrity → `node scripts/check-doc-links.mjs`.

## What to Change

### 1. First-use-safe `candidate`

Broaden the `candidate` decision-state definition so a first-use behavior-free shape registers as `candidate` without authorizing promotion.

### 2. Forward Per-Game Maintenance Cadence + Automatic Prior-Game Refactor Trigger

Add both sections (plan §5.7 draft) after the Non-Promotion List, before Current Entries: the pre-/post-implementation checkpoints and the queue-or-dispose trigger (named tracker unit or accepted `local-only`/`deferred`/`rejected` disposition with rationale, owner, evidence, next review trigger).

### 3. Current Entries intro + Review Checklist

Add the Current Entries intro paragraph and extend the Review Checklist with the forward-cadence checks.

## Files to Touch

- `docs/MECHANICAL-SCAFFOLDING-REGISTER.md` (modify)

## Out of Scope

- Adding or migrating any actual register *entry* for a future game (Spades/Gate 18 is not pre-audited here).
- Changing the existing 8C entries, the Non-Promotion List, or the pre-third-copy threshold.
- Authoring the receipt JSON (PREGAT18FORSCA-017) or the checker (PREGAT18FORSCA-018).

## Acceptance Criteria

### Tests That Must Pass

1. `grep -n "Forward Per-Game Maintenance Cadence" docs/MECHANICAL-SCAFFOLDING-REGISTER.md` and `grep -n "Automatic Prior-Game Refactor Trigger"` confirm both sections before Current Entries.
2. The `candidate` state reads first-use-safe (grep-proof).
3. `node scripts/check-doc-links.mjs` passes.

### Invariants

1. The new sections are normative additions; existing entries and the Non-Promotion List are unchanged.
2. Decision-state vocabulary stays consistent with the receipt schema consumers.

## Test Plan

### New/Modified Tests

1. `None — documentation-only ticket; verification is command-based and existing pipeline/test coverage is named in Assumption Reassessment.`

### Commands

1. `node scripts/check-doc-links.mjs`
2. `git diff -- docs/MECHANICAL-SCAFFOLDING-REGISTER.md` (review: two new sections + candidate clarification + checklist)
3. `grep -n "first use" docs/MECHANICAL-SCAFFOLDING-REGISTER.md`

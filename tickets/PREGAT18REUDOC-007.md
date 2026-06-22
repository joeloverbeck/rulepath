# PREGAT18REUDOC-007: Author docs/EVIDENCE-FIXTURE-CONTRACT.md

**Status**: PENDING
**Priority**: HIGH
**Effort**: Medium
**Engine Changes**: None — docs-only (new `docs/EVIDENCE-FIXTURE-CONTRACT.md`, `docs/README.md`)
**Deps**: 005

## Problem

Setup and domain evidence fixtures, viewer-scoped exports, and internal replay traces all currently live as undifferentiated `*.trace.json` artifacts. ADR 0009 decides the taxonomy; this ticket gives it a concrete contract doc with named profiles, validator ownership, and visibility classification, so authors and validators stop treating heterogeneous artifacts as one schema.

## Assumption Reassessment (2026-06-22)

1. Verified ADR 0009 (ticket 005) governs this fixture contract and defines the artifact/visibility classes it instantiates. Hence `Deps: 005` and the acceptance precondition.
2. Verified `docs/EVIDENCE-FIXTURE-CONTRACT.md` does not yet exist, and `docs/TRACE-SCHEMA-v1.md` currently conflates trace + fixture schemas (confirmed via `/reassess-spec` this session; spec D4 / WB5).
3. Cross-artifact boundary under audit: the contract's authority-map line lands in `docs/README.md` (shared with tickets 001/006); the contract is the doc that ticket 010's TRACE-SCHEMA narrowing points at.
4. FOUNDATIONS §11 (no-leak firewall + determinism) is the enforcement surface: the `public-export-v1` and `seat-private-export-v1` profiles classify visibility — confirm leak-safe classification, that allowed private data is **test-only** (never shipped to a browser), and that naming these profiles changes no fixture byte this pass.
5. Relates to ADR 0004's taxonomy and the TRACE-SCHEMA v1 schema: the named profiles formalize the existing artifact classes additively; no existing fixture or golden hash is altered, and the filename suffix is declared non-authoritative (the profile, not the suffix, classifies an artifact).

## Architecture Check

1. A named-profile contract (governed by ADR 0009) is cleaner than overloading `TRACE-SCHEMA-v1.md` with every artifact shape; it lets each validator own exactly its profile.
2. No backwards-compatibility shims; no fixture migration (Part C unit owns bytes).
3. `engine-core` (§3) / `game-stdlib` (§4) untouched; the contract is presentation-of-evidence law, not behavior.

## Verification Layers

1. Contract defines the five named profiles (`replay-command-v1`, `public-export-v1`, `seat-private-export-v1`, `setup-evidence-v1`, `domain-evidence-v1`) with validator ownership + version anchors + visibility classification -> codebase grep-proof.
2. Contract is governed by an `Accepted` ADR 0009 -> grep (`^Status: Accepted` on `docs/adr/0009-*.md`) — acceptance precondition.
3. No fixture byte changed -> `git diff --stat -- '**/*.trace.json'` is empty (FOUNDATIONS §11 determinism).
4. README authority-map line added + links resolve -> `node scripts/check-doc-links.mjs`.

## What to Change

### 1. Author the contract

Create `docs/EVIDENCE-FIXTURE-CONTRACT.md` with the five named profiles, per-profile validator ownership, visibility classification, version anchors, the allowed private test-only data rule, and the statement that the filename suffix is non-authoritative.

### 2. Index it

Add the contract's authority-map line to `docs/README.md` at its correct layer (subordinate to ADR 0004/0009).

## Files to Touch

- `docs/EVIDENCE-FIXTURE-CONTRACT.md` (new)
- `docs/README.md` (modify; authority map structured by ticket 001)

## Out of Scope

- Migrating/regenerating any `*.trace.json` fixture to a profile (Part C successor unit).
- Narrowing `TRACE-SCHEMA-v1.md` prose (ticket 010).

## Acceptance Criteria

### Tests That Must Pass

1. `grep -nE "replay-command-v1|public-export-v1|seat-private-export-v1|setup-evidence-v1|domain-evidence-v1" docs/EVIDENCE-FIXTURE-CONTRACT.md` returns all five profiles.
2. `grep -niE "filename suffix" docs/EVIDENCE-FIXTURE-CONTRACT.md` returns the non-authoritative statement.
3. `node scripts/check-doc-links.mjs` passes; `git diff --stat -- '**/*.trace.json'` is empty.

### Invariants

1. Visibility classification is leak-safe; allowed private data is test-only.
2. No fixture byte changes; the contract is decision-only this pass.

## Test Plan

### New/Modified Tests

1. `None — documentation-only ticket; verification is command-based (profile grep, no-byte-drift `git diff`, link check) named in Assumption Reassessment.`

### Commands

1. `grep -niE "v1|validator|visibility|version anchor" docs/EVIDENCE-FIXTURE-CONTRACT.md`
2. `node scripts/check-doc-links.mjs && git diff --stat -- '**/*.trace.json'`
3. The profile grep + no-byte-drift `git diff` is the correct boundary; the contract is decision-only with no code surface.

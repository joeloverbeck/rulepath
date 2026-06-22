# PREGAT18REUDOC-010: TESTING test-support law + fixture profiles + hash-migration protocol; narrow TRACE-SCHEMA-v1

**Status**: COMPLETED
**Priority**: HIGH
**Effort**: Medium
**Engine Changes**: None — docs-only (`docs/TESTING-REPLAY-BENCHMARKING.md`, `docs/TRACE-SCHEMA-v1.md`)
**Deps**: 005, 007

## Problem

With ADR 0009 accepted and the fixture contract authored, the testing doc needs a shared test-support law, named fixture profiles, and a hash-migration protocol, and `TRACE-SCHEMA-v1.md` (which today claims to be the schema for both traces *and* fixtures) must be narrowed to the command/replay contract (or marked superseded) and point at the new fixture contract — so the two stop conflating.

## Assumption Reassessment (2026-06-22)

1. Verified `docs/TESTING-REPLAY-BENCHMARKING.md` carries no "test-support law", "fixture profiles", or "hash-migration protocol" sections today (confirmed via `/reassess-spec` this session; spec §Assumptions A9). ADR 0009 (ticket 005) gates these; hence `Deps: 005` + acceptance precondition.
2. Verified `docs/TRACE-SCHEMA-v1.md` self-describes as the "canonical trace and replay fixture schema for Gate 2" — broader than a command/replay contract, no superseded marker. This ticket narrows it and points it at `docs/EVIDENCE-FIXTURE-CONTRACT.md` (ticket 007); hence `Deps: 007`.
3. Cross-artifact boundary under audit: the TESTING fixture profiles reference the named profiles defined in ticket 007; `TESTING-REPLAY-BENCHMARKING.md` is also touched by ticket 003 (ADR 0005 references) — independent regions, mechanical merge (flagged as a shared-file overlap).
4. FOUNDATIONS §11 determinism motivates the hash-migration protocol: restating the invariant — replay/hashes/serialization/RNG/traces stay deterministic or are *explicitly migrated*; the protocol documents the migration discipline without changing any byte this pass.
5. Touches the §11 deterministic-replay/hash surface: confirm `TRACE-SCHEMA-v1.md` **field bytes are unchanged** (prose narrowed/annotated only) and no fixture byte changes (the spec's "no bytes/contract drift" exit criterion).

## Architecture Check

1. A shared test-support law + named fixture profiles + an explicit hash-migration protocol, with TRACE-SCHEMA narrowed to its real (command/replay) role, cleanly separates the trace and fixture schemas instead of leaving one doc claiming both.
2. No backwards-compatibility shims; no fixture/hash migration (Part C unit).
3. `engine-core` (§3) / `game-stdlib` (§4) untouched.

## Verification Layers

1. `TESTING-REPLAY-BENCHMARKING.md` carries the test-support law + fixture profiles + hash-migration protocol -> codebase grep-proof.
2. `TRACE-SCHEMA-v1.md` narrowed to the command/replay contract (or superseded marker) and points at `EVIDENCE-FIXTURE-CONTRACT.md` -> grep.
3. `TRACE-SCHEMA-v1.md` field bytes unchanged (prose-only edit) -> manual diff; `git diff` shows only prose lines.
4. ADR 0009 `Accepted` precondition -> grep (`^Status: Accepted` on `docs/adr/0009-*.md`).
5. No fixture byte changed -> `git diff --stat -- '**/*.trace.json'` empty.
6. Links resolve -> `node scripts/check-doc-links.mjs`.

## What to Change

### 1. TESTING-REPLAY-BENCHMARKING.md

Add the shared test-support law, the named fixture profiles (cross-referencing `EVIDENCE-FIXTURE-CONTRACT.md`), and the hash-migration protocol.

### 2. TRACE-SCHEMA-v1.md

Narrow its scope statement to the command/replay contract (or mark it superseded by ADR 0009), and point at `EVIDENCE-FIXTURE-CONTRACT.md` for fixture profiles. Prose only — no field-byte change.

## Files to Touch

- `docs/TESTING-REPLAY-BENCHMARKING.md` (modify)
- `docs/TRACE-SCHEMA-v1.md` (modify)

## Out of Scope

- The WASM canonical seat grammar + alias policy (ticket 011).
- Any `*.trace.json` fixture / hash / `TRACE-SCHEMA-v1.md` field-byte migration (Part C successor unit).

## Acceptance Criteria

### Tests That Must Pass

1. `grep -niE "test-support|fixture profile|hash.migration" docs/TESTING-REPLAY-BENCHMARKING.md` returns the new sections.
2. `grep -niE "command/replay|superseded|EVIDENCE-FIXTURE-CONTRACT" docs/TRACE-SCHEMA-v1.md` returns the narrowing + pointer.
3. `node scripts/check-doc-links.mjs` passes; `git diff --stat -- '**/*.trace.json'` empty.

### Invariants

1. `TRACE-SCHEMA-v1.md` field bytes are unchanged (prose-only narrowing).
2. No fixture/hash byte migration occurs in this pass.

## Test Plan

### New/Modified Tests

1. `None — documentation-only ticket; verification is command-based (grep, no-byte-drift `git diff`, link check) named in Assumption Reassessment.`

### Commands

1. `grep -niE "test-support|fixture profile|hash.migration|superseded" docs/TESTING-REPLAY-BENCHMARKING.md docs/TRACE-SCHEMA-v1.md`
2. `node scripts/check-doc-links.mjs && git diff --stat -- '**/*.trace.json'`
3. The grep + no-byte-drift `git diff` is the correct boundary: prose narrowing without trace/hash migration.

## Outcome

Completed: 2026-06-22

Updated `docs/TESTING-REPLAY-BENCHMARKING.md` with shared test-support law,
named fixture profiles from `docs/EVIDENCE-FIXTURE-CONTRACT.md`, and a
hash-migration protocol that requires named surfaces, version anchors, migration
notes, owning validators, and no bulk trace/hash regeneration.

Narrowed `docs/TRACE-SCHEMA-v1.md` to legacy command/replay trace evidence and
pointed setup/domain evidence fixtures plus public/seat-private exports to
`docs/EVIDENCE-FIXTURE-CONTRACT.md` under ADR 0009. The root field table and
schema field names were not changed.

Deviations: none.

Verification:

- `grep -niE "test-support|fixture profile|hash.migration" docs/TESTING-REPLAY-BENCHMARKING.md`
  returned the new sections.
- `grep -niE "command/replay|superseded|EVIDENCE-FIXTURE-CONTRACT" docs/TRACE-SCHEMA-v1.md`
  returned the narrowed status/scope and fixture-contract pointer.
- `grep -nE "^Status: Accepted" docs/adr/0009-replay-fixture-hash-taxonomy.md`
  confirmed the governing ADR is accepted.
- `node scripts/check-doc-links.mjs` passed (`Checked 31 markdown files`).
- `git diff --stat -- '**/*.trace.json'` was empty.
- Manual diff review confirmed `TRACE-SCHEMA-v1.md` changes were prose-only
  before the existing field table.

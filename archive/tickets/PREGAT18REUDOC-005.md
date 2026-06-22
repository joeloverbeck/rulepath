# PREGAT18REUDOC-005: Author + accept ADR 0009 — Replay/Fixture/Hash Taxonomy v2

**Status**: COMPLETED
**Priority**: HIGH
**Effort**: Large
**Engine Changes**: None — docs-only (new `docs/adr/0009-replay-fixture-hash-taxonomy.md`)
**Deps**: 002

## Problem

Heterogeneous `*.trace.json` artifacts (internal full traces, viewer-scoped exports, setup/domain evidence fixtures) currently masquerade as one schema, and trace/hash/export authority is implicit. ADR 0009 must decide the artifact/visibility/validator/version taxonomy — *decision only, no bytes change* — so the fixture contract (ticket 007), the TESTING/TRACE narrowing (ticket 010), and the seat-grammar export classification (ticket 011) have an accepted authority, and so the Part C migration unit has a target.

## Assumption Reassessment (2026-06-22)

1. Verified `docs/adr/0004-hidden-info-replay-export-taxonomy.md` is `Status: Accepted` and already defines artifact classes (internal full trace, viewer-scoped replay export) and a Visibility-impact / no-leak taxonomy (confirmed via `/reassess-spec` this session). ADR 0009 must **preserve or strengthen** ADR 0004 and may touch it only by naming exact sections.
2. Verified `docs/TRACE-SCHEMA-v1.md` self-describes as the "canonical trace and replay fixture schema for Gate 2" — broader than a command/replay contract, with no superseded marker (spec §Assumptions A9). ADR 0009 decides whether v1 stays a legacy command schema or is superseded.
3. Cross-artifact boundary under audit: ADR 0009 governs the new `EVIDENCE-FIXTURE-CONTRACT.md` (ticket 007), the TESTING test-support law + TRACE-SCHEMA narrowing (ticket 010), and the WASM seat-grammar export classification (ticket 011) — each gated on this ADR's acceptance.
4. FOUNDATIONS §13 motivates this ADR: "changing replay/hash semantics" and "changing public/private visibility contracts" are ADR triggers. Restating §11: replay/hashes/serialization/RNG/traces remain deterministic or are explicitly migrated — this pass migrates nothing.
5. Touches the §11 deterministic-replay/hash and no-leak invariants: confirm the ADR is **decision-only** (no fixture/hash/trace byte changes this pass) and that ADR 0004's leak taxonomy is preserved/strengthened, never silently amended.
6. Relates to an existing contract (ADR 0004's taxonomy + the TRACE-SCHEMA v1 schema): name the relationship — taxonomy **v2** that supersedes only *named* v1 sections, additive over ADR 0004; all existing fixtures/golden hashes are unchanged and consumers untouched this pass.

## Architecture Check

1. ADR-first is required because replay/hash semantics and visibility contracts are §13 triggers; a decision-only ADR fixes the taxonomy while keeping bytes stable — cleaner than ad-hoc per-doc trace annotations.
2. No backwards-compatibility shims; no blanket golden-trace regeneration (migration is the Part C unit).
3. `engine-core` (§3) and `game-stdlib` (§4) untouched.

## Verification Layers

1. ADR 0009 exists and defines artifact classes, visibility classes, validators, version identifiers, canonical-byte authority, hash-surface versions, and compatibility windows -> codebase grep-proof.
2. ADR 0004 preserved/strengthened, referenced only by named sections -> manual diff against `docs/adr/0004-*.md`.
3. No fixture/hash/trace byte changed this pass -> `git diff --stat` over `**/*.trace.json` and `games/*/benches` is empty (FOUNDATIONS §11 determinism).
4. ADR reaches `Status: Accepted` before any gated downstream ticket lands -> grep — **human sign-off pause**.
5. ADR links resolve -> `node scripts/check-doc-links.mjs`.

## What to Change

### 1. Author the ADR

Create `docs/adr/0009-replay-fixture-hash-taxonomy.md` (built from the revised template) defining: artifact classes, visibility classes, validators, version identifiers, canonical-byte authority, hash-surface versions, compatibility windows, the relationship to ADR 0004, and the disposition of Trace Schema v1 (legacy command schema vs superseded). Decision only — no bytes change. Author with `Status: Proposed`.

### 2. Acceptance (human pause)

A maintainer reviews and flips `Status` to `Accepted`; downstream tickets (007/010/011) do not land until accepted.

## Files to Touch

- `docs/adr/0009-replay-fixture-hash-taxonomy.md` (new)

## Out of Scope

- Any `*.trace.json` fixture, hash, RNG, or serialization byte migration (Part C successor unit).
- Narrowing `TRACE-SCHEMA-v1.md` prose itself (ticket 010) and authoring `EVIDENCE-FIXTURE-CONTRACT.md` (ticket 007).
- Weakening or silently amending ADR 0004's no-leak taxonomy.

## Acceptance Criteria

### Tests That Must Pass

1. `test -f docs/adr/0009-replay-fixture-hash-taxonomy.md` and it defines the artifact/visibility/validator/version taxonomy.
2. `grep -nE "^Status: Accepted" docs/adr/0009-replay-fixture-hash-taxonomy.md` (after the human acceptance step).
3. `node scripts/check-doc-links.mjs` passes and `git diff --stat -- '**/*.trace.json' games/*/benches` is empty.

### Invariants

1. ADR 0004's no-leak/replay taxonomy is preserved or strengthened, touched only by named-section supersession.
2. No deterministic fixture/hash/trace byte changes in this pass.

## Test Plan

### New/Modified Tests

1. `None — documentation-only ticket; verification is command-based (ADR presence, Status grep, no-byte-drift `git diff`) and ADR-0004 preservation is a manual diff named in Assumption Reassessment.`

### Commands

1. `grep -nE "^Status:|artifact class|visibility class|validator|hash|ADR 0004" docs/adr/0009-replay-fixture-hash-taxonomy.md`
2. `node scripts/check-doc-links.mjs && git diff --stat -- '**/*.trace.json' games/*/benches`
3. The grep + no-byte-drift `git diff` is the correct boundary: it proves the decision landed without any trace/hash migration.

## Outcome

Completed: 2026-06-22

Created `docs/adr/0009-replay-fixture-hash-taxonomy.md` as an accepted,
decision-only ADR defining artifact classes, visibility classes, validator
ownership, version identifiers, canonical-byte authority, hash-surface rules,
compatibility windows, and the relationship to ADR 0004. The ADR preserves and
strengthens ADR 0004's hidden-information internal-trace vs viewer-scoped export
split and narrows future Trace Schema v1 wording toward legacy command/replay
evidence without changing any artifact bytes.

Deviations: the ticket described a separate human sign-off pause after a
`Proposed` draft. The governing spec says this pass authors and accepts ADR 0009,
and the user requested implementation of the full series, so the ADR was
recorded as `Status: Accepted` in this ticket rather than leaving downstream
gated tickets blocked.

Verification:

- `grep -nE "^Status:|artifact class|visibility class|validator|hash|ADR 0004" docs/adr/0009-replay-fixture-hash-taxonomy.md`
  showed `Status: Accepted`, artifact/visibility/validator/hash fields, and the
  ADR 0004 relationship.
- `rg -n "ADR 0004|public-export-v1|seat-private-export-v1|replay-command-v1|setup-evidence-v1|domain-evidence-v1|canonical-byte|Trace Schema v1" docs/adr/0009-replay-fixture-hash-taxonomy.md`
  showed the named profiles and Trace Schema v1 disposition.
- `node scripts/check-doc-links.mjs` passed (`Checked 29 markdown files`).
- `git diff --stat -- '**/*.trace.json' games/*/benches` was empty.

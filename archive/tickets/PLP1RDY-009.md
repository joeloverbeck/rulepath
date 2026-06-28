# PLP1RDY-009: Scale multi-seat / testing / fixtures / trace docs + templates

**Status**: COMPLETED
**Priority**: MEDIUM
**Effort**: Large
**Engine Changes**: None — contract docs + templates (`docs/MULTI-SEAT-AND-SURFACE-CONTRACT.md`, `docs/TESTING-REPLAY-BENCHMARKING.md`, `docs/EVIDENCE-FIXTURE-CONTRACT.md`, `docs/TRACE-SCHEMA-v1.md`, `templates/README.md`, `templates/AGENT-TASK.md`, `templates/GAME-RULES.md`, `templates/GAME-SOURCES.md`, `templates/GAME-HOW-TO-PLAY.md`, `templates/GAME-BENCHMARKS.md`)
**Deps**: PLP1RDY-004, PLP1RDY-006

## Problem

A COIN-scale private game stresses multi-seat asymmetry, large-game test/replay/
benchmark coverage, private-source evidence, and large-event traces. The spec
bundles the scaling doc + template edits as WB-6 (report `A-09`, `A-13`, `A-14`,
`A-15`, `B-01`, `B-02`, `B-03`, `B-06`, `B-07`, `B-12`). It depends on the
constitution carve-out (PLP1RDY-004) and the event-card boundary (PLP1RDY-006,
which the rules/event-deck template sections reference).

## Assumption Reassessment (2026-06-28)

1. Targets verified present: `docs/MULTI-SEAT-AND-SURFACE-CONTRACT.md`,
   `docs/TESTING-REPLAY-BENCHMARKING.md`, `docs/EVIDENCE-FIXTURE-CONTRACT.md`,
   `docs/TRACE-SCHEMA-v1.md`, `templates/README.md`, `templates/AGENT-TASK.md`,
   `templates/GAME-RULES.md`, `templates/GAME-SOURCES.md`,
   `templates/GAME-HOW-TO-PLAY.md`, `templates/GAME-BENCHMARKS.md`. The
   `B-01`/`B-02` template README + AGENT-TASK edits were confirmed in scope by
   `/reassess-spec` (2026-06-28, Improvement M1: added to WB-6 Items).
2. Spec source: `specs/private-lane-foundation-readiness.md` WB-6 + §Acceptance
   evidence ("Determinism / hash / RNG: Not applicable — TRACE-SCHEMA clarification
   is documentation only; report `A-15`").
3. Cross-artifact boundary under audit: `docs/TRACE-SCHEMA-v1.md` is a
   serialization/replay **contract**; the `A-15` edit is a large-event vocabulary
   **clarification with no migration** — additive doc text only, no schema field,
   hash, RNG, or serialization-order change.
4. FOUNDATIONS principle under audit (§11 multi-seat viewer safety): the
   MULTI-SEAT-AND-SURFACE-CONTRACT edit scales for asymmetric factions and the
   5-viewer no-leak obligation — pairwise seat-private redaction across all
   declared seats (§11 N-seat invariant).
5. §11 no-leak firewall + determinism touched (documented, not enforced here):
   the testing/fixtures/trace docs describe private large-game coverage,
   private-source evidence profiles, and large-event traces while keeping hidden
   information non-leaking and replay/hash deterministic. The TRACE-SCHEMA
   clarification introduces no nondeterminism and no schema migration; enforcement
   stays with the existing `replay-check` / golden-trace surfaces and a later
   private spec's tests.
6. Schema/contract extension check (§11 additive-only): the TRACE-SCHEMA-v1 and
   EVIDENCE-FIXTURE-CONTRACT edits are **additive clarifications** (new guidance
   text / profile rows), not breaking changes — no existing trace consumer or
   golden-trace hash is invalidated; "no migration" is explicit per report `A-15`.

## Architecture Check

1. Documenting large-game coverage + the no-migration trace clarification in the
   existing contract docs scales the obligation without a schema bump — the
   alternative (a trace-schema migration) would trip a §13 ADR trigger and is
   explicitly out of scope.
2. No backwards-compatibility shim: additive clarifications only; no trace format
   or hash semantics change.
3. `engine-core` stays noun-free (§3): faction/seat asymmetry is documented as
   game-local / multi-seat-contract vocabulary, not kernel nouns.

## Verification Layers

1. Multi-seat asymmetric + 5-viewer no-leak guidance present -> codebase
   grep-proof in `docs/MULTI-SEAT-AND-SURFACE-CONTRACT.md`.
2. TRACE-SCHEMA clarification is doc-only / no migration -> grep
   `docs/TRACE-SCHEMA-v1.md` for the no-migration note; no golden-trace/hash file
   changes (`git status --porcelain -- 'games/**/tests/**'` empty).
3. Determinism preserved -> FOUNDATIONS alignment check (§11 deterministic
   replay/hash/serialization unchanged; §13 trigger not engaged — no schema change).
4. Cross-artifact doc-link integrity -> `node scripts/check-doc-links.mjs`.

## What to Change

### 1. Docs

`docs/MULTI-SEAT-AND-SURFACE-CONTRACT.md` asymmetric factions + 5-viewer no-leak
(`A-09`); `docs/TESTING-REPLAY-BENCHMARKING.md` private large-game coverage
subsection (`A-13`); `docs/EVIDENCE-FIXTURE-CONTRACT.md` private-source evidence
profiles (`A-14`); `docs/TRACE-SCHEMA-v1.md` large-event clarification, **no
migration** (`A-15`).

### 2. Templates

`templates/README.md` private-lane index guidance (`B-01`); `templates/AGENT-TASK.md`
private-source fields (`B-02`); `templates/GAME-RULES.md` private-source +
event-deck sections (`B-03`); `templates/GAME-SOURCES.md` private-source receipts
(`B-06`); `templates/GAME-HOW-TO-PLAY.md` per-faction (`B-07`);
`templates/GAME-BENCHMARKS.md` COIN workloads (`B-12`).

## Files to Touch

- `docs/MULTI-SEAT-AND-SURFACE-CONTRACT.md` (modify)
- `docs/TESTING-REPLAY-BENCHMARKING.md` (modify)
- `docs/EVIDENCE-FIXTURE-CONTRACT.md` (modify)
- `docs/TRACE-SCHEMA-v1.md` (modify)
- `templates/README.md` (modify)
- `templates/AGENT-TASK.md` (modify)
- `templates/GAME-RULES.md` (modify)
- `templates/GAME-SOURCES.md` (modify)
- `templates/GAME-HOW-TO-PLAY.md` (modify)
- `templates/GAME-BENCHMARKS.md` (modify)

## Out of Scope

- Any trace-schema migration, golden-trace regeneration, hash/RNG, or
  serialization-order change (explicitly excluded; report `A-15`).
- The bot/AI + milestone docs (PLP1RDY-008).
- SOURCES external prior-art notes (PLP1RDY-010).

## Acceptance Criteria

### Tests That Must Pass

1. `grep -qiE '5[- ]viewer|asymmetric faction|pairwise' docs/MULTI-SEAT-AND-SURFACE-CONTRACT.md`.
2. `grep -qi 'no migration' docs/TRACE-SCHEMA-v1.md` — clarification is non-migrating.
3. `node scripts/check-doc-links.mjs` and no `games/**/tests/**` trace diff.

### Invariants

1. Replay/hash/serialization order/RNG/traces are unchanged (doc clarification only).
2. Multi-seat viewer-safety guidance preserves pairwise seat-private redaction (§11).

## Test Plan

### New/Modified Tests

1. `None — contract docs + templates; verification is command-based (scaling greps + no-trace-diff guard + doc-link gate) and the determinism/no-leak invariant set is named in Assumption Reassessment.`

### Commands

1. `git status --porcelain -- 'games/**/tests/**' 'docs/TRACE-SCHEMA-v1.md'`
2. `node scripts/check-doc-links.mjs`
3. A narrower command suffices: docs/templates only with an explicit no-trace-change guard, so the scaling greps + doc-link gate are the correct verification boundary.

## Outcome

Completed the multi-seat, large-game evidence, fixture, trace, and template
scaling pass. Added asymmetric faction and 5-viewer no-leak guidance to
`docs/MULTI-SEAT-AND-SURFACE-CONTRACT.md`, private large-game coverage and
COIN-scale workload guidance to `docs/TESTING-REPLAY-BENCHMARKING.md`,
private-source evidence profile guidance to `docs/EVIDENCE-FIXTURE-CONTRACT.md`,
and a large-event Trace Schema v1 clarification that explicitly requires no
migration.

Updated the templates for private-lane index guidance, private-source task
fields, event-deck rules coverage, private-source receipts, per-faction
How-to-Play orientation, and large asymmetric benchmark workload dimensions.

Verification:

- `grep -qiE '5[- ]viewer|asymmetric faction|pairwise' docs/MULTI-SEAT-AND-SURFACE-CONTRACT.md`
- `grep -qi 'no migration' docs/TRACE-SCHEMA-v1.md`
- `git status --porcelain -- 'games/**/tests/**'`
- `node scripts/check-doc-links.mjs`
- `git diff --check`

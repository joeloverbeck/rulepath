# PHA0NEXPHAFOU-007: TESTING-REPLAY-BENCHMARKING + TRACE-SCHEMA-v1 N-seat semantics (no schema change)

**Status**: PENDING
**Priority**: HIGH
**Effort**: Medium
**Engine Changes**: None — `docs/TESTING-REPLAY-BENCHMARKING.md` + `docs/TRACE-SCHEMA-v1.md` edits only; no trace field/version change.
**Deps**: PHA0NEXPHAFOU-002

## Problem

Two-player hidden-info tests do not scale to pairwise N-player surfaces, and benchmark rows do not require surface/object-count declarations. The trace schema is already N-seat-capable, but its examples and validation expectations need explicit N-seat semantics so authors don't read them as two-player or assume a migration is needed.

## Assumption Reassessment (2026-06-13)

1. The trace schema is already N-seat-capable: `crates/engine-core/src/replay.rs:137` `seats: Vec<SeatAssignment>` plus viewer hashes. No trace field or version change is required for N seats — the edit documents stricter semantics on the existing fields.
2. Docs: `docs/TESTING-REPLAY-BENCHMARKING.md` (test taxonomy, no-leak, benchmark budgets, golden traces) and `docs/TRACE-SCHEMA-v1.md` (`seats` array, actor seat, hashes, viewer hashes). `docs/MULTI-SEAT-AND-SURFACE-CONTRACT.md` (PHA0NEXPHAFOU-002) and ADR 0004 are the cross-reference targets; `tools/replay-check` is the consuming validator.
3. Cross-artifact boundary under audit: the test taxonomy + the trace schema; shared surface = the `seats`-array semantics and the no-leak test matrix.
4. FOUNDATIONS principle restate: §11 (no hidden-info leak; deterministic replay/hash/serialization). Edits are meaning-preserving; the TRACE-SCHEMA change is **semantics-only**, never a field or version change.
5. Enforcement surface: §11 no-leak firewall and deterministic replay/hash. The N-seat no-leak taxonomy and the stricter `seats`-array invariants clarify existing rules; because no field/version changes, no migration is implied and no nondeterminism path is introduced (seat-array order stays stable and authoritative). Enforced by the Infra D harness and `replay-check`.

## Architecture Check

1. Documenting stricter N-seat semantics on the existing `seats` array avoids a needless schema migration — cleaner than adding fields.
2. No backwards-compatibility aliasing/shims introduced.
3. No trace field/version change (that would require an ADR per §13); `engine-core` is untouched and stays noun-free.

## Verification Layers

1. N-seat no-leak taxonomy + benchmark-by-seat-count rows present → manual review.
2. TRACE-SCHEMA N-seat semantics note present **and** no field/version change → codebase grep-proof (the schema-version line is unchanged).
3. Stricter `seats`-array invariants preserve determinism → FOUNDATIONS alignment check (§11) + golden-trace/replay-hash reasoning.
4. Links resolve → `node scripts/check-doc-links.mjs`.

## What to Change

### 1. `docs/TESTING-REPLAY-BENCHMARKING.md`

Add an N-seat no-leak taxonomy: for every seat A and B, A's private payload never appears in B's view/effects/action tree/previews/DOM/export unless Rust marks it public. Require a public-observer export and at least two seat-private exports for hidden-info games; for 4+ seats require all seats in CI smoke or a documented sample matrix. Add benchmark fixtures keyed by seat count and max surface.

### 2. `docs/TRACE-SCHEMA-v1.md`

State that no migration is required for N seats when the existing fields are used. Add invariants: the `seats` array order is stable and authoritative; the actor seat must be in `seats`; view hashes should include the public observer and every authorized seat viewer for hidden-info games unless a sampled matrix is documented; terminal traces record per-seat standing arrays, not `seat_0`/`seat_1` scalars. **Do not change any field or the schema version.**

## Files to Touch

- `docs/TESTING-REPLAY-BENCHMARKING.md` (modify)
- `docs/TRACE-SCHEMA-v1.md` (modify)

## Out of Scope

- Changing any `TRACE-SCHEMA-v1` field or the schema version (would require its own ADR).
- Editing `crates/engine-core/src/replay.rs` or `tools/replay-check`.
- Recalibrating benchmark thresholds (governed by ADR 0003 / 0005).

## Acceptance Criteria

### Tests That Must Pass

1. `docs/TESTING-REPLAY-BENCHMARKING.md` carries the pairwise N-seat no-leak taxonomy and the benchmark-by-seat-count/max-surface fixtures.
2. `docs/TRACE-SCHEMA-v1.md` carries the N-seat semantics note and invariants with **no** field or version change.
3. `node scripts/check-doc-links.mjs` passes.

### Invariants

1. The `TRACE-SCHEMA-v1` version identifier is unchanged (grep-proof on the version line).
2. Deterministic replay/hash/serialization semantics (§11) are preserved — the edit clarifies, it does not migrate.

## Test Plan

### New/Modified Tests

1. `None — documentation-only ticket; verification is command-based and existing pipeline/test coverage is named in Assumption Reassessment.`

### Commands

1. `node scripts/check-doc-links.mjs`
2. `grep -niE "version|seats array|per-seat standing" docs/TRACE-SCHEMA-v1.md`
3. `grep -niE "no-leak|seat count|pairwise" docs/TESTING-REPLAY-BENCHMARKING.md`

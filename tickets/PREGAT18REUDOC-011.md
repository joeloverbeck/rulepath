# PREGAT18REUDOC-011: WASM-CLIENT-BOUNDARY canonical seat grammar + alias policy

**Status**: PENDING
**Priority**: HIGH
**Effort**: Medium
**Engine Changes**: None — docs-only (`docs/WASM-CLIENT-BOUNDARY.md`)
**Deps**: 005, 007

## Problem

`SeatId` is an opaque string and the corpus uses divergent seat-id forms across games, but `WASM-CLIENT-BOUNDARY.md` documents no canonical seat grammar. The report's risk register flags that a seat-grammar string-normalization bug can become a hidden-information leak. This ticket documents `seat_<zero-based>` as the going-forward canonical form plus a bounded alias policy covering the existing forms — migration deferred to the Part C unit.

## Assumption Reassessment (2026-06-22)

1. Verified `SeatId(pub String)` is opaque (`crates/engine-core/src/lib.rs:29`) and the corpus diverges: `seat-0` (`games/race_to_n`, `games/column_four`), `seat_0` (`games/event_frontier`, `games/secret_draft`), and `seat-a` in engine-core's own doctest (`lib.rs:91`); `WASM-CLIENT-BOUNDARY.md` defines no seat grammar today (reassess finding I1, this session).
2. Verified against spec D10 / WB7 (reframed per reassess I1): document `seat_<zero-based>` as the going-forward form + a bounded alias policy enumerating the existing `seat-<n>` / `seat_<n>` / `seat-a` forms; migration deferred. ADR 0009 (ticket 005) governs the export visibility classification this grammar feeds; hence `Deps: 005` + acceptance precondition.
3. Cross-artifact boundary under audit: the `seat-private-export-v1` profile in `EVIDENCE-FIXTURE-CONTRACT.md` (ticket 007) classifies seat visibility; the documented seat grammar must be consistent with it. Hence `Deps: 007`.
4. FOUNDATIONS §2 + §11 motivate this: restating the invariant — Rust owns seat-id parse/format and viewer projection (TypeScript presentation-only); the alias policy must specify rejecting unknown forms so a normalization bug cannot misroute a viewer (the strict Rust parser is the Part C code; this pass documents the contract).
5. Touches the §11 no-leak firewall and §2 behavior authority: confirm the documented grammar keeps Rust the parse/format authority, the alias policy covers the existing forms so the deferred migration stays leak-safe, and the WASM exported-API schema is **unchanged** this pass (migration deferred).

## Architecture Check

1. Documenting one canonical going-forward grammar + a bounded import-only alias policy is cleaner and safer than leaving the seat-id form ad-hoc per game; it sets the contract the Part C strict parser will enforce.
2. No backwards-compatibility shims in code; the alias policy is an explicit, bounded import-only allowance, not an open-ended normalizer.
3. `engine-core` stays noun-free (§3); no `game-stdlib` change (§4); Rust stays the behavior authority (§2).

## Verification Layers

1. `WASM-CLIENT-BOUNDARY.md` defines the canonical `seat_<zero-based>` going-forward grammar + bounded alias policy -> codebase grep-proof.
2. Alias policy enumerates the existing `seat-<n>` / `seat_<n>` / letter forms and states unknown forms are rejected -> grep.
3. ADR 0009 `Accepted` precondition -> grep (`^Status: Accepted` on `docs/adr/0009-*.md`).
4. WASM exported-API schema unchanged this pass -> `git diff --stat -- crates/wasm-api` is empty (migration deferred).
5. Links resolve -> `node scripts/check-doc-links.mjs`.

## What to Change

### 1. Canonical seat grammar

Document `seat_<zero-based>` (e.g. `seat_0`) as the going-forward canonical external seat-id form, noting `SeatId` is opaque today.

### 2. Bounded alias policy

Specify a bounded import-only alias policy covering the existing `seat-<n>` (hyphen), `seat_<n>` (underscore), and `seat-a` (letter) forms, with unknown forms rejected; state that migration to the canonical form is deferred to the Part C unit.

## Files to Touch

- `docs/WASM-CLIENT-BOUNDARY.md` (modify)

## Out of Scope

- Implementing the strict Rust seat-id parser or migrating any game's seat IDs (Part C successor unit).
- The TESTING / TRACE-SCHEMA edits (ticket 010).

## Acceptance Criteria

### Tests That Must Pass

1. `grep -niE "seat_<zero-based>|canonical seat" docs/WASM-CLIENT-BOUNDARY.md` returns the going-forward grammar.
2. `grep -niE "alias|seat-|reject" docs/WASM-CLIENT-BOUNDARY.md` returns the alias policy covering the existing forms.
3. `node scripts/check-doc-links.mjs` passes; `git diff --stat -- crates/wasm-api` is empty.

### Invariants

1. Rust remains the seat-id parse/format and projection authority (§2); TypeScript decides nothing.
2. The WASM exported-API schema is unchanged this pass; migration is deferred and leak-safe.

## Test Plan

### New/Modified Tests

1. `None — documentation-only ticket; verification is command-based (grammar/alias grep, no-WASM-byte-change `git diff`, link check) named in Assumption Reassessment.`

### Commands

1. `grep -niE "seat_|alias|reject unknown" docs/WASM-CLIENT-BOUNDARY.md`
2. `node scripts/check-doc-links.mjs && git diff --stat -- crates/wasm-api`
3. The grammar/alias grep + no-WASM-change `git diff` is the correct boundary: contract documented, migration deferred.

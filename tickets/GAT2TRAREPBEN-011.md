# GAT2TRAREPBEN-011: Implement `tools/rule-coverage`

**Status**: PENDING
**Priority**: HIGH
**Effort**: Medium
**Engine Changes**: Yes — replaces the `tools/rule-coverage` no-op placeholder with a structural rule-coverage drift checker over `race_to_n` docs.
**Deps**: None

## Problem

`tools/rule-coverage` is a 16-line no-op. Gate 2 requires a lightweight structural
checker that proves every stable rule ID in `RULES.md` is covered exactly once in
`RULE-COVERAGE.md`, that no row references an unknown rule ID, and that no
open/blank/unsupported row hides a silent gap (spec §D8). It checks coverage
integrity only — not rule correctness.

## Assumption Reassessment (2026-06-05)

1. `tools/rule-coverage/src/main.rs` is a 16-line no-op; the crate is in the workspace
   `Cargo.toml` `members` list. `games/race_to_n/docs/RULES.md` carries stable rule IDs
   (`R-SCOPE-001`, `R-VAR-001`, `R-COMP-001`, …) and states "Every rule ID in this
   document must appear in `RULE-COVERAGE.md`." `games/race_to_n/docs/RULE-COVERAGE.md`
   has a `Rule ID | Rule summary | Implementation | Evidence | Status | Notes` matrix
   with status labels `covered`, `covered-by-trace`, `not-applicable`,
   `intentionally-deferred`, `unsupported`.
2. Spec §D8 fixes the required checks (exactly-one coverage row per rule ID; no unknown
   IDs; no open/blank evidence rows; rationale for `not-applicable` / `unsupported` /
   `intentionally-deferred`; performance-deferral rows consistent with `BENCHMARKS.md`
   and the Gate 2 Stage-1 resolution). Non-goal: semantic proof of correctness.
3. Cross-artifact boundary under audit: the checker parses two docs
   (`RULES.md` rule IDs and `RULE-COVERAGE.md` matrix rows); the contract is the stable
   rule-ID set and the status-label vocabulary. The Stage-1 perf-deferral consistency
   check reads `BENCHMARKS.md`, whose decision is finalized by GAT2TRAREPBEN-008 — but
   that is runtime data the tool reads, not a build dependency.
4. FOUNDATIONS §6 (official games are evidence-heavy) and §11 (evidence coverage):
   restate that this enforces the "no silent doc gaps" rule structurally; it does not
   replace the Rust rule/property/trace tests that are the actual rule authority.

## Architecture Check

1. A structural parser (rule-ID set ↔ coverage rows) is exactly spec §D8's scope; a
   semantic analyzer is explicitly out of scope and would overreach.
2. No backwards-compatibility shims; the no-op is replaced.
3. `engine-core` untouched; the tool reasons over Markdown docs, adding no kernel noun.

## Verification Layers

1. Coverage completeness → codebase grep-proof / CLI run: every `RULES.md` rule ID
   appears exactly once in `RULE-COVERAGE.md`; a deleted coverage row fails.
2. Unknown-ID rejection → CLI run: a coverage row citing a non-existent rule ID fails.
3. Blank/open-row + missing-rationale rejection → CLI run: an empty-evidence row, and
   a `not-applicable`/`unsupported`/`intentionally-deferred` row lacking rationale,
   each fail.

## What to Change

### 1. CLI parsing (`tools/rule-coverage/src/main.rs`)

Implement `--game race_to_n`.

### 2. Coverage checker

Parse stable rule IDs from `RULES.md`; parse coverage matrix rows from
`RULE-COVERAGE.md`; enforce exactly-one coverage row per rule ID; reject unknown IDs
and open/blank evidence rows; require rationale for `not-applicable`, `unsupported`,
and `intentionally-deferred`; check performance-deferral rows are consistent with
`BENCHMARKS.md` and the Stage-1 resolution. Exit non-zero on any drift.

### 3. `tools/rule-coverage/Cargo.toml`

Add any Markdown/parse dependency needed.

## Files to Touch

- `tools/rule-coverage/src/main.rs` (modify) — replace no-op with the structural checker
- `tools/rule-coverage/Cargo.toml` (modify) — add parse deps if needed

## Out of Scope

- Semantic proof of rule correctness or test-coverage percentages (§D8 non-goals).
- Editing the docs themselves (GAT2TRAREPBEN-014 owns RULE-COVERAGE.md row updates).
- CI wiring (GAT2TRAREPBEN-013).

## Acceptance Criteria

### Tests That Must Pass

1. `cargo run -p rule-coverage -- --game race_to_n` — passes on the current valid docs.
2. A deleted coverage row, an unknown rule ID, and an open/blank evidence row each make it exit non-zero.
3. `cargo test -p rule-coverage` — fixture-driven negative cases prove each rejection.

### Invariants

1. Every stable rule ID is covered exactly once; unknown/blank/unjustified rows fail (§D8).
2. The checker is structural only — it asserts coverage integrity, not rule correctness (§6).

## Test Plan

### New/Modified Tests

1. `tools/rule-coverage/src/main.rs` (or `tools/rule-coverage/tests/`) — negative fixtures for deleted row, unknown ID, blank evidence, missing rationale.

### Commands

1. `cargo run -p rule-coverage -- --game race_to_n`
2. `cargo test -p rule-coverage`
3. A doc-fixture negative run is the correct boundary; the live-docs check runs in CI (GAT2TRAREPBEN-013).

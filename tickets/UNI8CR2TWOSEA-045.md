# UNI8CR2TWOSEA-045: R2 register receipts, report consolidation, and C-10 non-promotion

**Status**: PENDING
**Priority**: HIGH
**Effort**: Medium
**Engine Changes**: None — governance: append R2 register receipts (`docs/MECHANICAL-SCAFFOLDING-REGISTER.md`) and reconcile the characterization report; no code, schema, or trace change
**Deps**: 002, 003, 004, 005, 006, 007, 008, 009, 010, 011, 012, 013, 014, 015, 016, 017, 018, 019, 020, 021, 022, 023, 024, 025, 026, 027, 028, 029, 030, 031, 032, 033, 034, 035, 036, 037, 038, 039, 040, 041, 042, 043, 044

## Problem

Spec §4.3 + §5.14 tasks `8C-R2-801`/`802`: append the complete R2 receipt tables beneath the existing `MSC-8C-001…010` entries, reconcile every matrix cell / hash / visibility surface / exception / N/A / before-after in the characterization report, and record C-10's explicit rejection of behavioral promotion. Every row names decision state, evidence, byte/hash/visibility impact, rollback, and next review trigger; no unowned "remaining cleanup" bucket.

## Assumption Reassessment (2026-06-23)

1. `docs/MECHANICAL-SCAFFOLDING-REGISTER.md` contains `MSC-8C-001…010` (confirmed in the reassess pass); the characterization report was created in `-001` and is reconciled here. All migration tickets `-002`…`-044` have landed before this ticket (its `Deps`).
2. Spec §4.3/§4.2/§8.3: append R2 receipts under the existing entries (no rival register); `MSC-8C-001` six constructors + two private N/As; `002` four parsers + adapters + roster exceptions; `003` four count migrations + ring N/A; `004`/`005` four action-tree v1 + adjacent exceptions; `006` HCD receipt + three dev-only additions; `007` HCD pilot verification + three new matrices; `008` every profile verdict incl. seat-private/domain N/As; `009` three RNG migrations + Secret N/A; `010` four non-promotion affirmations.
3. Cross-artifact under audit: the register (`docs/MECHANICAL-SCAFFOLDING-REGISTER.md`) and the characterization report — both record evidence; neither ships code or canonical bytes.
4. §4 / §11: C-10 reaffirms that reveal/reaction/projection/pledge/pot/scoring/outcome stay game-owned (no promotion to shared code); every receipt names byte/hash/visibility impact, rollback, and next trigger, and no existing decision is weakened.

## Architecture Check

1. One append-only register + report-reconciliation ticket keeps every verdict, exception, and N/A owned in one auditable place — cleaner than scattering receipts across migration tickets.
2. No backwards-compat alias; existing `MSC-8C-*` entries are not weakened, only appended to.
3. `engine-core` untouched; C-10 explicitly records no `game-stdlib`/behavior promotion.

## Verification Layers

1. R2 receipts appended under `MSC-8C-001…010` with decision/evidence/impact/rollback/trigger -> manual review + codebase grep-proof in `docs/MECHANICAL-SCAFFOLDING-REGISTER.md`.
2. Every matrix cell / hash / visibility / exception / N/A reconciled -> manual review against spec §3.3–§3.10 and the report.
3. Doc-link integrity -> `node scripts/check-doc-links.mjs`; no existing decision weakened -> manual review.

## What to Change

### 1. Append R2 register receipts

Beneath the existing `MSC-8C-001…010` entries, append the R2 receipt tables enumerated in spec §4.3, one row per surface with decision state, evidence, byte/hash/visibility impact, rollback, and next review trigger.

### 2. Reconcile the characterization report

Complete the append-only report (`reports/8c-r2-two-seat-hidden-reaction-scaffolding-characterization.md`) with every before/after result, exception, and N/A; record C-10's rejection of behavioral promotion.

## Files to Touch

- `docs/MECHANICAL-SCAFFOLDING-REGISTER.md` (modify)
- `reports/8c-r2-two-seat-hidden-reaction-scaffolding-characterization.md` (modify; created by `-001`)

## Out of Scope

- The final acceptance run and `specs/README.md` status flip (`-046`).
- Any code/test/trace change; creating a rival register; weakening any existing `MSC-8C-*` decision.

## Acceptance Criteria

### Tests That Must Pass

1. `node scripts/check-doc-links.mjs` passes.
2. Every spec §4.3 receipt row is present under the existing `MSC-8C-001…010` entries, and the report reconciles every matrix cell, exception, and N/A.

### Invariants

1. No existing register decision is weakened; R2 receipts are append-only.
2. C-10 records that all reveal/reaction/projection/pledge/pot/scoring/outcome behavior stays game-owned.

## Test Plan

### New/Modified Tests

1. `None — docs-reconciliation governance ticket; verification is command-based (`check-doc-links` + the prior tickets' suites) and existing coverage is the regression guard.`

### Commands

1. `node scripts/check-doc-links.mjs`
2. `cargo test --workspace --all-targets` (confirms every migration the receipts record is green)

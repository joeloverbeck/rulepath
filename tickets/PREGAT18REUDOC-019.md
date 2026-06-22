# PREGAT18REUDOC-019: Realign PRIMITIVE-PRESSURE-LEDGER (behavioral; redirect plumbing to the register)

**Status**: PENDING
**Priority**: MEDIUM
**Effort**: Small
**Engine Changes**: None — docs-only (`templates/PRIMITIVE-PRESSURE-LEDGER.md`)
**Deps**: 006

## Problem

The primitive-pressure ledger is the behavioral mechanic-promotion record. With the scaffolding register now governing non-behavioral plumbing repetition, the ledger should explicitly stay behavioral and redirect any non-behavioral (mechanical-scaffolding) repetition to the register — so the two reuse lanes don't get conflated in the ledger.

## Assumption Reassessment (2026-06-22)

1. Verified `templates/PRIMITIVE-PRESSURE-LEDGER.md` is currently purely behavioral (every row maps to a behavior-laden mechanic shape; no plumbing repetition mixed in) — confirmed during the `/reassess-spec` validation this session. spec D12: keep it behavioral, add a redirect to the scaffolding register (ticket 006). Hence `Deps: 006`.
2. Verified against spec D12 + reassess finding M1: this ledger is a single-template diff in the WB10 cluster; the `B-NN → template` mapping is derived from report §4.
3. Cross-artifact boundary under audit: the redirect points at `docs/MECHANICAL-SCAFFOLDING-REGISTER.md` (ticket 006), which must exist. (No dependency on `GAME-EVIDENCE.md` — the ledger is redirected, not slimmed into the receipt; the spec WB10 `Deps: WB8` annotation does not apply structurally here, surfaced as a divergence.)
4. FOUNDATIONS §4 motivates this: restating the invariant — the behavioral primitive-pressure ledger stays behavioral; non-behavioral plumbing repetition routes to the scaffolding register (the clean lane separation ADR 0008 establishes).

## Architecture Check

1. An explicit behavioral-only ledger + a redirect to the register cleanly separates the two reuse lanes, preventing plumbing repetition from being mistaken for a behavioral third-use pressure.
2. No backwards-compatibility shims.
3. `engine-core` (§3) / `game-stdlib` (§4) untouched; docs-only; the behavioral earning rule is preserved.

## Verification Layers

1. The ledger stays behavioral — no non-behavioral/plumbing rows added -> manual review + grep.
2. A redirect to `MECHANICAL-SCAFFOLDING-REGISTER.md` is present -> grep.
3. `docs/MECHANICAL-SCAFFOLDING-REGISTER.md` exists (Deps 006) -> `test -f`.
4. Links resolve -> `node scripts/check-doc-links.mjs`.

## What to Change

### 1. Keep behavioral + redirect

Edit `templates/PRIMITIVE-PRESSURE-LEDGER.md` to state it governs behavioral mechanic pressure only, and add a redirect routing non-behavioral (mechanical-scaffolding) repetition to `docs/MECHANICAL-SCAFFOLDING-REGISTER.md`.

## Files to Touch

- `templates/PRIMITIVE-PRESSURE-LEDGER.md` (modify)

## Out of Scope

- Authoring the scaffolding register (ticket 006).
- The other template clusters (017/018/020) and `AGENT-TASK.md` (021).

## Acceptance Criteria

### Tests That Must Pass

1. `grep -niE "MECHANICAL-SCAFFOLDING-REGISTER|scaffolding register" templates/PRIMITIVE-PRESSURE-LEDGER.md` returns the redirect.
2. `grep -niE "behavioral" templates/PRIMITIVE-PRESSURE-LEDGER.md` confirms the behavioral-only framing.
3. `node scripts/check-doc-links.mjs` passes.

### Invariants

1. The ledger governs behavioral mechanic pressure only; plumbing repetition is redirected, never recorded as behavioral pressure.

## Test Plan

### New/Modified Tests

1. `None — documentation-only ticket; verification is command-based (redirect grep, link check) + manual behavioral-only review named in Assumption Reassessment.`

### Commands

1. `grep -niE "scaffolding register|behavioral" templates/PRIMITIVE-PRESSURE-LEDGER.md`
2. `node scripts/check-doc-links.mjs`
3. The redirect grep + link check is the correct boundary; docs-only with no code surface.

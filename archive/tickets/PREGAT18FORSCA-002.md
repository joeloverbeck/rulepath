# PREGAT18FORSCA-002: FOUNDATIONS §11 forward invariants + §12 stop conditions

**Status**: COMPLETED
**Priority**: HIGH
**Effort**: Small
**Engine Changes**: None — governance/law doc edit (`docs/FOUNDATIONS.md`)
**Deps**: PREGAT18FORSCA-001

## Problem

The standing forward scaffolding-reuse obligation must live in the constitution to bind every future game. This ticket adds the three forward invariants to `FOUNDATIONS.md` §11 and the four forward stop conditions to §12, making reuse-first-audit / register-new / queue-or-dispose universal acceptance invariants. It lands only after the ADR 0008 extension (PREGAT18FORSCA-001) is accepted, per FOUNDATIONS §11's "unless an accepted ADR explicitly changes them" clause.

## Assumption Reassessment (2026-06-25)

1. `docs/FOUNDATIONS.md` §11 carries the existing mechanical-scaffolding invariant at L204–205 (`Mechanical-scaffolding candidates are behavior-free, registered, and rejected or rerouted…`); §12 carries the promotion-debt stop conditions at L237–238 (`a promoted primitive leaves matching prior official games un-migrated…`; `a new mechanic-ladder gate proceeds while promotion debt is still open;`). Verified this session.
2. The spec (`specs/pre-gate-18-forward-scaffolding-reuse-governance.md` D2, §Foundational-document changes) embeds the exact insertion text: three §11 invariants after L204–205, four §12 stop conditions after the promotion-debt conditions. §4 and §13 text unchanged.
3. Shared contract under audit: the §11 acceptance-invariant list and §12 stop-condition list — additive insertions that must not renumber or reword adjacent bullets.
4. FOUNDATIONS §11 (universal acceptance invariants) and §12 (stop conditions) under audit: the additions are meaning-additive (more invariants, more stops) and gated by the accepted ADR 0008 extension (PREGAT18FORSCA-001); A3 holds — no existing invariant's meaning changes.
5. Enforcement surface named: these invariants are later enforced by the Gate 1 governance checker (PREGAT18FORSCA-018/020). The doctrine itself introduces no hidden-information leak and no nondeterminism — it is prose; the receipt it governs is static, unknown-field-rejecting data (PREGAT18FORSCA-017).

## Architecture Check

1. Placing the duty in §11/§12 (not only in area docs) makes it a universal acceptance invariant, the strongest available binding, rather than a per-doc convention that a new game could miss.
2. No backwards-compatibility shim — purely additive bullets; no existing invariant is reworded or removed.
3. `engine-core` noun-free and `game-stdlib` earned rules are untouched; the new invariants reinforce the behavioral third-use gate rather than relaxing it.

## Verification Layers

1. Additive-only invariant → grep-proof the existing L204–205 invariant and L237–238 stop conditions are byte-unchanged and the new bullets sit immediately after.
2. ADR-gating (§13) → FOUNDATIONS alignment check: PREGAT18FORSCA-001 is `Status: Accepted` before this lands (the constitution change cites the accepted ADR extension).
3. Doc-link integrity → `node scripts/check-doc-links.mjs`.

## What to Change

### 1. §11 — three forward invariants

Insert, immediately after the existing mechanical-scaffolding invariant (L204–205), the three invariants: (a) every new official game completes a reuse-first audit before serious implementation; (b) every new behavior-free scaffolding shape is registered with exclusions/surfaces/decision-state/next-review-trigger, first use not authorizing promotion; (c) a matching prior-game duplicate is queued as a bounded follow-on unit or recorded as an accepted `local-only`/`deferred`/`rejected` disposition.

### 2. §12 — four forward stop conditions

Insert, after the promotion-debt stop conditions, the four stops: no-audit start; unregistered new shape at close; unnamed-TODO prior match; local reimplementation of a known promoted helper without a register-backed exception.

## Files to Touch

- `docs/FOUNDATIONS.md` (modify)

## Out of Scope

- Any change to §4 behavioral first/second/third-use wording or §13 ADR triggers.
- Rewording or removing any existing §11 invariant or §12 stop condition (additive only).
- Authoring the enforcing checker or templates (later tickets).

## Acceptance Criteria

### Tests That Must Pass

1. `grep -c "reuse-first audit" docs/FOUNDATIONS.md` ≥ 1 and the existing L204–205 invariant text is byte-unchanged (source comparison).
2. `node scripts/check-doc-links.mjs` passes.
3. PREGAT18FORSCA-001 carries `Status: Accepted` (or the in-repo accepted marker) before this lands — acceptance-gated.

### Invariants

1. §11 gains exactly three forward invariants; §12 gains exactly four forward stop conditions; no existing bullet is reworded.
2. The change cites/relies on the accepted ADR 0008 extension as its §13 authority.

## Test Plan

### New/Modified Tests

1. `None — documentation-only ticket; verification is command-based and existing pipeline/test coverage is named in Assumption Reassessment.`

### Commands

1. `node scripts/check-doc-links.mjs`
2. `git diff -- docs/FOUNDATIONS.md` (review: only additive bullets in §11/§12)
3. `grep -n "Status: Accepted" docs/adr/0008-mechanical-scaffolding-governance.md` (acceptance-gate precondition)

## Outcome

Completed: 2026-06-25

Changed `docs/FOUNDATIONS.md` with the Unit 8F constitution-level forward
scaffolding governance obligations:

- added three §11 acceptance invariants after the existing
  mechanical-scaffolding invariant;
- added four §12 stop conditions after the existing promotion-debt stop
  conditions;
- left §4 behavioral first/second/third-use wording and §13 ADR triggers
  unchanged.

Deviation: none. The change landed after PREGAT18FORSCA-001 extended accepted
ADR 0008, satisfying the constitution-change precondition.

Verification:

- `rg -n "reuse-first audit|Mechanical-scaffolding candidates are behavior-free|a promoted primitive leaves matching prior official games|a new mechanic-ladder gate proceeds|register-backed exception" docs/FOUNDATIONS.md` confirmed the new bullets sit after the existing invariant and promotion-debt stops.
- `grep -n "Status: Accepted" docs/adr/0008-mechanical-scaffolding-governance.md` confirmed the accepted ADR precondition.
- `node scripts/check-doc-links.mjs` passed (`Checked 31 markdown files`).
- `git diff --check` passed.

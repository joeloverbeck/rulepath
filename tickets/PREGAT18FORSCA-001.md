# PREGAT18FORSCA-001: ADR 0008 append-only forward-obligation extension

**Status**: PENDING
**Priority**: HIGH
**Effort**: Medium
**Engine Changes**: None — governance/law doc edit (`docs/adr/0008-mechanical-scaffolding-governance.md`)
**Deps**: None

## Problem

Accepted ADR 0008 records a *retroactive* mechanical-scaffolding lane (the 8C/R1–R4 program). Unit 8F converts that doctrine into a standing, forward, per-new-game obligation, which amends `FOUNDATIONS.md` §11/§12 — a §13 ADR-trigger surface. Per the spec's load-bearing dependency order, the ADR 0008 append-only extension is authored and accepted **first**, before the constitution edits it gates (D2 / PREGAT18FORSCA-002). This ticket lands that extension.

## Assumption Reassessment (2026-06-25)

1. ADR 0008 exists at `docs/adr/0008-mechanical-scaffolding-governance.md` with `Status: Accepted` (L3), an inline `Affected foundation sections:` field (L27: `FOUNDATIONS.md §4, §11, §12, §13; …`), an inline `Migration matrix:` (L60), and `## Decision` (L90) appearing before `## Alternatives considered` (L147). Verified this session.
2. The spec (`specs/pre-gate-18-forward-scaffolding-reuse-governance.md` D1, §Scope, plan §7.1–§7.5) requires four append-only edits: status-note addition (§7.2), complete `Affected foundation sections` replacement (§7.3), the forward per-new-game obligation subsection appended to the end of `## Decision` **before** `## Alternatives considered` (§7.4), and migration-matrix rows (§7.5). Original 2026-06-22 context/decision text stays untouched.
3. Shared contract under audit: the ADR's `## Decision` section boundary (insert before `## Alternatives considered`, not after) and the existing `Migration matrix:` table shape (pipe-delimited, append rows only).
4. FOUNDATIONS §13 (ADR triggers) and §4 (`game-stdlib` earned): operationalizing an accepted ADR's decision and amending the constitution routes through this append-only extension. Reassess A2 (resolved this session): `docs/adr/ADR-TEMPLATE.md` carries no immutable-accepted-ADR rule — only a "Superseded decision, if any" field — and ADR 0008 records `Superseded decision: none`, so the append-only extension is lawful and the §7.6 fallback (successor ADR 0010) is **not** triggered.
5. Preservation surface: the behavioral third-use hard gate quoted in ADR 0008 (L121–123) and `MECHANIC-ATLAS.md` §4 (L73) stays word-for-word effective; this extension adds no new allowed home, helper, or Non-Promotion change, and introduces no leak/determinism path (it is doctrine, not code).

## Architecture Check

1. Append-only extension to the *accepted* ADR keeps one authoritative governing decision rather than splitting the lane across two ADRs; the repo's `ADR-TEMPLATE.md` "Required scaling / supersession fields" sanction in-place scaling.
2. No backwards-compatibility shim — the original decision text is preserved verbatim; the extension is additive and dated.
3. `engine-core` is untouched (no mechanic noun); `game-stdlib` discipline is unchanged — the extension reaffirms the behavioral third-use gate is authoritative.

## Verification Layers

1. Append-only / preservation invariant → grep-proof that the original `## Decision` and `## Alternatives considered` bodies are byte-unchanged and the new subsection sits between them.
2. Behavioral-gate preservation → source comparison: the third-use hard-gate sentence (ADR L121–123, atlas §4 L73) is byte-for-byte unchanged.
3. Doctrine alignment (§13/§4) → FOUNDATIONS alignment check: the extension operationalizes, not reverses, ADR 0008's lane/homes/Non-Promotion List/thresholds.
4. Doc-link integrity → `node scripts/check-doc-links.mjs`.

## What to Change

### 1. Status-note + Affected-foundation-sections

Add the dated forward-obligation status note (plan §7.2) and replace the `Affected foundation sections:` field with the complete plan §7.3 text (still naming `FOUNDATIONS.md §4, §11, §12, §13` plus the forward-obligation scope).

### 2. Forward per-new-game obligation subsection

Append the plan §7.4 subsection to the end of `## Decision`, immediately before `## Alternatives considered` — the seven-point obligation (reuse-first audit, reuse-unless-accepted-exception, first-use registration without promotion, queue real prior-game migration, accepted no-unit disposition fields, Gate 1 receipt check, ADR-0009 deferral) with the closing guarantee that the behavioral third-use gate remains word-for-word effective and no new home/helper/Non-Promotion change is authorized.

### 3. Migration-matrix rows

Append the plan §7.5 rows to the existing `Migration matrix:` table.

## Files to Touch

- `docs/adr/0008-mechanical-scaffolding-governance.md` (modify)

## Out of Scope

- Any change to ADR 0008's allowed homes, Non-Promotion List, semantic-identity rule, second-use review, or pre-third-copy threshold (append-only, operational).
- The §7.6 fallback successor ADR `0010` (not triggered — A2 resolved).
- Editing `FOUNDATIONS.md` (PREGAT18FORSCA-002, gated by this ticket).
- Reopening or amending ADR 0009.

## Acceptance Criteria

### Tests That Must Pass

1. `grep -n "## Decision" docs/adr/0008-mechanical-scaffolding-governance.md` and `grep -n "## Alternatives considered"` confirm the new subsection lands between them.
2. The third-use hard-gate sentence at the original ADR location is byte-for-byte unchanged (source comparison).
3. `node scripts/check-doc-links.mjs` passes.

### Invariants

1. The extension is visibly append-only and dated; original decision text is preserved.
2. No allowed home, helper, or Non-Promotion List entry is added or weakened.

## Test Plan

### New/Modified Tests

1. `None — documentation-only ticket; verification is command-based and existing pipeline/test coverage is named in Assumption Reassessment.`

### Commands

1. `node scripts/check-doc-links.mjs`
2. `git diff -- docs/adr/0008-mechanical-scaffolding-governance.md` (review: original blocks untouched, additions dated)
3. Source comparison of the third-use hard-gate sentence against `docs/MECHANIC-ATLAS.md` §4 L73 — the narrower correct boundary, since this ticket only appends to the ADR.

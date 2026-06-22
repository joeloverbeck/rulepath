# UNI8CMECSCA-001: Flip `specs/README.md` row 8C to Planned

**Status**: COMPLETED
**Priority**: HIGH
**Effort**: Small
**Engine Changes**: None — docs/status-only (`specs/README.md`)
**Deps**: None

## Problem

Unit 8C's design is finalized (`specs/unit-8c-mechanical-scaffolding-code-extraction.md`), but the living progress index still carries stale state: `specs/README.md` row 8C reads `_(seed; unwritten)_` / `Not started` / **Blocked** on ADRs 0008/0009 and the 8M realignment. Those interlocks are now satisfied (ADRs 0008/0009 `Accepted`, 8M `Done`), so the row must be flipped to `Planned` with a link to the spec, and the stale `Blocked` interlock text replaced by an execution/closeout description. This admission step gates every other 8C ticket.

## Assumption Reassessment (2026-06-22)

1. Docs-only ticket — no `engine-core` / `game-stdlib` / `games/*` code is touched. The spec file already exists at `specs/unit-8c-mechanical-scaffolding-code-extraction.md` (renamed/finalized during the reassess-spec session).
2. `specs/README.md` row 8C currently reads `| 8C | … | _(seed; unwritten)_ | Not started | **Blocked** on ADRs 0008/0009 \`Accepted\` + the 8M realignment \`Done\`. … |`; row 8M reads `Done`; ADRs 0004/0008/0009 carry `Status: Accepted`. The `Blocked` predicate is therefore satisfied, not live.
3. Cross-artifact boundary under audit: the `specs/README.md` index contract (the selection rule "pick the lowest unit not `Done`" and the per-row interlock column). The flip changes only this row; it does not edit `docs/ROADMAP.md` (forbidden by spec §9.22) or any sibling row.
4. FOUNDATIONS §11 "Documentation truth": `specs/README.md` is the authoritative progress record, so leaving the stale `Blocked` text is a documentation-truth defect this ticket closes. No behavior authority moves.

## Architecture Check

1. Editing the single index row (not `docs/ROADMAP.md`) keeps progress tracking in its one canonical home, matching the spec's §10.E/§10.B split.
2. No backwards-compatibility shim — the stale interlock text is replaced, not aliased.
3. `engine-core` untouched; `game-stdlib` untouched. No mechanic noun introduced.

## Verification Layers

1. Row 8C status is `Planned` and links the spec → codebase grep-proof on `specs/README.md`.
2. Stale `Blocked` interlock text removed → grep-proof (no `**Blocked**` on the 8C row).
3. Doc-link integrity holds → `node scripts/check-doc-links.mjs`.

## What to Change

### 1. `specs/README.md` row 8C

- Replace the spec-link cell `_(seed; unwritten)_` with a markdown link to `unit-8c-mechanical-scaffolding-code-extraction.md`.
- Change the Status cell `Not started` → `Planned`.
- Replace the `**Blocked** on ADRs 0008/0009 …` interlock prose with an execution/closeout description: shared mechanical-scaffolding (C-01…C-10) plus the dev-only `game-test-support` crate and bounded pilots, with the remaining C-11 retrofits seeded forward; hash/visibility migration governed by ADR 0009; no blanket golden regeneration.

## Files to Touch

- `specs/README.md` (modify)

## Out of Scope

- Editing `docs/ROADMAP.md` (spec §9.22 forbids progress edits there).
- Creating the four C-11 follow-on rows (owned by UNI8CMECSCA-030) or flipping 8C to `Done` (owned by UNI8CMECSCA-031).
- Any code, register, or test change.

## Acceptance Criteria

### Tests That Must Pass

1. `grep -nE '^\| 8C ' specs/README.md` shows Status `Planned` and a link to `unit-8c-mechanical-scaffolding-code-extraction.md`.
2. `grep -n '8C' specs/README.md` shows no `**Blocked**` on the 8C row.
3. `node scripts/check-doc-links.mjs` passes (the new spec link resolves).

### Invariants

1. Only the 8C row changes; row 8M (`Done`) and the Gate 18 row are untouched.
2. `docs/ROADMAP.md` is not modified.

## Test Plan

### New/Modified Tests

1. `None — documentation-only ticket; verification is command-based and existing pipeline/test coverage is named in Assumption Reassessment.`

### Commands

1. `grep -nE '^\| 8C ' specs/README.md`
2. `node scripts/check-doc-links.mjs`
3. A narrower command is correct because the deliverable is a single index-row edit; doc-link integrity is the only pipeline guard that can regress.

## Outcome

Completed: 2026-06-22

Changed `specs/README.md` row 8C from a stale unwritten/`Not started`/`Blocked`
seed to a `Planned` row linking
`specs/unit-8c-mechanical-scaffolding-code-extraction.md`. The replacement
interlock text now describes the shared mechanical-scaffolding C-01...C-10
scope, the dev-only `game-test-support` crate, bounded pilots, forward-seeded
C-11 retrofits, ADR-0009-governed hash/visibility migration, and the no-blanket-
golden-regeneration constraint.

Deviations: none. The edit stayed limited to the single 8C index row; Gate 18
and `docs/ROADMAP.md` were not changed.

Verification:

- `grep -nE '^\| 8C ' specs/README.md` showed row 8C with the spec link and
  Status `Planned`.
- `grep -n '8C' specs/README.md` showed no `**Blocked**` text on the 8C row.
- `node scripts/check-doc-links.mjs` passed (`Checked 31 markdown files`).

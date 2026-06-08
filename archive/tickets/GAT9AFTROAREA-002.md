# GAT9AFTROAREA-002: Capstone — optional `specs/README.md` maintenance row, doc-link acceptance, and handoff

**Status**: COMPLETED
**Priority**: LOW
**Effort**: Small
**Engine Changes**: None — documentation only (`specs/README.md`); no Rust/WASM/engine surface, no TypeScript behavior.
**Deps**: GAT9AFTROAREA-001

## Problem

The Gate 9 aftermath pass needs a closeout that (a) optionally tracks the cleanup in
the living spec index while it is active and (b) records the acceptance evidence and
a handoff confirming the pass stayed within its documentation-only boundary. The spec
(`specs/gate-9-aftermath-roadmap-realignment.md`) names two trailing deliverables: an
**optional** `specs/README.md` maintenance row (D4) and a handoff note (D5) recording
exact files changed, the validation command(s), and the fact that no
gameplay/CI/tooling files were touched. This is the capstone for the gate — it
introduces no new prose about Token Bazaar; it exercises and records the result of
GAT9AFTROAREA-001.

## Assumption Reassessment (2026-06-08)

1. Verified against current structure: `specs/README.md` uses the table format
   `| Stage | Gate | Spec | Status |` (line 25). The Gate-8-aftermath row (`6M`,
   line 39) is the exact precedent template for a maintenance/aftermath row. There is
   no existing `7M` / `gate-9-aftermath` row yet, and the Gate 9 row (`7`, line 40,
   `Done`, `secret_draft` deferred to Gate 9.1) must NOT be rewritten.
2. Verified against specs/docs: the spec D4 marks the row **optional** ("Only add a
   new maintenance row if tracking this cleanup"). The spec's own Status flip to
   `Done` and any move into `archive/specs/` follow `docs/archival-workflow.md` (the
   canonical archival process) — the 6M precedent's row already points at
   `../archive/specs/gate-8-aftermath-roadmap-realignment.md`, confirming aftermath
   specs are archived after completion.
3. Cross-artifact boundary under audit: `specs/README.md` is the living progress
   index. The only change here is appending one `7M` row; existing rows (especially
   the Gate 9 row at line 40) are preserved verbatim.
4. FOUNDATIONS principle restated: §12 stop conditions — the handoff (D5) is the
   verification that this pass did NOT cross into code, CI, tools, ROADMAP-progress,
   or archived-spec territory. The acceptance proof below asserts a docs-only diff, so
   no stop condition is crossed.

## Architecture Check

1. A single trailing capstone is cleaner than scattering the index row, the doc-link
   run, and the handoff across the implementation ticket: it keeps GAT9AFTROAREA-001's
   diff to `apps/web/README.md` alone and isolates the optional index decision and the
   acceptance evidence in one reviewable closeout.
2. No backwards-compatibility aliasing or shims — one additive index row.
3. `engine-core` and `game-stdlib` are untouched; the change is `specs/README.md`
   documentation only.

## Verification Layers

1. Index row truthful and Gate 9 row intact -> codebase grep-proof: a `7M` row exists
   citing `gate-9-aftermath-roadmap-realignment.md`, and `grep -n "Gate 9 " specs/README.md`
   still shows the unchanged `7` row.
2. Doc-link integrity -> `node scripts/check-doc-links.mjs` passes (catches a broken
   relative link in the new row).
3. Documentation-only boundary (handoff D5) -> FOUNDATIONS §12 alignment check:
   `git diff --name-only` lists only `apps/web/README.md` and `specs/README.md`; no
   path under `crates/`, `games/`, `tools/`, `.github/`, or `apps/web/src` is changed.

## What to Change

### 1. Optional `specs/README.md` maintenance row (D4)

If maintainers want to track this cleanup while active, append one row modeled on the
`6M` precedent (line 39). Suggested row:

```markdown
| 7M | Gate 9 aftermath / web README realignment | [`gate-9-aftermath-roadmap-realignment.md`](gate-9-aftermath-roadmap-realignment.md) realigns the web-shell README (intro, Shell Surface, Smoke Layers) to register Token Bazaar after Gate 9. | Planned |
```

Flip the row's Status to `Done` only after the README edit (GAT9AFTROAREA-001) and the
doc-link/diff evidence below pass. Use the same-directory relative link
(`gate-9-aftermath-roadmap-realignment.md`) while the spec lives under `specs/`; when
the spec is later archived per `docs/archival-workflow.md`, the link becomes
`../archive/specs/gate-9-aftermath-roadmap-realignment.md` (as the 6M row shows). Do
not rewrite the existing Gate 9 (`7`) row.

If maintainers decline the row, record that decision in the handoff (D5) instead; the
row is genuinely optional and skipping it is a valid outcome.

### 2. Handoff note (D5)

Record, in the PR/commit description (no file artifact required):

- exact files changed (`apps/web/README.md`; optionally `specs/README.md`);
- validation command(s) run and their result (`node scripts/check-doc-links.mjs`);
- explicit confirmation that no gameplay, Rust crate, WASM, tool, CI, trace, fixture,
  benchmark, or archived-spec file was touched.

## Files to Touch

- `specs/README.md` (modify) — optional `7M` maintenance row only (skip if maintainers
  decline; then this ticket is handoff/acceptance evidence only).

## Out of Scope

- Rewriting the existing Gate 9 (`7`) row or any other `specs/README.md` row.
- Editing `apps/web/README.md` (owned by GAT9AFTROAREA-001).
- Flipping the spec file's own metadata Status or moving it into `archive/specs/` —
  that is the archival step governed by `docs/archival-workflow.md`, performed when the
  maintainers archive the completed spec.
- Any Rust, WASM, tool, CI, game, trace, fixture, benchmark, root `README.md`,
  `progress.md`, `docs/ROADMAP.md`, or `docs/MECHANIC-ATLAS.md` change.

## Acceptance Criteria

### Tests That Must Pass

1. `node scripts/check-doc-links.mjs` passes after the (optional) row is added.
2. If the row was added: `grep -n "7M" specs/README.md` shows the new aftermath row
   citing `gate-9-aftermath-roadmap-realignment.md`, and `grep -n "| 7 | Gate 9 |" specs/README.md`
   still shows the original Gate 9 row unchanged.
3. `git diff --name-only` lists at most `apps/web/README.md` and `specs/README.md` —
   no code, CI, tool, or archived-spec path.

### Invariants

1. The existing Gate 9 (`7`) row's text and `Done` status are byte-for-byte unchanged.
2. The pass is documentation-only: no file under `crates/`, `games/`, `tools/`,
   `.github/`, or `apps/web/src` appears in the diff (the D5 handoff assertion).

## Test Plan

### New/Modified Tests

1. `None — documentation-only capstone; verification is command-based (doc-link run + diff inspection) per the commands below.`

### Commands

1. `node scripts/check-doc-links.mjs`
2. `git diff --name-only` (must list only doc surfaces — the docs-only boundary proof)
3. A narrower command set is correct because the capstone reconciles the spec index
   and records the handoff; the shipped browser surface and its tests are owned by
   Gate 9, and the README prose is verified by GAT9AFTROAREA-001.

## Outcome

Completed: 2026-06-08

What changed:

- Added the optional `7M` Gate 9 aftermath / web README realignment row to `specs/README.md` with status `Done`.
- Preserved the existing Gate 9 row unchanged.
- Kept the related Gate 9 aftermath spec clarification that preserves the `apps/web/README.md` Rust/WASM behavior-authority sentence in the implementation reference.

Deviations from original plan:

- The working tree already contained a related `specs/gate-9-aftermath-roadmap-realignment.md` clarification before this capstone edit. It was kept as documentation-only context for the completed README change.

Verification results:

- `node scripts/check-doc-links.mjs` passed: `Checked 26 markdown files`.
- `grep -n "7M" specs/README.md` showed the Gate 9 aftermath row.
- `grep -n "| 7 | Gate 9 |" specs/README.md` showed the original Gate 9 row still present and unchanged.
- `git diff --name-only` listed only documentation surfaces for this capstone: `specs/README.md` and `specs/gate-9-aftermath-roadmap-realignment.md`; no gameplay, Rust crate, WASM, tool, CI, trace, fixture, benchmark, or archived-spec file was touched.

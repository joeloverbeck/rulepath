# PLP1RDY-005: Governance area docs — roadmap note, IP policy, agent discipline, docs map, archival

**Status**: COMPLETED
**Priority**: HIGH
**Effort**: Medium
**Engine Changes**: None — governance/area docs (`docs/ROADMAP.md`, `docs/IP-POLICY.md`, `docs/AGENT-DISCIPLINE.md`, `docs/README.md`, `docs/archival-workflow.md`)
**Deps**: PLP1RDY-001, PLP1RDY-002, PLP1RDY-003

## Problem

The accepted ADRs and the amended constitution need their governance surfaces
updated: the roadmap's ADR-0007-limited note, IP policy's sanctioned-lane section
+ no-leak checklist, agent-discipline's private-monster task law, the docs map's
ADR index, and the archival note for ADR-limited roadmap text. The spec bundles
these as WB-2's non-constitution edits (report `A-02`, `A-03`, `A-17`, `A-18`,
`A-19`). They land only after the three ADRs are accepted (the README ADR index
lists all three; the ROADMAP note is gated on ADR 0010 acceptance).

## Assumption Reassessment (2026-06-28)

1. Target docs verified present: `docs/ROADMAP.md`, `docs/IP-POLICY.md`,
   `docs/AGENT-DISCIPLINE.md`, `docs/README.md`, `docs/archival-workflow.md`.
   IP-POLICY's current §10-aligned private paragraph ("Private licensed/
   commercial-game stress tests are late, isolated, optional, non-public…") is the
   sanctioned-lane edit target (report `A-03`).
2. Spec source: `specs/private-lane-foundation-readiness.md` §Scope (in scope,
   foundation/area docs) + §Documentation-updates required; the README ADR index
   and ROADMAP ADR-0007-limited note are explicitly WB-2.
3. Cross-artifact boundary under audit: `docs/IP-POLICY.md` is **also** edited by
   PLP1RDY-007 (Part C repo doctrine) and `docs/ROADMAP.md` **also** by
   PLP1RDY-011 (Private Lane P1 section). To avoid a merge conflict on those two
   shared files, PLP1RDY-007 `Deps` this ticket and PLP1RDY-011 `Deps` this ticket;
   this ticket lands the WB-2 sections only.
4. FOUNDATIONS principle under audit (§10 IP conservatism / §11 isolation
   invariant): the IP-POLICY no-leak checklist + opaque-private-identifier rule
   reinforce — never relax — the "shipped to an unauthorized browser = shipped"
   rule; the AGENT-DISCIPLINE private-monster law keeps private decomposition
   bounded and reviewable (§11 agent-output invariant, §12 unbounded-scope stop).

## Architecture Check

1. Grouping the routine governance edits in one ticket keeps the constitution
   diff (PLP1RDY-004) isolated and gives reviewers one coherent area-doc diff.
2. No backwards-compatibility shim: the ROADMAP note *limits* ADR 0007's timing
   via an explicit pointer, consistent with PLP1RDY-001; nothing is aliased.
3. `engine-core` untouched (§3); these are doctrine/process docs only.

## Verification Layers

1. Each section present -> codebase grep-proof: ROADMAP ADR-0007-limited note,
   IP-POLICY sanctioned-lane section + no-leak checklist, AGENT-DISCIPLINE
   private-monster law, README ADR `0010`/`0011`/`0012` index lines, archival note.
2. Acceptance precondition -> grep `^Status: Accepted` across the three ADRs
   (README index must not list an unaccepted ADR).
3. Cross-artifact doc-link integrity (new ADR index links resolve) ->
   `node scripts/check-doc-links.mjs`.

## What to Change

### 1. `docs/ROADMAP.md`

Add the ADR-0007-limited note (Gate P timing limited, not superseded, by ADR 0010).

### 2. `docs/IP-POLICY.md`

Add the sanctioned-lane section (authorized-now/isolated-always) and the no-leak
checklist (opaque private-lane identifier; no licensed title/ID/card/e2e/fixture/
catalog string in public files), per report `A-03`.

### 3. `docs/AGENT-DISCIPLINE.md`

Add the private-monster task-discipline / decomposition law (report `A-17`).

### 4. `docs/README.md`

Add the ADR index entries for `0010`/`0011`/`0012` (report `A-18`).

### 5. `docs/archival-workflow.md`

Add the ADR-limited roadmap-text archival note (report `A-19`).

## Files to Touch

- `docs/ROADMAP.md` (modify)
- `docs/IP-POLICY.md` (modify)
- `docs/AGENT-DISCIPLINE.md` (modify)
- `docs/README.md` (modify)
- `docs/archival-workflow.md` (modify)

## Out of Scope

- FOUNDATIONS amendments (PLP1RDY-004).
- The Part C repo doctrine in IP-POLICY (PLP1RDY-007) and the Private Lane P1
  ROADMAP section (PLP1RDY-011) — those land in their own tickets to avoid
  shared-file conflict.

## Acceptance Criteria

### Tests That Must Pass

1. `grep -q '0010' docs/README.md && grep -q '0011' docs/README.md && grep -q '0012' docs/README.md` — ADR index complete.
2. `for a in 0010 0011 0012; do grep -q '^Status: Accepted' docs/adr/$a-*.md; done` — index lists only accepted ADRs.
3. `node scripts/check-doc-links.mjs` — ADR index links resolve.

### Invariants

1. The no-leak checklist + opaque-identifier rule reinforce, never relax, §10/§11
   IP isolation.
2. The ROADMAP note limits ADR 0007 timing only; ADR 0007 stays `Status: Accepted`.

## Test Plan

### New/Modified Tests

1. `None — governance/area docs; verification is command-based (section greps + doc-link gate) and existing pipeline coverage is named in Assumption Reassessment.`

### Commands

1. `grep -nE 'sanctioned|no-leak|private-monster' docs/IP-POLICY.md docs/AGENT-DISCIPLINE.md`
2. `node scripts/check-doc-links.mjs`
3. A narrower command suffices: docs-only edits, so section-presence greps + doc-link integrity are the correct verification boundary.

## Outcome

Completed: 2026-06-28

Updated the PLP1-RDY governance area docs:

- `docs/ROADMAP.md` now records that ADR 0010 limits accepted ADR 0007 for
  timing only, while public Gates 21-23 remain the public roadmap order and
  Gate P remains isolated, optional, non-public, and non-architectural.
- `docs/IP-POLICY.md` now describes the sanctioned private lane as
  authorized-now/isolated-always and adds a no-name/no-ID public no-leak
  checklist.
- `docs/AGENT-DISCIPLINE.md` now has private-monster decomposition law requiring
  bounded private-lane tasks with source/IP handling, public-boundary impact,
  no-leak surfaces, forbidden public changes, and exact verification commands.
- `docs/README.md` now lists accepted ADRs 0010, 0011, and 0012 and records the
  authority-map note for private-lane doctrine.
- `docs/archival-workflow.md` now records how to handle ADR-limited roadmap text
  without deleting older accepted ADR rationale.

Deviations from plan: none. The full Private Lane P1 roadmap/tracker content
remains out of scope for this ticket and is left to PLP1RDY-011. Existing
unrelated `.claude/skills/*` worktree changes were left untouched and unstaged.

Verification:

- `grep -nE 'sanctioned|no-leak|private-monster|0010|0011|0012|ADR-Limited Roadmap Text|ADR 0010|ADR 0007' docs/IP-POLICY.md docs/AGENT-DISCIPLINE.md docs/README.md docs/archival-workflow.md docs/ROADMAP.md`
  passed, confirming the requested governance sections and ADR index entries.
- `grep -nE '^Status: Accepted' docs/adr/0010-*.md docs/adr/0011-*.md docs/adr/0012-*.md`
  passed for all three prerequisite ADRs.
- `node scripts/check-doc-links.mjs` passed (`Checked 34 markdown files`).

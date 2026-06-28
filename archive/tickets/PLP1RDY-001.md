# PLP1RDY-001: Author and accept ADR 0010 — Sanctioned Parallel Private-Game Lane

**Status**: COMPLETED
**Priority**: HIGH
**Effort**: Medium
**Engine Changes**: None — new governance doc (`docs/adr/0010-sanctioned-parallel-private-game-lane.md`)
**Deps**: None

## Problem

Rulepath needs to host its first sanctioned **private licensed** game (Private
Lane P1) in parallel with the unfinished public ladder, but FOUNDATIONS §1
priority order ranks "later private stress tests" below the public ladder and
ADR 0007 places Gate P at the very tail. Changing the priority order is a §13
ADR trigger ("changing the priority order"). The spec
(`archive/specs/private-lane-foundation-readiness.md`, WB-1a) requires an **accepted**
ADR that sanctions a timing-only carve-out *before* any FOUNDATIONS/ROADMAP/IP
edit that operationalizes it. This ticket authors and accepts that ADR.

## Assumption Reassessment (2026-06-28)

1. The ADR directory and template exist: `docs/adr/ADR-TEMPLATE.md` is present,
   and the highest accepted ID is `docs/adr/0009-replay-fixture-hash-taxonomy.md`,
   so `0010` is the correct next integer (verified: `ls docs/adr/`).
2. The spec's verbatim Decision block is `archive/specs/private-lane-foundation-readiness.md`
   §5.1; the spec directs WB-1 to land it first (§Work breakdown: "WB-1 … must
   complete and be accepted before any other item"). Assumption A3 records that
   maintainer acceptance (`Status: Accepted`) is part of this unit.
3. Cross-artifact boundary under audit: ADR 0010 **limits** (does not supersede)
   accepted `docs/adr/0007-next-public-scaling-phase-and-gate-p-tail.md`'s
   Gate-P-tail *timing* while leaving its isolation/non-public/non-architectural
   intent intact (spec §Forbidden changes; report `A-19`). Confirmed ADR 0007 is
   `Status: Accepted` and its Decision keeps Gate P "private, optional, isolated,
   non-architectural, and non-public."
4. FOUNDATIONS principle under audit (§13 ADR triggers / §1 priority order):
   the carve-out changes priority-order **timing only** (priority item 5 may run
   in parallel with items 1–4 once authorized); it does not raise private work
   above public product quality and authorizes no private content in public
   surfaces or `engine-core`. The ADR must restate this before the FOUNDATIONS
   §1 edit (PLP1RDY-004) trusts it.

## Architecture Check

1. An accepted ADR is the FOUNDATIONS-mandated mechanism for a priority-order
   change (§13). Landing the constitution edit without it would cross a §13
   trigger; authoring the ADR first keeps the change auditable and supersedable.
2. No backwards-compatibility shim: the ADR *limits* ADR 0007's timing via an
   explicit partial-supersession note, not an alias or silent override.
3. `engine-core` is untouched (§3); the ADR explicitly forbids private content
   in `engine-core` and confines COIN nouns to the private game crate (§4 earned
   only by later public-safe evidence).

## Verification Layers

1. ADR exists with correct ID/status -> codebase grep-proof (`test -f
   docs/adr/0010-sanctioned-parallel-private-game-lane.md`; `grep '^Status: Accepted'`).
2. Decision matches the spec's verbatim §5.1 block -> manual review against
   `archive/specs/private-lane-foundation-readiness.md` §5.1.
3. Priority-order change is timing-only, no invariant weakened -> FOUNDATIONS
   alignment check (§1, §10, §11 isolation invariant preserved; §13 trigger satisfied).
4. ADR 0007 limited, not superseded -> manual review of the partial-supersession
   note + `docs/adr/0007-*.md` remains `Status: Accepted`.

## What to Change

### 1. Author `docs/adr/0010-sanctioned-parallel-private-game-lane.md`

From `docs/adr/ADR-TEMPLATE.md`, full section set (Context, Decision,
Alternatives considered, Consequences, the impact sections —
Determinism / Replay-hash / Visibility / Data-Rust-boundary /
`engine-core`-contamination / UI / Bot / IP / Benchmark —, Migration notes,
Review checklist). `Status: Accepted`, date 2026-06-28, next-integer ID after
`0009`. The Decision text is the verbatim block from spec §5.1. The ADR names
the amended FOUNDATIONS/ROADMAP/IP-POLICY sections and flags the constitution
supersession, and records that it *limits* (not supersedes) ADR 0007's Gate-P
tail for timing only.

### 2. Acceptance

Record `Status: Accepted` (maintainer act, Assumption A3). Downstream tickets
(PLP1RDY-004/005/008/011) gate on this `Accepted` status.

## Files to Touch

- `docs/adr/0010-sanctioned-parallel-private-game-lane.md` (new)

## Out of Scope

- Any FOUNDATIONS / ROADMAP / IP-POLICY edit (PLP1RDY-004, -005, -011).
- ADR 0011 and ADR 0012 (PLP1RDY-002, -003).
- Creating the private repository or any private implementation.

## Acceptance Criteria

### Tests That Must Pass

1. `test -f docs/adr/0010-sanctioned-parallel-private-game-lane.md && grep -q '^Status: Accepted' docs/adr/0010-sanctioned-parallel-private-game-lane.md`
2. `node scripts/check-doc-links.mjs` — no broken links introduced by the ADR.
3. `grep -q '0007' docs/adr/0010-sanctioned-parallel-private-game-lane.md` — the limited-not-superseded note references ADR 0007.

### Invariants

1. The ADR changes priority-order **timing only**; no §11 isolation, no-leak,
   determinism, or v1/v2 bot-ban invariant is weakened.
2. ADR 0007 remains `Status: Accepted` and is limited, not superseded.

## Test Plan

### New/Modified Tests

1. `None — governance/ADR doc; verification is command-based (doc-link gate + status grep) and existing pipeline coverage is named in Assumption Reassessment.`

### Commands

1. `grep -nE '^Status: Accepted' docs/adr/0010-sanctioned-parallel-private-game-lane.md`
2. `node scripts/check-doc-links.mjs`
3. A narrower command suffices: this ADR ships no code, so doc-link integrity + the status grep are the correct verification boundary.

## Outcome

Completed: 2026-06-28

Added `docs/adr/0010-sanctioned-parallel-private-game-lane.md` as an accepted
ADR using the repository ADR template. The ADR creates the sanctioned parallel
private-game lane as a timing-only carve-out, explicitly limits accepted ADR
0007 only for Gate-P-tail timing, preserves the private isolation and
non-public/non-architectural requirements, and names the downstream PLP1-RDY
foundation, roadmap, IP-policy, agent-discipline, and spec-index updates.

Deviations from plan: none. No Rust, web, CI, catalog, fixture, trace, replay,
hash, RNG, or private implementation files changed.

Verification:

- `grep -nE '^Status: Accepted' docs/adr/0010-sanctioned-parallel-private-game-lane.md`
  passed (`3:Status: Accepted`).
- `grep -n '0007' docs/adr/0010-sanctioned-parallel-private-game-lane.md`
  passed and confirms the limited-not-superseded note is present.
- `node scripts/check-doc-links.mjs` passed (`Checked 32 markdown files`).

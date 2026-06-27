# GAT20STACROSTA-003: forward-v1 reuse-first scaffolding audit + implementation admission

**Status**: COMPLETED
**Priority**: HIGH
**Effort**: Medium
**Engine Changes**: None — game-local governance docs (`games/starbridge_crossing/docs/MECHANICS.md`, `GAME-IMPLEMENTATION-ADMISSION.md`)
**Deps**: none

## Problem

`docs/FOUNDATIONS.md` §11 makes a completed mechanical-scaffolding reuse-first audit a universal acceptance invariant for every new official game, and §12 makes starting serious implementation without it a stop condition. Starbridge Crossing is the third `forward-v1` user (after Blackglass Pact and Meldfall Ledger). This ticket authors the C-01…C-10 reuse-first audit and the implementation-admission doc that **blocks** the crate skeleton (GAT20STACROSTA-004 `Deps` on it).

## Assumption Reassessment (2026-06-27)

1. `docs/MECHANICAL-SCAFFOLDING-REGISTER.md` uses C-01…C-10 checkpoints (MSC-8C-001…010: effect envelopes, seat grammar, seat-range/ring arithmetic, action-tree encoding/hash, stable-byte writer, dev-only test support, no-leak assertion geometry, evidence-profile drivers, bounded-index sampling, Non-Promotion behavioral bundle) — confirmed against the register.
2. `ci/scaffolding-audits.json` has `coverage: "forward-v1"` for exactly `blackglass_pact` and `meldfall_ledger` — confirmed; Starbridge is the third. The machine receipt itself lands in the closeout (GAT20STACROSTA-019), not here.
3. Cross-artifact boundary: the C-01…C-10 reuse review against `docs/MECHANICAL-SCAFFOLDING-REGISTER.md` and the lawful shared homes (`crates/game-stdlib/src/seat.rs` ring/seat-count helpers, the engine-core effect/action-tree/visibility/replay contracts). `crates/game-stdlib/src/seat.rs` exports `SeatCount`, `SeatCountRange`, `next_ring_index`, `checked_index` — reusable for `{2,3,4,6}`.
4. §11 reuse-first-audit invariant and §12 stop condition motivate this ticket; the audit must reuse matching promoted scaffolding (seat helpers, effect/action-tree framing, replay/hash bytes, evidence profiles) and **reject** classifying board topology / path-jump legality as scaffolding (C-10 Non-Promotion bundle).
5. Mechanical-scaffolding gate (§11/ADR 0008): the enforcement surface is `docs/MECHANICAL-SCAFFOLDING-REGISTER.md` + `ci/scaffolding-audits.json` (checked by `scripts/check-scaffolding-governance.mjs` in the closeout). Confirm the admission doc names reuse decisions per checkpoint and identifies any new behavior-free plumbing as `candidate`/`local-only`, never encoding legality/scoring/visibility policy.

## Architecture Check

1. Authoring the audit + admission doc before code makes scaffolding reuse a precondition rather than a retrofit, matching the 8F/forward-v1 cadence and preventing graph-behavior-as-scaffolding misclassification.
2. No backwards-compatibility shims; governance docs only.
3. Confirms `engine-core` stays noun-free and `game-stdlib` is reused (not grown) for seat arithmetic; topology/path legality is reaffirmed game-local (C-10).

## Verification Layers

1. Reuse-first audit completeness (§11) -> manual review: `MECHANICS.md` records a disposition for each C-01…C-10 checkpoint.
2. No behavior-as-scaffolding (§4/§11) -> FOUNDATIONS alignment check: C-10 reaffirms graph/topology/path/jump/scoring/bot policy as game-local.
3. Admission blocks code -> cross-ticket: GAT20STACROSTA-004 `Deps: 003`.
4. Single-artifact governance doc otherwise — the register/receipt machine artifact is deferred to the closeout (GAT20STACROSTA-019), so no `ci/scaffolding-audits.json` edit here.

## What to Change

### 1. Author the C-01…C-10 reuse-first audit in `games/starbridge_crossing/docs/MECHANICS.md`

Per-checkpoint disposition: C-02/C-03 reuse `seat`/seat-grammar helpers where they fit (home/target assignment + finish-skipping stay game-local); C-01/C-04/C-05 reuse effect/action-tree/stable-byte framing (move/jump/finish meanings are game behavior); C-06/C-07/C-08/C-09 reuse test/no-leak/catalog/evidence shapes; C-10 reaffirm the Non-Promotion behavioral bundle. Expected register-new result: none.

### 2. Author `games/starbridge_crossing/docs/GAME-IMPLEMENTATION-ADMISSION.md`

Record that serious implementation is admitted once the topology hard gate (GAT20STACROSTA-002) and this forward-v1 audit are resolved and no forbidden boundary change is required.

## Files to Touch

- `games/starbridge_crossing/docs/MECHANICS.md` (new)
- `games/starbridge_crossing/docs/GAME-IMPLEMENTATION-ADMISSION.md` (new)

## Out of Scope

- The `ci/scaffolding-audits.json` `forward-v1` receipt + register reconciliation (closeout GAT20STACROSTA-019).
- The §4 topology ledger (GAT20STACROSTA-002) — separate lane.
- Any crate code.

## Acceptance Criteria

### Tests That Must Pass

1. `grep -nE "C-0[1-9]|C-10" games/starbridge_crossing/docs/MECHANICS.md` — all ten checkpoints addressed.
2. `node scripts/check-doc-links.mjs`
3. `bash scripts/boundary-check.sh`

### Invariants

1. Crate skeleton does not begin until this admission doc and the topology ledger exist (§12).
2. No checkpoint classifies topology/path-jump legality as behavior-free scaffolding.

## Test Plan

### New/Modified Tests

1. `None — documentation-only ticket; verification is command-based and existing pipeline/test coverage is named in Assumption Reassessment.`

### Commands

1. `node scripts/check-doc-links.mjs`
2. `bash scripts/boundary-check.sh`
3. Narrower command is correct: governance docs only; `check-scaffolding-governance.mjs` runs once the machine receipt lands in GAT20STACROSTA-019.

## Outcome

Completed: 2026-06-27

What changed:

- Added `games/starbridge_crossing/docs/MECHANICS.md` with the game-local
  mechanic inventory, primitive-pressure posture, C-01 through C-10 forward-v1
  scaffolding reuse-first audit, lawful shared homes review, expected
  `no-new-scaffolding` admission disposition, and no-follow-on-unit expectation
  unless later implementation invents a pure scaffolding match.
- Added `games/starbridge_crossing/docs/GAME-IMPLEMENTATION-ADMISSION.md` with
  the pre-code admission receipt, authority references, source/rule readiness,
  topology hard-gate evidence, C-01 through C-10 audit summary, boundary risks,
  required evidence profile, and explicit constraints for later crate work.

Deviations from plan:

- None.

Verification:

- `grep -nE "C-0[1-9]|C-10" games/starbridge_crossing/docs/MECHANICS.md`
  passed and showed all ten checkpoints.
- `node scripts/check-doc-links.mjs` passed (`Checked 31 markdown files`).
- `bash scripts/boundary-check.sh` passed (`engine-core boundary check passed`;
  `game-test-support dev-only boundary check passed`).
- `git diff --check` passed.

Unrelated worktree changes left untouched:

- `.claude/skills/spec-to-tickets/SKILL.md`

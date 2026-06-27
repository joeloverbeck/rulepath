# GAT20STACROSTA-002: Topology/path-jump third-use primitive-pressure ledger + atlas decision

**Status**: PENDING
**Priority**: HIGH
**Effort**: Medium
**Engine Changes**: None — governance docs (`games/starbridge_crossing/docs/PRIMITIVE-PRESSURE-LEDGER.md`, `docs/MECHANIC-ATLAS.md`)
**Deps**: none

## Problem

Starbridge Crossing is the third official graph/topology/path-jump pressure after Frontier Control and Event Frontier. `docs/FOUNDATIONS.md` §4 makes the third use a **hard gate**: the primitive-pressure ledger must record reuse / promote / defer-reject / ADR-escalation before the game proceeds (§12 stop condition: "a third repeated mechanic proceeds without a ledger decision"). This ticket resolves the gate so the crate skeleton (GAT20STACROSTA-004) may begin.

## Assumption Reassessment (2026-06-27)

1. `crates/game-stdlib/src/board_space.rs` is rectangular-only ("Rule-agnostic rectangular board-space helpers": `Dimensions`, `Coord`, `RowMajor`, `rNcM` parse/format, parity) — confirmed; it cannot model a masked non-rectangular six-point star, so it is audited **not applicable**.
2. `docs/MECHANIC-ATLAS.md` §10A "Open promotion-debt register" currently reads `Current debt: None` (last reviewed Gate 19 closeout 2026-06-26) — confirmed; this ticket must keep §10A empty (decision is defer/reject, creating no debt).
3. Cross-artifact boundary under audit: the §4 mechanic-atlas third-use ledger and `games/frontier_control/docs/PRIMITIVE-PRESSURE-LEDGER.md` / `games/event_frontier/docs/PRIMITIVE-PRESSURE-LEDGER.md` prior-graph entries; this ledger compares Frontier Control (named site/edge maps), Event Frontier (event/faction graph), and Starbridge (geometric hex-like coords + opposite-home triangles + multi-hop legality).
4. §4 (`game-stdlib` earned) and the §12 third-use stop condition motivate this ticket: the spec (reassessed) locks the decision **defer/reject promotion** — no graph/path/jump helper, no `engine-core` noun, topology stays game-local typed content with Rust-owned legality.
5. Third-use mechanic hard gate (§4): the enforcement surface is `docs/MECHANIC-ATLAS.md` + this game's `PRIMITIVE-PRESSURE-LEDGER.md`. Confirm the decision introduces no `game-stdlib` graph/path/jump helper and no §10A promotion debt; record the Gate 21 (Pachisi track topology) reopen trigger so the comparison value is preserved.

## Architecture Check

1. Resolving the hard gate as defer/reject keeps topology/path legality in game-local Rust, preserving trace stability, replay clarity, and a clean comparison point for Gate 21 — superior to an anemic shared coordinate iterator that would still need game-owned metadata + legality policy.
2. No backwards-compatibility shims; no helper extracted.
3. `engine-core` stays noun-free (no board/space/peg/graph/path noun added); `game-stdlib` is unchanged — the earned-promotion bar is explicitly not met (§3/§4).

## Verification Layers

1. Third-use gate resolved (§4) -> FOUNDATIONS alignment check: ledger records compare-vs-prior + decision defer/reject + reopen trigger.
2. No promotion debt (§11/§10A) -> codebase grep-proof: `grep -n "Current debt" docs/MECHANIC-ATLAS.md` still reads `None`; no Starbridge row added to the debt table.
3. `board_space` N/A -> manual review against `crates/game-stdlib/src/board_space.rs` module doc (rectangular scope).
4. No `game-stdlib` graph helper introduced -> `git diff --stat crates/game-stdlib` is empty for this ticket.

## What to Change

### 1. Author `games/starbridge_crossing/docs/PRIMITIVE-PRESSURE-LEDGER.md`

Record: `board_space` audit → not applicable (rectangular scope vs masked star); prior-graph comparison (Frontier Control, Event Frontier) → related but too small a shared part for a behavior-free helper; **decision: defer/reject promotion**; no prior-game conformance required; no §10A debt; reopen trigger = Gate 21 Pachisi-family race must compare its track topology/capture/safety against Frontier Control, Event Frontier, and Starbridge before any helper proposal.

### 2. Update `docs/MECHANIC-ATLAS.md`

Add the Gate 20 graph/topology/path-jump third-use decision (defer/reject, no helper, no `engine-core` noun); record `board_space` N/A; keep §10A `Current debt: None`; note the Gate 21 reopen trigger.

## Files to Touch

- `games/starbridge_crossing/docs/PRIMITIVE-PRESSURE-LEDGER.md` (new)
- `docs/MECHANIC-ATLAS.md` (modify)

## Out of Scope

- Any `game-stdlib` extraction (the decision is defer/reject; no helper this gate).
- The forward-v1 scaffolding audit (GAT20STACROSTA-003) — parallel, distinct governance lane.
- Crate code (later tickets).

## Acceptance Criteria

### Tests That Must Pass

1. `grep -n "Current debt: None\|_None_" docs/MECHANIC-ATLAS.md` — §10A stays empty.
2. `node scripts/check-doc-links.mjs` — ledger + atlas links resolve.
3. `bash scripts/boundary-check.sh` — no forbidden `engine-core` noun introduced.

### Invariants

1. The third-use hard gate is resolved before the crate skeleton lands (§4/§12).
2. No `game-stdlib` helper and no §10A promotion debt result from this decision.

## Test Plan

### New/Modified Tests

1. `None — documentation-only ticket; verification is command-based and existing pipeline/test coverage is named in Assumption Reassessment.`

### Commands

1. `node scripts/check-doc-links.mjs`
2. `bash scripts/boundary-check.sh`
3. Narrower command is correct: governance docs only; the boundary noun-check and doc-link gates are the relevant verification, no crate builds here.

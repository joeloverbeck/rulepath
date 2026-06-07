# GAT7DRALITCOM-002: Primitive-pressure reopen decision & ledger/atlas update

**Status**: PENDING
**Priority**: HIGH
**Effort**: Medium
**Engine Changes**: None — decision/documentation only (`games/draughts_lite/docs/PRIMITIVE-PRESSURE-LEDGER.md`, `docs/MECHANIC-ATLAS.md`). Outcome gates GAT7DRALITCOM-003 (`game-stdlib`) and informs GAT7DRALITCOM-005.
**Deps**: None

## Problem

FOUNDATIONS §4 makes board-space helper promotion an earned decision recorded through the mechanic atlas, and `docs/MECHANIC-ATLAS.md` already carries a standing Gate 6 as-built decision of `rejected/deferred with rationale` for the "fixed 2D occupancy" and "coordinate/targeted placement" shapes. Draughts Lite is the next official spatial game, which the atlas names as a reopen trigger. This ticket performs the **reopen-and-decide**: it records the decision (promote a narrow rule-agnostic board-space primitive, or defer/reject) with evidence, and supersedes the affected atlas rows. The decision must land before the crate skeleton (GAT7DRALITCOM-004), because the §4 hard gate must resolve before the game proceeds.

## Assumption Reassessment (2026-06-07)

1. `crates/game-stdlib/src/lib.rs` is still a placeholder (`placeholder_version()`, 16 lines; no promoted helper). `crates/engine-core` carries the generic `ActionPath { segments: Vec<String> }` (`crates/engine-core/src/lib.rs:59`) — coordinate/board-space helpers are NOT in the kernel and must not enter it.
2. `docs/MECHANIC-ATLAS.md` (repository-level primitive-pressure law) records `rejected/deferred with rationale` for "fixed 2D occupancy", "coordinate/targeted placement", "simple line/pattern detection", and a `local-only` row for "movement/capture/forced continuation" (`draughts_lite`, "Keep game-local until repeated"). The reopen conditions are stated verbatim: reopen "if another repeated direct-cell game proves one stable coordinate helper without origin/order flags" and "if a post-Gate 6 audit proves one narrow behavior-free helper without trace/hash migration". The spec §R12 frames this ticket as the reopen.
3. Cross-artifact boundary under audit: `docs/MECHANIC-ATLAS.md` is the shared primitive-pressure contract for the whole repo; `games/directional_flip/docs/PRIMITIVE-PRESSURE-LEDGER.md` is the per-game ledger precedent. The decision token recorded here is consumed by GAT7DRALITCOM-003 (conditional extraction) and GAT7DRALITCOM-005 (local fallback).
4. FOUNDATIONS §4 (`game-stdlib` is earned) motivates this ticket: restate before deciding — promotion is warranted ONLY if the helper is rule-agnostic (no draughts vocabulary), carries no origin/order policy, and forces no trace/hash migration on the three admitted games. Otherwise defer/reject remains the live outcome.
5. Third-use mechanic hard gate (§4) enforcement surface: the atlas ledger row is the gate. Confirm the decision (a) introduces no hidden-information path (board geometry is public, perfect-information) and (b) preserves deterministic replay/hash — a promoted coordinate helper must not change any existing game's serialization or trace output (the "without trace/hash migration" reopen constraint), which this ticket asserts as the decision criterion handed to GAT7DRALITCOM-003.

## Architecture Check

1. Recording the decision in the atlas + per-game ledger BEFORE any crate code is the §4 hard-gate discipline — it prevents an unearned promotion landing silently inside the implementation tickets.
2. No backwards-compatibility shims; the atlas rows are superseded (rewritten with the Gate 7 outcome), not aliased.
3. `engine-core` stays noun-free (§3) — any promoted primitive targets `game-stdlib`, never the kernel; the decision explicitly forbids draughts vocabulary in the helper (§4).

## Verification Layers

1. Decision recorded -> `docs/MECHANIC-ATLAS.md` grep-proof: the "fixed 2D occupancy", "coordinate/targeted placement", and "movement/capture/forced continuation" rows are updated with the Gate 7 outcome (superseded, not appended).
2. Per-game ledger present -> `games/draughts_lite/docs/PRIMITIVE-PRESSURE-LEDGER.md` exists with evidence, decision, limits, and retrofit policy (spec §R12).
3. §4 alignment -> FOUNDATIONS alignment check: the decision cites the reopen constraints (rule-agnostic, no origin/order flags, no trace/hash migration) and names the retrofit policy (no forced retrofit of `three_marks`/`column_four`/`directional_flip`).
4. Doc-link integrity -> `node scripts/check-doc-links.mjs`.

## What to Change

### 1. `PRIMITIVE-PRESSURE-LEDGER.md`

Author from `templates/PRIMITIVE-PRESSURE-LEDGER.md`: state the reopen of the standing Gate 6 deferral, the evidence (four rectangular-board games now exist; the coordinate/bounds/offset/row-major-iteration/`rNcM`-id shape recurs), the decision (promote a minimal rule-agnostic board-space primitive into `game-stdlib`, OR defer/reject with rationale), the explicit limits (no move generation, captures, promotion, occupancy policy, piece identity, win detection, gravity/flip, UI, WASM, bot heuristics, or playable-square policy), and the retrofit policy (no forced retrofit of admitted games).

### 2. `docs/MECHANIC-ATLAS.md`

Supersede the standing rows: update "fixed 2D occupancy", "coordinate/targeted placement", and the `draughts_lite` "movement/capture/forced continuation" row to reflect the Gate 7 outcome and the boundary line between rule-agnostic board-space helpers (promotable) and draughts mechanics (game-local). Keep the line/pattern-detection rows local unless the decision explicitly changes them.

## Files to Touch

- `games/draughts_lite/docs/PRIMITIVE-PRESSURE-LEDGER.md` (new)
- `docs/MECHANIC-ATLAS.md` (modify)

## Out of Scope

- Implementing the `game-stdlib` helper itself (GAT7DRALITCOM-003, conditional on this decision).
- Any draughts rules logic (stays in `games/draughts_lite/src/rules.rs`, GAT7DRALITCOM-005).
- Retrofitting `three_marks` / `column_four` / `directional_flip` to any new primitive (forbidden this gate; spec §R12 retrofit policy).

## Acceptance Criteria

### Tests That Must Pass

1. `node scripts/check-doc-links.mjs` — passes.
2. Manual review: the atlas rows are superseded (not duplicated), and the ledger states decision + evidence + limits + retrofit policy.

### Invariants

1. Any promotion is rule-agnostic and `game-stdlib`-only; `engine-core` gains no board/coordinate noun (FOUNDATIONS §3).
2. The decision preserves deterministic replay/hash for existing games — no trace/hash migration is triggered (FOUNDATIONS §11/§13; atlas reopen constraint).

## Test Plan

### New/Modified Tests

1. `None — decision/documentation-only ticket; the helper's tests (if promoted) land in GAT7DRALITCOM-003 and existing-game stability is asserted there.`

### Commands

1. `node scripts/check-doc-links.mjs`
2. `git diff --stat docs/MECHANIC-ATLAS.md` — confirm the standing rows are edited in place (superseded), not appended below.
3. A docs/decision ticket's correct verification boundary is link integrity + a manual atlas-row diff review; code-level proof belongs to GAT7DRALITCOM-003.

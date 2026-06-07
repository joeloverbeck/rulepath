# GAT71BOASPA-004: Record `race_to_n` board-space not-applicable audit

**Status**: PENDING
**Priority**: MEDIUM
**Effort**: Small
**Engine Changes**: None — documentation-only (`games/race_to_n/docs/MECHANICS.md`); confirms `game-stdlib` stays unused for `race_to_n`. No Rust, schema, trace, or bot surface changes.
**Deps**: None

## Problem

The `game-stdlib::board_space` open promotion-debt register (`docs/MECHANIC-ATLAS.md` §10A) can only be cleared once `race_to_n` is explicitly audited and recorded as **not applicable** to the board-space primitive (spec `specs/gate-7-1-board-space-primitive-back-port.md` §12; atlas §10A "`race_to_n` must be audited and recorded as not applicable because it has no board-space mechanic"). `race_to_n` is a numeric counter race with no spatial topology, so the correct outcome is an audit/no-op: no `game-stdlib` dependency is added and no counter state is refactored. This ticket adds the explicit, discoverable audit note so the capstone (GAT71BOASPA-005) can clear the register.

## Assumption Reassessment (2026-06-07)

1. `games/race_to_n/Cargo.toml` does NOT depend on `game-stdlib` (verified — the dep grep returns 0), and `games/race_to_n/src/` has no `board`/cell/coordinate types. `games/race_to_n/docs/MECHANICS.md` already records the relevant facts: "No spatial topology; one public numeric counter" (`:25`), "no entities move, capture, or occupy spaces … No board-like mechanic" (`:32`), and "`game-stdlib` remains unused" (`:64`). The remaining gap is a single explicit reference tying that to the `board_space` promotion-debt audit.
2. Spec §12.2 directs "add or update documentation evidence only if needed so the board-space promotion debt register can mark `race_to_n` not applicable"; spec §15 directs `games/race_to_n/docs/MECHANICS.md` to "add or preserve an audit note that the board-space primitive is not applicable." `docs/MECHANIC-ATLAS.md` §10A (`:199`) names `race_to_n` as the audit-pending entry.
3. Cross-artifact boundary under audit: `games/race_to_n/docs/MECHANICS.md` (audit evidence) ↔ `docs/MECHANIC-ATLAS.md` §10A register (consumer of that evidence). This ticket authors the evidence; the capstone consumes it to clear the register — no atlas edit happens here.
4. FOUNDATIONS §4 (`game-stdlib` is earned) and §12 stop condition ("a promoted primitive leaves matching prior official games un-migrated without an explicit exception or recorded closure gate") motivate the audit: recording not-applicable is the correct closure for a game with no matching mechanic, and spec §12.2 / §17.2 explicitly forbid adding a pointless `game-stdlib` dependency "for symmetry."
5. Mismatch + correction: the spec's §12 framing implies fresh audit work, but the codebase shows the not-applicable evidence is largely already present in `MECHANICS.md`. Scope is therefore narrowed to adding one explicit `board_space` promotion-debt audit line referencing the §10A register — not re-authoring the mechanics table.

## Architecture Check

1. An explicit `board_space`-not-applicable line makes the audit outcome discoverable from the game's own docs (the spec's "without chat memory" goal), rather than leaving a future reader to infer it from scattered "no board-like mechanic" rows. This is cleaner than recording the audit only in the atlas, because the per-game doc is where a maintainer checks a specific game's primitive obligations.
2. No backwards-compatibility aliasing/shim — documentation-only, no code path.
3. `engine-core` and `game-stdlib` are untouched; the ticket's whole point is to confirm `game-stdlib` stays unused for `race_to_n` (§4).

## Verification Layers

1. Explicit not-applicable audit evidence exists in the game doc → docs grep-proof: `grep -i "board_space" games/race_to_n/docs/MECHANICS.md` returns the new audit line.
2. No accidental dependency or code change → codebase grep-proof: `grep -c "game-stdlib" games/race_to_n/Cargo.toml` returns `0`; `cargo run -p replay-check -- --game race_to_n --all` passes unchanged (behavior untouched).
3. Audit conclusion aligns with the primitive-pressure contract → FOUNDATIONS §4 / §12 alignment check (no `game-stdlib` added "for symmetry").

## What to Change

### 1. Add the explicit board-space audit note

In `games/race_to_n/docs/MECHANICS.md`, add (or extend an existing row with) an explicit audit statement that the `game-stdlib::board_space` primitive is **not applicable** to `race_to_n` because it has no board-space mechanic, and that `game-stdlib` therefore remains unused — phrased so the `docs/MECHANIC-ATLAS.md` §10A register can cite it as the audit evidence for closing the `board_space` debt.

## Files to Touch

- `games/race_to_n/docs/MECHANICS.md` (modify)

## Out of Scope

- Adding a `game-stdlib` dependency to `games/race_to_n/Cargo.toml` (spec §12.2, §17.2 — explicitly forbidden).
- Refactoring `race_to_n` counter state into any generic primitive (§12.2).
- Editing `docs/MECHANIC-ATLAS.md` §10A or `specs/README.md` — the register/index updates belong to the capstone (GAT71BOASPA-005).
- Any change to `race_to_n` rules, traces, effects, or bots.

## Acceptance Criteria

### Tests That Must Pass

1. `grep -i "board_space" games/race_to_n/docs/MECHANICS.md` returns the new not-applicable audit line.
2. `grep -c "game-stdlib" games/race_to_n/Cargo.toml` returns `0` (no dependency added).
3. `cargo run -p replay-check -- --game race_to_n --all` passes unchanged (six golden traces intact).
4. `node scripts/check-doc-links.mjs` passes (doc link integrity).

### Invariants

1. `race_to_n` is explicitly documented as not applicable to `board_space` and does not gain a `game-stdlib` dependency.
2. No `race_to_n` behavior, trace, or hash changes.

## Test Plan

### New/Modified Tests

1. `None — documentation-only ticket; verification is command-based and existing pipeline/trace coverage is named in Assumption Reassessment.`

### Commands

1. `grep -i "board_space" games/race_to_n/docs/MECHANICS.md && grep -c "game-stdlib" games/race_to_n/Cargo.toml`
2. `cargo run -p replay-check -- --game race_to_n --all && node scripts/check-doc-links.mjs`
3. A narrower command is correct here because the ticket changes only prose: the grep proves the audit line landed, and `replay-check` proves no behavioral drift; no `cargo test` delta exists for a docs-only change.

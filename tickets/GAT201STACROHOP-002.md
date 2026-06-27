# GAT201STACROHOP-002: Add rule SC-MOVE-010 (origin-return prohibition) + coverage

**Status**: PENDING
**Priority**: MEDIUM
**Effort**: Small
**Engine Changes**: None — game-local docs (`games/starbridge_crossing/docs/RULES.md`, `RULE-COVERAGE.md`)
**Deps**: GAT201STACROHOP-001

## Problem

The origin-return prohibition implemented in `GAT201STACROHOP-001` needs a
governing rule of record. It is a distinct **turn-model** constraint — a
committed turn must change board occupancy unless Rust issues `pass_blocked`
(`SC-TURN-002` + `SC-MOVE-009`) — not a finiteness clarification of `SC-MOVE-007`,
whose concern (uniformly across `RULES.md`, `RULE-COVERAGE.md`, and `SOURCES.md`
`SC-AMB-004`) is keeping cyclic hop chains' action trees finite by forbidding
re-visiting a landing already used. This ticket adds a new rule `SC-MOVE-010`
and its coverage row, leaving `SC-MOVE-007` and `SC-AMB-004` untouched.

## Assumption Reassessment (2026-06-27)

1. The behaviour `SC-MOVE-010` documents is implemented by the
   `legal_jump_landings` guard in `GAT201STACROHOP-001`; its covering test is
   that ticket's `tests/rules.rs::hop_chain_cannot_return_to_origin_space`. The
   move-rule ID space in `RULES.md` is currently `SC-MOVE-001..009` (verified:
   `grep -oE 'SC-MOVE-[0-9]+'` → 001–009, with 008 present), so the next free ID
   is `SC-MOVE-010`.
2. `RULES.md` carries the move rule in two tables — the move table
   (`SC-MOVE-007` at `RULES.md:108`, `SC-MOVE-009` at `:110`) and the
   ambiguity/resolution table (`SC-MOVE-007` at `:178`, `SC-MOVE-009` at `:179`);
   `RULE-COVERAGE.md` carries `SC-MOVE-007` at `:40` and its trace row at `:74`.
   `SC-MOVE-007`'s anchor in `SOURCES.md` `SC-AMB-004` (`:109`) and both
   `SC-MOVE-007` rows stay **unchanged** — origin-return is not a finiteness
   concern.
3. Cross-artifact boundary under audit: `tools/rule-coverage` consumes
   `RULES.md` + `RULE-COVERAGE.md` for this game. The new `SC-MOVE-010` row must
   cite a covering artifact that already exists, so this ticket `Deps`
   `GAT201STACROHOP-001` (whose `tests/rules.rs` test is the cited evidence);
   landing the rule before that test exists would make `rule-coverage` red.
4. FOUNDATIONS §11 (a committed turn changes occupancy unless `pass_blocked`)
   and §6 (official games are evidence-heavy: every rule is coverage-mapped)
   motivate this ticket. `SC-MOVE-010` cross-references `SC-TURN-002` /
   `SC-MOVE-009`, whose intent the defect violated — it does not restate
   `SC-MOVE-007`.

## Architecture Check

1. A new `SC-MOVE-010` is cleaner than overloading `SC-MOVE-007`: the two
   concerns (turn-model net-displacement vs. action-tree finiteness) stay
   separable, and `SC-MOVE-007` + its `SC-AMB-004` source anchor remain a single
   coherent finiteness rule rather than a conflated pair.
2. No backwards-compatibility shims or alias paths introduced; purely additive
   rule text and one coverage row.
3. `engine-core` is untouched — game-local documentation only; no mechanic noun
   enters the kernel and no `game-stdlib` change is made (§3/§4).

## Verification Layers

1. `SC-MOVE-010` present in both `RULES.md` tables with a `SC-TURN-002` /
   `SC-MOVE-009` cross-reference and a Rule-ID Migration Note -> `grep` proof.
2. `RULE-COVERAGE.md` carries an `SC-MOVE-010` row citing the `-001` rules test ->
   `grep` proof + `rule-coverage` run.
3. `SC-MOVE-007` rows (`RULES.md`, `RULE-COVERAGE.md`) and `SOURCES.md`
   `SC-AMB-004` are unchanged -> `git diff` shows no edit to those lines.
4. `rule-coverage --game starbridge_crossing` reports `SC-MOVE-010` covered ->
   tool run (its validation surface is exactly `RULES.md` + `RULE-COVERAGE.md`).
5. Doc links/anchors intact -> `node scripts/check-doc-links.mjs`.

## What to Change

### 1. Add `SC-MOVE-010` to `RULES.md`

Add `SC-MOVE-010` to the move table and the ambiguity/resolution table:
"a hop chain may not land on the moving peg's own origin space; a committed turn
must change board occupancy unless Rust issues `pass_blocked`," cross-referencing
`SC-TURN-002` / `SC-MOVE-009`, with a Rule-ID Migration Note recording that this
narrows the Gate 20 action tree (ADR-0009-governed migration in
`GAT201STACROHOP-003`). Do not edit `SC-MOVE-007`.

### 2. Add the coverage row to `RULE-COVERAGE.md`

Add an `SC-MOVE-010` row citing
`tests/rules.rs::hop_chain_cannot_return_to_origin_space` (and the no-op-turn
property) as covering evidence, with the appropriate `covered-by-test` status.
Do not edit the `SC-MOVE-007` row or its trace row.

## Files to Touch

- `games/starbridge_crossing/docs/RULES.md` (modify)
- `games/starbridge_crossing/docs/RULE-COVERAGE.md` (modify)

## Out of Scope

- The `legal_jump_landings` guard and its tests (`GAT201STACROHOP-001`).
- Golden-trace / replay-fixture / benchmark regeneration and `GAME-EVIDENCE.md`
  refresh (`GAT201STACROHOP-003`).
- Any edit to `SC-MOVE-007` or `SOURCES.md` `SC-AMB-004` (the finiteness rule
  stays as-is); any new pass option, variant, or seat/piece count.

## Acceptance Criteria

### Tests That Must Pass

1. `cargo run -p rule-coverage -- --game starbridge_crossing` — reports
   `SC-MOVE-010` covered (and no regression on other rule IDs).
2. `node scripts/check-doc-links.mjs` passes.
3. `grep -n "SC-MOVE-010" games/starbridge_crossing/docs/RULES.md
   games/starbridge_crossing/docs/RULE-COVERAGE.md` returns the new rule +
   coverage row.

### Invariants

1. `SC-MOVE-007` rule text and the `SOURCES.md` `SC-AMB-004` anchor are
   byte-unchanged by this ticket.
2. Every `RULES.md` move-rule ID has a `RULE-COVERAGE.md` row (`rule-coverage`
   stays green).

## Test Plan

### New/Modified Tests

1. `None — documentation-only ticket; verification is command-based
   (`rule-coverage`, `check-doc-links`) and the covering test is
   `GAT201STACROHOP-001`'s `hop_chain_cannot_return_to_origin_space`, named in
   Assumption Reassessment.`

### Commands

1. `cargo run -p rule-coverage -- --game starbridge_crossing`
2. `node scripts/check-doc-links.mjs`
3. `rule-coverage` is the correct verification boundary here — it is the tool
   that consumes `RULES.md` + `RULE-COVERAGE.md`, so a green run proves the new
   rule is coverage-mapped without needing the full crate test suite.

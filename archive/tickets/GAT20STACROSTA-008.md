# GAT20STACROSTA-008: Jump-chain action-tree enumeration

**Status**: COMPLETED
**Priority**: MEDIUM
**Effort**: Large
**Engine Changes**: Yes — `games/starbridge_crossing/src/{rules.rs,actions.rs,effects.rs}` (hop/chain), `tests/rules.rs`
**Deps**: GAT20STACROSTA-007

## Problem

A move may be a chain of one or more hops over adjacent occupied spaces into the empty space beyond, changing direction after each landing, stopping at any legal landing, never revisiting a landing within the turn. This ticket lands the progressive jump-chain action-tree enumeration, the cycle guard, deterministic ordering, and grouped jump effects.

## Assumption Reassessment (2026-06-27)

1. Jump legality extends `src/rules.rs` (created by 007), reading adjacency/occupancy from `topology.rs`/`state.rs`; the hop rule (over exactly one adjacent occupied space into the empty beyond, same direction; jumped peg stays) is pinned from spec §3.
2. The chain path encoding `move/<peg-id>/jump/<landing-1>/continue/<landing-2>/.../stop` extends `actions.rs` (007); `stop` leaves are available after each landing per spec Appendix A.
3. Cross-artifact boundary: this modifies the rules/actions/effects files created in 007 (chain via `Deps: 007`); the cycle guard (no landing revisited within one turn) is a Rulepath finite-action-tree resolution, documented as an implementation resolution, not a universal table rule.
4. §2 (behavior authority) motivates this ticket: Rust enumerates the finite jump tree depth-first with the cycle guard and validates the accepted path; TypeScript never enumerates jumps or validates paths.
5. Deterministic replay (§11): the action-tree ordering (pegs seat-local, step leaves before jump roots, canonical six-direction order, depth-first continuations, `stop` after each landing) is the canonical enumeration the golden traces/hashes (011) depend on; confirm the cycle guard keeps the tree finite so enumeration is deterministic and bounded.

## Architecture Check

1. A progressive depth-first tree with a per-turn visited-set cycle guard keeps the legal set finite and replay-stable while supporting stop-anywhere and direction changes — cleaner than enumerating fixed-length chains.
2. No backwards-compatibility shims.
3. `engine-core` action-tree contract consumed generically; no `game-stdlib` graph/path helper (the 002 defer/reject decision holds).

## Verification Layers

1. Hop + multi-hop legality (§2) -> rule test: one-hop over an occupied peg to the empty beyond; multi-hop with direction change; jumped pegs remain.
2. Stop-anywhere + cycle guard -> rule test: chain may end after any hop; a chain revisiting a landing is rejected; no repeated landing in one turn.
3. Mixed step+jump rejection -> rule test: a single turn cannot mix a step and a hop.
4. Deterministic ordering + grouped effect -> unit test on canonical leaf order; grouped jump-chain effect shape (per-hop substeps).

## What to Change

### 1. Extend `src/rules.rs` (hop / chain)

Enumerate hop roots and depth-first continuations with the visited-landing cycle guard; expose `stop` leaves after each landing; reject mixed step+jump in one turn.

### 2. Extend `src/actions.rs`

`jump`/`continue`/`stop` path encode/parse; deterministic continuation ordering.

### 3. Extend `src/effects.rs`

Grouped jump-chain effect (ordered per-hop substeps; jumped-over spaces recorded) for one-turn renderer animation.

## Files to Touch

- `games/starbridge_crossing/src/rules.rs` (modify; created by 007)
- `games/starbridge_crossing/src/actions.rs` (modify; created by 007)
- `games/starbridge_crossing/src/effects.rs` (modify; created by 007)
- `games/starbridge_crossing/tests/rules.rs` (modify; created by 006 — add hop/chain cases)

## Out of Scope

- Finish/rank, blocked pass, turn-limit — GAT20STACROSTA-009.
- Golden jump traces — authored in GAT20STACROSTA-011.

## Acceptance Criteria

### Tests That Must Pass

1. `cargo test -p starbridge_crossing --test rules`
2. `cargo test -p starbridge_crossing`
3. `bash scripts/boundary-check.sh`

### Invariants

1. A jump chain never revisits a landing within one turn; the legal tree is finite and deterministically ordered (§11).
2. Step and hop are never mixed in one turn; jumped pegs are never removed.

## Test Plan

### New/Modified Tests

1. `games/starbridge_crossing/tests/rules.rs` — one-hop, multi-hop direction change, stop-midway, repeat-landing rejection, mixed step+jump rejection.
2. `games/starbridge_crossing/src/effects.rs` — inline grouped jump-chain effect test.

### Commands

1. `cargo test -p starbridge_crossing --test rules`
2. `cargo test -p starbridge_crossing && bash scripts/boundary-check.sh`
3. `--test rules` isolates jump-chain legality; full crate run confirms no step-path regression.

## Outcome

Completed: 2026-06-27

What changed:

- Extended `games/starbridge_crossing/src/actions.rs` with
  `move/<peg-id>/jump/<landing>/continue/<landing>/.../stop` encoding,
  parsing, mixed step/jump diagnostics, and progressive jump action-tree nodes.
- Extended `games/starbridge_crossing/src/rules.rs` with hop landing
  enumeration, dynamic chain occupancy, depth-first continuation validation,
  stop-anywhere validation, repeated-landing rejection, and jump application.
- Extended `games/starbridge_crossing/src/effects.rs` with a grouped public
  jump-chain effect carrying ordered per-hop `over` and landing substeps.
- Extended `games/starbridge_crossing/tests/rules.rs` with one-hop,
  jumped-peg-retention, direction-changing multi-hop, stop-midway,
  repeated-landing, and mixed step/jump rejection coverage.
- Updated `games/starbridge_crossing/src/lib.rs` exports for jump actions,
  rules, and effects.

Deviations from plan:

- None. Finish/rank, blocked pass, turn-limit, and golden jump traces remain
  deferred to their later tickets.

Verification:

- `cargo fmt --all --check` passed.
- `cargo test -p starbridge_crossing --test rules` passed: 12 integration
  tests.
- `cargo test -p starbridge_crossing` passed: 20 unit tests, 13 integration
  tests, 0 doctests.
- `bash scripts/boundary-check.sh` passed (`engine-core boundary check passed`;
  `game-test-support dev-only boundary check passed`).
- `git diff --check` passed.

Unrelated worktree changes left untouched:

- `.claude/skills/spec-to-tickets/SKILL.md`

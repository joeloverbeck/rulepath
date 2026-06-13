# DEADBRANCH-003: Assert no dead-branch action trees across all games in simulation

**Status**: COMPLETED
**Priority**: MEDIUM
**Effort**: Medium
**Engine Changes**: Yes — `tools/simulate` (`src/main.rs`): per-game step invariant. No `engine-core`/`game-stdlib`/`games/*` rule change.
**Deps**: DEADBRANCH-001, DEADBRANCH-002

## Problem

The dead-branch action-tree defect (DEADBRANCH-002) was invisible to the existing
test/simulation layer because the shared random-legal bot
(`crates/ai-core/src/random_legal.rs` `collect_choice`, lines 47-54) recurses into an
empty `next` node and pushes **zero** legal paths — so the bot never selects a dead
branch, and no simulation, replay, or bot-legality test ever exercises it. Only a
human descending the tree in the UI hits the incompletable `Confirm` and the
`malformed_action` rejection.

This means the bot-driven simulation harness structurally cannot catch this class of
bug as long as it asserts only over bot-selected paths. To prevent recurrence across
all current and future games, the simulation sweep must assert directly on the
**legal action tree** (not on flattened bot paths) at every reachable state that no
dead branch is present, using the generic primitive from DEADBRANCH-001.

The race_to_n simulation already has a per-step `check_invariants`
(`tools/simulate/src/main.rs:1817-1840`) that asserts a non-terminal state has a
non-empty root (`tree.root.choices.is_empty()`), but it does **not** check for dead
branches, and it is race_to_n-specific. This ticket generalizes the dead-branch
invariant across every game's per-step simulation loop.

## Assumption Reassessment (2026-06-13)

1. `tools/simulate/src/main.rs` dispatches per game via `run_<game>_simulation` /
   `run_one_<game>_game` functions (verified: `--game` parser at `:145`, usage list at
   `:211`, per-game runners at `:292-1404` and `run_one_event_frontier_game` at
   `:1507`). Each runner imports that game's `legal_action_tree` and drives the
   random-legal bot.
2. `crates/ai-core/src/random_legal.rs` `legal_paths`/`collect_paths`/`collect_choice`
   (verified lines 35-54) flattens the tree to leaf paths and silently omits empty
   `next` subtrees, so asserting over `legal_paths` output would **not** detect a dead
   branch. The assertion must run on the `ActionTree` itself
   (`tree.dead_branch_paths()` from DEADBRANCH-001).
3. Cross-crate boundary under audit: the simulate tool consumes each game's
   `legal_action_tree` (returning `engine_core::ActionTree`) and the
   `engine_core::ActionTree::dead_branch_paths` primitive (DEADBRANCH-001). This ticket
   adds a read-only assertion at the point each per-step tree is built; it changes no
   game rules and no schema.
4. FOUNDATIONS principles under audit: §6/§11 (official games are evidence-heavy;
   tests/simulations cover the change) and the testing law in
   `docs/TESTING-REPLAY-BENCHMARKING.md` (simulation as a verification layer). The
   dead-branch assertion is a deterministic structural invariant, not a probabilistic
   one, so it does not weaken determinism (§11).
8. Information-path note: the same fact ("is this tree well-formed?") is currently
   computed nowhere; DEADBRANCH-001 establishes the single canonical computation, and
   this ticket is its first enforcement consumer. No competing transport path exists.

## Architecture Check

1. Asserting on `tree.dead_branch_paths()` at the point each per-step tree is built is
   cleaner than (a) re-deriving dead-branch logic in the simulate tool, or (b) changing
   `ai-core::legal_paths` to surface the divergence (which would entangle the bot's
   selection contract with a diagnostic concern). It reuses the single canonical
   primitive (DEADBRANCH-001) and matches the existing per-step `check_invariants`
   pattern.
2. No backwards-compatibility aliasing/shims: net-new assertion.
3. `engine-core`/`game-stdlib` untouched; no mechanic nouns added in the tool. The
   assertion is generic over `ActionTree`.

## Verification Layers

1. Dead-branch invariant holds for every reachable state of every game -> simulation
   run per game (`cargo run -p simulate -- --game <id> --games <n>`) with the new
   assertion failing the run if any tree has a dead branch.
2. The assertion actually fires on a dead branch (anti-vacuous) -> a focused test that
   feeds a hand-built dead-branch tree (or a temporarily un-fixed fixture) through the
   shared assertion helper and confirms it reports failure.
3. No false positive on healthy nested trees -> the existing event_frontier,
   draughts_lite, masked_claims, plain_tricks simulations (which use nested choices)
   pass under the new assertion.

## What to Change

### 1. Shared per-step dead-branch assertion in the simulate tool

Add a small generic helper in `tools/simulate/src/main.rs`, e.g.
`assert_tree_well_formed(game_id, seed, action_index, &tree) -> Result<(), String>`,
that fails with a descriptive, replay-pointing message
(`tree.dead_branch_paths()` non-empty → include the offending segment path and a
`replay_command=...` line consistent with each runner's existing failure-report style).

### 2. Wire it into every game's per-step loop

At the point each `run_one_<game>_game` obtains the per-step `legal_action_tree` (the
same place the bot selects an action / where `check_invariants` runs for race_to_n),
invoke the shared assertion for **all 14 games**: race_to_n, three_marks, column_four,
directional_flip, draughts_lite, high_card_duel, masked_claims, flood_watch,
frontier_control, event_frontier, token_bazaar, secret_draft, poker_lite,
plain_tricks. For race_to_n, fold the dead-branch check into the existing
`check_invariants` rather than duplicating tree construction.

## Files to Touch

- `tools/simulate/src/main.rs` (modify — shared assertion helper + per-game wiring)

## Out of Scope

- The detection primitive itself (DEADBRANCH-001) and the Event Frontier rule fix
  (DEADBRANCH-002) — this ticket assumes both are merged so the full sweep is green.
- Changing `ai-core::legal_paths` or the bot selection contract.
- Adding dead-branch checks to `replay-check`/`fixture-check`/`wasm-api` (a possible
  future hardening; note it here rather than expanding scope).

## Acceptance Criteria

### Tests That Must Pass

1. The full simulation sweep is green for every game with the assertion active:
   `cargo run -p simulate -- --game <id> --games 1000` for each of the 14 game ids
   (at minimum the active per-game gate from `specs/README.md`, plus event_frontier).
2. Anti-vacuous test: a unit/integration test feeding a known dead-branch
   `ActionTree` through `assert_tree_well_formed` confirms it returns an error naming
   the offending path.
3. `cargo test -p simulate`, `cargo clippy --workspace --all-targets -- -D warnings`,
   `cargo test --workspace`.

### Invariants

1. Any game whose `legal_action_tree` produces a dead branch at any reached state
   fails its simulation run with a deterministic, replayable diagnostic.
2. Healthy flat and nested trees produce no false positives.

## Test Plan

### New/Modified Tests

1. `tools/simulate/src/main.rs` (tests) — `dead_branch_tree_is_rejected`
   (anti-vacuous): hand-built tree with an empty-`next` choice → assertion errors.
2. `tools/simulate/src/main.rs` (tests) — `well_formed_nested_tree_passes`: nested
   choice with reachable leaves → assertion ok.
3. Full per-game simulation runs (command-based) prove the invariant over reachable
   states.

### Commands

1. `cargo test -p simulate`
2. `for g in race_to_n three_marks column_four directional_flip draughts_lite high_card_duel masked_claims flood_watch frontier_control event_frontier token_bazaar secret_draft poker_lite plain_tricks; do cargo run -p simulate -- --game $g --games 1000 || break; done`
3. The simulate sweep is the correct boundary because the bug class is a property of
   the per-state legal action tree exercised across reachable states, which only the
   end-to-end simulation enumerates; unit tests cover the assertion helper itself.

## Outcome

Completed: 2026-06-13

Added a shared `assert_tree_well_formed` helper in `tools/simulate/src/main.rs`
that checks `ActionTree::dead_branch_paths()` and returns a deterministic simulation
failure block naming the offending path(s) and replay command.

Wired the helper into all 14 per-game simulation loops: `race_to_n`, `three_marks`,
`column_four`, `directional_flip`, `draughts_lite`, `high_card_duel`,
`masked_claims`, `flood_watch`, `frontier_control`, `event_frontier`,
`token_bazaar`, `secret_draft`, `poker_lite`, and `plain_tricks`. For `race_to_n`,
the check is folded into the existing `check_invariants` path.

Added simulate-tool tests proving the assertion rejects a hand-built dead-branch
tree and accepts a healthy nested tree.

Deviations from plan: none. The change is enforcement-only in `tools/simulate`; it
does not alter game rules, `engine-core`, `game-stdlib`, action-tree schema,
replay-check, fixture-check, or the bot selection contract.

Verification:

- `cargo test -p simulate`
- `for g in race_to_n three_marks column_four directional_flip draughts_lite high_card_duel masked_claims flood_watch frontier_control event_frontier token_bazaar secret_draft poker_lite plain_tricks; do cargo run -p simulate -- --game $g --games 1000 || break; done`
- `cargo test --workspace`
- `cargo clippy --workspace --all-targets -- -D warnings`

# STACROSCISIM-002: Reduce per-action cost of starbridge_crossing jump-chain enumeration (behavior-preserving)

**Status**: COMPLETED
**Priority**: LOW
**Effort**: Small
**Engine Changes**: Yes — `games/starbridge_crossing/src/actions.rs` (legal-action-tree generation). Behavior-preserving: the emitted `ActionTree` must be byte-identical. No schema, trace format, or public/private view change.
**Deps**: None. Independent of STACROSCISIM-001 (which unblocks CI on its own); this is a follow-up perf improvement that may later permit raising the CI games count.

## Problem

`starbridge_crossing` simulation is ~3× more expensive per action than comparable games, on top of its ~30× longer games. The recursive jump-chain enumerator `jump_landing_choices` (`games/starbridge_crossing/src/actions.rs:288–308`) clones the entire `visited: Vec<StarSpaceId>` landing history at every hop (`let mut next_visited = visited.clone(); next_visited.push(jump.landing);`), so a chain of depth *d* performs O(d²) cumulative copying across the branch. On dense boards with long jump chains this inflates legal-action generation, which is called once per ply by both the bot (`StarbridgeCrossingL0Bot::select_decision`) and the UI/view path.

This is the secondary factor behind the Gate 1 lane slowness addressed structurally by STACROSCISIM-001. It is split out because it touches product code (the action-tree generator) and is strictly an optimization — it must not change any legal action, ordering, or tree shape.

## Assumption Reassessment (2026-06-27)

1. `jump_landing_choices` (`games/starbridge_crossing/src/actions.rs:288`) recurses with `next_visited = visited.clone()` (lines 297, 308). `visited` feeds `legal_jump_landings(state, peg, current, &visited)` (`games/starbridge_crossing/src/rules.rs:81`), which reads it only to exclude already-visited landings — i.e. it is used as a read-only set during recursion and unwound on return. This is the classic backtracking shape where a single push/pop-on-a-shared-buffer replaces per-level cloning.
2. The action tree this builds is consumed by the bot (`games/starbridge_crossing/src/bots.rs` `select_decision` → `legal_action_tree`, `actions.rs:35`), the rules/apply path, and any golden traces / fixtures under `games/starbridge_crossing/`. Per FOUNDATIONS §2 (Rust owns legal-action generation) and §11 (deterministic replay/hash, no-leak), the optimization is acceptable only if the produced `ActionTree` — choices, ordering, `next` nesting, labels — is identical before and after.
3. Cross-artifact boundary under audit: `actions.rs` (producer) ⇄ existing golden traces / replay fixtures (consumers that pin the exact tree). Confirm no trace or fixture must change; if any would, the optimization is not behavior-preserving and the ticket must stop and be reassessed.
4. FOUNDATIONS §11 restated: deterministic replay-hash and serialization order must hold. A backtracking rewrite that changes iteration order would change the action tree and break replay — so the rewrite must preserve `legal_jump_landings` ordering exactly.

## Architecture Check

1. Replacing per-hop `Vec::clone` with a single shared, mutable `visited` buffer that is pushed before the recursive call and popped after (depth-first backtracking) removes the O(d²) copying with no change in output. Cleaner than memoization (no cache-invalidation surface) and cleaner than passing a persistent immutable set (extra dependency/allocation). Alternative rejected: caching legal landings across plies — state mutates every ply, so the cache hit rate is low and the invalidation risk is high.
2. No backwards-compatibility shim: this replaces the cloning implementation in place; there is no second code path or alias.
3. `engine-core` is untouched (no mechanic nouns introduced). `game-stdlib` is untouched. The change is local to one game's `actions.rs`; no shared helper is promoted.

## Verification Layers

1. Behavior preservation (action tree identical) -> golden trace / deterministic replay-hash check: `cargo run -p replay-check -- --game starbridge_crossing --all` passes unchanged, and existing `cargo test -p starbridge_crossing` action-tree/legality tests pass with no fixture edits.
2. Determinism preserved (no ordering drift) -> `cargo run -p fixture-check -- --game starbridge_crossing` and `cargo run -p rule-coverage -- --game starbridge_crossing` pass unchanged.
3. No-leak unaffected -> grep-proof that the rewrite touches only the `visited` accumulation in `jump_landing_choices` and introduces no new field in any choice/preview (no hidden-information surface change).
4. Performance win realized -> `cargo run -p simulate -- --game starbridge_crossing --games 100 --seat-count 6 --action-cap 4096` shows higher `throughput_games_per_sec` than the pre-change baseline (record both numbers).

## What to Change

### 1. `games/starbridge_crossing/src/actions.rs` — backtracking buffer in `jump_landing_choices`

Rewrite `jump_landing_choices` to thread a single `&mut Vec<StarSpaceId>` (push the landing before the recursive call, pop it after) instead of cloning `visited` at each hop. Preserve the exact iteration order of `legal_jump_landings(...)` and the exact `ActionChoice`/`ActionNode` construction (Stop choice first, then optional Continue with the recursive `next`), so the emitted tree is unchanged. Adjust the function signature and its single call site in `legal_action_tree` (`actions.rs:35`) accordingly; if an owned-`Vec` public signature must be retained for callers, keep a thin owned-entry wrapper that seeds the buffer once.

## Files to Touch

- `games/starbridge_crossing/src/actions.rs` (modify)

## Out of Scope

- Any change to legal actions, action ordering, labels, accessibility labels, previews, or tree nesting (must be byte-identical).
- The CI games-count budget (STACROSCISIM-001).
- Building Gate 1 tools in `--release`.
- The game's `max_plies` / turn limit or any other rules/terminal change.
- Optimizing `legal_jump_landings` internals beyond what the `visited` threading requires.

## Acceptance Criteria

### Tests That Must Pass

1. `cargo test -p starbridge_crossing` passes with no test or fixture edits.
2. `cargo run -p replay-check -- --game starbridge_crossing --all` passes (deterministic replay-hash unchanged).
3. `cargo run -p fixture-check -- --game starbridge_crossing` and `cargo run -p rule-coverage -- --game starbridge_crossing` pass.
4. `cargo clippy --workspace --all-targets -- -D warnings` and `cargo fmt --all --check` pass.
5. `cargo run -p simulate -- --game starbridge_crossing --games 100 --seat-count 6 --action-cap 4096` runs and reports a higher `throughput_games_per_sec` than the recorded pre-change baseline.

### Invariants

1. The `ActionTree` emitted by `legal_action_tree` for any reachable state is identical before and after (FOUNDATIONS §2 behavior authority; §11 deterministic replay/hash).
2. No new information enters any `ActionChoice`/`ActionPreview` (FOUNDATIONS §11 no-leak firewall).

## Test Plan

### New/Modified Tests

1. `None — behavior-preserving optimization; existing starbridge_crossing action-tree/legality tests, golden traces, replay-check, fixture-check, and rule-coverage are the proof surface that output is unchanged (named in Acceptance Criteria).`

### Commands

1. `cargo test -p starbridge_crossing`
2. `cargo run -p replay-check -- --game starbridge_crossing --all && cargo run -p fixture-check -- --game starbridge_crossing && cargo run -p rule-coverage -- --game starbridge_crossing`
3. `cargo run -p simulate -- --game starbridge_crossing --games 100 --seat-count 6 --action-cap 4096` (record throughput before and after; the narrower per-game checks above are the correct boundary because the change is local to one game's action generator and proven by its golden/replay surfaces).

## Outcome

Completed: 2026-06-27

What changed:

- Replaced per-hop `visited.clone()` allocation in `jump_landing_choices` with a single mutable backtracking buffer.
- Preserved the existing landing iteration order, Stop-before-Continue choice construction, labels, metadata, tags, previews, and action-tree nesting.
- No fixtures, golden traces, schema files, public/private view fields, preview payloads, or shared crates changed.

Deviations from plan:

- The pre-existing `games/starbridge_crossing/src/rules.rs` worktree diff was left unstaged and unchanged by this ticket.
- An unrelated `.claude/skills/brainstorm/SKILL.md` worktree change appeared during the run and was left unstaged.

Verification:

- Baseline before the `actions.rs` rewrite: `cargo run -p simulate -- --game starbridge_crossing --games 100 --seat-count 6 --action-cap 4096` reported `throughput_games_per_sec=0.51`, `games_run=100`, `total_actions=200000`, `average_length=2000.00`, and `capped_matches=0`.
- After the rewrite, the same simulation command reported `throughput_games_per_sec=0.54`, with the same `wins_by_seat`, `games_run=100`, `total_actions=200000`, `average_length=2000.00`, and `capped_matches=0`.
- `cargo test -p starbridge_crossing` passed.
- `cargo run -p replay-check -- --game starbridge_crossing --all` passed all Starbridge golden traces.
- `cargo run -p fixture-check -- --game starbridge_crossing` passed.
- `cargo run -p rule-coverage -- --game starbridge_crossing` passed.
- `cargo fmt --all --check` passed.
- `cargo clippy --workspace --all-targets -- -D warnings` passed.
- Grep/diff review showed the ticket changed only visited-buffer threading in `games/starbridge_crossing/src/actions.rs`; it introduced no new action metadata, preview field, schema field, or visibility surface.

# GAT4THRMARBOA-006: Three Marks bots — Level 0 random legal + Level 1 rule-informed priority policy

**Status**: PENDING
**Priority**: HIGH
**Effort**: Large
**Engine Changes**: Yes — new `games/three_marks/src/bots.rs`; new `tests/bot_tests.rs`
**Deps**: GAT4THRMARBOA-003, GAT4THRMARBOA-005

## Problem

Three Marks needs a Level 0 random-legal bot and a Level 1 deterministic, explainable, non-search priority-policy bot. Both must choose only among Rust legal actions, validate through the normal apply path, be deterministic under declared inputs, and (Level 1) return a viewer-safe explanation. Per FOUNDATIONS §8 public v1/v2 bots exclude MCTS/ISMCTS/Monte Carlo/ML/RL — Level 1 is a tiny local board-inspection priority policy, not a search engine.

## Assumption Reassessment (2026-06-06)

1. `crates/ai-core/src/random_legal.rs` provides the generic `RandomLegalBot` (`select_action`/`select_action_with_rng`) and `legal_paths(tree)` — Level 0 reuses this generic infra. `games/race_to_n/src/bots.rs` is the per-game mirror (e.g. `RaceRandomBot`, policy version `race_to_n-random-legal-v1` referenced in `tools/simulate/src/main.rs:18`). Verified `RandomLegalBot::select_action(&self, tree: &ActionTree)` signature.
2. Spec `specs/gate-4-three-marks-board-smoke.md` §14.1 (Level 0), §14.2 (Level 1 priority order: win → block → fork → block-fork → center → opposite corner → corner → side → deterministic fallback), §15.5 (bot tests), §24 (Level 1 fork handling may be a documented subset if exact forks are disproportionate — never replaced by minimax/search). Legal actions + apply path from GAT4THRMARBOA-003; public view from 005 (Level 1 inspects the board via Rust-provided view/state).
3. Cross-crate boundary under audit: `crates/ai-core` stays generic random-legal infrastructure; all Three Marks *strategy* lives in `games/three_marks/src/bots.rs` (spec §8.3). No board noun enters `ai-core`.
4. FOUNDATIONS §8 (public bots are product opponents, not research AI) and §11 (bots use the normal legal action API and allowed views only; v1/v2 exclude MCTS/ISMCTS/Monte Carlo/ML/RL) motivate this ticket: both bots choose from the Rust legal action tree, mutate no state directly, use no hidden information, are deterministic under seed/policy inputs, and remain beatable.
5. Bot-legality + no-leak enforcement surface (§8/§11): the legal-action API is the gate — name it. Every bot choice MUST be a Rust legal action validated through the same apply path as humans; Level 1 inspects only the seat's allowed (public) view; the Level-1 explanation and any candidate reasoning are viewer-safe (no hidden state, no candidate-ranking leak), filling the bot-chose-action effect from GAT4THRMARBOA-004.

## Architecture Check

1. A deterministic priority policy (ordered tactical rules over the Rust legal set) is the right Level-1 shape for a tiny perfect-information game: explainable, fast, beatable, and ADR-free (no search class). Alternative (minimax/alpha-beta/MCTS) is forbidden by spec §4/§14 and FOUNDATIONS §8; even though `docs/OFFICIAL-GAME-CONTRACT.md` §9 *permits* Level 3 shallow search for small perfect-info games, Gate 4 deliberately scopes to Level 0/1.
2. No backwards-compatibility aliasing/shims — new module; `ai-core` reused, not modified.
3. `engine-core`/`ai-core` gain no board/line nouns; the priority policy's board inspection is local to `games/three_marks/src/bots.rs`; no `game-stdlib` extraction.

## Verification Layers

1. Level 0 legality/determinism invariant -> bot legality check + determinism test (`tests/bot_tests.rs`: many-seed legality, fixed-seed determinism, terminal no-action diagnostic, normal apply-path validation).
2. Level 1 tactical-correctness invariant -> bot test (immediate win taken; immediate block when no own win; center/opposite-corner/corner/side preference on representative states; documented fork behaviour covered or its exact limitation asserted).
3. Level 1 determinism + explanation-safety invariant -> determinism test + no-leak visibility test (same state/seed/variant → same action and explanation; explanation is viewer-safe with no private/debug internals).
4. Bot-choice-validates-through-apply invariant -> bot legality check (every Level 0/1 choice is a legal action applied through the normal Rust path, mutating no state directly).

## What to Change

### 1. `src/bots.rs` — Level 0

Per-game random-legal bot over `ai-core::RandomLegalBot` / `legal_paths`, with a stable policy version id; deterministic under seed; returns a safe terminal/no-action diagnostic when no legal action exists.

### 2. `src/bots.rs` — Level 1

Deterministic priority policy with a stable policy id/version and documented deterministic tie-breaking, evaluating the spec §14.2 order over the Rust legal set: win → block opponent win → create fork → block opponent fork (subset allowed if documented) → center → opposite corner → empty corner → empty side → deterministic fallback. Produce a viewer-safe explanation ("completed a line", "blocked a line", "took center", "chose first stable corner", …) feeding the bot-chose-action effect.

### 3. `tests/bot_tests.rs`

All §15.5 tests: many-seed Level 0 legality, fixed-seed Level 0 determinism, Level 1 immediate win, Level 1 immediate block, Level 1 fork behaviour (creation/blocking, or asserted limitation), Level 1 center/corner/side preference, explanation exists-and-is-safe, every bot choice validates through ordinary apply.

## Files to Touch

- `games/three_marks/src/bots.rs` (new)
- `games/three_marks/src/lib.rs` (modify)
- `games/three_marks/tests/bot_tests.rs` (new)

## Out of Scope

- Latency benchmarks for Level 0/1 decisions (GAT4THRMARBOA-008).
- Bot-vs-bot simulation via CLI (GAT4THRMARBOA-014) and run-bot WASM path (009).
- Any change to `crates/ai-core` beyond reusing its public API; any Level 2/3 bot (spec §14, §25).

## Acceptance Criteria

### Tests That Must Pass

1. `cargo test -p three_marks --test bot_tests` — all §15.5 bot tests pass.
2. `cargo test -p three_marks` — full crate suite green.
3. `bash scripts/boundary-check.sh` — `engine-core`/`ai-core` stay noun-free.

### Invariants

1. Every Level 0/1 choice is a Rust legal action validated through the normal apply path; bots mutate no state directly and use no hidden information.
2. Level 1 is deterministic under declared inputs and returns a viewer-safe explanation; no MCTS/ISMCTS/Monte Carlo/ML/RL/search is used.

## Test Plan

### New/Modified Tests

1. `games/three_marks/tests/bot_tests.rs` — Level 0 legality/determinism + Level 1 tactical/determinism/explanation-safety coverage.

### Commands

1. `cargo test -p three_marks --test bot_tests`
2. `cargo test -p three_marks && bash scripts/boundary-check.sh`
3. Decision-latency benchmarking is deferred to 008; correctness/legality/determinism tests are the right boundary for the policy diff.

# GAT7DRALITCOM-012: Bots — Level 0 recursive random-legal & Level 1 rule-informed

**Status**: COMPLETED
**Priority**: HIGH
**Effort**: Large
**Engine Changes**: Yes — `games/draughts_lite/src/bots.rs` (Level 0 recursive random-legal selection over the nested action tree; Level 1 modest rule-informed policy), `src/lib.rs` (export); reuses `crates/ai-core` recursive random-legal behavior.
**Deps**: 007, 008, 011

## Problem

Every official game needs a Level 0 random-legal bot; a serious public demo needs a Level 1 rule-informed bot. Both must choose complete legal leaf paths from the Rust action tree (handling nested continuation recursively), use deterministic seeded randomness, mutate no state directly, and never emit a partial continuation path (FOUNDATIONS §8). This ticket implements Level 0 (random legal leaf) and the documented Level 1 heuristics from GAT7DRALITCOM-011.

## Assumption Reassessment (2026-06-07)

1. `crates/ai-core/src/lib.rs:7` exports `RandomLegalBot` (`mod random_legal`); the spec §R17 says the existing recursive random-legal behavior is the precedent to reuse or mirror. `games/draughts_lite/src/actions.rs` (006) supplies the nested tree the bot walks, and `rules.rs::{validate_command,apply}` (007) is the legal API the bot routes through; `effects.rs` (008) defines the bot-action effect shape (level, policy version, selected path, public rationale). `games/directional_flip/src/bots.rs` is the per-game bot precedent.
2. The bot scope is fixed by spec §R17 and `games/draughts_lite/docs/BOT-STRATEGY-EVIDENCE-PACK.md` (GAT7DRALITCOM-011): Level 0 random legal; Level 1 the bounded heuristic list; explicitly NO minimax/alpha-beta/MCTS/playout/transposition/endgame DB/opening book. `docs/ROADMAP.md` §9 forbids "search without benchmarks".
3. Cross-artifact boundary under audit: bots consume the action tree (006) and legal API (007), emit the bot-action effect (008), and produce deterministic traces consumed by golden traces (014) and benchmarks (015). The bot must choose a complete leaf path — never a partial continuation.
4. FOUNDATIONS §8/§11 motivate this ticket: restate before coding — public bots use the normal legal action API and allowed views only, mutate no state directly, use no hidden information, are deterministic under declared inputs, and exclude MCTS/ISMCTS/Monte Carlo/ML/RL. Level 1 heuristics rank candidates; they do not search a game tree.
5. No-leak + determinism enforcement surface (§11): the bot rationale is emitted to effect logs / replay exports, so confirm it is public-safe (no hidden state, no candidate-ranking leak beyond a short public explanation) and that selection is deterministic for a fixed seed (spec §R17 "bot traces are deterministic for fixed seeds").

## Architecture Check

1. Reusing/mirroring `ai-core::RandomLegalBot` for Level 0 (rather than a bespoke random walker) keeps random-legal behavior consistent across games; Level 1 layers a bounded ranking on top of the same legal-leaf enumeration — no separate legality path.
2. No backwards-compatibility shims; new bot module.
3. `engine-core` stays noun-free (§3) — bot heuristics are game-local in `bots.rs`; `game-stdlib` gains no bot logic (§4, spec §R12 out-of-scope list). The bot adds no search class (§8, no §13 ADR trigger).

## Verification Layers

1. Level 0 legality -> bot test: Level 0 always selects a legal complete leaf path when one exists, including multi-segment paths; it routes through `validate_command`.
2. Level 1 heuristics -> bot tests: Level 1 prefers a promotion over a comparable non-promotion, prefers a capture path when applicable, and completes mandatory continuation; it never emits a partial continuation path.
3. Determinism -> bot test + golden trace (014): fixed seed yields a fixed selection; bot traces are stable.
4. No search infrastructure -> codebase grep-proof + FOUNDATIONS alignment check: `bots.rs` contains no minimax/alpha-beta/MCTS/playout/transposition machinery (§8).
5. Rationale no-leak -> no-leak visibility test: the bot-action rationale exposes only public-safe explanation (FOUNDATIONS §11).

## What to Change

### 1. Level 0 random-legal

In `bots.rs`, implement Level 0 by enumerating complete legal leaf paths from the action tree and selecting one with deterministic seeded randomness (reusing/mirroring `ai-core::RandomLegalBot`); handle nested continuation recursively.

### 2. Level 1 rule-informed

Implement the documented §R17 heuristics as a deterministic ranking over complete leaf paths (winning move when cheaply detectable on a cloned state, capture/promotion/capture-to-promotion preference, king preservation, one-ply hanging-king avoidance), with seeded tie-breaking after ranking. Emit the bot-action effect (level, policy version, selected path, short public rationale).

## Files to Touch

- `games/draughts_lite/src/bots.rs` (new)
- `games/draughts_lite/src/lib.rs` (modify — export the bots module)

## Out of Scope

- Bot-strategy docs (GAT7DRALITCOM-011; this ticket implements them).
- Bot benchmarks (GAT7DRALITCOM-015) and bot golden traces (GAT7DRALITCOM-014).
- Any search/learning method (forbidden; FOUNDATIONS §8, spec §R17).

## Acceptance Criteria

### Tests That Must Pass

1. `cargo test -p draughts_lite` — Level 0 legality (incl. multi-segment), Level 1 capture/promotion/forced-continuation preference, and deterministic-seed tests pass.
2. `bash scripts/boundary-check.sh` — `engine-core` stays noun-free.

### Invariants

1. Bots choose complete legal leaf paths via the legal API; no partial continuation, no direct state mutation, no hidden information (FOUNDATIONS §8/§11; spec §R17).
2. Bot selection is deterministic for a fixed seed; no search class is introduced (FOUNDATIONS §8; no §13 ADR trigger).

## Test Plan

### New/Modified Tests

1. `games/draughts_lite/tests/bots.rs` — Level 0 legal-leaf + multi-segment selection; Level 1 heuristic-preference scenarios from the evidence pack; deterministic-seed assertions (bot golden trace lands in GAT7DRALITCOM-014).

### Commands

1. `cargo test -p draughts_lite bots`
2. `cargo test -p draughts_lite && bash scripts/boundary-check.sh`
3. Crate-scoped bot tests are correct; bot-vs-bot simulation throughput is verified via `tools/simulate` (GAT7DRALITCOM-017) and benchmarks (015).

## Outcome

Implemented Draughts Lite Level 0 and Level 1 bots. Level 0 reuses the existing
recursive random-legal bot over complete leaf paths. Level 1 ranks complete Rust
legal paths using the documented modest heuristics: terminal wins,
capture-to-promotion, promotion, capture preference, longer capture as a
heuristic only, one-ply king-safety, material tie-breaks, and deterministic
seeded tie-breaks. Both bots route through the legal action tree/validation API,
emit no partial continuation paths, and Level 1 emits a viewer-safe bot-action
effect rationale.

Verification passed:

1. `cargo test -p draughts_lite bots`
2. `cargo test -p draughts_lite`
3. `cargo fmt --all --check`
4. `bash scripts/boundary-check.sh`
5. Manual no-search grep of `games/draughts_lite/src/bots.rs`; the only
   matching term is a no-search rationale assertion in the tests.

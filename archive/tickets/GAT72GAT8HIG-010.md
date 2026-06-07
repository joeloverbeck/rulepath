# GAT72GAT8HIG-010: Level 0 random-legal bot + bot tests

**Status**: COMPLETED
**Priority**: HIGH
**Effort**: Medium
**Engine Changes**: Yes — `games/high_card_duel/src/bots.rs`, `games/high_card_duel/tests/bots.rs`
**Deps**: GAT72GAT8HIG-006, GAT72GAT8HIG-007

## Problem

Every official game needs a Level 0 random-legal bot. For `high_card_duel` the
bot must choose only from the actor's own legal action tree and allowed
actor-private view — never reading the opponent's hand, the hidden lead
commitment before reply, unrevealed deck order, or future draws — and must be
deterministic under (seed, policy version, view).

## Assumption Reassessment (2026-06-07)

1. Verified the bot infra: `crates/ai-core/src/random_legal.rs` provides the
   random-legal selection primitive over a legal action tree; sibling games wire
   a bot in `src/bots.rs` (e.g. `games/draughts_lite::DraughtsLiteLevel1Bot`,
   re-exported in `crates/wasm-api/src/lib.rs:26`).
2. Verified against the spec: §4.2.7 requires Level 0 (mandatory), bot input
   restricted to the actor's legal tree + allowed actor-private view,
   determinism under declared inputs, many-seed terminal completion, latency
   benchmarked, and `docs/AI.md` stating hidden-info boundaries. Level 1 is
   optional and Level 2 forbidden.
3. Cross-artifact boundary under audit: the bot-input contract — the bot
   consumes the same actor-private view/action tree produced by 006/008, never
   raw internal state.
4. FOUNDATIONS principle under audit (§8 public bots + §11): the bot uses the
   normal legal action API and allowed views only, mutates no state directly,
   and never uses hidden information unavailable to its seat. No
   MCTS/ISMCTS/Monte Carlo/ML/RL.
5. Enforcement surface named: the §11 no-leak firewall on bot input/output.
   Confirm the bot's input type cannot expose the opponent hand / deck / hidden
   commitment, and the bot emits no candidate rankings or explanations into any
   public surface. Proven by the bot tests here + no-leak suite (011).

## Architecture Check

1. Building Level 0 on `ai-core::random_legal` over the actor-scoped tree keeps
   bot legality identical to human legality (single source) — cleaner than a
   bespoke selector that could drift from the legal set.
2. No backwards-compatibility shims — new game-local bot.
3. `engine-core`/`ai-core` reused as generic infra; no mechanic noun in the
   kernel; no `game-stdlib` change.

## Verification Layers

1. Legal-only selection -> bot legality check: every chosen action is in the actor's legal tree across many seeds.
2. No hidden-info access -> no-leak visibility test: the bot-input type exposes no opponent hand / deck / hidden commitment.
3. Determinism -> deterministic replay check: same seed/policy/version/view ⇒ same choice; `bot-action` golden trace (authored in 012) reproduces.
4. Terminal completion -> simulation/CLI run: many-seed simulation completes terminal games with all-legal actions.

## What to Change

### 1. `bots.rs`

Level 0 random-legal bot selecting uniformly (seeded) from the actor's legal
action tree over the allowed actor-private view; deterministic under (seed,
policy version, view); no candidate rankings/explanations in any output.

### 2. `tests/bots.rs`

Legality, no-hidden-input, determinism, and many-seed terminal cases.

## Files to Touch

- `games/high_card_duel/src/bots.rs` (modify — fill stub)
- `games/high_card_duel/tests/bots.rs` (new)

## Out of Scope

- Optional Level 1 baseline (not shipped this gate; spec §4.2.7 "allowed but not required") and Level 2 (forbidden).
- WASM `run_bot_turn` wiring (016) and bot latency bench (014).

## Acceptance Criteria

### Tests That Must Pass

1. `level0_chooses_only_legal_actions`, `level0_uses_actor_private_action_tree_only`.
2. `bot_cannot_access_opponent_hand_deck_or_hidden_commitment_via_input_type`.
3. `same_seed_policy_version_deterministic`, plus a many-seed terminal simulation.

### Invariants

1. The bot uses only the legal action API + allowed actor-private view (§8/§11).
2. No public bot explanation/candidate leak.

## Test Plan

### New/Modified Tests

1. `games/high_card_duel/tests/bots.rs` — legality / no-hidden-input / determinism / many-seed terminal.

### Commands

1. `cargo test -p high_card_duel --test bots`
2. `cargo test -p high_card_duel`
3. The bot test is the correct boundary; large-scale legality is confirmed by `simulate` in the capstone (021).

## Outcome

Completed: 2026-06-07

What changed:

- Added `HighCardDuelRandomBot` as the Level 0 seeded random-legal bot over the actor-private legal action tree.
- Added `HighCardDuelBotInput` containing only bot seat, actor-private action tree, own hand, and own commitment; it exposes no opponent hand, deck order, or hidden opponent commitment.
- Added `BotDecision` with policy metadata and no public explanation/candidate ranking output.
- Added bot tests for legal-only selection, actor-private tree use, hidden-input redaction, same-seed determinism, and many-seed terminal completion.

Deviations from original plan:

- None.

Verification results:

- `cargo fmt --all --check` passed.
- `cargo test -p high_card_duel --test bots` passed.
- `cargo test -p high_card_duel` passed.

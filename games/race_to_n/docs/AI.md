# race_to_n AI

Game ID: `race_to_n`

Rules version: `race_to_n-rules-v1`

Last updated: 2026-06-05

## Scope

Gate 1 provides only a Level 0 random legal bot. The bot is a product opponent
and simulation driver, not a strategy claim.

## Level 0 random legal bot

The generic random legal selector lives in `ai-core` and chooses one path from a
Rust-supplied legal `ActionTree` using the deterministic `engine-core` RNG
contract. `race_to_n` wiring supplies the public view and the legal tree for the
bot seat, then validates the selected path through the normal game validation
path.

## Boundary notes

- The bot does not inspect hidden information; `race_to_n` is perfect
  information and its bot-facing view is public.
- The bot does not mutate state directly.
- The bot does not bypass validation.
- Fixed seed plus fixed legal tree yields the same selected action.
- Level 1+ strategy, search, MCTS, ML, and RL are out of Gate 1 scope.

## Evidence

`games/race_to_n/tests/bot_tests.rs` covers many-seed legality, deterministic
selection for fixed input, validation through the normal path, and no direct
state mutation.

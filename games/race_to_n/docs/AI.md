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

Policy version: `race_to_n-random-legal-v1`

Selection rule: uniformly select one complete action path from the current
Rust-supplied action tree. For `race_to_n`, the tree is flat, so this is a
uniform choice among the currently available `add-*` segments.

Determinism: fixed bot seed plus fixed public view/action tree yields the same
selected action. The game rules themselves use no randomness; bot seeds affect
only bot choice.

Legality: the bot never constructs action semantics. It receives the action tree
from Rust, selects a path, and the caller validates the resulting command through
`validate_command` before `apply_action`.

## Boundary notes

- The bot does not inspect hidden information; `race_to_n` is perfect
  information and its bot-facing view is public.
- The bot does not mutate state directly.
- The bot does not bypass validation.
- Fixed seed plus fixed legal tree yields the same selected action.
- Level 1+ strategy, search, MCTS, ML, and RL are out of Gate 1 scope.
- The bot makes no strength claim and has no competence baseline beyond
  legality, determinism, and simulation usefulness.

## Evidence

`games/race_to_n/tests/bot_tests.rs` covers many-seed legality, deterministic
selection for fixed input, validation through the normal path, no direct state
mutation, and terminal-state no-action diagnostics.

`tools/simulate` drives full random legal games through the same bot wiring and
validates every selected action through the normal Rust path. The web harness
uses `run_bot_turn` through `wasm-api`, so TypeScript does not decide or repair
bot actions.

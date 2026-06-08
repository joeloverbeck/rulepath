# Token Bazaar AI

Game ID: `token_bazaar`

Rules version: `token-bazaar-rules-v1`

Last updated: 2026-06-08

## Shipped Bot Levels

Token Bazaar ships two Rust bot policies:

| Level | Policy id | Implementation | Public role |
|---:|---|---|---|
| 0 | `token_bazaar-random-legal-v1` | `TokenBazaarRandomBot` | Seeded random legal baseline. |
| 1 | `token_bazaar_level1_v1` | `TokenBazaarLevel1Bot` | Deterministic public heuristic used by WASM bot turns. |

Both policies choose from the normal Rust legal action tree and the resulting command must validate through the same command path as a human action.

## Level 1 Policy

The Level 1 bot uses only public Token Bazaar state:

1. Fulfill the highest-value affordable visible contract.
2. Otherwise target the best visible contract by points, smaller public deficit, then stable id tie-break.
3. Collect a legal bundle that reduces the target cost gap.
4. Otherwise exchange a legal resource toward the target.
5. Otherwise take the deterministic first legal action, including forced `pass` when that is the only legal action.

The seed is carried for bot API consistency, but the Level 1 policy is deterministic for a fixed state and seed.

## Public Rationale Boundary

`BotDecision.rationale` is public-safe prose. Tests require it to omit candidate lists, debug terms, valuation tables, internal dumps, and serialized array/object structures. The rationale may name public visible contracts such as `Amber Focus` because all market facts are public.

The browser does not expose a separate bot candidate table or hidden bot memory.

## Verification

- `cargo test -p token_bazaar --test bots`
- `cargo test -p token_bazaar --test property level1_bot_actions_validate_during_playout`
- `cargo run -p simulate -- --game token_bazaar --games 1000`
- `node apps/web/e2e/token-bazaar.smoke.mjs`

## Forbidden AI Changes

- No MCTS, ISMCTS, Monte Carlo, ML, RL, or runtime LLM bot.
- No hidden-state sampling. Token Bazaar is public, but future hidden games must not reuse this policy as a hidden-information shortcut.
- No TypeScript bot policy. Browser bot turns call Rust/WASM.

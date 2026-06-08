# Veiled Draft AI

Game ID: `secret_draft`

Rules version: `secret-draft-rules-v1`

Last updated: 2026-06-08

## Shipped Bot Levels

Veiled Draft ships two Rust bot policies:

| Level | Policy id | Implementation | Public role |
|---:|---|---|---|
| 0 | `secret_draft_random_legal_v0` | `SecretDraftRandomBot` | Seeded random legal baseline. |
| 1 | `secret_draft_level1_v1` | `SecretDraftLevel1Bot` | Deterministic public heuristic used by WASM bot turns. |

Both policies choose from the normal Rust legal action tree and the resulting
command must validate through the same command path as a human commitment.

## Level 1 Policy

The Level 1 bot ranks only currently visible legal commitment choices:

1. Prefer an item that completes a public `ember`/`tide`/`grove` set.
2. Prefer higher public item value.
3. Prefer an item that adds a public high-thread terminal bonus.
4. Consider public fallback safety from the bot seat's priority status and the
   stable visible pool.
5. Use stable item id and deterministic seed tie-breaks.

The policy never sees or samples the opponent's hidden commitment. If the other
seat has committed, the bot sees only the public pending flag and the still
visible pool.

## Public Rationale Boundary

`BotDecision.rationale` is public-safe prose. It may mention visible legal
commitments, public values, public thread-set completion, public thread bonuses,
public priority/fallback exposure, and deterministic tie-breaks.

The rationale must not contain hidden opponent choices, sampled possibilities,
candidate dumps, debug state, private state, internal state, MCTS/Monte Carlo
claims, ML/RL claims, or LLM policy claims. The browser does not expose a
separate bot candidate table.

## Verification

- `cargo test -p secret_draft --test bots`
- `cargo test -p secret_draft level1_prefers_completing_a_thread_set`
- `cargo run -p simulate -- --game secret_draft --games 1000`
- `node apps/web/e2e/secret-draft.smoke.mjs`

## Forbidden AI Changes

- No MCTS, ISMCTS, Monte Carlo, ML, RL, or runtime LLM bot.
- No hidden-state sampling, opponent-commit peeking, or omniscient heuristic.
- No TypeScript bot policy. Browser bot turns call Rust/WASM.
- No public candidate ranking that can expose hidden/private/internal data.

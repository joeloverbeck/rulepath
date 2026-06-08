# Veiled Draft Bot Strategy Evidence Pack

Game ID: `secret_draft`

Last updated: 2026-06-08

## Status

Level 1 shipped for Gate 9.1.

Policy id: `secret_draft_level1_v1`

Implementation: `SecretDraftLevel1Bot` in `games/secret_draft/src/bots.rs`

No Level 2 authored-policy bot ships in Gate 9.1. This pack records the Level 1
public heuristic evidence and the no-leak boundary that any future Level 2 pack
must preserve.

## Policy Summary

The Level 1 bot chooses from the active seat's Rust legal actions:

1. complete a public thread set when available;
2. prefer higher visible item value;
3. add a public high-thread bonus when available;
4. account for public conflict priority or fallback exposure;
5. use stable id and deterministic seed tie-breaks.

It emits a short public rationale. It does not expose hidden commitments,
candidate rankings, debug values, hidden state, private state, internal state,
serialized decision structures, or sampled belief state.

## Evidence Fixtures

| Evidence | Test / trace | Expected behavior |
|---|---|---|
| random and Level 1 validate | `random_and_level1_decisions_are_legal_and_do_not_mutate_state` | Bot choices are legal command paths and selection does not mutate state. |
| deterministic fixed-state decisions | `seeded_bots_are_deterministic_on_same_public_state` | Same state and seed produce identical decisions. |
| hidden opponent commitment ignored | `level1_uses_only_public_information_when_opponent_commitment_differs` | Different hidden commitments with the same public projection produce the same decision/rationale. |
| rationale no-leak terms | `bot_rationales_do_not_claim_hidden_or_sampled_information` | Rationales omit hidden, sample, MCTS, Monte Carlo, ML, and LLM language. |
| playout validation | `level1_bots_finish_many_games_with_legal_actions` | Level 1 bots finish repeated games through legal actions only. |
| strategy preference | `level1_prefers_completing_a_thread_set` | A public set-completion opportunity outranks other legal choices. |
| committed-seat guard | `committed_seat_has_no_decision` | A seat that already committed receives `no_legal_actions`. |
| golden bot trace | `golden_traces/bot-action.trace.json` | Bot action trace remains replay-checkable. |

## Public Rationale Samples

Allowed rationale shape from the shipped policy:

- `Ranked visible legal commitments by public set completion, public value 4, public thread bonuses, public fallback safety, and deterministic tie-breaks; selected a commitment that completes a public thread set with public fallback exposure considered.`
- `Ranked visible legal commitments by public set completion, public value 4, public thread bonuses, public fallback safety, and deterministic tie-breaks; selected a commitment that does not complete a public thread set with public priority for conflicts.`

These examples name only public facts. They must not grow opponent-choice
phrasing, hidden-state phrasing, candidate dumps, or debug math in public output.

## Hidden-Information Boundary

| Information | Bot access | Evidence |
|---|---:|---|
| visible pool item ids, labels, values, and threads | yes | legal action tree and `level1_*` tests |
| public drafted collections and scores | yes | public state used by set/high-thread ranking |
| public priority seat | yes | fallback safety ranking |
| own previous revealed awards | yes | drafted collection is public |
| opponent hidden commitment before reveal | no | `level1_uses_only_public_information_when_opponent_commitment_differs` |
| internal commitment slots | no | visibility/no-leak tests and browser smoke |
| candidate ranking table | no public surface | `bot_rationales_do_not_claim_hidden_or_sampled_information` |

## Verification Commands

- `cargo test -p secret_draft --test bots`
- `cargo run -p replay-check -- --game secret_draft --all`
- `cargo run -p simulate -- --game secret_draft --games 1000`
- `node apps/web/e2e/secret-draft.smoke.mjs`

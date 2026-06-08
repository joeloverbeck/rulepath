# Veiled Draft Competent Player

Game ID: `secret_draft`

Last updated: 2026-06-08

## Status

Veiled Draft ships a Level 1 competent public bot,
`SecretDraftLevel1Bot`, with policy id `secret_draft_level1_v1`.

The competent player is intentionally simple. It does not search future trees,
sample hidden commitments, model the opponent's private choice, or use
statistical rollout. It is an authored public heuristic that demonstrates the
game without replacing Rust legal-action authority.

## Competent Play Guidance

A competent Veiled Draft player should:

1. Complete `ember`/`tide`/`grove` sets when the visible item cost in value is
   reasonable.
2. Prefer higher-value visible items when set completion is not available.
3. Track high-thread bonuses, especially when a third item in a thread is legal.
4. Use public priority to contest a valuable item when conflict fallback risk is
   acceptable.
5. When not the priority seat, consider what public fallback item would be
   awarded if both seats choose the same item.
6. Remember that all scoring and tie-break facts are public after reveal:
   total score, set count, highest item, distinct threads, fewer priority-won
   contested items, then draw.

## Bot Mapping

The Level 1 policy covers the guidance above:

| Guidance | Bot behavior | Evidence |
|---|---|---|
| complete public thread sets | set-completion rank dominates other legal choices | `level1_prefers_completing_a_thread_set` |
| prefer visible value | item value is the second rank slot | `SecretDraftLevel1Bot::level1_rank` |
| account for high-thread bonus | high-thread rank slot is applied after value | `SecretDraftLevel1Bot::level1_rank` |
| account for conflict priority | public priority/fallback safety is part of the rank | `SecretDraftLevel1Bot::level1_rank` |
| use only legal actions | decisions validate through `validate_command` | `random_and_level1_decisions_are_legal_and_do_not_mutate_state` |
| ignore hidden opponent choice | same public state with different hidden commitments gives same decision | `level1_uses_only_public_information_when_opponent_commitment_differs` |
| safe rationale | rationale omits hidden/sampled/search/ML terms | `bot_rationales_do_not_claim_hidden_or_sampled_information` |

## Limits

The competent player does not claim optimal play. It has no lookahead, no
opponent model, no hidden-state belief model, no multi-round pool planning
beyond current public facts, and no generalized drafting evaluator. Those are
deliberate limits for public v1/v2.

# Token Bazaar Competent Player

Game ID: `token_bazaar`

Last updated: 2026-06-08

## Status

Token Bazaar ships a Level 1 competent public bot, `TokenBazaarLevel1Bot`, with policy id `token_bazaar_level1_v1`.

The competent player is intentionally simple. It does not search future trees, sample hidden state, or use statistical rollout. It is an authored public heuristic that demonstrates the game without replacing the Rust legal-action authority.

## Competent Play Guidance

A competent Token Bazaar player should:

1. Fulfill affordable visible contracts before drifting into resource hoarding.
2. Prefer higher point contracts when multiple visible contracts are reachable.
3. Collect resources that reduce a visible contract's exact cost gap.
4. Use exchange only when it moves public inventory toward a target cost.
5. Treat forced pass as a consequence of public supply/market exhaustion, not a voluntary strategic action.
6. Remember that final tie-breaks are score, fulfilled-contract count, total remaining inventory, then draw.

## Bot Mapping

The Level 1 policy covers the guidance above:

| Guidance | Bot behavior | Evidence |
|---|---|---|
| fulfill affordable contracts | chooses `fulfill/slot_0` on the initial state | `level1_fulfills_affordable_contract` |
| collect toward a visible target | chooses `collect/amber` when only `Amber Focus` is visible and unaffordable | `level1_collects_toward_unaffordable_visible_contract` |
| use only legal actions | validates decisions through `validate_command` across public states | `level1_selection_validates_across_public_states` |
| deterministic public fallback | chooses forced `pass` when no economy action exists | `level1_fallback_reaches_forced_pass` |
| no public rationale leak | rationale omits candidate/debug/valuation/internal structures | `assert_public_safe_rationale` |

## Limits

The competent player does not claim optimal play. It has no lookahead, no opponent model, no market-exhaustion planning beyond current visible facts, and no generalized economy evaluator. Those are deliberate limits for public v1/v2.

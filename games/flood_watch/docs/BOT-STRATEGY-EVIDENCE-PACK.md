# Flood Watch Bot Strategy Evidence Pack

Game ID: `flood_watch`

Policies: `flood_watch_random_legal_v0`,
`flood_watch_level1_public_priority_v1`

Rules version checked: `flood-watch-rules-v1`

Date: 2026-06-11

## Purpose and Authority

This evidence pack documents the implemented Flood Watch bot policies and the
tests that constrain them. It does not define rules. If this document conflicts
with [RULES.md](RULES.md), the rules win.

## Implemented Policies

| Policy | Level | Implementation | Inputs | Notes |
|---|---:|---|---|---|
| `flood_watch_random_legal_v0` | 0 | [bots.rs](../src/bots.rs) | legal action tree and declared seed | Selects a seeded random legal leaf. |
| `flood_watch_level1_public_priority_v1` | 1 | [bots.rs](../src/bots.rs) | public projection, legal action tree, remaining composition, public forecast, declared seed | Deterministic cooperative priority policy. |

## Level 1 Priority Order

The Level 1 policy is deterministic and explainable:

1. Bail a level-2 district if a legal bail action exists.
2. Mitigate a public forecast that would otherwise inundate a district.
3. Reinforce the district with the highest public expected pressure from
   remaining-composition counts.
4. Forecast when no public district action improves the position.
5. End turn when no public legal action improves the position.

Tie-breaks use stable district order. The policy never reads `event_deck` or
any ordered hidden event tail.

## Executable Evidence

| Evidence | File/command | Covered behavior |
|---|---|---|
| Bot legality through command validation | [../tests/bots.rs](../tests/bots.rs) | Level 0 and Level 1 choose legal actions for both seats/roles. |
| Determinism | [../tests/bots.rs](../tests/bots.rs) | Same state and seed produce identical decisions and rationales. |
| Hidden-order invariance | [../tests/bots.rs](../tests/bots.rs) | Same public view with different hidden deck order produces the same Level 1 decision. |
| Rationale no-leak checks | [../tests/bots.rs](../tests/bots.rs) | Rationales contain no event IDs, copy IDs, sampling claims, or search/ML claims. |
| Golden bot trace shape | [../tests/golden_traces/bot-coop-full-game.trace.json](../tests/golden_traces/bot-coop-full-game.trace.json) | Bot policy/action/rationale fields are public and redacted. |
| Playout smoke | [BENCHMARKS.md](BENCHMARKS.md) | `random_playout` benchmark exercises legal cooperative bot playouts. |

## Fairness and Beatable Posture

The Level 1 bot is intentionally competent but bounded:

- It does not search game trees.
- It does not simulate hidden deck tails.
- It does not sample unknown events.
- It does not use MCTS, ISMCTS, Monte Carlo, ML, or RL.
- It reacts to public pressure and public forecast information only.
- It can waste tempo by over-reinforcing public expected pressure when a human
  plan would use forecast or role sequencing better.

That makes it a fair cooperative teammate/baseline rather than an opaque
solver.

## Balance Status

The intended Level 1 + Level 1 win-rate target for the standard scenario is
approximately 35-75%. The simulator command
`cargo run -p simulate -- --game flood_watch --games 1000` is registered in
GAT12FLOWATCOO-015, so numeric win-rate evidence is not claimed in this doc.
Once that registration exists, an out-of-band result must trigger a scenario
constant retune before public polish.

## No-Leak Checklist

- Bot inputs are public projection, legal tree, public forecast, remaining
  composition, and declared seed.
- Bot explanations do not name unrevealed event IDs or card copy IDs.
- Bot policy never reads undrawn deck order.
- Bot policy does not use hidden-state sampling or search-class methods
  excluded for public v1/v2.

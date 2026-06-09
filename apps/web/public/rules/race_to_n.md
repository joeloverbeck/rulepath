# Race to 21 - How to Play

_Game ID: `race_to_n`_  
_Formal rules source: `games/race_to_n/docs/RULES.md`_  
_Formal rules version checked: `race_to_n-rules-v1`_  
_Strategy guide: Not applicable - no separate strategy guide is required for this foundation-smoke game._

## At a glance

- Players share one counter that starts at 0.
- On your turn, add 1, 2, or 3 to the counter.
- The UI only shows additions that do not overshoot the target.
- The player who makes the counter exactly 21 wins immediately.
- There are no draws and no hidden pieces.

## What you can see

All game information is public: the current counter, target, active seat, legal additions, and winner if the game has ended.

Not applicable - this is a perfect-information game. All game state needed for play is public in the normal game view.

## Setup

The match starts with the counter at 0, target 21, and player 1 active.

There are no setup choices, hidden values, or random setup steps.

## On your turn

Choose one legal addition shown by the interface.

After a non-winning addition, the turn passes to the other player. If your addition makes the counter exactly 21, the match ends immediately.

## Actions

### Add 1

Adds 1 to the shared counter.

### Add 2

Adds 2 to the shared counter when that would not overshoot 21.

### Add 3

Adds 3 to the shared counter when that would not overshoot 21.

## Scoring and winning

Race to 21 has no running score beyond the shared counter.

The acting player wins when their legal addition makes the counter exactly 21. Draws are not possible in this variant.

## Hidden information and reveal timing

Not applicable - this is a perfect-information game.

## Common terms

| Term | Meaning |
|---|---|
| Counter | The shared total players add to each turn. |
| Target | The exact total, 21, needed to win. |
| Legal addition | An add choice supplied by Rust that does not overshoot the target. |

## What this page is not

This page teaches rules and turn flow. It is not strategy advice, and it is not the formal implementation contract.

## Source notes for maintainers

The formal rule source is `games/race_to_n/docs/RULES.md`.

The formal rules version checked is `race_to_n-rules-v1`.

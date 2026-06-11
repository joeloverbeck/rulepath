# Frontier Control - How to Play

_Game ID: `frontier_control`_
_Formal rules source: `games/frontier_control/docs/RULES.md`_
_Formal rules version checked: `frontier-control-rules-v1`_
_Strategy guide: `games/frontier_control/docs/COMPETENT-PLAYER.md`_

## At a glance

- Frontier Control is a two-player perfect-information game on a small trail map.
- One player is the Garrison; the other player is the Prospectors.
- The Garrison scores by holding forts with guards and no crews present.
- The Prospectors score by placing stakes that still have a guard-free route back to Base Camp when the round scores.
- Each side spends a small action budget on its turn, then the Garrison turn triggers round scoring.
- After the final scheduled scoring round, the higher score wins. Tied final scores go to the Garrison.

## What you can see

Not applicable - this is a perfect-information game. All game state needed for
play is public in the normal game view.

## Setup

The map shows named frontier sites connected by trails. Some sites are forts,
some can hold Prospector stakes, and Base Camp is the Prospector home site.

At the start of the standard game, Garrison guards occupy the forts and
Prospector crews start at Base Camp. Scores begin at zero, round 1 begins, and
the Prospectors take the first action phase.

The Highlands variant uses the same rules with a different site layout, fort
set, starting units, stake values, and round count.

## On your turn

The active faction spends its action budget one legal action at a time. The
action panel only shows choices that Rust says are legal for the active seat.

The Prospectors act first in each round. The Garrison acts second. When the
Garrison action phase ends, the round scores immediately. If the match is not
over, the next round begins with the Prospectors active again.

You may end your turn early. If you spend the last action point, the turn ends
automatically.

## Actions

### March

Prospectors move one crew from a crew-occupied site to an adjacent site along a
trail. If the crew enters a guarded site, one guard is removed and the entering
crew is also removed.

### Stake

Prospectors place a stake on an eligible site occupied by a crew and no guards.
Staked sites can score for the Prospectors during round scoring if they remain
connected to Base Camp through sites with no guards.

### Muster

Prospectors add one crew at Base Camp when Base Camp can legally receive it.

### Patrol

The Garrison moves one guard from a guard-occupied site to an adjacent site
along a trail. If the guard enters a site with crews, one crew is removed and
the guard remains.

### Reinforce

The Garrison adds one guard to a held fort when that fort is below the site unit
cap.

### Dismantle

The Garrison removes a stake from a site that has at least one guard.

### End turn

The active faction gives up any remaining action budget and passes play to the
next phase.

## Scoring and winning

Rounds score after the Garrison turn ends.

The Garrison gains 1 point for each fort that has at least one guard and no
crews.

The Prospectors gain each staked site's public stake value when that site has a
guard-free route to Base Camp. A cut stake remains on the map, but it scores 0
for that round.

After the final scheduled round scores, the faction with the higher total score
wins. If the final scores are tied, the Garrison wins the tiebreak. The outcome
surface shows the final scores and whether the Garrison tiebreak was decisive.

## Hidden information and reveal timing

Not applicable - this is a perfect-information game.

## Common terms

| Term | Meaning |
|---|---|
| Garrison | The faction that uses guards, holds forts, dismantles stakes, and wins tied final scores. |
| Prospectors | The faction that uses crews, places stakes, and scores supplied stake values. |
| Site | A named point on the frontier map. |
| Trail | A connection between two sites. Movement follows trails. |
| Fort | A Garrison scoring site. |
| Stake | A Prospector marker that can score when supplied. |
| Supplied | A staked site has a guard-free route to Base Camp. |
| Cut | A staked site has no guard-free route to Base Camp and scores 0 for the round. |
| Clash | A movement result when a unit enters a site with opposing units. |
| Action budget | The number of actions the active faction can still spend this turn. |

## What this page is not

This page teaches rules and turn flow. It is not strategy advice, and it is not
the formal implementation contract.

Formal rule IDs, Rust validation notes, rule coverage, bot strategy, and
implementation details belong in the other game docs.

## Source notes for maintainers

Confirm before merging:

- [x] Prose is original Rulepath wording.
- [x] No copied rulebook text, examples, diagrams, assets, names, fonts, or trade dress.
- [x] No strategy advice copied from `COMPETENT-PLAYER.md`.
- [x] No hidden match-state examples or seed-specific data.
- [x] No YAML front matter.
- [x] No selectors, conditions, triggers, or action schemas.
- [x] Formal rules version checked matches `RULES.md`.

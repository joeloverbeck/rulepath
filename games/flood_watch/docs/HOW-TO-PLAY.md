# Flood Watch - How to Play

_Game ID: `flood_watch`_  
_Formal rules source: `games/flood_watch/docs/RULES.md`_  
_Formal rules version checked: `flood-watch-rules-v1`_  
_Strategy guide: `games/flood_watch/docs/COMPETENT-PLAYER.md`_

## At a glance

- Flood Watch is a two-seat cooperative game. Both seats win or lose together.
- On your turn, spend up to three actions to bail districts, add levees, forecast the next event, or end the turn.
- After a turn ends, Rust resolves an environment batch by drawing event cards from the hidden event deck.
- You win if the final event card resolves without any district reaching flood level 3.
- You lose immediately if any district reaches flood level 3.
- The remaining event composition is public, but the undrawn deck order is hidden.

## What you can see

All players and observers see the same public projection: seats, roles, active seat, turn number, action budget, district flood levels, levee stacks, drawn events, the forecast card if one is active, remaining composition counts, and terminal result.

The exact order and identities of undrawn event cards below any forecast card are hidden from every browser-facing viewer and bot. The browser never shows the original shuffled deck order, including after terminal.

## Setup

Flood Watch starts with two seats, `seat_0` and `seat_1`. The standard scenario uses five districts: Riverside, Old Docks, Market, Terraces, and Gardens. Each district starts with a public flood level and levee count from the scenario. `seat_0` is the Pumpwright and `seat_1` is the Levee Warden.

Rust builds the event deck from closed event kinds and shuffles it deterministically from the match seed. Scenario data declares counts and constants only; it does not define event behavior.

## On your turn

Only the active seat acts. The active seat receives a Rust legal action tree and may spend the action budget one action at a time. Each bail, reinforce, or forecast action spends one budget. The active seat may also end the turn early.

When the budget reaches zero, or when the active seat chooses End turn, Rust resolves the environment phase. It draws the scenario's event count in order, applies each event, emits public effects, clears any spent forecast marker, and either advances to the teammate's turn or ends the match.

## Actions

### Bail

Choose a district with flood level at least 1. Pumpwright removes up to 2 flood levels; other roles remove up to 1. Flood level never drops below 0.

### Reinforce

Choose a district below the levee cap. Levee Warden places up to 2 levees; other roles place up to 1. Levees absorb future flood rise before flood levels increase.

### Forecast

Reveal the current top event card publicly without drawing it. Forecast is unavailable if the top card is already forecast or the deck is empty.

### End turn

Forfeit any remaining budget and start the environment phase.

## Scoring and winning

Flood Watch has no individual score, no tiebreaker, and no per-seat winner. The terminal result is shared.

The team loses when any district reaches flood level 3. The terminal explanation names the inundated district and uses public flood-level facts only.

The team wins when the final event card resolves and no district reaches flood level 3. The terminal explanation cites deck exhaustion and public surviving district levels.

Terminal explanations never reveal the hidden order of undrawn cards.

## Hidden information and reveal timing

The event deck order is hidden after setup. A forecast reveals only the current top event card and makes it public to both seats and observers. Drawn event cards remain public history. Remaining composition counts are public because they are counts, not order.

Undrawn event identities below the forecasted card are never exposed through player view, effects, replay export, bot explanation, logs, storage, or terminal outcome.

## Common terms

| Term | Meaning |
|---|---|
| District | A public flood target: Riverside, Old Docks, Market, Terraces, or Gardens. |
| Flood level | A public counter from 0 to 3. Level 3 causes shared loss. |
| Levee | A public prevention counter that absorbs incoming flood rise. |
| Role | A public seat modifier. Pumpwright bails better; Levee Warden reinforces better. |
| Forecast | A public reveal of the next event card. |
| Remaining composition | Public counts of event kinds still unseen, without order. |
| Environment phase | Rust automation that draws and resolves event cards after a turn. |

## What this page is not

This page teaches rules and turn flow. It is not strategy advice, and it is not the formal implementation contract.

Formal rule IDs, Rust validation notes, rule coverage, bot strategy, and implementation details belong in the other game docs.

## Source notes for maintainers

Confirm before merging:

- [x] Prose is original Rulepath wording.
- [x] No copied rulebook text, examples, diagrams, assets, names, fonts, or trade dress.
- [x] No strategy advice copied from `COMPETENT-PLAYER.md`.
- [x] No hidden match-state examples or seed-specific data.
- [x] No YAML front matter.
- [x] No selectors, conditions, triggers, or action schemas.
- [x] Formal rules version checked matches `RULES.md`.

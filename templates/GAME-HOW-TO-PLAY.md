# <Game Display Name> - How to Play

_Game ID: `<game_id>`_  
_Formal rules source: `games/<game_id>/docs/RULES.md`_  
_Formal rules version checked: `<rules-version>`_  
_Strategy guide: `games/<game_id>/docs/COMPETENT-PLAYER.md`_

## At a glance

Write 3-6 short bullets for a first-time player:

- What is the goal?
- What does a normal turn or round look like?
- What are players trying to collect, place, reveal, score, or avoid?
- How does the game end?
- What is the most important visibility rule, if any?

Do not include strategy advice.

## What you can see

Describe public information first.

For hidden-information games, describe only:

- what the player can see from their own perspective;
- what all players can see publicly;
- what remains hidden until reveal;
- what is never exposed in the browser.

For perfect-information games, write:

`Not applicable - this is a perfect-information game. All game state needed for play is public in the normal game view.`

## Setup

Explain setup in player terms. Do not paste fixtures, seeds, Rust structs, JSON, or validation tables.

## On your turn

Explain the turn or phase flow in ordinary language.

If turns can branch, explain what the player sees and chooses. Do not encode selectors, conditions, or legality rules as a behavior table.

## Actions

List the action labels that can appear in the UI and explain what each means to a player.

Use one subsection per action:

### <Action label>

Plain-language explanation.

## Scoring and winning

Explain scoring, victory, loss, draw, split, terminal, or exhaustion outcomes.

## Hidden information and reveal timing

Required for hidden-information games.

For perfect-information games, write:

`Not applicable - this is a perfect-information game.`

For hidden-information games, explain reveal timing without exposing hidden values.

## Common terms

| Term | Meaning |
|---|---|
| <Term> | <Player-facing meaning> |

## What this page is not

This page teaches rules and turn flow. It is not strategy advice, and it is not the formal implementation contract.

Formal rule IDs, Rust validation notes, rule coverage, bot strategy, and implementation details belong in the other game docs.

## Source notes for maintainers

Confirm before merging:

- [ ] Prose is original Rulepath wording.
- [ ] No copied rulebook text, examples, diagrams, assets, names, fonts, or trade dress.
- [ ] No strategy advice copied from `COMPETENT-PLAYER.md`.
- [ ] No hidden match-state examples or seed-specific data.
- [ ] No YAML front matter.
- [ ] No selectors, conditions, triggers, or action schemas.
- [ ] Formal rules version checked matches `RULES.md`.

# Directional Flip - How to Play

_Game ID: `directional_flip`_  
_Formal rules source: `games/directional_flip/docs/RULES.md`_  
_Formal rules version checked: `directional_flip-rules-v1`_  
_Strategy guide: `games/directional_flip/docs/COMPETENT-PLAYER.md`_

## At a glance

- Players place discs on an 8 by 8 board.
- A legal placement must bracket one or more opposing discs in at least one direction.
- Bracketed opposing discs flip to the acting player.
- If you have no legal placement, Rust supplies a forced pass.
- The game ends when the board is full or neither player can place; higher final disc count wins.

## What you can see

The full board, disc owners, active seat, legal placements, forced-pass state, previews, disc counts, effects, terminal winner, and draw outcome are public.

Not applicable - this is a perfect-information game. All game state needed for play is public in the normal game view.

## Setup

The game starts with the fixed opening discs in the center of the board. Player 1 has the first legal placement options.

## On your turn

If Rust shows legal placements, choose one empty cell. The placement flips every contiguous bracketed line of opposing discs.

If you have no legal placement, choose the forced-pass action. A second consecutive forced pass ends the game.

## Actions

### Place

Place a disc on a Rust-supplied legal empty cell. At least one direction must bracket opposing discs between your new disc and another of your discs.

### Forced pass

Pass when Rust says you have no legal placement. You cannot pass voluntarily when a placement is available.

## Scoring and winning

After terminal state, count each player's discs. The higher count wins. If both players have the same count, the game is a draw.

Empty cells are not awarded to either player.

## Hidden information and reveal timing

Not applicable - this is a perfect-information game.

## Common terms

| Term | Meaning |
|---|---|
| Disc | A placed piece owned by one player. |
| Bracket | A line of opposing discs ending at one of your discs. |
| Flip | Changing bracketed opposing discs to your ownership. |
| Forced pass | The only legal action when the active player has no legal placement. |

## What this page is not

This page teaches rules and turn flow. It is not strategy advice, and it is not the formal implementation contract.

## Source notes for maintainers

The formal rule source is `games/directional_flip/docs/RULES.md`.

The formal rules version checked is `directional_flip-rules-v1`.

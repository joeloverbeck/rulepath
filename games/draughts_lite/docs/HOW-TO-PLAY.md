# Draughts Lite - How to Play

_Game ID: `draughts_lite`_  
_Formal rules source: `games/draughts_lite/docs/RULES.md`_  
_Formal rules version checked: `draughts_lite-rules-v1`_  
_Strategy guide: `games/draughts_lite/docs/COMPETENT-PLAYER.md`_

## At a glance

- Players move pieces on the dark squares of an 8 by 8 board.
- If any capture is available, capturing is mandatory.
- A capture may require the same piece to continue jumping.
- Men promote when they reach the far king row; a promoting jump ends that move.
- You win when your opponent has no pieces or no legal move.

## What you can see

The board, playable squares, piece owners, piece ranks, active seat, Rust legal action tree, selected path guidance, effects, terminal winner, and replay commands are public.

Not applicable - this is a perfect-information game. All game state needed for play is public in the normal game view.

## Setup

Pieces start on the playable dark squares in each player's starting rows. Men move forward; kings can move diagonally in either direction.

## On your turn

Choose one complete Rust-supplied move path.

If no capture is available, choose an origin and one quiet diagonal landing. If a capture is available anywhere, quiet moves are absent and you must choose a capture path. If the same piece can keep jumping, continue choosing jump landings until Rust marks the path complete.

## Actions

### Choose piece

Select a Rust-legal origin piece.

### Move

Move quietly to an empty diagonal landing when no capture is available.

### Jump

Jump over an adjacent opposing piece to an empty landing and remove the captured piece. Continue jumping with the same piece when Rust requires it.

### Cancel

Clear your in-progress UI selection before submitting a complete move. Cancel is UI-only and does not create a replay command.

## Scoring and winning

Draughts Lite has no running point score.

After a completed move, if your opponent has no pieces, you win. If your opponent still has pieces but no legal move, you also win. Draw claims, repetition, clocks, and tournament adjudication are not part of this variant.

## Hidden information and reveal timing

Not applicable - this is a perfect-information game.

## Common terms

| Term | Meaning |
|---|---|
| Man | A normal piece that moves forward diagonally. |
| King | A promoted piece that can move diagonally in either direction. |
| Quiet move | A non-capturing diagonal move. |
| Jump | A capturing move over an opposing piece. |
| Forced continuation | A required next jump by the same piece. |

## What this page is not

This page teaches rules and turn flow. It is not strategy advice, and it is not the formal implementation contract.

## Source notes for maintainers

The formal rule source is `games/draughts_lite/docs/RULES.md`.

The formal rules version checked is `draughts_lite-rules-v1`.

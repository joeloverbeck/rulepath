# Column Four - How to Play

_Game ID: `column_four`_  
_Formal rules source: `games/column_four/docs/RULES.md`_  
_Formal rules version checked: `column_four-rules-v1`_  
_Strategy guide: `games/column_four/docs/COMPETENT-PLAYER.md`_

## At a glance

- Players take turns choosing columns on a 7 by 6 board.
- Your piece drops to the lowest empty cell in the chosen column.
- Full columns are not legal choices.
- Connect four of your pieces horizontally, vertically, or diagonally to win.
- If the board fills with no four-in-a-row, the game is a draw.

## What you can see

The board, pieces, active seat, legal columns, previews, terminal winner, winning line, and draw outcome are public.

Not applicable - this is a perfect-information game. All game state needed for play is public in the normal game view.

## Setup

The board starts empty. Player 1 acts first, then players alternate after each non-terminal placement.

## On your turn

Choose one legal column.

Rust places your piece in the lowest empty row of that column, checks for a line, and either ends the game or passes the turn.

## Actions

### Drop

Choose a non-full column. The piece falls to that column's lowest open cell.

## Scoring and winning

Column Four has no running score.

You win immediately when a placement creates four contiguous pieces of your color horizontally, vertically, or diagonally. If one placement creates multiple lines, Rust chooses one deterministic primary line to highlight. If every cell is filled with no winning line, the game is a draw.

## Hidden information and reveal timing

Not applicable - this is a perfect-information game.

## Common terms

| Term | Meaning |
|---|---|
| Column | One vertical lane where a piece can be dropped. |
| Full column | A column with no empty cells; it is not legal to choose. |
| Winning line | Four connected pieces owned by the same player. |
| Draw | A full board with no winning line. |

## What this page is not

This page teaches rules and turn flow. It is not strategy advice, and it is not the formal implementation contract.

## Source notes for maintainers

The formal rule source is `games/column_four/docs/RULES.md`.

The formal rules version checked is `column_four-rules-v1`.

# Three Marks - How to Play

_Game ID: `three_marks`_  
_Formal rules source: `games/three_marks/docs/RULES.md`_  
_Formal rules version checked: `three_marks-rules-v1`_  
_Strategy guide: Not applicable - no separate strategy guide is required for this small perfect-information game._

## At a glance

- Players take turns placing marks on a 3 by 3 board.
- Each turn places one mark in one empty cell.
- Complete any row, column, or diagonal with your marks to win.
- If all nine cells fill with no winning line, the game is a draw.
- Occupied cells and terminal boards are not legal move targets.

## What you can see

The whole board is public: all marks, empty cells, the active seat, legal placement targets, the winning line if one exists, and draw status if the board fills.

Not applicable - this is a perfect-information game. All game state needed for play is public in the normal game view.

## Setup

The board starts empty. Player 1 takes the first turn, then players alternate after each non-terminal placement.

## On your turn

Choose one empty cell offered by the interface.

After your mark is placed, Rust checks for a winning line first. If no one has won and the board is not full, the turn passes to the other player.

## Actions

### Place

Place your mark in one empty cell. The UI presents one legal placement control per available cell.

## Scoring and winning

Three Marks has no running score.

You win immediately by owning all three cells in any row, column, or diagonal. If the final placement fills the board and also completes a line, the line win takes precedence. If the full board has no line, the result is a draw.

## Hidden information and reveal timing

Not applicable - this is a perfect-information game.

## Common terms

| Term | Meaning |
|---|---|
| Cell | One square on the 3 by 3 board. |
| Mark | A player's placed symbol. |
| Line | Three cells in a row, column, or diagonal. |
| Draw | A full board with no winning line. |

## What this page is not

This page teaches rules and turn flow. It is not strategy advice, and it is not the formal implementation contract.

## Source notes for maintainers

The formal rule source is `games/three_marks/docs/RULES.md`.

The formal rules version checked is `three_marks-rules-v1`.

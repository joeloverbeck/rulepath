# River Ledger - How to Play

_Game ID: `river_ledger`_
_Formal rules source: `games/river_ledger/docs/RULES.md`_
_Formal rules version checked: `river-ledger-rules-v1`_
_Strategy guide: `games/river_ledger/docs/COMPETENT-PLAYER.md`_

## At a glance

River Ledger is a 3-6 seat hidden-card contribution game.

- Each seat receives two private cards.
- Five community cards can be revealed over the flop, turn, and river.
- Seats add abstract contribution units to one shared pot.
- The game uses fixed contribution units and a capped number of raises per street.
- A hand can end when everyone but one seat folds.
- If more than one seat remains after the river, the strongest five-card hand wins, and exact ties split the pot.

## What you can see

You can always see public match information:

- seat order and the button, small blind, and big blind;
- the active seat;
- the current street;
- the public board cards already revealed;
- public contribution totals;
- folded/live status;
- the final pot allocation after the hand ends.

You can see your own two private cards. You cannot see another seat's private cards unless the hand reaches an authorized showdown reveal.

No browser view shows future board cards, burn positions, or the deck tail.

## Setup

A standard match uses 3, 4, 5, or 6 seats.

At the start:

- the button, small blind, and big blind are assigned from seat order;
- the small blind and big blind post forced abstract contributions;
- each seat receives two private cards;
- the remaining board cards are reserved internally;
- preflop action starts after the big blind by seat order.

## On your turn

The UI offers only legal Rust-generated actions for the current position.

If you owe no additional contribution, you may usually check or open the street with a bet.

If another seat has opened the street, you may call, raise if the cap allows it, or fold.

## Actions

### Fold

Use **Fold** to leave the hand. If only one live seat remains, that seat wins immediately.

### Check

Use **Check** when you owe no additional contribution and no bet is open.

### Call

Use **Call** to add exactly enough contribution units to match the current live amount.

### Bet

Use **Bet** to open contribution on a street when no bet is already open.

### Raise

Use **Raise** to call the current amount and add one more street unit. Each street allows one opening bet plus three raises.

## Scoring and winning

There are two terminal paths:

1. **Last live hand:** all other live seats fold. The remaining seat wins the pot without revealing folded seats' private cards.
2. **Showdown:** betting closes on the river with at least two live seats. Each live seat makes the best five-card hand from its two private cards and the five public cards.

Showdown compares hand category first, then ordered tie-break ranks. Suits do not break ties.

If one seat has the strongest hand, that seat wins the pot. If seats tie exactly, they split the pot; any remainder is assigned by the stable button-order rule.

## Hidden information and reveal timing

Your private cards belong only to your seat view before showdown.

Other seats' private cards stay hidden from you before showdown. Folded seats' private cards stay hidden if the hand ends by last-live foldout.

Future community cards, burn advancement, and deck-tail identities are internal. They are not shown in views, action trees, diagnostics, effects, public replay exports, bot explanations, DOM text, storage, or logs.

At showdown, only authorized showdown information is revealed through Rust-authored outcome fields.

## Common terms

| Term | Meaning |
|---|---|
| Seat | A player position in stable order. |
| Button | The public marker used for blind assignment and split remainders. |
| Blind | A forced opening contribution. |
| Street | Preflop, flop, turn, river, or showdown. |
| Board | The public community cards already revealed. |
| Pot | The shared total of abstract contribution units. |
| Raise cap | The limit of one opening bet plus three raises per street. |
| Last live hand | A foldout result where one seat remains live. |
| Showdown | The final comparison when multiple live seats reach the end. |

## What this page is not

This page teaches the rules and turn flow. It is not strategy advice, and it is not the formal implementation contract.

For maintainers, the formal rule source is `games/river_ledger/docs/RULES.md`. Strategy and competent-play notes belong in `games/river_ledger/docs/COMPETENT-PLAYER.md`.

## Source notes for maintainers

The formal rule source is `games/river_ledger/docs/RULES.md`.

The formal rules version checked is `river-ledger-rules-v1`.

Strategy and competent-play notes belong in `games/river_ledger/docs/COMPETENT-PLAYER.md`.

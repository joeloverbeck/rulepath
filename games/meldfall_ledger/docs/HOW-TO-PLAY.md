# Meldfall Ledger - How to Play

_Game ID: `meldfall_ledger`_
_Formal rules source: `games/meldfall_ledger/docs/RULES.md`_
_Formal rules version checked: `meldfall-ledger-rules-v1`_
_Strategy guide: `games/meldfall_ledger/docs/COMPETENT-PLAYER.md`_

## At a glance

Meldfall Ledger is a hidden-hand public-meld game for two to six seats.

- Each round deals private hands from one standard deck.
- One public discard starts beside a hidden stock.
- On your turn, draw from stock or pick up from the discard pile.
- Discard pickups create an immediate-use commitment for the selected card.
- You may table same-rank sets, same-suit runs, and lay-offs onto any public meld.
- Tabled cards score positive points for the seat that played them.
- Cards left in hand subtract when the round settles.
- The first unique highest seat at or above 500 wins; a tie at 500 or above continues.

## What you can see

You can always see public match information:

- seat order, active seat, dealer, phase, and score ledger;
- stock count, not stock order;
- discard pile cards in public oldest-to-newest order;
- all public meld groups and laid-off cards;
- hand counts for every seat;
- round settlement totals and terminal standings after Rust publishes them.

You can see your own hand while seated. You cannot see other seats' unmelded cards, the hidden stock order, private stock draws for other seats, or hidden card-derived bot inputs.

## Setup

Rust creates the match from the seed, rules version, data version, standard variant, and selected seat count. Meldfall Ledger supports two, three, four, five, or six seats, with four seats as the default.

The game uses one standard 52-card deck with no jokers or wild cards. Two-seat games deal 13 cards to each seat. Three- through six-seat games deal 7 cards to each seat. One card starts the public discard pile, and the remaining cards form the hidden stock.

The dealer and first active seat are deterministic. Play moves clockwise.

## On your turn

Start by choosing a Rust-supplied draw source.

If you draw from stock, one hidden card enters your hand. Observers and other seats see that a stock draw happened and the new stock count, not the card identity.

If you draw from discard, you take the selected public discard and every newer discard above it. The selected card must be used immediately in a new meld or legal lay-off before you may discard or finish.

After drawing, you may table legal melds or lay off cards onto existing public melds. If your hand is not empty and no pickup commitment remains, discard one card to end the turn. If your hand is empty after table plays, the round can end without a final discard.

## Actions

### Draw stock

Take one hidden stock card into your hand. The public effect reports counts only.

### Draw discard

Choose a public discard index. You take that card plus all newer discard cards, and Rust records the selected card as an immediate-use commitment.

### Meld new

Table a new set or run from your own hand. A set is three or four cards of the same rank with distinct suits. A run is three or more consecutive cards in one suit. Aces can be low or high, but runs do not wrap around.

### Lay off

Add one card from your hand to the start or end of an existing public meld when the resulting meld remains legal. You may lay off onto your own or another seat's public meld.

### Discard or finish

Discard one owned card to end your turn, or finish after table plays when Rust says that is legal. Empty-hand finishes settle the round.

## Scoring and winning

Each tabled card scores positive points for the seat that played it. Lay-offs score for the seat that laid off the card, not for the original meld owner.

At round settlement, each seat's round delta is tabled card points minus the value of cards still in that seat's hand. Aces are worth 15 points, face cards and tens are worth 10, and pip cards are worth their number.

Round deltas add to cumulative match scores. Scores can be negative. After settlement, if exactly one seat has the highest score and that score is at least 500, that seat wins. If the highest eligible score is tied, the match continues.

## Hidden information and reveal timing

Your private hand is visible only in your own seat view. Other seats and observers see your hand count, not your unmelded card identities.

The stock order is never exposed. A stock draw reveals the drawn card only to the drawing seat. Public observers and other seats receive viewer-filtered effects and replay exports that keep stock card identities hidden.

Discard pile cards are public once discarded. Melded and laid-off cards become public when Rust accepts the table action. Settlement exposes public totals, counts, deltas, scores, ranks, and winners without revealing unauthorized unmelded card identities.

Action trees are viewer-scoped. Private hand-dependent meld, lay-off, and discard choices are available only to the active seat's authorized view.

## Common terms

| Term | Meaning |
|---|---|
| Stock | The hidden face-down draw pile. |
| Discard pile | Public cards available for discard pickup, ordered oldest to newest. |
| Set | Three or four same-rank cards with distinct suits. |
| Run | Three or more consecutive cards in one suit. |
| Lay off | Add a legal card to an existing public meld. |
| Pickup commitment | The selected discard card that must be used immediately after a discard draw. |
| Round delta | Tabled positives minus in-hand penalties for one settlement. |

## What this page is not

This page teaches the public rules and turn flow. It is not strategy advice, and it is not the formal implementation contract.

Formal rule IDs, Rust validation notes, rule coverage, bot strategy, replay evidence, benchmark thresholds, and implementation details belong in the other game docs.

## Source notes for maintainers

The formal rule source is `games/meldfall_ledger/docs/RULES.md`.

The formal rules version checked is `meldfall-ledger-rules-v1`.

Meldfall Ledger is the neutral Rulepath catalog name. The source-family label appears in source notes and formal docs, not as the catalog title.

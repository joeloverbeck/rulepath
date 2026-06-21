# Vow Tide - How to Play

_Game ID: `vow_tide`_
_Formal rules source: `games/vow_tide/docs/RULES.md`_
_Formal rules version checked: `vow-tide-rules-v1`_
_Strategy guide: `games/vow_tide/docs/COMPETENT-PLAYER.md`_

## At a glance

Vow Tide is a hidden-hand exact-bid trick-taking game for three to seven seats.

- Each hand deals the same number of private cards to every seat.
- One public trump indicator sets the trump suit for that hand.
- Everyone bids exactly how many tricks they expect to take.
- The dealer bids last and may be blocked from one bid value by the hook.
- Players follow the led suit if they can.
- Exact bids score; misses score zero for that hand.
- After the fixed hand schedule ends, the highest cumulative score wins. Tied leaders share the win.

## What you can see

You can always see public table information:

- seat labels and clockwise order;
- the dealer, active seat, hand number, and current hand size;
- the public trump indicator;
- submitted bids;
- each seat's hand count and trick count;
- cards already played to the current or completed tricks;
- completed hand scores and terminal standings.

You can see your own unplayed hand while seated. You cannot see other seats' unplayed cards or the hidden stock card identities.

## Setup

Vow Tide uses a standard 52-card deck with four suits and ranks from 2 through ace.

For the selected seat count, Rust computes the maximum hand size, deals hands from that size down to one card, then back up. The one-card hand appears once.

At the start of each hand:

- Rust shuffles deterministically from the match seed and hand index;
- cards are dealt clockwise until every seat has the scheduled hand size;
- the next undealt card becomes the public trump indicator;
- any remaining undealt cards stay hidden as stock;
- bidding starts with the seat left of the dealer.

## On your turn

During bidding, choose one legal bid shown by the UI. Bids are public and cannot be changed after Rust accepts them.

During trick play:

- if you lead, you may play any card from your hand;
- if another seat led, you must play the led suit when you have it;
- if you do not have the led suit, you may play any card from your hand, including trump.

The UI only presents Rust-supplied legal choices for the current seat and position.

## Actions

### Bid

Choose a number from zero through the current hand size. If you are the dealer, one otherwise normal bid can be unavailable because the dealer's bid may not make total table bids equal the hand size.

### Play

Choose one legal card from your hand. Played cards are public immediately and remain visible in trick history.

## Scoring and winning

At the end of a hand, compare each seat's bid with that seat's tricks taken.

- If tricks taken exactly equals the bid, that seat scores `10 + bid`.
- If tricks taken is under or over the bid, that seat scores zero for the hand.
- A successful zero bid scores 10.

Scores accumulate across the whole match and never decrease. The match ends after the final scheduled hand. The seat or seats with the highest cumulative score win.

## Hidden information and reveal timing

Your private hand is visible only in your own seat view. Other seats and observers see your hand count, not your card identities.

Other seats' unplayed hands are never shown to you. Hidden stock identities and order are never shown in browser views, replay exports, diagnostics, bot explanations, or effects.

A card becomes public only when it is the trump indicator or when a player plays it to a trick. Public bids, played cards, trick winners, hand scores, and final standings remain public after they happen.

## Common terms

| Term | Meaning |
|---|---|
| Bid | A public promise of exactly how many tricks a seat expects to take this hand. |
| Dealer hook | The dealer cannot choose the one bid that would make all bids add up to the hand size. |
| Led suit | The suit of the first card played to a trick. |
| Trump | The suit of the public trump indicator; trump cards beat non-trump cards in a trick. |
| Stock | Undealt cards kept internal by Rust after the trump indicator. |
| Exact bid | A hand result where tricks taken equals the seat's bid. |

## What this page is not

This page teaches the public rules and turn flow. It is not strategy advice, and it is not the formal implementation contract.

## Source notes for maintainers

The formal rule source is `games/vow_tide/docs/RULES.md`.

The formal rules version checked is `vow-tide-rules-v1`.

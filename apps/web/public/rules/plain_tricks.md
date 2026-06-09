# Plain Tricks - How to Play

_Game ID: `plain_tricks`_
_Formal rules source: `games/plain_tricks/docs/RULES.md`_
_Formal rules version checked: `plain-tricks-rules-v1`_
_Strategy guide: `games/plain_tricks/docs/COMPETENT-PLAYER.md`_

## At a glance

Plain Tricks is a two-player hidden-hand trick-taking game.

- Each round deals six private cards to each seat.
- Six undealt cards form an internal tail that is never shown.
- The leader plays any card from hand.
- The follower must play the led suit if they have one.
- The higher card in the led suit wins the trick.
- The trick winner leads the next trick.
- The match has two rounds of six tricks. Most total tricks wins; 6-6 is a split.

## What you can see

You can always see public match information:

- whose turn it is;
- the current round and trick;
- each seat's hand count;
- played cards in the current trick;
- resolved trick history;
- round trick counts and match totals;
- terminal outcome after the match ends.

You can see your own unplayed hand. You cannot see your opponent's unplayed cards.

The tail is never shown in the browser, including at terminal.

## Setup

The deck has eighteen cards: Gale 1-6, River 1-6, and Ember 1-6.

At the start of each round:

- Rust shuffles the full deck with deterministic seeded RNG;
- each seat receives six private cards;
- six cards are left as the internal tail;
- round 1 starts with `seat_0` leading;
- round 2 starts with `seat_1` leading.

## On your turn

If you lead a trick, you may play any card in your hand.

If you follow a trick, the led suit controls your legal choices:

- if you have at least one card of the led suit, you must play one of those cards;
- if you have no card of the led suit, you may play any card from your hand.

The UI only offers legal Rust-supplied play choices for the current position.

## Actions

### Play

Choose one legal card from your hand.

The available card buttons come from Rust's legal action tree. If a card is not legal for the current trick, it is not clickable.

## Scoring and winning

Each trick is worth one point to the trick winner.

After six tricks, the round ends. Round 1 deals a fresh second round. Round 2 ends the match.

After both rounds:

1. The seat with more total tricks wins.
2. If totals are tied 6-6, the match is split.

## Hidden information and reveal timing

Your unplayed hand belongs only to your seat view. Opponent and observer views show hand counts, not card identities.

A card becomes public only when it is played to the current trick. Played cards remain public in trick history.

The tail remains internal and is never revealed.

## Common terms

| Term | Meaning |
|---|---|
| Led suit | The suit of the first card played to a trick. |
| Follower | The seat playing the second card of a trick. |
| Tail | The six undealt cards kept internal by Rust. |
| Trick history | Public resolved tricks with played cards and winners. |

## What this page is not

This page teaches rules and turn flow. It is not strategy advice, and it is not the formal implementation contract.

## Source notes for maintainers

The formal rule source is `games/plain_tricks/docs/RULES.md`.

The formal rules version checked is `plain-tricks-rules-v1`.

# Blackglass Pact - How to Play

_Game ID: `blackglass_pact`_
_Formal rules source: `games/blackglass_pact/docs/RULES.md`_
_Formal rules version checked: `blackglass-pact-rules-v1`_
_Strategy guide: `games/blackglass_pact/docs/COMPETENT-PLAYER.md`_

## At a glance

Blackglass Pact is a four-player partnership trick-taking game.

- Seats are North, East, South, and West.
- North and South are partners; East and West are partners.
- Each hand may open with blind-nil choices before any cards are dealt.
- After the deal, each non-blind player bids nil or 1 through 13.
- Spades are always trump.
- Teams score from ordinary bids, nil bids, blind nils, overtrick bags, and bag penalties.
- The first team to reach the target threshold wins when the hand is scored.

## What you can see

You can always see public match information:

- dealer, hand number, phase, and active seat;
- team scores, team bags, partnerships, public bids, and team contracts;
- each seat's public hand count;
- whether spades have been broken;
- cards played to the current trick and completed trick winners;
- completed hand scoring and match outcome after Rust publishes them.

You can see your own hand after the deal. You cannot see partner or opponent
hands, undealt deck order, future deals, or hidden card-derived bot inputs.

## Players, Seats, And Roles

Blackglass Pact supports exactly four seats. North is `seat_0`, East is
`seat_1`, South is `seat_2`, and West is `seat_3`.

Partners sit opposite each other. North-South is `team_0`; East-West is
`team_1`. Partnership is public, but it does not grant access to a partner's
hand or private legal choices.

## Setup

Rust creates the fixed-four match from the match seed, rules version, data
version, and standard variant. Seat order is clockwise: North, East, South,
West.

At the start of a hand, seats whose team trails by enough points may receive a
blind-nil choice before the deal. Once those choices are finished, Rust shuffles
and deals a standard 52-card deck until every seat has 13 private cards.

## On your turn

If you are active in the blind-nil window, choose whether to declare or decline.
This choice happens before you see a hand.

If bidding is active for your seat, choose nil or a number from 1 through 13.
Blind-nil declarers skip ordinary bidding.

If trick play is active for your seat, play one legal card from your hand. You
must follow the led suit when you can. If you cannot follow suit, you may play
another legal card. Spades are trump, and a legal spade play can break spades.

The UI only offers Rust-supplied legal choices for the current position.

## Actions

### Blind nil

Declare or decline blind nil before the deal when Rust marks your seat eligible.
The decision is public and cannot be changed.

### Bid

Choose nil or a numeric bid from 1 through 13. Your accepted bid is public and
locked for the hand.

### Play

Play one legal card to the current trick. A played card becomes public
immediately.

## Scoring and winning

Each team has an ordinary contract equal to the sum of its positive numeric
bids. Nil and blind-nil bids are scored for the individual seat and do not add
to the ordinary contract.

A made ordinary contract earns 10 points per contracted trick plus one point
and one bag for each ordinary overtrick. A set ordinary contract loses 10
points per contracted trick.

A made nil earns 100 points; a failed nil loses 100 points. A made blind nil
earns 200 points; a failed blind nil loses 200 points. Tricks taken by failed
nil or blind-nil seats also add bags. Every 10 raw bags applies a 100-point
penalty and leaves the remainder as the next bag count.

After a hand is scored, Rust advances the dealer or publishes the final team
outcome.

## Hidden information and reveal timing

Before the deal, no hand identities exist in any browser view, action tree,
preview, bot input, effect, or replay export.

After the deal, your unplayed cards belong only to your seat view. Other seats
and the observer see hand counts, public bids, public trick cards, scores, and
published scoring records, not hidden card identities. Partner status never
reveals another hand.

Deck order, future deals, private legal choices, private bot inputs, and
seed-reconstructable hidden setup data are never exposed to unauthorized views,
DOM payloads, storage, logs, effects, or replay exports. A card becomes public
only when Rust publishes it as played or as part of an authorized owner-private
seat view.

## Common terms

| Term | Meaning |
|---|---|
| Blind nil | A pre-deal zero-trick commitment by an eligible trailing seat. |
| Nil | A zero-trick ordinary bid made after seeing the hand. |
| Contract | The ordinary positive numeric bid total for a team. |
| Bag | An overtrick counter that can trigger a penalty. |
| Trump | The suit that beats non-trump cards; spades are always trump. |
| Led suit | The suit of the first card played to a trick. |

## What this page is not

This page teaches rules and turn flow. It is not strategy advice, and it is not
the formal implementation contract.

Formal rule IDs, Rust validation notes, rule coverage, bot strategy, replay
evidence, benchmark thresholds, and implementation details belong in the other
game docs.

## Source Notes For Maintainers

Confirm before merging:

- [x] Prose is original Rulepath wording.
- [x] No copied rulebook text, examples, diagrams, assets, names, fonts, or
  trade dress.
- [x] No strategy advice copied from `COMPETENT-PLAYER.md`.
- [x] No hidden match-state examples or seed-specific data.
- [x] No YAML front matter.
- [x] No selectors, conditions, triggers, or action schemas.
- [x] Formal rules version checked matches `RULES.md`.

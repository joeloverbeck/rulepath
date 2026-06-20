# Briar Circuit - How to Play

_Game ID: `briar_circuit`_
_Formal rules source: `games/briar_circuit/docs/RULES.md`_
_Formal rules version checked: `briar-circuit-rules-v1`_
_Strategy guide: `games/briar_circuit/docs/COMPETENT-PLAYER.md`_

## At a glance

Briar Circuit is a four-player hidden-hand trick-taking game about avoiding
penalty cards.

- Each hand deals 13 private cards to each seat.
- Most hands begin with a private three-card pass; every fourth hand has no pass.
- The holder of 2 clubs opens the hand.
- Players must follow the led suit when they can.
- Hearts and queen of spades add penalty points when captured.
- Lower score is better. Once a completed hand pushes any score to 100 or more,
  the unique lowest score wins.

## What you can see

You can always see public match information:

- pass direction and pass progress;
- whose turn it is;
- the current trick and cards already played to it;
- completed tricks and their winners;
- each seat's public hand count;
- whether hearts have been broken;
- hand scores, cumulative scores, moon events, and terminal standings after
  they are public.

You can see your own private hand. You cannot see other seats' private hands or
unplayed cards.

During a pass hand, you can see your own staged pass selection. Other seats and
the observer can see only pass progress, not selected card identities. After the
exchange, passed cards become part of their new owner's hand, but the pass
origin remains private.

Deck order and future deal facts are never shown in the browser.

## Players, Seats, And Roles

Briar Circuit supports exactly four players. Every seat plays independently;
there are no teams, partnerships, coalitions, or special powers.

Turn order follows the trick. One seat leads, the other seats play clockwise,
and the trick winner leads the next trick. The dealer and pass direction rotate
between hands.

## Setup

Rust shuffles a standard 52-card deck with deterministic seeded randomness and
deals all cards clockwise until each seat has 13 cards. The initial dealer is
fixed for reproducible setup, then the dealer rotates after every completed
hand.

On pass hands, the pass direction is left, right, across, then hold/no-pass,
repeating for the match.

## On your turn

If the hand is in the pass phase, choose exactly three cards from your hand and
confirm them. The exchange happens only after all four seats have confirmed.

If you lead the first trick of a hand, the only legal opening play is 2 clubs.

For later tricks:

- the first card played to a trick sets the led suit;
- if you have any card of the led suit, you must play that suit;
- if you have none of the led suit, you may discard another legal card;
- on the first trick, you cannot discard a heart or queen of spades while you
  still have any non-point card available;
- hearts cannot be led before hearts are broken unless your hand contains only
  hearts.

The UI only offers legal Rust-supplied choices for the current position.

## Actions

### Select

Choose one card from your hand for the current pass. Selected cards stay private
to your seat.

### Unselect

Remove one staged pass card before confirming.

### Confirm pass

Commit exactly three selected cards. You cannot change the selection after
confirming. The exchange waits until every seat has confirmed.

### Play

Play one legal card to the current trick. A card becomes public when it is
played.

## Scoring and winning

Each captured heart is 1 penalty point. Capturing queen of spades is 13 penalty
points. Every other card is 0.

A complete hand has 26 raw penalty points total. If one seat captures all 26
points, that seat shoots the moon: the shooter adds 0 for the hand and each
other seat adds 26.

Hand additions are added to cumulative scores. After a completed hand, if any
score is at least 100, Rust checks the standings:

1. The unique lowest cumulative score wins.
2. If the lowest score is tied, the match continues with complete additional
   hands until the lowest score is unique.

The outcome explanation shows each seat's hand points, any moon adjustment,
cumulative scores, threshold/tie reason, and final standing.

## Hidden information and reveal timing

Your unplayed hand belongs only to your seat view. Opponents and the observer
see counts, not card identities.

Pass selections and incoming cards before exchange stay private. After a passed
card is later played, that card identity is public like any played card, but who
passed it remains private.

Deck order, future deals, hidden pass provenance, private legal choices, private
bot rationale, and seed-reconstructable hidden setup data are never exposed to
unauthorized browser views or replay exports.

## Common terms

| Term | Meaning |
|---|---|
| Led suit | The suit of the first card played to a trick. |
| Void | Having no card of the led suit. |
| Broken hearts | The state after a heart has been played, allowing hearts to be led normally. |
| Point card | Any heart, or queen of spades. |
| Moon | Capturing all 26 raw penalty points in one hand. |
| Pass provenance | The private fact of who originally passed a card. |

## What this page is not

This page teaches rules and turn flow. It is not strategy advice, and it is not
the formal implementation contract.

Formal rule IDs, Rust validation notes, rule coverage, bot strategy, and
implementation details belong in the other game docs.

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

# Starbridge Crossing - How to Play

_Game ID: `starbridge_crossing`_
_Formal rules source: `games/starbridge_crossing/docs/RULES.md`_
_Formal rules version checked: `starbridge-crossing-rules-v1`_

## At a glance

Starbridge Crossing is a public race across a six-pointed star board for two,
three, four, or six seats.

- Every seat starts with ten public pegs in its home point.
- Your target is the opposite point of the star.
- On your turn, move one of your pegs by one step or by a hop chain.
- Hops jump over adjacent occupied spaces into the empty space beyond.
- You may stop a hop chain after any legal landing.
- Seats earn finish ranks when all ten of their pegs reach their target home.

## What you can see

All game information is public. Every viewer can see the full board topology,
all peg locations, the active seat, legal Rust-supplied actions, public effects,
finish ranks, and terminal status.

Seat views may highlight a viewer's own pegs, but they do not reveal extra
rules facts because there are no private rules facts in this game.

## Setup

Rust creates the board, seats, homes, targets, pegs, and active seat from the
match seed, rules version, data version, standard variant, and selected seat
count.

The supported seat counts are exactly two, three, four, and six. Two seats use
opposite points. Three seats use alternating points. Four seats use two
opposite pairs. Six seats use every point.

Each active seat starts with ten pegs in its home point. Play begins with
`seat_0` and advances through the active seats in clockwise order.

## On your turn

Choose one Rust-supplied legal action for the active seat.

A step moves one of your pegs to an adjacent empty space. A hop moves one of
your pegs over one adjacent occupied space into the empty space immediately
beyond it. The jumped peg stays on the board.

After a hop, you may continue hopping with the same peg if Rust offers another
legal landing. A hop chain may change direction after each landing. You may stop
after any legal hop landing.

If the active seat has no legal step and no legal hop, Rust offers a forced
blocked pass.

## Actions

### Step

Move one owned peg into an adjacent empty space.

### Hop chain

Move one owned peg through one or more legal hop landings. The same turn cannot
revisit a landing space already used in that hop chain.

### Stop

End a hop chain after a legal landing.

### Pass blocked

Pass only when Rust says the active seat has no legal move.

## Scoring and winning

There are no points. Seats receive finish ranks.

When all ten of a seat's pegs occupy that seat's target home after an accepted
move, Rust assigns the next finish rank. Lower rank is better.

The match normally ends when all but one active seat have finish ranks. The last
unfinished seat receives the final rank. If a match reaches the deterministic
turn limit, Rust assigns unfinished standings by public progress and seat order.

## Hidden information and reveal timing

Hidden information is not applicable. Starbridge Crossing has no private hands,
hidden decks, concealed commitments, secret roles, private scores, or private
bot inputs.

Every board fact, legal action, effect, replay export, finish rank, and terminal
reason is public.

## Common terms

| Term | Meaning |
|---|---|
| Home | The point where a seat's pegs start. |
| Target | The opposite point where that seat tries to move all pegs. |
| Step | A one-space move into an adjacent empty space. |
| Hop | A jump over one adjacent occupied space into the empty space beyond. |
| Hop chain | One turn with one peg making one or more hops. |
| Finish rank | The order in which seats complete their target home. |

## What this page is not

This page teaches the public rules and turn flow. It is not strategy advice, and
it is not the formal implementation contract.

Formal rule IDs, Rust validation details, rule coverage, bot evidence, replay
fixtures, benchmark thresholds, and implementation notes belong in the other
game docs.

## Source notes for maintainers

The formal rule source is `games/starbridge_crossing/docs/RULES.md`.

The formal rules version checked is `starbridge-crossing-rules-v1`.

Starbridge Crossing is the neutral Rulepath catalog name. Source-family labels
appear in source notes and formal docs, not as the catalog title.

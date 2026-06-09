# Crest Ledger - How to Play

_Game ID: `poker_lite`_  
_Formal rules source: `games/poker_lite/docs/RULES.md`_  
_Formal rules version checked: `poker-lite-rules-v1`_  
_Strategy guide: `games/poker_lite/docs/COMPETENT-PLAYER.md`_

## At a glance

Crest Ledger is a two-player hidden-crest pledge game.

- You have one private crest. Your opponent cannot see it until a showdown happens.
- A center crest starts hidden. It is revealed only if the first pledge round closes without a yield.
- Players add pledge markers to a shared pool over up to two rounds.
- If a player yields while facing pressure, the other player wins the pool immediately and private crests stay hidden.
- If both rounds close without a yield, the private crests reveal together and the stronger crest result wins the pool.
- A private crest that matches the center crest's rank makes a pair. A pair beats no pair.

## What you can see

You can always see public match information:

- whose turn it is;
- the current pledge round;
- the shared pool;
- each player's public contribution;
- whether there is an outstanding pledge to answer;
- whether the round's lift has already been used;
- the center crest after it has been revealed.

You can see your own private crest. You cannot see your opponent's private crest unless the game reaches showdown.

No browser view shows the hidden deck tail. A yield result does not reveal either player's private crest.

## Setup

Each match uses six crests: two low crests, two middle crests, and two high crests. The copies have different names, but copy names do not break ties.

At the start:

- each player receives one private crest;
- one center crest is set aside face down;
- each player contributes one marker to the pool;
- round 1 begins with player 1 active.

## On your turn

Your available actions depend on whether you are facing an outstanding pledge.

If there is no outstanding pledge, you may either keep the round calm or create pressure.

If you are facing an outstanding pledge, you must answer it by matching, lifting, or yielding.

The UI only offers legal actions for the current position.

## Actions

### Hold

Use **Hold** when there is no outstanding pledge. You add no markers.

If both players hold in the same round, that round closes.

### Press

Use **Press** when there is no outstanding pledge. You add the current round's pledge unit to your contribution and ask your opponent to answer.

Round 1 uses one-marker pressure. Round 2 uses two-marker pressure.

### Match

Use **Match** when you are facing an outstanding pledge. You add enough markers to equal your opponent's contribution, then the round closes.

### Lift

Use **Lift** when you are facing an outstanding pledge and the round's lift has not been used. You match the outstanding pledge, add one more pledge unit, and send pressure back to your opponent.

Only one lift can be used in a round.

### Yield

Use **Yield** when you are facing an outstanding pledge and do not want to continue. The match ends immediately. Your opponent wins the pool, and private crests are not revealed.

## How rounds close

A round can close because both players held or because an outstanding pledge was matched.

If round 1 closes without a yield, the center crest is revealed and round 2 begins with player 2 active.

If round 2 closes without a yield, the game goes to showdown.

## Scoring and winning

There are two ways to win the pool:

1. **Yield win:** your opponent yields while facing your pressure. You win immediately.
2. **Showdown win:** both rounds close without a yield, then private crests reveal and the stronger result wins.

At showdown:

- first, check whether each private crest matches the center crest's rank;
- a matching private crest makes a pair;
- any pair beats any non-pair;
- if both players have the same pair status, the higher private crest rank wins;
- if both players are still tied, the pool is split evenly.

Crest copy names do not break ties.

## Hidden information and reveal timing

Your private crest belongs to your view. Your opponent's private crest stays hidden from you until showdown.

The center crest stays hidden during round 1. It is revealed only if round 1 closes without a yield.

Private crests reveal together only at showdown. If the match ends by yield, private crests stay hidden.

The hidden deck tail is never shown in the browser.

## Common terms

| Term | Meaning |
|---|---|
| Crest | The private or center item used to determine showdown strength. |
| Center crest | The shared crest that starts hidden and may later be revealed. |
| Pair | A private crest whose rank matches the center crest's rank. |
| Pool | The shared markers awarded to the winner or split on a true tie. |
| Pledge round | One of the game's two pressure rounds. |
| Outstanding pledge | A contribution lead that the other player must answer. |
| Lift | A one-per-round raise after matching pressure. |
| Yield | Ending the match immediately while giving the pool to the other player. |

## What this page is not

This page teaches the rules and turn flow. It is not strategy advice, and it is not the formal implementation contract.

For maintainers, the formal rule source is `games/poker_lite/docs/RULES.md`. Strategy and competent-play notes belong in `games/poker_lite/docs/COMPETENT-PLAYER.md`.

## Source notes for maintainers

The formal rule source is `games/poker_lite/docs/RULES.md`.

The formal rules version checked is `poker-lite-rules-v1`.

Strategy and competent-play notes belong in `games/poker_lite/docs/COMPETENT-PLAYER.md`.

The per-seat maximum contribution cap is deliberately omitted from the player-facing body because the UI offers only legal actions; the cap remains part of the formal implementation contract in `RULES.md`.

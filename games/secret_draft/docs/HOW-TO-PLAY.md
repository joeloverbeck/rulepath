# Veiled Draft - How to Play

_Game ID: `secret_draft`_  
_Formal rules source: `games/secret_draft/docs/RULES.md`_  
_Formal rules version checked: `secret-draft-rules-v1`_  
_Strategy guide: `games/secret_draft/docs/COMPETENT-PLAYER.md`_

## At a glance

- Two players draft from one visible pool for six rounds.
- Each round, both players secretly commit to one visible item.
- Pending status is public, but committed item identities stay hidden until both players have committed.
- The two choices reveal together, awards resolve, and drafted items leave the pool.
- After the sixth reveal, score and public tie-breakers decide the winner.

## What you can see

You can see the visible pool, round number, priority seat, drafted collections, score summary, pending booleans, reveal history, and terminal outcome.

Before reveal, you cannot see either hidden committed item identity in browser-facing views, including your own committed item after submission. You only see that a seat is committed or waiting.

## Setup

The visible item pool starts in a fixed public order. Scores and drafted collections start empty. Player 1 is the first priority seat, and priority alternates each round.

## On your turn

If you have not committed this round, choose one item from the visible pool. Your choice is stored by Rust and shown publicly only as a pending commitment.

When both players have committed, Rust reveals both choices together and resolves awards. The next round begins unless the sixth reveal just ended the game.

## Actions

### Commit

Choose one currently visible item. The other player's hidden commitment does not remove that item from your legal choices before reveal.

## Scoring and winning

Drafted items score their public values. Complete sets of one ember, one tide, and one grove add bonus points. Having at least three drafted items in a thread can add a high-thread bonus. A terminal conflict-discipline bonus may apply if a player received no fallback awards.

After the sixth reveal, compare:

1. Higher total score.
2. More complete ember/tide/grove sets.
3. Higher single drafted item value.
4. More distinct represented threads.
5. Fewer priority-won contested items.
6. Draw if all comparisons tie.

## Hidden information and reveal timing

A committed item identity is hidden from all browser-facing viewers until the synchronized reveal batch. Pending booleans are public; item IDs are not.

If both players commit to different items, both chosen items are awarded and removed. If both commit to the same item, the priority seat receives that contested item, and the other seat receives a deterministic fallback item after reveal.

Commitments do not carry into later rounds. After reveal, the revealed choices, awards, and scoring facts are public history.

## Common terms

| Term | Meaning |
|---|---|
| Visible pool | The public items still available to choose. |
| Commitment | A hidden submitted item choice for the current round. |
| Priority seat | The player who wins a contested item this round. |
| Fallback award | The public item awarded to the non-priority player after a contested choice. |
| Reveal batch | The synchronized public reveal after both commitments exist. |

## What this page is not

This page teaches rules and turn flow. It is not strategy advice, and it is not the formal implementation contract.

## Source notes for maintainers

The formal rule source is `games/secret_draft/docs/RULES.md`.

The formal rules version checked is `secret-draft-rules-v1`.

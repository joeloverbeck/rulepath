# High Card Duel - How to Play

_Game ID: `high_card_duel`_  
_Formal rules source: `games/high_card_duel/docs/RULES.md`_  
_Formal rules version checked: `high-card-duel-rules-v1`_  
_Strategy guide: `games/high_card_duel/docs/COMPETENT-PLAYER.md`_

## At a glance

- Each player has a private hand.
- In each round, the lead player commits one card face down, then the reply player commits one card without seeing the lead card.
- Both committed cards reveal together.
- The higher revealed rank scores the round; equal ranks do not award a point.
- The match ends after six rounds. Higher score wins; equal score is a draw.

## What you can see

You can see public round number, score, phase, active/lead/reply seats, hand counts, deck count, face-down commitment occupancy, revealed cards, and terminal result.

You can see your own hand and your own committed card after you commit it. You cannot see your opponent's hand, opponent's committed card identity before reveal, unrevealed deck order, or future draws.

## Setup

Rust shuffles the deck deterministically from the match seed, deals private hands, keeps the remaining deck order internal, and starts round 1 with player 1 as lead.

## On your turn

If you are the active committing player, choose one card from your own hand. The card becomes a face-down commitment.

When both players have committed, the cards reveal together, the round is scored, revealed cards move to public history, hands refill when possible, and the next round begins unless the sixth round just ended.

## Actions

### Commit

Choose one card from your own private hand. Opponent cards and deck cards are never legal choices.

## Scoring and winning

Each reveal compares the two committed ranks. The higher rank scores one point for that round. Equal ranks score no point.

After six rounds, the higher total score wins. If both players have the same score, the game is a draw.

## Hidden information and reveal timing

Your hand is private to your view. Your opponent's hand stays hidden from you.

A face-down commitment is public as an occupied slot, but its card identity stays hidden from the opponent and observers until the simultaneous reveal. Your own committed card may remain visible to you after you commit.

Both committed cards reveal at the same time. Unrevealed deck order, unused deck tail, and future draws are never shown in public browser views or public replay exports.

## Common terms

| Term | Meaning |
|---|---|
| Hand | The private cards available to one player. |
| Lead | The player who commits first in the round. |
| Reply | The player who commits second in the round. |
| Commitment | A selected card stored face down until reveal. |
| Reveal | The moment both committed cards become public together. |

## What this page is not

This page teaches rules and turn flow. It is not strategy advice, and it is not the formal implementation contract.

## Source notes for maintainers

The formal rule source is `games/high_card_duel/docs/RULES.md`.

The formal rules version checked is `high-card-duel-rules-v1`.

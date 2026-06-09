# Token Bazaar - How to Play

_Game ID: `token_bazaar`_  
_Formal rules source: `games/token_bazaar/docs/RULES.md`_  
_Formal rules version checked: `token-bazaar-rules-v1`_  
_Strategy guide: `games/token_bazaar/docs/COMPETENT-PLAYER.md`_

## At a glance

- Players collect public resources, exchange resources, and fulfill visible contracts.
- Contracts award points and then refill from the public queue.
- A pass is legal only when no collect, exchange, or fulfill action is available.
- The game ends after both players take eight turns or when the contract queue and visible market are empty.
- Higher score wins, with public tie-breakers if scores match.

## What you can see

Everything needed for play is public: supply, inventories, visible contracts, contract costs and points, scores, fulfilled lists, turn counts, active seat, legal actions, and terminal outcome.

Not applicable - this is a perfect-information game. All game state needed for play is public in the normal game view.

## Setup

Each player starts with one amber, one jade, and one iron. Scores start at 0. The public supply and visible contract market are set up from the fixed contract queue.

## On your turn

Choose one legal action from the interface. After your action resolves, public accounting effects are shown and the turn passes unless the game ended.

## Actions

### Collect

Take one Rust-supplied legal resource bundle from the public supply. Bundles that cannot be fully paid from the supply are not legal.

### Exchange

Pay two of one resource you own to take one different resource from the public supply.

### Fulfill

Pay the exact public cost of one visible contract you can afford. You gain that contract's points, the contract is added to your fulfilled list, and the market refills when the queue has more contracts.

### Pass

Pass only when Rust says no collect, exchange, or fulfill action is legal. Passing is not voluntary.

## Scoring and winning

Contracts award their printed points when fulfilled.

The game ends when both players have taken eight turns, or immediately when the last contract is fulfilled and all visible market slots are empty after refill.

Winner comparison is public and deterministic:

1. Higher score wins.
2. If tied, more fulfilled contracts wins.
3. If still tied, higher total remaining inventory wins.
4. If all comparisons tie, the game is a draw.

## Hidden information and reveal timing

Not applicable - this is a perfect-information game.

## Common terms

| Term | Meaning |
|---|---|
| Supply | The public pool of available resources. |
| Inventory | A player's public resources. |
| Contract | A visible scoring card with a cost and point value. |
| Market | The visible contract slots. |
| Fulfilled list | A public list of contracts a player has completed. |

## What this page is not

This page teaches rules and turn flow. It is not strategy advice, and it is not the formal implementation contract.

## Source notes for maintainers

The formal rule source is `games/token_bazaar/docs/RULES.md`.

The formal rules version checked is `token-bazaar-rules-v1`.

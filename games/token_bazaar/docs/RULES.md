# Token Bazaar Rules

Game ID: `token_bazaar`

Public display name: `Token Bazaar`

Implemented variant: `token_bazaar_standard`

Rules version: `token-bazaar-rules-v1`

Prepared by: `Codex`

Created: 2026-06-08

Last updated: 2026-06-08

## Rule authority

This document is the original Rulepath rules summary for the implemented
variant. Sources belong in `SOURCES.md`; this document states the Rulepath
implementation contract.

Stable rule IDs are requirements. They must remain stable after implementation
unless intentionally migrated with a migration note and corresponding updates in
`RULE-COVERAGE.md`, traces, tests, and docs.

## Metadata

| Field | Value |
|---|---|
| game id | `token_bazaar` |
| public display name | `Token Bazaar` |
| variant | `token_bazaar_standard` |
| rules version | `token-bazaar-rules-v1` |
| source note | `games/token_bazaar/docs/SOURCES.md` |
| coverage matrix | `games/token_bazaar/docs/RULE-COVERAGE.md` |
| mechanic inventory | `games/token_bazaar/docs/MECHANICS.md` |
| implementation admission | `games/token_bazaar/docs/GAME-IMPLEMENTATION-ADMISSION.md` |

## Purpose and scope

Token Bazaar is a two-seat, original Rulepath public-resource economy game. Rust
owns setup, legal actions, validation, accounting effects, market refill,
terminal checks, tie-breaks, replay behavior, and bot decisions. TypeScript may
present only Rust/WASM output.

The game proves public resource accounting, visible market state, deterministic
contract refill, fixed turn pressure, and Rust-owned action-tree metadata. It
does not implement hidden commitments, auctions, betting, card decks, or any
generic economy engine.

## Implemented variant

The only shipped variant is `token_bazaar_standard`.

| Field | Value |
|---|---|
| seats | `seat_0`, `seat_1` |
| starting seat | `seat_0` |
| turn cap | 8 turns per seat |
| resources | `amber`, `jade`, `iron` |
| public supply | 14 of each resource |
| initial inventory | 1 of each resource per seat |
| initial score | 0 per seat |
| visible market slots | `slot_0`, `slot_1`, `slot_2` |
| contract queue | deterministic ten-contract queue |
| randomness | none |

## Components and game-local vocabulary

Game nouns in this section belong to `games/token_bazaar` only. They do not
authorize `resource`, `market`, `token`, `contract`, `supply`, `card`, `deck`,
`hand`, `board`, `grid`, `auction`, `betting`, `pot`, or similar nouns in
`engine-core`.

| Rule ID | Term/component | Original Rulepath definition | Visibility | Notes |
|---|---|---|---|---|
| `TB-COMP-001` | seat | One of the two players, `seat_0` or `seat_1`. | public | `seat_0` starts. |
| `TB-COMP-002` | resource | One game-local token type: `amber`, `jade`, or `iron`. | public | Resource names are local vocabulary only. |
| `TB-COMP-003` | public supply | The shared count of each resource available for collection or exchange. | public | Supply starts at 14 of each resource. |
| `TB-COMP-004` | inventory | A seat's public resource counts. | public | Each seat starts with 1 of each resource. |
| `TB-COMP-005` | contract | A visible or queued scoring opportunity with an id, display label, exact cost, and point value. | public while visible; deterministic internal queue order | Queue order is fixed at setup. |
| `TB-COMP-006` | market slot | One of three visible positions, `slot_0`, `slot_1`, or `slot_2`, that may hold a contract. | public | Empty slots remain visible after queue exhaustion. |
| `TB-COMP-007` | fulfilled-contract list | A public ordered list of contract ids each seat has fulfilled. | public | Used for the second terminal tie-break. |

## Setup

| Rule ID | Setup rule | Deterministic/random? | Visibility | Notes |
|---|---|---|---|---|
| `TB-SETUP-001` | Create two seats, `seat_0` and `seat_1`. | deterministic | public | No other player counts ship in this variant. |
| `TB-SETUP-002` | Set the active seat to `seat_0`. | deterministic | public | Active seat alternates after every applied action. |
| `TB-SETUP-003` | Set public supply to 14 `amber`, 14 `jade`, and 14 `iron`. | deterministic | public | Supply may later decrease or increase through effects. |
| `TB-SETUP-004` | Give each seat 1 `amber`, 1 `jade`, and 1 `iron`; set each score to 0 and each fulfilled-contract list to empty. | deterministic | public | Inventories and scores are public from setup. |
| `TB-SETUP-005` | Build the standard ten-contract queue in the exact order listed in `TB-SETUP-006`. | deterministic | public contract identities; no randomness | Queue order is replay-stable. |
| `TB-SETUP-006` | Fill `slot_0`, `slot_1`, and `slot_2` with the first three queued contracts, then keep the remaining contracts in queue order. | deterministic | public visible slots; remaining queue count and order may be exposed by the game docs/tests but UI must follow Rust projection | Later tickets define projection detail. |

### Standard contract queue

| Queue order | Contract id | Display label | Cost | Points |
|---:|---|---|---|---:|
| 1 | `balanced-wares` | Balanced Wares | 1 `amber`, 1 `jade`, 1 `iron` | 3 |
| 2 | `amber-guild` | Amber Guild | 2 `amber`, 1 `jade` | 3 |
| 3 | `iron-guild` | Iron Guild | 2 `iron`, 1 `amber` | 3 |
| 4 | `jade-guild` | Jade Guild | 2 `jade`, 1 `iron` | 3 |
| 5 | `amber-focus` | Amber Focus | 3 `amber` | 4 |
| 6 | `jade-focus` | Jade Focus | 3 `jade` | 4 |
| 7 | `iron-focus` | Iron Focus | 3 `iron` | 4 |
| 8 | `sun-route` | Sun Route | 2 `amber`, 2 `jade` | 5 |
| 9 | `stone-route` | Stone Route | 2 `jade`, 2 `iron` | 5 |
| 10 | `crown-route` | Crown Route | 2 `iron`, 2 `amber` | 5 |

## Turn sequence

Each seat may take at most eight turns.

| Rule ID | Situation/phase | Active seat(s) | Required sequence | Advances when |
|---|---|---|---|---|
| `TB-TURN-001` | Nonterminal active turn. | Exactly one active seat. | Rust presents that seat's legal `collect`, `exchange`, `fulfill`, or forced `pass` actions. | The seat submits one valid action. |
| `TB-TURN-002` | After any applied nonterminal action. | none during transition | Emit deterministic accounting and status effects, then alternate the active seat. | The next seat's turn begins unless terminal was reached. |
| `TB-TURN-003` | A seat reaches its eighth turn. | normal active-seat alternation until both seats reach the cap | That seat may not exceed eight applied turns. | The game ends once both seats have taken eight turns. |
| `TB-TURN-004` | Terminal state. | none | Expose outcome and no normal gameplay actions. | No further gameplay action advances the game. |

## Legal actions

Rust must generate legal actions. TypeScript must not decide legality,
affordability, refill, winner, terminal outcome, or bot policy.

| Rule ID | Situation | Legal action(s) | Action shape | Rust-owned validation notes |
|---|---|---|---|---|
| `TB-ACT-001` | Nonterminal active turn with satisfiable supply bundle. | `collect/<bundle-id>` for every legal bundle. | action tree path, preferably `["collect", "<bundle-id>"]` | Supply must satisfy the entire bundle. |
| `TB-ACT-002` | Nonterminal active turn with exchangeable resources. | `exchange/<pay-resource>/<take-resource>`. | action tree path | Pay and take resources must differ; the player must have at least 2 of the paid resource; supply must have at least 1 of the taken resource. |
| `TB-ACT-003` | Nonterminal active turn with affordable visible contract. | `fulfill/<slot-id>`. | action tree path | Slot must be occupied and the active seat must have every required resource. |
| `TB-ACT-004` | No collect, exchange, or fulfill action is legal. | `pass`. | single action path | Forced pass is legal only in this no-other-action state. |
| `TB-ACT-005` | Terminal state. | none. | empty gameplay tree | Terminal states expose no normal gameplay actions. |

### Collect bundles

Collect takes resources from the public supply into the active seat's inventory.

| Rule ID | Bundle id | Gain | Legality |
|---|---|---|---|
| `TB-COLLECT-001` | `amber` | 2 `amber` | public supply has at least 2 `amber` |
| `TB-COLLECT-002` | `jade` | 2 `jade` | public supply has at least 2 `jade` |
| `TB-COLLECT-003` | `iron` | 2 `iron` | public supply has at least 2 `iron` |
| `TB-COLLECT-004` | `amber-jade` | 1 `amber`, 1 `jade` | public supply has at least 1 `amber` and 1 `jade` |
| `TB-COLLECT-005` | `jade-iron` | 1 `jade`, 1 `iron` | public supply has at least 1 `jade` and 1 `iron` |
| `TB-COLLECT-006` | `iron-amber` | 1 `iron`, 1 `amber` | public supply has at least 1 `iron` and 1 `amber` |

If a resource supply is exhausted, any bundle requiring that resource is illegal
until payments or exchanges return enough of that resource to supply.

### Exchange

Exchange converts two matching resources from the active seat's inventory into
one different resource from public supply.

| Rule ID | Exchange rule | Effect | Notes |
|---|---|---|---|
| `TB-EXCHANGE-001` | `pay-resource` and `take-resource` must be different. | none if invalid | Same-resource exchange is never legal. |
| `TB-EXCHANGE-002` | The active seat must have at least 2 of `pay-resource`. | Pay 2 `pay-resource` back to public supply. | Payment is exact and effect-visible. |
| `TB-EXCHANGE-003` | Public supply must have at least 1 of `take-resource`. | Take 1 `take-resource` from public supply. | Supply return and supply take are both replay-visible. |

Exchange is intentionally inefficient. It exists to prove conversion legality,
supply return, and bot valuation pressure, not to be a dominant strategy.

### Fulfill contract

Fulfill pays the exact cost of one occupied visible market slot and scores that
contract's points.

| Rule ID | Fulfill rule | Effect | Notes |
|---|---|---|---|
| `TB-FULFILL-001` | The target slot must be visible and occupied. | The slot's contract is selected. | Empty slots are not legal targets. |
| `TB-FULFILL-002` | The active seat must have all resources required by the contract cost. | Required resources are paid back to public supply. | Partial payment is never legal. |
| `TB-FULFILL-003` | A fulfilled contract awards its printed point value. | Active seat's score increases by that value. | Score change is public and effect-visible. |
| `TB-FULFILL-004` | The fulfilled contract id is appended to the active seat's fulfilled-contract list. | Fulfilled list changes publicly. | List length is the second terminal tie-break. |
| `TB-FULFILL-005` | The vacated slot refills immediately from the front of the queue if any queued contract remains. | New visible contract appears in the same slot. | If the queue is empty, the slot remains empty. |
| `TB-FULFILL-006` | Terminal conditions are checked after payment, scoring, list update, and refill. | The game may end immediately. | Last-contract exhaustion can end before the turn cap. |

## Forced actions and restrictions

| Rule ID | Situation | Forced action/restriction | Illegal/stale diagnostic expectation | Notes |
|---|---|---|---|---|
| `TB-RESTRICT-001` | A seat other than the active seat submits a gameplay action. | Reject without mutation. | Public-safe wrong-seat diagnostic may identify the active seat because it is public. | No hidden information exists in this game, but diagnostics still use normal viewer-safe conventions. |
| `TB-RESTRICT-002` | Submitted action references an illegal bundle, resource, slot, empty slot, unaffordable contract, or unavailable supply. | Reject without mutation. | Public-safe invalid-action diagnostic names the invalid class and public target. | Public targets are not secret. |
| `TB-RESTRICT-003` | A stale command is submitted after state has advanced. | Reject without mutation. | Public-safe stale-command diagnostic. | Replay/hash state must not change. |
| `TB-RESTRICT-004` | At least one collect, exchange, or fulfill action is legal. | `pass` is illegal. | Public-safe diagnostic states that pass is not forced. | Prevents voluntary pass. |

## Scoring and accounting

| Rule ID | Scoring/accounting rule | Timing | Tiebreaker/edge case | Notes |
|---|---|---|---|---|
| `TB-SCORE-001` | Scores start at 0 and increase only through fulfilled contracts. | setup and fulfill | Higher score is the first terminal tie-break. | Score never decreases. |
| `TB-SCORE-002` | Resource collection decreases public supply and increases active-seat inventory by the exact bundle. | collect | Bundle is illegal if supply cannot satisfy all resources. | All deltas are effect-visible. |
| `TB-SCORE-003` | Exchange returns exactly 2 of one resource to public supply and takes exactly 1 different resource from public supply. | exchange | Illegal if inventory or supply constraints fail. | All deltas are effect-visible. |
| `TB-SCORE-004` | Fulfill returns the exact contract cost to public supply and increases score by printed points. | fulfill | The fulfilled contract count is the second terminal tie-break. | Contract id is public. |
| `TB-SCORE-005` | Total remaining inventory is the sum of all `amber`, `jade`, and `iron` in a seat's inventory. | terminal only | Higher total inventory is the third terminal tie-break. | Resource type mix does not matter for this tie-break. |

## Terminal conditions

| Rule ID | Terminal condition | Outcome | Tie handling | Notes |
|---|---|---|---|---|
| `TB-END-001` | Both seats have taken exactly eight turns. | Determine winner by the terminal tie-break order. | `TB-END-003` | This is the normal turn-cap ending. |
| `TB-END-002` | The last contract is fulfilled and all visible market slots are empty after refill. | End immediately and determine winner by the terminal tie-break order. | `TB-END-003` | This can end before both seats reach eight turns. |
| `TB-END-003` | Terminal tie-break order. | Higher score wins; if tied, more fulfilled contracts wins; if still tied, higher total remaining inventory wins; if still tied, the game is a draw. | Draw if all three comparisons tie. | All tie-break facts are public and deterministic. |

Terminal public views also expose a Rust-owned outcome rationale. The rationale names the terminal trigger (`TB-END-001` or `TB-END-002`), the ordered `TB-END-003` ladder, the decisive cause (`score`, `fulfilled_contracts`, `inventory_total`, or `all_tied_draw`), and the final score/fulfilled-count/inventory-total standing for each seat.

## Visibility and private information

Token Bazaar is fully public, but it still uses the normal Rulepath viewer,
effect, action-tree, replay, DOM, storage, log, bot, and dev-inspector
boundaries.

| Rule ID | Information | Visible to whom | When visible | Surfaces requiring protection | Notes |
|---|---|---|---|---|---|
| `TB-VIS-001` | Supply, inventories, market slots, visible contract costs and points, scores, turn counts, fulfilled lists, active seat, and terminal outcome. | observer and both seat viewers | always after setup | public view, action tree, preview, diagnostics, effect log, DOM, logs, storage, replay export, bot explanation, dev inspector | Observer and seat viewers see the same game facts. |
| `TB-VIS-002` | Legal actions and Rust-owned action metadata for the active public turn. | observer and both seat viewers unless a later generic shell constraint limits controls to the actor | nonterminal active turn | action tree, UI controls, dev inspector, replay export | TypeScript may render but not compute legality. |
| `TB-VIS-003` | Hidden choices, private hands, pending commitments, hidden draws, deck order, and hidden bot candidates. | no one, because they do not exist in this game | not applicable | all public and private projection surfaces | Gate 9.1 handles simultaneous hidden commitments separately. |

## Replay and randomness notes

| Rule ID | Randomness/replay rule | Seed/log/hash requirement | Visibility | Notes |
|---|---|---|---|---|
| `TB-RNG-001` | Token Bazaar uses no random setup, refill, contract order, action resolution, scoring, or tie-break. | Same variant plus same command sequence must reproduce state, effects, action-tree hashes, public-view hashes, and outcome. | public | Do not touch `engine-core::DeterministicRng` for this game. |
| `TB-RNG-002` | Contract queue order and refill are deterministic. | Replay logs must preserve enough public command and effect data to reproduce each refill. | public | No browser state may influence refill. |
| `TB-RNG-003` | Serialization order must remain stable. | Golden traces and replay checks must fail on unintended ordering changes. | public | Later tickets define trace fixtures. |

## Bot-relevant non-authoritative strategy notes

These notes are not rule authority. They guide `COMPETENT-PLAYER.md`,
`BOT-STRATEGY-EVIDENCE-PACK.md`, and `AI.md`. Strategy claims must be checked
against rule IDs above.

| Situation | Practical note | Rule IDs checked | Hidden-info limit |
|---|---|---|---|
| A visible contract is affordable. | Prefer fulfilling high-value affordable contracts, with stable tie-breaks. | `TB-ACT-003`, `TB-FULFILL-001` through `TB-FULFILL-006` | All state is public. |
| No visible contract is affordable. | Evaluate deficits against visible contracts and collect toward a plausible target. | `TB-COLLECT-001` through `TB-COLLECT-006`, `TB-SCORE-002` | All state is public. |
| Exchange can reduce a valuable deficit. | Exchange only when it improves the path to a higher-value contract. | `TB-EXCHANGE-001` through `TB-EXCHANGE-003` | All state is public. |
| No useful preference exists. | Fall back to the first legal collect action in stable bundle order, or forced pass when no other action is legal. | `TB-ACT-001`, `TB-ACT-004` | All state is public. |

## Known ambiguities and chosen resolutions

| Rule ID | Ambiguity | Chosen resolution | Source/rationale | Tests/traces required | Notes |
|---|---|---|---|---|---|
| `TB-AMB-001` | Whether queued contracts beyond the visible three slots are shown in browser projection. | Rust may expose queue count and/or upcoming public queue data only through its own projection; TypeScript must not infer or own it. | Gate 9 spec fixes deterministic queue order but requires visible market state as the browser proof. | public-view, serialization, replay, and browser smoke tests | The rules do not require hidden queue information. |
| `TB-AMB-002` | Whether `pass` may be voluntary. | `pass` is legal only when no collect, exchange, or fulfill action is legal. | Gate 9 forced-pass rule. | forced-pass legality test if reachable | Prevents players from skipping resource pressure. |
| `TB-AMB-003` | Whether terminal tie-breaks use resource type values. | Remaining inventory tie-break counts total resources only; resource type mix has no value ordering. | Gate 9 winner and tie-break rule. | terminal tie-break tests and golden traces | Keeps terminal scoring deterministic and small. |

## Rulepath deviations from common variants

Token Bazaar is an original Rulepath game, not an implementation of a public
domain or commercial rule set.

| Rule ID | Common variant behavior | Rulepath behavior | Why | Public note needed? |
|---|---|---|---|---:|
| `TB-VAR-001` | Resource-market games often include random decks, hidden hands, auctions, negotiation, or variable player counts. | `token_bazaar_standard` has exactly two seats, fully public state, deterministic contract order, no auctions, no hidden choices, and no random setup. | Gate 9 proves public accounting and browser readability without adding hidden-state or generic economy primitives. | yes |

## Explicit out-of-scope variants

| Rule ID | Out-of-scope variant/rule | Reason out of scope | Future review trigger |
|---|---|---|---|
| `TB-VAR-002` | `secret_draft` simultaneous commitment/reveal. | Deferred to the successor Gate 9.1 commitment/reveal gate. | Gate 9 completion and accepted Gate 9.1 spec. |
| `TB-VAR-003` | Auctions, betting, trading, negotiation, random contract setup, alternate contract queues, more than two seats, and generic economy primitives. | Outside Gate 9 proof scope and may trigger ADR or mechanic-atlas promotion review. | Accepted future spec or ADR. |

## Rule coverage link

The implementation and evidence mapping lives in `RULE-COVERAGE.md`.

Every stable rule ID in this document must appear in `RULE-COVERAGE.md`. Silent
gaps are not allowed.

## Rule-ID migration notes

| Old rule ID | New rule ID(s) | Reason | Coverage/traces updated? | Date |
|---|---|---|---:|---|
| none | not applicable | Initial Token Bazaar rule set. | not applicable | 2026-06-08 |

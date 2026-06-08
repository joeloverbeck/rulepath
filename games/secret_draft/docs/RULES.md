# Veiled Draft Rules

Game ID: `secret_draft`

Public display name: `Veiled Draft`

Implemented variant: `secret_draft_standard`

Rules version: `secret-draft-rules-v1`

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
| game id | `secret_draft` |
| public display name | `Veiled Draft` |
| variant | `secret_draft_standard` |
| rules version | `secret-draft-rules-v1` |
| source note | `games/secret_draft/docs/SOURCES.md` |
| coverage matrix | `games/secret_draft/docs/RULE-COVERAGE.md` |
| mechanic inventory | `games/secret_draft/docs/MECHANICS.md` |
| implementation admission | `games/secret_draft/docs/GAME-IMPLEMENTATION-ADMISSION.md` |

## Purpose and scope

Veiled Draft is a two-seat, original Rulepath drafting game. Each round, both
seats secretly choose from a shared visible set of draft items. The game proves
simultaneous hidden commitment, synchronized reveal, deterministic conflict
resolution, visible pool removal, Rust-owned legality, and viewer-safe pending
state.

Rust owns setup, legal actions, validation, hidden commitment storage, reveal
timing, conflict fallback, scoring, terminal checks, tie-breaks, semantic
effects, replay behavior, visibility projection, and bot decisions. TypeScript
may present only Rust/WASM output.

The game does not implement hosted multiplayer, cryptographic commitments,
betting, auctions, reaction windows, generalized drafting primitives, or any
shared engine vocabulary for this game's local nouns.

## Implemented variant

The only shipped variant is `secret_draft_standard`.

| Field | Value |
|---|---|
| seats | `seat_0`, `seat_1` |
| starting priority seat | `seat_0` |
| rounds | 6 |
| visible draft items | 12 |
| threads | `ember`, `tide`, `grove` |
| values | 1, 2, 3, 4 in each thread |
| action shape | simultaneous hidden commitment |
| scoring | base values, complete-thread sets, high-thread bonuses, conflict-discipline bonus |
| terminal | after the sixth reveal batch |
| randomness | none |

## Components and game-local vocabulary

Game nouns in this section belong to `games/secret_draft` only. They do not
authorize `draft`, `pick`, `commit`, `reveal`, `pool`, `tile`, `card`, `deck`,
`hand`, `resource`, `bid`, `reaction`, or similar nouns in `engine-core`.

| Rule ID | Term/component | Original Rulepath definition | Visibility | Notes |
|---|---|---|---|---|
| `SD-COMP-001` | seat | One of the two players, `seat_0` or `seat_1`. | public | Both seats may commit during the same round. |
| `SD-COMP-002` | draft item | One game-local item with an `item_id`, thread, value, and label. | public while in the visible pool or after reveal | Item identities are not hidden until a seat commits one. |
| `SD-COMP-003` | thread | One of `ember`, `tide`, or `grove`. | public | Used for set scoring and high-thread bonuses. |
| `SD-COMP-004` | value | A draft item's numeric value, 1 through 4. | public | Used for base score and highest-item tie-break. |
| `SD-COMP-005` | visible pool | The ordered public list of draft items still available to choose. | public | Items leave the pool only during reveal resolution. |
| `SD-COMP-006` | hidden commitment slot | A per-seat internal slot holding that seat's submitted item choice until the reveal batch. | private to Rust authority before reveal | Public and seat-facing payloads expose only pending booleans. |
| `SD-COMP-007` | drafted collection | A public ordered list of draft items awarded to a seat. | public | Used for scoring and tie-breaks. |
| `SD-COMP-008` | priority seat | The seat that wins a contested item for the current round. | public | Alternates by round, starting with `seat_0`. |
| `SD-COMP-009` | fallback award | The lowest stable-order remaining draft item awarded to the non-priority seat after a contested choice. | public after reveal | The fallback item is removed with the contested item. |
| `SD-COMP-010` | reveal batch | The single ordered public event emitted after both seats have committed. | public after both commitments exist | Contains both choices and awards together, never early. |
| `SD-COMP-011` | public score summary | Each seat's score facts and terminal tie-break facts. | public after each reveal | Scores are computed by Rust. |

### Standard draft item list

Static data may carry the item IDs, labels, threads, values, and stable order in
this table. Static data must not carry scoring formulas, conflict formulas,
selectors, triggers, conditions, or procedural behavior.

| Stable order | Item ID | Label | Thread | Value |
|---:|---|---|---|---:|
| 1 | `ember_1` | Ember One | `ember` | 1 |
| 2 | `ember_2` | Ember Two | `ember` | 2 |
| 3 | `ember_3` | Ember Three | `ember` | 3 |
| 4 | `ember_4` | Ember Four | `ember` | 4 |
| 5 | `tide_1` | Tide One | `tide` | 1 |
| 6 | `tide_2` | Tide Two | `tide` | 2 |
| 7 | `tide_3` | Tide Three | `tide` | 3 |
| 8 | `tide_4` | Tide Four | `tide` | 4 |
| 9 | `grove_1` | Grove One | `grove` | 1 |
| 10 | `grove_2` | Grove Two | `grove` | 2 |
| 11 | `grove_3` | Grove Three | `grove` | 3 |
| 12 | `grove_4` | Grove Four | `grove` | 4 |

## Setup

| Rule ID | Setup rule | Deterministic/random? | Visibility | Notes |
|---|---|---|---|---|
| `SD-SETUP-001` | Create exactly two seats, `seat_0` and `seat_1`. | deterministic | public | No other player counts ship in this variant. |
| `SD-SETUP-002` | Set round number to 1 and priority seat to `seat_0`. | deterministic | public | Priority alternates after each reveal. |
| `SD-SETUP-003` | Fill the visible pool with all twelve standard draft items in stable order. | deterministic | public | No shuffle or random deal is used. |
| `SD-SETUP-004` | Set both drafted collections to empty, both hidden commitment slots to empty, both fallback-award counts to 0, both priority-conflict-win counts to 0, and both scores to 0. | deterministic | mixed | Commitment slots are internal only. |
| `SD-SETUP-005` | Set terminal outcome to none and initialize the freshness token. | deterministic | public except internal token representation | Freshness prevents stale commands. |

## Round sequence

Each game has six rounds. A round has a commitment phase followed by a reveal
resolution after both seats have submitted valid commitments.

| Rule ID | Situation/phase | Active seat(s) | Required sequence | Advances when |
|---|---|---|---|---|
| `SD-TURN-001` | Start of a nonterminal round. | Both uncommitted seats. | Rust exposes legal commitment actions for each uncommitted seat using the current visible pool. | A valid seat commitment is submitted. |
| `SD-TURN-002` | One seat has committed and the other has not. | Only the uncommitted seat. | Public and seat views show pending-seat status without the committed item ID. | The second valid commitment is submitted. |
| `SD-TURN-003` | Both seats have committed. | none during resolution | Rust emits one synchronized reveal batch, resolves awards, removes drafted items from the visible pool, updates scoring facts, and clears commitment slots. | Resolution completes. |
| `SD-TURN-004` | Rounds 1 through 5 after reveal. | Both seats for the next round. | Advance the round number by 1 and alternate the priority seat. | Next round begins. |
| `SD-TURN-005` | Round 6 after reveal. | none | Compute terminal outcome by score and tie-break ladder. | Game enters terminal state. |
| `SD-TURN-006` | Terminal state. | none | Expose outcome and no normal gameplay actions. | No further gameplay action advances the game. |

## Legal actions

Rust must generate legal actions. TypeScript must not decide legality,
availability, pending state, reveal timing, scoring, terminal outcome, tie
breaks, hidden-info filtering, or bot policy.

| Rule ID | Situation | Legal action(s) | Action shape | Rust-owned validation notes |
|---|---|---|---|---|
| `SD-ACT-001` | Nonterminal round where the submitting seat has not committed. | `commit/<item-id>` for every item currently in the visible pool. | simultaneous action tree path | Target item must still be visible, and actor must be `seat_0` or `seat_1`. |
| `SD-ACT-002` | Nonterminal round where the submitting seat has already committed. | none for that seat until reveal resolution completes. | empty actor tree with pending status | Diagnostics must not name the committed item. |
| `SD-ACT-003` | Nonterminal round where the other seat has committed. | `commit/<item-id>` for every item currently in the visible pool. | simultaneous action tree path | The other seat's hidden choice does not remove choices from the public pool before reveal. |
| `SD-ACT-004` | Terminal state. | none. | empty gameplay tree | Terminal states expose no normal gameplay actions. |

## Forced actions and restrictions

| Rule ID | Situation | Forced action/restriction | Illegal/stale diagnostic expectation | Notes |
|---|---|---|---|---|
| `SD-RESTRICT-001` | Unknown or non-seat actor submits a gameplay action. | Reject without mutation. | Viewer-safe wrong-actor diagnostic. | Only `seat_0` and `seat_1` can commit. |
| `SD-RESTRICT-002` | A seat submits a second commitment in the same round. | Reject without mutation. | Viewer-safe already-committed diagnostic that never names the prior item. | Pending flag is public; chosen item is not. |
| `SD-RESTRICT-003` | A commitment references an item that is not in the visible pool. | Reject without mutation. | Viewer-safe unavailable-item diagnostic may name the requested public item id. | Removed items and unknown ids are invalid. |
| `SD-RESTRICT-004` | A stale command is submitted after state has advanced. | Reject without mutation. | Viewer-safe stale-command diagnostic. | Replay/hash state must not change. |
| `SD-RESTRICT-005` | A gameplay action is submitted in terminal state. | Reject without mutation. | Viewer-safe terminal-state diagnostic. | Outcome is final. |

## Reveal resolution and drafting with removal

| Rule ID | Resolution rule | Effect | Notes |
|---|---|---|---|
| `SD-REVEAL-001` | The first valid commitment in a round stores only an internal hidden item choice and emits pending-only public effects. | The committing seat becomes publicly pending/committed. | No public or seat-facing payload may include that item id before the reveal batch. |
| `SD-REVEAL-002` | The second valid commitment triggers one synchronized reveal batch. | Both committed item ids become public at the same reveal point. | Reveal order is stable: `seat_0` then `seat_1` for displayed choices, followed by award effects. |
| `SD-REVEAL-003` | If seats chose different available items, each seat is awarded its chosen item. | Both awarded items are removed from the visible pool. | Awards are public and effect-visible. |
| `SD-REVEAL-004` | If both seats chose the same item, the priority seat is awarded the contested item. | The contested item is removed from the visible pool. | The priority seat's priority-conflict-win count increases by 1. |
| `SD-REVEAL-005` | In a contested reveal, the non-priority seat is awarded the lowest stable-order item still remaining after the contested item is removed. | The fallback item is removed from the visible pool, and the non-priority seat's fallback-award count increases by 1. | The fallback is deterministic and public after reveal. |
| `SD-REVEAL-006` | After awards, both commitment slots are cleared. | The next round has no carried hidden choices. | Revealed history keeps the public reveal and award facts. |

## Scoring and accounting

Scores are recomputed by Rust from the public drafted collections after each
reveal batch and at terminal. Static data may list item values only; scoring,
conflict fallback, terminal detection, and tie-breaks live in typed Rust.

| Rule ID | Scoring/accounting rule | Timing | Tiebreaker/edge case | Notes |
|---|---|---|---|---|
| `SD-SCORE-001` | Base score is the sum of values in a seat's drafted collection. | after each reveal and terminal | Higher score is the first terminal comparison. | Values are public. |
| `SD-SCORE-002` | Each complete set of one `ember`, one `tide`, and one `grove` scores 3 bonus points. | after each reveal and terminal | Set count is the second terminal comparison. | Each drafted item can contribute to at most one complete set. |
| `SD-SCORE-003` | A seat scores one 2-point high-thread bonus for each thread where that seat has at least three drafted items of that thread. | after each reveal and terminal | Multiple threads can each score once. | A fourth item in the same thread does not add another high-thread bonus. |
| `SD-SCORE-004` | A seat scores a 1-point conflict-discipline bonus at terminal if it received no fallback awards. | terminal only | Both seats can earn this bonus. | This rewards avoiding or winning contested outcomes. |
| `SD-SCORE-005` | Visible pool removal is exact: two items leave the pool each reveal, either two distinct chosen items or one contested item plus one fallback item. | each reveal | Pool count reaches 0 after six rounds. | No duplicate award is legal. |

## Terminal conditions

| Rule ID | Terminal condition | Outcome | Tie handling | Notes |
|---|---|---|---|---|
| `SD-END-001` | The sixth reveal batch has resolved. | Determine winner by the terminal tie-break order. | `SD-END-002` | The game always ends after six rounds. |
| `SD-END-002` | Terminal tie-break order. | Higher total score wins; if tied, more complete ember/tide/grove sets wins; if tied, higher single drafted item value wins; if tied, more distinct represented threads wins; if tied, fewer priority-won contested items wins; if still tied, the game is a draw. | Draw if all comparisons tie. | All tie-break facts are public and deterministic. |

## Visibility and private information

Public/browser payloads must not reveal hidden information through public views,
seat views, action trees, previews, diagnostics, effect logs, DOM attributes,
test IDs, logs, local storage, replay exports, bot explanations, candidate
rankings, or dev inspectors.

| Rule ID | Information | Visible to whom | When visible | Surfaces requiring protection | Notes |
|---|---|---|---|---|---|
| `SD-VIS-001` | Visible pool, round number, priority seat, drafted collections, score summary, pending booleans, terminal outcome. | observer and both seat viewers | after setup and after each state projection | public view, seat view, action tree, preview, diagnostics, effect log, DOM, logs, storage, replay export, bot explanation, dev inspector | These are the safe public facts. |
| `SD-VIS-002` | A seat's committed item id. | no public or seat-facing viewer before reveal; all viewers at reveal | only inside the synchronized reveal batch and later revealed history | public view, seat view, action tree, preview, diagnostics, effect log, DOM, logs, storage, replay export, bot explanation, candidate ranking, dev inspector | Even the committing seat's browser payload must not contain the item id before reveal. |
| `SD-VIS-003` | Whether each seat has committed. | observer and both seat viewers | immediately after each commitment | public view, seat view, effect log, DOM, replay export | Pending booleans are safe and required. |
| `SD-VIS-004` | Legal commitment choices for an uncommitted seat. | that seat and any public renderer Rust permits for controls | nonterminal round before that seat commits | action tree and controls | Legal choices are based on the visible pool only. |
| `SD-VIS-005` | Bot rationale and candidate rankings. | public only if projected by Rust as viewer-safe text/data | after bot decision or diagnostics | bot explanation, dev inspector, logs, replay export | Rationale may cite visible pool, public scores, and own legal choices, never hidden opponent commitment. |

## Replay and randomness notes

| Rule ID | Randomness/replay rule | Seed/log/hash requirement | Visibility | Notes |
|---|---|---|---|---|
| `SD-RNG-001` | Veiled Draft uses no random setup, draft order, reveal resolution, scoring, or tie-break. | Same variant plus same command sequence must reproduce state, effects, action-tree hashes, view hashes, and outcome. | mixed because internal traces include hidden choices before reveal | Do not touch `engine-core::DeterministicRng` for this game. |
| `SD-RNG-002` | Public replay export is viewer-scoped. | Before reveal, export timelines may show pending seats but not committed item ids. | public export is redacted | Internal test traces may retain full hidden choices under native test authority. |
| `SD-RNG-003` | Serialization order must remain stable. | Golden traces and replay checks must fail on unintended ordering changes. | public after reveal; redacted before reveal | Stable item order drives fallback and deterministic hashes. |

## Bot-relevant non-authoritative strategy notes

These notes are not rule authority. They guide `COMPETENT-PLAYER.md`,
`BOT-STRATEGY-EVIDENCE-PACK.md`, and `AI.md`. Strategy claims must be checked
against rule IDs above.

| Situation | Practical note | Rule IDs checked | Hidden-info limit |
|---|---|---|---|
| A high-value item is visible. | Prefer higher value, with stable tie-breaks that consider thread balance. | `SD-ACT-001`, `SD-SCORE-001`, `SD-SCORE-002`, `SD-SCORE-003` | May not inspect an opponent's hidden commitment. |
| A thread set can be completed. | Prefer a legal item that completes a set when value loss is acceptable. | `SD-SCORE-002` | Only drafted public collections and visible pool may be used. |
| Conflict risk is public only as a possibility. | A bot may account for public priority and visible alternatives but may not sample or peek at hidden choices. | `SD-REVEAL-004`, `SD-REVEAL-005`, `SD-VIS-002` | No hidden-state sampling. |
| No useful preference exists. | Fall back to the first legal item in stable visible-pool order. | `SD-ACT-001`, `SD-RNG-003` | Stable and deterministic. |

## Known ambiguities and chosen resolutions

| Rule ID | Ambiguity | Chosen resolution | Source/rationale | Tests/traces required | Notes |
|---|---|---|---|---|---|
| `SD-AMB-001` | Whether the committing seat may see its own committed item before reveal. | No browser-facing or seat-view payload contains the item id after commitment and before reveal, including for the committing seat. | Gate 9.1 no-leak proof scope. | visibility/no-leak tests, public export tests, browser no-leak smoke | Rust internal state remains authoritative. |
| `SD-AMB-002` | Whether a hidden commitment removes an item from the visible pool for the other seat's legal choices. | No. The item remains publicly visible and legally choosable until reveal; conflict is resolved deterministically if both seats chose it. | Simultaneous commitment model. | legal-action tests, contested-pick trace | Prevents early leakage through removed options. |
| `SD-AMB-003` | What fallback item is awarded after a contested reveal. | Award the lowest stable-order item remaining after removing the contested item. | Deterministic fallback design. | contested-pick fallback tests and traces | No randomness or browser choice. |
| `SD-AMB-004` | Whether priority-won contested items are good or bad in the terminal ladder. | Fewer priority-won contested items wins after the earlier tie-breaks. | Conflict-discipline design. | terminal tie-break tests and traces | Keeps conflict history public and deterministic. |

## Rulepath deviations from common variants

Veiled Draft is an original Rulepath game, not an implementation of a public
domain or commercial rule set.

| Rule ID | Common variant behavior | Rulepath behavior | Why | Public note needed? |
|---|---|---|---|---:|
| `SD-VAR-001` | Drafting games may use hands, decks, tableaus, simultaneous selection with retained private knowledge, random deals, or more than two players. | `secret_draft_standard` has exactly two seats, a fully visible fixed pool, no random setup, no retained private hand, hidden commitments only until reveal, and deterministic fallback. | Gate 9.1 proves no-leak pending state and synchronized reveal without adding a generic drafting engine. | yes |

## Explicit out-of-scope variants

| Rule ID | Out-of-scope variant/rule | Reason out of scope | Future review trigger |
|---|---|---|---|
| `SD-VAR-002` | More than two seats. | Adds reveal, UI, and bot complexity without improving the Gate 9.1 proof. | A later roadmap gate needs multi-seat simultaneous choices. |
| `SD-VAR-003` | Randomized draft pool order or hidden item deal. | The standard variant is fixed to keep replay/hash and no-leak evidence small. | A later hidden-info gate needs seeded setup variation. |
| `SD-VAR-004` | Cryptographic commitment scheme. | Gate 9.1 proves local Rust/WASM authority and viewer-safe redaction, not adversarial network secrecy. | Hosted multiplayer or server authority is accepted into scope. |
| `SD-VAR-005` | Generic drafting, commitment, reveal, or pool primitives. | This is the first focused official local use and does not justify promotion. | A second official use creates atlas pressure. |

## Rule coverage link

The implementation and evidence mapping will live in `RULE-COVERAGE.md` after
the rule-coverage ticket lands.

Every rule ID in this document must appear in `RULE-COVERAGE.md`. Silent gaps
are not allowed.

## Rule-ID migration notes

| Old rule ID | New rule ID(s) | Reason | Coverage/traces updated? | Date |
|---|---|---|---:|---|
| none | not applicable | no migrations yet | not applicable | 2026-06-08 |

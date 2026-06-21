# Vow Tide Mechanics Inventory

Game ID: `vow_tide`

Roadmap stage/gate: Gate 17 variable-seat exact-bid trick-taking proof

Rules version: `vow-tide-rules-v1`

Last updated: 2026-06-21

## Purpose

This inventory records Vow Tide's game-local mechanic shapes and primitive
pressure posture. It is evidence for
[../../../docs/MECHANIC-ATLAS.md](../../../docs/MECHANIC-ATLAS.md), not
permission to generalize.

Vow Tide is a 3-7 seat hidden-hand exact-bid trick-taking game. Rust owns setup,
schedule, deal, bidding, dealer hook filtering, follow-suit legality, trump
comparison, trick capture, scoring, terminal standings, visibility, effects,
replay, bots, and benchmark evidence. TypeScript presents Rust/WASM output only.

## Mechanic Inventory

| Category | Game-local description | Evidence | Current status | Notes |
|---|---|---|---|---|
| N-seat model | 3, 4, 5, 6, or 7 independent seats; observer plus seat-private viewers. | [RULES.md](RULES.md), `setup.rs`, `visibility.rs` | `local-only` | First official 7-seat public browser game; labels are `Tide 1` through `Tide 7`. |
| turn-order policy | Dealer rotates each hand; bidding starts left of dealer and ends with dealer; first leader is left of dealer; trick winner leads next. | [RULES.md](RULES.md), `state.rs`, `rules.rs` | `repeated-shape candidate` | Winner-leads repeats earlier trick games, but dealer/schedule/bid pressure stays local. |
| team/partnership/coalition | No teams, partnerships, alliances, or asymmetric roles. | [RULES.md](RULES.md) | `rejected/deferred with rationale` | Later gates may own partnership pressure. |
| topology/spatial model | No board topology; public layout is seat rail, trump indicator, current trick, bids, scores, and standings. | [UI.md](UI.md) | `local-only` | No graph, grid, path, region, or movement primitive. |
| component/zone model | 52 game-local cards, private hands, public trump indicator, hidden stock, current trick, captured tricks, bids, scores, and hand history. | `cards.rs`, `state.rs`, [RULE-COVERAGE.md](RULE-COVERAGE.md) | `repeated-shape candidate` | Hidden hand/card pressure remains game-local except for the promoted pure trick helper. |
| hidden-hand/deck/wall model | Own hand is seat-private; other hands and stock identity/order never project to unauthorized viewers. | `visibility.rs`, `replay_support.rs`, e2e no-leak smoke | `repeated-shape candidate` | Pairwise no-leak spans every supported seat count and stock canaries. |
| action shape | Bidding uses flat `bid/<n>` leaves; play uses flat `play/<card_id>` leaves. | `actions.rs`, WASM action tree | `local-only` | UI maps Rust leaves directly; no TypeScript legality. |
| turn/phase model | Bidding, playing trick, hand scoring/advance, terminal. | `state.rs`, [RULES.md](RULES.md) | `local-only` | Hand schedule descends to one card, then ascends. |
| randomness/chance | Seeded setup shuffle/deal per hand only. No runtime chance after each hand is dealt. | `setup.rs`, golden traces | `local-only` | Browser time/randomness is not an input. |
| visibility/hidden information | Public facts are projected; own hand is owner-only; stock and other hands stay hidden across views, effects, exports, DOM, logs, and bots. | `visibility.rs`, `wasm-api`, e2e smoke | `repeated-shape candidate` | Hotseat handoff must replace the private subtree when the active seat changes. |
| resource/accounting | Public bids, trick counts, cumulative scores, exact-bid additions, final ranks. | `scoring.rs`, [RULES.md](RULES.md) | `local-only` | No spendable resource, side pot, debt, refund, or negative score. |
| shared accounting/side-pot/split allocation | Not applicable. | [RULES.md](RULES.md) | `rejected/deferred with rationale` | River Ledger owns shared-pot pressure; Vow Tide has independent cumulative scores. |
| movement/capture/placement | Played cards enter the public trick; winner captures the trick into history. | `rules.rs`, `state.rs` | `local-only` | No board movement or conversion. |
| pattern/line/directional scanning | Not applicable. | [RULES.md](RULES.md) | `rejected/deferred with rationale` | No alignment, rays, adjacency, or spatial pattern. |
| commitment/reveal | Public sequential bids are contracts, not hidden commitments. | `actions.rs`, `rules.rs` | `local-only` | First official numeric contract/bid shape. |
| reaction/window/pending response | Not applicable. | [RULES.md](RULES.md) | `rejected/deferred with rationale` | No interrupts, responders, replacement, or cancellation window. |
| scoring/outcome | Highest cumulative score wins after fixed schedule; tied top scores are co-winners. | `scoring.rs`, [RULES.md](RULES.md), [UI.md](UI.md) | `local-only` | No extra tie-break hands. |
| evaluator/showdown/ranking | Not a showdown evaluator; terminal standings rank public cumulative scores. | `scoring.rs` | `local-only` | Competition ranking is Rust-projected. |
| semantic effect shape | Bid accepted, dealer hook, bidding complete, card played, trick captured, hand scored, hand advanced, match completed. | `effects.rs`, WASM bridge | `local-only` | Effects are viewer-safe public facts. |
| UI interaction pattern | Seat selector, public rail, private owner hand, bid buttons, legal card buttons, replay import/export, shared outcome panel. | `VowTideBoard.tsx`, [UI.md](UI.md) | `local-only` | Seven-seat e2e cycles viewer selector through every seat. |
| bot policy pattern | Level 0 random legal and bounded Level 1 own-hand/public-fact heuristic. | `bots.rs`, [AI.md](AI.md) | `local-only` | No hidden-world sampling, search, ML, RL, or LLM policy. |
| benchmark/performance pressure | Setup/deal, legal trees, validate/apply, trick/scoring, projections, effect filtering, replay/export/import, bots, full matches for 3-7 seats. | [BENCHMARKS.md](BENCHMARKS.md) | `local-only` | Native full-match floors are seat-keyed. |

## Repeated-Shape Comparison

| Mechanic shape | Already appears in | Same shape? | Similarities | Differences | Required next step |
|---|---|---:|---|---|---|
| follow-suit legality | `plain_tricks`, `briar_circuit` | similar | Led suit restricts followers when able. | Vow adds trump, variable seats, contracts, and schedule pressure. | Reuse `game-stdlib::trick_taking::follow_suit_indices`. |
| trick comparator | `plain_tricks`, `briar_circuit` | similar | Highest eligible card wins a trick. | Vow has optional trump; Briar passes `None`; Plain Tricks has no trump. | Reuse `game-stdlib::trick_taking::winning_play_index`. |
| trick winner leads next | `plain_tricks`, `briar_circuit` | similar | Winner becomes next leader. | Vow embeds it in variable-seat scheduled hands and exact-bid scoring. | Keep local; no stdlib policy. |
| private hand no-leak | `high_card_duel`, `poker_lite`, `plain_tricks`, `river_ledger`, `briar_circuit` | similar | Owner-private components and public projections. | Vow spans 3-7 seats and hidden stock. | Keep game-local projection/e2e proof. |
| numeric contract vs result | none official before Vow Tide | first use | Public numeric commitment affects later scoring. | Dealer hook and exact-bid scoring are new. | Record as local-only; compare at next close use. |

## Primitive Pressure Decision

Gate 17 cleared the third-use trick-taking hard gate through two narrow
`game-stdlib::trick_taking` primitives:

- `follow_suit_indices` for the pure led-suit filtering subset;
- `winning_play_index` for the pure led-suit/trump comparison subset.

Vow-specific seat count, schedule, dealer hook, bid range, command validation,
state mutation, trick advancement, scoring, visibility, effects, bots, replay,
and UI remain local. No `engine-core` noun is introduced.

## Effects, UI, Bot, Visibility, And Benchmark Notes

| Area | Requirement | Rule IDs | Notes |
|---|---|---|---|
| semantic effects | Bid, hook, play, trick, score, hand advance, terminal feedback. | `VT-BID-*`, `VT-HOOK-001`, `VT-TRICK-WIN-001`, `VT-SCORE-001` | Effects carry public cause facts only. |
| UI interaction pattern | Board-native Rust legal leaves for bids and cards. | `VT-BOUNDARY-001`, `VT-BID-*`, `VT-FOLLOW-001` | The generic action panel is not rendered for Vow Tide. |
| Rust-generated previews | Action metadata includes safe bid/play facts; no hidden alternatives. | `VT-VIEW-001` | No TypeScript-computed legality. |
| bot policy pattern | L0/L1 use legal tree, own hand, and public facts only. | `VT-BOT-001` | L1 is bounded baseline, not a competent-player claim. |
| visibility/no-leak | Observer, seat viewers, hotseat, replay, DOM/storage/logs/export. | `VT-VIEW-001`, `VT-REPLAY-001` | Browser and WASM tests cover hidden hands/stock. |
| benchmark pressure | Every supported seat count gets native lanes and full-match floor. | `VT-REPLAY-001`, `VT-OUTCOME-001` | No benchmark shortcut may weaken legality or redaction. |

## Required Repo Atlas Update

| Update target | Required? | Reason | Owner |
|---|---:|---|---|
| `docs/MECHANIC-ATLAS.md` | yes | Gate 17 records the promoted trick helper and numeric-bid first-use pressure. | GAT17VOWTIDOHHEL-022 |
| `PRIMITIVE-PRESSURE-LEDGER.md` | yes | Vow's game-local ledger records helper reuse and local-only bid pressure. | completed by Gate 17 tickets |
| ADR | no | No kernel, DSL, YAML, generic bot, or schema exception. | not applicable |

## Review Checklist

- `engine-core` remains noun-free.
- Static data remains typed metadata/fixtures/reports only.
- TypeScript presents Rust/WASM payloads only.
- Other hands, hidden stock, deck order, and bot private facts stay out of unauthorized surfaces.
- Level 2/search bots are not admitted.
- Promoted trick helper use is narrow and back-port evidence stays green.

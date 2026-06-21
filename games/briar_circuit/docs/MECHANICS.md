# Briar Circuit Mechanics Inventory

Game ID: `briar_circuit`

Roadmap stage/gate: Gate 16 fixed-four-seat trick-taking proof

Rules version: `briar-circuit-rules-v1`

Last updated: 2026-06-21

## Purpose

This inventory records Briar Circuit's game-local mechanic shapes and primitive
pressure posture. It is evidence for
[../../../docs/MECHANIC-ATLAS.md](../../../docs/MECHANIC-ATLAS.md), not
permission to generalize.

Briar Circuit is a fixed-four-seat, hidden-hand, penalty trick-taking game.
Rust owns setup, legal actions, validation, private hands, pass commitment and
exchange, trick resolution, scoring, terminal outcome, effects, visibility,
replay, and bots. TypeScript presents Rust/WASM output only.

## Mechanic Inventory

| Category | Game-local description | Evidence | Current status | Notes |
|---|---|---|---|---|
| N-seat model | Exactly four independent seats, public observer plus four seat-private viewers. | [RULES.md](RULES.md), `setup.rs`, `visibility.rs` | `local-only` | Unsupported seat counts reject in Rust. |
| turn-order policy | Dealer rotates by hand; 2 clubs holder opens; trick winner leads next; pass waits for all four commitments. | [RULES.md](RULES.md), `state.rs`, `actions.rs` | `repeated-shape candidate` | Trick-winner-leads repeats Plain Tricks, but with four seats and pass/scoring pressure. |
| team/partnership/coalition | No teams, partnerships, coalitions, or temporary alliances. | [RULES.md](RULES.md) | `rejected/deferred with rationale` | Gate 18 owns partnership pressure later. |
| topology/spatial model | No board topology; the public surface is four seat rails, one current trick, trick history, and score table. | [UI.md](UI.md) | `local-only` | No grid, graph, path, region, or movement primitive. |
| component/zone model | 52 game-local cards, four private hands, current trick, captured tricks, cumulative score table. | `cards.rs`, `state.rs`, [RULE-COVERAGE.md](RULE-COVERAGE.md) | `repeated-shape candidate` | Card/private-hand pressure remains local pending atlas closeout. |
| hidden-hand/deck/wall model | Owner sees own unplayed cards; other viewers see counts; deck order and seed-derived future facts never project. | `visibility.rs`, `replay_support.rs`, `crates/wasm-api/src/games/briar.rs` | `repeated-shape candidate` | Pairwise no-leak is required for all ordered seat pairs. |
| action shape | Pass actions are progressive `pass/select/<card>`, `pass/unselect/<card>`, `pass/confirm`; play actions are `play/<card>`. | `actions.rs`, WASM action tree | `local-only` | UI maps Rust leaves; TypeScript does not compute legality. |
| turn/phase model | Passing, playing trick, scoring hand, terminal. Hold hands skip passing. | `state.rs`, [RULES.md](RULES.md) | `local-only` | Multi-hand advancement is represented by deterministic hand index/deal evidence. |
| randomness/chance | Seeded setup shuffle/deal only. No runtime chance after deal. | `setup.rs`, [SOURCES.md](SOURCES.md) | `local-only` | No browser randomness or bot sampling of hidden states. |
| visibility/hidden information | Private hands, pass selections, pass provenance, deck order, private effects, bot inputs, and exports are redacted per viewer. | `visibility.rs`, `replay_support.rs`, e2e no-leak smoke | `repeated-shape candidate` | Hidden-info surfaces are broader than Plain Tricks because four seats and pass commitments interact. |
| resource/accounting | Penalty scores: hearts = 1, queen of spades = 13, moon transform, cumulative low-score race. | `scoring.rs`, [RULES.md](RULES.md) | `local-only` | No shared pot, spendable resource, debt, or refund rule. |
| shared accounting/side-pot/split allocation | Not applicable. | [RULES.md](RULES.md) | `rejected/deferred with rationale` | River Ledger owns side-pot/split pressure; Briar has independent seat scores only. |
| movement/capture/placement | Trick winner captures four public played cards into captured-trick history. | `rules.rs`, `state.rs` | `local-only` | No board movement or capture conversion. |
| pattern/line/directional scanning | Not applicable. | [RULES.md](RULES.md) | `rejected/deferred with rationale` | No alignment, ray, adjacency, or directional scanning. |
| commitment/reveal | Four simultaneous pass commitments; identities stay owner/private until authorized by exchange or later public play. | `actions.rs`, `effects.rs`, `visibility.rs` | `repeated-shape candidate` | Similar to hidden commitment shape, but card-pass-specific and kept game-local. |
| reaction/window/pending response | No reactive interruption; pass has pending seats but deterministic serialized commands. | [RULES.md](RULES.md) | `local-only` | No cancellation/replacement/priority response stack. |
| scoring/outcome | Unique lowest cumulative score wins after threshold; low-score ties continue. | `scoring.rs`, [RULES.md](RULES.md), [UI.md](UI.md) | `local-only` | Seat order never breaks a tie. |
| evaluator/showdown/ranking | Not a showdown evaluator; final standings rank cumulative scores. | `scoring.rs` | `local-only` | No poker-style comparison vector or hidden reveal. |
| semantic effect shape | Pass selection, pass commitment, pass exchange public/private, card played, hearts broken, trick captured. | `effects.rs`, WASM bridge | `local-only` | Score/terminal presentation currently comes through projected view/outcome surface, not a separate terminal effect. |
| UI interaction pattern | Card buttons for owner hand, pass selection/confirm, current trick, captured-trick timeline, seat rail, outcome panel, replay import/export. | `BriarCircuitBoard.tsx`, [UI.md](UI.md) | `local-only` | Keyboard/e2e smoke covers pass selection and responsive table. |
| bot policy pattern | Level 0 random legal plus bounded Level 1 public/own-hand policy; Level 2 not admitted. | `bots.rs`, [AI.md](AI.md) | `local-only` | No MCTS, ISMCTS, Monte Carlo, ML, RL, or belief model. |
| benchmark/performance pressure | Setup/deal/serialization, pass/play legality, trick/scoring, projections, effect filtering, export/import, bots, full hand/match. | [BENCHMARKS.md](BENCHMARKS.md) | `local-only` | Native benchmark lane has 21 operations. |

## Repeated-Shape Comparison

| Mechanic shape | Already appears in | Same shape? | Similarities | Differences | Required next step |
|---|---|---:|---|---|---|
| follow-suit legality | `plain_tricks`, `vow_tide` | similar | Led suit restricts followers when able. | Briar adds 4 seats, 2 clubs opening, first-trick point restriction, hearts-broken lead rule, and penalty scoring. | Gate 17 promoted `game-stdlib::trick_taking::follow_suit_indices`; Briar adopts it for the pure base subset while keeping Hearts restrictions local. |
| led-suit trick comparator | `plain_tricks`, `vow_tide` | similar | Highest led suit wins; off-suit cannot win. | Briar has four-card tricks, captured point cards, and moon/threshold scoring. | Gate 17 promoted `game-stdlib::trick_taking::winning_play_index`; Briar adopts it with `trump = None`. |
| trick winner leads next | `plain_tricks` | similar | Winner becomes next leader. | Briar hand has 13 tricks and pass/dealer cycle. | Keep local/defer. |
| private hand and viewer no-leak | `high_card_duel`, `poker_lite`, `plain_tricks`, `river_ledger` | similar | Owner-private components and public projections. | Briar adds four-seat pairwise pass provenance and all-seat projections. | Record in atlas/capstone; no `engine-core` noun. |
| simultaneous hidden commitment | `secret_draft`, `masked_claims` | related, not same | Hidden choice before reveal/public consequence. | Briar commits card identities to deterministic pass routing, not bids/claims. | Keep local; no generic commitment primitive. |

## Primitive Pressure Decision

No `game-stdlib` or `engine-core` primitive was promoted by Gate 16. Gate 17
later promoted the narrow pure `game-stdlib::trick_taking` helper after Vow Tide
created the third close use. Briar Circuit now adopts `follow_suit_indices` for
the base led-suit subset and `winning_play_index` with `trump = None` for the
pure comparator.

Briar-specific pass, first-trick, hearts-broken, moon, threshold, four-seat
no-leak, scoring, effects, diagnostics, winner-leads, and UI rules remain
game-local.

`games/briar_circuit/docs/PRIMITIVE-PRESSURE-LEDGER.md` records the detailed
keep-local/defer decision. The series capstone owns the central atlas/status
update.

## Effects, UI, Bot, Visibility, And Benchmark Notes

| Area | Requirement | Rule IDs | Notes |
|---|---|---|---|
| semantic effects | Pass/progress, card played, hearts broken, trick captured, viewer filtering. | `BC-PASS-*`, `BC-PLAY-007`, `BC-TRICK-002` | Private pass effects are seat-scoped. |
| UI interaction pattern | Rust legal leaves only; pass confirm is Rust-authorized after three selections. | `BC-UI-001` | UI may group and highlight; it does not validate. |
| Rust-generated previews | Basic card/action previews are owner-scoped. | `BC-VIS-004` | No hidden opponent alternatives or deck facts. |
| bot policy pattern | L0 and L1 use legal action API only. | `BC-BOT-001`, `BC-BOT-002` | L1 is not claimed competent-human or Level 2. |
| visibility/no-leak | Public observer plus four seat viewers; pairwise seat-private card and pass data. | `BC-VIS-001` through `BC-VIS-004` | Includes DOM, storage, logs, exports, and bot explanations. |
| benchmark pressure | Native benchmark covers 21 operation lanes. | `BC-REPLAY-*`, `BC-OUTCOME-001` | No shortcut or weakened redaction is allowed to hit a floor. |

## Required Repo Atlas Update

| Update target | Required? | Reason | Owner |
|---|---:|---|---|
| `docs/MECHANIC-ATLAS.md` | yes | Gate 16 creates trick-taking second-use evidence and public scaling pressure. | GAT16BRICIRTRI-018 |
| `PRIMITIVE-PRESSURE-LEDGER.md` | already local | Game-local ledger records the defer decision. | completed before this trailing doc |
| ADR | no | No accepted boundary change, DSL, YAML, kernel noun, or bot/search exception. | not applicable |

## Review Checklist

- `engine-core` remains noun-free.
- Static data remains typed metadata/fixtures/reports only.
- TypeScript presents Rust/WASM payloads only.
- Hidden hands, pass provenance, deck order, and bot private facts stay out of unauthorized surfaces.
- Level 2 is not admitted.
- Gate 17 resolves the follow-suit/comparator third-use hard gate through the
  narrow helper; Briar Circuit adopts it without opening promotion debt.

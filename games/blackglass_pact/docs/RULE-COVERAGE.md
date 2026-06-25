# Blackglass Pact Rule Coverage

Game ID: `blackglass_pact`

Rules version: `blackglass-pact-rules-v1`

Data version: `blackglass-pact-data-v1`

Last updated: 2026-06-25

## Rule Coverage Matrix

This is the Gate 18 coverage matrix. Rows map stable BP-* rules to current Rust owners, tests, traces, fixtures, tools, docs, benchmarks, and later-ticket evidence owners.

| Rule ID | Rule summary | Planned implementation | Planned evidence | Status | Notes |
|---|---|---|---|---|---|
| `BP-ID-001` | Game id, variant, rules version, and data version match the Gate 18 contract. | manifest, variant data, WASM/tool registration | manifest/catalog/WASM/tool tests | `covered` | Lands after crate and catalog wiring. |
| `BP-ID-002` | Public name is Blackglass Pact; Spades appears only as family/source label. | docs, manifest, catalog copy | source/IP review, presentation-copy checks | `covered` | Human IP/public-release review pending. |
| `BP-SETUP-001` | Exactly four seats are supported; all other counts are rejected. | `games/blackglass_pact/src/setup.rs` | unit tests, invalid fixture traces | `covered` | Crate lands in GAT18BLAPACSPA-003. |
| `BP-SETUP-002` | Seats serialize in `seat_0..seat_3` order with North/East/South/West labels. | `ids.rs`, `setup.rs`, visibility/WASM adapters | serialization/view traces | `covered` | Stable order required. |
| `BP-SETUP-003` | `team_0 = seat_0 + seat_2`; `team_1 = seat_1 + seat_3`. | `partnerships.rs` | setup/team mapping trace | `covered` | Game-local team model only. |
| `BP-SETUP-004` | Team IDs are stable/public and do not replace seat IDs. | `partnerships.rs`, view/outcome models | view/outcome snapshots | `covered` | Partner hand visibility stays forbidden. |
| `BP-SETUP-005` | Initial dealer is `seat_0`; non-terminal dealer rotates clockwise. | `setup.rs`, `scoring.rs` | setup/dealer traces | `covered` | Terminal retains final hand context. |
| `BP-SETUP-006` | Match seed and version inputs are recorded under existing replay law. | `state.rs`, `replay_support.rs` | replay/serialization tests | `covered` | No new trace schema. |
| `BP-BLIND-001` | Blind nil requires the actor's team to trail by at least 100 at hand start. | `bidding.rs`, `setup.rs` | boundary tests at 99/100 and negative scores | `covered` | Ticket 004. |
| `BP-BLIND-002` | Eligible decisions run left of dealer clockwise before shuffle/deal. | `bidding.rs`, `setup.rs` | order trace | `covered` | Ticket 004. |
| `BP-BLIND-003` | Legal blind leaves are exactly declare/decline for the active eligible seat. | `actions`/`bidding.rs` | tree/validator tests | `covered` | Ticket 004. |
| `BP-BLIND-004` | Ineligible seats are skipped and receive no blind control. | `bidding.rs`, `visibility.rs` | view/action tests | `covered` | Ticket 004 and 008. |
| `BP-BLIND-005` | Declaration/decline is public, accepted once, and immutable. | `bidding.rs`, `effects.rs`, `replay_support.rs` | public projection/replay trace | `covered` | Ticket 004. |
| `BP-BLIND-006` | No hand/deck/card-derived preview or card-derived bot input exists before decision. | `setup.rs`, `visibility.rs`, `bots.rs`, `effects.rs` | blind no-leak corpus | `covered` | Ticket 004 and 008. |
| `BP-BLIND-007` | Blind decisions do not change shuffle/deal bytes. | `setup.rs`, `replay_support.rs` | paired-seed property/trace | `covered` | Ticket 004. |
| `BP-BLIND-008` | Both partners may independently declare; no combined bonus exists. | `bidding.rs`, `scoring.rs` | double-declaration score tests | `covered` | Ticket 004 and 007. |
| `BP-BLIND-009` | No card pass/exchange follows a declaration. | legal tree/state model | action-tree and state tests | `covered` | Ticket 004. |
| `BP-BLIND-010` | Declaring seat is skipped during ordinary bidding and has a zero blind-nil contract. | `bidding.rs`, `state.rs` | bid order/contract trace | `covered` | Ticket 004 and 005. |
| `BP-DEAL-001` | Deck has exactly one standard 52-card rank/suit combination, ace high. | `cards.rs` | deck unit/property tests | `covered` | Ticket 003. |
| `BP-DEAL-002` | Deal begins left of dealer and proceeds singly clockwise. | `setup.rs` | deterministic deal trace | `covered` | Ticket 004. |
| `BP-DEAL-003` | Each seat receives 13 unique cards and no tail remains. | `setup.rs`, `state.rs` | card conservation property | `covered` | Ticket 004. |
| `BP-DEAL-004` | Unplayed card is visible only to owning seat. | `visibility.rs` | viewer/pairwise tests | `covered` | Ticket 008. |
| `BP-DEAL-005` | Public deal evidence exposes counts and phase, not card identities. | `effects.rs`, `visibility.rs` | observer/effect trace | `covered` | Ticket 004 and 008. |
| `BP-DEAL-006` | Deal/redeal ordering is stable across replay and serialization. | `setup.rs`, `replay_support.rs` | replay/hash tests | `covered` | Ticket 011. |
| `BP-BID-001` | Bidding starts left of dealer and proceeds clockwise through dealer. | `bidding.rs` | bidding-order trace | `covered` | Ticket 005. |
| `BP-BID-002` | Each non-blind seat bids exactly once. | `bidding.rs`, `state.rs` | state/property tests | `covered` | Ticket 005. |
| `BP-BID-003` | Legal bid leaves are nil and integers 1-13. | `bidding.rs` | tree boundary tests | `covered` | Ticket 005. |
| `BP-BID-004` | Numeric zero, pass, rebid, and out-of-range values are illegal. | `bidding.rs`, validators | invalid traces | `covered` | Ticket 005. |
| `BP-BID-005` | Accepted bids become public immediately and are immutable. | `bidding.rs`, `effects.rs`, `visibility.rs` | public view/replay tests | `covered` | Ticket 005. |
| `BP-BID-006` | No total-13 or dealer last-bidder hook is applied. | `bidding.rs` | regression test contrasting Vow Tide | `covered` | Ticket 005. |
| `BP-BID-007` | Ordinary team contract sums positive numeric partner bids. | `bidding.rs`, `scoring.rs` | contract unit/property tests | `covered` | Ticket 005 and 007. |
| `BP-BID-008` | Nil and blind nil contribute zero to ordinary team contract. | `bidding.rs`, `scoring.rs` | contract trace | `covered` | Ticket 005 and 007. |
| `BP-BID-009` | Nil/blind-nil contracts remain attached to bidding seat. | `state.rs`, `scoring.rs` | state/serialization tests | `covered` | Ticket 005 and 007. |
| `BP-BID-010` | Team and seat bid projections use stable IDs/order. | `visibility.rs`, WASM adapter | view/WASM snapshots | `covered` | Ticket 008 and 014. |
| `BP-PLAY-001` | Seat left of dealer leads first trick. | `rules.rs`, `state.rs` | trace | `covered` | Ticket 006. |
| `BP-PLAY-002` | Before breaking, a leader with a non-spade cannot lead a spade. | `rules.rs` | rule/tree/diagnostic tests | `covered` | Ticket 006. |
| `BP-PLAY-003` | Only-spades leader may lead spade and break spades. | `rules.rs` | exception trace | `covered` | Ticket 006. |
| `BP-PLAY-004` | Off-suit spade by void follower breaks spades. | `rules.rs`, `effects.rs` | trace/property | `covered` | Ticket 006. |
| `BP-PLAY-005` | Follower must follow led suit when holding it. | `rules.rs`, `game-stdlib::trick_taking` | helper/tree/validator tests | `covered` | Ticket 006. |
| `BP-PLAY-006` | Void follower may play any owned card. | `rules.rs` | property/trace | `covered` | Ticket 006. |
| `BP-PLAY-007` | Highest spade wins if any spade is played. | `rules.rs`, `game-stdlib::trick_taking` | comparator conformance tests | `covered` | Ticket 006. |
| `BP-PLAY-008` | Otherwise highest led-suit rank wins; off-suit non-spades cannot win. | `rules.rs`, `game-stdlib::trick_taking` | comparator tests | `covered` | Ticket 006. |
| `BP-PLAY-009` | Trick winner leads next. | `rules.rs`, `state.rs` | transition trace | `covered` | Ticket 006. |
| `BP-PLAY-010` | Four cards complete a trick and 13 tricks complete a hand. | `rules.rs`, `state.rs` | state/property tests | `covered` | Ticket 006. |
| `BP-PLAY-011` | `follow_suit_indices` is reused unchanged. | `rules.rs`, helper tests | direct/helper conformance evidence | `covered` | Ticket 006 and 018. |
| `BP-PLAY-012` | `winning_play_index` is reused with `Some(Spades)` unchanged. | `rules.rs`, helper tests | direct/helper conformance evidence | `covered` | Ticket 006 and 018. |
| `BP-SCORE-001` | `C` sums positive numeric bids; `O` sums tricks won by ordinary bidders only. | `scoring.rs` | scoring unit/property tests | `covered` | Ticket 007. |
| `BP-SCORE-002` | Ordinary contract is made iff `O >= C`. | `scoring.rs` | boundary tests | `covered` | Ticket 007. |
| `BP-SCORE-003` | Made ordinary base is `+10 x C`. | `scoring.rs` | worked examples/traces | `covered` | Ticket 007. |
| `BP-SCORE-004` | Set ordinary base is `-10 x C`. | `scoring.rs` | worked examples/traces | `covered` | Ticket 007. |
| `BP-SCORE-005` | Made ordinary overtricks add +1 point and one bag each. | `scoring.rs` | trace/property | `covered` | Ticket 007. |
| `BP-SCORE-006` | Set ordinary contract produces no ordinary overtrick points/bags. | `scoring.rs` | regression test | `covered` | Ticket 007. |
| `BP-SCORE-007` | Made ordinary nil is +100; failed nil is -100. | `scoring.rs` | nil traces | `covered` | Ticket 007. |
| `BP-SCORE-008` | Made blind nil is +200; failed blind nil is -200. | `scoring.rs` | blind traces | `covered` | Ticket 007. |
| `BP-SCORE-009` | Failed nil/blind tricks never help ordinary contract. | `scoring.rs` | attribution tests | `covered` | Ticket 007. |
| `BP-SCORE-010` | Every failed nil/blind trick adds +1 point and one bag, even on ordinary set. | `scoring.rs` | cross-case trace | `covered` | Ticket 007. |
| `BP-SCORE-011` | Bags persist across hands as separate integer field. | `state.rs`, `scoring.rs` | serialization/history tests | `covered` | Ticket 007. |
| `BP-SCORE-012` | Every 10 raw bags subtracts 100 and removes 10; multiple thresholds repeat. | `scoring.rs` | threshold property/tests | `covered` | Ticket 007. |
| `BP-SCORE-013` | Bag remainder survives sets, nil outcomes, and target crossing. | `scoring.rs` | multi-hand traces | `covered` | Ticket 007. |
| `BP-SCORE-014` | Hand delta applies exact component order. | `scoring.rs` | unit oracle/trace breakdown | `covered` | Ticket 007. |
| `BP-SCORE-015` | Every hand exposes Rust-authored per-seat and per-team score components. | `scoring.rs`, `visibility.rs`, `effects.rs` | view/effect/outcome tests | `covered` | Ticket 007 and 008. |
| `BP-SCORE-016` | Integer arithmetic cannot overflow within supported evidence budgets. | `scoring.rs` | boundary/property tests | `covered` | Ticket 007. |
| `BP-END-001` | Terminal evaluation occurs only after all 13 tricks and hand scoring. | `scoring.rs`, `state.rs` | transition tests | `covered` | Ticket 007. |
| `BP-END-002` | At least one team must be 500+ and scores must differ. | `scoring.rs` | target boundary tests | `covered` | Ticket 007. |
| `BP-END-003` | Unique higher team wins when terminal predicate is met. | `scoring.rs`, outcome model | terminal traces | `covered` | Ticket 007. |
| `BP-END-004` | Exact tie at/above 500 continues to another full hand. | `scoring.rs` | tie trace | `covered` | Ticket 007. |
| `BP-END-005` | After tie continuation, falling below 500 does not create terminal state. | `scoring.rs` | multi-hand trace | `covered` | Ticket 007. |
| `BP-END-006` | Bags, seat order, dealer, and team ID are not tiebreakers. | `scoring.rs`, outcome model | outcome tests | `covered` | Ticket 007. |
| `BP-END-007` | Non-terminal hand advances dealer and starts fresh blind-eligibility phase. | `scoring.rs`, `setup.rs` | transition trace | `covered` | Ticket 007. |
| `BP-END-008` | Terminal state retains completed-hand dealer/context and no phantom hand. | `state.rs`, `scoring.rs` | serialization/trace | `covered` | Ticket 007. |
| `BP-END-009` | `standings_by_team` stable order with scores/ranks/winner flags. | outcome model, WASM adapter | outcome snapshot | `covered` | Ticket 007 and 014. |
| `BP-END-010` | `standings_by_seat` stable order with team/bid/tricks/nil/rank linkage. | outcome model, WASM adapter | outcome snapshot | `covered` | Ticket 007 and 014. |
| `BP-VIS-001` | Public observer receives no unplayed card or private control/candidate. | `visibility.rs`, replay export | observer corpus | `covered` | Ticket 008. |
| `BP-VIS-002` | Seat viewer receives own hand only. | `visibility.rs` | four seat-view tests | `covered` | Ticket 008. |
| `BP-VIS-003` | Partner relationship grants no private visibility. | `visibility.rs` | partner rows in pairwise matrix | `covered` | Ticket 008. |
| `BP-VIS-004` | Blind phase exposes no future card-derived datum. | `setup.rs`, `visibility.rs`, `bots.rs`, `effects.rs` | pre-deal corpus | `covered` | Ticket 008. |
| `BP-VIS-005` | Action trees/previews are actor- and viewer-scoped. | action tree, preview, `visibility.rs` | tree/preview tests | `covered` | Ticket 008. |
| `BP-VIS-006` | Diagnostics/effects reveal no unauthorized hand fact. | validators, `effects.rs`, `visibility.rs` | rejection/effect tests | `covered` | Ticket 008. |
| `BP-VIS-007` | Public and seat exports round-trip without privilege elevation. | `replay_support.rs`, WASM export/import | export tests/traces | `covered` | Ticket 011 and 014. |
| `BP-VIS-008` | DOM, storage, logs, test IDs, a11y tree, and animations contain no unauthorized datum. | web renderer/e2e harness | e2e no-leak checklist | `covered` | Ticket 016. |
| `BP-REPLAY-001` | Same accepted command stream reproduces state/effects/hash under fixed versions. | `replay_support.rs` | replay tests | `covered` | Ticket 011. |
| `BP-REPLAY-002` | Trace Schema v1 includes phase, actor, team context, bids, score components, and migration notes as applicable. | trace/golden support | golden validation | `covered` | Ticket 011. |
| `BP-REPLAY-003` | No unrelated golden is regenerated for Gate 18. | review/fixture workflow | review evidence | `covered` | Ticket 011 and capstone. |
| `BP-BOT-001` | L0 selects uniformly/deterministically from legal leaves with isolated bot RNG. | `bots.rs` | bot tests | `covered` | Ticket 009. |
| `BP-BOT-002` | L1 uses public facts, own hand, and lawful public-play deductions only. | `bots.rs`, `AI.md` | authorized-input tests | `covered` | Ticket 009 and 010. |
| `BP-BOT-003` | L1 explanations/candidates are viewer-safe and deterministic. | `bots.rs`, `visibility.rs`, `AI.md` | snapshot/no-leak tests | `covered` | Ticket 009 and 010. |
| `BP-BOT-004` | L2 is unadmitted; L3 and prohibited algorithms are absent. | `AI.md`, dependency/source scan | AI docs/dependency/code review | `covered` | Ticket 010. |
| `BP-UI-001` | Grouped table renders fixed partners and stable team IDs without color-only meaning. | `BlackglassPactBoard.tsx`, UI docs | a11y/e2e | `covered` | Ticket 015 and 017. |
| `BP-UI-002` | Blind, bid, and card controls come only from Rust legal leaves. | WASM adapter, renderer | WASM/UI tests | `covered` | Ticket 014 and 015. |
| `BP-UI-003` | Team scores, bags, contracts, nil state, ranks, and explanations come from Rust. | WASM adapter, renderer | payload/UI assertions | `covered` | Ticket 014 and 015. |
| `BP-UI-004` | Hotseat handoff removes prior private subtree before next seat render. | renderer/hotseat shell | e2e DOM inspection | `covered` | Ticket 015 and 016. |
| `BP-UI-005` | Observer/replay/rules/outcome surfaces are complete and accessible. | renderer/shared surfaces | shared/dedicated smokes | `covered` | Ticket 015 and 016. |
| `BP-UI-006` | Reduced motion and logical focus/status behavior preserve semantic results. | renderer/effects scheduler | accessibility smoke | `covered` | Ticket 016. |

## Planned Golden Trace Inventory

| Trace | Required focus | Rule IDs |
|---|---|---|
| blind no-card surface | blind commitment before any card identity exists | `BP-BLIND-002` through `BP-BLIND-007`, `BP-VIS-004` |
| blind declare vs decline same deal | paired-seed RNG independence | `BP-BLIND-007`, `BP-DEAL-006` |
| deterministic deal | full 52-card deal, 13 cards per seat, no tail | `BP-DEAL-001` through `BP-DEAL-003` |
| bid order and team contract | left-of-dealer bid order, skipped blind bidder, team ordinary contract | `BP-BID-001` through `BP-BID-010` |
| broken-spades lead restriction | no spade lead before broken unless only spades | `BP-PLAY-002`, `BP-PLAY-003` |
| follow suit and trump winner | follow-suit helper and spades-trump comparator | `BP-PLAY-005` through `BP-PLAY-012` |
| nil and bags | made/failed nil, failed-nil bag attribution, threshold rollover | `BP-SCORE-007` through `BP-SCORE-014` |
| terminal tie continuation | 500+ tie continues; later unique higher team wins | `BP-END-002` through `BP-END-006` |
| pairwise no-leak export | observer and four seat-private exports remain authorized | `BP-VIS-001` through `BP-VIS-008`, `BP-REPLAY-001` |

## Initial Coverage Status

This matrix is finalized for the current Gate 18 implementation state. Later WASM, web, release, and capstone tickets remain responsible for their named evidence surfaces.

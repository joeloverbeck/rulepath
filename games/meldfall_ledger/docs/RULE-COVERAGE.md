# Meldfall Ledger Rule Coverage

Game ID: `meldfall_ledger`

Rules version: `meldfall-ledger-rules-v1`

Data version: `meldfall-ledger-data-v1`

Last updated: 2026-06-27

## Rule Coverage Matrix

This is the Gate 19 coverage matrix. Rows map stable `ML-*` rules to Rust owners,
tests, golden traces, fixtures, tools, docs, benchmarks, and later-ticket
evidence owners.

| Rule ID | Rule summary | Implementation | Evidence | Status | Notes |
|---|---|---|---|---|---|
| `ML-ID-001` | Game id, variant, rules version, and data version are stable. | `ids.rs`, `variants.rs`, manifest data, tool registrations | static-data tests, `fixture-check`, `replay-check`, [BENCHMARKS.md](BENCHMARKS.md) | `covered` | WASM/catalog wiring lands later but Rust/tool identity is pinned. |
| `ML-ID-002` | Public copy uses Meldfall Ledger; family names are source labels only. | [RULES.md](RULES.md), [SOURCES.md](SOURCES.md), [AI.md](AI.md) | source/IP docs and doc-link check | `covered` | Human public-release review remains pending. |
| `ML-SETUP-001` | Seat counts are exactly 2 through 6, default 4. | `setup.rs`, `ids.rs` | `setup_deals_correct_counts_for_supported_seat_counts`, invalid-seat-count traces, fixture profiles | `covered-by-trace` | Diagnostics: `ML_INVALID_SEAT_COUNT`. |
| `ML-SETUP-002` | Stable `seat_0` through `seat_5` serialization and labels. | `ids.rs`, `setup.rs`, `visibility.rs` | `canonical_seats_use_engine_core_grammar`, setup traces, 6p no-leak trace | `covered-by-trace` | Browser labels land later. |
| `ML-SETUP-003` | One standard 52-card deck, no jokers or duplicates. | `cards.rs`, `setup.rs` | `canonical_deck_has_52_unique_local_card_ids`, property tests | `covered` | Game-local card nouns only. |
| `ML-SETUP-004` | Deal 13 cards at 2 seats, 7 cards at 3-6 seats, one initial discard, hidden stock. | `setup.rs` | setup 2p/4p/6p traces, fixture profiles, property tests | `covered-by-trace` | One-deck 5/6-seat choice is deliberate. |
| `ML-SETUP-005` | Initial dealer and first active seat are deterministic and clockwise. | `setup.rs`, `ids.rs` | `deal_order_starts_left_of_dealer_clockwise`, setup traces | `covered-by-trace` | Dealer rotation after nonterminal rounds remains Rust-owned. |
| `ML-SETUP-006` | Setup/deal ordering is deterministic from seed and version inputs. | `setup.rs`, `variants.rs` | `setup_is_deterministic_for_seed_and_seat_count`, serialization tests | `covered` | Wall-clock/browser randomness is not an input. |
| `ML-VIS-001` | Public observer sees only public table state, counts, scores, diagnostics, effects, and standings. | `visibility.rs`, `effects.rs` | `public_observer_view_exposes_counts_and_public_cards_not_hidden_hands_or_stock`, public-observer no-leak trace | `covered-by-trace` | Includes max-seat observer surface. |
| `ML-VIS-002` | Seat viewer sees only that seat's private hand. | `visibility.rs` | `seat_private_view_exposes_only_the_viewer_hand`, pairwise no-leak tests | `covered` | Other hands remain counts only. |
| `ML-VIS-003` | Opponent hands, stock order, private labels, rankings, and hidden diagnostics never leak. | `visibility.rs`, `replay_support.rs`, `bots.rs` | visibility/replay/bot no-leak tests, no-leak traces | `covered-by-trace` | Browser DOM/storage/log proof lands later. |
| `ML-VIS-004` | Discard pile is public and ordered oldest to newest. | `state.rs`, `visibility.rs`, `actions.rs` | draw-source and discard-pickup traces, serialization tests | `covered-by-trace` | Newest/top discard is last. |
| `ML-VIS-005` | Melded and laid-off cards are public after tabled. | `visibility.rs`, `rules.rs` | public-tableau trace, tableau projection tests | `covered-by-trace` | Tabled cards remain public for every viewer. |
| `ML-VIS-006` | Public settlement exposes totals/counts without unauthorized unmelded identities. | `scoring.rs`, `state.rs`, `visibility.rs`, `crates/wasm-api`, web renderer | round-scoring trace, settlement no-leak tests, `last_settlement` projection tests, Meldfall browser settlement smoke | `covered-by-trace` | `last_settlement` carries round end, tabled totals, held penalties/counts, delta, cumulative score, rank, and winner flag; no exact opponent unmelded identities. |
| `ML-TURN-001` | Turn starts with active seat choosing a draw source. | `actions.rs`, `rules.rs`, `state.rs` | draw-source-choice and discard-after-draw traces | `covered-by-trace` | Wrong-seat and wrong-phase diagnostics tested. |
| `ML-TURN-002` | Active seat may draw one hidden stock card when stock is non-empty. | `rules.rs`, `effects.rs`, `visibility.rs` | deterministic-stock-draw no-leak trace, stock draw tests | `covered-by-trace` | Public effect hides drawn identity. |
| `ML-TURN-003` | Active seat may draw selected discard plus all newer cards. | `rules.rs`, `actions.rs` | draw-source-choice, multi-card pickup, top-discard traces | `covered-by-trace` | Public discard indices are oldest-to-newest. |
| `ML-TURN-004` | Selected discard from any discard draw must be used immediately. | `rules.rs`, `state.rs` | invalid pickup, top discard, multi-card pickup traces | `covered-by-trace` | Diagnostic: `ML_PICKUP_COMMITMENT_UNSATISFIED`. |
| `ML-TURN-005` | After drawing, active seat may create melds and lay off cards. | `rules.rs`, `actions.rs` | discard-after-draw, go-out-without-discard traces, rules tests | `covered-by-trace` | Optional except commitment/empty-hand cases. |
| `ML-TURN-006` | If hand remains and commitment is clear, discard exactly one card to end turn. | `rules.rs`, `effects.rs` | discard-after-draw and go-out-by-final-discard traces | `covered-by-trace` | Diagnostic: `ML_DISCARD_CARD_NOT_OWNED`. |
| `ML-TURN-007` | Empty hand after table plays ends round without final discard. | `rules.rs`, `state.rs` | go-out-without-final-discard trace | `covered-by-trace` | Floating is excluded. |
| `ML-TURN-008` | Final discard after table plays can end the round. | `rules.rs`, `state.rs` | go-out-by-final-discard trace | `covered-by-trace` | Final discard remains public. |
| `ML-TURN-009` | Empty stock with no legal/accepted discard draw settles the round. | `rules.rs`, `scoring.rs` | stock-exhausted-round-settlement trace | `covered-by-trace` | No discard reshuffle. |
| `ML-MELD-001` | Sets are 3-4 same-rank cards with distinct suits. | `rules.rs`, `cards.rs` | meld-set trace, property tests | `covered-by-trace` | Single-deck duplicates are rejected. |
| `ML-MELD-002` | Runs are 3+ consecutive cards in one suit. | `rules.rs`, `cards.rs` | meld-run trace, property tests | `covered-by-trace` | Run order is Rust-owned. |
| `ML-MELD-003` | Aces may be low or high but not around-the-corner. | `cards.rs`, `rules.rs` | meld-run ace-low/high/no-wrap trace | `covered-by-trace` | Ace score remains 15. |
| `ML-MELD-004` | New melds use only active seat owned cards and remove them atomically. | `rules.rs`, `state.rs` | meld traces, ownership conservation property | `covered-by-trace` | Diagnostic: `ML_MELD_CARD_NOT_OWNED`. |
| `ML-MELD-005` | Accepted meld groups have stable public IDs, origin seat, cards, and credit owner. | `state.rs`, `visibility.rs` | public-tableau trace, serialization tests | `covered-by-trace` | Origin seat is not a scoring shortcut. |
| `ML-LAYOFF-001` | Seat may lay off onto any existing public meld when the result remains legal. | `rules.rs`, `actions.rs` | own/opponent layoff traces, invalid layoff trace | `covered-by-trace` | Diagnostic: `ML_INVALID_LAYOFF`. |
| `ML-LAYOFF-002` | Laid-off card scores to the seat that played it. | `rules.rs`, `scoring.rs` | opponent-tableau score-credit trace, scoring tests | `covered-by-trace` | Per-card credit owner is authoritative. |
| `ML-LAYOFF-003` | Tabled meld groups cannot be rearranged, split, merged, or remelded. | `rules.rs`, `state.rs` | invalid-layoff trace and layoff tests | `covered-by-trace` | Extension only. |
| `ML-SCORE-001` | Card values are ace 15, face/tens 10, pips 2-9. | `cards.rs`, `scoring.rs` | card value tests, round-scoring trace | `covered-by-trace` | Low ace still scores 15. |
| `ML-SCORE-002` | Tabled cards score positive to their credit owner. | `scoring.rs`, `rules.rs`, `visibility.rs` | round-scoring and public-tableau traces, `last_settlement` projection tests | `covered-by-trace` | Includes meld and layoff cards; projected as `tabled_positive`. |
| `ML-SCORE-003` | In-hand cards subtract from their holders at settlement. | `scoring.rs`, `visibility.rs` | round-scoring and scores-can-go-negative traces, `last_settlement` no-leak tests | `covered-by-trace` | Public settlement exposes penalty total and held count, not exact opponent identities. |
| `ML-SCORE-004` | Round delta is tabled positives minus in-hand penalties. | `scoring.rs`, `visibility.rs`, web renderer | score-delta property, round-scoring trace, Meldfall settlement smoke | `covered-by-trace` | Rust computes and projects `delta`; TypeScript renders only. |
| `ML-SCORE-005` | Round deltas add to cumulative match scores. | `scoring.rs`, `state.rs` | scores-can-go-negative, multi-round traces | `covered-by-trace` | Negative cumulative scores allowed. |
| `ML-SCORE-006` | Per-card `played_by` is score-credit owner; meld origin is not a shortcut. | `state.rs`, `rules.rs`, `scoring.rs` | opponent layoff score-credit trace | `covered-by-trace` | Protects layoff credit. |
| `ML-SCORE-007` | Public settlement exposes totals/counts without unauthorized unmelded identities. | `scoring.rs`, `state.rs`, `visibility.rs`, `crates/wasm-api`, web renderer | no-leak matrix, round-scoring trace, `last_settlement` projection tests, a11y no-leak settlement panel smoke | `covered-by-trace` | Mirrors `ML-VIS-006`; persistent view field survives the next round and stays count/total only. |
| `ML-MATCH-001` | Terminal eligibility is evaluated only after settlement and needs a score at/above 500. | `scoring.rs`, `state.rs` | multi-round-first-to-500, target-tie traces | `covered-by-trace` | No mid-turn terminal shortcut. |
| `ML-MATCH-002` | Unique highest eligible seat wins. | `scoring.rs` | multi-round-first-to-500 trace, terminal tests | `covered-by-trace` | Winner is seat-indexed. |
| `ML-MATCH-003` | Equal highest scores at/above 500 continue. | `scoring.rs`, `rules.rs` transition path | target-tie-continues trace, `round-transition-resets-table-state` trace, transition/full-play tests | `covered-by-trace` | Seat order is not a tiebreaker; tied eligible scores continue through the next-round transition. |
| `ML-MATCH-004` | If no seat is at/above 500, match continues with next round. | `scoring.rs` | target-tie and scoring tests | `covered-by-trace` | Below-target scores remain ordinary. |
| `ML-MATCH-005` | Terminal standings are stable in seat order with score, delta, rank, and winner. | `scoring.rs`, `visibility.rs` | multi-round trace, serialization tests | `covered-by-trace` | Rust-authored outcome only. |
| `ML-MATCH-006` | Nonterminal settled round advances dealer and deals a fresh round. | `state.rs`, `setup.rs`, `rules.rs`, WASM bridge, `tools/simulate` | `round-transition-resets-table-state` trace, transition tests, host full-play tests, simulator completion | `covered-by-trace` | Dealer rotates, round-only state resets, cumulative scores carry forward, and both hosts continue to terminal. |
| `ML-REPLAY-001` | Same accepted command stream reproduces state, effects, views, and hashes. | `replay_support.rs`, tests | replay/serialization tests, `replay-check` registration | `covered` | Trace schema unchanged. |
| `ML-REPLAY-002` | Viewer-scoped exports never elevate privilege on import. | `replay_support.rs`, `visibility.rs` | viewer export/import tests and traces | `covered-by-trace` | Public and seat exports stay scoped. |
| `ML-REPLAY-003` | Trace Schema v1 records setup, draw, pickup, meld, layoff, scoring, terminal, visibility, and migration notes. | golden traces, `replay_support.rs`, `replay-check` | golden trace inventory and `replay-check --game meldfall_ledger --all` | `covered-by-trace` | No migration authorized. |
| `ML-BOT-001` | L0 random legal selects deterministically from Rust legal actions and validates normally. | `bots.rs`, `simulate` | bot tests, L0 trace, simulator evidence | `covered-by-trace` | L0 is legality/simulation baseline only. |
| `ML-BOT-002` | L1 may use only public facts, own hand, Rust legal actions, and deterministic authored preferences. | `bots.rs`, [AI.md](AI.md), [BOT-STRATEGY-EVIDENCE-PACK.md](BOT-STRATEGY-EVIDENCE-PACK.md) | L1-not-admitted trace, AI docs, no forbidden-method grep | `covered` | L1 is not admitted pending strategy evidence. |
| `ML-BOT-003` | Bot explanations and candidate rankings are viewer-safe. | `bots.rs`, `visibility.rs` | bot explanation tests, visibility no-leak matrix | `covered` | No private rankings are public. |
| `ML-UI-001` | Browser controls present Rust legal actions and Rust-safe previews only. | future WASM/web adapter | later WASM/UI tickets | `intentionally-deferred` | Owned by GAT19MELLEDFIV-019 through 021. |
| `ML-UI-002` | Public UI supports large hands, meld groups, discard-tail choices, score ledger, replay/import/export, and no-drag action path. | future web renderer | later UI smoke and browser no-leak evidence | `intentionally-deferred` | Owned by GAT19MELLEDFIV-020 through 021. |
| `ML-UI-003` | DOM, a11y, test IDs, storage, logs, and effects must not leak hidden cards. | future web renderer and e2e smokes | later browser no-leak/a11y smoke | `intentionally-deferred` | Rust no-leak surfaces are covered; browser proof lands later. |

## Golden Trace Inventory

| Trace | Required focus | Rule IDs |
|---|---|---|
| `setup-2p-13-card-deal.trace.json` | 2-seat deal, public counts only | `ML-SETUP-001`, `ML-SETUP-004`, `ML-VIS-001` |
| `setup-4p-default.trace.json` | default setup and stock/discard counts | `ML-SETUP-004`, `ML-SETUP-005` |
| `setup-6p-max-seat.trace.json` | max-seat setup and stable seat keys | `ML-SETUP-001`, `ML-SETUP-002`, `ML-SETUP-004` |
| `invalid-seat-count-below.trace.json` | unsupported low seat diagnostic | `ML-SETUP-001` |
| `invalid-seat-count-above.trace.json` | unsupported high seat diagnostic | `ML-SETUP-001` |
| `deterministic-stock-draw-no-leak.trace.json` | stock draw hides identity from public | `ML-TURN-002`, `ML-VIS-003` |
| `draw-source-choice-stock-vs-discard.trace.json` | stock/discard draw action tree | `ML-TURN-001`, `ML-TURN-002`, `ML-TURN-003` |
| `multi-card-discard-pickup-melds-deepest.trace.json` | deep discard pickup and immediate use | `ML-TURN-003`, `ML-TURN-004`, `ML-MELD-004` |
| `invalid-discard-pickup-without-use.trace.json` | commitment blocks finish/discard | `ML-TURN-004`, `ML-TURN-006` |
| `top-discard-pickup-also-requires-use.trace.json` | top discard also requires use | `ML-TURN-003`, `ML-TURN-004` |
| `meld-set-valid-and-invalid.trace.json` | set validation and diagnostics | `ML-MELD-001`, `ML-MELD-004`, `ML-MELD-005` |
| `meld-run-valid-ace-low-high-no-wrap.trace.json` | run validation and ace handling | `ML-MELD-002`, `ML-MELD-003`, `ML-MELD-004` |
| `layoff-onto-own-tableau.trace.json` | own meld extension | `ML-LAYOFF-001`, `ML-LAYOFF-002`, `ML-LAYOFF-003` |
| `layoff-onto-opponent-tableau-score-credit.trace.json` | opponent meld extension with credit owner | `ML-LAYOFF-001`, `ML-LAYOFF-002`, `ML-SCORE-006` |
| `invalid-layoff-gap-or-wrong-rank.trace.json` | layoff rejection | `ML-LAYOFF-001`, `ML-LAYOFF-003` |
| `discard-after-draw-turn-end.trace.json` | normal draw/table/discard lifecycle | `ML-TURN-001`, `ML-TURN-002`, `ML-TURN-005`, `ML-TURN-006` |
| `go-out-by-final-discard.trace.json` | round end by final discard | `ML-TURN-006`, `ML-TURN-008` |
| `go-out-without-final-discard.trace.json` | round end without discard | `ML-TURN-005`, `ML-TURN-007` |
| `stock-exhausted-round-settlement.trace.json` | stock exhaustion settlement | `ML-TURN-009` |
| `round-scoring-positive-negative.trace.json` | tabled positives, hand penalties, visibility | `ML-SCORE-001`, `ML-SCORE-002`, `ML-SCORE-003`, `ML-SCORE-004`, `ML-SCORE-006`, `ML-VIS-006` |
| `scores-can-go-negative.trace.json` | negative cumulative score | `ML-SCORE-003`, `ML-SCORE-005` |
| `multi-round-first-to-500.trace.json` | unique highest target win | `ML-MATCH-001`, `ML-MATCH-002`, `ML-MATCH-005` |
| `target-tie-continues.trace.json` | 500 tie continuation | `ML-MATCH-001`, `ML-MATCH-003`, `ML-MATCH-004` |
| `round-transition-resets-table-state.trace.json` | nonterminal transition reset/preserve contract | `ML-MATCH-003`, `ML-MATCH-004`, `ML-MATCH-006`, `ML-SETUP-005`, `ML-VIS-003` |
| `public-observer-no-leak-6p.trace.json` | max-seat public no-leak | `ML-VIS-001`, `ML-VIS-003` |
| `seat-private-export-round-trip-all-viewers.trace.json` | all seat-private exports | `ML-REPLAY-002`, `ML-VIS-002`, `ML-VIS-003` |
| `viewer-export-no-privilege-elevation.trace.json` | import scope cannot elevate | `ML-REPLAY-002` |
| `l0-random-legal-full-match.trace.json` | bounded L0 random-legal playout evidence | `ML-BOT-001` |
| `l1-rule-informed-smoke.trace.json` | L1 not admitted receipt | `ML-BOT-002` |

## Diagnostic Coverage

| Diagnostic code | Owner | Evidence |
|---|---|---|
| `ML_INVALID_SEAT_COUNT` | `setup.rs` | invalid-seat-count traces and setup tests |
| `ML_INVALID_DEALER_INDEX`, `ML_INVALID_HAND_SIZE`, `ML_DEAL_CAPACITY_EXCEEDED`, `ML_DEAL_STOCK_EMPTY`, `ML_DEAL_DECK_EXHAUSTED` | `setup.rs` | setup validation tests and code review |
| `ML_MELD_TOO_SMALL`, `ML_MELD_DUPLICATE_CARD`, `ML_INVALID_MELD_SHAPE`, `ML_MELD_CARD_NOT_OWNED` | `rules.rs` | meld set/run traces and rules tests |
| `ML_INVALID_LAYOFF`, `ML_UNKNOWN_MELD`, `ML_LAYOFF_CARD_NOT_OWNED` | `rules.rs` | layoff traces and rules tests |
| `ML_STOCK_EMPTY`, `ML_INVALID_DISCARD_INDEX`, `ML_PICKUP_COMMITMENT_UNSATISFIED`, `ML_DISCARD_CARD_NOT_OWNED`, `ML_WRONG_PHASE`, `ML_WRONG_SEAT`, `ML_INVALID_SEAT_INDEX`, `ML_INVALID_TURN_RING` | `rules.rs` | turn lifecycle, pickup, stock, and diagnostics tests |
| `ML_BOT_UNKNOWN_CARD`, `ML_BOT_UNKNOWN_ACTION`, `no_legal_actions` | `bots.rs` | bot parser tests and L0 decision tests |

## Variant Decisions And Exclusions

| Decision/exclusion | Rule IDs | Evidence |
|---|---|---|
| One 52-card deck for all supported seats; no jokers/wilds/two-deck shoe. | `ML-SETUP-003`, `ML-SETUP-004` | [SOURCES.md](SOURCES.md), setup/property tests, setup traces |
| Top discard pickup creates immediate-use commitment. | `ML-TURN-003`, `ML-TURN-004` | top-discard pickup trace |
| No floating or mandatory final discard. | `ML-TURN-007`, `ML-TURN-008` | go-out traces |
| No tabled-meld rearrangement/remelding. | `ML-LAYOFF-003` | invalid layoff trace and layoff tests |
| No partnerships/teams/trick-taking/bids/contracts/nils/bags. | `ML-SETUP-001`, `ML-ID-001` | rules/source docs and absence from Rust state model |
| No MCTS, ISMCTS, Monte Carlo rollouts/search, ML, RL, runtime LLMs, hidden-world sampling, or stock/opponent-hand peeking. | `ML-BOT-002`, `ML-BOT-003` | [AI.md](AI.md), [BOT-STRATEGY-EVIDENCE-PACK.md](BOT-STRATEGY-EVIDENCE-PACK.md), bot tests |

## Coverage Status

This matrix is complete for the current Rust/tooling evidence state. Browser UI
rows are intentionally deferred to the WASM/web tickets that own those surfaces.

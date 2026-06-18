# River Ledger Rule Coverage

Game ID: `river_ledger`

Variant: `river_ledger_standard`

Last updated: 2026-06-16

## Coverage status

This matrix reconciles every stable `RL-*` rule ID in `RULES.md` to the Rust
implementation, fixture, trace, simulation, benchmark, bot, visibility, replay,
and later WASM/web proof surfaces. Tool registration for this matrix is owned by
GAT15RIVLEDTEX-015.

Status values follow `docs/OFFICIAL-GAME-CONTRACT.md`: `covered`,
`covered-by-trace`, `not-applicable`, `intentionally-deferred`, or
`unsupported`.

## Rule Coverage Matrix

| Rule ID | Rule summary | Implementation | Evidence | Status | Notes |
|---|---|---|---|---|---|
| `RL-SETUP-SEATS-001` | Stable seat positions. | `ids.rs`, `setup.rs`, `state.rs` | `games/river_ledger/tests/rules.rs`; setup traces 3p-6p; fixture-check | covered | Seat IDs remain game-local. |
| `RL-DEAL-DECK-001` | Game-local deck. | `cards.rs`, `setup.rs` | serialization tests; setup fixtures; no-leak traces | covered | Deck tail stays internal. |
| `RL-DEAL-CARD-001` | Game-local card identity. | `cards.rs`, `evaluator.rs`, `visibility.rs` | evaluator tests; visibility tests; no-leak traces | covered | Card nouns are not engine-core vocabulary. |
| `RL-DEAL-HOLE-001` | Owner-private hole cards. | `setup.rs`, `visibility.rs`, `effects.rs` | `visibility.rs` tests; `deal-private-no-leak.trace.json`; replay export tests | covered | Opponent holes stay redacted. |
| `RL-DEAL-BOARD-001` | Revealed community cards. | `rules.rs`, `effects.rs`, `visibility.rs` | street traces; visibility tests; replay-check | covered-by-trace | Future board cards remain internal. |
| `RL-BET-LEDGER-001` | Public contribution ledger. | `state.rs`, `betting.rs`, `visibility.rs` | rule tests; fixture-check; simulator output | covered | Units are abstract. |
| `RL-BET-BUTTON-001` | Button marker. | `setup.rs`, `pot.rs`, `state.rs` | setup fixtures; split-remainder trace | covered-by-trace | Also defines remainder order. |
| `RL-BET-BLIND-001` | Blind roles. | `setup.rs`, `betting.rs` | setup fixtures; preflop trace; simulator | covered | Public forced contributions only. |
| `RL-STREET-PHASE-001` | Street phase. | `state.rs`, `rules.rs`, `visibility.rs` | street traces; replay hashes; rule tests | covered-by-trace | Public phase names are stable. |
| `RL-POT-SINGLE-001` | Single pot term. | `pot.rs`, `state.rs` | pot tests; showdown traces | covered | Side pots excluded. |
| `RL-SETUP-SEATS-002` | Seat-count validation. | `setup.rs`, `ids.rs` | invalid-seat-count trace; setup tests; fixture-check | covered-by-trace | Accepts exactly 3-6 seats. |
| `RL-SETUP-VARIANT-001` | Typed standard variant. | `variants.rs`, `ui.rs` | variant loader tests; boundary-check; fixture-check | covered | Static data remains metadata. |
| `RL-DEAL-DECK-002` | Stable deck construction. | `cards.rs`, `setup.rs` | deterministic setup tests; serialization tests | covered | Stable order supports replay. |
| `RL-DEAL-SHUFFLE-001` | Seeded shuffle. | `setup.rs`, `cards.rs` | replay tests; golden traces; simulator seeds | covered-by-trace | Deterministic engine RNG only. |
| `RL-BET-BUTTON-002` | Button/blind assignment. | `setup.rs` | setup traces 3p-6p; fixtures | covered-by-trace | Single-hand base records button. |
| `RL-BET-BLINDS-002` | Forced blind contributions. | `setup.rs`, `betting.rs` | preflop trace; fixtures; simulator | covered-by-trace | Contributions are public counts. |
| `RL-DEAL-HOLE-002` | Deal two hole cards. | `setup.rs`, `visibility.rs` | setup fixtures; visibility tests; no-leak traces | covered | Owners only before authorized reveal. |
| `RL-DEAL-BOARD-002` | Reserve board internally. | `setup.rs`, `rules.rs`, `visibility.rs` | street traces; no-leak traces | covered-by-trace | Burn/future board identities are not projected. |
| `RL-STREET-PREFLOP-001` | Initial preflop actor. | `setup.rs`, `state.rs` | setup traces; preflop trace | covered-by-trace | Actor after big blind by seat order. |
| `RL-STREET-PREFLOP-002` | Preflop round closure. | `rules.rs`, `betting.rs` | preflop-blinds-call-check-advance trace | covered-by-trace | Matching/checking closes round. |
| `RL-STREET-FLOP-001` | Flop reveal. | `rules.rs`, `effects.rs` | flop-small-bet-cap trace; effect tests | covered-by-trace | Reveals three public cards. |
| `RL-STREET-TURN-001` | Turn reveal. | `rules.rs`, `effects.rs` | turn-river-big-bet trace | covered-by-trace | Uses big contribution unit. |
| `RL-STREET-RIVER-001` | River reveal. | `rules.rs`, `effects.rs`, `showdown.rs` | turn-river-big-bet trace; showdown traces | covered-by-trace | Advances to showdown when live seats remain. |
| `RL-STREET-SHOWDOWN-001` | Terminal showdown. | `rules.rs`, `showdown.rs` | showdown traces; replay tests | covered-by-trace | Terminal outcome recorded by Rust. |
| `RL-STREET-FOLDOUT-001` | Last-live foldout. | `rules.rs`, `showdown.rs` | foldout-last-live-hand trace; no-leak tests | covered-by-trace | No folded-card reveal. |
| `RL-BET-ACTION-001` | Fold action. | `actions.rs`, `rules.rs` | rule tests; foldout trace | covered | May trigger foldout. |
| `RL-BET-ACTION-002` | Check/bet when no amount owed. | `actions.rs`, `betting.rs`, `rules.rs` | legal-action tests; flop trace | covered | Rust legal tree owns availability. |
| `RL-BET-ACTION-003` | Call/raise/fold facing contribution. | `actions.rs`, `rules.rs` | preflop trace; cap tests | covered | Validation rejects unavailable actions. |
| `RL-BET-ACTION-004` | Cap-reached actions. | `actions.rs`, `betting.rs` | raise-cap-diagnostic trace | covered-by-trace | Diagnostics cite public cap only. |
| `RL-BET-ACTION-005` | Terminal action tree empty. | `actions.rs`, `rules.rs` | terminal traces; action-tree tests | covered | No gameplay actions after terminal. |
| `RL-BET-ACTION-006` | Wrong/stale/malformed rejection. | `actions.rs`, `rules.rs` | wrong-seat-diagnostic trace; validation tests | covered-by-trace | Invalid commands do not mutate state. |
| `RL-BET-LIMIT-001` | Small unit preflop/flop. | `betting.rs`, `state.rs` | preflop/flop traces; rule tests | covered-by-trace | Abstract units only. |
| `RL-BET-LIMIT-002` | Big unit turn/river. | `betting.rs`, `state.rs` | turn-river-big-bet trace | covered-by-trace | Fixed by Rust rules. |
| `RL-BET-CAP-001` | Raise cap. | `betting.rs`, `actions.rs` | raise-cap trace; rule tests | covered-by-trace | Resets per street. |
| `RL-BET-CALL-001` | Exact call amount. | `betting.rs`, `rules.rs` | rule tests; preflop trace | covered | Invalid calls mutate nothing. |
| `RL-BET-RAISE-001` | Exact raise amount. | `betting.rs`, `rules.rs` | cap tests; property tests | covered | Cap enforcement in validation. |
| `RL-BET-CHECK-001` | Check only when even. | `actions.rs`, `rules.rs` | legal-action tests; preflop trace | covered | Browser controls consume Rust tree later. |
| `RL-BET-AMB-001` | Base play never requires all-in handling. | `betting.rs`, `pot.rs`, `variants.rs` | property tests; out-of-scope docs; fixture-check | covered | Contribution capacity avoids all-in-required states. |
| `RL-POT-SINGLE-002` | Single terminal pot. | `pot.rs`, `betting.rs` | pot tests; showdown traces | covered | No side-pot model. |
| `RL-POT-ALLIN-001` | All-in absent. | `pot.rs`, `betting.rs` | property tests; out-of-scope docs | covered | Capacity avoids all-in-required states. |
| `RL-EVAL-FIVE-001` | Five-card category. | `evaluator.rs` | evaluator unit tests; high-card trace | covered | Includes all standard categories. |
| `RL-EVAL-ACELOW-001` | Ace-low straight. | `evaluator.rs` | straight-ace-low trace; evaluator tests | covered-by-trace | Ace-high remains highest straight. |
| `RL-EVAL-SEVEN-001` | Seven-card best of 21. | `evaluator.rs`, `showdown.rs` | evaluator tests; benchmark lane | covered | No lookup table. |
| `RL-EVAL-TIEBREAK-001` | Category then rank vector. | `evaluator.rs`, `showdown.rs` | pair/high-card and flush/full-house traces | covered-by-trace | Suits do not break ties. |
| `RL-EVAL-USED-001` | Used-card explanation. | `evaluator.rs`, `showdown.rs`, `visibility.rs` | showdown traces; visibility tests | covered | Redacted by viewer authorization. |
| `RL-SHOW-ELIGIBLE-001` | Only live seats evaluated. | `showdown.rs` | foldout and showdown traces | covered-by-trace | Folded seats excluded. |
| `RL-SHOW-WINNER-001` | Strongest hand wins. | `showdown.rs`, `pot.rs` | high-card/pair/showdown traces; `showdown-seat-label-consistency.trace.json`; seed-10018 replay test | covered-by-trace | Rust computes outcome and public winner labels. |
| `RL-SHOW-SPLIT-001` | Tied best hands split. | `showdown.rs`, `pot.rs` | `split-pot-even.trace.json`; `split-winner-order-vs-remainder.trace.json`; split rules/replay tests | covered-by-trace | Canonical co-winner order is distinct from remainder order. |
| `RL-POT-REMAINDER-001` | Remainder by button order. | `pot.rs` | `split-pot-remainder-button-order.trace.json`; `split-winner-order-vs-remainder.trace.json`; pot tests | covered-by-trace | Public deterministic order only assigns odd units. |
| `RL-SHOW-FOLDOUT-001` | Foldout explanation. | `showdown.rs`, `visibility.rs` | foldout trace; no-leak tests | covered-by-trace | Folded private cards stay hidden. |
| `RL-VIS-PUBLIC-001` | Public facts only. | `visibility.rs`, `effects.rs` | public-observer-no-leak trace; visibility tests | covered | Public payload excludes hidden cards. |
| `RL-VIS-PRIVATE-HOLE-001` | Own hole cards only. | `visibility.rs` | seat-private-view trace; visibility tests | covered-by-trace | Owner authorization only. |
| `RL-VIS-OPPONENT-HOLE-001` | Opponent holes hidden. | `visibility.rs`, `effects.rs` | pairwise no-leak tests; no-leak traces | covered | Cross-seat leakage rejected. |
| `RL-VIS-DECKTAIL-001` | Deck tail/future board hidden. | `setup.rs`, `visibility.rs`, `replay_support.rs` | no-leak traces; replay export tests | covered | Public exports cannot reconstruct deck tail. |
| `RL-VIS-DIAGNOSTIC-001` | Safe diagnostics. | `actions.rs`, `rules.rs` | wrong-seat and cap diagnostic traces | covered-by-trace | Public facts only. |
| `RL-VIS-SHOWDOWN-001` | Authorized showdown reveal. | `showdown.rs`, `visibility.rs` | showdown traces; seed-10018/31 replay tests; visibility tests | covered | Folded data remains redacted. |
| `RL-VIS-FOLDOUT-001` | Foldout redaction. | `showdown.rs`, `visibility.rs` | foldout no-leak trace | covered-by-trace | No folded-card reveal. |
| `RL-VIS-VIEWHASH-001` | Viewer-scoped hashes. | `visibility.rs`, `replay_support.rs` | view-hash tests; replay-check | covered | Hashes cover authorized projection only. |
| `RL-REPLAY-RNG-001` | Seeded replay determinism. | `replay_support.rs`, `setup.rs` | replay tests; replay-check; simulator seeds | covered | No wall-clock/browser RNG. |
| `RL-REPLAY-HASH-001` | Stable hashes. | `replay_support.rs`, `state.rs` | golden traces; seed-10018/31 replay tests; replay-check | covered-by-trace | Hash drift caught by fixtures and locked regression summaries. |
| `RL-REPLAY-EXPORT-001` | Redacted public export. | `replay_support.rs`, `visibility.rs` | public-replay-export-import trace; no-leak tests | covered-by-trace | Viewer scoped. |
| `RL-REPLAY-IMPORT-001` | Import through Rust rules. | `replay_support.rs`, `rules.rs` | replay tests; replay-check | covered | Commands validate normally. |
| `RL-REPLAY-SERIAL-001` | Deterministic serialization order. | `state.rs`, `replay_support.rs` | serialization tests; `split-winner-order-vs-remainder.trace.json`; golden traces | covered | Canonical winners/allocations serialize in stable order. |
| `RL-BOT-LEGAL-001` | Legal-action-only bots. | `bots.rs`, `actions.rs` | bot tests; simulator; bot trace | covered | Bots submit normal commands. |
| `RL-BOT-L0-001` | Level 0 random legal. | `bots.rs` | bot tests; AI docs | covered | Deterministic seed tie handling. |
| `RL-BOT-L1-001` | Level 1 heuristics. | `bots.rs`, `AI.md` | bot tests; evidence docs | covered | Authorized inputs only. |
| `RL-BOT-L2-001` | Level 2 authored policy. | `bots.rs`, evidence pack | bot tests; simulator; benchmark full playout | covered | No MCTS/ML/RL/sampling. |
| `RL-BOT-EXPLAIN-001` | Viewer-safe bot explanations. | `bots.rs`, `visibility.rs`, WASM bridge, web renderer | bot no-leak tests; `smoke:ui`; a11y no-leak smoke | covered | Non-random bot why uses public facts only and is not routed through effects or replay. |
| `RL-UI-PRESENT-001` | UI presents Rust/WASM output. | `ui.rs`, WASM bridge, web renderer | wasm-api tests; `smoke:wasm`; `smoke:effects`; `smoke:e2e` | covered | Browser renders Rust-projected fields and Rust-authored copy; TypeScript adds layout only. |
| `RL-UI-SEATS-001` | UI seat metadata from Rust. | `ui.rs`, web renderer | River Ledger e2e seat/street assertions; `smoke:ui` | covered | Seat count, labels, roles, active/pending markers, street strip, and seat-ledger display consume Rust/WASM metadata. |
| `RL-UI-ACTIONS-001` | UI legal controls from Rust. | `ui.rs`, WASM legal tree, web action panel | legal-action tests; River Ledger e2e action metadata assertions; `smoke:ui` | covered | Relevant action helper rows, call price, added ledger units, and cap-left copy come from Rust legal-action metadata. |
| `RL-UI-PREVIEW-001` | Viewer-safe previews. | `ui.rs`, WASM/web | not applicable to shipped River Ledger surface | intentionally-deferred | River Ledger has no separate browser preview surface yet; future preview work must remain Rust-authored and viewer-safe. |
| `RL-UI-LEDGER-001` | Abstract ledger display. | `ui.rs`, web renderer | River Ledger e2e; public-copy audit | covered | Browser copy uses ledger/abstract units, not pot/chip/money/rake language, with labeled `This round` and `Hand total` fields. |
| `RL-UI-SHOWDOWN-001` | Rust-authored outcome. | `showdown.rs`, `visibility.rs`, `ui.rs`, WASM bridge, web renderer | showdown and visibility tests; `showdown-seat-label-consistency.trace.json`; wasm-api bridge tests; outcome and River Ledger e2e | covered | V2 banner, decisive contrast, ranked standings, folded rows, card-usage marks, hand names, and teaching aid are Rust-authored and reveal-scoped. |
| `RL-UI-NOCASINO-001` | No casino presentation. | docs, web renderer, catalog icon | player rules; public-copy audit; `smoke:e2e`; shell smoke | covered | Public River Ledger UI and catalog icon avoid casino trade dress and money/chip/payout/rake vocabulary. |
| `RL-UI-NOLEAK-001` | Browser no-leak. | visibility projection, WASM/web/e2e | visibility tests; River Ledger DOM/storage/console no-leak e2e; bot why no-leak smoke | covered | Observer and wrong-seat browser contexts contain no unauthorized private cards, hidden hand-strength facts, bot-private reasons, or raw seat IDs. |
| `RL-SETUP-AMB-001` | No heads-up official mode. | `setup.rs`, `ids.rs` | invalid-seat-count trace; setup tests | covered-by-trace | Official seats are 3-6. |
| `RL-DEAL-AMB-001` | Burn cards hidden. | `setup.rs`, `visibility.rs` | no-leak traces; visibility tests | covered | Burn advancement is internal only. |
| `RL-EVAL-AMB-001` | Suits do not break ties. | `evaluator.rs` | evaluator tests; split traces | covered | Tiebreak vector ignores suit. |
| `RL-POT-AMB-001` | Remainder allocation. | `pot.rs` | split-remainder trace | covered-by-trace | Button-order among tied winners. |
| `RL-VIS-AMB-001` | Public replay redaction scope. | `replay_support.rs`, `visibility.rs` | public replay export/import trace | covered-by-trace | Public export excludes seed and hidden facts. |
| `RL-VAR-SEATS-001` | Standard seat range. | `variants.rs`, `setup.rs` | variant tests; setup tests | covered | 3, 4, 5, 6 only. |
| `RL-VAR-LIMIT-001` | Fixed-limit variant. | `variants.rs`, `betting.rs` | variant tests; rule tests | covered | No no-limit behavior. |
| `RL-VAR-CAP-001` | Raise cap variant metadata. | `variants.rs`, `betting.rs` | variant tests; cap trace | covered-by-trace | Metadata mirrors Rust constants. |
| `RL-VAR-PRESENT-001` | Neutral presentation metadata. | `variants.rs`, `ui.rs`, docs | source docs; player docs | covered | Original Rulepath identity. |
| `RL-VAR-ALLIN-001` | No all-in variant. | `variants.rs`, `pot.rs` | variant tests; out-of-scope docs | covered | Side pots absent. |
| `RL-OOS-ALLIN-001` | All-in/side pots out of scope. | `pot.rs`, docs | pot tests; sources/admission docs | covered | Explicitly absent from model. |
| `RL-OOS-NOLIMIT-001` | No no-limit/pot-limit. | `betting.rs`, variants | rule tests; docs | covered | Fixed-limit only. |
| `RL-OOS-TOURNAMENT-001` | No tournament features. | docs, data | source/admission docs; boundary-check | covered | No lobby/payout/rake features. |
| `RL-OOS-BOT-001` | No advanced public bots. | `bots.rs`, AI docs | bot tests; evidence pack | covered | No MCTS/ISMCTS/Monte Carlo/ML/RL. |
| `RL-OOS-BROWSER-001` | No browser authority. | architecture docs, later WASM/web | WASM/web tickets | intentionally-deferred | Final browser enforcement lands in GAT15RIVLEDTEX-016/017/018. |
| `RL-OOS-ENGINE-001` | No engine-core game nouns. | crate boundaries | `bash scripts/boundary-check.sh`; workspace checks | covered | Game nouns remain in `games/river_ledger`. |

# Meldfall Ledger Primitive-Pressure Ledger

Game ID: `meldfall_ledger`

Variant: `classic_500_single_deck_v1`

Created: 2026-06-26

Last updated: 2026-06-26

## Decision summary

Mechanic shape: local rummy-family meld validation, public meld tableau,
draw/discard zones with multi-card pickup, lay-off onto any public meld,
multi-round cumulative scoring to 500, and deterministic shuffle/private-hand
redacted exports.

Status: `local-only` first-use entries plus existing deferred private-hand
review.

Decision: keep all Meldfall Ledger mechanics in `games/meldfall_ledger`. No
`game-stdlib` rummy helper, no `engine-core` card/meld/tableau/pile nouns, no
static-data behavior formula, and no promotion debt are authorized.

Review owner/date: Codex, 2026-06-26.

## First-use entries

### ML-PP-001 - Meld validation: sets and runs

| Field | Decision |
|---|---|
| Status | `local-only` first official use |
| Games exerting pressure | `meldfall_ledger` |
| Relevant files/docs | `games/meldfall_ledger/src/cards.rs`, `games/meldfall_ledger/src/rules.rs`, `games/meldfall_ledger/src/actions.rs`, `games/meldfall_ledger/docs/RULE-COVERAGE.md` |
| What is repeated | No prior official game has set/run meld validation. |
| What differs | Sets require same rank and distinct suits; runs require one suit, consecutive ranks, ace-low or ace-high but no wrap. |
| Decision | Keep local; hard-gate before a third close meld/tableau helper. |
| Why not `engine-core` | Rank, suit, set, run, meld, and card value are game nouns. |
| Why not `game-stdlib` | One official use cannot prove a shared helper boundary, and the legality shape is behavior-bearing. |
| Data/Rust boundary | Static data may name variants only; meld conditions remain typed Rust. |
| Replay/hash impact | Meld application mutates game-local state and traces only. No shared hash migration. |
| Visibility impact | Tabled melds become public; in-hand candidates stay viewer-private. |
| Bot/UI impact | Bots and UI consume Rust legal choices only. |
| Next review trigger | A second official meld game records comparison; a third close use blocks until a ledger decision. |

### ML-PP-002 - Public meld tableau and zone model

| Field | Decision |
|---|---|
| Status | `local-only` first official use |
| Games exerting pressure | `meldfall_ledger` |
| Relevant files/docs | `games/meldfall_ledger/src/state.rs`, `games/meldfall_ledger/src/visibility.rs`, `games/meldfall_ledger/docs/RULE-COVERAGE.md`, `apps/web/src/components/MeldfallLedgerBoard.tsx` |
| What is repeated | Public card zones exist in earlier games, but not public rummy meld groups with origin and per-card credit. |
| What differs | Meld groups have stable public ids, origin seats, ordered cards, and played-by credit owners. |
| Decision | Keep local; no generic tableau helper. |
| Why not `engine-core` | Tableau, meld group, origin, and score-credit semantics are game behavior. |
| Why not `game-stdlib` | The zone model is coupled to meld legality, lay-off, scoring, visibility, and UI. |
| Data/Rust boundary | Tableau structure and legal extensions remain Rust-owned. |
| Replay/hash impact | Public tableau order is deterministic game-local export state. |
| Visibility impact | Tabled cards are public; private hands and stock order remain redacted. |
| Bot/UI impact | UI renders Rust projection; bots choose from Rust legal leaves. |
| Next review trigger | Third close public meld/tableau/zone use. |

### ML-PP-003 - Draw/discard piles with multi-card pickup

| Field | Decision |
|---|---|
| Status | `local-only` first official use |
| Games exerting pressure | `meldfall_ledger` |
| Relevant files/docs | `games/meldfall_ledger/src/actions.rs`, `games/meldfall_ledger/src/rules.rs`, `games/meldfall_ledger/src/state.rs`, `games/meldfall_ledger/docs/SOURCES.md` |
| What is repeated | Earlier card games shuffle/deal and expose public cards, but do not support Rummy 500 discard-tail pickup. |
| What differs | Selecting any discard takes that card plus all newer cards, and the selected card must be used immediately. |
| Decision | Keep local; no draw/discard helper. |
| Why not `engine-core` | Stock, discard pile, pickup, and immediate-use commitment are game nouns and behavior. |
| Why not `game-stdlib` | The useful helper would need policy flags for ordered piles, commitment, diagnostics, effects, and scoring. |
| Data/Rust boundary | Discard-pickup legality and commitment enforcement remain typed Rust. |
| Replay/hash impact | Ordered discard pile and pickup effects remain game-local trace bytes. |
| Visibility impact | Discard pile is public; stock count is public; stock order and drawn stock card remain viewer-scoped. |
| Bot/UI impact | Browser surfaces public discard choices from Rust; bots do not infer hidden stock order. |
| Next review trigger | Third close draw/discard-zone use with pickup semantics. |

### ML-PP-004 - Lay-off onto any public meld

| Field | Decision |
|---|---|
| Status | `local-only` first official use |
| Games exerting pressure | `meldfall_ledger` |
| Relevant files/docs | `games/meldfall_ledger/src/rules.rs`, `games/meldfall_ledger/src/actions.rs`, `games/meldfall_ledger/src/scoring.rs`, `games/meldfall_ledger/docs/RULE-COVERAGE.md` |
| What is repeated | No earlier official game extends another player's public meld. |
| What differs | A laid-off card may extend any public meld while score credit goes to the laying-off seat. |
| Decision | Keep local; no lay-off/tableau-extension helper. |
| Why not `engine-core` | Lay-off legality, meld extension, and score-credit attribution are behavior. |
| Why not `game-stdlib` | Extension legality depends on the local meld validator and scoring model. |
| Data/Rust boundary | Static data cannot describe valid extensions or credit ownership. |
| Replay/hash impact | Lay-off effects and tabled-card order are deterministic game-local exports. |
| Visibility impact | Laid-off cards are public after application; candidates remain private until played. |
| Bot/UI impact | UI and bots use Rust legal leaves and Rust effects. |
| Next review trigger | Third close lay-off/tableau-extension use. |

### ML-PP-005 - Multi-round cumulative scoring to 500

| Field | Decision |
|---|---|
| Status | `local-only` first official use for this rummy scoring pattern |
| Games exerting pressure | `meldfall_ledger` |
| Relevant files/docs | `games/meldfall_ledger/src/scoring.rs`, `games/meldfall_ledger/src/state.rs`, `games/meldfall_ledger/docs/RULE-COVERAGE.md` |
| What is repeated | Prior games have scores and targets, but not positive tabled-card values plus in-hand penalties and target-tie continuation. |
| What differs | Scores can go negative, tabled cards score to played-by seats, in-hand cards penalize holders, and a tie at or above 500 continues. |
| Decision | Keep local; no cumulative-score helper. |
| Why not `engine-core` | Card values, round deltas, target, ranking, and tie policy are game behavior. |
| Why not `game-stdlib` | The pattern combines rummy-specific score credit, penalty, settlement visibility, and match continuation. |
| Data/Rust boundary | Card values and scoring formulas remain Rust code. |
| Replay/hash impact | Score ledger and terminal state remain game-local deterministic exports. |
| Visibility impact | Public settlement exposes totals/counts without unauthorized remaining-hand identities. |
| Bot/UI impact | Bots and UI consume Rust score deltas/outcomes. |
| Next review trigger | Third close rummy-style cumulative scoring target. |

### ML-PP-006 - Deterministic shuffle/private hand redacted export review

| Field | Decision |
|---|---|
| Status | reviewed against existing §10B deferred pressure; no new hard gate |
| Games exerting pressure | `high_card_duel`, `poker_lite`, `plain_tricks`, `masked_claims`, `river_ledger`, `blackglass_pact`, `meldfall_ledger` |
| Relevant files/docs | `games/meldfall_ledger/src/setup.rs`, `games/meldfall_ledger/src/visibility.rs`, `games/meldfall_ledger/src/replay_support.rs`, `docs/MECHANIC-ATLAS.md` |
| What is repeated | Deterministic shuffle, private hands, public observer redaction, and viewer-scoped exports. |
| What differs | Meldfall has no staged commitment reveal or showdown evaluator; public discard/tableau cards coexist with hidden hands and stock order. |
| Decision | Reuse existing generic contracts only; keep shuffle/deal/projection policy local. |
| Why not `engine-core` | Deck, hand, stock, discard, and redacted card projection are game nouns and hidden-info policy. |
| Why not `game-stdlib` | The repo already defer/rejects private-hand helper extraction; Meldfall adds no narrower proven boundary. |
| Data/Rust boundary | Shuffle/deal, visibility, and export scoping remain Rust-owned. |
| Replay/hash impact | No RNG, trace, export, or hash migration is authorized. |
| Visibility impact | Public exports hide every hand and stock order; seat-private exports reveal only the viewer's own hand. |
| Bot/UI impact | Bots use authorized views only; UI displays Rust/WASM projection only. |
| Next review trigger | Reopen per atlas §10B trigger if future games repeat enough evidence to justify another hard review. |

## Post-build evidence

| Evidence | Result |
|---|---|
| `docs/MECHANIC-ATLAS.md` | Records Meldfall first-use rows and keeps §10A open promotion debt empty. |
| `docs/MECHANICAL-SCAFFOLDING-REGISTER.md` | Records Gate 19 `forward-v1` no-new-scaffolding closeout. |
| `ci/scaffolding-audits.json` | Adds the `meldfall_ledger` forward-v1 receipt. |
| `games/meldfall_ledger/docs/RULE-COVERAGE.md` | Maps the behavior rows to Rust owners, traces, tests, and deferred surfaces. |

## Rejected alternatives

| Alternative | Why rejected |
|---|---|
| Promote a generic rummy meld helper | First use only; would encode set/run/ace/wrap and diagnostic policy. |
| Add stock/discard/tableau nouns to `engine-core` | Violates the noun-free kernel boundary. |
| Encode meld/scoring rules in TOML or JSON | Static data behavior is forbidden. |
| Treat public tableau transport as mechanical scaffolding | Meld groups, lay-off, and score credit decide game behavior and route through the mechanic atlas. |
| Let TypeScript build or filter legal melds/lay-offs | TypeScript legality authority is forbidden; Rust legal trees are authoritative. |

## Back-port or conformance plan

No helper is promoted, so no prior game requires back-porting. No promotion debt
is opened.

Affected prior games: not applicable.

Exceptions: not applicable.

Closure gate if debt is deferred: not applicable because there is no promotion
debt.

## Agent misuse risks

- Extracting a generic card, deck, hand, stock, discard, meld, or tableau helper
  from one rummy-family implementation.
- Treating ordered public discard data as permission to expose hidden stock
  order or opponent hand identities.
- Moving score formulas or meld predicates into static data.
- Broadening action-tree scaffolding into legality generation.
- Treating the public browser tableau as TypeScript authority for legal
  lay-offs or scoring.

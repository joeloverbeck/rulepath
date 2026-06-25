# Primitive Pressure Ledger: Blackglass Pact partnership trick contracts

Candidate name: `blackglass-pact-partnership-trick-contracts`

Status: pre-code decisions recorded; reuse promoted trick helpers unchanged;
numeric contract second use keep local; partnership/team first use local-only

Decision date: 2026-06-25

Prepared by: `Codex`

## Summary

Blackglass Pact reuses the already-promoted pure trick-taking helpers from
`game-stdlib::trick_taking` and keeps all game behavior local. It is the second
official close numeric trick-contract game after Vow Tide, but the shared shape
is not enough to promote a bid/contract/scoring helper. It is the first
official fixed competitive partnership/team scoring game, so partnership
behavior is recorded as `local-only`.

No helper is added to `engine-core` or `game-stdlib`. No promotion debt is
created.

## Decision 1: Promoted Trick Helper Reuse

| Factor | Finding |
|---|---|
| status | reuse existing promoted primitive |
| helpers | `game-stdlib::trick_taking::follow_suit_indices`; `game-stdlib::trick_taking::winning_play_index` |
| why they fit | Blackglass needs pure led-suit filtering and pure trump/led-suit winner selection. |
| caller-owned policy | phase, actor, ownership, broken-spades lead restriction, permanent spades trump selection, winner-to-seat mapping, effects, scoring, replay, visibility, bots, and UI. |
| why no third-use gate fires | Gate 17 already resolved the third-use hard gate and closed Plain Tricks/Briar Circuit/Vow Tide conformance. |
| tests required | GAT18BLAPACSPA-006 helper conformance tests and traces; GAT18BLAPACSPA-018 atlas closeout. |

## Decision 2: Numeric Trick Contract Second-Use Comparison

| Factor | Vow Tide | Blackglass Pact | Decision |
|---|---|---|---|
| bid owner | independent seat | seat inside fixed partnership | different |
| bid range | public exact bid over hand size with dealer hook | nil or 1-13; blind declarers skipped | different |
| hook | dealer cannot make total equal hand size | no total-13 hook | different |
| contract target | each seat's own exact bid | team sum of positive numeric partner bids | different |
| nil | not part of Vow's contract shape | individual nil and blind nil, scored separately | different |
| scoring | exact made/missed seat addition | team ordinary +/-10x, nil deltas, bag points, bag penalties | different |
| persistence | per-hand exact score additions | cumulative score plus persistent team bags | different |
| visibility | public bids plus hidden hands/stock | public bids plus hidden hands and partner no-leak | similar hidden-info pressure, different policy |

Decision: `local-only` second-use comparison. The next review trigger is a
third close official numeric trick-contract game whose bid/contract/scoring
shape can be compared against both Vow Tide and Blackglass Pact.

## Numeric Contract Boundary Decision

| Factor | Finding |
|---|---|
| use count | second close official use |
| decision | `local-only`; no promotion |
| why not `engine-core` | Bid, contract, nil, blind nil, trick count, partner, team, bag, and score are game/mechanic nouns. |
| why not `game-stdlib` | The two implementations differ in ownership, hook policy, aggregation, nil handling, scoring, persistence, visibility, effects, bots, and UI. A helper would either be trivial or behavior-bearing. |
| data/Rust boundary | Static data may name variant identity and presentation only. Bid legality, contracts, nils, bags, scoring, effects, bots, and views stay typed Rust. |
| replay/hash impact | Accepted bids and scored hands mutate deterministic Rust state. No unsubmitted UI choice or static formula is authoritative state. |
| visibility impact | Accepted bids, contracts, scores, and bags are public. Private hands, future deal material, blind-phase absence of cards, action trees, previews, and bot features remain viewer-scoped. |
| bot/UI impact | Bots and TypeScript consume Rust legal leaves and Rust-derived score/contract/outcome fields; neither may compute bid legality or scoring. |

## Decision 3: Partnership/Team First Use

| Factor | Finding |
|---|---|
| use count | first official fixed competitive partnership/team scoring game |
| decision | `local-only` |
| shape | stable public `team_0`/`team_1`; fixed opposite seats; ordinary bid aggregation; individual nil/blind nil inside team scoring; persistent team bags; team standings/outcomes; explicit partner no-leak. |
| why not `engine-core` | Team, partnership, bid aggregation, shared score, and partner visibility are game behavior and domain nouns. |
| why not `game-stdlib` | First use cannot justify promotion, and the shape is policy-heavy across scoring, visibility, bots, and UI. |
| relation to Flood Watch | Flood Watch is cooperative all-win/all-lose; Blackglass is competitive two-team scoring. Not the same shared-outcome shape. |
| next review trigger | a second official competitive partnership/team scoring game, with hard gate before a third close team-helper proposal. |

## Tests Required

| Decision | Required evidence |
|---|---|
| trick helper reuse | helper conformance tests, follow-suit and comparator traces, no helper signature change, atlas closeout |
| numeric keep-local | bid/contract/scoring unit and property tests, Vow Tide contrast regression, rule coverage rows |
| partnership first-use | setup/team mapping tests, partner no-leak rows, team score/outcome snapshots, UI grouping smoke |
| no promotion debt | `docs/MECHANIC-ATLAS.md` §10A remains empty at closeout |

## Rejected Alternatives

| Alternative | Why rejected |
|---|---|
| Promote a generic bid/contract helper now | Second use differs materially and would encode behavior. |
| Add team/partnership semantics to seat identity | Violates the seat/team boundary and risks partner-hand leakage. |
| Extend `game-stdlib::trick_taking` for broken spades, teams, or scoring | The promoted helpers are intentionally pure and behavior-free. |
| Encode scoring formulas in TOML/JSON | Static data behavior is forbidden. |
| Let UI sum bids, bags, or determine winner | TypeScript legality/scoring/outcome authority is forbidden. |

## Next Review Triggers

- Reopen numeric trick-contract pressure before a third close official numeric
  contract-vs-result game.
- Reopen partnership/team pressure when a second competitive partnership/team
  scoring game appears.
- Reopen trick helper conformance only if implementation discovers a real
  mismatch with the promoted pure helper boundary.
- Any proposal to migrate trace/hash/visibility behavior requires explicit
  ADR 0009 authority or a later accepted ADR naming the surface.

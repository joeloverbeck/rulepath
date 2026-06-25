# Blackglass Pact Mechanics Inventory

Game ID: `blackglass_pact`

Roadmap stage/gate: Stage 18 / Gate 18 partnership trick-taking proof

Rules version: `blackglass-pact-rules-v1`

Prepared by: `Codex`

Last updated: 2026-06-25

Evidence receipt: [GAME-EVIDENCE.md](GAME-EVIDENCE.md)

## Purpose

This inventory records Blackglass Pact's game-local mechanic shapes, promoted
primitive reuse, primitive-pressure posture, and the pre-code mechanical
scaffolding reuse-first audit required for the first `forward-v1` official
game. It is evidence for `docs/MECHANIC-ATLAS.md` and
`docs/MECHANICAL-SCAFFOLDING-REGISTER.md`, not permission to generalize.

Blackglass Pact is a fixed-four hidden-hand partnership trick-taking game.
Rust owns setup, blind-nil commitment, shuffle/deal, bidding, follow-suit
legality, spades-trump comparison, trick capture, team scoring, bags, terminal
outcomes, visibility, effects, replay, bots, and benchmark evidence.
TypeScript presents Rust/WASM output only.

## Mechanic Inventory

| Category | Game-local description | Evidence in rules | Current status | Notes |
|---|---|---|---|---|
| N-seat model | Exactly four stable seats, public observer, four seat-private viewers, fixed North/East/South/West labels. | `BP-SETUP-001`, `BP-SETUP-002`, `BP-VIS-*` | `local-only` | Reuse canonical seat grammar/scaffolding, but setup admission and labels stay game-local. |
| turn-order policy | Dealer rotates clockwise; blind and bid order start left of dealer; first leader is left of dealer; trick winner leads next. | `BP-SETUP-005`, `BP-BLIND-002`, `BP-BID-001`, `BP-PLAY-001`, `BP-PLAY-009` | `rejected/deferred with rationale` | Winner-leads and dealer/bid/blind order remain behavioral and local. |
| team/partnership/coalition | Fixed competitive partnerships: `team_0` North-South and `team_1` East-West. | `BP-SETUP-003`, `BP-SETUP-004` | `local-only` | First official competitive-team use; no generic team helper. |
| topology/spatial model | No board topology; public layout is seat ring plus team grouping. | `BP-UI-001` | `local-only` | No graph, grid, route, region, or movement primitive. |
| graph/track/topology size | Not applicable: no graph/track surface. | out-of-scope table in `RULES.md` | `rejected/deferred with rationale` | Seat ring is turn/order policy, not board topology. |
| component/zone model | Standard 52-card deck, private hands, public current trick/completed tricks, bids, scores, bags, and hand history. | `BP-DEAL-*`, `BP-BID-*`, `BP-PLAY-*`, `BP-SCORE-*` | `repeated-shape candidate` | Card/deck/hand behavior stays game-local except promoted pure trick helpers. |
| hidden-hand/deck/wall model | Own hand is seat-private; other hands, future deck, RNG material, private action trees, and bot candidates stay hidden. | `BP-DEAL-004`, `BP-VIS-*` | `repeated-shape candidate` | Uses the shared no-leak harness shape as dev-only scaffolding; policy remains game-owned. |
| action shape | Flat phase-specific leaves: `blind_nil/*`, `bid/*`, `play/<card-id>`. | `BP-BLIND-003`, `BP-BID-003`, `BP-PLAY-*` | `local-only` | Action-tree encoding scaffolding may be reused; leaves/legality stay local. |
| turn/phase model | Blind-nil commitment, bidding, playing trick, hand scoring/advance, terminal. | `BP-BLIND-*`, `BP-BID-*`, `BP-PLAY-*`, `BP-END-*` | `local-only` | No generic phase machine. |
| randomness/chance | Seeded shuffle/deal after blind commitments; blind decisions cannot perturb deal bytes. | `BP-BLIND-007`, `BP-DEAL-002`, `BP-DEAL-006` | `local-only` | RNG helpers may be reused for bounded sampling if parity is proven; shuffle policy stays local. |
| visibility/hidden information | Public teams/bids/plays/scores; owner-private hands and controls; no partner-hand visibility. | `BP-VIS-*` | `repeated-shape candidate` | Pairwise proof spans observer and every ordered seat pair. |
| resource/accounting | Team scores, bags, ordinary contracts, nil/blind nil deltas, bag penalties. | `BP-SCORE-*`, `BP-END-*` | `local-only` | Scoring/accounting semantics are behavior and stay local. |
| shared accounting/side-pot/split allocation | Team scoring and bags only; no pot, side pot, market, or resource allocation. | `BP-SCORE-*` | `rejected/deferred with rationale` | Team scoring is not River Ledger-style allocation. |
| movement/capture/placement | Played cards enter the public trick; trick winner captures the trick into history. | `BP-PLAY-*` | `local-only` | No board movement or conversion. |
| pattern/line/directional scanning | Not applicable. | out-of-scope table in `RULES.md` | `rejected/deferred with rationale` | No line/ray/pattern mechanic. |
| commitment/reveal | Public blind-nil decision before deal; public sequential bids; played cards reveal as public trick history. | `BP-BLIND-*`, `BP-BID-*`, `BP-PLAY-*` | `local-only` | Blind commitment is not a hidden simultaneous reveal. |
| reaction/window/pending response | Not applicable. | out-of-scope table in `RULES.md` | `rejected/deferred with rationale` | No interrupts or responder windows. |
| scoring/outcome | Team-keyed cumulative score, bag rollover, exact-tie continuation, stable team/seat standings. | `BP-SCORE-*`, `BP-END-*` | `local-only` | Competitive team outcome is first official use. |
| evaluator/showdown/ranking | No card-hand evaluator; outcome ranks team scores and seat/team contributions. | `BP-END-009`, `BP-END-010` | `local-only` | TypeScript does not infer winner/rank. |
| semantic effect shape | Blind decision, deal, bid, spades broken, card played, trick captured, hand scored, bag penalty, dealer advanced, match complete. | `BP-BLIND-*`, `BP-SCORE-015`, `BP-UI-*` | `local-only` | Effect-envelope constructors are scaffolding only; effect meaning stays local. |
| UI interaction pattern | Grouped partnership table, blind/bid/card controls from Rust leaves, observer/replay/outcome surfaces. | `BP-UI-*` | `local-only` | Shared renderer scaffolding may carry data; renderer cannot decide legality/scoring. |
| bot policy pattern | L0 random legal; bounded L1 with public facts, own hand after deal, lawful public deductions, and no hidden-world sampling. | `BP-BOT-*` | `local-only` | No L2, L3, MCTS, ISMCTS, Monte Carlo, ML, RL, or runtime LLM. |
| benchmark/performance pressure | Setup/deal, legal trees, validation/apply, projections, effects, replay/export/import, bots, full matches. | `BP-REPLAY-*`, `BP-BOT-*` | `local-only` | Benchmarks are evidence, not extraction authority. |

## Repeated-Shape Comparison

| Shape | Classification | Governing owner | Already appears in | Same shape? | Similarities | Differences | Required next step |
|---|---|---|---|---:|---|---|---|
| follow-suit legality | behavioral mechanic with promoted pure helper | mechanic atlas / `game-stdlib::trick_taking` | `plain_tricks`, `briar_circuit`, `vow_tide` | yes for pure held-index filter | Led suit restricts followers when able. | Blackglass adds spades-broken lead policy, fixed teams, nil/bags, and partnership scoring. | Reuse `follow_suit_indices` unchanged; keep policy local. |
| trick comparator | behavioral mechanic with promoted pure helper | mechanic atlas / `game-stdlib::trick_taking` | `plain_tricks`, `briar_circuit`, `vow_tide` | yes for winner index | Highest eligible led/trump card wins. | Blackglass uses permanent spades trump; team scoring/outcome remain local. | Reuse `winning_play_index` with `Some(Spades)` unchanged. |
| numeric trick contract | behavioral mechanic | primitive-pressure ledger / mechanic atlas | `vow_tide` | related, not identical | Public numeric commitments later compare to trick count. | Vow is per-seat exact bid with dealer hook; Blackglass aggregates positive partner bids and has nil/blind nil/bags. | Record second-use comparison; keep local; next trigger is third close numeric trick-contract game. |
| fixed competitive partnerships | behavioral mechanic | primitive-pressure ledger / mechanic atlas | none official | first use | Team identity and grouped outcome. | Not Flood Watch cooperation; not River Ledger seat-independent standings. | Record first-use `local-only`; no generic team helper. |
| hidden hands and pairwise no-leak | behavior plus dev/test scaffolding | foundations + MSC C-07 | many hidden-info games | similar proof geometry | Owner-private components and unauthorized redaction. | Partnership explicitly does not grant private hand visibility; blind phase has no cards yet. | Reuse no-leak assertion geometry; keep projection policy local. |
| evidence/profile receipts | test scaffolding | MSC C-08 | recent official games | yes for profile shape | Replay/export/setup/domain evidence needs profile metadata. | Blackglass still delegates all validation to game/tool code. | Reuse drivers where applicable in later tickets. |

## Mechanical Scaffolding Reuse-First Audit

Pre-implementation checkpoint for `blackglass_pact`, Gate 18, coverage
`forward-v1`. The machine receipt is intentionally deferred to
GAT18BLAPACSPA-018; this section is the doc-side admission predicate.

| Planned surface | Existing MSC entry/shared symbol reviewed | Decision | Why the accepted boundary fits or does not fit | New register entry needed? | Earlier official-game matches | Expected follow-on unit or accepted no-unit disposition | Hash/visibility/determinism expectation |
|---|---|---|---|---:|---|---|---|
| semantic effect envelopes | `MSC-8C-001`; `EffectEnvelope<T>` public/seat-private constructors | reuse expected | Constructors carry caller-supplied payload plus existing visibility scope; they do not define reveal policy or effect meaning. | no | prior effect-envelope adopters | no follow-on expected unless implementation invents a new behavior-free effect shape | effect order/payload/visibility unchanged versus local literals |
| seat identity strings and WASM import boundary | `MSC-8C-002`; strict canonical `seat_<n>` grammar and WASM import aliases | reuse expected | Four stable seats fit canonical grammar; team labels remain game-local and must not enter the seat helper. | no | all canonical-seat adopters | no follow-on expected; non-seat team IDs stay local | canonical seat bytes; no hidden-info impact; no alias output migration without ADR 0009 |
| fixed-four setup count and clockwise index arithmetic | `MSC-8C-003`; `game-stdlib::seat` count/ring helpers | reuse expected with local policy | Count/ring helpers can validate structure and wrap indices, but dealer, blind, bid, partnership, and leader policy stay Blackglass-owned. | no | fixed-four/ring adopters such as Briar Circuit | no follow-on expected if helper is used only structurally | setup diagnostics and replay bytes must remain stable once pinned |
| action-tree encoding/hash v1 | `MSC-8C-004`; versioned action-tree encoding/hash | reuse expected | Blackglass has phase-specific action trees; v1 encoding may supply parallel evidence only and must not generate legality. | no | action-tree v1 adopters | no follow-on expected; no authority flip without ADR 0009 | parallel bytes only unless a later ticket names migration authority |
| stable-byte writer v1 | `MSC-8C-005`; `StableBytesWriter` | reuse expected for named evidence surfaces only | Writer frames caller-supplied bytes; it does not decide state meaning, hashes, visibility, or ordering beyond explicit fields. | no | stable-byte/action-tree adopters | no follow-on expected unless a new canonical-byte surface is introduced | no broad state/effect/view/replay authority flip in this ticket |
| dev-only test-support crate | `MSC-8C-006`; `game-test-support` | reuse expected as dev-dependency only | Hidden-info/no-leak/profile tests can use dev-only helpers; production crates, WASM, and tools may not gain normal/build edges. | no | recent hidden-info/profile adopters | no follow-on expected; boundary check must guard production edge | no runtime hash/visibility impact |
| pairwise no-leak matrix | `MSC-8C-007`; source-seat x viewer x surface matrix | reuse expected | Proof geometry fits observer plus four seat viewers, including partner rows; Blackglass supplies canaries and authorization expectations. | no | hidden-info games including River/Briar/Vow | no follow-on expected; policy remains game-owned | deterministic matrix enumeration; no persistent canary artifacts |
| evidence-profile drivers | `MSC-8C-008`; replay/public-export/seat-private/setup/domain profile drivers | reuse expected where profiles apply | Drivers validate profile shape and metadata while delegating setup, commands, projection, import/export, scoring, and domain checks to game/tool code. | no | profile-driver adopters | no follow-on expected unless a new profile type is invented | metadata checks only; canonical-byte authority named per artifact |
| bounded-index sampling | `MSC-8C-009`; legacy `next_index` and `next_index_unbiased_v1` | reuse expected only with parity evidence | Any bounded sampler use must preserve or explicitly version RNG word consumption; shuffle/deal policy remains game-local. | no | RNG sampler adopters | no follow-on expected unless an RNG migration is proposed | no RNG/hash migration without explicit evidence/authority |
| behavioral policy bundle | `MSC-8C-010`; Non-Promotion List rejection/local-only | rejected-rerouted | Deal, reveal, projection, bidding, trick lifecycle, teams, scoring, outcome, bots, and UI policy are behavior, not scaffolding. | no | all games with local behavior | accepted no-unit disposition: keep these behaviors local under this ledger/atlas; revisit via mechanic atlas, not register | no shared behavior extraction; no visibility or determinism policy migration |

## Second-Use Note

| Shape | First game | Second game | Candidate? | Rationale | Ledger/atlas update needed? |
|---|---|---|---:|---|---:|
| numeric trick contract | `vow_tide` | `blackglass_pact` | yes | Both use public numeric commitments judged against tricks, but Blackglass uses team aggregation, nil/blind nil, and bags instead of Vow's per-seat exact bid and dealer hook. | yes: game ledger now, repo atlas in GAT18BLAPACSPA-018 |
| competitive fixed partnership/team outcome | none official | `blackglass_pact` | no, first use | First official team/partnership use; implement locally. | yes: local ledger now, repo atlas in GAT18BLAPACSPA-018 |

## Third-Use Hard-Gate Warning

| Shape | Games exerting pressure | Third-use? | Gate cleared? | Evidence |
|---|---|---:|---:|---|
| follow-suit legality | `plain_tricks`, `briar_circuit`, `vow_tide`; Blackglass reuses helper | yes, already cleared | yes | `docs/MECHANIC-ATLAS.md` promoted `follow_suit_indices`; no open debt. |
| trick comparator | `plain_tricks`, `briar_circuit`, `vow_tide`; Blackglass reuses helper | yes, already cleared | yes | `docs/MECHANIC-ATLAS.md` promoted `winning_play_index`; no open debt. |
| numeric trick contract | `vow_tide`, `blackglass_pact` | no | not applicable | Second-use comparison recorded here and in `PRIMITIVE-PRESSURE-LEDGER.md`. |
| partnership/team scoring | `blackglass_pact` | no | not applicable | First-use `local-only` ledger entry. |

## Primitives Reused

| Primitive | Source | Why reused | Rule IDs covered | Tests proving compatibility | Notes |
|---|---|---|---|---|---|
| `follow_suit_indices` | `game-stdlib::trick_taking` | Pure led-suit held-index filtering fits Blackglass follow-suit after local ownership/phase/actor checks. | `BP-PLAY-005`, `BP-PLAY-006`, `BP-PLAY-011` | planned helper conformance tests and rule traces in GAT18BLAPACSPA-006 | No broken-spades, team, scoring, or visibility policy enters the helper. |
| `winning_play_index` | `game-stdlib::trick_taking` | Pure comparator fits permanent spades trump when caller passes `Some(Spades)`. | `BP-PLAY-007`, `BP-PLAY-008`, `BP-PLAY-012` | planned comparator conformance tests and rule traces in GAT18BLAPACSPA-006 | Caller maps winning play to seat/team locally. |

## Local Mechanics

| Local mechanic | Why local | Extraction risk | Rule IDs | Tests/traces | Notes |
|---|---|---|---|---|---|
| fixed partnerships and team IDs | First official competitive-team shape; team policy is behavior. | high | `BP-SETUP-003`, `BP-SETUP-004` | setup/team traces | Must not enter generic seat identity. |
| blind-nil commitment | Timing, eligibility, skipped bid, no-leak, and RNG independence are game rules. | high | `BP-BLIND-*` | blind traces/no-leak/property tests | Pre-deal commitment is a Gate 18 proof point. |
| numeric/team contracts, nils, bags | Second-use comparison differs materially from Vow Tide. | high | `BP-BID-*`, `BP-SCORE-*` | contract/scoring traces/properties | Keep local; next close game reopens. |
| broken-spades lead policy | Lead restriction and break timing are Spades-specific. | medium | `BP-PLAY-002` through `BP-PLAY-004` | rule/property traces | Not part of promoted helpers. |
| terminal/tie outcome | Team target, exact-tie continuation, and standings are game behavior. | medium | `BP-END-*` | terminal traces | TypeScript renders Rust outcome only. |

## Required Repo Atlas/Register Update

| Update target | Required? | Reason | Owner |
|---|---:|---|---|
| `docs/MECHANIC-ATLAS.md` | yes | Gate 18 must append helper reuse, numeric second-use keep-local, and partnership/team first-use local-only rows while keeping §10A empty. | GAT18BLAPACSPA-018 |
| `PRIMITIVE-PRESSURE-LEDGER.md` | yes | This game-local ledger records the pre-code trick/numeric/team decisions. | GAT18BLAPACSPA-002 and finalized by GAT18BLAPACSPA-018 |
| `docs/MECHANICAL-SCAFFOLDING-REGISTER.md` | yes at closeout | Post-build evidence must record reuse/no-new-scaffolding or any actual new behavior-free shape. | GAT18BLAPACSPA-018 |
| `specs/README.md` follow-on unit | not expected pre-code | No prior-game scaffolding refactor is identified pre-code; ticket 018 must queue or dispose any actual prior match. | GAT18BLAPACSPA-018 |
| ADR | no | No kernel, DSL, YAML, trace/hash, visibility, or architecture exception is admitted. | not applicable |

## Review Checklist

- `engine-core` remains noun-free.
- Static data remains typed content/parameters/metadata/fixtures/traces/reports only.
- TypeScript presents Rust/WASM payloads only.
- Reused trick helpers stay behavior-free and unchanged.
- Teams, bidding, scoring, visibility, bots, and UI policy stay game-local.
- The forward-v1 C-01 through C-10 reuse-first audit is complete pre-code.
- Machine `ci/scaffolding-audits.json` receipt is intentionally deferred to closeout ticket GAT18BLAPACSPA-018.

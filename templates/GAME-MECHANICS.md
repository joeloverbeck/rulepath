# <game_id> Mechanics Inventory

Game ID: `<game_id>`

Roadmap stage/gate: `<stage_or_gate>`

Rules version: `<rules_version>`

Prepared by: `<name/agent>`

Last updated: YYYY-MM-DD

Evidence receipt: [`GAME-EVIDENCE.md`](GAME-EVIDENCE.md)

Template realignment mapping: report `B-06 -> GAME-MECHANICS.md`. This template
owns game-local mechanic classification and mechanic/scaffolding pressure
analysis. `GAME-EVIDENCE.md` owns the conformance receipt links for mechanic and
scaffolding decisions.

## Purpose

This inventory applies `docs/MECHANIC-ATLAS.md` to one game. It records game-local mechanic shapes, primitive reuse, repeated-shape pressure, extraction/defer decisions, and effects/UI/bot/benchmark implications.

The mechanic inventory is evidence. It is not permission to generalize. Shared law belongs in the foundation docs and repo-level mechanic atlas.

## Status labels

Use only these status labels:

- `local-only`
- `repeated-shape candidate`
- `extraction required`
- `promoted primitive`
- `rejected/deferred with rationale`
- `ADR-required`

## Mechanic inventory categories

| Category | Game-local description | Evidence in rules | Current status | Notes |
|---|---|---|---|---|
| N-seat model | `<min/max seats, seat IDs, viewer classes, role/team assignment>` | `<rule_ids>` | `<status>` | `<notes>` |
| turn-order policy | `<fixed rotation/leader/dealer/priority/simultaneous/reaction/passed/eliminated seats>` | `<rule_ids>` | `<status>` | `<notes>` |
| team/partnership/coalition | `<solo/team/partnership/temporary coalition/asymmetric roles/none>` | `<rule_ids>` | `<status>` | `<notes>` |
| topology/spatial model | `<positions/tracks/maps/graphs/routes/regions/none>` | `<rule_ids>` | `<status>` | `<notes>` |
| graph/track/topology size | `<object count, branch factor, map bounds, generated/static topology, scaling pressure>` | `<rule_ids>` | `<status>` | `<notes>` |
| component/zone model | `<components/zones/decks/hands/public-private areas>` | `<rule_ids>` | `<status>` | `<notes>` |
| hidden-hand/deck/wall model | `<private hand/deck/wall/bag/stock/discard visibility and ownership>` | `<rule_ids>` | `<status>` | `<notes>` |
| action shape | `<flat/action tree/progressive/simultaneous/reaction/forced>` | `<rule_ids>` | `<status>` | `<notes>` |
| turn/phase model | `<turns/rounds/phases/tricks/reactions/cleanup/events>` | `<rule_ids>` | `<status>` | `<notes>` |
| randomness/chance | `<none/setup shuffle/draw/event sample/etc.>` | `<rule_ids>` | `<status>` | `<notes>` |
| visibility/hidden information | `<perfect/private hands/commitments/roles/redacted logs>` | `<rule_ids>` | `<status>` | `<notes>` |
| resource/accounting | `<counters/payments/scores/pots/budgets/conservation/debts>` | `<rule_ids>` | `<status>` | `<notes>` |
| shared accounting/side-pot/split allocation | `<shared pool/pot/side pot/team score/split allocation/remainder rule/none>` | `<rule_ids>` | `<status>` | `<notes>` |
| movement/capture/placement | `<placement/removal/movement paths/capture/conversion/promotion>` | `<rule_ids>` | `<status>` | `<notes>` |
| pattern/line/directional scanning | `<alignment/scanning rays/neighborhood/pattern/bracketed flips>` | `<rule_ids>` | `<status>` | `<notes>` |
| commitment/reveal | `<secret choices/simultaneous selection/reveal timing/redaction/waiting>` | `<rule_ids>` | `<status>` | `<notes>` |
| reaction/window/pending response | `<responders/priority/cancellation/replacement/forced windows>` | `<rule_ids>` | `<status>` | `<notes>` |
| reaction/simultaneous windows | `<pending responder set, response order, timeout/non-response policy, reveal barrier>` | `<rule_ids>` | `<status>` | `<notes>` |
| scoring/outcome | `<instant win/score totals/shared outcome/asymmetric victory/tie>` | `<rule_ids>` | `<status>` | `<notes>` |
| evaluator/showdown/ranking | `<hand evaluator/standing comparator/pairwise ranking/table-wide ranking/no evaluator>` | `<rule_ids>` | `<status>` | `<notes>` |
| semantic effect shape | `<effects needed for logs/animation/replay/bots/explanations>` | `<rule_ids>` | `<status>` | `<notes>` |
| UI interaction pattern | `<direct selection/progressive construction/drag optionality/previews/confirmations/replay>` | `<rule_ids>` | `<status>` | `<notes>` |
| bot policy pattern | `<random/baseline/authored priorities/search/belief model>` | `<rule_ids>` | `<status>` | `<notes>` |
| benchmark/performance pressure | `<hot paths/action branching/playout/serialization/replay>` | `<rule_ids>` | `<status>` | `<notes>` |

## Repeated-shape comparison

| Shape | Classification | Governing owner | Already appears in | Same shape? | Similarities | Differences | Required next step |
|---|---|---|---|---:|---|---|---|
| `<shape>` | behavioral mechanic / production mechanical scaffolding / test scaffolding / presentation scaffolding / superficial similarity | mechanic atlas / mechanical scaffolding register / local-only / ADR | `<game_ids>` | yes/no/unclear | `<similarities>` | `<differences>` | none / compare / ledger / register / reuse / promote / defer / ADR |

## Second-use note

If this is the second official game with a similar mechanic shape, the shape MUST be recorded as a `repeated-shape candidate` unless the comparison rejects the similarity with rationale.

| Shape | First game | Second game | Candidate? | Rationale | Ledger/atlas update needed? |
|---|---|---|---:|---|---:|
| `<shape>` | `<game_id>` | `<game_id>` | yes/no | `<rationale>` | yes/no |

## Third-use hard-gate warning

A third official game with the same mechanic shape is blocked until the primitive-pressure ledger records one of:

- reuse of an existing primitive;
- promotion to a narrow typed primitive;
- explicit defer/reject with rationale;
- `ADR-required`.

| Shape | Games exerting pressure | Third-use? | Gate cleared? | Evidence |
|---|---|---:|---:|---|
| `<shape>` | `<games>` | yes/no | yes/no/not applicable | `<ledger/atlas path>` |

## Primitives reused

| Primitive | Source | Why reused | Rule IDs covered | Tests proving compatibility | Notes |
|---|---|---|---|---|---|
| `<primitive>` | `game-stdlib` / local helper | `<reason>` | `<rule_ids>` | `<tests/traces>` | `<notes>` |

## Local mechanics

| Local mechanic | Why local | Extraction risk | Rule IDs | Tests/traces | Notes |
|---|---|---|---|---|---|
| `<mechanic>` | `<reason>` | low / medium / high | `<rule_ids>` | `<tests/traces>` | `<notes>` |

## Primitive candidates

| Candidate | Status | Games exerting pressure | Required next step | Blocker? |
|---|---|---|---|---:|
| `<candidate>` | local-only / repeated-shape candidate / extraction required / promoted primitive / rejected/deferred with rationale / ADR-required | `<games>` | `<next_step>` | yes/no |

## Extraction or defer rationale

For every repeated shape, explain why it is staying local, being reused, being promoted, being deferred/rejected, or requiring ADR.

| Shape | Decision | Rationale | Back-port needed? | Trace impact | Benchmark impact |
|---|---|---|---:|---|---|
| `<shape>` | local / reuse / promote / defer / reject / ADR-required | `<rationale>` | yes/no | preserve / update with rationale / none | `<impact>` |

## Effects, UI, and bot notes

Cross-template conformance status for effects, UI, bot, visibility, and
benchmark evidence lives in `GAME-EVIDENCE.md`. Keep only mechanic implications
and rule links here.

| Area | Requirement | Rule IDs | Notes |
|---|---|---|---|
| semantic effects | `<effects needed for log/animation/replay>` | `<rule_ids>` | `<notes>` |
| UI interaction pattern | `<direct/progressive/preview/confirm/replay>` | `<rule_ids>` | `<notes>` |
| Rust-generated previews | `<preview needs>` | `<rule_ids>` | `<notes>` |
| bot policy pattern | `<candidate extraction/priorities/search/belief>` | `<rule_ids>` | `<notes>` |
| visibility/no-leak | `<surfaces>` | `<rule_ids>` | `<notes>` |
| benchmark pressure | `<hot paths>` | `<rule_ids>` | `<notes>` |

## Required repo atlas update

Update `docs/MECHANIC-ATLAS.md` and the relevant `PRIMITIVE-PRESSURE-LEDGER.md` instance when:

- this game repeats a mechanic shape from another official game;
- this game is the third official game with the same mechanic shape;
- a primitive is reused, promoted, rejected, or deferred;
- trace preservation or intentional trace migration changes primitive evidence;
- benchmarks reveal mechanic-level pressure;
- examples or anti-examples become clearer.

| Update target | Required? | Reason | Owner |
|---|---:|---|---|
| `docs/MECHANIC-ATLAS.md` | yes/no | `<reason>` | `<owner>` |
| `PRIMITIVE-PRESSURE-LEDGER.md` | yes/no | `<reason>` | `<owner>` |
| ADR | yes/no | `<reason>` | `<owner>` |

## Anti-patterns

The following are forbidden:

- generalizing from one game;
- promoting game nouns into `engine-core`;
- creating a universal behavior language;
- extracting speculative private-stress-test helpers;
- allowing agents to “clean up” mechanics without the ledger;
- hiding rule behavior in static data;
- adding YAML for convenience;
- using mechanic tags as a substitute for rule coverage;
- treating a UI pattern as proof of rule correctness.

## Review checklist

- All mechanic atlas categories are filled.
- Every `not applicable` has a rationale.
- Repeated shapes are compared honestly.
- Second-use candidate status is recorded when applicable.
- Third-use hard gate is explicit and cleared or blocked.
- `engine-core` remains noun-free.
- Static data remains typed content/parameters/metadata/fixtures/traces/reports only.
- Effects, UI, bot, visibility, and benchmark impacts are recorded.
- Cross-template mechanic/scaffolding decision status links from `GAME-EVIDENCE.md`.
- Repo atlas and primitive-pressure ledger updates are identified.

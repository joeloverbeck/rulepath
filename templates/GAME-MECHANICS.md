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
| card-driven initiative/eligibility | `<event/card/round timing that affects actor order, eligibility, or pending responders; Rust-owned behavior only>` | `<rule_ids>` | `<status>` | `<notes>` |
| asymmetric faction menus | `<per-faction/role operation menus, special activities, local restrictions, or none>` | `<rule_ids>` | `<status>` | `<notes>` |
| operation and special-activity coupling | `<combine/chain/interrupt/forbid relationships; legality owner>` | `<rule_ids>` | `<status>` | `<notes>` |
| periodic upkeep/propaganda | `<recurring reset/score/reveal/pay/reshuffle/victory phases>` | `<rule_ids>` | `<status>` | `<notes>` |
| conditional event branches | `<branch owner, target choice owner, Rust function/match/trait owner; no static selectors>` | `<rule_ids>` | `<status>` | `<notes>` |
| persistent/temporary effects | `<duration, expiry, suspension, rule override owner, replay/hash impact>` | `<rule_ids>` | `<status>` | `<notes>` |
| faction-specific victory tracks | `<per-faction/role outcome checks, explanation payloads, tie/kingmaking risk>` | `<rule_ids>` | `<status>` | `<notes>` |
| private stress evidence | `<private-only pressure, sanitized public rationale, accepted ADR if used for public review>` | `<private evidence ids or not applicable>` | `<status>` | `<notes>` |
| non-flowchart bot pressure | `<bot pressure without copying publisher flowcharts/priority charts/examples>` | `<rule_ids or not applicable>` | `<status>` | `<notes>` |

## Repeated-shape comparison

| Shape | Classification | Governing owner | Already appears in | Same shape? | Similarities | Differences | Required next step |
|---|---|---|---|---:|---|---|---|
| `<shape>` | behavioral mechanic / production mechanical scaffolding / test scaffolding / presentation scaffolding / superficial similarity | mechanic atlas / mechanical scaffolding register / local-only / ADR | `<game_ids>` | yes/no/unclear | `<similarities>` | `<differences>` | none / compare / ledger / register / reuse / promote / defer / ADR |

## Mechanical scaffolding reuse-first audit

Complete this table for every new official game before implementation
admission. A game with no relevant scaffolding surface records one explicit
not-applicable row with rationale.

| Planned surface | Existing MSC entry/shared symbol reviewed | Decision | Why the accepted boundary fits or does not fit | New register entry needed? | Earlier official-game matches | Expected follow-on unit or accepted no-unit disposition | Hash/visibility/determinism expectation |
|---|---|---|---|---:|---|---|---|
| `<effect/seat/action-tree/stable-byte/test/evidence/bridge surface>` | `<MSC id and symbol/path>` | reuse / accepted exception / local-only / new candidate / rejected-rerouted / not applicable | `<rationale>` | yes/no | `<game ids/sites or none>` | `<unit id / register decision / none>` | unchanged / ADR 0009 migration required: `<authority>` |

Audit rules:

- compare semantic responsibility, not only names or text;
- reuse a matching promoted helper before writing a parallel local shape;
- register every new behavior-free first-use shape without treating first use
  as promotion authority;
- route legality, scoring, reveal, turn, trick, team, graph, accounting,
  reaction, outcome, strategy, effect meaning, renderer policy, and
  hidden-state policy back to the behavioral lane; and
- identify prior-game refactoring work now, not after the game ships.

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

## Required repo atlas/register update

Update `docs/MECHANIC-ATLAS.md` and the relevant
`PRIMITIVE-PRESSURE-LEDGER.md` instance when behavioral mechanic pressure
changes. Update `docs/MECHANICAL-SCAFFOLDING-REGISTER.md` when scaffolding is
reused with new evidence, newly invented, kept local by decision, deferred,
rejected, promoted, or leaves prior-game migration work.

| Update target | Required? | Reason | Owner |
|---|---:|---|---|
| `docs/MECHANIC-ATLAS.md` | yes/no | `<behavioral pressure reason>` | `<owner>` |
| `PRIMITIVE-PRESSURE-LEDGER.md` | yes/no | `<behavioral pressure reason>` | `<owner>` |
| `docs/MECHANICAL-SCAFFOLDING-REGISTER.md` | yes/no | `<reuse/new shape/prior-game migration/no-unit disposition>` | `<owner>` |
| `specs/README.md` follow-on unit | yes/no | `<prior-game migration set or accepted no-unit rationale>` | `<owner>` |
| ADR | yes/no | `<boundary/hash/visibility reason>` | `<owner>` |

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
- treating private-stress evidence as public promotion authority without
  sanitized rationale and accepted ADR review;
- treating event-card coverage rows as executable selectors, conditions,
  triggers, or effect formulas.

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
- The mechanical-scaffolding reuse-first audit is complete even when the result is not applicable.
- Every known promoted scaffolding match is reused or covered by an accepted exception.
- Every new behavior-free scaffolding shape has a planned register entry.
- Every prior-game match has a planned tracker unit or accepted no-unit disposition.
- The required repo update section names the scaffolding register explicitly.

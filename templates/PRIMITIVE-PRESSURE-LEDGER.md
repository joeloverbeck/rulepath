# Primitive Pressure Ledger: <candidate_name>

Candidate name: `<candidate_name>`

Status: local-only | repeated-shape candidate | extraction required | promoted primitive | rejected/deferred with rationale | ADR-required

Last updated: YYYY-MM-DD

Prepared by: `<name/agent>`

Template realignment mapping: report `B-15 -> PRIMITIVE-PRESSURE-LEDGER.md`

Scaffolding register: [`MECHANICAL-SCAFFOLDING-REGISTER.md`](../docs/MECHANICAL-SCAFFOLDING-REGISTER.md)

Behavioral scope only: this ledger governs repeated behavioral mechanic
pressure. Non-behavioral repeated plumbing, production mechanical
scaffolding, test scaffolding, evidence scaffolding, and presentation
scaffolding must be rejected here and routed to the scaffolding register.

## Hard gate

A third official game with the same mechanic shape is blocked until this ledger records one of:

- reuse of an existing primitive;
- promotion to a narrow typed primitive;
- explicit defer/reject with rationale;
- `ADR-required`.

This ledger is not a universal behavior-language proposal. It is evidence for or against a narrow typed helper.
It is also not the decision surface for behavior-free infrastructure such as effect envelopes,
seat IDs, action-tree transport, replay/hash bytes, benchmark/evidence records, or test harness
plumbing. Those candidates belong in the scaffolding register.

## Mechanic shape

Describe the repeated behavioral mechanic shape in prose. Avoid game-specific brand names. Avoid adding game nouns to `engine-core`.

- `<mechanic_shape_description>`

Reject or redirect the candidate if the repeated shape is mechanical scaffolding
rather than behavior. Use the scaffolding register for behavior-free duplicate
plumbing around existing generic contracts.

## Games exerting pressure

| Game | Roadmap stage/gate | Local implementation area | Pressure type | Status at time of review | Notes |
|---|---:|---|---|---|---|
| `<game_id>` | `<stage>` | `<module/helper>` | first / second / third / benchmark / bug / maintenance | `<status>` | `<notes>` |

## Local implementations compared

| Aspect | Game A | Game B | Game C | Same shape? | Notes |
|---|---|---|---|---:|---|
| seat count | `<notes>` | `<notes>` | `<notes>` | yes/no/unclear | `<min/max seats and role/team model>` |
| topology size | `<notes>` | `<notes>` | `<notes>` | yes/no/unclear | `<graph/track/map/object-count pressure>` |
| data size | `<notes>` | `<notes>` | `<notes>` | yes/no/unclear | `<decks/walls/hands/tables/fixture payload size>` |
| state shape | `<notes>` | `<notes>` | `<notes>` | yes/no/unclear | `<notes>` |
| action shape | `<notes>` | `<notes>` | `<notes>` | yes/no/unclear | `<notes>` |
| action fanout | `<notes>` | `<notes>` | `<notes>` | yes/no/unclear | `<branching/candidate count pressure>` |
| validation | `<notes>` | `<notes>` | `<notes>` | yes/no/unclear | `<notes>` |
| transitions | `<notes>` | `<notes>` | `<notes>` | yes/no/unclear | `<notes>` |
| diagnostics | `<notes>` | `<notes>` | `<notes>` | yes/no/unclear | `<notes>` |
| semantic effects | `<notes>` | `<notes>` | `<notes>` | yes/no/unclear | `<notes>` |
| visibility | `<notes>` | `<notes>` | `<notes>` | yes/no/unclear | `<notes>` |
| view payload size | `<notes>` | `<notes>` | `<notes>` | yes/no/unclear | `<public/private projection pressure>` |
| no-leak complexity | `<notes>` | `<notes>` | `<notes>` | yes/no/unclear | `<pairwise/private-datum-by-viewer-by-surface pressure>` |
| UI pattern | `<notes>` | `<notes>` | `<notes>` | yes/no/unclear | `<notes>` |
| bot use | `<notes>` | `<notes>` | `<notes>` | yes/no/unclear | `<notes>` |
| replay/hash impact | `<notes>` | `<notes>` | `<notes>` | yes/no/unclear | `<notes>` |
| benchmark pressure | `<notes>` | `<notes>` | `<notes>` | yes/no/unclear | `<notes>` |

## Similarities

- `<similarity>`

## Differences

- `<difference>`

## Extraction decision

Decision: reuse / promote / defer / reject / ADR-required

| Decision factor | Finding |
|---|---|
| repeated shape is real? | yes/no/unclear |
| helper can stay narrow and typed? | yes/no/unclear |
| helper belongs in `game-stdlib`? | yes/no/unclear |
| would contaminate `engine-core`? | yes/no/unclear |
| static-data behavior risk? | none/low/medium/high |
| replay/hash impact acceptable? | yes/no/unclear |
| visibility/no-leak impact acceptable? | yes/no/unclear/not applicable |
| examples and anti-examples known? | yes/no |
| benchmarks support extraction? | yes/no/not needed |
| ADR required? | yes/no |

Rationale:

- `<rationale>`

## Rejected alternatives

| Alternative | Why rejected | Notes |
|---|---|---|
| `<alternative>` | `<reason>` | `<notes>` |

## API sketch in prose only

Do not write implementation code here. Describe the narrow typed helper contract in prose.

| Aspect | Prose sketch |
|---|---|
| inputs | `<inputs>` |
| outputs | `<outputs>` |
| error/diagnostic behavior | `<diagnostics>` |
| determinism requirements | `<determinism>` |
| replay/hash requirements | `<replay_hash>` |
| visibility requirements | `<visibility>` |
| effect/log requirements | `<effects>` |
| bot-facing notes | `<bot_notes>` |
| non-goals | `<non_goals>` |
| good-fit examples | `<examples>` |
| anti-examples | `<anti_examples>` |

## Determinism and replay impact

| Impact | Required action | Tests/traces |
|---|---|---|
| action ordering | `<action>` | `<tests/traces>` |
| diagnostics | `<action>` | `<tests/traces>` |
| semantic effects | `<action>` | `<tests/traces>` |
| trace hashes | preserve / migrate with rationale / not affected | `<tests/traces>` |
| serialization | `<action>` | `<tests>` |
| seed/randomness | `<action>` | `<tests>` |

## Visibility and no-leak impact

| Surface | Impact | Required safeguard/test |
|---|---|---|
| public view | none / `<impact>` | `<test>` |
| action tree | none / `<impact>` | `<test>` |
| preview | none / `<impact>` | `<test>` |
| diagnostics | none / `<impact>` | `<test>` |
| effect log | none / `<impact>` | `<test>` |
| DOM/test IDs/local storage/replay export | none / `<impact>` | `<test>` |
| bot explanations/candidate rankings | none / `<impact>` | `<test>` |
| dev inspector | none / `<impact>` | `<test>` |

## UI and effect impact

| Area | Impact | Required update |
|---|---|---|
| semantic effect names/payloads | `<impact>` | `<update>` |
| animation mapping | `<impact>` | `<update>` |
| Rust-generated previews | `<impact>` | `<update>` |
| UI controls/action tree mapping | `<impact>` | `<update>` |
| reduced-motion behavior | `<impact>` | `<update>` |
| accessibility labels/summaries | `<impact>` | `<update>` |

## Bot impact

| Bot level | Impact | Required update/tests |
|---|---|---|
| Level 0 random legal | `<impact>` | `<tests>` |
| Level 1 baseline | `<impact>` | `<tests>` |
| Level 2 authored policy | `<impact>` | `<evidence/tests>` |
| Level 3 shallow deterministic search | `<impact>` | `<tests/not applicable>` |

## Tests required

| Test | Required before promotion? | Required before reuse? | Notes |
|---|---:|---:|---|
| primitive unit tests | yes/no | yes/no | `<notes>` |
| compatibility tests in each back-ported game | yes/no | yes/no | `<notes>` |
| named rule tests remain mapped | yes/no | yes/no | `<notes>` |
| golden trace preservation/update notes | yes/no | yes/no | `<notes>` |
| property/invariant tests | yes/no | yes/no | `<notes>` |
| replay/hash tests | yes/no | yes/no | `<notes>` |
| serialization tests | yes/no | yes/no | `<notes>` |
| visibility/no-leak tests | if relevant | if relevant | `<notes>` |
| bot tests | if relevant | if relevant | `<notes>` |
| benchmark tests | yes/no | yes/no | `<notes>` |

## Traces affected

| Trace | Game | Preserve or update? | Reason | Rule IDs/mechanics |
|---|---|---|---|---|
| `<trace_file>` | `<game_id>` | preserve / update with behavior rationale / update with format rationale | `<reason>` | `<ids>` |

## Benchmarks affected

| Benchmark | Game(s) | Expected impact | Required threshold | Status |
|---|---|---|---:|---|
| `<benchmark>` | `<games>` | `<impact>` | `<threshold>` | `<status>` |

## Examples

Good fits:

- `<example>`
- graph topology with bounded typed adjacency and path queries
- private-hand/deck or wall projection with repeated no-leak evidence
- trick-taking turn/lead/follow-suit pressure across official games
- side-pot or split allocation with repeated typed accounting shape

## Anti-examples

Not a fit:

- `<anti_example>`
- generic multiplayer framework work without a specific repeated typed shape
- effect-envelope, seat-ID, action-tree, replay/hash, benchmark/evidence, or test-harness plumbing that should be routed to the scaffolding register
- large static maps used as a behavior language
- one-off UI layout similarity without shared Rust behavior pressure
- broad hidden-information abstraction that cannot state pairwise no-leak tests

## ADR need

ADR required? yes/no

Reason:

- `<reason>`

ADR is required if the proposal changes architecture, replay/hash semantics, data policy, kernel boundaries, browser authority, bot policy class, or public/private content policy.

## Review checklist

- Third-game hard gate is satisfied or the game is blocked.
- Repeated shape was compared across actual official games.
- Non-behavioral scaffolding candidates were rejected or redirected to the scaffolding register.
- No game noun enters `engine-core`.
- Helper belongs in `game-stdlib` or stays local.
- No untyped behavior language is created.
- Static data remains typed content/parameters/metadata/fixtures/traces/reports only.
- Existing games are back-ported if promoted.
- Golden traces are preserved or intentionally updated with rationale.
- Replay/hash and serialization impacts are recorded.
- Visibility/no-leak impacts are covered.
- UI/effect and bot impacts are covered.
- Benchmarks are measured.
- Examples and anti-examples are documented.
- ADR need is explicit.

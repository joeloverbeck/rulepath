# <game_id> Bot Strategy Evidence Pack

Game ID: `<game_id>`

Implemented variant: `<variant>`

Rules version: `<rules_version>`

Bot target: Level 2 authored policy

Policy name/version: `<policy_name>/<policy_version>`

Prepared by: `<name/agent>`

Date: YYYY-MM-DD

## Purpose and gate

This is the formal design input for a Level 2 authored-policy bot. A Level 2 bot MUST NOT be coded until this pack is complete and reviewed.

This pack consumes `COMPETENT-PLAYER.md`; it does not replace it. It also does not replace `GAME-AI.md`, which is the per-game bot registry/status document.

The policy MUST be deterministic under seed, rules version, policy version, input view, and limits. It MUST use the legal action API. It MUST produce viewer-safe explanations.

## Explicit public v1/v2 exclusions

The Level 2 public bot MUST NOT use:

- omniscient state;
- hidden-state shortcuts;
- future random outcomes;
- unbounded weight soup;
- static data tactical conditions;
- random blunder injection by default;
- public v1/v2 MCTS;
- public v1/v2 ISMCTS;
- public v1/v2 Monte Carlo-style bots;
- public v1/v2 ML;
- public v1/v2 RL.

Future search, ML, or RL work requires an ADR under the foundation docs.

## Source documents consumed

| Document/source | Path/reference | Required? | Status | Notes |
|---|---|---:|---|---|
| `GAME-RULES.md` | `<path>` | yes | read / incomplete / blocked | `<notes>` |
| `GAME-RULE-COVERAGE.md` | `<path>` | yes | read / incomplete / blocked | `<notes>` |
| `COMPETENT-PLAYER.md` | `<path>` | yes | read / incomplete / blocked | `<notes>` |
| `GAME-SOURCES.md` strategy references | `<path>` | yes/no | read / incomplete / blocked | `<notes>` |
| `GAME-MECHANICS.md` | `<path>` | yes | read / incomplete / blocked | `<notes>` |
| `GAME-AI.md` | `<path>` | yes | read / incomplete / blocked | `<notes>` |
| other evidence | `<path/source>` | no | read / incomplete / blocked | `<notes>` |

## Exact bot input view

| Input item | Included? | Source | Visible to acting seat? | Notes/no-leak test |
|---|---:|---|---:|---|
| legal action tree/action paths | yes | Rust legal action API | yes | `<test>` |
| public view | yes/no | Rust view projection | yes | `<test>` |
| acting seat private view | yes/no | Rust view projection | yes | `<test>` |
| command/effect history visible to seat | yes/no | viewer-filtered log | yes | `<test>` |
| policy seed/tie-break state | yes | bot framework | not game info | deterministic only |
| hidden opponent/private state | no | forbidden | no | `<test>` |
| unrevealed deck/order/future random outcomes | no | forbidden | no | `<test>` |
| dev/test full state | no | forbidden for public bot | no | `<test>` |

## Legal action API used

| API/contract | Purpose | Determinism requirement | Tests |
|---|---|---|---|
| `<legal_action_api>` | enumerate candidates from Rust action tree | fixed input view and seed produce fixed legal set | `<tests>` |
| `<validation_api>` | submit chosen action path through normal validation | bot never mutates state directly | `<tests>` |
| `<preview_api>` | optional viewer-safe candidate annotation | preview must be safe and Rust-generated | `<tests>` |

## Candidate extraction plan

| Candidate group | Extraction rule | Rule IDs | Visible facts used | Hidden-info risk | Tests |
|---|---|---|---|---|---|
| `<group>` | `<how legal action paths become candidates>` | `<rule_ids>` | `<visible_facts>` | none / low / medium / high | `<tests>` |

Candidates are legal action paths plus policy annotations. They MUST NOT include actual hidden information unavailable to the bot seat.

## Phase model

| Phase/situation | Detection from allowed input | Policy node(s) active | Rule IDs | Notes |
|---|---|---|---|---|
| `<phase>` | `<allowed_input>` | `<nodes>` | `<rule_ids>` | `<notes>` |

## Lexicographic priority vector

Prefer a lexicographic priority vector over giant weighted sums. Earlier slots dominate later slots.

| Slot | Priority | Higher/better value | Source evidence | Rule IDs | Tests | Explanation fragment |
|---:|---|---|---|---|---|---|
| 1 | terminal win / mandatory compliance / `<priority>` | `<definition>` | `COMPETENT-PLAYER.md#...` | `<rule_ids>` | `<tests>` | `<public_safe_text>` |
| 2 | avoid terminal loss / block immediate threat / `<priority>` | `<definition>` | `<evidence>` | `<rule_ids>` | `<tests>` | `<public_safe_text>` |
| 3 | immediate tactical gain / `<priority>` | `<definition>` | `<evidence>` | `<rule_ids>` | `<tests>` | `<public_safe_text>` |
| 4 | opponent denial / `<priority>` | `<definition>` | `<evidence>` | `<rule_ids>` | `<tests>` | `<public_safe_text>` |
| 5 | positional/resource preference / `<priority>` | `<definition>` | `<evidence>` | `<rule_ids>` | `<tests>` | `<public_safe_text>` |
| 6 | style profile hook | `<definition>` | `<evidence>` | `<rule_ids>` | `<tests>` | `<public_safe_text>` |
| 7 | bounded scoring tie-break | `<definition>` | `<evidence>` | `<rule_ids>` | `<tests>` | `<public_safe_text>` |
| 8 | deterministic seeded tie-break | stable order from seed and candidate identity | framework | not applicable | `<tests>` | `<public_safe_text>` |

## Bounded scoring tie-breakers

Small scoring is allowed only after higher lexicographic categories. It must be bounded, named, documented, tested, and explainable.

| Score term | Range | Meaning | Used after slots | Visible inputs | Tests | Explanation text |
|---|---:|---|---|---|---|---|
| `<term>` | `<min..max>` | `<meaning>` | `<slots>` | `<inputs>` | `<tests>` | `<text>` |

Forbidden weight soup examples:

- dozens of magic weights with no priority rationale;
- style implemented only by multiplying weights;
- tactical conditions hidden in static data;
- scores that cannot produce clear explanations;
- tuning without simulations and benchmark evidence.

## Deterministic seeded tie-break

| Item | Decision |
|---|---|
| seed source | `<source>` |
| tie-break input identity | `<candidate_identity_fields>` |
| stable ordering rule | `<rule>` |
| reproducibility tests | `<tests>` |
| replay/hash interaction | `<notes>` |

Tie-break randomness MUST be deterministic. Random blunder injection is forbidden by default.

## Style profile hooks

One strong default bot comes first. Style profiles MAY vary risk posture, priority order after mandatory/terminal priorities, bounded tie-break preferences, and explanation tone.

| Profile | Variation | Must not affect | Hidden-info safe? | Tests |
|---|---|---|---:|---|
| default | strongest public policy | legality, hidden-info boundary, determinism | yes/no | `<tests>` |
| `<profile>` | `<variation>` | mandatory rules, terminal priorities, legal action API | yes/no | `<tests>` |

## Forbidden hidden information

| Information | Why forbidden | Potential leak surface | Required no-leak test |
|---|---|---|---|
| opponent hand/private zone | unavailable to seat | input view, explanation, candidate ranking, replay export | `<test>` |
| unrevealed deck/order/future random outcome | unavailable to seat | candidate features, tie-break, simulation shortcut | `<test>` |
| hidden commitment/role | unavailable until reveal | input view, diagnostics, dev inspector | `<test>` |
| private logs | unavailable to seat/viewer | explanation, DOM, local storage | `<test>` |
| actual sampled hidden state for belief | forbidden shortcut | belief model/determinization | `<test>` |
| `<info>` | `<reason>` | `<surface>` | `<test>` |

## Memory and belief model

Fill this section for hidden-information games. For perfect-information games, mark explicit `not applicable`.

| Item | Decision | Allowed inputs | Forbidden inputs | Tests |
|---|---|---|---|---|
| memory model | none / visible-history memory / `<model>` | public history, own private observations | hidden actual state | `<tests>` |
| belief model | none / legal-possibility abstraction / sampled possibilities from legal info / `<model>` | legal information only | actual hidden state, future random outcomes | `<tests>` |
| redaction model | `<model>` | viewer-safe facts | hidden facts | `<tests>` |

Sampled possibilities, if any, MUST be generated from the bot's legal information, not copied from actual hidden state.

## Explanation contract

Every non-random public bot decision SHOULD produce a viewer-safe explanation with:

| Field | Required? | Notes |
|---|---:|---|
| policy name/version | yes | `<notes>` |
| chosen priority reason | yes | must map to priority vector |
| relevant visible fact | yes | must be visible to viewer or redacted |
| tie-break note | if applicable | bounded score or seeded tie-break |
| hidden-info disclaimer | if relevant | do not reveal hidden facts |
| fallback/search note | if relevant | Level 2 should normally not search |
| known weakness if surfaced | optional | keep concise in public UI |

## Public explanation examples

| Situation | Chosen action | Public explanation | Hidden-info safe? | Rule IDs |
|---|---|---|---:|---|
| `<situation>` | `<action>` | `<short viewer-safe explanation>` | yes/no | `<rule_ids>` |

## Dev-mode ranking examples

Dev mode may show candidate rankings only when viewer-safe and not shipped as public hidden-state leakage.

| Situation | Candidate ranking excerpt | Redactions needed? | Hidden-info safe? | Notes |
|---|---|---:|---:|---|
| `<situation>` | `<ranking excerpt>` | yes/no | yes/no | `<notes>` |

## Decision examples and expected choices

| Example ID | Situation | Candidate choices | Expected choice | Priority vector reason | Rule IDs | Test name |
|---|---|---|---|---|---|---|
| `BOT-EX-001` | `<situation>` | `<choices>` | `<choice>` | `<reason>` | `<rule_ids>` | `<test_name>` |

## Known weaknesses

| Weakness | Why acceptable for public Level 2 | Mitigation | Future trigger |
|---|---|---|---|
| `<weakness>` | `<rationale>` | `<mitigation>` | `<trigger>` |

Do not hide weaknesses behind magic weights.

## Test plan

| Test area | Required? | Specific tests | Evidence |
|---|---:|---|---|
| legality | yes | bot chooses only legal action paths over many seeds | `<tests>` |
| determinism | yes | fixed seed/view/rules/policy/limits produce fixed decision | `<tests>` |
| no hidden-state access | if hidden info exists | input view/explanation/ranking/replay/export no-leak | `<tests>` |
| candidate extraction | yes | candidate groups match legal actions and visible facts | `<tests>` |
| priority vector | yes | decision examples hit expected slots | `<tests>` |
| bounded scoring | if used | ranges and explanations tested | `<tests>` |
| seeded tie-break | yes | stable tie ordering | `<tests>` |
| explanations | yes | viewer-safe public explanations | `<tests>` |
| simulation/fuzz | yes | many-seed games, failure reporting | `<runs>` |
| replay/hash | yes | bot decision reproducible in replay | `<tests>` |
| benchmark | yes | latency and throughput | `<benchmarks>` |

## Latency and benchmark budget

| Operation | Target/budget | Measurement command | Baseline | Notes |
|---|---:|---|---:|---|
| legal action generation | `<target>` | `<command>` | `<baseline>` | `<notes>` |
| candidate extraction | `<target>` | `<command>` | `<baseline>` | `<notes>` |
| priority ranking | `<target>` | `<command>` | `<baseline>` | `<notes>` |
| full decision latency | `<target>` | `<command>` | `<baseline>` | `<notes>` |
| playout throughput with bot | `<target>` | `<command>` | `<baseline>` | `<notes>` |
| explanation generation | `<target>` | `<command>` | `<baseline>` | `<notes>` |

Native Rust benchmark evidence is primary. Browser/WASM smoke is required if the bot is public-web exposed.

## Public UX note

Describe how the public UI should expose the bot's recent decision or “why?” explanation without turning the game into a debug console.

- `<public_ux_note>`

## Review checklist

- `COMPETENT-PLAYER.md` was consumed.
- Legal action API and validation path are exact.
- Bot input view is explicit.
- No omniscient state, hidden-state shortcuts, or future random outcomes are used.
- Candidate extraction uses legal action paths and allowed views.
- Priority vector is lexicographic.
- Bounded scores are small, named, documented, and tested.
- Tie-breaks are deterministic under seed and candidate identity.
- Style profiles do not cheat or weaken mandatory priorities.
- Hidden-information no-leak tests cover explanations, candidate rankings, replay exports, diagnostics, and dev inspector where applicable.
- Public v1/v2 MCTS, ISMCTS, Monte Carlo bots, ML, and RL are absent.
- Test plan, simulation plan, replay/hash plan, and benchmark plan are complete.
- Public UX note is concise and product-facing.

# <game_id> Rules

Game ID: `<game_id>`

Public display name: `<display_name>`

Implemented variant: `<variant>`

Rules version: `<rules_version>`

Prepared by: `<name/agent>`

Created: YYYY-MM-DD

Last updated: YYYY-MM-DD

## Rule authority

This document is the original Rulepath rules summary for the implemented variant. It MUST be written in original Rulepath prose.

Do not copy rulebook prose, card text, UI text, source examples, screenshots, scans, or proprietary presentation. Sources belong in `GAME-SOURCES.md`; this document states the Rulepath implementation contract.

Stable rule IDs are requirements. They MUST remain stable after implementation unless intentionally migrated with a migration note and corresponding updates in `GAME-RULE-COVERAGE.md`, traces, tests, and docs.

Suggested rule-ID prefixes:

- `R-SCOPE-001`
- `R-COMP-001`
- `R-SETUP-001`
- `R-TURN-001`
- `R-ACTION-001`
- `R-RESTRICT-001`
- `R-SCORE-001`
- `R-END-001`
- `R-VIS-001`
- `R-RNG-001`
- `R-VAR-001`
- `R-AMB-001`

## Metadata

| Field | Value |
|---|---|
| game id | `<game_id>` |
| public display name | `<display_name>` |
| variant | `<variant>` |
| rules version | `<rules_version>` |
| source note | `<path/to/GAME-SOURCES.md>` |
| coverage matrix | `<path/to/GAME-RULE-COVERAGE.md>` |
| mechanic inventory | `<path/to/GAME-MECHANICS.md>` |
| implementation admission | `<path/to/GAME-IMPLEMENTATION-ADMISSION.md>` |

## Purpose and scope

| Rule ID | Rule statement | Notes |
|---|---|---|
| `R-SCOPE-001` | `<original Rulepath description of what the game is and what experience the implemented variant supports>` | `<notes>` |
| `R-SCOPE-002` | `<public role: scaffolding, showcase, hidden-info proof, original portfolio game, etc.>` | `<notes>` |

## Implemented variant

| Rule ID | Rule statement | Source/rationale link |
|---|---|---|
| `R-VAR-001` | `<exact variant implemented>` | `GAME-SOURCES.md#...` |
| `R-VAR-002` | `<variant parameters, player count, setup options, or scoring options>` | `GAME-SOURCES.md#...` |

## Seat model

Every official game MUST state its seat model explicitly, including two-seat games.

| Rule ID | Seat-model field | Rule statement | Source/rationale link | Notes |
|---|---|---|---|---|
| `R-SEAT-001` | min/max seats | `<minimum and maximum supported seats; use exact counts or bounded ranges>` | `GAME-SOURCES.md#...` / `<design rationale>` | `<notes>` |
| `R-SEAT-002` | official seat IDs | `<stable seat identifiers used by Rust, traces, replays, and views>` | `<source/rationale>` | `<notes>` |
| `R-SEAT-003` | seat labels | `<player-facing labels for each seat or label-generation rule>` | `<source/rationale>` | `<notes>` |
| `R-SEAT-004` | role/team assignment | `<solo, team, partnership, asymmetric role, coalition, or not applicable>` | `<source/rationale>` | `<notes>` |
| `R-SEAT-005` | order of play | `<seat order, dealer/lead/priority rule, simultaneous order, or initiative rule>` | `<source/rationale>` | `<notes>` |
| `R-SEAT-006` | setup rejection | `<wrong-seat-count, invalid role assignment, unsupported topology, or variant rejection rules>` | `<source/rationale>` | `<viewer-safe diagnostic expectations>` |
| `R-SEAT-007` | viewer classes | `<public observer, owning seat, teammate, opponent, eliminated seat, replay viewer, dev-only harness, etc.>` | `<source/rationale>` | `<notes>` |

## Components and game-local vocabulary

Define only terms needed for this game. Game nouns belong here or in `games/*`, not in `engine-core`.

| Rule ID | Term/component | Original Rulepath definition | Visibility | Notes |
|---|---|---|---|---|
| `R-COMP-001` | `<term>` | `<definition>` | public / private / mixed / not applicable | `<notes>` |

## Setup

| Rule ID | Setup rule | Deterministic/random? | Visibility | Notes |
|---|---|---|---|---|
| `R-SETUP-001` | `<players/seats and initial state>` | deterministic / random | public/private/mixed | `<notes>` |
| `R-SETUP-002` | `<first player or starting condition>` | deterministic / random | public/private/mixed | `<notes>` |
| `R-SETUP-003` | `<variant parameters or setup constants>` | deterministic / random | public/private/mixed | `<notes>` |

## Turn, round, and phase sequence

| Rule ID | Situation/phase | Active seat(s) | Required sequence | Advances when |
|---|---|---|---|---|
| `R-TURN-001` | `<situation>` | `<seat>` | `<steps in original prose>` | `<advance condition>` |
| `R-TURN-002` | `<cleanup/end-of-turn rule>` | `<seat>` | `<steps>` | `<advance condition>` |

## Legal actions

Rust MUST generate legal actions. TypeScript MUST NOT decide legality.

| Rule ID | Situation | Legal action(s) | Action shape | Rust-owned validation notes |
|---|---|---|---|---|
| `R-ACTION-001` | `<state/phase>` | `<legal choices>` | flat / action tree / progressive / simultaneous / reaction | `<notes>` |
| `R-ACTION-002` | `<state/phase>` | `<legal choices>` | flat / action tree / progressive / simultaneous / reaction | `<notes>` |

## Forced actions and restrictions

| Rule ID | Situation | Forced action/restriction | Illegal/stale diagnostic expectation | Notes |
|---|---|---|---|---|
| `R-RESTRICT-001` | `<situation>` | `<mandatory/disabled choice>` | `<viewer-safe diagnostic>` | `<notes>` |
| `R-RESTRICT-002` | `<situation>` | `<mandatory/disabled choice>` | `<viewer-safe diagnostic>` | `<notes>` |

## Scoring and accounting

| Rule ID | Scoring/accounting rule | Timing | Tiebreaker/edge case | Notes |
|---|---|---|---|---|
| `R-SCORE-001` | `<points/resources/counters/pots/etc.>` | `<when>` | `<tie/edge>` | `<notes>` |

## Showdown and evaluator rules

Required when final ranking, hand comparison, reveal resolution, pairwise comparison, allocation, or table-wide evaluator logic can decide the result. For games without showdown/evaluator behavior, add one explicit `not applicable` row.

| Rule ID | Evaluator/showdown rule | Applies to seats | Decisive facts | Reveal/no-reveal limit | Notes |
|---|---|---|---|---|---|
| `R-SHOW-001` | `<ranking/comparison/evaluator/allocation rule or not applicable>` | `<seat set / pairwise / table-wide / team>` | `<facts Rust may cite in outcome explanation>` | `<what remains hidden per viewer>` | `<notes>` |

## Terminal conditions

| Rule ID | Terminal condition | Outcome | Tie handling | Notes |
|---|---|---|---|---|
| `R-END-001` | `<condition>` | `<winner/loss/draw/shared outcome>` | `<tie>` | `<notes>` |
| `R-END-002` | `<condition>` | `<winner/loss/draw/shared outcome>` | `<tie>` | `<notes>` |

## Outcome explanation traceability

Every scoring and terminal rule that can decide a match MUST have a stable rule ID and enough detail for the web outcome surface to cite it.

| Outcome/explanation fact | Stable rule ID(s) | Decisive when | Notes for UI explanation |
|---|---|---|---|
| `<score component / tiebreaker / line / showdown strength / terminal reason>` | `<R-SCORE-*/R-END-*>` | `<condition decided by Rust>` | `<player-facing wording constraint>` |

This table is traceability only. It is not a behavior DSL, selector table, or TypeScript decision source. Rust remains the source of scoring, terminal detection, and rationale projection.

## Visibility and private information

Public/browser payloads MUST NOT reveal hidden information through public views, action trees, previews, diagnostics, effect logs, DOM attributes, test IDs, logs, local storage, replay exports, bot explanations, candidate rankings, or dev inspectors.

| Rule ID | Information | Visible to whom | When visible | Surfaces requiring protection | Notes |
|---|---|---|---|---|---|
| `R-VIS-001` | `<information>` | all / acting seat / owning seat / no one until reveal / dev-only test harness | `<timing>` | public view, action tree, preview, diagnostics, effect log, DOM, logs, storage, replay export, bot explanation, candidate ranking, dev inspector | `<notes>` |
| `R-VIS-002` | `<information>` | `<viewer>` | `<timing>` | `<surfaces>` | `<notes>` |

## Replay and randomness notes

| Rule ID | Randomness/replay rule | Seed/log/hash requirement | Visibility | Notes |
|---|---|---|---|---|
| `R-RNG-001` | `<random setup/draw/event/sample or not applicable>` | `<determinism requirement>` | `<visibility>` | `<notes>` |
| `R-RNG-002` | `<replay/hash requirement>` | `<requirement>` | `<visibility>` | `<notes>` |

## Bot-relevant non-authoritative strategy notes

These notes are not rule authority. They guide `COMPETENT-PLAYER.md`, `BOT-STRATEGY-EVIDENCE-PACK.md`, and `GAME-AI.md`. Strategy claims MUST be checked against rule IDs above.

| Situation | Practical note | Rule IDs checked | Hidden-info limit |
|---|---|---|---|
| `<situation>` | `<strategy note>` | `<rule_ids>` | `<limit>` |

## Known ambiguities and chosen resolutions

| Rule ID | Ambiguity | Chosen resolution | Source/rationale | Tests/traces required | Notes |
|---|---|---|---|---|---|
| `R-AMB-001` | `<ambiguity>` | `<resolution>` | `GAME-SOURCES.md#...` / `<design rationale>` | `<tests/traces>` | `<notes>` |

## Rulepath deviations from common variants

| Rule ID | Common variant behavior | Rulepath behavior | Why | Public note needed? |
|---|---|---|---|---:|
| `R-VAR-003` | `<common behavior>` | `<Rulepath behavior>` | `<reason>` | yes/no |

## Explicit out-of-scope variants

| Rule ID | Out-of-scope variant/rule | Reason out of scope | Future review trigger |
|---|---|---|---|
| `R-VAR-004` | `<variant/rule>` | `<reason>` | `<trigger or none>` |

## Rule coverage link

The implementation and evidence mapping lives in `GAME-RULE-COVERAGE.md`.

Every rule ID in this document MUST appear in `GAME-RULE-COVERAGE.md`. Silent gaps are not allowed.

## Rule-ID migration notes

| Old rule ID | New rule ID(s) | Reason | Coverage/traces updated? | Date |
|---|---|---|---:|---|
| `<old_id>` | `<new_id>` | `<reason>` | yes/no/not applicable | YYYY-MM-DD |

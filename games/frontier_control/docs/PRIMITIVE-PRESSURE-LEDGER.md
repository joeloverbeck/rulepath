# Primitive Pressure Ledger: Frontier Control graph and faction asymmetry

Candidate name: `frontier-control-graph-faction-asymmetry`

Status: pre-implementation reviews recorded; first-use shapes stay local-only; second-use shapes stay local

Decision date: 2026-06-11

Last updated: 2026-06-11

Prepared by: `Codex`

## Hard gate

This ledger records Gate 13's primitive-pressure decisions before any
`frontier_control` implementation code, typed map data, setup validation,
movement logic, scoring logic, bot policy, WASM bridge, or browser UI is
written.

Decision: keep Frontier Control mechanics local. No helper is added to
`engine-core` or `game-stdlib`. No promotion debt is created, so
`docs/MECHANIC-ATLAS.md` §10A remains `_None_`.

The repository-level record is
[../../../docs/MECHANIC-ATLAS.md](../../../docs/MECHANIC-ATLAS.md) §10/§10B.

This ledger records four negative or no-op reviews:

- `game-stdlib::board_space` is not applicable.
- Frontier Control is not a shared-outcome cooperative terminal use.
- Frontier Control is not reaction-capable.
- Frontier Control uses no deterministic shuffle, hidden private holdings, or
  staged reveal.

It also records two second-use comparisons:

- role-modified action effects, compared with Flood Watch;
- multi-action turn budgets, compared with Flood Watch.

It records three first official uses:

- graph-map topology, adjacency-constrained legality, and connectivity scoring;
- site control and deterministic contest resolution;
- faction-asymmetric action sets and faction-specific scoring.

## Mechanic shape

Negative `board_space` audit:

```text
Frontier Control uses a site/edge graph. It has no rectangular dimensions, no
row-major coordinate iteration, no `rNcM` coordinate identity, and no generic
board-space coordinate parsing. The promoted `game-stdlib::board_space`
primitive is therefore not applicable.
```

Negative shared-outcome review:

```text
Frontier Control is competitive. Factions score on one comparable numeric track
and one faction wins. It does not repeat Flood Watch's all-seats-win or
all-seats-lose terminal shape.
```

Negative reaction-window review:

```text
Frontier Control clashes resolve immediately inside the mover's command. No
seat receives a responder tree, no interrupt/cancel window opens, and no action
waits on another seat's pending response.
```

Negative deterministic-shuffle/private-holdings review:

```text
Frontier Control uses no game-rule randomness, no shuffled deck, no private
holdings, no hidden component order, and no staged reveal.
```

Second-use comparisons:

```text
Flood Watch roles modify magnitudes inside one shared action set. Frontier
Control factions have disjoint action sets, asymmetric clash rules, and
faction-specific scoring formulas. These are related public role/faction
modifier pressures, but not one narrow helper shape.

Flood Watch and Frontier Control both use budgeted turns with regenerated legal
trees, `end_turn`, and waiting metadata. This is a second official use. Keep it
local; the third-use hard gate is armed for `event_frontier`.
```

First-use shapes:

```text
site/edge graph topology where adjacency constrains movement and connectivity
determines supplied stake scoring;

site control through public occupancy, stakes, forts, and deterministic
asymmetric clashes;

faction-asymmetric action vocabularies and faction-specific scoring formulas.
```

## Games exerting pressure

| Game | Roadmap stage/gate | Local implementation area | Pressure type | Status at time of review | Notes |
|---|---:|---|---|---|---|
| `three_marks`, `column_four`, `directional_flip`, `draughts_lite` | Gates 4-7 | `game-stdlib::board_space` conformance | promoted primitive audit | implemented | Rectangular coordinate identity is promoted and closed. |
| `flood_watch` | Gate 12 | `games/flood_watch/src/actions.rs`, `rules.rs`, `effects.rs`, `visibility.rs`, `bots.rs` | first role-modifier and multi-action-budget use | implemented | Public roles modify shared actions; budgeted turns regenerate legal trees. |
| `masked_claims` | Gate 11 | `games/masked_claims/src/actions.rs`, `rules.rs`, `visibility.rs` | first reaction-window use | implemented | A claim opens one responder-only accept/challenge window. |
| `high_card_duel`, `poker_lite`, `plain_tricks`, `masked_claims` | Gates 8-11 | setup, visibility, effects, replay/export docs and tests | deterministic private-hand/staged-reveal pressure | implemented | Frontier Control does not use this shape. |
| `frontier_control` | Gate 13 | planned `games/frontier_control/src/{actions,rules,effects,visibility,bots}.rs` | first graph/control/asymmetry use; second budget and role/faction comparison | admitted, not implemented | All graph, control, and faction nouns stay game-local. |

## Local implementations compared

| Aspect | Flood Watch role modifiers | planned Frontier Control factions | Same shape? | Notes |
|---|---|---|---:|---|
| action vocabulary | Both roles use bail/reinforce/forecast/end-turn. | Factions have disjoint action sets: march/stake/muster vs patrol/reinforce/dismantle. | no | Disjoint vocabulary is stronger asymmetry. |
| modifier type | Role changes action magnitude. | Faction changes available actions, clash rule, and scoring formula. | partial | Both are public role/faction pressure. |
| validation | Shared action validators branch by public role magnitude. | Separate faction-specific validators and path shapes. | no | A helper would need policy flags. |
| semantic effects | Same effect kinds carry role-modified magnitudes. | Movement, clash, stake, dismantle, scoring, and terminal effects are faction-flavored. | partial | Both are public and Rust-owned. |
| bot use | Role-informed cooperative policy. | Separate Garrison and Prospector policies. | partial | Strategy remains game-local. |

| Aspect | Flood Watch budgeted turns | planned Frontier Control budgeted turns | Same shape? | Notes |
|---|---|---|---:|---|
| budget | Active seat spends several actions before turn resolution. | Active faction spends two actions before turn handoff/scoring. | yes | Both expose remaining-budget metadata. |
| legal tree regeneration | Regenerates after each action. | Regenerates after each action. | yes | Same generic contract pressure. |
| `end_turn` | Always legal during action phase. | Always legal during action phase. | yes | Prevents stalls in both games. |
| waiting metadata | Teammate receives safe waiting metadata. | Non-active faction receives safe waiting metadata. | yes | Public and viewer-safe. |
| post-turn automation | Environment event batch. | Round scoring after Garrison turn. | partial | Automation/scoring logic differs. |

| Aspect | `game-stdlib::board_space` | planned Frontier Control graph | Same shape? | Notes |
|---|---|---|---:|---|
| topology | Rectangular grid dimensions and coordinates. | Arbitrary site/edge graph. | no | No rectangular coordinate identity. |
| iteration | Deterministic row-major coordinate iteration. | Deterministic site/edge order from typed content. | no | Not `board_space` scope. |
| IDs | Stable `rNcM` parse/format. | Game-local site IDs such as `site_quarry`. | no | Site IDs stay local. |
| behavior | Behavior-free coordinate helper. | Adjacency and connectivity are score-bearing game behavior. | no | Must stay in Rust game module. |

## First-use local records

| Mechanic shape | Status | Current decision | Second-use revisit trigger |
|---|---|---|---|
| graph-map topology / adjacency legality / connectivity scoring | `local-only` first official use | Keep site IDs, edge validation, adjacency checks, path traversal, and supplied-stake scoring local to `games/frontier_control`. | Revisit when `event_frontier` or another official game repeats graph-map topology, adjacency-constrained legality, or connectivity scoring. |
| site control / deterministic contest resolution | `local-only` first official use | Keep occupancy, stake/fort control, guard/crew clashes, and control effects local. | Revisit when another official game repeats public site control plus deterministic contest resolution. |
| faction-asymmetric action sets and scoring | `local-only` first official use | Keep disjoint action vocabularies, faction-specific validation, faction-specific clash rules, scoring, UI metadata, and bot policy local. | Revisit when another official game repeats comparable faction-asymmetric action/scoring pressure. |

## Extraction decision

Decision: defer/reject extraction for the reviewed repeated rows; keep first-use
shapes local-only; keep second-use budget/faction pressure local.

| Decision factor | Finding |
|---|---|
| repeated shape is real? | yes for multi-action budgets; partial but not helper-ready for role/faction modifiers; no for shared outcome, reaction window, shuffle/private holdings, or `board_space` applicability |
| helper can stay narrow and typed? | no for current graph/control/asymmetry; unclear for budgets before a third use |
| helper belongs in `game-stdlib`? | no |
| would contaminate `engine-core`? | yes if graph, map, site, edge, adjacency, faction, unit, guard, crew, stake, fort, clash, supply, control, movement, budget, or role nouns moved there |
| static-data behavior risk? | medium if map data encoded movement rules, clash outcomes, scoring formulas, selectors, conditions, or bot policy; current plan forbids that |
| replay/hash impact acceptable? | yes for local implementation; no shared helper or migration is authorized |
| visibility/no-leak impact acceptable? | yes; the game is perfect-information and still Rust-owned |
| examples and anti-examples known? | yes |
| benchmarks support extraction? | no; no helper extraction is proposed |
| ADR required? | no, because no architecture, replay/hash, visibility, data, or kernel boundary change is made |

Rationale:

- The promoted rectangular board-space primitive does not match a graph of named
  sites and edges.
- Flood Watch and Frontier Control prove real second-use budget pressure, but a
  helper before `event_frontier` would be premature and could hide turn-policy
  behavior.
- Flood Watch role modifiers and Frontier Control faction asymmetry are related
  public asymmetry pressure, but they differ enough that a helper would need
  action-set, effect, scoring, and bot-policy flags.
- Graph topology, site control, and faction-asymmetric scoring are first uses.
  Rulepath's first-use rule keeps them local.
- No existing trace/hash migration or prior-game conformance work is required.

## Rejected alternatives

| Alternative | Why rejected | Notes |
|---|---|---|
| Reuse `game-stdlib::board_space` | The map is not rectangular and does not use coordinate identity. | Audit records not applicable, not an exception. |
| Promote a graph/pathfinding helper | First official graph-map use; connectivity is score-bearing behavior. | Revisit only after another official game repeats the shape. |
| Promote an area-control helper | First official control/contest use and too policy-bearing. | Stake, fort, clash, and supply rules stay local. |
| Promote a faction/asymmetry helper | First official disjoint faction action/scoring use; role comparison is only partial. | Per-faction bots and UI remain game-local. |
| Promote a budget helper now | This is the second official budget use, not the third-use hard gate. | `event_frontier` is the named hard-gate candidate. |
| Move nouns into `engine-core` | Foundation law forbids mechanic nouns in the kernel. | `engine-core` remains unchanged. |
| Encode movement, clash, or scoring behavior in map data | Static data must remain typed content/parameters only. | Behavior stays in Rust. |
| Escalate to ADR | No architecture, kernel vocabulary, data policy, visibility contract, or replay/hash semantic change is proposed. | ADR becomes required only if a future helper changes those boundaries. |

## API sketch in prose only

No API is approved by this ledger.

| Aspect | Prose sketch |
|---|---|
| inputs | not applicable; no helper promoted |
| outputs | not applicable; no helper promoted |
| error/diagnostic behavior | game-local viewer-safe diagnostics stay in `games/frontier_control` |
| determinism requirements | setup, action generation, clashes, scoring, effects, views, and replay remain deterministic locally |
| replay/hash requirements | no migration of existing games or trace hashes |
| visibility requirements | all projected game facts are public; bot/dev output remains Rust-owned and viewer-safe |
| effect/log requirements | movement, clash, stake, dismantle, muster, reinforce, scoring, and terminal effects remain game-local |
| bot-facing notes | bots consume legal tree and public view only; per-faction policy remains game-local |
| non-goals | generic graph, area-control, faction, clash, supply, budget, scoring, bot, or TypeScript legality helper |
| good-fit examples | none until a later ledger proves a repeated behavior-free shape with worthwhile conformance payoff |
| anti-examples | compute connectivity in TypeScript, put adjacency in `engine-core`, define clash formulas in data, generalize faction policy into `game-stdlib` |

## Determinism and replay impact

| Impact | Required action | Tests/traces |
|---|---|---|
| action ordering | Preserve faction-specific action path order and regenerate budgeted legal tree after each action. | future action/rules tests and golden traces |
| diagnostics | Keep wrong-seat, stale, invalid-trail, invalid-occupancy, invalid-stake, invalid-reinforce, and terminal diagnostics viewer-safe. | future rules/visibility tests |
| semantic effects | Emit Rust effects for movement, clash, stake, dismantle, muster, reinforce, budget, round scoring, and terminal. | future effect/order traces and browser smoke |
| trace hashes | preserve existing games; create Frontier Control traces locally | future replay-check for `frontier_control` |
| serialization | stable local state/effect/view ordering | future serialization tests |
| seed/randomness | no game-rule randomness; bot RNG remains bot infrastructure | future setup/replay tests |

## Visibility and no-leak impact

| Surface | Impact | Required safeguard/test |
|---|---|---|
| public view | all game facts are public, but full internal debug state must not leak | future visibility tests |
| action tree | legal choices and waiting metadata must be Rust-owned and public-safe | future action-tree tests |
| preview | previews cite public movement/clash/scoring facts only | future preview/visibility tests |
| diagnostics | diagnostics must not expose internal-only parser/debug state | future invalid-action tests |
| effect log | all effects are public and ordered | future golden traces/effect tests |
| DOM/test IDs/local storage/replay export | no internal-only fields, no TS-computed hidden state | future browser smoke and export tests |
| bot explanations/candidate rankings | no direct state mutation or hidden evaluator output | future bot tests |
| dev inspector | viewer-safe public data only | future WASM/browser checks |

## UI and effect impact

| Area | Impact | Required update |
|---|---|---|
| semantic effect names/payloads | game-local effect set for graph movement, clashes, scoring, and terminal | future `effects.rs`, WASM mirror, and `effectFeedback.ts` |
| animation mapping | graph-map animation from Rust effects | future `FrontierControlBoard.tsx` and E2E smoke |
| Rust-generated previews | public adjacency, clash, budget, and scoring hints only | future `ui.rs`/WASM tests |
| UI controls/action tree mapping | faction-specific leaves and waiting state | future web smoke |
| reduced-motion behavior | preserve effect order with less motion | future E2E smoke |
| accessibility labels/summaries | public state only, with supplied/cut status from Rust | future a11y smoke |

## Bot impact

| Bot level | Impact | Required update/tests |
|---|---|---|
| Level 0 random legal | choose from Rust legal tree only | future bot legality tests |
| Level 1 baseline | separate public Garrison and Prospector policies | future strategy evidence and determinism tests |
| Level 2 authored policy | not claimed by Gate 13 | not applicable |
| Level 3 shallow deterministic search | not allowed/claimed | not applicable |

## Tests required

| Test | Required before promotion? | Current status |
|---|---:|---|
| primitive unit tests | yes if a future helper is proposed | not applicable now |
| compatibility tests in prior games | yes if promoted | not applicable now |
| named rule tests remain mapped | yes | future `RULE-COVERAGE.md` ticket |
| golden trace preservation/update notes | yes if promoted | not applicable now; no migration |
| property/invariant tests | yes | future Frontier Control property tests |
| replay/hash tests | yes | future Frontier Control replay tests |
| serialization tests | yes | future Frontier Control serialization tests |
| visibility/no-leak tests | yes | future native, WASM, and browser evidence |
| bot tests | yes | future Frontier Control bot tests |
| benchmark tests | yes | future Frontier Control benchmarks |

## Traces affected

| Trace | Game | Preserve or update? | Reason | Rule IDs/mechanics |
|---|---|---|---|---|
| existing golden traces | all prior official games | preserve | No shared helper or migration. | prior mechanics |
| future `games/frontier_control/tests/golden_traces/*.trace.json` | `frontier_control` | create locally | Local implementation must prove deterministic setup, action order, graph/control scoring, terminal, visibility, and replay/export behavior. | `FC-*` rules |

## Back-port and conformance plan

No back-port is required because no helper is promoted.

Affected prior games:

- `three_marks`, `column_four`, `directional_flip`, `draughts_lite`: no code or
  trace change; `board_space` remains promoted and Frontier Control is audited
  not applicable.
- `flood_watch`: no code or trace change; role/faction and budget rows record
  comparisons only.
- `masked_claims`: no code or trace change; reaction-window row remains first
  use.
- `high_card_duel`, `poker_lite`, `plain_tricks`, `masked_claims`: no code or
  trace change for deterministic shuffle/private-holding rows.

Exceptions:

- None. This is a no-promotion decision, not a promoted primitive with
  exceptions.

Closure gate if debt is deferred:

- Not applicable. No promotion debt is created, so `docs/MECHANIC-ATLAS.md`
  §10A remains `_None_`.

## Examples

Good fits for future review only:

- Another official game whose map is a site/edge graph and whose legal actions
  or scoring depend on adjacency/connectivity.
- Another official game whose site control uses public occupancy and
  deterministic contest resolution.
- Another official game whose factions have disjoint action vocabularies and
  faction-specific scoring.
- Another official game whose turn grants a fixed budget of multiple validated
  actions.

## Anti-examples

Not a fit:

- A rectangular coordinate board that already fits `game-stdlib::board_space`.
- A seat response window, interrupt, cancel, or pending challenge.
- A private hand or per-seat hidden holding.
- A data-driven graph rule, clash formula, scoring expression, or selector.
- A TypeScript graph traversal that computes scoring or legality.
- A generic faction engine, control tracker, or pathfinding service.

## ADR need

ADR required? no

Reason:

- No architecture, replay/hash semantics, data policy, kernel boundary, browser
  authority, bot policy class, or public/private content policy changes are
  proposed.

ADR becomes required if a future proposal adds generic graph, control,
faction/asymmetry, budget, visibility/export, or data-behavior helpers, or any
new kernel vocabulary.

## Review checklist

- `game-stdlib::board_space` is audited not applicable.
- Third-game hard gates are not triggered by Frontier Control.
- Second-use role/faction and budget pressure is recorded without promotion.
- Graph/control/asymmetry first uses are recorded local-only.
- No game noun enters `engine-core`.
- No helper is added to `game-stdlib`.
- No untyped behavior language is created.
- Static data remains typed content/parameters/metadata/fixtures/traces/reports
  only.
- Golden traces are preserved because no prior game migration occurs.
- Replay/hash and serialization impacts are recorded.
- Visibility/no-leak impacts are covered for future implementation tickets.
- UI/effect and bot impacts are covered for future implementation tickets.
- ADR need is explicit.

# race_to_n Mechanics Inventory

Game ID: `race_to_n`

Roadmap stage/gate: `Gate 1`

Rules version: `race_to_n-rules-v1`

Prepared by: `Codex`

Last updated: 2026-06-05

## Purpose

This inventory applies `docs/MECHANIC-ATLAS.md` to `race_to_n`. It records
game-local mechanic shapes, primitive reuse, repeated-shape pressure,
extraction/defer decisions, and effects/UI/bot/benchmark implications.

The mechanic inventory is evidence. It is not permission to generalize.

## Mechanic inventory categories

| Category | Game-local description | Evidence in rules | Current status | Notes |
|---|---|---|---|---|
| topology/spatial model | No spatial topology; one public numeric counter. | `R-COMP-001`, `R-SETUP-001` | `local-only` | No positions, maps, routes, regions, boards, piles, or tracks. |
| component/zone model | Two public seats and one public counter; no zones or hidden areas. | `R-COMP-001`, `R-COMP-002`, `R-VIS-001` | `local-only` | Components are game-local vocabulary only. |
| action shape | Flat choice among 1, 2, or 3, capped by remaining distance to 21. | `R-ACTION-001` | `local-only` | Rust generates the legal list; UI only renders it. |
| turn/phase model | Alternating single-action turns; no rounds, reactions, cleanup windows, or simultaneous choices. | `R-TURN-001`, `R-TURN-002` | `local-only` | Terminal action stops the alternation. |
| randomness/chance | No game-rule randomness. | `R-RNG-001` | `local-only` | Bot RNG exists outside game rules. |
| visibility/hidden information | Perfect information; all state and legal choices are public. | `R-VIS-001` | `local-only` | Visibility tests can record no-leak as not applicable with rationale. |
| resource/accounting | One increasing counter and winner-only outcome; no score ledger or resource economy. | `R-COMP-001`, `R-SCORE-001`, `R-END-001` | `local-only` | The counter is not a shared engine resource primitive. |
| movement/capture/placement | Not applicable; no entities move, capture, or occupy spaces. | `R-SCOPE-001` | `local-only` | No board-like mechanic. |
| pattern/line/directional scanning | Not applicable. | `R-SCOPE-001` | `local-only` | No pattern recognition. |
| commitment/reveal | Not applicable; choices are immediate and public. | `R-TURN-001`, `R-VIS-001` | `local-only` | No hidden commitment or reveal. |
| reaction/window/pending response | Not applicable; no response windows. | `R-TURN-001`, `R-TURN-002` | `local-only` | Wrong-seat submissions are diagnostics, not reactions. |
| scoring/outcome | Instant win when a valid action reaches 21; no tie. | `R-SCORE-001`, `R-END-001` | `local-only` | Outcome proof is simple terminal validation. |
| semantic effect shape | Effects should record match start, counter advanced, turn changed, invalid/stale diagnostic, and game ended. | `R-TURN-002`, `R-RESTRICT-001`, `R-END-001` | `local-only` | Effects are needed for logs, UI smoke, replay, and bot trace context. |
| UI interaction pattern | Direct button selection from Rust-supplied legal additions. | `R-ACTION-001`, `R-VIS-001` | `local-only` | No TypeScript legality and no Gate 3 shell scope. |
| bot policy pattern | Level 0 random legal bot chooses uniformly from Rust-supplied legal actions using deterministic bot RNG. | `R-ACTION-001`, `R-RNG-001` | `local-only` | Bot must not bypass validation. |
| benchmark/performance pressure | Legal-action generation, apply, public view/effect filtering, serialization/replay, random playout, and bot decision latency. | `R-ACTION-001`, `R-END-001`, `R-RNG-002` | `local-only` | Gate 1 uses this tiny game to measure plumbing overhead. |

## Repeated-shape comparison

| Mechanic shape | Already appears in | Same shape? | Similarities | Differences | Required next step |
|---|---|---:|---|---|---|
| tiny numeric turn race | `docs/MECHANIC-ATLAS.md` initial row for `race_to_n` | yes | This is the first official use. | No prior implementation exists. | none |

## Second-use note

| Shape | First game | Second game | Candidate? | Rationale | Ledger/atlas update needed? |
|---|---|---|---:|---|---:|
| tiny numeric turn race | `race_to_n` | not applicable | no | First official use only; keep local. | no |

## Third-use hard-gate warning

| Shape | Games exerting pressure | Third-use? | Gate cleared? | Evidence |
|---|---|---:|---:|---|
| tiny numeric turn race | `race_to_n` | no | not applicable | `docs/MECHANIC-ATLAS.md#initial-atlas-table` records `local-only`. |

## Primitives reused

| Primitive | Source | Why reused | Rule IDs covered | Tests proving compatibility | Notes |
|---|---|---|---|---|---|
| none | not applicable | First-use local mechanic. | not applicable | not applicable | `game-stdlib` remains unused. |

## Local mechanics

| Local mechanic | Why local | Extraction risk | Rule IDs | Tests/traces | Notes |
|---|---|---|---|---|---|
| target counter race | First official use and too small to justify a primitive. | low | `R-COMP-001`, `R-ACTION-001`, `R-END-001` | Future rule tests, traces, replay, simulation, benchmarks. | Must not create a general counter/resource helper. |
| target-bounded flat additions | First official use; simple game-local validation. | low | `R-ACTION-001`, `R-RESTRICT-001` | Future rule tests at 18/19/20 and invalid diagnostic trace. | Legal actions stay Rust-owned. |

## Primitive candidates

| Candidate | Status | Games exerting pressure | Required next step | Blocker? |
|---|---|---|---|---:|
| tiny numeric turn race helper | `local-only` | `race_to_n` | none | no |

## Extraction or defer rationale

| Shape | Decision | Rationale | Back-port needed? | Trace impact | Benchmark impact |
|---|---|---|---:|---|---|
| tiny numeric turn race | local | First use only; extracting now would violate the atlas rule against generalizing from one game. | no | none | Benchmarks should measure local implementation and generic engine overhead only. |

## Effects, UI, and bot notes

| Area | Requirement | Rule IDs | Notes |
|---|---|---|---|
| semantic effects | Emit viewer-safe effects for match start, counter advance, turn change, diagnostics, and terminal outcome. | `R-TURN-002`, `R-RESTRICT-001`, `R-END-001` | Later tickets define exact effect envelopes. |
| UI interaction pattern | Render Rust-supplied legal additions as direct controls. | `R-ACTION-001`, `R-VIS-001` | No TypeScript legality. |
| Rust-generated previews | Preview needs are minimal; future preview may show resulting counter and terminal flag. | `R-ACTION-001`, `R-END-001` | If omitted in Gate 1, UI still must not invent legality. |
| bot policy pattern | Level 0 random legal bot chooses from legal paths and submits through normal validation. | `R-ACTION-001`, `R-RNG-001` | No strategy authority in bot. |
| visibility/no-leak | All state is public; no hidden-info leak surface exists. | `R-VIS-001` | Record as not applicable in later evidence with rationale. |
| benchmark pressure | Tiny action branch highlights overhead in action generation, apply, view/effects, replay, serialization, playout, and bot selection. | `R-ACTION-001`, `R-RNG-002` | Stage 1 budget expects high native throughput. |

## Required repo atlas update

| Update target | Required? | Reason | Owner |
|---|---:|---|---|
| `docs/MECHANIC-ATLAS.md` | no | The initial atlas already lists `tiny numeric turn race` -> `race_to_n` as `local-only`; GAT1RACTON-014 owns final confirmation. | GAT1RACTON-014 |
| `PRIMITIVE-PRESSURE-LEDGER.md` | no | No repeated shape or third-use pressure. | not applicable |
| ADR | no | No architecture, replay/hash, visibility, data-policy, or kernel-boundary change in this ticket. | not applicable |

## Anti-patterns

The following are forbidden:

- generalizing from this one game;
- promoting game nouns into `engine-core`;
- creating a universal behavior language;
- hiding rule behavior in static data;
- adding YAML for convenience;
- using mechanic tags as a substitute for rule coverage;
- treating a UI pattern as proof of rule correctness.

## Review checklist

- All mechanic atlas categories are filled.
- Every not-applicable category has a rationale.
- Repeated shapes are compared honestly.
- Second-use candidate status is recorded.
- Third-use hard gate is explicit and not applicable.
- `engine-core` remains noun-free.
- Static data remains typed content/parameters/metadata/fixtures/traces/reports only.
- Effects, UI, bot, visibility, and benchmark impacts are recorded.
- Repo atlas and primitive-pressure ledger updates are identified.

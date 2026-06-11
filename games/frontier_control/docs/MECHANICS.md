# frontier_control Mechanics Inventory

Game ID: `frontier_control`

Roadmap stage/gate: Gate 13

Rules version: `frontier-control-rules-v1`

Prepared by: Codex

Last updated: 2026-06-11

## Purpose

This inventory records Frontier Control's game-local mechanic shapes and the
extraction decisions summarized in [PRIMITIVE-PRESSURE-LEDGER.md](PRIMITIVE-PRESSURE-LEDGER.md).
It is evidence for the mechanic atlas; it is not permission to promote graph,
control, faction, or movement nouns into shared infrastructure.

## Mechanic inventory categories

| Category | Game-local description | Evidence in rules | Current status | Notes |
|---|---|---|---|---|
| topology/spatial model | A typed site/trail graph with adjacency-constrained movement and supply connectivity. | `FC-COMP-003`, `FC-COMP-004`, `FC-CTRL-001`, `FC-COMP-010` | local-only | First official graph-map use; no `engine-core` or `game-stdlib` helper. |
| component/zone model | Public sites hold guards, crews, forts, stakes, and stake values. | `FC-COMP-005` through `FC-COMP-010` | local-only | All components are public. |
| action shape | Flat Rust legal leaves with faction-specific vocabularies. | `FC-ACT-001` through `FC-ACT-011` | local-only | No progressive construction or TypeScript legality. |
| turn/phase model | Alternating faction action phases with a budget, auto-end on budget exhaustion, and scoring after the Garrison turn. | `FC-TURN-001` through `FC-TURN-008` | repeated-shape candidate | Second official multi-action-budget shape after Flood Watch; kept local. |
| randomness/chance | No game-rule randomness; setup is typed content and command-stream deterministic. | `FC-RNG-001` through `FC-RNG-004` | rejected/deferred with rationale | Bot seed is bot infrastructure, not rule randomness. |
| visibility/hidden information | Perfect information; one public projection is equivalent for seats and observer. | `FC-VIS-001` through `FC-VIS-004` | local-only | Hidden information is explicitly not applicable. |
| resource/accounting | Public action budget, cumulative faction scores, fort points, and supplied stake values. | `FC-SCORE-*`, `FC-COMP-011`, `FC-COMP-012` | local-only | Scoring formulas are Rust behavior. |
| movement/capture/placement | March, patrol, immediate clashes, stake placement, dismantle, muster, and reinforce. | `FC-ACT-002` through `FC-ACT-008`, `FC-CTRL-*` | local-only | First official graph movement and deterministic contest shape. |
| pattern/line/directional scanning | Not applicable. | `FC-OOS-005` | rejected/deferred with rationale | No rectangular board, ray scan, alignment, or line pattern. |
| commitment/reveal | Not applicable. | `FC-VIS-004`, `FC-OOS-002` | rejected/deferred with rationale | No secret choices or reveal timing. |
| reaction/window/pending response | Not applicable. | `FC-AMB-003`, `FC-OOS-004` | rejected/deferred with rationale | Clashes resolve immediately inside the mover's command. |
| scoring/outcome | Final scheduled scoring round compares one numeric track; tied finals go to Garrison. | `FC-SCORE-*`, `FC-TERM-*` | local-only | Faction-specific scoring formulas, not asymmetric victory conditions. |
| semantic effect shape | Movement, clash, stake, muster, reinforce, turn end, round scoring, and terminal effects. | `FC-CTRL-*`, `FC-SCORE-*`, `FC-TERM-*` | local-only | Drives logs, animation, replay, and outcome text. |
| UI interaction pattern | Graph-map renderer plus direct legal action controls from Rust action leaves. | `FC-ACT-*`, `FC-VIS-*` | local-only | Browser renders Rust-projected supply and legality. |
| bot policy pattern | Per-faction Level 0 random and Level 1 public-priority policies. | `FC-BOT-*` | local-only | Distinct policy IDs and faction-specific decision order. |
| benchmark/performance pressure | Legal tree, apply, supply scoring, projection, replay/export, bots, and playout throughput. | `FC-RNG-*`, `FC-BOT-*`, `FC-SCORE-*` | local-only | Covered by `BENCHMARKS.md`. |

## Repeated-shape comparison

| Mechanic shape | Already appears in | Same shape? | Similarities | Differences | Required next step |
|---|---|---:|---|---|---|
| multi-action turn budget | `flood_watch` | yes | Budget appears in phase state, legal action metadata, and no-stall `end_turn`. | Flood Watch has cooperative environment automation; Frontier Control alternates factions and scores after Garrison. | keep local; third-use hard gate is armed for Gate 14. |
| role or faction modified action effects | `flood_watch` | no | Both games have identity-specific action consequences. | Flood Watch roles modify a shared cooperative action set; Frontier Control factions have disjoint action sets and clash/scoring rules. | keep local and record as related-but-distinct. |
| shared outcome | `flood_watch` | no | Both expose terminal public rationale. | Frontier Control has a competitive per-faction winner and Garrison tiebreak. | no shared-outcome pressure. |
| reaction or pending response | prior reaction-window games | no | Clashes can feel like contests. | No responder or pending decision exists; resolution is immediate. | no reaction-window pressure. |

## Local mechanics

| Local mechanic | Why local | Extraction risk | Rule IDs | Tests/traces | Notes |
|---|---|---|---|---|---|
| graph topology and adjacency legality | First official use; game nouns are forbidden in `engine-core`. | high | `FC-COMP-003`, `FC-COMP-004`, `FC-CTRL-001`, `FC-RESTRICT-005` | `tests/rules.rs`, `tests/property.rs`, `non-adjacent-move-diagnostic.trace.json` | `boundary-check.sh` guards `faction` and `territory`; graph remains too generic for the pattern. |
| supply connectivity scoring | First official connectivity-scoring use. | high | `FC-COMP-010`, `FC-SCORE-PROSPECTOR-SUPPLY` | `tests/rules.rs`, `supply-cut-scores-zero.trace.json` | Browser receives supplied/cut projection from Rust. |
| deterministic clash resolution | First official asymmetric contest shape. | medium | `FC-CTRL-002`, `FC-CTRL-003`, `FC-CTRL-004` | clash traces and rule tests | No response window. |
| faction-specific action sets and scoring | Gate 13 proof target. | high | `FC-ACT-*`, `FC-SCORE-*`, `FC-BOT-*` | rule, bot, visibility, and simulation tests | Stays in `games/frontier_control`. |
| multi-action budget | Second official use after Flood Watch. | medium | `FC-TURN-*`, `FC-SCORE-ACTION-BUDGET` | budget traces and simulation | Third official use must revisit promotion/defer. |

## Third-use hard-gate warning

| Shape | Games exerting pressure | Third-use? | Gate cleared? | Evidence |
|---|---|---:|---:|---|
| multi-action turn budget | `flood_watch`, `frontier_control` | no | not applicable | [PRIMITIVE-PRESSURE-LEDGER.md](PRIMITIVE-PRESSURE-LEDGER.md) |
| graph topology / adjacency legality | `frontier_control` | no | not applicable | [PRIMITIVE-PRESSURE-LEDGER.md](PRIMITIVE-PRESSURE-LEDGER.md) |
| site control / deterministic contest | `frontier_control` | no | not applicable | [PRIMITIVE-PRESSURE-LEDGER.md](PRIMITIVE-PRESSURE-LEDGER.md) |
| faction-asymmetric action sets and scoring | `frontier_control` | no | not applicable | [PRIMITIVE-PRESSURE-LEDGER.md](PRIMITIVE-PRESSURE-LEDGER.md) |

## Effects, UI, and bot notes

| Area | Requirement | Rule IDs | Notes |
|---|---|---|---|
| semantic effects | crew march, guard patrol, clash, stake placed, stake dismantled, crew mustered, guard reinforced, turn ended, round scored, terminal | `FC-CTRL-*`, `FC-SCORE-*`, `FC-TERM-*` | All effects are public. |
| UI interaction pattern | direct action buttons grouped by Rust action path plus graph-map site controls | `FC-ACT-*`, `FC-VIS-*` | No TypeScript adjacency, connectivity, clash, or score computation. |
| Rust-generated previews | not implemented for Gate 13 | `FC-ACT-*` | Current legal tree and public view are sufficient; future preview work must remain Rust-owned. |
| bot policy pattern | faction-specific Level 1 priority bots and random legal baseline | `FC-BOT-*` | Evidence in [AI.md](AI.md) and [BOT-STRATEGY-EVIDENCE-PACK.md](BOT-STRATEGY-EVIDENCE-PACK.md). |
| visibility/no-leak | equivalent public view for all viewers; hidden-info not applicable | `FC-VIS-*` | Covered by visibility tests and trace not-applicable markers. |
| benchmark pressure | legal actions, apply, supply scoring, projection, replay/export, bot decisions, playout | `FC-RNG-*`, `FC-BOT-*`, `FC-SCORE-*` | Covered by [BENCHMARKS.md](BENCHMARKS.md). |

## Required repo atlas update

| Update target | Required? | Reason | Owner |
|---|---:|---|---|
| `docs/MECHANIC-ATLAS.md` | yes | Gate 13 records first-use and second-use mechanic pressure. | Gate 13 |
| `PRIMITIVE-PRESSURE-LEDGER.md` | yes | Frontier-specific local-only and repeated-shape decisions are recorded there. | Gate 13 |
| ADR | no | No primitive promotion, kernel vocabulary change, DSL, or replay migration. | none |

## Review checklist

- All mechanic atlas categories are filled.
- Repeated shapes are compared without promotion.
- `engine-core` remains noun-free.
- Static data remains typed content/parameters/metadata/fixtures/traces/reports only.
- Effects, UI, bot, visibility, and benchmark impacts are recorded.

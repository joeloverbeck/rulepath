# flood_watch Mechanics Inventory

Game ID: `flood_watch`

Roadmap stage/gate: Gate 12

Rules version: `flood-watch-rules-v1`

Prepared by: Codex

Last updated: 2026-06-11

## Purpose

This inventory records Flood Watch's game-local mechanic shapes and the extraction decisions already summarized in [PRIMITIVE-PRESSURE-LEDGER.md](PRIMITIVE-PRESSURE-LEDGER.md).

## Mechanic inventory categories

| Category | Game-local description | Evidence in rules | Current status | Notes |
|---|---|---|---|---|
| topology/spatial model | Five flat districts, no adjacency or movement. | `FW-COMP-003`, `FW-AMB-004` | local-only | Gate 13 owns graph pressure. |
| component/zone model | Public districts, public roles, hidden event deck, public drawn history. | `FW-COMP-*` | local-only | Event-deck order stays game-local. |
| action shape | Flat legal leaves: bail, reinforce, forecast, end turn. | `FW-ACT-*` | local-only | Existing generic action tree is reused. |
| turn/phase model | Multi-action turn budget followed by Rust environment automation. | `FW-TURN-*`, `FW-ENV-*` | local-only | No synthetic environment actor. |
| randomness/chance | Seeded setup shuffle; deterministic event draws after setup. | `FW-RNG-001`, `FW-RNG-002` | local-only | No runtime random sampling. |
| visibility/hidden information | Hidden deck order, public composition counts, public forecast reveal. | `FW-VIS-*` | local-only | No per-seat private hand. |
| resource/accounting | Flood levels, levee stacks, action budget, remaining composition counts. | `FW-SCORE-*` | local-only | No points or economy. |
| movement/capture/placement | No movement or capture; reinforce places prevention counters. | `FW-ACT-003` | local-only | Placement is district-local. |
| pattern/line/directional scanning | Not applicable. | `FW-OOS-002` | rejected/deferred with rationale | No graph or pattern scan. |
| commitment/reveal | Forecast publicly reveals the next event. | `FW-COMP-008`, `FW-VIS-003` | local-only | Not a private commitment. |
| reaction/window/pending response | Not applicable. | `FW-OOS-003` | rejected/deferred with rationale | Environment is automation. |
| scoring/outcome | Shared win/loss only. | `FW-END-*`, `FW-SCORE-005` | local-only | No individual ranking. |
| semantic effect shape | Public action, event, terminal, and environment effects. | `FW-ENV-*`, `FW-VIS-*` | local-only | Drives UI animation/logs. |
| UI interaction pattern | Direct district/action buttons plus effect log and public status. | `FW-ACT-*`, `FW-VIS-*` | local-only | Rust owns legality. |
| bot policy pattern | Random legal and Level 1 public-priority cooperative bot. | `FW-BOT-*` | local-only | No hidden sampling or MCTS. |
| benchmark/performance pressure | Legal tree, apply, projection, replay/export, playout. | `FW-RNG-*`, `FW-BOT-*` | local-only | Benchmarked in `BENCHMARKS.md`. |

## Repeated-shape comparison

| Mechanic shape | Already appears in | Same shape? | Similarities | Differences | Required next step |
|---|---|---:|---|---|---|
| hidden deck redaction | `poker_lite`, `plain_tricks`, `masked_claims` | no | Browser payloads must not leak hidden order. | Flood Watch has no per-seat private hand; only shared hidden deck order. | keep local; no primitive promotion. |
| multi-action budget | none as official repeated shape | no | Budget appears in action metadata and turn flow. | No existing official repeated budget primitive. | keep local. |
| shared cooperative outcome | none as official repeated shape | no | Terminal outcome is public. | No opponent winner or split. | keep local. |
| environment automation | none as official repeated shape | no | Effects are Rust-generated. | No reaction actor or timer. | keep local. |

## Local mechanics

| Local mechanic | Why local | Extraction risk | Rule IDs | Tests/traces | Notes |
|---|---|---|---|---|---|
| role-modified action strength | First official use; role nouns are game-local. | medium | `FW-COMP-002`, `FW-ACT-002`, `FW-ACT-003` | role-power traces | `boundary-check.sh` now guards `role` in `engine-core`. |
| scenario event composition | First official Flood Watch shape. | medium | `FW-VAR-003`, `FW-SETUP-002` | serialization tests, fixture-check | Static data remains typed content only. |
| event-deck automation | First official cooperative event pressure. | medium | `FW-ENV-*` | environment traces | No TypeScript timer or synthetic actor. |
| shared terminal outcome | First official cooperative result. | low | `FW-END-*` | standard-win, loss traces | Outcome remains local. |

## Effects, UI, and bot notes

| Area | Requirement | Rule IDs | Notes |
|---|---|---|---|
| semantic effects | bail, reinforce, forecast, environment, draw, levee, rise, terminal | `FW-ENV-*` | Effects are viewer-safe. |
| UI interaction pattern | direct buttons from Rust legal tree | `FW-ACT-*` | TypeScript presents only. |
| Rust-generated previews | action metadata and public view; no hidden previews | `FW-VIS-*` | No TypeScript legality. |
| bot policy pattern | Level 1 public priority | `FW-BOT-*` | Evidence in bot docs. |
| visibility/no-leak | no undrawn deck order in browser/replay/bot | `FW-VIS-002`, `FW-RNG-003` | Covered by visibility and WASM tests. |
| benchmark pressure | playout, apply, legal actions, projection/export | `FW-RNG-*` | Covered by native benchmarks. |

## Required repo atlas update

| Update target | Required? | Reason | Owner |
|---|---:|---|---|
| `docs/MECHANIC-ATLAS.md` | yes | Flood Watch rows already added for Gate 12 pressure. | Gate 12 |
| `PRIMITIVE-PRESSURE-LEDGER.md` | yes | Local-only extraction decisions recorded. | Gate 12 |
| ADR | no | No primitive promotion or foundation migration. | none |

## Review checklist

- All mechanic atlas categories are filled.
- Repeated shapes are compared without promotion.
- `engine-core` remains noun-free.
- Static data remains typed content/parameters/metadata/fixtures/traces/reports only.
- Effects, UI, bot, visibility, and benchmark impacts are recorded.

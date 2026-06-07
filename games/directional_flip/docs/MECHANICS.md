# Directional Flip Mechanics Inventory

Game ID: `directional_flip`

Roadmap stage/gate: Gate 6 directional-flip official game

Rules version: `directional_flip-rules-v1`

Last updated: 2026-06-07

## Purpose

This inventory records Directional Flip's game-local mechanic shapes and primitive-pressure posture. It is evidence for [../../../docs/MECHANIC-ATLAS.md](../../../docs/MECHANIC-ATLAS.md) and closes the local third-use review through [PRIMITIVE-PRESSURE-LEDGER.md](PRIMITIVE-PRESSURE-LEDGER.md).

Directional Flip is the third official fixed-grid public board game after `three_marks` and `column_four`. The repeated rectangular coordinate and directional-ray pressure is real, but Gate 6 keeps the coordinate, ray, legality, flip, pass, preview, and effect behavior local to `games/directional_flip`.

## Mechanic Inventory

| Category | Game-local description | Evidence | Current status | Notes |
|---|---|---|---|---|
| topology/spatial model | Fixed public 8 by 8 board, cells `r1c1` through `r8c8`, row 1 at the top. | [RULES.md](RULES.md), `ids.rs`, setup/rule tests | `third-use pressure reviewed` | Ledger defer-rejects shared helper promotion for Gate 6. |
| component/zone model | Sixty-four public cells, two seats, public discs, no hands/decks/private zones. | `setup.rs`, [RULE-COVERAGE.md](RULE-COVERAGE.md) | `local-only` | Perfect-information board only. |
| action shape | Flat Rust actions `place/rNcM`; `pass/forced` appears only when Rust proves no placement exists. | `actions.rs`, forced-pass traces | `local-only` | Browser never synthesizes placement or pass actions. |
| turn/phase model | Alternating active seat; normal placement or explicit forced pass until terminal. | `rules.rs`, `forced-pass.trace.json`, `double-pass-terminal.trace.json` | `local-only` | No simultaneous or reaction windows. |
| randomness/chance | Rules use no randomness; bot seed is outside game-rule state. | [RULES.md](RULES.md), replay tests | `local-only` | Replays record resolved commands. |
| visibility/hidden information | Perfect information; public view has empty hidden fields and private view not applicable. | `visibility.rs`, WASM smoke, no-leak E2E | `local-only` | Internals, rankings, and raw score arrays still stay out of public surfaces. |
| resource/accounting | Public disc counts for both seats; final count decides win/draw. | `DF-SCORE-*`, terminal traces | `local-only` | Counts update after placement and flips. |
| movement/capture/placement | Place one disc on an empty legal cell, then flip bracketed opposing discs. | `rules.rs`, multi-direction traces | `local-only` | This is not area fill and not Column Four gravity. |
| pattern/directional scanning | Eight ordered rays: north, northeast, east, southeast, south, southwest, west, northwest; each qualifying run flips nearest to farthest. | `DF-FLIP-004`, `DF-EFFECT-002`, [PRIMITIVE-PRESSURE-LEDGER.md](PRIMITIVE-PRESSURE-LEDGER.md) | `local-only with third-use review` | Ordering is replay/hash and animation contract. |
| commitment/reveal | Not applicable; all board state and choices are public. | `DF-VIEW-001` | `local-only` | No secret commitments. |
| reaction/window/pending response | Not applicable; one actor acts at a time. | `DF-ACTION-*` | `local-only` | Forced pass is an explicit normal action, not an implicit window. |
| scoring/outcome | Higher final disc count wins; equal final count draws. | `DF-SCORE-*`, `full-board-terminal.trace.json`, `draw.trace.json` | `local-only` | Double pass and no-continuation terminal paths both score publicly. |
| semantic effect shape | Placement accepted, disc placed, grouped flips, pass taken, active player changed, game ended, bot chose action. | `effects.rs`, WASM serializers, browser smoke | `local-only` | Effects drive logs, animation, replay, and bot rationale display. |
| UI interaction pattern | 8 by 8 keyboard grid, Rust legal/preview highlights, forced-pass control, replay projection, reduced-motion support. | [UI.md](UI.md), `DirectionalFlipBoard.tsx`, `directional-flip.smoke.mjs` | `local-only` | React presents Rust/WASM output only. |
| bot policy pattern | Level 0 random legal and Level 2-lite authored public-feature policy. | [AI.md](AI.md), [BOT-STRATEGY-EVIDENCE-PACK.md](BOT-STRATEGY-EVIDENCE-PACK.md) | `local-only` | No MCTS, Monte Carlo, ML, RL, or runtime LLM move selection. |
| benchmark/performance pressure | Legal actions, flip scanning, apply, public view, replay, serialization, playout, Level 0/2 bot decisions. | [BENCHMARKS.md](BENCHMARKS.md), thresholds | `local-only` | Thresholds are baseline-pending smoke floors. |

## Repeated-Shape Comparison

| Mechanic shape | Already appears in | Same shape? | Similarities | Differences | Decision |
|---|---|---:|---|---|---|
| fixed public grid occupancy | `three_marks`, `column_four` | yes as broad shape | stable cells, public occupancy, row/column display, replay projection | 8 by 8 top-origin board; direct cell placements plus pass | third-use reviewed; keep local |
| directional traversal | `column_four` | partial | bounded rays and deterministic order matter | Column Four scans four win directions; Directional Flip scans eight candidate rays and grouped flip runs | keep local |
| direct placement controls | `three_marks` | partial | cells are direct action targets | Directional Flip requires bracketed flips and previews; not every empty cell is legal | keep local |
| terminal pattern/scoring | `race_to_n`, `three_marks`, `column_four` | broad only | Rust terminalizes and exposes public outcome | Directional Flip scores disc counts after double pass/full/no continuation | keep local |
| public bot rationale | `three_marks`, `column_four` | yes as UI contract | public prose only, no rankings | policy features are mobility/corner/frontier/count | keep local |

## Third-Use Hard-Gate Receipt

| Shape | Games exerting pressure | Gate decision | Evidence |
|---|---|---|---|
| rectangular coordinates and directional rays | `three_marks`, `column_four`, `directional_flip` | defer-reject shared helper for Gate 6 | [PRIMITIVE-PRESSURE-LEDGER.md](PRIMITIVE-PRESSURE-LEDGER.md) |
| fixed-grid public board renderer | `three_marks`, `column_four`, `directional_flip` | no shared UI primitive | each board has different controls/previews/animation |
| perfect-information no-leak posture | `race_to_n`, `three_marks`, `column_four`, `directional_flip` | shared checklist only | [../../../apps/web/e2e/NO-LEAK-A11Y-CHECKLIST.md](../../../apps/web/e2e/NO-LEAK-A11Y-CHECKLIST.md) |

## Primitives Reused

| Primitive | Source | Why reused | Evidence |
|---|---|---|---|
| action tree, command envelope, actor/viewer, seed, stable serialization | `engine-core` | Generic contracts own dispatch, replay identity, viewer identity, deterministic serialization. | `cargo test --workspace`, replay-check |
| web shell and WASM bridge operations | `crates/wasm-api`, `apps/web` | Existing browser boundary carries catalog, view, action, effects, bot, and replay operations. | `npm --prefix apps/web run smoke:wasm` |
| benchmark report tool | tools | Existing report/threshold lane checks operation floors. | Gate 2 workflow, [BENCHMARKS.md](BENCHMARKS.md) |

## Local Mechanics

| Local mechanic | Why local | Extraction risk | Rule IDs | Evidence |
|---|---|---|---|---|
| 8 by 8 coordinate ids and top-origin row order | Display, traces, UI, and previews are game contracts. | medium | `DF-SETUP-001` | setup tests, opening trace |
| bracketed eight-direction flip scan | This is the core game legality and apply policy. | high | `DF-LEGAL-*`, `DF-FLIP-*` | rule tests, multi-direction trace |
| forced pass command | Pass availability is rules-owned and replay-visible. | medium | `DF-ACTION-002`, `DF-PASS-*` | forced-pass and double-pass traces |
| exact placement previews | Preview set must equal apply set and effect order. | high | `DF-PREVIEW-001`, `DF-EFFECT-002` | preview trace, property tests, WASM smoke |
| Level 2-lite policy | Strategy is game-specific and public-feature bounded. | low | `DF-BOT-002` | bot tests, strategy docs |
| Directional Flip board renderer | Product-facing UI for legal cells, previews, pass, and flips. | low | `DF-UI-*` | browser smoke |

## Extraction Or Defer Rationale

| Shape | Decision | Rationale | Trace impact | Benchmark impact |
|---|---|---|---|---|
| rectangular coordinate/ray helper | defer-reject | A helper narrow enough to be behavior-free was not proven before the 8 by 8 implementation; flags for origins, direction order, and policy would be too risky. | none now | future reconsideration needs measured local evidence |
| legal flip scan | local | It owns legality, pass, preview, and effects. | protected locally | covered by local benches |
| board UI grid | local | Each game has different legal controls and previews. | none | browser smoke only |
| bot mobility/corner policy | local | Public strategy and rationale are game-specific. | bot effect hashes stay local | bot-decision benches stay local |

## Review Checklist

- `engine-core` remains noun-free.
- TypeScript renders Rust choices, previews, effects, and views only.
- `MECHANICS.md` links the primitive-pressure ledger.
- Third-use fixed-grid/ray pressure is recorded and explicitly deferred.
- No helper is promoted to `game-stdlib` in Gate 6.
- Replay/hash ordering remains owned by Directional Flip rules and traces.

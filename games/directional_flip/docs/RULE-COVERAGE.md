# Directional Flip Rule Coverage Matrix

Game ID: `directional_flip`

Rules version: `directional_flip-rules-v1`

Data/manifest version: `1`

Engine version: `0.1.0`

Prepared by: `Codex`

Last updated: 2026-06-07

## Purpose

This matrix maps every stable rule ID in `RULES.md` to implementation and evidence. Rust tests, golden traces, replay checks, fixture checks, simulations, and benchmarks are primary evidence; browser smoke proves integration only.

## Status labels

| Status | Meaning |
|---|---|
| covered | Implementation and required evidence exist. |
| covered-by-trace | Golden trace or replay evidence is the primary proof. |
| not-applicable | The surface truly does not apply, with rationale. |
| intentionally-deferred | Deferred by a documented gate decision. |
| unsupported | Explicitly not implemented for this variant. |

## Primary Evidence Files

| Evidence | Coverage role |
|---|---|
| `games/directional_flip/src/setup.rs` | deterministic setup and fixed variant |
| `games/directional_flip/src/actions.rs` | Rust legal placement/pass action tree and previews |
| `games/directional_flip/src/rules.rs` | validation, flip resolution, pass, terminal, and scoring behavior |
| `games/directional_flip/src/effects.rs` | viewer-safe semantic effects |
| `games/directional_flip/src/visibility.rs` | Rust-projected public board view and no-leak surface |
| `games/directional_flip/src/bots.rs` | Level 0 and Level 2-lite bot policies |
| `games/directional_flip/src/replay_support.rs` | replay hashes and step projections |
| `games/directional_flip/tests/rules.rs` | named setup, action, legality, flip, pass, terminal, and score tests |
| `games/directional_flip/tests/property.rs` | random legal sequence invariants |
| `games/directional_flip/tests/replay.rs` | golden trace and replay hash checks |
| `games/directional_flip/tests/visibility.rs` | public-view and no-leak checks |
| `games/directional_flip/tests/bots.rs` | bot legality, determinism, and explanations |
| `games/directional_flip/tests/golden_traces/*.trace.json` | command, diagnostic, pass, terminal, preview, bot, and WASM replay evidence |
| `games/directional_flip/docs/PRIMITIVE-PRESSURE-LEDGER.md` | primitive-pressure governance evidence |
| `games/directional_flip/docs/SOURCES.md` | neutral naming and ambiguity decisions |
| `apps/web/scripts/smoke-load-wasm.mjs` | WASM bridge smoke for catalog, previews, actions, bot, diagnostics, replay |
| `games/directional_flip/benches/directional_flip.rs` | native hot-path benchmark evidence |

## Rule Coverage Matrix

| Rule ID | Rule summary | Implementation | Evidence | Status | Notes |
|---|---|---|---|---|---|
| `DF-ACTION-001` | Legal placement actions appear when placements exist. | `actions.rs`, `rules.rs`. | `cargo test -p directional_flip`; `opening-legal-move.trace.json`; simulation. | covered | Rust owns legal target generation. |
| `DF-ACTION-002` | Forced pass is the only action when no placement exists. | `actions.rs`, `rules.rs`. | `forced-pass.trace.json`; rule tests; bot tests. | covered-by-trace | Explicit pass is replayed as a command. |
| `DF-ACTION-003` | Pass is absent while placement exists. | `legal_action_tree`, `legal_placements`. | action-tree rule tests; property tests. | covered | UI follows Rust choices only. |
| `DF-BOT-001` | Level 0 random bot validates through the command path. | `DirectionalFlipRandomBot`. | `cargo test -p directional_flip bots`; `bot-action.trace.json`. | covered | Random seed chooses among legal actions only. |
| `DF-BOT-002` | Level 2-lite bot is deterministic and command-path legal. | `DirectionalFlipLevel2Bot`. | `games/directional_flip/tests/bots.rs`; strategy evidence pack. | covered | No search, learning, or hidden data. |
| `DF-EFFECT-001` | Placement emits accepted/place/grouped-flip/turn/terminal effects as applicable. | `effects.rs`, `rules.rs`. | effect rule tests; `multi-direction-flip.trace.json`; WASM smoke. | covered | Effects are viewer-safe payloads. |
| `DF-EFFECT-002` | Grouped flip children use deterministic order. | `Direction::ALL`, `FlipEntry`. | `multi-direction-flip.trace.json`; effect/order tests. | covered-by-trace | Direction order is north through northwest, near to far. |
| `DF-FLIP-001` | Every bracketed disc in each qualifying direction flips. | `placement_flips`, `apply_action`. | rule tests; `multi-direction-flip.trace.json`; property tests. | covered | Applies all qualifying runs. |
| `DF-FLIP-002` | Skipped own discs do not create flips. | scan logic in `rules.rs`. | legality rule tests; property tests. | covered | Requires contiguous opposing run. |
| `DF-FLIP-003` | Indirect and non-line discs do not flip. | directional scan only. | rule tests; `preview-flip-set.trace.json`. | covered | No area-fill behavior. |
| `DF-FLIP-004` | Flip order is stable and documented. | ordered direction scan and `FlipEntry` order. | effect tests; replay hashes; `SOURCES.md`. | covered | Stabilizes replay and animation. |
| `DF-IP-001` | Public presentation uses neutral original naming. | docs, manifest, WASM catalog. | `SOURCES.md`; fixture-check; WASM smoke. | covered | Uses Directional Flip. |
| `DF-LEGAL-001` | Placement requires at least one bracketed line. | `validate_command`, `placement_flips`. | rule tests; `invalid-non-flipping-placement.trace.json`. | covered-by-trace | Empty cell alone is insufficient. |
| `DF-LEGAL-002` | Occupied targets reject. | `validate_command`. | `invalid-occupied-cell.trace.json`; rule tests. | covered-by-trace | State remains unchanged. |
| `DF-LEGAL-003` | Non-flipping targets reject. | `validate_command`. | `invalid-non-flipping-placement.trace.json`; rule tests. | covered-by-trace | Diagnostic is viewer-safe. |
| `DF-LEGAL-004` | Malformed/out-of-bounds cells reject. | segment parser in `rules.rs`. | parser/validation tests. | covered | Stable `r1c1` through `r8c8` coordinates. |
| `DF-LEGAL-005` | Stale commands reject. | freshness validation. | `stale-diagnostic.trace.json`; WASM smoke. | covered-by-trace | No mutation on stale token. |
| `DF-LEGAL-006` | Non-active actor rejects. | actor validation. | `non-active-seat-diagnostic.trace.json`; rule tests. | covered-by-trace | Turn ownership is Rust-enforced. |
| `DF-PASS-001` | Forced pass advances turn through normal command path. | `ValidatedAction::ForcedPass`, `apply_action`. | `forced-pass.trace.json`; rule tests. | covered-by-trace | Board is preserved. |
| `DF-PASS-002` | Double forced pass ends the game. | pass terminal branch. | `double-pass-terminal.trace.json`; rule tests. | covered-by-trace | Final score/outcome exposed publicly. |
| `DF-PREVIEW-001` | Preview flip set equals apply flip set. | shared flip collection in `actions.rs`/`rules.rs`. | `preview-flip-set.trace.json`; property tests; WASM smoke. | covered-by-trace | No parallel TS preview logic. |
| `DF-PRIM-001` | Primitive-pressure ledger decision is complete. | `PRIMITIVE-PRESSURE-LEDGER.md`. | governance doc; archived GAT6DIRFLI-002 outcome. | covered | Gate decision is documented. |
| `DF-REPLAY-001` | Replay/hash deterministic across export/import/step/reset. | `replay_support.rs`. | `cargo run -p replay-check -- --game directional_flip --all`; replay tests. | covered-by-trace | Covers command, pass, terminal, diagnostic, bot, and WASM traces. |
| `DF-SCORE-001` | Higher final disc count wins. | terminal score in `rules.rs`. | terminal rule tests; `full-board-terminal.trace.json`. | covered | Winner is public seat id. |
| `DF-SCORE-002` | Equal final disc count draws. | terminal score in `rules.rs`. | `draw.trace.json`; draw rule tests. | covered-by-trace | Winner is null. |
| `DF-SER-001` | Unknown and behavior-looking fields reject. | fixture/static-data parsers; trace validators. | fixture-check; serialization tests. | covered | Static data stays non-procedural. |
| `DF-SETUP-001` | Standard 8 by 8 setup has four center discs and first seat. | `setup.rs`, `ids.rs`, static data. | setup tests; `opening-legal-move.trace.json`; fixture-check. | covered | Only `directional_flip_standard` ships. |
| `DF-TERM-001` | Terminal action tree has no choices. | terminal checks in `actions.rs`/`rules.rs`. | terminal rule tests; `double-pass-terminal.trace.json`. | covered-by-trace | Terminal commands reject. |
| `DF-UI-001` | UI uses Rust choices/previews/effects and no TS legality. | WASM bridge and client types. | `apps/web/scripts/smoke-load-wasm.mjs`; GAT6DIRFLI-015 outcome. | covered | Full board renderer lands later. |
| `DF-UI-002` | Keyboard grid, forced pass control, reduced motion, and non-color-only state encoding pass smoke. | pending browser renderer/a11y smoke. | GAT6DIRFLI-018 ticket. | intentionally-deferred | Browser E2E/a11y smoke is explicitly owned by GAT6DIRFLI-018. |
| `DF-VIEW-001` | Public view contains no hidden/internal state. | `visibility.rs`, public effects, replay export, WASM JSON. | visibility tests; WASM smoke; fixture-check. | covered | Perfect information, but internals remain out of payloads. |

## Golden Trace Catalog

| Trace file | Purpose | Rule IDs covered | Expected result/hash evidence | Diagnostic coverage |
|---|---|---|---|---|
| `opening-legal-move.trace.json` | opening legal placement | `DF-SETUP-001`, `DF-ACTION-001`, `DF-REPLAY-001` | nonterminal hashes | none |
| `corner-capture.trace.json` | corner placement and flips | `DF-FLIP-001`, `DF-LEGAL-001` | board/view hashes | none |
| `multi-direction-flip.trace.json` | multiple directional flip groups | `DF-FLIP-001`, `DF-FLIP-004`, `DF-EFFECT-002` | ordered effect hashes | none |
| `preview-flip-set.trace.json` | preview/apply parity | `DF-PREVIEW-001`, `DF-FLIP-003` | action-tree and view hashes | none |
| `forced-pass.trace.json` | explicit forced pass | `DF-ACTION-002`, `DF-PASS-001` | pass effect hashes | none |
| `double-pass-terminal.trace.json` | double pass terminal | `DF-PASS-002`, `DF-TERM-001`, `DF-SCORE-001` | terminal hashes | none |
| `draw.trace.json` | equal count draw | `DF-SCORE-002`, `DF-REPLAY-001` | draw terminal hashes | none |
| `full-board-terminal.trace.json` | full-board terminal scoring | `DF-SCORE-001`, `DF-TERM-001` | terminal hashes | none |
| `stale-diagnostic.trace.json` | stale rejection | `DF-LEGAL-005` | diagnostic hash | `stale_action` |
| `non-active-seat-diagnostic.trace.json` | wrong actor rejection | `DF-LEGAL-006` | diagnostic hash | `not_active_seat` |
| `invalid-occupied-cell.trace.json` | occupied rejection | `DF-LEGAL-002` | diagnostic hash | `occupied_cell` |
| `invalid-non-flipping-placement.trace.json` | non-flipping rejection | `DF-LEGAL-003` | diagnostic hash | `non_flipping_placement` |
| `bot-action.trace.json` | bot-selected legal action | `DF-BOT-001`, `DF-BOT-002` | replay hashes for Rust-selected command | none |
| `wasm-exported.trace.json` | WASM-compatible command export shape | `DF-REPLAY-001`, `DF-UI-001` | replay hashes | none |

## Simulation And Benchmark Coverage

| Surface | Command/evidence | Rule IDs stressed | Status |
|---|---|---|---|
| simulation | `cargo run -p simulate -- --game directional_flip --games 1000` | `DF-ACTION-*`, `DF-LEGAL-*`, `DF-PASS-*`, `DF-TERM-*`, `DF-SCORE-*` | covered |
| replay drift gate | `cargo run -p replay-check -- --game directional_flip --all` | `DF-REPLAY-001`, trace hashes | covered |
| fixture/schema gate | `cargo run -p fixture-check -- --game directional_flip` | `DF-SER-001`, static-data integrity | covered |
| rule coverage gate | `cargo run -p rule-coverage -- --game directional_flip` | all rule IDs | covered |
| native benchmarks | `cargo bench -p directional_flip` | legal/apply/view/replay/bot/playout hot paths | covered |
| benchmark threshold report | `cargo run -p bench-report -- --game directional_flip --input <report>` | benchmark schema and threshold mapping | covered |

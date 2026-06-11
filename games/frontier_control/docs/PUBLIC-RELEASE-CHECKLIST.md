# frontier_control Public Release Checklist

Game ID: `frontier_control`

Public display name: `Frontier Control`

Implemented variant: `frontier_control_standard`, `frontier_control_highlands`

Release target: local preview / public web build

Rules version: `frontier-control-rules-v1`

Data/manifest version: `1`

Engine version: `engine-core-0.1.0`

Prepared by: Codex

Date: 2026-06-11

## Official-Game Contract Status

| Requirement | Status | Evidence |
|---|---|---|
| source notes | pass | [SOURCES.md](SOURCES.md) |
| formal rules | pass | [RULES.md](RULES.md) |
| rule coverage | pass | [RULE-COVERAGE.md](RULE-COVERAGE.md); `rule-coverage --game frontier_control` passed. |
| mechanic inventory | pass | [MECHANICS.md](MECHANICS.md) |
| implementation admission | pass | [GAME-IMPLEMENTATION-ADMISSION.md](GAME-IMPLEMENTATION-ADMISSION.md) |
| competent-player analysis | pass | [COMPETENT-PLAYER.md](COMPETENT-PLAYER.md) |
| bot evidence pack | pass | [BOT-STRATEGY-EVIDENCE-PACK.md](BOT-STRATEGY-EVIDENCE-PACK.md) |
| AI docs | pass | [AI.md](AI.md) |
| UI docs | pass | [UI.md](UI.md) |
| benchmarks | pass | [BENCHMARKS.md](BENCHMARKS.md); `cargo bench -p frontier_control` passed smoke thresholds. |
| primitive-pressure ledger | pass | [PRIMITIVE-PRESSURE-LEDGER.md](PRIMITIVE-PRESSURE-LEDGER.md) |

## Public Shipment Review

| Check | Status | Evidence/notes |
|---|---|---|
| original rules/help prose | pass | [RULES.md](RULES.md), [HOW-TO-PLAY.md](HOW-TO-PLAY.md), and `apps/web/public/rules/frontier_control.md`. |
| no copied component text or art | pass | Abstract SVG/CSS presentation; no imported assets. |
| trade-dress risk reviewed | pass | [SOURCES.md](SOURCES.md) records avoided terminology and originality rationale. |
| private licensed content excluded | pass | No private names, assets, traces, or data in public files. |
| fonts | pass | System font stack only. |

## Hidden-Information And Replay Safety

Frontier Control is perfect-information. Hidden information, private views,
redaction, and game-rule randomness are not applicable, but every public surface
was still checked for unsafe internals.

| Surface | Status | Evidence |
|---|---|---|
| public/browser payload | pass | `tests/visibility.rs`, `wasm-api` tests, `frontier-control.smoke.mjs`. |
| action tree | pass | Rust action tree only; browser renders legal choices. |
| effect log | pass | `smoke:effects` and Frontier E2E effect assertions. |
| DOM/test IDs/local storage | pass | Frontier E2E no-leak and storage checks. |
| replay export/import | pass | `replay-check --game frontier_control --all` and Frontier E2E export/import checks. |
| bot explanations | pass | `tests/bots.rs`, [AI.md](AI.md), and [BOT-STRATEGY-EVIDENCE-PACK.md](BOT-STRATEGY-EVIDENCE-PACK.md). |

## UI, Accessibility, And Legal-Only Controls

| Check | Status | Evidence |
|---|---|---|
| play-first UI | pass | `apps/web/src/components/FrontierControlBoard.tsx` and Frontier E2E. |
| TypeScript does not decide legality | pass | Controls derive from Rust action tree; no TS adjacency/connectivity/clash/scoring logic. |
| semantic effects drive feedback | pass | `smoke:effects` covers Frontier movement, clash, stake, scoring, and terminal effects. |
| reduced motion | pass | Frontier E2E verifies `.frontier-control-board.reduced`. |
| responsive layout | pass | Frontier E2E verifies mobile layout remains measurable. |
| shared rules and outcome surfaces | pass | `check-player-rules`, `check-outcome-explanations`, `rules-display.smoke.mjs`, and `outcome-explanation.smoke.mjs`. |

## Tests, Traces, Simulations, And Benchmarks

| Evidence | Status | Notes |
|---|---|---|
| unit/rule/property/replay/serialization/visibility/bot tests | pass | `cargo test --workspace` passed. |
| golden traces | pass | `replay-check --game frontier_control --all` accepted 17 traces. |
| fixtures | pass | `fixture-check --game frontier_control` passed. |
| simulation | pass with balance constraint | `simulate --game frontier_control --games 1000` completed; documented 1000-0 Garrison Level 1 result remains balance debt. |
| benchmarks | pass | `cargo bench -p frontier_control` passed smoke thresholds. |
| browser smoke | pass | `smoke:wasm`, `smoke:ui`, `smoke:effects`, and `smoke:e2e` passed. |
| docs/checks | pass | `check-doc-links`, `check-catalog-docs`, `check-player-rules`, and `check-outcome-explanations` passed. |

## Public Release Decision

Decision: release with explicit constraints

Rationale:
- Frontier Control satisfies the Gate 13 graph topology, control, asymmetry,
  per-faction UI, and per-faction bot proof without changing foundation
  boundaries.
- The browser and replay surfaces are viewer-safe for a perfect-information
  game, and TypeScript stays presentation-only.

Release constraints:
- Do not claim balanced Level 1 play until the documented Garrison-dominant
  simulation result is retuned and re-recorded.
- Do not promote graph/control/faction helpers without a future atlas decision
  or ADR.

## Final Checklist

- Official-game contract is satisfied.
- Rule/source/IP evidence is complete.
- No private licensed content ships publicly.
- Hidden-information no-leak surfaces are verified or explicitly not applicable.
- Replay/export safety is verified.
- UI is legal-only, accessible, reduced-motion-aware, and responsive.
- Bot explanations are safe.
- Dev inspector/public build boundary is safe.
- Tests, traces, simulations, replay, serialization, benchmarks, and browser smokes are green, with the balance constraint recorded.

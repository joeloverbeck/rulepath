# Gate 3 — WASM/Static Web Shell Requirements Spec

**Document status:** requirements/specification only. This is not an implementation plan, not a ticket set, not a patch, and not a code deliverable.  
**Target gate:** Gate 3 — WASM/static web shell.  
**Target game for this gate:** `race_to_n` only.  
**Prepared for:** `joeloverbeck/rulepath` at the user-supplied target commit listed below.

---

## Header

- Spec ID: `gate-3-wasm-static-web-shell`
- Roadmap stage: 1 (Gates 1–3 — `race_to_n`, trace/replay hardening, WASM shell; "plumbing proof")
- Roadmap build gate: Gate 3 — WASM/static web shell
- Status: Done
- Date: 2026-06-06
- Owner: `joeloverbeck`
- Target game for this gate: `race_to_n` only
- Target commit reviewed: `0f2e66f361a47ea2c19ead3c5e5959fb5cb137ab` (equals current `main` HEAD at reassessment)
- Authority order: the source-of-truth hierarchy enumerated in [Section 2](#2-source-of-truth-hierarchy). This spec is subordinate to the foundation set; where this spec and a foundation document disagree, the foundation document wins.

> This Header and the canonical sections immediately below (Deliverables, Work breakdown, Exit criteria, Acceptance evidence, FOUNDATIONS & boundary alignment, Forbidden changes, Documentation updates required, Sequencing, Assumptions) make the spec decomposable per the format in [`specs/README.md`](README.md). The detailed requirements that ground every item live in the numbered sections §1–§24 below; each canonical item cites the §N section that specifies it in full. Read the canonical sections to decompose; read §1–§24 for the binding detail.

---

## Deliverables

Concrete artifacts, grounded in `docs/ARCHITECTURE.md` and the existing tree. Each cites the detailed requirement section.

- **D1 — Typed TypeScript WASM client module.** Extract and harden the existing inline `RulepathApi` class in `apps/web/src/main.tsx` into a dedicated module (e.g. `apps/web/src/wasm/client.ts`) owning load/instantiate, memory, UTF-8 encode/decode, `last_output` reads, JSON parse, response normalization, and typed methods (§9.3). This is an extraction of working code, not a greenfield client.
- **D2 — Expanded WASM/API operations in `crates/wasm-api/src/lib.rs`.** Add feature/version report, `list_games`, and the replay operation group (export, import/load, step/reset) to the existing raw-ABI surface (§9.4), keeping the batched response contract (§9.5).
- **D3 — React shell regions.** App/bootstrap shell, WASM load/error boundary, game picker, match setup, active match view, Race-to-N renderer/status, legal-action controls, effect log, replay viewer controls, dev/debug panel, diagnostics (§7, §10, §12).
- **D4 — Reducer/state-machine state model** covering WASM load → catalog → setup → play → replay → dev/error (§11).
- **D5 — Play modes:** human-vs-bot, local hotseat, bot-vs-bot autoplay, replay viewer, safe local replay import/export (§8).
- **D6 — Base-aware static asset loading + `preview`/static-serve script + dist smoke** in `apps/web` (§18); replaces the current absolute `/wasm_api.wasm` fetch assumption.
- **D7 — Browser UI E2E smoke harness** (Puppeteer-preferred) exercising the rendered shell, not the low-level WASM API (§19.3–§19.4).
- **D8 — WASM/API smoke upgrade** covering version/features, list games, new match, view, action tree, apply action, bot turn, effects, and replay export/import (§19.2).
- **D9 — Accessibility / reduced-motion / no-leak review + smoke** (§17, §19.5, §19.6).
- **D10 — Documentation updates** including the `specs/README.md` index status flip (§20).

## Work breakdown

Bounded items, each a candidate AGENT-TASK, in dependency order. "Detail §" points to the binding requirement.

| ID | Item | Depends on | Detail § |
|---|---|---|---|
| WB1 | Extract typed TS WASM client module from the inline `RulepathApi` | — | §9.3, §3.4 |
| WB2 | Add feature/version report + `list_games` WASM ops | — | §9.4 |
| WB3 | Add replay export/import/step WASM ops, anchored on the Gate 2 trace/replay schema | WB2 | §9.4, §15, `docs/TRACE-SCHEMA-v1.md`, `games/race_to_n/src/replay_support.rs` |
| WB4 | Reducer/state-machine state model | WB1 | §11 |
| WB5 | App shell + game picker + match setup regions | WB1, WB4 | §7.1–§7.3, §10 |
| WB6 | Race-to-N renderer + action-tree-driven legal-action controls | WB4, WB5 | §12, §13 |
| WB7 | Effect log + effect-driven feedback + reduced motion | WB4, WB6 | §14 |
| WB8 | Play modes: human-vs-bot, hotseat, bot-vs-bot autoplay | WB6, WB7 | §8.1–§8.3 |
| WB9 | Replay viewer + safe local import/export UI | WB3, WB8 | §8.4–§8.5, §15 |
| WB10 | Dev/replay panel (viewer-safe, secondary) | WB7, WB9 | §16 |
| WB11 | Base-aware asset loading + `preview`/static-serve script + dist smoke | WB1 | §18 |
| WB12 | WASM/API smoke upgrade | WB2, WB3 | §19.2 |
| WB13 | Browser UI E2E smoke (Puppeteer) | WB8, WB9, WB11 | §19.3–§19.4 |
| WB14 | Accessibility + no-leak review/smoke | WB10, WB13 | §17, §19.5–§19.6 |
| WB15 | Documentation + `specs/README.md` index flip | all | §20 |

## Exit criteria

Mapped row-for-row to the ROADMAP Gate 3 Exit line (`docs/ROADMAP.md` §5): *"static site plays the tiny game with no backend; human vs bot, hotseat where applicable, bot-vs-bot replay, and replay viewer work; no legality exists in TypeScript."* The full acceptance detail is in §21.

| ROADMAP Gate 3 exit clause | Gate 3 spec criterion | Evidence |
|---|---|---|
| static site plays the tiny game with no backend | §21.1, §21.6 | WB11 + WB13 dist/browser smoke; no backend required |
| human vs bot works | §21.4 | WB8 + browser smoke |
| hotseat where applicable | §21.4 (§8.2) | WB8 + browser smoke (race_to_n is perfect-info; hotseat applies) |
| bot-vs-bot replay works | §21.4 | WB8/WB9 + browser smoke |
| replay viewer works | §21.4 | WB9 + browser smoke; Rust-authoritative step/reset |
| no legality exists in TypeScript | §21.2 | WB6 action-tree-driven controls + boundary/no-leak review §19.6 |

## Acceptance evidence

Re-runnable confirmation per `docs/TESTING-REPLAY-BENCHMARKING.md`; the full matrix is Appendix A.

- **Game-level evidence (rule coverage, golden traces, replay/hash, bot legality, benchmarks, serialization): already satisfied by Gates 1–2** for `race_to_n` — *not applicable as new Gate 3 work*; Gate 3 MUST keep these green (§19.1).
- **WASM/API smoke** — version/features, list games, new match, view, action tree, apply action, bot turn, effects, replay export/import (§19.2).
- **Browser UI smoke** — rendered shell flows: picker, setup, play, human action, bot turn, effect log, dev panel, replay export/import/step, bot-vs-bot autoplay (§19.3).
- **Accessibility smoke** — keyboard path, focus visible, accessible names, reduced motion, no color-only cues (§19.5).
- **Hidden-info no-leak review** — all browser surfaces reviewed even though `race_to_n` is perfect-information (§19.6).
- **Build/static-serve** — `npm ci` + `npm run build` succeed; dist serves locally with WASM loading (§18, §21.6).

## FOUNDATIONS & boundary alignment

| Principle | Stance | Rationale |
|---|---|---|
| §2 Behavior authority | aligned | Setup, legality, validation, state, effects, bots, replay, view projection stay in Rust; TypeScript is presentation-only (§9.1, §10.2). WB6 sources legal controls from the Rust action tree, never TS legality. |
| §3 `engine-core` is a contract kernel | aligned | No deliverable adds a mechanic noun to `engine-core`; new WASM ops live in `crates/wasm-api` and call `games/race_to_n` (§9.4). |
| §5 Static data is typed content | aligned | No YAML/DSL; replay documents are typed trace/command data validated by Rust, not behavior (§5, §15.3, §22). |
| §7 Public UI is central product work | aligned | Play-first public-presentable baseline; animation driven by Rust semantic effects; dev panel secondary (§6, §10.3, §14.1). |
| §8 Public bots | aligned | Only the existing Level-0 random-legal bot via the normal Rust bot op; no MCTS/ISMCTS/ML/RL (§7.5, §8.3, §22). |
| §9 Local-first v1/v2 | aligned | Static/local only; no accounts/DB/hosted multiplayer; browser owns no authoritative state (§5, §9.6, §18.4). |
| §10 IP conservatism | aligned | Original neutral visuals, neutral seat labels, no trade-dress/proprietary assets (§4.3, §12.1). |
| §11 Acceptance invariants | aligned | No-leak firewall, viewer-safe views/exports, deterministic replay via Rust, reject-unknown on imported data (§15.3, §16.3, §19.6). |
| §12 Stop conditions | clear | TS never decides legality; animation from Rust effects only; no hidden-info leak; no YAML/DSL; UI is play-first (see §12 check in the alignment narrative §3.5, §9.1, §14.1). |
| §13 ADR triggers | clear | `wasm-bindgen`, Canvas/PixiJS, and search/ML bots are explicitly deferred behind ADR, not adopted (§9.2, §22, §23.1, §23.8). |

## Forbidden changes

The gate-specific prohibitions are enumerated in full in [Section 5 (Non-goals)](#5-non-goals) and [Section 22 (Explicitly deferred work)](#22-explicitly-deferred-work). The load-bearing prohibitions: no second game / `three_marks`; no new mechanics; no TypeScript legality/replay authority; no hosted deployment or hosted multiplayer; no DSL/YAML behavior; no search/ML/RL bots; no Canvas/PixiJS without profiling + ADR; no proprietary/trade-dress assets; this spec deliverable produces no tickets/patches.

## Sequencing

- **Predecessor:** Gate 2 — trace/replay/benchmark hardening. Status `Done` (archived at [`archive/specs/gate-2-trace-replay-benchmark-hardening.md`](../archive/specs/gate-2-trace-replay-benchmark-hardening.md)).
- **Successor:** Gate 4 — `three_marks` (`Not started`, not yet specced).
- **Admission rule:** per [`specs/README.md`](README.md), Gate 3 is admitted as the lowest non-`Done` gate now that Gate 2's exit criteria pass. On adoption, flip the Gate 3 index row from `Not started` to `Planned`; flip to `Done` only when the Exit criteria above pass with evidence.

## Assumptions

One-line-correctable assumptions surfaced for task-time validation:

- **A-1:** The raw WASM ABI is retained (no `wasm-bindgen`) unless implementation finds a concrete blocker (§9.2, §23.1).
- **A-2:** Browser smoke uses Puppeteer unless Playwright shows a material cross-browser/reliability/role-locator benefit (§19.4).
- **A-3:** An in-memory Rust match store is acceptable; no browser persistence of authoritative match state (§9.6).
- **A-4:** `race_to_n` game-level evidence (rules/traces/replay/bots/benchmarks/serialization) is already complete from Gates 1–2; Gate 3 adds only shell/WASM/browser/a11y evidence (§19.1).
- **A-5:** Target commit `0f2e66f` equals current `main` HEAD at reassessment; re-verify `main` HEAD if landing from a different checkout.

---

## Evidence ledger and exact-commit discipline

This workflow does **not** independently verify the latest `main` branch. It analyzes the user-supplied target commit only.

- **Requested repository:** `joeloverbeck/rulepath`
- **Target commit:** `0f2e66f361a47ea2c19ead3c5e5959fb5cb137ab`
- **Freshness claim:** user-supplied target commit only; not independently verified as latest main
- **Manifest role:** path inventory only
- **Repository metadata used:** no
- **Default-branch lookup used:** no
- **Branch-name file fetch used:** no
- **Code search used:** no
- **Clone used:** no
- **URL fetch method:** `web.run.open` exact raw URL fetch
- **Fetched files:**
- https://raw.githubusercontent.com/joeloverbeck/rulepath/0f2e66f361a47ea2c19ead3c5e5959fb5cb137ab/README.md
- https://raw.githubusercontent.com/joeloverbeck/rulepath/0f2e66f361a47ea2c19ead3c5e5959fb5cb137ab/specs/README.md
- https://raw.githubusercontent.com/joeloverbeck/rulepath/0f2e66f361a47ea2c19ead3c5e5959fb5cb137ab/docs/README.md
- https://raw.githubusercontent.com/joeloverbeck/rulepath/0f2e66f361a47ea2c19ead3c5e5959fb5cb137ab/docs/FOUNDATIONS.md
- https://raw.githubusercontent.com/joeloverbeck/rulepath/0f2e66f361a47ea2c19ead3c5e5959fb5cb137ab/docs/ARCHITECTURE.md
- https://raw.githubusercontent.com/joeloverbeck/rulepath/0f2e66f361a47ea2c19ead3c5e5959fb5cb137ab/docs/ENGINE-GAME-DATA-BOUNDARY.md
- https://raw.githubusercontent.com/joeloverbeck/rulepath/0f2e66f361a47ea2c19ead3c5e5959fb5cb137ab/docs/OFFICIAL-GAME-CONTRACT.md
- https://raw.githubusercontent.com/joeloverbeck/rulepath/0f2e66f361a47ea2c19ead3c5e5959fb5cb137ab/docs/MECHANIC-ATLAS.md
- https://raw.githubusercontent.com/joeloverbeck/rulepath/0f2e66f361a47ea2c19ead3c5e5959fb5cb137ab/docs/AI-BOTS.md
- https://raw.githubusercontent.com/joeloverbeck/rulepath/0f2e66f361a47ea2c19ead3c5e5959fb5cb137ab/docs/UI-INTERACTION.md
- https://raw.githubusercontent.com/joeloverbeck/rulepath/0f2e66f361a47ea2c19ead3c5e5959fb5cb137ab/docs/TESTING-REPLAY-BENCHMARKING.md
- https://raw.githubusercontent.com/joeloverbeck/rulepath/0f2e66f361a47ea2c19ead3c5e5959fb5cb137ab/docs/ROADMAP.md
- https://raw.githubusercontent.com/joeloverbeck/rulepath/0f2e66f361a47ea2c19ead3c5e5959fb5cb137ab/docs/IP-POLICY.md
- https://raw.githubusercontent.com/joeloverbeck/rulepath/0f2e66f361a47ea2c19ead3c5e5959fb5cb137ab/docs/AGENT-DISCIPLINE.md
- https://raw.githubusercontent.com/joeloverbeck/rulepath/0f2e66f361a47ea2c19ead3c5e5959fb5cb137ab/docs/SOURCES.md
- https://raw.githubusercontent.com/joeloverbeck/rulepath/0f2e66f361a47ea2c19ead3c5e5959fb5cb137ab/docs/TRACE-SCHEMA-v1.md
- https://raw.githubusercontent.com/joeloverbeck/rulepath/0f2e66f361a47ea2c19ead3c5e5959fb5cb137ab/docs/adr/0001-stage-1-random-playout-budget.md
- https://raw.githubusercontent.com/joeloverbeck/rulepath/0f2e66f361a47ea2c19ead3c5e5959fb5cb137ab/docs/adr/0002-ci-benchmark-gating-lanes.md
- https://raw.githubusercontent.com/joeloverbeck/rulepath/0f2e66f361a47ea2c19ead3c5e5959fb5cb137ab/docs/adr/ADR-TEMPLATE.md
- https://raw.githubusercontent.com/joeloverbeck/rulepath/0f2e66f361a47ea2c19ead3c5e5959fb5cb137ab/docs/archival-workflow.md
- https://raw.githubusercontent.com/joeloverbeck/rulepath/0f2e66f361a47ea2c19ead3c5e5959fb5cb137ab/progress.md
- https://raw.githubusercontent.com/joeloverbeck/rulepath/0f2e66f361a47ea2c19ead3c5e5959fb5cb137ab/Cargo.toml
- https://raw.githubusercontent.com/joeloverbeck/rulepath/0f2e66f361a47ea2c19ead3c5e5959fb5cb137ab/.github/workflows/gate-0-hygiene.yml
- https://raw.githubusercontent.com/joeloverbeck/rulepath/0f2e66f361a47ea2c19ead3c5e5959fb5cb137ab/.github/workflows/gate-1-game-smoke.yml
- https://raw.githubusercontent.com/joeloverbeck/rulepath/0f2e66f361a47ea2c19ead3c5e5959fb5cb137ab/.github/workflows/gate-2-benchmarks.yml
- https://raw.githubusercontent.com/joeloverbeck/rulepath/0f2e66f361a47ea2c19ead3c5e5959fb5cb137ab/apps/web/package.json
- https://raw.githubusercontent.com/joeloverbeck/rulepath/0f2e66f361a47ea2c19ead3c5e5959fb5cb137ab/apps/web/vite.config.ts
- https://raw.githubusercontent.com/joeloverbeck/rulepath/0f2e66f361a47ea2c19ead3c5e5959fb5cb137ab/apps/web/index.html
- https://raw.githubusercontent.com/joeloverbeck/rulepath/0f2e66f361a47ea2c19ead3c5e5959fb5cb137ab/apps/web/src/main.tsx
- https://raw.githubusercontent.com/joeloverbeck/rulepath/0f2e66f361a47ea2c19ead3c5e5959fb5cb137ab/apps/web/src/styles.css
- https://raw.githubusercontent.com/joeloverbeck/rulepath/0f2e66f361a47ea2c19ead3c5e5959fb5cb137ab/apps/web/scripts/smoke-load-wasm.mjs
- https://raw.githubusercontent.com/joeloverbeck/rulepath/0f2e66f361a47ea2c19ead3c5e5959fb5cb137ab/apps/web/scripts/smoke-ui.mjs
- https://raw.githubusercontent.com/joeloverbeck/rulepath/0f2e66f361a47ea2c19ead3c5e5959fb5cb137ab/apps/web/tsconfig.json
- https://raw.githubusercontent.com/joeloverbeck/rulepath/0f2e66f361a47ea2c19ead3c5e5959fb5cb137ab/crates/wasm-api/Cargo.toml
- https://raw.githubusercontent.com/joeloverbeck/rulepath/0f2e66f361a47ea2c19ead3c5e5959fb5cb137ab/crates/wasm-api/src/lib.rs
- https://raw.githubusercontent.com/joeloverbeck/rulepath/0f2e66f361a47ea2c19ead3c5e5959fb5cb137ab/crates/engine-core/src/lib.rs
- https://raw.githubusercontent.com/joeloverbeck/rulepath/0f2e66f361a47ea2c19ead3c5e5959fb5cb137ab/crates/engine-core/src/action.rs
- https://raw.githubusercontent.com/joeloverbeck/rulepath/0f2e66f361a47ea2c19ead3c5e5959fb5cb137ab/crates/engine-core/src/game.rs
- https://raw.githubusercontent.com/joeloverbeck/rulepath/0f2e66f361a47ea2c19ead3c5e5959fb5cb137ab/crates/engine-core/src/replay.rs
- https://raw.githubusercontent.com/joeloverbeck/rulepath/0f2e66f361a47ea2c19ead3c5e5959fb5cb137ab/crates/engine-core/src/rng.rs
- https://raw.githubusercontent.com/joeloverbeck/rulepath/0f2e66f361a47ea2c19ead3c5e5959fb5cb137ab/crates/ai-core/src/lib.rs
- https://raw.githubusercontent.com/joeloverbeck/rulepath/0f2e66f361a47ea2c19ead3c5e5959fb5cb137ab/crates/ai-core/src/random_legal.rs
- https://raw.githubusercontent.com/joeloverbeck/rulepath/0f2e66f361a47ea2c19ead3c5e5959fb5cb137ab/games/race_to_n/Cargo.toml
- https://raw.githubusercontent.com/joeloverbeck/rulepath/0f2e66f361a47ea2c19ead3c5e5959fb5cb137ab/games/race_to_n/data/manifest.toml
- https://raw.githubusercontent.com/joeloverbeck/rulepath/0f2e66f361a47ea2c19ead3c5e5959fb5cb137ab/games/race_to_n/data/variants.toml
- https://raw.githubusercontent.com/joeloverbeck/rulepath/0f2e66f361a47ea2c19ead3c5e5959fb5cb137ab/games/race_to_n/docs/AI.md
- https://raw.githubusercontent.com/joeloverbeck/rulepath/0f2e66f361a47ea2c19ead3c5e5959fb5cb137ab/games/race_to_n/docs/BENCHMARKS.md
- https://raw.githubusercontent.com/joeloverbeck/rulepath/0f2e66f361a47ea2c19ead3c5e5959fb5cb137ab/games/race_to_n/docs/GAME-IMPLEMENTATION-ADMISSION.md
- https://raw.githubusercontent.com/joeloverbeck/rulepath/0f2e66f361a47ea2c19ead3c5e5959fb5cb137ab/games/race_to_n/docs/MECHANICS.md
- https://raw.githubusercontent.com/joeloverbeck/rulepath/0f2e66f361a47ea2c19ead3c5e5959fb5cb137ab/games/race_to_n/docs/RULE-COVERAGE.md
- https://raw.githubusercontent.com/joeloverbeck/rulepath/0f2e66f361a47ea2c19ead3c5e5959fb5cb137ab/games/race_to_n/docs/RULES.md
- https://raw.githubusercontent.com/joeloverbeck/rulepath/0f2e66f361a47ea2c19ead3c5e5959fb5cb137ab/games/race_to_n/docs/SOURCES.md
- https://raw.githubusercontent.com/joeloverbeck/rulepath/0f2e66f361a47ea2c19ead3c5e5959fb5cb137ab/games/race_to_n/docs/UI.md
- https://raw.githubusercontent.com/joeloverbeck/rulepath/0f2e66f361a47ea2c19ead3c5e5959fb5cb137ab/games/race_to_n/src/actions.rs
- https://raw.githubusercontent.com/joeloverbeck/rulepath/0f2e66f361a47ea2c19ead3c5e5959fb5cb137ab/games/race_to_n/src/bots.rs
- https://raw.githubusercontent.com/joeloverbeck/rulepath/0f2e66f361a47ea2c19ead3c5e5959fb5cb137ab/games/race_to_n/src/effects.rs
- https://raw.githubusercontent.com/joeloverbeck/rulepath/0f2e66f361a47ea2c19ead3c5e5959fb5cb137ab/games/race_to_n/src/ids.rs
- https://raw.githubusercontent.com/joeloverbeck/rulepath/0f2e66f361a47ea2c19ead3c5e5959fb5cb137ab/games/race_to_n/src/lib.rs
- https://raw.githubusercontent.com/joeloverbeck/rulepath/0f2e66f361a47ea2c19ead3c5e5959fb5cb137ab/games/race_to_n/src/replay_support.rs
- https://raw.githubusercontent.com/joeloverbeck/rulepath/0f2e66f361a47ea2c19ead3c5e5959fb5cb137ab/games/race_to_n/src/rules.rs
- https://raw.githubusercontent.com/joeloverbeck/rulepath/0f2e66f361a47ea2c19ead3c5e5959fb5cb137ab/games/race_to_n/src/setup.rs
- https://raw.githubusercontent.com/joeloverbeck/rulepath/0f2e66f361a47ea2c19ead3c5e5959fb5cb137ab/games/race_to_n/src/state.rs
- https://raw.githubusercontent.com/joeloverbeck/rulepath/0f2e66f361a47ea2c19ead3c5e5959fb5cb137ab/games/race_to_n/src/variants.rs
- https://raw.githubusercontent.com/joeloverbeck/rulepath/0f2e66f361a47ea2c19ead3c5e5959fb5cb137ab/games/race_to_n/src/visibility.rs
- https://raw.githubusercontent.com/joeloverbeck/rulepath/0f2e66f361a47ea2c19ead3c5e5959fb5cb137ab/games/race_to_n/tests/bot_tests.rs
- https://raw.githubusercontent.com/joeloverbeck/rulepath/0f2e66f361a47ea2c19ead3c5e5959fb5cb137ab/games/race_to_n/tests/property_tests.rs
- https://raw.githubusercontent.com/joeloverbeck/rulepath/0f2e66f361a47ea2c19ead3c5e5959fb5cb137ab/games/race_to_n/tests/replay_tests.rs
- https://raw.githubusercontent.com/joeloverbeck/rulepath/0f2e66f361a47ea2c19ead3c5e5959fb5cb137ab/games/race_to_n/tests/rule_tests.rs
- https://raw.githubusercontent.com/joeloverbeck/rulepath/0f2e66f361a47ea2c19ead3c5e5959fb5cb137ab/games/race_to_n/tests/serialization_tests.rs
- https://raw.githubusercontent.com/joeloverbeck/rulepath/0f2e66f361a47ea2c19ead3c5e5959fb5cb137ab/games/race_to_n/tests/golden_traces/bot-action.trace.json
- https://raw.githubusercontent.com/joeloverbeck/rulepath/0f2e66f361a47ea2c19ead3c5e5959fb5cb137ab/games/race_to_n/tests/golden_traces/invalid-stale-diagnostic.trace.json
- https://raw.githubusercontent.com/joeloverbeck/rulepath/0f2e66f361a47ea2c19ead3c5e5959fb5cb137ab/games/race_to_n/tests/golden_traces/not-applicable.trace.json
- https://raw.githubusercontent.com/joeloverbeck/rulepath/0f2e66f361a47ea2c19ead3c5e5959fb5cb137ab/games/race_to_n/tests/golden_traces/shortest-normal.trace.json
- https://raw.githubusercontent.com/joeloverbeck/rulepath/0f2e66f361a47ea2c19ead3c5e5959fb5cb137ab/games/race_to_n/tests/golden_traces/terminal.trace.json
- https://raw.githubusercontent.com/joeloverbeck/rulepath/0f2e66f361a47ea2c19ead3c5e5959fb5cb137ab/games/race_to_n/benches/race_to_n.rs
- https://raw.githubusercontent.com/joeloverbeck/rulepath/0f2e66f361a47ea2c19ead3c5e5959fb5cb137ab/games/race_to_n/benches/thresholds.json
- https://raw.githubusercontent.com/joeloverbeck/rulepath/0f2e66f361a47ea2c19ead3c5e5959fb5cb137ab/tools/replay-check/src/main.rs
- https://raw.githubusercontent.com/joeloverbeck/rulepath/0f2e66f361a47ea2c19ead3c5e5959fb5cb137ab/tools/trace-viewer/src/main.rs
- https://raw.githubusercontent.com/joeloverbeck/rulepath/0f2e66f361a47ea2c19ead3c5e5959fb5cb137ab/tools/simulate/src/main.rs
- https://raw.githubusercontent.com/joeloverbeck/rulepath/0f2e66f361a47ea2c19ead3c5e5959fb5cb137ab/tools/bench-report/src/main.rs
- https://raw.githubusercontent.com/joeloverbeck/rulepath/0f2e66f361a47ea2c19ead3c5e5959fb5cb137ab/tools/seed-reducer/src/main.rs
- https://raw.githubusercontent.com/joeloverbeck/rulepath/0f2e66f361a47ea2c19ead3c5e5959fb5cb137ab/tools/fixture-check/src/main.rs
- https://raw.githubusercontent.com/joeloverbeck/rulepath/0f2e66f361a47ea2c19ead3c5e5959fb5cb137ab/tools/rule-coverage/src/main.rs
- https://raw.githubusercontent.com/joeloverbeck/rulepath/0f2e66f361a47ea2c19ead3c5e5959fb5cb137ab/scripts/boundary-check.sh
- https://raw.githubusercontent.com/joeloverbeck/rulepath/0f2e66f361a47ea2c19ead3c5e5959fb5cb137ab/scripts/check-doc-links.mjs
- https://raw.githubusercontent.com/joeloverbeck/rulepath/0f2e66f361a47ea2c19ead3c5e5959fb5cb137ab/templates/AGENT-TASK.md
- https://raw.githubusercontent.com/joeloverbeck/rulepath/0f2e66f361a47ea2c19ead3c5e5959fb5cb137ab/templates/BOT-STRATEGY-EVIDENCE-PACK.md
- https://raw.githubusercontent.com/joeloverbeck/rulepath/0f2e66f361a47ea2c19ead3c5e5959fb5cb137ab/templates/COMPETENT-PLAYER.md
- https://raw.githubusercontent.com/joeloverbeck/rulepath/0f2e66f361a47ea2c19ead3c5e5959fb5cb137ab/templates/GAME-AI.md
- https://raw.githubusercontent.com/joeloverbeck/rulepath/0f2e66f361a47ea2c19ead3c5e5959fb5cb137ab/templates/GAME-BENCHMARKS.md
- https://raw.githubusercontent.com/joeloverbeck/rulepath/0f2e66f361a47ea2c19ead3c5e5959fb5cb137ab/templates/GAME-IMPLEMENTATION-ADMISSION.md
- https://raw.githubusercontent.com/joeloverbeck/rulepath/0f2e66f361a47ea2c19ead3c5e5959fb5cb137ab/templates/GAME-MECHANICS.md
- https://raw.githubusercontent.com/joeloverbeck/rulepath/0f2e66f361a47ea2c19ead3c5e5959fb5cb137ab/templates/GAME-RULE-COVERAGE.md
- https://raw.githubusercontent.com/joeloverbeck/rulepath/0f2e66f361a47ea2c19ead3c5e5959fb5cb137ab/templates/GAME-RULES.md
- https://raw.githubusercontent.com/joeloverbeck/rulepath/0f2e66f361a47ea2c19ead3c5e5959fb5cb137ab/templates/GAME-SOURCES.md
- https://raw.githubusercontent.com/joeloverbeck/rulepath/0f2e66f361a47ea2c19ead3c5e5959fb5cb137ab/templates/GAME-UI.md
- https://raw.githubusercontent.com/joeloverbeck/rulepath/0f2e66f361a47ea2c19ead3c5e5959fb5cb137ab/templates/PRIMITIVE-PRESSURE-LEDGER.md
- https://raw.githubusercontent.com/joeloverbeck/rulepath/0f2e66f361a47ea2c19ead3c5e5959fb5cb137ab/templates/PUBLIC-RELEASE-CHECKLIST.md
- https://raw.githubusercontent.com/joeloverbeck/rulepath/0f2e66f361a47ea2c19ead3c5e5959fb5cb137ab/templates/README.md
- **Contamination observed:** no
- **Connector/tool namespace trusted as evidence:** no
- **Abort status:** not triggered; all repository evidence used for codebase analysis came from exact raw URLs under `joeloverbeck/rulepath` at the target commit.

The official documentation and web-platform references consulted for current practice are listed in [Section 24](#24-research-notes-and-citations). Those sources are research context only; they are not repository evidence.

---

## 1. Purpose

Gate 3 is the transition from Rulepath’s current browser proof/harness into a durable, local-first, public-presentable static web shell backed by the Rust/WASM authority boundary.

The gate should prove that Rulepath can run a real browser app with no backend, where Rust remains authoritative for setup, legality, validation, state transitions, effects, bots, replay, serialization, and viewer-safe projections, while TypeScript/React owns presentation, layout, local UI state, and accessibility affordances only. This is the next logical step because the repository already contains Gate 0 hygiene, Gate 1 game smoke, Gate 2 benchmark workflows, a `race_to_n` Rust game, a raw WASM JSON bridge, and a thin web harness; however, the spec index explicitly leaves Gate 3 as the next unspecced stage, and the roadmap defines Gate 3 as the static site/browser shell gate. See [specs/README.md](https://raw.githubusercontent.com/joeloverbeck/rulepath/0f2e66f361a47ea2c19ead3c5e5959fb5cb137ab/specs/README.md) and [docs/ROADMAP.md](https://raw.githubusercontent.com/joeloverbeck/rulepath/0f2e66f361a47ea2c19ead3c5e5959fb5cb137ab/docs/ROADMAP.md).

Gate 3 should be built as a **public-presentable baseline**, not a full showcase. The result should feel like an early polished game shell rather than a raw diagnostic page. The goal is to retire UI debt before later gates and later games depend on the shell.

The gate must not prove new mechanics. It must not add a second game. It must not turn TypeScript into a rules engine. It must not introduce online infrastructure. It should create a stable foundation that later gates can reuse.

### Requirements language

- **MUST** means required for Gate 3 acceptance.
- **SHOULD** means strongly expected unless repository evidence or a documented implementation constraint justifies a narrower choice.
- **MAY** means permitted but not required.
- **DEFERRED** means explicitly outside Gate 3.

---

## 2. Source-of-truth hierarchy

Gate 3 implementation must obey the repository’s source-of-truth hierarchy. Foundation documents win over this Gate 3 spec if a conflict is discovered during implementation.

The authoritative hierarchy for this gate is:

1. [docs/README.md](https://raw.githubusercontent.com/joeloverbeck/rulepath/0f2e66f361a47ea2c19ead3c5e5959fb5cb137ab/docs/README.md) as the index of source-of-truth documents.
2. [docs/FOUNDATIONS.md](https://raw.githubusercontent.com/joeloverbeck/rulepath/0f2e66f361a47ea2c19ead3c5e5959fb5cb137ab/docs/FOUNDATIONS.md) as the constitutional layer.
3. [docs/ARCHITECTURE.md](https://raw.githubusercontent.com/joeloverbeck/rulepath/0f2e66f361a47ea2c19ead3c5e5959fb5cb137ab/docs/ARCHITECTURE.md) for Rust/TypeScript ownership, runtime pipeline, API shape, replay, static local-first direction, and hidden-information safety.
4. [docs/ENGINE-GAME-DATA-BOUNDARY.md](https://raw.githubusercontent.com/joeloverbeck/rulepath/0f2e66f361a47ea2c19ead3c5e5959fb5cb137ab/docs/ENGINE-GAME-DATA-BOUNDARY.md) for behavior/data boundaries and WASM bridge expectations.
5. [docs/OFFICIAL-GAME-CONTRACT.md](https://raw.githubusercontent.com/joeloverbeck/rulepath/0f2e66f361a47ea2c19ead3c5e5959fb5cb137ab/docs/OFFICIAL-GAME-CONTRACT.md) for game evidence, UI smoke, replay, and admission expectations.
6. [docs/UI-INTERACTION.md](https://raw.githubusercontent.com/joeloverbeck/rulepath/0f2e66f361a47ea2c19ead3c5e5959fb5cb137ab/docs/UI-INTERACTION.md) for public web product direction, renderer defaults, action interaction, replay UI, dev tools, accessibility, and hidden-info browser constraints.
7. [docs/TESTING-REPLAY-BENCHMARKING.md](https://raw.githubusercontent.com/joeloverbeck/rulepath/0f2e66f361a47ea2c19ead3c5e5959fb5cb137ab/docs/TESTING-REPLAY-BENCHMARKING.md) for test taxonomy, replay/golden trace expectations, and UI smoke expectations.
8. [docs/AI-BOTS.md](https://raw.githubusercontent.com/joeloverbeck/rulepath/0f2e66f361a47ea2c19ead3c5e5959fb5cb137ab/docs/AI-BOTS.md) for bot authority and v1/v2 bot exclusions.
9. [docs/IP-POLICY.md](https://raw.githubusercontent.com/joeloverbeck/rulepath/0f2e66f361a47ea2c19ead3c5e5959fb5cb137ab/docs/IP-POLICY.md) for public-facing assets, prose, fonts, trade dress, and private-content restrictions.
10. [docs/ROADMAP.md](https://raw.githubusercontent.com/joeloverbeck/rulepath/0f2e66f361a47ea2c19ead3c5e5959fb5cb137ab/docs/ROADMAP.md) for staged gate scope and explicit v1/v2 exclusions.
11. Filled game docs under [games/race_to_n/docs/](https://raw.githubusercontent.com/joeloverbeck/rulepath/0f2e66f361a47ea2c19ead3c5e5959fb5cb137ab/games/race_to_n/docs/UI.md) for current `race_to_n` rule/UI/bot/evidence constraints.
12. Templates under [templates/](https://raw.githubusercontent.com/joeloverbeck/rulepath/0f2e66f361a47ea2c19ead3c5e5959fb5cb137ab/templates/README.md) as checklist and documentation shapes, not as permission to create tickets in this deliverable.

Gate 3 must not reinterpret the foundation docs to make the web app “just a demo.” The foundations describe the public web UI as central product work; Gate 3 should satisfy that direction at baseline quality without overreaching into showcase polish.

---

## 3. Current repository state summary

### 3.1 Gate status and CI shape

The user reports Gate 2 complete. The target commit contains:

- Gate 0 hygiene workflow for Rust formatting, clippy, build, tests, and documentation checks: [gate-0-hygiene.yml](https://raw.githubusercontent.com/joeloverbeck/rulepath/0f2e66f361a47ea2c19ead3c5e5959fb5cb137ab/.github/workflows/gate-0-hygiene.yml).
- Gate 1 game smoke workflow that builds the WASM target, runs simulation/replay/fixture/rule-coverage checks, performs boundary checks, builds the web app, and runs the current smoke scripts: [gate-1-game-smoke.yml](https://raw.githubusercontent.com/joeloverbeck/rulepath/0f2e66f361a47ea2c19ead3c5e5959fb5cb137ab/.github/workflows/gate-1-game-smoke.yml).
- Gate 2 benchmark workflow with smoke and scheduled/manual/full gate lanes: [gate-2-benchmarks.yml](https://raw.githubusercontent.com/joeloverbeck/rulepath/0f2e66f361a47ea2c19ead3c5e5959fb5cb137ab/.github/workflows/gate-2-benchmarks.yml) and the benchmark ADRs [ADR 0001](https://raw.githubusercontent.com/joeloverbeck/rulepath/0f2e66f361a47ea2c19ead3c5e5959fb5cb137ab/docs/adr/0001-stage-1-random-playout-budget.md) / [ADR 0002](https://raw.githubusercontent.com/joeloverbeck/rulepath/0f2e66f361a47ea2c19ead3c5e5959fb5cb137ab/docs/adr/0002-ci-benchmark-gating-lanes.md).
- `specs/README.md` marks Gate 0, Gate 1, and Gate 2 specs done/archive-oriented, while Gate 3 “WASM/static web shell” is not yet specced and not started: [specs/README.md](https://raw.githubusercontent.com/joeloverbeck/rulepath/0f2e66f361a47ea2c19ead3c5e5959fb5cb137ab/specs/README.md).

### 3.2 Rust workspace and game/kernel state

The workspace contains generic engine contracts, a small AI core, the `race_to_n` game, WASM bridge, and supporting tools: [Cargo.toml](https://raw.githubusercontent.com/joeloverbeck/rulepath/0f2e66f361a47ea2c19ead3c5e5959fb5cb137ab/Cargo.toml).

Relevant existing capabilities:

- `engine-core` defines generic game contracts, action trees, freshness tokens, commands, effects, replay support, visibility scope, and deterministic RNG without game nouns: [engine-core lib.rs](https://raw.githubusercontent.com/joeloverbeck/rulepath/0f2e66f361a47ea2c19ead3c5e5959fb5cb137ab/crates/engine-core/src/lib.rs), [action.rs](https://raw.githubusercontent.com/joeloverbeck/rulepath/0f2e66f361a47ea2c19ead3c5e5959fb5cb137ab/crates/engine-core/src/action.rs), [game.rs](https://raw.githubusercontent.com/joeloverbeck/rulepath/0f2e66f361a47ea2c19ead3c5e5959fb5cb137ab/crates/engine-core/src/game.rs), [replay.rs](https://raw.githubusercontent.com/joeloverbeck/rulepath/0f2e66f361a47ea2c19ead3c5e5959fb5cb137ab/crates/engine-core/src/replay.rs), and [rng.rs](https://raw.githubusercontent.com/joeloverbeck/rulepath/0f2e66f361a47ea2c19ead3c5e5959fb5cb137ab/crates/engine-core/src/rng.rs).
- `ai-core` contains a deterministic random legal bot path selection layer: [ai-core lib.rs](https://raw.githubusercontent.com/joeloverbeck/rulepath/0f2e66f361a47ea2c19ead3c5e5959fb5cb137ab/crates/ai-core/src/lib.rs) and [random_legal.rs](https://raw.githubusercontent.com/joeloverbeck/rulepath/0f2e66f361a47ea2c19ead3c5e5959fb5cb137ab/crates/ai-core/src/random_legal.rs).
- `race_to_n` implements a deterministic two-seat Race to 21 game with add-1/add-2/add-3 legal actions, exact-target terminal state, public visibility, semantic effects, stale token diagnostics, replay/hash helpers, serialization tests, property tests, rule tests, and random legal bot support: [race_to_n lib.rs](https://raw.githubusercontent.com/joeloverbeck/rulepath/0f2e66f361a47ea2c19ead3c5e5959fb5cb137ab/games/race_to_n/src/lib.rs), [rules.rs](https://raw.githubusercontent.com/joeloverbeck/rulepath/0f2e66f361a47ea2c19ead3c5e5959fb5cb137ab/games/race_to_n/src/rules.rs), [actions.rs](https://raw.githubusercontent.com/joeloverbeck/rulepath/0f2e66f361a47ea2c19ead3c5e5959fb5cb137ab/games/race_to_n/src/actions.rs), [visibility.rs](https://raw.githubusercontent.com/joeloverbeck/rulepath/0f2e66f361a47ea2c19ead3c5e5959fb5cb137ab/games/race_to_n/src/visibility.rs), [effects.rs](https://raw.githubusercontent.com/joeloverbeck/rulepath/0f2e66f361a47ea2c19ead3c5e5959fb5cb137ab/games/race_to_n/src/effects.rs), [bots.rs](https://raw.githubusercontent.com/joeloverbeck/rulepath/0f2e66f361a47ea2c19ead3c5e5959fb5cb137ab/games/race_to_n/src/bots.rs), [replay_support.rs](https://raw.githubusercontent.com/joeloverbeck/rulepath/0f2e66f361a47ea2c19ead3c5e5959fb5cb137ab/games/race_to_n/src/replay_support.rs), and the game docs under [games/race_to_n/docs/RULES.md](https://raw.githubusercontent.com/joeloverbeck/rulepath/0f2e66f361a47ea2c19ead3c5e5959fb5cb137ab/games/race_to_n/docs/RULES.md).
- Tooling exists for replay checking, trace viewing, simulation, benchmark reporting, seed reduction, fixture checking, and rule coverage: [replay-check](https://raw.githubusercontent.com/joeloverbeck/rulepath/0f2e66f361a47ea2c19ead3c5e5959fb5cb137ab/tools/replay-check/src/main.rs), [trace-viewer](https://raw.githubusercontent.com/joeloverbeck/rulepath/0f2e66f361a47ea2c19ead3c5e5959fb5cb137ab/tools/trace-viewer/src/main.rs), [simulate](https://raw.githubusercontent.com/joeloverbeck/rulepath/0f2e66f361a47ea2c19ead3c5e5959fb5cb137ab/tools/simulate/src/main.rs), [bench-report](https://raw.githubusercontent.com/joeloverbeck/rulepath/0f2e66f361a47ea2c19ead3c5e5959fb5cb137ab/tools/bench-report/src/main.rs), [seed-reducer](https://raw.githubusercontent.com/joeloverbeck/rulepath/0f2e66f361a47ea2c19ead3c5e5959fb5cb137ab/tools/seed-reducer/src/main.rs), [fixture-check](https://raw.githubusercontent.com/joeloverbeck/rulepath/0f2e66f361a47ea2c19ead3c5e5959fb5cb137ab/tools/fixture-check/src/main.rs), and [rule-coverage](https://raw.githubusercontent.com/joeloverbeck/rulepath/0f2e66f361a47ea2c19ead3c5e5959fb5cb137ab/tools/rule-coverage/src/main.rs).

### 3.3 Existing WASM/API surface

The current `crates/wasm-api` crate exports a raw ABI and JSON response surface rather than `wasm-bindgen` generated bindings. It exposes memory allocation/deallocation, string-output helpers, a version function, and JSON-returning operations for:

- creating a match;
- fetching a public view;
- fetching the action tree;
- applying an action;
- running a bot turn;
- fetching effects.

The current bridge stores matches in a thread-local Rust-side map keyed by match id and returns normalized-ish JSON with status/diagnostics. See [crates/wasm-api/src/lib.rs](https://raw.githubusercontent.com/joeloverbeck/rulepath/0f2e66f361a47ea2c19ead3c5e5959fb5cb137ab/crates/wasm-api/src/lib.rs) and [crates/wasm-api/Cargo.toml](https://raw.githubusercontent.com/joeloverbeck/rulepath/0f2e66f361a47ea2c19ead3c5e5959fb5cb137ab/crates/wasm-api/Cargo.toml).

This surface is sufficient proof that a static browser app can talk to Rust/WASM. It is not yet sufficient as a durable Gate 3 shell boundary because it lacks a stable TypeScript client module, formal typed response shapes, feature/game listing, replay import/export operations, and a clean separation between low-level ABI calls and React components.

### 3.4 Existing web app state

The current web app is React + Vite and builds the WASM artifact into `apps/web/public/wasm_api.wasm`: [apps/web/package.json](https://raw.githubusercontent.com/joeloverbeck/rulepath/0f2e66f361a47ea2c19ead3c5e5959fb5cb137ab/apps/web/package.json) and [apps/web/vite.config.ts](https://raw.githubusercontent.com/joeloverbeck/rulepath/0f2e66f361a47ea2c19ead3c5e5959fb5cb137ab/apps/web/vite.config.ts).

Current app limitations relevant to Gate 3:

- `apps/web/src/main.tsx` contains the low-level WASM memory/string/JSON bridge inline with the React app, rather than a stable client boundary: [main.tsx](https://raw.githubusercontent.com/joeloverbeck/rulepath/0f2e66f361a47ea2c19ead3c5e5959fb5cb137ab/apps/web/src/main.tsx).
- The React state is a collection of local `useState` values for lifecycle, view, action tree, effects, diagnostics, match id, and stale-action demo behavior. This is acceptable for a harness but too fragile for Gate 3’s modes and replay/dev surface.
- The normal page is still harness-like: it can start a match, show a rendered Race-to-N view, show legal action buttons, run a bot turn, and demonstrate stale diagnostics. It does not yet provide a real game picker, match setup flow, hotseat mode surface, replay viewer, import/export, persistent shell layout, polished dev panel, or browser E2E coverage.
- `apps/web/scripts/smoke-load-wasm.mjs` and `apps/web/scripts/smoke-ui.mjs` instantiate/use WASM directly in Node-style scripts; they do not exercise the rendered browser DOM as a user-facing shell: [smoke-load-wasm.mjs](https://raw.githubusercontent.com/joeloverbeck/rulepath/0f2e66f361a47ea2c19ead3c5e5959fb5cb137ab/apps/web/scripts/smoke-load-wasm.mjs) and [smoke-ui.mjs](https://raw.githubusercontent.com/joeloverbeck/rulepath/0f2e66f361a47ea2c19ead3c5e5959fb5cb137ab/apps/web/scripts/smoke-ui.mjs).
- Current styles are simple and responsive enough for a harness, but not yet a public-presentable shell: [styles.css](https://raw.githubusercontent.com/joeloverbeck/rulepath/0f2e66f361a47ea2c19ead3c5e5959fb5cb137ab/apps/web/src/styles.css).

### 3.5 Repository evidence that constrains Gate 3

The repository docs require or strongly imply the following for Gate 3:

- Public web UI is central product work, not afterthought tooling: [FOUNDATIONS.md](https://raw.githubusercontent.com/joeloverbeck/rulepath/0f2e66f361a47ea2c19ead3c5e5959fb5cb137ab/docs/FOUNDATIONS.md).
- Rust is the behavior authority; TypeScript is presentation only: [ARCHITECTURE.md](https://raw.githubusercontent.com/joeloverbeck/rulepath/0f2e66f361a47ea2c19ead3c5e5959fb5cb137ab/docs/ARCHITECTURE.md) and [ENGINE-GAME-DATA-BOUNDARY.md](https://raw.githubusercontent.com/joeloverbeck/rulepath/0f2e66f361a47ea2c19ead3c5e5959fb5cb137ab/docs/ENGINE-GAME-DATA-BOUNDARY.md).
- The browser shell should support local-first v1/v2 modes: human-vs-bot, hotseat, bot-vs-bot replay, replay viewer, and local replay import/export, with no hosted services: [FOUNDATIONS.md](https://raw.githubusercontent.com/joeloverbeck/rulepath/0f2e66f361a47ea2c19ead3c5e5959fb5cb137ab/docs/FOUNDATIONS.md) and [ROADMAP.md](https://raw.githubusercontent.com/joeloverbeck/rulepath/0f2e66f361a47ea2c19ead3c5e5959fb5cb137ab/docs/ROADMAP.md).
- The UI should be product-facing, not debug-console-first, with React + SVG as the default renderer and Canvas/PixiJS deferred unless profiling or ADR evidence justifies it: [UI-INTERACTION.md](https://raw.githubusercontent.com/joeloverbeck/rulepath/0f2e66f361a47ea2c19ead3c5e5959fb5cb137ab/docs/UI-INTERACTION.md) and [GAME-UI template](https://raw.githubusercontent.com/joeloverbeck/rulepath/0f2e66f361a47ea2c19ead3c5e5959fb5cb137ab/templates/GAME-UI.md).
- UI smoke should cover load, setup, public view, legal actions, human action, bot turn, effects, replay stepping, safe dev toggle, reduced motion, responsive behavior, keyboard/focus baseline, and hidden-info no-leak where applicable: [TESTING-REPLAY-BENCHMARKING.md](https://raw.githubusercontent.com/joeloverbeck/rulepath/0f2e66f361a47ea2c19ead3c5e5959fb5cb137ab/docs/TESTING-REPLAY-BENCHMARKING.md) and [OFFICIAL-GAME-CONTRACT.md](https://raw.githubusercontent.com/joeloverbeck/rulepath/0f2e66f361a47ea2c19ead3c5e5959fb5cb137ab/docs/OFFICIAL-GAME-CONTRACT.md).
- Hidden information must not leak through browser payloads, DOM attributes, local storage, logs, test IDs, replay exports, bot explanations, candidate rankings, or dev inspectors. `race_to_n` is perfect-information, but the shell must be designed for later hidden-information games: [UI-INTERACTION.md](https://raw.githubusercontent.com/joeloverbeck/rulepath/0f2e66f361a47ea2c19ead3c5e5959fb5cb137ab/docs/UI-INTERACTION.md), [AGENT-DISCIPLINE.md](https://raw.githubusercontent.com/joeloverbeck/rulepath/0f2e66f361a47ea2c19ead3c5e5959fb5cb137ab/docs/AGENT-DISCIPLINE.md), and [PUBLIC-RELEASE-CHECKLIST.md](https://raw.githubusercontent.com/joeloverbeck/rulepath/0f2e66f361a47ea2c19ead3c5e5959fb5cb137ab/templates/PUBLIC-RELEASE-CHECKLIST.md).

---

## 4. Gate 3 scope

Gate 3 scope is the smallest durable public-presentable web shell that proves Rulepath can run a static browser app backed by the Rust/WASM authority boundary.

### 4.1 Required product scope

Gate 3 MUST deliver:

- A local static web app for `race_to_n`, built through `npm run build` in `apps/web`.
- A public-presentable baseline layout with coherent typography, clear hierarchy, and no raw-JSON-first user experience.
- A game picker shell that can show one selectable game: `race_to_n`.
- A match setup panel for `race_to_n` with clear mode/seat/seed/start affordances appropriate to this simple game.
- User-facing play for `race_to_n` through Rust-supplied views, legal action trees, effects, and bot operations.
- Required Gate 3 play modes at practical Race-to-N scale: human-vs-bot, local hotseat, bot-vs-bot replay/autoplay, replay viewer, and safe local replay import/export.
- A deliberate dev/replay panel that is secondary to play and restricted to viewer-safe data.
- A stable TypeScript client wrapper around the current or evolved WASM exports.
- Structured React state management suitable for the shell lifecycle and replay/dev modes.
- Responsive, keyboard-accessible, reduced-motion-aware UI baseline.
- CI/smoke coverage proving the built browser shell, not merely the low-level WASM API.

### 4.2 Required technical scope

Gate 3 MUST harden the browser/WASM boundary without over-rotating into a new binding stack unless blocking evidence appears.

Minimum technical scope:

- Keep Rust as the behavior authority.
- Wrap the raw exported ABI behind a typed TypeScript client module.
- Normalize all WASM response parsing and errors at the client boundary.
- Keep React components away from low-level exported memory functions and raw `last_output` mechanics.
- Add or expose enough Rust/WASM operations to support game listing, match creation, views, action trees, action application, bot turns, effects, replay export/import, and replay stepping.
- Build and serve as static files with no backend.
- Validate local build output and browser behavior.

### 4.3 Quality target

Gate 3’s UI quality target is **public-presentable baseline**:

- coherent app shell;
- readable typography;
- visually intentional Race-to-N renderer;
- clear match setup;
- legal actions obvious and reachable;
- current status visible;
- effect/replay log readable;
- game picker clean even with one game;
- dev tools secondary;
- responsive layout;
- keyboard/focus baseline;
- reduced-motion support;
- no normal raw JSON UX;
- no proprietary/trade-dress mimicry.

It is **not** showcase polish. It does not require production art, final brand direction, complex animation, online deployment, many games, or later-game renderer sophistication.

---

## 5. Non-goals

Gate 3 MUST NOT implement or require:

- `three_marks` or any second game.
- New game mechanics.
- A broad game catalog.
- TypeScript-side legality, hidden-state authority, rule consequences, bot decisions, replay authority, or rules previews.
- Hosted multiplayer.
- Accounts, databases, matchmaking, chat, ranked play, online persistence, analytics, telemetry, or server authority.
- GitHub Pages, Netlify, Vercel, Cloudflare, Firebase, or any hosted deployment.
- DSL/YAML/static-data behavior.
- Public MCTS, ISMCTS, Monte Carlo-style bots, ML, or RL.
- Private monster-game work or private licensed content.
- Canvas/PixiJS renderer adoption without profiling evidence and ADR-level justification.
- Production art, final branding, custom fonts without license review, copied assets, screenshots/scans, copied rulebook prose, or trade-dress mimicry.
- A patch, tickets, or implementation decomposition as part of this spec deliverable.

---

## 6. User/product intent

Gate 3 should be implemented as if it were heading toward a public-presentable static demo, even though immediate public presentation is not required.

The reason is architectural: later gates and games will build on this shell. A debug-first harness would force later rewrites and create UI debt at the exact boundary where Rulepath’s architecture is most distinctive: Rust owns behavior, TypeScript presents viewer-safe state.

The product experience should say: “This is an early but intentional game shell.” It should not say: “This is a raw WASM diagnostic page.”

Concrete intent:

- Start with `race_to_n` only.
- Build the app shell in a way that can later host other games.
- Keep debug and diagnostics available but secondary.
- Make replay/dev functionality deliberate and readable.
- Preserve the current raw-WASM/JSON bridge if it can be made stable and safe behind a typed TypeScript boundary.
- Discuss `wasm-bindgen` as future/optional evaluation, not a Gate 3 mandate.
- Prefer minimizing tooling churn for smoke/E2E; use Puppeteer-style browser smoke if it can cover Gate 3 requirements cleanly, and recommend Playwright only for material reliability or role-locator/cross-browser needs.

---

## 7. Required user-facing capabilities

Gate 3 MUST expose the following user-facing capabilities.

### 7.1 App shell

The shell MUST include:

- A header or landmarked top area with the Rulepath name and concise product context.
- A main play region.
- A game picker region, even if it has only one selectable game.
- A match setup region before a match begins.
- A Race-to-N board/status region once active.
- A legal actions region generated from Rust action choices.
- A match status/turn/winner region.
- An effect log/recent events region.
- A replay/dev panel that is collapsed, secondary, or visually subordinate by default.
- A clear error/diagnostic surface for user-safe failures.

### 7.2 Game picker

The game picker MUST:

- Show `race_to_n` as the only active selectable game.
- Use game metadata supplied by Rust/WASM where practical, not hardcoded React behavior authority.
- Avoid presenting future games as selectable or implied-complete.
- MAY omit future-game placeholders entirely.
- MAY show future slots only if clearly noninteractive and labeled as future work; this is not required and should not distract from Gate 3.

### 7.3 Match setup

The match setup panel MUST:

- Identify the selected game and variant as Race to 21 / `race_to_n` using Rust-backed metadata where available.
- Let the user choose a play mode among the Gate 3 practical modes.
- Show seat roles clearly for human-vs-bot and hotseat.
- Show seed/version metadata if available; seed display may be dev-panel-only if the setup screen would become cluttered.
- Provide a clear “start match” action.
- Avoid exposing raw JSON as setup output.
- Avoid implying online multiplayer or accounts.

For `race_to_n`, setup MAY stay simple: fixed two seats, default Race to 21 variant, random legal bot when bot mode is selected, and a deterministic default seed. The shell should still model setup explicitly because later games will need it.

### 7.4 Active play

Active play MUST:

- Render the Rust public view for the current viewer.
- Render legal actions from the Rust action tree, not from TypeScript-calculated legality.
- Apply user actions by sending Rust action paths/segments through the WASM client.
- Refresh the public view, action tree, and effects from Rust after actions and bot turns.
- Make the current actor/turn obvious.
- Make terminal/winner status obvious.
- Prevent normal users from clicking illegal or stale actions.
- Handle stale/invalid diagnostics gracefully and refresh from Rust.

### 7.5 Bot controls

Bot controls MUST:

- Use Rust bot operations only.
- Never choose actions in TypeScript.
- Support at least one human-vs-random-legal-bot flow.
- Support bot-vs-bot replay/autoplay as a controlled mode using Rust bot turns.
- Include pause/step controls for autoplay.
- Respect reduced-motion mode by shortening/removing nonessential delay/animation while preserving event feedback.

### 7.6 Replay/dev user surface

The replay/dev surface MUST be deliberate, viewer-safe, and secondary. It SHOULD include:

- game id;
- display name/variant;
- rules/data/API version metadata available from Rust;
- match id or handle if viewer-safe;
- seed if available;
- selected mode and seat roles;
- current actor;
- freshness token or token summary;
- legal action count/tree summary;
- effect cursor and effect log;
- command/replay stream summary if available;
- replay export/import;
- replay stepping;
- bot/autoplay controls;
- diagnostics;
- stale-action demonstration only if still useful, clearly debug-labeled, and not part of normal play.

The normal user experience MUST NOT be dominated by JSON inspectors, stale-action demos, or debug fields.

---

## 8. Required play modes

Gate 3 MUST implement the repository-required local-first modes at a practical scale for `race_to_n`.

### 8.1 Human vs bot

Minimum requirement:

- User can start a Race-to-N match with one human seat and one Rust random-legal bot seat.
- The user can apply legal human actions using Rust action choices.
- When it is the bot’s turn, the UI can run a Rust bot turn.
- Bot action results are shown through Rust effects and settled Rust view.
- The mode can complete a match to terminal state.

Preferred baseline:

- The user can choose whether the human plays first or second.
- The UI labels seats neutrally, e.g. “Seat 1” and “Seat 2,” not brand/proprietary labels.
- Bot turn can be manual (“Run bot turn”) and optionally automatic with pause.

### 8.2 Local hotseat

Minimum requirement:

- User can start a two-human local hotseat match.
- The active seat is clearly labeled.
- Only Rust legal actions for the active actor are clickable.
- After a legal action, the UI updates turn/status/effects and presents the next actor’s legal actions.

Because `race_to_n` has no hidden information, no seat-handoff privacy screen is required for Gate 3. The state architecture should still use viewer/actor concepts so later hidden-information games can introduce seat handoff without replacing the shell.

### 8.3 Bot-vs-bot replay/autoplay

Minimum requirement:

- User can start a bot-vs-bot Race-to-N run where Rust bots choose all actions.
- The user can step one bot turn at a time.
- The user can start/pause autoplay.
- The UI records/shows semantic effects and terminal state.
- The resulting run can be replayed/exported if the replay API is implemented as required in this gate.

This mode is a shell/replay proof, not an AI-strength proof. Only the existing random legal bot class is required.

### 8.4 Replay viewer

Minimum requirement:

- User can view a replay generated by the current shell.
- User can step forward through at least one command/effect boundary.
- User can reset to the beginning.
- The displayed state/effects come from Rust replay/validation/projected view logic, not TypeScript simulation.
- The replay surface shows safe metadata and safe effect summaries.

Preferred baseline:

- Step backward is supported either by Rust snapshots/checkpoints or by resetting and replaying from the beginning through Rust.
- Speed controls exist for autoplay.
- Reduced-motion mode uses textual/highlight feedback rather than motion-dependent animation.

### 8.5 Safe local replay import/export

Minimum requirement:

- User can export a replay from a Race-to-N match in a safe local format.
- User can import that exported replay back into the app.
- Rust validates imported replay data before it becomes user-visible app state.
- Invalid, mismatched, oversized, malformed, unsupported-version, or wrong-game imports produce clear safe diagnostics.
- Exported replay data contains no hidden or unauthorized information. For `race_to_n`, hidden information is not applicable; the implementation must still use a pattern safe for future hidden-information games.

The shell MUST NOT treat imported JSON or local storage as authoritative state. Imported data is input to Rust validation/replay, not TypeScript-owned truth.

---

## 9. WASM/API boundary requirements

### 9.1 Authority boundary

Rust MUST remain authoritative for:

- match setup;
- legal action generation;
- action validation;
- freshness/stale checks;
- state transitions;
- semantic effects;
- public/private view projection;
- bot decisions;
- replay validation/execution;
- serialization/deserialization;
- diagnostics;
- version/rules metadata.

TypeScript/React MUST NOT:

- compute legal actions;
- decide whether a requested action is legal;
- infer rule consequences;
- mutate authoritative game state;
- choose bot actions;
- validate replay semantics;
- expose hidden or private state;
- use CSS/display toggles as a security or visibility boundary;
- derive disabled illegal reasons unless supplied by Rust and viewer-safe.

### 9.2 Binding strategy

Gate 3 SHOULD keep the current raw-WASM/JSON strategy if it can be hardened behind a typed TypeScript client boundary. Repository evidence does not show that `wasm-bindgen` is blocking or required for Gate 3.

Rationale:

- The current raw ABI already works for `race_to_n` and is integrated with the build scripts.
- The conceptual architecture emphasizes coarse/batched WASM calls, not a chatty object-oriented JS/Rust boundary.
- `wasm-bindgen` provides high-level JS/WASM interactions and TypeScript bindings, but adopting it would add binding/build churn that is not necessary unless the raw bridge blocks typed safety, memory safety, static build behavior, or testability.
- Gate 3’s immediate problem is the missing shell/client architecture, not the absence of generated bindings.

Gate 3 MAY include an evaluation note for future `wasm-bindgen` or generated bindings. It MUST NOT require migration unless implementation analysis finds a concrete blocker in the current raw bridge.

### 9.3 TypeScript client boundary

The React component tree MUST NOT call low-level WASM exports directly. A dedicated TypeScript client module MUST own:

- loading/fetching the `.wasm` artifact;
- selecting the correct artifact URL under Vite build/base behavior;
- instantiating WebAssembly;
- verifying required exports;
- managing allocation/deallocation;
- UTF-8 encoding/decoding;
- reading/writing WebAssembly memory;
- calling raw exported functions;
- retrieving `last_output` or equivalent output buffer;
- parsing JSON;
- validating response shape enough to prevent app crashes;
- normalizing status/error/diagnostic values;
- exposing typed methods to the React app;
- reporting API/features/version information;
- isolating raw ABI details from UI components and tests.

The client boundary MUST return typed, normalized results to the app. React should work with app-level types such as “loaded wasm client,” “game catalog,” “match handle,” “public view,” “action tree,” “effect batch,” “diagnostic,” and “replay document,” not raw pointers, lengths, exported function names, or arbitrary JSON blobs.

### 9.4 Minimum required operations

Gate 3 MUST support the following operation groups. Exact function names may differ, but the capability boundary must be clear and documented.

#### Required for Gate 3 minimum

- **Feature/version report:** returns API version and supported operation names/feature flags sufficient for diagnostics.
- **List games:** returns at least `race_to_n` metadata. This may be a new `list_games` WASM operation or an equivalent Rust-supplied catalog response.
- **New match:** creates a Race-to-N match from explicit setup/mode inputs or a documented default setup.
- **Get view:** returns the current viewer-safe public view.
- **Get action tree:** returns the Rust-generated action tree and freshness token.
- **Apply action:** accepts a Rust action path and freshness token; applies through Rust validation; returns normalized success/diagnostic result.
- **Run bot turn:** asks Rust to choose/apply for the active bot actor; returns normalized success/diagnostic result.
- **Get effects:** returns viewer-safe effects since a cursor.
- **Export replay:** returns safe replay data or command/effect stream sufficient for the replay viewer/import test.
- **Import/load replay:** validates and loads a safe replay document, or validates and reconstructs a replay session from Rust.
- **Replay step/reset:** allows the browser replay viewer to obtain Rust-authoritative projected state/effects for a replay cursor.

#### Strongly preferred but deferrable if not needed for Race-to-N Gate 3 acceptance

- **Preview action:** useful for compound games and richer UI previews. For flat Race-to-N, the action tree labels and effects may be enough. Gate 3 MUST NOT implement TypeScript-side previews as a substitute.
- **Serialize/load match:** useful for persistence. Gate 3 should avoid automatic match persistence; serialize/load match is only required if replay import/export cannot satisfy the required replay mode safely.
- **Viewer-specific view selection:** Race-to-N is public/perfect-information, but the API should not paint itself into a corner. If implementation keeps only public view for Gate 3, it should document how later private views will be added without leaking through the shell.

### 9.5 Response and diagnostic shape

All WASM-facing operations MUST normalize to a stable response contract with:

- status/success indicator;
- typed data payload on success;
- typed diagnostic on failure;
- stable diagnostic code;
- safe human-readable message;
- optional recoverability/retry hints;
- no hidden or unauthorized state;
- no panic text or internal Rust backtrace in normal public output.

Diagnostics MUST be viewer-safe. Stale action diagnostics may include that an action is stale and that the view/action tree was refreshed; they must not include unauthorized state.

### 9.6 Memory and lifecycle

The TypeScript client MUST ensure:

- allocated request buffers are deallocated after calls;
- output reads are decoded before overwritten;
- exceptions cannot skip cleanup without bounded leak risk;
- missing/malformed exports fail with a clear app diagnostic;
- a failed WASM load leaves the shell in a recoverable error state;
- hot reload/dev rebuild behavior does not corrupt match lifecycle unexpectedly.

For Gate 3, an in-memory Rust match store is acceptable. No browser persistence of authoritative match state is required.

### 9.7 API documentation

Gate 3 implementation MUST document the effective WASM client contract in the repo, either in an app/web README, a dedicated API boundary doc, or updated architecture notes. The doc should state:

- which operations exist;
- which operation group each supports;
- request/response shape at a high level;
- which data is viewer-safe;
- which operations are deferred;
- why raw ABI is retained or why a binding migration was required.

---

## 10. TypeScript/React shell requirements

### 10.1 Component responsibilities

The shell SHOULD be organized around durable regions/components, not one monolithic diagnostic component. The exact component names are implementation details, but the responsibilities MUST be separated:

- app/bootstrap shell;
- WASM loading/error boundary;
- game picker;
- match setup;
- active match view;
- Race-to-N renderer/status;
- legal action controls;
- effect log;
- replay viewer controls;
- dev/debug panel;
- diagnostics/toasts/alerts;
- reduced-motion preference integration.

### 10.2 Presentation-only rule

React components MUST only render Rust-provided data and dispatch user intentions to the client boundary. Components may:

- group controls visually;
- choose layout and responsive behavior;
- choose labels derived from Rust-provided accessible labels/display names;
- manage focus;
- manage pending UI state;
- manage animations based on Rust semantic effects;
- manage dev panel visibility;
- store safe user preferences.

Components MUST NOT:

- synthesize action choices from numeric game rules;
- use `legal_additions` in the public view as the source of clickable legal actions if the action tree is available;
- decide add-1/add-2/add-3 legality in TypeScript;
- run bot policy in TypeScript;
- trust imported replay JSON as state;
- reveal internal Rust state or private fields in DOM/test ids/logs.

### 10.3 Normal UX and debug UX separation

The normal page MUST be play-first. Debugging and replay controls MUST be intentionally placed, labeled, and collapsible or visually secondary.

Normal UX MUST NOT include:

- raw JSON dumps as the main content;
- permanent stale-action demo controls in the main action row;
- low-level WASM pointer/length details;
- internal match store dumps;
- full state inspectors;
- “test harness” copy or layout as the primary presentation.

Debug UX MAY include viewer-safe JSON summaries, but only inside a labeled dev panel and not as the normal way to understand the game.

### 10.4 Error handling

The shell MUST handle:

- WASM file missing or failed to instantiate;
- unsupported/missing WASM export;
- malformed JSON response;
- Rust diagnostic response;
- stale action;
- no legal actions when terminal;
- replay import parse failure;
- replay version/game mismatch;
- bot turn requested when no bot action is applicable;
- effect cursor mismatch or reset.

Errors should be readable and nonfatal where possible. The user should be able to restart a match after recoverable errors.

---

## 11. State management requirements

Gate 3 MUST avoid an uncontrolled pile of scattered React `useState` calls for the match lifecycle. Use a reducer-style state model, explicit state machine, or equivalent structured state architecture.

The state model MUST cover:

- WASM loading: idle/loading/ready/error;
- game catalog loading;
- selected game;
- match setup inputs;
- active match handle;
- active mode: setup/play/replay/dev-error as appropriate;
- active actor/current viewer;
- latest public view;
- latest action tree;
- selected action/path, if any;
- pending command/action/bot/replay operation;
- diagnostics;
- effect cursor;
- effect queue/log;
- replay document/session;
- replay cursor;
- replay/autoplay state;
- bot/autoplay state;
- dev-panel visibility;
- reduced-motion preference;
- safe local preferences loaded/saved status.

State transitions SHOULD be explicit enough that tests can reason about them. For example:

- `wasmLoaded` enables catalog fetch;
- `gameSelected` enables setup;
- `matchStarted` resets effect cursor and replay state;
- `actionApplied` triggers view/action/effects refresh;
- `staleDiagnostic` triggers refresh and safe diagnostic display;
- `replayImported` enters replay viewer state only after Rust validation;
- `autoplayPaused` stops queued bot/replay advancement.

The reducer/state machine MUST maintain a single authoritative app state representation. Derived visual state is allowed, but duplicate independent copies of Rust view/action/effect truth should be avoided.

---

## 12. Race-to-N renderer requirements

Gate 3 MUST replace the raw harness feel with a simple, intentional Race-to-N renderer.

### 12.1 Visual model

The renderer SHOULD use React + SVG/semantic HTML as the default direction. It SHOULD represent:

- current counter;
- target value;
- recent increment;
- active seat;
- winner/terminal status;
- legal add choices;
- turn progression;
- effect history.

A simple visual track, meter, number line, or table-like board is sufficient. It must be original and neutral. No copied/proprietary trade dress is allowed.

### 12.2 Status display

The renderer MUST make these visible:

- current counter;
- target number;
- maximum add value if useful;
- active seat;
- whether the match is active or terminal;
- winner when terminal;
- latest action/effect summary.

### 12.3 Legal action controls

For Race-to-N, legal actions are flat. The UI MUST map each Rust action choice to a control with:

- visible label;
- accessible name;
- action path/segment from Rust;
- disabled/pending state while an operation is in flight;
- no TypeScript-side legality calculation.

If an action is absent from the action tree, it is not clickable in normal play. The UI must not show disabled add values unless a deliberate learning/debug mode exists and Rust supplies viewer-safe reasons.

### 12.4 Terminal state

When Race-to-N is terminal:

- legal action controls should be absent or inert with clear terminal status;
- bot/autoplay should stop;
- effect log should show the game-ended event;
- replay/export controls should remain available;
- “new match” should be prominent enough to recover.

---

## 13. Action-tree and legality requirements

### 13.1 Action tree as UI source

The Rust action tree is the only source for normal clickable actions.

The shell MUST:

- fetch the action tree for the active actor/viewer context;
- render action choices from that tree;
- preserve action path/segment identity when dispatching;
- include freshness token information required by Rust validation;
- refresh action tree after every successful action, bot turn, replay jump, match restart, or stale/invalid diagnostic.

### 13.2 Freshness and stale action handling

The UI MUST avoid normal stale actions by disabling controls during pending operations and refreshing after each operation.

A stale-action demo MAY remain only in the dev/debug panel. It MUST:

- be clearly labeled as a diagnostic/dev tool;
- use viewer-safe stale diagnostics;
- never appear as the main way to play;
- refresh view/action/effects after use.

### 13.3 Illegal/unavailable actions

Normal play SHOULD show legal actions only. If a learning/debug mode shows unavailable choices:

- reasons MUST be Rust-supplied;
- reasons MUST be viewer-safe;
- controls MUST remain disabled/inert;
- DOM/test IDs/classes MUST NOT leak hidden facts;
- this mode MUST be secondary to normal play.

Gate 3 does not require unavailable-choice explanations for Race-to-N.

### 13.4 Compound-action future readiness

Race-to-N uses flat actions, but the shell should not hardwire itself to one-segment actions. The client/app types SHOULD preserve enough action tree shape to support later progressive action construction without a rewrite.

Gate 3 does not need to implement a full compound-action UI. It does need to avoid making later compound actions impossible.

---

## 14. Effect queue and animation requirements

### 14.1 Effect authority

Animations and event summaries MUST be driven by Rust semantic effects, not TypeScript state diffs as normal authority.

The UI MAY use view differences as a diagnostic check or fallback rendering hint, but normal action feedback should come from Rust effects.

### 14.2 Race-to-N effect mapping

The renderer SHOULD map current Race-to-N effects to simple feedback:

- action started: mark/announce active actor action;
- counter advanced: update/highlight counter movement;
- turn changed: update/highlight active seat;
- game ended: show terminal/winner banner;
- action completed: finalize event summary.

Exact animation style is flexible. The output should be readable and calm, not flashy.

### 14.3 Effect log

The effect log MUST:

- show a chronological sequence of viewer-safe effect summaries;
- distinguish new/recent effects from older entries;
- preserve enough information to understand the match flow;
- avoid raw JSON as the default display;
- remain usable with reduced motion;
- handle effect cursor reset/restart cleanly.

### 14.4 Reduced motion

Reduced-motion mode MUST reduce, replace, or remove nonessential motion while preserving feedback. For example:

- instant counter update instead of animated movement;
- text summary/highlight instead of sliding/bouncing;
- no autoplay animation dependency;
- no information conveyed only through motion.

The shell MUST honor OS `prefers-reduced-motion` and MAY offer a local explicit preference. Persisting that preference locally is allowed because it is safe user preference data.

### 14.5 Settle-to-view invariant

After every action, bot turn, replay step, or import/reset, the renderer MUST settle to the latest Rust-projected view. If an animation runs, the final rendered state must match Rust view data.

---

## 15. Replay/import/export requirements

### 15.1 Replay authority

Rust MUST remain replay authority. TypeScript may store replay UI cursor and render returned views/effects, but it must not replay commands by mutating game state itself.

Replay import/export in Gate 3 MUST be safe and local. No server is involved.

The replay format MUST be anchored on the existing Gate 2 trace/replay infrastructure rather than a parallel invention: `docs/TRACE-SCHEMA-v1.md` (canonical trace schema), `games/race_to_n/src/replay_support.rs` (game replay helpers), `tools/replay-check/` (validation reference), and the `engine-core` `CommandEnvelope` / `EffectLog` / `EffectCursor` contracts. New WASM replay operations reuse these surfaces so exported replays remain validatable by the same Rust path Gate 2 hardened.

### 15.2 Export

Replay export MUST:

- produce a local downloadable/copyable replay document or payload;
- include game id and version metadata needed for validation;
- include seed/setup metadata where available;
- include command/effect data or equivalent replay data sufficient to reconstruct through Rust;
- exclude hidden/unauthorized data by design;
- avoid exporting full internal match state unless it is explicitly viewer-safe and future hidden-info risks are documented.

For Race-to-N, all game facts are public, but the export design should still prefer command/effect/replay data over internal state dumps.

### 15.3 Import

Replay import MUST:

- accept a local file or pasted payload;
- enforce size limits appropriate for a smoke/demo shell;
- parse safely;
- pass data to Rust validation before entering replay mode;
- reject wrong game id, unsupported version, malformed data, unknown/unsafe shape, and inconsistent replay data with clear diagnostics;
- never execute scripts or treat imported HTML/JS as content;
- never store imported data automatically as authoritative state.

### 15.4 Replay stepping

Replay viewer MUST:

- show initial state;
- step forward at least one command/effect boundary;
- reset to start;
- show current replay cursor/progress;
- show current public view and effect summaries;
- support bot-vs-bot replay/autoplay or provide a clear path from bot-vs-bot run to replay viewer.

Backward stepping is preferred but not mandatory if reset/replay-through-Rust provides safe deterministic navigation for Race-to-N.

### 15.5 Replay diagnostics

Replay errors MUST be safe and user-readable. They SHOULD include:

- parse failure;
- unsupported version;
- wrong game;
- command validation failure;
- hash mismatch if applicable;
- truncated or oversized payload;
- no-op empty replay.

Diagnostics MUST NOT expose internal hidden state or panic/backtrace details.

---

## 16. Dev/debug panel requirements

### 16.1 Purpose

The dev/debug panel exists to support Rulepath development, replay understanding, and smoke validation without degrading the public baseline.

It MUST be secondary to play. It must not be the main app.

### 16.2 Allowed viewer-safe fields

The panel MAY show:

- API version;
- feature flags/operation support;
- game id/display name;
- rules/data/schema versions;
- match id/handle if safe;
- seed/setup metadata;
- selected mode;
- active actor/current viewer;
- freshness token or token summary;
- action tree summary/count;
- selected action path;
- pending command status;
- effect cursor;
- effect log summaries;
- replay metadata;
- command log if safe;
- normalized diagnostics;
- smoke/test status hints.

### 16.3 Forbidden or restricted fields

The panel MUST NOT expose:

- full internal state by default;
- hidden/private state;
- unrevealed future random outcomes;
- bot-only facts;
- private licensed content;
- arbitrary raw Rust panic/backtrace text;
- unsafe imported replay payload as rendered HTML;
- secret data in DOM attributes, test IDs, CSS classes, local storage, console logs, replay exports, or inspector payloads.

For Race-to-N, full state is not hidden, but the shell must not normalize the habit of public internal-state dumps. Internal-state inspectors belong in test harnesses or explicitly non-public debug builds, not the normal static build.

### 16.4 Dev toggle

The dev panel toggle MUST be clearly labeled and keyboard accessible. A CSS toggle alone is not a security boundary. If data is unsafe for public/static output, it must not be loaded into the browser payload at all.

### 16.5 Stale-action diagnostic

A stale-action demonstration is optional. If retained, it MUST live in the dev/debug panel, be safe, and not clutter normal play.

---

## 17. Accessibility and responsive requirements

Gate 3’s accessibility target is a practical baseline suitable for a public-presentable local demo. It is not a full formal WCAG audit, but the shell must be designed with accessibility as a core requirement.

### 17.1 Landmarks and structure

The shell MUST provide:

- meaningful document title;
- main landmark or equivalent semantic structure;
- clear headings for setup, game board/status, actions, log, and dev/replay panel;
- form labels for setup controls;
- button elements for actions where possible;
- accessible names for interactive controls;
- non-color cues for state/seat/winner/error distinctions.

### 17.2 Keyboard behavior

The shell MUST be usable by keyboard for Gate 3 flows:

- load app;
- select Race-to-N;
- choose/setup mode;
- start match;
- choose a legal action;
- run/pause bot turn or autoplay;
- open/close dev panel;
- import/export replay where practical;
- step replay;
- start a new match.

Keyboard focus MUST remain visible and predictable. Do not remove native focus outlines unless a robust visible replacement is provided.

### 17.3 Focus management

The app SHOULD manage focus after major state transitions:

- after match start, focus should move to or remain near the active play/action area;
- after applying an action, focus should land predictably on the next legal action region or status;
- after terminal state, focus should not be lost;
- after opening/closing dev panel or import dialog, focus should return logically;
- after replay import success/failure, focus should move to the relevant status/diagnostic.

### 17.4 Accessible names and screen-reader summaries

The UI MUST use accessible names for controls. Visible text labels are preferred when practical.

The app SHOULD provide concise screen-reader-accessible summaries for:

- current Race-to-N counter/target/status;
- active actor;
- legal action count and labels;
- last action/effect;
- terminal winner;
- replay cursor/progress.

### 17.5 Reduced motion

The app MUST honor `prefers-reduced-motion` and preserve all essential information without relying on animation. A safe local preference override MAY be persisted.

### 17.6 Responsive layout

The app MUST be usable on desktop and common mobile-width screens. Minimum acceptable behavior:

- no horizontal overflow in normal play;
- actions remain reachable;
- status and counter remain readable;
- effect/replay log can collapse/stack below play area;
- dev panel does not crowd out core play on small screens;
- tap/click targets are reasonably sized.

### 17.7 Color and contrast

The UI MUST NOT rely on color alone. Use text, labels, icons/shapes, outlines, or position in addition to color for important state. Contrast should be sufficient for readable text and visible focus, with special care around disabled/pending states.

---

## 18. Local build/static deployment requirements

Gate 3 targets local static output only.

### 18.1 Required build behavior

The following MUST succeed:

- Rust/WASM build for `wasm32-unknown-unknown` through the web package script path.
- `npm ci` in `apps/web` in CI.
- `npm run build` in `apps/web`.
- TypeScript typecheck as part of build.
- Vite production build output includes the WASM artifact.
- Built app can be served locally with a static file server or Vite preview-equivalent.
- No backend is required.

### 18.2 WASM artifact placement

The implementation MUST ensure the built static app can locate the WASM artifact under local static serving. The current app fetches `/wasm_api.wasm` by absolute root path. Gate 3 MUST revisit that assumption because Vite’s base path controls production asset paths and local static serving may not always be root-mounted.

Acceptable outcomes:

- document and enforce root-relative local serving if that is intentionally the only supported build mode; or
- prefer a base-aware artifact URL that works with Vite build/preview and local static paths.

The preferred Gate 3 direction is base-aware/local-static robust asset loading, not silent dependence on `/` unless explicitly documented and tested.

### 18.3 Preview/static smoke

Gate 3 SHOULD include a smoke path that serves the built `dist` output, not only the dev server. `npm run preview` or an equivalent local static server is sufficient.

The smoke should prove:

- the production build loads;
- WASM fetch succeeds from built output;
- the app can start a match;
- legal actions render;
- a human action and bot turn can apply;
- replay/dev panel basics work.

### 18.4 No hosted deployment

Gate 3 MUST NOT require hosted deployment. The spec does not require GitHub Pages, Netlify, Vercel, Cloudflare, Firebase, Surge, Render, or any equivalent service.

Hosted deployment can be evaluated in a later gate or release-prep pass.

---

## 19. Testing and CI requirements

### 19.1 Existing checks must remain green

Gate 3 MUST preserve existing Rust/game/tool evidence. At minimum:

- Gate 0 hygiene remains green.
- Gate 1 game smoke remains green.
- Gate 2 benchmark lanes remain valid and not weakened.
- Existing `race_to_n` rule tests, property tests, replay tests, bot tests, serialization tests, golden trace checks, simulations, fixture checks, rule coverage, boundary checks, and docs link checks continue to pass unless intentionally updated with documented rationale.

Tests must not be deleted or weakened merely to achieve green output.

### 19.2 WASM/API tests

Existing low-level WASM smoke scripts SHOULD remain useful, but they are not sufficient Gate 3 browser-shell proof.

WASM/API smoke MUST verify:

- WASM artifact loads/instantiates;
- required exports or client operations exist;
- version/feature report works;
- new match works;
- public view fetch works;
- action tree fetch works;
- legal action apply works;
- bot turn works;
- effect fetching works;
- stale-action diagnostic remains safe;
- replay export/import operations work if exposed at this layer.

### 19.3 Browser UI smoke

Gate 3 MUST add or upgrade browser smoke coverage so it exercises the rendered app as a user-facing shell.

Minimum browser smoke MUST verify:

- built app or preview app loads in a browser;
- the normal page is not dominated by raw JSON/debug UI;
- game picker shows Race-to-N;
- match setup can start a match;
- public view/status appears;
- Rust-supplied legal actions appear as user controls;
- one human action applies;
- one bot turn applies;
- stale-action diagnostics remain safe and debug-labeled if exposed;
- effect log appears and updates;
- replay/dev panel opens and shows safe basics;
- replay export/import/step works at least minimally;
- bot-vs-bot step/autoplay can advance;
- reduced-motion path can be enabled or emulated;
- keyboard/focus smoke passes for the critical flow.

### 19.4 Puppeteer vs Playwright decision

Current repository scripts are Node smoke scripts, not browser E2E tests. They directly instantiate/use WASM and cannot prove the rendered shell.

Gate 3 SHOULD use the smallest tool that cleanly validates the required browser behavior.

- If a lightweight Puppeteer script can run the built/preview app, click through the required flows, inspect accessible names/text, and remain reliable in CI, prefer Puppeteer to minimize tooling churn.
- Recommend Playwright only if Gate 3 implementation materially benefits from Playwright’s cross-browser projects, web-first assertions, auto-waiting, tracing, role-based locators, or CI reliability enough to justify the added dependency and browser install surface.
- Do not add Playwright merely because it is trendy.
- If Playwright is adopted, scope it narrowly: one or a few smoke specs for the Gate 3 flows, not a broad test framework migration.

### 19.5 Accessibility smoke

Gate 3 MUST include practical accessibility smoke checks. They can be manual checklist plus automated smoke where practical. Minimum evidence:

- controls have accessible names;
- critical controls are keyboard reachable;
- focus is visible;
- reduced-motion preference path works;
- no important state relies on color alone;
- dev panel toggle is keyboard accessible;
- replay controls are keyboard accessible.

An automated axe-style scan MAY be added if tooling churn is acceptable, but it is not mandatory for Gate 3.

### 19.6 No-leak checks

For Race-to-N, hidden information is not applicable. Gate 3 MUST still include a no-leak review/checklist covering:

- browser payloads;
- action tree;
- diagnostics;
- effect log;
- DOM attributes;
- test IDs;
- console logs;
- local storage/session storage;
- replay exports;
- bot explanations/logs;
- candidate rankings if any;
- dev inspector.

### 19.7 CI integration

Gate 3 SHOULD integrate web shell smoke into existing Gate 1/Gate 3-appropriate CI without making benchmark lanes noisy. The CI flow should remain clear:

- Rust hygiene and game tests are rule authority.
- WASM/API smoke proves bridge basics.
- Browser smoke proves static shell behavior.
- Benchmark lanes remain benchmark lanes.

---

## 20. Documentation requirements

Gate 3 implementation MUST update documentation sufficiently for a later contributor to understand and verify the shell.

Required documentation updates during implementation:

- Update `progress.md` with Gate 3 status and verification evidence.
- Update `specs/README.md` to point to the Gate 3 spec and flip its index row status from `Not started` to `Planned` on adoption (and to `Done` once the Exit criteria pass with evidence).
- Update `apps/web` documentation or root README with local build/preview/smoke commands.
- Document the WASM client boundary and operation groups.
- Update `games/race_to_n/docs/UI.md` or add an equivalent game UI note so Race-to-N’s UI status reflects the Gate 3 shell rather than only the Gate 1 harness.
- Document replay import/export behavior, supported version(s), and safety limits.
- Document dev panel data-source safety.
- Record any decision to retain raw ABI or adopt `wasm-bindgen`; if the decision changes architecture materially, use an ADR.

Documentation SHOULD NOT invent future game guarantees. It should clearly state that Gate 3 is Race-to-N only and that later games plug into the shell later.

---

## 21. Acceptance criteria

Gate 3 is accepted only if all criteria below are satisfied.

### 21.1 Scope and product acceptance

- The app is a public-presentable baseline, not a raw debug harness.
- `race_to_n` is the only implemented playable game.
- No `three_marks` or later game work appears.
- Game picker exists and has one active Race-to-N option.
- Match setup exists and is understandable.
- Active Race-to-N play is clear, readable, and responsive.
- Normal play does not rely on raw JSON.
- Debug/replay tools are secondary and deliberate.
- No hosted deployment is required.

### 21.2 Authority boundary acceptance

- Rust remains behavior authority.
- TypeScript does not compute legality, apply rules, choose bots, or own replay semantics.
- Legal controls are generated from Rust action tree data.
- Action submission uses Rust action paths and freshness tokens.
- Stale/invalid action handling is Rust-diagnostic-driven and safe.
- Effect log and animation are driven by Rust semantic effects.
- Replay import/export/stepping is Rust-validated/Rust-authoritative.

### 21.3 WASM/client acceptance

- React components do not call low-level WASM exports directly.
- A typed TypeScript client boundary owns loading, memory, encoding/decoding, JSON parsing, response normalization, and typed operation methods.
- Required operations exist or are documented with equivalent capabilities: version/features, list games, new match, get view, get action tree, apply action, run bot turn, get effects, replay export/import, replay step/reset.
- Missing/malformed WASM/API failures produce clear diagnostics.
- Raw ABI retention is justified; `wasm-bindgen` migration is not required unless a concrete blocker is documented.

### 21.4 Mode acceptance

- Human-vs-bot Race-to-N can start, play at least one human turn, play at least one Rust bot turn, and finish or continue normally.
- Hotseat Race-to-N can start and alternate active seat actions.
- Bot-vs-bot step/autoplay can advance using Rust bot turns and pause/reset.
- Replay viewer can show a generated replay and step/reset through Rust-authoritative state/effects.
- Replay export/import round trip works locally for a Race-to-N run.

### 21.5 Accessibility/responsive acceptance

- Keyboard-only user can complete the critical start/play/bot/dev/replay flow.
- Focus indicators are visible.
- Controls have accessible names.
- Important information does not rely on color alone.
- Reduced-motion preference is honored.
- Layout is usable on desktop and common mobile-width screens.

### 21.6 Build/deployment acceptance

- `npm ci` and `npm run build` succeed in `apps/web`.
- WASM builds for `wasm32-unknown-unknown` through the app script path.
- Vite production output includes/serves the WASM artifact.
- Built output can be served locally with no backend.
- Asset/WASM paths are compatible with the documented local static serving target.

### 21.7 Testing acceptance

- Existing Rust/game/tool tests remain green.
- Existing Gate 0/1/2 checks are not weakened.
- WASM/API smoke covers bridge operations.
- Browser shell smoke covers user-facing Gate 3 flows.
- Replay import/export and replay stepping are tested.
- Stale-action diagnostics remain safe.
- Dev panel safe basics are tested.
- Reduced-motion/keyboard/focus baseline is tested or checklist-verified.
- No raw JSON/debug-first UI dominates normal play.

### 21.8 Documentation acceptance

- Gate 3 behavior, local build/preview, smoke commands, WASM boundary, replay import/export, and dev panel safety are documented.
- Race-to-N UI documentation is updated to Gate 3 reality.
- Any architectural binding/tooling decision is recorded.

---

## 22. Explicitly deferred work

The following are explicitly deferred beyond Gate 3:

- `three_marks`.
- Additional games.
- Hosted deployment.
- Hosted multiplayer.
- Accounts/databases/matchmaking/chat/ranked play.
- Server persistence.
- Broad game catalog UX.
- Production art/final branding.
- Complex animation/showcase-level polish.
- Canvas/PixiJS migration.
- DSL/YAML behavior.
- Public MCTS/ISMCTS/Monte Carlo-style bots.
- ML/RL.
- Private monster-game work.
- Full hidden-information renderer proof beyond no-leak design review.
- Comprehensive accessibility audit beyond Gate 3 baseline.
- Full cross-browser E2E matrix unless Playwright is justified.
- Automatic match persistence.
- Local storage of hidden or authoritative state.

---

## 23. Risks and design decisions

### 23.1 Raw ABI and JSON bridge risk

Risk: the current raw ABI requires manual memory/string/JSON handling and can leak low-level concerns into React.

Decision: retain it for Gate 3 if wrapped behind a typed client boundary. This addresses the immediate risk without imposing binding-stack churn. Reevaluate `wasm-bindgen` later if raw ABI blocks typed safety, maintainability, testing, or asset integration.

### 23.2 React state entropy risk

Risk: adding setup, play, bot, replay, dev panel, effects, diagnostics, and reduced motion on top of scattered local state will create bugs.

Decision: require reducer/state-machine-style architecture for Gate 3.

### 23.3 Browser smoke gap

Risk: existing smoke scripts exercise WASM directly and may pass while the actual browser shell is broken.

Decision: add rendered-browser smoke coverage for Gate 3. Prefer a lightweight Puppeteer-style path unless Playwright has a concrete advantage.

### 23.4 Vite base/WASM asset risk

Risk: absolute `/wasm_api.wasm` fetch works only when root-mounted and can fail under nested/static local serving.

Decision: Gate 3 must document and test the target serving mode, preferably using base-aware asset loading.

### 23.5 Debug-data leakage risk

Risk: a dev panel that exposes full state in a perfect-information game can normalize unsafe patterns before hidden-information games arrive.

Decision: keep dev data viewer-safe and secondary. Do not load unsafe data into public/static browser payloads. A CSS toggle is not a boundary.

### 23.6 Replay authority risk

Risk: TypeScript replay stepping could become a second rules engine.

Decision: replay import/export/stepping must go through Rust validation/projection. TypeScript may drive UI cursor only.

### 23.7 Scope creep risk

Risk: polishing a public-presentable shell can drift into showcase work or second-game work.

Decision: keep the visual target “early polished shell,” not showcase. Race-to-N only. No `three_marks`.

### 23.8 Preview API decision

Risk: later games need Rust-generated previews; Race-to-N may not.

Decision: `preview_action` is not mandatory for Gate 3 if Race-to-N flows are clear without it. The UI must not implement TypeScript previews as a substitute. The API/client design should leave room for Rust previews later.

---

## 24. Research notes and citations

Research was consulted on 2026-06-06. Repository exact-commit evidence remains the authority for Rulepath-specific requirements. These notes only support current web/Rust/browser practice decisions.

### 24.1 Rust/WebAssembly browser integration

- The `wasm-bindgen` guide describes `wasm-bindgen` as a Rust library/CLI for high-level interactions between Wasm modules and JavaScript, including richer types and generated TypeScript bindings. This supports treating `wasm-bindgen` as a real future option, not as mandatory for Gate 3: [Rust Wasm Bindgen Guide](https://rustwasm.github.io/docs/wasm-bindgen/introduction.html).
- MDN’s Rust-to-Wasm guide presents `wasm-pack`/browser packaging paths for Rust-generated WebAssembly. This supports the idea that generated binding/tooling workflows are established, but adopting them is a build-stack decision: [MDN Rust to Wasm](https://developer.mozilla.org/en-US/docs/WebAssembly/Guides/Rust_to_Wasm).
- MDN describes `WebAssembly.Memory` as raw byte storage accessible between WebAssembly and JavaScript. This supports the requirement that the TypeScript client boundary, not React components, own memory/encoding/decoding when a raw ABI is retained: [MDN WebAssembly.Memory](https://developer.mozilla.org/en-US/docs/WebAssembly/Reference/JavaScript_interface/Memory).

Gate 3 conclusion: retain the raw ABI if stable behind a typed TypeScript client; evaluate `wasm-bindgen` later if concrete blockers emerge.

### 24.2 React state architecture

- React documents `useReducer` as a hook for adding reducer-managed state to a component: [React useReducer](https://react.dev/reference/react/useReducer).
- React’s reducer/context guidance says reducers consolidate state update logic and can be combined with context to manage state of a complex screen: [React reducer and context guide](https://react.dev/learn/scaling-up-with-reducer-and-context).

Gate 3 conclusion: a reducer/state-machine-style model is appropriate for WASM load, setup, match, action, effect, bot, replay, diagnostics, dev panel, and reduced-motion state.

### 24.3 Vite static build and base-path behavior

- Vite’s production build guide states that `vite build` produces an application bundle suitable for static hosting and that the `base` config option rewrites asset paths for nested public paths: [Vite build guide](https://vite.dev/guide/build).
- Vite’s shared options document lists the `base` default as `/` and allows absolute path, full URL, empty string, or `./` for embedded deployment: [Vite base option](https://vite.dev/config/shared-options.html#base).
- Vite’s static deployment guide uses `npm run build` and `dist` output as the basic deployment artifact pattern: [Vite static deployment](https://vite.dev/guide/static-deploy.html).

Gate 3 conclusion: local static output is enough, but WASM artifact loading must be compatible with the documented Vite/static serving base.

### 24.4 Browser E2E testing choices

- Puppeteer is documented as a JavaScript library providing high-level control of Chrome or Firefox, headless by default: [Puppeteer docs](https://pptr.dev/).
- Playwright Test is documented as an E2E framework with Chromium, WebKit, and Firefox support, assertions, isolation, parallelization, and tooling: [Playwright intro](https://playwright.dev/docs/intro).
- Playwright locators emphasize role/text/label locators and auto-waiting/retryability; role locators align tests with user/assistive-technology perception: [Playwright locators](https://playwright.dev/docs/locators).

Gate 3 conclusion: add real browser smoke. Prefer a narrow Puppeteer-style smoke if it covers the flows cleanly; choose Playwright only for material reliability/cross-browser/role-locator benefits.

### 24.5 Accessibility practices for custom game surfaces

- WAI-ARIA Authoring Practices emphasizes keyboard-operable interactive elements, visible/predictable focus, and focus persistence: [WAI-ARIA APG keyboard interface](https://www.w3.org/WAI/ARIA/apg/practices/keyboard-interface/).
- WAI-ARIA APG accessible-name guidance states that focusable interactive elements require accessible names and recommends visible text/native techniques where possible: [WAI-ARIA APG names and descriptions](https://www.w3.org/WAI/ARIA/apg/practices/names-and-descriptions/).
- MDN describes `prefers-reduced-motion` as a media feature for detecting a user setting to minimize nonessential motion: [MDN prefers-reduced-motion](https://developer.mozilla.org/en-US/docs/Web/CSS/@media/prefers-reduced-motion).
- WCAG’s Focus Visible understanding document explains that keyboard users need a visible indicator of keyboard focus: [WCAG Focus Visible](https://www.w3.org/WAI/WCAG22/Understanding/focus-visible.html).
- WCAG’s Use of Color understanding document states that color must not be the only means of conveying information or indicating actions: [WCAG Use of Color](https://www.w3.org/WAI/WCAG22/Understanding/use-of-color.html).
- MDN’s keyboard accessibility guide states that clickable elements must be focusable and interactive elements must be keyboard-operable/focusable with focus styling: [MDN keyboard accessibility](https://developer.mozilla.org/en-US/docs/Web/Accessibility/Guides/Understanding_WCAG/Keyboard).

Gate 3 conclusion: keyboard/focus/accessibility/reduced-motion requirements are core shell requirements, not optional polish.

---

## Appendix A — Minimum Gate 3 verification matrix

| Area | Minimum evidence |
|---|---|
| Rust tests | Existing workspace tests pass; Race-to-N rule/property/replay/bot/serialization tests pass. |
| Golden traces | Existing trace validation remains green; changes require rationale. |
| Boundary | Boundary checks remain green; TypeScript does not own legality. |
| WASM build | `wasm32-unknown-unknown` target builds through web package script. |
| Web build | `npm ci` and `npm run build` succeed in `apps/web`. |
| Static serving | Built output can be served locally; WASM loads from built output. |
| WASM/API smoke | Version/features, list games, new match, view, action tree, apply action, bot turn, effects, replay import/export basics. |
| Browser smoke | Game picker, setup, public view, legal controls, human action, bot turn, effect log, dev panel, replay/export/import, bot-vs-bot step/autoplay. |
| Accessibility smoke | Keyboard path, focus visible, accessible names, reduced motion, no color-only cues. |
| Hidden-info review | All browser surfaces reviewed even though Race-to-N is perfect-information. |
| Documentation | Build/preview/smoke, WASM boundary, replay/import/export, dev panel safety, Race-to-N UI status documented. |

---

## Appendix B — Gate 3 one-sentence success definition

Gate 3 succeeds when a reviewer can build the static app locally, open it in a browser, select and play Race-to-N through Rust/WASM legal actions and bot turns, inspect safe replay/dev information, export/import and step a replay, verify keyboard/reduced-motion basics, and see a coherent public-presentable shell with no TypeScript rule authority and no backend.

## Outcome

Completed on 2026-06-06.

What changed:

- Added the typed TypeScript WASM client and kept the raw ABI behind that module.
- Added Rust WASM catalog, feature report, and replay operation groups.
- Built the reducer-backed React shell with game picker, setup, Race to 21 board, Rust action controls, effect log, play modes, replay UI, developer panel, reduced-motion handling, and base-aware static WASM loading.
- Added raw-ABI, shell-state, static-dist, rendered-browser, accessibility, and no-leak smoke coverage.
- Documented the WASM client boundary, web commands/static serving, Race to 21 UI status, replay safety, and developer-panel whitelist.

Deviations:

- Puppeteer uses system Chrome by default through `/usr/bin/google-chrome` or `PUPPETEER_EXECUTABLE_PATH`; Puppeteer browser download was skipped during installation because the environment already has Chrome.
- The replay/no-leak review whitelists `expected_private_view_hashes.not_applicable` as the schema's explicit perfect-information marker.

Verification:

- `npm --prefix apps/web run smoke:wasm`
- `npm --prefix apps/web run smoke:preview`
- `npm --prefix apps/web run smoke:e2e`
- `npm --prefix apps/web run build`
- `node scripts/check-doc-links.mjs`

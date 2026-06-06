# Gate 5 — Column Four public polish

## 1. Header and gate summary

**Spec ID:** `gate-5-column-four-public-polish`  
**Stage:** 3  
**Gate:** Gate 5 — Column Four public polish  
**Status**: Done
**Date:** 2026-06-06  
**Owner:** joeloverbeck  
**Authority order:** `docs/FOUNDATIONS.md` → the `docs/README.md` foundation set → `docs/ROADMAP.md` (Gate 5) → this spec. Where this spec and a foundation document disagree, the foundation document wins.  
**New official game id:** `column_four`  
**Public game name:** `Column Four`  
**Default variant id:** `column_four_standard`  
**Rules id:** `column_four-rules-v1`

> **Reader orientation.** Sections 2–22 are the **detailed requirements reference**. The
> canonical decomposition scaffolding an AGENT-TASK decomposer needs — Work breakdown, Exit
> criteria mapped to ROADMAP, FOUNDATIONS & boundary alignment, Sequencing, and Assumptions —
> is in the lettered sections **A–E** immediately below, each pointing back into the detailed
> body.

This gate admits `Column Four` as the next official Rulepath game and uses it as the first public-facing milestone where the project should feel like a real playable portfolio application, not merely a diagnostic shell with a board attached.

`Column Four` is a neutral vertical four-in-a-row connection game: seven columns, six rows, two seats, perfect information, no rule randomness, alternating turns, gravity-based placement, win by four contiguous same-seat pieces horizontally, vertically, or diagonally, and draw when the board fills without a winning line.

Gate 5 must deliver an official-game-quality implementation with complete game documentation, deterministic replay, golden traces, simulation and benchmark coverage, WASM/web exposure, a polished accessible board UI, and at least two bot levels: a Level 0 random-legal bot and a Level 2 authored tactical policy suitable for public demonstration.

This is **not** a ticket decomposition. This document is the requirements/specification that a later implementation agent may consume and decompose.

## A. Work breakdown

Bounded, AGENT-TASK-sized items in dependency order. Each maps into the detailed requirements
in sections 5–19. Items at the same depth can run in parallel once their dependencies land.

| ID | Item | Primary targets | Depends on | Detail |
|---|---|---|---|---|
| WB-1 | Crate skeleton + workspace wiring | `games/column_four/{Cargo.toml,src/lib.rs,src/ids.rs,src/state.rs,src/setup.rs,src/variants.rs}`; add `games/column_four` to root `Cargo.toml` members | — | §5, §7 |
| WB-2 | Rules core: coordinates, occupancy, gravity/landing, legal columns, turn order, terminal win (H/V/both diagonals) + draw, diagnostics | `games/column_four/src/{actions.rs,rules.rs}` | WB-1 | §7 |
| WB-3 | Public view + visibility projection (viewer-safe perfect-info, winning-line metadata, column summaries, Rust-provided preview) | `games/column_four/src/visibility.rs`, `src/ui.rs` | WB-2 | §8, §10 |
| WB-4 | Semantic effects + ordered effect log | `games/column_four/src/effects.rs` | WB-2 | §8 |
| WB-5 | Replay support (Rust-owned projection) | `games/column_four/src/replay_support.rs` | WB-2, WB-3, WB-4 | §13 |
| WB-6 | Bots: Level 0 (reuse `ai-core::RandomLegalBot`) + Level 2 authored tactical policy with viewer-safe rationale | `games/column_four/src/bots.rs` | WB-2, WB-3, WB-4; WB-15 docs (COMPETENT-PLAYER / EVIDENCE-PACK) before final Level 2 admission | §12 |
| WB-7 | Rust tests: unit/rule/property/serialization/visibility/replay/bot | `games/column_four/tests/**`, `src` unit tests | WB-2…WB-6 | §13, §18 |
| WB-8 | Golden traces + fixtures | `games/column_four/tests/golden_traces/**`, `games/column_four/data/{fixtures/**,manifest.toml,variants.toml}` | WB-5 | §13 |
| WB-9 | Benchmarks + honest thresholds | `games/column_four/benches/**`, threshold file | WB-2…WB-6 | §14 |
| WB-10 | WASM exposure: register `column_four` (`RegisteredGame` enum, `MatchRecord` variant, all op match arms) | `crates/wasm-api/src/lib.rs` | WB-2…WB-6 | §9 |
| WB-11 | Web: `ColumnFourBoard` + integration | new `apps/web/src/components/ColumnFourBoard.tsx`; `ColumnFourPublicView` in `apps/web/src/wasm/client.ts`; play-mode routing in `apps/web/src/main.tsx`; discriminant in `apps/web/src/components/ReplayViewer.tsx`; `GamePicker.tsx` (catalog-driven, usually no edit) | WB-10 | §10, §11 |
| WB-12 | Web smoke + a11y/no-leak | new `apps/web/e2e/column-four.smoke.mjs`; extend `apps/web/e2e/NO-LEAK-A11Y-CHECKLIST.md`; `apps/web/package.json` smoke chain | WB-11 | §18 |
| WB-13 | Tool registration | match arms in `tools/{simulate,replay-check,fixture-check,rule-coverage}/src/main.rs` (seed-reducer/trace-viewer optional — see A-4) | WB-8 | §18 |
| WB-14 | CI: per-game steps | `.github/workflows/gate-1-game-smoke.yml` (hardcoded per-game steps) and `gate-2-benchmarks.yml` | WB-9, WB-12, WB-13 | §18 |
| WB-15 | Official-game docs (filled, not template shells) | `games/column_four/docs/{RULES,MECHANICS,RULE-COVERAGE,UI,AI,BENCHMARKS,SOURCES,GAME-IMPLEMENTATION-ADMISSION,COMPETENT-PLAYER,BOT-STRATEGY-EVIDENCE-PACK,PUBLIC-RELEASE-CHECKLIST}.md` | parallels WB-2…WB-14 | §15, §17 |
| WB-16 | Status hygiene + mechanic atlas | `docs/MECHANIC-ATLAS.md`; `specs/README.md` (→ `Done`); root `README.md`; `docs/ROADMAP.md` status; `progress.md`; `apps/web/README.md` | all | §16, §19 |

## B. Exit criteria — mapped to ROADMAP Gate 5

Mapped row-for-row to the `docs/ROADMAP.md` Gate 5 exit list. The detailed acceptance evidence
lives in §20.

| ROADMAP Gate 5 exit line | Satisfied by | Verification |
|---|---|---|
| public page feels polished | WB-11; §10 public quality bar | Browser E2E smoke (`column-four.smoke.mjs`): play-first default, board renders 7×6 |
| legal columns only are clickable | WB-2, WB-3, WB-11; §9, §10 | Smoke: only legal columns submit; full columns inert; §20 Rust/WASM-boundary checks |
| previews are Rust-safe | WB-3, WB-11; §8 preview, §10 | Smoke: hover/focus preview from Rust-provided data; no-leak check |
| animations come from semantic effects | WB-4, WB-11; §10 animation | Smoke: drop animation effect-log-driven, settles to Rust view; reduced-motion path |
| bot explanations are available for non-random bot | WB-6; §12 Level 2 rationale | Bot tests + smoke: Level 2 rationale present and viewer-safe |
| replay viewer smoke passes | WB-5, WB-11, WB-12; §13 replay viewer smoke | `column_four` replay export/import/step renders `ColumnFourBoard` projection |
| benchmark and UI smoke coverage exists | WB-9, WB-12, WB-14; §14, §18 | `cargo bench` surfaces + UI/browser smoke in CI |
| mechanic atlas records repeated coordinate/line pressure | WB-16; §16 | `docs/MECHANIC-ATLAS.md` records `three_marks`→`column_four` second-use pressure |

ROADMAP **Not allowed** (carried into §6 Non-goals and §C below): debug-first public screen;
TypeScript legality; early Canvas/PixiJS without evidence; `engine-core` grid nouns.

## C. FOUNDATIONS & boundary alignment

| Principle | Stance | Rationale |
|---|---|---|
| §2 Behavior authority | aligned | All legality, gravity/landing, terminal/winner/draw, view projection, effects, replay, and bot decisions stay in Rust; TypeScript renders Rust-provided views/actions only (§4 Rust authority, §9 boundary rule). |
| §3 `engine-core` is a contract kernel | aligned | No board/grid/cell/column/row/line/gravity/pattern noun enters `engine-core`; logic stays local to `games/column_four` (§4, §16). Enforcement reach: see A-5. |
| §4 `game-stdlib` is earned | aligned | Second-use pressure recorded; no promotion this gate; extraction deferred to a later gate after `directional_flip` (§16). |
| §5 Static data is not behavior | aligned | Variants/fixtures/traces/thresholds are typed content only; no selectors, line-as-logic, or bot-policy branches in static data (§4 static-data boundary). |
| §7 Public UI is central product work | aligned | Play-first, legal-only seven-column controls, React + SVG default, effect-driven animation, dev panel secondary (§10). |
| §8 Public bots are product opponents | aligned | Level 0 + Level 2 authored, deterministic under seed, same legal-action API, no search/ML/RL (§12). Level 1 intentionally skipped — see A-3. |
| §11 Universal acceptance invariants | aligned | Viewer-safe views, no-leak DOM/storage/replay, deterministic replay/hash, fail-closed trace validation, evidence coverage (§8, §11, §13, §18). |
| §12 Stop conditions | clear | No engine-core mechanic nouns, no TS legality, no clickable illegal moves, no guessed-diff animation, no hidden-info leak, no bot API bypass, no search/ML/RL bot — all explicitly forbidden (§6, §9, §12, §16). |
| §13 ADR triggers | clear | No ADR-requiring decision is made; Canvas/Pixi/WebGL correctly gated behind a future ADR and kept out of scope (§6, §10). |

## D. Sequencing

- **Predecessor:** Gate 4 (`three_marks`, Stage 2) — `Done` in `specs/README.md`. `column_four`
  follows the official-game path proven by `race_to_n` and `three_marks` and reuses the
  `three_marks` crate shape.
- **Successor:** Gate 6 (`directional_flip`, Stage 4) — not yet specced. The spatial-primitive
  extraction decision (board/grid/cell/line/gravity) is deferred to that gate's comparison
  evidence; Gate 5 records pressure only (§16).
- **Admission rule:** Gate 5 is admitted only after its §B exit criteria pass with the §20
  evidence. `column_four` must not extract primitives into `engine-core`/`game-stdlib` to
  proceed; if an implementer feels forced to, the gate stops and produces a primitive-pressure
  decision note/ADR (§16 hard-gate rule).

## E. Assumptions (one-line-correctable)

- **A-1:** The repository state described in §2 matches the current working tree; if the tree has
  moved, re-verify the registry/tool/web touch-points named in §A.
- **A-2:** `column_four` follows the shared `wasm-api` `SCHEMA_VERSION`/`RULES_VERSION` constants
  plus a per-game `rules_version` string `column_four-rules-v1`, mirroring `three_marks`.
- **A-3:** Level 1 is intentionally skipped (Level 0 + Level 2); ROADMAP Gate 5's "baseline and
  preferably Level 2 policy bot" permits this, and Level 2 subsumes Level 1's tactics. `column_four`
  is the first game to fill `COMPETENT-PLAYER.md` / `BOT-STRATEGY-EVIDENCE-PACK.md` — only the
  `templates/*` versions exist, no worked example (`three_marks` stopped at Level 1).
- **A-4:** `seed-reducer` and `trace-viewer` are currently `race_to_n`-only; `three_marks` is not
  wired into either. By that precedent, `column_four` need not register with them unless tooling
  conventions change (WB-13 treats them as optional).
- **A-5:** `scripts/boundary-check.sh` enforces only `board|grid` of this game's spatial nouns
  (its pattern is `board|card|deck|grid|suit|resource|capture|hand|pile|trick|pot|auction|betting|drafting`).
  Leakage of `line`/`cell`/`column`/`row`/`gravity`/`pattern` into `engine-core` is caught by
  review, not by the script — see §18.

## 2. Current repository state

The repository is already beyond a skeleton. It has a Rust workspace, a thin WASM boundary, a static React web shell, two admitted official games, and several validation tools.

The current state relevant to Gate 5 is:

- `specs/README.md` records Gates 0 through 5 as done and points Gate 5 at the archived completion spec.
- `docs/ROADMAP.md` identifies Gate 5 as the first true showcase after `three_marks`. Its expected pressure points are gravity, legal column targets, Rust-safe previews, line detection under gravity, effect-log-driven drop animation, win-line effects, bot explanations, replay viewer smoke, benchmarks, UI smoke, and atlas updates.
- `docs/FOUNDATIONS.md` is the governing contract. It says Rulepath is Rust-first, deterministic, replayable, testable, portfolio-quality, and public-facing. It also says Rust owns game behavior and TypeScript/React only presents viewer-safe state and submits actions.
- `docs/ENGINE-GAME-DATA-BOUNDARY.md` forbids `engine-core` from learning concrete mechanic nouns such as board, grid, cell, coordinate, adjacency, line, card, hand, trick, pot, auction, and similar game concepts. Those belong in games, or later in `game-stdlib` only after enough repeated pressure is documented.
- `docs/OFFICIAL-GAME-CONTRACT.md` defines official game admission as more than a playable surface: the game must be correct, documented, replayable, tested, benchmarked, bot-supported, IP-safe, and publicly presentable.
- `docs/UI-INTERACTION.md` requires a consumer-quality play-first UI, Rust-owned legal actions and previews, effect-driven animations, React + SVG by default, and dev/debug tools that stay secondary.
- `docs/AI-BOTS.md` requires bots to use the same legal action APIs as other actors, remain deterministic under seed and game state, expose viewer-safe explanations, and avoid public MCTS, Monte Carlo, ML/RL, runtime LLMs, and external services.
- `docs/TRACE-SCHEMA-v1.md` defines a strict replay/trace JSON schema. It rejects unknown fields, duplicate IDs, behavior-looking fields, and malformed or unsupported trace documents.
- `docs/MECHANIC-ATLAS.md` already anticipates `column_four` as a comparison point for repeated fixed 2D occupancy and simple line detection first exercised by `three_marks`, while gravity placement remains local-only until more games exert pressure.
- `docs/IP-POLICY.md` specifically treats neutral naming and original expression as mandatory for commercial-adjacent abstract games, including four-in-a-row style games.
- `templates/**` contains required official-game documentation templates for rules, mechanics, rule coverage, UI, AI, benchmarks, sources, implementation admission, bot strategy evidence, primitive pressure, and public release review.
- `archive/specs/**` shows prior gate style and closeout expectations. Gate 4 is especially important: it admitted `three_marks` while explicitly deferring engine-core board/grid/cell/line extraction until later pressure.

Current official games:

- `race_to_n` is the arithmetic foundation game. It has complete official-game docs, traces, simulation/replay/fixture/rule-coverage coverage, benchmark docs, and a Level 0 random legal bot.
- `three_marks` is the first spatial placement game. It uses a local fixed 3×3 occupancy model, stable cell IDs, line detection, Rust-owned legal targets and winner/draw detection, golden traces for win/draw/diagnostics/bot/WASM export, a React board surface, and Level 0 plus Level 1 authored bots.

Current web state:

- `apps/web` has a static React shell backed by `crates/wasm-api`.
- The shell loads the Rust/WASM catalog, starts seeded matches, renders public views, receives legal action trees, applies chosen action paths, shows semantic effect logs, supports hotseat/human-vs-bot/bot-vs-bot modes, exports/imports replay documents, steps replay projections, and keeps a developer panel secondary.
- `ThreeMarksBoard.tsx` currently demonstrates Rust-projected board cells, Rust-provided legal targets, terminal win/draw rendering, bot explanation display, replay projection reuse, keyboard focus smoke coverage, no-leak checks, and reduced-motion behavior.
- `ReplayViewer.tsx` currently has explicit rendering for `three_marks`; Gate 5 must extend replay smoke and projection rendering to `column_four` rather than leaving it as a generic text-only fallback.
- `crates/wasm-api/src/lib.rs` currently has explicit registration for `race_to_n` and `three_marks`. Gate 5 must add `column_four` to that registry and preserve the thin-WASM-boundary shape.

Current CI/tooling state:

- Gate 0 hygiene runs format, clippy, build, and workspace tests.
- Gate 1 smoke runs quick simulations, replay checks, fixture validation, rule coverage, boundary checks, WASM smoke, web build, UI smoke, browser E2E, and docs link checks for existing games.
- Gate 2 benchmarks run non-gating benchmark smoke on pull requests and threshold gates elsewhere for existing games.
- `scripts/boundary-check.sh` hard-fails if `engine-core` contains mechanic vocabulary including `board`, `grid`, and other forbidden nouns, and if `engine-core` gains forbidden dependencies.

The important conclusion: `Column Four` should follow the official-game path proven by `race_to_n` and `three_marks`, but it should not extract generic board primitives yet. It must create local game-owned spatial/gravity/line logic, record mechanic pressure, and improve the public-facing shell enough that Rulepath starts to feel like a real game site.

## 3. Why this gate is next

Gate 5 is the right next gate because `three_marks` proved a small fixed 2D placement game and Gate 5 applies that pressure to a more public, more physically intuitive, more animated game without prematurely generalizing the engine.

`Column Four` adds several important product and architecture pressures:

- **Gravity placement:** the chosen action is a column, but the resulting piece lands in a row determined by Rust.
- **Column-first interaction:** the user’s real decision is one of seven columns, not one of forty-two cells.
- **Line detection under gravity:** the game has horizontal, vertical, and diagonal wins in a larger state space than `three_marks`.
- **Public comprehension:** a falling piece, terminal line, and draw state are easy for users to understand visually, making this a good public showcase.
- **Bot quality:** the game is simple enough for a credible authored tactical bot but rich enough to expose why search/solver work is out of scope.
- **Mechanic atlas evidence:** it becomes the second data point for fixed 2D occupancy and simple line detection after `three_marks`, while creating first-use pressure for gravity/drop placement and column actions.

Gate 5 should therefore be intentionally bounded: official-game complete, public-polished, and architecture-disciplined, but not a generalized grid-game framework, not an AI research project, and not a broad web-platform rewrite.

## 4. Foundation alignment

Gate 5 must align with `docs/FOUNDATIONS.md` before all local convenience decisions.

### Rust authority

Rust must own all game behavior:

- match setup;
- variant identity;
- seat identity and turn order;
- legal column calculation;
- stale action rejection;
- full-column rejection;
- board occupancy;
- gravity and landing row;
- terminal win/draw detection;
- winning line coordinates;
- public view projection;
- replay projection;
- semantic effects;
- diagnostics;
- bot decisions;
- bot rationale/explanations;
- serialization and stable hashing.

TypeScript/React must not compute or infer authoritative game behavior. It may render Rust-provided views, render Rust-provided legal actions, request Rust-provided previews, show Rust-provided effects, and submit the selected Rust action path back to WASM.

### Product-first public polish

Gate 5 must move the public web shell away from “developer demo with a board” and toward “small polished playable game.” The board should have a thoughtful visual hierarchy, clear game status, natural hover/focus previews, accessible controls, readable effect feedback, and a non-debug-first default layout.

### Determinism and replay

All state transitions, bot turns, effect logs, public views, and replay projections must be deterministic from the trace inputs, seed, variant, rules version, and command sequence. Animation timing may be presentation-layer behavior, but the post-animation state must settle exactly to the Rust-projected view.

### Minimal engine-core

Gate 5 must not teach `engine-core` about boards, grids, cells, columns, rows, lines, gravity, or patterns. Those concepts remain local to `games/column_four` for this gate. If repeated mechanics feel uncomfortable, document the pressure rather than extracting them.

### Static data boundary

Static files may describe variants, fixture metadata, docs, examples, source notes, and benchmark thresholds. They must not contain behavior selectors, tactical rules, legal-move logic, trigger conditions, line definitions that behave like executable logic, bot policy branches, or Rust-surrogate decision tables.

### Public IP safety

The game may use the general abstract method of a vertical four-in-a-row column game, but all public names, rules prose, UI copy, board visuals, token visuals, colors, icons, layout, and marketing language must be original and neutral.

## 5. Scope

Gate 5 includes the following product, engine, WASM, web, documentation, and validation scope.

### Game admission scope

Create and admit a new official game:

- game crate: `games/column_four`;
- internal game id: `column_four`;
- public name: `Column Four`;
- default variant id: `column_four_standard`;
- rules id: `column_four-rules-v1`;
- docs directory: `games/column_four/docs/`;
- fixture/data directory consistent with existing game patterns;
- golden trace directory consistent with existing game patterns;
- benchmarks and thresholds consistent with current benchmark tooling.

`Column Four` must be official-game complete by the standards of `docs/OFFICIAL-GAME-CONTRACT.md`; merely showing a board in the web shell is insufficient.

### Rules scope

The standard variant must implement:

- a 7-column × 6-row board;
- two seats, `seat_0` and `seat_1`;
- no hidden information;
- no rule randomness;
- alternating turns;
- legal move = choose a non-full column on the active turn;
- chosen piece lands in the lowest empty row of that column;
- win = four contiguous same-seat pieces horizontally, vertically, or diagonally;
- draw = all 42 cells occupied and no winning line;
- terminal = win or draw;
- no legal player moves after terminal;
- stale commands rejected;
- non-active seat commands rejected;
- invalid column commands rejected;
- full-column commands rejected with a public diagnostic.

### Rust game implementation scope

The game crate must own its local coordinate, occupancy, gravity, line detection, terminal resolution, public view, effects, bot policy, replay support, tests, fixtures, and benchmarks.

The crate should follow the shape proven by `games/three_marks` where useful: typed ids, local state, local action parsing/validation, local effects, local visibility projection, local bot module, local replay support, docs, tests, and benchmarks.

### WASM exposure scope

`crates/wasm-api` must expose `column_four` through the same conceptual browser-facing operations used for existing games:

- catalog/listing;
- match creation;
- public view;
- legal action tree;
- action application;
- bot turn;
- effect retrieval;
- replay export/import;
- replay reset/step.

The WASM layer must remain a thin bridge. It may adapt game-specific types into viewer-safe JSON, but it must not become the owner of rules, tactical policy, replay projection behavior, or web-specific legality shortcuts.

### Web scope

`apps/web` must add a polished `ColumnFourBoard` experience and integrate it into the existing shell:

- game picker;
- match setup;
- hotseat, human-vs-bot, and bot-vs-bot modes;
- action choice submission;
- effect log;
- replay viewer;
- dev panel;
- public/no-leak/a11y smoke tests;
- README/status docs.

`ColumnFourBoard` must be a first-class renderer, not a generic board fallback. The default public page should make the game approachable for a new visitor.

### Documentation scope

Gate 5 must create complete game docs under `games/column_four/docs/` and update repository status surfaces so the project does not keep presenting itself as only Gate 3 or Gate 4 complete after Gate 5 lands.

### CI/testing/benchmark scope

Gate 5 must add `column_four` to the existing validation surfaces:

- Rust unit/rule/property/replay/serialization/visibility/bot tests as appropriate;
- golden trace validation;
- fixture validation;
- simulation;
- rule coverage;
- benchmark smoke and threshold reporting;
- WASM smoke;
- web build;
- browser E2E smoke;
- a11y/no-leak smoke;
- docs link check;
- boundary check.

## 6. Non-goals

Gate 5 explicitly excludes the following.

Repository/process exclusions:

- cloning the repository for spec writing;
- changing repository history;
- using branch-name fetches during spec writing;
- using GitHub code search during spec writing;
- relying on default-branch metadata for this spec;
- treating this spec as verification of latest `main`.

IP/product exclusions:

- proprietary game branding;
- proprietary rulebook text;
- proprietary board art;
- proprietary token art;
- proprietary packaging style;
- proprietary trade dress;
- proprietary marketing language;
- Hasbro or Connect 4 naming in public product surfaces, except as cited source evidence in documentation if truly necessary;
- public UI/test IDs/assets/copy that imitate commercial branding or recognizable commercial trade dress.

Rules/variant exclusions:

- PopOut;
- misère variant;
- Five-in-a-Row;
- commercial variant modes;
- arbitrary m/n/k configurability;
- custom board sizes;
- alternate gravity modes;
- multiplayer variants beyond two local seats;
- rule randomness;
- hidden information;
- simultaneous turns.

Platform exclusions:

- online multiplayer;
- accounts;
- persistence backend;
- matchmaking;
- public chat;
- social features;
- hosted server-side game state;
- analytics-driven personalization;
- external service dependency for play.

Rendering exclusions:

- Canvas rendering unless profiling and an ADR justify it;
- Pixi rendering unless profiling and an ADR justify it;
- WebGL rendering unless profiling and an ADR justify it;
- asset-heavy board art that risks trade dress or public-load bloat;
- 42 independent primary click targets unless a documented accessibility/product reason justifies overriding the column-first decision model.

Architecture exclusions:

- generalized board-game engine extraction;
- generic m/n/k framework;
- `engine-core` board nouns;
- `engine-core` grid nouns;
- `engine-core` cell nouns;
- `engine-core` line/pattern/gravity nouns;
- generic line detector in `engine-core`;
- generic gravity/drop primitive in `engine-core`;
- generic board-game framework in `game-stdlib` during this gate.

TypeScript authority exclusions:

- TypeScript-owned legality;
- TypeScript-owned full-column detection;
- TypeScript-owned current turn decisions;
- TypeScript-owned winner detection;
- TypeScript-owned draw detection;
- TypeScript-owned gravity/landing logic;
- TypeScript-owned board occupancy rules;
- TypeScript-owned tactical bot policy;
- TypeScript reconstruction of hidden or private state;
- TypeScript candidate rankings or bot internals exposed to the DOM.

AI/search exclusions:

- perfect solver;
- minimax search;
- negamax search;
- alpha-beta search;
- MCTS;
- ISMCTS;
- Monte Carlo playout search;
- ML/RL;
- runtime LLM bot;
- external AI services;
- web-worker search engine;
- precomputed perfect-play tablebase.

## 7. Rules model

### Rules identity

The standard rules identity is:

- `game_id`: `column_four`
- `display_name`: `Column Four`
- `variant_id`: `column_four_standard`
- `rules_id`: `column_four-rules-v1`
- `schema_version`: current Rulepath trace/public-view schema version used by the repository
- `data_version`: `1` unless implementation conventions require a more specific value

### Board and coordinates

The game has exactly seven columns and six rows. The rules documentation must define a stable coordinate convention before implementation begins. The convention should be simple, readable, and consistent across Rust, WASM, traces, docs, and UI.

Recommended convention:

- columns: `c1` through `c7`, left to right from the current public viewer perspective;
- rows: `r1` through `r6`, bottom to top, because gravity lands pieces at the lowest available row;
- cell ids: `r1c1` through `r6c7` or a similarly explicit stable form;
- winning lines: ordered cell ids using that same convention;
- public action labels: neutral column labels such as “Column 1” through “Column 7.”

The exact convention may differ if the implementation documents it clearly, but it must be stable, deterministic, and reflected in rules docs, UI docs, traces, public view fields, and accessibility labels.

### Seats and turn order

The game has exactly two seats. `seat_0` starts unless the repository’s official-game conventions require a documented setup option. The standard variant should not include configurable starting seat behavior unless existing Rulepath conventions already require it.

After each non-terminal legal action, the active seat alternates. A terminal action that creates a win or draw ends the game immediately; no next active seat may have legal moves.

### Legal action

A legal action is choosing a non-full column while it is the actor’s turn and the match is non-terminal.

The Rust action tree should expose exactly the legal columns available to the active actor. Full columns must not appear as legal choices. The public view may expose non-legal column metadata for rendering, but that metadata must not be used by TypeScript to become the authority over legality.

Invalid action classes that must produce public diagnostics include:

- stale freshness token;
- actor is not the active seat;
- action path has wrong shape;
- unknown column id;
- choosing a full column;
- attempting any move after terminal.

### Gravity

When a legal column is chosen, Rust determines the landing row: the lowest empty row in that column. TypeScript must never calculate or infer the landing row as behavior. The public view may include a Rust-provided preview landing row for a selected/hovered/focused legal column.

### Terminal resolution

After each legal placement, Rust checks for a winning line for the placed seat. If a winning line exists, terminal outcome is win and the winning line is preserved in state/public view/effects/replay projection.

If no winning line exists and all 42 cells are occupied, terminal outcome is draw.

Win takes precedence over draw. A final placement that fills the board and creates a line is a win, not a draw.

### Winning lines

A win is four contiguous same-seat pieces in one of these directions:

- horizontal;
- vertical;
- diagonal rising;
- diagonal falling.

The implementation may detect additional longer contiguous lines, but the terminal public view must expose a clear winning line suitable for highlighting. If more than one line is created by a final move, Rust must choose a deterministic line representation and document the tie-breaking rule in `RULES.md` and `RULE-COVERAGE.md`.

### Draw

A draw is terminal only when:

- every cell is occupied;
- no winning line exists after the final placement.

The public view must distinguish draw from non-terminal full-looking intermediate states; in standard play, a full board with no line is terminal.

### Rule IDs and coverage

`games/column_four/docs/RULES.md` must assign stable rule IDs covering at least:

- game identity and variant;
- board dimensions;
- seat count and turn order;
- legal column action;
- full-column illegality;
- gravity/landing;
- occupancy mutation;
- win detection;
- draw detection;
- terminal no-actions rule;
- stale action diagnostic;
- invalid action diagnostic;
- public perfect-information view;
- replay determinism;
- bot legal-action restriction.

`RULE-COVERAGE.md` must map each rule ID to implementation, tests, traces, replay checks, simulation checks, serialization checks, bot checks, UI checks, and benchmark relevance.

## 8. Public view and action model

### Public view requirements

The Rust public view for `column_four` must include enough viewer-safe data for the web shell to render the board and controls without inferring rules.

Required public-view concepts:

- schema version;
- rules version;
- `game_id`;
- display name;
- variant id;
- rules version label;
- board rows = 6;
- board columns = 7;
- complete cell occupancy in stable coordinate order;
- active seat when non-terminal;
- ply count;
- status label;
- freshness token;
- legal column targets for the active actor when non-terminal;
- terminal kind: non-terminal, win, or draw;
- winning seat when terminal win;
- winning line cell ids when terminal win;
- draw marker when terminal draw;
- private-view status such as `not_applicable_perfect_information`;
- hidden fields array, empty for this game;
- replay step index or equivalent replay projection marker if used by existing conventions;
- optional selected/preview column information if the preview is modeled as Rust-provided state rather than action metadata.

Recommended public-view concepts for public polish:

- column summaries for each of the seven columns: column id, label, full/non-full status, legal target id when legal, and top/next landing row if Rust elects to expose preview data;
- last placement summary for effect animation anchoring;
- winning-line metadata sufficient to style four highlighted cells without TypeScript line inference;
- token presentation keys that are original and neutral, such as shape labels or style tokens, not proprietary color identities.

### Action tree requirements

The legal action tree for the active actor must expose one action choice per legal column. Each choice must carry:

- a stable action segment or path;
- a short visual label;
- an accessibility label;
- viewer-safe metadata identifying the column;
- freshness token through the action tree root or existing action-tree convention;
- optional Rust-provided preview metadata if that is the chosen route.

The action path should remain simple. A flat one-segment path is acceptable if consistent with current WASM/replay export limitations. A multi-segment path may be used only if replay export/import and the WASM bridge are extended intentionally and tested.

### Diagnostics

Diagnostics must be public, stable, and user-comprehensible. They should include a code and message, consistent with existing API error patterns.

Required diagnostic categories:

- `stale_action` or existing stale-code convention;
- `not_active_seat` or equivalent;
- `invalid_action_path` or equivalent;
- `unknown_column` or equivalent;
- `full_column` or equivalent;
- `terminal_match` or equivalent.

Public diagnostic copy should explain what happened without exposing internal state dumps. Example: “That column is full. Choose another column.” The exact wording must be original and documented.

### Effect model

Effects must be semantic, deterministic, ordered, replayable, and viewer-safe.

Required effect concepts:

- piece/drop action accepted;
- piece landed, including actor seat, column, row/cell id, and possibly from/to display anchors;
- turn advanced when non-terminal;
- win detected, including winner and line ids;
- draw detected;
- bot chose action, including public rationale;
- diagnostic events if the existing effect system records them.

The effect log should support public comprehension and animations, but the primary UX must not be raw engine internals. The board should show the action result; the effect log should explain it.

### Replay projection

Replay projection must be Rust-owned. When the replay viewer steps a `column_four` trace, the view at each cursor must be a Rust-projected public view. TypeScript may render that projection using `ColumnFourBoard` in non-interactive replay mode.

## 9. Rust/WASM boundary

### Boundary rule

Rust/WASM is the authority; TypeScript/React is presentation. Gate 5 must preserve that boundary even if a local TypeScript calculation would be easy.

Forbidden TypeScript behavior includes:

- checking whether a column is full in order to decide legality;
- computing the landing row;
- computing winning lines;
- computing draw state;
- deciding terminal status;
- choosing bot moves;
- ranking tactical threats;
- reconstructing authoritative board state from previous effects;
- modifying replay outcomes.

Allowed TypeScript behavior includes:

- mapping Rust-provided cells/columns to SVG coordinates;
- styling legal/illegal/full/terminal states based on Rust-provided fields;
- submitting the selected action segment/path;
- showing Rust-provided hover/focus preview data;
- scheduling an animation based on Rust-provided semantic effects;
- settling the visual board to the latest Rust public view;
- respecting reduced-motion settings;
- rendering Rust-provided bot explanations;
- showing public diagnostics.

### WASM catalog

The WASM catalog must include `column_four` with display name, schema/rules version, and default variant metadata. Existing games must remain listed and functional.

### WASM operations

Existing operations should be extended to cover `column_four` rather than inventing a parallel browser API:

- list games;
- feature report;
- new match;
- get view;
- get action tree;
- apply action;
- run bot turn;
- get effects;
- export replay;
- import replay;
- replay reset;
- replay step.

If any operation’s payload shape must change to support `column_four`, the change must be backward-compatible for existing games or accompanied by explicit migration and smoke coverage.

### Public-view typing

The TypeScript WASM client may add a `ColumnFourPublicView` type or equivalent. That type must describe the Rust-provided public view; it must not introduce behavior helpers that decide rules.

The TypeScript discriminant should be `game_id === "column_four"` or another Rust-provided stable discriminant. Avoid fragile shape-based inference when a clear game id is available.

### Replay import/export

Replay export/import must preserve Trace Schema v1 compatibility. Exported `column_four` replay documents must use `game_id: "column_four"`, `rules_version: "column_four-rules-v1"`, and the default variant id unless a future gate adds variants.

If the current replay export path only supports one-segment action paths, Gate 5 should keep `column_four` action paths compatible with that limitation unless there is a strong reason and full test coverage to change it.

## 10. Web UI requirements

### Public quality bar

The public default page should feel like a polished playable game, not a diagnostic shell. The user should understand:

- what game is selected;
- whose turn it is;
- which columns are legal;
- where the piece will land;
- what just happened;
- whether the game ended;
- who won or whether it drew;
- why a bot chose a move;
- how to replay or inspect without losing the play-first experience.

### Renderer choice

Use React + SVG by default for `ColumnFourBoard`.

Canvas, Pixi, or WebGL are out of scope unless all of the following are true:

- a measured rendering problem exists;
- React + SVG cannot meet the requirement with reasonable simplification;
- an ADR documents the evidence, alternatives, and boundary implications;
- accessibility and no-leak coverage remain at least as strong as the SVG approach.

### Interaction model

The primary interaction model must be seven selectable column controls, not forty-two independent cell controls.

Reason: the player’s real decision is the column. The landing row is determined by gravity and Rust. Exposing forty-two cells as primary controls would misrepresent the action model, create unnecessary focus noise, and invite TypeScript to infer legality.

The board may render forty-two cells visually. It may also render per-cell hover or animation surfaces. But the actionable controls should be seven column hit zones/buttons, one per column, with Rust-provided legal/disabled states.

Required column interaction behavior:

- each legal column is clearly selectable;
- each full column is visibly unavailable and not submitted as a legal action;
- hover over a legal column shows the Rust-provided landing preview;
- keyboard focus on a legal column shows the same Rust-provided landing preview;
- pointer/touch hit zones are forgiving and aligned with columns;
- terminal boards have no active playable column controls;
- pending actions disable duplicate submission without losing public status clarity.

### Visual board requirements

The board must show:

- seven columns and six rows;
- empty cells;
- `seat_0` pieces;
- `seat_1` pieces;
- active-turn status;
- selected/hovered/focused column preview;
- most recent placement, if useful for comprehension;
- terminal win highlight over the four winning cells;
- draw state;
- disabled/full columns;
- replay mode state when non-interactive.

Visual tokens must be original. Do not copy commercial piece colors, board color, rack shape, token shape, slot styling, packaging cues, or trade dress. Use a neutral Rulepath visual language. Color may help, but seat identity must not rely on color alone.

### Animation requirements

Gate 5 should include a semantic effect-log-driven drop animation. It must be deterministic as presentation and must settle to the Rust-projected public view.

Required behavior:

- placement effects can animate from a column entry point to the Rust-provided landing cell;
- animation source and destination are derived from Rust-provided effect/public view data, not TypeScript gravity logic;
- after animation, the visible board equals the latest Rust public view;
- replay stepping can show or summarize the same effect without changing replay outcomes;
- reduced motion replaces or shortens motion without losing information;
- animation failures must not corrupt game state or desynchronize the board.

The effect log should describe the action in public terms. It should not be the only way a user understands that a piece fell or a line won.

### Terminal states

Win state must be obvious:

- status text names the winning seat in public terms;
- exactly the Rust-provided winning line is highlighted;
- no new player action can be submitted;
- replay export remains available;
- bot/autoplay controls stop or become inert appropriately.

Draw state must be obvious:

- status text says draw;
- no win line is highlighted;
- no new player action can be submitted;
- replay export remains available.

### Bot explanation UI

When the Rust bot acts, the public UI must show a concise bot explanation derived from Rust-provided bot rationale. It should be understandable to a casual player, such as “blocked an immediate threat in Column 4” or “played the center because no immediate win or block was available.”

Do not expose candidate rankings, internal state dumps, search trees, private/debug-only scoring tables, or implementation-specific tactical internals in the public page or DOM.

### Dev/debug panels

The developer panel must remain available and useful, but secondary. It must not dominate the default public page.

The default public page must not start with raw JSON, debug `<pre>` dumps, trace payloads, or state snapshots. Development surfaces should stay collapsed or visually subordinate, consistent with the current Gate 3/4 direction.

### Responsive layout

The board must be playable at desktop and mobile viewport sizes. Mobile layout should preserve:

- seven clear column hit zones;
- readable status;
- accessible controls;
- replay/export controls without overwhelming the board;
- reduced-motion behavior;
- no horizontal scroll for normal board use unless explicitly justified.

## 11. Accessibility requirements

Accessibility is a gate requirement, not a nice-to-have.

### Keyboard support

The user must be able to play `Column Four` without a pointer.

Minimum keyboard behavior:

- Tab reaches the column-control group from setup/status areas;
- each legal column can be focused or reached through a documented group navigation pattern;
- Enter or Space activates the focused legal column;
- disabled/full/terminal columns cannot submit moves;
- focus remains visible;
- focus order is predictable;
- replay controls are keyboard reachable;
- bot/autoplay controls are keyboard reachable;
- developer panel remains keyboard reachable but secondary.

Optional enhancement:

- Left/Right arrow keys move focus among the seven columns when the column group is focused.

If custom ARIA widgets are used, the implementation must follow WAI-ARIA keyboard-interface guidance. If native buttons can satisfy the design, prefer native button semantics over custom ARIA.

### Accessible names and announcements

Every column control must have an accessible name that includes:

- column number or label;
- whether the column is legal/unavailable/full when relevant;
- whose move it is if needed for context;
- landing preview if Rust provides it and if conveying it is useful.

The board must expose a text summary for screen readers. The summary should include active seat, terminal status, and enough occupancy information for orientation without dumping unreadable internal payloads.

Status changes should be perceivable through text, not only color or motion.

### Pointer and touch targets

Column controls must satisfy modern target-size guidance. At minimum, pointer targets must meet WCAG 2.2 Success Criterion 2.5.8 Target Size (Minimum): at least 24 × 24 CSS pixels, or sufficient spacing under the criterion’s exceptions. Because this is a game board with only seven decisions, the preferred target should be larger than the minimum, especially on touch devices.

### Non-color cues

Seat identity, legal columns, focused column, preview, win line, full column, and terminal state must not rely on color alone. Use combinations of text, shape, outline, pattern, symbol, or positional cues.

### Reduced motion

Respect `prefers-reduced-motion` and the existing Rulepath reduced-motion override. Users who prefer reduced motion must still receive equivalent game information through immediate placement, short fades, text updates, or other non-vestibular presentation.

### No-leak accessibility

Accessibility labels, `data-testid` attributes, DOM text, local storage, session storage, console logs, and replay textareas must not expose hidden/private/internal state, candidate rankings, full bot scoring internals, or implementation-only vocabulary prohibited by the no-leak checklist.

## 12. Bot requirements

Gate 5 requires two bot levels for `Column Four`: Level 0 and Level 2. **Level 1 is intentionally
skipped** — ROADMAP Gate 5 asks for "baseline and preferably Level 2," and the Level 2 authored
policy subsumes Level 1's obvious tactics (win-now / block). Note that `column_four` is the first
official game to reach Level 2: `three_marks` stopped at Level 0 + Level 1, so no game has yet
filled `COMPETENT-PLAYER.md` / `BOT-STRATEGY-EVIDENCE-PACK.md`. Only the `templates/*` versions
exist as a starting point (see A-3).

### Level 0: random legal bot

The Level 0 bot must:

- choose only from the Rust legal action tree;
- never mutate state directly;
- be deterministic for the same state, legal choices, and seed;
- produce a viewer-safe explanation;
- work for non-terminal states;
- return no action or a public diagnostic when terminal;
- have bot tests and replay evidence.

This bot may reuse the generic random-legal approach if it fits the game’s action tree and seeded RNG conventions.

### Level 2: authored tactical public-demo bot

Gate 5 must include a Level 2 authored policy bot suitable for a public demo. It should feel competent enough that a casual player sees the app as real, without turning the gate into search, solver, or AI research work.

The Level 2 policy must be documented before coding in `games/column_four/docs/COMPETENT-PLAYER.md` and `games/column_four/docs/BOT-STRATEGY-EVIDENCE-PACK.md`, following the existing templates.

The policy may use bounded authored tactical heuristics such as:

- win immediately if a legal column creates a win;
- block an opponent’s immediate win;
- avoid a move that gives the opponent an immediate win;
- prefer center columns or strategically valuable columns when no urgent tactic exists;
- extend an existing line or threat;
- create a future threat;
- prefer moves that create multiple future threats;
- use deterministic tie-breaking or seeded random tie-breaking where appropriate.

The policy should be expressed as a clear priority vector. Example priority shape:

1. legal immediate win;
2. legal immediate block;
3. legal safe move that does not hand opponent an immediate win;
4. legal move creating or extending a tactical threat;
5. legal move creating multiple future threats;
6. center-preferred stable ordering;
7. deterministic or seeded tie-break.

The exact priority order may differ if the evidence pack justifies it, but it must be bounded, understandable, deterministic under seed, and tested.

### Allowed bounded evaluation

The Level 2 bot may evaluate legal successor states locally to answer tactical questions like “does this move win now?” or “does this move allow the opponent to win immediately next turn?” This is allowed because it is an authored bounded heuristic.

The bot must not become recursive search. It must not implement minimax, negamax, alpha-beta, MCTS, Monte Carlo playouts, tablebases, ML/RL, or runtime LLM calls.

### Bot explanations

Every public bot action must expose a viewer-safe rationale in traces/effects. Rationale should be concise and tied to public facts.

Good explanation examples:

- “Won by completing a diagonal line.”
- “Blocked an immediate threat in Column 5.”
- “Chose a center column because no immediate win or block was available.”
- “Avoided columns that would give the opponent an immediate win.”

Bad explanation examples:

- raw score arrays;
- private candidate rankings;
- search-tree dumps;
- internal state snapshots;
- references to proprietary strategy guides;
- unexplained policy codes.

### Bot tests

Bot tests must cover:

- Level 0 chooses only legal columns;
- Level 0 deterministic behavior under seed;
- Level 2 immediate win;
- Level 2 immediate block;
- Level 2 avoids immediate concession when a safe legal move exists;
- Level 2 center/strategic preference when no urgent tactic exists;
- Level 2 deterministic tie-breaking or seeded tie-breaking;
- Level 2 explanation payloads are present and viewer-safe;
- terminal states produce no illegal bot action;
- bot actions validate through the same Rust command validation path as human actions.

## 13. Replay/trace requirements

### Deterministic replay

`Column Four` must be fully replayable. Given the same trace document, seed, variant, rules version, seats, options, and command sequence, replay must produce the same public view hashes, effect hashes, action tree hashes where applicable, terminal outcome, and exported replay document hash behavior expected by Trace Schema v1.

Replay projection must be Rust-owned. TypeScript must not reconstruct board state from effect logs.

### Required golden traces

Gate 5 must include golden traces for at least:

1. shortest normal win;
2. vertical win;
3. horizontal win;
4. diagonal win;
5. draw;
6. invalid/stale/full-column diagnostic coverage;
7. bot action;
8. terminal replay;
9. WASM-exported trace.

The diagnostic category may be split into multiple traces if that improves clarity, but coverage must include stale action, invalid action/path or unknown column, and full-column rejection.

Suggested file names:

- `shortest-normal-win.trace.json`
- `vertical-win.trace.json`
- `horizontal-win.trace.json`
- `diagonal-win.trace.json`
- `draw.trace.json`
- `stale-diagnostic.trace.json`
- `invalid-column-diagnostic.trace.json`
- `full-column-diagnostic.trace.json`
- `bot-action.trace.json`
- `terminal-replay.trace.json`
- `wasm-exported.trace.json`

The exact filenames may follow existing repository naming conventions, but the coverage must be unmistakable in `RULE-COVERAGE.md`.

### Trace schema compatibility

All `column_four` traces must comply with `docs/TRACE-SCHEMA-v1.md`:

- valid root fields;
- no unknown fields;
- no duplicate IDs;
- no behavior-looking trace keys;
- supported schema version;
- stable command records;
- stable checkpoints;
- expected outcome and terminal state;
- explicit not-applicable markers where appropriate.

Trace documents must not become rule authorities. The rules live in Rust and docs; traces are fixtures and regression evidence.

### Replay viewer smoke

The web replay viewer smoke must include `column_four`. It must prove:

- a `column_four` replay can be exported;
- a `column_four` replay can be imported;
- replay reset renders an empty or initial `ColumnFourBoard` projection;
- replay step renders a board projection with the placed piece at the Rust-projected landing row;
- terminal replay renders win or draw state;
- replay mode is non-interactive or only exposes replay controls, not live play controls;
- no hidden/internal leak terms appear in the replay DOM or textareas.

### Effect-log comprehension

Effect logs must help a user understand what happened, but must not be the primary representation of game state. The board, status text, and controls must communicate the game state directly from the Rust public view.

## 14. Benchmark requirements

### Benchmark surfaces

`Column Four` must include benchmarks comparable to existing official games and useful for public demo confidence.

Required benchmark areas:

- setup/new match;
- legal action tree generation;
- action validation;
- action application;
- public view projection;
- random playout or simulation throughput;
- Level 0 bot decision;
- Level 2 bot decision;
- replay import/export or replay stepping if existing benchmark conventions cover it;
- WASM/browser smoke timing if current tooling supports it.

### Honest thresholds

Thresholds must be honest, measured, and documented. Do not set aspirational thresholds that immediately fail on normal development machines or shared runners.

`docs/TESTING-REPLAY-BENCHMARKING.md` anticipates early `Column Four` random-playout throughput around 100,000+ games/sec as a provisional target. Gate 5 should treat that as an expectation to measure against, not as a reason to fake a threshold. If measured results miss the provisional target, document the miss plainly in `games/column_four/docs/BENCHMARKS.md`, choose a defensible threshold if CI requires one, and create a separate cleanup note only if broader benchmark policy needs revision.

### No unrelated recalibration

Do not hide unrelated benchmark recalibration in Gate 5. Existing stale thresholds from prior games may be called out separately if current CI forces attention, but Gate 5 must not become a general benchmark-hardening gate.

### Benchmark docs

`games/column_four/docs/BENCHMARKS.md` must include:

- benchmark command names or surfaces;
- environment notes;
- measured results;
- threshold files and rationale;
- CI lane behavior;
- public latency relevance;
- known limitations;
- comparison to existing game thresholds only where useful.

## 15. Official game documentation requirements

Gate 5 must create complete official-game docs under `games/column_four/docs/`. Required docs:

- `RULES.md`
- `MECHANICS.md`
- `RULE-COVERAGE.md`
- `UI.md`
- `AI.md`
- `BENCHMARKS.md`
- `SOURCES.md`
- `GAME-IMPLEMENTATION-ADMISSION.md`
- `COMPETENT-PLAYER.md`
- `BOT-STRATEGY-EVIDENCE-PACK.md`
- `PUBLIC-RELEASE-CHECKLIST.md`, if the web shell presents `Column Four` as a public showcase game or if the repository’s release checklist conventions require it before public surfacing

Each doc must be filled out with game-specific evidence, not left as a template shell.

### `RULES.md`

Must include:

- original rules prose;
- game metadata;
- variant metadata;
- stable rule IDs;
- legal actions;
- illegal actions and diagnostics;
- gravity/landing behavior;
- win/draw/terminal behavior;
- public/private information model;
- replay notes;
- bot notes;
- out-of-scope variants;
- source/IP note.

Do not copy proprietary rulebook text. Do not use proprietary names in public-facing game title/copy.

### `MECHANICS.md`

Must include:

- game-local mechanic inventory;
- local coordinate/occupancy model;
- gravity/drop placement model;
- line-detection model;
- action shape;
- effect shape;
- UI pressure;
- bot pressure;
- repeated-shape comparison against `three_marks`;
- explicit decision not to extract to `engine-core` or `game-stdlib` in Gate 5;
- mechanic atlas updates required.

### `RULE-COVERAGE.md`

Must include a row per rule ID and columns or sections mapping each rule to:

- Rust implementation file/module;
- unit/rule tests;
- golden traces;
- replay checks;
- simulation checks;
- serialization checks;
- visibility/public-view checks;
- bot checks;
- UI smoke checks;
- benchmark relevance.

It must include a golden trace catalog and explicitly call out coverage for vertical, horizontal, diagonal, draw, stale/invalid/full-column diagnostics, bot action, terminal replay, and WASM-exported trace.

### `UI.md`

Must include:

- public product goal;
- default layout;
- board anatomy;
- column controls;
- Rust/WASM-owned view/action/preview/effect boundary;
- visual language and IP-safe asset policy;
- hover/focus/drop preview behavior;
- effect-driven animation behavior;
- terminal win/draw behavior;
- replay behavior;
- bot explanation behavior;
- accessibility behavior;
- reduced-motion behavior;
- responsive behavior;
- dev/debug secondary surface;
- no-leak expectations;
- UI smoke coverage.

### `AI.md`

Must include:

- bot registry;
- Level 0 policy;
- Level 2 policy;
- seed/determinism behavior;
- legal action authority;
- public explanation format;
- tests;
- benchmarks;
- explicit excluded AI/search approaches.

### `BENCHMARKS.md`

Must include benchmark surfaces, results, thresholds, CI lanes, environment notes, and honest caveats.

### `SOURCES.md`

Must include:

- rules source notes;
- IP/copyright/trademark/trade dress notes;
- public-domain/general method notes;
- accessibility references;
- UI/rendering references;
- bot strategy references, if any;
- confirmation that all rules prose, UI copy, colors, assets, and names are original;
- confirmation that proprietary rules text, artwork, token designs, packaging style, and trade dress were not copied.

### `GAME-IMPLEMENTATION-ADMISSION.md`

Must include the official admission checklist and mark each requirement with evidence. Admission should not be considered complete until tests, traces, docs, benchmarks, web smoke, public UI, IP review, and mechanic atlas updates pass.

### `COMPETENT-PLAYER.md` and `BOT-STRATEGY-EVIDENCE-PACK.md`

Must be completed before final Level 2 bot admission. They should describe tactical principles in original words, translate them into bounded policy priorities, define tie-breaking, define explanations, and explicitly rule out search/solver/ML/LLM approaches.

### `PUBLIC-RELEASE-CHECKLIST.md`

If `Column Four` is presented as the first “Rulepath is real” public milestone, the public release checklist is required. It must review:

- official-game contract;
- IP/trade dress;
- public UI polish;
- accessibility;
- reduced motion;
- no-leak surfaces;
- replay/export/import;
- bot explanations;
- static bundle behavior;
- dev/debug boundaries;
- docs/status surfaces.

## 16. Mechanic atlas / primitive pressure requirements

Gate 5 must update `docs/MECHANIC-ATLAS.md` and any local primitive pressure ledger required by templates.

### Local-only mechanics in `column_four`

The following must remain local to `games/column_four` during Gate 5:

- coordinate ids;
- fixed 7×6 occupancy;
- column actions;
- gravity/drop placement;
- line detection;
- terminal line highlighting metadata;
- tactical threat evaluation;
- local board/public-view encoding.

### Repeated-shape pressure to record

Record pressure for later comparison, especially:

- fixed 2D occupancy: `three_marks` first use, `column_four` second use;
- coordinate-based placement: `three_marks` cell placement, `column_four` column-to-landing placement;
- simple line detection: `three_marks` 3-in-line, `column_four` 4-in-line with horizontal/vertical/diagonal directions;
- column actions: first clear use in `column_four`;
- gravity/drop placement: first clear use in `column_four`;
- terminal line highlighting: repeated with richer line metadata;
- local tactical threat evaluation: repeated from `three_marks` bot pressure, richer in `column_four`.

### No extraction yet

Do not promote board/grid/cell/line/gravity concepts into `engine-core` during Gate 5. Do not create a generalized m/n/k helper. Do not create a generalized board-game framework.

`game-stdlib` promotion should also be deferred unless repository policy absolutely requires a small helper; the preferred Gate 5 posture is local duplication plus pressure documentation. The likely extraction decision belongs to a later gate after more games, especially after `directional_flip`, provide enough comparison evidence.

### Hard-gate rule

If an implementation agent feels forced to extract a primitive to proceed, Gate 5 must stop and produce a primitive pressure decision note/ADR rather than quietly smuggling mechanic nouns into `engine-core`.

## 17. IP and source policy requirements

### Naming and public copy

The public name is `Column Four`. The internal id is `column_four`. Public UI, docs intended for users, route labels, buttons, screenshots, test IDs, and assets must use neutral Rulepath naming.

Do not use Hasbro or Connect 4 naming in public product surfaces. Those names may appear only in source/IP documentation as cited evidence if necessary, not as product branding, copy, UI text, component names shown to users, CSS class names intended as public identity, or asset names.

### Original expression

All rules prose, UI prose, bot explanations, docs summaries, board visuals, token visuals, colors, shapes, icons, animations, and public marketing language must be original.

Do not copy:

- proprietary rulebook text;
- commercial board art;
- commercial token designs;
- commercial color/trade dress combinations;
- package styling;
- ad copy;
- screenshots;
- official diagrams;
- recognizable commercial visual layout.

### Visual identity

The board should look like Rulepath’s `Column Four`, not like a clone of a commercial product. Use a neutral, original visual language. Avoid the recognizable blue rack/red-yellow disc presentation or any close substitute that would create trade dress risk.

Seat presentation should use original shape/color combinations and non-color cues. The precise palette is a product decision, but it must be documented and reviewed for IP safety.

### Sources docs

`games/column_four/docs/SOURCES.md` must cite source categories carefully:

- general public knowledge of vertical four-in-a-row mechanics;
- copyright guidance on game methods versus expressive rule text/art;
- trademark guidance for product names/brands;
- trade dress caution around product look and feel;
- accessibility target-size and keyboard guidance;
- reduced-motion guidance;
- any bot strategy source used.

Do not quote proprietary rules text except a tiny excerpt if absolutely necessary for source evidence. Prefer paraphrase. The final `RULES.md` must be original prose.

### IP acceptance

Gate 5 is not accepted until `SOURCES.md` and the admission/public-release checklist state that:

- `Column Four` is neutral/original naming;
- no proprietary rules text was copied;
- no proprietary artwork or board/token trade dress was copied;
- external references were used for understanding and compliance, not as copied expression;
- UI/assets are original or permissively licensed with evidence;
- no commercial brand name appears in public product surfaces except as documented source evidence if necessary.

## 18. CI and smoke test requirements

Gate 5 must extend existing CI and smoke coverage without broadening into unrelated CI redesign.

### Rust and workspace checks

`column_four` must participate in workspace-level format, lint, build, and tests. Adding the crate to the workspace must not weaken existing checks.

Required Rust coverage:

- setup tests;
- legal column tests;
- full-column tests;
- gravity/landing tests;
- turn alternation tests;
- horizontal win tests;
- vertical win tests;
- diagonal win tests in both diagonal directions;
- draw tests;
- terminal no-actions tests;
- stale command tests;
- invalid action tests;
- visibility/public view tests;
- serialization/stable hash tests where consistent with current games;
- replay tests;
- bot tests;
- property/simulation tests appropriate for the game.

### Tool coverage

Add `column_four` to:

- simulation tool coverage;
- replay-check coverage;
- fixture-check coverage;
- rule-coverage tool coverage;
- benchmark-report/threshold coverage as appropriate;
- seed reducer support if existing tool patterns require game registration;
- trace viewer support if current architecture expects explicit game support.

Note (A-4): `seed-reducer` and `trace-viewer` are currently `race_to_n`-only; `three_marks` is not registered with either. By that precedent these two are optional for `column_four` — register only if tooling conventions change. `simulate`, `replay-check`, `fixture-check`, and `rule-coverage` use explicit per-game match arms and must be extended (WB-13).

### WASM smoke

WASM smoke must prove:

- catalog lists `column_four`;
- new match works;
- public view returns a 7×6 non-terminal board;
- legal action tree returns seven initial legal column choices;
- applying a column action updates Rust view and effects;
- full-column diagnostic can be surfaced through WASM;
- stale diagnostic still works;
- bot turn works and produces public explanation effect;
- export replay works;
- import replay works;
- replay step projects a `column_four` view;
- existing games still work.

### Web smoke

Browser smoke must prove:

- game picker shows `Column Four`;
- setup starts a `column_four` match;
- default page is play-first, not debug-first;
- board renders 7 columns × 6 rows;
- initial legal controls are seven column controls;
- selecting a legal column through pointer/touch-equivalent click applies Rust action;
- keyboard focus and Enter/Space can choose a column;
- hover/focus preview appears from Rust-provided data;
- full columns are not legal controls after they fill;
- win line highlights exactly the Rust-provided line;
- draw state renders clearly;
- human-vs-bot shows Rust bot rationale;
- bot-vs-bot/autoplay stops on terminal;
- replay export/import/step renders `ColumnFourBoard` projection;
- reduced-motion mode disables or replaces drop motion;
- no hidden/internal/candidate-ranking leak terms appear in DOM text, attributes, storage, console logs, or replay textareas;
- developer panel starts secondary/collapsed and remains viewer-safe.

### Docs and boundary checks

CI must continue to run docs link checks and `scripts/boundary-check.sh`. If `column_four` work triggers boundary-check failures in `engine-core`, the implementation is wrong unless an explicit foundation-approved ADR changes the rule. Gate 5 should not require such an ADR.

**Coverage caveat (A-5):** `scripts/boundary-check.sh` only scans for the mechanic-vocabulary pattern `board|card|deck|grid|suit|resource|capture|hand|pile|trick|pot|auction|betting|drafting`. Of the spatial nouns this game must keep out of `engine-core`, the script catches `board` and `grid` but **not** `line`, `cell`, `column`, `row`, `gravity`, or `pattern`. Those tokens leaking into `engine-core` would pass the script and must be caught by review against FOUNDATIONS §3. Gate 5 does not mandate extending the script's pattern; reviewers and the implementer carry the line/gravity/cell discipline directly.

## 19. Documentation/status hygiene requirements

Gate 5 must update all status surfaces that would otherwise mislead a reader after implementation.

Required review/update surfaces:

- `specs/README.md` — Gate 5 must read `Done` with a link to the archived spec after §B exit criteria pass with evidence.
- Root `README.md` — available games and public status should include `Column Four` if it becomes public-facing.
- `docs/ROADMAP.md` — Gate 5 completion/status surface should be updated if the roadmap tracks completion.
- `progress.md` — if it remains the living tracker, record Gate 5 completion evidence and do not leave it stuck at Gate 3.
- `docs/MECHANIC-ATLAS.md` — record the second-use pressure and local-only decisions.
- `docs/README.md` — update reading order/status only if needed.
- `apps/web/README.md` — update available games and smoke coverage; it currently describes a Gate 3 shell centered on `race_to_n` and should not remain stale.
- Relevant web/e2e checklists — update no-leak/a11y coverage to include `Column Four`.
- `games/column_four/docs/**` — complete all official-game docs and admission evidence.
- Root or project status docs — update any “only Gate 3/Gate 4” language discovered during implementation.

All documentation must preserve alignment with `docs/FOUNDATIONS.md`. Do not write status language that suggests TypeScript owns rules, that the engine has gained generic board primitives, or that public release uses copied commercial expression.

## 20. Acceptance criteria

Gate 5 is accepted only when all of the following are true.

### Official game admission

- `column_four` exists as an official game crate and is included in the workspace.
- The game id is `column_four`.
- The public name is `Column Four`.
- The default variant id is `column_four_standard`.
- The rules id is `column_four-rules-v1`.
- `games/column_four/docs/` contains complete, game-specific official-game docs.
- `GAME-IMPLEMENTATION-ADMISSION.md` shows passing evidence for official-game admission.
- Public release checklist is complete if the game is surfaced as a public showcase.

### Rules correctness

- The board is exactly seven columns by six rows.
- Only two seats play.
- There is no hidden information and no rule randomness.
- Turns alternate correctly.
- Legal moves are exactly the non-full columns for the active seat on non-terminal turns.
- Pieces land in the lowest empty row of the chosen column.
- Full columns are rejected and not offered as legal choices.
- Horizontal wins are detected.
- Vertical wins are detected.
- Both diagonal directions are detected.
- Draw is detected only for full board with no winning line.
- Win takes precedence over draw on the final placement.
- Terminal states expose no legal moves.
- Stale, invalid, non-active, full-column, and terminal-action diagnostics are covered.

### Rust/WASM boundary

- Rust owns legality, board state, gravity, terminal outcome, winner/draw, public view, replay projection, effects, diagnostics, and bot decisions.
- TypeScript does not compute legality, full-column state, landing rows, winner/draw, tactical bot policy, or replay state.
- WASM exposes `column_four` through the existing catalog/match/view/action/effect/bot/replay operations.
- Existing games still work through WASM.
- Public view and effect payloads are viewer-safe.

### Web UI polish

- `ColumnFourBoard` is a polished first-class public renderer.
- React + SVG is used unless a profiled reason and ADR justify otherwise.
- Seven column controls are the primary interaction model.
- The board does not use forty-two independent primary click targets without a documented product/accessibility reason.
- Hover/focus/drop preview comes from Rust-provided data.
- Drop animation is effect-log-driven and settles to Rust view.
- Reduced-motion behavior preserves information without nonessential motion.
- Win-line highlighting is clear.
- Draw state is clear.
- Terminal state is clear and inert for player moves.
- Bot explanations are visible and public-safe.
- Dev/debug panels remain available but secondary.
- The default page is not debug-first.

### Accessibility

- Column controls are keyboard accessible.
- Enter or Space can activate a focused legal column.
- Focus is visible.
- Controls have accessible names.
- Status is available as text.
- Seat identity and legal/terminal status do not rely on color alone.
- Pointer/touch targets meet at least WCAG 2.2 2.5.8 minimum sizing/spacing, with larger targets preferred.
- Reduced motion is respected.
- Accessibility labels and test IDs do not leak internal state or candidate rankings.

### Bots

- Level 0 random legal bot exists and is tested.
- Level 2 authored tactical bot exists and is tested.
- Level 2 policy is documented in `COMPETENT-PLAYER.md` and `BOT-STRATEGY-EVIDENCE-PACK.md`.
- Bots use the same Rust legal/validation path as human actions.
- Bot decisions are deterministic under state and seed where applicable.
- Bot rationales appear in public effects/traces.
- No perfect solver, minimax/negamax, alpha-beta, MCTS/ISMCTS, Monte Carlo playout search, ML/RL, runtime LLM, or external AI service is implemented.

### Replay and traces

- Trace Schema v1 compatibility is preserved.
- Golden traces exist for shortest normal win, vertical win, horizontal win, diagonal win, draw, invalid/stale/full-column diagnostics, bot action, terminal replay, and WASM-exported trace.
- Replay-check passes for `column_four`.
- WASM export/import replay round trip is covered.
- Replay viewer smoke covers `column_four`.
- Replay projection is Rust-owned.

### Benchmarks and CI

- `column_four` benchmarks exist.
- Benchmark thresholds/docs are honest and measured.
- Simulation coverage exists.
- Replay-check coverage exists.
- Fixture-check coverage exists.
- Rule-coverage coverage exists.
- WASM smoke covers `column_four`.
- Browser E2E smoke covers play, bot, win, draw, replay, keyboard, reduced motion, and no-leak requirements.
- CI continues to run boundary checks and docs link checks.
- Existing games remain green.

### Mechanic atlas and architecture

- `docs/MECHANIC-ATLAS.md` is updated.
- Fixed 2D occupancy and simple line detection are recorded as repeated pressure with `three_marks` and `column_four`.
- Gravity/drop placement, column actions, terminal line highlighting, and tactical threat evaluation are recorded.
- No board/grid/cell/line/pattern/gravity nouns are introduced into `engine-core`.
- No generic m/n/k framework is introduced.
- No generalized board-game framework is introduced.

### IP and public release

- Public name/copy/assets are neutral and original.
- No proprietary rules prose is copied.
- No proprietary board art, token art, packaging style, colors-as-trade-dress, or marketing language is copied.
- Commercial brand names do not appear in public product surfaces except as cited source evidence if documentation requires it.
- `SOURCES.md` documents source use and IP review.

### Status hygiene

- `specs/README.md`, root/project status docs, `docs/ROADMAP.md`, `progress.md` if still living, `docs/MECHANIC-ATLAS.md`, and relevant web/app README/checklist files are reviewed and updated.
- No status page remains misleadingly stuck at Gate 3 or Gate 4 after Gate 5 is complete.

## 21. Risks and explicit deferrals

### Risk: premature engine extraction

`Column Four` will make local coordinate, occupancy, and line logic feel repetitive after `three_marks`. That is expected. Gate 5 should tolerate duplication and record pressure. The risk is smuggling board/grid/cell/line/gravity concepts into `engine-core` before the foundation’s pressure threshold is met.

Mitigation: keep logic local, update the atlas, and defer primitive promotion until a later gate with more comparison evidence, especially after `directional_flip`.

### Risk: TypeScript convenience creep

Because the board is visually simple, it will be tempting to let TypeScript infer full columns, landing rows, or win lines for UI convenience.

Mitigation: treat every such inference as a boundary violation unless it is purely coordinate-to-pixel rendering of Rust-provided data. Smoke tests and review should look specifically for TypeScript-owned legality, gravity, terminal, and bot logic.

### Risk: public UI polish expands too far

Gate 5 should improve public feel, but it should not become a total web redesign.

Mitigation: focus polish on `ColumnFourBoard`, status, previews, drop animation, win/draw states, bot explanation, replay projection, and dev-panel subordination. Defer broader site design, routing, theming systems, marketing pages, accounts, and hosted multiplayer.

### Risk: bot scope creep

A known vertical four-in-a-row game can attract solver/search work. That is out of scope.

Mitigation: require a Level 2 authored heuristic bot, document the policy, and explicitly ban perfect solvers, minimax/negamax, alpha-beta, MCTS/ISMCTS, Monte Carlo playouts, ML/RL, runtime LLMs, external services, and tablebases.

### Risk: draw trace complexity

A full-board draw trace is long and easy to get wrong because accidental winning lines are common.

Mitigation: build the draw trace deliberately, validate it through Rust replay, and document it in `RULE-COVERAGE.md`. A longer trace is acceptable if it proves the rule honestly.

### Risk: benchmark target mismatch

`Column Four` has a larger state space and Level 2 tactical evaluation than prior games. Throughput may differ from provisional expectations.

Mitigation: measure first, document honestly, keep CI thresholds realistic, and avoid hiding unrelated benchmark recalibration in Gate 5.

### Risk: IP/trade dress similarity

A vertical four-in-a-row game is adjacent to well-known commercial products. The mechanics may be abstract, but names, art, rulebook prose, board presentation, token colors, packaging cues, and trade dress can create avoidable risk.

Mitigation: use `Column Four`, original copy, original visuals, non-commercial color/shape language, careful docs, and an explicit public-release IP checklist.

### Risk: animation determinism misunderstanding

Animations are inherently time-based, but Rulepath state must remain deterministic.

Mitigation: effect logs provide semantic animation cues; Rust public view remains authoritative; animation always settles to the Rust-projected view; replay state does not depend on browser timing.

### Explicit deferrals

The following are deferred beyond Gate 5:

- generic board/grid/cell/line/gravity primitives;
- arbitrary m/n/k engine;
- `game-stdlib` board-game framework;
- perfect play or solver work;
- search-based bots;
- ML/RL/LLM bots;
- online multiplayer;
- persistence backend;
- accounts;
- matchmaking;
- public chat/social features;
- Canvas/Pixi/WebGL rendering without evidence and ADR;
- commercial variants such as PopOut, misère, Five-in-a-Row, or configurable board sizes;
- full site redesign beyond the public polish needed for `Column Four`.

## 22. Evidence ledger / source references

### Repository evidence

The spec was grounded in the following repository sources. Implementers should re-read these in
the current working tree; if the tree has moved from the state described in §2, re-verify the
registry/tool/web touch-points named in §A (see A-1).

Key repository sources and what they establish:

- `docs/FOUNDATIONS.md` — Rust-first authority, public polish, no premature engine extraction, bot limits, UI boundary, IP stance.
- `docs/ROADMAP.md` — Gate 5 direction as `column_four` showcase, gravity, legal columns, previews, drop animation, line highlighting, bot explanation, replay/UI/bench smoke.
- `docs/ARCHITECTURE.md` — Rust workspace/web/WASM shape, game-owned rules, action tree, effect log, replay pipeline.
- `docs/ENGINE-GAME-DATA-BOUNDARY.md` — forbidden engine-core mechanic nouns and static-data behavior boundary.
- `docs/OFFICIAL-GAME-CONTRACT.md` — official-game admission requirements.
- `docs/AI-BOTS.md` — bot level definitions, deterministic/legal API requirements, public AI exclusions.
- `docs/UI-INTERACTION.md` — play-first UI, React + SVG default, legal-only controls, effect-driven animation, no-leak UI.
- `docs/TESTING-REPLAY-BENCHMARKING.md` — trace/replay/simulation/benchmark/CI expectations and provisional Column Four benchmark expectations.
- `docs/TRACE-SCHEMA-v1.md` — strict trace schema and forbidden behavior-like trace fields.
- `docs/MECHANIC-ATLAS.md` — primitive pressure process and current local-only mechanic inventory.
- `docs/IP-POLICY.md` and `docs/SOURCES.md` — original expression, neutral naming, source-use requirements.
- `docs/WASM-CLIENT-BOUNDARY.md` — thin WASM/client boundary and viewer-safe payload rule.
- `templates/**` — required official-game docs, AI evidence, primitive pressure, public release, and source documentation shapes.
- `archive/specs/**` — prior gate expectations and closeout style, especially Gate 4’s explicit no-extraction posture.
- `games/race_to_n/docs/**` — first official-game documentation and trace/test/benchmark patterns.
- `games/three_marks/docs/**` — first spatial game pattern, local 2D occupancy/line detection, Rust-owned UI targets, replay/view/bot patterns.
- `crates/engine-core/src/**` — generic game/action/replay contracts and lack of game-mechanic ownership.
- `crates/wasm-api/src/lib.rs` — current explicit game registry and browser API shape.
- `apps/web/src/**` and `apps/web/e2e/**` — current web shell, board renderer pattern, replay viewer, dev panel, no-leak/a11y smoke expectations.
- `.github/workflows/**` and `scripts/boundary-check.sh` — current CI and mechanic-boundary enforcement.

### External source references used for requirements context

The following external references were used as context for accessibility, IP/source policy, and general game-background requirements. They are not repository content sources and do not replace the repository evidence above.

- W3C WAI, “Understanding SC 2.5.8: Target Size (Minimum)” — target-size minimum and spacing guidance: `https://www.w3.org/WAI/WCAG22/Understanding/target-size-minimum.html`
- W3C WAI-ARIA Authoring Practices, “Developing a Keyboard Interface” — keyboard/focus principles for custom widgets: `https://www.w3.org/WAI/ARIA/apg/practices/keyboard-interface/`
- MDN Web Docs, “prefers-reduced-motion CSS media feature” — reduced-motion preference behavior and motivation: `https://developer.mozilla.org/en-US/docs/Web/CSS/Reference/At-rules/%40media/prefers-reduced-motion`
- U.S. Copyright Office, “Games” — distinction between unprotected game ideas/methods and protectable literary/pictorial expression: `https://www.copyright.gov/register/tx-games.html`
- USPTO, “What is a trademark?” — trademark as source-identifying word/phrase/symbol/design: `https://www.uspto.gov/trademarks/basics/what-trademark`
- Justia, “Trade Dress Under the Law” — trade dress as product/service look and feel that can identify source: `https://www.justia.com/intellectual-property/trademarks/trade-dress/`
- Wikipedia, “Connect Four” — background-only summary of the common 7×6 vertical four-in-a-row game form, used only to corroborate general public mechanics and not for copied expression: `https://en.wikipedia.org/wiki/Connect_Four`
- Steele and Larremore, “Misère Connect Four is Solved” — background-only evidence that solver/misère variants are substantial enough to exclude from this gate: `https://arxiv.org/html/2410.05551v1`

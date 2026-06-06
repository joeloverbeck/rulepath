# Gate 4 — Three Marks Board Smoke

Spec ID: `gate-4-three-marks-board-smoke`  
Roadmap stage: 2  
Roadmap build gate: Gate 4 (`three_marks`)  
Status: Planned  
Date: 2026-06-06  
Owner: joeloverbeck  
Target game id: `three_marks`  
Public game name: `Three Marks`  
Default variant id: `three_marks_standard`  
Rules version string: `three_marks-rules-v1`  
Authority order: see §2 Source-of-truth hierarchy below; `docs/FOUNDATIONS.md` is supreme law and supersedes this spec on any conflict.

This is a requirements/specification document, not an implementation ticket set, patch, or agent-task decomposition. A coding agent may decompose it later (see §6 Work breakdown); the document itself stays at the gate/specification level.

---

## 1. Objective

Gate 4 implements `three_marks`, a neutral, public-domain-safe, Tic-Tac-Toe-like official game whose public name is **Three Marks**. The gate proves the first intentionally pleasant board-game surface in Rulepath: a small fixed-position placement game with a polished 3×3 board UI, board-aware replay, deterministic baseline bots, and serious official-game evidence.

The success condition is not “a second game compiles.” The success condition is that Rulepath can expose a small public board game that feels playable and finished while preserving the hard architecture laws:

- Rust owns behavior: setup, state, rules, legal actions, validation, transitions, terminal detection, semantic effects, bots, replay projections, serialization/hash surfaces, and diagnostics.
- TypeScript owns presentation only: rendering, layout, interaction dispatch, local shell state, replay controls, animation scheduling, and dev affordances.
- The browser renders Rust-provided legal choices, Rust-provided views, Rust-provided semantic effects, Rust-provided bot decisions, and Rust-provided replay projections.
- `engine-core` remains noun-free with respect to board games. No board, grid, cell, coordinate, line, pattern, adjacency, row, column, diagonal, or occupancy concepts are allowed to enter `engine-core`.
- Spatial and pattern concepts remain local to `games/three_marks` for this gate. `column_four` is expected to become the second comparison point later; `directional_flip` and later games create the real extraction pressure. Gate 4 does not extract helpers.

The design stance is narrow and opinionated: implement Three Marks well, make the board pleasant, make replay board-aware, include a real deterministic Level 1 baseline bot, update the mechanic atlas, and do not generalize the engine.

---

## 2. Source-of-truth hierarchy

The implementation must follow the repository hierarchy already established by `docs/README.md`, `specs/README.md`, the archived gate specs, and the foundation documents at the target commit.

| Priority | Source | Gate 4 interpretation |
|---:|---|---|
| 1 | `docs/FOUNDATIONS.md` | Supreme law. Rust owns behavior. TypeScript is presentation-only. Public playability beats speculative generality. `engine-core` stays generic and noun-free. |
| 2 | `docs/ARCHITECTURE.md` | Dependency direction, runtime pipeline, official game layout, and Rust/WASM/browser roles. |
| 3 | `docs/ENGINE-GAME-DATA-BOUNDARY.md` | Explicit boundary for engine/game/static data. Static data may be typed content, never rule behavior. Board concepts stay in the game crate. |
| 4 | `docs/OFFICIAL-GAME-CONTRACT.md` | Official-game admission evidence, docs, tests, traces, replay, simulation, benchmarks, bots, UI metadata, and IP posture. |
| 5 | `docs/ROADMAP.md` | Gate 4 is `three_marks`; its exit clause is mandatory and mapped row-for-row in Section 19. |
| 6 | `docs/MECHANIC-ATLAS.md` | Mechanic inventory and primitive-pressure ledger. Gate 4 records first fixed 2D occupancy and line/pattern use without extraction. |
| 7 | `docs/AI-BOTS.md` | Level 0 random legal bot and Level 1 rule-informed baseline bot expectations. No public MCTS/Monte Carlo/ML/RL/runtime LLM selection. |
| 8 | `docs/UI-INTERACTION.md` | Board-first public UI, Rust-generated legal moves/previews/effects, accessibility, reduced motion, original SVG assets, dev panel secondary. |
| 9 | `docs/TESTING-REPLAY-BENCHMARKING.md` and `docs/TRACE-SCHEMA-v1.md` | Deterministic traces, replay reproducibility, hashes, diagnostics, benchmark evidence, and not-applicable rationale. |
| 10 | `docs/WASM-CLIENT-BOUNDARY.md` | Existing Gate 3 operation groups remain stable. Browser receives viewer-safe typed JSON and does not infer rules. |
| 11 | `docs/IP-POLICY.md` and `docs/SOURCES.md` | Neutral public naming, original prose/assets, source notes with dates, no copied rules prose or trade dress. |
| 12 | `specs/README.md` and archived gate specs | Gate spec format and predecessor pattern. Gate 3 is the immediate shell predecessor, but Gate 4 is not a Gate 3 hardening pass. |
| 13 | Existing `race_to_n` game module and docs | Implementation seriousness benchmark: docs, traces, tests, replay surfaces, benchmark docs, and admission evidence equivalent in spirit, adapted to a board game. |

Where this specification appears to conflict with the foundation documents, the foundation documents win. Where the implementation encounters a required contract change, it must add explicit migration notes or an ADR as required by the existing documentation discipline.

---

## 3. Scope

Gate 4 is in scope when it directly supports the first official board-placement game and its public-facing board smoke.

| Area | In scope |
|---|---|
| Game module | Add `games/three_marks` as an official game module with typed Rust state, actions, rules, effects, visibility/view projection, variants, bots, replay support, docs, tests, golden traces, and benchmarks. |
| Rules | One default variant, `three_marks_standard`, using a fixed 3×3 board. Two players alternate placing their own mark in empty cells. First completed row, column, or diagonal wins. Full board without a line is a draw. |
| Presentation | Add a board-aware Three Marks renderer to the web shell. It should be board-first, direct-manipulation, polished, accessible, and not dominated by the generic dev/action list UI. |
| Multi-game shell | Extend the existing Gate 3 shell minimally so users can choose and play `race_to_n` or `three_marks`. This is a catalog/registry extension, not a plugin system. |
| Replay | Add board-aware Three Marks replay. Replay must reconstruct Rust replay projections rather than inspecting generic JSON or guessing TypeScript diffs. |
| Bots | Add Level 0 random legal and Level 1 deterministic rule-informed baseline bot for Three Marks. |
| Tests/traces | Add evidence-heavy Rust, WASM/API, browser, replay, bot, serialization, golden trace, and benchmark coverage. |
| Docs | Add the full official game documentation set under `games/three_marks/docs/`, update mechanic atlas and source notes, and record primitive pressure without extraction. |
| IP posture | Use neutral Rulepath-owned prose and visuals. Do not copy Tic-Tac-Toe rules text, boards, images, screenshots, fonts, or commercial presentation. |

Gate 4 should be shippable as a small public game. It should not look like an internal debug toy with a board bolted on.

---

## 4. Non-goals / forbidden changes

The following changes are explicitly forbidden for Gate 4:

| Forbidden change | Requirement |
|---|---|
| Add board/grid/cell/coordinate/line/pattern nouns to `engine-core` | Forbidden. `engine-core` must remain generic. Boundary checks must make this visible. |
| Create a grid primitive in `engine-core` | Forbidden. Fixed-position mechanics belong in `games/three_marks`. |
| Extract a board/grid helper into `game-stdlib` from only Three Marks | Forbidden. One game is not extraction pressure. |
| Add a plugin system or dynamic game loading | Forbidden. Use minimal static catalog/registry support only. |
| Make TypeScript compute legality | Forbidden. TypeScript must not decide whether a cell is legal, occupied, terminal, winning, drawable, or stale. |
| Make TypeScript compute win/draw detection | Forbidden. Rust owns win, draw, terminal, and winning-line detection. |
| Make TypeScript perform bot decisions | Forbidden. Bots run in Rust and validate through normal action application. |
| Raw command editing as normal UI | Forbidden. It may remain a dev/debug affordance only if clearly secondary and viewer-safe. |
| YAML, DSL, data-driven rules, procedural static data | Forbidden. Static data may hold typed manifests/fixtures/options/presentation metadata, never rule behavior. |
| Hosted multiplayer, accounts, database, matchmaking, chat, ranked play | Forbidden. Gate 4 is local browser/static-shell play only. |
| Public MCTS/ISMCTS/Monte Carlo/ML/RL/runtime LLM move selection | Forbidden. Level 1 is deterministic rule-informed policy, not a search engine or learned player. |
| Proprietary board assets, copied rule prose, screenshots, scans, fonts without verified rights, trade-dress imitation | Forbidden. Three Marks must have original Rulepath prose and original Rulepath visual tokens. |
| Broad unrelated refactors | Forbidden. Refactor only when required for the multi-game shell or official-game evidence, and document why. |
| Modify `race_to_n` broadly | Forbidden except where absolutely required to keep the multi-game shell working and tests green. |
| Change trace/hash/replay contracts silently | Forbidden. Contract changes require migration notes and, where required by repository policy, an ADR. |
| Add movement/sliding phase | Forbidden. This is not Three Men's Morris, Nine Holes, Achi, or a movement game. |
| Add misère/wild/larger-board/configurable-size/generalized m,n,k behavior | Forbidden. Gate 4 proves one small fixed game, not a general abstraction. |

---

## 5. Deliverables

Gate 4 must deliver a new official game module and supporting shell integration equivalent in seriousness to `race_to_n`, but adapted to a board game.

### 5.1 Game module deliverables

Concrete file names may adapt to existing repository conventions, but the delivered evidence set must include the following shape or a clearly equivalent shape:

```text
games/three_marks/
  Cargo.toml
  src/
    lib.rs
    ids.rs
    state.rs
    setup.rs
    actions.rs
    rules.rs
    effects.rs
    visibility.rs
    variants.rs
    bots.rs
    replay_support.rs
    ui.rs
  data/
    manifest.toml
    variants.toml
    fixtures/
  docs/
    RULES.md
    SOURCES.md
    RULE-COVERAGE.md
    MECHANICS.md
    AI.md
    UI.md
    BENCHMARKS.md
    GAME-IMPLEMENTATION-ADMISSION.md
  benches/
    three_marks.rs
    thresholds.json
  tests/
    rule_tests.rs
    property_tests.rs
    replay_tests.rs
    serialization_tests.rs
    bot_tests.rs
    visibility_tests.rs
    golden_traces/
```

`visibility_tests.rs` may be omitted only if the existing repository convention makes it genuinely redundant for a perfect-information game. If omitted, the omission must be called out explicitly in `RULE-COVERAGE.md` or `GAME-IMPLEMENTATION-ADMISSION.md` with a not-applicable rationale. A perfect-information game still needs public-view safety tests.

Two entries in the tree above have **no `race_to_n` precedent** and are not copy-from-template: `src/ui.rs` (the `race_to_n` module exposes UI metadata through its view projection and `docs/UI.md`, not a dedicated `ui.rs`) and `tests/visibility_tests.rs` (`race_to_n` carries no such file). They are legitimate for a board game, but the implementation must derive their placement from how `race_to_n` actually exposes UI metadata and public-view coverage rather than assuming an equivalent file already exists to mirror.

### 5.2 Repository integration deliverables

| Path or area | Required outcome |
|---|---|
| `Cargo.toml` workspace | Add `games/three_marks` and its benches/tests consistently with existing workspace conventions. |
| `crates/wasm-api` | Register/bridge Three Marks through existing operation groups without creating dynamic loading or a plugin architecture. |
| `apps/web` | Add Three Marks selection, setup, board renderer, board-aware replay view, effect rendering, bot explanation affordance, and smoke coverage. |
| `docs/MECHANIC-ATLAS.md` | Record Three Marks as first official fixed 2D occupancy and simple line/pattern detection implementation; no extraction. |
| `specs/README.md` | The index Gate 4 row flips to `Planned` (with a link to this spec) now that the spec is written, and to `Done` after the exit criteria pass — per the `Not started → Planned → In progress → Done` lifecycle in `specs/README.md`. |
| `docs/SOURCES.md` or game-level source docs | Ensure source posture is discoverable from the official documentation set. |
| Native per-game CLI tools (`tools/simulate`, `tools/replay-check`, `tools/fixture-check`, `tools/rule-coverage`) | Each is currently hardcoded to `race_to_n` (const game id/paths, an `if config.game != "race_to_n"` rejection, direct `race_to_n::` calls, and a Cargo dependency on `race_to_n` only). Add a game-resolution layer so each accepts `--game three_marks`, depends on the `three_marks` crate, and resolves the `three_marks` golden-trace/data/docs paths, rules version, and replay-support entry points. This is the same multi-game extension already required for `crates/wasm-api`, applied to the native tools that `docs/OFFICIAL-GAME-CONTRACT.md` §1 and `docs/ROADMAP.md` §2 require for CLI-simulation / replay / fixture / rule-coverage evidence. `tools/trace-viewer` and `tools/seed-reducer` are out of Gate 4 scope (see §25). |
| CI (`.github/workflows/gate-1-game-smoke.yml`) | Add `three_marks` steps mirroring the existing `race_to_n` ones: `simulate --game three_marks --games 1000`, `replay-check --game three_marks --all`, `fixture-check --game three_marks`, `rule-coverage --game three_marks`, and a `three_marks` UI smoke. Do not broaden CI beyond Gate 4 needs. |
| Other workflows/scripts | Extend only as needed for tests, web smoke, and benchmark reporting. |

---

## 6. Work breakdown

This section describes requirement workstreams, not implementation tickets.

| Workstream | Required outcome | Hard constraints |
|---|---|---|
| Source and rule admission | Establish neutral variant definition, original rules prose, source notes, IP posture, and official-game admission. | No copied rules prose, no copied board presentation, no commercial trade dress. |
| Rust game module | Implement typed game-local state, actions, validation, effects, terminal detection, view projection, serialization, replay, and fixture validation. | No board nouns in `engine-core`; no TypeScript rule logic. |
| Action model and diagnostics | Provide flat targeted placement actions and safe rejection for stale, occupied, invalid, and terminal submissions. | Legal actions must be Rust-generated and stable across equivalent states. |
| Bots | Implement Level 0 random legal and Level 1 deterministic rule-informed baseline with explanations and benchmarks. | No minimax, alpha-beta, MCTS, Monte Carlo playouts, ML/RL, runtime LLM move choice, or TypeScript bots. |
| Replay and traces | Add board-aware replay projections and golden traces including win, draw, diagnostic, bot, and WASM-exported cases. | Replay authority is Rust projection, not TypeScript diffing or JSON guessing. |
| WASM/API shell | Extend static catalog/registry and existing operation groups for two games. | No plugin system, no dynamic loading, no server dependencies. |
| Web board UI | Add polished Three Marks board renderer, accessible direct play, reduced motion, effect log, and replay controls. | Dev panel secondary; generic action list hidden or debug-only for normal play. |
| Benchmarks and CI evidence | Add native benchmarks, thresholds, WASM/browser smoke, and documentation of benchmark interpretation. | `300,000+ games/sec` random playout target must remain visible unless implementation evidence formally changes threshold decision. |
| Mechanic atlas and boundary review | Update primitive-pressure ledger, admission docs, and boundary checks. | No helper extraction in Gate 4. |

---

## 7. Rules and variant definition

### 7.1 Public identity

| Item | Required value |
|---|---|
| Public name | `Three Marks` |
| Internal game id | `three_marks` |
| Default variant id | `three_marks_standard` |
| Rules version | `three_marks-rules-v1` |
| Public rules posture | Original Rulepath prose summarizing a classic public-domain-safe three-in-a-row placement game. |
| Visual posture | Original Rulepath abstract mark tokens and board styling. |

### 7.2 Required rule content

`games/three_marks/docs/RULES.md` and game-local Rust docs must cover, in original Rulepath prose:

| Rule area | Requirement |
|---|---|
| Players/seats | Exactly two seats. Seat order is deterministic and documented. The first seat places the first mark. |
| Components/marks | Each seat owns a distinct mark token. Tokens must be represented by original Rulepath SVG/presentation assets, not raw default text as the only public identity. |
| Board/cell terminology | A fixed 3×3 board with nine named cells. Cell ids must be stable and documented. Suggested ids are `r1c1` through `r3c3` unless the implementation documents an equivalent stable convention. |
| Setup | Board starts empty. Active seat starts at the documented first seat. No hidden setup state. |
| Turn structure | Active seat chooses one empty cell and places its own mark there. Turns alternate after a legal placement unless the placement ends the game. |
| Legal placement | A placement is legal only when the target cell exists, is empty, the game is non-terminal, the actor is the active seat, and the action envelope is fresh. |
| Occupied-cell illegality | Occupied cells are never legal actions in normal play. Attempted occupied-cell submissions through API/dev paths must be rejected safely with Rust diagnostics. |
| Win detection | A seat wins immediately after placing a mark that completes any row, column, or diagonal of three of that seat's marks. Rust must report the winning seat and exact three cells. |
| Draw detection | If all nine cells are occupied and no winning line exists, the game ends in a draw. Draw wording should use “draw” consistently, with “tie” allowed only as explanatory secondary wording in docs. |
| Terminal behavior | Terminal states expose no normal placement actions. Subsequent normal apply attempts are rejected safely. Replay and view projection must preserve terminal outcome. |
| Chosen variant | The only shipped variant for Gate 4 is `three_marks_standard`. |
| Excluded variants | No movement/sliding phase, no Three Men's Morris/Achi behavior, no misère variant, no wild variant, no larger board, no configurable board size, no generalized m,n,k engine. |

Game-local terms such as board, cell, row, column, diagonal, line, mark, and occupancy are permitted inside `games/three_marks`. They are prohibited from entering `engine-core`.

### 7.3 Variant and static data

`games/three_marks/data/manifest.toml` and `variants.toml` may describe typed manifest information, public names, variant ids, rules version, supported seat count, UI copy identifiers, and fixture metadata. They must not encode rule behavior.

Static data may say that the public variant is `three_marks_standard`; Rust code must own what that variant means.

---

## 8. Rust game-module requirements

Rust owns all Three Marks behavior. The game crate must be self-contained and authoritative.

### 8.1 Required Rust-owned behavior

| Behavior | Requirement |
|---|---|
| Setup | Create a deterministic initial state for `three_marks_standard`, including empty board, seat order, active seat, rules version, and freshness/hash seed surfaces consistent with repository conventions. |
| Internal state | Represent board occupancy, active seat, turn count, terminal outcome, and winning line locally inside `games/three_marks`. |
| Public view projection | Produce viewer-safe JSON/typed view data sufficient for browser board rendering, active-seat/status, mark ownership, legal target metadata, terminal outcome, draw state, and winning line. |
| Action generation | Generate one legal placement action per empty cell for the active player before terminal state. Generate no normal placement actions after terminal state. |
| Action validation | Validate actor, action path/id, cell existence, cell emptiness, active seat, non-terminal state, and freshness token in Rust. |
| Stale/freshness rejection | Reject stale action envelopes safely and deterministically with Rust diagnostics. |
| Command envelope creation | Provide repository-consistent command/action envelopes with freshness token and stable action identity. |
| Action application | Apply a legal placement through the normal Rust path, update state, emit effects, update active player or terminal outcome, and update hash/replay surfaces. |
| Win-line detection | Detect rows, columns, and diagonals after placement. Report exact cells in stable order. |
| Draw detection | Detect full-board draw after confirming no winning line. |
| Semantic effects | Emit semantic facts sufficient for UI, logs, replay, diagnostics, and bot explanation display. |
| Bot decisions | Choose Level 0 and Level 1 bot actions in Rust from Rust legal actions; validate through the same action path as human choices. |
| Replay support | Export/import viewer-safe traces and provide board-aware replay projections. |
| Serialization/hash surfaces | Preserve stable serialization, state hashes, public-view hashes, action-tree hashes, effect hashes, and replay hashes according to existing replay/testing conventions. |
| Fixture validation | Ensure fixtures and traces can be checked by existing or extended repository tools. |

### 8.2 Public view shape requirements

The public view must include enough viewer-safe data to render without TypeScript rule inference:

| View field category | Required data |
|---|---|
| Game identity | `game_id`, public name, variant id, rules version, match id/session id where existing conventions expose it. |
| Board | Stable cell ids, display positions/order, occupancy per cell, owner seat for occupied cells, and mark token metadata or presentation keys. |
| Active state | Active seat, turn number/ply count, and status text/label from Rust or Rust-provided semantic state. |
| Legal targets | The set of legal placement targets for the active viewer/state, with action ids/path segments, labels, accessibility labels, and freshness token. |
| Terminal state | Non-terminal, win, or draw. For wins, include winning seat and exact line cells. For draws, include draw outcome. |
| Diagnostics | Last or pending diagnostics where existing view conventions expose them safely. |
| Replay projection | Step index, board at step, semantic effects for step, terminal outcome if reached, and winning line/draw data. |

The TypeScript client may transform this data for display, but must not derive legality, occupancy, terminal outcome, draw, winning line, or bot choice as a rule decision.

### 8.3 Boundary rules

| Boundary | Requirement |
|---|---|
| `engine-core` | Must not receive board/grid/cell/line/pattern terminology, types, helpers, validation, or scanning logic. |
| `ai-core` | May remain generic random-legal infrastructure. Three Marks strategy lives in `games/three_marks`. |
| `game-stdlib` | Must not gain board/grid helpers from Three Marks alone. If tiny local helper functions exist, they remain game-local and documented as local-only. |
| `wasm-api` | May carry viewer-safe game-specific JSON/discriminated payloads, but must not acquire rule logic. |
| `apps/web` | May render board-specific presentation, dispatch Rust-provided action ids, and animate Rust effects. It must not validate or simulate the game. |

---

## 9. Action model

Gate 4 uses a flat targeted action tree.

| Requirement | Details |
|---|---|
| One placement action per empty cell | Before terminal state, the active player has exactly one legal action for each empty cell. |
| Stable path | Use stable path segments equivalent to `place` + cell id, for example `place/r1c1`. The exact repository representation may vary, but the action identity must be stable across equivalent states. |
| Rust labels | Action labels, short labels, accessibility labels, and target metadata must come from Rust. |
| Freshness token | Every normal placement command must include the Rust-provided freshness token or repository-equivalent freshness marker. |
| Occupied cells absent or inert | Occupied cells must not appear as legal placement actions in normal play. In the board UI they are visibly occupied and non-clickable. |
| Diagnostics | Attempted occupied-cell, invalid-cell, wrong-actor, terminal-state, and stale-token submissions through API/dev paths must be rejected by Rust with safe diagnostics. |
| Terminal action tree | Terminal states expose no normal placement actions. Replay controls are not gameplay actions. |

Required action-model tests:

| Test | Acceptance expectation |
|---|---|
| Every empty cell legal before terminal | For any non-terminal legal state, action tree targets are exactly the empty cells for the active seat. |
| Occupied cells illegal | Occupied cells are absent from normal legal actions and explicit occupied submissions are rejected. |
| Terminal actions absent | Win and draw terminal states expose no normal placement actions. |
| Stale submissions rejected | A command built from an earlier freshness token is rejected safely after the state changes. |
| Invalid cell ids rejected | Unknown or malformed cell ids are rejected safely and do not mutate state. |
| Stable action ids | Equivalent states produce identical action ids/path segments and labels for equivalent legal cells. |
| Unique action ids | No duplicate action ids exist in a legal action tree. |

---

## 10. Semantic effects

Three Marks must emit Rust semantic effects that are useful for UI, logs, replay, diagnostics, and bot explanations. Effects are semantic facts, not animation instructions. React may schedule animations from them, but Rust effects are the cause.

Minimum effect set:

| Effect | Required payload |
|---|---|
| Match started / setup complete | Required if existing effect conventions support startup effects; otherwise document as not applicable in coverage. Include game id, variant id, rules version, seats. |
| Turn started / active player changed | Active seat, turn/ply number, and any previous seat where useful. |
| Mark placed | Seat, cell id, turn/ply number, and resulting occupancy summary or hash reference. |
| Placement rejected diagnostic | Reason category such as occupied, stale, invalid cell, wrong actor, terminal state; safe human-readable label; no private internals. |
| Line completed | Winning seat and exact ordered cells in the completed line. |
| Draw reached | Draw outcome and final board/full-board indicator. |
| Game ended | Outcome type, winning seat or draw, final turn/ply count, terminal hash where conventions support it. |
| Bot chose action | Bot level/policy id, chosen cell/action id, seed/policy inputs where viewer-safe, and a concise safe explanation for Level 1. |

Effect names should follow existing repository conventions where practical, but their semantic payload must remain game-specific and viewer-safe.

---

## 11. WASM/API requirements

Gate 4 extends the Gate 3 shell/API to support at least two games: `race_to_n` and `three_marks`.

### 11.1 Operation groups

The existing operation groups remain conceptually stable:

| Operation group | Gate 4 requirement |
|---|---|
| Version/features | Report Three Marks support and any replay/board feature flags without exposing private implementation details. |
| List games/catalog | Include `race_to_n` and `three_marks` with public names, supported modes, variant ids, and safe setup metadata. |
| New match | Create a Three Marks match for `three_marks_standard` with supported local modes. |
| Get view | Return Rust-projected Three Marks public view including board, active seat, legal targets, terminal result, and winning line/draw. |
| Get action tree | Return flat Rust legal placement actions for current state. |
| Apply action | Accept Rust action envelope/freshness token and apply or reject through Rust validation. |
| Run bot turn | Run Level 0 or Level 1 bot in Rust and return resulting view/effects/diagnostics through existing conventions. |
| Get effects | Return semantic effects since last checkpoint or according to existing shell semantics. |
| Export/import replay | Export/import viewer-safe replay compatible with trace/replay schema requirements and local import size limits. |
| Replay reset/step | Reset and step replay with board-aware projections. |

### 11.2 Catalog/registry posture

| Requirement | Acceptance |
|---|---|
| Static minimal registry | The implementation may add a small static catalog/registry to choose between `race_to_n` and `three_marks`. |
| No plugin architecture | Do not design dynamic loading, plugin manifests, remote module loading, or runtime-installed games. |
| Typed game-specific JSON allowed | View payloads may be game-specific behind viewer-safe typed JSON. TypeScript should use discriminated rendering or an equivalent safe type guard. |
| No TypeScript rules | TypeScript discriminates renderer type; it does not decide legality, terminal state, or bot moves. |
| Compatibility with Race to N | Existing Race to N functionality must keep working. Any changes to `race_to_n` must be narrow and justified by multi-game shell compatibility. |

### 11.3 Setup modes

The game picker and setup UI must support choosing Three Marks and Race to N. Match setup for Three Marks should support human-vs-bot, hotseat, and bot-vs-bot where the current shell already supports those modes. If any current shell mode is not practical for Three Marks in Gate 4, the implementation must document a concrete reason and preserve Race to N behavior.

---

## 12. Web UI requirements

Gate 4 must add a board-aware renderer such as `ThreeMarksBoard` or an equivalent component structure. Normal play should be board-first.

### 12.1 Required normal-play experience

| UI element | Requirement |
|---|---|
| Polished 3×3 board | Display a clear board with nine cells, pleasant spacing, responsive layout, and tactile board-game styling. |
| Direct placement | Click/tap a legal empty cell to dispatch its Rust-provided placement action. |
| Legal target highlighting | Highlight legal empty cells based only on Rust-provided legal actions/metadata. |
| Occupied cells | Show the owning mark token and make the cell non-clickable/inert in normal play. |
| Current-player/status banner | Show active seat, mark identity, and turn status using Rust-provided view/effects. |
| Terminal banner | Show win or draw in a dedicated terminal banner. |
| Winning line highlight | Highlight exact Rust-provided winning cells/line. |
| Draw presentation | Show a clear draw state when Rust reports draw. |
| Concise effect log | Show semantic events in human-readable form without overwhelming the board. |
| Bot explanation affordance | For Level 1, expose a concise safe explanation such as why a cell was chosen. |
| Dev panel secondary | Keep dev panel and raw JSON/action tooling behind a debug affordance or visually secondary area. |
| Generic action list hidden in normal play | Normal users should not need a generic action list. It may remain in debug/dev mode. |

### 12.2 Visual direction

Three Marks should use original Rulepath-style abstract SVG. The desired feel is cozy, tactile, warm, and restrained: a small table game that belongs in the product, not a copied classroom worksheet or commercial app clone.

Player marks must be original SVG mark tokens rather than raw default text as the only presentation. They may remain recognizably “first mark” and “second mark,” but must be project-owned. Use color plus shape, not color alone.

Do not imitate commercial boards, brands, screenshots, fonts, trade dress, or source-specific presentation. The game may be Tic-Tac-Toe-like in rules but must look like Rulepath.

### 12.3 TypeScript responsibility limits

| TypeScript may | TypeScript must not |
|---|---|
| Choose `ThreeMarksBoard` based on game id/view discriminant. | Decide whether a cell is legal. |
| Render Rust-provided cells, marks, labels, and legal targets. | Decide whether a cell is occupied. |
| Dispatch Rust-provided action ids and freshness tokens. | Decide whether a row/column/diagonal is complete. |
| Schedule animation from Rust semantic effects. | Decide game outcome or draw. |
| Display Rust-provided bot explanations. | Choose bot actions. |
| Disable pointer interaction for cells not present in Rust legal targets. | Treat disabled status as rule authority independent from Rust. |

---

## 13. Replay requirements

Gate 4 must include board-aware replay. Replay UI must not degrade into generic JSON inspection.

| Replay capability | Requirement |
|---|---|
| Board reconstruction | Replay playback reconstructs the Three Marks board at each step from Rust replay projection. |
| Placement sequence | Show mark placement sequence and current step in a readable way. |
| Semantic effects | Display step effects such as mark placed, turn changed, line completed, draw reached, and game ended. |
| Winning line/draw | Show winning line highlight or draw result at the correct replay step. |
| Controls | Support reset, step forward, and any existing Gate 3 replay controls that apply. |
| Local export/import | Export and import local viewer-safe replay through the existing shell operation group. |
| Trace compatibility | Replay export must be compatible with repo trace/replay schema requirements. |
| Reduced motion | Replay animation must honor reduced-motion mode. Stepping must remain usable with animations disabled. |
| Authority | TypeScript must never use guessed state diffs as replay authority. It renders Rust replay projection. |
| JSON inspection | Generic JSON may remain as a dev panel, not as the public replay experience. |

Replay tests must prove that exported commands/options/seeds reproduce final state hash, effect hash, action-tree hash, public-view hash, outcome, terminal state, and board-aware projection.

---

## 14. Bot requirements

Gate 4 must include Level 0 and Level 1 bots for Three Marks.

### 14.1 Level 0 random legal bot

| Requirement | Details |
|---|---|
| Legal source | Chooses only among Rust legal actions. |
| Determinism | Deterministic under seed/policy inputs. |
| Validation path | Chosen action validates through normal Rust action application. |
| Diagnostics | If no legal action exists, returns safe terminal/no-action diagnostic according to existing conventions. |
| Tests | Many-seed legality, fixed-seed determinism, terminal no-action, and normal action-path validation. |
| Benchmarks | Include Level 0 decision benchmark and random playout benchmark. |

### 14.2 Level 1 rule-informed baseline bot

Level 1 is a deterministic, explainable, non-search priority policy. It is allowed because Three Marks is tiny and perfect-information, but it must be documented as a game-local baseline policy, not a generic search engine or universal bot framework.

Recommended priority order:

1. Win immediately if possible.
2. Block the opponent's immediate win.
3. Create a fork if possible.
4. Block an opponent fork if necessary.
5. Take center.
6. Take the opposite corner.
7. Take an empty corner.
8. Take an empty side.
9. Deterministic fallback among legal actions.

Level 1 must not use minimax, alpha-beta, MCTS, Monte Carlo playouts, ML, RL, runtime LLM move selection, or any generic search framework. A tiny local board inspection routine for the documented priority policy is allowed; it remains game-local.

| Required evidence | Details |
|---|---|
| Policy version | Expose/document a stable Level 1 policy id/version. |
| Tie-breaking | Deterministic tie-breaking must be documented and tested. |
| Explanation | Return viewer-safe explanation for the chosen action, such as “completed a line,” “blocked a line,” “took center,” or “chose first stable corner.” |
| Known limitations | Document in `AI.md`; if fork handling is simplified, document exact behavior and tests. |
| Legality tests | Every Level 1 choice must be a Rust legal action and validate through the normal path. |
| Determinism tests | Same state, seed/policy inputs, and variant produce same action and explanation. |
| Explanation smoke tests | Explanations exist, are safe, and do not expose private/debug internals. |
| Simulation coverage | Bot-vs-bot and human-simulation paths terminate within nine placements. |
| Latency benchmarks | Include Level 1 decision benchmark with threshold or measured baseline. |

No Level 2 evidence pack is required for Gate 4 unless the implementation deliberately exceeds the roadmap minimum. Level 3 search is not allowed for Gate 4.

---

## 15. Trace, test, and benchmark requirements

Gate 4 is evidence-heavy. The implementation is not accepted merely because the board can be clicked once.

### 15.1 Rust rule tests

Required rule tests:

| Test | Required assertion |
|---|---|
| Initial board empty | All nine cells are empty after setup. |
| Correct active player | First active seat matches documented setup. |
| Legal moves exactly empty cells | Initial state has nine legal placement actions; later non-terminal states have one per empty cell. |
| Occupied cells illegal | Occupied target is absent from legal actions and explicit submission is rejected. |
| Alternating turns | Active seat alternates after each legal non-terminal placement. |
| Row win | Completing a row ends the game with correct winner and line cells. |
| Column win | Completing a column ends the game with correct winner and line cells. |
| Diagonal win | Completing main diagonal ends the game with correct winner and line cells. |
| Anti-diagonal win | Completing anti-diagonal ends the game with correct winner and line cells. |
| Draw on full board | Full board without line ends in draw. |
| No moves after terminal state | Win/draw states expose no normal placement actions. |
| Invalid cell rejected | Unknown or malformed cell id does not mutate state and returns diagnostic. |
| Stale action rejected | Old freshness token does not mutate state and returns diagnostic. |

### 15.2 Property / invariant tests

Required property or invariant coverage:

| Invariant | Required assertion |
|---|---|
| No legal occupied target | Generated legal actions never target occupied cells. |
| Mark counts valid | Count difference between seats remains valid for turn order. |
| Active player alternates | Active seat matches mark counts until terminal. |
| Terminal matches pattern | Win/draw terminal outcome matches board pattern. |
| Single mark per cell | Board never contains more than one mark per cell. |
| Bounded termination | Simulations always terminate within nine legal placements. |
| Stable unique actions | Legal action ids are stable and unique across equivalent states. |
| Serialization round trip | State, view, replay surfaces, and relevant hashes survive serialization round trips. |

### 15.3 Golden traces

Required golden traces under `games/three_marks/tests/golden_traces/`:

| Trace | Required content |
|---|---|
| Shortest normal win trace | A legal five-ply win sequence with expected outcome, hashes, effects, and winning line. |
| Representative draw trace | A nine-ply draw with expected draw outcome and no winning line. |
| Terminal trace | Demonstrates terminal action tree has no normal placements. |
| Occupied-cell diagnostic trace | Explicit occupied-cell submission yields diagnostic and no mutation. |
| Stale-action diagnostic trace | Stale freshness token yields diagnostic and no mutation. |
| Bot-action trace | Level 0 or Level 1 bot chooses through Rust legal action path. Prefer Level 1 if stable enough. |
| WASM-exported replay trace | Export produced through WASM/API path and accepted by replay checker. |
| Not-applicable trace | Explicitly documents hidden information and stochastic game-rule events as not applicable. |

### 15.4 Replay tests

| Test | Required assertion |
|---|---|
| Reproduce final state hash | Seed/options/commands reproduce final state hash. |
| Effect hash | Replay reproduces expected effect hash. |
| Action-tree hash | Replay reproduces action-tree hash at checkpoints. |
| Public-view hash | Replay reproduces public view hash at checkpoints. |
| Outcome | Replay reaches expected win/draw outcome. |
| Terminal state | Replay terminal flag and no-action state match expected. |
| Board-aware projection | Replay projection contains board, marks, step effects, winning line/draw state. |

### 15.5 Bot tests

| Test | Required assertion |
|---|---|
| Many-seed Level 0 legality | Level 0 never chooses an illegal action across many seeds/states. |
| Fixed-seed Level 0 determinism | Same seed/view/action tree produces same output. |
| Level 1 immediate win | Level 1 takes a winning placement when available. |
| Level 1 immediate block | Level 1 blocks opponent's immediate win when no own immediate win exists. |
| Level 1 fork behavior | If fork behavior is implemented, tests cover creation and blocking; if simplified, docs and tests state exact limitation. |
| Level 1 center/corner preference | Center, opposite corner, corner, and side preferences are covered by representative states. |
| Explanation exists and is safe | Level 1 returns viewer-safe explanation without private/debug internals. |
| Normal validation path | Every bot choice validates through ordinary Rust apply logic. |

### 15.6 WASM/API smoke tests

Required smoke coverage:

| Smoke | Required assertion |
|---|---|
| Catalog | Game list includes both `race_to_n` and `three_marks`. |
| Start match | Three Marks match starts with `three_marks_standard`. |
| Get view | View contains board, active seat, legal targets, and variant identity. |
| Get legal actions | Initial action tree contains nine placement targets. |
| Apply placement | Placement mutates state and emits semantic effects. |
| Run bot turn | Rust bot chooses and applies a legal action. |
| Get effects | Effects reflect placement/turn/terminal events. |
| Export/import replay | Replay export/import succeeds and remains viewer-safe. |
| Replay reset/step | Replay reset/step returns board-aware projection. |

### 15.7 Browser UI smoke tests

Required browser smoke coverage:

| Smoke | Required assertion |
|---|---|
| Load game picker | Picker loads and shows Race to N and Three Marks. |
| Start Three Marks | Setup creates a visible Three Marks match. |
| Render board | Board renders nine cells and mark/status affordances. |
| Click/tap legal cell | A legal cell dispatches Rust action and board updates. |
| Occupied cell not clickable | Occupied cells are inert/non-clickable in normal mode. |
| Bot turn works | Human-vs-bot or bot-vs-bot path advances using Rust bot. |
| Win highlight displays | Winning line is shown from Rust-provided cells. |
| Draw displays | Draw terminal state is presented clearly. |
| Replay step board | Replay step shows board state, not generic JSON only. |
| Dev panel secondary | Dev/debug panel remains secondary and viewer-safe. |
| Reduced motion | Reduced-motion mode does not block play or replay controls. |
| Keyboard/focus path | Keyboard navigation and focus path can select legal cells. |

### 15.8 Benchmark requirements

Native Rust benchmarks are primary. WASM/browser performance smoke is required but is not the primary threshold authority.

Required benchmarks:

| Benchmark | Requirement |
|---|---|
| Legal action generation | Measure action tree generation for representative non-terminal states. |
| Action application | Measure applying legal placements. |
| View generation | Measure public view projection. |
| Replay stepping | Measure replay reset/step projection. |
| Random playout throughput | Measure complete random Three Marks games per second. |
| Level 0 bot decision | Measure seeded random legal decision latency/throughput. |
| Level 1 bot decision | Measure deterministic rule-informed decision latency/throughput. |
| Serialization/replay | Measure serialization/replay surfaces where consistent with existing tools. |
| WASM/browser smoke | Basic browser responsiveness smoke; not authoritative for native threshold. |

The intended native random playout target for Stage 2 / Three Marks is **300,000+ games/sec**, as recorded in the Stage table of `docs/TESTING-REPLAY-BENCHMARKING.md` (the source of this figure). Do not silently weaken or hide this threshold. If the first measured baseline cannot meet it on the accepted benchmark lane, the implementation must document the result in `games/three_marks/docs/BENCHMARKS.md`, preserve a visible threshold decision in `benches/thresholds.json` or equivalent, and avoid false performance claims. Any recalibration must follow the repository's benchmark/ADR discipline — `docs/adr/0001-stage-1-random-playout-budget.md` is the governing precedent: it recalibrated the `race_to_n` validated-playout floor (the same correctness-preserving harness shape, where each game runs the legal-action tree + validation + apply per ply) from a provisional target down to an accepted **100,000 games/sec** CI floor in `games/race_to_n/benches/thresholds.json`. A `three_marks` miss is therefore resolved by an analogous accepted-ADR threshold decision, never by a silent lowering. Note that `three_marks` is a far smaller game (≤9 plies), so the 300,000 target may well be reachable; the ADR precedent governs only the path if it is not.

---

## 16. Documentation requirements

Gate 4 requires the full official game documentation set for `games/three_marks/docs/`.

| Document | Required content |
|---|---|
| `RULES.md` | Original Rulepath rules prose covering players, marks, board/cell terms, setup, turn order, legal placements, occupied-cell illegality, win, draw, terminal behavior, variant, and excluded variants. |
| `SOURCES.md` | Consulted sources, consulted date, chosen variant, excluded variants, no copied prose/assets statement, public name rationale, IP posture, and remaining ambiguities. |
| `RULE-COVERAGE.md` | Matrix tying every rule requirement to tests, golden traces, replay checks, UI smoke where applicable, and not-applicable rows. |
| `MECHANICS.md` | Mechanic inventory answering repository categories, especially fixed 2D positions, occupancy, targeted placement, simple line/pattern detection, terminal win/draw, semantic effects, direct board interaction, bots, and benchmark pressure. |
| `AI.md` | Level 0 and Level 1 policy descriptions, policy versions, determinism, tie-breaking, explanations, limitations, tests, simulations, and benchmarks. |
| `UI.md` | Board UI behavior, Rust/TypeScript boundary, accessibility plan, reduced-motion plan, original SVG/token posture, replay UI, and dev panel boundaries. |
| `BENCHMARKS.md` | Benchmark lane, targets, measured results, threshold decisions, hardware/environment notes where existing conventions require them, and honest interpretation. |
| `GAME-IMPLEMENTATION-ADMISSION.md` | Admission checklist proving official-game requirements, foundation alignment, IP/source readiness, testing/traces/replay/benchmarks, UI smoke, and known deferrals. |

Docs must use original prose. Template language may guide structure, but it must not become copied external rule text or generic filler.

---

## 17. Mechanic-atlas / primitive-pressure requirements

Gate 4 must update `docs/MECHANIC-ATLAS.md` and the game-level `MECHANICS.md`.

Note: `docs/MECHANIC-ATLAS.md` already lists `three_marks` in its primitive-pressure ledger for "fixed 2D occupancy" and "simple line/pattern detection" (rows currently labeled `repeated-shape candidate after Stage 3`). The Gate 4 update therefore **edits those existing rows in place** to record that `three_marks` is now the first *implemented* use (status local-only / first-use, with the extraction decision deferred to the Stage 4 `directional_flip` review) — it does not add duplicate rows.

| Mechanic pressure item | Requirement |
|---|---|
| Fixed 2D positions | Record `three_marks` as the first implemented official game exercising a fixed 3×3 spatial arrangement. Status: local-only. |
| Occupancy | Record cell occupancy as game-local. Occupied targets are illegal. Status: local-only. |
| Targeted placement | Record direct placement into empty cells. Status: local-only unless already covered by generic action shape. |
| Simple line/pattern detection | Record row/column/diagonal detection as first use. Status: local-only. |
| Terminal win/draw | Record terminal line win and full-board draw. |
| Semantic effect shape | Record mark placement, line completion, draw, and terminal effects. |
| Direct board interaction | Record Rust legal target highlighting mapped into board UI. |
| Level 0 / Level 1 bot pressure | Record random legal and deterministic priority baseline; keep policy game-local. |
| Benchmark pressure | Record random playout target and board-game benchmark relevance. |
| Extraction decision | Explicitly state no extraction in Gate 4. `column_four` is the second comparison point; `directional_flip` and later gates create real extraction pressure. |

No helper extraction is allowed in Gate 4. In particular, do not promote board/cell/line helpers to `game-stdlib`, do not add grid/board/cell/line nouns to `engine-core`, and do not create a general m,n,k abstraction.

---

## 18. Accessibility and reduced-motion requirements

Gate 4 requires a practical accessibility baseline. It does not need to be perfect, but it must be real, tested, and not an afterthought.

| Requirement | Acceptance expectation |
|---|---|
| Pointer/touch support | Legal cells can be selected with pointer/touch. Touch targets should be large enough for mobile/tablet play. |
| Keyboard support | Legal cells can be focused and selected by keyboard. Enter/Space places a focused legal cell. |
| Arrow-key navigation | Arrow-key navigation across the 3×3 board is required where practical. A roving-focus grid may be used if tested; semantic buttons with predictable tab order are acceptable if simpler and robust. |
| Visible focus | Focus indicator is visible and not hidden by SVG styling. |
| Accessible names | Board, cells, marks, active player, legal targets, terminal outcome, and replay controls have accessible names/labels. |
| Screen-reader summary | Provide a concise board-state and legal-move summary derived from Rust-provided view/actions. |
| No color-only meaning | Mark identity, legal targets, winning line, and terminal state use shape/text/position in addition to color. |
| Contrast | Board, marks, text, focus, and state highlights have sufficient contrast. |
| Reduced motion | Honor reduced-motion preference. Disable or simplify placement/replay animations without blocking play. |
| SVG accessibility | SVG board/tokens include appropriate title/description or are wrapped in accessible controls. SVG must not be the only interaction path for keyboard users. |
| Tested path | Browser smoke must cover keyboard/focus and reduced motion. Manual checklist updates are acceptable only in addition to automated smoke where practical. |

Acceptable implementation patterns:

| Pattern | Conditions |
|---|---|
| Semantic HTML buttons over or within an SVG board | Preferred if it yields reliable focus, labels, and activation. Buttons dispatch Rust-provided actions and do not create legality. |
| ARIA grid / roving-focus implementation | Acceptable if tested and not overcomplicated. It must follow expected keyboard behavior and still consume Rust-provided labels/actions. |

Accessibility wrappers consume Rust-provided labels/actions and do not create legality. If a wrapper disables a button, it does so because Rust did not provide that cell as a legal action or because the Rust view says the state is terminal.

---

## 19. IP and source-note requirements

Three Marks must be neutral and public-domain-safe in implementation posture.

### 19.1 Required source posture

| Requirement | Acceptance |
|---|---|
| Public name rationale | `Three Marks` is documented as a neutral project-owned name for a classic three-in-a-row placement game. |
| Original rules prose | `RULES.md` uses original Rulepath wording. Do not copy prose from consulted sites. |
| Original visual assets | Board and marks are original Rulepath SVG/presentation. No copied screenshots, scans, icons, fonts, or trade dress. |
| Source notes | `SOURCES.md` records consulted source, consulted date, how it was used, no copied prose/assets, chosen variant, excluded variants, and ambiguity notes. |
| Classic rules only | Implement only the simple classic placement game described in this spec. Do not borrow presentation from a commercial app or named proprietary variant. |
| No source confusion | Public UI should say Three Marks, not present itself as affiliated with any external brand or source. |

### 19.2 External sources consulted for this specification

Consulted date for all rows: 2026-06-06. These sources informed rule summary, IP posture, strategy baseline, accessibility requirements, and SVG/keyboard/reduced-motion guidance. They are not copied into game rules or assets.

| Topic | Source | How it may be used |
|---|---|---|
| Classic Tic-Tac-Toe / Noughts and Crosses rules | `https://www.exploratorium.edu/explore/puzzles/tictactoe` | Rule confirmation only: alternating placement on a 3×3 board, three-in-a-row win, filled-board tie/draw. |
| Classic game overview and simple strategy priority | `https://en.wikipedia.org/wiki/Tic-tac-toe` | Background only for classic rule scope and deterministic priority strategy concepts; do not copy prose. |
| Game copyright boundary | `https://www.copyright.gov/register/tx-games.html` | IP posture: protect expression/assets/prose; implement neutral game mechanics with original expression. |
| ARIA grid keyboard interaction | `https://www.w3.org/WAI/ARIA/apg/patterns/grid/` | Guidance if implementing ARIA grid/roving focus. |
| Layout grid examples | `https://www.w3.org/WAI/ARIA/apg/patterns/grid/examples/layout-grids/` | Guidance for arrow-key focus patterns when using layout grids. |
| Target size | `https://www.w3.org/WAI/WCAG22/Understanding/target-size-minimum.html` | Touch target sizing baseline. |
| Animation from interactions | `https://www.w3.org/WAI/WCAG21/Understanding/animation-from-interactions` | Reduced-motion acceptance rationale. |
| CSS reduced motion preference | `https://developer.mozilla.org/en-US/docs/Web/CSS/Reference/At-rules/%40media/prefers-reduced-motion` | Implementation guidance for reduced-motion styling. |
| SVG title | `https://developer.mozilla.org/en-US/docs/Web/SVG/Reference/Element/title` | SVG accessible short-description guidance. |
| SVG desc | `https://developer.mozilla.org/en-US/docs/Web/SVG/Reference/Element/desc` | SVG accessible long-description guidance. |
| ARIA image role | `https://developer.mozilla.org/en-US/docs/Web/Accessibility/ARIA/Reference/Roles/img_role` | Guidance for grouped SVG/image semantics when appropriate. |

---

## 20. Exit criteria mapped row-for-row to ROADMAP Gate 4 exit clause

The roadmap Gate 4 exit clause is expanded here into concrete acceptance evidence. Implementation acceptance requires every row.

| ROADMAP Gate 4 exit clause | Concrete Gate 4 acceptance evidence |
|---|---|
| Occupied positions are never legal | Rust rule tests prove occupied cells are absent from action trees and rejected when submitted. Property tests prove generated legal actions never target occupied cells. Golden diagnostic trace covers occupied submission. Browser smoke proves occupied cells are inert in normal UI. WASM smoke proves API rejection path. |
| Win/draw detection is covered | Rule tests cover row, column, main diagonal, anti-diagonal, draw, and no moves after terminal. Property tests prove terminal outcome matches board pattern. Golden traces include win and draw. Replay tests reproduce winning line/draw projection. Browser smoke shows win highlight and draw presentation. |
| Random and Level 1 bots exist | Level 0 random legal bot exists, is deterministic under seed/policy inputs, chooses Rust legal actions only, and has tests/benchmarks. Level 1 rule-informed deterministic bot exists with policy version, tie-breaking, explanations, immediate win/block tests, preference tests, simulation coverage, and latency benchmark. |
| UI is pleasant and accessible where practical | Three Marks has a board-first polished 3×3 renderer, original SVG marks, legal target highlighting, terminal/draw banners, winning line highlight, concise effect log, bot explanation affordance, dev panel secondary, pointer/touch support, keyboard/focus support, accessible labels, screen-reader summary, sufficient contrast, color-plus-shape design, and reduced-motion support. Browser smoke and checklist evidence cover these paths. |
| Spatial/pattern mechanics are recorded but not extracted | `docs/MECHANIC-ATLAS.md` and `games/three_marks/docs/MECHANICS.md` record fixed 2D positions, occupancy, targeted placement, and simple line/pattern detection as local-only first-use mechanics. Boundary checks and review show no board/grid/cell/line/pattern nouns entered `engine-core` and no board helper was extracted to `game-stdlib`. The docs explicitly name `column_four` as the second comparison point and later gates as real extraction pressure. |

---

## 21. Acceptance evidence

Gate 4 acceptance requires concrete artifacts. Silent omissions are not allowed; use explicit not-applicable rows.

| Evidence category | Required artifact/evidence | Status expectation |
|---|---|---|
| Game crate | `games/three_marks` compiles and participates in workspace tests. | Required |
| Official docs | Full docs set under `games/three_marks/docs/`. | Required |
| Source/IP notes | `SOURCES.md` records sources, dates, no-copy posture, public name rationale. | Required |
| Rule coverage | `RULE-COVERAGE.md` maps rules to tests/traces. | Required |
| Mechanic inventory | `MECHANICS.md` and `docs/MECHANIC-ATLAS.md` updated. | Required |
| Rust rule tests | All tests listed in Section 15.1. | Required |
| Property tests | All invariants listed in Section 15.2. | Required |
| Golden traces | All traces listed in Section 15.3. | Required |
| Replay checks | Hash/outcome/projection checks listed in Section 15.4. | Required |
| Serialization tests | State/view/replay/action surfaces round-trip and reject unsafe unknowns according to existing conventions. | Required |
| Bot tests | Level 0 and Level 1 tests listed in Section 15.5. | Required |
| Benchmarks | Native benches and threshold docs listed in Section 15.8. | Required |
| CLI simulation | `cargo run -p simulate -- --game three_marks --games 1000` runs without crash and emits a seed/command failure report on any failure. | Required |
| CLI replay / fixture / coverage | `replay-check --game three_marks --all`, `fixture-check --game three_marks`, and `rule-coverage --game three_marks` all pass against the `three_marks` traces, fixtures, and rule-coverage matrix. | Required |
| CI gate-1 steps | `.github/workflows/gate-1-game-smoke.yml` runs the four `three_marks` CLI checks and the `three_marks` UI smoke alongside the existing `race_to_n` ones. | Required |
| WASM/API smoke | All operation-group smoke paths listed in Section 15.6. | Required |
| Browser UI smoke | Board, click/tap, bot, terminal, replay, keyboard, reduced-motion smoke listed in Section 15.7. | Required |
| Boundary check | Evidence that `engine-core` remains free of board/grid/cell/coordinate/line/pattern mechanics. | Required |
| `game-stdlib` restraint | Evidence that no board/grid helper was extracted from Three Marks alone. | Required |
| Race to N regression | Existing Race to N tests/smoke continue to pass. | Required |
| Hidden information | Documented as not applicable: Three Marks is perfect information. | Required not-applicable row |
| Private views | Documented as not applicable unless existing contracts require public/private view hash placeholders; no hidden state exists. | Required not-applicable row |
| Stochastic game-rule events | Documented as not applicable: randomness belongs to bot policy, not game rules. | Required not-applicable row |
| Movement/capture/sliding | Documented as explicitly deferred/not applicable. | Required not-applicable row |
| Hosted/network play | Documented as forbidden/not applicable. | Required not-applicable row |
| Level 2/Level 3 bots | Documented as deferred/not required. | Required not-applicable row |

---

## 22. FOUNDATIONS and boundary alignment

| Foundation / boundary principle | Gate 4 alignment requirement |
|---|---|
| Public playability beats speculative generality | Three Marks must feel like a small polished public board game. Do not spend Gate 4 budget on generalized board engines. |
| Rust owns behavior | Rules, legal actions, validation, terminal detection, effects, bots, replay, hashes, and diagnostics are Rust-owned. |
| TypeScript owns presentation only | React/SVG renders Rust-provided data and dispatches Rust action ids. No TS legality/win/draw/bot decisions. |
| Engine is noun-free | `engine-core` remains generic. Board terms stay in `games/three_marks`. |
| Static data is not behavior | `manifest.toml` and `variants.toml` describe typed content; Rust implements the rules. |
| `game-stdlib` is earned | No extraction from one board game. Local helpers stay local. |
| Official games need evidence | Full docs, source notes, rule coverage, traces, replay, tests, bots, benchmarks, and UI smoke are mandatory. |
| Replay is deterministic evidence | Replay must reproduce hashes/outcome and drive board-aware UI from Rust projection. |
| Bots are accountable | Level 0 and Level 1 bot decisions are deterministic, legal, tested, benchmarked, and documented. |
| IP posture matters | Three Marks uses neutral naming, original prose, and original visual assets. |
| Accessibility is product work | Keyboard/focus/reduced-motion/screen-reader paths are acceptance criteria, not polish optionality. |

---

## 23. Sequencing

Recommended sequencing for implementation, without decomposing into tickets:

1. Confirm Gate 3 is the immediate predecessor and keep its shell contracts stable. Do not turn Gate 4 into a Gate 3 hardening pass.
2. Create Three Marks admission docs first: source notes, rules, variant, excluded variants, mechanics, and UI/AI intent. This prevents rule drift and IP shortcuts.
3. Implement game-local Rust state/actions/rules/effects/view projection and core rule tests before web UI work.
4. Add replay support, serialization/hash surfaces, fixture validation, and golden traces while the rule surface is still small.
5. Add Level 0 and Level 1 bots with tests and benchmarks. Keep strategy local and explainable.
6. Extend WASM/API static catalog minimally for two games and prove operation smoke paths.
7. Add board-first web UI, keyboard/focus support, reduced motion, board-aware replay, and browser smoke.
8. Add native benchmarks, threshold documentation, mechanic atlas update, boundary review, and final admission evidence.
9. Only after acceptance evidence is complete, update `specs/README.md` gate status according to repository lifecycle conventions.

Any sequencing change is acceptable if it preserves evidence quality and avoids speculative generalization.

---

## 24. Assumptions

| Assumption | Requirement if contradicted by implementation evidence |
|---|---|
| The canonical public rules version string is `three_marks-rules-v1`. | If existing Rust core still requires numeric rules versions, document the mapping and expose the stable string in docs/replay/view where appropriate. |
| Suggested cell ids are `r1c1` through `r3c3`. | A different stable convention is acceptable only if documented and tested for action-id stability. |
| First seat places the first mark. | If repository seat conventions require different labels, preserve deterministic order and document public labels. |
| Three Marks is perfect information. | If any private-view machinery is required by generic contracts, use explicit empty/not-applicable private surfaces and tests. |
| Existing Gate 3 raw JSON WASM ABI remains in place. | If the ABI changes, document migration notes and add ADR if repository policy requires. |
| Race to N is the regression baseline. | Multi-game changes must keep Race to N working unless a deliberate migration is documented and justified. |
| Level 1 fork handling is feasible without search. | If exact fork handling creates disproportionate complexity, implement the strongest deterministic priority subset, document known limitations in `AI.md`, and keep immediate win/block plus center/corner preferences tested. Do not replace it with minimax/search. |

---

## 25. Explicitly deferred work

| Deferred item | Reason |
|---|---|
| Generalized m,n,k game engine | Gate 4 proves one small board game; abstraction pressure is insufficient. |
| Board/grid/cell primitives in `engine-core` | Violates foundation noun-free engine law. |
| Board/grid helper in `game-stdlib` | One game does not earn a shared helper. |
| `column_four` | Later second comparison point for spatial/pattern mechanics. |
| `directional_flip` and later directional games | Later gates create real extraction pressure. |
| Movement/sliding games | Not part of Three Marks. No Morris/Achi phase. |
| Misère/wild/larger-board/configurable variants | Out of scope and likely to obscure the board-smoke goal. |
| Search bots / minimax / alpha-beta / MCTS / Monte Carlo / ML / RL / runtime LLM play | Forbidden for Gate 4 public bot surface. |
| Level 2 authored evidence pack | Optional only if implementation exceeds Gate 4; not required. |
| Hosted multiplayer/accounts/database/matchmaking/chat/ranked play | Explicit roadmap exclusions. |
| Plugin system/dynamic loading | Gate 4 uses static catalog/registry only. |
| `three_marks` support in `tools/trace-viewer` and `tools/seed-reducer` | Deferred. Neither is part of the `docs/OFFICIAL-GAME-CONTRACT.md` §1 definition-of-done or the `docs/ROADMAP.md` §2 per-stage requirement; both are repo-wide debugging conveniences. A failing `three_marks` simulation is already reproducible from `tools/simulate`'s seed + action-cap failure report, so per-game wiring of these two tools is not Gate 4 evidence. |
| Canvas/Pixi renderer | React/SVG remains default unless future profiling and accessibility evidence justify another renderer. |
| Rich account persistence or cloud replay storage | Gate 4 uses local export/import only. |
| Private monster-game work | Out of public Rulepath gate scope. |

---

## Appendix A. Immediate predecessor alignment: Gate 3 without becoming Gate 3 hardening

Gate 3 proved the WASM/static web shell for Race to N: game picker, match setup, action tree/application, bot turns, effects, replay export/import/reset/step, dev panel, smoke checks, and Rust-owned behavior through the browser boundary.

Gate 4 should reuse that shell foundation but move the product proof forward. The correct increment is board-game quality:

| Gate 3 capability | Gate 4 extension |
|---|---|
| Single game shell | Static two-game catalog: Race to N and Three Marks. |
| Race to N renderer | Add board-aware Three Marks renderer. |
| Generic action controls | Hide/move generic action list behind dev/debug for normal Three Marks play. |
| Effect log | Make board-relevant effects readable and concise. |
| Replay controls | Add board-aware replay projection and step display. |
| Bot turn operation | Add Three Marks Level 0 and Level 1 bots with explanations. |
| UI smoke | Add board, accessibility, reduced-motion, terminal, and replay smoke. |

Do not spend Gate 4 polishing unrelated Gate 3 shell internals unless needed to support this board-game surface.

---

## Appendix B. Rule coverage matrix seed

The implementation should expand this into `games/three_marks/docs/RULE-COVERAGE.md` with exact test names and trace file names.

| Rule requirement | Rule tests | Property tests | Golden traces | Replay/UI evidence |
|---|---|---|---|---|
| Empty setup | Initial board empty | Mark counts valid | Shortest normal / draw start checkpoint | Initial view hash; board renders empty |
| Active first seat | Correct active player | Active alternates | Shortest normal start | Status banner shows active seat |
| Place in empty cell | Legal moves exactly empty | No legal occupied target | Shortest normal | Click legal cell smoke |
| Occupied illegal | Occupied rejected | No legal occupied target | Occupied diagnostic | Occupied cell inert smoke |
| Alternate turns | Alternating turns | Active alternates | Shortest normal / draw | Effect log turn change |
| Row win | Row win | Terminal matches pattern | Shortest normal win if row | Win highlight smoke |
| Column win | Column win | Terminal matches pattern | Additional terminal/win trace if not shortest | Win replay projection |
| Main diagonal win | Diagonal win | Terminal matches pattern | Golden or rule fixture | Replay projection if traced |
| Anti-diagonal win | Anti-diagonal win | Terminal matches pattern | Golden or rule fixture | Replay projection if traced |
| Draw | Full-board draw | Terminal matches pattern; bounded termination | Representative draw | Draw banner/replay |
| No terminal moves | No moves after terminal | Bounded termination | Terminal trace | Terminal action tree hash |
| Invalid cell | Invalid cell rejected | State unchanged on invalid | Diagnostic trace | API smoke diagnostic |
| Stale token | Stale action rejected | State unchanged on stale | Stale diagnostic trace | API/dev path diagnostic |
| Hidden info | Not applicable | Not applicable | Not-applicable trace | Documented perfect information |
| Stochastic rule events | Not applicable | Not applicable | Not-applicable trace | Bot randomness documented as bot policy only |

---

## Appendix C. Three Marks public rules prose requirements seed

The final `RULES.md` must be original prose. The following is a requirements seed, not mandatory copy:

- Three Marks is played by two seats on a board of nine cells arranged as three rows and three columns.
- The board starts empty.
- Seats alternate turns. On a turn, the active seat places its own mark in one empty cell.
- A cell that already contains a mark cannot be chosen.
- After each placement, Rust checks whether that seat owns all three cells in any row, column, or diagonal.
- If a line is completed, that seat wins and the match ends.
- If all nine cells are filled and no line has been completed, the match ends in a draw.
- Once the match has ended, no further placement actions are legal.

The implementation should rewrite this in polished Rulepath voice rather than copying it mechanically.

---

## Appendix D. Browser acceptance checklist seed

| Area | Required acceptance note |
|---|---|
| Visual board | Looks intentional, not like a debug grid. |
| Direct play | User can play entirely by board interaction. |
| Legal highlighting | Empty legal cells are obvious; occupied cells are not presented as playable. |
| Mark tokens | Tokens are original SVG shapes using color plus shape. |
| Status | Active player and terminal outcome are clear. |
| Bot feedback | Level 1 can explain a move safely and concisely. |
| Replay | Replay is board-aware and step-based. |
| Keyboard | User can reach and activate legal cells by keyboard. |
| Reduced motion | Animations disable/simplify without blocking use. |
| Dev panel | Useful but not dominant; viewer-safe. |
| Mobile/tablet | Cells are comfortably tappable and layout does not collapse. |

---

## Appendix E. Boundary review checklist seed

| Check | Required result |
|---|---|
| `engine-core` search/review for board nouns | No board/grid/cell/coordinate/line/pattern/occupancy primitives introduced. |
| `game-stdlib` review | No board helper extracted from Three Marks alone. |
| `wasm-api` review | API transports game-specific view/action payloads without implementing rules. |
| `apps/web` review | Board renderer maps Rust data and dispatches Rust actions; no rule inference. |
| Static data review | TOML files contain typed metadata/content only, no rule behavior. |
| Bot review | Bot choice implemented in Rust and validates through normal action path. |
| Replay review | Board projection comes from Rust replay support, not TypeScript diffs. |
| Race to N regression | Existing game behavior and smoke tests remain green. |

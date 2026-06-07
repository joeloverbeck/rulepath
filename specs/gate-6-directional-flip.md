# Gate 6 — Directional Flip Implementation Specification

## 1. Header

| Field | Value |
|---|---|
| Spec id | `GATE-6-DIRECTIONAL-FLIP` |
| Stage | Stage 4 — official game expansion / product mechanic ladder |
| Gate | Gate 6 |
| Status | Planned |
| Date | 2026-06-06 |
| Owner | Rulepath implementer / coding agent assigned later |
| Internal game id | `directional_flip` |
| Variant id | `directional_flip_standard` |
| Public display name | `Directional Flip` |
| Authority order | `docs/FOUNDATIONS.md` first; then architecture/boundary/official-game/mechanic/testing/UI/WASM/AI/IP docs; then `docs/ROADMAP.md`; then `specs/README.md` format/status rules; then archived spec patterns and templates; then external rules/strategy/accessibility/IP research; then this spec. |

This file is an implementation specification only. It must not be treated as Rust code, TypeScript code, a patch, a ticket set, a zip archive, a roadmap edit, or a game implementation.

**Canonical-section map** (this spec keeps the canonical `specs/README.md` section set, with extra technical detail in the higher-numbered sections): Header §1, Objective §3, Scope §5, Deliverables §8, Work breakdown §9, Exit criteria §14, Acceptance evidence §15, FOUNDATIONS & boundary alignment §16, Forbidden changes §17, Documentation updates §18, Sequencing §19, Assumptions §20. Sections §6, §7, §10, §11, §12, §13, and §21 carry the rules/effects/primitive-pressure/bot/WASM/tooling/rule-id detail that grounds decomposition.

---

## 2. Repository status facts

The following repository facts ground Gate 6 planning. Each is verified against the cited file in the current `main`.

| Fact | Source |
|---|---|
| `specs/README.md` is the living implementation-spec progress index. | `specs/README.md` |
| Specs are subordinate to the foundation set. | `specs/README.md`, `docs/FOUNDATIONS.md` |
| Gates 0–5 are marked done in the spec progress index. | `specs/README.md` |
| Gate 6 is the lowest non-Done gate and is `directional_flip`. | `specs/README.md` |
| Gate 6 is not started in the spec progress index. | `specs/README.md` |
| `docs/ROADMAP.md` gives Gate 6 the public role: directional scanning and grouped effects; extraction decision. | `docs/ROADMAP.md` |
| Gate 6’s mechanic pressure includes directional scans, bracketed grouped changes, pass/no-move if scoped, and multi-piece effects. | `docs/ROADMAP.md` |
| Gate 6 must make the third-use coordinate/scan extraction decision. | `docs/ROADMAP.md`, `docs/MECHANIC-ATLAS.md` |
| `docs/FOUNDATIONS.md` is the repository constitution and wins over specs. | `docs/FOUNDATIONS.md` |
| Rust owns setup, legal action generation, validation, state transitions, scoring, terminal detection, RNG, semantic effects, views, replay, serialization, and bots. | `docs/FOUNDATIONS.md` |
| TypeScript owns presentation only. | `docs/FOUNDATIONS.md`, `docs/UI-INTERACTION.md`, `docs/WASM-CLIENT-BOUNDARY.md` |
| `engine-core` must remain mechanic-noun-free. Forbidden vocabulary includes board, grid, cell, coordinate, adjacency, line, capture, flip, and similar nouns. | `docs/FOUNDATIONS.md` |
| `game-stdlib` may receive only narrow typed helpers after mechanic pressure is proven through the mechanic atlas / primitive-pressure process. | `docs/FOUNDATIONS.md`, `docs/MECHANIC-ATLAS.md` |
| Official games require source notes, original rules prose, rule coverage, mechanics inventory, primitive-pressure review, Rust rule coverage, golden traces, replay/hash determinism, serialization behavior, simulation, benchmarks, bots, UI metadata, web smoke once exposed, and public-release/IP evidence. | `docs/OFFICIAL-GAME-CONTRACT.md` |
| Current game patterns are `race_to_n`, `three_marks`, and `column_four`. | `Cargo.toml`, `games/**`, `crates/wasm-api/src/lib.rs` |
| `wasm-api` currently registers supported games explicitly and must be extended as thin glue only. | `crates/wasm-api/src/lib.rs`, `docs/WASM-CLIENT-BOUNDARY.md` |
| `game-stdlib` is currently a small placeholder crate, so any Gate 6 helper extraction would be meaningful new surface area. | `crates/game-stdlib/src/lib.rs` |
| Existing tools accept a known set of game ids and must be extended deliberately. | `tools/**` |
| Column Four is the closest public-polish precedent for web UI, WASM bridge, bot evidence, golden traces, release checklist, and e2e smoke style. | `games/column_four/**`, `apps/web/e2e/column-four.smoke.mjs`, `archive/specs/gate-5-column-four-public-polish.md` |

---

## 3. Objective

Implement **Directional Flip** as the Gate 6 official game for Rulepath: a neutral, original, 8×8 directional-flipping game in the Othello/Reversi-family mechanic tradition, with Rust-owned legal placement, forced-pass handling, exact flip previews, grouped flip semantic effects, deterministic replay/hash behavior, serialization, simulations, baseline benchmarks, Level 0 and Level 2-lite bots, tool integration, WASM/web integration, polished accessible public UI, IP-safe public presentation, and a resolved primitive-pressure decision for rectangular coordinates / direction deltas / ray walking.

The objective is sourced from `docs/ROADMAP.md`: Gate 6 exists to prove directional scanning and grouped effects, and to force the extraction decision for repeated coordinate/scan helpers after `three_marks` and `column_four`.

The gate must **not** become a generic tabletop engine project. It must implement one official game and resolve the narrow helper pressure created by that game. The game is official only when the Rulepath official-game contract evidence passes; a browser component that renders an 8×8 layout is not enough.

---

## 4. External research findings that shape this spec

All game prose in the repository must be original Rulepath prose. The following research is used only to shape requirements and source notes.

| Topic | Finding | Source |
|---|---|---|
| Core rules | A standard modern 8×8 directional-flipping game starts with four center discs, the first color moves first, a placement must bracket opponent discs in a direct line, all bracketed discs in qualifying lines flip, a player with no legal placement forfeits that turn, a player with any legal placement cannot forfeit, and the game ends when neither player can move; the winner has the higher disc count. | World Othello Federation official rules: <https://www.worldothello.org/about/about-othello/othello-rules/official-rules/english> |
| IP / naming caution | Othello-related sources display trademark/copyright notices involving Othello Co. and MegaHouse, WOF routes Othello(TM) selling/license inquiries to MegaHouse, and a public trademark record lists OTHELLO as registered and renewed for parlor-game equipment by Kabushiki Kaisha MegaHouse. | WOF Othello information: <https://www.worldothello.org/about/wof-council-committees/partners-sponsors/othello-information>; WOF official rules footer: <https://www.worldothello.org/about/about-othello/othello-rules/official-rules/english>; Justia trademark record: <https://trademarks.justia.com/730/61/othello-73061971.html> |
| Strategy: greed is bad | Beginner greed that maximizes immediate disc count is a known strategic trap; mobility, stable discs, and position quality matter more until the late game. | Othello Belgium strategy: <https://en.othellobelgium.be/leer-othello/tips-en-strategie>; Nederlandse Othello Vereniging strategy guide: <https://www.othello.nl/content/guides/comteguide/strategy.html> |
| Strategy: corners and stability | Corner discs are stable anchors; stable edge/corner extensions are valuable. | Othello Belgium strategy; Nederlandse Othello Vereniging strategy guide |
| Strategy: X/C danger | Squares adjacent to open corners, especially diagonal X-squares and edge-adjacent C-squares, are dangerous unless tactically justified. | Othello Belgium strategy |
| Strategy: mobility | A strong beginner-to-competent policy should consider own mobility and opponent mobility; reducing opponent choices is central. | Othello Belgium strategy; Nederlandse Othello Vereniging strategy guide |
| Strategy: frontier discs | Frontier discs are discs adjacent to empty spaces; creating large frontier exposure can reduce future mobility. Quiet moves that do not expand frontier can be useful. | Othello Belgium strategy; Nederlandse Othello Vereniging strategy guide |
| Accessibility: grid navigation | WAI-ARIA grid guidance treats a grid as a composite widget with author-managed focus, directional navigation, Home/End, and a single focusable element in the page tab sequence. | WAI-ARIA APG grid pattern: <https://www.w3.org/WAI/ARIA/apg/patterns/grid/> |
| Accessibility: color | WCAG 2.2 says color must not be the only visual means of conveying information, indicating action, prompting response, or distinguishing a visual element. | WCAG 2.2 Use of Color: <https://www.w3.org/TR/WCAG22/#use-of-color> |
| Accessibility: motion | WCAG 2.2 describes disabling motion animation triggered by interaction unless essential. | WCAG 2.2 Animation from Interactions: <https://www.w3.org/TR/WCAG22/#animation-from-interactions> |
| Implementation background | External Reversi/Othello engine material often uses bitboards and directional move generation; this confirms directional scanning is a real pressure point, but Rulepath’s foundation documents override any pressure to introduce opaque bitboard-first or generic engine abstractions. | ReversiWorld bitboard move generation: <https://reversiworld.wordpress.com/2013/11/05/generating-moves-using-bitboard/> |

The Cornell Othello AI page listed in the prompt was attempted but was not relied on because the page fetch failed in this session. The Level 2-lite policy below is grounded in Othello Belgium and Nederlandse Othello Vereniging strategy sources, plus Rulepath’s `docs/AI-BOTS.md` restrictions.

---

## 5. Scope

### 5.1 In scope

| Area | Required scope |
|---|---|
| Game identity | Add official game `directional_flip` with public display name `Directional Flip`. |
| Variant | Add exactly one fixed variant, `directional_flip_standard`. |
| Board shape | 8×8 square grid represented in the game crate with typed Rust state. This wording is allowed in the game crate and game docs, not in `engine-core`. |
| Starting position | Four center discs in the standard diagonal arrangement: first seat at `r4c5` and `r5c4`; second seat at `r4c4` and `r5c5`, assuming row 1 is top and column 1 is left. If repository conventions use another coordinate orientation, update this assumption in one line and preserve the same diagonal relationship. |
| Turn order | First seat acts first. |
| Legal placement | A placement is legal only if it brackets one or more contiguous opposing discs between the placed disc and an existing own disc along at least one of the eight directions. |
| Flip resolution | Applying a legal placement flips all bracketed opposing discs in every qualifying direction. |
| Forced pass | A player with no legal placement and a nonterminal state must take an explicit Rust-generated forced-pass action. |
| Pass prohibition | Pass must not appear when the active player has any legal placement. |
| Terminal condition | Terminal after both seats have proven no legal placement through a forced-pass sequence, or immediately after a placement fills the board and no future placement exists. The trace suite must include double-pass terminal. |
| Winner | Higher final disc count wins; equal final disc count is a draw. |
| Action tree | Prefer flat action tree: `place/<cell-id>` and `pass/forced`, with Rust-owned labels, accessibility labels, metadata, and previews. |
| Previews | Rust supplies exact placement previews, including target cell, ordered flip set, optional direction grouping, and viewer-safe explanation text. |
| Effects | Rust emits grouped semantic flip effects with stable order. |
| Replay/hash | Replay reconstructs placement, flips, pass actions, terminal state, effects, views, and stable hashes deterministically. |
| Serialization | Strict serialization/unknown-field behavior consistent with current game patterns and `docs/TRACE-SCHEMA-v1.md`. |
| Bots | Level 0 random legal bot and Level 2-lite authored policy bot. |
| UI | Public polished React + SVG board by default, equal to Column Four polish, with accessible keyboard grid behavior, visible legal cells, Rust-driven previews/effects, reduced-motion fallback, and no TypeScript legality. |
| WASM | Add game to existing public WASM operations with thin glue only. |
| Tools | Extend simulation, replay, fixture, rule coverage, benchmark report, seed reducer, and trace viewer tools. |
| Benchmarks | Add honest baseline-first benchmarks and report; no fake throughput claims. |
| Primitive pressure | Resolve rectangular coordinate / direction / ray-walk helper pressure before Gate 6 exit. |
| Public exposure | Add to public game picker/catalog only after the public release checklist passes. |

### 5.2 Out of scope

| Area | Out-of-scope decision |
|---|---|
| Optional variants | No alternate board sizes, alternate openings, handicap rules, clock/tournament rules, scored-empty-square variants, or custom setup variants. |
| Network features | No accounts, hosted multiplayer, matchmaking, ranked play, database, chat, server persistence, or cloud replay store. |
| Advanced AI | No minimax, alpha-beta, recursive search, MCTS, ISMCTS, Monte Carlo playouts, ML/RL, or LLM runtime move selection. |
| Generic engine redesign | No broad tabletop engine framework, no generic board-game kernel, no engine-core mechanic nouns. |
| Static data DSL | No rule behavior in TOML/JSON or any YAML/DSL-like mechanism. |
| Roadmap/archive edits | Do not edit `docs/ROADMAP.md`; do not archive this spec during implementation. |
| Brand replication | No Othello-branded name, trade dress, visual palette, icons, diagrams, screenshots, scans, copied rules prose, or proprietary assets. |
| Performance boasts | No speculative `30,000+ games/sec` threshold unless measured CI evidence supports it. |

### 5.3 Not allowed

The forbidden-change list in Section 15 is binding. If a later task would violate it, stop and ask for a new spec or ADR direction.

---

## 6. Rules model for `directional_flip_standard`

The implementation must write rules in original Rulepath prose in `games/directional_flip/docs/RULES.md`. The following model is normative for the code.

### 6.1 Entities

| Entity | Requirement |
|---|---|
| Seats | Two seats only: first seat and second seat. Use repository seat conventions, not color names as rule authority. |
| Disc ownership | Each occupied cell is owned by exactly one seat. Empty cells have no owner. |
| Cell ids | Stable IDs should be `r1c1` through `r8c8` unless existing repository style clearly demands another stable scheme. |
| Directions | Eight directions in stable order: north, northeast, east, southeast, south, southwest, west, northwest. Direction deltas may exist in `game-stdlib` only if the primitive-pressure decision promotes them. |
| Counts | Public view and terminal scoring may expose disc counts for each seat. |
| Freshness | Actions must carry or validate freshness using current engine-core patterns. |

### 6.2 Setup

- The board has 64 cells.
- Initial occupied cells:
  - first seat: `r4c5`, `r5c4`;
  - second seat: `r4c4`, `r5c5`.
- All other cells are empty.
- Active seat is the first seat.
- Consecutive forced-pass count starts at zero.
- Terminal outcome is absent.
- Initial legal placements for the first seat must be exactly the four standard opening placements under the chosen coordinate orientation. With the default orientation above, they are `r3c4`, `r4c3`, `r5c6`, and `r6c5`.

### 6.3 Placement legality

A placement candidate is legal only when all of the following are true:

1. The match is not terminal.
2. The actor is the active seat.
3. The command freshness token is current.
4. The target cell id parses as an in-bounds cell.
5. The target cell is empty.
6. At least one direction from the target cell has:
   - one or more contiguous opposing discs immediately after the target cell; and
   - an own disc beyond that contiguous opposing run before leaving the board or hitting an empty cell.
7. The placement is represented as a path in the current Rust-generated action tree.

A placement candidate is illegal when it is occupied, out of bounds, non-flipping, stale, from the wrong actor, terminal, malformed, or absent from the current action tree. Diagnostics must distinguish these cases enough for tests and trace evidence.

### 6.4 Flip resolution

Applying a legal placement must:

1. Place an owned disc at the target cell.
2. Identify every qualifying direction.
3. Flip every bracketed opposing disc in every qualifying direction.
4. Not skip over own discs.
5. Not flip discs that only appear indirectly enclosed, non-contiguously enclosed, or not in a direct line from the placed disc.
6. Preserve deterministic flip ordering:
   - direction order: north, northeast, east, southeast, south, southwest, west, northwest;
   - within each direction: nearest to farthest from the placed disc.
7. Produce semantic effects that encode the same ordered flip set that was previewed before commit.
8. Increment or update the freshness token.
9. Reset consecutive forced-pass count to zero.
10. Advance active seat unless terminal.
11. Terminalize if no legal placement can ever follow because the board is full or if the terminal condition has been proven by forced-pass sequence.

### 6.5 Forced pass

Forced pass is explicit and Rust-generated.

| Situation | Required legal action tree |
|---|---|
| Active seat has at least one legal placement. | Contains placement choices only; no pass choice. |
| Active seat has no legal placement, the match is nonterminal, and the previous action was not a forced pass by the other seat. | Contains exactly one legal forced-pass choice: `pass/forced` or repository-equivalent stable segment. |
| Active seat has no legal placement, the match is nonterminal, and the previous action was a forced pass by the other seat. | Contains exactly one legal forced-pass choice. Applying it emits pass and terminal effects and ends the game. |
| Match is terminal. | Contains no placement choices and no pass choices, except for any existing repository convention that represents terminal action trees as empty with status metadata. |

TypeScript must not decide pass availability. Bots and humans must choose forced pass through the same legal action API.

### 6.6 Terminal and scoring

The game ends when both seats have no legal placement, proven by a forced-pass sequence, or after a placement leaves no possible continuation and the terminal check is explicit in Rust. Final disc counts determine outcome:

- first seat wins if first-seat count is greater;
- second seat wins if second-seat count is greater;
- equal counts are a draw.

A terminal state must have no legal placement choices and no forced-pass choices. Terminal views and traces must include final counts and outcome.

---

## 7. Rust-owned action tree, previews, views, and effects

### 7.1 Action tree

Use a flat action tree unless exact current code constraints make a different stable path shape unavoidable.

| Action kind | Preferred segment | Required metadata |
|---|---|---|
| Placement | `place/<cell-id>` | `action_kind`, `cell_id`, `row`, `column`, stable display label, accessibility label, preview id or embedded preview, viewer-safe explanation. |
| Forced pass | `pass/forced` | `action_kind`, stable display label, accessibility label, viewer-safe explanation, reason code indicating no legal placements. |

Placement choices must be sorted deterministically, preferably row-major by cell id. Forced pass must be the only legal choice when present.

No UI code may synthesize a legal action path that Rust did not expose.

### 7.2 Placement previews

Every legal placement target must include a Rust-generated preview. The preview must include:

- action segment or complete action path;
- target cell id;
- row and column;
- accessible label suitable for screen readers;
- ordered list of cells that would flip;
- optional direction grouping, preserving the same direction order used by effects;
- if a confirmation UX is added, whether confirmation is required;
- viewer-safe explanation text, for example “Places at r3c4 and flips r4c4 southward” in original Rulepath style;
- a stable preview id if the view/effect bridge needs cross-reference.

The preview flip set must equal the apply flip set exactly. This must be asserted by rule/property tests and by a golden trace.

### 7.3 Semantic effects

State diffs are dev diagnostics only. Normal UI animation authority comes from Rust semantic effects.

Required effect family, with repository-conformant exact names allowed:

| Effect | Requirement |
|---|---|
| `PlacementAccepted` or equivalent | Records accepted actor and target. |
| `DiscPlaced` | Records target cell, owner, and display anchors. |
| `DiscsFlipped` | One grouped semantic effect containing ordered child flip entries. Each child must include cell id, previous owner, new owner, direction, and distance or order index. |
| `PassTaken` / `TurnPassed` | Records actor, reason, and pass sequence position. |
| `ActivePlayerChanged` | Records previous and next active seat when nonterminal. |
| `GameEnded` | Records outcome, counts, reason, and terminal step. |
| `BotChoseAction` | Records bot level/id, action path, safe rationale, and seed/tie-break summary where current patterns allow. |

If `ForcedPassAvailable` exists as a view/status/event helper, it must not substitute for the explicit `pass/forced` command.

### 7.4 Views

Public views must be perfect-information but still no-leak safe. They may expose:

- board cells and current owners;
- active seat;
- legal targets and Rust preview metadata;
- counts;
- terminal outcome;
- last action/effects summary;
- bot rationale after bot actions;
- UI metadata such as token shapes, labels, and legal target labels.

They must not expose private engine internals, raw RNG state, hidden debug structures, stale-token internals beyond safe diagnostics, or behavior-like rule data. “Perfect information” does not excuse dumping internal state.

---

## 8. Concrete deliverables

Exact file names may be adjusted only where current repository conventions require a different name. Equivalent names must preserve the same responsibilities and coverage. Silent shrinkage is forbidden.

### 8.1 Game crate

```text
games/directional_flip/Cargo.toml
games/directional_flip/src/lib.rs
games/directional_flip/src/ids.rs
games/directional_flip/src/state.rs
games/directional_flip/src/setup.rs
games/directional_flip/src/actions.rs
games/directional_flip/src/rules.rs
games/directional_flip/src/effects.rs
games/directional_flip/src/visibility.rs
games/directional_flip/src/variants.rs
games/directional_flip/src/bots.rs
games/directional_flip/src/replay_support.rs
games/directional_flip/src/ui.rs
games/directional_flip/data/manifest.toml
games/directional_flip/data/variants.toml
games/directional_flip/data/fixtures/directional_flip_standard.fixture.json
```

Responsibilities:

| File | Required responsibility |
|---|---|
| `ids.rs` | Stable seat/cell/action identifiers, parsing/formatting, display labels; no generic engine changes. |
| `state.rs` | Typed board occupancy, active seat, freshness, pass sequence, terminal outcome, stable summaries. |
| `setup.rs` | Standard setup constructor and fixture loading. |
| `actions.rs` | Action tree generation, flat path segments, legal target metadata, forced-pass action generation, previews. |
| `rules.rs` | Validation, apply, directional scan/flip collection, terminal detection, scoring. |
| `effects.rs` | Semantic effect types, grouped flip children, deterministic ordering, display anchors. |
| `visibility.rs` | Public view projection, no-leak handling, viewer-safe diagnostics. |
| `variants.rs` | Single variant id and non-behavioral metadata. |
| `bots.rs` | Level 0 random legal bot and Level 2-lite authored policy through normal legal action path. |
| `replay_support.rs` | Replay command/effect/view serialization, stable hashes, import/export support. |
| `ui.rs` | Rust-owned UI labels, token metadata, accessibility labels, preview copy. |
| `data/*.toml` | Variant/manifest metadata only; no behavior, selectors, conditions, triggers, loops, or rule DSL. |

### 8.2 Game docs

```text
games/directional_flip/docs/SOURCES.md
games/directional_flip/docs/RULES.md
games/directional_flip/docs/RULE-COVERAGE.md
games/directional_flip/docs/MECHANICS.md
games/directional_flip/docs/GAME-IMPLEMENTATION-ADMISSION.md
games/directional_flip/docs/COMPETENT-PLAYER.md
games/directional_flip/docs/BOT-STRATEGY-EVIDENCE-PACK.md
games/directional_flip/docs/AI.md
games/directional_flip/docs/UI.md
games/directional_flip/docs/BENCHMARKS.md
games/directional_flip/docs/PUBLIC-RELEASE-CHECKLIST.md
```

Required doc content:

| Document | Required content |
|---|---|
| `SOURCES.md` | Consulted sources, source-to-rule-id xref, ambiguity decisions, IP/trademark rationale, neutral naming, asset/font status, explicit statement that no Othello-branded prose/visuals/trade dress is copied. |
| `RULES.md` | Original Rulepath rules prose for `directional_flip_standard`, including setup, legal placement, all-direction flips, forced pass, terminal, scoring, draw. |
| `RULE-COVERAGE.md` | Matrix of every rule id to unit/property/golden/replay/simulation/UI evidence. No silent gaps. |
| `MECHANICS.md` | Mechanic inventory and comparison against `three_marks` and `column_four`; repeated-shape pressure entries. |
| `GAME-IMPLEMENTATION-ADMISSION.md` | Contract checklist that admits the game as official only after all evidence passes. |
| `COMPETENT-PLAYER.md` | Human-understandable strategic principles for Level 2-lite: corners, X/C danger, mobility, stability, frontier, phase-aware disc-count tie-break. Must exist before Level 2 code. |
| `BOT-STRATEGY-EVIDENCE-PACK.md` | Sources, positions, expected policy priorities, anti-patterns, deterministic tie-break policy, “no search” boundaries. Must exist before Level 2 code. |
| `AI.md` | Bot levels, rationale shape, legal-API validation, deterministic seed/tie-break behavior, exclusions. |
| `UI.md` | Public visual language, accessibility behavior, keyboard mapping, preview/effect responsibilities, reduced motion, no-leak notes. |
| `BENCHMARKS.md` | Baseline benchmark report, environment, commands, measured values, threshold posture, no fake performance claims. |
| `PUBLIC-RELEASE-CHECKLIST.md` | Release checklist passed before game picker exposure. |

### 8.3 Primitive-pressure ledger / atlas

Required either as an existing repository-conformant location or a new narrowly documented location:

```text
games/directional_flip/docs/PRIMITIVE-PRESSURE-LEDGER.md
```

If the repository convention strongly prefers a central ledger, use that convention, but `games/directional_flip/docs/MECHANICS.md` must link to the final ledger entry.

Required atlas update:

```text
docs/MECHANIC-ATLAS.md
```

### 8.4 Optional `game-stdlib` helper extraction

If the ledger decides promotion is required, require narrow helper files such as:

```text
crates/game-stdlib/src/lib.rs
crates/game-stdlib/src/grid.rs
crates/game-stdlib/src/direction.rs
crates/game-stdlib/src/ray.rs
crates/game-stdlib/tests/rect_grid.rs
```

Names are illustrative. Follow repository style. Helper scope must stay behavior-free and cannot include flip/capture/win/action/bot/UI policy.

### 8.5 Golden traces

Require a trace set covering at least:

```text
games/directional_flip/tests/golden_traces/opening-legal-move.trace.json
games/directional_flip/tests/golden_traces/multi-direction-flip.trace.json
games/directional_flip/tests/golden_traces/corner-capture.trace.json
games/directional_flip/tests/golden_traces/forced-pass.trace.json
games/directional_flip/tests/golden_traces/double-pass-terminal.trace.json
games/directional_flip/tests/golden_traces/full-board-terminal.trace.json
games/directional_flip/tests/golden_traces/draw.trace.json
games/directional_flip/tests/golden_traces/invalid-occupied-cell.trace.json
games/directional_flip/tests/golden_traces/invalid-non-flipping-placement.trace.json
games/directional_flip/tests/golden_traces/stale-diagnostic.trace.json
games/directional_flip/tests/golden_traces/non-active-seat-diagnostic.trace.json
games/directional_flip/tests/golden_traces/bot-action.trace.json
games/directional_flip/tests/golden_traces/wasm-exported.trace.json
games/directional_flip/tests/golden_traces/preview-flip-set.trace.json
```

Names may change, but coverage must not silently shrink. Every trace must follow `docs/TRACE-SCHEMA-v1.md` strictness: no unknown fields, no behavior-looking keys, stable IDs, expected diagnostics/outcomes/checkpoints/hashes where applicable.

### 8.6 Tests

Require tests at least for:

```text
games/directional_flip/tests/rules.rs
games/directional_flip/tests/property.rs
games/directional_flip/tests/replay.rs
games/directional_flip/tests/serialization.rs
games/directional_flip/tests/visibility.rs
games/directional_flip/tests/bots.rs
```

The `column_four` precedent folds serialization / unknown-field coverage into `tests/replay.rs` and `tests/rules.rs` rather than a standalone `tests/serialization.rs` (only `three_marks` uses a separate `serialization_tests.rs`). Either layout is acceptable as long as the serialization/unknown-field coverage below is not silently dropped; follow whichever the implementer adopts.

Required test coverage:

- standard setup;
- first legal moves;
- legal placement requires at least one bracketed line;
- all bracketed discs flip in every qualifying direction;
- multi-direction flip;
- no skipped own discs;
- no indirect/non-line flips;
- occupied cells rejected;
- non-flipping placements rejected;
- stale tokens rejected;
- wrong actor rejected;
- terminal state has no placement choices;
- forced pass only when no placement exists;
- pass forbidden when placement exists;
- double-pass terminal;
- full-board terminal;
- count-based winner;
- draw;
- exact preview flip set equals apply flip set;
- deterministic effect order;
- stable action segments;
- action tree contains only legal choices;
- public view contains no hidden state;
- random bot validates through normal command path;
- Level 2-lite validates through normal command path;
- Level 2-lite is deterministic;
- Level 2-lite emits safe explanations;
- serialization rejects unknown fields and behavior-looking data;
- replay hash determinism across export/import/step/reset.

### 8.7 Benchmarks

```text
games/directional_flip/benches/directional_flip.rs
games/directional_flip/benches/thresholds.json
games/directional_flip/docs/BENCHMARKS.md
```

Required benchmark coverage:

- setup;
- legality generation;
- placement validation;
- flip scanning;
- apply legal placement;
- forced-pass handling;
- projection/view generation;
- action tree generation with previews;
- semantic effect encoding;
- replay export/import;
- replay step/reset if benchmarkable;
- serialization round trip;
- random bot move;
- Level 2-lite bot move;
- full random simulation throughput;
- full Level 2-lite-vs-random or Level 2-lite-vs-Level 2-lite smoke throughput if not too costly.

Thresholds must be honest baseline-first. For the first Gate 6 implementation, prefer a non-blocking baseline report unless existing CI conventions require a threshold file. If `thresholds.json` is blocking, derive conservative values from measured CI behavior and document the evidence in `BENCHMARKS.md`.

### 8.8 Web app

Require updates or equivalent additions to:

```text
apps/web/src/components/DirectionalFlipBoard.tsx
apps/web/src/components/effectFeedback.ts
apps/web/src/wasm/client.ts
apps/web/src/main.tsx
apps/web/src/state/shellReducer.ts
apps/web/e2e/directional-flip.smoke.mjs
apps/web/e2e/a11y-noleak.smoke.mjs
apps/web/e2e/NO-LEAK-A11Y-CHECKLIST.md
```

Likely touched supporting files:

```text
apps/web/src/components/AppShell.tsx
apps/web/src/components/GamePicker.tsx
apps/web/src/components/ReplayViewer.tsx
apps/web/src/styles.css
apps/web/package.json
```

The UI must follow current app shell patterns and remain presentation-only.

### 8.9 Workspace / registry / tooling

Require updates to:

```text
Cargo.toml
crates/wasm-api/Cargo.toml
crates/wasm-api/src/lib.rs
tools/simulate/**
tools/replay-check/**
tools/fixture-check/**
tools/rule-coverage/**
tools/bench-report/**
tools/seed-reducer/**
tools/trace-viewer/**
.github/workflows/**
```

Required tool changes:

- add `directional_flip` to accepted `--game` values;
- update help text;
- add tests/smoke checks for new game paths;
- ensure machine-readable failure output where relevant;
- ensure seed reducer can normalize directional-flip simulation reports when enough context exists;
- ensure trace viewer can display grouped flip effects and pass actions.

---

## 9. Work breakdown

These are bounded implementation work items suitable for later `AGENT-TASK.md` decomposition. They are not tickets and do not authorize broadening the gate.

| Order | Work item | Depends on | Required output |
|---:|---|---|---|
| 1 | G6-00 intake against current `main` | None | Implementer reads the foundation set and confirms every referenced path/symbol against the current `main` working tree (existing game crates, `wasm-api` registration, tools, web shell) before changing code. |
| 2 | G6-01 source notes, neutral naming, original rules docs | G6-00 | `SOURCES.md`, `RULES.md`, initial rule ids, IP-safe naming rationale. |
| 3 | G6-02 mechanic inventory and primitive-pressure comparison | G6-01 | `MECHANICS.md` comparing `three_marks`, `column_four`, and planned `directional_flip`; draft ledger. |
| 4 | G6-03 primitive-pressure decision | G6-02 | Promote/defer/reject/ADR decision before deep game implementation proceeds. |
| 5 | G6-04 optional narrow `game-stdlib` extraction | G6-03 only if promote | Typed helper API, tests, docs/examples/anti-examples, benchmarks, back-port plan, atlas update. If rejected, explicit rationale and next review trigger. |
| 6 | G6-05 game crate skeleton and static data | G6-03 or G6-04 | `games/directional_flip` crate, manifest, variant data, workspace registration. |
| 7 | G6-06 rules, actions, previews, effects, views | G6-05 | Rust-owned setup, legality, forced pass, apply, grouped effects, public view projection, diagnostics. |
| 8 | G6-07 rule/property/visibility/serialization tests | G6-06 | Test files covering required rule matrix and no-leak behavior. |
| 9 | G6-08 replay support and golden traces | G6-06, G6-07 | Replay export/import/step/reset support, stable hashes, required golden traces. |
| 10 | G6-09 simulation and seed reducer support | G6-08 | Many random legal games, terminal reports, average length, throughput, failure normalization. |
| 11 | G6-10 Level 0 and Level 2-lite bots | G6-01, G6-06, G6-07 | Random legal bot; authored one-ply policy; `COMPETENT-PLAYER.md`, evidence pack, `AI.md`, deterministic rationale tests. |
| 12 | G6-11 benchmarks and benchmark docs | G6-06 through G6-10 | `benches/directional_flip.rs`, `thresholds.json` if required, `BENCHMARKS.md` baseline report. |
| 13 | G6-12 tools integration | G6-08 through G6-11 | Updated simulate/replay-check/fixture-check/rule-coverage/bench-report/seed-reducer/trace-viewer. |
| 14 | G6-13 WASM API integration | G6-06 through G6-10 | Catalog/list, new match, view, action tree, apply, bot turn, effects, replay export/import, replay step/reset, JSON mappings, TS types. |
| 15 | G6-14 polished web UI and accessibility | G6-13 | `DirectionalFlipBoard.tsx`, effect feedback mapping, keyboard grid, previews, pass control, reduced motion, e2e smoke, no-leak/a11y checklist. |
| 16 | G6-15 public release checklist and game picker exposure | G6-01 through G6-14 | Completed `PUBLIC-RELEASE-CHECKLIST.md`; only then expose in picker/catalog. |
| 17 | G6-16 final validation and status instruction | G6-15 | Full test/tool/benchmark/smoke evidence; implementation may then update `specs/README.md` status according to repo workflow. |

Stop after any work item that uncovers a foundation conflict, unresolved primitive-pressure decision, IP concern, replay/hash migration need, or need for architecture-changing helper extraction.

---

## 10. Primitive-pressure decision requirements

This is the architectural center of Gate 6.

### 10.1 Required comparison

Before Gate 6 exit, and preferably before heavy rule implementation, compare:

| Game | Existing or planned pressure |
|---|---|
| `three_marks` | Rectangular cell vocabulary; static winning lines; stable cell ids; placement occupancy. |
| `column_four` | Rectangular grid coordinates; row/column ids; directional line scanning; local direction/offset helpers; gravity-dependent placement. |
| `directional_flip` | Rectangular 8×8 coordinates; eight-direction ray scanning; bracketed directional changes; ordered grouped effects; previews derived from ray scans. |

The comparison must answer whether the repeated helper shape is narrow, behavior-free, and typed enough to live in `game-stdlib`.

### 10.2 Decision options

The primitive-pressure ledger must choose exactly one of:

| Decision | Requirement |
|---|---|
| Promote | Add narrow typed spatial helpers to `game-stdlib`; test, document, benchmark, and back-port where natural. |
| Defer/reject | Keep local duplication and explain why it is safer; state the next review trigger. |
| Escalate to ADR | Required if the helper would affect architecture, replay/hash, data policy, visibility, kernel vocabulary, or broad public API. |

No Gate 6 exit while this decision is missing.

### 10.3 Likely extraction boundary if promoted

Promote only behavior-free utilities such as:

- bounded rectangular coordinates;
- row/column indexing;
- dimension-checked coordinate construction;
- deterministic row-major iteration;
- cardinal/diagonal/eight-direction deltas;
- deterministic ray stepping / ray iteration within bounds;
- stable coordinate formatting/parsing helpers, if compatible with existing game ids.

Do **not** promote:

- flip logic;
- capture logic;
- win-condition logic;
- occupancy policy;
- legal action generation;
- forced-pass logic;
- bot strategy;
- UI presentation;
- semantic effect names;
- static-data behavior;
- generic “board game engine” abstractions;
- any vocabulary to `engine-core`.

### 10.4 If promotion happens

Required evidence:

- `game-stdlib` unit tests;
- property tests for bounds, direction order, ray termination, row-major iteration, parsing/formatting;
- examples and anti-examples;
- benchmarks before/after or at least helper microbenchmarks;
- docs for helper scope and non-goals;
- `docs/MECHANIC-ATLAS.md` update;
- primitive-pressure ledger complete;
- back-port plan for `column_four`, and possibly `three_marks`, only where the helper fits without contortion;
- trace preservation notes or intentional trace update notes;
- no `engine-core` vocabulary change.

### 10.5 If promotion is rejected or deferred

The ledger must state:

- exactly which helper shape was considered;
- why local duplication is safer now;
- what evidence was missing;
- what future gate or repeated shape reopens the decision;
- why replay/hash/visibility/data/kernel boundaries remain safer without extraction;
- why `engine-core` remains untouched.

---

## 11. Bot specification

### 11.1 Required bots

| Bot | Required behavior |
|---|---|
| Level 0 random legal | Uses `ai-core` random legal selection pattern, consumes Rust action tree, validates through normal command path, deterministic by seed. |
| Level 2-lite authored policy | Uses visible deterministic facts only; consumes legal placements and forced pass from Rust; validates through normal command path; emits safe rationale; deterministic by seed/tie-break. |

No Level 2-lite code may be committed before `COMPETENT-PLAYER.md` and `BOT-STRATEGY-EVIDENCE-PACK.md` exist and cite strategy evidence.

### 11.2 Level 2-lite policy boundary

Allowed:

- bounded one-ply evaluation over current legal moves;
- inspect the immediate successor position after a candidate move;
- count immediate legal moves for each seat in that successor;
- compute deterministic features from visible state: corner, X-square/C-square exposure, own/opponent mobility, approximate stable edge/corner extension, frontier count, immediate terminal outcome, disc count;
- use phase-aware rules, for example early/mid/late based on empty-cell count;
- deterministic seeded tie-break among equivalent candidates.

Forbidden:

- minimax;
- alpha-beta;
- recursive search;
- search opponent replies beyond one immediate successor feature pass;
- MCTS;
- ISMCTS;
- Monte Carlo playouts;
- ML/RL;
- LLM runtime move selection;
- hidden-state peeking;
- TypeScript bot logic;
- rule-DSL tactical conditions in static data.

### 11.3 Required lexicographic policy outline

The exact scoring/ranking must be documented and tested. A recommended order:

1. If forced pass is the only legal action, take it.
2. Prefer a move that immediately ends the game with a favorable final count.
3. Prefer corners.
4. Avoid X-squares and dangerous C-squares adjacent to open corners unless forced or tactically justified by immediate corner capture / terminal outcome.
5. Prefer moves that reduce opponent mobility after the move.
6. Prefer moves that preserve or improve own mobility.
7. Prefer stable edge/corner extensions over unstable frontier expansion.
8. Avoid high flip count in the opening when it creates frontier exposure or gives the opponent mobility.
9. Use disc count / immediate flips as a late-game or final tie-break, not as opening greed.
10. Use deterministic seeded tie-break among equivalent candidates.

Rationale text must be safe for all viewers and must not include debug-only internals.

---

## 12. WASM and web shell requirements

### 12.1 WASM API

Integrate through the same public surfaces as prior games:

- catalog/list games;
- new match;
- get view;
- get legal action tree;
- apply action;
- run bot turn;
- get effects;
- export replay;
- import replay;
- replay step;
- replay reset;
- effect JSON mapping;
- public view JSON mapping;
- bot rationale mapping;
- client TypeScript types;
- renderer switch in web app.

`wasm-api` must stay thin. It may route, serialize, deserialize, and map Rust-owned data to JSON. It must not contain game rule logic, pass logic, flip logic, legal target computation, bot policy, or preview computation.

### 12.2 TypeScript client

Update `apps/web/src/wasm/client.ts` or equivalent to include a `directional_flip` public view type and effect metadata without making TypeScript authoritative for legality.

Allowed:

- type definitions;
- rendering data shape;
- call wrappers;
- action path submission from Rust-provided choices;
- reduced-motion preference transport;
- effect presentation mapping.

Forbidden:

- computing legal placements;
- computing pass availability;
- computing flip consequences;
- inferring animation targets from state diffs in normal mode;
- synthesizing action paths not present in Rust action tree.

### 12.3 Public UI

The public UI must be play-first, not debug-first, and equal in polish to Column Four.

Required:

- React + SVG board by default;
- warm, tactile, original visual presentation;
- no Othello trade dress, copied diagrams, copied color layout, or brand mimicry;
- distinguish seats by color plus shape/pattern, not color alone;
- legal placement cells visibly marked;
- exact flip preview on focus and hover from Rust preview payloads;
- selected/just-placed disc highlight from Rust effects;
- flipped-disc highlights from grouped Rust flip effects;
- grouped flip animation with reduced-motion fallback;
- forced pass reachable as a normal button/control when Rust exposes it;
- illegal cells absent, inert, or visually unavailable;
- no illegal hit targets;
- no raw command editing;
- responsive layout;
- visible focus indicator;
- screen-reader labels from Rust metadata where practical;
- action controls and board controls remain usable by keyboard and pointer.

Keyboard requirements:

| Key | Required behavior |
|---|---|
| Arrow keys | Move focus between cells in a grid-like pattern. |
| Home / End | Move to start/end of current row, or repository-documented equivalent. |
| Ctrl+Home / Ctrl+End | Prefer support for first/last grid cell if it does not conflict with app shell. |
| Enter / Space | Activate the focused legal cell, or the focused forced-pass control. |
| Tab | Enter/exit the board region predictably; avoid 64 tab stops if using roving focus. |
| Escape | Optional, may clear preview/selection if introduced; must not mutate game state. |

Smoke tests must cover start, legal move display, keyboard activation, human action, bot action, forced pass, replay stepping, reduced motion, dev toggle safety, and no-leak/accessibility checklist items.

---

## 13. Tooling and CI requirements

### 13.1 Tool integration

Update all applicable tools:

| Tool | Required update |
|---|---|
| `tools/simulate` | Accept `--game directional_flip`; run many random legal games; report terminal outcomes, wins/draws, average length, throughput, and machine-readable failure context. |
| `tools/replay-check` | Discover/check directional-flip golden traces; verify stable hashes and expected diagnostics/outcomes. |
| `tools/fixture-check` | Validate `directional_flip_standard.fixture.json` and static data strictness. |
| `tools/rule-coverage` | Parse/validate `RULE-COVERAGE.md`; fail on gaps. |
| `tools/bench-report` | Include directional-flip benchmark report and threshold posture. |
| `tools/seed-reducer` | Normalize failing directional-flip simulation reports when enough context exists. |
| `tools/trace-viewer` | Display placement, grouped flip children, pass actions, terminal outcomes, and preview/effect correlation. |

Every updated tool must have help text, accepted game-id lists, and smoke/tests updated.

### 13.2 CI/workflows

Update workflows only as needed to include the new game in existing lanes. Do not create broad CI redesign. Gate 6 evidence must include:

- Rust build/test for the crate and affected workspace;
- game docs/checks;
- trace checks;
- replay checks;
- fixture checks;
- rule coverage;
- simulation smoke;
- benchmarks/report generation;
- web build;
- WASM smoke/load;
- web e2e smoke;
- no-leak/a11y smoke;
- boundary checks.

Benchmark gating must honor existing ADR posture. Do not hard-fail on speculative thresholds not supported by measured baseline evidence.

---

## 14. Exit criteria mapped to roadmap and official-game contract

Gate 6 exit is impossible unless all rows pass.

| Source obligation | Gate 6 exit criterion |
|---|---|
| Roadmap: directional scans | Rust legality and apply logic scan all eight directions deterministically, with unit/property/golden coverage. |
| Roadmap: bracketed grouped changes | Every placement flips all bracketed discs in every qualifying direction and emits a grouped semantic flip effect with ordered child entries. |
| Roadmap: pass/no-move if scoped | Forced pass is explicit, Rust-generated, legal only when no placement exists, forbidden when placement exists, and covered by tests/traces. |
| Roadmap: multi-piece effects | UI animation authority comes from grouped Rust effects, not state diffs. |
| Roadmap: replay reconstructs consequences | Replay export/import/step/reset reproduces placements, flips, passes, effects, views, terminal outcome, and hashes. |
| Roadmap: extraction documented | Primitive-pressure ledger resolves coordinate/direction/ray helper pressure. |
| Roadmap: helper if extracted typed/narrow/tested/documented/back-ported/benchmarked | If promoted, `game-stdlib` helper evidence is complete. If rejected/deferred, rationale and next trigger are complete. |
| Roadmap: no untyped directional selectors in data | Static data remains metadata only; no selectors/conditions/loops/triggers/procedural behavior. |
| Roadmap: no grid concepts in engine-core | `engine-core` remains mechanic-noun-free. |
| Official contract: source notes | `SOURCES.md` records consulted rules/strategy/accessibility/IP sources and original-prose policy. |
| Official contract: original rules | `RULES.md` contains Rulepath-authored prose for the fixed variant. |
| Official contract: rule coverage | `RULE-COVERAGE.md` has no silent gaps and is tool-checked. |
| Official contract: mechanics inventory | `MECHANICS.md` inventories chance, hidden information, action topology, resources, spatial topology, scoring, terminal, UI, bots, replay implications, and primitive pressure. |
| Official contract: admission workflow | `GAME-IMPLEMENTATION-ADMISSION.md` checklist passes. |
| Official contract: Rust-owned behavior | Game crate owns setup, action generation, validation, transitions, scoring, terminal detection, effects, views, replay, serialization, RNG usage, and bots. |
| Official contract: tests | Required unit/property/visibility/serialization/bot tests pass. |
| Official contract: golden traces | Required trace suite passes and includes multi-direction flip, forced pass, terminal double-pass, invalid diagnostics, bot action, preview flip set, and WASM-exported trace. |
| Official contract: replay/hash | Replay/hash determinism passes across native and WASM where applicable. |
| Official contract: serialization | Unknown-field and behavior-looking data rejections pass. |
| Official contract: simulation | Many random legal games complete with terminal outcomes, average length, and throughput report. |
| Official contract: benchmarks | Honest baseline benchmarks exist; thresholds are documented and not fake. |
| Official contract: bots | Level 0 and Level 2-lite bots validate through legal command path; Level 2-lite has docs/evidence/tests/determinism/safe explanations. |
| Official contract: UI metadata | Rust supplies UI labels, legal metadata, previews, token metadata, and effect anchors. |
| Official contract: web smoke once exposed | Directional Flip web smoke, reduced-motion smoke, replay smoke, bot smoke, forced-pass smoke, and no-leak/a11y smoke pass before exposure. |
| Official contract: IP | Neutral public identity and original assets/prose pass `SOURCES.md` and `PUBLIC-RELEASE-CHECKLIST.md`. |
| Spec progress workflow | Implementation documents that `specs/README.md` can be updated from Not started/Planned only after evidence passes; no roadmap progress edit is required. |

---

## 15. Acceptance evidence

The implementer must produce evidence for every row. “Not applicable” must be written explicitly where allowed.

| # | Required fact for Gate 6 exit | Evidence required |
|---:|---|---|
| 1 | Intake confirmed every referenced path/symbol against current `main`. | Implementation PR/spec notes record the foundation-set read and the path/symbol confirmation done in G6-00. |
| 2 | `directional_flip` crate compiles. | Workspace build/test evidence. |
| 3 | Standard variant setup and rules documented in original Rulepath prose. | `RULES.md`, source cross-reference. |
| 4 | Source notes complete and IP-safe. | `SOURCES.md`, no copied prose/assets/brand/trade dress. |
| 5 | Rule coverage matrix has no silent gaps. | `RULE-COVERAGE.md` and `tools/rule-coverage --game directional_flip`. |
| 6 | Mechanic inventory complete. | `MECHANICS.md`. |
| 7 | Primitive-pressure ledger resolves coordinate/direction/ray pressure. | Ledger plus `docs/MECHANIC-ATLAS.md` update. |
| 8 | Any required `game-stdlib` extraction complete, tested, documented, benchmarked, and back-ported where required. | If promoted: helper tests/docs/benchmarks/back-port evidence. If rejected/deferred: explicit `not applicable` plus rationale and trigger. |
| 9 | Rust generates legal actions, previews, diagnostics, semantic effects, views, replay support, and bots. | Game crate tests, trace evidence, WASM snapshots. |
| 10 | TypeScript remains presentation-only. | Code review, smoke tests, no forbidden TS legality/flip/pass computation. |
| 11 | Golden traces pass and include multi-direction flip, forced pass, terminal double-pass, invalid diagnostics, bot action, and WASM-exported trace. | Trace files and replay-check output. |
| 12 | Replay/hash determinism passes. | Native and WASM replay-check output. |
| 13 | Serialization/unknown-field behavior passes. | Serialization tests and fixture-check. |
| 14 | Simulations complete many random legal games and report terminal outcomes, average length, and throughput. | `tools/simulate --game directional_flip` report. |
| 15 | Seed reducer can normalize a failing directional-flip simulation report when enough context exists. | `tools/seed-reducer` test/smoke. |
| 16 | Level 0 random bot validates through normal command path. | Bot tests and bot-action trace. |
| 17 | Level 2-lite authored policy has competent-player doc, evidence pack, tests, deterministic behavior, benchmarks, and safe explanations. | `COMPETENT-PLAYER.md`, `BOT-STRATEGY-EVIDENCE-PACK.md`, `AI.md`, tests, benchmark, trace. |
| 18 | Benchmarks produce honest baseline evidence; thresholds are not fake. | `BENCHMARKS.md`, benchmark output, threshold rationale. |
| 19 | WASM API supports the game across normal match, bot turn, effects, replay export/import, replay step/reset. | WASM tests/smoke and exported trace. |
| 20 | Public UI is polished, accessible, reduced-motion safe, and uses Rust legal choices/previews/effects. | E2E smoke, UI checklist, no TS legality. |
| 21 | UI smoke tests pass. | `apps/web/e2e/directional-flip.smoke.mjs` output. |
| 22 | No-leak/accessibility checklist is updated and passes applicable checks. | `NO-LEAK-A11Y-CHECKLIST.md` and a11y smoke. |
| 23 | Public release checklist passes before game picker exposure. | `PUBLIC-RELEASE-CHECKLIST.md`; commit/order evidence showing exposure after checklist. |
| 24 | `specs/README.md` update requirement documented. | Implementation notes say status can be flipped only after evidence passes. |
| 25 | No foundation stop condition remains open. | Final review checklist; boundary check; unresolved ADR/stops list empty. |

---

## 16. FOUNDATIONS and boundary alignment

| FOUNDATIONS principle | Gate 6 stance |
|---|---|
| §1 Priority order | Directional Flip must be polished before public exposure, not debug-first; public playability wins over speculative generality. |
| §2 Behavior authority | All legality, pass, flip, preview, effect, bot, replay, scoring, and serialization behavior are Rust-owned; the UI renders Rust-provided legal choices/previews/effects and never computes legality or flips. |
| §3 `engine-core` is a contract kernel | `engine-core` remains generic and mechanic-noun-free; game logic lives in `games/directional_flip`; `wasm-api` is thin glue; no board/grid/cell/flip vocabulary enters the kernel. |
| §4 `game-stdlib` is earned | Gate 6 cannot exit without a coordinate/direction/ray primitive-pressure ledger decision; any promotion is the earned third-use outcome, not speculative. |
| §5 Static data is not behavior | TOML/JSON may contain ids, names, dimensions, labels, and asset references only; no selectors, conditions, triggers, loops, or procedural rule behavior. |
| §6 Official games are evidence-heavy | Rust action tree, validation, effects, replay, hashing, simulations, traces, benchmarks, docs, and tests are required before release. |
| §7 Public UI is central product work | React + SVG board polished to Column Four parity; animation driven by Rust semantic effects, not renderer state diffs. |
| §8 Public bots are product opponents | Level 0 random legal and Level 2-lite authored policy only; no minimax/alpha-beta/MCTS/ISMCTS/Monte Carlo/ML/RL/LLM. |
| §10 IP conservatism | Neutral name and original visuals/prose avoid Othello branding and trade dress. |
| §9 Local-first v1/v2 | No accounts, hosted multiplayer, server persistence, matchmaking, ranked play, or chat. |
| §11 Universal acceptance invariants | Determinism, fail-closed serialization, viewer-safe no-leak views, keyboard / non-color-only / reduced-motion accessibility, and evidence coverage all hold; see §14/§15. |
| §12 Stop conditions | Kept clear; see the Stop conditions list below. |
| §13 ADR triggers | No replay/hash semantic change, no kernel-vocabulary change, and no new bot search class — so no ADR is triggered by Gate 6. |

### Stop conditions

Stop implementation and ask for decision/ADR if any of these occur:

- a proposed change adds board/grid/cell/coordinate/line/capture/flip or similar mechanic vocabulary to `engine-core`;
- TypeScript computes legal placements, pass availability, or flip consequences;
- static data starts containing selectors, rule branches, loops, triggers, conditions, tactical AI conditions, or DSL-like fields;
- primitive-pressure decision is unresolved;
- helper extraction would affect replay/hash/data/visibility/kernel architecture;
- Level 2-lite code appears before competent-player and evidence-pack docs;
- implementation needs minimax/alpha-beta/MCTS/ISMCTS/Monte Carlo/ML/RL/LLM runtime selection;
- public picker exposure is proposed before release checklist passes;
- external IP/trademark review raises unresolved naming/trade-dress concern;
- benchmark thresholds are invented rather than measured;
- replay/hash semantics must change without a migration/ADR.

---

## 17. Forbidden changes

The Gate 6 implementation must not:

- add board, grid, cell, coordinate, adjacency, line, capture, flip, or similar mechanic nouns to `engine-core`;
- put directional-flip rule logic in `engine-core`;
- make TypeScript compute legality;
- make TypeScript compute pass availability;
- make TypeScript compute flip consequences;
- make TypeScript guess normal-mode animations from state diffs;
- use state diffs as normal animation authority instead of Rust semantic effects;
- put rule behavior, selectors, conditions, triggers, loops, tactical AI conditions, or DSL-like fields in static data;
- introduce YAML;
- do broad generic engine cleanup;
- introduce speculative `game-stdlib` abstractions beyond the primitive-pressure ledger decision;
- implement MCTS;
- implement ISMCTS;
- implement Monte Carlo playout bots;
- implement ML/RL bots;
- use LLM runtime move selection;
- implement minimax, alpha-beta, or recursive search for Gate 6;
- copy Othello rule prose;
- copy Othello diagrams;
- use Othello branding;
- use Othello screenshots, scans, icons, proprietary assets, or trade dress;
- make trademark-forward presentation;
- expose the game publicly before the release checklist passes;
- add accounts, hosted multiplayer, matchmaking, database, chat, ranked play, or server persistence;
- change replay/hash semantics without explicit migration/ADR as repository law requires;
- archive this spec as part of implementation;
- edit `docs/ROADMAP.md` for progress tracking.

---

## 18. Documentation updates required

| Path | Required update |
|---|---|
| `games/directional_flip/docs/SOURCES.md` | Complete source/IP notes with original-prose policy and neutral naming rationale. |
| `games/directional_flip/docs/RULES.md` | Original rules for fixed 8×8 variant. |
| `games/directional_flip/docs/RULE-COVERAGE.md` | Matrix linking each rule id to tests/traces/tools. |
| `games/directional_flip/docs/MECHANICS.md` | Mechanic inventory and repeated-shape comparison. |
| `games/directional_flip/docs/GAME-IMPLEMENTATION-ADMISSION.md` | Official-game admission checklist. |
| `games/directional_flip/docs/COMPETENT-PLAYER.md` | Strategy foundation for Level 2-lite. |
| `games/directional_flip/docs/BOT-STRATEGY-EVIDENCE-PACK.md` | Source-backed bot policy evidence. |
| `games/directional_flip/docs/AI.md` | Bot levels, boundaries, rationale, determinism. |
| `games/directional_flip/docs/UI.md` | UI design, accessibility, preview/effect ownership. |
| `games/directional_flip/docs/BENCHMARKS.md` | Baseline benchmark results and threshold strategy. |
| `games/directional_flip/docs/PUBLIC-RELEASE-CHECKLIST.md` | Release evidence; must pass before picker exposure. |
| Primitive-pressure ledger path | Filled ledger for coordinate/direction/ray decision. |
| `docs/MECHANIC-ATLAS.md` | Update with Directional Flip mechanics and extraction/defer decision. |
| `apps/web/e2e/NO-LEAK-A11Y-CHECKLIST.md` | Add Directional Flip applicable checks. |
| Tool help/docs if present | Add `directional_flip` accepted game id and examples. |
| `specs/README.md` | Per the index workflow (`Not started → Planned` once the spec is written), flip the Gate 6 row to `Planned` and link this spec now. Advance to `In progress` when AGENT-TASKs execute, and to `Done` only after the exit criteria pass with evidence. |
| `docs/ROADMAP.md` | Not applicable; do not edit for progress. |
| Archive docs | Not applicable during implementation; do not archive this spec. |

---

## 19. Sequencing

| Item | Requirement |
|---|---|
| Predecessor | Gate 5 Column Four public polish is done and archived; use it as pattern evidence only. |
| Current gate | Gate 6 Directional Flip. |
| Successor | Next roadmap gate is not admitted by this spec. Do not broaden into Gate 7+. |
| Admission rule | Directional Flip may become public only after official-game contract evidence and public-release checklist pass. |
| Implementation order | Docs/source notes and primitive-pressure decision precede or happen very early; Level 2-lite docs precede Level 2-lite code; public exposure comes last. |
| Status update | `specs/README.md` may read `Planned` once this spec exists; it advances to `In progress` during execution and to `Done` only after implementation evidence passes. |
| Archival | Archival is not part of this Gate 6 implementation spec. |

---

## 20. One-line-correctable assumptions

| Assumption | Correction mechanism |
|---|---|
| Cell ids use `r<row>c<col>`, row 1 at top, column 1 at left. | Change the cell-id convention in one doc line and update tests/traces consistently before implementation. |
| First seat starts at `r4c5` and `r5c4`; second seat starts at `r4c4` and `r5c5`. | If repository coordinate orientation differs, restate the diagonal arrangement with equivalent cells. |
| Direction order is north, northeast, east, southeast, south, southwest, west, northwest. | If existing helper style requires another stable order, document it once and use it consistently in previews/effects/traces. |
| Flat action segments are `place/<cell-id>` and `pass/forced`. | If engine action path conventions require a different segment shape, preserve stable labels and legal-only tree semantics. |
| Level 2-lite phase thresholds can use empty-cell count. | `COMPETENT-PLAYER.md` may set exact thresholds if evidence supports them; tests must pin behavior. |
| `DirectionalFlipBoard.tsx` is a new component. | If renderer conventions favor a shared board component, reuse only presentation code without moving game legality into TypeScript. |
| `games/directional_flip/docs/PRIMITIVE-PRESSURE-LEDGER.md` is acceptable. | If repository convention dictates a central ledger, use that location and link it from game docs. |
| Benchmark thresholds start non-blocking baseline-first. | If CI requires blocking thresholds, choose conservative measured thresholds and document CI evidence. |
| React + SVG is the correct default renderer. | Canvas/Pixi or equivalent requires repository-approved evidence/ADR under current UI rules. |

---

## 21. Required rule-id coverage seed

The implementer may refine ids, but `RULE-COVERAGE.md` must cover at least these obligations.

| Rule id seed | Rule |
|---|---|
| `DF-SETUP-001` | Standard 8×8 setup has the correct four center discs and active first seat. |
| `DF-ACTION-001` | Action tree exposes only legal placement choices when placements exist. |
| `DF-ACTION-002` | Action tree exposes exactly one forced-pass choice when no placement exists and state is nonterminal. |
| `DF-ACTION-003` | Pass is absent when any legal placement exists. |
| `DF-LEGAL-001` | Placement requires at least one bracketed contiguous opposing line. |
| `DF-LEGAL-002` | Occupied target is rejected. |
| `DF-LEGAL-003` | Non-flipping target is rejected. |
| `DF-LEGAL-004` | Out-of-bounds/malformed cell is rejected. |
| `DF-LEGAL-005` | Stale command is rejected. |
| `DF-LEGAL-006` | Non-active actor is rejected. |
| `DF-FLIP-001` | All bracketed discs in every qualifying direction flip. |
| `DF-FLIP-002` | No skipped own discs are used to create flips. |
| `DF-FLIP-003` | No indirect/non-line discs flip. |
| `DF-FLIP-004` | Flip order is stable and documented. |
| `DF-PREVIEW-001` | Preview flip set equals apply flip set. |
| `DF-PASS-001` | Forced pass advances turn through normal command path. |
| `DF-PASS-002` | Double forced pass ends the game. |
| `DF-TERM-001` | Terminal action tree has no legal placement/pass choices. |
| `DF-SCORE-001` | Higher disc count wins. |
| `DF-SCORE-002` | Equal disc count draws. |
| `DF-EFFECT-001` | Placement emits accepted/place/grouped-flip/turn/terminal effects as applicable. |
| `DF-EFFECT-002` | Grouped flip child entries match deterministic order. |
| `DF-VIEW-001` | Public view contains no hidden/internal state. |
| `DF-REPLAY-001` | Replay/hash deterministic across export/import/step/reset. |
| `DF-SER-001` | Unknown fields and behavior-looking fields are rejected. |
| `DF-BOT-001` | Level 0 random bot validates through command path. |
| `DF-BOT-002` | Level 2-lite validates through command path and remains deterministic. |
| `DF-UI-001` | UI uses Rust legal choices/previews/effects and no TypeScript legality. |
| `DF-UI-002` | Keyboard grid, forced pass control, reduced motion, and non-color-only state encoding pass smoke. |
| `DF-IP-001` | Public presentation is neutral and original. |
| `DF-PRIM-001` | Primitive-pressure ledger decision completed. |

---

## 22. Source references

All game prose in the repository must be original Rulepath prose. The external sources below shaped requirements and source notes only (see §4); they are recorded here for IP provenance and decomposition traceability.

### 22.1 External research sources consulted

| Source | URL | Used for |
|---|---|---|
| World Othello Federation official rules | <https://www.worldothello.org/about/about-othello/othello-rules/official-rules/english> | Core 8×8 directional-flip rule shape: setup, first move, outflank, forced forfeit/pass, all flips, terminal count. |
| World Othello Federation Othello information | <https://www.worldothello.org/about/wof-council-committees/partners-sponsors/othello-information> | Othello(TM) license/naming caution. |
| WOF history | <https://www.worldothello.org/about/wof-council-committees/history-wof> | Trademark-holder context and MegaHouse/Othello Co relationship. |
| Justia OTHELLO trademark record | <https://trademarks.justia.com/730/61/othello-73061971.html> | Trademark status/owner caution. |
| Othello Belgium strategy | <https://en.othellobelgium.be/leer-othello/tips-en-strategie> | Strategy evidence: greed trap, stable discs, corners, X/C squares, mobility, frontier, quiet moves, parity. |
| Nederlandse Othello Vereniging strategy guide | <https://www.othello.nl/content/guides/comteguide/strategy.html> | Strategy evidence: stable discs, mobility, frontier, quiet moves, opening greed caution, parity/passing. |
| WAI-ARIA APG grid pattern | <https://www.w3.org/WAI/ARIA/apg/patterns/grid/> | Interactive grid focus, arrows, Home/End, roving focus constraints. |
| WCAG 2.2 Use of Color | <https://www.w3.org/TR/WCAG22/#use-of-color> | Non-color-only state encoding requirement. |
| WCAG 2.2 Animation from Interactions | <https://www.w3.org/TR/WCAG22/#animation-from-interactions> | Reduced-motion / disable interaction-triggered motion requirement. |
| ReversiWorld bitboard move generation | <https://reversiworld.wordpress.com/2013/11/05/generating-moves-using-bitboard/> | Background implementation lesson that directional move generation is a real computational pressure point; not used to override Rulepath boundaries. |

### 22.2 Sources attempted but not relied on

| Source | Reason |
|---|---|
| Cornell Othello AI page, <https://www.cs.cornell.edu/~yuli/othello/othello.html> | Fetch failed in this session; not used as evidence. |
| PDF strategy/AI sources from search results | Not needed because HTML strategy sources provided sufficient evidence and no PDF analysis was required for this specification. |

# Gate 7.1 — Board-Space Primitive Back-Port and Promotion-Debt Closure

**Header**

- Spec ID: `gate-7-1-board-space-primitive-back-port`
- Roadmap stage: 5M (maintenance / interlock)
- Roadmap build gate: Gate 7.1 (`docs/ROADMAP.md` §9A)
- Status: Completed
- Date: 2026-06-07
- Owner: joeloverbeck
- Authority order (this spec is subordinate to, in order): `docs/FOUNDATIONS.md` →
  `docs/ARCHITECTURE.md` → `docs/ENGINE-GAME-DATA-BOUNDARY.md` →
  `docs/OFFICIAL-GAME-CONTRACT.md` → `docs/MECHANIC-ATLAS.md` → `docs/ROADMAP.md` →
  `docs/AGENT-DISCIPLINE.md` → `docs/TESTING-REPLAY-BENCHMARKING.md` →
  `specs/README.md` → per-game `MECHANICS.md` / pressure ledgers / tests / traces /
  fixtures / benches / Rust source. Where this spec and a foundation document
  disagree, the foundation document wins.

> **Reader orientation.** Sections **A–L** are the canonical spec surface (the
> section set in `specs/README.md` §Spec format). Sections **2–21** below them are
> the **retained detailed reference** — the per-game retrofit requirements and
> evidence lists that the Work-breakdown items (§D) point into. The source plan's
> Section 1 (a commit-pinned evidence ledger and fetch-authority preamble) was
> removed as generation-harness scaffolding: this spec validates against the live
> repository, not a pinned commit.

## A. Objective

Close the promotion debt created when Gate 7 promoted `game-stdlib::board_space`
(narrow, behavior-free rectangular board-space identity) and made Draughts Lite use
it, but intentionally did not force-retrofit the earlier official board games.
Back-port the promoted coordinate/cell/bounds/index/iteration/offset subset into
`three_marks`, `column_four`, and `directional_flip`; audit `race_to_n` as not
applicable; keep `draughts_lite` as the exemplar/regression target — all while
preserving public behavior. This is a maintenance/interlock gate, mandatory before
Gate 8 or any later mechanic-ladder advancement (ROADMAP §9A; FOUNDATIONS §4, §11).
Detailed motivation: §3.

## B. Scope

**In scope:** §3 (purpose), §6 (affected primitive), §7 (classification), §8
(cross-cutting requirements), §9–§14 (per-game and `game-stdlib` requirements), the
per-game documentation updates and the atlas debt-status flip in §15.

**Out of scope / not allowed** (carries ROADMAP §9A "Not allowed" plus §4 here): new
`engine-core` board/grid/cell vocabulary; a broad generic board-game engine; generic
occupancy boards; generic line-win, gravity, directional-flip/bracketing, ray-scan
capture, movement, capture, promotion, mandatory-capture, forced-continuation, or
bot-strategy helpers; a DSL or data-driven rule language; TypeScript
legality/preview migration; accidental trace/hash migration; implementation-ticket
decomposition inside this spec; a new official game before this debt closes. Full
list: §4, §18.

## C. Deliverables

| # | Deliverable | Target | Detail |
|---|---|---|---|
| D1 | `three_marks` board-space retrofit | `games/three_marks/src/{ids,rules,state}.rs`, `Cargo.toml` | §9 |
| D2 | `column_four` board-space retrofit | `games/column_four/src/{ids,rules,…}.rs`, `Cargo.toml` | §10 |
| D3 | `directional_flip` board-space retrofit | `games/directional_flip/src/{ids,rules}.rs`, `Cargo.toml` | §11 |
| D4 | `race_to_n` not-applicable audit | `games/race_to_n/docs/MECHANICS.md` | §12 |
| D5 | `draughts_lite` regression + exemplar confirmation | `games/draughts_lite/*` (no code change expected) | §13 |
| D6 | `game-stdlib` narrow test/example additions (only if required to remove duplication) | `crates/game-stdlib/src/board_space.rs` | §14 |
| D7 | Per-game `MECHANICS.md` / pressure-ledger doc updates | per §15 second list | §15 |
| D8 | Atlas debt-status flip + index status flip (after evidence) | `docs/MECHANIC-ATLAS.md` §10A, `specs/README.md` | §15, §J |

> The governance/foundation-doc back-port *language* (FOUNDATIONS §4/§11/§12,
> MECHANIC-ATLAS §10A register + the `board_space` row, ROADMAP §9A, specs/README
> Gate 7.1 row, and the supporting ARCHITECTURE / ENGINE-GAME-DATA-BOUNDARY /
> OFFICIAL-GAME-CONTRACT / AGENT-DISCIPLINE / TESTING-REPLAY-BENCHMARKING edits)
> already landed in commit `0286f9f` ("Improved foundational docs"). Rewriting those
> documents is **not** a deliverable of this gate. See §15.

## D. Work breakdown

| WB | Item | Depends on | AGENT-TASK |
|---|---|---|---|
| WB-1 | Retrofit `three_marks` coordinate/cell identity to `board_space`; preserve `CellId` surface, `ALL` order, `index`, action paths, summaries (§9) | — | yes |
| WB-2 | Retrofit `column_four` cell/coordinate identity to `board_space`; preserve `ColumnId`/`RowId`, `CellId` IDs/`ALL`/`index`, row-origin, landing (§10) | — | yes |
| WB-3 | Retrofit `directional_flip` coordinate identity + bounded offset stepping to `board_space`; preserve `CellId`, `Direction::ALL`, `step` behavior (§11) | — | yes |
| WB-4 | Audit `race_to_n`; add not-applicable note to its `MECHANICS.md` (§12) | — | yes |
| WB-5 | Run `draughts_lite` regression (tests + replay) as exemplar proof; ledger note (§13) | — | yes |
| WB-6 | Add only the behavior-free `board_space` tests/examples needed to remove duplication surfaced by WB-1..3 (§14) | WB-1..3 | optional |
| WB-7 | Update per-game `MECHANICS.md` / pressure-ledger docs to record conformance (§15) | WB-1..5 | yes |
| WB-8 | After all code + evidence pass: flip atlas §10A `board_space` debt to closed; flip `specs/README.md` Gate 7.1 status to `Done` (§15, §17.12) | WB-1..7 | yes |

WB-1..5 are independent and may run in parallel. WB-8 is the closure gate and MUST
NOT run until every affected game's evidence (§F) passes.

## E. Exit criteria (mapped row-for-row to ROADMAP §9A "Exit")

| ROADMAP §9A exit line | This spec's mechanism |
|---|---|
| `three_marks` / `column_four` / `directional_flip` depend on `game-stdlib` and no longer duplicate `board_space` coord/cell/bounds/index behavior in promoted scope | §9.2, §10.2, §11.2; §8; WB-1..3; §17.1 |
| Game-local semantics stay local (TM lines; C4 columns/gravity/four-in-a-row; DF ray bracketing/flips/forced pass; DL movement/capture/promotion/forced continuation) | §9.2, §10.2, §11.2, §13.2; §4; §17.7 |
| All native tests, replay/golden-trace, visibility, serialization, benchmark, and web-smoke checks for affected games pass with stable behavior by default | §8.2, §8.7, §8.8, §16; §9.3, §10.3, §11.3, §12.2, §13; §17.8–§17.9 |
| `race_to_n` audited and documented as not applicable (or the gate stops with evidence) | §12; §17.2 |
| Atlas open promotion-debt register has no open `board_space` debt | §15 (final paragraph), WB-8; §17.10, §17.12 |

## F. Acceptance evidence

Re-runnable confirmation (exact script/lane names may follow repo CI; see §16, §19):
workspace `fmt`/`clippy`; `cargo test` for `game-stdlib`, `three_marks`,
`column_four`, `directional_flip`, `draughts_lite`, `race_to_n`; replay-check across
every golden trace listed in §9.3 / §10.3 / §11.3 / §12.2 / §13.2; fixture /
static-data validation; rule-coverage where configured; benchmark smoke with
unchanged thresholds; `wasm-api` build/tests; web build; web smoke for
`three_marks` / `column_four` / `directional_flip` / `draughts_lite`; no-leak/a11y
smoke; doc-link checks. The implementation summary MUST state whether any golden
trace changed (expected: **no**) per §16. Detailed per-game evidence: §9.3, §10.3,
§11.3, §12.2, §13.

## G. FOUNDATIONS & boundary alignment

| Principle | Stance | Rationale |
|---|---|---|
| §2 Behavior authority | aligned | No legality, validation, effect, view, replay, or bot logic moves out of Rust; TypeScript stays presentation-only (§8.6, §17.6, §18). |
| §3 `engine-core` is a contract kernel | aligned | No board/grid/cell/mechanic noun enters `engine-core`; all retrofit lands in `games/*` against `game-stdlib` (§4, §8.5, §17.5, §18). |
| §4 `game-stdlib` is earned | aligned | No new promotion; this gate executes the already-recorded back-port of the promoted `board_space` per the atlas §10A debt register and the §4 third-use rule. |
| §5 Static data is not behavior | aligned | No new static data, selectors, or rule-like fields; win lines, gravity, and flip logic stay as typed Rust, not data (§9.2, §10.2, §11.2). |
| §11 Universal acceptance invariants | aligned | Promoted primitive adopted by all matching games (or audited n/a); replay/hash/traces preserved deterministically; views stay viewer-safe; evidence coverage in §F. |
| §12 Stop conditions | clear | This spec **is** the recorded promotion-debt closure gate, sequenced before Gate 8; no stop condition is crossed. |

**§12 stop-condition check.** "A promoted primitive leaves matching prior official
games un-migrated without an explicit exception or recorded closure gate" — **clear**
(this is that closure gate). "A new mechanic-ladder gate proceeds while promotion
debt is still open" — **clear** (Gate 8 is gated behind this, §J). "`engine-core`
gains mechanic nouns / TypeScript decides legality / `board_space` broadened" —
**clear** (all forbidden, §4 / §17 / §18).

## H. Forbidden changes

See §18 for the retained detailed checklist. Summary: no public-surface change from
swapping semantic IDs to raw `Coord`; no row-origin/orientation flip; no
action/effect ordering change; no `board_space` broadening beyond behavior-free
coordinate operations; no game rules moved into `game-stdlib` or `engine-core`; no
trace/replay/schema-version change without an explicit migration decision; no test
deletion, weakening, or renaming away from failing behavior.

## I. Documentation updates required

- Per-game `MECHANICS.md` / pressure-ledger updates as code lands (§15 second list).
- After evidence passes: flip `docs/MECHANIC-ATLAS.md` §10A `board_space` debt to
  closed (§15 final paragraph) and flip this spec's row in `specs/README.md` from
  `Planned` to `Done` (per the index admission rule, §J).
- Foundation/governance docs: already landed in commit `0286f9f`; no further edits
  required by this gate (§15).

## J. Sequencing

- **Predecessor:** Gate 7 (`archive/specs/gate-7-draughts-lite-compound-action-tree.md`,
  status `Done` in `specs/README.md`) — promoted `board_space` and incurred the debt.
- **Successor:** Gate 8 (cards / chance / hidden information; not yet specced). The
  `specs/README.md` index and the ROADMAP §9A interlock forbid admitting Gate 8 while
  `board_space` promotion debt is open. This gate flips to `Done` only after its Exit
  criteria (§E) pass with evidence (`specs/README.md` admission rule).

## K. Assumptions

See §20 (retained, de-scaffolded). Core: `board_space` is intended to remain narrow;
the earlier official board games are admitted games subject to promoted-primitive
conformance obligations; existing behavior is correct unless a failing test or
explicit rule-source conflict proves otherwise.

## L. Out of this planning deliverable

See §21. No implementation tickets, no code patch, and no replacement governance
docs (those already landed in `0286f9f`); the implementer updates per-game docs after
code changes.

---

# Detailed reference (retained from the source plan)

> The sections below keep their original numbering (2–21). They are the substantive
> per-game requirements the canonical Work-breakdown items (§D) point into. Section 1
> (the commit-pinned evidence ledger and fetch-authority preamble) was removed as
> generation-harness scaffolding.

## 2. Authority and reading order

This spec is constrained by repository doctrine, especially:

1. `docs/FOUNDATIONS.md`
2. `docs/ARCHITECTURE.md`
3. `docs/ENGINE-GAME-DATA-BOUNDARY.md`
4. `docs/OFFICIAL-GAME-CONTRACT.md`
5. `docs/MECHANIC-ATLAS.md`
6. `docs/ROADMAP.md`
7. `docs/AGENT-DISCIPLINE.md`
8. `docs/TESTING-REPLAY-BENCHMARKING.md`
9. `specs/README.md`
10. per-game `MECHANICS.md`, pressure ledgers, tests, golden traces, fixtures, benches, Rust source, and WASM/UI smoke files.

If implementation discovers a direct contradiction between this spec and repository law, stop and raise a reassessment finding. Do not silently patch around the contradiction.

## 3. Purpose

Gate 7 promoted a narrow, behavior-free rectangular board-space primitive into `crates/game-stdlib/src/board_space.rs` and made Draughts Lite use it. Gate 7 also intentionally avoided forcing previous official board games to retrofit during that same gate. The result is real promotion debt: the repository now has a promoted primitive while earlier admitted official games still carry local coordinate/cell/bounds/indexing logic for the same promoted mechanic shape.

This gate closes that debt before the next mechanic-ladder gate. The work is governance repair as much as code cleanup: once a primitive is promoted, prior official games that use the promoted mechanic must either conform to it, be audited not applicable, or carry an explicit accepted exception. Future spec authors must be able to infer that obligation from the docs and atlas without chat memory.

## 4. Non-purpose

This spec does not authorize:

- implementation ticket generation or ticket breakdowns;
- a new official game;
- `engine-core` board/grid/cell vocabulary;
- a broad generic board-game engine;
- a DSL or data-driven rule language;
- TypeScript legality migration;
- generic occupancy boards;
- generic line-win detection;
- generic gravity/drop mechanics;
- generic directional-flip/bracketing mechanics;
- generic ray-scan capture primitives;
- generic draughts/checkers movement, capture, promotion, mandatory-capture, or forced-continuation helpers;
- bot strategy primitives;
- replay/hash migration unless an existing behavior is proven wrong and this spec's preservation rules are explicitly amended.

## 5. Background findings

Gate 7's own scope described the board-space primitive as a reopen-and-decide primitive decision and recommended promotion of only the smallest rule-agnostic rectangular board-space utilities. It also stated that retrofitting `three_marks`, `column_four`, and `directional_flip` during Gate 7 was out of scope.

The current `docs/MECHANIC-ATLAS.md` records `game-stdlib::board_space` as the promoted primitive for the narrow behavior-free subset of fixed 2D occupancy / coordinate identity: dimensions, coordinates, bounds, deterministic row-major iteration, signed offsets, stable `rNcM` parse/format, and parity. The same atlas row says Draughts Lite uses the helper and earlier games were not force-retrofitted during Gate 7.

The current `docs/ROADMAP.md` lists Gate 8 as the next new mechanic-ladder gate. Because open promotion debt exists, this Gate 7.1 maintenance/interlock must be accepted and implemented before Gate 8 or any later mechanic-ladder advancement.

## 6. Affected primitive: `game-stdlib::board_space`

The promoted primitive is narrow and rule-agnostic. Its current API includes:

- `Dimensions::checked(rows, cols)` for nonzero rectangular dimensions;
- `Dimensions::rows()` and `Dimensions::cols()`;
- `Dimensions::contains(coord)`;
- `Dimensions::coord(row, col)`;
- `Dimensions::parse_coord_id(value)`;
- `Dimensions::offset(coord, d_row, d_col)`;
- `Dimensions::coord_count()`;
- `Dimensions::row_major()`;
- `Coord::checked(row, col)`;
- `Coord::row()`, `Coord::col()`, `Coord::row_index()`, `Coord::col_index()`;
- `Coord::row_col_index(dimensions)`;
- `Coord::id()`, `Coord::parse_id(value)`, and `Display` as stable `rNcM` formatting;
- `Coord::parity()` and `Parity::of(row, col)`;
- `CoordIdError` variants for malformed, zero, and out-of-bounds coordinate IDs;
- `RowMajor` as a deterministic exact-size row-major iterator.

For this gate, `board_space` means exactly behavior-free board-space identity and coordinate operations. It does not include occupancy policy, game legality, target selection, landing selection, movement, capture, promotion, forced continuation, directional bracketing, line/pattern detection, win logic, semantic effects, UI behavior, WASM payload contracts, replay format, or bot policy.

## 7. Official-game classification

| Game | Classification | Required outcome |
|---|---|---|
| `race_to_n` | Audit/no-op | Record that it has no board-space mechanic. Do not add `game-stdlib` for symmetry. Preserve all behavior and traces. |
| `three_marks` | Must retrofit | Migrate duplicated 3×3 coordinate/cell identity, parse/format, bounds/indexing, and deterministic cell iteration to `game-stdlib::board_space` where it applies. Keep game rules local. |
| `column_four` | Must retrofit | Migrate rectangular board-space identity and coordinate/cell operations to `board_space`; keep column actions, gravity, full-column checks, landing row, and four-in-a-row scans local. |
| `directional_flip` | Must retrofit | Migrate 8×8 coordinate identity, bounds/indexing, deterministic iteration, and behavior-free bounded offset stepping to `board_space`; keep directional flip/bracketing/forced-pass logic local. |
| `draughts_lite` | Already conforming exemplar | Keep using `board_space`; use it as regression evidence and documentation exemplar. Do not broaden draughts mechanics into stdlib. |

## 8. Cross-cutting implementation requirements

1. Add `game-stdlib = { path = "../../crates/game-stdlib" }` only to official game crates that must use the promoted primitive and do not already depend on it.
2. Preserve public action path segments, metadata, labels, accessibility strings, semantic effects, effect ordering, diagnostic codes/messages, stable summaries, view payloads, trace hashes, replay hashes, bot legality, benchmark operation names, and UI/WASM smoke behavior unless a specific existing behavior is proven wrong and the accepted implementation records an intentional migration.
3. Prefer thin adapters/wrappers around `Coord` for game-local semantic IDs when IDs appear in traces, views, effects, docs, tests, or UI payloads. Do not rip out game-local vocabulary merely because `Coord` exists.
4. Replace duplicated local coordinate machinery only inside the promoted primitive's scope. Preserve game-specific rule semantics in game crates.
5. Do not move behavior into `engine-core`.
6. Do not move legality, preview authority, or rule decisions into TypeScript.
7. Do not update golden traces to mask accidental drift. If a trace changes, the implementation must prove that the old behavior was wrong and document the intentional compatibility decision.
8. Run native tests and replay checks before UI smoke checks. UI passing is not proof of rule correctness.
9. Leave private/hidden information and viewer-safety boundaries unchanged.
10. Keep the final implementation reviewable as conformance work, not a general abstraction project.

## 9. `three_marks` requirements

### 9.1 Current shape

`three_marks` currently has a game-local `CellId` enum for the nine `rNcM` cells, with local `ALL`, `index`, `as_str`, and `parse` behavior. Rules and state use `CellId::ALL`, cell indices, and `CellId::as_str()` in action choices, occupancy summaries, terminal lines, and effect summaries.

### 9.2 Required retrofit

- Add a `game-stdlib` dependency to `games/three_marks/Cargo.toml`.
- Define or reuse a local board-dimensions accessor equivalent to `Dimensions::checked(3, 3).expect("3x3 board dimensions are valid")`.
- Make 3×3 board-space identity rely on `Coord`/`Dimensions` for coordinate construction, parse/format, bounds checks, row-major indexing, and deterministic iteration.
- Preserve the public `CellId` surface unless implementation proves it is completely invisible. The preferred shape is a thin semantic wrapper around `Coord`, or lossless `CellId` ↔ `Coord` conversion, so trace/UI/action/effect vocabulary remains stable.
- Preserve `CellId::ALL` order exactly: `r1c1`, `r1c2`, `r1c3`, `r2c1`, `r2c2`, `r2c3`, `r3c1`, `r3c2`, `r3c3`.
- Preserve `CellId::index()` row-major indices exactly.
- Preserve action paths such as `place/r1c1`, action ordering, action metadata, labels, and diagnostic behavior.
- Preserve `ThreeMarksSnapshot::stable_summary()` cell ordering and string formatting.
- Keep `WINNING_LINES` and line-completion logic game-local. Static win lines are game rules, not the board-space primitive.

### 9.3 Required `three_marks` tests and evidence

Implementation must run existing unit/rule/property/serialization/visibility/replay/bot/benchmark coverage and preserve these golden traces unless an explicitly approved migration note exists:

- `games/three_marks/tests/golden_traces/bot-action.trace.json`
- `games/three_marks/tests/golden_traces/draw.trace.json`
- `games/three_marks/tests/golden_traces/not-applicable.trace.json`
- `games/three_marks/tests/golden_traces/occupied-diagnostic.trace.json`
- `games/three_marks/tests/golden_traces/shortest-normal.trace.json`
- `games/three_marks/tests/golden_traces/stale-diagnostic.trace.json`
- `games/three_marks/tests/golden_traces/terminal.trace.json`
- `games/three_marks/tests/golden_traces/wasm-exported.trace.json`

Additional conformance evidence required:

- a Rust test proving `CellId` ↔ `Coord` conversion or wrapper behavior round-trips all nine cells;
- a Rust test proving `CellId::ALL` remains exactly the old order;
- a Rust test proving invalid/out-of-bounds IDs still return the same user-visible diagnostics through normal validation;
- replay-check output proving old golden traces still pass;
- web smoke for `three_marks` proving Rust-supplied legal choices still render and apply.

## 10. `column_four` requirements

### 10.1 Current shape

`column_four` currently has local `ColumnId`, `RowId`, and `CellId` types. `ColumnId` is the player action vocabulary. `CellId` combines a row and column, exposes a 42-cell row-major `ALL`, implements local `index`, `as_string`, and `parse`, and is used in occupancy state, summaries, effects, diagnostics, and UI. Rule code keeps gravity local through `landing_cell`, checks full columns, and scans four directions for terminal lines.

### 10.2 Required retrofit

- Add a `game-stdlib` dependency to `games/column_four/Cargo.toml`.
- Use `Dimensions::checked(6, 7).expect("6x7 board dimensions are valid")` as the board-space dimension source.
- Preserve public `ColumnId`. Players choose a column; `ColumnId` is game-local action vocabulary and must not be replaced by raw cell coordinates.
- Preserve `RowId` when it is useful as public or game-local vocabulary. It may become an adapter around `Coord::row()` or remain a thin row wrapper, but duplicated parse/index/bounds behavior within the `board_space` scope should be removed or delegated.
- Make `CellId` rely on `Coord`/`Dimensions` for cell coordinate identity, bounds, `rNcM` parse/format, row-major indexing, and deterministic iteration. A thin `CellId(Coord)` wrapper or lossless conversion is preferred.
- Preserve public cell IDs exactly, including `r1c1` through `r6c7` and all existing display strings.
- Preserve the existing row-origin semantics. If row 1 is the public lowest/bottom row for landing and effects, do not flip it just because generic row-major iteration is abstract.
- Preserve `CellId::ALL` order and `index()` values exactly.
- Preserve action paths and legal-action ordering by `ColumnId::ALL`: `c1` through `c7`.
- Keep gravity, landing-row calculation, full-column checks, draw/full-board detection, four-in-a-row scans, winning-line tie-breaks, terminal effects, and highlight semantics game-local.

### 10.3 Required `column_four` tests and evidence

Implementation must run existing rule/property/visibility/replay/bot/benchmark coverage and preserve these golden traces unless an explicitly approved migration note exists:

- `games/column_four/tests/golden_traces/bot-action.trace.json`
- `games/column_four/tests/golden_traces/diagonal-win.trace.json`
- `games/column_four/tests/golden_traces/draw.trace.json`
- `games/column_four/tests/golden_traces/full-column-diagnostic.trace.json`
- `games/column_four/tests/golden_traces/horizontal-win.trace.json`
- `games/column_four/tests/golden_traces/invalid-column-diagnostic.trace.json`
- `games/column_four/tests/golden_traces/shortest-normal-win.trace.json`
- `games/column_four/tests/golden_traces/stale-diagnostic.trace.json`
- `games/column_four/tests/golden_traces/terminal-replay.trace.json`
- `games/column_four/tests/golden_traces/vertical-win.trace.json`
- `games/column_four/tests/golden_traces/wasm-exported.trace.json`

Additional conformance evidence required:

- Rust tests proving `CellId` ↔ `Coord` conversion or wrapper behavior round-trips all 42 cells;
- Rust tests proving `CellId::ALL` and `index()` match the previous row-major public order;
- Rust tests proving `ColumnId::ALL` action ordering is unchanged;
- Rust tests proving landing-cell behavior is unchanged for empty, partially filled, and full columns;
- replay-check output proving existing column diagnostics and terminal traces still pass;
- web smoke for `column_four` proving board rendering, column actions, diagnostics, and effects remain stable.

## 11. `directional_flip` requirements

### 11.1 Current shape

`directional_flip` currently has local `RowId`, `ColumnId`, and `CellId` types for an 8×8 board. Rule code defines local `Direction::ALL`, direction deltas, legal placement discovery, flip runs, ordered flip previews, pass behavior, terminal behavior, and a local `step(cell, direction)` helper that advances through row/column indices and bounds via game-local cell conversion.

### 11.2 Required retrofit

- Add a `game-stdlib` dependency to `games/directional_flip/Cargo.toml`.
- Use `Dimensions::checked(8, 8).expect("8x8 board dimensions are valid")` as the board-space dimension source.
- Make coordinate identity, bounds checks, `rNcM` parse/format, row-major indexing, deterministic iteration, and simple bounded offset stepping rely on `Coord`/`Dimensions`.
- Preserve the public `CellId` surface unless implementation proves it is invisible. The preferred shape is a thin `CellId(Coord)` wrapper or lossless `CellId` ↔ `Coord` conversion.
- Preserve `CellId::ALL` order and `index()` values exactly.
- Preserve `Direction::ALL` order exactly:
  `North`, `Northeast`, `East`, `Southeast`, `South`, `Southwest`, `West`, `Northwest`.
- `Direction::delta()` may remain game-local but `step` should use `Dimensions::offset` or an equivalent `board_space` operation for bounds-safe coordinate stepping.
- Keep legal placement generation, ray/bracket scanning, grouped flip runs, ordered flip previews, forced pass, double-pass terminal detection, full-board terminal detection, scoring, effects, and UI preview metadata game-local.
- Preserve preview flip-set ordering, effect grouping/order, action ordering, diagnostics, and stable summary strings.

### 11.3 Required `directional_flip` tests and evidence

Implementation must run existing rule/property/visibility/replay/bot/benchmark coverage and preserve these golden traces unless an explicitly approved migration note exists:

- `games/directional_flip/tests/golden_traces/bot-action.trace.json`
- `games/directional_flip/tests/golden_traces/corner-capture.trace.json`
- `games/directional_flip/tests/golden_traces/double-pass-terminal.trace.json`
- `games/directional_flip/tests/golden_traces/draw.trace.json`
- `games/directional_flip/tests/golden_traces/forced-pass.trace.json`
- `games/directional_flip/tests/golden_traces/full-board-terminal.trace.json`
- `games/directional_flip/tests/golden_traces/invalid-non-flipping-placement.trace.json`
- `games/directional_flip/tests/golden_traces/invalid-occupied-cell.trace.json`
- `games/directional_flip/tests/golden_traces/multi-direction-flip.trace.json`
- `games/directional_flip/tests/golden_traces/non-active-seat-diagnostic.trace.json`
- `games/directional_flip/tests/golden_traces/opening-legal-move.trace.json`
- `games/directional_flip/tests/golden_traces/preview-flip-set.trace.json`
- `games/directional_flip/tests/golden_traces/stale-diagnostic.trace.json`
- `games/directional_flip/tests/golden_traces/wasm-exported.trace.json`

Additional conformance evidence required:

- Rust tests proving `CellId` ↔ `Coord` conversion or wrapper behavior round-trips all 64 cells;
- Rust tests proving `CellId::ALL`, `index()`, and public `rNcM` formatting are unchanged;
- Rust tests proving `step` through each `Direction` matches previous behavior at interior cells, edges, and corners;
- Rust tests proving `Direction::ALL` and flip-run ordering are unchanged;
- replay-check output proving flip preview, multi-direction flip, forced pass, diagnostics, and terminal traces still pass;
- web smoke for `directional_flip` proving legal placement previews and effects remain viewer-safe and stable.

## 12. `race_to_n` audit requirements

### 12.1 Current shape

`race_to_n` is a numeric counter race. Its mechanics docs state there is no spatial topology, no positions, maps, routes, regions, boards, piles, tracks, movement, capture, placement, or board-like mechanic.

### 12.2 Required audit/no-op

- Do not add `game-stdlib` to `games/race_to_n/Cargo.toml`.
- Do not refactor counter state into a generic primitive.
- Add or update documentation evidence only if needed so the board-space promotion debt register can mark `race_to_n` not applicable.
- Preserve all behavior and tests.

Golden traces to preserve:

- `games/race_to_n/tests/golden_traces/bot-action.trace.json`
- `games/race_to_n/tests/golden_traces/invalid-stale-diagnostic.trace.json`
- `games/race_to_n/tests/golden_traces/not-applicable.trace.json`
- `games/race_to_n/tests/golden_traces/shortest-normal.trace.json`
- `games/race_to_n/tests/golden_traces/terminal.trace.json`
- `games/race_to_n/tests/golden_traces/wasm-exported.trace.json`

## 13. `draughts_lite` exemplar and regression requirements

### 13.1 Current shape

`draughts_lite` already depends on `game-stdlib` and imports `Coord`, `Dimensions`, and `Parity` from `game_stdlib::board_space`. It uses `board_dimensions()`, `is_playable_cell`, `state.board.row_major()`, `state.board.offset()`, `parse_coord_id()`, `Coord::id()`, and `row_col_index` in board-space-appropriate places.

### 13.2 Required outcome

- Keep Draughts Lite as the exemplar for `board_space` use.
- Do not broaden `board_space` to include draughts-specific behavior.
- Keep playable dark-square policy local to Draughts Lite even though it uses generic parity.
- Keep movement, capture path generation, mandatory capture, same-piece forced continuation, promotion, terminal detection, diagnostics, effects, bot policy, and UI guidance local.
- Run Draughts Lite tests and replay checks as regression proof that the promoted primitive still serves the game that introduced it.

Golden traces to preserve:

- `games/draughts_lite/tests/golden_traces/bot-action.trace.json`
- `games/draughts_lite/tests/golden_traces/forced-continuation-branch.trace.json`
- `games/draughts_lite/tests/golden_traces/illegal-continuation-diagnostic.trace.json`
- `games/draughts_lite/tests/golden_traces/mandatory-capture-suppresses-quiet.trace.json`
- `games/draughts_lite/tests/golden_traces/multi-jump.trace.json`
- `games/draughts_lite/tests/golden_traces/non-active-seat-diagnostic.trace.json`
- `games/draughts_lite/tests/golden_traces/non-playable-cell-diagnostic.trace.json`
- `games/draughts_lite/tests/golden_traces/occupied-destination-diagnostic.trace.json`
- `games/draughts_lite/tests/golden_traces/path-after-promotion-stop-diagnostic.trace.json`
- `games/draughts_lite/tests/golden_traces/promotion-during-capture-stop.trace.json`
- `games/draughts_lite/tests/golden_traces/promotion-quiet.trace.json`
- `games/draughts_lite/tests/golden_traces/quiet-while-capture-diagnostic.trace.json`
- `games/draughts_lite/tests/golden_traces/shortest-quiet.trace.json`
- `games/draughts_lite/tests/golden_traces/single-capture.trace.json`
- `games/draughts_lite/tests/golden_traces/stale-diagnostic.trace.json`
- `games/draughts_lite/tests/golden_traces/terminal-no-legal-moves.trace.json`
- `games/draughts_lite/tests/golden_traces/terminal-no-pieces.trace.json`
- `games/draughts_lite/tests/golden_traces/wasm-exported.trace.json`

## 14. `game-stdlib` requirements

`crates/game-stdlib/src/board_space.rs` should remain narrow. Implementation may add tests, examples, or tiny helpers only when they are behavior-free and required to remove duplicated coordinate code from the affected games.

Permitted, if directly justified by the retrofit:

- tests for existing `Dimensions`, `Coord`, `CoordIdError`, `Parity`, and `RowMajor` behavior;
- examples/anti-examples documenting correct use and non-use;
- narrow helpers for rectangular dimensions, one-based `rNcM` IDs, row/column indexing, row-major iteration, signed offsets, or generic parity.

Forbidden in `game-stdlib` for this gate:

- generic occupancy boards;
- generic line-win detection;
- generic gravity/drop mechanics;
- generic directional flip/bracketing mechanics;
- generic ray-scan capture primitives;
- generic draughts/checkers movement, capture, promotion, mandatory-capture, or forced-continuation logic;
- generic bot strategy;
- any behavior DSL or data-driven legality expression.

## 15. Required documentation updates after implementation

The foundation/governance back-port obligation language **already landed** in commit
`0286f9f` ("Improved foundational docs"). It is recorded across `docs/FOUNDATIONS.md`
(§4 promotion-is-not-only-extraction, §11 promoted-primitives-adopted + debt-closed
invariants, §12 stop conditions), `docs/MECHANIC-ATLAS.md` (§10A open-promotion-debt
register + the `board_space` row), `docs/ROADMAP.md` (§9A Gate 7.1), `specs/README.md`
(Gate 7.1 index row + interlock note), and the supporting edits to
`docs/ARCHITECTURE.md`, `docs/ENGINE-GAME-DATA-BOUNDARY.md`,
`docs/OFFICIAL-GAME-CONTRACT.md`, `docs/AGENT-DISCIPLINE.md`, and
`docs/TESTING-REPLAY-BENCHMARKING.md`. **This gate does NOT rewrite those documents;**
acceptance criterion §17.11 is already satisfied by that commit. Implementation must
treat them as authoritative and must not regress their back-port language.

Implementation must still update per-game docs as code changes land:

- `games/three_marks/docs/MECHANICS.md` — record `board_space` conformance while preserving line logic as local.
- `games/column_four/docs/MECHANICS.md` — record `board_space` conformance for cell/coordinate identity while preserving columns/gravity/four-in-row as local.
- `games/directional_flip/docs/MECHANICS.md` — record `board_space` conformance for coordinates/offset stepping while preserving ray bracketing/flips/forced pass as local.
- `games/directional_flip/docs/PRIMITIVE-PRESSURE-LEDGER.md` — update any prior board-space deferral language to say the narrow coordinate subset is now promoted and conformed, while scanning/flipping remains local.
- `games/draughts_lite/docs/MECHANICS.md` — keep it as the `board_space` exemplar and avoid implying draughts movement was promoted.
- `games/draughts_lite/docs/PRIMITIVE-PRESSURE-LEDGER.md` — clarify that Gate 7.1 closed back-port debt if the implementation does so.
- `games/race_to_n/docs/MECHANICS.md` — add or preserve an audit note that the board-space primitive is not applicable.

After implementation, `docs/MECHANIC-ATLAS.md` must move the `game-stdlib::board_space` row from `promotion-debt-open` to fully `promoted primitive` (and clear the §10A open-promotion-debt register entry) only when all must-retrofit games have actually conformed and `race_to_n` has audit/no-op evidence. Do not mark debt closed before code and tests prove it.

## 16. Required validation commands and evidence

The accepted implementation must provide exit evidence for native Rust, replay, fixtures, benchmarks, docs, and web smoke. Exact command names may follow the repository's existing scripts and CI lanes, but the evidence must cover at least:

- workspace formatting and linting;
- `cargo test` for `game-stdlib`, `three_marks`, `column_four`, `directional_flip`, `draughts_lite`, and `race_to_n`;
- replay-check for all golden traces listed in this spec;
- fixture/static-data validation for all official games;
- rule-coverage validation where configured;
- benchmark smoke for all affected games, with thresholds unchanged unless separately justified;
- `wasm-api` build/tests;
- web build;
- web smoke tests for `three_marks`, `column_four`, `directional_flip`, and `draughts_lite`;
- no-leak/a11y smoke where the web test package requires it;
- docs link checks.

The implementation summary must explicitly state whether any golden trace changed. The expected answer is no. If not no, the implementation must name every changed trace, every changed hash field, the old/new reason, and the accepted migration authority.

## 17. Acceptance criteria

This gate is accepted only when all of the following are true:

1. `three_marks`, `column_four`, and `directional_flip` use `game-stdlib::board_space` for the promoted coordinate/cell/bounds/index/iteration/offset scope, or an accepted atlas exception names a non-migration reason and proof.
2. `race_to_n` is explicitly audited as not applicable and does not gain a pointless `game-stdlib` dependency.
3. `draughts_lite` remains conforming and passes regression checks.
4. `board_space` remains rule-agnostic and behavior-free.
5. No `engine-core` board/grid/cell/mechanic nouns are introduced.
6. No TypeScript legality or preview authority is introduced.
7. No generic line, gravity, flip, ray, movement, capture, promotion, forced-continuation, occupancy, or bot-strategy primitive is introduced.
8. Public action paths, ordering, diagnostics, effects, traces, replay hashes, views, UI behavior, and benchmarks are preserved by default.
9. Golden trace preservation is proved for all affected games.
10. Per-game docs and the repo atlas reflect the actual implemented state.
11. The governance docs make future primitive-promotion back-port obligations mandatory and discoverable. *(Already satisfied by commit `0286f9f`; see §15. This gate must not regress that language.)*
12. `specs/README.md` records Gate 7.1 and does not advance Gate 8 ahead of open promotion debt; its status flips to `Done` only after the exit criteria (§E) pass with evidence.
13. No implementation tickets are produced by this spec.

## 18. Forbidden changes checklist

During implementation, reject any change that:

- replaces game-local semantic IDs with raw `Coord` in a way that changes public surfaces;
- flips row origin or display orientation;
- changes action ordering or legal action availability;
- changes effect ordering or viewer-safe payload shape;
- broadens `board_space` beyond behavior-free coordinate operations;
- introduces a generic board object with occupancy state;
- moves local game rules into `game-stdlib` or `engine-core`;
- updates traces just to make tests pass;
- changes replay/data/schema versions without an explicit migration decision;
- removes tests, weakens tests, or renames tests away from failing behavior.

## 19. Exit evidence checklist

The implementation PR or final work summary must report:

```text
Implemented gate: Gate 7.1 — Board-Space Primitive Back-Port and Promotion-Debt Closure
Board-space primitive unchanged/broadened: <unchanged | list exact additions>
engine-core changed: <no | explain accepted reason>
TypeScript legality changed: no
Golden traces changed: <no | list exact migrations>
three_marks conformance evidence: <tests/docs/traces>
column_four conformance evidence: <tests/docs/traces>
directional_flip conformance evidence: <tests/docs/traces>
race_to_n audit evidence: <doc path and statement>
draughts_lite regression evidence: <tests/traces>
Mechanic atlas debt status: <closed | exceptions named>
Roadmap/spec index status: <Gate 7.1 done only after evidence>
```

## 20. Assumptions

- The current `board_space` primitive is intended to remain narrow.
- The previous official board games are admitted games and therefore subject to promoted-primitive conformance obligations.
- Existing behavior is correct unless a failing test or explicit rule-source conflict proves otherwise.

## 21. Explicit non-goals

- No implementation tickets.
- No code patch in this planning deliverable.
- No replacement governance or per-game docs in this planning deliverable; the foundation/governance docs already landed in commit `0286f9f`, and the implementation must update per-game docs after code changes.
- No new official game before this debt is closed.
- No broad cleanup outside the promoted primitive's scope.

## Outcome

Completed: 2026-06-07

What changed:
- `three_marks`, `column_four`, and `directional_flip` now consume
  `game-stdlib::board_space` for the promoted coordinate/cell identity,
  bounds, row-major iteration, indexing, parsing/formatting, and bounded-offset
  scope while preserving public semantic ID surfaces.
- `race_to_n` is explicitly audited as not applicable to `board_space`.
- `draughts_lite` remains the conforming exemplar; movement, capture,
  promotion, forced continuation, effects, UI, and bot policy remain local.
- `docs/MECHANIC-ATLAS.md` closes the open `board_space` promotion debt and
  `specs/README.md` marks Gate 7.1 `Done`.

Deviations from original plan:
- Public semantic ID types were preserved as wrappers/adapters instead of being
  replaced by raw `Coord`, avoiding surface churn while still delegating the
  promoted behavior-free operations.

Verification results:
- `cargo test --workspace`
- `bash scripts/boundary-check.sh`
- `cargo clippy --workspace --all-targets -- -D warnings`
- `cargo run -p replay-check -- --game three_marks --all`
- `cargo run -p replay-check -- --game column_four --all`
- `cargo run -p replay-check -- --game directional_flip --all`
- `cargo run -p replay-check -- --game draughts_lite --all`
- `cargo run -p replay-check -- --game race_to_n --all`
- `cargo run -p fixture-check -- --game three_marks`
- `cargo run -p fixture-check -- --game column_four`
- `cargo run -p fixture-check -- --game directional_flip`
- `cargo run -p fixture-check -- --game draughts_lite`
- `cargo run -p fixture-check -- --game race_to_n`
- `cargo run -p rule-coverage -- --game three_marks`
- `cargo run -p rule-coverage -- --game column_four`
- `cargo run -p rule-coverage -- --game directional_flip`
- `cargo run -p rule-coverage -- --game draughts_lite`
- `cargo run -p rule-coverage -- --game race_to_n`
- `npm --prefix apps/web run build`
- `npm --prefix apps/web run smoke:ui`
- `node scripts/check-doc-links.mjs`
- Board-space primitive broadened: no.
- `engine-core` changed: no.
- TypeScript legality changed: no.
- Golden traces changed: no.

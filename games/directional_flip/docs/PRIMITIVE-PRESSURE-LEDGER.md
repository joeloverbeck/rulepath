# Primitive Pressure Ledger: rectangular coordinates and directional rays

Candidate name: `rectangular-coordinate-directional-ray`

Status: rejected/deferred with rationale

Last updated: 2026-06-07

Prepared by: `Codex`

## Hard gate

A third official game with the same mechanic shape is blocked until this ledger
records one of reuse, promotion, explicit defer/reject, or ADR-required.

Decision: defer-reject

Gate 6 may proceed with game-local Directional Flip coordinate and ray logic.
No helper is promoted to `game-stdlib` by this decision, and no vocabulary or
surface is added to `engine-core`.

## Mechanic shape

Repeated shape considered:

- bounded rectangular position identity;
- stable row/column cell formatting and parsing;
- deterministic row-major iteration;
- directional offsets over a bounded rectangle;
- line or ray traversal used by Rust-owned rules.

The considered helper shape is behavior-free coordinate math. It does not
include occupancy, legality, win detection, flip/capture policy, pass policy,
bot strategy, semantic effect names, UI presentation, or static-data behavior.

## Games exerting pressure

| Game | Roadmap stage/gate | Local implementation area | Pressure type | Status at time of review | Notes |
|---|---:|---|---|---|---|
| `three_marks` | Gate 4 | `games/three_marks/src/ids.rs`, `rules.rs`, `visibility.rs`, `ui.rs` | first fixed-grid occupancy and static line groups | implemented | Uses a nine-case `CellId` enum and a static list of eight winning lines. |
| `column_four` | Gate 5 | `games/column_four/src/ids.rs`, `rules.rs`, `state.rs`, `visibility.rs` | second rectangular occupancy plus local line scanning | implemented | Uses typed `RowId`, `ColumnId`, `CellId`, bottom-origin rows, gravity landing, and four win directions. |
| `directional_flip` | Gate 6 | planned `games/directional_flip/src/ids.rs`, `rules.rs`, `visibility.rs`, effects/previews | third rectangular occupancy plus eight-direction rays and grouped flips | pre-implementation | Needs row-1-top 8 by 8 coordinates, legal bracketing scans, exact previews, and ordered grouped flip effects. |

## Local implementations compared

| Aspect | `three_marks` | `column_four` | `directional_flip` | Same shape? | Notes |
|---|---|---|---|---:|---|
| state shape | Fixed `[CellOccupancy; 9]` indexed by enum cells. | Fixed `[CellOccupancy; 42]` indexed by row/column cells. | Planned fixed `[CellOccupancy; 64]` or equivalent indexed by row/column cells. | yes | Fixed public occupancy repeats. |
| action shape | Direct `place/<cell>` target. | Direct column action; Rust derives landing cell. | Direct `place/<cell>` and `pass/forced`. | partial | Coordinate helper would not cover column gravity or pass. |
| validation | Empty-cell placement plus line terminal rules. | Non-full column plus gravity landing. | Empty target plus bracketed opposing ray in at least one direction. | partial | The repeated coordinate math is smaller than the actual validation logic. |
| transitions | Place one mark; no conversion. | Place one piece; no conversion. | Place one disc and convert many discs. | no | Flip policy must stay game-local. |
| diagnostics | Invalid cell, occupied, stale, wrong actor. | Unknown/full column plus stale/wrong actor. | Malformed/out-of-bounds, occupied, non-flipping, stale, wrong actor, terminal/pass cases. | partial | Diagnostic shape is behavior-owned by each game. |
| semantic effects | Placement, line, draw, terminal. | Column selected, piece dropped, line, draw, terminal. | Placement accepted, disc placed, grouped flips, pass, terminal. | no | Grouped flip effects are game-local. |
| visibility | Perfect information 3 by 3 grid. | Perfect information 7 by 6 grid with bottom-origin rows. | Perfect information 8 by 8 grid with top-origin rows and previews. | partial | Public grid repeats, but projection details differ. |
| UI pattern | Direct cell grid. | Column controls plus board cells. | Direct cell grid plus previews, flip animation, forced-pass control. | partial | UI cannot become helper authority. |
| bot use | Static line heuristics. | Gravity-aware immediate tactics. | Mobility/corner/frontier policy over legal moves. | no | Bot policy remains game-local. |
| replay/hash impact | Static cell order and line effects. | Row-major cell order, gravity, primary-line tie-break. | Ordered direction scans and grouped flip children. | partial | A helper could affect ordering; avoiding promotion avoids cross-game hash migration. |
| benchmark pressure | Tiny board; low pressure. | Directional line scans; modest pressure. | Eight-direction legal generation and previews; real hot path. | partial | Directional Flip should first measure local costs before extraction. |

## Similarities

- All three games use public rectangular occupancy with stable cell identifiers.
- `three_marks` and `directional_flip` both expose direct placement cells.
- `column_four` and `directional_flip` both need directional traversal rather than only static line constants.
- All three require deterministic ordering for action trees, views, traces, and replay/hash evidence.

## Differences

- `three_marks` has no dynamic ray walking; its eight winning lines are static.
- `column_four` rows count bottom to top because gravity is the core rule; Directional Flip rows count top to bottom per Gate 6 setup.
- `column_four` scans four forward win directions; Directional Flip scans eight directions from a candidate target and preserves direction-grouped flip order.
- Directional Flip previews and grouped semantic effects must match apply-time ray results; existing games do not prove that helper boundary.
- A helper broad enough to cover all three would likely expose dimensions, origins, parsing, direction sets, line lengths, ray iteration, and ordering flags; that is too much surface before the 8 by 8 game exists.

## Extraction decision

The recorded decision is defer-reject.

| Decision factor | Finding |
|---|---|
| repeated shape is real? | yes |
| helper can stay narrow and typed? | unclear before Directional Flip is implemented |
| helper belongs in `game-stdlib`? | no for Gate 6 start |
| would contaminate `engine-core`? | no, because no helper is promoted and `engine-core` is untouched |
| static-data behavior risk? | none from this decision; medium if a future helper becomes configurable behavior |
| replay/hash impact acceptable? | yes, because local implementation avoids cross-game trace/hash migration now |
| visibility/no-leak impact acceptable? | yes, no shared visibility surface is added |
| examples and anti-examples known? | yes |
| benchmarks support extraction? | no, Directional Flip local hot-path evidence does not exist yet |
| ADR required? | no |

Rationale:

- The third-use hard gate is satisfied by an explicit decision, not by promotion.
- `game-stdlib` is earned, not speculative. The existing games prove broad fixed-grid pressure, but not one small helper that fits all three without flags or policy leakage.
- Implementing Directional Flip locally first preserves deterministic ordering, previews, effects, diagnostics, and replay/hash behavior under the game crate that owns those rules.
- Promotion may be reconsidered after the as-built Directional Flip implementation shows duplicated behavior-free code that can be extracted without changing traces or public APIs.

## Rejected alternatives

| Alternative | Why rejected | Notes |
|---|---|---|
| Reuse an existing promoted primitive | No promoted primitive exists; `game-stdlib` currently has only `placeholder_version()`. | Not applicable. |
| Promote a rectangular coordinate helper before Directional Flip rules land | The exact useful API is still speculative and would require back-porting two finished games before local 8 by 8 evidence exists. | Rejected for Gate 6 start. |
| Promote line/ray plus pattern detection | This would include win/flip/legality pressure and risks behavior policy in a shared helper. | Rejected. |
| Escalate to ADR | No architecture, replay/hash, data-policy, visibility, or kernel-boundary change is being made. | Not required. |

## API sketch in prose only

No API is approved by this decision. A future reconsideration may propose a
behavior-free helper with the following narrow shape only after evidence exists.

| Aspect | Prose sketch |
|---|---|
| inputs | bounded dimensions, zero-based row/column indexes, optional stable display parser/formatter contract |
| outputs | checked coordinate values, deterministic row-major coordinate iteration, direction deltas, bounded ray iterator |
| error/diagnostic behavior | parse/check failures return typed local errors; games translate to viewer-safe diagnostics |
| determinism requirements | direction order and row-major order are explicit and test-covered |
| replay/hash requirements | helper use must preserve existing action/view/effect order or intentionally migrate traces |
| visibility requirements | coordinate math has no hidden state and exposes no private data |
| effect/log requirements | helper does not name or emit effects |
| bot-facing notes | bots may consume game-owned legal actions and views; no shared evaluator policy |
| non-goals | legality, occupancy, pass, flip/capture, win, bot strategy, UI layout, static-data behavior |
| good-fit examples | checked coordinate construction, row-major iteration, stepping one bounded ray |
| anti-examples | "find legal flips", "find winning line", "drop in column", "score mobility", "emit grouped effect" |

## Determinism and replay impact

| Impact | Required action | Tests/traces |
|---|---|---|
| action ordering | Keep local per game. | Directional Flip action-order tests in later tickets. |
| diagnostics | Keep local per game. | Directional Flip validation tests in later tickets. |
| semantic effects | Keep local per game. | Directional Flip grouped-effect tests and golden traces in later tickets. |
| trace hashes | not affected by this decision | Existing Three Marks and Column Four traces are not migrated. |
| serialization | no shared serialization surface added | Directional Flip serialization tests in later tickets. |
| seed/randomness | no impact | Bot seed behavior remains game-local. |

## Visibility and no-leak impact

| Surface | Impact | Required safeguard/test |
|---|---|---|
| public view | none | Game-local view/no-leak tests. |
| action tree | none | Game-local legal-action tests. |
| preview | none | Directional Flip preview/apply equality tests. |
| diagnostics | none | Game-local viewer-safe diagnostic tests. |
| effect log | none | Game-local effect tests. |
| DOM/test IDs/local storage/replay export | none | Browser no-leak smoke in later tickets. |
| bot explanations/candidate rankings | none | Bot explanation safety tests in later tickets. |
| dev inspector | none | Public/private payload checks in later tickets. |

## UI and effect impact

| Area | Impact | Required update |
|---|---|---|
| semantic effect names/payloads | none from this decision | Directional Flip owns grouped flip effects locally. |
| animation mapping | none from this decision | Browser maps Rust effects in later UI tickets. |
| Rust-generated previews | none from this decision | Directional Flip generates previews locally. |
| UI controls/action tree mapping | none from this decision | UI consumes Rust action tree only. |
| reduced-motion behavior | none from this decision | UI docs/smoke cover later. |
| accessibility labels/summaries | none from this decision | Rust/view and UI docs cover later. |

## Bot impact

| Bot level | Impact | Required update/tests |
|---|---|---|
| Level 0 random legal | none | Bot validates Rust legal actions in later tickets. |
| Level 1 baseline | not applicable | Gate 6 requires Level 0 and Level 2-lite only. |
| Level 2 authored policy | none | Directional Flip policy remains game-local. |
| Level 3 shallow deterministic search | not applicable | Out of scope and forbidden for public v1/v2. |

## Tests required

| Test | Required before promotion? | Required before reuse? | Notes |
|---|---:|---:|---|
| primitive unit tests | yes if reconsidered | yes if a helper exists | No helper is promoted now. |
| compatibility tests in each back-ported game | yes if reconsidered | yes if a helper exists | Back-port is not performed now. |
| named rule tests remain mapped | yes if reconsidered | yes if a helper exists | Later Directional Flip tests map to `DF-*` ids. |
| golden trace preservation/update notes | yes if reconsidered | yes if a helper exists | No existing traces change now. |
| property/invariant tests | yes if reconsidered | yes if a helper exists | Bounds/direction/ray tests would be required. |
| replay/hash tests | yes if reconsidered | yes if a helper exists | Required if shared ordering changes. |
| serialization tests | no for this decision | no for this decision | No shared serialization surface. |
| visibility/no-leak tests | if relevant | if relevant | No shared visibility surface. |
| bot tests | no | no | Bot policy is game-local. |
| benchmark tests | yes if reconsidered | yes if a helper exists | Need local Directional Flip baseline first. |

## Traces affected

| Trace | Game | Preserve or update? | Reason | Rule IDs/mechanics |
|---|---|---|---|---|
| existing golden traces | `three_marks`, `column_four` | preserve | No helper/back-port or behavior change. | fixed-grid/line pressure |
| future golden traces | `directional_flip` | preserve once created | Local implementation defines ordering before any future extraction. | `DF-REPLAY-001`, `DF-FLIP-004`, `DF-EFFECT-002` |

## Benchmarks affected

| Benchmark | Game(s) | Expected impact | Required threshold | Status |
|---|---|---|---:|---|
| none now | not applicable | no runtime code changed | none | not applicable |
| future local ray legality/apply benchmarks | `directional_flip` | establish evidence for any future helper | measured baseline first | required in later Gate 6 tickets |

## Back-port plan

No back-port happens under this decision.

If a future accepted task reopens promotion, it must:

- name exact call sites in `games/three_marks/src/` and `games/column_four/src/`;
- prove the helper preserves or intentionally migrates golden traces and stable summaries;
- avoid origin/ordering flags that make the helper a hidden rules language;
- add helper tests, examples, anti-examples, and benchmarks before any game consumes it.

## Examples

Good fits for a future helper:

- construct an in-bounds coordinate from row and column indexes;
- iterate all coordinates in row-major order for one fixed rectangle;
- step a coordinate by a named direction until the next step leaves bounds;
- format or parse `rNcM` only when a game explicitly adopts that contract.

## Anti-examples

Not a fit:

- decide whether a placement is legal;
- find bracketed discs to flip;
- choose which discs convert ownership;
- find a terminal winning line;
- decide forced-pass availability;
- rank bot moves by mobility or corners;
- emit semantic effects or UI labels;
- read static data as behavior.

## ADR need

ADR required? no

Reason:

- This decision changes no architecture, replay/hash semantics, data policy,
  kernel boundary, browser authority, bot policy class, or public/private
  content policy. It keeps the behavior local and records a next review trigger.

## Review checklist

- Third-game hard gate is satisfied by the defer-reject decision.
- Repeated shape was compared across actual official games and the planned Gate 6 shape.
- No game noun enters `engine-core`.
- No helper is added to `game-stdlib` in this ticket.
- No untyped behavior language is created.
- Static data remains typed content/parameters/metadata/fixtures/traces/reports only.
- Existing games are not back-ported because promotion was rejected/deferred.
- Golden traces are preserved.
- Replay/hash and serialization impacts are recorded.
- Visibility/no-leak impacts are covered.
- UI/effect and bot impacts are covered.
- Benchmarks are not changed; future benchmark evidence is required before reconsideration.
- Examples and anti-examples are documented.
- ADR need is explicit.

## Atlas field receipt

| Required field | Value |
|---|---|
| Mechanic shape | bounded rectangular coordinates, deterministic row/column cell ids, row-major iteration, directional offsets, bounded line/ray traversal |
| Status | rejected/deferred with rationale |
| Games exerting pressure | `three_marks`, `column_four`, `directional_flip` |
| Relevant files/docs | `games/three_marks/src/ids.rs`, `games/three_marks/src/rules.rs`, `games/column_four/src/ids.rs`, `games/column_four/src/rules.rs`, `specs/gate-6-directional-flip.md`, this ledger |
| What is repeated | public rectangular occupancy, stable cell ids, deterministic ordering, and Rust-owned spatial traversal |
| What differs | static 3 by 3 lines, bottom-origin gravity columns, and top-origin eight-direction flip rays |
| Why local duplication is now risky or acceptable | duplication is real, but local code is safer until Directional Flip proves the exact behavior-free helper boundary |
| Decision | defer-reject |
| Why not engine-core | mechanic nouns and spatial concepts remain outside the generic kernel |
| Why game-stdlib is or is not appropriate | not appropriate yet because the useful API would be speculative and likely flag-heavy before local 8 by 8 evidence exists |
| Data/Rust boundary impact | none; Rust game crates keep behavior |
| Replay/hash impact | none now; no trace or hash migration |
| Visibility impact | none; no shared public/private surface |
| Bot impact | none; bot policies remain game-local |
| UI/effect impact | none; previews/effects remain game-local |
| Tests required | none for this doc decision beyond doc-link/presence; future helper would require unit/property/back-port/replay tests |
| Benchmarks required | none now; future helper requires measured Directional Flip local baseline first |
| Back-port plan | no back-port now; future accepted promotion must name call sites and preserve traces |
| Examples | checked coordinate, row-major iteration, bounded ray step |
| Anti-examples | legal flips, winning lines, forced pass, bot mobility, semantic effects |
| Agent misuse risks | prematurely extracting a board engine, using flags as behavior, migrating hashes without notes, putting mechanic nouns into `engine-core` |
| Review owner/date | Rulepath implementer / Codex, 2026-06-07 |

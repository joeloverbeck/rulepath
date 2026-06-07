# Primitive Pressure Ledger: rectangular board-space coordinates

Candidate name: `rectangular-board-space`

Status: promoted primitive

Last updated: 2026-06-07

Prepared by: `Codex`

## Hard gate

Gate 7 reopens the Gate 6 deferral for bounded rectangular board-space helpers.
The hard-gate decision is:

Decision: promote

Promote a minimal, rule-agnostic board-space primitive to `game-stdlib` in
GAT7DRALITCOM-003. The promoted surface is limited to public rectangular
dimensions, coordinate values, bounds checks, deterministic row-major iteration,
signed offsets, stable `rNcM` parsing/formatting, and generic parity helpers.

No helper is added to `engine-core`. No draughts movement, capture, promotion,
occupancy, legality, bot, UI, WASM, static-data behavior, or semantic-effect
policy is promoted.

## Mechanic shape

Repeated shape considered:

- bounded rectangular position identity;
- stable row/column coordinate formatting and parsing;
- deterministic row-major coordinate iteration;
- signed offset arithmetic over a bounded rectangle;
- generic parity checks over row and column values.

This is behavior-free board-space math. It does not include occupancy, playable
square policy, move generation, captures, promotion, win detection, gravity,
flips, forced continuation, diagnostics, semantic effects, bot heuristics, UI
labels, WASM payload policy, or behavior encoded in static data.

## Games exerting pressure

| Game | Roadmap stage/gate | Local implementation area | Pressure type | Status at time of review | Notes |
|---|---:|---|---|---|---|
| `three_marks` | Gate 4 | `games/three_marks/src/ids.rs`, `rules.rs`, `visibility.rs`, `ui.rs` | first fixed rectangular board and stable public cell ids | implemented | Uses nine stable cells and deterministic presentation/order. |
| `column_four` | Gate 5 | `games/column_four/src/ids.rs`, `rules.rs`, `state.rs`, `visibility.rs` | second rectangular board with stable row/column cells | implemented | Uses rows, columns, and deterministic board projection; gravity remains game-local. |
| `directional_flip` | Gate 6 | `games/directional_flip/src/ids.rs`, `rules.rs`, `visibility.rs`, `ui.rs` | third rectangular board with direct cells and offsets | implemented | Gate 6 deferred broad extraction but named the next official spatial game as the reopen trigger. |
| `draughts_lite` | Gate 7 | planned `games/draughts_lite/src/ids.rs`, `state.rs`, `rules.rs`, `visibility.rs`, `ui.rs` | fourth rectangular board with direct origin/landing cells and offsets | pre-implementation | Needs 8 by 8 `rNcM` coordinates, playable parity, diagonal offsets, row-major ordering, and no trace/hash migration of earlier games. |

## Local implementations compared

| Aspect | Existing games | `draughts_lite` pressure | Same shape? | Notes |
|---|---|---|---:|---|
| state shape | Fixed rectangular public boards with stable coordinate or cell identity. | Fixed 8 by 8 public board with pieces on playable cells. | yes | Occupancy itself remains game-local. |
| action shape | Direct cell placement, column selection, forced pass, or direct target cells depending on game. | Compound origin and landing path segments. | partial | The helper must not encode action segment meaning or origin/order policy. |
| validation | Game-owned placement, gravity, line, or flip validation. | Game-owned movement, mandatory capture, continuation, and promotion. | no | Validation remains in `games/draughts_lite`. |
| coordinate formatting | Stable public cell IDs recur, with `rNcM` now repeated across larger board games. | Needs stable `rNcM` parse/format for docs, traces, UI, and effects. | yes | This is the narrow promoted shape. |
| coordinate arithmetic | Directional Flip and Draughts Lite both need bounded offsets. | Needs diagonal one-step and two-step offsets. | yes | Offset arithmetic is behavior-free when it only returns checked coordinates. |
| deterministic ordering | Existing views, actions, traces, and effects depend on stable order. | Legal origins and destinations must be emitted deterministically. | yes | Row-major iteration can be shared. |
| playable-square parity | Not shared by every board game. | Draughts uses dark-square parity. | partial | Generic parity is allowed; dark/playable policy is game-local. |
| replay/hash impact | Existing games already have hashes/traces. | New helper must not force existing trace migration. | yes | No forced retrofit of prior games in this gate. |

## Similarities

- Four official games now use rectangular board-space identity.
- Stable public cell IDs and deterministic coordinate ordering recur.
- Bounds checks and offset stepping are repeated pressure but do not decide
  legality by themselves.
- The useful common helper can be smaller than the game rules that use it.

## Differences

- `three_marks` has static 3 by 3 cells and line groups.
- `column_four` has bottom-origin gravity and column-first actions.
- `directional_flip` has eight-direction bracketing and grouped flips.
- `draughts_lite` has origin/landing action paths, mandatory capture,
  same-piece continuation, and promotion-stop rules.
- These differences are rule policy and remain game-local.

## Extraction decision

The recorded decision is promote.

| Decision factor | Finding |
|---|---|
| repeated shape is real? | yes |
| helper can stay narrow and typed? | yes |
| helper belongs in `game-stdlib`? | yes |
| would contaminate `engine-core`? | no, because the helper is not in `engine-core` |
| static-data behavior risk? | low if the API stays geometry-only |
| replay/hash impact acceptable? | yes, because existing games are not forced to retrofit and Draughts Lite starts with the helper before traces exist |
| visibility/no-leak impact acceptable? | yes, public geometry contains no hidden information |
| examples and anti-examples known? | yes |
| benchmarks support extraction? | not needed for the decision; helper performance can be unit-tested and later measured through Draughts Lite benches |
| ADR required? | no |

Rationale:

- Gate 6 deferred broad coordinate/ray extraction because the useful API was
  still unclear. Gate 7 narrows the proposal to behavior-free board-space:
  coordinates, dimensions, row-major iteration, offsets, id parse/format, and
  generic parity.
- The helper removes repeated low-level coordinate handling without encoding any
  draughts rules.
- Starting Draughts Lite on the helper avoids trace/hash migration for the new
  game, and the retrofit policy avoids changing earlier official games during
  this gate.

## Rejected alternatives

| Alternative | Why rejected | Notes |
|---|---|---|
| Put board-space in `engine-core` | Spatial concepts are game/mechanic vocabulary, not kernel contract vocabulary. | Forbidden by FOUNDATIONS §3. |
| Promote occupancy or movement helpers | Occupancy, movement, capture, continuation, promotion, gravity, and flips are rule policy. | Keep in game crates. |
| Promote line/ray or pattern detection now | Existing line/flip/capture logic differs enough that a shared helper would invite flags or behavior policy. | Keep line/pattern rows deferred. |
| Force retrofit of `three_marks`, `column_four`, or `directional_flip` | It would risk trace/hash churn without being necessary for Gate 7. | Retrofit is explicitly not part of this gate. |
| Defer again | The narrower coordinate-only helper has enough repeated pressure and can be implemented without rule policy. | Rejected for Gate 7. |

## API sketch in prose only

Do not write implementation code here. GAT7DRALITCOM-003 owns the exact Rust API.

| Aspect | Prose sketch |
|---|---|
| inputs | rectangular dimensions, row and column values, stable `rNcM` strings, signed row/column offsets |
| outputs | checked coordinate values, bounds results, row-major coordinate iterator, formatted `rNcM` strings, optional generic parity values |
| error/diagnostic behavior | helper parse/check failures are typed and rule-agnostic; games translate them into viewer-safe diagnostics |
| determinism requirements | row-major iteration and formatting are stable and unit-tested |
| replay/hash requirements | helper use must not change existing game traces; Draughts Lite traces start with this contract |
| visibility requirements | board geometry is public and contains no hidden information |
| effect/log requirements | helper emits no semantic effects and names no effect payloads |
| bot-facing notes | bots consume game-owned legal actions and public views; no shared evaluator or heuristic policy |
| non-goals | occupancy, move generation, captures, promotion, win detection, gravity, flips, playable-square policy, UI, WASM, static-data behavior |
| good-fit examples | parse `r3c4`, format a coordinate, iterate an 8 by 8 board, offset one coordinate by `(1, -1)`, check row/column parity |
| anti-examples | find legal jumps, choose capture paths, decide dark-square playability, drop a piece in a column, find a winner, emit a capture effect |

## Determinism and replay impact

| Impact | Required action | Tests/traces |
|---|---|---|
| action ordering | Draughts Lite may use row-major coordinates as one input to game-owned legal ordering. | Draughts Lite action-tree tests in later tickets. |
| diagnostics | Games translate helper parse/bounds errors to their own diagnostic reason classes. | Draughts Lite validation tests in later tickets. |
| semantic effects | No shared effect names or payloads. | Draughts Lite effect tests in later tickets. |
| trace hashes | Existing games are preserved; Draughts Lite starts with the helper before golden traces exist. | `cargo test --workspace` in GAT7DRALITCOM-003 and later golden traces. |
| serialization | Helper itself owns no trace schema and no save schema. | Unit tests for parse/format and game serialization tests later. |
| seed/randomness | No impact. | Not applicable. |

## Visibility and no-leak impact

| Surface | Impact | Required safeguard/test |
|---|---|---|
| public view | public geometry only | Draughts Lite visibility tests later. |
| action tree | coordinate ids may appear, but legality remains game-owned | Action-tree tests later. |
| preview | none from helper | Game-owned preview/effect tests later. |
| diagnostics | helper errors must be translated by games | Viewer-safe diagnostic tests later. |
| effect log | none | Effect tests later. |
| DOM/test IDs/local storage/replay export | no hidden data introduced | Browser no-leak smoke later. |
| bot explanations/candidate rankings | no heuristic data introduced | Bot tests later. |
| dev inspector | no internal-only state introduced | Public/private payload checks later. |

## UI and effect impact

| Area | Impact | Required update |
|---|---|---|
| semantic effect names/payloads | none | Draughts Lite owns effects locally. |
| animation mapping | none | Browser maps Rust effects in later UI tickets. |
| Rust-generated previews | none | Draughts Lite generates path guidance locally. |
| UI controls/action tree mapping | stable coordinate ids may be easier to render | UI still consumes Rust action trees only. |
| reduced-motion behavior | none | UI docs/smoke cover later. |
| accessibility labels/summaries | stable ids support labels, but labels remain game/UI-owned | A11y smoke covers later. |

## Bot impact

| Bot level | Impact | Required update/tests |
|---|---|---|
| Level 0 random legal | none beyond consuming game-owned legal paths | Bot legality tests later. |
| Level 1 baseline | none beyond public coordinate features available in game state | Bot policy tests later. |
| Level 2 authored policy | not applicable for Gate 7 | Out of scope. |
| Level 3 shallow deterministic search | not applicable | Forbidden for public v1/v2. |

## Tests required

| Test | Required before promotion? | Required before reuse? | Notes |
|---|---:|---:|---|
| primitive unit tests | yes | yes | Bounds, row-major iteration, `rNcM` round-trip, offsets, generic parity. |
| compatibility tests in each back-ported game | no | no | No back-port during Gate 7. |
| named rule tests remain mapped | yes for Draughts Lite | yes | Later rule tests map to `DL-*` IDs. |
| golden trace preservation/update notes | yes | yes | Existing games preserve traces; Draughts Lite traces start after helper adoption. |
| property/invariant tests | yes if useful | yes if useful | Offset/bounds invariants can be unit-tested. |
| replay/hash tests | yes through workspace and later replay checks | yes | No existing-game trace migration. |
| serialization tests | no for helper alone | no for helper alone | Game serialization tests later. |
| visibility/no-leak tests | if relevant | if relevant | Geometry is public; game visibility tests still required. |
| bot tests | no for helper | no for helper | Bots remain game-owned. |
| benchmark tests | no for helper decision | no for helper decision | Draughts Lite benches later cover hot paths. |

## Traces affected

| Trace | Game | Preserve or update? | Reason | Rule IDs/mechanics |
|---|---|---|---|---|
| existing golden traces | `three_marks`, `column_four`, `directional_flip` | preserve | No forced retrofit or behavior change. | fixed board-space pressure |
| future golden traces | `draughts_lite` | create under helper contract | Helper lands before Draughts Lite traces exist. | `DL-REPLAY-001`, `DL-ACTION-002` |

## Benchmarks affected

| Benchmark | Game(s) | Expected impact | Required threshold | Status |
|---|---|---|---:|---|
| none now | not applicable | no runtime game code changed in this ticket | none | not applicable |
| future legal-tree and bot benchmarks | `draughts_lite` | measure game-owned use of helper in hot paths | calibrated later | required in GAT7DRALITCOM-015 |

## Back-port plan

No forced back-port happens under this Gate 7 decision.

If a later accepted task back-ports earlier games, it must:

- name exact call sites in `games/three_marks`, `games/column_four`, and
  `games/directional_flip`;
- prove traces, hashes, views, action order, diagnostics, and effects are
  preserved or intentionally migrated;
- avoid origin/order flags that make the helper a hidden behavior language;
- update game docs and rule coverage where relevant.

## Examples

Good fits:

- construct an in-bounds coordinate from row and column values;
- iterate all coordinates in row-major order for a fixed rectangle;
- format and parse a stable `rNcM` coordinate string;
- step a coordinate by a signed offset and get `None` outside bounds;
- ask for generic row/column parity.

## Anti-examples

Not a fit:

- decide whether a draughts move is legal;
- find legal jumps or forced continuations;
- decide whether a square is playable for a specific variant;
- remove captured pieces or promote a man;
- drop a piece into a column;
- find bracketed discs to flip;
- find a terminal line or winner;
- rank bot moves;
- emit semantic effects or UI labels;
- read static data as behavior.

## ADR need

ADR required? no

Reason:

- The decision changes no architecture, replay/hash semantics, data policy,
  kernel boundary, browser authority, bot policy class, or public/private
  content policy. It promotes only a narrow `game-stdlib` helper and leaves all
  rules in Rust game crates.

## Review checklist

- Third-game hard gate is satisfied by the promote decision.
- Repeated shape was compared across actual official games.
- No game noun enters `engine-core`.
- Helper belongs in `game-stdlib`, not `engine-core`.
- No untyped behavior language is created.
- Static data remains typed content/parameters/metadata/fixtures/traces/reports only.
- Existing games are not force-retrofitted during Gate 7.
- Golden traces are preserved.
- Replay/hash and serialization impacts are recorded.
- Visibility/no-leak impacts are covered.
- UI/effect and bot impacts are covered.
- Benchmarks are not changed by this decision; Draughts Lite benchmarks land later.
- Examples and anti-examples are documented.
- ADR need is explicit.

## Atlas field receipt

| Required field | Value |
|---|---|
| Mechanic shape | bounded rectangular dimensions, coordinates, `rNcM` ids, row-major iteration, signed offsets, generic parity |
| Status | promoted primitive |
| Games exerting pressure | `three_marks`, `column_four`, `directional_flip`, `draughts_lite` |
| Relevant files/docs | `docs/MECHANIC-ATLAS.md`, `games/directional_flip/docs/PRIMITIVE-PRESSURE-LEDGER.md`, `games/draughts_lite/docs/RULES.md`, this ledger |
| What is repeated | public rectangular board-space identity, stable ids, bounds, offsets, and deterministic ordering |
| What differs | occupancy, gravity, flips, capture, promotion, forced continuation, terminal logic, UI, and bots |
| Why local duplication is now risky or acceptable | low-level coordinate duplication is now risky enough to extract; rule-policy duplication remains acceptable and required |
| Decision | promote |
| Why not engine-core | board-space is mechanic vocabulary outside the generic contract kernel |
| Why game-stdlib is or is not appropriate | appropriate because it is an earned, behavior-free helper shared by game crates |
| Data/Rust boundary impact | none; Rust game crates still own behavior and typed static data remains non-behavior |
| Replay/hash impact | no existing trace/hash migration; Draughts Lite starts from this helper |
| Visibility impact | public geometry only; no hidden data path |
| Bot impact | none; bots consume game-owned legal actions |
| UI/effect impact | none; UI and effects remain game-owned |
| Tests required | helper unit tests in GAT7DRALITCOM-003; workspace/boundary checks |
| Benchmarks required | none for this decision; Draughts Lite benchmarks later |
| Back-port plan | no forced retrofit during Gate 7 |
| Examples | checked coordinate, row-major iteration, `rNcM` round trip, signed offset, parity |
| Anti-examples | legal jumps, capture paths, promotion, occupancy, gravity, flips, win detection, semantic effects, UI labels, bot ranking |
| Agent misuse risks | promoting a board engine, adding draughts terms to `game-stdlib`, smuggling behavior flags, changing old traces unnecessarily, putting coordinates into `engine-core` |
| Review owner/date | Rulepath implementer / Codex, 2026-06-07 |

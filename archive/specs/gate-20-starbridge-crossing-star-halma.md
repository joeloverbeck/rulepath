# Gate 20 — Starbridge Crossing (Star Halma / Chinese Checkers family)

## 1. Header

### Spec metadata

| Field | Value |
|---|---|
| Spec ID | `GATE20-STARCROSS-STAR-HALMA` |
| Source provenance | Spec authored against `joeloverbeck/rulepath` at commit `b3e7efd`; freshness caveat in Assumptions A-1 |
| Stage | Public scaling phase, next topology/path proof |
| Roadmap gate | Gate 20 — Star Halma / Chinese Checkers family |
| Status | `Done` |
| Date | 2026-06-27 |
| Owner | Rulepath maintainers |
| Authority order | `docs/README.md` authority order: `FOUNDATIONS.md` → `ARCHITECTURE.md` → `ENGINE-GAME-DATA-BOUNDARY.md` → area docs → `ROADMAP.md`; accepted ADRs supersede only explicitly named sections. |
| New spec path | `specs/gate-20-starbridge-crossing-star-halma.md` |
| Internal game id | `starbridge_crossing` |
| Public display name | **Starbridge Crossing** |
| Rules-family label | Star Halma / Chinese Checkers family |
| Implemented variant id | `starbridge_crossing_classic_star_v1` |
| Rules version | `starbridge-crossing-rules-v1` |
| Data / manifest version | `starbridge-crossing-data-v1` |
| Trace rules version | `starbridge-crossing-trace-v1` |
| Browser implementation required | yes |
| Official board fixture | 121-space six-pointed-star topology, 10 pegs per participating seat |
| Official seat declaration | variable seats; exact supported set `{2, 3, 4, 6}`; minimum `2`; maximum `6`; default `2` as the smallest complete public-race fixture; unsupported counts, including `1` and `5`, are Rust setup diagnostics |
| Official seat labels | `north`, `north_east`, `south_east`, `south`, `south_west`, `north_west`; active seats are deterministic subsets of this clockwise ring |
| Seat/home assignment | `2`: `north ↔ south`; `3`: every other point (`north`, `south_east`, `south_west`) targets the opposite points; `4`: two opposite pairs with one opposite pair unused; `6`: all points |
| Teams / partnerships | absent; individual competitive race only |
| Partnership variant | sourced note only; explicitly out of scope for Gate 20 |
| Public observer stance | entire board, active seat, legal public action tree, effects, history, finish order, and terminal explanation are public |
| Information model | perfect information; no hidden-information class; no per-seat private datum; ADR 0004 hidden-info replay/export taxonomy is not applicable except as a contrast note |
| Bot floor | L0 random-legal bot required |
| Bot ceiling scoped by this spec | L2 authored or L3 bounded deterministic search may be added only after `COMPETENT-PLAYER.md` and `BOT-STRATEGY-EVIDENCE-PACK.md`; MCTS, ISMCTS, Monte Carlo playout/search, ML, RL, and runtime LLM move selection are forbidden |
| Kernel stance | no `engine-core` board, space, peg, marble, hole, cell, coordinate, adjacency, jump, path, home, target, track, graph, node, or edge noun |
| Primitive stance | `game-stdlib::board_space` audited **not applicable**; graph/topology/path-jump third-use hard gate resolved inline as **defer/reject promotion**; topology is typed content, legality remains game-local Rust |
| Scaffolding stance | Gate 20 is the third `forward-v1` reuse-first audit user; add a `forward-v1` receipt for `starbridge_crossing` to `ci/scaffolding-audits.json` |
| Delivery posture | spec only; tickets are later via `/reassess-spec` then `/spec-to-tickets` |

---

## 2. Objective

Gate 20 implements **Starbridge Crossing**, Rulepath's neutral, IP-safe Star Halma / Chinese Checkers family game. The objective is to turn the ROADMAP Gate 20 row into an official public game that proves a large, public, perfect-information board surface: the 121-space six-pointed star, variable seats, long hop chains, Rust-owned topology/path legality, no hidden cards, no hidden hands, no teams, no card/meld/trick mechanics, replayable deterministic traces, benchmarks, and a polished browser renderer.

### Sequencing determination

This spec treats Gate 20 as the next gate of record because the exact-commit repository tracker records Gate 19, Gate 19.1, and Gate 19.2 as `Done`, with Gate 20 as the lowest non-`Done` active-epoch row. `docs/ROADMAP.md` names Gate 20 as the Star Halma / Chinese Checkers family gate and states its purpose as larger topology and board-surface scaling without hidden cards. `docs/MECHANIC-ATLAS.md` §10A records no open promotion debt at the Gate 19 closeout, so no separate pre-Gate-20 debt-closure spec blocks this gate.

### Product objective

Starbridge Crossing must demonstrate the next public scaling pressure:

- a large **all-public** board state where every peg and every legal move is visible to every viewer;
- a discontinuous variable-seat declaration (`{2, 3, 4, 6}`) with deterministic setup diagnostics;
- a non-rectangular topology that cannot be shoehorned into the promoted rectangular `board_space` primitive;
- Rust-owned single-step and multi-hop jump-chain action trees, including stop-anywhere jump paths;
- finish-order outcome projection for 3+ seats;
- large-board renderer performance and accessibility for the full 121-space fixture;
- an inline graph/topology/path-jump third-use hard-gate decision before Gate 21 can depend on topology pressure;
- a third `forward-v1` mechanical-scaffolding reuse-first audit receipt.

The shipped trick-taking lane, `game-stdlib::trick_taking` helpers, River Ledger betting/all-in exemplar, Meldfall Ledger rummy exemplar, and 8F/forward-v1 governance are **baseline evidence**, not missing work. Starbridge Crossing has no cards, no tricks, no follow suit, no trump, no bidding, no melds, no draw/discard, no private hands, and no showdown evaluator. It reuses only lawful non-card plumbing: seat helpers where applicable, action-tree framing, replay/hash/evidence conventions, WASM/catalog registration seams, and web presentation infrastructure.

---

## 3. Scope

### In scope

| Area | Gate 20 requirement |
|---|---|
| Neutral identity | Ship as **Starbridge Crossing** with game id `starbridge_crossing`; document Star Halma / Chinese Checkers only as the rules-family label in source/IP notes. |
| Board topology | Game-local typed content for the 121-space six-pointed star. Each space has a stable `StarSpaceId`, axial/cube coordinate metadata, UI projection metadata, home/target zone metadata, and neighbor-direction metadata. The data describes spaces; Rust owns legal behavior. |
| Seat counts | Official support for exactly `{2, 3, 4, 6}` seats; setup rejects unsupported counts with Rust diagnostics. |
| Components | 10 public pegs per participating seat, starting in that seat's home point. |
| Turn model | One active seat at a time in deterministic clockwise order, skipping seats already assigned a finish rank. |
| Out-of-turn handling | Only the active seat may act; wrong-seat, stale-token, or post-finish actions are Rust diagnostics. No out-of-turn or simultaneous-move variant is implemented. |
| Legal moves | A turn is exactly one move: either one step to an adjacent empty space, or a chain of one or more hops. A step and hop cannot be mixed in the same turn. |
| Step rule | A peg may move from its current space to any adjacent empty space. |
| Hop rule | A peg may hop over exactly one adjacent occupied space into the empty space immediately beyond in the same direction. Hopped pegs are not captured or removed. |
| Hop-chain rule | A hop chain may change direction after each landing. The player may stop at any legal landing after at least one hop. Rust enumerates the finite action tree and validates the accepted path. |
| Hop-chain cycle guard | A single turn may not revisit a landing space already present in the current hop path. This keeps the legal action tree finite and replay-stable. It is recorded as a Rulepath implementation resolution, not as a claim that every table rule uses the same guard. |
| Blocking / no legal move | If Rust finds no legal step and no legal hop for the active seat, it exposes one forced `pass_blocked` action that records the no-move condition and advances the turn. This is required for explicit blocked/no-move traces and simulation termination. |
| Capture | none; all pegs remain on board unless a future variant explicitly changes the rules through a new spec. |
| Teams/partnerships | absent; every seat is an individual competitor. |
| Target home | A seat's target is the opposite point from its starting home. |
| Finish rule | A seat receives the next finish rank when all 10 of its pegs occupy that seat's target home at the end of its accepted move. Finished seats are skipped thereafter; their pegs remain public occupancy. |
| Match terminal rule | The match ends when all but one active seat have finish ranks; the last unfinished seat receives the final rank. A full-rank terminal trace is required. |
| Turn-limit safeguard | Official variants include a deterministic `max_plies` option. Default: `2000` plies for public simulations and benchmarks unless a fixture intentionally sets a lower value. On limit, Rust records a `turn_limit` terminal with completed finish ranks plus deterministic unfinished-seat standings by progress vector and clockwise seat order. |
| Replay / serialization | All setup, actions, effects, views, finish ranks, diagnostics, and hashes are deterministic and versioned. |
| Visibility | perfect information; public observer and every seat viewer receive the same board facts. No seat-private payload exists. |
| Bot floor | L0 random legal over Rust legal action paths. |
| Optional bot ceiling | L2 authored policy or L3 bounded deterministic search only after strategy evidence; no MCTS/ISMCTS/Monte Carlo/ML/RL/runtime LLM. |
| Browser | React/SVG public board renderer with Rust-provided legal paths, labels, previews, effects, replay, keyboard navigation, and accessibility metadata. |
| Evidence | official docs, rule coverage, traces, no-leak confirming audit, benchmarks, fixture profile, scaffolding receipt, and mechanic atlas updates. |

### Out of scope

- The partnership Chinese Checkers variant is documented only as a sourced future option.
- Two-set-per-player variants, 15-piece two-player variants, hop-across-empty-distance variants, swap/no-permanent-block house rules, and “must leave own home” house rules are excluded from `starbridge_crossing_classic_star_v1` unless explicitly listed as fixture-only diagnostics.
- No alternative square Halma board is implemented.
- No out-of-turn, simultaneous, or negotiated-move variant is supported.
- No 5-seat variant is supported.
- No capture, safe-space, team scoring, dice/chance, card, trick, meld, discard, betting, side-pot, hidden hand, hidden commitment, or showdown behavior is implemented.
- No helper extraction is performed for graph/topology/path/jump legality in this gate.
- No foundation amendment is expected.

### Not allowed

- Do not add board/space/peg/graph/path/topology nouns or helpers to `engine-core`.
- Do not encode path legality, jump legality, blocking, finish detection, scoring, action eligibility, or bot tactics in static data.
- Do not let TypeScript compute adjacency, enumerate jumps, validate paths, choose bot moves, or decide terminal state.
- Do not broaden `game-stdlib::board_space` to fit this board.
- Do not reuse `game-stdlib::trick_taking` or rummy/meld helpers.
- Do not introduce YAML, DSL, selector tables, formulas, scripts, or behavior-bearing data.
- Do not introduce MCTS, ISMCTS, Monte Carlo playout/search, ML, RL, or runtime LLM move selection.
- Do not copy external rulebook prose, diagrams, board art, marble art, source code, or trade dress.

---

## 4. Deliverables

### New game crate

Create `games/starbridge_crossing/`:

```text
games/starbridge_crossing/Cargo.toml
games/starbridge_crossing/src/actions.rs
games/starbridge_crossing/src/bots.rs
games/starbridge_crossing/src/effects.rs
games/starbridge_crossing/src/ids.rs
games/starbridge_crossing/src/lib.rs
games/starbridge_crossing/src/replay_support.rs
games/starbridge_crossing/src/rules.rs
games/starbridge_crossing/src/setup.rs
games/starbridge_crossing/src/state.rs
games/starbridge_crossing/src/topology.rs
games/starbridge_crossing/src/ui.rs
games/starbridge_crossing/src/variants.rs
games/starbridge_crossing/src/visibility.rs
games/starbridge_crossing/data/manifest.toml
games/starbridge_crossing/data/variants.toml
games/starbridge_crossing/data/fixtures/starbridge_crossing_2p_standard.fixture.json
games/starbridge_crossing/data/fixtures/starbridge_crossing_3p_standard.fixture.json
games/starbridge_crossing/data/fixtures/starbridge_crossing_4p_standard.fixture.json
games/starbridge_crossing/data/fixtures/starbridge_crossing_6p_standard.fixture.json
games/starbridge_crossing/benches/starbridge_crossing.rs
games/starbridge_crossing/benches/thresholds.json
games/starbridge_crossing/tests/bots.rs
games/starbridge_crossing/tests/property.rs
games/starbridge_crossing/tests/replay.rs
games/starbridge_crossing/tests/rules.rs
games/starbridge_crossing/tests/serialization.rs
games/starbridge_crossing/tests/visibility.rs
games/starbridge_crossing/tests/golden_traces/*.trace.json
```

The `topology.rs` module is game-local. It may contain generated constants or typed static-content loaders for the 121-space shape, but it owns no generic kernel contract and exports no shared helper.

### Official game documents

Fill the current template set for `games/starbridge_crossing/docs/`:

```text
SOURCES.md
RULES.md
RULE-COVERAGE.md
MECHANICS.md
GAME-IMPLEMENTATION-ADMISSION.md
HOW-TO-PLAY.md
COMPETENT-PLAYER.md
BOT-STRATEGY-EVIDENCE-PACK.md
AI.md
UI.md
BENCHMARKS.md
GAME-EVIDENCE.md
PRIMITIVE-PRESSURE-LEDGER.md
PUBLIC-RELEASE-CHECKLIST.md
```

`BOT-STRATEGY-EVIDENCE-PACK.md` may be present with L2/L3 marked `not started` if the gate ships only L0. If an L2/L3 bot is admitted in the same gate, it must be filled before that bot is public-default eligible.

### Repository registration

Update the same seams used by Vow Tide, Blackglass Pact, and Meldfall Ledger:

- workspace and gate lists: `Cargo.toml`, `ci/games.json`;
- WASM/API: `crates/wasm-api/src/constants.rs`, `lib.rs`, `catalog.rs`, `games.rs`, `games/starbridge.rs` or `games/starbridge_crossing.rs` following existing file naming conventions;
- tools: `tools/simulate/src/main.rs`, and confirm `replay-check`, `fixture-check`, and `rule-coverage` are generic or add explicit registration if required;
- web app: catalog entry, public rules markdown (`HOW-TO-PLAY.md` copied to `apps/web/public/rules/starbridge_crossing.md` via `scripts/copy-player-rules.mjs`, guarded by `scripts/check-player-rules.mjs`), `apps/web/public/rules/manifest.json`, board renderer component, e2e smoke (`apps/web/e2e/starbridge-crossing.smoke.mjs`, appended to the `smoke:e2e` chain), `apps/web/README.md`, and any root README catalog references enforced by `scripts/check-catalog-docs.mjs`;
- governance: add the `starbridge_crossing` `forward-v1` receipt to `ci/scaffolding-audits.json` and pass `scripts/check-scaffolding-governance.mjs`.

### Documentation updates

- `specs/README.md`: add this spec path and flip Gate 20 from seed/unwritten `Not started` to `Planned` when the spec lands.
- `docs/SOURCES.md`: add Star Halma / Chinese Checkers source-notes summary and naming/IP receipt link.
- `docs/MECHANIC-ATLAS.md`: record the Gate 20 third-use graph/topology/path-jump hard-gate decision; record `board_space` not-applicable; keep §10A open-promotion debt empty because this spec rejects promotion.
- `docs/MECHANICAL-SCAFFOLDING-REGISTER.md`: add the Gate 20 forward-v1 receipt summary and any first-use scaffolding disposition if implementation invents behavior-free plumbing.
- `games/starbridge_crossing/docs/GAME-EVIDENCE.md`: cross-link the all-public no-leak audit, trace profile, benchmark receipts, bot status, and scaffolding receipt.

---

## 5. Work breakdown

These are **candidate AGENT-TASK items** for later `/spec-to-tickets`; they are not tickets yet.

| Order | Candidate task | Depends on | Required output |
|---:|---|---|---|
| 1 | `GAT20STARCROSS-001` — Source/IP/naming and variant pin | none | `SOURCES.md`, naming rationale, chosen Rulepath variant, supported seat set, out-of-scope variants, source-consulted dates |
| 2 | `GAT20STARCROSS-002` — Graph/topology/path-jump third-use hard gate | 1 | `PRIMITIVE-PRESSURE-LEDGER.md` entry comparing Frontier Control, Event Frontier, `board_space`, and Starbridge; decision = defer/reject promotion; `docs/MECHANIC-ATLAS.md` update plan |
| 3 | `GAT20STARCROSS-003` — `forward-v1` reuse-first scaffolding audit | 1 | C-01…C-10 audit, lawful-home reuse decisions, no graph-behavior scaffolding misclassification, register disposition, `ci/scaffolding-audits.json` planned receipt |
| 4 | `GAT20STARCROSS-004` — Crate scaffold and typed board topology content | 2, 3 | crate skeleton, 121 stable `StarSpaceId`s, axial/cube coordinates, home/target zones, neighbor metadata, variant manifest, no behavior in data |
| 5 | `GAT20STARCROSS-005` — Setup, seat validation, and deterministic state | 4 | supported `{2, 3, 4, 6}` setup, wrong-seat diagnostics, deterministic home assignment, 10 pegs per seat, public state snapshot |
| 6 | `GAT20STARCROSS-006` — Single-step legal moves and validation | 5 | Rust legal leaves for adjacent empty spaces, invalid diagnostics for occupied/off-board/non-adjacent/stale/wrong-seat actions, step effects |
| 7 | `GAT20STARCROSS-007` — Jump-chain action-tree enumeration | 6 | progressive action tree for multi-hop paths, change-direction support, stop-anywhere leaves, no repeated landing within one chain, deterministic ordering, jump effects |
| 8 | `GAT20STARCROSS-008` — Finish, rank, blocked pass, and turn-limit outcomes | 7 | reach-target finish detection, continuing finish-order model, forced no-move pass, turn-limit terminal projection, Rust-owned outcome explanations |
| 9 | `GAT20STARCROSS-009` — Replay, serialization, fixtures, traces, and all-public no-leak audit | 8 | Trace Schema v1 files, fixture profiles, serialization round trip, public observer/export parity, no hidden-information class receipt |
| 10 | `GAT20STARCROSS-010` — Bot floor and optional bot evidence gate | 8 | L0 random-legal bot, many-seed legality simulations, `AI.md`; optional L2/L3 blocked until competent-player + evidence pack passes |
| 11 | `GAT20STARCROSS-011` — Tool/WASM/catalog registration | 9, 10 | simulator/replay/fixture/rule-coverage/WASM registrations, game constants, public rules manifest, catalog icon |
| 12 | `GAT20STARCROSS-012` — Large-board renderer, previews, animation, and accessibility | 11 | React/SVG 121-space star board, Rust-generated previews, keyboard path construction, ARIA labels, target sizing, reduced motion, e2e smoke |
| 13 | `GAT20STARCROSS-013` — Benchmarks and evidence receipts | 12 | native and CI benchmark thresholds for setup, action-tree enumeration, jump chains, playout, replay/serialization, and renderer smoke metrics |
| 14 | `GAT20STARCROSS-014` — Closeout docs and status flip | all | docs updates, `specs/README.md` Done criteria references, public-release checklist, no foundation amendment unless explicitly justified |

Tasks 2 and 3 are hard gating prerequisites. No implementation-admission task may proceed silently past unresolved topology primitive pressure or missing `forward-v1` receipt planning.

---

## 6. Exit criteria

Gate 20 is `Done` only when every row below has committed evidence.

| Roadmap / contract obligation | Exit evidence |
|---|---|
| Official seat variants | Rust setup supports exactly `{2, 3, 4, 6}` and rejects unsupported counts with stable diagnostics; fixtures and simulations cover each supported count. |
| 121-space topology | `topology.rs` plus typed content expose exactly 121 stable spaces; tests prove neighbor symmetry, expected degree range, home/target zone sizes, opposite-home mapping, and deterministic ordering. |
| Move chains | Rule tests and traces cover legal single-step moves, legal one-hop moves, legal multi-hop chains, direction changes, stop-anywhere leaves, and illegal mixed step+jump attempts. |
| Blocked-path behavior | At least one fixture constructs a no-legal-move active seat and proves the Rust-owned forced `pass_blocked` action, effect, trace, and replay behavior. |
| Win conditions | Tests and terminal traces prove all-pegs-in-opposite-home finish assignment, continuing finish order, finished-seat skipping, final rank assignment, and turn-limit fallback. |
| Replay | Native replay checks reproduce action, state, effect, public view, and terminal hashes for the trace catalog. |
| Serialization | Public view, action tree, replay export/import, fixture, and WASM payload serialization round trips are stable and versioned. |
| Benchmarks | `games/starbridge_crossing/benches/thresholds.json` records stable operation names and variance-aware thresholds for setup, move generation, jump-chain enumeration, playout, replay, serialization, and any renderer-facing smoke metric the repository can measure. |
| Renderer performance | Browser e2e smoke proves the 121-space fixture renders, updates, animates grouped move/jump-chain effects, and settles without leaking rule decisions into TypeScript. Large-board UI budget is documented in `UI.md` and `BENCHMARKS.md`. |
| Accessibility | Keyboard path construction, focus model, labels, color+shape affordances, target sizing, reduced motion, and replay controls are covered by e2e smoke and `UI.md`. |
| Topology/path helper pressure | `PRIMITIVE-PRESSURE-LEDGER.md` and `docs/MECHANIC-ATLAS.md` record the third-use decision as defer/reject promotion; no `game-stdlib` graph/path helper and no promotion debt remain. |
| `board_space` audit | `game-stdlib::board_space` is explicitly `not applicable` for the non-rectangular star board; no `board_space` reuse is required or forced. |
| Perfect-information no-leak | `GAME-EVIDENCE.md` records the all-surfaces-public audit; public observer and every seat viewer receive the same public board facts; ADR 0004 hidden-info replay/export taxonomy is marked not applicable with rationale. |
| Forward-v1 scaffolding | `ci/scaffolding-audits.json` has `starbridge_crossing` with `coverage: "forward-v1"`; `scripts/check-scaffolding-governance.mjs` passes; register dispositions are recorded. |
| Official docs | Every template document listed in §4 is filled or has explicit `not applicable` / `not started` rows with rationale. |
| Catalog/public rules | Rules markdown is copied into `apps/web/public/rules/`, catalog docs and smoke lists include Starbridge Crossing, and `scripts/check-catalog-docs.mjs` passes. |
| Boundary checks | `bash scripts/boundary-check.sh` and relevant CI checks pass; no forbidden `engine-core` noun or static behavior language is introduced. |

---

## 7. Acceptance evidence

### Command suite

The final AGENT-TASK closeout should record exact output for the command suite below, with platform/toolchain versions where existing evidence templates require them.

```bash
cargo fmt --check
cargo test -p starbridge_crossing
cargo test -p starbridge_crossing --test rules
cargo test -p starbridge_crossing --test replay
cargo test -p starbridge_crossing --test serialization
cargo test -p starbridge_crossing --test visibility
cargo test -p starbridge_crossing --test bots
cargo test -p wasm-api
cargo run -p simulate -- --game starbridge_crossing --seat-count 2 --games 100 --start-seed 20
cargo run -p simulate -- --game starbridge_crossing --seat-count 3 --games 100 --start-seed 20
cargo run -p simulate -- --game starbridge_crossing --seat-count 4 --games 100 --start-seed 20
cargo run -p simulate -- --game starbridge_crossing --seat-count 6 --games 100 --start-seed 20
cargo run -p replay-check -- --game starbridge_crossing
cargo run -p fixture-check -- --game starbridge_crossing
cargo run -p rule-coverage -- --game starbridge_crossing
cargo bench -p starbridge_crossing
node scripts/check-ci-games.mjs
node scripts/check-catalog-docs.mjs
node scripts/check-scaffolding-governance.mjs
bash scripts/boundary-check.sh
npm --prefix apps/web run build
npm --prefix apps/web run smoke:e2e   # runs the hardcoded suite incl. apps/web/e2e/starbridge-crossing.smoke.mjs
```

`simulate` selects seat count with `--seat-count` and the start seed with `--start-seed`; there is no `--seats`, `--seed`, or `--bot` flag. The L0 random-legal bot is wired into the per-game `simulate` dispatch (as for Meldfall Ledger's `MeldfallL0Bot`), not chosen by a CLI flag. `smoke:e2e` is a fixed chain of `node apps/web/e2e/<game>.smoke.mjs` invocations with no `--game` filter, so Gate 20 adds `apps/web/e2e/starbridge-crossing.smoke.mjs` and appends it to that chain. If existing CLI syntax differs further, the implementation task must record the exact accepted command and avoid weakening the coverage target.

### Test taxonomy

| Evidence class | Required Gate 20 coverage |
|---|---|
| Unit tests | topology invariants, neighbor symmetry, opposite mapping, coordinate conversion, progress vector, action ordering |
| Rule tests | setup counts, step legality, jump legality, chain stop, cycle guard, blocked pass, finish assignment, terminal cutoff, diagnostics |
| Property tests | generated public states preserve one occupant per space, legal actions never land on occupied spaces, every accepted action validates through the same path, no chain repeats a landing, replay determinism over many seeds |
| Golden traces | see trace catalog below |
| Replay tests | command stream + seed + variant reproduce hashes and terminal summaries |
| Serialization tests | public view, action tree, effects, setup/options, replay export/import, fixture profiles |
| Visibility/no-leak tests | explicit all-public confirming audit; public observer and all seat viewers receive no private-only datum because none exists |
| Bot tests | L0 chooses only Rust legal paths over many seeds and seat counts; optional L2/L3 tests require evidence pack and latency budgets |
| UI smoke tests | 121-space renderer, legal move previews, jump-chain path building, keyboard operation, replay viewer, reduced motion, responsive layout |
| Benchmarks | setup, action tree, hop-chain enumeration, bot playout throughput, serialization/replay, renderer-facing smoke budget |
| Mechanic primitive evidence | topology hard-gate decision and no-promotion evidence; `board_space` not-applicable audit; no `game-stdlib` promotion debt |
| Scaffolding governance | `forward-v1` receipt, C-01…C-10 audit, register disposition, governance checker pass |

### Golden trace minimum set

| Trace id | Purpose | Required proof |
|---|---|---|
| `setup-2p-standard` | deterministic 2-seat setup | homes, targets, 20 pegs, public view |
| `setup-3p-standard` | deterministic 3-seat setup | alternating homes, targets, 30 pegs, public view |
| `setup-4p-standard` | deterministic 4-seat setup | two opposite pairs, unused pair, 40 pegs |
| `setup-6p-standard` | deterministic 6-seat setup | all homes, 60 pegs, largest official board fixture |
| `invalid-seat-count-5` | unsupported seat diagnostic | Rust rejects five seats |
| `single-step-move` | adjacent empty-space move | effect, public view update, hash |
| `invalid-step-occupied` | occupied destination diagnostic | safe diagnostic, no mutation |
| `invalid-step-nonadjacent` | non-adjacent destination diagnostic | safe diagnostic, no mutation |
| `one-hop-move` | hop over one occupied peg | hopped peg remains, landing empty becomes occupied |
| `multi-hop-change-direction` | long chain with direction change | progressive action tree, grouped jump effects |
| `jump-chain-stop-midway` | stop at legal intermediate landing | player may end chain after hop |
| `jump-chain-repeat-landing-rejected` | finite-chain guard | no repeated landing within same turn |
| `invalid-mixed-step-jump` | one move cannot mix step and hop | diagnostic |
| `blocked-forced-pass` | active seat has no legal move | Rust exposes only `pass_blocked` |
| `reach-home-first-finish` | first finish rank | all pegs in target home assigns rank 1 |
| `finish-order-continues` | 3+ seat ranking | finished seat skipped, remaining seats continue |
| `terminal-full-standings` | terminal rank assignment | all but one finished, last rank assigned |
| `turn-limit-cutoff` | deterministic safeguard | partial ranks and progress-vector standings |
| `public-observer-all-public` | no hidden-information class | public observer sees every board fact |
| `seat-viewer-parity-all-seats` | confirming no-leak pass | every seat view equals public board facts modulo viewer label affordances |
| `public-replay-export-import` | public replay round trip | no private class, stable hashes |
| `wasm-exported` | web bridge | exported state/action/effect surface matches native expectation |
| `bot-l0-action` | random-legal bot | selected path is legal, deterministic under seed |

### All-surfaces-public no-leak audit

Gate 20 has no hidden facts. The no-leak section must therefore be explicit rather than omitted:

| Surface | Gate 20 classification | Evidence required |
|---|---|---|
| public view | public | contains all spaces, occupants, active seat, finish ranks, terminal reason |
| seat view | public-equivalent | same board facts as public view; seat-local labels may highlight “you” but add no private facts |
| action tree | public | legal paths for the active seat only; no hidden causes exist |
| previews | public | Rust-provided legal path/landing previews only |
| effects | public | setup, move, jump-chain, blocked-pass, finish, terminal effects public |
| diagnostics | public-safe | no private state to leak; still stable and not misleading |
| DOM/test ids | public | no hidden fact encoded |
| replay/export | public | seed + command stream is safe for this perfect-information game; no ADR 0004 viewer-scoped redaction class applies |
| bots | public | L0 receives legal public view; any non-random bot receives only public facts |

### Benchmark expectations

Operation names should be stable and recorded in `BENCHMARKS.md` and `thresholds.json`:

- `setup_121_spaces_2p`, `setup_121_spaces_3p`, `setup_121_spaces_4p`, `setup_121_spaces_6p`;
- `legal_actions_start_2p`, `legal_actions_midgame_6p`, `legal_actions_dense_jump_fixture`;
- `jump_chain_enumeration_dense_public_board`;
- `apply_single_step`, `apply_multi_hop_chain_8_plus_landings`, `apply_blocked_pass`;
- `simulate_l0_2p_100_games`, `simulate_l0_6p_100_games`;
- `serialize_public_view_121_spaces`, `replay_full_trace_6p`, `wasm_public_view_bridge_121_spaces`;
- browser smoke metrics for first render, legal-preview update, grouped jump-chain animation settle, and keyboard focus traversal on the 121-space board.

The spec does not set exact numerical thresholds; implementation must measure native baselines, commit variance-aware CI floors, and keep failing thresholds visible rather than hiding large-board pressure.

### Evidence-fixture completion profile

`GAME-EVIDENCE.md` must classify artifacts under the evidence-fixture contract:

- command/replay traces: `replay-command-v1`;
- setup fixtures: `setup-evidence-v1`;
- topology and rule-domain fixtures: `domain-evidence-v1`;
- public browser exports: `viewer-scoped` profile with `visibility_class = public` and `not_applicable` rationale for seat-private export;
- benchmark reports: benchmark evidence profile with stable operation names;
- no hidden-information class: explicit `not_applicable` rows for hidden/private fields.

### Third-use primitive evidence

The primitive-pressure evidence must include:

1. `board_space` audit: rejected as not applicable because its promoted scope is rectangular dimensions, row-major iteration, bounds, signed offsets, stable `rNcM` parse/format, and parity. Starbridge needs a masked non-rectangular six-pointed star with home/target zones and hop-vector behavior.
2. Prior graph comparison: Frontier Control and Event Frontier use named site/edge maps and graph adjacency for movement/scoring pressure. Starbridge uses regular hex-like coordinates, opposite home triangles, and multi-hop legality. These are related topology pressures but not close enough for a behavior-free shared helper.
3. Decision: **defer/reject promotion**. No graph/path/jump helper is introduced. No prior game conformance is required. No §10A promotion debt is created.
4. Reopen trigger: Gate 21 Pachisi-family race must compare its track topology/capture/safety pressure against Frontier Control, Event Frontier, and Starbridge Crossing before any helper proposal.

---

## 8. FOUNDATIONS and boundary alignment

| Authority | Gate 20 alignment |
|---|---|
| `FOUNDATIONS.md` | Rust owns setup, legal actions, validation, transitions, effects, views, replay, serialization, terminal detection, and bot decisions. TypeScript presents only. Public product polish and deterministic correctness outrank speculative generality. |
| `ARCHITECTURE.md` | New game is a normal `games/*` crate with WASM/catalog/tool registration following existing sibling games. Effects drive UI animation; replay and hashes remain Rust-owned. |
| `ENGINE-GAME-DATA-BOUNDARY.md` | Board topology terms live in `games/starbridge_crossing`; static data is typed content only; no engine-core nouns or data-driven rules. |
| `OFFICIAL-GAME-CONTRACT.md` | Requirements-first source notes, original rules prose, rule coverage, UI exposure, trace set, evidence receipt, and release checklist are required before official status. |
| `MECHANIC-ATLAS.md` | Third-use graph/topology/path pressure is resolved inline; `board_space` is N/A; no helper promotion or debt. |
| `MULTI-SEAT-AND-SURFACE-CONTRACT.md` | Exact variable seat set, setup diagnostics, public observer stance, outcome breakdown, surface budget, and simulator summaries are explicit. Teams/partnerships absent. |
| `AI-BOTS.md` | L0 required; optional L2/L3 gated by competent-player/evidence pack; no forbidden bot techniques. |
| `UI-INTERACTION.md` | Rust-generated legal action trees and previews, progressive compound path construction, effect-driven animation, replay UI, and accessibility are mandatory. |
| `TESTING-REPLAY-BENCHMARKING.md` | Rule tests, golden traces, replay, serialization, no-leak audit, bots, benchmarks, and §12 primitive evidence are mapped. |
| `EVIDENCE-FIXTURE-CONTRACT.md` / ADR 0009 | Fixture/export/hash classes are explicit; no blanket trace regeneration; only bounded authorized artifact updates. |
| ADR 0004 | Not applicable to hidden-info export because Gate 20 is perfect-information. The spec still records why the public seed+command stream is safe for this game. |
| ADR 0007 | Gate 20 consumes the public scaling phase admission; it does not amend the roadmap order. |
| ADR 0008 / `MECHANICAL-SCAFFOLDING-REGISTER.md` | Gate 20 runs the standing `forward-v1` reuse-first audit and adds a machine receipt. This audit stays parallel to, and distinct from, the behavioral topology hard gate. |
| `IP-POLICY.md` | Neutral name, original prose/assets, source notes, no copied rules/diagrams/trade dress, and human public-release review required. |

### Perfect-information stance

Every board fact is public: topology, peg occupancy, active seat, legal actions for the active seat, effects, move history, finish ranks, terminal state, bot public explanations, and replay exports. There is no seat-private payload, no public/private board split, no redacted card/deck/commitment class, and no hidden-information no-leak matrix beyond the explicit all-public confirming audit.

### Topology hard-gate resolution

Decision: **defer/reject promotion; keep topology and path/jump legality game-local**.

Rationale:

- The existing promoted `board_space` primitive is rectangular and row-major; Starbridge is a masked six-point star with 121 stable holes and six-direction topology.
- Frontier Control and Event Frontier are prior graph/topology pressures, but they use named sites/edges and movement/connectivity policy. Starbridge uses geometric space IDs, home/target triangles, occupancy, and multi-hop path legality. The shared part is too small unless it becomes an anemic coordinate iterator, and a coordinate iterator would still need game-owned topology metadata and legality policy.
- The Non-Promotion List excludes graph/topology/adjacency/movement behavior from the mechanical-scaffolding lane. Any behavioral promotion must go through the mechanic atlas; this spec decides not to promote.
- Keeping the rule in game-local Rust preserves trace stability, replay clarity, diagnostics, and future comparison value for Gate 21.

No prior games require conformance. `docs/MECHANIC-ATLAS.md` §10A remains empty.

### Forward-v1 audit posture

Gate 20 is the third `forward-v1` user. The audit must document reuse of existing behavior-free scaffolding where it fits and must reject any attempt to classify board topology behavior as scaffolding. Expected dispositions:

| Register checkpoint | Gate 20 disposition |
|---|---|
| C-01 effect envelopes | reuse framing only; move/jump/finish meanings remain game behavior |
| C-02 canonical seat grammar | reuse `seat_0`…`seat_5` / stable viewer IDs where existing helpers fit |
| C-03 seat range / ring arithmetic | reuse `game-stdlib::seat` helpers where they fit; home/target assignment and finish skipping remain game-local |
| C-04 action-tree encoding/hash | reuse framing; legal leaves and jump-chain semantics remain game-local |
| C-05 stable-byte / replay hashes | reuse canonical byte infrastructure; no hash surface migration without ADR 0009 note |
| C-06 dev/test support | reuse only as test scaffolding; no production behavior edge |
| C-07 visibility/no-leak test geometry | reuse evidence shape; Gate 20 fills all-public rationale |
| C-08 catalog/static checks | reuse `ci/games.json`, catalog docs, and smoke check patterns |
| C-09 fixture/evidence profiles | reuse evidence profile contracts; topology fixtures are domain evidence, not rule data |
| C-10 Non-Promotion behavioral bundle | reaffirm graph/topology/path/jump/scoring/bot policy as game-local behavior |

Expected register-new result: none. If implementation invents new behavior-free large-board UI plumbing, register it as `candidate` or `local-only` with behavior exclusions and a Gate 21 review trigger. Do not register graph/topology/path legality as scaffolding.

### Stop-condition review

Implementation must stop and require an ADR or amended spec if it needs any of these:

- changing `engine-core` vocabulary or contracts;
- changing Trace Schema v1, replay/hash bytes, fixture profile semantics, or public export semantics beyond the bounded game evidence;
- adding a static behavior language, YAML/DSL, formulas, selectors, or rule procedures;
- promoting topology/path/jump legality or hidden-state policy to a shared helper;
- introducing a forbidden public bot technique;
- copying protected expression or trade dress;
- weakening no-leak, accessibility, benchmark, or replay requirements.

---

## 9. Forbidden changes

- No `engine-core` board, space, peg, marble, hole, cell, coordinate, adjacency, jump, path, home, target, track, graph, node, or edge nouns.
- No `engine-core` or `game-stdlib` helper for Starbridge legality unless a later accepted spec/ADR supersedes this decision.
- No broadening of `game-stdlib::board_space`.
- No path, jump, blocking, terminal, scoring, bot, or strategy behavior in static data.
- No TypeScript legality, adjacency, jump enumeration, path validation, blocked/no-move logic, finish detection, bot choice, or outcome math.
- No YAML, DSL, selector language, trigger table, formula language, or scriptable rule layer.
- No cards, decks, hands, tricks, follow-suit, trump, bids, melds, discards, side pots, private hands, hidden commitments, or showdown evaluators.
- No teams or partnerships in the official Gate 20 variant.
- No MCTS, ISMCTS, Monte Carlo playout/search, ML, RL, or runtime LLM move selection in public v1/v2.
- No copied rule prose, screenshots, scans, board diagrams, peg art, fonts, proprietary icons, trademark-forward presentation, or trade-dress mimicry.
- No blanket golden trace regeneration; every trace/hash change requires a named rule/effect/format migration note.
- No weakening or deleting tests to pass CI.
- No foundation amendment by implication; if an actual gap appears, stop and name the required ADR/foundation update.

---

## 10. Documentation updates required

| File / area | Required update |
|---|---|
| `specs/README.md` | Add `specs/gate-20-starbridge-crossing-star-halma.md`; flip Gate 20 from seed/unwritten `Not started` to `Planned` when this spec lands; later flip to `Done` only with evidence. |
| `docs/SOURCES.md` | Add Starbridge Crossing / Star Halma source summary, source classes, IP review note, and consulted dates. |
| `docs/MECHANIC-ATLAS.md` | Add Gate 20 graph/topology/path-jump third-use decision. Record `board_space` N/A. Record decision = defer/reject promotion, no helper, no `engine-core` noun, no §10A debt. Add Gate 21 reopen trigger. |
| `docs/MECHANICAL-SCAFFOLDING-REGISTER.md` | Add Gate 20 `forward-v1` receipt summary; document C-01…C-10 audit and any local-only/candidate first-use scaffolding dispositions. |
| `ci/scaffolding-audits.json` | Add `starbridge_crossing` with `coverage: "forward-v1"`, gate 20, evidence path, register disposition path, and no migration debt. |
| `ci/games.json` | Add `starbridge_crossing` with `id`, a single representative `sim_flags` config (e.g. `--seat-count 6` for the largest fixture, plus an action cap consistent with siblings), and `e2e: "starbridge-crossing.smoke.mjs"`. The per-seat `{2, 3, 4, 6}` sweep lives in acceptance evidence, not the single CI sim lane. |
| `games/starbridge_crossing/docs/*` | Fill all official-game templates; use explicit `not applicable` rows for hidden information, ADR 0004, teams, partnerships, cards, tricks, melds, randomness if any. |
| `games/starbridge_crossing/data/*` | Add manifest and variants with typed content/parameters only; no behavior language. |
| `apps/web/README.md` | Add Starbridge Crossing to intro catalog list, Shell Surface renderer list, and Smoke Layers `smoke:e2e` list. |
| `apps/web/public/rules/manifest.json` and rules markdown | Add public rules entry and original player-facing rules prose. |
| `docs/ROADMAP.md` | No required foundation/roadmap amendment. Gate 20 already exists; closeout may add evidence links only if repository convention allows. |
| Foundation docs | None expected. Documentation updates only; any actual foundation gap must be flagged as an ADR/foundation amendment, not smuggled into this spec. |

---

## 11. Sequencing

Predecessors are closed at the target commit: Gate 19, Gate 19.1, and Gate 19.2 are `Done`; the mechanic atlas open-promotion debt register is empty; Gate 20 is the lowest non-`Done` active-epoch unit. Gate 20 is therefore admitted as the next new-game spec, not as a maintenance detour and not as a pre-interlock spec.

Gate 20 is also:

- the first large-topology, perfect-information N-seat board game of the public scaling phase;
- the third official graph/topology/path pressure after Frontier Control and Event Frontier;
- the third `forward-v1` mechanical-scaffolding audit user after Blackglass Pact and Meldfall Ledger;
- the topology/path evidence point that Gate 21 Pachisi-family race must review before it proceeds.

Admission rule for implementation: no code task may be accepted until the topology hard-gate ledger item and the forward-v1 audit item are present in the work breakdown and no forbidden boundary change is required.

Successor note: Gate 21 will add track topology, deterministic chance, capture, and safety semantics. Gate 20 must leave topology pressure resolved as local Rust evidence so Gate 21 can compare against it rather than inheriting unresolved debt.

---

## 12. Assumptions

- `assumption:` The user-supplied commit `b3e7efd0cd8f2622a4548a62e93c4489cc398990` is the target of record; this spec does not independently verify current `main`.
- `assumption:` The uploaded manifest is path inventory only, not file content authority.
- `assumption:` No foundation amendment is expected; Gate 20 consumes the foundation set and performs documentation updates only.
- `assumption:` The deliverable is the spec only. Ticket decomposition happens later via `/reassess-spec` and `/spec-to-tickets`.
- `assumption:` The neutral name **Starbridge Crossing** is accepted as the public display name; `Chinese Checkers`, `Star Halma`, and `Halma` stay in source/IP notes as family labels.
- `assumption:` The official variant uses 10 pegs per participating seat for every supported seat count. Common variants with 15 pegs in two-player play are documented but out of scope.
- `assumption:` The official variant excludes special swap, anti-permanent-block, and forced-vacate house rules. The replay/simulation safeguard is the Rust-owned forced blocked pass plus deterministic ply limit.
- `assumption:` Finished seats are skipped and their pegs remain as public occupancy; if playtest evidence shows this creates unacceptable public UX for multi-seat finish-order continuation, a bounded follow-on spec may revisit finish continuation without changing Gate 20's perfect-information stance.
- `assumption:` The React/SVG renderer is sufficient for 121 spaces unless benchmark/e2e profiling proves otherwise. Canvas or WebGL would require a documented renderer-specific follow-on, not TypeScript legality.

---

## Appendix A — Rulepath variant pin and external research notes

### Chosen variant: `starbridge_crossing_classic_star_v1`

Source tags used below:

- `[SRC-WIKI]`: Wikipedia, “Chinese checkers,” overview source for family history, 121-hole star board, common seat counts, 10-piece homes, no-capture move rules, and partnership/variant notes.
- `[SRC-ACM]`: ACM Hong Kong Chapter Computer Chinese Checkers Competition rules, computational source for 121 positions, six-direction neighbor model, jump sequences, homes, targets, and numbered board representation.
- `[SRC-BELL]`: George I. Bell, “The Shortest Game of Chinese Checkers,” source for 121-hole board, 10 men, step/jump-chain rules, no capture, voluntary stop, and base-blocking ambiguity.
- `[SRC-ENV]`: ArXiv 2405.18733 Chinese Checkers environment, computational source for explicit turn limits, action-cycle constraints, and cube/axial masked-board modeling.
- `[SRC-HEX]`: Amit Patel, “Hexagonal Grids,” source for axial/cube coordinate tradeoffs and neighbor/distance implementation patterns.
- `[SRC-A11Y]`: W3C / accessibility sources listed below for grid-like keyboard focus, WCAG 2.2, target size, and SVG accessibility.

| Parameter | Pinned Rulepath value | Source/rationale |
|---|---|---|
| Board | 121-space six-pointed star, modeled as six-direction hex-like topology with masked non-board coordinates | `[SRC-WIKI]`, `[SRC-ACM]`, and `[SRC-BELL]` all support the 121-hole star; `[SRC-ACM]`, `[SRC-ENV]`, and `[SRC-HEX]` support a six-direction coordinate/neighbor model. |
| Seat counts | `{2, 3, 4, 6}` | `[SRC-WIKI]` records common 2/3/4/6 play and the 5-player exclusion; Gate 20 locks this exact discontinuous set. |
| Default seats | `2` | Rulepath UX default: smallest complete race fixture, with 3/4/6 still official and mandatory in fixtures, simulation, and benchmarks. |
| Pegs per seat | 10 | `[SRC-WIKI]` and `[SRC-BELL]` describe the classic 10-piece home; two-player 15-piece variants are noted but excluded. |
| Objective | Move all own pegs into the opposite home point | `[SRC-WIKI]`, `[SRC-ACM]`, and `[SRC-BELL]` describe the opposite-home objective. |
| Moves | one adjacent step or one multi-hop chain | `[SRC-WIKI]`, `[SRC-ACM]`, and `[SRC-BELL]` distinguish one-step movement and jump sequences. |
| Hop target | over one adjacent occupied space to the empty space immediately beyond in the same direction | `[SRC-WIKI]`, `[SRC-ACM]`, and `[SRC-BELL]`; long-distance “hop-across” variants are out of scope. |
| Capture | none | `[SRC-WIKI]` and `[SRC-BELL]`; hopped pieces remain on board. |
| Hop chain | may continue and may change direction; player may stop at any legal landing | `[SRC-WIKI]` and `[SRC-BELL]` support voluntary continuation/stop; `[SRC-ACM]` supplies a computational jump-sequence model. |
| Chain guard | no repeated landing in a single turn | Rulepath deterministic finite action-tree resolution, informed by `[SRC-ENV]` cycle constraints; documented as an implementation resolution. |
| Finish-order | continue until all but one seat finish; assign final rank to last unfinished seat | Rulepath multi-seat outcome contract; common first-winner rules are insufficient for §11 standings, so this is a documented Rulepath resolution. |
| Blocked no-move | forced `pass_blocked` action | Rulepath replay/simulation resolution for explicit blocked/no-move traces; `[SRC-BELL]` identifies base-blocking as a real ambiguity pressure. |
| Ply limit | default `2000` plies | Deterministic simulation/benchmark safeguard; `[SRC-ENV]` notes no natural turn limit in computational play and uses explicit limits. |
| Out-of-turn play | unsupported; wrong-seat/stale diagnostics only | Rulepath active-seat model and replay determinism; no simultaneous/out-of-turn variant is adopted. |
| Partnership variant | out of scope | `[SRC-WIKI]` notes partnership variants; Gate 20 declares teams absent and records partnership play as a future option only. |
| Square Halma ancestry | source note only | Record the Stern-Halma / Halma lineage in `SOURCES.md`; no square Halma board or rules are implemented. |

### Naming and IP rationale

The public display name **Starbridge Crossing** is an original Rulepath coinage. It evokes a race across a star-shaped network and the bridge-like hop chains without using source-game branding as the product name. `Chinese Checkers`, `Star Halma`, `Stern-Halma`, and `Halma` remain rules-family/source-history labels in `SOURCES.md`; they do not appear as the public game title. Human IP/public-release review remains required before external publication, and all rules/help/board art must be original.

### Source notes to carry into `SOURCES.md`

Consulted external sources, all to be summarized in original Rulepath prose rather than copied:

1. Wikipedia, “Chinese checkers,” consulted 2026-06-27: overview of Stern-Halma / Chinese Checkers history, 121-hole star board, 10 pieces, move/hop/no-capture rules, common seat counts and variants. URL: `https://en.wikipedia.org/wiki/Chinese_checkers`.
2. ACM Hong Kong Chapter Computer Chinese Checkers Competition rules, consulted 2026-06-27: 121 positions, six-direction neighbor model, sequence of jumps, home/target areas, computational board representation. URL: `https://i.cs.hku.hk/~clyip/ACM/2005/CCC/ccc.html`.
3. George I. Bell, “The Shortest Game of Chinese Checkers,” arXiv PDF, consulted 2026-06-27: 121-hole board, coordinate representation, 10 pieces, step/jump chain rules, no capture, voluntary stop, base-blocking discussion. URL: `https://arxiv.org/pdf/0803.1245`.
4. Sturtevant et al., Chinese Checkers endgame / UCT paper, consulted 2026-06-27: strategy/evaluation context around shortest paths, blocking, and endgames. URL: `https://www.cs.du.edu/~sturtevant/papers/UCT-endgame.pdf`.
5. ArXiv 2405.18733 Chinese Checkers environment paper, consulted 2026-06-27: computational environment with explicit turn limit, no-backtracking/cycle constraints, cube/axial coordinate masking. URL: `https://arxiv.org/pdf/2405.18733`.
6. Amit Patel, “Hexagonal Grids,” Red Blob Games, consulted 2026-06-27: axial/cube coordinate modeling, neighbor/distance algorithms, and map-storage tradeoffs for hex-like boards. URL: `https://www.redblobgames.com/grids/hexagons/`.
7. W3C WAI-ARIA Authoring Practices Grid Pattern, consulted 2026-06-27: keyboard/focus model for grid-like interactive surfaces. URL: `https://www.w3.org/WAI/ARIA/apg/patterns/grid/`.
8. W3C WCAG 2.2 and Understanding SC 2.5.8 Target Size Minimum, consulted 2026-06-27: accessibility baseline and target-size expectations. URLs: `https://www.w3.org/TR/WCAG22/`, `https://www.w3.org/WAI/WCAG22/Understanding/target-size-minimum.html`.
9. Inclusive Learning Design Handbook, “SVG and Accessibility,” consulted 2026-06-27: SVG title/description and keyboard-accessible alternatives. URL: `https://handbook.floeproject.org/approaches/svg-and-accessibility/`.

External open-source implementations may be consulted as comparison examples for graph/coordinate modeling, but they must never be treated as Rulepath target-repository evidence and no code may be copied.

### Core rules prose seed for `RULES.md`

Use original prose. This seed is not final player-facing text, but it pins the implementation contract:

- Starbridge Crossing is played on a public 121-space six-pointed star.
- Each participating seat owns 10 pegs in its starting point and races them to the opposite point.
- On a turn, the active seat chooses one of its pegs and makes exactly one move.
- A move is either one step to an adjacent empty space or a chain of hops.
- A hop jumps over one adjacent occupied space into the empty space immediately beyond it. The jumped peg stays in place.
- After a hop, the same peg may continue hopping from its new landing space. A chain may turn after each landing. The player may stop after any hop. A chain may not revisit a landing space already used in that same move.
- If the active seat has no legal step or hop, Rust supplies a forced blocked pass.
- A seat finishes when all 10 of its pegs occupy its target home at the end of its move.
- Finished seats keep their public pegs on the board and are skipped in turn order.
- The match ends when all but one seat have finished; the last unfinished seat receives the final rank. A deterministic turn-limit fallback may terminate simulations and benchmarks.

### Coordinate and topology model

Recommended game-local representation:

```rust
pub struct StarSpaceId(u16); // 0..120, stable manifest order only

pub struct StarCoord {
    pub q: i8,
    pub r: i8,
    pub s: i8, // q + r + s == 0
}

pub enum StarPoint { North, NorthEast, SouthEast, South, SouthWest, NorthWest }

pub struct StarSpace {
    pub id: StarSpaceId,
    pub coord: StarCoord,
    pub zone: StarZone,
    pub ui_anchor: StarUiAnchor,
    pub neighbors: [Option<StarSpaceId>; 6],
}
```

The manifest may store `id`, `q`, `r`, `s`, zone labels, UI anchors, and neighbor IDs as typed content. Rust still owns all legality: a step is valid only when the destination is a listed neighbor and empty; a hop is valid only when the adjacent midpoint is occupied and the beyond space exists and is empty; a finish is valid only when all pegs occupy the target home.

### Action-tree model

Use the Draughts Lite compound-action-tree lineage but keep Starbridge semantics local:

```text
move/<peg-id>/step/<dest-space>
move/<peg-id>/jump/<landing-1>/continue/<landing-2>/.../stop
pass_blocked
```

Action-tree ordering must be stable:

1. pegs in deterministic seat-local order;
2. step leaves before jump roots, unless sibling conventions require the opposite;
3. directions in canonical six-direction order;
4. jump continuations depth-first with cycle guard;
5. `stop` leaves available after each landing.

Every accepted path is revalidated by Rust against current state and freshness token. UI previews are Rust-provided summaries attached to action-tree nodes or preview calls; TypeScript never derives legal landings.

### Bot policy seed

- `L0_RANDOM_LEGAL`: required, picks one legal action path from Rust's action tree using deterministic seeded tie-breaks.
- `L1_RACE_BASELINE`: optional future candidate, prefer moves that reduce aggregate target distance, finish a peg, or avoid moving pegs out of target home; requires `AI.md` evidence.
- `L2_AUTHORED_BRIDGE_POLICY`: optional only after competent-player and evidence pack; candidate heuristics may value ladders, advancing rear pegs, preserving bridge opportunities, avoiding self-blocks, and occupying target home.
- `L3_BOUNDED_SEARCH`: permitted by repository bot law only because the game is perfect-information, but not required and not default. If attempted, it must use strict depth/time/node limits, deterministic evaluation, deterministic transposition behavior, fallback to L0/L1 when limits trigger, latency budgets, and public explanations.
- Forbidden always: MCTS, ISMCTS, Monte Carlo playout/search, ML, RL, or runtime LLM move selection.

### UI/accessibility seed

- Render the 121 spaces as an original abstract star board with no commercial trade-dress mimicry.
- Each space has an accessible label including stable seat-neutral coordinate label, occupancy, home/target zone, and legal-action state when applicable.
- Use color plus shape/pattern/label, not color alone, for up to six seats.
- Provide keyboard selection of peg → legal step/hop target → optional continuation → stop.
- Use a roving-focus or grid-like navigation model; do not require pointer-only path construction.
- Legal previews come from Rust and include the full proposed path, landings, jumped-over spaces, and resulting finish/blocked effects where known.
- Group multi-hop animation as one semantic turn with per-hop substeps, respecting reduced motion.
- Keep 121-space render and preview updates profiled. If SVG pressure appears, document it; do not move legality into the renderer.

## Outcome

Completed 2026-06-27. Gate 20 shipped Starbridge Crossing as an official
public, perfect-information board game with Rust-owned 121-space topology,
2/3/4/6-seat setup, step and hop-chain legality, finish-rank outcomes,
all-public replay/export evidence, L0 bot simulation, WASM/catalog/web
registration, a Starbridge web renderer and e2e smoke, benchmark receipts, and
the `forward-v1` scaffolding governance receipt.

Implementation commits: `79e70cd` through `8ae467a`, plus the capstone closeout
commit that archives this spec and ticket.

Deviations and repairs:

- The capstone found a `rule-coverage` receipt gap for the validator-visible
  `SC-END-*` and `SC-SCORE-*` aliases added during trailing docs. The closeout
  repaired `games/starbridge_crossing/docs/RULE-COVERAGE.md` with alias rows
  pointing to the existing finish/terminal evidence, then reran
  `cargo run -p rule-coverage -- --game starbridge_crossing` successfully.
- Human public name/IP and asset release review remains pending maintainer
  work before external public release; no human legal signoff was performed in
  this implementation session.

Verification completed on 2026-06-27:

- `cargo test -p starbridge_crossing`
- `cargo test -p wasm-api`
- `cargo run -p simulate -- --game starbridge_crossing --seat-count 2 --games 100 --start-seed 20`
- `cargo run -p simulate -- --game starbridge_crossing --seat-count 3 --games 100 --start-seed 20`
- `cargo run -p simulate -- --game starbridge_crossing --seat-count 4 --games 100 --start-seed 20`
- `cargo run -p simulate -- --game starbridge_crossing --seat-count 6 --games 100 --start-seed 20`
- `cargo run -p replay-check -- --game starbridge_crossing --all`
- `cargo run -p fixture-check -- --game starbridge_crossing`
- `cargo run -p rule-coverage -- --game starbridge_crossing`
- `cargo bench -p starbridge_crossing`
- `node scripts/check-ci-games.mjs`
- `node scripts/check-catalog-docs.mjs`
- `node scripts/check-scaffolding-governance.mjs`
- `node scripts/check-outcome-explanations.mjs`
- `node scripts/check-doc-links.mjs`
- `bash scripts/boundary-check.sh`
- `npm --prefix apps/web run build`
- `npm --prefix apps/web run smoke:e2e`
- `git diff --check`

Closeout status: no `engine-core` noun, TypeScript legality, hidden-information
leak, topology/path helper promotion, prior-game migration set, hash migration,
visibility migration, determinism migration, or open §10A promotion debt was
introduced.

# Gate 13 Frontier Control Asymmetric Area-Control Proof

**Status**: COMPLETED

| Field | Value |
|---|---|
| Spec ID | `gate-13-frontier-control-asymmetric-area-control-proof` |
| Roadmap stage | 13 |
| Roadmap build gate | Gate 13 — asymmetric area-control proof |
| Status | Done |
| Date | 2026-06-11 |
| Owner | Rulepath maintainers |
| Primary crate / internal game id | `frontier_control` |
| Public display name | `Frontier Control` unless IP review prefers another neutral original name |
| Browser implementation | Required |
| Authority order | `docs/FOUNDATIONS.md` → `docs/ARCHITECTURE.md` → `docs/ENGINE-GAME-DATA-BOUNDARY.md` → `docs/OFFICIAL-GAME-CONTRACT.md` → `docs/MECHANIC-ATLAS.md` → `docs/AI-BOTS.md` → `docs/UI-INTERACTION.md` → `docs/TESTING-REPLAY-BENCHMARKING.md` → `docs/ROADMAP.md` → accepted ADRs that explicitly supersede those documents → this spec |

Where this spec and a foundation document disagree, the foundation document wins.

> Reader orientation: this spec carries the canonical Rulepath section set: Objective, Scope, Deliverables, Work breakdown, Exit criteria, Acceptance evidence, FOUNDATIONS & boundary alignment, Forbidden changes, Documentation updates required, Sequencing, and Assumptions. Detailed proposed rules, state/effect/view sketches, bot policy, WASM/browser wiring, fixtures, traces, benchmarks, and IP/source guidance are preserved below the canonical sections under **Implementation reference**.

## Objective

Implement `frontier_control` as the Gate 13 browser proof for the ROADMAP §14 asymmetric area-control exit lines: **graph topology, control, asymmetry, faction-specific legal actions/scoring, per-faction UI, and per-faction bots** — with no faction nouns entering `engine-core`, each faction holding random and baseline bots, simulations producing useful metrics, and effect logs staying readable.

Gates 1–12 proved flat actions, boards, compound trees, hidden information, resources, commitment/reveal, betting, tricks, reaction windows, and cooperative environment automation. Every spatial game so far has lived on a rectangular coordinate board, and every competitive game has given both seats the *same* action vocabulary. This gate adds the two missing shapes before Gate 14's event-complexity capstone: a **graph map** — sites joined by edges, where adjacency, not coordinates, constrains legality and where score depends on network connectivity — and **true faction asymmetry**, in which the two seats command different unit types, draw legal actions from *disjoint* action sets, resolve contests by different rules, and score by different formulas, while remaining fair, explainable, and balanced.

The result is a small, original, deterministic, perfect-information, two-seat competitive browser game, `Frontier Control`, that proves:

- graph topology: a typed static site/edge map with adjacency-constrained movement legality and a connectivity-dependent scoring rule, computed and projected entirely from Rust;
- control: per-site occupation by faction units, deterministic contest (clash) resolution on entry, and fort/stake control states that drive scoring;
- asymmetry: two factions with disjoint action sets (`march`/`stake`/`muster` vs `patrol`/`reinforce`/`dismantle`), asymmetric clash rules, and faction-specific scoring formulas — all in Rust application logic;
- per-faction UI: faction-aware action panels, score-track explanations, and a supply-connectivity display driven only by Rust view data;
- per-faction bots: Level 0 random and Level 1 rule-informed bots **for each faction**, with distinct deterministic priority policies, viewer-safe explanations, and useful bot-vs-bot simulation metrics;
- multi-action turn budgets as a recorded second official use of the Gate 12 shape, compared and kept local.

This is not an engine-generalization gate. It is a small official game that keeps all graph, site, edge, faction, unit, stake, fort, clash, supply, and control nouns local to `games/frontier_control`, and that deliberately contains **no hidden information and no game-rule randomness**, so the gate's whole proof budget goes to graph topology and asymmetry.

## Scope

### In scope

- New official game crate `games/frontier_control` with typed Rust setup, state, actions, validation, application, contest resolution, round scoring, connectivity computation, effects, visibility projection, replay support, variants, UI metadata, and bots.
- Default two-seat variant `frontier_control_standard` (seven-site map) plus a second map `frontier_control_highlands` (different graph, starts, and round count) — two typed maps prove the map is content, not behavior.
- Original deterministic rules: two factions (the Garrison and the Prospectors) on a fixed site/edge graph; alternating turns with a two-action budget; faction-specific actions; deterministic asymmetric clash resolution; end-of-round faction-specific scoring (Garrison fort-holding points; Prospector supply-connected stake values); fixed round count; higher score wins with a deterministic incumbent tiebreak.
- Game-local typed nouns only: site IDs, edges, factions, guards, crews, stakes, forts, supply connectivity, action budget, round/turn markers, scores, and the winner outcome. These stay inside `games/frontier_control` and its docs/tests/UI projection.
- **Pre-implementation atlas work** (work item 1): the ledger reviews and new first-use rows listed under Deliverables — the `board_space` promoted-primitive applicability audit, the role-modified-action-effects second-use comparison, the multi-action-budget second-use comparison, the shared-outcome comparison review, the reaction-window review, and the deterministic-shuffle non-use note — all before implementation code is written.
- Perfect information end to end: every projection is identical for both seats and observers; golden traces carry the explicit `not_applicable` markers Trace Schema v1 §5 requires for hidden-information and stochastic surfaces.
- Viewer-safe action trees: the active faction's tree carries its faction-specific legal actions with remaining-budget metadata; the waiting faction's tree is empty with safe waiting metadata; `end_turn` is always present during the action phase, so a turn can never stall.
- Effect-log-driven animation: march/patrol movement, clash resolution, stake placement/dismantling, muster/reinforce, round scoring breakdowns, and the terminal result emitted by Rust and rendered by React with reduced-motion support and log copy that keeps the asymmetric story readable ("The Garrison patrol cleared the Quarry: one crew driven off").
- Level 0 random legal bots and Level 1 rule-informed bots for **both factions** (`FrontierGarrisonLevel1Bot`, `FrontierProspectorLevel1Bot`): distinct deterministic priority policies per faction, usable for human-vs-bot on either side and bot-vs-bot replay, with per-faction win-rate simulation metrics.
- Full official-game evidence: unit/rule/property/replay/serialization/visibility/bot tests, golden traces, simulations, fixture validation, rule coverage, benchmarks, per-game docs (thirteen, including `HOW-TO-PLAY.md` and `PRIMITIVE-PRESSURE-LEDGER.md`), WASM registration, browser board, E2E smoke, a11y checks, player-rules and outcome-explanation surfaces, and CI/tool registration.
- Documentation updates per the section below, including the `specs/README.md` index flip and the web-shell catalog closeout surfaces.

### Out of scope

- Hidden information of any kind: hidden units, fog of war, secret objectives, hidden faction powers, or face-down components. Both seats see everything; this keeps the gate's proof budget on graph/asymmetry and means no ADR 0004 export-taxonomy work and no `HIDDEN_INFO_GAMES` registration.
- Game-rule randomness: no shuffles, dice, or random events. Setup is fully determined by the chosen map variant. (Bot tie-break RNG under the declared bot seed is unaffected — that is bot infrastructure, not game-rule chance.)
- Three or more factions, three or more seats, team play, or solo play. Two factions prove asymmetry; faction-count generality adds nothing to the Gate 13 proof.
- Asymmetric **victory conditions** (factions winning by different terminal rules or at different times). Gate 13 proves faction-specific *scoring formulas* feeding one comparable score track; differing win conditions are Gate 14's asymmetric-victory proof (`event_frontier`).
- Event decks, periodic automation, environment phases, initiative/eligibility systems, or scenario scripting — Gate 14's proof. The only automation here is deterministic end-of-round scoring inside the turn-ending command's application, mirroring the Gate 12 mechanism at much smaller scale.
- Reaction windows, pending responses, or interrupts. Clash resolution is immediate and deterministic inside the moving faction's command; no seat ever responds to another seat's pending action. If design drift introduces one, stop and re-run the atlas reaction-window review.
- Rectangular boards, coordinates, gravity, lines, or anything that would re-enter `game-stdlib::board_space` scope. The map is a graph; the work-item-1 audit records `board_space` as not applicable (or stops the gate if the audit contradicts that).
- A generic graph/map/adjacency/pathfinding/control/faction/asymmetry helper in `engine-core` or `game-stdlib`. Graph topology and area control are first official uses and stay local; budgets and faction modifiers are at most second uses and stay local per the atlas rule.
- Difficulty scaling beyond the two named maps, campaign structure, timers, real-time pressure, or undo.
- Hosted multiplayer, accounts, matchmaking, server persistence, chat, or ranked play. Play is hotseat and human-vs-bot, local-first.
- MCTS/ISMCTS/Monte Carlo/ML/RL bot work, search beyond the Level 1 deterministic priority policies, or any Level 2/3 claim without the evidence-pack workflow.
- Ticket decomposition. The Work breakdown lists bounded candidate AGENT-TASKs only; ticket files are a later step.

### Not allowed

Carried from ROADMAP §14 and tightened for this gate:

- Private licensed content, DSL by stealth, or architecture claims beyond proven games (the ROADMAP Gates 12–14 shared prohibition).
- Trade-dress imitation of published area-control and asymmetric-faction games (Root, Risk, El Grande, Small World, Kemet, Blood Rage, A Game of Thrones: The Board Game, Twilight Struggle, or similar). Mechanics-level similarity is acceptable; names, labels, prose, art direction, and theme must be original — see the IP-risk register under Implementation reference for the named hazards (woodland-faction theming, "clearings" vocabulary, conquest-map trade dress, influence-cube presentation).
- `engine-core` nouns such as `faction`, `territory`, `graph`, `map`, `site`, `edge`, `adjacency`, `unit`, `guard`, `crew`, `stake`, `fort`, `clash`, `supply`, `control`, or `movement` beyond existing generic actor/viewer/action/effect/replay envelopes. `faction`, `territory`, `movement`, and `adjacency` are explicitly on the FOUNDATIONS §3 forbidden-noun list, and this is the first game built around all four concepts.
- Generic `game-stdlib` helpers such as `GraphMap`, `AdjacencyIndex`, `ControlTracker`, `FactionPowers`, `ContestResolver`, `ConnectivityScorer`, or `AsymmetricBot`. Graph and control shapes are first official uses; budgets and faction modifiers are second uses at most; the atlas hard gate fires at the third use, not now.
- Static data behavior: movement rules, clash outcomes, scoring formulas, conditions, selectors, triggers, or expressions in manifests, variants, or fixtures. Map data declares typed site IDs, labels, edges, start positions, fort flags, stake values, round counts, budgets, and caps only; what an action *does*, how clashes resolve, and how scores are computed are typed Rust.
- TypeScript legality, adjacency checking, connectivity computation, clash resolution, score computation, terminal detection, outcome explanation authority, replay authority, or bot policy. Supply connectivity shown in the UI must come from Rust view data, never from a TypeScript graph traversal — connectivity is score-bearing behavior.
- Bots bypassing the legal action API, reading anything beyond the public view and declared bot seed, or sharing a single "plays both factions" policy that hides faction strategy in untestable branches — each faction's Level 1 policy is its own documented, tested policy.
- Accidental trace/hash/schema migration. Any intentional migration needs explicit notes and accepted review.

## Deliverables

| Area | Required artifacts |
|---|---|
| Atlas / ledger (pre-implementation) | `games/frontier_control/docs/PRIMITIVE-PRESSURE-LEDGER.md` recording (a) the `game-stdlib::board_space` promoted-primitive applicability audit (expected: **not applicable** — a site/edge graph has no rectangular dimensions, row-major iteration, or `rNcM` coordinates; per OFFICIAL-GAME-CONTRACT §7 the audit or an accepted exception is mandatory because the primitive is promoted); (b) the role-modified-action-effects second-use comparison against `games/flood_watch/docs/PRIMITIVE-PRESSURE-LEDGER.md` (expected: related but distinct — Flood Watch roles modify magnitudes of a *shared* action set, Frontier Control factions hold *disjoint* action sets and asymmetric clash rules; record the comparison, keep both local per the atlas second-use rule); (c) the multi-action-turn-budget second-use comparison (expected: second official use of the Gate 12 shape — compare budget size, regeneration, `end_turn` no-stall, and waiting metadata; keep local; the third-use hard gate arms for Gate 14); (d) the shared-outcome-cooperative-terminal comparison review (expected: **not** a second use — Frontier Control has a per-faction competitive winner, not a shared outcome; the row stays armed); (e) the reaction-window review (expected: not reaction-capable — clash resolution is immediate inside the mover's command); (f) a deterministic-shuffle non-use note (no shuffle, no randomness; the row's fifth-use trigger is untouched); (g) first-use records for graph-map topology / adjacency-constrained legality / connectivity scoring, site control / deterministic contest resolution, and faction-asymmetric action sets and scoring; `docs/MECHANIC-ATLAS.md` §10/§10B updates for all reviewed/new rows; confirmation that §10A stays `_None_`. |
| Workspace and crate | Root `Cargo.toml` registration; `games/frontier_control/Cargo.toml`; source modules `src/actions.rs`, `src/bots.rs`, `src/effects.rs`, `src/ids.rs`, `src/lib.rs`, `src/replay_support.rs`, `src/rules.rs`, `src/setup.rs`, `src/state.rs`, `src/ui.rs`, `src/variants.rs`, `src/visibility.rs`. Mirror the `games/flood_watch` file-for-file shape unless a file is explicitly documented as not applicable. `flood_watch` is the closest template for budgets, waiting metadata, and scenario constants; `token_bazaar` for typed content/constants data; `draughts_lite` for movement-shaped action paths. |
| Static data | `games/frontier_control/data/manifest.toml`, `games/frontier_control/data/variants.toml`, `games/frontier_control/data/fixtures/frontier_control_standard.fixture.json`, `games/frontier_control/data/fixtures/frontier_control_highlands.fixture.json`. Static files contain typed metadata, site IDs and labels, edge lists (pairs of site IDs), fort flags, per-site stake values, start units, caps, budgets, round counts, and faction labels only; no behavior. Unknown and behavior-looking fields are rejected. |
| Benchmarks | `games/frontier_control/benches/frontier_control.rs`, `games/frontier_control/benches/thresholds.json`. The benchmark identity list under Implementation reference, including the connectivity-scoring hot path and full random playouts. Initial thresholds are non-blocking smoke floors plus a named calibration follow-up under ADR 0002/0003/0005 (multiple representative CI runs per ADR 0005). |
| Native tests | `games/frontier_control/tests/rules.rs`, `property.rs`, `replay.rs`, `serialization.rs`, `visibility.rs`, `bots.rs`. Coverage detailed under Acceptance evidence. |
| Golden traces | Under `games/frontier_control/tests/golden_traces/`: the seventeen traces listed under Acceptance evidence, each carrying the perfect-information / no-randomness `not_applicable` markers Trace Schema v1 §5 requires. |
| Per-game docs | Thirteen docs instantiated from `templates/*`: `games/frontier_control/docs/AI.md`, `BENCHMARKS.md`, `BOT-STRATEGY-EVIDENCE-PACK.md`, `COMPETENT-PLAYER.md`, `GAME-IMPLEMENTATION-ADMISSION.md`, `HOW-TO-PLAY.md`, `MECHANICS.md`, `PRIMITIVE-PRESSURE-LEDGER.md`, `PUBLIC-RELEASE-CHECKLIST.md`, `RULE-COVERAGE.md`, `RULES.md`, `SOURCES.md`, `UI.md`. `HOW-TO-PLAY.md` marks its "Hidden information and reveal timing" section explicitly **not applicable** (perfect information) so `scripts/check-player-rules.mjs` passes without `HIDDEN_INFO_GAMES` registration. |
| Tools | Register `frontier_control` in `tools/simulate`, `tools/replay-check`, `tools/fixture-check`, `tools/rule-coverage`, and `tools/bench-report` (if thresholds/reporting enumerate game IDs). `tools/seed-reducer` and `tools/trace-viewer` carry no per-game dispatch today (`flood_watch` is absent from both), so no registration is expected there — confirm this still holds at implementation time before adding any. Simulation output reports per-faction win counts and average scores — the ROADMAP "simulations produce useful metrics" line. |
| WASM/API | Register `frontier_control` in `crates/wasm-api/src/lib.rs` catalog, setup, action, bot, effect, view, replay/export/import paths. `get_view(match_id, viewer_seat)` returns output-equivalent projections for all viewers (perfect information); replay export carries the full public command/effect timeline with no redaction surface needed. |
| Browser | `apps/web/src/components/FrontierControlBoard.tsx`; GamePicker/catalog support through Rust metadata; `ActionControls` support for the per-faction budgeted action phase (faction-specific action grouping, remaining-budget display, waiting state) without TS legality; `EffectLog` / `effectFeedback.ts` entries for march, patrol, clash, stake, dismantle, muster, reinforce, round-scoring, and terminal effects; winner/decisive-cause entries in `apps/web/src/components/outcomeExplanationTemplates.ts` (faction-flavored score-comparison and tiebreak causes from Rust terminal data) and viewer-safe rationale mirrors in `apps/web/src/wasm/client.ts`; player-facing rules at `apps/web/public/rules/frontier_control.md` generated from `games/frontier_control/docs/HOW-TO-PLAY.md` via `scripts/copy-player-rules.mjs`; an SVG graph-map renderer (sites as nodes, edges as trails, per-faction unit and stake markers, fort badges, Rust-supplied supply-connectivity highlighting); shell reducer/client type coverage; safe dev panel output; replay import/export wiring; reduced-motion support; responsive, accessible board presentation. |
| Browser smoke | `apps/web/e2e/frontier-control.smoke.mjs` plus a11y checklist updates. Smoke must cover both factions' action panels (disjoint action sets visible), a march/patrol move, a clash animation, stake placement and supply-connectivity display, the round-scoring effect, a full game to the outcome-explanation surface for each faction winning, the bot turn for each faction, bot-vs-bot replay, replay step/export/import, and reduced motion. |
| CI | `.github/workflows/gate-1-game-smoke.yml` native smoke, replay, fixture, rule coverage, web build, and E2E registration; `.github/workflows/gate-2-benchmarks.yml` smoke and threshold registration. |
| Repository docs | `specs/README.md` Gate 13 row maintenance; `docs/MECHANIC-ATLAS.md` updates per the atlas deliverable; `progress.md` and root `README.md` after implementation; no ROADMAP progress edit. |

## Work breakdown

Bounded candidate AGENT-TASKs, in dependency order. Do not decompose these into ticket files as part of this spec.

| # | Candidate task | Depends on | Notes / forbidden drift |
|---:|---|---|---|
| 1 | Pre-implementation primitive-pressure decisions | — | Write `games/frontier_control/docs/PRIMITIVE-PRESSURE-LEDGER.md` with the seven items under Deliverables (board_space audit, role-modifier comparison, budget second use, shared-outcome comparison, reaction-window review, shuffle non-use, graph/control/asymmetry first-use records). Update atlas §10/§10B; confirm §10A stays `_None_`. **Blocks all implementation tasks.** If any review contradicts its expected outcome — especially if the board_space audit finds applicable scope or the role-modifier comparison concludes a third-use-equivalent pressure — stop: the ledger decision or hard gate fires before code is written. |
| 2 | Crate skeleton and workspace registration | 1 | Add `games/frontier_control` with IDs, both map variants, setup constants, manifest/variant parsers, fixture shells. No behavior in data: edges are ID pairs, forts are flags, stake values are integers; nothing in data says what movement, clashes, or scoring *do*. |
| 3 | State model, typed IDs, deterministic setup | 2 | Model the site graph (typed site IDs, adjacency from validated edge data), per-site occupancy (guards, crews, stake, fort flag), faction-to-seat assignment, round/turn markers, two-action budget, per-faction scores, terminal outcome, freshness token. Setup is fully deterministic from the map variant — no RNG anywhere in game rules. Validate map data: connected graph, valid edge endpoints, no duplicate sites/edges, start positions on existing sites. |
| 4 | Faction action trees, validation, application, clash resolution | 3 | Garrison tree: `patrol/<from>/<to>`, `reinforce/<fort>`, `dismantle/<site>`, `end_turn`. Prospector tree: `march/<from>/<to>`, `stake/<site>`, `muster`, `end_turn`. Each validated against faction, budget, adjacency, occupancy, caps, and stake/fort state; tree regenerates with remaining-budget metadata after each action; waiting faction's tree is empty with safe waiting metadata; `end_turn` always present (no stall). Asymmetric clash resolution in Rust application only: a guard entering a crewed site removes one crew and survives; a crew entering a guarded site removes one guard and is itself removed. |
| 5 | Round scoring, supply connectivity, and terminal | 4 | The command ending the second faction's turn triggers end-of-round scoring inside the same deterministic application (the Gate 12 automation mechanism at small scale): Garrison scores per held fort (≥1 guard, 0 crews); Prospectors score each staked site's value if a guard-free path connects it to the base camp (Rust graph traversal). After the final round, terminal: higher score wins; tie goes to the Garrison (stable rule ID). `RoundScored` carries the full per-faction breakdown for the UI and outcome surface. |
| 6 | Visibility and replay surfaces | 5 | Single public projection, output-equivalent for both seats and observers, including the Rust-computed supply-connectivity set. Effect filtering trivially public; stable summaries; action/effect/view hashes; replay export/import of the full public timeline; Trace Schema v1 `not_applicable` markers for hidden-info/stochastic/private-view surfaces. |
| 7 | Per-faction Level 0 and Level 1 bots | 5,6 | Level 0 random per faction over the legal tree. `FrontierGarrisonLevel1Bot`: defend threatened forts, clear stakes that feed Prospector scoring, patrol toward the highest-value threat (deterministic stable site-order tie-breaks). `FrontierProspectorLevel1Bot`: stake the highest-value eligible site, march toward the nearest high-value unguarded site (deterministic shortest-path with stable tie-break), muster when crews run low. Legal action API only; public view + bot seed only; viewer-safe per-faction explanations; bot-vs-bot simulation reports per-faction win rates. |
| 8 | Native tests and golden traces | 5,6,7 | Rule/property/replay/serialization/visibility/bot suite and the full golden trace set. Follow the failing-test protocol; never weaken tests to get green. |
| 9 | Benchmarks and thresholds | 8 | The benchmark identity list under Implementation reference. Smoke floors first, calibration follow-up named (multiple CI runs per ADR 0005). |
| 10 | Per-game documentation | 8,9 | Instantiate all thirteen docs. `RULES.md` carries stable rule IDs including scoring/terminal/tiebreak IDs for the outcome-explanation contract; `HOW-TO-PLAY.md` carries the required player-rules section set with hidden information marked not applicable; `COMPETENT-PLAYER.md`/`BENCHMARKS.md` record the per-faction balance evidence (Assumption A5). |
| 11 | WASM, tools, and CI registration | 8 | Register game ID across wasm-api, simulate, replay-check, fixture-check, rule-coverage, bench-report, and CI lanes (`seed-reducer`/`trace-viewer` carry no per-game dispatch and are not expected to need registration — confirm at implementation). Extend `scripts/boundary-check.sh`'s `mechanic_pattern` with `faction` and `territory` — the two FOUNDATIONS §3 forbidden kernel nouns this gate is built around (the pattern currently ends at `role|scenario`); evaluate `adjacency` and `movement` for substring/word-boundary false positives before including them, and exclude `graph` (too generic — legitimate in non-mechanic contexts). `engine-core` is clean of `faction`/`territory` today, so the extension must stay green on the existing tree. |
| 12 | React board and browser smoke | 11 | `FrontierControlBoard.tsx` SVG graph map, per-faction action panels, score tracks with breakdown display, budget indicator, supply-connectivity highlighting from Rust view data, clash/scoring animation from effects, outcome-explanation surface, player-rules copy with the not-applicable hidden-info marker, replay UI, E2E smoke, reduced motion, responsive layout. TS remains presentation-only: no adjacency, connectivity, clash, or score computation in the browser. |
| 13 | Repository documentation and final admission evidence | 10,11,12 | Spec index flip, atlas confirmation, progress/root README updates after implementation, catalog closeout surfaces, command transcript, unresolved issues. Do not edit ROADMAP as progress diary. |

## Exit criteria

Mapped row-for-row to ROADMAP §14 (Gate 13).

| ROADMAP §14 line | Gate 13 exit criterion |
|---|---|
| no faction nouns enter `engine-core` | `scripts/boundary-check.sh` passes with its `mechanic_pattern` extended to include `faction` and `territory`; kernel review confirms no graph/site/edge/unit/stake/fort/clash/supply/control vocabulary entered `engine-core`, and no graph/control/faction helper entered `game-stdlib`. The same seed-free determinism, action-tree, effect, and replay envelopes that served Gates 1–12 carry this game unchanged. |
| each faction has random and baseline bot | Level 0 random and Level 1 rule-informed bots exist **per faction**, with distinct documented priority policies, many-seed legality tests in both factions, determinism tests under declared inputs, viewer-safe explanations naming faction-appropriate reasons, and latency benchmarks. Human-vs-bot works with the human on either side; bot-vs-bot replay completes. |
| simulations produce useful metrics | `cargo run -p simulate -- --game frontier_control --games 1000` completes without illegal actions or invariant failures and reports per-faction win counts, average scores, average rounds, and tiebreak frequency; the balance band (Assumption A5) is evaluated from these metrics and recorded in `BENCHMARKS.md`/`COMPETENT-PLAYER.md`, with a constants retune before public polish if the band is missed. |
| effect logs stay readable | Every state change is a semantic effect with faction-aware log copy (move, clash, stake, dismantle, muster, reinforce, per-round scoring breakdown, terminal cause); the E2E smoke asserts the log renders readable entries for a clash, a round scoring, and both terminal outcomes; reduced motion preserves order and copy. |
| Not allowed: private licensed content | **Honored.** All names, labels, prose, and visuals are original and IP-reviewed per `docs/IP-POLICY.md`; `SOURCES.md` records the consulted-mechanics-only posture and the trade-dress avoidance register. |
| Not allowed: DSL by stealth | **Honored.** Map data is typed IDs, edges, flags, values, and counts; movement, clash, scoring, and terminal rules are typed Rust; unknown and behavior-looking fields are rejected by tests. |
| Not allowed: architecture claims beyond proven games | **Honored.** No graph/control/faction/asymmetry abstraction is promoted anywhere; first-use shapes are recorded `local-only`, second-use shapes are compared and kept local in the atlas. |

## Acceptance evidence

### Native rules, replay, visibility, and bot evidence

- `cargo test -p frontier_control` passes.
- Rule tests cover deterministic setup for both maps, adjacency-constrained move legality (every legal `march`/`patrol` follows an existing edge; no move ever crosses a non-edge), faction-action separation (no Garrison action ever appears in a Prospector tree and vice versa), budget tracking and exhaustion, both clash directions (guard-enters-crews and crew-enters-guards) including multi-unit sites and caps, stake placement/dismantle legality, muster/reinforce caps and fort-holding conditions, supply-connectivity scoring (connected stakes score their value; a stake cut off by a guard on every path scores zero; reconnecting restores it), per-round scoring for both factions, the final-round terminal, the score comparison, and the Garrison tiebreak.
- Diagnostics tests cover stale freshness tokens, wrong-seat/wrong-faction actions, out-of-budget submissions, non-adjacent moves, staking a guarded or already-staked site, dismantling where no stake exists, reinforcing a lost fort, mustering at cap, and post-terminal submissions — all with viewer-safe messages.
- Property tests cover many bot-seeded legal action sequences asserting: every move follows an edge; unit counts stay within caps and never go negative; total units change only through documented clash/muster/reinforce effects; stakes exist only on sites that were legally staked; the action-phase tree always contains `end_turn` (no stall); round scoring runs exactly once per round; terminal occurs exactly at the configured round bound; the winner matches the score comparison plus tiebreak; and no panics.
- Replay tests prove the same seats, map variant, and command stream reproduce state hashes, effect hashes, action-tree hashes, view hashes, scoring breakdowns, and the terminal outcome. (There is no game-rule seed; determinism is over the command stream alone, and the trace records that explicitly.)
- Serialization tests prove stable summaries and unknown-field rejection for manifest, both map variants, fixtures, replay export, and internal trace helpers — including rejection of behavior-looking fields in map data (`when`, `condition`, `trigger`, etc.).
- Visibility tests prove all-viewer output equivalence: `get_view` for `seat_0`, `seat_1`, and observer are identical; private-view surfaces are explicitly `not_applicable` per Trace Schema v1 §5; the dev panel and replay export contain only that same public projection.
- Bot tests prove Level 0 and Level 1 select only legal action paths for both factions, are deterministic under declared inputs, finish many games in both pairings (L0vL0, L1vL1, L1vL0 both ways), and produce faction-appropriate viewer-safe explanations.
- Balance evidence: Level 1 vs Level 1 simulation across both maps reports per-faction win rates; a result outside the target band (Assumption A5) triggers a constants retune before public polish, recorded in `BENCHMARKS.md`/`COMPETENT-PLAYER.md`.

### Golden traces

Committed under `games/frontier_control/tests/golden_traces/`, each with perfect-information/no-randomness `not_applicable` markers, and checked by `replay-check`:

- `standard-garrison-win.trace.json` (full match; fort-holding outscores stakes)
- `standard-prospector-win.trace.json` (full match; connected stakes outscore forts)
- `tie-garrison-tiebreak.trace.json`
- `clash-crew-into-guards.trace.json`
- `clash-guard-into-crews.trace.json`
- `stake-and-dismantle.trace.json`
- `supply-cut-scores-zero.trace.json` (staked site disconnected by a guard, then reconnected)
- `round-scoring-breakdown.trace.json`
- `budget-exhaustion-auto-end.trace.json`
- `early-end-turn.trace.json`
- `muster-and-reinforce-caps.trace.json`
- `highlands-setup.trace.json`
- `wrong-faction-diagnostic.trace.json`
- `non-adjacent-move-diagnostic.trace.json`
- `stake-on-guarded-site-diagnostic.trace.json`
- `bot-vs-bot-full-game.trace.json`
- `replay-export-import.trace.json`

### Tools, benchmarks, browser, docs, and CI

- `cargo run -p simulate -- --game frontier_control --games 1000` finishes with no illegal bot action or invariant failure and reports per-faction metrics.
- `cargo run -p replay-check -- --game frontier_control --all` passes.
- `cargo run -p fixture-check -- --game frontier_control` passes for both maps.
- `cargo run -p rule-coverage -- --game frontier_control` passes and maps every rules-doc obligation to tests/traces.
- `cargo bench -p frontier_control` runs the benchmark identity list; `bench-report` enforces smoke floors where calibrated, with the calibration follow-up named in the handoff.
- `npm --prefix apps/web run smoke:wasm`, `smoke:ui`, and `smoke:effects` pass after `frontier_control` is registered.
- `node apps/web/e2e/frontier-control.smoke.mjs` covers the per-faction UI flows listed under Deliverables.
- `bash scripts/boundary-check.sh` passes with `mechanic_pattern` extended to include `faction` and `territory` (plus `adjacency`/`movement` if the false-positive evaluation admits them).
- `node scripts/check-doc-links.mjs` passes.
- `node scripts/check-catalog-docs.mjs` passes, confirming the mechanically-checked catalog surfaces name `frontier_control` / `Frontier Control`.
- `node scripts/check-player-rules.mjs` passes with the generated `apps/web/public/rules/frontier_control.md` in sync and the hidden-information section marked not applicable (no `HIDDEN_INFO_GAMES` entry).
- `node scripts/check-outcome-explanations.mjs` passes with the frontier-control winner/tiebreak templates and rule-ID mirrors registered.
- All thirteen per-game docs are present, link-checkable, original, and consistent with implemented behavior.

## FOUNDATIONS & boundary alignment

| Principle | Stance | Rationale |
|---|---|---|
| §2 Behavior authority | Aligned | Rust owns setup, adjacency legality, faction action trees, validation, clash resolution, stake/control state, supply-connectivity computation, round scoring, terminal detection and tiebreak, semantic effects, replay projection, and both factions' bot decisions. React presents Rust-provided data, including the connectivity highlight and the per-faction score breakdowns. |
| §3 `engine-core` contract kernel | Aligned | The graph map, factions, units, and contests are game-local types expressed through existing generic `Actor`/`Viewer`/`ActionTree`/`CommandEnvelope`/`EffectEnvelope`/replay contracts. `faction`, `territory`, `movement`, and `adjacency` are explicitly forbidden kernel nouns; this gate's forbidden-changes list restates them and work item 11 extends the mechanical boundary check to cover `faction`/`territory`. |
| §4 `game-stdlib` earned | Aligned | Nothing is promoted. Graph topology, control/contest, and faction-asymmetric action sets are first official uses recorded `local-only`; multi-action budgets and role/faction modifiers are second-use comparisons that keep both implementations local; the work-item-1 board_space audit records the only promoted primitive as not applicable (or stops the gate). |
| §5 Static data is typed content | Aligned | Manifest/variants/fixtures hold site IDs, labels, edge pairs, fort flags, stake values, start units, caps, budgets, and round counts only. What `march` does, how clashes resolve, when scoring runs, and what connectivity means are typed Rust. This is the gate's highest-risk boundary — a graph map in data invites "the map says what happens" drift — and the forbidden-changes list bans behavior fields outright. |
| §6 Official games are evidence-heavy | Aligned | The Acceptance-evidence section carries the full official-game contract: rule/property/replay/serialization/visibility/bot tests, the golden trace set, `simulate`/`replay-check`/`fixture-check`/`rule-coverage` runs, benchmarks, and all thirteen per-game docs — browser playability alone is not treated as done. |
| §7 Public UI is central product work | Aligned | `FrontierControlBoard.tsx` renders legal-only faction controls (actions appear only when Rust's tree contains them; the waiting faction gets a waiting state), the graph map and connectivity highlight come from Rust view data, animation is driven by semantic effects with reduced-motion support, and the renderer settles to the latest view. TypeScript computes no adjacency, connectivity, clash, or score. |
| §8 Public bots | Aligned | Both factions' bots act through the same legal action API as humans, with distinct deterministic priority policies, viewer-safe faction-appropriate explanations, and no search beyond Level 1 priorities — no MCTS/ISMCTS, Monte Carlo, ML, or RL. Asymmetry changes the policies, not the fairness posture. |
| §9 Local-first v1/v2 | Aligned | Play is hotseat and human-vs-bot on either side; bot-vs-bot replay is supported. No accounts, hosting, matchmaking, persistence, or network play. Command logs, deterministic replay, and full-timeline export keep the future-hosted path open. |
| §10 IP conservatism | Aligned | Area control, graph maps, and faction asymmetry are unprotected rules territory; the IP-risk register names the adjacent commercial designs (Root foremost) and the specific trade-dress hazards to avoid; all names, labels, prose, and visuals are original; `SOURCES.md` records consulted sources and the originality rationale. |
| §11 Universal invariants | Aligned | Deterministic replay/hash/serialization, output-equivalent viewer projections, legal-only UI, semantic-effect animation, local-first scope, docs/traces/simulations/benchmarks, and bounded agent output are explicit acceptance evidence. With no hidden information and no game-rule randomness, the hidden-info and RNG invariants are satisfied trivially and documented as such in traces and docs. |
| §12 Stop conditions | Clear | Stop if a kernel noun appears, map data turns procedural ("static files start acting procedural"), TypeScript computes adjacency/connectivity/clash/score legality, a bot bypasses the legal API, the board_space audit finds applicable scope without a recorded decision, a reviewed atlas expectation is contradicted, or a graph/control/faction helper is generalized without ledger pressure. |
| §13 ADR triggers | **No ADR expected.** | All shapes stay game-local; visibility/replay contracts, kernel vocabulary, data policy, and renderer defaults are unchanged. Perfect information means ADR 0004's hidden-info taxonomy is not engaged. If the work-item-1 reviews escalate, or implementation genuinely requires changing replay/hash semantics, stop and write the ADR before proceeding. |
| ADR 0004 hidden-info replay/export | Not engaged | `frontier_control` is perfect-information: every projection is public, the export is the full public timeline, and the traces record the hidden-info surfaces as explicitly not applicable per Trace Schema v1 §5. |
| Benchmark ADRs 0002/0003/0005 | Aligned | Smoke benchmarks and threshold files now (`baseline_pending_non_blocking`, matching the flood-watch pattern); variance-aware calibration follow-up named once repeated CI measurements exist (multiple representative runs per ADR 0005); PR smoke stays non-gating. |
| ADR 0006 Blackjack placement | Aligned | Untouched. Gate 13 is not a draw/stand or casino-comparison trigger. |

## Forbidden changes

Do not, in this gate:

- Add `faction`, `territory`, `graph`, `map`, `site`, `edge`, `adjacency`, `unit`, `guard`, `crew`, `stake`, `fort`, `clash`, `supply`, `control`, `movement`, or similar mechanic/domain vocabulary to `engine-core`.
- Add generic `game-stdlib` primitives for graph maps, adjacency, pathfinding/connectivity, control tracking, contest resolution, faction powers, asymmetric scoring, or per-faction bot policy. Graph and control shapes are first official uses; budgets and faction modifiers are second uses; no promotion is authorized by this spec.
- Skip, reorder after implementation, or rubber-stamp the work-item-1 primitive-pressure reviews — including the mandatory `board_space` applicability audit.
- Introduce behavior-in-data: movement rules, clash outcomes, scoring formulas, conditions, selectors, triggers, expressions, or untyped nested behavior objects in any manifest, variant, or fixture. The map is IDs, edges, flags, and numbers. No YAML, no DSL.
- Let TypeScript compute legality, adjacency, supply connectivity, clash resolution, scores, terminal outcome, outcome explanation authority, replay authority, or bot policy.
- Add hidden information or game-rule randomness. Both are deliberately excluded (Assumption A4); introducing either re-scopes the gate's visibility/replay surface and requires a spec correction first.
- Use MCTS, ISMCTS, Monte Carlo, ML, RL, LLMs, or search beyond the documented Level 1 deterministic priorities in any bot.
- Copy area-control or asymmetric-faction game rules prose, faction names, map labels, icons, art, screenshots, or trade dress; do not use the named hazard vocabulary in the IP-risk register for public-facing labels.
- Add hosted multiplayer, accounts, server authority, matchmaking, chat, ranked play, persistence, timers, or real-time pressure.
- Add event decks, periodic automation beyond end-of-round scoring, initiative/eligibility systems, or asymmetric victory conditions — Gate 14 owns those proofs.
- Change trace schema, replay/hash semantics, data versions, action path stability, effect ordering, public-view shape, or golden traces accidentally.
- Delete, rename away, weaken, or rewrite tests merely to get green output. Follow the failing-test protocol.

## Documentation updates required

- `specs/README.md`: Gate 13 row points at this spec with status `Planned` now, then `In progress`/`Done` as the lifecycle advances; flip to `Done` only after exit criteria pass with evidence.
- Do **not** edit `docs/ROADMAP.md` to record progress.
- `docs/MECHANIC-ATLAS.md`:
  - §10 `fixed 2D occupancy / board-space identity`: record the work-item-1 audit outcome (expected: `frontier_control` is **not applicable** to `game-stdlib::board_space` — graph sites are not rectangular coordinates; the primitive stays contract-clean with no new debt).
  - §10B `role-modified action effects`: record the second-use comparison outcome (expected: related but distinct shapes — magnitude modifiers on shared actions vs disjoint faction action sets; both stay local; name `event_frontier` as the next comparison candidate).
  - §10B `multi-action turn budgets`: record the second official use comparison against `flood_watch` (keep local; the third-use hard gate arms for `event_frontier`).
  - §10B `shared-outcome cooperative terminal`: record the comparison review outcome (expected: not a second use — per-faction competitive winner; the row stays armed for team/shared-victory pressure).
  - §10B `reaction window/pending response`: record the review outcome (expected: not reaction-capable — clashes resolve immediately inside the mover's command; the second-use trigger stays armed for Gate 14).
  - §10B `deterministic shuffle / private hand / staged reveal`: note the non-use (no shuffle, no randomness; trigger untouched).
  - New §10/§10B rows for `graph-map topology / adjacency legality / connectivity scoring`, `site control / deterministic contest resolution`, and `faction-asymmetric action sets and scoring`: record each as `local-only` first official use with a second-use revisit trigger naming `event_frontier` (and Gate P pressure as a non-public comparison only).
  - Confirm §10A stays `_None_`.
- Author all thirteen `games/frontier_control/docs/*` from templates and keep them consistent with implemented behavior; `SOURCES.md` records consulted prior art at implementation time (see the IP-risk register under Implementation reference), what was and was not used, and the originality/naming review.
- Update `progress.md` and root `README.md` after implementation, not before evidence passes.
- Reconcile the web-shell catalog surfaces as a closeout inside this gate (per `specs/README.md` §10), not a later aftermath pass:
  - `apps/web/README.md` intro catalog list (add `frontier_control` / `Frontier Control`);
  - root `README.md` "current official games are" list;
  - `apps/web/README.md` Smoke Layers `smoke:e2e` bullet, and the `smoke:e2e` script in `apps/web/package.json` (add `node e2e/frontier-control.smoke.mjs`);
  - the `apps/web/README.md` Shell Surface renderer list (manual closeout surface — `check-catalog-docs.mjs` deliberately does **not** mechanically check it).
  - `node scripts/check-catalog-docs.mjs` enforces the mechanically-checked surfaces in CI; it must pass.
- Player-rules and outcome-explanation surfaces: `games/frontier_control/docs/HOW-TO-PLAY.md` → `apps/web/public/rules/frontier_control.md` via `scripts/copy-player-rules.mjs`; the hidden-information section marked not applicable (perfect information, no `HIDDEN_INFO_GAMES` entry); winner/tiebreak templates and rule-ID mirrors registered so `scripts/check-outcome-explanations.mjs` passes.
- Ensure `scripts/check-doc-links.mjs` passes after doc changes.

## Sequencing

- **Predecessor:** Gate 12 (`flood_watch`) is `Done` in the spec index with evidence; the atlas §10A promotion-debt register records `_None_`, so no maintenance interlock precedes this gate.
- **Admission rule:** Before implementation starts, confirm `docs/MECHANIC-ATLAS.md` §10A still has no open promotion debt (it records `_None_` at spec time), and complete work item 1's ledger reviews — including the mandatory `board_space` applicability audit — before any map, action, or scoring code is written.
- **This gate:** Gate 13 proves graph topology, control, asymmetry, faction-specific actions/scoring, per-faction UI, and per-faction bots per ROADMAP §14, with the graph map as typed content and faction asymmetry in Rust application logic as the headline architectural proofs.
- **Successor:** Gate 14 (`event_frontier`) remains not yet specced. It should not start until Gate 13 evidence passes and no open promotion debt remains. The Gate 14 spec author should check the atlas rows this gate touches: `event_frontier` is the named third-use candidate for multi-action budgets (the hard gate fires there), the second comparison for graph/control shapes, the expected second use of event decks/automation from Gate 12, and the asymmetric-victory proof this gate deliberately deferred.

## Assumptions

- A1: Two seats and two factions are the sufficient proof shape for Gate 13; more factions or seats add balance/UI surface without strengthening the graph, control, asymmetry, or per-faction-bot proofs.
- A2: Public display name `Frontier Control`, the faction names (`Garrison`, `Prospectors`), and the site labels (`Gatehouse`, `Signal Hill`, `Base Camp`, `Ford`, `Quarry`, `Timberline`, `Goldfield`) are original placeholders; maintainers may rename after IP review before implementation. Woodland-creature theming and the word `clearing` are deliberately avoided (Root); `Warden` is avoided as a faction name (already a Flood Watch role).
- A3: The standard map uses seven sites and ten edges, two Garrison forts (two guards each), one Prospector base camp (three crews), per-site stake values (Goldfield 3, Quarry 2, Ford 1, Timberline 1), unit caps of three per faction per site, a two-action budget, and eight rounds; the highlands map varies the graph, starts, values, and round count. Maintainers may tune any constant while preserving the fixed-round terminal and proof scope.
- A4: Perfect information and zero game-rule randomness are deliberate scope decisions: they keep the gate's proof budget on graph topology and asymmetry, make every trace's hidden-info/stochastic surfaces explicitly `not_applicable` (Trace Schema v1 §5), and leave ADR 0004 unengaged. Adding either later is a spec change, not drift.
- A5: The balance target is that each faction wins between roughly 35% and 65% of Level 1 vs Level 1 games on the standard map (asymmetric but fair), measured by simulation; results outside the band trigger a constants retune recorded in `COMPETENT-PLAYER.md`/`BENCHMARKS.md` before public polish.
- A6: Clash resolution is deterministic and asymmetric by design — a guard entering a crewed site removes one crew and survives; a crew entering a guarded site removes one guard and is itself removed. This asymmetry lives in Rust application logic and is itself part of the faction-asymmetry proof; maintainers may adjust the exact exchange rule while keeping it deterministic and faction-asymmetric.
- A7: Both factions score onto one comparable numeric track via faction-specific formulas; the terminal is a score comparison with a Garrison tiebreak. Differing *victory conditions* are deliberately deferred to Gate 14's asymmetric-victory proof.
- A8: The work-item-1 expectations hold: `board_space` audits not applicable, the role-modifier and budget rows resolve as keep-local comparisons, the shared-outcome and reaction-window rows record non-uses, and no shuffle row is touched. Work item 1 verifies all of these against the atlas before implementation and stops the gate if any fails.
- A9: Level 0 + Level 1 bots per faction satisfy the gate (mirroring Gates 9.1–12); a Level 2 claim requires the full `COMPETENT-PLAYER.md` + `BOT-STRATEGY-EVIDENCE-PACK.md` evidence workflow before coding.
- A10: No external research pass was run for this spec (matching Gate 12's posture); the IP-risk register and balance reasoning draw on maintainer review, and `SOURCES.md` must record whatever is actually consulted at implementation time. If deeper prior-art or game-design research is wanted, run `research-brief` or `deep-research` before decomposition.
- A11: If any assumption is wrong, correct the smallest local game/spec surface; do not generalize the engine.

---

# Implementation reference

## Product intent / what this gate proves

`Frontier Control` is a deliberately compact original asymmetric area-control game. A frontier valley of seven sites is joined by trails. The **Garrison** holds two forts and wants to keep them garrisoned; the **Prospectors** strike out from their base camp to stake valuable sites and keep supply lines open back to camp. The factions play by different rules: different units, different actions, different ways of winning fights, different ways of scoring. After eight rounds, the higher score takes the frontier.

Architecturally, the gate makes the platform prove two new shapes cleanly:

- **The map is a graph.** Legality ("you may march there") and scoring ("that stake is supplied") both depend on edges and connectivity, not coordinates. The graph lives as typed static content (IDs, edge pairs, flags, values) validated into Rust types; every adjacency check and connectivity traversal is Rust behavior. Nothing about the graph enters `engine-core` or `game-stdlib`, and the existing `board_space` primitive is audited not applicable.
- **The seats are not symmetric.** Disjoint action vocabularies, asymmetric contest rules, and faction-specific scoring all flow through the same generic action-tree/command/effect contracts that served symmetric games — proving the contracts themselves never assumed symmetry. Per-faction UI panels and per-faction bot policies complete the proof at the presentation and AI layers.

## Proposed original rules: `Frontier Control`

### Components

- Two seats: `seat_0` plays faction `faction_garrison` (the Garrison), `seat_1` plays `faction_prospectors` (the Prospectors).
- Seven sites with stable IDs: `site_gatehouse`, `site_signal_hill` (Garrison forts), `site_base_camp` (Prospector camp), `site_ford`, `site_quarry`, `site_timberline`, `site_goldfield`.
- Ten edges (trails): gatehouse–ford, gatehouse–quarry, gatehouse–signal_hill, signal_hill–quarry, signal_hill–goldfield, quarry–ford, quarry–timberline, ford–base_camp, timberline–base_camp, timberline–goldfield.
- Garrison **guards** (cap 3 per site), Prospector **crews** (cap 3 per site), Prospector **stakes** (one per site at most).
- Per-site stake values (typed data): Goldfield 3, Quarry 2, Ford 1, Timberline 1; forts and base camp are not stakeable (value 0, flagged).
- A round marker (eight rounds), a two-action turn budget, per-faction score totals, and a freshness token.

### Setup

1. Validate exactly two seats; assign factions from variant data.
2. Load the map variant (`frontier_control_standard` or `frontier_control_highlands`) from typed validated static data: sites, labels, edges, fort flags, stake values, start units, caps, budget, round count. Validation rejects disconnected graphs, dangling edges, duplicates, and behavior-looking fields.
3. Place start units: two guards at each fort; three crews at the base camp. No randomness — setup is fully determined by the variant.
4. Set round 1, Prospectors to move first, full budget, terminal `None`, freshness token `0`.

### Turn flow

Each round, the Prospectors take a turn, then the Garrison; each turn is up to two actions, submitted one at a time with the tree regenerating (remaining-budget metadata) after each:

**Prospector actions**
- `march/<from>/<to>` — move one crew along an edge. If the destination holds guards, a **clash** resolves: one guard is removed and the marching crew is also removed (crews trade themselves for guards).
- `stake/<site>` — place a stake at a site holding at least one crew, no guards, no existing stake, and a nonzero stake value.
- `muster` — add one crew at the base camp (cap 3) if no guards occupy it.
- `end_turn` — always legal; forfeits remaining budget.

**Garrison actions**
- `patrol/<from>/<to>` — move one guard along an edge. If the destination holds crews, a **clash** resolves: one crew is removed and the patrolling guard survives and occupies (guards win entering clashes).
- `reinforce/<fort>` — add one guard at a fort the Garrison still holds (≥1 guard, no crews), cap 3.
- `dismantle/<site>` — remove the stake at a site where a guard is present.
- `end_turn` — always legal.

The waiting faction's tree is empty with safe waiting metadata. Spending the final budget point ends the turn exactly as `end_turn` does.

### Round scoring (deterministic, inside the turn-ending command)

When the Garrison's turn ends, the round scores as a deterministic consequence of that command's application (the Gate 12 automation mechanism at small scale):

- **Garrison:** +1 point per fort holding ≥1 guard and 0 crews.
- **Prospectors:** for each staked site, +its stake value if a path of edges exists from the site to the base camp passing only through sites with no guards (the stake is *supplied*). Cut stakes score zero this round but are not removed.

`RoundScored` carries the full per-faction breakdown (which forts held, which stakes supplied/cut) for the effect log, score tracks, and outcome surface.

### Terminal

After round 8 scores, the match ends: the higher total wins; a tie goes to the Garrison (the incumbent holds the frontier — stable rule ID for the tiebreak). The `Terminal` effect carries the winner, both totals, the final breakdown, and the decisive cause for the outcome-explanation surface.

### Why these constants resist degeneracy

Maximum Garrison income is 2 points per round (16 total); maximum Prospector income is 7 per round (all four value sites staked and supplied) but reaching it requires spreading thin across the graph while every supply path runs within one trail of a fort. Guards threaten supply cuts cheaply (one patrol can sever Goldfield's two paths at Timberline or Signal Hill's flank), but every guard pulled off a fort risks the fort point and a crew trade. The crews-trade clash rule means the Prospectors can buy guard removals at material cost, so turtling on forts loses to a patient stake economy and over-patrolling loses the fort race. Constants are tunable under Assumption A5 with simulation evidence; the per-faction win band, not the exact numbers, is the requirement.

## State, actions, and validation sketch

### State

- `variant: Variant` (map)
- `seats: [SeatId; 2]`, `factions: [FactionId; 2]` (from variant data)
- `round_number: u8`, `active_faction: FactionId`, `phase: Phase` — `Phase::Action { budget_remaining: u8 }`, `Phase::Terminal`
- `sites: Vec<SiteState>` — `guards: u8`, `crews: u8`, `stake: bool`, with `fort: bool` and `stake_value: u8` from validated map data; adjacency held as a validated edge index
- `scores: [u16; 2]`, `terminal_outcome: Option<Outcome>` (`Winner { faction } + totals + tiebreak flag`), `freshness_token: FreshnessToken`

Supply connectivity is computed by Rust traversal at scoring time and projected into the public view (per-stake supplied/cut booleans) so the UI never re-derives it.

### Legal action tree

- Active Prospector: `march/<from>/<to>` for each crew-holding site and incident edge; `stake/<site>` where legal; `muster` when below cap and camp unguarded; `end_turn` always. Choice metadata carries remaining budget, clash previews ("marching here trades one crew for one guard"), and stake values — all Rust-supplied.
- Active Garrison: `patrol/<from>/<to>`, `reinforce/<fort>`, `dismantle/<site>`, `end_turn`, same discipline.
- Waiting faction: flat empty tree with safe waiting metadata naming who acts and why. Terminal: empty trees.

### Validation

Rejects: stale freshness token; actor not seated; non-active faction; terminal phase; out-of-budget submission; non-adjacent move; moving a unit that is not present; staking with guards present, an existing stake, or zero value; mustering at cap or under occupation; reinforcing a lost fort or at cap; dismantling without a guard or stake; malformed path segments. Diagnostics are viewer-safe and faction-aware ("the trail ends elsewhere — those sites are not connected", "that fort has fallen; retake it before reinforcing").

## Semantic effect model

| Effect | Visibility | Payload rule |
|---|---|---|
| `CrewMarched { from, to }` / `GuardPatrolled { from, to }` | Public | Movement before any clash effect. |
| `ClashResolved { site, guard_removed: bool, crew_removed: bool, entering_faction }` | Public | Encodes both asymmetric outcomes with one shape. |
| `StakePlaced { site }` / `StakeDismantled { site }` | Public | — |
| `CrewMustered { site, new_count }` / `GuardReinforced { site, new_count }` | Public | — |
| `TurnEnded { faction, unspent_budget }` | Public | — |
| `RoundScored { round, garrison_points, prospector_points, fort_breakdown, stake_breakdown }` | Public | Per-stake supplied/cut flags included — the UI's connectivity story. |
| `Terminal { winner, garrison_total, prospector_total, tiebreak_applied, summary }` | Public | Drives the outcome-explanation surface. |
| `PrivateDiagnostic` / `PublicDiagnostic` | Private / public | Viewer-safe reasons only. |

Everything is public (perfect information); effect filtering is the identity projection, asserted by visibility tests. React animates moves, clashes, stakes, and the round-scoring breakdown in effect order and settles to the latest view. Reduced motion preserves order and copy.

## Visibility and replay model

- **Single public projection, all viewers:** sites with guards/crews/stakes/fort flags/values, supplied/cut flags from the last scoring, scores, round/turn/budget, factions, terminal outcome. `get_view` is output-equivalent for `seat_0`, `seat_1`, and observers; visibility tests assert it.
- **No hidden information, no game-rule randomness:** traces carry explicit `not_applicable` markers for hidden-information redaction, stochastic events, and private-view hashes per Trace Schema v1 §5; there is no seed-derived game state to protect and ADR 0004 is not engaged.
- **Replay/export:** the full public command/effect timeline exports and imports losslessly; determinism is over seats + variant + command stream.

## Bot policy

### Level 0

`FrontierRandomBot` selects uniformly from the legal tree using deterministic bot RNG helpers, for either faction. It never constructs out-of-tree actions.

### Level 1 — Garrison (`FrontierGarrisonLevel1Bot`)

Deterministic priorities, public view + bot seed only, viewer-safe explanations:

1. **Hold the forts:** if a fort has adjacent crews and its guards ≤ that crew count, `reinforce` it (or `patrol` a guard back if at cap or fallen). Explanation: "Reinforced the Gatehouse: prospector crews are at the walls."
2. **Cut the richest supply:** patrol to the guard-free site that disconnects the highest total supplied stake value (deterministic evaluation over the public graph, stable site-order tie-break).
3. **Dismantle** a stake where a guard already stands.
4. **Reinforce** below cap with spare budget; else **end turn**.

### Level 1 — Prospectors (`FrontierProspectorLevel1Bot`)

1. **Stake the richest claim:** `stake` the highest-value eligible site.
2. **Open the path:** march toward the nearest unstaked nonzero-value unguarded site by deterministic shortest path (stable tie-break); trade into a guard only when the cleared site restores or protects more stake value than the crew costs (simple public arithmetic, documented).
3. **Muster** when crews on the map < 2.
4. **End turn** when nothing improves the position.

Bot tests assert per-faction legality, determinism, distinct policies (a Garrison tree never yields a Prospector action), and explanation copy that names faction-appropriate reasons. Simulation reports per-faction win rates, average scores, and tiebreak frequency.

## WASM/browser wiring

- Catalog entry with `game_id: frontier_control`, display name `Frontier Control`, perfect-information marker in game metadata, both map variants, and docs links.
- `get_view` / `get_action_tree` / `apply_action` / `run_bot_turn` / `export_replay` / `import_replay` follow the existing raw-ABI contract; a turn-ending action's result carries the round-scoring effect batch.
- `FrontierControlBoard.tsx` renders the SVG graph (nodes positioned from typed layout metadata, edges as trails), unit/stake/fort markers, supplied/cut highlighting from view data, per-faction action panels grouped from the Rust tree, score tracks with breakdown disclosure, budget and round indicators, effect log, and replay controls. Anchors use site/round IDs.
- Winner templates ("The Garrison held the frontier, 14–11." / "The Prospectors struck it rich, 16–12." / tiebreak copy) register on the outcome-explanation surface with rule-ID mirrors; TypeScript interpolates Rust-supplied parameters only.

## Benchmark operations

- `legal_actions_garrison_midgame`
- `legal_actions_prospectors_midgame`
- `validate_action`
- `apply_march_with_clash`
- `apply_end_turn_round_scoring` (the connectivity hot path)
- `supply_connectivity_traversal`
- `project_public_view_midgame`
- `state_hash_terminal`
- `garrison_level1_bot_decision`
- `prospector_level1_bot_decision`
- `random_playout` (full games per second)

Thresholds start as non-blocking smoke floors (`baseline_pending_non_blocking`, matching the flood-watch pattern) with a named calibration follow-up under ADR 0002/0003/0005 once repeated CI measurements exist.

## IP-risk register and source guidance

No external research pass backs this spec (Assumption A10); this register names the adjacent commercial designs so implementation steers clear of their protected expression while freely using unprotected mechanics:

- **Root (Leder Games):** the closest published shape — asymmetric factions with different action economies on a node-and-path map. Avoid: the woodland-creature theme, faction identities (Marquise, Eyrie, Alliance, Vagabond), the word `clearing` for map nodes, the crafting/card economy, and Kyle Ferrin's art style. Frontier Control has two human-neutral factions, no cards, no crafting, and original site/faction names.
- **Risk (Hasbro):** avoid the world-conquest framing, continent-bonus presentation, dice-battle vocabulary, and any "Risk" naming. Frontier Control has no dice, no elimination, and a fixed eight-round score race.
- **El Grande / area-majority family:** avoid caballero/courtier theming and majority-cube trade dress; control here is occupation-based, not majority-count.
- **Small World, Kemet, Blood Rage, A Game of Thrones: The Board Game, Twilight Struggle:** more distant; avoid their named factions/powers, region names, and board trade dress generally.
- Mechanics themselves — graph maps, area control, asymmetric factions, action points, supply lines — are unprotected rules territory (the same rules-vs-expression boundary recorded in the Gate 11 and Gate 12 specs via the U.S. Copyright Office games circular and *DaVinci Editrice v. ZiKo Games*).

`games/frontier_control/docs/SOURCES.md` must record what was actually consulted at implementation time, what was used and not copied, why every name and label is original, and asset/font status. If any label feels trademark-forward or trade-dress-adjacent at review time, rename it before implementation.

## Outcome

Completed on 2026-06-11.

Gate 13 shipped `frontier_control` / Frontier Control as the asymmetric graph-map
area-control proof. The completed gate includes the game crate, typed standard
and Highlands graph-map variants, deterministic setup, adjacency-constrained
movement, immediate asymmetric clash resolution, site control, supply-connected
stake scoring, Garrison fort scoring, comparable terminal score track with
Garrison tiebreak, Level 0 and Level 1 bots for both factions, native tests and
golden traces, benchmarks, tool and CI registration, WASM/API bridge
registration, SVG browser board, generated player rules, outcome explanations,
catalog documentation, and browser E2E no-leak coverage.

The mechanic-atlas reviews completed as planned: `game-stdlib::board_space` was
audited not applicable to site/edge graph topology, graph/control/asymmetry
shapes remain local-only first uses, role/faction modifiers and multi-action
turn budgets remain local second-use comparisons, shared-outcome and
reaction-window rows were reviewed as not applicable to this competitive
immediate-clash gate, and deterministic shuffle is not used. No `engine-core`
noun, `game-stdlib` helper, or promotion debt was introduced; `docs/MECHANIC-ATLAS.md`
§10A remains `Current debt: _None_`.

Final verification passed:

- `cargo fmt --all --check`
- `cargo clippy --workspace --all-targets -- -D warnings`
- `cargo test --workspace`
- `cargo run -p simulate -- --game frontier_control --games 1000`
- `cargo run -p replay-check -- --game frontier_control --all`
- `cargo run -p fixture-check -- --game frontier_control`
- `cargo run -p rule-coverage -- --game frontier_control`
- `cargo bench -p frontier_control`
- `bash scripts/boundary-check.sh`
- `node scripts/check-doc-links.mjs`
- `node scripts/check-catalog-docs.mjs`
- `node scripts/check-player-rules.mjs`
- `node scripts/check-outcome-explanations.mjs`
- `npm --prefix apps/web run smoke:wasm`
- `npm --prefix apps/web run smoke:ui`
- `npm --prefix apps/web run smoke:effects`
- `npm --prefix apps/web run smoke:e2e`

Release constraint: the registered Level 1 simulation currently reports a
Garrison-dominant 1000-0 result on the standard map; the balance retune debt is
recorded in the game docs and does not block the Gate 13 architecture proof.

No unresolved blocking issues remain. The completed tickets are archived under
`archive/tickets/GAT13FROCONASY-001.md` through
`archive/tickets/GAT13FROCONASY-017.md`, and this spec is archived at
`archive/specs/gate-13-frontier-control-asymmetric-area-control-proof.md`.

# Gate 12 Flood Watch Cooperative Event-Pressure Proof

| Field | Value |
|---|---|
| Spec ID | `gate-12-flood-watch-cooperative-event-pressure-proof` |
| Roadmap stage | 12 |
| Roadmap build gate | Gate 12 — cooperative event pressure |
| Status | Done |
| Date | 2026-06-11 |
| Owner | Rulepath maintainers |
| Primary crate / internal game id | `flood_watch` |
| Public display name | `Flood Watch` unless IP review prefers another neutral original name |
| Browser implementation | Required |
| Authority order | `docs/FOUNDATIONS.md` → `docs/ARCHITECTURE.md` → `docs/ENGINE-GAME-DATA-BOUNDARY.md` → `docs/OFFICIAL-GAME-CONTRACT.md` → `docs/MECHANIC-ATLAS.md` → `docs/AI-BOTS.md` → `docs/UI-INTERACTION.md` → `docs/TESTING-REPLAY-BENCHMARKING.md` → `docs/ROADMAP.md` → accepted ADRs that explicitly supersede those documents → this spec |

Where this spec and a foundation document disagree, the foundation document wins.

> Reader orientation: this spec carries the canonical Rulepath section set: Objective, Scope, Deliverables, Work breakdown, Exit criteria, Acceptance evidence, FOUNDATIONS & boundary alignment, Forbidden changes, Documentation updates required, Sequencing, and Assumptions. Detailed proposed rules, state/effect/view sketches, bot policy, WASM/browser wiring, fixtures, traces, benchmarks, and IP/source guidance are preserved below the canonical sections under **Implementation reference**.

## Objective

Implement `flood_watch` as the Gate 12 browser proof for the ROADMAP §14 cooperative exit lines: **shared win/loss, event deck pressure, role powers, environment automation, multi-action budgets, scenario setup, and a cooperative bot baseline** — with automation that is deterministic and effect-log-driven, a tested shared outcome, game-local role powers, and a UI that explains event pressure clearly.

Gates 8–11 proved hidden information, resources, commitment/reveal, betting, tricks, and reaction windows — all competitive, per-seat-outcome games. This gate adds the missing interaction classes before the Gate 13–14 asymmetry/event games: a **shared terminal outcome** that both seats win or lose together, an **environment that acts** — a deterministic, effect-log-driven automation phase driven by a shuffled event deck rather than by any seat's command — and a **multi-action turn budget** in which one seat submits several validated commands before the environment responds.

The result is a small, original, deterministic, two-seat cooperative browser game, `Flood Watch`, that proves:

- a shared win/loss outcome computed, tested, and explained from Rust;
- event-deck pressure: a deterministically shuffled, hidden-until-drawn event deck whose order never leaks, with staged public reveals on draw and forecast;
- environment automation: an environment phase that resolves as a deterministic, replayable consequence of the turn-ending command, expressed entirely through Rust semantic effects — never a TypeScript timer, never a synthetic actor in the command stream;
- role powers: two asymmetric, public, deterministic role modifiers that stay entirely game-local;
- multi-action budgets: several validated actions per turn against a tracked budget, with the legal tree regenerating after every action;
- scenario setup: at least two typed scenario variants (starting levels, deck composition, budgets) as static content, never behavior;
- a cooperative bot baseline: Level 0 random and Level 1 rule-informed bots that can play either role, as a human's teammate or in bot-vs-bot replay, through the normal legal-action API.

This is not an engine-generalization gate. It is a small official game that reuses the proven deterministic-shuffle, redacted-export, and effect-log machinery while keeping all event, deck, role, scenario, district, flood, levee, budget, and shared-outcome nouns local to `games/flood_watch`.

## Scope

### In scope

- New official game crate `games/flood_watch` with typed Rust setup, state, actions, validation, application, environment automation, effects, visibility projection, replay support, variants, UI metadata, and bots.
- Default two-seat cooperative variant `flood_watch_standard` plus a second, harder scenario `flood_watch_deluge` — two typed scenarios prove scenario setup.
- Original deterministic rules: five districts with flood levels, per-district levee tokens, a deterministically shuffled event deck dealt face-down, alternating seat turns with a three-action budget, a public forecast action, an environment phase that draws and resolves events after each turn, shared loss on any inundated district, and shared win when the deck is exhausted without loss.
- Game-local typed nouns only: district IDs, flood levels, levee tokens, event card kinds, roles, action budget, environment phase, scenario constants, and the shared outcome. These stay inside `games/flood_watch` and its docs/tests/UI projection.
- **Pre-implementation atlas work** (work item 1): record the reaction-window second-use review (expected outcome: `flood_watch` is **not** reaction-capable), the deterministic-shuffle row review (expected outcome: **not** a fifth use of the shuffle + private-holdings + redacted-reveal triple, because there are no per-seat private holdings), and the new first-use rows (shared-outcome cooperative terminal, event-deck environment automation, role-modified action effects, multi-action turn budgets) — all before implementation code is written.
- Hidden-from-all event-deck order under ADR 0004: internal full traces carry the order; viewer-scoped exports and every browser surface redact the undrawn deck, with each card appearing publicly only from its draw or forecast effect.
- Viewer-safe action trees: the active seat's tree carries the legal budgeted actions with remaining-budget metadata; the teammate's tree is empty with safe waiting metadata; the action-phase tree always contains `end_turn`, so a turn can never stall.
- Effect-log-driven environment animation: event draw, levee absorption, flood-level rise, inundation, and terminal effects emitted by Rust and rendered by React with reduced-motion support and log copy that explains the pressure ("Downpour over the Old Docks: the levee absorbed the rise").
- Level 0 random legal bot and Level 1 rule-informed `FloodWatchLevel1Bot` playing **either role and either seat**: a deterministic priority policy (rescue imminent losses first, mitigate forecast threats, reinforce by public expected-threat counting, forecast with spare budget), usable as a human's teammate and in bot-vs-bot cooperative replay.
- Full official-game evidence: unit/rule/property/replay/serialization/visibility/no-leak/bot tests, golden traces, simulations, fixture validation, rule coverage, benchmarks, per-game docs (thirteen, including `HOW-TO-PLAY.md` and `PRIMITIVE-PRESSURE-LEDGER.md`), WASM registration, browser board, E2E smoke, a11y/no-leak checks, player-rules and outcome-explanation surfaces, and CI/tool registration.
- Documentation updates per the section below, including the `specs/README.md` index flip and the web-shell catalog closeout surfaces.

### Out of scope

- Three- and four-seat variants and solo play. Two seats prove shared outcome, role asymmetry, and the teammate-bot mode; seat-count generality adds nothing to the Gate 12 proof.
- Graph topology, adjacency, routes, regions, spreading/cascading floods, or movement of any kind. Districts are a flat fixed list with no spatial relationships and players have no board position. Graph maps are Gate 13's proof (`frontier_control`); flood spread between adjacent districts would pre-empt it.
- Reaction windows, pending responses, interrupts, or any seat responding to another seat's pending action. The environment phase is automation, not a response window; if design drift introduces one, stop and re-run the atlas reaction-window review.
- Per-seat hidden information. Both seats see the same public state; the only hidden information is the undrawn event-deck order, hidden from everyone. Hands, secret roles, hidden objectives, and traitor mechanics are excluded.
- Deck re-shuffle escalation, event cards that schedule other cards, chained/conditional event scripting, or any event behavior expressed in static data. Event kinds are a closed typed Rust enum; data declares counts only.
- A generic cooperative engine, event-deck engine, automation engine, role-power system, action-budget system, or shared-outcome helper in `engine-core` or `game-stdlib`. All shapes are first official uses and stay local.
- Difficulty scaling beyond the two named scenarios, campaign/legacy structure, timers, real-time pressure, or undo.
- Hosted multiplayer, accounts, matchmaking, server persistence, chat, or ranked play. Cooperative play is hotseat and human-plus-bot, local-first.
- MCTS/ISMCTS/Monte Carlo/ML/RL bot work, deck-order inference beyond public composition counting, or hidden-state sampling.
- Ticket decomposition. The Work breakdown lists bounded candidate AGENT-TASKs only; ticket files are a later step.

### Not allowed

Carried from ROADMAP §14 and tightened for this gate:

- Private licensed content, DSL by stealth, or architecture claims beyond proven games (the ROADMAP Gates 12–14 shared prohibition).
- Trade-dress imitation of published cooperative games (Forbidden Island, Forbidden Desert, Pandemic, Spirit Island, or similar). Mechanics-level similarity is acceptable; names, labels, prose, art direction, and theme must be original — see the IP-risk register under Implementation reference for the named hazards (island tiles, sandbag labels, pawn roles, outbreak/epidemic vocabulary, re-shuffle escalation).
- `engine-core` nouns such as `event`, `deck`, `card`, `role`, `scenario`, `district`, `flood`, `levee`, `budget`, `environment`, `cooperative`, or `shared outcome` beyond existing generic actor/viewer/action/effect/replay envelopes. `role` and `scenario` are explicitly on the FOUNDATIONS §3 forbidden-noun list, and this is the first game built around both.
- Generic `game-stdlib` helpers such as `EventDeck`, `AutomationPhase`, `RolePower`, `ActionBudget`, `SharedOutcome`, or `CoopBot`. Every Gate 12 shape is a first official use; the atlas hard gate fires at the third use, not now.
- Static data behavior: event effects, triggers, selectors, conditions, formulas, or scripted sequences in manifests, variants, or fixtures. Scenario data declares typed counts and constants only; what an event *does* is typed Rust.
- The undrawn event-deck order in DOM, `data-testid`, local storage, browser payloads, effect logs, diagnostics, replay exports, dev panels, bot explanations, or candidate rankings — at any point in the match, including post-terminal.
- TypeScript legality, budget tracking, environment resolution, terminal detection, outcome explanation authority, replay authority, or bot policy. The environment phase must not be driven by TypeScript timers or client-side sequencing.
- Bots reading the undrawn deck order, omniscient mitigation, or any input beyond the public view and declared bot seed.
- Accidental trace/hash/schema migration. Any intentional migration needs explicit notes and accepted review.

## Deliverables

| Area | Required artifacts |
|---|---|
| Atlas / ledger (pre-implementation) | `games/flood_watch/docs/PRIMITIVE-PRESSURE-LEDGER.md` recording (a) the reaction-window second-use review against `games/masked_claims/docs/PRIMITIVE-PRESSURE-LEDGER.md` (expected: not reaction-capable; the atlas row's second-use trigger stays armed); (b) the deterministic-shuffle row review (expected: not a fifth use — no per-seat private holdings — with rationale and any new shuffle-helper evidence recorded); (c) first-use records for shared-outcome cooperative terminal, event-deck environment automation, role-modified action effects, and multi-action turn budgets; `docs/MECHANIC-ATLAS.md` §10B updates for all reviewed/new rows; confirmation that §10A stays `_None_`. |
| Workspace and crate | Root `Cargo.toml` registration; `games/flood_watch/Cargo.toml`; source modules `src/actions.rs`, `src/bots.rs`, `src/effects.rs`, `src/ids.rs`, `src/lib.rs`, `src/replay_support.rs`, `src/rules.rs`, `src/setup.rs`, `src/state.rs`, `src/ui.rs`, `src/variants.rs`, `src/visibility.rs`. Mirror the `games/masked_claims` file-for-file shape unless a file is explicitly documented as not applicable. `masked_claims` is the closest template for deterministic shuffle and redacted-export machinery; `secret_draft` for waiting-state UI metadata; `token_bazaar` for typed scenario/constants data. |
| Static data | `games/flood_watch/data/manifest.toml`, `games/flood_watch/data/variants.toml`, `games/flood_watch/data/fixtures/flood_watch_standard.fixture.json`, `games/flood_watch/data/fixtures/flood_watch_deluge.fixture.json`. Static files contain typed metadata, district IDs and labels, scenario constants (starting levels, deck composition counts per typed event kind, budgets, levee caps), role labels, and fixtures only; no behavior. Unknown and behavior-looking fields are rejected. |
| Benchmarks | `games/flood_watch/benches/flood_watch.rs`, `games/flood_watch/benches/thresholds.json`. The benchmark identity list under Implementation reference, including the environment-phase resolution hot path and a full cooperative random playout. Initial thresholds are non-blocking smoke floors plus a named calibration follow-up under ADR 0002/0003/0005. |
| Native tests | `games/flood_watch/tests/rules.rs`, `property.rs`, `replay.rs`, `serialization.rs`, `visibility.rs`, `bots.rs`. Coverage detailed under Acceptance evidence. Keep `tests/rules.rs` as a dedicated rule-test file per the hidden-information-game convention. |
| Golden traces | Under `games/flood_watch/tests/golden_traces/`: the seventeen traces listed under Acceptance evidence. |
| Per-game docs | Thirteen docs instantiated from `templates/*`: `games/flood_watch/docs/AI.md`, `BENCHMARKS.md`, `BOT-STRATEGY-EVIDENCE-PACK.md`, `COMPETENT-PLAYER.md`, `GAME-IMPLEMENTATION-ADMISSION.md`, `HOW-TO-PLAY.md`, `MECHANICS.md`, `PRIMITIVE-PRESSURE-LEDGER.md`, `PUBLIC-RELEASE-CHECKLIST.md`, `RULE-COVERAGE.md`, `RULES.md`, `SOURCES.md`, `UI.md`. |
| Tools | Register `flood_watch` in `tools/simulate`, `tools/replay-check`, `tools/fixture-check`, `tools/rule-coverage`, `tools/bench-report` if thresholds/reporting enumerate game IDs, and `tools/seed-reducer` / `tools/trace-viewer` if their dispatch tables need game IDs or effect names. |
| WASM/API | Register `flood_watch` in `crates/wasm-api/src/lib.rs` catalog, setup, action, bot, effect, view, replay/export/import, and no-leak redaction paths. `get_view(match_id, viewer_seat)` honors viewer scope (all seats receive the same public projection; the undrawn deck order is in no projection); public export defaults to the viewer-scoped observation timeline under ADR 0004 with the undrawn deck redacted. |
| Browser | `apps/web/src/components/FloodWatchBoard.tsx`; GamePicker/catalog support through Rust metadata; `ActionControls` support for the budgeted action phase (remaining-budget display; teammate waiting state) without TS legality; `EffectLog` / `effectFeedback.ts` entries for action, draw, absorption, rise, inundation, and terminal effects; **shared-outcome** entries in `apps/web/src/components/outcomeExplanationTemplates.ts` (team won/lost — the first *cooperative shared-win/shared-loss* outcome on that surface; the existing `*_draw` templates already render winner-free results with `requiredParams: []`, so the `OutcomeExplanationTemplate` type is reused unchanged) and viewer-safe rationale mirrors in `apps/web/src/wasm/client.ts`; player-facing rules at `apps/web/public/rules/flood_watch.md` generated from `games/flood_watch/docs/HOW-TO-PLAY.md` via `scripts/copy-player-rules.mjs`; `flood_watch` added to the `HIDDEN_INFO_GAMES` set in `scripts/check-player-rules.mjs` (deck order is hidden information); shell reducer/client type coverage; safe dev panel output; replay import/export wiring; reduced-motion support; responsive, accessible board presentation. |
| Browser smoke | `apps/web/e2e/flood-watch.smoke.mjs` plus a11y/no-leak checklist updates. Smoke must cover the human multi-action budget, a visible role-power difference, forecast reveal, the environment-phase effect animation, a win flow, a loss flow, the bot-teammate turn, bot-vs-bot cooperative replay, replay step/export/import, reduced motion, and no undrawn-deck data in DOM/storage/logs/test IDs. |
| CI | `.github/workflows/gate-1-game-smoke.yml` native smoke, replay, fixture, rule coverage, web build, and E2E registration; `.github/workflows/gate-2-benchmarks.yml` smoke and threshold registration. |
| Repository docs | `specs/README.md` Gate 12 row maintenance; `docs/MECHANIC-ATLAS.md` updates per the atlas deliverable; `progress.md` and root `README.md` after implementation; no ROADMAP progress edit. |

## Work breakdown

Bounded candidate AGENT-TASKs, in dependency order. Do not decompose these into ticket files as part of this spec.

| # | Candidate task | Depends on | Notes / forbidden drift |
|---:|---|---|---|
| 1 | Pre-implementation primitive-pressure decisions | — | Write `games/flood_watch/docs/PRIMITIVE-PRESSURE-LEDGER.md`: reaction-window second-use review (expected: not reaction-capable), deterministic-shuffle row review (expected: not a fifth use — no private holdings), and first-use records for shared outcome, event automation, role powers, and action budgets. Update atlas §10B; confirm §10A stays `_None_`. **Blocks all implementation tasks.** If either review contradicts its expected outcome, stop: the reaction-window row's hard-gate or the shuffle row's reopen fires before code is written. |
| 2 | Crate skeleton and workspace registration | 1 | Add `games/flood_watch` with IDs, both scenario variants, setup constants, manifest/variant parsers, fixture shells. No behavior in data: deck composition is typed counts of closed-enum event kinds. |
| 3 | State model, typed IDs, deterministic setup | 2 | Model districts with flood levels and levee stacks, the shuffled event deck (internal order; public remaining-composition counts), forecast marker, roles per seat, action budget, turn/phase, shared terminal outcome, freshness token. Deterministic shuffle with the existing `SeededRng` discipline per the task-1 ledger review. |
| 4 | Budgeted action phase: tree, validation, application | 3 | Active seat's tree: `bail/<district>`, `reinforce/<district>`, `forecast`, `end_turn`, each validated against budget, district existence, level/levee bounds, and forecast availability; tree regenerates with remaining-budget metadata after each action; teammate's tree is empty with safe waiting metadata; `end_turn` always present (no stall). Role powers applied in Rust application logic only. |
| 5 | Environment automation and event resolution | 4 | The command that spends the final budget point (or `end_turn`) triggers the environment phase inside the same deterministic application: draw N events, resolve each in order (levee absorption before level rise), stop immediately on inundation. All steps are grouped semantic effects. No synthetic actor, no TS sequencing. |
| 6 | Shared outcome and terminal | 5 | Lost the moment any district reaches the inundation level; Won when the final deck card resolves without loss. Terminal effect carries the shared outcome and a public summary (surviving levels, drawn-card count). No per-seat winner. |
| 7 | Visibility and replay surfaces | 6 | Public projection (identical for both seats and observers), effect filtering, internal full trace with deck order, viewer-scoped export/import with the undrawn deck redacted, stable summaries, action/effect/view hashes, no-leak helpers. ADR 0004 rules are mandatory. |
| 8 | Level 0 and Level 1 cooperative bots | 5,7 | Both roles, both seats: deterministic priority policy (rescue level-2 districts, mitigate forecast threats, reinforce by public expected-threat counting from remaining composition, forecast with spare budget, stable district-order tie-break). Legal action API only; public view + bot seed only; viewer-safe rationale; usable as human teammate and in bot-vs-bot replay. |
| 9 | Native tests and golden traces | 6,7,8 | Rule/property/replay/serialization/visibility/bot suite and the full golden trace set. Follow the failing-test protocol; never weaken tests to get green. |
| 10 | Benchmarks and thresholds | 9 | The benchmark identity list under Implementation reference. Smoke floors first, calibration follow-up named. |
| 11 | Per-game documentation | 9,10 | Instantiate all thirteen docs. `RULES.md` carries stable rule IDs including shared-outcome/end IDs for the outcome-explanation contract; `HOW-TO-PLAY.md` carries the required player-rules section set; `COMPETENT-PLAYER.md`/`BENCHMARKS.md` record the cooperative win-rate tuning evidence (Assumption A5). |
| 12 | WASM, tools, and CI registration | 9 | Register game ID across wasm-api, simulate, replay-check, fixture-check, rule-coverage, bench-report, seed-reducer/trace-viewer if needed, and CI lanes. Extend `scripts/boundary-check.sh`'s `mechanic_pattern` to add `role` and `scenario` — the two FOUNDATIONS §3 forbidden kernel nouns this gate is built around (currently the pattern covers `card`/`deck` but not these); evaluate adding `event` for word-boundary/substring false positives (`prevent`, `eventually`) before including it. `engine-core` is clean of all three today, so the extension must stay green on the existing tree. |
| 13 | React board and browser no-leak smoke | 12 | `FloodWatchBoard.tsx`, flood gauges, levee and deck displays, remaining-composition panel, forecast display, budget indicator, environment-phase animation from effects, shared-outcome explanation surface, player-rules copy + `HIDDEN_INFO_GAMES` registration, replay UI, E2E smoke, reduced motion, DOM/storage/test-ID no-leak assertions. TS remains presentation-only. |
| 14 | Repository documentation and final admission evidence | 11,12,13 | Spec index flip, atlas confirmation, progress/root README updates after implementation, catalog closeout surfaces, command transcript, unresolved issues. Do not edit ROADMAP as progress diary. |

## Exit criteria

Mapped row-for-row to ROADMAP §14 (Gate 12).

| ROADMAP §14 line | Gate 12 exit criterion |
|---|---|
| automation is deterministic and effect-log-driven | The environment phase resolves entirely inside Rust command application: the same seed, seats, scenario, and command stream reproduce the same draws, absorptions, rises, terminal outcome, and hashes. Every automation step (`EventDrawn`, `LeveeAbsorbed`, `FloodLevelRose`, `DistrictInundated`, `DeckExhausted`, `Terminal`) is a semantic effect; the browser animation and the replay viewer render only those effects, and no TypeScript timer, scheduler, or client-side sequencing participates in resolution. |
| shared outcome is tested | Rule tests cover the loss trigger (any district reaching the inundation level, including mid-environment-phase early stop) and the win trigger (final card resolved without loss); property tests assert the outcome is always shared (never per-seat), terminal is reached within the deck bound, and no post-terminal action is legal; golden traces pin one full win and one full loss; the outcome-explanation surface renders the team result from Rust-supplied data. |
| role powers stay game-local | Role IDs, labels, and modifiers exist only in `games/flood_watch` (typed Rust application logic plus static labels). `scripts/boundary-check.sh` plus kernel review confirm no `role`, `scenario`, or other Gate 12 noun enters `engine-core`, and no role/power helper enters `game-stdlib`. |
| UI explains event pressure clearly | The board shows per-district flood gauges and levees, the remaining deck count, the public remaining-composition panel, and the forecast card; environment effects animate in drawn order with log copy explaining each pressure step and its mitigation; the E2E smoke asserts the explanation surfaces render for draw, absorption, rise, and both terminal outcomes, with reduced-motion preserving order. |
| Not allowed: private licensed content | **Honored.** All names, labels, prose, and visuals are original and IP-reviewed per `docs/IP-POLICY.md`; `SOURCES.md` records the consulted-mechanics-only posture and the trade-dress avoidance register. |
| Not allowed: DSL by stealth | **Honored.** Event kinds are a closed typed Rust enum; scenario data declares counts and constants only; unknown and behavior-looking fields are rejected by tests. |
| Not allowed: architecture claims beyond proven games | **Honored.** No cooperative/automation/role/budget abstraction is promoted anywhere; every new shape is recorded as a first official local use in the atlas. |

## Acceptance evidence

### Native rules, replay, visibility, and bot evidence

- `cargo test -p flood_watch` passes.
- Rule tests cover deterministic setup for both scenarios, action legality for every action kind (bail level bounds, reinforce levee cap, forecast availability, end-turn), budget tracking and exhaustion, role-power application for both roles, event resolution order (levee absorption before rise), storm-surge double rises, the Reprieve no-op, mid-phase early stop on inundation, the shared loss and win triggers, and terminal immutability.
- Diagnostics tests cover stale freshness tokens, wrong-seat actions, out-of-budget submissions, bail on a dry district, reinforce at the levee cap, forecast when the top card is already revealed, and post-terminal submissions — all with viewer-safe messages that do not hint at the undrawn deck.
- Property tests cover many deterministic seeds and legal action sequences asserting: flood levels stay within bounds, levee counts stay within cap, the deck only shrinks and each card resolves exactly once, the action-phase tree always contains `end_turn` (no stall), the environment phase runs exactly once per turn, terminal occurs within the deck bound, the outcome is always shared, and no panics.
- Replay tests prove same seed + seats + scenario + command stream reproduces state hashes, effect hashes, action-tree hashes, view hashes, draw ordering, and the terminal outcome.
- Serialization tests prove stable summaries and unknown-field rejection for manifest, both scenario variants, fixtures, viewer-scoped export, and internal trace helpers.
- Visibility/no-leak tests search public views, action trees, previews, diagnostics, effect payloads, public effect text, command summaries, public export/import timelines, bot explanations, and candidate rankings for undrawn-deck order or identities — a drawn or forecast card's identity first appears in its `EventDrawn`/`ForecastRevealed` effect.
- Bot tests prove Level 0 and Level 1 select only legal action paths in both roles and seats, are deterministic under declared inputs, finish many cooperative games, and never change a decision or rationale when the hidden deck order differs but the public view is identical.
- Balance evidence: Level 1 + Level 1 cooperative simulation across both scenarios reports win rates; a result outside the target band (Assumption A5) triggers a scenario-constant retune before public polish, recorded in `BENCHMARKS.md`/`COMPETENT-PLAYER.md`.

### Golden traces

Committed under `games/flood_watch/tests/golden_traces/` and checked by `replay-check`:

- `standard-win.trace.json` (full match; deck exhausted, no loss)
- `loss-by-inundation.trace.json`
- `mid-phase-early-stop.trace.json` (inundation on the first of two draws)
- `levee-absorption.trace.json`
- `storm-surge-double-rise.trace.json`
- `reprieve-no-op.trace.json`
- `forecast-public-reveal.trace.json`
- `early-end-turn.trace.json`
- `budget-exhaustion-auto-environment.trace.json`
- `role-power-pumpwright.trace.json`
- `role-power-levee-warden.trace.json`
- `scenario-deluge-setup.trace.json`
- `wrong-seat-diagnostic.trace.json`
- `out-of-budget-diagnostic.trace.json`
- `bail-dry-district-diagnostic.trace.json`
- `public-observer-no-leak.trace.json`
- `bot-coop-full-game.trace.json` and `public-replay-export-import.trace.json`

### Tools, benchmarks, browser, docs, and CI

- `cargo run -p simulate -- --game flood_watch --games 1000` finishes with no illegal bot action or invariant failure.
- `cargo run -p replay-check -- --game flood_watch --all` passes.
- `cargo run -p fixture-check -- --game flood_watch` passes for both scenarios.
- `cargo run -p rule-coverage -- --game flood_watch` passes and maps every rules-doc obligation to tests/traces.
- `cargo bench -p flood_watch` runs the benchmark identity list; `bench-report` enforces smoke floors where calibrated, with the calibration follow-up named in the handoff.
- `npm --prefix apps/web run smoke:wasm`, `smoke:ui`, and `smoke:e2e` pass after `flood_watch` is registered.
- `node apps/web/e2e/flood-watch.smoke.mjs` covers the cooperative UI flows listed under Deliverables.
- `bash scripts/boundary-check.sh` passes with no new `engine-core` mechanic nouns and no TypeScript legality drift; its `mechanic_pattern` now includes `role` and `scenario` (and `event` if adopted) so the gate's headline kernel-noun risk is enforced mechanically in CI, not by review alone.
- `node scripts/check-doc-links.mjs` passes.
- `node scripts/check-catalog-docs.mjs` passes, confirming the mechanically-checked catalog surfaces name `flood_watch` / `Flood Watch`.
- `node scripts/check-player-rules.mjs` passes with `flood_watch` in `HIDDEN_INFO_GAMES` and the generated `apps/web/public/rules/flood_watch.md` in sync.
- `node scripts/check-outcome-explanations.mjs` passes with the flood-watch shared-outcome templates and rule-ID mirrors registered.
- All thirteen per-game docs are present, link-checkable, original, and consistent with implemented behavior.

## FOUNDATIONS & boundary alignment

| Principle | Stance | Rationale |
|---|---|---|
| §2 Behavior authority | Aligned | Rust owns setup, shuffle, action legality, budget tracking, role-power application, environment automation, event resolution order, terminal detection, the shared outcome, semantic effects, replay/export projection, and bot decisions. React presents Rust-provided data, including the teammate waiting state and the automation animation. |
| §3 `engine-core` contract kernel | Aligned | The environment phase is a game-local consequence of normal command application expressed through existing generic `Actor`/`Viewer`/`ActionTree`/`CommandEnvelope`/`EffectEnvelope`/`VisibilityScope`/replay contracts. `event`, `deck`, `role`, `scenario`, `district`, `flood`, `levee`, `budget`, and `cooperative` nouns remain game-local; `role` and `scenario` are explicitly forbidden kernel nouns and this gate's forbidden-changes list restates them. |
| §4 `game-stdlib` earned | Aligned | Nothing is promoted. Shared outcome, event automation, role powers, and action budgets are all first official uses recorded `local-only` in the atlas; the work-item-1 reviews confirm neither the reaction-window second-use trigger nor the shuffle-row fifth-use reopen fires (or stop the gate if they do). |
| §5 Static data is typed content | Aligned | Manifest/variants/fixtures hold district IDs, labels, scenario constants, deck composition **counts** of closed-enum event kinds, budgets, and caps only. What an event does, when the environment runs, and how roles modify actions are typed Rust. Unknown and behavior-looking fields are rejected. This is the gate's highest-risk boundary — event decks invite data-driven triggers — and the forbidden-changes list bans them outright. |
| §6 Official games are evidence-heavy | Aligned | The Acceptance-evidence section carries the full official-game contract: rule/property/replay/serialization/visibility/no-leak/bot tests, the golden trace set, `simulate`/`replay-check`/`fixture-check`/`rule-coverage` runs, benchmarks, and all thirteen per-game docs — browser playability alone is not treated as done. |
| §7 Public UI is central product work | Aligned | `FloodWatchBoard.tsx` renders legal-only budgeted controls (actions appear only when Rust's tree contains them; the teammate gets a waiting state), the environment animation is driven by Rust semantic effects with reduced-motion support, and the renderer settles to the latest viewer-safe view. TypeScript invents no legality and runs no automation. |
| §8 Public bots | Aligned | Cooperative bots act through the same legal action API as humans, with a deterministic public-counting priority policy, viewer-safe explanations, and no deck-order access, MCTS/ISMCTS, Monte Carlo, ML, or RL. The teammate mode changes nothing about the API or fairness posture. |
| §9 Local-first v1/v2 | Aligned | Cooperative play is hotseat and human-plus-bot; bot-vs-bot replay is supported. No accounts, hosting, matchmaking, persistence, or network play. Command logs, deterministic replay, and viewer-scoped export keep the future-hosted path open. |
| §10 IP conservatism | Aligned | Cooperative event-pressure mechanics are unprotected rules territory; the IP-risk register names the adjacent commercial designs and the specific trade-dress hazards to avoid; all names, labels, prose, and visuals are original; `SOURCES.md` records consulted sources and the originality rationale. |
| §11 Universal invariants | Aligned | Deterministic replay/hash/serialization, viewer-safe views, the never-leaked deck order, legal-only UI, semantic-effect animation, local-first scope, docs/traces/simulations/benchmarks, and bounded agent output are explicit acceptance evidence. |
| §12 Stop conditions | Clear | Stop if a kernel noun appears, scenario data turns procedural (event scripting in data is "static files start acting procedural"), TypeScript decides legality or runs automation, the deck order reaches a browser/dev/replay/bot surface, a bot reads the deck, a reviewed atlas expectation is contradicted (reaction window or shuffle fifth use), or a cooperative/automation helper is generalized without ledger pressure. |
| §13 ADR triggers | **No ADR expected.** | All shapes stay game-local; visibility/replay contracts, kernel vocabulary, data policy, and renderer defaults are unchanged. If the work-item-1 reviews escalate, or implementation genuinely requires changing replay/hash semantics or visibility contracts, stop and write the ADR before proceeding. |
| ADR 0004 hidden-info replay/export | Aligned | Internal full traces (with deck order) remain native test authority. Browser export defaults to the viewer-scoped observation timeline: undrawn cards never appear; each card appears from its `EventDrawn`/`ForecastRevealed` effect onward. |
| Benchmark ADRs 0002/0003/0005 | Aligned | Smoke benchmarks and threshold files now (`baseline_pending_non_blocking`, matching the masked-claims pattern); variance-aware calibration follow-up named once repeated CI measurements exist; PR smoke stays non-gating. |
| ADR 0006 Blackjack placement | Aligned | Untouched. Gate 12 is not a draw/stand or casino-comparison trigger. |

## Forbidden changes

Do not, in this gate:

- Add `event`, `deck`, `card`, `role`, `scenario`, `district`, `flood`, `levee`, `budget`, `environment`, `cooperative`, `shared outcome`, or similar mechanic/domain vocabulary to `engine-core`.
- Add generic `game-stdlib` primitives for event decks, automation phases, role powers, action budgets, shared outcomes, cooperative bot policy, or deck-composition counting. Every Gate 12 shape is a first official use; no promotion is authorized by this spec.
- Skip, reorder after implementation, or rubber-stamp the work-item-1 primitive-pressure reviews.
- Introduce behavior-in-data: event effects, triggers, selectors, conditions, formulas, scripted sequences, re-shuffle escalation rules, or untyped nested behavior objects in any manifest, variant, or fixture. No YAML, no DSL.
- Let TypeScript compute legality, budget state, environment resolution, event ordering, terminal outcome, outcome explanation authority, replay authority, bot policy, or no-leak filtering — or drive automation with client-side timers or sequencing.
- Leak the undrawn event-deck order or identities through any surface enumerated in the Exit criteria and no-leak tests, including post-terminal views and exports.
- Use MCTS, ISMCTS, Monte Carlo, ML, RL, LLMs, hidden-state sampling, deck peeking, or omniscient mitigation in any bot.
- Copy cooperative-game rules prose, names, role names, card text, icons, art, screenshots, or trade dress; do not use the named hazard vocabulary in the IP-risk register for public-facing labels.
- Add hosted multiplayer, accounts, server authority, matchmaking, chat, ranked play, persistence, timers, or real-time pressure.
- Add graph topology, adjacency, flood spread, or player movement — Gate 13 owns graph pressure.
- Change trace schema, replay/hash semantics, data versions, action path stability, effect ordering, public-view shape, or golden traces accidentally.
- Delete, rename away, weaken, or rewrite tests merely to get green output. Follow the failing-test protocol.

## Documentation updates required

- `specs/README.md`: Gate 12 row points at the archived spec with status `Done` after exit criteria passed with evidence.
- Do **not** edit `docs/ROADMAP.md` to record progress.
- `docs/MECHANIC-ATLAS.md`:
  - §10B `reaction window/pending response`: record the work-item-1 review outcome (expected: `flood_watch` is not a second reaction-capable use — no seat responds to another seat's pending action and the environment phase is automation, not a response window; the second-use trigger stays armed for Gate 13/14 event games). If implementation contradicts this, the row's review fires before proceeding.
  - §10B `deterministic shuffle / private hand / staged reveal`: record the work-item-1 review outcome (expected: not a fifth use — `flood_watch` has deterministic shuffle and staged reveal but no per-seat private holdings; record any new shuffle-helper evidence the row's "or earlier if" clause asks for).
  - §10B new rows for `shared-outcome cooperative terminal`, `event-deck environment automation`, `role-modified action effects`, and `multi-action turn budgets`: record each as `local-only` first official use with a second-use revisit trigger naming the Gate 13/14 candidates (`frontier_control` faction powers and asymmetric victory; `event_frontier` event decks and periodic automation are the expected second-pressure sources).
  - Confirm §10A stays `_None_`.
- Author all thirteen `games/flood_watch/docs/*` from templates and keep them consistent with implemented behavior; `SOURCES.md` records consulted prior art at implementation time (see the IP-risk register under Implementation reference), what was and was not used, and the originality/naming review.
- Update `progress.md` and root `README.md` after implementation, not before evidence passes.
- Reconcile the web-shell catalog surfaces as a closeout inside this gate (per `specs/README.md` §10), not a later aftermath pass:
  - `apps/web/README.md` intro catalog list (add `flood_watch` / `Flood Watch`);
  - root `README.md` "current official games are" list;
  - `apps/web/README.md` Smoke Layers `smoke:e2e` bullet, and the `smoke:e2e` script in `apps/web/package.json` (add `node e2e/flood-watch.smoke.mjs`);
  - the `apps/web/README.md` Shell Surface renderer list (manual closeout surface — `check-catalog-docs.mjs` deliberately does **not** mechanically check it; the script covers only the intro catalog list, the root `README.md` games list, and the `smoke:e2e` bullet).
  - `node scripts/check-catalog-docs.mjs` enforces the mechanically-checked surfaces in CI; it must pass.
- Player-rules and outcome-explanation surfaces: `games/flood_watch/docs/HOW-TO-PLAY.md` → `apps/web/public/rules/flood_watch.md` via `scripts/copy-player-rules.mjs`; `flood_watch` added to `HIDDEN_INFO_GAMES` in `scripts/check-player-rules.mjs`; shared-outcome templates/rule-ID mirrors registered so `scripts/check-outcome-explanations.mjs` passes.
- Ensure `scripts/check-doc-links.mjs` passes after doc changes.

## Sequencing

- **Predecessor:** Gate 11 (`masked_claims`) is `Done` in the spec index with evidence; the atlas §10A promotion-debt register records `_None_`, so no maintenance interlock precedes this gate.
- **Admission rule:** Before implementation starts, confirm `docs/MECHANIC-ATLAS.md` §10A still has no open promotion debt (it records `_None_` at spec time), and complete work item 1's ledger reviews before any shuffle, automation, or visibility code is written.
- **This gate:** Gate 12 proves shared win/loss, event deck pressure, role powers, environment automation, multi-action budgets, scenario setup, and a cooperative bot baseline per ROADMAP §14, with deterministic effect-log-driven automation as the headline architectural proof.
- **Successor:** Gate 13 (`frontier_control`) remains not yet specced. It should not start until Gate 12 evidence passes and no open promotion debt remains. The Gate 13 spec author should check the atlas rows this gate touches: `frontier_control` faction powers are the expected second use of role-modified action effects, and its asymmetric victory is comparison pressure for the shared-outcome row; graph topology arrives there deliberately unburdened because this gate kept districts adjacency-free.

## Assumptions

- A1: Two seats are the sufficient proof shape for Gate 12; more seats or solo play add coordination/quarterbacking design questions without strengthening the shared-outcome, automation, role, or budget proofs.
- A2: Public display name `Flood Watch`, the district labels (`Riverside`, `Old Docks`, `Market`, `Terraces`, `Gardens`), the role names (`Pumpwright`, `Levee Warden`), and the event names (`Downpour`, `Storm Surge`, `Reprieve`) are original placeholders; maintainers may rename after IP review before implementation. `Engineer` is deliberately avoided as a role name (Forbidden Island role).
- A3: Standard scenario uses five districts with flood levels 0–3 (3 = inundated, shared loss), a levee cap of 2 per district, a 24-card deck (per-district Downpours and Storm Surges plus Reprieves, composition in typed data), a 3-action budget, and 2 draws per environment phase (twelve turns, six per seat). The deluge scenario raises starting levels and surge counts. Maintainers may tune counts while preserving the fixed deck-bound terminal and proof scope.
- A4: Role powers are public deterministic action modifiers — the Pumpwright's bail removes 2 levels, the Levee Warden's reinforce places 2 levees — applied in Rust application logic. Roles are fixed per seat by scenario data; selectable role assignment is later product polish, not part of this proof.
- A5: The cooperative balance target is a Level 1 + Level 1 win rate between roughly 35% and 75% on the standard scenario (tense but winnable), measured by simulation; results outside the band trigger a scenario-constant retune recorded in `COMPETENT-PLAYER.md`/`BENCHMARKS.md` before public polish.
- A6: The environment phase resolves inside the application of the turn-ending command (final budget point or `end_turn`) — no synthetic environment actor appears in the command stream. This keeps command-stream replay semantics identical to every prior game.
- A7: The undrawn deck order is hidden from all seats and never leaks; the public remaining-composition counts (scenario composition minus drawn cards) are derivable public information and may be surfaced to players, bots, and the UI. Forecast reveals the top card publicly to both seats; there is no per-seat hidden information anywhere in the game.
- A8: `flood_watch` is not reaction-capable and is not a fifth use of the deterministic-shuffle triple; work item 1 verifies both expectations against the atlas before implementation and stops the gate if either fails.
- A9: Level 0 + Level 1 bots satisfy the gate (mirroring Gates 9.1–11); a Level 2 claim requires the full `BOT-STRATEGY-EVIDENCE-PACK.md` evidence workflow.
- A10: No external research pass was run for this spec (unlike Gate 11's research-backed design); the IP-risk register and the anti-quarterbacking/balance notes draw on the maintainers' review at implementation time, and `SOURCES.md` must record whatever is actually consulted. If deeper prior-art research is wanted before implementation, run it before decomposition.
- A11: If any assumption is wrong, correct the smallest local game/spec surface; do not generalize the engine.

---

# Implementation reference

## Product intent / what this gate proves

`Flood Watch` is a deliberately compact original cooperative game. Two wardens defend a riverside town through one storm: each turn a warden spends a small action budget bailing water and raising levees, then the storm answers — the environment draws event cards from a shuffled deck and floods districts deterministically. Everyone wins together if the town survives the whole deck; everyone loses together the moment any district goes under.

Architecturally, the gate makes the platform prove three new shapes cleanly:

- **The environment acts.** Every prior game's state changed only through a seat's command. Here, a deterministic automation phase resolves multiple event steps after each turn — and it must be replayable from the same command stream, animated only from semantic effects, and owned entirely by Rust. The chosen mechanism (automation as a consequence of the turn-ending command, Assumption A6) is the smallest design that preserves the proven command/replay contract.
- **The outcome is shared.** Terminal detection produces a team result, not a per-seat winner — a new shape for terminal state, outcome explanation templates, and the victory surface.
- **Turns are budgets.** A seat submits several validated commands per turn against a tracked budget, with the legal tree regenerating between them — the multi-action shape Gates 13–14 will need at larger scale.

## Proposed original rules: `Flood Watch`

### Components

- Two seats: `seat_0` (role `pumpwright`), `seat_1` (role `levee_warden`).
- Five districts, stable IDs `district_riverside`, `district_old_docks`, `district_market`, `district_terraces`, `district_gardens`, each with a flood level `0..=3` (0 dry, 1 soaked, 2 swamped, 3 inundated = shared loss) and a levee stack `0..=2`.
- A 24-card event deck, deterministically shuffled at setup, face-down, order hidden from everyone. Standard composition (typed counts in scenario data): `Downpour { district }` ×3 per district (15; +1 rise), `StormSurge { district }` ×1 per district (5; +2 rise), `Reprieve` ×4 (no effect; "the rain slackens").
- A forecast marker (the top card may be publicly revealed).
- A shared outcome (`Won` / `Lost`), turn marker, action budget, and freshness token.

### Setup

1. Validate exactly two seats; assign roles from scenario data.
2. Load the scenario constants (`flood_watch_standard` or `flood_watch_deluge`) from typed Rust/validated static metadata: starting flood levels, deck composition counts, budget, draws per phase, levee cap.
3. Deterministically shuffle the event deck with `SeededRng` per the work-item-1 ledger review.
4. Set turn 1 (active seat `seat_0`; seats alternate), full action budget, no forecast, terminal `None`, freshness token `0`.
5. Emit setup/view state through Rust projection only. No projection contains the deck order.

### Turn flow

1. **Action phase.** The active seat holds a budget of 3 actions and submits commands one at a time; the tree regenerates with remaining-budget metadata after each:
   - `bail/<district>` — reduce that district's flood level by 1 (Pumpwright: by 2, floor 0). Legal only if the level is ≥ 1.
   - `reinforce/<district>` — add 1 levee to that district (Levee Warden: 2, capped). Legal only below the levee cap.
   - `forecast` — publicly reveal the top card of the event deck to both seats (it stays on top, marked revealed). Legal only while the top card is unrevealed.
   - `end_turn` — forfeit remaining budget and proceed to the environment phase. Always legal during the action phase, so the tree is never empty.
   The teammate's tree is empty with safe waiting metadata. Spending the final budget point triggers the environment phase exactly as `end_turn` does.
2. **Environment phase** (deterministic automation inside the turn-ending command's application): draw 2 events from the top of the deck and resolve each in order:
   - `Downpour`/`StormSurge`: levees absorb first — consume up to the rise amount from the district's levee stack (`LeveeAbsorbed`), then any remainder raises the flood level (`FloodLevelRose`).
   - If a district reaches level 3: `DistrictInundated`, resolution stops immediately (remaining draws do not occur), and the match ends `Lost`.
   - `Reprieve`: no state change beyond the draw (`EventDrawn` with a calm rendering).
   - If the deck empties with no loss: `DeckExhausted`, and the match ends `Won`.
3. **Cleanup.** The active seat alternates, the budget refills, the forecast marker clears if the revealed card was drawn, and the next turn begins unless terminal.

### Terminal

- **Lost:** the moment any district reaches level 3, including mid-environment-phase.
- **Won:** the final deck card resolves without a loss.

The `Terminal` effect carries the shared outcome, surviving flood levels, and a public summary for the outcome-explanation surface. There are no per-seat scores and no tie-breaks.

### Why these constants resist degeneracy

Total incoming pressure on the standard scenario is 25 rise points (15 Downpours + 5 double Storm Surges) across 12 turns. Total mitigation capacity is 36 actions, but role efficiency matters: unaided bailing costs one action per rise point, while role-matched actions (Pumpwright bails, Warden reinforces) handle two points per action — the team wins comfortably only by exploiting both roles, and a team that ignores roles or wastes budget loses ground at roughly one district level per round. Forecast converts spare budget into planning information without changing the pressure arithmetic. The deluge scenario tightens the margin (higher starting levels, more surges). Constants are tunable under Assumption A5 with simulation evidence; the win-rate band, not the exact numbers, is the requirement.

## State, actions, and validation sketch

### State

- `variant: Variant` (scenario)
- `seats: [SeatId; 2]`, `roles: [RoleId; 2]` (from scenario data)
- `turn_number: u8`, `active_seat: SeatId` (alternates)
- `phase: Phase` — `Phase::Action { budget_remaining: u8 }`, `Phase::Terminal`
- `districts: [DistrictState; 5]` — `flood_level: u8 (0..=3)`, `levees: u8 (0..=cap)` — all public
- `event_deck: Vec<EventCard>` internal order only; `drawn: Vec<EventCard>` public; `forecast: Option<EventCard>` public once revealed
- `remaining_composition` is derived (scenario counts minus drawn) and public
- `terminal_outcome: Option<SharedOutcome>` (`Won` / `Lost { district }`), `freshness_token: FreshnessToken`

There is no `Phase::Environment` a seat could act in: the environment resolves atomically inside the turn-ending command's application (Assumption A6), and the next observable phase is the teammate's action phase or terminal.

### Legal action tree

- Action phase, active seat: `bail/<district>` for each district with level ≥ 1; `reinforce/<district>` for each district below the levee cap; `forecast` while the top card is unrevealed; `end_turn` always. Choice metadata carries remaining budget, the role's effect magnitude (Rust-supplied preview: "removes 2 levels"), and public district facts only.
- Action phase, teammate: flat empty tree with safe waiting metadata naming who acts and why.
- Terminal: empty trees.

### Validation

Rejects: stale freshness token; actor not seated; non-active seat; terminal phase; bail on a dry district; reinforce at the levee cap; forecast when already revealed; malformed/extra path segments; any submission with zero remaining budget (unreachable if the tree is honored, still validated). Diagnostics are viewer-safe ("it is your teammate's turn", "that district's levees are at full height") and never reference the undrawn deck.

## Semantic effect model

| Effect | Visibility | Payload rule |
|---|---|---|
| `DistrictBailed { district, by_seat, role, amount, new_level }` | Public | Role magnitude is public knowledge. |
| `LeveePlaced { district, by_seat, role, amount, new_count }` | Public | — |
| `ForecastRevealed { card }` | Public | First public appearance of the top card; both seats see it. |
| `TurnEnded { seat, unspent_budget }` | Public | — |
| `EnvironmentPhaseBegan { turn, draws }` | Public | Announces the automation batch. |
| `EventDrawn { index, card }` | Public | First public appearance of a drawn card (unless forecast already revealed it). |
| `LeveeAbsorbed { district, amount, remaining_levees }` | Public | Absorption renders before any rise. |
| `FloodLevelRose { district, amount, new_level }` | Public | — |
| `DistrictInundated { district }` | Public | Triggers the early stop. |
| `DeckExhausted` | Public | Precedes a `Won` terminal. |
| `Terminal { outcome, summary }` | Public | Shared outcome plus the public summary; no deck data beyond drawn cards. |
| `PrivateDiagnostic` / `PublicDiagnostic` | Private / public | Viewer-safe reasons only. |

React animates the environment batch in effect order (draw flip, levee shrink, gauge rise, inundation, terminal) and settles to the latest viewer-safe view. Reduced motion preserves event order and the explanation copy.

## Visibility and no-leak model

- **Single public projection:** both seats and observers receive the same view — districts, levees, budget, roles, drawn cards, forecast card if revealed, remaining-composition counts, deck size, turn/phase, terminal outcome. There is no per-seat private view content in this game.
- **Hidden from everyone:** the undrawn event-deck order and identities. A card's identity first appears in its `ForecastRevealed` or `EventDrawn` effect. Remaining-composition *counts* are public derived data and are not a leak.
- **Internal full trace vs browser export (ADR 0004):** internal native traces carry the seed and full deck order as test authority. Viewer-scoped browser export carries commands and public effects only; undrawn cards never appear in any export at any point, including post-terminal.
- The auxiliary-surface rule is mandatory: undo/history affordances, dev panels, effect logs, fixtures, screenshots, and E2E anchors all draw from the same viewer-scoped projection. Fixture files must not embed the shuffled order in any publicly-served artifact.

## Bot policy

### Level 0

`FloodWatchRandomBot` selects uniformly from the legal tree using deterministic bot RNG helpers, in either role and seat. It never constructs out-of-tree actions and never reads the deck.

### Level 1

`FloodWatchLevel1Bot`, deterministic under declared inputs (public view + bot seed), with viewer-safe explanations, playing either role as teammate or in bot-vs-bot replay:

1. **Rescue:** if any district is at level 2, bail the most threatened one (highest expected incoming rise by public composition counting; stable district-order tie-break). Explanation: "Bailed the Old Docks: it was one rise from going under."
2. **Forecast mitigation:** if the revealed top card would inundate a district this coming phase, bail or reinforce that district (whichever the role does more efficiently).
3. **Reinforce by expected threat:** place levees on the district with the highest expected incoming rise among those below the cap, preferring the role-efficient action.
4. **Forecast:** with spare budget and an unrevealed top card, forecast.
5. **End turn** when no action improves the position.

Forbidden explanations: anything referencing undrawn card identities or order. Bot tests assert decisions and rationales are unchanged when the hidden deck order differs but the public view is identical.

## WASM/browser wiring

- Catalog entry with `game_id: flood_watch`, display name `Flood Watch`, cooperative flag in game metadata, hidden-information flag (deck order), viewer modes, both scenario variants, and docs links.
- `get_view(match_id, viewer_seat)` returns the public projection per the visibility model. `get_action_tree(match_id, actor_seat)` returns budgeted choices or the empty waiting tree per phase and seat.
- `apply_action` validates and returns safe effects + view; a turn-ending action's result carries the full environment batch.
- `run_bot_turn` routes the bot's several budgeted actions through the legal tree and validation; decision JSON is viewer-safe.
- `export_replay` defaults to the viewer-scoped observation export with the undrawn deck redacted.
- `FloodWatchBoard.tsx` renders the five district gauges with levees, the deck count and remaining-composition panel, the forecast display, the budget indicator, role cards, effect log, and replay controls. Action controls render only what Rust's tree contains; the teammate sees waiting copy. Anchors use district/turn IDs, never deck data.
- Shared-outcome explanation templates ("The watch held — every district stayed above water." / "The flood took the Old Docks on turn 9.") register on the outcome-explanation surface; this is that surface's first *cooperative team* outcome (the existing `*_draw` templates already render winner-free results with `requiredParams: []`, so no `OutcomeExplanationTemplate` change is needed), and the work item confirms the team templates render without a per-seat winner.

## Benchmark operations

- `legal_actions_action_phase`
- `validate_action`
- `apply_bail`
- `apply_reinforce`
- `apply_end_turn_environment_phase` (the automation hot path)
- `project_public_view_midgame`
- `state_hash_terminal`
- `public_export_timeline`
- `level1_bot_decision`
- `random_playout` (full cooperative games per second)

Thresholds start as non-blocking smoke floors (`baseline_pending_non_blocking`, matching the masked-claims pattern) with a named calibration follow-up under ADR 0002/0003/0005 once repeated CI measurements exist.

## IP-risk register and source guidance

No external research pass backs this spec (Assumption A10); this register names the adjacent commercial designs so implementation steers clear of their protected expression while freely using unprotected mechanics:

- **Forbidden Island / Forbidden Desert (Gamewright, Matt Leacock):** the closest published shape — rising water, shoring up locations, role powers. Avoid: island/tile vocabulary, "shore up"/"sandbag" labels, the named roles (`Engineer`, `Pilot`, `Diver`, `Messenger`, `Explorer`, `Navigator`), water-meter trade dress, and treasure-collection framing. Flood Watch has no tiles, no pawns, no movement, no treasures, and original role names.
- **Pandemic (Z-Man, Matt Leacock):** avoid outbreak/epidemic/infection vocabulary, the intensify re-shuffle escalation mechanic as a recognizable combination, and chained spreading between connected locations (also excluded because graph topology is Gate 13's proof).
- **Spirit Island, Flash Point: Fire Rescue:** more distant; avoid their named role/power text and board trade dress generally.
- Mechanics themselves — cooperative shared outcome, event decks, action points, role asymmetry — are unprotected rules territory (the same rules-vs-expression boundary the Gate 11 spec recorded via the U.S. Copyright Office games circular and *DaVinci Editrice v. ZiKo Games*).

`games/flood_watch/docs/SOURCES.md` must record what was actually consulted at implementation time, what was used and not copied, why every name and label is original, and asset/font status. If any label feels trademark-forward or trade-dress-adjacent at review time, rename it before implementation.

## Outcome

Completed on 2026-06-11.

Gate 12 shipped `flood_watch` / Flood Watch as the cooperative event-pressure proof. The completed gate includes the game crate, typed standard and Deluge scenarios, deterministic hidden event-deck setup, Rust-owned action budgets and environment automation, public forecast/draw reveal, role-modified bail/reinforce effects, shared win/loss terminal rationale, Level 0 and Level 1 cooperative bots, native tests and golden traces, benchmarks, tool and CI registration, WASM/API bridge registration, browser board, player-rules and outcome-explanation surfaces, catalog documentation, and browser E2E no-leak coverage.

The mechanic-atlas reviews completed as planned: Flood Watch is not reaction-capable, is not a fifth full deterministic-shuffle/private-hand/staged-reveal use because it has no per-seat private holdings, and records first official local uses for shared-outcome cooperative terminal, event-deck environment automation, role-modified action effects, and multi-action turn budgets. `docs/MECHANIC-ATLAS.md` §10A remains `Current debt: _None_`.

Final verification passed:

- `cargo test --workspace`
- `cargo run -p simulate -- --game flood_watch --games 1000`
- `cargo run -p replay-check -- --game flood_watch --all`
- `cargo run -p fixture-check -- --game flood_watch`
- `cargo run -p rule-coverage -- --game flood_watch`
- `bash scripts/boundary-check.sh`
- `node scripts/check-doc-links.mjs`
- `node scripts/check-catalog-docs.mjs`
- `node scripts/check-player-rules.mjs`
- `node scripts/check-outcome-explanations.mjs`
- `npm --prefix apps/web run smoke:wasm`
- `npm --prefix apps/web run smoke:ui`
- `npm --prefix apps/web run smoke:effects`
- `npm --prefix apps/web run smoke:e2e`

No unresolved blocking issues remain. The completed tickets are archived under `archive/tickets/GAT12FLOWATCOO-001.md` through `archive/tickets/GAT12FLOWATCOO-019.md`, and this spec is archived at `archive/specs/gate-12-flood-watch-cooperative-event-pressure-proof.md`.

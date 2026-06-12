# Gate 14 Event Frontier Event-Complexity Capstone Proof

| Field | Value |
|---|---|
| Spec ID | `gate-14-event-frontier-event-complexity-capstone` |
| Roadmap stage | 14 |
| Roadmap build gate | Gate 14 — highest public complexity before private red-team |
| Status | Done |
| Date | 2026-06-12 |
| Owner | Rulepath maintainers |
| Primary crate / internal game id | `event_frontier` |
| Public display name | `Event Frontier` unless IP review prefers another neutral original name |
| Browser implementation | Required |
| Authority order | `docs/FOUNDATIONS.md` → `docs/ARCHITECTURE.md` → `docs/ENGINE-GAME-DATA-BOUNDARY.md` → `docs/OFFICIAL-GAME-CONTRACT.md` → `docs/MECHANIC-ATLAS.md` → `docs/AI-BOTS.md` → `docs/UI-INTERACTION.md` → `docs/TESTING-REPLAY-BENCHMARKING.md` → `docs/ROADMAP.md` → accepted ADRs that explicitly supersede those documents → this spec |

Where this spec and a foundation document disagree, the foundation document wins.

> Reader orientation: this spec carries the canonical Rulepath section set: Objective, Scope, Deliverables, Work breakdown, Exit criteria, Acceptance evidence, FOUNDATIONS & boundary alignment, Forbidden changes, Documentation updates required, Sequencing, and Assumptions. Detailed proposed rules, state/effect/view sketches, bot policy, WASM/browser wiring, IP/source guidance, and the external-research notes are preserved below the canonical sections under **Implementation reference**.

## Objective

Implement `event_frontier` as the Gate 14 browser proof for the ROADMAP §14 exit lines: **public Rulepath stands without private experiments; action trees remain usable; scripted bots are demo-coherent; every repeated mechanic has ledger resolution** — proving, per the ROADMAP stage 14 ladder row and §14 purpose, **event decks with exceptions, eligibility/initiative, periodic scoring/reset, asymmetric victory, large action trees, scripted policy bots, scenario setup, and long-game replay/debug tools**.

Gates 1–13 proved flat actions, boards, compound trees, hidden information, resources, commitment/reveal, betting, tricks, reaction windows, cooperative environment automation, graph maps, and faction asymmetry. This capstone gate combines the remaining unproven shapes into one original game before any Gate P work is admissible:

- **an event deck drives the whole match**: a deterministic seeded deck of typed event cards sequences play, and individual cards impose bounded, expiring **rule exceptions** (edicts) — the single highest data/Rust-boundary risk surface on the roadmap, handled as typed Rust effects with typed-parameter card data and nothing rule-like in any file;
- **eligibility/initiative**: each card names a first-eligible faction; the first eligible's choice (event / operation / pass) constrains the second's menu; acting costs eligibility on the next card; passing preserves it and pays — a fully deterministic, sequential, timeout-free initiative system with no simultaneity and no reaction windows;
- **periodic scoring/reset**: scoring cards ("Reckonings") seeded one per deck epoch resolve an ordered victory-check → site-scoring → income → reset pipeline, with edict expiry and eligibility restoration as the reset semantics;
- **asymmetric victory**: the two factions win by different instant conditions checked at each Reckoning, with a deterministic comparable-score fallback only when the final Reckoning resolves with neither met — the proof Gate 13 explicitly deferred here;
- **large action trees**: operations are compound multi-site actions (op type → site set bounded by the card's ops value → per-site sub-choice), built through the existing progressive-construction contracts at the largest branching factor of any official game;
- **scripted policy bots**: per-faction deterministic two-layer bots — a decision table for event-vs-operation-vs-pass keyed on public state, then ordered site-priority lists with total-order tiebreaks — selecting only from the engine's legal action tree;
- **scenario setup**: three typed scenarios varying starts, resources, thresholds, and deck epoch composition — content, never behavior;
- **long-game replay/debug**: the longest command timeline of any official game, with replay markers, export/import under the hidden-deck-order taxonomy, and simulation metrics that report per-faction win rates *and* victory-type frequencies.

This is not an engine-generalization gate. Every event, card, deck, initiative, eligibility, edict, Reckoning, site, depot, cache, and faction noun stays local to `games/event_frontier`. Two armed third-use hard gates are resolved by ledger **before** implementation: the multi-action-turn-budget candidate (expected outcome: non-use — see Scope) and the public-resource-accounting third use (a genuine hard gate requiring a recorded decision).

## Scope

### In scope

- New official game crate `games/event_frontier` with typed Rust setup, state, card sequencing, eligibility/initiative flow, operations, event effects, edict modifiers, Reckoning pipeline, asymmetric victory, terminal detection, effects, visibility projection (hidden deck order), replay support, scenarios, UI metadata, and bots.
- Original deterministic rules (full proposal under Implementation reference): two factions — the **Charter** (`faction_charter`, institutional side: agents, depots, funds) and the **Freeholders** (`faction_freeholders`, independent side: settlers, caches, provisions) — on a fixed six-site/eight-trail graph; a 21-card event deck (18 events including 4 edicts, 3 Reckonings) in three seeded epochs; COIN-style first/second-eligible card flow; compound operations funded by resources; instant asymmetric victory conditions at Reckonings with a deterministic final fallback.
- **Pre-implementation atlas work (work item 1)** — the gate's heaviest ledger slate, all before implementation code is written:
  - **Hard gate — public resource accounting (third use).** `token_bazaar` and `poker_lite` are recorded first/second uses (`docs/MECHANIC-ATLAS.md` §10/§10B). Op funding, pass income, and Reckoning income make `event_frontier` the third official economy/accounting use; the ledger MUST decide reuse / promote / defer-reject / ADR before proceeding. Expected: defer/reject — per-faction op funding differs structurally from market purchase accounting and pot/showdown allocation; record rationale, risk, and next trigger.
  - **Hard-gate candidate — multi-action turn budgets (named third-use candidate, §10B).** Expected: **non-use** — each faction takes exactly one choice per event card; an operation is one compound command resolved through the action tree, not a budgeted sequence of validated commands with tree regeneration. Record the non-use rationale. If design drift introduces budgeted turns, the hard gate fires and the ledger must decide before any code.
  - Second-use comparisons, kept local per the atlas rule: **event-deck shape** vs `flood_watch` environment automation (player-facing card sequencing plus automated Reckoning resolution vs pure environment batch); **graph-map topology / adjacency legality** vs `frontier_control`; **site control** vs `frontier_control` (presence majority without contest/clash resolution — expected related-but-distinct); **faction-asymmetric action sets and scoring** vs `frontier_control`; **role/faction modifiers** vs `flood_watch`/`frontier_control` (edicts are temporal event-imposed modifiers, not seat-carried role powers — expected related-but-distinct).
  - Reviews recording expected non-uses: **reaction window** (eligibility menus are normal sequential turns; no pending response — the row stays armed); **shared-outcome cooperative terminal** (competitive); **deterministic shuffle / private hand** (shuffle yes, but no per-seat private holdings — the fifth-use trigger stays untouched, mirroring the `flood_watch` review).
  - Mandatory promoted-primitive audit: `game-stdlib::board_space` (expected: **not applicable** — graph sites, no rectangular coordinates; per OFFICIAL-GAME-CONTRACT §7 the audit or an accepted exception is mandatory).
  - New first-use rows recorded `local-only`: event-card initiative/eligibility sequencing; periodic scoring/reset pipeline; asymmetric instant victory conditions; timed rule-exception modifiers (edicts).
- Hidden information: exactly one hidden surface — **undrawn deck order**, hidden from all seats and observers alike. The current and next cards are public; discards are public; everything else on the table is public. `HIDDEN_INFO_GAMES` registration, ADR 0004 export taxonomy, and Trace Schema v1 hidden-info/stochastic markers mirror the `flood_watch` posture.
- Deterministic seeded setup: epoch-wise shuffle with the typed constraint that each epoch's Reckoning is never that epoch's first card, resolved deterministically from the seed.
- Viewer-safe action trees: the choosing faction receives its constrained menu (event / full op / limited op / pass, per the eligibility table) and, inside an operation, the progressive compound construction; the waiting faction receives an empty tree with safe waiting metadata; a Reckoning resolves with no player choice.
- Effect-log-driven animation: card reveal, choice, event resolution, edict activation/expiry, op sub-steps, Reckoning breakdowns, victory; readable copy that keeps the event story legible.
- Per-faction Level 0 random and Level 1 scripted policy bots (`EventCharterLevel1Bot`, `EventFreeholdersLevel1Bot`) with documented decision tables, total-order tiebreaks, viewer-safe explanations, and bot-vs-bot simulation metrics including victory-type frequencies.
- Full official-game evidence: unit/rule/property/replay/serialization/visibility/bot tests, golden traces, simulations, fixture validation, rule coverage, benchmarks, the thirteen per-game docs, WASM registration, browser board, E2E smoke, a11y checks, player-rules and outcome-explanation surfaces, CI/tool registration, and the web-shell catalog closeout.
- Documentation updates per the section below, including the `specs/README.md` index lifecycle.

### Out of scope

- Per-seat private holdings, hidden hands, secret roles, hidden victory conditions, or any hidden information beyond undrawn deck order. This keeps the gate's proof budget on event/initiative/scoring/victory shapes and keeps the shuffle-row review a non-use of the private-hand shape.
- Reaction windows, interrupts, pending responses, cancellation, or replacement. The eligibility flow is strictly sequential. If design drift introduces a response window, stop and re-run the atlas reaction-window review first.
- Three or more factions or seats, team play, solo campaign structure. Two factions prove every Gate 14 shape.
- Unit-vs-unit contest/clash resolution (Gate 13's proof). Units coexist at sites; majority is counted, not fought. Events may remove components as typed effects.
- Multi-action turn budgets: no faction ever spends a sequence of separately validated budgeted commands in one turn. This is a deliberate scope decision feeding the work-item-1 non-use record.
- Dice, mid-game shuffles, or any randomness beyond the single setup shuffle.
- Generic event/card/deck/initiative/eligibility/scoring/victory/modifier helpers in `engine-core` or `game-stdlib`. Every new shape is a first use recorded `local-only`; every repeated shape is at most a compared second use or a defer/reject decision.
- A DSL, expression language, or data-driven card effects of any kind. Each card's behavior is an exhaustive typed Rust match.
- New replay/trace/export infrastructure beyond what `flood_watch` proved. "Long-game replay/debug tools" means replay markers, metrics, and ergonomics within existing contracts (Assumption A11), not new tooling architecture.
- Hosted multiplayer, accounts, matchmaking, persistence, chat, ranked play, timers, real-time pressure, or undo.
- MCTS/ISMCTS/Monte Carlo/ML/RL bot work; any Level 2 claim without the full evidence-pack workflow.
- Gate P admission, private licensed content, or any private red-team preparation. This gate must complete with public Rulepath standing alone.
- Ticket decomposition. The Work breakdown lists bounded candidate AGENT-TASKs only.

### Not allowed

Carried from ROADMAP §14 (Gates 12–14 shared prohibition) and tightened for this gate:

- Private licensed content, DSL by stealth, or architecture claims beyond proven games.
- **Behavior in card data — the gate's defining hazard.** Card files declare typed identity and parameters only: card ID, label, epoch pool, first-eligible faction ID, ops value, edict flag, UI metadata. What an event *does*, what an edict *modifies*, when it expires, what a Reckoning resolves, and every legality consequence are typed Rust matched on a closed card-ID enum. No `when`/`if`/`condition`/`trigger`/`selector`/`effect`/`script` fields; unknown and behavior-looking fields are rejected by tests. An event deck in data is precisely where "static files start acting procedural" (FOUNDATIONS §12) begins — treat any drift as a stop condition.
- `engine-core` nouns such as `event`, `card`, `deck`, `epoch`, `initiative`, `eligibility`, `edict`, `reckoning`, `scenario`, `faction`, `site`, `depot`, `cache`, or similar beyond the existing generic envelopes. `card`, `deck`, `scenario`, and `faction` are already on the FOUNDATIONS §3 forbidden list and in `scripts/boundary-check.sh`'s `mechanic_pattern`; work item 13 evaluates adding `initiative` and `eligibility` (word-boundary false-positive check first; `event` is excluded as too generic to pattern-match).
- Generic `game-stdlib` helpers such as `EventDeck`, `InitiativeTrack`, `EligibilityState`, `ScoringRound`, `ModifierStack`, `VictoryCondition`, or `ScriptedBot`. No promotion is authorized by this spec; the only promotions the ledger may consider are the two work-item-1 hard gates, and the expected decisions there are defer/reject and non-use.
- TypeScript legality, eligibility computation, menu constraint logic, op cost computation, edict applicability, scoring, victory evaluation, outcome explanation authority, replay authority, or bot policy.
- Leaking undrawn deck order through views, action trees, previews, diagnostics, effect logs, bot explanations, candidate rankings, DOM, storage, test IDs, dev panels, or replay exports.
- Bots bypassing the legal action API, reading undrawn deck order, or sharing one undifferentiated both-factions policy.
- Trade-dress or terminology proximity to the commercial games whose mechanics were researched: no COIN-series vocabulary (`Propaganda`, `Coup`, eligibility-cylinder presentation), no Twilight Struggle vocabulary (`DEFCON`, scoring-card names), no El Grande presentation. Mechanics are unprotected; names, prose, art direction, and presentation must be original (see the IP-risk register under Implementation reference).
- Accidental trace/hash/schema migration. Any intentional migration needs explicit notes and accepted review.

## Deliverables

| Area | Required artifacts |
|---|---|
| Atlas / ledger (pre-implementation) | `games/event_frontier/docs/PRIMITIVE-PRESSURE-LEDGER.md` recording all work-item-1 items: the two hard-gate decisions (public resource accounting third use; multi-action-budget candidate non-use), the five second-use comparisons, the three non-use reviews, the `board_space` audit, and the four new first-use records; `docs/MECHANIC-ATLAS.md` §10/§10B updates for every touched row; confirmation that §10A stays `_None_`. |
| Workspace and crate | Root `Cargo.toml` registration; `games/event_frontier/Cargo.toml`; source modules mirroring the twelve-file `games/flood_watch` / `games/frontier_control` shape (`src/actions.rs`, `bots.rs`, `effects.rs`, `ids.rs`, `lib.rs`, `replay_support.rs`, `rules.rs`, `setup.rs`, `state.rs`, `ui.rs`, `variants.rs`, `visibility.rs`) **plus a game-specific `cards.rs`** (neither template crate is card-driven, so `cards.rs` has no precedent file) holding the closed card-ID enum and the exhaustive typed event/edict implementations. `flood_watch` is the closest template for deck/automation/hidden-order handling; `frontier_control` for graph data and faction asymmetry; `token_bazaar` for typed constants. |
| Static data | `games/event_frontier/data/manifest.toml`, `variants.toml` (three scenarios), card list data (IDs, labels, epoch pools, first-eligible faction, ops values, edict flags only), and per-scenario fixtures `data/fixtures/event_frontier_<scenario>.fixture.json`. No behavior fields; unknown/behavior-looking fields rejected. |
| Benchmarks | `games/event_frontier/benches/event_frontier.rs`, `benches/thresholds.json`. Identities under Implementation reference, including full-deck playout (the ROADMAP §15-budget row `event_frontier: 100+ turns/sec` in `docs/TESTING-REPLAY-BENCHMARKING.md`), action-tree generation at peak op branching, Reckoning pipeline, and bot latency. Smoke floors first (`baseline_pending_non_blocking`), calibration follow-up per ADRs 0002/0003/0005. |
| Native tests | `games/event_frontier/tests/rules.rs`, `property.rs`, `replay.rs`, `serialization.rs`, `visibility.rs`, `bots.rs`. Coverage under Acceptance evidence. |
| Golden traces | Under `games/event_frontier/tests/golden_traces/`: the eighteen traces listed under Acceptance evidence, each carrying Trace Schema v1 §5 markers for the stochastic surface (setup shuffle) and the hidden-info surface (undrawn deck order), with per-seat surfaces marked not applicable (all seats see the same projection). |
| Per-game docs | Thirteen docs instantiated from `templates/*`: `AI.md`, `BENCHMARKS.md`, `BOT-STRATEGY-EVIDENCE-PACK.md`, `COMPETENT-PLAYER.md`, `GAME-IMPLEMENTATION-ADMISSION.md`, `HOW-TO-PLAY.md`, `MECHANICS.md`, `PRIMITIVE-PRESSURE-LEDGER.md`, `PUBLIC-RELEASE-CHECKLIST.md`, `RULE-COVERAGE.md`, `RULES.md`, `SOURCES.md`, `UI.md`. `HOW-TO-PLAY.md` fills the hidden-information section for real this time (undrawn deck order), and `event_frontier` joins `HIDDEN_INFO_GAMES` in `scripts/check-player-rules.mjs`. |
| Tools | Register `event_frontier` in `tools/simulate`, `tools/replay-check`, `tools/fixture-check`, `tools/rule-coverage`, and `tools/bench-report` (where game IDs are enumerated). Simulation output reports per-faction win counts, **victory-type frequencies** (Charter instant / Freeholder instant / final fallback), average card count, average Reckoning scores, and pass rates — the "useful metrics" and "demo-coherent" evidence base. Confirm at implementation time that `event_frontier` needs no registration in `seed-reducer`/`trace-viewer` (both carry a hardcoded `race_to_n`/`directional_flip` allowlist that rejects every other game id, so the capstone is correctly excluded). |
| WASM/API | Register `event_frontier` in `crates/wasm-api/src/lib.rs` catalog, setup, action, bot, effect, view, replay/export/import paths. `get_view` projections are output-equivalent across seats and observers and never contain undrawn deck order; replay export follows the ADR 0004 taxonomy as proven by `flood_watch`. |
| Browser | `apps/web/src/components/EventFrontierBoard.tsx`: SVG graph map, current/next card display, eligibility indicators, constrained-choice menu, progressive op construction (site picking bounded by ops value, Rust-supplied at every step), edict banner (active modifiers with expiry copy from Rust view data), Reckoning breakdown panel, asymmetric-victory progress indicators (both factions' distances from Rust view data), outcome-explanation templates and rule-ID mirrors, player rules via `scripts/copy-player-rules.mjs`, replay controls with Reckoning/epoch markers, reduced motion, responsive accessible layout. |
| Browser smoke | `apps/web/e2e/event-frontier.smoke.mjs` plus a11y updates: card reveal and choice menu for both factions, an event resolution, a full op with multi-site selection, a pass, an edict activating and expiring, a Reckoning breakdown, each victory type reachable in scripted fixtures (Charter instant, Freeholder instant, final fallback), bot turns for both factions, bot-vs-bot replay, replay step/export/import with no deck-order leak, reduced motion. |
| CI | `.github/workflows/gate-1-game-smoke.yml` native smoke, replay, fixture, rule coverage, web build, E2E registration; `.github/workflows/gate-2-benchmarks.yml` smoke and threshold registration. |
| Repository docs | `specs/README.md` Gate 14 row lifecycle; `docs/MECHANIC-ATLAS.md` updates; `progress.md` and root `README.md` after implementation; web-shell catalog closeout surfaces; no ROADMAP progress edit. |

## Work breakdown

Bounded candidate AGENT-TASKs, in dependency order. Do not decompose these into ticket files as part of this spec.

| # | Candidate task | Depends on | Notes / forbidden drift |
|---:|---|---|---|
| 1 | Pre-implementation primitive-pressure decisions | — | Write `games/event_frontier/docs/PRIMITIVE-PRESSURE-LEDGER.md` covering the full slate under Deliverables: resource-accounting third-use **hard gate decision**, multi-action-budget candidate **non-use record**, five second-use comparisons, three non-use reviews, `board_space` audit, four new first-use rows. Update atlas §10/§10B; confirm §10A stays `_None_`. **Blocks all implementation tasks.** If the resource-accounting decision is promote/ADR, or the budget review finds the design actually repeats budgeted turns, stop: resolve before code. |
| 2 | Crate skeleton, workspace registration, typed data parsing | 1 | Card list, scenario, and map data parse into closed typed enums/structs; reject unknown and behavior-looking fields. Nothing in data says what a card does. |
| 3 | State model and deterministic seeded setup | 2 | Site graph, per-site agents/settlers/depot/cache, resource pools, deck/discard/current/next card, eligibility markers, active-edict list, scores, epoch/Reckoning markers, terminal outcome, freshness token. Epoch-wise seeded shuffle with the Reckoning-placement constraint, deterministic from the seed; replay reproduces it exactly. |
| 4 | Eligibility/initiative card flow | 3 | Card reveal, first/second-eligible determination from card data + eligibility state, constrained choice menus per the typed eligibility table, pass income, eligibility transitions, no-eligible-faction discard, Reckoning cards always resolve. The waiting faction's tree is empty with safe waiting metadata; the flow can never stall. |
| 5 | Operations: compound action trees, validation, application | 4 | Per-faction op types; site-set selection bounded by ops value (limited op = 1 site); per-site sub-choices; resource costs validated against pools and active edicts; one compound command per operation — **no budgeted command sequences**. Largest branching factor yet; tree generation stays within latency budgets. |
| 6 | Event effects and edict modifier system | 5 | Exhaustive typed match on the card-ID enum; each event a typed Rust effect; edicts append typed variants to the active-modifier list consulted at the specific validation/application points they modify, applied in stable (kind, activation-index) order; expiry at Reckoning is a list clear with expiry effects. Never mutate base rules to apply an override. |
| 7 | Reckoning pipeline, asymmetric victory, terminal | 6 | Ordered deterministic pipeline inside the Reckoning card's resolution: victory check (Charter instant / Freeholder instant, stable both-met rule) → site majority scoring → income → reset (expire edicts, restore eligibility). Final-Reckoning fallback by cumulative score with the stable Freeholder tiebreak. `ReckoningResolved` carries the full breakdown; `Terminal` carries winner, victory type, and decisive cause. |
| 8 | Visibility, hidden deck order, and replay surfaces | 7 | Output-equivalent seat/observer projections excluding undrawn deck order; current/next card public; effect filtering; stable summaries; hashes; replay export/import under the ADR 0004 taxonomy mirroring `flood_watch`; Trace Schema v1 markers. |
| 9 | Per-faction Level 0 and Level 1 scripted bots | 7,8 | Level 0 random per faction. Level 1 per faction: documented decision table for event/op/pass keyed on public state (next card, eligibility, resources, victory distance), then ordered site-priority lists with an explicit total-order final tiebreak (stable site index). Legal tree + public view + bot seed only; viewer-safe explanations; never reads deck order. |
| 10 | Native tests and golden traces | 8,9 | Full suite per Acceptance evidence. Follow the failing-test protocol; never weaken tests to get green. |
| 11 | Benchmarks and thresholds | 10 | Identity list under Implementation reference; the 100+ turns/sec stage budget; smoke floors with named ADR 0005 calibration follow-up. |
| 12 | Per-game documentation | 10,11 | All thirteen docs; `RULES.md` stable rule IDs including eligibility-table, edict, Reckoning-order, victory, and tiebreak IDs; `HOW-TO-PLAY.md` hidden-info section filled; `SOURCES.md` records the researched mechanics posture (see Implementation reference). |
| 13 | WASM, tools, CI registration, boundary-check evaluation | 10 | Register across wasm-api, the five game-enumerating tools, and both CI lanes. Add `event_frontier` to `HIDDEN_INFO_GAMES` in `scripts/check-player-rules.mjs`. Evaluate extending `scripts/boundary-check.sh` `mechanic_pattern` with `initiative` and `eligibility` (word-boundary false-positive check; pattern already covers `card`, `deck`, `scenario`, `faction`); keep `event` out as too generic. The extension must stay green on the existing tree. |
| 14 | React board, long-game replay ergonomics, browser smoke | 13 | `EventFrontierBoard.tsx` per Deliverables; outcome-explanation templates in `apps/web/src/components/outcomeExplanationTemplates.ts` and the rationale mirror in `apps/web/src/wasm/client.ts`; replay controls gain Reckoning/epoch jump markers driven from the effect log (presentation over existing replay contracts, not new tooling); E2E smoke per Deliverables; TS computes no eligibility, cost, edict, scoring, or victory logic. |
| 15 | Repository documentation and final admission evidence | 12,13,14 | Spec index flip, atlas confirmation, progress/root README updates after evidence passes, catalog closeout surfaces, command transcript, unresolved issues. Do not edit ROADMAP as a progress diary. |

## Exit criteria

Mapped row-for-row to ROADMAP §14 (Gate 14).

| ROADMAP §14 line | Gate 14 exit criterion |
|---|---|
| public Rulepath stands without private experiments | The full public verification suite passes with `event_frontier` registered (workspace tests, per-game tools, boundary check, doc-link/catalog/player-rules/outcome checks, web build, smoke layers); no private content, names, or dependencies exist anywhere in the change; the public site presents the complete ladder of official games with this capstone playable end to end. |
| action trees remain usable | The largest-branching op trees generate within the <100ms latency budget (`docs/TESTING-REPLAY-BENCHMARKING.md` latency targets) at native benchmark; progressive construction in the browser presents bounded, legible choices at every step (E2E asserts the multi-site op flow); property tests prove the choice menus never stall (a legal choice or terminal always exists). |
| scripted bots are demo-coherent | Per-faction Level 1 scripted bots have documented decision tables and priority lists with total-order tiebreaks; bot-vs-bot replay completes full games with explanations that narrate a coherent plan; simulation across 1,000+ games reports per-faction win rates inside the balance band (Assumption A5) and a victory-type mix in which every victory type occurs (Assumption A5); no bot ever submits an out-of-tree action or reads undrawn deck order. |
| every repeated mechanic has ledger resolution | Work item 1's ledger slate is recorded before implementation and re-confirmed at closeout: both hard gates decided (resource accounting; budget non-use), all second-use comparisons and non-use reviews recorded, the `board_space` audit recorded, the four new first-use rows present, and `docs/MECHANIC-ATLAS.md` §10A still `_None_`. |
| Not allowed: private licensed content | **Honored.** All names, labels, prose, and visuals are original and IP-reviewed per `docs/IP-POLICY.md`; `SOURCES.md` records the consulted-mechanics-only posture and the trade-dress avoidance register. |
| Not allowed: DSL by stealth | **Honored.** Card data is typed identity and parameters; every event, edict, Reckoning, and victory behavior is typed Rust; unknown and behavior-looking fields are rejected by tests; the edict system is a typed modifier list, not an interpreter. |
| Not allowed: architecture claims beyond proven games | **Honored.** No event/initiative/scoring/victory/modifier abstraction is promoted anywhere; first uses are recorded `local-only`; the only promotion question this gate may decide is the resource-accounting hard gate, and only through the ledger. |

## Acceptance evidence

### Native rules, replay, visibility, and bot evidence

- `cargo test -p event_frontier` passes.
- Rule tests cover: deterministic epoch shuffle and Reckoning-placement constraint for all three scenarios; first/second-eligible determination including ineligibility, skips, and the no-eligible-faction discard; every cell of the eligibility constraint table (first event → second's menu; first op → second's menu; first pass → second's menu; double pass); pass income; op site-count bounds (full vs limited), per-op-type legality, resource costs, and cap enforcement; every event card's typed effect; every edict's activation, modified-rule consequence, and expiry; the full Reckoning order (victory check before scoring before income before reset); both instant victory conditions including the both-met rule; the final-fallback comparison and Freeholder tiebreak; and terminal behavior after victory.
- Diagnostics tests cover: stale freshness tokens, wrong-seat/ineligible-faction submissions, menu-constraint violations, over-budget site selections, unaffordable ops, edict-blocked actions, staking/cache/depot/cap violations, and post-terminal submissions — all viewer-safe.
- Property tests over many bot-seeded games assert: the card flow never stalls (every non-terminal state offers a legal choice or auto-resolves); eligibility state is always consistent with the acted/passed history; resources never go negative and respect caps; active edicts always expire at the next Reckoning; Reckonings fire exactly once per epoch; victory is checked before reset; component counts change only through documented effects; replay determinism; no panics.
- Replay tests prove seed + scenario + command stream reproduce the shuffle, state hashes, effect hashes, action-tree hashes, view hashes, Reckoning breakdowns, and terminal outcome.
- Serialization tests prove stable summaries and unknown-field rejection for manifest, scenarios, card data, fixtures, and replay export — including rejection of behavior-looking fields in card data (`when`, `condition`, `trigger`, `effect`, `script`).
- Visibility tests prove: seat and observer projections are output-equivalent; **no projection, action tree, preview, diagnostic, effect, bot explanation, or replay export contains undrawn deck order** (serialize-and-search per `docs/TESTING-REPLAY-BENCHMARKING.md` §8); the current/next-card surfaces show exactly the public cards.
- Bot tests prove per-faction legality over many seeds, determinism under declared inputs, decision-table conformance on constructed states, distinct per-faction policies, no deck-order access, and viewer-safe faction-appropriate explanations.
- Balance and demo-coherence evidence: Level 1 vs Level 1 simulation reports per-faction win rates inside the Assumption A5 band and a victory-type distribution in which all three victory types occur; misses trigger a constants retune recorded in `COMPETENT-PLAYER.md`/`BENCHMARKS.md` before public polish.

### Golden traces

Committed under `games/event_frontier/tests/golden_traces/`, each carrying Trace Schema v1 §5 stochastic (setup shuffle) and hidden-info (undrawn deck order) markers, checked by `replay-check`:

- `standard-charter-instant-win.trace.json`
- `standard-freeholder-cache-win.trace.json`
- `final-reckoning-fallback.trace.json` (neither instant condition; cumulative score + Freeholder tiebreak path)
- `event-choice-resolves-card.trace.json`
- `op-full-multi-site.trace.json` (op type → multiple sites → sub-choices)
- `limited-op-after-full-op.trace.json`
- `pass-keeps-eligibility.trace.json`
- `double-pass-discards-card.trace.json`
- `no-eligible-faction-discard.trace.json`
- `edict-activation-and-expiry.trace.json`
- `edict-blocks-action-diagnostic.trace.json`
- `reckoning-scoring-breakdown.trace.json`
- `reckoning-never-first-in-epoch.trace.json` (setup-constraint evidence)
- `hard-winter-setup.trace.json` / `land-rush-setup.trace.json` (scenario evidence)
- `ineligible-faction-diagnostic.trace.json`
- `bot-vs-bot-full-game.trace.json`
- `replay-export-import-no-deck-leak.trace.json`

### Tools, benchmarks, browser, docs, and CI

- `cargo run -p simulate -- --game event_frontier --games 1000` finishes with no illegal bot action or invariant failure and reports per-faction win counts, victory-type frequencies, average card count, and pass rates.
- `cargo run -p replay-check -- --game event_frontier --all` passes.
- `cargo run -p fixture-check -- --game event_frontier` passes for all three scenarios.
- `cargo run -p rule-coverage -- --game event_frontier` passes with no silent gaps.
- `cargo bench -p event_frontier` runs the benchmark identity list; `bench-report` enforces smoke floors where calibrated.
- `npm --prefix apps/web run smoke:wasm`, `smoke:ui`, and `smoke:effects` pass after registration.
- `node apps/web/e2e/event-frontier.smoke.mjs` covers the flows listed under Deliverables.
- `bash scripts/boundary-check.sh` passes, including any work-item-13 pattern extension, on the whole tree.
- `node scripts/check-doc-links.mjs`, `node scripts/check-catalog-docs.mjs`, `node scripts/check-player-rules.mjs` (with `event_frontier` in `HIDDEN_INFO_GAMES` and the hidden-info section filled), and `node scripts/check-outcome-explanations.mjs` all pass.
- All thirteen per-game docs are present, link-checkable, original, and consistent with implemented behavior.

## FOUNDATIONS & boundary alignment

| Principle | Stance | Rationale |
|---|---|---|
| §2 Behavior authority | Aligned | Rust owns the shuffle, card sequencing, eligibility/menu constraints, op legality and costs, event/edict semantics, Reckoning pipeline, asymmetric victory, terminal detection, effects, hidden-order-safe projection, replay, and both factions' bot decisions. React presents card faces, menus, maps, breakdowns, and progress indicators from Rust view data only. |
| §3 `engine-core` contract kernel | Aligned | The event deck, initiative system, edicts, and victory conditions flow through the existing generic action-tree/command/effect/visibility/replay envelopes. `card`, `deck`, `scenario`, and `faction` are forbidden kernel nouns already mechanically checked; work item 13 evaluates `initiative`/`eligibility` pattern additions. |
| §4 `game-stdlib` earned | Aligned | Nothing is promoted by default. The two armed third-use pressures are resolved by ledger before implementation (resource accounting: genuine hard gate, expected defer/reject; budgets: expected non-use); all other shapes are first uses or compared second uses kept local. |
| §5 Static data is typed content | **Aligned — highest-risk surface of the roadmap.** | An event deck is the canonical temptation to put behavior in data. Card files carry identity and typed parameters only; behavior is an exhaustive Rust match on a closed enum; edicts are typed modifier variants; serialization tests reject behavior-looking fields. Any drift here is a FOUNDATIONS §12 stop condition ("static files start acting procedural"). |
| §6 Official games are evidence-heavy | Aligned | Full contract coverage per Acceptance evidence: tests, traces, simulations, tools, benchmarks, thirteen docs, rule coverage with no silent gaps. |
| §7 Public UI is central product work | Aligned | Legal-only constrained menus, progressive op construction from Rust trees, effect-driven card/Reckoning/victory animation with reduced motion, readable event-story log copy, accessible responsive layout. |
| §8 Public bots | Aligned | Per-faction scripted policy bots are Level 1 deterministic priority policies — the digital correction of the COIN-paper-bot pitfall: they select only from the engine's legal tree, with explicit total-order tiebreaks and viewer-safe explanations. No MCTS/ISMCTS/Monte Carlo/ML/RL. |
| §9 Local-first v1/v2 | Aligned | Hotseat, human-vs-bot either side, bot-vs-bot replay, local import/export. No hosted services. |
| §10 IP conservatism | Aligned | Eligibility systems, event-vs-ops choices, scoring rounds, and area majority are unprotected mechanics; the IP-risk register names the researched commercial designs (COIN series, Twilight Struggle, El Grande) and the specific vocabulary/trade-dress hazards to avoid; all names, prose, and visuals are original. |
| §11 Universal invariants | Aligned | Deterministic shuffle/replay/hashes, viewer-safe projections with the single hidden surface tested by serialize-and-search, legal-only UI, semantic-effect animation, local-first scope, full evidence coverage, bounded agent output. |
| §12 Stop conditions | Clear | Stop if: card data turns procedural; a kernel noun appears; TypeScript computes eligibility/cost/scoring/victory; deck order leaks anywhere; a bot bypasses the tree or reads deck order; either work-item-1 hard gate is skipped, reordered after implementation, or rubber-stamped; a reviewed atlas expectation is contradicted; or a reaction window or budgeted turn drifts in. |
| §13 ADR triggers | **No ADR expected.** | Kernel vocabulary, data policy, visibility contracts, replay/hash semantics, and renderer defaults are unchanged; the hidden-order export posture reuses ADR 0004 as `flood_watch` proved it. If the resource-accounting ledger decision escalates, or any contract genuinely must change, stop and write the ADR first. |
| ADR 0004 hidden-info replay/export | Engaged, no extension | The only hidden surface is undrawn deck order, hidden from all viewers symmetrically — the exact shape `flood_watch` already carries under the existing taxonomy. Mirror its registration, markers, and export redaction; no new taxonomy category. |
| Benchmark ADRs 0002/0003/0005 | Aligned | Smoke thresholds first (`baseline_pending_non_blocking`), PR lane non-gating, variance-aware calibration follow-up named once representative CI runs exist. |
| ADR 0006 Blackjack placement | Untouched | Gate 14 has no draw/stand threshold shape and no casino comparison; `blackjack_lite` remains a deferred comparison case only. |

## Forbidden changes

Do not, in this gate:

- Put behavior in card, scenario, or map data — no conditions, selectors, triggers, expressions, effect scripts, or untyped nested objects anywhere in static files. No YAML, no DSL.
- Add `event`, `card`, `deck`, `epoch`, `initiative`, `eligibility`, `edict`, `reckoning`, `scenario`, `faction`, `site`, `depot`, `cache`, or similar mechanic vocabulary to `engine-core`.
- Add generic `game-stdlib` primitives for event decks, initiative/eligibility, scoring rounds, victory conditions, modifier stacks, or scripted-bot scaffolding. Only the work-item-1 ledger may authorize any promotion, and only for the resource-accounting row.
- Skip, reorder after implementation, or rubber-stamp the work-item-1 ledger slate — especially the two hard gates and the mandatory `board_space` audit.
- Introduce budgeted multi-command turns, reaction windows, pending responses, per-seat private holdings, mid-game shuffles, or dice — each re-arms an atlas row or re-scopes the visibility surface and requires a spec correction plus ledger re-review first.
- Let TypeScript compute eligibility, menu constraints, op costs, edict applicability, scoring, victory, terminal causes, replay authority, or bot policy.
- Leak undrawn deck order through any payload, DOM surface, storage, log, explanation, test ID, dev panel, or export.
- Use MCTS, ISMCTS, Monte Carlo, ML, RL, LLMs, or search beyond the documented Level 1 deterministic priorities in any bot; never let a bot read undrawn deck order.
- Use COIN/Twilight Struggle/El Grande vocabulary, card names, rules prose, art direction, or trade dress in any public-facing surface.
- Add hosted services, accounts, persistence, timers, or real-time pressure.
- Change trace schema, replay/hash semantics, data versions, action-path stability, effect ordering, or golden traces accidentally.
- Delete, rename away, weaken, or rewrite tests merely to get green output. Follow the failing-test protocol.
- Start any Gate P / private red-team work. Gate 14 must close with public Rulepath standing alone.

## Documentation updates required

- `specs/README.md`: Gate 14 row points at this spec with status `Planned` now, then `In progress`/`Done` as the lifecycle advances; flip to `Done` only after exit criteria pass with evidence.
- Do **not** edit `docs/ROADMAP.md` to record progress.
- `docs/MECHANIC-ATLAS.md`:
  - §10B `multi-action turn budgets`: record the third-use candidate resolution (expected: non-use — one choice per card, compound ops; the row's hard gate is satisfied by the recorded review; name the next candidate trigger).
  - §10/§10B `public resource accounting / shared ledgers`: record the third-use **hard gate decision** (expected: defer/reject with rationale, risks, and next trigger; any promote/ADR outcome stops the gate until resolved).
  - §10B `event-deck environment automation`: record the second-use comparison vs `flood_watch`.
  - §10B `role-modified action effects`: record the edict comparison (expected: related but distinct — temporal event-imposed modifiers, not seat-carried roles; the third-close-shape hard gate is evaluated honestly).
  - §10B `graph-map topology`, `site control / deterministic contest resolution`, `faction-asymmetric action sets and scoring`: record the second-use comparisons vs `frontier_control` (site control expected related-but-distinct: majority without contest).
  - §10B `reaction window/pending response`, `shared-outcome cooperative terminal`, `deterministic shuffle / private hand / staged reveal`: record the non-use reviews (shuffle row: shuffle without per-seat private holdings, mirroring `flood_watch`).
  - §10 `fixed 2D occupancy / board-space identity`: record the audit outcome (expected: not applicable — graph sites).
  - New rows recorded `local-only` first use: event-card initiative/eligibility sequencing; periodic scoring/reset pipeline; asymmetric instant victory conditions; timed rule-exception modifiers (edicts) — each with a next-trigger naming Gate P pressure as non-public comparison only.
  - Confirm §10A stays `_None_`.
- Author all thirteen `games/event_frontier/docs/*` from templates; `SOURCES.md` records the consulted-mechanics posture from this spec's research pass (COIN eligibility, CDG event-vs-ops, El Grande/COIN scoring rounds, MtG-style layered modifiers as engineering prior art) with the originality/naming review.
- `scripts/check-player-rules.mjs`: add `event_frontier` to `HIDDEN_INFO_GAMES`; `games/event_frontier/docs/HOW-TO-PLAY.md` fills the hidden-information section (undrawn deck order) and generates `apps/web/public/rules/event_frontier.md` via `scripts/copy-player-rules.mjs`.
- Outcome-explanation surfaces: victory-type and tiebreak templates in `apps/web/src/components/outcomeExplanationTemplates.ts` plus the rationale/rule-ID mirror in `apps/web/src/wasm/client.ts` (the catalog-driven checker also reads the game's `docs/UI.md` outcome section and `docs/RULES.md` rule IDs), registered so `scripts/check-outcome-explanations.mjs` passes.
- Web-shell catalog closeout inside this gate (per `specs/README.md` §10): `apps/web/README.md` intro catalog list, Shell Surface renderer list (manual surface), Smoke Layers `smoke:e2e` bullet; the `smoke:e2e` script in `apps/web/package.json`; root `README.md` official-games list; `node scripts/check-catalog-docs.mjs` must pass.
- Update `progress.md` and root `README.md` after implementation, not before evidence passes.
- Ensure `node scripts/check-doc-links.mjs` passes after doc changes.

## Sequencing

- **Predecessor:** Gate 13 (`frontier_control`) is `Done` in the spec index with evidence; the atlas §10A promotion-debt register records `_None_`, so no maintenance interlock precedes this gate.
- **Admission rule:** Before implementation starts, confirm §10A still records `_None_`, and complete work item 1's full ledger slate — both hard gates decided, all comparisons and reviews recorded, the `board_space` audit done — before any card, eligibility, or scoring code is written.
- **This gate:** Gate 14 is the public ladder's capstone: event decks with typed rule exceptions, eligibility/initiative, periodic scoring/reset, asymmetric victory, large action trees, scripted policy bots, scenario setup, and long-game replay — with the data/Rust boundary under maximum pressure and held.
- **Successor:** Gate P (private monster-game red-team) admits only after Gate 14 is complete and public Rulepath is coherent without private work (ROADMAP §15). Gate P is optional, isolated, and non-public; nothing in this gate may prepare for it.

## Assumptions

- A1: Two seats and two factions are the sufficient proof shape; more factions would enrich the eligibility dance but add balance/UI surface without strengthening any §14 exit line.
- A2: The display name `Event Frontier`, faction names (`Charter`, `Freeholders`), site labels (`Charterhouse`, `Landing`, `Crossing`, `Granite Pass`, `High Meadow`, `Old Mill`), card names, and the terms `edict`/`Reckoning` are original placeholders; maintainers may rename after IP review before implementation. COIN/TS/El Grande vocabulary is deliberately avoided; `Warden`(`s`) and Gate 13's faction names are avoided as already used.
- A3: The proposed constants — 6 sites/8 trails, 21 cards (18 events incl. 4 edicts + 3 Reckonings) in 3 epochs of 7 with the Reckoning never first in its epoch, ops values 1–3, start resources 3 (cap 9), pass income +1, Reckoning income +2, unit caps 3/site, depot cap 1/site, cache cap 2/site, Charter instant victory at majority of ≥4 sites, Freeholder instant victory at ≥8 total caches, both-met → Freeholders, final fallback by cumulative site-score with Freeholder tiebreak — are tunable while preserving the deck-driven terminal and proof scope.
- A4: The only hidden information is undrawn deck order, hidden from all viewers symmetrically; current and next cards are public; caches and all components are public. This mirrors `flood_watch`'s ADR 0004 posture exactly and keeps the shuffle-row review a non-use of the private-hand shape. Adding any per-seat hidden surface is a spec change, not drift.
- A5: The balance target is each faction winning roughly 35–65% of Level 1 vs Level 1 games on the standard scenario, **and** every victory type (Charter instant, Freeholder instant, final fallback) occurring in a 1,000-game simulation; misses trigger a constants retune recorded in `COMPETENT-PLAYER.md`/`BENCHMARKS.md` before public polish.
- A6: The turn flow is one choice per faction per event card; multi-action turn budgets are **not** repeated, and the atlas third-use candidate row resolves as a recorded non-use. If the design drifts into budgeted turns, the hard gate fires before any code.
- A7: An operation is one compound command built through the existing progressive action-tree contracts (op type → sites → sub-choices); the large-action-tree proof needs no new tree infrastructure.
- A8: The work-item-1 expected outcomes hold (resource accounting: defer/reject; budgets: non-use; board_space: not applicable; the comparisons and non-use reviews as listed). Work item 1 verifies each against the atlas before implementation and stops the gate if any fails.
- A9: Level 0 random plus Level 1 scripted policy bots per faction satisfy the ROADMAP "scripted bots are demo-coherent" exit line; a Level 2 claim requires the full `COMPETENT-PLAYER.md` + `BOT-STRATEGY-EVIDENCE-PACK.md` evidence workflow before coding.
- A10: An external research pass **was** run for this spec (unlike Gates 12–13): COIN-series eligibility and scripted-bot structure, CDG event-vs-ops design, periodic-scoring pipelines, and digital rule-exception engineering (layered-modifier ordering) informed the proposed rules; the research notes under Implementation reference seed `SOURCES.md`, which must record what is actually consulted at implementation time.
- A11: "Long-game replay/debug tools" is satisfied by Reckoning/epoch replay markers driven from the effect log, the richer simulation metrics, and export/import at the longest command timeline yet — presentation and metrics over existing replay contracts, not new tooling architecture. If implementation proves existing contracts insufficient, stop and re-scope rather than improvise infrastructure.
- A12: If any assumption is wrong, correct the smallest local game/spec surface; do not generalize the engine.

---

# Implementation reference

## Product intent / what this gate proves

`Event Frontier` is a deliberately compact original card-driven frontier game. A season of events sweeps a six-site frontier territory. The **Charter** — the chartered survey company — plants agents and builds depots to bring the territory under orderly administration. The **Freeholders** — independent settlers — trek the trails, lay caches, and try to outlast the Charter's season. Every turn of play is driven by the event deck: each card names who moves first, offers its event or its operations value, and sooner or later the season's **Reckoning** arrives — scores are tallied, edicts expire, and both sides learn whether the frontier is won.

Architecturally, the gate makes the platform prove, at the highest public complexity:

- **Events are code, not data.** Each card's behavior is an exhaustive typed Rust match; the deck file is an inventory of typed identities and numbers. The platform's defining boundary survives its strongest temptation.
- **Rule exceptions are projections, not mutations.** Edicts append typed variants to an active-modifier list consulted at exactly the validation/application points they modify, in stable (kind, activation-index) order; expiry is a list clear. Base rules are never patched and never reverse-patched.
- **Initiative is state, not ceremony.** Eligibility is a small per-faction state machine driven by card data and player choices — fully deterministic, sequential, and replayable, with no simultaneity and no reaction windows.
- **Victory can be asymmetric without forking the contracts.** Different instant win conditions per faction, checked at deterministic points, flow through the same terminal/outcome-explanation contracts every symmetric game used.

## Proposed original rules: `Event Frontier`

### Components

- Two seats: `seat_0` plays `faction_charter` (the Charter), `seat_1` plays `faction_freeholders` (the Freeholders).
- Six sites: `site_charterhouse` (Charter home), `site_landing` (Freeholder home), `site_crossing`, `site_granite_pass`, `site_high_meadow`, `site_old_mill`; eight trails (edges) forming a connected graph (exact edge list is typed scenario data).
- Charter **agents** (cap 3/site) and **depots** (cap 1/site); Freeholder **settlers** (cap 3/site) and **caches** (cap 2/site).
- Resource pools: Charter **funds**, Freeholder **provisions** (start 3, cap 9).
- The event deck: 21 cards — 18 events (4 of them **edicts**) + 3 **Reckonings** — in three epochs of 7; within each epoch the seeded shuffle places the Reckoning anywhere except first.
- Per-faction eligibility markers; current card and next card are public.

### Card anatomy (typed data; behavior in Rust)

Each card's data row: stable card ID, display label, epoch pool, first-eligible faction ID, ops value (1–3), edict flag, UI metadata. The card's effect — including every edict's modified rule and expiry — is typed Rust matched on the closed card-ID enum.

### Sequence per event card

1. The current card is public; the next card is public (no next card before the final card).
2. The **first eligible** faction is the card's printed faction if eligible, otherwise the other faction if eligible. If neither is eligible, the card is discarded unresolved (a real tempo cost of both factions having acted) — except **Reckonings always resolve**.
3. The first eligible chooses **Event** (resolve the card's typed effect), **Operation** (compound action funded by resources, up to the card's ops value in sites), or **Pass** (+1 resource, stay eligible).
4. The second faction's menu is constrained by the first's choice (typed table): first took Event → second may take Operation or Pass; first took Operation → second may take Event, **Limited Operation** (1 site), or Pass; first Passed → second has the full menu (double pass discards the card; both stay eligible).
5. Any faction that took Event or an Operation is **ineligible for the next card only**; passing preserves eligibility. Eligibility is restored for all at each Reckoning.

### Operations (one compound command each)

Cost: 1 resource per selected site. Select an op type, then 1..N sites (N = ops value; limited op N = 1), then a per-site sub-choice where the type requires it.

**Charter ops** — `survey` (place 1 agent at a site adjacent to Charter presence, or at the Charterhouse), `fortify` (build a depot at a site with ≥2 agents and no depot), `writ` (at an agent-occupied site, remove one cache and gain 1 fund).

**Freeholder ops** — `trek` (move one settler from the selected site along a trail), `cache` (lay a cache at a settler-occupied, depot-free site), `rally` (place 1 settler at the Landing or at any cache site).

### Events and edicts

Fourteen ordinary events with typed one-shot effects (component placement/removal/relocation, resource swings, targeted at typed parameters). Four **edicts** — proposed: `Toll Roads` (each selected site costs +1 resource until the Reckoning), `Survey Ban` (no `survey`/`rally` at contested sites), `Requisition` (Charter ops at depot sites cost 0), `Long Season` (the first eligible may select one site beyond the ops value) — each a typed modifier variant active until the next Reckoning, applied in stable (kind, activation-index) order.

### Reckoning (always resolves; ordered pipeline)

1. **Victory check** (before any reset): the Charter wins instantly with majority presence at ≥4 of 6 sites; the Freeholders win instantly with ≥8 total caches; if both hold, the Freeholders win (the frontier slips the Charter's grasp — stable rule ID).
2. **Site scoring**: each site awards 1 point to the faction with strictly greater presence (agents + depot count as presence for the Charter; settlers for the Freeholders; caches do not count for presence). Ties award no one.
3. **Income**: each faction gains +2 resources (respecting caps).
4. **Reset**: all edicts expire (with expiry effects), all factions become eligible.

After the third Reckoning's pipeline, if no instant victory has fired: the higher cumulative score wins; a tie goes to the **Freeholders** (the frontier outlasts the season — stable rule ID, deliberately the inverse of Gate 13's incumbent tiebreak).

### Why these constants resist degeneracy

The Charter needs breadth (majority at 4 of 6 sites) while its placement is adjacency-chained from its home and its depots demand local concentration — breadth and depth compete for the same funded sites. The Freeholders' cache victory (8 caches at cap 2/site) forces presence at ≥4 sites too, but `writ` lets the Charter tax exposed caches, so cache placement wants settler escorts that `trek` keeps mobile. Passing pays +1 and keeps initiative for the next card — the COIN-shaped tempo economy — while acting on a strong event card costs the next card's initiative. Reckoning timing inside each epoch is uncertain (position 2–7), so over-extension right before a Reckoning is punished. Constants are tunable under Assumption A5 with simulation evidence; the win band and victory-type mix, not the exact numbers, are the requirement.

## State, actions, and validation sketch

### State

- `scenario: Scenario`; `seats: [SeatId; 2]`, `factions: [FactionId; 2]`
- Deck state: `undrawn: Vec<CardId>` (hidden order), `current: Option<CardId>`, `next_public: Option<CardId>`, `discard: Vec<CardId>`, `epoch: u8`
- Sequence state: `eligibility: [Eligibility; 2]`, `card_phase: CardPhase` (`FirstChoice { faction }`, `SecondChoice { faction, first_took }`, resolution markers)
- Map state: per-site `agents: u8`, `settlers: u8`, `depot: bool`, `cache_count: u8`; adjacency from validated edge data
- `funds: u8`, `provisions: u8`, `active_edicts: Vec<ActiveEdict>` (typed variant + activation index), `scores: [u16; 2]`, `terminal_outcome: Option<Outcome>` (winner, victory type, totals), `freshness_token`

### Legal action tree

- Choosing faction: the constrained menu as flat choices (`event`, `op/<type>/...` compound paths, `pass`), with op paths expanding progressively: op type → site set (bounded by ops value, affordability, and edicts) → per-site sub-choice. Choice metadata carries costs, ops value, eligibility consequences ("acting forfeits initiative on the next card"), and edict annotations — all Rust-supplied.
- Waiting faction: empty tree with safe waiting metadata. Reckonings and discards resolve without player choice. Terminal: empty trees.

### Validation

Rejects: stale freshness token; non-seated actor; ineligible or out-of-phase faction; menu-constraint violations (e.g. Event after the first faction took Event); site counts beyond the ops bound; unaffordable ops; edict-blocked selections; placement/movement/cap violations (survey without adjacency, fortify under 2 agents or with a depot, writ without a cache, trek without a settler or trail, cache at a depot site or at cap, rally at an invalid site); post-terminal submissions. Diagnostics are viewer-safe and name the typed reason ("the Toll Roads edict raises that site's cost beyond your funds").

## Semantic effect model

| Effect | Visibility | Payload rule |
|---|---|---|
| `CardRevealed { card, next_public }` | Public | Never exposes undrawn order beyond the public next card. |
| `ChoiceTaken { faction, choice }` / `CardDiscarded { card, reason }` | Public | Menu/eligibility story for the log. |
| `EligibilityChanged { faction, eligible, reason }` | Public | — |
| `EventResolved { card, summary }` + typed per-event effects | Public | One semantic effect per component/resource change. |
| `EdictActivated { card, edict }` / `EdictExpired { edict }` | Public | Drives the edict banner. |
| `OpResolved { faction, op, sites }` + per-site effects (`AgentPlaced`, `DepotBuilt`, `CacheRemoved`, `SettlerMoved`, `CacheLaid`, `SettlerRallied`, …) | Public | Movement/placement before consequence effects; stable order. |
| `ResourcesChanged { faction, delta, reason }` | Public | Pass income, op costs, writ gains, Reckoning income. |
| `ReckoningResolved { round, victory_check, site_breakdown, income, expired_edicts }` | Public | Full per-site majority breakdown — the scoring story. |
| `Terminal { winner, victory_type, totals, summary }` | Public | Drives the outcome-explanation surface with the victory-type cause. |
| `PrivateDiagnostic` / `PublicDiagnostic` | Private / public | Viewer-safe reasons only. |

All effects are public; the hidden surface (undrawn order) never appears in any effect payload. React animates card reveals, op steps, edict banners, Reckoning tallies, and the terminal in effect order, settling to the latest view; reduced motion preserves order and copy.

## Visibility and replay model

- **One public projection for all viewers** (seats and observer): map, components, resources, scores, eligibility, current and next cards, discards, active edicts, epoch/Reckoning progress, victory-distance summaries, terminal outcome. `get_view` is output-equivalent across viewers; visibility tests assert it.
- **Hidden surface**: undrawn deck order only — hidden from everyone symmetrically. Serialize-and-search tests prove no payload, preview, diagnostic, explanation, export, or DOM fixture contains it.
- **Replay/export**: seed + scenario + command stream reproduce the epoch shuffle and the full timeline; export follows the ADR 0004 taxonomy exactly as `flood_watch` proved it (public timeline with the hidden-order surface redacted/recorded per taxonomy); traces carry Trace Schema v1 §5 stochastic and hidden-info markers with per-seat surfaces not applicable.

## Bot policy

### Level 0

`EventFrontierRandomBot` selects uniformly from the legal tree using deterministic bot RNG helpers, for either faction.

### Level 1 — scripted policy bots (the ROADMAP "scripted bots" proof)

Each faction's bot is a documented two-layer deterministic policy — the digital descendant of COIN flowchart bots with their known failure corrected: it only ever **filters and ranks the engine's legal action tree**, never generates moves, and every priority list ends in an explicit total-order tiebreak (stable site index, then stable action-path order).

**Layer 1 — choice table** (event / op / pass), keyed on public state only: is the current event favorable (typed per-card favorability classification), is the next public card worth keeping initiative for, resource pressure, and victory distance (own and opponent's).

**Layer 2 — site priorities** per op type. `EventCharterLevel1Bot`: deny an imminent Freeholder cache victory (writ the largest cache cluster) → extend majority toward the 4-site threshold (survey the cheapest contested site) → fortify the most contested held site → save funds. `EventFreeholdersLevel1Bot`: complete the cache threshold (cache at the safest eligible site) → escort exposed caches (trek toward writ-threatened sites) → spread presence to break Charter majorities (rally/trek to the emptiest site) → save provisions.

Explanations are viewer-safe and narrate the plan ("Laid a cache at High Meadow: two more secure the season"). Bot tests assert table conformance on constructed states; simulation reports the demo-coherence metrics under Acceptance evidence.

## WASM/browser wiring

- Catalog entry `game_id: event_frontier`, display name `Event Frontier`, hidden-info marker (deck order), three scenarios, docs links.
- `EventFrontierBoard.tsx`: SVG graph map (sites as nodes, trails as edges, agent/settler/depot/cache markers), card panel (current card face, public next card, deck count without order), eligibility indicators, the constrained choice menu and progressive op constructor driven purely by the Rust tree, edict banner, Reckoning breakdown panel, victory-progress indicators from Rust view data, effect-driven animation, outcome-explanation surface, replay controls with Reckoning/epoch markers, reduced motion, responsive accessible layout.

## Benchmark identities

`setup_standard`, `shuffle_and_deal_epochs`, `legal_tree_first_choice`, `legal_tree_peak_op_branching`, `apply_event`, `apply_op_multi_site`, `edict_modifier_projection`, `reckoning_pipeline`, `bot_l1_choice_charter`, `bot_l1_choice_freeholders`, `full_random_playout` (the 100+ turns/sec stage-budget row), `serialize_view`, `replay_full_game`.

## IP-risk register

Researched prior art is mechanics-only; mechanics are not copyrightable, but vocabulary, card text, art direction, and trade dress are protected. Named hazards to avoid in all public surfaces:

- **GMT COIN series** (Fire in the Lake, Cuba Libre, etc.): the eligibility *mechanic* is safe; the vocabulary cluster `Propaganda`, `Coup`, `1st/2nd Eligible` presentation, eligibility-cylinder trade dress, and faction-board layouts are not. Use `initiative`/`eligibility` as plain descriptive terms with original presentation.
- **Twilight Struggle / CDG lineage**: event-vs-ops choice is safe; `DEFCON`, scoring-card names, space-race tracks, and the map trade dress are not.
- **El Grande**: periodic area scoring is safe; `Castillo`, caballero presentation, and score-round card art are not.
- **Root / area-control trade dress** (carried from Gate 13): avoid woodland-faction theming and `clearing` vocabulary.
- Naming review before implementation per `docs/IP-POLICY.md` §11/§13; `SOURCES.md` records consulted sources, dates, what was and was not used, and the originality rationale.

## External research notes (seed for `SOURCES.md`)

A web research pass informed this spec (2026-06-12): COIN eligibility sequencing and its determinism-friendly properties (printed faction order + small eligibility state machine; The Players' Aid COIN guide; BGG COIN series wiki); COIN solo-bot structure and pitfalls — flowchart ambiguity and underspecified tiebreakers as the chief errata source, and paper bots' reliance on human legality adjudication (InsideGMT All Bridges Burning solitaire series; FitL Trưng bot rules); CDG event-vs-ops design and digital implementation experience — card-combination interactions and scoring-sequence bugs dominating digital TS's defect history, arguing for per-card fixture tests and scenario fixtures (InsideGMT TS digital updates); periodic scoring edge cases — tie demotion semantics, victory-check-before-reset ordering, scoring-card deck-position fairness (El Grande rules; FitL rules; BGG threads); and rule-exception engineering — layered continuous-effect application with timestamp ordering as the deterministic pattern for card-imposed overrides, adapted here as the typed active-modifier list in (kind, activation-index) order (MtG layers system; Argentum engine writeup). All consulted material informed mechanics and engineering patterns only; no rules prose, card text, names, or presentation was or may be copied.

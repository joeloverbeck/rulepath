# Gate 11 Masked Claims Bluff / Reaction-Window Proof

| Field | Value |
|---|---|
| Spec ID | `gate-11-masked-claims-bluff-reaction-proof` |
| Roadmap stage | 11 |
| Roadmap build gate | Gate 11 — bluffing / reaction-window proof |
| Status | Done |
| Date | 2026-06-10 |
| Owner | Rulepath maintainers |
| Primary crate / internal game id | `masked_claims` |
| Public display name | `Masked Claims` unless IP review prefers another neutral original name |
| Browser implementation | Required |
| Authority order | `docs/FOUNDATIONS.md` → `docs/ARCHITECTURE.md` → `docs/ENGINE-GAME-DATA-BOUNDARY.md` → `docs/OFFICIAL-GAME-CONTRACT.md` → `docs/MECHANIC-ATLAS.md` → `docs/AI-BOTS.md` → `docs/UI-INTERACTION.md` → `docs/TESTING-REPLAY-BENCHMARKING.md` → `docs/ROADMAP.md` → accepted ADRs that explicitly supersede those documents → this spec |

Where this spec and a foundation document disagree, the foundation document wins.

> Reader orientation: this spec carries the canonical Rulepath section set: Objective, Scope, Deliverables, Work breakdown, Exit criteria, Acceptance evidence, FOUNDATIONS & boundary alignment, Forbidden changes, Documentation updates required, Sequencing, and Assumptions. Detailed proposed rules, state/effect/view sketches, bot policy, WASM/browser wiring, fixtures, traces, benchmarks, and source notes are preserved below the canonical sections under **Implementation reference**.

## Objective

Implement `masked_claims` as the Gate 11 browser proof for the ROADMAP §13 bluffing exit lines: **claims, challenges, pending responses, reaction windows, conditional resolution, and no-leak logs**.

Gate 10/10.1 proved betting/showdown and trick-taking over hidden cards. This gate adds the missing interaction class before the Gate 12–14 event/asymmetry games: an action whose resolution is **suspended** while another seat holds a constrained response window, and whose outcome is **conditional** on both the response choice and hidden information that may or may not reveal.

The result is a small, original, deterministic, two-seat browser game, `Masked Claims`, that proves:

- a claim action that binds a hidden component to a public declaration;
- a reaction window in which only the responding seat has legal actions and the log explains who may respond and why;
- conditional resolution: accept (claim scores as declared, hidden component never reveals) versus challenge (hidden component reveals, graded honest/exposed scoring applies);
- deterministic scoring, terminal detection, and tie-breaks;
- Rust-owned legality, previews, effects, replay, visibility projection, and bot decisions in **both** the claim phase and the reaction window;
- browser-safe no-leak behavior across payloads, DOM, `data-testid`, local storage, logs, dev panels, replay exports, and bot explanations — including masks that are *never* revealed for the lifetime of the match.

This is not an engine-generalization gate. It is a small official game that extends the proven `high_card_duel` / `secret_draft` / `plain_tricks` hidden-information machinery while keeping all claim, challenge, reaction-window, mask, grade, and scoring nouns local to `games/masked_claims`.

## Scope

### In scope

- New official game crate `games/masked_claims` with typed Rust setup, state, actions, validation, application, effects, visibility projection, replay support, variants, UI metadata, and bots.
- Default two-seat variant `masked_claims_standard` / public name `Masked Claims`.
- Original deterministic rules: fifteen mask tiles in five grades, deterministic shuffle and deal, five-tile private hands, eight alternating claim turns, one accept/challenge reaction window per claim, graded honest/exposed resolution, fixed terminal turn count, public deterministic scoring, and a public deterministic tie-break ladder.
- Game-local typed nouns only: mask tile IDs, grades, claim ladder, claim pedestal, reaction window, response choices, veiled/exposed galleries, and scoring. These stay inside `games/masked_claims` and its docs/tests/UI projection.
- **Pre-implementation atlas work** (work item 1): reopen the `deterministic shuffle / private hand / staged reveal` ledger row for its fourth official use, record the Stage-11 review of the `simultaneous commitment/reveal` candidate row, and record `reaction window/pending response` as realized first official local use — all before implementation code is written.
- Viewer-safe action trees in both phases. During a reaction window, the claimant's tree is empty with safe waiting metadata; the responder's tree contains exactly the typed response choices.
- Public pending state: whose claim is on the pedestal, the declared grade, which seat may respond, turn number, and public galleries/scores — never the face-down tile identity.
- Reaction-window and reveal effects emitted by Rust and rendered by React, with reduced-motion support and log copy that states who may respond and why.
- Level 0 random legal bot and Level 1 rule-informed `MaskedClaimsLevel1Bot` covering **both roles**: a claim policy (honest/underclaim/bluff selection with a deterministic parameterized bluff rate) and a response policy (certain-lie detection by public counting plus a calibrated challenge threshold), both using the normal legal-action API and only the bot seat's allowed view.
- Full official-game evidence: unit/rule/property/replay/serialization/visibility/no-leak/bot tests, golden traces, simulations, fixture validation, rule coverage, benchmarks, per-game docs (thirteen, including `HOW-TO-PLAY.md` and `PRIMITIVE-PRESSURE-LEDGER.md`), WASM registration, browser board, E2E smoke, a11y/no-leak checks, player-rules and outcome-explanation surfaces, and CI/tool registration.
- Documentation updates per the section below, including the `specs/README.md` index flip and the web-shell catalog closeout surfaces.

### Out of scope

- Three- and four-seat variants. Two seats prove the reaction-window contract; multi-seat bluffing adds kingmaking/elimination design questions (unresolved in the consulted research) without strengthening the Gate 11 proof.
- Cancellation/replacement reactions, counter-claims, blocks, challenge-the-block chains, or nested/stacked reaction windows. ROADMAP §13 scopes cancellation/replacement as optional ("if scoped"); this gate deliberately proves one clean single-depth window.
- Hosted multiplayer, accounts, matchmaking, server persistence, chat, ranked play, or network-time reaction deadlines. The window is turn-structured and timeout-free, per the mechanic-atlas reaction category.
- A generic reaction-window engine, pending-response engine, claim engine, bluff engine, interrupt stack, or priority system in `engine-core` or `game-stdlib`. The atlas marks broad reaction-window generalization `ADR-required`.
- Hidden-role decks, role abilities, named character rosters, or social-deduction identity mechanics. Masks carry a numeric grade only; there are no roles.
- MCTS/ISMCTS/Monte Carlo/ML/RL bot work, opponent-modeling beliefs beyond public counting, or hidden-state sampling.
- Ticket decomposition. The Work breakdown lists bounded candidate AGENT-TASKs only; ticket files are a later step.

### Not allowed

Carried from ROADMAP §13 and tightened for this gate:

- Trademark-forward hidden-role names, proprietary role/card text, or trade-dress imitation of published bluffing games (Coup, Mascarade, Skull, Sheriff of Nottingham, Cockroach Poker, Perudo, or similar). Mechanics-level similarity is acceptable; names, prose, labels, art direction, and theme must be original.
- A generic reaction window in `engine-core`, or `engine-core` nouns such as `claim`, `challenge`, `reaction`, `response window`, `bluff`, `mask`, `grade`, `card`, `deck`, `hand`, or `pedestal` beyond existing generic actor/viewer/action/effect/replay envelopes.
- Generic `game-stdlib` helpers such as `ReactionWindow`, `PendingResponse`, `ClaimLadder`, `ChallengeResolver`, `BluffPolicy`, or hidden-claim helpers. The only `game-stdlib` change this gate may make is a narrow behavior-free helper **explicitly authorized by the reopened fourth-use shuffle ledger decision**, with the full atlas §5A promotion/back-port process.
- Static data formulas for claim legality, challenge resolution, scoring, penalties, tie-breaks, selectors, triggers, or conditions.
- Hidden tile identities in DOM, `data-testid`, local storage, browser payloads, effect logs, diagnostics, replay exports, dev panels, bot explanations, or candidate rankings — before reveal for challenged masks, and **forever** for accepted masks, unplayed hand tiles, and the undealt reserve.
- Actual hidden-state sampling by bots, omniscient challenge decisions, or response policies conditioned on the opponent's face-down tile.
- TypeScript legality, reveal timing, challenge resolution, scoring, terminal detection, tie-breaks, replay authority, or bot policy.
- Accidental trace/hash/schema migration. Any intentional migration needs explicit notes and accepted review.

## Deliverables

| Area | Required artifacts |
|---|---|
| Atlas / ledger (pre-implementation) | `games/masked_claims/docs/PRIMITIVE-PRESSURE-LEDGER.md` recording (a) the fourth-use `deterministic shuffle / private hand / staged reveal` hard-gate decision before any shuffle/deal/visibility code, cross-referencing `games/plain_tricks/docs/PRIMITIVE-PRESSURE-LEDGER.md`; (b) the first-use `reaction window / pending response` record; `docs/MECHANIC-ATLAS.md` §10B updates for both rows plus the recorded Stage-11 review outcome of the `simultaneous commitment/reveal` row. |
| Workspace and crate | Root `Cargo.toml` registration; `games/masked_claims/Cargo.toml`; source modules `src/actions.rs`, `src/bots.rs`, `src/effects.rs`, `src/ids.rs`, `src/lib.rs`, `src/replay_support.rs`, `src/rules.rs`, `src/setup.rs`, `src/state.rs`, `src/ui.rs`, `src/variants.rs`, `src/visibility.rs`. Mirror the `games/plain_tricks` file-for-file shape unless a file is explicitly documented as not applicable. `plain_tricks` is the closest template for deterministic shuffle/deal/private-hand machinery; `high_card_duel` for face-down placement with later reveal; `secret_draft` for waiting-state UI metadata and redacted-forever surfaces. |
| Static data | `games/masked_claims/data/manifest.toml`, `games/masked_claims/data/variants.toml`, `games/masked_claims/data/fixtures/masked_claims_standard.fixture.json`. Static files contain typed metadata, constants, labels, variant IDs, and fixtures only; no behavior. Unknown and behavior-looking fields are rejected. |
| Benchmarks | `games/masked_claims/benches/masked_claims.rs`, `games/masked_claims/benches/thresholds.json`. Include claim-phase legal actions, reaction-window legal actions, validate/apply for claim and both responses, challenge resolution, project-view (pending and post-reveal), replay/hash, public export, and Level 1 bot claim + response decisions. Initial thresholds are smoke floors plus a named calibration follow-up under ADR 0002/0003/0005. |
| Native tests | `games/masked_claims/tests/rules.rs`, `property.rs`, `replay.rs`, `serialization.rs`, `visibility.rs`, `bots.rs`. Coverage detailed under Acceptance evidence. The integration-test shape follows the hidden-information-game convention (`tests/rules.rs` as a dedicated rule-test file, matching `high_card_duel`/`secret_draft`/`poker_lite`/`token_bazaar`); note that the closest deterministic-shuffle template, `plain_tricks`, instead inlines its rule tests in `src/rules.rs` and ships no `tests/rules.rs` — do not let the "mirror `plain_tricks` file-for-file" guidance above drop this file. |
| Golden traces | Under `games/masked_claims/tests/golden_traces/`: the seventeen traces listed under Acceptance evidence. |
| Per-game docs | Thirteen docs instantiated from `templates/*`: `games/masked_claims/docs/AI.md`, `BENCHMARKS.md`, `BOT-STRATEGY-EVIDENCE-PACK.md`, `COMPETENT-PLAYER.md`, `GAME-IMPLEMENTATION-ADMISSION.md`, `HOW-TO-PLAY.md`, `MECHANICS.md`, `PRIMITIVE-PRESSURE-LEDGER.md`, `PUBLIC-RELEASE-CHECKLIST.md`, `RULE-COVERAGE.md`, `RULES.md`, `SOURCES.md`, `UI.md`. |
| Tools | Register `masked_claims` in `tools/simulate`, `tools/replay-check`, `tools/fixture-check`, `tools/rule-coverage`, `tools/bench-report` if thresholds/reporting enumerate game IDs, and `tools/seed-reducer` / `tools/trace-viewer` if their dispatch tables need game IDs or effect names. |
| WASM/API | Register `masked_claims` in `crates/wasm-api/src/lib.rs` catalog, setup, action, bot, effect, view, replay/export/import, and no-leak redaction paths. `get_view(match_id, viewer_seat)` honors viewer scope; public export defaults to viewer-scoped observation timeline under ADR 0004, with claim action paths redacted to declared grades. |
| Browser | `apps/web/src/components/MaskedClaimsBoard.tsx`; GamePicker/catalog support through Rust metadata; `ActionControls` support for the reaction window (responder choices; claimant waiting state) without TS legality; `EffectLog` / `effectFeedback.ts` entries for claim, window-open, accept, challenge, reveal, and resolution; outcome explanation entries in `apps/web/src/components/outcomeExplanationTemplates.ts` and viewer-safe rationale mirrors in `apps/web/src/wasm/client.ts`; player-facing rules at `apps/web/public/rules/masked_claims.md` generated from `games/masked_claims/docs/HOW-TO-PLAY.md` via `scripts/copy-player-rules.mjs`; `masked_claims` added to the `HIDDEN_INFO_GAMES` set in `scripts/check-player-rules.mjs`; shell reducer/client type coverage; safe dev panel output; replay import/export wiring; reduced-motion support; responsive, accessible board presentation. |
| Browser smoke | `apps/web/e2e/masked-claims.smoke.mjs` plus a11y/no-leak checklist updates. Smoke must cover human claim, reaction-window prompt and waiting state, accept resolution, challenge resolution with reveal, bot claim, bot response, replay step/export/import, reduced motion, and no hidden tile ID in DOM/storage/logs/test IDs for unrevealed masks. |
| CI | `.github/workflows/gate-1-game-smoke.yml` native smoke, replay, fixture, rule coverage, web build, and E2E registration; `.github/workflows/gate-2-benchmarks.yml` smoke and threshold registration. |
| Repository docs | `specs/README.md` Gate 11 row maintenance; `docs/MECHANIC-ATLAS.md` updates per the atlas deliverable; `progress.md` and root `README.md` after implementation; no ROADMAP progress edit. |

## Work breakdown

Bounded candidate AGENT-TASKs, in dependency order. Do not decompose these into ticket files as part of this spec.

| # | Candidate task | Depends on | Notes / forbidden drift |
|---:|---|---|---|
| 1 | Pre-implementation primitive-pressure decisions | — | Write `games/masked_claims/docs/PRIMITIVE-PRESSURE-LEDGER.md`: fourth-use hard-gate decision for deterministic shuffle / private hand / staged reveal (reuse / promote narrow behavior-free helper / defer-reject / ADR), first-use reaction-window record, and the Stage-11 `simultaneous commitment/reveal` review outcome. Update atlas §10B. **Blocks all implementation tasks.** If the decision is promote, the same gate must carry the §5A back-port/debt plan. |
| 2 | Crate skeleton and workspace registration | 1 | Add `games/masked_claims` with IDs, variants, setup constants, manifest/variant parsers, fixture shell. No behavior in data. |
| 3 | State model, typed IDs, deterministic setup | 2 | Model hands, pedestal (face-down tile + declared grade), reaction-window phase, veiled/exposed galleries, reserve, scores, turn counter, terminal outcome, freshness token. Deterministic shuffle/deal per the task-1 ledger decision, using existing `SeededRng` discipline. |
| 4 | Claim-phase action tree and validation | 3 | Actor-specific legal tree: one claim per held tile × declared grade from the ladder. Validate freshness, actor seat, phase, tile possession, and grade range. Rust owns all preview metadata. |
| 5 | Reaction-window phase and pending-response tree | 4 | On claim apply: open window, emit `ClaimPlaced` + `ReactionWindowOpened` with viewer-safe who-may-respond-and-why copy; responder tree contains exactly `respond/accept` and `respond/challenge`; claimant tree is empty with waiting metadata. Validate wrong-phase, wrong-seat, and stale submissions with safe diagnostics. |
| 6 | Conditional resolution, scoring, terminal | 5 | Accept: score declared grade, move tile face-down to veiled gallery, never reveal. Challenge: emit reveal, grade honest/exposed resolution, graded awards, gallery moves. Turn advance, terminal after the fixed claim count, deterministic tie-break ladder. |
| 7 | Visibility and replay surfaces | 6 | Public/seat views, effect filtering, internal full trace, viewer-scoped export/import with claim-path redaction, stable summaries, action/effect/view hashes, no-leak helpers. Accepted masks, unplayed hand tiles, and the reserve stay redacted in every export forever. ADR 0004 rules are mandatory. |
| 8 | Level 0 and Level 1 bots | 5,7 | Both roles: claim policy (honest default, deterministic parameterized bluff/underclaim selection) and response policy (certain-lie counting, challenge threshold). Legal action API only; own allowed view only; deterministic under declared inputs; viewer-safe rationale. |
| 9 | Native tests and golden traces | 6,7,8 | Rule/property/replay/serialization/visibility/bot suite and full golden trace set. Follow the failing-test protocol; never weaken tests to get green. |
| 10 | Benchmarks and thresholds | 9 | The benchmark identity list under Implementation reference. Smoke floors first, calibration follow-up named. |
| 11 | Per-game documentation | 9,10 | Instantiate all thirteen docs. `RULES.md` carries stable rule IDs including scoring/end IDs for the outcome-explanation contract; `HOW-TO-PLAY.md` carries the required player-rules section set. |
| 12 | WASM, tools, and CI registration | 9 | Register game ID across wasm-api, simulate, replay-check, fixture-check, rule-coverage, bench-report, seed-reducer/trace-viewer if needed, and CI lanes. |
| 13 | React board and browser no-leak smoke | 12 | `MaskedClaimsBoard.tsx`, reaction-window UI, effect feedback, outcome-explanation templates, player-rules copy + `HIDDEN_INFO_GAMES` registration, replay UI, E2E smoke, reduced motion, DOM/storage/test-ID no-leak assertions. TS remains presentation-only. |
| 14 | Repository documentation and final admission evidence | 11,12,13 | Spec index flip, atlas confirmation, progress/root README updates after implementation, catalog closeout surfaces, command transcript, unresolved issues. Do not edit ROADMAP as progress diary. |

## Exit criteria

Mapped row-for-row to ROADMAP §13 (Gate 11).

| ROADMAP §13 line | Gate 11 exit criterion |
|---|---|
| logs explain who may respond and why | **Met.** Every claim emits `ReactionWindowOpened { turn, claimant, responder, declared_grade }` whose public log rendering states which seat may respond, with which choices, and why (a claim is pending on the pedestal). Diagnostics for out-of-window submissions name the safe reason. The effect log, replay viewer, and trace-viewer rendering all carry this copy from Rust. |
| bots respond legally | **Met.** Level 0 and Level 1 bots act in both roles through the normal legal-action API: claims only from the claim-phase tree, responses only from the reaction-window tree, validated like human commands, deterministic under declared inputs. Simulation evidence covers many full games with zero illegal bot actions in either phase. |
| hidden claims do not leak | **Met.** The face-down pedestal tile, accepted veiled-gallery masks, unplayed hand tiles, and the undealt reserve never appear in public views, opponent seat views, action-tree metadata, previews, diagnostics, effect payloads, command summaries, DOM, `data-testid`, local storage, logs, dev panels, viewer-scoped replay exports, bot explanations, or candidate rankings. A challenged mask's identity first appears in the `MaskRevealed` effect. Accepted masks never reveal, including at terminal. |
| reaction UI smoke tests pass | **Met.** `apps/web/e2e/masked-claims.smoke.mjs` covers the responder prompt, claimant waiting state, accept flow, challenge flow with reveal animation, bot response, replay of a reaction window, reduced motion, and a11y checks, and passes in CI. |
| any reaction-window abstraction has repeated pressure or ADR before promotion | **Met.** The reaction window is implemented entirely inside `games/masked_claims`. The atlas row records realized first official use and keeps `ADR-required` posture for broad generalization. No `engine-core` or `game-stdlib` reaction/pending/claim vocabulary is added. |
| Not allowed: trademark-forward hidden-role names | **Honored.** There are no roles. The name, grade labels, and all prose are original and IP-reviewed per `docs/IP-POLICY.md`; `SOURCES.md` records the originality rationale and the consulted-mechanics-only posture. |
| Not allowed: proprietary role/card text | **Honored.** All rules prose, labels, and explanations are original Rulepath text. |
| Not allowed: generic reaction window in `engine-core` | **Honored.** Forbidden-changes list bans reaction/claim/challenge vocabulary in `engine-core`; `scripts/boundary-check.sh` plus kernel review enforce it. |

## Acceptance evidence

### Native rules, replay, visibility, and bot evidence

- `cargo test -p masked_claims` passes.
- Rule tests cover setup and deal, claim legality for every held tile and grade, reaction-window legality (responder-only), accept resolution, honest-challenge resolution (including underclaim), exposed-challenge resolution with graded award, gallery moves, turn alternation, terminal after the fixed claim count, scoring, and the full tie-break ladder.
- Diagnostics tests cover stale freshness tokens, wrong-seat claims, wrong-phase responses (claim during a window; response outside a window), unowned-tile claims, and out-of-range grades — all with viewer-safe messages that do not hint at hidden tiles.
- Property tests cover many deterministic seeds and legal action sequences asserting: hand+pedestal+gallery+reserve conservation (always fifteen tiles), exactly one open window at a time, no reveal of accepted masks, terminal at the fixed turn count, score consistency with the resolution rules, and no panics.
- Replay tests prove same seed + seats + options + command stream reproduces state hashes, effect hashes, action-tree hashes, public/seat-view hashes, reveal ordering, and terminal outcome.
- Serialization tests prove stable summaries and unknown-field rejection for manifest, variants, fixtures, viewer-scoped export, and internal trace helpers.
- Visibility/no-leak tests search public views, opponent seat views, action trees, previews, diagnostics, effect payloads, public effect text, command summaries, public export/import timelines, bot explanations, and candidate rankings for unrevealed tile IDs — covering the pedestal pre-resolution, veiled galleries, hands, and reserve.
- Bot tests prove Level 0 and Level 1 select only legal action paths in both phases, are deterministic under declared inputs, finish many games, and never change a response decision or rationale when the hidden pedestal tile differs but the bot's allowed view is identical.
- Balance evidence: mirrored Level 1 vs Level 1 simulation across both seatings reports per-seat win rates; a material asymmetry (outside roughly 40–60%) triggers a scoring-constant retune (Assumption A4) before public polish, recorded in `BENCHMARKS.md`/`COMPETENT-PLAYER.md`.

### Golden traces

Committed under `games/masked_claims/tests/golden_traces/` and checked by `replay-check`:

- `shortest-normal.trace.json` (accept-only match)
- `claim-pending-window.trace.json`
- `accept-resolution.trace.json`
- `challenge-honest-reveal.trace.json`
- `challenge-exposed-lie.trace.json`
- `underclaim-trap-reveal.trace.json`
- `certain-lie-challenge.trace.json` (challenge justified by public counting)
- `terminal-tie-break.trace.json`
- `draw-after-tie-breaks.trace.json`
- `stale-diagnostic.trace.json`
- `wrong-phase-claim-diagnostic.trace.json`
- `wrong-seat-response-diagnostic.trace.json`
- `unowned-tile-diagnostic.trace.json`
- `public-observer-no-leak.trace.json`
- `accepted-mask-never-revealed.trace.json` (full match; veiled gallery redacted at terminal)
- `bot-claim-and-response.trace.json`
- `public-replay-export-import.trace.json`

### Tools, benchmarks, browser, docs, and CI

- `cargo run -p simulate -- --game masked_claims --games 1000` finishes with no illegal bot action or invariant failure.
- `cargo run -p replay-check -- --game masked_claims --all` passes.
- `cargo run -p fixture-check -- --game masked_claims` passes.
- `cargo run -p rule-coverage -- --game masked_claims` passes and maps every rules-doc obligation to tests/traces.
- `cargo bench -p masked_claims` runs the benchmark identity list; `bench-report` enforces smoke floors where calibrated, with the calibration follow-up named in the handoff.
- `npm --prefix apps/web run smoke:wasm`, `smoke:ui`, and `smoke:e2e` pass after `masked_claims` is registered.
- `node apps/web/e2e/masked-claims.smoke.mjs` covers the reaction-window UI flows listed under Exit criteria.
- `bash scripts/boundary-check.sh` passes with no new `engine-core` mechanic nouns and no TypeScript legality drift.
- `node scripts/check-doc-links.mjs` passes.
- `node scripts/check-catalog-docs.mjs` passes, confirming the mechanically-checked catalog surfaces name `masked_claims` / `Masked Claims`.
- `node scripts/check-player-rules.mjs` passes with `masked_claims` in `HIDDEN_INFO_GAMES` and the generated `apps/web/public/rules/masked_claims.md` in sync.
- `node scripts/check-outcome-explanations.mjs` passes with the masked-claims outcome templates and rule-ID mirrors registered.
- All thirteen per-game docs are present, link-checkable, original, and consistent with implemented behavior.

## FOUNDATIONS & boundary alignment

| Principle | Stance | Rationale |
|---|---|---|
| §2 Behavior authority | Aligned | Rust owns setup, shuffle/deal, claim legality, reaction-window membership and timing, response legality, conditional resolution, reveal timing, scoring, terminal detection, tie-breaks, semantic effects, replay/export projection, and bot decisions. React presents Rust-provided data, including the waiting state. |
| §3 `engine-core` contract kernel | Aligned | The reaction window is a game-local phase expressed through existing generic `Actor`/`Viewer`/`ActionTree`/`CommandEnvelope`/`EffectEnvelope`/`VisibilityScope`/replay contracts — the actor-specific empty-tree-while-waiting pattern was already proven by `secret_draft`. `claim`, `challenge`, `reaction`, `mask`, `grade`, and gallery nouns remain game-local. `ARCHITECTURE.md` already names commitments/reveals, pending responses, and grouped batches as expected effect shapes behind generic envelopes. |
| §4 `game-stdlib` earned | Aligned | No reaction/claim helper is promoted; the atlas marks broad reaction-window generalization `ADR-required`. The fourth-use shuffle/private-hand pressure is resolved by the work-item-1 ledger decision **before** implementation, satisfying the §4 hard-gate rule; any promotion it authorizes carries the full §5A back-port/debt process inside this gate. |
| §5 Static data is typed content | Aligned | Manifest/variants/fixtures hold tile IDs, grade labels, constants, and metadata only. Claim legality, resolution, scoring, and tie-breaks are typed Rust. Unknown and behavior-looking fields are rejected. |
| §6 Official games are evidence-heavy | Aligned | The Acceptance-evidence section carries the full official-game contract: rule/property/replay/serialization/visibility/no-leak/bot tests, the seventeen golden traces, `simulate`/`replay-check`/`fixture-check`/`rule-coverage` runs, benchmarks, and all thirteen per-game docs — browser playability alone is not treated as done. |
| §7 Public UI is central product work | Aligned | `MaskedClaimsBoard.tsx` renders legal-only controls in both phases (response buttons appear only when Rust's tree contains them; the claimant gets a waiting state), animation is driven by Rust semantic effects with reduced-motion support, and the renderer settles to the latest viewer-safe view. TypeScript invents no legality. |
| §8 Public bots | Aligned | Bots act in both the claim and response roles through the same legal action API as humans, with deterministic parameterized policies (equilibrium-informed bluff/challenge rates, public counting), viewer-safe explanations, and no hidden-state access, MCTS/ISMCTS, Monte Carlo, ML, or RL. |
| §9 Local-first v1/v2 | Aligned | The reaction window is turn-structured and timeout-free; Out-of-scope and Forbidden-changes exclude hosted multiplayer, accounts, matchmaking, server persistence, chat, ranked play, and network-time reaction deadlines. Command logs, deterministic replay, and viewer-scoped export keep the future-hosted path open without browser-owned authoritative state. |
| §10 IP conservatism | Aligned | Mechanics-level prior art (claim/challenge loops, graded penalties, ordered claim spaces) is unprotected rules territory; all names, labels, prose, and visuals are original; no role rosters exist to imitate; `SOURCES.md` records consulted sources and the originality rationale. |
| §11 Universal invariants | Aligned | Deterministic replay/hash/serialization, viewer-safe views, never-revealed hidden surfaces, legal-only UI in both phases, semantic-effect animation, local-first scope, docs/traces/simulations/benchmarks, and bounded agent output are explicit acceptance evidence. |
| §12 Stop conditions | Clear | Stop if a kernel noun appears, static data turns procedural, TypeScript decides legality or reveal timing, any unrevealed tile ID reaches a browser/dev/replay/bot surface, a bot bypasses the legal API or reads the pedestal, the fourth-use ledger decision is skipped, or a reaction-window helper is generalized without ADR. |
| §13 ADR triggers | **No ADR expected.** | The reaction window stays game-local; visibility/replay contracts, kernel vocabulary, data policy, and renderer defaults are unchanged. If the work-item-1 ledger decision escalates to ADR, or implementation genuinely requires changing replay/hash semantics or visibility contracts, stop and write the ADR before proceeding. |
| ADR 0004 hidden-info replay/export | Aligned | Internal full traces remain native test authority. Browser export defaults to the viewer-scoped observation timeline: claim commands are redacted to declared grades; challenged masks appear only from their reveal; accepted masks, hands, and the reserve never appear. |
| Benchmark ADRs 0002/0003/0005 | Aligned | Smoke benchmarks and threshold files now; variance-aware calibration follow-up named; PR smoke stays non-gating. |
| ADR 0006 Blackjack placement | Aligned | Untouched. Gate 11 is not a comparison-case trigger for `blackjack_lite`. |

## Forbidden changes

Do not, in this gate:

- Add `claim`, `challenge`, `reaction`, `response window`, `pending response`, `bluff`, `mask`, `grade`, `card`, `deck`, `hand`, `pedestal`, `gallery`, or similar mechanic/domain vocabulary to `engine-core`.
- Add generic `game-stdlib` primitives for reaction windows, pending responses, claim ladders, challenge resolution, bluff policy, hidden-claim storage, or waiting-state UI policy. The single permitted exception is a narrow behavior-free helper explicitly authorized by the work-item-1 fourth-use ledger decision, with tests/docs/examples/anti-examples/benchmarks and same-gate back-ports per atlas §5A.
- Skip, reorder after implementation, or rubber-stamp the work-item-1 primitive-pressure decisions.
- Introduce behavior-in-data, YAML, DSLs, formulas, selectors, triggers, conditions, rule scripts, procedural data, hidden defaults, or untyped nested behavior objects.
- Let TypeScript compute legality, window membership, reveal timing, challenge resolution, scoring, terminal outcome, tie-breaks, replay authority, bot policy, or no-leak filtering.
- Leak unrevealed tile identities through any surface enumerated in the Exit criteria no-leak row, including post-terminal views and exports for accepted masks, hands, and the reserve.
- Use MCTS, ISMCTS, Monte Carlo, ML, RL, LLMs, hidden-state sampling, pedestal peeking, omniscient challenge heuristics, or giant opaque weight tables.
- Copy proprietary bluffing-game rules prose, names, role rosters, card text, icons, art, screenshots, or trade dress.
- Add hosted multiplayer, accounts, server authority, matchmaking, chat, ranked play, persistence, reaction timeouts, or cryptographic commitments.
- Change trace schema, replay/hash semantics, data versions, action path stability, effect ordering, public-view shape, or golden traces accidentally.
- Delete, rename away, weaken, or rewrite tests merely to get green output. Follow the failing-test protocol.

## Documentation updates required

- `specs/README.md`: Gate 11 row points at this spec with status `Planned` on acceptance; flip to `In progress` when AGENT-TASKs execute and `Done` only after exit criteria pass with evidence.
- Do **not** edit `docs/ROADMAP.md` to record progress.
- `docs/MECHANIC-ATLAS.md`:
  - §10B `deterministic shuffle / private hand / staged reveal`: record the fourth-use reopen decision from work item 1 (and §10A debt if the decision is promote-with-debt — otherwise confirm §10A stays `_None_`).
  - §10B `reaction window/pending response`: convert from candidate to realized first official local use; keep the `ADR-required if generalized broadly` posture; set the next-gate trigger to the next reaction-capable game (Gate 12+ event games).
  - §10B `simultaneous commitment/reveal + visible draft-pool removal`: record the Stage-11 review outcome (expected: the masked-claims pedestal is a single-seat sequential hidden placement with conditional reveal, not a second simultaneous-commitment use; row stays first-use candidate unless implementation evidence contradicts this).
- Author all thirteen `games/masked_claims/docs/*` from templates and keep them consistent with implemented behavior; `SOURCES.md` records consulted prior art (see Implementation reference), what was and was not used, and the originality/naming review.
- Update `progress.md` and root `README.md` after implementation, not before evidence passes.
- Reconcile the web-shell catalog surfaces as a closeout inside this gate (per `specs/README.md` §10), not a later aftermath pass:
  - `apps/web/README.md` intro catalog list (add `masked_claims` / `Masked Claims`);
  - root `README.md` "current official games are" list;
  - `apps/web/README.md` Smoke Layers `smoke:e2e` bullet, and the `smoke:e2e` script in `apps/web/package.json` (add `node e2e/masked-claims.smoke.mjs`);
  - the `apps/web/README.md` Shell Surface renderer list (process-enforced).
  - `node scripts/check-catalog-docs.mjs` enforces the mechanically-checked surfaces in CI; it must pass.
- Player-rules and outcome-explanation surfaces: `games/masked_claims/docs/HOW-TO-PLAY.md` → `apps/web/public/rules/masked_claims.md` via `scripts/copy-player-rules.mjs`; `masked_claims` added to `HIDDEN_INFO_GAMES` in `scripts/check-player-rules.mjs`; outcome templates/rule-ID mirrors registered so `scripts/check-outcome-explanations.mjs` passes.
- Ensure `scripts/check-doc-links.mjs` passes after doc changes.

## Sequencing

- **Predecessor:** Gate 10.1 (`plain_tricks`) is `Done` in the spec index; it closed the Gate 10 trick rows and recorded the third-use card/private-hand hard-gate decision this gate's work item 1 reopens for a fourth use.
- **Admission rule:** Before implementation starts, confirm `docs/MECHANIC-ATLAS.md` §10A still has no open promotion debt (it records `_None_` at spec time), and complete work item 1's ledger decisions before any shuffle/deal/visibility code is written.
- **This gate:** Gate 11 proves claims, challenges, pending responses, reaction windows, conditional resolution, and no-leak logs per ROADMAP §13.
- **Successor:** Gate 12 (`flood_watch`) remains not yet specced. It should not start until Gate 11 evidence passes and no open promotion debt remains. The Gate 12 spec author should check the atlas reaction-window row: a second reaction-capable game is the repeated-pressure trigger for that row's review.

## Assumptions

- A1: Two seats are the sufficient proof shape for Gate 11; 3–4 player bluffing adds kingmaking/elimination questions the consulted research left open, without strengthening the reaction-window proof.
- A2: Public display name `Masked Claims` and the grade labels (`Plain`, `Trimmed`, `Gilded`, `Jeweled`, `Master`) are original placeholders; maintainers may rename after IP review before implementation.
- A3: Standard variant uses fifteen tiles (three per grade 1–5), five-tile hands, a five-tile never-dealt reserve, and four claims per seat (eight turns); maintainers may tune counts while preserving the fixed terminal, hidden-residue, and proof scope.
- A4: Scoring constants (accept = declared grade; honest challenge = actual grade + 2 truth bonus; exposed challenge = claimant 0, challenger declared − actual) are starting parameters chosen to sit in the no-pure-equilibrium regime; maintainers tune them with simulation evidence while preserving the anti-degeneracy condition that neither always-challenge nor never-challenge is a best response.
- A5: Accepted masks never reveal — including at terminal and in every export — and the per-seat unplayed tile and the reserve never reveal. This stricter posture is intentional for the no-leak proof and makes bluffing pay.
- A6: Claim action paths carry the tile ID internally; every public surface (command summaries, exports, logs) shows only the declared grade, under ADR 0004 redaction. Internal native traces may contain raw private action paths.
- A7: The fourth-use shuffle/private-hand ledger reopen is expected to resolve defer/reject or, at most, a narrow behavior-free shuffle helper; whichever way it resolves, the §5A conformance rules bind within this gate, and an ADR fires only if the decision escalates.
- A8: A single-depth accept/challenge window is the Gate 11 proof; cancellation/replacement and nested windows are explicitly out of scope per ROADMAP §13's "if scoped" wording.
- A9: Level 0 + Level 1 bots satisfy the gate (mirroring Gates 9.1–10.1); a Level 2 claim requires the full `BOT-STRATEGY-EVIDENCE-PACK.md` evidence workflow.
- A10: If any assumption is wrong, correct the smallest local game/spec surface; do not generalize the engine.

---

# Implementation reference

## Product intent / what this gate proves

`Masked Claims` is a deliberately compact original bluffing game. Its job is to make the architecture prove one hard interaction shape cleanly: a seat binds a hidden component to a public declaration, the opposing seat holds a constrained pending-response window whose existence and reasons are fully explained in the logs, and resolution is conditional — on the response choice and, only when challenged, on hidden information that reveals at exactly that moment. Accepted claims never reveal, so the no-leak obligation extends across the whole match lifetime, including terminal views and replay exports.

The design draws on three verified bodies of prior art at the mechanics level only:

- **Reaction-window state machines.** Claim/challenge flows in published bluffing games have been formally modeled as deterministic phase machines in which non-acting players hold constrained response options and transitions are deterministic given all submitted actions; production digital frameworks model response windows as within-turn states whose move sets *replace* the global action space by construction. Masked Claims adopts exactly this: a `ReactionWindow` phase in which the responder's legal tree contains only the typed responses and the claimant's tree is empty with waiting metadata — legality by construction, not by validation patches.
- **Anti-degeneracy from game theory.** Small bluffing games collapse when a deterministic strategy dominates: agents that price a bluff against a realistic challenge rate stop bluffing entirely if being caught is too costly, and the game degenerates into a deterministic race; conversely, equilibrium analysis of minimal bluffing games (informed player bluffs 1/3, caller calls 2/3 in the canonical stripped-down poker example) shows the payoff structure must leave both bluffing and challenging profitable at intermediate frequencies. The scoring constants below were chosen so that against a never-challenge opponent, max-bluffing dominates honesty, while against an always-challenge opponent, honesty (and deliberate underclaiming) dominates bluffing — placing optimal play strictly between.
- **Graded penalties and ordered claim spaces.** Claim spaces that are small, enumerable, and ordered keep the action tree flat and bot policy explainable; penalties graded by *how wrong* the loser was (rather than binary) reward calibrated bluffs. Masked Claims uses a five-grade ladder and awards the challenger the gap between declared and actual grade on an exposed lie.

## Proposed original rules: `Masked Claims`

### Components

- Two seats: `seat_0`, `seat_1`.
- Fifteen mask tiles: three copies of each grade `1..=5`, stable IDs `mask_g1_a` … `mask_g5_c`. Grade display labels (original, IP-reviewed placeholders): 1 `Plain`, 2 `Trimmed`, 3 `Gilded`, 4 `Jeweled`, 5 `Master`.
- A claim pedestal holding at most one face-down tile plus its public declared grade.
- Per-seat private hand (five tiles at setup).
- A five-tile face-down reserve that is never dealt and never revealed.
- Per-seat public **veiled gallery** (accepted claims, face-down forever) and **exposed row** (revealed tiles from challenges, face-up).
- Public scores, turn marker `1..=8`, and tie-break counters.

### Setup

1. Validate exactly two seats.
2. Load `masked_claims_standard` constants from typed Rust/validated static metadata.
3. Deterministically shuffle the fifteen tiles with `SeededRng` (per the work-item-1 ledger decision) and deal five to each seat; the remaining five form the hidden reserve.
4. Set turn `1` (claimant `seat_0`; claimant alternates every turn), empty pedestal, empty galleries, scores `0`, terminal `None`, freshness token `0`.
5. Emit setup/view state through Rust projection only. Each seat's view contains its own hand; no view contains the opponent hand or reserve.

### Turn flow

Each of the eight turns has the same two-phase shape.

1. **Claim phase.** The turn's claimant receives a legal tree with one choice per held tile × declared grade `1..=5` (`claim/<tile-id>/<grade>` internally; public summary `claim/grade-<g>`). The responder's tree is empty with safe waiting metadata. Submitting a claim moves the tile face-down to the pedestal, records the declared grade publicly, and opens the window: Rust emits `ClaimPlaced { turn, claimant, declared_grade }` and `ReactionWindowOpened { turn, responder, choices: [accept, challenge] }` with log copy explaining who may respond and why. No payload carries the tile ID.
2. **Reaction window.** Only the responder has legal actions: `respond/accept` or `respond/challenge`. The claimant's tree is empty ("waiting for Seat 1 to respond"). Stale/wrong-phase/wrong-seat submissions get viewer-safe diagnostics.
3. **Conditional resolution.**
   - **Accept:** the claimant scores the declared grade. The pedestal tile moves face-down into the claimant's veiled gallery and is never revealed. Effects: `ClaimAccepted { turn, declared_grade }`, `ScoreChanged`, `TurnAdvanced` or `Terminal`. No tile ID appears.
   - **Challenge:** Rust reveals the tile — `ChallengeDeclared { turn, responder }`, `MaskRevealed { turn, tile_id, actual_grade }` (the first public appearance of the identity) — then resolves:
     - **Honest claim (actual ≥ declared):** the claimant scores `actual + 2` (truth bonus; an underclaimed tile pays its full actual grade). The revealed tile goes face-up to the claimant's exposed row. `ChallengeResolved { outcome: honest, … }`.
     - **Exposed lie (actual < declared):** the claimant scores nothing; the responder scores `declared − actual` (graded by the size of the lie). The revealed tile goes face-up to the responder's exposed row as a trophy. `ChallengeResolved { outcome: exposed, … }`.
4. **Cleanup.** The pedestal clears, the claimant role alternates, and the next turn begins unless turn 8 has resolved.

Each seat makes exactly four claims and ends the match holding one unplayed tile, which remains hidden forever.

### Terminal and tie-breaks

The game ends immediately after turn 8 resolves. The terminal winner uses this public ladder:

1. Higher total score wins.
2. If tied, fewer exposed lies wins.
3. If tied, more successful challenges (lies exposed as responder) wins.
4. If tied, fewer challenges declared in total wins (challenge discipline).
5. If still tied, the terminal outcome is `Draw`.

The final `Terminal` effect carries the winner or draw, final scores, and a public tie-break summary. It does not reveal veiled galleries, remaining hand tiles, or the reserve.

### Why these constants resist degeneracy

With challenge probability `q` against a grade-5 claim on a grade-2 tile: bluff EV `5(1−q)` versus honest EV `2 + 2q` — bluffing pays only while `q < 3/7`. Against always-challenge, honesty yields `actual + 2` every turn and underclaim traps punish the challenger's wasted challenges; against never-challenge, always-claiming `Master` dominates. Neither pure response policy is a best response, which is the formal anti-degeneracy requirement. Public counting adds a deduction layer: three copies per grade means a seat holding or seeing all three copies of a grade can challenge a matching claim with certainty, which keeps late-game claims honest without any hidden-state access. Constants are tunable under Assumption A4 with simulation evidence.

## State, actions, and validation sketch

### State

- `variant: Variant`
- `seats: [SeatId; 2]`
- `turn_number: u8` (1..=8), `claimant: SeatId` (alternates)
- `phase: Phase` — `Phase::Claim`, `Phase::Reaction { responder: SeatId }`, `Phase::Terminal`
- `hands: [Vec<MaskTileId>; 2]` internal; own-seat visible only
- `pedestal: Option<PendingClaim { tile: MaskTileId, declared: Grade }>` — `tile` internal only; `declared` public
- `reserve: Vec<MaskTileId>` internal only, never revealed
- `veiled_gallery: [Vec<MaskTileId>; 2]` internal IDs; public projection shows counts and declared grades only
- `exposed_row: [Vec<(MaskTileId, Grade)>; 2]` public after reveal
- `scores: [u32; 2]`, `exposed_lies: [u8; 2]`, `successful_challenges: [u8; 2]`, `challenges_declared: [u8; 2]`
- `terminal_outcome: Option<TerminalOutcome>`, `freshness_token: FreshnessToken`

### Legal action tree

- Claim phase, claimant: `claim/<tile-id>/<grade>` for every held tile and grade `1..=5`. Choice metadata may include the tile's own grade and label **only in the claimant's seat view**, plus public score preview for the declared grade; never opponent or pedestal data.
- Claim phase, responder / reaction window, claimant: flat empty tree with safe waiting metadata naming who acts next and why.
- Reaction window, responder: exactly `respond/accept` and `respond/challenge`, with viewer-safe metadata (declared grade, public counting facts such as "two Master masks are already exposed" — derived from public state only).
- Terminal: empty trees.

### Validation

Rejects: stale freshness token; actor not seated; terminal phase; claim outside `Phase::Claim` or by the non-claimant; claim of a tile not in the actor's hand; out-of-range grade; response outside `Phase::Reaction` or by the non-responder; malformed/extra path segments. Diagnostics are viewer-safe ("it is not your turn to respond", "that mask is not in your hand") and never reference hidden tile identities.

## Semantic effect model

| Effect | Visibility | Payload rule |
|---|---|---|
| `ClaimPlaced { turn, claimant, declared_grade }` | Public | Declared grade only. No tile ID. |
| `ReactionWindowOpened { turn, responder, choices }` | Public | Names the responder, the allowed responses, and the reason (pending claim). Drives the "who may respond and why" log line. |
| `ClaimAccepted { turn, declared_grade, score_delta }` | Public | No tile ID, ever. |
| `ChallengeDeclared { turn, responder }` | Public | Choice only. |
| `MaskRevealed { turn, tile_id, actual_grade }` | Public, challenge-only | First and only public appearance of a pedestal tile's identity. |
| `ChallengeResolved { turn, outcome, awards }` | Public | `honest` or `exposed`, with graded awards. |
| `ScoreChanged { scores, counters }` | Public | Public scores and tie-break counters. |
| `TurnAdvanced { next_turn, next_claimant }` | Public | Next public decision state. |
| `Terminal { outcome, final_scores, tie_break_summary }` | Public | No reveal of veiled galleries, hands, or reserve. |
| `PrivateDiagnostic` / `PublicDiagnostic` | Private / public | Viewer-safe reasons only. |

React animates from these effects (pedestal placement, window prompt, reveal flip, trophy move) and settles to the latest viewer-safe view. Reduced motion preserves event order and no-leak guarantees.

## Visibility and no-leak model

- **Public observer view:** game/variant metadata; turn, phase, claimant/responder; pedestal declared grade and presence; veiled gallery counts with their declared grades; exposed rows with revealed identities; scores, counters, tie-break summary; terminal outcome. Never: pedestal tile ID pre-reveal, veiled gallery IDs, hand contents, reserve contents, hand-tile metadata in opponent/observer scope.
- **Seat-private view:** adds the seat's own hand (IDs, grades, labels). After the seat commits a claim, its own view shows the pedestal's declared grade and a "your mask is on the pedestal" marker; echoing the seat's own pedestal tile ID back is permitted but unnecessary — the stricter posture (no echo, matching `secret_draft` A6) is recommended for cleaner DOM no-leak assertions.
- **Internal full trace vs browser export (ADR 0004):** internal native traces may carry full command streams including raw `claim/<tile-id>/<grade>` paths, hidden state, and hashes. Viewer-scoped browser export redacts claim commands to declared grades and includes reveals only from their `MaskRevealed` event; veiled galleries, hands, and the reserve never appear in any export at any point, including post-terminal.
- The auxiliary-surface rule is mandatory: undo/history affordances, dev panels, effect logs, fixtures, screenshots, and E2E anchors all draw from the same viewer-scoped projection (the consulted engine prior art's leak postmortem — secrets escaping through undo/initial-state history while the primary view was filtered — is the named anti-pattern).

## Bot policy

### Level 0

`MaskedClaimsRandomBot` selects uniformly from the legal tree in both phases using deterministic bot RNG helpers. It never constructs out-of-tree actions and never reads hidden opponent state.

### Level 1

`MaskedClaimsLevel1Bot`, deterministic under declared inputs (seat view + bot seed), with viewer-safe explanations:

**Claim policy** (own hand + public state only):
1. Default honest: claim the held tile's actual grade, preferring the highest-grade held tile.
2. Bluff selection: with a deterministic parameterized rate (starting near the equilibrium-informed one-third, tuned by simulation), claim one grade above actual on a mid-grade tile — bounded, explainable lies.
3. Counting guard: never claim a grade for which all three copies are publicly exposed or held by the bot itself in excess (a certain-lie claim).
4. Stable tile-ID tie-break.

**Response policy** (public counting + own hand only):
1. Certain-lie detection: challenge when the declared grade is impossible — all copies of that grade are accounted for in the bot's hand plus exposed rows. Explanation: "Challenged: all three Master masks are already visible to me."
2. Threshold challenge: challenge when the declared grade's plausibility (remaining unseen copies vs. unseen tiles) falls below a calibrated threshold, biased by the declared grade's score value.
3. Otherwise accept. Explanation: "Accepted: a Gilded claim is plausible and worth only 3."

Forbidden explanations: anything referencing the pedestal tile's actual identity, opponent hand contents, or reserve contents. Bot tests assert decisions and rationales are unchanged when hidden state differs but the allowed view is identical.

## WASM/browser wiring

- Catalog entry with `game_id: masked_claims`, display name `Masked Claims`, hidden-information flag, viewer modes, variants, and docs links.
- `get_view(match_id, viewer_seat)` returns viewer-scoped projection per the visibility model. `get_action_tree(match_id, actor_seat)` returns claim choices, response choices, or an empty waiting tree per phase and seat.
- `apply_action` validates and returns safe effects + view; a claim result carries the window-open effects, a response result carries the resolution batch.
- `run_bot_turn` routes both bot roles through the legal tree and validation; decision JSON is viewer-safe.
- `export_replay` defaults to the viewer-scoped observation export with claim-path redaction.
- `MaskedClaimsBoard.tsx` renders own hand, pedestal (face-down with declared grade), reaction prompt or waiting copy, galleries (veiled as face-down count + declared grades; exposed face-up), scores/counters, effect log, and replay controls. Response buttons render only when Rust's tree contains them. Pending anchors use seat/turn IDs, never tile IDs; veiled gallery anchors use position indices.
- Player-rules and outcome-explanation surfaces register per the Deliverables row so `check-player-rules.mjs` and `check-outcome-explanations.mjs` pass.

## Benchmark operations

- `legal_actions_claim_phase`
- `legal_actions_reaction_window`
- `validate_claim`
- `apply_claim_open_window`
- `apply_accept_resolution`
- `apply_challenge_resolve_reveal`
- `project_public_view_pending_reaction`
- `project_public_view_after_reveal`
- `state_hash_terminal`
- `public_export_timeline`
- `level1_bot_claim_decision`
- `level1_bot_response_decision`

Thresholds start as non-heroic smoke floors with a named calibration follow-up under ADR 0002/0003/0005.

## Source notes and originality guidance

Consulted external sources shape the proof vocabulary, the anti-degeneracy analysis, and the implementation pattern — not the original rules, name, labels, prose, UI, or assets:

- Shafi, Truong & Lee-Heidenreich, "Learning to Play Coup" (Stanford AA228 course report, 2018), consulted for the deterministic four-phase claim/challenge state-machine model and the demonstrated never-bluff degeneration when bluff cost × assumed challenge rate dominates payoff: https://web.stanford.edu/class/aa228/reports/2018/final81.pdf (non-peer-reviewed; used as a modeling pattern only).
- boardgame.io documentation, "Stages" and "Secret State", consulted for the within-turn response-window pattern (stage move sets replacing the global action space; `playerView` as a pure per-viewer filter; server-authoritative secret-state moves): https://github.com/boardgameio/boardgame.io/blob/main/docs/documentation/stages.md and https://github.com/boardgameio/boardgame.io/blob/main/docs/documentation/secret-state.md
- boardgame.io issue #399 / PR #400 (2019), consulted as the auxiliary-surface leak postmortem (secrets escaping via undo/initial-state history despite a filtered primary view): https://github.com/boardgameio/boardgame.io/issues/399
- Neller & Hnath, "Approximating Optimal Dudo Play with Fixed-Strategy Iteration Counterfactual Regret Minimization" (ACG 2011), consulted for ordered enumerable claim ladders, graded challenge penalties, and the bounded-memory result (conditioning on the last few claims is nearly lossless): http://cs.gettysburg.edu/~tneller/papers/acg2011.pdf
- Reiley, Urbancic & Walker, "Stripped-Down Poker: A Classroom Game with Signaling and Bluffing" (Journal of Economic Education), consulted for the no-pure-equilibrium requirement and the 1/3-bluff / 2/3-call equilibrium used as bot-parameter starting points: http://www.davidreiley.com/papers/Poker.pdf
- Ahle, "snyd" (Liar's Dice equilibrium solver via Koller–Megiddo sequence form), consulted for first-mover-imbalance evidence in strictly increasing claim ladders and per-ruleset balance sensitivity: https://github.com/thomasahle/snyd
- U.S. Copyright Office, "Games" registration circular, and *DaVinci Editrice S.r.l. v. ZiKo Games* (S.D. Tex. 2016), consulted for the rules-vs-expression boundary: mechanics are unprotected; rules text, art, theme, and names must be original: https://www.copyright.gov/register/tx-games.html

`games/masked_claims/docs/SOURCES.md` must record consulted dates, what was used, what was not copied, why the name and labels are original, and asset/font status. The game deliberately has no named roles or abilities, avoiding the unsettled role-roster expression question entirely. If any label feels trademark-forward or trade-dress-adjacent at review time, rename it before implementation.

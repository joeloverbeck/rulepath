# Gate 9.1 Secret Draft Commitment / Reveal

| Field | Value |
|---|---|
| Spec ID | `gate-9-1-secret-draft-commitment-reveal` |
| Roadmap stage | 8 |
| Roadmap build gate | Gate 9.1 — simultaneous commitment / reveal / drafting proof |
| Status | Planned |
| Date | 2026-06-08 |
| Owner | Rulepath maintainers |
| Primary crate / internal game id | `secret_draft` |
| Public display name | `Veiled Draft` unless implementation finds a clearer neutral original name |
| Browser implementation | Required |
| Authority order | `docs/FOUNDATIONS.md` → `docs/ARCHITECTURE.md` → `docs/ENGINE-GAME-DATA-BOUNDARY.md` → `docs/OFFICIAL-GAME-CONTRACT.md` → `docs/MECHANIC-ATLAS.md` → `docs/AI-BOTS.md` → `docs/UI-INTERACTION.md` → `docs/TESTING-REPLAY-BENCHMARKING.md` → `docs/ROADMAP.md` → accepted ADRs that explicitly supersede those documents → this spec |

Where this spec and a foundation document disagree, the foundation document wins. This spec does not verify that commit `65ec79d403e8481b439b1908332c263c73e1d002` is the current `main`; it is authored against that user-supplied target commit as the file baseline.

> Reader orientation: this spec carries the canonical Rulepath section set: Objective, Scope, Deliverables, Work breakdown, Exit criteria, Acceptance evidence, FOUNDATIONS & boundary alignment, Forbidden changes, Documentation updates required, Sequencing, and Assumptions. Detailed proposed rules, state/effect/view sketches, bot policy, WASM/browser wiring, fixtures, traces, benchmarks, and source notes are preserved below the canonical sections under **Implementation reference**.

## Objective

Implement `secret_draft` as the focused Gate 9.1 browser proof for the two ROADMAP §11 exit lines that Gate 9 explicitly deferred: **simultaneous choices remain hidden until reveal** and **UI shows pending seats without leaking choices**.

Gate 9 (`token_bazaar`) completed the public resource/economy half of ROADMAP §11. This gate completes the simultaneous-commitment / synchronized-reveal / drafting half before Gate 10 (`poker_lite` / `plain_tricks`), because later betting, trick, bluffing, and reaction games need a proven waiting-state and no-leak reveal pattern first.

The result is a small, original, deterministic, two-seat browser game, `Veiled Draft`, that proves:

- a shared visible draft pool with removal;
- hidden, binding per-seat commitments;
- pending-seat status without hidden-choice leakage;
- a single synchronized reveal batch after all seats have committed;
- deterministic conflict resolution, scoring, terminal outcome, and tie-breaks;
- Rust-owned legality, previews, effects, replay, visibility projection, and bot decisions;
- browser-safe no-leak behavior across payloads, DOM, `data-testid`, local storage, logs, dev panels, replay exports, and bot explanations.

This is not an engine-generalization gate. It is a small official game that extends existing `high_card_duel` hidden-information machinery while keeping all drafting, commitment, reveal, pool, item, and scoring nouns local to `games/secret_draft`.

## Scope

### In scope

- New official game crate `games/secret_draft` with typed Rust setup, state, actions, validation, application, effects, visibility projection, replay support, variants, UI metadata, and bots.
- Default two-seat variant `secret_draft_standard` / public name `Veiled Draft`.
- Original deterministic rules: six rounds, twelve visible draft tiles, simultaneous hidden pick each round, public deterministic conflict fallback, fixed terminal cap, public deterministic scoring, and public deterministic tie-break ladder.
- Game-local typed nouns only: draft item/tile IDs, visible pool, commitment slots, reveal batch, awards, scoring categories, and conflict fallback. These stay inside `games/secret_draft` and its docs/tests/UI projection.
- Viewer-safe action trees and previews. Before reveal, a seat may know the visible pool and its own legal choices, but no browser payload may contain a submitted hidden choice after commitment until the reveal batch emits it.
- Public pending state: `seat_0_committed: true/false`, `seat_1_committed: true/false`, round number, pool count, and waiting copy; no committed item IDs pre-reveal.
- Grouped reveal effects emitted by Rust and rendered by React as a simultaneous/reveal batch with reduced-motion support.
- Level 0 random legal bot and Level 1 rule-informed `VeiledDraftLevel1Bot`, both using the normal legal-action API and only the bot seat's allowed view.
- Full official-game evidence: unit/rule/property/replay/serialization/visibility/no-leak/bot tests, golden traces, simulations, fixture validation, rule coverage, benchmarks, per-game docs, WASM registration, browser board, E2E smoke, a11y/no-leak checks, and CI/tool registration.
- Documentation updates that add a first-use simultaneous-commitment atlas note, game docs, progress/index updates after implementation, and source notes for consulted prior art.

### Out of scope

- Hosted multiplayer, accounts, matchmaking, server persistence, chat, ranked play, or network-time simultaneity. The proof is local-first and command-log ready.
- A cryptographic commitment scheme. In v1/v2, Rust/WASM is the local authority; the proof is viewer-safe redaction, deterministic replay, and synchronized reveal, not adversarial network secrecy.
- A general drafting engine, card engine, pool engine, commitment engine, reveal engine, waiting-state engine, reaction-window engine, or bot-policy framework.
- `game-stdlib` promotion. The atlas marks simultaneous commitment/reveal as a candidate after repeated pressure; this gate is the first official focused simultaneous-commitment drafting use.
- Any `engine-core` vocabulary or responsibility change.
- Poker, trick-taking, Blackjack, bluffing/reaction windows, auctions, betting, hidden-role claims, or private licensed/commercial designs.
- Ticket decomposition. The Work breakdown lists bounded candidate AGENT-TASKs only; ticket files are a later step.

### Not allowed

Carried from ROADMAP §11 and tightened for this gate:

- Static data formulas for scoring, tie-breaks, conflict resolution, payments, selectors, triggers, or conditions.
- Hidden choices in DOM, `data-testid`, local storage, browser payloads, effect logs, diagnostics, replay exports, dev panels, bot explanations, or candidate rankings before reveal.
- Actual hidden-state sampling by bots.
- MCTS, ISMCTS, Monte Carlo bots, ML, RL, LLM policy, or hidden-state-derived candidate evaluation.
- `engine-core` nouns such as `draft`, `pick`, `commit`, `reveal`, `pool`, `card`, `deck`, `hand`, `tile`, `resource`, `bid`, `reaction`, or `pending response` beyond existing generic actor/viewer/action/effect/replay envelopes.
- Generic `game-stdlib` helpers such as `DraftPool`, `Commitment`, `RevealBatch`, `PendingSeats`, `DraftItem`, `ConflictResolver`, `ScoreFormula`, or bot-policy helpers.
- TypeScript legality, reveal timing, conflict resolution, scoring, terminal detection, tie-breaks, replay authority, or bot policy.
- Proprietary drafting-game names, rules prose, card text, icons, art direction, screenshots, trade dress, or private licensed material.
- Accidental trace/hash/schema migration. Any intentional migration needs explicit notes and accepted review.

## Deliverables

| Area | Required artifacts |
|---|---|
| Workspace and crate | Root `Cargo.toml` registration; `games/secret_draft/Cargo.toml`; source modules `src/actions.rs`, `src/bots.rs`, `src/effects.rs`, `src/ids.rs`, `src/lib.rs`, `src/replay_support.rs`, `src/rules.rs`, `src/setup.rs`, `src/state.rs`, `src/ui.rs`, `src/variants.rs`, `src/visibility.rs`. Mirror the `games/token_bazaar` file-for-file shape unless a file is explicitly documented as not applicable. |
| Static data | `games/secret_draft/data/manifest.toml`, `games/secret_draft/data/variants.toml`, `games/secret_draft/data/fixtures/secret_draft_standard.fixture.json`. Static files contain typed metadata, constants, labels, variant IDs, and fixtures only; they do not contain behavior. Unknown and behavior-looking fields are rejected. |
| Benchmarks | `games/secret_draft/benches/secret_draft.rs`, `games/secret_draft/benches/thresholds.json`. Include legal action generation, validate/apply, reveal resolution, project-view, replay/hash, and Level 1 bot decision operations. Initial thresholds are smoke floors plus a named calibration follow-up under ADR 0002/0003/0005. |
| Native tests | `games/secret_draft/tests/rules.rs`, `property.rs`, `replay.rs`, `serialization.rs`, `visibility.rs`, `bots.rs`. Tests must cover simultaneous pending state, no-leak redaction, reveal batch ordering, conflict fallback, deterministic scoring/tie-breaks, stale diagnostics, invalid unavailable item diagnostics, bot legality, bot determinism, public export/import, and stable hashes. |
| Golden traces | Under `games/secret_draft/tests/golden_traces/`: `shortest-normal.trace.json`, `first-commit-pending.trace.json`, `simultaneous-reveal-batch.trace.json`, `contested-pick-fallback.trace.json`, `terminal-tie-break.trace.json`, `draw-after-tie-breaks.trace.json`, `already-committed-diagnostic.trace.json`, `unavailable-item-diagnostic.trace.json`, `stale-diagnostic.trace.json`, `public-observer-no-leak.trace.json`, `seat-private-no-prereveal-choice.trace.json`, `bot-action.trace.json`, `public-replay-export-import.trace.json`, `wasm-exported.trace.json`. |
| Per-game docs | Eleven docs instantiated from `templates/*`: `games/secret_draft/docs/AI.md`, `BENCHMARKS.md`, `BOT-STRATEGY-EVIDENCE-PACK.md`, `COMPETENT-PLAYER.md`, `GAME-IMPLEMENTATION-ADMISSION.md`, `MECHANICS.md`, `PUBLIC-RELEASE-CHECKLIST.md`, `RULE-COVERAGE.md`, `RULES.md`, `SOURCES.md`, `UI.md`. |
| Tools | Register `secret_draft` in `tools/simulate`, `tools/replay-check`, `tools/fixture-check`, `tools/rule-coverage`, `tools/bench-report` if thresholds/reporting enumerate game IDs, and `tools/seed-reducer` / `tools/trace-viewer` if their dispatch tables need game IDs or effect names. |
| WASM/API | Register `secret_draft` in `crates/wasm-api/src/lib.rs` catalog, setup, action, bot, effect, view, replay/export/import, and no-leak redaction paths. `get_view(match_id, viewer_seat)` must honor viewer scope. Public export defaults to viewer-scoped observation timeline under ADR 0004. |
| Browser | `apps/web/src/components/SecretDraftBoard.tsx`; GamePicker/catalog support through Rust metadata; `ActionControls` support for simultaneous pending state without TS legality; `EffectLog` / `effectFeedback.ts` entries for commit and reveal batch; shell reducer/client type coverage; safe dev panel output; replay import/export wiring; reduced-motion support; responsive, accessible board presentation. |
| Browser smoke | `apps/web/e2e/secret-draft.smoke.mjs` plus any a11y/no-leak checklist updates. Smoke must cover human commit, bot commit, pending-seat UI, synchronized reveal batch, conflict fallback, replay step/export/import, reduced motion, and no hidden item ID in DOM/storage/logs/test IDs before reveal. |
| CI | `.github/workflows/gate-1-game-smoke.yml` native smoke, replay, fixture, rule coverage, web build, and E2E registration; `.github/workflows/gate-2-benchmarks.yml` smoke and threshold registration. |
| Repository docs | `specs/README.md` Gate 9.1 row; `docs/MECHANIC-ATLAS.md` first-use simultaneous-commitment/reveal note and §10B candidate update; `progress.md` and root `README.md` after implementation; no ROADMAP progress edit. |

## Work breakdown

Bounded candidate AGENT-TASKs, in dependency order. Do not decompose these into ticket files as part of this spec.

| # | Candidate task | Depends on | Notes / forbidden drift |
|---:|---|---|---|
| 1 | Crate skeleton and workspace registration | — | Add `games/secret_draft` with IDs, variants, setup constants, manifest/variant parsers, fixture shell. No behavior in data. |
| 2 | State model, typed IDs, deterministic setup | 1 | Model visible pool, drafted collections, commitment slots, round number, priority seat, scores, terminal outcome, freshness token. Use existing deterministic seed discipline only if setup ordering varies; default standard variant may be fully fixed. |
| 3 | Action tree and validation | 2 | Actor-specific legal action tree for any uncommitted non-terminal seat. Validate freshness, actor seat, already-committed conflict, and item availability. Rust owns all preview metadata. |
| 4 | Apply/resolve/reveal effects | 3 | First commit emits pending-only effects; second commit emits one grouped reveal batch, award/removal effects, score effects, round advance/terminal. No pre-reveal item ID in public/browser payloads. |
| 5 | Visibility and replay surfaces | 4 | Public/seat views, effect filtering, internal full trace, viewer-scoped export/import, stable summaries, action/effect/view hashes, no-leak helpers. ADR 0004 rules are mandatory. |
| 6 | Level 0 and Level 1 bots | 3,5 | Bots choose through legal action API from own allowed view only; deterministic tie-breaks; viewer-safe rationale; no hidden-state sampling or opponent-commit access. |
| 7 | Native tests and golden traces | 4,5,6 | Rule/property/replay/serialization/visibility/bot suite and full golden trace set. Follow failing-test protocol; never weaken tests to get green. |
| 8 | Benchmarks and thresholds | 7 | Legal action, validate/apply, reveal resolution, project-view, export/import, bot. Smoke floors first, calibration follow-up named. |
| 9 | Per-game documentation | 7,8 | Instantiate all eleven docs, including rules, sources, AI evidence pack, UI, benchmarks, coverage, admission, and release checklist. |
| 10 | WASM, tools, and CI registration | 7 | Register game ID across wasm-api, simulate, replay-check, fixture-check, rule-coverage, bench-report, seed-reducer/trace-viewer if needed, and CI lanes. |
| 11 | React board and browser no-leak smoke | 10 | Add `SecretDraftBoard.tsx`, effect feedback, pending-seat UI, reveal animation, replay UI, E2E smoke, reduced motion, DOM/storage/test-ID no-leak assertions. TS remains presentation-only. |
| 12 | Repository documentation and final admission evidence | 9,10,11 | Add spec index row, atlas first-use note, progress/root README updates after implementation, command transcript, unresolved issues. Do not edit ROADMAP as progress diary. |

## Exit criteria

Mapped row-for-row to ROADMAP §11. Gate 9 met the resource/economy rows with `token_bazaar`; Gate 9.1 must explicitly claim the deferred simultaneous-choice and pending-seat rows while preserving the full §11 contract.

| ROADMAP §11 line | Gate 9.1 exit criterion |
|---|---|
| resource accounting is effect-visible | **Met for this game’s public drafting/accounting surface.** Every draft award, pool removal, conflict fallback, score delta, round advance, and terminal/tie-break result is emitted as ordered semantic effects and is replay/hash-visible. This gate does not re-open Token Bazaar resource economy. |
| costs/previews come from Rust | **Met.** Rust supplies all legal draft choices, preview copy, scoring deltas visible from the current public state, conflict-risk copy that reveals no hidden choice, disabled/stale diagnostics, and action metadata. TypeScript computes no legality, availability, scoring, or reveal timing. |
| simultaneous choices remain hidden until reveal | **Met.** A submitted commitment is stored only in Rust internal state. Before the reveal batch, public views, seat views, browser payloads, action trees after commitment, previews, diagnostics, effects, DOM, `data-testid`, local storage, dev panels, bot explanations, candidate rankings, and viewer-scoped replay exports contain only pending booleans/counts, never the chosen item ID. |
| UI shows pending seats without leaking choices | **Met.** Browser UI shows `seat_0 committed`, `seat_1 waiting`, round number, visible pool, and safe waiting copy. It renders the synchronized reveal as a grouped effect-driven batch after all seats commit. Reduced motion still preserves event order and no-leak guarantees. |
| bots use allowed views | **Met.** Level 0 and Level 1 bots call the normal legal-action API, validate through Rust, mutate no state directly, and rank only candidates visible to their seat. If the opponent has committed, the bot sees only a pending flag, never the hidden item. |
| invariant/no-leak tests and benchmarks pass | **Met.** Native visibility/no-leak, serialization, replay/export/import, property, bot, benchmark, browser E2E, a11y/no-leak, boundary, and doc-link evidence pass with stable hashes and no hidden-choice leak. |
| Not allowed: static data formulas for payments | **Honored.** Static data may name item IDs, labels, categories, values, variant constants, fixtures, and UI metadata only. Conflict fallback, scoring, terminal detection, and tie-breaks live in typed Rust. Unknown and behavior-looking fields are rejected. |
| Not allowed: hidden choices in DOM/local storage | **Honored.** E2E no-leak tests search DOM text, attributes, `data-testid`s, local/session storage, console/error logs, effect log, action controls, dev panel, replay export, and imported replay text for unrevealed chosen IDs. |
| Not allowed: actual hidden-state sampling by bots | **Honored.** Bot tests construct states with opponent commitments and assert bot decisions/rationales are unchanged when the hidden opponent choice differs but the visible projection is identical. |

## Acceptance evidence

The implementation is accepted only with evidence that covers the full official-game contract.

### Native rules, replay, visibility, and bot evidence

- `cargo test -p secret_draft` passes.
- Rule tests cover setup, legal choices for both uncommitted seats, already-committed diagnostics, unavailable item diagnostics, stale token diagnostics, conflict fallback, terminal cap, scoring, and tie-breaks.
- Property tests cover many deterministic seeds / legal action sequences and assert pool removal count, no duplicate awards, terminal within six rounds, score stability, visibility invariants, and no panics.
- Replay tests prove same seed + seats + options + command stream reproduces state hashes, effect hashes, action-tree hashes, public/seat-view hashes, reveal ordering, and terminal outcome.
- Serialization tests prove stable summaries and unknown-field rejection for manifest, variants, fixtures, viewer-scoped export, and internal trace helpers.
- Visibility/no-leak tests search public views, seat views, action trees after commit, previews, diagnostics, effect payloads, public effect text, command summaries, public export/import timelines, bot explanations, and candidate rankings for unrevealed committed item IDs.
- Bot tests prove Level 0 and Level 1 select only legal action paths, are deterministic under declared inputs, finish many games, and never change decision/rationale based on hidden opponent commitment when the allowed view is unchanged.

### Golden traces

The required golden trace set under `games/secret_draft/tests/golden_traces/` must be committed and checked by `replay-check`:

- `shortest-normal.trace.json`
- `first-commit-pending.trace.json`
- `simultaneous-reveal-batch.trace.json`
- `contested-pick-fallback.trace.json`
- `terminal-tie-break.trace.json`
- `draw-after-tie-breaks.trace.json`
- `already-committed-diagnostic.trace.json`
- `unavailable-item-diagnostic.trace.json`
- `stale-diagnostic.trace.json`
- `public-observer-no-leak.trace.json`
- `seat-private-no-prereveal-choice.trace.json`
- `bot-action.trace.json`
- `public-replay-export-import.trace.json`
- `wasm-exported.trace.json`

### Tools, benchmarks, browser, docs, and CI

- `cargo run -p simulate -- --game secret_draft --games 1000` finishes with no illegal bot action or invariant failure.
- `cargo run -p replay-check -- --game secret_draft --all` passes.
- `cargo run -p fixture-check -- --game secret_draft` passes.
- `cargo run -p rule-coverage -- --game secret_draft` passes and maps every rules-doc obligation to tests/traces.
- `cargo bench -p secret_draft -- legal_actions`, `validate_apply`, `resolve_reveal_batch`, `project_view`, `replay_hash`, and `level1_bot_decision` run. `bench-report` enforces smoke floors where thresholds are calibrated, and the implementation handoff names the calibration follow-up.
- `npm --prefix apps/web run smoke:wasm`, `smoke:ui`, `smoke:preview`, and `smoke:e2e` pass after `secret_draft` is registered.
- `node apps/web/e2e/secret-draft.smoke.mjs` covers human commit, waiting/pending state, bot commit, synchronized reveal, conflict fallback, replay step/export/import, reduced motion, and a11y/no-leak assertions.
- `bash scripts/boundary-check.sh` passes with no new `engine-core` mechanic nouns and no TypeScript legality drift.
- `node scripts/check-doc-links.mjs` passes.
- All eleven per-game docs are present, link-checkable, original, and consistent with implemented behavior.

## FOUNDATIONS & boundary alignment

| Principle | Stance | Rationale |
|---|---|---|
| §2 Behavior authority | Aligned | Rust owns setup, legal choices, validation, commitments, reveal timing, conflict fallback, scoring, terminal detection, semantic effects, replay/export projection, and bot decisions. React presents Rust-provided data. |
| §3 `engine-core` contract kernel | Aligned | Existing generic `Actor`, `Viewer`, `ActionTree`, `CommandEnvelope`, `EffectEnvelope`, `VisibilityScope`, replay/hash, and deterministic RNG contracts are sufficient. `draft`, `pick`, `commit`, `reveal`, `pool`, `tile`, and scoring nouns remain game-local. |
| §4 `game-stdlib` earned | Aligned | No helper is promoted. `secret_draft` is a first focused official simultaneous-commitment drafting proof; atlas notes it as local first-use pressure. If a helper feels unavoidable, implementation must stop and write a primitive-pressure ledger entry before coding the helper. |
| §5 Static data is typed content | Aligned | Manifest/variants/fixtures hold IDs, labels, constants, and metadata only. Conflict resolution, fallback, scoring, legality, and tie-breaks are typed Rust. Unknown and behavior-looking fields are rejected. |
| §8 Public bots | Aligned | Bots use the same legal action API as humans, validate normally, mutate no state directly, expose viewer-safe explanations, and use no hidden-state sampling, MCTS/ISMCTS, Monte Carlo, ML, RL, or LLM policy. |
| §11 Universal invariants | Aligned | Deterministic replay/hash/serialization, viewer-safe public/private views, hidden-info no-leak before reveal, legal-only UI, semantic-effect animation, local-first scope, source notes, docs, simulations, benchmarks, and tests are explicit acceptance evidence. |
| §12 Stop conditions | Clear | Stop if a kernel noun appears, static data becomes procedural, TypeScript decides legality/reveal/scoring, hidden choice reaches any browser/dev/replay/bot surface before reveal, bot uses hidden opponent state, or a helper is promoted without atlas process. |
| §13 ADR triggers | **No ADR expected.** | The simultaneous reveal model is already inside the architectural envelope: `high_card_duel` proves hidden commitments/private views/redaction, and `ARCHITECTURE.md` already lists commitments/reveals, pending responses, grouped batches, and simultaneous/reveal batches as effect shapes. If implementation genuinely requires changing replay/hash semantics, visibility contracts, kernel vocabulary, new data formats, renderer defaults, or bot classes, stop and write an ADR before proceeding. |
| ADR 0004 hidden-info replay/export | Aligned | Internal full traces remain native test authority. Browser export defaults to viewer-scoped observation timeline and must not contain pre-reveal commitments, raw private action paths, seed material that reconstructs hidden choices, bot private candidates, or hidden-state-derived explanations. |
| Benchmark ADRs 0002/0003/0005 | Aligned | Add smoke benchmarks and threshold files now; calibrate stable floors after enough runs, with variance-aware treatment and non-gating PR smoke as prescribed. |
| ADR 0006 Blackjack placement | Aligned | This spec does not resurrect Blackjack. Blackjack remains deferred; Gate 9.1 is a prerequisite-style commitment/reveal proof before Gate 10 comparison cases. |

## Forbidden changes

Do not, in this gate:

- Add `draft`, `pick`, `commit`, `reveal`, `pool`, `card`, `deck`, `hand`, `tile`, `resource`, `bet`, `pot`, `trick`, `claim`, `reaction`, or similar mechanic/domain vocabulary to `engine-core`.
- Add generic `game-stdlib` primitives for draft pools, commitments, reveal batches, pending seats, conflict resolution, score formulas, hidden-choice bots, or UI waiting-state policy.
- Introduce behavior-in-data, YAML, DSLs, formulas, selectors, triggers, conditions, rule scripts, procedural data, hidden defaults, or untyped nested behavior objects.
- Let TypeScript compute legality, availability, previews, reveal timing, conflict fallback, scoring, terminal outcome, tie-breaks, replay authority, bot policy, hidden-info filtering, or no-leak filtering.
- Leak committed item IDs before reveal through public/seat views, action trees after commit, previews, diagnostics, semantic effects, effect-log text, command summaries, DOM text, attributes, `data-testid`, local/session storage, browser logs, dev panels, replay exports, bot rationales, candidate rankings, screenshots, fixtures, or E2E anchors.
- Use MCTS, ISMCTS, Monte Carlo, ML, RL, LLMs, hidden-state sampling, opponent-commit peeking, omniscient heuristics, or giant opaque weight tables.
- Copy proprietary drafting-game rules, names, card text, icons, art, screenshots, trade dress, or private licensed material.
- Add hosted multiplayer, accounts, server authority, matchmaking, chat, ranked play, persistence, or cryptographic commitments.
- Change trace schema, replay/hash semantics, data versions, action path stability, effect ordering, public-view shape, or golden traces accidentally.
- Delete, rename away, weaken, or rewrite tests merely to get green output. Follow the failing-test protocol: validate test intent, locate SUT vs test issue, fix, add regression coverage, report.

## Documentation updates required

- Add a `specs/README.md` row after Gate 9 and before Gate 10:
  - Stage `8`, Gate `Gate 9.1`, Spec `gate-9-1-secret-draft-commitment-reveal.md`, status `Planned` when this spec is accepted, then `Done` only after evidence passes.
- Do **not** edit `docs/ROADMAP.md` to record progress. ROADMAP remains ladder law; `specs/README.md` tracks status.
- Update `docs/MECHANIC-ATLAS.md`:
  - Add a first-use local-only note for `simultaneous commitment/reveal + visible draft-pool removal` with `secret_draft` as the official use.
  - Keep §10B `simultaneous commitment/reveal` as a candidate after second use; do not promote.
  - Confirm §10A open promotion-debt register remains `_None_` after implementation.
- Update `progress.md` and root `README.md` after implementation, not before evidence passes.
- Author all eleven `games/secret_draft/docs/*` from templates and keep them consistent with implemented behavior.
- Update `docs/SOURCES.md` only if maintainers decide the repo-level game-specific source-note table should enumerate `secret_draft`; in all cases, add `games/secret_draft/docs/SOURCES.md` with consulted prior art and original-content notes.
- Update `apps/web/README.md` only after the browser board and smoke are actually registered.
- Ensure `scripts/check-doc-links.mjs` passes after doc changes.

## Sequencing

- **Predecessor:** Gate 9 (`token_bazaar`) is Done in the exact-commit spec index and explicitly deferred `secret_draft` to a Gate 9.1 commitment/reveal gate.
- **Admission rule:** Before implementation starts, confirm `docs/MECHANIC-ATLAS.md` §10A still has no open promotion debt. At the target commit, it records no open debt.
- **This gate:** Gate 9.1 fills the deferred ROADMAP §11 simultaneous-choice / pending-seat / synchronized-reveal proof.
- **Successor:** Gate 10 (`poker_lite` / `plain_tricks`) remains not yet specced. It should not start until Gate 9.1 evidence proves no-leak waiting/reveal behavior and no open promotion debt remains.
- **Blackjack:** Still deferred by ADR 0006; not a Gate 9.1 target or shortcut.

## Assumptions

- A1: Two seats remain the default and sufficient proof shape; adding N>2 would add UI/replay/bot complexity without improving the Gate 9.1 proof.
- A2: Public display name `Veiled Draft` is an original placeholder; maintainers may rename it to another neutral original name before implementation.
- A3: Standard variant uses twelve visible draft tiles and six rounds; maintainers may tune values while preserving fixed cap, deterministic scoring, and proof scope.
- A4: The standard setup can be fully fixed; if seeded setup ordering is added, it must use existing `SeededRng` contracts and preserve replay determinism.
- A5: Conflict fallback uses stable item order after the contested item is removed; maintainers may choose a different deterministic public fallback only if tests/traces/docs are updated before implementation.
- A6: A submitted commitment item ID should not be echoed back to any browser payload before reveal, even to the committing seat; this stricter posture is intentional for the no-leak proof.
- A7: Internal native traces may contain raw private action paths under ADR 0004; browser exports must not.
- A8: No `game-stdlib` helper is needed. If implementation pressure contradicts this, stop and write an atlas/ADR decision before extracting.
- A9: The existing WASM shell can support “both seats may commit until each has one commitment” through actor-specific legal action trees and safe pending views without kernel changes.
- A10: If any assumption is wrong, correct the smallest local game/spec surface; do not generalize the engine.

---

# Implementation reference

## Product intent / what this gate proves

`Veiled Draft` is a deliberately compact original game. It is not trying to be a deep commercial draft. It is trying to make the architecture prove one hard thing cleanly: two seats can choose from the same visible public options, bind their choices privately, wait while the other seat commits, reveal together, resolve deterministically, and replay/export safely without leaking the hidden choice before reveal.

The design combines two widely used tabletop structures as abstract prior art, not copied content: simultaneous action selection, where players choose secretly and reveal together, and open drafting, where players select from a common pool. BoardGameGeek describes simultaneous action selection as secret simultaneous planning followed by reveal, and open drafting as choosing from a common pool of cards, tiles, resources, dice, or similar items. OpenSpiel research is also useful because it treats simultaneous-move games as a game class that can be represented with imperfect information, matching Rulepath’s need to model hidden committed choices before reveal.

## Proposed original rules: `Veiled Draft`

### Components

- Two seats: `seat_0`, `seat_1`.
- Twelve visible draft tiles in stable order. Each tile has:
  - `item_id`: stable original ID, for example `ember_1`, `ember_2`, `tide_1`, `tide_2`, `grove_1`, `grove_2`, `ember_3`, `tide_3`, `grove_3`, `ember_4`, `tide_4`, `grove_4`.
  - `thread`: one of `ember`, `tide`, `grove`.
  - `value`: `1`, `2`, `3`, or `4`.
  - Original display label such as `Ember One`, `Tide Three`, `Grove Four`. These labels are placeholders and must be IP-reviewed as original prose.
- Public round marker `1..=6`.
- Public priority seat, alternating by round: `seat_0` on odd rounds, `seat_1` on even rounds.
- Per-seat public drafted collection, initially empty.
- Per-seat hidden commitment slot, initially empty.
- Public score summary after each reveal.

### Setup

1. Validate exactly two seats.
2. Load `secret_draft_standard` constants from typed Rust/validated static metadata.
3. Place all twelve standard tiles in the visible pool in stable order.
4. Set round `1`, priority `seat_0`, both commitments empty, both drafted collections empty, both scores `0`, terminal `None`, freshness token `0`.
5. Emit setup/view state through Rust projection only.

### Round flow

Each of the six rounds has the same shape.

1. **Commit phase.** Each seat that has not yet committed receives a Rust legal action tree containing the currently visible pool items. Both seats are eligible until they commit. A seat that has already committed receives no commit choices and sees safe waiting copy.
2. **First commitment.** When the first seat submits a valid item choice, Rust stores the item ID internally, emits `CommitmentPlaced { seat, round }`, increments freshness, and projects only a pending flag. No browser payload echoes the item ID.
3. **Waiting state.** UI shows which seats are pending/committed, the visible pool, and safe copy such as “Seat 0 has committed; waiting for Seat 1.” It does not show, store, log, or encode the committed item.
4. **Second commitment.** When the second seat commits, Rust stores the second item internally, then immediately resolves the reveal batch.
5. **Synchronized reveal batch.** Rust emits a grouped reveal sequence:
   - `RevealBatchStarted { round }`
   - `ChoicesRevealed { round, seat_0_item, seat_1_item }`
   - `DraftResolved { round, awards, removed_items, conflict }`
   - `ScoreChanged { round, score_0, score_1, tie_break_summary }`
   - `RoundAdvanced` or `Terminal`.
6. **Cleanup.** Commitments are cleared, awarded items are removed from the visible pool, next round starts unless round `6` has resolved.

### Conflict resolution and drafting with removal

- If the two seats chose different available items, each seat receives its chosen item. Both chosen items leave the pool.
- If both seats chose the same item:
  1. The public priority seat for that round receives the contested item.
  2. The other seat receives the lowest stable-order remaining item after the contested item is removed.
  3. Both awarded items leave the pool.
  4. The public effect records the contested item, priority winner, fallback item, and both awards after reveal.
- This deterministic fallback avoids adding a post-reveal reaction window while still proving simultaneous choice from a shared visible draft pool and removal.

### Scoring

Scoring is public and deterministic after every reveal. Suggested standard scoring:

- Base points: sum of drafted tile values.
- Thread set bonus: `+3` for each complete set of one `ember`, one `tide`, and one `grove` among drafted tiles. A tile can contribute to at most one set bonus.
- High-thread bonus: `+2` if a seat has at least three tiles in the same thread by game end. Award once per thread.
- Conflict discipline bonus: `+1` at terminal to each seat that received at least one fallback item and still completed a thread set. This turns fallback from pure punishment into a visible strategic compensation without requiring hidden logic.

All scoring functions live in Rust. Static data may list item IDs, thread labels, and values, but not formulas.

### Terminal and tie-breaks

The game ends immediately after round 6 resolves. Terminal winner uses this public ladder:

1. Higher total score wins.
2. If tied, more complete thread sets wins.
3. If tied, higher single drafted tile value wins.
4. If tied, more distinct threads represented wins.
5. If tied, fewer priority-won contested items wins. This slightly rewards needing less priority help and is fully public after reveal.
6. If still tied, terminal outcome is `Draw`.

The final `Terminal` effect carries the winner or draw and a public tie-break summary. There is no hidden terminal reveal of unchosen pool items beyond the already-visible pool.

## State, actions, and validation sketch

### State

Game-local state should include:

- `variant: Variant`
- `seats: [SeatId; 2]`
- `round_number: u8`
- `phase: Phase` where `Phase::Commit`, `Phase::Terminal` are enough unless implementation wants a transient internal `Resolving` marker for tests.
- `visible_pool: Vec<DraftItemId>` in stable order.
- `drafted: [Vec<DraftItemId>; 2]`
- `commitments: [Option<DraftItemId>; 2]` internal only.
- `fallback_awards: [u8; 2]`
- `priority_conflict_wins: [u8; 2]`
- `scores: [u32; 2]`
- `revealed_history: Vec<RevealedRound>`
- `terminal_outcome: Option<TerminalOutcome>`
- `freshness_token: FreshnessToken`

### Legal action tree

For an actor mapped to a seat:

- If terminal: flat empty tree.
- If actor already committed this round: flat empty tree with safe pending/waiting metadata.
- Otherwise: flat choices for every item currently in `visible_pool`.

Choice metadata may include public item ID, value, thread, label, current public score preview, and safe warning that another committed seat may already be pending. It must not include opponent hidden choices, hidden predictions, hidden bot candidates, or TypeScript-computed consequences.

### Validation

Validation rejects:

- stale freshness token;
- actor not seated;
- terminal phase;
- actor already committed;
- malformed action segment;
- item not currently visible in pool;
- action path with extra segments.

Validation returns a game-local `ValidatedAction { actor, item }`. Diagnostics must be viewer-safe. For example, use “item is unavailable for this commitment” rather than “opponent already chose that item.”

## Semantic effect model

| Effect | Visibility before reveal | Payload rule |
|---|---|---|
| `CommitmentPlaced { seat, round }` | Public | Seat and round only. No item ID. |
| `OwnCommitAccepted { seat, round }` | Private or omitted | If emitted, still no item ID before reveal. Its purpose is confirmation, not memory. |
| `PendingSeatsChanged { round, seat_0_committed, seat_1_committed }` | Public | Booleans only. |
| `RevealBatchStarted { round, group_id }` | Public | Batch marker only. |
| `ChoicesRevealed { round, seat_0_item, seat_1_item }` | Public after both commits | First public appearance of committed item IDs. |
| `DraftResolved { round, awards, removed_items, conflict }` | Public after reveal | Awards/removals/tie resolution. |
| `PoolChanged { remaining_items }` | Public | Remaining pool was already visible; stable order. |
| `ScoreChanged { scores, tie_break_summary }` | Public | Public score/tie-break data only. |
| `RoundAdvanced { next_round, priority_seat }` | Public | Next public decision state. |
| `Terminal { outcome, final_scores, tie_break_summary }` | Public | No auto-reveal of unavailable private data; unpicked pool is visible by definition. |
| `PrivateDiagnostic` | Private to seat | No hidden opponent data. |
| `PublicDiagnostic` | Public | Safe stale/invalid/public-state reasons only. |

React animates from these effects and settles to Rust projection. No renderer diff becomes authoritative causality.

## Visibility and no-leak model

### Public observer view

May contain:

- game ID/version/variant;
- round number, phase, priority seat;
- visible pool with item IDs/labels/values/threads;
- drafted public collections;
- pending booleans per seat;
- public scores and tie-break summary;
- revealed history;
- terminal outcome.

Must not contain before reveal:

- `commitments` values;
- raw submitted action paths for committed seats;
- own/other hidden item in command summaries;
- bot candidate rankings or explanation facts derived from hidden choices;
- seed material that reconstructs hidden commands in browser export.

### Seat-private view

For this stricter proof, pre-reveal seat-private browser view should also avoid echoing the submitted item ID. The acting human saw the choice during selection; after commit, the UI deliberately shows only “You have committed.” This makes DOM/storage/log no-leak tests cleaner and proves the waiting state is safe even in hotseat.

### Internal full trace vs browser export

- Internal native traces may include full command stream, private action path, hidden commitments, state hashes, and seed evidence needed for deterministic replay checks.
- Browser export must default to viewer-scoped observation timeline. Before `ChoicesRevealed`, export contains only pending booleans/effects with item IDs redacted. After reveal, item IDs are public and may appear.
- Public export/import replays observation timeline, not an omniscient internal state.

## Bot policy

### Level 0

`SecretDraftRandomBot` chooses uniformly from the Rust legal action tree using deterministic bot RNG helpers. It never constructs actions outside the legal tree and never sees hidden opponent commitments.

### Level 1

`VeiledDraftLevel1Bot` ranks visible legal items by public marginal value for its own drafted collection:

1. completing a thread set;
2. higher immediate value;
3. improving high-thread terminal bonus;
4. reducing vulnerability to deterministic fallback in a conflict, based only on public priority seat and visible pool, never opponent hidden choice;
5. stable item ID tie-break;
6. deterministic bot seed tie-break if the implementation already supports such tie-breaks safely.

Explanation examples:

- Safe: “Chose Tide Three because it completes a thread set and adds 3 base points.”
- Safe: “Chose Ember Four because it is the highest visible value and supports a high-thread bonus.”
- Forbidden: “Chose Grove Two because the opponent committed to Ember Four.”
- Forbidden: “Opponent probably picked Tide Three according to hidden candidate sampling.”

No Level 2 bot is required unless maintainers choose to treat the Level 1 policy as the default authored policy and complete the strategy evidence pack. If Level 2 is claimed, `BOT-STRATEGY-EVIDENCE-PACK.md` must include win-rate/simulation evidence, rationale examples, latency, and no-leak tests.

## WASM/browser wiring

- Add catalog entry with `game_id: secret_draft`, display name `Veiled Draft`, hidden-information flag, viewer modes, variants, and docs links.
- `new_match` creates a `SecretDraft` match record.
- `get_view(match_id, viewer_seat)` returns viewer-scoped projection. For observer/public view, pre-reveal choices are redacted. For seat view, pre-reveal submitted choice is also redacted by spec assumption A6.
- `get_action_tree(match_id, actor_seat)` returns choices only for uncommitted seats. If a seat has committed, it returns no choices and safe pending metadata.
- `apply_action` validates/commits and returns safe effects + view. First commit result contains no item ID. Second commit result contains reveal batch.
- `run_bot_turn` routes through legal tree and validation; bot decision JSON must be viewer-safe.
- `export_replay` defaults to viewer-scoped observation export. Internal full traces remain native tooling evidence.
- `SecretDraftBoard.tsx` renders visible pool, drafted collections, pending seats, priority marker, scores, reveal history, and safe action affordances. It never renders hidden committed IDs before reveal.
- `ActionControls` must not use `choice-${submitted_hidden_id}` after commitment as a persistent DOM/test anchor. Legal choices can use visible item IDs while they are merely public pool choices; after submit, pending UI uses seat/round anchors only.
- `EffectLog` / `effectFeedback.ts` must treat reveal as a grouped batch and support reduced motion.
- Dev panel must receive only the same viewer-safe projection and safe command/effect summaries.

## Fixture and golden trace guidance

Standard fixture should represent initial state with stable IDs and no commitments. Golden traces should be small and intentionally named so failures explain the invariant being broken. The most important traces are:

- **first-commit-pending**: one seat commits; exported public observer timeline contains pending boolean but not chosen item.
- **simultaneous-reveal-batch**: both seats commit different items; reveal batch emits both IDs together in stable order.
- **contested-pick-fallback**: both choose same item; priority/fallback deterministic resolution removes exactly two items.
- **seat-private-no-prereveal-choice**: committing seat’s browser-safe view confirms commitment without echoing item ID.
- **public-replay-export-import**: public export hides pre-reveal choice and imports to the same observation timeline.

## Benchmark operations

Initial benchmark identities should include at least:

- `legal_actions_initial_pool`
- `legal_actions_after_one_commit`
- `validate_commit`
- `apply_first_commit`
- `apply_second_commit_resolve_reveal`
- `project_public_view_pending`
- `project_public_view_after_reveal`
- `state_hash_terminal`
- `public_export_timeline`
- `level1_bot_decision`

Thresholds should start as non-heroic smoke floors with a calibration note. Do not optimize until benchmark evidence points to a real issue.

## Source notes and originality guidance

Consulted external sources shape only the proof vocabulary, not the original rules, labels, prose, UI, or assets:

- BoardGameGeek, “Simultaneous Action Selection,” consulted for the generic mechanic concept of secret simultaneous planning followed by reveal: https://boardgamegeek.com/boardgamemechanic/2020/simultaneous-action-selection
- BoardGameGeek, “Open Drafting,” consulted for the generic mechanic concept of selecting from a common pool: https://boardgamegeek.com/boardgamemechanic/2041/open-drafting
- Lanctot et al., “OpenSpiel: A Framework for Reinforcement Learning in Games,” consulted for simultaneous-move / imperfect-information modeling context, not for Rulepath bot techniques: https://arxiv.org/abs/1908.09453
- OpenSpiel documentation/repository, consulted as a research-system comparison that supports simultaneous-move and imperfect-information games, not as a UI or bot-policy model: https://openspiel.readthedocs.io/ and https://github.com/google-deepmind/open_spiel
- GamesRadar, “Types of board games, explained by experts,” consulted only for public-facing open-vs-closed draft vocabulary; no game rules or trade dress are copied: https://www.gamesradar.com/tabletop-gaming/types-of-board-games/

`games/secret_draft/docs/SOURCES.md` must record consulted date, what was used, what was not copied, why the name and labels are original, and asset/font status. If any label feels trademark-forward or trade-dress-adjacent, rename it before implementation.

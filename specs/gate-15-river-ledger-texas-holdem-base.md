# Gate 15 — River Ledger / Texas Hold'Em base

## 1. Header

| Field | Value |
| --- | --- |
| Spec ID | `GAT15-RIVLED-THEBASE-001` |
| File | `specs/gate-15-river-ledger-texas-holdem-base.md` |
| Roadmap stage | Stage 15 / Public scaling phase |
| Roadmap build gate | Gate 15 |
| Status | `Planned` |
| Date | 2026-06-14 |
| Owner | Rulepath maintainers |
| Authority order | `docs/FOUNDATIONS.md` → `docs/ARCHITECTURE.md` → `docs/ENGINE-GAME-DATA-BOUNDARY.md` → area contracts → accepted ADRs where they explicitly supersede prior sections → roadmap/specs/tickets. |

This document is a requirements and implementation-planning spec. It does not edit repository files, create ticket files, or decompose the work beyond bounded candidate `AGENT-TASK` packets.

The product-facing game name is **River Ledger**. The rules-family label is **Texas Hold'Em rules family**. The implementation is a new coexisting official game crate, `games/river_ledger`, not a rename, fork, or replacement of `games/poker_lite`.

## 2. Objective

Gate 15 admits **River Ledger / Texas Hold'Em base** as Rulepath's first official 3–6-seat hidden-information betting game. It turns the Public Scaling Phase ladder entry into an executable plan that an agent team can reassess and then decompose into tickets.

Gate 15 must prove that Rulepath can support a real N-seat imperfect-information betting game while preserving the constitution:

- Rust owns all game behavior: setup, shuffle/deal, blinds/button rotation, legal actions, validation, betting transitions, street advancement, contribution accounting, hand evaluation, showdown explanation, outcome breakdowns, visibility filtering, trace/replay serialization, simulation, and bot decisions.
- TypeScript/React presents only viewer-safe Rust/WASM payloads. It never decides legality, derives private facts, evaluates hands, computes betting obligations, or repairs missing Rust state.
- `engine-core` remains noun-free. Card, deck, hand, street, pot, blind, button, evaluator, and contribution nouns are game-local to `games/river_ledger` unless a future mechanic-atlas promotion is separately authorized.
- Trace schema v1 is reused. The existing `seats` array is sufficient for N-seat River Ledger; the gate tightens semantics around per-seat view hashes and public-observer exports, but it does not migrate the schema.
- N-player no-leak proof is first-class: every pair of distinct seats in 3-, 4-, 5-, and 6-seat matches must be tested across Rust projections, effects, action diagnostics, replay exports, bot explanations, browser DOM, storage, logs, and smoke traces.

### Gate determination

This gate is locked as the next unit. The exact-target tree's living index shows Phase 0 foundation realignment as `Done`, Infra A-D N-seat public infrastructure as `Done`, and Gate 15 as the lowest non-`Done` public-scaling unit. The mechanic-atlas open promotion-debt register is `_None_`, so no primitive promotion, back-port, or debt retirement interposes before this gate. This spec confirms that determination; it does not reopen the “what should be built next” question.

## 3. Scope

### 3.1 In scope

| Area | Committed Gate 15 scope |
| --- | --- |
| Seat range | Official River Ledger supports exactly **3–6 seats**. `setup(seed, seats, setup)` accepts seat counts 3, 4, 5, and 6, and rejects counts outside that range with deterministic diagnostics. Heads-up is not the primary official path for this game. |
| Seat order and roles | Seats retain stable `SeatId` order. Button, small blind, and big blind are derived deterministically from that order. The button rotates in later hands/fixtures if multi-hand fixtures are added; the base single-hand fixture records the selected button. |
| Deck and deal | Standard 52-card deck. Deterministic shuffle from engine RNG. Two private hole cards per seat. Five public community cards. Burn cards may be modeled only as internal deterministic deck advancement and must never appear in unauthorized views, DOM, logs, effect payloads, bot explanations, public replay exports, or action diagnostics. |
| Street structure | Preflop → flop → turn → river → showdown. Community reveal pattern is three-card flop, one-card turn, one-card river. |
| Betting model | **Fixed-limit, capped-raise**. Abstract contribution units, never casino-chip or currency language. Small bet on preflop/flop; big bet on turn/river. Legal actions are `fold`, `check`, `call`, `bet`, and `raise`, plus Rust-owned street advancement when a betting round closes. |
| Raise cap | Rulepath v1 cap is **one opening bet plus three raises per street**. The cap is explicit in rules, legal-action generation, diagnostics, traces, bots, and benchmarks to bound action trees. |
| Blinds | Button/SB/BB are assigned in seat order. The small blind and big blind are forced contributions before preflop action. Contribution units are abstract public counters. |
| Contribution capacity | v1 uses a high enough deterministic contribution capacity that legal play cannot require all-in handling. All-in and side-pot mechanics are deliberately absent from the base model. |
| Split pots | **In scope.** Tied best hands among showdown-eligible seats split the final pot. Integer-unit remainders are allocated deterministically by stable button-order among tied winners and explained in the Rust-authored outcome rationale. |
| Fold-out terminal | If all but one live seat folds, the last live hand wins immediately. The outcome explanation uses a distinct “last live hand” rationale and does not reveal folded seats' private hole cards unless a separate future rule explicitly authorizes that reveal. |
| Hand evaluator | Straightforward deterministic five-card evaluator. Seven-card best hand is found by enumerating the 21 five-card subsets of the two private cards plus five community cards, returning `(category, ordered_tie_break_vector, exact_used_cards)`. Correctness, auditability, and replayability beat throughput. No lookup-table evaluator in Gate 15. Texas Hold'Em uses the best five-card poker hand from a player's two hole cards and five community cards; a seven-card choice therefore has `C(7,5)=21` candidate five-card hands.[^pagat-holdem][^pagat-ranking] |
| Showdown explanation | Mandatory, Rust-authored, and viewer-aware. Per seat: folded-before-showdown vs reached-showdown; private-card reveal only where the viewer is authorized; best five cards from seven; hand category; ordered tie-break vector; decisive comparison; pot allocation; split/tie/remainder rule; and final ledger totals. |
| Bots | Level 0, Level 1, and Level 2 only. L0 chooses randomly from legal actions in its own authorized view. L1 is conservative and uses only own hole strength, public board texture, call price, live-opponent count, street, and cap pressure. L2 is a limited opponent-count-aware heuristic. No MCTS, ISMCTS, Monte Carlo, ML, RL, external solvers, lookup strategy tables, hidden-state sampling, or omniscient candidate rankings. |
| UI | Public, neutral, original Rulepath presentation. Seat order, button/SB/BB, active/pending seats, public board, abstract contribution ledger, legal-action controls, safe previews, final breakdown, and viewer-safe explanations are rendered from Rust/WASM payloads. No casino trade dress, tournament branding, cash/chip/rake language, or copied prose/assets. |
| Registration | `river_ledger` is registered through the existing workspace, WASM catalog, simulator, replay/fixture/rule-coverage tools, CI game manifest, web catalog, rules manifest, and smoke surfaces. Infra A-D seams are consumed, not rebuilt. |
| Documentation | The new game fills the official-game document set from templates: rules, how-to-play, sources, rule coverage, mechanics, AI, UI, benchmarks, competent-player analysis, Level 2 evidence pack, primitive-pressure ledger, implementation admission, and public-release checklist. |
| Mechanic atlas | Card/deck/hand/evaluator/accounting/pot behavior is game-local. River Ledger records atlas pressure against `high_card_duel`, `poker_lite`, `plain_tricks`, `masked_claims`, and existing accounting entries before any reuse or promotion. No `game-stdlib` promotion is authorized by this spec. |
| Source posture | Texas Hold'Em rules and hand-ranking references are used only to verify public-domain rules facts. Rulepath prose, naming, presentation, and assets remain original. External sources describe the rules family, not the product identity.[^pagat-holdem][^pagat-ranking][^fournier-holdem] |
| Prior-art posture | OpenSpiel and boardgame.io are useful only as vocabulary/prior art for N-player imperfect-information representation, player-specific views, logs, phases, and view-layer separation; Rulepath does not import their architecture, algorithms, or AI methods.[^openspiel][^boardgameio] |

### 3.2 Out of scope

| Area | Gate 15 stance |
| --- | --- |
| All-in | Out of scope. The base game must not create states that require all-in handling. |
| Side pots | Out of scope and deferred to Gate 15.1. No side-pot data model is designed into Gate 15. |
| No-limit / pot-limit | Out of scope. Fixed-limit only. |
| Tournament structure | Out of scope. No blinds escalation, rebuys, eliminations, payouts, rake, table balancing, lobby, or ranking ladder. |
| Real money / gambling product features | Out of scope and forbidden. Contributions are abstract rule units only. |
| Heads-up official mode | Out of scope as an official public path. A later variant may consider it only after this gate completes. |
| Networked multiplayer/accounts | Out of scope. The project remains static/local-first. |
| Lookup-table evaluator optimization | Out of scope. Gate 15 values auditability and explanation over evaluator speed. |
| New trace schema | Out of scope unless an accepted ADR proves a real expressiveness gap. Base River Ledger is not expected to need one. |
| Primitive promotion | Out of scope. Any future helper promotion must pass the mechanic-atlas gate separately. |
| Ticket files | Out of scope. This spec stops at candidate `AGENT-TASK` granularity. |

### 3.3 ROADMAP §15 “Not allowed” list

Gate 15 carries the ROADMAP §15 prohibition verbatim:

> Real-money/casino features, tournament/product mimicry, hidden card/deck leakage, omniscient bots, public MCTS/ISMCTS/Monte Carlo/ML/RL, or side-pot/all-in scope unless explicitly admitted by Gate 15.1.

## 4. Deliverables

### 4.1 New game crate

The implementation gate creates a new crate that follows the existing official-game shape while remaining independent of `poker_lite`.

```text
games/river_ledger/
  Cargo.toml
  benches/
    river_ledger.rs
    thresholds.json
  data/
    manifest.toml
    variants.toml
    fixtures/
      river_ledger_3p_standard.fixture.json
      river_ledger_4p_standard.fixture.json
      river_ledger_5p_standard.fixture.json
      river_ledger_6p_standard.fixture.json
      river_ledger_showdown_split.fixture.json
      river_ledger_foldout.fixture.json
      river_ledger_raise_cap.fixture.json
  docs/
    RULES.md
    HOW-TO-PLAY.md
    SOURCES.md
    RULE-COVERAGE.md
    MECHANICS.md
    AI.md
    UI.md
    BENCHMARKS.md
    COMPETENT-PLAYER.md
    BOT-STRATEGY-EVIDENCE-PACK.md
    PRIMITIVE-PRESSURE-LEDGER.md
    GAME-IMPLEMENTATION-ADMISSION.md
    PUBLIC-RELEASE-CHECKLIST.md
  src/
    lib.rs
    ids.rs
    cards.rs
    state.rs
    setup.rs
    actions.rs
    betting.rs
    evaluator.rs
    showdown.rs
    pot.rs
    rules.rs
    effects.rs
    visibility.rs
    variants.rs
    bots.rs
    ui.rs
    replay_support.rs
  tests/
    rules.rs
    property.rs
    replay.rs
    serialization.rs
    visibility.rs
    bots.rs
    golden_traces/
      setup-3p.trace.json
      setup-4p.trace.json
      setup-5p.trace.json
      setup-6p.trace.json
      invalid-seat-count.trace.json
      preflop-blinds-call-check-advance.trace.json
      flop-small-bet-cap.trace.json
      turn-river-big-bet.trace.json
      raise-cap-diagnostic.trace.json
      foldout-last-live-hand.trace.json
      high-card-showdown.trace.json
      pair-beats-high-card.trace.json
      straight-ace-low.trace.json
      flush-kicker-order.trace.json
      full-house-tiebreak.trace.json
      split-pot-even.trace.json
      split-pot-remainder-button-order.trace.json
      deal-private-no-leak.trace.json
      public-observer-no-leak.trace.json
      seat-private-view.trace.json
      wrong-seat-diagnostic.trace.json
      public-replay-export-import.trace.json
      bot-vs-bot-full-game-6p.trace.json
      wasm-exported.trace.json
```

Module responsibilities:

| Module | Responsibility |
| --- | --- |
| `ids.rs` | `RiverLedgerSeat`, `STANDARD_MIN_SEATS = 3`, `STANDARD_MAX_SEATS = 6`, rule ID prefix constants, actor/viewer conversion helpers. |
| `cards.rs` | Game-local `Rank`, `Suit`, `Card`, `Deck`, deterministic deck construction, public card rendering labels. No export of burn/deck-tail identities to unauthorized projections. |
| `state.rs` | `Phase`, `Street`, `SeatStatus`, `SeatLedger`, `BettingRoundState`, `ContributionLedger`, `TerminalOutcome`, `ShowdownReveal`, `ShowdownSeatExplanation`, public/private state records. |
| `setup.rs` | Seat-range validation, deterministic shuffle/deal, blind/button assignment, forced contributions, initial preflop active seat, fixture setup options. |
| `actions.rs` | Rust legal-action tree and command validation for `fold`, `check`, `call`, `bet`, `raise`. Contains no UI-only legality. |
| `betting.rs` | Fixed-limit unit selection, cap tracking, call price, contribution matching, round closure, street advancement. |
| `evaluator.rs` | Five-card evaluator, seven-card 21-subset search, comparable category/tie-break tuple, exact used cards. |
| `showdown.rs` | Showdown-eligible seat discovery, winner comparison, split pot/remainder allocation, Rust-authored rationale. |
| `pot.rs` | Single-pot contribution allocation only. Explicitly rejects side-pot/all-in design. |
| `rules.rs` | `apply_action`, state transitions, terminal checks, diagnostics, effect production, rule-ID hooks. |
| `effects.rs` | Semantic public/private effects and filtering scopes. |
| `visibility.rs` | `project_view`, `PublicView`, `PrivateView`, observer projection, seat-private projection, per-seat view hashes, effect redaction. |
| `variants.rs` | Typed setup/static-data loader. Static data may describe variants/metadata only; no selectors, conditions, triggers, formulas, or behavior. |
| `bots.rs` | L0/L1/L2 policies from authorized views and legal actions only. |
| `ui.rs` | Rust-authored UI metadata: labels, seat metadata, viewer modes, action presentation hints, outcome explanation fields. |
| `replay_support.rs` | Golden-trace fixture helpers, public replay export/import redaction, view-hash helpers. |

### 4.2 Filled official-game documents

The implementation must fill these game-local documents from templates before public admission:

| File | Required content |
| --- | --- |
| `games/river_ledger/docs/RULES.md` | Original Rulepath rules summary with stable `RL-*` rule IDs. Must cover seat range, setup, blinds, betting, streets, caps, contribution ledger, evaluator, showdown, split/remainder, visibility, replay, bots, and out-of-scope all-in/side-pots. |
| `games/river_ledger/docs/HOW-TO-PLAY.md` | Player-facing original prose. Must explain River Ledger without casino mimicry or copied source prose. |
| `games/river_ledger/docs/SOURCES.md` | Per-game source notes, source-use limits, variant decisions, naming rationale, and exact consulted external references. |
| `games/river_ledger/docs/RULE-COVERAGE.md` | Matrix from every `RL-*` rule to implementation modules, tests, golden traces, replay/serialization, UI, bots, benchmarks. |
| `games/river_ledger/docs/MECHANICS.md` | Game-local mechanic inventory across the atlas categories, including N-seat hidden information, betting/accounting, deck/deal, evaluator, public/private projections, and split-pot allocation. |
| `games/river_ledger/docs/AI.md` | Bot registry/status for L0/L1/L2, legal-action-only guarantee, information boundary, and non-goals. |
| `games/river_ledger/docs/UI.md` | Product UI plan, N-seat viewer matrix, pairwise no-leak matrix, observer/seat-private projections, surface budget, safe outcome explanation, and no-casino visual direction. |
| `games/river_ledger/docs/BENCHMARKS.md` | Native/WASM benchmark plan and final report by 3-, 4-, 5-, and 6-seat fixture. |
| `games/river_ledger/docs/COMPETENT-PLAYER.md` | Human strategy analysis that feeds L2, written only from authorized information. |
| `games/river_ledger/docs/BOT-STRATEGY-EVIDENCE-PACK.md` | Formal L2 authored-policy evidence pack. Must enumerate inputs, forbidden hidden facts, priority vector, opponent-count adjustments, and deterministic tie-breaks. |
| `games/river_ledger/docs/PRIMITIVE-PRESSURE-LEDGER.md` | Mechanic-atlas pressure evidence. Must compare against `high_card_duel`, `poker_lite`, `plain_tricks`, `masked_claims`, and existing accounting entries, then record `game-local / no promotion`. |
| `games/river_ledger/docs/GAME-IMPLEMENTATION-ADMISSION.md` | Receipt proving source/rules/mechanics/coverage/UI/bot/no-leak/bench prerequisites before coding and before public admission. |
| `games/river_ledger/docs/PUBLIC-RELEASE-CHECKLIST.md` | Final public/web release gate, including IP, no-leak, catalog, docs, e2e, presentation-copy, smoke, replay export/import, and bot-boundary checks. |

### 4.3 Registration and public surfaces

All registration work consumes existing Infra A-D seams.

| Surface | Required River Ledger registration |
| --- | --- |
| Workspace | Add `games/river_ledger` to root `Cargo.toml` workspace members. |
| CI game list | Add `river_ledger` to `ci/games.json` so CI discovery and game-list checks include it. |
| WASM crate | Add dependency in `crates/wasm-api/Cargo.toml`; import the crate in `crates/wasm-api/src/lib.rs`; add `RegisteredGame::RiverLedger`; update `list_games`; update catalog metadata/seat labels/viewer modes; update `MatchRecord`; update setup/action/view/effects/replay/bot dispatch; update the `#[cfg(test)]` no-leak test-harness dispatch (`pairwise_no_leak_result` / `assert_pairwise_no_leak`). Note `RegisteredGame`, `MatchRecord`, and the no-leak helpers are internal/test-only symbols — extend their dispatch in place; do not export them. |
| Simulator | Add `river_ledger` to `tools/simulate` dependency list and game dispatch with a seat-count-aware `run_river_ledger_simulation`. Add a `--seat-count` flag to `tools/simulate` (existing flags are `--games`, `--start-seed`, `--action-cap`, `--failure-report-out`; there is no `--seat-count` today). River Ledger is the first game to feed more than two seats into the simulator: build the summary from the generic `Summary::new(seats)` / `increment_seat_count` helpers (the existing `wins_by_seat: BTreeMap` shape, delivered by Infra B), **not** the two-seat `two_seat_counts` helper that every current game runner hard-codes. Infra B delivered the seat-keyed summary *shape* but not a seat-count *input*, so the `--seat-count` flag is additive new work, not an N-seat plumbing rebuild. |
| Replay checker | Add `river_ledger` to `tools/replay-check` dependency list and `resolve_game`. |
| Fixture checker | Add `river_ledger` to `tools/fixture-check` dependency list and `resolve_game`. |
| Rule coverage | Add `river_ledger` to `tools/rule-coverage` dependency list and `resolve_game`; add the `RL-*` rule ID prefix to the rule-ID validator. |
| Web WASM client | Ensure `apps/web/src/wasm/client.ts` can load River Ledger catalog entries, seat counts, seat labels, viewer modes, views, legal actions, previews, and outcome explanations from WASM without TypeScript legality. |
| Web renderer | Add `apps/web/src/components/RiverLedgerBoard.tsx` and register it in the app shell renderer mapping. It should follow `PokerLiteBoard` for hidden-info/showdown presentation patterns but must be N-seat and River Ledger-specific. |
| Web setup | `MatchSetup.tsx` must consume `supportedSeatCounts` from the catalog for `3,4,5,6` instead of hard-coding River Ledger seat counts. |
| Seat frame | `SeatFrame.tsx` should be reused for multi-seat seat order, active/pending indicators, observer, and seat-private view presentation. |
| Rules assets | Add `apps/web/public/rules/river_ledger.md` and add it to `apps/web/public/rules/manifest.json`. |
| E2E smoke | Add `apps/web/e2e/river-ledger.smoke.mjs` and include it in the web smoke suite. |
| Web README | Update `apps/web/README.md` intro catalog list, Shell Surface renderer list, action presentation audit, effect animation audit, and `smoke:e2e` list. This is enforced by `scripts/check-catalog-docs.mjs`. |
| Catalog checks | If `scripts/check-catalog-docs.mjs` contains game-specific expectations, update it for `river_ledger`. If it is data-driven, add only the README/catalog rows it checks. |
| Player rules copy | Ensure the public rules copy script includes River Ledger and that copied prose remains original and safe. |

## 5. Work breakdown

These are bounded candidate `AGENT-TASK` packets. They are **not** ticket files and do not replace the later `/spec-to-tickets` step. Each task inherits the global forbidden changes in §9 and must follow the failing-test protocol from `docs/AGENT-DISCIPLINE.md`.

### G15-RL-001 — Admission docs, source notes, and stable rule-ID plan

| Field | Candidate task packet |
| --- | --- |
| Goal | Create the pre-coding admission spine for River Ledger: `RULES.md`, `SOURCES.md`, `MECHANICS.md`, `RULE-COVERAGE.md`, `GAME-IMPLEMENTATION-ADMISSION.md`, and the initial `PRIMITIVE-PRESSURE-LEDGER.md`. |
| Inputs | This spec; official-game contract; sources policy; mechanic atlas; Texas Hold'Em and hand-ranking source notes; `poker_lite` docs as structural precedent only. |
| Deliverables | Original `RL-*` rule ID set; source-use notes; source-to-rule mapping; initial coverage matrix rows; atlas pressure record; admission checklist with explicit blockers. |
| Required rule-ID families | `RL-SETUP-*`, `RL-DEAL-*`, `RL-BET-*`, `RL-STREET-*`, `RL-EVAL-*`, `RL-SHOW-*`, `RL-POT-*`, `RL-VIS-*`, `RL-BOT-*`, `RL-UI-*`, `RL-REPLAY-*`. |
| Non-goals | No code implementation; no ticket files; no source prose copying. |
| Evidence | `cargo run -p rule-coverage -- --game river_ledger` may fail until code exists, but the doc must define all planned rows and mark status honestly. |

### G15-RL-002 — Crate scaffold and typed static-data boundary

| Field | Candidate task packet |
| --- | --- |
| Goal | Add `games/river_ledger` as a compiling game crate with typed modules, static-data loading, and no behavior in data. |
| Deliverables | `Cargo.toml`; module skeleton; `ids.rs`; `cards.rs`; `state.rs`; `variants.rs`; `ui.rs`; `data/manifest.toml`; `data/variants.toml`; fixture directory; baseline docs. |
| Implementation constraints | No YAML. No DSL. Static data describes presentation/metadata/setup parameters only. Unknown or behavior-looking data keys fail tests. |
| Non-goals | No betting/evaluator implementation yet except placeholder types needed for compilation. No `engine-core` changes. |
| Evidence | `cargo check -p river_ledger`; unit tests for variant loader behavior-key rejection; boundary check passes. |

### G15-RL-003 — Setup, 3–6-seat validation, deterministic shuffle/deal, blinds

| Field | Candidate task packet |
| --- | --- |
| Goal | Implement deterministic setup for 3–6 seats: standard 52-card deck, shuffle, button/SB/BB assignment, forced blinds, two private hole cards, reserved community deck state, and initial preflop active seat. |
| Deliverables | `setup.rs`; deck/deal helpers; setup fixtures for 3/4/5/6 seats; invalid-seat diagnostics; initial view projection tests. |
| Rule coverage | `RL-SETUP-*`, `RL-DEAL-*`, `RL-BET-BLINDS-*`, `RL-VIS-PRIVATE-HOLE-*`. |
| Non-goals | No street betting resolution beyond initial blind state. No side-pot/all-in state. |
| Evidence | Rule tests for accepted/rejected seat counts; deterministic same-seed setup equality; different-seed shuffle variance; public observer cannot see hole cards, deck order, deck tail, or burn placeholders. |

### G15-RL-004 — Fixed-limit betting engine and street advancement

| Field | Candidate task packet |
| --- | --- |
| Goal | Implement `fold`, `check`, `call`, `bet`, `raise`, fixed small/big units, cap tracking, contribution ledger, live/folded statuses, round closure, and street advancement through flop/turn/river. |
| Deliverables | `actions.rs`; `betting.rs`; `rules.rs` integration; diagnostics; effects for public contribution changes and private no-op where appropriate. |
| Rule coverage | `RL-BET-*`, `RL-STREET-*`, `RL-POT-SINGLE-*`, `RL-VIS-DIAGNOSTIC-*`. |
| Cap definition | One opening bet plus three raises per street. Diagnostics must say that the street raise cap has been reached without exposing private cards or hidden deck facts. |
| Non-goals | No showdown evaluator yet except terminal fold-out. No all-in/side-pot handling. |
| Evidence | Golden traces for blinds, calls, checks, cap reached, wrong-seat action, stale action, fold-out terminal, street reveal, and 6-seat action-order wraparound. Property tests over random legal action sequences ensure contribution ledger invariants and no negative units. |

### G15-RL-005 — Evaluator, showdown, split pot, and outcome explanation

| Field | Candidate task packet |
| --- | --- |
| Goal | Implement deterministic poker hand evaluation, seven-card 21-subset search, showdown comparison, single-pot split allocation, remainder rule, and Rust-authored outcome explanations. |
| Deliverables | `evaluator.rs`; `showdown.rs`; `pot.rs`; terminal outcome structs; public/seat-private explanation fields. |
| Rule coverage | `RL-EVAL-*`, `RL-SHOW-*`, `RL-POT-SPLIT-*`, `RL-POT-REMAINDER-*`, `RL-VIS-SHOWDOWN-*`. |
| Required categories | High card, one pair, two pair, three of a kind, straight including ace-low straight, flush, full house, four of a kind, straight flush. Royal flush is represented as the highest straight flush, not a separate category unless docs explicitly add an alias. |
| Tie-break design | Category first, then ordered rank vector. Suits never break ties. Exact five used cards are recorded for explanation. |
| Remainder design | Split winners receive equal integer shares first; remaining units are assigned one at a time by stable button-order among tied winners. |
| Non-goals | No side pots; no all-in side-ledger; no optimized lookup tables. |
| Evidence | Unit tests for every category and tie-break; property tests that evaluator comparison is deterministic and antisymmetric; golden traces for fold-out, single winner showdown, tied split, and remainder allocation; replay/hash equality for showdown traces. |

### G15-RL-006 — Visibility, effect filtering, replay export/import, and view hashes

| Field | Candidate task packet |
| --- | --- |
| Goal | Prove viewer-safe River Ledger projections for observer, each seat, replay exports, diagnostics, logs, effects, bot explanations, and WASM/browser payloads. |
| Deliverables | `effects.rs`; `visibility.rs`; `replay_support.rs`; pairwise no-leak test harness inputs; replay export/import helpers; per-seat and observer view hashes. |
| Rule coverage | `RL-VIS-*`, `RL-REPLAY-*`, `RL-SHOW-VIEWER-*`. |
| Required viewer modes | Public observer, each participating seat, internal full trace only for engine/test authority. |
| Non-goals | No trace schema migration. No dev-only leak bypass in browser payloads. |
| Evidence | Pairwise no-leak tests for every ordered pair of seats in 3/4/5/6 matches; observer no-leak; public replay export/import no-leak; diagnostics no-leak; browser payload fixture that contains no unauthorized hole, burn, or deck-tail identity. |

### G15-RL-007 — Rule, golden, property, serialization, replay, and fixture test suite

| Field | Candidate task packet |
| --- | --- |
| Goal | Build the official-game test suite matching `TESTING-REPLAY-BENCHMARKING.md`. |
| Deliverables | `tests/rules.rs`; `tests/property.rs`; `tests/replay.rs`; `tests/serialization.rs`; `tests/visibility.rs`; golden traces; fixtures. |
| Required classes | Rule tests, golden traces, property tests, simulation tests, replay/checkpoint/hash tests, serialization determinism tests, visibility/no-leak tests, fixture tests. |
| Non-goals | No deleted or weakened tests to reach green. |
| Evidence | `cargo test -p river_ledger`; `cargo run -p fixture-check -- --game river_ledger`; `cargo run -p replay-check -- --game river_ledger`; `cargo run -p rule-coverage -- --game river_ledger`. |

### G15-RL-008 — Bots L0/L1/L2 and evidence pack

| Field | Candidate task packet |
| --- | --- |
| Goal | Implement River Ledger bots and prove they use only legal actions from authorized views. |
| Deliverables | `bots.rs`; `docs/AI.md`; `docs/COMPETENT-PLAYER.md`; `docs/BOT-STRATEGY-EVIDENCE-PACK.md`; bot tests and traces. |
| Bot levels | L0 legal-random; L1 conservative public/own-hole heuristic; L2 limited opponent-count-aware heuristic. |
| L2 priority vector | Terminal safety → legal action availability → fold/call/check obligation → own-hole class estimate → board texture → call price → live-opponent count → street/cap pressure → deterministic action tie-break. |
| Hard exclusions | No MCTS, ISMCTS, Monte Carlo, ML, RL, rollout sampling, hidden deck access, hidden opponent-card access, omniscient rank estimates, or public strategy solver. |
| Non-goals | No bot UI autonomy beyond Rust-generated choices; no TypeScript bot decisions. |
| Evidence | Bot legality tests; no-leak tests on bot explanations/candidate rankings; seeded full-game bot simulations at 3/4/5/6 seats; evidence pack maps every bot input field to an authorized view field. |

### G15-RL-009 — Native simulation and benchmarks by seat count

| Field | Candidate task packet |
| --- | --- |
| Goal | Register River Ledger in native simulation and benchmarking while keeping native benchmarks primary and WASM checks secondary. |
| Deliverables | `benches/river_ledger.rs`; `benches/thresholds.json`; simulator fixtures; seat-count-aware `run_river_ledger_simulation` plus a new `--seat-count` flag on `tools/simulate`; `docs/BENCHMARKS.md`. |
| Required seats | 3, 4, 5, and 6. The 6-seat fixture is the surface-budget stress case. |
| Metrics | Setup, legal-action generation, apply-action transition, projection by viewer, full seeded playout, replay export/import, evaluator showdown batch. |
| Simulator flag | `tools/simulate` has no `--seat-count` flag today (only `--games`, `--start-seed`, `--action-cap`, `--failure-report-out`). Add `--seat-count` and feed it through a seat-count-aware River Ledger runner that builds an N-seat `wins_by_seat` summary via the generic `Summary::new(seats)` helpers. |
| Non-goals | No CI floor tuning that hides regressions; no lookup-table evaluator. |
| Evidence | `cargo bench -p river_ledger`; `cargo run -p simulate -- --game river_ledger --seat-count N` for each `N ∈ {3,4,5,6}`; benchmark report records native and WASM smoke results. |

### G15-RL-010 — Workspace, WASM, and tools registration

| Field | Candidate task packet |
| --- | --- |
| Goal | Wire River Ledger end-to-end through the existing registries and command-line tools. |
| Deliverables | Root workspace member; `ci/games.json`; `crates/wasm-api` dependency/import/enum/list/catalog/match dispatch; `tools/simulate` (with the new `--seat-count` flag from G15-RL-009); `tools/replay-check`; `tools/fixture-check`; `tools/rule-coverage` with `RL-*` prefix support. |
| Non-goals | No default-branch lookup assumptions; no N-seat plumbing rebuild; no engine-core nouns. |
| Evidence | `cargo check --workspace`; `cargo run -p simulate -- --game river_ledger`; `cargo run -p replay-check -- --game river_ledger`; `cargo run -p fixture-check -- --game river_ledger`; `cargo run -p rule-coverage -- --game river_ledger`; WASM catalog lists River Ledger with supported seat counts 3–6. |

### G15-RL-011 — Web UI, catalog, public rules, and e2e no-leak smoke

| Field | Candidate task packet |
| --- | --- |
| Goal | Add a polished public River Ledger browser surface that presents only Rust/WASM state. |
| Deliverables | `RiverLedgerBoard.tsx`; app-shell renderer registration; catalog entry; public rules markdown; rules manifest row; e2e smoke; README catalog rows. |
| UI requirements | Neutral table/card presentation; seat frames; active/pending; button/SB/BB markers; board/community cards; contribution ledger; legal action controls; preview rendering; final outcome breakdown; viewer-safe explanation. |
| Non-goals | No TypeScript legality; no hand evaluator in TypeScript; no casino chips/cash/felt/branding mimicry; no hidden data in DOM attributes, local storage, test snapshots, or console logs. |
| Evidence | `npm --prefix apps/web run build`; WASM load smoke; `smoke:e2e` including `river-ledger.smoke.mjs`; DOM no-leak assertions for observer and wrong-seat viewers; `node scripts/check-catalog-docs.mjs`; `node scripts/check-presentation-copy.mjs`. |

### G15-RL-012 — Official docs completion and public-release receipt

| Field | Candidate task packet |
| --- | --- |
| Goal | Complete the game-local official docs and public-release checklist after implementation evidence exists. |
| Deliverables | Final filled `RULES`, `HOW-TO-PLAY`, `SOURCES`, `RULE-COVERAGE`, `MECHANICS`, `AI`, `UI`, `BENCHMARKS`, `COMPETENT-PLAYER`, `BOT-STRATEGY-EVIDENCE-PACK`, `PRIMITIVE-PRESSURE-LEDGER`, `GAME-IMPLEMENTATION-ADMISSION`, `PUBLIC-RELEASE-CHECKLIST`. |
| Non-goals | No new doctrine; no ADR amendment unless a real conflict is discovered. |
| Evidence | All docs link-check; rule coverage 100% for required rows; public-release checklist signed with exact command output references. |

### G15-RL-013 — Mechanic-atlas final pressure review

| Field | Candidate task packet |
| --- | --- |
| Goal | Reassess repeated-shape pressure after implementation, before public release. |
| Deliverables | Updated River Ledger primitive-pressure ledger; proposed `docs/MECHANIC-ATLAS.md` entry; explicit `game-local / no promotion` decision or an ADR-triggering finding if the implementation unexpectedly hits a hard third-use gate. |
| Required comparisons | `high_card_duel` hidden card/deck, `poker_lite` betting/showdown/no-leak, `plain_tricks` N-seat hand/deck visibility, `masked_claims` hidden-info reveal/diagnostic discipline, existing accounting/resource entries. |
| Non-goals | No promotion in Gate 15; no `engine-core` nouns. |
| Evidence | Ledger cites modules/tests/traces and leaves §10A open debt `_None_` unless a separately accepted ADR changes that. |

### G15-RL-014 — Verification sweep and gate close

| Field | Candidate task packet |
| --- | --- |
| Goal | Run the complete acceptance suite and prepare the Gate 15 close-out update. |
| Deliverables | Command transcript summary; failure triage if needed; `specs/README.md` lifecycle update from `Planned` to `Done` only after all exit criteria pass; no ticket artifacts. |
| Required commands | Workspace tests, River Ledger tests, tools, simulations, benches, WASM build, web build, e2e smoke, doc-link checks, catalog checks, presentation-copy checks, boundary checks. |
| Non-goals | No deletion/weakened tests; no side-pot/all-in work; no hidden fallback computations in TypeScript. |
| Evidence | Final release receipt points to all green checks and no-leak matrices. |

## 6. Exit criteria

Gate 15 closes only when each ROADMAP §15 exit row has evidence.

| ROADMAP §15 exit row | River Ledger acceptance mapping |
| --- | --- |
| Setup accepts/rejects the documented seat range deterministically. | `setup.rs` accepts exactly 3–6 seats, rejects all other counts with stable diagnostics, and records button/SB/BB from stable seat order. Rule tests cover 0/1/2/3/4/5/6/7 seat inputs. Fixtures exist for 3/4/5/6 seats. Same seed + seats + setup serializes identically. |
| Public/private views prove N-player no-leak, including public observer and replay exports. | `visibility.rs`, `effects.rs`, replay export/import, bot explanations, and WASM/browser payloads pass pairwise no-leak tests for every ordered pair of distinct seats in 3/4/5/6 games. Public observer sees no hole cards, burn cards, deck order, deck tail, unrevealed folded cards, private bot inputs, or private diagnostics. Public replay exports omit hidden facts while internal full traces remain test-authority only. |
| Betting state, legal actions, contribution accounting, terminal/showdown rationale, and split results are covered by rules/tests/traces/sim/replay/hash/serialization/benchmarks. | `actions.rs`, `betting.rs`, `rules.rs`, `showdown.rs`, `pot.rs`, and `evaluator.rs` are covered by rule tests, golden traces, property tests, seeded simulations, replay/checkpoint/hash tests, serialization tests, and native benchmarks. Split-pot and remainder traces are required. |
| Bots use legal APIs and authorized views only. | L0/L1/L2 all call Rust legal-action APIs, consume only the bot seat's authorized projection plus public state, and produce viewer-safe explanation/candidate payloads. Tests fail if bots consult internal deck order, opponent hole cards, hidden burn cards, raw internal trace fields, or TypeScript-derived legality. |
| UI shows seat order, active/pending state, safe previews, final breakdown, and viewer-safe explanations with no casino trade dress. | `RiverLedgerBoard.tsx` renders from Rust/WASM catalog/view/action/outcome payloads. E2E smokes assert seat order, active/pending, viewer modes, legal-only controls, safe previews, final outcome breakdown, no hidden DOM/storage/log facts, and neutral Rulepath visual language. Presentation-copy checks reject casino, real-money, tournament mimicry, copied rules prose, and chip/cash language. |

Gate 15 must also leave these non-row invariants true:

- `engine-core` remains noun-free.
- Trace schema v1 remains unchanged.
- `poker_lite` remains present and unchanged except for shared registry patterns if needed.
- `docs/MECHANIC-ATLAS.md` has no open promotion debt caused by this gate.
- Gate 15.1 side pots remain a successor, not hidden Gate 15 scope.

## 7. Acceptance evidence

### 7.1 Command suite

The final Gate 15 close-out must record exact command output for the following, adjusted only for repository-local script names if they differ at implementation time.

```bash
# Rust crate and workspace
cargo check -p river_ledger
cargo test -p river_ledger
cargo test -p river_ledger --test rules
cargo test -p river_ledger --test property
cargo test -p river_ledger --test replay
cargo test -p river_ledger --test serialization
cargo test -p river_ledger --test visibility
cargo test -p river_ledger --test bots
cargo test --workspace

# Game tooling
cargo run -p fixture-check -- --game river_ledger
cargo run -p rule-coverage -- --game river_ledger
cargo run -p replay-check -- --game river_ledger
cargo run -p simulate -- --game river_ledger --seat-count 3 --games 1000 --start-seed 1503
cargo run -p simulate -- --game river_ledger --seat-count 4 --games 1000 --start-seed 1504
cargo run -p simulate -- --game river_ledger --seat-count 5 --games 1000 --start-seed 1505
cargo run -p simulate -- --game river_ledger --seat-count 6 --games 1000 --start-seed 1506

# Benchmarks
cargo bench -p river_ledger

# Boundary and docs
bash scripts/boundary-check.sh
node scripts/check-doc-links.mjs
node scripts/check-catalog-docs.mjs
node scripts/check-presentation-copy.mjs
node scripts/check-player-rules.mjs
node scripts/check-ci-games.mjs

# Web/WASM
npm --prefix apps/web run build
npm --prefix apps/web run smoke:e2e
```

If any command is renamed before implementation, the close-out document must cite the exact replacement and explain why it is equivalent.

### 7.2 Required test classes

| Test class | Required River Ledger coverage |
| --- | --- |
| Rule tests | Seat count validation; blinds/button; legal actions by street; check/call/bet/raise/fold legality; raise cap; contribution ledger; street advancement; fold-out terminal; evaluator categories; split/remainder allocation; diagnostics. |
| Golden traces | Every public rule path in §4.1 golden-trace list, including 3–6 setup, cap diagnostic, split even, split remainder, observer no-leak, seat-private no-leak, public replay export/import, and 6-seat bot full game. |
| Property tests | Deterministic setup; action-sequence invariants; contribution conservation; no negative ledgers; all live seats either matched or folded when a street closes; evaluator total ordering; split allocation sums to pot; stable serialization. |
| Simulation tests | Seeded L0/L1/L2 playouts at 3/4/5/6 seats. Sim summaries must use seat-keyed maps, not two-seat assumptions. |
| Replay/checkpoint/hash tests | Same seed and command stream produces identical trace/checkpoints; public replay export/import omits hidden fields; per-seat and observer view hashes are stable; internal full trace remains internal/test-only. |
| Serialization tests | Stable JSON ordering for state, views, effects, explanations, bot outputs, fixture records, and trace exports. No nondeterministic map order in public payloads. |
| Visibility/no-leak tests | Pairwise no-leak for every ordered seat pair in 3/4/5/6 games; observer no-leak; wrong-seat diagnostic no-leak; action tree no-leak; preview no-leak; effect log no-leak; replay export no-leak; DOM/storage/log no-leak through e2e. |
| Bot legality tests | Each bot chooses from legal-action tree only; bot explanations reveal no hidden facts; L1/L2 do not read raw internal state, deck tail, burn cards, or opponent hole cards. |
| Benchmark tests | Native setup, legal-action generation, apply, projection, evaluator showdown, replay export/import, and full playout for each supported seat count. WASM benchmark is smoke-level unless the existing benchmark doctrine requires more. |
| UI/e2e tests | Catalog seat counts 3–6; observer mode; seat-private mode; legal-only controls; active/pending markers; Rust-authored previews/outcome; no casino copy; no hidden facts in DOM/storage/console. |

### 7.3 Pairwise no-leak matrix

This matrix applies for all seat counts `N ∈ {3,4,5,6}` and every ordered pair `(A, B)` where `A != B`.

| Private or internal fact | Owning seat A may see | Other seat B may see | Public observer may see | Public replay export may see | Bot explanation/candidate payload may see | Browser DOM/storage/log may see |
| --- | --- | --- | --- | --- | --- | --- |
| A's own hole cards before showdown | Yes, in A's seat-private view only. | No. | No. | No unless public showdown reveal rule explicitly makes it public. | A-bot may use them internally; explanation must not expose raw cards to unauthorized viewers. | Only in A viewer DOM/session; never in other/observer DOM, logs, or snapshots. |
| B's hole cards before showdown | No, unless A is B. | Yes for B only. | No. | No. | No bot may see another seat's unrevealed holes. | No. |
| Folded seat's unrevealed hole cards | Owning folded seat may retain its private view. | No. | No. | No. | No. | No. |
| Showdown-eligible revealed hole cards | Yes if the Rust rules mark them revealed for showdown. | Yes if public showdown reveal applies. | Yes if public showdown reveal applies. | Yes if public showdown reveal applies. | Explanation may mention only authorized/public revealed cards. | Yes only after public reveal. |
| Burn cards | No viewer should see identities. Internal deterministic advancement only. | No. | No. | No. | No. | No. |
| Deck order / deck tail | No. | No. | No. | No. | No. | No. |
| Community cards already revealed | Yes. | Yes. | Yes. | Yes. | Yes. | Yes. |
| Future community cards | No. | No. | No. | No. | No. | No. |
| Contribution ledger | Public abstract units only. | Public abstract units only. | Public abstract units only. | Public abstract units only. | Public abstract units only. | Public abstract units only. |
| Legal-action tree for active seat | Active seat sees its legal commands. | Non-active seats see no private decision tree beyond public pending/action metadata. | Observer sees public pending/action metadata only. | Export may include public commands applied, not hidden alternatives. | Bot sees only its own active legal tree. | DOM shows only current viewer's authorized controls. |
| Wrong-seat/stale/cap diagnostics | Must identify public reason without hidden facts. | Same. | Same. | Same. | Same. | Same. |
| Evaluator intermediate comparisons | Reached-showdown viewer sees authorized/public used cards and category/tie vector. | Unauthorized viewers see only public-safe winner facts. | Same. | Same. | Bot explanation sees no hidden opponent cards. | Same. |
| Raw internal full trace | Test/engine authority only. | No. | No. | No. | No. | No. |

### 7.4 Golden-trace minimum set

The golden trace set in §4.1 is mandatory. Additional traces are encouraged when they reveal failure modes, but the minimum set must include:

- accepted setup at every official seat count;
- rejected setup below and above the range;
- blind posting and preflop action-order wraparound;
- street advancement through flop, turn, and river;
- small-bet and big-bet unit differences;
- raise-cap diagnostic;
- fold-out terminal with no unnecessary reveal;
- every evaluator category and at least one tie-break per category family;
- even split and deterministic remainder split;
- observer no-leak;
- seat-private no-leak;
- wrong-seat/stale diagnostics;
- public replay export/import;
- 6-seat bot-vs-bot full game;
- WASM-exported trace.

### 7.5 Benchmark expectations

Native benchmarks are the source of performance truth. River Ledger must benchmark the expensive paths that could regress as seat count increases:

- setup/deal/shuffle for 3/4/5/6 seats;
- legal-action tree generation in preflop, flop, turn, and river states;
- apply-action transition in call/check/bet/raise/fold states;
- projection for observer and every seat;
- public replay export/import;
- full seeded playout;
- evaluator showdown batch, including 6 seats × 21 candidate five-card hands.

Throughput thresholds must be realistic, explicit, and variance-aware. They must not incentivize a lookup-table evaluator or weaken explanation fidelity.

## 8. FOUNDATIONS & boundary alignment

| Principle / contract | River Ledger stance | Rationale |
| --- | --- | --- |
| Rust behavior authority | Required and non-negotiable. | All legality, validation, betting, evaluator, showdown, effects, views, bot decisions, replay, and serialization are Rust-owned. React renders only Rust/WASM outputs. |
| TypeScript presentation-only | Required and non-negotiable. | Web code may map Rust action IDs to buttons and SVG/card presentation, but may not compute legal actions, call prices, hand ranks, winner selection, split allocation, or hidden-card availability. |
| `engine-core` noun-free | Preserved. | No card/deck/hand/pot/blind/button/street/evaluator nouns are added to `engine-core`. Existing generic `SeatId`, `Actor`, `Viewer`, `VisibilityScope`, and `Game` APIs are sufficient. |
| Game-local typed Rust nouns | Required. | River Ledger's card/deck/pot/evaluator/accounting types live in `games/river_ledger`. |
| `game-stdlib` promotion boundary | No promotion in Gate 15. | Mechanic-atlas §10A open debt is `_None_`. River Ledger records pressure but keeps helpers local. Any future promotion requires the atlas process and, if necessary, an ADR. |
| Static data boundary | Preserved. | `data/*.toml` may hold metadata, presentation labels, and setup parameters only. It may not hold rule selectors, conditions, triggers, evaluator formulas, betting formulas, or action logic. No YAML and no DSL. |
| Determinism | Required. | Shuffle, deal, action order, betting transitions, split remainders, bot tie-breaks, serialization, replay, and hashes must be stable under seed + setup + command stream. |
| Trace schema v1 | Reused. | The existing `seats` array is N-seat-capable. Gate 15 adds stricter semantics around per-seat/observer view hashes and no-leak tests, not a schema migration. |
| Hidden-information no-leak | Required across every surface. | Hole cards, burn cards, deck order/tail, future community cards, raw full trace, private bot inputs, private diagnostics, and folded unrevealed cards stay out of unauthorized payloads, effects, DOM, storage, logs, public replay exports, and explanations. |
| Legitimate inference | Allowed only from public facts. | A bot or player may infer from public betting, board texture, contribution units, and live-opponent count. It may not receive hidden payload facts disguised as inference. |
| Bot law | L0/L1/L2 only. | Legal-action-only, authorized-view-only, deterministic heuristics. No MCTS/ISMCTS/Monte Carlo/ML/RL, no hidden-state sampling, and no external solver. |
| Official-game admission | Fully discharged. | The deliverables include original rules prose, how-to-play, source notes, rule coverage, mechanic inventory, competent-player workflow, bot evidence pack, UI plan, benchmarks, and release checklist. |
| N-seat law | Fully engaged. | 3–6 seats, stable seat roles/order, observer projection, pairwise no-leak matrix, seat-private views, public replay exports, simulator summaries, final breakdowns, and larger-surface budgets are all mandatory. |
| UI interaction | Preserved. | Legal-only controls, Rust-authored previews, effect-driven presentation, neutral original visuals, and safe outcome explanations. |
| IP policy | Preserved. | River Ledger is original Rulepath presentation of a public-domain rules family. No copied prose, no casino trade dress, no tournament/product mimicry. |
| ADR discipline | Preserved. | Any true need for trace schema migration, engine-core noun growth, behavior DSL, or primitive promotion is an ADR trigger, not an implementation shortcut. |
| AGENT discipline | Preserved. | Candidate tasks are bounded packets. Failing tests are investigated as valid/invalid before SUT/test fixes. Tests are not deleted or weakened to go green. |

### Mechanic-atlas pressure stance

River Ledger arms these pressure rows:

| Mechanic shape | Existing comparison set | Gate 15 decision |
| --- | --- | --- |
| Hidden card/deck setup | `high_card_duel`, `plain_tricks`, `secret_draft`, `poker_lite` | Game-local implementation; record comparison in `PRIMITIVE-PRESSURE-LEDGER.md`; no promotion. |
| Seat-private hand projection | `high_card_duel`, `plain_tricks`, `poker_lite` | Reuse projection patterns, not code promotion. River Ledger must extend to 3–6 seats and all ordered seat pairs. |
| Betting/contribution accounting | `poker_lite`, existing resource/accounting docs | Game-local single-pot contribution ledger; no `game-stdlib` accounting primitive. |
| Showdown/evaluator | `poker_lite` simplified showdown; external hand-ranking sources | Game-local full Hold'Em evaluator and explanation. No lookup-table optimization and no shared evaluator promotion. |
| Split pot/remainder | `poker_lite` tie split and public-game tiebreak precedents | Game-local split allocation with stable button-order remainder rule; record pressure. |
| Public replay no-leak | `high_card_duel`, `plain_tricks`, `secret_draft`, `masked_claims`, `poker_lite` | Adopt the internal-full-trace vs viewer-scoped-export taxonomy. No schema migration. |

The atlas decision for Gate 15 is explicit: **game-local / no promotion / no open promotion debt**. If implementation reveals a third-use hard gate that cannot be honestly handled by local pressure recording, work must stop and an ADR or separate primitive-promotion gate must precede public release.

## 9. Forbidden changes

River Ledger implementation must not:

1. Add card, deck, hand, pot, blind, button, street, evaluator, contribution, or poker nouns to `engine-core`.
2. Promote River Ledger helpers into `game-stdlib` during Gate 15.
3. Move legality, validation, betting, hand evaluation, winner selection, split allocation, bot decisions, or visibility filtering into TypeScript.
4. Add YAML, a DSL, behavior formulas, rule selectors, triggers, conditions, or scriptable behavior data.
5. Fetch, branch, or infer behavior from repository metadata, default branches, or prior chats during implementation audits; use the repository state selected by the implementing task.
6. Migrate trace schema v1 without an accepted ADR.
7. Leak hole cards, folded unrevealed cards, burn cards, future community cards, deck order, deck tail, raw internal traces, private bot inputs, or private diagnostics to unauthorized viewers.
8. Place hidden facts in effect logs, browser DOM attributes, local/session storage, console logs, replay exports, simulator summaries, bot explanations, candidate rankings, error messages, or test snapshots.
9. Implement MCTS, ISMCTS, Monte Carlo rollouts, ML, RL, external solvers, lookup strategy tables, hidden-state sampling, or omniscient bots.
10. Add all-in or side-pot mechanics to the Gate 15 base game.
11. Add no-limit, pot-limit, tournament, lobby, payout, rake, currency, real-money, or gambling-product features.
12. Use casino trade dress, tournament branding, copied rules prose, copied images/assets, chip/cash language, or product mimicry.
13. Delete, weaken, skip, or rewrite valid tests to make CI green.
14. Rename or replace `poker_lite`; River Ledger coexists with it.
15. Decompose this spec into ticket files inside this deliverable.

## 10. Documentation updates required

### 10.1 `specs/README.md`

The index already carries an Order 5 / Gate 15 seed row (`_(seed; unwritten)_`, status `Not started`). **Update** that existing row when this spec is admitted — replace `_(seed; unwritten)_` with the spec path and flip the status `Not started` → `Planned`; do not add a second Order 5 row. Flip to `Done` only after the Gate 15 exit criteria and release checklist are satisfied.

Paste-ready replacement row (reconciling the existing seed row's interlock note):

```markdown
| 5 | Gate 15 — River Ledger / Texas Hold'Em base | `specs/gate-15-river-ledger-texas-holdem-base.md` | Planned | First official 3–6-seat hidden-information betting game; fixed-limit capped-raise; split pots; Rust-authored showdown; N-player no-leak. |
```

The close-out update must preserve the determination evidence:

- Phase 0 foundation realignment: `Done`.
- Infra A-D N-seat public infrastructure: `Done`.
- Gate 15 is the lowest non-`Done` public-scaling unit at admission.
- Mechanic-atlas open promotion-debt register: `_None_`.

### 10.2 `docs/SOURCES.md`

Add a global source note for the Hold'Em rules family. Paste-ready text:

```markdown
### Texas Hold'Em rules family / River Ledger

- John McLeod, Pagat, “Texas Hold'em Poker - card game rules,” consulted 2026-06-14.
  Used to verify public-domain rules-family facts: broad player range, 52-card deck,
  dealer/button and blinds, hole cards, preflop/flop/turn/river betting streets,
  fixed-limit doubled later streets, showdown best five from seven, and split/tie examples.
  Rulepath prose remains original and the public product name is River Ledger.

- John McLeod, Pagat, “Poker Hand Ranking,” consulted 2026-06-14.
  Used to verify five-card hand category order, ace-low straight treatment, no suit ranking,
  and same-category tie-break comparison. Rulepath evaluator and explanation prose remain original.

- Naipes Heraclio Fournier S.A., “How to play Texas Hold'em,” consulted 2026-06-14.
  Used only as corroboration for fixed-limit four-round structure, blinds, doubled turn/river
  betting unit, and bounded raises. No prose, imagery, or product presentation is copied.

- Marc Lanctot et al., “OpenSpiel: A Framework for Reinforcement Learning in Games,”
  arXiv:1908.09453, consulted 2026-06-14.
  Used as external prior art vocabulary for N-player imperfect-information game representation
  and player/viewer information boundaries. Rulepath does not import OpenSpiel algorithms,
  MCTS, reinforcement learning, or architecture.

- boardgame.io, “Open Source Game Engine for Turn-Based Games,” consulted 2026-06-14.
  Used as web-game prior art for turn phases, logs, multiplayer/view-layer separation, and
  player-specific presentation concepts. Rulepath remains Rust-authoritative and does not adopt
  boardgame.io architecture.
```

### 10.3 `docs/MECHANIC-ATLAS.md`

Add or update a River Ledger pressure entry that records:

- new game: `river_ledger`;
- repeated shapes: standard deck, hidden hands, N-seat private projections, betting/contribution ledger, showdown evaluator, split allocation;
- comparison set: `high_card_duel`, `poker_lite`, `plain_tricks`, `masked_claims`, existing accounting/resource entries;
- decision: `game-local / no promotion`;
- open debt: `_None_` unless an accepted ADR or later primitive gate changes it.

Paste-ready row fragment:

```markdown
| River Ledger / Gate 15 | Standard deck, hidden hole cards, N-seat projections, fixed-limit contribution ledger, seven-card evaluator, split allocation | `high_card_duel`; `poker_lite`; `plain_tricks`; `masked_claims`; existing accounting entries | Armed pressure only. Keep game-local for Gate 15; no `game-stdlib` promotion and no `engine-core` noun. Record implementation evidence in `games/river_ledger/docs/PRIMITIVE-PRESSURE-LEDGER.md`. |
```

### 10.4 Game-local docs

Create and fill every document listed in §4.2. Silent omissions are not allowed; use explicit `not applicable` rows where a template field does not apply.

### 10.5 `apps/web/README.md`

Update all catalog-enforced surfaces:

- intro game list: add River Ledger;
- Shell Surface renderer list: add `RiverLedgerBoard`;
- action presentation audit: River Ledger is `board-native` because the renderer maps Rust legal actions to neutral table/card controls;
- effect animation audit: River Ledger is `adopt` for deal/community-reveal/contribution/showdown feedback if custom presenters are registered, otherwise `generic-only` must be recorded honestly until presenters exist;
- `smoke:e2e` list: add `river-ledger.smoke.mjs`;
- no-leak/a11y checklist: add River Ledger observer and seat-private DOM assertions.

`node scripts/check-catalog-docs.mjs` must pass after these edits.

### 10.6 Web/public rules docs

Update:

- `apps/web/public/rules/river_ledger.md`;
- `apps/web/public/rules/manifest.json`;
- `scripts/copy-player-rules.mjs` or equivalent copy path if it is game-specific;
- `node scripts/check-player-rules.mjs` expected catalog if it is game-specific.

### 10.7 Registration docs and manifests

Update:

- root `Cargo.toml` workspace member list;
- `crates/wasm-api/Cargo.toml`;
- `crates/wasm-api/src/lib.rs` registry and dispatch;
- `tools/simulate`;
- `tools/replay-check`;
- `tools/fixture-check`;
- `tools/rule-coverage`;
- `ci/games.json`.

### 10.8 External references used by this spec

The external facts shaping this spec are limited and conservative:

- Texas Hold'Em commonly uses a standard 52-card deck, private hole cards, blinds/button, flop/turn/river public cards, betting rounds, fixed-limit structures, and best-five-from-seven showdown.[^pagat-holdem][^fournier-holdem]
- Poker hand comparison is based on five-card hands, category order, rank tie-breaks, ace-low straight handling, and no suit ranking.[^pagat-ranking]
- OpenSpiel is prior art that frames games across N-player, turn-taking/simultaneous, and imperfect-information dimensions; this spec uses that only to sharpen Rulepath's explicit information-boundary vocabulary.[^openspiel]
- boardgame.io is prior art for turn-based phases/logs and view-layer separation; this spec uses that only as a contrast with Rulepath's Rust-authoritative browser boundary.[^boardgameio]

## 11. Sequencing

### 11.1 Predecessors

Gate 15 is admitted only because these predecessors are already complete in the exact-target evidence set:

| Predecessor | Status | Gate 15 implication |
| --- | --- | --- |
| Phase 0 — foundation realignment | `Done` | Official-game, N-seat, bot, testing, boundary, and template requirements are already in place. |
| Infra A — N-seat setup/catalog metadata | `Done` | River Ledger consumes `supportedSeatCounts`, seat labels, roles, and viewer modes instead of rebuilding setup metadata plumbing. |
| Infra B — seat-keyed simulator summaries | `Done` | River Ledger simulator results must use seat-keyed maps across 3–6 seats. |
| Infra C — multi-seat shell frame | `Done` | River Ledger reuses the shared multi-seat shell/seat frame. |
| Infra D — N-player no-leak harness | `Done` | River Ledger extends the harness with Hold'Em hidden facts and 3–6-seat pairwise matrices. |
| Mechanic-atlas open debt | `_None_` | No primitive promotion/back-port blocks Gate 15 admission. |

### 11.2 Admission rule before coding

Implementation may start only after `G15-RL-001` produces a reviewed admission spine:

- original `RULES.md` with stable `RL-*` rules;
- source notes and source-use limits;
- mechanic inventory and primitive-pressure ledger;
- planned rule-coverage matrix;
- implementation-admission receipt;
- explicit no-leak viewer matrix;
- bot evidence-pack outline.

If the reassess-spec step finds that any foundation/architecture conflict exists, execution stops until the conflict is resolved by spec correction or accepted ADR.

### 11.3 Successor

Gate 15.1 is the side-pot/all-in extension. It may begin only after Gate 15 is `Done` and public-release evidence confirms the base game is deterministic, replayable, no-leak, and stable.

Gate 15.1 may add:

- contribution capacity limits;
- all-in terminal/legal states;
- side-pot construction;
- side-pot showdown allocation;
- side-pot explanations;
- extra no-leak tests.

Gate 15 must not prebuild this machinery. It may leave clear extension seams and `not applicable until Gate 15.1` rows, but not dormant side-pot logic.

## 12. Assumptions

1. `GAT15-RIVLED-THEBASE-001`, owner wording, and `RL-*` stable rule-ID prefix are correctable administrative fields.
2. `tools/simulate` is currently two-seat-oriented (every game runner hard-codes the `two_seat_counts` helper); River Ledger is the first game to exercise 3–6 seats through the simulator. The seat-keyed `wins_by_seat: BTreeMap` summary shape from Infra B is sufficient, but the `--seat-count` flag and a seat-count-aware runner are new additive work in G15-RL-009/010.
3. The Rulepath v1 fixed-limit cap is one opening bet plus three raises per street. If maintainers prefer four raises after the opener, they must update `RL-BET-*` rules, traces, bots, and benchmarks before ticket decomposition.
4. Burn cards are optional internal deterministic deck advancement. If implemented, they are never visible; if omitted, source notes must say River Ledger abstracts them away without changing public rules outcomes.
5. A single-hand match is sufficient for Gate 15 base. Button rotation must still be deterministic and documented so later multi-hand or tournament-like variants do not invent incompatible semantics.
6. Contribution units are abstract counters with high capacity. Any later capacity/all-in behavior belongs to Gate 15.1.
7. Royal flush is treated as the highest straight flush unless maintainers choose to expose it as a presentation alias in `RULES.md`; either way the evaluator comparison is identical.
8. External source references verify public rules facts only. They do not license copied prose, copied assets, casino presentation, or product mimicry.

[^pagat-holdem]: John McLeod, Pagat, “Texas Hold'em Poker - card game rules,” https://www.pagat.com/poker/variants/texasholdem.html, consulted 2026-06-14.
[^pagat-ranking]: John McLeod, Pagat, “Poker Hand Ranking,” https://www.pagat.com/poker/rules/ranking.html, consulted 2026-06-14.
[^fournier-holdem]: Naipes Heraclio Fournier S.A., “How to play Texas Hold'em,” https://www.nhfournier.es/en/como-jugar/texas-holdem/, consulted 2026-06-14.
[^openspiel]: Marc Lanctot et al., “OpenSpiel: A Framework for Reinforcement Learning in Games,” arXiv:1908.09453, https://arxiv.org/abs/1908.09453, consulted 2026-06-14.
[^boardgameio]: boardgame.io, “Open Source Game Engine for Turn-Based Games,” https://boardgame.io/, consulted 2026-06-14.

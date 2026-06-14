# Gate 15 — River Ledger / Texas Hold'Em base: implementation-spec research brief

*Paste this entire document into a ChatGPT-Pro deep-research session and upload the
manifest named in §1. You (Session 2) have full read access to the `joeloverbeck/rulepath`
repository. Produce the deliverable directly — do not interview, do not ask clarifying
questions. The requirements below are final.*

---

## 1. Context

The uploaded manifest `manifest_2026-06-14_47e202a.txt` is the exact path inventory of the
`joeloverbeck/rulepath` repository at commit `47e202a` — a Rust-first, rule-enforcing,
replayable, testable card/board-game platform where **Rust owns all behavior and
TypeScript/React present only**. The foundation docs are an ordered, layered authority
indexed by `docs/README.md`: `FOUNDATIONS.md` (the constitution) → `ARCHITECTURE.md` →
`ENGINE-GAME-DATA-BOUNDARY.md` → the area docs → `ROADMAP.md`; earlier documents govern later
ones, and accepted ADRs supersede them only by explicitly naming the affected sections.

**Fetch every file from commit `47e202a`** — the manifest reflects that exact tree. This is
the verified repository HEAD at brief-authoring time (clean working tree). Earlier reports in
`reports/` cite their own older "commit of record" (for example
`public-game-ladder-and-implementation-order.md` names `e3b1729…`); those are each that
report's own baseline and predate later merges. Ignore them as fetch baselines and use
`47e202a`. If you find any file referenced below missing at `47e202a`, say so in the
deliverable rather than silently substituting another commit.

## 2. Read in full (authority order)

Read these in full, in this order, before producing. Each line states why it is load-bearing
*for a Gate 15 Texas Hold'Em spec*.

**Foundation (govern everything):**

```
docs/README.md — authority order and the layering rule; tells you which doc wins on conflict.
docs/FOUNDATIONS.md — the constitution: §11 universal acceptance invariants, §12 stop conditions, §13 ADR triggers; every line of the spec must satisfy these (esp. Rust behavior authority, determinism, no-leak).
docs/ARCHITECTURE.md — workspace shape, dependency direction, runtime transition model, §7 semantic effect log, §8 replay/checkpoint/hash, §10 WASM API shape, §11 game module shape; the spec's deliverable tree must match these.
docs/ENGINE-GAME-DATA-BOUNDARY.md — engine-core stays noun-free; typed Rust game module authoring (§4), allowed static data (§5), variant enum discipline (§9), game-stdlib promotion boundary (§13). River Ledger's card/deck/pot nouns live in the game crate, never the kernel.
```

**Gate-15 governing contracts:**

```
docs/OFFICIAL-GAME-CONTRACT.md — the official-game definition of done: §1 done, §3 requirements-first workflow, §5 original rules prose + how-to-play + outcome explanation, §6 rule coverage matrix, §7 mechanic inventory, §8 competent-player workflow, §9 bot requirements by role, §10 UI exposure incl. N-seat seat-range/observer/seat-private projections, §12 acceptance check. This is the admission bar the spec must fully discharge.
docs/MULTI-SEAT-AND-SURFACE-CONTRACT.md — the N-seat law River Ledger is the first real test of: seat-range declaration (§2), roles/turn-order model (§3–§4), viewer matrix (§5), pairwise no-leak matrix for 3+ hidden-info seats (§6), public-observer rules (§7), larger-surface budgets (§8), outcomes/final-breakdowns (§11), trace/view-hash and simulator-summary expectations (§12–§13), spec/ticket minimums (§14).
docs/AI-BOTS.md — bot law: §1 legal-action-only, §2 bot levels, §3 hard exclusions (no MCTS/ISMCTS/Monte Carlo/ML/RL), §4 information boundary, §4A N-player imperfect-information bots (allowed inference vs forbidden hidden-state peeking), §8 lexicographic priorities, §9 competent-player intake. Bounds the L0/L1/L2 bot scope.
docs/TESTING-REPLAY-BENCHMARKING.md — required test classes (rule/golden/property/simulation/replay/serialization/visibility), §8 N-seat pairwise no-leak taxonomy, §13 failing-test protocol, §14 native-first benchmark doctrine, CI expectations. The spec's acceptance-evidence section maps to this.
docs/TRACE-SCHEMA-v1.md — the `seats` array is already N-seat-capable; §2 root fields, §4 checkpoints/hashes and per-seat view-hash evidence. The spec must NOT migrate the schema — reuse it and document stricter N-seat semantics.
docs/WASM-CLIENT-BOUNDARY.md — Rust/WASM→browser operation groups and multi-seat shapes the new game registers through; the spec's WASM work must stay inside these.
docs/UI-INTERACTION.md — public UI target (legal-only, preview-driven, effect-driven animation) and visual direction (no casino mimicry). Bounds the GAME-UI deliverable.
docs/IP-POLICY.md — public/private content policy; the "no casino trade dress / original presentation" constraint and the River Ledger naming posture.
docs/AGENT-DISCIPLINE.md — §1 agent role and forbidden changes, §2 required task packet, §4 failing-test protocol; the work-breakdown's candidate AGENT-TASKs must conform.
docs/MECHANIC-ATLAS.md — §4 first/second/third-use rule and hard gate, §9A next-phase armed interlocks (the River Ledger row tells you which prior games to compare before reusing/promoting any card/deck/hand/evaluator/accounting helper), §10A open promotion-debt register (currently `_None_`). The spec must arm/record the right atlas pressure, not promote prematurely.
docs/SOURCES.md — where Hold'Em rule sources and variant decisions are recorded; the spec's source notes must follow §2 source-use rules (summarize, never copy prose).
docs/adr/0004-hidden-info-replay-export-taxonomy.md — internal full trace vs viewer-scoped replay export, the viewer-aware WASM visibility contract, and the N-player pairwise extension River Ledger must satisfy.
docs/adr/0007-next-public-scaling-phase-and-gate-p-tail.md — the accepted decision that admits the public scaling phase and seeds Gate 15+; the spec builds on it and must not silently amend it.
```

**Roadmap & living index (the determination evidence):**

```
docs/ROADMAP.md — §15 "Public scaling phase" and the Gate 15 subsection fix the scope, exit list, and "Not allowed" list the spec carries verbatim. This is ladder law.
specs/README.md — the living progress tracker; proves Phase 0 and Infra A–D are Done and Gate 15 is the lowest non-Done unit (Order 5, unwritten seed). Also defines the 12-section spec format the deliverable must follow.
```

**Precedent reports (frame the task as a delta, not a cold start):**

```
reports/public-game-ladder-and-implementation-order.md — §5 is the advisory Hold'Em scope (3–6 seats, fixed-limit capped-raise, defer side pots, split pots, 21-subset evaluator, bot levels, showdown explanation) the locked decisions in §3 below ratify; §6 enumerates the capability gaps. Treat as advisory input, not law (it cites an older baseline — see §1).
reports/foundation-doc-realignment.md — the doc/template realignment that Phase 0 executed; confirms the N-seat contract and template fields the spec depends on are already in place.
```

**Templates a new official game gate must fill (the spec's Deliverables section enumerates filled instances of these):**

```
templates/AGENT-TASK.md — the bounded packet each work-breakdown item becomes.
templates/GAME-RULES.md — original Rulepath rules summary with stable rule IDs.
templates/GAME-MECHANICS.md — game-local mechanic inventory (15 atlas categories, status labels).
templates/GAME-RULE-COVERAGE.md — rule-ID → implementation/tests/traces/UI/bots/benchmarks traceability matrix.
templates/GAME-AI.md — bot registry/status.
templates/GAME-UI.md — product UI plan incl. N-seat viewer/pairwise matrices.
templates/GAME-BENCHMARKS.md — native/WASM benchmark report with seat count and surface fixtures.
templates/GAME-HOW-TO-PLAY.md — player-facing original prose.
templates/GAME-SOURCES.md — per-game source notes and variant/naming rationale.
templates/COMPETENT-PLAYER.md — strategy analysis feeding the Level 2 bot.
templates/BOT-STRATEGY-EVIDENCE-PACK.md — formal Level 2 authored-policy design pack.
templates/PRIMITIVE-PRESSURE-LEDGER.md — game-local evidence ledger for repeated-shape/promotion decisions.
templates/GAME-IMPLEMENTATION-ADMISSION.md — gate receipt proving prerequisites before coding.
templates/PUBLIC-RELEASE-CHECKLIST.md — final public/web exposure gate.
```

### Code seams to inspect directly (inspect, not read-fully — read these *in the repo*; they are NOT pasted here and NOT part of the read-in-full set)

- `games/poker_lite/` — the closest exemplar: module split `setup.rs` / `state.rs` (`Phase`, `PledgeRoundState`, `TerminalOutcome`, `ShowdownReveal`), `rules.rs` (`apply_action`, `compare_showdown`), `effects.rs` (public/private effect filtering), `visibility.rs` (`filter_effects_for_viewer`, `project_view`, `PrivateView`/`PublicView`), `actions.rs` (`legal_action_tree`, `validate_command`), `bots.rs` (`PokerLiteLevel2Bot`), `ids.rs` (`PokerLiteSeat`, `STANDARD_SEAT_COUNT = 2`). River Ledger is a *new coexisting crate*, not a rename — but it should follow this shape and lift the betting/showdown/no-leak lessons.
- `games/high_card_duel/`, `games/secret_draft/`, `games/plain_tricks/`, `games/masked_claims/` — the other card/hidden-info exemplars and the atlas comparison set named in MECHANIC-ATLAS §9A; inspect their `setup.rs` seat-count validation and visibility patterns.
- Infra A–D landings (completed 2026-06-14) — reuse, do not rebuild: `crates/wasm-api/src/lib.rs` (`RegisteredGame` enum, `catalog_seat_metadata_fields`/`catalog_seat_labels_json`/`catalog_viewer_modes_json`, `list_games`, `rulepath_get_view_for_viewer`, `pairwise_no_leak_result`/`assert_pairwise_no_leak`); `tools/simulate/src/main.rs` (`Summary.wins_by_seat: BTreeMap`, `run_simulation` game dispatch); `apps/web/src/components/SeatFrame.tsx` and `MatchSetup.tsx` (`supportedSeatCounts`); `apps/web/src/wasm/client.ts` (`GameCatalogEntry` seat fields).
- `crates/engine-core/src/lib.rs` + `game.rs` — generic seat machinery (`SeatId`, `Actor`, `Viewer`, `VisibilityScope::PrivateToSeat`, `Game::setup(seed, seats: &[SeatId], …)`); confirm noun-free — River Ledger adds no kernel nouns.
- Registration surfaces a new crate must touch (the spec's work-breakdown must name these): `Cargo.toml` workspace members; `crates/wasm-api` imports + `RegisteredGame` + `list_games` + `MatchRecord`; `tools/simulate`, `tools/replay-check`, `tools/fixture-check`, `tools/rule-coverage` game dispatch/`resolve_game`; `apps/web` catalog + smoke lists; `apps/web/README.md` catalog surfaces (enforced by `scripts/check-catalog-docs.mjs`).
- Confirmed at `47e202a`: **no `river_ledger` crate and no `gate-15` spec exist yet** — this is a greenfield gate spec.

## 3. Settled intentions (final — do not reopen)

These decisions were resolved before this brief was authored. They pre-empt every clarifying
question you might otherwise ask.

1. **The next unit is Gate 15 — River Ledger / Texas Hold'Em base, and it is locked.**
   `specs/README.md` shows Phase 0 (`Done`) and Infra A–D (`Done`, completed 2026-06-14); Gate
   15 (Order 5) is the lowest non-`Done` row and still an unwritten seed. **Confirm-and-document
   this determination** in the spec, citing the evidence that fixed it (Phase 0 + Infra A–D
   done; Gate 15 lowest non-`Done`; promotion-debt register empty). Do **not** re-open
   "what should we build next" — that question is answered.

2. **The atlas interlock is satisfied.** `docs/MECHANIC-ATLAS.md` §10A open promotion-debt
   register reads `_None_`. No primitive promotion or back-port interposes before Gate 15. The
   spec must still *arm/record* the correct atlas pressure per §9A (compare against
   `high_card_duel`, `poker_lite`, `plain_tricks`, `masked_claims`, and existing accounting
   entries before any card/deck/hand/evaluator/accounting helper is reused or promoted) and
   keep River Ledger's card/deck/hand/evaluator/pot behavior **game-local** — no `game-stdlib`
   promotion and no `engine-core` noun.

3. **Scope is locked to ROADMAP §15 + advisory §5** — commit to these firmly in the spec, not
   as open options:
   - **Seats:** official **3–6** seats; setup deterministically accepts that range and rejects
     anything outside it. Heads-up is *not* the primary official path.
   - **Deck/deal:** standard 52-card deck; deterministic shuffle from engine RNG; two private
     hole cards per seat; five community cards; Hold'Em street structure (preflop → flop →
     turn → river → showdown). Burn cards, if modeled, are internal deterministic deck
     advancement and never leak to any view/DOM/log/export.
   - **Betting model:** **fixed-limit, capped-raise**. Button/SB/BB rotate in seat order;
     legal actions fold/check/call/bet/raise plus street advancement; abstract contribution
     units (no casino chip language); small bet preflop/flop, big bet turn/river; raises per
     street capped to bound action trees.
   - **Split pots:** **IN scope.** Tied best hands among showdown-eligible seats split the pot;
     integer-unit remainders allocated deterministically by stable button-order among tied
     winners and explained in the outcome rationale.
   - **All-in / side pots:** **DEFERRED to Gate 15.1.** v1 uses contribution capacity high
     enough that legal play cannot require all-in handling. Do not design side-pot machinery
     into the base spec.
   - **Hand evaluator:** straightforward deterministic five-card evaluator + seven-card
     best-hand search by enumerating the 21 five-card subsets, returning a comparable
     (category + ordered tie-break vector) tuple *and* the exact cards used, for explanation.
     No lookup-table optimization; correctness/explainability/replayability over throughput.
   - **Showdown explanation:** **Rust-authored and mandatory** — per seat: folded-before vs
     reached-showdown; private cards revealed only when the viewer is authorized; best five
     from seven; hand category; ordered tie-break vector; decisive comparison; pot allocation
     incl. ties and remainder rule. A fold-out winner gets a distinct "last live hand" rationale
     with no unnecessary reveal of folded seats' private cards.
   - **Bots:** **Level 0 / Level 1 / Level 2 only** — L0 legal-random from its own view; L1
     conservative policy from hole strength, board texture, call price, live-opponent count,
     street; L2 limited opponent-count-aware heuristic. **No** MCTS/ISMCTS/Monte Carlo/ML/RL.
     The bot evidence pack proves every decision derives only from the bot's authorized seat
     view.
   - **IP posture:** public/product name **River Ledger**; docs label the rules family **Texas
     Hold'Em rules family**; neutral table/card language, no casino trade dress, tournament
     branding, or borrowed prose. Original Rulepath presentation.

4. **River Ledger is a new crate that coexists with `poker_lite` / Crest Ledger** — it is not a
   rename or replacement. It reuses `poker_lite`'s patterns (betting-state clarity, hidden-info
   visibility, showdown explanation) but is its own `games/river_ledger` crate and its own spec.

5. **Build on the landed Infra A–D seams, do not rebuild N-seat plumbing.** Seat metadata/setup
   acceptance, seat-keyed simulator summaries, the multi-seat shell frame, and the pairwise
   no-leak harness already exist. The spec's work-breakdown consumes and extends them; it does
   not re-introduce N-seat infrastructure.

6. **Trace schema is reused, not migrated.** The v1 `seats` array already expresses N seats.
   The spec documents stricter N-seat semantics (per-seat + observer view hashes, ≥3-seat
   wrong-seat diagnostics) but proposes a schema change only via ADR if a genuine
   simultaneous/pending-actor expressiveness gap is proven — and the base game is not expected
   to need one.

`assumption:` the spec's machine fields (Spec ID string, owner, exact stable rule-ID prefixes)
follow the repo's canonical spec format and sibling archived specs (e.g.
`archive/specs/gate-10-poker-lite-betting-showdown.md`,
`archive/specs/gate-0-repository-skeleton.md`); the user can correct any field after delivery.

## 4. The task

Produce a single **new implementation spec** — a Rulepath roadmap-gate spec for **Gate 15 —
River Ledger / Texas Hold'Em base** — that turns the locked ROADMAP §15 scope into a concrete,
reviewable, bounded plan an agent team can execute. It is a *new official game* spec: it must
discharge the full official-game admission bar (`OFFICIAL-GAME-CONTRACT.md`) for the first
official 3–6-seat hidden-information betting game, prove N-player no-leak across every surface,
keep all behavior in typed Rust with TypeScript presenting only, stay deterministic and
replayable, and arm the correct mechanic-atlas pressure without premature promotion. The spec
is the bridge between ROADMAP ladder law and the later `/reassess-spec` → `/spec-to-tickets`
decomposition; it ends at bounded **candidate AGENT-TASKs**, not at ticket files.

## 5. Exploration + online-research mandate

Explore the repository as deeply as needed beyond the files listed above — read the code seams,
trace how a sibling game crate (especially `poker_lite`) is wired end-to-end through
`wasm-api`, the four `tools/*` dispatchers, and the web catalog, so the work-breakdown names
every real registration touchpoint.

Research online as deeply as needed and cite sources for any external claim that shapes a
decision:

- **Texas Hold'Em rules** (player range, blinds/button rotation, betting streets,
  flop/turn/river/showdown, fixed-limit capped-raise structure) — e.g. Pagat
  (`pagat.com/poker/variants/texas_holdem.html`).
- **Poker hand ranking and comparison** — e.g. Pagat (`pagat.com/poker/rules/ranking.html`).
- **Hand-evaluation approaches** (the 21-subset enumeration vs lookup tables) to justify the
  locked simple-evaluator choice on correctness/auditability grounds.
- **N-player and imperfect-information game-framework prior art** for representation and no-leak
  reasoning — e.g. OpenSpiel (`arxiv.org/abs/1908.09453`), boardgame.io (`boardgame.io`).

Use external research to *sharpen* the rules summary, evaluator design, no-leak test matrix,
and bot heuristics — never to relax a locked decision in §3 or a constraint in §6. Propose
concrete `docs/SOURCES.md` additions for the Hold'Em/hand-ranking references the spec relies on
(as paste-ready text inside the spec's documentation-updates section), since updating that file
is a downstream in-repo step, not your job.

## 6. Doctrine & constraints (honor all)

- `docs/FOUNDATIONS.md` is the constitution — every product-behavior decision must satisfy its
  §11 universal invariants and clear its §12 stop conditions; a genuine divergence requires an
  accepted ADR superseding the affected principle first, never designing against it silently.
- Authority order: foundation docs govern area docs govern specs govern tickets; if execution
  would conflict with architecture or foundation, execution is wrong.
- `engine-core` stays generic and **noun-free** — no `card`, `deck`, `hand`, `pot`, `seat`-role
  nouns; River Ledger's typed mechanic nouns live in `games/river_ledger`, shared helpers only
  in `game-stdlib` via the mechanic atlas (and **no** promotion is authorized for Gate 15).
- **TypeScript never decides legality.** Legal actions, validation, effects, views, betting
  legality, showdown evaluation, and bot decisions all come from Rust/WASM. The React shell
  renders seat order, active/pending seats, previews, and outcome breakdowns but computes none
  of them.
- **No YAML and no DSL.** Static data is typed content/parameters/metadata only — never
  selectors, conditions, or triggers. Betting/payment/evaluation logic is typed Rust, not data
  formulas.
- **Determinism:** shuffle, replay, hashes, RNG, serialization order, and traces stay
  deterministic; reuse trace schema v1 (no migration without ADR).
- **No hidden-information leaks** into payloads, DOM, storage, logs, effect logs, bot
  explanations, candidate rankings, or replay exports — pairwise across all seats per
  MULTI-SEAT-AND-SURFACE-CONTRACT §6 and ADR 0004. Hole cards, burn cards, and deck order never
  reach an unauthorized viewer; legitimate inference from public betting is allowed and must be
  distinguished from payload leakage.
- **No MCTS / ISMCTS / Monte Carlo / ML / RL bots.** L0–L2 policy/belief heuristics only.
- **No casino/real-money features or trade dress;** original presentation, public-domain rules
  family only, no copied prose or assets (`IP-POLICY.md`).
- **Never delete or weaken tests to get green** — the spec's acceptance evidence follows the
  AGENT-DISCIPLINE §4 failing-test protocol.
- The spec must **not** decompose into ticket files (that is the later `/spec-to-tickets`
  step) and must **not** itself edit any repo file — it is a standalone planning document.

## 7. Deliverable specification

Produce exactly **one downloadable markdown document**:

- **`specs/gate-15-river-ledger-texas-holdem-base.md`** — **NEW** file (no existing file is
  replaced). Follow the repo's canonical **12-section spec format** defined in `specs/README.md`
  ("Spec format"), using `archive/specs/gate-10-poker-lite-betting-showdown.md` and
  `archive/specs/gate-0-repository-skeleton.md` as the structural exemplars:

  1. **Header** — Spec ID, stage (Stage 15 / public scaling phase), gate (Gate 15), status
     (`Planned`), date, owner, authority order.
  2. **Objective** — what Gate 15 achieves, sourced from ROADMAP §15.
  3. **Scope** — in scope / out of scope / not allowed; carry ROADMAP §15's "Not allowed" list
     verbatim and the §3 locked scope (3–6 seats, fixed-limit capped-raise, split pots in,
     all-in/side-pots out → 15.1).
  4. **Deliverables** — concrete artifact/file tree grounded in ARCHITECTURE.md and the code
     seams: the `games/river_ledger` crate module layout; the filled `templates/GAME-*.md` +
     COMPETENT-PLAYER / BOT-STRATEGY-EVIDENCE-PACK / PRIMITIVE-PRESSURE-LEDGER /
     GAME-IMPLEMENTATION-ADMISSION / PUBLIC-RELEASE-CHECKLIST instances; registration edits;
     web-shell exposure.
  5. **Work breakdown** — bounded items in dependency order, each a **candidate AGENT-TASK**
     (per `templates/AGENT-TASK.md` / AGENT-DISCIPLINE §2). NOT ticket files.
  6. **Exit criteria** — mapped row-for-row to ROADMAP §15's exit list.
  7. **Acceptance evidence** — tests/traces/benchmarks/reviews per
     TESTING-REPLAY-BENCHMARKING.md (rule, golden-trace, property, simulation, replay/hash,
     serialization, N-seat pairwise no-leak, bot-legality, benchmark) and the per-game CLI
     verification commands.
  8. **FOUNDATIONS & boundary alignment** — principles engaged, with stance and rationale; keep
     §12 stop conditions clear; record the MECHANIC-ATLAS §9A pressure stance (game-local, no
     promotion) explicitly.
  9. **Forbidden changes** — gate-specific prohibitions (carry §6 doctrine).
  10. **Documentation updates required** — including the `specs/README.md` status flip
      (Order 5 → `Planned`/`Done` lifecycle), the `apps/web/README.md` catalog surfaces (intro
      list, Shell Surface renderer list, `smoke:e2e` list, enforced by
      `scripts/check-catalog-docs.mjs`), the MECHANIC-ATLAS arming entry, and the proposed
      `docs/SOURCES.md` Hold'Em/hand-ranking additions as paste-ready text.
  11. **Sequencing** — predecessor (Infra A–D, `Done`) and successor (Gate 15.1 side pots);
      admission rule.
  12. **Assumptions** — one-line-correctable.

Use explicit `not applicable` rows over silent omissions wherever a section item does not apply.

**Locked / no-questions instruction:** Produce the deliverable directly as a downloadable
markdown document. Do not interview, do not ask clarifying questions — the requirements above
are final. If a genuine contradiction makes a requirement impossible, state it in the
deliverable and proceed with the most faithful interpretation.

## 8. Self-check (run against your own output before returning)

- The deliverable is exactly one NEW file, `specs/gate-15-river-ledger-texas-holdem-base.md`,
  in the repo's 12-section spec format — no other files produced, no repo files edited.
- The spec **confirms-and-documents** Gate 15 as the next unit with the evidence that fixed it
  (Phase 0 + Infra A–D `Done`; Gate 15 lowest non-`Done`; promotion-debt register `_None_`) —
  it does not re-decide "what's next."
- Every §3 locked decision appears in the spec as a committed decision (3–6 seats, fixed-limit
  capped-raise, split pots in, all-in/side-pots deferred to 15.1, 21-subset evaluator, Rust
  showdown explanation, bots L0–L2, River Ledger naming/IP posture, coexists with `poker_lite`).
- No constraint in §6 is weakened: engine-core stays noun-free; no TS legality; determinism and
  trace-schema-v1 reuse preserved; N-player pairwise no-leak covered across payloads/DOM/
  storage/logs/effects/bot-explanations/replay-exports; no MCTS/ISMCTS/ML/RL; no casino trade
  dress; no DSL/YAML.
- Exit criteria map row-for-row to ROADMAP §15; acceptance evidence maps to
  TESTING-REPLAY-BENCHMARKING.md classes; documentation-updates names the `specs/README.md`
  flip and `apps/web/README.md` catalog surfaces.
- The work-breakdown names every real registration touchpoint (workspace `Cargo.toml`,
  `wasm-api` `RegisteredGame`/`list_games`/`MatchRecord`, the four `tools/*` dispatchers, web
  catalog) and stays at candidate-AGENT-TASK granularity — no ticket decomposition.
- No new doctrine silently amends an accepted ADR (0004, 0007) or any foundation doc; any
  proposed schema/promotion change is flagged as requiring an ADR, not assumed.
- Every external claim (rules, hand ranking, evaluator, prior art) is cited.
- All files named in §2 resolve at commit `47e202a`.

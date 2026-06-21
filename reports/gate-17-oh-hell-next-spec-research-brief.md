# Deep-research brief — author the **Gate 17 (Oh Hell) implementation spec** for Rulepath

> **You are ChatGPT-Pro, the deep researcher (Session 2).** This brief is final and
> self-contained. The requirements below were settled in a prior session with full repository
> access. **Do not interview, do not ask clarifying questions, do not re-decide what comes
> next.** Produce the deliverable directly as a downloadable markdown document. If a genuine
> contradiction makes a requirement impossible, state it in the deliverable and proceed with
> the most faithful interpretation.

---

## 1. Context

The uploaded file `manifest_2026-06-21_8ff22d9.txt` is the exact path inventory of the
`joeloverbeck/rulepath` repository — a Rust-first, rule-enforcing, replayable, testable
card/board-game platform where **Rust owns all behavior and TypeScript/React present only**.
The foundation docs are an ordered, layered authority indexed by `docs/README.md`:
`FOUNDATIONS.md` (the constitution) → `ARCHITECTURE.md` → `ENGINE-GAME-DATA-BOUNDARY.md` →
the area docs → `ROADMAP.md`; earlier documents govern later ones, and accepted ADRs supersede
them only by explicitly naming the affected sections.

**Fetch every file you read from commit `8ff22d9` (verified `HEAD`); the uploaded manifest is
that exact tree (`git ls-tree -r --name-only HEAD`).** If any source you consult cites a
different "commit of record," note the divergence and use `8ff22d9`, not the cited string.

**What just shipped (the delta baseline).** Gate 16 — **Briar Circuit** (Rulepath's classic
four-seat Hearts implementation) flipped to `Done` on 2026-06-21 per `specs/README.md`. It was
itself produced from a prior deep-research brief in this same series
(`reports/gate-16-hearts-next-spec-research-brief.md`, now shipped). The mechanic-atlas
open-promotion-debt register (`docs/MECHANIC-ATLAS.md` §10A) is **empty** as of the Gate 15.1
closeout and was not reopened by Gate 16. `games/briar_circuit` and `games/plain_tricks` are now
the two live official trick-taking implementations; `games/river_ledger` is the live N-seat
(3–6) hidden-information exemplar. This brief commissions the **next** spec on the ladder —
**Gate 17 — Oh Hell** — it is **not** a revisit of Briar Circuit, Plain Tricks, or River Ledger,
and **must not re-recommend any already-shipped work on those games as if it were missing**.
Build on them as exemplars and comparison baselines; improve only where this gate genuinely
requires it.

---

## 2. Read in full (authority order)

Read these in this order before producing. The repository tree in the manifest is ground truth;
read the **entire `docs/**` tree** (the explicit floor for this task), plus the planning and
delta-baseline artifacts below. Each line states why the file is load-bearing *for the Gate 17
spec*.

**Tier 1 — constitution & authority**
- `docs/README.md` — the authority order and the layering rule every section of the spec must respect.
- `docs/FOUNDATIONS.md` — the constitution: product priority, §11 universal invariants, §12 stop conditions, §13 ADR triggers; every spec decision must satisfy these.

**Tier 2 — area law the gate engages (read the whole `docs/**`; these are the load-bearing ones)**
- `docs/ARCHITECTURE.md` — workspace shape, dependency direction, action/view/effect/replay/determinism model the new game crate must fit.
- `docs/ENGINE-GAME-DATA-BOUNDARY.md` — the exact `engine-core` / `game-stdlib` / `games/*` / static-data boundary; trick-taking and bidding nouns (suit, rank, trick, hand, bid, contract, trump) stay in the game crate, never `engine-core`.
- `docs/OFFICIAL-GAME-CONTRACT.md` — what makes a game official: §3 requirements-first workflow, §4 rules-research/source-notes format, §5 original RULES.md prose, §6 rule-coverage matrix, §10 UI-exposure obligations, §11 required trace set, §12 acceptance checklist the spec must map deliverables to.
- `docs/MECHANIC-ATLAS.md` — primitive-pressure law and **the single most important document for this gate's hard gate**: §4 first/second/**third-use** rule, §5 hard-gate decision options (reuse / promote-narrow / defer-reject / ADR), §5A promotion-conformance lifecycle (back-port-or-record-debt), §9A armed interlock row **"Hearts, Oh Hell, and Spades"** ("Reopen trick-taking before Gate 17 Oh Hell as the third close use"), §10 atlas-table rows `follow-suit legality`, `trick resolution / led-suit comparator`, `trick-winner-leads turn order`, `deal rotation / trick-round redeal` (all `repeated-shape candidate`; each explicitly names **"Gate 17 Oh Hell is the third-use hard-gate trigger"**), and §10A empty debt register.
- `docs/AI-BOTS.md` — bot law: §2 levels (L0 random required; L1 rule-informed; L2 authored; L3 deterministic search **perfect-info only**), §3 v1/v2 exclusions (no MCTS/ISMCTS/Monte Carlo/ML/RL), §4/§4A N-player imperfect-information information boundary (allowed public/own-private/inference/belief; forbidden hidden cards/commitments/sampled worlds), the competent-player → strategy-evidence-pack gate for any L2, §12 hidden-info bot-policy fields. **Bidding bots are first-use** — there is no prior numeric bid/contract bot in the repo.
- `docs/UI-INTERACTION.md` — legal-only interaction, Rust-safe previews, effect-driven animation, replay UI, accessibility — the GAME-UI deliverable's law; the bidding interface and changing-hand-size table are new presentation surfaces.
- `docs/TESTING-REPLAY-BENCHMARKING.md` — §1 test taxonomy, §3 golden-trace Trace Schema v1 obligations, §4 replay determinism, §5 determinism hazards, §8 visibility/no-leak, **§8.1 N-seat no-leak taxonomy** and **§8.2 (3+ seat games must include public-observer + ≥2 seat-private exports; 4+ seats exercise every seat-viewer in CI or the spec documents a sampled matrix — directly engaged because Oh Hell is variable-N up to 7)**, §12 mechanic-primitive third-use/back-port tests, §15 provisional budgets, §17 CI expectations.
- `docs/MULTI-SEAT-AND-SURFACE-CONTRACT.md` — N-seat obligations a **variable-N (3–7)** game must honor: §2 seat-range declaration (min/max/default/supported-set/labels/diagnostics — the variable-range case, wider than any shipped game), §4 turn-order/dealer-rotation model, §5 viewer matrix, §6 pairwise no-leak matrix, §7 public-observer rules, §8 surface budgets, §11 outcome/breakdown model, §12 trace/view-hash expectations, §13 seat-keyed simulator summaries, §14 Gate 15+ spec/ticket minimums.
- `docs/ROADMAP.md` — §1 crosswalk and **§15 "Gate 17: Oh Hell"** (purpose: "generalize trick-taking from fixed four seats to variable N with contract/bid pressure"; the exit list — official seat range, dealer rotation, changing hand size, bidding order, last-bidder constraint, trick play, scoring, terminal standings, by-seat-count simulation/benchmarks, and **"trick-taking and bidding helper decisions are resolved through the primitive pressure process"**). The prescriptive ladder law this spec realizes.
- `docs/IP-POLICY.md` — naming/original-presentation rules for a public-domain game: §5 common-vs-neutral names (whether "Oh Hell" may be used or must be coined-neutral), §4/§11 source-notes checklist, §6 original prose/assets, §12 release checklist.
- `docs/AGENT-DISCIPLINE.md` — coding-agent law: bounded tasks, forbidden changes, failing-test protocol the spec's work breakdown must respect.
- `docs/SOURCES.md` — researched bibliography + Rulepath lessons; the spec's GAME-SOURCES deliverable extends this convention.
- `docs/TRACE-SCHEMA-v1.md` — the golden-trace schema fields the Gate 17 traces must populate (load-bearing because Oh Hell adds variable-N private-hand redacted traces plus bidding-phase traces).
- `docs/WASM-CLIENT-BOUNDARY.md` — Rust/WASM-to-browser contract, operation groups, replay safety, dev-panel whitelist the web-exposed game must obey.
- `docs/adr/0004-hidden-info-replay-export-taxonomy.md` — the accepted hidden-information replay-export taxonomy Oh Hell's redacted exports must satisfy.
- `docs/adr/0007-next-public-scaling-phase-and-gate-p-tail.md` — the accepted ADR that admits the public scaling phase (Gates 15–23) and pins Gate 17's place in it.
- (Read the remaining `docs/**` — `archival-workflow.md`, ADRs `0001`/`0002`/`0003`/`0005`/`0006`, `adr/ADR-TEMPLATE.md` — as the explicit `docs/**` floor; they shape benchmark gating, archival, and ADR conventions the spec touches at closeout. `adr/0006-blackjack-lite-roadmap-placement.md` in particular: do not accidentally re-open it.)

**Tier 3 — planning artifacts**
- `specs/README.md` — the living spec index and progress tracker; carries the **determination evidence** (active-epoch table: Gate 16 `Done` 2026-06-21, Gate 17 the lowest non-`Done` row at Order 8, §10A debt empty), the 12-section **spec format** your deliverable must follow, and the author workflow (`/reassess-spec` → `/spec-to-tickets` happens *after* the spec, not in it).
- `templates/**` — the per-game template set the spec's deliverables instantiate: `GAME-SOURCES.md`, `GAME-RULES.md`, `GAME-RULE-COVERAGE.md`, `GAME-MECHANICS.md`, `GAME-IMPLEMENTATION-ADMISSION.md`, `GAME-HOW-TO-PLAY.md`, `COMPETENT-PLAYER.md`, `BOT-STRATEGY-EVIDENCE-PACK.md`, `GAME-AI.md`, `GAME-UI.md`, `GAME-BENCHMARKS.md`, `PRIMITIVE-PRESSURE-LEDGER.md`, `PUBLIC-RELEASE-CHECKLIST.md`, `AGENT-TASK.md`, `README.md`. The spec's Deliverables section names which of these Oh Hell fills.

**Tier 4 — delta baselines (sibling specs to mirror, not rebuild)**
- `archive/specs/gate-16-briar-circuit-trick-taking.md` — the **most recent** sibling and the closest structural template (Header + 12 sections + appendices B–E + Outcome). It is the **second** official trick-taking spec and the direct comparison baseline for the third-use ledger reopening. Mirror its structure; do not rebuild Briar Circuit.
- `archive/specs/gate-10-1-plain-tricks-trick-taking-proof.md` — the **first** official trick-taking spec; the other comparison baseline for the ledger reopening (note its implementation appendices).
- `archive/specs/gate-15-river-ledger-texas-holdem-base.md` — the widest-seat-range shipped game (3–6 seats); the structural template for **variable seat-range declaration**, viewer matrix, pairwise no-leak matrix, seat-keyed summaries, golden-trace set, and benchmark-by-seat-count sections. This is the closest variable-N precedent for Oh Hell's 3–7 range.
- `archive/specs/gate-0-repository-skeleton.md` — the canonical 12-section spec example referenced by `specs/README.md`.

### Code seams to inspect directly (*inspect in the repo, not read-fully; not pasted here*)
- `games/plain_tricks/src/**` and `games/plain_tricks/docs/PRIMITIVE-PRESSURE-LEDGER.md` — the **first** trick-taking implementation. Inspect how follow-suit legality, led-suit comparator / trick resolution, trick-winner-leads turn order, deterministic deal rotation, and owner-only hand visibility are structured (`src/actions.rs`, `rules.rs`, `state.rs`, `setup.rs`, `visibility.rs`, `bots.rs`).
- `games/briar_circuit/src/**` and `games/briar_circuit/docs/PRIMITIVE-PRESSURE-LEDGER.md` — the **second** trick-taking implementation and the ledger that explicitly defers extraction to "Gate 17 [which] will decide the third-use hard gate with two real implementations available for comparison." This ledger is the entry your reopening extends. Inspect `src/{actions,rules,state,setup,scoring,visibility,cards,bots}.rs`.
- `games/river_ledger/src/**` — the **variable seat-range** + N-seat private-hand no-leak + Rust-authored outcome exemplar (`src/setup.rs` for 3–6 seat-range setup/validation/diagnostics; `src/visibility.rs` for per-seat pairwise no-leak; showdown/outcome projection). This is the pattern Oh Hell's 3–7 declaration, per-seat no-leak, and per-seat outcome breakdown follow.
- **Bidding has no prior art.** Confirm by inspection: `secret_draft` (sequential/simultaneous secret commitments), `masked_claims` (claim/reveal), and `high_card_duel` (face-down commitment) are the nearest commitment shapes, but **none is a numeric bid/contract-vs-result mechanic**. Treat bidding/contracts as a **genuinely first official use** → local, with a **new** primitive-pressure ledger entry.
- New-crate registration seams (same pattern Briar Circuit / River Ledger followed): `crates/wasm-api/src/lib.rs` (game import + dispatch) and `crates/wasm-api/src/constants.rs` (game id + display-name constants); `tools/simulate/src/main.rs` (game const + match arm for seat-variable setup + bot dispatch); `tools/{replay-check,fixture-check,rule-coverage}/src/main.rs` (generic — confirm whether any game-specific registration is needed); `apps/web` catalog + `apps/web/README.md` (intro catalog list, Shell Surface renderer list, Smoke Layers `smoke:e2e` list, enforced by `scripts/check-catalog-docs.mjs`).

---

## 3. Settled intentions (final — do not re-open)

These decisions were locked in Session 1. They pre-empt every clarifying question.

1. **The next spec is Gate 17 — Oh Hell.** This determination is **settled**; your job is to
   *confirm-and-document* it (citing the evidence below) and then **write the spec** — not to
   re-decide what comes next, and not to propose a River Ledger / Briar Circuit continuation or a
   maintenance detour. The evidence that fixed it, which the spec's Objective/Sequencing sections
   must cite:
   - Gate 16 — Briar Circuit (classic four-seat Hearts) is `Done` (2026-06-21) per
     `specs/README.md`.
   - `docs/MECHANIC-ATLAS.md` §10A open-promotion-debt register is **empty** ("Current debt:
     _None_") — so the "close open promotion debt before the next mechanic-ladder gate"
     interlock does **not** block, and no separate debt-closure spec is needed first.
   - Gate 17 — Oh Hell is the **lowest non-`Done`** unit on the `specs/README.md` active-epoch
     tracker (Order 8), and its predecessor (Gate 16) is `Done`.
   - `docs/ROADMAP.md` §15 admits Gate 17 as ladder law.

2. **This is the same brief series as the shipped Gate 16 cycle — do not re-emit shipped work.**
   `reports/gate-16-hearts-next-spec-research-brief.md` commissioned Briar Circuit, which shipped.
   Treat Briar Circuit, Plain Tricks, and River Ledger as **implemented exemplars and comparison
   baselines**, not as gaps to fill.

3. **Variant: classic Oh Hell; family locked, exact parameters research-pinned by you.** The spec
   locks the variant **family**: variable **3–7 declared seats**, dealer rotation each hand,
   **changing hand size** across the deal schedule, a **bidding/contract phase** (bid order, a
   **last-bidder constraint** — the "hook" / "screw-the-dealer" rule that bids may not sum to the
   number of tricks), trump determination, trick play under follow-suit, and **exact-bid scoring**
   (you make exactly your bid or you score the penalty). **You must deep-research the canonical
   Oh Hell ruleset and its common house variants and research-PIN the exact parameters inside the
   spec, with sources**, choosing one canonical Rulepath variant and documenting any deliberate
   deviation in the GAME-SOURCES-style notes the spec calls for. At minimum pin:
   - the **deal schedule** (e.g. ascending 1→max then descending, fixed N, or up-and-down) and how
     max hand size is derived from seat count and the 52-card deck;
   - **trump determination** (turn-up after the deal vs. fixed rotation vs. no-trump hands);
   - the **bidding order and the last-bidder "hook"** rule precisely;
   - the **scoring formula** (e.g. 10 + tricks on an exact bid, vs. 1-per-trick-plus-bonus
     variants; over/under penalties);
   - **tie-break** and **game-end** (fixed number of hands vs. points target);
   - first-leader and trick-play edge rules.
   `ROADMAP.md` §15 Gate 17 names "bids/contracts, changing hand size" and "last-bidder
   constraint" explicitly, so the full variable-N bidding game — not a stripped slice — is the
   target.

4. **The third-use trick-taking hard gate is resolved INSIDE this spec** (one deliverable, not a
   separate interlock spec). Trick-taking has now appeared in **two** official games:
   `plain_tricks` (Gate 10.1, first use) and `briar_circuit` (Gate 16, second use). Oh Hell is the
   **third** close use, which **fires the `docs/MECHANIC-ATLAS.md` §4 third-use hard gate** for
   the `follow-suit legality`, `trick resolution / led-suit comparator`, `trick-winner-leads turn
   order`, and `deal rotation / trick-round redeal` shapes (each §10 row already names Gate 17 as
   the trigger; the `briar_circuit` ledger names Gate 17 as the deciding gate). The spec must carry
   this hard-gate resolution as a **gating work-breakdown item** (it must resolve before
   trick-taking behavior is implemented) **and** as a §8 FOUNDATIONS-&-boundary stance, using the
   `templates/PRIMITIVE-PRESSURE-LEDGER.md` shape and reopening/extending the existing
   `briar_circuit` ledger entry as a three-way `plain_tricks` ↔ `briar_circuit` ↔ Oh Hell
   comparison.

5. **The extraction OUTCOME is delegated to you as a bounded sub-decision** (the determination and
   scope stay locked; only this named, optioned sub-decision is handed down). The spec must
   **perform the full third-use ledger analysis** and **decide exactly one** of the
   `docs/MECHANIC-ATLAS.md` §5 options — **reuse an existing promoted primitive / promote a narrow
   typed helper / explicitly defer-reject / escalate to ADR** — with a **required, justified
   recommendation**. Hard constraints on whichever you choose:
   - If you **promote** a narrow `game-stdlib` trick-taking helper, the spec must satisfy §5A: name
     the helper's narrow behavior-free boundary (e.g. led-suit comparison over typed cards, or
     deterministic deal rotation) with explicit limits and no hidden policy, and **either** plan
     the in-gate back-port of `plain_tricks` and `briar_circuit` **or** record explicit §10A
     promotion debt (named primitive, affected games, deferral reason, behavior-preservation risk,
     closure gate, closing evidence) — never a half-promotion. `ROADMAP.md`'s Gate 17 interlock
     note says "trick-taking helper promotion likely," so promotion is a live possibility, but it
     must be *earned* by the analysis, not assumed.
   - If you **defer/reject** (as the `briar_circuit` ledger did at second use, because the
     behavior-bearing exceptions — passing, hearts-broken, point-card restrictions, negative
     scoring, and now bidding/contracts/changing-hand-size/variable-N — dominate the small shared
     core), record the third-use defer rationale, risk notes, and the next review trigger; §10A
     stays empty.
   - **Regardless of outcome**, introduce **no** `card`, `suit`, `rank`, `hand`, `trick`, `trump`,
     `bid`, or `contract` noun into `engine-core`.

6. **Bidding/contracts is a genuinely first official use → keep local, with a NEW ledger entry.**
   No prior official game has a numeric bid/contract-vs-result mechanic (see §2 code seams). Per
   `docs/MECHANIC-ATLAS.md` §4 first-use rule, implement bidding locally and **record a new
   first-use `local-only` primitive-pressure ledger entry** (bid order, last-bidder hook, exact-bid
   scoring) in the spec and the atlas-update list — do **not** generalize it, and do **not** encode
   bid legality or scoring formulas in static data.

7. `assumption:` the deliverable is the **spec only** — it is *not* decomposed into `tickets/`
   AGENT-TASK packets. Ticket decomposition happens separately afterward via the repo's
   `/reassess-spec` then `/spec-to-tickets` workflow (`specs/README.md` "Workflow"). The spec's
   work breakdown should enumerate **candidate** AGENT-TASK items with dependency order, as the
   sibling specs do, without writing the tickets themselves.

8. `assumption:` **Neutral game name — bounded delegation to you.** Rulepath ships original,
   evocative neutral names rather than the source game's name (River Ledger ← Texas Hold'Em, Crest
   Ledger ← poker, Plain Tricks, Masked Claims, Flood Watch, **Briar Circuit ← Hearts**). **Coin an
   original, IP-safe neutral name for this Oh Hell implementation** consistent with that catalog
   convention, keep "Oh Hell" only as the rules-family label in source/IP notes, and **derive the
   game module id (snake_case) and the spec filename slug from it.** Per `docs/IP-POLICY.md` §5,
   keeping "Oh Hell" is *permissible*, but the established convention is to coin a neutral name, so
   coin one and use it, documenting the naming rationale in the spec's source/IP notes (and noting,
   as Briar Circuit did, that the coinage does not replace the human IP review IP-POLICY requires).
   The spec filename is `specs/gate-17-<neutral-slug>-oh-hell-bidding-trick-taking.md`; if you
   judge a neutral coinage genuinely unjustified and keep "Oh Hell," use
   `specs/gate-17-oh-hell-bidding-trick-taking.md`.

---

## 4. The task

Produce the **Gate 17 (Oh Hell) implementation spec** — a **new** roadmap-gate spec — that turns
`docs/ROADMAP.md` §15 "Gate 17: Oh Hell" into one concrete, reviewable, foundation-aligned plan.
This is a **new-spec** deliverable. The spec must (a) confirm-and-document the Gate 17
determination with the evidence in §3.1; (b) lock the classic Oh Hell variant family and
research-pin its exact parameters; (c) define the new `games/<neutral_id>` crate, its filled
official-game documents, and its registration across tools/WASM/web-catalog surfaces; (d) scope
the **variable 3–7-seat** private-hand no-leak obligations to the multi-seat contract and the §8.1
N-seat / §8.2 export-coverage taxonomy; (e) **resolve the third-use trick-taking primitive-pressure
hard gate** with a justified reuse/promote/defer/ADR decision (delegated per §3.5) and **record
bidding as a new first-use local mechanic** (§3.6); and (f) map every deliverable to the
OFFICIAL-GAME-CONTRACT, MULTI-SEAT-AND-SURFACE-CONTRACT, TESTING, and AI-BOTS obligations,
following the `specs/README.md` 12-section spec format and mirroring the structure of
`archive/specs/gate-16-briar-circuit-trick-taking.md`, `archive/specs/gate-10-1-plain-tricks-trick-taking-proof.md`,
and `archive/specs/gate-15-river-ledger-texas-holdem-base.md` (for the variable-N sections).

---

## 5. Exploration & online-research mandate

Explore the repository as deeply as needed beyond the files listed above. **Research online as
deeply as needed** — the canonical Oh Hell ruleset and its common regional/house variants (deal
schedules: ascending-then-descending vs. fixed vs. up-and-down; trump-determination methods;
bidding order; the precise last-bidder "hook"/"screw-the-dealer" constraint; exact-bid scoring
formulas and over/under penalties; game-end thresholds and tie-breaks; first-trick/first-leader
edge rules), public-domain rules sources suitable for an original prose summary, prior open-source
trick-taking-with-bidding engine implementations and how they model bid legality, the last-bidder
constraint, trick resolution, and contract-vs-result scoring, and research or write-ups on Oh Hell
**bidding and play strategy** for a competent-player analysis and any L1/L2 bot policy (**without
proposing any MCTS/ISMCTS/Monte Carlo/ML/RL approach — those are forbidden in public v1/v2**), and
accessibility/UX prior art for presenting a variable-seat hidden-hand bidding-then-trick game.
Cite sources for any external claim that shapes a decision in the spec (variant parameters,
naming/IP rationale, strategy posture, the extraction decision). The deep research is **your** job;
do it thoroughly.

---

## 6. Doctrine & constraints (honor these in every spec decision)

- `docs/FOUNDATIONS.md` is the constitution — every product-behavior decision must satisfy its
  §11 universal invariants and clear its §12 stop conditions; a genuine divergence requires an
  accepted ADR superseding the affected principle first ("supersede only by accepted ADR"), never
  designing against it silently. The spec is **subordinate** to the foundation set and must not
  redefine or override any foundation contract.
- Authority order: foundation docs govern area docs govern specs govern tickets. Where the spec and
  a foundation document disagree, the foundation document wins.
- `engine-core` stays generic and **noun-free** — no `card`, `deck`, `hand`, `suit`, `rank`,
  `trick`, `trump`, `bid`, `contract`, `board`, etc.; typed mechanic nouns belong in `games/*`
  first, shared helpers in `game-stdlib` only via the mechanic atlas. The §3.5 third-use trick-
  taking decision may promote a **narrow, behavior-free** `game-stdlib` helper **only** if the
  analysis earns it and §5A back-port/debt is honored; bidding (§3.6) stays local first-use.
- **TypeScript never decides legality.** Legal actions (legal bids, legal cards), validation,
  effects, views, previews, and bot decisions all come from Rust/WASM. The browser must not filter
  bids or cards by suit/legality.
- **No YAML and no DSL without an accepted ADR.** Static data is typed content / parameters /
  metadata only — never selectors, conditions, or triggers; no scoring, follow-suit, or
  bid-legality formulas encoded in data.
- **Determinism**: deterministic shuffle/deal across the changing-hand-size schedule, replay,
  hashes, serialization order, and traces stay deterministic (or are explicitly migrated with trace
  notes).
- **No hidden-information leaks**: variable-N (up to 7) private hands, bids before reveal if the
  variant hides them, and any unrevealed deck tail must not leak into payloads, action trees,
  previews, DOM, storage, logs, effect logs, bot explanations, candidate rankings, or replay
  exports — proven by the §8.1 N-seat pairwise no-leak taxonomy and the §8.2 export-coverage rule
  for 3+/4+ seats.
- **No MCTS / ISMCTS / Monte Carlo / ML / RL bots** in public v1/v2. L0 random-legal is required;
  any L2 authored bidding/play policy needs a competent-player analysis + strategy-evidence pack
  first.
- **No casino/real-money/trade-dress mimicry; original prose and assets only** (`docs/IP-POLICY.md`).
- **Never delete or weaken tests to get green** — follow the failing-test protocol
  (`docs/AGENT-DISCIPLINE.md` §4).
- The spec is **authored, not decomposed** (§3.7): enumerate candidate AGENT-TASKs; do not write
  tickets.

---

## 7. Deliverable specification

Produce exactly **one downloadable markdown document**:

- **`specs/gate-17-<neutral-slug>-oh-hell-bidding-trick-taking.md`** — a **new** file (not a
  replacement). `<neutral-slug>` is derived from the neutral name you coin (§3.8); fall back to
  `specs/gate-17-oh-hell-bidding-trick-taking.md` only if you keep "Oh Hell."

It MUST follow the **12-section spec format** defined in `specs/README.md` ("Spec format"),
mirroring `archive/specs/gate-16-briar-circuit-trick-taking.md`:
1. Header (Spec ID, stage, gate, status `Planned`/`Not started`, date, owner, authority order;
   plus the game-identity fields the sibling specs carry — internal game id, public display name,
   rules-family label, variant id, trace rules version, data/manifest version, browser-impl-
   required flag, **official seat declaration for the variable 3–7 range** (min/max/default/
   supported set/labels), public-observer stance, bot floor, kernel stance, primitive stance,
   delivery posture);
2. Objective (sourced from ROADMAP §15 Gate 17; cite the §3.1 determination evidence; state the
   **third-use** trick-taking posture and the **first-use** bidding posture);
3. Scope (in / out / **not allowed** — carry ROADMAP §15's prohibitions and the public-scaling-
   phase "not allowed" list verbatim);
4. Deliverables (the new `games/<neutral_id>` crate tree; the filled official-game documents from
   `templates/**`; registration across `simulate`/`replay-check`/`fixture-check`/`rule-coverage`,
   WASM, and the `apps/web` catalog);
5. Work breakdown (bounded **candidate** AGENT-TASK items with dependency order — **including the
   third-use trick-taking ledger-resolution item as a gating prerequisite** and the new bidding
   first-use ledger item);
6. Exit criteria (mapped row-for-row to ROADMAP §15 Gate 17's exit list — official seat range,
   dealer rotation, changing hand size, bidding order + last-bidder constraint, trick play,
   scoring, terminal standings, by-seat-count simulations/benchmarks, and the trick-taking/bidding
   primitive-pressure resolution);
7. Acceptance evidence (command suite; the test-taxonomy table; the **pairwise no-leak matrix**
   scaled to the variable seat range with the §8.2 export-coverage / sampled-matrix statement for
   up to 7 seats; the golden-trace minimum set per OFFICIAL-GAME-CONTRACT §11 / TESTING §6 —
   including bidding-phase and variable-hand-size traces; benchmark expectations **by seat count**);
8. FOUNDATIONS & boundary alignment (principles engaged; the **third-use trick-taking primitive-
   pressure analysis and decision** per §3.5, and the **first-use bidding** local stance per §3.6;
   §12 stop conditions);
9. Forbidden changes (gate-specific prohibitions);
10. Documentation updates required (the `specs/README.md` status flip; `docs/SOURCES.md`;
    **`docs/MECHANIC-ATLAS.md`** — the §10 trick-taking rows updated to record the third-use
    decision, a new bidding/contract first-use row, and §10A updated if promotion creates debt;
    game-local docs; **`apps/web/README.md`** intro catalog list + Shell Surface renderer list +
    Smoke Layers `smoke:e2e` list, per OFFICIAL-GAME-CONTRACT §10/§12 and `scripts/check-catalog-docs.mjs`);
11. Sequencing (predecessor Gate 16 `Done`; successor Gate 18 Spades; admission rule);
12. Assumptions (one-line-correctable).

Use explicit `not applicable` rows over silent omissions. You may include implementation
appendices (core rules, bidding/last-bidder model, bot policy, replay/export model, WASM specifics,
benchmark operations, sources) as the sibling specs do, but keep them inside the single spec file.

> Produce the deliverable directly as a downloadable markdown document. Do not interview, do not
> ask clarifying questions — the requirements above are final. If a genuine contradiction makes a
> requirement impossible, state it in the deliverable and proceed with the most faithful
> interpretation.

---

## 8. Self-check (run against your own output before returning)

- [ ] The spec **confirms-and-documents** Gate 17 as next with the §3.1 evidence (Gate 16 `Done`;
      §10A empty; lowest non-`Done` Order 8; ROADMAP §15 admits it); it does **not** re-open
      "what's next" or propose a different gate / River Ledger / Briar Circuit continuation.
- [ ] It locks the **classic Oh Hell variant family** and **research-pins** exact parameters with
      sources (deal schedule + max hand size derivation, trump determination, bidding order, the
      last-bidder "hook" constraint, exact-bid scoring + over/under penalties, game-end + tie-break).
- [ ] It coins an original, IP-safe **neutral name** (or justifies keeping "Oh Hell") and derives
      the module id and spec filename slug from it.
- [ ] It **fires and resolves the third-use trick-taking hard gate** (`plain_tricks` ↔
      `briar_circuit` ↔ Oh Hell) with a justified **reuse / promote-narrow / defer-reject / ADR**
      decision per §3.5; if it promotes, it honors §5A (narrow behavior-free boundary + in-gate
      back-port **or** explicit §10A debt); it adds **no** `engine-core` card/suit/rank/hand/trick/
      trump/bid/contract noun.
- [ ] It records **bidding/contracts as a NEW first-use `local-only`** primitive-pressure entry
      (§3.6); no bid legality or scoring formula is encoded in static data.
- [ ] It scopes **variable 3–7-seat private-hand no-leak** to the MULTI-SEAT §6 / TESTING §8.1
      pairwise taxonomy and the §8.2 export-coverage / sampled-matrix rule, across every surface
      (views, action trees, previews, diagnostics, effects, bot explanations, candidate rankings,
      replay exports, DOM, storage, logs, accessibility text, test IDs).
- [ ] It declares the **variable seat range** per MULTI-SEAT §2 (min/max/default/supported set/
      labels/diagnostics) and uses **seat-keyed** simulator summaries (§13) and **by-seat-count**
      benchmarks.
- [ ] It requires an **L0 random-legal bot** and gates any L2 bidding/play policy behind a
      competent-player analysis + strategy-evidence pack; it forbids MCTS/ISMCTS/Monte Carlo/ML/RL.
- [ ] Every deliverable maps to OFFICIAL-GAME-CONTRACT §3/§5/§6/§10/§11/§12 and the `templates/**`
      set; the **12-section spec format** is fully present with `not applicable` used over silent
      omission.
- [ ] Documentation-updates (§10) names the `specs/README.md` flip, the `docs/MECHANIC-ATLAS.md`
      rows (trick-taking third-use + new bidding first-use + §10A), and the `apps/web/README.md`
      catalog/smoke surfaces.
- [ ] No spec decision weakens an upstream foundation doc or silently amends an accepted ADR.
- [ ] The deliverable is **one spec file**, authored (not decomposed into tickets).
- [ ] Commit `8ff22d9` contains every file named in §2 (the manifest is that tree).

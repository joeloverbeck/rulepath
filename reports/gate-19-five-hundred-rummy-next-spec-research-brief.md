# Deep-research brief — author the **Gate 19 (Five Hundred Rummy) implementation spec** for Rulepath

> **You are ChatGPT-Pro, the deep researcher (Session 2).** This brief is final and
> self-contained. The requirements below were settled in a prior session with full repository
> access. **Do not interview, do not ask clarifying questions, do not re-decide what comes
> next.** Produce the deliverable directly as a downloadable markdown document. If a genuine
> contradiction makes a requirement impossible, state it in the deliverable and proceed with
> the most faithful interpretation.

---

## 1. Context

The uploaded file `manifest_2026-06-25_3defeaa.txt` is the exact path inventory of the
`joeloverbeck/rulepath` repository — a Rust-first, rule-enforcing, replayable, testable
card/board-game platform where **Rust owns all behavior and TypeScript/React present only**.
The foundation docs are an ordered, layered authority indexed by `docs/README.md`:
`FOUNDATIONS.md` (the constitution) → `ARCHITECTURE.md` → `ENGINE-GAME-DATA-BOUNDARY.md` →
the area docs → `ROADMAP.md`; earlier documents govern later ones, and accepted ADRs supersede
them only by explicitly naming the affected sections.

**Fetch every file you read from commit `3defeaa` (verified `HEAD`); the uploaded manifest is
that exact tree (`git ls-tree -r --name-only HEAD`).** If any source you consult cites a
different "commit of record," note the divergence and use `3defeaa`, not the cited string.

**What just shipped (the delta baseline).** **Gate 18 — Spades (`games/blackglass_pact`)**
flipped to `Done` on 2026-06-25 per `specs/README.md`. It shipped the fixed-four partnership
Spades-family proof (Rust-owned legality, nil/blind-nil/bag scoring, fixed teams, viewer-scoped
exports, grouped web presentation) and was the **first `forward-v1` reuse-first scaffolding audit
user** under the 8F forward-governance extension. Gate 18 closed the last predecessor interlock for
Gate 19; the mechanic-atlas open-promotion-debt register (`docs/MECHANIC-ATLAS.md` §10A) is
**empty** (last reviewed at the Gate 18 closeout). The whole trick-taking lane —
`games/plain_tricks` (Gate 10.1), `games/briar_circuit` (Gate 16, Hearts), `games/vow_tide`
(Gate 17, Oh Hell), `games/blackglass_pact` (Gate 18, Spades) — the two promoted
`game-stdlib::trick_taking` helpers, the N-seat hidden-information betting exemplar
`games/river_ledger` (Gates 15 / 15.1), and the 8F governance are all shipped. This brief
commissions the **next** spec on the ladder — **Gate 19 — Five Hundred Rummy** — which is the
**first non-trick-taking card game** of the public scaling phase. It is **not** a revisit of any
shipped game or helper, and **must not re-recommend any already-shipped work — the trick-taking
games and helpers, River Ledger, or the 8F/forward-v1 governance — as if it were missing.** Build
on them as exemplars and comparison baselines; the trick-taking helpers are explicitly **not**
reused by this gate (Five Hundred Rummy has no tricks).

---

## 2. Read in full (authority order)

Read these in this order before producing. The repository tree in the manifest is ground truth;
read the **entire `docs/**` tree** (the explicit floor for this task), plus the planning and
delta-baseline artifacts below. Each line states why the file is load-bearing *for the Gate 19
spec*.

**Tier 1 — constitution & authority**
- `docs/README.md` — the authority order and the layering rule every section of the spec must respect.
- `docs/FOUNDATIONS.md` — the constitution: product priority, §11 universal invariants, §12 stop conditions, §13 ADR triggers; every spec decision must satisfy these.

**Tier 2 — area law the gate engages (read the whole `docs/**`; these are the load-bearing ones)**
- `docs/ARCHITECTURE.md` — workspace shape, dependency direction, action/view/effect/replay/determinism model the new game crate must fit.
- `docs/ENGINE-GAME-DATA-BOUNDARY.md` — the exact `engine-core` / `game-stdlib` / `games/*` / static-data boundary; **all rummy nouns (card, deck, hand, suit, rank, meld, set, run, sequence, stock, draw pile, discard pile, tableau, lay-off) stay in the game crate, never `engine-core`.**
- `docs/OFFICIAL-GAME-CONTRACT.md` — what makes a game official: §3 requirements-first workflow, §4 rules-research/source-notes format, §5 original RULES.md prose, §6 rule-coverage matrix, §10 UI-exposure obligations, §11 required trace set, §12 acceptance checklist the spec must map deliverables to.
- `docs/MECHANIC-ATLAS.md` — primitive-pressure law and a **load-bearing document for this gate**: §4 first/second/third-use rule, §5 hard-gate decision options, §5A promotion-conformance lifecycle, §5B parallel mechanical-scaffolding check, **§9A "Next-phase armed interlocks" — the `Five Hundred Rummy / Rummy 500 family` row, which states: "meld validation, public meld tableau, draw/discard zones, multi-round score target … Start local. Hard-gate before a third meld/tableau/zone helper. Do not encode meld conditions or scoring formulas in data."**, §10 atlas table (confirm there is **no** existing meld/tableau/draw-discard primitive — these are first uses), §10A **empty** debt register, and §10B deferred/candidate register (the `deterministic shuffle / private hand / staged reveal` row — Five Hundred Rummy is another private-hand card game to compare against, though it has no staged hidden-reveal; review the trigger). **No third-use hard gate fires this gate; all meld/tableau/zone shapes are first official use.**
- `docs/MULTI-SEAT-AND-SURFACE-CONTRACT.md` — **the variable-N seat law for this gate**: §2 seat-range declaration (Five Hundred Rummy is **variable 2–6, default 4**, supported {2,3,4,5,6}); §3 "Roles, Teams, and Partnerships" (declare **absent** — Five Hundred Rummy is individual competitive, no teams); §5 viewer matrix; §6 pairwise no-leak matrix (each seat's hand is private); §7 public-observer rules (the meld tableau and discard pile are **public**); §8 surface budgets (large hand + multi-seat public tableau); §10 effect grouping; §11 "Outcomes and Final Breakdowns" (per-seat score/rank to the 500 target, terminal trace summaries); §13 seat-keyed simulator summaries; §14 Gate 15+ spec/ticket minimums.
- `docs/AI-BOTS.md` — bot law: §2 levels (L0 random-legal required; L1 rule-informed; L2 authored; L3 deterministic search **perfect-info only**), §3 v1/v2 exclusions (no MCTS/ISMCTS/Monte Carlo/ML/RL), §4/§4A N-player imperfect-information boundary (allowed public/own-private/inference/belief; forbidden opponents' hidden hands, the unseen stock order), the competent-player → strategy-evidence-pack gate for any L2, §12 hidden-info bot-policy fields. **Meld-selection / lay-off / discard-choice bot policy is first-use bot territory** — no prior official game reasons about meld formation or a public tableau.
- `docs/UI-INTERACTION.md` — legal-only interaction, Rust-safe previews, effect-driven animation, replay UI, accessibility — the GAME-UI deliverable's law; the **larger hand + public meld tableau + draw/discard piles** is the new presentation surface, and the ROADMAP exit criterion "action affordances remain usable with larger hand/tableau surfaces" lives here.
- `docs/TESTING-REPLAY-BENCHMARKING.md` — §1 test taxonomy, §3 golden-trace Trace Schema v1 obligations, §4 replay determinism, §5 determinism hazards, §8 visibility/no-leak, **§8.1 N-seat no-leak taxonomy** and **§8.2 export-coverage (3+/4+ seat games: public-observer + ≥2 seat-private exports; 4+ seats exercise every seat-viewer in CI or the spec documents a sampled matrix — supported up to 6 seats)**, §12 mechanic-primitive third-use/back-port tests (first-use here, but the meld/tableau ledger entries still need first-use tests), §15 provisional budgets, §17 CI expectations.
- `docs/EVIDENCE-FIXTURE-CONTRACT.md` — the canonical evidence/fixture contract (consolidated by 8M) the spec's acceptance-evidence section instantiates; fixture-profile and completion-profile obligations.
- `docs/TRACE-SCHEMA-v1.md` — the golden-trace schema fields Gate 19 traces must populate (load-bearing because Five Hundred Rummy adds **draw-source choice, meld-formation, lay-off-onto-another-tableau, multi-card discard-pile draw, going-out, and multi-round cumulative-scoring** traces).
- `docs/ROADMAP.md` — §1 crosswalk, the **Gate 19 ladder row ("Five Hundred Rummy — draw/discard piles, public meld tableau, private hands, multi-round target; larger card-zone/action-surface proof; meld/tableau pressure")** and the **Gate 19 exit block** ("draw/discard, public melds, laying off if scoped, private hands, scoring, round/match flow, and terminal results are covered; action affordances remain usable with larger hand/tableau surfaces; meld/tableau primitive pressure is recorded and resolved or deferred"), §2 per-stage requirements and the **per-gate debt-review obligation** (mechanic-atlas pressure, mechanical-scaffolding debt, trace debt, fixture-profile debt, seat/viewer grammar debt, replay/hash debt, evidence-receipt blockers — each named or `not applicable`). The prescriptive ladder law this spec realizes.
- `docs/IP-POLICY.md` — naming/original-presentation rules for a public-domain game: §5 common-vs-neutral names (whether "Five Hundred Rummy"/"Rummy 500"/"rummy" may be used or must be coined-neutral), §4/§4A/§11 source-notes + IP-evidence-receipt checklist, §6 original prose/assets, §12 release checklist.
- `docs/AGENT-DISCIPLINE.md` — coding-agent law: bounded tasks, forbidden changes, failing-test protocol the spec's work breakdown must respect.
- `docs/SOURCES.md` — researched bibliography + Rulepath lessons; the spec's GAME-SOURCES deliverable extends this convention.
- `docs/WASM-CLIENT-BOUNDARY.md` — Rust/WASM-to-browser contract, operation groups, replay safety, dev-panel whitelist the web-exposed game must obey.
- `docs/MECHANICAL-SCAFFOLDING-REGISTER.md` — the scaffolding register and **the reuse-first audit shape Gate 19 must run as the SECOND `forward-v1` user**: the lawful shared homes, the C-01…C-10 scaffolding catalog, the "Forward Per-Game Maintenance Cadence" (pre-implementation + post-implementation checkpoints), the "Automatic Prior-Game Refactor Trigger", and the per-new-game audit record fields.
- `docs/adr/0008-mechanical-scaffolding-governance.md` — the accepted mechanical-scaffolding governance **including the 2026-06-25 forward-obligation extension (Unit 8F)**: the standing per-new-game reuse-first audit, first-use registration, queue-or-dispose prior-game refactor, and Gate 1 CI receipt (`ci/scaffolding-audits.json`, `forward-v1` required for future games). Gate 18 was the first game admitted under this rule; **Gate 19 is the second.**
- `docs/adr/0009-replay-fixture-hash-taxonomy.md` — the accepted replay/fixture/export/hash taxonomy v2 governing any trace/fixture/hash surface the new game introduces (no blanket golden regeneration; bounded authorized exports only).
- `docs/adr/0004-hidden-info-replay-export-taxonomy.md` — the accepted hidden-information replay-export taxonomy Five Hundred Rummy's redacted exports must satisfy (private hands + the unseen stock order).
- `docs/adr/0007-next-public-scaling-phase-and-gate-p-tail.md` — the accepted ADR that admits the public scaling phase (Gates 15–23) and pins Gate 19's place in it.
- (Read the remaining `docs/**` — `archival-workflow.md`, ADRs `0001`/`0002`/`0003`/`0005`/`0006`, `adr/ADR-TEMPLATE.md` — as the explicit `docs/**` floor; they shape benchmark gating, archival, and ADR conventions the spec touches at closeout. `adr/0006-blackjack-lite-roadmap-placement.md` in particular: do not accidentally re-open it.)

**Tier 3 — planning artifacts**
- `specs/README.md` — the living spec index and progress tracker; carries the **determination evidence** (active-epoch table: Gate 18 `Done` 2026-06-25, Gate 19 the lowest non-`Done` row at Order 10; §10A debt empty; all predecessors closed), the 12-section **spec format**, the new-game **mechanical-scaffolding reuse-first audit / register-update / prior-game-retrofit** requirement (a spec is incomplete when those fields are silent), and the author workflow (`/reassess-spec` → `/spec-to-tickets` happens *after* the spec, not in it).
- `templates/**` — the per-game template set the spec's deliverables instantiate: `GAME-SOURCES.md`, `GAME-RULES.md`, `GAME-RULE-COVERAGE.md`, `GAME-MECHANICS.md`, `GAME-IMPLEMENTATION-ADMISSION.md`, `GAME-HOW-TO-PLAY.md`, `COMPETENT-PLAYER.md`, `BOT-STRATEGY-EVIDENCE-PACK.md`, `GAME-AI.md`, `GAME-UI.md`, `GAME-BENCHMARKS.md`, `GAME-EVIDENCE.md`, `PRIMITIVE-PRESSURE-LEDGER.md`, `PUBLIC-RELEASE-CHECKLIST.md`, `AGENT-TASK.md`, `README.md`. The spec's Deliverables section names which of these Five Hundred Rummy fills.

**Tier 4 — delta baselines (sibling specs to mirror, not rebuild)**
- `archive/specs/gate-17-vow-tide-oh-hell-bidding-trick-taking.md` — the **variable-N seat** spec (3–7 seats) and the closest structural template for Five Hundred Rummy's variable 2–6 seats (declared seat range, setup diagnostics, by-seat simulator summaries). Mirror its variable-N seat handling; do not rebuild Vow Tide, and note its trick-taking mechanics do **not** carry over.
- `archive/specs/gate-15-river-ledger-texas-holdem-base.md` — the **N-seat private-hand no-leak + Rust-authored outcome/scoring** exemplar; the structural template for the pairwise no-leak matrix, seat-keyed summaries, golden-trace set, deterministic shuffle/deal, and per-seat outcome breakdown. The closest non-trick-taking private-hand baseline.
- `archive/specs/gate-18-blackglass-pact-spades-partnership-trick-taking.md` — the **most recent sibling** and the **first `forward-v1` audit user**: read it for the `forward-v1` reuse-first audit pattern, the register/CI-receipt closeout, and the 12-section spec anatomy. Mirror its forward-v1 audit + closeout sections; do not rebuild Blackglass Pact (its partnerships/teams do **not** apply — Five Hundred Rummy declares teams **absent**).
- `archive/specs/pre-gate-18-forward-scaffolding-reuse-governance.md` — **the 8F governance Gate 19 consumes as the second forward-v1 user**: read it to author the `forward-v1` reuse-first audit, the register-new-on-first-use obligation, the queue-or-dispose prior-game refactor closeout, and the Gate 1 CI receipt the spec must satisfy.
- `archive/specs/gate-0-repository-skeleton.md` — the canonical 12-section spec example referenced by `specs/README.md`.

### Code seams to inspect directly (*inspect in the repo, not read-fully; not pasted here*)
- `games/river_ledger/src/**` — `src/setup.rs` (declared/variable-seat setup + diagnostics), `src/visibility.rs` (per-seat pairwise no-leak filtering), deterministic shuffle/deal, and Rust-owned outcome projection — the pattern for variable-N private-hand no-leak and Rust-owned terminal scoring that Five Hundred Rummy follows for hands, stock order, and per-seat results.
- `games/vow_tide/src/**` — the **variable-N seat-range** implementation: how a 3–7 game declares its supported seat set, validates seat count at setup, and emits by-seat summaries. Five Hundred Rummy mirrors the variable-N plumbing (not the trick-taking behavior).
- `games/blackglass_pact/src/**` and `games/blackglass_pact/docs/{PRIMITIVE-PRESSURE-LEDGER.md,GAME-EVIDENCE.md}` — the most recent new-game crate and the **first forward-v1 audit evidence**: the crate module layout, the ledger format for first-use entries, and the GAME-EVIDENCE completion profile to mirror.
- `crates/game-stdlib/src/**` — confirm the lawful shared homes and that **no meld/tableau/draw-discard helper exists** (these are first uses; do not propose promotion). The `trick_taking` module is **not** reused.
- New-crate registration seams (same pattern Vow Tide / Blackglass Pact followed): `crates/wasm-api/src/lib.rs` (game import + dispatch) and `crates/wasm-api/src/constants.rs` (game id + display-name constants); `tools/simulate/src/main.rs` (game const + match arm + bot dispatch); `tools/{replay-check,fixture-check,rule-coverage}/src/main.rs` (confirm generic vs game-specific registration); `apps/web` catalog + `apps/web/README.md` (intro catalog list, Shell Surface renderer list, Smoke Layers `smoke:e2e` list, enforced by `scripts/check-catalog-docs.mjs`).
- **Forward-v1 governance receipt seams:** `ci/scaffolding-audits.json` (the audit-receipt file — already carries the frozen 17-game legacy set and the Gate 18 `forward-v1` entry; Gate 19 must add its **own `forward-v1`** entry) and `scripts/check-scaffolding-governance.mjs` (the checker that enforces it at Gate 1). Inspect both to author the audit deliverable correctly.

---

## 3. Settled intentions (final — do not re-open)

These decisions were locked in Session 1. They pre-empt every clarifying question.

1. **The next spec is Gate 19 — Five Hundred Rummy.** This determination is **settled**; your job is
   to *confirm-and-document* it (citing the evidence below) and then **write the spec** — not to
   re-decide what comes next, and not to propose a maintenance detour or a different gate. The
   evidence that fixed it, which the spec's Objective/Sequencing sections must cite:
   - **Gate 18 — Spades** (`games/blackglass_pact`) is `Done` (2026-06-25) per `specs/README.md` —
     the **last** predecessor; it was the first `forward-v1` audit user and left no open debt.
   - `docs/MECHANIC-ATLAS.md` §10A open-promotion-debt register is **empty** — so the "close open
     promotion debt before the next mechanic-ladder gate" interlock does **not** block, and no
     separate debt-closure spec is needed first.
   - Gate 19 — Five Hundred Rummy is the **lowest non-`Done`** unit on the `specs/README.md`
     active-epoch tracker (Order 10), and **every** listed predecessor (8M, 8C, 8C-R1…R4, 8F,
     Gates 15–18, accepted ADRs 0007/0008/0009) is closed.
   - `docs/ROADMAP.md` admits Gate 19 as ladder law.

2. **Do not re-emit shipped work; the trick-taking lane does NOT carry over.**
   `reports/gate-16-hearts-next-spec-research-brief.md`, `gate-17-oh-hell-next-spec-research-brief.md`,
   and `gate-18-spades-next-spec-research-brief.md` commissioned Briar Circuit, Vow Tide, and
   Blackglass Pact, which all shipped. Treat the trick-taking games, the promoted
   `game-stdlib::trick_taking` helpers, River Ledger, and the 8F/forward-v1 governance as
   **implemented exemplars and comparison baselines**, not as gaps to fill. **Five Hundred Rummy has
   no tricks, no follow-suit, no trump, no bidding** — the trick-taking helpers are explicitly **not
   reused**. The reusable inheritance is the *non-trick* plumbing: variable-N seat handling (Vow
   Tide), deterministic shuffle/deal + per-seat private-hand no-leak + Rust-owned terminal scoring
   (River Ledger), and the new-crate registration + forward-v1 audit pattern (Blackglass Pact).

3. **Variant locked: FULL CLASSIC Five Hundred Rummy (a.k.a. Rummy 500 / 500 Rum). Family locked;
   exact parameters research-pinned by you.** The spec locks:
   - **melds** of both kinds — **sets** (3+ same-rank) and **runs/sequences** (3+ consecutive
     same-suit), laid to a **public meld tableau**;
   - **laying off** — a seat may extend **any player's** existing melds (its own and opponents'),
     not only its own;
   - the **signature multi-card discard-pile draw** — a seat may take more than the top card from the
     discard pile by committing to immediately use the deepest taken card in a meld/lay-off (the
     characteristic Rummy-500 mechanic), in addition to drawing one from the stock;
   - **private hands**, a face-up **discard pile**, and a face-down **stock/draw pile**;
   - **going out** and end-of-round resolution;
   - **full classic 500-point cumulative scoring** — per-card meld values (the namesake: aces high,
     face cards, number cards), **penalties for cards left in hand**, scores accumulated across
     rounds, **first seat to 500 points wins**;
   - **multi-round match flow** to that target.

   **You must deep-research the canonical Five Hundred Rummy / Rummy 500 ruleset and its common
   house variants and research-PIN the exact parameters inside the spec, with sources**, choosing
   one canonical Rulepath variant and documenting any deliberate deviation in the GAME-SOURCES-style
   notes the spec calls for. At minimum pin:
   - the **deck** (single 52-card deck; note whether/when a two-deck shoe is conventional above a
     seat threshold — **for this gate keep a single 52-card deck across the supported 2–6 range** and
     record the two-deck convention only as a sourced note, not an implemented variant);
   - the **deal** (cards per seat by seat count) and **initial discard / first-draw** rule;
   - **meld legality** — set and run definitions, minimum meld size, ace high/low in runs (pin one
     canonical rule), whether melding is optional each turn or required to go out;
   - **laying-off** rules — onto whose melds, and validity constraints;
   - the **multi-card discard-pile draw** rule — exactly how many may be taken, the
     commit-to-use-the-deepest-card constraint, and any "frozen"/no-pickup conditions;
   - **going-out** rules — whether a final discard is required, and whether a seat may go out by
     melding/laying off the entire hand;
   - **scoring** — exact per-card values (e.g. ace = 15, face/ten = 10, pip = face value — pin the
     canonical figures), whether melded cards score positive and in-hand cards score negative, and
     how laid-off cards are credited;
   - **game-end target** (first to **500**) and **tie-break** (e.g. highest score wins ties; or
     continue a round).

4. **Seat range locked: variable 2–6 seats, default 4.** Supported set = {2, 3, 4, 5, 6}. Declare
   per `docs/MULTI-SEAT-AND-SURFACE-CONTRACT.md` §2; validate seat count at setup with diagnostics
   (mirror Vow Tide's variable-N declaration). The pairwise no-leak matrix and CI export coverage
   run up to 6 seats (§8.2 — every seat-viewer in CI, or document a sampled matrix). Single 52-card
   deck across the whole range.

5. **Teams declared ABSENT — individual competitive game.** Per
   `docs/MULTI-SEAT-AND-SURFACE-CONTRACT.md` §3, declare roles/teams/partnerships **absent**: Five
   Hundred Rummy is individual, each seat scores for itself, exactly one winner at 500. Do **not**
   carry over any partnership/team machinery from Blackglass Pact. §11 outcomes are **per-seat**
   score/rank to the 500 target.

6. **Meld / public-tableau / draw-discard-zone / laying-off / multi-round-cumulative-target are
   genuine FIRST official uses → local-only, NEW primitive-pressure ledger entries.** No prior
   official game has melds, a public meld tableau, draw/discard piles with a multi-card pickup, or
   laying off onto another player's melds. Per `docs/MECHANIC-ATLAS.md` §4 first-use rule and the
   §9A `Five Hundred Rummy / Rummy 500 family` interlock ("**Start local. Hard-gate before a third
   meld/tableau/zone helper. Do not encode meld conditions or scoring formulas in data.**"),
   implement every one of these locally and **record NEW first-use `local-only` primitive-pressure
   ledger entries** — for (a) meld validation (sets + runs), (b) public meld tableau / zone model,
   (c) draw/discard pile mechanics including the multi-card discard-pile draw, (d) laying off onto
   any player's melds, and (e) multi-round cumulative scoring to a points target — in the game's
   `PRIMITIVE-PRESSURE-LEDGER.md` and the atlas-update list. **No third-use hard gate fires this
   gate**; do **not** promote any `game-stdlib` helper for these shapes, add **no** `engine-core`
   meld/tableau/pile noun, and encode **no** meld conditions or scoring formulas in static data.
   Record the next review trigger (a third close meld/tableau/zone use) for each entry.
   Also review the §10B `deterministic shuffle / private hand / staged reveal` row: Five Hundred
   Rummy is another deterministic-shuffle private-hand game — record where it sits against that
   trigger (it has private hands but no per-seat *staged hidden reveal*), keeping shuffle/deal local.

7. **Gate 19 is the SECOND `forward-v1` reuse-first scaffolding audit user (8F governance).** The
   spec must run the forward reuse-first audit **inline, before implementation admission**, per
   `docs/adr/0008-*.md` (the 2026-06-25 forward extension), `docs/MECHANICAL-SCAFFOLDING-REGISTER.md`,
   and the 8F spec: (a) audit the scaffolding register (C-01…C-10) and lawful shared homes for
   reusable behavior-free scaffolding — a `not applicable` result requires a rationale, never
   silence; (b) **register-new** any newly invented behavior-free scaffolding shape on first use
   (`candidate` / `local-only` / `rejected`; first use does **not** authorize promotion; meld/tableau
   *behavior* is not scaffolding and stays out of the register); (c) **queue-or-dispose** any
   prior-game refactor the new scaffolding exposes — a named bounded tracker unit in
   `specs/README.md` **or** an accepted `local-only`/`deferred`/`rejected` register disposition with
   rationale, owner, evidence, and next review trigger; and (d) add the **`forward-v1`** audit receipt
   to `ci/scaffolding-audits.json`, enforced by `scripts/check-scaffolding-governance.mjs` at Gate 1.
   The spec's Scope/Deliverables/Acceptance-Evidence sections are **incomplete** if the reuse-first
   audit, expected register updates, prior-game retrofit disposition, and the `forward-v1` receipt
   are silent.

8. **Foundation-amendment posture: Gate 19 is a *user* of the foundation set, not an amender.** The
   8F/forward-v1 governance, ADRs 0008/0009/0004/0007, the multi-seat contract, and the trace/evidence
   taxonomy are already in place; Gate 19 **consumes** them. The spec therefore performs
   **documentation UPDATES** — `docs/MECHANIC-ATLAS.md` (the new first-use meld/tableau/zone/lay-off/
   multi-round-target rows + the §10B private-hand note; §10A stays empty, since no promotion is
   earned), `docs/MECHANICAL-SCAFFOLDING-REGISTER.md` (forward-v1 audit + any register-new),
   `docs/SOURCES.md`, the `specs/README.md` status flip, and `apps/web/README.md` catalog/smoke
   surfaces — **not foundation amendments**. Amend a foundation doc **only if a genuine gap forces
   it** (e.g. if the public meld tableau exposes a multi-seat-surface gap not covered by the existing
   contract); if so, flag the amendment explicitly in a dedicated spec subsection, justify it, and
   honor the supersession rule (a divergence from a foundation principle requires an accepted ADR
   naming the affected section — never silent redefinition). `assumption:` no foundation amendment is
   expected.

9. `assumption:` the deliverable is the **spec only** — it is *not* decomposed into `tickets/`
   AGENT-TASK packets. Ticket decomposition happens separately afterward via the repo's
   `/reassess-spec` then `/spec-to-tickets` workflow (`specs/README.md` "Workflow"). The spec's work
   breakdown should enumerate **candidate** AGENT-TASK items with dependency order, as the sibling
   specs do, without writing the tickets themselves.

10. `assumption:` **Neutral game name — bounded delegation to you.** Rulepath ships original,
    evocative neutral names rather than the source game's name (River Ledger ← Texas Hold'Em, Crest
    Ledger ← poker, Briar Circuit ← Hearts, Vow Tide ← Oh Hell, **Blackglass Pact ← Spades**).
    **Coin an original, IP-safe neutral name for this Five Hundred Rummy implementation** consistent
    with that catalog convention, keep "Five Hundred Rummy" / "Rummy 500" / "rummy" only as the
    rules-family label in source/IP notes, and **derive the game module id (snake_case) and the spec
    filename slug from it.** Per `docs/IP-POLICY.md` §5, keeping a common name may be *permissible*,
    but the established convention is to coin a neutral name, so coin one and use it, documenting the
    naming rationale in the spec's source/IP notes (and noting, as the siblings did, that the coinage
    does not replace the human IP/legal review IP-POLICY requires). The spec filename is
    `specs/gate-19-<neutral-slug>-five-hundred-rummy.md`; if you judge a neutral coinage genuinely
    unjustified, fall back to `specs/gate-19-five-hundred-rummy.md`.

---

## 4. The task

Produce the **Gate 19 (Five Hundred Rummy) implementation spec** — a **new** roadmap-gate spec —
that turns `docs/ROADMAP.md`'s Gate 19 row ("Five Hundred Rummy — draw/discard piles, public meld
tableau, private hands, multi-round target") into one concrete, reviewable, foundation-aligned plan.
This is a **new-spec** deliverable. The spec must (a) confirm-and-document the Gate 19 determination
with the evidence in §3.1; (b) lock the full classic Five Hundred Rummy variant (melds + laying off +
multi-card discard-pile draw + 500-point cumulative scoring) and research-pin its exact parameters
with sources; (c) define the new `games/<neutral_id>` crate, its filled official-game documents, and
its registration across tools/WASM/web-catalog surfaces; (d) scope the **variable-2–6-seat,
teams-absent** private-hand no-leak obligations to the MULTI-SEAT §2/§6/§11 model and the TESTING
§8.1/§8.2 N-seat taxonomy, with the **public meld tableau and discard pile** correctly modelled as
public surfaces; (e) **record meld / public-tableau / draw-discard-zone / laying-off / multi-round-
target as NEW first-use local mechanics** with primitive-pressure ledger entries (§3.6), confirming
**no third-use hard gate fires** and **no helper is promoted**; (f) run the **second `forward-v1`
reuse-first scaffolding audit** with register-new, queue-or-dispose, and the CI receipt (§3.7); and
(g) map every deliverable to the OFFICIAL-GAME-CONTRACT, MULTI-SEAT-AND-SURFACE-CONTRACT,
TESTING/EVIDENCE, and AI-BOTS obligations, following the `specs/README.md` 12-section spec format and
mirroring `archive/specs/gate-15-river-ledger-texas-holdem-base.md` (N-seat private-hand no-leak +
Rust outcome), `archive/specs/gate-17-vow-tide-oh-hell-bidding-trick-taking.md` (variable-N seat
plumbing), and `archive/specs/gate-18-blackglass-pact-spades-partnership-trick-taking.md` (forward-v1
audit + closeout anatomy).

---

## 5. Exploration & online-research mandate

Explore the repository as deeply as needed beyond the files listed above. **Research online as
deeply as needed** — the canonical Five Hundred Rummy / Rummy 500 ruleset and its common house
variants (deck and deal by seat count; set vs. run/sequence meld definitions and ace high/low; the
multi-card discard-pile draw and its commit-to-use constraint and any "frozen pile" rule; laying off
onto any player's melds; going-out rules; the exact per-card point values and in-hand penalties; the
500-point target and tie-break), public-domain rules sources suitable for an original prose summary,
prior open-source **rummy / meld-game engine implementations** and how they model meld validation
(sets and runs), a public tableau, draw/discard zones, the multi-card pickup, and laying off, and
research or write-ups on **rummy meld/lay-off/discard strategy** for a competent-player analysis and
any L1/L2 bot policy (**without proposing any MCTS/ISMCTS/Monte Carlo/ML/RL approach — those are
forbidden in public v1/v2**), and accessibility/UX prior art for presenting a **large private hand
plus a public multi-seat meld tableau plus stock/discard piles** for a variable-2–6-seat game (the
ROADMAP "action affordances remain usable with larger hand/tableau surfaces" criterion). Cite sources
for any external claim that shapes a decision in the spec (variant parameters, scoring values,
naming/IP rationale, strategy posture, the meld/tableau first-use ledger rationale). The deep research
is **your** job; do it thoroughly.

---

## 6. Doctrine & constraints (honor these in every spec decision)

- `docs/FOUNDATIONS.md` is the constitution — every product-behavior decision must satisfy its §11
  universal invariants and clear its §12 stop conditions; a genuine divergence requires an accepted
  ADR superseding the affected principle first ("supersede only by accepted ADR"), never designing
  against it silently. The spec is **subordinate** to the foundation set and must not redefine or
  override any foundation contract (§8 posture: updates, not amendments).
- Authority order: foundation docs govern area docs govern specs govern tickets. Where the spec and a
  foundation document disagree, the foundation document wins.
- `engine-core` stays generic and **noun-free** — no `card`, `deck`, `hand`, `suit`, `rank`, `meld`,
  `set`, `run`, `sequence`, `stock`, `pile`, `tableau`, `discard`, `board`, etc.; typed mechanic nouns
  belong in `games/*` first, shared helpers in `game-stdlib` only via the mechanic atlas. All
  meld/tableau/pile/lay-off behavior stays game-local (first use); the `trick_taking` helpers are not
  reused.
- **TypeScript never decides legality.** Legal actions (legal melds, legal lay-offs, legal draws
  including the multi-card discard pickup, legal discards, going out), validation, effects, views,
  previews, and bot decisions all come from Rust/WASM. The browser must not validate melds, compute
  scores, decide going-out, or filter the draw/discard choices.
- **No YAML and no DSL without an accepted ADR.** Static data is typed content / parameters / metadata
  only — never selectors, conditions, or triggers; **no meld conditions, lay-off legality, or scoring
  formulas encoded in data.**
- **Determinism**: deterministic shuffle/deal, deterministic stock ordering, replay, hashes,
  serialization order, and traces stay deterministic (or are explicitly migrated under ADR 0009 with
  trace notes — no blanket golden regeneration).
- **No hidden-information leaks**: variable-2–6 private hands and the **unseen stock order** must not
  leak into payloads, action trees, previews, DOM, storage, logs, effect logs, bot explanations,
  candidate rankings, or replay exports — proven by the §8.1 N-seat pairwise no-leak taxonomy and the
  §8.2 export-coverage rule (up to 6 seats). The **meld tableau and discard pile are public**; model
  them as such, but never leak a seat's private hand or the stock through them.
- **No MCTS / ISMCTS / Monte Carlo / ML / RL bots** in public v1/v2. L0 random-legal is required; any
  L2 authored meld/lay-off/discard policy needs a competent-player analysis + strategy-evidence pack
  first.
- **No casino/real-money/trade-dress mimicry; original prose and assets only** (`docs/IP-POLICY.md`);
  coin a neutral name (§3.10) and keep "rummy" only as the rules-family label.
- **Never delete or weaken tests to get green** — follow the failing-test protocol
  (`docs/AGENT-DISCIPLINE.md` §4).
- **The forward-v1 scaffolding governance is mandatory** (§3.7): the reuse-first audit, register-new,
  queue-or-dispose, and the `ci/scaffolding-audits.json` `forward-v1` receipt are gate obligations,
  not optional. Gate 19 is the second forward-v1 user.
- The spec is **authored, not decomposed** (§3.9): enumerate candidate AGENT-TASKs; do not write
  tickets.

---

## 7. Deliverable specification

Produce exactly **one downloadable markdown document**:

- **`specs/gate-19-<neutral-slug>-five-hundred-rummy.md`** — a **new** file (not a replacement).
  `<neutral-slug>` is derived from the neutral name you coin (§3.10); fall back to
  `specs/gate-19-five-hundred-rummy.md` only if you keep a common name. This is the `new-spec`
  pipeline deliverable: after download, the user saves it to that `specs/` path, `/reassess-spec`
  reassesses it in place, then `/spec-to-tickets` decomposes it — so author it as a complete,
  decompose-ready spec, **not** pre-decomposed into tickets.

It MUST follow the **12-section spec format** defined in `specs/README.md` ("Spec format"), mirroring
`archive/specs/gate-15-river-ledger-texas-holdem-base.md`,
`archive/specs/gate-17-vow-tide-oh-hell-bidding-trick-taking.md`, and
`archive/specs/gate-18-blackglass-pact-spades-partnership-trick-taking.md`:
1. Header (Spec ID, stage, gate, status `Planned`/`Not started`, date, owner, authority order; plus
   the game-identity fields the sibling specs carry — internal game id, public display name,
   rules-family label, variant id, trace rules version, data/manifest version,
   browser-impl-required flag, **official seat declaration: variable 2–6 seats** (min 2 / max 6 /
   default 4; supported set = {2,3,4,5,6}; seat labels; **teams/partnerships = absent**),
   public-observer stance, bot floor, kernel stance, primitive stance, delivery posture);
2. Objective (sourced from the ROADMAP Gate 19 row; cite the §3.1 determination evidence; state the
   **meld/tableau/zone/lay-off/multi-round-target first-use** posture and that the **trick-taking
   helpers are not reused**);
3. Scope (in / out / **not allowed** — carry the ROADMAP Gate 19 prohibitions and the
   public-scaling-phase "not allowed" list; meld/tableau/scoring stays game-local, no data formulas);
4. Deliverables (the new `games/<neutral_id>` crate tree; the filled official-game documents from
   `templates/**` including `PRIMITIVE-PRESSURE-LEDGER.md` and `GAME-EVIDENCE.md`; registration across
   `simulate`/`replay-check`/`fixture-check`/`rule-coverage`, WASM, and the `apps/web` catalog; **the
   `forward-v1` entry in `ci/scaffolding-audits.json`**);
5. Work breakdown (bounded **candidate** AGENT-TASK items with dependency order — **including the
   `forward-v1` reuse-first scaffolding audit as a gating prerequisite item**, a meld-validation
   (sets + runs) item, a public-tableau/zone-model item, a draw/discard-pile item covering the
   multi-card pickup, a laying-off item, a multi-round cumulative-scoring item, and the new first-use
   primitive-pressure ledger items);
6. Exit criteria (mapped row-for-row to the ROADMAP Gate 19 exit block — draw/discard, public melds,
   **laying off (in scope)**, private hands, scoring, round/match flow, terminal results;
   **larger hand/tableau action affordances remain usable**; and **meld/tableau primitive pressure is
   recorded and resolved/deferred** — plus by-seat simulation/benchmarks and the scaffolding-audit
   receipt);
7. Acceptance evidence (command suite; the test-taxonomy table; the **pairwise no-leak matrix** for up
   to 6 seats with the §8.2 export-coverage statement; the golden-trace minimum set per
   OFFICIAL-GAME-CONTRACT §11 / TESTING §6 — including **draw-source choice, meld-formation, lay-off-
   onto-another-tableau, multi-card discard-pile draw, going-out, and multi-round cumulative-scoring**
   traces; benchmark expectations; the `forward-v1` scaffolding-audit CI receipt; the
   EVIDENCE-FIXTURE-CONTRACT completion profile);
8. FOUNDATIONS & boundary alignment (principles engaged; the **meld/tableau/zone/lay-off/multi-round
   first-use local stance** (§3.6), the explicit **no-reuse of trick-taking helpers**, and the
   **forward-v1 audit** (§3.7); §12 stop conditions);
9. Forbidden changes (gate-specific prohibitions — no `engine-core` card/deck/meld/tableau/pile nouns,
   no meld/tableau/scoring helper promotion at first use, no static-data meld/scoring formulas, no
   partnership/team machinery, no TypeScript legality);
10. Documentation updates required (the `specs/README.md` status flip; `docs/SOURCES.md`;
    **`docs/MECHANIC-ATLAS.md`** — the new first-use meld/tableau/zone/lay-off/multi-round-target
    rows + the §10B private-hand note, §10A only if a promotion is earned (it should not be);
    **`docs/MECHANICAL-SCAFFOLDING-REGISTER.md`** — the forward-v1 audit record + any register-new
    entry + prior-game disposition; game-local docs; **`apps/web/README.md`** intro catalog list +
    Shell Surface renderer list + Smoke Layers `smoke:e2e` list, per OFFICIAL-GAME-CONTRACT §10/§12
    and `scripts/check-catalog-docs.mjs`; **the foundation-amendment posture statement** per §3.8 —
    "none expected; updates only" unless a genuine gap is flagged);
11. Sequencing (predecessor Gate 18 `Done`; this is the second `forward-v1` audit user; successor
    Gate 20 Star Halma / Chinese Checkers; admission rule);
12. Assumptions (one-line-correctable — including the carried `assumption:` lines from §3.8/§3.9/§3.10).

Use explicit `not applicable` rows over silent omissions. You may include implementation appendices
(core rules, meld/set/run model, draw/discard + multi-card-pickup model, laying-off model, scoring +
multi-round-target model, bot policy, replay/export model, WASM specifics, benchmark operations,
sources) as the sibling specs do, but keep them inside the single spec file.

> Produce the deliverable directly as a downloadable markdown document. Do not interview, do not ask
> clarifying questions — the requirements above are final. If a genuine contradiction makes a
> requirement impossible, state it in the deliverable and proceed with the most faithful
> interpretation.

---

## 8. Self-check (run against your own output before returning)

- [ ] The spec **confirms-and-documents** Gate 19 as next with the §3.1 evidence (Gate 18 `Done`
      2026-06-25; §10A empty; lowest non-`Done` Order 10; ROADMAP admits it); it does **not** re-open
      "what's next" or propose a different gate / maintenance detour.
- [ ] It locks the **full classic Five Hundred Rummy variant** (melds = sets + runs, laying off onto
      any player's melds, the multi-card discard-pile draw, 500-point cumulative scoring) and
      **research-pins** exact parameters with sources (deck/deal, meld legality, multi-card pickup +
      commit constraint, laying-off, going-out, per-card values + in-hand penalties, 500 target +
      tie-break).
- [ ] It coins an original, IP-safe **neutral name** (or justifies a common-name fallback) and
      derives the module id and spec filename slug from it.
- [ ] It declares **variable 2–6 seats** (min 2 / max 6 / default 4; supported {2,3,4,5,6}) with a
      single 52-card deck and **teams/partnerships absent** per MULTI-SEAT §2/§3, and uses
      **seat-keyed** simulator summaries (§13) and benchmarks.
- [ ] It records **meld validation, public meld tableau, draw/discard zones (incl. multi-card pickup),
      laying off, and multi-round cumulative scoring as NEW first-use `local-only`** primitive-pressure
      entries; it confirms **no third-use hard gate fires**, promotes **no** `game-stdlib` helper for
      these shapes, adds **no** `engine-core` card/deck/meld/tableau/pile noun, and encodes **no** meld
      or scoring formula in static data.
- [ ] It explicitly does **not** reuse the `game-stdlib::trick_taking` helpers (no tricks/follow-suit/
      trump/bidding) and does not re-emit any shipped trick-taking work.
- [ ] It scopes **variable-2–6 private-hand no-leak** to MULTI-SEAT §6 / TESTING §8.1 pairwise
      taxonomy and the §8.2 export-coverage rule across every surface (views, action trees, previews,
      diagnostics, effects, bot explanations, candidate rankings, replay exports, DOM, storage, logs,
      accessibility text, test IDs), models the **meld tableau + discard pile as public**, and treats
      the **unseen stock order** as hidden information.
- [ ] It runs the **second `forward-v1` reuse-first scaffolding audit** (register C-01…C-10 + lawful
      homes), **register-new** on first use, **queue-or-dispose** any prior-game refactor, and adds the
      **`forward-v1` receipt** to `ci/scaffolding-audits.json` enforced by
      `scripts/check-scaffolding-governance.mjs`; the Scope/Deliverables/Acceptance-Evidence sections
      are not silent on the audit.
- [ ] It requires an **L0 random-legal bot** and gates any L2 meld/lay-off/discard policy behind a
      competent-player analysis + strategy-evidence pack; it forbids MCTS/ISMCTS/Monte Carlo/ML/RL.
- [ ] Every deliverable maps to OFFICIAL-GAME-CONTRACT §3/§5/§6/§10/§11/§12, the MULTI-SEAT/TESTING/
      EVIDENCE obligations, and the `templates/**` set; the **12-section spec format** is fully present
      with `not applicable` used over silent omission.
- [ ] The **foundation-amendment posture** is documentation-updates-only (§3.8); any foundation
      amendment is explicitly flagged and ADR-justified, never silent; `assumption:` none is expected.
- [ ] No spec decision weakens an upstream foundation doc or silently amends an accepted ADR
      (0004/0007/0008/0009).
- [ ] The deliverable is **one spec file**, authored (not decomposed into tickets).
- [ ] Commit `3defeaa` contains every file named in §2 (the manifest is that tree).

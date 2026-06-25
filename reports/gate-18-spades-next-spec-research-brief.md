# Deep-research brief — author the **Gate 18 (Spades) implementation spec** for Rulepath

> **You are ChatGPT-Pro, the deep researcher (Session 2).** This brief is final and
> self-contained. The requirements below were settled in a prior session with full repository
> access. **Do not interview, do not ask clarifying questions, do not re-decide what comes
> next.** Produce the deliverable directly as a downloadable markdown document. If a genuine
> contradiction makes a requirement impossible, state it in the deliverable and proceed with
> the most faithful interpretation.

---

## 1. Context

The uploaded file `manifest_2026-06-25_0038e44.txt` is the exact path inventory of the
`joeloverbeck/rulepath` repository — a Rust-first, rule-enforcing, replayable, testable
card/board-game platform where **Rust owns all behavior and TypeScript/React present only**.
The foundation docs are an ordered, layered authority indexed by `docs/README.md`:
`FOUNDATIONS.md` (the constitution) → `ARCHITECTURE.md` → `ENGINE-GAME-DATA-BOUNDARY.md` →
the area docs → `ROADMAP.md`; earlier documents govern later ones, and accepted ADRs supersede
them only by explicitly naming the affected sections.

**Fetch every file you read from commit `0038e44` (verified `HEAD`); the uploaded manifest is
that exact tree (`git ls-tree -r --name-only HEAD`).** If any source you consult cites a
different "commit of record," note the divergence and use `0038e44`, not the cited string.

**What just shipped (the delta baseline).** Unit **8F — Pre-Gate-18 forward scaffolding-reuse
governance** flipped to `Done` on 2026-06-25 per `specs/README.md`. It was the **last
predecessor** blocking Gate 18: it converted the retroactive mechanical-scaffolding doctrine
(ADR 0008, the register, the 8C / 8C-R1…R4 retrofit waves) into a **standing, forward,
per-new-game obligation** — every new official game must run a reuse-first audit, register every
new behavior-free scaffolding shape on first use, and queue-or-dispose any prior-game refactor it
exposes, proven by a Gate 1 CI receipt. `specs/README.md` records: *"Gate 18 … becomes the first
`forward-v1` audit user."* The mechanic-atlas open-promotion-debt register
(`docs/MECHANIC-ATLAS.md` §10A) is **empty** — Gate 17 promoted the narrow
`game-stdlib::trick_taking` helper, conformed Plain Tricks and Briar Circuit in-gate, and left no
debt. The two promoted trick-taking helpers, the three live trick-taking games
(`games/plain_tricks`, `games/briar_circuit`, `games/vow_tide`), and the N-seat hidden-information
exemplar (`games/river_ledger`) are all shipped. This brief commissions the **next** spec on the
ladder — **Gate 18 — Spades (partnerships)** — it is **not** a revisit of Briar Circuit, Vow
Tide, Plain Tricks, or River Ledger, and **must not re-recommend any already-shipped work on
those games, the promoted helpers, or the 8F governance as if it were missing**. Build on them as
exemplars and comparison baselines; improve only where this gate genuinely requires it.

---

## 2. Read in full (authority order)

Read these in this order before producing. The repository tree in the manifest is ground truth;
read the **entire `docs/**` tree** (the explicit floor for this task), plus the planning and
delta-baseline artifacts below. Each line states why the file is load-bearing *for the Gate 18
spec*.

**Tier 1 — constitution & authority**
- `docs/README.md` — the authority order and the layering rule every section of the spec must respect.
- `docs/FOUNDATIONS.md` — the constitution: product priority, §11 universal invariants, §12 stop conditions, §13 ADR triggers; every spec decision must satisfy these.

**Tier 2 — area law the gate engages (read the whole `docs/**`; these are the load-bearing ones)**
- `docs/ARCHITECTURE.md` — workspace shape, dependency direction, action/view/effect/replay/determinism model the new game crate must fit.
- `docs/ENGINE-GAME-DATA-BOUNDARY.md` — the exact `engine-core` / `game-stdlib` / `games/*` / static-data boundary; trick-taking and partnership nouns (suit, rank, trick, hand, bid, contract, trump, **partnership, team, nil**) stay in the game crate, never `engine-core`.
- `docs/OFFICIAL-GAME-CONTRACT.md` — what makes a game official: §3 requirements-first workflow, §4 rules-research/source-notes format, §5 original RULES.md prose, §6 rule-coverage matrix, §10 UI-exposure obligations, §11 required trace set, §12 acceptance checklist the spec must map deliverables to.
- `docs/MECHANIC-ATLAS.md` — primitive-pressure law and a **load-bearing document for this gate**: §4 first/second/third-use rule, §5 hard-gate decision options, §5A promotion-conformance lifecycle, **§9A "Next-phase armed interlocks" — the `Hearts, Oh Hell, and Spades` row, which states the trick-taking helper is already promoted and that "Winner-leads, dealing, dealer rotation, scoring, hidden-hand projection, and all partnership/team behavior remain game-local comparison points; Gate 18 must not fold partnerships into generic seat identity"**, §10 rows `follow-suit legality` and `trick resolution / led-suit comparator` (both **`promoted primitive`** — "Gate 18 may reuse the helper … but must keep partnership policy local"), the `numeric trick bid / contract-vs-result / last-bidder hook` row (`local-only` first use in `vow_tide`; **"Reopen at Gate 18 … before any second-use comparison"**), the `shared-outcome cooperative terminal` row ("Revisit when another official game adds team/shared victory comparison pressure"), and §10A **empty** debt register.
- `docs/MULTI-SEAT-AND-SURFACE-CONTRACT.md` — **the partnership law for this gate**: §3 "Roles, Teams, and Partnerships" (fixed/inferred/temporary/asymmetric/absent declaration; how role/team data appears in public/seat-private/team views, traces, and outcomes; the no-leak rule that no role/team fact reaches a viewer payload unless lawfully shared), §2 seat-range declaration (Spades is **fixed 4** with a **partnership-pair grouping** over those seats), §5 viewer matrix, §6 pairwise no-leak matrix, §7 public-observer rules, §8 surface budgets (grouped partnership UI), §10 effect grouping (team-private effects), **§11 "Outcomes and Final Breakdowns" — per-team score/rank when teams exist; team results keyed by stable team ID; terminal trace summaries use per-seat/per-team standing arrays**, §13 seat-keyed simulator summaries, §14 Gate 15+ spec/ticket minimums.
- `docs/AI-BOTS.md` — bot law: §2 levels (L0 random required; L1 rule-informed; L2 authored; L3 deterministic search **perfect-info only**), §3 v1/v2 exclusions (no MCTS/ISMCTS/Monte Carlo/ML/RL), §4/§4A N-player imperfect-information boundary (allowed public/own-private/inference/belief; forbidden hidden cards/commitments/sampled worlds), the competent-player → strategy-evidence-pack gate for any L2, §12 hidden-info bot-policy fields. **Partnership-aware bidding/nil and partner-signal play are first-use bot territory** — the only prior bid bot is Vow Tide's single-seat contract; no prior bot reasons about a *partner's* contract.
- `docs/UI-INTERACTION.md` — legal-only interaction, Rust-safe previews, effect-driven animation, replay UI, accessibility — the GAME-UI deliverable's law; the **grouped partnership table** (paired seats, combined bid/bags display, nil/blind-nil indicators) is the new presentation surface.
- `docs/TESTING-REPLAY-BENCHMARKING.md` — §1 test taxonomy, §3 golden-trace Trace Schema v1 obligations, §4 replay determinism, §5 determinism hazards, §8 visibility/no-leak, **§8.1 N-seat no-leak taxonomy** and **§8.2 export-coverage (3+/4+ seat games: public-observer + ≥2 seat-private exports; 4+ seats exercise every seat-viewer in CI or the spec documents a sampled matrix — Spades is fixed 4)**, §12 mechanic-primitive third-use/back-port tests, §15 provisional budgets, §17 CI expectations.
- `docs/EVIDENCE-FIXTURE-CONTRACT.md` — the canonical evidence/fixture contract (consolidated by 8M) the spec's acceptance-evidence section instantiates; fixture-profile and completion-profile obligations.
- `docs/TRACE-SCHEMA-v1.md` — the golden-trace schema fields Gate 18 traces must populate (load-bearing because Spades adds bidding-phase, **blind-nil pre-deal commitment**, partnership-keyed scoring, and bags-rollover traces).
- `docs/ROADMAP.md` — §1 crosswalk, the **Gate 18 ladder row ("Spades — partnership pairs, team scoring, contract evaluation, grouped UI; team outcome and partnership visibility pressure")**, §2 per-stage requirements and the **per-gate debt-review obligation** (mechanic-atlas pressure, mechanical-scaffolding debt, trace debt, fixture-profile debt, seat/viewer grammar debt, replay/hash debt, evidence-receipt blockers — each named or `not applicable`), and the **Pre-Gate-18 debt interlock** + **forward scaffolding-reuse governance** notes. The prescriptive ladder law this spec realizes.
- `docs/IP-POLICY.md` — naming/original-presentation rules for a public-domain game: §5 common-vs-neutral names (whether "Spades" may be used or must be coined-neutral), §4/§11 source-notes checklist, §6 original prose/assets, §12 release checklist.
- `docs/AGENT-DISCIPLINE.md` — coding-agent law: bounded tasks, forbidden changes, failing-test protocol the spec's work breakdown must respect.
- `docs/SOURCES.md` — researched bibliography + Rulepath lessons; the spec's GAME-SOURCES deliverable extends this convention.
- `docs/WASM-CLIENT-BOUNDARY.md` — Rust/WASM-to-browser contract, operation groups, replay safety, dev-panel whitelist the web-exposed game must obey.
- `docs/MECHANICAL-SCAFFOLDING-REGISTER.md` — the scaffolding register and **the reuse-first audit shape Gate 18 must run as the first `forward-v1` user**: the lawful shared homes, the C-01…C-10 scaffolding catalog, and the per-new-game audit record fields.
- `docs/adr/0008-mechanical-scaffolding-governance.md` — the accepted mechanical-scaffolding governance **including the 2026-06-25 forward-obligation extension (Unit 8F)**: the standing per-new-game reuse-first audit, first-use registration, queue-or-dispose prior-game refactor, and Gate 1 CI receipt (`ci/scaffolding-audits.json`, `forward-v1` required for future games). Gate 18 is the first game admitted under this rule.
- `docs/adr/0009-replay-fixture-hash-taxonomy.md` — the accepted replay/fixture/export/hash taxonomy v2 governing any trace/fixture/hash surface the new game introduces (no blanket golden regeneration; bounded authorized exports only).
- `docs/adr/0004-hidden-info-replay-export-taxonomy.md` — the accepted hidden-information replay-export taxonomy Spades' redacted exports must satisfy.
- `docs/adr/0007-next-public-scaling-phase-and-gate-p-tail.md` — the accepted ADR that admits the public scaling phase (Gates 15–23) and pins Gate 18's place in it.
- (Read the remaining `docs/**` — `archival-workflow.md`, ADRs `0001`/`0002`/`0003`/`0005`/`0006`, `adr/ADR-TEMPLATE.md` — as the explicit `docs/**` floor; they shape benchmark gating, archival, and ADR conventions the spec touches at closeout. `adr/0006-blackjack-lite-roadmap-placement.md` in particular: do not accidentally re-open it.)

**Tier 3 — planning artifacts**
- `specs/README.md` — the living spec index and progress tracker; carries the **determination evidence** (active-epoch table: 8F `Done` 2026-06-25, Gate 18 the lowest non-`Done` row at Order 9 and the first `forward-v1` audit user; §10A debt empty), the 12-section **spec format**, the new-game **mechanical-scaffolding reuse-first audit / register-update / prior-game-retrofit** requirement, and the author workflow (`/reassess-spec` → `/spec-to-tickets` happens *after* the spec, not in it).
- `templates/**` — the per-game template set the spec's deliverables instantiate: `GAME-SOURCES.md`, `GAME-RULES.md`, `GAME-RULE-COVERAGE.md`, `GAME-MECHANICS.md`, `GAME-IMPLEMENTATION-ADMISSION.md`, `GAME-HOW-TO-PLAY.md`, `COMPETENT-PLAYER.md`, `BOT-STRATEGY-EVIDENCE-PACK.md`, `GAME-AI.md`, `GAME-UI.md`, `GAME-BENCHMARKS.md`, `GAME-EVIDENCE.md`, `PRIMITIVE-PRESSURE-LEDGER.md`, `PUBLIC-RELEASE-CHECKLIST.md`, `GAME-IMPLEMENTATION-ADMISSION.md`, `AGENT-TASK.md`, `README.md`. The spec's Deliverables section names which of these Spades fills.

**Tier 4 — delta baselines (sibling specs to mirror, not rebuild)**
- `archive/specs/gate-16-briar-circuit-trick-taking.md` — the **fixed-four-seat** trick-taking spec and the closest structural template for Spades' fixed 4 seats (Header + 12 sections + appendices + Outcome). Mirror its structure; do not rebuild Briar Circuit.
- `archive/specs/gate-17-vow-tide-oh-hell-bidding-trick-taking.md` — the **most recent sibling** and the **bidding/contract first-use** template: numeric bid leaves, last-bidder hook, contract-vs-result scoring, and the **trick-taking helper reuse** pattern Spades extends. It is the second-use comparison baseline for the numeric-bid atlas row. Mirror its bidding/contract sections; do not rebuild Vow Tide.
- `archive/specs/gate-10-1-plain-tricks-trick-taking-proof.md` — the **first** official trick-taking spec; a comparison baseline.
- `archive/specs/gate-15-river-ledger-texas-holdem-base.md` — the N-seat private-hand no-leak + Rust-authored outcome exemplar; the structural template for the pairwise no-leak matrix, seat-keyed summaries, golden-trace set, and per-seat/**per-team** outcome breakdown.
- `archive/specs/gate-0-repository-skeleton.md` — the canonical 12-section spec example referenced by `specs/README.md`.
- `archive/specs/pre-gate-18-forward-scaffolding-reuse-governance.md` — **the 8F governance Gate 18 first consumes**: read it to author the `forward-v1` reuse-first audit, the register-new-on-first-use obligation, the queue-or-dispose prior-game refactor closeout, and the Gate 1 CI receipt the spec must satisfy.

### Code seams to inspect directly (*inspect in the repo, not read-fully; not pasted here*)
- `games/briar_circuit/src/**` and `games/briar_circuit/docs/PRIMITIVE-PRESSURE-LEDGER.md` — the **fixed-4 trick-taking** implementation: follow-suit legality, led-suit comparator reuse, trick-winner-leads turn order, deterministic deal, owner-only hand visibility, negative scoring (`src/{actions,rules,state,setup,scoring,visibility,cards,bots}.rs`).
- `games/vow_tide/src/**` and `games/vow_tide/docs/PRIMITIVE-PRESSURE-LEDGER.md` — the **bidding/contract** implementation and **helper-reuse** exemplar: sequential `bid/<n>` leaves, dealer-only last-bidder hook, contract-vs-result scoring, public per-hand trump, and how it calls `game-stdlib::trick_taking`. The numeric-bid ledger entry Spades' second-use review extends.
- `games/plain_tricks/src/**` — the first trick-taking implementation (comparison baseline).
- `games/river_ledger/src/**` — `src/setup.rs` (fixed/declared seat setup + diagnostics), `src/visibility.rs` (per-seat pairwise no-leak), showdown/outcome projection — the pattern for per-seat **and per-team** no-leak and Rust-owned outcome breakdown.
- `crates/game-stdlib/src/**` (the `trick_taking` module) — the **already-promoted** `follow_suit_indices` and `winning_play_index` helpers Spades **reuses** (spades-always-trump fits `winning_play_index`'s caller-projected-trump contract). Confirm their exact signatures and the behavior-free boundary; do **not** add partnership policy to them.
- New-crate registration seams (same pattern Briar Circuit / Vow Tide followed): `crates/wasm-api/src/lib.rs` (game import + dispatch) and `crates/wasm-api/src/constants.rs` (game id + display-name constants); `tools/simulate/src/main.rs` (game const + match arm + bot dispatch); `tools/{replay-check,fixture-check,rule-coverage}/src/main.rs` (confirm generic vs game-specific registration); `apps/web` catalog + `apps/web/README.md` (intro catalog list, Shell Surface renderer list, Smoke Layers `smoke:e2e` list, enforced by `scripts/check-catalog-docs.mjs`).
- **Forward-v1 governance receipt seams:** `ci/scaffolding-audits.json` (the audit-receipt file — bootstrapped with the frozen 17-game legacy set; Gate 18 must add a **`forward-v1`** entry) and `scripts/check-scaffolding-governance.*` (the checker that enforces it at Gate 1). Inspect both to author the audit deliverable correctly.

---

## 3. Settled intentions (final — do not re-open)

These decisions were locked in Session 1. They pre-empt every clarifying question.

1. **The next spec is Gate 18 — Spades.** This determination is **settled**; your job is to
   *confirm-and-document* it (citing the evidence below) and then **write the spec** — not to
   re-decide what comes next, and not to propose a maintenance detour or a different gate. The
   evidence that fixed it, which the spec's Objective/Sequencing sections must cite:
   - Unit **8F — forward scaffolding-reuse governance** is `Done` (2026-06-25) per
     `specs/README.md` — the **last** predecessor interlock; `specs/README.md` names Gate 18 as
     "the first `forward-v1` audit user."
   - `docs/MECHANIC-ATLAS.md` §10A open-promotion-debt register is **empty** — so the "close open
     promotion debt before the next mechanic-ladder gate" interlock does **not** block, and no
     separate debt-closure spec is needed first.
   - Gate 18 — Spades is the **lowest non-`Done`** unit on the `specs/README.md` active-epoch
     tracker (Order 9), and **every** listed predecessor (8M, 8C, 8C-R1…R4, 8F, accepted ADRs
     0008/0009, fixed trace profiles + canonical seat grammar, the partnership/trick-taking atlas
     interlock) is closed.
   - `docs/ROADMAP.md` admits Gate 18 as ladder law.

2. **This is the same brief series as the shipped Gate 16/17 cycle — do not re-emit shipped work.**
   `reports/gate-16-hearts-next-spec-research-brief.md` and `reports/gate-17-oh-hell-next-spec-research-brief.md`
   commissioned Briar Circuit and Vow Tide, which shipped. Treat Briar Circuit, Vow Tide, Plain
   Tricks, River Ledger, the promoted `game-stdlib::trick_taking` helpers, and the 8F governance as
   **implemented exemplars and comparison baselines**, not as gaps to fill.

3. **Variant locked: full classic *partnership Spades* — nil + blind nil + bags/sandbagging. Family
   locked; exact parameters research-pinned by you.** The spec locks: **fixed 4 seats in 2 fixed
   partnerships** (seats across the table are partners), **spades are always trump**, a **bidding
   phase** where each seat bids a number of tricks (the partnership's contract is the sum of its
   two seats' bids), **individual nil bids** (a seat contracts to take zero tricks, scored
   separately from the partnership total), **blind nil** (a seat may declare nil *before looking at
   its hand* for a larger reward/penalty), **exact-style partnership scoring with a sandbagging
   "bags" penalty** (accumulated overtricks trigger a penalty at a threshold), and a **points
   target** game end. **You must deep-research the canonical partnership-Spades ruleset and its
   common house variants and research-PIN the exact parameters inside the spec, with sources**,
   choosing one canonical Rulepath variant and documenting any deliberate deviation in the
   GAME-SOURCES-style notes the spec calls for. At minimum pin:
   - the **deal** (full 52-card deck, 13 cards per seat) and **first-lead** rule (e.g. holder of a
     fixed card leads, or left-of-dealer; and whether spades may be led before "broken");
   - **bidding order** (left of dealer around) and whether bids are public/sequential;
   - **nil scoring** (made-nil bonus / failed-nil penalty) and **blind-nil scoring** (typically
     double), including how a partner's tricks interact with a nil contract;
   - the **partnership contract scoring formula** (e.g. 10 × combined bid if made, plus 1 per
     overtrick "bag"; minus 10 × combined bid if set);
   - the **bags threshold and penalty** (e.g. 10 bags ⇒ −100, with rollover) and whether bags carry
     across hands;
   - **game-end target** (e.g. first partnership to 500, or fixed hand count) and **tie-break**.
   `ROADMAP.md`'s Gate 18 row names "partnership pairs, team scoring, contract evaluation, grouped
   UI" and "team outcome and partnership visibility pressure" explicitly, so the **full partnership
   game with nil/blind-nil/bags** — not a stripped slice — is the target.

4. **Trick-taking is REUSE, not a third-use hard gate — partnership policy stays game-local.** The
   `game-stdlib::trick_taking::{follow_suit_indices, winning_play_index}` helpers were **already
   promoted at Gate 17** (atlas §10 rows now `promoted primitive`; §10A empty). Spades **reuses**
   them: `follow_suit_indices` for follow-suit legality, and `winning_play_index` with
   spades-as-the-public-trump for trick resolution (spades-always-trump fits the helper's
   caller-projected-trump contract — confirm by inspecting the helper). The spec must **not** fire a
   new third-use hard gate for follow-suit/comparator, and must **not** add any partnership, team,
   nil, scoring, turn-order, dealing, or visibility policy into the shared helper. Atlas §9A is
   binding: *"all partnership/team behavior remain game-local … Gate 18 must not fold partnerships
   into generic seat identity."* Record the reuse as a primitive-pressure ledger note confirming
   the promoted helpers fit without modification (or, if inspection shows they do **not** fit
   Spades' trump model, record that finding and keep trick resolution local — do not silently widen
   the helper).

5. **Numeric bid/contract is a SECOND use (Vow Tide → Spades) → reopen/review, keep local.** The
   `numeric trick bid / contract-vs-result / last-bidder hook` atlas row is `local-only` first-use
   (Vow Tide) and explicitly says *"Reopen at Gate 18 … before any second-use comparison or shared
   helper proposal."* The spec must **perform that second-use comparison** (Vow Tide's single-seat
   exact-bid contract vs. Spades' partnership-combined bid + individual nil/blind-nil contract),
   **decide to keep bidding/contract behavior game-local** (second use does not authorize
   promotion; the shapes differ structurally — partnership aggregation, nil, blind-nil pre-deal
   commitment, bags), and **record the next review trigger** (a third close numeric-contract game).
   Do **not** promote a bid/contract/scoring helper, and do **not** encode bid legality or scoring
   formulas in static data.

6. **Team/partnership outcome is a genuine FIRST official use → local-only, NEW ledger entry.** No
   prior official game has fixed partnerships, team-aggregated contracts, or per-team outcomes (the
   `shared-outcome cooperative terminal` row covered *fully* cooperative all-win/all-lose, not
   competitive teams). Per `docs/MECHANIC-ATLAS.md` §4 first-use rule, implement partnerships
   locally and **record a NEW first-use `local-only` primitive-pressure ledger entry** — covering
   partnership pairing, combined-bid contract evaluation, partnership-keyed scoring, bags
   accumulation, nil/blind-nil interaction, and **partnership visibility** (what one partner may see
   of the other, and what the team view exposes) — in the spec and the atlas-update list. Do
   **not** generalize it; honor MULTI-SEAT §3 (team declaration: **fixed** partnerships) and §11
   (per-team outcome keyed by stable team ID). Note that **blind nil** adds a pre-deal
   hidden-commitment trace surface — declare it explicitly under the hidden-information/no-leak
   obligations.

7. **Gate 18 is the FIRST `forward-v1` reuse-first audit user (8F governance).** The spec must run
   the forward reuse-first audit **inline, before implementation admission**, per
   `docs/adr/0008-*.md` (the 2026-06-25 forward extension), `docs/MECHANICAL-SCAFFOLDING-REGISTER.md`,
   and the 8F spec: (a) audit the scaffolding register (C-01…C-10) and lawful shared homes for
   reusable scaffolding — a `not applicable` result requires a rationale, never silence; (b)
   **register-new** any newly invented behavior-free scaffolding shape on first use (`candidate` /
   `local-only` / `rejected`; first use does **not** authorize promotion); (c) **queue-or-dispose**
   any prior-game refactor Spades' scaffolding exposes — a named bounded tracker unit in
   `specs/README.md` **or** an accepted `local-only`/`deferred`/`rejected` register disposition with
   rationale, owner, evidence, and next review trigger; and (d) require the **`forward-v1`** audit
   receipt in `ci/scaffolding-audits.json`, enforced by `scripts/check-scaffolding-governance.*` at
   Gate 1. The spec's Scope/Deliverables/Acceptance-Evidence sections are **incomplete** if the
   reuse-first audit, expected register updates, prior-game retrofit disposition, and the
   `forward-v1` receipt are silent.

8. **Foundation-amendment posture: Gate 18 is a *user* of the foundation set, not an amender.** The
   8F governance, ADRs 0008/0009, the multi-seat partnership/team clauses, and the trace/evidence
   taxonomy are already in place; Gate 18 **consumes** them. The spec therefore performs
   **documentation UPDATES** — `docs/MECHANIC-ATLAS.md` (new team/partnership first-use row, the
   numeric-bid second-use note, the trick-taking reuse note; §10A only if a promotion is earned,
   which it should not be), `docs/MECHANICAL-SCAFFOLDING-REGISTER.md` (forward-v1 audit + any
   register-new), `docs/SOURCES.md`, the `specs/README.md` status flip, and `apps/web/README.md`
   catalog/smoke surfaces — **not foundation amendments**. Amend a foundation doc **only if a
   genuine gap forces it**; if so, flag the amendment explicitly in a dedicated spec subsection,
   justify it, and honor the supersession rule (a divergence from a foundation principle requires an
   accepted ADR naming the affected section — never silent redefinition). `assumption:` no
   foundation amendment is expected; the partnership/team surface is already covered by
   MULTI-SEAT-AND-SURFACE-CONTRACT §3/§11.

9. `assumption:` the deliverable is the **spec only** — it is *not* decomposed into `tickets/`
   AGENT-TASK packets. Ticket decomposition happens separately afterward via the repo's
   `/reassess-spec` then `/spec-to-tickets` workflow (`specs/README.md` "Workflow"). The spec's
   work breakdown should enumerate **candidate** AGENT-TASK items with dependency order, as the
   sibling specs do, without writing the tickets themselves.

10. `assumption:` **Neutral game name — bounded delegation to you.** Rulepath ships original,
    evocative neutral names rather than the source game's name (River Ledger ← Texas Hold'Em, Crest
    Ledger ← poker, Briar Circuit ← Hearts, **Vow Tide ← Oh Hell**). **Coin an original, IP-safe
    neutral name for this Spades implementation** consistent with that catalog convention, keep
    "Spades" only as the rules-family label in source/IP notes, and **derive the game module id
    (snake_case) and the spec filename slug from it.** Per `docs/IP-POLICY.md` §5, keeping "Spades"
    is *permissible* (it is a common public-domain name), but the established convention is to coin
    a neutral name, so coin one and use it, documenting the naming rationale in the spec's source/IP
    notes (and noting, as the siblings did, that the coinage does not replace the human IP review
    IP-POLICY requires). The spec filename is
    `specs/gate-18-<neutral-slug>-spades-partnership-trick-taking.md`; if you judge a neutral
    coinage genuinely unjustified and keep "Spades," use `specs/gate-18-spades-partnership-trick-taking.md`.

---

## 4. The task

Produce the **Gate 18 (Spades) implementation spec** — a **new** roadmap-gate spec — that turns
`docs/ROADMAP.md`'s Gate 18 row ("Spades — partnership pairs, team scoring, contract evaluation,
grouped UI") into one concrete, reviewable, foundation-aligned plan. This is a **new-spec**
deliverable. The spec must (a) confirm-and-document the Gate 18 determination with the evidence in
§3.1; (b) lock the full classic partnership-Spades variant family (nil + blind nil + bags) and
research-pin its exact parameters with sources; (c) define the new `games/<neutral_id>` crate, its
filled official-game documents, and its registration across tools/WASM/web-catalog surfaces; (d)
scope the **fixed-4-seat, two-partnership** private-hand and team-visibility no-leak obligations to
the MULTI-SEAT §3/§11 partnership model and the TESTING §8.1/§8.2 N-seat taxonomy; (e) **reuse the
promoted trick-taking helpers** (§3.4), perform the **numeric-bid second-use comparison and keep it
local** (§3.5), and **record partnership/team outcome as a new first-use local mechanic** (§3.6);
(f) run the **first `forward-v1` reuse-first scaffolding audit** with register-new, queue-or-dispose,
and the CI receipt (§3.7); and (g) map every deliverable to the OFFICIAL-GAME-CONTRACT,
MULTI-SEAT-AND-SURFACE-CONTRACT, TESTING/EVIDENCE, and AI-BOTS obligations, following the
`specs/README.md` 12-section spec format and mirroring `archive/specs/gate-16-briar-circuit-trick-taking.md`
(fixed-4 structure) and `archive/specs/gate-17-vow-tide-oh-hell-bidding-trick-taking.md`
(bidding/contract + helper reuse).

---

## 5. Exploration & online-research mandate

Explore the repository as deeply as needed beyond the files listed above. **Research online as
deeply as needed** — the canonical partnership-Spades ruleset and its common house variants (bidding
order; **nil** and **blind nil** scoring and how a partner's tricks interact with a nil contract;
the partnership combined-bid contract scoring formula; the **bags/sandbagging** threshold, penalty,
and rollover; whether spades may be led before "broken"; first-lead rule; game-end target and
tie-break), public-domain rules sources suitable for an original prose summary, prior open-source
trick-taking-with-partnership engine implementations and how they model partnership aggregation, nil
contracts, and bags, and research or write-ups on **Spades bidding/nil/partner-signal strategy** for
a competent-player analysis and any L1/L2 bot policy (**without proposing any
MCTS/ISMCTS/Monte Carlo/ML/RL approach — those are forbidden in public v1/v2**), and
accessibility/UX prior art for presenting a **grouped partnership table** (paired seats, combined
bid/bags, nil indicators) for a fixed-4 hidden-hand bidding-then-trick game. Cite sources for any
external claim that shapes a decision in the spec (variant parameters, naming/IP rationale, strategy
posture, the second-use comparison). The deep research is **your** job; do it thoroughly.

---

## 6. Doctrine & constraints (honor these in every spec decision)

- `docs/FOUNDATIONS.md` is the constitution — every product-behavior decision must satisfy its §11
  universal invariants and clear its §12 stop conditions; a genuine divergence requires an accepted
  ADR superseding the affected principle first ("supersede only by accepted ADR"), never designing
  against it silently. The spec is **subordinate** to the foundation set and must not redefine or
  override any foundation contract (§8 posture: updates, not amendments).
- Authority order: foundation docs govern area docs govern specs govern tickets. Where the spec and
  a foundation document disagree, the foundation document wins.
- `engine-core` stays generic and **noun-free** — no `card`, `deck`, `hand`, `suit`, `rank`,
  `trick`, `trump`, `bid`, `contract`, `partnership`, `team`, `nil`, `board`, etc.; typed mechanic
  nouns belong in `games/*` first, shared helpers in `game-stdlib` only via the mechanic atlas. The
  trick-taking helpers are **reused** (§3.4); partnership, bidding, nil, and bags stay game-local.
- **TypeScript never decides legality.** Legal actions (legal bids, legal nil/blind-nil
  declarations, legal cards), validation, effects, views, previews, and bot decisions all come from
  Rust/WASM. The browser must not filter bids or cards, compute partnership scores, or decide nil
  outcomes.
- **No YAML and no DSL without an accepted ADR.** Static data is typed content / parameters /
  metadata only — never selectors, conditions, or triggers; no scoring, follow-suit, bid-legality,
  or bags formulas encoded in data.
- **Determinism**: deterministic shuffle/deal, the blind-nil pre-deal commitment, replay, hashes,
  serialization order, and traces stay deterministic (or are explicitly migrated under ADR 0009 with
  trace notes — no blanket golden regeneration).
- **No hidden-information leaks**: fixed-4 private hands, **blind-nil pre-deal commitments**, and any
  unrevealed deck tail must not leak into payloads, action trees, previews, DOM, storage, logs,
  effect logs, bot explanations, candidate rankings, or replay exports — proven by the §8.1 N-seat
  pairwise no-leak taxonomy and the §8.2 export-coverage rule. **No partnership/team fact may reach a
  viewer payload unless Rust has made it public or team-authorized** (MULTI-SEAT §3).
- **No MCTS / ISMCTS / Monte Carlo / ML / RL bots** in public v1/v2. L0 random-legal is required;
  any L2 authored bidding/nil/play policy needs a competent-player analysis + strategy-evidence pack
  first.
- **No casino/real-money/trade-dress mimicry; original prose and assets only** (`docs/IP-POLICY.md`).
- **Never delete or weaken tests to get green** — follow the failing-test protocol
  (`docs/AGENT-DISCIPLINE.md` §4).
- **The forward-v1 scaffolding governance is mandatory** (§3.7): the reuse-first audit,
  register-new, queue-or-dispose, and the `ci/scaffolding-audits.json` `forward-v1` receipt are gate
  obligations, not optional.
- The spec is **authored, not decomposed** (§3.9): enumerate candidate AGENT-TASKs; do not write
  tickets.

---

## 7. Deliverable specification

Produce exactly **one downloadable markdown document**:

- **`specs/gate-18-<neutral-slug>-spades-partnership-trick-taking.md`** — a **new** file (not a
  replacement). `<neutral-slug>` is derived from the neutral name you coin (§3.10); fall back to
  `specs/gate-18-spades-partnership-trick-taking.md` only if you keep "Spades". This is the
  `new-spec` pipeline deliverable: after download, the user saves it to that `specs/` path,
  `/reassess-spec` reassesses it in place, then `/spec-to-tickets` decomposes it — so author it as a
  complete, decompose-ready spec, **not** pre-decomposed into tickets.

It MUST follow the **12-section spec format** defined in `specs/README.md` ("Spec format"),
mirroring `archive/specs/gate-16-briar-circuit-trick-taking.md` and
`archive/specs/gate-17-vow-tide-oh-hell-bidding-trick-taking.md`:
1. Header (Spec ID, stage, gate, status `Planned`/`Not started`, date, owner, authority order; plus
   the game-identity fields the sibling specs carry — internal game id, public display name,
   rules-family label, variant id, trace rules version, data/manifest version,
   browser-impl-required flag, **official seat declaration: fixed 4 seats in 2 fixed partnerships**
   (min/max/default = 4; supported set = {4}; seat labels; **partnership/team grouping + stable team
   IDs**), public-observer stance, bot floor, kernel stance, primitive stance, delivery posture);
2. Objective (sourced from the ROADMAP Gate 18 row; cite the §3.1 determination evidence; state the
   **trick-taking reuse** posture, the **bid/contract second-use** posture, and the **team/partnership
   first-use** posture);
3. Scope (in / out / **not allowed** — carry the ROADMAP Gate 18 prohibitions and the
   public-scaling-phase "not allowed" list; partnerships stay game-local, not generic seat identity);
4. Deliverables (the new `games/<neutral_id>` crate tree; the filled official-game documents from
   `templates/**` including `PRIMITIVE-PRESSURE-LEDGER.md` and `GAME-EVIDENCE.md`; registration
   across `simulate`/`replay-check`/`fixture-check`/`rule-coverage`, WASM, and the `apps/web`
   catalog; **the `forward-v1` entry in `ci/scaffolding-audits.json`**);
5. Work breakdown (bounded **candidate** AGENT-TASK items with dependency order — **including the
   `forward-v1` reuse-first scaffolding audit as a gating prerequisite item**, the trick-taking
   helper-reuse conformance item, the numeric-bid second-use ledger item, and the new
   team/partnership first-use ledger item);
6. Exit criteria (mapped row-for-row to the ROADMAP Gate 18 row — fixed 4 seats / 2 partnerships,
   bidding order, nil + blind-nil declarations, partnership combined-contract evaluation, bags
   accumulation + penalty, trick play under follow-suit with spades-trump, team scoring, terminal
   per-team standings, by-seat simulation/benchmarks, the scaffolding-audit receipt, and the
   primitive-pressure resolutions);
7. Acceptance evidence (command suite; the test-taxonomy table; the **pairwise no-leak matrix** for
   4 seats with the §8.2 export-coverage statement; the golden-trace minimum set per
   OFFICIAL-GAME-CONTRACT §11 / TESTING §6 — including **bidding-phase, blind-nil pre-deal
   commitment, partnership-keyed scoring, and bags-rollover** traces; benchmark expectations; the
   `forward-v1` scaffolding-audit CI receipt; the EVIDENCE-FIXTURE-CONTRACT completion profile);
8. FOUNDATIONS & boundary alignment (principles engaged; the **trick-taking reuse confirmation**
   (§3.4), the **numeric-bid second-use comparison + keep-local decision** (§3.5), the **first-use
   team/partnership local stance** (§3.6), and the **forward-v1 audit** (§3.7); §12 stop conditions);
9. Forbidden changes (gate-specific prohibitions — no partnership policy in the shared helper, no
   `engine-core` nouns, no bid/scoring/bags helper promotion at second use, no static-data formulas);
10. Documentation updates required (the `specs/README.md` status flip; `docs/SOURCES.md`;
    **`docs/MECHANIC-ATLAS.md`** — the trick-taking reuse note, the numeric-bid **second-use** note,
    a **new team/partnership first-use row**, §10A only if promotion is earned;
    **`docs/MECHANICAL-SCAFFOLDING-REGISTER.md`** — the forward-v1 audit record + any register-new
    entry + prior-game disposition; game-local docs; **`apps/web/README.md`** intro catalog list +
    Shell Surface renderer list + Smoke Layers `smoke:e2e` list, per OFFICIAL-GAME-CONTRACT §10/§12
    and `scripts/check-catalog-docs.mjs`; **the foundation-amendment posture statement** per §3.8 —
    "none expected; updates only" unless a genuine gap is flagged);
11. Sequencing (predecessor 8F `Done`; this is the first `forward-v1` audit user; successor Gate 19
    Five Hundred Rummy; admission rule);
12. Assumptions (one-line-correctable — including the carried `assumption:` lines from §3.8/§3.9/§3.10).

Use explicit `not applicable` rows over silent omissions. You may include implementation appendices
(core rules, bidding/nil/blind-nil model, bags/scoring model, partnership outcome model, bot policy,
replay/export model, WASM specifics, benchmark operations, sources) as the sibling specs do, but
keep them inside the single spec file.

> Produce the deliverable directly as a downloadable markdown document. Do not interview, do not ask
> clarifying questions — the requirements above are final. If a genuine contradiction makes a
> requirement impossible, state it in the deliverable and proceed with the most faithful
> interpretation.

---

## 8. Self-check (run against your own output before returning)

- [ ] The spec **confirms-and-documents** Gate 18 as next with the §3.1 evidence (8F `Done` and the
      first `forward-v1` user; §10A empty; lowest non-`Done` Order 9; ROADMAP admits it); it does
      **not** re-open "what's next" or propose a different gate / maintenance detour.
- [ ] It locks the **full classic partnership-Spades variant (nil + blind nil + bags)** and
      **research-pins** exact parameters with sources (deal + first-lead, bidding order, nil +
      blind-nil scoring, partnership contract formula, bags threshold/penalty/rollover, game-end +
      tie-break).
- [ ] It coins an original, IP-safe **neutral name** (or justifies keeping "Spades") and derives the
      module id and spec filename slug from it.
- [ ] It **reuses** the promoted `game-stdlib::trick_taking::{follow_suit_indices, winning_play_index}`
      helpers (spades-as-trump) and records the reuse as a ledger note; it does **not** fire a new
      third-use hard gate and adds **no** partnership/team/nil/scoring policy to the shared helper;
      it adds **no** `engine-core` card/suit/rank/hand/trick/trump/bid/contract/partnership/team/nil
      noun.
- [ ] It performs the **numeric-bid second-use comparison** (Vow Tide ↔ Spades) and **keeps
      bidding/contract local** with a recorded next-review trigger; no bid/scoring/bags helper is
      promoted; no formula is in static data.
- [ ] It records **team/partnership outcome as a NEW first-use `local-only`** primitive-pressure
      entry (pairing, combined contract, partnership-keyed scoring, bags, nil/blind-nil interaction,
      partnership visibility) honoring MULTI-SEAT §3 (fixed partnerships) and §11 (per-team outcome).
- [ ] It runs the **first `forward-v1` reuse-first scaffolding audit** (register C-01…C-10 + lawful
      homes), **register-new** on first use, **queue-or-dispose** any prior-game refactor, and adds
      the **`forward-v1` receipt** to `ci/scaffolding-audits.json` enforced by
      `scripts/check-scaffolding-governance.*`; the Scope/Deliverables/Acceptance-Evidence sections
      are not silent on the audit.
- [ ] It scopes **fixed-4-seat private-hand + team-visibility no-leak** to MULTI-SEAT §6 / TESTING
      §8.1 pairwise taxonomy and the §8.2 export-coverage rule, across every surface (views, action
      trees, previews, diagnostics, effects, bot explanations, candidate rankings, replay exports,
      DOM, storage, logs, accessibility text, test IDs), and treats **blind-nil pre-deal commitment**
      as a hidden-information surface.
- [ ] It declares the **fixed 4-seat / 2-partnership** seat+team grammar per MULTI-SEAT §2/§3 (stable
      team IDs) and uses **seat-keyed / team-keyed** simulator summaries (§13) and benchmarks.
- [ ] It requires an **L0 random-legal bot** and gates any L2 bidding/nil/play policy behind a
      competent-player analysis + strategy-evidence pack; it forbids MCTS/ISMCTS/Monte Carlo/ML/RL.
- [ ] Every deliverable maps to OFFICIAL-GAME-CONTRACT §3/§5/§6/§10/§11/§12, the
      MULTI-SEAT/TESTING/EVIDENCE obligations, and the `templates/**` set; the **12-section spec
      format** is fully present with `not applicable` used over silent omission.
- [ ] The **foundation-amendment posture** is documentation-updates-only (§3.8); any foundation
      amendment is explicitly flagged and ADR-justified, never silent; `assumption:` none is expected.
- [ ] No spec decision weakens an upstream foundation doc or silently amends an accepted ADR
      (0004/0007/0008/0009).
- [ ] The deliverable is **one spec file**, authored (not decomposed into tickets).
- [ ] Commit `0038e44` contains every file named in §2 (the manifest is that tree).

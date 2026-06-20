# Deep-research brief — author the **Gate 16 (Hearts) implementation spec** for Rulepath

> **You are ChatGPT-Pro, the deep researcher (Session 2).** This brief is final and
> self-contained. The requirements below were settled in a prior session with full repository
> access. **Do not interview, do not ask clarifying questions, do not re-decide what comes
> next.** Produce the deliverable directly as a downloadable markdown document. If a genuine
> contradiction makes a requirement impossible, state it in the deliverable and proceed with
> the most faithful interpretation.

---

## 1. Context

The uploaded file `manifest_2026-06-20_83e7161.txt` is the exact path inventory of the
`joeloverbeck/rulepath` repository — a Rust-first, rule-enforcing, replayable, testable
card/board-game platform where **Rust owns all behavior and TypeScript/React present only**.
The foundation docs are an ordered, layered authority indexed by `docs/README.md`:
`FOUNDATIONS.md` (the constitution) → `ARCHITECTURE.md` → `ENGINE-GAME-DATA-BOUNDARY.md` →
the area docs → `ROADMAP.md`; earlier documents govern later ones, and accepted ADRs supersede
them only by explicitly naming the affected sections.

**Fetch every file you read from commit `83e7161` (verified `HEAD`); the uploaded manifest is
that exact tree (`git ls-tree -r --name-only HEAD`).** If any source you consult cites a
different "commit of record," note the divergence and use `83e7161`, not the cited string.

**What just shipped (the delta baseline).** Gate 15.1 — River Ledger all-in / side pots —
flipped to `Done` on 2026-06-20. The mechanic-atlas open-promotion-debt register
(`docs/MECHANIC-ATLAS.md` §10A) is **empty** as of that closeout. The River Ledger family
(`games/river_ledger`) is the live N-seat hidden-information exemplar. This brief commissions
the **next** spec on the ladder — it is **not** a revisit of River Ledger and must not
re-recommend any already-shipped River Ledger work.

---

## 2. Read in full (authority order)

Read these in this order before producing. The repository tree in the manifest is the ground
truth; read the **entire `docs/**` tree** (the explicit floor for this task), plus the
planning and delta-baseline artifacts below. Each line states why the file is load-bearing
*for the Gate 16 spec*.

**Tier 1 — constitution & authority**
- `docs/README.md` — the authority order and the layering rule every section of the spec must respect.
- `docs/FOUNDATIONS.md` — the constitution: product priority, §11 universal invariants, §12 stop conditions, §13 ADR triggers; every spec decision must satisfy these.

**Tier 2 — area law the gate engages (read the whole `docs/**`; these are the load-bearing ones)**
- `docs/ARCHITECTURE.md` — workspace shape, dependency direction, action/view/effect/replay/determinism model the new game crate must fit.
- `docs/ENGINE-GAME-DATA-BOUNDARY.md` — the exact `engine-core` / `game-stdlib` / `games/*` / static-data boundary; trick-taking nouns (suit, rank, trick, hand) stay in the game crate, never `engine-core`.
- `docs/OFFICIAL-GAME-CONTRACT.md` — what makes a game official: the requirements-first workflow (§3), required RULES.md sections (§5), UI-exposure obligations (§10), required trace set (§11), and the acceptance checklist (§12) the spec must map deliverables to.
- `docs/MECHANIC-ATLAS.md` — primitive-pressure law: §4 first/second/third-use rule, §5/§5A promotion lifecycle, §9A armed interlock for "Hearts, Oh Hell, and Spades" (reopen trick-taking + private-hand ledger entries), §10 atlas table (`follow-suit legality`, `trick resolution / led-suit comparator`, `trick-winner-leads turn order`, `deal rotation / trick-round redeal` rows, all `local-only` first-use under `plain_tricks`), and §10A empty debt register.
- `docs/AI-BOTS.md` — bot law: levels (L0 required), §3 v1/v2 exclusions (no MCTS/ISMCTS/Monte Carlo/ML/RL), §4/§4A information boundary for N-player imperfect-info bots, §9 competent-player → strategy-evidence-pack gate for L2, §10 viewer-safe explanation contract, §12 hidden-info bot policy fields.
- `docs/UI-INTERACTION.md` — legal-only interaction, Rust-safe previews, effect-driven animation, replay UI, accessibility — the GAME-UI deliverable's law.
- `docs/TESTING-REPLAY-BENCHMARKING.md` — test taxonomy (§1), golden-trace Trace Schema v1 obligations (§3), replay determinism (§4), required trace set (§6), visibility/no-leak (§8) and the **N-seat no-leak taxonomy (§8.1)**, UI smoke (§11), native-first benchmark doctrine (§14), provisional budgets (§15), CI expectations (§17).
- `docs/MULTI-SEAT-AND-SURFACE-CONTRACT.md` — N-seat obligations a 4-seat game must honor: seat-range declaration (§2), viewer matrix (§5), pairwise no-leak matrix (§6), public-observer rules (§7), surface budgets (§8), outcome/breakdown model (§11), trace/view-hash expectations (§12), and the Gate 15+ spec/ticket minimums (§14).
- `docs/ROADMAP.md` — §1 crosswalk and **§15 "Gate 16: Hearts"** (the gate's purpose and exit list this spec realizes); the prescriptive ladder law.
- `docs/IP-POLICY.md` — naming/original-presentation rules for a public-domain game: §5 common/neutral names (governs whether "Hearts" may be used or must be coined-neutral), §6 original prose/assets, §7 AI-asset review, §11 source-notes checklist, §12 release checklist.
- `docs/AGENT-DISCIPLINE.md` — coding-agent law: bounded tasks, forbidden changes, failing-test protocol the spec's work breakdown must respect.
- `docs/SOURCES.md` — researched bibliography + Rulepath lessons; the spec's GAME-SOURCES deliverable extends this convention.
- `docs/TRACE-SCHEMA-v1.md` — the golden-trace schema fields the Gate 16 traces must populate (load-bearing because Hearts adds 4-seat private-hand redacted traces).
- `docs/WASM-CLIENT-BOUNDARY.md` — Rust/WASM-to-browser contract, operation groups, replay safety, dev-panel whitelist the web-exposed Hearts game must obey.
- `docs/adr/0004-hidden-info-replay-export-taxonomy.md` — the accepted hidden-information replay-export taxonomy Hearts' redacted exports must satisfy.
- `docs/adr/0007-next-public-scaling-phase-and-gate-p-tail.md` — the accepted ADR that admits the public scaling phase (Gates 15–23) and pins Gate 16's place in it.
- `docs/adr/0006-blackjack-lite-roadmap-placement.md` — context on deferred card-game comparison cases; read so the spec does not accidentally re-open it.
- (Read the remaining `docs/**` — `archival-workflow.md`, the other ADRs 0001/0002/0003/0005, `adr/ADR-TEMPLATE.md` — as the explicit `docs/**` floor; they shape benchmark gating, archival, and ADR conventions the spec touches at closeout.)

**Tier 3 — planning artifacts**
- `specs/README.md` — the living spec index and progress tracker; carries the **determination evidence** (active-epoch table: Gate 15.1 `Done`, Gate 16 the lowest non-`Done` row), the 12-section **spec format** your deliverable must follow, and the author workflow.
- `templates/**` — the per-game template set the spec's deliverables instantiate: `GAME-SOURCES.md`, `GAME-RULES.md`, `GAME-RULE-COVERAGE.md`, `GAME-MECHANICS.md`, `GAME-IMPLEMENTATION-ADMISSION.md`, `GAME-HOW-TO-PLAY.md`, `COMPETENT-PLAYER.md`, `BOT-STRATEGY-EVIDENCE-PACK.md`, `GAME-AI.md`, `GAME-UI.md`, `GAME-BENCHMARKS.md`, `PRIMITIVE-PRESSURE-LEDGER.md`, `PUBLIC-RELEASE-CHECKLIST.md`, `AGENT-TASK.md`, `README.md`. The spec's Deliverables section names which of these Hearts fills.

**Tier 4 — delta baselines (sibling specs to mirror, not rebuild)**
- `archive/specs/gate-10-1-plain-tricks-trick-taking-proof.md` — the **first** official trick-taking spec; the direct comparison baseline for the trick-taking ledger reopening and the closest structural template for a small card-game gate (note its implementation appendices A–G).
- `archive/specs/gate-15-river-ledger-texas-holdem-base.md` — the most recent N-seat hidden-information game spec; the structural template for seat-range, viewer matrix, pairwise no-leak matrix, golden-trace set, and benchmark sections.
- `archive/specs/gate-15-1-river-ledger-all-in-side-pots.md` — the immediately-preceding spec; shows the current closeout/evidence conventions and the empty-debt posture.
- `archive/specs/gate-0-repository-skeleton.md` — the canonical 12-section spec example referenced by `specs/README.md`.

### Code seams to inspect directly (*inspect in the repo, not read-fully; not pasted here*)
- `games/plain_tricks/docs/**` and `games/plain_tricks/src/**` — the trick-taking baseline. Inspect how follow-suit legality, the led-suit comparator / trick resolution, trick-winner-leads turn order, deterministic deal rotation, and owner-only hand visibility are structured (`src/state.rs`, `src/actions.rs`, `src/rules.rs`, `src/visibility.rs`, `src/setup.rs`, `src/bots.rs`; `docs/MECHANICS.md`, `docs/RULES.md`, `docs/RULE-COVERAGE.md`, `docs/PRIMITIVE-PRESSURE-LEDGER.md`). This is what the Gate 16 ledger entry compares Hearts against.
- `games/river_ledger/docs/**` and `games/river_ledger/src/**` — the N-seat private-hand no-leak + Rust-authored outcome exemplar (`src/visibility.rs`, `src/showdown.rs`, `src/replay_support.rs`; the pairwise no-leak matrix and seat-viewer matrix in `docs/PRIMITIVE-PRESSURE-LEDGER.md` / `docs/UI.md`). This is the pattern Hearts' 4-seat no-leak and per-seat outcome breakdown follow.
- `games/race_to_n/`, `games/three_marks/` (any minimal recent game) — only if you need the current crate-registration / tooling-wiring shape (`simulate`, `replay-check`, `fixture-check`, `rule-coverage`, web catalog) the spec's Deliverables must list.

---

## 3. Settled intentions (final — do not re-open)

These decisions were locked in Session 1. They pre-empt every clarifying question.

1. **The next spec is Gate 16 — Hearts.** This determination is **settled**; your job is to
   *confirm-and-document* it (citing the evidence below) and then **write the spec** — not to
   re-decide what comes next. The evidence that fixed it, which the spec's Objective/Sequencing
   sections must cite:
   - Gate 15.1 (River Ledger all-in / side pots) is `Done` (2026-06-20) per `specs/README.md`.
   - `docs/MECHANIC-ATLAS.md` §10A open-promotion-debt register is **empty** ("Current debt:
     _None_", last reviewed at Gate 15.1 closeout) — so the "close open promotion debt before
     the next mechanic-ladder gate" interlock does **not** block, and no debt-closure spec is
     needed first.
   - Gate 16 — Hearts is the **lowest non-`Done`** unit on the `specs/README.md` active-epoch
     tracker (Order 7), and its predecessor (Gate 15.1) is `Done`.
   - `docs/ROADMAP.md` §15 admits Gate 16 as ladder law.

2. **Variant: classic full Hearts ruleset.** The spec locks standard 4-seat Hearts: full
   52-card deal (13 cards/seat), pass-direction rotation (left → right → across → hold/no-pass),
   2♣ leads the first trick, follow-suit obligation, hearts cannot be led until "broken," no
   point cards on the first trick (with the standard exception handling), scoring heart = 1 and
   Q♠ = 13, **shoot-the-moon**, and a game-end points threshold (~100) with lowest cumulative
   score winning. **You must deep-research the canonical Hearts ruleset and research-pin the
   exact parameters** (pass-rotation cycle, exact "broken hearts" and first-trick exceptions,
   game-end threshold, tie-break, and the precise shoot-the-moon resolution) **inside the spec**,
   documenting any deliberate Rulepath deviation in the GAME-SOURCES-style notes the spec calls
   for. ROADMAP §15 Gate 16 explicitly names shoot-the-moon ("shoot-the-moon or scoped
   equivalent"), so the full ruleset — not a stripped slice — is the target.

3. **Trick-taking is the *second* close use, so no `game-stdlib` promotion here.** Trick-taking
   first appeared in `plain_tricks` (Gate 10.1, `local-only`). Hearts is the **second** close
   use; the third-use hard gate (per `docs/MECHANIC-ATLAS.md` §4) does **not** fire until the
   third (Oh Hell, Gate 17). Therefore the spec must:
   - **reopen and compare** the trick-taking and private-hand primitive-pressure ledger entries
     (`plain_tricks` ↔ Hearts) per the §9A armed interlock, recording the comparison and a
     **keep-local / defer** decision as a work-breakdown item plus a FOUNDATIONS-&-boundary
     stance, using the `templates/PRIMITIVE-PRESSURE-LEDGER.md` shape; and
   - **not** promote any `game-stdlib` trick/suit/hand helper, and not introduce any trick/suit/
     rank/hand noun into `engine-core`. (Hearts' negative scoring, shoot-the-moon, passing, and
     hearts-broken rules differ materially from Plain Tricks' plain-trick scoring; the expected
     decision is keep-local with the comparison documented — but you must do the comparison, not
     assume the outcome.)

4. **Deliverable = a single spec file.** The trick-taking ledger comparison lives **inside** the
   spec (as a work-breakdown item + the §8 FOUNDATIONS-&-boundary-alignment section), not as a
   separate downloadable document.

5. **Neutral game name — bounded delegation to you.** Rulepath ships original, evocative neutral
   names rather than the source game's name (River Ledger ← Texas Hold'Em, Crest Ledger ←
   poker, Plain Tricks, Masked Claims, Flood Watch). **Coin an original, IP-safe neutral name
   for this Hearts implementation** consistent with that catalog convention, and **derive the
   game module id (snake_case) and the spec filename slug from it.** Per `docs/IP-POLICY.md` §5,
   "Hearts" is a public-domain game name with no active card-game trademark, so keeping "Hearts"
   as the display name is *permissible*; the established Rulepath convention, however, is to coin
   a neutral name, so coin one and use it, documenting the naming rationale in the spec's
   source/IP notes. If you judge a neutral coinage genuinely unjustified, you may keep "Hearts" —
   but state the reasoning. `assumption:` the spec filename is
   `specs/gate-16-<neutral-slug>-trick-taking.md`; if you keep "Hearts," use
   `specs/gate-16-hearts-trick-taking.md`.

6. `assumption:` the deliverable is the **spec only** — it is *not* decomposed into `tickets/`
   AGENT-TASK packets. Ticket decomposition happens separately afterward via the repo's
   `/reassess-spec` then `/spec-to-tickets` workflow (`specs/README.md` "Workflow"). The spec's
   work breakdown should therefore enumerate **candidate** AGENT-TASK items with dependency
   order, as the sibling specs do, without writing the tickets themselves.

---

## 4. The task

Produce the **Gate 16 (Hearts) implementation spec** — a **new** roadmap-gate spec — that turns
`docs/ROADMAP.md` §15 "Gate 16: Hearts" into one concrete, reviewable, foundation-aligned plan.
This is a **new-spec** deliverable. The spec must (a) confirm-and-document the Gate 16
determination with the evidence in §3.1; (b) lock the classic full Hearts variant, research-
pinned; (c) define the new `games/<neutral_id>` crate, its filled official-game documents, and
its registration across tools/WASM/web-catalog surfaces; (d) scope the 4-seat private-hand
no-leak obligations to the N-seat contract; (e) reopen the trick-taking primitive-pressure
ledger as a keep-local/defer comparison against `plain_tricks`; and (f) map every deliverable
to the OFFICIAL-GAME-CONTRACT, MULTI-SEAT-AND-SURFACE-CONTRACT, TESTING, and AI-BOTS obligations,
following the `specs/README.md` 12-section spec format and mirroring the structure of
`archive/specs/gate-15-river-ledger-texas-holdem-base.md` and
`archive/specs/gate-10-1-plain-tricks-trick-taking-proof.md`.

---

## 5. Exploration & online-research mandate

Explore the repository as deeply as needed beyond the files listed above. **Research online as
deeply as needed** — the canonical Hearts ruleset and its common regional/house variants
(pass-rotation cycles, "no points on the first trick" edge cases, the Q♠-on-first-trick rule,
hearts-broken-to-lead timing, shoot-the-moon vs. shoot-the-sun, game-end thresholds, tie-breaks),
public-domain rules sources suitable for an original prose summary, prior open-source
trick-taking engine implementations and how they model follow-suit legality / trick resolution /
negative scoring, research or write-ups on Hearts bot strategy (without proposing any
MCTS/ISMCTS/Monte Carlo/ML/RL approach — those are forbidden in public v1/v2), and any
accessibility/UX prior art for presenting a 4-seat hidden-hand trick game. Cite sources for any
external claim that shapes a decision in the spec (variant parameters, naming/IP rationale,
strategy posture). The deep research is **your** job; do it thoroughly.

---

## 6. Doctrine & constraints (honor these in every spec decision)

- `docs/FOUNDATIONS.md` is the constitution — every product-behavior decision must satisfy its
  §11 universal invariants and clear its §12 stop conditions; a genuine divergence requires an
  accepted ADR superseding the affected principle first ("supersede only by accepted ADR"),
  never designing against it silently. The spec is **subordinate** to the foundation set and
  must not redefine or override any foundation contract.
- Authority order: foundation docs govern area docs govern specs govern tickets. Where the spec
  and a foundation document disagree, the foundation document wins.
- `engine-core` stays generic and **noun-free** — no `card`, `deck`, `hand`, `suit`, `rank`,
  `trick`, `board`, etc.; typed mechanic nouns belong in `games/*` first, shared helpers in
  `game-stdlib` only via the mechanic atlas. **No `game-stdlib` trick-taking promotion at
  Gate 16** (second use — keep local; see §3.3).
- **TypeScript never decides legality.** Legal actions, validation, effects, views, previews,
  and bot decisions all come from Rust/WASM.
- **No YAML and no DSL without an accepted ADR.** Static data is typed content / parameters /
  metadata only — never selectors, conditions, or triggers; no scoring or follow-suit formulas
  encoded in data.
- **Determinism**: deterministic shuffle/deal, replay, hashes, serialization order, and traces
  stay deterministic (or are explicitly migrated with trace notes).
- **No hidden-information leaks**: 4-seat private hands, pass choices in flight, and any
  unrevealed deck tail must not leak into payloads, action trees, previews, DOM, storage, logs,
  effect logs, bot explanations, candidate rankings, or replay exports — proven by the §8.1
  N-seat pairwise no-leak taxonomy.
- **No MCTS / ISMCTS / Monte Carlo / ML / RL bots** in public v1/v2. L0 random-legal is required;
  any L2 authored policy needs a strategy-evidence pack first.
- **No casino/real-money/trade-dress mimicry; original prose and assets only** (`docs/IP-POLICY.md`).
- **Never delete or weaken tests to get green** — follow the failing-test protocol
  (`docs/AGENT-DISCIPLINE.md` §4).
- The spec is **authored, not decomposed** (§3.6): enumerate candidate AGENT-TASKs; do not write
  tickets.

---

## 7. Deliverable specification

Produce exactly **one downloadable markdown document**:

- **`specs/gate-16-<neutral-slug>-trick-taking.md`** — a **new** file (not a replacement).
  `<neutral-slug>` is derived from the neutral name you coin (§3.5); fall back to
  `specs/gate-16-hearts-trick-taking.md` only if you keep "Hearts."

It MUST follow the **12-section spec format** defined in `specs/README.md` ("Spec format"):
1. Header (Spec ID, stage, gate, status `Planned`/`Not started`, date, owner, authority order;
   plus game-identity fields as the sibling specs carry — internal game id, public display name,
   variant id, trace rules version, browser-impl-required flag);
2. Objective (sourced from ROADMAP §15 Gate 16; cite the §3.1 determination evidence and the
   second-use trick-taking posture);
3. Scope (in / out / **not allowed** — carry ROADMAP §15's prohibitions verbatim);
4. Deliverables (the new `games/<neutral_id>` crate tree; the filled official-game documents
   from `templates/**`; registration across `simulate`/`replay-check`/`fixture-check`/
   `rule-coverage`, WASM, and the `apps/web` catalog);
5. Work breakdown (bounded **candidate** AGENT-TASK items with dependency order — including the
   trick-taking ledger-reopening item);
6. Exit criteria (mapped row-for-row to ROADMAP §15 Gate 16's exit list);
7. Acceptance evidence (command suite; the test taxonomy table; the **pairwise no-leak matrix**
   for all four seats; the golden-trace minimum set per OFFICIAL-GAME-CONTRACT §11 /
   TESTING §6; benchmark expectations);
8. FOUNDATIONS & boundary alignment (principles engaged; the trick-taking + private-hand
   primitive-pressure stance and keep-local/defer decision; §12 stop conditions);
9. Forbidden changes (gate-specific prohibitions);
10. Documentation updates required (the `specs/README.md` status flip; `docs/SOURCES.md`;
    `docs/MECHANIC-ATLAS.md` ledger rows; game-local docs; **`apps/web/README.md`** intro
    catalog list + Shell Surface renderer list + Smoke Layers `smoke:e2e` list, per
    OFFICIAL-GAME-CONTRACT §10/§12 and `scripts/check-catalog-docs.mjs`);
11. Sequencing (predecessor Gate 15.1 `Done`; successor Gate 17 Oh Hell; admission rule);
12. Assumptions (one-line-correctable).

Use explicit `not applicable` rows over silent omissions. You may include implementation
appendices (core rules, bot policy, replay/export model, WASM specifics, benchmark operations,
sources) as `gate-10-1-plain-tricks-trick-taking-proof.md` does, but keep them inside the single
spec file.

> Produce the deliverable directly as a downloadable markdown document. Do not interview, do not
> ask clarifying questions — the requirements above are final. If a genuine contradiction makes a
> requirement impossible, state it in the deliverable and proceed with the most faithful
> interpretation.

---

## 8. Self-check (run against your own output before returning)

- [ ] The spec **confirms-and-documents** Gate 16 as next with the §3.1 evidence; it does **not**
      re-open "what's next" or propose a different gate.
- [ ] It locks the **classic full Hearts variant** with exact parameters research-pinned and
      sourced (pass rotation, broken-hearts/first-trick rules, Q♠=13/heart=1, shoot-the-moon,
      game-end threshold, tie-break), and cites sources for each externally-derived parameter.
- [ ] It coins an original, IP-safe **neutral name** (or justifies keeping "Hearts") and derives
      the module id and spec filename slug from it.
- [ ] It reopens the **trick-taking + private-hand primitive-pressure ledger** as a `plain_tricks`
      ↔ Hearts comparison and records a **keep-local/defer** decision; it promotes **no**
      `game-stdlib` helper and adds **no** `engine-core` card/suit/trick/hand noun.
- [ ] It scopes **4-seat private-hand no-leak** to the MULTI-SEAT §6 / TESTING §8.1 pairwise
      taxonomy across every surface (views, action trees, previews, diagnostics, effects, bot
      explanations, replay exports, DOM, storage, logs, accessibility text, test IDs).
- [ ] It requires an **L0 random-legal bot** and gates any L2 policy behind a strategy-evidence
      pack; it forbids MCTS/ISMCTS/Monte Carlo/ML/RL.
- [ ] Every deliverable maps to OFFICIAL-GAME-CONTRACT §1/§3/§5/§10/§11/§12 and the
      `templates/**` set; the **12-section spec format** is fully present with `not applicable`
      used over silent omission.
- [ ] Documentation-updates (§10) names the `specs/README.md` flip, `docs/MECHANIC-ATLAS.md`
      rows, and the `apps/web/README.md` catalog/smoke surfaces.
- [ ] No spec decision weakens an upstream foundation doc or silently amends an accepted ADR.
- [ ] The deliverable is **one spec file**, authored (not decomposed into tickets).
- [ ] Commit `83e7161` contains every file named in §2 (the manifest is that tree).

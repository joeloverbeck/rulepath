# Deep-research brief — Rulepath doc & template overhaul, evidenced by 17 shipped games

You are ChatGPT-Pro running a **locked deep-research session**. Everything you need is in this
prompt plus the uploaded manifest. **Do not interview, do not ask clarifying questions** — the
requirements below are final. Produce the deliverable directly (see §7).

---

## 1. Context

The uploaded file `manifest_2026-06-22_db0c50b.txt` is the complete path inventory of the
`joeloverbeck/rulepath` repository — a Rust-first, rule-enforcing, replayable, testable
card/board-game platform where **Rust owns all behavior and TypeScript/React present only**.

The foundation docs are an ordered, layered authority indexed by `docs/README.md`:
`FOUNDATIONS.md` (the constitution) → `ARCHITECTURE.md` → `ENGINE-GAME-DATA-BOUNDARY.md` →
`OFFICIAL-GAME-CONTRACT.md` → `MECHANIC-ATLAS.md` → the area docs → `ROADMAP.md` → `IP-POLICY.md`
→ `AGENT-DISCIPLINE.md` → `SOURCES.md` → `WASM-CLIENT-BOUNDARY.md`. Earlier documents govern
later ones; accepted ADRs (`docs/adr/`) supersede a foundation doc only by explicitly naming the
affected sections.

**Fetch every file you read from commit `db0c50b95f84df12b349710033c77db2bf7326b3` (short
`db0c50b`)** — the manifest reflects exactly that tree. Read the file contents from the repository
at that commit (e.g. `https://raw.githubusercontent.com/joeloverbeck/rulepath/db0c50b95f84df12b349710033c77db2bf7326b3/<path>`).
The manifest is a **path inventory only** — it lists which files exist, not their contents; open
each path you need at the commit above. If any source you consult cites a different "commit of
record," note the divergence and use `db0c50b`, not the other string.

**The repository is mature.** The public mechanic ladder (Gates 0–14) and the public scaling-phase
Gates 15–17 are `Done`; **17 official games** are shipped under `games/*`:
`briar_circuit`, `column_four`, `directional_flip`, `draughts_lite`, `event_frontier`,
`flood_watch`, `frontier_control`, `high_card_duel`, `masked_claims`, `plain_tricks`,
`poker_lite`, `race_to_n`, `river_ledger`, `secret_draft`, `three_marks`, `token_bazaar`,
`vow_tide`. Gates 18–23 and Gate P are admitted roadmap rows but remain unwritten seeds
(see `specs/README.md`). The accumulated implementation experience of those 17 games is the
**evidence base** for this research.

---

## 2. Read in full (authority order)

Read these in full before producing. Order follows the `docs/README.md` authority index;
**every template named below is a target of the overhaul, not merely background.**

**Tier 1 — constitution & authority**
- `docs/README.md` — the authority order and the layering/decision-hierarchy rule that any doc change must preserve.
- `docs/FOUNDATIONS.md` — the constitution: product priority, §11 universal invariants, §12 stop conditions, §13 ADR triggers; every recommendation must satisfy these or propose an ADR.

**Tier 2 — boundary & reuse law (load-bearing for the code-reuse question)**
- `docs/ARCHITECTURE.md` — workspace shape, dependency direction, Rust/WASM boundary, action/view/effect/replay/determinism model.
- `docs/ENGINE-GAME-DATA-BOUNDARY.md` — the exact boundary between `engine-core`, `game-stdlib`, `games/*`, static data, and future DSL pressure; the home that any new shared scaffolding must respect.
- `docs/OFFICIAL-GAME-CONTRACT.md` — what makes a game official; the requirements-first workflow the templates operationalize (§10/§12 catalog-doc obligations included).
- `docs/MECHANIC-ATLAS.md` — the mechanic inventory, primitive-pressure ledger, **second-use comparison and third-use hard gate**, §10/§10A promotion + debt register, §9A armed interlocks. This is the governing doc for code reuse; the central reuse question turns on whether its promotion bar is calibrated correctly.

**Tier 3 — area docs (each a candidate for correction/merge)**
- `docs/AI-BOTS.md` — bot law, levels, hidden-information safety, Level 2 evidence workflow, explanations.
- `docs/UI-INTERACTION.md` — public visual target, legal-only interaction, previews, effect-driven animation, replay UI, accessibility.
- `docs/TESTING-REPLAY-BENCHMARKING.md` — test taxonomy, golden traces, deterministic replay/hash, no-leak tests, benchmarks, CI gates.
- `docs/MULTI-SEAT-AND-SURFACE-CONTRACT.md` — N-seat and larger-surface obligations; the doctrine the recent multi-seat games exercise hardest.
- `docs/TRACE-SCHEMA-v1.md` — trace schema; relevant because per-game trace/export shapes diverge (see code-seam findings).
- `docs/WASM-CLIENT-BOUNDARY.md` — Rust/WASM-to-browser client contract, replay safety, dev-panel whitelist.
- `docs/ROADMAP.md` — the staged ladder and the upcoming Gate 18–23 mechanics any reuse recommendation should anticipate.
- `docs/IP-POLICY.md` — public/private content policy, original prose/assets; bounds what templates may require.
- `docs/AGENT-DISCIPLINE.md` — coding-agent law: bounded tasks, forbidden changes, failing-test protocol; governs how the change-plan must be executable.
- `docs/SOURCES.md` — researched bibliography and Rulepath-specific lessons; the home for any new external references you cite.
- `docs/archival-workflow.md` — the archival workflow; assess whether it stays accurate as the doc set evolves.

**Tier 4 — ADRs**
- `docs/adr/0001`–`docs/adr/0007` and `docs/adr/ADR-TEMPLATE.md` — the accepted decisions the overhaul must not silently amend; an overhaul recommendation that touches their doctrine must propose a superseding ADR, not edit around them. Pay special attention to **ADR 0004** (hidden-info replay/export taxonomy — governs any view/export reuse) and **ADR 0007** (public scaling phase / Gate P tail — governs the roadmap framing).

**Tier 5 — the templates (all are overhaul targets)**
- `templates/README.md` — the template index, completion rule, N-seat adoption note, lifecycle order; the spine any template merge/removal must keep coherent.
- `templates/GAME-SOURCES.md`, `templates/GAME-RULES.md`, `templates/GAME-RULE-COVERAGE.md`, `templates/GAME-MECHANICS.md`, `templates/GAME-IMPLEMENTATION-ADMISSION.md`, `templates/AGENT-TASK.md`, `templates/GAME-HOW-TO-PLAY.md`, `templates/COMPETENT-PLAYER.md`, `templates/BOT-STRATEGY-EVIDENCE-PACK.md`, `templates/GAME-AI.md`, `templates/GAME-UI.md`, `templates/GAME-BENCHMARKS.md`, `templates/PRIMITIVE-PRESSURE-LEDGER.md`, `templates/PUBLIC-RELEASE-CHECKLIST.md` — each assessed for dead fields, missing fields, cross-template duplication, underuse, and divergence in how the 17 games actually filled them.

**Tier 6 — planning & delta context**
- `specs/README.md` — the living spec index/progress tracker; confirms Gates 0–17 `Done`, Gates 18–23 + P unwritten, and the interlock rule (atlas promotion debt closes before the next mechanic-ladder gate).
- `archive/reports/foundation-doc-realignment.md` — **the prior doc/template realignment that already shipped (Phase 0).** This is your delta baseline: build on it; do **not** re-recommend its shipped changes as if missing (see §3.5).
- `archive/reports/public-game-ladder-and-implementation-order.md` — the Gate 15+ public game ladder and implementation order; grounds which future mechanics the reuse recommendations should anticipate.

### Code seams to inspect directly (inspect, not read-fully)

Read these in the repo at `db0c50b` to gather the implementation evidence; do **not** paste them
into the deliverable.

- `crates/game-stdlib/**` — the only promoted shared helpers today: `board_space` (`Dimensions`/`Coord`/`Parity`) and `trick_taking` (`follow_suit_indices`, `winning_play_index`). The ground truth for "what is already shared."
- `crates/engine-core/src/lib.rs` (and the crate's modules) — the generic, **noun-free** kernel surface (`ActionTree`, `EffectEnvelope`, `StableSerialize`, `DeterministicRng`, `Actor`/`Viewer`, `VisibilityScope`, etc.). Establishes what scaffolding already exists vs. what each game re-implements.
- A representative spread of `games/*` source and their filled docs, to verify duplication and template-friction claims first-hand: `race_to_n` (tiny, 2-seat), `column_four`/`draughts_lite` (board / compound-action), `river_ledger` (3–6-seat hidden-info betting), `vow_tide`/`briar_circuit`/`plain_tricks` (trick-taking), `poker_lite`/`high_card_duel` (chance/hidden-info). For each, inspect `src/` (seat enums in `ids.rs`, `setup.rs`, `actions.rs`, `effects.rs`, `visibility.rs`, replay/stable-hash) and the filled `games/*/docs/` (`RULES.md`, `RULE-COVERAGE.md`, `MECHANICS.md`, `UI.md`, `AI.md`, `BENCHMARKS.md`, `PRIMITIVE-PRESSURE-LEDGER.md`, etc.).

---

## 3. Settled intentions (final — do not re-open)

These decisions are already made. They are why this session is locked.

1. **Deliverable = ONE consolidated, prioritized change-plan document** (see §7 for the exact
   spec) — *not* rewritten foundation/template files. Each recommendation is an actionable
   change-plan entry, not a finished replacement file.

2. **Each recommendation must cite concrete game-implementation evidence.** Name the specific
   file(s)/symbol(s)/template field(s) across `games/*` (and their `games/*/docs/`) that
   demonstrate the gap, friction, duplication, or dead field. A recommendation with no grounded
   evidence from the shipped games does not belong in the plan. Where a claim of duplication
   drives a promotion recommendation, you must have **verified it against the live code at
   `db0c50b`** — not inferred it (see §3.6).

3. **Code-reuse moves are in scope, alongside doc/template text changes.** You may recommend
   specific `game-stdlib` promotions and propose **new shared scaffolding**, but every such move
   must be expressed as a **doc-governed directive** (which doc authorizes it, which boundary it
   respects) and must stay **Rust-typed** — no YAML, no DSL, no data-driven rule behavior
   (FOUNDATIONS non-negotiable; `ENGINE-GAME-DATA-BOUNDARY.md`).

4. **You may revise the doctrine itself, with justification.** You may recommend:
   - changing or recalibrating the **atlas third-use hard gate** and its promotion bar;
   - introducing a **distinct reuse category for "mechanical scaffolding"** (e.g. per-seat enum
     boilerplate, effect-envelope wrappers, `StableSerialize`/stable-hash plumbing, view-projection
     skeletons, setup/seat-count validation, deterministic shuffle/RNG plumbing) that is *not* a
     behavioral mechanic and therefore is not well-served by the mechanic-atlas third-use gate —
     deciding where in the doc set such a category should live and how it should be governed;
   - **merging, removing, splitting, or adding** foundation docs and templates.
   Every such change must be **explicitly argued, never silent**, must **not weaken an upstream
   FOUNDATIONS §11 invariant or §12 stop condition**, and must **not silently amend an accepted
   ADR** — where doctrine genuinely must change, recommend a superseding ADR (per FOUNDATIONS
   §13 / `docs/README.md` decision hierarchy) as part of the plan.

5. **This is a delta on already-shipped realignment, not a cold start.** The Phase-0 realignment
   in `archive/reports/foundation-doc-realignment.md` already shipped (multi-seat contract,
   N-seat template fields, ROADMAP scaling phase, ADR 0007). **Do not re-recommend that shipped
   work as if it were missing.** Build on it; improve only where the *17 games' implementation
   experience* shows it falls short. State the implemented baseline you are building on.

6. **Verify before promoting; respect or explicitly argue against the hard gate.** The current
   doctrine deliberately *rejects* promoting several repeated shapes (e.g. deterministic
   shuffle/private-hand/staged-reveal across 7+ games; public resource accounting) because the
   per-game specifics differ. Do not blanket-promote on raw repetition counts. For each reuse
   recommendation, either (a) show it clears the existing bar, or (b) make an explicit, argued
   case to change the bar (per intention 4). Distinguish **genuinely shareable mechanical
   scaffolding** (mechanical, behavior-free) from **behavioral mechanics** the gate intentionally
   keeps local.

7. **`assumption:`** The change-plan *names* game-retrofit opportunities (existing games adopting
   newly-shared scaffolding) as follow-on work, with rough scope, but does **not** itself execute
   or fully spec those game refactors. (User-correctable.)

8. **`assumption:`** Recommendations are **prioritized** (e.g. high / medium / low, by leverage on
   future-game ease + robustness + reuse), and — where useful — tied to the specific upcoming
   gates (18 Spades/partnerships, 19 Five Hundred Rummy/meld tableau, 20 Halma/board topology,
   21 Pachisi/track topology, 22 Four Winds/reaction windows, 23 capstone) that would benefit, so
   the plan is execution-ready via the repo's spec → ticket workflow. (User-correctable.)

---

## 4. The task

This is a **foundational / doc-overhaul** research task. Using the 17 shipped games as the
evidence base, comprehensively assess the **entire foundation doc set** (`docs/**`, all 15 docs +
7 ADRs + `archival-workflow.md`) and the **entire template set** (`templates/**`, all 15 files)
and produce a single, prioritized, evidence-grounded **change-plan** whose execution will make
future game implementations (Gates 18–23 and beyond) **easier, more solid, more robust, and built
on as much appropriate code reuse as the doctrine warrants.** Identify gaps, corrections,
duplications, dead/missing fields, staleness, and merge/removal/addition opportunities across the
docs and templates — and the code-reuse moves (`game-stdlib` promotions, new shared scaffolding)
that the doc/template changes should authorize and govern. Removal, merging, correction, and
addition of foundational docs and templates are all permitted where warranted, each justified.

---

## 5. Exploration & online-research mandate

Explore the repository as deeply as needed beyond the files listed in §2 — open any `games/*`
module, any `games/*/docs/` filled template, any `crates/*`, `tools/*`, or spec/ticket/archive
file that sharpens a recommendation.

Research online as deeply as needed — comparable open-source game-engine / rules-engine
architectures, board-game-framework designs (e.g. how mature frameworks separate generic kernels
from per-game rules, how they organize shared "scaffolding" vs. game-specific behavior, how they
template new-game onboarding), determinism/replay and hidden-information modeling literature,
documentation-system and requirements-traceability practice, and any research papers or prior art
on primitive-extraction / "rule of three" promotion discipline. Use external prior art to
pressure-test whether Rulepath's promotion bar, doc layering, and template set match or improve on
how comparable systems scale to many games. **Cite every external source that shapes a
recommendation.** The deep online research is yours to perform here — do not defer it.

---

## 6. Doctrine & constraints (honor these in every recommendation)

- `docs/FOUNDATIONS.md` is the constitution — every product-behavior decision must satisfy its
  §11 universal invariants and clear its §12 stop conditions; a genuine divergence requires an
  accepted ADR superseding the affected principle first (FOUNDATIONS "supersede only by accepted
  ADR"), never designing against it silently.
- **Authority order** flows downward: foundation docs govern area docs govern templates/specs
  govern tickets. A recommendation must never make a lower artifact contradict a higher one, and
  must not invalidate a cross-reference in a lower doc/spec without flagging the cascade.
- `engine-core` stays generic and **noun-free** — no `board`, `card`, `deck`, `grid`, `hand`, etc.
  Typed mechanic nouns belong in `games/*` first; shared helpers live in `game-stdlib` and only via
  the mechanic atlas (or, if you propose a new scaffolding category, via the governing home you
  define — without breaking the noun-free kernel).
- **TypeScript never decides legality.** Legal actions, validation, effects, views, and bot
  decisions all come from Rust/WASM.
- **No YAML and no DSL without an accepted ADR.** Static data is typed content / parameters /
  metadata only — never selectors, conditions, or triggers. Any reuse recommendation stays Rust-typed.
- **Determinism**: replay, hashes, RNG, serialization order, and traces stay deterministic (or are
  explicitly migrated). Shared scaffolding for shuffle/RNG/serialization must preserve this.
- **No hidden-information leaks** into payloads, DOM, storage, logs, effect logs, bot explanations,
  or replay exports (ADR 0004 taxonomy). Any shared view/projection/export scaffolding must be
  leak-safe by construction.
- **No MCTS / ISMCTS / Monte Carlo / ML / RL bots** in public v1/v2.
- **Never weaken tests, coverage, or no-leak/replay proofs to ease implementation** — follow the
  failing-test protocol (`AGENT-DISCIPLINE` §4). Reuse must not erode the test taxonomy.
- The deliverable is **advisory**: it recommends changes; it does not itself rewrite repository law.

---

## 7. Deliverable specification

Produce **one downloadable markdown document**:

**Filename:** `RULEPATH-DOC-AND-TEMPLATE-CHANGE-PLAN.md` — **new** (does not replace an existing
repo file; it is an advisory change-plan the user will execute via the repo's spec/ticket workflow).

It must contain, in order:

1. **Executive summary** — the highest-leverage changes and the central thesis on code reuse
   (including your position on whether the atlas promotion bar is correctly calibrated and whether a
   distinct "mechanical scaffolding" reuse category is warranted).
2. **Method & evidence base** — the commit (`db0c50b`), which games/docs/templates you inspected,
   and how you verified duplication/friction claims against live code; list external sources consulted.
3. **Implemented-baseline acknowledgment** — what the Phase-0 realignment already shipped, which you
   are building on and will not re-recommend (per §3.5).
4. **The change-plan**, organized in clearly separated parts:
   - **Part A — Foundation docs** (`docs/**` incl. ADRs and `archival-workflow.md`);
   - **Part B — Templates** (`templates/**`);
   - **Part C — Code-reuse moves** (`game-stdlib` promotions, proposed new shared scaffolding, and
     the doc/template changes that authorize and govern them);
   - **Part D — Doctrine changes** (any recalibration of the third-use gate, new reuse category,
     merges/removals/additions of docs or templates, and any superseding ADRs to author).
   Present each recommendation as a self-contained entry with these fields:
   **`ID` · `Target file(s)` · `Type` (correct / add / remove / merge / split / clarify / promote /
   new-scaffolding / ADR) · `Evidence` (specific game files/symbols/template fields proving the
   need) · `Proposed change` (the exact edit/addition/removal, concretely) · `Rationale`
   (how it makes future games easier/solider/more reuse-driven) · `Doctrine check` (which FOUNDATIONS
   invariant / ADR / boundary it engages and how it stays compliant) · `Priority` (high/medium/low) ·
   `Benefiting gates` (which of Gates 18–23+ it most helps, where applicable) · `Follow-on`
   (any game-retrofit work it implies, named not executed).**
5. **Prioritized execution order** — a suggested sequence (honoring the repo's interlock rule that
   atlas promotion debt closes before the next mechanic-ladder gate), grouped so the user can turn
   batches into specs/tickets.
6. **Risks & explicitly-rejected ideas** — reuse you considered and rejected (with why), and any
   recommendation that carries determinism/leak/boundary risk to watch.

> Produce the deliverable directly as a downloadable markdown document. Do not interview, do not ask
> clarifying questions — the requirements above are final. If a genuine contradiction makes a
> requirement impossible, state it in the deliverable and proceed with the most faithful
> interpretation.

---

## 8. Self-check (run before returning)

- Every recommendation cites **specific** game-implementation evidence (file/symbol/field) at
  `db0c50b`; no ungrounded suggestions.
- Every duplication-driven promotion was **verified against live code**, not inferred from counts;
  shapes the doctrine intentionally keeps local are either left local or have an **explicit argued
  case** to change the bar.
- No recommendation weakens a FOUNDATIONS §11 invariant or §12 stop condition, introduces YAML/DSL
  or data-driven rule behavior, breaks the noun-free `engine-core`, risks a hidden-info leak
  (ADR 0004), or silently amends an accepted ADR — any genuine doctrine change is paired with a
  superseding-ADR recommendation.
- The Phase-0 shipped realignment is acknowledged and **not** re-recommended as missing.
- The deliverable is exactly one markdown document named `RULEPATH-DOC-AND-TEMPLATE-CHANGE-PLAN.md`,
  matching the §7 structure, with prioritized, execution-ready entries.
- The sweep is **comprehensive**: every one of the 15 foundation docs, 7 ADRs, `archival-workflow.md`,
  and 15 templates is either given recommendations or explicitly marked "no change needed, and why."
- Every external claim that shaped a recommendation is cited.
- Commit `db0c50b` contains every file named in §2 (the manifest reflects that tree).

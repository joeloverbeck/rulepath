# Research brief — Shared cross-game "How to Play / Rules" surface for the Rulepath web app

> **Session 2 (you, ChatGPT-Pro): this is a locked brief.** Everything you need is in
> this prompt plus the uploaded manifest. Do not interview or ask clarifying questions —
> the requirements below are final. Produce the deliverable directly as a downloadable
> markdown document. The interview that fixed these decisions already happened; re-opening
> them violates the contract.

---

## 1. Context

The uploaded file `manifest_<TODAY>_<SHORTSHA>.txt` (see the exact filename at the end of
this section) is the path inventory of the `joeloverbeck/rulepath` repository — a Rust-first,
rule-enforcing, replayable, testable card/board-game platform where **Rust owns all behavior
and TypeScript/React present only**. The foundation docs form an ordered, layered authority
indexed by `docs/README.md`: `FOUNDATIONS.md` (the constitution) → `ARCHITECTURE.md` →
`ENGINE-GAME-DATA-BOUNDARY.md` → the area docs → `ROADMAP.md`. Earlier documents govern later
ones; accepted ADRs (`docs/adr/`) supersede them only by explicitly naming the affected
sections.

**Fetch every file from commit `e6f4706` (`e6f47067116df34fe9ed89293bd2d4aa7e267b7b`, branch
`main`).** The uploaded manifest reflects exactly that tree. If any file you open cites a
different "commit of record," that is that document's own historical baseline — note the
divergence in your output and use commit `e6f4706`, not the cited string.

**Upload this manifest file:** `manifest_2026-06-09_e6f4706.txt`.

---

## 2. Read in full (authority order)

Read these completely, in this order, before producing anything. Each is load-bearing for
this specific target — a player-facing, cross-game rules-display surface.

```
docs/README.md — the foundation authority index and the layering rule (earlier docs govern later).
docs/FOUNDATIONS.md — the constitution: §2 behavior authority (Rust owns behavior; TS presents only), §7 public-UI-is-central-product, §11 universal acceptance invariants, §12 stop conditions, §13 ADR triggers. Every part of the deliverable must satisfy these.
docs/ARCHITECTURE.md — §2 dependency direction (apps/web reaches games only through the wasm-api boundary), §3 ownership table, §10 WASM API shape, §11 game-module shape (the per-game docs/ folder convention).
docs/ENGINE-GAME-DATA-BOUNDARY.md — §5 allowed static data, §11 UI-metadata boundary, §12 explanation-template boundary. Decides what player-rules content may legitimately be static vs. Rust-owned, and what must never become behavior/DSL.
docs/OFFICIAL-GAME-CONTRACT.md — §5 the RULES.md original-prose contract, §7 mechanic inventory, §10 UI-exposure requirements. This is where a NEW per-game player-facing rules-doc obligation must attach.
docs/UI-INTERACTION.md — §1–3 public-UI target / visual direction / ownership split, §5 browser-payload rules, §12–13 hidden-information safety + dev-inspector boundary, §16 accessibility baseline. Governs where and how the rules surface lives.
docs/WASM-CLIENT-BOUNDARY.md — §2 operation groups (current wasm-api operations), §4 developer-panel safety. The seam where a Rust/WASM-served rules operation would attach if you recommend that delivery path.
docs/IP-POLICY.md — §2 public-forbidden (no copied rulebook prose/assets), §4 public-rules-documentation requirements, §5 neutral/common names. Governs the new player-facing prose: original-only, IP-safe.
docs/ROADMAP.md — §1 stage/gate crosswalk, §2 per-stage requirements. To position this cross-game UI-infrastructure work relative to the mechanic-ladder gates (it is NOT a per-game mechanic gate).
docs/AGENT-DISCIPLINE.md — §2 required task packet (forbidden-changes declaration), §4 failing-test protocol, §5 kernel-change protocol. The eventual tickets must conform.
templates/README.md — the template index and what each per-game template is for.
templates/GAME-RULES.md — the current (formal, rule-ID-driven) rules-doc template; the baseline you decide to amend or sit a new template beside.
templates/COMPETENT-PLAYER.md — the existing strategy-guide template; confirm the new player-facing rules doc is distinct from this (rules tutorial ≠ strategy guidance).
templates/GAME-UI.md — the per-game UI template; relevant to how a rules affordance is documented per game.
templates/AGENT-TASK.md — the bounded task packet format the spec's tickets will follow.
games/poker_lite/docs/RULES.md — the pilot game's actual formal rules doc; the raw material to adapt into player-facing prose and to demonstrate the formal-vs-player gap concretely.
games/poker_lite/docs/COMPETENT-PLAYER.md — the pilot's strategy doc; shows what already exists and what the new player rules must NOT duplicate.
specs/README.md — the living spec index, progress tracker, and the canonical spec format your deliverable must mirror.
```

**Code seams to inspect directly** *(read in the repo to ground the design — do NOT treat as
the authority read-list above; do not paste them wholesale):*

- `crates/wasm-api/src/lib.rs` — the hardcoded game catalog (`list_games()`), the `GAME_*`
  display-name constants, and current operation surface; where a `get_rules`-style op or
  catalog metadata extension would land.
- `apps/web/src/components/GamePicker.tsx` and `apps/web/src/components/MatchSetup.tsx` —
  the pre-play surfaces where a "How to Play" affordance naturally lives.
- `apps/web/src/wasm/client.ts` — the `GameCatalogEntry` type and the JS↔WASM call shape.
- `apps/web/src/main.tsx`, `apps/web/src/components/AppShell.tsx`,
  `apps/web/src/state/shellReducer.ts` — app bootstrap, catalog loading/dispatch, and shell
  state, to find the in-play help entry point and how the catalog reaches React.
- `apps/web/vite.config.ts`, `apps/web/package.json`, `apps/web/public/` — the build pipeline
  and static-asset handling, decisive for the static-bundled delivery option and any new
  build/sync step.
- `scripts/check-catalog-docs.mjs` — the existing catalog↔docs CI invariant; the pattern a new
  "every catalog game has player rules" sync check should follow.
- `games/*/docs/` across all nine games (`column_four`, `directional_flip`, `draughts_lite`,
  `high_card_duel`, `poker_lite`, `race_to_n`, `secret_draft`, `three_marks`, `token_bazaar`)
  — confirm the uniform `docs/RULES.md` path/structure and the de-facto per-game doc set, since
  the new player-doc obligation applies uniformly to all nine.

---

## 3. Settled intentions (decisions already made — do not re-open)

These were resolved in interview. State them in your spec as committed decisions, not options.
They pre-empt every clarifying question you might ask:

1. **What to build:** a single shared, cross-game "How to Play / Rules" surface in `apps/web`,
   available for **every** game in the catalog, with **no LLM / no runtime text generation** —
   content is authored, version-controlled, and rendered.

2. **Content model — LOCKED: a new player-facing rules doc per game.** The existing
   `games/<id>/docs/RULES.md` is a *formal specification* (stable rule IDs such as
   `CL-PLEDGE-001`, tables with "Rust-owned validation notes" and visibility-enforcement
   columns) and is **not** suitable for player display. The spec therefore mandates a **new,
   uniform, player-facing rules document per game** (working name e.g. `HOW-TO-PLAY.md` — you
   finalize the name), authored in **original prose** under each game's `docs/`. The web surface
   renders **that** document, never `RULES.md` directly. This new doc is **distinct from
   `COMPETENT-PLAYER.md`**: it teaches the rules / how a turn works / how you win, not strategy.
   It must remain IP-safe per `docs/IP-POLICY.md` (original Rulepath prose, neutral names, no
   copied rulebook text) and must never leak hidden information — for hidden-information games
   (e.g. `poker_lite`, `secret_draft`, `high_card_duel`), the player rules describe the ruleset
   from the player's own perspective and never expose opponents' secret state.

3. **Delivery mechanism — YOU RECOMMEND, with a default and justification.** The *source of
   truth* is fixed (the new per-game player doc); the open question is how that text reaches the
   browser. Evaluate at least these two paths against the boundary doctrine and pick one:
   - **(a) Static-bundled markdown** — a build step copies the per-game player docs into
     `apps/web` (e.g. `public/rules/<id>.md` or a typed import), TS fetches/renders. Treated as
     allowed static UI content; no `wasm-api` change; requires a CI sync check so no catalog
     game can ship without player rules, plus a staleness guard.
   - **(b) Rust/WASM-served** — a new viewer-safe `wasm-api` operation (e.g. `get_rules`)
     returns the player text/structured sections; TS renders. Stronger single-source-of-truth
     and boundary alignment; more Rust work; grows the WASM payload.
   Justify the choice explicitly against `docs/FOUNDATIONS.md` §2/§11,
   `docs/ENGINE-GAME-DATA-BOUNDARY.md` §5/§11/§12, and `docs/WASM-CLIENT-BOUNDARY.md` §2, and
   list the doc amendments each path implies (see §7). Note for the static path: static UI
   content is permitted by the boundary doctrine *only* as inert presentation text — it must
   carry no selectors, conditions, triggers, or anything that could be read as behavior.

4. **Scope — LOCKED: mechanism + player content for all nine existing games.** The deliverable
   covers the shared surface AND the obligation to author player-facing rules for every one of
   the nine games listed in §2's code-seams block, with **`poker_lite` (Crest Ledger) as the
   fully worked pilot** — the spec includes the actual drafted player-facing rules text for
   `poker_lite` as the reference example, and defines the remaining eight as a repeatable,
   bounded per-game authoring task (template-driven) rather than ad-hoc.

5. **UI placement & interaction pattern — DELEGATED to your research.** Design the affordance
   and pattern from usability evidence (board/card-game help patterns: player aids /
   recognition-over-recall, progressive disclosure, dedicated rules panel vs. tooltips,
   modal vs. drawer; access from the game picker, match setup, AND in-play; accessibility per
   `docs/UI-INTERACTION.md` §16 — focus management, keyboard access, contrast, screen-reader
   summaries). Cite the sources that shape your recommendation.

6. **Boundary guardrails (non-negotiable):** TypeScript presents only and never decides
   legality; no hidden-information leak into payloads, DOM, `data-testid`, storage, logs, effect
   logs, replay exports, or the rules surface; content stays deterministic/static (no DSL, no
   YAML behavior); `engine-core` stays noun-free; the eventual tickets declare forbidden changes
   per `docs/AGENT-DISCIPLINE.md` §2.

`assumption: the deliverable is a SINGLE spec document` — the docs/ amendment list, the
templates/ decision, and the extra-suggestions section live as sections **within** the one
spec file, not as separate downloadable files. (User-confirmed; noted here so you treat it as a
default, not an open question.)

---

## 4. The task

**Target type: new-spec.** Produce one implementation spec (a downloadable markdown document)
that specifies a shared, cross-game, LLM-free "How to Play / Rules" surface for the Rulepath
web app — covering the new per-game player-facing rules-doc contract, a recommended-and-justified
delivery mechanism, a researched UI/UX design, the per-game authoring obligation for all nine
games with `poker_lite` as a fully drafted pilot, testing/CI guards, and the exact downstream
docs/ and templates/ changes. The spec must be drop-in for this repo's `specs/` pipeline:
subordinate to the foundation docs, mirroring the canonical spec format, and decomposable into
`templates/AGENT-TASK.md` tickets.

---

## 5. Exploration + online-research mandate

Explore the repository as deeply as needed beyond the files listed above — open any game's
`docs/`, any web-shell component, or any tool you need to ground a decision. Research online as
deeply as needed: in-app "how to play" / rules-screen UX patterns, board- and card-game
onboarding and player-aid research, progressive-disclosure and dedicated-panel-vs-tooltip
guidance, accessible modal/drawer patterns, and any prior art for surfacing version-controlled
game documentation in a web client without runtime generation. **Cite every external source that
shapes a recommendation.** The repository is the authority on what is allowed; outside research
informs the *how*, never overrides the doctrine.

---

## 6. Doctrine & constraints (honor all that the target engages)

- `docs/FOUNDATIONS.md` is the constitution — every product-behavior decision must satisfy its
  §11 universal invariants and clear its §12 stop conditions; a genuine divergence requires an
  accepted ADR superseding the affected principle first, never designing against it silently.
- Authority order: foundation docs govern area docs govern specs govern tickets. If a proposed
  design conflicts with architecture or foundation, the design is wrong.
- **TypeScript never decides legality.** Legal actions, validation, effects, views, and bot
  decisions all come from Rust/WASM. The rules surface is pure presentation of authored text.
- `engine-core` stays generic and **noun-free** (no `board`, `card`, `deck`, `grid`, `hand`).
  Nothing in this feature introduces mechanic nouns into the kernel.
- **No YAML and no DSL without an accepted ADR.** Static rules content is inert presentation
  text/metadata only — never selectors, conditions, or triggers.
- **Determinism:** the feature must not perturb replay, hashes, RNG, serialization order, or
  traces; rendering authored text is side-effect-free.
- **No hidden-information leaks** into payloads, DOM, storage, logs, effect logs, bot
  explanations, replay exports, or the rules surface itself.
- **No copied rulebook prose or proprietary assets** (`docs/IP-POLICY.md`); player rules are
  original Rulepath prose using neutral game names.
- **Never delete or weaken tests to get green** — the spec's testing section adds coverage; it
  does not relax existing gates (AGENT-DISCIPLINE §4).

---

## 7. Deliverable specification

Produce **one downloadable markdown document**: a **new** implementation spec named in this
repo's convention (a cross-game UI-infrastructure spec — e.g. `rules-display-shared-surface.md`
or a short descriptive slug; it is **not** a `gate-N` mechanic gate, so do not number it as one).
It does not replace any existing file. Mirror the canonical spec format described in
`specs/README.md` (goal/scope/non-goals, decisions, design, testing, acceptance/exit criteria,
explicit `not applicable` rows over silent omissions).

The single spec must contain, as explicit sections:

1. **Goal, scope, non-goals** — the shared surface; all nine games in scope; LLM-free; explicit
   non-goals (e.g. not a strategy guide, not a tutorial mode/replayable walkthrough unless you
   justify it).
2. **Content model & the new per-game player-rules-doc contract** — the new doc's purpose,
   required sections, original-prose/IP rules, hidden-information rules, and how it differs from
   `RULES.md` and `COMPETENT-PLAYER.md`. State the chosen filename.
3. **Delivery mechanism** — your recommendation (static-bundled vs. Rust/WASM-served) with the
   justification against the cited boundary sections, the default, and the rejected option's
   tradeoffs.
4. **UI/UX design** — the researched affordance, placement (picker / setup / in-play), the
   panel/drawer/modal pattern, and accessibility, with citations.
5. **Per-game authoring plan** — the repeatable task for all nine games, **plus the fully
   drafted player-facing rules text for `poker_lite` (Crest Ledger)** as the worked pilot/example.
6. **Testing & CI** — the sync/coverage guard (no catalog game ships without player rules,
   modeled on `scripts/check-catalog-docs.mjs`), a UI smoke test for the surface, and any
   staleness guard; no relaxation of existing gates.
7. **Acceptance / exit criteria** — concrete, evidence-based.
8. **`docs/**` amendment list** — name each foundation doc to amend and the exact additions
   (expect at least: `OFFICIAL-GAME-CONTRACT.md` for the new per-game player-doc obligation;
   `UI-INTERACTION.md` for the rules affordance in the public-UI target; `ARCHITECTURE.md`
   ownership/WASM-shape if a new op is chosen; `WASM-CLIENT-BOUNDARY.md` if Rust/WASM-served;
   `ENGINE-GAME-DATA-BOUNDARY.md` if static content is formalized). Tie each amendment to the
   delivery path you chose.
9. **`templates/**` decision** — an explicit ruling: a NEW template (e.g.
   `GAME-HOW-TO-PLAY.md`) vs. amending `GAME-RULES.md`, with rationale, and the template body
   or the precise amendment.
10. **Claude's extra suggestions** — adjacent improvements you surfaced (e.g. catalog
    short-descriptions, game-local glossary tooltips, first-run onboarding, localization
    readiness), each flagged clearly as *beyond the core ask* so the user can accept or defer
    them separately. Do not inflate the core scope with these.

Roadmap slotting: mention only briefly (one short note on where this cross-game infrastructure
sits relative to the gate ladder) — it is **not** a required workstream and must not become one.

> Produce the deliverable directly as a downloadable markdown document. Do not interview, do not
> ask clarifying questions — the requirements above are final. If a genuine contradiction makes a
> requirement impossible, state it in the deliverable and proceed with the most faithful
> interpretation.

---

## 8. Self-check (run before returning)

- The deliverable is exactly **one** spec markdown document matching §7; the docs/ amendment
  list, templates/ decision, and extra-suggestions are sections within it.
- Every settled intention in §3 is reflected as a committed decision, not re-opened as a question.
- The content model surfaces a NEW player-facing per-game doc, never `RULES.md` as-is, and the
  `poker_lite` pilot text is actually drafted (not just described).
- The delivery-mechanism recommendation is justified against the cited boundary sections, with
  its doc-amendment implications listed.
- No proposed design weakens an upstream foundation doc or silently amends an accepted ADR;
  divergence, if any, is called out and routed through an ADR.
- No hidden-information leak and no IP-unsafe prose anywhere in the design or the pilot text.
- Every external claim that shaped a decision is cited.
- The §1 fetch-baseline commit `e6f4706` contains every file named in the §2 read-in-full list
  (it does, as of authoring); flag any path that fails to resolve at that commit.

# Brief template & target-type reads

This file defines (A) the canonical anatomy of the emitted ChatGPT-Pro prompt and (B) the
research-target → load-bearing-reads map. The SKILL.md flow references both.

---

## A. Canonical brief anatomy

The emitted file `reports/<topic>-research-brief.md` is the *prompt the user pastes into
ChatGPT-Pro Session 2*. It is self-contained: Session 2 sees this prompt, the uploaded
manifest, and any out-of-band inputs the brief explicitly enumerates (e.g. uploaded source
documents — see SKILL "Out-of-band / private-IP-subject briefs"). Use these eight sections,
in order, as the backbone. Scale each to the target; omit a section only when genuinely N/A
and say so — and you MAY *add* target-specific sections beyond the eight when the target
warrants (e.g. an inputs/how-to-use preamble for out-of-band uploads, or an IP-discipline
section for a private-IP subject; see "Optional target-specific sections" below).

### 1. Context

One or two sentences. Begin with the manifest pointer, then repo identity, then the **exact
fetch-baseline commit** Session 2 must read every file from (the verified repo HEAD per the
Step 6 baseline-commit rule — never a commit string copied from a report without confirming it
contains the §2 read-list):

> The uploaded manifest is the path inventory of the `joeloverbeck/rulepath` repo —
> a Rust-first, rule-enforcing, replayable, testable card/board-game platform where **Rust
> owns all behavior and TypeScript/React present only**. The foundation docs are an ordered,
> layered authority indexed by `docs/README.md`: `FOUNDATIONS.md` (the constitution) →
> `ARCHITECTURE.md` → `ENGINE-GAME-DATA-BOUNDARY.md` → the area docs → `ROADMAP.md`; earlier
> documents govern later ones, and accepted ADRs supersede them only by explicitly naming the
> affected sections. Fetch every file from commit `<HEAD>` — the manifest reflects that tree.
> (If a referenced report cites a different "commit of record," note the divergence here and
> use the verified HEAD, not the report's string.)

### 2. Read in full (authority order)

An explicit, tiered path list — every file Session 2 must read before producing — each
with a one-line reason it is load-bearing *for this target* (when a volunteered floor is a
directory/glob like `docs/**`, give per-file reasons for the load-bearing files and one
directory-level reason for the incidental remainder — see SKILL Step 2). Built from Step 2 exploration.
Order by the `docs/README.md` authority index. Example shape:

```
Read these in full, in this order:

docs/README.md — authority order and the layering rule.
docs/FOUNDATIONS.md — the constitution: priority order, §11 universal invariants, §12 stop conditions, §13 ADR triggers; every deliverable must satisfy these.
docs/<area>.md — <subsystem contract / law this target depends on>.
specs/README.md — the living spec index and progress tracker; <which gate this target sits at>.
specs/<gate>.md — <sibling spec this target extends or realigns>.
docs/adr/NNNN-<slug>.md — <accepted decision this target builds on or must not silently amend>.
reports/<report> — <prior finding-set this target builds on>.
archive/specs/<spec> — <completed work that established the current state>.
```

After the read-in-full path list, add a short **Code seams to inspect directly** block —
the files/modules in `crates/`, `games/`, `tools/`, or `apps/web/` that Session 2 should read
*in the repo* but that are **not** pasted and are **not** part of the read-in-full set (this
is the SKILL Step 2 "relevant code seams" output). Keep it a brief bulleted/inline list with a
one-line reason each; mark it *inspect, not read-fully* so it reads as distinct from the §2
authority list above.

### 3. Settled intentions

The decisions the interview resolved — the heart of why Session 2 is *locked*. State each
as a committed decision, not an option. This section pre-empts every clarifying question
Session 2 might otherwise ask. Carry any early-exit gaps — or a minor residual ambiguity
surfaced at the Step 5 approval gate — here as `assumption: <X>` lines so they read as
defaults the user can override, not as open questions.

When a prior brief/report cycle on the same subject has already been **implemented**, state
the implemented baseline here and lock a decision that Session 2 must **not re-recommend
already-shipped work as if missing** — build on it, improve where it falls short, do not
rebuild. The same applies to a **predecessor / sibling unit in a serial lane** (wave N→N+1,
sequential gates — e.g. the `8C-R*` waves) even when no same-subject prior brief exists: its
shipped surfaces (the named pilots / receipts in the relevant register) are the implemented
baseline to build on, not re-propose. This is the sharp-delta sub-case of framing the task as a
delta (SKILL Step 2); §B recognizes this serial-unit class for the read-set. When the
immediate sibling sits in a **different mechanic family** than the target (a ladder pivot —
e.g. trick-taking → rummy), mirror it for section anatomy and any governance/process delta
(e.g. the `forward-v1` audit ordinal), but state the **mechanic** implemented baseline from
the nearest *earlier* same-family siblings — and forbid carrying over the immediate sibling's
family-specific surfaces or helpers (a trick-taking → rummy gate reuses no trick-taking helper).
The same split applies to other load-bearing delta axes — most sharply the **information model**:
if the target's hidden-information posture differs from the immediate sibling's (e.g. hidden-hand
gates → a perfect-information board game), flag the shift in §3 and re-derive its consequences
(all-surfaces-public no-leak audit, hidden-info export ADR `not applicable`, bounded
perfect-information search bots permissible) rather than mirroring the sibling's posture.

### 4. The task

A precise statement of what Session 2 must achieve — the goal behind the deliverable. One
tight paragraph. Name the target type (new spec / thorny fix / hardening / overhaul).

### 5. Exploration + online-research mandate

Authorize depth explicitly:

> Explore the repository as deeply as needed beyond the files listed above. Research online
> as deeply as needed — similar implementations, research papers, prior art — wherever it
> sharpens the deliverable. Cite sources for any external claim that shapes a decision.

**Calibrate the depth to the target.** The stock block above suits targets (new game gates,
novel mechanics) where external prior art genuinely shapes the design. For a behavior-free
infra / scaffolding / plumbing target governed by already-accepted ADRs, append a calibration
sentence telling Session 2 that external prior art is a *sharpening aid, not the crux* — name
the optional angles (e.g. characterization-testing discipline, canonical/versioned
serialization, dev-only test scaffolding, information-flow/no-leak geometry) and instruct it not
to let online research expand the locked scope — so research effort matches the target rather
than over-authorizing a literal open-ended sweep.

### 6. Doctrine & constraints

Pointers Session 2 must honor — trim to the constraints the target actually engages:

- `docs/FOUNDATIONS.md` is the constitution — every product-behavior decision must satisfy
  its §11 universal invariants and clear its §12 stop conditions; a genuine divergence
  requires an accepted ADR superseding the affected principle first (FOUNDATIONS: "supersede
  only by accepted ADR"), never designing against it silently.
- Authority order: foundation docs govern area docs govern specs govern tickets; if execution
  conflicts with architecture or foundation, execution is wrong.
- `engine-core` stays generic and **noun-free** — no `board`, `card`, `deck`, `grid`, `hand`,
  etc.; typed mechanic nouns belong in `games/*` first, shared helpers in `game-stdlib` only
  via the mechanic atlas.
- **TypeScript never decides legality.** Legal actions, validation, effects, views, and bot
  decisions all come from Rust/WASM.
- **No YAML and no DSL without an accepted ADR.** Static data is typed content / parameters /
  metadata only — never selectors, conditions, or triggers.
- **Determinism**: replay, hashes, RNG, serialization order, and traces stay deterministic
  (or are explicitly migrated).
- **No hidden-information leaks** into payloads, DOM, storage, logs, effect logs, bot
  explanations, or replay exports.
- **No MCTS / ISMCTS / Monte Carlo / ML / RL bots** in public v1/v2.
- **Never delete or weaken tests to get green** — follow the failing-test protocol
  (AGENT-DISCIPLINE §4).

### 7. Deliverable specification

Exactly what Session 2 outputs — leave no ambiguity:

- each **downloadable markdown document**, by filename and which of three shapes it is:
  it **replaces** an existing file, it is a **new** file landing at a fixed repo path, or
  it is an **intermediate artifact** — a design / work-indication document that a downstream
  skill revalidates and decomposes before it becomes final. For the `new-spec` pipeline the
  deliverable **is** the spec: the user saves it to `specs/<gate>-<slug>.md`, then
  `/reassess-spec` reassesses it **in place** against the codebase, then `/spec-to-tickets`
  decomposes it into `tickets/`. (`/reassess-spec` takes a `spec_path` under `specs/` — it
  does **not** convert a `reports/` draft, so name `specs/<gate>-<slug>.md` as the save target,
  not `reports/`.) For an intermediate artifact, name the eventual target path but state plainly
  that the deliverable is **not yet final** — so Session 2 doesn't present a ready-to-decompose
  file and skip the reassess step. Name the artifact's **own** emitted filename with a kebab-case
  stem that mirrors the brief's `<topic>-research-brief.md` slug — e.g. `<topic>-change-plan.md`
  for an advisory change-plan — so the brief and the deliverable it commissions share a stem and
  don't drift into ad-hoc or all-caps names;
- for replacements, name the file being replaced and what must be preserved vs. changed;
- the **locked / no-questions** instruction, verbatim intent:

> Produce the deliverables directly as downloadable markdown documents. Do not interview,
> do not ask clarifying questions — the requirements above are final. If a genuine
> contradiction makes a requirement impossible, state it in the deliverable and proceed
> with the most faithful interpretation.

### 8. Self-check

A short acceptance checklist Session 2 runs against its own output before returning —
e.g. every replacement preserves the load-bearing content of the original; no new doctrine
weakens an upstream foundation doc or silently amends an accepted ADR; every external claim
is cited; the deliverable set matches §7 exactly; the §1 fetch-baseline commit contains every
file named in the §2 read-in-full list.

### Optional target-specific sections

The eight sections above are the backbone, not a ceiling. Add sections when the target
warrants:

- **Inputs preamble** — when Session 2 receives out-of-band uploads beyond the manifest
  (PDFs, datasets), open with a short block enumerating *exactly* its inputs (this prompt;
  the named manifest; each uploaded document by name and role) and the fetch-baseline
  commit, so the locked Session 2 knows its full input set rather than assuming the default
  brief-plus-manifest pair.
- **IP discipline** — when the subject is private licensed IP, add a section instructing
  Session 2 not to reproduce licensed prose, art, or component text into the deliverable, to
  keep the subject material out of the repo and every public surface, and noting that the
  manifest deliberately excludes it. Cite `docs/IP-POLICY.md` and FOUNDATIONS §10.

---

## B. Target-type → load-bearing reads

A starting map for §2; always refine against Step 2 exploration. `docs/FOUNDATIONS.md`
and `docs/README.md` are load-bearing for every type.

| Target type | Load-bearing docs / files (beyond the two universal) |
|---|---|
| **new-spec** | `docs/ARCHITECTURE.md` and `docs/ENGINE-GAME-DATA-BOUNDARY.md` for the touched subsystem; the relevant `docs/ROADMAP.md` gate; `specs/README.md` and sibling `specs/*.md`; `tickets/README.md` and `tickets/_TEMPLATE.md` if decomposition follows; the relevant `games/*` module if game-specific. **For a mechanic-ladder game gate** (the dominant new-spec type this epoch — e.g. Gates 16–23): also `docs/MECHANIC-ATLAS.md` (§4 first/second/third-use rules, §9A armed interlocks, §10 per-shape rows, §10A debt register) — the primitive-pressure reuse / second-use / first-use deltas the brief's §3 must lock and which decide whether a third-use hard gate fires; `docs/OFFICIAL-GAME-CONTRACT.md` and `templates/**`; and, for any N-seat / team / partnership game, `docs/MULTI-SEAT-AND-SURFACE-CONTRACT.md` (§3 roles/teams/partnerships, §11 per-team outcomes) as the partnership/seat law. For a **perfect-information** game gate (no hidden hands — e.g. a board/topology race), also `docs/AI-BOTS.md`, and flag the information-model inversion in the brief's §3: the MULTI-SEAT §6 / TESTING §8.1 no-leak obligation becomes an explicit all-surfaces-public audit and the hidden-info replay-export ADR (0004) is `not applicable`. At a **mechanic-family pivot** (sibling N and N+1 differ in family — e.g. trick-taking → rummy), include the nearest *earlier* same-family game's `games/*` module and spec as the mechanic delta baseline, not only the immediate predecessor (which serves as the section-anatomy / governance template). **For a non-game infra/scaffolding/governance unit** (a recurring class here — e.g. 8M, 8C, 8C-R*, Infra A–D): also `docs/MECHANIC-ATLAS.md` (§10/§10A debt), `docs/MECHANICAL-SCAFFOLDING-REGISTER.md`, and the governing ADRs (e.g. 0008/0009/0004); the `games/*` module is then optional. |
| **thorny-fix** | the area doc(s) for the affected subsystem (`ARCHITECTURE.md`, `ENGINE-GAME-DATA-BOUNDARY.md`, `TESTING-REPLAY-BENCHMARKING.md`, etc.); the relevant code seams in `crates/`, `games/`, `tools/`, or `apps/web/`; any `reports/**` or `archive/**` audit touching the defect; any accepted `docs/adr/**` the fix must still satisfy. |
| **hardening / boundary-enforcement / anti-leak** | `docs/ENGINE-GAME-DATA-BOUNDARY.md` (engine-core / game-stdlib / static-data boundary) and `docs/AI-BOTS.md` (hidden-information safety); `docs/TESTING-REPLAY-BENCHMARKING.md` (no-leak tests, determinism, replay/hash); `docs/IP-POLICY.md` (public/private leak boundary); `docs/AGENT-DISCIPLINE.md`; prior hardening specs in `archive/specs/**` and any audit `reports/**`. |
| **foundational / doc-overhaul** | the full doc being overhauled plus every document above it in the `docs/README.md` order (authority flows downward); `docs/README.md` for the authority table and layering rule; any staleness/downstream `reports/**`; cross-references in lower docs/specs that the overhaul will invalidate; relevant `docs/adr/**` that pin the affected doctrine. |
| **other** (incl. audit / review / presentation-overhaul) | derive from exploration; default to the universal two plus whatever the target names. For an audit / review / UX-overhaul target: the area doc(s) for the touched subsystem; the game's or subsystem's own `docs/` and its `tests/` + golden traces as the correctness oracle; `docs/IP-POLICY.md` and `docs/UI-INTERACTION.md` when presentation is in scope; and — for presentation/behavior-facing targets — exercise the running artifact, not source alone. |

# Private Lane P1 — Milestone 1 correctness & coverage audit research brief

You are an external deep researcher (ChatGPT-Pro). Produce the deliverable in
§7 **directly**. The requirements below are final — **do not interview, do not
ask clarifying questions**. The interview already happened; this brief is the
result.

---

## 0. Your inputs (read this first)

Your full input set is exactly:

1. **This prompt.**
2. **The public repository.** The uploaded manifest
   `joeloverbeck-rulepath_manifest_2026-06-29_a0117ec.txt` is the path inventory
   of the public `joeloverbeck/rulepath` repository at one exact commit. Read its
   files at that commit (see §1 for the access pattern).
3. **The private repository.** The uploaded manifest
   `joeloverbeck-rulepath-private_manifest_2026-06-29_e7a22e7.txt` is the path
   inventory of the **private** `joeloverbeck/rulepath-private` repository at one
   exact commit. This is the sanctioned private game lane (ADR-0010/0011/0012). It
   contains the **completed Milestone-1 implementation** that this audit examines.
   You can reach it through the GitHub connector by its commit SHA exactly as you
   reach a public repo (the user has confirmed this access path); the manifest is
   your inventory/checklist of every file to read.
4. **Two source PDFs**, provided by the user out of band: a **rules /
   living-rules rulebook PDF** and a **playbook PDF** for *the first private
   licensed game* (a GMT COIN-series title). These are **private licensed IP**
   and your authoritative rules source for the game's components, setup,
   operations, special activities, sequence of play, full event deck, propaganda
   sequence, victory, and standard scenario. They are gitignored in the private
   repo (only their checksums are committed as source receipts), so they are
   delivered to you separately — treat them as in-scope source material.

Both manifests **deliberately exclude** the two PDFs and any licensed source
expression. That is **intentional, not a gap**: the PDFs are private licensed IP
recorded only as checksummed receipts. The public manifest also excludes the
private game entirely — the private game lives only in the private repository, by
design.

This brief itself is a **public-repository artifact** (it lives in the tracked
public `reports/` tree), so it stays **opaque**: it never names the licensed
title and never names the per-card source filenames. Your **deliverable**, by
contrast, is destined for the **private repository**, so it MAY name the title
and describe the game's mechanics — subject to the IP discipline in §8.

Fetch every **public** repository file from this exact commit:

```
https://raw.githubusercontent.com/joeloverbeck/rulepath/a0117ec6097c1b980bbc0f0c3b6bcbc864deb4e1/<manifest path>
```

Read every **private** repository file at private commit
`e7a22e727f3da3d0fddfdbb1165de3e059e6eead` (short `e7a22e7`) — the private
manifest reflects that tree. If any report or doc you encounter cites a different
"commit of record," ignore that string and use the two commits above.

---

## 1. Context

The uploaded public manifest is the path inventory of the `joeloverbeck/rulepath`
repository — a **Rust-first, rule-enforcing, replayable, testable card/board-game
platform where Rust owns all behavior and TypeScript/React present only**. The
foundation docs are an ordered, layered authority indexed by `docs/README.md`:
`FOUNDATIONS.md` (the constitution) → `ARCHITECTURE.md` →
`ENGINE-GAME-DATA-BOUNDARY.md` → the area docs → `ROADMAP.md`; earlier documents
govern later ones, and accepted ADRs supersede them only by explicitly naming the
affected sections. Fetch every public file from commit `a0117ec`.

The private `joeloverbeck/rulepath-private` repository is the sanctioned private
game lane authorized by ADRs **0010** (parallel private-game lane), **0011**
(constrained typed Rust event-card mechanism), and **0012** (private
repository / CI federation / catalog overlay). It pins the public repo at commit
`a0117ec` and depends on the public crates as git dependencies. It contains the
**completed Milestone-1 implementation** of the first private licensed game: the
implementation spec and its decomposed tickets are **done and archived** (under
`archive/specs/` and `archive/tickets/`), the game crate is fully built (rules,
operations, special activities, the full event deck, propaganda, scoring,
visibility, replay), there is a test suite and benchmark set, and per-game docs
including a rule-coverage matrix and an event-coverage matrix are in place.

This brief commissions a **correctness-and-coverage audit** of that completed
implementation. The author wants near-total confidence that the Rust
implementation of every rule — operations, special activities, the sequence /
flow of play, all 48 events, propaganda rounds, victory/terminal detection — is
faithful to the source rulebook *and* thoroughly proven by tests, **before**
future sessions implement designed faction AI. The implementation and its spec
are **done**; do **not** re-propose them, re-author the spec, or re-implement.
Audit them: confirm what is correct, and find what is wrong, thin, or missing.

---

## 2. Read in full (authority order)

### Public repository (fetch at `a0117ec`)

Read these in full, in this order. Each is load-bearing for *this* audit.

**Constitution & decisions**

- `docs/README.md` — authority order and layering rule; the ADR status index (0010/0011/0012 are `Accepted`).
- `docs/FOUNDATIONS.md` — the constitution: priority order, **§1.1 sanctioned private-game lane**, §10 IP conservatism, §11 universal invariants, §12 stop conditions (incl. private-lane stops), §13 ADR triggers. Every correctness judgment must be framed against these.
- `docs/adr/0010-sanctioned-parallel-private-game-lane.md` — the non-contamination rule the implementation runs under.
- `docs/adr/0011-constrained-typed-rust-event-card-mechanism.md` — the **only** sanctioned event-card shape: typed inert content + Rust behavior; **no** YAML/DSL/untyped effect rows. The event deck must conform; flag any drift.
- `docs/adr/0012-private-repository-ci-catalog-overlay.md` — the private-repo/CI architecture the milestone lives in.

**Boundary, IP, architecture**

- `docs/ENGINE-GAME-DATA-BOUNDARY.md` — **§10A typed Rust card-effect registries**: the inert-content-vs-Rust-behavior line; `engine-core` stays noun-free. Audit that no game noun leaked into `engine-core` and no behavior leaked into data.
- `docs/IP-POLICY.md` — **§9/§9A/§9B**: the public no-leak checklist, opaque-placeholder rule, and "if it ships to an unauthorized browser, it has shipped." Governs the audit's own opacity and the public back-leak check.
- `docs/ARCHITECTURE.md` — action/view/effect/replay/determinism model and **§11A the sanctioned private overlay lane** + large-action-tree guidance.

**Contract, atlas, scaffolding, multi-seat, bots**

- `docs/OFFICIAL-GAME-CONTRACT.md` — **§1A completion profiles** (esp. `private-milestone-1-rule-complete`), the P1-M1 capability/non-goals note, and the required private-spec field set the implementation should carry — the audit checks the implementation actually meets the `private-milestone-1-rule-complete` bar.
- `docs/MECHANIC-ATLAS.md` — **§2 private-stress categories** (card-driven initiative/eligibility, asymmetric faction menus, operation/special-activity coupling, propaganda upkeep, conditional event branches, persistent/temporary effects, faction victory tracks) and **§10A promotion-debt register**. The stress categories are a checklist of mechanic areas to audit.
- `docs/MECHANICAL-SCAFFOLDING-REGISTER.md` — what behavior-free scaffolding the implementation was meant to reuse vs. re-implement; relevant only where reuse correctness bears on game correctness.
- `docs/MULTI-SEAT-AND-SURFACE-CONTRACT.md` — **§6A asymmetric-faction + 5-viewer no-leak floor** (public observer + 4 faction viewers; pairwise redaction across every surface). The no-leak correctness bar.
- `docs/AI-BOTS.md` — **§4B private asymmetric sourcing + no-flowchart rule** and Level-0 deferral; confirms the bot posture (Level-0 random-legal driver only; no designed AI; no publisher flowchart).

**Testing, evidence, trace, sources, discipline**

- `docs/TESTING-REPLAY-BENCHMARKING.md` — **§8 private large-game coverage**: the proof plan (seat counts, largest fixtures, 5-viewer matrix, event-deck/large-action-tree/upkeep/terminal traces, native replay/hash + browser smoke, benchmark targets). The yardstick for the test-coverage adequacy judgment.
- `docs/EVIDENCE-FIXTURE-CONTRACT.md` — `visibility_class = private-source` evidence profiles and receipt fields; the evidence the implementation owes.
- `docs/TRACE-SCHEMA-v1.md` — the trace/replay-fixture schema law; **no migration is authorized** — determinism/replay must live within it.
- `docs/SOURCES.md` — recorded external prior art (Rally the Troops/GMT, VASSAL, boardgame.io, OpenSpiel, etc.) with each Rulepath lesson and non-adoption; the starting point for §5 online research.
- `docs/AGENT-DISCIPLINE.md` — the bounded-task / failing-test law your remediation items must be decomposable under.

**Specs & templates**

- `specs/README.md` — the **Private lane tracker** and the canonical **Spec format**; context for where this unit sits.
- `templates/GAME-RULE-COVERAGE.md`, `templates/GAME-EVENT-COVERAGE.md`, `templates/GAME-RULES.md`, `templates/GAME-MECHANICS.md`, `templates/GAME-EVIDENCE.md`, `templates/GAME-IMPLEMENTATION-ADMISSION.md`, `templates/PRIVATE-RELEASE-CHECKLIST.md`, `templates/GAME-SOURCES.md`, `templates/AGENT-TASK.md` — the contracts the private docs and the audit's findings format are built against. (`templates/README.md` for the set's overview.)

### Private repository (read at `e7a22e7`; exact paths in the private manifest)

Read the private implementation in full — the manifest is your enumerated
checklist. Described by role (the manifest carries the exact paths):

- **The archived implementation spec** (`archive/specs/`) and the **archived
  tickets** (`archive/tickets/`) — the as-built record of what M1 set out to
  deliver and how it was decomposed. Read these first to learn the intended
  behavior contract; the audit measures the code and tests against them *and*
  against the PDFs.
- **The private game crate's core source modules** — setup, state, ids, actions,
  operations, special activities, the events dispatcher, propaganda, scoring,
  effects, visibility, replay support, ui, bots, and the crate root. Read every
  one; these own all behavior.
- **The 48 per-event-card source modules** (one Rust module per card) — the full
  event deck. Every card's selectors, target constraints, legality, branches,
  effects, persistence, and visibility live here. This is the single largest
  correctness surface; audit all 48.
- **The crate's test suite** (~16 test files covering setup, sequence of play,
  the action tree, the three faction operation families, propaganda, the event
  deck split across four ranges, replay, serialization, visibility, and the
  Level-0 bot driver) — the existing proof. Judge depth, not just presence.
- **The crate's benchmarks and data** (the benchmark suite + thresholds, the
  data manifest, and the source-receipt file) — the performance proof and the
  IP-receipt record.
- **The crate's game docs** — the rule-coverage matrix (`RULE-*` IDs → Rust owner
  → evidence), the event-coverage matrix (48 cards → branch kind → Rust owner →
  receipt → required-proof id), the rules summary, mechanics, evidence, admission,
  private-release checklist, sources, AI/competent-player, UI, how-to-play,
  benchmarks, and primitive-pressure ledger. These are the documentation oracles
  the audit cross-checks against both the code and the PDFs.

### Out-of-band source (provided by the user)

- **The rules/living-rules rulebook PDF** and **the playbook PDF** — the
  authoritative rules. Read thoroughly for components, setup, the operations and
  special-activities matrix per faction, the full sequence of play / eligibility
  model, every event card and its branches, the propaganda round sequence,
  victory/terminal conditions, and the standard full-length scenario. Cite them
  by **private-source receipt id** (recorded in the crate's source-receipt file),
  never by quoting them.

---

## 3. Settled intentions (these are decided — do not reopen)

1. **The target is a correctness-and-coverage audit of a *completed* M1
   implementation.** The implementation, its spec, and its tickets are **done and
   archived**. Do **not** re-author the spec, re-decompose it, or re-implement.
   Your job is to *verify* and *recommend changes* — confirm what is correct,
   surface what is wrong/thin/missing, and prescribe the intended changes.
2. **The game** is *the first private licensed game* — the GMT COIN-series title
   identified by the two uploaded PDFs — handled as private licensed IP.
3. **Full correctness envelope is in scope.** Audit, against the PDFs and the
   foundation/area docs: setup; the full sequence of play / card-driven
   eligibility and initiative; all operations; all special activities; **every one
   of the 48 event cards** and all their branches/persistence; propaganda rounds
   and their upkeep/scoring pipeline; victory/terminal detection; **and** the
   cross-cutting behavioral invariants — determinism (setup, legal-action
   generation, transitions, view projection, serialization order), replay and
   hash stability within `TRACE-SCHEMA-v1`, and the **5-viewer no-leak /
   visibility** correctness (public observer + 4 faction viewers, pairwise
   redaction across every surface).
4. **Test-coverage bar: per-rule + per-card traceability.** For every `RULE-*` ID
   in the rule-coverage matrix and for each of the 48 cards, judge whether the
   cited tests exercise the actual behavior **meaningfully** (real assertions on
   state/effects/branches, edge cases, failure paths) versus smoke-only or
   presence-only coverage. Flag every thin or missing case and recommend the
   specific tests to add (unit / rule / golden-trace / property / sim / replay /
   serialization / visibility, as appropriate).
5. **Completeness audit against the PDFs is in scope.** Do not trust the
   documented `RULE-*` / event enumeration as complete. Cross-check the rulebook
   and playbook against the `RULE-*` set, the coverage matrices, and the code, and
   flag any rule, sub-rule, exception, timing nuance, or event branch that is
   present in the source but **absent** from the documented rule set, the
   implementation, or the tests. Missing rules are findings, not gaps in this
   brief.
6. **The correctness oracles, in priority order:** (a) the two source PDFs (the
   authoritative rules); (b) the crate's `GAME-RULES`/rule-coverage and
   event-coverage matrices (the implementation's own claims, themselves audited
   for fidelity to the PDFs); (c) the test suite and golden traces (the existing
   proof, audited for depth); (d) the archived spec/tickets (intended contract).
   Where (b)/(c)/(d) diverge from (a), the PDFs win and the divergence is a
   finding. Where the PDFs are ambiguous, consult published errata/FAQ/living-rules
   and record the interpretation taken.
7. **Out of scope.** (a) **Web/UI** — there is no web/WASM interface yet; do not
   audit or design one (note the absence only where it bears on a deferred
   evidence obligation). (b) **Designed faction AI** — a later milestone; the
   existing **Level-0 random-legal move driver** is in scope only as an *evidence
   harness* (does it faithfully sample the legal-action API uniformly and feed
   sim/replay/property evidence?), never as strategy to critique. Do not design
   bots, and flag any publisher flowchart/priority-chart leakage as a violation.
8. **Boundary, determinism, and IP criteria hold as audit axes.** ADR-0011
   typed-Rust event boundary (no YAML/DSL/untyped effect rows; identity/deck
   order/inert display metadata MAY be typed static content, but every condition,
   selector, trigger, override, legality hook, transition, and visibility decision
   is Rust); `engine-core` noun-freedom; determinism within `TRACE-SCHEMA-v1` with
   no public trace/hash migration; and the public back-leak boundary (no licensed
   title/id/fixture name in any public surface) are all things the audit checks
   the implementation against.

`assumption:` the deliverable is **one** advisory change-plan document (not a
formal spec and not also tickets). The author will later decompose its
remediation items inside the private repository using the project's private
analog of the `/reassess-spec` → `/spec-to-tickets` flow, so the change-plan's
items must be **bounded, prioritized, and reassessable/decomposable** (each a
discrete diff with clear exit criteria).

`assumption:` "near-total / 100% confidence" is the goal, but the deliverable is
an *audit + intended-changes plan*, not a proof of total correctness. Where the
audit cannot reach certainty (e.g. a PDF ambiguity, a behavior only a long
property/sim run could falsify), say so explicitly and prescribe the evidence
that would close it, rather than asserting correctness.

---

## 4. The task

Produce **one advisory change-plan** that audits the completed Milestone-1 Rust
implementation of the first private licensed game for **rule correctness** and
**test-coverage adequacy**, and prescribes the intended changes. Working from the
source PDFs as the authoritative rules and from the foundation/area docs for every
boundary, determinism, no-leak, IP, and evidence obligation, verify: setup; the
full card-driven sequence of play / eligibility; all operations; all special
activities; every one of the 48 event cards and their branches; propaganda rounds
and scoring; victory/terminal detection; and the cross-cutting determinism,
replay/hash, and 5-viewer no-leak invariants. For coverage, judge per-rule and
per-card whether the existing tests prove the behavior meaningfully, and identify
both rules present-but-undertested and rules present-in-the-PDFs-but-absent from
the documented set/implementation. The deliverable catalogs findings by severity,
maps each to a concrete intended change (code fix and/or specific tests to add),
and sequences the remediation into bounded, decomposable items. This is an
**audit / change-plan**, not a new spec and not an implementation.

---

## 5. Exploration + online-research mandate

Explore both repositories as deeply as needed beyond the files listed above, and
read the two uploaded PDFs thoroughly — the full rules, the complete event deck,
the per-faction operation/special-activity matrix, the eligibility/sequence-of-play
model, the propaganda sequence, victory conditions, and the standard scenario
setup. Research online as deeply as needed and cite sources for any external claim
that shapes a finding.

Genuinely-relevant external angles (start from `docs/SOURCES.md`): the specific
GMT COIN-series title identified by the PDFs and its **published errata, FAQ, and
living-rules revisions** (rule corrections are prime sources of subtle
implementation bugs — diff the living rules against the playbook's original
printing); the COIN-series **sequence-of-play / eligibility (1st-/2nd-Eligible,
pass, limited-command) model** and common implementation pitfalls; open
implementations of card-driven counterinsurgency / area-control wargames (e.g.
Rally the Troops, VASSAL modules) and where their rule-edge handling differs;
boardgame.io / OpenSpiel **public/private player-view and information-state
separation** as a no-leak reference (**not** their RL/search approach — public
v1/v2 bots exclude MCTS/ISMCTS/Monte Carlo/ML/RL and M1 has no designed AI);
deterministic event-resolution and large-action-tree correctness patterns.

Let research **sharpen** the audit; it must not **expand** scope into bot design,
web/UI, or anything beyond verifying the M1 implementation. Do not recommend any
change that would put rules in TypeScript, encode behavior in data/YAML/DSL, add
game nouns to `engine-core`, weaken a test to get green, or copy a publisher
flowchart.

---

## 6. Doctrine & constraints (honor all)

- `docs/FOUNDATIONS.md` is the constitution — every correctness judgment and every
  prescribed change must satisfy its §11 universal invariants and clear its §12
  stop conditions (including the private-lane stops: no private content in public
  surfaces; no private game shaping public architecture). A genuine divergence
  would require a *new* accepted ADR first, never designing against the
  constitution silently.
- **Authority order:** foundation docs govern area docs govern specs govern
  tickets. If the implementation conflicts with architecture or the constitution,
  the implementation is wrong and the change-plan says so.
- `engine-core` stays generic and **noun-free** — no `faction`, `card`, `deck`,
  `operation`, `eligibility`, `board`, `grid`, `hand`, etc. COIN nouns live in the
  private game crate; any noun that leaked into `engine-core` is a finding.
- **TypeScript never decides legality.** Legal actions, validation, effects,
  views, visibility, victory, and any bot decisions all come from Rust.
- **No YAML and no DSL.** Static data is typed content/parameters/metadata only —
  never selectors, conditions, triggers, overrides, or effect formulas (ADR-0011
  is the exact boundary). Any untyped effect row is a finding.
- **Determinism:** replay, hashes, RNG, serialization order, and traces stay
  deterministic within `TRACE-SCHEMA-v1`; no public trace/hash migration. A
  non-deterministic path or an unstable hash is a finding.
- **No hidden-information leaks** into payloads, DOM, storage, logs, effect logs,
  bot explanations, candidate rankings, or replay exports — proven against the
  5-viewer matrix. Any cross-faction or public-observer leak is a high-severity
  finding.
- **No MCTS/ISMCTS/Monte Carlo/ML/RL bots**, and **no publisher
  flowchart/priority chart** in bot policy/tests/strategy docs. M1 has only the
  strategy-free Level-0 random-legal driver.
- **Never weaken tests to get green** (AGENT-DISCIPLINE §4); fix the code, not the
  test. The change-plan's coverage recommendations strengthen proof, never relax
  it.

---

## 7. Deliverable specification

Produce **one downloadable markdown document**:

- **Filename:** `private-lane-p1-milestone-1-correctness-audit-change-plan.md`
  (opaque stem, mirroring this brief's slug).
- **Shape:** an **advisory change-plan / audit report** that lands in the
  **private repository** (its `reports/` tree), **not** the public repo. It is
  **not** a formal spec and **not** tickets — it is the intended-changes document
  the author will later decompose with the private analog of
  `/reassess-spec` → `/spec-to-tickets`, so its remediation items must be bounded,
  prioritized, and decomposable.
- **Structure (use this backbone):**
  1. **Executive summary** — overall confidence verdict and the count of findings
     by severity.
  2. **Method & oracles** — exactly how each area was verified, the oracle
     priority order (§3.6), the commits/PDFs/receipt ids used, and the limits of
     what the audit could and could not establish.
  3. **Findings by severity** (P0 correctness-breaking → P3 minor/cosmetic). For
     each finding: a stable id; the area (setup / sequence / operations / special
     activities / a named event card / propaganda / victory / determinism / replay
     / visibility-no-leak / boundary / IP); the rulebook basis (paraphrased + a
     receipt-id citation, never quoted); the observed implementation behavior with
     the Rust owner (module/function); why it diverges; and the **intended
     change** (the specific code fix and/or the specific test(s) to add).
  4. **Per-rule traceability table** — every `RULE-*` ID: implemented? tested
     meaningfully? finding id(s) if not.
  5. **Per-card traceability table** — all 48 cards: every branch implemented?
     every branch tested? persistence/visibility correct? finding id(s) if not.
  6. **Completeness gaps** — rules/sub-rules/exceptions/event branches present in
     the PDFs (or in errata/living-rules) but **absent** from the documented set,
     the implementation, or the tests.
  7. **Cross-cutting invariants** — determinism, replay/hash within
     `TRACE-SCHEMA-v1`, serialization order, and the 5-viewer no-leak matrix:
     verdict + findings.
  8. **Test-coverage plan** — the prioritized list of tests to add/strengthen,
     grouped by kind, each tied to the finding(s) it closes.
  9. **Sequenced remediation** — the bounded, dependency-ordered change items
     (each a discrete reviewable diff with exit criteria) the author will
     decompose.
  10. **Confidence assessment & residual risk** — what reaches near-total
      confidence, what does not, and the evidence (e.g. a longer property/sim run,
      a specific golden trace) that would close each residual.
- **If the audit finds no change is warranted in an area, say so explicitly** —
  "confirmed correct, evidence: …" is a valid and valuable finding.

Locked / no-questions instruction:

> Produce the deliverable directly as a downloadable markdown document. Do not
> interview, do not ask clarifying questions — the requirements above are final.
> If a genuine contradiction makes a requirement impossible, state it in the
> deliverable and proceed with the most faithful interpretation.

---

## 8. IP discipline (private licensed subject)

- The **deliverable** (private repo) MAY name the title and describe mechanics in
  **original Rulepath prose**. It MUST **not** reproduce licensed expression: no
  copied rulebook prose, card text, examples, diagrams, tables, charts, art,
  icons, trade dress, or — critically — **publisher flowchart / non-player /
  priority-chart text**. Summarize every rule and event in your own words;
  reference the PDFs by **private-source receipt id** (recorded in the crate's
  source-receipt file), not by quoting them.
- Keep the subject material **out of the public repository and every public
  surface**. Nothing in this brief, either manifest, public source, public docs,
  public CI, public traces, public bundles, or public WASM/JS may name the title,
  a card/event/faction/scenario id, a private fixture/e2e filename, or a catalog
  string. The private game lives only in the private repository.
- Both manifests **deliberately exclude** the source PDFs (recorded only as
  checksummed receipts); the public manifest excludes the private game entirely.
  These absences are by design, not gaps.
- Governing law: `docs/IP-POLICY.md` (esp. §9/§9A/§9B and the public no-leak
  checklist) and `docs/FOUNDATIONS.md` §10.

---

## 9. Self-check (run before returning)

- The deliverable is exactly the **one advisory change-plan** named in §7, landing
  in the private repo, structured per the §7 backbone — not a spec, not tickets,
  not a re-implementation.
- The completed M1 implementation, its archived spec, and its archived tickets are
  treated as the **done baseline being audited**, never re-proposed or re-authored.
- Every area in §3.3 is audited against the PDFs as the authoritative oracle:
  setup, full sequence/eligibility, all operations, all special activities, **all
  48 event cards and branches**, propaganda, victory/terminal, determinism,
  replay/hash, and the 5-viewer no-leak matrix.
- Per-rule and per-card traceability tables are present; thin/smoke-only coverage
  is flagged; specific tests-to-add are prescribed.
- A **completeness pass vs the PDFs** (incl. errata/living-rules) is included —
  anything in the source but missing from the documented set/implementation/tests
  is flagged.
- The event deck is verified against the **ADR-0011 typed-Rust** boundary
  (no YAML/DSL/untyped effect rows); `engine-core` carries no COIN noun.
- Bots: the Level-0 random-legal driver is audited only as an evidence harness;
  no designed AI is critiqued or proposed; **no publisher flowchart** is reproduced
  anywhere.
- **No licensed expression** is reproduced; the title and all private ids stay out
  of every public surface; the PDFs are cited by receipt id.
- Every external claim that shaped a finding (errata, FAQ, prior-art comparison) is
  **cited**.
- The §1 fetch baselines are correct: every public file named in §2 exists at
  `a0117ec`, and every private file is read at `e7a22e7`.

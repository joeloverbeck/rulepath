---
name: brainstorm
description: "Use when starting a new feature, design, or architectural decision that needs requirements discovery before implementation. Triggers: vague requests, exploration keywords, uncertainty about what to build, need for external research before designing."
user-invocable: true
arguments:
  - name: request
    description: "The brainstorming topic or question (string). Can be a simple sentence or a detailed description."
    required: true
  - name: reference_path
    description: "Optional path to a reference file (report, brainstorming doc, analysis) to read as context before starting the interview."
    required: false
---

# Brainstorm

Confidence-driven collaborative brainstorming. Interviews you until it understands what you **actually want** — not what you think you should want — then proposes approaches, builds a design, and lets you choose what happens next.

<HARD-GATE>
Do NOT write any code, scaffold any crate, invoke any implementation skill, or take any implementation action until you have presented a design (or a triage recommendation) and the user has explicitly approved it. This applies to EVERY topic regardless of perceived simplicity.

Carve-out: when the user's initial request pre-authorizes a deliverable contingent on the brainstorm's verdict (e.g. "write an ADR if changes are warranted") — or issues an unconditional build/port/"adapt" directive for a specific deliverable (e.g. "adapt this skill into a repo-local one") — presenting the design or triage recommendation (for a Port, the transformations table) IS the approval that satisfies this gate — see Guardrails §User pre-authorization. The Step 6 menu still fires afterward.
</HARD-GATE>

## Process flow

```
Read context (reference file + classify topic)
        |
        v
Confidence-driven interview loop (target: 95%)
        |
        v
Propose 2-3 approaches with tradeoffs
        |
        v
Present design section by section, get approval per section
        |
        v
[If product-behavior] Validate against docs/FOUNDATIONS.md
        |
        v
Persist the deliverable (design doc / ADR / agent task / ...)
        |
        v
Next-steps menu (user chooses)
```

**In plan mode**: the design writes to the plan file instead of `docs/plans/`, and `ExitPlanMode` replaces the next-steps menu.

**Triage/analysis brainstorms** (evaluate a report or finding-set, or answer a diagnostic question, and produce work items *or a single recommended fix*): the interview loop and the propose-approaches step are replaced by a single triage recommendation — see Step 3 §Triage brainstorms and `references/triage-and-deliverables.md`.

## Step 1: Read context

1. **Reference material.**
   - **`reference_path` argument**: read the entire file. Extract its key claims, proposals, and open questions.
   - **Inline file references**: if the user names files inline in the request, read them with the same treatment. Read all of them.
   - **Inline-pasted prose**: if the user pastes reference content (external research output, a report excerpt, a finding-set) directly into the request body, that prose IS the reference — there's no file to read.
   - **Not reference material**: the user's own first-hand problem description (bug report, symptoms, repro steps, pasted error/program output as evidence of the defect) is the request itself, not externally-authored reference content. A file named only as the alignment authority (`docs/FOUNDATIONS.md`) is also not reference material — it's governed by sub-step 3. Inline-named **internal codebase files read for orientation** (`README.md`, CI/config, source files the user points at to ground the design) are likewise not reference material — they're covered by exploration (sub-steps 6–7) and get no reference summary; reserve the 2-3 sentence summary for inline-named **authored artifacts** (a report, research doc, external spec). The discriminator is authored-reference-content vs. repo-state-for-context, not merely whether a filename was mentioned.
   - **Oversized files**: if a reference exceeds the Read token limit, read in sequential offset/limit chunks until fully covered, and say so in the summary. Run a quick `wc -l <path>` first when you suspect a file is large.
   - **Path resolution**: if an inline filename doesn't resolve, glob the parent directory for the closest match (pluralization, hyphen drift, capitalization) before asking the user to re-specify; announce any correction inline.

   **Emit a 2-3 sentence summary** of what the reference material contains as user-facing text before any further exploration — this is the user's audit-trail anchor for verifying you understood the reference. For a multi-item report, summarize its *shape* (item count, structure, headline position) rather than enumerating each item; per-item detail belongs in the Step 3 triage. When the reference makes claims about the codebase or product state, treat them as hypotheses to verify during exploration (sub-step 6), and flag prominently any that exploration contradicts. When there is no reference material at all (pure analysis from the request text), no summary fires — proceed to exploration; a single confirming line ("No reference material — the request itself is the spec; proceeding to exploration") is acceptable as the audit-trail anchor and is distinct from the multi-sentence reference summary.

2. **Topic classification.** Announce the classification on its own line as soon as it's determinable — often right after reading the primary reference's frontmatter/intro, and in all cases before the broader project-context exploration in sub-step 7. Don't let a large multi-file reference read push the announcement arbitrarily late; classify from the minimum evidence that settles the category, then keep reading. Pick the category the brainstorm falls into:
   - **product-behavior**: anything that changes how the app behaves at runtime or what it persists — rules, legal action generation, validation, state transitions, scoring, terminal detection, deterministic randomness, semantic effects, replay/checkpoint/hash behavior, public/private view projection, serialization contracts, bot choices, or effect-log-driven UI/renderer behavior, together with the `engine-core` kernel boundary, the earned-`game-stdlib` boundary, the static-data-is-not-behavior boundary, and the IP/public-private boundary that govern them. These are governed by `docs/FOUNDATIONS.md` and the architecture docs it anchors (`docs/ARCHITECTURE.md`, `docs/ENGINE-GAME-DATA-BOUNDARY.md`, `docs/OFFICIAL-GAME-CONTRACT.md`, `docs/MECHANIC-ATLAS.md`, `docs/AI-BOTS.md`, `docs/UI-INTERACTION.md`, `docs/TESTING-REPLAY-BENCHMARKING.md`, `docs/IP-POLICY.md`, `docs/AGENT-DISCIPLINE.md`).
   - **dev-process / docs / tooling**: work that does not change product behavior — build/test/benchmark scripts, dev ergonomics, repo tooling, skill design, or documentation/template edits that don't change product semantics. But a benchmark/CI/tooling change that moves gating *doctrine* fixed by an accepted ADR or the testing law (a threshold calibration, lane, or budget) is **product-behavior** — classify by the doctrine it moves, not the file it edits, and read FOUNDATIONS accordingly.

   When genuinely ambiguous, give a one-to-two-sentence justification. Example: `Classification: product-behavior (the design adds a validation rule to the legal-action generation path).`

   **Planning/spec artifacts.** A deliverable that is itself a *plan* (a spec, roadmap, design doc, or agent task) is classified by its own nature — it's a document, so it is **dev-process/docs** even when the work it plans is product-behavior. Still apply the Step 4 FOUNDATIONS alignment table to the *work the plan describes* when that work engages a principle or a §12 stop condition (e.g. a spec that establishes the `engine-core` boundary). Classify by the artifact you write, align by the behavior it governs. **When the deliverable artifact is undetermined at the sub-step 2 announcement** — common in triage/diagnostic brainstorms, where Step 3/5 decides only later whether the fix is tickets, an ADR, or a design doc — classify at announcement time by the *topic/behavior under analysis* (so a bug-behavior diagnosis announces `product-behavior`, which correctly pulls in the FOUNDATIONS read), then re-state the artifact-based classification if Step 5 selects a plan artifact. The Step 4 "Classification pivot check" is that re-statement hook; the two rules don't conflict once the announcement is understood as topic-keyed and the pivot as artifact-keyed. **When the symptom-surface and the root-cause-surface differ** — a CI/gate/build/test-failure diagnostic whose failing *job* is a tooling surface but whose root cause is not yet located ("why is Gate 0 failing") — don't announce `product-behavior` on the speculation that the fix *might* touch product code; default to `dev-process/tooling` (or defer the announcement) until exploration locates the failing code, then classify by that root-cause surface. The FOUNDATIONS-read decision hinges on where the cause lives, not on which job reported it — a lint or build-script fix stays dev-process even though the gate also fences product behavior. The same divergence arises for a **UI/display defect** whose cause could be either the presentation layer (TS/CSS — dev-process) or a behavior/view-projection surface (product-behavior): announce provisional `product-behavior` so the FOUNDATIONS read primes the boundary check, then pivot via the Step 4 pivot check once exploration locates the cause (a parser or CSS fix settles to dev-process; a view-projection or legality change stays product-behavior).

   **Governance-doc editorial gate.** When a dev-process/docs topic restructures `docs/FOUNDATIONS.md` or the law/architecture docs it anchors (the doc set listed under product-behavior above) — merging, splitting, de-duplicating, moving, or re-indexing them — keep the dev-process/docs classification but apply a semantic-preservation gate: meaning-preserving moves/merges/de-duplication are editorial and need no ADR, whereas any change to a *principle's meaning* requires an ADR superseding it first (FOUNDATIONS: "Supersede only by accepted ADR"). In this case read FOUNDATIONS even though the topic is dev-process (sub-step 3's optional-read carve-out does not apply), state the preservation guarantee in the design, and surface any merge that would force a genuine semantic reconciliation rather than silently resolve it.

3. **Read `docs/FOUNDATIONS.md` for product-behavior topics.** You'll need it in Steps 3 and 4 to validate approaches against the constitution and its §12 Stop conditions (and, for agent-task and review work, the AGENT-DISCIPLINE §13 review checklist and the ADR review checklist in `docs/adr/ADR-TEMPLATE.md`). For dev-process/tooling/docs topics the read is optional — read it only if the topic directly engages a FOUNDATIONS principle. The read is satisfied if FOUNDATIONS content is already in this session's context; re-read only after compaction or when you need a specific principle's wording. If a brainstorm reveals that the *right* design conflicts with FOUNDATIONS, the deliverable may need to be an **ADR that supersedes the relevant principle first** — FOUNDATIONS states "Supersede only by accepted ADR" — so flag this prominently and get explicit sign-off; never quietly design against the constitution. The same ADR-first trigger applies when the design **amends or supersedes the doctrine fixed by an existing accepted ADR** (recalibrating a value, threshold, boundary, or lane an ADR established) — even for a **dev-process/tooling topic** where the FOUNDATIONS read is otherwise optional, write the amending ADR before the implementing change; a plain ticket or config edit must not silently weaken accepted-ADR doctrine (this is a FOUNDATIONS §13 ADR trigger, not only a FOUNDATIONS-principle trigger).

4. **Calibrate initial confidence from the reference and request.** A thorough reference (rationale, decisions, structure) or a richly-specified request (problem analysis, evidence, root cause, code locations, a clear ask) can start confidence at 70-95%, reducing the interview to closing operational gaps. Don't ask motivational questions when the user has already demonstrated deep understanding.

5. **Interview-skip threshold.** If exploration and research bring confidence to 95%+ before the interview starts, skip Step 2 and go to Step 3; announce the confidence level. At 85-94% with a user-framed, fully-constrained directive (enumerated alternatives, or a clear "do X" with minimal open choices — triage brainstorms are the common case), also skip the discovery interview: go to Step 3, announce confidence, and carry remaining gaps as named assumptions for the user to correct during approach/triage selection. **When a remaining gap would *materially shape the deliverable*** (which approach, what gets stripped vs. kept, the deliverable's structure or class) rather than only a detail, prefer posing it via `AskUserQuestion` at approach-selection rather than carrying it as a silent assumption; reserve named-assumptions for detail-only gaps. (Mirrors §Guardrails §User pre-authorization, which applies the same shape-confirmation rule before writing.)

6. **External research.** If the topic needs domain knowledge beyond the codebase (algorithms, game-rule sources, industry practice, competing architectures), launch research agents BEFORE the interview and summarize findings for the user first. If codebase exploration already yields a clear root cause with concrete evidence, research may be skipped — note the skip. If research was supplied as a reference file or pasted prose, the sub-step 1 summary satisfies the announcement.

7. **Project context.** Explore relevant existing material before the interview — `docs/`, `docs/adr/`, `docs/plans/`, `specs/`, `tickets/`, `templates/`, prior design docs, and the relevant game module (`games/*`), `engine-core`, schema, or trace files when the topic touches them. Launch Explore agents for broad surveys. For frustration-triggered brainstorms, also explore the history of attempted fixes — repeated remediation or accumulated workarounds signal that Step 3 should include radical options (reset, strip, rebuild) alongside incremental fixes. For triage brainstorms whose input is a report, check whether that report (or an earlier version) was already triaged into an existing ADR/agent task/`docs/triage/` file, and frame the current pass as a delta so already-accepted or already-rejected items aren't silently re-proposed. **For UI or runtime-behavior topics with a running instance available**, exploration MAY also include driving the live app (Puppeteer/Playwright — navigate, screenshot, read the DOM) to reproduce the defect and confirm the root cause before proposing; present the reproduction as empirical findings per Step 2 rule 10.

   **Sequential / roadmap-driven brainstorms.** When the request is "produce the next X to continue Y" (the next spec for a roadmap, the next stage of a staged plan), the retread risk is cross-session, not just intra-session. Before proposing, read the progress index for that series — a `specs/README.md` spec index, a roadmap status table, or the existing `specs/`/`tickets/` files — pick the lowest unit whose status is not done, and frame the deliverable as exactly that increment. Make updating the index status part of the deliverable so the next cycle resumes cleanly; authoring a spec flips its index row to the *spec-written* status (e.g. `Planned`), not the terminal *done* status, which waits until its exit criteria pass with evidence. If no index exists yet (greenfield series), create one alongside the first deliverable so future cycles have a tracker; confirm the empty/greenfield repo state explicitly, since "the first unit" is then the load-bearing premise.

8. **Confidence checkpoint.** After all context reading, announce post-exploration confidence in one sentence on its own line (e.g. `Post-exploration confidence: ~90% — reference file is exhaustive, scope fully specified by the request`). This drives the Step 2 decision and is the audit-trail anchor before any triage recommendation.

## Step 2: Confidence-driven interview

The core of the skill. Reach **95% confidence** about what the user actually wants before proposing solutions.

### The protocol

After each user answer, display a confidence block:

```
Confidence: X%
Gaps: [specific list of remaining unknowns]
```

Keep asking until confidence reaches 95%, then announce: "I'm at 95% confidence. Moving to approaches." Under auto mode with a 0-2 question interview, an inline prose alternative is fine (`Confidence: ~90% — gaps listed as named assumptions below`) provided the gaps surface as named assumptions in the very next message.

### Interview rules

1. **One question per message.** Never batch questions in the discovery interview. (This does not restrict `AskUserQuestion`, whose multi-question schema is the sanctioned form for batching approach-selection or section-approval decisions outside the discovery loop.)
2. **Prefer multiple-choice** when the answer space is bounded; open-ended when it isn't. Use `AskUserQuestion` with labeled options when its schema is available; inline numbered options are an acceptable fallback (and are preferred in plan mode for speed).
3. **Probe motivations before solutions.** Ask "What problem does this solve?" and "What happens if we don't?" before "What do you want built?" The first request often describes a solution, not the problem.
4. **Challenge premature specificity.** If the user jumps to implementation details, ask why that specific approach matters. The constraint is often softer than stated.
5. **Detect "should want" vs "actually want".** Watch for buzzword-heavy descriptions, over-scoped requests, vague success criteria ("it should be good" — probe what "good" means), and solutions stated as requirements.
6. **Name your uncertainty specifically.** "I don't know whether this needs to handle edge case X" is useful; "I need more information" is not.
7. **Respect user expertise.** If the user gives a clear, well-reasoned answer, don't re-ask it in different words. Advance.
8. **Handle delegation gracefully.** If the user says "you decide," re-evaluate the options against FOUNDATIONS and project constraints, present your reasoned recommendation, and advance. Don't re-ask.
9. **Verify load-bearing technical premises.** When a user (or reference) premise is load-bearing for the recommendation — crate/library status, version compatibility, feature presence, external-tool behavior — verify it (Cargo metadata, docs, targeted codebase read) before adapting the recommendation to it. The 30-second check prevents recommending against a true premise the user momentarily mis-stated.
10. **Present empirical findings before asking questions.** When exploration produces concrete findings (data, root-cause evidence, confirmed/refuted hypotheses), present a concise summary — hypothesis, evidence, verdict per hypothesis — before the first interview question.

### Confidence scoring

| Range | Meaning | Action |
|-------|---------|--------|
| 0-30% | Don't understand the problem yet | Ask about the problem, not the solution |
| 30-60% | Understand the problem, unclear on constraints | Ask about constraints, success criteria, scope |
| 60-80% | Understand problem + constraints, unclear on priorities | Ask about tradeoffs, what matters most |
| 80-95% | Clear picture, a few edge cases/preferences unknown | Ask targeted questions about specific gaps |
| 95%+ | Ready to propose | Move to Step 3 |

Confidence rises from **both** user answers and research findings; when research closes a gap, note which gaps it closed vs which still need user input.

### Early exit

If the user says "just go" or "that's enough questions," respect it: announce current confidence, list remaining gaps as assumptions you'll make, and proceed. Mark those assumptions explicitly in the design so the user can correct them. Use the format `(N) X — assuming Y` (the unknown as X, the assumption after `— assuming`) so a correction is one line.

### Recovery/reset brainstorms

When triggered by frustration indicators ("huge mistake", "wrong approach", "start over", "strip everything"):

1. **Validate the diagnosis before accepting it.** Present what exploration revealed — both what's broken AND what's working — before agreeing with a scorched-earth instinct. The user may be catastrophizing, or the system may be more salvageable than they think.
2. **Focus confidence on "what's actually broken?"** before "what's the fix?" Is the problem structural (architecture, kernel boundary) or tactical (configuration, content, missing inputs)? Is the frustration proportional to the evidence?
3. **Always include a radical option** in Step 3 (strip, delete, rebuild) — even if you recommend a less radical path. Validating the aggressive instinct as a real choice respects the user's judgment.

## Step 3: Propose approaches

Present **2-3 distinct approaches**, each with:

- **Name** — a short descriptive label
- **How it works** — 2-4 sentences
- **Tradeoffs** — what you gain, what you give up

Close with a recommendation: name the chosen option upfront, then justify in 1-3 sentences. The recommendation is one global decision, not a per-approach attribute.

**Sub-variant splitting.** If an option's "How it works" hides two internally distinct paths with materially different cost/risk, split them into sibling options or labeled sub-rows (`C1`, `C2`) rather than bundle — otherwise the user's selection forces an immediate clarification round.

**Comparing concrete artifacts.** When approaches differ in a concrete, comparable artifact (file layout, section set, transformations table, deliverable scope), use `AskUserQuestion` option `preview` fields to render the candidates side by side, so the user compares the actual artifacts rather than labels.

**For product-behavior topics**, annotate each approach with FOUNDATIONS alignment: `Foundations: <principle> @ <surface> (aligns / tensions — reason naming the surface)`. Surfaces include setup/authoring, legal-action generation, validation gate, state transitions, scoring, RNG/determinism, semantic effects, replay/hash, public/private view projection, serialization, bots, effect-log UI/renderer, the `engine-core` boundary, the static-data boundary, and the IP boundary. Naming the surface forces a second-order check that the proposal actually touches what the principle governs. If any approach tensions a FOUNDATIONS principle or trips a §12 stop condition, flag it explicitly and get sign-off — do not recommend a violating approach silently.

**If the problem space is fully constrained** (a reference provides a proven design, or requirements eliminate alternatives), present a single approach with a one-sentence rationale naming the constraint that narrows the space. Don't invent artificial alternatives.

**If the architecture is settled but a scope/density/coverage dimension is the live decision** (common for content-document deliverables — `README.md`, `CLAUDE.md`, config docs — where there is one obvious design and the real question is how much it should contain), present the **tiers along that dimension** as the options (e.g. minimal / standard / comprehensive) rather than forcing distinct architectural approaches. Render the tiers with `AskUserQuestion` `preview` fields per the note above, so the user compares the actual candidate artifacts. This is the natural shape when the "approach" axis is volume, not structure.

**For a Port external skill deliverable** (adapting an external skill into this repo), the approach IS a transformations table — per-element strip / replace / preserve decisions, one row per substitution site (not per source line). The **source skill being ported is reference material** — it is the authoritative input for the transformation, not a file read for orientation, so give it the Step 1 sub-step 1 reference summary. Present the table as the approach/design so the user can redirect any row. This inline guidance is self-contained; `references/triage-and-deliverables.md` §Deliverable classification carries the same Port-external-skill rules for cross-reference. **A substitution not itemized in the table is out of scope until itemized** — itemize file drops (e.g. a source `examples/` dir), domain re-mappings, and convention removals as rows, not loose prose. **Row granularity**: one *decision* is one row, not one token — a set of related tokens sharing a single strip/replace decision (a whole stripped domain vocabulary, an entire stripped checklist section) is one row that names the decision and lists the tokens. A large or heterogeneous port may present several grouped tables (strip / replace / preserve / per-co-ported-file) rather than one flat table; the invariant is that every substitution site is itemized and individually redirectable, not that there is exactly one table. **Co-ported dependency files**: a ported skill often depends on co-ported files from the source repo (templates, READMEs, referenced docs the skill hard-depends on); itemize each as its own strip/replace/preserve row — they typically carry the same source-repo residue as the skill itself. This shapes Step 3/Step 4; the Step 5 destination row points back to the same rule.

**If the user challenges a dismissal**, redo the analysis from first principles rather than defending prior reasoning; if it reverses, say where the prior reasoning was incomplete. **If the user clarifies a constraint**, redo the approach proposal under the refined constraint without restarting Step 2 — cite which assumption was sharpened. Brief confirmation suffices when the recommendation still holds; full re-presentation is reserved for clarifications that flip it.

**Wait for the user to choose** (or to ask you to refine/combine) before proceeding.

### Triage brainstorms

When the deliverable is a set of work items — or a single recommended fix — evaluated against a report, finding-set, or diagnostic question, replace the approaches step with a **triage recommendation**: which items warrant action, which are dismissed, and why. The user's approval of the triage serves the same gating purpose as choosing an approach. Render the Step 1 sub-step 8 post-exploration confidence sentence on its own line before the triage recommendation as the audit-trail anchor. Verdict types, per-item structure, verdict-bucket grouping, the out-of-report and no-source-report cases, and the closing structure live in `references/triage-and-deliverables.md` §Triage recommendation structure. A "figure out why X" diagnostic that resolves to one inline fix still routes through that reference's §No-source-report diagnostic case — the single answer is the finding, keyed `O1`; a diagnostic yielding several findings keys them `O1`, `O2`, … in that same out-of-report sequence (never a new prefix), and each carries a disposition per the reference's §No-source-report case.

## Step 4: Present design

**Plan mode**: skip per-section gates. Present key decisions in 1-2 messages with conversation-level checkpoints, then write to the plan file. `ExitPlanMode` carries approval.

**Classification pivot check**: if the design reveals a deliverable type different from the Step 1 classification (e.g. a "docs tweak" now changes validation behavior), re-state the refined classification before presenting Section 1.

Once an approach is chosen, present the design **section by section**, scaling each to its complexity (a sentence for trivial parts, up to ~200 words for nuanced ones). Cover the relevant sections, renaming/combining to fit the topic's natural structure:

1. **Overview** — what this achieves, 1-2 sentences
2. **Architecture / structure** — how the pieces fit together
3. **Key decisions** — important choices and why
4. **Data flow / process** — how information moves
5. **Edge cases** — tricky scenarios and how they're handled
6. **Testing / verification** — how to confirm it works (unit, rule, golden trace, property, simulation, replay, serialization, visibility/no-leak, bot legality, UI smoke, benchmark — per `docs/TESTING-REPLAY-BENCHMARKING.md`)
7. **FOUNDATIONS alignment** — for product-behavior topics, a 3-5 row table: principle (`§N <name>`), stance (`aligns` / `tensions` / `N/A`), and a one-line rationale citing the specific design mechanism and the surface it operates at. List only principles the design actively engages or tensions, plus any relevant §12 stop condition the design must clear. Omit for dev-process/tooling/docs topics unless the design touches a FOUNDATIONS principle.

**After each section**, ask "Does this section look right?" and wait for confirmation. If the user pushes back, revise before continuing. After 3 consecutive approvals with no pushback (2 under auto mode), batch remaining sections in groups of 2-3 — announce the batching. Keep a section standalone if it's substantially more complex or higher-risk.

**Carve-outs** (see `references/triage-and-deliverables.md` §Design-presentation carve-outs for detail):
- **Small deliverable** (≤4 decisions, ≥85% confidence): present as a single structured artifact approved in one turn. Announce the consolidation outside auto mode.
- **Template-structured deliverable** (an ADR, an agent-task ticket, a spec with the `specs/README.md` canonical section set, or a skill file with its own canonical sections): the template provides the navigation; present the full draft as one artifact, one approval covering it — and under §User pre-authorization that approval is the recommendation presentation, so the draft may be written directly in the same turn.
- **Multi-deliverable triage**: triage approval covers all N deliverables together; each is then approved as a single template-structured draft (no per-deliverable gate).

**Verify operator-introduced design premises before persisting them.** When the design names a specific field, file path, schema shape, crate/module, trace format, or build/test/bench command from the codebase, verify it exists (a quick Read or grep) before writing it into the deliverable — the 30-second check prevents a premise falsifying during implementation. For removals, merges, renames, or moves, enumerate ALL references with a broad grep before drafting the files-to-touch and acceptance criteria. When the design **reuses** an existing tool, script, or codepath for a *new input* (a new game, variant, or data shape), "exists" is insufficient — verify it actually accepts that input (a quick run, or a read of its input-parsing path) before quoting a cost/scope estimate; a parser or codepath hardcoded to the original input is a common falsifier, and the unverified reuse premise silently inflates a "minimal change" estimate.

## Step 5: Persist the deliverable

After design approval, do NOT implement until the user picks an implementation option from the Step 6 menu — the design doc is typically the deliverable; implementation is a separate, user-chosen act. Exception: inline ops/setup and in-place edits are executed during the session (the HARD-GATE still requires approval first), and no separate design doc is written.

**Quick triage of deliverable shape → destination** (full per-type rules in `references/triage-and-deliverables.md` §Deliverable classification):

| Deliverable shape | Destination |
|---|---|
| Inline ops/setup task or mechanical-fix batch | execute inline + summary; no file persisted |
| New skill design | `.claude/skills/<name>/SKILL.md` |
| Modify existing skill file(s) | edit in place |
| Project documentation & root instruction/config files (`README.md`, `docs/*.md`, `templates/*.md`, `CLAUDE.md`, `AGENTS.md`) | edit/create in place |
| New dev-tooling/CI/config file (`.github/workflows/*`, lint/format/build config) | create in place |
| Architectural decision, or amend/supersede an accepted ADR's doctrine or `docs/FOUNDATIONS.md` (constitutional change) | `docs/adr/NNNN-<slug>.md` from `docs/adr/ADR-TEMPLATE.md` — flag a constitution supersession explicitly, require sign-off |
| Port external skill into repo | new skill file(s) (`SKILL.md` + any `references/`) + delete source + a transformations table itemizing per-element strip/replace/preserve (see the Step 3 Port-external-skill note + `references/triage-and-deliverables.md` §Deliverable classification) |
| Replaces an existing artifact | new file + delete old + update cross-references |
| Implementation work item (agent-task ticket) | filled `tickets/_TEMPLATE.md` → `tickets/<PREFIX>-NNN.md` (decomposing a spec: `/reassess-spec` then `/spec-to-tickets`) |
| Decision + its implementing tickets (a fix needing both a recorded decision and the work items that carry it out) | ADR first (`docs/adr/NNNN-<slug>.md`), then tickets that cite it in `Deps`; encode inter-ticket dependencies; one combined Step 6 menu — see `references/triage-and-deliverables.md` §Deliverable classification |
| Spec (roadmap-gate / stage implementation plan) | `specs/<gate-or-stage>.md` + maintain the `specs/README.md` index (progress tracker) — see `references/triage-and-deliverables.md` §Deliverable classification |
| Triage → ≥2 ADRs / ≥3 agent tasks | deliverables + `docs/triage/YYYY-MM-DD-<topic>-triage.md` |
| Triage analysis, all deliverables deferred | `docs/triage/YYYY-MM-DD-<topic>-triage.md` (full verdict content) |
| Design doc (default) | `docs/plans/YYYY-MM-DD-<topic>-design.md` |
| Research brief for an external researcher | `docs/research/<topic>-research-brief.md` |

A default design doc consolidates all approved sections into a clean document with a "Brainstorm Context" header (original request, reference file if any, key decisions that shaped the design, final confidence and assumptions). ADRs and agent tasks skip the design doc — the ADR/task IS the design. An **ADR** follows `docs/adr/ADR-TEMPLATE.md`'s section set (Context, Decision, Alternatives considered, Consequences, Determinism / Replay-hash / Visibility / Data-Rust-boundary / `engine-core`-contamination / UI / Bot / IP / Benchmark impact, Migration notes, Review checklist); assign its ID by scanning `docs/adr/` for the highest existing `NNNN-` filename prefix and claiming the next integer, zero-padded to four digits (first is `0001`; the filename carries no `ADR-` prefix; `ADR-TEMPLATE.md` is not numbered). An **agent-task ticket** fills `tickets/_TEMPLATE.md` (Problem, Assumption Reassessment, Architecture Check, Verification Layers, What to Change, Files to Touch, Out of Scope, Acceptance Criteria, Test Plan) following the `tickets/README.md` authoring contract — do not invent a different structure when the template exists. Decomposing a spec into tickets routes through `/reassess-spec` (assumption reassessment), then `/spec-to-tickets` (one ticket per reviewable diff).

If working in a git worktree, resolve every output path against the **worktree root**, not the main repo root, and confirm the file landed there before the Step 6 menu.

Do NOT commit the file. Leave it for user review.

## Step 6: Next-steps menu

| Deliverable category | Step 6 action |
|---|---|
| Plan mode active (any category) | Call `ExitPlanMode` — skip the menu |
| ADR / agent task / triage / proposal / spec — or any deliverable whose options describe NEXT phases (decompose / implement / done) | **Menu MANDATORY**, even when written this same turn — writing the file is pre-implementation, and skipping the menu strips the user's named next-phase options |
| Inline-completion (skill design; port external skill; small inline ops/setup; new dev-tooling/CI/config files; new or in-place-edited project-doc / root-instruction files — `README.md`, `docs/*.md`, `templates/*.md`, `CLAUDE.md`, `AGENTS.md`; in-place edits to existing ADR/agent-task/docs where the edit IS the completed work) | Skip the menu; summarize what was done |

Categorize by the middle row first; the inline-completion skip fires only when the deliverable falls outside it. For a mandatory menu, present options like:

```
Design doc written to docs/plans/YYYY-MM-DD-<topic>-design.md

What would you like to do next?
1. Record the decision as an ADR (write to docs/adr/)
2. Decompose into tickets (`/reassess-spec`, then `/spec-to-tickets` → `tickets/<PREFIX>-NNN.md`)
3. Start implementing directly
4. Done for now — I'll review the design doc later
```

Adjust options to the deliverable: for an ADR, offer decompose-into-agent-tasks / implement / done; for agent tasks, offer begin-first-task / refine / done; for an ADR-plus-implementing-tickets set, present one combined menu covering the set (record/accept the decision / refine / implement the first ticket / done) rather than a separate menu per file. Use `AskUserQuestion` when its schema is available; inline numbered options are an acceptable fallback. If the user picks an option that invokes another skill, invoke it. If they pick "done," end.

**If a Step 6 option spawns a follow-up brainstorm cycle for related scope** (a next agent task alongside the just-written one, a related ADR the menu surfaced — distinct from re-triaging the prior verdicts): re-enter at Step 1, not Step 2; emit the Step 1 sub-step 8 post-exploration confidence anchor for the new cycle (even when exploration is trivial); the prior cycle's context typically elevates initial confidence so the interview-skip threshold applies.

## Guardrails

- **YAGNI ruthlessly.** Strip optional extras from every design unless the user explicitly asked for them.
- **No scope inflation.** The design covers what was asked. Resist "while we're at it" improvements.
- **One question at a time** during the Step 2 discovery interview. (Does not restrict `AskUserQuestion` batching outside the loop.)
- **No implementation before approval.** The HARD-GATE means what it says, subject only to the §User pre-authorization carve-out.
- **FOUNDATIONS is authoritative.** For product-behavior topics, if an approach violates a FOUNDATIONS principle or trips a §12 stop condition, flag it immediately and get explicit sign-off. Never design against the constitution silently; a feature that genuinely needs to diverge requires an ADR superseding that principle first (FOUNDATIONS: "Supersede only by accepted ADR").
- **Respect early exit.** If the user wants to skip ahead, let them — list your assumptions clearly.
- **Worktree discipline.** If working in a worktree, all file paths use the worktree root.
- **Blocker discovery during implementation.** If implementation reveals an architectural issue or a cost/scope re-estimate that blocks the deliverable or invalidates the approach ranking, present the blocker, options, and a recommendation. If the fix is small (< 50 lines, single file, no kernel-boundary or public-interface change — a *backward-compatible* generalization where no existing caller breaks and a regression test covers the new path does not count as a "public-interface change" here, whereas a signature or contract change that breaks existing callers does), fix it inline and continue; otherwise create an agent task and either work around it with a documented constraint or implement the fix with user approval. **Pre-write case:** when the blocker is surfaced not during implementation but during a Step 4/5 verification *before* any deliverable is written, and its fix would add a deliverable of a new class or cross the deliverable count, the §User-pre-authorization shape-confirmation rule takes precedence over this fix-inline allowance — pose a one-shot `AskUserQuestion` confirming the expanded shape before writing, even under pre-authorization, however small the fix.
- **Auto mode** applies when a system reminder instructs continuous operation, the user issues an explicit "just go" directive, or a harness `auto_mode` flag is set. Under auto mode: clarifying questions are minimized (gaps become named assumptions), the Step 4 batching threshold drops from 3 to 2 consecutive approvals, and small/template-structured deliverables may be written in the same turn as approval. Approach-selection and section-approval gates still hold, and the HARD-GATE is never relaxed.
- **User pre-authorization.** When the request pre-authorizes a deliverable contingent on the verdict ("write an ADR if warranted", "implement if changes are needed") — or issues an unconditional build/port/"adapt" directive naming a specific deliverable ("adapt this skill into a repo-local one", "port X", "build Y"), which is an even stronger authorization than the contingent case — the HARD-GATE is satisfied by presenting the design or triage recommendation (for a Port external skill, the transformations table) — no separate approval round-trip. Pre-authorization satisfies the HARD-GATE; it does NOT skip the Step 6 menu. The user can still redirect between presentation and write by issuing a counter-instruction. When a remaining gap would materially change the deliverable shape (count crossing a previously-stated boundary — 1↔multi, or N↔N+ when the addition is a new class — type changing class, or a new deliverable file becoming required), prefer `AskUserQuestion` to confirm the shape before writing even under pre-authorization. This rule takes precedence over the §Guardrails "Blocker discovery" fix-inline allowance when a Step 4/5 pre-write verification surfaces the blocker (see that bullet). **Exception:** if the recommendation would reverse a decision the user previously committed to (an accepted ADR, a `docs/triage/` decision record, a deliberately-rejected item), surface the reversal prominently and pose a direction-confirmation `AskUserQuestion` before writing — pre-authorization does not extend to silently overturning a prior user decision.

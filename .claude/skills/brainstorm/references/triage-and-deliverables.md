# Triage & Deliverables

Detailed rules for two SKILL.md branches: the triage recommendation (Step 3 §Triage brainstorms) and deliverable classification (Step 5). Also collects the Step 4 design-presentation carve-outs.

---

## Triage recommendation structure

Used when the brainstorm evaluates a report, finding-set, or diagnostic question and produces work items, instead of proposing approaches.

### Per-item structure

Each triage item has:

1. **verdict** — one of the types below.
2. **rationale** — 1-2 sentences naming the FOUNDATIONS / codebase / boundary grounds.
3. a **conditional sub-field** — `modification scope` (for accept-with-modification) / `alternative path` (for reject) / `deferred_to` (for defer) / `verification source` (for refuted-by-verification). Absent for accept / already-resolved.

### Verdict types

| Verdict | Sub-field | Use when |
|---|---|---|
| `accept` | none | item warrants action as recommended |
| `accept-with-modification` | `modification scope` | item warrants action with refinements (scope-narrowed, severity-shifted, technique-substituted) |
| `reject` | `alternative path` | item declined with no positive scheduling intent; pair with what to do instead (or "none") |
| `defer` | `deferred_to` | item judged sound but routed to a follow-up deliverable; names the follow-up shape + the trigger condition for re-evaluation |
| `already-resolved` | none | re-triage case: the item was actioned between the original pass and this one; cite the resolving artifact + date |
| `refuted-by-verification` | `verification source` | the item's claimed gap or premise is disproved by codebase/contract verification at triage time; quote the file:line evidence |

The seven-bucket vocabulary is closed — don't coin new verdicts. A user-elected skip ("skip the polish for now") is a `defer` whose trigger is the user's batch-scoping choice. An item whose premise is refuted but carries a valid residual best folded into another finding uses the dominant verdict plus a rationale cross-reference to the absorbing item's ID.

### Per-item identifiers

Derive from the source report's own numbering when present (`P1`, `R10`, `F-01`). When unnumbered, use `R<N>` for source-report items. Use `O<N>` for **out-of-report** findings (auditor-discovered, no presence in the source report) — always the literal `O<N>` prefix so they're unambiguous in cross-references. IDs must be stable so the user can reference them by number.

### Grouping & out-of-report findings

Group items by verdict bucket so the user can scan by shape (all accept together, all reject together, etc.). In cross-references and the verdict field itself, use the canonical lowercase-hyphenated form (`accept-with-modification`).

Findings discovered during exploration that are NOT in the source report (adjacent pre-existing bugs, schema drift, kernel-boundary leaks) go in a separate **out-of-report findings** sub-section AFTER the verdict buckets, keyed `O<N>` — not a new verdict bucket. But a correction that refutes a *source-report item's* premise HAS presence in the report, so it's a `refuted-by-verification` (or `already-resolved`) verdict keyed `R<N>` in the buckets. Corrections that reframe the whole triage (tied to no single item) go in the triage lead or a verification headline before the buckets.

### No-source-report diagnostic case

When the request is a diagnostic question or exploration prompt rather than a formal report ("figure out why X", "what's happening with Z"), there's no source report to evaluate verdicts against:

- **Omit the verdict-bucket section entirely** — the verdicts are defined against source-report items.
- **Route all findings to the out-of-report sub-section**, keyed `O<N>`. The answers to the user's questions ARE the findings — emit them as `O1`, `O2`, …, not as synthetic `R<N>` items restating the questions.
- **The closing structure still applies.**
- **When the diagnostic resolves to a recommended *action* (not a set of independent work items)** — i.e. the question is "what should we do about X?" and the answer is one course of action weighed against alternatives — the `O<N>` findings carry the *answer* (the diagnosis), and the close borrows Step 3's recommendation shape in place of (or alongside) the deliverable-shape recommendation: name the recommended action upfront, then the rejected alternatives with their grounds, then any optional add-ons. This is the sanctioned blend for action-shaped diagnostics; don't force an action choice into the flat `O<N>` finding list.

### Closing structure

Close every triage recommendation with:

1. **Deliverable-shape recommendation** — one ADR / N agent tasks / mixed batch / in-place edits, per §Deliverable classification.
2. **Named assumptions** — remaining gaps in the format `(N) X — assuming Y`.

For a multi-deliverable triage (≥2 ADRs or ≥3 agent tasks), make the finding→deliverable mapping explicit in the recommendation (either inline `R3 — <summary> → ADR-002`, or a `deliverable → findings` map) so the user can see which accepted finding lands where at approval time.

**`AskUserQuestion` vs named-assumptions at close-out:** if a remaining gap is material-deliverable-shape (changes deliverable type / scope / count), prefer `AskUserQuestion` to settle it before proceeding — even under auto mode or pre-authorization — because a shape mismatch requires rewriting rather than refining. For content-level gaps within a stable shape, prefer `AskUserQuestion` outside auto mode; under auto mode or pre-authorization, default to named-assumptions plus the design-approval gate.

### Worked skeleton

```markdown
## Verification headline (only if a correction reframes the whole report; else omit)

## Triage verdicts

### Accept
- **R<N>** — <summary>[ → <target deliverable, for multi-deliverable triages>]. _Rationale_: <grounds>.

### Accept-with-modification
- **R<N>** — <summary>. _Modification scope_: <refinement>. _Rationale_: <grounds>.

### Defer
- **R<N>** — <summary>. _Rationale_: <reason>. _deferred_to_: <follow-up>; re-evaluate when <condition>.

### Reject
- **R<N>** — <summary>. _Alternative path_: <what to do instead, or "none">. _Rationale_: <grounds>.

### Refuted-by-verification
- **R<N>** — <summary>. _Verification source_: <file:line / grep / agent finding>. _Rationale_: <verbatim evidence>.

## Out-of-report findings (auditor-introduced)
- **O<N>** — <description>. <Resolution: landed in <site> | flagged for follow-up>.

## Deliverable-shape recommendation
<one ADR / N agent tasks / mixed batch — per §Deliverable classification; finding→deliverable map for ≥2 deliverables>

## Named assumptions
(1) <unknown> — assuming <assumption>; (2) ...
```

---

## Deliverable classification

The full per-type rules behind SKILL.md Step 5's quick-triage table.

- **Inline ops/setup task or mechanical-fix batch** — small tooling/ops work or a bounded mechanical-fix batch executed inline with no persisted design artifact (repo setup, local config, a short pre-approved sequence). Skip both the `docs/plans/` design doc and the Step 6 menu; the deliverable is the in-conversation design plus a post-execution summary. The HARD-GATE still requires explicit approval of the consolidated design before executing. The file edits/new files ARE persisted — only the design doc is elided.

- **New skill design** — the deliverable is the skill file at `.claude/skills/<name>/SKILL.md`; the skill file IS the design, so skip the `docs/plans/` doc. Adjust the Step 6 menu (omit "record as ADR"). In plan mode, write the plan file first; write the full SKILL.md as the first post-approval implementation step.

- **Modify existing skill file(s)** — the edits ARE the design; skip the design doc. For a merge, include the new unified file, deletion of superseded directories, and updates to any cross-references.

- **Project documentation & root instruction/config files** — edits to (or creation of) `README.md`, `docs/*.md`, `templates/*.md`, or root agent-instruction/config files (`CLAUDE.md`, `AGENTS.md`) where the doc IS the deliverable; the content IS the design. The Step 6 menu may be omitted when it completes inline in the same turn. (A root instruction file matches neither the `README.md` nor `docs/*.md` glob literally but is handled identically — inline-completion, menu-skip.)

- **New dev-tooling/CI/config file** — a created tooling/CI/config file (`.github/workflows/*`, lint/format/build config) where the file IS the deliverable; the file content IS the design. Created in place; the Step 6 menu may be omitted when it completes inline in the same turn. Verify the commands/actions the file invokes (build/test/lint/bench scripts, action versions) before writing, as for any operator-introduced premise.

- **ADR — architectural decision or constitution change** — when the design records an architectural decision, sets or changes a boundary, or amends/supersedes `docs/FOUNDATIONS.md`, the deliverable is an ADR at `docs/adr/ADR-NNN-<slug>.md` built from `docs/adr/ADR-TEMPLATE.md`; the ADR IS the design, so skip the design doc. **ID assignment**: scan `docs/adr/` for the highest existing `ADR-NNN` and claim the next integer (first ADR is `ADR-001`; `ADR-TEMPLATE.md` itself is unnumbered). **Section structure**: use the template's sections verbatim — Context, Decision, Alternatives considered, Consequences, then the impact sections (Determinism, Replay/hash, Visibility, Data/Rust boundary, `engine-core` contamination, UI, Bot, IP, Benchmark), Migration notes, and Review checklist. Fill the Status field (`Proposed` for a new brainstorm output) and the Date. A constitution supersession (per FOUNDATIONS, "Supersede only by accepted ADR") must be flagged explicitly: name the principle(s) affected and the downstream docs/games/tasks the change unblocks or invalidates, and require explicit user sign-off before writing. The Step 6 menu is mandatory (post-deliverable phase): offer decompose-into-agent-tasks / implement / done.

- **Replaces an existing artifact** — include (a) confirmed deletion of the old artifact, (b) a check for cross-references to it (in other skills, `README.md`, docs, the ADR template's related-documents list), (c) a note of the replacement in the deliverable.

- **Port external skill** — deliverable is (a) the new skill file, (b) deletion of the reference source once verified, and (c) a transformations table enumerating per-element strip/replace/preserve decisions (one row per substitution site, not per source line). The approach focuses on identifying extraneous source-repo elements and their repo-appropriate replacements. A substitution not itemized in the table is out of scope until itemized. **Co-ported dependency files**: a ported skill often depends on co-ported files from the source repo (templates, READMEs, referenced docs the skill hard-depends on); itemize each as its own strip/replace/preserve row — they typically carry the same source-repo residue as the skill itself, and a dependency file not itemized is out of scope until itemized.

- **Implementation work item (agent task)** — the deliverable is a filled `templates/AGENT-TASK.md` persisted to `tasks/<slug>.md`; the task IS the design, so skip the design doc. Follow the template's section set exactly — Context (foundation docs + ADRs + per-game/source notes that apply), Target, Stage (ladder gate + mechanics), Goal, Non-goals, Forbidden changes, Sources/docs, Tests, Benchmarks, Documentation, Output format, Review checklist — and keep the task **bounded, test-driven, and explicit about non-goals** per FOUNDATIONS §11 and AGENT-DISCIPLINE §3 (narrow good tasks, not architecture-seeking vague ones). **Slug**: derive a short kebab-case name from the task's primary subject (e.g. `column-four-legal-actions`); cite it in the deliverable lead so the user can redirect. Agent-task *creation* presents the Step 6 menu; an *update in place* is inline-completed — skip the menu and summarize the delta.

- **Triage producing ≥2 ADRs or ≥3 agent tasks** — additionally write `docs/triage/YYYY-MM-DD-<topic>-triage.md` summarizing the source report, accepted items (with the full path to each ADR/task + a one-line rationale), dismissed items (one-line reason each), and identified-but-unactioned follow-ups. Keep it under ~80 lines; reference deliverables by path rather than duplicating their content. This makes the brainstorm's decisions durable without re-running it. For a single ADR or fewer than 3 agent tasks, skip this file by default — the deliverables are sufficient history.

- **Triage analysis, all deliverables deferred** — when the brainstorm emits verdicts but produces no ADR/agent task now (everything deferred) yet the user wants the verdicts persisted, write the decision record to `docs/triage/YYYY-MM-DD-<topic>-triage.md` with the full triage (source, per-item verdicts + rationale, recommended shape, named assumptions). The file IS the deliverable, so it carries full verdict content (the ≤80-line companion budget does not apply). Step 6 offers: re-invoke `brainstorm` on this file to produce the deferred deliverables / adjust a named assumption / done.

- **Design doc (default)** — when none of the above fit, write `docs/plans/YYYY-MM-DD-<topic>-design.md`, where `<topic>` is a kebab-case short name. Consolidate all approved sections into a clean document with a "Brainstorm Context" header (original request, reference file, load-bearing decisions, final confidence + assumptions). In plan mode, write to the plan file instead.

- **Research brief** — a self-contained markdown report at `docs/research/<topic>-research-brief.md` targeted at an external researcher/LLM whose findings feed a later design (the `deep-research` skill is the in-repo consumer). Inline all schemas, evidence, terminology, hard constraints, and explicit research questions, since the audience has no repo access. For product-behavior topics, include a non-negotiable-constraint section naming the FOUNDATIONS principles the topic engages (Rust-owns-behavior, noun-free `engine-core`, static-data-is-not-behavior, fair explainable bots, IP conservatism, determinism/replay) and any rejection criteria future recommendations must satisfy. Optimize for completeness over brevity. Step 6: feed the brief to the researcher / wait for findings.

**Deliverable pivot.** If the user redirects the deliverable type mid-brainstorm ("actually, make this an ADR"), reclassify and adjust the flow; don't re-confirm — they told you what they want. When the request pre-authorizes a choice among types ("agent task or ADR, whichever fits"), the operator may select based on scope evidence from exploration without re-prompting — cite the scope basis in the deliverable lead.

When persisting ≥3 files, track one task per file so progress is visible. Do NOT commit any deliverable — leave it for user review.

---

## Design-presentation carve-outs

Detail for the Step 4 carve-outs. Each keeps the HARD-GATE — explicit approval of the consolidated artifact (or the per-tier unit) is required before any write.

- **Small-deliverable carve-out.** When the design comprises ≤4 distinct *decisions* (user-approveable choice points where the user could meaningfully redirect — atomic facts following from a parent decision count with the parent) AND confidence is ≥85%, present the design as a single structured artifact (a transformations table, a bullet list of decisions, a short enumerated summary) approved in one turn. Permitted by default under auto mode; outside auto mode, announce the consolidation. When the count is borderline, prefer consolidating — the gate that matters is "can the user review this in one turn".

- **Template-structured-deliverable carve-out.** When the deliverable has its own canonical template (an ADR via `docs/adr/ADR-TEMPLATE.md`, an agent task via `templates/AGENT-TASK.md`, a skill file), the template provides the section navigation; present the full draft as one artifact, and one approval covers it. Covers the common case of a single template-structured deliverable with many small atomic line-items that don't decompose into the Step 4 section list.

- **Multi-deliverable triage navigation.** For triage brainstorms producing ≥2 deliverables, apply the template-structured carve-out per deliverable — each ADR/agent task is approved as a single consolidated draft. The triage approval covers all N deliverables together; no per-deliverable gate fires. (A companion `docs/triage/` file is a companion to the set, not a member of it, and doesn't count toward the ≥2 threshold.)

- **Non-plan-mode fast-track.** When confidence is ≥85%, a single approach is approved with named assumptions covering remaining gaps, and the deliverable is template-structured, the consolidated-draft approval may be collapsed with the approach/triage approval: present a delta summary in the same turn as the file write. For triage brainstorms the triage-recommendation approval transitively covers the write in any mode; for non-triage approach-selection the same-turn collapse needs auto mode *or* §User pre-authorization (the consolidated draft is the user's first look at the full surface, but pre-authorization independently satisfies the gate via the recommendation presentation — auto mode is not separately required when the request pre-authorized the deliverable). The material-deliverable-shape check (Guardrails §User pre-authorization) still fires before a same-turn write — if the shape shifts, confirm with `AskUserQuestion` first.

- **Re-emergent interview during design.** If the user asks a discovery-style question or requests enumeration of open decisions during Step 3/4 ("ask me the questions that need settling to amend ADR-X"), conduct a constrained interview applying the Step 2 rules (one question per message, prefer multiple-choice, name uncertainty, recommend when delegated). Label questions (A, B, C) so the design presentation can cite them. The settled decisions feed the design; the HARD-GATE still holds.

- **Mid-design term-clarification.** When the user asks what a term you introduced means, answer inline with one self-contained explanation (diagram, worked example, or short prose), then continue the section flow — not a labeled-question sequence.

- **Mid-design scope-narrowing.** When the user requests reduced surface area after sections are approved (the architecture stays valid), recompute under the narrowed scope and announce the deltas before re-presenting; name the dropped elements with a concrete re-evaluation trigger each. Update the design doc in place unless the user asks for a fresh one. This is not a register shift and not a term clarification — don't mis-route it as either.

- **Plan-mode interaction.** Per-section approval is replaced by whole-plan approval via `ExitPlanMode`. Present key decisions inline (1-2 messages, grouping related sections) before writing the plan file, pausing after the first message for course corrections. The goal is conversation-level checkpoints, not per-section gates. When the approach is architecturally constrained (single viable option), the confidence announcement, approach proposal, and design presentation may fold into one message.

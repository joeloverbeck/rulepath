---
name: reassess-spec
description: "Use when preparing a Rulepath roadmap-gate spec for AGENT-TASK decomposition. Reassesses a spec at specs/<gate>-<slug>.md against the codebase (docs/, .claude/skills/, existing specs) and docs/FOUNDATIONS.md; identifies issues/improvements/additions, presents findings for approval, then writes the updated spec. Produces: findings report + updated spec file. Mutates: the target spec file on user approval."
user-invocable: true
arguments:
  - name: spec_path
    description: "Path to the spec file (e.g., specs/gate-0-repository-skeleton.md)"
    required: true
---

# Reassess Spec

Reassess a Rulepath roadmap-gate spec against the codebase and `docs/FOUNDATIONS.md`. Validates assumptions, identifies issues / improvements / additions, presents findings for approval, then writes the updated spec.

<HARD-GATE>
Do NOT Write or Edit the spec file until:
(a) Step 6 findings have been presented and the user has responded — either explicit per-finding disposition (fix / defer / reject), OR no explicit objection to a finding (silence on a finding while answering Questions counts as approval; an explicit objection re-opens that finding and requires re-presenting the corrected recommendation first);
(b) Step 7's pre-apply verification table has been emitted in chat, one check + result row per finding, with any detected mismatch reclassified and — for recommendation-changing mismatches — re-presented for fresh approval;
(c) any open Questions from Step 6 have been answered.

This gate holds under auto mode and any autonomous-execution context. Auto-mode carve-out: when Step 6 findings contain no Issues (CRITICAL/HIGH or FOUNDATIONS hard-fails) and no open Questions, Step 7 may proceed without fresh approval, but the pre-apply verification table MUST still be emitted before any Write/Edit.
</HARD-GATE>

## Invocation

```
/reassess-spec <spec-path> [inline user hint]
```

**Argument** (required): `<spec-path>` — path to the spec file. If missing, ask for it before proceeding.

**Glob resolution**: if the argument contains wildcards, resolve via `ls`/`find`; proceed if exactly one match (note the resolution inline), disambiguate if many, stop with an error if none.

**Inline user hint (optional)**: text accompanying the path — a parenthetical, post-dash note, or follow-on message (e.g. `specs/gate-1-race-to-n.md (Note: I'm worried the bot-legality scope is too thin)`) — is an audit-lens constraint. It shapes severity assignment at Step 5 and may reframe Questions at Step 6; it is NOT a second path argument and does NOT override FOUNDATIONS alignment or approved recommendations. When a hint materially shaped a finding's classification, cite it in the Step 6 presentation. A hint that would force a FOUNDATIONS hard-fail (a §11 acceptance-invariant violation or a §12 stop condition) is flagged as a CRITICAL Issue, not applied.

## Process Flow

```
Pre-Process: classify the spec (a / b / c / d) + hybrid detection
       |
       v
Step 1: mandatory reads (spec file + docs/FOUNDATIONS.md)
       |
       v
Step 2: extract references (file paths, types, schema fields, commands, deps, source docs)
       |
       v
Step 3: codebase validation (load references/codebase-validation.md)
       |
       v
Step 4: FOUNDATIONS alignment + §11 invariant / §12 stop-condition check (load references/foundations-alignment.md)
       |
       v
Steps 5-6: classify findings + present to user (load references/findings-and-questions.md)
       |
       v   [user approval gate — HARD-GATE fires here]
       v
Step 7: pre-apply verification table -> write updated spec (load references/spec-writing-rules.md)
       |
       v
Step 8: final summary + suggested next step
```

## Reference Files

Five reference files, each loaded with the Read tool before its step:

- **Step 3** — `references/codebase-validation.md`
- **Step 4** — `references/foundations-alignment.md`
- **Steps 5-6** — `references/findings-and-questions.md`
- **Step 7** — `references/spec-writing-rules.md`
- **Plan mode** — `references/plan-mode.md`, loaded at entry when plan mode is active.

Load each before the corresponding work begins. Loading all of them in one parallel batch right after Step 1 is the simplest path; on-demand loading per step is also fine. Steps 1, 2, and 8 need no reference file.

## Inputs / Output

**Input**: `spec_path` (required). Plan-mode and worktree-root resolution are auto-detected.

**Output**:
- **Findings report** — presented in chat at Step 6 (Issues / Improvements / Additions, severity-ranked; open Questions; optional Substantial Redesign Flag).
- **Pre-apply verification table** — emitted in chat at Step 7 before any Write/Edit.
- **Updated spec at `<spec_path>`** — edited in place on approval. For classification (d): Status flipped to `Done`, Outcome/Acceptance-evidence populated.
- **Post-apply confirmation** — emitted at Step 8 (grep-proofs that eliminated references are gone and corrected ones resolve).

## Prerequisites

Before acting, this skill MUST read:

- `<spec_path>` — the target spec, entire contents.
- `docs/FOUNDATIONS.md` — the constitution. Skip only if read earlier this session and unmodified.
- Every file path, skill directory, and sibling-spec reference extracted at Step 2 — read as part of Step 3.

Reading scope: anything under `specs/`, `.claude/skills/`, `docs/`, `templates/`, `tasks/`, `tickets/`, and the crate/game tree (`crates/`, `games/`, `apps/`, `tools/`). This skill does not author game content — it reasons about specs that plan app behavior.

## Worktree & Plan-Mode Awareness

If working inside a git worktree, ALL paths (reads, writes, globs, greps) resolve from the worktree root. If plan mode is active, load `references/plan-mode.md` at entry.

## Pre-Process: Spec Classification

Classify the spec into exactly one of four classes. Classification drives which Step 3 substeps apply (see the substep table in `references/codebase-validation.md`).

- **(a) New component** — introduces a new crate/module/surface, a new game under `games/*`, a new skill (new `.claude/skills/<name>/`), a new validator, a new tool, or a new doc-governed contract. Full Step 3 checklist applies.
- **(b) Extension** — extends an existing crate, game, skill, validator, schema, or contract without introducing a new one. Most substeps apply; skill-structure (3.5) applies only when a SKILL.md changes structurally; FOUNDATIONS-contract fidelity (3.8) applies only when the deliverable touches behavior-authority / acceptance-invariant / visibility / replay-hash / boundary semantics. **Removal/teardown specs** (deliverable is *deleting* a field, validator, command, crate, or symbol) classify as (b) with 3.6 downstream-consumer analysis load-bearing — an un-removed consumer is a correctness break, not mere drift.
- **(c) Refactor** — structural restructuring with no behavioral change. Substeps 3.0–3.4 apply; skip consumer/fidelity/completeness substeps unless boundaries or SKILL.md content move. Focus on symbol existence, count accuracy, blast radius.
- **(d) Retroactive** — validation concludes (via Step 3 evidence) that all deliverables already landed. **Not pre-selected** — activates only when every deliverable verifies as implemented. A user hint ("I think this gate already shipped") is a soft signal; only Step 3 evidence confirms (d). Step 7 switches to flipping Status to `Done` + populating Acceptance evidence (see the retroactive branch in `references/spec-writing-rules.md`).

**Per-deliverable already-landed**: when one deliverable (or one Work-breakdown item) shipped while others remain pending, the spec stays (a)/(b)/(c) — reframe just that deliverable as historical, cite the delivering commit/spec/task, and route residual sub-tasks to a deferred note. Do not flip the whole spec to (d).

**Removal target that never existed**: when a spec tasks removing a symbol that returns zero matches AND was never created, there is nothing to delete — classify as a stale-premise Issue (HIGH when restated across multiple sections), drop the removal framing, and where the symbol name still appears in Exit criteria / Acceptance evidence, keep it as a trivially-passing absence guard.

**Hybrid specs**: apply the union of applicable substeps, using the most rigorous classification's checklist for shared substeps.

**Externally-generated / commit-pinned specs**: when the spec carries generation-harness scaffolding — a commit-pinned evidence ledger (lists of raw source URLs at a fixed SHA), "exact-commit workflow" / fetch-discipline boilerplate, contamination checks, or references to a foreign repository — treat stripping that scaffolding as an expected HIGH Issue at Step 5, and route any genuine provenance into a normal source-reference section. This is orthogonal to the (a)–(d) class: tag it alongside whichever class the spec's deliverables fall under.

**Re-reassessment shortcut**: if the same spec was reassessed earlier this session and not externally modified, Steps 2–3 may scope to only the references affected by the triggering change. Step 1 still applies.

## Step 1: Mandatory Reads

Read both before any analysis:

1. **The spec file** (entire).
2. **`docs/FOUNDATIONS.md`** — skip only if read earlier this session and unmodified.

Parse the spec's metadata (Spec ID, Roadmap stage, Roadmap build gate, Status, Date, Owner, authority order) and its sections (Objective, Scope [in scope / out of scope / not allowed], Deliverables, Work breakdown, Exit criteria, Acceptance evidence, FOUNDATIONS & boundary alignment, Forbidden changes, Documentation updates required, Sequencing, Assumptions). The canonical spec format is documented in `specs/README.md` and exemplified by `specs/gate-0-repository-skeleton.md`.

**Non-standard sections**: if the spec uses a different section set or a numbered `§Scope` list instead of a `Work breakdown` table, treat each distinct implementation item (or in-scope item) as a deliverable for validation purposes.

## Step 2: Extract References

Extract every concrete codebase reference from the spec:

- **File paths / target tree** — both existing (`docs/FOUNDATIONS.md`, `docs/ARCHITECTURE.md`) and proposed (`crates/engine-core/`, `games/race_to_n/`, `tools/replay-check/`).
- **Type / interface / schema / contract names** — action-tree, command-envelope, effect-envelope, public/private view, diagnostic, checkpoint, hash, serialization-boundary fields; static-data manifest entries.
- **Function / command / surface / crate / module names** — including CLI tools (`simulate`, `replay-check`, `rule-coverage`, `bench-report`, …) and engine/game symbols.
- **Skill / template names** — `.claude/skills/<name>/`, `templates/AGENT-TASK.md`, other `templates/*.md`.
- **Spec sequencing** — the `authority order` list and the Sequencing section's predecessor/successor gate; the row for this spec in the `specs/README.md` index.
- **Source documents** — when the spec cites an external source (a `ROADMAP.md` gate, a brainstorm output under `docs/plans/`, a report) in its Objective / Scope / Work breakdown, extract the path AND enumerate its actionable claims at Step 2 itself (read the source; for oversized docs use targeted greps with permissive anchoring). Tag each claim's adjudication status (accept / reject / defer / unadjudicated) by scanning the spec's Scope / Deliverables / Out of scope. This feeds Step 3.10.
- **Code / tree / schema examples** (Rust signatures, JSON/TOML schema snippets, directory trees) — extract for fidelity checking.
- **Verification surfaces / thresholds** — exit-criteria rows, acceptance-evidence categories, benchmark thresholds, and severity mappings (warning vs blocker per FOUNDATIONS §11).

**Reference-count checkpoint**: before Step 3, emit a one-line note with the reference count and the tracking decision — e.g. `Reference count: 12 — mental tracking sufficient`, `Reference count: 23 — TaskCreate recommended`, or `Reference count: 47 — themed Explore agents (3)`. Three tracking mechanisms, by scale: **mental tracking** for tightly-clustered sets; **`TaskCreate` per-reference tracking** for `>15` references spanning unrelated areas; **parallel themed Explore agents (max 3)** for `>10` references, especially when they cluster into validation themes (see `references/codebase-validation.md` §3 top-matter) — spot-check agent claims with direct Grep/Read. Use an exact integer up to the delegation threshold so the decision is reproducible (not `~20` or `20+`); once the count is high enough to route to Explore agents, the theme decomposition — not the precise integer — drives the work, so an approximate count there is acceptable if the themes are named. A fixed closed set checked by one presence grep counts as 1 reference, not N. A source/evidence ledger — a closed list of URLs/paths cited as provenance rather than as validation targets (common in externally-generated specs) — likewise counts as 1 reference; the exact-integer count then covers only the remaining genuine references.

**Source-document engagement checkpoint** (when source documents are cited): emit a one-line note naming each source and its per-document adjudication counts — `Source-document engagement: <doc>: N claims enumerated, M adjudicated (accept / reject / defer), (N-M) unadjudicated flagged as findings.` When no source document is cited, emit `Source-document engagement: N/A — no external source cited`.

Prioritize references most likely to have drifted (crate/module paths, function signatures, schema fields the spec extends, tool names, sibling-gate sequencing). Stable references (FOUNDATIONS principle names, ROADMAP gate names) can be spot-checked.

## Step 3: Codebase Validation

**Load `references/codebase-validation.md` before classification-driven substep selection.** Then validate every Step 2 reference, applying the substep subset for the Pre-Process classification. Collect everything; do not present findings yet.

## Step 4: FOUNDATIONS Alignment Check

**Load `references/foundations-alignment.md` before alignment classification.** Then check the spec against applicable FOUNDATIONS principles — the §2 behavior-authority split, §3 `engine-core` kernel boundary, §4 `game-stdlib`-earned rule, §5 static-data-not-behavior rule, §11 universal acceptance invariants, the **§12 stop conditions**, and the §13 ADR triggers. Any §12 stop condition the spec would cross, or any §11 invariant it would violate, is a CRITICAL Issue.

## Steps 5-6: Classify and Present Findings

**Load `references/findings-and-questions.md` before classifying.** Classify findings into Issues (CRITICAL / HIGH / MEDIUM / LOW), Improvements, Additions, and Questions; present using the template there.

**Redesign-count checkpoint**: count deliverables (or Work-breakdown items) whose *approach* materially changed (eliminated, replaced with a different mechanism, or restructured beyond a refinement) over the pre-reassessment total. Emit the `N/total` ratio in the Step 6 Classification block. If it exceeds 50%, the Substantial Redesign Flag section is mandatory immediately above Questions.

**Wait for the user before Step 7.** In plan mode, write the plan file per `references/plan-mode.md`, then call ExitPlanMode. Auto-mode / no-stopping directives proceed directly to Step 7 ONLY when there are no Issues (CRITICAL/HIGH) and no open Questions; cite the directive inline.

## Step 7: Write the Updated Spec

**Load `references/spec-writing-rules.md` before writing.** Build the pre-apply verification table and emit it in chat before any Write/Edit. For each finding (keyed `I1`, `M1`, `A1`…), run a targeted check and record the command + result. Reclassify any mismatch (evidence-refining / recommendation-changing / scope-extending) per the reference; re-present recommendation-changing mismatches before applying. Then apply all approved changes, preserving structure and voice.

For classification **(d) retroactive**, Step 7 instead flips Status to `Done` and populates the Acceptance-evidence / Outcome section (see the retroactive branch in `references/spec-writing-rules.md`).

## Step 8: Final Summary

Present:

- Issues fixed, improvements applied, additions incorporated.
- Change inventory grouped by finding type.
- **Post-apply confirmation**: for every eliminated or renamed reference, grep-prove it is gone and corrected references resolve.
- Deferred items and reassessment-driven scope exclusions (with reasons).
- The 1–3 sections that changed most, flagged for review.
- **Classification shift note**, if the effective classification shifted (e.g. "(a) collapsed to (b) after deliverable removal"; "(b) shifted to (d) after Step 3 verified full landing"). Omit if unchanged.
- **Suggested next step**: "Review the updated spec, then decompose its Work breakdown into `tasks/` AGENT-TASK packets — by hand from `templates/AGENT-TASK.md`, or via `/brainstorm` (decompose path). reassess-spec prepares specs for decomposition but does not perform it." For (d): note the spec is now a historical record (Status `Done`) and remind the user to flip its row in the `specs/README.md` index.

Do NOT commit. Leave the file for user review.

## Guardrails

- **FOUNDATIONS is authoritative**: never approve a spec change that violates a §11 acceptance invariant or crosses a §12 stop condition, even if requested — flag it as a CRITICAL Issue instead.
- **Codebase truth**: every reference in the updated spec must be validated. Never propagate stale paths, renamed types, removed functions, or wrong crate/module locations through Step 7.
- **No scope creep**: the deliverable is the updated spec file. Do not write AGENT-TASKs, create tickets, start implementation, or edit sibling spec files — unless a Step 6 question response explicitly authorizes a named sibling-spec edit (record it in Step 8 under a "Cross-spec scope extension" line). Updating this spec's row in the `specs/README.md` index is in scope only when the spec's own §Documentation-updates section calls for it (e.g. a (d) retroactive Status flip).
- **No greenfield approach proposals**: validate and refine the existing design, not alternatives — except when the approach violates a FOUNDATIONS principle or conflicts with an established contract (`docs/ARCHITECTURE.md`, `docs/ENGINE-GAME-DATA-BOUNDARY.md`, `docs/OFFICIAL-GAME-CONTRACT.md`), where a minimum-viable alternative is part of the Issue finding.
- **Substantial redesign flag**: if reassessment changes >50% of deliverables' approach, flag at Step 6.
- **Worktree discipline**: inside a worktree, all paths resolve from the worktree root.
- **Plan-mode discipline**: load `references/plan-mode.md`, write the plan file, call ExitPlanMode, then execute Steps 7–8 after approval.
- **Do not `git commit`**: writes land in the working tree; the user reviews and commits.

## FOUNDATIONS Alignment

| Principle | Step | Mechanism |
|-----------|------|-----------|
| §2 Behavior authority | Step 3.8, Step 4 | Specs are checked to keep Rust the behavior authority and TypeScript presentation-only; a spec that lets TS decide legality is flagged. |
| §3 `engine-core` is a contract kernel | Steps 3.1, 3.8, 4 | Specs touching `engine-core` are checked for mechanic-noun leakage into the kernel; a domain noun in `engine-core` is a boundary-failure Issue. |
| §4 `game-stdlib` is earned | Step 4 | Specs promoting a helper into `game-stdlib` are checked against the mechanic-atlas / third-use ledger; unearned promotion is flagged. |
| §5 Static data is not behavior | Step 4 | Specs introducing static data are checked for behavior-looking fields (selectors, branches, triggers); YAML/DSL without ADR is a §12 stop condition. |
| §11 Universal acceptance invariants | Steps 3.8, 4 | Specs proposing validation, visibility, replay/hash, serialization, or bot behavior are checked to keep validation fail-closed and blocking, views viewer-safe, hidden information non-leaking, and replay/hash deterministic; deviations are flagged. |
| §12 Stop conditions | Step 4 | Any stop condition the spec would cross (mechanic nouns in `engine-core`, procedural static data, YAML/DSL without ADR, TS deciding legality, hidden-info leakage, bot bypass, unbounded agent scope) is a CRITICAL Issue blocking decomposition. |
| §13 ADR triggers | Step 4 | Specs making an architecture-changing decision (replay/hash semantics, visibility contracts, hosted scope, kernel vocabulary, new bot search class) are checked for a required ADR; a missing-ADR decision is flagged. |

## Final Rule

A reassessment is not complete until every reference in the updated spec is validated against the current codebase and `docs/FOUNDATIONS.md`, every approved finding has a pre-apply verification row proving the fix landed, and every eliminated or renamed reference has a post-apply grep-proof that it is gone.

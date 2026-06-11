---
name: skill-audit
description: Use when a Codex skill was exercised in the current session and the user wants to audit its quality, friction, gaps, alignment, or implementation suggestions. Use for end-of-session skill review, after using a skill, after skill-related confusion, or when asked to implement prior audit findings.
---

# Skill Audit

Analyze a Codex skill against the work done in the current session. During the
audit phase, report only: do not modify the target skill. Editing is allowed
only in the follow-up implementation phase, and only when the user asks for it.

## Invocation

Typical prompt:

```text
$skill-audit .agents/skills/<skill-name>
```

The argument is a skill directory. Resolve `SKILL.md` inside it. If the path
does not resolve, try `<path>*/SKILL.md` and then `<path>**/SKILL.md`; use a
single unique match and note the correction. On zero or multiple matches, stop
and report the error.

## Audit Workflow

1. **Read the target skill.** Read `SKILL.md` and parse its name, description,
   and instructions. If it has `references/`, `templates/`, `scripts/`, or
   `agents/`, list those directories before making file-specific suggestions.
   Re-read after compaction or when the skill changed during the session.

2. **Read alignment documents.**
   - Read `AGENTS.md` when present. It is Codex's repo-local orientation.
   - Read `docs/FOUNDATIONS.md` for product/workflow skills that can affect
     Rulepath behavior, architecture, tickets, tests, or docs.
   - For pure meta-tooling skills such as `skill-audit`, product-foundation
     alignment is usually N/A; say so rather than forcing product findings.

3. **Reflect on session evidence.** Review the current conversation and tool
   results for:
   - unclear, ambiguous, skipped, reordered, or worked-around instructions;
   - unexpected inputs, edge cases, or missing guidance;
   - places where Codex had to improvise beyond the skill;
   - outcomes that diverged from the skill's intent;
   - branches not exercised this session.

   For self-audit, use evidence from prior audit invocations in this session,
   including follow-up implementation work. If there were none, report
   `No session evidence available -- self-audit with no prior invocations` and
   skip evidence-dependent findings.

4. **Cross-check alignment.** For each finding, check whether the skill
   contradicts or fails to implement:
   - `AGENTS.md` repo orientation and workflow expectations;
   - `docs/FOUNDATIONS.md` sections, especially universal invariants and stop
     conditions, when foundation alignment applies.

5. **Classify findings.**
   - **Issue**: broken, misleading, contradictory, or already caused wrong work.
   - **Improvement**: refinement to existing behavior that would have reduced
     observed friction.
   - **Feature**: a new capability that fits the skill's stated intent but was
     not exercised enough to prove as an issue or improvement.

6. **Assign severity.**
   - **CRITICAL**: would corrupt state, produce wrong output, or violate a
     foundation invariant. Fix before next use.
   - **HIGH**: already caused rework or wrong output, or is likely to fail on
     the next similar use.
   - **MEDIUM**: caused non-trivial friction or judgment, but the right outcome
     still emerged.
   - **LOW**: wording, coverage, or polish that did not block progress.

   Before finalizing any MEDIUM or higher finding that says content is absent,
   undocumented, or located in a specific section, verify with a fresh read or
   `rg` of the cited file/section. LOW findings may use judgment, but verify
   when uncertain.

7. **Report using the template.** Output the report in the conversation. Do not
   write a file or edit the target during the audit phase.

## Report Template

```markdown
# Skill Audit: <skill-name>

**Skill path**: <path>
**Session date**: YYYY-MM-DD
**Session summary**: <1-2 sentences on the work that exercised the skill>

## Alignment Check

- **AGENTS.md**: <aligned / N deviations found / skipped -- not present>
- **FOUNDATIONS.md**: <aligned / N violations found / N/A -- meta-tooling skill>

## Issues

[If none: "0 issues."]

1. **[SEVERITY]** <title>
   - **What happened**: <session evidence>
   - **Skill gap**: <what the skill says or fails to say>
   - **Suggestion**: <specific fix and target file/section>

## Improvements

[If none: "0 improvements."]

1. **[SEVERITY]** <title>
   - **Current behavior**: <what the skill currently says>
   - **Why improve**: <session evidence or reasoning>
   - **Suggestion**: <specific fix and target file/section>

## Features

[If none: "0 features."]

1. **[SEVERITY]** <title>
   - **What's missing**: <gap description>
   - **Why it fits**: <how it matches the skill's intent>
   - **Suggestion**: <specific addition and target file/section>

## Not Exercised This Session

[Omit if every relevant branch was exercised. Otherwise list one-line bullets.]

## Summary

**Total**: N issues, N improvements, N features (N findings) -- N CRITICAL, N HIGH, N MEDIUM, N LOW
```

Write empty buckets as `0 issues`, `0 improvements`, and `0 features`. Before
presenting, recount bucket totals and severity totals independently; both sums
must equal total findings.

## Report Conventions

- Every Issue and Improvement needs session evidence. Purely hypothetical gaps
  belong under Features.
- Do not treat unexercised branches as defects. List them under Not Exercised.
- Suggestions should name the exact file and section when possible, such as
  `SKILL.md` Follow-Up Implementation or `references/audit-patterns.md`.
- If an audit finding should be visible but should not be auto-implemented,
  mark the title `-- skip` or `-- informational`, or end the suggestion with
  `-- no change needed`.
- "Implement all", "implement recommended", and "implement suggestions" mean
  implement every numbered finding that is not explicitly marked skip,
  informational, or no-change-needed.

## Follow-Up Implementation

Use this phase only when the user asks to implement audit findings.

1. **Resolve scope.** Map requested items to findings. If "1 and 3" is
   ambiguous across Issues, Improvements, and Features, ask once.
2. **Re-evaluate current state.** Re-read touched files. If a finding is moot or
   its premise changed, adapt or discard it and tell the user.
3. **Read before editing.** Read every file you will edit in the current
   session. Use `apply_patch` for manual edits and keep changes scoped.
4. **Edit in document order.** Combine overlapping edits into one patch when
   they touch the same block.
5. **Run cascade scans.** After planning each primary edit, `rg` the skill for
   terminology, counts, cross-references, paths, and section names that may have
   gone stale. Update cascades with the primary edit.
6. **Verify the skill.** Re-read or grep changed regions and confirm:
   frontmatter is still valid, numbering and sections are coherent, referenced
   files exist, and related text is not stale.
7. **Cross-skill scan.** List current sibling skills across all skill roots with
   `find .agents/skills .claude/skills -maxdepth 2 -name SKILL.md -print`. When the
   target itself has a twin/port under another root, record an explicit per-edit
   propagate-or-diverge decision for the twin — never an automatic cascade, since
   ports differ by design. If an edit changes shared terminology,
   conventions, prompt forms, output paths, or workflow ordering, grep siblings
   for the affected tokens and repair inconsistencies when in scope.
8. **Summarize per finding.** Report each finding as `implemented`,
   `co-edit with finding N`, `cascade from finding N -- <reason>`, or
   `skipped -- <reason>`.

## Auxiliary Investigation

Auxiliary reads, greps, directory listings, and diffs are allowed when they test
a concrete hypothesis about the target skill. Announce the hypothesis in a short
commentary update before running them, for example:

```text
Investigating whether the skill has stale output-path language: grepping sibling skills for `tickets/`.
```

This applies during both audit and follow-up implementation.

## Guardrails

- Report only during audit; edit only on explicit follow-up request.
- Evaluate the skill against its stated intent. Do not expand scope just because
  a broader tool would be useful.
- Do not propose changes that violate `AGENTS.md` or `docs/FOUNDATIONS.md`.
- Preserve unrelated worktree changes.
- Prefer current repo truth over memory or prior sessions.
- Use the smallest verification that proves the skill remains coherent.

---
name: skill-audit
description: "Use when a skill was exercised during the current session and you want to evaluate its quality, find gaps, or identify improvements. Triggers: end of session, after implementing with a skill, after encountering skill friction."
user-invocable: true
arguments:
  - name: skill-path
    description: "Path to skill directory (e.g., .claude/skills/brainstorm)"
    required: true
---

# Skill Audit

Analyze a skill against the work done in the current Claude Code session to determine whether it has issues, could be improved, or needs new features. **Report only** during the audit — never modify the target skill. Editing happens only in the optional follow-up-implementation phase, and only when the user requests it.

## Invocation

```
/skill-audit <path-to-skill-directory>
```

Example: `/skill-audit .claude/skills/brainstorm`. The argument is the skill directory; the framework resolves `SKILL.md` within it. If the exact path doesn't resolve, glob `<path>*/SKILL.md` then `<path>**/SKILL.md` — use a single unique match and note the correction; on zero or multiple matches, stop and report the error.

## Audit checklist (Steps 1–7)

Steps 1–7 are the audit. Step 8 (follow-up implementation) fires only if the user requests it after the report.

1. **Read the target skill.** Read its `SKILL.md` and parse name, description, and content. If it ships a `references/`, `templates/`, `scripts/`, or `agents/` directory, list those directories first so per-finding suggestions can cite exact file paths. (A Read is satisfied by in-context content from earlier this session; re-read only after compaction, or if the skill was modified this session.)

2. **Read alignment documents.**
   - Read `docs/FOUNDATIONS.md` — skip only if it's already in this session's context, and when skipping, name the load mechanism explicitly (e.g., "already in context via direct Read at message N"). It is the constitution: §1–§12 principles, where §12 is the Stop conditions checklist.
   - If a root `CLAUDE.md` exists, read it too; if absent, treat that as normal and skip the CLAUDE.md alignment check.
   - If a root `AGENTS.md` exists, read it too — it is the Codex/agent repo-local orientation, and these skills are exercised under both harnesses (Claude skills are symlinked into `.agents/skills/`); if absent, treat that as normal and skip the AGENTS.md alignment check.
   - **Meta-tooling carve-out**: when the target is itself a process/tooling skill (e.g., `brainstorm`, `skill-audit`), FOUNDATIONS alignment is N/A — these skills don't touch product behavior. The read may be skipped for such targets. This **includes the spec/ticket pipeline skills** (`brainstorm`, `reassess-spec`, `spec-to-tickets`): they reason about and *enforce* FOUNDATIONS in their artifacts (specs, tickets) but write no product code, so alignment is N/A for their own (markdown) output — while the audit should still confirm the skill's *enforcement instructions* don't weaken a FOUNDATIONS principle (e.g. a ticket-template change that would let a §12 stop condition through).

3. **Session reflection.** Review the conversation for:
   - Moments where the skill's instructions were unclear or ambiguous
   - Steps that were skipped, reordered, or worked around
   - Behaviors the skill didn't anticipate (edge cases, unexpected inputs)
   - Places where Claude had to improvise because the skill gave no guidance
   - Outcomes that diverged from what the skill intended
   - Steps not exercised this session (mark "not exercised" — do not speculate about them)

   **Self-audit** (target is `skill-audit` itself): use evidence from any prior audit invocation(s) this session — including any Step 8 follow-up-implementation phase, not just the Steps 1–7 audit phase (the cascade scan, post-edit verification, and cross-skill scan run in Step 8 and often surface the richest self-audit evidence). If there were none, report "No session evidence available — self-audit with no prior invocations" and skip steps 3–6.

4. **Cross-check alignment.** For each finding, check whether the skill contradicts or fails to implement:
   - Principles from `docs/FOUNDATIONS.md` (reference by § number, including the §12 stop conditions)
   - Conventions from `CLAUDE.md` (by section name) — skip if `CLAUDE.md` is absent
   - Repo orientation from `AGENTS.md` (by section name) — skip if `AGENTS.md` is absent

5. **Classify each finding** into one bucket:
   - **Issue** — something broken, misleading, or contradictory in the skill
   - **Improvement** — a refinement to existing behavior that would make the skill more effective
   - **Feature** — a new capability that fits the skill's stated intent but is currently missing

   **Evidenced-gap tiebreaker**: when a gap *has* session evidence but reads as a "new capability", the Session-evidence guardrail (only purely hypothetical gaps belong in Features) wins over the Feature definition — route it to **Issue** if adjacent existing behavior is wrong or misleading for the new case, else **Improvement** (a clean additive extension of existing behavior). Reserve **Feature** for capability gaps with *no* session evidence.

6. **Severity-tag each finding** — CRITICAL / HIGH / MEDIUM / LOW:
   - **CRITICAL** — skill produces wrong output, corrupts state, or violates a FOUNDATIONS principle. Fix before next use.
   - **HIGH** — a missing guardrail/instruction that already caused rework or wrong output this session, or a plausible near-term failure on next use.
   - **MEDIUM** — friction that cost non-trivial improvisation or non-obvious judgment to work around; the right outcome still emerged, but the path wasn't smooth.
   - **LOW** — wording refinement, coverage gap, or polish that didn't block progress.

   **Pre-finalization verification** — before finalizing any finding tagged MEDIUM or higher whose Suggestion or Skill-gap field claims content is absent, missing, or undocumented, or mis-cites a specific location (phrasings like "Add X", "there is no documented Y", "the skill never mentions Z", "§W currently reads…"), verify the claim by a Read or grep of the cited file/section *before* writing the finding. The 30-second check keeps the report's premises true to the file's actual state, rather than to which content you happened to load. For MEDIUM+ absence/miscitation claims this fresh check is **required** and **in-context content does not discharge it** when that content is partial or was loaded in a prior turn — the bright line: content delivered by the *current turn's* invocation counts as fresh; anything loaded in an earlier turn requires the Read/grep. This case overrides Step 1's general "a Read is satisfied by in-context content" allowance. LOW findings are exempt from the mandatory check, but verify ad-hoc when you're unsure a claim holds.

7. **Present the report** using the template below. Output it to the conversation — do not write a file, do not modify the target.

## Report template

```markdown
# Skill Audit: <skill-name>

**Skill path**: <path>
**Session date**: YYYY-MM-DD
**Session summary**: <1–2 sentences on the session work that exercised the target skill>

## Alignment Check

- **FOUNDATIONS.md**: <aligned / N violations found / N/A — meta-tooling skill>
- **CLAUDE.md**: <aligned / N deviations found / skipped — not present>
- **AGENTS.md**: <aligned / N deviations found / skipped — not present>
[If violations: bullets naming the specific § number or CLAUDE.md/AGENTS.md section and what conflicts]

## Issues

[If none: "No issues identified."]

1. **[SEVERITY]** <title>
   - **What happened**: <session evidence — what went wrong or was confusing>
   - **Skill gap**: <what the skill says or fails to say that caused this>
   - **Suggestion**: <how to fix the skill>

## Improvements

[If none: "No improvements identified."]

1. **[SEVERITY]** <title>
   - **Current behavior**: <what the skill currently says>
   - **Why improve**: <session evidence or reasoning>
   - **Suggestion**: <proposed change>

## Features

[If none: "No features identified."]

1. **[SEVERITY]** <title>
   - **What's missing**: <gap description>
   - **Why it fits**: <how this aligns with the skill's stated intent>
   - **Suggestion**: <proposed addition>

## Not Exercised This Session

[Optional — omit when every step/branch was exercised. Otherwise one-line bullets naming steps or branches the session didn't trigger, surfacing coverage gaps without speculating about them.]

## Summary

**Total**: N issues, N improvements, N features (N findings) — N CRITICAL, N HIGH, N MEDIUM, N LOW
```

Write empty buckets as `0 issues` / `0 improvements` / `0 features` — never omit a bucket or substitute an ad-hoc phrase.

## Report conventions

- **Suggestion specificity** — when a fix could land in either `SKILL.md` or a `references/` file, cite a **single** exact path (e.g., "Add to `references/X.md` §Section") — the report is a plan, so an either/or location menu defers a decision the implementation phase then makes silently. If genuinely undecided, name the chosen default and note the alternative parenthetically. The bare `§Section` form is fine when the section name is unique across the skill's files. If Step-8 implementation lands the fix in a different valid location than cited, announce the relocation per finding (Step 8 item 7).
- **Severity-count double-check** — before presenting, recount each bucket (issues / improvements / features) and each severity (CRITICAL / HIGH / MEDIUM / LOW) independently from the numbered findings, then confirm two invariants: the **bucket counts sum to total findings** AND the **severity counts sum to total findings** (the cheap check that catches a miscount like `1 issue + 2 improvements ≠ 4 findings` or a phantom bucket). Write an empty bucket as `0 <bucket>`, never an ad-hoc phrase. If you correct a count after presenting, strike the wrong line and restate.
- **Implement-all by default** — "implement all", "implement recommended", "implement suggestions" are synonymous: apply every numbered finding. Anything worth numbering is worth implementing. To surface a finding *without* auto-applying it, tag it on the title line: `— skip` (considered and declined), `— informational` (context, no code change), or append `— no change needed` to the Suggestion line. Tagged findings are excluded from "implement all"; everything else is applied. **Exception:** a finding whose suggestion weakens, relaxes, or normalizes overriding a documented safety, permission, or branch-protection default must be tagged `— informational` at report time so it is surfaced but not swept into "implement all" — that decision needs an explicit, separate user opt-in, and auto-applying it tends to be denied by the harness anyway (see Step 8).

## Follow-up implementation (Step 8, on user request)

After the report, the user may ask you to implement specific findings (or all of them). Now editing the target file is permitted — the report-only guardrail applied to the audit phase only.

**Edits are agent-config self-modification.** Target files live under `.claude/skills/` (or another skill root), so the harness may still gate a given edit independently of this report-only→implement transition — especially for content with safety, permission, or branch-protection implications, where "implement all" does not by itself supply the explicit user intent the harness looks for. If an edit is denied: do **not** attempt to bypass it; apply a reduced-scope variant that omits the flagged content when one exists (announcing the reduction), and record the finding as `skipped — blocked: <reason>` in the summary, surfacing to the user that it needs review or a non-auto-mode run.

1. **Scope** — partial ("implement 1 and 3") vs. inclusive ("implement all" / "implement recommended", synonymous). Findings are section-prefixed (`Issue 1`, `Improvement 1`, `Feature 1`); "implement 1 and 3" defaults to the Issues counter — confirm the section if the count is ambiguous.
2. **Re-evaluation** — if any covered file changed since the report, or a re-read shows an audit premise was falsified (it claimed X absent but X is present, or vice versa), re-evaluate each finding against the current state first: discard moot findings, adapt shifted ones, announce the outcome per finding. When nothing changed and no premise was falsified, a single confirmation line suffices.
3. **Read before Edit** — every file you will Edit must have been Read in this session via the Read tool. In-context content from grep/Bash/skill-invocation output does *not* satisfy the Edit tool's validator. For large files, chunked Reads (`offset`/`limit`) covering each edit region satisfy it.
4. **Apply edits in document order** (top → bottom) to avoid line-shift breakage; parallel Edit batching is fine when each `old_string` is unique and non-overlapping. Construct `old_string`s from the current in-context Read of the file, not from audit-phase *memory* — whitespace and list markers are easily misremembered. A complete Step-1 (or later) Read that the harness confirms is unchanged (its wasted-call guard — "file unchanged since your last Read") satisfies this; issue a *new* Read only when the content is partial, was compacted away, or the file may have changed since (an intervening edit or external modification). When two findings target the same contiguous block, apply them as a single Edit covering both rather than forcing two overlapping `old_string`s, and report the secondary as `co-edit with finding M` (item 7).
5. **Intra-skill cascade scan** — after planning each primary edit, scan the rest of the skill's files for related text using the same terminology, concept, or count that would go stale if only the primary changed (search semantic variants too: plurals, count phrases like "three categories", word-form numbers). Apply cascades alongside the primary. Key them `N.cascade` in the summary; when one finding needs co-equal parallel edits at several sites, key `N.a` / `N.b` instead.
6. **Post-edit verification** — re-read or grep each edited file's changed regions (full file if short; edited regions with flanking context if long — a targeted `grep` that surfaces the full changed lines plus enough surrounding structure to judge numbering/sequence is a valid mechanism, consistent with Step 6's "Read or grep" allowance) and confirm: edits don't conflict; numbering/steps/sections stay sequential; cross-references and file paths still resolve; the skill reads coherently end-to-end; YAML frontmatter still parses if touched. Fix and re-run the full pass if any check fails. **Grep-pattern fidelity**: a fragment grep used to confirm an edit landed must include the inserted text's literal markdown punctuation (backticks, asterisks, em-dashes) **verbatim**, or anchor on a punctuation-free substring — and any `grep -c` returning `0` **on a grep confirming inserted text is present** must trigger a substring re-check (and a scan for an over-narrow `grep -v`/filter that excludes the target line) **before** concluding the edit failed; never re-apply an edit on a bare count-`0` result, which risks a double-edit. (A grep confirming *old/stale text is absent* is the opposite polarity — there `0` is the intended success, not a failure signal, and needs no re-check.)
7. **Post-implementation summary** — a status row per finding (`implemented` / `implemented — relocated to <path> (report cited <path>)` / `cascade from finding N — <reason>` / `co-edit with finding M` / `skipped — <reason>` / `skipped — blocked: <harness denial reason>`), so the user gets a clear per-finding outcome. When an edit lands in a different valid location than the report's Suggestion cited, use the `relocated` form so the audit↔implementation record stays reconciled.

## Cross-skill note

This repo maintains skills under more than one root — enumerate the current set with `find .agents/skills .claude/skills -maxdepth 2 -name SKILL.md -print` (each entry is a real `SKILL.md` path, so the full path keeps a skill's root unambiguous and confirms the skill exists in one step — no separate twin-existence check needed) rather than assuming a fixed list (e.g. `brainstorm` — illustrative only; the set changes). Without `-L`, `find` does not descend symlinked skill directories, so once Claude skills are symlinked into `.agents/skills/` each skill lists once (under `.claude/skills/`) rather than double-counting the symlinked twin. Run the enumeration as its own call and build the sibling-grep target set from its output — don't batch the `find` with the greps it is supposed to feed, or the targets get written before the enumeration is visible. When the *target itself* has a twin/port under another root (e.g. a Codex port in `.agents/skills/`), Step 8 records an explicit per-edit propagate-or-diverge decision for the twin — never an automatic cascade, since ports differ by design — formatted per edit as `<twin>: diverge — <reason, e.g. port omits this surface (condensed)>` or `<twin>: propagate — Edit N re-applied at <twin> line M`. A cross-skill check matters only when a follow-up edit introduces or changes terminology, a convention, or an output path a sibling also relies on — in that case, grep each affected sibling for the token and record the outcome (`Scanned <sibling> via grep for <token> — no inconsistencies`, or name what was adjusted). The reverse direction also counts: when an edit adds an *outbound* by-name reference **to** a sibling skill (e.g. "per the `<sibling>` §X rule"), grep that sibling to confirm the cited surface exists before finalizing and record the coupling (`Confirmed <sibling> §X exists — outbound reference resolves`), so the new cross-skill pointer isn't left dangling. Pay particular attention to the spec/ticket pipeline coupling: `brainstorm` → `reassess-spec` → `spec-to-tickets` share the canonical-spec section set and the `specs/` → `tickets/` output paths, so an edit to any one that touches that surface should be checked against the others. A confirming grep is always acceptable to *establish* whether a surface is shared — "omit any cross-skill section" governs the report, not the verification. When no edit touches a shared surface, either omit the section entirely or record a single line (`Cross-skill: no shared surface touched`); never silently skip the check just because you believe the surface is unshared.

## Auxiliary investigation and announcements

Beyond reading the target `SKILL.md`, you may list its directory, read or grep sibling skills, and diff files against named reference sources — but each auxiliary call must support a specific hypothesis about the target's behavior, not probe speculatively. **Announce any tool call beyond the target Read hypothesis-first**, as user-facing text immediately before the call: `Investigating <hypothesis>: <grep/read/list> <target> to verify.` This keeps the audit trail reproducible. A tool's `description` field does not satisfy this — it doesn't render where reproducibility needs it. The hypothesis-first announcement applies equally to Step 8 verification calls (the post-edit re-read/grep and the cross-skill scan), not only audit-phase auxiliary calls.

## Guardrails

- **Report only** during the audit phase — output to the conversation, never modify the target. Step 8 follow-up implementation is the sole exception, scoped to user-directed requests.
- **No false positives** — a step not exercised this session is noted "not exercised", never speculated about as an issue.
- **FOUNDATIONS alignment is mandatory** — any suggestion that would violate a `docs/FOUNDATIONS.md` principle (including a §12 stop condition) must be flagged and rejected, even if it would otherwise improve the skill.
- **Scope discipline** — evaluate the skill as written against its stated intent; don't propose expanding its scope.
- **Preserve unrelated worktree changes** — Step 8 edits touch only the target skill's files; never revert, restage, or disturb unrelated modified files already present in the worktree.
- **Session evidence required** — every Issue and Improvement cites specific session evidence (what happened, what was expected). Purely hypothetical gaps belong in Features.
- **Repeated-audit shortcut** — if the same skill was audited 2+ times this session and the most recent audit found 0 findings with no intervening edits, note "Skill stable — no new session evidence since last audit" and skip the full checklist. If the skill was modified since the last audit, treat the next audit as fresh.

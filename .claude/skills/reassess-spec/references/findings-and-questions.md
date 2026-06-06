# Findings Classification and Presentation (Steps 5-6)

## Step 5: Classify Findings

Organize findings from Steps 3 and 4 into:

- **Issues**: factually wrong, stale, violates a FOUNDATIONS §11 acceptance invariant, crosses a §12 stop condition, makes a §13 ADR-trigger decision without an ADR, or proposes redundant deliverables when existing infrastructure suffices. Blocks AGENT-TASK decomposition.
- **Improvements**: not wrong, but a refinement would make implementation cleaner, safer, or more aligned.
- **Additions**: beneficial features not in the spec that align with its goals. Apply YAGNI — only natural extensions of the spec's scope.

For each finding, record: what the spec says (or omits); what the codebase actually has (with file/crate paths and line references); the recommended change.

Tag severity: **CRITICAL** (blocks decomposition), **HIGH** (fix before decomposition), **MEDIUM** (improves quality), **LOW** (nice to fix).

**Severity-bump re-keying**: a finding that crosses the must-fix threshold (HIGH/CRITICAL) belongs under "Issues", not "Improvements", regardless of initial classification; a finding that drops below HIGH moves to "Improvements". Re-key during drafting, not during the user-facing presentation, so the keys presented at Step 6 are stable.

**Pre-finalization site-counting**: for an Issue or Improvement that corrects a stale claim OR renames/eliminates a token (symbol-name, crate-name, tool-name, mechanism token) restated across multiple spec sections, grep the spec for ALL restatements of the claim/token BEFORE finalizing the finding's recommendation. List each site in the recommendation (e.g. `apply at §Scope + §Deliverables + §Exit-criteria row 3 + §Assumptions A-2`). Use a bare-token grep with `-i` (not the specific phrasing the finding expects to remove) to catch syntax variants. Audit-time enumeration is cheaper than the late-discovery recovery that the Step 7 pre-edit grep and Step 8 post-apply grep otherwise have to absorb.

## Step 6: Present Findings

**Redesign-count checkpoint** (before drafting): count deliverables (or Work-breakdown items) whose approach was materially changed — eliminated, replaced with a different mechanism, or restructured beyond a refinement. Express as `N / total`, where `total` is the **pre-reassessment** deliverable count (dropped deliverables stay in the denominator; added ones increase it; use the same denominator through every emission). Emit the ratio as a one-line note in the `### Classification` block regardless of whether the flag fires. If `N/total > 50%`, the Substantial Redesign Flag section MUST appear immediately above Questions; otherwise omit it. **When the spec has no enumerated deliverable set** (e.g. a narrative spec with no Work-breakdown or Deliverables list), there is no denominator — state that explicitly (`Redesign count: denominator N/A — no enumerated deliverables; redesign assessed qualitatively as none | some | substantial`) rather than fabricating a ratio, and base the Substantial Redesign Flag decision on that qualitative judgment.

Material changes: elimination of a deliverable; replacement of its mechanism; restructuring that changes which crates/files it touches; changing the read/write direction of data flow; introducing a new authoritative state where the original was a derived view (or vice versa). NOT material: field renames, type-shape adjustments that preserve the deliverable's role, signature-preserving reorderings, prose rewording. A verification/evidence deliverable that re-anchors *only because* its drivers were redesigned counts once with its driver, not separately. For a multi-item inventory (a Work-breakdown table, an exit-criteria set), count each materially-changed sub-item toward the numerator but the container as one deliverable in the denominator.

Present in this format:

```
## Reassessment: <spec-name>

### Classification
<spec type (a)-(d)> — <one-line description>. Steps applied: <list>. Steps skipped: <list>.
Redesign count: <N/total> deliverables materially changed (<above | below> 50% threshold; Substantial Redesign Flag <included | omitted>).

### Issues (must fix)
[If none: "No issues found."]
1. **[SEVERITY] <title>** — <spec says> vs. <codebase has>. Recommendation: <change>.

### Improvements (should fix)
[If none: "No improvements found."]
1. **[SEVERITY] <title>** — <current text> could be improved because <reason>. Recommendation: <change>.

### Additions (consider adding)
[If none: "No additions proposed."]
1. **[SEVERITY] <title>** — <what's missing> because <reason>. Recommendation: <new section>.

### FOUNDATIONS & boundary alignment
- <Principle name>: <aligned | see Issue #N [SEVERITY]>

### §12 Stop-Condition Check
[Only if a §12 stop condition is engaged. One line per engaged condition:]
- <condition>: clear | N/A | **flag** (reason)

### Substantial Redesign Flag
[Only if >50% of deliverables change approach: "This reassessment proposes substantial redesign of N/M deliverables. Goals preserved but implementation path changes significantly."]
[If not triggered: omit section.]

### Questions
[If none: "No questions."]
1. <question>
```

**Finding-key convention**: Issues keyed `I1, I2, …`; Improvements `M1, M2, …`; Additions `A1, A2, …`. Preserve the within-category number into Step 7 and Step 8. A candidate finding merged into another during drafting does not receive a key.

## Question Handling

- **Option fidelity**: each option that names a type/field/function/crate must cite its current definition (grepped at presentation time), not a summary. The user's approval binds to the option label, so an imprecise label produces an ambiguously approved fix.
- **Initial round**: at most 3 open-ended questions. Confirmation-shape (y/n, binary-with-default) questions interdependent with the open-ended ones may be bundled in the same round; non-interdependent ones count against the cap and defer.
- **Discrete options (2–4), single question, terse descriptions (≤2 sentences each)**: use `AskUserQuestion` with a recommended default.
- **Discrete options, rich descriptions OR bundled (2–3 questions)**: prefer plain-text bullets with labeled options `(a)/(b)/…` and a recommendation per question under a single `### Questions` heading; the user answers inline (e.g. "1) a, 2) b").
- **Single discrete question coexisting with silence-approval dispositions**: when one discrete (2–4 option) question rides alongside other findings being approved-by-silence, inline plain-text under the `### Questions` heading is acceptable — it keeps the discrete answer and the silent dispositions on one approval surface. Reserve `AskUserQuestion` for when the discrete question stands alone with no concurrent silence-approval flow.
- **Reform-depth question for non-canonical specs**: when Step 3.11 finds the spec missing most of the canonical section set (Header, Work breakdown, Exit criteria, Assumptions, alignment table all absent), do not silently pick a restructuring depth. Present reform depth as an explicit discrete question — typically (a) full canonical rewrite, (b) graft the missing canonical sections while retaining the existing body as a detailed reference, (c) minimal (add only Work breakdown + Exit-criteria mapping). The output structure is a user decision with real effort tradeoffs; recommend a default (usually (b)).
- **Open-ended questions**: present as plain text.
- **Follow-up rounds**: one question at a time; repeat until resolved.
- **Delegated resolution**: if the user delegates ("you decide based on FOUNDATIONS"), resolve by reasoning against the referenced constraint; if resolution needs investigation, run a mini Step 3 scoped to the question. If none of the options is ideal, propose a new one with justification.
- **Conditional approval**: if the user approves an option contingent on a verifiable premise ("go with (a) as long as X is true"), treat the premise as a mini Step 3 — confirm or refute it, surface the outcome in the Step 7 pre-apply row, and proceed only if confirmed; if refuted, re-present the finding with a corrected recommendation.

Wait for user response before Step 7. (In plan mode, this wait is replaced by ExitPlanMode approval — see `references/plan-mode.md`.) Findings are approved unless explicitly objected to: silence on a finding while answering Questions counts as approval; an explicit objection re-opens that finding and requires re-presenting the corrected recommendation before Step 7.

**Auto-mode interaction**: when auto mode (or an in-session no-stopping directive) is active AND the findings contain no Issues (CRITICAL/HIGH or FOUNDATIONS hard-fails) AND no open Questions, proceed directly to Step 7, reporting the auto-approval inline. If any Issue is present or any Question is open, the wait-for-user gate still applies; cite the directive inline when it holds.

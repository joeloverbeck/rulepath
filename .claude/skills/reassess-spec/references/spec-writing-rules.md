# Writing the Updated Spec (Step 7)

After all findings are resolved and approved.

## Pre-Apply Verification

Run targeted checks to confirm each finding still holds, and **emit the verification table in chat before any Write/Edit call** — a vague "I checked the findings" is not sufficient and is treated as no verification. For each finding (by its Step 6 key — `I1`, `M1`, `A1`…), record both the command and the result.

Example:

| Finding | Check | Result |
|---------|-------|--------|
| I1 | `grep -rn "engine-core" crates/engine-core/src \| grep -iE "board\|card\|deck"` | 0 matches — kernel stays noun-free, confirms §3 scope |
| I2 | `test -f docs/ENGINE-GAME-DATA-BOUNDARY.md` | file exists — boundary-doc dependency path valid |
| M3 | Judgment — §11 warnings-vs-blockers reasoning; Q2 delegated | selected (a): the unknown-field check is a blocker, not a warning, per §11 reject-unknown rule |

**Row shapes**:
- **Command-backed** (default): `Finding | <grep / test / cargo / file-read command> | <result>`. Use whenever a symbol can be grepped, a path `test`-ed, a crate built, or a line read.
- **Judgment-only**: `Finding | Judgment — <restated rationale> | <result>`. For purely analytical recommendations, or when the user delegates ("you decide") — append `; Q<N> delegated`. Use sparingly; a bare `Judgment` without rationale is a skipped check.
- **User-answered**: `Finding | User answer Q<N> = (<option>): <one-line paraphrase> | Apply as: <edit description>`. Expand terse replies ("go with (a)") into the canonical form. When the answer confirms existing text and no edit follows, the Result reads `no edit — confirms existing §<section>`.

**Multi-section pre-edit grep**: when a finding's Result names multiple sections to edit, run an exact-string grep for the changed terminology across the entire spec before the first Edit AND before drafting the Result, and record the actual count + line numbers (e.g. `3 instances at lines L1, L2, L3 — apply at all`). Do not estimate ahead of the grep — the grep is authoritative. Cross-section restatements drift silently because the deliverable's number is unchanged.

**Syntax-variant + `replace_all` discipline**: when the changed terminology may appear with different surrounding syntax (parens vs none, capitalization, list markers, trailing punctuation), grep for the BARE token plus `-i` — not the surrounding context. `replace_all: true` matches only the exact `old_string`; it cannot catch sibling sites with different surrounding syntax, so the pre-edit grep is still required to enumerate the variants needing separate Edit calls.

**Mismatch classification** — if a check reveals a finding/codebase mismatch:
- **Recommendation-changing**: the check invalidates the finding's recommendation (the fix no longer applies, the target moved, a different fix is warranted). Re-present the corrected finding and wait for confirmation before applying that finding's edit. A pure retraction (no substitute) needs a transparent `retracted: <reason>` note but not fresh re-approval.
- **Evidence-refining**: the check refines supporting evidence but the recommendation holds. Note the refinement inline in the Result column and proceed.
- **Scope-extending**: the recommendation still applies but fulfilling it requires a new deliverable or change not discussed at question time. Note it inline in the Result column, proceed, and surface it in the Step 8 summary under a dedicated line. (If a Step 6 option already named the consequence, cite the question — it's a confirmation, not fresh approval.)

When in doubt, treat the mismatch as recommendation-changing and re-present — cheaper to ask than to apply the wrong fix.

## Apply Changes

- Incorporate corrections from the user's question responses. Preserve existing structure and voice; change only what was agreed upon. Keep the canonical Rulepath spec section set (see `specs/README.md` and `gate-0-repository-skeleton.md`) — prefer explicit `not applicable` rows over silent omissions.
- Prefer `Edit` for ≤3 localized changes; prefer full `Write` when insertions cause **cascading renumbering** or the change is a **diffuse rewrite** of contiguous prose. The decision keys on the *shape* of the change, not the count of sections touched — many independent surgical edits across many sections are well-served by targeted Edits.
- **Inserting deliverables / Work-breakdown items**: renumber all subsequent items and update intra-spec cross-references (Work-breakdown `Depends on` columns, Exit-criteria rows that name a WB item, Sequencing). **Removing items**: grep the spec for all references to the removed number/ID (Scope, Deliverables, Work breakdown, Exit criteria, Acceptance evidence, FOUNDATIONS alignment, Forbidden changes, Assumptions, cross-references) and update or remove them. Exclude citations to OTHER gates' deliverables (e.g. `Gate 2 §trace hardening`) from renumbering — those are external and preserved verbatim.
- **Material mechanism modification (number unchanged)**: grep the spec for the deliverable's old key concepts (function/type/crate/tool names the modification eliminates) and scan Objective, Scope, Deliverables, FOUNDATIONS alignment, Exit criteria, Acceptance evidence, Assumptions for restatements.
- **Material mechanism redirect** (one approach rejected for another): consider a brief "Why X and not Y" rationale in the affected §Scope or §Deliverables, recording the boundary/contract/FOUNDATIONS reason the rejected approach was insufficient. This authors the spec-level audit trail so future readers don't re-propose it; the §Post-Apply Confirmation audit-trail retention exception then recognizes the rejected token's appearance there as acceptable retention.
- **Assumptions / open-question resolution**: if a finding resolves an entry in the spec's §Assumptions (or a deferred open question), update or remove that entry alongside the primary edit. A "still open" assumption the reassessment actually closed is a misleading audit trail.
- **New deliverable vs. amendment**: when a finding introduces substantial new logic (new mechanism, new type, new tool, new crate surface), consider a new numbered deliverable / Work-breakdown item rather than expanding an existing one — criteria: distinct implementation site, independently testable, would make the existing item unwieldy if inlined.
- **Late-discovered findings**: if writing reveals minor factual errors not in the plan (wrong symbol names, typos, outdated crate paths), fix them and note in Step 8 as "Also fixed:". If a late finding would be HIGH/CRITICAL, re-present before applying. If discovered during edit *planning*, key it `LD-N` in the pre-apply table; if during application or post-apply, Step 8 "Also fixed:" alone suffices.

## Retroactive Branch (classification (d))

If Step 3 concluded all deliverables already landed, Step 7's output is NOT deliverable refinement. Instead:

1. Flip the spec's **Status** to `Done` (the repo's done marker per `specs/README.md`).
2. Populate the **Acceptance evidence** section (and/or an **Outcome** note) with: completion date (absolute); landed changes (cite file/crate paths + line numbers); delivering commit(s), sibling spec(s), or AGENT-TASK(s); deviations from the original plan; verification/exit-criteria commands **re-run at reassessment time** with pass/fail status (do not copy from memory — rerun each to catch post-delivery regressions, e.g. `cargo test`, replay-hash checks, benchmark runs per `docs/TESTING-REPLAY-BENCHMARKING.md`).
3. Mark the historical **Objective / motivating context** as such — a short parenthetical noting the gap it describes was closed by the landed implementation, so a future reader doesn't treat a stale condition as live.
4. Cross-reference any later gates/specs/skills that extended or absorbed the original scope.
5. Do NOT apply structural refinements to deliverables that already shipped — the spec is now a historical record; editing deliverable sections to match current code would confuse the causal narrative.
6. **Index + archival**: the spec's own §Documentation-updates section drives the `specs/README.md` index Status flip to `Done` (do it only if that section calls for it; otherwise remind the user in Step 8). If the spec or repo convention moves completed specs, follow `docs/archival-workflow.md` as the canonical archival process rather than inventing a move.

## Post-Apply Confirmation

Grep the updated spec for:

1. **Eliminated stale references** — should return zero matches. For phrase-elimination, use literal-string `grep` without `-E`; when regex is necessary, prefer `[^.;]*` over `.*` to avoid greedy cross-sentence false positives.
2. **Corrected references** — should return the expected matches.
3. **File/crate path references in newly added deliverables** — should resolve to existing paths (or be clearly marked as proposed targets created by a named Work-breakdown item).
4. **Re-runnable commands** — if a finding added or edited an §Exit-criteria, §Acceptance-evidence, or embedded completeness-sweep/gate command, confirm the command text resolves; for an edited gate-command, re-run it to confirm sane output.

**Audit-trail retention exception**: when an eliminated reference appears in a deliberate "this was removed and why" rejection paragraph, check 1's grep shows N≥1 for the term. Matches inside an explanatory rejection paragraph are acceptable retentions; matches in an active deliverable, instruction, or normative statement indicate incomplete elimination. Cite the retention site explicitly in Step 8 so a future reader can distinguish acceptable retention from unfinished elimination.

For classification (d), additionally: grep every concrete artifact named in the spec's Objective / motivating context (symbols, paths, type names) and prove its landed/corrected form in the current codebase.

Record results for Step 8.

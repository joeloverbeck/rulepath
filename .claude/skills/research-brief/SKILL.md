---
name: research-brief
description: "Use when you need to hand a research task to an external deep researcher (ChatGPT-Pro) and want the comprehensive prompt authored here, with full repo access, instead of in a throwaway ChatGPT-Pro session. Interviews you to 95% confidence, then emits a self-contained, paste-ready requirements prompt and refreshes the upload manifest. Triggers: needing a next spec, a thorny fix, a hardening / boundary-enforcement / anti-leak pass, or a foundational/doc overhaul deep-researched externally. Produces: reports/<topic>-research-brief.md + a refreshed reports/manifest_<date>_<shortsha>.txt. Mutates: only reports/ on user approval."
user-invocable: true
arguments:
  - name: research_target
    description: "What the external deep researcher (ChatGPT-Pro) should produce — the thing to be deep-researched (string). A sentence is fine; the skill sharpens it through exploration and interview."
    required: true
  - name: reference_path
    description: "Optional path to a report, finding-set, or analysis to fold into the brief as established context."
    required: false
---

# Research Brief

Author the comprehensive, paste-ready prompt for an **external deep-research session** (ChatGPT-Pro) — here, where Claude has direct access to the whole repository — instead of reconstructing it interactively in ChatGPT-Pro.

This skill replaces **"Session 1"** of the user's two-stage routine:

- **Session 1 (replaced by this skill)**: explore the repo, interview the user to 95% confidence about their *actual* intent, then author a requirements-style prompt.
- **Session 2 (ChatGPT-Pro, the actual deep researcher)**: receives the uploaded manifest + the emitted prompt, explores and researches online as deeply as needed, and **produces the deliverable directly** — it does not re-interview.

The emitted brief is **self-contained**: ChatGPT-Pro Session 2 has none of this session's context, so everything it needs must live in the prompt plus the uploaded manifest.

<HARD-GATE>
Do NOT Write the brief file or refresh the manifest until BOTH hold:
(a) the interview has reached ~95% confidence (or the user issued an early-exit "just go" — in which case remaining gaps are written into the brief as explicit, labeled assumptions); AND
(b) the Step 5 brief outline + settled-intentions summary has been presented in chat and the user has approved it (silence on a section while answering other questions counts as approval; an explicit objection re-opens that section and requires re-presenting the correction first).
The skill mutates only `reports/`. It NEVER edits `docs/`, `specs/`, `tickets/`, `templates/`, `.claude/skills/`, or source code.
</HARD-GATE>

## Invocation

```
/research-brief "<what ChatGPT-Pro should deep-research>"   [reference_path]
```

## Step 1: Classify the research target

Read `research_target` (and `reference_path` in full if given). Classify into one type — this selects the load-bearing "read in full" set for Session 2 (the type→reads map is in `references/brief-template.md`):

- **new-spec** — what to build/create next for the repo (a roadmap-gate spec).
- **thorny-fix** — diagnose and resolve a stubborn defect or design knot.
- **hardening / boundary-enforcement / anti-leak** — strengthen an existing system against drift, an engine/game/data boundary breach, a hidden-information leak, or weak proof.
- **foundational / doc-overhaul** — overhaul a foundation doc (or the cascade from an upstream doc change).
- **other** — anything else; build the read set from exploration alone.

Announce the classification on its own line as your **first user-facing output**, before any exploration tool call — emit `Classification: <type>` so the audit trail records it independent of your reasoning. When ambiguous, give a one-sentence justification.

## Step 2: Explore the repo to ground the brief

The point of authoring here is that Claude can read the repo directly — so the *user never types out what the researcher should read*. Build, from exploration:

- the **authority-ordered read list** following `docs/README.md`'s ordered index (`FOUNDATIONS.md` → `ARCHITECTURE.md` → `ENGINE-GAME-DATA-BOUNDARY.md` → the relevant area docs → `ROADMAP.md`), then relevant `specs/`, `tickets/`, `templates/`, `docs/adr/`, `docs/plans/`, `reports/`, `archive/`, each with a one-line reason it is load-bearing for *this* target;
- the **relevant code seams** Session 2 should inspect (name files/modules in `crates/`, `games/`, `tools/`, `apps/web/` — don't paste them; Session 2 reads them itself);
- any **prior report / spec / ADR / archived work** that already bears on the target, so the brief frames the task as a delta rather than a cold start.

When the target concerns presentation, UX, or observable runtime behavior, ground it by **exercising the running artifact** — drive the live app and capture the actual rendered state (screenshots, verbatim on-screen strings) — and fold those concrete captures into the brief's context; source and docs alone under-ground a behavior-facing brief, and the live capture is also how you catch exploration findings that the current build has already falsified.

Launch Explore agents for broad surveys, partitioning the fan-out by authority layer / subsystem (foundation + area docs; touched code seams in `crates/`/`games/`/`tools/`/`apps/web/`; planning artifacts in `specs/`/`tickets/`/`reports/`; the worked failing case if there is one) so each agent has a non-overlapping beat. Instruct each agent to return **concise, path-and-symbol-focused findings** — file paths with one-line load-bearing reasons plus the relevant type/field/component names, never whole-file dumps or long prose — so a wide parallel fan stays within the per-turn tool-result budget and no later agent's results get silently dropped or deferred. Pass each agent a concrete bound, not just the adjective "concise": cap it at a fixed digest skeleton (path → one-line reason → relevant symbols), roughly ≤150 lines, with no narrative synthesis or summary tables. Sub-agents reliably over-produce against a qualitative instruction; a structural cap is the lever that actually holds. Verify any repo claim in `research_target` or `reference_path` against the actual tree; flag contradictions prominently. Apply the same skepticism to your **own exploration's conclusions**: before a subagent finding about current behavior or UI shapes the brief's framing (delta vs. cold start), cross-check it against ground truth — recent git history (`git log` for the touched area) and, where a runnable artifact exists, the running app — because Explore agents reason from static reads and miss recently-landed wiring.

## Step 3: Light online research (optional)

Only to **sharpen scope and interview questions** — surface the named techniques, prior art, or decision axes the interview should resolve. The *deep* research is Session 2's job; do not do it here. **If skipped, you MUST still emit the one-liner** in-session (e.g., "Online research: skipped — repo-internal realignment") — treat it with the same discipline and prominence as the Step 1 `Classification:` line, so the audit trail is visible independent of the brief body. Emit it at a fixed point — immediately before you open the Step 4 interview (i.e., before the first `AskUserQuestion` or clarifying question), regardless of how many exploration waves precede it — so it has a structural anchor as durable as the `Classification:` line (which is pinned to "first user-facing output") and cannot silently slip just because the determination was repo-internal or the exploration ran in several passes. (Anchor to the *interview opening*, not to "exploration returns": Step 2 is often iterative — direct reads, greps, then `Explore` agents — so there is no single moment exploration "returns.")

## Step 4: Interview to 95% confidence

Reach **95% confidence about what the user actually wants** — not what they think they should want — before drafting. Display this block after each answer (or, when a batch is sent, once before the batch and once after it):

```
Confidence: X%
Gaps: [specific remaining unknowns]
```

The Confidence figure tracks **user-intent resolution only**. Gaps you deliberately delegate to Session 2's design scope (UX/interaction details, final naming, anything the user explicitly said is "yours/Session 2's call") do **not** count against the 95% threshold — list them under `Gaps` tagged `(delegated to Session 2)` so they read as out-of-scope-for-the-user, not unresolved intent. The "95% — drafting the brief" announcement fires once user intent is locked, even when such delegated design scope remains open.

Rules: ask one *conceptual* question at a time when probing motivation or uncertainty sequentially, where each answer reshapes the next; but batch independent, already-scoped bounded choices into a single `AskUserQuestion` call (≤4 questions). Prefer bounded multiple-choice (`AskUserQuestion` when available). Probe motivation before solution; challenge premature specificity; name uncertainty specifically; respect demonstrated expertise and "you decide" delegation (re-evaluate and recommend, don't re-ask). Confidence rises from both answers and exploration findings; note which gaps each closes. On receiving batched answers, re-display the `Confidence / Gaps` after-block before proceeding — unless confidence reaches threshold, in which case the "95% — drafting the brief" announcement subsumes it. Announce "95% — drafting the brief" when reached.

You MAY dispatch the Step 2 exploration agents and the Step 4 interview batch in the *same* response — overlapping the two phases to save a turn — but only when the batched questions are already grounded by your own direct reads and do **not** depend on the still-pending exploration results. When you do overlap them, the pre-batch `Confidence / Gaps` block still fires (anchored to findings-so-far): the before/after-batch display is not waived just because the batch was launched alongside exploration. If a question's answer genuinely needs the exploration results, hold the interview until those agents return.

**Determination-as-target**: when the research target is itself a decision ("what should we build / fix / spec next"), the interview MUST resolve and *lock* that decision here in Session 1. The brief then instructs Session 2 to **confirm-and-document** the chosen direction — citing the evidence that fixed it (e.g. the lowest non-`Done` gate, an empty promotion-debt register, satisfied predecessor preconditions) — never to re-open the determination open-endedly. A brief that leaves Session 2 to re-decide "what's next" violates the locked / no-questions contract (Guardrails). This is distinct from a **bounded delegation**: the brief MAY ask Session 2 to *recommend among enumerated options with a required default and justification* for a scoped design sub-choice the user deliberately delegated (e.g. "you pick the delivery mechanism from these two"). That is not re-determination and does not breach the locked contract — the determination and scope stay locked; only a named, optioned sub-decision is handed down, and Session 2 still produces directly rather than asking the user.

**Early exit**: if the user says "just go," announce current confidence, list remaining gaps, and carry them into the brief as labeled assumptions (`assumption: X`) so Session 2 — which will not ask — treats them as decisions the user can later correct.

## Step 5: Present the brief outline (HARD-GATE)

Before writing, present in chat:

1. the **settled intentions** — the resolved decisions the interview produced (these become §3 of the brief and are what make Session 2 "locked");
2. the **deliverable spec** — exactly which downloadable markdown docs Session 2 must produce (replace vs. new, filenames);
3. the **read-in-full list** (authority-ordered, with the one-line reasons). When the read set is large — e.g. a near-whole-doc-set overhaul spanning many docs plus templates — the gate may present the list authority-ordered with reasons summarized by tier, or deferred to the brief, rather than one reason per file; the brief's §2 must still carry a one-line reason for every file regardless.

If a minor residual ambiguity persists at this gate without an early exit (e.g. the user's wording left a detail genuinely open), surface it here as a labeled `assumption: <X>` for the user to confirm or override, rather than re-interviewing — then carry it into the brief's §3 the same way an early-exit assumption is carried.

Get approval (per the HARD-GATE). Revise on pushback before writing.

## Step 6: Write the brief and refresh the manifest

On approval, do BOTH:

1. **Write the brief** to `reports/<topic>-research-brief.md`, following the canonical anatomy in `references/brief-template.md`. `<topic>` is a short kebab-case slug of the target.
2. **Refresh the manifest**: write the current repository path inventory to `reports/manifest_<today>_<shortsha>.txt`, where `<today>` is the real current date (`date +%F`), `<shortsha>` is the fetch-baseline commit's short hash (`git rev-parse --short HEAD`), and the inventory is the exact fetch-baseline commit's tree — `git ls-tree -r --name-only HEAD` (use the same `<baseline>` you pin in the brief's §1, not `git ls-files`, so the manifest provably equals the commit Session 2 fetches from). Encoding the baseline SHA in the filename keeps each brief's manifest pinned: a second brief authored the **same day at a different commit** gets its own file instead of overwriting an earlier brief's manifest — overwriting would silently break that earlier brief's §1 baseline↔manifest correspondence (its §1 still cites the old commit while the file now lists a new tree). Reference the manifest by its exact `manifest_<today>_<shortsha>.txt` filename in the brief's §1 so Session 2 uploads the right file. Leave older manifests in place for the user to clean; only when a manifest with the identical `<today>_<shortsha>` name already exists (a same-day re-run at the *same* commit) do you regenerate it rather than trusting the stale one.

**Baseline-commit rule.** The brief instructs Session 2 to fetch every file from one exact commit, so the manifest must list exactly that commit's tree. Derive the fetch-baseline commit from verified repo HEAD (`git rev-parse HEAD`) at manifest-refresh time, and generate the manifest from that same commit (`git ls-tree -r --name-only HEAD`) — do NOT use `git ls-files`, which reflects the staged index and silently diverges from HEAD under any uncommitted add/delete/rename. If you do fall back to `git ls-files`, first confirm `git status --porcelain` is clean (or reconcile every listed delta) and note the check in the Step 7 summary, since otherwise the manifest and the §1 fetch baseline will not agree. NEVER adopt a commit string copied from a report, doc, or `research_target` without confirming it contains every file in the §2 read-in-full list (`git ls-tree <commit> <path>` / `git cat-file -e <commit>:<path>`) — a "commit of record" cited inside a report is that report's *own* baseline and often predates later merges. If a referenced source cites a different commit, call out the divergence inside the brief rather than propagating it. Independently of how the baseline was derived (verified HEAD or a cited string), confirm before writing that every §2 read-in-full path resolves at the chosen baseline (`git cat-file -e <baseline>:<path>`); a hand-assembled read-list can name a moved, renamed, or deleted path even at a correct HEAD, and catching it here is cheaper than letting Session 2's self-check surface it after the brief ships.

Resolve both paths against the worktree root if in a worktree. Do NOT commit.

## Step 7: Summarize

Report:

- the two written files (brief + refreshed manifest) — the **upload bundle** for ChatGPT-Pro Session 2;
- a one-line reminder that Session 2 is **locked / no-questions**: paste the brief, upload the manifest, and ChatGPT-Pro should produce the deliverable directly;
- any labeled assumptions carried from an early exit (or surfaced at the approval gate), so the user can correct them before pasting.

This is an inline-completion deliverable — no next-steps menu. Surface any adjacent improvement spotted during exploration as a flagged note with a concrete trigger, not as scope creep.

## Guardrails

- **Self-containment is the contract.** Session 2 has none of this session's context. Every path, decision, constraint, and acceptance check it needs lives in the brief or the uploaded manifest — never implied.
- **Claude authors; ChatGPT-Pro researches.** Don't perform the deep research here. The brief *commissions* it.
- **Locked, no questions.** The emitted brief instructs Session 2 to produce directly and NOT interview or ask clarifying questions — the interview already happened here.
- **Mutates only `reports/`.** Never touch `docs/`, `specs/`, `tickets/`, `templates/`, `.claude/skills/`, or source.
- **No scope inflation.** The brief commissions what was asked. Resist "while we're at it" additions to the deliverable spec.

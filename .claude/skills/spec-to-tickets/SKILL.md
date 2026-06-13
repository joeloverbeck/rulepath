---
name: spec-to-tickets
description: "Use when decomposing a Rulepath roadmap-gate spec into actionable implementation tickets aligned with docs/FOUNDATIONS.md. Reads the spec, validates its assumptions against the codebase, then writes one ticket per reviewable diff to tickets/<PREFIX>-NNN.md. Produces: ticket files. Mutates: only tickets/ (never specs/, docs/, or .claude/skills/)."
user-invocable: true
arguments:
  - name: spec_path
    description: "Path to the spec file (e.g., specs/gate-0-repository-skeleton.md)"
    required: true
  - name: namespace
    description: "Ticket namespace prefix, used as <PREFIX>-<NNN>.md (e.g., GATE0SKEL). If omitted, the skill derives one from the gate number and slug and asks the user to confirm."
    required: false
---

# Spec to Tickets

Break a Rulepath roadmap-gate spec into small, actionable implementation tickets a reviewer can merge one at a time, each validated against the current codebase and aligned with `docs/FOUNDATIONS.md`.

<HARD-GATE>
Do NOT Write any ticket file at `tickets/<PREFIX>-<NNN>.md` until ALL of the following hold:

(a) Pre-flight has verified `docs/FOUNDATIONS.md`, `tickets/_TEMPLATE.md`, `tickets/README.md`, and `<spec_path>` are all readable; if any is missing the skill aborts before Step 1.

(b) Step 2 (codebase validation) has completed, and every surfaced Issue has an explicit user disposition — one of: fix-before-decomposition, defer-to-follow-up-ticket (named dependency), reject-with-rationale (route back to `/reassess-spec`), expand-scope-in-place (decompose against the wider surface the codebase requires; the spec text is not edited), or drop-as-moot (named target doesn't exist AND intent is covered by sibling deliverables or is a structural no-op).

(c) Step 4 has emitted the decomposition summary table in chat (numbered tickets with Title, Scope, Effort, Deps, FND, Notes) AND the user has explicitly approved it, OR the auto-mode carve-out has fired (auto mode active AND Step 2 surfaced no Issues AND no `/reassess-spec` findings were deferred by the user).

(d) Every `Deps` reference resolves to a ticket produced in this run, or to a pre-existing `tickets/` / `specs/` path verified at Pre-flight or at Step 4's cross-spec Deps check before approval.

This gate is authoritative under auto mode and any autonomous-execution context. Invoking the skill does not constitute approval of the decomposition.
</HARD-GATE>

## Process Flow

```
Pre-flight: verify required files readable; derive + confirm <namespace> if omitted
       |
       v
Step 1: mandatory reads (spec, tickets/_TEMPLATE.md, tickets/README.md, docs/FOUNDATIONS.md)
       |
       v
Step 2: codebase validation (load references/codebase-validation.md); surface Issues; await per-Issue disposition
       |
       v
Step 3: decompose the spec (load references/decomposition-patterns.md)
       |
       v
Step 4: present decomposition summary table; await user approval
       |
       +-- [HARD-GATE fires here]
       |
       v
Step 5: batched ticket writes (one or a few messages, parallel Write calls, one per ticket)
       |
       v
Step 6: final summary (cross-ticket Deps check, deliverable coverage, dependency graph, suggested order). Do NOT commit.
```

## Inputs / Output

**Input**: `spec_path` (required); `namespace` (optional, derived + confirmed if omitted). Plan-mode and worktree-root resolution are auto-detected.

**Output**:
- **Ticket files at `tickets/<PREFIX>-<NNN>.md`** — one per reviewable diff, each following `tickets/_TEMPLATE.md` exactly.
- **Decomposition summary table** — emitted in chat at Step 4 before any Write.
- **Final summary** — emitted at Step 6 (cross-ticket Deps verification, deliverable coverage mapping, dependency graph, suggested implementation order).

This skill emits markdown tickets only. It operates at pipeline scope: it produces tickets that feed implementation, so FOUNDATIONS alignment applies even though it writes no game content or engine code itself.

## Prerequisites

Before acting, this skill MUST read:

- `<spec_path>` — the target spec, entire contents (Step 1).
- `tickets/_TEMPLATE.md` — the canonical ticket structure; every ticket must follow it exactly (Step 1).
- `tickets/README.md` — the ticket authoring contract (Step 1).
- `docs/FOUNDATIONS.md` — the non-negotiable design contract. Step 1's in-context skip allowance applies to all four mandatory reads.
- Every file path, crate/module, skill directory, type, schema field, and spec reference extracted from the spec — read on demand at Step 2.

Reading scope: anything under `specs/`, `.claude/skills/`, `docs/`, `templates/`, `tickets/`, and the code tree (`crates/`, `games/`, `apps/`, `tools/`). This skill does not author game content (rules, traces, engine code) — it reasons about specs that plan app behavior.

## Reference Files

- **Step 2** — `references/codebase-validation.md`
- **Step 3** — `references/decomposition-patterns.md`

Load each before the corresponding step. Loading both right after Step 1 is the simplest path; on-demand is also fine.

## Worktree & Plan-Mode Awareness

Inside a git worktree, ALL paths (reads, writes, globs, greps) resolve from the worktree root. If plan mode is active, present the decomposition in the plan file and call `ExitPlanMode` in lieu of the Step 4 chat-table approval; write tickets only after approval.

## Pre-flight Check

Before Step 1, verify:
1. `docs/FOUNDATIONS.md` exists and is readable.
2. `tickets/_TEMPLATE.md` exists and is readable.
3. `tickets/README.md` exists and is readable.
4. `<spec_path>` exists and is readable. If it is a glob (e.g. `specs/gate-0*`), resolve first: exactly one match → use it (note the resolution); zero or many → abort or ask to disambiguate.
5. `<namespace>` is provided, OR derive one from the gate number and abbreviated slug (e.g. `specs/gate-0-repository-skeleton.md` → `GATE0SKEL`) and ask the user to confirm or override before Step 1.

If any of checks 1–4 fails, abort with a clear missing-file error. If check 5's gate parsing is ambiguous, ask the user for the namespace directly.

## Step 1: Mandatory Reads

Read ALL of: the spec file (entire), `tickets/_TEMPLATE.md`, `tickets/README.md`, and `docs/FOUNDATIONS.md` — skipping any of these whose full content is already current in this session's context (read via the Read tool, or authored via Write this session) and unmodified since.

**House-style calibration (optional)**: when prior tickets already exist under `tickets/` or `archive/tickets/`, skim one as a depth/voice exemplar — the citation density, Verification-Layers granularity, and FOUNDATIONS §-referencing the repo expects. Structure is still governed by `tickets/_TEMPLATE.md`; the exemplar only calibrates detail level.

Parse the spec's metadata (Spec ID, Roadmap stage, Roadmap build gate, Status, authority order, Sequencing predecessor/successor gate) and its sections (Objective, Scope [in scope / out of scope / not allowed], Deliverables, Work breakdown, Exit criteria, Acceptance evidence, FOUNDATIONS & boundary alignment, Forbidden changes, Documentation updates required, Sequencing, Assumptions). The canonical spec format is documented in `specs/README.md` and exemplified by `specs/gate-0-repository-skeleton.md`.

**Non-standard deliverables**: if the spec uses named sections or a numbered `§Scope` / `§In scope` list instead of a `Work breakdown` table, treat each distinct implementation section (or in-scope item) as a deliverable for decomposition.

## Step 2: Codebase Validation

**Load `references/codebase-validation.md`.** Validate the spec's assumptions against the current codebase, surface Issues, and obtain a per-Issue disposition before Step 3. A spec that was reassessed via `/reassess-spec` earlier this session with all findings resolved qualifies for the abbreviated spot-check path documented in the reference.

## Step 3: Decompose the Spec

**Load `references/decomposition-patterns.md`.** Identify discrete work units — each a reviewable diff — map dependencies into each ticket's `Deps`, order by dependency graph and criticality, and ensure every spec deliverable is covered (no silent skipping). The reference documents the deliverable-coverage categories, the merge/split rules, and the recurring ticket-shape patterns (capstone integration ticket, cross-cutting docs ticket).

**Number tickets in dependency (topological) order where practical** — lower numbers land first. When a `Deps` must point at a higher-numbered ticket (e.g. a tool ticket consuming a doc authored by a later docs ticket), that backward-in-number dependency is valid, but call it out explicitly in the Step 6 suggested-implementation-order line so a reviewer reading in numeric order isn't surprised.

## Step 4: Present Summary for Approval

Before writing any ticket files, present a numbered summary table:

| # | Ticket ID | Title | Scope | Effort | Deps | FND | Notes |
|---|-----------|-------|-------|--------|------|-----|-------|
| 1 | <NS>-001  | …     | <5-10 word scope> | Small  | None | — | — |
| 2 | <NS>-002  | …     | <5-10 word scope> | Medium | 001  | §11 | shared file set |

Column roles: **Title** matches the ticket's first line; **Scope** is the deliverable mapping (`D1+D6`) or acceptance surface — must not duplicate the Title; **Effort** Small/Medium/Large; **Deps** other tickets in this batch or pre-existing tickets/specs (state once if all independent); **FND** a FOUNDATIONS section only when notable (e.g. §11 acceptance invariants, §12 stop conditions, §3 boundary), `—` otherwise; **Notes** merged/split deliverables, shared files, multi-dependency validation tickets.

**Cross-spec Deps verification (before HARD-GATE fires)**: run `test -f` (or equivalent) on every cross-spec `Deps` path introduced during Step 3 that was not verified at Pre-flight (typically `specs/<sibling>.md` or `tickets/<PREFIX>-NNN.md` from a prior batch). Abort with a missing-Deps error if any fails. Cite the result alongside the table (e.g. `Cross-spec Deps verification: N/A — all Deps resolve to tickets produced in this run`).

**Intra-batch create-then-modify Deps pre-check (before presenting the table)**: for every planned `(new)` file, confirm that each sibling ticket whose Files-to-Touch will `(modify)` that file lists the creator ticket in its `Deps` — and that any ticket consuming a Rust op/symbol another ticket introduces depends on that producer. Bake these structural Deps into the table BEFORE approval, so the approved Deps already reflect file-creation and producer/consumer ordering. The same create-then-modify rule is enforced again at Step 5's pre-write existence check, but surfacing it here prevents an approved-but-incomplete Deps set that composition then has to expand (see `references/decomposition-patterns.md` §Intra-batch create-then-modify chains).

**Contingent ticket count from a Step-2 Issue disposition or spec-level conditionality**: when a Step-2 Issue's disposition — or a conditionality declared in the spec's own Assumptions / a §Conditional / decision-gated ticket (see `references/decomposition-patterns.md`), e.g. a `game-stdlib` extraction ticket added only if a primitive-pressure ledger decides *promote* — would add or remove a ticket (e.g. an ADR-decision ticket added only under one option), present the count-affecting delta inline in the table (e.g. `21 tickets, +1 if Issue I1 → option (a)`) and resolve that disposition together with table approval — HARD-GATE (b) before (c) — so the approved count is unambiguous before any write. This applies **even when the gating decision is expected to resolve the no-extra-ticket way**: a §4 third-use hard gate whose ledger is *expected* to decide defer/reject (no `game-stdlib` extraction) still warrants the one-line delta (e.g. `19 tickets, +1 if the resource-accounting ledger → promote: conditional game-stdlib extraction`), so the contingency is surfaced rather than silently assumed away by the expected outcome.

**Spec-declared implementation latitude (scope-shaping, not count-affecting)**: when the spec's own Assumptions / §Conditional explicitly leaves an implementation choice open (a "MAY do X or Y") that reshapes a ticket's Files-to-Touch / Engine-Changes / `Deps` but adds or removes **no** ticket — e.g. realizing per-game theme via a CSS/TS presentation path versus a Rust→WASM projection — surface it inline at the table with a recommendation and resolve it together with table approval (like the contingent-count delta, but the count is unchanged). This is distinct from a §2 behavior-authority risk: when the spec sanctions a §2-clean presentation realization that legitimately avoids a projection hop, choosing it is a decomposition decision, not a risk (see `references/codebase-validation.md` §"Data-availability / projection").

**Wait for user approval or adjustments.** Do not write files until the user confirms. **Auto-mode / no-stopping carve-out**: when auto mode (or an in-session "work without stopping" directive) is active AND Step 2 surfaced no Issues AND no `/reassess-spec` findings were deferred, auto-approve and proceed; announce it inline and cite the directive. When the carve-out fires, do not end the turn after the table — emit the rehearsal and continue straight into the first write batch in the same agentic continuation; a text-only stop re-introduces the wait the carve-out just waived. Any open Issue or deferred finding holds the wait-gate per HARD-GATE clause (c). When every Issue carries an explicit recommended disposition under a no-stopping directive, the operator MAY proceed by applying the named dispositions, citing each before the writes; the user can redirect.

## Step 5: Batched Ticket Writes

**Post-approval refinement (mechanical only)**: while composing, you MAY apply a *mechanical tightening* to the approved Step-4 table without re-approval — specifically, removing a `Deps` entry that composition shows is unnecessary, adding a structurally-required `Deps` entry that composition reveals (a create-then-modify dependency on the file's creator, or a producer/consumer dependency on a ticket that introduces a Rust op/symbol this one uses), relocating a sub-feature between already-approved sibling tickets of the same deliverable, or completing a registration/consumer surface the approved deliverable already names — e.g. a tool match-arm the spec's tool-list enumerated, or a CI step — that the Step-4 table Notes under-listed (add a `(modify)` target only; see `references/codebase-validation.md` §"Enum / consumer blast radius", which flags consumer-set under-enumeration as the most common way a deliverable understates its true Files to Touch) — provided it adds no ticket, creates no new file, crosses no deliverable boundary, and does not change the ticket count. Disclose every such tightening in the Step 6 summary (original table entry → applied change). Anything beyond that — adding/removing a ticket, changing the count, moving work across deliverables, or introducing a new file/deliverable — must round-trip to the user for re-approval per HARD-GATE clause (c).

**Pre-write rehearsal (mandatory)**: in the turn immediately before the writes, state the exact number of Write calls the next response will contain.

| Ticket bodies | Cold session | Warm session (ceiling already proven) |
|---|---|---|
| ~80–150 lines (typical gate ticket) | ≤3 / batch; self-check post-batch | up to proven ceiling; self-check post-batch |
| ~150+ lines, or after a divergence | 1 / turn; self-check post-write | 1 / turn |

The table routes you to the applicable branch; the prose below is the authority on every edge case. **Default: ≤3 parallel Writes per batch** unless a larger parallel batch has already succeeded this session (cite the tested ceiling when committing to more). **When ticket bodies are very long (~150+ lines each), rehearse 1 from the start** — reliably emitting several very-long Writes in one message is precisely the uncertain-emission case described at the end of this paragraph, and a steady one-per-turn cadence avoids the divergence ceremony. For **moderately long bodies (~80–150 lines — the typical official-game-gate ticket), starting at the `≤3` default on a cold session is acceptable**: a single clean 3-write batch then licenses continuing at 3 without first proving 1 (in one official-game gate run this way, 100–130-line tickets emitted at 3-per-batch across six batches with zero divergences). **But tickets carrying the full `_TEMPLATE.md` section set plus a 4–5-item Assumption Reassessment and a Verification Layers block — the typical capstone / high-complexity official-game-gate ticket — reliably run ~130–180 lines, the upper end of (or past) the moderate band; when a gate's tickets trend past ~130 lines, treat them as the rehearse-1 case and rehearse 1 from the start rather than starting at `≤3` and risking the divergence ceremony.** **This section-density trigger is primary and line-count-independent**: a ticket carrying the full `_TEMPLATE.md` section set + a 4–5-item Assumption Reassessment + a Verification Layers block defaults to rehearse-1 even when tighter prose lands it under ~130 lines, and applies equally to a non-game infrastructure spec as to an official-game gate (a 10-ticket infra decomposition of ~110–130-line full-template tickets diverged on batch 1 when started at `≤3`). Reserve forced batch-1 for the ~150+-line case or after an actual divergence (per §Divergence stabilization), and a larger-than-3 batch only for a warm session with a demonstrated ceiling. The cap can only ratchet up by demonstrated success (3 → then a larger batch → …), so on a cold session a large decomposition realistically takes ⌈N/3⌉ batches the first time; fewer batches is a warm-session optimization, not a cold-session target. You MAY state the full batch plan once upfront (e.g. `5 batches: [001,002,003], [004,005,006], …`); each subsequent write turn then needs only a one-line restatement of the current batch, not a fresh standalone rehearsal turn. The one-line restatement MAY share the turn with that batch's Write calls — a restatement sentence plus the N parallel Writes in a single turn is compliant; what is forbidden is emitting more than N Writes in the turn, or splitting the N across multiple turns. If the emitted count diverges from the rehearsal, the next turn is a zero-Write acknowledgment that restates the remaining count and re-batches at ≤ the last successfully-emitted count — do not emit a single catch-up Write "to keep momentum." **Divergence stabilization**: after two or more divergences in a run, stop attempting ratchet-ups and hold the batch size at 1 for the remaining tickets — each failed ratchet-up costs another zero-Write turn, so a reliable one-Write-per-turn cadence finishes faster than chasing a larger batch. More generally, rehearse the number you are confident you will emit (1 is always safe): throughput is a warm-session optimization, never a cold-session target, so when emission reliability is uncertain, rehearse 1.

**Pre-write existence checks** (same rehearsal turn): for every `(modify)` Files-to-Touch entry across the composed tickets, run `test -f` against the working tree; correct or reclassify any path that doesn't resolve. A `(modify)` entry pointing to a file another ticket creates `(new)` in this batch is valid only when the modifying ticket declares `Deps:` on the creator (per `references/decomposition-patterns.md` §Intra-batch create-then-modify chains). For every command in a ticket's Test Plan, confirm it resolves against the repo. Enumerate `(modify)` entries individually, not as a collapsed "all new" claim. **Discovered-set / wildcard modify-targets**: when the exact set of files a ticket will modify is genuinely unknowable until implementation (e.g. "accessibility fixes across whatever components the smoke surfaces"), do NOT write a bare glob like `components/*.tsx` that `test -f` cannot validate — either name the candidate files explicitly with an "as surfaced" qualifier, or mark the entry as an implementation-discovered set and `test -f` its parent directory instead, so the existence check stays honest about what is verified versus deferred. Also run a **section-presence self-check** on every composed ticket before emitting the writes — assert each `_TEMPLATE.md` `## ` header (Problem, Assumption Reassessment, Architecture Check, Verification Layers, What to Change, Files to Touch, Out of Scope, Acceptance Criteria, Test Plan) **and each bold metadata field (Status, Priority, Effort, Engine Changes, Deps)** is present in the draft, and assert each ticket's Assumption-Reassessment list is numbered sequentially from 1 with no gaps (`awk '/^## Assumption Reassessment/,/^## Architecture Check/' <draft> | grep -oE '^[0-9]+'` strictly increasing — the same numbering check as Step 6 item 2, run pre-write to catch a skipped menu-item renumber like `1, 2, 3, 6`); a missing section/field or a numbering gap is cheaper to fix pre-write than to catch at Step 6. When the self-check runs post-batch or post-write (the middle and single-Write cadences) and surfaces a gap — e.g. an Assumption-Reassessment list emitted as `1, 2, 3, 6` because a selected menu item kept its template number instead of being renumbered — fix it with an immediate Edit to the offending file before composing or writing the next batch, not deferred to Step 6.

**Flow**: compose every ticket's full content first, then emit the Write calls in as few batched messages as the current tested ceiling allows (parallel Write calls, one per ticket). The batch *count* is governed by the ≤3-per-batch cap above, not a fixed target: a cold session writing 10+ tickets takes ⌈N/3⌉ batches; once a larger batch has succeeded, the remaining tickets may go in correspondingly fewer batches. Do NOT alternate compose → Write per ticket — *except* once the batch size is legitimately 1 (post-divergence-stabilization, or a deliberately conservative cold-session rehearsal per the Step 5 rehearsal guidance), where composing each ticket immediately before its single Write is the correct cadence, not avoidable thrash. **Large cold-session exception**: when the ⌈N/3⌉ cap makes a single compose-all-then-emit turn impractical (e.g. a 20+-ticket gate; ~15+ long-bodied tickets — official-game or otherwise — also qualifies; the `20+` is illustrative, not a hard floor), composing each batch's tickets immediately before that batch's writes is acceptable — provided a single upfront `test -f` existence sweep covers every planned `(modify)` target from the approved Step-4 plan, and the Step 6 section/metadata grep loop is run (it MAY be run pre-write to discharge the §Pre-write-existence-checks section-presence self-check — they are the same mechanism). This is batched compose-then-write, still never per-ticket alternation. For a long-ticket gate this **composes with** the §Pre-write rehearsal cadence: the exception governs *when* you compose — each batch's tickets immediately before that batch's writes, after the upfront sweep — independent of batch size. Under the forced rehearse-1 rule (very-long ~150+-line bodies, or post-divergence) that is one upfront sweep + N single-Write turns; for moderate (~80–150-line) bodies started at the `≤3` default it is one upfront sweep + ⌈N/3⌉ compose-then-write batches — not N separate rehearsal-then-write turn pairs either way. **Self-check timing in single-Write-per-turn cadence**: the §Pre-write-existence-checks section/metadata/numbering self-check is a *file-based* `awk`/`grep` (it targets `<draft>` as a file), so it can only run on tickets already on disk — it cannot inspect the in-memory draft you are about to Write this turn. In the single-Write cadence, therefore, run that grep loop as an **immediate post-write check on each file before composing the next ticket** (catch-and-fix within the same or next turn), rather than treating it as a true pre-write gate — most efficiently by folding the prior file's self-check `grep`/`awk` Bash call into the same turn as the current file's Write (self-check ticket *N−1* while emitting the Write for ticket *N*), which keeps the single-Write cadence at N turns rather than 2N. The pre-write file-grep is genuine only in the batched compose-all-first cadence, where every draft is already written before the loop runs. **In a middle cadence — batches of N>1 but fewer than all (e.g. compose 3, write 3, then grep those 3 before composing the next batch) — run the self-check post-batch on the just-written files**, the same post-write timing as the single-Write cadence extended to the batch. Either way the loop must run before Step 6, never be deferred to it.

For each approved ticket, compose its full content following `tickets/_TEMPLATE.md` exactly — every required section present (Status, Priority, Effort, Engine Changes, Deps, Problem, Assumption Reassessment, Architecture Check, Verification Layers, What to Change, Files to Touch, Out of Scope, Acceptance Criteria, Test Plan). **Presentation-only tickets**: for a ticket whose changes are confined to the TypeScript/React shell (`apps/web`), fill `Engine Changes` as `Yes (presentation-only) — <web files>` when the ticket ships web **code** (the web files are the touched code surface per `_TEMPLATE.md`'s "or code surfaces"), and reserve `None` for a **docs/status-only** ticket (e.g. a closeout capstone that edits only docs, the `specs/README.md` index, and status markers). Because every `apps/web` ticket is Rust-free, "no Rust/engine surface" must NOT collapse a code-bearing web ticket to `None`. Never phrase it so as to imply behavior authority moved to TypeScript (FOUNDATIONS §2). For the **Assumption Reassessment** menu: items 1–3 are always required; for items 4+ **Select** the menu items matching this ticket's scope, **Rewrite** each selected item's number to its position in the surviving list (starting at 4), and **Verify** the final list reads `1, 2, 3, 4, …` with no gaps (a list like `1, 2, 3, 6` means the rewrite step was skipped). **Substrate-only tickets**: when a ticket builds the *inputs* to a FOUNDATIONS enforcement surface a later gate will implement (e.g. a schema/contract that feeds future fail-closed validation, deterministic replay/hash, or the no-leak visibility firewall — but no validator exists yet), item 5 still applies: satisfy it by naming the deferred enforcement surface and confirming the data-model change introduces no leakage or nondeterminism path the later surface would have to undo, citing the gate that will enforce it. Every ticket modifying existing behavior must cite the change rationale in Assumption Reassessment (no silent retcon — `tickets/README.md` change-rationale requirement; durable change is gated by §13 ADR triggers).

After the batch returns, verify every ticket file exists; retry any failed Write before Step 6. If a system-reminder shows a ticket was externally edited (e.g. a linter hook), treat the edit as authoritative and re-verify sibling references against the edited content before the final summary.

## Step 6: Final Summary

After writing all files:

1. **Cross-ticket dependency consistency**: for each `Deps`, confirm the depended-on ticket actually produces what the dependent needs; `test -f` every `Deps` path at emission time. If a `(modify)` Files-to-Touch entry names a file a sibling creates `(new)` in this batch without a declared `Deps` on the creator, flag it — **unless an unbroken predecessor `Deps` chain transitively reaches the creator** (a sequential pipeline where the creator stubs the files and each ticket `Deps` its immediate predecessor; see `references/decomposition-patterns.md` §Intra-batch create-then-modify chains), in which case the `(modify; created by NNN)` annotation is the audit trail and no direct `Deps` on the creator is required.
2. **Template fidelity**: confirm every required section is present and that each ticket's Assumption Reassessment uses gapless sequential numbering starting at 1. Section-presence check (run per ticket): `for s in "## Problem" "## Assumption Reassessment" "## Architecture Check" "## Verification Layers" "## What to Change" "## Files to Touch" "## Out of Scope" "## Acceptance Criteria" "## Test Plan"; do grep -qF "$s" tickets/<PREFIX>-NNN.md || echo "MISSING $s in <PREFIX>-NNN"; done` — must print nothing. Metadata-field check (run per ticket): `for h in '**Status**' '**Priority**' '**Effort**' '**Engine Changes**' '**Deps**'; do grep -qF "$h" tickets/<PREFIX>-NNN.md || echo "MISSING $h in <PREFIX>-NNN"; done` — must also print nothing (catches a ticket missing a required `_TEMPLATE.md` metadata header field, which the `## `-only section check cannot). Numbering check: `awk '/^## Assumption Reassessment/,/^## Architecture Check/' tickets/<PREFIX>-NNN.md | grep -oE '^[0-9]+'` should be strictly sequential. Note both greps are **presence-only and order-insensitive**: they pass a ticket whose metadata fields are out of `_TEMPLATE.md` order (e.g. `Engine Changes` before `Effort`) and a ticket carrying a stray duplicate subsection (e.g. a second `### Test Plan` nested under Acceptance Criteria — the `## `-anchored section grep matches the real `## Test Plan` and never sees the stray `### `). Since "uses `_TEMPLATE.md` exactly" implies field order and no stray sections, eyeball metadata-field order and scan for duplicate `##`/`###` headers per ticket in addition to the greps. Duplicate-header check (run per ticket): `grep -E '^#{2,3} ' tickets/<PREFIX>-NNN.md | sort | uniq -d` — must print nothing (it compares full header lines, so it catches a stray second `### Test Plan` without false-positiving; do NOT reduce headers to the bare `## `/`### ` marker before `uniq`, which flags every ticket). **Test-Plan fallback contradiction** (eyeball, greps can't catch it): when a ticket modifies or adds a test, confirm its Test Plan does not *also* carry the `_TEMPLATE.md` "None — documentation-only ticket" fallback line — a real-test entry and the None fallback are mutually exclusive, but the presence-only section grep passes a ticket carrying both (both sit under `## Test Plan`). Also confirm each applicable conditional menu item is present — a FOUNDATIONS principle / §11 acceptance invariant motivated → item 4; a third-use mechanic hard gate (§4) / fail-closed-validation / no-leak visibility firewall / deterministic replay-hash & serialization surface touched (including substrate that feeds a deferred enforcement surface — see Step 5) → item 5; an existing schema or contract extended → item 6; a public symbol / mechanic / acceptance invariant / doc-governed contract / schema field renamed or removed → item 7.
3. **Deliverable coverage mapping**: list each spec deliverable and the ticket(s) covering it (`D1→001`, `D3→003+004` for a split), including the exempt categories from `references/decomposition-patterns.md`. When the spec identifies deliverables by section/area rather than `D`-numbers (the common Rulepath case — e.g. `§5.1`, `§15.6`), key the map by the spec's own identifiers (`§5.1 actions→003`, `§15.6 WASM smoke→009`); the `D<n>` form is illustrative, not required. Flag any uncovered deliverable.
4. List: all ticket files created, the dependency graph, the suggested implementation order, any **deferred `/reassess-spec` findings** ("may warrant separate tickets"), any **cross-spec follow-ups** surfaced by the spec's Risks / Assumptions section or discovered during decomposition, and any **post-approval mechanical refinements** applied during Step 5 (per §Step 5 Post-approval refinement — original table entry → applied change).
5. **Shared-file overlaps**: enumerate files that ≥2 mutually-independent tickets each modify — tickets with no `Deps` on each other, even when they share a common upstream `Deps` (parallel siblings off one foundation ticket still merge-conflict on a shared file), and whether the shared file is pre-existing or created in-batch by that foundation ticket — so implementers coordinate mechanical merges.

Do NOT commit. Leave files for user review.

## Guardrails

- **FOUNDATIONS is authoritative**: never approve a decomposition that violates a FOUNDATIONS principle or crosses a §12 stop condition — flag it as a CRITICAL Issue at Step 2 and await disposition.
- **Template fidelity**: every ticket uses `tickets/_TEMPLATE.md` exactly — no ad-hoc sections, no missing fields, no "simplified" variants. Template evolution is a separate spec.
- **Ticket fidelity**: never silently skip a deliverable. If one seems wrong, use the 1-problem / 3-options / 1-recommendation format and ask.
- **Codebase truth**: file paths, crate/module names, types, and schema references in tickets must be validated against the actual codebase, not assumed from the spec. Stale references propagated spec → ticket are a skill failure.
- **Reviewable size**: each ticket should be reviewable as a single diff. When in doubt, split.
- **Explicit dependencies**: declare inter-ticket ordering in `Deps`; never leave it implicit. Every `Deps` entry resolves to a ticket produced this run or a verified pre-existing path.
- **No spec edits**: this skill never edits the source spec. If decomposition reveals a spec defect, flag it as an Issue and route the fix to `/reassess-spec`.
- **Worktree discipline**: inside a worktree, all paths resolve from the worktree root.
- **Do not `git commit`**: writes land in the working tree; the user reviews and commits.

## FOUNDATIONS Alignment

| Principle | Step | Mechanism |
|-----------|------|-----------|
| §2 Behavior authority | Step 2 | Deliverables moving setup, legal-action generation, validation, state transitions, scoring, RNG, semantic effects, view projection, replay/hash, serialization, or bot decisions out of Rust (or letting TS decide legality) are flagged. |
| §3 `engine-core` is a contract kernel | Step 2 | Deliverables introducing a mechanic/domain noun into `engine-core` trip a boundary-failure Issue. |
| §4 `game-stdlib` is earned | Step 2 | Deliverables promoting a helper into `game-stdlib` without the mechanic-atlas / third-use earning are flagged. |
| §11 Universal acceptance invariants | Step 2 | Deliverables proposing validation must stay deterministic, fail-closed, and blocking and distinguish warnings from blockers; deliverables touching public/private views, effect logs, or replay exports must keep hidden information non-leaking; replay/hash/serialization must stay deterministic. Deviations are flagged. |
| §12 Stop conditions | Step 2 | Any stop condition the decomposition would cross (mechanic nouns in `engine-core`, procedural static data, YAML/DSL without ADR, TS deciding legality, hidden-info leakage, bot bypass, unbounded scope) is a CRITICAL Issue. |
| §13 ADR triggers; no silent retcon | Step 5 | Every ticket modifying existing behavior cites the change rationale in Assumption Reassessment (`tickets/README.md`); an architecture-changing decision (replay/hash semantics, visibility contracts, kernel vocabulary, new bot search class) is flagged for a required ADR. |

## Final Rule

A decomposition is not complete until every spec deliverable maps to a ticket OR to an explicit non-goal OR to a documented exempt category, every `Deps` resolves to a real target, every ticket's Files to Touch matches the current codebase, and every FOUNDATIONS-impacting deliverable has been validated against `docs/FOUNDATIONS.md` before its ticket was written.

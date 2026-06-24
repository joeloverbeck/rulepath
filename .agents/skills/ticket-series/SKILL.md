---
name: ticket-series
description: Use for Rulepath goals that implement a glob or series of tickets in dependency order from tickets/ with a referenced spec in specs/ or another reference artifact, including foundation-doc loading, one-ticket-at-a-time implementation, acceptance verification, per-ticket archival and commits, final reference closeout, and repository truthing.
---

# Ticket Series

Use this skill when the user asks to implement a Rulepath ticket series such as
`tickets/GATEPREFIX-*` with a reference spec such as `specs/gate-*`, especially
inside a `/goal`. Some maintenance series are ticket-only or are rooted in a
non-spec reference artifact such as a triage note, plan, or task document; close
that reference artifact instead of forcing a spec workflow.

Rulepath is Rust-first. Rust owns game behavior; TypeScript/React presents
Rust/WASM output only. Keep every ticket inside the boundaries in the live
foundation docs.

## Support Assets

- `scripts/audit-series-closeout.mjs` collects the standard final closeout
  audit surfaces for archived tickets, archived references, stale live paths,
  commit ledger, and git status. Use it as directed in Completion Audit. It
  also supports `--reference-only` for focused archived-reference truthing and
  `--expected-ticket-list <file>` for non-contiguous ticket families, plus
  `--ledger-format compact` for final-report-ready commit ledgers and
  `--summary` for low-noise successful long-series audits that still print
  exact failure lines.
- `agents/openai.yaml` is an OpenAI-facing skill manifest and prompt stub. It
  does not change the main workflow and does not authorize skipping the
  `SKILL.md` instructions.

## Inputs

- Ticket selector: usually a glob under `tickets/`.
- Reference selector: usually a glob under `specs/`, but sometimes a triage,
  task, plan, or other document that created the ticket family.
- Any explicit sequencing, verification, commit, or archival constraints from
  the prompt.

If an input glob is ambiguous, inspect matching paths and choose only when the
repo context makes the intended family clear. Ask before proceeding if multiple
families plausibly match.

If an input glob resolves to zero paths, make one bounded correction pass before
asking: search nearby ticket/spec prefixes in the live checkout, prefer exact
same-family stems and obvious one-character suffix/prefix differences, and
proceed only when there is a single plausible family. Report the correction in
the next update or final response. If there are zero or multiple plausible
families after that pass, stop and ask for the intended selector.

## Startup

1. Read the live checkout first. Do not rely on memory or prior runs for current
   ticket/reference state.
2. Check `git status --short` before editing. Preserve unrelated user changes.
3. If the prompt or dirty worktree indicates an interrupted or partially
   implemented prior run, reconcile the existing state before editing:
   - inspect staged, modified, and untracked paths;
   - classify which changes belong to the current ticket, earlier completed
     tickets, unrelated user work, generated output, or unknown state;
   - identify the resumed ticket boundary and evidence for that boundary;
   - do not commit or archive until the boundary is clear.
   After context compaction or any resumed continuation, run a minimal
   revalidation pass before editing: `git status --short`, active ticket glob,
   current ticket read, reference/index status, staged index, unrelated dirty
   paths, and the next exact action. Treat conversation summaries and resume
   ledgers as orientation, not proof of current checkout state.
4. Read the always-required Rulepath orientation and workflow docs:
   - `AGENTS.md`
   - `docs/README.md`
   - `docs/FOUNDATIONS.md`
   - `docs/AGENT-DISCIPLINE.md`
   - `docs/archival-workflow.md`
   - `specs/README.md`
   - `tickets/README.md`
5. Resolve the ticket selector and any reference selector to concrete paths.
   If there is no active spec, identify whether a triage, task, plan, or
   ticket-only series is the authoritative reference. Do not invent a spec.
6. Read the resolved reference artifact and tickets. Determine dependency order
   from explicit dependency sections, numbering, ticket prose, and reference
   sequencing.
7. Load targeted foundation docs from `docs/` for the surfaces the tickets touch.
   Use `docs/README.md` as the routing index. At minimum:
   - `docs/ARCHITECTURE.md` for crate ownership, dependency direction, action
     tree, replay, view, effect, WASM, or app boundary work.
   - `docs/ENGINE-GAME-DATA-BOUNDARY.md` for `engine-core`, `game-stdlib`,
     `games/*`, static content, formats, schema, or DSL/YAML pressure.
   - `docs/OFFICIAL-GAME-CONTRACT.md` for official-game implementation,
     docs, source notes, traces, coverage, simulations, benchmarks, bots, or UI.
   - `docs/MECHANIC-ATLAS.md` for repeated mechanics, helper promotion,
     primitive-pressure debt, or `game-stdlib` changes.
   - `docs/AI-BOTS.md` for bot policy, bot explanations, determinism, hidden
     information, candidate ranking, or strategy evidence.
   - `docs/UI-INTERACTION.md` for web presentation, legal-only controls,
     preview UX, semantic-effect animation, replay UI, accessibility, browser
     no-leak work, outcome surfaces, or status/live-region behavior.
   - `docs/TESTING-REPLAY-BENCHMARKING.md` and `docs/TRACE-SCHEMA-v1.md` for
     tests, golden traces, replay, hashes, fixtures, simulations, no-leak proof,
     benchmarks, CI evidence, browser smoke, e2e harness, or proof-lane changes
     even when no Rust behavior changes.
   - `docs/WASM-CLIENT-BOUNDARY.md` for Rust/WASM/browser API contracts, raw
     WASM payload shape, client bridge, or web smoke changes that depend on the
     built WASM artifact.
   - `docs/IP-POLICY.md` and `docs/SOURCES.md` for public rules prose, source
     notes, naming, assets, or private/licensed content risk.
   - relevant `docs/adr/*.md` files when the ticket/reference invokes an
     accepted ADR or trips an ADR trigger in `docs/FOUNDATIONS.md`.
8. If the ticket/reference conflicts with a foundation doc, the foundation doc wins.
   Stop and reassess when a `docs/FOUNDATIONS.md` stop condition appears.

## Per-Ticket Loop

Complete exactly one ticket before starting the next.

For each ticket:

1. Reassess assumptions against current code, docs, templates, specs, and crate
   ownership. Apply `tickets/README.md` pre-implementation checks.
2. If the ticket/reference diverges from current truth, update it first before
   implementation. Commit material ticket/reference correction separately when
   it changes scope or acceptance.
3. Identify the narrow implementation surface and exact acceptance criteria.
4. Make the minimal code/doc/test changes that satisfy the ticket while
   preserving Rulepath invariants:
   - `engine-core` stays generic and free of mechanic nouns.
   - game/mechanic behavior lives in typed Rust game modules first.
   - `game-stdlib` changes are earned through the mechanic atlas.
   - TypeScript never decides legality.
   - static data remains typed content/parameters/metadata, not behavior.
   - replay, hashes, RNG, serialization order, and traces remain deterministic
     unless the spec explicitly migrates them.
   - hidden information does not leak into payloads, DOM, storage, logs, bot
     explanations, replay exports, traces, tests, or diagnostics.
5. Run targeted checks that prove the ticket acceptance criteria. Use broader
   gates when the touched surface or ticket requires them. Prefer the narrowest
   truthful proof, but do not use unit tests as a substitute for replay,
   simulation, benchmark, browser, no-leak, or docs-link evidence when the ticket
   requires those surfaces. When a ticket changes Rust source or tests, run
   `cargo fmt --all --check` and at least the relevant crate clippy/test lane
   before archiving that ticket, unless the ticket records why a narrower proof
   is sufficient. When touching web play-surface files, run the
   repo's guard scripts when present and avoid introducing guard-trigger
   vocabulary such as debug payload terms into helper names, visible-adjacent
   strings, comments, or tests unless the checker is intentionally updated with
   a documented reason. When a ticket names a browser or e2e smoke as proof for
   a game-specific or UI-specific path, inspect the smoke enough to confirm it
   actually exercises that game/path. If it does not, extend the smoke or record
   why another proof surface is sufficient before claiming acceptance.
   If a browser smoke fails before assertions because the environment cannot
   bind its local `127.0.0.1` server, for example `listen EPERM`, rerun through
   the approved escalation path for localhost binding and record both the
   sandbox failure and successful rerun in the ticket `Outcome`, reference
   `Outcome`, or final report, whichever owns the evidence. Treat assertion
   failures as real test failures.
   When a passing command emits surprising scope or coverage details, record
   the observation and narrow the claim instead of overclaiming. For example, if
   a simulator passes but reports setup-level action counts rather than
   full-match play, quote that in the ticket or reference `Outcome`.
   For multi-hand, variable-seat, or schedule-heavy games, do not assume the
   simulator's default action cap is enough to prove terminal completion. Inspect
   the ticket/spec or simulator help when needed, set an explicit
   complete-match `--action-cap`, and record cap retries separately from rules
   failures.
6. Update the ticket following `docs/archival-workflow.md`:
   - mark final status at the top using exactly the archival workflow vocabulary:
     `**Status**: COMPLETED`, `**Status**: REJECTED`,
     `**Status**: DEFERRED`, or `**Status**: NOT IMPLEMENTED`
     (emoji variants allowed by the workflow);
   - for implemented tickets, use `COMPLETED`; `ACCEPTED` is an acceptance
     review state, not an archival final status;
   - add a bottom `Outcome` section for completed items;
   - include completion date, what changed, deviations from the plan, and
     verification results.
7. Archive the ticket:
   - Create `archive/tickets/` if absent.
   - Detect tracked state with `git ls-files --error-unmatch <ticket>`.
   - Use `git mv` for tracked tickets.
   - Use plain `mv` only for untracked tickets.
   - Confirm the original `tickets/` path is gone.
   - Stage archived renames with path-scoped staging such as
     `git add -A tickets archive/tickets`; after `git mv`, do not rely on
     adding the removed source path directly.
     A safe sequence for tracked tickets is:

```sh
git mv tickets/TICKET_ID.md archive/tickets/TICKET_ID.md
git add -A tickets archive/tickets
git add <other-ticket-owned-files>
```

   - Before committing, run a strict archived-ticket truth check against the
     archived ticket path. It must have an archival final status and `## Outcome`,
     and it must not have informal statuses such as `DONE`, `COMPLETE`,
     `ACCEPTED`, or `## Completion Notes`:

```sh
rg -n "^\*\*Status\*\*: (✅ )?COMPLETED$|^\*\*Status\*\*: (❌ )?REJECTED$|^\*\*Status\*\*: (⏸️ )?DEFERRED$|^\*\*Status\*\*: (🚫 )?NOT IMPLEMENTED$|^## Outcome" archive/tickets/TICKET_ID.md
rg -n "^\*\*Status\*\*: (DONE|COMPLETE|ACCEPTED)$|^## Completion Notes" archive/tickets/TICKET_ID.md && exit 1 || true
```

8. Sweep active specs, tickets, docs, indexes, README tables, and scripts for
   stale live ticket paths. Update references that should now point to
   `archive/tickets/`.
9. If current changes refine behavior, ownership, or verification facts from an
   already archived ticket in the same series, amend that archived ticket's
   `Outcome` before finalization. Follow `docs/archival-workflow.md` and add
   `Outcome amended: YYYY-MM-DD` inside `## Outcome` for each post-completion
   refinement.
10. Review the diff for unrelated changes. Before every per-ticket commit,
    inspect the staged index with `git diff --cached --name-status` or an
    equivalent path-scoped staged diff, and also check `git status --short`.
    Unstage or exclude unrelated user changes before committing.
    Run all git index-mutating commands serially, including `git add`,
    `git mv`, `git commit`, and `git restore --staged`; never run them through
    parallel tool calls.
    After build, copy, codegen, fixture-regeneration, or WASM/web bundling
    commands, run `git status --short` and classify generated changes before
    staging. Commit generated artifacts only when the ticket explicitly owns
    them, and otherwise leave or discard only your own unintended generated
    outputs without touching unrelated user changes.
11. Commit the completed ticket work before moving on. Use a concise message
    that names the ticket.

Do not advance to the next ticket on plausible implementation alone. Acceptance
criteria must pass, or the ticket must be explicitly blocked with evidence.

## Final Reference Closeout

After all tickets in the series are complete:

For ticket-only series with no reference artifact, do not force a spec or
reference closeout. Instead, perform a ticket-only closeout: confirm the active
ticket glob is empty, the archived ticket list and count match the startup
resolution, every archived ticket has a valid final status plus `## Outcome`,
active specs/tickets/docs/apps/games/scripts have no stale live ticket paths,
required verification commands were run, required per-ticket commits exist, and
the final worktree/index excludes unrelated changes. State that no reference
artifact was closed and skip the reference archival steps below.

1. Re-read the reference artifact and verify every work item and exit criterion
   is done, explicitly rejected, deferred, not implemented, or not applicable.
2. Run the relevant final gates. Choose based on the reference artifact and
   touched surfaces.
   Common Rulepath gates include:

```sh
cargo fmt --all --check
cargo clippy --workspace --all-targets -- -D warnings
cargo build --workspace
cargo test --workspace
cargo run -p simulate -- --game <game_id> --games 1000
cargo run -p replay-check -- --game <game_id> --all
cargo run -p fixture-check -- --game <game_id>
cargo run -p rule-coverage -- --game <game_id>
bash scripts/boundary-check.sh
node scripts/check-doc-links.mjs
node scripts/check-catalog-docs.mjs
node scripts/check-player-rules.mjs
node scripts/check-outcome-explanations.mjs
npm --prefix apps/web ci
npm --prefix apps/web run smoke:wasm
npm --prefix apps/web run build
npm --prefix apps/web run smoke:ui
npm --prefix apps/web run smoke:effects
npm --prefix apps/web run smoke:e2e
```

When adding or renaming e2e smoke scripts, check whether each script represents
a catalog game or a non-game/app-level proof. Non-game smokes may need explicit
whitelisting in catalog/doc checker scripts and corresponding README smoke-list
updates before `node scripts/check-catalog-docs.mjs` can truthfully pass.

If a browser smoke fails before assertions because the environment cannot bind
its local `127.0.0.1` server, for example `listen EPERM`, rerun through the
approved escalation path for localhost binding and record both the sandbox
failure and successful rerun in the ticket `Outcome`, reference `Outcome`, or
final report, whichever owns the evidence. Treat assertion failures as real test
failures.

If the ticket or reference requires non-command visual evidence such as
screenshots, manual focus-visible checks, reduced-motion observations, contrast
notes, or visual review notes, either produce and name the artifact/note in the
ticket or reference `Outcome`, or explicitly record the skipped evidence with a
reason. Do not silently substitute browser smoke commands for requested visual
evidence.

If final gates pass and only docs/archive/tracker closeout files change
afterward, do not treat the earlier heavy gate as stale by default. Re-run
lightweight truth checks such as `git diff --check`, doc-link/catalog checks
when those surfaces changed, and the closeout audit; record that no source,
test, fixture, golden, or active behavior-authority file changed after the
heavy gate. If source, tests, fixtures, golden traces, generated behavior
artifacts, or active authority docs change after a gate, rerun the affected gate
or mark the earlier result as preliminary.

3. Update the reference artifact following `docs/archival-workflow.md`:
   - mark final status at the top using exactly the archival workflow vocabulary:
     `**Status**: COMPLETED`, `**Status**: REJECTED`,
     `**Status**: DEFERRED`, or `**Status**: NOT IMPLEMENTED`
     (emoji variants allowed by the workflow);
   - for implemented non-spec references, use `COMPLETED`; `Done`, `ACCEPTED`,
     or other informal statuses are not archival final statuses;
   - add a bottom `Outcome` section for completed specs or reference docs;
   - include completion date, what changed, deviations from the plan, and
     verification results.
   For specs that already use the repository's spec-table header convention and
   prior archived specs use that convention, preserve the spec-table convention
   instead of forcing a `**Status**: COMPLETED` line. Use the exact archived-spec
   row label `| Status |` with value `Done` or `` `Done` `` (for example,
   ``| Status | `Done` |``). Do not use bolded labels such as
   ``| **Status** | `Done` |``; the closeout helper intentionally rejects that
   near-miss. The archived spec still must have a truthful final status and a
   bottom `## Outcome`. Before the reference archive commit, the archived spec
   should contain this exact shape:

```markdown
| Field | Value |
|---|---|
| Status | `Done` |

## Outcome
```

   Tickets continue to use the `**Status**: COMPLETED` archival workflow
   vocabulary.
4. Archive the reference artifact, using `git mv` when tracked:
   - specs to `archive/specs/`;
   - triage notes from `docs/triage/` to `archive/triage/`;
   - task docs to `archive/tasks/`;
   - plans to `archive/plans/`.
   If the series is ticket-only and has no reference artifact, state that and
   skip this archive step.
   If the final/capstone ticket says reference archival is out of that ticket's
   local scope, do not let that ticket-local note override the series-level
   closeout. Finish, archive, and commit the capstone ticket first; then perform
   the reference artifact archival and truthing as a separate final closeout
   step unless the user explicitly forbids reference archival.
5. Repair active references and progress surfaces, especially `specs/README.md`
   when a spec was closed, and any active tickets, repo docs, per-game docs
   under `games/*/docs/`, app README tables, catalog/smoke lists, or scripts
   that referenced the live reference path.
   For report artifacts, distinguish current evidence reports from historical
   provenance reports. Retarget live-path references in active characterization
   or acceptance reports that remain part of the closeout evidence. Historical
   research briefs, archived-ticket notes, and provenance-only report text may
   keep the original live path unless the active ticket/reference explicitly
   requires repair.
   If capstone success makes a live checklist, status table, release note, or
   source/reference note false, update that closeout fact even when the ticket's
   local `Files to Touch` list is narrower. Record the extra closeout repair as
   a deviation in the capstone ticket or reference `Outcome`.
   If a release/IP checklist asks for human or legal review and no human has
   provided that review in the current session, record the review as pending or
   maintainer-owned. Do not invent a human signoff; distinguish automated/docs
   evidence from human release clearance.
   For specs, distinguish the progress index from the archived artifact:
   `specs/README.md` may keep its progress status as `Done`, while the archived
   spec document itself must use the repo's current archived-spec status
   convention plus `## Outcome`.
6. Assert the archived reference is truthy before goal completion:
   - read or grep the archived artifact and confirm archival final status and
     `Outcome`;
   - confirm the original active reference path is gone;
   - confirm active references point to the archive path when they should.
   This is a hard stop: execute the concrete completion-audit commands from the
   next section against the live checkout before any final response or
   `update_goal` call. A mental checklist is not enough for final reference
   truthing. For every spec or reference closeout commit, run the focused
   archived-reference helper after the reference status/outcome edits and
   before staging or committing the reference closeout:

```sh
node .agents/skills/ticket-series/scripts/audit-series-closeout.mjs --reference-only --active-reference ACTIVE_REFERENCE_PATH --archived-reference archive/specs/ARCHIVED_REFERENCE.md --summary
```
7. Run a final status/diff check and commit the reference archive/truthing work.
   Run all git index-mutating commands serially during this closeout, including
   `git add`, `git mv`, `git commit`, and `git restore --staged`; never run
   them through parallel tool calls.
8. If a `/goal` is active, mark it complete only after implementation,
   verification, ticket archives, reference archive/repair when applicable, and
   required commits are done.

Goal completion gate:

1. Run `scripts/audit-series-closeout.mjs` or the manual Completion Audit
   equivalent against the live checkout.
2. Fix every failure or explicitly classify it as out of scope with evidence.
3. Commit any audit repair, including reference-status or stale-path fixes.
4. Rerun the closeout audit after the final commit and inspect the output.
5. Confirm `git status --short` shows only unrelated worktree changes and
   `git diff --cached --name-status` is empty.
6. Call `update_goal` only after all checks above are true.

If a resumed continuation finds that implementation, archival, reference repair,
verification, and commits are already complete, switch to an audit-only
completion pass: re-run the live closeout audit, confirm no active ticket or
reference path remains unexpectedly, check the staged index and worktree, and
avoid edits unless the audit exposes a real closeout defect. When that pass is
green, call `update_goal` without reopening completed tickets.

If a final gate uncovers a stale test assertion, proof fixture, or small defect
owned by an earlier archived ticket, fix it before closeout. Record the fix in
the current ticket or reference `Outcome`. Amend the earlier archived ticket
only when its recorded behavior, accepted surface, or verification claim became
inaccurate.

For large capstones, record a compact evidence ledger in the reference artifact
and/or final ticket before archival. Use headings that match the actual proof
surface, for example:

- Rust: fmt, clippy, build, workspace tests.
- Per-game: simulation, replay-check, fixture-check, rule-coverage.
- Web: build, smoke:wasm, smoke:ui, smoke:effects, smoke:e2e.
- Docs/boundary: doc links, catalog docs, player rules, presentation copy,
  boundary checks.
- Archive truthing: active glob empty, archived ticket count/status/outcome,
  archived reference status/outcome, stale live-path sweep.

For long series, build the ledger from the archived ticket `Outcome` sections
instead of memory. A compact checklist is enough:

- List the archived ticket paths and count.
- Group verification commands by proof surface.
- Note any rerun or escalation evidence and where it was recorded.
- Note intentionally skipped gates with reasons.
- Note surprising-but-passing command output that narrows the evidence claim.
- Confirm the archived reference status/outcome and stale-path sweep.

For long-running or resumed series that may span context compaction, keep an
optional resume ledger in the conversation or a repo-approved run-state file
when one already exists for the workflow. The ledger should be compact and
current: active ticket, archived tickets and commits, commands already run,
known unrelated dirty paths, current blockers, and the next exact action. Do
not create a new persistent run-state file unless the repo or user already
expects one.

For a final reference `Outcome`, this compact structure is usually enough:

- Completed tickets: archived paths and count.
- Implementation summary: major shipped surfaces.
- Deviations: intentional differences from the plan.
- Verification evidence: grouped commands actually run.
- Manual/non-command evidence: screenshots, focus/contrast notes, or skipped
  evidence with reasons.
- Archive truthing: active glob empty, archived status/outcome checks, stale
  path sweeps.
- Unrelated worktree changes: named paths left untouched.

## Completion Audit

Before reporting done or marking a `/goal` complete, prove the final state from
the live checkout:

- Active ticket/reference globs are empty, or any remaining active files are
  intentionally out of scope and explained.
- `archive/tickets/` contains every expected ticket in the series; each archived
  ticket has archival final status (`COMPLETED`, `REJECTED`, `DEFERRED`, or
  `NOT IMPLEMENTED`, with allowed emoji variants) and an `Outcome` or a clear
  non-completed disposition. Reject `ACCEPTED` or other review-state statuses
  for archived implemented tickets.
- The appropriate archive directory contains the reference artifact when the
  series closes one; the archived artifact has a repo-valid final status and an
  `Outcome`. For ticket files and non-spec references governed by
  `docs/archival-workflow.md`, reject `Done`, `ACCEPTED`, or other informal
  statuses before reporting done or calling `update_goal`. For archived specs
  that use the repo's table-style spec header, `Done` in the status table is
  acceptable only with the exact `| Status |` row label, when prior archived
  specs use the same convention and `## Outcome` is present. Reject bolded
  near-misses such as ``| **Status** | `Done` |``.
- `specs/README.md`, progress surfaces, README/catalog surfaces, docs,
  `games/*/docs/`, scripts, and active tickets/reference artifacts no longer
  point at stale live paths.
  Distinguish live paths from archive paths during this sweep; references to
  `archive/specs/...` or `archive/tickets/...` are acceptable when the active
  tracker intentionally points there.
- Required verification commands were actually run and match the ticket/reference
  scope; skipped checks are named with reasons.
- The final diff/status excludes unrelated user changes, and required commits
  are present.

Run a concrete version of this checklist before reporting done or calling
`update_goal`; adapt placeholders to the ticket prefix and reference path:

When the ticket range and reference artifact are known, run
`scripts/audit-series-closeout.mjs` by default to collect the standard audit
surfaces, then inspect its output and run any repo-specific checks that the
helper cannot infer. For non-contiguous or unusual series where the helper
cannot express the expected set cleanly, run the manual audit commands below
and explain why the helper was skipped.

For second-pass reruns after a passing final commit, prefer `--summary` with
`--ledger-format compact`, inspect failures and key `OK` surfaces first, and
summarize the rerun result instead of re-printing every archived-ticket or
stale-path row into the final response. Expand the output only when a mismatch
or unusual series shape needs detailed evidence.

```sh
node .agents/skills/ticket-series/scripts/audit-series-closeout.mjs --reference-only --active-reference ACTIVE_REFERENCE_PATH --archived-reference archive/specs/ARCHIVED_REFERENCE.md --summary
node .agents/skills/ticket-series/scripts/audit-series-closeout.mjs --ticket-prefix TICKET_PREFIX --active-reference ACTIVE_REFERENCE_PATH --archived-reference archive/specs/ARCHIVED_REFERENCE.md --expected-count N
node .agents/skills/ticket-series/scripts/audit-series-closeout.mjs --ticket-prefix TICKET_PREFIX --active-reference ACTIVE_REFERENCE_PATH --archived-reference archive/specs/ARCHIVED_REFERENCE.md --expected-ticket-list /tmp/expected-tickets.txt --ledger-format compact --summary
node .agents/skills/ticket-series/scripts/audit-series-closeout.mjs --ticket-prefix TICKET_PREFIX --active-reference ACTIVE_REFERENCE_PATH --archived-reference archive/specs/ARCHIVED_REFERENCE.md --expected-ticket-range TICKET_PREFIX-001..020 --ledger-format compact --summary
rg -n "TICKET_PREFIX" tickets || true
find archive/tickets -maxdepth 1 -name "TICKET_PREFIX*.md" -print | sort
find archive/tickets -maxdepth 1 -name "TICKET_PREFIX*.md" -print | sort | wc -l
rg -n '^\*\*Status\*\*:|^## Outcome' archive/tickets/TICKET_PREFIX*.md
test ! -e ACTIVE_REFERENCE_PATH
rg -n '^\*\*Status\*\*: (✅ )?COMPLETED$|^\*\*Status\*\*: (❌ )?REJECTED$|^\*\*Status\*\*: (⏸️ )?DEFERRED$|^\*\*Status\*\*: (🚫 )?NOT IMPLEMENTED$|^\| Status \| `?Done`? \||^## Outcome' archive/specs/ARCHIVED_REFERENCE.md
rg -n '^\s*-?\s*\*\*Status\*\*:\s*(Done|ACCEPTED)|^\s*- \*\*Status:\*\*' archive/specs/ARCHIVED_REFERENCE.md && exit 1 || true
rg -n -P '(?<!archive/)ACTIVE_REFERENCE_PATH|(?<!archive/)tickets/TICKET_PREFIX' specs tickets docs apps games scripts || true
rg -n -P 'archive/(specs/ARCHIVED_REFERENCE|tickets/TICKET_PREFIX)' specs docs apps games scripts || true
git log --oneline --grep='TICKET_PREFIX' --all
git status --short
git diff --cached --name-status
```

Compare the archived ticket name list and count against the concrete ticket
paths resolved at startup. Compare the commit ledger output against the archived
ticket list before the final response, and include every per-ticket commit ID
when commits were made as part of the series. When using the helper, pass
`--expected-count`, `--expected-ticket-range`, or `--expected-ticket-list` when
the startup ticket list is known. Use `--expected-ticket-list` for
non-contiguous series; the file may contain ticket IDs such as
`TICKET_PREFIX-001`, basenames such as `TICKET_PREFIX-001.md`, or archive paths
such as `archive/tickets/TICKET_PREFIX-001.md`, one per line, with blank lines
and `#` comments ignored. Still inspect the printed names for unusual series.
If the reference status grep finds no valid archival status line, or the
informal-status guard finds `ACCEPTED`, a bullet-style status field, or `Done`
in a non-spec archival status line, the reference is not truthy enough for
completion. For archived specs using the table-style status header, accept
`Done` only when `## Outcome` is present and the active progress index points at
the archived spec. Historical `reports/` may contain provenance-only live paths;
do not hard-fail those unless the active reference or ticket requires report
repair, but do retarget active characterization/evidence reports that remain
part of closeout evidence and sweep active per-game docs under `games/*/docs/`.

## Reporting

Final responses must include:

- Confirmation that the closeout audit passed after the final commit and before
  writing the final response.
- Tickets completed and archived.
- Per-ticket commit IDs when commits were made as part of the series.
  For long contiguous series, use a compact ledger instead of summarizing only
  the latest commits, for example:

```text
001 4119f20, 002 cb5bea1, 003 a88b254, 004 1c02d99, 005 e32f411
006 2ac1f7b, 007 7819e22, 008 84fb0c5, 009 a5321dd, 010 d4198be
```

  The closeout helper can generate this form with `--ledger-format compact`.

- Reference artifact archived, or reason no spec/reference artifact was closed.
- Verification commands actually run.
- Any checks not run and why.
- Any unrelated pre-existing changes left untouched.

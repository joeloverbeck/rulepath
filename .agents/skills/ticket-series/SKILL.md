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

## Inputs

- Ticket selector: usually a glob under `tickets/`.
- Reference selector: usually a glob under `specs/`, but sometimes a triage,
  task, plan, or other document that created the ticket family.
- Any explicit sequencing, verification, commit, or archival constraints from
  the prompt.

If an input glob is ambiguous, inspect matching paths and choose only when the
repo context makes the intended family clear. Ask before proceeding if multiple
families plausibly match.

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
   requires those surfaces.
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
11. Commit the completed ticket work before moving on. Use a concise message
    that names the ticket.

Do not advance to the next ticket on plausible implementation alone. Acceptance
criteria must pass, or the ticket must be explicitly blocked with evidence.

## Final Reference Closeout

After all tickets in the series are complete:

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

If a browser smoke fails before assertions because the environment cannot bind
its local `127.0.0.1` server, for example `listen EPERM`, rerun through the
approved escalation path for localhost binding and record both the sandbox
failure and successful rerun in the ticket `Outcome`, reference `Outcome`, or
final report, whichever owns the evidence. Treat assertion failures as real test
failures.

3. Update the reference artifact following `docs/archival-workflow.md`:
   - mark final status at the top using exactly the archival workflow vocabulary:
     `**Status**: COMPLETED`, `**Status**: REJECTED`,
     `**Status**: DEFERRED`, or `**Status**: NOT IMPLEMENTED`
     (emoji variants allowed by the workflow);
   - for implemented references, use `COMPLETED`; `Done`, `ACCEPTED`, or other
     informal statuses are not archival final statuses;
   - add a bottom `Outcome` section for completed specs or reference docs;
   - include completion date, what changed, deviations from the plan, and
     verification results.
4. Archive the reference artifact, using `git mv` when tracked:
   - specs to `archive/specs/`;
   - triage notes from `docs/triage/` to `archive/triage/`;
   - task docs to `archive/tasks/`;
   - plans to `archive/plans/`.
   If the series is ticket-only and has no reference artifact, state that and
   skip this archive step.
5. Repair active references and progress surfaces, especially `specs/README.md`
   when a spec was closed, and any active tickets, docs, app README tables,
   catalog/smoke lists, or scripts that referenced the live reference path.
   For specs, distinguish the progress index from the archived artifact:
   `specs/README.md` may keep its progress status as `Done`, while the archived
   spec document itself must use archival status `**Status**: COMPLETED` plus
   `## Outcome`.
6. Assert the archived reference is truthy before goal completion:
   - read or grep the archived artifact and confirm archival final status and
     `Outcome`;
   - confirm the original active reference path is gone;
   - confirm active references point to the archive path when they should.
7. Run a final status/diff check and commit the reference archive/truthing work.
8. If a `/goal` is active, mark it complete only after implementation,
   verification, ticket archives, reference archive, reference repair, and required
   commits are done.

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
  series closes one; the archived artifact has archival final status and an
  `Outcome`. Reject `Done`, `ACCEPTED`, or other informal statuses before
  reporting done or calling `update_goal`.
- `specs/README.md`, progress surfaces, README/catalog surfaces, docs, scripts,
  and active tickets/reference artifacts no longer point at stale live paths.
- Required verification commands were actually run and match the ticket/reference
  scope; skipped checks are named with reasons.
- The final diff/status excludes unrelated user changes, and required commits
  are present.

Run a concrete version of this checklist before reporting done or calling
`update_goal`; adapt placeholders to the ticket prefix and reference path:

```sh
rg -n "TICKET_PREFIX" tickets || true
find archive/tickets -maxdepth 1 -name "TICKET_PREFIX*.md" -print | sort
find archive/tickets -maxdepth 1 -name "TICKET_PREFIX*.md" -print | sort | wc -l
rg -n "^\*\*Status\*\*:|^## Outcome" archive/tickets/TICKET_PREFIX*.md
test ! -e ACTIVE_REFERENCE_PATH
rg -n "^\*\*Status\*\*: (✅ )?COMPLETED$|^\*\*Status\*\*: (❌ )?REJECTED$|^\*\*Status\*\*: (⏸️ )?DEFERRED$|^\*\*Status\*\*: (🚫 )?NOT IMPLEMENTED$|^## Outcome" archive/specs/ARCHIVED_REFERENCE.md
rg -n "^\s*-?\s*\*\*Status\*\*:\s*(Done|ACCEPTED)|^\s*- \*\*Status:\*\*" archive/specs/ARCHIVED_REFERENCE.md && exit 1 || true
rg -n "ACTIVE_REFERENCE_PATH|tickets/TICKET_PREFIX" specs tickets docs apps scripts || true
git status --short
git diff --cached --name-status
```

Compare the archived ticket name list and count against the concrete ticket
paths resolved at startup. If the reference status grep finds no valid archival
status line, or the informal-status guard finds `Done`, `ACCEPTED`, or a
bullet-style status field, the reference is not truthy enough for completion.

## Reporting

Final responses must include:

- Tickets completed and archived.
- Reference artifact archived, or reason no spec/reference artifact was closed.
- Verification commands actually run.
- Any checks not run and why.
- Any unrelated pre-existing changes left untouched.

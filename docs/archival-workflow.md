# Archival Workflow

Use this as the canonical, single-source archival policy for tickets, specs, agent tasks, design/plan docs, and triage docs.

## Required Steps

1. Edit the document to mark final status at the top:
   - `**Status**: ✅ COMPLETED` or `**Status**: COMPLETED`
   - `**Status**: ❌ REJECTED` or `**Status**: REJECTED`
   - `**Status**: ⏸️ DEFERRED` or `**Status**: DEFERRED`
   - `**Status**: 🚫 NOT IMPLEMENTED` or `**Status**: NOT IMPLEMENTED`
2. For completed items, add an `Outcome` section at the bottom with:
   - completion date
   - what actually changed
   - deviations from original plan
   - verification results
3. For archived specs, reports, ticket series, agent tasks, design/plan docs, and triage docs whose work changes repository authority, evidence surfaces, or closeout state, add a closeout receipt inside `## Outcome` with:
   - implementation commit or commit range, when known
   - accepted, rejected, deferred, or not-implemented recommendations
   - affected foundation sections, ADR status/supersession, or authority-map changes
   - open mechanic-atlas or mechanical-scaffolding debt and its owner
   - trace, hash, replay, fixture, catalog, doc-link, boundary, or browser checks run
   - live index/tracker links that now point at the archived artifact
   - note that historical commit strings inside archived reports remain content, not current-repository provenance
4. If implementation is refined after archival and the archived `Outcome` becomes stale, amend the archived document before merge/finalization so ownership, behavior, and verification facts remain accurate.
   - Add `Outcome amended: YYYY-MM-DD` inside `## Outcome` for each post-completion refinement update.
5. Ensure destination archive directory exists (create with `mkdir -p` if absent):
   - `archive/tickets/`
   - `archive/specs/`
   - `archive/tasks/`
   - `archive/plans/`
   - `archive/triage/`
6. Move the document. Prefer `git mv <source> <destination>` when the source is tracked; fall back to plain `mv` when untracked. Detect via `git ls-files --error-unmatch <source>`; non-zero exit → untracked → use plain `mv`.
7. If there is a filename collision, pass an explicit non-colliding destination filename.
8. Confirm the original path no longer exists in its source folder (`tickets/`, `specs/`, `tasks/`, `docs/plans/`, or `docs/triage/`).

## Roadmap Phase Rollover

When the living spec index rolls to a new roadmap phase:

1. Archive the old `specs/README.md` under `archive/specs/` with a
   date-suffixed name, such as `README-YYYY-MM-DD.md`.
2. Add an archive note inside the old index before moving it. The note should
   name the rollover date, the authority commit or manifest used, and the reason
   the old index is frozen.
3. Create the new `specs/README.md` from the accepted roadmap and ADR authority
   for the new phase.
4. Preserve links from the new index to the archived index so completed prior
   gates remain discoverable.
5. Record the authority commit, manifest, or source report that seeded the new
   index. If no accepted ADR exists for a roadmap reorder/addition, stop before
   changing roadmap law.
6. Archive and commit the rollover as its own bounded change when practical,
   before executing the new phase's implementation tickets.

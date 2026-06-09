---
name: ticket-series
description: Use for Rulepath goals that implement a glob or series of tickets in dependency order from tickets/ with a referenced spec in specs/, including foundation-doc loading, one-ticket-at-a-time implementation, acceptance verification, per-ticket archival and commits, final spec archival, and repository truthing.
---

# Ticket Series

Use this skill when the user asks to implement a Rulepath ticket series such as
`tickets/GATEPREFIX-*` with a reference spec such as `specs/gate-*`, especially
inside a `/goal`.

Rulepath is Rust-first. Rust owns game behavior; TypeScript/React presents
Rust/WASM output only. Keep every ticket inside the boundaries in the live
foundation docs.

## Inputs

- Ticket selector: usually a glob under `tickets/`.
- Reference spec selector: usually a glob under `specs/`.
- Any explicit sequencing, verification, commit, or archival constraints from
  the prompt.

If an input glob is ambiguous, inspect matching paths and choose only when the
repo context makes the intended family clear. Ask before proceeding if multiple
families plausibly match.

## Startup

1. Read the live checkout first. Do not rely on memory or prior runs for current
   ticket/spec state.
2. Check `git status --short` before editing. Preserve unrelated user changes.
3. Read the always-required Rulepath orientation and workflow docs:
   - `AGENTS.md`
   - `docs/README.md`
   - `docs/FOUNDATIONS.md`
   - `docs/AGENT-DISCIPLINE.md`
   - `docs/archival-workflow.md`
   - `specs/README.md`
   - `tickets/README.md`
4. Resolve the ticket and spec selectors to concrete paths.
5. Read the resolved spec and tickets. Determine dependency order from explicit
   dependency sections, numbering, ticket prose, and spec sequencing.
6. Load targeted foundation docs from `docs/` for the surfaces the tickets touch.
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
     preview UX, semantic-effect animation, replay UI, accessibility, or
     browser no-leak work.
   - `docs/TESTING-REPLAY-BENCHMARKING.md` and `docs/TRACE-SCHEMA-v1.md` for
     tests, golden traces, replay, hashes, fixtures, simulations, no-leak proof,
     benchmarks, or CI evidence.
   - `docs/WASM-CLIENT-BOUNDARY.md` for Rust/WASM/browser API contracts.
   - `docs/IP-POLICY.md` and `docs/SOURCES.md` for public rules prose, source
     notes, naming, assets, or private/licensed content risk.
   - relevant `docs/adr/*.md` files when the ticket/spec invokes an accepted
     ADR or trips an ADR trigger in `docs/FOUNDATIONS.md`.
7. If the ticket/spec conflicts with a foundation doc, the foundation doc wins.
   Stop and reassess when a `docs/FOUNDATIONS.md` stop condition appears.

## Per-Ticket Loop

Complete exactly one ticket before starting the next.

For each ticket:

1. Reassess assumptions against current code, docs, templates, specs, and crate
   ownership. Apply `tickets/README.md` pre-implementation checks.
2. If the ticket/spec diverges from current truth, update the ticket/spec first
   before implementation. Commit material ticket/spec correction separately when
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
   - mark final status at the top (`COMPLETED`, `REJECTED`, `DEFERRED`, or
     `NOT IMPLEMENTED`, with emoji variants allowed by the workflow);
   - add a bottom `Outcome` section for completed items;
   - include completion date, what changed, deviations from the plan, and
     verification results.
7. Archive the ticket:
   - Create `archive/tickets/` if absent.
   - Detect tracked state with `git ls-files --error-unmatch <ticket>`.
   - Use `git mv` for tracked tickets.
   - Use plain `mv` only for untracked tickets.
   - Confirm the original `tickets/` path is gone.
8. Sweep active specs, tickets, docs, indexes, README tables, and scripts for
   stale live ticket paths. Update references that should now point to
   `archive/tickets/`.
9. Review the diff for unrelated changes.
10. Commit the completed ticket work before moving on. Use a concise message
    that names the ticket.

Do not advance to the next ticket on plausible implementation alone. Acceptance
criteria must pass, or the ticket must be explicitly blocked with evidence.

## Final Spec Closeout

After all tickets in the series are complete:

1. Re-read the reference spec and verify every work item and exit criterion is
   done, explicitly rejected, deferred, not implemented, or not applicable.
2. Run the relevant final gates. Choose based on the spec and touched surfaces.
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
npm --prefix apps/web ci
npm --prefix apps/web run smoke:wasm
npm --prefix apps/web run build
npm --prefix apps/web run smoke:ui
npm --prefix apps/web run smoke:e2e
```

3. Update the spec following `docs/archival-workflow.md`:
   - mark final status at the top (`COMPLETED`, `REJECTED`, `DEFERRED`, or
     `NOT IMPLEMENTED`, with emoji variants allowed by the workflow);
   - add a bottom `Outcome` section for completed specs;
   - include completion date, what changed, deviations from the plan, and
     verification results.
4. Archive the spec to `archive/specs/`, using `git mv` when tracked.
5. Repair active references and progress surfaces, especially `specs/README.md`
   and any active tickets, docs, app README tables, catalog/smoke lists, or
   scripts that referenced the live spec path.
6. Assert the archived spec is truthy before goal completion:
   - read or grep the archived spec and confirm final status and `Outcome`;
   - confirm the original active spec path is gone;
   - confirm active references point to the archive path when they should.
7. Run a final status/diff check and commit the spec archive/truthing work.
8. If a `/goal` is active, mark it complete only after implementation,
   verification, ticket archives, spec archive, reference repair, and required
   commits are done.

## Completion Audit

Before reporting done or marking a `/goal` complete, prove the final state from
the live checkout:

- Active ticket/spec globs are empty, or any remaining active files are
  intentionally out of scope and explained.
- `archive/tickets/` contains every expected ticket in the series; each archived
  ticket has final status and an `Outcome` or a clear non-completed disposition.
- `archive/specs/` contains the reference spec when the series closes it; the
  archived spec has final status and an `Outcome`.
- `specs/README.md`, progress surfaces, README/catalog surfaces, docs, scripts,
  and active tickets/specs no longer point at stale live paths.
- Required verification commands were actually run and match the ticket/spec
  scope; skipped checks are named with reasons.
- The final diff/status excludes unrelated user changes, and required commits
  are present.

## Reporting

Final responses must include:

- Tickets completed and archived.
- Spec archived or reason it remains active.
- Verification commands actually run.
- Any checks not run and why.
- Any unrelated pre-existing changes left untouched.

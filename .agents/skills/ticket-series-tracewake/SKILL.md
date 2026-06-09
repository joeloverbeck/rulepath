---
name: ticket-series
description: Use for goals that implement a glob or series of tickets in dependency order from tickets/ with a referenced spec in specs/ or docs/4-specs/, including one-ticket-at-a-time implementation, acceptance verification, per-ticket archival and commits, final spec archival, and repository truthing.
---

# Ticket Series

Use this skill when the user asks to implement a ticket series such as
`tickets/0004PHA1THIHAR*` using a reference spec such as `specs/0004_PHASE_1*`,
especially inside a `/goal`.

## Inputs

- Ticket selector: usually a glob under `tickets/`.
- Reference spec selector: usually a glob under `specs/` or `docs/4-specs/`.
- Any explicit sequencing, verification, commit, or archival constraints from
  the prompt.

If an input glob is ambiguous, inspect matching paths and choose only when the
repo context makes the intended family clear. Ask before proceeding if multiple
families plausibly match.

## Startup

1. Read the live checkout first. Do not rely on memory or prior runs for current
   ticket/spec state.
2. Confirm repository guidance and authority:
   - `AGENTS.md`
   - `docs/README.md`
   - `docs/0-foundation/02_CONSTITUTIONAL_INVARIANTS.md`
   - `tickets/README.md`
   - `docs/4-specs/SPEC_LEDGER.md`
   - `docs/archival-workflow.md`
3. Resolve the ticket and spec selectors to concrete paths.
4. Read the resolved spec and tickets. Determine dependency order from explicit
   dependency sections, numbering, ticket prose, and spec sequencing.
5. Check `git status --short` before editing. Preserve unrelated user changes.

## Per-Ticket Loop

Complete exactly one ticket before starting the next.

For each ticket:

1. Reassess assumptions against current code, docs, and crate ownership.
   If the ticket/spec diverges from current truth, correct the ticket/spec first
   and commit that correction separately when it is material.
2. Identify the narrow implementation surface and the exact acceptance criteria.
3. Make the minimal code/doc/test changes that satisfy the ticket while
   preserving the repository's documented invariants, ownership boundaries, and
   dependency direction.
4. Run targeted checks that prove the ticket acceptance criteria. Use broader
   gates when the touched surface or ticket requires them.
5. Update the ticket with final status and an `Outcome` section following
   `docs/archival-workflow.md`.
6. Archive the ticket:
   - Create `archive/tickets/` if absent.
   - Prefer `git mv` for tracked tickets.
   - Use plain `mv` only for untracked tickets.
   - Confirm the original `tickets/` path is gone.
7. Sweep active specs, ledgers, docs, and ticket references for stale live
   ticket paths. Update references that should now point to `archive/tickets/`.
8. Review the diff for unrelated changes.
9. Commit the completed ticket work before moving on. Use a concise message that
   names the ticket.

Do not advance to the next ticket on plausible implementation alone. Acceptance
criteria must pass, or the ticket must be explicitly blocked with evidence.

## Final Spec Closeout

After all tickets in the series are complete:

1. Re-read the reference spec and verify every acceptance item is either done,
   explicitly rejected, deferred, or not implemented.
2. Run the relevant final gates. For full completion, prefer:

```sh
cargo fmt --all --check
cargo clippy --workspace --all-targets -- -D warnings
cargo build --workspace --all-targets --locked
cargo test --workspace
```

3. Update the spec with final status and an `Outcome` section following
   `docs/archival-workflow.md`.
4. Archive the spec to `archive/specs/`, using `git mv` when tracked.
5. Repair active references and ledgers, especially `docs/4-specs/SPEC_LEDGER.md`
   and any implementation-order or index surfaces found in the repo.
6. Run a final status/diff check and commit the spec archive/truthing work.
7. If a `/goal` is active, mark it complete only after implementation,
   verification, ticket archives, spec archive, reference repair, and required
   commits are done.

## Reporting

Final responses must include:

- Tickets completed and archived.
- Spec archived or reason it remains active.
- Verification commands actually run.
- Any checks not run and why.
- Any unrelated pre-existing changes left untouched.

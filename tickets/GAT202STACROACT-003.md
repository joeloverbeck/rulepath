# GAT202STACROACT-003: Gate 20.2 docs reconciliation + closeout

**Status**: PENDING
**Priority**: MEDIUM
**Effort**: Small
**Engine Changes**: None — docs/status-only (`games/starbridge_crossing/docs/UI.md`, `games/starbridge_crossing/docs/GAME-EVIDENCE.md`, `specs/README.md`, the spec's own Status)
**Deps**: GAT202STACROACT-001, GAT202STACROACT-002

## Problem

With the Rust active-seat catalog metadata (GAT202STACROACT-001) and the web-shell
consumption (GAT202STACROACT-002) landed, Gate 20.2's documentation and tracker
surfaces must be reconciled and the gate closed out: `UI.md` must record the new
setup-preview seat source, `GAME-EVIDENCE.md` must carry the fix receipt, and the
`specs/README.md` Gate 20.2 row plus the spec's own Status must flip to `Done`.

## Assumption Reassessment (2026-06-28)

1. The implementation surfaces this ticket documents already exist post-001/002:
   the Rust-owned active-seat-by-count catalog metadata
   (`crates/wasm-api/src/catalog.rs`) and the shell consumption
   (`apps/web/src/components/MatchSetup.tsx`). This ticket touches no code, tests,
   or deterministic artifacts.
2. The doc/tracker surfaces exist at their stated paths:
   `games/starbridge_crossing/docs/UI.md`,
   `games/starbridge_crossing/docs/GAME-EVIDENCE.md`, the Gate 20.2 row in
   `specs/README.md` (currently `Planned`, row 11.2), and the spec
   `specs/gate-20-2-starbridge-crossing-active-seat-setup-labels.md` (Status
   `Planned`). Spec §10 confirms the `specs/README.md` row already exists — only
   the `Done` flip remains — and that no renderer-list / smoke-list membership
   changes (game already listed).
3. Cross-artifact boundary under audit: the `specs/README.md` index and the spec's
   own Status header must move together to `Done`, gated on 001 and 002 having
   landed (exit criteria pass). This is a status-reconciliation capstone; it adds
   no production logic.

## Architecture Check

1. A single trailing docs-reconcile + `Done`-flip capstone is cleaner than
   co-locating the status flip with 001 or 002: the flip is gated on both prior
   tickets passing their exit criteria, so it belongs at the dependency leaf.
2. No backwards-compatibility aliasing/shims (docs/status only).
3. `engine-core` untouched; no `game-stdlib` change; no behavior moves to
   TypeScript (docs only).

## Verification Layers

1. Doc-link integrity after the edits -> `node scripts/check-doc-links.mjs`.
2. Doc/status content correctness -> grep-proofs: `UI.md` names the catalog
   active-seat metadata as the setup-preview source; `GAME-EVIDENCE.md` carries
   the Gate 20.2 receipt; `specs/README.md` row 11.2 and the spec Status both read
   `Done` (manual review of prose quality).

## What to Change

### 1. `games/starbridge_crossing/docs/UI.md`

Note that the setup-preview seat source is the Rust active-seat-by-count catalog
metadata (not a TypeScript-derived "first *N*" slice).

### 2. `games/starbridge_crossing/docs/GAME-EVIDENCE.md`

Add the Gate 20.2 fix receipt: the setup-preview active-seat mislabel for 2/3/4
seats, the additive catalog metadata fix, and the verifying commands
(`cargo test -p wasm-api`, the starbridge e2e smoke).

### 3. `specs/README.md` + spec Status

Flip the Gate 20.2 row (11.2) Status to `Done` with a one-line outcome, and set
the spec's own Header Status (`specs/gate-20-2-starbridge-crossing-active-seat-setup-labels.md`)
to `Done`.

## Files to Touch

- `games/starbridge_crossing/docs/UI.md` (modify)
- `games/starbridge_crossing/docs/GAME-EVIDENCE.md` (modify)
- `specs/README.md` (modify)
- `specs/gate-20-2-starbridge-crossing-active-seat-setup-labels.md` (modify)

## Out of Scope

- Any code, test, snapshot, or deterministic-artifact change (owned by
  GAT202STACROACT-001 / -002).
- Renderer-list / smoke-list membership changes (game already listed; spec §10).
- Web-shell catalog docs membership edits (no game added).

## Acceptance Criteria

### Tests That Must Pass

1. `node scripts/check-doc-links.mjs` (doc-link integrity after the edits).
2. `grep -q "Done" ` on the `specs/README.md` Gate 20.2 row and the spec Header
   Status (both read `Done`).
3. `cargo test --workspace` && `npm --prefix apps/web run smoke:e2e` (re-confirm
   001/002 green at closeout — exit criteria hold).

### Invariants

1. The spec Status and the `specs/README.md` index row are consistent (both
   `Done`) — no half-flipped tracker state.
2. Docs-only contract: no code/test/artifact surface is touched by this ticket.

## Test Plan

### New/Modified Tests

1. `None — docs-reconciliation + closeout capstone; it reconciles the named
   docs/status surfaces and re-runs the prior tickets' acceptance commands, adding
   no test file.`

### Commands

1. `node scripts/check-doc-links.mjs`.
2. `cargo test --workspace && npm --prefix apps/web run build && npm --prefix apps/web run smoke:e2e`.
3. A docs/status-only capstone has no narrower production-code boundary; the doc
   gates plus a re-run of the 001/002 acceptance commands are the correct
   verification surface.

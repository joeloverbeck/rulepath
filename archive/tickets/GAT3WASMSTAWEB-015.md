# GAT3WASMSTAWEB-015: Documentation + specs index/status flips (Gate 3 closeout)

**Status**: COMPLETED
**Priority**: HIGH
**Effort**: Medium
**Engine Changes**: None — documentation + spec/index status updates only; no Rust/crate/app-logic change.
**Deps**: 013, 014

## Problem

Gate 3 implementation must document the shell sufficiently for a later contributor
to understand and verify it, and flip the spec/index status once exit criteria pass
(spec §20; §21.8). This trailing cross-cutting docs ticket lands the WASM client
boundary doc, the local build/preview/smoke commands, replay import/export safety,
dev-panel data-source safety, the Race-to-N UI status update, and the
`specs/README.md` index + spec `Status` flips. It depends on the capstone smokes
(GAT3WASMSTAWEB-013/-014) so the docs describe a verified shell.

## Assumption Reassessment (2026-06-06)

1. Current state: `specs/README.md` Gate 3 row reads `WASM/static web shell — not
   yet specced | Not started` (line 29); the spec
   `specs/gate-3-wasm-static-web-shell.md` carries `Status: Planned` (Header) with a
   Sequencing/§20 instruction to flip to `Done` on exit; `apps/web` has no
   `README.md`; `games/race_to_n/docs/UI.md` reflects the Gate 1 harness;
   `progress.md` and root `README.md`/`docs/` exist. There is no WASM-client-boundary
   doc yet. (The reassess session already updated §20 to name the index flip and
   added the Sequencing section.)
2. Spec §20 required updates: `progress.md` (Gate 3 status + evidence);
   `specs/README.md` index pointer + status flip (Not started → Planned on adoption,
   → Done on exit); `apps/web` docs/root README with build/preview/smoke commands;
   document the WASM client boundary + operation groups (§9.7); replay import/export
   behavior + supported version(s) + safety limits; dev-panel data-source safety;
   `games/race_to_n/docs/UI.md` updated to the Gate 3 shell; record the raw-ABI-retention
   decision (ADR only if materially architecture-changing — not required here, §23.1).
   Docs must not invent future-game guarantees (Race-to-N only).
3. Cross-artifact boundary under audit: this is the §Ticket-shapes cross-cutting docs
   ticket. Per the decomposition pattern and spec §20 (which assigns the index flip
   to documentation updates), it carries the `specs/README.md` index `Planned`→`Done`
   flip and the spec `Status`→`Done` flip, gated on the capstone smokes
   (GAT3WASMSTAWEB-013/-014). `Deps: 013, 014` is the leaf set whose transitive deps
   cover every implementation surface the docs describe.

## Architecture Check

1. A single trailing docs ticket landing atomically after the capstone smokes
   prevents a staleness window where docs describe an unverified shell; co-locating
   each doc with its implementing ticket would risk status-line/index drift across
   the gate. The index/status flips belong here because they assert aggregate
   completion gated on exit evidence (§Ticket-shapes Done-flip default, with §20's
   index-flip-to-docs assignment applying).
2. No backwards-compatibility shims: docs are updated in place; no parallel doc
   authority is introduced (the WASM boundary doc is the single client-contract
   record).
3. `engine-core` untouched; documentation only; `game-stdlib` untouched.

## Verification Layers

1. Docs link integrity → simulation/CLI run: `node scripts/check-doc-links.mjs`
   passes with the new docs linked.
2. WASM boundary documented → manual review + grep-proof: the boundary doc names the
   operations, op groups, request/response shape, viewer-safe data, deferred ops, and
   the raw-ABI-retention rationale (§9.7).
3. Index/status flips correct → codebase grep-proof: `specs/README.md` Gate 3 row and
   the spec `Status` read `Done`; the row points to the spec (or its archive path per
   `docs/archival-workflow.md`).
4. No future-game guarantees → manual review: docs state Gate 3 is Race-to-N only.

## What to Change

### 1. New `docs/WASM-CLIENT-BOUNDARY.md`

Document the effective WASM client contract: which operations exist, their op group,
request/response shape at a high level, which data is viewer-safe, which ops are
deferred, and why the raw ABI is retained (§9.7, §23.1).

### 2. New `apps/web/README.md`

Local build/preview/smoke commands (`build:wasm`, `build`, `smoke:wasm`, `smoke:ui`,
`smoke:preview`, `smoke:e2e`), the static-serving target, and how to run the shell
locally with no backend.

### 3. `progress.md`, `games/race_to_n/docs/UI.md`, replay/dev-panel safety notes

Record Gate 3 status + verification evidence in `progress.md`; update
`games/race_to_n/docs/UI.md` to the Gate 3 shell reality; document replay
import/export behavior + supported version(s) + size/safety limits and dev-panel
data-source safety (in the boundary doc or `apps/web/README.md`).

### 4. `specs/README.md` + `specs/gate-3-wasm-static-web-shell.md` status flips

Flip the `specs/README.md` Gate 3 index row to `Done` (pointing to the spec / its
archive path per `docs/archival-workflow.md`) and the spec `Status` to `Done`, once
GAT3WASMSTAWEB-013/-014 pass.

## Files to Touch

- `docs/WASM-CLIENT-BOUNDARY.md` (new)
- `apps/web/README.md` (new)
- `progress.md` (modify) — Gate 3 status + evidence
- `games/race_to_n/docs/UI.md` (modify) — Gate 3 shell UI status
- `specs/README.md` (modify) — Gate 3 index row → `Done`
- `specs/gate-3-wasm-static-web-shell.md` (modify) — `Status` → `Done`

## Out of Scope

- Any implementation/behavior change (docs + status only).
- An ADR for raw-ABI retention (not required unless materially architecture-changing, §23.1).
- Archiving the spec/tickets, unless repo convention requires it now — follow `docs/archival-workflow.md` if so.

## Acceptance Criteria

### Tests That Must Pass

1. `node scripts/check-doc-links.mjs` — passes with the new/updated docs linked, no broken links.
2. `grep -n "Gate 3" specs/README.md` — the Gate 3 row reads `Done` and points to the spec.
3. `grep -nE "^- Status: Done|\\*\\*Status\\*\\*: Done|Status: Done" specs/gate-3-wasm-static-web-shell.md` — the spec `Status` is `Done`.

### Invariants

1. The WASM client boundary, build/preview/smoke commands, replay import/export safety, and dev-panel data-source safety are documented in exactly one authoritative place each.
2. Docs state Gate 3 is Race-to-N only and invent no future-game guarantees; the index/status flips land only after the capstone smokes pass.

## Test Plan

### New/Modified Tests

1. `None — documentation/closeout ticket; verification is command-based (`check-doc-links.mjs` + status greps) and the capstone smokes named in Deps (GAT3WASMSTAWEB-013/-014).`

### Commands

1. `node scripts/check-doc-links.mjs`
2. `grep -n "Gate 3" specs/README.md`
3. A docs-link + status-grep boundary is correct here: behavior is verified by the upstream tickets; this ticket only ships prose + status flips.

## Outcome

Completed on 2026-06-06.

Changes:

- Added `docs/WASM-CLIENT-BOUNDARY.md` as the Gate 3 WASM client contract, operation-group, replay-safety, and developer-panel whitelist record.
- Added `apps/web/README.md` with build, preview, smoke, static-serving, and system-Chrome E2E notes.
- Updated `progress.md`, `docs/README.md`, and `games/race_to_n/docs/UI.md` for the Gate 3 shell.
- Flipped `specs/README.md` Gate 3 to `Done` and `specs/gate-3-wasm-static-web-shell.md` status to `Done`.

Deviations:

- None.

Verification:

- `node scripts/check-doc-links.mjs`
- `grep -n "Gate 3" specs/README.md`
- `grep -nE "^- Status: Done|\\*\\*Status\\*\\*: Done|Status: Done" specs/gate-3-wasm-static-web-shell.md`

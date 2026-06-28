# WEBSHEREP-004: Document replay import policy; flip spec Status → Done

**Status**: COMPLETED
**Priority**: LOW
**Effort**: Small
**Engine Changes**: None — docs + spec/index status only
**Deps**: WEBSHEREP-001, WEBSHEREP-002, WEBSHEREP-003

## Problem

The replay import-size bound was an undocumented constant, and the docs that mention it are now stale and were never fully correct: `docs/WASM-CLIENT-BOUNDARY.md:88` and `apps/web/README.md:87` both state "Replay import is capped at 128 KiB in the UI", which (a) is the value being raised and (b) wrongly attributes the cap to the UI when the authority is the Rust importer. Per spec §10, the import policy and the export/import round-trip guarantee must be documented, naming `MAX_REPLAY_IMPORT_BYTES` as the single source of truth. This closeout also flips the spec Status and its `specs/README.md` index row to `Done`.

## Assumption Reassessment (2026-06-28)

1. The stale claims live at `docs/WASM-CLIENT-BOUNDARY.md:88` (§Replay Safety) and `apps/web/README.md:87`, both reading "capped at … 128 KiB in the UI before Rust parsing". The authoritative bound is `MAX_REPLAY_IMPORT_BYTES` in `crates/wasm-api/src/constants.rs` (raised by WEBSHEREP-001). No other doc references the 128 KiB figure (grep-verified).
2. Spec `specs/web-shell-replay-import-size-roundtrip.md` §10 names `docs/WASM-CLIENT-BOUNDARY.md` / `apps/web/README.md` as the doc home and requires naming the Rust bound as source of truth; §Status is currently `Planned`. The `specs/README.md:115` tracker row exists (Status `Planned`) and §10 calls for the closeout `Done` flip.
3. Shared boundary under audit: the WASM client surface doc (`docs/WASM-CLIENT-BOUNDARY.md` §Replay Safety) and the web-shell README replay bullet — both must describe the Rust importer as the authoritative size guard and the shell as deferring to it (no UI-side cap).
4. FOUNDATIONS §2/§9: the corrected docs must state that validation authority (including size) lives in Rust and that the local replay import/export round-trip is restored for full-length games — without implying TypeScript owns any acceptance decision.

## Architecture Check

1. A single trailing docs + status-flip ticket is cleaner than co-locating doc edits with each implementation ticket: the round-trip guarantee can only be stated coherently once the bound, the TS deferral, and the regression smoke have all landed (it `Deps` all three). The `Done` flip is gated on exit evidence passing, so it belongs on this closeout.
2. No backwards-compatibility shim: stale claims are corrected in place.
3. No code/engine surface; docs and status markers only.

## Verification Layers

1. Docs name Rust as the authoritative bound -> codebase grep-proof: `docs/WASM-CLIENT-BOUNDARY.md` and `apps/web/README.md` reference `MAX_REPLAY_IMPORT_BYTES` and no longer claim a UI-side 128 KiB cap.
2. Spec + index flipped to Done -> grep-proof: spec `**Status**` reads `Done` and `specs/README.md` row for this spec reads `Done`.
3. Doc links/anchors intact -> `node scripts/check-doc-links.mjs` green.

## What to Change

### 1. Correct the import-policy docs

In `docs/WASM-CLIENT-BOUNDARY.md` §Replay Safety, replace the "capped in the UI at 128 KiB" statement with: the authoritative import-size guard is `import_replay` in Rust (`MAX_REPLAY_IMPORT_BYTES`), the shell defers to it (no stricter UI cap), and the documented export→import round-trip is guaranteed for full-length games up to that bound. Update `apps/web/README.md:87` to match (the UI imposes no cap stricter than the Rust bound), recording the derivation rule reference.

### 2. Flip Status to Done

Set `**Status**` in `specs/web-shell-replay-import-size-roundtrip.md` to `Done` and update the `specs/README.md:115` tracker row Status to `Done` with a one-line closeout note. If `games/starbridge_crossing/docs/UI.md` references replay controls, reconcile only as needed (no behavior change).

## Files to Touch

- `docs/WASM-CLIENT-BOUNDARY.md` (modify)
- `apps/web/README.md` (modify)
- `specs/README.md` (modify)
- `specs/web-shell-replay-import-size-roundtrip.md` (modify)

## Out of Scope

- All production code and tests (WEBSHEREP-001/002/003) — this ticket only documents and reconciles status.
- Any replay/trace schema or behavior change.

## Acceptance Criteria

### Tests That Must Pass

1. `node scripts/check-doc-links.mjs` green.
2. `grep -n "128 KiB" docs/WASM-CLIENT-BOUNDARY.md apps/web/README.md` shows no surviving "capped in the UI at 128 KiB" claim (the authoritative-bound wording replaces it).
3. `node scripts/check-catalog-docs.mjs` green (README catalog surfaces unaffected).

### Invariants

1. The import-size bound is documented as Rust-authoritative (`MAX_REPLAY_IMPORT_BYTES`), not a UI constant.
2. Spec and index Status both read `Done` only after WEBSHEREP-001/002/003 land.

## Test Plan

### New/Modified Tests

1. `None — documentation + status-reconciliation closeout; verification is the doc-gate commands below and existing pipeline coverage named in WEBSHEREP-001/003.`

### Commands

1. `node scripts/check-doc-links.mjs`
2. `node scripts/check-catalog-docs.mjs`
3. `grep -rn "MAX_REPLAY_IMPORT_BYTES" docs/WASM-CLIENT-BOUNDARY.md apps/web/README.md` (expect the authoritative-bound reference present)

## Outcome

Completed: 2026-06-28

Updated `docs/WASM-CLIENT-BOUNDARY.md` and `apps/web/README.md` to describe
`MAX_REPLAY_IMPORT_BYTES` in `crates/wasm-api` as the authoritative Rust/WASM
replay import-size guard. Both docs now state that the shell imposes no stricter
UI cap, delegates oversize rejection to Rust, and preserves import of the
catalog's own full-length exports including Starbridge Crossing 6-seat 2000-ply
documents. Flipped `specs/web-shell-replay-import-size-roundtrip.md` to `Done`,
added its `## Outcome` evidence ledger, and updated the `specs/README.md`
tracker row to `Done`.

Deviation from plan: the live spec retains historical `128 KiB` mentions as
defect evidence and reassessment rationale. The stale-doc acceptance grep was
therefore applied to the authoritative docs named by the ticket
(`docs/WASM-CLIENT-BOUNDARY.md` and `apps/web/README.md`), where the UI-side cap
claim is removed.

Verification:

- `node scripts/check-doc-links.mjs` passed (`Checked 31 markdown files`).
- `node scripts/check-catalog-docs.mjs` passed (`catalog-docs check passed — 20 games reflected in intro, root, and smoke surfaces`).
- `rg -n "128 KiB|capped.*UI" docs/WASM-CLIENT-BOUNDARY.md apps/web/README.md || true` produced no stale-cap matches.
- `rg -n "MAX_REPLAY_IMPORT_BYTES" docs/WASM-CLIENT-BOUNDARY.md apps/web/README.md` found both authoritative-bound references.

# WEBSHEREP-002: Remove TS import-size shadow; gate render-time replay parse

**Status**: COMPLETED
**Priority**: HIGH
**Effort**: Small
**Engine Changes**: Yes (presentation-only) — `apps/web/src/components/ReplayImportExport.tsx`
**Deps**: WEBSHEREP-001

## Problem

`ReplayImportExport.tsx` carries a TypeScript pre-check, `MAX_IMPORT_CHARS = 128 * 1024` (line 10), that rejects an import with `replay_too_large` (line 25) **before** calling WASM. This is a stricter shadow of the authoritative Rust guard (`import_replay`) and an acceptance decision TypeScript should not own (FOUNDATIONS §2: TS presentation-only; §11: validation authority in Rust). With WEBSHEREP-001 raising the Rust bound, this shadow is the remaining barrier to the round-trip. Separately, `replayCommandSummary(documentText)` runs unmemoized on every render (line 15), so a large pasted document is `JSON.parse`-d on every keystroke — a responsiveness footgun the import-click guard never covered.

## Assumption Reassessment (2026-06-28)

1. `apps/web/src/components/ReplayImportExport.tsx`: `MAX_IMPORT_CHARS` is defined at line 10 and used only at line 25 (verified repo-wide: the symbol appears in no other file). `importReplay` (line 23) returns at line 31 before `onImport(normalizeJsonDocument(documentText))` (line 33). `replayCommandSummary(documentText)` is called unmemoized in the render body at line 15. The component is wired once in `apps/web/src/main.tsx:856` (shared across all catalog games).
2. Spec `specs/web-shell-replay-import-size-roundtrip.md` §4 (Deliverable 2) and §5 select "remove the TS hard reject; defer to the Rust `replay_too_large` diagnostic, optionally a soft non-blocking notice" + "memoize/length-gate the on-render summary parse". The spec's Not-allowed carve-out (§3) sanctions removing the TS guard once reassessment proves the Rust importer bounds the risk — which WEBSHEREP-001 does.
3. Shared boundary under audit: the WASM client surface. On import, `onImport` → `apps/web/src/wasm/client.ts` `importReplay(doc)` → `rulepath_import_replay` (`client.ts:73,1987`). The component already surfaces `ApiError` diagnostics from the `onImport` catch (line 34-36), so removing the pre-check lets the Rust `replay_too_large` diagnostic surface naturally for a genuinely oversize document.
4. FOUNDATIONS §2 motivates this ticket: removing a TypeScript-side accept/reject decision keeps TS presentation-only and leaves size validation solely with Rust. This ticket adds no legality/validation authority to TS; it removes one.

## Architecture Check

1. Deleting the pre-check and delegating to WASM is cleaner than raising `MAX_IMPORT_CHARS` to match the Rust bound: a single authoritative guard (Rust) replaces two divergent caps and removes the §2 smell. An optional soft non-blocking notice (no reject) is presentation-only and acceptable.
2. No backwards-compatibility shim or alias path: the constant and its branch are removed outright, not retained behind a flag.
3. No `engine-core` / `game-stdlib` involvement (web shell only). No legality decided in TypeScript.

## Verification Layers

1. TS no longer makes a size accept/reject decision (§2) -> codebase grep-proof: `MAX_IMPORT_CHARS` and the `replay_too_large` literal are gone from `ReplayImportExport.tsx`.
2. Render-time parse is bounded for large documents -> manual review + `smoke:ui`: `replayCommandSummary` is memoized (`useMemo` on `documentText`) and/or length-gated so a large paste is not re-parsed every render.
3. Shell still builds and renders the replay panel -> `npm --prefix apps/web run build` and `npm --prefix apps/web run smoke:ui` green.

## What to Change

### 1. Remove the import-size shadow

Delete `MAX_IMPORT_CHARS` (line 10) and the `if (documentText.length > MAX_IMPORT_CHARS) { … return; }` block (lines 25-31) in `importReplay`. The handler proceeds directly to `onImport(normalizeJsonDocument(documentText))`; a genuinely oversize document now surfaces the Rust `replay_too_large` diagnostic via the existing catch. Optionally render a non-blocking "large replay" hint based on `documentText.length` that never blocks import.

### 2. Bound the render-time summary parse

Memoize `replayCommandSummary(documentText)` with `useMemo` keyed on `documentText` (and/or skip the parse above a generous length threshold, showing a "summary hidden for large document" note), so pasting a large document does not re-`JSON.parse` it on every render.

## Files to Touch

- `apps/web/src/components/ReplayImportExport.tsx` (modify)

## Out of Scope

- The Rust bound (`MAX_REPLAY_IMPORT_BYTES`) — WEBSHEREP-001.
- The e2e round-trip smoke (WEBSHEREP-003) and docs (WEBSHEREP-004).
- Any structural/shape validation of the document in TypeScript (Rust remains the validator).

## Acceptance Criteria

### Tests That Must Pass

1. `npm --prefix apps/web run build` (incl. `tsc --noEmit`) green.
2. `npm --prefix apps/web run smoke:ui` green.
3. End-to-end round-trip proof lands in WEBSHEREP-003 (this ticket's behavior is exercised there).

### Invariants

1. TypeScript decides no replay legality, validity, or size acceptance — the import path delegates to `rulepath_import_replay`.
2. The replay panel imposes no hard import cap stricter than the Rust bound.

## Test Plan

### New/Modified Tests

1. `None — production component change; the full round-trip regression is added in WEBSHEREP-003 (e2e smoke) and the Rust round-trip test in WEBSHEREP-001. Build + smoke:ui are the local guards named in Acceptance Criteria.`

### Commands

1. `npm --prefix apps/web run build`
2. `npm --prefix apps/web run smoke:ui`
3. `grep -n "MAX_IMPORT_CHARS\|replay_too_large" apps/web/src/components/ReplayImportExport.tsx` (expect no matches — narrower than a full e2e run because the deletion is the verification boundary; behavior is exercised in WEBSHEREP-003)

## Outcome

Completed: 2026-06-28

Removed the TypeScript-side import-size hard reject from
`ReplayImportExport.tsx`, so replay import now normalizes the text and delegates
acceptance or `replay_too_large` diagnostics to the Rust/WASM importer. Replaced
the removed `MAX_IMPORT_CHARS` gate with a presentation-only
`MAX_COMMAND_SUMMARY_CHARS` threshold that skips command-summary parsing for
large documents, and memoized the command summary by `documentText`.

Deviation from plan: no soft warning was added; the component simply hides the
command summary for large documents. This avoids adding a second import policy
message while still preventing repeated render-time parsing of full-length
replay text. The import path remains unblocked and Rust-authoritative.

Verification:

- `rg -n "MAX_IMPORT_CHARS|replay_too_large" apps/web/src/components/ReplayImportExport.tsx` returned no matches.
- `npm --prefix apps/web run build` passed.
- `npm --prefix apps/web run smoke:ui` passed.

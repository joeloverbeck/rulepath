# PHA0NEXPHAFOU-008: WASM-CLIENT-BOUNDARY conceptual refresh for multi-seat view projection

**Status**: PENDING
**Priority**: MEDIUM
**Effort**: Medium
**Engine Changes**: None — `docs/WASM-CLIENT-BOUNDARY.md` edit only; no `crates/wasm-api` or `apps/web` code, no exported-API schema change.
**Deps**: PHA0NEXPHAFOU-002

## Problem

`WASM-CLIENT-BOUNDARY.md` is stale toward early `race_to_n` and uses singular `active_seat` assumptions. It does not explain multi-seat view projection, public observer vs. seat viewer, or hotseat seat switching for 3+ seats — so the next ladder has no conceptual boundary doc to anchor multi-seat WASM operations.

## Assumption Reassessment (2026-06-13)

1. The Rust↔browser bridge is `crates/wasm-api/src/lib.rs`; the TS client is `apps/web/src/wasm/client.ts`. This ticket describes the conceptual boundary only — the implementer confirms those surfaces exist and makes **no** exported-API schema change.
2. Docs: `docs/WASM-CLIENT-BOUNDARY.md` (operation groups, active seat, replay export, dev-panel whitelist). `docs/MULTI-SEAT-AND-SURFACE-CONTRACT.md` (PHA0NEXPHAFOU-002) is the cross-reference target; `apps/web` and `crates/wasm-api` are the described surfaces.
3. Cross-artifact boundary under audit: the boundary doc vs. the actual `wasm-api`/web surfaces; shared surface = per-viewer view projection and active/pending seats.
4. FOUNDATIONS principle restate: §2 (Rust owns behavior; TypeScript presents) and §11 (browser payloads are viewer-safe). The refresh is meaning-preserving and conceptual — no exported-API schema change.
5. Enforcement surface: §2 behavior authority and §11 viewer-safe payloads. The refreshed operation list describes conceptual operations and the "TS displays but never computes" rule; it introduces no leakage path and is enforced by the no-leak browser smoke and `wasm-api` tests.

## Architecture Check

1. A current catalog-boundary doc is cleaner and less misleading than a Gate-3-only stale doc; it gives the next ladder a place to anchor multi-seat projection.
2. No backwards-compatibility aliasing/shims introduced.
3. No incompatible exported-API schema change (that would require an ADR per §13); `engine-core` is untouched and stays noun-free.

## Verification Layers

1. Doc refreshed to the current catalog boundary → manual review.
2. Multi-seat operations + `active_seats`/pending responders described → manual review.
3. No exported-API schema change → codebase grep-proof (no `crates/wasm-api` edit; `git diff --stat crates/wasm-api` empty).
4. Links resolve → `node scripts/check-doc-links.mjs`.

## What to Change

### 1. `docs/WASM-CLIENT-BOUNDARY.md`

Rewrite as the current catalog boundary, not Gate-3-only. Add multi-seat operations conceptually: `new_match(game_id, seed, seats, options)`, `get_view(viewer)`, `get_action_tree(actor/viewer)`, `submit_action(actor, path)`, `run_bot_turn(bot_seat)`, `export_replay(viewer_scope)`. Clarify that `active_seat` may become `active_seats` / phase-owned pending responders per game view, and that TypeScript displays but never computes them.

## Files to Touch

- `docs/WASM-CLIENT-BOUNDARY.md` (modify)

## Out of Scope

- Editing `crates/wasm-api` or `apps/web` code (Infra A/C and Gate 15+ own that).
- Changing the exported-API JSON schema incompatibly (would require an ADR).
- Letting TypeScript decide legality or compute turn order.

## Acceptance Criteria

### Tests That Must Pass

1. `docs/WASM-CLIENT-BOUNDARY.md` reads as the current catalog boundary and describes the multi-seat operation set plus `active_seats`/pending responders.
2. No `crates/wasm-api` or `apps/web` file is modified by this ticket.
3. `node scripts/check-doc-links.mjs` passes.

### Invariants

1. §2 behavior authority is preserved — the doc states TS displays but never computes.
2. The doc *describes* the exported API; it introduces no incompatible schema change.

## Test Plan

### New/Modified Tests

1. `None — documentation-only ticket; verification is command-based and existing pipeline/test coverage is named in Assumption Reassessment.`

### Commands

1. `node scripts/check-doc-links.mjs`
2. `grep -niE "get_view|active_seats|export_replay|displays but never computes" docs/WASM-CLIENT-BOUNDARY.md`
3. `git diff --stat crates/wasm-api apps/web` (expect no changes from this ticket)

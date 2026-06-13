# PHA0NEXPHAFOU-002: Add the multi-seat & larger-surface contract doc and index it in docs/README.md

**Status**: PENDING
**Priority**: HIGH
**Effort**: Large
**Engine Changes**: None — new `docs/MULTI-SEAT-AND-SURFACE-CONTRACT.md` + `docs/README.md` map edit; no crates/schemas/traces.
**Deps**: PHA0NEXPHAFOU-001

## Problem

The public scaling phase needs the N-seat and larger-surface obligations stated once, so every Gate 15+ spec does not re-litigate seat ranges, turn order, pairwise hidden-information no-leak, surface budgets, and per-seat outcome rationale. No such doc exists, and `docs/README.md`'s authority map has no place for an N-seat/larger-surface area contract.

## Assumption Reassessment (2026-06-13)

1. The kernel is already seat-generic: `crates/engine-core/src/game.rs:16` `setup(seats: &[SeatId])`; `crates/engine-core/src/lib.rs:46` `VisibilityScope::PrivateToSeat(SeatId)`; `lib.rs:54` `Viewer` (optional `seat_id`); `crates/engine-core/src/replay.rs:137` `seats: Vec<SeatAssignment>`. The contract codifies these facts; it MUST add no noun to `engine-core`.
2. Docs/specs: `docs/README.md` carries the document map; `docs/FOUNDATIONS.md`, `docs/ARCHITECTURE.md`, `docs/ENGINE-GAME-DATA-BOUNDARY.md` govern; `docs/adr/0004-hidden-info-replay-export-taxonomy.md` governs viewer-scoped export no-leak.
3. Cross-artifact boundary under audit: `docs/README.md` authority map. The new doc is subordinate to the constitution, architecture, data boundary, hidden-info ADRs, and bot law — it adds no new top-level authority.
4. FOUNDATIONS principle: §3 noun-free kernel; §11 invariants apply to *any positive seat count*; §5 static-data-is-not-behavior (topology is typed content, not behavior). The doc codifies these for N-seat contexts; it changes no principle's meaning.
5. Enforcement surfaces named: the doc states the pairwise N-seat no-leak obligation (§11 no-leak firewall) and trace/view-hash expectations (deterministic replay/hash, §11/§13). It MUST reference — not redefine — ADR 0004 and `docs/TRACE-SCHEMA-v1.md`, and introduces no leakage or nondeterminism path. Actual enforcement lands later in the Infra D no-leak harness and the per-game Gate 15+ gates.

## Architecture Check

1. One area doc codifying existing principles is cleaner and less drift-prone than restating N-seat obligations inside every Gate 15+ spec.
2. No backwards-compatibility aliasing/shims introduced.
3. `engine-core` stays noun-free: the contract explicitly keeps seat-enum/poker/map/topology nouns game-local or `game-stdlib`-via-atlas (§3/§4); it adds nothing to the kernel.

## Verification Layers

1. Doc exists and is indexed → codebase grep-proof (`docs/README.md` links the new doc before ROADMAP).
2. No `engine-core` noun introduced → `bash scripts/boundary-check.sh` + grep confirms no kernel file changed.
3. No trace-schema/kernel-boundary change → grep-proof that `docs/TRACE-SCHEMA-v1.md` is untouched + FOUNDATIONS alignment check (§3/§11/§12).
4. Links resolve → `node scripts/check-doc-links.mjs`.

## What to Change

### 1. Create `docs/MULTI-SEAT-AND-SURFACE-CONTRACT.md`

Cover, codification-only, cross-referencing the governing docs rather than redefining them: seat-range declaration; stable seat IDs and optional roles/teams; turn-order model (active seat / active set / pending responders are Rust-owned); viewer matrix; pairwise hidden-information no-leak matrix (seat A never reads seat B's private payload); public-observer rules; topology/object-count budget; action-tree fanout budget; semantic-effect batching; per-seat final-breakdown requirement; trace/view-hash expectations for the public observer and every authorized seat viewer; simulator-summary format for arbitrary winners/splits/teams. State explicitly that it is subordinate to `FOUNDATIONS.md`, `ARCHITECTURE.md`, `ENGINE-GAME-DATA-BOUNDARY.md`, ADR 0004, and the bot law, and that "large map is not a DSL license."

### 2. Index it in `docs/README.md`

Add the doc to the document map under the foundation/area-doc layer, before `ROADMAP.md`, marked subordinate to the constitution, architecture, data boundary, hidden-info ADRs, and bot law.

## Files to Touch

- `docs/MULTI-SEAT-AND-SURFACE-CONTRACT.md` (new)
- `docs/README.md` (modify)

## Out of Scope

- Changing trace-schema semantics, the WASM exported-API schema, or any kernel boundary (each requires its own ADR).
- Adding any mechanic noun to `engine-core`; promoting any `game-stdlib` helper.
- Editing the foundation docs' own bodies — PHA0NEXPHAFOU-003+ clarify those; this ticket only creates the new doc and the map entry.

## Acceptance Criteria

### Tests That Must Pass

1. `docs/MULTI-SEAT-AND-SURFACE-CONTRACT.md` exists and covers the required content areas listed in What to Change §1.
2. `docs/README.md` links the new doc in the document map before the ROADMAP entry.
3. `node scripts/check-doc-links.mjs` passes and `bash scripts/boundary-check.sh` passes.

### Invariants

1. The new doc adds no normative rule that supersedes a FOUNDATIONS principle — it codifies and cross-references the governing docs.
2. `engine-core` gains no mechanic noun; `docs/TRACE-SCHEMA-v1.md` fields/version are unchanged by this ticket.

## Test Plan

### New/Modified Tests

1. `None — documentation-only ticket; verification is command-based and existing pipeline/test coverage is named in Assumption Reassessment.`

### Commands

1. `node scripts/check-doc-links.mjs`
2. `bash scripts/boundary-check.sh`
3. `grep -n "MULTI-SEAT-AND-SURFACE-CONTRACT" docs/README.md`

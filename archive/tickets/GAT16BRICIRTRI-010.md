# GAT16BRICIRTRI-010: Replay support, serialization, and golden-trace pack

**Status**: COMPLETED
**Priority**: HIGH
**Effort**: Large
**Engine Changes**: Yes (deterministic evidence) — `games/briar_circuit/src/replay_support.rs`, `tests/serialization.rs`, `tests/replay.rs`, `tests/golden_traces/*`
**Deps**: 008, 009

## Problem

Briar Circuit needs deterministic replay: an internal full trace reproducing state/effect/action/view hashes, viewer-scoped public and seat-private observation exports/imports (under ADR 0004), canonical serialization with strict unknown-field rejection, and the §7.6 golden-trace minimum set. This ticket adds the public-export redaction layer and the behavioral trace pack that `replay-check` validates.

## Assumption Reassessment (2026-06-20)

1. `engine-core`'s replay recorder (`crates/engine-core/src/lib.rs` — `ReplayRecord`/`Checkpoint`/`HashSurface`/`StableSerialize`/`UnknownFieldPolicy`) already exists; modules may already emit golden traces. This ticket fills `replay_support.rs` (the public-export redaction layer) and consolidates the behavioral trace pack. The no-leak traces were authored in GAT16BRICIRTRI-009.
2. `specs/gate-16-briar-circuit-trick-taking.md` §4.2 (Replay/Views rows), §7.6 (golden-trace minimum set, 37 traces), Appendix A `BC-REPLAY-001/002`/`BC-VIS-003`, and the `docs/adr/0004-hidden-info-replay-export-taxonomy.md` taxonomy fix the contract. Trace Schema v1; no schema migration authorized.
3. Cross-artifact boundary under audit: golden traces record `game_id = "briar_circuit"`, `rules_version = "briar-circuit-rules-v1"`, four-seat ordering, all four private-view hashes, and migration notes; they are the contract `replay-check` validates after GAT16BRICIRTRI-012 registration. Serialization order must be canonical and stable.
4. FOUNDATIONS §11/§2 determinism + §13 (replay/hash semantics) are under audit: internal seed+command replay reproduces identical hashes; viewer-scoped exports carry only authorized observation timelines (no seed/deck reconstruction). This uses existing Trace Schema v1 — no hash-semantic change, no ADR trigger.
5. Schema-extension check: the golden traces and viewer-export records extend the existing Trace Schema v1 / ADR-0004 export taxonomy **additively** (new `briar_circuit` instances of existing fields; a game-local viewer-export version anchor), not a breaking schema change; consumers (`replay-check`, the trace viewer) read the same v1 shape.

## Architecture Check

1. Keeping the omniscient internal trace separate from viewer-scoped exports (the ADR-0004 taxonomy) localizes redaction in `replay_support.rs`; the internal trace stays a test/dev artifact never used as a browser export.
2. No backwards-compatibility aliasing/shims — fills the `replay_support.rs` stub; reuses engine-core's recorder.
3. `engine-core` untouched (§3); no `game-stdlib` change (§4); serialization rides the generic `StableSerialize`/`UnknownFieldPolicy` contracts.

## Verification Layers

1. Internal replay reproduces state/effect/action/view hashes -> `tests/replay.rs` (`BC-REPLAY-001`) + golden traces.
2. Public and seat-private exports reproduce only authorized timelines; no seed/deck leak -> `tests/replay.rs` + `tests/visibility.rs` export scan (`BC-REPLAY-002`/`BC-VIS-003`).
3. Canonical serialization order; strict unknown-field rejection; version anchors -> `tests/serialization.rs`.
4. The §7.6 minimum trace set exists with v1 fields + migration notes -> `replay-check --game briar_circuit --all` (after 012) + trace-count assertion.

## What to Change

### 1. `games/briar_circuit/src/replay_support.rs`

Internal full deterministic trace support plus viewer-scoped observation export/import (public default + explicitly-labelled seat-private classes) per ADR 0004; the public export carries no seed/private-command/deck-order path.

### 2. Serialization

Canonical map/list/card/seat ordering, strict unknown-field rejection, and version anchors in `tests/serialization.rs`.

### 3. Golden-trace pack (`tests/golden_traces/*.trace.json`)

The §7.6 behavioral minimum set (setup/deal, pass directions, play rules, scoring/moon, threshold/tie, diagnostics, replay export/import) — the no-leak observer/pairwise traces already landed in GAT16BRICIRTRI-009.

## Files to Touch

- `games/briar_circuit/src/replay_support.rs` (modify; created by 004)
- `games/briar_circuit/tests/replay.rs` (modify; created by 005)
- `games/briar_circuit/tests/serialization.rs` (modify; created by 004)
- `games/briar_circuit/tests/golden_traces/*.trace.json` (new — §7.6 behavioral set)

## Out of Scope

- The `replay-check`/`fixture-check`/`rule-coverage` tool registrations (GAT16BRICIRTRI-012) — this ticket authors the traces those tools validate.
- WASM replay classes and browser replay import/export (GAT16BRICIRTRI-013/015).
- Any Trace Schema v2 or hash-semantic change (forbidden absent an ADR).

## Acceptance Criteria

### Tests That Must Pass

1. `cargo test -p briar_circuit --test replay --test serialization` — deterministic hashes, canonical order, unknown-field rejection.
2. `cargo test -p briar_circuit` — full crate green.
3. `cargo run -p replay-check -- --game briar_circuit --all` (after GAT16BRICIRTRI-012) — every §7.6 trace validates; deferred check noted.

### Invariants

1. Identical seed+command reproduces identical state/effect/action/view hashes (§2/§11).
2. Viewer-scoped exports contain no seed/deck fact reconstructing private hands (§11 no-leak; ADR 0004).

## Test Plan

### New/Modified Tests

1. `games/briar_circuit/tests/replay.rs` — internal replay-hash reproduction + viewer-scoped export/import round-trip.
2. `games/briar_circuit/tests/serialization.rs` — canonical ordering + unknown-field rejection.
3. `games/briar_circuit/tests/golden_traces/*.trace.json` — §7.6 behavioral trace set.

### Commands

1. `cargo test -p briar_circuit --test replay --test serialization`
2. `cargo test -p briar_circuit`
3. `replay-check --game briar_circuit --all` is the cross-cutting validator but is exercised once the tool arm lands (GAT16BRICIRTRI-012); native replay/serialization tests are the correct boundary here.

## Outcome

Completed on 2026-06-21. Added game-local replay hash snapshots for state,
view, action-preview, and effect surfaces; viewer-scoped replay exports/imports
with ADR-style public and seat-private classes; strict export header parsing
with version anchors; and the full §7.6 37-file golden trace inventory. The
no-leak traces from GAT16BRICIRTRI-009 were normalized to include schema,
rules-version, and migration-note anchors.

Deferred verification:

1. `cargo run -p replay-check -- --game briar_circuit --all` is deferred until
   GAT16BRICIRTRI-012 registers Briar Circuit with replay-check.

Verification:

1. `cargo fmt --all --check`
2. `cargo test -p briar_circuit --test replay --test serialization`
3. `cargo test -p briar_circuit`

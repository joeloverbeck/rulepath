# GAT17VOWTIDOHHEL-011: Replay support, serialization, viewer-scoped exports, golden-trace pack

**Status**: PENDING
**Priority**: HIGH
**Effort**: Large
**Engine Changes**: Yes — new `games/vow_tide/src/replay_support.rs`; new `games/vow_tide/tests/{replay,serialization}.rs`; consolidated golden-trace pack
**Deps**: 009, 010

## Problem

Provide deterministic internal replay plus viewer-scoped observation exports (public and seat-private) under ADR 0004, stable serialization for every state/view/effect/action/trace, and the consolidated golden-trace pack (Trace Schema v1). No retroactive private reveal; no schema migration.

## Assumption Reassessment (2026-06-21)

1. `engine-core`'s replay recorder already exists (sibling games author traces before their own `replay_support.rs`); `games/vow_tide/src/visibility.rs` (010) supplies the viewer projections this export layer redacts through. The earlier tickets (006/007/008/009/010) already authored their module-specific traces; this ticket adds the replay/export-specific traces and the cross-cutting serialization round-trips.
2. Spec §7.6 enumerates the ~50-trace minimum (`game_id="vow_tide"`, `rules_version="vow-tide-rules-v1"`, Trace Schema v1, per-seat private-view hashes); ADR 0004 governs the viewer-scoped export taxonomy; §3.3/Forbidden-changes bar exporting an internal full trace to the browser.
3. Cross-artifact boundary under audit: the viewer-scoped export/import is the contract `replay-check` (016) and the WASM replay op (017) consume; the internal full trace stays native/test-only.
4. FOUNDATIONS §11 (deterministic replay/hash, no-leak) is the principle under audit: same setup+commands reproduce state/effect/action-tree/observer/every-seat views; export preserves only authorized history.
5. §11/§13 enforcement surface: no Trace Schema v2 / hash-semantics change (would need an ADR); replay scrubbing never projects final-state knowledge backward; serialization order is stable (sorted/insertion-ordered, no incidental map iteration).

## Architecture Check

1. Layering viewer-scoped export on top of the 010 projections (rather than re-deriving redaction) keeps one no-leak authority and reuses the existing engine replay recorder.
2. No shims; new replay/serialization modules + tests.
3. `engine-core` untouched; no `game-stdlib` change; Trace Schema v1 unchanged.

## Verification Layers

1. Same setup+commands reproduce all hashes (state/effect/action-tree/observer/every-seat) → `cargo test -p vow_tide --test replay`.
2. Viewer-scoped export/import preserves only authorized history; no retroactive reveal → replay no-leak tests + export/import traces.
3. Stable serialization + version fields for state/views/effects/actions/exports → `cargo test -p vow_tide --test serialization`.
4. Determinism across the pack → `cargo run -p replay-check -- --game vow_tide --all` (after 016 registers the tool arm).

## What to Change

### 1. `replay_support.rs`

Internal full deterministic trace (native/test authority) + viewer-scoped public and per-seat observation export/import under ADR 0004; redact through the 010 projections; never export raw state/seed/stock/full-trace.

### 2. Serialization round-trips

`tests/serialization.rs`: stable JSON ordering + round trip for state, public/private views, effects, actions, bot outputs (after 012), fixtures, internal traces, viewer exports; versions present; unknown/newer behavior explicit.

### 3. Consolidated golden-trace pack

Author the remaining §7.6 traces not owned by an earlier module (replay export/import, schedule, diagnostics, wasm-exported hook-to-terminal, the back-port preservation manifest), all with per-seat private-view hashes.

## Files to Touch

- `games/vow_tide/src/replay_support.rs` (new)
- `games/vow_tide/tests/replay.rs` (new)
- `games/vow_tide/tests/serialization.rs` (new)
- `games/vow_tide/tests/golden_traces/public-replay-export-import.trace.json` (new)
- `games/vow_tide/tests/golden_traces/seat-private-replay-export-import-all-viewers.trace.json` (new)

## Out of Scope

- `replay-check` tool registration (016) and WASM replay op (017) — they consume this.
- Any Trace Schema v2 / hash-semantics change.

## Acceptance Criteria

### Tests That Must Pass

1. `cargo test -p vow_tide --test replay` — deterministic reproduction of all hashes.
2. `cargo test -p vow_tide --test serialization` — stable ordering + round-trip.
3. `cargo test -p vow_tide` — full crate suite green.

### Invariants

1. No export exposes another seat's hand, the hidden stock, the seed, or the internal full trace.
2. Identical inputs + versions produce byte-identical traces/hashes; no schema migration.

## Test Plan

### New/Modified Tests

1. `games/vow_tide/tests/replay.rs` — internal + viewer-scoped export/import determinism + no-leak.
2. `games/vow_tide/tests/golden_traces/{bot-vs-bot-full-match-3p,bot-vs-bot-full-match-7p,wasm-exported-hook-to-terminal,promoted-helper-backport-preservation}.trace.json` (bot traces finalized once 012 lands).

### Commands

1. `cargo test -p vow_tide --test replay --test serialization`
2. `cargo test -p vow_tide`
3. Narrower command rationale: native replay/serialization tests are the determinism boundary; `replay-check --all` confirms via the tool once 016 registers vow_tide.

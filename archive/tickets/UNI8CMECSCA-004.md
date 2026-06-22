# UNI8CMECSCA-004: Add collision/ambiguity characterization tests around the local encoders

**Status**: COMPLETED
**Priority**: MEDIUM
**Effort**: Medium
**Engine Changes**: Yes (deterministic evidence) — characterization tests around the per-game local action-tree encoders
**Deps**: UNI8CMECSCA-003

## Problem

The current per-game action-tree byte encoders (in `games/*/src/replay_support.rs`) join field values without explicit framing, so distinct trees can be ambiguous: a delimiter inside a string, an empty-vs-absent value, a nested choice boundary, or a metadata/tag-order swap can collide or silently reorder. Before introducing the framed versioned writer (UNI8CMECSCA-012/013), this ticket adds tests that **expose** that ambiguity against the current encoders without changing any expected legacy output — demonstrating *why* a framed `StableBytesWriter`/`ActionTreeEncodingVersion::V1` surface is needed. It declares no legacy hash "wrong" and mutates no golden.

## Assumption Reassessment (2026-06-22)

1. Local action-tree encoders live in `games/race_to_n/src/replay_support.rs` and `games/draughts_lite/src/replay_support.rs` (confirmed by grep for action-tree byte/hash logic); `crates/wasm-api/src/action_tree.rs` carries the WASM-side tree shaping. The kernel `ActionTree`/`ActionNode`/`ActionChoice`/`ActionMetadata`/`ActionPreview` contract lives in `crates/engine-core/src/action.rs`.
2. The ambiguity classes to exercise are fixed by spec §5 Wave-0 8C-004: delimiters inside strings, empty vs. absent values, nested choice boundaries, metadata/tag order, and fixture-metadata absence.
3. Cross-artifact boundary under audit: the existing local encoders and their golden traces (`games/race_to_n/tests/golden_traces/shortest-normal.trace.json`, `games/draughts_lite/tests/golden_traces/multi-jump.trace.json`). The tests observe current behavior; they do not modify the encoders or the goldens.
4. FOUNDATIONS §11 determinism: the tests demonstrate where the unframed encoding is ambiguous, motivating the framed v1 surface, while keeping all existing expected output byte-identical.
5. Deterministic replay/hash surface under audit: the per-game action-tree hash. The new tests must not change any existing expected hash or golden; they assert the *ambiguity*, characterized against the UNI8CMECSCA-003 baseline.

## Architecture Check

1. Demonstrating ambiguity with tests (rather than asserting it in prose) gives the framed-writer tickets a concrete regression target and proves the migration is motivated, not cosmetic.
2. No backwards-compatibility shim — the encoders are untouched; only tests are added.
3. `engine-core` untouched; no mechanic noun introduced.

## Verification Layers

1. New tests exercise each ambiguity class against the current encoders → the added characterization tests.
2. No existing golden or expected hash changes → `cargo run -p replay-check -- --game race_to_n --all`, `--game draughts_lite --all`.
3. Tests are deterministic and SUT-derived → `cargo test -p race_to_n -p draughts_lite`.

## What to Change

### 1. Collision/ambiguity tests (Race flat + Draughts compound)

Add tests that construct trees differing only by a delimiter-in-string, empty-vs-absent value, nested-boundary, or metadata/tag-order swap and show the current unframed encoder cannot always distinguish them (or relies on incidental separators). Reference the UNI8CMECSCA-003 baseline values; assert ambiguity, not a "corrected" output.

### 2. Fixture-metadata-absence case

Add a test showing the current path's behavior when fixture profile metadata is absent, motivating the profile-driver strictness in Wave 3.

## Files to Touch

- `games/race_to_n/tests/serialization_tests.rs` (modify — flat-tree ambiguity cases)
- `games/draughts_lite/tests/replay.rs` (modify — compound/nested ambiguity cases)

## Out of Scope

- Implementing the framed writer/encoder (UNI8CMECSCA-012/013).
- Changing any encoder, golden trace, or expected hash.
- Declaring a legacy hash invalid.

## Acceptance Criteria

### Tests That Must Pass

1. `cargo test -p race_to_n -p draughts_lite` passes, including the new ambiguity tests.
2. `cargo run -p replay-check -- --game race_to_n --all` and `--game draughts_lite --all` pass unchanged.
3. Each named ambiguity class (delimiter-in-string, empty-vs-absent, nested-boundary, metadata/tag-order, fixture-metadata-absence) has at least one test.

### Invariants

1. No existing golden trace or expected hash is modified.
2. The tests assert ambiguity/characterization, never a replacement "correct" legacy output.

## Test Plan

### New/Modified Tests

1. `games/race_to_n/tests/serialization_tests.rs` — flat-tree delimiter/empty-vs-absent/metadata-order ambiguity cases.
2. `games/draughts_lite/tests/replay.rs` — nested-choice-boundary ambiguity cases for the compound tree.

### Commands

1. `cargo test -p race_to_n -p draughts_lite`
2. `cargo run -p replay-check -- --game race_to_n --all`
3. The two pilot crates plus `replay-check` are the correct boundary because the ambiguity must be shown against the real local encoders and their goldens.

## Outcome

Completed: 2026-06-22

What changed:
- Added Race to N characterization tests showing the current flat action-tree hash collides for delimiter-bearing segments, empty-choice vs absent-boundary shapes, and metadata/tag order changes that are ignored by the legacy segment-only encoder.
- Added Draughts Lite characterization tests showing the current compound action-tree hash collides across recursive child boundaries and unframed metadata/tag entry boundaries.
- Added a Draughts Lite legacy-trace characterization that confirms current trace parsing still accepts a command fixture with no future profile metadata fields.

Deviations:
- The tests characterize current ambiguity and legacy profile-metadata absence only; no encoder, golden trace, expected hash, or fixture was changed.
- `cargo fmt --all` also normalized two assertion layouts in `games/river_ledger/tests/replay.rs` and `games/vow_tide/tests/replay.rs` that came from the previous characterization ticket; those are formatting-only changes.

Verification:
- `cargo fmt --all --check`
- `cargo test -p race_to_n -p draughts_lite`
- `cargo run -p replay-check -- --game race_to_n --all`
- `cargo run -p replay-check -- --game draughts_lite --all`

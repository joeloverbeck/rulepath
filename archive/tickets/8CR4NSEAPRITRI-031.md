# 8CR4NSEAPRITRI-031: Briar Circuit C-08 public-export profile driver

**Status**: COMPLETED
**Priority**: MEDIUM
**Effort**: Small
**Engine Changes**: Yes (dev-only profile adapter) — `games/briar_circuit/tests/replay.rs`; export bytes/class unchanged
**Deps**: 8CR4NSEAPRITRI-001, 8CR4NSEAPRITRI-022

## Problem

Briar's public viewer-timeline export is not yet validated through the shipped `public-export-v1` driver with round-trip and no-leak evidence (MSC-8C, C-08). Add a `public-export-v1` adapter over `export_viewer_timeline` with `ViewerExportClass::Public`, leaving existing export bytes/class authority unchanged (spec §3.9 Briar public-export, §5.9).

## Assumption Reassessment (2026-06-24)

1. `crates/game-test-support/src/profiles.rs::PublicExportV1Driver` exists; `games/briar_circuit/src/replay_support.rs::{export_viewer_timeline, import_viewer_timeline}` and `ViewerExportClass::Public` (game-local, in `games/briar_circuit/src/replay_support.rs`) exist. Confirmed during `/reassess-spec`.
2. Spec §3.9 classifies Briar `public-export-v1` as `migrate`; this ticket `Deps` `-022` (viewer geometry precedes export profile closeout, §11.3).
3. Cross-artifact: the public-export evidence contract is owned by `game-test-support`; Briar's export bytes/class stay authority. Baseline export bytes come from `-001`.
4. §11 no-leak firewall motivates this ticket: the public export round-trips and carries no private hand/pass datum.
5. Enforcement surface = `PublicExportV1Driver` round-trip + no-leak over the `ViewerExportClass::Public` timeline; no export byte/class flip.

## Architecture Check

1. A thin public-export profile adapter is cleaner than ad-hoc round-trip asserts — it routes evidence through the owned driver while Briar owns export bytes.
2. No backwards-compatibility shim is introduced; no export byte changes. Rollback removes only the adapter.
3. `engine-core` stays noun-free (§3); `game-test-support` is dev-only (§4/§5).

## Verification Layers

1. Public timeline round-trips, wrong metadata rejected -> schema/serialization validation via `PublicExportV1Driver`.
2. No private hand/pass datum in the public export -> no-leak visibility test over the public timeline.
3. Export bytes/class unchanged -> golden trace byte check (`replay-check --game briar_circuit --all` byte-identical to baseline).

## What to Change

### 1. Add the `public-export-v1` profile adapter

In `games/briar_circuit/tests/replay.rs`, add a `PublicExportV1Driver` adapter over `export_viewer_timeline`/`import_viewer_timeline` with `ViewerExportClass::Public`, providing round-trip + no-leak evidence and delegating export meaning to Briar.

## Files to Touch

- `games/briar_circuit/tests/replay.rs` (modify)

## Out of Scope

- The replay-command (`-029`), setup (`-030`), and seat-private export (`-032`) profiles; the domain profile is pilot credit.
- Any export byte/class flip.
- Any reveal/scoring policy change.

## Acceptance Criteria

### Tests That Must Pass

1. `cargo test -p briar_circuit` is green, including the `public-export-v1` round-trip + no-leak test.
2. `cargo run -p replay-check -- --game briar_circuit --all` passes with export bytes unchanged.
3. `bash scripts/boundary-check.sh` passes.

### Invariants

1. The public export round-trips with no private datum; export bytes/class are unchanged.
2. The driver validates metadata only and delegates to Briar.

## Test Plan

### New/Modified Tests

1. `games/briar_circuit/tests/replay.rs` — `public-export-v1` round-trip + no-leak over the `Public` viewer timeline.

### Commands

1. `cargo test -p briar_circuit`
2. `cargo run -p replay-check -- --game briar_circuit --all`
3. The per-game replay test is the correct boundary: the profile is a dev-only evidence adapter over the existing export.

## Outcome

Completed: 2026-06-24

What changed:

1. Added a virtual `public-export-v1` profile adapter test for Briar public viewer timelines using `PublicExportV1Driver`.
2. Covered public timeline round-trip through `import_viewer_timeline` and asserted hidden hand/pass card ids are absent from the public export.
3. Added reject coverage for wrong validator owner, wrong visibility, and unknown profile fields.

Deviations: None.

Verification:

1. `cargo fmt --all --check` - passed.
2. `cargo test -p briar_circuit` - passed.
3. `cargo run -p replay-check -- --game briar_circuit --all` - passed.
4. `bash scripts/boundary-check.sh` - passed.

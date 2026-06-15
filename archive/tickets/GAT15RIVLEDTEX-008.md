# GAT15RIVLEDTEX-008: Semantic effects, visibility projection, view hashes, and UI metadata

**Status**: COMPLETED
**Priority**: HIGH
**Effort**: Large
**Engine Changes**: Yes — `games/river_ledger/src/effects.rs`, `src/visibility.rs`, `src/ui.rs` (filled), `src/lib.rs`
**Deps**: GAT15RIVLEDTEX-007

## Problem

River Ledger needs viewer-safe projection: semantic public/private effects with filtering scopes, `project_view` producing `PublicView`/`PrivateView` for the public observer and each seat, per-seat and observer view hashes, effect redaction, and the Rust-authored UI metadata (labels, seat metadata, viewer modes, action presentation hints, outcome-explanation fields). This is the no-leak firewall the tests in 009 exercise.

## Assumption Reassessment (2026-06-14)

1. `games/poker_lite/src/{effects,visibility,ui}.rs` are the precedent; `engine-core` already exposes generic `VisibilityScope { Public, PrivateToSeat(SeatId) }`, `Viewer`, and `Actor` (verified in `crates/engine-core/src/lib.rs`), reused without extension.
2. `specs/...-base.md` §4.1 (effects/visibility/ui), §8 (trace schema v1 reused; stricter per-seat/observer view-hash semantics, no migration), and §4.1 G15-RL-006 fix `RL-VIS-*`, `RL-REPLAY-*`, `RL-SHOW-VIEWER-*`.
3. Cross-artifact boundary under audit: visibility projects the `state.rs`/`TerminalOutcome` records (003/007); `ui.rs` was stubbed by 003 and is filled here (it consumes the outcome-explanation fields from 007 and the action hints from 005); the projections + view hashes feed the no-leak tests (009) and the replay export (010). Trace schema v1 (`ReplayRecord.seats: Vec<SeatAssignment>`, `SchemaVersion(1)`) is reused, not migrated.
4. FOUNDATIONS §2 (behavior authority) motivates this ticket: public/private view projection, effect emission, and UI metadata are Rust-owned; TypeScript renders them and never derives private facts.
5. §11 no-leak firewall is the enforcement surface under audit: hole cards, burn cards, deck order/tail, future community cards, raw full trace, private bot inputs, private diagnostics, and folded unrevealed cards must never reach an unauthorized viewer's projection, effect log, or view hash; per-seat and observer view hashes are deterministic. Confirm no projection or effect path carries a hidden fact to the wrong viewer.

## Architecture Check

1. A single `project_view(viewer)` + scoped effects make the no-leak firewall one auditable boundary and view hashes a deterministic function of the viewer-safe view, matching the sibling visibility pattern.
2. No backwards-compatibility aliasing/shims — new modules; `ui.rs` stub filled in place; no trace-schema migration.
3. `engine-core` stays noun-free (§3); reuses generic visibility contracts; no `game-stdlib` promotion (§4).

## Verification Layers

1. Observer/seat projections expose only authorized facts -> `cargo test -p river_ledger` projection unit tests (full pairwise sweep in 009).
2. Per-seat + observer view hashes deterministic and viewer-distinct -> view-hash unit tests.
3. Effect redaction drops hidden facts from public effect logs -> effect-scope assertion tests (§11 no-leak).
4. Trace schema v1 unchanged -> grep-proof `SchemaVersion(1)` reuse; no migration note.

## What to Change

### 1. `games/river_ledger/src/effects.rs`

Semantic public/private effects and filtering scopes for deal, community reveal, contribution change, street advance, and showdown.

### 2. `games/river_ledger/src/visibility.rs`

`project_view`, `PublicView`, `PrivateView`, observer projection, seat-private projection, per-seat + observer view hashes, effect redaction.

### 3. `games/river_ledger/src/ui.rs` (filled)

Labels, seat metadata, viewer modes, action presentation hints, and outcome-explanation fields (Rust-authored viewer-facing metadata).

## Files to Touch

- `games/river_ledger/src/effects.rs` (new)
- `games/river_ledger/src/visibility.rs` (new)
- `games/river_ledger/src/ui.rs` (modify; created by 003)
- `games/river_ledger/src/lib.rs` (modify; created by 003)

## Out of Scope

- The full pairwise N-seat no-leak test harness and no-leak golden traces (GAT15RIVLEDTEX-009).
- Replay export/import redaction and serialization tests (GAT15RIVLEDTEX-010).
- Any trace-schema migration (forbidden without ADR).

## Acceptance Criteria

### Tests That Must Pass

1. `cargo test -p river_ledger` — observer/seat projection, view-hash determinism, effect redaction unit tests.
2. View hashes differ per authorized viewer and are stable across re-projection of the same state.
3. `bash scripts/boundary-check.sh` — no mechanic noun reaches `engine-core`.

### Invariants

1. View projection, effects, and UI metadata are Rust-owned and viewer-safe (§2/§11).
2. No hidden fact reaches an unauthorized projection, effect log, or view hash (§11); trace schema v1 unchanged.

## Test Plan

### New/Modified Tests

1. `games/river_ledger/src/visibility.rs` + `src/effects.rs` (new) — `#[cfg(test)]` projection, view-hash, and redaction unit tests.

### Commands

1. `cargo test -p river_ledger`
2. `cargo test -p river_ledger && bash scripts/boundary-check.sh`
3. A crate-scoped test is the correct boundary; the cross-seat pairwise sweep and export no-leak are exercised in 009/010.

## Outcome

Completed: 2026-06-14

Implemented River Ledger semantic effects, viewer-safe visibility projection, stable view hashes, and expanded Rust-authored UI metadata. Added `effects.rs` with public/private scoped effect envelopes, setup deal effects, and viewer filtering. Added `visibility.rs` with observer and seat-private projections, public board/ledger/status fields, terminal allocation summaries, stable serialization, and deterministic `view_hash`. Expanded `ui.rs` with viewer modes, seat metadata labels, action hint metadata, and outcome explanation metadata.

Added crate tests proving observer projections expose only counts/public board facts, seat projections expose only the viewer's own hole cards, view hashes are stable and viewer-distinct, and private deal effects are scoped to the owning seat. Confirmed the trace schema was not migrated.

Deviations: full pairwise N-seat no-leak sweep remains deferred to GAT15RIVLEDTEX-009, and replay export/import redaction remains deferred to GAT15RIVLEDTEX-010. `apply_action` remains a state-transition API; semantic effects are provided as scoped constructors/filtering helpers for the later replay/WASM integration tickets.

Verification:

- `cargo fmt --all`
- `cargo fmt --all --check`
- `cargo test -p river_ledger`
- `bash scripts/boundary-check.sh`
- `git diff --check`
- `rg -n "SchemaVersion\\(2\\)|schema_version.*2|trace schema" games/river_ledger/src crates/engine-core/src tools specs/gate-15-river-ledger-texas-holdem-base.md`

Unrelated pre-existing worktree changes left untouched:

- `.claude/skills/spec-to-tickets/SKILL.md`
- `.claude/skills/spec-to-tickets/references/decomposition-patterns.md`

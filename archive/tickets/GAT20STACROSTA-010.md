# GAT20STACROSTA-010: Public visibility, UI metadata, and all-public no-leak audit

**Status**: COMPLETED
**Priority**: HIGH
**Effort**: Medium
**Engine Changes**: Yes — `games/starbridge_crossing/src/{visibility.rs,ui.rs}`, `tests/visibility.rs`
**Deps**: GAT20STACROSTA-009

## Problem

Starbridge Crossing is perfect-information: the whole board, active seat, legal action tree, effects, history, finish order, and terminal explanation are public, with no per-seat private datum. This ticket lands the public view projection, the viewer-facing Rust UI metadata, and the explicit all-public confirming no-leak audit (`docs/FOUNDATIONS.md` §11 requires the no-leak posture to be proven, not assumed).

## Assumption Reassessment (2026-06-27)

1. View projection uses the generic `engine-core` public/private view contract (`VisibilityScope`, viewer-safe view) — confirmed `crates/engine-core/src/lib.rs`; Starbridge's public view = the full `state.rs` snapshot for every viewer, with no redaction class.
2. `src/ui.rs` is viewer-facing Rust UI metadata (labels, seat-neutral coordinate label, occupancy, home/target zone, legal-action state, preview copy) — co-located here per the official-game pipeline (distinct from the TS renderer in GAT20STACROSTA-015).
3. Cross-artifact boundary: the public view is what the WASM bridge (014) and the renderer (015) consume; ADR 0004 (hidden-info replay/export taxonomy) is **not applicable** and is recorded as such with rationale, since no hidden class exists.
4. §11 no-leak firewall motivates this ticket: even with no hidden information, the audit must be explicit — public observer and every seat viewer receive identical board facts (seat-local labels may highlight "you" but add no private fact); no private-only datum exists in payloads, DOM, logs, previews, effect logs, bot explanations, or replay exports.
5. No-leak enforcement surface (§11): `tests/visibility.rs` is the confirming audit. Confirm every seat view equals the public view modulo viewer-label affordances, and that diagnostics/previews carry no fabricated private state — there is none to leak, but the test asserts parity rather than omitting the check.

## Architecture Check

1. Merging public view + no-leak audit into one ticket (no viewer-scoped export split) is correct for a perfect-information game and mirrors the flood_watch/Gate 12 single-visibility-ticket precedent.
2. No backwards-compatibility shims.
3. `engine-core` view contract consumed generically; no mechanic noun added; no `game-stdlib` change.

## Verification Layers

1. Public-observer completeness -> no-leak visibility test: public view contains all spaces, occupants, active seat, finish ranks, terminal reason.
2. Seat-viewer parity (§11) -> no-leak visibility test: each seat view equals public board facts modulo "you" labels; no private datum.
3. ADR 0004 N/A -> FOUNDATIONS alignment check: `GAME-EVIDENCE.md` (authored in 018) will record the not-applicable rationale; here the visibility test asserts no redaction class is needed.
4. UI metadata is presentation data only -> manual review: `ui.rs` carries labels/zones/preview copy, decides no legality.

## What to Change

### 1. Author `src/visibility.rs`

Public view projection: the full public snapshot for every viewer; a confirming all-public audit path (no redaction class).

### 2. Author `src/ui.rs`

Viewer-facing UI metadata: seat-neutral coordinate labels, occupancy/zone tags, legal-action state, Rust-provided preview copy.

## Files to Touch

- `games/starbridge_crossing/src/visibility.rs` (new)
- `games/starbridge_crossing/src/ui.rs` (new)
- `games/starbridge_crossing/tests/visibility.rs` (new)
- `games/starbridge_crossing/src/lib.rs` (modify; created by 004 — add `pub mod {visibility,ui};`)

## Out of Scope

- Replay export / serialization round-trip — GAT20STACROSTA-011.
- The TS web renderer + DOM no-leak smoke — GAT20STACROSTA-015/017.
- `GAME-EVIDENCE.md` cross-links — GAT20STACROSTA-018.

## Acceptance Criteria

### Tests That Must Pass

1. `cargo test -p starbridge_crossing --test visibility`
2. `cargo test -p starbridge_crossing`
3. `bash scripts/boundary-check.sh`

### Invariants

1. Public observer and every seat viewer receive identical board facts; no private-only datum exists or leaks (§11).
2. `ui.rs` and `visibility.rs` decide no legality (§2).

## Test Plan

### New/Modified Tests

1. `games/starbridge_crossing/tests/visibility.rs` — public-observer all-public, seat-viewer parity across `{2,3,4,6}`, no fabricated private state.

### Commands

1. `cargo test -p starbridge_crossing --test visibility`
2. `cargo test -p starbridge_crossing && bash scripts/boundary-check.sh`
3. `--test visibility` is the correct no-leak boundary; full crate run confirms view/state integration.

## Outcome

Completed: 2026-06-27

What changed:

- Added `games/starbridge_crossing/src/visibility.rs` with an all-public view
  projection for observers and seat viewers: board spaces, occupancy, seats,
  active seat, finish ranks, terminal status, counters, and an explicit
  no-redaction audit.
- Added `games/starbridge_crossing/src/ui.rs` with seat-neutral coordinate,
  zone, and space metadata for renderer-facing Rust labels.
- Added `games/starbridge_crossing/tests/visibility.rs` proving public observer
  completeness, seat-view parity across `{2,3,4,6}`, and absence of private,
  hidden, or redacted visibility classes.
- Updated `games/starbridge_crossing/src/lib.rs` exports for visibility and UI
  metadata.

Deviations from plan:

- None. Browser DOM no-leak smoke, replay exports, and evidence cross-links
  remain deferred to their later tickets.

Verification:

- `cargo fmt --all --check` passed.
- `cargo test -p starbridge_crossing --test visibility` passed: 3 integration
  tests.
- `cargo test -p starbridge_crossing` passed: 22 unit tests, 20 integration
  tests, 0 doctests.
- `bash scripts/boundary-check.sh` passed (`engine-core boundary check passed`;
  `game-test-support dev-only boundary check passed`).
- `git diff --check` passed.

Unrelated worktree changes left untouched:

- `.claude/skills/spec-to-tickets/SKILL.md`

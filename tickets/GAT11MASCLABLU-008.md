# GAT11MASCLABLU-008: Visibility, replay/export surfaces, and UI metadata

**Status**: PENDING
**Priority**: HIGH
**Effort**: Medium
**Engine Changes**: Yes — new `games/masked_claims/src/{visibility.rs,replay_support.rs,ui.rs}`, `src/lib.rs`
**Deps**: GAT11MASCLABLU-007

## Problem

Every viewer-facing surface — public observer view, seat-private view, effect filtering, replay export/import, and UI metadata — must be projected from Rust and must never leak an unrevealed tile identity. The browser export defaults to a viewer-scoped observation timeline with claim commands redacted to declared grades (ADR 0004), and accepted masks, hands, and the reserve never appear in any export, ever.

## Assumption Reassessment (2026-06-10)

1. State, effects, and resolution from GAT11MASCLABLU-004–007 provide the projection inputs. The view-projection model is `pub fn project_view(state: &State, viewer: &Viewer) -> PublicView` (confirmed in `games/plain_tricks/src/visibility.rs`); the replay model is `<Game>InternalTrace` plus `PublicReplayExport` / `PublicReplayStep` / `PublicReplayTimeline` with `to_json` / `from_json` (confirmed in `games/plain_tricks/src/replay_support.rs`).
2. Spec §"Visibility and no-leak model" plus ADR 0004 (`docs/adr/0004-hidden-info-replay-export-taxonomy.md`): internal full traces remain native test authority; viewer-scoped browser export redacts claim commands to declared grades and includes reveals only from their `MaskRevealed` event; veiled galleries, hands, and the reserve never appear in any export at any point, including post-terminal.
3. Cross-artifact boundary under audit: the public/private view contract and the replay/export taxonomy (ADR 0004) — the shared boundary across `visibility.rs`, `replay_support.rs`, and the future WASM bridge (GAT11MASCLABLU-014).
4. FOUNDATIONS §2 (view projection and replay/hash are Rust-owned) and §11 (views are viewer-safe; hidden info does not leak through any payload, log, dev panel, or replay export) are the principles under audit.
5. Determinism + no-leak enforcement surfaces: view/export hashes and the auxiliary-surface rule. Confirm the viewer-scoped export is deterministic (stable serialization order, `to_json`/`from_json` round-trip) and that undo/history/dev-panel surfaces draw from the same viewer-scoped projection — the named anti-pattern (secrets escaping via history while the primary view was filtered) must be impossible here.

## Architecture Check

1. A single viewer-scoped projection feeding every auxiliary surface (views, effect log, export, dev panels) is the robust fix for the boardgame.io/secret_draft leak postmortem — secrets cannot escape through a side channel because there is no second source of truth.
2. No backwards-compatibility aliasing or shims introduced.
3. `engine-core` stays noun-free (`veiled gallery`/`pedestal` nouns are game-local); no `game-stdlib` visibility or export helper.

## Verification Layers

1. Public + seat view no-leak -> no-leak visibility test over the pedestal pre-reveal, veiled galleries, hands, and reserve (full suite in GAT11MASCLABLU-010).
2. Viewer-scoped export redaction -> no-leak export test + golden trace `public-replay-export-import` (GAT11MASCLABLU-011).
3. Deterministic view/export hashes + round-trip -> deterministic replay-hash check; `to_json`/`from_json` round-trip test.
4. UI metadata viewer-safe -> `ui.rs` unit test (labels/preview copy reference no hidden identity).

## What to Change

### 1. `src/visibility.rs`

`project_view` for the public observer view (turn/phase, pedestal declared grade + presence, veiled-gallery counts with declared grades, exposed rows, scores/counters, terminal) and the seat-private view (adds the seat's own hand); effect filtering; no-leak helpers. Never expose pedestal tile ID pre-reveal, veiled-gallery IDs, hands, or reserve to observer/opponent scope.

### 2. `src/replay_support.rs`

Internal full trace (`MaskedClaimsInternalTrace`) plus viewer-scoped `PublicReplayExport`/`PublicReplayStep`/`PublicReplayTimeline` with `to_json`/`from_json`. Claim commands redacted to declared grades; reveals only from `MaskRevealed`; veiled galleries, hands, and reserve never present in export.

### 3. `src/ui.rs`

Viewer-safe UI metadata: grade labels, tokens, accessibility metadata, and preview/score-preview copy — Rust-emitted, referencing only public or own-seat state.

## Files to Touch

- `games/masked_claims/src/visibility.rs` (new)
- `games/masked_claims/src/replay_support.rs` (new)
- `games/masked_claims/src/ui.rs` (new)
- `games/masked_claims/src/lib.rs` (modify)

## Out of Scope

- Bots (GAT11MASCLABLU-009).
- The native test suite and golden traces (GAT11MASCLABLU-010/011).
- WASM bridge wiring (GAT11MASCLABLU-014).

## Acceptance Criteria

### Tests That Must Pass

1. `cargo test -p masked_claims` visibility tests pass; no unrevealed tile ID appears in any public/opponent view.
2. Viewer-scoped export redacts claim commands to declared grades and never contains veiled galleries, hands, or the reserve — including post-terminal.
3. Export `to_json` → `from_json` round-trips deterministically.

### Invariants

1. View projection and replay/export are Rust-owned and viewer-safe (FOUNDATIONS §2/§11).
2. All auxiliary surfaces draw from the same viewer-scoped projection (no second source of truth).

## Test Plan

### New/Modified Tests

1. `games/masked_claims/src/visibility.rs` `#[cfg(test)]` — public/seat projection no-leak.
2. `games/masked_claims/src/replay_support.rs` `#[cfg(test)]` — export redaction + round-trip determinism.

### Commands

1. `cargo test -p masked_claims`
2. `cargo clippy -p masked_claims --all-targets -- -D warnings`
3. Unit-level boundary; the cross-surface no-leak sweep and `replay-check` proof land in GAT11MASCLABLU-010/011/015.

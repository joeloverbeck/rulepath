# GAT19MELLEDFIV-013: Viewer-scoped replay export/import and six-seat pairwise no-leak harness

**Status**: PENDING
**Priority**: HIGH
**Effort**: Large
**Engine Changes**: Yes — `games/meldfall_ledger/src/{replay_support,visibility}.rs`; no-leak golden traces; pairwise no-leak tests (ADR 0004/0009)
**Deps**: GAT19MELLEDFIV-012

## Problem

Meldfall Ledger must export/import replays under viewer scope (public observer and `seat_0`…`seat_5`) without privilege elevation, and prove no leak across **every** seat-private viewer at six seats (a full pairwise source-seat × viewer-seat matrix, exceeding the minimum sampling rule). This ticket completes the §11 no-leak firewall for the export surface and adds the pairwise harness + no-leak golden traces.

## Assumption Reassessment (2026-06-25)

1. `games/river_ledger/src/{visibility,replay_support}.rs` (viewer-scoped export pattern) is the model; the in-state projection firewall exists from GAT19MELLEDFIV-012; `replay_support.rs` export scaffolding from GAT19MELLEDFIV-005. `crates/game-test-support` provides the no-leak matrix geometry (MSC-8C-007).
2. Spec §7.3 (pairwise no-leak matrix + no-leak surfaces list), §7.4 (no-leak traces), and §8.1 (ADR 0004 viewer-scoped exports) define the harness; spec mandates all six seat-private viewers exercised in CI.
3. Cross-artifact: the replay-export contract (ADR 0004 viewer-scoped exports, ADR 0009 taxonomy v2) is the boundary — a public export must not import as a seat-private export (no privilege elevation), and stable serialization order must hold.
4. FOUNDATIONS §11 no-leak + ADR 0004/0009: export/import is the highest-risk surface; hidden hands and stock order must not appear in any viewer-scoped export, and replay must stay deterministic with stable order.
5. FOUNDATIONS §11 determinism: viewer-scoped export/import round-trips byte-identically; no nondeterministic field enters the canonical export form (builds on the GAT19MELLEDFIV-005 stable-order substrate).

## Architecture Check

1. Layering export redaction on the GAT19MELLEDFIV-012 projection (rather than a parallel path) means one firewall serves both in-state views and exports, and the pairwise harness proves it uniformly.
2. No backwards-compatibility shims.
3. `engine-core` untouched; export logic crate-local; reuses behavior-free no-leak geometry (MSC-8C-007), no new helper.

## Verification Layers

1. Public + all six seat-private exports round-trip without privilege elevation -> `seat-private-export-round-trip-all-viewers.trace.json`, `viewer-export-no-privilege-elevation.trace.json`.
2. Pairwise source-seat × viewer-seat matrix at six seats leaks nothing -> `tests/visibility.rs` pairwise matrix + `public-observer-no-leak-6p.trace.json`.
3. Viewer-scoped export is deterministic with stable order -> deterministic replay-hash check on the export traces.

## What to Change

### 1. `replay_support.rs` — viewer-scoped export/import

Export/import for public observer and `seat_0`…`seat_5`, redacting per viewer (no hidden hands, no stock order); reject privilege elevation on import (a public export cannot import as seat-private); stable serialization order under ADR 0009.

### 2. `visibility.rs` — pairwise no-leak utilities

Pairwise source-seat × viewer-seat redaction utilities backing the six-seat matrix assertions.

### 3. Pairwise no-leak harness + traces

`tests/visibility.rs` full six-seat pairwise matrix over view/action-tree/preview/diagnostics/effects/bot-explanation/export surfaces. Traces: `public-observer-no-leak-6p`, `seat-private-export-round-trip-all-viewers`, `viewer-export-no-privilege-elevation`.

## Files to Touch

- `games/meldfall_ledger/src/replay_support.rs` (modify; created by GAT19MELLEDFIV-003)
- `games/meldfall_ledger/src/visibility.rs` (modify; created by GAT19MELLEDFIV-003)
- `games/meldfall_ledger/tests/visibility.rs` (modify; created by GAT19MELLEDFIV-003 — pairwise matrix)
- `games/meldfall_ledger/tests/replay.rs` (modify; created by GAT19MELLEDFIV-003 — export/import round-trip)
- `games/meldfall_ledger/tests/golden_traces/public-observer-no-leak-6p.trace.json` (new)
- `games/meldfall_ledger/tests/golden_traces/seat-private-export-round-trip-all-viewers.trace.json` (new)
- `games/meldfall_ledger/tests/golden_traces/viewer-export-no-privilege-elevation.trace.json` (new)

## Out of Scope

- Browser DOM/a11y/storage/log no-leak (GAT19MELLEDFIV-020/021).
- Bot-explanation content (GAT19MELLEDFIV-014) — the matrix asserts bot-explanation surfaces leak nothing, but bot logic lands later.

## Acceptance Criteria

### Tests That Must Pass

1. `cargo test -p meldfall_ledger`: public + all six seat-private exports redact hidden hands and stock order; no privilege elevation on import.
2. Six-seat pairwise matrix: facts private to seat A never reach seat B, the public observer, effects, diagnostics, bot-explanation surfaces, or exports.
3. `cargo test --workspace` passes; `cargo run -p replay-check -- --game meldfall_ledger --all` (registered in GAT19MELLEDFIV-016) replays the no-leak traces deterministically.

### Invariants

1. No hidden information reaches any viewer-scoped export (FOUNDATIONS §11 no-leak; ADR 0004); no privilege elevation on import.
2. Viewer-scoped export/import is deterministic with stable order (FOUNDATIONS §11; ADR 0009).

## Test Plan

### New/Modified Tests

1. `games/meldfall_ledger/tests/visibility.rs` — six-seat pairwise no-leak matrix across all listed surfaces.
2. `games/meldfall_ledger/tests/replay.rs` — viewer-scoped export/import round-trip + no-privilege-elevation.
3. `games/meldfall_ledger/tests/golden_traces/{public-observer-no-leak-6p,seat-private-export-round-trip-all-viewers,viewer-export-no-privilege-elevation}.trace.json`.

### Commands

1. `cargo test -p meldfall_ledger`
2. `cargo test --workspace`
3. `cargo run -p replay-check -- --game meldfall_ledger --all` is fully wired in GAT19MELLEDFIV-016; until then export determinism is asserted by `cargo test`.

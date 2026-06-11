# GAT12FLOWATCOO-009: Replay support and viewer-scoped export/import

**Status**: PENDING
**Priority**: HIGH
**Effort**: Large
**Engine Changes**: Yes — `games/flood_watch/src/replay_support.rs` (internal full trace with deck order; viewer-scoped export/import with the undrawn deck redacted), stable summaries
**Deps**: GAT12FLOWATCOO-008

## Problem

`flood_watch` must replay deterministically and export safely. Internal native traces carry the seed and full deck order as test authority; the browser export defaults to the viewer-scoped observation timeline with the undrawn deck redacted — undrawn cards never appear in any export at any point, including post-terminal, and each card appears only from its `EventDrawn`/`ForecastRevealed` effect onward. This is the ADR 0004 hidden-info replay/export contract applied to a deck whose order is hidden from everyone.

## Assumption Reassessment (2026-06-11)

1. GAT12FLOWATCOO-008 provides the single public projection, effect filtering, and stable hashes; GAT12FLOWATCOO-004 keeps the deck order internal. `games/masked_claims/src/replay_support.rs` is the verified exemplar for the internal-full-trace vs viewer-scoped-export split (the spec names `masked_claims` as the closest template for redacted-export machinery); Gate 8/10 established the viewer-scoped export split this ticket mirrors.
2. The spec (§Implementation reference "Visibility and no-leak model" → internal-vs-export, Work-breakdown item 7, FOUNDATIONS-alignment "ADR 0004" row) fixes: internal traces carry seed + full deck order (test authority); viewer-scoped export carries commands + public effects only; undrawn cards never appear in any export at any point including post-terminal; public export defaults to the viewer-scoped observation timeline. `docs/adr/0004-hidden-info-replay-export-taxonomy.md` is the governing ADR (verified to exist, titled "Hidden-Info Replay-Export Taxonomy And Viewer-Aware Visibility Contract").
3. Cross-artifact boundary under audit: the replay/export serialization is the contract consumed by `tools/replay-check` (GAT12FLOWATCOO-015), the WASM `export_replay`/`import_replay` (GAT12FLOWATCOO-014), and the golden traces (GAT12FLOWATCOO-011, including `public-replay-export-import.trace.json` and `public-observer-no-leak.trace.json`). The export schema must be additive to the existing replay contract — no trace-schema or hash-semantics migration (the spec forbids accidental migration).
4. FOUNDATIONS §11 (replay/hashes/serialization order/RNG/traces remain deterministic; hidden information does not leak through replay exports) and §2 (Rust owns replay/hash behavior) motivate this ticket; ADR 0004 governs the viewer-aware export taxonomy. The §13 ADR trigger "changing replay/hash semantics" does **not** fire — this reuses the existing contract for a new game, no semantics change.
5. Enforcement surface: this is the no-leak firewall on the auxiliary export path (§11). The internal trace (with deck order) is native test authority only and is never served to a browser; the viewer-scoped export redacts the undrawn deck by construction. Determinism: same seed+seats+scenario+command stream reproduces identical state/effect/action-tree/view hashes and draw ordering.

## Architecture Check

1. Separating the internal full trace (deck order, native authority) from the viewer-scoped export (redacted, browser-facing) — rather than a single trace with conditional redaction at serve time — makes the no-leak guarantee structural: the browser export is built from the public timeline that never contained the undrawn order.
2. No backwards-compatibility aliasing/shims; reuses the existing replay/export contract additively (no schema/hash migration).
3. `engine-core` stays noun-free — replay uses the generic replay/checkpoint/hash contracts; deck/event nouns stay game-local.

## Verification Layers

1. Deterministic replay -> deterministic replay-hash check: same seed+seats+scenario+command stream reproduces state/effect/action-tree/view hashes, draw ordering, and terminal outcome.
2. Redacted export, all phases -> no-leak visibility test: viewer-scoped export and import timelines contain no undrawn-deck order/identities at any point, including post-terminal.
3. Internal trace is authority, not served -> grep-proof the full-order trace path is native-test-only and the browser export defaults to the viewer-scoped timeline.
4. No accidental migration -> schema/serialization validation: the export is additive to the existing replay contract; no trace-schema or hash-semantics change.

## What to Change

### 1. `games/flood_watch/src/replay_support.rs`

Implement: the internal full trace carrying seed + full deck order (native test authority); the viewer-scoped export/import carrying commands + public effects with the undrawn deck redacted (defaults to the viewer-scoped observation timeline per ADR 0004); stable summaries; and the replay-hash plumbing reusing the existing deterministic contract. Ensure import reconstructs a viewer-safe timeline without the undrawn order.

## Files to Touch

- `games/flood_watch/src/replay_support.rs` (modify — fill the stub)

## Out of Scope

- WASM `export_replay`/`import_replay` bridge wiring (GAT12FLOWATCOO-014).
- Golden trace files themselves (GAT12FLOWATCOO-011) — this ticket builds the machinery the traces exercise.
- `tools/replay-check` registration (GAT12FLOWATCOO-015).

## Acceptance Criteria

### Tests That Must Pass

1. Replay tests prove same seed+seats+scenario+command stream reproduces state hashes, effect hashes, action-tree hashes, view hashes, draw ordering, and the terminal outcome.
2. Serialization tests prove stable summaries and unknown-field rejection for viewer-scoped export and internal trace helpers.
3. A no-leak test proves the viewer-scoped export/import timeline carries no undrawn-deck order/identities, including post-terminal.

### Invariants

1. The undrawn event-deck order appears only in the internal native trace (test authority) and never in any browser-facing export at any point.
2. Replay/hash/serialization stay deterministic; the export is additive with no trace-schema or hash-semantics migration.

## Test Plan

### New/Modified Tests

1. `games/flood_watch/tests/replay.rs` — full hash/draw-order/outcome reproduction (expanded in GAT12FLOWATCOO-011).
2. `games/flood_watch/tests/serialization.rs` — stable export summaries + unknown-field rejection (expanded in GAT12FLOWATCOO-011).

### Commands

1. `cargo test -p flood_watch --test replay`
2. `cargo test -p flood_watch`
3. `cargo run -p replay-check -- --game flood_watch --all` is the full golden-trace boundary but needs the traces (GAT12FLOWATCOO-011) and tool registration (GAT12FLOWATCOO-015); the replay + serialization unit tests are the correct boundary for the machinery diff.

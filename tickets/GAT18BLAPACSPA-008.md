# GAT18BLAPACSPA-008: public/seat visibility, pairwise no-leak harness, viewer-scoped replay export/import

**Status**: PENDING
**Priority**: HIGH
**Effort**: Large
**Engine Changes**: Yes — `games/blackglass_pact` (visibility/replay_support) + golden traces
**Deps**: GAT18BLAPACSPA-007

## Problem

Implement the public observer and four seat-private views, the pairwise no-leak harness over all 12 ordered seat pairs (partner pairs included), the blind pre-deal no-card surfaces, and viewer-scoped public + seat-private replay export/import under ADR 0004/0009 — with no team-private viewer (the locked variant has no team-private fact). Partner-hand visibility is never granted by partnership (spec §3.1, §7.3–§7.5, §8.7, ADR 0004/0009, `BP-VIS-*`/`BP-REPLAY-*`, candidate task `GAT18-BLAPAC-008`).

## Assumption Reassessment (2026-06-25)

1. `visibility.rs`/`replay_support.rs` stubs from GAT18BLAPACSPA-003 are implemented here; sibling `games/briar_circuit/tests/{visibility,replay}.rs` + its `public/seat-private-replay-export-import` traces are the convention.
2. Spec §7.3 fixes the viewer matrix (1 observer + 4 seats, 0 team-private); §7.4 fixes the 12 ordered pairs + additional edges; ADR 0009 governs any byte change (expected `none`).
3. Cross-artifact boundary under audit: the public/private view projection and the export/import payloads — hidden facts private to seat A must not reach seat B, the observer, DOM/logs, bot explanations, or replay exports.
4. FOUNDATIONS §11 (no-leak firewall) motivates this ticket: it is the gate's exhaustive viewer-safety proof; coverage is exhaustive, not sampled (TESTING §8.2).
5. Enforcement surface: every named surface in spec §7.5 (action trees, previews, diagnostics, effect logs, candidate rankings, exports) must carry no unauthorized datum, and import cannot elevate viewer authorization; replay reproduces byte-for-byte under fixed versions with no unauthorized migration (§11/§13, ADR 0009).

## Architecture Check

1. A single Rust projection layer that emits already-safe viewer payloads (vs. client-side redaction) is the only §11-compliant design; the export is a viewer-scoped observation history, not full-state serialization.
2. No shims; no team-private viewer invented for a ruleset with no team-private fact.
3. `engine-core` untouched; visibility/export policy is game-local; no `game-stdlib` change.

## Verification Layers

1. Observer + 4 seat views + all 12 ordered pairs carry no protected datum -> pairwise no-leak harness + `seat-private-pairwise-no-leak-all-four` / `public-observer-no-leak` traces.
2. Blind pre-deal surfaces leak no future card -> `blind-phase-no-future-card-leak` trace (re-proving GAT18BLAPACSPA-004 cross-viewer).
3. Public + all four seat-private exports round-trip; import cannot elevate authorization; replay byte-stable -> export/import tests + replay traces; ADR 0009 migration note `none`.

## What to Change

### 1. View projection

`visibility.rs`: public observer view (all public facts, no hand) and four seat views (own hand + own controls + all public facts); no team-private mode; safe previews/diagnostics only.

### 2. Pairwise no-leak harness

`tests/visibility.rs`: 12 ordered seat-to-seat pairwise assertions (partner pairs explicitly), source→observer edges, blind future-deck edge, candidate-state edge, cross-export edge.

### 3. Viewer-scoped replay export/import

`replay_support.rs`: `public-export-v1` + `seat-private-export-v1` (per seat) round-trip; import grants no other viewer's authorization; ADR 0009 hash discipline. Add visibility/export golden traces (spec §7.6 #61–#68).

## Files to Touch

- `games/blackglass_pact/src/{visibility,replay_support}.rs` (modify)
- `games/blackglass_pact/tests/{visibility,replay,serialization}.rs` (modify)
- `games/blackglass_pact/tests/golden_traces/*.trace.json` (new — no-leak + export/import)

## Out of Scope

- Browser/DOM no-leak assertions (GAT18BLAPACSPA-016 e2e smoke).
- WASM export bridge wiring (GAT18BLAPACSPA-014).

## Acceptance Criteria

### Tests That Must Pass

1. `cargo test -p blackglass_pact --test visibility` (observer + 4 seats + 12 ordered pairs + blind edge).
2. `cargo test -p blackglass_pact --test replay` (public + 4 seat-private export/import round-trip; no privilege elevation).
3. `cargo test -p blackglass_pact --test serialization` (stable bytes; no unordered map defines hashes).

### Invariants

1. No protected datum private to seat A reaches seat B, the observer, or any export/log surface.
2. No team-private viewer exists; partnership grants no hand visibility; import elevates no authorization.

## Test Plan

### New/Modified Tests

1. `games/blackglass_pact/tests/visibility.rs` — 12-pair pairwise + observer + blind no-leak harness.
2. `games/blackglass_pact/tests/replay.rs` — public + four seat-private export/import round-trips.
3. `games/blackglass_pact/tests/golden_traces/seat-private-pairwise-no-leak-all-four.trace.json` — exhaustive no-leak evidence.

### Commands

1. `cargo test -p blackglass_pact --test visibility --test replay --test serialization`
2. `cargo test -p blackglass_pact`
3. Crate-scoped exhaustive viewer tests are the boundary; `replay-check --all` (GAT18BLAPACSPA-011) validates the traces, browser no-leak is GAT18BLAPACSPA-016.

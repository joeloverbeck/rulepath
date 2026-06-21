# GAT17VOWTIDOHHEL-010: Variable-N visibility projection, effect filtering, exhaustive pairwise no-leak

**Status**: COMPLETED
**Priority**: HIGH
**Effort**: Large
**Engine Changes**: Yes — new `games/vow_tide/src/visibility.rs`; modifies `effects.rs`; new `games/vow_tide/tests/visibility.rs` + no-leak golden traces
**Deps**: 008, 009

## Problem

Project public-observer and every seat-private view for N=3..7, redacting other seats' hands and the hidden stock identity/order, and filter semantic effects independently from state views. Prove no private datum leaks across the exhaustive source→unauthorized-viewer matrix. This is the §11 no-leak firewall for the gate.

## Assumption Reassessment (2026-06-21)

1. `games/vow_tide/src/state.rs` (006–009) holds per-seat hands, hidden stock, public bids/tricks/scores; `visibility.rs` is new and is the only authorized projector. Sibling `games/briar_circuit/src/visibility.rs` + `tests/visibility.rs` are the structural precedent (canary tokens per seat + stock).
2. Spec §7.3–§7.5 + Appendix B.5 + `VT-VIEW/EFFECT-001` fix the viewer matrix (observer, `Seat(i)`, internal authority) and the mandatory no-leak datum taxonomy; §7.4 chooses **exhaustive CI viewer coverage** (not sampling), 110 ordered seat-pairs + 25 source→observer edges + 25 export classes across N=3..7.
3. Cross-artifact boundary under audit: the viewer-safe view/effect projections are the contract every downstream surface (replay 011, WASM bridge 017, DOM 018) must not widen; the per-seat private-view hash set is the determinism witness.
4. FOUNDATIONS §11 no-leak firewall is the principle under audit: facts private to seat A must not reach seat B, the observer, effects, bot inputs, or exports; hidden stock identity/order reaches no browser viewer ever.
5. §11/§12 enforcement surface: the exhaustive pairwise harness asserts that for every ordered (A,B), no A-private canary appears in B's view/serialized view/action tree/preview/diagnostics/effect stream. Deterministic: each source seat + stock gets a distinct canary; projection introduces no nondeterminism.

## Architecture Check

1. A single Rust projector plus an exhaustive canary harness is the strongest possible no-leak guarantee and the only place visibility is decided — the browser receives already-safe payloads.
2. No shims; new visibility module + tests.
3. `engine-core` untouched (uses its view contract); no `game-stdlib` change.

## Verification Layers

1. Observer sees only public facts; seat sees own hand + public; no other hand/stock → `cargo test -p vow_tide --test visibility` (exhaustive N=3..7 matrix).
2. Effects filtered independently; no private identity for unauthorized viewer → effect-filter unit tests.
3. Per-seat private-view hashes stable + distinct → trace hashes in no-leak traces.
4. No hidden-info path → `bash scripts/boundary-check.sh` + manual review of `visibility.rs` (no stock identity in any public branch).

## What to Change

### 1. Viewer projections

`visibility.rs`: `PublicObserver` and `Seat(seat_i)` projections per Appendix B.5; redact other hands, hidden stock identity/order, and other seats' private legal trees/previews. Active seat's own legal tree only when active.

### 2. Effect filtering

Filter semantic effects per viewer independently from the state view; never include hidden stock/hand identities for an unauthorized viewer.

### 3. Exhaustive no-leak harness

`tests/visibility.rs`: per-N canary harness over the §7.4 matrix (110 seat-pairs + 25 observer edges + stock canaries), asserting absence in view/serialized view/action tree/preview/diagnostics/effect stream.

## Files to Touch

- `games/vow_tide/src/visibility.rs` (new)
- `games/vow_tide/src/effects.rs` (modify)
- `games/vow_tide/tests/visibility.rs` (new)
- `games/vow_tide/tests/golden_traces/public-observer-no-leak-3p.trace.json` (new)
- `games/vow_tide/tests/golden_traces/seat-private-pairwise-no-leak-7p.trace.json` (new)

## Out of Scope

- Viewer-scoped replay export/import (011) and the WASM bridge no-leak dispatch (017) — those reuse these projections.
- Browser DOM/storage no-leak (019).

## Acceptance Criteria

### Tests That Must Pass

1. `cargo test -p vow_tide --test visibility` — exhaustive pairwise no-leak for N=3..7.
2. `cargo test -p vow_tide --test serialization` — projected views round-trip with stable per-seat hashes.
3. `bash scripts/boundary-check.sh`.

### Invariants

1. No seat-A private datum (hand, stock) appears on any unauthorized viewer surface for any N.
2. Hidden stock identity/order reaches no browser-exposable view or effect.

## Test Plan

### New/Modified Tests

1. `games/vow_tide/tests/visibility.rs` — exhaustive canary matrix.
2. `games/vow_tide/tests/golden_traces/{public-observer-no-leak-7p,seat-private-pairwise-no-leak-3p}.trace.json` — with all private-view hashes.

### Commands

1. `cargo test -p vow_tide --test visibility`
2. `cargo test -p vow_tide`
3. Narrower command rationale: the native exhaustive matrix is the authoritative pairwise proof; the WASM-bridge harness (017) and browser samples (019) extend it to those surfaces.

## Outcome

Completed on 2026-06-21.

- Added `games/vow_tide/src/visibility.rs` as the Rust-owned projector for public observer and seat-private views.
- Projected public facts, owner-only hands, hand counts, stock count without stock identity/order, public bids/tricks/scores, terminal standings, and stable view bytes.
- Added effect filtering entrypoint; current semantic effects are public-only and the filter does not expose hidden hands or stock.
- Added exhaustive native no-leak coverage for 3–7 seats, verifying observer and every ordered seat-pair does not receive unauthorized hand or stock canaries.
- Added visibility trace fixtures for public observer no-leak and 7-seat pairwise no-leak.

Verification:

- `cargo fmt --all --check` passed.
- `cargo test -p vow_tide --test visibility` passed: 3 visibility/no-leak tests.
- `cargo test -p vow_tide --test serialization` passed: 4 serialization tests.
- `cargo test -p vow_tide` passed: 34 integration tests plus crate/doc test harnesses.
- `bash scripts/boundary-check.sh` passed with `engine-core boundary check passed`.
- `cargo clippy -p vow_tide --all-targets -- -D warnings` passed.

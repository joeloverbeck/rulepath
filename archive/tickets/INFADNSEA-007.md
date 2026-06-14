# INFADNSEA-007: Infra D — N-player pairwise no-leak test harness

**Status**: COMPLETED
**Priority**: HIGH
**Effort**: Large
**Engine Changes**: Yes — `crates/wasm-api` (a reusable pairwise no-leak harness, inline `#[cfg(test)]` or a new test module); no `engine-core`/`games/*` behavior change
**Deps**: INFADNSEA-001

## Problem

No-leak proof today is per-game and two-seat: each game's bridge tests and `tests/replay.rs`/`tests/serialization.rs` assert viewer safety for the fixed `observer/seat_0/seat_1` viewers. There is no reusable, seat-count-parameterized harness asserting pairwise *source-seat A × viewer-seat B × surface* no-leak across the full ADR-0004 / contract-§6 surface set (payloads, action trees, previews, effects, bot explanations, candidate rankings, replay exports, DOM/test-id/storage/logs). Gate 15 (hidden-information, 3–6 seats) needs one. This ticket builds that harness, parameterized by seat count and viewer, exercising the bridge's N-seat viewer projection (enabled by INFADNSEA-001) and using synthetic N-seat fixtures where no official >2-seat game exists yet.

## Assumption Reassessment (2026-06-14)

1. Current no-leak proof is per-game and inline: `crates/wasm-api/src/lib.rs` bridge tests use `get_view_for_viewer`/`get_action_tree_for_viewer` keyed to `observer/seat_0/seat_1`; per-game `games/*/tests/replay.rs` + `src/visibility.rs` carry viewer-scoped assertions. There is no `crates/wasm-api/tests/` dir — bridge tests are inline — so this harness lands inline `#[cfg(test)]` or as a new shared test module (spec hedged "or a shared test module").
2. Grounded in `specs/infra-a-d-n-seat-public-infrastructure.md` WB7, `docs/MULTI-SEAT-AND-SURFACE-CONTRACT.md §6` (pairwise no-leak matrix), and `docs/adr/0004-hidden-info-replay-export-taxonomy.md` (Accepted — the replay/export authority). The Phase 0 no-leak taxonomy (FOUNDATIONS §11 pairwise seat-private redaction) is already landed (archived `phase-0-next-phase-foundation-realignment` spec).
3. Shared boundary under audit: the viewer-scoped projection contract (`get_view_for_viewer` / `get_action_tree_for_viewer` / replay export) across every surface in contract §6 — the harness asserts no source-seat-A private datum reaches viewer-seat B unless Rust authorized it.
4. FOUNDATIONS §11 + §12 (no-leak firewall; "a 3+ seat game cannot prove viewer-safe public and per-seat projections" is a stop condition): this harness is the mechanism that discharges that stop condition for Gate 15. Restating it: hidden information must not leak through payloads, DOM, storage, logs, previews, effect logs, bot explanations, candidate rankings, or replay exports for any viewer.
5. No-leak firewall + deterministic replay surface under audit: the harness asserts the existing viewer-scoped projection leaks nothing and must itself introduce no nondeterminism (fixed seeds, deterministic seat ordering). It conforms to ADR 0004's export taxonomy (internal full traces may carry omniscient evidence; browser/default exports for hidden-info games are viewer-scoped). It adds assertions only — it changes no view-projection or replay/hash semantics (no §13 trigger).

## Architecture Check

1. One reusable seat-count-parameterized harness is cleaner than copying per-game two-seat assertions: it gives every current game (at 2 seats) and Gate 15 (at N seats) the same pairwise proof, and removes the per-game drift risk.
2. No backwards-compat shim: the harness is new; existing per-game tests remain until INFADNSEA-008 routes them through it. No production code changes.
3. `engine-core` untouched; no `game-stdlib` change. The harness lives in `wasm-api` test scope.

## Verification Layers

1. Pairwise no-leak across the contract-§6 surface set -> the harness asserts, for each (source seat A, viewer seat B≠A, surface), no unauthorized A-private datum appears (payload/action-tree/preview/effect/bot-explanation/candidate-ranking/replay-export/DOM-test-id).
2. Seat-count parameterization -> a synthetic N>2 fixture drives the harness (no official >2-seat game pre-Gate-15).
3. Determinism -> fixed seeds + deterministic seat ordering produce identical harness results across runs.
4. Conformance to ADR 0004 -> FOUNDATIONS/ADR alignment check: internal full traces may carry omniscient evidence; the harness asserts viewer-scoped *exports* redact unauthorized data.

## What to Change

### 1. Build the reusable pairwise no-leak harness

A seat-count- and viewer-parameterized harness (inline `#[cfg(test)]` in `crates/wasm-api/src/lib.rs`, or a new shared test module) that, given a game + seed + seat count, enumerates (source seat, viewer seat, surface) triples and asserts no unauthorized private-datum leak across the contract-§6 surfaces, conforming to ADR 0004's export taxonomy.

### 2. Synthetic N-seat fixture

A synthetic >2-seat fixture so the harness proves N-player behavior before any official N-seat game ships.

## Files to Touch

- `crates/wasm-api/src/lib.rs` (modify — add the harness inline under `#[cfg(test)]`; or introduce `crates/wasm-api/tests/no_leak_harness.rs` (new) if a shared module is cleaner)

## Out of Scope

- Adopting the harness for the existing hidden-info games + wiring web no-leak smoke (INFADNSEA-008).
- Any change to view projection, replay, or hash semantics (assertions only; spec §3.3).
- Any official >2-seat game (Gate 15); N>2 here is synthetic.

## Acceptance Criteria

### Tests That Must Pass

1. The harness runs over an existing hidden-info game at 2 seats and passes (no leak), and over a synthetic 3+-seat fixture and passes.
2. A deliberately-seeded leak (negative test) makes the harness fail — proving it is not vacuously green.
3. `cargo test -p wasm-api` — harness compiles and runs deterministically.

### Invariants

1. For every (source seat A, viewer seat B≠A, surface), no A-private datum reaches B unless Rust authorized it (§11 no-leak firewall; contract §6).
2. The harness is deterministic (fixed seeds, deterministic seat ordering) and changes no view/replay/hash semantics (ADR 0004 conformance; no §13 trigger).

## Test Plan

### New/Modified Tests

1. `crates/wasm-api/src/lib.rs` (`#[cfg(test)]`) or `crates/wasm-api/tests/no_leak_harness.rs` — the pairwise harness + a synthetic N-seat fixture + a negative (induced-leak) test.

### Commands

1. `cargo test -p wasm-api`
2. `cargo test --workspace`

## Outcome

Completed on 2026-06-14.

Added a reusable pairwise no-leak harness under `crates/wasm-api` test scope. The harness enumerates source-seat private terms against every non-own viewer surface and fails with the source seat, viewer, and surface name if a private token appears where it is not authorized.

Coverage added:

- Existing hidden-info bridge case: High Card Duel at two seats, covering viewer payloads, viewer-scoped action trees, effect logs after a private commit, observer replay export, and not-applicable/redacted placeholders for preview, bot explanation, candidate ranking, DOM/test-id, storage, and log surfaces.
- Synthetic N-seat case: deterministic four-seat fixture covering payload, action tree, preview, effect log, bot explanation, candidate ranking, replay export, DOM/test-id, storage, and log surfaces before an official >2-seat hidden-info game exists.
- Negative fixture: deliberately injects `seat_0` private data into a `seat_2` surface and asserts the harness fails, proving the harness is not vacuously green.

The harness is assertion-only; no view projection, replay/hash semantics, `engine-core`, or game behavior changed.

Verification:

- `cargo test -p wasm-api pairwise_no_leak_harness -- --nocapture`
- `cargo fmt --all`
- `cargo test -p wasm-api`
- `cargo test --workspace`

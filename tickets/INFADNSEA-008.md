# INFADNSEA-008: Infra D — adopt the no-leak harness + wire web no-leak smoke

**Status**: PENDING
**Priority**: HIGH
**Effort**: Medium
**Engine Changes**: Yes — `crates/wasm-api` / `games/*/tests` (route existing hidden-info no-leak proofs through the harness) + `apps/web/e2e/a11y-noleak.smoke.mjs` (presentation-layer smoke wiring)
**Deps**: INFADNSEA-005, INFADNSEA-007

## Problem

The reusable harness (INFADNSEA-007) must be adopted so it actually guards the catalog: the existing hidden-information games run through it at 2 seats (regression), and the web no-leak smoke (`apps/web/e2e/a11y-noleak.smoke.mjs`) is wired to cover the multi-seat shell frame (INFADNSEA-005) — so leaks through DOM/test-ids/storage on the seat surface are caught. The evidence must name the supported seat counts and the max-surface fixtures it covers (ROADMAP §15 Infra exit 4).

## Assumption Reassessment (2026-06-14)

1. The harness exists from INFADNSEA-007; the multi-seat `SeatFrame` exists from INFADNSEA-005. The web no-leak smoke is `apps/web/e2e/a11y-noleak.smoke.mjs` (exists; run via `smoke:e2e`). Hidden-info games with existing no-leak tests live under `games/*/tests/` (14 game test dirs).
2. Grounded in `specs/infra-a-d-n-seat-public-infrastructure.md` WB8, `docs/MULTI-SEAT-AND-SURFACE-CONTRACT.md §6`, and ADR 0004. ROADMAP §15 Infra exit 4 requires the evidence to name seat counts + max-surface fixtures.
3. Shared boundary under audit: the existing per-game no-leak assertions versus the shared harness — routing them through the harness must preserve every current assertion (no weakening) and add the pairwise/seat-frame coverage.
4. FOUNDATIONS §11 + §12: this ticket is where the no-leak firewall becomes continuously enforced for the seat surface; restating — no hidden information leaks through any payload/DOM/storage/log/effect/bot-explanation/replay-export surface for any viewer.
5. No-leak firewall surface under audit: the web smoke must assert the `SeatFrame` viewer selector exposes only the selected viewer's authorized data in the DOM/test-ids (no other seat's private data in the rendered tree); the Rust harness covers the bridge surfaces. Both are assertion-only — no view/replay/hash semantics change (ADR 0004 conformance; no §13 trigger).

## Architecture Check

1. Routing existing hidden-info games through the shared harness is cleaner than maintaining parallel per-game no-leak code: one harness, uniformly applied, with the per-game tests becoming thin harness invocations.
2. No backwards-compat shim: existing per-game no-leak assertions are migrated to the harness, not duplicated alongside it (no weakening — every prior assertion is preserved or strengthened).
3. `engine-core`/`game-stdlib` untouched; web smoke is presentation-layer only (§2).

## Verification Layers

1. Existing hidden-info games pass the harness at 2 seats -> `cargo test -p wasm-api` + affected `games/*/tests` green.
2. Seat-frame DOM/test-ids leak nothing -> `apps/web/e2e/a11y-noleak.smoke.mjs` extended to assert the `SeatFrame` viewer selector exposes only the selected viewer's authorized data.
3. Evidence names seat counts + max-surface fixtures -> the ticket's acceptance evidence enumerates the seat counts and largest fixtures the harness + smoke cover (ROADMAP §15 exit 4).
4. ADR 0004 conformance -> alignment check: viewer-scoped exports redact unauthorized data; internal full traces may stay omniscient.

## What to Change

### 1. Route existing hidden-info no-leak proofs through the harness

Migrate the existing hidden-information games' no-leak assertions to invoke the INFADNSEA-007 harness at 2 seats, preserving every current assertion.

### 2. Wire the web no-leak smoke to the seat frame

Extend `apps/web/e2e/a11y-noleak.smoke.mjs` to exercise the `SeatFrame` viewer selector and assert no non-selected-seat private data appears in the DOM/test-ids/storage; record seat-count + max-surface coverage in the evidence.

## Files to Touch

- `crates/wasm-api/src/lib.rs` (modify — route bridge no-leak tests through the harness) and affected `games/*/tests/*.rs` (modify; as surfaced — `games/*/tests/` is the verified parent, 14 dirs)
- `apps/web/e2e/a11y-noleak.smoke.mjs` (modify)

## Out of Scope

- Building the harness itself (INFADNSEA-007).
- The seat frame component/adoption (INFADNSEA-005/006).
- Any official >2-seat game (Gate 15); seat-count coverage here is current games at 2 + synthetic N from the harness.

## Acceptance Criteria

### Tests That Must Pass

1. `cargo test -p wasm-api` — existing hidden-info no-leak proofs pass through the harness with no weakened assertion.
2. `npm --prefix apps/web run smoke:e2e` — `a11y-noleak.smoke.mjs` covers the `SeatFrame` viewer selector; no non-selected-seat private data in the DOM.
3. `cargo test --workspace` — affected `games/*/tests` green.

### Invariants

1. Every prior per-game no-leak assertion is preserved or strengthened (no weakening to adopt the harness).
2. The no-leak evidence names the supported seat counts and the max-surface fixtures covered (ROADMAP §15 Infra exit 4); no view/replay/hash semantics change.

## Test Plan

### New/Modified Tests

1. `games/*/tests/*.rs` (as surfaced) — migrate hidden-info no-leak assertions to the harness.
2. `apps/web/e2e/a11y-noleak.smoke.mjs` — extend for the seat-frame viewer selector.

### Commands

1. `cargo test -p wasm-api && cargo test --workspace`
2. `npm --prefix apps/web run smoke:e2e`

# GAT10POKLITBET-007: Public/seat view projection, UI metadata, and no-leak tests

**Status**: PENDING
**Priority**: HIGH
**Effort**: Large
**Engine Changes**: Yes — `games/poker_lite/src/visibility.rs`, `games/poker_lite/src/ui.rs`, `games/poker_lite/tests/visibility.rs`. Consumes `engine-core` public/private view contract. No kernel change.
**Deps**: GAT10POKLITBET-006

## Problem

The viewer-safe projection is the no-leak firewall: an observer or opponent must never see either private card before showdown, the center card before its reveal, or the deck tail ever; the owning seat sees only its own private card. This ticket implements `PublicView`/`SeatPrivateView` projection, the Rust-owned UI metadata (`ui.rs`: neutral labels, rules summaries, accessibility copy — no casino language), and the exhaustive no-leak string-search tests over the projected JSON surfaces.

## Assumption Reassessment (2026-06-08)

1. The projection + ui.rs shape matches siblings: `games/secret_draft/src/visibility.rs` defines `PublicView`/`SeatPrivateView` with observer/seat projection and no-leak helpers, and `games/secret_draft/src/ui.rs` carries viewer-facing labels/accessibility metadata; `games/secret_draft/tests/visibility.rs` is the no-leak test precedent. The spec names `PublicView`, `SeatPrivateView` in §4 visibility.rs.
2. The spec (`specs/gate-10-poker-lite-betting-showdown.md` §B2 Public/seat view, §6 "Universal hidden-info no-leak exit criteria") fixes exactly what each viewer sees at each phase: observer-before-reveal (counts + `center: hidden`), seat-before-showdown (+ own card + own strength bucket), observer-after-center-reveal (center id, no privates, deck tail hidden), showdown (both privates + center + comparator), yield terminal (no private reveal; center only if already revealed).
3. Cross-artifact boundary under audit: `engine-core`'s public/private view contract (authoritative in `docs/ENGINE-GAME-DATA-BOUNDARY.md`) and the internal/public state split from `state.rs` (003) + effect scopes from `effects.rs` (006). Projection drops hidden fields; it must not be a redaction of a shared blob. The own-seat strength bucket (`low_private`/`middle_private`/`paired_high`…) is private to owner and must never appear in observer/opponent views.
4. FOUNDATIONS §11 (public/private views are viewer-safe; hidden info does not leak) and §7 (public UI is play-first, viewer-safe) motivate this ticket. Restated before trusting the spec narrative.
5. No-leak firewall surface under audit (§11/§12) — this is the primary enforcement ticket: no observer/opponent/projection JSON may contain a hidden private card id/rank/label, the center card before reveal, the deck tail, or the owner-only strength bucket. The tests here are exhaustive string-search over view JSON; action-tree/effect/diagnostic JSON were allow-listed in 004/006 and are re-swept here. Confirm projection physically omits (not merely masks) hidden fields.

## Architecture Check

1. Projection-by-omission (build the public view from only public fields) is strictly safer than redaction (scrub a serialized internal state), because a newly added internal field cannot leak unless explicitly projected. Matches the sibling firewall design.
2. No backwards-compatibility aliasing/shims — new modules.
3. `engine-core` stays noun-free (views are crate-local typed projections over the generic view contract, §3); `ui.rs` is viewer-facing Rust output (labels/accessibility), not TS-owned legality (§2); no `game-stdlib` promotion (§4).

## Verification Layers

1. Per-phase viewer correctness (each viewer sees exactly the §B2 field set) -> `cargo test -p poker_lite --test visibility` per-phase projection tests.
2. No-leak string search (no hidden id/rank/label/deck-tail/strength-bucket in observer/opponent JSON before reveal) -> exhaustive string-search no-leak tests over view JSON.
3. Owner-only strength bucket isolation -> projection test asserting the bucket appears in seat view only.
4. UI metadata neutrality (no casino/poker/chip/payout terms in labels/accessibility copy) -> manual IP/UI review + grep-proof over `ui.rs` strings.

## What to Change

### 1. `games/poker_lite/src/visibility.rs`

Implement `PublicView`, `SeatPrivateView`, and observer/seat projection per §B2, with stable summaries and no-leak helpers. Project by omission of hidden fields; gate center on `center_visible`, privates on showdown, deck tail never.

### 2. `games/poker_lite/src/ui.rs`

Neutral display labels, rules summaries, accessibility copy, viewer-mode UI metadata. No casino/poker/chip/payout/ante/blind/rake language (spec §E `effectFeedback` note + §2 Objective neutral terms).

### 3. `games/poker_lite/tests/visibility.rs` (new)

Per-phase projection tests + exhaustive no-leak string search over view/action-tree/effect/diagnostic JSON for every hidden card id, rank label, sigil label, deck-tail id, and owner strength bucket.

## Files to Touch

- `games/poker_lite/src/visibility.rs` (new)
- `games/poker_lite/src/ui.rs` (new)
- `games/poker_lite/tests/visibility.rs` (new)
- `games/poker_lite/src/lib.rs` (modify — add `mod visibility; mod ui;` + re-exports)

## Out of Scope

- Property tests (GAT10POKLITBET-008).
- Replay export redaction (GAT10POKLITBET-009) — public-export no-leak is verified there.
- Browser DOM / `data-testid` / local-storage no-leak (GAT10POKLITBET-016) — those are web-surface sweeps.
- The TS renderer (GAT10POKLITBET-015).

## Acceptance Criteria

### Tests That Must Pass

1. `cargo test -p poker_lite --test visibility` — per-phase projection + exhaustive no-leak string search.
2. Owner-strength-bucket isolation test: bucket present in seat view, absent from observer/opponent views.
3. UI-neutrality grep: no casino/poker/chip/payout/ante/blind/rake term in `ui.rs`.

### Invariants

1. Before its rule-defined reveal point, no hidden private card, center card, deck tail, or owner strength bucket appears in any observer/opponent projection (§11 no-leak firewall).
2. Hidden fields are physically omitted from public projections, not masked in a shared serialized blob (§11).

## Test Plan

### New/Modified Tests

1. `games/poker_lite/tests/visibility.rs` — per-phase projection + no-leak string search + strength-bucket isolation.
2. `games/poker_lite/src/ui.rs` (inline `#[cfg(test)]`) — label/accessibility neutrality.

### Commands

1. `cargo test -p poker_lite --test visibility`
2. `cargo test -p poker_lite`
3. `bash scripts/boundary-check.sh`

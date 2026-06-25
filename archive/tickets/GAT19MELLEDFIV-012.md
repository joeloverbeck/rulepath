# GAT19MELLEDFIV-012: Visibility — public-observer and seat-private view projection with action/preview/effect redaction

**Status**: COMPLETED
**Priority**: HIGH
**Effort**: Large
**Engine Changes**: Yes — `games/meldfall_ledger/src/visibility.rs`; no-leak view tests
**Deps**: GAT19MELLEDFIV-011

## Problem

Meldfall Ledger is a hidden-information game: each seat's hand and the unseen stock order are hidden. This ticket implements the viewer-scoped projection in `visibility.rs` — a public observer view (public meld tableau, public discard pile, stock count only, hand counts, scores, active seat, dealer/turn order, public effects) and per-seat-private views (own hand only) — plus redaction of the action tree, action previews, and semantic effects so hidden card identities never reach a viewer the projection forbids. This is the §11 no-leak firewall for in-state surfaces; the replay-export firewall is GAT19MELLEDFIV-013.

## Assumption Reassessment (2026-06-25)

1. `games/river_ledger/src/visibility.rs` (per-seat hidden-info filtering + public/seat-private projection) is the closest pattern; `crates/game-test-support` no-leak utilities are reused in tests (confirmed during reassessment). `visibility.rs` is a stub from GAT19MELLEDFIV-003; state/effects/actions exist from 005–011.
2. Spec §3.1 (Visibility row), §7.3 (authorized public facts / hidden facts lists), and the Header hidden-information stance define the projection; Appendix B.4 (a11y labels never include opponent hand text) constrains downstream rendering.
3. Cross-artifact: the public/private view contract (`docs/ENGINE-GAME-DATA-BOUNDARY.md`) and the action-tree/effect-envelope contracts are the boundaries under audit — projection must redact at the view, action-tree, preview, and effect layers, not just the top-level view.
4. FOUNDATIONS §11 no-leak firewall: hidden information must not reach view JSON, action tree, previews, diagnostics, or semantic effects for a forbidden viewer; this is the central acceptance invariant for this ticket. §12 makes a leak a stop condition.
5. FOUNDATIONS §2: projection is Rust-owned; the browser renders only what Rust already made viewer-safe — no TypeScript redaction.

## Architecture Check

1. Centralizing all redaction in `visibility.rs` (view + action-tree + preview + effect projection) gives a single audited firewall rather than per-surface guards that can drift.
2. No backwards-compatibility shims.
3. `engine-core` untouched; projection logic is crate-local; the no-leak geometry reuses behavior-free test-support utilities (MSC-8C-007), not a new helper.

## Verification Layers

1. Public observer sees only the §7.3 authorized public facts (counts, not hands; stock count, not order) -> no-leak visibility test on the public projection.
2. Seat-private view shows only the viewer's own hand; opponents' hands redacted to counts -> no-leak visibility test across seats.
3. Action tree, previews, and effects carry no hidden identities for a forbidden viewer -> redaction tests on each surface (full six-seat pairwise matrix + exports in GAT19MELLEDFIV-013).

## What to Change

### 1. `visibility.rs` — view projection

Public-observer and seat-private view projection: public tableau + discard pile + stock count + hand counts + scores + active seat + dealer + public effects; seat-private adds only the viewer's own hand.

### 2. Action-tree / preview / effect redaction

Redact the legal action tree, action previews, and semantic effects per viewer so a stock-draw card is visible only to the acting seat, opponents' hand cards never appear, and diagnostics name no hidden cards.

### 3. No-leak view tests

`tests/visibility.rs` assertions over view JSON, action-tree JSON, preview JSON, diagnostics, and effects for the public observer and each seat-private viewer (single/representative seats here; the full six-seat pairwise matrix is GAT19MELLEDFIV-013).

## Files to Touch

- `games/meldfall_ledger/src/visibility.rs` (modify; created by GAT19MELLEDFIV-003)
- `games/meldfall_ledger/tests/visibility.rs` (modify; created by GAT19MELLEDFIV-003)

## Out of Scope

- Replay export/import and the six-seat pairwise export matrix (GAT19MELLEDFIV-013).
- Browser DOM/a11y/storage no-leak (GAT19MELLEDFIV-020/021).

## Acceptance Criteria

### Tests That Must Pass

1. `cargo test -p meldfall_ledger` visibility tests: public observer view exposes only §7.3 authorized public facts.
2. Seat-private view shows only the viewer's own hand; opponents' hands and stock order are redacted on the view, action tree, preview, and effect surfaces.
3. `cargo test --workspace` passes.

### Invariants

1. No hidden card identity reaches a forbidden viewer through any in-state surface (FOUNDATIONS §11 no-leak firewall; §12).
2. Projection is Rust-owned; the browser receives only viewer-safe payloads (FOUNDATIONS §2).

## Test Plan

### New/Modified Tests

1. `games/meldfall_ledger/tests/visibility.rs` — public-observer + seat-private projection, action-tree/preview/effect redaction assertions.

### Commands

1. `cargo test -p meldfall_ledger`
2. `cargo test --workspace`
3. The full six-seat pairwise matrix + replay-export no-leak is GAT19MELLEDFIV-013; this ticket fixes the in-state projection firewall.

## Outcome

Completed: 2026-06-26

Implemented Rust-owned Meldfall Ledger in-state visibility projection in `games/meldfall_ledger/src/visibility.rs`: public observer and seat-private views expose public tableau, discard identities, stock count, hand counts, scores, active/dealer seats, round-end, terminal standings, and only the authorized viewer's own private hand.

Added action-tree projection that keeps active-seat card-bearing meld, lay-off, and discard choices visible only to the active seat while retaining public-safe draw source choices with stock identity hidden. Added semantic-effect filtering by `VisibilityScope` and diagnostic card-id redaction for forbidden viewers.

Added `games/meldfall_ledger/tests/visibility.rs` coverage for observer no-leak, per-seat private view no-leak, active-hand action redaction, public draw action-tree stock-order redaction, private stock-draw effect filtering, and diagnostic redaction. The full six-seat pairwise replay/export matrix remains scoped to GAT19MELLEDFIV-013.

Verification:

1. `cargo fmt --all --check`
2. `cargo test -p meldfall_ledger`
3. `cargo test --workspace`

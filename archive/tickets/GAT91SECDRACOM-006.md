# GAT91SECDRACOM-006: secret_draft visibility/no-leak projection (visibility.rs) + UI metadata (ui.rs)

**Status**: ✅ COMPLETED
**Priority**: HIGH
**Effort**: Large
**Engine Changes**: Yes — `games/secret_draft/src/visibility.rs` and `src/ui.rs`; no `engine-core` / `game-stdlib` change.
**Deps**: GAT91SECDRACOM-005

## Problem

This ticket implements the gate's central no-leak guarantee: public and seat views, and viewer-safe UI metadata, that expose the visible pool, drafted collections, pending booleans, scores, revealed history, and terminal outcome — but NEVER a committed item ID before reveal, not even to the committing seat (spec A6). Effect filtering ensures pre-reveal effects reaching any viewer carry no hidden choice.

## Assumption Reassessment (2026-06-08)

1. `games/high_card_duel/src/visibility.rs` is the direct behavioral template (verified): it defines `CommitmentViews`, `PrivateView`, `RevealedRoundView`, and uses `face_down_commitment_label` / redaction helpers. `secret_draft` adapts this rather than copying `token_bazaar`'s non-hidden-info projection (spec §Deliverables WB-item-5 note, as updated by reassessment).
2. State + effects from GAT91SECDRACOM-003/005 are inputs (`commitments` internal, the effect set with pre-reveal payload rules). Spec §"Visibility and no-leak model" defines public-observer view contents (game/round/phase/priority/visible pool/drafted/pending booleans/scores/revealed history/terminal) and the must-not-contain list (`commitments` values, raw submitted action paths for committed seats, own/other hidden item in command summaries, bot candidate rankings from hidden choices, seed material reconstructing hidden commands).
3. Cross-artifact boundary under audit: the public/private view contract (`docs/ENGINE-GAME-DATA-BOUNDARY.md`, `docs/ARCHITECTURE.md`) and the effect-filtering surface. `ui.rs` is viewer-facing Rust output (labels/token/accessibility metadata/preview copy) co-located here per `docs/OFFICIAL-GAME-CONTRACT.md`; it is distinct from the TS renderer (GAT91SECDRACOM-015).
4. §11 no-leak firewall is the motivating invariant: restate before trusting spec — hidden information must not reach any viewer the deterministic views forbid, through views, previews, diagnostics, effect logs, command summaries, candidate rankings, or UI test IDs. Spec A6 makes this stricter: even the committing seat's pre-reveal view shows only "You have committed," never the chosen item ID, so DOM/storage no-leak tests stay clean even in hotseat.
5. Determinism: projections are pure functions of state; stable field/iteration order (visible pool in stable item order). No nondeterministic input enters a view.

## Architecture Check

1. A single projection layer that derives both public and seat views from internal state — with `commitments` never read into a pre-reveal view — is cleaner than per-call redaction, making the firewall structural and auditable.
2. No backwards-compatibility aliasing/shims — fills GAT91SECDRACOM-002 stubs.
3. `engine-core` stays noun-free (generic visibility-scope/public-private-view contracts reused); draft nouns stay game-local. No `game-stdlib` helper; high_card_duel commitment-view machinery is adapted in-crate, not promoted.

## Verification Layers

1. Pre-reveal no-leak (public + seat views) -> no-leak unit tests: neither view contains any committed item ID before `ChoicesRevealed`, including the committing seat's own view (A6).
2. Effect filtering -> unit test: viewer-scoped effect stream pre-reveal carries pending booleans only, no item ID.
3. Post-reveal correctness -> unit test: after reveal, item IDs appear in revealed history / drafted collections as expected.
4. UI metadata safety -> manual review + grep that `ui.rs` preview/accessibility copy carries no hidden choice.
5. Schema conformance -> view shape validated against `docs/ENGINE-GAME-DATA-BOUNDARY.md`.

## What to Change

### 1. `src/visibility.rs`

Implement public-observer and seat-private view projection from internal state, plus viewer-scoped effect filtering. `commitments` are read only after reveal. Seat-private pre-reveal view shows a "committed" flag, not the item ID (A6). Provide no-leak helper(s) used by tests and WASM (GAT91SECDRACOM-013).

### 2. `src/ui.rs`

Viewer-safe UI metadata: tile labels, pending/waiting copy, priority marker, score/tie-break presentation data, accessibility labels, reveal-batch grouping hints, and preview copy — all derived from the safe views, never from `commitments` pre-reveal.

## Files to Touch

- `games/secret_draft/src/visibility.rs` (modify)
- `games/secret_draft/src/ui.rs` (modify)

## Out of Scope

- Replay export/import (GAT91SECDRACOM-007) — though it consumes these views.
- The full no-leak negative test suite across command summaries / candidate rankings / replay exports (GAT91SECDRACOM-009); this ticket carries the view/effect-level no-leak tests.
- TS renderer and DOM/storage no-leak e2e (GAT91SECDRACOM-015/016).

## Acceptance Criteria

### Tests That Must Pass

1. `cargo test -p secret_draft visibility` — public + seat-view no-leak tests pass (including A6 committing-seat redaction).
2. Viewer-scoped pre-reveal effect filtering test passes (booleans only).
3. Post-reveal view exposes revealed item IDs correctly.

### Invariants

1. No committed item ID in any view or viewer-bound effect before reveal, including the committing seat's view (§11 no-leak, A6).
2. Projections are deterministic, stable-ordered pure functions of state (§11).

## Test Plan

### New/Modified Tests

1. `games/secret_draft/src/visibility.rs` inline no-leak unit tests — public/seat pre-reveal redaction, effect filtering, post-reveal exposure.

### Commands

1. `cargo test -p secret_draft visibility`
2. `cargo test --workspace && bash scripts/boundary-check.sh`
3. Targeted `visibility` filter is the correct boundary; cross-surface no-leak (exports, command summaries, DOM) is proven in GAT91SECDRACOM-007/009/016.

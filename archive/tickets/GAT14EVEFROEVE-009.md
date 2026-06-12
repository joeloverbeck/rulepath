# GAT14EVEFROEVE-009: Visibility, hidden deck order, and replay surfaces

**Status**: COMPLETED
**Priority**: HIGH
**Effort**: Medium
**Engine Changes**: Yes — `games/event_frontier/src/{visibility,replay_support}.rs` (output-equivalent projections, hidden-order firewall, replay export/import, trace markers)
**Deps**: GAT14EVEFROEVE-008

## Problem

The gate has exactly one hidden surface — **undrawn deck order** — hidden from all seats and observers symmetrically, mirroring `flood_watch`'s ADR 0004 posture. This ticket builds the single public projection (map, components, resources, scores, eligibility, current/next cards, discards, active edicts, epoch/Reckoning progress, victory-distance summaries, terminal outcome) that is output-equivalent across all viewers, the effect filtering, stable summaries/hashes, and replay export/import under the ADR 0004 taxonomy — proving no projection, action tree, preview, diagnostic, effect, or export ever contains undrawn deck order.

## Assumption Reassessment (2026-06-12)

1. The state, effects, and terminal this projects exist: verified ticket 004's state (incl. the internal `undrawn` field), tickets 005–008's effects (`CardRevealed`, `OpResolved`, `ReckoningResolved`, `Terminal`, …), all marked public with the hidden surface never in a payload. The generic public/private view + replay/export contracts come from `engine-core` / ADR 0004 as `games/flood_watch` proved.
2. The ADR 0004 posture is current: verified `docs/adr/0004-hidden-info-replay-export-taxonomy.md` exists and `flood_watch` registers a single symmetric hidden surface; verified the serialize-and-search no-leak doctrine in `docs/TESTING-REPLAY-BENCHMARKING.md` §8. This ticket mirrors `flood_watch`'s registration, markers, and export redaction with no new taxonomy category.
3. Cross-crate boundary under audit: `get_view` must be output-equivalent across seats and observer (all see the same projection); the undrawn-order field must be excluded from every projection, effect, diagnostic, and export. Stable summary/hash ordering is required for replay determinism.
4. FOUNDATIONS §11 (viewer-safe views; hidden information does not leak) and §12 (hidden info reaching payloads/exports is a stop condition) motivate this ticket. Restated before trusting the spec: the no-leak firewall covers payloads, previews, diagnostics, effect logs, bot explanations, candidate rankings, DOM/test-IDs, and replay exports; the only hidden surface is undrawn order.
5. No-leak + determinism surface (§11): this is the firewall enforcement point. Confirm serialize-and-search tests (ticket 011) can prove no payload/preview/diagnostic/export contains undrawn order; confirm export follows ADR 0004 (public timeline with the hidden-order surface redacted/recorded per taxonomy) and that replay reproduces hashes deterministically. No replay/hash *semantics* change — ADR 0004 is reused, not extended.

## Architecture Check

1. One public projection for all viewers (rather than per-seat redaction) is the simplest correct design for a single symmetric hidden surface: equivalence is the test, and there is no per-seat private holding to redact.
2. No backwards-compatibility aliasing/shims — fills the visibility/replay stubs; reuses ADR 0004.
3. `engine-core` stays noun-free; no `game-stdlib` promotion.

## Verification Layers

1. Output-equivalent projections (§11) -> a visibility test that seat-0, seat-1, and observer `get_view` outputs are identical.
2. Hidden-order firewall (§11/§12) -> serialize-and-search tests that no projection, action tree, preview, diagnostic, effect, or replay export contains the undrawn order (per `docs/TESTING-REPLAY-BENCHMARKING.md` §8).
3. Replay/export determinism -> a replay test that seed + scenario + command stream reproduce state/effect/action-tree/view hashes and the export round-trips under ADR 0004.
4. Trace markers -> golden traces (ticket 011) carry Trace Schema v1 §5 stochastic (setup shuffle) + hidden-info (undrawn order) markers; here, confirm the marker emission path exists.

## What to Change

### 1. Public projection (`src/visibility.rs`)

Build the single public view (all the public surfaces listed in the Problem) excluding `undrawn`; make `get_view` output-equivalent across seats and observer; filter effects to public payloads; produce stable summaries.

### 2. Replay/export (`src/replay_support.rs`)

Implement replay reproduction (seed + scenario + command stream → full timeline) and export/import under the ADR 0004 taxonomy mirroring `flood_watch` (public timeline; undrawn-order surface redacted/recorded per taxonomy); emit Trace Schema v1 §5 stochastic + hidden-info markers with per-seat surfaces marked not applicable.

## Files to Touch

- `games/event_frontier/src/visibility.rs` (modify; created by 003)
- `games/event_frontier/src/replay_support.rs` (modify; created by 003)

## Out of Scope

- The full visibility/replay test suite and golden traces (ticket 011) — this ticket builds the surfaces they exercise.
- WASM `get_view`/export wiring (ticket 014) and the browser presentation (ticket 017).
- Any new replay/trace/export infrastructure beyond what `flood_watch` proved (spec Out of scope) — presentation/markers within existing contracts only.

## Acceptance Criteria

### Tests That Must Pass

1. `cargo test -p event_frontier` passes the seat/observer output-equivalence visibility test.
2. Serialize-and-search no-leak tests pass: no payload/preview/diagnostic/effect/export contains undrawn deck order.
3. The replay round-trip test reproduces all hashes and the ADR 0004 export.

### Invariants

1. `get_view` is output-equivalent across all viewers; the current/next-card surfaces show exactly the public cards.
2. Undrawn deck order appears in no projection, effect, diagnostic, or export.

## Test Plan

### New/Modified Tests

1. `games/event_frontier/tests/visibility.rs` — output-equivalence + serialize-and-search no-leak.
2. `games/event_frontier/tests/replay.rs` — replay reproduction + ADR 0004 export round-trip.

### Commands

1. `cargo test -p event_frontier --test visibility --test replay`
2. `cargo test -p event_frontier`
3. The per-crate visibility/replay tests are the correct boundary; the WASM/DOM no-leak surface is additionally exercised by the browser smoke (ticket 018).

## Outcome

- Implemented a single public projection for Event Frontier that exposes public map, resources, scores, eligibility, current/next/discard card surfaces, active edicts, reckoning progress, victory distance, terminal state, and freshness without exposing `deck.undrawn`.
- Added public effect filtering/text, stable public view summaries/hashes, ADR 0004-style public replay export/import, and internal trace markers for `undrawn_deck_order` plus `setup_shuffle`.
- Added visibility and replay tests for seat/observer output equivalence, serialize-and-search hidden-order firewall coverage, public export/import hash reproduction, replayed state/effect/action-tree/view hash determinism, and hidden/stochastic marker emission.

## Verification

- `cargo fmt --all --check`
- `cargo test -p event_frontier --test visibility --test replay`
- `cargo test -p event_frontier`

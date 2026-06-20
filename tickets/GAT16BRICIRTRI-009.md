# GAT16BRICIRTRI-009: Four-seat visibility and semantic-effect boundary

**Status**: PENDING
**Priority**: HIGH
**Effort**: Large
**Engine Changes**: Yes — `games/briar_circuit/src/visibility.rs`, `src/effects.rs` (+ private action-tree/preview filtering)
**Deps**: 006, 007, 008

## Problem

This ticket builds the no-leak firewall: a public-observer projection and four seat-private projections derived in Rust, deterministically-ordered public and private semantic effects, and private action trees/previews — proven by all 12 ordered pairwise seat checks plus observer checks over every required surface. Unplayed cards, pass selections/provenance, and deck order are owner-private; a card identity becomes public only when legally played, and that never reveals who passed it.

## Assumption Reassessment (2026-06-20)

1. The pass phase (006), play rules (007), and scoring/outcome (008) produce the state this ticket projects; `visibility.rs` and `effects.rs` were stubbed in GAT16BRICIRTRI-004. `engine-core`'s `VisibilityScope { Public, PrivateToSeat(SeatId) }` and effect/view contracts (`crates/engine-core/src/lib.rs`) are the generic projection envelopes — no new kernel concept.
2. `specs/gate-16-briar-circuit-trick-taking.md` §3.1 (Public/Private information rows), §4.2 (Effects/Views rows), §7.3 (viewer matrix), §7.4 (pairwise matrix — 12 ordered seat pairs + 4 observer), §7.5 (no-leak surfaces/datum taxonomy), Appendix A `BC-VIS-001..004`, and Appendix B.4 (public vs private effect lists) fix the contract.
3. Cross-artifact boundary under audit: the Rust projection is generated *before* serialization; the same filtering must hold for views, action trees, previews, diagnostics, and effects. Effect filters are tested independently from view filters (an effect hidden from rendering but present in JSON is a leak).
4. FOUNDATIONS §11 no-leak firewall and §12 ("hidden information reaches browser payloads…") are the principles under audit: every ordered source-seat → unauthorized-viewer pair, plus the observer, must receive no private datum (unplayed cards, staged/committed pass cards, incoming cards pre-exchange, pass provenance, private trees/previews/diagnostics/effects, or any seed/deck fact reconstructing them). Canary identifiers are searched in structured values and serialized strings.

## Architecture Check

1. Deriving one public projection + four owner-only projections in Rust (over filtering in TypeScript) keeps the firewall on the authority side; the browser receives already-safe payloads.
2. No backwards-compatibility aliasing/shims — fills the visibility/effects stubs.
3. `engine-core` stays free of mechanic nouns; visibility uses the generic `VisibilityScope` (§3). No `game-stdlib` change (§4); the game reuses generic viewer/effect infrastructure only.

## Verification Layers

1. Only the owner view contains owner hand/selection; counts public, identities not -> `tests/visibility.rs` per-seat projection check (`BC-VIS-001/002`).
2. All 12 ordered pairwise seat checks + 4 observer checks over views/trees/previews/diagnostics/effects -> `tests/visibility.rs` canary matrix (§7.4).
3. Public/private effects deterministically ordered; private effects correctly scoped; effect filter tested independently from view filter -> `tests/visibility.rs` effect-scope check + golden no-leak traces.
4. Card identity becomes public only via a legal public play effect; pass provenance never public -> `tests/visibility.rs` (`BC-VIS-002`).

## What to Change

### 1. `games/briar_circuit/src/visibility.rs`

Public-observer and four seat-private view projections; private action-tree and preview filtering; owner-specific diagnostics that never name the actual owner of an unowned card.

### 2. `games/briar_circuit/src/effects.rs`

Deterministically-ordered public semantic effects (counts/status only for commitments) and private effects (own deal/pass/preview), scoped per recipient; never animation instructions.

### 3. No-leak golden traces

`public-observer-no-leak` and `seat-private-pairwise-no-leak` (all four viewer hashes + canaries) golden traces capturing the firewall (validated by `replay-check` in GAT16BRICIRTRI-012).

## Files to Touch

- `games/briar_circuit/src/visibility.rs` (modify; created by 004)
- `games/briar_circuit/src/effects.rs` (modify; created by 004, extended by 006)
- `games/briar_circuit/tests/visibility.rs` (modify; created by 006)
- `games/briar_circuit/tests/golden_traces/public-observer-no-leak.trace.json` (new)
- `games/briar_circuit/tests/golden_traces/seat-private-pairwise-no-leak.trace.json` (new)

## Out of Scope

- Internal full traces, viewer-scoped export/import, and the behavioral trace pack (GAT16BRICIRTRI-010).
- The WASM pairwise no-leak harness dispatch (GAT16BRICIRTRI-013) and browser DOM/storage no-leak (GAT16BRICIRTRI-015).
- Bot candidate/explanation no-leak (GAT16BRICIRTRI-011).

## Acceptance Criteria

### Tests That Must Pass

1. `cargo test -p briar_circuit --test visibility` — all 12 ordered pairwise + 4 observer canary checks pass over every §7.5 surface.
2. `cargo test -p briar_circuit` — full crate green including effect-scope independence.
3. `cargo run -p replay-check -- --game briar_circuit` (after 012 registration) validates the no-leak golden traces — deferred check noted.

### Invariants

1. No private datum reaches an unauthorized viewer before serialization (§11 no-leak firewall).
2. Effects are filtered independently of views; a JSON-present but render-hidden datum is a failing leak (§11).

## Test Plan

### New/Modified Tests

1. `games/briar_circuit/tests/visibility.rs` — 12 pairwise + observer canary matrix across all surfaces.
2. `games/briar_circuit/tests/golden_traces/public-observer-no-leak.trace.json` — observer no-leak.
3. `games/briar_circuit/tests/golden_traces/seat-private-pairwise-no-leak.trace.json` — four viewer hashes + canaries.

### Commands

1. `cargo test -p briar_circuit --test visibility`
2. `cargo test -p briar_circuit`
3. The native pairwise matrix is the correct boundary here; WASM-payload and DOM/storage no-leak are proven in 013/015 against the same canaries.

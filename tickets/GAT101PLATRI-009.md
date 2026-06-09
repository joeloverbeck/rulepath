# GAT101PLATRI-009: Public/seat view projection, UI metadata, and no-leak tests

**Status**: PENDING
**Priority**: HIGH
**Effort**: Large
**Engine Changes**: Yes — `games/plain_tricks/src/visibility.rs`, `games/plain_tricks/src/ui.rs`, `games/plain_tricks/tests/visibility.rs`. No `engine-core`/`game-stdlib` change.
**Deps**: GAT101PLATRI-008

## Problem

The game needs viewer-safe public/seat view projection (own hand to owner only; opponent hand as count only; tail never visible; played cards public from play; trick history/scores public) plus Rust-owned UI metadata (neutral labels, rule summaries, accessibility copy), and the exhaustive no-leak test suite. This is the central hidden-information firewall the gate exists to prove.

## Assumption Reassessment (2026-06-09)

1. `games/plain_tricks/src/{state,effects}.rs` (GAT101PLATRI-005/008) hold internal hands/tail and viewer-scoped effects. The public/private view contract comes from `engine-core` (`PublicView`/private view, `VisibilityScope`); mirror `games/poker_lite/src/visibility.rs` + `ui.rs`.
2. Spec §5 item 7 and appendix A5 fix the visibility model: own hand owner-only; opponent count-only; tail never visible (incl. terminal + seat exports); played cards public from play; voids revealed only implicitly by off-suit play (no explicit void flag); scores/trick counts/round index/leader/turn state public; observer view = counts + public trick surface + history + scores; seat view = observer fields + own hand.
3. Shared boundary under audit: the public/private view schema (`engine-core` / `docs/ENGINE-GAME-DATA-BOUNDARY.md`). `ui.rs` is viewer-facing Rust UI metadata (labels/accessibility), distinct from the TS renderer (GAT101PLATRI-017).
4. FOUNDATIONS §11 (public/private views viewer-safe; hidden info never leaks) and §2 (Rust owns view projection) are under audit.
5. Enforcement surface: §11 no-leak firewall — this is the primary enforcement ticket. Every unplayed card id and suit/rank label from either hand and every tail card must be absent from observer view JSON, non-actor action-tree JSON, effect JSON, and diagnostic JSON. Off-suit play revealing a void is rule-implied public information and is documented/tested as such. Deterministic projection only (no nondeterministic input).
6. Extends the public/private view contract additively (a new game's view types); consumers are replay export (GAT101PLATRI-011), the WASM bridge (GAT101PLATRI-016), and the renderer (GAT101PLATRI-017).

## Architecture Check

1. Building observer/seat projection as the single Rust source of viewer-safe truth (vs. redacting in TS) keeps the firewall in Rust where it belongs (FOUNDATIONS §2/§11); TS only renders what projection emits.
2. No backwards-compatibility aliasing/shims.
3. `engine-core` untouched and noun-free; view payload types are `plain_tricks`-local. No `game-stdlib` change.

## Verification Layers

1. Own hand visible only to owner; opponent count-only; tail never visible -> no-leak visibility tests (exhaustive string-search) on observer + seat view JSON.
2. Played cards public from play; trick history/scores public -> view unit tests + golden traces `no-leak-public-observer`, `seat-private-view`, `deal-private-no-leak`.
3. Void revealed only implicitly by off-suit play (no explicit void flag) -> targeted view test.
4. UI metadata is neutral and viewer-safe (no raw hidden ids) -> manual review + no-leak test on `ui.rs` summaries.

## What to Change

### 1. `games/plain_tricks/src/visibility.rs`

Implement `PublicView` (counts, public trick surface, history, scores; no hand identities), `SeatPrivateView` (observer fields + own hand), observer/seat projection, stable summaries, and no-leak helpers. The tail is never projected to anyone.

### 2. `games/plain_tricks/src/ui.rs`

Neutral display labels (`Gale`/`River`/`Ember`, ranks 1–6), rule summaries, and accessibility copy — viewer-safe Rust UI metadata only.

### 3. `games/plain_tricks/tests/visibility.rs`

Exhaustive string-search no-leak tests for every unplayed card id and suit/rank label across observer view JSON, non-actor action-tree JSON, effect JSON, and diagnostic JSON. Document/test that off-suit play publicly revealing a void is rule-implied public info.

## Files to Touch

- `games/plain_tricks/src/visibility.rs` (new)
- `games/plain_tricks/src/ui.rs` (new)
- `games/plain_tricks/tests/visibility.rs` (new)

## Out of Scope

- Replay export/import no-leak (GAT101PLATRI-011) and browser-surface no-leak (GAT101PLATRI-018) — those extend this firewall to their surfaces.
- Property tests (GAT101PLATRI-010).
- The TS renderer (GAT101PLATRI-017).

## Acceptance Criteria

### Tests That Must Pass

1. `cargo test -p plain_tricks --test visibility`: no unplayed card id / suit-rank label from either hand or the tail appears in observer view, non-actor action-tree, effect, or diagnostic JSON.
2. Seat private view contains only that seat's own hand; observer view contains no hand identities.
3. The tail never appears in any view, including at terminal.

### Invariants

1. Hidden information never leaks through any view/effect/diagnostic payload (FOUNDATIONS §11 no-leak firewall).
2. A card identity becomes public exactly when played, through a public effect and the public view (FOUNDATIONS §11; spec §6).

## Test Plan

### New/Modified Tests

1. `games/plain_tricks/tests/visibility.rs` — exhaustive no-leak string searches across views/action-trees/effects/diagnostics.
2. `seat-private-view` / `no-leak-public-observer` / `deal-private-no-leak` golden-trace fixtures (authored in GAT101PLATRI-011) anchored here.

### Commands

1. `cargo test -p plain_tricks --test visibility`
2. `cargo test -p plain_tricks`
3. Per-crate visibility scope is the correct boundary for the Rust firewall; the browser/export extensions of the firewall are verified in GAT101PLATRI-011/018.

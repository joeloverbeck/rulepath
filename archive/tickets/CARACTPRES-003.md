# CARACTPRES-003: Flood Watch presentation metadata parity

**Status**: COMPLETED
**Priority**: MEDIUM
**Effort**: Medium
**Engine Changes**: Yes — `games/flood_watch` (additive public-view extension, static presentation-data loader, `ui.rs` metadata channel); no `engine-core` or `game-stdlib` changes
**Deps**: None (mirrors CARACTPRES-001's pattern; independent crate, no shared files)

## Problem

Flood Watch has the same card-meaning gap the spec verified for Event Frontier: `PublicView` projects `drawn_cards: Vec<String>`, `forecast: Option<String>` as raw card IDs (`games/flood_watch/src/visibility.rs:28-29,97-98`), `src/ui.rs` is a display-name stub, and the board renders the raw ID (`view.forecast ?? "None"` at `apps/web/src/components/FloodWatchBoard.tsx:103`). Spec WB3 brings the Gate 12 event-deck game to Workstream A parity: authored card presentation metadata, fail-closed loader, `ui.rs` UiMetadata, resolved card faces in the view.

## Assumption Reassessment (2026-06-12)

1. View fields verified this session: `drawn_cards`/`forecast` are ID strings and `undrawn_count: u8` is already public by rule (`games/flood_watch/src/visibility.rs:28-31,97-100`) — unlike Event Frontier, the face-down pile count is *not* redacted here, so the deck surface can show a real count badge (spec §6 D3). `src/ui.rs` is a stub; `games/flood_watch/data/` exists (`fixtures`, `manifest.toml`, `variants.toml`) with no presentation file — `cards_presentation.toml` is collision-free.
2. Spec authority: `specs/card-and-action-presentation-shared-surfaces.md` §6 D1/D2, §8 WB3. Pattern source: CARACTPRES-001's loader/ui/projection shape, itself mirroring `games/token_bazaar/src/ui.rs`. Card-kind enumeration site (`card.kind.id()` producers at `visibility.rs:97-98`) is the ID vocabulary the presentation table must cover.
3. Cross-artifact boundary under audit: the flood_watch public-view JSON contract into `apps/web/src/wasm/client.ts`. Additive-only extension (new resolved-face fields + `ui`); the existing TS reader keeps working until CARACTPRES-006 adopts the new data.
4. FOUNDATIONS §5 / `docs/ENGINE-GAME-DATA-BOUNDARY.md` §5/§7/§11: presentation TOML is typed inert content keyed by card kind; unknown fields rejected; no behavior-looking fields. Restated before implementation per the enforcement-surface rule.
5. No-leak and determinism (§11): drawn cards and forecast are public by rule; `undrawn_count` is already projected. Metadata is resolved only for projected IDs; no order or identity facts about undrawn cards are added. View extension is additive; fixtures/golden traces regenerate through the ordinary migration path; no hash-semantics change.
6. Schema extension audit: consumers are `crates/wasm-api` (opaque serialization), `apps/web/src/wasm/client.ts` + `FloodWatchBoard.tsx` (typed reader, updated in CARACTPRES-006), replay/fixture tooling. Additive-only.

## Architecture Check

1. Same code/label separation and inline face resolution as CARACTPRES-001 — one Rust lookup at projection, no TS join, Rust stays the display-text source (FOUNDATIONS §2). Implementing locally (second similar use after 001 within this spec) matches the §4 first/second-use rule: no extraction, no shared helper; the *contract shape* is convention, not a `game-stdlib` primitive (spec §3.3).
2. No backwards-compatibility aliasing/shims: view fields change once; consumer updated in-spec.
3. `engine-core` noun-free (§3); `game-stdlib` untouched (§4).

## Verification Layers

1. Presentation-table completeness over the card-kind enumeration -> unit test iterating all card kinds against the loaded table.
2. Fail-closed loader (unknown field / duplicate / missing ID) -> schema validation unit tests.
3. No-leak firewall (no undrawn order/identity facts; metadata only for projected IDs) -> no-leak visibility test in the crate's test suite.
4. Deterministic replay/serialization -> regenerated golden traces + `replay-check --game flood_watch --all`.

## What to Change

### 1. Authored presentation data

`games/flood_watch/data/cards_presentation.toml`: one row per storm/event card kind — `label`, `summary`, `family`, `accessibility_label`. Original prose (IP §10).

### 2. Typed fail-closed loader

Parse into a typed table at the crate's existing static-data parsing site (the module that owns `manifest.toml`/`variants.toml` parsing — implementation-discovered within `games/flood_watch/src/`, parent verified). Reject unknown fields, duplicates, gaps, with named diagnostics.

### 3. `ui.rs` UiMetadata + view projection

Replace the stub with `UiMetadata` + `ui_metadata()` + face resolution; project `drawn_cards`/`forecast` as resolved faces and add `pub ui` in `src/visibility.rs`. Regenerate affected fixtures/traces.

## Files to Touch

- `games/flood_watch/data/cards_presentation.toml` (new)
- `games/flood_watch/src/ui.rs` (modify)
- `games/flood_watch/src/visibility.rs` (modify)
- `games/flood_watch/src/lib.rs` (modify — module/exports wiring if needed)
- `games/flood_watch/src/` static-data parsing module (modify — as surfaced; implementation-discovered, parent verified)
- `games/flood_watch/tests/` (modify — golden traces/fixtures as surfaced; implementation-discovered set, parent dir verified via crate layout)

## Out of Scope

- Any visibility-contract change (no new hidden-info exposure; `undrawn_count` stays as-is — already public).
- TS/web changes — CARACTPRES-006.
- Event Frontier and Frontier Control work — CARACTPRES-001/002/004.
- `engine-core` / `game-stdlib` edits; YAML; behavior-looking metadata fields.

## Acceptance Criteria

### Tests That Must Pass

1. Unit tests: table completeness, fail-closed loader, face resolution, no-debug-vocabulary metadata assertion.
2. Visibility tests: resolved faces + `ui` projected for exactly the already-projected IDs; undrawn projection unchanged beyond the existing count.
3. `cargo test -p flood_watch && cargo run -p replay-check -- --game flood_watch --all && cargo run -p fixture-check -- --game flood_watch` green.

### Invariants

1. Rust is the sole display-text source for flood_watch components (FOUNDATIONS §2).
2. Presentation data is typed inert content, fail-closed on unknown fields (§5).

## Test Plan

### New/Modified Tests

1. `games/flood_watch/src/ui.rs` (inline `#[cfg(test)]`) — completeness, fail-closed cases, no-debug-vocabulary assertion.
2. `games/flood_watch/tests/` visibility/golden-trace updates — projected faces; regenerated traces.

### Commands

1. `cargo test -p flood_watch`
2. `cargo run -p replay-check -- --game flood_watch --all && cargo run -p fixture-check -- --game flood_watch && cargo run -p rule-coverage -- --game flood_watch`
3. Narrow boundary rationale: single-crate change; workspace hygiene runs at CARACTPRES-010 evidence.

## Outcome

Completed: 2026-06-12

What changed:

- Added `games/flood_watch/data/cards_presentation.toml` with authored presentation rows for all Flood Watch event kinds.
- Expanded `games/flood_watch/src/ui.rs` from a display-name stub into `UiMetadata`, `CardFaceView`, and a strict `CardPresentationCatalog` loader with completeness, duplicate, unknown-key, behavior-key, and empty-field rejection.
- Changed Flood Watch `PublicView.drawn_cards` and `PublicView.forecast` from raw event-kind strings to resolved card-face views while preserving the existing public `undrawn_count`.
- Updated Flood Watch bot forecast parsing, visibility/no-leak checks, public effect text, replay assertions, and the WASM public-view serializer for the new card-face shape.

Deviations from plan:

- `crates/wasm-api/src/lib.rs` was included for the same current-code reason found in CARACTPRES-001: Flood Watch view JSON is manually serialized by the bridge.
- No golden trace files changed; replay hashes remained valid after the view/text updates.

Verification:

- `cargo test -p flood_watch` — passed.
- `cargo run -p replay-check -- --game flood_watch --all` — passed.
- `cargo run -p fixture-check -- --game flood_watch` — passed.
- `cargo run -p rule-coverage -- --game flood_watch` — passed.
- `cargo check -p wasm-api` — passed.
- `cargo clippy --workspace --all-targets -- -D warnings` — passed.
- `cargo fmt --all --check` — passed after formatting.

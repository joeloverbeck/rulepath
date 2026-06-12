# CARACTPRES-001: Event Frontier card presentation metadata and view projection

**Status**: PENDING
**Priority**: HIGH
**Effort**: Large
**Engine Changes**: Yes — `games/event_frontier` (additive public-view extension, static presentation-data loader, `ui.rs` metadata channel); no `engine-core` or `game-stdlib` changes
**Deps**: None (first ticket of `specs/card-and-action-presentation-shared-surfaces.md` WB1)

## Problem

Players cannot tell what Event Frontier's event cards are or do. `PublicView` projects raw card IDs only (`current_card`, `next_public_card`, `discard` as `String`s — `games/event_frontier/src/visibility.rs:15-39`), the authored labels in `games/event_frontier/data/cards.toml` are parsed into `VariantSetup.labels` (`src/variants.rs:496-527`) but never projected, no card has effect-summary text anywhere, and the crate has no `ui` metadata field on its view (unlike `token_bazaar`, `high_card_duel`, `masked_claims`). This ticket builds the Rust side of spec Workstream A for the pilot game: authored presentation metadata, a typed loader, a `ui.rs` UiMetadata channel, and resolved card faces in the public view.

## Assumption Reassessment (2026-06-12)

1. `EventFrontierPublicView` exposes `current_card: Option<String>`, `next_public_card: Option<String>`, `discard: Vec<String>` and has no `ui` field — verified at `games/event_frontier/src/visibility.rs:15-39`. Card labels exist in `data/cards.toml` and are parsed in `src/variants.rs:496-527` (`CardListEntry.label` at `src/cards.rs:466,527`).
2. The spec (`specs/card-and-action-presentation-shared-surfaces.md` §6 D1/D2, §7, §8 WB1) commits to a sibling presentation file — not new fields on rule-bearing card data — so `EF-COMP-012` ("card data is identity and parameters only", `games/event_frontier/docs/RULES.md`) stands unmodified. The UiMetadata pattern to mirror is `games/token_bazaar/src/ui.rs:1-97` + `games/token_bazaar/src/visibility.rs:34`.
3. Cross-artifact boundary under audit: the viewer-safe public-view JSON contract crossing `crates/wasm-api` into `apps/web/src/wasm/client.ts`. This ticket extends the Rust producer only; the TS consumer types land in CARACTPRES-005. The extension must be additive (new fields; no existing field renamed, removed, or retyped) so the current renderer keeps working until 005 adopts the new data.
4. FOUNDATIONS §5 / `docs/ENGINE-GAME-DATA-BOUNDARY.md` §5/§11 motivate this ticket: static data MAY carry display names, short descriptions, UI metadata, and accessibility labels, and MUST NOT carry selectors, conditions, triggers, or behavior-looking fields; unknown fields are rejected by default (§7 of that doc). The presentation TOML is inert typed content keyed by `CardId`.
5. No-leak and determinism surfaces (§11): deck composition and discard are public by rule in Event Frontier, so a static card encyclopedia leaks nothing; the undrawn count and order stay redacted per `EF-VIS-002` (`games/event_frontier/docs/RULES.md`) — this ticket projects metadata only for IDs the view already projects, adds no count/order/position facts to any payload, and keeps view serialization deterministic (fixture/golden-trace updates ride the ordinary migration path, no hash-semantics change).
6. Schema extension audit: consumers of the Event Frontier public-view JSON are `crates/wasm-api/src/lib.rs` (serializes whatever `event_frontier_project_view` returns — no per-field code), `apps/web/src/wasm/client.ts` + `apps/web/src/components/EventFrontierBoard.tsx` (typed reader; unaffected by additive fields until 005), and replay/fixture tooling. Extension is additive-only.

## Architecture Check

1. Resolved card faces projected inline (`current_card: Option<CardFaceView>` etc.) beat a TS-side ID→metadata join: one Rust lookup, no duplicated table shipped separately, and the view stays the single viewer-safe source — consistent with FOUNDATIONS §2 (Rust owns view projection; TS presents).
2. No backwards-compatibility aliasing/shims: the typed view fields change shape in one step; the TS consumer is updated in CARACTPRES-005 within the same spec, and no deprecated parallel fields are kept.
3. `engine-core` stays free of mechanic nouns (§3): `CardFaceView`, the presentation table, and the loader are `games/event_frontier` types. No `game-stdlib` change; the repeated UiMetadata *shape* is a per-game convention, not an atlas-earned helper (spec §3.3).

## Verification Layers

1. Presentation table completeness (every `CardId` variant has exactly one row; no orphan rows) -> unit test over the loaded table against `CardId` iteration.
2. Fail-closed static data (unknown field, missing label/summary, duplicate ID rejected) -> schema/serialization validation tests on the TOML loader.
3. No-leak firewall (no undrawn count/order/position facts in view or metadata; metadata rendered only for projected IDs) -> no-leak visibility test extended in `tests/rules.rs`/visibility tests asserting redacted surfaces unchanged.
4. Deterministic replay/serialization after the view extension -> `cargo run -p replay-check -- --game event_frontier --all` plus regenerated golden traces/fixtures.
5. FOUNDATIONS §5 alignment (typed content only) -> FOUNDATIONS alignment check citing §5 and `docs/ENGINE-GAME-DATA-BOUNDARY.md` §11/§12 in review.

## What to Change

### 1. Authored presentation data

Add `games/event_frontier/data/cards_presentation.toml`: one row per `CardId` (21 rows) with `label`, `summary` (one-line original effect prose), `family` (existing ordinary/edict/reckoning UI-family tags), `accessibility_label`. Original Rulepath prose only (IP §10).

### 2. Typed loader with fail-closed validation

Parse the file into a typed `CardPresentation` table (location: alongside the existing `cards.toml` parsing in `src/variants.rs` / `src/cards.rs`, whichever owns static-data parsing after inspection). Reject unknown fields, duplicate IDs, missing IDs, and empty strings; diagnostics name file, field, and offending value (`docs/ENGINE-GAME-DATA-BOUNDARY.md` §7).

### 3. `ui.rs` UiMetadata channel

Replace the one-line stub with a `UiMetadata` struct + `ui_metadata()` (panel/deck-slot/hidden-pile labels and copy — including the face-down-pile presentation copy that currently lives hardcoded in TS) and a card-face resolution helper, mirroring `games/token_bazaar/src/ui.rs`.

### 4. View projection

In `src/visibility.rs`: project `current_card`/`next_public_card`/`discard` as resolved card-face views (id, label, summary, family, accessibility label) and add `pub ui: UiMetadata`. Update serialization-order tests and regenerate affected fixtures/golden traces.

## Files to Touch

- `games/event_frontier/data/cards_presentation.toml` (new)
- `games/event_frontier/src/ui.rs` (modify)
- `games/event_frontier/src/visibility.rs` (modify)
- `games/event_frontier/src/cards.rs` (modify)
- `games/event_frontier/src/variants.rs` (modify)
- `games/event_frontier/src/lib.rs` (modify — module/exports wiring if needed)
- `games/event_frontier/tests/golden_traces/` (modify — regenerated trace JSON as surfaced; parent verified)
- `games/event_frontier/data/fixtures/` (modify — fixture refresh as surfaced; implementation-discovered set, parent `games/event_frontier/data/` verified)

## Out of Scope

- Any visibility-contract change: no undrawn count, order, identity, or reveal-timing exposure (`EF-VIS-002` unchanged; FOUNDATIONS §13 ADR trigger).
- TS/web changes (`client.ts`, boards, shared components) — CARACTPRES-005.
- Player-facing Rust copy hygiene in effects/eligibility strings — CARACTPRES-002.
- `flood_watch` / `frontier_control` parity — CARACTPRES-003/004.
- `engine-core` / `game-stdlib` edits; YAML; any behavior-looking metadata field.
- Edits to `EF-COMP-012`-governed card identity/parameter data.

## Acceptance Criteria

### Tests That Must Pass

1. Unit tests: presentation-table completeness, fail-closed loader (unknown field / duplicate / missing ID rejected with named diagnostics), card-face resolution.
2. Visibility tests: view projects resolved faces + `ui` for exactly the already-projected IDs; redaction surfaces byte-identical in intent (no new count/order facts).
3. `cargo test -p event_frontier` and `cargo run -p replay-check -- --game event_frontier --all` and `cargo run -p fixture-check -- --game event_frontier` green.

### Invariants

1. Rust remains the sole source of card display text; no TS-derivable-from-ID fallback is required by the new contract (FOUNDATIONS §2).
2. Static presentation data is typed inert content: no selectors, conditions, triggers, or fields whose meaning mutates state (§5); unknown fields rejected.

## Test Plan

### New/Modified Tests

1. `games/event_frontier/src/ui.rs` (inline `#[cfg(test)]`) — table completeness, fail-closed loader cases, face resolution, no-debug-vocabulary metadata assertion (mirrors `token_bazaar`'s `ui_metadata_has_labels_without_debug_or_candidate_data`).
2. `games/event_frontier/tests/rules.rs` / visibility tests — projected-faces and no-new-redaction-facts assertions.
3. `games/event_frontier/tests/golden_traces/` — regenerated traces for the additive view fields.

### Commands

1. `cargo test -p event_frontier`
2. `cargo run -p replay-check -- --game event_frontier --all && cargo run -p fixture-check -- --game event_frontier && cargo run -p rule-coverage -- --game event_frontier`
3. `cargo clippy --workspace --all-targets -- -D warnings` — full-hygiene boundary since the crate's public surface changed.

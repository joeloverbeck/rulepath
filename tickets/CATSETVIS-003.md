# CATSETVIS-003: Per-game `catalog_theme` metadata + per-game accents

**Status**: PENDING
**Priority**: MEDIUM
**Effort**: Medium
**Engine Changes**: Yes — `games/*/src/ui.rs` (Rust inert UI metadata, incl. new `games/race_to_n/src/ui.rs` + its `lib.rs` module wiring), `apps/web/src/styles.css`
**Deps**: 001

## Problem

Each official game needs a canonical, inert theme identity so the catalog and setup surfaces can render a distinct color-plus-shape per game without TypeScript inventing presentation data. This ticket authors per-game `catalog_theme` metadata (icon id, theme key, accent/secondary-accent token names, shape token, accessibility label) in all 14 games' `ui.rs` — adding a minimal `games/race_to_n/src/ui.rs` (the only game lacking one) — and the per-game `data-game-id` accent blocks in `styles.css`. This is the **CSS-bootstrap** realization sanctioned by spec §14 A3: no wasm-api typed projection in P2. Spec WB3 / §6 D5.

## Assumption Reassessment (2026-06-13)

1. 13 of 14 games carry `games/<id>/src/ui.rs`; `games/race_to_n/src/ui.rs` is absent (verified `ls`); `catalog_theme`/`CatalogTheme` appears nowhere in `games/` / `crates/` / `apps/` (verified — new). Sibling `lib.rs` files declare `pub mod ui;` (e.g. `games/flood_watch/src/lib.rs:11`), so the new `race_to_n` module needs the same one-line wiring. Existing `ui.rs` already carry inert presentation metadata (shapes, seat/faction labels), the proven local home.
2. Spec §6 D5 + §14 A3 govern: `catalog_theme` is inert content selecting presentation tokens and an icon id only — never legality, selectors, hidden identities, action availability, or behavior-by-naming. A3 sanctions the CSS-bootstrap path (CSS `data-game-id` accents + the CATSETVIS-002 TS icon registry transcribing `icon_id`), deferring any wasm-api typed projection.
3. Cross-artifact boundary: the per-game `data-game-id` blocks in `styles.css` override the `--game-*` default tokens defined by CATSETVIS-001 (hence `Deps: 001`); the `icon_id` values are the source-of-truth the CATSETVIS-002 registry transcribes. No `crates/wasm-api` or `client.ts` change occurs here (CSS-bootstrap).
4. FOUNDATIONS §5 (static data is typed content) + §2 (behavior authority): `catalog_theme` is typed inert Rust content — Rust owns the per-game theme record, TypeScript renders it. §3 `engine-core` is untouched (`ui.rs` is `games/*`-local); §4 no `game-stdlib` promotion — UI-INTERACTION §10A states repeated `ui.rs` shapes are presentation governed by UI law, not mechanic-atlas pressure.
5. No-leak substrate (§11): `catalog_theme` carries no hidden identity or game state and introduces no leak path; in P2 it is not projected to any viewer-filtered payload (CSS-bootstrap). If a future ticket projects it through the catalog `ui` JSON (like `seat_labels`/`faction_labels` already are), that ticket — not this one — owns the viewer-safety review; this ticket confirms the data model adds no leakage the later surface would have to undo.

## Architecture Check

1. `ui.rs` is the established local home for inert per-game presentation metadata (13 precedents); the CSS-bootstrap realization is the lowest-risk path per A3, keeping `crates/wasm-api/src/lib.rs` touched only by the description ticket (CATSETVIS-006) and deferring typed projection until a consumer needs it.
2. No backwards-compatibility shims; `race_to_n` gets a minimal `ui.rs` matching its siblings' shape rather than a special-case.
3. `engine-core` stays free of mechanic nouns (`catalog_theme` is `games/*`-local); no `game-stdlib` promotion (UI-INTERACTION §10A; `game-stdlib` exports only `board_space`).

## Verification Layers

1. `catalog_theme` on all 14 games → codebase grep-proof (`catalog_theme` present in every `games/<id>/src/ui.rs`, including the new `race_to_n`).
2. Inert (no behavior-looking fields) → manual review against FOUNDATIONS §5 (the record holds only token names, an icon id, and an a11y label).
3. Rust integrity → `cargo build --workspace` + `cargo test --workspace` green (the new module compiles; no behavior changes) and `bash scripts/boundary-check.sh` (engine-core stays noun-free).
4. Per-game accents render → manual review (each `data-game-id` block overrides the `--game-*` defaults) confirmed by `smoke:ui`.

## What to Change

### 1. `catalog_theme` on the 13 existing games

Add an inert `catalog_theme` record (icon id, theme key, accent/secondary-accent token names, shape token, `a11y_label`) to each existing `games/<id>/src/ui.rs`.

### 2. New `games/race_to_n/src/ui.rs`

Create a minimal `ui.rs` for `race_to_n` matching the sibling shape (incl. its `catalog_theme`), and add `pub mod ui;` to `games/race_to_n/src/lib.rs`.

### 3. Per-game `data-game-id` accent blocks

Add per-game `.game-card[data-game-id="…"]` / setup-header accent overrides in `styles.css`, layered on the CATSETVIS-001 token defaults.

## Files to Touch

- `games/column_four/src/ui.rs` (modify)
- `games/directional_flip/src/ui.rs` (modify)
- `games/draughts_lite/src/ui.rs` (modify)
- `games/event_frontier/src/ui.rs` (modify)
- `games/flood_watch/src/ui.rs` (modify)
- `games/frontier_control/src/ui.rs` (modify)
- `games/high_card_duel/src/ui.rs` (modify)
- `games/masked_claims/src/ui.rs` (modify)
- `games/plain_tricks/src/ui.rs` (modify)
- `games/poker_lite/src/ui.rs` (modify)
- `games/secret_draft/src/ui.rs` (modify)
- `games/three_marks/src/ui.rs` (modify)
- `games/token_bazaar/src/ui.rs` (modify)
- `games/race_to_n/src/ui.rs` (new)
- `games/race_to_n/src/lib.rs` (modify; add `pub mod ui;`)
- `apps/web/src/styles.css` (modify; per-game `data-game-id` accent blocks)

## Out of Scope

- Any `crates/wasm-api` / `client.ts` typed projection of `catalog_theme` (deferred per §14 A3 — a future ticket if typed projection is wanted).
- The TS icon registry (CATSETVIS-002) and token definitions (CATSETVIS-001).
- The variant `description` field (CATSETVIS-005/006).

## Acceptance Criteria

### Tests That Must Pass

1. `cargo build --workspace` and `cargo test --workspace` are green (the new `race_to_n::ui` module compiles; no behavior regression across games).
2. `grep -rl 'catalog_theme' games/*/src/ui.rs | wc -l` returns `14`.
3. `npm --prefix apps/web run smoke:ui` is green and `bash scripts/boundary-check.sh` passes.

### Invariants

1. `catalog_theme` is inert typed content — no behavior-looking fields (no `when`/`if`/`selector`/`trigger`/legality), no hidden identity.
2. `engine-core` stays mechanic-noun-free; `race_to_n/src/ui.rs` matches the sibling metadata shape.

## Test Plan

### New/Modified Tests

1. `None — inert Rust UI-metadata addition; no behavior test files change. Verification is `cargo build`/`cargo test` (the new module compiles, no regression) + `smoke:ui` + `boundary-check.sh`, the existing pipeline named in Assumption Reassessment.`

### Commands

1. `cargo build --workspace && cargo test --workspace`
2. `grep -rl 'catalog_theme' games/*/src/ui.rs | wc -l` (expect `14`)
3. `npm --prefix apps/web run smoke:ui && bash scripts/boundary-check.sh`

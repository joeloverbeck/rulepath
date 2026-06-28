# GAT204STACROMAT-001: Rust-owned Starbridge in-match seat display labels on the public view

**Status**: COMPLETED
**Priority**: HIGH
**Effort**: Medium
**Engine Changes**: Yes — `crates/wasm-api` (`src/games/starbridge_crossing.rs`: public-view projection gains `ui.seat_labels` + per-seat `label`/`target_label`; `src/catalog.rs`: ring-label lookup helper; `src/tests.rs`: discontinuity view-label test; `tests/snapshots/api_surface.tsv`: additive snapshot refresh)
**Deps**: None

## Problem

Every in-match Starbridge Crossing surface names seats by raw index (`Seat 1`) or by a TypeScript title-casing of the lowercase `home` token, because the Starbridge public view projects no viewer-safe per-seat display label in the shape the shared shell consumes. `docs/UI-INTERACTION.md` §10B/§19 require normal-mode surfaces to name the acting faction in display terms (point names), with seat indices reserved for the dev panel. The shared turn-status bar (`ModeControls.seatLabelsForView`) reads only `view.ui.seat_labels`, which Starbridge does not project, so it falls back to `Seat N`. This ticket adds the Rust-owned labels — `view.ui.seat_labels` plus per-seat `label` (home point name) and `target_label` (destination point name) on `seats[]` — sourced from the authored catalog ring labels resolved per the seat's point index, so the board (002) and the shared turn bar consume one Rust source of truth.

## Assumption Reassessment (2026-06-28)

1. The Starbridge public-view JSON is built in `crates/wasm-api/src/games/starbridge_crossing.rs` (the `seats` projection emits `{seat_id, seat_index, home, target, finish_rank}`; the top-level `format!` has no `ui` object today). It imports the game crate, and reads `seat.home`/`seat.target` as lowercase `StarPoint::label()` tokens (`escape_json(&seat.home)`). Confirmed.
2. The authored title-case labels live in `crates/wasm-api/src/catalog.rs::catalog_starbridge_seat_labels_json` (`[{"seat":"seat_0","label":"North"},…,{"seat":"seat_5","label":"North West"}]`) — the same source Gate 20.2 used for setup-preview labels. The game crate (`games/starbridge_crossing/src/ids.rs`) exposes only the lowercase `StarPoint::label()`, plus `clockwise_index()` (`North=0…NorthWest=5`) and `active_points_for_seat_count`. The catalog ring list is ordered to match `clockwise_index`. Confirmed — so resolution stays in `wasm-api` (catalog + game-crate index helper); no game-crate change, no relocation of the authored labels.
3. Cross-artifact boundary under audit: the public-view JSON contract in `crates/wasm-api/src/games/starbridge_crossing.rs`, the additive `api_surface.tsv` snapshot (`crates/wasm-api/tests/snapshots/api_surface.tsv`, starbridge rows present), and the catalog ring-label source in `src/catalog.rs`. The `ui.seat_labels` path precedent is Event Frontier (`crates/wasm-api/src/games/event.rs:236`, `seat_labels` inside `ui`), NOT River Ledger's top-level `active_seat_labels`.
4. FOUNDATIONS §2 (behavior authority) motivates this ticket: the seat display name is Rust-owned view metadata. The shell presents it; it never synthesizes the name from a token or index. This removes (in 002) the interim TypeScript token formatting rather than entrenching it.
5. Deterministic replay/hash & serialization surface: the labels are an additive **view-projection** field only — they enter neither the game-local `StarbridgePublicView::stable_bytes` nor any accepted-command/state/effect/trace form (cf. Gate 20.3 adding `terminal_rationale` with no hash change). No-leak firewall (§11): Starbridge is fully public; the labels are public point names already implied by `seats[].home`/`target`. The only artifact diff is the additive `api_surface.tsv` snapshot.
6. Schema extension: the public view gains an additive top-level `ui: { seat_labels: SeatDisplayLabel[] }` object and additive `seats[].label` / `seats[].target_label` strings. Consumers: the web shell (`apps/web/src/wasm/client.ts` types + `StarbridgeCrossingBoard` + `ModeControls`), updated in 002. Additive-only; no existing field is renamed or removed.

## Architecture Check

1. Resolving the labels in `wasm-api`, where the authored catalog ring labels already live, keeps a single ground-truth source and a single resolution hop (`home`/`target` token → `StarPoint` → `clockwise_index()` → ring label). Resolving in the game crate is rejected: it would duplicate or relocate the authored catalog labels (against Assumption A1's single-ground-truth posture and the Gate 20.2 catalog-in-wasm-api precedent) and would invert the crate dependency. Projecting onto `view.ui.seat_labels` (the field `ModeControls` already reads) reuses the shared resolution path with no game-specific shell coupling.
2. No backwards-compatibility shim: `ui`, `label`, and `target_label` are new additive fields, not aliases over `home`/`target`; the existing `home`/`target` tokens are retained unchanged.
3. `engine-core` stays free of mechanic nouns — the seat-ring/point mapping stays game-local in `games/starbridge_crossing` and is surfaced through the existing `wasm-api` view/catalog plumbing; no topology/seat-ring noun enters the kernel or `game-stdlib` (§3/§4).

## Verification Layers

1. Label content + discontinuity resolution → unit test in `crates/wasm-api/src/tests.rs`: for `{2,3,4,6}` seats, each projected `seat_labels`/`label` equals the authored ring label **at the seat's home-point index** (`ring_labels[seat.home.clockwise_index()]`) and `target_label` equals the ring label at the target-point index, with explicit discontinuity assertions (2-seat `seat_1` label == `"South"`; 3-seat `seat_1` label == `"South East"`). Expected values are computed via the point index, never a flat `catalog[seat_id]` echo.
2. Schema/serialization conformance → `crates/wasm-api/tests/api_surface.rs` snapshot test passes after refreshing `tests/snapshots/api_surface.tsv` with the single additive `ui.seat_labels` / `label` / `target_label` diff.
3. Determinism / no-leak (all-public) → FOUNDATIONS §11 alignment review plus `cargo run -p replay-check -- --game starbridge_crossing --all`: the labels project public point names only, enter no `stable_bytes`/trace/hash form, and replay stays byte-identical.

## What to Change

### 1. Ring-label lookup helper in `src/catalog.rs`

Add a helper that exposes the authored Starbridge ring labels as an indexable `[&str; 6]` / `Vec<&str>` in `clockwise_index` order (the same data `catalog_starbridge_seat_labels_json` serializes), so the view projection can resolve a point's display label by index without re-parsing JSON. Keep `catalog_starbridge_seat_labels_json` as the catalog serialization; the helper is the shared source both call sites read.

### 2. Project the labels in `src/games/starbridge_crossing.rs`

For each projected seat, map the `home` (and `target`) token → `StarPoint` → `clockwise_index()` → ring label via the helper, and emit additive `"label"` and `"target_label"` strings on each `seats[]` entry. Add a top-level `"ui":{"seat_labels":[…]}` object whose entries are `{seat: seat_id, label}` keyed by the play-time `seat_id`, matching the `SeatDisplayLabel[]` shape. Resolve by point index, not by a flat `seat_id` lookup, so the discontinuous `{2,3,4}` configs label correctly.

### 3. Tests + snapshot

Add the discontinuity view-label test in `crates/wasm-api/src/tests.rs` (reuse `create_starbridge_match` / `starbridge_view_json`, the helpers used by `starbridge_view_projects_terminal_rationale_payload`). Refresh `crates/wasm-api/tests/snapshots/api_surface.tsv` for the additive diff.

## Files to Touch

- `crates/wasm-api/src/games/starbridge_crossing.rs` (modify)
- `crates/wasm-api/src/catalog.rs` (modify)
- `crates/wasm-api/src/tests.rs` (modify)
- `crates/wasm-api/tests/snapshots/api_surface.tsv` (modify)

## Out of Scope

- All web consumption — `client.ts` types, `StarbridgeCrossingBoard`, `ModeControls`, browser smoke (GAT204STACROMAT-002).
- Any movement, finish, terminal-result, visibility, bot, or legality change; any new variant/seat-count/piece-count; any ring-label renaming.
- Any change to `home`/`target` tokens, accepted commands, state, effects, golden traces, fixtures, `stable_bytes`, or benchmark thresholds.

## Acceptance Criteria

### Tests That Must Pass

1. New `crates/wasm-api/src/tests.rs` case: for `{2,3,4,6}` seats, projected `seat_labels`/`label` == ring label at `seat.home.clockwise_index()` and `target_label` == ring label at `seat.target.clockwise_index()`, with the discontinuity assertions (2-seat `seat_1` == `"South"`, 3-seat `seat_1` == `"South East"`).
2. `cargo test -p wasm-api` (includes the refreshed `api_surface.rs` snapshot test).
3. `cargo run -p replay-check -- --game starbridge_crossing --all` — hashes/traces unchanged.

### Invariants

1. The seat display labels are Rust-owned and resolved by the seat's home/target point index from the authored catalog ring — never a flat `seat_id` lookup and never a token title-casing.
2. The labels are additive view-projection fields only; they appear in no `stable_bytes`/trace/hash form, so replay/hash artifacts stay byte-identical.

## Test Plan

### New/Modified Tests

1. `crates/wasm-api/src/tests.rs` — discontinuity view-label test across `{2,3,4,6}` seats (home `label`, `target_label`, and `ui.seat_labels`).
2. `crates/wasm-api/tests/snapshots/api_surface.tsv` — refreshed for the single additive `ui.seat_labels`/`label`/`target_label` diff.

### Commands

1. `cargo test -p wasm-api`
2. `cargo run -p replay-check -- --game starbridge_crossing --all`
3. `cargo test -p wasm-api` is the correct boundary — the projection and its snapshot are wasm-api-local; web consumption is verified in GAT204STACROMAT-002.

## Outcome

Completed: 2026-06-28

Implemented the Rust/WASM public-view projection for Starbridge in-match seat display names:

- Added `catalog_starbridge_ring_labels()` as the indexable authored label source used by the Starbridge catalog JSON and view serialization.
- Added Starbridge `ui.seat_labels` plus per-seat `label` and `target_label` fields to the WASM public view, resolved from the seat home/target point index rather than a flat play-time seat index.
- Added `starbridge_view_projects_point_index_seat_labels`, including explicit discontinuity assertions for 2-seat `seat_1 == South` and 3-seat `seat_1 == South East`.
- Refreshed `crates/wasm-api/tests/snapshots/api_surface.tsv` for the additive Starbridge public-view/import/replay-step shape.

Deviations from plan: none. No game rules, state, effects, stable bytes, trace fixtures, or hash surfaces changed.

Verification:

- `UPDATE_API_SNAPSHOT=1 cargo test -p wasm-api --test api_surface` passed after the expected additive snapshot refresh.
- `cargo test -p wasm-api` passed.
- `cargo run -p replay-check -- --game starbridge_crossing --all` passed; all Starbridge traces were accepted unchanged.
- `cargo fmt --all --check` passed.
- `cargo clippy -p wasm-api --all-targets -- -D warnings` passed.

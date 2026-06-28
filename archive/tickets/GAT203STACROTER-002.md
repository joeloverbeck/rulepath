# GAT203STACROTER-002: wasm-api serialization of Starbridge `terminal_rationale`

**Status**: COMPLETED
**Priority**: MEDIUM
**Effort**: Medium
**Engine Changes**: Yes — `crates/wasm-api` (`src/games/starbridge_crossing.rs` projection JSON; `src/tests.rs`; refreshes `tests/snapshots/api_surface.tsv`)
**Deps**: GAT203STACROTER-001

## Problem

The wasm-api Starbridge projection (`crates/wasm-api/src/games/starbridge_crossing.rs`) serializes the public view to JSON via an inline `format!`, emitting `finish_ranks` / per-seat `finish_rank` / flat `terminal` but **no** `terminal_rationale` — even though the web bridge type `apps/web/src/wasm/client.ts::StarbridgeCrossingPublicView.terminal_rationale?` (`:1617`) declares an optional slot that is never populated. After GAT203STACROTER-001 projects the rationale on the Rust view, the bridge must serialize it so the shell can render it.

## Assumption Reassessment (2026-06-28)

1. `crates/wasm-api/src/games/starbridge_crossing.rs` builds the view JSON with an inline `format!` over `finish_ranks` / `finish_rank` / `terminal` (around `:214`–`:245`); there is no `terminal_rationale` key today. Confirmed.
2. The river projection `crates/wasm-api/src/games/river.rs` is the shape to mirror: its view `format!` emits `"terminal_rationale":{}` (`:94`) filled by `river_rationale_json` (`:469`) with `null` when absent (`:140` `map_or_else(|| "null".to_owned(), …)`), and per-seat standings via `river_rationale_standing_json`. Confirmed — reuse these conventions rather than a divergent shape.
3. Cross-crate boundary under audit: the `StarbridgeOutcomeRationaleView` produced by GAT203STACROTER-001 (`games/starbridge_crossing/src/visibility.rs`) is the input; the output is the public-view JSON consumed by `client.ts`. The web TS type is the bare alias `StarbridgeCrossingOutcomeRationale = OutcomeRationalePayload` (`client.ts:176`), whose `final_standing[].values?: OutcomeRationaleField[]` slot (`OutcomeRationaleStanding`) absorbs the turn-limit progress count — so the JSON routes the progress count through a `values` entry, no TS type change (spec Assumption A5).
4. FOUNDATIONS §11 determinism invariant motivates this ticket: the serialized field is additive and viewer-safe; it must not perturb any deterministic artifact other than the additive snapshot diff.
5. Serialization surface: the wasm view JSON and its golden snapshot `crates/wasm-api/tests/snapshots/api_surface.tsv`. The three `starbridge_crossing/view/{observer,seat_0,seat_1}` rows currently carry no `terminal_rationale`; adding the field (serialized `null` at the non-terminal new-match snapshot state) adds `"terminal_rationale":null` to those three rows. Implementation also showed the shared Starbridge view serializer is embedded by replay import/step/reset snapshot rows, so the intended additive field appears there too. No replay/hash/fixture/bench surface is touched (those live in `games/*`, not the wasm snapshot).
6. Schema extension: the Starbridge public-view JSON gains `terminal_rationale` (object at terminal, `null` otherwise). Consumer: `client.ts::StarbridgeCrossingPublicView`, whose optional slot already exists — additive-only.

## Architecture Check

1. Reusing river's `*_rationale_json` shape conventions keeps every game's wasm rationale serialization uniform and lets the shared TS `OutcomeRationalePayload` consumer read Starbridge without special-casing.
2. No shim: the field is appended to the existing `format!`; the flat `terminal` key is retained.
3. No `engine-core` / `game-stdlib` noun added — serialization stays in `crates/wasm-api/src/games/starbridge_crossing.rs`, game-scoped.

## Verification Layers

1. Serialization correctness → `src/tests.rs` projection test: mid-match JSON carries `terminal_rationale: null`; terminal JSON carries the populated object with `decisive_cause` / `template_key` / seat-ordered `final_standing`.
2. Snapshot fidelity (additive-only) → `cargo test -p wasm-api` against the refreshed `api_surface.tsv`; the diff is exactly `"terminal_rationale":null` on Starbridge view payload rows, including replay rows that embed the same Starbridge view JSON.
3. Determinism preservation → `replay-check --all` / `fixture-check` on the game show no diff (game-side artifacts untouched by this crate change).

## What to Change

### 1. Emit `terminal_rationale` in the projection JSON

In `crates/wasm-api/src/games/starbridge_crossing.rs`, add `"terminal_rationale":{}` to the view `format!`, filled `null` when `view.terminal_rationale` is `None` and via a `starbridge_rationale_json` helper (mirroring `river_rationale_json` / `river_rationale_standing_json`) when present — routing the per-seat turn-limit progress count through a standing `values` entry. No raw `seat_<n>` in human-facing copy.

### 2. Refresh the snapshot + add the projection test

Regenerate `crates/wasm-api/tests/snapshots/api_surface.tsv` (expected diff: Starbridge view payload rows gain `terminal_rationale:null`) and add a projection assertion for the mid-match-`null` / terminal-populated pair.

## Files to Touch

- `crates/wasm-api/src/games/starbridge_crossing.rs` (modify)
- `crates/wasm-api/src/tests.rs` (modify)
- `crates/wasm-api/tests/snapshots/api_surface.tsv` (modify)

## Out of Scope

- The Rust rationale projection itself (GAT203STACROTER-001) and web rendering (GAT203STACROTER-003).
- Any change to game-side replay/hash/fixture/trace artifacts.
- Any divergent rationale JSON shape — the river shape is reused.

## Acceptance Criteria

### Tests That Must Pass

1. `src/tests.rs` projection test: `terminal_rationale: null` mid-match, populated object at terminal.
2. `cargo test -p wasm-api` (snapshot matches the refreshed `api_surface.tsv`).
3. `cargo run -p replay-check -- --game starbridge_crossing --all` and `cargo run -p fixture-check -- --game starbridge_crossing` show no diff.

### Invariants

1. The only `api_surface.tsv` diff is the additive `terminal_rationale` field on Starbridge view payload rows.
2. The serialized rationale carries only public facts; no hidden information enters the payload.

## Test Plan

### New/Modified Tests

1. `crates/wasm-api/src/tests.rs` — Starbridge projection: mid-match `null`, terminal populated.
2. `crates/wasm-api/tests/snapshots/api_surface.tsv` — refreshed snapshot (additive field on Starbridge view payload rows).

### Commands

1. `cargo test -p wasm-api`
2. `cargo run -p replay-check -- --game starbridge_crossing --all && cargo run -p fixture-check -- --game starbridge_crossing`
3. `cargo test -p wasm-api` is the correct boundary — this ticket owns only the bridge serialization + its snapshot; game-side determinism is GAT203STACROTER-001's.

## Outcome

Completed: 2026-06-28

Implemented Starbridge `terminal_rationale` serialization in
`crates/wasm-api/src/games/starbridge_crossing.rs`, mirroring the shared
outcome-rationale payload shape used by River Ledger: nullable nonterminal
field, terminal object with `result_kind`, `decisive_cause`, `template_key`,
decisive rule IDs, and `final_standing` rows. Per-seat turn-limit progress is
serialized through standing `values` entries, with human-facing labels such as
`Seat 1` rather than raw seat IDs.

Added a focused wasm-api test covering `terminal_rationale:null` for a live
Starbridge view and a populated turn-limit rationale with rule IDs, ranked
standings, winner emphasis, and progress values. Refreshed
`crates/wasm-api/tests/snapshots/api_surface.tsv`.

Verification:

- `cargo fmt --all --check` passed.
- `cargo clippy -p wasm-api --all-targets -- -D warnings` passed.
- `cargo test -p wasm-api` passed, including the refreshed API surface
  snapshot.
- `cargo run -p replay-check -- --game starbridge_crossing --all` passed; all
  Starbridge traces were accepted.
- `cargo run -p fixture-check -- --game starbridge_crossing` passed.

Deviations: the original ticket expected only the three
`starbridge_crossing/view/{observer,seat_0,seat_1}` snapshot rows to change.
The actual additive snapshot diff is six Starbridge rows because replay
import/step/reset rows embed the same Starbridge view JSON. No non-Starbridge
snapshot row changed, and no game-side replay or fixture artifact changed.

Outcome amended: 2026-06-28

During GAT203STACROTER-003 web-panel wiring, the shared
`OutcomeExplanationPanel` adapter was confirmed to read `final_standing[].seat`.
The Starbridge standing serializer now emits `seat` alongside the river-shaped
`id` field so the web panel consumes Rust-projected standings directly instead
of falling back to undefined row ids. The repair preserves the same additive
Starbridge-only bridge scope and is covered by the updated wasm-api projection
test, refreshed API surface snapshot, web build, and Starbridge browser smoke.

# GAT17VOWTIDOHHEL-015: Data manifest/variants/fixtures and fixture-check registration

**Status**: COMPLETED
**Priority**: MEDIUM
**Effort**: Medium
**Engine Changes**: Yes — finalizes `games/vow_tide/data/{manifest,variants}.toml`, new `games/vow_tide/data/fixtures/*.json`, `load_manifest`/`load_variants`; modifies `tools/fixture-check/src/main.rs`
**Deps**: 006, 009

## Problem

Finalize the typed static-data layer (manifest, variants, fixtures) carrying identity/presentation only, expose `vow_tide::load_manifest()` / `load_variants()`, and register `vow_tide` in the hard-coded `fixture-check` registry so the tool validates typed manifest/fixtures, rejects unknown and behavior-looking fields, and checks version consistency.

## Assumption Reassessment (2026-06-21)

1. `tools/fixture-check/src/main.rs` is a hard-coded `resolve_game()` registry (`:200`) that calls per-game `<game>::load_manifest()` / `load_variants()` (e.g. `:432`) and a `RegisteredGame` with `game_id`/`rules_version`/`trace_dir`/`fixture_dir`/`manifest_path`/`variants_path`/`variant_id`. The 005 skeleton stubbed `data/{manifest,variants}.toml` + the loader functions — this ticket finalizes them.
2. Spec §4.6 + §3.1 (variants row) fix the static-data boundary: identity/version/seat metadata/labels/presentation only — no schedule/hook/follow-suit/scoring/visibility/bot/trigger field. Sibling `games/briar_circuit/data/` (manifest.toml, variants.toml, fixtures/*.fixture.json) is the layout precedent.
3. Cross-artifact boundary under audit: the manifest/variants schema + the `load_*` signatures are the contract `fixture-check` consumes; unknown-field and behavior-key rejection is the fail-closed guard under audit.
4. FOUNDATIONS §5/§11 under audit: static data is typed content only; unknown fields rejected by default; behavior-looking fields blocked. The reassessment-resolved finding (fixture-check is a hard-coded registry needing `load_*` exports) is the change rationale.

## Architecture Check

1. Finalizing the data layer with the fixture-check arm together keeps the validator's target and its source in one reviewable diff and proves the fail-closed boundary immediately.
2. No shims; additive registry arm + finalized data.
3. `engine-core`/`game-stdlib` untouched; the manifest carries no behavior.

## Verification Layers

1. Typed manifest/fixtures validate; unknown + behavior-looking fields rejected → `cargo run -p fixture-check -- --game vow_tide`.
2. Version consistency across manifest/variants/rules → fixture-check version check.
3. Fixtures drive real setups (3/4/6/7p, hook, terminal tie) → fixture load + `cargo test -p vow_tide`.

## What to Change

### 1. Finalize data layer

`data/manifest.toml` + `data/variants.toml`: identity, `rules_version="vow-tide-rules-v1"`, `data` version, supported-seat metadata, labels, presentation-safe params only. Implement `load_manifest()`/`load_variants()` (typed, unknown-field-rejecting). Author `data/fixtures/{vow_tide_3p,4p,6p,7p}_standard.fixture.json`, `vow_tide_hook.fixture.json`, `vow_tide_terminal_tie.fixture.json`.

### 2. Fixture-check registration

Add the `vow_tide` `resolve_game()` arm with all paths + `variant_id = "vow_tide_standard"`.

## Files to Touch

- `games/vow_tide/data/manifest.toml` (modify)
- `games/vow_tide/data/variants.toml` (modify)
- `games/vow_tide/src/lib.rs` (modify)
- `games/vow_tide/data/fixtures/vow_tide_4p_standard.fixture.json` (new)
- `games/vow_tide/data/fixtures/vow_tide_hook.fixture.json` (new)
- `games/vow_tide/data/fixtures/vow_tide_terminal_tie.fixture.json` (new)
- `tools/fixture-check/src/main.rs` (modify)

## Out of Scope

- `replay-check`/`rule-coverage` arms (016); WASM/web registration (017/018).
- Any behavior/selector field in data.

## Acceptance Criteria

### Tests That Must Pass

1. `cargo run -p fixture-check -- --game vow_tide` — typed validation passes; unknown/behavior keys rejected.
2. `cargo test -p vow_tide` — fixtures load into real setups.
3. `cargo build -p fixture-check`.

### Invariants

1. `manifest.toml`/`variants.toml` encode no schedule/hook/legality/scoring/visibility/bot behavior.
2. Unknown and behavior-looking fields fail closed.

## Test Plan

### New/Modified Tests

1. `games/vow_tide/data/fixtures/*.fixture.json` — authoritative setup fixtures.
2. A behavior-key-rejection assertion in fixture-check (mirrors sibling-game fixture tests).

### Commands

1. `cargo run -p fixture-check -- --game vow_tide`
2. `cargo test -p vow_tide`
3. Narrower command rationale: fixture-check is the static-data boundary validator; rule/replay coverage are proven by their own tools (016).

## Outcome

Completed 2026-06-21. Finalized Vow Tide's typed static-data boundary so `manifest.toml` and `variants.toml` carry identity, version, supported-seat, and presentation metadata only. Removed scaffold schedule/deck/scoring-style metadata from the data layer and kept those behaviors Rust-owned. Added declarative setup/hook/terminal-tie fixtures for 3, 4, 6, and 7 seats plus fixture-check registration for `vow_tide`.

Fixture-check now validates Vow Tide fixture schema, version consistency, stable `seat_N` ordering, supported seat counts, unknown-key rejection, and behavior-looking key rejection, including formula and bot-policy keys. Game-level metadata parser tests also assert unknown and behavior-looking TOML fields fail closed.

Verification:

1. `cargo fmt --all --check` passed.
2. `cargo run -p fixture-check -- --game vow_tide` passed.
3. `cargo test -p vow_tide` passed.
4. `cargo build -p fixture-check` passed.
5. `cargo test -p fixture-check` passed, including Vow Tide unknown-field and `score_formula` behavior-key rejection assertions.

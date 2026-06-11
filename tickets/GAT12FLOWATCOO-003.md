# GAT12FLOWATCOO-003: Crate skeleton, workspace registration, and static data

**Status**: PENDING
**Priority**: HIGH
**Effort**: Medium
**Engine Changes**: Yes — new crate `games/flood_watch` (`Cargo.toml`, `src/*` module stubs, `src/ids.rs`, setup constants); new static data (`data/manifest.toml`, `data/variants.toml`, two fixtures); root `Cargo.toml` workspace members (modify)
**Deps**: GAT12FLOWATCOO-002

## Problem

`flood_watch` needs a registered workspace crate with the conventional `games/*` module layout and its typed static data before any behavior is implemented. The static data is the gate's highest-risk boundary (FOUNDATIONS §5): event decks invite data-driven triggers, so the manifest/variant/fixture schema must declare typed counts and constants only — district IDs, scenario constants, deck composition as counts of a closed Rust event enum, budgets, levee caps, role labels — and reject unknown and behavior-looking fields. This ticket lays the crate skeleton and the parsers that enforce that boundary.

## Assumption Reassessment (2026-06-11)

1. The template crate `games/masked_claims` has the exact module set the spec lists for `flood_watch` (verified 12 `src/*` files: `actions.rs`, `bots.rs`, `effects.rs`, `ids.rs`, `lib.rs`, `replay_support.rs`, `rules.rs`, `setup.rs`, `state.rs`, `ui.rs`, `variants.rs`, `visibility.rs`), plus `data/manifest.toml`, `data/variants.toml`, and `data/fixtures/<game>_standard.fixture.json`. `flood_watch` ships **two** fixtures (`flood_watch_standard`, `flood_watch_deluge`) per spec Scope and Assumption A3.
2. The spec (§Deliverables "Workspace and crate", "Static data", Work-breakdown item 2, FOUNDATIONS-alignment §5 row) requires typed-counts-only data and the same file-for-file shape as `masked_claims` unless a file is documented not-applicable; `token_bazaar` is the cited precedent for typed scenario constants. Assumption A3 fixes the standard scenario constants (five districts, levels 0–3, levee cap 2, 24-card deck, 3-action budget, 2 draws/phase); A4 fixes role assignment from scenario data.
3. Cross-artifact boundary under audit: root `Cargo.toml` `members` (verified: 11 game crates listed, `"games/masked_claims"` last) is the workspace registration contract — `cargo build --workspace` and every `cargo run -p <tool>` resolve crates through it. Adding `"games/flood_watch"` is additive; the new crate's `Cargo.toml` must depend on `engine-core` (for `SeededRng`, `SeatId`, `Seed`, generic envelopes) exactly as `masked_claims` does, and on no mechanic helper.
4. FOUNDATIONS §5 (static data is typed content, not behavior) and the §11 invariants "unknown fields rejected by default" / "behavior-looking fields blocked or escalated" motivate the parser design: the manifest/variant/fixture deserializers must reject unknown fields (`#[serde(deny_unknown_fields)]` per `masked_claims` convention) and refuse selectors/triggers/conditions/formulas. What an event *does* is typed Rust, not data — data declares counts of a closed `EventKind` enum only.
5. Enforcement surface: the static-data parser is a fail-closed validation surface feeding `tools/fixture-check` (registered in GAT12FLOWATCOO-015) and the deterministic setup (GAT12FLOWATCOO-004). It must not admit any field that could later carry behavior, and the fixtures must not embed the shuffled deck order (no-leak; the order is computed at setup from the seed, never authored in a publicly-served artifact).

## Architecture Check

1. A skeleton-first ticket gives every later pipeline ticket a compiling crate to extend, and isolates the §5 data-boundary decision (closed-enum counts, deny-unknown-fields) into one reviewable diff rather than smearing it across the state and rules tickets.
2. No backwards-compatibility aliasing/shims; net-new crate, additive workspace member.
3. `engine-core` gains no mechanic noun — all `district`/`flood`/`levee`/`event`/`deck`/`role`/`scenario`/`budget` types live in `games/flood_watch`. `game-stdlib` is untouched (no earned promotion; GAT12FLOWATCOO-002 confirmed first use).

## Verification Layers

1. Workspace registration -> `cargo build -p flood_watch` compiles and the crate appears in `cargo metadata`.
2. Closed-enum typed-counts data -> schema/serialization validation: manifest/variant/fixture deserialize into typed structs; deck composition is counts of a closed `EventKind` enum.
3. Unknown / behavior-looking field rejection -> a serialization unit test feeding an unknown or selector-shaped field is rejected (fail-closed, §5/§11).
4. No deck-order leak in static artifacts -> grep-proof that no fixture embeds an ordered deck array (only composition counts).

## What to Change

### 1. Workspace registration

Add `"games/flood_watch"` to root `Cargo.toml` `members`. Author `games/flood_watch/Cargo.toml` mirroring `games/masked_claims/Cargo.toml` (depend on `engine-core`; dev-deps and bench harness wired in their own tickets).

### 2. Module skeleton and IDs

Create the 12 `src/*` modules as compiling stubs with `lib.rs` re-exports following `masked_claims`. Author `src/ids.rs` with stable typed IDs: district IDs (`district_riverside`, `district_old_docks`, `district_market`, `district_terraces`, `district_gardens`), role IDs (`pumpwright`, `levee_warden`), and the closed `EventKind` enum (`Downpour { district }`, `StormSurge { district }`, `Reprieve`). Author the scenario setup constants (starting levels, deck composition, budget, draws/phase, levee cap) as typed Rust.

### 3. Static data + parsers

Author `data/manifest.toml` (display metadata, district IDs/labels, role labels), `data/variants.toml` (the two scenarios' constants as typed counts), and the two fixtures `data/fixtures/flood_watch_standard.fixture.json` + `data/fixtures/flood_watch_deluge.fixture.json` (typed metadata/constants only — no behavior, no embedded deck order). Implement `#[serde(deny_unknown_fields)]` deserializers in `variants.rs`/`setup.rs` per `masked_claims`, rejecting unknown and behavior-looking fields.

## Files to Touch

- `Cargo.toml` (modify — add `games/flood_watch` member)
- `games/flood_watch/Cargo.toml` (new)
- `games/flood_watch/src/lib.rs`, `actions.rs`, `bots.rs`, `effects.rs`, `ids.rs`, `replay_support.rs`, `rules.rs`, `setup.rs`, `state.rs`, `ui.rs`, `variants.rs`, `visibility.rs` (new — stubs; substance lands in later tickets)
- `games/flood_watch/data/manifest.toml` (new)
- `games/flood_watch/data/variants.toml` (new)
- `games/flood_watch/data/fixtures/flood_watch_standard.fixture.json` (new)
- `games/flood_watch/data/fixtures/flood_watch_deluge.fixture.json` (new)

## Out of Scope

- State transitions, legal-action generation, validation logic, effects, replay, bots (their own tickets) — modules are stubs here.
- Any selector/trigger/condition/formula in data (forbidden; §5).
- Deterministic shuffle and full setup logic (GAT12FLOWATCOO-004), beyond the constants and parsers needed to compile and validate the schema.

## Acceptance Criteria

### Tests That Must Pass

1. `cargo build -p flood_watch` compiles; `cargo build --workspace` succeeds with the new member.
2. A `serialization` unit test deserializes `manifest.toml`, both variants, and both fixtures into typed structs and **rejects** an injected unknown field and a behavior-looking (selector/trigger) field.
3. `cargo fmt --all --check` and `cargo clippy -p flood_watch -- -D warnings` pass.

### Invariants

1. `engine-core` contains no new mechanic noun (`bash scripts/boundary-check.sh` still green; the `role`/`scenario` pattern extension lands in GAT12FLOWATCOO-015).
2. Static data declares typed counts/constants only; the deck composition is counts of a closed `EventKind` enum and no fixture embeds an ordered deck.

## Test Plan

### New/Modified Tests

1. `games/flood_watch/tests/serialization.rs` — deny-unknown-fields + behavior-looking-field rejection for manifest, both variants, both fixtures (expanded in GAT12FLOWATCOO-011).
2. `games/flood_watch/data/fixtures/flood_watch_standard.fixture.json`, `flood_watch_deluge.fixture.json` — typed fixtures, no embedded deck order.

### Commands

1. `cargo test -p flood_watch serialization`
2. `cargo build --workspace && cargo clippy -p flood_watch -- -D warnings`
3. `cargo run -p fixture-check -- --game flood_watch` is the eventual full-pipeline boundary but cannot run until the tool registers the game (GAT12FLOWATCOO-015); the serialization test is the correct boundary for the skeleton diff.

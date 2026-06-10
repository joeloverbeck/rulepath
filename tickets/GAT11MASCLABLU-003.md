# GAT11MASCLABLU-003: Crate skeleton and workspace registration

**Status**: PENDING
**Priority**: HIGH
**Effort**: Small
**Engine Changes**: Yes — new crate `games/masked_claims` (`Cargo.toml`, `src/lib.rs`, `src/ids.rs`), new `data/{manifest.toml,variants.toml,fixtures/}`; modifies root `Cargo.toml` workspace members
**Deps**: GAT11MASCLABLU-002

## Problem

Subsequent pipeline tickets (state, actions, resolution, visibility, bots, tests) need a compiling crate skeleton to land into. This ticket creates `games/masked_claims/` with typed IDs, module declarations, static-data files (typed metadata only), a fixture shell, and the workspace membership so `cargo build -p masked_claims` succeeds.

## Assumption Reassessment (2026-06-10)

1. Root `Cargo.toml` lists `games/*` as workspace members (confirmed lines 8–17, `games/plain_tricks` last at line 17); this ticket appends `games/masked_claims`. `games/plain_tricks/Cargo.toml` is the shape model: `name`, `ai-core`/`engine-core` path deps, `[lib] path = "src/lib.rs"`, and a `[[bench]]` table (confirmed).
2. The spec Deliverables "Workspace and crate" row enumerates the twelve src modules (`actions.rs`, `bots.rs`, `effects.rs`, `ids.rs`, `lib.rs`, `replay_support.rs`, `rules.rs`, `setup.rs`, `state.rs`, `ui.rs`, `variants.rs`, `visibility.rs`) and the data files; mirror `games/plain_tricks` file-for-file. This ticket creates only the skeleton subset (`lib.rs`, `ids.rs`, data); the remaining modules are added by their pipeline tickets.
3. Cross-artifact boundary under audit: the root `Cargo.toml` workspace-members list is the shared boundary; this is a create-then-modify on a pre-existing file (the games/masked_claims crate is new, the workspace list is appended).
4. FOUNDATIONS §5 (static data is typed content) motivates the data files: `manifest.toml` and `variants.toml` hold tile IDs, grade labels, setup constants, variant IDs, and metadata only — no selectors, branches, or triggers; unknown and behavior-looking fields are rejected by the typed parsers (added with setup in GAT11MASCLABLU-004).

## Architecture Check

1. A skeleton-first crate lets each pipeline ticket compile incrementally and be reviewed as an isolated diff, rather than one giant crate-creation diff.
2. No backwards-compatibility aliasing or shims introduced.
3. `engine-core` stays noun-free — all `mask`/`grade`/`tile` nouns live in `games/masked_claims`; no `game-stdlib` change.

## Verification Layers

1. Crate compiles -> `cargo build -p masked_claims`.
2. Workspace membership registered -> grep-proof in root `Cargo.toml`.
3. Static data is typed/no-behavior -> schema validation via `fixture-check` (registered later in GAT11MASCLABLU-015) + manual review now that `manifest.toml`/`variants.toml` carry only typed metadata.
4. Formatting/hygiene -> `cargo fmt --all --check`.

## What to Change

### 1. `games/masked_claims/Cargo.toml`

Package `name = "masked_claims"`, `ai-core` + `engine-core` path deps, `[lib] path = "src/lib.rs"`. Do NOT add the `[[bench]]` table yet — it points at `benches/masked_claims.rs`, which is created in GAT11MASCLABLU-012; adding it now would break `cargo build`.

### 2. `games/masked_claims/src/lib.rs` and `src/ids.rs`

`lib.rs`: module declarations and re-export skeleton mirroring `games/plain_tricks/src/lib.rs`. `ids.rs`: typed IDs — `MaskTileId` (`mask_g1_a` … `mask_g5_c`), `Grade` (1..=5 with original labels), `SeatId` usage — no behavior.

### 3. Static data + fixture shell

`data/manifest.toml` (game/variant metadata, tile IDs, grade labels, setup constants), `data/variants.toml` (`masked_claims_standard`), `data/fixtures/masked_claims_standard.fixture.json` (shell; populated in GAT11MASCLABLU-011).

### 4. Root `Cargo.toml`

Append `games/masked_claims` to the `[workspace] members` list.

## Files to Touch

- `games/masked_claims/Cargo.toml` (new)
- `games/masked_claims/src/lib.rs` (new)
- `games/masked_claims/src/ids.rs` (new)
- `games/masked_claims/data/manifest.toml` (new)
- `games/masked_claims/data/variants.toml` (new)
- `games/masked_claims/data/fixtures/masked_claims_standard.fixture.json` (new)
- `Cargo.toml` (modify)

## Out of Scope

- State model and deterministic setup logic (GAT11MASCLABLU-004).
- The `[[bench]]` table and benches (GAT11MASCLABLU-012).
- Any behavior in static data (selectors, branches, triggers) — forbidden by FOUNDATIONS §5.

## Acceptance Criteria

### Tests That Must Pass

1. `cargo build -p masked_claims` succeeds.
2. Root `Cargo.toml` lists `games/masked_claims` as a workspace member.
3. `cargo fmt --all --check` passes.

### Invariants

1. `engine-core` gains no mechanic noun from this crate.
2. Static data files carry typed content/metadata only; no behavior-looking fields.

## Test Plan

### New/Modified Tests

1. `None — skeleton ticket; behavioral tests land with the modules they cover (GAT11MASCLABLU-004 onward). Compilation is the verification boundary here.`

### Commands

1. `cargo build -p masked_claims`
2. `cargo fmt --all --check`
3. A build + format check is the correct boundary: no game behavior exists yet, so full `cargo test` has nothing game-specific to assert until GAT11MASCLABLU-004.

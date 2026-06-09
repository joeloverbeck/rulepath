# GAT10POKLITBET-002: poker_lite crate skeleton, typed ids, variants, and data manifests

**Status**: PENDING
**Priority**: HIGH
**Effort**: Medium
**Engine Changes**: Yes — new crate `games/poker_lite` (`lib.rs`, `ids.rs`, `variants.rs`) + workspace member in root `Cargo.toml`; typed static-data manifests under `games/poker_lite/data/`. No `engine-core` / `game-stdlib` change.
**Deps**: GAT10POKLITBET-001

## Problem

`poker_lite` needs a compiling crate skeleton wired into the workspace before any rules/state code can land. This ticket creates the crate, its typed id vocabulary (`PokerLiteSeat`, `CrestCardId`, `CrestRank`, action-segment / variant-id constants, stable label helpers), strict typed variant parsing, and the typed static-data manifests/fixtures — establishing the `games/poker_lite/` directory all downstream tickets build into.

## Assumption Reassessment (2026-06-08)

1. The crate file layout matches the three existing card/economy games exactly: `games/secret_draft/src/` and `games/token_bazaar/src/` both contain `ids.rs`, `variants.rs`, `lib.rs` (among the 12-file set). This ticket creates `lib.rs`, `ids.rs`, `variants.rs` only; remaining `src/*` files arrive in later tickets. Verified by directory inventory this session.
2. The spec (`specs/gate-10-poker-lite-betting-showdown.md` §4 "New crate and source tree", appendix §A1 Components, §B1) names the id types and the six-card deck (ranks `low`/`middle`/`high`, two copies `dawn`/`dusk`, neutral labels `Sprout`/`Current`/`Crown`) and variant id `poker_lite_standard`. ids.rs/variants.rs encode exactly this.
3. Cross-artifact boundary under audit: the workspace `Cargo.toml` `[workspace] members` list (currently 12 games + tools, last game entry `"games/token_bazaar"`). Adding `"games/poker_lite"` is the wiring point; verified the list exists and the crate is absent. The crate's public surface (`lib.rs` re-exports) is the boundary later tickets and `wasm-api` consume.
4. FOUNDATIONS §3 (engine-core noun-free) and §5 (static data is typed content, not behavior) motivate this ticket. Restated: typed mechanic nouns (crest, rank, seat, pledge) live in `games/poker_lite`, never in `engine-core`; data files carry labels/metadata/version only — no selectors, conditions, formulas, or behavior-looking keys.
5. Fail-closed substrate surface under audit (§5/§11 unknown-field rejection): `variants.rs` performs strict typed parsing of `data/variants.toml` / `data/manifest.toml` and MUST reject unknown fields and behavior-looking keys by default — mirroring `games/secret_draft/src/variants.rs`. This is the data-side fail-closed gate; it introduces no behavior path and no hidden-info leak. Confirm the standard variant constants are Rust-owned (the data only mirrors display metadata).

## Architecture Check

1. A thin compiling skeleton (ids + variants + empty/typed lib surface) before rules/state lets every later ticket be a reviewable diff against a stable crate, rather than one mega-crate diff. Matches the established per-game crate shape.
2. No backwards-compatibility aliasing/shims — new crate.
3. `engine-core` stays free of mechanic nouns (all crest/rank/seat/pledge types are crate-local, §3); no `game-stdlib` promotion — card/private-hand is second use, accounting is second use, pledge is first use, all kept local per spec §8 (§4).

## Verification Layers

1. Crate compiles + is workspace-wired -> `cargo build -p poker_lite` and presence in `Cargo.toml` members (codebase grep-proof).
2. Typed-id fidelity (id/label set matches spec §A1) -> `cargo test -p poker_lite` id/label unit tests + manual review against spec.
3. Static-data discipline (no behavior-looking fields; unknown fields rejected) -> `variants.rs` strict-parse unit test (reject an unknown key) + schema review against `docs/ENGINE-GAME-DATA-BOUNDARY.md` (§5/§11).
4. Boundary stays clean (no mechanic noun in engine-core) -> `bash scripts/boundary-check.sh`.

## What to Change

### 1. `games/poker_lite/Cargo.toml` + root `Cargo.toml`

Create the crate manifest mirroring `games/secret_draft/Cargo.toml` dependencies (engine-core, serde, etc., consistent with siblings). Add `"games/poker_lite"` to root `Cargo.toml` `[workspace] members` after `"games/token_bazaar"`.

### 2. `games/poker_lite/src/ids.rs`

Define `PokerLiteSeat` (`seat_0`/`seat_1`), `CrestCardId`, `CrestRank` (`low`/`middle`/`high` with rank values + copies `dawn`/`dusk`), action-segment constants (`hold`/`press`/`lift`/`match`/`yield`), variant-id constant `poker_lite_standard`, and stable label helpers (`Sprout`/`Current`/`Crown`). Neutral labels only.

### 3. `games/poker_lite/src/variants.rs`

Strict typed parsing of the data manifests; reject unknown and behavior-looking keys (fail-closed). Standard variant constants Rust-owned; data mirrors display metadata only.

### 4. `games/poker_lite/src/lib.rs`

Public crate surface (module declarations + re-exports) matching the established games' `lib.rs` shape, exporting only what exists at this stage.

### 5. `games/poker_lite/data/{manifest.toml,variants.toml}` + `data/fixtures/poker_lite_standard.fixture.json`

Typed content/labels/variant metadata/version + a standard-variant fixture skeleton, mirroring `games/secret_draft/data/` layout (`fixtures/<game>_standard.fixture.json`).

## Files to Touch

- `games/poker_lite/Cargo.toml` (new)
- `games/poker_lite/src/lib.rs` (new)
- `games/poker_lite/src/ids.rs` (new)
- `games/poker_lite/src/variants.rs` (new)
- `games/poker_lite/data/manifest.toml` (new)
- `games/poker_lite/data/variants.toml` (new)
- `games/poker_lite/data/fixtures/poker_lite_standard.fixture.json` (new)
- `Cargo.toml` (modify — add workspace member)

## Out of Scope

- `setup.rs`, `state.rs`, `actions.rs`, `rules.rs`, `effects.rs`, `visibility.rs`, `ui.rs`, `bots.rs`, `replay_support.rs` (later tickets).
- Any rule transition, accounting, or showdown logic.
- WASM registration and tool registration (GAT10POKLITBET-012/014).
- Any behavior-looking field in data (§5 stop condition).

## Acceptance Criteria

### Tests That Must Pass

1. `cargo build -p poker_lite` succeeds.
2. `cargo test -p poker_lite` passes (id/label fidelity + variant strict-parse reject-unknown test).
3. `bash scripts/boundary-check.sh` confirms `engine-core` remains noun-free after the workspace addition.

### Invariants

1. All crest/rank/seat/pledge types live in `games/poker_lite` — none added to `engine-core` (§3).
2. `data/*` files contain typed content/labels/metadata/version only; `variants.rs` rejects unknown and behavior-looking keys (§5/§11).

## Test Plan

### New/Modified Tests

1. `games/poker_lite/src/ids.rs` (inline `#[cfg(test)]`) — id/rank/label fidelity against spec §A1.
2. `games/poker_lite/src/variants.rs` (inline `#[cfg(test)]`) — strict parse accepts the standard manifest, rejects an injected unknown/behavior-looking key.

### Commands

1. `cargo build -p poker_lite`
2. `cargo test -p poker_lite`
3. `bash scripts/boundary-check.sh` — the correct boundary verification surface; full `cargo test --workspace` is deferred to later tickets once more surface exists.

# GAT15RIVLEDTEX-003: Crate scaffold, workspace wiring, and typed static-data boundary

**Status**: COMPLETED
**Priority**: HIGH
**Effort**: Medium
**Engine Changes**: Yes — new crate `games/river_ledger` (`ids.rs`, `cards.rs`, `state.rs`, `variants.rs`, `ui.rs` stubs, `lib.rs`), `data/manifest.toml`, `data/variants.toml`; root `Cargo.toml` workspace member
**Deps**: GAT15RIVLEDTEX-002

## Problem

River Ledger needs a compiling game crate with typed modules, deterministic static-data loading, and zero behavior in data, so every downstream pipeline ticket has stub modules and shared types to extend. This is the create-then-modify root for the gate.

## Assumption Reassessment (2026-06-14)

1. `games/poker_lite/src/` has the 12-module shape (`ids/cards/state/setup/actions/betting/.../variants/ui/visibility/effects/replay_support`); root `Cargo.toml` lists members by bare game id (`games/poker_lite`) and `[package] name = "poker_lite"`, so this crate is `name = "river_ledger"`, `cargo -p river_ledger`.
2. `specs/...-base.md` §4.1 fixes the module responsibilities and the `data/` layout (`manifest.toml`, `variants.toml`, `fixtures/`); the admission spine (002) is reviewed, satisfying §11.2 before coding.
3. Cross-artifact boundary under audit: this ticket creates `games/river_ledger/src/lib.rs` (the `mod` spine appended by 004–010/013) and stubs `ui.rs` (filled in 008) and the typed-state structs (`Phase`, `Street`, `SeatStatus`, `SeatLedger`, `ContributionLedger`, `TerminalOutcome`, ...) consumed across the pipeline.
4. FOUNDATIONS §3 (engine-core noun-free) motivates this ticket: all card/deck/rank/suit/pot/blind/button/street/evaluator nouns are game-local types in `games/river_ledger`; `engine-core`'s generic `SeatId`/`Actor`/`Viewer`/`VisibilityScope`/`Game` are reused, never extended with mechanic nouns.
5. Fail-closed static-data validation (§5/§11) under audit: `variants.rs` loads `data/*.toml` as typed metadata/setup parameters only and rejects unknown or behavior-looking keys (selectors/conditions/triggers/formulas); the variant-loader rejection test is the enforcement surface. No YAML, no DSL.

## Architecture Check

1. A thin compiling skeleton with typed stubs lets each later ticket be a small reviewable diff against a stable module/type contract, matching the proven new-crate gate shape.
2. No backwards-compatibility aliasing/shims — greenfield crate; all files `(new)` except the root `Cargo.toml` member append.
3. `engine-core` stays noun-free (§3, asserted by `boundary-check.sh`); no `game-stdlib` change (§4); static data carries no behavior (§5).

## Verification Layers

1. Crate compiles and is a workspace member -> `cargo check -p river_ledger`.
2. `engine-core` gains no mechanic noun from the new crate -> `bash scripts/boundary-check.sh`.
3. Static-data loader rejects unknown/behavior-looking keys -> `variants.rs` unit test (fail-closed) per §5/§11.

## What to Change

### 1. Crate skeleton + workspace

Create `games/river_ledger/Cargo.toml` and `src/lib.rs` with the module spine; add `"games/river_ledger"` to the root `Cargo.toml` workspace members.

### 2. Typed modules (stubs) + static data

`ids.rs` (`RiverLedgerSeat`, `STANDARD_MIN_SEATS = 3`, `STANDARD_MAX_SEATS = 6`, `RL-*` prefix constants, actor/viewer helpers); `cards.rs` (game-local `Rank`/`Suit`/`Card`/`Deck`, deterministic construction, public labels); `state.rs` (phase/street/seat-status/ledger/terminal/showdown record types); `variants.rs` (typed loader with behavior-key rejection); `ui.rs` (stub for 008); `data/manifest.toml` + `data/variants.toml`; create the `data/fixtures/` directory.

## Files to Touch

- `games/river_ledger/Cargo.toml` (new)
- `games/river_ledger/src/lib.rs` (new)
- `games/river_ledger/src/ids.rs` (new)
- `games/river_ledger/src/cards.rs` (new)
- `games/river_ledger/src/state.rs` (new)
- `games/river_ledger/src/variants.rs` (new)
- `games/river_ledger/src/ui.rs` (new — stub, filled in 008)
- `games/river_ledger/data/manifest.toml` (new)
- `games/river_ledger/data/variants.toml` (new)
- `Cargo.toml` (modify — add workspace member)

## Out of Scope

- Setup/deal/betting/evaluator/showdown logic (004–007).
- Effects/visibility/replay (008–010); `ui.rs` is a stub here.
- Any tool/WASM/web registration (014–018).

## Acceptance Criteria

### Tests That Must Pass

1. `cargo check -p river_ledger` — crate compiles as a workspace member.
2. `cargo test -p river_ledger` — variant-loader behavior-key rejection unit test passes.
3. `bash scripts/boundary-check.sh` — `engine-core` stays noun-free.

### Invariants

1. All mechanic nouns are game-local; `engine-core` is unchanged (§3).
2. `data/*.toml` holds metadata/parameters only; unknown/behavior-looking keys are rejected (§5/§11); no YAML/DSL.

## Test Plan

### New/Modified Tests

1. `games/river_ledger/src/variants.rs` (new) — unit test asserting unknown/behavior-looking data keys fail loading.

### Commands

1. `cargo check -p river_ledger`
2. `cargo test -p river_ledger && bash scripts/boundary-check.sh`
3. `cargo fmt --all --check`

## Outcome

Completed: 2026-06-14

Implemented the River Ledger crate scaffold and typed static-data boundary:

- Added `games/river_ledger` as a workspace member with a compiling
  `river_ledger` crate.
- Added the module spine and typed stubs for IDs, cards, state, UI metadata, and
  variants/static-data loading.
- Added `data/manifest.toml`, `data/variants.toml`, and a tracked
  `data/fixtures/.gitkeep` placeholder.
- Added fail-closed flat TOML parsing that rejects unknown keys and
  behavior-looking keys such as `selector`, `trigger`, `valid_if`, and
  `showdown_formula`.
- Let Cargo update `Cargo.lock` with the new workspace package entry.

Deviations: none. The ticket intentionally did not implement setup/deal,
betting, evaluator, showdown, visibility/effects/replay, tools, WASM, or web
registration.

Verification:

- `cargo check -p river_ledger` passed.
- `cargo test -p river_ledger` passed (6 tests).
- `bash scripts/boundary-check.sh` passed (`engine-core boundary check passed`).
- `cargo fmt --all --check` passed after formatting the new Rust files.

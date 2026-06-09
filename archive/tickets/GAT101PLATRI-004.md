# GAT101PLATRI-004: plain_tricks crate skeleton, typed ids, variants, and data manifests

**Status**: COMPLETED
**Priority**: HIGH
**Effort**: Medium
**Engine Changes**: Yes — new crate `games/plain_tricks` (added to workspace `Cargo.toml`); new `src/{ids,variants,lib}.rs` skeleton, `data/{manifest,variants}.toml`, fixture manifest. No `engine-core`/`game-stdlib` change.
**Deps**: GAT101PLATRI-002

## Problem

The game needs a compiling crate skeleton with typed ids, variant constants, strict data parsing, and a fixture manifest before rules code can be written. This mirrors `games/poker_lite` and is the foundation every later `plain_tricks` ticket builds on. Per FOUNDATIONS §4/§12 and spec §3 "not allowed", no shuffle/hand/deal code is written until GAT101PLATRI-002's ledger decision is recorded — hence the `Deps`.

## Assumption Reassessment (2026-06-09)

1. `games/poker_lite/src/{ids,variants,lib}.rs`, `games/poker_lite/data/{manifest,variants}.toml`, and `games/poker_lite/data/fixtures/poker_lite_standard.fixture.json` exist as the structural template; `games/plain_tricks/` does not yet exist. Root `Cargo.toml` `members` lists `games/poker_lite` etc. and is the wiring point.
2. Spec §4 fixes the typed ids: `PlainTricksSeat`, `TrickCardId`, `TrickSuit`, `TrickRank`, action-segment constants, variant id constants (`plain_tricks_standard`), stable label helpers; `variants.rs` does strict typed parsing rejecting behavior-looking keys.
3. Shared boundary under audit: workspace `Cargo.toml` `members`, and the static-data manifest schema (`docs/ENGINE-GAME-DATA-BOUNDARY.md`). Data files carry typed content/labels/variant metadata/version only.
4. FOUNDATIONS §3 (no card/deck/suit/rank/trick noun in `engine-core` — typed nouns live in `games/plain_tricks`) and §5 (static data is typed content, not behavior) are under audit.
5. The fixture manifest + `data/*.toml` extend the static-data manifest contract additively (a new per-game manifest, not a change to a shared schema). The schema must reject unknown fields and refuse behavior-looking keys (selectors/conditions/formulas/follow-suit/trick-winner/scoring/bot-policy/deal-routing) per spec §4.

## Architecture Check

1. A compile-only skeleton with typed ids and strict data parsing (vs. writing rules + ids together) keeps the first diff small and reviewable and isolates the workspace-wiring change.
2. No backwards-compatibility aliasing/shims — new crate, new types.
3. `engine-core` untouched and stays noun-free; all card/suit/trick nouns are `games/plain_tricks`-local (FOUNDATIONS §3). No `game-stdlib` change here.

## Verification Layers

1. Crate compiles and is workspace-wired -> simulation/CLI build (`cargo build -p plain_tricks`).
2. `engine-core` stays noun-free after the new crate lands -> `bash scripts/boundary-check.sh`.
3. Data manifests reject unknown/behavior-looking fields -> schema/serialization validation via `variants.rs` strict parse unit test.
4. Workspace members updated -> codebase grep-proof on `Cargo.toml`.

## What to Change

### 1. Workspace wiring

Add `"games/plain_tricks"` to root `Cargo.toml` `members`. Add `games/plain_tricks/Cargo.toml` with dependencies consistent with `poker_lite`.

### 2. `src/ids.rs`, `src/variants.rs`, `src/lib.rs`

Define `PlainTricksSeat`, `TrickCardId`, `TrickSuit`, `TrickRank`, action-segment constants, variant id constants, and stable label helpers (`Gale`/`River`/`Ember`, ranks 1–6). `variants.rs` strictly parses `data/variants.toml`, rejecting behavior-looking keys. `lib.rs` exposes the compile-only public surface matching established games.

### 3. `data/` manifests + fixture manifest

Add `data/manifest.toml`, `data/variants.toml`, and `data/fixtures/plain_tricks_standard.fixture.json` (manifest scaffold) with typed content/labels/version only.

## Files to Touch

- `Cargo.toml` (modify)
- `games/plain_tricks/Cargo.toml` (new)
- `games/plain_tricks/src/ids.rs` (new)
- `games/plain_tricks/src/variants.rs` (new)
- `games/plain_tricks/src/lib.rs` (new)
- `games/plain_tricks/data/manifest.toml` (new)
- `games/plain_tricks/data/variants.toml` (new)
- `games/plain_tricks/data/fixtures/plain_tricks_standard.fixture.json` (new)

## Out of Scope

- Shuffle/deal/state logic (GAT101PLATRI-005), action tree (GAT101PLATRI-006), rules (GAT101PLATRI-007).
- Tool/WASM/web registration (later tickets).

## Acceptance Criteria

### Tests That Must Pass

1. `cargo build -p plain_tricks` compiles.
2. `bash scripts/boundary-check.sh` passes (no mechanic noun in `engine-core`).
3. `variants.rs` unit test rejects an unknown/behavior-looking data key.

### Invariants

1. `engine-core` contains no card/deck/suit/rank/trick noun (FOUNDATIONS §3).
2. Static data contains no selectors/formulas/triggers/behavior-looking fields (FOUNDATIONS §5).

## Test Plan

### New/Modified Tests

1. `games/plain_tricks/src/variants.rs` strict-parse unit test — rejects unknown fields and behavior-looking keys.
2. `games/plain_tricks/data/fixtures/plain_tricks_standard.fixture.json` manifest scaffold — consumed later by `fixture-check`.

### Commands

1. `cargo build -p plain_tricks`
2. `cargo test -p plain_tricks && bash scripts/boundary-check.sh`
3. A per-crate build + boundary check is the correct boundary: this ticket adds only the skeleton; full pipeline checks belong to later tickets.

## Outcome

Completed: 2026-06-09

What changed:

- Added `games/plain_tricks` as a workspace crate and updated `Cargo.lock` with the new package entry.
- Added compile-only crate surface in `games/plain_tricks/src/{ids,variants,lib}.rs`.
- Defined `PlainTricksSeat`, `TrickSuit`, `TrickRank`, `TrickCardId`, `ACTION_PLAY`, `plain_tricks_standard`, `plain-tricks-rules-v1`, stable label helpers, and canonical deck order.
- Added strict static-data parsing for manifest, variant catalog, and fixture scaffold, including unknown-field rejection and behavior-looking key rejection.
- Added `games/plain_tricks/data/{manifest,variants}.toml` and `games/plain_tricks/data/fixtures/plain_tricks_standard.fixture.json` with metadata only.

Deviations from original plan:

- `Cargo.lock` changed as a consequence of adding the workspace member and building the new crate. No rules/setup/action/visibility behavior was added.

Verification results:

- `cargo fmt --all --check` passed after applying rustfmt.
- `cargo build -p plain_tricks` passed.
- `cargo test -p plain_tricks` passed: 9 tests passed.
- `bash scripts/boundary-check.sh` passed.

# GAT72GAT8HIG-003: high_card_duel crate skeleton + workspace wiring + data manifests

**Status**: PENDING
**Priority**: HIGH
**Effort**: Small
**Engine Changes**: Yes — new crate `games/high_card_duel` (`Cargo.toml`, `src/lib.rs` + module stubs, `src/variants.rs`, `data/manifest.toml`, `data/variants.toml`); workspace `Cargo.toml` members
**Deps**: GAT72GAT8HIG-002

## Problem

Gate 8 needs a compiling crate skeleton under `games/high_card_duel` registered
in the workspace, with the static data files (`manifest.toml`, `variants.toml`)
and a typed variants module, so the pipeline tickets (004+) can fill modules
against a structure that already builds and is convention-accurate.

## Assumption Reassessment (2026-06-07)

1. Verified the sibling crate shape: `games/draughts_lite/Cargo.toml` declares
   `ai-core`/`engine-core`/`game-stdlib` path deps, `[lib] path = "src/lib.rs"`,
   and `[[bench]] name = "draughts_lite" harness = false`; `src/` holds
   `lib.rs ids.rs state.rs setup.rs actions.rs rules.rs effects.rs visibility.rs
   variants.rs bots.rs replay_support.rs ui.rs`; `data/` holds `manifest.toml`,
   `variants.toml`, `fixtures/`.
2. Verified workspace registration: root `Cargo.toml:8-12` lists each game under
   `members`; `high_card_duel` must be added there (the four CLI tools and
   `cargo test --workspace` resolve members from this list).
3. Cross-artifact boundary under audit: the workspace `members` list and the
   static-data manifest schema (`docs/ENGINE-GAME-DATA-BOUNDARY.md`) — the new
   `manifest.toml`/`variants.toml` must be typed content only.
4. FOUNDATIONS principle under audit (§5 static data is not behavior): the data
   files carry display metadata, the `high_card_duel_standard` variant, and
   setup constants (deck size, ranks, hand size, round count) — no selectors,
   triggers, or rule branches.

## Architecture Check

1. A buildable skeleton + data scaffolding first lets later tickets land as
   focused diffs against a compiling crate — cleaner than a single mega-crate
   ticket. Module stubs are empty/`todo!()`-free placeholders that compile.
2. No backwards-compatibility shims — greenfield crate.
3. `engine-core` untouched and noun-free; the crate depends on it, never the
   reverse. `game-stdlib` is depended on but no new helper is added (cards stay
   local; FOUNDATIONS §4).

## Verification Layers

1. Crate builds -> simulation/CLI run: `cargo build -p high_card_duel` succeeds.
2. Workspace registration -> codebase grep-proof: `high_card_duel` present in root `Cargo.toml` `members`.
3. Static-data typing -> schema/serialization validation: `manifest.toml`/`variants.toml` parse as typed content with no behavior-looking fields (§5).
4. Boundary intact -> FOUNDATIONS alignment check: `bash scripts/boundary-check.sh` passes (engine-core stays noun-free).

## What to Change

### 1. Crate package + modules

Create `games/high_card_duel/Cargo.toml` (mirroring draughts_lite deps + bench
target) and `src/lib.rs` re-exporting empty module stubs: `ids state setup
actions rules effects visibility variants bots replay_support ui`. Implement
`variants.rs` with the typed `high_card_duel_standard` variant (six rounds,
3-card hands, 24-card deck) loaded from `data/variants.toml`.

### 2. Static data

- `data/manifest.toml`: game id, display name, rules/schema version, neutral
  themed metadata.
- `data/variants.toml`: the `high_card_duel_standard` variant parameters.
- Create `data/fixtures/` (fixture JSON authored later in GAT72GAT8HIG-012).

### 3. Workspace

Add `"games/high_card_duel"` to root `Cargo.toml` `members`.

## Files to Touch

- `games/high_card_duel/Cargo.toml` (new)
- `games/high_card_duel/src/lib.rs` (new)
- `games/high_card_duel/src/{ids,setup,actions,rules,effects,visibility,bots,replay_support,ui}.rs` (new — stubs)
- `games/high_card_duel/src/variants.rs` (new)
- `games/high_card_duel/data/manifest.toml` (new)
- `games/high_card_duel/data/variants.toml` (new)
- `Cargo.toml` (modify — add workspace member)

## Out of Scope

- Rules/setup/effects/views/bots logic (subsequent tickets fill the stubs).
- Tool/WASM/CI registration (GAT72GAT8HIG-013/014/015/016/019).

## Acceptance Criteria

### Tests That Must Pass

1. `cargo build -p high_card_duel` — compiles.
2. `cargo build --workspace` — workspace still builds with the new member.
3. `bash scripts/boundary-check.sh` — passes.

### Invariants

1. `manifest.toml`/`variants.toml` contain only typed content/parameters/metadata (no behavior-looking fields, §5).
2. The crate depends on `engine-core`/`game-stdlib`; neither depends on it.

## Test Plan

### New/Modified Tests

1. `games/high_card_duel/src/variants.rs` — a unit test asserting the standard variant parses with six rounds / 3-card hands / 24-card deck.

### Commands

1. `cargo build -p high_card_duel`
2. `cargo test -p high_card_duel` (variant parse test)
3. `cargo build --workspace` is the correct full-pipeline boundary — proves the new member doesn't break the tree.

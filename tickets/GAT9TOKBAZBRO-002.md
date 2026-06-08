# GAT9TOKBAZBRO-002: token_bazaar crate skeleton + workspace wiring + data manifests

**Status**: PENDING
**Priority**: HIGH
**Effort**: Small
**Engine Changes**: Yes — new crate `games/token_bazaar` (`Cargo.toml`, `src/lib.rs`, `src/ids.rs`), workspace member in root `Cargo.toml`, static-data manifests (`data/manifest.toml`, `data/variants.toml`)
**Deps**: GAT9TOKBAZBRO-001

## Problem

Gate 9 needs a compilable home for the new game before rules/state/bot work can
land. This ticket creates the `games/token_bazaar` crate skeleton, registers it as
a workspace member, defines the typed ID vocabulary (seats, resources, bundles,
contracts, slots) the rest of the game references, and authors the static-data
manifests (game metadata + the `token_bazaar_standard` variant). It is the
foundation every subsequent `games/token_bazaar/*` ticket builds on.

## Assumption Reassessment (2026-06-08)

1. The sibling crate `games/high_card_duel` defines the house skeleton shape:
   `Cargo.toml`, `src/lib.rs` (module re-exports), `src/ids.rs`, `data/manifest.toml`,
   `data/variants.toml` (all verified present). `games/token_bazaar` does not exist
   yet, so all crate files are `(new)`; `src/lib.rs` is created here and modified
   (mod declarations added) by later tickets, which therefore `Deps` on this one.
2. Root `Cargo.toml` lists workspace members including `"games/high_card_duel"`
   (verified at `Cargo.toml:13`); this ticket adds `"games/token_bazaar"` to that
   `members` list. RULES.md (GAT9TOKBAZBRO-001) fixes the ID names (resource ids
   `amber`/`jade`/`iron`, bundle ids, contract ids `balanced-wares`…`crown-route`,
   slot ids `slot_0`…`slot_2`) this ticket encodes.
3. Cross-artifact boundary under audit: the static-data manifest schema. The
   per-game `data/manifest.toml` + `data/variants.toml` shape is the contract the
   game loader and WASM catalog consume; this ticket conforms to the existing
   `high_card_duel` manifest shape, adding a new game entry (additive, not a
   schema change).
4. FOUNDATIONS §3 (`engine-core` is noun-free) and §5 (static data is typed
   content, not behavior) motivate this ticket: the resource/contract/slot nouns
   live only in `games/token_bazaar`; the manifests carry typed metadata,
   variant ids, and setup constants only — no selectors, formulas, or triggers.
   `bash scripts/boundary-check.sh` enforces the §3 half.
5. Deterministic-serialization substrate: `ids.rs` defines the canonical typed
   identifiers that feed stable serialization and replay/hash in later tickets.
   The IDs must have a stable string form and stable ordering (e.g. `rNcM`-style
   discipline for slots/bundles) so GAT9TOKBAZBRO-007/010 replay hashes are
   reproducible; this ticket confirms the ID encoding is deterministic and
   carries no hidden state.
6. Static-data manifest entry: the manifest names `game_id = token_bazaar`,
   display metadata, and the `token_bazaar_standard` variant. Consumers are the
   game loader, `fixture-check`, `rule-coverage`, and the WASM catalog
   (GAT9TOKBAZBRO-012/013). The entry is additive-only — a new game alongside
   existing ones; no existing manifest is mutated.

## Architecture Check

1. A thin skeleton that compiles with only IDs + manifests (rules/state added by
   later tickets) keeps each diff reviewable and matches the proven
   `high_card_duel` layout; the alternative (one giant crate ticket) would be
   unreviewable.
2. No backwards-compatibility aliasing/shims — new crate, new manifests.
3. `engine-core` is untouched; no mechanic noun enters the kernel. `game-stdlib`
   is untouched — no resource/market/contract helper is created (the spec's
   generic-promotion decision keeps accounting local first-use; atlas records it
   as a later candidate with no open promotion debt).

## Verification Layers

1. Crate compiles + is a workspace member -> `cargo build -p token_bazaar`.
2. `engine-core` stays noun-free after the new crate lands -> `bash scripts/boundary-check.sh`.
3. Manifest/variant shape conforms to the loader contract -> `cargo test -p token_bazaar`
   (skeleton manifest-parse test) and later `fixture-check` (GAT9TOKBAZBRO-012).
4. ID string forms are stable/deterministic -> grep-proof of the `ids.rs`
   canonical string mapping + unit test asserting round-trip parse/format.

## What to Change

### 1. Root `Cargo.toml`

Add `"games/token_bazaar"` to the `[workspace] members` list (alphabetical /
existing order alongside `games/high_card_duel`).

### 2. `games/token_bazaar/Cargo.toml`

New crate manifest mirroring `games/high_card_duel/Cargo.toml` (package name
`token_bazaar`, edition, `engine-core` path dep; no `game-stdlib` dep unless a
later ticket earns it — it does not).

### 3. `games/token_bazaar/src/lib.rs`

Module root: re-export public types; declare `mod ids;` now, with later tickets
adding `mod state; mod setup; mod actions; mod rules; mod effects; mod visibility;
mod variants; mod replay_support; mod bots; mod ui;` as they land.

### 4. `games/token_bazaar/src/ids.rs`

Typed IDs with stable string forms: seats (`seat_0`/`seat_1`), resources
(`amber`/`jade`/`iron`), collect bundles (`amber`,`jade`,`iron`,`amber-jade`,
`jade-iron`,`iron-amber`), contracts (`balanced-wares`…`crown-route`), slots
(`slot_0`…`slot_2`). Provide stable parse/format and stable ordering.

### 5. `games/token_bazaar/data/manifest.toml` + `data/variants.toml`

Game metadata (`game_id = token_bazaar`, display name `Token Bazaar`) and the
`token_bazaar_standard` variant entry, following the `high_card_duel` manifest
shape. Typed content only.

## Files to Touch

- `Cargo.toml` (modify)
- `games/token_bazaar/Cargo.toml` (new)
- `games/token_bazaar/src/lib.rs` (new)
- `games/token_bazaar/src/ids.rs` (new)
- `games/token_bazaar/data/manifest.toml` (new)
- `games/token_bazaar/data/variants.toml` (new)

## Out of Scope

- State, setup, actions, rules, effects, visibility, replay, bots, UI (later tickets).
- Adding any `game-stdlib` helper or `engine-core` type.
- The JSON fixture (`data/fixtures/...`) — authored with the tests (GAT9TOKBAZBRO-009).

## Acceptance Criteria

### Tests That Must Pass

1. `cargo build -p token_bazaar` — crate compiles as a workspace member.
2. `cargo test -p token_bazaar` — the ID round-trip and manifest-parse skeleton
   tests pass.
3. `bash scripts/boundary-check.sh` — `engine-core` remains noun-free.

### Invariants

1. `engine-core` and `game-stdlib` are unchanged by this ticket.
2. Every typed ID has a stable, deterministic string form (no nondeterministic
   ordering) so downstream replay/hash stays reproducible.

## Test Plan

### New/Modified Tests

1. `games/token_bazaar/src/ids.rs` (unit) — ID parse/format round-trip + stable ordering.
2. `games/token_bazaar/src/lib.rs` (unit) — manifest/variant loads and exposes
   `token_bazaar_standard`.

### Commands

1. `cargo build -p token_bazaar`
2. `cargo test -p token_bazaar && bash scripts/boundary-check.sh`
3. Workspace-wide build is unnecessary here; the crate is additive and isolated,
   so the per-crate build + boundary check is the correct verification boundary.

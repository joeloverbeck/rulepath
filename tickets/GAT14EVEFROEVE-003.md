# GAT14EVEFROEVE-003: Crate skeleton, workspace registration, and typed data parsing

**Status**: PENDING
**Priority**: HIGH
**Effort**: Medium
**Engine Changes**: Yes — new crate `games/event_frontier` (all `src/*.rs` modules, stubbed where later tickets fill them); root `Cargo.toml` workspace member; typed static data (`data/manifest.toml`, `data/variants.toml`, `data/cards.toml`)
**Deps**: GAT14EVEFROEVE-002

## Problem

The gate needs its crate skeleton and the typed parsing layer that proves the gate's defining boundary: **nothing in any data file says what a card does**. Card data is an inventory of typed identity and parameters only — card ID, label, epoch pool, first-eligible faction, ops value, edict flag, UI metadata — parsed into a **closed `CardId` enum** and typed structs that reject unknown and behavior-looking fields. This ticket creates `games/event_frontier/`, registers it in the workspace, defines the typed ID enums (`CardId`, `FactionId`, `SiteId`), and parses the manifest, the three scenario variants, and the card/map inventories. Behavior (what each card does) is deferred to ticket 007's exhaustive Rust match.

## Assumption Reassessment (2026-06-12)

1. The crate file shape mirrors the twelve-file sibling layout plus a game-specific `cards.rs`: verified `games/flood_watch/src/` and `games/frontier_control/src/` each hold `actions, bots, effects, ids, lib, replay_support, rules, setup, state, ui, variants, visibility` (no `cards.rs`); `cards.rs` is new to this gate (spec Deliverables "Workspace and crate") and has no precedent file.
2. The workspace members list and typed-data convention are current: verified root `Cargo.toml` lists each game under `members` (e.g. `games/frontier_control`, line ~20); typed static data parses via serde with `#[serde(deny_unknown_fields)]` per the sibling games and `docs/ENGINE-GAME-DATA-BOUNDARY.md`. The exact card-inventory file (`data/cards.toml` vs a `[[card]]` array in `manifest.toml`) follows the sibling typed-data convention; implementer confirms against `docs/ENGINE-GAME-DATA-BOUNDARY.md` and `games/token_bazaar/data` (closest typed-constants precedent).
3. Cross-crate boundary under audit: the closed `CardId` enum authored here is the contract every later rule ticket matches exhaustively (005 reveal, 007 effects, 008 Reckoning); it must be greppable and stable from first authoring, and the data parse must reject behavior-looking fields so the §5 boundary holds from the skeleton on.
4. FOUNDATIONS §5 (static data is typed content, not behavior) and §3 (`engine-core` kernel boundary) motivate this ticket. Restated before trusting the spec: an event deck is "precisely where static files start acting procedural" (§12); the parse must reject `when`/`if`/`condition`/`trigger`/`selector`/`effect`/`script` and unknown fields, and every mechanic noun (`card`, `deck`, `event`, `edict`, `site`, `faction`) stays in `games/event_frontier`, never `engine-core`.
5. Substrate for deferred enforcement: the unknown-field/behavior-field rejection authored here is the §11 fail-closed surface that serialization tests (ticket 011) exercise; confirm the data model introduces no nondeterminism (deck order is data, shuffled deterministically in ticket 004) and no leakage path (undrawn order is internal state, never serialized into card data). No replay/hash semantics change — this is additive new-crate scaffolding.

## Architecture Check

1. A closed `CardId` enum with an exhaustive-match obligation is cleaner and safer than a string-keyed card registry: the compiler forces every card to have typed behavior in Rust, making "behavior in data" impossible to express.
2. No backwards-compatibility aliasing/shims — new crate, additive workspace member.
3. `engine-core` stays free of mechanic nouns (all typed nouns live in `games/event_frontier`); no `game-stdlib` promotion — the §4 ledger (ticket 002) authorized none.

## Verification Layers

1. Crate builds and registers -> `cargo build -p event_frontier`; grep root `Cargo.toml` for `games/event_frontier`.
2. Typed data parses, rejects unknown/behavior fields (§5/§11) -> a parse unit test that loads `manifest.toml`/`variants.toml`/`cards.toml` and a negative test that a behavior-looking field (`trigger`/`effect`) is rejected.
3. Closed-enum boundary (§3) -> grep `cards.rs`/`ids.rs` for the `CardId` enum; `bash scripts/boundary-check.sh` stays green (no kernel noun leak).
4. Deterministic-data invariant -> the card inventory carries no order-dependent behavior; deck order is established in ticket 004, not here (manual review).

## What to Change

### 1. Crate and workspace scaffolding

Create `games/event_frontier/Cargo.toml` (mirroring a sibling game's dependencies on `engine-core`, `game-stdlib`, `ai-core`, serde) and add `"games/event_frontier"` to the root `Cargo.toml` `members`. Create `src/lib.rs` declaring all modules, and stub modules `actions.rs`, `rules.rs`, `effects.rs`, `state.rs`, `setup.rs`, `ui.rs`, `visibility.rs`, `bots.rs`, `replay_support.rs`, `variants.rs` (filled by later tickets in dependency order).

### 2. Typed IDs and card inventory

Author `src/ids.rs` (`FactionId` = {`faction_charter`, `faction_freeholders`}, `SiteId` = the six sites, `CardId` = the closed 21-card enum) and `src/cards.rs` (typed card-data struct: id, label, epoch pool, first-eligible faction, ops value, edict flag, UI metadata — **no behavior fields**; the closed `CardId` enum; the parse from `data/cards.toml`). Effect bodies are stubbed/deferred to ticket 007.

### 3. Static data files

Author `data/manifest.toml` (display metadata, data version), `data/variants.toml` (the three scenarios: standard, hard_winter, land_rush — typed parameters only: starts, resources, thresholds, epoch composition), and `data/cards.toml` (the 21-card typed inventory). All parse with `deny_unknown_fields`.

## Files to Touch

- `Cargo.toml` (modify) — add `games/event_frontier` workspace member
- `games/event_frontier/Cargo.toml` (new)
- `games/event_frontier/src/lib.rs` (new)
- `games/event_frontier/src/ids.rs` (new)
- `games/event_frontier/src/cards.rs` (new) — closed `CardId` enum + card-data parse; effect bodies deferred to 007
- `games/event_frontier/src/{actions,rules,effects,state,setup,ui,visibility,bots,replay_support,variants}.rs` (new, stubs)
- `games/event_frontier/data/manifest.toml` (new)
- `games/event_frontier/data/variants.toml` (new)
- `games/event_frontier/data/cards.toml` (new)

## Out of Scope

- State transitions, setup shuffle, and scenario fixtures (ticket 004).
- Eligibility/initiative flow, operations, event/edict effect bodies, Reckoning (tickets 005–008).
- Any `wasm-api` registration (ticket 014) — the new crate does not modify `wasm-api` here.

## Acceptance Criteria

### Tests That Must Pass

1. `cargo build -p event_frontier` succeeds with all stub modules compiling.
2. `cargo test -p event_frontier` runs the data-parse test (manifest/variants/cards load) and the negative test (a behavior-looking or unknown field is rejected).
3. `bash scripts/boundary-check.sh` passes (no mechanic noun leaked into `engine-core`).

### Invariants

1. The `CardId` enum is closed; card data carries identity and typed parameters only — no `when`/`condition`/`trigger`/`selector`/`effect`/`script` field parses.
2. Unknown fields in any hand-authored data file are rejected (`deny_unknown_fields`).

## Test Plan

### New/Modified Tests

1. `games/event_frontier/src/cards.rs` (or `tests/serialization.rs` stub) — parse-success + unknown-field/behavior-field rejection unit tests, the §5 boundary proof at the skeleton stage.

### Commands

1. `cargo test -p event_frontier`
2. `cargo build --workspace && bash scripts/boundary-check.sh`
3. The workspace build is the correct full-pipeline boundary here because registering a new crate must not break the workspace; the parse tests are the targeted boundary.

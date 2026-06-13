# CATSETVIS-005: Variant `description` Rust structs + parser hardening

**Status**: PENDING
**Priority**: MEDIUM
**Effort**: Medium
**Engine Changes**: Yes — `games/{flood_watch,frontier_control,event_frontier}/src/variants.rs`, `games/{flood_watch,frontier_control,event_frontier}/data/variants.toml`
**Deps**: None

## Problem

Variants carry only `id` + `display_name`, so the catalog cannot offer choice-support copy for the multi-variant games. This ticket adds an optional, inert one-line `description` (`Option<String>`) to the three multi-variant games' variant structs and TOML tables, with a hardened parser that accepts **only** the new `description` key, keeps rejecting unknown and behavior-looking keys, and validates length (≤120 chars) and non-behavior prose. This is the Rust source-of-truth that CATSETVIS-006 projects. Spec WB5 / §6 D8, D10; scope is multi-variant only (§4.3).

## Assumption Reassessment (2026-06-13)

1. No `games/*/src/variants.rs` defines a `description` field (verified — empty grep). The three multi-variant games build their catalogs from variant structs fed by TOML — `flood_watch` (`deluge`), `frontier_control` (`highlands`), `event_frontier` (`hard_winter`, `land_rush`) — confirmed at `games/<g>/src/variants.rs`. **Change rationale:** add an inert `Option<String>` per §6 D8; the 11 single-variant games are out of scope (§4.3) because their `list_games()` catalog tuples are constant-fed, not struct-fed.
2. Spec §6 D8 (authoring contract: optional, one line, recommended 55–95 / hard-max 120 chars, neutral original prose; no hidden info, rule procedure, conditionals, selectors, triggers, legality, scoring, strategy advice, trademarks, copied prose, casino terms, or raw IDs) and D10 (parser allows only the `description` key, keeps rejecting `when`/`if`/`then`/`selector`/`trigger`/`effect`/`action`/`legal`) govern.
3. Cross-artifact shared boundary: this Rust field is the source CATSETVIS-006 reads to project `description` through `variant_json`/`variants_json` (so 006 `Deps: 005`). Each game's variant parser has its own error type and TOML table shape; the `description` key is added to each existing variant table using the least-disruptive existing pattern.
4. FOUNDATIONS §5 (static data is typed content, not behavior): `description` is inert content; the parser must keep static data free of selectors/branches/triggers — the behavior-key rejection is the enforcement.
5. §11 fail-closed validation + no-leak / determinism: the variant parser is the enforcement surface — unknown keys and behavior-like keys/prose stay **rejected (a parse error, not a warning, non-overridable)**, and `description` is validated for length and non-behavior prose. `description` carries no hidden state (no §11 no-leak path) and is display-only metadata sibling to `display_name`, so it never enters canonical serialization/hash/trace (spec §14 A11) — proven by unchanged replay/fixture output.
6. Schema extension: this extends the per-game variant **static-data manifest entry** (the TOML variant table + its struct). Consumers are the catalog projection (CATSETVIS-006) and the in-game variant resolution; the extension is **additive-only** (a new optional key defaulting to `None`), so existing variant data and consumers are unaffected.

## Architecture Check

1. Adding `description` to the existing variant struct + TOML and extending each game's existing parser (allow one key; keep rejection) is least-disruptive versus a parallel metadata file — it keeps variant data single-sourced and reuses the established unknown-key-rejection machinery.
2. No backwards-compatibility shims; the field is an additive optional with a `None` default.
3. `engine-core` / `game-stdlib` untouched; `description` is `games/*`-local inert content — no mechanic noun, no promotion.

## Verification Layers

1. `description` field present on the three multi-variant structs + TOML → codebase grep-proof.
2. Parser fail-closed → unit tests (a behavior-like description is rejected; a >120-char description is rejected; an unknown key is still rejected; a valid description is accepted).
3. Additive-only / no replay-hash break → `replay-check` / `fixture-check` per game stay green (`description` is display-only, absent from canonical forms).
4. Inert content → manual review against §5 (the authored prose carries no selectors/triggers/legality).

## What to Change

### 1. `description: Option<String>` on the variant structs

Add the optional field near `display_name` in the three games' variant structs.

### 2. `description` key in the TOML variant tables

Add an optional `description` key inside each existing variant table in the three `data/variants.toml` files (authored per the §6 D8 contract).

### 3. Parser hardening

Extend each game's variant parser to accept only the new `description` key, keep rejecting unknown and behavior-looking keys, and validate `description` length (≤120 after trim) and non-behavior prose; update the unknown-key tests.

## Files to Touch

- `games/flood_watch/src/variants.rs` (modify)
- `games/frontier_control/src/variants.rs` (modify)
- `games/event_frontier/src/variants.rs` (modify)
- `games/flood_watch/data/variants.toml` (modify)
- `games/frontier_control/data/variants.toml` (modify)
- `games/event_frontier/data/variants.toml` (modify)

## Out of Scope

- WASM/TS projection of `description` (CATSETVIS-006).
- Single-variant games (§4.3) and their constant-fed catalog tuples.
- `catalog_theme` (CATSETVIS-003) and any variant behavior / selection change.

## Acceptance Criteria

### Tests That Must Pass

1. `cargo test --workspace` is green, including new parser tests: a behavior-like description, an over-length (>120) description, and an unknown key are each **rejected**; a valid description is accepted.
2. `cargo run -p replay-check -- --game flood_watch --all` (and for `frontier_control`, `event_frontier`) plus `cargo run -p fixture-check -- --game <g>` stay green — `description` does not alter canonical replay/hash/serialization.
3. `grep -rl 'description' games/{flood_watch,frontier_control,event_frontier}/src/variants.rs games/{flood_watch,frontier_control,event_frontier}/data/variants.toml | wc -l` returns `6`.

### Invariants

1. The variant parser rejects unknown keys and behavior-like keys/prose by default and allows only `description` (≤120 chars) — fail-closed, non-overridable.
2. `description` is display-only; replay, hash, fixtures, and traces are byte-identical to pre-change.

## Test Plan

### New/Modified Tests

1. `games/flood_watch/src/variants.rs`, `games/frontier_control/src/variants.rs`, `games/event_frontier/src/variants.rs` — unit tests asserting behavior-like / over-length / unknown-key rejection and valid-description acceptance.

### Commands

1. `cargo test -p flood_watch -p frontier_control -p event_frontier`
2. `cargo run -p replay-check -- --game flood_watch --all && cargo run -p fixture-check -- --game flood_watch` (repeat for `frontier_control`, `event_frontier`) — proves canonical forms are unchanged.
3. `cargo test --workspace` — full regression (the field is additive; no other crate should change behavior).

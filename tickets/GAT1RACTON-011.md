# GAT1RACTON-011: Minimal batched wasm-api gameplay surface

**Status**: PENDING
**Priority**: HIGH
**Effort**: Medium
**Engine Changes**: Yes — `crates/wasm-api` gains a thin batched gameplay surface (`new_match`, `get_view`, `get_action_tree`, `apply_action`, `run_bot_turn`, `get_effects`) over `race_to_n` resolved through a games registry.
**Deps**: GAT1RACTON-006, GAT1RACTON-007

## Problem

The browser needs a thin, batched Rust surface to play `race_to_n` (ARCHITECTURE
§10). The current `wasm-api` is a Gate 0 placeholder exposing only a version
string. This ticket adds the minimal gameplay surface — enough for human vs random
bot — resolving `race_to_n` through the games registry (ARCHITECTURE §2:
`wasm-api -> games registry + engine-core contracts`). No legality in the surface
beyond what Rust supplies; payloads are viewer-safe before crossing.

## Assumption Reassessment (2026-06-05)

1. `crates/wasm-api/src/lib.rs` currently exposes `placeholder_version()` and two
   `#[no_mangle] extern "C"` pointer/len functions (verified). No gameplay surface
   exists yet — additive. The game (006: view/serialization) and bot (007) are in
   place; the engine effect-cursor contract (003) backs `get_effects`.
2. The surface mirrors ARCHITECTURE §10's batched operations; Gate 1 implements
   the minimal subset (spec §2 in-scope): `new_match`, `get_view`,
   `get_action_tree`, `apply_action`, `run_bot_turn`, `get_effects`. The polished
   surface (`list_games`, `preview_action`, `get_replay`, `serialize_match`, store
   wiring) is Gate 3 (spec §2 out-of-scope).
3. Cross-crate boundary under audit: `wasm-api` depends on the games registry +
   `engine-core` contracts (ARCHITECTURE §2) and must NOT contain rule logic,
   hidden-state leakage, or chatty hot-loop crossings (ARCHITECTURE §3). It
   returns complete viewer-safe payloads (the public view from GAT1RACTON-006).
4. FOUNDATIONS §2 (Rust owns behavior; TS presents only) and §11 (browser
   payloads already safe for the viewer; coarse batched crossings) motivate this
   ticket. `apply_action` carries the freshness token (ARCHITECTURE §10
   `apply_action(match, actor, action_path, freshness_token)`); legality is
   decided in Rust (005), never in the surface or TS.
5. No-leak + boundary enforcement surface: `get_view`/`get_effects` are the
   browser-facing payload surfaces. Confirm they return only the viewer-safe
   public view / viewer-filtered effects (GAT1RACTON-006), never internal state.
   `race_to_n` is perfect-information so there is no hidden state to leak, but the
   surface still returns the public-view type by construction (recorded rationale,
   per ARCHITECTURE §6). `apply_action` rejects stale tokens via Rust validation.
6. Schema/contract: the surface serializes the public view / action tree / effects
   JSON shapes from GAT1RACTON-006/003 across the boundary (additive consumption).
   A games registry entry for `race_to_n` is added so `wasm-api` can resolve it
   (ARCHITECTURE §2) — structural wiring, registration is the consumer model.

## Architecture Check

1. A thin batched surface returning complete viewer-safe payloads is exactly
   ARCHITECTURE §10's contract and avoids JS/WASM crossings in rule hot loops.
   Resolving the game through a registry keeps `wasm-api` game-agnostic (it knows
   contracts + registry, not `race_to_n` internals). Alternative (per-game ad hoc
   exports, or returning internal state) violates §3/§6.
2. No backwards-compatibility shims — the placeholder version fn may remain or be
   removed; no alias layer is added.
3. `engine-core` untouched; `wasm-api` holds no rule logic (ARCHITECTURE §3).
   `game-stdlib` untouched.

## Verification Layers

1. Viewer-safe payloads -> no-leak visibility test / schema validation (`get_view`
   returns the public-view type; for this perfect-info game all fields are public).
2. Behavior authority preserved -> FOUNDATIONS alignment check (§2: the surface
   calls Rust legal-action/validate/apply; no legality logic in `wasm-api`).
3. Batched surface shape -> schema/serialization validation (the six operations
   match ARCHITECTURE §10 signatures; `apply_action` takes a freshness token).
4. Cross-crate registry resolution -> codebase grep-proof (`race_to_n` is
   registered and resolvable by `wasm-api`; build proof).

## What to Change

### 1. Games registry + wasm-api surface (`crates/wasm-api/src/lib.rs`)

Add a minimal games registry resolving `race_to_n`, and the six batched
operations (`new_match`, `get_view`, `get_action_tree`, `apply_action` [with
freshness token], `run_bot_turn`, `get_effects` [since cursor]) over the game's
`engine_core::Game` impl and the wired bot. Return viewer-safe payloads
(serialized public view / action tree / effects).

### 2. Build wiring

Ensure the wasm target builds (`wasm32-unknown-unknown` is already installed in
CI). Keep the binding mechanism consistent with the existing crate setup.

## Files to Touch

- `crates/wasm-api/src/lib.rs` (modify)
- `crates/wasm-api/Cargo.toml` (modify) — add `race_to_n` + `engine-core` + `ai-core` deps

## Out of Scope

- `apps/web` harness + UI smoke (GAT1RACTON-012).
- Gate 3 surface: `list_games`, `preview_action`, `get_replay`, `serialize_match`,
  stores, replay controls (spec §2 out-of-scope).
- Any TypeScript legality (forbidden, FOUNDATIONS §2/§7).

## Acceptance Criteria

### Tests That Must Pass

1. `cargo test -p wasm-api` — surface unit tests: `new_match` → `get_action_tree` → `apply_action` → `get_view`/`get_effects` drives a turn; `run_bot_turn` advances a bot seat.
2. `cargo build -p wasm-api --target wasm32-unknown-unknown` — wasm build succeeds.
3. `cargo clippy -p wasm-api --all-targets -- -D warnings` — clean.

### Invariants

1. `wasm-api` contains no rule/legality logic; it calls Rust behavior and returns viewer-safe payloads (FOUNDATIONS §2; ARCHITECTURE §3/§6).
2. `apply_action` rejects stale freshness tokens via Rust validation (FOUNDATIONS §11).

## Test Plan

### New/Modified Tests

1. `crates/wasm-api/src/lib.rs` `#[cfg(test)]` — full minimal turn loop over `race_to_n` through the surface (native test of the batched ops).
2. `crates/wasm-api/src/lib.rs` `#[cfg(test)]` — `apply_action` with a stale token returns a diagnostic, not a mutation.

### Commands

1. `cargo test -p wasm-api`
2. `cargo build -p wasm-api --target wasm32-unknown-unknown`
3. `grep -rniE 'fn .*(legal|valid)' crates/wasm-api/src` — expect no legality decisioning in the surface (boundary proof).

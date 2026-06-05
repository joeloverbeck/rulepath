# GAT1RACTON-006: race_to_n public-view projection + serialization

**Status**: PENDING
**Priority**: HIGH
**Effort**: Medium
**Engine Changes**: Yes — `games/race_to_n` gains viewer-safe public-view projection and serialization (internal snapshot, public view, replay JSON) with version fields and unknown-field rejection.
**Deps**: GAT1RACTON-005

## Problem

The observe/serialize path: project internal state into a viewer-safe **public
view** (ARCHITECTURE §6) and provide **serialization** for internal snapshot,
public view, and replay JSON with version fields, stable order, and unknown-field
rejection (TESTING §9; FOUNDATIONS §11). This is what crosses the `wasm-api`
boundary later and what replay/hash tests (GAT1RACTON-008) hash.

## Assumption Reassessment (2026-06-05)

1. After GAT1RACTON-005, `games/race_to_n` has state, rules, transitions, and
   effects; the `engine_core::Game` impl needs its `project_view` and
   serialization completed. The hash/serialization-boundary contract from
   GAT1RACTON-003 and `VisibilityScope`/`SchemaVersion` (`engine-core` existing)
   are in place.
2. `race_to_n` is perfect-information (spec §6 visibility row = `not-applicable`);
   the public view is wholly public. The serialization contract (versions,
   round-trips, unknown-field rejection) follows TESTING §9 and
   `docs/ENGINE-GAME-DATA-BOUNDARY.md`.
3. Cross-crate boundary under audit: the public view is the payload that crosses
   the `wasm-api` boundary (GAT1RACTON-011) — it MUST be a distinct type from
   internal state (ARCHITECTURE §6, "different types or impossible to confuse").
   Replay JSON is consumed by GAT1RACTON-008's replay tests.
4. FOUNDATIONS §11 (public/private views are viewer-safe; serialization order
   deterministic; unknown fields rejected) and §2 (view projection lives in Rust)
   motivate this ticket. Even though there is no hidden state, the firewall
   discipline is recorded: the public view is the only browser-facing payload and
   it contains no internal-only fields.
5. No-leak + determinism enforcement surface: serialization here is a §11
   enforcement surface. Confirm stable serialization order for hashing
   (ARCHITECTURE §9), version-field presence, and unknown-field rejection for
   hand-authored inputs. Because the game is perfect-information, the no-leak
   firewall is satisfied trivially (public view == full game facts); record this
   as the rationale rather than a leak test (the leak negative-test belongs to
   hidden-info games per TESTING §8 — n/a here).
6. Schema/contract extension: public-view and replay JSON are new serialized
   shapes for this game built on the `engine-core` serialization boundary
   (additive; no kernel change). Consumers: `wasm-api` (011), replay tests (008),
   golden traces (008).

## Architecture Check

1. A dedicated public-view type (separate from internal state) makes leakage
   structurally hard and matches ARCHITECTURE §6. Stable, versioned serialization
   makes hashes reproducible (ARCHITECTURE §8/§9). Alternative (serializing
   internal state directly to the browser) is the anti-pattern §6 forbids.
2. No backwards-compatibility shims.
3. `engine-core` untouched; projection/serialization for this game live in
   `games/race_to_n` (§3). `game-stdlib` untouched (§4).

## Verification Layers

1. Viewer-safety -> schema/serialization validation (public view is a distinct
   type; serialized public payload contains only public fields — for this
   perfect-info game, all fields are public).
2. Round-trip fidelity -> serialization test (snapshot, public view, replay JSON
   each round-trip; version field present; unknown field rejected — TESTING §9).
3. Hash stability -> deterministic replay-hash check (stable serialization order
   → stable view hash; consumed by GAT1RACTON-008).
4. Cross-crate: the public-view type is what `wasm-api` returns (GAT1RACTON-011);
   build/type proof that the boundary payload is the view type, not internal state.

## What to Change

### 1. Public-view projection (`src/visibility.rs`)

`project_view(state, viewer) -> PublicView` producing a viewer-safe, wholly
public view distinct from internal state. Complete the `project_view` part of the
`engine_core::Game` impl.

### 2. Serialization (`src/lib.rs` / `src/state.rs`)

Serde serialization for internal snapshot, public view, and replay record JSON,
with version fields (`RulesVersion`/`SchemaVersion`), stable field order, and
deny-unknown-fields for hand-authored inputs.

## Files to Touch

- `games/race_to_n/src/visibility.rs` (new)
- `games/race_to_n/src/lib.rs` (modify) — complete `project_view` + serialization
- `games/race_to_n/tests/serialization_tests.rs` (new)

## Out of Scope

- Replay/hash reproduction tests + golden traces (GAT1RACTON-008) — this ticket
  provides the serialized forms they hash.
- Bot, simulation, benchmarks, wasm, UI (007/009/010/011/012).

## Acceptance Criteria

### Tests That Must Pass

1. `cargo test -p race_to_n` — serialization round-trip tests (snapshot, public view, replay JSON), version-field presence, and unknown-field rejection pass.
2. The public view is a distinct type from internal state (compile-time / type assertion).
3. `cargo clippy -p race_to_n --all-targets -- -D warnings` — clean.

### Invariants

1. The only browser-facing payload type is the public view; internal state never crosses the boundary (ARCHITECTURE §6; FOUNDATIONS §11).
2. Serialization order is stable and version fields are present (ARCHITECTURE §8/§9; TESTING §9).

## Test Plan

### New/Modified Tests

1. `games/race_to_n/tests/serialization_tests.rs` — round-trip + version-field + unknown-field-rejection for snapshot/public-view/replay JSON.
2. `games/race_to_n/src/visibility.rs` `#[cfg(test)]` — `project_view` yields the public-view type; field set is the expected public set.

### Commands

1. `cargo test -p race_to_n`
2. `cargo test --workspace`
3. `grep -rni 'deny_unknown_fields' games/race_to_n/src` — expect presence on hand-authored deserialization (fail-closed proof).

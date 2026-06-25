# GAT18BLAPACSPA-011: fixtures, cross-cutting tests, and replay-check / fixture-check registration

**Status**: PENDING
**Priority**: MEDIUM
**Effort**: Large
**Engine Changes**: Yes (deterministic evidence + tooling) — `games/blackglass_pact/tests`, `games/blackglass_pact/data/fixtures`, `tools/replay-check`, `tools/fixture-check`
**Deps**: GAT18BLAPACSPA-004, GAT18BLAPACSPA-005, GAT18BLAPACSPA-006, GAT18BLAPACSPA-007, GAT18BLAPACSPA-008, GAT18BLAPACSPA-009

## Problem

Consolidate the deterministic evidence corpus and register the validating tools: the five required fixtures, the cross-cutting property/serialization/replay test suites, the standard short-path fixture, and the `tools/replay-check` + `tools/fixture-check` game arms (with the per-game allowed-JSON-keys constant and Cargo dependency `fixture-check` requires). This makes the golden traces authored across GAT18BLAPACSPA-004–009 validate under the standard pipeline (spec §4.4, §4.5, §7.2, candidate task `GAT18-BLAPAC-010`/`012`).

## Assumption Reassessment (2026-06-25)

1. Reassessment confirmed (spec Assumption 19) that **both** `tools/fixture-check` and `tools/rule-coverage` use a hard-coded `match game` in `resolve_game` (`tools/fixture-check/src/main.rs:283`); `fixture-check` carries a per-game `*_ALLOWED_JSON_KEYS` constant (`VOW_TIDE_ALLOWED_JSON_KEYS:195`) + a game Cargo dep. `replay-check` likewise dispatches explicitly. None auto-discovers.
2. Spec §4.4 fixes the fixture set (standard, blind-nil, bags-rollover, double-bag, target-tie) and the property/serialization/replay coverage; §7.2 fixes the test taxonomy.
3. Cross-artifact boundary under audit: the golden traces (authored in the module tickets) + fixtures are validated cross-cuttingly by `replay-check --all`; the `fixture-check` allowed-keys constant is the static-data boundary guard.
4. FOUNDATIONS §2 (determinism) / §11 motivate this ticket: replay reproduces terminal state/effects/hash byte-for-byte under fixed versions; fixtures hold typed data only (no behavior).

## Architecture Check

1. Registering against the existing explicit-`match` tool drivers (vs. forking a bespoke per-game driver) reuses the shared validator body and matches the codebase reality reassessment confirmed.
2. No shims; `fixture-check` reuses the shared validation with a game-local allowed-keys list.
3. `engine-core` untouched; no `game-stdlib` change; tool arms only.

## Verification Layers

1. Every named golden trace validates and replays byte-stable -> `cargo run -p replay-check -- --game blackglass_pact --all`.
2. Fixtures contain only typed/allowed keys -> `cargo run -p fixture-check -- --game blackglass_pact`.
3. Property/serialization/replay invariants hold -> `cargo test -p blackglass_pact --test property --test serialization --test replay`.

## What to Change

### 1. Fixtures + cross-cutting tests

Finalize `data/fixtures/blackglass_pact_standard.fixture.json` (+ the four authored in 004/007); complete `tests/property.rs`, `tests/serialization.rs`, `tests/replay.rs` to the §7.2 taxonomy (card conservation, comparator agreement, score equations, bag rollover, team partition, deterministic replay, stable byte order).

### 2. fixture-check arm

`tools/fixture-check/src/main.rs`: `blackglass_pact` match arm + `BLACKGLASS_PACT_ALLOWED_JSON_KEYS`; `tools/fixture-check/Cargo.toml`: add `blackglass_pact` path dep.

### 3. replay-check arm

`tools/replay-check/src/main.rs`: register `blackglass_pact` replay/export validators + viewer variants (public + four seat-private) where the tool dispatches explicitly.

## Files to Touch

- `games/blackglass_pact/tests/{property,serialization,replay}.rs` (modify)
- `games/blackglass_pact/data/fixtures/blackglass_pact_standard.fixture.json` (new)
- `tools/fixture-check/src/main.rs` (modify), `tools/fixture-check/Cargo.toml` (modify)
- `tools/replay-check/src/main.rs` (modify)

## Out of Scope

- `rule-coverage` arm + `RULE-COVERAGE.md` finalize (GAT18BLAPACSPA-013).
- Benchmarks (GAT18BLAPACSPA-012); WASM/web (GAT18BLAPACSPA-014+).

## Acceptance Criteria

### Tests That Must Pass

1. `cargo run -p fixture-check -- --game blackglass_pact`.
2. `cargo run -p replay-check -- --game blackglass_pact --all`.
3. `cargo test -p blackglass_pact --test property --test serialization --test replay`.

### Invariants

1. Every golden trace validates and replays byte-for-byte under fixed versions.
2. Fixtures carry only typed/allowed keys; no behavior-looking field passes the allowed-keys guard.

## Test Plan

### New/Modified Tests

1. `games/blackglass_pact/tests/property.rs` — full §7.2 property/invariant set.
2. `games/blackglass_pact/tests/replay.rs` — command-replay reproduces state/effects/hash.
3. `games/blackglass_pact/data/fixtures/blackglass_pact_standard.fixture.json` — standard short-path fixture.

### Commands

1. `cargo run -p fixture-check -- --game blackglass_pact && cargo run -p replay-check -- --game blackglass_pact --all`
2. `cargo test -p blackglass_pact`
3. The fixture/replay tools + crate tests are the correct boundary; rule-coverage finalize is GAT18BLAPACSPA-013.

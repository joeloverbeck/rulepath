# GAT16BRICIRTRI-012: Fixtures, native tool registrations, rule-coverage, and gate-1 CI

**Status**: PENDING
**Priority**: HIGH
**Effort**: Medium
**Engine Changes**: Yes (tooling + deterministic evidence) — `games/briar_circuit/data/fixtures/*`, `tools/{replay-check,fixture-check,rule-coverage}/src/main.rs`, `ci/games.json`, `.github/workflows/gate-1-game-smoke.yml`, finalize `RULE-COVERAGE.md`
**Deps**: 002, 010, 011

## Problem

Briar Circuit must be registered with the native verification tools and CI: typed fixtures with unknown-field rejection, `replay-check`/`fixture-check`/`rule-coverage` dispatch arms, the `BC-*` rule-ID prefix in the coverage validator, the `ci/games.json` row that drives the per-game CI matrix, and the gate-1 workflow wiring. This closes the `check-ci-games` red window opened when the crate directory landed (GAT16BRICIRTRI-004).

## Assumption Reassessment (2026-06-20)

1. `tools/{simulate,replay-check,fixture-check,rule-coverage}/src/main.rs` register games as per-game dispatch arms; `tools/rule-coverage/src/main.rs` gates rule IDs via an `is_rule_id`-style prefix check (no `BC-` prefix yet). `ci/games.json` is an array of `{id, sim_flags, e2e}` rows (no `briar_circuit`). The `simulate` arm already landed in GAT16BRICIRTRI-011.
2. `specs/gate-16-briar-circuit-trick-taking.md` §4.4 (Simulator/Replay-checker/Fixture-checker/Rule-coverage/CI-catalog rows), §10.5, and the §7.1 command suite fix the registrations; the fixtures are the four named in §4.1 (`standard`, `first_trick_exception`, `moon`, `threshold_tie`).
3. Cross-artifact boundary under audit: `ci/games.json` ↔ `games/` dir ↔ `apps/web/e2e/<id>.smoke.mjs` parity is enforced by `scripts/check-ci-games.mjs`; the `rule-coverage` validator reads `RULES.md`+`RULE-COVERAGE.md`+`BENCHMARKS.md`, so its fully-green state also depends on `BENCHMARKS.md` (GAT16BRICIRTRI-016).
4. FOUNDATIONS §11 fail-closed validation is under audit: `fixture-check` rejects unknown and behavior-looking fields by default; `rule-coverage` fails on undocumented or unproved `BC-*` rules. These are blockers, not warnings.

## Architecture Check

1. Co-locating all native tool arms + `ci/games.json` in one ticket gives a single reviewable "the game is now CI-discoverable" diff and closes the `check-ci-games` window deterministically.
2. No backwards-compatibility aliasing/shims — additive dispatch arms and one `ci/games.json` row.
3. `engine-core` untouched (§3); no `game-stdlib` change (§4); fixtures hold typed evidence only (§5) — no selectors/formulas/bot rules.

## Verification Layers

1. Fixtures validate; unknown/behavior-looking fields rejected; version anchors consistent -> `fixture-check --game briar_circuit`.
2. Internal traces, all view hashes, diagnostics, and viewer-scoped export stability validate -> `replay-check --game briar_circuit --all`.
3. Every `BC-*` rule is documented and proved; the `BC-` prefix is registered -> `rule-coverage --game briar_circuit` (partial-green until `BENCHMARKS.md` lands in 016).
4. `ci/games.json` ↔ dir ↔ e2e parity -> `node scripts/check-ci-games.mjs --emit`.

## What to Change

### 1. Fixtures (`games/briar_circuit/data/fixtures/*.fixture.json`)

The four typed fixtures: `briar_circuit_standard`, `briar_circuit_first_trick_exception`, `briar_circuit_moon`, `briar_circuit_threshold_tie`.

### 2. Tool dispatch arms

Register `briar_circuit` in `tools/replay-check`, `tools/fixture-check`, and `tools/rule-coverage` (including the `BC-*` prefix in the rule-ID validator).

### 3. CI catalog + workflow

Add the `ci/games.json` row (`{ "id": "briar_circuit", "sim_flags": "--seat-count 4", "e2e": "briar-circuit.smoke.mjs" }`) and confirm the gate-1 workflow matrix picks it up; finalize `RULE-COVERAGE.md` proof links.

## Files to Touch

- `games/briar_circuit/data/fixtures/{briar_circuit_standard,briar_circuit_first_trick_exception,briar_circuit_moon,briar_circuit_threshold_tie}.fixture.json` (new)
- `tools/replay-check/src/main.rs` (modify)
- `tools/fixture-check/src/main.rs` (modify)
- `tools/rule-coverage/src/main.rs` (modify)
- `ci/games.json` (modify)
- `.github/workflows/gate-1-game-smoke.yml` (modify — if matrix wiring needs the e2e reference)
- `games/briar_circuit/docs/RULE-COVERAGE.md` (modify; created by 002)

## Out of Scope

- WASM catalog/operation groups (GAT16BRICIRTRI-013) and the e2e smoke file itself (GAT16BRICIRTRI-015) — the `ci/games.json` `e2e` field references a file that lands in 015 (expected interim red e2e matrix step).
- Benchmarks and `BENCHMARKS.md` (GAT16BRICIRTRI-016) — `rule-coverage` stays partial-green until then.

## Acceptance Criteria

### Tests That Must Pass

1. `cargo run -p fixture-check -- --game briar_circuit` — all four fixtures validate; unknown fields rejected.
2. `cargo run -p replay-check -- --game briar_circuit --all` — internal + view-hash + export checks pass.
3. `node scripts/check-ci-games.mjs --emit` — dir/json/e2e parity (e2e file pending 015 is the only expected gap).

### Invariants

1. `fixture-check` and `rule-coverage` are deterministic, fail-closed blockers (§11).
2. Static fixtures hold typed evidence only — no behavior-looking fields (§5).

## Test Plan

### New/Modified Tests

1. `games/briar_circuit/data/fixtures/*.fixture.json` — four typed fixtures (validated by `fixture-check`).
2. `tools/{replay-check,fixture-check,rule-coverage}/src/main.rs` — `briar_circuit` dispatch arms.
3. `games/briar_circuit/docs/RULE-COVERAGE.md` — finalized proof mapping.

### Commands

1. `cargo run -p fixture-check -- --game briar_circuit && cargo run -p rule-coverage -- --game briar_circuit`
2. `cargo run -p replay-check -- --game briar_circuit --all`
3. The tool CLIs are the correct boundary; full `rule-coverage` green is reached once `BENCHMARKS.md` lands (GAT16BRICIRTRI-016), flagged as an expected partial-green window.

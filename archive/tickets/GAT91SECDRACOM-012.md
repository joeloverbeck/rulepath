# GAT91SECDRACOM-012: secret_draft native tool registration + RULE-COVERAGE.md

**Status**: ✅ COMPLETED
**Priority**: HIGH
**Effort**: Medium
**Engine Changes**: Yes — `tools/{simulate,replay-check,fixture-check,rule-coverage}/src/main.rs` (modify); `games/secret_draft/docs/RULE-COVERAGE.md` (new). No `engine-core` / `game-stdlib` change.
**Deps**: GAT91SECDRACOM-001, GAT91SECDRACOM-008, GAT91SECDRACOM-010

## Problem

The four native tools must dispatch `--game secret_draft` so simulation, replay-check, fixture-check, and rule-coverage can validate the game end-to-end; and `RULE-COVERAGE.md` must map every rules-doc obligation to a test/trace. `rule-coverage` consumes `RULES.md` + `RULE-COVERAGE.md`, so the doc co-lands with the tool registration (tool-validated-doc rule).

## Assumption Reassessment (2026-06-08)

1. `token_bazaar` is registered in exactly these four tools (verified by grep: `tools/{rule-coverage,simulate,fixture-check,replay-check}/src/main.rs`). `bench-report`, `seed-reducer`, and `trace-viewer` do NOT enumerate `token_bazaar` — confirming the spec's conditional ("register in bench-report/seed-reducer/trace-viewer only if dispatch tables need game IDs"). For `secret_draft` the same four are in scope; the other three are out of scope unless their dispatch tables require the ID (they currently do not).
2. The crate (GAT91SECDRACOM-002), bots (008, for `simulate` 1000-game runs), and golden traces (010, for `replay-check --all`) are inputs. `RULES.md` (GAT91SECDRACOM-001) supplies the obligation list `rule-coverage` maps. Spec §Deliverables (Tools row) + §"Acceptance evidence → Tools".
3. Cross-artifact boundary under audit: each tool's game-dispatch table (the match arm registering a game ID) and the `rule-coverage` RULES↔tests/traces mapping contract. Adding a new dispatch arm at each of the four sites is the registration; this is the structural consumer model (registration is the wiring), not a name-grep for callers.
4. §11 evidence coverage is the motivating invariant: restate before trusting spec — `rule-coverage` must map every rules-doc obligation to a test/trace; `simulate` must finish 1000 games with no illegal bot action or invariant failure; `replay-check --all` must pass; `fixture-check` must pass. Fail-closed: a missing mapping or an illegal action fails the tool.
5. Enum/consumer blast radius: the new value `secret_draft` needs a new arm at each of the four dispatch sites; under-registering any one silently drops that validation. All four are listed in Files to Touch.

## Architecture Check

1. Registering the game ID at each tool's existing dispatch table (mirroring token_bazaar's arms) is the cleanest minimal change; no tool logic is generalized.
2. No backwards-compatibility aliasing/shims.
3. `engine-core` stays noun-free; tool dispatch is presentation/CLI plumbing over game-local APIs. No `game-stdlib` helper.

## Verification Layers

1. Simulation legality -> `cargo run -p simulate -- --game secret_draft --games 1000` finishes with no illegal bot action / invariant failure.
2. Replay validation -> `cargo run -p replay-check -- --game secret_draft --all` passes against the 14 golden traces.
3. Fixture validation -> `cargo run -p fixture-check -- --game secret_draft` passes.
4. Rule coverage -> `cargo run -p rule-coverage -- --game secret_draft` passes and maps every obligation; `RULE-COVERAGE.md` present and doc-link clean.

## What to Change

### 1. Tool dispatch registration

Add a `secret_draft` arm to the game-dispatch table in `tools/simulate/src/main.rs`, `tools/replay-check/src/main.rs`, `tools/fixture-check/src/main.rs`, and `tools/rule-coverage/src/main.rs`, mirroring the existing `token_bazaar` arms.

### 2. `games/secret_draft/docs/RULE-COVERAGE.md`

Instantiate from `templates/GAME-RULE-COVERAGE.md`, mapping each `RULES.md` obligation (setup, commit/pending/reveal flow, conflict fallback, scoring components, terminal, tie-break ladder, no-leak guarantees) to the covering test(s)/trace(s) from GAT91SECDRACOM-009/010.

## Files to Touch

- `tools/simulate/src/main.rs` (modify)
- `tools/replay-check/src/main.rs` (modify)
- `tools/fixture-check/src/main.rs` (modify)
- `tools/rule-coverage/src/main.rs` (modify)
- `games/secret_draft/docs/RULE-COVERAGE.md` (new)

## Out of Scope

- `bench-report` / `seed-reducer` / `trace-viewer` registration — not required (their dispatch tables do not enumerate game IDs; token_bazaar precedent confirms).
- WASM registration (GAT91SECDRACOM-013) and CI workflow steps (GAT91SECDRACOM-011 gate-2, GAT91SECDRACOM-016 gate-1).

## Acceptance Criteria

### Tests That Must Pass

1. `cargo run -p simulate -- --game secret_draft --games 1000` finishes clean.
2. `cargo run -p replay-check -- --game secret_draft --all` passes.
3. `cargo run -p fixture-check -- --game secret_draft` and `cargo run -p rule-coverage -- --game secret_draft` pass; `node scripts/check-doc-links.mjs` passes.

### Invariants

1. Every rules-doc obligation maps to a test/trace (§11 evidence coverage); `rule-coverage` is fail-closed.
2. The game is registered at all four dispatch sites — no silently-dropped validation (§11).

## Test Plan

### New/Modified Tests

1. `games/secret_draft/docs/RULE-COVERAGE.md` — obligation→test/trace mapping (validated by `rule-coverage`).

### Commands

1. `cargo run -p rule-coverage -- --game secret_draft`
2. `cargo run -p simulate -- --game secret_draft --games 1000 && cargo run -p replay-check -- --game secret_draft --all && cargo run -p fixture-check -- --game secret_draft`
3. These four CLI runs are the correct end-to-end boundary for native tool registration; web/WASM validation is GAT91SECDRACOM-013/016.

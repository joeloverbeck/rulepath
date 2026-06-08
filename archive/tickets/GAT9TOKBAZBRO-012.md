# GAT9TOKBAZBRO-012: Native tool registration + RULE-COVERAGE.md

**Status**: PENDING
**Priority**: HIGH
**Effort**: Large
**Engine Changes**: Yes — `tools/{simulate,replay-check,fixture-check}/Cargo.toml` + `src/main.rs` (modify), `tools/rule-coverage/src/main.rs` (modify), `games/token_bazaar/docs/RULE-COVERAGE.md` (new)
**Deps**: GAT9TOKBAZBRO-001, GAT9TOKBAZBRO-010, GAT9TOKBAZBRO-011

## Problem

Token Bazaar must be drivable by the native game tools the way `high_card_duel`
is: `simulate` (mass random playouts), `replay-check --all` (trace hash
reproduction), `fixture-check` (fixture validity), and `rule-coverage` (every rule
section mapped to a test/fixture/note). This ticket adds the per-game arms +
Cargo path deps to each tool and authors `RULE-COVERAGE.md` co-located with the
rule-coverage registration that validates it.

## Assumption Reassessment (2026-06-08)

1. The game crate + traces + fixture + benchmark doc exist by now
   (GAT9TOKBAZBRO-002…011). Each tool currently enumerates games by a per-game
   const/arm: `tools/simulate/src/main.rs` has a `GAME_HIGH_CARD_DUEL` const +
   `run_high_card_duel_simulation` branch (verified, 23 refs); `tools/replay-check`
   (49 refs) and `tools/fixture-check` (12 refs) carry per-game arms + a Cargo
   path dep (verified at `tools/{simulate,replay-check,fixture-check}/Cargo.toml`);
   `tools/rule-coverage/src/main.rs` registers games by a `RegisteredGame` entry
   referencing `/docs/RULES.md`, `/docs/RULE-COVERAGE.md`, `/docs/BENCHMARKS.md`
   paths (verified, 6 refs) and has NO Cargo path dep (it reads docs by path).
2. The registration set is fixed by `specs/gate-9-token-bazaar-browser-proof.md` →
   "Tooling and workflow registration" (`simulate`, `replay-check`, `fixture-check`,
   `rule-coverage`; `bench-report`/`seed-reducer`/`trace-viewer` only "if
   enumerated"). Validation confirms `seed-reducer` and `trace-viewer` have 0
   `high_card_duel` refs and `bench-report` is generic → all three N/A (recorded
   in the capstone's registration summary). RULE-COVERAGE.md is authored from
   `templates/GAME-RULE-COVERAGE.md`.
3. Cross-artifact boundary under audit: the per-game tool-registration contract.
   `simulate`/`replay-check`/`fixture-check` need both a Cargo path dep
   (`token_bazaar = { path = "../../games/token_bazaar" }`) and a `main.rs` arm;
   `rule-coverage` needs only a `RegisteredGame` doc-path arm. RULE-COVERAGE.md is
   a tool-validated doc — it MUST co-land with its rule-coverage arm (this ticket),
   not trail, or the tool has nothing valid to check. It reads RULES.md (-001) and
   BENCHMARKS.md (-011), hence those deps.

## Architecture Check

1. Registering all four tools in one ticket keeps the native-tool surface
   consistent and reviewable as a single "the tools now know this game" diff;
   co-locating RULE-COVERAGE.md with its validator follows the tool-validated-docs
   rule (a trailing docs ticket would leave the rule-coverage arm checking a
   missing/invalid file).
2. No backwards-compatibility aliasing/shims — additive arms + new doc.
3. `engine-core` untouched; tools dispatch on opaque game ids. No `game-stdlib`
   helper introduced.

## Verification Layers

1. Mass simulation runs -> simulation/CLI run: `cargo run -p simulate -- --game token_bazaar --games 1000 --start-seed 1`.
2. Trace reproduction across the tool -> deterministic replay-hash: `cargo run -p replay-check -- --game token_bazaar --all`.
3. Fixture validity -> `cargo run -p fixture-check -- --game token_bazaar`.
4. Every rule section mapped -> `cargo run -p rule-coverage -- --game token_bazaar`
   (validates RULES.md / RULE-COVERAGE.md / BENCHMARKS.md presence + mapping).

## What to Change

### 1. `tools/simulate` (Cargo.toml + src/main.rs)

Add the path dep + a `GAME_TOKEN_BAZAAR` const, dispatch arm, usage-string entry,
and the random-playout summary path mirroring the `high_card_duel` branch.

### 2. `tools/replay-check` (Cargo.toml + src/main.rs)

Add the path dep + the per-game arm running `--all` over the twelve traces with
hash reproduction.

### 3. `tools/fixture-check` (Cargo.toml + src/main.rs)

Add the path dep + the per-game arm validating `token_bazaar_standard.fixture.json`.

### 4. `tools/rule-coverage/src/main.rs`

Add a `RegisteredGame` entry referencing the token_bazaar `RULES.md`,
`RULE-COVERAGE.md`, `BENCHMARKS.md` paths.

### 5. `games/token_bazaar/docs/RULE-COVERAGE.md`

Map every RULES.md rule section to a test/fixture/golden-trace or an explicit
not-applicable note.

## Files to Touch

- `tools/simulate/Cargo.toml` (modify)
- `tools/simulate/src/main.rs` (modify)
- `tools/replay-check/Cargo.toml` (modify)
- `tools/replay-check/src/main.rs` (modify)
- `tools/fixture-check/Cargo.toml` (modify)
- `tools/fixture-check/src/main.rs` (modify)
- `tools/rule-coverage/src/main.rs` (modify)
- `games/token_bazaar/docs/RULE-COVERAGE.md` (new)

## Out of Scope

- `bench-report` (generic), `seed-reducer`, `trace-viewer` — not applicable (0 refs / generic).
- WASM registration (GAT9TOKBAZBRO-013) and gate-1 CI steps (GAT9TOKBAZBRO-016).

## Acceptance Criteria

### Tests That Must Pass

1. `cargo run -p simulate -- --game token_bazaar --games 1000 --start-seed 1` — completes with a summary.
2. `cargo run -p replay-check -- --game token_bazaar --all` — all traces reproduce.
3. `cargo run -p fixture-check -- --game token_bazaar` and `cargo run -p rule-coverage -- --game token_bazaar` — pass.

### Invariants

1. Each tool dispatches on the opaque `token_bazaar` game id; no rule logic moves
   into a tool.
2. RULE-COVERAGE.md maps every rule section; rule-coverage passes (no unmapped rule).

## Test Plan

### New/Modified Tests

1. `games/token_bazaar/docs/RULE-COVERAGE.md` — rule→test mapping (validated by rule-coverage).
2. `None new Rust tests` — tool arms are exercised by the four CLI commands above.

### Commands

1. `cargo run -p simulate -- --game token_bazaar --games 1000 --start-seed 1`
2. `cargo run -p replay-check -- --game token_bazaar --all && cargo run -p fixture-check -- --game token_bazaar && cargo run -p rule-coverage -- --game token_bazaar`
3. The four game-id CLIs are the correct verification boundary — they are exactly
   the gate-1 lane steps GAT9TOKBAZBRO-016 wires into CI.

## Outcome

Completed: 2026-06-08

What changed:

- Registered `token_bazaar` in `simulate`, `replay-check`, `fixture-check`, and
  `rule-coverage`.
- Added the required tool Cargo path dependencies and `Cargo.lock` updates.
- Extended replay-check and fixture-check schema handling for Token Bazaar trace
  fields such as `setup_patch`, `expected_diagnostic_hashes`, and
  `expected_public_export_hashes`.
- Added `games/token_bazaar/docs/RULE-COVERAGE.md` mapping every `TB-*` rule ID
  to implementation evidence.

Deviations from original plan:

- `replay-check` gained a Token Bazaar-specific near-state reconstruction for
  the two explicit setup-patch traces authored in GAT9TOKBAZBRO-010.
- `fixture-check` accepts Token Bazaar traces without `expected_private_view_hashes`
  because the game is fully public and records private-view hashes as
  not-applicable rationale.

Verification results:

- `cargo run -p simulate -- --game token_bazaar --games 1000 --start-seed 1` passed.
- `cargo run -p replay-check -- --game token_bazaar --all` passed for all twelve traces.
- `cargo run -p fixture-check -- --game token_bazaar` passed.
- `cargo run -p rule-coverage -- --game token_bazaar` passed.
- `cargo fmt --all --check` passed.
- `cargo test -p simulate -p replay-check -p fixture-check -p rule-coverage` passed.
- `cargo test -p rule-coverage` passed after adding the direct `TB-*` prefix assertion.

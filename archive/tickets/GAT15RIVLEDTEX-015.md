# GAT15RIVLEDTEX-015: Tool registration — replay-check, fixture-check, rule-coverage, and RULE-COVERAGE.md

**Status**: COMPLETED
**Priority**: MEDIUM
**Effort**: Medium
**Engine Changes**: Yes — `tools/replay-check`, `tools/fixture-check`, `tools/rule-coverage` (`src/main.rs` + `Cargo.toml` each); `games/river_ledger/docs/RULE-COVERAGE.md`
**Deps**: GAT15RIVLEDTEX-010, GAT15RIVLEDTEX-014

## Problem

River Ledger must be discoverable by the per-game CLI validators: `replay-check` (golden traces), `fixture-check` (data fixtures), and `rule-coverage` (rule-to-evidence matrix, gated on the `RL-*` prefix validator). This also reconciles the planned `RULE-COVERAGE.md` (002) to the final implemented matrix.

## Assumption Reassessment (2026-06-14)

1. `tools/{replay-check,fixture-check,rule-coverage}/src/main.rs` each resolve games via a `resolve_game`/`RegisteredGame` path (verified); `tools/rule-coverage` `is_rule_id` (lines ~268–298) registers prefixes (R, TM, CF, DF, DL, HCD, TB, SD, CL, PT, MC, FW, EF) with no `RL-` yet — this ticket adds `RL-`.
2. `specs/...-base.md` §4.3 (tool rows), §10.7, and §7.1 fix the registration; `RULE-COVERAGE.md` was authored as planned rows in 002 and is reconciled to final here.
3. Cross-artifact boundary under audit: `replay-check` validates the golden traces authored across 004–013; `fixture-check` validates the `data/fixtures/` files from 004; `rule-coverage` reads `RULES.md` (001) + `RULE-COVERAGE.md` (here) + `BENCHMARKS.md` (014 — hence `Deps` 014, so coverage is fully green only after the benches land).
4. FOUNDATIONS §2 motivates this ticket: the tools observe and validate Rust-owned artifacts; they decide no behavior. Adding the `RL-` prefix and three match arms is additive registration (change rationale: new game discovery).

## Architecture Check

1. Registering each validator's match arm is the structural consumer-wiring for a new game, the standard per-tool registration shape.
2. No backwards-compatibility aliasing/shims — additive arms + one new prefix.
3. `engine-core` stays noun-free (§3); no `game-stdlib` change (§4); the `RL-` prefix is a tool-local validator entry.

## Verification Layers

1. `replay-check` validates every River Ledger golden trace -> `cargo run -p replay-check -- --game river_ledger --all`.
2. `fixture-check` validates the data fixtures -> `cargo run -p fixture-check -- --game river_ledger`.
3. `rule-coverage` accepts `RL-*` IDs and reports 100% required rows -> `cargo run -p rule-coverage -- --game river_ledger`.
4. Workspace builds with the three tool deps -> `cargo check --workspace`.

## What to Change

### 1. Tool registration

Add `river_ledger` dependency + dispatch/`resolve_game` arm to `tools/replay-check`, `tools/fixture-check`, and `tools/rule-coverage`; add the `RL-` prefix to `tools/rule-coverage` `is_rule_id` (and its test cases).

### 2. `games/river_ledger/docs/RULE-COVERAGE.md`

Reconcile the planned matrix (002) to the final implemented module/test/trace/bench mapping for every `RL-*` rule.

## Files to Touch

- `tools/replay-check/src/main.rs` (modify)
- `tools/replay-check/Cargo.toml` (modify)
- `tools/fixture-check/src/main.rs` (modify)
- `tools/fixture-check/Cargo.toml` (modify)
- `tools/rule-coverage/src/main.rs` (modify)
- `tools/rule-coverage/Cargo.toml` (modify)
- `games/river_ledger/docs/RULE-COVERAGE.md` (modify; created by 002)

## Out of Scope

- `tools/simulate` registration + benches (GAT15RIVLEDTEX-014).
- `ci/games.json` + CI workflow (GAT15RIVLEDTEX-018).
- WASM/web registration (016/017).

## Acceptance Criteria

### Tests That Must Pass

1. `cargo run -p replay-check -- --game river_ledger --all` — all golden traces validate.
2. `cargo run -p fixture-check -- --game river_ledger` and `cargo run -p rule-coverage -- --game river_ledger` — pass with 100% required coverage rows.
3. `cargo test -p rule-coverage` — `is_rule_id` accepts `RL-*`; `cargo check --workspace`.

### Invariants

1. Tools validate Rust-owned artifacts and decide no behavior (§2).
2. Coverage is 100% for required rows; `RL-*` IDs are recognized (§6).

## Test Plan

### New/Modified Tests

1. `tools/rule-coverage/src/main.rs` (modify) — `RL-` prefix + `is_rule_id` test cases.

### Commands

1. `cargo run -p replay-check -- --game river_ledger --all && cargo run -p fixture-check -- --game river_ledger`
2. `cargo run -p rule-coverage -- --game river_ledger && cargo test -p rule-coverage`
3. These per-tool runs are the correct boundary; CI wiring of these commands is in GAT15RIVLEDTEX-018.

## Outcome

Completed: 2026-06-14

Summary:

- Registered River Ledger in `replay-check`, `fixture-check`, and `rule-coverage`, including Cargo dependencies, resolver/help entries, and `Cargo.lock`.
- Added River Ledger-specific structural validation paths for the current placeholder golden-trace shape used by earlier Gate 15 tickets, covering game/rules identity, seat bounds, public command actions, expected evidence sections, duplicate IDs, and fixture/static-data checks.
- Added `RL-*` rule-ID support for multi-segment River Ledger rule IDs and unit coverage in `rule-coverage`.
- Reconciled `games/river_ledger/docs/RULE-COVERAGE.md` from planned wildcard rows to one exact row per stable `RL-*` ID in `RULES.md`, with later WASM/web proof rows explicitly marked `intentionally-deferred`.

Deviations:

- River Ledger traces are still placeholder fixtures from earlier tickets rather than the older hash-rich replay-check schema, so the new replay/fixture validators are structural for this game until a later trace migration changes the artifact format.
- CI game manifest/workflow wiring remains out of scope for GAT15RIVLEDTEX-018.
- Pre-existing unrelated `.claude/skills/spec-to-tickets/*` worktree edits were left untouched and unstaged.

Verification:

- `cargo run -p replay-check -- --game river_ledger --all`
- `cargo run -p fixture-check -- --game river_ledger`
- `cargo run -p rule-coverage -- --game river_ledger`
- `cargo test -p rule-coverage`
- `cargo check --workspace`
- `cargo fmt --all --check`
- `node scripts/check-doc-links.mjs`
- `bash scripts/boundary-check.sh`
- `git diff --check`

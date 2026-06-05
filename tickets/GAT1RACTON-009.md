# GAT1RACTON-009: Native random legal simulation via tools/simulate

**Status**: PENDING
**Priority**: HIGH
**Effort**: Medium
**Engine Changes**: Yes — `tools/simulate` is wired from a Gate 0 placeholder into a real native random-legal simulation runner for `race_to_n` with per-action invariant checks and failure seed/command output.
**Deps**: GAT1RACTON-006, GAT1RACTON-007

## Problem

Every official game must support native random legal simulation through a CLI
(TESTING §7; OFFICIAL-GAME-CONTRACT §1). The spec §5 exit criterion requires
100,000 native random games to complete without crash, with per-action invariant
checks and reproducible failing seeds. `tools/simulate` is currently a Gate 0
placeholder; this ticket makes it run `race_to_n`.

## Assumption Reassessment (2026-06-05)

1. `tools/simulate/src/main.rs` is a placeholder that prints `"simulate: no-op
   placeholder"` (verified). It is already a workspace member
   (`Cargo.toml` members includes `tools/simulate`). This ticket replaces the
   placeholder body with a real runner; it does not create the crate.
2. The runner drives `race_to_n` via its legal action generation (GAT1RACTON-005),
   public view (006), and the wired random legal bot (007), using the
   deterministic RNG contract (002) for seeding.
3. Cross-crate boundary under audit: `tools/simulate` depends on `games/race_to_n`
   + `engine-core` + `ai-core` (a tool may depend on games per ARCHITECTURE §3
   `tools/*` ownership). It MUST NOT contain game behavior not present in the game
   (ARCHITECTURE §3 — tools must not own game behavior); it only drives + checks.
4. FOUNDATIONS §11 (simulations check invariants after every action; failing
   seeds are reproducible) and §7 (random legal simulation) motivate this. The
   simulation validates every bot action through the normal engine path (TESTING
   §7) — no bypass.
5. Determinism + bot-legality enforcement surface: the per-action invariant checks
   are a §11 enforcement surface. Confirm: every action is validated through the
   normal path; invariants are checked after each action; on failure the runner
   emits the TESTING §7 failure block (game id, rules/data version, seed, options,
   turn index, actor, chosen path, command stream so far, hash at failure, failure
   reason, replay command). Reproducible from seed (deterministic). No leakage
   (perfect-info; output is public-safe).
6. Schema/contract: the CLI flags/output follow `tools/*` conventions and TESTING
   §7's failure-output shape. No existing schema changed; the failure block is a
   new tool output format.

## Architecture Check

1. Wiring the existing `tools/simulate` (rather than adding a new tool) matches
   ARCHITECTURE §1's tool inventory and keeps Gate 1 scope tight; the general
   harness generalization is Gate 2 (spec §2 out-of-scope). The runner stays a
   thin driver over Rust behavior — no game logic in the tool.
2. No backwards-compatibility shims — the placeholder body is replaced, not
   aliased.
3. `engine-core` untouched; no game behavior added to the tool (ARCHITECTURE §3).
   `game-stdlib` untouched.

## Verification Layers

1. 100k-without-crash -> simulation/CLI run (`cargo run -p simulate -- ...` over
   ≥100,000 seeded games completes; no panic — spec §5).
2. Per-action invariants -> property/invariant check (invariants asserted after
   each action during the run; reuses GAT1RACTON-005/008 invariant predicates).
3. Failure reproducibility -> simulation/CLI run (an injected failing seed prints
   the TESTING §7 failure block and is replayable from the printed command).
4. Bot legality through normal path -> bot legality check (the runner validates
   each bot action via normal validation; no bypass — TESTING §7).

## What to Change

### 1. Real simulation runner (`tools/simulate/src/main.rs` + `Cargo.toml`)

Replace the placeholder with a runner: parse flags (game = `race_to_n`, seed
range/count, turn/action caps); for each seed run a random-legal playout via the
bot; check invariants after each action; on failure print the TESTING §7 failure
block and continue/abort per flag. Add `race_to_n`/`engine-core`/`ai-core` deps to
`tools/simulate/Cargo.toml`.

### 2. Summary output

Report games run, terminal-outcome distribution, average length, and throughput
(throughput feeds GAT1RACTON-010's context; the benchmark proper is 010).

## Files to Touch

- `tools/simulate/src/main.rs` (modify)
- `tools/simulate/Cargo.toml` (modify) — add `race_to_n` + `engine-core` + `ai-core` deps

## Out of Scope

- Native criterion benchmarks + `BENCHMARKS.md` (GAT1RACTON-010).
- `seed-reducer` minimal-trace reduction + general harness (Gate 2).
- CI wiring of a quick-sim job (GAT1RACTON-013).

## Acceptance Criteria

### Tests That Must Pass

1. `cargo run -p simulate -- --game race_to_n --games 100000` (or equivalent flags) completes with no panic and prints a summary.
2. An injected/known failing condition prints the TESTING §7 failure block including a runnable replay command.
3. `cargo build --workspace && cargo clippy -p simulate --all-targets -- -D warnings` — clean.

### Invariants

1. Every bot action is validated through the normal engine path; the tool adds no game behavior (ARCHITECTURE §3; TESTING §7).
2. Failing seeds are reproducible from the printed command (deterministic; FOUNDATIONS §11).

## Test Plan

### New/Modified Tests

1. `tools/simulate/src/main.rs` `#[cfg(test)]` — a small-N run (e.g. 1,000 seeds) completes and invariants hold (keeps CI fast; the 100k run is a manual/nightly command).
2. `tools/simulate/src/main.rs` `#[cfg(test)]` — failure-path formatting produces a parseable replay command.

### Commands

1. `cargo test -p simulate`
2. `cargo run -p simulate -- --game race_to_n --games 100000`
3. A narrower CI command (`--games 1000`) is the correct quick-sim boundary; the full 100k run is the exit-criteria command (GAT1RACTON-015) run manually/nightly per TESTING §17.

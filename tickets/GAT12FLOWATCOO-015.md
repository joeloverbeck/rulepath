# GAT12FLOWATCOO-015: Native tools, RULE-COVERAGE.md, boundary-check, and gate-1 CI

**Status**: PENDING
**Priority**: HIGH
**Effort**: Large
**Engine Changes**: Yes — `tools/{simulate,replay-check,fixture-check,rule-coverage}/src/main.rs` (modify — register `flood_watch`); `games/flood_watch/docs/RULE-COVERAGE.md` (new); `scripts/boundary-check.sh` (modify — add `role`/`scenario`); `.github/workflows/gate-1-game-smoke.yml` (modify — add `flood_watch` steps)
**Deps**: GAT12FLOWATCOO-001, GAT12FLOWATCOO-011, GAT12FLOWATCOO-012

## Problem

`flood_watch` must register in the native tool suite (`simulate`, `replay-check`, `fixture-check`, `rule-coverage`) with its tool-validated `RULE-COVERAGE.md`, gain its gate-1 CI smoke steps, and extend `scripts/boundary-check.sh` to mechanically enforce the gate's headline kernel-noun risk (`role`, `scenario`) — the reassessment-approved Q1(a) change. This is where the gate's native pipeline goes green end-to-end.

## Assumption Reassessment (2026-06-11)

1. Each tool registers games via a per-tool match (verified): `simulate` (`run_masked_claims_simulation` dispatch), `replay-check`/`rule-coverage`/`fixture-check` (`resolve_game()` → `RegisteredGame { game_id, rules_path/trace_dir }`). `scripts/boundary-check.sh`'s `mechanic_pattern` is verified to be `board|card|deck|grid|suit|resource|capture|hand|pile|trick|pot|auction|betting|drafting` — it covers `card`/`deck` but **not** `role`/`scenario`/`event`; `engine-core` is verified clean of all three (zero matches, no `prevent`/`eventually` false positives). `.github/workflows/gate-1-game-smoke.yml` enumerates games explicitly by id (verified `masked_claims` steps).
2. The spec (§Deliverables "Tools" + "CI" + "Repository docs", §Exit-criteria "role powers stay game-local" + boundary-check rows, Work-breakdown item 12; reassessment finding Q1(a) applied to §Work-breakdown item 12 + §Acceptance-evidence) fixes: register `flood_watch` in the four native tools; author `RULE-COVERAGE.md`; extend `boundary-check.sh` to add `role` and `scenario` (evaluate `event` for substring false positives before including it); add gate-1 native smoke/replay/fixture/rule-coverage steps. `seed-reducer`/`trace-viewer` are NOT registered (verified: they only support `race_to_n`/`directional_flip`, no game-id enumeration).
3. Cross-artifact boundary under audit: `tools/rule-coverage` reads `RULES.md` (GAT12FLOWATCOO-001) + `RULE-COVERAGE.md` (this ticket) + `BENCHMARKS.md` (GAT12FLOWATCOO-012) — so a fully-green `rule-coverage --game flood_watch` depends on `BENCHMARKS.md` already existing; hence `Deps: GAT12FLOWATCOO-012`. `replay-check` consumes the golden traces (GAT12FLOWATCOO-011); `fixture-check` consumes the data + parsers (GAT12FLOWATCOO-003). `boundary-check.sh` is a shared CI script — the `role`/`scenario` additions only constrain `engine-core/src` and must stay green on the current clean tree.
4. FOUNDATIONS §3 (`engine-core` is a contract kernel; `role` and `scenario` are explicitly forbidden kernel nouns) and §6 (rule coverage is part of the done contract) motivate this ticket. Extending `boundary-check.sh` turns the §3 enforcement for this gate's two headline nouns from review-only into mechanical CI — the reassessment's M2/Q1(a) resolution.
5. Enforcement surface: this touches the §3 kernel-boundary enforcement surface (`boundary-check.sh`) and the §6/§11 evidence surface (`rule-coverage`). The boundary-check extension must add `role`/`scenario` without introducing false positives (verified clean); `event` is evaluated and only added if substring-safe (word boundary or confirmed absence) — otherwise deferred with a note, per Q1(a).

## Architecture Check

1. Co-landing the native tool arms + `RULE-COVERAGE.md` + gate-1 steps keeps the native CI lane green in one diff (no multi-PR red window); folding the `boundary-check.sh` extension here keeps all CI-script changes in the registration ticket the reassessment assigned them to.
2. No backwards-compatibility aliasing/shims; additive match arms + additive pattern terms + additive CI steps.
3. Extending `boundary-check.sh` strengthens the `engine-core` noun-free guarantee (§3) rather than weakening it; no mechanic noun is added to the kernel — the script now catches `role`/`scenario` if a future change tries.

## Verification Layers

1. Tool registration -> simulation/CLI run: `simulate`/`replay-check`/`fixture-check`/`rule-coverage --game flood_watch` all resolve and pass.
2. Rule coverage -> `rule-coverage --game flood_watch` maps every `RULES.md` obligation to tests/traces (needs `RULE-COVERAGE.md` + `BENCHMARKS.md`).
3. Kernel-boundary enforcement -> `bash scripts/boundary-check.sh` passes with `role`/`scenario` added to the pattern and `engine-core` still clean.
4. CI registration -> the gate-1 workflow runs the four `flood_watch` tool steps.

## What to Change

### 1. Native tool registration

Add `flood_watch` to `tools/simulate/src/main.rs` (game const + simulation dispatch + per-game playout), `tools/replay-check/src/main.rs` (`resolve_game` arm → trace dir), `tools/fixture-check/src/main.rs` (arm → manifest/variants/fixtures), `tools/rule-coverage/src/main.rs` (arm → `RULES.md` path).

### 2. `RULE-COVERAGE.md` + boundary-check extension

Instantiate `games/flood_watch/docs/RULE-COVERAGE.md` from `templates/GAME-RULE-COVERAGE.md`, mapping each `FW-*` rule obligation to its test/trace. Extend `scripts/boundary-check.sh` `mechanic_pattern` with `role` and `scenario`; evaluate `event` (add only if substring-safe, else note the deferral) per Q1(a).

### 3. gate-1 CI

Add `flood_watch` steps to `.github/workflows/gate-1-game-smoke.yml`: `simulate --games 1000`, `replay-check --all`, `fixture-check`, `rule-coverage`, mirroring the `masked_claims` steps.

## Files to Touch

- `tools/simulate/src/main.rs` (modify)
- `tools/replay-check/src/main.rs` (modify)
- `tools/fixture-check/src/main.rs` (modify)
- `tools/rule-coverage/src/main.rs` (modify)
- `games/flood_watch/docs/RULE-COVERAGE.md` (new)
- `scripts/boundary-check.sh` (modify — add `role`/`scenario`)
- `.github/workflows/gate-1-game-smoke.yml` (modify — `flood_watch` steps)

## Out of Scope

- `bench-report` registration + gate-2 (GAT12FLOWATCOO-012).
- `seed-reducer`/`trace-viewer` (no game-id enumeration — not registered).
- WASM and web smoke (GAT12FLOWATCOO-014/017/018).

## Acceptance Criteria

### Tests That Must Pass

1. `cargo run -p simulate -- --game flood_watch --games 1000`, `replay-check -- --game flood_watch --all`, `fixture-check -- --game flood_watch`, and `rule-coverage -- --game flood_watch` all pass.
2. `bash scripts/boundary-check.sh` passes with `role`/`scenario` in the pattern and `engine-core` clean.
3. The gate-1 workflow `flood_watch` steps are present and consistent with the `masked_claims` precedent.

### Invariants

1. `engine-core` gains no mechanic noun; the extended `boundary-check.sh` enforces `role`/`scenario` absence mechanically.
2. `rule-coverage` maps every `RULES.md` obligation to a test/trace (no unmapped rule).

## Test Plan

### New/Modified Tests

1. `games/flood_watch/docs/RULE-COVERAGE.md` — obligation→test/trace mapping (consumed by `rule-coverage`).
2. `scripts/boundary-check.sh` — extended `mechanic_pattern` (modify); re-run as the kernel-noun gate.

### Commands

1. `cargo run -p rule-coverage -- --game flood_watch && cargo run -p replay-check -- --game flood_watch --all`
2. `cargo run -p simulate -- --game flood_watch --games 1000 && cargo run -p fixture-check -- --game flood_watch && bash scripts/boundary-check.sh`
3. These native-tool runs are the correct boundary; the web/E2E lane is GAT12FLOWATCOO-017/018.

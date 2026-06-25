# GAT18BLAPACSPA-013: rule-coverage registration, RULE-COVERAGE finalize, GAME-EVIDENCE profile links

**Status**: PENDING
**Priority**: MEDIUM
**Effort**: Medium
**Engine Changes**: Yes (deterministic evidence + docs) — `tools/rule-coverage`, `games/blackglass_pact/docs/{RULE-COVERAGE,GAME-EVIDENCE}.md`
**Deps**: GAT18BLAPACSPA-011, GAT18BLAPACSPA-012

## Problem

Register the `tools/rule-coverage` arm for `blackglass_pact` and finalize `RULE-COVERAGE.md` so every `BP-*` rule maps to its Rust owner, named tests, properties, traces, replay/export evidence, UI smoke, bot evidence, and benchmark workload — with no orphan implementation and no uncovered rule. Populate the `GAME-EVIDENCE.md` profile rows for the five evidence profiles (spec §4.2, §7.7, §7.10, candidate task `GAT18-BLAPAC-010`).

## Assumption Reassessment (2026-06-25)

1. `tools/rule-coverage/src/main.rs` resolves games via a hard-coded `match game` (`:34`) wiring `games/<g>/docs/{RULES,RULE-COVERAGE,BENCHMARKS}.md`; no Cargo dep is needed (it reads doc files by path). The new arm points at `games/blackglass_pact/docs/`.
2. `rule-coverage` reads `BENCHMARKS.md` (GAT18BLAPACSPA-012) and `RULES.md` (GAT18BLAPACSPA-001) + `RULE-COVERAGE.md` (here); hence the `Deps` on 011 (evidence corpus) and 012 (benchmarks) — a green `rule-coverage --game blackglass_pact` requires all three docs present.
3. Cross-artifact boundary under audit: `RULE-COVERAGE.md` maps the `BP-*` IDs (authored in 001) to the owners/tests/traces shipped in 003–012; `GAME-EVIDENCE.md` (created in 002) gains the profile-link rows.
4. FOUNDATIONS §6 (evidence-heavy official games) motivates this ticket: every rule must have implementation + evidence and no implementation behavior may lack a rule/diagnostic owner.

## Architecture Check

1. Co-locating `RULE-COVERAGE.md` finalize with the rule-coverage registration (vs. a trailing docs ticket) ensures the tool has a valid doc to check the moment it is wired (no red window for this surface).
2. No shims; reuses the existing explicit-`match` rule-coverage driver.
3. `engine-core` untouched; no `game-stdlib` change; tool arm + docs.

## Verification Layers

1. Every `BP-*` ID maps to an owner + named evidence; no orphan/uncovered rule -> `cargo run -p rule-coverage -- --game blackglass_pact`.
2. `GAME-EVIDENCE.md` carries the five profile rows (`replay-command-v1`, `public-export-v1`, `seat-private-export-v1`, `setup-evidence-v1`, `domain-evidence-v1`) -> grep-proof + manual review against §7.7.
3. Coverage doc agrees with the shipped tests/traces -> manual cross-check.

## What to Change

### 1. rule-coverage arm

`tools/rule-coverage/src/main.rs`: add the `blackglass_pact` match arm (rules/coverage/benchmarks doc paths).

### 2. RULE-COVERAGE.md finalize

Map every `BP-*` ID to Rust owner + named tests/properties/traces/replay/UI/bot/benchmark evidence; no orphan implementation, no uncovered rule.

### 3. GAME-EVIDENCE.md profile links

Populate the five Evidence Fixture Contract profile rows + canonical byte authority, version, and artifact-link columns per §7.7 (final status flip stays with the capstone GAT18BLAPACSPA-019).

## Files to Touch

- `tools/rule-coverage/src/main.rs` (modify)
- `games/blackglass_pact/docs/RULE-COVERAGE.md` (modify)
- `games/blackglass_pact/docs/GAME-EVIDENCE.md` (modify; created by GAT18BLAPACSPA-002)

## Out of Scope

- WASM/web/CI registration (GAT18BLAPACSPA-014+).
- Final `GAME-EVIDENCE.md` status flip and forward-v1 receipt (GAT18BLAPACSPA-018/019).

## Acceptance Criteria

### Tests That Must Pass

1. `cargo run -p rule-coverage -- --game blackglass_pact` (full coverage matrix passes).
2. `grep -c "BP-" games/blackglass_pact/docs/RULE-COVERAGE.md` equals the Appendix A ID count.
3. `GAME-EVIDENCE.md` lists all five required profile rows.

### Invariants

1. No `BP-*` rule is uncovered and no implementation behavior lacks a rule/diagnostic owner.
2. The coverage matrix matches the shipped tests/traces/benchmarks.

## Test Plan

### New/Modified Tests

1. `games/blackglass_pact/docs/RULE-COVERAGE.md` — full BP-ID → owner/evidence matrix.
2. `games/blackglass_pact/docs/GAME-EVIDENCE.md` — five profile-link rows.
3. `tools/rule-coverage` arm exercised by the command below.

### Commands

1. `cargo run -p rule-coverage -- --game blackglass_pact`
2. `cargo run -p replay-check -- --game blackglass_pact --all`
3. The rule-coverage tool is the correct boundary; it depends on the 011 evidence corpus + 012 benchmarks already landing.

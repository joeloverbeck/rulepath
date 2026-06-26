# GAT19MELLEDFIV-018: RULE-COVERAGE.md and rule-coverage tool registration

**Status**: COMPLETED
**Priority**: MEDIUM
**Effort**: Medium
**Engine Changes**: Yes — deterministic evidence + docs (`games/meldfall_ledger/docs/RULE-COVERAGE.md`); `tools/rule-coverage` game arm
**Deps**: GAT19MELLEDFIV-016, GAT19MELLEDFIV-017

## Problem

Gate 19 needs the rule-coverage matrix (`RULE-COVERAGE.md` — every source rule, variant decision, exclusion, diagnostic, and trace mapped) and the `meldfall_ledger` registration in `tools/rule-coverage` so the matrix is mechanically validated. `tools/rule-coverage` reads `RULES.md`, `RULE-COVERAGE.md`, and `BENCHMARKS.md`, so this ticket depends on those docs (GAT19MELLEDFIV-001/017) being present for a fully-green run.

## Assumption Reassessment (2026-06-25)

1. `tools/rule-coverage/src/main.rs` accepts `--game <id>` against a per-game allowlist (confirmed during reassessment — invocation `rule-coverage -- --game meldfall_ledger`); `games/blackglass_pact/docs/RULE-COVERAGE.md` is the matrix exemplar.
2. Spec §4.2 (RULE-COVERAGE.md row), §6 (exit-criteria rule-coverage rows), and §7.2 (test taxonomy) define the matrix; the rules (`RULES.md`, GAT19MELLEDFIV-001), traces (004–013), and benchmarks (017) it maps already exist.
3. Cross-artifact: `tools/rule-coverage` is the validator reading `RULES.md` + `RULE-COVERAGE.md` + `BENCHMARKS.md` — a fully-green `rule-coverage --game meldfall_ledger` requires `BENCHMARKS.md` (GAT19MELLEDFIV-017) to have landed, hence the `Deps` on 017.
4. FOUNDATIONS §6 evidence-heavy: rule coverage is a required official-game deliverable; every source rule, variant decision, exclusion, diagnostic, and trace must map to a row.

## Architecture Check

1. Co-landing `RULE-COVERAGE.md` with the `rule-coverage` registration avoids a red validator window (the tool has the doc to check the moment it is registered).
2. No backwards-compatibility shims.
3. `engine-core` untouched; the tool arm is additive registration.

## Verification Layers

1. Coverage matrix maps every source rule/variant/exclusion/diagnostic/trace -> `cargo run -p rule-coverage -- --game meldfall_ledger` passes.
2. Matrix rows resolve to real rule IDs and trace filenames -> grep cross-check against `RULES.md` and `tests/golden_traces/`.
3. Doc links resolve -> `node scripts/check-doc-links.mjs`.

## What to Change

### 1. `docs/RULE-COVERAGE.md`

Matrix covering every source rule (by `RULES.md` ID), variant decision (single deck, no jokers, top-discard immediate-use, etc.), exclusion (Appendix A out-of-scope variants), diagnostic, and golden trace.

### 2. `tools/rule-coverage` arm

Register `meldfall_ledger` in `tools/rule-coverage/src/main.rs` (game allowlist + the doc paths it reads).

## Files to Touch

- `games/meldfall_ledger/docs/RULE-COVERAGE.md` (new)
- `tools/rule-coverage/src/main.rs` (modify)

## Out of Scope

- WASM/web registration (GAT19MELLEDFIV-019/020/021).
- `MECHANICS.md`/`UI.md`/`GAME-EVIDENCE.md` (GAT19MELLEDFIV-020/023).

## Acceptance Criteria

### Tests That Must Pass

1. `cargo run -p rule-coverage -- --game meldfall_ledger` passes with full coverage.
2. Every matrix row resolves to a real `RULES.md` rule ID and (where applicable) a real trace filename.
3. `node scripts/check-doc-links.mjs` passes; `cargo test --workspace` passes.

### Invariants

1. Every source rule, variant decision, exclusion, diagnostic, and trace is mapped (FOUNDATIONS §6).
2. No rule-coverage row references a non-existent rule ID or trace.

## Test Plan

### New/Modified Tests

1. `games/meldfall_ledger/docs/RULE-COVERAGE.md` — the coverage matrix (validated by `rule-coverage`, not a unit test).

### Commands

1. `cargo run -p rule-coverage -- --game meldfall_ledger`
2. `node scripts/check-doc-links.mjs`
3. `cargo test --workspace`

## Outcome

Completed: 2026-06-26

Added `games/meldfall_ledger/docs/RULE-COVERAGE.md` with a Gate 19 matrix that maps all stable `ML-*` rule IDs from `RULES.md` to implementation owners, tests, traces, fixture/tool evidence, docs, and benchmark coverage. The matrix also records the golden trace inventory, diagnostic coverage, variant decisions, and exclusions.

Registered `meldfall_ledger` in `tools/rule-coverage/src/main.rs` with its `RULES.md`, `RULE-COVERAGE.md`, and `BENCHMARKS.md` paths, and extended the rule-id parser to accept the `ML` prefix.

The coverage status is intentionally exact: `ML-MATCH-006` is marked `intentionally-deferred` because full nonterminal multi-round transition wiring is completed in later gate/tooling closeout, and `ML-UI-001` through `ML-UI-003` are marked `intentionally-deferred` for the later WASM/web tickets that own browser surfaces.

Verification passed:

1. `cargo run -p rule-coverage -- --game meldfall_ledger`
2. `cargo fmt --all --check`
3. `node scripts/check-doc-links.mjs`
4. `cargo test --workspace`
5. Two rule-id set comparisons between `RULES.md` and `RULE-COVERAGE.md` produced no output, confirming no missing or extra `ML-*` rule IDs.
6. The trace filename set comparison between `RULE-COVERAGE.md` and `games/meldfall_ledger/tests/golden_traces/` produced no output, confirming every referenced trace file exists.

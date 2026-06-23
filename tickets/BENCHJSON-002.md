# BENCHJSON-002: Catch benchmark JSON schema drift on the PR lane (schema-only validation)

**Status**: PENDING
**Priority**: MEDIUM
**Effort**: Small
**Engine Changes**: Yes — `tools/bench-report` (new `--schema-only` mode), `.github/workflows/gate-2-benchmarks.yml` (PR `bench-smoke` job), and a clarifying amendment to `docs/adr/0002-ci-benchmark-gating-lanes.md` (no `engine-core`/`game-stdlib`/rules changes)
**Deps**: BENCHJSON-001 (the two non-conforming games must be fixed first, or the new PR check fails immediately on them)

## Problem

BENCHJSON-001 fixed two games whose benchmark JSON drifted from the `bench-report` schema. The *reason the drift survived to `main`* is structural: per ADR 0002, the pull-request lane runs a non-gating bench **smoke** and "MUST NOT invoke `bench-report`" (shared runners cannot validly gate throughput). `bench-report` is the only artifact that validates the benchmark JSON **shape**, and it only runs on the post-merge `main`/scheduled gate. So a game can merge with a malformed/incomplete benchmark report and only break CI after merge — the failure mode this whole series addresses.

Schema conformance (required metadata fields present, `operations` array well-formed) is **environment-independent** — unlike throughput, it does not depend on runner speed or noise. It can therefore be validated on the PR lane without reintroducing the throughput-gating problem ADR 0002 exists to prevent. This ticket adds that PR-time guard so future drift fails the contributing PR, not `main`.

## Assumption Reassessment (2026-06-23)

1. **ADR 0002 currently forbids `bench-report` on the PR lane — verified.** `docs/adr/0002-ci-benchmark-gating-lanes.md` Decision: "The pull-request lane MUST run a non-gating benchmark **smoke** only … and MUST NOT invoke `bench-report`. Shared PR runners are not a valid throughput-gating environment." The rationale is explicitly **throughput** noise on shared runners, not schema validation. This ticket therefore touches accepted-ADR doctrine and **must amend ADR 0002 first** (FOUNDATIONS §13 ADR trigger) to carve out schema-only (non-threshold) validation as permitted on PR. This is a clarifying carve-out that preserves the ADR's intent (no throughput gating on shared runners), not a reversal — surface it for sign-off before implementing.
2. **`bench-report` CLI admits a clean schema-only mode.** `tools/bench-report/src/main.rs` `Config::parse` (lines 49–88) parses `--input`/`--thresholds`/`--game`; `main` (lines 3–41) does `Report::parse` → `ThresholdSet::parse` → `validate_report`. A `--schema-only` flag can run `Report::parse` plus the report-side metadata-presence checks and the non-empty `operations` guard, then exit 0 **without** requiring `--thresholds`/`--game` and **without** `validate_report`'s threshold comparison. The metadata-non-empty checks currently live inside `validate_report`; factor the report-only subset (the `build_profile/command/os/rust_version/hardware_environment_notes` non-empty loop) so schema-only can reuse it.
3. **PR smoke output is sufficient for schema validation.** The `bench-smoke` job runs `cargo bench -p <game> -- legal_actions` (a filtered subset) per game; each custom harness still emits its full top-level JSON object (metadata fields are independent of which operations ran) with a non-empty `operations` array for the filtered op(s). Schema-only validates shape, not which operations are present, so the existing filtered smoke output is a valid input — no need to add slow unfiltered runs to the PR lane.
4. **Cross-artifact boundary under audit:** the benchmark-report JSON contract (producer = each `[[bench]]` harness; consumer/schema-of-record = `tools/bench-report`). This ticket adds an earlier, environment-independent enforcement point for the *existing* contract; it does not change the contract or any threshold.
5. **Adjacent contradiction classification:** factoring all 15 emitters onto a shared bench-support helper (so schema can't drift per-game in the first place) would be a stronger structural fix but is a larger refactor — explicitly future cleanup as its own ticket, not pulled into this one.

## Architecture Check

1. **Validate shape where it's cheap and deterministic; gate throughput where it's valid.** Adding a schema-only check to the PR lane catches the BENCHJSON-001 class of drift at contribution time without measuring throughput on noisy shared runners — it honors ADR 0002's purpose while closing the blind spot. The alternative (leave PR blind, rely on the `main` gate) is the status quo that produced days of red `main`.
2. **No backwards-compatibility shim.** `--schema-only` is a new, explicit mode; default `bench-report` behavior (full threshold gate) is unchanged. The metadata-presence logic is factored, not duplicated.
3. **`engine-core` untouched**; changes are in a dev tool, a CI workflow, and an ADR. No product behavior, no mechanic nouns, no `game-stdlib` change.

## Verification Layers

1. **`--schema-only` accepts a conforming report and rejects a malformed one** -> Rust unit/integration test in `tools/bench-report` (a fixture missing `build_profile` → non-zero; a complete fixture → zero). This is the **negative test for the new check** — prove it goes red on induced drift, not just green on good input.
2. **`--schema-only` requires no `--thresholds`** -> codebase test: invoking with only `--input <conforming>` exits 0.
3. **PR smoke output validates** -> CI/full-pipeline: the updated `bench-smoke` job pipes each game's smoke bench through `bench-report --schema-only` and exits 0 for all 15 games (post-BENCHJSON-001).
4. **ADR doctrine remains coherent** -> FOUNDATIONS alignment check: the ADR 0002 amendment records that schema-only (non-threshold) validation is permitted on PR because it is environment-independent, and that throughput thresholds remain `main`/scheduled-only.

## What to Change

### 1. ADR 0002 clarifying amendment (`docs/adr/0002-ci-benchmark-gating-lanes.md`) — do this first

Add a short carve-out to the Decision (and a Consequences note): the PR lane MAY invoke `bench-report --schema-only` to validate benchmark-report **shape** (required metadata fields present and non-empty, `operations` well-formed), because schema conformance is environment-independent; it MUST NOT compare values against thresholds on the PR lane. Throughput gating stays `push`/`schedule`/`workflow_dispatch`-only. Keep the amendment minimal and preserve the existing lane wording.

### 2. `--schema-only` mode in `bench-report` (`tools/bench-report/src/main.rs`)

- Add a `schema_only: bool` to `Config` and parse a `--schema-only` flag (`Config::parse`, lines 59–72).
- When set: `--thresholds`/`--game` are optional (relax the `(None, None)` error at lines 77–79 for this mode).
- In `main`: when `schema_only`, run `Report::parse` + the report-side metadata-presence/non-empty-`operations` checks (factor that subset out of `validate_report`), print e.g. `bench-report: schema OK for <game_id> (N operations)`, and exit 0; skip `ThresholdSet::parse` and threshold comparison.
- Keep default (non-schema-only) behavior byte-for-byte unchanged.

### 3. PR `bench-smoke` job (`.github/workflows/gate-2-benchmarks.yml`)

- For each game's smoke step, capture the bench output and validate its schema, e.g.:
  `cargo bench -p <game> -- <filter> | tee /tmp/<game>-smoke.txt` then
  `cargo run -p bench-report -- --schema-only --input /tmp/<game>-smoke.txt`.
- Apply to all 15 games (preserve each game's existing filter: `legal_actions`, `legal_tree` for `event_frontier`). The `bench-gate` (non-PR) job is unchanged.

## Files to Touch

- `docs/adr/0002-ci-benchmark-gating-lanes.md` (modify)
- `tools/bench-report/src/main.rs` (modify)
- `.github/workflows/gate-2-benchmarks.yml` (modify)
- `tools/bench-report/` test module or `tests/` fixture(s) (new/modify)

## Out of Scope

- Any threshold value, lane definition beyond the schema-only carve-out, or budget change.
- The actual masked_claims/frontier_control emitter fixes (BENCHJSON-001).
- Refactoring the 15 per-game emitters onto a shared bench-support helper (separate future ticket).
- Adding throughput gating to the PR lane (explicitly forbidden by ADR 0002, and unchanged here).

## Acceptance Criteria

### Tests That Must Pass

1. `cargo test -p bench-report` → new schema-only tests pass, including a negative case (a fixture missing a required metadata field exits non-zero) and a positive case (a complete fixture with no `--thresholds` exits zero).
2. `cargo run -p bench-report -- --schema-only --input <conforming-report>` → exit 0; `… --schema-only --input <report-missing-build_profile>` → non-zero with a clear message.
3. `cargo fmt --all --check && cargo clippy --workspace --all-targets -- -D warnings && cargo build --workspace` → clean.
4. The PR `bench-smoke` job (act/locally or on a draft PR) runs each game's smoke bench through `--schema-only` and passes for all 15 games (with BENCHJSON-001 merged).

### Invariants

1. Schema-only validation never compares a measured value to a threshold (no throughput gating on the PR lane); it only asserts report shape.
2. Default `bench-report` behavior (full threshold gate used by the `bench-gate` job) is unchanged.
3. ADR 0002 and the workflow remain consistent: PR lane = smoke + schema-only; `main`/scheduled = full threshold gate.

## Test Plan

### New/Modified Tests

1. `tools/bench-report/src/main.rs` (or `tools/bench-report/tests/`) — schema-only unit/integration tests: a conforming fixture passes; fixtures missing `build_profile` / `operations` / a per-op `current_value` each fail (negative test proving the guard is not vacuous).

### Commands

1. `cargo test -p bench-report`
2. `cargo run -p bench-report -- --schema-only --input games/.../<a captured conforming report>.txt`
3. The unit/integration test in `bench-report` is the correct primary boundary because it can assert both accept and reject paths deterministically without running benchmarks; the workflow edit is verified end-to-end on a draft PR.

# GAT16BRICIRTRI-018: Public-release closeout and Done-flip capstone

**Status**: COMPLETED
**Priority**: MEDIUM
**Effort**: Medium
**Engine Changes**: None (docs/status — `specs/README.md`, `specs/gate-16-briar-circuit-trick-taking.md`, closeout evidence)
**Deps**: 015, 016, 017

## Problem

Briar Circuit is `Done` only after every §6/§7 receipt exists. This capstone exercises the full exit-criteria / acceptance-evidence suite the prior tickets composed, records the closeout evidence (command log, trace inventory, benchmark receipt, pairwise no-leak result, IP review, atlas no-debt confirmation), and flips the spec Status and the `specs/README.md` index row to `Done`. It introduces no new production logic.

## Assumption Reassessment (2026-06-20)

1. All implementation, tooling, WASM, web, e2e, benchmark, and doc tickets (004–017) have landed; `specs/README.md` Gate 16 row currently reads `Planned` / `gate-16-briar-circuit-trick-taking.md` (set when this spec landed), and `docs/MECHANIC-ATLAS.md` §10A debt is `_None_` (confirmed unchanged by GAT16BRICIRTRI-003).
2. `specs/gate-16-briar-circuit-trick-taking.md` §6 (exit criteria), §7.1 (command suite), §10.1 (status-flip rule), §10.6 (closeout evidence), and `docs/archival-workflow.md` fix the closeout requirements; the spec must not be marked `Done` when the browser merely appears playable.
3. Cross-artifact boundary under audit: this ticket reconciles status across `specs/README.md` and the spec's own Status only — it exercises (does not modify) the upstream tickets' surfaces. The atlas debt register stays `_None_`.
4. FOUNDATIONS §6 (evidence-heavy official games) + §11 acceptance invariants are under audit: the Done-flip is gated on the full command suite passing (rule/property/trace/replay/serialization/visibility/bot/tool/benchmark/WASM/UI), pairwise no-leak holding, and Rust producing the final outcome — not on browser playability alone.

## Architecture Check

1. A single verification + status-flip capstone (over scattering the flip across tickets) keeps the `Done` decision gated on the whole acceptance suite passing at once, matching the OGC closeout pattern.
2. No backwards-compatibility aliasing/shims — status/docs reconciliation only.
3. `engine-core`/games behavior untouched (§3); the capstone verifies, it does not implement.

## Verification Layers

1. Full §7.1 command suite passes -> run each command and record pass/fail in the closeout note.
2. Pairwise no-leak holds across native/WASM/DOM/storage; replay/hash deterministic -> `cargo test -p briar_circuit --test visibility`, `replay-check --game briar_circuit --all`, `briar-circuit.smoke.mjs`.
3. Atlas debt remains `_None_`; no kernel noun; no `game-stdlib` promotion -> `bash scripts/boundary-check.sh` + atlas grep.
4. Status reconciled -> `specs/README.md` Gate 16 row and spec Status both read `Done`.

## What to Change

### 1. Exit-evidence run and closeout note

Run the §7.1 command suite, record exact commands/versions/outcomes, the trace inventory, the benchmark environment/floors, the pairwise no-leak result, the source/IP review, and the atlas no-debt confirmation per §10.6.

### 2. Status flip

`specs/gate-16-briar-circuit-trick-taking.md` Status → `Done` (with completion date/evidence summary), and the `specs/README.md` Gate 16 index row → `Done`; follow `docs/archival-workflow.md` for any later archive move.

## Files to Touch

- `specs/gate-16-briar-circuit-trick-taking.md` (modify — Status → Done)
- `specs/README.md` (modify — Gate 16 row → Done)

## Out of Scope

- Any new production logic, test, doc content, or behavior — this is a verification + status capstone.
- Marking `Done` before the full §6/§7 suite passes.
- Beginning Gate 17 (Oh Hell) trick-taking implementation (successor rule, spec §11.3).

## Acceptance Criteria

### Tests That Must Pass

1. The full `specs/gate-16-briar-circuit-trick-taking.md` §7.1 command suite — all green (cargo fmt/clippy/test, fixture/replay/rule-coverage, simulate, bench, boundary, doc/catalog/player-rules/outcome/presentation checks, web build/smoke).
2. `cargo run -p replay-check -- --game briar_circuit --all` and `cargo test -p briar_circuit --test visibility` — replay determinism + pairwise no-leak.
3. `bash scripts/boundary-check.sh && grep -n 'Current debt: _None_' docs/MECHANIC-ATLAS.md` — kernel noun-free; no promotion debt created.

### Invariants

1. `Done` is gated on the full evidence suite, not browser playability (§6).
2. No kernel noun, no `game-stdlib` promotion, atlas debt `_None_` (§3/§4/§11).

## Test Plan

### New/Modified Tests

1. `None — docs-reconciliation + verification capstone; it reconciles the named docs/status surfaces and exercises the prior tickets' acceptance suite, adding no test file.`

### Commands

1. `cargo fmt --all -- --check && cargo clippy --workspace --all-targets --all-features -- -D warnings && cargo test --workspace`
2. `cargo run -p fixture-check -- --game briar_circuit && cargo run -p rule-coverage -- --game briar_circuit && cargo run -p replay-check -- --game briar_circuit --all && cargo run -p simulate -- --game briar_circuit --seat-count 4 --games 1000 --start-seed 1600 --action-cap 4096`
3. `npm --prefix apps/web run build && npm --prefix apps/web run smoke:wasm && npm --prefix apps/web run smoke:ui && npm --prefix apps/web run smoke:e2e && node scripts/check-catalog-docs.mjs && node scripts/check-outcome-explanations.mjs`

## Outcome

Completed: 2026-06-21

Changed:

- Flipped the Gate 16 Briar Circuit spec status to `Done` and recorded a compact
  closeout ledger covering ticket archive inventory, trace/no-leak evidence,
  benchmark receipt, source/IP posture, atlas debt, docs/catalog checks, and
  web smoke proof.
- Updated the public scaling tracker row in `specs/README.md` to `Done` and
  admitted Gate 17 as the next not-started row.
- Reconciled the Briar Circuit public-release checklist so the capstone closeout
  is no longer listed as a blocking release issue.
- Fixed clippy warnings in the benchmark source surfaced by the full capstone
  gate by avoiding unnecessary allocation and a redundant conversion.

Verification:

- `cargo fmt --all --check`
- `cargo clippy --workspace --all-targets --all-features -- -D warnings`
- `cargo check --workspace`
- `cargo test -p briar_circuit --test rules`
- `cargo test -p briar_circuit --test property`
- `cargo test -p briar_circuit --test replay`
- `cargo test -p briar_circuit --test serialization`
- `cargo test -p briar_circuit --test visibility`
- `cargo test -p briar_circuit --test bots`
- `cargo test -p briar_circuit`
- `cargo test -p wasm-api`
- `cargo test --workspace`
- `cargo run -p fixture-check -- --game briar_circuit`
- `cargo run -p rule-coverage -- --game briar_circuit`
- `cargo run -p replay-check -- --game briar_circuit`
- `cargo run -p replay-check -- --game briar_circuit --all`
- `cargo run -p simulate -- --game briar_circuit --seat-count 4 --games 1000 --start-seed 1600 --action-cap 4096`
- `cargo bench -p briar_circuit`
- `bash scripts/boundary-check.sh`
- `rg -n 'Current debt: _None_' docs/MECHANIC-ATLAS.md`
- `node scripts/check-doc-links.mjs`
- `node scripts/check-player-rules.mjs`
- `node scripts/check-catalog-docs.mjs`
- `node scripts/check-ci-games.mjs`
- `node scripts/check-outcome-explanations.mjs`
- `node scripts/check-presentation-copy.mjs`
- `npm --prefix apps/web run build`
- `npm --prefix apps/web run smoke:wasm`
- `npm --prefix apps/web run smoke:ui`
- `npm --prefix apps/web run smoke:e2e`

Deviations:

- The ticket's test-plan example used `cargo fmt --all -- --check`; the live
  repository and AGENTS guidance use `cargo fmt --all --check`, which was run.
- The capstone touched `games/briar_circuit/benches/briar_circuit.rs` only to
  satisfy clippy in the benchmark gate added by GAT16BRICIRTRI-016. No game
  legality, replay, visibility, WASM, or web behavior changed.
- The simulator command passed with `total_actions=1000` and
  `average_length=1.00`, which is recorded as the current setup-level simulator
  proof shape for Briar Circuit rather than a full-match simulation claim.

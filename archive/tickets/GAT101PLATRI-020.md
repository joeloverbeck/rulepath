# GAT101PLATRI-020: Capstone — mechanic atlas first-use rows, status reconciliation, and exit evidence

**Status**: COMPLETE
**Priority**: HIGH
**Effort**: Medium
**Engine Changes**: Yes — modifies `docs/MECHANIC-ATLAS.md` (trick first-use rows), `specs/README.md`, `progress.md`, `README.md`. Verification-only exercise of the whole gate. No production logic.
**Deps**: GAT101PLATRI-015, GAT101PLATRI-018, GAT101PLATRI-019

## Problem

The gate needs a capstone: record the trick-specific first-use atlas rows, run the full exit-criteria / acceptance-evidence set end-to-end, and reconcile status — flip Gate 10.1 to `Done` in `specs/README.md` and, because this closes the remaining trick/follow-suit half, flip the parent Gate 10 from `In progress` to `Done`; update `progress.md` and the root `README.md` catalog. This is the only ticket that flips completion status.

## Assumption Reassessment (2026-06-09)

1. `specs/README.md` carries Gate 10 (`In progress`) at the index and a Gate 10.1 row (`Planned`); `progress.md` records the Gate 10 trick half as deferred to `plain_tricks`; `docs/MECHANIC-ATLAS.md` §10B holds the third-use decision (recorded in GAT101PLATRI-002); root `README.md` has the implemented-game catalog. All upstream implementation/docs tickets (001–019) land before this.
2. Spec §6 (exit criteria), §7 (acceptance evidence), and §10 (documentation updates) fix the closeout: flip Gate 10.1 → `Done` and Gate 10 → `Done` (and not before); add atlas first-use rows for follow-suit legality / trick resolution / trick-winner-leads / deal rotation as `local-only`; update `progress.md`, root `README.md`; run doc-link + catalog checks; do NOT edit `docs/ROADMAP.md` to record progress.
3. Shared boundary under audit: the `specs/README.md` index status contract and the `docs/MECHANIC-ATLAS.md` first-use registry. The Gate 10 parent flip depends on the poker_lite half already being complete (it is) plus this gate's evidence.
4. FOUNDATIONS §11 (every acceptance invariant proven by tests/traces/sims/benchmarks/docs before `Done`) and §6 (evidence-heavy games) are under audit — the flip is gated on the full evidence set passing.
5. Enforcement surface: this capstone re-runs the gate-wide no-leak, deterministic replay/hash, bot-legality, and benchmark evidence (§11) as the precondition for the `Done` flip; it records, not weakens, that evidence. If GAT101PLATRI-002 decided *promote*, this ticket also confirms atlas §10A promotion debt is closed (by GAT101PLATRI-003) before flipping — add `Deps: GAT101PLATRI-003` in that branch.

## Architecture Check

1. A single capstone that runs the exit evidence and performs status reconciliation (vs. scattering status flips across tickets) keeps the `Done` flip gated on the full evidence set and gives one reviewable completion diff.
2. No backwards-compatibility aliasing/shims; status + atlas-row reconciliation only.
3. `engine-core` untouched; no `game-stdlib` change. (Any §4 promotion adoption is owned by GAT101PLATRI-002/003.)

## Verification Layers

1. Full acceptance-evidence command set passes -> `cargo fmt --check` / `clippy` / `cargo test --workspace` / `simulate` / `replay-check` / `fixture-check` / `rule-coverage` / `cargo bench -p plain_tricks` / boundary + doc-link + catalog checks / `smoke:wasm` / `smoke:ui` / `smoke:e2e`.
2. Atlas first-use trick rows present as `local-only` -> codebase grep-proof on `docs/MECHANIC-ATLAS.md`.
3. Status flips correct (Gate 10.1 → Done; Gate 10 → Done; progress + root README updated; ROADMAP untouched) -> grep-proof on `specs/README.md`, `progress.md`, `README.md`; confirm `docs/ROADMAP.md` unchanged.
4. Gate-wide FOUNDATIONS alignment -> FOUNDATIONS alignment check (§11 invariants all proven).

## What to Change

### 1. `docs/MECHANIC-ATLAS.md`

Add first-use rows for the trick-specific shapes (follow-suit legality, trick resolution, trick-winner-leads, deal rotation) marked `local-only`. (The §10B third-use decision row was updated in GAT101PLATRI-002; §10A debt — if any — closed in GAT101PLATRI-003.)

### 2. `specs/README.md`

Flip the Gate 10.1 row to `Done`; flip the parent Gate 10 row from `In progress` to `Done` (only now that the trick half is complete).

### 3. `progress.md` + root `README.md`

Record Gate 10.1 evidence; flip the Gate 10 narrative from "trick half deferred" to complete; add **Plain Tricks** / `plain_tricks` to the root README implemented-game catalog.

## Files to Touch

- `docs/MECHANIC-ATLAS.md` (modify)
- `specs/README.md` (modify)
- `progress.md` (modify)
- `README.md` (modify)

## Out of Scope

- Any production code or per-game docs (tickets 001–019).
- Editing `docs/ROADMAP.md` to record progress (spec §10 forbids it).
- The third-use ledger decision / §10A debt closure (GAT101PLATRI-002/003).

## Acceptance Criteria

### Tests That Must Pass

1. The full spec §7 acceptance-evidence command set passes (fmt/clippy/test/simulate/replay-check/fixture-check/rule-coverage/bench/boundary/doc-links/catalog/smoke:wasm/smoke:ui/smoke:e2e).
2. `grep -n "Gate 10.1" specs/README.md` shows `Done`; the Gate 10 row shows `Done`.
3. `git diff --stat docs/ROADMAP.md` is empty (no progress edit).

### Invariants

1. The `Done` flip happens only after every acceptance invariant is proven by tests/traces/sims/benchmarks/docs (FOUNDATIONS §6/§11).
2. No open promotion debt remains in atlas §10A before the successor gate (Gate 11) can be admitted (FOUNDATIONS §4/§11).

## Test Plan

### New/Modified Tests

1. `None — verification-only capstone; it runs the existing pipeline/evidence set and reconciles status/atlas/index docs (named in Assumption Reassessment).`

### Commands

1. `cargo fmt --all --check && cargo clippy --workspace --all-targets -- -D warnings && cargo test --workspace`
2. `cargo run -p simulate -- --game plain_tricks --games 1000 --start-seed 0 --action-cap 32 && cargo run -p replay-check -- --game plain_tricks && cargo run -p fixture-check -- --game plain_tricks && cargo run -p rule-coverage -- --game plain_tricks && cargo bench -p plain_tricks`
3. `bash scripts/boundary-check.sh && node scripts/check-doc-links.mjs && node scripts/check-catalog-docs.mjs && npm --prefix apps/web run smoke:wasm && npm --prefix apps/web run smoke:ui && npm --prefix apps/web run smoke:e2e`

## Outcome

Complete. Gate 10.1 and the parent Gate 10 are marked `Done`; the Gate 10.1 spec is archived; `progress.md`, root `README.md`, and `docs/MECHANIC-ATLAS.md` record the Plain Tricks completion and first-use local-only trick rows. `docs/ROADMAP.md` was intentionally left untouched.

Verification evidence:

1. `cargo fmt --all --check && cargo clippy --workspace --all-targets -- -D warnings && cargo test --workspace`
2. `cargo run -p simulate -- --game plain_tricks --games 1000 --start-seed 0 --action-cap 32 && cargo run -p replay-check -- --game plain_tricks && cargo run -p fixture-check -- --game plain_tricks && cargo run -p rule-coverage -- --game plain_tricks && cargo bench -p plain_tricks`
3. `bash scripts/boundary-check.sh && node scripts/check-doc-links.mjs && node scripts/check-catalog-docs.mjs && npm --prefix apps/web run smoke:wasm && npm --prefix apps/web run smoke:ui`
4. `npm --prefix apps/web run smoke:e2e` (rerun with elevated localhost permission after the sandbox rejected `127.0.0.1` binding with `EPERM`)
5. `grep -n "Gate 10" specs/README.md`
6. `grep -n "follow-suit legality\|trick resolution\|trick-winner-leads\|deal rotation" docs/MECHANIC-ATLAS.md`
7. `git diff --stat docs/ROADMAP.md`

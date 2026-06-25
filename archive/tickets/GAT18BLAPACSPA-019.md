# GAT18BLAPACSPA-019: capstone — exit-criteria command suite, source bibliography, and Gate 18 Done flip

**Status**: COMPLETED
**Priority**: MEDIUM
**Effort**: Medium
**Engine Changes**: Acceptance-maintenance only — final closeout updated docs/status surfaces and applied non-behavioral clippy/smoke fixes exposed by the full suite.
**Deps**: GAT18BLAPACSPA-010, GAT18BLAPACSPA-012, GAT18BLAPACSPA-013, GAT18BLAPACSPA-016, GAT18BLAPACSPA-017, GAT18BLAPACSPA-018

## Problem

Run the full §7 command suite and confirm every §6 exit-criteria / §6.3 acceptance row, add the repo-level `docs/SOURCES.md` bibliography entry and Rulepath-lesson notes, finalize the `GAME-EVIDENCE.md` completion status, and flip Gate 18 to `Done` in `specs/README.md` and in the spec's own Status — only after all evidence passes and no blocker remains (spec §6, §7.1, §10.1, §10.2, §11.2, candidate task `GAT18-BLAPAC-015`). This is the verification-only closeout that exercises the suite the prior tickets composed.

## Assumption Reassessment (2026-06-25)

1. `specs/README.md` lists Gate 18 (Order 9) as `Not started`, slug "Gate 18 — Spades (partnerships)", currently `(seed; unwritten)` (verified `:107`); this ticket links the spec and flips it to `Done` only at closeout. 8F is `Done` (`:106`).
2. The §7.1 command suite (cargo fmt/clippy/test, fixture/replay/rule-coverage/simulate, benches, the `scripts/check-*.mjs` set, the `apps/web` smoke set) is the acceptance surface; the spec's §6.2/§6.3 rows are the checklist. No new production logic is added.
3. Cross-artifact boundary under audit: this capstone exercises tickets 003–018 end-to-end; it modifies only status/index/source-doc/evidence surfaces, never upstream implementation files.
4. FOUNDATIONS §6 (evidence-heavy) / §11 motivate this ticket: the gate is not done until tests, traces, replay, visibility, bots, benchmarks, docs, and the forward-v1 receipt all pass; the `Done` flip is gated on that evidence, and human IP/release review is recorded as the remaining release blocker.

## Architecture Check

1. A single verification-only capstone owning the status reconciliation (vs. flipping status mid-gate) keeps `Done` gated on the full evidence run and gives one auditable closeout diff.
2. No shims; no production logic; the `Done` flip follows `specs/README.md` convention.
3. `engine-core` untouched; no `game-stdlib` change; docs/status only.

## Verification Layers

1. Full command suite green -> run the §7.1 suite (cargo + tool + `scripts/check-*.mjs` + `apps/web` smokes).
2. Every §6.2/§6.3 exit row satisfied -> manual checklist against the spec with each row's evidence command.
3. Index + spec Status consistent and links resolve -> grep-proof of `Done` in `specs/README.md` Gate 18 row + `node scripts/check-doc-links.mjs`.

## What to Change

### 1. Exit-criteria verification run

Execute the §7.1 command suite and the §6.2/§6.3 rows; record the command/evidence summary in `GAME-EVIDENCE.md` (final status) — no production edits.

### 2. Repo source bibliography

`docs/SOURCES.md`: add the Blackglass Pact bibliography entry + Rulepath-lesson notes (variant pinning, failed-nil attribution, pre-deal blind commitment, public-vs-private partnership signals); distinguish external sources from in-repository evidence.

### 3. Status flip

`specs/README.md`: link the spec and flip the Gate 18 row to `Done` with date + evidence link (do not mark Gate 19 active); `specs/gate-18-...md`: set Status to `Done`. Record human IP/release review as the remaining blocker.

## Files to Touch

- `specs/README.md` (modify)
- `specs/gate-18-blackglass-pact-spades-partnership-trick-taking.md` (modify — Status)
- `docs/SOURCES.md` (modify)
- `games/blackglass_pact/docs/GAME-EVIDENCE.md` (modify; created by GAT18BLAPACSPA-002)

## Out of Scope

- Any behavioral production code/test/bench change (prior tickets own those).
- Marking Gate 19 active or archiving the spec (separate later workflow per `docs/archival-workflow.md`).

## Acceptance Criteria

### Tests That Must Pass

1. The full §7.1 command suite passes: `cargo fmt --all --check`, `cargo clippy --workspace --all-targets -- -D warnings`, `cargo test --workspace`, the four game tools for `blackglass_pact`, `cargo bench -p blackglass_pact`, the `scripts/check-*.mjs` set, and the `apps/web` smoke set.
2. `grep -n "blackglass" specs/README.md` shows the Gate 18 row flipped to `Done` with an evidence link.
3. `node scripts/check-doc-links.mjs` and `node scripts/check-scaffolding-governance.mjs` pass.

### Invariants

1. Gate 18 flips to `Done` only after every exit/debt row is closed; Gate 19 is not marked active.
2. No behavioral upstream implementation change is modified by this capstone; human IP/release review is recorded as the remaining blocker.

## Test Plan

### New/Modified Tests

1. `None — verification-only capstone; it exercises the acceptance suite composed by GAT18BLAPACSPA-003–018 and adds no test file.`

### Commands

1. `cargo test --workspace && cargo run -p rule-coverage -- --game blackglass_pact && cargo run -p replay-check -- --game blackglass_pact --all`
2. `npm --prefix apps/web run smoke:e2e && node scripts/check-catalog-docs.mjs && node scripts/check-scaffolding-governance.mjs`
3. The whole-gate suite is the correct boundary for a capstone; it adds no production logic, only the status reconciliation.

## Outcome

Completed 2026-06-25.

The full Gate 18 command suite passed and the gate was flipped to `Done` in
both `specs/README.md` and the Gate 18 spec header. `GAME-EVIDENCE.md` now
records final closeout status, command evidence, and the remaining
human-owned IP/public-release review blocker. `docs/SOURCES.md` now links the
Blackglass Pact source notes and records the Rulepath lessons for variant
pinning, failed-nil bag attribution, pre-deal blind commitment, and public vs.
private partnership signals.

The capstone remained behaviorally closed, but the full acceptance run exposed
two narrow maintenance fixes required to make the suite truthful:

- `cargo clippy --workspace --all-targets -- -D warnings` required boxing the
  large `MatchOutcome` effect payload and removing a useless WASM bridge
  `format!` wrapper.
- `npm --prefix apps/web run smoke:animation` exposed a stale catalog-sweep
  count/list after Blackglass Pact joined the catalog; the smoke script now
  derives its matrix count from the explicit game lists and treats Blackglass
  Pact as generic-only for that sweep.

Verification passed:

- `cargo fmt --all --check`
- `cargo clippy --workspace --all-targets -- -D warnings`
- `cargo build --workspace`
- `cargo test --workspace`
- `cargo test -p blackglass_pact`
- `cargo test -p wasm-api`
- `cargo run -p replay-check -- --game blackglass_pact --all`
- `cargo run -p simulate -- --game blackglass_pact --seat-count 4 --games 1000 --start-seed 180400 --action-cap 4096`
- `cargo bench -p blackglass_pact`
- `cargo run -p fixture-check -- --game blackglass_pact`
- `cargo run -p rule-coverage -- --game blackglass_pact`
- `node scripts/check-catalog-docs.mjs`
- `node scripts/check-scaffolding-governance.mjs`
- `node --test scripts/check-scaffolding-governance.test.mjs`
- `node scripts/check-doc-links.mjs`
- `node scripts/check-outcome-explanations.mjs`
- `node scripts/check-ci-games.mjs`
- `node scripts/check-player-rules.mjs`
- `node scripts/check-presentation-copy.mjs`
- `bash scripts/boundary-check.sh`
- `git diff --check`
- `npm --prefix apps/web run smoke:wasm`
- `npm --prefix apps/web run smoke:ui`
- `npm --prefix apps/web run smoke:effects`
- `npm --prefix apps/web run smoke:e2e`
- `npm --prefix apps/web run smoke:preview`
- `npm --prefix apps/web run smoke:animation`

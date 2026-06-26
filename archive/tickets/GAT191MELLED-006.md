# GAT191MELLED-006: Evidence + docs + Gate 19.1 closeout

**Status**: COMPLETED
**Priority**: MEDIUM
**Effort**: Medium
**Engine Changes**: Yes (deterministic evidence + docs) — new golden trace; `games/meldfall_ledger/docs/*`; `specs/*`
**Deps**: GAT191MELLED-004, GAT191MELLED-005

## Problem

With the transition implemented and verified, Gate 19.1 needs its evidence and
docs reconciled to the now-true state: the reset/preserve contract captured as a
new declarative trace, `ML-MATCH-006` marked covered (it is currently
`intentionally-deferred`), a multi-round completion receipt recorded, and the spec
+ index flipped to `Done`. This closeout capstone exercises no new production
logic — it records the evidence the prior tickets produced.

## Assumption Reassessment (2026-06-26)

1. `games/meldfall_ledger/tests/golden_traces/` holds **declarative** fixtures
   validated by `validate_meldfall_ledger_trace` (`tools/replay-check/src/main.rs:717`)
   via parse / card-conservation / setup-projection — not command replay. The
   existing `multi-round-first-to-500.trace.json` and `target-tie-continues.trace.json`
   are static scoring-illustration fixtures and are **left intact** (spec §7.3). The
   one new trace `round-transition-resets-table-state.trace.json` must conform to
   the same `{game, trace, rules, ...}` schema the siblings use.
2. `games/meldfall_ledger/docs/RULE-COVERAGE.md:62` marks `ML-MATCH-006`
   `intentionally-deferred` ("future round transition") — flip to covered, citing
   the GAT191MELLED-002/-004 tests + the new trace; `ML-MATCH-003` tie continuation
   likewise. `GAME-EVIDENCE.md` gains the completion receipt. `specs/README.md`
   row 10.1 (Gate 19.1) reads `Planned` (line 109) → `Done`; the spec's own Status
   header flips to `Done`.
3. Cross-artifact boundary under audit: ADR 0009
   (`docs/adr/0009-replay-fixture-hash-taxonomy.md`, Accepted) governs the
   legitimate Meldfall trace/fixture/hash artifact movement — the new trace and any
   regenerated meldfall artifact fall under it; no unrelated artifact is touched.
4. FOUNDATIONS §11 evidence coverage restated: the change is covered by the
   transition unit tests (002), host-parity + no-leak tests (004), `simulate`
   completion (004), `replay-check`/`rule-coverage`/`fixture-check`, and these docs.
   `MECHANICS.md` / `HOW-TO-PLAY.md` are verify-only (rules version unchanged, so
   prose should already match); if `HOW-TO-PLAY.md` changes, the generated
   `apps/web/public/rules/meldfall_ledger.md` is regenerated via
   `scripts/copy-player-rules.mjs` and guarded by `scripts/check-player-rules.mjs`.

## Architecture Check

1. Authoring one new declarative trace (the reset/preserve contract) while leaving
   the existing scoring-illustration fixtures intact is cleaner than clobbering
   them — the *executable* proof of the transition is the host-parity test (004),
   not a declarative trace.
2. No backwards-compatibility shim: the docs are reconciled to the true state and
   the `intentionally-deferred` marker is removed, not aliased.
3. `engine-core` / `game-stdlib` untouched; this ticket ships evidence and docs only.

## Verification Layers

1. New trace conforms and passes -> `replay-check --game meldfall_ledger --all` green.
2. `ML-MATCH-006` (+ `ML-MATCH-003`) covered -> `rule-coverage --game meldfall_ledger` green; grep `RULE-COVERAGE.md` shows covered, not `intentionally-deferred`.
3. Completion receipt + status flip recorded -> grep `GAME-EVIDENCE.md` for the receipt; `specs/README.md` row 10.1 and the spec Status both read `Done`.
4. Doc/fixture gates stay green -> `fixture-check --game meldfall_ledger`, `node scripts/check-doc-links.mjs` (and `check-player-rules` if `HOW-TO-PLAY.md` changed).

## What to Change

### 1. New golden trace

Author `round-transition-resets-table-state.trace.json` conforming to the meldfall
declarative trace schema, asserting dealer rotation, round-only-state reset
(tableau/discard/hands/pending/round_end), and carried-forward cumulative scores.

### 2. Coverage + evidence docs

Mark `ML-MATCH-006` and `ML-MATCH-003` covered in `RULE-COVERAGE.md` with the new
tests/trace; add the multi-round completion receipt (commands, host-parity,
`simulate` completion) to `GAME-EVIDENCE.md`.

### 3. Verify-only prose

Confirm `MECHANICS.md` / `HOW-TO-PLAY.md` round/match-flow prose matches the
implemented transition; edit only if stale, regenerating the player-rules copy if
`HOW-TO-PLAY.md` changes.

### 4. Closeout

Flip the spec Status to `Done` and the `specs/README.md` row 10.1 to `Done`.

## Files to Touch

- `games/meldfall_ledger/tests/golden_traces/round-transition-resets-table-state.trace.json` (new)
- `games/meldfall_ledger/docs/RULE-COVERAGE.md` (modify)
- `games/meldfall_ledger/docs/GAME-EVIDENCE.md` (modify)
- `games/meldfall_ledger/docs/MECHANICS.md` (verify-only — edit only if stale)
- `games/meldfall_ledger/docs/HOW-TO-PLAY.md` (verify-only — edit only if stale; regenerate `apps/web/public/rules/meldfall_ledger.md` via `scripts/copy-player-rules.mjs` if changed)
- `specs/gate-19-1-meldfall-ledger-multi-round-completion.md` (modify — Status → `Done`)
- `specs/README.md` (modify — row 10.1 → `Done`)

## Out of Scope

- Any engine, host, or web behavior change (delivered in GAT191MELLED-001…005).
- Rewriting or clobbering the existing scoring-illustration fixtures
  (`multi-round-first-to-500.trace.json`, `target-tie-continues.trace.json`).
- Regenerating any non-Meldfall golden/fixture/hash artifact.

## Acceptance Criteria

### Tests That Must Pass

1. `cargo run -p replay-check -- --game meldfall_ledger --all` — green including the new trace.
2. `cargo run -p rule-coverage -- --game meldfall_ledger && cargo run -p fixture-check -- --game meldfall_ledger` — green; `ML-MATCH-006` covered.
3. `node scripts/check-doc-links.mjs` — green (and `node scripts/check-player-rules.mjs` if `HOW-TO-PLAY.md` changed).

### Invariants

1. `ML-MATCH-006` is marked covered with real evidence and the
   `intentionally-deferred` marker is gone; the spec and index both read `Done`.
2. Only ADR-0009-governed Meldfall artifacts moved; no unrelated artifact was regenerated.

## Test Plan

### New/Modified Tests

1. `games/meldfall_ledger/tests/golden_traces/round-transition-resets-table-state.trace.json`
   — the reset/preserve declarative trace (dealer rotates, round-only state resets,
   scores carry forward).

### Commands

1. `cargo run -p replay-check -- --game meldfall_ledger --all`
2. `cargo run -p rule-coverage -- --game meldfall_ledger && cargo run -p fixture-check -- --game meldfall_ledger`
3. `node scripts/check-doc-links.mjs` — doc-link integrity for the reconciled docs
   (the narrow gate for this docs/evidence closeout; full behavior is guarded by
   the prior tickets' suites).

## Outcome

Completed: 2026-06-26

Added `round-transition-resets-table-state.trace.json` as the declarative
reset/preserve contract for the Rust-owned multi-round transition. The trace
records dealer rotation, active-seat selection, round-only state reset, score
carry-forward, tie-continuation advancement, and no hidden new stock/private-hand
leak claim.

Reconciled Meldfall evidence docs: `RULE-COVERAGE.md` now marks `ML-MATCH-003`
and `ML-MATCH-006` covered with the new trace/tests/host evidence, and
`GAME-EVIDENCE.md` includes the Gate 19.1 multi-round completion receipt. Verified
`MECHANICS.md` and `HOW-TO-PLAY.md` already matched the implemented multi-round
flow, so no player-rules regeneration was needed.

Flipped the Gate 19.1 spec status and `specs/README.md` row to `Done`.

Verification:

- `cargo run -p replay-check -- --game meldfall_ledger --all`
- `cargo run -p rule-coverage -- --game meldfall_ledger && cargo run -p fixture-check -- --game meldfall_ledger`
- `node scripts/check-doc-links.mjs`
- `git diff --check`

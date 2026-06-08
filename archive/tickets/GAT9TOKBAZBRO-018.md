# GAT9TOKBAZBRO-018: Capstone — acceptance evidence + MECHANIC-ATLAS first-use + status reconciliation

**Status**: COMPLETED
**Priority**: HIGH
**Effort**: Medium
**Engine Changes**: None — verification + docs/status only (`docs/MECHANIC-ATLAS.md`, `specs/gate-9-token-bazaar-browser-proof.md`, `specs/README.md`, `progress.md`, `README.md`)
**Deps**: GAT9TOKBAZBRO-010, GAT9TOKBAZBRO-011, GAT9TOKBAZBRO-012, GAT9TOKBAZBRO-016, GAT9TOKBAZBRO-017

## Problem

Gate 9 is accepted only when the full acceptance-evidence suite passes end-to-end
and the repository's status records are reconciled: the mechanic atlas records
Token Bazaar as first official public resource/accounting pressure (kept local),
and `specs/README.md` + the spec Status flip to `Done`, with `progress.md` and root
`README.md` updated. This capstone exercises every prior ticket's deliverable and
performs the status reconciliation; it introduces no new production logic.

## Assumption Reassessment (2026-06-08)

1. Every implementation ticket has landed: rules/effects/views/replay/bots
   (GAT9TOKBAZBRO-002…008), tests + traces (-009/-010), benchmarks (-011), tools +
   rule-coverage (-012), WASM + web + e2e/CI (-013…016), and docs (-001/-011/-012/
   -017). `docs/MECHANIC-ATLAS.md` currently records resource accounting as a
   "repeated-shape candidate after second economy/betting use" (verified, line
   ~206) and "No open promotion debt remains" (verified, line ~199); this ticket
   adds the Token Bazaar first-use row. `specs/README.md` Gate 9 row reads
   `Planned` (already flipped during the `/reassess-spec` session on 2026-06-08);
   this ticket flips it and the spec Status to `Done`.
2. The acceptance criteria are fixed by
   `specs/gate-9-token-bazaar-browser-proof.md` → "Acceptance criteria" (Rules/
   state, Docs, Tests/replay, Bot, Browser, Boundary/CI) and "Exit criteria" (the
   ROADMAP §11 row-for-row table, with the two simultaneous-choice rows deferred to
   the successor Gate 9.1 — out of scope here).
3. Cross-artifact boundary under audit: the `specs/README.md` index status
   contract + the `docs/MECHANIC-ATLAS.md` first-use ledger. The Done-flip is gated
   on the exit evidence passing (this ticket re-runs it), so it belongs to the
   capstone, not the docs ticket. ROADMAP.md is NOT edited (the index tracks
   progress).
4. FOUNDATIONS §4 + §11 (mechanic atlas earned; full evidence coverage): restating
   before trusting the spec — Token Bazaar's resource/accounting stays local
   first-use; the atlas row records the evidence and keeps it a later promotion
   candidate (no `game-stdlib` primitive). The capstone confirms the gate's full
   evidence set (tests, traces, replay, no-leak, bot legality, benchmarks, docs)
   passes before the Done-flip.

## Architecture Check

1. A single trailing capstone that re-runs the exit evidence and performs the
   status reconciliation keeps the Done-flip honest (it cannot pass until the gate
   actually passes) and isolates the cross-repo status edits from implementation
   diffs; this matches the `high_card_duel` capstone shape.
2. No backwards-compatibility aliasing/shims — additive atlas row + status edits.
3. No new production logic; `engine-core`/`game-stdlib` untouched. The atlas row
   explicitly keeps resource/accounting local (no promotion).

## Verification Layers

1. Rust evidence -> `cargo test --workspace` + `cargo run -p simulate -- --game token_bazaar --games 1000 --start-seed 1`.
2. Replay/fixture/rule-coverage -> `cargo run -p replay-check -- --game token_bazaar --all` + `fixture-check` + `rule-coverage`.
3. Benchmarks -> `cargo bench -p token_bazaar` (+ bench-report threshold check).
4. Browser + boundary + docs -> `npm --prefix apps/web run smoke:e2e`,
   `bash scripts/boundary-check.sh`, `node scripts/check-doc-links.mjs`.
5. Status reconciliation -> grep-proof that the atlas first-use row, the spec
   Status `Done`, and the index row `Done` are present.

## What to Change

### 1. `docs/MECHANIC-ATLAS.md`

Add the Token Bazaar first-use row to the resource/accounting ledger: first
official public resource/accounting pressure, implemented local, kept as a later
promotion candidate; promotion-debt status unchanged (none).

### 2. `specs/gate-9-token-bazaar-browser-proof.md` + `specs/README.md`

Flip the spec Status to `Done` and the index Gate 9 row to `Done` (per the spec's
Documentation-updates section), once the evidence above passes.

### 3. `progress.md` + `README.md`

Record Gate 9 / Token Bazaar as landed.

## Files to Touch

- `docs/MECHANIC-ATLAS.md` (modify)
- `specs/gate-9-token-bazaar-browser-proof.md` (modify)
- `specs/README.md` (modify)
- `progress.md` (modify)
- `README.md` (modify)

## Out of Scope

- Any production code, test, trace, or doc *content* (owned by GAT9TOKBAZBRO-001…017).
- Editing `docs/ROADMAP.md` to record progress (the index tracks status).
- Authoring the successor Gate 9.1 `secret_draft` spec (out of scope for Gate 9).

## Acceptance Criteria

### Tests That Must Pass

1. `cargo test --workspace` and all four token_bazaar tool CLIs pass.
2. `cargo bench -p token_bazaar` + threshold check; `npm --prefix apps/web run smoke:e2e`;
   `bash scripts/boundary-check.sh`; `node scripts/check-doc-links.mjs`.
3. Grep-proof: atlas first-use row present; spec Status `Done`; index Gate 9 row `Done`.

### Invariants

1. The Done-flip is gated on the full exit-evidence suite passing (re-run at
   capstone time, not copied).
2. Resource/accounting remains local first-use; no `game-stdlib` primitive and no
   `engine-core` economy noun was added across the gate (§3/§4).
3. The two deferred ROADMAP §11 simultaneous-choice exit lines are carried to the
   successor Gate 9.1, not marked satisfied here.

## Test Plan

### New/Modified Tests

1. `None — verification-only capstone; it exercises the pipeline GAT9TOKBAZBRO-001…017 composed.`

### Commands

1. `cargo test --workspace && cargo run -p simulate -- --game token_bazaar --games 1000 --start-seed 1 && cargo run -p replay-check -- --game token_bazaar --all && cargo run -p fixture-check -- --game token_bazaar && cargo run -p rule-coverage -- --game token_bazaar`
2. `cargo bench -p token_bazaar && npm --prefix apps/web run smoke:e2e && bash scripts/boundary-check.sh && node scripts/check-doc-links.mjs`
3. This full re-run is the correct boundary for a capstone — it is exactly the
   spec's acceptance-evidence + exit-criteria suite, end-to-end.

## Outcome (2026-06-08)

Completed the Gate 9 Token Bazaar capstone.

- Re-ran the full acceptance evidence suite before status reconciliation.
- Recorded Token Bazaar as first official public resource/accounting pressure in
  `docs/MECHANIC-ATLAS.md`, kept local with no `game-stdlib` promotion debt.
- Flipped the Gate 9 spec and `specs/README.md` index row to `Done`.
- Updated `progress.md` and root `README.md` to record Gate 9 completion and
  the deferred successor commitment/reveal gate.
- Archived the active Gate 9 spec after updating the index link.

Verification:

1. `cargo test --workspace`
2. `cargo run -p simulate -- --game token_bazaar --games 1000 --start-seed 1`
3. `cargo run -p replay-check -- --game token_bazaar --all`
4. `cargo run -p fixture-check -- --game token_bazaar`
5. `cargo run -p rule-coverage -- --game token_bazaar`
6. `cargo bench -p token_bazaar`
7. `cargo run -p bench-report -- --input /tmp/token_bazaar-bench.txt --thresholds games/token_bazaar/benches/thresholds.json`
8. `npm --prefix apps/web run smoke:e2e`
9. `bash scripts/boundary-check.sh`
10. `node scripts/check-doc-links.mjs`

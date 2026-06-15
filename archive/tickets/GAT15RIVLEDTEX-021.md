# GAT15RIVLEDTEX-021: Verification sweep and Gate 15 close-out

**Status**: COMPLETED
**Priority**: HIGH
**Effort**: Medium
**Engine Changes**: None (status/docs — `specs/README.md`, `specs/gate-15-river-ledger-texas-holdem-base.md` Status)
**Deps**: GAT15RIVLEDTEX-011, GAT15RIVLEDTEX-015, GAT15RIVLEDTEX-018, GAT15RIVLEDTEX-019, GAT15RIVLEDTEX-020

## Problem

Gate 15 closes only when every exit criterion has passing evidence. This capstone runs the complete acceptance suite end-to-end (no new production logic), then performs the status reconciliation: flip the spec Status and the `specs/README.md` Order 5 / Gate 15 row to `Done`.

## Assumption Reassessment (2026-06-14)

1. No `games/river_ledger/` crate existed at gate start; tickets 003–020 compose the full game, tools, WASM, web, docs, and atlas evidence this ticket exercises. `specs/README.md` carries an Order 5 / Gate 15 seed row (`_(seed; unwritten)_`, `Not started`) — the reassessment did not edit the index, so this ticket performs the row update.
2. `specs/...-base.md` §6 (exit criteria), §7 (acceptance evidence + command suite), and §10.1 (index row update) fix the close-out; the spec's own Status is `Planned`.
3. Cross-artifact boundary under audit: this ticket exercises the surfaces built by every prior ticket and modifies only the status-reconciliation surfaces (`specs/README.md` row, the spec Status); it adds no production logic and does not edit upstream tickets' files.
4. FOUNDATIONS §11 acceptance invariants motivate this ticket: re-enumerate expected trace/fixture counts from the fixtures at run time rather than hardcoding, and confirm the full no-leak / determinism / bot-legality / boundary suite is green before the `Done`-flip.
5. §11 acceptance-invariant sweep is the enforcement surface: replay/hash determinism, pairwise no-leak (Rust + browser), bot legality, serialization order, and boundary noun-freedom must all pass; confirm no exit row is flipped `Done` without its command evidence.

## Architecture Check

1. A single verification-only capstone that runs the spec's acceptance suite and then flips status keeps the gate close auditable and gated on real evidence, matching the prior-gate close-out shape.
2. No backwards-compatibility aliasing/shims — verification + status only.
3. `engine-core` untouched (§3); no `game-stdlib` change (§4).

## Verification Layers

1. Full Rust suite + tools + sims green -> `cargo test --workspace`; `cargo run -p {fixture-check,rule-coverage,replay-check} -- --game river_ledger`; per-seat sims.
2. Benches + WASM + web + e2e green -> `cargo bench -p river_ledger`; `npm --prefix apps/web run build`; `npm --prefix apps/web run smoke:e2e`.
3. Boundary + docs + catalog + presentation-copy + player-rules + ci-games green -> the `scripts/check-*` + `boundary-check.sh` suite.
4. Status reconciliation -> `specs/README.md` Order 5 row + spec Status both read `Done`.

## What to Change

### 1. Acceptance sweep (no production logic)

Run the §7.1 command suite; triage any failure back to the owning ticket; record exact command output for the close-out.

### 2. Status reconciliation

Update the `specs/README.md` Order 5 / Gate 15 row (`_(seed; unwritten)_` → the spec path; `Not started` → `Done`) and flip the spec's own Status to `Done`, only after all exit criteria pass.

## Files to Touch

- `specs/README.md` (modify)
- `specs/gate-15-river-ledger-texas-holdem-base.md` (modify — Status → `Done`)

## Out of Scope

- Any production-logic, doc-content, or atlas change (owned by 003–020).
- Flipping status before exit criteria pass.
- Ticket archival (follow `docs/archival-workflow.md` separately).

## Acceptance Criteria

### Tests That Must Pass

1. `cargo test --workspace && cargo bench -p river_ledger` — full Rust suite + benches green.
2. `cargo run -p fixture-check -- --game river_ledger && cargo run -p rule-coverage -- --game river_ledger && cargo run -p replay-check -- --game river_ledger --all` — tools green.
3. `npm --prefix apps/web run build && npm --prefix apps/web run smoke:e2e && bash scripts/boundary-check.sh && node scripts/check-doc-links.mjs && node scripts/check-catalog-docs.mjs && node scripts/check-presentation-copy.mjs && node scripts/check-player-rules.mjs && node scripts/check-ci-games.mjs` — web + boundary + docs gates green.

### Invariants

1. Every exit row has command evidence before the `Done`-flip (§6/§11).
2. No-leak, determinism, bot-legality, and boundary invariants all hold at close (§11).

## Test Plan

### New/Modified Tests

1. `None — verification-only capstone; it exercises the full acceptance suite composed by tickets 003–020 and adds no tests.`

### Commands

1. `cargo test --workspace`
2. `cargo run -p fixture-check -- --game river_ledger && cargo run -p rule-coverage -- --game river_ledger && cargo run -p replay-check -- --game river_ledger --all && for n in 3 4 5 6; do cargo run -p simulate -- --game river_ledger --seat-count $n --games 1000 --start-seed 150$n; done`
3. `npm --prefix apps/web run build && npm --prefix apps/web run smoke:e2e && bash scripts/boundary-check.sh && node scripts/check-doc-links.mjs && node scripts/check-catalog-docs.mjs && node scripts/check-presentation-copy.mjs && node scripts/check-player-rules.mjs && node scripts/check-ci-games.mjs`

## Outcome

Completed: 2026-06-14

What changed:

- Ran the Gate 15 acceptance suite end-to-end and recorded the evidence in
  `specs/gate-15-river-ledger-texas-holdem-base.md`.
- Flipped the Gate 15 spec status to `Done`.
- Updated `specs/README.md` Order 5 / Gate 15 row to point at the archived
  River Ledger spec and read `Done`.
- Archived the Gate 15 spec after adding its `Outcome` section.

Deviations from plan:

- The exact spec command list also included `cargo check -p river_ledger` and
  individual River Ledger test targets; those were run explicitly in addition
  to `cargo test --workspace`.
- `cargo run -p replay-check -- --game river_ledger --all` was run for the
  ticket acceptance lane, and the exact spec command
  `cargo run -p replay-check -- --game river_ledger` was also run.

Verification:

- `cargo check -p river_ledger` — passed.
- `cargo test -p river_ledger` — passed.
- `cargo test -p river_ledger --test rules` — passed.
- `cargo test -p river_ledger --test property` — passed.
- `cargo test -p river_ledger --test replay` — passed.
- `cargo test -p river_ledger --test serialization` — passed.
- `cargo test -p river_ledger --test visibility` — passed.
- `cargo test -p river_ledger --test bots` — passed.
- `cargo test --workspace` — passed.
- `cargo run -p fixture-check -- --game river_ledger` — passed.
- `cargo run -p rule-coverage -- --game river_ledger` — passed.
- `cargo run -p replay-check -- --game river_ledger` — passed.
- `cargo run -p replay-check -- --game river_ledger --all` — passed.
- `cargo run -p simulate -- --game river_ledger --seat-count 3 --games 1000 --start-seed 1503` — passed.
- `cargo run -p simulate -- --game river_ledger --seat-count 4 --games 1000 --start-seed 1504` — passed.
- `cargo run -p simulate -- --game river_ledger --seat-count 5 --games 1000 --start-seed 1505` — passed.
- `cargo run -p simulate -- --game river_ledger --seat-count 6 --games 1000 --start-seed 1506` — passed.
- `cargo bench -p river_ledger` — passed.
- `npm --prefix apps/web run build` — passed.
- `npm --prefix apps/web run smoke:e2e` — passed.
- `bash scripts/boundary-check.sh` — passed.
- `node scripts/check-doc-links.mjs` — passed.
- `node scripts/check-catalog-docs.mjs` — passed.
- `node scripts/check-presentation-copy.mjs` — passed.
- `node scripts/check-player-rules.mjs` — passed.
- `node scripts/check-ci-games.mjs` — passed.

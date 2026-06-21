# GAT17VOWTIDOHHEL-022: Public-release closeout, central docs, helper conformance receipt, `Done`-flip

**Status**: COMPLETED
**Priority**: HIGH
**Effort**: Small
**Engine Changes**: None — docs/status-only (capstone): `games/vow_tide/docs/PUBLIC-RELEASE-CHECKLIST.md`, central `docs/SOURCES.md` / `docs/MECHANIC-ATLAS.md`, `specs/README.md`, the spec Status
**Deps**: 003, 004, 013, 014, 016, 019, 020, 021

## Problem

Close out Gate 17: complete `PUBLIC-RELEASE-CHECKLIST.md`, reconcile the central source/atlas records, record the promoted-helper conformance receipt with no open promotion debt, verify the §6 exit criteria / §7 acceptance evidence pass end-to-end, and flip Vow Tide to `Done` in the spec and the `specs/README.md` index. This capstone exercises the prior tickets; it adds no production logic.

## Assumption Reassessment (2026-06-21)

1. The implementation/evidence tickets (003–021) supply every exit-criterion surface; `specs/README.md` currently shows Gate 17 as `Not started` with a `(seed; unwritten)` link (it is flipped to `Planned` at spec acceptance and to `Done` here). The spec's §10.1/§10.7 drive this closeout; `docs/MECHANIC-ATLAS.md` §10A must read `Current debt: _None_`.
2. Spec §6/§7 enumerate the completion criteria + command suite; the back-ports (003/004) + helper (002) must show preserved Plain Tricks/Briar Circuit suites/traces/hashes — the no-debt receipt.
3. Cross-artifact boundary under audit: the `specs/README.md` index row, the spec Status, the central `docs/SOURCES.md` Vow Tide entry, and the `docs/MECHANIC-ATLAS.md` §10/§10A final state are the status-reconciliation surfaces; the capstone owns the `Done`-flip per the trailing-shape default.
4. FOUNDATIONS §4/§6 under audit: the gate is not `Done` until helper conformance has no open debt and every official-game evidence class passes; §10A staying `_None_` is the no-debt witness.

## Architecture Check

1. A single verification-only capstone owning the `Done`-flip + central reconciliation keeps the completion gate honest (it depends on the full leaf set) and avoids scattering status edits.
2. No shims; docs/status edits only.
3. `engine-core`/`game-stdlib` untouched; the capstone exercises, it does not modify, the implementation tickets' files.

## Verification Layers

1. Full native + tool + web suite green → the §7.1 command suite (workspace tests, fixture/replay/rule-coverage, simulate 3–7, benches, boundary/doc/catalog checks, web build/smokes).
2. Helper conformance: Plain Tricks/Briar Circuit suites/traces/hashes unchanged + §10A `_None_` → `cargo run -p replay-check -- --game {plain_tricks,briar_circuit} --all` + grep `docs/MECHANIC-ATLAS.md` §10A.
3. Catalog/rules/source/atlas/web README closeout consistent → `node scripts/check-catalog-docs.mjs` + `node scripts/check-doc-links.mjs`.
4. Status reconciled → grep `specs/README.md` + spec header for `Done`.

## What to Change

### 1. Closeout docs + central reconciliation

Complete `games/vow_tide/docs/PUBLIC-RELEASE-CHECKLIST.md`; add the Vow Tide / Oh Hell entry to central `docs/SOURCES.md`; finalize `docs/MECHANIC-ATLAS.md` §10/§10A (promoted-helper conformance receipt; §10A `Current debt: _None_`); record the human IP/release-review receipt.

### 2. `Done`-flip

Flip Gate 17 to `Done` in `specs/README.md` (completion date + concise evidence) and set the spec `Status` to `Done`; archive per `docs/archival-workflow.md`.

## Files to Touch

- `games/vow_tide/docs/PUBLIC-RELEASE-CHECKLIST.md` (new)
- `docs/SOURCES.md` (modify)
- `docs/MECHANIC-ATLAS.md` (modify)
- `specs/README.md` (modify)
- `specs/gate-17-vow-tide-oh-hell-bidding-trick-taking.md` (modify)

## Out of Scope

- Any production-logic, web-code, or test change (those are the upstream tickets).
- Modifying the upstream tickets' files — this capstone exercises them only.

## Acceptance Criteria

### Tests That Must Pass

1. `cargo test --workspace` + `cargo run -p {fixture-check,replay-check,rule-coverage} -- --game vow_tide` + the simulate 3–7 matrix — all green.
2. `npm --prefix apps/web run build && npm --prefix apps/web run smoke:e2e` + `node scripts/check-catalog-docs.mjs` — web acceptance green.
3. `grep -n "Done" specs/README.md` (Gate 17 row) and the spec header confirm the flip; `docs/MECHANIC-ATLAS.md` §10A reads `_None_`.

### Invariants

1. Gate 17 is `Done` only after every §6 criterion and §7 evidence class passes and helper conformance has no open promotion debt.
2. Plain Tricks and Briar Circuit behavior/traces/hashes remain unchanged post-conformance.

## Test Plan

### New/Modified Tests

1. `None — docs-reconciliation + verification capstone; it reconciles the named docs/status surfaces and exercises the prior tickets' acceptance suite, adding no test file.`

### Commands

1. `cargo test --workspace && cargo run -p replay-check -- --game vow_tide --all`
2. `cargo run -p replay-check -- --game plain_tricks --all && cargo run -p replay-check -- --game briar_circuit --all`
3. Narrower command rationale: the workspace suite + per-game replay-check (incl. the back-ported games) is the end-to-end acceptance + no-debt boundary; the web/doc gates confirm the public surface.

## Outcome

Completed: 2026-06-21.

Gate 17 was closed as `Done`. Added
`games/vow_tide/docs/PUBLIC-RELEASE-CHECKLIST.md`, reconciled central
`docs/SOURCES.md` and `docs/MECHANIC-ATLAS.md`, flipped the Gate 17 row in
`specs/README.md`, set the Gate 17 spec status/outcome to `Done`, and archived
the spec at
`archive/specs/gate-17-vow-tide-oh-hell-bidding-trick-taking.md`.

The atlas §10A no-debt receipt now records the Gate 17
`game-stdlib::trick_taking` helper promotion, same-gate Plain Tricks/Briar
Circuit conformance, Vow Tide helper use, and `Current debt: _None_`.

Verification:

- `cargo test --workspace`
- `cargo run -p fixture-check -- --game vow_tide`
- `cargo run -p replay-check -- --game vow_tide --all`
- `cargo run -p rule-coverage -- --game vow_tide`
- `cargo run -p simulate -- --game vow_tide --seat-count 3 --games 1000 --action-cap 512`
- `cargo run -p simulate -- --game vow_tide --seat-count 4 --games 1000 --action-cap 1024`
- `cargo run -p simulate -- --game vow_tide --seat-count 5 --games 1000 --action-cap 1024`
- `cargo run -p simulate -- --game vow_tide --seat-count 6 --games 1000 --action-cap 1024`
- `cargo run -p simulate -- --game vow_tide --seat-count 7 --games 1000 --action-cap 1024`
- `cargo run -p replay-check -- --game plain_tricks --all`
- `cargo run -p replay-check -- --game briar_circuit --all`
- `npm --prefix apps/web run build`
- `npm --prefix apps/web run smoke:e2e`
- `cargo bench -p vow_tide`
- `node scripts/check-catalog-docs.mjs`
- `node scripts/check-doc-links.mjs`
- `node scripts/check-outcome-explanations.mjs`
- `bash scripts/boundary-check.sh`

Deviation: the first 3-seat simulation attempt used the simulator default
`--action-cap 64` and correctly stopped before terminal because Vow Tide has
multi-hand matches. The accepted simulation evidence uses explicit complete-match
caps: 512 for 3 seats and 1024 for 4-7 seats.

# 8CR1PUBFIXSEA-038: Consolidated verification and 8C-R1 tracker closeout (capstone)

**Status**: PENDING
**Priority**: HIGH
**Effort**: Medium
**Engine Changes**: None — docs/status-only (`specs/README.md`, spec Status, register/report closeout); verification-only
**Deps**: 8CR1PUBFIXSEA-037

## Problem

Unit 8C-R1 closes only after the full §7 acceptance command set passes, the candidate diff is audited against the admission byte inventory (proving exactly the six authorized WASM traces changed), and the transcript is attached — then the `specs/README.md` `8C-R1` row flips to `Done` (spec §5.10 tasks `8C-R1-602`/`8C-R1-603`, EC-R1-19/EC-R1-21). This capstone exercises every prior ticket end-to-end and performs the status reconciliation; it introduces no production logic and modifies only docs/status surfaces.

## Assumption Reassessment (2026-06-23)

1. All migration tickets (`-002`…`-036`) and the register/checkpoint ticket (`-037`) have landed; `specs/README.md` currently shows `8C-R1` as the lowest non-`Done` row and `specs/8c-r1-public-fixed-seat-scaffolding.md` Status is `Planned`. Successor rows `8C-R2`/`R3`/`R4` and Gate 18 must remain sequenced after it. Confirmed during reassessment.
2. Spec §7.1 lists the exact command set; §7.4 authorizes exactly six `wasm-exported.trace.json` changes by default; §5.10 and EC-R1-21 require flipping only the `8C-R1` row after closeout. Confirmed during reassessment.
3. Cross-artifact: this capstone reconciles `specs/README.md`, the spec's own Status, the register, and the characterization report; it exercises (does not modify) the prior tickets' surfaces.
4. §11/§12 motivate this ticket: closeout must prove deterministic replay/hash across the wave and that no unexplained byte/hash/visibility change occurred — any such change blocks the flip (a §12 stop condition).
5. Enforcement surface = the full acceptance suite and the byte-diff audit; the capstone runs them and proves only the six authorized WASM traces changed, leaking no hidden information and breaking no deterministic replay/hash. It writes no code.

## Architecture Check

1. A single trailing verification-and-closeout capstone is the right shape: it gates the `Done`-flip on the exit evidence rather than letting any one migration ticket claim wave completion.
2. No backwards-compatibility shim is introduced; this ticket runs commands and reconciles status.
3. `engine-core` stays noun-free (§3); no `game-stdlib` change (§4); the closeout records the §10A no-promotion-debt-impacting result via `-037`'s register receipts.

## Verification Layers

1. Full §7 acceptance command set passes -> `cargo fmt --check`, per-crate + `cargo test --workspace --all-targets`, `replay-check --all` ×6, `fixture-check` ×6, `boundary-check.sh`, `cargo tree` dev-only proof, `check-doc-links.mjs`, `check-catalog-docs.mjs`.
2. Byte-diff audit: only the six authorized WASM traces changed -> `git diff --name-only` against the admission byte inventory from `-001`.
3. Status reconciliation: only `8C-R1` flips to `Done`; successors untouched -> grep `specs/README.md` rows.

## What to Change

### 1. Run the consolidated §7 verification

Run the full §7.1 command set from repository root, compare the candidate diff against the `-001` admission byte inventory, prove only the six authorized `wasm-exported.trace.json` files changed, and attach the complete transcript to the characterization report. Any unexplained hash/byte/visibility change blocks closeout.

### 2. Tracker and status closeout

After every exit criterion passes, flip the `specs/README.md` `8C-R1` row to `Done` with a concise closeout note and set the spec's own Status to `Done`. Record the register/report closeout. Do NOT edit `8C-R2`/`R3`/`R4` or Gate 18 status.

## Files to Touch

- `specs/README.md` (modify)
- `specs/8c-r1-public-fixed-seat-scaffolding.md` (modify; Status → `Done`)
- `docs/MECHANICAL-SCAFFOLDING-REGISTER.md` (modify; closeout note)
- `reports/8c-r1-public-fixed-seat-scaffolding-characterization.md` (modify; command transcript — created by 8CR1PUBFIXSEA-001)

## Out of Scope

- Any production code, hash, seat, or visibility change (verification + status only).
- Editing `8C-R2`/`8C-R3`/`8C-R4` or Gate 18 status.
- Regenerating any golden trace or accepting a new hash.

## Acceptance Criteria

### Tests That Must Pass

1. The full §7.1 command set passes (`cargo fmt --check`, `cargo test --workspace --all-targets`, all six `replay-check --all` and `fixture-check`, `boundary-check.sh`, `cargo tree` dev-only proof, `check-doc-links.mjs`, `check-catalog-docs.mjs`).
2. `git diff --name-only` shows exactly six `wasm-exported.trace.json` files changed across the wave and no other unexplained byte/hash/visibility change.
3. `specs/README.md` shows `8C-R1` as `Done` and `8C-R2`/`R3`/`R4` + Gate 18 unchanged.

### Invariants

1. Only `8C-R1` flips to `Done`; successor sequencing (EC-28/EC-30) is intact.
2. Only the six authorized WASM traces changed across the entire wave; every other byte/hash/visibility surface is identical to baseline.

## Test Plan

### New/Modified Tests

1. `None — docs-reconciliation + verification capstone; it reconciles the named docs/status surfaces and exercises the prior tickets' acceptance suite, adding no test file.`

### Commands

1. `cargo test --workspace --all-targets`
2. `cargo run -p replay-check -- --game token_bazaar --all` (representative; run all six per §7.1) and `node scripts/check-catalog-docs.mjs`
3. The full §7.1 command set is the correct boundary: this capstone's scope IS the spec's exit criteria, exercised end-to-end with no new production logic.

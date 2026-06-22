# UNI8CMECSCA-031: 8C closeout capstone — evidence, register finalize, `Done`-flip

**Status**: COMPLETED
**Priority**: HIGH
**Effort**: Medium
**Engine Changes**: None — docs/status-only (`specs/unit-8c-…md`, `specs/README.md`, `docs/MECHANICAL-SCAFFOLDING-REGISTER.md`)
**Deps**: UNI8CMECSCA-030

## Problem

Final closeout for 8C: run the §7 evidence commands, update the register statuses/evidence with exact pilot paths and migration receipts, confirm every §6 exit criterion (EC-01…EC-30) has linked evidence, and flip 8C to `Done` (the spec's Status and the `specs/README.md` row), leaving the four C-11 follow-on rows `Not started`. This is a verification-only + docs-reconciliation capstone: it exercises the suite the prior tickets composed and reconciles status surfaces; it adds no production logic.

## Assumption Reassessment (2026-06-22)

1. All implementation/pilot tickets (UNI8CMECSCA-005…027) and the consolidation/C-10/seed tickets (028/029/030) have landed; `specs/unit-8c-mechanical-scaffolding-code-extraction.md` Status is `Planned`; `specs/README.md` row 8C is `Planned` (UNI8CMECSCA-001) and carries the four C-11 rows (UNI8CMECSCA-030); the register entries `MSC-8C-001`…`010` exist with 001–009 `accepted` and 010 `rejected/local-only` (UNI8CMECSCA-002/028/029).
2. Spec §6 (EC-01…EC-30) and §7 fix the acceptance evidence: the command set (fmt, per-crate + workspace tests, `replay-check`/`fixture-check` per pilot, `boundary-check.sh`, `cargo tree -e normal --invert game-test-support`, `check-doc-links`, `check-catalog-docs`) and the focused evidence IDs (EV-REG…EV-FORWARD). `apps/web/README.md` is explicitly not applicable (§10.G).
3. Cross-artifact boundary under audit: the status-reconciliation surfaces (spec Status, `specs/README.md` row 8C, register statuses/evidence). Each is modified by an earlier ticket in this run; this capstone is the create-then-modify tail (`specs/README.md` created/edited by 001/030; register by 002/029 — the chain `031→030→029→028→…→002→001` reaches both creators).
4. FOUNDATIONS §11 "Documentation truth": the `Done`-flip happens only after all §6 criteria pass with linked evidence; no foundation/ADR/architecture/roadmap text is edited merely to record progress (§9.22, §10.E/F).
5. Determinism/no-leak (§11): a verification + status-only ticket; it runs the acceptance suite and reconciles status surfaces, touching no code/byte/fixture itself. It confirms no open scaffold/hash/trace/fixture/seat debt is attributable to 8C.

## Architecture Check

1. A single closeout capstone gated on the full exit-criteria suite keeps the `Done`-flip honest (evidence-first) and isolates status reconciliation from the implementation diffs.
2. No backwards-compatibility shim — status edits only; nothing aliased.
3. `engine-core`/`game-stdlib` untouched; the capstone exercises prior tickets, it does not modify their files.

## Verification Layers

1. Full acceptance suite passes → `cargo test --workspace --all-targets`, the per-pilot `replay-check`/`fixture-check`, `bash scripts/boundary-check.sh`, `cargo tree --workspace -e normal --invert game-test-support`, `node scripts/check-doc-links.mjs`, `node scripts/check-catalog-docs.mjs`.
2. Every EC-01…EC-30 has linked evidence → manual exit-criteria checklist against the spec.
3. Register statuses/evidence finalized (accepted entries carry pilot paths + receipts; 010 rejected/local-only) → grep-proof on `docs/MECHANICAL-SCAFFOLDING-REGISTER.md`.
4. 8C `Done` in both the spec Status and the index row; C-11 rows still `Not started` → grep-proof.

## What to Change

### 1. `docs/MECHANICAL-SCAFFOLDING-REGISTER.md`

Finalize each accepted entry's status/evidence with exact pilot paths, hash/visibility impact, migration receipts, and validator commands; keep `MSC-8C-010` rejected/local-only.

### 2. `specs/unit-8c-mechanical-scaffolding-code-extraction.md`

Set Status to `Done`; populate the outcome with links to the characterization/migration evidence, register entries, and C-11 seeds.

### 3. `specs/README.md`

Flip row 8C `Planned` → `Done` with the concise outcome/evidence link; leave 8C-R1…R4 `Not started`.

## Files to Touch

- `docs/MECHANICAL-SCAFFOLDING-REGISTER.md` (modify; entries created by UNI8CMECSCA-002, finalized here)
- `specs/unit-8c-mechanical-scaffolding-code-extraction.md` (modify — Status + outcome)
- `specs/README.md` (modify; row created/edited by UNI8CMECSCA-001/030, flipped to `Done` here)

## Out of Scope

- Any production-code, test, byte, or fixture change (those landed in earlier tickets).
- Modifying the upstream tickets' files (this capstone exercises them).
- Editing `docs/ROADMAP.md`, `apps/web/README.md`, or any foundation/ADR/architecture doc merely to record progress.

## Acceptance Criteria

### Tests That Must Pass

1. `cargo test --workspace --all-targets` passes; the per-pilot `replay-check`/`fixture-check` pass; `bash scripts/boundary-check.sh`, `cargo tree --workspace -e normal --invert game-test-support`, `node scripts/check-doc-links.mjs`, and `node scripts/check-catalog-docs.mjs` pass.
2. Every EC-01…EC-30 has linked evidence in the spec outcome / register.
3. `grep -nE '^\| 8C ' specs/README.md` shows `Done`; 8C-R1…R4 remain `Not started`.

### Invariants

1. The `Done`-flip occurs only after all §6 exit criteria pass with linked evidence.
2. No open scaffold/hash/trace/fixture/seat debt is attributable to 8C; no foundation/ADR/roadmap text is edited to record progress.

## Test Plan

### New/Modified Tests

1. `None — docs-reconciliation + verification capstone; it reconciles the spec Status / index / register surfaces and exercises the prior tickets' acceptance suite, adding no test file.`

### Commands

1. `cargo test --workspace --all-targets`
2. `bash scripts/boundary-check.sh && cargo tree --workspace -e normal --invert game-test-support && node scripts/check-doc-links.mjs && node scripts/check-catalog-docs.mjs`
3. The full workspace suite plus the boundary/dependency/doc guards are the correct boundary — the capstone verifies the composed pipeline and reconciles status, adding no logic.

## Outcome

Completed: 2026-06-22

Finalized Unit 8C as `Done` after running the capstone evidence suite. The spec
now has a closeout outcome with EC-01...EC-30 evidence mapping, the register has
a final 8C closeout proof block, and `specs/README.md` flips row 8C to `Done`
while leaving 8C-R1...8C-R4 `Not started`.

No production code, tests, fixtures, hashes, WASM surface, atlas text, roadmap,
foundation doc, ADR, architecture doc, or runtime behavior changed in this
capstone.

Verification:

1. `cargo fmt --all -- --check`
2. `cargo test -p engine-core`
3. `cargo test -p game-stdlib`
4. `cargo test -p game-test-support`
5. `cargo test -p wasm-api`
6. `cargo test -p race_to_n`
7. `cargo test -p draughts_lite`
8. `cargo test -p high_card_duel`
9. `cargo test -p river_ledger`
10. `cargo test -p vow_tide`
11. `cargo test -p briar_circuit`
12. `cargo test --workspace --all-targets`
13. `cargo run -p replay-check -- --game race_to_n --all`
14. `cargo run -p replay-check -- --game draughts_lite --all`
15. `cargo run -p replay-check -- --game high_card_duel --all`
16. `cargo run -p replay-check -- --game river_ledger --all`
17. `cargo run -p replay-check -- --game vow_tide --all`
18. `cargo run -p replay-check -- --game briar_circuit --all`
19. `cargo run -p fixture-check -- --game race_to_n`
20. `cargo run -p fixture-check -- --game river_ledger`
21. `cargo run -p fixture-check -- --game vow_tide`
22. `cargo run -p fixture-check -- --game briar_circuit`
23. `bash scripts/boundary-check.sh`
24. `cargo tree --workspace -e normal --invert game-test-support`
25. `node scripts/check-doc-links.mjs`
26. `node scripts/check-catalog-docs.mjs`
27. `grep -nE '^\| 8C ' specs/README.md`
28. `grep -nE '8C-R[1-4]' specs/README.md`
29. `rg -n 'Status.*Done|Unit 8C is `Done`|EC-01...EC-30|Unit 8C closeout evidence' archive/specs/unit-8c-mechanical-scaffolding-code-extraction.md docs/MECHANICAL-SCAFFOLDING-REGISTER.md`
30. `git diff --check`

Note: `cargo test --workspace --all-targets` exited successfully. Because this
workspace includes benchmark binaries as all-targets, that command printed some
pre-existing local benchmark rows with `pass: false`; those are not the
capstone's benchmark gate, and 8C did not edit benchmark sources or thresholds.

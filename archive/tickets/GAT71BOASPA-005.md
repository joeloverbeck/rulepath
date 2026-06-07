# GAT71BOASPA-005: Capstone — promotion-debt closure, acceptance evidence, and `draughts_lite` regression

**Status**: COMPLETED
**Priority**: HIGH
**Effort**: Medium
**Engine Changes**: Yes (governance/docs closure only) — `docs/MECHANIC-ATLAS.md`, `specs/README.md`, `games/draughts_lite/docs/MECHANICS.md`, `games/draughts_lite/docs/PRIMITIVE-PRESSURE-LEDGER.md`. No game rule code changes; this ticket runs the full acceptance pipeline and records closure.
**Deps**: GAT71BOASPA-001, GAT71BOASPA-002, GAT71BOASPA-003, GAT71BOASPA-004

## Problem

Once `three_marks`, `column_four`, and `directional_flip` conform to `game-stdlib::board_space` (GAT71BOASPA-001/002/003) and `race_to_n` is audited not-applicable (GAT71BOASPA-004), the Gate 7.1 exit criteria (spec `specs/gate-7-1-board-space-primitive-back-port.md` §E/§17) must be proven end-to-end and the promotion debt formally closed. This capstone exercises the full acceptance pipeline (§16) across every affected game, confirms `draughts_lite` still passes as the exemplar/regression target (§13), flips the `docs/MECHANIC-ATLAS.md` §10A `board_space` debt from open to closed, and flips the `specs/README.md` Gate 7.1 index row from `Planned` to `Done`. Closing this debt is the §12 interlock that unblocks Gate 8.

## Assumption Reassessment (2026-06-07)

1. The upstream retrofits land via GAT71BOASPA-001/002/003 (consuming `crates/game-stdlib::board_space`) and the audit via GAT71BOASPA-004. `games/draughts_lite` already depends on `game-stdlib` (`Cargo.toml:11`) and consumes `board_space` (`src/ids.rs:1`, `src/actions.rs`) — it needs no code change here, only a regression run and two doc clarifications.
2. `docs/MECHANIC-ATLAS.md` currently marks the primitive `promotion-debt-open` (`:186`, `:188`) with an open §10A register row naming `three_marks`/`column_four`/`directional_flip` as un-migrated and `race_to_n` as audit-pending (`:199`). `specs/README.md` lists Gate 7.1 as `Planned` (`:35`) and states `Done` is flipped "only after its exit-criteria section is satisfied with evidence" (`:46-47`). `games/draughts_lite/docs/PRIMITIVE-PRESSURE-LEDGER.md` still says "No forced back-port happens under this Gate 7 decision" (`:213`, `:298`). Spec authority: §15 (final paragraph), §17.10–§17.12, §E.
3. Cross-artifact boundary under audit: the atlas §10A register + the `board_space` atlas row + the `specs/README.md` index + the `draughts_lite` docs, all gated on the full per-game acceptance pipeline (`cargo test` / `replay-check` / `fixture-check` / `rule-coverage` / benchmarks / web smoke) passing across `three_marks`, `column_four`, `directional_flip`, `draughts_lite`, and `race_to_n`. The debt-status flip MUST NOT precede that evidence (spec §15: "Do not mark debt closed before code and tests prove it").
4. FOUNDATIONS §11 (promoted primitives adopted by all matching games; replay/hash/traces deterministic; views viewer-safe), §4 (debt closure), and the §12 stop conditions ("a promoted primitive leaves matching games un-migrated …"; "a new mechanic-ladder gate proceeds while promotion debt is still open") motivate this ticket: it is the recorded closure gate that clears both conditions before Gate 8.
5. Deterministic replay/hash & no-leak firewall: the acceptance pipeline is the enforcement surface. It proves every affected game's golden traces, replay hashes, serialization order, visibility/no-leak tests, and bot legality are byte-identical/intact after the retrofits (FOUNDATIONS §11). The implementation summary must state "Golden traces changed: no" (spec §16); a non-`no` answer reopens the gate and blocks closure rather than triggering a trace update here.
6. Mismatch + correction: none at authoring. Note the §15 governance/foundation-doc back-port language already landed in commit `0286f9f` and is NOT re-edited here — only the atlas debt *status*, the index *status*, and the `draughts_lite` per-game docs change.

## Architecture Check

1. A single trailing closure ticket gives one auditable place where "all evidence passed → debt closed → index Done" is recorded atomically, so the atlas, the index, and the exemplar ledger never disagree about closure state. Splitting the status flips across the retrofit tickets would let the register show "closed" while a sibling game still failed.
2. No backwards-compatibility aliasing/shim — verification + governance bookkeeping only; no production logic is added or aliased.
3. `engine-core` and `game-stdlib` are untouched (no code change). The atlas update records that the already-earned primitive is now adopted by all matching games (§4); no new promotion occurs.

## Verification Layers

1. All affected games pass natively → `cargo test --workspace` (or per-game `cargo test -p {three_marks,column_four,directional_flip,draughts_lite,race_to_n}`).
2. Deterministic replay/hash preserved across all games → `cargo run -p replay-check -- --game <g> --all` for each affected game (golden traces unchanged, per `docs/TESTING-REPLAY-BENCHMARKING.md`).
3. Static-data + rule-coverage intact → `cargo run -p fixture-check` and `cargo run -p rule-coverage` per affected game.
4. Benchmarks within thresholds → `cargo bench` smoke for affected games (thresholds unchanged, spec §16).
5. Web build + viewer-safe smoke + no-leak/a11y → `npm --prefix apps/web run build` and `npm --prefix apps/web run smoke:ui` (covers `three_marks`/`column_four`/`directional_flip`/`draughts_lite` and the a11y/no-leak smoke).
6. `engine-core` noun-free; boundary intact → `bash scripts/boundary-check.sh`.
7. Atlas debt closed, index flipped, ledger updated → docs grep-proof (no `promotion-debt-open` for `board_space`; `specs/README.md` Gate 7.1 row reads `Done`) + `node scripts/check-doc-links.mjs`.

## What to Change

### 1. Run the full acceptance pipeline (no code edits)

Execute every command in §Test Plan across `three_marks`, `column_four`, `directional_flip`, `draughts_lite`, and `race_to_n`. Confirm "Golden traces changed: no". If any trace/hash drifted, STOP and reopen the relevant retrofit ticket — do not update traces or close the debt.

### 2. Update `draughts_lite` exemplar docs

- `games/draughts_lite/docs/MECHANICS.md`: keep it as the `board_space` exemplar; ensure it does not imply draughts movement/capture was promoted.
- `games/draughts_lite/docs/PRIMITIVE-PRESSURE-LEDGER.md`: replace the "no forced back-port under Gate 7" language with a clarification that Gate 7.1 closed the board-space back-port debt (`three_marks`/`column_four`/`directional_flip` conformed; `race_to_n` audited not applicable), while draughts movement/capture/promotion stays local.

### 3. Close the promotion debt in the atlas

In `docs/MECHANIC-ATLAS.md`: move the `game-stdlib::board_space` row from `promotion-debt-open` to fully `promoted primitive`, and clear the §10A open-promotion-debt register entry (all must-retrofit games conformed; `race_to_n` audited not applicable). Do this only after step 1's evidence passes.

### 4. Flip the spec index status

In `specs/README.md`: change the Gate 7.1 row status from `Planned` to `Done` (admission rule: only after exit criteria pass with evidence).

## Files to Touch

- `docs/MECHANIC-ATLAS.md` (modify)
- `specs/README.md` (modify)
- `games/draughts_lite/docs/MECHANICS.md` (modify)
- `games/draughts_lite/docs/PRIMITIVE-PRESSURE-LEDGER.md` (modify)

## Out of Scope

- Any production code change in `three_marks`/`column_four`/`directional_flip` (owned by 001/002/003) or in `draughts_lite` (no code change — regression only).
- Re-editing the foundation/governance docs (`FOUNDATIONS.md`, `ARCHITECTURE.md`, `ENGINE-GAME-DATA-BOUNDARY.md`, `OFFICIAL-GAME-CONTRACT.md`, `AGENT-DISCIPLINE.md`, `ROADMAP.md`, `TESTING-REPLAY-BENCHMARKING.md`) — that back-port language already landed in commit `0286f9f` (spec §15).
- Updating any golden trace to absorb drift (spec §16, §18) — a drift reopens a retrofit ticket instead.
- Closing the debt or flipping the index before the acceptance pipeline passes (spec §15).

## Acceptance Criteria

### Tests That Must Pass

1. `cargo test --workspace` green (all affected games).
2. `cargo run -p replay-check -- --game three_marks --all && … --game column_four --all && … --game directional_flip --all && … --game draughts_lite --all && … --game race_to_n --all` — all golden traces pass unchanged.
3. `npm --prefix apps/web run build && npm --prefix apps/web run smoke:ui` — web build + viewer-safe smoke (incl. a11y/no-leak) green.
4. `grep -n "promotion-debt-open" docs/MECHANIC-ATLAS.md` shows no `board_space` debt remains, and `specs/README.md` Gate 7.1 row reads `Done`.

### Invariants

1. The atlas open promotion-debt register has no open `board_space` debt, and the `specs/README.md` index reflects Gate 7.1 `Done` only after exit evidence passed.
2. No golden trace, replay hash, serialization order, view payload, or bot legality changed for any affected game (the gate's "preserve by default" contract, §8.2).

## Test Plan

### New/Modified Tests

1. `None — capstone verification + governance closure ticket; it adds no production logic and exercises the existing per-game test/replay/smoke pipeline composed by GAT71BOASPA-001..004. Doc changes are grep-verified.`

### Commands

1. `cargo test --workspace && bash scripts/boundary-check.sh && cargo clippy --workspace --all-targets -- -D warnings`
2. `for g in three_marks column_four directional_flip draughts_lite race_to_n; do cargo run -p replay-check -- --game $g --all && cargo run -p fixture-check -- --game $g && cargo run -p rule-coverage -- --game $g; done`
3. `npm --prefix apps/web run build && npm --prefix apps/web run smoke:ui && node scripts/check-doc-links.mjs`

## Outcome

Completed: 2026-06-07

What changed:
- `docs/MECHANIC-ATLAS.md` now marks `game-stdlib::board_space` as a promoted
  primitive with no open board-space promotion debt.
- `specs/README.md` marks Gate 7.1 as `Done`.
- `games/draughts_lite/docs/MECHANICS.md` and
  `games/draughts_lite/docs/PRIMITIVE-PRESSURE-LEDGER.md` record Gate 7.1
  closure while keeping draughts movement, capture, promotion, forced
  continuation, effects, UI, and bot policy local.

Deviations from original plan:
- None.

Verification results:
- `cargo test --workspace`
- `bash scripts/boundary-check.sh`
- `cargo clippy --workspace --all-targets -- -D warnings`
- `cargo run -p replay-check -- --game three_marks --all`
- `cargo run -p replay-check -- --game column_four --all`
- `cargo run -p replay-check -- --game directional_flip --all`
- `cargo run -p replay-check -- --game draughts_lite --all`
- `cargo run -p replay-check -- --game race_to_n --all`
- `cargo run -p fixture-check -- --game three_marks`
- `cargo run -p fixture-check -- --game column_four`
- `cargo run -p fixture-check -- --game directional_flip`
- `cargo run -p fixture-check -- --game draughts_lite`
- `cargo run -p fixture-check -- --game race_to_n`
- `cargo run -p rule-coverage -- --game three_marks`
- `cargo run -p rule-coverage -- --game column_four`
- `cargo run -p rule-coverage -- --game directional_flip`
- `cargo run -p rule-coverage -- --game draughts_lite`
- `cargo run -p rule-coverage -- --game race_to_n`
- `npm --prefix apps/web run build`
- `npm --prefix apps/web run smoke:ui`
- `node scripts/check-doc-links.mjs`
- `grep -n "promotion-debt-open" docs/MECHANIC-ATLAS.md` shows only the status
  definition, not a `board_space` debt row.
- `grep -n "Gate 7.1" specs/README.md` shows `Done`.
- Golden traces changed: no.

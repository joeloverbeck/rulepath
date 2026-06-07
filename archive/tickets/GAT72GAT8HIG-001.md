# GAT72GAT8HIG-001: Gate 7.2 orientation hygiene + gate tracker

**Status**: COMPLETED
**Priority**: HIGH
**Effort**: Small
**Engine Changes**: None — orientation/docs only (`README.md`, `progress.md`, `AGENTS.md`, `CLAUDE.md`, `specs/README.md`, `docs/MECHANIC-ATLAS.md`)
**Deps**: None

## Problem

After Gate 7.1, several repository-orientation surfaces are stale and would
misdirect future agents before the Gate 8 hidden-information work begins.
`README.md` still describes Gate 5 as the latest completed state; `progress.md`
records Gate 3/Gate 5-era progress; `AGENTS.md` and `CLAUDE.md` hardcode
`race_to_n` as the current verification game; `specs/README.md` has no Gate 7.2
interlock row and lists Gate 8 as `Not started` / `not yet specced`. Gate 8 may
not begin until this orientation drift is removed (spec §1.3, §6.1, §11).

## Assumption Reassessment (2026-06-07)

1. Verified the stale surfaces against current code: `specs/README.md:27-43`
   marks Gates 0–7.1 `Done` and Gate 8 `Not started`; `CLAUDE.md` "Commands"
   block names `race_to_n` as "current game"; `AGENTS.md` carries the same
   race_to_n-only verification cue. `README.md`/`progress.md` predate Gate 6.
2. Verified against specs/docs: `docs/ROADMAP.md:20,67` defines Gate 8 as the
   Stage 6 chance/hidden-information proof (`high_card_duel`/`blackjack_lite`);
   `specs/gate-7-2-and-gate-8-high-card-duel-hidden-info-chance-proof.md` §4.1
   and §6.1 enumerate the exact orientation edits and exit criteria.
3. Cross-artifact boundary under audit: the gate-tracker consistency contract in
   `specs/README.md` (one row per gate/interlock; Gate 7.1 sits as the "5M"
   maintenance interlock at line 35). Gate 7.2 must follow that same interlock
   convention, and the single combined spec file is referenced by two rows
   (7.2 interlock + Gate 8), per the reassessed spec §4.1 deliverable 1.

## Architecture Check

1. Editing only the proven-stale orientation surfaces (Appendix C "minimal edit
   policy") is cleaner than a broad docs rewrite: it removes misdirection
   without touching foundation law or architecture concepts.
2. No backwards-compatibility shims — these are forward-correcting doc edits.
3. `engine-core` untouched (no mechanic nouns); `game-stdlib` untouched (no
   promotion). `docs/MECHANIC-ATLAS.md` is only *verified* (register already
   empty at line 199; board_space debt closed at line 186) — no new row here.

## Verification Layers

1. Orientation no-longer-stale -> codebase grep-proof: `grep -i "race_to_n" AGENTS.md CLAUDE.md` no longer presents race_to_n as the sole/current verification game; `grep -i "gate 5" README.md` no longer claims Gate 5 is latest.
2. Gate-tracker coherence -> manual review: `specs/README.md` shows a Gate 7.2 interlock row + Gate 8 flipped to `Planned` pointing at the combined spec, plus a `blackjack_lite` continuation checkpoint before Gate 9.
3. Doc-link integrity -> simulation/CLI run: `node scripts/check-doc-links.mjs` passes.
4. FOUNDATIONS alignment -> FOUNDATIONS alignment check: no foundation law rewritten (§12 "agents asked to clean up without bounded scope" stays clear; edits are bounded to Appendix C allowed list).

## What to Change

### 1. Stale orientation files

- `README.md`: replace Gate 5-era status with current orientation — official
  games through Gate 7 / Gate 7.1, `board_space` back-port status, Gate 8 as the
  next chance/hidden-information proof. Do not rewrite project identity.
- `progress.md`: add concise Gate 6 / Gate 7 / Gate 7.1 completion entries (or a
  current-progress pointer consistent with the file's pattern).
- `AGENTS.md` and `CLAUDE.md`: remove "current game: race_to_n" framing; replace
  with verification guidance covering all official game crates or pointing to
  `specs/README.md` for the active gate. Preserve discipline rules verbatim.

### 2. Gate tracker (`specs/README.md`)

- Add a Gate 7.2 maintenance-interlock row (mirroring Gate 7.1's "5M") pointing
  at `specs/gate-7-2-and-gate-8-high-card-duel-hidden-info-chance-proof.md`.
- Flip the Gate 8 row from `Not started` / `not yet specced` to `Planned`,
  pointing at the same spec file.
- Add an explicit `blackjack_lite` continuation checkpoint before Gate 9
  admission (Appendix B wording or a semantic equivalent).

### 3. Mechanic atlas verification only

- Confirm `docs/MECHANIC-ATLAS.md` open promotion-debt register (§10A) remains
  empty and the Gate 7.1 `board_space` debt is recorded closed. No row is added
  here — the `high_card_duel` pressure row is extended later (GAT72GAT8HIG-020).

## Files to Touch

- `README.md` (modify)
- `progress.md` (modify)
- `AGENTS.md` (modify)
- `CLAUDE.md` (modify)
- `specs/README.md` (modify)
- `docs/MECHANIC-ATLAS.md` (modify — verification only; edit only if a stale claim is found)

## Out of Scope

- Any Gate 8 implementation (crate, code, WASM, UI).
- Rewriting foundation law, renaming architecture concepts, reformatting whole docs, or adding aspirational roadmap prose (spec Appendix C "Not allowed").
- Resolving the `blackjack_lite` checkpoint (authored here, resolved by GAT72GAT8HIG-021).

## Acceptance Criteria

### Tests That Must Pass

1. `node scripts/check-doc-links.mjs` — passes (no broken links introduced).
2. `bash scripts/boundary-check.sh` — passes (engine-core stays noun-free; unaffected but confirms no accidental edit).
3. `grep -niE "current game.*race_to_n" AGENTS.md CLAUDE.md` — returns nothing.

### Invariants

1. `specs/README.md` lets a future agent infer Gates 0–7.1 are `Done` and Gate 8 is the next planned gate, with the blackjack checkpoint visible before Gate 9.
2. No foundation/architecture doctrine is altered; only stale status language changes.

## Test Plan

### New/Modified Tests

1. `None — documentation-only ticket; verification is command-based and existing pipeline/test coverage is named in Assumption Reassessment.`

### Commands

1. `node scripts/check-doc-links.mjs`
2. `grep -niE "gate 5" README.md` (manually confirm no "latest completed = Gate 5" claim remains)
3. Doc-link + boundary checks are the correct boundary — this ticket ships no code, so `cargo` suites are unaffected.

## Outcome

Completed: 2026-06-07

What changed:

- Updated `README.md` to identify Gates 0-7.1 as complete, list the five current official games, describe the Gate 7 / Gate 7.1 status, and point Gate 8 at the planned `high_card_duel` hidden-information proof plus the `blackjack_lite` checkpoint.
- Added current Gate 6, Gate 7, and Gate 7.1 progress entries to `progress.md`, with `specs/README.md` identified as the mutable progress source of truth.
- Removed the stale `race_to_n`-as-current-game framing from `AGENTS.md` and `CLAUDE.md`; per-game verification now points agents to the game under change and the active gate tracker.
- Updated `specs/README.md` with a Gate 7.2 maintenance-interlock row, flipped Gate 8 to `Planned` against the combined spec, and added the post-Gate-8 `blackjack_lite` continuation checkpoint before Gate 9.
- Verified `docs/MECHANIC-ATLAS.md` already records Gate 7.1 board-space debt as closed and the open promotion-debt register as empty, so no atlas edit was needed.

Deviations from original plan:

- None.

Verification results:

- `node scripts/check-doc-links.mjs` passed.
- `bash scripts/boundary-check.sh` passed.
- `grep -niE "current game.*race_to_n" AGENTS.md CLAUDE.md` returned no matches.
- `grep -niE "Gate 5 complete|latest completed.*Gate 5|Gate 5.*latest" README.md` returned no matches.

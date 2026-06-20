# GAT151RIVLED-001: Admission, rule contract, and v2 version plan

**Status**: COMPLETED
**Priority**: HIGH
**Effort**: Medium
**Engine Changes**: None (docs-only: authors the implementation-admission + rule-family + version plan; no code, traces, or version bump lands here)
**Deps**: None (gated on Gate 15.1 spec acceptance)

## Problem

Gate 15.1 adds a finite-stack, all-in, and ordered side-pot layer to the already-shipped River Ledger hand. Before any behavior code, the implementation-admission plan must supersede the base game's all-in/side-pot exclusion, define the new stable `RL-*` rule families, identify the existing single-pot / all-in-out-of-scope rule rows that require explicit supersession, and decide the rules/data v2 and v1-replay-import policy. FOUNDATIONS §4/§12 require the contract to resolve before behavior so the v2 cutover is explicit and reviewable rather than discovered mid-implementation.

## Assumption Reassessment (2026-06-20)

1. The rule IDs slated for supersession exist verbatim in `games/river_ledger/docs/RULES.md` + `docs/RULE-COVERAGE.md`: `RL-POT-SINGLE-001`/`-002`, `RL-POT-ALLIN-001`, `RL-VAR-ALLIN-001`, `RL-OOS-ALLIN-001`. Current version is `river-ledger-rules-v1` (`games/river_ledger/src/ids.rs::RULES_VERSION_LABEL`; `data/manifest.toml` `rules_version = 1`, `data_version = 1`).
2. Specs/docs: `docs/ROADMAP.md` §15.1 admits the gate; `archive/specs/gate-15-river-ledger-texas-holdem-base.md` defers all-in/side-pots to this gate; `games/river_ledger/docs/GAME-IMPLEMENTATION-ADMISSION.md` records the base admission this ticket extends.
3. Cross-artifact boundary under audit: the per-game rules/data version contract — `ids.rs` label ↔ `manifest.toml` `rules_version`/`data_version` ↔ `RULES.md` version line ↔ `HOW-TO-PLAY.md` cited formal-rules version, enforced by `scripts/check-player-rules.mjs`.
4. (§11/§13 enforcement surface) This ticket plans the deterministic replay/hash v2 surface but changes no hash and bumps no version; the enforcement surface — stable hashing of stack/reopen/pot state plus v1-replay rejection — lands in GAT151RIVLED-011. Confirm the plan introduces no leakage or nondeterminism path the later surface must undo.
5. (rename/remove blast radius) The four superseded IDs appear only in `RULES.md` + `RULE-COVERAGE.md` (no code/test references those exact IDs). The live doc edits are owned by GAT151RIVLED-019; this ticket records the supersession map in ADMISSION only, which is why no `check-player-rules` red window opens here.

## Architecture Check

1. Authoring the rule/version contract before code makes the v2 cutover explicit and lets every downstream ticket cite a fixed family scheme, versus colliding rule IDs discovered during implementation.
2. No backwards-compatibility shims: supersession is an explicit migration entry, never silent semantic reuse of a legacy ID.
3. `engine-core` is untouched; no `game-stdlib` promotion is decided here (that is GAT151RIVLED-002).

## Verification Layers

1. Rule-family contract complete -> manual review: every superseded ID maps to its v2 replacement family in the ADMISSION supersession table.
2. Version-contract coherence -> FOUNDATIONS alignment check (§13 migration): the plan names the v2 label/manifest/doc surfaces that GAT151RIVLED-011 and -019 must move in lockstep.
3. No premature code/version change -> codebase grep-proof: `rules_version = 1` and `river-ledger-rules-v1` remain unchanged after this ticket.

## What to Change

### 1. Implementation-admission plan

Update `GAME-IMPLEMENTATION-ADMISSION.md`: supersede the base all-in/side-pot exclusion; record option (b) configurable per-seat stacks with the equal 24-unit default; the pure-delta boundary over the shipped base; the game-local (no `game-stdlib`/`engine-core`) decision; and the documented full-unit reopening divergence from external poker authorities.

### 2. Rule-family, version, and supersession map

Define the new stable families per spec §7.2 (`RL-STACK-*`, `RL-ALLIN-*`, `RL-POT-{LAYER,ELIG,RETURN,ALLOC,REMAINDER}-*`, `RL-OUTCOME-*`, `RL-VIS-*`, `RL-BOT-*`, `RL-REPLAY-*`) under the existing `RL-<DOMAIN>-<CONCEPT>-NNN` scheme. Tabulate each superseded legacy ID → replacement family. Decide River Ledger rules/data **v2** and the v1-replay-import policy (reject-with-deterministic-diagnostic default; bounded explicit converter only if separately justified).

## Files to Touch

- `games/river_ledger/docs/GAME-IMPLEMENTATION-ADMISSION.md` (modify)

## Out of Scope

- Editing `RULES.md` / `RULE-COVERAGE.md` prose or bumping the rules/data version (owned by GAT151RIVLED-011 for the version/hash bump and GAT151RIVLED-019 for doc reconciliation).
- Any `src/` behavior change.
- The primitive-pressure promotion decision (GAT151RIVLED-002).

## Acceptance Criteria

### Tests That Must Pass

1. `node scripts/check-doc-links.mjs` — ADMISSION links resolve.
2. `node scripts/check-player-rules.mjs` — still green (no version drift introduced yet).
3. Manual: the ADMISSION supersession table maps all four legacy IDs to v2 replacement families.

### Invariants

1. No code, version, or hash change lands in this ticket.
2. Every new `RL-*` family in the plan is unique and namespaced per the existing `RL-<DOMAIN>-<CONCEPT>-NNN` scheme.

## Test Plan

### New/Modified Tests

1. `None — documentation-only ticket; verification is command-based and existing pipeline/test coverage is named in Assumption Reassessment.`

### Commands

1. `node scripts/check-doc-links.mjs`
2. `node scripts/check-player-rules.mjs`
3. A narrower doc-link/player-rules boundary is correct here: no Rust changes land, so `cargo` gates are not exercised by this ticket.

## Outcome

Completed: 2026-06-20

What changed:

- Updated `games/river_ledger/docs/GAME-IMPLEMENTATION-ADMISSION.md` with the Gate 15.1 v2 admission delta, including configurable per-seat stacks with the equal 24-unit default, full-unit reopening, side-pot/accounting scope, game-local boundary, and v1 replay rejection plan.
- Added the planned `river-ledger-rules-v2` / `rules_version = 2` / `data_version = 2` cutover note without changing current code or manifest versions.
- Added the v2 rule-family plan and explicit supersession map for `RL-POT-SINGLE-001`, `RL-POT-SINGLE-002`, `RL-POT-ALLIN-001`, `RL-VAR-ALLIN-001`, and `RL-OOS-ALLIN-001`.

Deviations:

- None. This ticket remained documentation-only; `RULES.md`, `RULE-COVERAGE.md`, code, manifests, hashes, and traces remain unchanged for their later owning tickets.

Verification:

- `node scripts/check-doc-links.mjs` passed (`Checked 27 markdown files`).
- `node scripts/check-player-rules.mjs` passed (`player-rules check passed -- 15 catalog games validated`).

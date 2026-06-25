# GAT18BLAPACSPA-018: atlas, scaffolding register, and forward-v1 CI receipt closeout

**Status**: COMPLETED
**Priority**: HIGH
**Effort**: Medium
**Engine Changes**: Yes (CI evidence receipt) — `ci/scaffolding-audits.json`, `docs/MECHANIC-ATLAS.md`, `docs/MECHANICAL-SCAFFOLDING-REGISTER.md`, `games/blackglass_pact/docs/PRIMITIVE-PRESSURE-LEDGER.md`
**Deps**: GAT18BLAPACSPA-011, GAT18BLAPACSPA-013, GAT18BLAPACSPA-016, GAT18BLAPACSPA-017

## Problem

Close the primitive-pressure and post-build forward-v1 governance: update `docs/MECHANIC-ATLAS.md` (trick-helper reuse/conformance rows, numeric-contract second-use keep-local row, partnership/team first-use `local-only` row, §9A interlock, §10A truthfully empty), the `docs/MECHANICAL-SCAFFOLDING-REGISTER.md` post-build evidence, the game ledger, and the machine `ci/scaffolding-audits.json` `forward-v1` receipt that `check-scaffolding-governance.mjs` validates — the gate-1 enforcement of the first forward-v1 audit (spec §8.5, §10.3–§10.4, §7.10, candidate task `GAT18-BLAPAC-014`).

## Assumption Reassessment (2026-06-25)

1. `ci/scaffolding-audits.json` carries `legacy_8c_games` + per-game records with `coverage/evidence_paths/register_entries_reviewed/register_decisions/disposition/prior_matching_games/compatibility`; no `blackglass_pact` record exists yet. This ticket adds exactly one record with `coverage: "forward-v1"` (not `legacy-8c-covered`).
2. `docs/MECHANIC-ATLAS.md` §10A reads "Current debt: None" (verified `:260`) and records the Gate 17 trick-helper promotion; this ticket appends Gate 18 reuse/second-use/first-use rows without creating promotion debt. `MSC-8C-001…010` are the register entries the receipt reviews.
3. Cross-artifact boundary under audit: the atlas, register, game ledger (`PRIMITIVE-PRESSURE-LEDGER.md`), `GAME-EVIDENCE.md`, and `ci/scaffolding-audits.json` must all agree (spec §7.10); the receipt must pass schema + the checker's test suite.
4. FOUNDATIONS §4 (`game-stdlib` earned) / §12 (stop conditions) motivate this ticket: it confirms reuse-only, register-new for any new behavior-free shape, queue-or-dispose for any prior match, and a truthful `disposition` — gate closeout is blocked until these pass.

## Architecture Check

1. A single closeout that reconciles atlas + register + ledger + machine receipt (vs. scattering them) makes the forward-v1 audit auditable in one diff and lets the checker validate consistency.
2. No shims; the receipt is generated against the live checker schema, not copied from the spec; `disposition` is truth-based (expected `reuse-only`/`no-new-scaffolding`).
3. `engine-core` untouched; no `game-stdlib` promotion; the reused trick helpers stay unchanged (no third-use gate).

## Verification Layers

1. `forward-v1` receipt valid and consistent with code/register/evidence -> `node scripts/check-scaffolding-governance.mjs` + `node --test scripts/check-scaffolding-governance.test.mjs`.
2. Atlas rows (reuse/second-use/first-use, §9A, empty §10A) -> grep-proof in `MECHANIC-ATLAS.md`.
3. Register + ledger agree on behavioral-vs-scaffolding classification -> manual cross-check + `node scripts/check-doc-links.mjs`.

## What to Change

### 1. Mechanic atlas

`docs/MECHANIC-ATLAS.md`: append Gate 18 reuse/conformance notes to the follow-suit + comparator rows; record the Vow Tide↔Blackglass numeric second-use keep-local decision + next trigger; add the partnership/team first-use `local-only` row; record the §9A interlock; keep §10A empty.

### 2. Scaffolding register + ledger

`docs/MECHANICAL-SCAFFOLDING-REGISTER.md`: post-build reuse evidence for the applicable C-01…C-10 entries, register-new for any newly invented behavior-free shape, and a named tracker unit or accepted no-unit disposition for any prior match; finalize `games/blackglass_pact/docs/PRIMITIVE-PRESSURE-LEDGER.md`.

### 3. forward-v1 CI receipt

`ci/scaffolding-audits.json`: add the `blackglass_pact` `forward-v1` record (evidence paths, `MSC-8C-001…010` reviewed, register decisions, truth-based disposition, prior-matching-games, known-signal dispositions, `compatibility` all `none`/`migration_authority: "none"`).

## Files to Touch

- `ci/scaffolding-audits.json` (modify)
- `docs/MECHANIC-ATLAS.md` (modify)
- `docs/MECHANICAL-SCAFFOLDING-REGISTER.md` (modify)
- `games/blackglass_pact/docs/PRIMITIVE-PRESSURE-LEDGER.md` (modify; created by GAT18BLAPACSPA-002)

## Out of Scope

- Exit-criteria command suite run + `specs/README.md` `Done` flip (GAT18BLAPACSPA-019).
- Any helper promotion or behavior change (reuse-only closeout).

## Acceptance Criteria

### Tests That Must Pass

1. `node scripts/check-scaffolding-governance.mjs` (forward-v1 receipt passes).
2. `node --test scripts/check-scaffolding-governance.test.mjs` (checker suite green).
3. Atlas/register/ledger/`GAME-EVIDENCE.md`/`ci/scaffolding-audits.json` agree (manual + grep cross-check).

### Invariants

1. The receipt carries `coverage: "forward-v1"` (never `legacy-8c-covered`); `compatibility` is all `none` with `migration_authority: "none"`.
2. §10A stays empty; no promotion debt is created for the local team or second-use bid shapes.

## Test Plan

### New/Modified Tests

1. `ci/scaffolding-audits.json` — `blackglass_pact` forward-v1 record (validated by the checker suite).
2. `docs/MECHANIC-ATLAS.md` — Gate 18 reuse/second-use/first-use rows.
3. `docs/MECHANICAL-SCAFFOLDING-REGISTER.md` — post-build evidence + dispositions.

### Commands

1. `node scripts/check-scaffolding-governance.mjs`
2. `node --test scripts/check-scaffolding-governance.test.mjs && node scripts/check-doc-links.mjs`
3. The governance checker + its test suite are the correct boundary; the receipt is the gate-1 enforcement surface.

## Outcome

Completed: 2026-06-25

Added the `blackglass_pact` `forward-v1` record to
`ci/scaffolding-audits.json` with all `MSC-8C-001` through `MSC-8C-010`
entries reviewed, no new register decisions, no follow-on unit, compatibility
all `none`, and `migration_authority: "none"`.

Updated `docs/MECHANIC-ATLAS.md` to record Gate 18 Blackglass Pact reuse of the
promoted follow-suit and trick-comparator helpers, the Vow Tide to Blackglass
numeric-contract second-use keep-local decision, and the first-use fixed
competitive partnership/team scoring local-only row. Section 10A remains empty.

Updated `docs/MECHANICAL-SCAFFOLDING-REGISTER.md` with the Gate 18 forward-v1
post-build receipt and finalized
`games/blackglass_pact/docs/PRIMITIVE-PRESSURE-LEDGER.md` with post-build
evidence. The atlas, register, ledger, game evidence paths, and CI receipt now
agree that no helper was broadened and no promotion debt was created.

Verification:

- `node scripts/check-scaffolding-governance.mjs` passed.
- `node --test scripts/check-scaffolding-governance.test.mjs` passed.
- `node scripts/check-doc-links.mjs` passed.
- `git diff --check` passed.
- Grep truthing confirmed `blackglass_pact`, Gate 18 forward-v1 receipt, numeric
  contract, partnership/team, and `Current debt: _None_` entries across the
  atlas/register/ledger/receipt surfaces.

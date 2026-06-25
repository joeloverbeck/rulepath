# GAT18BLAPACSPA-002: forward-v1 pre-code reuse-first audit and implementation admission

**Status**: PENDING
**Priority**: HIGH
**Effort**: Medium
**Engine Changes**: None — governance/admission docs (`games/blackglass_pact/docs/{MECHANICS,GAME-EVIDENCE,GAME-IMPLEMENTATION-ADMISSION,PRIMITIVE-PRESSURE-LEDGER}.md`)
**Deps**: GAT18BLAPACSPA-001

## Problem

Gate 18 is the first game admitted under ADR 0008's `forward-v1` standing obligation (Unit 8F, `Done` 2026-06-25). FOUNDATIONS §11/§12 block serious implementation until a completed mechanical-scaffolding reuse-first audit exists. This ticket authors the pre-code C-01…C-10 audit, the lawful-shared-home review, the primitive-pressure ledger seed (trick-helper reuse, numeric second-use, partnership first-use), and the signed implementation-admission receipt — the gate that unblocks GAT18BLAPACSPA-003+ (spec §2.6, §4.2, §8.5, candidate task `GAT18-BLAPAC-002`).

## Assumption Reassessment (2026-06-25)

1. `docs/MECHANICAL-SCAFFOLDING-REGISTER.md` defines `MSC-8C-001`…`MSC-8C-010` (verified `:152`–`:870`); the audit maps C-01…C-10 to those exact IDs per spec §8.5.
2. `ci/scaffolding-audits.json` carries a `forward-v1`-capable schema (`coverage`, `register_entries_reviewed`, `register_decisions`, `disposition`, `prior_matching_games`, `compatibility`) but no `blackglass_pact` record yet — the machine receipt is authored at closeout (GAT18BLAPACSPA-018), not here; this ticket is the **doc-side** audit.
3. Cross-artifact boundary under audit: the audit narrative (here) and the CI receipt (018) must agree; the lawful homes are `game-stdlib::trick_taking` (reuse), canonical seat grammar, `game-test-support` dev-only, and shared `wasm-api`/`apps/web` scaffolding.
4. FOUNDATIONS §4 (`game-stdlib` earned) and §12 (stop conditions) motivate this ticket: it must confirm reuse-first, register-new on first use, and queue-or-dispose — the admission stays blocked if any C-row is unresolved.
5. The audit touches the §4 third-use framing and the no-leak/determinism enforcement surfaces it plans: it must state that the promoted trick helpers are reused **unchanged** (no third-use hard gate fires) and that the planned blind/deal/visibility work introduces no leakage or nondeterminism the later tickets would have to undo. The actual enforcement lands in 004/008; this ticket records the admission predicate.

## Architecture Check

1. A pre-code admission gate (vs. retrospective paperwork) is what ADR 0008 forward-v1 requires; it prevents scaffolding work from silently becoming a hidden game framework.
2. No shims; first artifacts of a new game's governance set.
3. Confirms `engine-core` stays noun-free and that no `game-stdlib` promotion is proposed (reuse-only of already-promoted helpers).

## Verification Layers

1. C-01…C-10 each have a reuse/new/disposition decision -> manual review against `MSC-8C-001…010` in the register.
2. Admission predicate (rules+sources ready, boundary clear, no active stop condition) -> manual review of `GAME-IMPLEMENTATION-ADMISSION.md` against spec §4.2.
3. Primitive-pressure entries (trick reuse, numeric keep-local, partnership first-use local-only) -> grep-proof the three decisions appear in `PRIMITIVE-PRESSURE-LEDGER.md`.

## What to Change

### 1. `games/blackglass_pact/docs/MECHANICS.md`

Full mechanic inventory + the **pre-code C-01…C-10 reuse-first audit table** (spec §8.5) and the lawful-shared-home review; mark expected disposition `reuse-only` as an expectation to be confirmed at build.

### 2. `games/blackglass_pact/docs/PRIMITIVE-PRESSURE-LEDGER.md`

Trick-helper reuse/conformance note; Vow Tide↔Blackglass numeric-contract second-use comparison with keep-local decision and next trigger; partnership/team first-use `local-only` entry.

### 3. `games/blackglass_pact/docs/GAME-IMPLEMENTATION-ADMISSION.md` and `GAME-EVIDENCE.md`

Signed admission receipt (no active stop condition; required evidence profiles named) and the initialized `GAME-EVIDENCE.md` status/artifact-link scaffold (completion profile `n-seat-hidden-information-release-candidate`).

## Files to Touch

- `games/blackglass_pact/docs/MECHANICS.md` (new)
- `games/blackglass_pact/docs/PRIMITIVE-PRESSURE-LEDGER.md` (new)
- `games/blackglass_pact/docs/GAME-IMPLEMENTATION-ADMISSION.md` (new)
- `games/blackglass_pact/docs/GAME-EVIDENCE.md` (new)

## Out of Scope

- The machine `ci/scaffolding-audits.json` forward-v1 receipt + `check-scaffolding-governance` pass (GAT18BLAPACSPA-018).
- Any Rust code (GAT18BLAPACSPA-003+).
- Repo-level `docs/MECHANIC-ATLAS.md` / register edits (GAT18BLAPACSPA-018).

## Acceptance Criteria

### Tests That Must Pass

1. `MECHANICS.md` contains a C-01…C-10 row each citing `MSC-8C-00N`/`MSC-8C-010`.
2. `PRIMITIVE-PRESSURE-LEDGER.md` records the three decisions (trick reuse, numeric keep-local, partnership first-use local-only).
3. `node scripts/check-doc-links.mjs` passes.

### Invariants

1. Admission stays blocked until every C-row is resolved and no stop condition is active.
2. No promotion is claimed for the reused trick helpers (no third-use hard gate).

## Test Plan

### New/Modified Tests

1. `None — governance/admission documentation; the machine receipt + checker test land in GAT18BLAPACSPA-018.`

### Commands

1. `grep -oE "MSC-8C-0(0[1-9]|10)" games/blackglass_pact/docs/MECHANICS.md | sort -u`
2. `node scripts/check-doc-links.mjs`
3. Doc-side admission ticket — the CI receipt and `check-scaffolding-governance.mjs` are exercised at the closeout ticket, the correct boundary for the machine-validated artifact.

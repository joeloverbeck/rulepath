# PREGAT18REUDOC-012: Author templates/GAME-EVIDENCE.md + completion profiles in templates/README.md

**Status**: COMPLETED
**Priority**: HIGH
**Effort**: Large
**Engine Changes**: None — docs-only (new `templates/GAME-EVIDENCE.md`, `templates/README.md`)
**Deps**: None

## Problem

Per-game evidence (no-leak matrices, IP receipts, benchmark/strategy declarations) is duplicated across five templates. The consolidation needs a single canonical machine-friendly conformance receipt (`GAME-EVIDENCE.md`) and a completion-profile system in `templates/README.md` before any domain template can be slimmed into it. This ticket defines the migration target the slimming tickets (017–021) depend on.

## Assumption Reassessment (2026-06-22)

1. Verified `templates/GAME-EVIDENCE.md` does not exist, and `templates/README.md` carries a "Recommended lifecycle order" but **no** completion-profile system (confirmed via `/reassess-spec` this session; spec D5 / WB8).
2. Verified against spec D5: the receipt is a status/links-only conformance record with game/rules/data/trace versions, completion profile, supported seats/variants, source/IP receipt, rule-coverage summary, named trace profiles, public + per-seat viewer matrix, replay/hash compatibility, benchmark workload IDs, bot levels/policy IDs, mechanic + scaffolding-register decisions, release state, blockers, and exact artifact links — **no duplicated domain prose**.
3. Cross-artifact boundary under audit: `GAME-EVIDENCE.md` is the migration target for the template-slimming tickets (017–021, which `Deps: 012`); its "named trace profiles" field aligns with the profile names defined in `EVIDENCE-FIXTURE-CONTRACT.md` (ticket 007) — referenced by name, not a build dependency (this is a template). Spec WB8 annotates `Deps: WB1`, but no structural dependency on `docs/README.md` authority hygiene (ticket 001) exists, so `Deps` is dropped to `None` per the advisory-WB-annotation rule (divergence surfaced in Step 4/6).
4. FOUNDATIONS §5 + §11 motivate the receipt: restating the invariants — `GAME-EVIDENCE.md` carries status/links, **not** rule data (§5 static-data-is-not-behavior), and completion profiles carry explicit not-applicable reasons but **never waive a §11 invariant or §12 stop condition**.
5. Substrate for a deferred enforcement surface: the machine-checkable fields feed a future `GAME-EVIDENCE` checker (deferred to the Part C / tooling unit per spec Assumption A5). Name that deferred surface and confirm the receipt introduces no leak path — it records status/links only, never hidden information — so the later checker has nothing to undo.

## Architecture Check

1. A single canonical receipt + an explicit completion-profile system is the consolidation's foundation; it is cleaner than the current duplication of evidence blocks across five templates.
2. No backwards-compatibility shims; the receipt is the new canonical surface, not an alias over the old blocks.
3. `engine-core` (§3) / `game-stdlib` (§4) untouched; the receipt is typed content/status (§5), not behavior.

## Verification Layers

1. `templates/GAME-EVIDENCE.md` exists with the full field set (status/links only) -> codebase grep-proof.
2. `templates/README.md` defines completion profiles with explicit not-applicable reasons -> grep.
3. Profiles never waive a §11 invariant or §12 stop condition -> FOUNDATIONS alignment check.
4. No duplicated domain prose in the receipt (status/links only) -> manual review.
5. Links resolve -> `node scripts/check-doc-links.mjs`.

## What to Change

### 1. Author the receipt

Create `templates/GAME-EVIDENCE.md` with the D5 field set (versions, completion profile, seats/variants, source/IP receipt, rule-coverage summary, named trace profiles, public + per-seat viewer matrix, replay/hash compatibility, benchmark workload IDs, bot levels/policy IDs, mechanic + scaffolding-register decisions, release state, blockers, artifact links). Status + links only.

### 2. Completion profiles in templates/README.md

Add the completion-profile system + lifecycle, with explicit not-applicable reasons, asserting that foundation invariants always apply regardless of profile.

## Files to Touch

- `templates/GAME-EVIDENCE.md` (new)
- `templates/README.md` (modify)

## Out of Scope

- Slimming the domain templates into the receipt (tickets 017–021).
- Implementing the machine checker for `GAME-EVIDENCE` fields (deferred per spec Assumption A5).

## Acceptance Criteria

### Tests That Must Pass

1. `test -f templates/GAME-EVIDENCE.md` and `grep -niE "completion profile|viewer matrix|artifact link" templates/GAME-EVIDENCE.md` returns the field set.
2. `grep -niE "completion profile|not.applicable" templates/README.md` returns the profile system.
3. `node scripts/check-doc-links.mjs` passes.

### Invariants

1. The receipt carries status/links only — no rule data (§5) and no duplicated domain prose.
2. No completion profile waives a §11 acceptance invariant or §12 stop condition.

## Test Plan

### New/Modified Tests

1. `None — documentation-only ticket; verification is command-based (field grep, profile grep, link check) and the §5/§11 stance is a FOUNDATIONS alignment check named in Assumption Reassessment.`

### Commands

1. `grep -niE "completion profile|viewer matrix|benchmark workload|bot level|scaffolding register" templates/GAME-EVIDENCE.md`
2. `node scripts/check-doc-links.mjs`
3. The field grep + link check is the correct boundary; the checker that consumes these fields is deferred (A5).

## Outcome

Completed: 2026-06-22

Created `templates/GAME-EVIDENCE.md` as a status/rationale/artifact-link
receipt for official-game conformance. It records versions, completion profile,
supported seats and variants, source/IP receipt, rule-coverage summary, named
evidence fixture profiles, public and per-seat viewer matrix, replay/hash
compatibility, benchmark workload IDs, bot levels/policy IDs, mechanic and
mechanical-scaffolding decisions, release state, blockers, and exact artifact
links. The receipt explicitly forbids duplicated domain prose, hidden
information, rule data, and procedural behavior.

Updated `templates/README.md` with a completion-profile system, lifecycle
placement for `GAME-EVIDENCE.md`, a template-index row, and usage rules requiring
explicit `not applicable: <rationale>` entries. The README states that
completion profiles never waive `docs/FOUNDATIONS.md` §11 invariants or §12 stop
conditions.

Verification:

- `test -f templates/GAME-EVIDENCE.md` passed.
- `grep -niE "completion profile|viewer matrix|artifact link" templates/GAME-EVIDENCE.md`
  returned the receipt field set.
- `grep -niE "completion profile|not.applicable" templates/README.md` returned
  the completion-profile and not-applicable guidance.
- `grep -niE "completion profile|viewer matrix|benchmark workload|bot level|scaffolding register|source and ip|rule-coverage|named trace profiles|replay and hash|release state|artifact links" templates/GAME-EVIDENCE.md`
  returned the expected D5 field groups.
- `node scripts/check-doc-links.mjs` passed (`Checked 31 markdown files`).

Deviation: none; the future machine checker remains deferred as planned.

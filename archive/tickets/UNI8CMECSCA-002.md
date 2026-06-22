# UNI8CMECSCA-002: Add register entries MSC-8C-001…010 before any helper implementation

**Status**: COMPLETED
**Priority**: HIGH
**Effort**: Medium
**Engine Changes**: None — governance doc (`docs/MECHANICAL-SCAFFOLDING-REGISTER.md`)
**Deps**: UNI8CMECSCA-001

## Problem

ADR 0008 requires every mechanical-scaffolding candidate to be recorded in the register **before** extraction, with a complete Entry Schema (not a placeholder). 8C introduces ten candidates (C-01…C-10). This ticket lands all ten full entries `MSC-8C-001`…`MSC-8C-010` up front so each helper ticket can flip its entry from `candidate` to `accepted` only after its evidence passes, and so C-10 is recorded as `rejected / local-only` from the start. Register-first governance is the spec's EC-02 and a §11 acceptance invariant.

## Assumption Reassessment (2026-06-22)

1. `docs/MECHANICAL-SCAFFOLDING-REGISTER.md` exists with an Entry Schema enumerating: owner, semantic risk, exact duplicate sites, explicit exclusions, affected hashes, visibility impact, determinism impact, migration set, acceptance evidence, rejection rationale, next review trigger. The current entries table carries no promoted scaffolding rows (status "Not applicable"; next trigger "Part C candidate review") — confirmed against the register at the reassessed commit.
2. The ten candidates and their required homes/decisions are fixed by spec §4.2 and §4.4: 001 effect-envelope ctors (`engine-core`); 002 canonical seat grammar (`engine-core` + `wasm-api` adapter); 003 seat-count/ring (`game-stdlib::seat`); 004 action-tree encoding/hash v1 (`engine-core`); 005 stable-byte writer v1 (`engine-core`); 006 dev-only test-support crate; 007 no-leak geometry; 008 profile drivers; 009 versioned bounded-index sampler (`engine-core`); 010 behavioral-policy bundle (`rejected / local-only`).
3. Cross-artifact boundary under audit: the register Entry Schema (`docs/MECHANICAL-SCAFFOLDING-REGISTER.md`) and its seam with the behavioral mechanic atlas (`docs/MECHANIC-ATLAS.md` §10A, `Current debt: None`). Entries must not duplicate or relocate the atlas's behavioral Non-Promotion List.
4. FOUNDATIONS §4 + ADR 0008: the scaffolding lane is for behavior-free typed infrastructure only. Each entry restates that its candidate encodes no legality, scoring, reveal/turn policy, or hidden-state semantics; a candidate that does is rerouted to the atlas, not registered here.

## Architecture Check

1. Authoring all ten entries before implementation makes acceptance evidence-gated and auditable, rather than retro-documenting helpers after they land.
2. No backwards-compatibility shim — entries are new rows; the schema is not altered to fit a candidate.
3. `engine-core` and `game-stdlib` code untouched; the register documents homes without moving any symbol. Atlas seam preserved (no behavioral promotion).

## Verification Layers

1. Ten entries present with every Entry Schema field populated (no "TBD" for affected hashes / visibility / migration set / evidence) → codebase grep-proof + manual schema review.
2. `MSC-8C-010` recorded `rejected / local-only` pointing at the existing atlas list → grep-proof.
3. Atlas §10A behavioral debt unchanged (`Current debt: None`) → grep-proof on `docs/MECHANIC-ATLAS.md`.
4. Doc links resolve → `node scripts/check-doc-links.mjs`.

## What to Change

### 1. `docs/MECHANICAL-SCAFFOLDING-REGISTER.md` — ten entries

For each of `MSC-8C-001`…`MSC-8C-010`, add a full Entry Schema row/block. Use `candidate` as the initial decision for 001–009 (each owning helper ticket promotes it to `accepted` with evidence); `MSC-8C-010` is `rejected / local-only` with a next-mechanic-gate review trigger. Populate owner, semantic risk, exact duplicate sites (the pilot call-site paths), explicit exclusions, affected hashes, visibility impact, determinism impact, migration set, acceptance evidence pointer, and rejection rationale (010).

## Files to Touch

- `docs/MECHANICAL-SCAFFOLDING-REGISTER.md` (modify)

## Out of Scope

- Implementing any helper or pilot (owned by the Wave 1–3 tickets).
- Rewriting the behavioral Non-Promotion List or `docs/MECHANIC-ATLAS.md` (C-10 affirms by reference; finalized in UNI8CMECSCA-029).
- Flipping any entry to `accepted` (each helper ticket does that on evidence).

## Acceptance Criteria

### Tests That Must Pass

1. `grep -cE 'MSC-8C-0(0[1-9]|10)' docs/MECHANICAL-SCAFFOLDING-REGISTER.md` reports all ten entry IDs present.
2. `grep -n 'MSC-8C-010' docs/MECHANICAL-SCAFFOLDING-REGISTER.md` shows `rejected` / `local-only`.
3. `node scripts/check-doc-links.mjs` passes.

### Invariants

1. No Entry Schema field reads "TBD"/placeholder for affected hashes, visibility, migration set, or acceptance evidence.
2. `docs/MECHANIC-ATLAS.md` §10A still reads `Current debt: None`.

## Test Plan

### New/Modified Tests

1. `None — documentation-only ticket; verification is command-based and existing pipeline/test coverage is named in Assumption Reassessment.`

### Commands

1. `grep -cE 'MSC-8C-0(0[1-9]|10)' docs/MECHANICAL-SCAFFOLDING-REGISTER.md`
2. `node scripts/check-doc-links.mjs`
3. The register is a governance doc with no compiled surface, so grep + doc-link integrity is the correct verification boundary.

## Outcome

Completed: 2026-06-22

Added complete register blocks for `MSC-8C-001` through `MSC-8C-010` in
`docs/MECHANICAL-SCAFFOLDING-REGISTER.md` before any helper implementation.
Entries `MSC-8C-001` through `MSC-8C-009` start as `candidate` entries owned by
their later implementation tickets. `MSC-8C-010` starts as
`rejected / local-only` because the behavioral-policy bundle belongs in game
crates or the behavioral mechanic atlas, not in the mechanical-scaffolding lane.

Each entry includes owner/status, candidate, semantic risk, proposed home,
production-vs-test home, exact duplicate/pilot sites, behavior exclusions,
affected hashes, visibility impact, determinism impact, migration set,
acceptance evidence, rejection rationale, and next review trigger.

Deviations: none. This ticket changed only the register and did not implement
helpers, alter code, edit `docs/MECHANIC-ATLAS.md`, or flip any candidate to an
accepted/promoted state.

Verification:

- `grep -cE 'MSC-8C-0(0[1-9]|10)' docs/MECHANICAL-SCAFFOLDING-REGISTER.md`
  returned `10`.
- `grep -n 'MSC-8C-010' docs/MECHANICAL-SCAFFOLDING-REGISTER.md` showed the
  `MSC-8C-010` heading with `rejected / local-only`.
- `rg -n 'TBD|placeholder|to be determined|_None_ \| _No promoted' docs/MECHANICAL-SCAFFOLDING-REGISTER.md`
  returned no matches.
- `rg -n 'Current debt: _None_' docs/MECHANIC-ATLAS.md` confirmed atlas open
  promotion debt remains none.
- `node scripts/check-doc-links.mjs` passed (`Checked 31 markdown files`).

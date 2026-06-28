# PLP1RDY-004: FOUNDATIONS constitution amendments — sanctioned private-lane timing

**Status**: COMPLETED
**Priority**: HIGH
**Effort**: Medium
**Engine Changes**: None — constitution doc (`docs/FOUNDATIONS.md`)
**Deps**: PLP1RDY-001, PLP1RDY-002, PLP1RDY-003

## Problem

The sanctioned private lane needs its **timing-only** carve-out written into the
constitution. The spec (WB-2, report `A-01`/`A-03`/`A-04`) lands five verbatim
FOUNDATIONS amendments — §1 carve-out, §10 timing relaxation, §11 invariant
extension, §12 stop conditions, §13 ADR-trigger note — captured verbatim in spec
§4. These are the load-bearing constitutional change; they must land **only after**
ADRs 0010/0011/0012 are accepted, because §13 gates priority-order, rule-like-data,
and private-architecture changes on accepted ADRs.

## Assumption Reassessment (2026-06-28)

1. Current constitution anchors verified against `docs/FOUNDATIONS.md`: the two
   `MUST NOT` paragraphs are L18 and L20 (§4.1 inserts §1.1 after them); the §10
   private-licensed paragraph is L185 (§4.2 amends it); the §11 invariant
   "Private licensed experiments remain isolated and non-architectural." is L232
   (§4.3 extends it); the private-content §12 stops are L265–266 (§4.4 adds after
   them); the private-licensed-influence §13 trigger is L286 (§4.5 adds after it).
   Exact placement/wording may be refined at edit time; the **meaning** is fixed
   by spec §4.
2. Spec source: `specs/private-lane-foundation-readiness.md` §4.1–§4.5 carry the
   verbatim draft blocks; §Exit-criteria item 2 requires Rust-owns-behavior,
   noun-free `engine-core`, no-DSL, no-leak, and determinism invariants to remain
   unchanged.
3. Cross-artifact boundary under audit: `docs/FOUNDATIONS.md` is the repository
   constitution ("Supersede only by accepted ADR"). The three ADRs
   (PLP1RDY-001/002/003) are the authorizing supersession; this ticket gates on
   their `Status: Accepted`.
4. FOUNDATIONS principles under audit (§1 priority order, §10 IP conservatism,
   §13 ADR triggers): all edits are **timing + lane** only. §1.1 permits priority
   item 5 to run parallel with items 1–4 once ADR-authorized; §10 relaxes only
   *timing*, preserving isolation/non-public/non-`engine-core`; no behavior,
   boundary, or leak invariant is weakened.
5. §11/§12 no-leak + isolation invariants touched (amended, not enforced here):
   §4.3 *strengthens* the isolation invariant (public repo gains only generic
   private-free seams; no public file/bundle/doc/CI/trace/WASM carries private
   content); §4.4 *adds* two stop conditions (implementation before accepted ADRs;
   private game in public Cargo members/catalog/CI/submodule). The edit introduces
   no hidden-information leak path and no nondeterminism; enforcement stays with
   the existing `boundary-check.sh` / leak-scan surfaces and the later private-lane
   admission gate.

## Architecture Check

1. Editing the constitution in one focused ticket keeps the highest-risk diff
   (the supersession event) independently reviewable, separate from the routine
   area-doc edits in PLP1RDY-005.
2. No backwards-compatibility shim: the amendments add a lane + timing carve-out;
   they alias nothing and weaken no invariant.
3. `engine-core` stays noun-free (§3): §1.1 and the §11 extension explicitly bar
   private content from `engine-core`; the edit adds no mechanic noun.

## Verification Layers

1. Five amendments present and placed -> codebase grep-proof (each new clause
   grep-matches in `docs/FOUNDATIONS.md` at the §1/§10/§11/§12/§13 anchor).
2. No invariant weakened -> FOUNDATIONS alignment check: §2 behavior-authority,
   §3 noun-free kernel, §5 no-DSL, §11 no-leak/determinism, §8 bot-ban lines are
   unchanged (grep their current text still present).
3. Acceptance precondition -> grep `^Status: Accepted` in
   `docs/adr/0010-*.md`, `0011-*.md`, `0012-*.md` (the §13 note names all three).
4. Cross-artifact: doc-link integrity after the edit -> `node scripts/check-doc-links.mjs`.

## What to Change

### 1. Amend `docs/FOUNDATIONS.md` per spec §4

- §4.1 — insert new §1.1 "Sanctioned private-game lane (timing carve-out)" after
  the two §1 `MUST NOT` paragraphs.
- §4.2 — amend the §10 private-licensed paragraph (early-start allowed only inside
  a sanctioned lane authorized by an accepted ADR; otherwise late-tail).
- §4.3 — extend the §11 "Private licensed experiments remain isolated and
  non-architectural." invariant (generic private-free seams only; no public
  surface carries private content/identifiers).
- §4.4 — add two §12 stop conditions (implementation before authorizing ADRs
  accepted; private game added to public Cargo members/catalog/CI/submodule).
- §4.5 — add the §13 ADR-trigger note naming ADRs 0010/0011/0012.

Use the verbatim draft blocks from spec §4 as the meaning; refine placement/wording
against the live constitution.

## Files to Touch

- `docs/FOUNDATIONS.md` (modify)

## Out of Scope

- ROADMAP / IP-POLICY / AGENT-DISCIPLINE / README / archival edits (PLP1RDY-005).
- ENGINE-GAME-DATA-BOUNDARY typed-registry section (PLP1RDY-006).
- Any change to §2/§3/§5/§8 invariant *semantics* — only §1/§10/§11/§12/§13 timing.

## Acceptance Criteria

### Tests That Must Pass

1. `grep -q 'Sanctioned private-game lane' docs/FOUNDATIONS.md` and the §10/§11/§12/§13 clauses each grep-match.
2. `for a in 0010 0011 0012; do grep -q '^Status: Accepted' docs/adr/$a-*.md; done` — all three ADRs accepted.
3. `node scripts/check-doc-links.mjs && bash scripts/boundary-check.sh` — links intact, `engine-core` noun-free.

### Invariants

1. Rust-owns-behavior, noun-free `engine-core`, no-DSL, no-leak, determinism, and
   v1/v2 bot-ban invariants are textually unchanged.
2. Only §1/§10/§11/§12/§13 **timing + lane** text changes.

## Test Plan

### New/Modified Tests

1. `None — constitution doc; verification is command-based (clause greps + doc-link + boundary-check) and the unchanged-invariant set is named in Assumption Reassessment.`

### Commands

1. `grep -nE 'private-game lane|sanctioned private' docs/FOUNDATIONS.md`
2. `node scripts/check-doc-links.mjs && bash scripts/boundary-check.sh`
3. A narrower command suffices: this is a constitution-text edit, so clause-presence greps plus the doc-link and boundary gates are the correct verification boundary.

## Outcome

Completed: 2026-06-28

Updated `docs/FOUNDATIONS.md` with the five sanctioned private-lane
constitution amendments from `specs/private-lane-foundation-readiness.md`:
the §1.1 timing-only carve-out, the §10 private-stress timing paragraph, the
§11 private-isolation invariant extension, two §12 stop conditions, and the §13
ADR-trigger note naming ADRs 0010, 0011, and 0012.

Deviations from plan: none. The change is constitution text only. It did not
change Rust, web, CI, catalog, fixture, trace, replay, hash, RNG, benchmark, or
private implementation files. Existing unrelated `.claude/skills/*` worktree
changes were left untouched and unstaged.

Verification:

- Clause grep passed for `Sanctioned private-game lane`, sanctioned private
  timing, the generic private-free seam invariant, and ADRs `0010`/`0011`/`0012`
  in `docs/FOUNDATIONS.md`.
- Invariant-presence greps passed for Rust behavior authority, TypeScript
  legality ban, noun-free `engine-core`, no YAML/DSL, deterministic replay/hash
  wording, and the public v1/v2 bot-technique ban.
- `grep -nE '^Status: Accepted' docs/adr/0010-*.md docs/adr/0011-*.md docs/adr/0012-*.md`
  passed for all three prerequisite ADRs.
- `node scripts/check-doc-links.mjs` passed (`Checked 34 markdown files`).
- `bash scripts/boundary-check.sh` passed (`engine-core boundary check passed`;
  `game-test-support dev-only boundary check passed`).

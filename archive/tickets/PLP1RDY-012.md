# PLP1RDY-012: Closeout capstone — doc gates, leak scan, Done-flip

**Status**: COMPLETED
**Priority**: MEDIUM
**Effort**: Small
**Engine Changes**: None — docs/status-only (`specs/README.md`, `specs/private-lane-foundation-readiness.md`)
**Deps**: PLP1RDY-007, PLP1RDY-009, PLP1RDY-010, PLP1RDY-011

## Problem

The readiness unit closes only when the public tree is leak-clean, the doc gates
pass, and the spec's tracker row + Status are flipped to `Done`. The spec (WB-9)
runs `check-doc-links` / `check-catalog-docs` / `boundary-check`, confirms no
public file names the licensed title and no private ID/string entered public
source, then marks the unit complete. This is the verification-only capstone.

## Assumption Reassessment (2026-06-28)

1. Verification surfaces verified present: `scripts/check-doc-links.mjs`,
   `scripts/check-catalog-docs.mjs`, `scripts/boundary-check.sh`. The status
   targets are `specs/README.md` (the `PLP1-RDY` row, currently `Planned`) and
   `specs/private-lane-foundation-readiness.md` (Status field, currently `Planned`).
2. Spec source: `specs/private-lane-foundation-readiness.md` WB-9 + §Exit-criteria
   item 7 + §Acceptance evidence (boundary-check / doc-link / catalog-docs all
   pass; manual public-tree no-leak scan).
3. Cross-artifact boundary under audit: this capstone exercises the prior tickets'
   doc edits end-to-end; it introduces no new production logic. `specs/README.md`
   is shared with PLP1RDY-011 (which adds the tracker section) — this ticket
   `Deps` PLP1RDY-011 and flips the `PLP1-RDY` row after it lands. Its `Deps` leaf
   set (007, 009, 010, 011) transitively covers tickets 001–011.
4. FOUNDATIONS principle under audit (§12 leak-clean exit / §11 no-leak): the
   capstone's gate is that no §12 stop condition was crossed — no private content
   in public files/CI/docs/traces/bundles/WASM, no private work shaping public
   architecture, bounded reviewable scope.
5. §11 no-leak firewall enforcement surface: the manual public-tree scan + the
   `check-catalog-docs` gate are the no-leak proof (no licensed title, private ID,
   card ID, e2e/fixture filename, rules prose, or catalog string in any public
   file). `boundary-check.sh` proves `engine-core` gained no COIN nouns. No code
   ships, so no nondeterminism path is introduced.

## Architecture Check

1. A single verification-only capstone keeps the exit evidence in one reviewable
   place and owns the `Done`-flip, gated on the doc gates passing.
2. No backwards-compatibility shim: the capstone exercises prior tickets; it adds
   no production logic.
3. `engine-core` stays noun-free (§3) — proven by `boundary-check.sh`, not asserted.

## Verification Layers

1. Doc gates green -> `node scripts/check-doc-links.mjs`, `node scripts/check-catalog-docs.mjs`, `bash scripts/boundary-check.sh`.
2. Public tree leak-clean -> manual grep scan: no licensed title / private ID /
   card / e2e / fixture / catalog string in any public file (runbook below).
3. Spec + tracker flipped -> codebase grep-proof: `specs/README.md` `PLP1-RDY` row
   and the spec's Status field both read `Done`.

## What to Change

### 1. Run the closeout gates (runbook)

Run `node scripts/check-doc-links.mjs`, `node scripts/check-catalog-docs.mjs`,
and `bash scripts/boundary-check.sh`; all must pass. Perform the manual
public-tree no-leak scan (no licensed title, private ID/card/e2e/fixture/catalog
string in any public file).

### 2. Flip status to `Done`

After the gates pass, flip the `PLP1-RDY` row in `specs/README.md` from `Planned`
to `Done` (with evidence), and set the Status field in
`specs/private-lane-foundation-readiness.md` to `Done`.

## Files to Touch

- `specs/README.md` (modify; flip `PLP1-RDY` row `Planned` → `Done`)
- `specs/private-lane-foundation-readiness.md` (modify; Status `Planned` → `Done`)

## Out of Scope

- Any new doctrine/template content (tickets 001–011 own it).
- Creating the private repository or authoring the private implementation spec.

## Acceptance Criteria

### Tests That Must Pass

1. `node scripts/check-doc-links.mjs && node scripts/check-catalog-docs.mjs && bash scripts/boundary-check.sh` — all pass.
2. Manual public-tree scan returns zero licensed-title / private-ID / catalog-string matches.
3. `grep -q 'Done' <(grep 'PLP1-RDY' specs/README.md)` and the spec's Status field reads `Done`.

### Invariants

1. No §12 stop condition was crossed; the public tree is leak-clean.
2. `engine-core` gained no COIN nouns (`boundary-check.sh`).

## Test Plan

### New/Modified Tests

1. `None — docs-reconciliation + verification capstone; it reconciles the spec/index status surfaces and exercises the prior tickets' doc-gate suite, adding no test file.`

### Commands

1. `node scripts/check-doc-links.mjs && node scripts/check-catalog-docs.mjs && bash scripts/boundary-check.sh`
2. `grep -rniE '<licensed-title-token>|<private-id-token>' apps/ crates/ docs/ specs/ templates/ games/ tools/ scripts/ ci/ 2>/dev/null` — the manual leak-scan boundary (operator substitutes the opaque tokens being scanned for; expect zero matches).
3. A narrower command suffices: this capstone runs only existing gates + a leak scan and flips status, so the gate trio + leak grep are the correct verification boundary.

## Outcome

Completed the closeout capstone. The required gate trio passed, the focused
public-tree leak scan found and removed public licensed-title/source-token leaks
from the seed reports and references, and the spec/tracker status surfaces were
flipped to `Done`.

Closeout changes:

- renamed the public seed reports to opaque `private-lane-p1-*` filenames;
- scrubbed the licensed title and private source-file tokens from the public
  seed reports and one archived public spec example;
- updated `specs/README.md` and
  `specs/private-lane-foundation-readiness.md` references/status;
- archived this capstone ticket.

Verification:

- `node scripts/check-doc-links.mjs`
- `node scripts/check-catalog-docs.mjs`
- `bash scripts/boundary-check.sh`
- focused public-tree licensed-title/source-token scan, excluding dependency and
  build-output directories, returned zero matches
- focused public-path filename scan for licensed-title/source-token patterns
  returned zero matches
- `grep -nE 'PLP1-RDY|\| Status \|' specs/README.md specs/private-lane-foundation-readiness.md`
- `git diff --check`

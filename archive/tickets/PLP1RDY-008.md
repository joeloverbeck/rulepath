# PLP1RDY-008: Private milestone profiles + AI sourcing/deferral + evidence/admission

**Status**: COMPLETED
**Priority**: MEDIUM
**Effort**: Large
**Engine Changes**: None — contract docs + templates (`docs/OFFICIAL-GAME-CONTRACT.md`, `docs/AI-BOTS.md`, `templates/GAME-AI.md`, `templates/COMPETENT-PLAYER.md`, `templates/BOT-STRATEGY-EVIDENCE-PACK.md`, `templates/GAME-EVIDENCE.md`, `templates/GAME-IMPLEMENTATION-ADMISSION.md`)
**Deps**: PLP1RDY-004, PLP1RDY-005

## Problem

A COIN-scale private game needs documented completion profiles, an explicit
Level-0 bot deferral rule, four-faction AI sourcing limits (no publisher
flowchart transcription), and scaled bot/evidence/admission templates. The spec
bundles this as WB-5 (report `A-05`, `A-10`, `B-09`, `B-10`, `B-11`, `B-13`,
`B-14`). It depends on the constitution carve-out (PLP1RDY-004) and the IP
no-leak doctrine (PLP1RDY-005) being in place.

## Assumption Reassessment (2026-06-28)

1. Targets verified present: `docs/OFFICIAL-GAME-CONTRACT.md`, `docs/AI-BOTS.md`,
   `templates/GAME-AI.md`, `templates/COMPETENT-PLAYER.md`,
   `templates/BOT-STRATEGY-EVIDENCE-PACK.md`, `templates/GAME-EVIDENCE.md`,
   `templates/GAME-IMPLEMENTATION-ADMISSION.md`. The `docs/AI-BOTS.md` edit
   (report `A-10`, four-faction sourcing limits + no-flowchart) was confirmed by
   `/reassess-spec` (2026-06-28) to belong in WB-5 (Issue I1 fix).
2. Spec source: `archive/specs/private-lane-foundation-readiness.md` WB-5 + §Exit-criteria
   item 5 (OFFICIAL-GAME-CONTRACT defines `private-milestone-1-rule-complete`,
   `private-release-candidate`, `public-release-candidate`, with Level-0 bot
   deferral explicit and bounded).
3. Cross-artifact boundary under audit: `docs/OFFICIAL-GAME-CONTRACT.md` is
   **also** edited by PLP1RDY-011 (E-03 M1 capability note); to avoid a shared-file
   conflict, PLP1RDY-011 `Deps` this ticket and lands the M1 note separately. This
   ticket adds the completion-profile + bot-deferral sections.
4. FOUNDATIONS principle under audit (§8 public bots / §11 bot invariants): the
   four-faction sourcing limits ban copying publisher non-player flowcharts/
   priority charts into bot policy/tests/strategy docs; public v1/v2 still exclude
   MCTS/ISMCTS/Monte Carlo/ML/RL.
5. §8/§11 bot-legality invariants touched (documented, not enforced here): the
   templates keep bots using the normal legal action API and allowed views only,
   with no hidden-state access; the Level-0 deferral is bounded by an explicit
   profile, not an open-ended exemption. No leakage or nondeterminism path is
   introduced; enforcement stays with the per-game bot-legality tests a later
   private spec lands.

## Architecture Check

1. Defining completion profiles + a bounded Level-0 deferral in the contract doc
   keeps the private milestone admission explicit and reviewable, rather than ad
   hoc per private spec.
2. No backwards-compatibility shim: the bot-ban and legal-action-API invariants
   are unchanged; the no-flowchart rule reinforces them.
3. `engine-core` untouched (§3); bot policies are game-local/private (§4).

## Verification Layers

1. Three completion profiles + bounded Level-0 deferral present -> codebase
   grep-proof in `docs/OFFICIAL-GAME-CONTRACT.md`
   (`private-milestone-1-rule-complete`, `private-release-candidate`,
   `public-release-candidate`).
2. No-flowchart sourcing rule present -> grep `docs/AI-BOTS.md`.
3. Bot invariants preserved -> FOUNDATIONS alignment check (§8, §11 bot-ban + legal-API).
4. Acceptance precondition -> the constitution carve-out exists (PLP1RDY-004) and
   the three ADRs are `Status: Accepted` (transitive via PLP1RDY-004 `Deps`).

## What to Change

### 1. `docs/OFFICIAL-GAME-CONTRACT.md`

Add the private completion profiles (`private-milestone-1-rule-complete`,
`private-release-candidate`, `public-release-candidate`) and the explicit,
bounded Level-0 bot-deferral rule (report `A-05`).

### 2. `docs/AI-BOTS.md`

Add the four-faction sourcing limits + the no-flowchart-transcription rule
(report `A-10`).

### 3. Templates

`templates/GAME-AI.md` four asymmetric policies + milestone deferral (`B-09`);
`templates/COMPETENT-PLAYER.md` per-faction, non-flowchart (`B-10`);
`templates/BOT-STRATEGY-EVIDENCE-PACK.md` multi-opponent (`B-11`);
`templates/GAME-EVIDENCE.md` private build/source proof (`B-13`);
`templates/GAME-IMPLEMENTATION-ADMISSION.md` private-lane ADR gates (`B-14`).

## Files to Touch

- `docs/OFFICIAL-GAME-CONTRACT.md` (modify; completion-profile + deferral sections — serialized before PLP1RDY-011)
- `docs/AI-BOTS.md` (modify)
- `templates/GAME-AI.md` (modify)
- `templates/COMPETENT-PLAYER.md` (modify)
- `templates/BOT-STRATEGY-EVIDENCE-PACK.md` (modify)
- `templates/GAME-EVIDENCE.md` (modify)
- `templates/GAME-IMPLEMENTATION-ADMISSION.md` (modify)

## Out of Scope

- The OFFICIAL-GAME-CONTRACT M1 capability/non-goals note + required private-spec
  field set (PLP1RDY-011, report `E-03`).
- Any bot code, policy, or evidence pack for the actual private game (private repo).
- The scaling docs/templates (PLP1RDY-009).

## Acceptance Criteria

### Tests That Must Pass

1. `grep -qE 'private-milestone-1-rule-complete' docs/OFFICIAL-GAME-CONTRACT.md` and the other two profile IDs grep-match.
2. `grep -qiE 'flowchart|priority chart' docs/AI-BOTS.md` — the no-flowchart rule is present.
3. `node scripts/check-doc-links.mjs` — no broken links.

### Invariants

1. Public v1/v2 bot ban (MCTS/ISMCTS/Monte Carlo/ML/RL) and legal-action-API rule
   are unchanged; the Level-0 deferral is bounded by an explicit profile.
2. No publisher flowchart text is copied into any public file.

## Test Plan

### New/Modified Tests

1. `None — contract docs + templates; verification is command-based (profile/sourcing greps + doc-link gate) and the bot-invariant set is named in Assumption Reassessment.`

### Commands

1. `grep -nE 'private-(milestone|release)|public-release-candidate|flowchart' docs/OFFICIAL-GAME-CONTRACT.md docs/AI-BOTS.md`
2. `node scripts/check-doc-links.mjs`
3. A narrower command suffices: docs/templates only, so the profile/sourcing greps + doc-link gate are the correct verification boundary.

## Outcome

Completed the private milestone and bot-sourcing doctrine pass. Added
`private-milestone-1-rule-complete`, `private-release-candidate`, and
`public-release-candidate` completion profiles to
`docs/OFFICIAL-GAME-CONTRACT.md`, including the bounded Level 0 bot deferral
rule and closure gate. Added the private asymmetric sourcing and no-flowchart
rule to `docs/AI-BOTS.md`.

Updated the AI, competent-player, bot-strategy, evidence, and admission
templates to carry per-role/faction policy status, multi-opponent inputs,
private source receipts, no-flowchart compliance, Level 0 deferral closure, and
private release/admission evidence fields.

Verification:

- `grep -nE 'private-milestone-1-rule-complete|private-release-candidate|public-release-candidate|flowchart|priority chart' docs/OFFICIAL-GAME-CONTRACT.md docs/AI-BOTS.md`
- `node scripts/check-doc-links.mjs`
- `git diff --check`
- `git status --porcelain -- '*.rs' '*.mjs' '*.yml' '*.toml' '*.json'`

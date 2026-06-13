# PHA0NEXPHAFOU-005: OFFICIAL-GAME-CONTRACT + UI-INTERACTION N-seat & showdown clarifications

**Status**: COMPLETED
**Priority**: HIGH
**Effort**: Medium
**Engine Changes**: None — `docs/OFFICIAL-GAME-CONTRACT.md` + `docs/UI-INTERACTION.md` edits only.
**Deps**: PHA0NEXPHAFOU-002

## Problem

`OFFICIAL-GAME-CONTRACT.md` already requires every player in outcome surfaces, but it lacks explicit N-seat acceptance rows and showdown-specific per-seat rationale. `UI-INTERACTION.md` has strong outcome/no-leak doctrine but no concrete multi-seat panel, turn-order visualization, or showdown rendering requirements. Without these, each Gate 15+ hidden-info game re-derives the per-seat outcome and pairwise no-leak obligations.

## Assumption Reassessment (2026-06-13)

1. Per the realignment report, `apps/web/src/components/OutcomeExplanationPanel.tsx` already renders arbitrary final standings, so the outcome surface is closer to N-seat-ready than setup/mode controls; the implementer confirms that component exists before citing it, and this ticket touches **no** web code (doctrine only).
2. Docs: `docs/OFFICIAL-GAME-CONTRACT.md` (rules, coverage, UI exposure, outcome explanation, hidden-info evidence) and `docs/UI-INTERACTION.md` (legal-only controls, outcome surface, accessibility, hidden-info safeguards). `docs/MULTI-SEAT-AND-SURFACE-CONTRACT.md` (PHA0NEXPHAFOU-002) is the cross-reference target.
3. Cross-artifact boundary under audit: the official-game contract + the UI doctrine; shared surface = per-seat/per-team outcome and showdown rationale plus the pairwise no-leak proof.
4. FOUNDATIONS principle restate: §11 (viewer-safe views, no hidden-info leak) and §7 (public UI is legal-only; Rust supplies views, TS presents). Edits are meaning-preserving clarifications.
5. Enforcement surface: §11 no-leak firewall. The pairwise no-leak proof matrix (public observer, each seat viewer, replay export, effect log, bot explanation, dev panel) clarifies the existing no-leak invariant; it introduces no leakage path and is enforced by the Infra D harness and Gate 15+ no-leak tests.

## Architecture Check

1. Explicit N-seat acceptance rows and a required showdown rationale prevent under-specified hidden-info games more reliably than per-gate rediscovery.
2. No backwards-compatibility aliasing/shims introduced.
3. `engine-core` stays noun-free; the showdown rationale is Rust-authored output (§2) — TypeScript presents it, never computes it.

## Verification Layers

1. OGC N-seat acceptance + showdown rationale rows present → manual review.
2. UI multi-seat layout + showdown rendering guidance present → manual review.
3. Pairwise no-leak matrix reads as a clarification → FOUNDATIONS alignment check (§11/§7).
4. Links resolve → `node scripts/check-doc-links.mjs`.

## What to Change

### 1. `docs/OFFICIAL-GAME-CONTRACT.md`

Add official-game requirements for a declared seat range; a per-seat/per-team final breakdown; for any game with showdown/evaluation, Rust MUST produce each contender's evaluated combination, comparison vector, and decisive comparison reason; and, for hidden-info N-seat games, a pairwise no-leak proof matrix across public observer, each seat viewer, replay export, effect log, bot explanation, and dev panel.

### 2. `docs/UI-INTERACTION.md`

Add "multi-seat layout" guidance: the seat rail handles 3–7 seats without two-column assumptions; active seat and pending responders are displayed from the Rust view; turn order is visualized as Rust data; a local hotseat/observer selector is explicit. Add "showdown/explanation" requirements: each contender's best combination, used components, comparison vector, split/tie reason, and the redaction rule for folded/non-revealed private data.

## Files to Touch

- `docs/OFFICIAL-GAME-CONTRACT.md` (modify)
- `docs/UI-INTERACTION.md` (modify)

## Out of Scope

- Editing `OutcomeExplanationPanel.tsx` or any web/Rust code (Infra C and Gate 15+ own that).
- Letting TypeScript decide legality or compute showdown results.
- Trace-schema or WASM exported-API schema changes.

## Acceptance Criteria

### Tests That Must Pass

1. `docs/OFFICIAL-GAME-CONTRACT.md` carries the declared-seat-range, per-seat/per-team breakdown, Rust-authored showdown rationale, and pairwise no-leak proof-matrix requirements.
2. `docs/UI-INTERACTION.md` carries the multi-seat layout and showdown/explanation rendering requirements.
3. `node scripts/check-doc-links.mjs` passes.

### Invariants

1. The §11 no-leak invariant is unchanged in meaning — the pairwise matrix strengthens, never weakens, it.
2. §2 behavior authority is preserved — the showdown rationale is Rust-authored; the UI presents it.

## Test Plan

### New/Modified Tests

1. `None — documentation-only ticket; verification is command-based and existing pipeline/test coverage is named in Assumption Reassessment.`

### Commands

1. `node scripts/check-doc-links.mjs`
2. `grep -niE "pairwise|showdown|seat range|seat rail" docs/OFFICIAL-GAME-CONTRACT.md docs/UI-INTERACTION.md`
3. `bash scripts/boundary-check.sh`

## Outcome

Completed: 2026-06-13

Updated `docs/OFFICIAL-GAME-CONTRACT.md` with declared-seat-range,
per-seat/per-team final-breakdown, Rust-authored showdown/evaluation rationale,
and hidden-info N-seat pairwise no-leak proof-matrix requirements. The contract
now names public observer, each seat viewer, replay export, effect log, bot
explanation, and dev panel surfaces for N-seat hidden-info proof.

Updated `docs/UI-INTERACTION.md` with multi-seat payload and layout doctrine:
Rust/WASM supplies active seats, pending responders, viewer/observer identity,
turn-order cues, and safe role/team labels; TypeScript presents those facts and
does not infer turn order or legality. Added showdown/comparison rendering
requirements for evaluated results, used components, comparison vectors,
split/tie reasons, decisive reasons, and redaction for folded/non-revealed
private data.

Deviations from plan: none. `apps/web/src/components/OutcomeExplanationPanel.tsx`
was inspected and exists, but no web or Rust code was changed.

Verification:

- `node scripts/check-doc-links.mjs` passed (`Checked 27 markdown files`).
- `bash scripts/boundary-check.sh` passed (`engine-core boundary check passed`).
- `grep -niE "pairwise|showdown|seat range|seat rail" docs/OFFICIAL-GAME-CONTRACT.md docs/UI-INTERACTION.md`
  found the new proof, showdown, seat-range, and seat-rail doctrine.
- `test -e apps/web/src/components/OutcomeExplanationPanel.tsx` confirmed the
  cited shared outcome panel exists.

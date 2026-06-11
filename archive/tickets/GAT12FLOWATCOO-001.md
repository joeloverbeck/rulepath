# GAT12FLOWATCOO-001: Rules and IP source docs for Flood Watch

**Status**: ✅ COMPLETED
**Priority**: HIGH
**Effort**: Medium
**Engine Changes**: None — documentation only (`games/flood_watch/docs/RULES.md`, `games/flood_watch/docs/SOURCES.md`); no Rust, schema, or trace surface
**Deps**: None

## Problem

Per `docs/OFFICIAL-GAME-CONTRACT.md` §3 and the official-game-gate pattern, original rules prose and IP source notes are front-loaded before implementation. Every later ticket — state model, validation, golden traces, `tools/rule-coverage`, and the outcome-explanation surface — cites stable rule IDs that must already exist, and the IP-conservatism review (FOUNDATIONS §10) must be on record before any public-facing label is coded. This ticket authors `flood_watch`'s two front-loaded docs so downstream work has authoritative rule IDs and a documented originality posture.

## Assumption Reassessment (2026-06-11)

1. The structural exemplar `games/masked_claims/docs/RULES.md` exists and carries stable rule IDs (e.g. `MC-END-005`) consumed by `tools/rule-coverage` and the outcome-explanation surface; `flood_watch` mirrors that shape with an `FW-` prefix. Templates `templates/GAME-RULES.md` and `templates/GAME-SOURCES.md` exist and are the instantiation source.
2. The spec (`specs/gate-12-flood-watch-cooperative-event-pressure-proof.md`) §Deliverables "Per-game docs" and Work-breakdown item 11 require `RULES.md` to carry stable rule IDs "including shared-outcome/end IDs for the outcome-explanation contract"; the proposed rules (`Implementation reference` → "Proposed original rules") and Assumptions A2–A4 fix district/role/event names as original placeholders subject to IP review before implementation.
3. Cross-artifact boundary under audit: `RULES.md` rule IDs are a contract. `tools/rule-coverage` (registered in GAT12FLOWATCOO-015) maps each rules-doc obligation to a test/trace, and `scripts/check-outcome-explanations.mjs` reads `RULES.md`'s "Scoring and accounting" and "Terminal conditions" sections plus `UI.md`'s outcome section. Rule IDs and those section headers must be stable from this ticket onward so later tickets bind to them, not redefine them.
4. FOUNDATIONS §10 IP-conservatism motivates this ticket: all names, labels, and prose must be original and the trade-dress register (Forbidden Island/Desert, Pandemic, Spirit Island, Flash Point) recorded. `docs/IP-POLICY.md` is the governing policy; the spec's IP-risk register (Implementation reference) names the specific hazards to avoid (island/tile vocabulary, "shore up"/"sandbag", the named roles, outbreak/epidemic vocabulary, re-shuffle escalation). §6 evidence-heavy makes these docs part of the done contract, not optional.

## Architecture Check

1. Front-loading rules + sources is cleaner than authoring them at the end: downstream tickets bind to stable rule IDs rather than inventing throwaway IDs that later churn, and the IP posture gates naming before any label ships to code.
2. No backwards-compatibility aliasing/shims — these are net-new docs for a net-new game.
3. No `engine-core` or `game-stdlib` surface is touched; all event/deck/role/district/flood/levee/budget nouns stay game-local in `games/flood_watch/docs`.

## Verification Layers

1. Original prose, no copied rulebook text or trade-dress vocabulary -> manual review (IP-conservatism audit per `docs/IP-POLICY.md` + spec IP-risk register) recorded in `SOURCES.md`.
2. Stable rule IDs present (including shared-outcome/terminal IDs) -> codebase grep-proof (`grep -E "FW-(SETUP|ACT|ENV|END)" games/flood_watch/docs/RULES.md`); full mapping to tests/traces is enforced later by `tools/rule-coverage` (GAT12FLOWATCOO-015).
3. Outcome-explanation contract sections present -> grep-proof of the "Scoring and accounting" / "Terminal conditions" section headers `check-outcome-explanations.mjs` will consume (GAT12FLOWATCOO-017).
4. Doc link integrity -> `node scripts/check-doc-links.mjs`.

## What to Change

### 1. `games/flood_watch/docs/RULES.md`

Instantiate from `templates/GAME-RULES.md`. Author original rules covering: components (five districts, flood levels 0–3, levee stacks 0–2, the 24-card event deck, roles, action budget, forecast marker, shared outcome), setup for both scenarios, the budgeted turn flow (`bail`/`reinforce`/`forecast`/`end_turn`), the environment phase (draw, levee absorption before rise, inundation early-stop, deck exhaustion), and terminal conditions. Assign stable rule IDs with an `FW-` prefix and a dedicated terminal/shared-outcome ID block (e.g. `FW-END-001` shared loss on inundation, `FW-END-002` shared win on deck exhaustion) for the outcome-explanation contract. Include the "Scoring and accounting" and "Terminal conditions" section headers `check-outcome-explanations.mjs` requires.

### 2. `games/flood_watch/docs/SOURCES.md`

Instantiate from `templates/GAME-SOURCES.md`. Record the consulted-mechanics-only posture, the trade-dress avoidance register naming the adjacent commercial designs and the specific hazards avoided (island/tile vocabulary, "shore up"/"sandbag" labels, the named roles, outbreak/epidemic vocabulary, re-shuffle escalation), the originality rationale for every name/label (per Assumption A2/A10), and asset/font status. Note that no external research pass backed the spec (A10) and that names remain renameable before implementation if IP review flags them.

## Files to Touch

- `games/flood_watch/docs/RULES.md` (new)
- `games/flood_watch/docs/SOURCES.md` (new)

## Out of Scope

- Any Rust, schema, fixture, or trace implementation (GAT12FLOWATCOO-003+).
- The other eleven per-game docs (their own tickets).
- Web player-rules generation and outcome-template wiring (GAT12FLOWATCOO-017).
- Renaming decisions deferred to IP review — record the posture; do not pre-empt maintainers' final naming.

## Acceptance Criteria

### Tests That Must Pass

1. `node scripts/check-doc-links.mjs` passes with the two new docs present.
2. `grep -E "FW-(SETUP|ACT|ENV|END)" games/flood_watch/docs/RULES.md` returns the rule-ID set, including at least two `FW-END-*` shared-outcome/terminal IDs.
3. `grep -E "Scoring and accounting|Terminal conditions" games/flood_watch/docs/RULES.md` returns both section headers (the outcome-explanation contract surface).

### Invariants

1. All prose, names, and labels are original; no copied rulebook text, role names, or trade-dress vocabulary from the named commercial designs.
2. Rule IDs are stable from this ticket onward — later tickets bind to them and must not redefine them.

## Test Plan

### New/Modified Tests

1. `None — documentation-only ticket; verification is command-based and the rule-ID-to-test mapping is enforced later by `tools/rule-coverage` (GAT12FLOWATCOO-015) and `scripts/check-outcome-explanations.mjs` (GAT12FLOWATCOO-017).`

### Commands

1. `node scripts/check-doc-links.mjs`
2. `grep -E "FW-(SETUP|ACT|ENV|END)" games/flood_watch/docs/RULES.md && grep -E "Scoring and accounting|Terminal conditions" games/flood_watch/docs/RULES.md`
3. A full `cargo`/tool pipeline is not the verification boundary here: no code exists yet, and `rule-coverage` cannot run until the crate and tests land. The correct boundary is doc-link integrity plus rule-ID/section grep-proofs that downstream tickets bind to.

## Outcome

Completed: 2026-06-11

Implemented `games/flood_watch/docs/RULES.md` with original Flood Watch rules,
stable `FW-*` rule IDs, shared-outcome terminal IDs, the required "Scoring and
accounting" and "Terminal conditions" sections, replay/randomness notes,
visibility boundaries, bot constraints, and explicit out-of-scope variants.

Implemented `games/flood_watch/docs/SOURCES.md` with the project-authority
source list, original-name rationale, variant/deviation notes, ambiguity log,
trade-dress avoidance register, asset/font status, public/private content
boundary, and rule-ID cross-reference.

Deviations from plan: none. No Rust, schema, fixture, trace, web, or tool
surface was changed.

Verification:

- `node scripts/check-doc-links.mjs` passed (`Checked 25 markdown files`).
- `grep -E "FW-(SETUP|ACT|ENV|END)" games/flood_watch/docs/RULES.md` returned
  the setup/action/environment/end rule IDs, including `FW-END-001` and
  `FW-END-002`.
- `grep -E "Scoring and accounting|Terminal conditions" games/flood_watch/docs/RULES.md`
  returned both required section headers.

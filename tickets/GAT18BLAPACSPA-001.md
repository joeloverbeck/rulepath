# GAT18BLAPACSPA-001: Blackglass Pact rules and source documents

**Status**: PENDING
**Priority**: HIGH
**Effort**: Medium
**Engine Changes**: None — game-local docs (`games/blackglass_pact/docs/{RULES,SOURCES,RULE-COVERAGE}.md`)
**Deps**: none

## Problem

Per `docs/OFFICIAL-GAME-CONTRACT.md` §3, original rules prose and source notes must precede implementation. Gate 18 admits the partnership-Spades game **Blackglass Pact**; this ticket front-loads its formal rules (keyed to stable `BP-*` IDs), the source bibliography and variant reconciliation, the neutral-name/IP rationale, and the initial rule-coverage skeleton, so every later module ticket implements against a fixed rule contract (spec §4.2, candidate task `GAT18-BLAPAC-001`).

## Assumption Reassessment (2026-06-25)

1. The per-game doc convention drops the template `GAME-` prefix: `games/briar_circuit/docs/` carries `RULES.md` / `SOURCES.md` / `RULE-COVERAGE.md` (not `GAME-RULES.md`). Blackglass Pact follows the same names; the spec §4.1 tree matches this.
2. The `BP-*` rule-ID skeleton in spec Appendix A (§A.1–§A.8) is the normative anchor set; `RULES.md` and `RULE-COVERAGE.md` must use exactly those IDs. Locked parameters live in spec §3.1.
3. Cross-artifact boundary under audit: the rule contract these docs fix is consumed by every later ticket (setup→bots→tests) and by `tools/rule-coverage` (which reads `RULES.md` + `RULE-COVERAGE.md`); the IDs must stay stable.
4. FOUNDATIONS §10 (IP conservatism) motivates this ticket: rules prose, examples, and the "Blackglass Pact" identity must be original; "Spades" is a public-domain family label only. Human IP review under `docs/IP-POLICY.md` stays mandatory and is recorded as pending.

## Architecture Check

1. Front-loading rules-as-contract (vs. authoring prose alongside code) prevents the browser/code from becoming the de facto rule source and keeps `BP-*` IDs stable across the gate.
2. No backwards-compatibility shims; this is a new game's first artifacts.
3. `engine-core` is untouched; no mechanic noun enters the kernel and no `game-stdlib` change is proposed.

## Verification Layers

1. Every locked parameter in spec §3.1 has a `BP-*` rule and a source citation or explicit "Rulepath formalization" -> manual review against §3.1 + §F.1.
2. Rule IDs match spec Appendix A exactly -> grep-proof of `BP-` IDs in `RULES.md` against the Appendix A list.
3. Original-prose / no-copied-text obligation -> manual IP review note recorded in `SOURCES.md`; `node scripts/check-doc-links.mjs` passes for any links.

## What to Change

### 1. `games/blackglass_pact/docs/RULES.md`

Original Rulepath prose keyed to the `BP-*` IDs from spec Appendix A: identity/setup/teams (`BP-ID-*`, `BP-SETUP-*`), blind-nil (`BP-BLIND-*`), deal (`BP-DEAL-*`), bidding/contracts (`BP-BID-*`), trick play (`BP-PLAY-*`), scoring/bags (`BP-SCORE-*`), terminal (`BP-END-*`), visibility/replay/bot/UI (`BP-VIS/REPLAY/BOT/UI-*`). No copied sequences or examples.

### 2. `games/blackglass_pact/docs/SOURCES.md`

Bibliography (spec §F.5 footnotes E1–E16), variant reconciliation table (spec §F.1), deliberate-deviation notes (failed-nil-as-bags, pre-deal blind commitment), neutral-name rationale (spec §F.4), original-prose declaration, and recorded **pending human IP review**.

### 3. `games/blackglass_pact/docs/RULE-COVERAGE.md`

Initial coverage skeleton: each `BP-*` ID with placeholder owner/test columns (finalized in GAT18BLAPACSPA-013). No orphan IDs.

## Files to Touch

- `games/blackglass_pact/docs/RULES.md` (new)
- `games/blackglass_pact/docs/SOURCES.md` (new)
- `games/blackglass_pact/docs/RULE-COVERAGE.md` (new)

## Out of Scope

- Any Rust code, fixtures, or traces (later tickets).
- `HOW-TO-PLAY.md` player-facing rules (co-lands with the WASM ticket GAT18BLAPACSPA-014 per `check-player-rules`).
- Finalizing rule-coverage owner/test mappings (GAT18BLAPACSPA-013).

## Acceptance Criteria

### Tests That Must Pass

1. `grep -oE "BP-[A-Z]+-[0-9]+" games/blackglass_pact/docs/RULES.md | sort -u` matches the Appendix A ID set.
2. `node scripts/check-doc-links.mjs` passes (no broken links introduced).
3. `SOURCES.md` records the pending human IP review line.

### Invariants

1. Every locked §3.1 parameter is traceable to a `BP-*` rule.
2. No copied rulebook prose, examples, or trade dress; "Spades" appears only as a family label.

## Test Plan

### New/Modified Tests

1. `None — documentation-only ticket; verification is command-based and downstream coverage is finalized in GAT18BLAPACSPA-013.`

### Commands

1. `grep -c "BP-" games/blackglass_pact/docs/RULES.md`
2. `node scripts/check-doc-links.mjs`
3. Doc-only ticket — the narrow grep + doc-link check is the correct verification boundary; rule-coverage tooling runs after the crate and coverage doc land.

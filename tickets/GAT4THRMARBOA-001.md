# GAT4THRMARBOA-001: Three Marks rules research & IP source docs

**Status**: PENDING
**Priority**: HIGH
**Effort**: Medium
**Engine Changes**: Yes — new docs `games/three_marks/docs/RULES.md`, `games/three_marks/docs/SOURCES.md` (no code surfaces)
**Deps**: None

## Problem

Gate 4 introduces the official game `three_marks` (public name **Three Marks**). Per `docs/OFFICIAL-GAME-CONTRACT.md` §3, rules research and original prose MUST precede implementation ("Do not start with UI and backfill rules later"). Without the original rules summary and source/IP notes landing first, the Rust implementation has no authoritative, IP-clean rule reference to build against and risks copied prose or trade-dress.

## Assumption Reassessment (2026-06-06)

1. The sibling game `race_to_n` ships `games/race_to_n/docs/RULES.md` and `games/race_to_n/docs/SOURCES.md` as original Rulepath prose; this ticket mirrors that shape for `three_marks`. Verified both files exist under `games/race_to_n/docs/`.
2. Spec `specs/gate-4-three-marks-board-smoke.md` §7 (rules/variant), §16 (doc set), and §19 (IP/source posture) define the required content; §19.2 lists the consulted external sources with consulted date 2026-06-06. The variant is `three_marks_standard`, rules version `three_marks-rules-v1`, cell ids `r1c1`..`r3c3` (spec §7.2, §24).
3. Cross-artifact boundary under audit: the doc-governed contract in `docs/OFFICIAL-GAME-CONTRACT.md` §4 (source notes) and §5 (original Rulepath rules summary), plus `docs/IP-POLICY.md`. These docs govern what RULES.md/SOURCES.md must contain; this ticket produces prose conforming to them, not new contract surface.
4. FOUNDATIONS §10 (IP conservatism) and §6 (official games are evidence-heavy) motivate this ticket: public games MUST use original rules prose and original assets, and every official game MUST carry an original rules summary and source notes. Restating before trusting the spec: no rulebook text, board art, or trade dress may be copied; neutral naming where commercial trademark risk exists (Three Marks, not Tic-Tac-Toe branding).

## Architecture Check

1. Front-loading rules prose is cleaner than backfilling docs after code: it gives every downstream ticket (002–015) a single authoritative, IP-reviewed rule reference, and prevents rule drift between code and prose. Alternative (write docs last) is rejected by `docs/OFFICIAL-GAME-CONTRACT.md` §3.
2. No backwards-compatibility aliasing/shims — these are new files.
3. No code surfaces touched; `engine-core` and `game-stdlib` are untouched (`engine-core` stays free of mechanic nouns trivially; no helper extraction).

## Verification Layers

1. Original-prose / IP-clean invariant -> manual review (IP-conservatism audit per `docs/IP-POLICY.md`; no copied prose/assets, neutral naming).
2. Rule-content completeness invariant -> manual review against spec §7.2 rule-area table (players, marks, board/cell terms, setup, turn order, legal placement, occupied illegality, win, draw, terminal, variant, excluded variants).
3. Source-note completeness invariant -> manual review against spec §19.1 (consulted source, consulted date, how used, no-copy statement, chosen/excluded variants, ambiguities).
4. Single-artifact-pair ticket: layers above map to the two distinct surfaces (rules prose vs. source/IP notes); no code-proof surface applies because the ticket ships no code.

## What to Change

### 1. `games/three_marks/docs/RULES.md`

Original Rulepath prose covering, with stable headings usable by later rule-coverage rows (spec §7.2, Appendix C seed — rewrite in polished voice, do not paste): two seats and deterministic seat order (first seat places first mark); distinct mark tokens; fixed 3×3 board with nine named cells `r1c1`..`r3c3`; empty setup; alternating placement into empty cells; legal-placement conditions; occupied-cell illegality; immediate win on completing any row/column/diagonal (winning seat + exact three cells reported by Rust); full-board draw with no line; terminal behaviour (no further legal placements); the single shipped variant `three_marks_standard`; and explicitly excluded variants (no movement/sliding, no Three Men's Morris/Achi, no misère/wild, no larger/configurable board, no generalized m,n,k).

### 2. `games/three_marks/docs/SOURCES.md`

Record consulted sources and consulted date (2026-06-06) from spec §19.2; how each was used (rule confirmation / IP posture / strategy-priority background / accessibility guidance); explicit no-copied-prose/assets statement; chosen variant `three_marks_standard`; excluded variants; public-name rationale (Three Marks as neutral project-owned name); and remaining rule ambiguities.

## Files to Touch

- `games/three_marks/docs/RULES.md` (new)
- `games/three_marks/docs/SOURCES.md` (new)

## Out of Scope

- Any Rust code, static data (`manifest.toml`/`variants.toml`), or tests (land in 002+).
- The remaining game docs (RULE-COVERAGE, MECHANICS, AI, UI, BENCHMARKS, ADMISSION) — they cite implemented surfaces and land in GAT4THRMARBOA-015.
- SVG/visual assets (land with the web renderer, GAT4THRMARBOA-011).

## Acceptance Criteria

### Tests That Must Pass

1. `test -f games/three_marks/docs/RULES.md && test -f games/three_marks/docs/SOURCES.md` — both files exist.
2. `grep -iE "three_marks_standard|three_marks-rules-v1" games/three_marks/docs/RULES.md` — variant id and rules version present.
3. Manual IP review: no sentence is a verbatim copy of any source in spec §19.2; naming is neutral (no commercial-brand framing).

### Invariants

1. Rules prose is original Rulepath wording and names only the classic placement game scoped by spec §7 (no excluded variant described as in-scope).
2. SOURCES.md records consulted date 2026-06-06 and an explicit no-copy posture.

## Test Plan

### New/Modified Tests

1. `None — documentation-only ticket; verification is command-based (grep/test -f) plus the manual IP/rule-completeness review named in Assumption Reassessment.`

### Commands

1. `test -f games/three_marks/docs/RULES.md && test -f games/three_marks/docs/SOURCES.md`
2. `grep -niE "draw|row, column, or diagonal|r1c1|three_marks_standard" games/three_marks/docs/RULES.md`
3. A narrower (grep/manual) boundary is correct here: there is no compiled surface to exercise, and `tools/rule-coverage` validation of RULE-COVERAGE.md is deferred to GAT4THRMARBOA-014/015.

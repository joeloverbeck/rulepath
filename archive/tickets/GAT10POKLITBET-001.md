# GAT10POKLITBET-001: Crest Ledger rules prose, source notes, and admission record

**Status**: ✅ COMPLETED
**Priority**: HIGH
**Effort**: Medium
**Engine Changes**: None — new per-game docs only (`games/poker_lite/docs/*`); no Rust/engine surface touched
**Deps**: None

## Problem

Gate 10 admits a new official game, `poker_lite` (public display name **Crest Ledger**), per `specs/gate-10-poker-lite-betting-showdown.md`. Per `docs/OFFICIAL-GAME-CONTRACT.md` §3, original rules prose and source/IP notes MUST precede implementation: rule IDs are the contract every later ticket (rules engine, rule-coverage, tests) maps against, and the IP posture must be fixed before any casino-adjacent mechanic is coded. This ticket front-loads `RULES.md`, `SOURCES.md`, and `GAME-IMPLEMENTATION-ADMISSION.md` so downstream tickets have a stable rule-ID set and an explicit non-copying stance to cite.

## Assumption Reassessment (2026-06-08)

1. The per-game docs set and naming match the freshest sibling: `games/secret_draft/docs/` contains `RULES.md`, `SOURCES.md`, `GAME-IMPLEMENTATION-ADMISSION.md` (plus 8 more authored later). Templates exist at `templates/GAME-RULES.md`, `templates/GAME-SOURCES.md`, `templates/GAME-IMPLEMENTATION-ADMISSION.md` — instantiate from these.
2. The spec (`specs/gate-10-poker-lite-betting-showdown.md` §2, §3 In scope, appendix §A "Core rules" and §G "Sources note") fixes the concrete variant: two seats, six-card deck (three ranks × two copies), one private card per seat, one hidden center card revealed after round 1, two pledge rounds with units `[1,2]`, one-lift cap, yield terminal without reveal, showdown pair-before-high-card comparator, exact split on tie. RULES.md must encode exactly this with stable rule IDs; no scope beyond the spec's §A.
3. Cross-artifact boundary under audit: the rule IDs authored here are consumed by `games/poker_lite/docs/RULE-COVERAGE.md` (GAT10POKLITBET-012) and asserted by `tools/rule-coverage`. The rule-ID namespace must be stable and complete before those land. This ticket owns the canonical rule-ID list.
4. FOUNDATIONS §10 (IP conservatism) and §6 (evidence-heavy official games) motivate this ticket. Restated: public files MUST use original rules prose and original/neutral naming; Kuhn/Leduc/OpenSpiel may be consulted as research-minimal structures but no rules prose, hand-ranking table, casino imagery, product naming, or trade dress may be copied. The spec's §G sources note and footnotes [^kuhn][^leduc][^openSpiel*] are the provenance to carry forward — as *consulted-not-copied* notes, never as copied text.

## Architecture Check

1. Authoring rules/IP docs before code is the OGC §3 contract: a single canonical rule-ID list prevents the rules engine (GAT10POKLITBET-005) and rule-coverage doc (012) from drifting apart. The alternative — deriving rule IDs from code after the fact — invites coverage gaps the `rule-coverage` tool cannot detect.
2. No backwards-compatibility aliasing/shims introduced — these are new files.
3. `engine-core` is untouched (no mechanic noun added to the kernel, §3); `game-stdlib` is untouched (§4) — this is prose only.

## Verification Layers

1. Rules completeness (every rule referenced by later tickets has an ID) -> manual review against spec §A + grep-proof that RULE-COVERAGE.md (012) can map each ID.
2. IP-conservatism (original prose, neutral naming, no copied tables/trade dress) -> manual review / IP-conservatism audit against FOUNDATIONS §10 and the spec's §G non-copying posture.
3. Doc-link integrity (no broken cross-links in new docs) -> `node scripts/check-doc-links.mjs`.
4. Single-artifact-class ticket (docs only); the three layers above are the applicable proof surfaces — no replay/serialization/no-leak code surface is introduced here.

## What to Change

### 1. `games/poker_lite/docs/RULES.md`

Instantiate from `templates/GAME-RULES.md`. Encode **Crest Ledger** original rules with stable rule IDs (e.g. `CL-SETUP-01`, `CL-PLEDGE-01`, `CL-REVEAL-01`, `CL-SHOWDOWN-01`, `CL-YIELD-01`, `CL-SPLIT-01`) covering: components (seats, six-card deck three ranks × two copies, private crest per seat, hidden center crest, deck tail, opening 1-marker contribution); action families (hold/press/lift/match/yield) with legality and accounting; round close + center reveal timing (after round 1 without yield); showdown comparator (pair_flag then private rank, lexicographic); yield terminal (no reveal); exact split on tie; deterministic maximum action/contribution bound. Use neutral terms (crest, marker, pledge, shared pool); no casino/poker prose.

### 2. `games/poker_lite/docs/SOURCES.md`

Instantiate from `templates/GAME-SOURCES.md`. Record Kuhn (1951), Leduc-style (Kroer & Sandholm 2017 §5), and OpenSpiel information-state framing as **consulted research-minimal structures only**, with the spec's footnote URLs. State explicitly: no rules prose, hand-ranking table, casino imagery, product naming, or trade dress copied; **Crest Ledger** is an original Rulepath microgame.

### 3. `games/poker_lite/docs/GAME-IMPLEMENTATION-ADMISSION.md`

Instantiate from `templates/GAME-IMPLEMENTATION-ADMISSION.md`. Record: internal id `poker_lite`, display name **Crest Ledger**, Stage 9 / Gate 10, neutral presentation posture, IP constraints, and confirmation that `docs/MECHANIC-ATLAS.md` §10A open promotion debt is empty (no interlock blocks this gate).

## Files to Touch

- `games/poker_lite/docs/RULES.md` (new)
- `games/poker_lite/docs/SOURCES.md` (new)
- `games/poker_lite/docs/GAME-IMPLEMENTATION-ADMISSION.md` (new)

## Out of Scope

- Any Rust code, crate skeleton, or data files (GAT10POKLITBET-002+).
- `RULE-COVERAGE.md`, `MECHANICS.md`, `UI.md`, `AI.md`, bot/benchmark docs, `PRIMITIVE-PRESSURE-LEDGER.md` (later tickets).
- Editing `docs/SOURCES.md` central bibliography or `docs/ROADMAP.md` (per spec §10: ROADMAP gets no progress edit).
- Any casino/real-money framing, copied rules prose, or proprietary naming (spec §3 Not allowed).

## Acceptance Criteria

### Tests That Must Pass

1. `node scripts/check-doc-links.mjs` passes with the three new docs present.
2. Every action family and resolution path in spec §A (hold/press/lift/match/yield, center reveal, showdown comparator, yield terminal, split) has a corresponding rule ID in RULES.md — manual checklist against spec §A.
3. SOURCES.md contains the non-copying/IP-posture statement and the consulted-only framing for Kuhn/Leduc/OpenSpiel.

### Invariants

1. Rule-ID namespace is complete and stable — no later ticket needs to invent a rule ID absent here.
2. No copied rules prose, hand-ranking table, or trade dress appears in any new doc (FOUNDATIONS §10).

## Test Plan

### New/Modified Tests

1. `None — documentation-only ticket; verification is command-based (`check-doc-links`) plus manual rules-completeness and IP-conservatism review per Assumption Reassessment.`

### Commands

1. `node scripts/check-doc-links.mjs`
2. `ls games/poker_lite/docs/RULES.md games/poker_lite/docs/SOURCES.md games/poker_lite/docs/GAME-IMPLEMENTATION-ADMISSION.md`
3. Narrower command boundary: there is no Rust/test surface yet, so `cargo` checks are not applicable until GAT10POKLITBET-002 creates the crate.

## Outcome

Completed: 2026-06-09

Changed:

- Added `games/poker_lite/docs/RULES.md` with the initial `CL-*` stable rule ID set for Crest Ledger setup, action families, pledge accounting, center/showdown reveal, terminal outcomes, visibility/no-leak, replay/randomness, ambiguity resolutions, deviations, and out-of-scope boundaries.
- Added `games/poker_lite/docs/SOURCES.md` with consulted-only notes for the Gate 10 spec, Rulepath source/IP docs, Kuhn, Leduc-style benchmark context, and OpenSpiel information-state framing.
- Added `games/poker_lite/docs/GAME-IMPLEMENTATION-ADMISSION.md` recording the Gate 10 admission constraints, mechanic-atlas precondition, evidence expectations, primitive-pressure posture, static-data boundary, no-leak risks, bot requirements, UI expectations, and benchmark expectations.

Deviations from original plan:

- Used the current concise sibling-doc style rather than copying the full template boilerplate verbatim.
- Kept future docs such as `RULE-COVERAGE.md`, `MECHANICS.md`, `AI.md`, and `PRIMITIVE-PRESSURE-LEDGER.md` as plain paths except where linked files already exist, so this ticket does not introduce broken markdown links before later tickets create those artifacts.

Verification:

- `node scripts/check-doc-links.mjs` passed: checked 25 markdown files.
- `ls games/poker_lite/docs/RULES.md games/poker_lite/docs/SOURCES.md games/poker_lite/docs/GAME-IMPLEMENTATION-ADMISSION.md` confirmed the three required docs exist.
- Manual checklist against `specs/gate-10-poker-lite-betting-showdown.md` §A confirmed rule IDs cover hold, press, lift, match, yield, center reveal, showdown comparator, yield terminal, exact split, bounded contributions, and no-leak visibility.
- Manual IP/source check confirmed `SOURCES.md` states Crest Ledger is original, Kuhn/Leduc/OpenSpiel are consulted-only context, and no public rules prose, hand-ranking table, casino imagery, product naming, or trade dress is copied.

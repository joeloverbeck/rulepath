# RIVLEDOUT-002: River Ledger outcome-explanation presentation contract + docs

**Status**: COMPLETED
**Priority**: HIGH
**Effort**: Medium
**Engine Changes**: Yes (presentation/docs only) — `apps/web/src/wasm/client.ts`, `apps/web/src/components/outcomeExplanationTemplates.ts`, `games/river_ledger/docs/UI.md`, `games/river_ledger/docs/RULES.md`. No Rust behavior change.
**Deps**: RIVLEDOUT-001 (the Rust `terminal_rationale` field + `template_key` set this ticket mirrors)

## Problem

With RIVLEDOUT-001 emitting a Rust-owned `terminal_rationale`, the presentation
tier and docs must mirror it so `node scripts/check-outcome-explanations.mjs`
(the failing `Gate 1 game smoke` step) passes for river_ledger. That checker
requires four river_ledger surfaces the game currently lacks or mis-cases:
the `client.ts` rationale type/field mirror, the static template keys, the UI.md
outcome section (currently Title-Cased `## Outcome / Victory Explanation`, which the
**case-sensitive** check at `scripts/check-outcome-explanations.mjs:31` rejects),
and the RULES.md `## Scoring and accounting` / `## Terminal conditions` sections with
stable `*-SCORE-*`/`*-END-*` rule IDs.

## Assumption Reassessment (2026-06-15)

1. `apps/web/src/wasm/client.ts` has the shared `OutcomeRationalePayload` type
   (line 147) and per-game aliases (e.g. `PokerLiteOutcomeRationale =
   OutcomeRationalePayload`, line 165) plus a `terminal_rationale?: ... | null`
   field on `PokerLitePublicView` (line 702). `RiverLedgerPublicView` (line 764)
   has **no** such alias/field. Verified by read.
2. The checker (`scripts/check-outcome-explanations.mjs`) requires, per catalog
   game: (a) `^## Outcome / victory explanation$` in `docs/UI.md` (case-sensitive,
   line 31) naming six markers (lines 32-39: terminal result variants, decisive
   cause variants, per-player breakdown, hidden-info redaction, RULES.md rule IDs,
   smoke); (b) `## Scoring and accounting` + `## Terminal conditions` in `docs/RULES.md`
   (lines 138/141); (c) a `[A-Z]+-(SCORE|END|TERM)-...` rule ID (line 144);
   (d) a `RiverLedger\w*(Outcome|Victory|Terminal)\w*(Explanation|Rationale)` type
   AND a `(outcome|terminal|victory)_(explanation|rationale)` field on
   `RiverLedgerPublicView` (lines 152-156); (e) every `river_ledger.*` template key
   named in the UI section must exist in `outcomeExplanationTemplates.ts` (lines
   171-175). Verified by reading the checker.
3. Cross-artifact boundary under audit: the `template_key` strings. They are
   emitted by Rust (RIVLEDOUT-001), listed in the UI.md section, and defined in
   `outcomeExplanationTemplates.ts`. All three MUST use the identical key set,
   `allowedGameIds: ["river_ledger"]`. This is the contract this ticket closes.
4. FOUNDATIONS §2 restated: TypeScript presents only. The `client.ts` field mirrors
   a Rust-emitted field; no TS-side outcome decision, comparison, or tiebreak is
   introduced. The checker's `FORBIDDEN_TEMPLATE_PATTERNS` (lines 40-47) fail-close
   on any selector/conditional/tiebreak/leak token in the templates file — the new
   river_ledger entries must be inert copy only.
5. river_ledger RULES.md already uses an `RL-*` rule-ID scheme
   (`games/river_ledger/docs/RULES.md:24`, e.g. `RL-BET-BLIND-001`); the new IDs
   are `RL-SCORE-*` / `RL-END-*`, consistent with that scheme — no new convention.
6. The UI.md outcome section already exists with `### Terminal Result Variants`,
   `### Per-Seat Final Breakdown`, `### No-Leak Rules`, `### Smoke And Tests`
   (`games/river_ledger/docs/UI.md:132-180`); the six body markers are case-
   insensitive (regex `/i`), so only the `##` heading needs the lowercase
   `victory`/`explanation` rename plus the addition of the template keys + a
   decisive-cause mention if absent.

## Architecture Check

1. A per-game alias of the shared `OutcomeRationalePayload` plus a nullable
   `terminal_rationale` field is exactly the established pattern (race_to_n,
   three_marks, poker_lite, …); reusing it keeps one TS shape for every game's
   outcome surface rather than a bespoke river_ledger type.
2. No backwards-compatibility shim: the field is added directly to
   `RiverLedgerPublicView`; no legacy parsing of `TerminalView.explanations` strings
   is retained on the TS side.
3. `engine-core` untouched; no `game-stdlib` change. Templates file stays static
   copy (no behavior), satisfying the static-data-is-not-behavior boundary (§5).

## Verification Layers

1. Outcome-explanation contract presence -> `node scripts/check-outcome-explanations.mjs`
   passes (all five river_ledger sub-checks green).
2. Template keys ↔ Rust `template_key` parity -> codebase grep-proof: every key in
   `outcomeExplanationTemplates.ts` for river_ledger matches a `template_key` emitted
   by RIVLEDOUT-001 and named in UI.md.
3. No TS legality / no leak -> manual review + checker `FORBIDDEN_TEMPLATE_PATTERNS`:
   templates contain inert copy only; no selector/tiebreak/conditional/leak tokens.
4. Browser outcome surface still renders -> web smoke: `node apps/web/e2e/outcome-explanation.smoke.mjs`
   (catalog-driven; confirm river_ledger is covered and passes).
5. Doc link / catalog integrity unaffected -> `node scripts/check-doc-links.mjs` and
   `node scripts/check-catalog-docs.mjs` still pass.

## What to Change

### 1. TS client mirror (`apps/web/src/wasm/client.ts`)

Add `export type RiverLedgerOutcomeRationale = OutcomeRationalePayload;` and add
`terminal_rationale?: RiverLedgerOutcomeRationale | null;` to `RiverLedgerPublicView`
(line 764), mirroring `PokerLitePublicView` (line 702).

### 2. Static templates (`apps/web/src/components/outcomeExplanationTemplates.ts`)

Add one entry per river_ledger `template_key` from RIVLEDOUT-001
(`river_ledger.showdown_best_hand_win`, `river_ledger.showdown_split_pot`,
`river_ledger.last_live_fold_win`), each with `allowedGameIds: ["river_ledger"]`,
inert summary/heading/ruleRefLabel copy, following the poker_lite block shape
(lines 157-184).

### 3. UI.md (`games/river_ledger/docs/UI.md`)

Rename `## Outcome / Victory Explanation` (line 132) to
`## Outcome / victory explanation`. Ensure the section body names a decisive-cause
variant and lists the static template keys from change 2 (so the
`extractTemplateKeys` ↔ templates-file parity check passes). Keep the existing
terminal-result / per-seat-breakdown / no-leak / smoke subsections.

### 4. RULES.md (`games/river_ledger/docs/RULES.md`)

Add a `## Scoring and accounting` section (gathering the existing fixed-limit
contribution / pot accounting content) and a `## Terminal conditions` section
(gathering hand-evaluation/showdown end + last-live-hand end), each introducing
stable `RL-SCORE-*` and `RL-END-*` rule IDs referenced by the rationale
`decisive_rule_ids` in RIVLEDOUT-001.

## Files to Touch

- `apps/web/src/wasm/client.ts` (modify)
- `apps/web/src/components/outcomeExplanationTemplates.ts` (modify)
- `games/river_ledger/docs/UI.md` (modify)
- `games/river_ledger/docs/RULES.md` (modify)

## Out of Scope

- Any Rust or `wasm-api` change (RIVLEDOUT-001 owns the emitted field and keys).
- Adding river_ledger-specific logic to the web e2e smoke if the catalog-driven
  smoke already covers it (verify first; only extend if river_ledger is skipped).

## Acceptance Criteria

### Tests That Must Pass

1. `node scripts/check-outcome-explanations.mjs` — passes for river_ledger (the Gate 1 failure clears).
2. `node apps/web/e2e/outcome-explanation.smoke.mjs` — river_ledger outcome surface renders.
3. `npm --prefix apps/web run build` — TS mirror compiles with the new type/field.

### Invariants

1. Every river_ledger `template_key` exists in exactly the three synchronized
   surfaces (Rust emit, UI.md, templates file) with `allowedGameIds: ["river_ledger"]`.
2. The templates file carries inert copy only — no selector/conditional/tiebreak/
   leak tokens (checker `FORBIDDEN_TEMPLATE_PATTERNS` stays green).

## Test Plan

### New/Modified Tests

1. `None — presentation/docs ticket; verification is command-based via the outcome-explanation checker, the web smoke, and the web build named below.`

### Commands

1. `node scripts/check-outcome-explanations.mjs`
2. `npm --prefix apps/web run build && node apps/web/e2e/outcome-explanation.smoke.mjs`
3. `node scripts/check-doc-links.mjs && node scripts/check-catalog-docs.mjs` — confirm the doc edits don't regress the other cross-cutting smokes in the same Gate 1 job.

## Outcome

Completed: 2026-06-15

What changed:

- Added the `RiverLedgerOutcomeRationale` TypeScript mirror and
  `terminal_rationale` field to `RiverLedgerPublicView`.
- Added inert River Ledger outcome templates for last-live fold wins,
  showdown wins, and showdown split allocation.
- Updated `games/river_ledger/docs/UI.md` to the checker-required
  `## Outcome / victory explanation` heading and documented the Rust-emitted
  template keys and decisive-cause values.
- Added `## Scoring and accounting` and `## Terminal conditions` sections to
  `games/river_ledger/docs/RULES.md`, with the `RL-SCORE-*` and `RL-END-*`
  IDs emitted by Rust.
- Routed `RiverLedgerBoard` through the shared `OutcomeExplanationPanel` so
  the new Rust rationale is actually rendered, then extended the outcome
  browser smoke with a River Ledger fold-out terminal path.

Deviations from the plan:

- `apps/web/src/components/RiverLedgerBoard.tsx`,
  `apps/web/e2e/outcome-explanation.smoke.mjs`, and
  `apps/web/e2e/river-ledger.smoke.mjs` were added to the touched surface. The
  existing outcome smoke only covered Race to 21 and Three Marks, and the
  River Ledger board still used a bespoke terminal block, so these changes were
  required to prove the presentation contract in browser behavior.

Verification:

- `node scripts/check-outcome-explanations.mjs` passed for all 15 catalog games.
- `npm --prefix apps/web run build` passed.
- `node apps/web/e2e/outcome-explanation.smoke.mjs` passed with River Ledger
  coverage.
- `node apps/web/e2e/river-ledger.smoke.mjs` passed.
- `node scripts/check-doc-links.mjs` passed.
- `node scripts/check-catalog-docs.mjs` passed.

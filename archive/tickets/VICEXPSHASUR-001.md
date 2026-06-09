# VICEXPSHASUR-001: Outcome-explanation contract — doctrine, area-doc & template amendments + spec-index row

**Status**: COMPLETED
**Priority**: HIGH
**Effort**: Medium
**Engine Changes**: None — docs, templates, and the `specs/README.md` index only; no Rust/engine, WASM, or behavior surface touched.
**Deps**: None

## Problem

Rulepath has no contract requiring a terminal outcome explanation. Each board owns an ad-hoc terminal status; there is no doctrine making "explain why this match ended" a mandatory, catalog-complete, viewer-safe official-game obligation. Before any Rust rationale (003–008), shared panel (009), or CI guard (002) is built, the obligation must be made permanent: amend `docs/UI-INTERACTION.md` and `docs/OFFICIAL-GAME-CONTRACT.md`, extend the three game templates, and register the spec in `specs/README.md` at `Planned`. Source: `archive/specs/victory-explanation-shared-surface.md` §15.1, Appendix A (§16), Appendix B (§17), Appendix C (§18), §14.1.

## Assumption Reassessment (2026-06-09)

1. No code surface is touched; the catalog this contract will be enforced against is `crates/wasm-api/src/lib.rs` `const GAME_*` (nine games), the same source `scripts/check-catalog-docs.mjs` / `check-player-rules.mjs` parse — relevant only because 002 enforces this contract against it.
2. Amendment targets verified during in-session `/reassess-spec`: `docs/UI-INTERACTION.md` has **no** discrete "shared rules surface" section (headings §1–§18; the rules surface is prose inside §16 "Accessibility baseline"), so the spec's Appendix A.1 anchor was corrected — add the new section after §15 "Bot explanation UI", renumbering §16 "Accessibility baseline"/§17 "Responsive behavior"/§18 "UI acceptance check" accordingly; A.2 targets the existing §18 "UI acceptance check". `docs/OFFICIAL-GAME-CONTRACT.md` §5 and §10 exist. `templates/GAME-HOW-TO-PLAY.md` **already carries** `## Scoring and winning` (line ~55, authored under rules-display), so Appendix C.3 *expands* that section rather than adding a new one (`check-player-rules.mjs` REQUIRED_SECTIONS already enforces its presence).
3. Cross-artifact boundary under audit: the outcome-explanation *contract* spans `templates/` (GAME-UI/GAME-RULES/GAME-HOW-TO-PLAY), `docs/**` (UI-INTERACTION + OFFICIAL-GAME-CONTRACT), and the `specs/README.md` index row. No single file owns it; this ticket lands them atomically so the obligation is coherent before 002–012 build against it.
4. FOUNDATIONS principle restated before trusting the spec: §2 (TypeScript/React present only; static files MUST NOT define rule behavior) and §5 (static data is typed content/metadata, never behavior). Per `docs/ENGINE-GAME-DATA-BOUNDARY.md` §5/§11/§12, explanation templates keyed to Rust effects/actions are *allowed* inert static content; the contract this ticket writes must keep outcome copy inert (no selectors/comparisons/tiebreak ladders/conditions/YAML/DSL).
5. Substrate for a deferred enforcement surface: the per-game `UI.md` "Outcome / victory explanation" section requirement and the forbidden-template-content rules this contract defines are the inputs to 002's fail-closed `scripts/check-outcome-explanations.mjs` (§11 fail-closed validation). This ticket adds no validator and no runtime path — no hidden-information leak and no nondeterminism path; enforcement is deferred to 002, which this contract names.

## Architecture Check

1. Per-game game-local rationale + one shared presentation surface (spec D1/D2) beats a generic `engine_core::Outcome`: the kernel stays noun-free and each game owns its decisive-cause vocabulary, while the UI stays singular (no nine bespoke panels). This ticket only writes the doctrine making that mandatory.
2. No backwards-compatibility aliasing/shims: every amendment is an additive new clause; no existing doc contract or template section is renamed or weakened (`RULES.md` stays the formal rule authority; the existing `## Scoring and winning` is expanded, not replaced).
3. `engine-core` stays free of mechanic nouns (untouched); `game-stdlib` unchanged (no promotion). The contract governs `apps/web` presentation + `games/*/docs`, never engine behavior.

## Verification Layers

1. UI-INTERACTION amendment landed → grep-proof `docs/UI-INTERACTION.md` contains the "Outcome / victory explanation surface" section and the §18 acceptance-check lines.
2. Official-game-contract amendment landed → grep-proof `docs/OFFICIAL-GAME-CONTRACT.md` §5 has the "Outcome explanation documentation" subsection and §10 the outcome-explanation web-exposure requirement.
3. Template amendments landed → grep-proof each of `templates/GAME-UI.md` (Outcome / victory explanation), `templates/GAME-RULES.md` (Outcome explanation traceability), `templates/GAME-HOW-TO-PLAY.md` (expanded `## Scoring and winning`).
4. Spec registered → grep-proof `specs/README.md` has the `victory-explanation-shared-surface` row at `Planned` (non-gate UI-infra, mirroring the `rules-display-shared-surface` row).
5. Inert-template alignment → FOUNDATIONS §5 + `docs/ENGINE-GAME-DATA-BOUNDARY.md` §5/§11/§12 manual review: the GAME-UI/GAME-RULES additions forbid comparisons/tiebreak-order/rank-ordering/selectors/YAML/DSL.
6. Doc-link integrity preserved → `node scripts/check-doc-links.mjs`.

## What to Change

### 1. `docs/UI-INTERACTION.md` (Appendix A)

Add the "Outcome / victory explanation surface" section (spec Appendix A.1 body) as a **new section after §15 "Bot explanation UI"**, renumbering the trailing sections (§16 Accessibility baseline → §17, §17 Responsive behavior → §18, §18 UI acceptance check → §19). Add the six acceptance-check lines (spec Appendix A.2) to the (renumbered) UI acceptance check section.

### 2. `docs/OFFICIAL-GAME-CONTRACT.md` (Appendix B)

Add the "Outcome explanation documentation" subsection to §5 (spec Appendix B.1) and the "Outcome explanation requirement" subsection to §10 (spec Appendix B.2).

### 3. Template amendments (Appendix C)

- `templates/GAME-UI.md` — add the "Outcome / victory explanation" section (spec Appendix C.1: terminal-result-variants table, decisive-cause-payload table, per-player-breakdown table, no-leak rules, player-facing copy contract, accessibility/reduced-motion, smoke/tests table).
- `templates/GAME-RULES.md` — add the "Outcome explanation traceability" section (spec Appendix C.2: traceability table, explicitly "not a behavior DSL").
- `templates/GAME-HOW-TO-PLAY.md` — **expand the existing `## Scoring and winning` section** (spec Appendix C.3) with the alignment + content requirements; do not add a duplicate section.

### 4. `specs/README.md` index row

Add the `victory-explanation-shared-surface` row matching the index's real schema, `Status: Planned`, as a non-gate UI-infrastructure spec (mirror the existing `rules-display-shared-surface` row).

## Files to Touch

- `docs/UI-INTERACTION.md` (modify)
- `docs/OFFICIAL-GAME-CONTRACT.md` (modify)
- `templates/GAME-UI.md` (modify)
- `templates/GAME-RULES.md` (modify)
- `templates/GAME-HOW-TO-PLAY.md` (modify)
- `specs/README.md` (modify)

## Out of Scope

- The `scripts/check-outcome-explanations.mjs` checker and CI wiring (002).
- Any Rust rationale, `games/*/docs/UI.md` per-game outcome sections, or golden traces (003–008).
- Any `apps/web` component, template-constants file, type, or board change (009–010).
- Flipping the `specs/README.md` row to `Done` (012, after exit criteria pass).
- Any Rust/engine/WASM change; any new `wasm-api` operation.

## Acceptance Criteria

### Tests That Must Pass

1. `node scripts/check-doc-links.mjs` passes (no broken links introduced).
2. `grep -q 'Outcome / victory explanation' docs/UI-INTERACTION.md templates/GAME-UI.md` succeeds.
3. `grep -q 'victory-explanation-shared-surface' specs/README.md` succeeds and the row reads `Planned`.

### Invariants

1. Every amendment is additive; no existing doc-governed contract or template section is renamed or weakened (`RULES.md` stays the formal rule authority; `## Scoring and winning` is expanded, not replaced).
2. The GAME-UI/GAME-RULES additions forbid behavior-looking content (comparisons, tiebreak-order logic, rank ordering, selectors, YAML, DSL), keeping outcome templates inert static data per FOUNDATIONS §5.

## Test Plan

### New/Modified Tests

1. `None — documentation/template-only ticket; verification is command-based (`check-doc-links.mjs` + grep-proofs) and the contract is exercised by 002's check script and 003–012's coverage.`

### Commands

1. `node scripts/check-doc-links.mjs`
2. `grep -n 'Outcome / victory explanation\|Outcome explanation documentation\|Outcome explanation traceability' docs/UI-INTERACTION.md docs/OFFICIAL-GAME-CONTRACT.md templates/GAME-UI.md templates/GAME-RULES.md`
3. Doc-link check is the correct full-pipeline boundary: no Rust/test surface changes, so `cargo`/web smokes are not the verification boundary for a docs/template diff.

## Outcome

Completed: 2026-06-09

What changed:

- Added the mandatory shared outcome/victory explanation contract to `docs/UI-INTERACTION.md`, including acceptance-check requirements for Rust-owned decisive-cause data, no coaching/counterfactuals, no hidden-info leaks, status-message accessibility, keyboard-accessible breakdowns, color independence, and reduced-motion preservation.
- Added outcome-explanation documentation and web-exposure requirements to `docs/OFFICIAL-GAME-CONTRACT.md`.
- Extended `templates/GAME-UI.md`, `templates/GAME-RULES.md`, and the existing `templates/GAME-HOW-TO-PLAY.md` scoring section with outcome-rationale, traceability, no-leak, and inert-template requirements.
- Registered the victory-explanation shared-surface spec in `specs/README.md` as a non-gate UI-infrastructure spec with `Planned` status.

Deviations from original plan:

- None. The ticket stayed documentation/template/index-only and did not touch Rust, WASM, TypeScript runtime, or engine/game behavior.

Verification results:

- `node scripts/check-doc-links.mjs` passed (`Checked 25 markdown files`).
- `grep -n 'Outcome / victory explanation\|Outcome explanation documentation\|Outcome explanation traceability' docs/UI-INTERACTION.md docs/OFFICIAL-GAME-CONTRACT.md templates/GAME-UI.md templates/GAME-RULES.md` found the required contract/template sections.
- `grep -n 'victory-explanation-shared-surface' specs/README.md` found the `Planned` index row.

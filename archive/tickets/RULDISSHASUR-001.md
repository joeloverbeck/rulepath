# RULDISSHASUR-001: Player-doc contract — GAME-HOW-TO-PLAY template + foundation/area-doc amendments

**Status**: COMPLETED
**Priority**: HIGH
**Effort**: Medium
**Engine Changes**: None — docs and templates only; no Rust/engine, WASM, or behavior surface touched.
**Deps**: None

## Problem

Rulepath has no player-facing per-game rules document and no contract requiring one. Formal `games/<id>/docs/RULES.md` is implementation-facing (rule IDs, validation/visibility notes) and unsuitable as player copy. Before any player docs are authored (RULDISSHASUR-003/-004) or rendered (RULDISSHASUR-005/-006), the obligation must be made permanent: a `GAME-HOW-TO-PLAY.md` template plus amendments to the foundation/area docs that establish `games/<id>/docs/HOW-TO-PLAY.md` as a required official-player doc rendered by the shared web surface. Source: `specs/rules-display-shared-surface.md` §5 (content contract), §12 (doc amendments), §13 (template decision).

## Assumption Reassessment (2026-06-09)

1. The catalog source of truth is `crates/wasm-api/src/lib.rs` `const GAME_*` / `GAME_*_DISPLAY_NAME` pairs surfaced by `list_games()` (nine games); `scripts/check-catalog-docs.mjs` already parses it via `/const GAME_([A-Z0-9_]+):\s*&str\s*=\s*"([^"]+)";/g`. This ticket adds no catalog consumer itself but the contract it writes (catalog-complete player docs) is enforced by RULDISSHASUR-002 against that same source.
2. All amendment targets exist at the sections the spec cites (verified during in-session `/reassess-spec`): `docs/OFFICIAL-GAME-CONTRACT.md` §1 docs set, `docs/UI-INTERACTION.md` §16 accessibility baseline, `docs/ENGINE-GAME-DATA-BOUNDARY.md` §5/§11/§12 (allowed static data / UI-metadata / explanation-template boundaries), `docs/ARCHITECTURE.md` §11 "Game module shape" docs-folder convention, `docs/WASM-CLIENT-BOUNDARY.md` §2 operation groups, `docs/IP-POLICY.md`, `docs/ROADMAP.md`, `templates/README.md`, `specs/README.md`. `templates/GAME-HOW-TO-PLAY.md` does not yet exist (no near-name sibling).
3. Cross-artifact boundary under audit: the player-doc *contract* spans `templates/` (the authoring template), `docs/**` (the permanence amendments), and the `specs/README.md` index row. No single file owns it; this ticket lands all of them atomically so the obligation is coherent before authoring begins.
4. FOUNDATIONS principle restated before trusting the spec: §2 (TypeScript/React present only; static files MUST NOT define rule behavior) and §5 (static data is typed content/metadata, never behavior). Per `docs/ENGINE-GAME-DATA-BOUNDARY.md` §5/§11/§12, authored player help text is *allowed* static presentation content; the contract this ticket writes must keep it inert (no selectors/conditions/triggers/action-schemas/YAML).
5. Substrate for a deferred enforcement surface: the required-section list and the "forbidden content" rules this template/contract define are the inputs to RULDISSHASUR-002's fail-closed `scripts/check-player-rules.mjs` (§11 fail-closed validation). This ticket introduces no validator and no runtime path, so it adds no hidden-information leak and no nondeterminism; enforcement is deferred to RULDISSHASUR-002, which this contract names.

## Architecture Check

1. A separate `GAME-HOW-TO-PLAY.md` template (not an amendment to formal `GAME-RULES.md`) keeps the Rust-owned formal rule contract distinct from public help, so CI and agents can target exactly one public-help document per game without scraping formal docs.
2. No backwards-compatibility aliasing/shims: all amendments are additive new clauses; no existing doc contract is renamed or weakened.
3. `engine-core` stays free of mechanic nouns (untouched); `game-stdlib` unchanged (no promotion). The contract governs `apps/web` presentation + `games/*/docs`, never engine behavior.

## Verification Layers

1. Template carries every required section → grep-proof each of the 11 `## ` headers (At a glance … Source notes for maintainers) present in `templates/GAME-HOW-TO-PLAY.md`.
2. Each foundation/area amendment landed → grep-proof each target doc contains its new `HOW-TO-PLAY.md` clause (e.g. `OFFICIAL-GAME-CONTRACT.md` "Player-facing rules document"; `ARCHITECTURE.md` §11 lists `HOW-TO-PLAY.md`).
3. Spec registered in index → grep-proof `specs/README.md` has the `rules-display-shared-surface.md` row at `Planned`.
4. Static-help-is-not-behavior alignment → FOUNDATIONS §5 + `ENGINE-GAME-DATA-BOUNDARY.md` §5/§11/§12 manual review: the template's "forbidden content" section bans selectors/conditions/triggers/action-schemas/YAML.
5. Doc-link integrity preserved → `node scripts/check-doc-links.mjs`.

## What to Change

### 1. Add `templates/GAME-HOW-TO-PLAY.md`

Create the template body from spec §13.2 (title + inert source/version metadata lines; sections: At a glance, What you can see, Setup, On your turn, Actions, Scoring and winning, Hidden information and reveal timing, Common terms, What this page is not, Source notes for maintainers). Include the maintainer checklist (original prose, no copied text, no strategy, no hidden state, no YAML, no selectors, version matches `RULES.md`).

### 2. Update `templates/README.md`

Add the entry from spec §13.3 listing `GAME-HOW-TO-PLAY.md` as required for every official catalog game, distinct from formal `GAME-RULES.md` and strategy `COMPETENT-PLAYER.md`, following the existing 4-column table format.

### 3. Foundation/area-doc amendments (spec §12)

- `docs/OFFICIAL-GAME-CONTRACT.md` — add the "Player-facing rules document" subsection (spec §12.1) and add `HOW-TO-PLAY.md` to the official-docs checklist + UI-exposure expectations.
- `docs/UI-INTERACTION.md` — add the shared How-to-Play/Rules surface affordance requirement + no-hover-only rule (spec §12.2).
- `docs/ENGINE-GAME-DATA-BOUNDARY.md` — formalize authored player help as allowed inert static UI content (spec §12.3).
- `docs/IP-POLICY.md` — add `HOW-TO-PLAY.md` to public rules-documentation original-prose requirements (spec §12.4).
- `docs/ARCHITECTURE.md` — add `HOW-TO-PLAY.md` to the §11 docs-folder convention + ownership note; record the static-bundled path adds no wasm-api operation (spec §12.5).
- `docs/WASM-CLIENT-BOUNDARY.md` — add the clarifying note that player rules text is static presentation content, not a WASM operation (spec §12.6).
- `docs/ROADMAP.md` — add the optional non-gate maintenance note (spec §12.7).

### 4. Register the spec in `specs/README.md`

Add the index row per spec §12.9, matching the real `Stage | Gate | Spec | Status` schema with `Status: Planned` (the index has no `Proposed` value).

## Files to Touch

- `templates/GAME-HOW-TO-PLAY.md` (new)
- `templates/README.md` (modify)
- `docs/OFFICIAL-GAME-CONTRACT.md` (modify)
- `docs/UI-INTERACTION.md` (modify)
- `docs/ENGINE-GAME-DATA-BOUNDARY.md` (modify)
- `docs/IP-POLICY.md` (modify)
- `docs/ARCHITECTURE.md` (modify)
- `docs/WASM-CLIENT-BOUNDARY.md` (modify)
- `docs/ROADMAP.md` (modify)
- `specs/README.md` (modify)

## Out of Scope

- Authoring any `games/<id>/docs/HOW-TO-PLAY.md` content (RULDISSHASUR-003/-004).
- The copy/check scripts and CI wiring (RULDISSHASUR-002).
- Any `apps/web` component, state, or rendering change (RULDISSHASUR-005/-006).
- Flipping the `specs/README.md` row to `Done` (RULDISSHASUR-008, after exit criteria pass).
- Any Rust/engine/WASM change; any new `wasm-api` operation.

## Acceptance Criteria

### Tests That Must Pass

1. `node scripts/check-doc-links.mjs` passes (no broken links introduced by the amendments).
2. `grep -c '^## ' templates/GAME-HOW-TO-PLAY.md` returns ≥ 10 (all required sections present).
3. `grep -q 'HOW-TO-PLAY.md' docs/OFFICIAL-GAME-CONTRACT.md docs/ARCHITECTURE.md docs/IP-POLICY.md docs/UI-INTERACTION.md docs/ENGINE-GAME-DATA-BOUNDARY.md` succeeds.

### Invariants

1. Every amendment is additive; no existing doc-governed contract is renamed or weakened (`RULES.md` remains the formal rule authority; the web app never renders it as player help).
2. The template forbids behavior-looking content (selectors/conditions/triggers/action-schemas/YAML), keeping player docs inert static data per FOUNDATIONS §5.

## Test Plan

### New/Modified Tests

1. `None — documentation/template-only ticket; verification is command-based (`check-doc-links.mjs` + grep-proofs) and the contract is exercised by RULDISSHASUR-002's check script.`

### Commands

1. `node scripts/check-doc-links.mjs`
2. `grep -n 'HOW-TO-PLAY' docs/OFFICIAL-GAME-CONTRACT.md docs/ARCHITECTURE.md docs/UI-INTERACTION.md docs/ENGINE-GAME-DATA-BOUNDARY.md docs/IP-POLICY.md docs/WASM-CLIENT-BOUNDARY.md docs/ROADMAP.md templates/README.md specs/README.md`
3. Doc-link check is the correct full-pipeline boundary here: no Rust/test surface changes, so `cargo`/web smokes are not the verification boundary for a docs/template diff.

## Outcome

Completed: 2026-06-09

What changed:

- Added `templates/GAME-HOW-TO-PLAY.md` with the required player-facing sections, inert source/version metadata, and maintainer checklist.
- Updated template ordering and index entries so every official catalog game has a required player-facing how-to-play template.
- Added the `HOW-TO-PLAY.md` contract to official-game documentation, UI accessibility, static-data boundary, IP, architecture, WASM-boundary, roadmap, and spec-index surfaces.
- Registered `specs/rules-display-shared-surface.md` in `specs/README.md` as a planned non-gate UI-infrastructure spec.

Deviations from original plan:

- The template uses ASCII hyphens in placeholder prose instead of typographic dashes to match repository editing defaults.
- The roadmap note does not mention the filename directly; it records the cross-game How to Play / Rules maintenance category.

Verification results:

- `node scripts/check-doc-links.mjs` passed (`Checked 25 markdown files`).
- `grep -c '^## ' templates/GAME-HOW-TO-PLAY.md` returned `10`, satisfying the ticket threshold for the second-level required sections.
- `grep -n 'HOW-TO-PLAY' docs/OFFICIAL-GAME-CONTRACT.md docs/ARCHITECTURE.md docs/UI-INTERACTION.md docs/ENGINE-GAME-DATA-BOUNDARY.md docs/IP-POLICY.md docs/WASM-CLIENT-BOUNDARY.md templates/README.md specs/README.md` found the landed contract entries.

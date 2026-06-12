# CARACTPRES-009: Catalog copy hygiene and check-presentation-copy CI guard

**Status**: PENDING
**Priority**: MEDIUM
**Effort**: Medium
**Engine Changes**: Yes (presentation-only) — `apps/web` board copy edits, plus `scripts/` guard and CI wiring; no Rust surface touched
**Deps**: CARACTPRES-002, CARACTPRES-006, CARACTPRES-008

## Problem

Debug vocabulary saturates normal-mode play surfaces: "Rust legal choices", "Rust projection", "Rust modifier", "Rust/WASM supplies card flow…" (`apps/web/src/components/EventFrontierBoard.tsx:181,209,221,228` pre-005/007; sibling boards have analogous headings), violating FOUNDATIONS §7/§11 ("public UI play-first… not debug-dominated") and `docs/UI-INTERACTION.md` §2. Spec D5: sweep normal-mode copy catalog-wide and add a CI guard (`scripts/check-presentation-copy.mjs`) so the vocabulary cannot return — wired only once green (no red-CI window).

## Assumption Reassessment (2026-06-12)

1. Guard precedent and wiring point verified: `scripts/check-catalog-docs.mjs` / `check-doc-links.mjs` run as plain `node scripts/*.mjs` steps in `.github/workflows/gate-1-game-smoke.yml:234-237`; `scripts/check-presentation-copy.mjs` is collision-free. Banned-vocabulary list per spec D5: "Rust", "WASM", "projection", "redacted", "payload", raw snake_case identifiers, internal enum strings — in *normal-mode player-facing* strings only; dev-panel/dev-mode surfaces are exempt.
2. Spec authority: `specs/card-and-action-presentation-shared-surfaces.md` §6 D5, §9 exit criterion 2, §16 risk row (guard must scope to play surfaces; component-scoped scanning preferred over an allowlist file). Deps rationale: 005/007 clean Event Frontier's deck/action sections, 006 cleans Flood Watch, 002 fixes Rust-origin strings, 008 settles audit-mandated panel rewrites — this ticket cleans the remaining boards and wires the guard green.
3. Cross-artifact boundary under audit: the guard's scope contract — which files count as normal-mode play surfaces (board components + shared play components) vs. exempt (dev panel, replay/dev inspectors, test files). The split is encoded in the script's scanned-path set and documented in its header; the guard checks *source literals* in scanned components, which is the right layer for heading/status copy. Rust-origin strings are out of the guard's reach by design — they are proven clean by CARACTPRES-002's unit tests (the per-language split is deliberate, not a coverage gap).
4. FOUNDATIONS §7/§11/§12 restated: "public UI becomes debug-first" is a §12 stop condition; this ticket builds the standing tripwire against it. The guard is deterministic and fail-closed on its scanned set (unknown new board components are auto-included by glob, not opt-in).

## Architecture Check

1. Source-literal scanning of a glob-included component set beats (a) runtime DOM scanning (flaky, needs a browser in the guard) and (b) an allowlist file (spec §16 rejects it — allowlists rot); dev-exempt surfaces are excluded by path convention, and new boards are guarded by default.
2. No backwards-compatibility aliasing/shims: headings change once; no dual copy.
3. `engine-core`/`game-stdlib` untouched; the guard is repo tooling beside the existing `scripts/check-*.mjs` siblings.

## Verification Layers

1. Guard catches violations -> negative test: run the script against a temporary seeded violation (in-test fixture or temp file) and assert non-zero exit with a named file/line diagnostic.
2. Tree is clean at wiring time -> `node scripts/check-presentation-copy.mjs` exits 0 on the post-sweep tree; CI step added in the same diff (no red window).
3. Dev surfaces not false-positived -> guard run asserts zero findings in exempt paths while the dev panel still contains the word "Rust" (proving the exemption works, not that the panel was scrubbed).
4. Copy sweep completeness -> grep-proof across `apps/web/src/components/` play surfaces for the banned list (the same patterns the guard encodes).

## What to Change

### 1. Copy sweep

Rewrite remaining normal-mode debug-flavored headings/status copy across the catalog's board components (e.g. "Rust legal choices" → "Actions", "Rust projection" → neutral panel headings, waiting text naming Rust/WASM → player-facing phrasing). Layout/chrome words stay; meaning is unchanged.

### 2. Guard script

`scripts/check-presentation-copy.mjs`: glob the play-surface component set (board components + shared play components; exempt dev-panel/replay-inspector paths by convention), scan string literals for the banned vocabulary and raw snake_case tokens, print file:line diagnostics, exit non-zero on findings. Document the scope contract and exemption rationale in the script header.

### 3. CI wiring

Add the `node scripts/check-presentation-copy.mjs` step to `.github/workflows/gate-1-game-smoke.yml` beside the sibling guards, in this same diff (tree already green).

## Files to Touch

- `scripts/check-presentation-copy.mjs` (new)
- `.github/workflows/gate-1-game-smoke.yml` (modify)
- `apps/web/src/components/` board components (modify — sweep-discovered set; parent verified; Event Frontier/Flood Watch sections already cleaned by 005/006/007)
- `apps/web/e2e/` smoke expectation strings (modify — as surfaced where smokes assert old headings)

## Out of Scope

- Rust-side string content — CARACTPRES-002 (already landed via Deps).
- Dev panel, replay viewer, and dev-mode inspectors (explicitly exempt surfaces).
- The game picker / match setup screens (spec §3.3 defers picker polish; the guard scans play surfaces only — extending it is the successor spec's call).
- Renaming test IDs or machine-readable tokens.

## Acceptance Criteria

### Tests That Must Pass

1. `node scripts/check-presentation-copy.mjs` exits 0 on the swept tree; seeded-violation negative check exits non-zero with file:line.
2. `npm --prefix apps/web run build && npm --prefix apps/web run smoke:ui && npm --prefix apps/web run smoke:e2e` green after the sweep (smoke heading expectations updated).
3. CI workflow lints clean (the new step runs in gate-1 alongside sibling guards).

### Invariants

1. Normal-mode play surfaces contain no engine/debug vocabulary or raw internal identifiers; the guard enforces this fail-closed for current and future board components (FOUNDATIONS §7/§11; §12 tripwire).
2. The guard never weakens legal-only or no-leak checks — it is additive tooling beside them.

## Test Plan

### New/Modified Tests

1. `scripts/check-presentation-copy.mjs` self-test path (seeded-violation negative check — inline `--self-test` flag or a sibling test file per the existing guard scripts' convention).
2. `apps/web/e2e/*.smoke.mjs` — updated heading expectations for swept boards (as surfaced).

### Commands

1. `node scripts/check-presentation-copy.mjs`
2. `npm --prefix apps/web run build && npm --prefix apps/web run smoke:e2e`
3. Narrow boundary rationale: copy + tooling only; Rust strings were proven clean at CARACTPRES-002.

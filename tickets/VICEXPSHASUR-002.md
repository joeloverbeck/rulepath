# VICEXPSHASUR-002: Outcome coverage checker + static-template guard

**Status**: PENDING
**Priority**: HIGH
**Effort**: Medium
**Engine Changes**: None — adds a Node CI hygiene script (`scripts/check-outcome-explanations.mjs`) and a `gate-1-game-smoke.yml` step; no Rust/engine, WASM, or behavior surface.
**Deps**: VICEXPSHASUR-001

## Problem

The outcome-explanation contract (001) is inert without a fail-closed guard. Nothing prevents a catalog game from shipping web-exposed without an outcome rationale (docs, `client.ts` type, template keys). Add `scripts/check-outcome-explanations.mjs` — a deterministic, catalog-complete, string-based guard analogous to `scripts/check-player-rules.mjs` — and wire it into the gate-1 hygiene lane so omission fails CI. Source: `specs/victory-explanation-shared-surface.md` §12.1, §15.2.

## Assumption Reassessment (2026-06-09)

1. The catalog source of truth is `crates/wasm-api/src/lib.rs` `const GAME_*: &str = "..."`, parsed by `scripts/check-player-rules.mjs:26` via `/const GAME_([A-Z0-9_]+):\s*&str\s*=\s*"([^"]+)";/g` (`CATALOG_RE`); this checker reuses that exact enumeration. `scripts/check-catalog-docs.mjs:27` carries `NON_GAME_SMOKE = new Set(["shell", "a11y-noleak", "rules-display"])` (the smoke-registration surface 011 extends, not this ticket).
2. Spec §12.1 step 5 was corrected during `/reassess-spec`: `scripts/check-player-rules.mjs` already lists `"Scoring and winning"` in `REQUIRED_SECTIONS` (line 38) and enforces it per catalog game, so this checker MUST NOT duplicate that assertion — it defers HOW-TO-PLAY coverage to `check-player-rules.mjs` and validates only the outcome-specific surfaces (UI.md outcome section, RULES.md rule IDs, `client.ts` rationale type, template-key coverage, forbidden content).
3. Cross-artifact boundary under audit: the checker reads `games/<id>/docs/UI.md` + `games/<id>/docs/RULES.md` + `apps/web/src/wasm/client.ts` + the static templates file (`apps/web/src/components/outcomeExplanationTemplates.ts`, created by 009), and is invoked from `.github/workflows/gate-1-game-smoke.yml` (alongside `check-doc-links` L167 / `check-catalog-docs` L170 / `check-player-rules` L175).
4. FOUNDATIONS principle restated: §11 fail-closed validation — the guard must be deterministic and blocking (non-zero exit on any missing/forbidden case), and §5 — it blocks behavior-looking template content (comparisons, tiebreak-order logic, rank ordering, selectors, YAML, DSL fragments).
5. This ticket *is* a §11 fail-closed enforcement surface. Confirm it introduces no leak and no nondeterminism: it performs pure file reads and string assertions, emits only pass/fail plus offending file/section names (never game state, never hidden values), and uses no RNG/wall-clock — so its output is deterministic and viewer-state-free.

## Architecture Check

1. A conservative string-based checker mirroring `check-player-rules.mjs` (rather than a typed AST/payload parser) is the right first cut: it lands fail-closed coverage now and can tighten as the rationale payloads settle (spec §12.1 closing note), without coupling CI to TS type internals.
2. No backwards-compatibility shims: a brand-new script + one additive workflow step; no existing checker is modified.
3. `engine-core`/`game-stdlib` untouched; the guard inspects docs/types/templates, it implements no behavior and decides no legality.

## Verification Layers

1. Catalog enumeration correct → the script parses `crates/wasm-api/src/lib.rs` with the same `CATALOG_RE` as `check-player-rules.mjs`; grep-proof the regex is present in the new script.
2. Fail-closed on missing coverage → negative test: temporarily remove a game's UI.md outcome section in a scratch copy and confirm the script exits non-zero naming that game (documented in Test Plan, not committed).
3. No HOW-TO-PLAY duplication → grep-proof the script does NOT assert `"Scoring and winning"` (that assertion stays in `check-player-rules.mjs`).
4. Forbidden-content rule → the script flags YAML front matter / comparison-operator / tiebreak-ladder tokens in the templates file (§5/§11).
5. CI wired without a hidden red-window surprise → grep-proof `gate-1-game-smoke.yml` runs `node scripts/check-outcome-explanations.mjs`; Step-6 flags the expected red window until coverage lands (003–010).
6. Determinism → manual review: pure file reads, no RNG/`Date`/network.

## What to Change

### 1. Add `scripts/check-outcome-explanations.mjs`

Enumerate the catalog from `crates/wasm-api/src/lib.rs` (`CATALOG_RE`). For every catalog game assert: (a) `games/<id>/docs/UI.md` contains an "Outcome / victory explanation" section naming terminal result variants, decisive-cause variants, per-player breakdown fields, hidden-info redaction rules, `RULES.md` rule IDs, and web-smoke coverage; (b) `games/<id>/docs/RULES.md` documents scoring/terminal conditions with stable rule IDs; (c) `apps/web/src/wasm/client.ts` carries an outcome rationale type/field for the game; (d) the static templates file covers every `template_key` named by the game docs. Fail if a catalog game lacks the rationale contract, and fail if forbidden patterns (TS outcome/comparison logic, YAML/DSL, hidden-info leak markers) appear in the templates file. **Do not** assert the HOW-TO-PLAY "Scoring and winning" section — defer to `check-player-rules.mjs`.

### 2. Wire into `gate-1-game-smoke.yml`

Add a `node scripts/check-outcome-explanations.mjs` step in the hygiene lane next to the existing `check-catalog-docs` / `check-player-rules` steps.

## Files to Touch

- `scripts/check-outcome-explanations.mjs` (new)
- `.github/workflows/gate-1-game-smoke.yml` (modify)

## Out of Scope

- Authoring any `games/<id>/docs/UI.md` outcome section or `client.ts` type (003–010) — this ticket only checks for them.
- The static templates file itself (`outcomeExplanationTemplates.ts`, created by 009).
- Registering the e2e smoke in `NON_GAME_SMOKE` / README / package.json (011).
- **Expected red-CI window**: once wired, this guard fails until the per-game coverage (003–008) and `client.ts`/templates (009–010) land. That mid-gate red state is resolved by 010/011 and confirmed green at 012 — flagged so a reviewer reading PRs in order isn't surprised; not a defect of this ticket.

## Acceptance Criteria

### Tests That Must Pass

1. `node scripts/check-outcome-explanations.mjs` runs deterministically and exits non-zero while coverage is incomplete, naming each game missing its outcome contract (correct fail-closed behavior at this point in the gate).
2. `grep -q 'CATALOG_RE\|const GAME_' scripts/check-outcome-explanations.mjs` succeeds (catalog enumeration present).
3. `grep -q 'check-outcome-explanations' .github/workflows/gate-1-game-smoke.yml` succeeds.

### Invariants

1. The guard is fail-closed and blocking: any catalog game lacking an outcome rationale contract, or any forbidden template content, causes a non-zero exit (FOUNDATIONS §11).
2. The guard emits only pass/fail plus offending file/section identifiers — never game state or hidden values — and is deterministic (no RNG/wall-clock/network), so it introduces no leak path (§11 no-leak firewall).

## Test Plan

### New/Modified Tests

1. `scripts/check-outcome-explanations.mjs` — the guard itself; its negative behavior (non-zero on missing coverage) is the test.
2. `None additional — Node hygiene script verified by direct invocation + the scratch-copy negative test described in Verification Layers (not committed).`

### Commands

1. `node scripts/check-outcome-explanations.mjs` (expected non-zero until 003–010 land; assert it names the uncovered games)
2. `node scripts/check-doc-links.mjs` (regression — no broken links from the workflow edit)
3. A `cargo`/web-build run is not the verification boundary: this is a Node CI guard over docs/types/templates, exercised by direct invocation.

# RULDISSHASUR-002: Static-copy delivery + CI coverage/staleness guard scripts

**Status**: PENDING
**Priority**: HIGH
**Effort**: Medium
**Engine Changes**: None — Node build/check scripts under `scripts/` + `apps/web/package.json` script wiring; no Rust/engine, WASM, or behavior surface.
**Deps**: RULDISSHASUR-001

## Problem

The shared rules surface delivers authored player docs as static web assets. Two deterministic Node scripts are needed: `scripts/copy-player-rules.mjs` (copy each `games/<id>/docs/HOW-TO-PLAY.md` to `apps/web/public/rules/<id>.md`) and `scripts/check-player-rules.mjs` (fail-closed CI guard asserting catalog-complete, valid, non-stale, behavior-free player docs). Source: `specs/rules-display-shared-surface.md` §6.3 (delivery design), §10.1 (coverage check), §10.2 (build-copy check).

## Assumption Reassessment (2026-06-09)

1. `scripts/check-catalog-docs.mjs` parses the catalog with `/const GAME_([A-Z0-9_]+):\s*&str\s*=\s*"([^"]+)";/g` against `crates/wasm-api/src/lib.rs` (nine `GAME_*` / `GAME_*_DISPLAY_NAME` pairs); the new scripts reuse this exact mechanical-source pattern. Each `games/<id>/docs/RULES.md` declares its version on line 9 as `Rules version: \`<token>\``.
2. Spec §6.3/§10.1/§10.2 define the script responsibilities; the required-section list and forbidden-content rules come from the `templates/GAME-HOW-TO-PLAY.md` contract landed in RULDISSHASUR-001.
3. Cross-artifact boundary under audit: the check spans three surfaces — the `wasm-api` catalog const (source of truth), `games/<id>/docs/HOW-TO-PLAY.md` (+ `RULES.md` for the version token), and the generated `apps/web/public/rules/<id>.md` assets (byte-identical drift target).
4. FOUNDATIONS principle restated: §5 (static data is not behavior) and §11 (validation is deterministic, fail-closed, blocking). The check MUST reject YAML front matter, raw `<script>`/iframe/embed/object/event-handler attributes, and behavior-looking structured headings — keeping player docs inert.
5. Fail-closed enforcement surface (this ticket IS it): `scripts/check-player-rules.mjs` must be deterministic, blocking (non-zero exit on any failure), and treat all checks as blockers (no warn-only path). It reads only static authored docs and the public catalog — no match state, no viewer ID — so it introduces no hidden-information leak and no nondeterminism.
6. Mismatch + correction (from `/reassess-spec` finding M4): rules-version tokens are NOT uniformly derived from `game_id` — five use underscore form (`race_to_n-rules-v1`), four use hyphen form (`high-card-duel-rules-v1`, `token-bazaar-rules-v1`, `secret-draft-rules-v1`, `poker-lite-rules-v1`). The staleness check MUST extract the `RULES.md` token and compare it *literally* against the `HOW-TO-PLAY.md` "Formal rules version checked" value; it MUST NOT reconstruct the token from `game_id`.

## Architecture Check

1. Plain deterministic Node scripts (mirroring `check-catalog-docs.mjs`) keep the player-doc contract enforceable in CI without expanding `wasm-api` or adding runtime behavior; static copy is the lowest-surface delivery for inert help text.
2. No backwards-compatibility aliasing/shims; the scripts are new and additive.
3. `engine-core` untouched (no mechanic noun); `game-stdlib` unchanged. The scripts read the catalog const as text, never the engine.

## Verification Layers

1. Fail-closed on a missing/invalid doc → run `check-player-rules.mjs` against a temp fixture with a missing section / missing version line; assert non-zero exit and a named failing game ID.
2. Catalog parse fidelity → grep-proof the scripts use the same `const GAME_(...)` regex as `check-catalog-docs.mjs`; assert all nine IDs+display names resolve.
3. Byte-identical copy → run `copy-player-rules.mjs` then `diff games/<id>/docs/HOW-TO-PLAY.md apps/web/public/rules/<id>.md` (exit 0) for a present doc.
4. Behavior-looking content blocked → fixture containing `---` front matter / `<script>` / `<iframe>` fails the check.
5. Literal version compare (no derive) → fixture where `HOW-TO-PLAY.md` cites a `game_id`-derived token that differs from the `RULES.md` literal token fails the staleness check.

## What to Change

### 1. `scripts/copy-player-rules.mjs`

Parse catalog IDs from `crates/wasm-api/src/lib.rs`; for each, read `games/<id>/docs/HOW-TO-PLAY.md`, fail if missing/empty/lacking required headings or the `Formal rules version checked` line or containing forbidden raw HTML; copy to `apps/web/public/rules/<id>.md`. Optionally emit `apps/web/public/rules/manifest.json` with inert coverage metadata only (`game_id`, `display_name`, `rules_asset_path`, `source_rules_version_checked`, content hash) — never action schemas, selectors, conditions, seed/match data.

### 2. `scripts/check-player-rules.mjs`

Parse catalog IDs + display names; assert every game has `HOW-TO-PLAY.md`; assert each declares the catalog `game_id`/display name; extract the `RULES.md` version token (line 9, literal) and assert `HOW-TO-PLAY.md` cites the same literal token; assert required sections present; assert hidden-info games (`high_card_duel`, `secret_draft`, `poker_lite`) carry a real (non-"Not applicable") `Hidden information and reveal timing` section and perfect-info games mark it not applicable; assert generated asset is byte-identical to source (run copy in dry-run and fail on drift); assert no raw `<script>`/iframe/embed/object/event-handler attributes and no YAML front matter. Print failing game IDs + invalid sections; exit non-zero on any failure.

### 3. `apps/web/package.json`

Add `"build:rules": "node ../../scripts/copy-player-rules.mjs"` and `"check:rules": "node ../../scripts/check-player-rules.mjs"`, and run `build:rules` before `vite build` in the existing `build` script.

## Files to Touch

- `scripts/copy-player-rules.mjs` (new)
- `scripts/check-player-rules.mjs` (new)
- `apps/web/package.json` (modify)

## Out of Scope

- Authoring any `HOW-TO-PLAY.md` content or committing generated assets (RULDISSHASUR-003/-004; full-catalog green and the CI workflow step land in -004).
- Any `apps/web` component/state/render change (RULDISSHASUR-005/-006).
- Editing `crates/wasm-api/src/lib.rs` (catalog read-only).
- Any new `wasm-api` operation.

## Acceptance Criteria

### Tests That Must Pass

1. `node scripts/check-player-rules.mjs` exits non-zero against a fixture with a missing/invalid doc and names the failing game ID (negative path; full-catalog green arrives in RULDISSHASUR-004).
2. `node scripts/copy-player-rules.mjs` produces an `apps/web/public/rules/<id>.md` byte-identical to a present source doc.
3. `npm --prefix apps/web run check:rules` and `npm --prefix apps/web run build:rules` resolve and run the new scripts.

### Invariants

1. The check is deterministic, blocking, and fail-closed: all conditions are blockers (no warn-only override), exit code non-zero on any failure.
2. Version comparison is literal (`RULES.md` token vs `HOW-TO-PLAY.md` cited token), never reconstructed from `game_id`; the scripts read only static docs + the public catalog (no hidden state).

## Test Plan

### New/Modified Tests

1. `scripts/check-player-rules.mjs` — its own negative-path fixtures (missing section, version drift, forbidden tag, behavior-looking heading) double as the validator's self-tests.
2. `apps/web/public/rules/.gitkeep` (or first generated asset) — establishes the copy target directory.

### Commands

1. `node scripts/check-player-rules.mjs` (negative path until -004) and `node scripts/copy-player-rules.mjs`
2. `node scripts/check-catalog-docs.mjs` (confirm the reused catalog-parse pattern stays consistent)
3. The web build (`npm --prefix apps/web run build`) is the integration boundary once docs exist (RULDISSHASUR-004); at this ticket the narrower script-level run is the correct boundary because no docs are authored yet.

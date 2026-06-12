# CARACTPRES-004: Frontier Control UiMetadata audit

**Status**: PENDING
**Priority**: LOW
**Effort**: Small
**Engine Changes**: Yes — `games/frontier_control` (possible additive `ui` view field), or none if the audit records a sufficient-as-is exception
**Deps**: None

## Problem

`frontier_control` is the third newest-game crate that skipped the established `ui.rs` UiMetadata pattern: its `PublicView` has no `pub ui` field (verified — `games/frontier_control/src/visibility.rs` has no `pub ui`; `src/ui.rs` is a placeholder), so any panel/heading copy its board needs is hardcoded in TypeScript. Spec WB4: audit whether Frontier Control's board actually hardcodes gameplay-meaningful copy that belongs in the Rust channel; adopt the field where it does, or record explicitly why the existing label projection suffices.

## Assumption Reassessment (2026-06-12)

1. `games/frontier_control/src/ui.rs` exists as a placeholder and `src/visibility.rs` projects no `ui` field — verified by grep this session ("frontier_control: no ui field"). The board component is `apps/web/src/components/FrontierControlBoard.tsx` (registered in `apps/web/src/main.tsx` per-game renderer dispatch).
2. Spec authority: `specs/card-and-action-presentation-shared-surfaces.md` §6 D2 and §8 WB4 — explicitly an audit ticket: "adopt the field where its board currently hardcodes copy, or record explicitly why its existing label projection suffices." The pattern to adopt (if adopted) is `games/token_bazaar/src/ui.rs` / CARACTPRES-001.
3. Cross-artifact boundary under audit: the frontier_control view JSON into `apps/web/src/wasm/client.ts`. If adoption happens, the extension is additive-only; if not, no contract changes at all.
4. FOUNDATIONS §2/§7 motivate the audit: TS must not author gameplay-meaningful copy; but site labels are already Rust-projected for this game (its sites carry `label` fields consumed by the board), so the audit may legitimately conclude the existing projection suffices — the spec pre-authorizes the exception outcome.

## Architecture Check

1. Audit-then-adopt beats unconditional adoption: frontier_control already projects site labels, and forcing an empty `UiMetadata` struct in for symmetry would be speculative structure (YAGNI); the spec's adoption criterion is hardcoded *gameplay-meaningful* copy, not pattern symmetry.
2. No backwards-compatibility aliasing/shims either way.
3. `engine-core` and `game-stdlib` untouched (§3/§4).

## Verification Layers

1. Audit completeness (every hardcoded string in `FrontierControlBoard.tsx` classified: gameplay-meaningful vs. layout/chrome) -> codebase grep-proof over the component's string literals, classification recorded in the audit note.
2. If adopted: additive view extension stays deterministic -> `replay-check --game frontier_control --all` + regenerated fixtures.
3. If exception: the recorded rationale names the existing Rust label projection as the sufficient source -> manual review of the audit record in `games/frontier_control/docs/UI.md`.

## What to Change

### 1. Audit

Enumerate `FrontierControlBoard.tsx` player-facing string literals; classify each as (a) gameplay-meaningful copy that should come from Rust, or (b) layout/chrome (panel scaffolding, generic words) that presentation legitimately owns. Debug-flavored headings are NOT this ticket — CARACTPRES-009 sweeps those catalog-wide.

### 2. Adopt or record exception

- If (a) is non-empty: add `UiMetadata` + `ui_metadata()` to `src/ui.rs`, project `pub ui` in `src/visibility.rs` (additive), regenerate fixtures/traces, and consume the new labels in the board.
- If (a) is empty: record the audit outcome and rationale in `games/frontier_control/docs/UI.md` (one short section: strings audited, classification, why existing label projection suffices).

## Files to Touch

- `games/frontier_control/docs/UI.md` (modify — audit record either way)
- `games/frontier_control/src/ui.rs` (modify — only on adoption)
- `games/frontier_control/src/visibility.rs` (modify — only on adoption)
- `apps/web/src/components/FrontierControlBoard.tsx` (modify — only on adoption)
- `apps/web/src/wasm/client.ts` (modify — only on adoption; coordinate with CARACTPRES-005/006 which also touch this file)
- `games/frontier_control/tests/` (modify — traces/fixtures as surfaced, only on adoption)

## Out of Scope

- Catalog-wide debug-vocabulary copy hygiene — CARACTPRES-009.
- Action-presentation audit — CARACTPRES-008.
- Any visibility-contract change beyond an additive `ui` field.
- Event deck surfaces (frontier_control has no card flow; the DeckFlowPanel does not apply).

## Acceptance Criteria

### Tests That Must Pass

1. On adoption: `cargo test -p frontier_control && cargo run -p replay-check -- --game frontier_control --all && cargo run -p fixture-check -- --game frontier_control` green.
2. Either way: `npm --prefix apps/web run smoke:ui` green (board behavior unchanged or label-equivalent).

### Invariants

1. TS authors no gameplay-meaningful copy for frontier_control after this ticket — either Rust supplies it or the audit record proves none was TS-authored (FOUNDATIONS §2/§7).
2. Any view change is additive-only; no hidden-info or determinism surface moves (§11).

## Test Plan

### New/Modified Tests

1. On adoption: `games/frontier_control/src/ui.rs` inline tests (labels, no-debug-vocabulary) + trace regeneration.
2. On exception: `None — documentation-only ticket; verification is command-based and existing pipeline/test coverage is named in Assumption Reassessment.`

### Commands

1. `grep -n '"' apps/web/src/components/FrontierControlBoard.tsx` — audit enumeration input.
2. `cargo test -p frontier_control && cargo run -p replay-check -- --game frontier_control --all` (adoption path) or `node scripts/check-doc-links.mjs` (exception path, doc edit only).
3. Narrow boundary rationale: single-game audit; catalog-wide guards land in CARACTPRES-009/010.

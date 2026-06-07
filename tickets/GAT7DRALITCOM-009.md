# GAT7DRALITCOM-009: Public view, visibility projection & UI metadata

**Status**: PENDING
**Priority**: HIGH
**Effort**: Medium
**Engine Changes**: Yes — `games/draughts_lite/src/visibility.rs` (perfect-information view projection), `games/draughts_lite/src/ui.rs` (viewer-facing UI metadata: cell/piece labels, board presentation tokens, accessibility metadata), `src/lib.rs` (export).
**Deps**: 005

## Problem

The web board renders only Rust-projected view data (FOUNDATIONS §2/§7). This ticket projects the perfect-information public view of the state (all 64 cells including non-playable, piece ownership/kind, terminal outcome) and supplies the viewer-facing UI metadata (cell ids, piece labels, board presentation tokens, accessibility names) the renderer consumes. Draughts Lite is perfect-information, so the public and private views are equivalent; the visibility tests prove no leak even though there is no hidden state — the habit is the contract.

## Assumption Reassessment (2026-06-07)

1. `games/draughts_lite/src/state.rs` (GAT7DRALITCOM-004) is the source the view projects; `games/directional_flip/src/visibility.rs` and `src/ui.rs` are the structural precedents (visibility projection + Rust-owned UI metadata file). `games/draughts_lite/src/rules.rs` (005) supplies any legal-status facts the view annotates.
2. The view/visibility contract is fixed by spec §R11 "Visibility" (perfect-information; public == private or private explicitly N/A) and §R14 "Board rendering" (all 64 cells incl. non-playable, playable vs non-playable, ownership, men vs kings, terminal). `docs/ENGINE-GAME-DATA-BOUNDARY.md` defines the public/private view contract this conforms to.
3. Cross-artifact boundary under audit: the public view + UI metadata is consumed by WASM (016, view path), the web renderer (018), and the no-leak visibility tests (013). The `ui.rs` UI-metadata file is viewer-facing Rust output, distinct from the TypeScript renderer (which only presents it).
4. FOUNDATIONS §2/§11 motivate this ticket: restate before coding — Rust owns view projection; the browser payload is already viewer-safe. Non-playable cells are rendered as board cells but can never hold pieces or be legal targets (spec §R8) — the view marks them, the UI does not compute playability beyond rendering Rust/public board data.
5. No-leak firewall enforcement surface (§11): even though Draughts Lite is perfect-information, the visibility test must prove the public view, previews, and UI metadata expose no engine internals, RNG state, or stale-token internals — establishing the no-leak habit and guarding against a future hidden-info game inheriting this code.

## Architecture Check

1. Projecting the view + UI metadata in Rust (rather than letting the renderer assemble it from raw state) keeps TypeScript presentation-only and gives one viewer-safe payload contract.
2. No backwards-compatibility shims; new projection + metadata logic.
3. `engine-core` stays noun-free (§3) — board/piece view structs are game-local; the projection conforms to the generic public/private view contract. `game-stdlib` is consumed only for the rule-agnostic coordinate helper if promoted (§4).

## Verification Layers

1. View completeness -> unit test: the public view exposes all 64 cells with playable/ownership/kind/terminal, matching the state.
2. Perfect-information equivalence -> visibility test: public view == private view (or private explicitly N/A), per spec §R11.
3. No-leak -> no-leak visibility test: view + UI-metadata payloads contain no engine internals / RNG / stale-token state (FOUNDATIONS §11).
4. UI metadata accessibility -> unit/manual review: cell/piece accessibility names exist for renderer + screen-reader consumption (spec §R15).

## What to Change

### 1. Visibility projection

In `visibility.rs`, project the perfect-information public view from state: 64 cells with playable flag, occupant (owner + man/king) or empty, and terminal outcome. Assert public == private equivalence (or mark private N/A) per the existing game convention.

### 2. UI metadata

In `ui.rs`, supply viewer-facing UI metadata: stable cell ids (`rNcM`), piece labels, board presentation tokens, and accessibility names — the data the renderer (018) and screen-reader announcements (019) consume. No legality computation; legal-status cues come from the action tree (006).

## Files to Touch

- `games/draughts_lite/src/visibility.rs` (new)
- `games/draughts_lite/src/ui.rs` (new)
- `games/draughts_lite/src/lib.rs` (modify — export visibility + ui modules)

## Out of Scope

- The TypeScript renderer itself (GAT7DRALITCOM-018 — consumes this view/metadata).
- Action-tree legal-status metadata (GAT7DRALITCOM-006).
- Effects (GAT7DRALITCOM-008).
- Any TypeScript-side playability/legality computation (forbidden; FOUNDATIONS §2, spec §R14).

## Acceptance Criteria

### Tests That Must Pass

1. `cargo test -p draughts_lite` — view-completeness + perfect-information-equivalence + no-leak tests pass.
2. `bash scripts/boundary-check.sh` — `engine-core` stays noun-free.

### Invariants

1. The public view is viewer-safe and complete; non-playable cells are marked, never legal (FOUNDATIONS §11; spec §R8/§R14).
2. Public and private views are equivalent (perfect information); no hidden/internal state leaks (FOUNDATIONS §11; spec §R11).

## Test Plan

### New/Modified Tests

1. `games/draughts_lite/tests/visibility.rs` — view completeness, perfect-information equivalence, no-leak assertions (expanded with the full suite in GAT7DRALITCOM-013).

### Commands

1. `cargo test -p draughts_lite visibility`
2. `cargo test -p draughts_lite && bash scripts/boundary-check.sh`
3. Crate-scoped visibility tests are the correct boundary; browser-payload no-leak is re-checked end-to-end in the a11y/no-leak smoke (GAT7DRALITCOM-019).

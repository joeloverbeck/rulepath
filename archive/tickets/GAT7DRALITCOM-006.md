# GAT7DRALITCOM-006: Compound action tree — origin/landing/continuation phases, segment vocabulary, choice metadata, previews

**Status**: COMPLETED
**Priority**: HIGH
**Effort**: Large
**Engine Changes**: Yes — `games/draughts_lite/src/actions.rs` (recursive action tree: origin selection → quiet/jump landing → forced continuation, multi-segment action paths, choice metadata, previews), `src/lib.rs` (export).
**Deps**: 005

## Problem

This is the architectural center of Gate 7: proving the existing recursive action-tree model carries a compound, multi-step, forced-continuation move without a checkers-specific engine concept. The UI and bots act only through the Rust-generated tree (FOUNDATIONS §2/§7). This ticket builds the nested tree — root choices are legal origins, each origin has quiet/first-jump landing children, jump children carry continuation children when continuation is mandatory, and a leaf action path commits the move — with stable segment names (`from/rNcM`, `to/rNcM`, `jump/rNcM`), per-choice metadata, and viewer-safe previews. There is no separate `commit`/`end-turn` segment.

## Assumption Reassessment (2026-06-07)

1. `crates/engine-core/src/action.rs` defines `ActionChoice { … next: Option<Box<ActionNode>> … }` and `ActionTree` (verified `action.rs:33,40,13`); `crates/engine-core/src/lib.rs:59` defines `ActionPath { segments: Vec<String> }`. The recursive shape and multi-segment path already exist — no new core action concept is required (spec §R9 "Architectural decision"). `games/directional_flip/src/actions.rs` is the precedent for game-local segments + metadata within the generic contract.
2. The tree phases, segment conventions, choice metadata, and preview contents are fixed by spec §R9 (phase table; segments `from/`, `to/`, `jump/`; metadata list; preview minimum) and the diagnostics list (§R9 "Invalid path diagnostics", consumed by validation in 007).
3. Cross-artifact boundary under audit: the action tree / action-path / preview shape is consumed by validation (007), WASM export (016), the web renderer (018), bots (012), and replay (010). The segment strings are part of the replay contract and must stay stable absent a documented migration (spec §R9 "Segment conventions"). The tree is built by calling the GAT7DRALITCOM-005 legality functions — not a re-implementation — so tree and rules cannot diverge.
4. FOUNDATIONS §2/§7 motivate this ticket: restate before coding — Rust owns legal action generation and previews; the UI traverses the tree and never synthesizes a path Rust did not expose. Continuation children appear only when rules-core reports a mandatory continuation for the same piece; quiet children are absent when any capture exists.
5. No-leak + determinism enforcement surface (§11): choice metadata and previews are destined for browser payloads — confirm they expose only perfect-information facts (cell ids, piece kind, capture flags, captured cell/piece id, promotion flag, labels) and no engine internals or stale-token state. Tree ordering reuses the §R11 canonical order from GAT7DRALITCOM-005, so action-tree hashes are deterministic (spec §R10 "deterministic action-tree hashes").
6. Schema extension under audit: this extends the generic `engine-core` `ActionTree`/`ActionChoice`/`ActionPath` contract with game-local segment strings and metadata. The extension is additive and game-local (no kernel change); consumers (007/010/012/016/018) read the recursive `next` chain and the segment list. Confirm the metadata is carried as game-local typed structs / string metadata on the choice (the `directional_flip` precedent), not as new kernel fields.

## Architecture Check

1. Representing a complete move as a root-to-leaf segment path over the existing recursive `ActionChoice.next` (rather than inventing a draughts move type in the kernel) keeps `engine-core` generic and proves the action-tree model scales to compound moves — the gate's thesis.
2. No backwards-compatibility shims; new generation logic.
3. `engine-core` stays noun-free (§3) — `from`/`to`/`jump` segments and capture/promotion metadata are game-local strings/structs within the generic action-tree contract; no draughts noun enters the kernel.

## Verification Layers

1. Phase structure -> rule tests: root choices are legal origins only; origin children are legal landings only; jump children carry continuation children only when mandatory; a leaf path is a complete move.
2. Mandatory-capture shaping -> rule test: when any capture exists, no origin offers a quiet landing child.
3. Segment stability + ordering -> rule test + golden trace (landed in 014): segments are `from/`/`to/`/`jump/` in canonical order, stable across runs.
4. Preview == apply -> property test (expanded in 013): a jump choice's previewed captured cell/piece equals what applying that segment removes.
5. Metadata no-leak -> no-leak visibility test: choice metadata/preview payloads contain only viewer-safe perfect-information fields (FOUNDATIONS §11).

## What to Change

### 1. Recursive action-tree generation

In `actions.rs`, build the tree from a state by calling GAT7DRALITCOM-005 legality: root `from/rNcM` choices per legal origin; `to/rNcM` quiet children (absent when any capture exists); `jump/rNcM` children with recursive `next` continuation nodes for mandatory same-piece continuation; leaf nodes where no continuation is legal or a man promotes during capture.

### 2. Choice metadata & previews

Attach per-choice metadata (phase, cell id, piece id for origins, piece kind, active seat, capture-mandatory flag, is-capture flag, captured cell/piece id, would-promote flag, forced-by-continuation flag, display + accessibility labels, styling tags) and viewer-safe previews (highlighted origin/landing, captured piece/cell, forced-continuation hint, promotion hint), per spec §R9.

## Files to Touch

- `games/draughts_lite/src/actions.rs` (new)
- `games/draughts_lite/src/lib.rs` (modify — export the actions module)

## Out of Scope

- Command validation / atomic apply / diagnostic emission (GAT7DRALITCOM-007 — consumes this tree's leaf-path contract).
- Semantic effects (GAT7DRALITCOM-008), public view projection (009).
- Legality computation itself (owned by GAT7DRALITCOM-005; this ticket consumes it).
- Any client-side action synthesis or preview computation (forbidden; FOUNDATIONS §2, spec §R14).

## Acceptance Criteria

### Tests That Must Pass

1. `cargo test -p draughts_lite` — action-tree phase/ordering/metadata tests pass, including a preview-vs-apply assertion over representative states.
2. `bash scripts/boundary-check.sh` — `engine-core` stays noun-free.

### Invariants

1. Every legal move is a Rust-generated root-to-leaf segment path; the UI never invents a path (FOUNDATIONS §2; spec §R9).
2. Segment strings (`from/`,`to/`,`jump/`) are stable and the tree order is deterministic (replay/hash-stable; spec §R9/§R10).
3. Choice metadata/previews expose only viewer-safe perfect-information facts (FOUNDATIONS §11).

## Test Plan

### New/Modified Tests

1. `games/draughts_lite/tests/rules.rs` (or inline in `actions.rs`) — action-tree phase, ordering, metadata, and preview-vs-apply property (expanded in GAT7DRALITCOM-013; cross-surface golden trace in 014).

### Commands

1. `cargo test -p draughts_lite actions`
2. `cargo test -p draughts_lite && bash scripts/boundary-check.sh`
3. Crate-scoped tests are correct here; the multi-segment WASM-exported golden trace round-trip is GAT7DRALITCOM-016 (needs the WASM boundary).

## Outcome

Completed: 2026-06-07

What changed:
- Added `games/draughts_lite/src/actions.rs` with recursive action-tree generation from the rules-core legal move surface.
- Added stable segment vocabulary: `from/rNcM`, `to/rNcM`, and `jump/rNcM`.
- Added origin, quiet landing, jump landing, and forced-continuation child phases with merged root-to-leaf paths.
- Added viewer-safe choice metadata for phase, cell, piece id, piece kind, active seat, mandatory capture, capture details, continuation availability, and promotion.
- Added action-tree tests for ordering, mandatory-capture shaping, continuation nodes, preview metadata matching rules-core capture details, promotion leaves, and inactive/terminal empty trees.

Deviations from original plan:
- Preview data is represented through `ActionPreview::Available` plus metadata fields, matching the current generic action-choice surface without adding kernel fields.

Verification:
- `cargo test -p draughts_lite actions` passed (6 focused action-tree tests).
- `cargo test -p draughts_lite` passed (27 unit tests).
- `cargo fmt --all --check` passed.
- `bash scripts/boundary-check.sh` passed (`engine-core boundary check passed`).

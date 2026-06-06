# GAT4THRMARBOA-005: Three Marks public view projection + UI metadata + visibility tests

**Status**: PENDING
**Priority**: HIGH
**Effort**: Medium
**Engine Changes**: Yes — new `games/three_marks/src/visibility.rs`, `src/ui.rs`; new `tests/visibility_tests.rs`
**Deps**: GAT4THRMARBOA-003

## Problem

The browser must render the Three Marks board without inferring any rule state. Rust must project a viewer-safe public view containing board cell ids/positions/occupancy/owner/mark metadata, active seat/status, the legal placement targets (with action ids, labels, accessibility labels, freshness token), terminal/win/draw outcome, and replay-projection scaffolding — plus UI presentation metadata. Per FOUNDATIONS §11 views must be viewer-safe; even a perfect-information game needs public-view-safety tests.

## Assumption Reassessment (2026-06-06)

1. `games/race_to_n/src/visibility.rs` is the mirror for public-view projection; `crates/wasm-api/src/lib.rs` returns the projected view (get-view path, `final_view` at line 326) and computes public-view hashes (`expected_public_view_hashes` in the exported trace). `race_to_n` has **no `ui.rs`** — `ui.rs` is a new file (no precedent; spec §5.1 + reassessment note); UI metadata placement is derived from how `race_to_n` exposes presentation data via its view projection and `docs/UI.md`.
2. Spec `specs/gate-4-three-marks-board-smoke.md` §8.2 (public view shape: game identity, board, active state, legal targets, terminal, diagnostics, replay projection), §12.3 (TS may transform but must not derive legality/occupancy/terminal/draw/winning-line/bot-choice), §5.1 (`visibility_tests.rs` required even for perfect info — a perfect-information game still needs public-view safety tests).
3. Cross-crate boundary under audit: the generic `engine-core` public/private-view contract and visibility-scope vocabulary (FOUNDATIONS §3) — `three_marks` produces a game-specific public view behind it, with private surfaces empty/not-applicable (perfect information).
4. FOUNDATIONS §11 (public/private views are viewer-safe; hidden information does not leak) and §2 (Rust owns view projection) motivate this ticket: the view carries enough for the board renderer (legal targets, occupancy, winning line, draw) so TypeScript never decides a rule, and exposes no information a viewer should not have.
5. No-leak firewall enforcement surface (§11): the public-view projection is the firewall — name it. Three Marks has no hidden state, so the visibility test asserts the *positive* contract (legal targets/occupancy/terminal are present and correct) and the *negative* contract (no field carries non-public or debug-internal data), and confirms private-view surfaces are explicitly empty/not-applicable per spec §24.
6. Extends the public-view contract with a game-specific board view + UI metadata. Consumers: `crates/wasm-api` get-view (GAT4THRMARBOA-009) and the `ThreeMarksBoard` renderer (011). The extension is additive game-specific JSON behind the generic view contract.

## Architecture Check

1. Projecting a complete, self-describing board view in Rust (cells + legal targets + outcome) is cleaner than a thin view the UI must re-interpret: it keeps legality authority in Rust and makes the renderer a pure presenter. Alternative (UI derives occupancy/winning line from raw state) violates §2/§12 and is rejected.
2. No backwards-compatibility aliasing/shims — new modules.
3. `engine-core` gains no board/cell/line nouns (the view's board vocabulary lives in `games/three_marks/src/visibility.rs`); no `game-stdlib` extraction.

## Verification Layers

1. View-completeness invariant -> schema/serialization validation (public view contains cell ids/occupancy/owner/mark metadata, active seat, legal targets with action ids + a11y labels + freshness, terminal/win-line/draw).
2. Viewer-safe / no-leak invariant -> no-leak visibility test (`tests/visibility_tests.rs`: no field carries non-public or debug-internal data; private surfaces explicitly empty/not-applicable).
3. Serialization-stability invariant -> golden/deterministic check (public-view serialization is stable and round-trips; deterministic field ordering — feeds the public-view hash exercised in 007).
4. TS-cannot-infer invariant -> FOUNDATIONS alignment check (§2/§12: the view supplies legality/occupancy/terminal so TypeScript never decides them).

## What to Change

### 1. `src/visibility.rs`

Public-view projection per spec §8.2: game identity (game_id, public name, variant id, rules version, match/session id where conventions expose it); board (stable cell ids, display positions/order, per-cell occupancy, owner seat, mark token metadata/presentation keys); active state (active seat, ply, status label); legal targets (action ids/path segments, labels, accessibility labels, freshness token); terminal state (non-terminal/win{seat, line cells}/draw); diagnostics where conventions expose them; replay-projection scaffolding (step board + step effects + outcome, consumed by 007). Private-view surfaces are explicit empty/not-applicable.

### 2. `src/ui.rs`

Game-local UI presentation metadata/keys (mark-token identity, board layout/order hints, status/label copy identifiers) that the renderer consumes — no legality, no rule logic. Mirror how `race_to_n` exposes presentation data; document that this is a new file.

### 3. `tests/visibility_tests.rs`

Public-view-safety tests: positive contract (legal targets / occupancy / owner / winning line / draw present and correct for representative states), negative contract (no non-public/debug field), private-surface not-applicable assertion, and a public-view serialization round-trip.

## Files to Touch

- `games/three_marks/src/visibility.rs` (new)
- `games/three_marks/src/ui.rs` (new)
- `games/three_marks/src/lib.rs` (modify)
- `games/three_marks/tests/visibility_tests.rs` (new)

## Out of Scope

- Replay/effect/action-tree/state hash suite and `serialization_tests.rs` (GAT4THRMARBOA-007).
- WASM view bridging (009) and the `ThreeMarksBoard` renderer (011).
- Bot decisions/explanations (006).

## Acceptance Criteria

### Tests That Must Pass

1. `cargo test -p three_marks --test visibility_tests` — view-completeness, no-leak, and round-trip assertions pass.
2. `cargo test -p three_marks` — full crate suite green.
3. `bash scripts/boundary-check.sh` — `engine-core` stays noun-free.

### Invariants

1. The public view supplies legal targets, occupancy/owner, terminal/win-line/draw, and freshness so TypeScript can render without deciding any rule.
2. No view field carries hidden/debug-internal data; private-view surfaces are explicitly empty/not-applicable.

## Test Plan

### New/Modified Tests

1. `games/three_marks/tests/visibility_tests.rs` — public-view completeness, viewer-safety (no-leak), and serialization round-trip.

### Commands

1. `cargo test -p three_marks --test visibility_tests`
2. `cargo test -p three_marks && bash scripts/boundary-check.sh`
3. Public-view *hash* stability across replay is exercised in 007; view-level safety/round-trip tests are the correct boundary for the projection diff.

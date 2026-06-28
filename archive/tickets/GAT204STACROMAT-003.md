# GAT204STACROMAT-003: Gate 20.4 evidence + closeout

**Status**: COMPLETED
**Priority**: LOW
**Effort**: Small
**Engine Changes**: None — docs/status-only (`games/starbridge_crossing/docs/UI.md`, `games/starbridge_crossing/docs/GAME-EVIDENCE.md`, `specs/README.md`, the Gate 20.4 spec Status)
**Deps**: GAT204STACROMAT-001, GAT204STACROMAT-002

## Problem

Once the Rust labels (GAT204STACROMAT-001) and the web consumption (GAT204STACROMAT-002) land, the Gate 20.4 documentation and tracker must record that in-match Starbridge seat naming is sourced from the Rust-projected per-seat display label, and the spec/index must flip to `Done`. This closeout reconciles `games/starbridge_crossing/docs/UI.md` (in-match seat naming now Rust-sourced; the superseded `Seat 1 … Seat 6` legend phrasing adjusted), records the fix receipt in `GAME-EVIDENCE.md`, and flips the `specs/README.md` Gate 20.4 row and the spec's own Status to `Done` after the gate-0/gate-1 acceptance run passes.

## Assumption Reassessment (2026-06-28)

1. `games/starbridge_crossing/docs/UI.md` and `games/starbridge_crossing/docs/GAME-EVIDENCE.md` exist and are the per-game UI/evidence docs; `UI.md` currently describes the seat legend in `Seat N`/home-point terms that 001/002 supersede. Confirmed.
2. `specs/README.md` row `11.4 | Gate 20.4 — Starbridge Crossing in-match seat display names` reads `Planned` (authored 2026-06-28 from a `/refine-game-ui` finding); the spec `specs/gate-20-4-starbridge-crossing-in-match-seat-display-names.md` Status is `Planned`. This ticket flips both to `Done`. Confirmed.
3. Cross-artifact boundary under audit: the doc/tracker surfaces only. This ticket introduces no production logic; its acceptance is the green gate run exercising the prior two tickets plus the doc/status reconciliation. It depends on 001 + 002 having landed so the documented behavior matches the code.

## Architecture Check

1. A single trailing docs+status ticket keeps the documentation reconciliation atomic and gated on the implementation being green, rather than editing docs to describe behavior before it exists. It exercises 001/002 end-to-end; it modifies none of their files.
2. No backwards-compatibility shim — this is documentation and status reconciliation only.
3. `engine-core` stays free of mechanic nouns — no code change; docs/status-only (§3).

## Verification Layers

1. Docs match landed behavior → grep-proof that `UI.md` records the Rust-projected in-match seat label as the source (and the superseded `Seat 1 … Seat 6` legend phrasing is removed/adjusted), and `GAME-EVIDENCE.md` carries the Gate 20.4 fix receipt.
2. Tracker/status reconciliation → grep-proof that the `specs/README.md` Gate 20.4 row and the spec Status both read `Done`.
3. Gate acceptance (distributed) → the full gate-0 + gate-1 command set passes against the post-implementation tree (this ticket adds no new test; it exercises the suite the prior tickets composed).

## What to Change

### 1. `games/starbridge_crossing/docs/UI.md`

Record that in-match seat naming (board heading, active-seat status, screen-reader summary, per-space accessibility labels, seat legend incl. the `to {…}` destination, and the shared turn-status bar) is sourced from the Rust-projected per-seat display label (`ui.seat_labels` / `seats[].label` / `seats[].target_label`). Remove/adjust the superseded `Seat 1 north to south` legend phrasing.

### 2. `games/starbridge_crossing/docs/GAME-EVIDENCE.md`

Add the Gate 20.4 fix receipt (in-match seat display names; Rust-owned label projection; the `formatPoint` removal; the additive `api_surface.tsv` snapshot diff).

### 3. `specs/README.md` + spec Status

Flip the Gate 20.4 row in `specs/README.md` and the Status field in `specs/gate-20-4-starbridge-crossing-in-match-seat-display-names.md` to `Done`, with a one-line completion note.

## Files to Touch

- `games/starbridge_crossing/docs/UI.md` (modify)
- `games/starbridge_crossing/docs/GAME-EVIDENCE.md` (modify)
- `specs/README.md` (modify)
- `specs/gate-20-4-starbridge-crossing-in-match-seat-display-names.md` (modify)

## Out of Scope

- Any code change — production logic and tests are owned by GAT204STACROMAT-001 (Rust) and GAT204STACROMAT-002 (web); this ticket is docs/status-only.
- Any movement/finish/terminal/visibility/bot change; any ring-label renaming.

## Acceptance Criteria

### Tests That Must Pass

1. CI gate 0: `cargo fmt --all --check`, `cargo clippy --workspace --all-targets -- -D warnings`, `cargo build --workspace`, `cargo test --workspace`.
2. CI gate 1 (Starbridge): `cargo run -p simulate -- --game starbridge_crossing --games 1000`, `cargo run -p replay-check -- --game starbridge_crossing --all`, `cargo run -p fixture-check -- --game starbridge_crossing`, `cargo run -p rule-coverage -- --game starbridge_crossing`, `node scripts/check-doc-links.mjs`, `node scripts/check-catalog-docs.mjs`, plus the web build/smokes.
3. `grep -n "Done" specs/README.md | grep "Gate 20.4"` and the spec Status both confirm `Done`.

### Invariants

1. The documented in-match seat-naming source matches the landed Rust projection (no stale `Seat N`/`formatPoint` narrative).
2. The Gate 20.4 tracker row and spec Status are `Done` only after the gate-0/gate-1 acceptance run passes.

## Test Plan

### New/Modified Tests

1. `None — docs-reconciliation + closeout capstone; it reconciles `UI.md` / `GAME-EVIDENCE.md` / `specs/README.md` / the spec Status and exercises the prior tickets' acceptance suite, adding no test file.`

### Commands

1. `node scripts/check-doc-links.mjs` (doc-link integrity after the UI.md/evidence edits)
2. `cargo run -p replay-check -- --game starbridge_crossing --all` (confirms 001/002 left determinism intact)
3. The gate-0 + gate-1 command sets above are the full acceptance boundary; this ticket runs them rather than adding a new test.

## Outcome

Completed: 2026-06-28

Closed the Gate 20.4 evidence and tracker surfaces:

- Updated `games/starbridge_crossing/docs/UI.md` to state that in-match board, accessibility, legend destination, replay, and shared turn-bar seat labels come from Rust/WASM-projected `ui.seat_labels`, `seats[].label`, and `seats[].target_label`.
- Added a Gate 20.4 receipt to `games/starbridge_crossing/docs/GAME-EVIDENCE.md`, including Rust/WASM projection, discontinuous seat-count regression, additive API snapshot, web consumption, shared turn-bar proof, and browser regression evidence.
- Flipped the Gate 20.4 spec Status and `specs/README.md` tracker row to `Done`.

Deviations from plan: none. This ticket made docs/status changes only.

Verification:

- `cargo fmt --all --check` passed.
- `cargo clippy --workspace --all-targets -- -D warnings` passed.
- `cargo build --workspace` passed.
- `cargo test --workspace` passed.
- `cargo run -p simulate -- --game starbridge_crossing --games 1000` passed: 1000 games, 2 seats, 2,000,000 total actions, zero capped matches.
- `cargo run -p replay-check -- --game starbridge_crossing --all` passed.
- `cargo run -p fixture-check -- --game starbridge_crossing` passed.
- `cargo run -p rule-coverage -- --game starbridge_crossing` passed.
- `node scripts/check-doc-links.mjs` passed (`Checked 31 markdown files`).
- `node scripts/check-catalog-docs.mjs` passed.
- `npm --prefix apps/web run smoke:wasm` passed.
- `npm --prefix apps/web run build` passed.
- `npm --prefix apps/web run smoke:ui` passed.
- `npm --prefix apps/web run smoke:effects` passed.
- `npm --prefix apps/web run smoke:e2e` passed.
- `rg -n "formatPoint" apps/web/src/components/StarbridgeCrossingBoard.tsx` returned no matches.
- `rg -n '\\| Status \\| `Done` \\|' specs/gate-20-4-starbridge-crossing-in-match-seat-display-names.md` confirmed the spec Status is `Done`.

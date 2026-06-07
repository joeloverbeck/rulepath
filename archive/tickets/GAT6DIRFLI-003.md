# GAT6DIRFLI-003: Optional `game-stdlib` spatial helper extraction (conditional)

**Status**: NOT IMPLEMENTED
**Priority**: MEDIUM
**Effort**: Large
**Engine Changes**: Yes — `game-stdlib` (new narrow typed spatial helpers + tests); possible back-port edits to `games/column_four/src/` and `games/three_marks/src/`. `engine-core` untouched.
**Deps**: 002

## Problem

**This ticket is CONDITIONAL.** It is worked only if GAT6DIRFLI-002's primitive-pressure ledger decides **promote**. The spec's stated default (§10.2, atlas §5) is to keep coordinate/ray logic local until repeated public pressure proves extraction; if the ledger decides defer/reject or ADR, this ticket is closed as **not-applicable** (no diff) and `directional_flip` rules-core (GAT6DIRFLI-005) implements the coordinate/ray logic locally. When promotion is decided, this ticket extracts only narrow, behavior-free spatial utilities into `game-stdlib`, with tests/docs/examples/anti-examples/benchmarks and a back-port, per FOUNDATIONS §4 and `docs/MECHANIC-ATLAS.md` §5–§7.

## Assumption Reassessment (2026-06-07)

1. `crates/game-stdlib/src/lib.rs` is a placeholder today; promotion adds new modules (illustratively `grid.rs`, `direction.rs`, `ray.rs` per spec §8.4) plus `tests/`. The crate is already a workspace member (`Cargo.toml` line 5), so no workspace wiring is needed beyond module declarations.
2. The decision and its scope come from `games/directional_flip/docs/PRIMITIVE-PRESSURE-LEDGER.md` (GAT6DIRFLI-002) and `docs/MECHANIC-ATLAS.md` §10; this ticket implements exactly the helper shape that ledger authorized — no broader "spatial library."
3. Cross-crate boundary under audit: `game-stdlib` ↔ `games/{column_four,three_marks,directional_flip}`. The back-port edits existing games' coordinate logic to consume the helper; the existing golden traces of `column_four`/`three_marks` must be preserved (or intentionally updated with trace notes per atlas §7 step 9). Name the exact call sites before editing.
4. FOUNDATIONS §4 motivates the ticket: restate before coding — `game-stdlib` is not a second kernel and not a universal tabletop library; a promoted helper MUST have small APIs, explicit limits, and no hidden game policy (atlas §7).
5. This ticket touches the §4 third-use hard gate directly. Confirm the helper carries **no** behavior policy (spec §10.3 forbidden list: flip/capture/win/occupancy/legal-action/forced-pass/bot/UI/effect-name/static-data behavior), introduces no mechanic noun into `engine-core` (§3), and that the back-port preserves deterministic replay/hash for the back-ported games (§11/§13 — golden traces re-pass or are migrated with notes). No hidden information is involved (pure coordinate math).

## Architecture Check

1. A narrow typed helper (bounded rectangular coordinates, row/column indexing, eight-direction deltas, bounded ray stepping, stable parse/format — spec §10.3) earns its place only because three games now share the shape; this is the §4 "earned, not speculative" path, not a generalization from one game.
2. No backwards-compatibility shims: the back-port replaces local logic outright; it does not leave alias paths (`tickets/README.md` core contract 1).
3. `engine-core` stays mechanic-noun-free (the helper lives in `game-stdlib`, and even there carries no `flip`/`capture` vocabulary — only generic `coordinate`/`direction`/`ray`); `game-stdlib` growth is the recorded earned outcome (§4).

## Verification Layers

1. Behavior-free helper invariant -> FOUNDATIONS alignment check (§4/§5) + manual review against spec §10.3 forbidden list.
2. Helper correctness -> schema/serialization N/A; instead property tests (bounds, direction order, ray termination, row-major iteration, parse/format round-trip — spec §10.4) in `crates/game-stdlib/tests/`.
3. Back-port determinism -> golden trace / deterministic replay-hash check: `column_four` (and `three_marks` if back-ported) golden traces and replay hashes re-pass unchanged, or are migrated with explicit trace notes (atlas §7 step 9).
4. Helper performance honesty -> benchmark check: helper microbenchmarks (or before/after) confirm the helper hides no slow generic behavior (atlas §8).

## What to Change

### 1. Promote narrow spatial helpers into `game-stdlib`

Add only the utilities the ledger authorized (e.g. `crates/game-stdlib/src/grid.rs`, `direction.rs`, `ray.rs`; names follow the ledger/repo style), re-exported from `crates/game-stdlib/src/lib.rs`. Behavior-free only.

### 2. Helper tests, examples, anti-examples

Add `crates/game-stdlib/tests/` unit + property tests, plus doc examples and anti-examples defining the helper boundary (atlas §7 step 10).

### 3. Back-port affected games

Replace local coordinate/ray logic in `games/column_four/src/` (and `games/three_marks/src/` only where it fits without contortion — spec §10.4) with the helper. Preserve or intentionally migrate golden traces.

### 4. Docs

Document helper scope and non-goals (in the crate and/or the ledger); the GAT6DIRFLI-002 atlas update already records the promotion status.

## Files to Touch

- `crates/game-stdlib/src/lib.rs` (modify)
- `crates/game-stdlib/src/grid.rs` (new — illustrative; per ledger)
- `crates/game-stdlib/src/direction.rs` (new — illustrative; per ledger)
- `crates/game-stdlib/src/ray.rs` (new — illustrative; per ledger)
- `crates/game-stdlib/tests/rect_grid.rs` (new — illustrative; per ledger)
- `games/column_four/src/` (modify — back-port call sites, as surfaced)
- `games/three_marks/src/` (modify — back-port call sites only where it fits, as surfaced)

## Out of Scope

- Any flip/capture/legality/forced-pass/bot/UI/effect logic (stays in `games/*`; FOUNDATIONS §3/§4, spec §10.3).
- `directional_flip`'s own rules-core (GAT6DIRFLI-005) — it consumes the helper if promoted, but is a separate ticket.
- Proceeding at all if GAT6DIRFLI-002 decided defer/reject/ADR (this ticket is then not-applicable).

## Acceptance Criteria

### Tests That Must Pass

1. `cargo test -p game-stdlib` — helper unit + property tests pass.
2. `cargo test -p column_four` (and `-p three_marks` if back-ported) — back-ported games still pass, including replay/golden-trace tests.
3. `cargo run -p replay-check -- --game column_four --all` — back-ported game replay hashes hold (or migrated with notes).
4. `bash scripts/boundary-check.sh` — `engine-core` stays noun-free; helper introduced no kernel vocabulary.

### Invariants

1. The helper contains no game behavior policy and no mechanic noun in `engine-core` (FOUNDATIONS §3/§4, spec §10.3).
2. Back-ported games' deterministic replay/hash is preserved or explicitly migrated (FOUNDATIONS §11/§13).

## Test Plan

### New/Modified Tests

1. `crates/game-stdlib/tests/rect_grid.rs` — unit + property tests for bounds, direction order, ray termination, row-major iteration, parse/format (per ledger).
2. `games/column_four/tests/replay.rs` — re-pass unchanged after back-port (or migrate with trace notes).

### Commands

1. `cargo test -p game-stdlib`
2. `cargo test --workspace && cargo run -p replay-check -- --game column_four --all && bash scripts/boundary-check.sh`
3. Workspace-wide test + boundary check is the correct boundary because back-port touches multiple existing game crates whose determinism must be re-proven.

## Outcome

Completed: 2026-06-07

What changed:

- No code or helper documentation was implemented for this conditional extraction ticket.
- `games/directional_flip/docs/PRIMITIVE-PRESSURE-LEDGER.md` from GAT6DIRFLI-002 records `Decision: defer-reject`, so `game-stdlib` helper promotion and back-port work are not applicable for Gate 6.

Deviations from original plan:

- This ticket intentionally closed as not applicable because its precondition was not met.
- `crates/game-stdlib/src/lib.rs`, `games/column_four/src/`, and `games/three_marks/src/` were not changed.

Verification results:

- Manual dependency check: GAT6DIRFLI-002 is archived and committed with the `defer-reject` primitive-pressure decision.
- Manual worktree check: no `game-stdlib`, `column_four`, or `three_marks` source changes were made for this ticket.

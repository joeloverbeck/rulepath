# GAT17VOWTIDOHHEL-002: Promote `game-stdlib::trick_taking` helper + atlas decision

**Status**: PENDING
**Priority**: HIGH
**Effort**: Medium
**Engine Changes**: Yes — new `crates/game-stdlib/src/trick_taking.rs` + `crates/game-stdlib/benches/trick_taking.rs`; modifies `crates/game-stdlib/src/lib.rs`, `docs/MECHANIC-ATLAS.md`
**Deps**: None

## Problem

Vow Tide is the third close trick-taking use; FOUNDATIONS §4 makes the third use a hard gate that must resolve before the game proceeds. The reassessed spec selected atlas **option 2 — promote a narrow, pure, typed selection/comparison helper** to `game-stdlib`. This ticket authors that helper (the single hard-gate outcome) and records the atlas decision, so trick code in Vow Tide and the back-ported games can adopt it.

## Assumption Reassessment (2026-06-21)

1. `crates/game-stdlib/src/lib.rs` currently exports only `pub mod board_space;` (confirmed at reassessment) — no `trick_taking` module exists; no naming collision. `crates/game-stdlib/Cargo.toml` has a `[lib]` but no `[[bench]]`/criterion dev-dep yet (the sibling game benches use criterion).
2. `docs/MECHANIC-ATLAS.md` §10 carries four `repeated-shape candidate` rows (follow-suit legality, trick resolution/comparator, trick-winner-leads, deal rotation) each naming "Gate 17 Oh Hell is the third-use hard-gate trigger"; §10A reads `Current debt: _None_`.
3. Cross-crate boundary: the helper's two-function API is the shared contract consumed by `plain_tricks` (003), `briar_circuit` (004), and `vow_tide` trick play (008); its signatures and stable-index semantics are under audit.
4. FOUNDATIONS §4 (earned promotion) is the principle under audit: the helper must be pure, game-state-agnostic, and free of policy; §3 — it must move no noun into `engine-core`.
5. §4 third-use hard gate enforcement surface: the helper must own only led-suit selection and winner-index comparison; winner-leads mutation, dealing, dealer rotation, scoring, and terminal policy stay caller-side (documented anti-examples). No hidden-info or determinism path is introduced — it inspects only caller-projected copyable suit/rank keys and returns stable indices.

## Architecture Check

1. A narrow pure helper over caller-projected keys is cleaner than a third local fork and avoids semantic drift across three games' edge tests; it carries no card/seat/phase type, so it cannot leak policy.
2. No shims — new module, additive `lib.rs` export.
3. `engine-core` untouched; the `game-stdlib` addition is the atlas-earned third-use promotion (§4), with winner-leads/deal as recorded non-goals.

## Verification Layers

1. `follow_suit_indices` returns led-suit indices in input order, else all, empty→empty → `game-stdlib` unit + property tests.
2. `winning_play_index` = highest trump else highest led, stable first-occurrence ties, off-suit non-trump ignored, empty→`None` → unit + property tests.
3. No `engine-core` noun growth → `bash scripts/boundary-check.sh`.
4. Helper is allocation-bounded/deterministic → criterion microbench `crates/game-stdlib/benches/trick_taking.rs`.
5. §4 atlas decision recorded → grep `docs/MECHANIC-ATLAS.md` for the promoted-helper rows + `§10A` unchanged-`None`.

## What to Change

### 1. `trick_taking.rs` module

Implement the spec §4.2 API: `follow_suit_indices<T,S>(held, led_suit, suit_of) -> Vec<usize>` and `winning_play_index<T,S,R>(plays, led_suit, trump, suit_of, rank_of) -> Option<usize>`. Pure, side-effect-free, no shared card/seat/trump-source type, no policy flags. Add unit tests, property tests, examples, and anti-example doc comments (no "must lead 2♣", no winner-leads, no scoring).

### 2. `lib.rs` export + bench

Add `pub mod trick_taking;`. Add `crates/game-stdlib/benches/trick_taking.rs` (criterion) and the `[[bench]]`/dev-dep wiring.

### 3. Atlas decision

Update `docs/MECHANIC-ATLAS.md` §10 follow-suit and comparator rows to `promoted` naming the three games; record winner-leads and deal-rotation as explicit anti-examples/non-goals; arm §9A for Gate 18. Leave §10A `Current debt: _None_` (back-ports land in-gate; the final no-debt receipt is 022).

## Files to Touch

- `crates/game-stdlib/src/trick_taking.rs` (new)
- `crates/game-stdlib/benches/trick_taking.rs` (new)
- `crates/game-stdlib/src/lib.rs` (modify)
- `crates/game-stdlib/Cargo.toml` (modify)
- `docs/MECHANIC-ATLAS.md` (modify)

## Out of Scope

- Any back-port of `plain_tricks`/`briar_circuit` (003/004) or Vow Tide trick code (008).
- Any policy flag, card/seat/trump-source type, winner-leads/deal logic, or TypeScript caller.

## Acceptance Criteria

### Tests That Must Pass

1. `cargo test -p game-stdlib` — helper unit + property tests pass.
2. `bash scripts/boundary-check.sh` — no kernel noun growth.
3. `cargo bench -p game-stdlib` — `trick_taking` microbench runs.

### Invariants

1. Helper functions are pure and game-state-agnostic; no allocation/genericity shortcut reorders results.
2. `engine-core` gains no mechanic noun; `game-stdlib` change is atlas-recorded.

## Test Plan

### New/Modified Tests

1. `crates/game-stdlib/src/trick_taking.rs` (inline unit tests) — empty/single/multi inputs, led-present/absent, trump/no-trump, off-suit exclusion, stable tie.
2. `crates/game-stdlib/tests/` or inline property tests — returned indices valid/ordered, follow set complete+exclusive, comparator maximal in winning class, input unmutated.

### Commands

1. `cargo test -p game-stdlib`
2. `cargo clippy -p game-stdlib --all-targets -- -D warnings && bash scripts/boundary-check.sh`
3. `cargo bench -p game-stdlib` (before/after baseline consumed by the back-port tickets and 020).

# GAT19MELLEDFIV-003: Crate skeleton, workspace wiring, ids, and typed variant data

**Status**: COMPLETED
**Priority**: HIGH
**Effort**: Medium
**Engine Changes**: Yes — new crate `games/meldfall_ledger` (workspace member; module stubs); `engine-core`/`game-stdlib` unchanged
**Deps**: GAT19MELLEDFIV-002

## Problem

Gate 19 needs the `games/meldfall_ledger` crate scaffold before any behavior lands: the workspace member entry, `Cargo.toml`, `lib.rs` module wiring, `ids.rs` (game/variant/rules-version + fixture-profile ids), the typed `data/manifest.toml` and `data/variants.toml` (`classic_500_single_deck_v1` constants only), and empty/stub modules so later pipeline tickets fill them. This is admitted only after the forward-v1 audit (GAT19MELLEDFIV-002).

## Assumption Reassessment (2026-06-25)

1. `games/blackglass_pact/` layout (confirmed during reassessment) is the model: `Cargo.toml`, `benches/`, `data/{manifest.toml,variants.toml,fixtures/}`, `docs/`, `src/{lib,ids,cards,setup,variants,state,actions,rules,scoring,effects,visibility,replay_support,bots,ui}.rs`, `tests/`. Module set matches spec §4.1.
2. Root `Cargo.toml` `[workspace] members` lists `games/blackglass_pact`, `games/vow_tide`, `games/river_ledger`, … but **not** `games/meldfall_ledger` (verified during reassessment); this ticket adds it.
3. Cross-artifact: the Cargo workspace graph is the shared boundary — the new crate depends on `engine-core` (+ `game-stdlib` for `seat` only), never the reverse; no dependency-direction inversion.
4. FOUNDATIONS §3 `engine-core` is a contract kernel: all rummy nouns (`Suit`, `Rank`, `CardId`, meld/tableau/pile/stock/discard types) live in this crate's modules, never in `engine-core`. `boundary-check.sh` enforces this.
5. FOUNDATIONS §5 static data is not behavior: `data/variants.toml` and `data/manifest.toml` carry only typed identifiers/constants (target score 500, deal counts, display text, fixture-profile ids) — no selectors, formulas, meld/lay-off/scoring rules. This is substrate the later rules tickets consume; it introduces no behavior path.

## Architecture Check

1. A thin skeleton with compile-clean stubs lets each pipeline ticket land a reviewable diff against a real crate, rather than one mega-diff; matches the established new-game crate shape.
2. No backwards-compatibility shims — greenfield crate; all files `(new)` except the workspace member edit.
3. `engine-core` stays noun-free (`boundary-check.sh`); `game-stdlib` gains nothing (only `seat` is reused, not promoted).

## Verification Layers

1. Workspace builds with the new member -> `cargo build -p meldfall_ledger`.
2. `engine-core` stays free of mechanic nouns -> `bash scripts/boundary-check.sh`.
3. Static data is typed-only (no behavior fields) -> manual review of `data/*.toml` against FOUNDATIONS §5 + schema parse in `cargo test -p meldfall_ledger`.

## What to Change

### 1. Workspace + crate manifest

Add `games/meldfall_ledger` to the root `Cargo.toml` `[workspace] members`. Create `games/meldfall_ledger/Cargo.toml` depending on `engine-core` and `game-stdlib` (seat helpers), mirroring `games/blackglass_pact/Cargo.toml`.

### 2. `lib.rs`, `ids.rs`, module stubs

`lib.rs` wires the module tree (`cards`, `setup`, `variants`, `state`, `actions`, `rules`, `scoring`, `effects`, `visibility`, `replay_support`, `bots`, `ui`) as compile-clean stubs. `ids.rs` defines game id `meldfall_ledger`, variant id `classic_500_single_deck_v1`, rules version `meldfall-ledger-rules-v1`, data version `meldfall-ledger-data-v1`, and fixture-profile ids.

### 3. Typed data files

`data/manifest.toml` and `data/variants.toml` with typed `classic_500_single_deck_v1` constants (seat range 2–6, default 4, deal counts, target 500, display metadata) — no behavior fields.

## Files to Touch

- `Cargo.toml` (modify — add workspace member)
- `Cargo.lock` (modify — workspace package lock entry)
- `games/meldfall_ledger/Cargo.toml` (new)
- `games/meldfall_ledger/src/lib.rs` (new)
- `games/meldfall_ledger/src/ids.rs` (new)
- `games/meldfall_ledger/src/{cards,setup,variants,state,actions,rules,scoring,effects,visibility,replay_support,bots,ui}.rs` (new — stubs)
- `games/meldfall_ledger/data/manifest.toml` (new)
- `games/meldfall_ledger/data/variants.toml` (new)

## Out of Scope

- Any rule/setup/scoring behavior (later pipeline tickets fill the stubs).
- `tests/`, `benches/`, `docs/` beyond what GAT19MELLEDFIV-001/002 already authored.
- WASM/web/tool registration (GAT19MELLEDFIV-014/016/018/019/020/021).

## Acceptance Criteria

### Tests That Must Pass

1. `cargo build -p meldfall_ledger` compiles the skeleton.
2. `bash scripts/boundary-check.sh` passes (no mechanic nouns leaked into `engine-core`).
3. `cargo build --workspace` succeeds with the new member.

### Invariants

1. All rummy nouns are crate-local; `engine-core` is unchanged.
2. `data/*.toml` contain only typed content/parameters/metadata (FOUNDATIONS §5), no behavior selectors or formulas.

## Test Plan

### New/Modified Tests

1. `None — skeleton ticket; behavior tests land with the pipeline tickets. Compile + boundary check are the verification boundary.`

### Commands

1. `cargo build -p meldfall_ledger`
2. `cargo build --workspace && bash scripts/boundary-check.sh`
3. A narrower per-crate build is the correct boundary; full `cargo test --workspace` belongs to later behavior tickets.

## Outcome

Completed: 2026-06-26

What changed:

- Added `games/meldfall_ledger` to the Cargo workspace and lockfile.
- Added a compile-clean `meldfall_ledger` crate manifest, module tree, `ids.rs` constants/helpers, local stub modules, and static typed data files for `manifest.toml` and `variants.toml`.
- Added a small local static-data parser and tests that accept the typed constants while rejecting unknown and behavior-looking fields.

Deviations from plan:

- `Cargo.lock` was updated by the workspace build and is included as ticket-owned workspace metadata.
- Added unit tests inside the skeleton for seat helpers and static-data parsing; no rule/setup/scoring behavior was implemented.

Verification:

- `cargo fmt --all --check` passed.
- `cargo build -p meldfall_ledger` passed.
- `cargo build --workspace` passed.
- `bash scripts/boundary-check.sh` passed (`engine-core boundary check passed`; `game-test-support dev-only boundary check passed`).
- `cargo test -p meldfall_ledger` passed (3 unit tests, 0 doctests).
- `rg -n "^(when|if|then|else|selector|condition|trigger|script|loop|foreach|priority_expression|ai_condition|effect_script|rule|requires|valid_if|on_play|on_reveal|formula|score_formula|tie_break_formula|meld_formula|layoff_formula|discard_pickup_formula|deal_formula|rotation_formula|bot_policy)\\s*=" games/meldfall_ledger/data` found no behavior-looking static-data keys.

# RIVLEDSHOWUX-007: `RiverLedgerShowdownPresentationV2` Rust payload

**Status**: PENDING
**Priority**: MEDIUM
**Effort**: Large
**Engine Changes**: Yes — `games/river_ledger/src/showdown.rs`, `games/river_ledger/src/state.rs`, `games/river_ledger/src/visibility.rs`, `games/river_ledger/tests/{rules,serialization}.rs`, `games/river_ledger/tests/golden_traces/*.trace.json`
**Deps**: RIVLEDSHOWUX-001

## Problem

The shipped V1 showdown explanation answers "who won and why" but is not ranked-standings-shaped: it names no closest challenger contrastively, shows the board once-per-seat rather than once with usage marks, and carries no hole/board card-usage marks. This ticket adds an additive `RiverLedgerShowdownPresentationV2` Rust payload (result banner, decisive contrast-seat, ranked standings, folded rows, card-usage marks, accessibility labels) alongside the V1 fields.

## Assumption Reassessment (2026-06-16)

1. Verified: V1 explanation fields are built in `showdown.rs` (headline/decisive/per-seat hand-name/best-five); `evaluator.rs` is the sole source of category/tie-break/best-five facts; seat labels come from the RIVLEDSHOWUX-001 helper (hence `Deps`).
2. Verified against spec §6 D6 + §8 WB7; the V2 field shape mirrors the report's `RiverLedgerShowdownPresentationV2` schema; `RULES.md` `RL-UI-SHOWDOWN-001`, `RL-EVAL-TIEBREAK-001`.
3. Shared boundary under audit: the terminal projection — V2 is **additive alongside** V1 (V1 fields and raw `category_key`/`tie_break_vector`/rule IDs are retained for the details tier), so existing serialization/replay consumers stay compatible.
4. FOUNDATIONS §2: Rust authors banner/standings/order/card-usage; `standings` arrives already ranked so TypeScript renders that order and derives nothing.
5. No-leak / determinism: standings and card-usage marks exist only for showdown-eligible revealed seats; folded seats appear only as `folded_rows` with a redaction label and no hand strength (`RL-VIS-SHOWDOWN-001` / `RL-VIS-FOLDOUT-001`); the new serialized fields are deterministic and covered by fixture/golden-trace updates (§11).
6. Schema extension: the terminal view gains additive `result_banner` / `decisive_reason` / `board_cards[]` / `standings[]` / `folded_rows[]`; consumers are the bridge (RIVLEDSHOWUX-008) and renderer (RIVLEDSHOWUX-009); additive-only.

## Architecture Check

1. An additive V2 payload (vs reshaping `ShowdownReveal`) keeps existing serialization/replay consumers compatible and lets the V1 path stay until the renderer cuts over; `evaluator.rs` stays the sole fact source.
2. No shims; V2 is new fields, not an alias of V1.
3. `engine-core` untouched (§3); explanation/card-usage types are `games/river_ledger`-local; no `game-stdlib` change (§4).

## Verification Layers

1. V2 banner/decisive/standings/card-usage authored from evaluator facts -> `games/river_ledger/tests/rules.rs` (worked example, split, foldout).
2. Reveal-scoped: folded seats carry only `folded_rows` redaction, no standings/usage -> `games/river_ledger/tests/visibility.rs`.
3. Serialized V2 deterministic -> `cargo run -p replay-check -- --game river_ledger --all` + updated golden traces.

## What to Change

### 1. `games/river_ledger/src/showdown.rs` / `state.rs`

Build `RiverLedgerShowdownPresentationV2`: `result_banner {headline,subheadline,accessibility_label}`; `decisive_reason {short_text,contrast_seat,contrast_seat_label,rule_refs}`; `board_cards[] {slot,card,public_label,used_by_selected}`; ranked `standings[]` (result/allocation labels, hand name, short comparison note, rank-ladder label, hole/board card-usage marks, best-five + a11y label, detail rows, default-expanded); `folded_rows[]`.

### 2. `games/river_ledger/src/visibility.rs` (build-side scoping)

Populate standings/card-usage only for showdown-eligible revealed seats; folded seats → `folded_rows` redaction only. (Cross-viewer projection lands in RIVLEDSHOWUX-008.)

### 3. Golden traces / serialization

Update affected golden traces where the terminal view JSON now carries V2 fields.

## Files to Touch

- `games/river_ledger/src/showdown.rs` (modify)
- `games/river_ledger/src/state.rs` (modify)
- `games/river_ledger/src/visibility.rs` (modify)
- `games/river_ledger/tests/rules.rs` (modify)
- `games/river_ledger/tests/serialization.rs` (modify)
- `games/river_ledger/tests/golden_traces/*.trace.json` (modify, as surfaced by view-JSON change)

## Out of Scope

- Cross-viewer reveal-scoped projection + bridge + TS types (RIVLEDSHOWUX-008).
- The V2 renderer + card-usage visualization (RIVLEDSHOWUX-009).

## Acceptance Criteria

### Tests That Must Pass

1. `cargo test -p river_ledger` — V2 payload authored for worked example / split / foldout; folded seats carry only `folded_rows`.
2. `cargo run -p replay-check -- --game river_ledger --all` + `cargo run -p fixture-check -- --game river_ledger` — deterministic; golden traces updated.
3. `cargo run -p rule-coverage -- --game river_ledger` — `RL-UI-SHOWDOWN-001` coverage intact.

### Invariants

1. `standings` is Rust-ranked; no TS reorders it (§2).
2. Standings/card-usage exist only for revealed seats; folded seats reveal no strength (§11 no-leak).

## Test Plan

### New/Modified Tests

1. `games/river_ledger/tests/rules.rs` — V2 banner/decisive/standings/card-usage on the worked example, split, foldout, high-card.
2. `games/river_ledger/tests/serialization.rs` — V2 fields serialize deterministically.

### Commands

1. `cargo test -p river_ledger`
2. `cargo run -p replay-check -- --game river_ledger --all`
3. `cargo run -p fixture-check -- --game river_ledger`

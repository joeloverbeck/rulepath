# RIVLEDOUT-001: River Ledger Rust outcome-rationale view + wasm serialization

**Status**: COMPLETED
**Priority**: HIGH
**Effort**: Medium
**Engine Changes**: Yes — `games/river_ledger` (`src/visibility.rs`, projection from `src/state.rs`/`src/showdown.rs`), `crates/wasm-api` (`river_view_json`), and regenerated `games/river_ledger/tests/golden_traces/*.trace.json` public-view hashes
**Deps**: None

## Problem

`river_ledger` is the only one of the 15 catalog games (`crates/wasm-api/src/lib.rs:168`
`GAME_RIVER_LEDGER`) that never received the shared outcome-rationale contract that
`archive/specs/victory-explanation-shared-surface.md` rolled out via the
`VICEXPSHASUR-*` / `POKLITOUTRAT-001` tickets. CI gate `Gate 1 game smoke` fails at
`node scripts/check-outcome-explanations.mjs` because river_ledger's terminal
explanation data is buried inside `TerminalView::Showdown.explanations`
(`games/river_ledger/src/visibility.rs:75`) and is never lifted into a Rust-owned
`OutcomeRationaleView` projected onto the public view. This ticket adds the
Rust-owned view and its deterministic serialization so the presentation tier
(RIVLEDOUT-002) can mirror real Rust data rather than invent it (FOUNDATIONS §2/§11
Rust behavior authority).

## Assumption Reassessment (2026-06-15)

1. `games/river_ledger/src/visibility.rs` defines `PublicView` (line 11) built by
   `project_view` (line 79) and a `TerminalView` enum (line 65) with
   `LastLiveHand { winner, pot_total }` and `Showdown { winners, pot_total,
   allocations, explanations }`. There is **no** `OutcomeRationaleView` and **no**
   `terminal_rationale`/`rationale` field on `PublicView`. Verified by read.
2. The proven pattern is `games/poker_lite/src/visibility.rs`: `OutcomeRationaleView`
   (line 89) with `result_kind`, `decisive_cause`, `template_key`,
   `decisive_rule_ids`, `per_seat`, projected by `outcome_rationale(...)` (line ~305)
   and carried on the public/terminal view. `archive/tickets/POKLITOUTRAT-001.md`
   is the closest precedent (poker-like, hidden-info). Mirror it; do not redesign
   the shared shape.
3. Cross-artifact boundary under audit: the Rust→JSON view serialization in
   `crates/wasm-api/src/lib.rs` `river_view_json(&river_project_view(...))`
   (call sites: lines 1135, 1799, 2391, 3253). The new field is serialized here;
   the TS consumer side is RIVLEDOUT-002 and is out of scope for this ticket.
4. FOUNDATIONS §11 invariant restated before implementation: public/private views
   are viewer-safe and hidden information must not leak; replay/hashes/serialization
   order remain deterministic **or are explicitly migrated**. Adding a field to the
   public view changes the river_ledger public-view hash, so golden-trace
   `expected_public_view_hashes` MUST be regenerated as an explicit migration (the
   `VICEXPSHASUR-004/007` fixtures did the same — see wasm-api migration notes at
   lines 5804/6030).
5. No-leak enforcement surface: the rationale's per-seat strength/hole-derived
   facts are lawful only for outcomes Rust has already revealed (showdown). For a
   `LastLiveHand` (fold-out) win, the rationale MUST NOT carry any unrevealed hole
   card, rank, or strength for any seat — mirror poker_lite's `YieldWin`
   (`strength: None`, no private reveal). The observer view and any non-winning,
   non-revealed seat must never receive opponent hole-derived strength.
6. Schema extension classification: this extends the public-view schema. It is
   **additive** at the Rust struct level (new field). Downstream consumers are
   (a) `river_view_json` (this ticket) and (b) the TS mirror + golden-trace hashes
   (hashes migrated here; TS mirror in RIVLEDOUT-002). The golden-trace hash change
   is a migration, not a break.
7. Adjacent contradiction classification: river_ledger already exposes showdown
   explanation strings (`explain_showdown`, `games/river_ledger/src/showdown.rs:42`).
   Reuse that data as the rationale source rather than recomputing strength —
   recomputation would duplicate the evaluator and is a separate concern. Folding
   the existing strings into the structured rationale is a required consequence of
   this ticket.

## Architecture Check

1. Lifting terminal explanations into a Rust-owned `OutcomeRationaleView` keeps the
   single behavior authority in Rust and lets every presentation surface read one
   canonical, viewer-filtered payload — cleaner than the current split where the
   UI would have to parse free-text `explanations` strings (which TS must not
   interpret).
2. No backwards-compatibility shim: the new field is added directly; the existing
   `TerminalView::Showdown.explanations` free-text is either replaced by or kept as
   the human-readable source feeding the structured rationale (implementer's call,
   documented), with no alias path retained for a removed shape.
3. `engine-core` is untouched and stays noun-free; all new nouns
   (`OutcomeRationaleView`, etc.) live in `games/river_ledger`. No `game-stdlib`
   change is claimed, so the mechanic-atlas earned-helper gate does not apply.

## Verification Layers

1. Rationale projection correctness -> Rust unit tests in
   `games/river_ledger/src/visibility.rs` asserting `result_kind`/`decisive_cause`/
   `template_key`/`decisive_rule_ids` per terminal outcome (last-live, showdown win,
   showdown split).
2. No-leak (fold-out + observer) -> no-leak visibility test: observer and
   non-revealed seats never receive opponent hole/strength; `LastLiveHand` rationale
   carries no private reveal.
3. Deterministic serialization + migration -> golden trace / replay-hash check:
   regenerate `expected_public_view_hashes` in
   `games/river_ledger/tests/golden_traces/*.trace.json` and confirm
   `cargo run -p replay-check -- --game river_ledger --all` passes.
4. JSON serialization conformance -> schema/serialization validation: `river_view_json`
   emits a stable `terminal_rationale` object (field order, key names) matching the
   shared payload shape consumed in RIVLEDOUT-002.

## What to Change

### 1. River Ledger Rust view (`games/river_ledger/src/visibility.rs`)

Add `OutcomeRationaleView` (and a per-seat breakdown view) mirroring
`games/poker_lite/src/visibility.rs:89`. Add a `terminal_rationale: Option<OutcomeRationaleView>`
field to `PublicView` (name the serialized key `terminal_rationale` to match the
shared TS field). Add an `outcome_rationale(...)` projection from `TerminalOutcome`
(`games/river_ledger/src/state.rs:116`, sourced from `explain_showdown`
`games/river_ledger/src/showdown.rs:42`). Emit a `template_key` per outcome from a
fixed set (proposed; finalize against actual outcomes and keep in sync with
RIVLEDOUT-002 and the UI.md/templates keys):

- `river_ledger.showdown_best_hand_win`
- `river_ledger.showdown_split_pot`
- `river_ledger.last_live_fold_win`

Each rationale carries `decisive_rule_ids` referencing the new `RL-SCORE-*`/`RL-END-*`
IDs that RIVLEDOUT-002 adds to RULES.md.

### 2. wasm-api serialization (`crates/wasm-api/src/lib.rs`)

Extend `river_view_json` to serialize the new `terminal_rationale` field into the
river_ledger view JSON, mirroring how the poker_lite view JSON carries its rationale.

### 3. Golden-trace hash migration

Regenerate the `expected_public_view_hashes` (and any dependent replay/export hashes)
for all `games/river_ledger/tests/golden_traces/*.trace.json` affected by the new
public-view field, with a migration note matching the established VICEXPSHASUR style.

## Files to Touch

- `games/river_ledger/src/visibility.rs` (modify)
- `games/river_ledger/src/showdown.rs` (modify — expose structured rationale source if needed)
- `crates/wasm-api/src/lib.rs` (modify — `river_view_json`)
- `games/river_ledger/tests/golden_traces/*.trace.json` (modify — regenerated hashes)

## Out of Scope

- `apps/web/src/wasm/client.ts` TS mirror, `outcomeExplanationTemplates.ts`, and the
  river_ledger `docs/UI.md`/`docs/RULES.md` edits — all RIVLEDOUT-002.
- Any change to the showdown evaluator math or allocation logic.
- New variants or seat-count changes.

## Acceptance Criteria

### Tests That Must Pass

1. `cargo test -p river_ledger` — new rationale projection + no-leak unit tests pass.
2. `cargo run -p replay-check -- --game river_ledger --all` — regenerated hashes verify.
3. `cargo clippy --workspace --all-targets -- -D warnings && cargo test --workspace` — no regressions.

### Invariants

1. The outcome rationale is Rust-owned and viewer-safe: no unrevealed hole card,
   rank, or strength reaches the observer or any non-revealed seat (FOUNDATIONS §11).
2. river_ledger public-view serialization remains deterministic; the hash change is
   captured as an explicit, committed migration, not a silent drift.

## Test Plan

### New/Modified Tests

1. `games/river_ledger/src/visibility.rs` (rationale projection + no-leak unit tests) — assert structured rationale per terminal outcome and fold-out redaction.
2. `games/river_ledger/tests/golden_traces/*.trace.json` — regenerated public-view hashes reflecting the additive field.

### Commands

1. `cargo test -p river_ledger`
2. `cargo run -p replay-check -- --game river_ledger --all`
3. `cargo run -p fixture-check -- --game river_ledger` — narrower fixture-shape check confirming the regenerated traces remain schema-valid.

## Outcome

Completed: 2026-06-15

What changed:

- Added River Ledger's Rust-owned `OutcomeRationaleView` to `PublicView` with
  deterministic `terminal_rationale` projection for last-live fold wins,
  single-winner showdowns, and split showdowns.
- Serialized `terminal_rationale` through `crates/wasm-api` in the shared
  outcome-rationale JSON shape, including stable `template_key`,
  `decisive_rule_ids`, and per-seat final-standing rows.
- Added Rust visibility tests for fold-out redaction, showdown-win rationale,
  and showdown-split rationale, plus a WASM serialization test proving the
  emitted River Ledger template keys and fold-out no-reveal behavior.

Deviations from the plan:

- `games/river_ledger/src/showdown.rs` did not need changes; the existing
  `ShowdownSeatExplanation.revealed` data already carried the structured
  source needed by the view.
- No golden trace files changed. `cargo run -p replay-check -- --game
  river_ledger --all` accepted every existing trace after the additive view
  field, so there was no committed hash migration to record.

Verification:

- `cargo test -p river_ledger` passed.
- `cargo test -p wasm-api river_ledger_view_projects_terminal_rationale_template_keys` passed.
- `cargo run -p replay-check -- --game river_ledger --all` passed.
- `cargo run -p fixture-check -- --game river_ledger` passed.
- `cargo fmt --all --check` passed after formatting.
- `cargo clippy --workspace --all-targets -- -D warnings` passed.
- `cargo test --workspace` passed.

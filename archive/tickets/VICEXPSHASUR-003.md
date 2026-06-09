# VICEXPSHASUR-003: `poker_lite` pilot outcome rationale + no-leak proof

**Status**: COMPLETED
**Priority**: HIGH
**Effort**: Medium
**Engine Changes**: Yes — `games/poker_lite` (`visibility.rs` rationale projection on `ShowdownView`/`TerminalView`, possibly `effects.rs`), per-game docs, and golden traces. No `engine-core`/`game-stdlib` change.
**Deps**: VICEXPSHASUR-001

## Problem

`poker_lite` is the worked failing case: Rust computes `ShowdownStrength { pair_flag, private_rank_value }` and resolves the pool in `compare_showdown`, but `ShowdownView`/`TerminalView` expose only revealed cards and the winner — a player sees *who* won, not *whether* it was pair-vs-rank strength or the private-rank tiebreaker. This pilot adds a viewer-safe `ShowdownRationaleView` projecting the decisive cause, anchors the rationale house style the remaining retrofits (004–008) follow, and proves the yield no-reveal firewall. Source: `archive/specs/victory-explanation-shared-surface.md` §9.9, §15.3, §8.

## Assumption Reassessment (2026-06-09)

1. Verified current code: `ShowdownStrength { pair_flag: bool, private_rank_value: u8 }` (`games/poker_lite/src/rules.rs:15-18`, derives `Ord`/`PartialOrd` so pair beats rank); `showdown_strength` (`rules.rs:40-45`); `compare_showdown` (`rules.rs:47-58`, pair first → rank → `None` on equal/split); `ShowdownView { seat_0_private, seat_1_private, center, winner }` (`visibility.rs:59-64`) exposes cards+winner but **not** the strength comparison; `TerminalView { NonTerminal, YieldWin{winner,loser,shared_pool}, ShowdownWin{winner,shared_pool}, Split{shared_pool,each} }` (`visibility.rs:67-82`); `showdown_view()` returns `None` for `YieldWin` (`visibility.rs:170-180`), so yield already withholds reveal. The rationale must *project* the comparison Rust already computes — never re-derive it downstream.
2. Spec §9.9. Rule IDs use the `CL-` prefix (`games/poker_lite/docs/RULES.md`, e.g. `CL-COMP-001`); the rationale's `decisive_rule_ids` cite the showdown-comparison / pair-vs-rank-order / yield / split / ledger-resolution `CL-` IDs. If a terminal/tiebreak rule the rationale must cite lacks a stable ID, add the ID in `RULES.md` (additive).
3. Cross-artifact boundary under audit: the rationale is projected through `ShowdownView`/`TerminalView` (`visibility.rs`), must round-trip deterministically through `games/poker_lite/tests/golden_traces`, and is consumed by `apps/web/src/wasm/client.ts` (wired in 010). No single file owns it.
4. FOUNDATIONS §2 restated: Rust owns scoring/terminal detection; the decisive cause is computed at terminal detection and projected as view data. TypeScript renders only — it MUST NOT call a `compare_showdown` equivalent or rank private crests.
5. §11 no-leak firewall + determinism: on `YieldWin` the rationale MUST NOT carry the yielded loser's `pair_flag`, `private_rank_value`, crest, or any inferred "would have won" detail (mirror `showdown_view()` returning `None`); on `ShowdownWin`/`Split` the crests/strength are public because the rules revealed them. New view fields must serialize deterministically (golden-trace migration), no RNG/wall-clock in canonical form.
6. Schema extension: this adds a rationale field/variant to the `ShowdownView`/`TerminalView` public/private view schema. Consumers: golden traces (this ticket) and `client.ts` (010). The extension is additive (new field/variant with viewer-scoped projection); no existing variant is removed.

## Architecture Check

1. A game-local `ShowdownRationaleView` (spec D2) keeps the pair-vs-rank vocabulary inside `games/poker_lite` and out of the kernel; the shared UI consumes it without an `engine_core::Outcome`. Projecting at terminal detection (D3) keeps the cause as deterministic view data, not an after-the-fact UI guess.
2. No backwards-compatibility shims: the rationale is an additive viewer-scoped projection; `compare_showdown`/`ShowdownStrength` are reused as-is, not aliased.
3. `engine-core` stays free of mechanic nouns (no `pair`/`crest`/`pot` leak); `game-stdlib` unchanged (no promotion).

## Verification Layers

1. Decisive cause projected at terminal → grep-proof `visibility.rs` carries the rationale on `ShowdownView`/`TerminalView`; unit test asserts pair-beats-high-card, high-card rank win, and equal-strength split each surface the correct cause.
2. Yield no-reveal → no-leak visibility test: on `YieldWin`, the public observer and each seat viewer payload contain no `pair_flag`/`private_rank_value`/crest for the yielded loser (assert absence in serialized view).
3. Showdown reveal lawful → test: on `ShowdownWin`/`Split`, strength data is present *because* the crests are revealed (public).
4. Determinism → golden trace / replay-hash check: regenerated `poker_lite` terminal traces are byte-stable and replay-equal (`docs/TESTING-REPLAY-BENCHMARKING.md`).
5. §2 behavior authority → manual/grep review: no TS comparison is introduced (this ticket is Rust + docs + traces only).

## What to Change

### 1. Project `ShowdownRationaleView` in `games/poker_lite/src/visibility.rs`

Add a viewer-safe rationale reachable through `ShowdownView`/`TerminalView` carrying: `result_kind` (`YieldWin`/`ShowdownWin`/`Split`); `decisive_cause` (pair-beats-non-pair / higher-private-rank / equal-strength-split / opponent-yielded); per-seat showdown strength (pair bucket + rank label + center crest + allocation) **only when legally revealed**; `decisive_rule_ids` (`CL-` IDs); `template_key` (`poker_lite.yield_win_no_reveal`, `poker_lite.pair_beats_high_card`, `poker_lite.private_rank_tiebreak`, `poker_lite.equal_strength_split`). Compute it where `TerminalOutcome` is resolved; reuse `compare_showdown`/`ShowdownStrength`. On `YieldWin`, emit the yield cause with no private reveal.

### 2. Rule-ID + docs alignment

Ensure `games/poker_lite/docs/RULES.md` exposes stable IDs for the showdown comparison, pair-vs-rank order, split, yield, and ledger resolution. Add the "Outcome / victory explanation" section to `games/poker_lite/docs/UI.md` per the template (001): terminal result variants, decisive-cause payload fields, per-player breakdown, hidden-info redaction (yield no-reveal), template keys, and the smoke coverage 011 will add.

### 3. Tests + golden traces

Add visibility/no-leak unit tests (pair-beats-high-card, high-card rank win, equal split, yield no-reveal) and intentionally regenerate the affected `games/poker_lite/tests/golden_traces` terminal entries.

## Files to Touch

- `games/poker_lite/src/visibility.rs` (modify)
- `games/poker_lite/src/effects.rs` (modify — only if the terminal effect must carry the same decisive cause; otherwise omit)
- `games/poker_lite/docs/UI.md` (modify)
- `games/poker_lite/docs/RULES.md` (modify)
- `games/poker_lite/tests/golden_traces/` (modify — intentional terminal-trace regeneration)

## Out of Scope

- Any other game's rationale (004–008).
- The shared panel, templates file, `client.ts` types, or board wiring (009–010).
- The browser smoke (011).
- Introducing any TypeScript comparison/scoring logic, or an `engine-core`/`game-stdlib` outcome type.

## Acceptance Criteria

### Tests That Must Pass

1. `cargo test -p poker_lite` passes, including new tests for pair-beats-high-card, high-card rank win, equal-strength split, and yield no-reveal.
2. The yield no-leak test proves no yielded-loser `pair_flag`/`private_rank_value`/crest appears in any public observer or seat-viewer payload.
3. `cargo run -p replay-check -- --game poker_lite --all` passes (regenerated terminal traces are replay-deterministic).

### Invariants

1. The decisive cause is Rust-projected view data computed at terminal detection; no downstream code re-derives it (FOUNDATIONS §2/§D3).
2. On yield, the rationale reveals no private strength/crest; on showdown/split, strength appears only because the rules revealed the crests (FOUNDATIONS §11 no-leak firewall).

## Test Plan

### New/Modified Tests

1. `games/poker_lite/src/visibility.rs` (or `tests/`) — rationale + no-leak unit tests (the four decisive-cause/yield cases).
2. `games/poker_lite/tests/golden_traces/` — intentionally regenerated terminal traces carrying the new rationale fields.

### Commands

1. `cargo test -p poker_lite`
2. `cargo run -p replay-check -- --game poker_lite --all`
3. `cargo run -p fixture-check -- --game poker_lite` (confirms fixtures/traces remain consistent after schema growth)

## Outcome

Completed: 2026-06-09

What changed:

- Added game-local `OutcomeRationaleView`, `SeatOutcomeBreakdownView`, and `ShowdownStrengthView` projection in `games/poker_lite/src/visibility.rs`.
- Added rationale fields to `ShowdownView` and terminal `YieldWin` / `ShowdownWin` / `Split` variants, including decisive cause IDs, static template keys, stable `CL-*` rule refs, per-seat allocations/contributions, and revealed strength rows only for showdown/split.
- Preserved yield no-reveal behavior: yield rationale carries no strength rows and no private crest facts; public observer and winning-seat views do not receive the yielded loser's private crest.
- Added `poker_lite` UI outcome documentation with terminal variants, payload fields, no-leak rules, template keys, accessibility/reduced-motion expectations, and future smoke cases.
- Added visibility tests for pair beats high card, private-rank tiebreak, equal-strength split, and yield rationale no-reveal.
- Updated affected `poker_lite` golden trace public-view/private-view/public-export hashes with migration notes for the terminal outcome-rationale view growth.
- Tightened `scripts/check-outcome-explanations.mjs` so stable scoring/end rule IDs may use game-local prefixes such as `CL-SCORE-*` and `CL-END-*`.

Deviations from original plan:

- `games/poker_lite/src/effects.rs` was not changed; the terminal public view already carries the rationale and existing terminal/showdown effects remain deterministic.
- `RULES.md` did not need new IDs because the existing `CL-SCORE-*`, `CL-END-*`, `CL-PLEDGE-*`, `CL-REVEAL-*`, and `CL-VIS-*` IDs cover the rationale refs.
- The losing seat's ordinary private view still shows only its own private crest, consistent with the existing owner-private view contract; the new outcome rationale itself never reveals or derives yielded-loser strength.

Verification results:

- `cargo fmt --all --check` passed.
- `cargo test -p poker_lite` passed, including the new rationale/no-leak visibility tests.
- `cargo run -p replay-check -- --game poker_lite --all` passed.
- `cargo run -p fixture-check -- --game poker_lite` passed.
- `node scripts/check-doc-links.mjs` passed (`Checked 25 markdown files`).
- `git diff --check` passed.
- `node scripts/check-outcome-explanations.mjs` still exits non-zero as expected until later tickets add `client.ts`, shared template, and remaining-game coverage; after this ticket it no longer reports missing `poker_lite` UI/RULES outcome documentation.

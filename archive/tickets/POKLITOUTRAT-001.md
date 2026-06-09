# POKLITOUTRAT-001: Project poker_lite outcome rationale into the WASM public view

**Status**: COMPLETED
**Priority**: HIGH
**Effort**: Small
**Engine Changes**: Yes — `crates/wasm-api` (JSON bridge serialization only); no `engine-core`, `game-stdlib`, or `games/*` rule/view changes. Optional presentation hardening in `apps/web`.
**Deps**: None (completes the VICEXPSHASUR "victory explanation shared surface" wiring for `poker_lite`; the VICEXPSHASUR spec/tickets are archived/Done)

## Problem

Every Crest Ledger (`poker_lite`) **showdown win** is explained in the browser as
"`<winner>` wins with a pair", even when no pair exists and the win was decided by
higher private rank. Reproduced live (bot-vs-bot autoplay) across seeds:

| Seed | Seat 0 | Center | Seat 1 | True cause | Shown text |
|---|---|---|---|---|---|
| 2 | Current (mid) | Sprout (low) | Crown (high) | Seat 1, higher rank (no pair) | ❌ "seat_1 wins with a pair" |
| 3 | Crown (high) | Current (mid) | Sprout (low) | Seat 0, higher rank (no pair) | ❌ "seat_0 wins with a pair" |

Because the explanation text is identical for every win while the winner changes
game to game, the showdown reads as arbitrary ("same values, different winner").

The Rust engine already computes the **correct** rationale
(`decisive_cause` = `pair_beats_high_card` vs `higher_private_rank` vs
`equal_strength_split`, with the matching `template_key`). It is simply never
serialized into the public view JSON the browser consumes, so the TypeScript
presentation layer falls back to a hardcoded template that assumes "pair" for all
showdown wins. This ticket projects the existing Rust rationale into the WASM JSON
so the UI uses Rust's authoritative explanation.

This keeps explanation authority in Rust (FOUNDATIONS §2 behavior authority): the
fix is a missing serialization wire, not new behavior or a TypeScript-side
re-derivation of outcome semantics.

## Assumption Reassessment (2026-06-09)

1. **Rust computes the rationale correctly and it is already in the view struct.**
   `games/poker_lite/src/visibility.rs` `outcome_rationale()` builds
   `OutcomeRationaleView { result_kind, decisive_cause, template_key,
   decisive_rule_ids, per_seat: [SeatOutcomeBreakdownView; 2] }`.
   `showdown_template_key()` maps `pair_beats_high_card → "poker_lite.pair_beats_high_card"`
   and `higher_private_rank → "poker_lite.private_rank_tiebreak"`. The rationale is
   attached to every terminal variant via `terminal_view()`
   (`TerminalView::{YieldWin,ShowdownWin,Split}.rationale`) and, for showdowns, also
   to `ShowdownView.rationale`. Verified by code read and by the live reproduction
   above.
2. **The WASM bridge drops it.** `crates/wasm-api/src/lib.rs` `poker_view_json`
   (≈ line 4871) emits no `terminal_rationale` key; `poker_showdown_json`
   (≈ 4924) and `poker_terminal_json` (≈ 4946) omit the `rationale` field of the
   structs they serialize. `grep -c terminal_rationale crates/wasm-api/src/lib.rs`
   returns `0` — no game emits the key; the other four games' boards happen to be
   correct only because their hardcoded fallbacks are unambiguous from
   `terminal_kind`. `poker_lite` is the one game whose fallback cannot disambiguate
   its two showdown-win causes.
3. **The consumer contract already exists and prefers the Rust value.** TS type
   `PokerLitePublicView.terminal_rationale?: PokerLiteOutcomeRationale | null`
   (= `OutcomeRationalePayload`) in `apps/web/src/wasm/client.ts`.
   `apps/web/src/components/PokerLiteBoard.tsx` already passes
   `rationale: view.terminal_rationale` into `outcomeSurfaceData`, which prefers
   `input.rationale?.template_key` over the hardcoded `pokerTemplateKey(view)`
   fallback (`OutcomeExplanationPanel.tsx` `outcomeSurfaceData`). All four poker
   templates — including `poker_lite.private_rank_tiebreak` — already exist in
   `apps/web/src/components/outcomeExplanationTemplates.ts`. **Consequence: once the
   JSON carries the key, the UI fix requires no further TS change.**
4. **Shared boundary under audit: the public-view JSON contract between
   `crates/wasm-api` and `apps/web`.** The extension is **additive-only** (one new
   top-level optional field, already declared optional/nullable on the TS side), so
   no existing consumer breaks.
5. **Deterministic replay/hash (FOUNDATIONS §11/§13): unaffected.**
   `games/poker_lite/tests/replay.rs` hashes the Rust view via
   `view_hash`/`StableSerialize` (`PublicView::stable_summary`, which already folds
   in the rationale through `encode_showdown`/`encode_terminal`), **not** the
   `poker_view_json` string. No fixture hashes `poker_view_json`
   (`grep` finds no poker `expected_public_view_hashes`). Adding an unhashed JSON
   field changes no golden-trace or replay hash → **no fixture migration**. Confirm
   during implementation that no `apps/web` wasm smoke snapshot pins the poker view
   JSON; update it if one exists.
6. **Hidden-information no-leak firewall (FOUNDATIONS §11): must be preserved.**
   The rationale for `YieldWin` must not reveal private crests, because a yield ends
   the match with no showdown. The Rust `yield_breakdown` already sets
   `strength: None`, so the projected `final_standing` for a yield carries no
   private rank/pair fields. This must be asserted, not assumed.
7. **Adjacent contradiction classification.** The hardcoded
   `pokerTemplateKey(view)` mapping in `PokerLiteBoard.tsx` (always
   `poker_lite.pair_beats_high_card` for `showdown_win`) becomes dead for terminal
   states once the Rust rationale is present, but remains a latent wrong-fallback if
   the rationale is ever absent. Treated as **optional in-scope hardening** (see
   What to Change §2), not a required consequence.
8. **Mismatch + correction.** None to the engine. The original brainstorm
   hypothesis that a fixture-hash migration would be required is **corrected** by
   item 5: no migration is needed.

## Architecture Check

1. **Cleaner than the alternative** (deriving the cause in TypeScript from
   `view.showdown`): that alternative would duplicate, in TS, outcome semantics the
   Rust engine already computes — tensioning FOUNDATIONS §2 behavior authority and
   the very surface VICEXPSHASUR centralized. Projecting the existing Rust rationale
   keeps Rust the single source of truth and reuses the already-built TS seam.
2. **No backwards-compatibility shims or alias paths.** The change adds one JSON
   field consumed by an already-present optional TS field; no parallel/legacy path.
3. **`engine-core` untouched** (no mechanic nouns added; this is `wasm-api`
   serialization). No `game-stdlib` change, so the §4 earned-helper gate does not
   apply.

## Verification Layers

1. WASM JSON emits a `terminal_rationale` whose `template_key` matches the Rust
   `decisive_cause` for each terminal kind → schema/serialization validation
   (new `crates/wasm-api` inline test asserting `private_rank_tiebreak` for a
   higher-rank showdown, `pair_beats_high_card` for a pair, `equal_strength_split`
   for a tie, `yield_win_no_reveal` for a yield).
2. No private crest leaks via the yield-win rationale → no-leak visibility test
   (assert the projected yield-win `final_standing` contains no private rank / pair
   / crest-id fields).
3. Deterministic replay/hash unchanged → golden trace / replay-hash check
   (`cargo test -p poker_lite` + `cargo run -p replay-check -- --game poker_lite
   --all` pass with no fixture edits).
4. Browser shows the correct, cause-specific summary at showdown → UI smoke
   (manual Puppeteer play-through, plus `npm --prefix apps/web run smoke:wasm` /
   `smoke:ui` green).
5. Additive-only contract conformance (no existing consumer breaks) → codebase
   grep-proof (`terminal_rationale` is optional/nullable in `client.ts`; no
   required-field consumer added).

## What to Change

### 1. Project the rationale in `crates/wasm-api/src/lib.rs`

- Add a top-level `"terminal_rationale":<...>` field to the `poker_view_json`
  format string, sourced from `view.terminal` (the canonical terminal source —
  covers yield, showdown, and split; `TerminalView::NonTerminal` → `null`).
- Add helper `poker_rationale_json(&poker_lite::visibility::OutcomeRationaleView)
  -> String` (plus a small `poker_terminal_rationale(&TerminalView) ->
  Option<&OutcomeRationaleView>` accessor) emitting the
  `OutcomeRationalePayload` shape the TS contract expects:
  - `result_kind`, `decisive_cause`, `template_key` (strings),
  - `decisive_rule_ids` (string array),
  - `final_standing`: map each `per_seat` `SeatOutcomeBreakdownView` →
    `{ "seat", "result", "emphasized": result == "win",
       "values": [ {"label":"Contribution","value":<contribution>},
                   {"label":"Allocation","value":<allocation>},
       /* only when strength.is_some() (showdown/split, never yield): */
                   {"label":"Pair","value":<pair_bucket>},
                   {"label":"Private rank","value":<private_rank>} ] }`.
  - Omit `template_params` (the board already supplies `{winner,loser}` via the
    adapter fallback, satisfying the templates' `requiredParams`).
- Reuse existing JSON-escaping helpers (`escape_json`) and the established
  per-game serializer style; do not introduce a serde dependency.

### 2. (Optional hardening) `apps/web/src/components/PokerLiteBoard.tsx`

The hardcoded `pokerTemplateKey(view)` showdown mapping is superseded once the
rationale is present. Either leave it as the inert fallback, or make it honest by
deriving the cause from `view.showdown` so an absent-rationale fallback is still
correct. No required UI change otherwise — the adapter already consumes
`view.terminal_rationale`.

## Files to Touch

- `crates/wasm-api/src/lib.rs` (modify — `poker_view_json` + new
  `poker_rationale_json`/accessor helpers + inline `#[test]`s under the existing
  `mod tests`)
- `apps/web/src/components/PokerLiteBoard.tsx` (modify — optional §2 hardening only)

## Out of Scope

- The showdown-panel visual legibility redesign (flagging which private pairs the
  center, surfacing Sprout<Current<Crown ordering) — a separate follow-up; this
  ticket only makes the data available.
- The other four games' rationale projection — their hardcoded fallbacks are
  already correct; no behavior bug to fix there. (A future consistency pass could
  project all games, but is not required here.)
- Any `engine-core`, `game-stdlib`, or `games/poker_lite` rule/view/struct change.
- Any replay-fixture or golden-trace hash edit (none is required; see Assumption 5).

## Acceptance Criteria

### Tests That Must Pass

1. New `crates/wasm-api` inline tests: for constructed `poker_lite` terminal views,
   `poker_view_json` emits `terminal_rationale.template_key` =
   `"poker_lite.private_rank_tiebreak"` for a higher-rank showdown,
   `"poker_lite.pair_beats_high_card"` for a pair showdown,
   `"poker_lite.equal_strength_split"` for a tie, and
   `"poker_lite.yield_win_no_reveal"` for a yield; non-terminal view emits
   `"terminal_rationale":null`.
2. New no-leak assertion: the yield-win `terminal_rationale.final_standing` contains
   no private-crest / private-rank / pair field.
3. `cargo test --workspace` and
   `cargo run -p replay-check -- --game poker_lite --all` pass with **no** fixture
   changes; `cargo fmt --all --check` and
   `cargo clippy --workspace --all-targets -- -D warnings` clean.
4. `npm --prefix apps/web run smoke:wasm` and
   `npm --prefix apps/web run smoke:ui` green.

### Invariants

1. Outcome-explanation authority stays in Rust: the browser summary at showdown is
   driven by the Rust-supplied `template_key`/`decisive_cause`, not the TS
   hardcoded fallback (FOUNDATIONS §2).
2. The public-view JSON extension is additive-only: `terminal_rationale` is
   optional/nullable and no existing field changes shape (boundary contract between
   `crates/wasm-api` and `apps/web`).
3. No hidden information leaks: a yield-win rationale never carries either player's
   private crest (FOUNDATIONS §11 no-leak firewall).

## Test Plan

### New/Modified Tests

1. `crates/wasm-api/src/lib.rs` (inline `mod tests`) — assert `terminal_rationale`
   presence, per-kind `template_key`, and the yield no-leak invariant directly on
   `poker_view_json` output.
2. None added to `games/poker_lite/tests/` — the engine and its hashes are
   unchanged; existing `replay.rs` / golden traces serve as the regression guard
   that the Rust view (and its `StableSerialize` hash) did not move.

### Commands

1. `cargo test -p wasm-api` (targeted: the new serialization + no-leak assertions).
2. `cargo test --workspace && cargo run -p replay-check -- --game poker_lite --all`
   (full pipeline: proves no Rust hash/fixture regression).
3. `npm --prefix apps/web run smoke:wasm && npm --prefix apps/web run smoke:ui`
   (browser bridge + UI). A narrower web command is correct because the only
   browser-visible change is the additive JSON field consumed by the existing
   outcome panel.

## Outcome

Completed: 2026-06-09

What changed:

- `crates/wasm-api/src/lib.rs` now emits additive
  `terminal_rationale` JSON for `poker_lite` terminal views, sourced from the
  Rust `TerminalView` rationale.
- The rationale payload carries Rust-owned `result_kind`, `decisive_cause`,
  `template_key`, `decisive_rule_ids`, and `final_standing` rows. Yield wins omit
  showdown strength rows, so private crests/ranks remain unrevealed.
- Added inline `wasm-api` tests covering non-terminal `null`, higher-private-rank
  showdown, pair showdown, split, yield, and the yield no-leak invariant.
- Strengthened `apps/web/e2e/poker-lite.smoke.mjs` with a seed-2 browser
  showdown assertion that requires "wins on the revealed showdown rank" and
  rejects the old "wins with a pair" fallback.

Deviations from original plan:

- No `PokerLiteBoard.tsx` hardening was needed. The existing optional rationale
  adapter already prefers the Rust-supplied `template_key`; the remaining fallback
  stays inert for terminal views once the WASM field is present.
- No replay fixture or hash migration was needed.

Verification:

- `cargo test -p wasm-api poker_lite_` passed.
- `cargo test -p wasm-api` passed.
- `cargo fmt --all --check` passed.
- `cargo clippy --workspace --all-targets -- -D warnings` passed.
- `cargo test --workspace` passed.
- `cargo run -p replay-check -- --game poker_lite --all` passed.
- `npm --prefix apps/web run smoke:wasm` passed.
- `npm --prefix apps/web run smoke:ui` passed.
- `node apps/web/e2e/poker-lite.smoke.mjs` passed.

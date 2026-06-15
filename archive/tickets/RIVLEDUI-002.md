# RIVLEDUI-002: Make the Seats setup control match the game's seat range

**Status**: COMPLETED
**Priority**: HIGH
**Effort**: Medium
**Engine Changes**: None — TypeScript/React presentation + shell state only (`apps/web/src/components/MatchSetup.tsx`, `apps/web/src/state/shellReducer.ts`, `apps/web/src/main.tsx`). Rust already owns seat-count setup and validation; this ticket only wires the existing input through the UI.
**Deps**: none

## Problem

The SETUP → "Seats" control is wrong in two ways:

1. **Fixed-seat games show a pointless disabled `<select>`.** For every game
   whose `supported_seats` has a single value (e.g. `[2]`), `MatchSetup.tsx:116`
   still renders a `<select … disabled>` with one option plus the caption
   "Supported count: 2; default 2." A dropdown that can never change implies
   interactivity that does not exist. The seat count should be stated plainly as
   read-only text.

2. **Variable-seat games cannot choose a seat count.** River Ledger supports
   `[3, 4, 5, 6]` (default 6), but the same `<select>` is hardcoded `disabled`,
   has no `onChange`, and the chosen value is never stored — so the user cannot
   pick 3, 4, or 5 seats even though the rules support them. The
   [MULTI-SEAT-AND-SURFACE-CONTRACT.md](../docs/MULTI-SEAT-AND-SURFACE-CONTRACT.md)
   treats "Default seats" as the *public setup default* and assigns Rust the
   "wrong-seat-count diagnostics" (lines 44, 47), i.e. the public setup is
   expected to offer a seat-count choice within the supported range.

The end-to-end plumbing for the choice **already exists**: `start()`
(`main.tsx:209-214`) calls `api.newMatch(gameId, seed, variant, seatCount)`,
`client.ts:1322-1328` forwards `seatCount` to the Rust export
`rulepath_new_match_with_seat_count`, and Rust `new_match_with_seat_count`
(`crates/wasm-api/src/lib.rs:649`) builds the match for that count. The only gap
is the presentation layer: `start()` always passes `selectedGame?.default_seats`
because the UI has no state to hold a user-chosen seat count.

## Assumption Reassessment (2026-06-15)

1. `apps/web/src/components/MatchSetup.tsx:114-128` renders the Seats field: a
   hardcoded `<select … disabled aria-label="Supported seats from Rust catalog">`
   built from `supportedSeatCounts(selectedGame)` (`game.supported_seats ?? []`,
   lines 228-230), value `defaultSeatCount = selectedGame?.default_seats ??
   seatCounts[0] ?? ""` (line 49), plus `seatCountDetail()` caption (lines
   243-253). `MatchSetup` has no seat-count callback prop. Confirmed by read.
2. `apps/web/src/state/shellReducer.ts:51-55` `setup` state has `{ seed,
   playMode, variantId }` — **no `seatCount`**. `gameSelected`
   (lines 195-218) resets `variantId` from the catalog but not seats; `catalog`
   (line 48) is `GameCatalogEntry[]`. Confirmed by read.
3. `apps/web/src/main.tsx:204-217` `start()` passes `selectedGame?.default_seats`
   as the 4th `api.newMatch` argument. `selectedVariantForStart`
   (lines 843-848) returns `undefined` for games with ≤1 variant, so River
   Ledger (single variant `river_ledger_standard`) takes the
   `seatCount && !variantId` branch in `newMatch` (`client.ts:1323`) and the
   seat count is honored today (verified: `apps/web/e2e/river-ledger.smoke.mjs:147`
   asserts six seat rows). Confirmed by read.
4. Behavior-authority boundary under audit (FOUNDATIONS §2): Rust owns setup,
   validation, and view projection; TypeScript owns "game picker and setup UI".
   This ticket keeps that split — the UI only *selects an input value* (a seat
   count) from the Rust-supplied `supported_seats` and hands it to Rust
   `new_match_with_seat_count`, which performs the actual seated setup. TS makes
   no legality decision and invents no seat range; the options come straight
   from the catalog. No §12 stop condition is tripped.
5. Schema note (additive, no Rust change): the consumed catalog fields
   `supported_seats` / `default_seats` already exist on `GameCatalogEntry`
   (`client.ts:92-95`). The new `setup.seatCount` field is internal shell state,
   not a serialized contract, action tree, command/effect envelope, view, trace,
   or save — it does not touch deterministic replay/hash or any persisted
   schema, and carries no hidden information.
6. Mismatch + correction: none. The reported behavior matches current code.
7. Adjacent contradiction uncovered (classified as future cleanup, **not** this
   ticket): `newMatch` (`client.ts:1329`) routes to the variant-only export
   whenever `variantId` is set, silently dropping `seatCount`; the combined
   export `rulepath_new_match_with_variant_and_seat_count`
   (`crates/wasm-api/src/lib.rs:178`) is not bound in the client. No shipping
   game is both multi-variant and variable-seat (River Ledger is the only
   variable-seat game and has one variant), so wiring the combined path now
   would be speculative generality (FOUNDATIONS §1). See Out of Scope.

## Architecture Check

1. **Chosen approach — one control with two render modes, driven by
   `supported_seats.length`.** When the catalog reports a single supported seat
   count, render a static read-only value (no `<select>`); when it reports more
   than one, render an **enabled** `<select>` bound to new shell state
   `setup.seatCount` that `start()` passes to the already-wired
   `api.newMatch(..., seatCount)`. This is cleaner than keeping a disabled
   dropdown (which lies about interactivity) and cleaner than two separate
   components (the modes share the caption and the catalog data source).
2. The seat options and default come **only** from the Rust-projected catalog
   (`supported_seats`, `default_seats`); the UI never invents or filters the
   range, preserving FOUNDATIONS §2. Rust still validates the count and emits
   wrong-seat-count diagnostics.
3. No backwards-compatibility aliasing/shims introduced. No `engine-core` /
   `game-stdlib` changes and no mechanic nouns added; this is presentation +
   shell-reducer state only.

## Verification Layers

1. TS does not decide legality / invent seat range (FOUNDATIONS §2) ->
   FOUNDATIONS alignment check (options enumerated from `supported_seats`;
   chosen count passed to Rust `new_match_with_seat_count`, which validates).
2. Fixed-seat games render no seat `<select>` -> UI smoke (assert no enabled
   seat selector for a single-`supported_seats` game; seat count shown as text).
3. Variable-seat selection actually changes the seated match -> simulation/UI
   smoke (`apps/web/e2e/river-ledger.smoke.mjs`: pick a non-default supported
   count and assert the rendered seat-row count matches the choice).
4. No hidden-information leak via the new state -> no-leak visibility test
   (`apps/web/e2e/river-ledger.smoke.mjs` existing leak assertions still pass;
   `setup.seatCount` is a plain integer choice, not seat-private data).

## What to Change

### 1. Add `seatCount` to shell setup state (`apps/web/src/state/shellReducer.ts`)

- Add `seatCount: number | null` to the `setup` state shape (line 51-55) and to
  the initial state (line 135-138), defaulting to `null` until a game is chosen.
- Add an action `{ type: "setupSeatCountChanged"; seatCount: number }` and a
  reducer case that stores it.
- In `gameSelected` (lines 195-218) and on initial catalog load
  (`wasmLoaded`, lines 171-184), initialize `setup.seatCount` to the selected
  game's `default_seats`, clamped to membership in `supported_seats` (fallback
  `supported_seats[0] ?? null`). Reset it whenever the selected game changes,
  exactly as `variantId` is reset today.

### 2. Render the Seats field in two modes (`apps/web/src/components/MatchSetup.tsx`)

- Add props `seatCount: number | null` and `onSeatCountChange: (seatCount:
  number) => void` to `MatchSetupProps`.
- Replace the Seats `<label>` block (lines 114-128):
  - **`seatCounts.length <= 1`**: render the count as static read-only text
    (e.g. an `<output>`/`<span>` showing the single value with a "seats" label),
    no `<select>`. Simplify the caption so it does not imply a choice
    (e.g. "Fixed at N seats." via a single-count branch in `seatCountDetail`).
  - **`seatCounts.length > 1`**: render an **enabled** `<select>` whose `value`
    is the resolved chosen count (`seatCount ?? defaultSeatCount`), with
    `onChange={(e) => onSeatCountChange(Number(e.currentTarget.value))}`, options
    from `seatCounts`, and the existing "Supported counts: …; default …."
    caption.
- Keep the `aria-label` and ensure the enabled control is keyboard- and
  screen-reader-operable.

### 3. Pass the chosen seat count when starting (`apps/web/src/main.tsx`)

- In `start()` (lines 204-217), pass the resolved chosen count
  (`state.setup.seatCount ?? selectedGame?.default_seats`) as the 4th
  `api.newMatch` argument instead of `selectedGame?.default_seats`, and add it
  to the `useCallback` dependency list.
- Wire `MatchSetup` (rendered around lines 521-531) with
  `seatCount={state.setup.seatCount}` and `onSeatCountChange={(seatCount) =>
  dispatch({ type: "setupSeatCountChanged", seatCount })}`.

## Files to Touch

- `apps/web/src/state/shellReducer.ts` (modify)
- `apps/web/src/components/MatchSetup.tsx` (modify)
- `apps/web/src/main.tsx` (modify)
- `apps/web/e2e/river-ledger.smoke.mjs` (modify — assert non-default seat-count selection drives the seated match)

## Out of Scope

- **Combined variant + seat-count path.** Do not bind
  `rulepath_new_match_with_variant_and_seat_count` or change `newMatch`'s
  variant branch. No shipping game is both multi-variant and variable-seat;
  adding it now is speculative (FOUNDATIONS §1). If/when such a game ships, it
  becomes its own ticket (see Assumption Reassessment item 7).
- Any Rust change (`crates/wasm-api`, `games/*`): seat-count setup, validation,
  and wrong-seat-count diagnostics already exist and are unchanged.
- The catalog-card "Selected" pill overlap (covered by
  `archive/tickets/RIVLEDUI-001.md`).
- Seat-count selection in the replay/import flow or anywhere other than match
  setup.

## Acceptance Criteria

### Tests That Must Pass

1. For a single-`supported_seats` game (e.g. `race_to_n`), the SETUP → Seats
   field renders the count as static text with **no** enabled `<select>`, and
   the caption does not imply a choice.
2. For River Ledger (`supported_seats: [3, 4, 5, 6]`), the Seats `<select>` is
   enabled; selecting `4` and starting a match yields a match rendered with the
   number of seats matching the choice (extend `river-ledger.smoke.mjs`'s
   seat-row assertion from the fixed 6 to the chosen count).
3. River Ledger's existing no-leak and private-view assertions still pass.
4. `npm --prefix apps/web run build` succeeds.
5. `npm --prefix apps/web run smoke:e2e` passes.

### Invariants

1. The Seats control's options and default derive solely from the Rust catalog
   (`supported_seats` / `default_seats`); the UI never invents or filters the
   seat range (FOUNDATIONS §2).
2. The chosen seat count reaches Rust via `api.newMatch(..., seatCount)`; Rust
   remains the setup/validation authority.
3. `setup.seatCount` introduces no serialized-contract, replay/hash, or
   hidden-information change; it is internal shell state only.

## Test Plan

### New/Modified Tests

1. `apps/web/e2e/river-ledger.smoke.mjs` — select a non-default supported seat
   count (e.g. 4) in setup, start the match, and assert the rendered
   `.river-ledger-seat` row count equals the chosen count (replacing the
   hardcoded `=== 6` assertion at line 147 with the selected value). Existing
   leak / private-view assertions retained.
2. `apps/web/e2e/shell.smoke.mjs` (or `river-ledger.smoke.mjs`) — assert a
   single-`supported_seats` game shows no enabled seat `<select>` and renders
   the seat count as static text.

### Commands

1. `node apps/web/e2e/river-ledger.smoke.mjs` — targeted run while iterating.
2. `npm --prefix apps/web run build`
3. `npm --prefix apps/web run smoke:e2e` — full web e2e gate.

## Outcome

Completed: 2026-06-15

What changed:

- Added internal shell setup state for `seatCount`, initialized from the
  selected game's Rust catalog `default_seats` when valid and otherwise the
  first `supported_seats` value.
- Changed `MatchSetup` so fixed-seat games render a read-only seat-count value
  with no seat `<select>`, while variable-seat games render an enabled selector
  whose options come only from Rust catalog metadata.
- Passed the selected seat count through `api.newMatch(..., seatCount)`, using
  the existing Rust/WASM seat-count setup path.
- Extended `apps/web/e2e/river-ledger.smoke.mjs` to prove the fixed-seat setup
  has no seat selector, River Ledger exposes enabled `3/4/5/6` seat choices,
  selecting `4` starts a four-seat rendered match, and the existing no-leak /
  private-view assertions still pass.

Deviations from plan:

- None. The combined variant + seat-count WASM path remains out of scope
  because no current catalog game is both multi-variant and variable-seat.

Verification:

- `npm --prefix apps/web run build`
- `node apps/web/e2e/river-ledger.smoke.mjs`
- `npm --prefix apps/web run smoke:e2e`

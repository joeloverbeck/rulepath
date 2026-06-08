# GAT9TOKBAZBRO-019: Fix token_bazaar action-card resource deltas (parser delimiter)

**Status**: COMPLETED
**Priority**: HIGH
**Effort**: Small
**Engine Changes**: None — presentation-only; touches `apps/web` (`TokenBazaarBoard.tsx`, `e2e/token-bazaar.smoke.mjs`). No `engine-core`/`game-stdlib`/`games/*`/schema/trace changes.
**Deps**: None

## Problem

Every token_bazaar action card in the web UI renders its resource cost/gain as
`amber 0 / jade 0 / iron 0`, so the player cannot see what an action yields
(e.g. "Collect amber" should show gain `amber 2`). The data is correct and
already present: Rust emits the deltas in action metadata, and contract-cost
chips — which read structured view data — render real numbers. The defect is a
delimiter mismatch in the TypeScript parser only.

Rust serializes counts as colon-separated pairs
(`resource_counts_stable`, `games/token_bazaar/src/actions.rs:217-222`):

```
"amber:2,jade:0,iron:0"
```

The presenter's `parseCounts` splits each comma-part on `=` instead of `:`
(`apps/web/src/components/TokenBazaarBoard.tsx:243`), so the key parsed from
`"amber:2"` is the literal `"amber:2"`, never matches `amber|jade|iron`, and
every field keeps its `0` initializer.

## Assumption Reassessment (2026-06-08)

1. `resource_counts_stable` emits `format!("amber:{},jade:{},iron:{}", ...)`
   (colon-delimited) at `games/token_bazaar/src/actions.rs:217-222`; its Rust
   test asserts `Some("amber:1,jade:1,iron:1")` at `actions.rs:295`. The
   `cost`/`gain`/`points` metadata keys are attached in `collect_choice`
   (`actions.rs:137`), `exchange_choice` (`actions.rs:157-158`), and
   `fulfill_choice` (`actions.rs:184`). The format is the Rust-side contract and
   is correct — the bug is consumer-side.
2. `parseCounts` (`apps/web/src/components/TokenBazaarBoard.tsx:237-249`) does
   `part.split("=")`; `ActionMetadata` (`TokenBazaarBoard.tsx:199-210`) feeds it
   the `cost`/`gain` metadata values via `metadataValue`. Contract costs render
   correctly because `slot.contract.cost` is a structured
   `TokenBazaarResourceCounts` (`TokenBazaarBoard.tsx:95`) and never passes
   through `parseCounts`.
3. Cross-artifact boundary under audit: the action-metadata string contract
   between Rust (`resource_counts_stable`) and the TS presenter (`parseCounts`).
   `engine-core`'s `ActionChoice.metadata` is a generic `Vec<ActionMetadata>`
   key/value **string** map (`crates/engine-core/src/action.rs`); packing counts
   as a string is the sanctioned generic-metadata pattern. The contract is the
   string format, and only the consumer is wrong.
4. `docs/UI-INTERACTION.md:95` designates "UI metadata" (labels, layout hints)
   as Rust/static typed content the presenter renders — the presenter must
   faithfully display Rust's computed deltas; it must not recompute or decide
   them. This fix makes the presenter render the value Rust already supplies, so
   it strengthens, not weakens, the present-only boundary.
5. No hidden-information surface: action `cost`/`gain`/`points` are public
   action-affordance metadata already in the payload and already rendered (as
   zeros). Displaying the correct numbers exposes nothing new — the existing
   `assertNoLeak` passes on the same DOM today.
6. The metadata contract is not extended or changed. The fix is read-side only;
   the wire format (`amber:X,jade:Y,iron:Z`) is unchanged, so all other
   consumers and golden traces are unaffected (additive: none; breaking: none).

## Architecture Check

1. Correcting the parser delimiter is the minimal aligned fix: the data,
   wire format, and Rust contract are already correct, so the change is confined
   to one consumer function. Rejected alternative — restructuring action deltas
   into typed JSON fields like `slot.contract.cost`: that would push mechanic
   nouns/typed count fields into the generic `engine-core` `ActionChoice`
   metadata contract, contaminating the noun-free kernel (FOUNDATIONS §3). The
   packed-string-in-generic-metadata design is correct and is preserved.
2. No backwards-compatibility shim or aliasing: the broken `=` split is
   replaced outright; no dual-format parsing is introduced.
3. No `engine-core` change and no `game-stdlib` change — presentation-only.
   `engine-core` stays noun-free.

## Verification Layers

1. Presenter renders Rust's computed gain (e.g. `collect/amber` → gain
   `amber 2`) -> e2e smoke assertion reading the `token-action-collect-amber`
   card's gain chip in `apps/web/e2e/token-bazaar.smoke.mjs`.
2. TS parser matches the Rust `resource_counts_stable` format exactly (`:`
   delimiter, `amber,jade,iron` order) -> codebase grep-proof pairing
   `games/token_bazaar/src/actions.rs:217-222` with the corrected
   `parseCounts`, plus the round-trip exercised by assertion (1).
3. Correcting the displayed deltas leaks no hidden information -> existing
   `assertNoLeak` / `assertNoForbiddenTerms` passes in the same smoke run.

## What to Change

### 1. Fix the `parseCounts` delimiter

In `apps/web/src/components/TokenBazaarBoard.tsx`, change `parseCounts` to split
each comma-separated part on `:` (matching `resource_counts_stable`) instead of
`=`:

```ts
const [key, raw] = part.split(":");
```

No other change to the function; the `amber/jade/iron` guard and
`Number(raw) || 0` fallback remain.

### 2. Add a regression assertion to the e2e smoke

In `apps/web/e2e/token-bazaar.smoke.mjs`, after the board renders
(`assertBoardA11y`) and before the collect action is taken, read the gain chips
inside the `token-action-collect-amber` card and assert the amber gain is the
non-zero Rust value (`2`). This guards the exact regression that shipped: action
cards displaying all-zero deltas while the metadata carried real numbers.

## Files to Touch

- `apps/web/src/components/TokenBazaarBoard.tsx` (modify)
- `apps/web/e2e/token-bazaar.smoke.mjs` (modify)

## Out of Scope

- Issue 1 (cramped seat/supply inventory chip labels) — tracked in
  GAT9TOKBAZBRO-020.
- Restructuring action deltas into typed view fields (rejected; would
  contaminate generic `engine-core` metadata).
- Any Rust, schema, trace, or wire-format change.

## Acceptance Criteria

### Tests That Must Pass

1. New smoke assertion: the `token-action-collect-amber` card renders gain
   `amber = 2` (non-zero), with `jade`/`iron` correctly `0`.
2. `npm --prefix apps/web run smoke:e2e` passes (includes
   `e2e/token-bazaar.smoke.mjs`).
3. `npm --prefix apps/web run build` succeeds (TypeScript compiles).

### Invariants

1. The presenter renders Rust-computed action deltas verbatim; it never
   recomputes or decides them (`docs/UI-INTERACTION.md:95`, FOUNDATIONS
   present-only).
2. The TS count parser stays in lockstep with the Rust `resource_counts_stable`
   wire format (`amber:X,jade:Y,iron:Z`); no second format is introduced.

## Test Plan

### New/Modified Tests

1. `apps/web/e2e/token-bazaar.smoke.mjs` — add a gain-chip assertion on the
   collect-amber action card, guarding the all-zeros regression.

### Commands

1. `npm --prefix apps/web run smoke:e2e`
2. `npm --prefix apps/web run build`
3. The web app has no unit-test framework; the Puppeteer e2e smoke is the
   project's sanctioned UI verification boundary (it already drives the
   token_bazaar board, a11y, and no-leak checks), so the assertion belongs
   there rather than in a new harness.

## Outcome

Completed: 2026-06-08

What changed:

- `apps/web/src/components/TokenBazaarBoard.tsx` now parses action metadata
  resource counts with the Rust-emitted `:` delimiter.
- `apps/web/e2e/token-bazaar.smoke.mjs` now asserts the collect-amber action
  gain renders amber `2`, jade `0`, and iron `0` before taking the action.

Deviations from original plan:

- The assertion preserves the existing resource code labels (`AM`, `JA`, `IR`)
  instead of assuming one-letter codes.

Verification results:

- `npm --prefix apps/web run build` passed.
- `npm --prefix apps/web run smoke:e2e` passed.

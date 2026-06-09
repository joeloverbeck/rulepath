# PTRAT-001: Conform plain_tricks outcome-rationale projection to the top-level `terminal_rationale` convention

**Status**: ✅ COMPLETED
**Priority**: MEDIUM
**Effort**: Medium
**Engine Changes**: Yes — `crates/wasm-api` (plain_tricks public-view JSON projection + unit tests), `apps/web` presentation (`src/wasm/client.ts`, `src/components/PlainTricksBoard.tsx`). No `engine-core` / `game-stdlib` / `games/*` behavior change; no golden trace or hash change.
**Deps**: none

## Problem

`plain_tricks` is the only one of ten games whose terminal outcome rationale is projected **nested under `terminal`** instead of as a top-level `terminal_rationale` field on the public view. All nine prior games hoist the rationale to a top-level `terminal_rationale` JSON key in the WASM bridge (`crates/wasm-api/src/lib.rs:5405,5425` and helper `poker_terminal_rationale` at `:5682` for poker_lite), expose it as `terminal_rationale?` on their `client.ts` public-view type, and read `view.terminal_rationale` in their renderers. `plain_tricks` instead nests it end-to-end: `plain_terminal_json` (`crates/wasm-api/src/lib.rs:5517`, called at `:5457`) emits the rationale inside the `terminal` object, `client.ts` carries `rationale` inside the `PlainTricksTerminalView` variants (`apps/web/src/wasm/client.ts:697-698`), and `PlainTricksBoard.tsx:162` reads `view.terminal.rationale`.

This deviation trips the Gate 1 convention guard `scripts/check-outcome-explanations.mjs` (`checkClientMirror`, `:149-157`), which requires a `terminal_rationale`/`outcome_rationale` field on the `<Game>PublicView` type. The remaining Gate 1 failure after PTRAT's sibling inline fixes is exactly:

```
outcome-explanations check failed:
  - plain_tricks: apps/web/src/wasm/client.ts lacks an outcome rationale type or field mirror
```

Conform plain_tricks to the established projection so the convention holds uniformly and the renderer/tooling no longer special-cases one game.

## Assumption Reassessment (2026-06-10)

<!-- Items 1-3 always required. Items 4+ included only where in scope. -->

1. **Current plain_tricks projection (verified against code).** Rust nests the rationale in the terminal view: `games/plain_tricks/src/visibility.rs` defines `OutcomeRationaleView` (`:89`) and carries `rationale: OutcomeRationaleView` inside the `TrickWin`/`Split` `TerminalView` variants (`:79,:84`). The WASM bridge nests it in JSON via `plain_terminal_json` (`crates/wasm-api/src/lib.rs:5517`, invoked at `:5457` inside the plain_tricks public-view template). `client.ts` mirrors the nested shape (`apps/web/src/wasm/client.ts:697-698`); `PlainTricksBoard.tsx:162` consumes `view.terminal.kind === "non_terminal" ? null : view.terminal.rationale`.
2. **Canonical convention (verified against code and docs).** poker_lite — same Rust shape (rationale nested in `TerminalView`, `games/poker_lite/src/visibility.rs:74,79,84`) — hoists to a top-level `"terminal_rationale"` JSON key in the bridge (`crates/wasm-api/src/lib.rs:5405` template, populated by `poker_terminal_rationale(&view.terminal)` at `:5425`, helper at `:5682`), exposes `terminal_rationale?: PokerLiteOutcomeRationale | null` (`apps/web/src/wasm/client.ts:666`), and the renderer reads `view.terminal_rationale` (`apps/web/src/components/PokerLiteBoard.tsx:132`). The same top-level pattern holds for race_to_n, three_marks, column_four, directional_flip, draughts_lite, high_card_duel, token_bazaar, secret_draft.
3. **Shared boundary under audit.** The WASM public-view JSON contract (`crates/wasm-api/src/lib.rs`) and its TypeScript mirror (`apps/web/src/wasm/client.ts`). This is a presentation/serialization-projection change to the browser payload only.
4. **FOUNDATIONS principle restated (no-leak + determinism, §11).** The rationale is already a viewer-safe projection (redaction-tested; see `plain_tricks_public_export_omits_seed_tail_and_unplayed_cards`, `crates/wasm-api/src/lib.rs:8288`, and `plain_tricks_surface_filters_hidden_cards_and_authorizes_actor`, `:8212`). Hoisting changes only the **JSON key placement**, not the rationale content, so it introduces no new field and cannot leak hidden information.
5. **Schema/consumer audit (public-view contract).** Consumers of the plain_tricks public-view JSON are: `client.ts` (type), `PlainTricksBoard.tsx` (renderer), and `apps/web/e2e/plain-tricks.smoke.mjs` (e2e — verified to make **no** rationale-shape assertion, so it needs no change). The change is a key relocation within the already-projected data, not a new behavioral field.
6. **Determinism / hash blast radius (critical).** Public-view hashes in the 14 plain_tricks golden traces derive from Rust `StableSerialize::stable_summary()` (`games/plain_tricks/src/visibility.rs:166,200-203`), which is **independent** of the WASM JSON projection. This ticket does **not** touch `games/plain_tricks/src/visibility.rs` or `stable_summary()`, so no golden trace, replay hash, fixture hash, or `plain_tricks_wasm_export_matches_golden_fixture` (`crates/wasm-api/src/lib.rs:8654`) regeneration is required. **If implementation finds it must alter `stable_summary()`, stop and re-scope** — that would convert this into a hash-migration ticket.
7. **Adjacent contradiction classification.** None uncovered. The two sibling Gate 1 failures (player-rules `HIDDEN_INFO_GAMES` allowlist; RULES.md `## Scoring and accounting` heading) are already fixed inline and are out of scope here.

## Architecture Check

1. **Cleaner than the alternative.** The rejected alternative is to broaden `check-outcome-explanations.mjs` to also accept the nested shape; that permanently sanctions two divergent projections for one concept and forces every future renderer/tool to special-case which shape a game uses. Conforming to the single top-level convention keeps the public-view contract uniform across all ten games and keeps the guard strict (no test weakened — `docs/AGENT-DISCIPLINE.md §4`).
2. **No backwards-compatibility shims.** The nested `rationale` is removed from the plain_tricks `terminal` projection and TS terminal variants — no alias path, no dual-shape support.
3. **`engine-core` / `game-stdlib` untouched.** This is a WASM-bridge presentation projection plus a TS mirror; no mechanic nouns enter `engine-core`, and `game-stdlib` is not modified. `games/plain_tricks/src` is intentionally left unchanged.

## Verification Layers

1. Top-level `terminal_rationale` present in the plain_tricks WASM public-view JSON (and `null` when non-terminal) -> schema/serialization validation (new `crates/wasm-api` unit test mirroring `poker_lite_view_projects_terminal_rationale_template_keys`, `:8335`).
2. Rationale still leaks no hidden information after relocation -> no-leak visibility test (extend `plain_tricks_public_export_omits_seed_tail_and_unplayed_cards` / add a yield-style no-reveal assertion mirroring `poker_lite_yield_terminal_rationale_does_not_reveal_private_strength`, `:8365`).
3. TS mirror matches the JSON contract (`terminal_rationale?` on `PlainTricksPublicView`; no `rationale` on `PlainTricksTerminalView`) -> codebase grep-proof + `npm --prefix apps/web run build` (tsc).
4. Renderer reads the conformed field and the terminal explanation still renders -> web UI smoke (`apps/web/e2e/plain-tricks.smoke.mjs`).
5. Deterministic replay/hash unchanged -> golden trace / deterministic replay-hash check (`cargo run -p replay-check -- --game plain_tricks --all`) proves `stable_summary()` was not disturbed.
6. Convention guard passes -> `node scripts/check-outcome-explanations.mjs`.

## What to Change

### 1. WASM bridge — hoist the rationale (`crates/wasm-api/src/lib.rs`)

- Add a `plain_terminal_rationale(&view.terminal) -> String` helper mirroring `poker_terminal_rationale` (`:5682`): emits the `OutcomeRationaleView` JSON for terminal states and `null` for `NonTerminal`.
- In the plain_tricks public-view JSON template (around `:5457`), add a top-level `"terminal_rationale":{}` field populated by the new helper.
- Update `plain_terminal_json` (`:5517`) so the `terminal` object no longer nests `rationale` (match poker_lite's `PokerLiteTerminalView` JSON, which omits it).
- Add/extend unit tests mirroring `poker_lite_view_projects_terminal_rationale_template_keys` (`:8335`) and the yield no-reveal test (`:8365`) for plain_tricks `trick_win` and `split` terminals, asserting `"terminal_rationale":null` for non-terminal.

### 2. TypeScript mirror (`apps/web/src/wasm/client.ts`)

- Remove `rationale: PlainTricksOutcomeRationale` from the `PlainTricksTerminalView` `trick_win` and `split` variants (`:697-698`).
- Add `terminal_rationale?: PlainTricksOutcomeRationale | null;` to `PlainTricksPublicView` (alongside `terminal:`), matching `PokerLitePublicView` (`:666`).

### 3. Renderer (`apps/web/src/components/PlainTricksBoard.tsx`)

- Change the rationale read at `:162` from `view.terminal.kind === "non_terminal" ? null : view.terminal.rationale` to `view.terminal_rationale ?? null`, matching `PokerLiteBoard.tsx:132`.

## Files to Touch

- `crates/wasm-api/src/lib.rs` (modify)
- `apps/web/src/wasm/client.ts` (modify)
- `apps/web/src/components/PlainTricksBoard.tsx` (modify)

## Out of Scope

- Any change to `games/plain_tricks/src/**` (Rust rules, `visibility.rs`, `stable_summary()`) — would churn golden-trace hashes.
- Golden trace / fixture regeneration.
- The two already-fixed sibling Gate 1 failures (`scripts/check-player-rules.mjs` allowlist; `games/plain_tricks/docs/RULES.md` heading).
- Broadening `scripts/check-outcome-explanations.mjs` (the explicitly rejected alternative).

## Acceptance Criteria

### Tests That Must Pass

1. `node scripts/check-outcome-explanations.mjs` — passes for all 10 catalog games.
2. `cargo test -p wasm-api` — including the new plain_tricks `terminal_rationale` projection and no-leak assertions.
3. `cargo run -p replay-check -- --game plain_tricks --all` — public-view/replay hashes unchanged (proves `stable_summary()` untouched).
4. `npm --prefix apps/web run build` — tsc accepts the conformed `client.ts` types.
5. `node apps/web/e2e/plain-tricks.smoke.mjs` — terminal explanation still renders.

### Invariants

1. plain_tricks projects its outcome rationale identically to the other nine games: top-level `terminal_rationale` on the public view, `null` when non-terminal, absent from the nested `terminal` object.
2. No hidden information (seed tail, unplayed cards, opponent private hand) reaches the rationale or any public payload.
3. plain_tricks deterministic public-view/replay hashes are byte-identical to pre-change (no golden trace regeneration).

## Test Plan

### New/Modified Tests

1. `crates/wasm-api/src/lib.rs` (tests module) — add plain_tricks projection test mirroring `poker_lite_view_projects_terminal_rationale_template_keys`: asserts `"terminal_rationale":null` for non-terminal and `"terminal_rationale":{...template_key...}` for `trick_win`/`split`.
2. `crates/wasm-api/src/lib.rs` (tests module) — add/extend a plain_tricks no-leak assertion confirming the relocated rationale exposes no private hand or tail data.

### Commands

1. `node scripts/check-outcome-explanations.mjs`
2. `cargo test -p wasm-api`
3. `cargo run -p replay-check -- --game plain_tricks --all`
4. `npm --prefix apps/web run build && node apps/web/e2e/plain-tricks.smoke.mjs`

## Outcome

Completed: 2026-06-10

What changed:

- `crates/wasm-api/src/lib.rs` now projects `plain_tricks` outcome rationale as top-level `terminal_rationale`, with `null` for non-terminal views.
- `plain_terminal_json` no longer nests `rationale` inside `terminal`.
- Added `plain_tricks` bridge tests covering non-terminal, trick-win, split, and no-leak rationale projection.
- `apps/web/src/wasm/client.ts` mirrors the top-level `terminal_rationale` contract, and `PlainTricksBoard.tsx` reads `view.terminal_rationale`.

Deviations from plan:

- The projection test asserts the schema location and implemented rationale fields without pinning every fixture-specific terminal total.
- No `games/plain_tricks/src/**` files, golden traces, replay hashes, `engine-core`, or `game-stdlib` changed.

Verification:

- `cargo fmt --all --check`
- `node scripts/check-outcome-explanations.mjs`
- `cargo test -p wasm-api`
- `cargo run -p replay-check -- --game plain_tricks --all`
- `npm --prefix apps/web run build`
- `node apps/web/e2e/plain-tricks.smoke.mjs`

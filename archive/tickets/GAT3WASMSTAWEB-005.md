# GAT3WASMSTAWEB-005: App shell, game picker, and match setup regions

**Status**: COMPLETED
**Priority**: HIGH
**Effort**: Medium
**Engine Changes**: None — TypeScript/presentation only (`apps/web`); consumes the `list_games` WASM op from GAT3WASMSTAWEB-002.
**Deps**: 001, 002, 004

## Problem

The current page is a single harness-like `App` (start button, scoreboard,
effects). Gate 3 requires a public-presentable app shell with a landmarked header,
a game picker showing `race_to_n` as the one selectable game, and an explicit match
setup region (spec §7.1, §7.2, §7.3; §4.1). The picker MUST use Rust/WASM-supplied
game metadata, not hardcoded React behavior authority (§7.2, FOUNDATIONS §2). This
ticket builds the shell skeleton the renderer, modes, replay, and dev panel mount
into.

## Assumption Reassessment (2026-06-06)

1. `apps/web/src/main.tsx` currently renders one `<main className="shell">` with a
   topbar, play-surface, and effects section (lines ~355–443); there is no picker
   or setup region, and the game id `"race_to_n"` is hardcoded in `start`
   (`api.newMatch("race_to_n", 1)`, line ~288). After GAT3WASMSTAWEB-004 the shell
   reads reducer state; this ticket adds the `setup`-mode regions that precede `play`.
2. Spec §7.2 requires the picker to show `race_to_n` as the only active game using
   Rust-supplied metadata and to avoid presenting future games as selectable; §7.3
   requires setup to identify game/variant, choose a Gate-3 mode, show seat roles,
   provide a clear start action, and avoid raw-JSON output. §4.1 keeps it a "clean
   even with one game" picker.
3. Cross-artifact boundary under audit: the picker consumes `list_games` (Rust op,
   GAT3WASMSTAWEB-002) through a new typed wrapper added to
   `apps/web/src/wasm/client.ts` (module created by GAT3WASMSTAWEB-001). Setup mode
   inputs feed the reducer (`apps/web/src/state/shellReducer.ts`, GAT3WASMSTAWEB-004)
   and ultimately `new_match`. No game identity is hardcoded in React.
4. FOUNDATIONS §2/§7 (behavior authority; play-first UI): the catalog/metadata are
   Rust-owned; React presents them. Restated: the picker MUST render `list_games`
   output and MUST NOT synthesize the catalog or invent variants.

## Architecture Check

1. Durable region components (`AppShell`, `GamePicker`, `MatchSetup`) under
   `apps/web/src/components/` give the spec's §10.1 region separation and a mount
   point for later tickets, versus extending the monolithic `App`. The picker
   sourcing from `list_games` keeps one Rust catalog authority.
2. No backwards-compatibility shims: the hardcoded `"race_to_n"`/`seed 1` start path
   is replaced by setup-driven match creation; no parallel start path remains.
3. `engine-core` untouched; no mechanic noun in React; `game-stdlib` untouched.

## Verification Layers

1. Picker uses Rust metadata → codebase grep-proof: no game-identity/display-name
   literal for the catalog in `apps/web/src/components`; the picker reads
   `client.listGames()`.
2. Setup produces a valid match → simulation/CLI run: `npm run smoke:ui` can select
   the game, choose a mode, and start a match (no raw-JSON setup output).
3. Regions are landmarked/accessible → manual review: header landmark, headings for
   picker/setup, labeled form controls (full a11y audit in GAT3WASMSTAWEB-014).
4. Catalog authority in Rust → FOUNDATIONS §2 alignment check.

## What to Change

### 1. New region components under `apps/web/src/components/`

`AppShell.tsx` (header/landmark + play/dev layout slots), `GamePicker.tsx` (lists
`race_to_n` from `list_games`, marks it the active selectable game, no future-game
placeholders required), `MatchSetup.tsx` (game/variant identity, mode chooser among
Gate-3 modes, seat-role display, deterministic default seed, clear "Start match").

### 2. `apps/web/src/wasm/client.ts`

Add a typed `listGames()` method wrapping the `rulepath_list_games` export (op from
GAT3WASMSTAWEB-002) returning a typed game-catalog result.

### 3. `apps/web/src/state/shellReducer.ts` + `apps/web/src/main.tsx`

Add setup-input/selected-game state + transitions (`gameSelected`→setup,
`matchStarted`→play) and compose `AppShell`/`GamePicker`/`MatchSetup` for the
pre-match `setup` mode.

## Files to Touch

- `apps/web/src/components/AppShell.tsx` (new)
- `apps/web/src/components/GamePicker.tsx` (new)
- `apps/web/src/components/MatchSetup.tsx` (new)
- `apps/web/src/wasm/client.ts` (modify) — add `listGames()` wrapper
- `apps/web/src/state/shellReducer.ts` (modify) — setup/selected-game state + transitions
- `apps/web/src/main.tsx` (modify) — compose regions for setup mode

## Out of Scope

- Active-play board renderer + legal-action controls — GAT3WASMSTAWEB-006.
- The play modes themselves (human-vs-bot/hotseat/bot-vs-bot) — GAT3WASMSTAWEB-008.
- Full accessibility/responsive audit — GAT3WASMSTAWEB-014.
- Showcase styling/branding (baseline only per §4.3).

## Acceptance Criteria

### Tests That Must Pass

1. `cd apps/web && npm run build` — typecheck + Vite build succeed.
2. `cd apps/web && npm run smoke:ui` — smoke can reach the picker, select `race_to_n`, complete setup, and start a match.
3. `grep -rnE "\"race_to_n\"|Race to 21" apps/web/src/components` — the catalog identity/display name is not hardcoded in the picker (it comes from `list_games`); any match is a Rust-fed render, not a synthesized catalog.

### Invariants

1. The game catalog and variant metadata are Rust-supplied; React renders them.
2. Match creation flows through explicit setup state, not a hardcoded start path; setup shows no raw JSON.

## Test Plan

### New/Modified Tests

1. `None — UI region ticket; verification is `smoke:ui` (extended click path) + `tsc`; rendered-browser assertions land in GAT3WASMSTAWEB-013.`

### Commands

1. `cd apps/web && npm run build`
2. `cd apps/web && npm run smoke:ui`
3. Rendered DOM assertions are deferred to the Puppeteer harness (GAT3WASMSTAWEB-013); here the node smoke + typecheck are the correct boundary for region wiring.

## Outcome

Completed: 2026-06-06

What changed:

- Added `RulepathApi.listGames()` and a typed `GameCatalogEntry` wrapper for the Rust `rulepath_list_games` op.
- Added `AppShell`, `GamePicker`, and `MatchSetup` region components.
- Updated the reducer to store Rust catalog entries, select the first loaded game, and track setup seed/play mode.
- Updated `main.tsx` to load the catalog during WASM bootstrap, render picker/setup before a match, and start matches from setup-selected game state.
- Added baseline styles for the shell regions, picker, setup fields, mode chooser, and seat-role display.

Deviations from original plan:

- Rendered click-path assertions remain deferred to the later browser smoke ticket as planned; the existing `smoke:ui` command is still a Node/WASM smoke in this repo.

Verification results:

- `npm --prefix apps/web run build` passed.
- `npm --prefix apps/web run smoke:ui` passed with version `rulepath-wasm-api/0.1.0`, match `race_to_n-1`, counter `2`, `8` effects, and stale-action diagnostic coverage.
- `grep -rnE '"race_to_n"|Race to 21' apps/web/src/components` returned no matches.

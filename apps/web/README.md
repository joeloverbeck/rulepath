# Rulepath Web Shell

`apps/web` is the static React shell for Rulepath's local browser games:
`race_to_n` / Race to 21, `three_marks` / Three Marks, `column_four` / Column
Four, `directional_flip` / Directional Flip, `draughts_lite` / Draughts Lite,
`high_card_duel` / High Card Duel, `token_bazaar` / Token Bazaar, and
`secret_draft` / Veiled Draft, `poker_lite` / Crest Ledger, and
`plain_tricks` / Plain Tricks, `masked_claims` / Masked Claims, and
`flood_watch` / Flood Watch, `frontier_control` / Frontier Control, and
`event_frontier` / Event Frontier.
Rust/WASM owns game behavior; TypeScript presents Rust-provided catalog entries,
views, action trees, effects, diagnostics, bot turns, and replay projections.

## Commands

Run from the repository root:

```bash
npm --prefix apps/web install
npm --prefix apps/web run build:wasm
npm --prefix apps/web run build
npm --prefix apps/web run preview
npm --prefix apps/web run smoke:wasm
npm --prefix apps/web run smoke:ui
npm --prefix apps/web run smoke:effects
npm --prefix apps/web run smoke:preview
npm --prefix apps/web run smoke:e2e
```

`build:wasm` compiles `crates/wasm-api` for `wasm32-unknown-unknown` and copies the
artifact to `apps/web/public/wasm_api.wasm`. `build` typechecks and emits the Vite
`dist` output. `preview` serves the built app with Vite.

`smoke:preview` and `smoke:e2e` serve built `dist` from a nested `/rulepath/` mount,
proving the shell does not require a backend or root-only asset paths.

The Puppeteer E2E smoke uses system Chrome by default at `/usr/bin/google-chrome`.
Set `PUPPETEER_EXECUTABLE_PATH` to use a different Chrome/Chromium binary.

## Static Serving

Vite uses `base: "./"`, and the WASM client resolves `wasm_api.wasm` from the Vite
base URL. A static server must serve these files from the same directory:

- `index.html`
- `assets/*`
- `wasm_api.wasm`

No backend route is required. Browser state is local shell state only;
authoritative match state lives inside the Rust/WASM in-memory store.

## Shell Surface

The shell includes:

- Rust catalog-driven game picker;
- seeded match setup;
- human-vs-bot, hotseat, and bot-vs-bot modes;
- Race to 21 public board and status;
- first-class board renderers for Three Marks, Column Four, Directional Flip,
  Draughts Lite, High Card Duel, Token Bazaar, Veiled Draft, Crest Ledger, and
  Plain Tricks, Masked Claims, Flood Watch, Frontier Control, and Event
  Frontier;
- shared `DeckFlowPanel` deck/pile presentation for Rust-projected card flows;
- shared `ActionPathBuilder` staged construction for nested Rust action trees;
- Rust action-tree-driven buttons;
- semantic effect log with reduced-motion support;
- local replay export/import/reset/step;
- secondary developer panel with viewer-safe diagnostics and counters.

Replay import is capped at 128 KiB in the UI before Rust parsing. The developer
panel data whitelist is documented in
[`../../docs/WASM-CLIENT-BOUNDARY.md`](../../docs/WASM-CLIENT-BOUNDARY.md).

### Action Presentation Audit

Every catalog game has an explicit action-presentation disposition. `adopt`
means the board uses `ActionPathBuilder`; `board-native` means the board maps
domain controls directly to Rust-supplied choices; `fallback` means the generic
single-stage `ActionControls` surface is sufficient.

| Game | Disposition | Rationale |
| --- | --- | --- |
| `race_to_n` | fallback | Single-stage add choices render through `ActionControls`; no compound tree. |
| `three_marks` | board-native | Board cells/buttons map one-to-one to Rust mark/drop choices. |
| `column_four` | board-native | Column controls map one-to-one to Rust column choices. |
| `directional_flip` | board-native | Board cells expose Rust legal targets directly. |
| `draughts_lite` | board-native | Board-native pending-path flow walks Rust move choices with piece/cell controls. |
| `high_card_duel` | board-native | Hand card controls map to Rust commit choices. |
| `masked_claims` | board-native | Claim and response controls are derived from Rust choice groups without flattening compound paths. |
| `flood_watch` | board-native | District and turn controls map directly to Rust bail/reinforce/forecast/end-turn choices. |
| `frontier_control` | board-native | Grouped action controls render Rust choices directly; no nested action tree is flattened. |
| `event_frontier` | adopt | Compound Event Frontier operation trees use `ActionPathBuilder` for staged selection and leaf confirmation. |
| `token_bazaar` | board-native | Market/action grid maps to Rust legal actions and public slot metadata. |
| `secret_draft` | board-native | Pool-item controls map to Rust draft/reveal choices. |
| `poker_lite` | board-native | Poker action buttons map directly to Rust hold/press/lift/match/yield choices. |
| `plain_tricks` | board-native | Hand-card buttons map to Rust play-card choices. |

## Smoke Layers

- `smoke:wasm`: raw ABI coverage for version/features, catalog, match, action,
  bot, effects, stale diagnostics, replay, and all registered games.
- `smoke:ui`: fast Node/WASM shell-state smoke through `render_game_to_text`.
- `smoke:effects`: Node/WASM effect-feedback projection guard for every catalog game.
- `smoke:preview`: built `dist` static-serving and WASM fetch smoke.
- `smoke:e2e`: Puppeteer rendered-browser smoke plus accessibility/no-leak smoke
  for the shell, rules display, outcome explanation, Three Marks, Column Four,
  Draughts Lite, High Card Duel, Token Bazaar, Veiled Draft, Crest Ledger, and
  Plain Tricks, Masked Claims, Flood Watch, Frontier Control, and Event Frontier.
  A standalone Directional Flip E2E smoke file also exists under
  `e2e/`, but is not chained by `smoke:e2e`.

# Rulepath Web Shell

`apps/web` is the static React shell for Rulepath's local browser games:
`race_to_n` / Race to 21, `three_marks` / Three Marks, `column_four` / Column
Four, `directional_flip` / Directional Flip, `draughts_lite` / Draughts Lite,
`high_card_duel` / High Card Duel, and `token_bazaar` / Token Bazaar.
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
  Draughts Lite, High Card Duel, and Token Bazaar;
- Rust action-tree-driven buttons;
- semantic effect log with reduced-motion support;
- local replay export/import/reset/step;
- secondary developer panel with viewer-safe diagnostics and counters.

Replay import is capped at 128 KiB in the UI before Rust parsing. The developer
panel data whitelist is documented in
[`../../docs/WASM-CLIENT-BOUNDARY.md`](../../docs/WASM-CLIENT-BOUNDARY.md).

## Smoke Layers

- `smoke:wasm`: raw ABI coverage for version/features, catalog, match, action,
  bot, effects, stale diagnostics, replay, and all registered games.
- `smoke:ui`: fast Node/WASM shell-state smoke through `render_game_to_text`.
- `smoke:preview`: built `dist` static-serving and WASM fetch smoke.
- `smoke:e2e`: Puppeteer rendered-browser smoke plus accessibility/no-leak smoke
  for the shell, Three Marks, Column Four, Draughts Lite, High Card Duel, and
  Token Bazaar. A standalone Directional Flip E2E smoke file also exists under
  `e2e/`, but is not chained by `smoke:e2e`.

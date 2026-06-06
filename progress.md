Original prompt: Implement the GAT1RACTON tickets one at a time, archiving and committing each ticket before moving on.

## GAT1RACTON-012

- Using the web-game development loop because this ticket adds the browser harness.
- The previous `wasm-api` Rust-callable surface is not directly callable from raw WebAssembly JS; the web harness needs a small JSON bridge export.
- Added raw wasm JSON bridge exports in `crates/wasm-api` for the six batched operations.
- Added the React `race_to_n` harness, dependency-free UI smoke script, and `games/race_to_n/docs/UI.md`.
- Browser proof loaded `http://127.0.0.1:5173/`, clicked Start Match, `add-1`, and Submit Stale; `render_game_to_text` showed counter 2, eight effects, and `stale_action`. Desktop and mobile screenshots looked coherent; console had no messages.

## Gate 3 WASM/static web shell

- Completed on 2026-06-06 for `race_to_n` / Race to 21 only.
- Added the typed TypeScript WASM client, Rust feature/catalog/replay operations, reducer-backed React shell, game picker, match setup, Race to 21 board, Rust action controls, effect log, play modes, replay UI, developer panel, base-aware static WASM loading, and browser E2E smoke coverage.
- Verification evidence:
  - `npm --prefix apps/web run smoke:wasm`
  - `npm --prefix apps/web run smoke:preview`
  - `npm --prefix apps/web run smoke:e2e`
  - `npm --prefix apps/web run build`
- Boundary notes: TypeScript remains presentation-only; legal actions, validation, effects, bots, replay projection, diagnostics, and public views come from Rust/WASM.

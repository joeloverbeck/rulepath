Original prompt: Implement the GAT1RACTON tickets one at a time, archiving and committing each ticket before moving on.

## GAT1RACTON-012

- Using the web-game development loop because this ticket adds the browser harness.
- The previous `wasm-api` Rust-callable surface is not directly callable from raw WebAssembly JS; the web harness needs a small JSON bridge export.
- Added raw wasm JSON bridge exports in `crates/wasm-api` for the six batched operations.
- Added the React `race_to_n` harness, dependency-free UI smoke script, and `games/race_to_n/docs/UI.md`.
- Browser proof loaded `http://127.0.0.1:5173/`, clicked Start Match, `add-1`, and Submit Stale; `render_game_to_text` showed counter 2, eight effects, and `stale_action`. Desktop and mobile screenshots looked coherent; console had no messages.

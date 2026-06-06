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

## Gate 5 Column Four public polish

- Completed on 2026-06-06 for `column_four` / Column Four.
- Added a full official game crate with local typed grid/column/gravity/line rules, public view projection, semantic effects, replay support, golden traces, fixtures, Level 0 and Level 2 bots, native benchmarks, WASM registration, CLI tool registration, and a first-class React/SVG board.
- Added the browser proof surface: seven Rust-legal column controls, Rust landing previews, effect-log-driven landed-piece animation, terminal win/draw display, public bot rationale, replay projection, keyboard path, reduced-motion handling, and DOM/storage/console/replay no-leak checks.
- Updated CI gates to run Column Four simulation, replay drift, fixture validation, rule coverage, WASM smoke, browser E2E, and benchmark lanes.
- Acceptance evidence:
  - `cargo test --workspace`
  - `cargo run -p simulate -- --game column_four --games 1000`
  - `cargo run -p replay-check -- --game column_four --all`
  - `cargo run -p fixture-check -- --game column_four`
  - `cargo run -p rule-coverage -- --game column_four`
  - `npm --prefix apps/web run smoke:wasm`
  - `npm --prefix apps/web run smoke:e2e`
  - `bash scripts/boundary-check.sh`
  - `node scripts/check-doc-links.mjs`
- ROADMAP Gate 5 exit mapping:
  - public page feels polished: `ColumnFourBoard` plus `column-four.smoke.mjs`
  - legal columns only are clickable: Rust legal targets, full-column inertness smoke, fixture/replay coverage
  - previews are Rust-safe: hover/focus preview from Rust `landing_preview`
  - animations come from semantic effects: landed-piece class from Rust `piece_landed`; reduced-motion smoke
  - bot explanations are available: Level 2 public rationale in bot effects and browser smoke
  - replay viewer smoke passes: export/import/step renders `ColumnFourBoard`
  - benchmark and UI smoke coverage exists: `cargo bench -p column_four` plus `smoke:e2e`
  - mechanic atlas records repeated coordinate/line pressure: `docs/MECHANIC-ATLAS.md`
- Boundary notes: fixed-grid, coordinate/targeted placement, line detection, terminal-line highlighting, column actions, and gravity are recorded as local or repeated-shape pressure only. No `engine-core` or `game-stdlib` extraction occurred.

# Rulepath Web Shell

`apps/web` is the static React shell for Rulepath's local browser games:
`race_to_n` / Race to 21, `three_marks` / Three Marks, `column_four` / Column
Four, `directional_flip` / Directional Flip, `draughts_lite` / Draughts Lite,
`high_card_duel` / High Card Duel, `token_bazaar` / Token Bazaar, and
`secret_draft` / Veiled Draft, `poker_lite` / Crest Ledger, and
`plain_tricks` / Plain Tricks, `masked_claims` / Masked Claims, and
`flood_watch` / Flood Watch, `frontier_control` / Frontier Control,
`event_frontier` / Event Frontier, and `river_ledger` / River Ledger.
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
- catalog game icons, including a dedicated original River Ledger icon;
- seeded match setup;
- human-vs-bot, hotseat, and bot-vs-bot modes;
- Race to 21 public board and status;
- first-class board renderers for Three Marks, Column Four, Directional Flip,
  Draughts Lite, High Card Duel, Token Bazaar, Veiled Draft, Crest Ledger, and
  Plain Tricks, Masked Claims, Flood Watch, Frontier Control, Event Frontier,
  and River Ledger;
- shared `DeckFlowPanel` deck/pile presentation for Rust-projected card flows;
- shared `SeatFrame` for catalog-projected seat labels, active/pending seat
  rail, observer mode, and viewer selection;
- shared `ActionPathBuilder` staged construction for nested Rust action trees;
- shared action-affordance rendering for Rust-emitted cost/consequence metadata
  and confirmation summaries;
- shared effect-animation scheduler, burst grouping, registry, and dev settle
  assertion for viewer-filtered semantic effects;
- scheduler-owned turn orchestration for auto-advancing bot turns, automated
  phases, skip/pause, replay-step interruption, and reduced-motion pacing;
- `TurnReportPanel` narration of viewer-filtered bot turns and automated
  advances near the board;
- typed setup variant selector driven by Rust/WASM catalog variant labels;
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
| `river_ledger` | board-native | Seat, board, ledger, and action controls render Rust legal choices and viewer-safe fields. |

### Effect Animation Adoption Audit

Every catalog game has an explicit effect-animation disposition. `adopt` means
the game registers authored effect-to-animation mappings on the shared registry.
`generic-only` means the game intentionally relies on the shared tone-keyed
presentations for the current catalog surface.

| Game | Disposition | Rationale |
| --- | --- | --- |
| `race_to_n` | generic-only | Tiny counter effects are covered by shared highlight/turn/terminal presentations. |
| `three_marks` | generic-only | Board mark/drop effects remain legible through shared board highlighting and text. |
| `column_four` | generic-only | Column drops and terminal effects use baseline shared motion without per-game mapping. |
| `directional_flip` | generic-only | Directional flip effects are simple public board updates covered by generic highlighting. |
| `draughts_lite` | generic-only | Move/capture effects are viewer-safe and covered by generic movement/highlight motion. |
| `high_card_duel` | generic-only | Reveal/score/terminal effects stay readable through shared generic presentations. |
| `masked_claims` | generic-only | Redacted and reveal effects use the shared viewer-safe redacted/reveal baseline. |
| `flood_watch` | adopt | Flood phases, storm deck flow, and district automation use authored registry mappings. |
| `frontier_control` | generic-only | Public graph/control effects use baseline highlighting without authored overrides. |
| `event_frontier` | adopt | Event deck flow, resources, Reckoning, site changes, and terminal settlement use authored mappings. |
| `token_bazaar` | generic-only | Market/resource/contract effects use baseline shared movement/highlight presentations. |
| `secret_draft` | generic-only | Draft/reveal effects use shared redacted/reveal-safe presentations. |
| `poker_lite` | generic-only | Public poker-lite score/reveal effects use baseline shared presentations. |
| `plain_tricks` | generic-only | Deal/play/trick/score effects use baseline shared movement/highlight presentations. |
| `river_ledger` | adopt | River Ledger contribution, street-advance, board reveal, and showdown-settle feedback use authored registry mappings with reduced-motion coverage. |

## Smoke Layers

- `smoke:wasm`: raw ABI coverage for version/features, catalog, match, action,
  bot, effects, stale diagnostics, replay, and all registered games.
- `smoke:ui`: fast Node/WASM shell-state smoke through `render_game_to_text`.
- `smoke:effects`: Node/WASM effect-feedback projection guard for every catalog game.
- `smoke:animation`: Node checks for burst segmentation, scheduler behavior,
  presenter/registry behavior, and the catalog animation adoption sweep.
- `smoke:preview`: built `dist` static-serving and WASM fetch smoke.
- `smoke:e2e`: Puppeteer rendered-browser smoke plus accessibility/no-leak smoke
  for the shell, rules display, outcome explanation, Three Marks, Column Four,
  Draughts Lite, High Card Duel, Token Bazaar, Veiled Draft, Crest Ledger, and
  Plain Tricks, Masked Claims, Flood Watch, Frontier Control, Event Frontier,
  and River Ledger.
  The chain also runs `e2e/animation.smoke.mjs` for animate-and-settle, skip,
  replay-step interruption, and reduced-motion animation behavior.
  The accessibility/no-leak layer includes a runtime raw-identifier DOM guard
  over normal-mode visible text and accessibility labels, with induced-drift
  negative coverage in `e2e/a11y-noleak.smoke.mjs`; it also exercises the
  shared `SeatFrame` viewer selector on High Card Duel and checks DOM,
  attributes, test IDs, storage, and console output for private-token leaks.
  A standalone Directional Flip E2E smoke file also exists under
  `e2e/`, but is not chained by `smoke:e2e`.

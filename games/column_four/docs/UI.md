# Column Four UI Notes

Game ID: `column_four`

Implemented variant: `column_four_standard`

Rules version: `column_four-rules-v1`

Renderer assumptions version: `column-four-ui-v1`

Last updated: 2026-06-06

## Purpose

This document records the shipped web UI contract for Column Four. Rust/WASM owns legal actions, validation, public view projection, safe landing previews, semantic effects, replay projection, diagnostics, and bot decisions. TypeScript renders those viewer-safe payloads.

## Product And Visual Target

| Field | Decision |
|---|---|
| public role | Gate 5 public showcase |
| desired feel | polished neutral board game, compact and play-first |
| visual risk to avoid | proprietary mimicry, red/yellow or blue-rack trade dress, debug-console-first UI |
| public onboarding need | light; seven column controls and status text are the primary instructions |
| help/learning mode need | none in Gate 5 |

## Renderer Assumptions

| Assumption | Value | Evidence/notes |
|---|---|---|
| default renderer | React + SVG | Implemented in [../../../apps/web/src/components/ColumnFourBoard.tsx](../../../apps/web/src/components/ColumnFourBoard.tsx). |
| expected object count | 42 cells, 7 controls, small status panels | SVG remains low pressure. |
| animation pressure | medium | Landed piece animation is effect-driven and reduced-motion aware. |
| Canvas/PixiJS needed? | no | No profiling evidence or ADR supports another renderer. |
| WASM boundary | batched public view/action/effect/replay calls | `smoke:wasm` and `smoke:e2e` cover the browser path. |

## Legal Action Mapping

| Rust action choice/tree node | Rule IDs | UI control | Normal behavior | Accessibility label | Notes |
|---|---|---|---|---|---|
| `drop/c1` through `drop/c7` | `CF-ACTION-001`, `CF-GRAVITY-001`, `CF-RESTRICT-*` | seven `column-four-control-c*` buttons | enabled only when Rust legal target exists; disabled when full, terminal, pending, or replay-only | Rust-provided legal target label | UI does not calculate full columns or landing rows. |

Flat column choice is the only public action flow. There is no compound/progressive construction flow.

## Rust-Generated Previews

| Preview | Trigger | Viewer-safe contents | Must not contain | Tests |
|---|---|---|---|---|
| landing cell preview | hover/focus on a column control | Rust-provided `landing_preview` cell id for that legal column | TypeScript-guessed row, future win/draw inference, bot-only facts | `apps/web/e2e/column-four.smoke.mjs` |

## Semantic Effect To Animation Mapping

| Semantic effect | Visual feedback | Reduced-motion replacement | Settle-to-view check | Rule IDs |
|---|---|---|---|---|
| `drop_accepted` | effect log text | text only | latest public view still renders | `CF-TURN-002` |
| `piece_landed` | landed cell receives a short drop animation | animation disabled; final piece remains visible | board is latest Rust public view | `CF-GRAVITY-001` |
| `active_player_changed` | status text and legal count update | text only | action tree refreshed from Rust | `CF-TURN-002` |
| `win_detected` | exactly Rust `winning_line` cells highlighted plus winner text | highlight/text remain | terminal controls disabled | `CF-END-001` through `CF-END-006` |
| `draw_detected` | draw status text; controls disabled | text only | terminal controls disabled | `CF-END-005` |
| `bot_chose_action` | public bot rationale panel | text only | bot move result comes through Rust view/effects | bot evidence pack |

## Outcome / victory explanation

Rust includes terminal outcome rationale in the public view so the browser can render the result without recomputing line detection, primary-line choice, or draw logic.

Terminal result variants are `win` and `draw`. Decisive cause variants are `line_completed` and `full_board_no_line`.

| Terminal kind | Template key | Decisive cause | Public fields | Rule IDs |
|---|---|---|---|---|
| win | `column_four.line_completed` | `line_completed` | `winning_seat`, ordered `winning_line`, `line_orientation`, `board_full=false` | `CF-SCORE-001` plus one of `CF-END-001` through `CF-END-004` |
| draw | `column_four.full_board_draw` | `full_board_no_line` | no line cells, `board_full=true` | `CF-SCORE-001`, `CF-END-005` |

There are no per-player breakdown fields for Column Four; the result is a perfect-information terminal winner or draw. The ordered line cells are the Rust-selected primary line, including the deterministic tie-break described by `CF-END-006`. Hidden-info redaction is still explicit: no hidden fields, no private view payload, and no state dumps in DOM attributes, logs, storage, replay export, or tests. TypeScript may map these public ids to labels, but it must not infer a winner, draw, landing row, line, orientation, or primary-line tie-break from board occupancy.

Web smoke coverage must assert that win and draw explanations render from Rust fields, highlight only Rust-supplied line cells, and keep the no-leak scan green.

## Replay UI

`ReplayViewer.tsx` reuses `ColumnFourBoard` in non-interactive mode. Replay reset/step projects the board from Rust replay state; the placement sequence is display metadata from the replay document. The UI does not reconstruct board state from command diffs.

## Accessibility And Keyboard Plan

| Element/control | Accessible name/description | Role/semantic element | Keyboard path | Notes |
|---|---|---|---|---|
| seven column controls | Rust-provided action label | `button` inside a grouped control region | Tab to column, Enter/Space to play | Covered by `column-four.smoke.mjs`. |
| board SVG | board summary via surrounding `role="img"` container and live summary text | `role="img"` plus `sr-only` status | non-interactive | Board state is also text summarized. |
| replay controls | existing shell labels | `button`/`textarea` | Tab/Enter | Existing replay smoke plus Column Four replay arm. |
| bot rationale | public prose panel | text region | normal document reading order | No candidate rankings. |

Focus indicators remain visible. Seat pieces use different color and shape, and terminal status is textual; color is not the only cue.

## Reduced Motion And Responsive Behavior

| Motion/effect | Default behavior | Reduced-motion behavior | Information preserved? | Test |
|---|---|---|---:|---|
| landed piece animation | short CSS translate/fade on Rust `piece_landed` cell | disabled under `.column-four-board.reduced` | yes | `column-four.smoke.mjs` |
| effect log animation | existing shell effect entry animation | disabled by reduced-motion shell | yes | `a11y-noleak.smoke.mjs` |

The board is width-constrained with a stable SVG aspect ratio. The no-leak/a11y smoke runs a narrow viewport path for the shell; Column Four E2E covers the core desktop board path.

## Hidden-Information Safeguards

| Surface | Safeguard | Test |
|---|---|---|
| browser payload/public view | Rust view contains public board, columns, legal targets, status, terminal, no hidden fields. | `npm --prefix apps/web run smoke:wasm` |
| action tree | Rust legal choices only. | `column-four.smoke.mjs`; `cargo run -p fixture-check -- --game column_four` |
| Rust-generated preview | Public landing cell only. | `column-four.smoke.mjs` |
| effect log | Viewer-safe semantic effects only. | `column-four.smoke.mjs`; replay traces |
| diagnostics/disabled reasons | Safe diagnostic codes/messages. | full-column/stale traces; WASM smoke |
| DOM attributes/test IDs | Public column ids and generic hooks only. | `column-four.smoke.mjs` no-leak scan |
| browser console/logs | No forbidden leak vocabulary. | `column-four.smoke.mjs` |
| local/session storage | Only reduced-motion preference; session storage empty. | `a11y-noleak.smoke.mjs`; Column Four no-leak path |
| replay export/import | Public command/hash metadata, explicit not-applicable private view marker. | `column-four.smoke.mjs`; replay-check |
| bot explanations | Public rationale only, no score arrays/rankings. | bot tests; `column-four.smoke.mjs` |
| candidate rankings | Not exposed publicly. | bot rationale tests/no-leak scans |
| dev inspector | Secondary shell panel with viewer-safe public metadata only. | shell/a11y smokes |

## Verification

- `npm --prefix apps/web run smoke:wasm`
- `npm --prefix apps/web run smoke:e2e`
- `node apps/web/e2e/column-four.smoke.mjs` against a built app
- `cargo run -p replay-check -- --game column_four --all`

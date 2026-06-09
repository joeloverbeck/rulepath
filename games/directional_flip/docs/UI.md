# Directional Flip UI Notes

Game ID: `directional_flip`

Implemented variant: `directional_flip_standard`

Rules version: `directional_flip-rules-v1`

Renderer assumptions version: `directional-flip-ui-v1`

Last updated: 2026-06-07

## Purpose

This document records the shipped web UI contract for Directional Flip. Rust/WASM owns setup, legal actions, forced-pass availability, placement previews, validation, semantic effects, replay projection, diagnostics, bot decisions, and viewer-safe public views. TypeScript renders those payloads.

## Product And Visual Target

| Field | Decision |
|---|---|
| public role | Gate 6 official game / release candidate after checklist |
| desired feel | compact, polished, play-first public board game |
| visual risk to avoid | proprietary naming, copied board presentation, debug-console-first UI |
| public onboarding need | moderate; legal-cell highlights, preview text, score, and effects explain play |
| help/learning mode need | none in Gate 6 |

## Renderer Assumptions

| Assumption | Value | Evidence/notes |
|---|---|---|
| default renderer | React + SVG marks inside a keyboard grid | [../../../apps/web/src/components/DirectionalFlipBoard.tsx](../../../apps/web/src/components/DirectionalFlipBoard.tsx) |
| expected object count | 64 grid cells, four starting discs, score/status panels, optional forced-pass control | Board remains low-pressure for React. |
| animation pressure | medium | Placement/flip animation is effect-driven and reduced-motion aware. |
| Canvas/PixiJS needed? | no | No profiling evidence or ADR supports another renderer. |
| WASM boundary | public view/action/effect/replay calls | `smoke:wasm` and Directional Flip browser smoke cover the path. |

## Legal Action Mapping

| Rust legal target | Rule IDs | UI control | Normal behavior | Accessibility label | Notes |
|---|---|---|---|---|---|
| `place/rNcM` | `DF-ACTION-001`, `DF-LEGAL-*` | board cell button `directional-cell-rNcM` | playable only when Rust legal target exists and board is interactive | Rust-provided legal target label | UI does not calculate legal placements or flips. |
| `pass/forced` | `DF-ACTION-002`, `DF-PASS-*` | forced-pass button | shown only when Rust legal target exposes forced pass | Rust/Rust metadata label | UI does not infer pass availability. |

The browser submits the Rust-provided `action_segment` unchanged. There is no command editor for normal play.

## Rust-Generated Previews

| Preview | Trigger | Viewer-safe contents | Must not contain | Tests |
|---|---|---|---|---|
| placement preview | hover/focus on a Rust-legal cell | target cell, ordered flip cells, direction groups, explanation | TypeScript-guessed flips, future outcome inference, bot-only ranking | `apps/web/e2e/directional-flip.smoke.mjs` |
| forced pass state | Rust legal target with no placement | forced-pass label/control and pass effect after action | browser-calculated no-move result | forced-pass bot path in browser smoke |

## Semantic Effect To Animation Mapping

| Semantic effect | Visual feedback | Reduced-motion replacement | Settle-to-view check | Rule IDs |
|---|---|---|---|---|
| `placement_accepted` | effect log text | text only | latest public view still renders | `DF-EFFECT-001` |
| `disc_placed` | placed cell/disc receives short placement animation | animation disabled; final disc remains visible | board is latest Rust public view | `DF-FLIP-001` |
| `discs_flipped` | Rust-listed flipped cells receive flip animation | animation disabled; final owner marks remain visible | board is latest Rust public view | `DF-EFFECT-002` |
| `pass_taken` | effect log/status text | text only | board is unchanged except active/terminal status | `DF-PASS-*` |
| `active_player_changed` | status text and legal targets update | text only | action tree refreshed from Rust | `DF-ACTION-*` |
| `game_ended` | terminal status and final score text | text only | no legal controls remain | `DF-TERM-001`, `DF-SCORE-*` |
| `bot_chose_action` | public bot rationale panel | text only | bot move result comes through Rust view/effects | `DF-BOT-002` |

## Outcome / victory explanation

Rust includes terminal outcome rationale in the public view so the browser can render the result without comparing scores or reading the effect log for the terminal trigger.

Terminal result variants are `win` and `draw`. Decisive cause variants are `final_score_comparison`.

| Terminal kind | Template key | Decisive cause | Breakdown fields | Rule IDs |
|---|---|---|---|---|
| win | `directional_flip.final_score_win` | `final_score_comparison` | `winning_seat`, `final_score.seat_0`, `final_score.seat_1`, `terminal_trigger` | `DF-SCORE-001` plus `DF-END-001`, `DF-END-002`, or `DF-END-003` |
| draw | `directional_flip.final_score_draw` | `final_score_comparison` | `final_score.seat_0`, `final_score.seat_1`, `terminal_trigger` | `DF-SCORE-002` plus `DF-END-001`, `DF-END-002`, or `DF-END-003` |

The per-player breakdown fields are the public final disc counts for `seat_0` and `seat_1`; `terminal_trigger` is one of `board_full`, `no_continuation`, or `double_forced_pass`. Hidden-info redaction is explicit: no hidden fields, no private view payload, and no state dumps in DOM attributes, logs, storage, replay export, or tests. TypeScript may map these public values to labels, but it must not compare scores, infer the terminal trigger, or decide win/draw.

Web smoke coverage must assert that win and draw explanations render from Rust fields and keep the no-leak scan green.

## Replay UI

`ReplayViewer.tsx` reuses `DirectionalFlipBoard` in non-interactive mode. Replay reset/step projects the board from Rust replay state. The placement sequence is public replay command metadata; the UI does not reconstruct board state from command diffs.

## Accessibility And Keyboard Plan

| Element/control | Accessible name/description | Role/semantic element | Keyboard path | Notes |
|---|---|---|---|---|
| board grid | Rust board label plus cell labels | `role="grid"` with cell buttons | Tab into grid, Arrow keys move, Enter/Space plays legal target, Escape clears preview | Covered by `directional-flip.smoke.mjs` and shared a11y smoke. |
| legal cell | Rust-provided action accessibility label | `button` with `aria-disabled` when not playable | Enter/Space submits only if Rust target exists | Non-legal cells remain focusable for grid navigation but do not submit. |
| forced pass | Rust/Rust metadata label | `button` | Tab/Enter/Space when interactive | Appears only when Rust exposes forced pass. |
| score/status | visible text panels | text/live status | normal document reading order | Scores are public. |
| bot rationale | public prose panel | text region | normal document reading order | No rankings or raw scores. |
| replay controls | existing shell labels | `button`/`textarea` | Tab/Enter | Existing replay smoke plus Directional Flip arm. |

Focus indicators remain visible. Seat discs use different SVG marks/patterns plus color, and scores/effects/status text provide non-color cues.

## Reduced Motion And Responsive Behavior

| Motion/effect | Default behavior | Reduced-motion behavior | Information preserved? | Test |
|---|---|---|---:|---|
| placement/flip animation | short CSS scale/flip on Rust effect cells | animation disabled under `.directional-flip-board.reduced` | yes | `directional-flip.smoke.mjs` |
| effect log animation | existing shell effect entry animation | disabled by reduced-motion shell | yes | `a11y-noleak.smoke.mjs` |

The board is width-constrained with a stable square aspect ratio. The shared no-leak/a11y smoke also runs a narrow viewport path for the shell.

## Hidden-Information Safeguards

| Surface | Safeguard | Test |
|---|---|---|
| browser payload/public view | Rust view contains public board, score, legal targets, previews, terminal, no hidden fields. | `npm --prefix apps/web run smoke:wasm` |
| action tree | Rust legal choices only. | `directional-flip.smoke.mjs`; rule tests |
| Rust-generated preview | Public target and ordered flip cells only. | `directional-flip.smoke.mjs`; preview tests |
| effect log | Viewer-safe semantic effects only. | replay traces; browser smoke |
| diagnostics | Safe diagnostic codes/messages. | WASM smoke and diagnostic traces |
| DOM attributes/test IDs | Public cell ids and generic hooks only. | `directional-flip.smoke.mjs` |
| browser console/logs | No forbidden leak vocabulary. | `directional-flip.smoke.mjs` |
| local/session storage | Only reduced-motion preference; session storage empty. | `a11y-noleak.smoke.mjs` |
| replay export/import | Public command/hash metadata, explicit not-applicable private view marker. | WASM smoke; browser replay smoke |
| bot explanations | Public rationale only, no score arrays/rankings. | bot tests; browser no-leak smoke |
| dev inspector | Secondary shell panel with viewer-safe public metadata only. | shell/a11y smokes |

## Verification

- `npm --prefix apps/web run smoke:wasm`
- `node apps/web/e2e/directional-flip.smoke.mjs` against a built app
- `node apps/web/e2e/a11y-noleak.smoke.mjs` against a built app
- `cargo run -p replay-check -- --game directional_flip --all`

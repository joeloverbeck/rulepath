# Draughts Lite UI Notes

Game ID: `draughts_lite`

Implemented variant: `draughts_lite_standard`

Rules version: `draughts_lite-rules-v1`

Renderer assumptions version: `draughts-lite-ui-v1`

Last updated: 2026-06-07

## Purpose

This document records the shipped web UI contract for Draughts Lite. Rust/WASM owns setup, public view, legal action trees, validation, semantic effects, bot turns, diagnostics, replay import/export, and replay stepping. TypeScript renders those payloads and stores only transient UI state for an in-progress path.

## Renderer Assumptions

| Assumption | Value | Evidence/notes |
|---|---|---|
| default renderer | React grid of 64 cell buttons | [../../../apps/web/src/components/DraughtsLiteBoard.tsx](../../../apps/web/src/components/DraughtsLiteBoard.tsx) |
| object count | 64 cells, 24 opening pieces, status/cue/path panels | Stable square board with no canvas/WebGL need. |
| input model | select Rust origin, then Rust landing/continuation children, submit only a leaf path | `ActionChoice.next` from WASM |
| animation pressure | low/medium | Static highlights and effect log are primary; reduced motion removes transitions. |
| WASM boundary | catalog, view, action tree, apply path, effects, bot, replay | `smoke:wasm`, `draughts-lite.smoke.mjs` |

## Legal Action Mapping

| Rust action-tree choice | UI control | Normal behavior | Accessibility label | Notes |
|---|---|---|---|---|
| `from/rNcM` | board cell `draughts-cell-rNcM` | selects a Rust-legal origin and stores TS-only pending path | Rust-provided origin label | No command is sent yet. |
| `to/rNcM` | board cell `draughts-cell-rNcM` | submits a quiet leaf path if there are no children | Rust-provided landing label | Path is sent as `from/... > to/...` through WASM delimiter. |
| `jump/rNcM` | board cell `draughts-cell-rNcM` | appends capture landing; submits only at a leaf | Rust-provided landing label | Continuation children come only from Rust. |
| cancel | Cancel button or Escape | clears TS-only pending path | visible button text | No replay command is recorded. |

The UI never computes diagonals, playable parity, captures, continuation, promotion, terminal outcome, or bot moves. It maps Rust metadata such as `cell_id`, `capture_mandatory`, `is_capture`, `forced_by_continuation`, `would_promote`, and preview cells to labels/classes only.

## Visual And Text Cues

| Cue | Source | UI behavior |
|---|---|---|
| all cells and pieces | Rust public view | render `r1c1` through `r8c8`, ownership, man/crown shape text, and public labels |
| legal origins/destinations | Rust action tree | highlighted cells and accessible names |
| selected origin | pending path + selected tree node | selected cell highlight and pending path text |
| mandatory capture | Rust metadata/effects | live-region text and capture cue count |
| forced continuation | Rust `next` children and metadata | only continuation choices remain enabled; live region announces continuation |
| promotion | Rust metadata/effect | promotion cue and effect-log text |
| terminal win | Rust public view/effect | terminal status and no legal controls |
| recent path/captures | Rust effects | static highlights for origin, landing, and captured cell |

## Semantic Effect To Feedback Mapping

| Semantic effect | Visual feedback | Reduced-motion replacement | Settle-to-view check |
|---|---|---|---|
| `move_committed` / `MoveCommitted` | effect log text; recent origin/final highlights | text and static highlights | latest Rust public view renders |
| `quiet_step` / `QuietStep` | landing highlight and effect text | text/static highlight | piece is in latest view |
| `capture_step` / `CaptureStep` | captured-cell and landing highlights | text/static highlight | captured piece absent from latest view |
| `promotion` / `Promotion` | promotion cue/effect text | text/static highlight | piece kind is crown in latest view |
| `forced_capture_available` / `ForcedCaptureAvailable` | live-region/status text | text only | action tree refreshed from Rust |
| `forced_continuation_required` / `ForcedContinuationRequired` | live-region/status text | text only | only Rust continuation choices enabled |
| `terminal_win` / `TerminalWin` | terminal text | text only | no legal controls remain |
| `bot_chose_action` / `BotChoseAction` | public bot rationale panel | text only | bot move result comes through Rust view/effects |

## Outcome / victory explanation

Rust includes terminal outcome rationale in the public view so the browser can render why the win occurred without recomputing piece counts or legal moves.

Terminal result variants are `win` only for the declared variant. Decisive cause variants are `opponent_no_pieces` and `opponent_no_legal_move`.

| Terminal kind | Template key | Decisive cause | Breakdown fields | Rule IDs |
|---|---|---|---|---|
| win | `draughts_lite.opponent_no_pieces` | `opponent_no_pieces` | `winning_seat`, `losing_seat`, `seat_0_pieces`, `seat_1_pieces`, `losing_legal_move_count=0` | `DL-END-001` |
| win | `draughts_lite.opponent_no_legal_move` | `opponent_no_legal_move` | `winning_seat`, `losing_seat`, `seat_0_pieces`, `seat_1_pieces`, `losing_legal_move_count=0` | `DL-END-002` |

The per-player breakdown fields are public piece totals split by man/crown counts. Hidden-info redaction is trivial because Draughts Lite is perfect-information, but no state dumps, candidate rankings, debug internals, or recomputed legal-move scans may be exposed through DOM attributes, logs, storage, replay export, or tests.

Web smoke coverage must assert that terminal explanations render from Rust fields, terminal controls disappear, and the no-leak scan stays green.

## Accessibility And Keyboard

| Element/control | Role/semantic element | Keyboard path | Notes |
|---|---|---|---|
| board | `role="grid"` with `aria-activedescendant` | Tab into grid, arrows move by cell, Home/End move by row, Ctrl+Home/End move to board ends | Covered by `draughts-lite.smoke.mjs`. |
| cell | button with `role="gridcell"` and `aria-disabled` | Enter/Space activates only if Rust choice exists | Non-legal cells remain roving-focus targets but do not submit. |
| pending path | text/live region | normal reading order | Shows full segments with ` > `. |
| cancel | button | Escape or button activation | Clears UI-only path. |
| status/effects | text and `aria-live` | normal reading order | Announces turn, selected piece, destination count, mandatory capture, continuation, promotion, bot, and terminal feedback. |
| replay controls | existing shell controls | Tab/Enter | Replay viewer reuses the board in non-interactive mode. |

Pieces use non-color shape/text marks: men are discs, crowns show `K`, and seats have distinct visual treatments plus public text labels.

## Replay UI

`ReplayViewer.tsx` reuses `DraughtsLiteBoard` in non-interactive mode. `ReplayImportExport.tsx`, `ReplayViewer.tsx`, and `DevPanel.tsx` render full multi-segment paths such as `from/r3c2 > jump/r5c4 > jump/r7c6`; they do not truncate at the first segment.

## Hidden-Information Safeguards

| Surface | Safeguard | Test |
|---|---|---|
| public view | Rust projects public board/pieces/status only. | visibility tests; WASM smoke |
| action tree | Rust legal choices and viewer-safe metadata only. | action tests; browser smoke |
| pending path | UI-only until a leaf submits. | browser smoke |
| diagnostics/effects | safe codes/messages and public semantic payloads. | golden traces; E2E no-leak |
| DOM/test IDs | public cell IDs only; no state dumps. | `draughts-lite.smoke.mjs` |
| local/session storage | only reduced-motion preference; session empty. | shared no-leak smoke |
| replay export/import | public commands/hashes and explicit private-view not-applicable marker. | WASM and browser replay smokes |
| bot explanations | public rationale, no rankings/scores/debug/search claims. | bot tests; browser smoke |

## Verification

- `npm --prefix apps/web run smoke:wasm`
- `node apps/web/e2e/draughts-lite.smoke.mjs` against a built app
- `npm --prefix apps/web run smoke:e2e`
- `cargo run -p replay-check -- --game draughts_lite --all`

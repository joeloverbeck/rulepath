# race_to_n UI

Game ID: `race_to_n`

Implemented variant: `race_to_21`

Rules version: `1`

Renderer assumptions version: `2`

Prepared by: `Codex`

Last updated: 2026-06-06

## Purpose

This document records the Gate 3 browser shell for `race_to_n` / Race to 21.

Rust/WASM owns legality, validation, state transitions, public views, semantic
effects, diagnostics, bot decisions, replay export/import, and replay projection.
TypeScript/React owns presentation, local UI state, keyboard interaction, reduced
motion preferences, and static serving only.

## Product and Visual Target

| Field | Decision |
|---|---|
| public role | Gate 3 static web shell baseline |
| desired feel | clear, compact, play-first abstract counter race |
| visual risk to avoid | debug-console-first, raw JSON as the primary UI, proprietary mimicry |
| public onboarding need | minimal labels and visible state summaries |
| help/learning mode need | none for Gate 3 |

## Shell Regions

| Region | Source of truth | Notes |
|---|---|---|
| App shell and WASM status | Rust version string through WASM client | Shows load/error state before play. |
| Game picker | `list_games` | Gate 3 supports `race_to_n` only. |
| Match setup | local seed/mode form; Rust `new_match` on start | Modes are human-vs-bot, hotseat, bot-vs-bot. |
| Race board | Rust public view | Shows counter, target, active seat/winner, freshness token. |
| Action controls | Rust action tree | Buttons are absent when Rust returns no legal choices. |
| Effect log | Rust semantic effects | Rows include text labels and tone classes; animation is optional. |
| Replay import/export | Rust replay ops | UI passes replay text to Rust and renders Rust-projected reset/step output. |
| Developer panel | Whitelisted public data | Secondary panel, not the primary play surface. |

## Action Mapping

| Rust action choice/tree node | Rule IDs | UI control/hit target | Accessibility label | Notes |
|---|---|---|---|---|
| `add-1` | `R-TURN-001`, `R-ACT-001` | button from Rust action tree | Rust-provided label | Rendered only when present in the Rust tree. |
| `add-2` | `R-TURN-001`, `R-ACT-001` | button from Rust action tree | Rust-provided label | Rendered only when present in the Rust tree. |
| `add-3` | `R-TURN-001`, `R-ACT-001` | button from Rust action tree | Rust-provided label | Rendered only when present in the Rust tree. |

TypeScript never infers legal alternatives. It submits the selected Rust segment
with the Rust freshness token and displays Rust diagnostics on failure.

## Play Modes

| Mode | Behavior | Authority |
|---|---|---|
| Human vs bot | Seat 0 is local; Seat 1 uses Rust random-legal bot turns. | Rust action tree and bot op. |
| Hotseat | Both seats are local controls on the same device. | Rust action tree for active seat. |
| Bot vs bot | Both seats use Rust bot turns; user can step or start/pause autoplay. | Rust bot op; TypeScript only schedules local turns. |

## Replay UI

| Surface | Behavior | Safety |
|---|---|---|
| Export Current Run | Calls Rust `export_replay` for the active match and places formatted JSON in the textarea. | Local only; no network/backend. |
| Import Replay | Rejects documents over 128 KiB before calling Rust `import_replay`. | Rust parses and validates. |
| Replay viewer | Calls Rust `replay_reset` and `replay_step`, then renders public projected state/effects. | TypeScript does not replay commands itself. |

The replay schema may include `expected_private_view_hashes.not_applicable` because
Race to 21 is perfect information. No private view payload is exposed.

## Semantic Effect Mapping

| Semantic effect | Visual cue | Reduced-motion replacement | Settle-to-view check |
|---|---|---|---|
| `action_started` | Text row in effect log | same text row | Latest public view remains visible. |
| `counter_advanced` | Counter fill and numeric counter update | instant width update | Counter text and track match Rust view. |
| `turn_changed` | Turn field and text row update | same | Turn field matches Rust view. |
| `game_ended` | Winner text and terminal effect row | same | Winner matches Rust view. |
| `action_completed` | Text row in effect log | same | Latest public view remains visible. |

## Outcome / victory explanation

Rust includes terminal outcome rationale in the public view so the browser can render the result without comparing the counter to the target.

Terminal result variants are `win` only for the declared variant. Decisive cause variants are `exact_target_reached`.

| Terminal kind | Template key | Decisive cause | Breakdown fields | Rule IDs |
|---|---|---|---|---|
| win | `race_to_n.exact_target_reached` | `exact_target_reached` | `counter_before`, `addition`, `counter_after`, `target`, `max_add`, `winning_seat` | `R-SCORE-001`, `R-END-001` |

The per-player breakdown fields are minimal because Race to N has no score per seat beyond the winner; the public breakdown names the exact counter advance that reached the target. Hidden-info redaction is explicit: no hidden fields, no private view payload, and no state dumps in DOM attributes, logs, storage, replay export, or tests. TypeScript may map public numbers to labels, but it must not compare counter values, decide whether the target was reached, or infer the winner.

Web smoke coverage must assert that the win explanation renders from Rust fields and keep the no-leak scan green.

## Accessibility

| Requirement | Gate 3 stance | Evidence |
|---|---|---|
| keyboard path | setup, start, actions, dev panel, replay export/import/step are keyboard reachable | `npm --prefix apps/web run smoke:e2e` |
| focus visible | controls and custom mode labels have explicit focus outlines | CSS + E2E assertion |
| accessible names | buttons, inputs, selects, and textarea have visible labels/text or `aria-label` | E2E assertion |
| no color-only state | counter, turn, diagnostics, and effect rows include text | E2E assertion |
| reduced motion | `prefers-reduced-motion` and explicit override preserve information without animation | E2E assertion |
| responsive baseline | smoke runs the a11y/no-leak flow at a narrow viewport | E2E assertion |

## Developer Panel Boundary

| Item | Allowed? | Must not contain |
|---|---:|---|
| API version, feature names, operation count | yes | raw memory |
| selected game, match id, seed, mode | yes | hidden state |
| active actor, freshness token, action count | yes | full internal Rust state |
| effect cursor/count, pending op | yes | hidden outcomes |
| replay id/cursor | yes | replay private payloads |
| diagnostics | yes | hidden state or internal stack traces |
| bot explanation/candidate ranking | no | all hidden/private reasoning |

## Hidden-Information Safeguards

| Surface | Safeguard | Evidence |
|---|---|---|
| browser payload/public view | `get_view` returns Rust public view only | wasm-api tests, smoke scripts |
| action tree | controls derive from Rust tree only | browser smoke |
| effect log | `get_effects` returns viewer-safe effects | smoke scripts |
| diagnostics | Rust diagnostic code/message only | stale-action smoke |
| DOM attributes/test IDs | generic UI hooks, not state dumps | a11y/no-leak smoke |
| console logs | no app state logging | a11y/no-leak smoke captures console |
| storage | only `rulepath.reducedMotion` UI preference is allowed; session storage remains empty | a11y/no-leak smoke |
| replay export/import | Rust-owned schema; explicit not-applicable marker for private views | replay smoke + no-leak checklist |
| developer panel | public whitelist only | no-leak checklist |

## Smoke Tests

| Smoke test | Command | Notes |
|---|---|---|
| raw WASM ABI | `npm --prefix apps/web run smoke:wasm` | version/features, catalog, match, actions, bot, effects, replay |
| shell state | `npm --prefix apps/web run smoke:ui` | fast Node/WASM shell-state path |
| built static dist | `npm --prefix apps/web run smoke:preview` | nested `/rulepath/` static mount |
| rendered browser | `npm --prefix apps/web run smoke:e2e` | Puppeteer shell flow plus a11y/no-leak pass |

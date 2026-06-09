# High Card Duel UI

Game ID: `high_card_duel`

Rules version: `high-card-duel-rules-v1`

Last updated: 2026-06-07

## Contract

The web UI presents Rust/WASM output only. It never computes legality, reveal timing, scores, hand contents, deck order, or bot choice. Viewer mode is forwarded to Rust; the returned projection decides what can be rendered.

## Viewer Modes

| Viewer | UI behavior |
|---|---|
| Observer | Shows public round, score, active seat, hand counts, deck count, card backs, face-down commitment occupancy, and revealed cards only after Rust reveal. No private commit actions. |
| Seat 0 / Seat 1 | Shows that seat's own hand and own commitment identity. Opponent hand remains count/backs only. Legal commit buttons are built from the Rust action tree. |

## No-Leak Requirements

No browser-visible surface may contain private card IDs such as `hcd:r..`, deck order, hidden state, bot candidates, or private diagnostics unless the card has become public by Rust reveal and the surface is intended to show public card information. The current board avoids raw card IDs in DOM text, labels, data attributes, test IDs, dev-panel action paths, console output, storage, and pre-reveal replay exports.

## Outcome / victory explanation

Rust includes terminal outcome rationale in the public view so the browser can render the final result without comparing cards, comparing scores, or reading hidden deck state.

Terminal result variants are `win` and `draw`. Decisive cause variants are `final_score_after_round_limit`.

| Terminal kind | Template key | Decisive cause | Breakdown fields | Rule IDs |
|---|---|---|---|---|
| win | `high_card_duel.final_score_win` | `final_score_after_round_limit` | `winning_seat`, `final_score`, revealed per-round ranks, round winner/tie, point delta, cumulative score | `HCD-ROUND-005`, `HCD-END-001`, `HCD-END-002` |
| draw | `high_card_duel.final_score_draw` | `final_score_after_round_limit` | `final_score`, revealed per-round ranks, round tie, point delta, cumulative score | `HCD-ROUND-006`, `HCD-END-001`, `HCD-END-003` |

The per-player breakdown fields are public final score and public revealed round history only. Hidden-info redaction is load-bearing: never expose unrevealed deck order/tail, private unplayed hands, face-down commitment identity before reveal, bot candidates, or future draw identities in the view, rationale, effect log, replay export, DOM attributes, logs, storage, or tests.

Web smoke coverage must assert that win/draw explanations render from Rust fields, revealed cards only appear after Rust reveal, unrevealed deck tail stays absent, and the no-leak scan stays green.

## Accessibility And Motion

The board exposes named viewer controls, named private commit buttons, text score/round/phase status, and an aria-live board status. Reveal animation is effect-driven from `cards_revealed` and disabled by reduced-motion mode.

## Evidence

- `npm --prefix apps/web run build`
- `npm --prefix apps/web run smoke:ui`
- `node apps/web/e2e/high-card-duel.smoke.mjs`

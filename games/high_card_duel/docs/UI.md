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

## Accessibility And Motion

The board exposes named viewer controls, named private commit buttons, text score/round/phase status, and an aria-live board status. Reveal animation is effect-driven from `cards_revealed` and disabled by reduced-motion mode.

## Evidence

- `npm --prefix apps/web run build`
- `npm --prefix apps/web run smoke:ui`
- `node apps/web/e2e/high-card-duel.smoke.mjs`

# Token Bazaar UI

Game ID: `token_bazaar`

Rules version: `token-bazaar-rules-v1`

Last updated: 2026-06-08

## Contract

The web UI presents Rust/WASM output only. It never computes legality, affordability, market refill, terminal outcome, tie-breaks, or bot choice. The board consumes `TokenBazaarPublicView`, `ActionTree`, and effect payloads from the WASM bridge.

## Board Layout

`TokenBazaarBoard.tsx` renders:

| Surface | Rust payload source | UI behavior |
|---|---|---|
| active seat and terminal status | `active_seat`, `terminal`, `turns_taken` | Text turn pill and heading. |
| scores and turn cap | `scores`, `turns_taken.turns_per_seat` | Four metric boxes with labels. |
| inventories | `inventories`, `fulfilled` | One panel per seat with resource chips, score, fulfilled contract count/list. |
| public supply | `supply`, `ui.supply_label` | Central public supply panel. |
| market row | `market_slots`, contract labels/costs/points, `queue_remaining` | Three contract cards; empty slots remain visible. |
| legal controls | `actionTree.choices` | Native buttons submit Rust action segments through WASM. |
| recent accounting | `recent_effects` and effect log entries | Text list of recent public accounting effects. |
| replay | exported command stream and WASM replay projection | Replay viewer shows command sequence and snapshots. |

## UI Metadata

Rust `ui.rs` provides the stable labels:

| Field | Current value |
|---|---|
| `table_label` | `Token Bazaar market` |
| `supply_label` | `Public supply` |
| `inventory_label` | `Player inventory` |
| `market_label` | `Visible contracts` |
| `score_label` | `Score` |
| `turn_counter_label` | `Turns taken` |
| `reduced_motion_token` | `resource-accounting-reduced-motion` |

## Accessibility And Motion

- Resource information is not color-only. Each chip includes a short code, full resource name, and numeric count.
- Contract cards include label, cost chips, and point value as text.
- Economy controls are native buttons with accessible names from Rust `accessibility_label`.
- Focus-visible styling is inherited from the shell and asserted by `token-bazaar.smoke.mjs`.
- Reduced-motion mode adds `.reduced` to the board and `.effect-entry.reduced` to log rows; the e2e smoke exercises that path.
- Responsive layout collapses metrics, supply/inventory panels, and market slots to one column on narrow viewports.

## No-Leak Requirements

Token Bazaar is fully public, but browser surfaces must still avoid hidden/debug/candidate/internal data. The DOM, attributes, storage, console, replay export, and dev panel must not contain bot candidate rankings, debug tables, internal state dumps, private state, or hidden state. `apps/web/e2e/token-bazaar.smoke.mjs` asserts those terms are absent.

## Evidence

- `npm --prefix apps/web run build`
- `npm --prefix apps/web run smoke:ui`
- `node apps/web/e2e/token-bazaar.smoke.mjs`
- `npm --prefix apps/web run smoke:e2e`

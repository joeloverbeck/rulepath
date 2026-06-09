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

## Outcome / victory explanation

Rust includes terminal outcome rationale in the public view so the browser can render the tiebreak ladder without selecting the decisive rung.

Terminal result variants are `win` and `draw`. Terminal trigger variants are `turn_cap` and `market_exhaustion`. Decisive cause variants are `score`, `fulfilled_contracts`, `inventory_total`, and `all_tied_draw`.

| Terminal kind | Template key | Decisive cause | Breakdown fields | Rule IDs |
|---|---|---|---|---|
| win | `token_bazaar.score_win` | `score` | `winning_seat`, final score/fulfilled/inventory standing by seat, ordered ladder with score marked decisive | `TB-END-001` or `TB-END-002`, `TB-END-003`, `TB-SCORE-001` |
| win | `token_bazaar.fulfilled_tiebreak_win` | `fulfilled_contracts` | `winning_seat`, final score/fulfilled/inventory standing by seat, ordered ladder with fulfilled contracts marked decisive | `TB-END-001` or `TB-END-002`, `TB-END-003`, `TB-SCORE-001`, `TB-SCORE-004` |
| win | `token_bazaar.inventory_tiebreak_win` | `inventory_total` | `winning_seat`, final score/fulfilled/inventory standing by seat, ordered ladder with inventory total marked decisive | `TB-END-001` or `TB-END-002`, `TB-END-003`, `TB-SCORE-001`, `TB-SCORE-004`, `TB-SCORE-005` |
| draw | `token_bazaar.all_tied_draw` | `all_tied_draw` | final score/fulfilled/inventory standing by seat, ordered ladder with all tied draw marked decisive | `TB-END-001` or `TB-END-002`, `TB-END-003`, `TB-SCORE-001`, `TB-SCORE-004`, `TB-SCORE-005` |

The per-player breakdown fields are all public: final score, fulfilled contract count, and total remaining inventory. Hidden-info redaction is trivial because Token Bazaar is perfect-information, but no state dumps, candidate rankings, debug internals, or recomputed tiebreak scans may be exposed through DOM attributes, logs, storage, replay export, or tests. TypeScript must render Rust's decisive marker and must not compare scores, fulfilled counts, or inventory totals to decide why the match ended.

Web smoke coverage must assert that terminal explanations render from Rust fields, terminal controls disappear, and the no-leak scan stays green.

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

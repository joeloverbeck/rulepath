# Crest Ledger UI

Game ID: `poker_lite`

Implemented variant: `poker_lite_standard`

Rules version: `poker-lite-rules-v1`

Last updated: 2026-06-09

## Contract

The web UI presents Rust/WASM output only. It never computes legality,
availability, pledge accounting, reveal timing, terminal allocation, showdown
comparison, hidden-info redaction, replay authority, stale diagnostics, or bot
choice. The board consumes `PokerLitePublicView`, `ActionTree`, and effect
payloads from the WASM bridge.

## Board Layout

`PokerLiteBoard.tsx` renders:

| Surface | Rust payload source | UI behavior |
|---|---|---|
| active seat and terminal status | `active_seat`, `terminal`, `round` | Turn pill, heading, and terminal panel. |
| shared pool and round facts | `shared_pool`, `round.round_index`, `round.round_unit`, `round.outstanding_actor` | Metric boxes with viewer-safe labels. |
| seat ledgers | `contributions`, `private_counts`, `active_seat` | One panel per seat with public contribution count and hidden private count. |
| center crest | `center` | Hidden placeholder before center reveal; `CrestCard` after Rust marks it revealed. |
| contribution track | `contributions` | Public bar visualization of per-seat contribution totals. |
| private view | `private_view` | Owner seat view shows only the owner's private crest; observer/opponent get a hidden placeholder. |
| legal actions | `actionTree.choices` | Native buttons for Rust-legal choices only. |
| grouped showdown | `showdown` | One public grouped reveal panel after showdown terminal. |
| terminal outcome | `terminal` | Yield terminal states that the match resolved without private reveal; showdown states show shared-pool result. |
| recent effects | `latestEffect` and effect log entries | Text status for setup, pledge, reveal, allocation, terminal, and bot events. |
| replay | viewer-scoped public export/import projection | Replay viewer shows public timeline with redacted command summaries. |

## UI Metadata

Rust `ui.rs` provides the stable labels:

| Field | Current value |
|---|---|
| `display_name` | `Crest Ledger` |
| `surface_label` | `Crest Ledger board` |
| `shared_pool_label` | `Shared pool` |
| `hidden_center_label` | `Center crest hidden` |
| `hidden_private_label` | `Private crest hidden` |
| `hold_label` | `Hold` |
| `press_label` | `Press` |
| `lift_label` | `Lift` |
| `match_label` | `Match` |
| `yield_label` | `Yield` |
| `reduced_motion_note` | `Use simple reveal changes when reduced motion is enabled` |

## Legal Action Mapping

| Rust action | Rule IDs | UI control | Accessibility label source | Notes |
|---|---|---|---|---|
| `hold` | `CL-ACT-001` | Native action button | `choice.accessibility_label` from Rust | Adds no markers. |
| `press` | `CL-ACT-001` | Native action button | `choice.accessibility_label` from Rust | Shows Rust metadata such as added markers. |
| `lift` | `CL-ACT-002`, `CL-ACT-003` | Native action button only while Rust exposes it | `choice.accessibility_label` from Rust | Hidden after the round lift cap is consumed. |
| `match` | `CL-ACT-002` | Native action button | `choice.accessibility_label` from Rust | Uses Rust required-to-match metadata. |
| `yield` | `CL-ACT-002`, `CL-END-001` | Native action button | `choice.accessibility_label` from Rust | Ends without private reveal. |

Action `data-testid` values use `choice-poker-lite-round-${round}-${index}`.
They intentionally do not include action segments that could later carry hidden
ids if the action tree evolves.

## Accessibility And Motion

- Core choices are native buttons, reachable by keyboard tab order and
  activatable with Enter/Space.
- Board status is announced through a screen-reader-only live summary and a
  visible latest-effect status region.
- Seat ledgers, center crest, private view, actions, grouped showdown, and
  terminal panels use semantic sections and labels.
- Crest cards include accessible labels from Rust card metadata only after the
  viewer is authorized to see that crest.
- Color is not the only information channel; labels, headings, counts, and
  visible card text carry the state.
- Reduced-motion mode adds `.reduced` and suppresses crest reveal animation
  while preserving all text feedback and final state.
- Responsive layout keeps the table grid measurable on narrow viewports.

## No-Leak Requirements

Before the rule-defined reveal point, hidden crest ids, ranks, copy names, and
labels must not appear in observer/opponent browser payloads, DOM text,
attributes, `data-testid` values, local storage, session storage, console logs,
public replay export/import, dev-panel text, bot public rationale, or effect
logs. Owner seat view may show only that seat's own private crest.

After center reveal, the center crest is public. After showdown, both private
crests and the center crest are public as a grouped reveal. After yield,
private crests remain hidden and the terminal panel says the match resolved
without a private reveal.

## Evidence

- `npm --prefix apps/web run build`
- `npm --prefix apps/web run smoke:ui`
- `npm --prefix apps/web run smoke:wasm`
- `node apps/web/e2e/poker-lite.smoke.mjs`
- `npm --prefix apps/web run smoke:e2e`

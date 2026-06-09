# Veiled Draft UI

Game ID: `secret_draft`

Rules version: `secret-draft-rules-v1`

Last updated: 2026-06-08

## Contract

The web UI presents Rust/WASM output only. It never computes legality,
commitment availability, reveal timing, conflict fallback, scoring, terminal
outcome, tie-breaks, replay authority, hidden-info redaction, or bot choice. The
board consumes `SecretDraftPublicView`, `ActionTree`, and effect payloads from
the WASM bridge.

## Board Layout

`SecretDraftBoard.tsx` renders:

| Surface | Rust payload source | UI behavior |
|---|---|---|
| active seat and terminal status | `active_seat`, `terminal`, `round_number` | Text turn pill and heading. |
| round, score, priority, pool count | `round_number`, `round_limit`, `scores`, `priority_seat`, `visible_pool` | Metric boxes with labels. |
| drafted collections | `drafted.seat_0`, `drafted.seat_1` | One panel per seat with public awarded items. |
| visible pool | `visible_pool`, `actionTree.choices` | Native buttons for Rust-legal commitments while interactive. |
| pending seats | `commitments.seat_0`, `commitments.seat_1` | Seat/round status cards showing committed/waiting only. |
| reveal history | `revealed_history` | Ordered list with public awards and conflict result after reveal. |
| outcome rationale | `terminal.rationale` | Terminal-only public explanation of the sixth-reveal trigger, final standing, and decisive tie-break rung. |
| recent effects | `latestEffect` and effect log entries | Text status for commitment, pending, reveal, scoring, and terminal effects. |
| replay | viewer-scoped public export/import projection | Replay viewer shows public timeline with redacted command summaries. |

## UI Metadata

Rust `ui.rs` provides the stable labels:

| Field | Current value |
|---|---|
| `table_label` | `Veiled Draft table` |
| `visible_pool_label` | `Visible draft pool` |
| `pending_label` | `Pending commitments` |
| `score_label` | `Score` |
| `reduced_motion_token` | `secret-draft-reduced-motion` |

## Accessibility And Motion

- Draft items are not color-only. Each visible item includes thread text, label,
  and value.
- Commitment controls are native buttons with accessible names from Rust
  `accessibility_label`.
- Pending state uses seat labels plus `Committed`/`Waiting` text and "Choice
  hidden" copy.
- Reveal history names the round, awards, and conflict outcome as text.
- Focus-visible styling is inherited from the shell and asserted by
  `secret-draft.smoke.mjs`.
- Reduced-motion mode adds `.reduced` to the board and suppresses item/reveal
  animation while preserving text feedback and effect order.
- Responsive layout keeps the visible-pool grid measurable on narrow viewports.

## No-Leak Requirements

Before reveal, the committed item id must not appear in browser payloads, DOM
text, attributes, `data-testid` values, storage, console logs, the dev panel,
bot rationale, or viewer-scoped replay export. Pending UI uses seat/round
anchors such as `secret-draft-pending-seat_0-round-1`; visible action controls
use round/index anchors rather than committed item ids.

After reveal, item ids and labels are public and may appear in reveal history,
drafted collections, effect feedback, and replay projection.

Terminal outcome rationale is emitted only after the sixth reveal resolves. It
uses public drafted collections, scores, complete-set counts, highest single
values, distinct-thread counts, priority-conflict counts, the terminal trigger
rule ID, and the decisive tie-break rule IDs. It must not expose any committed
item before reveal or any private-only command authority.

## Outcome / victory explanation

Rust projects the terminal explanation as part of `TerminalView`. Win views
carry `winning_seat` plus `OutcomeRationaleView`; draw views carry
`OutcomeRationaleView`. The terminal result variants are `Win` and `Draw`; the
decisive cause variants are `score`, `complete_sets`, `highest_single_value`,
`distinct_threads`, `fewer_priority_conflict_wins`, and `all_tied_draw`. The
rationale includes:

| Field | Meaning |
|---|---|
| `result_kind` | `win` or `draw`. |
| `decisive_cause` | One of `score`, `complete_sets`, `highest_single_value`, `distinct_threads`, `fewer_priority_conflict_wins`, or `all_tied_draw`. |
| `template_key` | Stable copy key for the decisive cause, such as `secret_draft.score_win` or `secret_draft.all_tied_draw`. |
| `decisive_rule_ids` | `SD-END-001`, `SD-END-002`, and the tie-break rule IDs needed to reach the decisive rung. |
| `terminal_trigger` | `sixth_reveal_complete`. |
| `terminal_trigger_rule_id` | `SD-END-001`. |
| `final_standing` | Public per-player breakdown fields: scores, complete-set counts, highest single values, distinct-thread counts, priority-conflict counts, and drafted items. |
| `ladder` | Ordered public comparison ladder with exactly one decisive rung. |

The rule IDs are defined in `RULES.md` as `SD-END-001`, `SD-END-002`,
`SD-SCORE-001`, `SD-SCORE-002`, `SD-COMP-004`, `SD-COMP-003`, and `SD-AMB-004`.
Web smoke coverage remains responsible for confirming the browser renders Rust
payloads without adding legality, scoring, or hidden-info behavior.

## Evidence

- `npm --prefix apps/web run build`
- `npm --prefix apps/web run smoke:ui`
- `node apps/web/e2e/secret-draft.smoke.mjs`
- `npm --prefix apps/web run smoke:e2e`

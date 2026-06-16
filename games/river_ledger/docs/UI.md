# River Ledger UI

Game ID: `river_ledger`

Implemented variant: `river_ledger_standard`

Rules version: `river-ledger-rules-v1`

Last updated: 2026-06-15

## Contract

The web UI presents Rust/WASM output only. It never computes legal actions,
validation, street advancement, contribution obligations, hand strength,
showdown winners, split allocation, hidden-info redaction, replay authority, or
bot decisions. The board consumes River Ledger catalog metadata, structured
viewer-safe match views, legal action trees, terminal rationale, and semantic
effects from the WASM bridge.

## Board Layout

`RiverLedgerBoard.tsx` renders:

| Surface | Rust/WASM payload source | UI behavior |
|---|---|---|
| match phase and active seat | `phase`, `street`, `active_seat`, `pending_seats`, `terminal` | Status text, active marker, and terminal panel. |
| seat rail | `seats[]`, roles, statuses, contributions, acted flags | Six-seat-capable ledger with button, small blind, big blind, active, pending, folded, and terminal states. |
| public board | `community_cards`, `street` | Public community-card slots with hidden placeholders for unrevealed slots. |
| contribution ledger | ledger total (`pot` JSON field), `current_bet`, `street_bet`, per-seat contribution fields | Abstract ledger/unit counters only; no money/chip/pot/rake public copy. |
| private view | `private_view` for the requested viewer | Authorized seat viewer sees only that seat's hole cards; observer and other seats see hidden placeholders. |
| legal actions | `actionTree.choices` and Rust action metadata | Native buttons for Rust-legal choices only, with call price, added ledger units, and cap-left copy from Rust metadata. |
| diagnostics and effects | Rust diagnostics and `latestEffect` | Safe status text and effect log entries. |
| hand ranking reference | Rust/WASM catalog metadata | Collapsible category ladder, default-visible after showdown, with current category markers from Rust-projected showdown data. |
| outcome rationale | terminal public view and terminal effects | Decisive Rust-authored result sentence, foldout rationale, showdown hand summaries, best-five visual cards, split/remainder explanation, teaching aid, and final ledger totals. |
| replay | viewer-scoped public replay export/import projection | Replay viewer steps Rust-projected public states and never reconstructs hidden state in TypeScript. |

## N-Seat Viewer Matrix

River Ledger supports 3, 4, 5, and 6 seats. The browser shell uses the catalog
`supportedSeatCounts` and `defaultSeats` fields; it does not hard-code River
Ledger seat legality.

| Viewer | Authorized private data | Public facts | Must remain hidden |
|---|---|---|---|
| public observer | none | seat count, roles, statuses, public board, contributions, legal public outcome facts | every seat's unrevealed hole cards and all future hidden setup facts |
| seat 0 | seat 0 hole cards | public facts plus seat 0 private panel | seats 1-5 hole cards and future hidden setup facts |
| seat 1 | seat 1 hole cards | public facts plus seat 1 private panel | seats 0 and 2-5 hole cards and future hidden setup facts |
| seat 2 | seat 2 hole cards | public facts plus seat 2 private panel | seats 0-1 and 3-5 hole cards and future hidden setup facts |
| seat 3 | seat 3 hole cards in 4-6 seat matches | public facts plus seat 3 private panel | other seats' hole cards and future hidden setup facts |
| seat 4 | seat 4 hole cards in 5-6 seat matches | public facts plus seat 4 private panel | other seats' hole cards and future hidden setup facts |
| seat 5 | seat 5 hole cards in 6-seat matches | public facts plus seat 5 private panel | other seats' hole cards and future hidden setup facts |

The seat labels are presentation labels only. Rust/WASM remains the authority for
which seats exist, who may act, and which private projection a viewer receives.

## Pairwise No-Leak Matrix

For every supported seat count, the no-leak expectation is pairwise:

| Scope | Required proof |
|---|---|
| Rust projections | public observer and every seat projection expose only authorized hole cards. |
| action diagnostics | wrong-seat and stale-action diagnostics cite public facts only. |
| semantic effects | private deal and terminal effects are filtered before reaching unauthorized viewers. |
| replay export/import | public exports omit seed-reconstructable hidden state and unauthorized private cards. |
| bots | explanations use own authorized cards, public board texture, call price, live-opponent count, street, and cap pressure only. |
| browser DOM/storage/logs | observer and wrong-seat browser contexts contain no unauthorized card labels, hidden setup identifiers, candidate rankings, or private debug facts. |

`node apps/web/e2e/river-ledger.smoke.mjs` covers observer/wrong-seat browser
no-leak checks, storage and console checks, Rust-only legal controls, terminal
outcome rendering, and responsive layout.

## UI Metadata

Rust `ui.rs` and the WASM catalog provide stable presentation metadata:

| Field | Current value |
|---|---|
| `display_name` | `River Ledger` |
| `game_id` | `river_ledger` |
| `rules_version` | `river-ledger-rules-v1` |
| `min_seats` | 3 |
| `default_seats` | 6 |
| `max_seats` | 6 |
| `viewer_modes` | public observer and seat viewers |
| `primary_resource` | abstract contribution units |
| `hand_rankings` | strongest-to-weakest River Ledger hand-category labels |

The metadata is inert presentation support. It must not encode legality,
selectors, hidden card identities, rule branches, or behavior by naming.

## Legal Action Mapping

| Rust action | Rule IDs | UI control | Accessibility label source | Notes |
|---|---|---|---|---|
| `fold` | `RL-BET-ACTION-001`, `RL-STREET-FOLDOUT-001` | Native action button when Rust exposes it | `choice.accessibility_label` from Rust | May end the hand by foldout. |
| `check` | `RL-BET-ACTION-002`, `RL-BET-CHECK-001` | Native action button when no amount is owed | `choice.accessibility_label` from Rust | TypeScript does not inspect contribution equality. |
| `call` | `RL-BET-ACTION-003`, `RL-BET-CALL-001` | Native action button when Rust exposes it | `choice.accessibility_label` from Rust | Required amount and call-price copy are Rust metadata/payload. |
| `bet` | `RL-BET-ACTION-002`, `RL-BET-LIMIT-001`, `RL-BET-LIMIT-002` | Native action button when opening a street bet is legal | `choice.accessibility_label` from Rust | Unit size and added-ledger copy come from Rust street rules. |
| `raise` | `RL-BET-ACTION-003`, `RL-BET-RAISE-001`, `RL-BET-CAP-001` | Native action button until the cap is reached | `choice.accessibility_label` from Rust | Cap availability and cap-left copy are Rust-owned. |

Action `data-testid` values are stable UI selectors and must not include card
ids, hand strength, hidden setup facts, or private rationale.

## Accessibility And Motion

- Core choices are native buttons, keyboard reachable, and activatable with
  Enter/Space.
- Status and terminal result text are available as screen-reader-readable text,
  not only color or animation.
- Seat roles, active/pending states, street strip, public board slots,
  contributions, private card placeholders, action controls, hand-ranking
  reference, teaching aid, and outcome panels use visible labels.
- Card labels are shown only after Rust authorizes that viewer projection.
- Color is not the only information channel; role labels, counts, headings, and
  card text carry the state.
- Reduced-motion mode preserves all facts through text/status changes and skips
  nonessential motion.
- Responsive layout keeps the seat ledger, board, legal controls, and outcome
  readable at mobile and desktop widths.

## No-Leak Requirements

Before a rule-defined reveal, unauthorized hole-card labels, future community
cards, burn/deck-tail facts, hand-strength facts, candidate rankings, bot
private reasoning, and seed-reconstructable hidden setup data must not appear
in browser payloads, DOM text, attributes, `data-testid` values, local storage,
session storage, console logs, public replay exports, dev-panel text, bot
explanations, diagnostics, or effect logs.

At showdown, Rust may reveal only the private cards and evaluated hands that the
terminal rules authorize. On foldout, folded seats' private cards remain hidden
from public and opposing-seat viewers.

## Outcome / victory explanation

The terminal surface explains River Ledger results from Rust-owned terminal
view data. TypeScript renders the supplied fields only; it must not compare
cards, choose winners, allocate the ledger, decide split remainders, rank hands,
or infer why a hand ended. Rust emits one of these inert template keys:

- `river_ledger.last_live_fold_win`
- `river_ledger.showdown_best_hand_win`
- `river_ledger.showdown_split_pot`

The decisive cause variants are `last_live_after_folds`,
`best_showdown_hand`, and `equal_best_hand_split`; they are rendered as
Rust-authored explanation data, not interpreted by the browser.

For showdown, Rust also projects the human-readable explanation fields the
browser may display: `headline`, `decisive_comparison`, `comparison_basis`,
per-seat `result_label`, `hand_name`, `rank_explanation`, `comparison_note`,
`best_five`, `best_five_accessibility_label`, and terminal-only
`category_ladder_position`. Raw category keys, tie-break vectors, and rule IDs
remain available only in the details tier.

### Terminal Result Variants

| Result variant | Rust source of truth | Player-facing summary | Rule IDs |
|---|---|---|---|
| `foldout` | terminal outcome rationale and per-seat allocation | The last live hand receives the ledger allocation after all other live seats folded; unrevealed folded private cards stay hidden. | `RL-STREET-FOLDOUT-001`, `RL-SHOW-FOLDOUT-001`, `RL-VIS-FOLDOUT-001` |
| `showdown_win` | showdown rationale, evaluated hand summary, decisive comparison, allocation | One showdown-eligible seat has the strongest evaluated five-card hand and receives the ledger allocation. | `RL-STREET-SHOWDOWN-001`, `RL-EVAL-SEVEN-001`, `RL-SHOW-WINNER-001` |
| `split` | showdown rationale, tied hand summaries, allocation and remainder rule | Tied best hands split the ledger; any remainder follows deterministic button order among tied winners. | `RL-SHOW-SPLIT-001`, `RL-POT-REMAINDER-001` |

### Per-Seat Final Breakdown

| Breakdown value | Source | Visible to public observer? | Visible to seat viewer? | Hidden-info notes |
|---|---|---:|---:|---|
| seat result | terminal per-seat result | yes | yes | Winner, loser, folded, or split share. |
| final contribution | public ledger | yes | yes | Abstract public accounting. |
| final allocation | terminal allocation | yes | yes | Ledger allocation is public at terminal. |
| folded status | public seat status | yes | yes | Does not reveal folded private cards. |
| showdown hand name and rank explanation | terminal showdown explanation | showdown only | showdown only | Present only for authorized showdown reveals. |
| used cards and best-five group label | terminal showdown explanation | showdown only | showdown only | Redacted when cards are not authorized for that viewer. |
| decisive comparison and comparison basis | terminal showdown explanation | showdown only | showdown only | Rust-authored; TypeScript only renders it. |
| category ladder position | terminal showdown explanation | showdown only | showdown only | Teaching aid only, labeled as not a game value. |
| raw category key / tie-break vector / rule IDs | terminal showdown explanation | details tier only | details tier only | Retained for inspectability; not the primary player explanation. |

### No-Leak Rules

- Foldout explanations must say the result ended by last live hand and must not
  name unrevealed folded cards.
- Showdown explanations may name only cards Rust has authorized for the viewer.
- `aria-label`, `title`, hidden text, CSS classes, and test IDs must not encode
  unrevealed card ids, future board cards, hand-strength facts, or private bot
  rationale.
- Storage, logs, replay export/import, dev panel, and effect logs must use the
  same viewer-filtered payloads as the visible board.
- Bot explanations must not add opponent hidden-card facts, sampled hidden
  states, or counterfactual showdown analysis.

### Smoke And Tests

| Test case | Terminal path | Required assertion |
|---|---|---|
| foldout no reveal | browser starts a 6-seat human-vs-bot match and folds through legal Rust controls | terminal renders a final outcome and no unauthorized private card appears in observer/wrong-seat DOM, storage, or logs. |
| worked-example showdown | browser drives the audit seed-79 checkdown path to showdown | terminal states that Pair of Queens beats Pair of Eights and keeps unauthorized cards out of DOM, storage, and logs. |
| legal-only controls | browser action buttons come from `actionTree.choices` | TypeScript presents only Rust-supplied legal choices. |
| action metadata copy | browser renders call/raise choices from Rust metadata | call price, added ledger units, and cap-left text match Rust-projected action metadata. |
| N-seat setup | match setup uses catalog seat counts | 3, 4, 5, and 6 are selectable and the default is 6. |
| card and ranking presentation | public board, private cards, best-five groups, and hand-ranking reference render | card labels include rank, suit glyph, suit word, and group labels; hand ladder is available and marks showdown categories. |
| responsive layout | browser smoke checks desktop and mobile viewport widths | seat ledger, board, controls, and terminal text remain visible. |

## Evidence

- `npm --prefix apps/web run build`
- `npm --prefix apps/web run smoke:ui`
- `npm --prefix apps/web run smoke:wasm`
- `node apps/web/e2e/river-ledger.smoke.mjs`
- `node apps/web/e2e/rules-display.smoke.mjs`
- `npm --prefix apps/web run smoke:e2e`

# Gate 20.4 — Starbridge Crossing in-match seat display names

## 1. Header

| Field | Value |
|---|---|
| Spec ID | `gate-20-4-starbridge-crossing-in-match-seat-display-names` |
| Stage / unit | Public scaling phase — Gate 20 presentation/contract follow-on (post-`Done`) |
| Gate | Gate 20.4 (presentation/contract fix on shipped Gate 20 `starbridge_crossing`) |
| Status | `Planned` |
| Date | 2026-06-28 |
| Owner | TBD |
| Authority order | `docs/FOUNDATIONS.md` → `docs/UI-INTERACTION.md` → `docs/MULTI-SEAT-AND-SURFACE-CONTRACT.md` → `docs/ENGINE-GAME-DATA-BOUNDARY.md` → `games/starbridge_crossing/docs/RULES.md` → this spec |

This follows the established post-gate presentation-contract pattern used for
Gate 20.2 (setup-preview seat labels) and Gate 20.3 (terminal outcome
rationale): a shipped official game has an interface-contract gap whose
boundary-correct fix requires a small Rust-owned view-projection addition
consumed by the shared presentation shell, not a TypeScript-only relabel.

## 2. Objective

Make every **in-match** Starbridge Crossing surface name seats by their
display point name (e.g. _North_, _North East_) instead of by raw seat index
(_Seat 1_, _Seat 2_), sourced from a Rust-owned per-seat display label on the
public view. Today the in-match seat name is either title-cased in TypeScript
from the lowercase `home` token (the board, as an interim fix — see §3) or
falls back to `Seat N` (the shared turn-status bar), because the Starbridge
public view projects no viewer-safe per-seat display label in the shape the
shared seat-label resolution already consumes.

`docs/UI-INTERACTION.md` §10B and §19 are explicit: "seat indices are
dev-panel vocabulary; normal public UI uses display names, roles, or team
labels supplied by Rust/static typed metadata," and normal-mode surfaces must
"name … the acting faction in display terms." The setup screen already names
Starbridge seats by point (Gate 20.2). The in-match surfaces must match, from
one Rust-owned source of truth, with no shared component re-deriving the name
from the `home` token.

## 3. Reproduction (observed)

Built web shell (`npm --prefix apps/web run build`), opened **Starbridge
Crossing**, started a 6-seat Hotseat match:

| Surface | Before this spec | Source of defect |
|---|---|---|
| Shared turn-status bar (`ModeControls`) | _"Seat 1 to act"_ | `ModeControls.seatLabelsForView` reads `view.ui.seat_labels`; Starbridge projects none, so `resolveSeatLabel` returns the `Seat N` fallback. |
| Board heading / active-seat status / a11y / legend (`StarbridgeCrossingBoard`) | _"Seat 1 to move"_, legend _"Seat 1 … Seat 6"_ | The board's local `seatLabel()` formatted `Seat ${index+1}` from the raw seat id. |

Interim presentation fix already shipped (commit `0d21913`,
"Name Starbridge seats by home point in board surface"): the
`StarbridgeCrossingBoard` now derives each seat's name by title-casing the
Rust `view.seats[].home` token (`north` → _North_, `north_east` → _North
East_) and uses it in the board heading, active-seat status, screen-reader
summary, per-space accessibility labels, and legend. That commit closes the
board surface but (a) leaves the **shared** `ModeControls` bar still showing
_"Seat 1 to act"_ on the same screen — a worse, inconsistent half-state — and
(b) synthesizes the display string from a view token in TypeScript, which this
spec replaces with a Rust-owned label per the `docs/FOUNDATIONS.md §2`
behavior/data-authority posture.

The same interim `formatPoint` helper
(`apps/web/src/components/StarbridgeCrossingBoard.tsx:476`) is applied to **two**
token sites, not one: the seat **home** name
(`seatNameMap`, line 461) **and** the seat legend's destination point
`to {formatPoint(seat.target)}` (line 290). A Rust label that covers only the
home point therefore cannot replace `formatPoint` outright — removing the helper
would break the legend's target text, and retaining it for `seat.target` would
keep TypeScript synthesizing a display name from the `target` token, which §4
Not-allowed forbids. This spec projects a Rust-owned display label for **both**
the home and the target point so the helper is fully removed (see §4).

Root cause: the Starbridge public-view projection
(`crates/wasm-api/src/games/starbridge_crossing.rs:219`) emits each seat as
`{seat_id, seat_index, home, target, finish_rank}` with `home`/`target` as the
lowercase `StarPoint::label()` tokens (`games/starbridge_crossing/src/ids.rs:99`)
and **no** display label, and exposes no `ui` object (hence no
`ui.seat_labels`) on the view. The shared turn-status bar resolves its name
through `ModeControls.seatLabelsForView`, which reads **only**
`view.ui.seat_labels` (`apps/web/src/components/ModeControls.tsx:189`–`192`).
The live precedent that projects a `SeatDisplayLabel[]` on exactly that path is
**Event Frontier**, whose view `ui` object carries `seat_labels`
(`crates/wasm-api/src/games/event.rs:236`). (River Ledger also projects a
`SeatDisplayLabel[]` of the same shape, but as a **top-level**
`active_seat_labels` field — `apps/web/src/wasm/client.ts:925`–`926`,
`crates/wasm-api/src/games/river.rs:94` — which `seatLabelsForView` does **not**
read; it is a same-shape example, not a same-path precedent.) Starbridge
projects neither, so the shared shell has nothing viewer-safe to present and
falls back to the seat index.

## 4. Scope

**In scope**

- Rust-owned view projection: extend the Starbridge public-view JSON so it
  carries a viewer-safe per-seat **display label** for the active seats, using
  the existing `SeatDisplayLabel[]` shape already projected by other games and
  typed in `apps/web/src/wasm/client.ts:114`. Project it where the shared shell
  already looks — a new `view.ui` object carrying `seat_labels` (the
  `view.ui.seat_labels` field `ModeControls.seatLabelsForView` reads; the
  Starbridge view has no `ui` object today, so this adds one) — matching the
  live `ui.seat_labels` precedent in Event Frontier
  (`crates/wasm-api/src/games/event.rs:236`), **not** River Ledger's top-level
  `active_seat_labels` (a different, unread path). For the board's per-seat
  rendering, also add a `label` (home display name) **and** a `target_label`
  (destination display name) field alongside `home`/`target` on each entry of
  `seats[]` (`crates/wasm-api/src/games/starbridge_crossing.rs:219`); the
  `target_label` lets the board's legend `to {…}` text drop the interim
  `formatPoint(seat.target)` call (`StarbridgeCrossingBoard.tsx:290`) so the
  helper is removed entirely.
- Label **content** and **resolution**: the content is the authored title-case
  point name from the existing catalog ring
  (`catalog_starbridge_seat_labels_json`, the same source as the Gate 20.2 setup
  labels) — **not** re-authored and **not** title-cased from the lowercase token
  in Rust or TypeScript. Because that catalog list is a flat `seat_0`…`seat_5`
  ring ordering, each label MUST be resolved by the seat's **point**, not by a
  flat play-time `seat_id` lookup: `label = ring_labels[seat.home.clockwise_index()]`
  and `target_label = ring_labels[seat.target.clockwise_index()]`
  (`games/starbridge_crossing/src/ids.rs` `clockwise_index`). This is load-bearing
  for the discontinuous configs: for `{2,3,4}` seats `active_points_for_seat_count`
  assigns non-contiguous home points (2-seat = `[North, South]`, so play-time
  `seat_1`'s home is **South**, ring label `"South"` — a flat
  `catalog[seat_1]` lookup would wrongly yield `"North East"`). The projection
  output is keyed by the play-time `seat_id`s (`seat_0`…), so the shell does no
  catalog index remap.
- Web shell consumption:
  - `StarbridgeCrossingBoard` consumes the Rust labels — the seat name via
    `seat.label` and/or the projected `ui.seat_labels`
    (`seatNameMap`, `StarbridgeCrossingBoard.tsx:461`), and the legend
    destination via `seat.target_label` (line 290) — and **removes** the interim
    `formatPoint` helper added in `0d21913` (line 476) entirely, including both
    its `seat.home` and `seat.target` call sites. No `formatPoint`/token
    title-casing remains in the board after this.
  - `ModeControls` resolves the Starbridge active-seat name through its existing
    `seatLabelsForView` → `resolveSeatLabel` path with no game-specific coupling,
    once `view.ui.seat_labels` is present. Confirm the turn-status bar then reads
    _"North to act"_ / _"North turn in progress"_.
  - `ReplayViewer` (which also renders `StarbridgeCrossingBoard`) and the
    terminal `OutcomeExplanationPanel` standings stay consistent with the same
    label source.
- Tests:
  - A wasm-api/view test that the Starbridge public view projects, for each
    active seat and `{2,3,4,6}` seats, a `seat_labels`/`label` equal to the
    authored catalog ring label **at the seat's home-point index**
    (`ring_labels[seat.home.clockwise_index()]`) and a `target_label` equal to
    the ring label at the seat's target-point index. The expected value MUST be
    computed via the home/target point index, **not** a flat `catalog[seat_id]`
    echo — a flat comparison is tautological with the flat-lookup bug and would
    pass green while mis-labelling the discontinuous `{2,3,4}` configs. Assert
    the discontinuity explicitly (e.g. 2-seat `seat_1` label == `"South"`,
    3-seat `seat_1` label == `"South East"`).
  - Extend `apps/web/e2e/starbridge-crossing.smoke.mjs` so the existing
    `assertSeatDisplayNames` covers **both** the board legend/heading **and** the
    shared `ModeControls` turn-status bar (assert it reads a point name, never
    `Seat N`), for a multi-seat match.

**Out of scope**

- Any movement, finish, terminal-result, visibility, bot, or legality change.
- New variants, seat counts, piece counts, or ring-label renaming.
- Changing the setup-preview seat labels (Gate 20.2 owns those).
- Generalizing the seat-label projection across games beyond what Starbridge
  needs; other games already project their own labels.

**Not allowed**

- TypeScript (or any shared component) deciding or synthesizing the seat display
  name from the `home`/`target` token or the seat index (`SC-UI-001`;
  `docs/UI-INTERACTION.md` §3, §19).
- Game-specific (`starbridge_crossing`) branching inside shared shell components
  (`ModeControls`, `seatLabels.ts`).
- Behavior language (formulas/selectors/conditions) in catalog static data
  (`docs/ENGINE-GAME-DATA-BOUNDARY.md`).
- Any `engine-core`/`game-stdlib` seat-ring/topology noun to express this; the
  mapping stays game-local in `games/starbridge_crossing` and is surfaced through
  the existing view/catalog plumbing.
- Exposing any hidden state — Starbridge is fully public; the label is a public
  point name. No new redaction surface.

## 5. Work breakdown (indicative; final ticket split at decomposition)

1. **Rust view projection** — add the viewer-safe per-seat display labels to the
   Starbridge public view: a new `view.ui.seat_labels` and `seats[].label`
   (home name) plus `seats[].target_label` (destination name), each resolved
   from the catalog ring labels by the seat's point index
   (`ring_labels[seat.home.clockwise_index()]` /
   `[seat.target.clockwise_index()]`), keyed by play-time `seat_id`; refresh the
   additive `crates/wasm-api/tests/snapshots/api_surface.tsv` public-view
   snapshot and add the view-label regression test (with the explicit
   discontinuity assertions for `{2,3,4}` seats).
2. **Web consumption** — update `StarbridgeCrossingBoard` to consume the Rust
   `label`/`ui.seat_labels` (seat name) and `target_label` (legend destination)
   and remove the interim `formatPoint` helper from `0d21913` entirely (both the
   `seat.home` and `seat.target` sites); verify `ModeControls` resolves the name
   through the existing shared `view.ui.seat_labels` path; extend the Starbridge
   browser smoke to cover the turn-status bar.
3. **Evidence and closeout** — update `games/starbridge_crossing/docs/UI.md`,
   `games/starbridge_crossing/docs/GAME-EVIDENCE.md`, `specs/README.md`, and this
   spec's status after the code/web tickets pass.

## 6. Exit criteria

- Every in-match Starbridge surface (board heading, active-seat status,
  screen-reader summary, per-space accessibility labels, seat legend including
  its `to {…}` destination point, the shared `ModeControls` turn-status bar,
  replay viewer, and terminal standings) names seats by point, with **no**
  `Seat N` index visible in normal mode and **no** TypeScript deriving the name
  from the `home`/`target` token or seat index.
- The seat label (and the legend's target label) is supplied by the Rust public
  view and matches the authored catalog ring labels resolved per the seat's
  home/target point index, agreeing with the Gate 20.2 setup-preview labels for
  `{2,3,4,6}` seats — including the discontinuous `{2,3,4}` configs.
- The board's interim `formatPoint` helper (commit `0d21913`) is removed
  entirely — both its `seat.home` and `seat.target` call sites — in favor of the
  Rust labels.
- CI gate 0 (`fmt`, `clippy -D warnings`, `build`, `test`) and gate 1
  (`simulate`, `replay-check --all`, `fixture-check`, `rule-coverage`,
  `boundary-check.sh`, `check-doc-links.mjs`, `check-catalog-docs.mjs`, web
  smokes/build) pass with only the single additive public-view snapshot diff.

## 7. FOUNDATIONS & boundary alignment

- **Behavior/data authority (`docs/FOUNDATIONS.md §2`; `SC-UI-001`)** — the seat
  display name is Rust-owned view metadata; the shell presents it, never
  synthesizes it. This removes the interim TypeScript token formatting rather
  than entrenching it.
- **`docs/UI-INTERACTION.md` §10B / §19** — normal-mode surfaces name the acting
  faction in display terms; seat indices are dev-panel vocabulary. This fix
  brings the in-match surfaces into line with that rule and with the setup
  screen.
- **`docs/MULTI-SEAT-AND-SURFACE-CONTRACT.md`** — seat labels for shared surfaces
  come from Rust/static typed metadata; the shared seat frame and turn surfaces
  consume them.
- **No-leak (`docs/FOUNDATIONS.md §11`)** — Starbridge is fully public; the label
  is a public point name. No hidden state, no new redaction class.
- **Determinism** — additive public-view projection only; no accepted command,
  state, effect, trace, replay, or hash change (cf. Gate 20.3 adding
  `terminal_rationale` with no hash change). The catalog/view snapshot carries
  the single expected additive diff. No ADR trigger (no kernel/DSL/YAML/
  trace-hash/visibility/architecture change); flag for reviewer if the view-shape
  addition is judged to need one.

## 8. Forbidden changes

- No new pass option, variant, seat count, or piece count.
- No TypeScript legality, TypeScript seat-name synthesis, or game-specific
  branching in shared shell components.
- No ring-label renaming; no movement/finish/terminal/visibility change.
- No `engine-core`/`game-stdlib` seat-ring/topology noun.

## 9. Documentation updates required

- `games/starbridge_crossing/docs/UI.md` — record that in-match seat naming is
  sourced from the Rust-projected per-seat display label (and remove/adjust the
  "Seat 1 north to south" legend phrasing now superseded).
- `games/starbridge_crossing/docs/GAME-EVIDENCE.md` — fix receipt.
- `specs/README.md` — add the Gate 20.4 tracker row; flip to `Done` at closeout.

## 10. Assumptions (one-line-correctable)

- A1: The authored catalog ring labels (`catalog_starbridge_seat_labels_json`,
  the Gate 20.2 source) are the single ground-truth display labels; the view
  projection resolves them by **point index** —
  `ring_labels[seat.home.clockwise_index()]` for the seat name and
  `ring_labels[seat.target.clockwise_index()]` for the legend destination — not
  by a flat play-time `seat_id` lookup (which mis-labels the discontinuous
  `{2,3,4}` configs) and not re-authored or title-cased from the token.
- A4: The shared turn-status bar resolves names only through
  `view.ui.seat_labels` (`ModeControls.seatLabelsForView`); Event Frontier
  (`event.rs:236`) is the live precedent for that path. River Ledger's top-level
  `active_seat_labels` is the same shape on a different, unread path and is not
  the projection target here.
- A2: The defect is presentation/projection-only; no accepted command stream,
  state, effect, or hash changes, so no determinism migration is required beyond
  the additive public-view snapshot diff.
- A3: `ModeControls.seatLabelsForView` reading `view.ui.seat_labels` is the
  intended shared mechanism; projecting that field for Starbridge fixes the
  turn-status bar with no shared-component change.

## 11. Sequencing

- Predecessor: Gate 20 (`Done`), Gate 20.1/20.2/20.3 (`Done`), and the interim
  board fix `0d21913` (on `main`).
- Successor: does not block Gate 21; an independent presentation/contract
  follow-on on the shipped Gate 20 game, executable any time before public
  release (Gate 20 closeout already notes IP/public-release review pending).
</content>

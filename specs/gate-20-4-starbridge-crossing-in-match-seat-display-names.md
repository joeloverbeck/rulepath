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

Root cause: the Starbridge public-view projection
(`crates/wasm-api/src/games/starbridge_crossing.rs:219`) emits each seat as
`{seat_id, seat_index, home, target, finish_rank}` with `home`/`target` as the
lowercase `StarPoint::label()` tokens (`games/starbridge_crossing/src/ids.rs:99`)
and **no** display label, and exposes no `ui.seat_labels` / `active_seat_labels`
on the view. Other multi-seat games project a `SeatDisplayLabel[]` the shared
shell consumes (e.g. River Ledger `active_seat_labels`,
`apps/web/src/wasm/client.ts:925`–`926`; the `view.ui.seat_labels` path read by
`apps/web/src/components/ModeControls.tsx:189`–`192`). Starbridge bypasses that,
so the shared shell has nothing viewer-safe to present and falls back to the
seat index.

## 4. Scope

**In scope**

- Rust-owned view projection: extend the Starbridge public-view JSON so it
  carries a viewer-safe per-seat **display label** for the active seats, using
  the existing `SeatDisplayLabel[]` shape already projected by other games and
  typed in `apps/web/src/wasm/client.ts:114`. Project it where the shared shell
  already looks — `view.ui.seat_labels` (the field
  `ModeControls.seatLabelsForView` reads) — and, for the board's per-seat
  rendering convenience, optionally also add a `label` field alongside
  `home`/`target` on each entry of `seats[]`
  (`crates/wasm-api/src/games/starbridge_crossing.rs:219`). The label **content**
  is the authored title-case point name from the existing catalog ring
  (`catalog_starbridge_seat_labels_json`, the same source as the Gate 20.2 setup
  labels), resolved for each seat's home `StarPoint` — **not** re-authored and
  **not** title-cased from the lowercase token in Rust or TypeScript. Keys are
  the play-time `seat_id`s (`seat_0`…), so the discontinuous active-point
  mapping is already resolved by Rust (no catalog index remap in the shell).
- Web shell consumption:
  - `StarbridgeCrossingBoard` consumes the Rust label (via `seat.label` and/or
    the projected `ui.seat_labels`) and **removes** the interim
    `formatPoint(seat.home)` title-casing and the local point formatter added in
    `0d21913`.
  - `ModeControls` resolves the Starbridge active-seat name through its existing
    `seatLabelsForView` → `resolveSeatLabel` path with no game-specific coupling,
    once `view.ui.seat_labels` is present. Confirm the turn-status bar then reads
    _"North to act"_ / _"North turn in progress"_.
  - `ReplayViewer` (which also renders `StarbridgeCrossingBoard`) and the
    terminal `OutcomeExplanationPanel` standings stay consistent with the same
    label source.
- Tests:
  - A wasm-api/view test that the Starbridge public view projects the seat
    display labels equal to the catalog ring label for each active seat's home
    point, for `{2,3,4,6}` seats (compare against the authored catalog labels,
    not a re-derivation).
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

1. **Rust view projection** — add the viewer-safe per-seat display label to the
   Starbridge public view (`ui.seat_labels` and optional `seats[].label`),
   sourced from the catalog ring labels resolved per active seat's home point;
   refresh the additive `crates/wasm-api/tests/snapshots/api_surface.tsv`
   public-view snapshot and add the view-label regression test.
2. **Web consumption** — update `StarbridgeCrossingBoard` to consume the Rust
   label and drop the interim title-casing from `0d21913`; verify `ModeControls`
   resolves the name through the existing shared path; extend the Starbridge
   browser smoke to cover the turn-status bar.
3. **Evidence and closeout** — update `games/starbridge_crossing/docs/UI.md`,
   `games/starbridge_crossing/docs/GAME-EVIDENCE.md`, `specs/README.md`, and this
   spec's status after the code/web tickets pass.

## 6. Exit criteria

- Every in-match Starbridge surface (board heading, active-seat status,
  screen-reader summary, per-space accessibility labels, seat legend, the shared
  `ModeControls` turn-status bar, replay viewer, and terminal standings) names
  seats by point, with **no** `Seat N` index visible in normal mode and **no**
  TypeScript deriving the name from the `home` token or seat index.
- The seat label is supplied by the Rust public view and matches the authored
  catalog ring labels and the Gate 20.2 setup-preview labels for `{2,3,4,6}`
  seats.
- The board's interim `formatPoint`/`home` title-casing (commit `0d21913`) is
  removed in favor of the Rust label.
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
  projection resolves them per active seat's home `StarPoint`, not re-authored.
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

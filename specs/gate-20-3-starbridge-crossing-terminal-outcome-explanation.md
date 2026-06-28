# Gate 20.3 — Starbridge Crossing terminal outcome explanation

## 1. Header

| Field | Value |
|---|---|
| Spec ID | `gate-20-3-starbridge-crossing-terminal-outcome-explanation` |
| Stage / unit | Public scaling phase — Gate 20 correctness follow-on (post-`Done`) |
| Gate | Gate 20.3 (presentation/contract fix on shipped Gate 20 `starbridge_crossing`) |
| Status | `Planned` |
| Date | 2026-06-28 |
| Owner | TBD |
| Authority order | `docs/FOUNDATIONS.md` → `docs/MULTI-SEAT-AND-SURFACE-CONTRACT.md` → `docs/ENGINE-GAME-DATA-BOUNDARY.md` → `games/starbridge_crossing/docs/RULES.md` → `games/starbridge_crossing/docs/UI.md` → this spec |

This follows the established post-gate correctness-spec pattern used for
Gate 20.1 and Gate 20.2: a shipped official game has a defect that is not a
bounded ticket-only change because the boundary-correct fix requires a small
Rust-owned public-view contract addition (a terminal outcome rationale) consumed
by the presentation shell.

## 2. Objective

Make a finished Starbridge Crossing match present the **Rust-authored terminal
outcome explanation** that `games/starbridge_crossing/docs/UI.md` already
specifies (the "Outcome / victory explanation" section) and that
`games/starbridge_crossing/docs/RULES.md` `SC-FINISH-004` / `SC-FINISH-006`
already require. Today:

1. The Rust public view (`games/starbridge_crossing/src/visibility.rs::StarbridgePublicView`)
   projects only a flat `terminal` reason string (`"complete"` or
   `"turn_limit:2000"`) plus `finish_ranks` / per-seat `finish_rank`. It does
   **not** project a structured outcome rationale (decisive cause, decisive rule
   IDs, per-seat terminal standings with the rank-1 winner flag, or the
   turn-limit progress-vector facts).
2. The wasm-api projection
   (`crates/wasm-api/src/games/starbridge_crossing.rs`) therefore serializes no
   `terminal_rationale` field, even though the web bridge type
   (`apps/web/src/wasm/client.ts::StarbridgeCrossingPublicView.terminal_rationale?`)
   declares an optional slot for it that is never populated.
3. `apps/web/src/components/StarbridgeCrossingBoard.tsx` is the **only** game
   board of the 20 catalog games that never renders `OutcomeExplanationPanel`.
   At terminal it shows a raw heading — `terminalLabel(view)` =
   `view.terminal.replaceAll("_", " ")` → `"complete"` or `"turn limit:2000"` —
   and the seat legend's `, rank N` suffix, with no decisive-cause copy, no
   structured per-seat standings panel, and no `aria-live` outcome announcement.

The goal is to close the gap so that the documented outcome surface is delivered
end to end, Rust-owned and TypeScript-presented, for **both** terminal variants.

## 3. Reproduction (observed)

Built web shell (`npm --prefix apps/web run build`), served `dist/`, drove it
with Puppeteer:

- Started a 2-seat **Bot vs bot** Starbridge match (seed 1) and ran the match to
  terminal via the bot-step control. The match reached
  `view.status = "turn_limit:2000"` (random L0 play never completes a Halma race;
  `simulate -- --game starbridge_crossing --games 50` confirms all games reach the
  deterministic 2000-ply turn limit, `capped_matches=0`).
- At terminal the DOM contained:
  - board heading `#starbridge-heading` = `"turn limit:2000"` (raw, machine-y);
  - `[data-testid="turn"]` = `"Complete"`;
  - seat legend = `["A Seat 1 north to south, rank 1", "B Seat 2 south to north, rank 2"]`;
  - **no** outcome-explanation panel: no `[data-testid]` matching
    `outcome|result|explanation`, no decisive-cause / "Why" / finish-order copy,
    and no `aria-live` outcome announcement.
- By contrast, every other terminal game board renders `OutcomeExplanationPanel`
  (`grep -rln OutcomeExplanationPanel apps/web/src/components` lists 19 boards;
  `StarbridgeCrossingBoard.tsx` is absent).

Root cause: the Starbridge terminal outcome rationale is never produced by Rust
and never rendered by the shell. `scripts/check-outcome-explanations.mjs` passes
only because it validates **inert contract surfaces** (the UI.md section, the
RULES.md scoring/end IDs, the `client.ts` optional type slot, and the
`outcomeExplanationTemplates.ts` key) and explicitly "does not read match state …
views … or replay data", so it cannot catch that the rationale is unpopulated and
the panel unrendered.

## 4. Scope

**In scope**

- **Rust-owned outcome rationale (game-local).** Add a game-local
  `StarbridgeOutcomeRationaleView` (mirroring the per-game pattern of
  `games/river_ledger/src/visibility.rs::OutcomeRationaleView`) and project it on
  `StarbridgePublicView` as `terminal_rationale: Option<StarbridgeOutcomeRationaleView>`,
  populated by `project_view` **only when the match is terminal**. It is derived
  entirely from existing public terminal state (`state.terminal_status`,
  `state.finish_ranks`, the seat ring, and the same progress-vector accounting
  used by `rules.rs::assign_turn_limit_ranks` / `progress_score`); it introduces
  no new private or hidden facts. Fields, sourced from Rust authority only:
  - `result_kind` — e.g. `"finish_order"` (terminal `Complete`) vs
    `"turn_limit"` (terminal `TurnLimit`).
  - `decisive_cause` — `"finish_order_complete"` (`SC-END-001` / `SC-FINISH-003`)
    or `"turn_limit_progress_vector"` (`SC-END-002` / `SC-FINISH-006`), matching
    the two `UI.md` "Decisive cause variants".
  - `template_key` — `"starbridge_crossing.finish_order_complete"` or
    `"starbridge_crossing.turn_limit_progress_vector"`.
  - `decisive_rule_ids` — the stable `SC-FINISH-*` / `SC-END-*` IDs for the cause
    (e.g. `SC-FINISH-001..004` for finish order; `SC-FINISH-005..006` for turn
    limit), per `UI.md` "RULES.md rule IDs".
  - `final_standing` — seat-keyed, **stable in seat-ring order** (`SC-FINISH-004`):
    each entry carries the public seat label source (seat id / `seat_index`),
    `finish_rank`, a `winner` flag for rank 1, the finished flag, and — for the
    turn-limit cause — the public progress-vector count
    (`SC-FINISH-006`). No raw `seat_<n>` string is placed in human-facing copy
    (the templates file forbids it; see `check-outcome-explanations.mjs`
    `FORBIDDEN_TEMPLATE_PATTERNS`).
- **Determinism preservation.** The rationale is a *projection of existing public
  state*, not new state. It must **not** enter `StarbridgePublicView::stable_summary`
  / `stable_bytes`, so replay hashes, golden traces, setup fixtures, and benchmark
  thresholds are unchanged (additive view field only, like Gate 20.2's additive
  catalog field). Confirm no `replay-check`, `fixture-check`, or bench threshold
  diff.
- **wasm-api serialization.** Serialize the new `terminal_rationale` field in
  `crates/wasm-api/src/games/starbridge_crossing.rs` (the inline JSON `format!`
  at the projection site), emitting `null` when non-terminal and the rationale
  object when terminal — reusing the existing river projection's rationale JSON
  shape conventions (`crates/wasm-api/src/games/river.rs::river_rationale_json`)
  rather than inventing a divergent shape. Refresh the additive wasm-api snapshot
  `crates/wasm-api/tests/snapshots/api_surface.tsv` if (and only if) the view
  shape appears there; the single expected diff is the added field.
- **Web template key.** Add the missing
  `"starbridge_crossing.turn_limit_progress_vector"` key to
  `apps/web/src/components/outcomeExplanationTemplates.ts` (the
  `finish_order_complete` key already exists), static copy only — no logic,
  selectors, or raw seat ids.
- **Web rendering.** Make `StarbridgeCrossingBoard.tsx` build the panel via
  `outcomeSurfaceData({ gameId: "starbridge_crossing", … , rationale:
  view.terminal_rationale ?? null, … })` and render `OutcomeExplanationPanel`
  plus the `aria-live` `outcomeAnnouncementText` mirror, following the established
  board pattern (e.g. `BlackglassPactBoard.tsx:93`–`120`, `:290`–`296`). The
  panel's `finalStanding` is sourced from the Rust-projected `final_standing`
  (TypeScript renders Rust-authored standings only — `SC-FINISH-004`), not
  recomputed from `seats`/`finish_ranks` in TS.
- **Tests (failing-first):**
  - Rust: `games/starbridge_crossing` unit tests that `project_view` emits a
    `terminal_rationale` with the correct `decisive_cause` / `template_key` /
    seat-ring-ordered `final_standing` / rank-1 `winner` flag for (a) an
    all-but-one-finished `Complete` terminal and (b) a `TurnLimit` terminal, and
    `None` while the match is live.
  - Rust: a `stable_summary` regression asserting the rationale does **not**
    change `stable_bytes` (determinism guard).
  - wasm-api: a projection test that the serialized JSON carries
    `terminal_rationale: null` mid-match and the populated object at terminal.
  - Web e2e: extend `apps/web/e2e/starbridge-crossing.smoke.mjs` to drive a
    bot-vs-bot match to terminal and assert the `OutcomeExplanationPanel`
    renders with finish-order/turn-limit decisive copy and a per-seat standing
    for each seat, and that the no-leak scan still passes on the terminal surface.

**Out of scope**

- Any movement, step/hop legality, finish-rank assignment, turn-limit ranking, or
  active-seat order change — the *behavior* is already correct and stays
  byte-for-byte identical; this spec only *projects and presents* its explanation.
- New variants, seat counts, piece counts, or a new pass option.
- Re-labelling seats or changing the six-point ring names; the in-match generic
  `Seat N` legend label and `north to south` subtitle are unchanged.
- Any private/hidden-information surface — Starbridge is fully public; the
  rationale adds only public facts.

**Not allowed**

- TypeScript deciding the decisive cause, the rule IDs, the winner, or the
  standings order for the outcome (`SC-FINISH-004` "TypeScript renders
  Rust-authored standings only"; `SC-UI-001`;
  `docs/MULTI-SEAT-AND-SURFACE-CONTRACT.md`). TS may only map Rust-provided
  rationale fields to copy and DOM.
- Behavior language (formulas/selectors/conditions) or raw `seat_<n>` ids in the
  static templates file (`check-outcome-explanations.mjs` `FORBIDDEN_TEMPLATE_PATTERNS`).
- Any change to replay/hash/fixture/trace determinism artifacts; the rationale is
  an additive view projection excluded from `stable_bytes`.
- Any `engine-core`/`game-stdlib` outcome/standings/topology noun to express
  this; the rationale type stays game-local in `games/starbridge_crossing`.

## 5. Work breakdown

1. **GAT203STACROOUT-001** — Rust outcome rationale: add the game-local
   `StarbridgeOutcomeRationaleView`, project it on `StarbridgePublicView` from
   terminal state only, keep it out of `stable_bytes`, and add the unit +
   determinism-guard tests.
2. **GAT203STACROOUT-002** — wasm-api serialization: emit `terminal_rationale`
   in the Starbridge projection JSON (river-shaped), refresh the additive
   `api_surface.tsv` snapshot if affected, and add the projection test.
3. **GAT203STACROOUT-003** — Web rendering: add the
   `turn_limit_progress_vector` template key, render `OutcomeExplanationPanel` +
   `aria-live` mirror in `StarbridgeCrossingBoard.tsx` from
   `view.terminal_rationale`, and extend the browser smoke to a terminal match.
4. **GAT203STACROOUT-004** — Evidence and closeout: update
   `games/starbridge_crossing/docs/UI.md`,
   `games/starbridge_crossing/docs/GAME-EVIDENCE.md`,
   `games/starbridge_crossing/docs/RULE-COVERAGE.md` (if it tracks the
   outcome-explanation surface), `specs/README.md`, and this spec's status.

Dependency order: 001 → 002 → 003 → 004.

## 6. Exit criteria

- A finished Starbridge match (both `finish_order_complete` and
  `turn_limit_progress_vector` causes) renders `OutcomeExplanationPanel` with the
  Rust-authored decisive cause, decisive rule IDs, and seat-ring-ordered per-seat
  standings (rank-1 winner flagged), plus an `aria-live` announcement — matching
  the `UI.md` "Outcome / victory explanation" contract and `SC-FINISH-004` /
  `SC-FINISH-006`.
- No TypeScript code derives the decisive cause, rule IDs, winner, or standings
  order; the shell renders Rust-provided rationale fields only.
- Determinism artifacts unchanged: `replay-check --all`, `fixture-check`,
  `rule-coverage`, and benchmark thresholds show no diff; the only Rust snapshot
  diff (if any) is the single additive `terminal_rationale` field in
  `api_surface.tsv`.
- CI gate 0 (`fmt`, `clippy -D warnings`, `build`, `test`) and gate 1
  (`simulate`, `replay-check --all`, `fixture-check`, `rule-coverage`,
  `boundary-check.sh`, `check-doc-links.mjs`, `check-outcome-explanations.mjs`,
  web smokes/build) pass.

## 7. Acceptance evidence (to be filled at closeout)

- Failing-first transcript for the Rust rationale projection test and the web
  terminal-panel smoke.
- `cargo test -p starbridge_crossing`, `cargo test -p wasm-api`,
  `cargo test --workspace`.
- `cargo run -p replay-check -- --game starbridge_crossing --all`,
  `cargo run -p fixture-check -- --game starbridge_crossing`,
  `cargo run -p rule-coverage -- --game starbridge_crossing`,
  `bash scripts/boundary-check.sh`.
- `npm --prefix apps/web run build`,
  `node apps/web/e2e/starbridge-crossing.smoke.mjs`,
  `npm --prefix apps/web run smoke:e2e`,
  `node scripts/check-outcome-explanations.mjs`.
- Manual Puppeteer recheck: finished match shows the outcome-explanation panel
  with both decisive causes exercised.

## 8. FOUNDATIONS & boundary alignment

- **Behavior authority / `SC-FINISH-004` / `SC-FINISH-006`** — the decisive
  outcome cause, rule IDs, winner, and terminal standings are Rust-owned public
  facts; the shell must present them, not recompute them. This fix makes the Rust
  view carry them and the shell render them.
- **§12 stop condition** — leaving the shell to synthesize an outcome explanation
  from `finish_ranks` (including inventing which decisive cause applies and a
  turn-limit template) would be a `docs/FOUNDATIONS.md §12` "TypeScript decides …
  behavior" crossing. Routing the rationale through Rust avoids introducing that
  crossing; the raw-string status quo under-delivers the documented contract.
- **`docs/MULTI-SEAT-AND-SURFACE-CONTRACT.md`** — multi-seat terminal standings
  are Rust-authored and seat-keyed; the browser presents them.
- **engine-game-data boundary** — the rationale is typed identity/standings data
  projected from Rust terminal state, never a static-data selector; the type
  stays game-local in `games/starbridge_crossing`.
- **Determinism** — additive view projection excluded from `stable_bytes`; no
  accepted-command, state, effect, replay-hash, fixture, or bench change. No ADR
  trigger (no kernel/DSL/YAML/trace-hash/visibility-class/architecture change);
  flag for reviewer if adding a projected view field is judged to need one.

## 9. Forbidden changes

- No new pass option, variant, seat count, or piece count.
- No movement/finish/turn-limit *behavior* change; explanation projection only.
- No TypeScript legality or TypeScript outcome/standings derivation.
- No ring-label renaming; no change to the in-match generic seat legend label.
- No `engine-core`/`game-stdlib` outcome/standings/topology noun.
- No new private/hidden field; the all-public audit stance is preserved.

## 10. Documentation updates required

- `games/starbridge_crossing/docs/UI.md` — note that the terminal outcome
  explanation is now projected by Rust (`terminal_rationale`) and rendered via
  `OutcomeExplanationPanel`, naming both decisive causes.
- `games/starbridge_crossing/docs/GAME-EVIDENCE.md` — fix receipt / outcome-
  explanation row.
- `games/starbridge_crossing/docs/RULE-COVERAGE.md` — if it tracks the
  outcome-explanation surface, link the new tests.
- `specs/README.md` — add the Gate 20.3 tracker row and flip to `Done` at
  closeout.
- Web-shell catalog docs: confirm no renderer-list/smoke-list membership change
  (game already listed); only the terminal-surface rendering changes.

## 11. Sequencing

- Predecessor: Gate 20 (`Done`), Gate 20.1 (`Done`), Gate 20.2 (`Done`).
- Successor: does not block Gate 21; an independent correctness/contract
  follow-on on the shipped Gate 20 game, executable any time before public
  release (Gate 20 closeout already notes IP/public-release review pending).

## 12. Assumptions (one-line-correctable)

- A1: The two terminal variants are exactly `TerminalStatus::Complete`
  (decisive cause `finish_order_complete`) and `TerminalStatus::TurnLimit`
  (decisive cause `turn_limit_progress_vector`), per
  `games/starbridge_crossing/src/state.rs` and `RULES.md` `SC-END-001/002`.
- A2: The turn-limit progress count is the same `progress_score` (pegs on target
  home) used by `rules.rs::assign_turn_limit_ranks`; the rationale reuses it,
  not a re-authored metric.
- A3: The rationale is derived from already-public terminal state, so it is
  excluded from `stable_bytes` and changes no determinism artifact (additive
  view field only, like Gate 20.2's additive catalog field).
- A4: `OutcomeExplanationPanel` / `outcomeSurfaceData` tolerate a null Rust
  rationale (as `BlackglassPactBoard` passes `rationale: view.outcome_rationale
  ?? null`), so the web change degrades safely if the field is absent, but the
  populated Rust path is the contract target.

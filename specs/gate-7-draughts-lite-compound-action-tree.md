# Gate 7 — Draughts Lite Compound Action Tree

**Spec ID:** gate-7-draughts-lite-compound-action-tree
**Roadmap stage:** 5
**Roadmap build gate:** Gate 7 (`draughts_lite`)
**Status:** Planned
**Date:** 2026-06-07
**Owner:** joeloverbeck
**Authority order:** `docs/FOUNDATIONS.md` → `docs/ROADMAP.md` → `docs/OFFICIAL-GAME-CONTRACT.md` → `docs/ARCHITECTURE.md` → `docs/ENGINE-GAME-DATA-BOUNDARY.md` → `docs/WASM-CLIENT-BOUNDARY.md` → `docs/MECHANIC-ATLAS.md` → this spec

> **Reader orientation.** The canonical decomposition sections (**Objective** through
> **Assumptions**, immediately below) are authoritative for AGENT-TASK
> decomposition. The detailed requirements live in the numbered **Reference
> detail** sections (R2–R26) that follow; they are the original mission-style
> brief, preserved as the depth reference. (Reference section R1 — an
> exact-commit fetch ledger — was removed as generation-harness scaffolding; its
> genuine external sources are recorded under **Sources** below and are required
> again in the game's `SOURCES.md`.)

## Objective

Admit **Draughts Lite** (`draughts_lite`) as a public-polish official game and use
it to prove Rulepath's **compound action tree** model: a multi-step,
origin/destination, mandatory-capture, forced-continuation move presented and
validated entirely by Rust/WASM, committed as **one** deterministic replay
command with a multi-segment action path, with no legality logic in TypeScript.
Depth: §R2, §R3.

## Scope

**In scope** (see §R6 for the authoritative list):

1. New official game crate `games/draughts_lite`.
2. Workspace registration and tool/CI integration.
3. Complete public Draughts Lite rules in Rust (§R8).
4. Nested action-tree generation: origin → quiet/jump landing → forced continuation → promotion-terminated capture (§R9).
5. Deterministic validation and atomic state mutation from complete multi-segment paths (§R11).
6. Trace/replay/golden-fixture coverage (§R10, §R18).
7. WASM bridge support and web shell/game-picker registration (§R13, §R14).
8. Public-polish web board: pointer + keyboard, accessible status, legal-path guidance, animation/effect feedback, reduced motion (§R14–§R16).
9. Level 0 and modest Level 1 bots (§R17).
10. Native benchmarks and WASM/web smoke/perf expectations (§R19).
11. Official-game admission doc package from templates (§R20).
12. A reopen-and-decide board-space primitive decision + ledger update (§R12).

**Out of scope** (see §R7): international/variant draughts, flying kings, huffing,
maximum-capture legality, tournament adjudication (clocks/repetition/agreement
draws/resignation/ratings), network multiplayer, hidden information, and any
search/learning bot machinery.

**Not allowed** (carries ROADMAP §9 Gate 7 "Not allowed", with the spec's stricter
exclusions; see §R7, §R22): full chess exception load; generic movement in
`engine-core`; search without benchmarks (the spec excludes search entirely);
board/capture/promotion/movement nouns in `engine-core`; TypeScript legality;
forced retrofit of existing games to a new primitive; rewriting the
action/replay architecture beyond the smallest multi-segment changes.

## Deliverables

| ID | Deliverable | Primary targets | Depth |
|----|-------------|-----------------|-------|
| D1 | `games/draughts_lite` crate | `actions`, `rules`, `effects`, `state`, `setup`, `bots`, `visibility`, `variants`, `replay_support`, `ids`, `ui` modules; `data/manifest.toml`, `data/variants.toml`, `data/fixtures/` | §R8, §R11 |
| D2 | Workspace + tool + CI registration | root `Cargo.toml` members; `tools/{simulate,replay-check,fixture-check,rule-coverage,bench-report}`; `.github/workflows/gate-0/1/2` | §R23 |
| D3 | `wasm-api` registration + multi-segment replay export | `crates/wasm-api/src/lib.rs` (one-segment guard ~L132–138; `parse_action_path` ~L1231) | §R13 |
| D4 | Web shell + Draughts Lite board component | `apps/web/src/components/*`, game picker, dev panel/replay multi-segment rendering, `apps/web/src/wasm/client.ts` | §R14 |
| D5 | Accessibility + reduced motion + E2E | grid/keyboard pattern, live region; `apps/web/e2e/NO-LEAK-A11Y-CHECKLIST.md` + `smoke:e2e` | §R15, §R16, §R21 |
| D6 | Level 0 + Level 1 bots | `games/draughts_lite/src/bots.rs`, `ai-core` recursive random reuse | §R17 |
| D7 | Fixtures, golden traces, rule + property tests, tool updates | 18 golden traces; `tools/rule-coverage`, `tools/simulate` | §R18 |
| D8 | Native benchmarks + calibrated thresholds | `games/draughts_lite/benches/`, `thresholds.json`, `BENCHMARKS.md` | §R19 |
| D9 | Official-game doc package | `games/draughts_lite/docs/*` (12 docs from `templates/*`) | §R20 |
| D10 | Board-space primitive reopen decision | `crates/game-stdlib`; `PRIMITIVE-PRESSURE-LEDGER.md`; supersede `docs/MECHANIC-ATLAS.md` rows | §R12 |

## Work breakdown

Bounded, dependency-ordered AGENT-TASK candidates (decompose from
`templates/AGENT-TASK.md`):

| WB | Task | Depends on |
|----|------|-----------|
| WB1 | Crate scaffold, state, setup, ids, variants, manifest/fixture (D1) | — |
| WB2 | Board-space primitive reopen decision + (if promoted) `game-stdlib` helper with tests (D10) | WB1 |
| WB3 | Rules: legal generation, mandatory capture/continuation, promotion, terminal detection (D1) | WB1, WB2 |
| WB4 | Action-tree generation + multi-segment validation + atomic apply + effects (D1) | WB3 |
| WB5 | Visibility/perfect-info projection + serialization (D1) | WB3 |
| WB6 | Golden traces, rule tests, property tests; `rule-coverage` + `simulate` updates (D7) | WB4, WB5 |
| WB7 | Level 0 + Level 1 bots + bot tests/traces (D6) | WB4 |
| WB8 | `wasm-api` registration + multi-segment replay export/import (D3) | WB4, WB5 |
| WB9 | Web board, input model, dev panel/replay rendering (D4) | WB8 |
| WB10 | Accessibility, reduced motion, E2E/a11y checklist (D5) | WB9 |
| WB11 | Native benchmarks + threshold calibration (D8) | WB4, WB7 |
| WB12 | Doc package + cross-repo doc updates (D9) + tool/CI/workspace registration (D2) | WB6, WB7, WB10, WB11 |

## Exit criteria

Mapped row-for-row to `docs/ROADMAP.md` §9 (Gate 7):

| # | ROADMAP §9 exit line | This spec's evidence |
|---|----------------------|----------------------|
| 1 | action trees work in CLI and web | §R9 tree phases; §R13 WASM action-tree export; §R21 web pointer/keyboard play; §R23 "Action tree and replay" + "Web and accessibility" |
| 2 | forced continuations replay correctly | §R9 forced continuation; §R10 one-command multi-segment replay; §R18 forced-continuation + multi-jump golden traces; §R23 "Action tree and replay" |
| 3 | UI guides path construction clearly | §R14 input model; §R15 announcements; §R21 legal origins/destinations + forced continuation indicated; §R23 "Web and accessibility" |
| 4 | baseline bot follows forced rules | §R17 Level 0/1 + "never emits partial continuation paths"; §R23 "Bots" |
| 5 | legal tree and bot benchmarks exist | §R19 legal-generation + bot benchmarks; §R23 "Benchmarks" |

## Acceptance evidence

Re-runnable confirmation (per `docs/TESTING-REPLAY-BENCHMARKING.md` and
`docs/OFFICIAL-GAME-CONTRACT.md`); full criteria in §R23:

- `cargo fmt --all --check`, `cargo clippy --workspace --all-targets -- -D warnings`, `cargo build --workspace`, `cargo test --workspace`
- `cargo run -p simulate -- --game draughts_lite --games 1000` (bounded-nonterminal aware; §R18)
- `cargo run -p replay-check -- --game draughts_lite --all`
- `cargo run -p fixture-check -- --game draughts_lite`
- `cargo run -p rule-coverage -- --game draughts_lite`
- `cargo bench -p draughts_lite` (calibrated, conservative thresholds; §R19)
- `npm --prefix apps/web run smoke:wasm`, `npm --prefix apps/web run smoke:e2e`
- `bash scripts/boundary-check.sh` (engine-core noun-free), `node scripts/check-doc-links.mjs`

Evidence set must cover the `docs/OFFICIAL-GAME-CONTRACT.md` deliverables: rule
coverage, golden traces, replay determinism, visibility/no-leak, bot legality,
and benchmarks.

## FOUNDATIONS & boundary alignment

| Principle | Stance | Rationale |
|-----------|--------|-----------|
| §2 Behavior authority | aligns | Rust owns setup, legal generation, validation, mutation, effects, views, bots, replay; §R14/§R22 forbid TypeScript legality (TS traverses Rust action tree only). |
| §3 `engine-core` is a contract kernel | aligns | Compound moves reuse the existing generic `ActionTree`/`ActionChoice.next`/`ActionPath`; §R9/§R22 forbid board/capture/promotion/movement nouns in `engine-core`. |
| §4 `game-stdlib` is earned | reopen-and-decide | §R12 reopens the standing Gate 6 `rejected/deferred` atlas decision under its stated reopen constraints; promotion only if behavior-free, no origin/order flags, no trace/hash migration; ledger + atlas rows updated. |
| §5 Static data is typed content | aligns | Variants/manifest/fixtures are typed content/metadata only; no selectors, triggers, YAML, or DSL (§R8, §R18). |
| §11 Universal acceptance invariants | aligns | Determinism preserved (Trace Schema v1 retained, §R10), perfect-information no-leak (§R11), bots via the legal action API (§R17), effects drive animation (§R16), evidence-heavy package (§R18–§R20). |
| §12 Stop conditions | clear | No engine-core noun leak, no TS legality, no guessed-diff animation, ledger decision required before promotion, no hidden-info path (perfect information). |
| §13 ADR triggers | clear | No replay/hash semantics change (no schema bump), primitive stays in the §4 pressure path, no search/ML/RL bot class introduced. |

## Forbidden changes

(Detail in §R22; carries ROADMAP §9 "Not allowed".)

- Moving rules/legality/validation/mutation/effects/views/bots/replay out of Rust.
- TypeScript deciding legality, computing captures, diagonals, continuation, promotion, or terminal outcomes.
- Adding board/grid/capture/promotion/movement/UI nouns to `engine-core`.
- Promoting anything but a narrow rule-agnostic board-space helper into `game-stdlib`.
- Forced retrofit of `three_marks`/`column_four`/`directional_flip` to the new primitive.
- A Trace Schema version bump or any change to one-segment game traces.
- Search bots (minimax/alpha-beta/MCTS/playout/transposition/endgame DB/opening book), strong-engine claims, network multiplayer, hidden information.

## Documentation updates required

(Detail in §R20.) Full `games/draughts_lite/docs/` package from templates
(`RULES`, `SOURCES`, `MECHANICS`, `RULE-COVERAGE`, `GAME-IMPLEMENTATION-ADMISSION`,
`AI`, `BENCHMARKS`, `UI`, `PUBLIC-RELEASE-CHECKLIST`, `COMPETENT-PLAYER`,
`BOT-STRATEGY-EVIDENCE-PACK`, `PRIMITIVE-PRESSURE-LEDGER`); plus
`docs/MECHANIC-ATLAS.md` (supersede the board-space + movement/capture rows),
`docs/SOURCES.md` if global source policy is tracked there,
`apps/web/e2e/NO-LEAK-A11Y-CHECKLIST.md`, and `specs/README.md` (index flip — see
Sequencing).

## Sequencing

- **Predecessor:** Gate 6 `directional_flip` — `Done` in `specs/README.md` (admission satisfied).
- **Successor:** Gate 8 (`high_card_duel` / `blackjack_lite`, hidden-information proof) — not yet specced; do not couple to it.
- **Index:** on acceptance of this spec, flip the `specs/README.md` Gate 7 row from `Not started` to `Planned` and point it at this file (this is the Documentation-update action above, performed by the user/decomposition, not by this reassessment).

## Assumptions

- **A-1** The existing recursive action model is sufficient — `ActionChoice.next: Option<Box<ActionNode>>` and `ActionPath { segments: Vec<String> }` already exist in `engine-core` (verified); no new core action concept is required.
- **A-2** Trace Schema v1 is retained — golden traces already store `action_path` as a list even for one segment (verified); multi-segment paths need no schema bump.
- **A-3** The board-space primitive decision is a genuine reopen of the standing Gate 6 `rejected/deferred` atlas outcome; **defer/reject remains a live result** if the atlas reopen constraints are not met (§R12).
- **A-4** `seed-reducer` and `trace-viewer` are game-opt-in (currently `race_to_n` + `directional_flip` only); Draughts Lite support there is optional.
- **A-5** Draughts Lite is two-player perfect information; public and private views are equivalent (no hidden state to leak).

## Sources

External provenance for the rules and UX choices (re-stated in the game's
`SOURCES.md`, subject to the IP/source policy in §R20 / `docs/IP-POLICY.md`):

- World Checkers/Draughts Federation, *Rules of Draughts* (English draughts/checkers): <https://wcdf.net/rules/rules_of_checkers_english.pdf> — base movement, capture, mandatory capture/continuation, promotion-during-capture stop, win conditions; tournament adjudication intentionally omitted.
- W3C WAI-ARIA Authoring Practices Guide, *Grid pattern*: <https://www.w3.org/WAI/ARIA/apg/patterns/grid/> — accessible board interaction model.
- W3C WAI, *Understanding WCAG SC 2.3.3: Animation from Interactions*: <https://www.w3.org/WAI/WCAG21/Understanding/animation-from-interactions> — reduced-motion requirement.
- Schaeffer et al., *Checkers Is Solved*, Science 2007: <https://www.cs.cornell.edu/courses/cs6700/2013sp/readings/06-b-Checkers-Solved-Science-2007.pdf> — rationale for excluding strong-engine claims.
- University of Alberta Chinook project (solved-game context): <https://webdocs.cs.ualberta.ca/~chinook/games/>

---

## R2. Executive summary

Gate 7 should add **Draughts Lite** as a public-polish official game and use it as a pressure test for Rulepath’s compound action tree model. The gate is not “just another board game.” It must prove that the existing Rust-owned rules model can present and validate a multi-step, origin/destination, forced-continuation move without moving legality into TypeScript.

The target game contract is a deliberately small English draughts / American checkers subset:

- 8×8 board, playable dark squares only.
- 12 men per side in the first three playable rows for each side.
- Men move and capture diagonally forward only.
- Kings move and capture diagonally one square in any diagonal direction.
- Captures are mandatory.
- Continued captures by the same piece are mandatory.
- No maximum-capture rule.
- A man crowned on the king row ends the turn; if this happens during a capture sequence, the capture sequence stops immediately.
- Wins occur when the opponent has no pieces or no legal move.
- Tournament repetition, agreement draws, clocks, and long no-progress adjudication are non-goals for this gate.

The existing `engine-core` action model already has nested action choices, so Gate 7 should **not** introduce game-specific concepts into `engine-core`. The smallest compatible design is to represent a complete draughts move as one replay command containing a multi-segment action path: origin selection followed by one quiet landing or one-or-more jump landings. Partial selections are UI state only; Rust remains the source of legal next choices.

The most important boundary work is in `wasm-api` and the web shell: current evidence shows a static game registry and existing replay export paths that need to support multi-segment action paths end-to-end. Existing one-segment games must keep their traces and behavior unchanged.

Gate 7 should also promote, or explicitly decline with evidence, a minimal reusable rectangular board-space primitive. The recommended decision is **yes, promote a narrow rule-agnostic coordinate primitive into `game-stdlib`** and use it in `draughts_lite`, without retrofitting earlier games during this gate.

---

## R3. Why this gate exists

Previous gates proved baseline game admission, deterministic traces, a static WASM/web shell, board UI, public polish, and directional board mechanics. Draughts Lite raises the interaction complexity in a way the current portfolio has not fully exercised:

- A move has an **origin** and one or more **destinations**.
- Captures are not optional when available.
- A capture can force a continuation by the same piece.
- The UI must guide a user through a legal path without calculating legality itself.
- A replay should remain readable as one move, not a haze of partial client events.
- Effects must describe multiple semantic changes from one command: hops, captures, crowning, turn advance, and terminal outcomes.
- Bot behavior must choose complete legal paths and explain modest rule-informed choices without becoming a search engine.

This is the gate where Rulepath should prove that action trees are not just flat menu choices. The pressure point is a compound legal action assembled from a Rust-owned tree and committed as deterministic replay.

---

## R4. Repository alignment

The fetched repository evidence supports a full Rust/WASM/web Gate 7, not a native-only proof.

The foundation docs establish the key boundary: Rust owns setup, legal action generation, validation, state mutation, effects, views, bots, and replay, while TypeScript owns presentation and input forwarding. `engine-core` is intentionally mechanic-noun-free, and reusable mechanics belong in `game-stdlib` only when earned by repeated concrete pressure. See `docs/FOUNDATIONS.md`, `docs/ENGINE-GAME-DATA-BOUNDARY.md`, `docs/WASM-CLIENT-BOUNDARY.md`, and `docs/OFFICIAL-GAME-CONTRACT.md`.

The existing action model already contains recursive action choices through `ActionChoice.next`, and replay commands already carry `action_path` as a sequence rather than a single scalar. See `crates/engine-core/src/action.rs` (`ActionChoice.next`) and `crates/engine-core/src/lib.rs` (`ActionPath { segments: Vec<String> }`) with `crates/engine-core/src/replay.rs`. This is enough for Draughts Lite’s tree shape if `wasm-api`, web state, replay export, fixtures, and tests preserve multi-segment paths.

`wasm-api` currently registers games statically and has game-specific match records. Gate 7 should follow that precedent and add `draughts_lite` to the registry, match creation, view projection, action tree export, command application, bot turn execution, effects, and replay export. The bridge is `crates/wasm-api/src/lib.rs`.

`game-stdlib` is currently a placeholder crate. The fetched docs repeatedly warn against speculative abstraction, but the sequence of board games now creates real pressure: `three_marks`, `column_four`, `directional_flip`, and Draughts Lite all need stable rectangular board/cell conventions. Gate 7 is the right point to promote only the smallest rule-agnostic board-space utilities, not draughts mechanics.

The web shell already has game-picker, match setup, board components, effect feedback, replay import/export, and E2E smoke structure under `apps/web/src/**` and `apps/web/e2e/**`. A public-polish gate must update those surfaces.

One freshness caveat matters: overview/status prose can lag the actual code and spec tree (Gate 6 `directional_flip` is implemented and `Done`, even where some overview prose is not perfectly synchronized). This spec treats the actual crate/game/doc tree as the target evidence and requires Gate 7 to update public docs wherever new status is introduced.

---

## R5. Research summary and rules rationale

### Draughts/checkers rules

The World Checkers/Draughts Federation English rules support the selected base rules: ordinary men move one square diagonally forward; kings move one square diagonally forward or backward; men capture by jumping a forward adjacent opponent into the vacant square beyond; kings capture similarly forward or backward; captures are compulsory; continued jumps must be completed; a player may choose among available jump sequences rather than being forced to take the maximum number of pieces; and a man that reaches the king row during a capture is crowned but may not continue jumping until after the opponent has moved. The same source identifies no-piece and blocked-no-move wins, while tournament draw procedures are separate adjudication rules.

Gate 7 should use those rules, but intentionally omit tournament adjudication: repetition claims, agreement draws, referee timing, clocks, and long no-progress procedures are not needed to prove compound action trees. This omission must be clearly documented in `RULES.md` and `SOURCES.md`.

### Accessible board/grid interaction

The WAI-ARIA APG grid pattern supports an interactive grid where cells or elements inside cells are focusable, arrow keys move focus among cells, Home/End move within a row, and Control+Home/Control+End move to board extremes. For a draughts board, this maps cleanly to a roving focus or `aria-activedescendant` model. The implementation should use the APG guidance as a pattern, not as a license to overload the board with hidden client-side rules.

The board must announce legal origins, selected piece, legal destinations, captures, forced continuation, promotion, and terminal outcome through accessible labels and live-region status. Legal status must come from Rust-exported action trees and metadata, not TypeScript legality calculations.

### Reduced motion

WCAG Success Criterion 2.3.3 treats non-essential animation triggered by interaction as something users must be able to disable, and identifies `prefers-reduced-motion` as an accepted mechanism. Draughts Lite should have strong public-polish animation by default, but reduced-motion mode must replace movement/capture/promotion animations with short non-motion emphasis and text/effect feedback.

### AI scope

Schaeffer et al.’s *Checkers Is Solved* reports the solved-game scale of checkers as roughly 5×10²⁰ positions and describes a long-running research effort beginning in 1989 with major search/database machinery. That is exactly why Gate 7 must not make strong-engine claims. A Level 1 bot can be rule-informed and pleasant for smoke/play, but minimax, MCTS, opening books, solved-game databases, and competitive checkers strength are out of scope.

---

## R6. Scope

Gate 7 includes all of the following:

1. A new official game crate: `games/draughts_lite`.
2. Workspace registration and CI/tool integration for the new game.
3. A complete public Draughts Lite rules implementation in Rust.
4. Nested action-tree generation for origin selection, quiet landings, jump landings, forced continuation, and promotion-terminated capture sequences.
5. Deterministic validation and state mutation from complete multi-segment action paths.
6. Trace/replay/golden fixture coverage for normal, forced, multi-jump, promotion, terminal, invalid, bot, and WASM-exported cases.
7. WASM bridge support and web shell/game picker registration.
8. A public-polish web board with mouse, keyboard, accessible status, legal path guidance, animation/effect feedback, and reduced-motion support.
9. Level 0 and modest Level 1 bot behavior.
10. Honest native benchmarks and WASM/web smoke/perf expectations.
11. Official-game admission docs populated from repo templates.
12. A serious primitive-promotion decision and ledger update.

---

## R7. Non-goals

Gate 7 must not become a general checkers engine. The following are explicitly out of scope:

- International draughts, Russian draughts, pool checkers, Canadian draughts, 10×10 variants, flying kings, backward-capturing men, huffing, or variant packs.
- Maximum-capture legality.
- Tournament adjudication: clocks, referee claims, repetition counting, long no-capture/no-king-move rules, agreement-draw workflow, resignation workflow, ratings, or match series.
- Network multiplayer.
- Hidden information.
- Minimax, alpha-beta, MCTS, playout search, endgame databases, opening books, solved-game claims, or strong-engine language.
- Moving board/capture/promotion concepts into `engine-core`.
- TypeScript legality calculation.
- Retrofitting all existing games to a new board primitive during Gate 7.
- Rewriting the action/replay architecture beyond the smallest changes needed for multi-segment action paths at the WASM/web boundary.

---

## R8. Chosen game and rules contract

### Identity

- **Public name:** Draughts Lite
- **Game id:** `draughts_lite`
- **Rust crate:** `games/draughts_lite`
- **Default variant id:** `draughts_lite_standard`
- **Rules version:** `draughts_lite-rules-v1`
- **Data version:** `1`
- **Information model:** two-player, perfect information, deterministic rules
- **Seats:** `seat_0` and `seat_1`
- **Default first player:** `seat_0`

### Board

- The board is 8 rows by 8 columns.
- Canonical cell ids are `r1c1` through `r8c8`.
- Row 1 is the top row in canonical serialized view.
- `seat_0` moves toward increasing row numbers.
- `seat_1` moves toward decreasing row numbers.
- Playable cells are dark squares only.
- The standard playable parity is: a cell is playable when `row + column` is odd.
- Non-playable cells are visible as board cells but may never contain pieces and may never be legal origins or destinations.

### Setup

- `seat_0` starts with 12 men on playable cells in rows 1, 2, and 3.
- `seat_1` starts with 12 men on playable cells in rows 6, 7, and 8.
- Rows 4 and 5 start empty.
- Every piece receives a deterministic stable piece id for trace readability and UI animation. Piece ids must not be inferred by TypeScript.

### Pieces

- Men and kings are the only piece kinds.
- Men belonging to `seat_0` move forward by `+1` row.
- Men belonging to `seat_1` move forward by `-1` row.
- Kings move in all four diagonal directions.

### Quiet movement

- A man may move one playable diagonal square forward into an empty cell.
- A king may move one playable diagonal square in any diagonal direction into an empty cell.
- Quiet moves are legal only when the active player has no legal capture anywhere on the board.

### Capture movement

- A man may capture by jumping a diagonally forward adjacent opposing piece and landing on the empty playable square immediately beyond it.
- A king may capture by jumping a diagonally adjacent opposing piece in any diagonal direction and landing on the empty playable square immediately beyond it.
- Only adjacent jumps are supported. Kings are not flying kings.
- Captured pieces are removed when the full command is applied. During validation and path generation, pieces already captured in the same sequence must be treated as unavailable and may not be captured again.

### Mandatory capture

- If the active player has at least one legal capture, no quiet action path is legal.
- When multiple capture origins or capture paths exist, the player may choose any legal capture path. Draughts Lite has no maximum-capture rule.
- Level 1 bot may prefer longer capture paths as a heuristic, but that is not a legality rule.

### Mandatory continuation

- After a capture, if the same moving piece has another legal capture, continuation is mandatory.
- The action tree must expose only continuation jumps for that same piece.
- A command that stops early while another continuation is available must be rejected with a specific diagnostic.
- The player may not switch to another piece during a capture sequence.

### Promotion

- A man that reaches the opponent’s king row becomes a king.
- `seat_0` crowns on row 8.
- `seat_1` crowns on row 1.
- A man that reaches the king row by quiet move is crowned and the turn ends.
- A man that reaches the king row during a capture sequence is crowned and the move ends immediately, even if a king on that landing square would have a further capture.
- A command segment after promotion-during-capture must be rejected with a specific diagnostic.

### Terminal outcomes

- After every completed turn, if the opponent has no pieces, the mover wins.
- After every completed turn, if the opponent has no legal move, the mover wins.
- At game start, standard setup must not be terminal.
- Stalemate is not a draw in this rules contract; no legal move for the active player means that player loses.
- Draw adjudication is omitted in Gate 7. The docs must say this plainly.

---

## R9. Compound action tree requirements

### Architectural decision

Gate 7 should use the existing recursive action tree model. No draughts-specific extension belongs in `engine-core`.

The existing model is sufficient because an `ActionChoice` can carry a segment, label, accessibility label, metadata, tags, preview, and an optional next node. A complete command can therefore be represented as a sequence of path segments selected from root to leaf.

The required change is not a new core action concept. The required change is making sure all game, replay, WASM, and web code paths preserve and understand **multi-segment** `action_path` values.

### Tree phases

The legal action tree for a non-terminal active player must represent these phases:

| Phase | Tree shape | UI meaning | Rust-owned guarantees |
|---|---|---|---|
| Select origin | Root choices are legal active pieces. | User chooses the piece to move. | Choices include only legal origins under current mandatory-capture rules. |
| Select landing | The selected origin choice has destination children. | User chooses a quiet landing or first jump landing. | Children include only legal destinations for that origin. Quiet children are absent when any capture exists. |
| Continue capture | A jump child has another destination node when continuation is mandatory. | User continues with the same piece. | Children include only legal continuation jumps for the same piece, with already-captured pieces unavailable. |
| End move | A landing choice is a leaf when no continuation is legal, or when promotion during capture ends the sequence. | User has completed a legal move. | The full selected action path can be submitted as one command. |

There is no separate `commit` or `end turn` action segment. A leaf action path commits the move. Partial UI selections are not replay commands.

### Segment conventions

The implementation must choose stable, readable segment names and document them in `MECHANICS.md` and golden traces. The recommended convention is:

- Origin segment: `from/rNcM`
- Quiet landing segment: `to/rNcM`
- Capture landing segment: `jump/rNcM`

Examples:

- Quiet move: `from/r3c2 → to/r4c3`
- Single capture: `from/r3c2 → jump/r5c4`
- Multi-jump: `from/r3c2 → jump/r5c4 → jump/r7c6`

Segment strings are part of the replay contract. They must remain stable unless a future trace-schema migration is explicitly documented.

### Choice metadata

Each action choice must include enough metadata for presentation without TypeScript legality:

- phase: origin, quiet landing, jump landing, forced continuation landing
- cell id
- piece id for origin choices
- piece kind before the move
- active seat
- whether a capture is globally mandatory
- whether this choice is a capture
- captured cell and captured piece id for jump choices
- whether the landing would promote a man
- whether the choice is forced by continuation
- human-readable label
- accessibility label
- tags suitable for styling, such as legal-origin, legal-destination, capture, forced, promotion, king

Metadata is presentation data. Validation must not depend on the client returning metadata.

### Preview behavior

Rust may attach previews to action choices if existing JSON surfaces support them. Preview data should be public and should describe only the current choice path, not hidden state. The minimum useful preview is:

- highlighted origin
- highlighted landing
- captured piece/cell for a jump
- forced continuation hint when a jump child has a next node
- promotion hint when applicable

If the current preview surface cannot express this cleanly, the action choice metadata and public effects are sufficient for Gate 7. Do not block the gate on a broad preview architecture rewrite.

### Invalid path diagnostics

Validation must reject malformed or illegal complete paths with stable diagnostic codes. Required classes include:

- stale freshness token
- command actor is not the active seat
- terminal state already reached
- empty action path
- malformed segment
- origin outside board
- destination outside board
- origin on non-playable cell
- destination on non-playable cell
- no piece at origin
- origin piece belongs to another seat
- destination occupied
- quiet move while capture is available
- illegal quiet movement pattern
- illegal capture movement pattern
- jumped cell empty
- jumped piece belongs to active seat
- landing square not empty
- capture path stops before mandatory continuation
- continuation tries to move a different piece
- continuation tries to jump a piece already captured in the same sequence
- path continues after promotion during capture ended the sequence
- action path does not match any current legal tree leaf

Diagnostics must be public-safe and must not expose hidden state. Draughts Lite is perfect-information, but the habit matters.

---

## R10. Replay and trace semantics

### Storage model

A complete draughts move, including a multi-jump sequence, must be stored as **one high-level replay command** with a multi-segment `action_path`.

Do not store each jump as a separate replay command. Do not store UI origin selection as a replay command. Do not serialize partial action-tree navigation as game history.

This preserves the rule meaning of a draughts move: one turn, one active piece, possibly multiple jumps, one state transition.

### Trace schema impact

The existing trace command shape already represents `action_path` as a list of segments in fetched golden traces for existing games, even when those lists contain one segment. Gate 7 should not require a trace schema version bump merely because a path has more than one segment.

A schema bump is justified only if the implementation changes the trace envelope itself. The preferred path is:

- keep Trace Schema v1;
- allow command `action_path` lists with length greater than one;
- update replay export/import and WASM-exported traces so multi-segment paths round-trip exactly;
- keep all existing one-segment game traces stable.

### Effect semantics

A single applied command may emit multiple effects. Effects must be deterministic, public-safe, serializable, and hash-stable. Required semantic effect coverage:

| Event | Required effect information |
|---|---|
| Move committed | action path, active seat, moving piece id, start cell, final cell, move kind, path length |
| Quiet step | origin, landing, piece id, piece kind before/after |
| Capture step | hop origin, hop landing, captured cell, captured piece id, captured piece owner, moving piece id |
| Promotion | piece id, seat, cell, promoted from man to king, whether promotion occurred during capture |
| Forced capture available | active seat, count or existence of legal capture origins, public explanation |
| Forced continuation required | moving piece id, current landing, continuation destination count, public explanation |
| Illegal selection or invalid command | diagnostic code, public message, rejected action path when safe |
| Terminal win | winner, loser, reason no pieces or no legal move |
| Bot action | bot level, bot policy version, seed or deterministic bot context already permitted by existing bot traces, selected action path, short public rationale |

The effect vocabulary can use game-prefixed names if existing games do so. Whatever names are chosen must be documented in `MECHANICS.md`, reflected in golden traces, and consumed by web effect feedback.

### Replay readability

Golden traces must be readable to a human reviewing a failed replay. Each trace should include a plain note explaining the rule under test. Multi-jump traces should make the selected path intelligible from the action path alone.

The replay checker must prove:

- deterministic state hashes;
- deterministic effect hashes;
- deterministic action-tree hashes after each checkpoint;
- deterministic public-view hashes;
- deterministic replay hashes;
- stable handling of invalid diagnostics;
- stable WASM-exported command shape.

---

## R11. Rust game implementation requirements

### Crate shape

Create `games/draughts_lite` and align its file shape with the established official games. The exact module names may follow existing conventions, but the crate must contain clear equivalents for:

- actions/action-path parsing
- bots
- effects
- ids/constants
- library entry points
- replay support
- rules/legal generation/validation
- setup
- state
- UI/public view serialization helpers if needed
- variants/options
- visibility/perfect-information projection
- tests
- benches
- fixtures and manifests
- docs

Register the crate in root workspace membership and in any tool or CI game lists that enumerate official games.

### State requirements

The game state must be serializable and hash-stable. It must include:

- seats
- active seat
- board dimensions and variant identity
- stable piece records or equivalent deterministic board representation
- piece id, owner, kind, and cell for every live piece
- captured/removed pieces either absent from live board or represented in a stable non-live form
- terminal outcome
- ply count or equivalent command count
- freshness token
- accumulated effects or last effects, following existing game conventions

Stable piece ids are required for animation and trace readability. Do not force TypeScript to infer identity by diffing cells.

### Legal generation

Legal generation must be deterministic and ordered canonically. Recommended ordering:

1. row-major origin order by canonical cell id;
2. within origin, forward-left before forward-right for men from that seat’s perspective;
3. for kings, a documented fixed diagonal order;
4. for continuation, same documented landing order;
5. bot tie-breakers then apply deterministic seeded selection or deterministic ranking.

The exact order must be documented because action-tree hashes and bot traces depend on it.

Legal generation must compute global capture availability before offering quiet moves. If any active piece can capture, root choices must include only pieces with at least one capture.

### Validation

Validation must recompute legality from state and command path. It must not trust client metadata or labels. The validator must prove that the path is a leaf of the current action tree under the command’s freshness token and actor seat.

Validation must handle multi-jump path state internally without mutating the real game state until the command is applied.

### Apply

Applying a validated action must be deterministic and atomic:

- move the piece through the path;
- remove captured pieces;
- promote if needed;
- end capture immediately on promotion-to-king-row by a man;
- advance active seat unless terminal outcome is reached;
- update freshness token;
- append or replace effects according to existing game convention;
- evaluate terminal outcome after the move.

If validation fails, apply must not mutate state.

### Visibility

Draughts Lite is perfect-information. Public view and private view, if private view exists, should be equivalent or private view should be explicitly not applicable, following existing game conventions.

Visibility tests must prove no hidden-information leak because there is no hidden state.

### Native proof before web polish

The implementation should first make the Rust game deterministic and fully tested. However, Gate 7 acceptance is not Rust-only. The gate is complete only when WASM and public web integration pass.

---

## R12. Reusable primitive decision requirements

### Decision

Gate 7 **reopens** the board-space primitive decision and proposes promoting a **minimal rectangular board-space primitive** into `game-stdlib` for use in `draughts_lite`. This is a reopen-and-decide, **not a foregone conclusion**.

The standing repository decision in `docs/MECHANIC-ATLAS.md` (repository-level primitive-pressure law) is `rejected/deferred with rationale` as the Gate 6 as-built outcome for the related shapes: the "fixed 2D occupancy" and "coordinate/targeted placement" ledger rows kept these helpers game-local across `three_marks`, `column_four`, and `directional_flip`. That same ledger names the conditions under which the decision may reopen: another official spatial game proving "one stable coordinate helper **without origin/order flags**", or a post-Gate 6 audit proving "one narrow behavior-free helper **without trace/hash migration**". Draughts Lite is that next official spatial game, so the reopen is legitimate.

Promotion is warranted **only if** the implementer can satisfy those atlas reopen constraints: a coordinate/board-space helper with no origin/order policy, no draughts vocabulary, and no trace- or hash-migration impact on the three already-admitted games. If those constraints hold, Draughts Lite adds enough repeated pressure around coordinates, bounds, offsets, row/column ids, deterministic ordering, and board-cell presentation to justify a narrow, rule-agnostic promotion. If they do not hold, defer/reject remains the **live** outcome (see "If implementation evidence rejects promotion" below). The promoted primitive must remain rule-agnostic.

Either outcome MUST update — and **supersede** where it changes — the existing `docs/MECHANIC-ATLAS.md` ledger rows for "fixed 2D occupancy", "coordinate/targeted placement", and "movement/capture/forced continuation" (`draughts_lite`), rather than merely appending a new row, and MUST record the reasoning in `PRIMITIVE-PRESSURE-LEDGER.md`.

### Location

Place the primitive in `crates/game-stdlib`, not `engine-core`.

`engine-core` must remain game-mechanic noun-free. Board-space utilities are reusable game-building helpers, not universal engine law.

### Minimal safe primitive

The primitive should cover only stable board-space concepts:

- rectangular board dimensions;
- canonical row/column coordinate type;
- bounds checking;
- deterministic row-major coordinate iteration;
- stable cell id formatting and parsing compatible with `rNcM` public ids;
- coordinate offset arithmetic with signed row/column deltas;
- optional parity helper, if kept generic as parity rather than “dark square” policy.

### Explicitly out of scope for the primitive

Do not put these into `game-stdlib` during Gate 7:

- draughts move generation;
- captures;
- promotion;
- mandatory continuation;
- occupancy storage policy;
- piece identity;
- line-win detection;
- gravity/drop logic;
- flip/bracket logic;
- UI rendering;
- WASM types;
- bot heuristics;
- variant-specific playable-square rules;
- any assumption that an 8×8 board is special.

Draughts Lite may use a local helper for dark-square playability, diagonal move classification, capture path generation, and promotion. Those are draughts rules, not generic board primitives.

### Retrofit policy

Do not retrofit `three_marks`, `column_four`, or `directional_flip` to the new primitive during Gate 7 unless the change is trivial and risk-free. Gate 7 should not destabilize already admitted games.

The required proof is:

- `game-stdlib` tests for the primitive;
- `draughts_lite` uses the primitive;
- existing game tests still pass unchanged;
- `PRIMITIVE-PRESSURE-LEDGER.md` explains why the primitive was promoted now;
- `docs/MECHANIC-ATLAS.md` is updated with the new primitive and the boundary line between board-space helpers and game mechanics.

### If implementation evidence rejects promotion

If the implementer discovers a concrete contradiction that makes even this minimal primitive unsafe, the gate may proceed with local helpers only, but that must be treated as a reassessment finding, not a silent choice. The implementation must then populate `PRIMITIVE-PRESSURE-LEDGER.md` with the evidence and update `docs/MECHANIC-ATLAS.md` to say why promotion was deferred.

---

## R13. WASM/API boundary requirements

### Registration

`wasm-api` must expose Draughts Lite consistently with existing games:

- listed by `list_games`;
- creatable through match setup;
- public display name “Draughts Lite”;
- default variant `draughts_lite_standard`;
- serialized view returned by the existing view path;
- recursive action tree returned by the existing action-tree path;
- multi-segment actions accepted by the existing or minimally extended action-application path;
- Level 0/Level 1 bot turns executable through the existing bot path;
- effects accessible through the existing effects path;
- replay export produces a valid trace command stream.

### Multi-segment action paths

The WASM bridge must stop assuming that an exported replay action has only one segment. Existing one-segment games remain valid, but Draughts Lite requires command paths of length two or more.

The concrete one-segment assumptions live in `crates/wasm-api/src/lib.rs`: the replay-export path rejects multi-segment commands with the `unsupported_replay_action_path` diagnostic and returns only the first segment (the `action_path.len() != 1` guard, around lines 132–138), and `parse_action_path` (around line 1231) wraps the incoming string as a single segment without splitting it. Both must be extended so Draughts Lite multi-segment paths export and re-enter losslessly while existing one-segment games stay valid.

The API must preserve path segment order exactly. It must not flatten a path into an ambiguous string for replay storage. If a UI helper still accepts a string for manual/dev entry, that string must be parsed losslessly into the canonical segment list before validation.

### Recursive action tree serialization

The JSON action tree exposed to the web must preserve nested `next` nodes recursively. The web client must be able to walk from origin to destination to continuation without asking TypeScript to generate legal moves.

### Boundary tests

WASM tests or smoke scripts must prove:

- Draughts Lite appears in the game list;
- a match can be created;
- the standard initial public view serializes;
- the initial action tree has legal origins;
- a quiet move can be applied where legal;
- a forced capture path can be applied;
- a multi-jump path round-trips through action tree, apply, effects, and replay export;
- invalid paths return diagnostics;
- exported replay validates natively.

---

## R14. Web/UI requirements

### Game picker and shell

Add Draughts Lite to the public game picker and match setup flow. The game should be discoverable in the same way as existing public games.

The shell must not special-case Draughts Lite in a way that leaks rules into general components. Game-specific rendering belongs in a Draughts Lite board component or equivalent presentation layer.

### Board rendering

The board must render all 64 cells, including non-playable cells. It must clearly distinguish:

- playable vs non-playable cells;
- empty vs occupied cells;
- seat ownership;
- men vs kings;
- selected origin;
- legal origins;
- legal quiet destinations;
- legal capture destinations;
- forced continuation destinations;
- recently moved path;
- captured pieces;
- promotion;
- terminal outcome.

Non-playable cells must not appear interactive. Occupied legal origins and empty legal destinations must use both visual and accessible cues.

### Input model

Mouse/pointer behavior:

1. Clicking a legal origin selects that piece and advances to its child action node.
2. Clicking an illegal origin gives presentation feedback but does not submit a command.
3. Clicking a legal landing appends that segment to the pending path.
4. If the landing is a leaf, the UI submits the full path as one command.
5. If the landing has continuation children, the UI keeps the pending path selected and shows only Rust-provided continuation destinations.
6. Clicking another piece during forced continuation must not be treated as legal. It may clear only when Rust action tree state allows a new root selection, which it does not during an uncommitted continuation path.
7. Escape or an explicit cancel affordance clears the pending path before submission.

Keyboard behavior must mirror pointer behavior and is specified in the accessibility section.

### No TypeScript legality

TypeScript may:

- traverse the Rust-provided action tree;
- store the pending selected path;
- map action choice metadata to CSS classes and labels;
- display legal destinations returned by Rust;
- submit a complete path to WASM;
- display Rust diagnostics and effects.

TypeScript must not:

- determine whether a move is diagonal;
- determine whether a square is playable except by rendering Rust-provided/public board data or generic cell metadata;
- calculate captures;
- calculate mandatory continuation;
- decide promotion;
- infer terminal outcomes;
- choose bot moves;
- mutate authoritative game state.

### Dev panel and replay UI

The dev panel/replay viewer must display multi-segment paths legibly. It should show full paths with a stable separator and must not truncate after the first segment.

Replay import/export UI must round-trip Draughts Lite traces without corrupting multi-segment commands.

---

## R15. Accessibility requirements

### Semantics

Use a grid-like board interaction pattern aligned with WAI-ARIA APG guidance:

- one board grid in the tab order, or a roving focus model where exactly one cell/control is tabbable at a time;
- row and cell semantics, or button-in-cell semantics, chosen consistently;
- focusable cells or cell controls must have useful accessible names;
- legal selectable cells expose selected/available state through accessible attributes and labels;
- non-playable and illegal cells are announced as not available or are not interactive;
- the selected origin is announced;
- legal destinations are announced;
- forced continuation is announced through a live status region.

### Keyboard navigation

Required keys:

- `Tab`: enter or leave the board without trapping the user.
- Arrow keys: move focus one cell up, down, left, or right within the 8×8 board. Do not wrap by default; wrapping can disorient board coordinates.
- `Home`: move focus to the first cell in the current row.
- `End`: move focus to the last cell in the current row.
- `Control+Home`: move focus to `r1c1`.
- `Control+End`: move focus to `r8c8`.
- `Enter` or `Space`: activate the focused cell if it is a Rust-provided legal origin or legal destination for the current pending path.
- `Escape`: clear the pending path and return to root action selection.

Optional keys may be added only if they do not interfere with the required grid behavior.

### Announcements

The live region must announce:

- whose turn it is;
- when a capture is mandatory;
- selected piece and cell;
- legal destination count after selecting a piece;
- capture landing selected;
- forced continuation required and destination count;
- promotion;
- invalid command diagnostics;
- bot move summary;
- terminal winner and reason.

Examples of announcement content, not literal required strings:

- “Seat 0 must capture. Three pieces can capture.”
- “Selected seat 0 man at row 3 column 2. Two capture destinations.”
- “Capture to row 5 column 4. Continue jumping with the same piece.”
- “Promoted to king on row 8 column 7. Turn ends.”
- “Seat 1 wins because seat 0 has no legal move.”

### Reduced motion

The web implementation must respect both:

- the existing app-level reduced-motion preference if present; and
- the operating-system/browser `prefers-reduced-motion` setting.

When reduced motion is active:

- no long sliding or jumping animations;
- capture/promotion feedback uses non-motion emphasis and text;
- forced continuation emphasis must not pulse indefinitely;
- terminal celebration must be static or near-static;
- effect log remains complete.

### E2E/a11y checklist

Update `apps/web/e2e/NO-LEAK-A11Y-CHECKLIST.md` and smoke coverage to include Draughts Lite keyboard paths, focus visibility, accessible names, reduced motion, non-color cues, and no TypeScript legality leakage.

---

## R16. Animation and effect feedback requirements

Draughts Lite should receive public-polish treatment. Animation is not optional, but it must be driven by Rust effects and must respect reduced motion.

Required feedback:

- legal origin and destination highlights;
- selected piece emphasis;
- move path animation for quiet moves;
- hop animation or equivalent for captures;
- captured piece removal feedback;
- promotion/crowning feedback;
- forced continuation hint;
- bot action highlight;
- terminal win feedback;
- invalid selection/diagnostic feedback.

The default animation should make compound moves understandable: the user should see that a multi-jump was one move by one piece, not multiple turns. In reduced-motion mode, the same semantic sequence should be conveyed through static highlights and the effect log.

Do not animate by diffing arbitrary before/after board state in TypeScript when a Rust effect provides the semantic event. Diffing may be used only as a fallback for generic shell transitions, not as a rules interpretation layer.

---

## R17. Bot/AI requirements

### Level 0

Level 0 is random legal play:

- chooses a complete legal leaf action path from the current Rust action tree;
- handles nested action trees recursively;
- uses deterministic seeded randomness;
- returns a diagnostic only when no legal action exists before terminal, which should indicate a rules bug or terminal-state handling issue;
- makes no strategy claims.

The existing `ai-core` recursive random legal behavior is a precedent and should be reused or mirrored where compatible.

### Level 1

Level 1 is a modest rule-informed bot. It is not a search project.

Acceptable Level 1 heuristics:

- prefer an immediate winning move when cheaply detectable by applying a candidate on a cloned state;
- prefer capture paths when the tree contains both capture and non-capture paths, though mandatory-capture rules usually remove quiet alternatives already;
- prefer promotion moves;
- prefer capture-to-promotion moves;
- prefer paths that capture more pieces as a heuristic only;
- prefer preserving or creating kings;
- avoid landing a king where the opponent has an immediate obvious capture when this is cheaply detectable with a one-ply local check;
- use deterministic ordering or seeded tie-breaking after heuristic ranking.

Explicitly excluded:

- minimax;
- alpha-beta;
- MCTS;
- random playout evaluation;
- transposition tables;
- solved-game databases;
- opening books;
- external checkers engines;
- claims of competitive strength.

### Bot evidence

`AI.md`, `COMPETENT-PLAYER.md`, and `BOT-STRATEGY-EVIDENCE-PACK.md` must explain what Level 1 does and does not do. Bot tests must prove:

- Level 0 always chooses a legal complete path when one exists;
- Level 0 can choose multi-segment paths;
- Level 1 prefers a promotion over a non-promotion when both are legal and otherwise comparable;
- Level 1 prefers a capture path when applicable;
- Level 1 can complete mandatory continuation;
- Level 1 never emits partial continuation paths;
- bot traces are deterministic for fixed seeds.

---

## R18. Fixture, golden trace, property, and rule-coverage requirements

### Standard fixture

Create `games/draughts_lite/data/fixtures/draughts_lite_standard.fixture.json` or the repository-conventional equivalent.

The fixture must validate the standard setup:

- 8×8 board;
- correct playable-cell parity;
- 12 men per side;
- no pieces on non-playable cells;
- active seat `seat_0`;
- no terminal outcome;
- deterministic state/public view hash baseline if fixtures encode hashes.

### Golden traces

Create golden traces covering at least:

1. shortest quiet move;
2. mandatory capture suppressing quiet move;
3. single capture;
4. multi-jump capture;
5. forced continuation branch in the action tree;
6. promotion by quiet move;
7. promotion during capture ending the sequence;
8. terminal win by no pieces;
9. terminal win by no legal moves;
10. stale freshness diagnostic;
11. non-active-seat diagnostic;
12. occupied-destination diagnostic;
13. non-playable-cell diagnostic;
14. quiet move rejected while capture exists;
15. illegal continuation diagnostic;
16. path continuing after promotion-during-capture diagnostic;
17. bot action;
18. WASM-exported trace.

Each trace must include a clear purpose/note and expected hashes following existing trace conventions.

### Rule tests

Rule tests must cover:

- dark-square play;
- legal initial setup;
- invalid setup where relevant;
- man quiet move forward;
- man cannot move backward;
- king quiet move backward and forward;
- man forward capture;
- man cannot backward-capture;
- king captures in each diagonal direction;
- mandatory capture;
- no maximum-capture rule;
- multi-jump continuation;
- same-piece continuation only;
- captured piece cannot be captured twice in one sequence;
- promotion after quiet move;
- promotion during capture stops sequence;
- terminal no-piece win;
- terminal no-legal-move win;
- perfect-information view equivalence.

### Property tests

Draughts can cycle when draw adjudication is omitted. Therefore property tests must not assert that random legal play always terminates within a small fixed action count.

Instead, property tests should prove:

- every action-tree leaf validates;
- validated actions apply without panic;
- applied legal actions preserve board invariants;
- no live piece is off-board or on a non-playable square;
- no two live pieces occupy the same cell;
- piece counts change only by capture;
- men promote only on the correct king row;
- a man that promotes during capture has no further command segments accepted;
- bounded random simulations either terminate legally or stop at an action cap with a nonterminal smoke result, not a false failure.

### Rule coverage tool

Update `tools/rule-coverage` so Draughts Lite’s major clauses are recognized. The docs and tool output must cover the rule groups listed above, not merely count test files.

### Simulation tool

Update `tools/simulate` for `draughts_lite`. Because Gate 7 omits draw adjudication, simulation must distinguish:

- terminal games;
- bounded nonterminal games that reached the action cap without invariant failure;
- true failures such as invalid bot action, replay drift, panic, or invariant violation.

Do not add fake draw rules just to satisfy the simulation runner.

---

## R19. Benchmarking requirements

### Native benchmarks

Add Draughts Lite benchmarks for:

- standard setup;
- initial legal action tree;
- midgame legal action tree with no capture;
- midgame legal action tree with mandatory capture;
- capture-rich multi-jump legal generation;
- validate/apply quiet move;
- validate/apply single capture;
- validate/apply multi-jump;
- public view projection;
- replay check throughput;
- Level 0 bot selection;
- Level 1 bot selection.

Benchmark names must be stable and documented in `BENCHMARKS.md`.

### Threshold calibration

Follow the repository’s benchmark lane discipline: pull-request bench smoke should compile/run without hard threshold enforcement, while scheduled/main benchmark gates may enforce committed thresholds.

Do not invent aggressive thresholds before implementation evidence exists. The first Draughts Lite thresholds should be calibrated from actual local and CI measurements and documented as conservative baselines. If CI noise is high, thresholds should be broad enough to catch severe regressions without turning the gate into runner roulette.

### WASM/web smoke performance

Add web/WASM smoke expectations without pretending to have a full browser benchmark harness:

- game list loads;
- standard match creates quickly enough for smoke scripts;
- initial action tree export does not hang;
- forced-capture path can be selected and applied through the browser smoke;
- reduced-motion rendering does not skip required semantic feedback.

---

## R20. Documentation and admission requirements

Create and populate the full official-game admission package for `games/draughts_lite/docs/` using the repository templates:

- `RULES.md`
- `SOURCES.md`
- `MECHANICS.md`
- `RULE-COVERAGE.md`
- `GAME-IMPLEMENTATION-ADMISSION.md`
- `AI.md`
- `BENCHMARKS.md`
- `UI.md`
- `PUBLIC-RELEASE-CHECKLIST.md`
- `COMPETENT-PLAYER.md`
- `BOT-STRATEGY-EVIDENCE-PACK.md`
- `PRIMITIVE-PRESSURE-LEDGER.md`

No listed doc is intentionally omitted for Gate 7. Public-polish scope and primitive-pressure scope make the expanded package appropriate.

### Required doc content

`RULES.md` must state the chosen Draughts Lite rules plainly, including mandatory capture, mandatory continuation, no maximum-capture rule, promotion, promotion-during-capture stopping, terminal wins, and omitted draw/tournament adjudication.

`SOURCES.md` must cite the WCDF English rules and explain which rules are adopted and which adjudication details are omitted. It must also satisfy the repository IP/source policy: no proprietary assets, no copied rulebook prose beyond short quotations if any, no external engine data, and no opening book.

`MECHANICS.md` must document action path segments, action tree phases, effects, diagnostics, and board coordinate conventions.

`RULE-COVERAGE.md` must map major rule clauses to tests/traces/tool coverage.

`GAME-IMPLEMENTATION-ADMISSION.md` must show how the game satisfies official-game admission requirements and boundary discipline.

`AI.md` must define Level 0 and Level 1 behavior and exclusions.

`BENCHMARKS.md` must list benchmark names, rationale, and threshold calibration approach.

`UI.md` must describe board rendering, pointer flow, keyboard flow, forced continuation UX, animation, effect feedback, reduced motion, and no-legality-in-TypeScript boundary.

`PUBLIC-RELEASE-CHECKLIST.md` must be filled out, not left as a template shell.

`COMPETENT-PLAYER.md` must define what a competent casual Draughts Lite player should understand and how the UI teaches those concepts.

`BOT-STRATEGY-EVIDENCE-PACK.md` must demonstrate the modest Level 1 heuristics with test scenarios and must disclaim search strength.

`PRIMITIVE-PRESSURE-LEDGER.md` must record the board primitive decision, evidence, limits, and retrofit policy.

### Cross-repository docs

Update relevant top-level docs if the exact implementation changes them:

- `docs/MECHANIC-ATLAS.md` for the new game mechanics and board primitive.
- `docs/SOURCES.md` if it tracks official game source policy globally.
- `docs/ROADMAP.md` or equivalent status docs only if they already track gate progress at this commit.
- `specs/README.md` according to repository archival/spec workflow.
- `apps/web/e2e/NO-LEAK-A11Y-CHECKLIST.md` for Draughts Lite public accessibility/no-leak checks.

Do not introduce misleading gate-status claims into public docs; update status only where the gate's evidence actually supports it.

---

## R21. Public web integration acceptance

Gate 7 includes full web integration. A Rust-only implementation is not sufficient.

Acceptance requires:

- Draughts Lite appears in the web game picker.
- A standard match can be started from the UI.
- The board renders with all cells and pieces from Rust-projected view data.
- Pointer input can complete at least one quiet move.
- Pointer input can complete at least one forced capture.
- Pointer input can complete at least one multi-jump path.
- Keyboard input can select an origin and destination.
- Keyboard input can complete a forced continuation path.
- Legal origins and legal destinations are visibly and accessibly indicated.
- Forced continuation is unmistakable visually and through live-region text.
- Promotion feedback is visible and announced.
- Reduced-motion mode preserves semantic feedback.
- Effect log shows move/capture/promotion/terminal/bot events.
- Replay export displays complete multi-segment paths.
- No TypeScript code calculates legality.

---

## R22. Implementation constraints

The implementation must preserve these boundaries:

- Rust owns rules, legality, validation, state mutation, replay, bots, effects, and serialized views.
- TypeScript owns presentation, focus management, action-tree traversal, pending-path UI state, and input forwarding.
- WASM is a boundary, not a second rules engine.
- `engine-core` must not gain board, draughts, grid, capture, promotion, movement, or UI concepts.
- `game-stdlib` may gain only narrow board-space utilities justified by primitive pressure.
- Existing games must not regress.
- Existing golden traces must remain stable unless an intentional trace migration is documented and applied consistently.
- Public game docs must align with source/IP policy.
- CI must pass without weakening prior gates.
- No network multiplayer.
- No hidden information.
- No strong checkers AI claims.

---

## R23. Acceptance criteria

Gate 7 is accepted only when all applicable criteria pass.

### Crate, workspace, and registration

- `games/draughts_lite` exists.
- The crate is registered in the workspace.
- The game has manifest, variant data, fixture data, docs, tests, and benches following existing official-game conventions.
- `wasm-api` registers `draughts_lite` consistently with existing games.
- Tools that enumerate games include `draughts_lite`: simulate, replay-check, fixture-check, rule-coverage, bench-report, and CI workflow command lists. (`seed-reducer` and `trace-viewer` are game-opt-in — they currently support only `race_to_n` and `directional_flip` — so adding `draughts_lite` to them is optional, not required for the gate.)

### Rules

- Standard setup has 12 men per side on playable dark squares only.
- Non-playable cells can never contain pieces or legal moves.
- Men quiet-move diagonally forward only.
- Kings quiet-move diagonally one square in any diagonal direction.
- Men capture diagonally forward only by adjacent jump.
- Kings capture diagonally by adjacent jump in any diagonal direction.
- Captures are mandatory.
- Quiet moves are rejected when any capture exists.
- Multi-jump continuation is mandatory for the same piece.
- Multiple capture choices are allowed without maximum-capture legality.
- Promotion occurs on the far king row.
- Promotion during capture ends the sequence immediately.
- Opponent no pieces produces a win.
- Opponent no legal move produces a win.
- Invalid stale, non-active, occupied, non-playable, no-capture, illegal movement, illegal continuation, and promotion-ended-sequence diagnostics are stable and tested.

### Action tree and replay

- Initial and midgame legal action trees are deterministic.
- Root choices expose legal origins only.
- Destination children expose legal landings only.
- Forced continuation is represented as nested destination children.
- Leaf action paths validate and apply atomically.
- Partial continuation paths are rejected.
- Multi-jump replay is one command with multiple path segments.
- Existing one-segment traces for earlier games remain compatible.
- Golden traces cover all required scenarios.
- WASM-exported trace includes a multi-segment action path and validates natively.

### Effects

- Move, capture, promotion, forced capture, forced continuation, invalid diagnostic, terminal win, and bot action effects are emitted or exposed consistently.
- Effects are deterministic and public-safe.
- Web animation/effect feedback consumes Rust effects rather than computing rules.

### Bots

- Level 0 chooses legal complete paths recursively.
- Level 1 uses documented modest heuristics.
- Level 1 never requires minimax/search infrastructure.
- Bot tests cover capture preference, promotion preference, forced continuation completion, deterministic seed behavior, and legal path validation.
- Bot traces are stable.

### Fixtures, tests, and tools

- Standard fixture validates.
- Replay-check passes for all Draughts Lite golden traces.
- Rule-coverage recognizes Draughts Lite major clauses.
- Fixture-check supports the game.
- Simulate supports Draughts Lite bounded smoke without false failure on nonterminal action caps.
- Property tests cover invariants without requiring guaranteed termination.
- CI Gate 0/1/2 workflows pass after updating game lists.

### Benchmarks

- Draughts Lite native benchmarks exist.
- Benchmark report tooling recognizes the game.
- Thresholds, if committed, are calibrated from measured baselines and documented.
- PR bench smoke runs without relying on unstable runner-specific hard thresholds.

### Web and accessibility

- Web game picker includes Draughts Lite.
- Mouse and keyboard play work.
- Legal destinations and forced continuation are visually clear.
- Board focus is visible and deterministic.
- Screen-reader labels identify cells, pieces, legal status, selected origin, destinations, forced continuation, promotion, diagnostics, and terminal outcome.
- Reduced motion is respected.
- E2E smoke covers a basic playable path and at least one forced-capture path.
- Accessibility/no-leak checklist is updated.
- No TypeScript legality calculation is introduced.

### Primitive decision

- If the board primitive is promoted, `game-stdlib` tests prove it and Draughts Lite uses it.
- The primitive remains rule-agnostic and out of `engine-core`.
- Existing games are not destabilized by forced retrofit.
- `PRIMITIVE-PRESSURE-LEDGER.md` documents the decision.
- If promotion is rejected after implementation evidence, the rejection is explicitly documented with rationale and local helpers remain contained.

### Documentation

- All required Draughts Lite docs exist and are populated from templates.
- `SOURCES.md` cites adopted rules and non-goals.
- `RULES.md` is understandable by a player.
- `MECHANICS.md` is useful to future maintainers.
- `PUBLIC-RELEASE-CHECKLIST.md` is complete enough for public-polish admission.
- Top-level docs/checklists are updated where the repository convention requires it.

---

## R24. Risks and mitigations

| Risk | Mitigation |
|---|---|
| Multi-jump becomes several replay commands, making replay misleading. | Require one complete move command with multi-segment path and golden traces proving it. |
| TypeScript starts calculating diagonal/capture legality. | Export recursive Rust action tree and metadata; add no-legality E2E/checklist review. |
| `engine-core` gains board/checkers concepts. | Keep action tree generic; place any earned board-space utility in `game-stdlib`. |
| Primitive promotion overreaches. | Limit primitive to coordinates, dimensions, bounds, ids, iteration, and offsets. Keep rules local. |
| Promotion during capture is implemented like international draughts instead of English draughts. | Source it in `SOURCES.md`; add dedicated rule test and golden trace. |
| Random simulations fail due nonterminal cycles. | Treat action-cap reached as bounded smoke for Draughts Lite, not a fake draw or invariant failure. |
| Bot scope creeps into search. | Document Level 1 heuristics, exclude search methods, and keep tests scenario-based. |
| Public polish creates inaccessible animation. | Require reduced motion, live regions, keyboard grid behavior, and no color-only cues. |
| WASM replay export truncates paths. | Add multi-segment WASM-exported trace and regression tests. |
| Existing games regress due boundary changes. | Keep one-segment compatibility tests and avoid forced primitive retrofits. |

---

## R25. Out-of-scope future work

Future gates may consider:

- additional draughts variants;
- optional draw/adjudication packages;
- resign/draw-offer UI;
- board rotation by seat perspective;
- richer tutorials;
- stronger but still bounded bots;
- generalized preview schema improvements;
- retrofitting older games to `game-stdlib` board primitives;
- tournament/match-series tooling;
- analysis mode or legal-path explorer.

None of these should block Gate 7.

---

## R26. Final handoff notes

The core design choice is settled: use the existing recursive action tree and complete multi-segment action paths; do not invent a checkers-specific engine model. The hard parts are rule correctness, replay readability, WASM path preservation, and public-quality path construction in the UI.

A coding agent should start by making the native Rust game deterministic and traceable, then wire the WASM boundary and public UI without moving legality into TypeScript. The primitive promotion should be kept brutally small. If a proposed helper knows about captures, kings, playable dark squares, forced moves, or draughts terminology, it is not the generic primitive for this gate.

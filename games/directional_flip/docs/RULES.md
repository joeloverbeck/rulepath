# Directional Flip Rules

Game ID: `directional_flip`

Public display name: `Directional Flip`

Implemented variant: `directional_flip_standard`

Rules version: `directional_flip-rules-v1`

Prepared by: `Codex`

Created: 2026-06-07

Last updated: 2026-06-07

## Rule authority

This document is the original Rulepath rules summary for the implemented
variant. Sources belong in `SOURCES.md`; this document states the Rulepath
implementation contract.

Stable rule IDs are requirements. They must remain stable after implementation
unless intentionally migrated with a migration note and corresponding updates in
`RULE-COVERAGE.md`, traces, tests, and docs.

## Metadata

| Field | Value |
|---|---|
| game id | `directional_flip` |
| public display name | `Directional Flip` |
| variant | `directional_flip_standard` |
| rules version | `directional_flip-rules-v1` |
| source note | `games/directional_flip/docs/SOURCES.md` |
| coverage matrix | `games/directional_flip/docs/RULE-COVERAGE.md` |
| mechanic inventory | `games/directional_flip/docs/MECHANICS.md` |
| implementation admission | `games/directional_flip/docs/GAME-IMPLEMENTATION-ADMISSION.md` |

## Purpose and scope

| Rule ID | Rule statement | Notes |
|---|---|---|
| `DF-IP-001` | The public game name is `Directional Flip`, a neutral Rulepath name for the selected directional-bracketing shape. | Public UI and docs must not imply affiliation with any external source, federation, publisher, or commercial product. |
| `DF-VIEW-001` | `directional_flip` is a deterministic two-seat perfect-information game; public views, action trees, previews, diagnostics, effects, replay exports, bot explanations, DOM, logs, storage, and tests must not expose hidden or internal-only state. | The implemented game has no hidden state, but internal freshness/hash/debug details still stay out of public payloads unless a repo contract explicitly allows them. |
| `DF-UI-001` | Rust owns setup, legal actions, validation, forced-pass availability, placement, flips, terminal detection, scoring, semantic effects, bot choices, replay projections, and viewer-safe views. | TypeScript presents Rust output only and must not decide legality. |

## Implemented variant

| Rule ID | Rule statement | Source/rationale link |
|---|---|---|
| `DF-SETUP-001` | The only shipped variant is `directional_flip_standard`: an 8 by 8 board with exactly two seats and the four center cells occupied at setup. | `SOURCES.md#variant-choice-and-deviations` |
| `DF-ACTION-001` | Non-terminal turns expose Rust-generated placement actions only when the active seat has at least one legal placement. | `SOURCES.md#variant-choice-and-deviations` |

## Components and game-local vocabulary

Define only terms needed for this game. Game nouns belong here or in
`games/directional_flip`, not in `engine-core`.

| Rule ID | Term/component | Original Rulepath definition | Visibility | Notes |
|---|---|---|---|---|
| `DF-SETUP-001` | Board | The public 8 by 8 play area containing sixty-four stable cells. | public | Board vocabulary is game-local. |
| `DF-SETUP-001` | Cell | One board position identified by row and column, from `r1c1` through `r8c8`; row 1 is the top row and column 1 is the left column in the default public view. | public | Stable IDs are used by rules, traces, effects, replay, and UI accessibility labels. |
| `DF-SETUP-001` | Disc | The original Rulepath token owned by one seat and occupying one cell. | public | Discs must be visually distinct by more than color alone. |
| `DF-SETUP-001` | Seat | One of the two player positions: `seat_0` or `seat_1`. | public | Seat identity controls disc ownership, turn ownership, terminal winner, and bot viewpoint. |
| `DF-FLIP-004` | Direction | One of eight ordered scan directions: north, northeast, east, southeast, south, southwest, west, northwest. | public | Direction order controls preview and effect ordering. |

## Setup

| Rule ID | Setup rule | Deterministic/random? | Visibility | Notes |
|---|---|---|---|---|
| `DF-SETUP-001` | A new match starts with `seat_0` discs at `r4c5` and `r5c4`, `seat_1` discs at `r4c4` and `r5c5`, and every other cell empty. | deterministic | public | This is the fixed `directional_flip_standard` opening. |
| `DF-SETUP-001` | `seat_0` is the first active seat. | deterministic | public | No first-player randomization. |
| `DF-SETUP-001` | The initial legal placements for `seat_0` are exactly `r3c4`, `r4c3`, `r5c6`, and `r6c5`. | deterministic | public | This follows the row-1-top coordinate convention. |

## Turn, round, and phase sequence

| Rule ID | Situation/phase | Active seat(s) | Required sequence | Advances when |
|---|---|---|---|---|
| `DF-ACTION-001` | Normal turn with at least one legal placement. | Current active seat only. | The active seat selects one Rust-supplied placement action and submits it with the current freshness token. | The submitted action validates and applies. |
| `DF-ACTION-002` | Forced-pass turn with no legal placement and a non-terminal state. | Current active seat only. | The active seat selects the Rust-supplied forced-pass action. | The forced pass validates and applies. |
| `DF-PASS-001` | Applying a non-terminal forced pass. | System transition. | Rust records the pass, preserves the board, and changes the active seat to the other seat. | Immediately after the pass applies. |
| `DF-PASS-002` | Applying a second consecutive forced pass when neither seat can place. | System transition. | Rust records the pass, terminalizes the match, and exposes final counts and outcome. | Immediately after the pass applies. |

## Legal actions

Rust must generate legal actions. TypeScript must not decide legality.

| Rule ID | Situation | Legal action(s) | Action shape | Rust-owned validation notes |
|---|---|---|---|---|
| `DF-ACTION-001` | The game is non-terminal, the actor owns the current turn, and at least one empty cell brackets opposing discs. | Choose any Rust-supplied legal placement. | flat placement selection, one action per legal cell | Placement choices are sorted deterministically by cell id. |
| `DF-ACTION-002` | The game is non-terminal and the active seat has no legal placement. | Choose exactly one forced-pass action. | `pass/forced` or the repository-equivalent stable path | Forced pass is generated by Rust and is the only legal choice in this situation. |
| `DF-ACTION-003` | The active seat has at least one legal placement. | No pass action is legal. | not applicable | Pass must not appear when placement is available. |
| `DF-TERM-001` | The game is terminal. | No placement or forced-pass actions are available. | not applicable | Terminal views and replay surfaces preserve the final outcome. |

## Placement legality and restrictions

| Rule ID | Situation | Forced action/restriction | Illegal/stale diagnostic expectation | Notes |
|---|---|---|---|---|
| `DF-LEGAL-001` | A placement target is empty and at least one direction has one or more contiguous opposing discs followed by an own disc before the scan leaves the board or reaches an empty cell. | The placement is legal if it appears in the Rust-generated action tree. | not applicable | At least one bracketed line is required. |
| `DF-LEGAL-002` | A submitted placement targets an occupied cell. | Reject the submission without changing state. | Diagnostic is viewer-safe and identifies the occupied-target reason class. | Normal UI must not offer the action. |
| `DF-LEGAL-003` | A submitted placement targets an empty cell that flips no discs. | Reject the submission without changing state. | Diagnostic is viewer-safe and identifies the non-flipping reason class. | Empty alone is not enough. |
| `DF-LEGAL-004` | A submitted placement target is out of bounds, malformed, or unknown. | Reject the submission without changing state. | Diagnostic is viewer-safe and identifies the malformed or out-of-bounds reason class. | Stable cells are exactly `r1c1` through `r8c8`. |
| `DF-LEGAL-005` | A submitted command carries a stale freshness token. | Reject the submission without changing state. | Diagnostic is viewer-safe and identifies the stale-command reason class. | Freshness validation follows current engine patterns. |
| `DF-LEGAL-006` | A submitted command comes from a non-active actor. | Reject the submission without changing state. | Diagnostic is viewer-safe and identifies the wrong-actor reason class. | Bots and humans validate through the same command path. |
| `DF-ACTION-003` | TypeScript could infer legal cells from public data. | It still must not become the behavior authority. | not applicable | The browser may render Rust-provided data only. |

## Scoring and accounting

Placement and flip-resolution rules are included here because they determine the public disc counts used by terminal scoring.

| Rule ID | Rule | Timing | Tiebreaker/edge case | Notes |
|---|---|---|---|---|
| `DF-FLIP-001` | A valid placement creates one new disc for the acting seat at the target cell and flips every bracketed opposing disc in every qualifying direction. | During action application. | All qualifying directions are applied for the same placement. | The preview flip set and apply flip set must match. |
| `DF-FLIP-002` | A scan cannot skip over an own disc, empty cell, or board edge to create a flip. | During legality and application. | The first non-opposing cell in a direction determines whether that direction qualifies. | Only contiguous opposing runs can be bracketed. |
| `DF-FLIP-003` | Discs that are not in a direct qualifying line from the placed disc do not flip. | During action application. | Indirect enclosure, non-contiguous enclosure, and nearby-but-unbracketed discs are ignored. | No area-fill behavior exists. |
| `DF-FLIP-004` | Flip order is stable: directions are processed north, northeast, east, southeast, south, southwest, west, northwest; within a direction, discs are ordered nearest to farthest from the placed disc. | During preview and application. | The same order is used by semantic effects and replay evidence. | Stable ordering protects replay/hash determinism and UI animation. |
| `DF-PREVIEW-001` | Every legal placement preview names the target cell and the ordered cells that would flip; applying that action must flip exactly that set in exactly that order. | Before and during action application. | Preview grouping by direction may be included, but it must preserve the same ordered set. | Viewer-safe explanations are Rust-supplied. |
| `DF-SCORE-001` | Each valid placement updates the public disc counts for both seats. | Immediately after placement and flips apply. | Invalid submissions do not change counts. | Counts are public. |

## Terminal conditions

| Rule ID | Terminal condition | Outcome | Tie handling | Notes |
|---|---|---|---|---|
| `DF-END-001` | A placement fills the board. | Rust terminalizes the match after the placement resolves. | Final counts decide win or draw. | This is the `board_full` terminal trigger. |
| `DF-END-002` | A placement leaves no possible continuation for either seat before the board is full. | Rust terminalizes the match after explicit legal-placement checks. | Final counts decide win or draw. | This is the `no_continuation` terminal trigger. |
| `DF-END-003` | Both seats have no legal placement, proven by two consecutive forced passes. | The game ends immediately after the second forced pass. | Final counts decide win or draw. | This is the `double_forced_pass` terminal trigger; the double-pass trace is required evidence. |
| `DF-PASS-002` | Both seats have no legal placement, proven by two consecutive forced passes. | The game ends immediately after the second forced pass. | Final counts decide win or draw. | Legacy pass-rule reference for `DF-END-003`. |
| `DF-TERM-001` | A placement fills the board or otherwise leaves no possible continuation for either seat. | Rust terminalizes the match after explicit legal-placement checks. | Final counts decide win or draw. | Legacy terminal-rule reference for `DF-END-001` and `DF-END-002`. |
| `DF-SCORE-001` | `seat_0` or `seat_1` has a higher final disc count. | The seat with the higher count wins. | not applicable | Empty cells, if any remain after no-move terminal, are not awarded. |
| `DF-SCORE-002` | Both seats have the same final disc count. | The game ends in a draw. | Draw is the canonical wording. | Tie may appear only as secondary explanatory wording in docs. |

Terminal public views also expose a Rust-owned outcome rationale. Wins use template key `directional_flip.final_score_win`, decisive cause `final_score_comparison`, final score by seat, terminal trigger, and rule ID `DF-SCORE-001` plus the matching trigger ID `DF-END-001` through `DF-END-003`. Draws use template key `directional_flip.final_score_draw`, decisive cause `final_score_comparison`, final score by seat, terminal trigger, and rule ID `DF-SCORE-002` plus the matching trigger ID.

## Semantic effects, replay, and serialization

| Rule ID | Rule | Timing | Tiebreaker/edge case | Notes |
|---|---|---|---|---|
| `DF-EFFECT-001` | A valid placement emits semantic effects for placement acceptance, disc placement, grouped disc flips, turn advance, and terminal outcome when applicable. | During action application. | Forced pass emits pass and turn or terminal effects instead of placement effects. | Effect names may use repository-conformant exact names. |
| `DF-EFFECT-002` | The grouped flip effect contains ordered child entries matching `DF-FLIP-004`; each child records cell id, previous owner, new owner, direction, and order or distance. | During action application. | Empty flip groups are not emitted for legal placements. | Effect order must match preview/apply order. |
| `DF-REPLAY-001` | Replay evidence must bind setup, variant, command stream, effects, action tree, previews, public view, terminal outcome, and deterministic hashes. | Replay/export/import/step/reset. | Replaying the same commands from the same setup reproduces the same state and hashes. | Bot randomness is owned by the bot layer, not by game rules. |
| `DF-SER-001` | Serialized commands, state, traces, and fixtures reject unknown fields and behavior-looking fields according to current repo patterns. | During decode and validation. | Viewer-safe diagnostics identify the reason class without exposing internal state. | Serialization must not move behavior into data. |

## Visibility and private information

| Rule ID | Information | Visible to whom | When visible | Surfaces requiring protection | Notes |
|---|---|---|---|---|---|
| `DF-VIEW-001` | Board cells, disc owners, active seat, legal action metadata, Rust-provided previews, semantic effects, counts, terminal winner, and draw outcome. | all viewers | Always once state exists. | public view, action tree, preview, diagnostics, effect log, DOM, logs, storage, replay export, bot explanation, candidate ranking, dev inspector | The game is perfect-information; internal freshness/hash/debug details still require contract-safe exposure. |

## Bot-relevant non-authoritative strategy notes

These notes are not rule authority. Strategy claims must be checked against rule
IDs above.

| Rule ID | Situation | Practical note | Hidden-info limit |
|---|---|---|---|
| `DF-BOT-001` | Level 0 bot turn. | A random legal bot may select among Rust-supplied legal actions and submit through the normal command path. | No hidden information exists. |
| `DF-BOT-002` | Level 2-lite bot turn. | A deterministic authored policy may prefer legal moves using public features such as corners, mobility, frontier exposure, and immediate count pressure, then validate through the normal command path. | No hidden information exists; no search, playout, ML, RL, or runtime LLM move selection is allowed. |

## UI and accessibility rule notes

| Rule ID | Rule statement | Notes |
|---|---|---|
| `DF-UI-001` | The browser renders Rust legal choices, Rust previews, Rust effects, Rust counts, and Rust terminal outcome; it does not synthesize legal placement or pass choices. | Human and bot action paths share the Rust validation path. |
| `DF-UI-002` | Public UI must support keyboard grid interaction, forced-pass activation, reduced-motion behavior, and non-color-only state encoding. | Accessibility evidence belongs in later UI and smoke tickets. |

## Primitive-pressure rule note

| Rule ID | Rule statement | Notes |
|---|---|---|
| `DF-PRIM-001` | Directional Flip cannot exit Gate 6 until the primitive-pressure ledger decides the coordinate/direction/ray helper question. | The decision belongs in `PRIMITIVE-PRESSURE-LEDGER.md`, not in `engine-core`. |

## Known ambiguities and chosen resolutions

| Rule ID | Ambiguity | Chosen resolution | Source/rationale | Tests/traces required | Notes |
|---|---|---|---|---|---|
| `DF-IP-001` | Which public name to use for an Othello/Reversi-family directional-flipping game. | Use `Directional Flip`. | `SOURCES.md#public-naming-rationale` | Public docs and UI review. | Neutral project-owned naming avoids source confusion. |
| `DF-SETUP-001` | Which coordinate orientation to use. | Use `r1c1` through `r8c8`, with row 1 at the top and column 1 at the left. | Gate 6 spec and existing stable cell-id style. | Setup, action-id, trace, and UI tests. | This makes opening placements `r3c4`, `r4c3`, `r5c6`, and `r6c5`. |
| `DF-FLIP-004` | How to order multi-direction flips. | Use the documented direction order and nearest-to-farthest ordering within each direction. | Replay/hash determinism and UI effect stability. | Rule tests, effect tests, and golden traces. | The order is part of the game contract. |
| `DF-PASS-002` | How terminal no-move state is proven. | Use explicit Rust-generated forced pass for each seat; the second consecutive forced pass terminalizes. | Gate 6 spec. | Forced-pass and double-pass trace tests. | Pass is never browser-synthesized. |

## Rulepath deviations from common variants

| Rule ID | Common variant behavior | Rulepath behavior | Why | Public note needed? |
|---|---|---|---|---:|
| `DF-IP-001` | Some related games use trademarked names, branded rulebooks, tournament institutions, or recognizable commercial presentation. | Gate 6 implements only a neutral 8 by 8 directional-flipping game with original Rulepath prose, naming, and presentation. | Keeps the official game IP-conservative while proving the mechanic pressure. | yes |
| `DF-ACTION-002` | Some casual descriptions treat a no-move turn as an automatic skip. | Rulepath exposes a Rust-generated forced-pass command. | Replay, validation, effects, and UI must share one explicit command path. | yes |

## Explicit out-of-scope variants

| Rule ID | Out-of-scope variant/rule | Reason out of scope | Future review trigger |
|---|---|---|
| `DF-IP-001` | Othello-branded name, copied rulebook prose, copied board presentation, tournament branding, screenshots, scans, fonts, or proprietary assets. | Gate 6 must use neutral original Rulepath presentation. | A legal/IP review and accepted spec explicitly authorize reviewed assets or wording. |
| `DF-SETUP-001` | Alternate board sizes, alternate openings, handicaps, clocks, tournament tie-breaks, scored-empty-square variants, or custom setup variants. | Gate 6 proves one fixed official game and the directional-scan helper decision. | A later accepted spec requests a new variant. |
| `DF-BOT-002` | Minimax, alpha-beta, recursive search, MCTS, ISMCTS, Monte Carlo playouts, ML/RL, or runtime LLM move selection. | Public v1/v2 bot scope forbids advanced search and learning agents. | A later bot policy spec and ADR allow it. |

## Rule coverage link

The implementation and evidence mapping lives in `RULE-COVERAGE.md`.

Every rule ID in this document must appear in `RULE-COVERAGE.md`. Silent gaps
are not allowed.

## Rule-ID migration notes

| Old rule ID | New rule ID(s) | Reason | Coverage/traces updated? | Date |
|---|---|---|---:|---|
| not applicable | not applicable | Initial rule set. | not applicable | 2026-06-07 |

# Draughts Lite Rules

Game ID: `draughts_lite`

Public display name: `Draughts Lite`

Implemented variant: `draughts_lite_standard`

Rules version: `draughts_lite-rules-v1`

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
| game id | `draughts_lite` |
| public display name | `Draughts Lite` |
| variant | `draughts_lite_standard` |
| rules version | `draughts_lite-rules-v1` |
| source note | `games/draughts_lite/docs/SOURCES.md` |
| coverage matrix | `games/draughts_lite/docs/RULE-COVERAGE.md` |
| mechanic inventory | `games/draughts_lite/docs/MECHANICS.md` |
| implementation admission | `games/draughts_lite/docs/GAME-IMPLEMENTATION-ADMISSION.md` |

## Purpose and scope

| Rule ID | Rule statement | Notes |
|---|---|---|
| `DL-SCOPE-001` | `draughts_lite` is a two-seat, perfect-information, deterministic implementation of a small English draughts / American checkers rules subset. | The gate proves compound action trees, mandatory capture, and forced continuation without adding draughts nouns to `engine-core`. |
| `DL-SCOPE-002` | The public game presents a complete move as one Rust-validated replay command whose action path starts with an origin and ends with one quiet landing or one or more jump landings. | Partial path construction is UI state only; Rust owns legal next choices and validation. |
| `DL-SCOPE-003` | Tournament adjudication, clocks, agreement draws, repetition claims, huffing, maximum-capture requirements, and strong-engine play are outside this variant. | These exclusions are intentional Gate 7 scope limits. |

## Implemented variant

| Rule ID | Rule statement | Source/rationale link |
|---|---|---|
| `DL-VAR-001` | The only shipped variant is `draughts_lite_standard`: two seats play on an 8 by 8 board using only playable dark squares. | `SOURCES.md#variant-choice-and-deviations` |
| `DL-VAR-002` | Stable public cell IDs use `rNcM`, where row 1 is the top public row and column 1 is the left public column; playable dark squares are cells where `row + column` is odd. | `SOURCES.md#ambiguity-log` |
| `DL-VAR-003` | Men move and jump only toward the opponent's king row; kings move and jump one diagonal square in any direction. | `SOURCES.md#variant-choice-and-deviations` |

## Components and game-local vocabulary

Define only terms needed for this game. Game nouns belong here or in
`games/draughts_lite`, not in `engine-core`.

| Rule ID | Term/component | Original Rulepath definition | Visibility | Notes |
|---|---|---|---|---|
| `DL-COMP-001` | Board | The public 8 by 8 play area. | public | Board vocabulary is game-local or earned through `game-stdlib`, never `engine-core`. |
| `DL-COMP-002` | Cell | One board position identified by a stable `rNcM` ID. | public | Only dark-parity cells are playable in this variant. |
| `DL-COMP-003` | Piece | A public token owned by one seat and occupying one playable cell. | public | A piece is either a man or king. |
| `DL-COMP-004` | Man | An uncrowned piece that moves and jumps diagonally forward only. | public | Forward direction is seat-relative. |
| `DL-COMP-005` | King | A crowned piece that moves and jumps one diagonal square forward or backward. | public | No flying kings. |
| `DL-COMP-006` | Jump | A capture step from one playable cell over one adjacent opposing piece into the empty playable cell immediately beyond it. | public | Captured pieces are removed during action application. |
| `DL-COMP-007` | Action path segment | A stable string in a Rust-generated path: one origin segment followed by one or more landing segments. | public | Exact segment names are fixed by implementation tickets and coverage docs. |

## Setup

| Rule ID | Setup rule | Deterministic/random? | Visibility | Notes |
|---|---|---|---|---|
| `DL-SETUP-001` | A new `draughts_lite_standard` match starts with exactly two seats, `seat_0` and `seat_1`. | deterministic | public | No player-count variants. |
| `DL-SETUP-002` | Each seat starts with 12 men on the playable dark squares in its first three home rows. | deterministic | public | With row 1 at the top, one seat starts on rows 1 through 3 and the other on rows 6 through 8. |
| `DL-SETUP-003` | `seat_0` is the first active seat. | deterministic | public | No first-player randomization. |
| `DL-SETUP-004` | All setup state is public. | deterministic | public | Draughts Lite has no hidden setup information. |

## Turn, round, and phase sequence

| Rule ID | Situation/phase | Active seat(s) | Required sequence | Advances when |
|---|---|---|---|---|
| `DL-TURN-001` | Normal non-terminal turn. | Current active seat only. | The active seat submits one complete Rust-generated action path. | The path validates and applies. |
| `DL-TURN-002` | Quiet move turn with no capture available anywhere for the active seat. | Current active seat only. | The active seat chooses one legal origin and one legal empty diagonal landing. | The quiet move applies, promotion is resolved if applicable, terminal state is checked, and the turn advances if nonterminal. |
| `DL-TURN-003` | Capture turn with at least one capture available. | Current active seat only. | The active seat chooses a capture origin and follows Rust-generated jump landings until the path is complete. | The complete capture sequence applies atomically. |
| `DL-TURN-004` | Forced continuation after a jump. | Same active seat and same moving piece. | If the same piece has another legal jump and has not just been promoted, the next landing segment is mandatory. | Continuation ends when no same-piece jump remains or a man reaches the king row. |

## Legal actions

Rust must generate legal actions. TypeScript must not decide legality.

| Rule ID | Situation | Legal action(s) | Action shape | Rust-owned validation notes |
|---|---|---|---|---|
| `DL-ACTION-001` | The active seat has no legal capture. | Choose any Rust-supplied legal quiet move. | action tree: origin -> quiet landing | Men move one playable diagonal cell forward into an empty cell; kings move one playable diagonal cell in any diagonal direction into an empty cell. |
| `DL-ACTION-002` | The active seat has one or more legal captures. | Choose any Rust-supplied complete capture path. | action tree: origin -> jump landing -> optional forced jump landings | Quiet moves are absent while any capture exists. |
| `DL-ACTION-003` | A man jumps into the opponent's king row. | The capture path ends immediately at that landing. | terminal leaf for that move path | The man is promoted, but it may not continue jumping as a king in the same turn. |
| `DL-ACTION-004` | A king captures. | The king may jump one adjacent opposing piece in any diagonal direction into the empty cell beyond. | action-tree jump landing | No flying king movement or long-range capture exists. |
| `DL-ACTION-005` | The game is terminal. | No legal action choices are exposed. | empty action tree | Terminal views preserve the final outcome. |

## Forced actions and restrictions

| Rule ID | Situation | Forced action/restriction | Illegal/stale diagnostic expectation | Notes |
|---|---|---|---|---|
| `DL-RESTRICT-001` | At least one capture is available to the active seat. | Capturing is mandatory; all quiet moves are illegal and must not appear in the legal tree. | Viewer-safe diagnostic identifies the mandatory-capture reason class if a quiet path is submitted. | No maximum-capture rule is applied; the actor may choose among legal capture sequences. |
| `DL-RESTRICT-002` | A capture path reaches an intermediate landing where the same piece can capture again and has not just been promoted. | Same-piece continuation is mandatory; ending the command early is illegal. | Viewer-safe diagnostic identifies the incomplete-continuation reason class. | The UI may show pending path state, but only Rust determines whether more landings are required. |
| `DL-RESTRICT-003` | A submitted path contains an unknown, malformed, out-of-bounds, light-square, occupied-landing, non-adjacent, non-diagonal, stale, or wrong-actor segment. | Reject the command without changing state. | Viewer-safe diagnostic identifies the reason class without exposing internal-only data. | Normal UI must not offer invalid segments. |
| `DL-RESTRICT-004` | A player has several legal capture paths with different lengths. | Any Rust-supplied complete capture path is legal. | not applicable | Draughts Lite intentionally has no maximum-capture rule. |

## Scoring and accounting

Draughts Lite has no numeric score. Terminal accounting is the Rust-owned public piece count and legal-move availability used by the terminal rules.

## Movement, capture, and promotion resolution

| Rule ID | Rule | Timing | Tiebreaker/edge case | Notes |
|---|---|---|---|---|
| `DL-MOVE-001` | A quiet move transfers the moving piece from its origin to its landing cell without removing any opposing piece. | During action application. | Quiet moves exist only when no capture exists for the active seat. | Invalid submissions do not mutate state. |
| `DL-MOVE-002` | A jump moves the piece over one adjacent opposing piece and removes that opposing piece from the board. | During action application. | Each jump removes exactly one piece. | Captures in one command resolve in path order. |
| `DL-MOVE-003` | A man that reaches the opponent's king row is promoted to a king. | During action application at the landing that reaches the row. | If promotion happens during a capture sequence, the move ends immediately. | A newly crowned king may act as a king only on a later turn. |
| `DL-MOVE-004` | Applying a complete move checks terminal conditions after movement, capture removal, and promotion are resolved. | End of action application. | If the game is not terminal, active seat changes to the other seat. | Terminal outcome is Rust-owned. |

## Terminal conditions

| Rule ID | Terminal condition | Outcome | Tie handling | Notes |
|---|---|---|---|---|
| `DL-END-001` | After a completed action, the opponent has no pieces. | The acting seat wins. | not applicable | Piece count is public. |
| `DL-END-002` | After a completed action, the opponent has pieces but no legal move. | The acting seat wins. | not applicable | A blocked player loses; there is no stalemate draw. |
| `DL-END-003` | Draw, repetition, agreement, clock, referee, and long no-progress procedures. | Not implemented. | not applicable | These tournament adjudications are out of scope for Gate 7. |

Terminal public views also expose a Rust-owned outcome rationale. Wins use template key `draughts_lite.opponent_no_pieces` or `draughts_lite.opponent_no_legal_move`, the matching decisive cause, public piece-count breakdowns by seat, `losing_legal_move_count=0`, and rule ID `DL-END-001` or `DL-END-002`.

## Visibility and private information

Public/browser payloads must not reveal hidden information through public views,
action trees, previews, diagnostics, effect logs, DOM attributes, test IDs, logs,
local storage, replay exports, bot explanations, candidate rankings, or dev
inspectors.

| Rule ID | Information | Visible to whom | When visible | Surfaces requiring protection | Notes |
|---|---|---|---|---|---|
| `DL-VIS-001` | Board cells, playable-square status, piece owners, piece ranks, active seat, Rust legal tree, Rust path guidance, semantic effects, terminal winner, and replay commands. | all viewers | Always once state exists. | public view, action tree, preview, diagnostics, effect log, DOM, logs, storage, replay export, bot explanation, candidate ranking, dev inspector | The game has no hidden information; internal freshness/hash/debug details still stay within repo contracts. |
| `DL-VIS-002` | Bot choices and explanations. | all viewers when exposed by supported surfaces. | After bot action selection or replay export. | bot explanation, effect log, replay export, dev inspector | Bots must use only public state and the normal legal action API. |

## Replay and randomness notes

| Rule ID | Randomness/replay rule | Seed/log/hash requirement | Visibility | Notes |
|---|---|---|---|---|
| `DL-RNG-001` | Draughts Lite setup and rules contain no randomness. | No game-rule RNG is required. | public | Bot randomness, if any, belongs to the bot layer and is replay-evidenced through commands. |
| `DL-REPLAY-001` | A complete draughts move is recorded as one command with a multi-segment action path. | Replaying the same setup and commands must reproduce state, effects, public views, terminal outcome, and deterministic hashes. | public replay export, subject to normal trace contracts | Trace Schema v1 action paths are lists and do not require a schema bump. |
| `DL-REPLAY-002` | Partial origin or intermediate landing selections are not replay commands. | Only complete validated paths enter replay. | public UI state only before commit | This preserves atomic move semantics. |

## Bot-relevant non-authoritative strategy notes

These notes are not rule authority. Strategy claims must be checked against rule
IDs above.

| Rule ID | Situation | Practical note | Hidden-info limit |
|---|---|---|---|
| `DL-BOT-001` | Level 0 bot turn. | A random legal bot may choose among complete Rust-supplied legal paths and submit through the normal command path. | No hidden information exists. |
| `DL-BOT-002` | Level 1 bot turn. | A modest authored policy may prefer legal captures, promotions, king safety, and material balance using only public state, then validate through the normal command path. | No search, playout, learning, or runtime LLM move selection is allowed. |

## UI and accessibility rule notes

| Rule ID | Rule statement | Notes |
|---|---|---|
| `DL-UI-001` | The browser renders Rust legal origins, Rust legal next landings, Rust forced-continuation guidance, Rust diagnostics, Rust effects, and Rust terminal outcome; it does not calculate captures, diagonals, promotion, or legal continuation. | Human and bot actions share the Rust validation path. |
| `DL-UI-002` | Public UI must support pointer play, keyboard grid interaction, forced-continuation status, reduced-motion behavior, non-color-only state encoding, and accessible announcements. | UI evidence lands in later tickets. |

## Known ambiguities and chosen resolutions

| Rule ID | Ambiguity | Chosen resolution | Source/rationale | Tests/traces required | Notes |
|---|---|---|---|---|---|
| `DL-AMB-001` | Which public name to use for a checkers/draughts-family game. | Use `Draughts Lite` and `draughts_lite`. | `SOURCES.md#public-naming-rationale` | Public docs and UI review. | Neutral project naming avoids affiliation with a federation, publisher, or commercial presentation. |
| `DL-AMB-002` | Which board coordinate orientation and playable-square parity to use. | Use `r1c1` through `r8c8`, row 1 at the top, column 1 at the left, and dark playable cells where `row + column` is odd. | Gate 7 spec and existing stable cell-id style. | Setup tests, legal-tree tests, traces, and UI smoke. | This makes the coordinate convention deterministic and visible. |
| `DL-AMB-003` | Whether a man promoted by a capture may continue capturing immediately as a king. | The move ends immediately when the man reaches the king row. | WCDF English draughts rules summary in `SOURCES.md`. | Promotion-stop rule tests and golden trace. | This is a required Gate 7 compound-action edge case. |
| `DL-AMB-004` | Whether a player must choose the capture sequence with the most captures. | No maximum-capture rule is implemented; any complete Rust-supplied capture sequence is legal. | WCDF English draughts rules summary and Gate 7 scope. | Mandatory-capture and alternative-capture tests/traces. | Mandatory capture exists; maximum-capture does not. |

## Rulepath deviations from common variants

| Rule ID | Common variant behavior | Rulepath behavior | Why | Public note needed? |
|---|---|---|---|---:|
| `DL-VAR-004` | Some draughts variants use flying kings, backward captures by men, maximum-capture rules, larger boards, huffing, or tournament draw procedures. | Draughts Lite implements only the scoped English draughts subset in this file. | Gate 7 proves compound action trees without full chess-like exception load or tournament machinery. | yes |
| `DL-END-003` | Some settings allow draw claims or tournament adjudication. | Gate 7 has only win by opponent no pieces or opponent no legal move. | Keeps terminal logic small, deterministic, and replay-evidenced. | yes |

## Explicit out-of-scope variants

| Rule ID | Out-of-scope variant/rule | Reason out of scope | Future review trigger |
|---|---|---|
| `DL-VAR-005` | International draughts, Canadian draughts, Russian draughts, Brazilian draughts, giveaway/suicide variants, flying kings, backward-capturing men, huffing, and maximum-capture mandates. | They are not needed for the Gate 7 compound-action proof and would expand the exception load. | A later accepted spec requests a new variant. |
| `DL-VAR-006` | Clocks, ratings, resignation, agreement draws, referee procedures, repetition claims, opening books, endgame databases, and tournament scoring. | Gate 7 excludes tournament adjudication and strong-engine framing. | A later accepted spec and, where needed, ADR authorize them. |
| `DL-BOT-003` | Minimax, alpha-beta, MCTS, ISMCTS, Monte Carlo playouts, transposition tables, endgame databases, opening books, ML, RL, or runtime LLM move selection. | Public v1/v2 bot scope forbids search and learning agents. | A later bot policy spec and ADR allow it. |

## Rule coverage link

The implementation and evidence mapping lives in `RULE-COVERAGE.md`.

Every rule ID in this document must appear in `RULE-COVERAGE.md`. Silent gaps
are not allowed.

## Rule-ID migration notes

| Old rule ID | New rule ID(s) | Reason | Coverage/traces updated? | Date |
|---|---|---|---:|---|
| not applicable | not applicable | Initial rule set. | not applicable | 2026-06-07 |

# Column Four Competent Player Analysis

Game ID: `column_four`

Implemented variant: `column_four_standard`

Rules version checked: `column_four-rules-v1`

Prepared by: `Codex`

Date: 2026-06-06

## Purpose and authority

This document is original Rulepath strategy analysis for the implemented
variant. It feeds the Level 2 bot design.

This document is not rule authority. Strategy claims must be checked against
`RULES.md`. If a strategy claim conflicts with rules, the rules win and this
document must be corrected.

## Sources and consulted strategy references

| Source/reference | URL/reference | Date consulted | Source quality | Used for | Copied prose status | Notes |
|---|---|---|---|---|---|---|
| Rulepath Column Four rules | `games/column_four/docs/RULES.md` | 2026-06-06 | project authority | legal tactics and terminal conditions | none | Rule IDs are the strategy cross-check source. |
| Rulepath Gate 5 spec | `specs/gate-5-column-four-public-polish.md` | 2026-06-06 | project authority | Level 2 scope and exclusions | none | Defines the public bot as authored, bounded, and explainable. |
| Human analysis of the implemented 7 by 6 vertical four-in-a-row variant | not external | 2026-06-06 | human analysis | tactical priority order | none | Original prose; no external strategy guide copied. |

## Rules cross-check

| Strategy area | Rule IDs checked | Any uncertainty? | Notes |
|---|---|---:|---|
| Legal candidate set | `CF-ACTION-001`, `CF-ACTION-002`, `CF-GRAVITY-001` | no | The bot may consider only Rust legal columns and Rust-computed landing cells. |
| Immediate wins and blocks | `CF-END-001` through `CF-END-006` | no | Threats are evaluated from public board state and the next legal landing cell only. |
| Draw and terminal no-actions | `CF-END-005`, `CF-END-007` | no | Terminal states produce no bot action. |
| Visibility | `CF-VIS-001`, `CF-VIS-002` | no | The game is perfect-information; no hidden-state shortcut exists or is needed. |

## Competent-player summary

Competent Column Four play is tactical and public. A player should:

- take an immediate winning column when one exists;
- block an opponent column that would win on the next turn;
- avoid placing a piece that gives the opponent an immediate win;
- build threats that force the opponent to respond;
- prefer central columns when no urgent tactic exists because central pieces can
  participate in more four-cell lines;
- keep choices deterministic and explainable rather than relying on opaque
  scores.

## Phases and situations

| Phase/situation | What competent players notice | Important rule IDs | Notes |
|---|---|---|---|
| Empty and early board | Center columns create flexible future lines. | `CF-ACTION-001`, `CF-GRAVITY-001` | Center preference is a tie-break, never above wins or blocks. |
| Immediate terminal opportunity | A legal landing cell completes a four-cell line. | `CF-END-001` through `CF-END-004` | Winning now is the highest priority. |
| Opponent immediate threat | The opponent has a legal next landing that completes a line. | `CF-END-001` through `CF-END-004` | Blocking is second priority. |
| Midgame threat-building | A move extends visible two- or three-piece structures without allowing an immediate reply. | `CF-GRAVITY-001`, `CF-END-001` through `CF-END-004` | This remains bounded one-ply analysis. |
| Terminal state | No legal player move remains. | `CF-END-007` | The bot reports no action. |

## Immediate tactics

| Tactic | Situation | Why it matters | Rule IDs | Bot feature candidate? |
|---|---|---|---|---:|
| Win now | A legal column lands the bot's piece into a winning line. | It ends the game immediately. | `CF-ACTION-001`, `CF-GRAVITY-001`, `CF-END-001` through `CF-END-004` | yes |
| Block now | A legal column prevents the opponent from winning on their next turn. | It avoids immediate terminal loss. | `CF-ACTION-001`, `CF-GRAVITY-001`, `CF-END-001` through `CF-END-004` | yes |
| Avoid giving a win | A legal column would let the opponent immediately win after the bot moves. | It prevents avoidable tactical collapse. | `CF-GRAVITY-001`, `CF-END-001` through `CF-END-004` | yes |
| Extend a threat | A legal column increases the bot's visible contiguous structure. | It creates pressure when no forced tactic exists. | `CF-END-001` through `CF-END-004` | yes |

## Threats to block

| Threat | How a player detects it from visible information | Blocking response candidates | Rule IDs | Hidden-info risk |
|---|---|---|---|---|
| Vertical three with open top | Opponent owns the top three occupied cells in a non-full column's next landing sequence. | Drop in that same column if legal. | `CF-GRAVITY-001`, `CF-END-002` | none |
| Horizontal three with a legal landing gap | The opponent can complete four horizontally at their next landing cell. | Drop into the gap column if legal. | `CF-GRAVITY-001`, `CF-END-001` | none |
| Diagonal three with support | The opponent's next legal landing cell is supported by occupied cells below and completes a diagonal. | Drop in that supported column if legal. | `CF-GRAVITY-001`, `CF-END-003`, `CF-END-004` | none |

## Positional, resource, card, and tempo principles

| Principle type | Principle | Visible evidence | Rule IDs | Notes |
|---|---|---|---|---|
| positional | Prefer central columns after urgent tactics. | Column ids and board geometry are public. | `CF-COMP-002`, `CF-ACTION-001` | Suggested order: `c4`, `c3`, `c5`, `c2`, `c6`, `c1`, `c7`. |
| resource/accounting | not applicable | No score or resource total exists. | `CF-SCORE-001` | Winner/draw is the only result. |
| card/hand/deck | not applicable | No cards, hands, or decks exist. | `CF-VIS-001` | Perfect information. |
| tempo/initiative | Force the opponent to answer immediate threats. | Visible legal landing cells and line opportunities. | `CF-END-001` through `CF-END-004` | Bounded to immediate next-turn threats for Level 2. |
| risk/control | Avoid legal moves that create an immediate opponent win. | Visible opponent response columns. | `CF-GRAVITY-001`, `CF-END-001` through `CF-END-004` | This is not search beyond one opponent reply. |

## Common beginner mistakes

| Mistake | Why it is bad | How competent play avoids it | Rule IDs | Bot test implied? |
|---|---|---|---|---:|
| Ignoring an immediate win | It misses a terminal result. | Always check own winning columns first. | `CF-END-001` through `CF-END-004` | yes |
| Ignoring an immediate block | It allows the opponent to win next turn. | Check opponent winning columns second. | `CF-END-001` through `CF-END-004` | yes |
| Treating every empty cell as playable | Gravity means only the lowest empty cell in a column can receive the next piece. | Candidate extraction uses legal columns and Rust landing previews. | `CF-GRAVITY-001` | yes |
| Overvaluing edge columns early | Edges participate in fewer possible four-cell lines. | Use center preference as a late tie-break. | `CF-COMP-002` | yes |

## Risk posture

| Situation | Cautious posture | Aggressive posture | Recommended default | Notes |
|---|---|---|---|---|
| Immediate win exists | Take it. | Take it. | aggressive | Terminal win dominates all later priorities. |
| Immediate block needed | Block. | Block. | cautious | Avoiding terminal loss dominates threat-building. |
| No immediate tactic | Prefer safe central pressure. | Prefer a threat extension. | balanced | Default should avoid unsafe moves before extending. |

## Visible signals

| Signal | Visible to whom | Strategic meaning | Bot feature candidate | Notes |
|---|---|---|---:|---|
| Legal column list | all viewers | Candidate set. | yes | Comes from Rust action tree. |
| Landing preview | all viewers | The actual cell affected by a column. | yes | Comes from Rust projection/effects. |
| Three-in-a-row with legal fourth landing | all viewers | Immediate win or block. | yes | Must be checked for both seats. |
| Center column availability | all viewers | Positional tie-break. | yes | Only after higher priorities. |

## Hidden/private information boundary

| Information | Human seat sees? | Competent inference allowed? | Bot may use? | Forbidden peeking risk | Notes |
|---|---:|---:|---:|---|---|
| Full board occupancy | yes | yes | yes | none | Perfect information. |
| Legal action tree | yes | yes | yes | none | Rust-generated. |
| Future bot tie-break seed internals | no | no | no as game information | low | Seed may only break ties deterministically. |
| Candidate rankings in public explanation | no | no | no | medium | Public rationale must not list score arrays or ranked alternatives. |

## Inference allowed vs forbidden peeking

| Scenario | Allowed inference | Forbidden shortcut | Test implied |
|---|---|---|---|
| Detect immediate opponent win | Simulate each legal opponent landing from public board facts. | Inspecting an implementation-only candidate ranking from another module. | block-immediate-win test |
| Choose between equal safe columns | Use documented center order, then deterministic seeded tie-break. | Random nondeterministic selection. | deterministic tie-break test |
| Explain a move | Mention the chosen priority and visible fact. | Emit scores, candidate rankings, or internal arrays. | explanation no-leak test |

## Strategy examples

| Example ID | Situation | Candidate choices | Competent choice | Explanation | Rule IDs |
|---|---|---|---|---|---|
| `CF-S-EX-001` | Bot has three vertical pieces in `c4`, and `c4` is legal. | Any legal column. | `drop/c4` | Win now by completing a vertical line. | `CF-GRAVITY-001`, `CF-END-002` |
| `CF-S-EX-002` | Opponent will win in `c2` next turn unless blocked. | Any legal column. | `drop/c2` | Block the visible immediate threat. | `CF-GRAVITY-001`, `CF-END-001` through `CF-END-004` |
| `CF-S-EX-003` | No immediate win or block exists on an empty board. | `drop/c1` through `drop/c7`. | `drop/c4` | Prefer the center column after urgent tactics are absent. | `CF-ACTION-001` |

## Anti-examples

| Anti-example ID | Bad choice | Why it is bad | Rule IDs | Bot guard/test implied |
|---|---|---|---|---|
| `CF-S-BAD-001` | Choose `drop/c7` while `drop/c4` wins immediately. | It ignores a terminal win. | `CF-END-001` through `CF-END-004` | win-now priority test |
| `CF-S-BAD-002` | Choose a center move while the opponent has an immediate winning column. | It loses next turn. | `CF-END-001` through `CF-END-004` | block priority test |
| `CF-S-BAD-003` | Explain a move as `score=[10,8,4]`. | It exposes implementation detail instead of public reasoning. | `CF-VIS-001` | explanation no-leak test |

## Known hard problems

| Problem | Why hard | Out-of-scope for current bot? | Notes |
|---|---|---:|---|
| Perfect play | Requires search or solved-game knowledge. | yes | Public v1/v2 Level 2 is authored policy, not a solver. |
| Multi-turn trap planning | Requires deeper search to prove. | yes | Level 2 may recognize immediate threats only. |
| Difficulty tuning | Needs simulations and user-facing UX decisions. | yes | Start with one strong default policy. |

## Out-of-scope advanced strategy

| Strategy idea | Why out of scope | Future trigger |
|---|---|---|
| Minimax / negamax / alpha-beta search | Exceeds the Level 2 authored policy scope for Gate 5. | Accepted ADR or later Level 3 ticket. |
| MCTS, ISMCTS, or Monte Carlo playouts | Explicitly forbidden for public v1/v2. | Accepted ADR only. |
| Tablebase or perfect solver | Not needed for a beatable public product opponent. | Later research gate with ADR. |
| ML/RL/LLM move selection | Explicitly forbidden for public v1/v2. | Accepted ADR only. |

## Translation to candidate Level 2 bot features

| Candidate feature | Derived from | Visible to bot? | Used for | Hidden-info risk | Test implied |
|---|---|---:|---|---|---|
| Legal column candidates | Legal action tree | yes | candidate extraction | none | candidates-equal-legal-actions |
| Own immediate winning column | Immediate tactics | yes | priority slot 1 | none | chooses-winning-column |
| Opponent immediate winning column | Threats to block | yes | priority slot 2 | none | blocks-immediate-loss |
| Unsafe own move marker | Common mistakes | yes | priority slot 3 | none | avoids-giving-immediate-win |
| Center order | Positional principles | yes | tie-break | none | prefers-center-on-empty-board |
| Viewer-safe explanation fragment | Strategy examples | yes | explanation | none | explanation-no-leak |

## Tests implied by strategy claims

| Strategy claim | Rule IDs | Test type | Test name placeholder | Notes |
|---|---|---|---|---|
| Bot chooses only legal columns. | `CF-ACTION-001`, `CF-ACTION-002` | bot legality | `level2_choices_validate_for_many_states` | 008/009 |
| Immediate win dominates. | `CF-END-001` through `CF-END-004` | bot decision | `level2_takes_immediate_win` | 008 |
| Immediate block dominates center preference. | `CF-END-001` through `CF-END-004` | bot decision | `level2_blocks_immediate_loss` | 008 |
| Tie-break is deterministic under seed. | `CF-RNG-001` | determinism | `level2_fixed_seed_is_deterministic` | 008 |
| Explanation is viewer-safe. | `CF-VIS-001`, `CF-VIS-002` | no-leak/explanation | `level2_explanation_has_no_rankings` | 008 |

## Review checklist

- All strategy prose is original.
- Strategy claims are checked against `RULES.md`.
- Hidden-information boundaries are explicit.
- Allowed inference and forbidden peeking are separated.
- Examples and anti-examples are concrete enough to test.
- Candidate bot features are evidence, not hidden implementation state.
- This document is linked from the Level 2 bot evidence pack.

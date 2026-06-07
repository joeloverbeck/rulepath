# Draughts Lite Competent Player Analysis

Game ID: `draughts_lite`

Implemented variant: `draughts_lite_standard`

Rules version checked: `draughts_lite-rules-v1`

Prepared by: `Codex`

Date: 2026-06-07

## Purpose and authority

This document is original Rulepath strategy analysis for the implemented
Draughts Lite variant. It feeds the modest Level 1 bot design.

This document is not rule authority. Strategy claims must be checked against
`RULES.md`. If a strategy claim conflicts with rules, the rules win and this
document must be corrected.

## Sources and consulted strategy references

| Source/reference | URL/reference | Date consulted | Source quality | Used for | Copied prose status | Notes |
|---|---|---|---|---|---|---|
| Rulepath Draughts Lite rules | `games/draughts_lite/docs/RULES.md` | 2026-06-07 | project authority | legal moves, capture, promotion, terminal, bot boundary | none | Rule IDs are the strategy cross-check source. |
| Rulepath Draughts Lite sources | `games/draughts_lite/docs/SOURCES.md` | 2026-06-07 | project authority/source note | adopted and omitted rules, strong-engine exclusion context | none | Records consulted external rule and solved-game context. |
| Rulepath Gate 7 spec | `specs/gate-7-draughts-lite-compound-action-tree.md` | 2026-06-07 | project authority | Level 1 scope, exclusions, and required bot tests | none | §R17 defines the modest heuristic boundary. |
| Rulepath AI bot policy | `docs/AI-BOTS.md` | 2026-06-07 | project authority | public bot fairness, determinism, explainability, exclusions | none | Public bots are product opponents, not research engines. |

## Rules cross-check

| Strategy area | Rule IDs checked | Any uncertainty? | Notes |
|---|---|---:|---|
| Legal candidate set | `DL-ACTION-001`, `DL-ACTION-002`, `DL-ACTION-003`, `DL-RESTRICT-001`, `DL-RESTRICT-002` | no | The bot may consider only complete Rust legal action paths. |
| Mandatory capture and continuation | `DL-TURN-003`, `DL-TURN-004`, `DL-RESTRICT-001`, `DL-RESTRICT-002` | no | Captures and same-piece continuations are rules, not strategy. |
| Promotion | `DL-ACTION-003`, `DL-MOVE-003`, `DL-AMB-003` | no | Promotion during capture ends the move immediately. |
| Capture length | `DL-RESTRICT-004`, `DL-AMB-004` | no | Longer capture can be a heuristic, never a legality mandate. |
| Terminal wins | `DL-END-001`, `DL-END-002` | no | No-piece and no-legal-move wins are public and Rust-owned. |
| Visibility | `DL-VIS-001`, `DL-VIS-002` | no | Perfect information still forbids explanations that dump internals. |
| Bot boundary | `DL-BOT-001`, `DL-BOT-002`, `DL-BOT-003` | no | Level 1 is modest, deterministic, and non-search. |

## Competent-player summary

Competent Draughts Lite play is tactical and public. A player should:

- obey mandatory capture and same-piece continuation;
- treat a complete multi-jump as one move by one piece, not as separate turns;
- prefer captures that win material, especially when they also preserve future
  mobility;
- notice promotion chances and capture-to-promotion paths;
- value kings because they can move and capture in all four diagonal directions;
- avoid leaving a king exposed to an obvious immediate reply capture;
- recognize terminal chances: remove the opponent's last piece or leave the
  opponent with no legal move;
- remember that no maximum-capture rule exists, so the longest capture is only a
  heuristic preference when otherwise comparable;
- use Rust legal guidance rather than inferring legality from board geometry
  alone.

## Phases and situations

| Phase/situation | What competent players notice | Important rule IDs | Notes |
|---|---|---|---|
| Opening | Men are forward-only and quiet moves exist only when no capture is available. | `DL-ACTION-001`, `DL-ACTION-002`, `DL-VAR-003` | Early play is about piece development and avoiding simple capture exposure. |
| Capture available | Quiet moves disappear; the legal tree exposes capture origins and complete paths. | `DL-RESTRICT-001`, `DL-ACTION-002` | Strategy begins after mandatory compliance. |
| Forced continuation | The same piece must keep jumping until no legal continuation remains or promotion stops the capture. | `DL-TURN-004`, `DL-RESTRICT-002`, `DL-ACTION-003` | Partial paths are not replay commands. |
| Promotion race | Men near the opponent's king row can become kings. | `DL-MOVE-003`, `DL-ACTION-003` | Promotion increases future mobility, but promotion during capture stops that move. |
| King play | Kings can move and capture one diagonal step in any direction. | `DL-COMP-005`, `DL-ACTION-004` | Kings are valuable but can still be captured. |
| Material swing | Captures remove public opposing pieces. | `DL-MOVE-002`, `DL-END-001` | Material matters because no-piece terminal wins exist. |
| Blocked opponent | A completed action can win by leaving the opponent no legal move. | `DL-END-002` | This is a visible terminal tactic. |
| Terminal state | No legal action choices remain. | `DL-ACTION-005`, `DL-END-001`, `DL-END-002` | Bot reports no action after terminal. |

## Immediate tactics

| Tactic | Situation | Why it matters | Rule IDs | Bot feature candidate? |
|---|---|---|---|---:|
| Complete mandatory capture | Capture exists or continuation is required. | It is the only legal class of move. | `DL-RESTRICT-001`, `DL-RESTRICT-002` | yes |
| Take a terminal win | A legal action removes the last opposing piece or blocks all opponent moves. | It ends the game immediately. | `DL-END-001`, `DL-END-002` | yes |
| Prefer promotion | A legal action crowns a man. | Kings are more mobile and tactically flexible. | `DL-MOVE-003`, `DL-ACTION-004` | yes |
| Prefer capture-to-promotion | A capture path both wins material and crowns the mover. | Combines immediate material gain with future mobility. | `DL-MOVE-002`, `DL-MOVE-003` | yes |
| Prefer longer capture path as heuristic | Several complete capture paths are legal. | More captured pieces is often useful, but not mandatory. | `DL-RESTRICT-004`, `DL-AMB-004` | yes |
| Preserve a king from obvious reply capture | A king move leaves an immediate one-ply capture available to the opponent. | Kings are valuable and should not be trivially donated. | `DL-ACTION-004`, `DL-MOVE-002` | yes |

## Threats to block

| Threat | How a player detects it from visible information | Blocking response candidates | Rule IDs | Hidden-info risk |
|---|---|---|---|---|
| Immediate opponent capture of a king | After a candidate, the opponent legal tree contains a jump over the moved or exposed king. | Prefer an otherwise comparable candidate without that reply. | `DL-ACTION-002`, `DL-ACTION-004`, `DL-MOVE-002` | none |
| Opponent promotion | Opponent man is near the king row and has legal forward movement or capture support. | Capture it, block it, or improve own promotion if legal. | `DL-MOVE-003`, `DL-ACTION-001`, `DL-ACTION-002` | none |
| Material collapse | Candidate allows an obvious immediate capture sequence against several pieces. | Prefer a safer legal path after higher priorities tie. | `DL-MOVE-002`, `DL-RESTRICT-004` | none |
| Blocked no-move loss | Candidate leaves own pieces with poor mobility and exposes a future block. | Preserve legal moves when no higher tactic dominates. | `DL-END-002`, `DL-ACTION-001` | none |

## Positional, resource, card, and tempo principles

| Principle type | Principle | Visible evidence | Rule IDs | Notes |
|---|---|---|---|---|
| positional | Kings are more flexible than men. | Piece kind and legal action tree are public. | `DL-COMP-005`, `DL-ACTION-004` | Promotion preference is justified. |
| positional | Men near promotion are valuable. | Public row, owner, and direction. | `DL-MOVE-003`, `DL-VAR-003` | Promotion row is seat-relative. |
| resource/accounting | Captures change material and can end the game. | Public piece count and capture effects. | `DL-MOVE-002`, `DL-END-001` | Capture count is a heuristic after legality. |
| card/hand/deck | not applicable | No cards, hands, or decks exist. | `DL-VIS-001` | Perfect-information board game. |
| tempo/initiative | Mobility matters because no legal move loses. | Rust legal actions after a candidate. | `DL-END-002`, `DL-ACTION-005` | Use bounded one-ply checks only for Level 1. |
| risk/control | Do not donate a king to an obvious immediate capture. | Public successor and opponent legal tree. | `DL-ACTION-004`, `DL-MOVE-002` | One-ply local safety check, not search. |

## Common beginner mistakes

| Mistake | Why it is bad | How competent play avoids it | Rule IDs | Bot test implied? |
|---|---|---|---|---:|
| Trying a quiet move while capture exists | It is illegal. | Follow the Rust legal tree. | `DL-RESTRICT-001` | yes |
| Stopping a multi-jump early | It is illegal unless promotion stopped the move. | Submit only complete Rust paths. | `DL-RESTRICT-002`, `DL-ACTION-003` | yes |
| Treating longest capture as mandatory | It misstates this variant. | Consider length as a heuristic only. | `DL-RESTRICT-004`, `DL-AMB-004` | yes |
| Ignoring promotion | It misses future mobility and king value. | Prefer promotion when comparable. | `DL-MOVE-003`, `DL-ACTION-004` | yes |
| Giving away a king for no tactical reason | It loses a flexible piece. | Avoid obvious one-ply reply captures of kings. | `DL-ACTION-004`, `DL-MOVE-002` | yes |
| Explaining from raw hashes or scores | It leaks implementation detail and is not useful. | Explain visible tactical reasons. | `DL-VIS-001`, `DL-VIS-002` | yes |

## Risk posture

| Situation | Cautious posture | Aggressive posture | Recommended default | Notes |
|---|---|---|---|---|
| Mandatory capture | Complete a legal capture path. | Complete a legal capture path. | cautious | Law beats strategy. |
| Terminal win available | Take it. | Take it. | aggressive | Immediate win dominates. |
| Promotion available | Crown safely. | Crown even for material pressure. | balanced | Promotion preference is strong but below terminal/mandatory compliance. |
| Multiple capture paths | Prefer safer or longer path after terminal/promotion ties. | Prefer longest path. | balanced | No maximum-capture rule. |
| King exposed | Avoid obvious reply capture. | Accept only for terminal or clear material gain. | cautious | Level 1 uses one-ply local safety only. |

## Visible signals

| Signal | Visible to whom | Strategic meaning | Bot feature candidate | Notes |
|---|---|---|---:|---|
| Legal action tree | all viewers | Candidate set and complete path shape. | yes | Rust-generated. |
| Capture path length | all viewers | Material-gain heuristic. | yes | Not a legality mandate. |
| Promotion flag/landing row | all viewers | Future king value. | yes | Rust path metadata and public board. |
| Piece kind | all viewers | King/men mobility. | yes | Public view. |
| Opponent legal actions after one candidate | all viewers under rules | Immediate reply capture or no-move terminal. | yes | Bounded one-ply policy only. |
| Terminal outcome | all viewers | Game-ending priority. | yes | Rust-owned. |

## Hidden/private information boundary

| Information | Human seat sees? | Competent inference allowed? | Bot may use? | Forbidden peeking risk | Notes |
|---|---:|---:|---:|---|---|
| Full board occupancy | yes | yes | yes | none | Perfect information. |
| Legal action tree | yes | yes | yes | none | Rust-generated. |
| Public view, effects, terminal state | yes | yes | yes | none | Viewer-safe surfaces. |
| Replay hashes/freshness internals | no as strategy facts | no | no | medium | Contract data, not strategy. |
| Raw candidate rankings/score tuples | no | no | no | medium | May exist in tests, not public explanations. |
| Future bot seed tie-break key | no | no | no as game information | low | Seed only resolves equivalent choices deterministically. |

## Inference allowed vs forbidden peeking

| Scenario | Allowed inference | Forbidden shortcut | Test implied |
|---|---|---|---|
| Compare capture paths | Count captured pieces in legal paths and use path length as a heuristic. | Treat longest capture as mandatory. | capture-length heuristic test |
| Evaluate promotion | Use public landing row and Rust move metadata. | Hard-code a private promotion table outside rules. | promotion preference test |
| Avoid king blunder | Check one-ply public opponent legal captures after candidate. | Run minimax or search deeper tactical trees. | king-safety local test |
| Choose equal candidates | Use deterministic seeded tie-break over stable path identity. | Nondeterministic random choice. | fixed-seed determinism test |
| Explain a move | Mention visible capture, promotion, terminal, or king-safety reason. | Emit raw scores, hashes, candidate arrays, or search claims. | explanation no-leak test |

## Strategy examples

| Example ID | Situation | Candidate choices | Competent choice | Explanation | Rule IDs |
|---|---|---|---|---|---|
| `DL-S-EX-001` | A capture path is available. | quiet move or capture path submitted by UI/dev. | complete capture path | Capture is mandatory and quiet moves are illegal. | `DL-RESTRICT-001` |
| `DL-S-EX-002` | A path has `from/r3c2`, `jump/r5c4`, `jump/r7c6`. | partial or complete path. | complete path | Same-piece continuation must be completed. | `DL-RESTRICT-002`, `DL-SCOPE-002` |
| `DL-S-EX-003` | One legal move promotes a man and another quiet move does not. | two legal paths. | promotion path | A crowned piece has more future mobility. | `DL-MOVE-003`, `DL-ACTION-004` |
| `DL-S-EX-004` | Two complete capture paths are legal; one captures two pieces and one captures one. | two legal capture paths. | longer path if otherwise comparable | More captured material is useful, but not forced by rule. | `DL-RESTRICT-004`, `DL-MOVE-002` |
| `DL-S-EX-005` | A legal capture removes the opponent's last piece. | any legal path. | terminal capture | No opponent pieces means the acting seat wins. | `DL-END-001` |

## Anti-examples

| Anti-example ID | Bad choice | Why it is bad | Rule IDs | Bot guard/test implied |
|---|---|---|---|---|
| `DL-S-BAD-001` | Prefer a quiet move while any capture exists. | Illegal under mandatory capture. | `DL-RESTRICT-001` | candidate extraction equals legal tree |
| `DL-S-BAD-002` | Emit only `from/r3c2`, `jump/r5c4` when `jump/r7c6` is mandatory. | Partial continuation is illegal and not a replay command. | `DL-RESTRICT-002`, `DL-REPLAY-002` | no partial continuation path |
| `DL-S-BAD-003` | Claim the longest capture is required. | Draughts Lite intentionally has no maximum-capture rule. | `DL-RESTRICT-004` | explanation wording guard |
| `DL-S-BAD-004` | Explain "depth 4 says this wins." | Search is excluded for Level 1. | `DL-BOT-003` | explanation no-search guard |

## Known hard problems

| Problem | Why hard | Out-of-scope for current bot? | Notes |
|---|---|---:|---|
| Perfect checkers-family play | Solved-game strength needs search/database machinery. | yes | Level 1 must not claim engine strength. |
| Multi-turn traps | Requires search or deeper tactical analysis. | yes | Level 1 uses one-ply local checks only. |
| Draw adjudication and repetition | Not implemented in this variant. | yes | Gate 7 omits tournament draw procedures. |
| Opening books/endgames | Source-like engine data and strong-engine claims are excluded. | yes | No opening book or endgame database. |

## Out-of-scope advanced strategy

| Strategy idea | Why out of scope | Future trigger |
|---|---|---|
| Minimax, negamax, or alpha-beta search | Exceeds modest Level 1 and requires policy/ADR review. | Accepted ADR or later research gate. |
| MCTS, ISMCTS, or Monte Carlo playouts | Forbidden for public v1/v2. | Accepted ADR only. |
| Opening books or endgame databases | Strong-engine framing and external data are out of scope. | Later accepted spec and ADR. |
| ML/RL/runtime LLM move selection | Forbidden for public v1/v2. | Accepted ADR only. |

## Translation to candidate Level 1 bot features

| Candidate feature | Derived from | Visible to bot? | Used for | Hidden-info risk | Test implied |
|---|---|---:|---|---|---|
| Complete legal action paths | Legal action tree | yes | candidate extraction | none | candidates-equal-legal-tree |
| Terminal win detection | Immediate tactics | yes | priority slot | none | takes-terminal-win |
| Capture path length | Immediate tactics | yes | heuristic tie-break | none | prefers-longer-capture |
| Promotion/capture-to-promotion | Phases and situations | yes | priority slot | none | prefers-promotion |
| King preservation | Threats to block | yes | local safety tie-break | none | avoids-obvious-king-hang |
| Deterministic seeded tie-break | Risk posture | not game info | tie-break | none | fixed-seed-determinism |
| Viewer-safe explanation fragment | Strategy examples | yes | explanation | none | explanation-no-leak |

## Tests implied by strategy claims

| Strategy claim | Rule IDs | Test type | Test name placeholder | Notes |
|---|---|---|---|---|
| Bot chooses only complete legal action paths. | `DL-ACTION-001`, `DL-ACTION-002`, `DL-RESTRICT-002`, `DL-BOT-002` | bot legality | `level1_choices_validate_for_many_states` | GAT7DRALITCOM-012 |
| Level 1 prefers a terminal win. | `DL-END-001`, `DL-END-002` | bot decision | `level1_takes_terminal_win` | GAT7DRALITCOM-012 |
| Level 1 prefers promotion over comparable non-promotion. | `DL-MOVE-003`, `DL-ACTION-004` | bot decision | `level1_prefers_promotion` | GAT7DRALITCOM-012 |
| Level 1 prefers longer capture as heuristic only. | `DL-RESTRICT-004`, `DL-MOVE-002` | bot decision/explanation | `level1_prefers_longer_capture_without_claiming_required` | GAT7DRALITCOM-012 |
| Level 1 completes mandatory continuation. | `DL-RESTRICT-002`, `DL-REPLAY-002` | bot legality | `level1_never_emits_partial_continuation` | GAT7DRALITCOM-012 |
| Level 1 is deterministic under fixed seed. | `DL-RNG-001`, `DL-BOT-002` | determinism | `level1_fixed_seed_is_deterministic` | GAT7DRALITCOM-012 |
| Explanation is public-safe and non-search. | `DL-VIS-002`, `DL-BOT-003` | no-leak/explanation | `level1_explanation_has_no_scores_or_search_claims` | GAT7DRALITCOM-012 |

## Review checklist

- All strategy prose is original.
- Sources are recorded and not copied.
- Strategy claims are checked against `RULES.md`.
- Hidden-information boundaries are explicit.
- Allowed inference and forbidden peeking are separated.
- Examples and anti-examples are concrete enough to test.
- Candidate bot features are evidence, not hidden implementation state.
- This document is linked from the Level 1 bot evidence pack.

# Directional Flip Competent Player Analysis

Game ID: `directional_flip`

Implemented variant: `directional_flip_standard`

Rules version checked: `directional_flip-rules-v1`

Prepared by: `Codex`

Date: 2026-06-07

## Purpose and authority

This document is original Rulepath strategy analysis for the implemented
Directional Flip variant. It feeds the Level 2-lite bot design.

This document is not rule authority. Strategy claims must be checked against
`RULES.md`. If a strategy claim conflicts with rules, the rules win and this
document must be corrected.

## Sources and consulted strategy references

| Source/reference | URL/reference | Date consulted | Source quality | Used for | Copied prose status | Notes |
|---|---|---|---|---|---|---|
| Rulepath Directional Flip rules | `games/directional_flip/docs/RULES.md` | 2026-06-07 | project authority | legal actions, forced pass, flips, scoring, visibility | none | Rule IDs are the strategy cross-check source. |
| Rulepath Gate 6 spec | `specs/gate-6-directional-flip.md` | 2026-06-07 | project authority | Level 2-lite scope, exclusions, and policy outline | none | Requires strategy docs before Level 2-lite code. |
| Rulepath AI bot policy | `docs/AI-BOTS.md` | 2026-06-07 | project authority | Level 2 authored-policy boundary | none | Defines explainable, deterministic, non-search public bots. |
| Othello Belgium strategy tips | `https://en.othellobelgium.be/leer-othello/tips-en-strategie` | 2026-06-06 | strategy reference | mobility, corners, stability, frontier exposure, X/C-square caution | none | Used as non-authoritative strategy context only. |
| Nederlandse Othello Vereniging strategy guide | `https://www.othello.nl/content/guides/comteguide/strategy.html` | 2026-06-06 | strategy reference | phase-aware disc count, frontier, stability, and corner-adjacent danger | none | Used as non-authoritative strategy context only. |

## Rules cross-check

| Strategy area | Rule IDs checked | Any uncertainty? | Notes |
|---|---|---:|---|
| Legal candidate set | `DF-ACTION-001`, `DF-ACTION-002`, `DF-ACTION-003`, `DF-LEGAL-001` | no | The bot may consider only Rust legal placement or forced-pass actions. |
| Corners and corner-adjacent risk | `DF-SETUP-001`, `DF-LEGAL-001`, `DF-FLIP-001`, `DF-FLIP-004` | no | Corners matter because edge and corner occupancy are public board geometry, not special rules. |
| Mobility and pass pressure | `DF-ACTION-001`, `DF-ACTION-002`, `DF-PASS-001`, `DF-PASS-002` | no | Mobility is measured from Rust legal action generation. |
| Disc count and terminal score | `DF-SCORE-001`, `DF-SCORE-002` | no | Disc count is important late; early greed is only a late tie-break. |
| Visibility | `DF-VIEW-001` | no | The game is perfect-information, but public bot output still cannot expose internals. |
| Bot boundary | `DF-BOT-002` | no | Level 2-lite is authored one-ply visible-feature policy; no search/playouts/ML/RL/LLM. |

## Competent-player summary

Competent Directional Flip play is positional and public. A player should:

- take a legal corner when it is available because corner discs cannot be
  bracketed from beyond the corner;
- avoid playing X-squares and C-squares next to an open corner unless the move
  is forced or creates a stronger visible benefit;
- value mobility: keep own legal placements available while reducing the
  opponent's legal placements;
- prefer moves that extend from stable corners and edges without exposing many
  frontier discs;
- avoid greedy early disc-count grabs that leave many unstable frontier discs;
- use final disc count as a late-game tie-break once mobility and corner danger
  are less important;
- keep explanations simple and tied to visible board facts.

## Phases and situations

| Phase/situation | What competent players notice | Important rule IDs | Notes |
|---|---|---|---|
| Opening | Disc count can mislead; mobility and corner-adjacent safety matter more. | `DF-LEGAL-001`, `DF-ACTION-001` | Early large flips can create frontier exposure. |
| Open corner available | A legal corner is unusually valuable. | `DF-FLIP-001`, `DF-SCORE-001` | A corner cannot be outflanked from beyond the board. |
| Open corner with adjacent candidates | X-square and C-square moves may hand the corner to the opponent. | `DF-LEGAL-001`, `DF-FLIP-001` | These are danger signals, not illegal moves. |
| Mobility pressure | One move changes both seats' next legal action counts. | `DF-ACTION-001`, `DF-ACTION-002` | Level 2-lite may inspect one public successor. |
| Forced pass | No legal placement exists; the only action is `pass/forced`. | `DF-ACTION-002`, `DF-PASS-001` | Mandatory compliance beats all strategy. |
| Late board | Empty cells are fewer and final count becomes more predictive. | `DF-SCORE-001`, `DF-SCORE-002` | Disc-count preference stays below forced tactical priorities. |
| Terminal state | No legal action remains and final count decides win/draw. | `DF-PASS-002`, `DF-TERM-001`, `DF-SCORE-001`, `DF-SCORE-002` | Bot reports no action after terminal. |

## Immediate tactics

| Tactic | Situation | Why it matters | Rule IDs | Bot feature candidate? |
|---|---|---|---|---:|
| Mandatory forced pass | Rust action tree exposes only `pass/forced`. | The rules require it. | `DF-ACTION-002`, `DF-PASS-001` | yes |
| Take a legal corner | Candidate target is `r1c1`, `r1c8`, `r8c1`, or `r8c8`. | Corner discs are stable anchors and shape later edge play. | `DF-LEGAL-001`, `DF-FLIP-001` | yes |
| Avoid open-corner X/C danger | Candidate is adjacent to an empty corner. | It can make a valuable corner easier for the opponent. | `DF-LEGAL-001`, `DF-FLIP-001` | yes |
| Reduce opponent mobility | Candidate leaves the opponent with fewer legal placements. | Low mobility can force weak moves or passes. | `DF-ACTION-001`, `DF-ACTION-002` | yes |
| Preserve own mobility | Candidate leaves the bot with more next-turn legal placements. | More options reduce forced bad choices. | `DF-ACTION-001` | yes |

## Threats to block

| Threat | How a player detects it from visible information | Blocking response candidates | Rule IDs | Hidden-info risk |
|---|---|---|---|---|
| Opponent corner access | A candidate leaves an open corner as a legal opponent response. | Prefer a legal alternative that does not expose that corner. | `DF-LEGAL-001`, `DF-FLIP-001` | none |
| Opponent mobility swing | A candidate gives the opponent many legal responses while reducing own future options. | Prefer a legal move with better mobility balance. | `DF-ACTION-001` | none |
| Frontier overexposure | A candidate flips many discs adjacent to empty cells. | Prefer a legal move with fewer unstable frontier discs. | `DF-FLIP-001`, `DF-FLIP-004` | none |
| Late count deficit | Near the end, a candidate loses visible final-count pressure. | Prefer legal flips that improve count when no higher priority applies. | `DF-SCORE-001`, `DF-SCORE-002` | none |

## Positional, resource, card, and tempo principles

| Principle type | Principle | Visible evidence | Rule IDs | Notes |
|---|---|---|---|---|
| positional | Corners are strongest stable anchors. | Corner occupancy and legal corner targets are public. | `DF-FLIP-001`, `DF-SCORE-001` | Legal corner targets rank high. |
| positional | Avoid X/C squares near open corners. | Candidate cell and adjacent corner occupancy are public. | `DF-LEGAL-001`, `DF-FLIP-001` | This is a penalty below mandatory/legal priorities. |
| resource/accounting | Final disc count wins, but early count greed is a trap. | Counts and empty-cell count are public. | `DF-SCORE-001`, `DF-SCORE-002` | Count is phase-aware and late. |
| card/hand/deck | not applicable | No cards, hands, or decks exist. | `DF-VIEW-001` | Perfect-information board game. |
| tempo/initiative | Mobility is tempo: fewer opponent actions can force passes or weak placements. | Rust legal action counts are public. | `DF-ACTION-001`, `DF-ACTION-002`, `DF-PASS-001` | Use one-ply public successor only. |
| risk/control | Frontier discs next to empty cells are less stable. | Board occupancy and empty neighbors are public. | `DF-FLIP-001` | Bounded feature, not deep stability solving. |

## Common beginner mistakes

| Mistake | Why it is bad | How competent play avoids it | Rule IDs | Bot test implied? |
|---|---|---|---|---:|
| Taking the largest early flip | It can create unstable frontier discs and poor mobility. | Treat immediate flip count as a late tie-break, not an early priority. | `DF-FLIP-001`, `DF-SCORE-001` | yes |
| Playing next to an open corner | It can give the opponent corner access. | Penalize X/C squares near empty corners. | `DF-LEGAL-001`, `DF-FLIP-001` | yes |
| Ignoring legal pass | Pass is mandatory only when Rust exposes it. | Candidate extraction comes from the action tree. | `DF-ACTION-002`, `DF-PASS-001` | yes |
| Optimizing from raw state internals | It risks drift from viewer-safe public facts. | Use legal actions and public projections. | `DF-VIEW-001`, `DF-BOT-002` | yes |
| Explaining with raw scores | It is not useful public reasoning and may expose internals. | Explain the chosen visible priority. | `DF-VIEW-001`, `DF-BOT-002` | yes |

## Risk posture

| Situation | Cautious posture | Aggressive posture | Recommended default | Notes |
|---|---|---|---|---|
| Forced pass only | Pass. | Pass. | cautious | Mandatory rule compliance dominates. |
| Legal corner available | Take the corner. | Take the corner. | aggressive | Strong positional priority. |
| Open-corner danger | Avoid adjacent X/C square. | Accept only for clearly better visible outcome. | cautious | Default should be beatable but not careless. |
| Mobility tradeoff | Preserve options and reduce opponent options. | Maximize opponent denial. | balanced | Default uses mobility delta before frontier/count. |
| Late board | Improve final count if safe. | Maximize final count. | balanced | Count rises only when empty cells are low. |

## Visible signals

| Signal | Visible to whom | Strategic meaning | Bot feature candidate | Notes |
|---|---|---|---:|---|
| Legal action tree | all viewers | Candidate set. | yes | Rust-generated. |
| Placement preview | all viewers | Exact target and ordered flips. | yes | Rust-generated. |
| Corner occupancy/legal corner target | all viewers | Stable anchor opportunity or danger. | yes | Public board geometry. |
| X/C-square candidate near empty corner | all viewers | Potential corner concession. | yes | Public board geometry. |
| Own and opponent legal counts after one move | all viewers | Mobility pressure. | yes | One-ply public successor only. |
| Frontier count | all viewers | Exposure around empty cells. | yes | Bounded tie-break. |
| Empty-cell count | all viewers | Phase signal for count tie-break. | yes | Public board state. |
| Disc counts | all viewers | Terminal scoring and late pressure. | yes | Late tie-break only. |

## Hidden/private information boundary

| Information | Human seat sees? | Competent inference allowed? | Bot may use? | Forbidden peeking risk | Notes |
|---|---:|---:|---:|---|---|
| Full board occupancy | yes | yes | yes | none | Perfect information. |
| Legal action tree | yes | yes | yes | none | Rust-generated. |
| Public view counts and terminal state | yes | yes | yes | none | Rust-generated. |
| Replay hashes/freshness internals | no as strategy facts | no | no | medium | May exist for contracts, not strategic evaluation. |
| Raw candidate score arrays | no | no | no | medium | Not public explanation content. |
| Future seeded tie-break internals | no | no | no as game information | low | Seed only resolves equivalent candidates deterministically. |

## Inference allowed vs forbidden peeking

| Scenario | Allowed inference | Forbidden shortcut | Test implied |
|---|---|---|---|
| Count mobility after a candidate | Apply the public rules to a one-ply successor and count Rust legal actions. | Read an implementation-only ranking table or bypass validation. | mobility-priority test |
| Identify X/C danger | Use candidate coordinates and empty corner occupancy. | Special-case behavior from static data conditions. | corner-danger test |
| Choose equal candidates | Use documented bounded tie-breaks and deterministic seed. | Nondeterministic random choice. | fixed-seed determinism test |
| Explain a move | Mention corner, mobility, frontier, or late count from visible facts. | Emit raw score vectors, hashes, private debug state, or search claims. | explanation no-leak test |

## Strategy examples

| Example ID | Situation | Candidate choices | Competent choice | Explanation | Rule IDs |
|---|---|---|---|---|---|
| `DF-S-EX-001` | `r1c1` is legal. | Any legal placement. | `place/r1c1` | Take the legal corner because it is a stable anchor. | `DF-LEGAL-001`, `DF-FLIP-001` |
| `DF-S-EX-002` | `r2c2` is legal while `r1c1` is empty and another safe move exists. | `place/r2c2` or safe alternative. | safe alternative | Avoid the X-square next to an open corner. | `DF-LEGAL-001`, `DF-FLIP-001` |
| `DF-S-EX-003` | No corner tactic exists; one move leaves the opponent with two legal actions and another leaves six. | any legal placement | lower-opponent-mobility move | Reduce the opponent's visible mobility. | `DF-ACTION-001` |
| `DF-S-EX-004` | Two late moves are otherwise equal and only eight empty cells remain. | two legal placements | better final-count swing | Late count pressure matters after higher priorities tie. | `DF-SCORE-001`, `DF-SCORE-002` |

## Anti-examples

| Anti-example ID | Bad choice | Why it is bad | Rule IDs | Bot guard/test implied |
|---|---|---|---|---|
| `DF-S-BAD-001` | Choose a large early flip while a legal corner is available. | It ignores a stronger positional anchor. | `DF-FLIP-001`, `DF-SCORE-001` | corner priority test |
| `DF-S-BAD-002` | Choose an X-square next to an empty corner when a safe move exists. | It can concede a corner. | `DF-LEGAL-001`, `DF-FLIP-001` | open-corner danger test |
| `DF-S-BAD-003` | Choose a move that gives the opponent many legal responses while another equal move reduces mobility. | It loses tempo. | `DF-ACTION-001` | mobility delta test |
| `DF-S-BAD-004` | Explain a move as `score=[8,3,-2]`. | It exposes implementation detail rather than public reasoning. | `DF-VIEW-001`, `DF-BOT-002` | explanation no-leak test |

## Known hard problems

| Problem | Why hard | Out-of-scope for current bot? | Notes |
|---|---|---:|---|
| True stable-disc solving | Exact stability can require deeper board analysis. | yes | Level 2-lite uses bounded corner/edge/frontier approximations. |
| Multi-turn sacrifice planning | Often needs search. | yes | Public v1/v2 forbids search/playouts here. |
| Perfect late-game play | Requires deeper endgame calculation. | yes | Level 2-lite uses phase-aware count tie-breaks only. |
| Difficulty tuning | Needs simulations and product feedback. | yes | Start with one deterministic default policy. |

## Out-of-scope advanced strategy

| Strategy idea | Why out of scope | Future trigger |
|---|---|---|
| Minimax, negamax, or alpha-beta search | Exceeds Level 2-lite and needs ADR. | Accepted ADR or later Level 3 ticket. |
| MCTS, ISMCTS, or Monte Carlo playouts | Forbidden for public v1/v2. | Accepted ADR only. |
| Exact stable-disc solver or opening book | Too solver-like for a beatable public Level 2-lite bot. | Later research gate with evidence. |
| ML/RL/LLM move selection | Forbidden for public v1/v2. | Accepted ADR only. |

## Translation to candidate Level 2 bot features

| Candidate feature | Derived from | Visible to bot? | Used for | Hidden-info risk | Test implied |
|---|---|---:|---|---|---|
| Legal action candidates | Legal action tree | yes | candidate extraction | none | candidates-equal-legal-actions |
| Forced-pass compliance | Immediate tactics | yes | priority slot 1 | none | forced-pass-only-choice |
| Corner target | Positional principles | yes | priority slot 2 | none | chooses-legal-corner |
| Open-corner X/C danger | Common mistakes | yes | priority slot 3 | none | avoids-open-corner-adjacent |
| Opponent mobility count | Mobility principles | yes | priority slot 4 | none | reduces-opponent-mobility |
| Own mobility count | Mobility principles | yes | priority slot 5 | none | preserves-own-mobility |
| Frontier exposure | Risk/control principles | yes | bounded tie-break | none | frontier-score-bounded |
| Phase-aware disc delta | Resource/accounting principles | yes | late tie-break | none | late-count-tie-break |
| Viewer-safe explanation fragment | Strategy examples | yes | explanation | none | explanation-no-leak |

## Tests implied by strategy claims

| Strategy claim | Rule IDs | Test type | Test name placeholder | Notes |
|---|---|---|---|---|
| Bot chooses only legal action paths. | `DF-ACTION-001`, `DF-ACTION-002`, `DF-BOT-002` | bot legality | `level2_choices_validate_for_many_states` | GAT6DIRFLI-011 |
| Forced pass is mandatory when only pass exists. | `DF-ACTION-002`, `DF-PASS-001` | bot decision | `level2_takes_forced_pass` | GAT6DIRFLI-011 |
| Legal corners dominate ordinary count gain. | `DF-FLIP-001`, `DF-SCORE-001` | bot decision | `level2_prefers_legal_corner` | GAT6DIRFLI-011 |
| Open-corner X/C danger is penalized. | `DF-LEGAL-001`, `DF-FLIP-001` | bot decision | `level2_avoids_open_corner_adjacent_square` | GAT6DIRFLI-011 |
| Mobility changes affect priority. | `DF-ACTION-001` | bot decision | `level2_prefers_better_mobility_delta` | GAT6DIRFLI-011 |
| Tie-break is deterministic under seed. | `DF-BOT-002` | determinism | `level2_fixed_seed_is_deterministic` | GAT6DIRFLI-011 |
| Explanation is viewer-safe. | `DF-VIEW-001`, `DF-BOT-002` | no-leak/explanation | `level2_explanation_has_no_rankings_or_scores` | GAT6DIRFLI-011 |

## Review checklist

- All strategy prose is original.
- Sources are recorded and not copied.
- Strategy claims are checked against `RULES.md`.
- Hidden-information boundaries are explicit.
- Allowed inference and forbidden peeking are separated.
- Examples and anti-examples are concrete enough to test.
- Candidate bot features are evidence, not hidden implementation state.
- This document is linked from the Level 2 bot evidence pack.

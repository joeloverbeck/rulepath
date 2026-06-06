# Column Four Bot Strategy Evidence Pack

Game ID: `column_four`

Implemented variant: `column_four_standard`

Rules version: `column_four-rules-v1`

Bot target: Level 2 authored policy

Policy name/version: `column_four_tactical_v1`

Prepared by: `Codex`

Date: 2026-06-06

## Purpose and gate

This is the formal design input for the Column Four Level 2 authored-policy
bot. The executable bot must not drift beyond this pack without updating the
pack and tests.

This pack consumes `COMPETENT-PLAYER.md`; it does not replace it. It also does
not replace `AI.md`, which will be filled after the bot implementation lands.

The policy must be deterministic under seed, rules version, policy version,
input view, and declared limits. It must use the legal action API. It must
produce viewer-safe explanations.

## Explicit public v1/v2 exclusions

The Level 2 public bot must not use:

- omniscient state;
- hidden-state shortcuts;
- future random outcomes;
- unbounded weight soup;
- static data tactical conditions;
- random blunder injection by default;
- minimax;
- negamax;
- alpha-beta search;
- public v1/v2 MCTS;
- public v1/v2 ISMCTS;
- public v1/v2 Monte Carlo or monte carlo playout bots;
- tablebase or perfect solver lookup;
- public v1/v2 ML/RL;
- runtime LLM move selection.

Future minimax, negamax, alpha-beta, MCTS, ISMCTS, Monte Carlo, tablebase,
ML/RL, or LLM work requires an ADR under `docs/FOUNDATIONS.md` and
`docs/AI-BOTS.md`.

## Source documents consumed

| Document/source | Path/reference | Required? | Status | Notes |
|---|---|---:|---|---|
| `RULES.md` | `games/column_four/docs/RULES.md` | yes | read | Rule IDs define legal actions, gravity, terminal conditions, and visibility. |
| `COMPETENT-PLAYER.md` | `games/column_four/docs/COMPETENT-PLAYER.md` | yes | read | Direct strategy input for priority vector. |
| Gate 5 spec | `specs/gate-5-column-four-public-polish.md` | yes | read | Requires Level 2 authored policy and excludes search/ML/RL. |
| `docs/AI-BOTS.md` | `docs/AI-BOTS.md` | yes | read | Defines Level 2, lexicographic priorities, explanations, and exclusions. |
| `RULE-COVERAGE.md` | `games/column_four/docs/RULE-COVERAGE.md` | yes | incomplete | Lands after implementation coverage in GAT5COLFOUPUB-013. |
| `MECHANICS.md` | `games/column_four/docs/MECHANICS.md` | yes | incomplete | Lands in trailing docs. |
| `AI.md` | `games/column_four/docs/AI.md` | yes | incomplete | Lands after bot implementation. |

## Exact bot input view

| Input item | Included? | Source | Visible to acting seat? | Notes/no-leak test |
|---|---:|---|---:|---|
| legal action tree/action paths | yes | Rust legal action API | yes | Candidate set must equal legal actions. |
| public view | yes | Rust `PublicView` projection | yes | Includes board, legal columns, terminal state, and landing previews. |
| acting seat private view | no | not applicable | not applicable | Perfect-information game. |
| command/effect history visible to seat | optional | public effect log | yes | May be used only for explanation context, not hidden facts. |
| policy seed/tie-break state | yes | bot framework | not game information | Used only after higher priorities tie. |
| hidden opponent/private state | no | forbidden | no | No such game state exists. |
| future random outcomes | no | forbidden | no | Game rules have no randomness. |
| dev/test full state | no | forbidden for public bot | no | Tests may inspect state; public bot may not depend on dev-only shortcuts. |

## Legal action API used

| API/contract | Purpose | Determinism requirement | Tests |
|---|---|---|---|
| `legal_action_tree(state, actor)` | Enumerate legal column action paths. | Same state and actor produce same ordered candidate list. | `level2_candidates_match_legal_action_tree` |
| `validate_command(state, command)` | Submit chosen action path through normal validation. | Bot decision must validate before application. | `level2_chosen_action_validates` |
| `project_view(state, viewer)` | Consume viewer-safe board, legal target, terminal, and landing data. | Same state produces same stable view. | `level2_uses_viewer_safe_projection` |

## Candidate extraction plan

| Candidate group | Extraction rule | Rule IDs | Visible facts used | Hidden-info risk | Tests |
|---|---|---|---|---|---|
| Legal columns | One candidate per Rust legal action segment. | `CF-ACTION-001`, `CF-ACTION-002` | Action tree, column summaries. | none | candidate count equals legal action count |
| Own wins | For each legal candidate, apply the Rust landing model to see if the acting seat completes a line. | `CF-GRAVITY-001`, `CF-END-001` through `CF-END-004` | Public board, landing cell, line rules. | none | immediate win fixture |
| Opponent wins to block | For each legal opponent response after current state, detect next-turn wins. | `CF-GRAVITY-001`, `CF-END-001` through `CF-END-004` | Public board and legal landings only. | none | immediate block fixture |
| Safety | Mark candidates that allow an immediate opponent win after the bot move. | `CF-END-001` through `CF-END-004` | One-ply public successor only. | none | avoid-giving-win fixture |
| Positional tie-break | Apply documented center order after tactical priorities. | `CF-COMP-002` | Column id only. | none | empty-board center fixture |

## Phase model

| Phase/situation | Detection from allowed input | Policy node(s) active | Rule IDs | Notes |
|---|---|---|---|---|
| Terminal | Public terminal view is win or draw. | no-action | `CF-END-007` | Return no decision. |
| Immediate win available | Candidate creates terminal win. | win-now | `CF-END-001` through `CF-END-004` | Highest priority. |
| Immediate block required | Opponent has a next-turn winning column. | block-now | `CF-END-001` through `CF-END-004` | Second priority. |
| No forced tactic | No own win or opponent win exists. | safety, threat extension, center | `CF-ACTION-001`, `CF-GRAVITY-001` | Bounded one-ply evaluation only. |

## Lexicographic priority vector

Earlier slots dominate later slots.

| Slot | Priority | Higher/better value | Source evidence | Rule IDs | Tests | Explanation fragment |
|---:|---|---|---|---|---|---|
| 1 | terminal win | Candidate wins immediately. | `COMPETENT-PLAYER.md#immediate-tactics` | `CF-END-001` through `CF-END-004` | `level2_takes_immediate_win` | "This column wins now." |
| 2 | block immediate loss | Candidate occupies the opponent's immediate winning landing. | `COMPETENT-PLAYER.md#threats-to-block` | `CF-END-001` through `CF-END-004` | `level2_blocks_immediate_loss` | "This blocks a visible immediate threat." |
| 3 | safe move | Candidate does not allow an immediate opponent win. | `COMPETENT-PLAYER.md#common-beginner-mistakes` | `CF-GRAVITY-001` | `level2_avoids_giving_immediate_win` | "This keeps the opponent from winning next turn." |
| 4 | extend own threat | Candidate increases visible own line pressure. | `COMPETENT-PLAYER.md#immediate-tactics` | `CF-END-001` through `CF-END-004` | `level2_extends_visible_threat` | "This builds a visible threat." |
| 5 | deny opponent structure | Candidate reduces a visible opponent threat when no immediate block exists. | `COMPETENT-PLAYER.md#threats-to-block` | `CF-END-001` through `CF-END-004` | `level2_denies_visible_pressure` | "This interrupts opposing pressure." |
| 6 | center preference | Prefer `c4`, then `c3`, `c5`, `c2`, `c6`, `c1`, `c7`. | `COMPETENT-PLAYER.md#positional-resource-card-and-tempo-principles` | `CF-COMP-002` | `level2_prefers_center_on_empty_board` | "No urgent tactic exists, so I prefer central pressure." |
| 7 | bounded one-ply score | Small named tie-break score after slots 1-6 tie. | this pack | `CF-GRAVITY-001` | `level2_bounded_score_is_in_range` | "The visible tie-break favored this column." |
| 8 | deterministic seeded tie-break | Stable order from seed and candidate identity. | `docs/AI-BOTS.md` | not applicable | `level2_fixed_seed_is_deterministic` | "Equivalent choices were resolved deterministically." |

## Bounded scoring tie-breakers

Small scoring is allowed only after higher lexicographic categories.

| Score term | Range | Meaning | Used after slots | Visible inputs | Tests | Explanation text |
|---|---:|---|---|---|---|---|
| `center_distance_bonus` | `0..3` | Closer to `c4` is better. | 1-5 | Column id. | center preference tests | "central pressure" |
| `own_two_extension` | `0..2` | Candidate contributes to visible own two-piece lines. | 1-5 | Public board and landing cell. | bounded score range test | "builds a visible threat" |
| `opponent_two_denial` | `0..2` | Candidate occupies a visible opponent line-building point. | 1-5 | Public board and landing cell. | bounded score range test | "interrupts opposing pressure" |

Forbidden weight soup examples:

- dozens of magic weights with no priority rationale;
- style implemented only by multiplying weights;
- tactical conditions hidden in static data;
- scores that cannot produce clear explanations;
- tuning without simulations and benchmark evidence.

## Deterministic seeded tie-break

| Item | Decision |
|---|---|
| seed source | Bot seed passed by the bot framework; not game-rule RNG. |
| tie-break input identity | policy id, rules version, active seat, ply, freshness token, action segment. |
| stable ordering rule | Sort by lexicographic priority vector, bounded score tuple, center order, then deterministic seeded key. |
| reproducibility tests | `level2_fixed_seed_is_deterministic`, replay/hash bot test in 008/010. |
| replay/hash interaction | Chosen action path and explanation are replayed as public effects. |

Tie-break randomness must be deterministic. Random blunder injection is forbidden
by default.

## Style profile hooks

One strong default bot comes first.

| Profile | Variation | Must not affect | Hidden-info safe? | Tests |
|---|---|---|---:|---|
| default | Balanced tactical policy. | legality, hidden-info boundary, determinism, terminal priorities. | yes | Level 2 bot tests. |
| future cautious | May prefer safety before non-forced threat extension. | win-now, block-now, legal action API. | yes | future ticket only. |
| future aggressive | May prefer threat extension after safety is satisfied. | win-now, block-now, legal action API. | yes | future ticket only. |

## Forbidden hidden information

| Information | Why forbidden | Potential leak surface | Required no-leak test |
|---|---|---|---|
| candidate ranking array | Not needed in public explanation. | bot explanation, replay export, DOM | explanation no-leak |
| raw score vector | Implementation detail, not public reasoning. | bot explanation, dev inspector | explanation no-leak |
| dev/test full state shortcut | Public bot must consume allowed view/legal API. | bot input, candidate features | bot input audit |
| future random or sampled outcome | Not part of game rules or legal view. | tie-break, explanation | determinism test |

## Memory and belief model

| Item | Decision | Allowed inputs | Forbidden inputs | Tests |
|---|---|---|---|---|
| memory model | none beyond current public state | public board, legal action tree, current view | private logs or dev-only state | not applicable / no-leak |
| belief model | none | not applicable | hidden state, future random outcomes | not applicable |
| redaction model | explanations use public priority fragments only | chosen action, visible threat/win/block fact | rankings, raw scores, internals | explanation no-leak |

## Explanation contract

Every Level 2 decision should produce a viewer-safe explanation with:

| Field | Required? | Notes |
|---|---:|---|
| policy name/version | yes | `column_four_tactical_v1` |
| chosen priority reason | yes | Maps to priority vector slot. |
| relevant visible fact | yes | Example: "column 4 completes a vertical line." |
| tie-break note | if applicable | Center or deterministic tie-break only. |
| hidden-info disclaimer | no | Perfect-information game, but no internals should leak. |
| fallback/search note | no | No search is used. |
| known weakness if surfaced | optional | Keep concise. |

## Public explanation examples

| Situation | Chosen action | Public explanation | Hidden-info safe? | Rule IDs |
|---|---|---|---:|---|
| Own immediate win in `c4` | `drop/c4` | "Chose Column 4 because it wins now." | yes | `CF-END-001` through `CF-END-004` |
| Opponent immediate win in `c2` | `drop/c2` | "Chose Column 2 to block a visible immediate threat." | yes | `CF-END-001` through `CF-END-004` |
| Empty board | `drop/c4` | "No immediate win or block exists, so I chose the center column." | yes | `CF-ACTION-001` |

Bad explanation examples:

| Situation | Forbidden explanation | Why forbidden |
|---|---|---|
| Any decision | "scores=[8,4,4,2]" | Raw score array. |
| Any decision | "candidate ranking: c4,c3,c5" | Public ranking dump. |
| Any decision | "search depth found a win" | Search is excluded for this Level 2 policy. |

## Dev-mode ranking examples

Dev-mode candidate rankings are not part of the public v1/v2 bot output for
this gate. If added later, they must be viewer-safe and hidden behind a dev
toggle.

## Decision examples and expected choices

| Example ID | Situation | Candidate choices | Expected choice | Priority vector reason | Rule IDs | Test name |
|---|---|---|---|---|---|---|
| `CF-BOT-EX-001` | Bot can win vertically in `c4`. | all legal columns | `drop/c4` | slot 1 terminal win | `CF-END-002` | `level2_takes_immediate_win` |
| `CF-BOT-EX-002` | Opponent can win horizontally in `c2`. | all legal columns | `drop/c2` | slot 2 block immediate loss | `CF-END-001` | `level2_blocks_immediate_loss` |
| `CF-BOT-EX-003` | Empty board. | `drop/c1` through `drop/c7` | `drop/c4` | slot 6 center preference | `CF-ACTION-001` | `level2_prefers_center_on_empty_board` |

## Known weaknesses

| Weakness | Why acceptable for public Level 2 | Mitigation | Future trigger |
|---|---|---|---|
| Does not prove perfect play. | Public bot should be competent and beatable, not a solver. | Explainable tactical priorities. | Later Level 3 ADR. |
| Does not plan deep traps. | Deep search is out of scope for Gate 5. | Immediate win/block/safety checks catch obvious tactics. | Later authored-policy expansion. |
| Bounded scoring may miss complex forks. | Keeps decisions fast and explainable. | Simulation evidence in 008/011. | Repeated poor public play evidence. |

## Test plan

| Test area | Required? | Specific tests | Evidence |
|---|---:|---|---|
| legality | yes | bot chooses only legal action paths over many seeds | 008/009 |
| determinism | yes | fixed seed/view/rules/policy/limits produce fixed decision | 008 |
| no hidden-state access | yes | explanation/replay/export no-leak for public fields | 008/015 |
| candidate extraction | yes | candidate groups match legal actions and visible facts | 008 |
| priority vector | yes | decision examples hit expected slots | 008 |
| bounded scoring | yes | ranges and explanations tested | 008 |
| seeded tie-break | yes | stable tie ordering | 008 |
| explanations | yes | viewer-safe public explanations | 008 |
| simulation/fuzz | yes | many-seed games, failure reporting | 009/013 |
| replay/hash | yes | bot decision reproducible in replay | 010 |
| benchmark | yes | latency and throughput | 011 |

## Latency and benchmark budget

| Operation | Target/budget | Measurement command | Baseline | Notes |
|---|---:|---|---:|---|
| legal action generation | under 1 ms native per decision | `cargo bench -p column_four` | pending | Action tree is at most seven legal columns. |
| candidate extraction | under 1 ms native per decision | `cargo bench -p column_four` | pending | Bounded one-ply only. |
| priority ranking | under 1 ms native per decision | `cargo bench -p column_four` | pending | Small candidate list. |
| full decision latency | under 2 ms native per decision | `cargo bench -p column_four` | pending | Exact threshold may be refined by benchmark ticket. |
| playout throughput with bot | benchmarked, no hard number yet | `cargo bench -p column_four` | pending | GAT5COLFOUPUB-011. |
| explanation generation | negligible string construction | `cargo bench -p column_four` | pending | No LLM or external service. |

## Public UX note

The public UI should show one concise recent-decision explanation, such as
"Column 4 wins now" or "Column 2 blocks an immediate threat." It should not show
candidate tables, raw scores, arrays, or a debug console by default.

## Review checklist

- `COMPETENT-PLAYER.md` was consumed.
- Legal action API and validation path are exact.
- Bot input view is explicit.
- No omniscient state, hidden-state shortcuts, or future random outcomes are used.
- Candidate extraction uses legal action paths and allowed views.
- Priority vector is lexicographic.
- Bounded scores are small, named, documented, and tested.
- Tie-breaks are deterministic under seed and candidate identity.
- Style profiles do not cheat or weaken mandatory priorities.
- Public explanations do not expose candidate rankings, score arrays, replay internals, diagnostics internals, or dev-only state.
- Public v1/v2 minimax, negamax, alpha-beta, MCTS, ISMCTS, Monte Carlo, tablebase, ML/RL, and LLM move selection are absent.
- Test plan, simulation plan, replay/hash plan, and benchmark plan are complete.
- Public UX note is concise and product-facing.

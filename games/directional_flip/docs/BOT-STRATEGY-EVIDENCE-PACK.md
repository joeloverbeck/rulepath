# Directional Flip Bot Strategy Evidence Pack

Game ID: `directional_flip`

Implemented variant: `directional_flip_standard`

Rules version: `directional_flip-rules-v1`

Bot target: Level 2-lite authored policy

Policy name/version: `directional_flip_level2_lite_v1`

Prepared by: `Codex`

Date: 2026-06-07

## Purpose and gate

This is the formal design input for the Directional Flip Level 2-lite
authored-policy bot. The executable bot must not drift beyond this pack without
updating the pack and tests.

This pack consumes `COMPETENT-PLAYER.md`; it does not replace it. It also does
not replace `AI.md`, which will be filled after the bot implementation lands.

The policy must be deterministic under seed, rules version, policy version,
input view, and declared limits. It must use the legal action API and submit
the chosen action through normal validation. It must produce viewer-safe
explanations.

## Explicit public v1/v2 exclusions

The Level 2-lite public bot must not use:

- omniscient state;
- hidden-state shortcuts;
- future random outcomes;
- unbounded weight soup;
- behavior conditions hidden in static data;
- random blunder injection by default;
- minimax;
- negamax;
- alpha-beta search;
- public v1/v2 MCTS;
- public v1/v2 ISMCTS;
- public v1/v2 Monte Carlo or monte carlo playout bots;
- opening books or exact stable-disc/endgame solvers;
- public v1/v2 ML/RL;
- runtime LLM move selection.

Future minimax, negamax, alpha-beta, MCTS, ISMCTS, Monte Carlo, solver,
ML/RL, or LLM work requires an ADR under `docs/FOUNDATIONS.md` and
`docs/AI-BOTS.md`.

## Source documents consumed

| Document/source | Path/reference | Required? | Status | Notes |
|---|---|---:|---|---|
| `RULES.md` | `games/directional_flip/docs/RULES.md` | yes | read | Rule IDs define legal placements, forced pass, flips, terminal, scoring, visibility, replay, and bot boundary. |
| `COMPETENT-PLAYER.md` | `games/directional_flip/docs/COMPETENT-PLAYER.md` | yes | read | Direct strategy input for priority vector. |
| `SOURCES.md` strategy references | `games/directional_flip/docs/SOURCES.md` | yes | read | Records Othello Belgium and Nederlandse Othello Vereniging strategy references. |
| Gate 6 spec | `specs/gate-6-directional-flip.md` | yes | read | Requires Level 2-lite docs before code and outlines the allowed policy class. |
| `docs/AI-BOTS.md` | `docs/AI-BOTS.md` | yes | read | Defines Level 2 evidence packs, lexicographic priorities, explanations, and exclusions. |
| `RULE-COVERAGE.md` | `games/directional_flip/docs/RULE-COVERAGE.md` | yes | incomplete | Lands in a later Gate 6 ticket. |
| `MECHANICS.md` | `games/directional_flip/docs/MECHANICS.md` | yes | incomplete | Lands in a later Gate 6 ticket. |
| `AI.md` | `games/directional_flip/docs/AI.md` | yes | incomplete | Lands after bot implementation. |

## Exact bot input view

| Input item | Included? | Source | Visible to acting seat? | Notes/no-leak test |
|---|---:|---|---:|---|
| legal action tree/action paths | yes | Rust legal action API | yes | Candidate set must equal legal actions. |
| public view | yes | Rust `PublicView` projection | yes | Includes board, active seat, counts, legal targets, previews, and terminal state. |
| acting seat private view | no | not applicable | not applicable | Perfect-information game. |
| command/effect history visible to seat | optional | public effect log | yes | May be used for concise explanation context only. |
| policy seed/tie-break state | yes | bot framework | not game information | Used only after higher priorities tie. |
| hidden opponent/private state | no | forbidden | no | No such game state exists. |
| future random outcomes | no | forbidden | no | Game rules have no randomness. |
| dev/test full state shortcut | no | forbidden for public bot | no | Tests may inspect state; public bot may not depend on dev-only shortcuts. |
| replay hashes/freshness internals | no as strategy input | replay/checkpoint contract | no | Not a candidate feature or explanation fact. |

## Legal action API used

| API/contract | Purpose | Determinism requirement | Tests |
|---|---|---|---|
| `legal_action_tree(state, actor)` | Enumerate legal placement or forced-pass action paths. | Same state and actor produce the same ordered candidate list. | `level2_candidates_match_legal_action_tree` |
| `validate_command(state, command)` | Submit chosen action path through normal validation. | Bot decision must validate before application. | `level2_chosen_action_validates` |
| `project_view(state, viewer)` | Consume viewer-safe board, legal target, counts, terminal, and preview data. | Same state produces same stable public view. | `level2_uses_viewer_safe_projection` |
| `apply_action(state, action)` in candidate evaluator | Build bounded one-ply public successors for feature extraction. | Successor evaluation must preserve canonical rules and deterministic ordering. | `level2_successor_features_are_deterministic` |

## Candidate extraction plan

| Candidate group | Extraction rule | Rule IDs | Visible facts used | Hidden-info risk | Tests |
|---|---|---|---|---|---|
| Forced pass | If the action tree contains only `pass/forced`, the candidate is mandatory. | `DF-ACTION-002`, `DF-PASS-001` | Action tree. | none | forced-pass fixture |
| Legal placements | One candidate per Rust legal placement segment. | `DF-ACTION-001`, `DF-LEGAL-001` | Action tree and public preview metadata. | none | candidate count equals legal placement count |
| Corner targets | Mark candidates whose target is a board corner. | `DF-LEGAL-001`, `DF-FLIP-001` | Target cell id. | none | legal-corner fixture |
| Open-corner danger | Penalize X/C-square targets adjacent to an empty corner. | `DF-LEGAL-001`, `DF-FLIP-001` | Target cell id and public corner occupancy. | none | open-corner danger fixture |
| Mobility delta | Compare bot next-turn legal count and opponent next-turn legal count after the candidate. | `DF-ACTION-001`, `DF-ACTION-002` | One-ply public successor and legal action API. | none | mobility-delta fixture |
| Frontier exposure | Count own discs adjacent to empty cells after the candidate. | `DF-FLIP-001` | One-ply public successor board. | none | frontier bounded-score fixture |
| Phase-aware count | Use disc-count delta only as a late tie-break, stronger as empty cells shrink. | `DF-SCORE-001`, `DF-SCORE-002` | Public counts and empty-cell count. | none | late-count fixture |

Candidates are legal action paths plus policy annotations. They must not include
hidden information, raw implementation arrays, replay hashes, or dev-only state.

## Phase model

| Phase/situation | Detection from allowed input | Policy node(s) active | Rule IDs | Notes |
|---|---|---|---|---|
| Terminal | Public terminal view is win or draw, or action tree is empty. | no-action | `DF-TERM-001`, `DF-SCORE-001`, `DF-SCORE-002` | Return no decision. |
| Forced pass | Action tree exposes only `pass/forced`. | mandatory-pass | `DF-ACTION-002`, `DF-PASS-001` | Highest priority compliance. |
| Corner tactic | At least one legal target is a corner. | take-corner | `DF-LEGAL-001`, `DF-FLIP-001` | Strong positional priority. |
| Open-corner risk | Legal targets include X/C squares next to empty corners. | avoid-danger | `DF-LEGAL-001` | Penalty unless all candidates share the danger or higher slots dominate. |
| Mobility fight | No forced pass/corner priority dominates. | mobility-delta | `DF-ACTION-001`, `DF-ACTION-002` | One-ply public successor only. |
| Late count pressure | Empty cells are low and higher slots tie. | count-tie-break | `DF-SCORE-001`, `DF-SCORE-002` | Disc count rises late, not early. |

## Lexicographic priority vector

Earlier slots dominate later slots.

| Slot | Priority | Higher/better value | Source evidence | Rule IDs | Tests | Explanation fragment |
|---:|---|---|---|---|---|---|
| 1 | mandatory forced pass | Candidate is the only legal `pass/forced` action. | `COMPETENT-PLAYER.md#immediate-tactics` | `DF-ACTION-002`, `DF-PASS-001` | `level2_takes_forced_pass` | "No placement is legal, so I must pass." |
| 2 | legal corner | Candidate target is a corner. | `COMPETENT-PLAYER.md#positional-resource-card-and-tempo-principles` | `DF-LEGAL-001`, `DF-FLIP-001` | `level2_prefers_legal_corner` | "This takes a corner anchor." |
| 3 | avoid open-corner danger | Candidate is not an X/C square next to an empty corner. | `COMPETENT-PLAYER.md#common-beginner-mistakes` | `DF-LEGAL-001`, `DF-FLIP-001` | `level2_avoids_open_corner_adjacent_square` | "This avoids giving up corner access." |
| 4 | reduce opponent mobility | Candidate leaves fewer opponent legal placements. | `COMPETENT-PLAYER.md#threats-to-block` | `DF-ACTION-001`, `DF-ACTION-002` | `level2_prefers_better_mobility_delta` | "This reduces the opponent's visible choices." |
| 5 | preserve own mobility | Candidate leaves more bot legal placements on the next bot turn estimate. | `COMPETENT-PLAYER.md#positional-resource-card-and-tempo-principles` | `DF-ACTION-001` | `level2_preserves_own_mobility` | "This keeps more future options open." |
| 6 | lower frontier exposure | Candidate creates fewer own frontier discs. | `COMPETENT-PLAYER.md#positional-resource-card-and-tempo-principles` | `DF-FLIP-001` | `level2_frontier_score_is_bounded` | "This exposes fewer discs next to empty cells." |
| 7 | phase-aware count tie-break | Later board positions prefer better visible disc-count delta. | `COMPETENT-PLAYER.md#phases-and-situations` | `DF-SCORE-001`, `DF-SCORE-002` | `level2_late_count_tie_break_is_phase_aware` | "Late in the game, this improves the visible count." |
| 8 | deterministic seeded tie-break | Stable order from seed and candidate identity. | `docs/AI-BOTS.md` | `DF-BOT-002` | `level2_fixed_seed_is_deterministic` | "Equivalent choices were resolved deterministically." |

## Bounded scoring tie-breakers

Small scoring is allowed only after higher lexicographic categories.

| Score term | Range | Meaning | Used after slots | Visible inputs | Tests | Explanation text |
|---|---:|---|---|---|---|---|
| `opponent_mobility_reduction` | `-32..32` | Fewer opponent legal moves is better. | 1-3 | Public one-ply successor legal counts. | mobility range test | "reduces the opponent's choices" |
| `own_mobility_preservation` | `-32..32` | More own future legal moves is better. | 1-4 | Public one-ply successor legal counts. | mobility range test | "keeps options open" |
| `frontier_exposure` | `-64..0` | Fewer own discs adjacent to empty cells is better. | 1-5 | Public one-ply successor board. | frontier range test | "exposes fewer discs" |
| `late_count_delta` | `-64..64` | Better disc count matters more when empty cells are low. | 1-6 | Public counts and empty-cell count. | late count test | "improves the late visible count" |

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
| tie-break input identity | policy id, policy version, rules version, active seat, ply, freshness token, action segment. |
| stable ordering rule | Sort by lexicographic priority vector, bounded score tuple, row-major action segment order, then deterministic seeded key. |
| reproducibility tests | `level2_fixed_seed_is_deterministic`, replay/hash bot test in GAT6DIRFLI-011/013/016. |
| replay/hash interaction | Chosen action path and viewer-safe rationale are replayed as public effects; replay hashes do not drive strategy. |

Tie-break randomness must be deterministic. Random blunder injection is forbidden
by default.

## Style profile hooks

One strong default bot comes first.

| Profile | Variation | Must not affect | Hidden-info safe? | Tests |
|---|---|---|---:|---|
| default | Balanced corner/mobility/frontier policy. | legality, hidden-info boundary, determinism, mandatory pass, corner priority. | yes | Level 2-lite bot tests. |
| future cautious | May weight open-corner danger and own mobility before opponent denial after mandatory/corner priorities. | mandatory rules, legal action API, hidden-info boundary. | yes | future ticket only. |
| future aggressive | May favor opponent mobility reduction before own mobility after safety is satisfied. | mandatory rules, legal action API, hidden-info boundary. | yes | future ticket only. |

## Forbidden hidden information

| Information | Why forbidden | Potential leak surface | Required no-leak test |
|---|---|---|---|
| raw candidate ranking array | Not needed in public explanation. | bot explanation, replay export, DOM | explanation no-leak |
| raw score vector | Implementation detail, not public reasoning. | bot explanation, dev inspector | explanation no-leak |
| replay hashes/freshness internals as strategy facts | Replay contract data, not public strategy. | candidate features, explanation | bot input audit |
| dev/test full state shortcut | Public bot must consume allowed view/legal API. | bot input, candidate features | bot input audit |
| future random or sampled outcome | Not part of game rules or legal view. | tie-break, explanation | determinism test |
| source-branded terms/prose | IP/trade-dress risk. | public explanation, docs | docs/public copy review |

## Memory and belief model

| Item | Decision | Allowed inputs | Forbidden inputs | Tests |
|---|---|---|---|---|
| memory model | none beyond current public state | public board, legal action tree, current public view | private logs or dev-only state | not applicable / no-leak |
| belief model | none | not applicable | hidden state, future random outcomes | not applicable |
| redaction model | explanations use public priority fragments only | chosen action, visible corner/mobility/frontier/count fact | rankings, raw scores, hashes, internals | explanation no-leak |

Directional Flip is perfect-information, so no hidden-state belief model is
needed. Perfect information still does not permit public explanations to dump
internal score arrays or replay/checkpoint internals.

## Explanation contract

Every Level 2-lite decision should produce a viewer-safe explanation with:

| Field | Required? | Notes |
|---|---:|---|
| policy name/version | yes | `directional_flip_level2_lite_v1` |
| chosen priority reason | yes | Maps to priority vector slot. |
| relevant visible fact | yes | Example: "r1c1 is a legal corner." |
| tie-break note | if applicable | Bounded visible tie-break or deterministic seed only. |
| hidden-info disclaimer | no | Perfect-information game, but no internals should leak. |
| fallback/search note | no | No search is used. |
| known weakness if surfaced | optional | Keep concise. |

## Public explanation examples

| Situation | Chosen action | Public explanation | Hidden-info safe? | Rule IDs |
|---|---|---|---:|---|
| Forced pass only | `pass/forced` | "No placement is legal, so I must pass." | yes | `DF-ACTION-002`, `DF-PASS-001` |
| Legal corner at `r1c1` | `place/r1c1` | "Chose r1c1 because it takes a corner anchor." | yes | `DF-LEGAL-001`, `DF-FLIP-001` |
| Safe alternative to `r2c2` while `r1c1` is empty | safe placement | "Chose a safer move to avoid giving up corner access." | yes | `DF-LEGAL-001`, `DF-FLIP-001` |
| Mobility tie | mobility-favored placement | "Chose this placement because it reduces the opponent's visible choices." | yes | `DF-ACTION-001` |
| Late count tie | count-favored placement | "Late in the game, this improves the visible count." | yes | `DF-SCORE-001`, `DF-SCORE-002` |

Bad explanation examples:

| Situation | Forbidden explanation | Why forbidden |
|---|---|---|
| Any decision | "scores=[12,4,-6]" | Raw score array. |
| Any decision | "candidate ranking: r1c1,r2c2,r3c4" | Public ranking dump. |
| Any decision | "search depth 3 found this." | Search is excluded for Level 2-lite. |
| Any decision | "hash changed to ..." | Replay/hash internals are not strategic explanation facts. |

## Dev-mode ranking examples

Dev-mode candidate rankings are not part of the public v1/v2 bot output for
this gate. If added later, they must be viewer-safe, hidden behind a dev toggle,
and must not include raw internals that would become public explanation text.

| Situation | Candidate ranking excerpt | Redactions needed? | Hidden-info safe? | Notes |
|---|---|---:|---:|---|
| Corner available | `place/r1c1` ranked above non-corner placements by slot 2. | no | yes | Slot label is viewer-safe. |
| Equivalent mobility candidates | Candidates shown only with visible priority labels, not raw seeded keys. | yes | yes | Seeded key stays internal. |

## Decision examples and expected choices

| Example ID | Situation | Candidate choices | Expected choice | Priority vector reason | Rule IDs | Test name |
|---|---|---|---|---|---|---|
| `DF-BOT-EX-001` | No placement is legal and `pass/forced` is exposed. | `pass/forced` | `pass/forced` | slot 1 mandatory forced pass | `DF-ACTION-002`, `DF-PASS-001` | `level2_takes_forced_pass` |
| `DF-BOT-EX-002` | A corner target is legal. | all legal placements | corner placement | slot 2 legal corner | `DF-LEGAL-001`, `DF-FLIP-001` | `level2_prefers_legal_corner` |
| `DF-BOT-EX-003` | `r2c2` is legal while `r1c1` is empty and another safe move exists. | `place/r2c2`, safe alternative | safe alternative | slot 3 avoid open-corner danger | `DF-LEGAL-001` | `level2_avoids_open_corner_adjacent_square` |
| `DF-BOT-EX-004` | No corner/danger priority dominates; one candidate leaves fewer opponent legal moves. | all legal placements | lower-opponent-mobility placement | slot 4 reduce opponent mobility | `DF-ACTION-001` | `level2_prefers_better_mobility_delta` |
| `DF-BOT-EX-005` | Late board and higher slots tie. | two legal placements | better count-delta placement | slot 7 phase-aware count tie-break | `DF-SCORE-001`, `DF-SCORE-002` | `level2_late_count_tie_break_is_phase_aware` |

## Known weaknesses

| Weakness | Why acceptable for public Level 2-lite | Mitigation | Future trigger |
|---|---|---|---|
| Does not prove perfect play. | Public bot should be competent and beatable, not a solver. | Explainable corner/mobility/frontier priorities. | Later Level 3 ADR. |
| Approximate stability only. | Exact stable-disc solving can become search-like. | Corners, edge extension, and frontier features catch common positional signals. | Repeated poor public play evidence. |
| No deep sacrifice planning. | Deep trap analysis requires search or playouts. | Bounded one-ply mobility and corner danger catch obvious mistakes. | Accepted ADR for search. |
| Late count tie-break can still miss exact endgames. | Perfect endgame is outside public v1/v2 Level 2-lite. | Keep the limitation documented and test deterministic behavior. | Later research gate. |

## Test plan

| Test area | Required? | Specific tests | Evidence |
|---|---:|---|---|
| legality | yes | bot chooses only legal action paths over many states/seeds | GAT6DIRFLI-011 |
| determinism | yes | fixed seed/view/rules/policy/limits produce fixed decision | GAT6DIRFLI-011 |
| no hidden-state access | yes | explanation/replay/export no-leak for public fields | GAT6DIRFLI-011/015 |
| candidate extraction | yes | candidate groups match legal actions and visible facts | GAT6DIRFLI-011 |
| priority vector | yes | decision examples hit expected slots | GAT6DIRFLI-011 |
| bounded scoring | yes | ranges and explanations tested | GAT6DIRFLI-011 |
| seeded tie-break | yes | stable tie ordering | GAT6DIRFLI-011 |
| explanations | yes | viewer-safe public explanations | GAT6DIRFLI-011 |
| simulation/fuzz | yes | many-seed games, failure reporting | GAT6DIRFLI-014 |
| replay/hash | yes | bot decision reproducible in replay/golden traces | GAT6DIRFLI-013/016 |
| benchmark | yes | latency and throughput | GAT6DIRFLI-018 |

## Latency and benchmark budget

| Operation | Target/budget | Measurement command | Baseline | Notes |
|---|---:|---|---:|---|
| legal action generation | under 2 ms native per decision | `cargo bench -p directional_flip` | pending | At most 64 board cells, typically far fewer legal targets. |
| candidate extraction | under 2 ms native per decision | `cargo bench -p directional_flip` | pending | Bounded one-ply visible features only. |
| priority ranking | under 1 ms native per decision | `cargo bench -p directional_flip` | pending | Small candidate list. |
| full decision latency | under 5 ms native per decision | `cargo bench -p directional_flip` | pending | Threshold may be refined by benchmark ticket. |
| playout throughput with bot | benchmarked, no hard number yet | `cargo bench -p directional_flip` | pending | GAT6DIRFLI-018. |
| explanation generation | negligible string construction | `cargo bench -p directional_flip` | pending | No LLM or external service. |

## Public UX note

The public UI should show one concise recent-decision explanation, such as
"Took a corner anchor" or "Reduced the opponent's visible choices." It should
not show candidate tables, raw scores, seeded keys, replay hashes, arrays, or a
debug console by default.

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
- Public v1/v2 minimax, negamax, alpha-beta, MCTS, ISMCTS, Monte Carlo, solver, ML/RL, and LLM move selection are absent.
- Test plan, simulation plan, replay/hash plan, and benchmark plan are complete.
- Public UX note is concise and product-facing.

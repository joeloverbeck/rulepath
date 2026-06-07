# Draughts Lite Bot Strategy Evidence Pack

Game ID: `draughts_lite`

Implemented variant: `draughts_lite_standard`

Rules version: `draughts_lite-rules-v1`

Bot target: Level 1 modest authored policy

Policy name/version: `draughts_lite_level1_v1`

Prepared by: `Codex`

Date: 2026-06-07

## Purpose and gate

This is the formal design input for the Draughts Lite Level 1 authored-policy
bot. The executable bot must not drift beyond this pack without updating the
pack and tests.

This pack consumes `COMPETENT-PLAYER.md`; it does not replace it. It also does
not replace [AI.md](AI.md), which records the shipped bot registry.

The policy must be deterministic under seed, rules version, policy version,
input view, and declared limits. It must use the legal action API, submit the
chosen path through normal validation, and produce viewer-safe explanations.

## Explicit public v1/v2 exclusions

The Level 1 public bot must not use:

- omniscient state outside the public rules/view/action surfaces;
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
- transposition tables;
- opening books;
- endgame databases or solved-game tables;
- public v1/v2 ML/RL;
- runtime LLM move selection.

Future minimax, alpha-beta, MCTS, ISMCTS, Monte Carlo, solver, opening-book,
endgame-database, ML/RL, or runtime LLM work requires an ADR under
`docs/FOUNDATIONS.md` and `docs/AI-BOTS.md`.

This bot makes no competitive checkers/draughts strength claim. It is a modest,
fair, beatable product opponent.

## Source documents consumed

| Document/source | Path/reference | Required? | Status | Notes |
|---|---|---:|---|---|
| `RULES.md` | `games/draughts_lite/docs/RULES.md` | yes | read | Rule IDs define legal moves, forced capture/continuation, promotion, terminal, replay, visibility, and bot boundary. |
| `COMPETENT-PLAYER.md` | `games/draughts_lite/docs/COMPETENT-PLAYER.md` | yes | read | Direct strategy input for the priority vector. |
| `SOURCES.md` | `games/draughts_lite/docs/SOURCES.md` | yes | read | Records adopted/omitted rules and solved-game/strong-engine exclusion context. |
| Gate 7 spec | `archive/specs/gate-7-draughts-lite-compound-action-tree.md` | yes | read | §R17 defines acceptable Level 1 heuristics and exclusions. |
| `docs/AI-BOTS.md` | `docs/AI-BOTS.md` | yes | read | Defines deterministic, explainable, non-search public bot policy. |
| `RULE-COVERAGE.md` | `games/draughts_lite/docs/RULE-COVERAGE.md` | yes | read | Rule-to-evidence matrix includes bot, visibility, replay, UI, and benchmark rows. |
| `MECHANICS.md` | `games/draughts_lite/docs/MECHANICS.md` | yes | read | Mechanic inventory records compound path and bot policy shapes. |
| `AI.md` | `games/draughts_lite/docs/AI.md` | yes | read | Shipped bot registry for Level 0 and Level 1. |

## Exact bot input view

| Input item | Included? | Source | Visible to acting seat? | Notes/no-leak test |
|---|---:|---|---:|---|
| legal action tree/action paths | yes | Rust legal action API | yes | Candidate set must equal complete legal paths. |
| public view | yes | Rust `PublicView` projection | yes | Includes all cells, piece owner/kind, active seat, and terminal. |
| acting seat private view | no | not applicable | not applicable | Perfect-information game. |
| command/effect history visible to seat | optional | public effect/replay surfaces | yes | May be used for concise explanation context only. |
| policy seed/tie-break state | yes | bot framework | not game information | Used only after higher priorities tie. |
| hidden opponent/private state | no | forbidden | no | No such game state exists. |
| future random outcomes | no | forbidden | no | Game rules have no randomness. |
| dev/test full-state shortcut | no | forbidden for public bot | no | Tests may inspect state; public bot must use normal legal/view APIs. |
| replay hashes/freshness internals | no as strategy input | replay/checkpoint contract | no | Not candidate features or explanation facts. |

## Legal action API used

| API/contract | Purpose | Determinism requirement | Tests |
|---|---|---|---|
| `legal_action_tree(state, actor)` | Enumerate complete quiet or capture action paths, including forced continuation. | Same state and actor produce the same ordered candidate tree. | `level1_candidates_match_legal_action_tree` |
| `validate_command(state, command)` | Submit chosen path through normal validation. | Bot decision must validate before application. | `level1_chosen_action_validates` |
| `project_view(state, viewer)` | Consume viewer-safe board, piece, active-seat, and terminal facts. | Same state produces same stable public view. | `level1_uses_viewer_safe_projection` |
| `apply_action(state, action)` in candidate evaluator | Build bounded one-ply public successors for terminal and local safety features. | Successor evaluation must preserve canonical rules and deterministic ordering. | `level1_successor_features_are_deterministic` |

## Candidate extraction plan

| Candidate group | Extraction rule | Rule IDs | Visible facts used | Hidden-info risk | Tests |
|---|---|---|---|---|---|
| Complete legal paths | Flatten every complete leaf path from the Rust action tree. | `DL-ACTION-001`, `DL-ACTION-002`, `DL-RESTRICT-002`, `DL-SCOPE-002` | Action tree segments and metadata. | none | candidates equal legal paths |
| Terminal wins | Mark candidates whose one-ply successor wins by no opponent pieces or no opponent legal move. | `DL-END-001`, `DL-END-002` | One-ply public successor. | none | terminal-win fixture |
| Captures | Mark paths with one or more jump segments. | `DL-ACTION-002`, `DL-MOVE-002` | Action path and legal metadata. | none | capture preference fixture |
| Longer captures | Count jump segments and captured pieces in complete legal paths. | `DL-RESTRICT-004`, `DL-MOVE-002` | Legal path/effect metadata. | none | longer capture heuristic fixture |
| Promotion | Mark candidates that crown a man. | `DL-MOVE-003`, `DL-ACTION-003` | Legal move metadata or one-ply successor. | none | promotion fixture |
| Capture-to-promotion | Mark capture paths that also promote. | `DL-MOVE-002`, `DL-MOVE-003` | Legal move metadata/effects. | none | capture-promotion fixture |
| King preservation | Penalize otherwise comparable candidates where a king can be immediately captured by the opponent in one ply. | `DL-ACTION-004`, `DL-MOVE-002` | One-ply public successor and opponent legal action tree. | none | king safety fixture |
| Material balance | Count public pieces after the candidate. | `DL-MOVE-002`, `DL-END-001` | Public successor board. | none | material tie-break fixture |

Candidates are complete legal action paths plus policy annotations. They must
not include partial origin-selection UI state, hidden information, raw score
arrays, replay hashes, or dev-only state.

## Phase model

| Phase/situation | Detection from allowed input | Policy node(s) active | Rule IDs | Notes |
|---|---|---|---|---|
| Terminal | Public terminal view is win, or action tree is empty. | no-action | `DL-ACTION-005`, `DL-END-001`, `DL-END-002` | Return no decision. |
| Mandatory capture | Legal paths contain jump segments; quiet paths are absent by rule. | capture policy | `DL-RESTRICT-001`, `DL-ACTION-002` | Compliance is already in candidate extraction. |
| Forced continuation | Complete leaf path has multiple jump segments. | complete-path policy | `DL-RESTRICT-002`, `DL-REPLAY-002` | Bot must never emit a partial path. |
| Immediate terminal | Candidate wins immediately. | terminal-result | `DL-END-001`, `DL-END-002` | Highest strategic priority after legal compliance. |
| Promotion race | Candidate promotes a man. | promotion priority | `DL-MOVE-003` | Promotion during capture still ends that move. |
| King safety | Candidate exposes a king to immediate legal capture. | local-safety tie-break | `DL-ACTION-004`, `DL-MOVE-002` | One-ply only; no deeper search. |
| Otherwise tactical | No terminal/promotion/safety slot dominates. | material and seeded tie-break | `DL-MOVE-002`, `DL-RESTRICT-004` | Keep modest and explainable. |

## Lexicographic priority vector

Earlier slots dominate later slots.

| Slot | Priority | Higher/better value | Source evidence | Rule IDs | Tests | Explanation fragment |
|---:|---|---|---|---|---|---|
| 1 | complete legal path / mandatory compliance | Candidate is a complete Rust legal leaf path. | `COMPETENT-PLAYER.md#immediate-tactics` | `DL-ACTION-001`, `DL-ACTION-002`, `DL-RESTRICT-002` | `level1_never_emits_partial_continuation` | "I chose a complete legal path." |
| 2 | immediate terminal win | Candidate immediately wins. | `COMPETENT-PLAYER.md#immediate-tactics` | `DL-END-001`, `DL-END-002` | `level1_takes_terminal_win` | "This move wins immediately." |
| 3 | capture-to-promotion | Candidate captures and crowns the moving man. | `COMPETENT-PLAYER.md#immediate-tactics` | `DL-MOVE-002`, `DL-MOVE-003` | `level1_prefers_capture_to_promotion` | "This captures and promotes the piece." |
| 4 | promotion | Candidate promotes a man. | `COMPETENT-PLAYER.md#phases-and-situations` | `DL-MOVE-003`, `DL-ACTION-004` | `level1_prefers_promotion` | "This creates a king." |
| 5 | capture path | Candidate captures at least one piece. | `COMPETENT-PLAYER.md#immediate-tactics` | `DL-ACTION-002`, `DL-MOVE-002` | `level1_prefers_capture_path_when_applicable` | "This wins material with a legal capture." |
| 6 | longer capture heuristic | More jump segments/captured pieces is better. | `COMPETENT-PLAYER.md#common-beginner-mistakes` | `DL-RESTRICT-004`, `DL-MOVE-002` | `level1_prefers_longer_capture_without_claiming_required` | "This captures more pieces." |
| 7 | preserve kings from obvious reply capture | Candidate avoids immediate one-ply opponent capture of a king. | `COMPETENT-PLAYER.md#threats-to-block` | `DL-ACTION-004`, `DL-MOVE-002` | `level1_avoids_obvious_king_hang` | "This keeps the king out of an immediate capture." |
| 8 | material balance | More own pieces minus opponent pieces after the candidate is better. | `COMPETENT-PLAYER.md#positional-resource-card-and-tempo-principles` | `DL-MOVE-002`, `DL-END-001` | `level1_material_tie_break_is_bounded` | "This leaves a better material balance." |
| 9 | deterministic seeded tie-break | Stable order from seed and complete action path identity. | `docs/AI-BOTS.md` | `DL-RNG-001`, `DL-BOT-002` | `level1_fixed_seed_is_deterministic` | "Equivalent choices were resolved deterministically." |

## Bounded scoring tie-breakers

Small scoring is allowed only after higher lexicographic categories.

| Score term | Range | Meaning | Used after slots | Visible inputs | Tests | Explanation text |
|---|---:|---|---|---|---|---|
| `terminal_win` | `0..1` | Candidate wins immediately. | candidate extraction | Public one-ply successor terminal. | terminal fixture | "wins immediately" |
| `capture_count` | `0..12` | Number of pieces captured by the complete path. | slots 1-5 | Legal path/effects. | capture range test | "captures more pieces" |
| `promotes` | `0..1` | Candidate crowns a man. | slots 1-3 | Legal path/effects or public successor. | promotion fixture | "creates a king" |
| `king_immediate_safety` | `-1..0` | Penalize immediate opponent capture of a king after candidate. | slots 1-6 | One-ply public successor and legal action tree. | king-safety fixture | "keeps a king safe" |
| `material_delta` | `-12..12` | Own public piece count minus opponent public piece count after candidate. | slots 1-7 | Public successor board. | material range test | "leaves better material" |

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
| tie-break input identity | policy id, policy version, rules version, active seat, ply, freshness token, complete action path segments joined in order. |
| stable ordering rule | Sort by lexicographic priority vector, bounded score tuple, complete path segment order, then deterministic seeded key. |
| reproducibility tests | `level1_fixed_seed_is_deterministic`, replay/hash bot tests in GAT7DRALITCOM-012/014/017. |
| replay/hash interaction | The chosen complete action path and viewer-safe rationale are replayed; replay hashes do not drive strategy. |

Tie-break randomness must be deterministic. Random blunder injection is forbidden
by default.

## Style profile hooks

One modest default bot comes first.

| Profile | Variation | Must not affect | Hidden-info safe? | Tests |
|---|---|---|---:|---|
| default | Terminal, promotion, capture, king-safety, material policy. | legality, hidden-info boundary, determinism, complete paths. | yes | Level 1 bot tests. |
| future cautious | May prefer king safety before longer capture after terminal/promotion priorities. | mandatory rules, legal action API, hidden-info boundary. | yes | future ticket only. |
| future aggressive | May favor longer captures before king safety after terminal/promotion priorities. | mandatory rules, legal action API, hidden-info boundary. | yes | future ticket only. |

## Forbidden hidden information

| Information | Why forbidden | Potential leak surface | Required no-leak test |
|---|---|---|---|
| raw candidate ranking array | Not needed in public explanation. | bot explanation, replay export, DOM | explanation no-leak |
| raw score tuple | Implementation detail, not public reasoning. | bot explanation, dev inspector | explanation no-leak |
| replay hashes/freshness internals as strategy facts | Replay contract data, not public strategy. | candidate features, explanation | bot input audit |
| dev/test full-state shortcut | Public bot must consume allowed view/legal API. | bot input, candidate features | bot input audit |
| future random or sampled outcome | Not part of game rules or legal view. | tie-break, explanation | determinism test |
| source-branded terms/prose | IP/trade-dress risk. | public explanation, docs | docs/public copy review |

## Memory and belief model

| Item | Decision | Allowed inputs | Forbidden inputs | Tests |
|---|---|---|---|---|
| memory model | none beyond current public state | public board, legal action tree, current public view | private logs or dev-only state | not applicable / no-leak |
| belief model | none | not applicable | hidden state, future random outcomes | not applicable |
| redaction model | explanations use public priority fragments only | chosen path and visible terminal/capture/promotion/king-safety/material fact | rankings, raw scores, hashes, internals | explanation no-leak |

Draughts Lite is perfect-information, so no hidden-state belief model is needed.
Perfect information still does not permit public explanations to dump internal
score arrays, replay hashes, seeded keys, or dev-only details.

## Explanation contract

Every Level 1 decision should produce a viewer-safe explanation with:

| Field | Required? | Notes |
|---|---:|---|
| policy name/version | yes | `draughts_lite_level1_v1` |
| chosen priority reason | yes | Maps to priority vector slot. |
| relevant visible fact | yes | Example: "the path captures two pieces" or "this move creates a king." |
| tie-break note | if applicable | Bounded visible tie-break or deterministic seed only. |
| hidden-info disclaimer | no | Perfect-information game, but no internals should leak. |
| fallback/search note | yes when useful | "No search or playouts are used." |
| known weakness if surfaced | optional | Keep concise. |

## Public explanation examples

| Situation | Chosen action | Public explanation | Hidden-info safe? | Rule IDs |
|---|---|---|---:|---|
| Multi-jump is legal | `from/r3c2`, `jump/r5c4`, `jump/r7c6` | "I chose the complete capture path." | yes | `DL-RESTRICT-002` |
| Capture removes last opposing piece | terminal capture path | "This capture wins immediately." | yes | `DL-END-001` |
| Capture-to-promotion | capture path ending on king row | "This captures and promotes the piece." | yes | `DL-MOVE-002`, `DL-MOVE-003` |
| Quiet promotion | quiet move to king row | "This creates a king." | yes | `DL-MOVE-003` |
| Longer capture tie | two legal complete captures | "This captures more pieces; longest capture is a preference here, not a rule." | yes | `DL-RESTRICT-004` |
| King safety tie | comparable moves, one exposes a king | "This keeps the king out of an immediate capture." | yes | `DL-ACTION-004`, `DL-MOVE-002` |

Bad explanation examples:

| Situation | Forbidden explanation | Why forbidden |
|---|---|---|
| Any decision | "scores=[7,2,-1]" | Raw score array. |
| Any decision | "candidate ranking: path A, path B, path C" | Public ranking dump. |
| Any decision | "depth 3 found this." | Search is excluded for Level 1. |
| Any decision | "hash changed to ..." | Replay/hash internals are not strategy facts. |
| Longer capture | "Rules require the longest capture." | False for Draughts Lite. |

## Dev-mode ranking examples

Dev-mode candidate rankings are not part of the public v1/v2 bot output for
this gate. If added later, they must be viewer-safe, hidden behind a dev toggle,
and must not include raw internals that would become public explanation text.

| Situation | Candidate ranking excerpt | Redactions needed? | Hidden-info safe? | Notes |
|---|---|---:|---:|---|
| Capture-to-promotion available | Complete path ranked by slot 3. | no | yes | Slot label is viewer-safe. |
| Equivalent paths | Candidates shown only with visible priority labels, not seeded keys. | yes | yes | Seeded key stays internal. |

## Decision examples and expected choices

| Example ID | Situation | Candidate choices | Expected choice | Priority vector reason | Rule IDs | Test name |
|---|---|---|---|---|---|---|
| `DL-BOT-EX-001` | Terminal capture is legal. | all legal paths | terminal capture | slot 2 terminal win | `DL-END-001`, `DL-END-002` | `level1_takes_terminal_win` |
| `DL-BOT-EX-002` | Capture-to-promotion and ordinary capture are both legal. | two complete capture paths | capture-to-promotion | slot 3 capture-to-promotion | `DL-MOVE-002`, `DL-MOVE-003` | `level1_prefers_capture_to_promotion` |
| `DL-BOT-EX-003` | Quiet promotion and non-promotion quiet move are both legal. | two quiet paths | promotion path | slot 4 promotion | `DL-MOVE-003` | `level1_prefers_promotion` |
| `DL-BOT-EX-004` | Two legal complete capture paths differ only by captured count. | one-jump and two-jump paths | longer path | slot 6 longer capture heuristic | `DL-RESTRICT-004`, `DL-MOVE-002` | `level1_prefers_longer_capture_without_claiming_required` |
| `DL-BOT-EX-005` | A legal king move exposes immediate capture while a comparable move does not. | two legal paths | safer king path | slot 7 king safety | `DL-ACTION-004`, `DL-MOVE-002` | `level1_avoids_obvious_king_hang` |
| `DL-BOT-EX-006` | Equivalent legal paths remain after all priorities. | two legal paths | deterministic seeded choice | slot 9 seeded tie-break | `DL-RNG-001`, `DL-BOT-002` | `level1_fixed_seed_is_deterministic` |

## Known weaknesses

| Weakness | Why acceptable for public Level 1 | Mitigation | Future trigger |
|---|---|---|---|
| Does not prove perfect play. | Public bot should be competent and beatable, not a solver. | Explainable terminal/capture/promotion/king-safety priorities. | Later ADR for stronger bot class. |
| No multi-turn trap planning. | Deep trap analysis requires search. | One-ply local king-safety catches obvious blunders. | Repeated poor public play evidence and accepted ADR. |
| Longer capture can be strategically wrong. | It is only a modest heuristic, not a rule. | Higher priorities and king-safety can dominate it. | Later authored policy revision. |
| No opening book or endgame database. | Strong-engine framing is out of scope. | Keep limitations documented and tests deterministic. | Later research gate. |

## Test plan

| Test area | Required? | Specific tests | Evidence |
|---|---:|---|---|
| legality | yes | bot chooses only complete legal action paths over many states/seeds | GAT7DRALITCOM-012 |
| determinism | yes | fixed seed/view/rules/policy/limits produce fixed decision | GAT7DRALITCOM-012 |
| no hidden-state access | yes | explanation/replay/export no-leak for public fields | GAT7DRALITCOM-012/013/019 |
| candidate extraction | yes | candidate groups match legal action tree leaves | GAT7DRALITCOM-012 |
| priority vector | yes | decision examples hit expected slots | GAT7DRALITCOM-012 |
| bounded scoring | yes | ranges and explanations tested | GAT7DRALITCOM-012 |
| seeded tie-break | yes | stable tie ordering | GAT7DRALITCOM-012 |
| explanations | yes | viewer-safe public explanations | GAT7DRALITCOM-012 |
| simulation/smoke | yes | bounded games without false terminal assumptions | GAT7DRALITCOM-017 |
| replay/hash | yes | bot decision reproducible in traces | GAT7DRALITCOM-014/017 |
| benchmark | yes | Level 0 and Level 1 selection benchmarked | GAT7DRALITCOM-020 |

## Latency and benchmark budget

| Operation | Target/budget | Measurement command | Baseline | Notes |
|---|---:|---|---:|---|
| legal action generation | under 2 ms native per decision | `cargo bench -p draughts_lite` | smoke-floor benchmarked | Board is 8 by 8 with bounded adjacent movement. |
| candidate extraction | under 2 ms native per decision | `cargo bench -p draughts_lite` | smoke-floor benchmarked | Complete legal paths can include multi-jumps but remain small in typical positions. |
| priority ranking | under 1 ms native per decision | `cargo bench -p draughts_lite` | smoke-floor benchmarked | Lexicographic tuple over legal paths. |
| full Level 1 decision latency | under 5 ms native per decision | `cargo bench -p draughts_lite` | smoke-floor benchmarked | Thresholds are baseline-pending smoke floors until stable CI measurements exist. |
| playout throughput with bot | benchmarked, no hard number yet | `cargo bench -p draughts_lite` | smoke-floor benchmarked | Draughts can cycle without draw adjudication. |
| explanation generation | negligible string construction | `cargo bench -p draughts_lite` | smoke-floor benchmarked | No LLM or external service. |

## Public UX note

The public UI should show one concise recent-decision explanation, such as
"Chose the complete capture path" or "This creates a king." It should not show
candidate tables, raw scores, seeded keys, replay hashes, arrays, or a debug
console by default. When a longer capture is chosen, the public text must avoid
claiming that the longest capture is legally required.

## Review checklist

- `COMPETENT-PLAYER.md` was consumed.
- Legal action API and validation path are exact.
- Bot input view is explicit.
- No omniscient state, hidden-state shortcuts, or future random outcomes are used.
- Candidate extraction uses complete legal action paths and allowed views.
- Priority vector is lexicographic.
- Bounded scores are small, named, documented, and tested.
- Tie-breaks are deterministic under seed and complete path identity.
- Style profiles do not cheat or weaken mandatory priorities.
- Public explanations do not expose candidate rankings, score arrays, replay internals, diagnostics internals, seeded keys, or dev-only state.
- Public v1/v2 minimax, alpha-beta, MCTS, ISMCTS, Monte Carlo, transposition tables, opening books, endgame databases, solver data, ML/RL, and runtime LLM move selection are absent.
- Test plan, simulation plan, replay/hash plan, and benchmark plan are complete.
- Public UX note is concise and product-facing.

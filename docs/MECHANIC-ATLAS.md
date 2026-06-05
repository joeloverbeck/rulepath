# Rulepath Mechanic Atlas

Status: repository-level primitive pressure law.

The mechanic atlas prevents repeated local implementations from quietly diverging while also preventing speculative abstractions from contaminating `engine-core` or `game-stdlib`.

Rulepath generalizes only after implemented games prove the shape.

## 1. Purpose

The atlas records:

- which mechanic shapes official games use;
- which mechanics remain local;
- which shapes are repeated candidates;
- which extractions are required;
- which helpers are promoted to `game-stdlib`;
- which proposals are deferred or rejected;
- which decisions require ADR.

The atlas is not a wish list for a universal tabletop engine. It is an evidence ledger.

## 2. Mechanic inventory categories

Every official game `MECHANICS.md` must inventory these categories:

| Category | Questions to answer |
|---|---|
| topology/spatial model | Does the game use positions, tracks, maps, graphs, routes, regions, or no spatial model? |
| component/zone model | What game-owned components and areas exist? Are any hidden, shared, ordered, or shuffled? |
| action shape | Are choices flat, targeted, compound, multi-step, simultaneous, forced, interruptible? |
| turn/phase model | What advances time: turns, rounds, phases, tricks, reactions, cleanup, events? |
| randomness/chance | What is random, when sampled, how replayed, and what is visible? |
| visibility/hidden information | What each seat/viewer may know, when, and through which payloads. |
| resource/accounting | Counters, payments, scores, pots, budgets, conservation rules, debts. |
| movement/capture/placement | Placement, removal, movement paths, capture, conversion, promotion, forced continuation. |
| pattern/line/directional scanning | Alignment, scanning rays, neighborhood checks, pattern detection, bracketed flips. |
| commitment/reveal | Secret choices, simultaneous selection, reveal timing, redaction, waiting states. |
| reaction/window/pending response | Who may respond, priority, cancellation, replacement, forced windows, timeout-free local flow. |
| scoring/outcome | Instant win, score totals, shared outcome, asymmetric victory, tie resolution. |
| semantic effect shape | What effects must be emitted for logs, animation, replay, bots, and explanations. |
| UI interaction pattern | Direct selection, progressive construction, drag optionality, previews, confirmations, replay. |
| bot policy pattern | Random, rule-informed, authored priorities, search if allowed, hidden-info belief model. |
| benchmark/performance pressure | Hot paths, action branching, playout throughput, serialization/replay overhead. |

Use game-specific words inside game inventories. Keep shared atlas names mechanic-shaped, not product-shaped.

## 3. Primitive status labels

| Status | Meaning |
|---|---|
| `local-only` | Exists in one game or is deliberately too game-specific. |
| `repeated-shape candidate` | At least two games show a similar mechanic shape worth comparing. |
| `extraction required` | A third official game would otherwise reimplement the same shape; decision required before proceeding. |
| `promoted primitive` | A narrow typed helper exists in `game-stdlib`, with tests, docs, examples, anti-examples, and back-ports. |
| `rejected/deferred with rationale` | Reuse was considered and intentionally declined with documented reasons. |
| `ADR-required` | The proposal changes architecture, replay/hash semantics, data policy, or kernel boundaries. |

## 4. Hard gate

A third official game with the same mechanic shape may not proceed until the primitive pressure ledger says one of:

1. reuse an existing promoted primitive;
2. promote a narrow typed helper to `game-stdlib`;
3. explicitly defer/reject extraction with rationale;
4. escalate to ADR.

This hard gate applies even when copying local code feels faster. Agent “cleanup” does not bypass the ledger.

## 5. Extraction process

1. Implement the first game locally.
2. Implement the second game honestly, without premature abstraction.
3. Compare both `MECHANICS.md` inventories.
4. Identify repeated shape and real differences.
5. Record a primitive-pressure ledger entry.
6. Decide local/reuse/promote/defer/ADR.
7. If promoted, extract a narrow typed helper into `game-stdlib`.
8. Back-port affected games.
9. Preserve golden traces or intentionally update them with trace notes.
10. Add helper tests, property tests, examples, and anti-examples.
11. Benchmark before and after extraction.
12. Update atlas table, game inventories, rule coverage notes if affected, and agent instructions.

Promoted helpers must have small APIs, explicit limits, and no hidden policy. A helper that requires many game-specific escape hatches is not ready.

## 6. Extraction review questions

Before promoting a primitive, answer:

- Which games exert pressure?
- What shape is actually repeated?
- What differs across games?
- Why is local duplication now riskier than extraction?
- Why does this belong in `game-stdlib`, not `engine-core`?
- Does it introduce any game nouns into `engine-core`?
- How do traces and replay hashes change?
- What examples and anti-examples define the helper boundary?
- What benchmarks prove the helper does not hide slow generic behavior?
- What agent tasks might accidentally misuse it?

## 7. Anti-patterns

Must not:

- generalize from one game;
- promote nouns into `engine-core`;
- create a universal behavior language;
- build speculative helpers for private monster games;
- let static data define behavior through a helper;
- let agents “clean up” mechanics without a ledger entry;
- extract a huge helper with game-specific flags for every exception;
- update golden traces without explaining whether behavior changed;
- call a primitive “generic” when it only fits one game’s vocabulary;
- use a helper to smuggle TypeScript legality or bot cheating.

## 8. Sample atlas table

| Mechanic shape | Games exerting pressure | Status | Current decision | Next gate |
|---|---|---|---|---|
| fixed 2D coordinate occupancy | `three_marks`, `column_four` | repeated-shape candidate | Keep local until `directional_flip` proves directional pressure. | Stage 4 review |
| line/pattern detection | `three_marks`, `column_four` | repeated-shape candidate | Compare exact win-shape APIs; no kernel change. | Stage 4 review |
| gravity placement into column | `column_four` | local-only | Too specific for extraction. | None |
| directional scanning with grouped changes | `directional_flip` | local-only | Inventory after implementation; possible future repeated candidate. | Stage 4 exit |
| deterministic shuffle + hidden draw | `high_card_duel` | local-only | Keep game-local until second card game. | Stage 8/9 review |
| public/private view projection | all games, hidden games intensify | promoted contract in `engine-core` | Generic viewer-safe contract only; no game nouns. | Continuous |
| commitment and reveal | `secret_draft`, later bluffing games | repeated-shape candidate after second game | No early helper until waiting/reveal behavior repeats. | Stage 11 review |
| reaction window / pending response | `masked_claims`, later event games | ADR-required if generalized | Keep local unless repeated pressure proves small typed helper. | Stage 11+ |

## 9. Repo-level ledger practice

Use `templates/PRIMITIVE-PRESSURE-LEDGER.md` for each meaningful candidate. Store accepted ledgers in a repo location such as `docs/mechanics/` or append summarized rows here. The exact storage location may be decided when the first candidate appears; the process exists now.

Every game PR that adds or changes mechanics must state whether the atlas changed and why.

## 10. Acceptance checklist

Before advancing a ladder stage, verify:

- the game has a completed `MECHANICS.md`;
- repeated shapes are listed in the atlas;
- third-use hard gates are resolved;
- `game-stdlib` promotions have tests, docs, examples, anti-examples, back-ports, and benchmarks;
- `engine-core` remains noun-free;
- no helper creates a behavior language;
- traces/replay effects are preserved or intentionally migrated;
- agents were not allowed to generalize without ledger review.

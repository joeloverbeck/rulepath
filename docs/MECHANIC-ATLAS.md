# Rulepath Mechanic Atlas

Status: repository-level primitive-pressure law.

The mechanic atlas prevents repeated local implementations from quietly diverging while also preventing speculative abstractions from contaminating `engine-core` or `game-stdlib`.

Rulepath generalizes only after implemented official games prove the shape.

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

Every official game `MECHANICS.md` MUST answer these categories:

| Category | Questions |
|---|---|
| topology/spatial model | Positions, tracks, maps, graphs, routes, regions, or no spatial model? |
| component/zone model | What game-owned components and areas exist? Are any hidden, shared, ordered, shuffled, or private? |
| action shape | Flat, targeted, compound, multi-step, simultaneous, forced, interruptible, confirm-required? |
| turn/phase model | Turns, rounds, phases, tricks, reactions, cleanup, events, pending responses? |
| randomness/chance | What is random, when sampled, how replayed, and what is visible? |
| visibility/hidden information | What may each seat/viewer know, when, and through which payloads? |
| resource/accounting | Counters, payments, scores, pots, budgets, debts, conservation rules? |
| movement/capture/placement | Placement, removal, movement paths, capture, conversion, promotion, forced continuation? |
| pattern/line/directional scanning | Alignment, rays, neighborhoods, pattern detection, bracketed changes? |
| commitment/reveal | Secret choices, simultaneous selection, reveal timing, redaction, waiting states? |
| reaction/window/pending response | Who may respond, priority, cancellation, replacement, local timeout-free flow? |
| scoring/outcome | Instant win, totals, shared outcome, asymmetric victory, tie resolution? |
| semantic effect shape | What effects must exist for logs, animation, replay, bots, and explanations? |
| UI interaction pattern | Direct selection, progressive construction, drag optionality, previews, confirmations, replay? |
| bot policy pattern | Random, rule-informed, authored priorities, search if allowed, hidden-info belief model? |
| benchmark/performance pressure | Hot paths, branching factor, playout throughput, serialization/replay overhead? |

Use game-specific words in game inventories. Use mechanic-shaped language in repo-level atlas entries.

## 3. Status labels

| Status | Meaning |
|---|---|
| `local-only` | Exists in one game or is deliberately too game-specific. |
| `repeated-shape candidate` | At least two games show a similar mechanic shape worth comparing. |
| `extraction required` | A third official game would otherwise reimplement the same shape; decision required before proceeding. |
| `promoted primitive` | A narrow typed helper exists in `game-stdlib` with tests, docs, examples, anti-examples, back-ports, and benchmarks. |
| `rejected/deferred with rationale` | Reuse was considered and intentionally declined. |
| `ADR-required` | The proposal changes architecture, replay/hash semantics, data policy, visibility, or kernel boundaries. |

## 4. First, second, and third use rule

| Use count | Required behavior |
|---:|---|
| First official game | Implement locally. Record the mechanic in the game inventory. Do not generalize. |
| Second similar official game | Normally keep local. Compare both implementations. Update both inventories and the atlas with similarities/differences. |
| Third official game | Hard gate. The game MUST NOT proceed until a primitive-pressure ledger decides reuse, promotion, explicit deferral/rejection, or ADR. |

This applies even when copying local code feels faster. Agent cleanup does not bypass the ledger.

## 5. Hard gate decision options

Before a third official game reimplements a repeated shape, the ledger MUST decide exactly one:

1. **Reuse existing promoted primitive.** The helper already exists and fits without hidden policy.
2. **Promote narrow typed helper.** Extract to `game-stdlib`, add tests/docs/examples/anti-examples/benchmarks, and back-port affected games.
3. **Explicitly defer/reject.** Keep local duplication with rationale, risk notes, and next review trigger.
4. **Escalate to ADR.** Required when the proposal changes architecture, replay/hash semantics, visibility, data policy, or kernel vocabulary.

Default: keep local until repeated public pressure proves extraction.

## 6. Primitive-pressure ledger entry fields

A ledger entry MUST include:

```text
Mechanic shape:
Status:
Games exerting pressure:
Relevant files/docs:
What is repeated:
What differs:
Why local duplication is now risky or acceptable:
Decision: reuse / promote / defer-reject / ADR
Why not engine-core:
Why game-stdlib is or is not appropriate:
Data/Rust boundary impact:
Replay/hash impact:
Visibility impact:
Bot impact:
UI/effect impact:
Tests required:
Benchmarks required:
Back-port plan:
Examples:
Anti-examples:
Agent misuse risks:
Review owner/date:
```

Ledger entries may live in a dedicated mechanics folder or in this document once the repository chooses a storage convention. The process exists now; the storage location can be decided at first real pressure.

## 7. Extraction process

1. Implement the first game locally.
2. Implement the second game honestly, usually locally.
3. Compare both game `MECHANICS.md` inventories.
4. Identify the repeated shape and real differences.
5. Record a ledger entry.
6. Decide local/reuse/promote/defer/ADR.
7. If promoted, extract a narrow typed helper into `game-stdlib`.
8. Back-port affected games.
9. Preserve golden traces or intentionally update them with trace notes.
10. Add helper unit/property tests, examples, anti-examples, and game-specific regression tests.
11. Benchmark before and after extraction.
12. Update atlas table, game inventories, rule coverage notes if affected, bot/UI docs if affected, and agent instructions if necessary.

A promoted helper MUST have small APIs, explicit limits, and no hidden game policy.

## 8. Extraction review questions

Before promoting a primitive, answer:

- Which official games exert pressure?
- What shape is actually repeated?
- What differs across games?
- Why is local duplication now riskier than extraction?
- Why does this belong in `game-stdlib`, not `engine-core`?
- Does the helper introduce any game noun into `engine-core`?
- Does the helper create or encourage behavior-in-data?
- How do traces and replay hashes change?
- What examples and anti-examples define the helper boundary?
- What benchmarks prove the helper does not hide slow generic behavior?
- What UI/effect requirements are shared versus game-specific?
- What bot policy, if any, remains game-specific?
- What agent task wording could misuse this helper?

## 9. Anti-patterns

MUST NOT:

- generalize from one game;
- promote mechanic nouns into `engine-core`;
- create a universal behavior language;
- build speculative helpers for private monster games;
- let static data define behavior through a helper;
- let agents “clean up” mechanics without a ledger entry;
- extract a huge helper with flags for every exception;
- update golden traces without explaining behavior/effect/view/hash change;
- call a primitive generic when it only fits one game’s vocabulary;
- smuggle TypeScript legality through UI metadata;
- smuggle bot cheating through shared evaluator helpers.

## 10. Initial atlas table

| Mechanic shape | Games exerting pressure | Status | Current decision | Next gate |
|---|---|---|---|---|
| tiny numeric turn race | `race_to_n` | `local-only` | Keep local; proves plumbing only. Confirmed 2026-06-05 after Gate 1 docs finalization; first official use, no `game-stdlib` promotion. | None. |
| fixed 2D occupancy | `three_marks`, later `column_four` | `first-use local-only` | Three Marks records the first implemented fixed-grid occupancy shape; keep local and compare after `column_four`, with no `game-stdlib` promotion from one game. | Stage 4 review. |
| simple line/pattern detection | `three_marks`, `column_four` | `first-use local-only` | Three Marks records the first implemented row/column/diagonal line scan; keep local and defer extraction until repeated pressure is proven. | Stage 4 review. |
| gravity placement into a column | `column_four` | `local-only` | Too specific for early extraction. | None unless repeated. |
| directional scanning and grouped flips | `directional_flip` | `local-only` initially | Inventory after implementation; may combine with line/scan pressure. | Stage 4 exit. |
| movement/capture/forced continuation | `draughts_lite` | `local-only` initially | Keep game-local until repeated. | Stage 5 exit. |
| deterministic shuffle and hidden draw | `high_card_duel` | `local-only` initially | Keep local until second card/hidden-info game. | Stage 8/9 review. |
| resource accounting | `token_bazaar`, later betting games | `repeated-shape candidate` after second economy/betting use | No formulas in data; compare ledgers before third use. | Stage 9/10 review. |
| simultaneous commitment/reveal | `secret_draft`, later bluffing/reaction games | candidate after second use | Keep local until waiting/reveal behavior repeats. | Stage 11 review. |
| reaction window/pending response | `masked_claims`, later event games | `ADR-required` if generalized broadly | Keep local unless repeated pressure proves small typed helper. | Stage 11+. |

## 11. Stage advancement check

Before advancing a roadmap gate, verify:

- the game has a completed mechanic inventory;
- repeated shapes are listed in this atlas;
- third-use hard gates are resolved;
- any `game-stdlib` promotions have tests, docs, examples, anti-examples, back-ports, and benchmarks;
- `engine-core` remains noun-free;
- no helper creates a behavior language;
- traces/replay effects are preserved or intentionally migrated;
- agents were not allowed to generalize without ledger review.

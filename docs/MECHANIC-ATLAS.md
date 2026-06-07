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
| `promoted primitive` | A narrow typed helper exists in `game-stdlib`; all matching official games use it or carry accepted exceptions; tests, docs, examples, anti-examples, and benchmarks prove the boundary. |
| `promotion-debt-open` | A helper has been promoted, but one or more matching official games have not yet migrated and do not yet have accepted exceptions. This blocks further mechanic-ladder advancement until closed. |
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

## 5A. Promotion conformance lifecycle

A promotion decision creates two obligations:

1. extract or reuse the narrow helper inside `game-stdlib`;
2. conform the official-game surface that created the pressure.

When a primitive is promoted, the same gate MUST either migrate every prior official game that the atlas identifies as using the promoted mechanic shape, or record explicit promotion debt. Promotion debt MUST name the primitive, affected games, current local duplication, reason migration was deferred, behavior-preservation risks, expected closure gate, and evidence needed to close it.

The next implementation spec before further mechanic-ladder advancement MUST close open promotion debt unless an accepted exception or ADR says otherwise. Exceptions MUST name the affected game, promoted primitive, reason for non-migration, evidence proving the game is not duplicating or forking the generic primitive, and the next review trigger.

A promoted primitive is not contract-clean merely because a new game uses it. It is contract-clean only when all matching official games are migrated, audited not applicable, or explicitly excepted.

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
Back-port/conformance plan:
Affected prior games:
Exceptions, if any:
Closure gate if debt is deferred:
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
8. Back-port affected games in the same gate, or record promotion debt with a named closure gate and explicit risk/evidence.
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
- mark a primitive as fully promoted while matching official games remain local without a named debt gate or accepted exception;
- advance to a new mechanic-ladder gate while promotion debt is open;
- extract a huge helper with flags for every exception;
- update golden traces without explaining behavior/effect/view/hash change;
- call a primitive generic when it only fits one game’s vocabulary;
- smuggle TypeScript legality through UI metadata;
- smuggle bot cheating through shared evaluator helpers.

## 10. Initial atlas table

| Mechanic shape | Games exerting pressure | Status | Current decision | Next gate |
|---|---|---|---|---|
| tiny numeric turn race | `race_to_n` | `local-only` | Keep local; proves plumbing only. Confirmed 2026-06-05 after Gate 1 docs finalization; first official use, no `game-stdlib` promotion. | None. |
| fixed 2D occupancy / board-space identity | `three_marks`, `column_four`, `directional_flip`, `draughts_lite` | `promoted primitive` for `game-stdlib::board_space` | Gate 7 superseded the Gate 6 deferral for the narrow behavior-free board-space subset: rectangular dimensions, coordinates, bounds checks, deterministic row-major iteration, signed offsets, stable `rNcM` parse/format, and generic parity are promoted to `game-stdlib::board_space`. Gate 7.1 back-ported that subset to `three_marks`, `column_four`, and `directional_flip`; `race_to_n` is audited not applicable. Occupancy, piece state, placement, gravity, flips, movement, capture, promotion, win detection, effects, UI, WASM, and bot policy remain game-local. | None; Gate 7.1 closed the board-space promotion debt. |
| simple line/pattern detection | `three_marks`, `column_four`, `directional_flip` | `rejected/deferred with rationale` | Gate 6 as-built outcome confirms the ledger decision: keep line/ray/flip scanning local. Static Three Marks lines, Column Four four-direction terminal scans, and Directional Flip eight-direction bracketed flips differ enough that shared pattern detection would risk policy in a helper. See `games/directional_flip/docs/PRIMITIVE-PRESSURE-LEDGER.md` and `games/directional_flip/docs/MECHANICS.md`. | Reopen only for behavior-free ray stepping; do not promote win/flip/capture legality. |
| coordinate/targeted placement | `three_marks`, `column_four`, `directional_flip`, `draughts_lite` | `promoted primitive` for coordinate identity; targeted placement local-only | Gate 7 superseded the Gate 6 coordinate deferral only for stable coordinate identity and behavior-free coordinate operations. Gate 7.1 back-ported that identity where it applies. The helper does not encode targeted placement, origin selection, landing order, forced pass, legal action availability, or action-tree policy. Targeted placement semantics stay in game crates. | None unless later games repeat action semantics in a narrow behavior-free way. |
| column actions | `column_four` | `local-only` | First clear official use of column-targeted actions. Too specific for early extraction. | None unless repeated. |
| gravity placement into a column | `column_four` | `local-only` | First clear official gravity/drop placement. Too specific for early extraction. | None unless repeated. |
| terminal line highlighting | `three_marks`, `column_four`, `directional_flip` | `rejected/deferred with rationale` | Gate 6 as-built outcome confirms the ledger decision: terminal and flip highlighting/effect geometry stay game-local. Three Marks and Column Four highlight terminal lines; Directional Flip highlights Rust-provided preview/apply flip sets and grouped effects, which is related presentation pressure but not one shared primitive. See `games/directional_flip/docs/PRIMITIVE-PRESSURE-LEDGER.md` and `games/directional_flip/docs/MECHANICS.md`. | Reopen only if a later UI/effect spec proves a behavior-free shared highlight projection contract. |
| directional scanning and grouped flips | `directional_flip` | `rejected/deferred with rationale` | Gate 6 as-built outcome confirms the ledger decision: Directional Flip rays, legal bracketing, previews, grouped flips, and effect ordering are implemented locally. The helper question is resolved as defer/reject for Gate 6; no helper was promoted after local benchmark/replay evidence landed. See `games/directional_flip/docs/PRIMITIVE-PRESSURE-LEDGER.md` and `games/directional_flip/docs/MECHANICS.md`. | Reopen only if another official game creates repeated ray-walk pressure and can preserve existing traces/hashes. |
| movement/capture/forced continuation | `draughts_lite` | `local-only` | Gate 7 decision: keep draughts movement, capture, mandatory capture, same-piece forced continuation, promotion, terminal detection, diagnostics, effects, bot policy, and UI guidance game-local. Only behavior-free board-space coordinates are promoted separately. See `games/draughts_lite/docs/PRIMITIVE-PRESSURE-LEDGER.md`. | Reopen only after another official game repeats forced continuation in a way that proves a narrow helper without behavior flags or trace/hash migration. |

## 10A. Open promotion-debt register

| Primitive | Scope | Already conforming | Must retrofit | Audit/no-op | Accepted exceptions | Closure gate |
|---|---|---|---|---|---|---|
| _None_ | _No open promotion debt remains._ | _Not applicable._ | _Not applicable._ | _Not applicable._ | _Not applicable._ | _Not applicable._ |

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

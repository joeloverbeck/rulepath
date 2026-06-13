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

## 9A. Next-phase armed interlocks

The public scaling phase intentionally creates larger N-seat and larger-surface
pressure. This section arms future ledger checks; it does not pre-decide reuse,
promote helpers, or relax the third-use hard gate.

Before each third official use of a repeated shape, write or update a primitive
pressure ledger entry. The entry must decide reuse, narrow promotion,
defer/reject, or ADR before the game proceeds.

| Upcoming pressure | Expected comparison point | Required posture |
|---|---|---|
| River Ledger / Texas Hold'Em base | deterministic shuffle, private hands, community cards, showdown evaluation, public contribution accounting | Compare against `high_card_duel`, `poker_lite`, `plain_tricks`, `masked_claims`, and existing accounting entries before any card/deck/hand/evaluator/accounting helper is reused or promoted. |
| River Ledger side pots / all-in extension | side-pot eligibility, partial allocation, split-pot rationale | Treat as public accounting/allocation pressure. Keep local unless the ledger proves a narrow behavior-free helper. |
| Hearts, Oh Hell, and Spades | follow-suit, trick resolution, trick-winner turn order, hidden-hand no-leak, partnerships | Reopen trick-taking and private-hand ledger entries before the third close use. Partnership/team scoring needs its own comparison rather than folding into seat IDs. |
| Five Hundred Rummy / Rummy 500 family | meld validation, public meld tableau, draw/discard zones, multi-round score target | Start local. Hard-gate before a third meld/tableau/zone helper. Do not encode meld conditions or scoring formulas in data. |
| Star Halma and Pachisi-family race | graph/track topology, route networks, jump/path validation, capture/safety semantics | Compare against prior board-space and graph-map decisions. Topology may be typed content; path legality and capture/safety behavior remain Rust. |
| Four Winds Melds | wall draw/discard rhythm, concealed/exposed sets, discard-claim priority, multi-opponent reaction windows | Treat reaction-window and hidden-zone pressure as hard-gate candidates. Priority/cancellation policy is game-local unless a later ADR says otherwise. |
| Commonwealth Frontier capstone | graph/site/faction/asymmetric-victory/event-resource pressure at larger scale | Resolve graph topology, site control, faction asymmetry, public resource accounting, event timing, and outcome-rationale interlocks before implementation. |

Large surfaces increase benchmark, no-leak, and UI pressure, but scale alone is
not a primitive. A helper earns promotion by repeated implemented public games,
not by anticipated size.

## 10. Initial atlas table

| Mechanic shape | Games exerting pressure | Status | Current decision | Next gate |
|---|---|---|---|---|
| tiny numeric turn race | `race_to_n` | `local-only` | Keep local; proves plumbing only. Confirmed 2026-06-05 after Gate 1 docs finalization; first official use, no `game-stdlib` promotion. | None. |
| fixed 2D occupancy / board-space identity | `three_marks`, `column_four`, `directional_flip`, `draughts_lite`; `frontier_control` and `event_frontier` audited not applicable | `promoted primitive` for `game-stdlib::board_space` | Gate 7 superseded the Gate 6 deferral for the narrow behavior-free board-space subset: rectangular dimensions, coordinates, bounds checks, deterministic row-major iteration, signed offsets, stable `rNcM` parse/format, and generic parity are promoted to `game-stdlib::board_space`. Gate 7.1 back-ported that subset to `three_marks`, `column_four`, and `directional_flip`; `race_to_n` is audited not applicable. Gate 13 records in [../games/frontier_control/docs/PRIMITIVE-PRESSURE-LEDGER.md](../games/frontier_control/docs/PRIMITIVE-PRESSURE-LEDGER.md) that `frontier_control` is also not applicable because its site/edge graph has no rectangular dimensions, coordinates, row-major iteration, or `rNcM` identity. Gate 14 records in [../games/event_frontier/docs/PRIMITIVE-PRESSURE-LEDGER.md](../games/event_frontier/docs/PRIMITIVE-PRESSURE-LEDGER.md) that `event_frontier` is likewise not applicable because it uses named graph sites and trails rather than rectangular coordinates. Occupancy, piece state, placement, gravity, flips, movement, capture, promotion, win detection, graph adjacency, connectivity scoring, effects, UI, WASM, and bot policy remain game-local. | None; Gate 7.1 closed the board-space promotion debt. |
| simple line/pattern detection | `three_marks`, `column_four`, `directional_flip` | `rejected/deferred with rationale` | Gate 6 as-built outcome confirms the ledger decision: keep line/ray/flip scanning local. Static Three Marks lines, Column Four four-direction terminal scans, and Directional Flip eight-direction bracketed flips differ enough that shared pattern detection would risk policy in a helper. See `games/directional_flip/docs/PRIMITIVE-PRESSURE-LEDGER.md` and `games/directional_flip/docs/MECHANICS.md`. | Reopen only for behavior-free ray stepping; do not promote win/flip/capture legality. |
| coordinate/targeted placement | `three_marks`, `column_four`, `directional_flip`, `draughts_lite` | `promoted primitive` for coordinate identity; targeted placement local-only | Gate 7 superseded the Gate 6 coordinate deferral only for stable coordinate identity and behavior-free coordinate operations. Gate 7.1 back-ported that identity where it applies. The helper does not encode targeted placement, origin selection, landing order, forced pass, legal action availability, or action-tree policy. Targeted placement semantics stay in game crates. | None unless later games repeat action semantics in a narrow behavior-free way. |
| column actions | `column_four` | `local-only` | First clear official use of column-targeted actions. Too specific for early extraction. | None unless repeated. |
| gravity placement into a column | `column_four` | `local-only` | First clear official gravity/drop placement. Too specific for early extraction. | None unless repeated. |
| terminal line highlighting | `three_marks`, `column_four`, `directional_flip` | `rejected/deferred with rationale` | Gate 6 as-built outcome confirms the ledger decision: terminal and flip highlighting/effect geometry stay game-local. Three Marks and Column Four highlight terminal lines; Directional Flip highlights Rust-provided preview/apply flip sets and grouped effects, which is related presentation pressure but not one shared primitive. See `games/directional_flip/docs/PRIMITIVE-PRESSURE-LEDGER.md` and `games/directional_flip/docs/MECHANICS.md`. | Reopen only if a later UI/effect spec proves a behavior-free shared highlight projection contract. |
| directional scanning and grouped flips | `directional_flip` | `rejected/deferred with rationale` | Gate 6 as-built outcome confirms the ledger decision: Directional Flip rays, legal bracketing, previews, grouped flips, and effect ordering are implemented locally. The helper question is resolved as defer/reject for Gate 6; no helper was promoted after local benchmark/replay evidence landed. See `games/directional_flip/docs/PRIMITIVE-PRESSURE-LEDGER.md` and `games/directional_flip/docs/MECHANICS.md`. | Reopen only if another official game creates repeated ray-walk pressure and can preserve existing traces/hashes. |
| movement/capture/forced continuation | `draughts_lite` | `local-only` | Gate 7 decision: keep draughts movement, capture, mandatory capture, same-piece forced continuation, promotion, terminal detection, diagnostics, effects, bot policy, and UI guidance game-local. Only behavior-free board-space coordinates are promoted separately. See `games/draughts_lite/docs/PRIMITIVE-PRESSURE-LEDGER.md`. | Reopen only after another official game repeats forced continuation in a way that proves a narrow helper without behavior flags or trace/hash migration. |
| follow-suit legality | `plain_tricks` | `local-only` first official use | Gate 10.1 records first official follow-suit pressure: leader may play any held card; follower must play led suit if holding it, otherwise any held card. The legality depends on hidden owner hand contents and is generated/validated only in Rust. No `game-stdlib` promotion is authorized. See `games/plain_tricks/docs/MECHANICS.md`. | Revisit only after another official trick/follow game repeats the shape, and hard-gate before a third similar use. |
| trick resolution / led-suit comparator | `plain_tricks` | `local-only` first official use | Gate 10.1 records first official trick-resolution pressure: led suit is established by the leader; same-suit higher rank wins; off-suit follower cards never win. This remains game-local and carries no engine-core card, suit, rank, or trick noun. | Revisit only after another official game repeats trick resolution without adding incompatible trump, point-card, partnership, or tie policy. |
| trick-winner-leads turn order | `plain_tricks` | `local-only` first official use | Gate 10.1 records first official trick-winner-led turn order: a resolved trick's winner becomes next trick leader unless the round closes. This is encoded in Plain Tricks state transitions and semantic effects only. | Revisit only after repeated trick-winner-led turn sequencing appears in another official game. |
| deal rotation / trick-round redeal | `plain_tricks` | `local-only` first official use | Gate 10.1 records first official trick-round deal rotation: round 1 starts with `seat_0`, round 2 redeals from the continuing RNG stream and starts with `seat_1`. This is not a generic deal/redeal primitive. | Revisit only after another official game repeats deterministic deal rotation with comparable replay/no-leak pressure. |
| public resource accounting / visible market costs | `token_bazaar`, `poker_lite`, `event_frontier` | `rejected/deferred with rationale` | Gate 9 records first official public resource/accounting pressure in Token Bazaar. Gate 10 adds Crest Ledger as a second similar but still distinct public-accounting proof: opening contributions, exact pledge additions, shared-pool accounting, yield award, showdown allocation, and split handling. Gate 14 records the third-use hard-gate decision in [../games/event_frontier/docs/PRIMITIVE-PRESSURE-LEDGER.md](../games/event_frontier/docs/PRIMITIVE-PRESSURE-LEDGER.md): defer/reject extraction because Event Frontier's faction-owned operation funding, pass income, Reckoning income, and capped resources differ structurally from market purchase accounting and shared-pool terminal allocation. No `game-stdlib` promotion, no `engine-core` resource noun, and no §10A promotion debt are authorized. | Reopen only when another official game repeats a close public economy/accounting shape, or when repeated defects prove a narrow behavior-free helper can cover the real shared surface without policy flags. |

## 10A. Open promotion-debt register

Current debt: _None_.

| Primitive | Scope | Already conforming | Must retrofit | Audit/no-op | Accepted exceptions | Closure gate |
|---|---|---|---|---|---|---|
| _None_ | _No open promotion debt remains._ | _Not applicable._ | _Not applicable._ | _Not applicable._ | _Not applicable._ | _Not applicable._ |

## 10B. Deferred and candidate mechanic pressure register

| Mechanic shape | Games/candidates exerting pressure | Status | Current decision | Next gate |
|---|---|---|---|---|
| deterministic shuffle / private hand / staged reveal | `high_card_duel`, `poker_lite`, `plain_tricks`, `masked_claims`; `flood_watch` and `event_frontier` reviewed as not full private-hand uses; `frontier_control` reviewed as non-use | `rejected/deferred with rationale` | Gate 8 records first official local card/deck pressure: deterministic shuffle, private hands, hidden face-down commitment, simultaneous reveal, viewer-filtered effects, and no-leak public replay export. Gate 10 adds Crest Ledger as the second similar official use with deterministic shuffle, owner-private crests, hidden center crest, staged center reveal, grouped showdown reveal, yield without private reveal, and browser no-leak export evidence. Gate 10.1 records the third-use hard-gate decision in [../games/plain_tricks/docs/PRIMITIVE-PRESSURE-LEDGER.md](../games/plain_tricks/docs/PRIMITIVE-PRESSURE-LEDGER.md): defer/reject extraction and keep local because holding size, deal shape, reveal timing, follow-suit legality coupling, diagnostics, replay export, and bot inputs differ enough that a helper would either be trivial shuffle-only code or behavior-bearing policy. Gate 11 reopens the decision for `masked_claims` in [../games/masked_claims/docs/PRIMITIVE-PRESSURE-LEDGER.md](../games/masked_claims/docs/PRIMITIVE-PRESSURE-LEDGER.md) and again defer/rejects extraction: mask hands, hidden reserve, claim pedestal, accepted masks that never reveal, challenged one-mask reveal, claim-path redaction, response-window policy, and bot inputs remain game-local. Gate 12 reviews `flood_watch` in [../games/flood_watch/docs/PRIMITIVE-PRESSURE-LEDGER.md](../games/flood_watch/docs/PRIMITIVE-PRESSURE-LEDGER.md) and records that it has deterministic shuffle and staged forecast/draw reveal, but not per-seat private holdings; the full fifth-use trigger does not fire. Gate 13 records in [../games/frontier_control/docs/PRIMITIVE-PRESSURE-LEDGER.md](../games/frontier_control/docs/PRIMITIVE-PRESSURE-LEDGER.md) that `frontier_control` has no game-rule randomness, shuffle, hidden private holdings, hidden order, or staged reveal; the trigger is untouched. Gate 14 records in [../games/event_frontier/docs/PRIMITIVE-PRESSURE-LEDGER.md](../games/event_frontier/docs/PRIMITIVE-PRESSURE-LEDGER.md) that `event_frontier` has deterministic shuffle and public current/next-card reveal, but no per-seat private holdings; the full trigger still does not fire. No `game-stdlib` promotion, no `engine-core` noun, no existing trace/hash migration, and no §10A promotion debt are authorized. `blackjack_lite` remains a deferred comparison case only under [ADR 0006](adr/0006-blackjack-lite-roadmap-placement.md). | Reopen before a fifth official game repeats deterministic shuffle plus private holdings plus redacted reveal/export, or earlier if local shuffle/deal divergence, no-leak helper pressure, benchmark evidence, or a proposed runtime visibility/reveal/export helper creates new evidence. |
| public resource accounting / shared ledgers | `token_bazaar`, `poker_lite`, `event_frontier` | `rejected/deferred with rationale` | Gate 9 keeps Token Bazaar resource/accounting local as first official use. Gate 10 adds Crest Ledger as second similar pressure with public contributions, a shared pool, exact pledge additions, yield award, showdown allocation, and split handling. Gate 10.1 explicitly records that Plain Tricks trick-count scoring is scoring/outcome, not a public resource-accounting or shared-ledger third use: it has no payments, costs, pooled assets, conservation ledger, or terminal allocation of resources. Gate 14 records the third-use hard-gate decision in [../games/event_frontier/docs/PRIMITIVE-PRESSURE-LEDGER.md](../games/event_frontier/docs/PRIMITIVE-PRESSURE-LEDGER.md): defer/reject extraction because Event Frontier has faction-owned operation funding, pass income, Reckoning income, and capped public pools rather than market purchases or shared-pool terminal allocation. No formulas in data, no helper promotion, and no §10A promotion debt. | Reopen only when another official game repeats a close public economy/accounting shape, or when repeated defects prove a narrow behavior-free helper can cover the real shared surface without policy flags. |
| bounded pledge rounds / shared-pool terminal allocation | `poker_lite` | `local-only` first official use | Gate 10 records first official bounded pledge/shared-pool pressure: two fixed pledge rounds, one-lift cap per round, response choices, immediate yield allocation, and grouped showdown allocation. Keep game-local; one official use cannot justify `game-stdlib` extraction. | Revisit only after a second pledge/shared-pool game, and hard-gate before a third similar use. |
| simultaneous commitment/reveal + visible draft-pool removal | `secret_draft`; later bluffing/reaction games are comparison candidates | `local-only` first official use; repeated-shape candidate after second use | Gate 9.1 records first official local use: hidden per-seat commitments, public pending booleans, synchronized reveal batch, deterministic conflict fallback, visible pool removal, viewer-scoped replay export, and browser no-leak UI. Gate 11 review in [../games/masked_claims/docs/PRIMITIVE-PRESSURE-LEDGER.md](../games/masked_claims/docs/PRIMITIVE-PRESSURE-LEDGER.md) confirms `masked_claims` is not a second use: it has one sequential claimant placement, one responder window, no synchronized multi-seat reveal batch, and no visible draft-pool removal. No `game-stdlib` promotion is authorized; see `games/secret_draft/docs/MECHANICS.md`. Keep local until waiting/reveal behavior repeats. | Revisit when a later official game repeats synchronized hidden commitments plus visible pool removal. |
| reaction window/pending response | `masked_claims`, later event games; `flood_watch`, `frontier_control`, and `event_frontier` reviewed as not reaction-capable | `local-only` first official use; `ADR-required` if generalized broadly | Gate 11 records first official local use in [../games/masked_claims/docs/PRIMITIVE-PRESSURE-LEDGER.md](../games/masked_claims/docs/PRIMITIVE-PRESSURE-LEDGER.md): a claim opens one timeout-free accept/challenge window, the responder receives exactly the response choices, the claimant receives an empty tree with safe waiting metadata, and resolution is conditional on response choice and hidden information. Gate 12 reviews `flood_watch` in [../games/flood_watch/docs/PRIMITIVE-PRESSURE-LEDGER.md](../games/flood_watch/docs/PRIMITIVE-PRESSURE-LEDGER.md) and records that its environment phase is automation, not a response window: no seat responds to another seat's pending action, no interrupt/cancel window opens, and teammate waiting metadata is not enough to count as reaction pressure. Gate 13 reviews `frontier_control` in [../games/frontier_control/docs/PRIMITIVE-PRESSURE-LEDGER.md](../games/frontier_control/docs/PRIMITIVE-PRESSURE-LEDGER.md) and records that clashes resolve immediately inside the mover's command; no responder tree or pending response exists. Gate 14 reviews `event_frontier` in [../games/event_frontier/docs/PRIMITIVE-PRESSURE-LEDGER.md](../games/event_frontier/docs/PRIMITIVE-PRESSURE-LEDGER.md) and records that eligibility is sequential initiative, not a response window. No `game-stdlib` promotion is authorized. Broad priority systems, interrupt stacks, cancellation chains, timeout policy, hosted networking, or cross-game reaction engines require ADR review before promotion. | Reopen when a second reaction-capable official game appears; hard-gate before a third similar use. |
| shared-outcome cooperative terminal | `flood_watch`; `frontier_control` and `event_frontier` reviewed as not second uses | `local-only` first official use | Gate 12 records first official shared-outcome cooperative terminal pressure in [../games/flood_watch/docs/PRIMITIVE-PRESSURE-LEDGER.md](../games/flood_watch/docs/PRIMITIVE-PRESSURE-LEDGER.md): all seats win together on deck exhaustion or lose together on district inundation, with no per-seat winner, score ranking, or tiebreaker. Gate 13 reviews `frontier_control` in [../games/frontier_control/docs/PRIMITIVE-PRESSURE-LEDGER.md](../games/frontier_control/docs/PRIMITIVE-PRESSURE-LEDGER.md) and records that it is competitive: factions score on one comparable numeric track and exactly one faction wins. Gate 14 reviews `event_frontier` in [../games/event_frontier/docs/PRIMITIVE-PRESSURE-LEDGER.md](../games/event_frontier/docs/PRIMITIVE-PRESSURE-LEDGER.md) and records that it is also competitive, with asymmetric faction victory conditions and one winner. Keep terminal outcome, rationale, tests, and UI copy game-local; no `game-stdlib` or `engine-core` shared-outcome helper is authorized. | Revisit when another official game adds team/shared victory comparison pressure. |
| event-deck environment automation | `flood_watch`, `event_frontier` | `repeated-shape candidate` | Gate 12 records first official event-deck automation pressure in [../games/flood_watch/docs/PRIMITIVE-PRESSURE-LEDGER.md](../games/flood_watch/docs/PRIMITIVE-PRESSURE-LEDGER.md): a deterministic environment batch resolves as a Rust consequence of the turn-ending/final-budget command, emits semantic effects, and never appears as a synthetic command-stream actor or TypeScript timer. Gate 14 records the second comparison in [../games/event_frontier/docs/PRIMITIVE-PRESSURE-LEDGER.md](../games/event_frontier/docs/PRIMITIVE-PRESSURE-LEDGER.md): Event Frontier's player-facing event/op/pass card flow and automatic Reckonings are related but distinct from Flood Watch's pure environment batch. Keep event kinds, card flow, draw/reveal cadence, resolution order, effects, replay/export redaction, and diagnostics game-local; no event-deck helper is authorized. | Revisit when another official game repeats event decks or periodic automation; hard-gate before a third close event-deck automation shape. |
| role-modified action effects / public faction modifiers | `flood_watch`, `frontier_control`, `event_frontier` | `repeated-shape candidate` | Gate 12 records first official public role-modifier pressure in [../games/flood_watch/docs/PRIMITIVE-PRESSURE-LEDGER.md](../games/flood_watch/docs/PRIMITIVE-PRESSURE-LEDGER.md): Pumpwright and Levee Warden change local bail/reinforce magnitudes through Rust validation/application/effects only. Gate 13 records the second comparison in [../games/frontier_control/docs/PRIMITIVE-PRESSURE-LEDGER.md](../games/frontier_control/docs/PRIMITIVE-PRESSURE-LEDGER.md): Frontier Control factions are related public asymmetry pressure, but they hold disjoint action vocabularies, asymmetric clash rules, and faction-specific scoring formulas rather than magnitude modifiers on a shared action set. Gate 14 records the Event Frontier comparison in [../games/event_frontier/docs/PRIMITIVE-PRESSURE-LEDGER.md](../games/event_frontier/docs/PRIMITIVE-PRESSURE-LEDGER.md): edicts are temporal event-imposed public modifiers, and factions have different operations and victory conditions; this is related pressure but not one close role-modifier helper shape. Keep role IDs, faction IDs, edicts, previews, validation, bot priorities, effect payloads, scoring, and UI labels game-local; no role/faction/edict helper is authorized. | Revisit when another official game repeats a close public role/faction/modifier shape; hard-gate before a third close shape. |
| multi-action turn budgets | `flood_watch`, `frontier_control`; `event_frontier` reviewed as non-use | `repeated-shape candidate` | Gate 12 records first official multi-action budget pressure in [../games/flood_watch/docs/PRIMITIVE-PRESSURE-LEDGER.md](../games/flood_watch/docs/PRIMITIVE-PRESSURE-LEDGER.md): the active seat spends several validated actions, the legal tree regenerates with remaining-budget metadata, `end_turn` remains legal, and the teammate receives safe waiting metadata. Gate 13 records the second official use in [../games/frontier_control/docs/PRIMITIVE-PRESSURE-LEDGER.md](../games/frontier_control/docs/PRIMITIVE-PRESSURE-LEDGER.md): the active faction spends a two-action budget, the tree regenerates after each action, `end_turn` remains legal, and the non-active faction receives safe waiting metadata; round scoring after the Garrison turn remains game-local. Gate 14 records in [../games/event_frontier/docs/PRIMITIVE-PRESSURE-LEDGER.md](../games/event_frontier/docs/PRIMITIVE-PRESSURE-LEDGER.md) that Event Frontier is a non-use: each faction makes one event-card choice, and an operation is one compound command rather than a regenerated budget sequence. Keep budget accounting, action ordering, diagnostics, and UI presentation local for the two existing budgeted games. | Reopen when a third actual budgeted-turn official game appears, or immediately if Event Frontier implementation drifts into regenerated budgeted commands. |
| graph-map topology / adjacency legality / connectivity scoring | `frontier_control`, `event_frontier`; Gate P is a comparison candidate | `repeated-shape candidate` | Gate 13 records first official site/edge graph pressure in [../games/frontier_control/docs/PRIMITIVE-PRESSURE-LEDGER.md](../games/frontier_control/docs/PRIMITIVE-PRESSURE-LEDGER.md): typed sites and edges constrain movement legality and Rust-computed connectivity determines supplied-stake scoring. Gate 14 records the second comparison in [../games/event_frontier/docs/PRIMITIVE-PRESSURE-LEDGER.md](../games/event_frontier/docs/PRIMITIVE-PRESSURE-LEDGER.md): Event Frontier uses named sites/trails for adjacency legality and presence majority, but has no connectivity scoring or path-scored supply. Keep graph IDs, edge validation, adjacency, traversal, diagnostics, effects, bot policy, and UI projection local; no graph/pathfinding helper is authorized. | Revisit when another official game repeats graph-map topology, adjacency-constrained legality, or connectivity scoring; hard-gate before a third close graph helper shape. |
| site control / deterministic contest resolution | `frontier_control`, `event_frontier`; Gate P is a comparison candidate | `repeated-shape candidate` | Gate 13 records first official site-control/contest pressure in [../games/frontier_control/docs/PRIMITIVE-PRESSURE-LEDGER.md](../games/frontier_control/docs/PRIMITIVE-PRESSURE-LEDGER.md): public occupancy, forts, stakes, and deterministic guard/crew clashes determine control and scoring. Gate 14 records the Event Frontier comparison in [../games/event_frontier/docs/PRIMITIVE-PRESSURE-LEDGER.md](../games/event_frontier/docs/PRIMITIVE-PRESSURE-LEDGER.md): Event Frontier uses public presence majority and site scoring, but has no clash/contest resolution and caches do not count as presence. Keep occupancy, majority/control, contest rules where present, diagnostics, effects, UI, and bot policy local; no control or contest helper is authorized. | Revisit when another official game repeats public site control plus deterministic contest resolution; hard-gate before a third close control/contest shape. |
| faction-asymmetric action sets and scoring | `frontier_control`, `event_frontier`; Gate P is a comparison candidate | `repeated-shape candidate` | Gate 13 records first official faction-asymmetry pressure in [../games/frontier_control/docs/PRIMITIVE-PRESSURE-LEDGER.md](../games/frontier_control/docs/PRIMITIVE-PRESSURE-LEDGER.md): Prospectors and Garrison have disjoint action vocabularies, asymmetric clash rules, faction-specific scoring formulas, per-faction UI presentation, and per-faction bot policies. Gate 14 records the second comparison in [../games/event_frontier/docs/PRIMITIVE-PRESSURE-LEDGER.md](../games/event_frontier/docs/PRIMITIVE-PRESSURE-LEDGER.md): Charter and Freeholders have different operations, different instant victory conditions, a both-met rule, a final fallback, and per-faction bot policies. Keep all faction/asymmetry behavior local; no generic faction, action-set, scoring, victory, UI, or bot helper is authorized. | Revisit when another official game repeats comparable faction-asymmetric action/scoring pressure; hard-gate before a third close faction-asymmetry helper shape. |
| event-card initiative/eligibility sequencing | `event_frontier` | `local-only` first official use | Gate 14 records first official player-facing event-card initiative/eligibility pressure in [../games/event_frontier/docs/PRIMITIVE-PRESSURE-LEDGER.md](../games/event_frontier/docs/PRIMITIVE-PRESSURE-LEDGER.md): printed first faction, eligibility markers, first/second menus, pass eligibility, and next-card consequences remain local to `games/event_frontier`. | Revisit when another official game repeats player-facing event-card eligibility sequencing. |
| periodic scoring/reset pipeline | `event_frontier` | `local-only` first official use | Gate 14 records first official Reckoning-style scoring/reset pipeline pressure in [../games/event_frontier/docs/PRIMITIVE-PRESSURE-LEDGER.md](../games/event_frontier/docs/PRIMITIVE-PRESSURE-LEDGER.md): victory check, site scoring, income, edict expiry, and eligibility reset remain local and ordered in Rust. | Revisit when another official game repeats scheduled scoring/reset cards or rounds with comparable replay pressure. |
| asymmetric instant victory conditions | `event_frontier` | `local-only` first official use | Gate 14 records first official asymmetric instant victory pressure in [../games/event_frontier/docs/PRIMITIVE-PRESSURE-LEDGER.md](../games/event_frontier/docs/PRIMITIVE-PRESSURE-LEDGER.md): different faction instant-win checks, both-met rule, final fallback, and outcome rationale remain local. | Revisit when another official game repeats asymmetric instant victory conditions. |
| timed rule-exception modifiers | `event_frontier` | `local-only` first official use | Gate 14 records first official timed rule-exception pressure in [../games/event_frontier/docs/PRIMITIVE-PRESSURE-LEDGER.md](../games/event_frontier/docs/PRIMITIVE-PRESSURE-LEDGER.md): edict activation, stable modifier ordering, validation/application hooks, and expiry remain local. Static card data does not define behavior. | Revisit when another official game repeats expiring rule exceptions. |

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

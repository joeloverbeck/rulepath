# Gate 15.1 — River Ledger all-in / side pots

## 1. Header

| Field | Value |
|---|---|
| Spec ID | `GAT15-1-RIVLED-ALLIN-SIDEPOTS-001` |
| File | `archive/specs/gate-15-1-river-ledger-all-in-side-pots.md` |
| Roadmap stage | Stage 15.1 / Public scaling phase |
| Roadmap build gate | Gate 15.1 |
| Game | `river_ledger` |
| Status | `Done` |
| Date | 2026-06-20 |
| Owner | Rulepath maintainers |
| Predecessor | Gate 15 — River Ledger / Texas Hold'Em base (`Done`) |
| Successor | Gate 16 — Hearts |
| Authority order | `docs/FOUNDATIONS.md` → `docs/ARCHITECTURE.md` → `docs/ENGINE-GAME-DATA-BOUNDARY.md` → applicable area contracts and accepted ADRs → `docs/ROADMAP.md` → this spec → future `tickets/` packets |
| Primary planning authorities | `specs/README.md`, `docs/ROADMAP.md`, `docs/MECHANIC-ATLAS.md`, `archive/specs/gate-15-river-ledger-texas-holdem-base.md` |
| Delivery boundary | Completed and archived. This document planned the delta; ticket outcomes carry the implementation slices and this Outcome records final gate closeout. |

Normative terms such as **MUST**, **MUST NOT**, **SHOULD**, and **MAY** are subordinate to the authority order above. Any conflict is resolved in favor of the earlier authority.

## 2. Objective

Gate 15.1 is the locked next implementation-spec unit; this document confirms that determination and does not re-open it.

| Determination evidence | Finding |
|---|---|
| `specs/README.md` active-epoch tracker | Order 6, Gate 15.1, is the lowest unit whose status is not `Done`; its spec is an unwritten seed. |
| `docs/MECHANIC-ATLAS.md` River Ledger entry | The entry explicitly says to reopen for Gate 15.1 side-pot/all-in work. |
| `docs/MECHANIC-ATLAS.md` open promotion-debt register | The register is empty, so no promotion-debt interlock blocks this gate. |
| `docs/ROADMAP.md` §15.1 | The gate proves partial eligibility, nested pots, all-in contribution caps, multi-way allocation explanation, and public-resource/accounting pressure. |
| `docs/adr/0007-next-public-scaling-phase-and-gate-p-tail.md` | Accepted ADR 0007 admits the public scaling phase containing Gate 15+. |
| `archive/specs/gate-15-river-ledger-texas-holdem-base.md` | The shipped Gate 15 base explicitly defers stack limits, all-in states, and side pots to this successor. |

The objective is to add a **finite-stack, all-in, and ordered side-pot accounting layer** to the already-shipped River Ledger hand without rebuilding its base game. The completed gate must make every contribution cap, eligibility boundary, uncalled return, pot award, split, and remainder:

1. decided by typed Rust behavior;
2. deterministic under replay, hashing, serialization, simulation, and browser projection;
3. visible through viewer-safe Rust projections and explanations;
4. legal for 3–6 seats, including bots and short-stack decisions;
5. covered by named rules, tests, golden traces, pairwise no-leak evidence, and benchmarks.

The shipped baseline remains authoritative: deterministic 3–6-seat setup; fixed-limit, capped-raise betting; a per-seat contribution ledger; deterministic card dealing; the seven-card evaluator; foldout and showdown; single-pot split/remainder allocation; Rust-authored outcome explanations; N-player no-leak; replay; bots; WASM; and the public renderer. Gate 15.1 is a pure delta over that baseline.

**Research reconciliation.** Contemporary public poker rule sets do not state a single universal fixed-limit reopening rule. The Poker TDA and the 2025 WSOP rules use a half-bet threshold for limit play, while incomplete all-ins in no-limit/pot-limit generally require a full cumulative raise to reopen action. The settled Gate 15.1 brief instead requires River Ledger to retain fixed units and to treat an all-in increase short of a full fixed unit as non-reopening. This spec therefore defines an explicit **River Ledger full-unit reopening rule** in §3: it is a deliberate game rule, documented as such, not a claim that all poker authorities use it.

## 3. Scope

### 3.1 Shipped baseline that MUST NOT be rebuilt

The implementation starts from the target-commit River Ledger surface, including:

- 3–6 public seats and deterministic button/blind assignment;
- fixed-limit small- and big-bet streets with the existing raise cap;
- `Fold`, `Check`, `Call`, `Bet`, and `Raise` action paths;
- a per-seat street/total contribution ledger and aggregate single pot;
- deterministic board runout, evaluator, foldout, showdown, split winners, and button-ordered remainders;
- viewer-scoped private hands, public board state, Rust-authored showdown details, and pairwise no-leak coverage;
- L0/L1/L2 bots, replay, serialization, WASM projection, browser UI, smoke coverage, and benchmarks;
- all post-Gate-15 correctness and presentation fixes already recorded as `Done` in `specs/README.md`.

Work that merely reimplements, renames, or re-proves those shipped mechanics is out of scope unless a narrow migration is required by finite stacks or multiple pots.

### 3.2 Stack-model decision

The delegated choice is resolved between exactly these options:

1. **Option (a): equal fixed starting stacks for all seats.** This is the smallest deterministic setup and the required default presentation, but in River Ledger's shipped single-hand match it does not make genuinely different live contribution caps normally reachable: non-folded seats begin with the same cap, while unmatched top excess is returned.
2. **Option (b): configurable per-seat starting stacks via setup metadata.** This adds an ordered public setup vector, deterministic validation, asymmetric short-stack fixtures, and more N-seat/no-leak surface area.

**Decision: choose option (b), configurable per-seat starting stacks via typed setup metadata, with an equal fixed default of 24 contribution units for every seat.**

This is one model, not a hybrid choice: the model is configurable; its default profile is equal.

Rationale:

- Gate 15 is a single-hand match. Genuine main/side-pot eligibility ladders need different live contribution caps during that hand. Option (a) would make the gate's signature behavior unreachable in ordinary public play except through synthetic internal states.
- An ordered stack vector is deterministic and fits the N-seat setup/catalog bridge already shipped by Infra A–D.
- Starting and remaining stacks are public poker accounting facts; adding them does not require private-data authority, although every projection and replay lane still receives no-leak tests.
- The added setup/test cost is the point of Gate 15.1's multi-way accounting pressure and remains bounded to one game.
- The equal 24-unit default preserves simple setup, works across the existing 2-unit/4-unit fixed-bet structure, and provides six big-bet units without implying money, cash value, tournament chips, or casino product mimicry.

Required setup contract:

- `SetupOptions` gains an ordered per-seat starting-stack vector or an equivalent typed N-seat structure.
- Omission selects the deterministic equal default of 24 units per seat.
- The vector length MUST equal the selected seat count and preserve canonical seat order.
- Each stack MUST be a positive, bounded integer. Behavior-critical arithmetic MUST use checked operations and a game-local amount type wide enough to hold the maximum six-seat sum; silent saturation is forbidden.
- The public setup UI MAY expose equal and neutral asymmetric presets plus direct numeric entry, but Rust validates all values and remains the sole authority.
- Minimum required asymmetric acceptance fixtures are `[8, 16, 24]` for 3 seats and `[4, 8, 12, 16, 20, 24]` for 6 seats. These are neutral contribution units, not currency.
- Stack configuration is hand-local. No stack carries into another match.

### 3.3 In scope

1. **Finite stack state**
   - Per-seat starting stack, remaining stack, street contribution, total contribution, and status.
   - A typed all-in status distinct from live/action-capable, folded, and showdown-resolved presentation states.
   - Checked conservation invariants before, during, and after settlement.
   - Blind posting capped by the posting seat's remaining stack.

2. **Stack-capped fixed-limit actions**
   - Keep the existing action families and action-path segments: all-in is a consequence/qualification of `Call`, `Bet`, or `Raise`, not an arbitrary generic amount-selection action.
   - Rust action metadata and previews MUST expose the exact amount, stack after action, whether the action is all-in, whether it is a full bet/raise, whether it consumes a raise-cap slot, and whether the actor's raise right is open.
   - If `amount_owed > stack_remaining`, legal actions are `Fold` and `Call` for the entire remaining stack; the call is all-in and may be less than the amount owed.
   - If `amount_owed == stack_remaining`, a full call is legal and leaves the seat all-in.
   - If a seat can cover the call but cannot cover `call + street_unit`, a short raise-all-in for the entire stack is legal only when raising is otherwise legal and the cap is open; an ordinary call remains separately legal.
   - If no wager is faced and `0 < stack_remaining < street_unit`, a short opening bet-all-in is legal when betting is otherwise legal; checking remains legal.
   - A full fixed bet or raise that consumes the actor's exact remaining stack is an all-in full bet/raise.
   - A zero-stack all-in seat never receives another action turn.

3. **River Ledger full-unit reopening rule**
   - A full fixed bet or raise increases the wager by the street unit, consumes one raise-cap slot, and reopens raising for seats entitled to respond.
   - A single incomplete all-in increase smaller than the street unit does not consume a raise-cap slot and does not reopen raising for a seat that already acted.
   - A seat that has not yet acted in the current betting sequence retains its ordinary raise right.
   - Multiple incomplete all-in increases are accumulated per previously acted seat. Raising reopens for that seat only when the cumulative increase faced since its last completed action reaches at least one full street unit.
   - Every seat that owes additional contribution must still respond with fold, call, or—if its right is open and the cap permits—raise.
   - Once the existing raise cap is reached, neither a full raise nor a short raise-all-in is legal; a seat may call, call all-in, check, or fold as otherwise applicable.
   - Per-seat raise-right/reopen state MUST be explicit deterministic Rust state. It MUST NOT be inferred in TypeScript.

4. **Betting completion and automatic runout**
   - Folded and all-in seats are excluded from future actor selection.
   - A street closes when every action-capable non-folded seat has responded to the current amount and no further response is owed.
   - If at least two non-folded seats remain and none can act because all are all-in, the remaining board is dealt deterministically and the hand advances directly to showdown.
   - If one non-all-in seat remains alongside all-in seats and owes nothing, unmatched excess is returned before deterministic runout; no meaningless check loop is generated.
   - If only one non-folded seat remains, ordinary foldout settlement applies without revealing hidden cards.

5. **Canonical layered side-pot construction**
   - Construct pots from total contributions after betting is complete or when a terminal settlement requires it.
   - Folded contributions remain in pot amounts.
   - Eligibility for a layer includes only non-folded seats whose contribution reaches that layer.
   - Unmatched top excess contributed by exactly one seat is returned to that seat and is not a pot.
   - Adjacent accounting layers with identical eligibility sets are coalesced before allocation so a folded contribution boundary cannot create artificial pot identities or repeated odd-unit awards.
   - Final pots are ordered by ascending contribution cap and identified deterministically as main pot, side pot 1, side pot 2, and so on.

The required pure algorithm, using checked integer arithmetic, is:

```text
input:
    total contribution c[seat] for every seat, including folded seats
    folded/non-folded status for every seat

levels = sorted distinct positive values in c
previous = 0
segments = []
returns = []

for level in levels:
    contributors = seats where c[seat] >= level
    delta = level - previous
    amount = delta * contributors.count
    eligible = contributors where seat is not folded

    if contributors.count == 1:
        returns.append(sole contributor, amount)
    else:
        assert eligible is not empty for a valid non-foldout history
        segments.append(
            lower_cap = previous,
            upper_cap = level,
            amount,
            contributors in canonical seat order,
            eligible in canonical seat order
        )

    previous = level

coalesce adjacent segments whose eligible sets are identical
assign stable pot ids in ascending-cap order
assert sum(pot amounts) + sum(returns) == sum(c)
```

6. **Per-pot showdown and settlement**
   - Evaluate winners independently within each pot's eligible seats.
   - A pot with one eligible seat is awarded without comparing or exposing other private hands.
   - A tied pot is split by integer units.
   - Each pot's remainder is assigned independently using the already-shipped button order: first eligible winning seat to the left of the button, then onward in table order.
   - Side pots are never recombined for allocation, even if the same seats win more than one.
   - Aggregate per-seat awards and final stacks are derived from the ordered per-pot awards plus uncalled returns.
   - The following invariants are mandatory:
     - no contribution exceeds its seat's starting stack;
     - no remaining stack is negative;
     - during play, `sum(remaining stacks) + sum(total contributions) == sum(starting stacks)`;
     - before settlement, `sum(pot amounts) + sum(uncalled returns) == sum(total contributions)`;
     - for every pot, `sum(shares) == pot amount`;
     - every winner is eligible for that pot;
     - folded seats can contribute but cannot be eligible;
     - after settlement, `sum(final stacks) == sum(starting stacks)`;
     - no final pot has only one contributor; such a top layer must have been returned.

7. **Public projection and explanation**
   - Public state includes each seat's starting stack, remaining stack, committed contribution, actionability/all-in status, aggregate pot, ordered current/final pot breakdown, and public eligibility.
   - Terminal outcome explains every pot separately: pot id, amount, eligible seats, winner(s), shares, remainder amount/order, and decisive public reason.
   - Uncalled excess is shown as a return, not as a won pot.
   - Foldout and sole-eligible awards explain eligibility without exposing cards.
   - Showdown card/category details remain subject to the existing viewer-scoped reveal contract. The fact that a seat is eligible is public; an unrevealed hand's contents are not.
   - Rust authors all allocation labels, explanatory rows, accessibility text, and effect semantics. TypeScript renders only the projection.

8. **Bots**
   - Every shipped bot level uses the legal-action API and an authorized seat view.
   - Bots correctly select among fold, short/full call, short/full bet, and short/full raise actions presented by Rust.
   - L1/L2 policy may use public stack, pot, call amount, and eligibility facts, but never private cards belonging to another seat, deck order, internal candidate rankings, or omniscient rollout.
   - Bot explanations distinguish “call all-in,” “short raise all-in,” and ordinary fixed-unit actions without leaking hidden information.

9. **Replay, traces, tests, simulation, benchmarks, documentation, WASM, and web closeout** as specified in §§4–10.

### 3.4 Out of scope

- Any rebuild of the Gate 15 base, evaluator, dealer/button system, existing single-pot split policy, shared seat frame, or shipped River Ledger presentation work except where the new data must be integrated.
- Multi-hand bankroll carryover, elimination, rebuy, add-on, tournament progression, blind escalation, table balancing, cash-out, rake, fees, currency, payouts, or economic value.
- Arbitrary user-selected bet sizes, no-limit, pot-limit, spread-limit, straddles, antes, bring-ins, insurance, run-it-twice, multiple boards, rabbit hunting, or additional poker variants.
- Hosted multiplayer, accounts, persistence, matchmaking, chat, ranked play, databases, or server authority.
- A general-purpose poker engine, generic betting DSL, YAML behavior, or a universal pot subsystem.
- Unrelated River Ledger polish, catalog redesign, animation redesign, evaluator expansion, or new game modes.
- Ticket decomposition and implementation. Those follow only after this spec is accepted.

### 3.5 Not allowed

- Real-money or casino product behavior, casino trade dress, tournament/product mimicry, or branded-room conventions.
- Hidden card, deck-tail, future-board, private evaluation, bot-ranking, or private replay leakage through JSON, DOM, accessibility text, test IDs, logs, storage, effects, diagnostics, explanations, or developer tools.
- TypeScript legality, action sizing, cap handling, all-in detection, pot construction, eligibility, winner selection, allocation, remainder ordering, or settlement.
- `engine-core` nouns such as seat, card, deck, hand, board, stack, all-in, pot, or side pot.
- `game-stdlib` promotion without the completed primitive-pressure decision and, where required, an accepted ADR.
- Static-data formulas, selectors, conditions, eligibility logic, allocation logic, or behavior-bearing YAML/DSL.
- Public MCTS, ISMCTS, Monte Carlo, ML, RL, omniscient bots, or unauthorized hidden-state access.
- Silent trace, hash, replay, serialization, rules-version, data-version, or visibility-taxonomy drift.
- Deleting, weakening, skipping, or rewriting a valid failing test merely to obtain green CI.

## 4. Deliverables

All behavior remains game-local unless §8's required atlas review changes the decision through the repository's accepted process.

| Repository seam | Required Gate 15.1 delta |
|---|---|
| `games/river_ledger/src/state.rs` | Add a checked game-local amount type or equivalent; per-seat starting/remaining stack; all-in/actionability state; per-seat reopen state; ordered resolved-pot and uncalled-return structures; aggregate final awards/stacks; stable deterministic ordering. Replace behavior-critical saturating arithmetic with checked validation. |
| `games/river_ledger/src/setup.rs` | Accept and validate ordered per-seat stacks; apply equal 24-unit default; cap blind posts by stack; create deterministic short-blind all-in states; reject invalid length, zero/overflow/out-of-range values, and malformed setup. |
| `games/river_ledger/src/variants.rs` and `games/river_ledger/data/{manifest.toml,variants.toml,fixtures/**}` | Add typed default/preset presentation data only; bump the River Ledger rules/data version as required; keep all conditional behavior in Rust; add equal and asymmetric fixtures. |
| `games/river_ledger/src/actions.rs` | Preserve the five action families while stack-capping exact amounts; emit authoritative metadata for `amount_owed`, `adds_to_pot`, `stack_before`, `stack_after`, `is_all_in`, `is_full_raise`, `raise_right_open`, `raises_remaining`, and accessible presentation. |
| `games/river_ledger/src/betting.rs` | Exclude folded/all-in seats from actor rotation; calculate owed amounts; track full-unit reopening per seat; distinguish response obligation from raise right; close streets and hands deterministically. |
| `games/river_ledger/src/rules.rs` | Apply checked stack deductions/contributions; transition to all-in; enforce cap/reopen rules; return unmatched excess; auto-run the board when no decisions remain; preserve foldout behavior. |
| `games/river_ledger/src/pot.rs` | Replace the single-pot-only helper with pure game-local contribution-layer construction, identical-eligibility coalescing, uncalled-return extraction, deterministic pot ids, and per-pot split/remainder allocation. Reuse the existing `games/river_ledger/src/pot.rs::winners_in_button_order(winners, button, seat_count)` button-order function or a behavior-equivalent tested helper. |
| `games/river_ledger/src/showdown.rs` | Evaluate each pot against its own eligibility set; support different winners and ties across pots; aggregate awards/final stacks; create one authoritative resolved result consumed by all projections. |
| `games/river_ledger/src/effects.rs` | Add ordered semantic effects equivalent to `StackChanged`, `SeatBecameAllIn`, `UncalledContributionReturned`, `PotResolved`, and `PotAwarded`; effects contain only viewer-safe public accounting and authorized reveal data. |
| `games/river_ledger/src/visibility.rs` | Project public stacks, all-in status, pot tiers, eligibility, returns, and per-pot awards while preserving private-hand/deck redaction for seat, cross-seat, and observer views. |
| `games/river_ledger/src/ui.rs` | Author neutral live/terminal pot labels, all-in indicators, per-pot explanation rows, uncalled-return text, button-order remainder text, and accessibility copy in Rust. |
| `games/river_ledger/src/bots.rs` | Consume only legal actions plus authorized public/seat data; handle short calls/raises and no-action all-in states; update deterministic policy evidence and explanations. |
| `games/river_ledger/src/replay_support.rs`, `lib.rs`, serialization surfaces | Include all behavior-critical stack, reopen, pot, return, and award state in stable hashes/serialization; add an explicit River Ledger v2 migration note; preserve global Trace Schema v1 unless an ADR authorizes otherwise. |
| `games/river_ledger/tests/**` | Add rule, unit, property, serialization, replay, bot, visibility, no-leak, WASM-export, and golden-trace evidence in §7; retain all valid Gate 15 tests. |
| `games/river_ledger/benches/river_ledger.rs` and `thresholds.json` | Benchmark maximum-layer construction, multi-pot allocation, all-in showdown, projection, replay, and 3–6-seat full-game paths under the accepted benchmark process. |
| `crates/wasm-api/src/games/river.rs` and related JSON/catalog/store/tests/snapshots | Marshal only Rust-projected setup and game fields; accept typed stack setup; carry all-in/pot/return/allocation views; add deterministic API surface tests and snapshots. |
| `apps/web/src/components/{MatchSetup,RiverLedgerBoard,RiverLedgerCard,OutcomeExplanationPanel,EffectLog}.tsx` and supporting client/state/style seams | Render stack setup, remaining stacks, all-in indicators, ordered pots, eligibility, returns, and per-pot outcome details. Do not compute legality or allocation. |
| `apps/web/e2e/river-ledger.smoke.mjs`, `a11y-noleak.smoke.mjs`, shared smoke scripts | Exercise an asymmetric all-in hand through terminal allocation; assert accessible labels, public pot accounting, no hidden-card leakage, and no duplicate/contradictory outcome computation. |
| `games/river_ledger/docs/**`, `apps/web/public/rules/river_ledger.md`, `specs/README.md`, `apps/web/README.md`, atlas/archival surfaces | Perform the enumerated documentation and closeout work in §10, but not as part of authoring this spec. |

Required public Rust projection shapes, by these names or behavior-equivalent names, are:

- `SeatStackView`: seat id/label, starting units, remaining units, committed units, and public status;
- `PotTierView`: stable pot id/label, amount, ordered eligible seats, and whether allocation is pending/resolved;
- `UncalledReturnView`: seat, amount, and Rust-authored reason;
- `PotAllocationView`: pot id, winners, shares, remainder, remainder order, and viewer-safe explanation;
- aggregate award/final-stack rows derived from the ordered per-pot result.

No new `engine-core` type is expected. No `game-stdlib` type is admitted by this spec.

## 5. Work breakdown

Each item below is a candidate bounded `templates/AGENT-TASK.md` packet. Every packet MUST name allowed paths, explicit exclusions, dependencies, acceptance evidence, and rollback/stop conditions. Implementation packets MUST follow `docs/AGENT-DISCIPLINE.md`: first determine whether a failing test remains valid, then whether the defect is in the system under test or the test, then fix the correct surface without weakening valid evidence.

| ID | Dependencies | Bounded work | Completion evidence |
|---|---|---|---|
| G15.1-W01 — Admission, rule contract, and version plan | Spec accepted | Update the implementation-admission plan; record the pure-delta boundary; define the new stable `RL-*` rule families; identify existing negative/single-pot rule rows that require explicit supersession; decide River Ledger rules/data v2 and replay-import policy before behavior changes. | Reviewed rule inventory and migration table; no code yet; stop and escalate if a foundation/ADR conflict appears. |
| G15.1-W02 — Primitive-pressure decision | W01 | Write the complete River Ledger pressure-ledger entry specified in §8. Default decision: keep contribution-layer construction and allocation game-local; no `game-stdlib` or `engine-core` change. | Every required ledger field populated; atlas debt register remains empty; any promotion proposal is separately reviewed before code. |
| G15.1-W03 — Typed stack setup | W01–W02 | Add configurable ordered starting stacks, equal 24-unit default, deterministic validation, asymmetric fixtures, and versioned setup/catalog metadata. No betting logic. | Setup/unit/fixture/WASM validation tests for 3–6 seats, malformed vectors, zero/out-of-range amounts, deterministic defaults, and public metadata. |
| G15.1-W04 — Stack ledger and forced posts | W03 | Add starting/remaining stack state, all-in status, checked conservation, and stack-capped blinds. Preserve existing contribution semantics and button/blind order. | Unit/property tests for conservation, short small/big blind, exact blind exhaustion, no underflow/overflow, and deterministic state summaries. |
| G15.1-W05 — Stack-capped legal actions | W04 | Make `Call`, `Bet`, and `Raise` amounts stack-aware while preserving action segments; add authoritative all-in/full-raise metadata and diagnostics. No street-closure changes yet. | Rule/action-tree/preview tests for short call, exact all-in call, short opening bet, short raise, full all-in raise, cap closure, and malformed/stale/wrong-seat diagnostics. |
| G15.1-W06 — Reopen rights and cap state | W05 | Add per-seat response and full-unit reopen accounting; separate “must respond” from “may raise”; implement cumulative incomplete-increase threshold; keep the existing cap. | Focused state-machine and property tests showing one short increase does not reopen, cumulative full-unit pressure does, full raises consume cap, and cap prevents any further raise. |
| G15.1-W07 — All-in actor rotation and runout | W06 | Skip folded/all-in seats, close streets correctly, return control only to action-capable seats, auto-run remaining board cards when no decision exists, and preserve no-reveal foldout. | 3–6-seat actor-order tests, runout traces, foldout tests, no infinite check/action loops, deterministic effects. |
| G15.1-W08 — Contribution-layer constructor | W04 | Implement the pure algorithm in §3.3: sorted caps, contributor/eligibility sets, singleton uncalled returns, identical-eligibility coalescing, stable ids, checked conservation. No hand evaluation. | Pure unit tests plus permutation/property tests; folded money retained; no singleton-contributor pot; deterministic ordering independent of map/container iteration. |
| G15.1-W09 — Per-pot allocation and settlement | W07–W08 | Evaluate each pot's eligible seats, split each pot independently, award each remainder independently in existing button order, aggregate awards, and produce final stacks. | Unit/rule/property tests for different winners, ties across pots, sole eligibility, odd units, uncalled excess, and total conservation. |
| G15.1-W10 — Effects, views, and explanations | W09 | Add ordered semantic effects and viewer-safe public projections; create Rust-authored live and terminal per-pot explanations; retain existing hand-comparison teaching detail where authorized. | Rust projection snapshots, outcome tests, effect-order tests, accessibility-copy review, and proof that every public allocation/return has one authoritative explanation. |
| G15.1-W11 — Replay, hash, and serialization migration | W10 | Add behavior-critical stack/reopen/pot state to stable summaries, hashes, serialization, and viewer-scoped export; record intentional River Ledger v2 drift; reject or explicitly convert v1 replays. | Replay equivalence, stable serialization/order, viewer hash tests, migration note, deterministic version-mismatch behavior, no global Trace Schema v1 change. |
| G15.1-W12 — Bots | W05–W11 | Update L0/L1/L2 policies for stack-capped legal actions and all-in terminality using only authorized views; update explanations/evidence. | Every bot always returns a legal action or no action when all-in/terminal; deterministic seed tests; hidden-state poisoning/no-leak tests; full 3–6-seat simulations. |
| G15.1-W13 — WASM bridge | W03, W10–W12 | Marshal stack setup and projected all-in/pot fields through `wasm-api`; update catalog/store/replay/API snapshots without duplicating rules. | Rust/WASM tests, API surface snapshot, malformed setup diagnostics, replay export/import, JSON ordering and redaction checks. |
| G15.1-W14 — Web renderer and smoke path | W13 | Add neutral setup controls, stack displays, all-in indicators, current pot tiers, eligibility, returns, and terminal allocations; render Rust-authored text only. | Web build, component tests where present, River Ledger end-to-end asymmetric hand, a11y/live-region checks, DOM/storage/log no-leak assertions. |
| G15.1-W15 — Integrated rule/property/serialization suite | W04–W14 | Extend the complete test matrix, including conservation, partition, eligibility, actor rotation, setup validation, deterministic ordering, and all existing regression tests. | `cargo test --workspace` green without weakened tests; rule-coverage rows point to exact tests; failure protocol recorded for any corrected test. |
| G15.1-W16 — Pairwise N-seat no-leak proof | W10–W14 | Run the Infra D matrix for N=3,4,5,6 across seat A→seat B, self, and observer; cover actions, previews, diagnostics, effects, bot rationale, replay, JSON, DOM, a11y, logs, storage, and dev surfaces. | Matrix artifact/checklist referenced from tests; intentional public stack/eligibility facts distinguished from prohibited card/deck/private evaluation facts. |
| G15.1-W17 — Golden traces | W07–W16 | Author the minimum trace set in §7, update affected shipped traces explicitly, and document every intentional River Ledger v2 drift. | Golden traces replay identically; expected hashes/versions recorded; no unexplained deletion or mass rewrite. |
| G15.1-W18 — Simulation and benchmarks | W12, W15–W17 | Exercise all seat counts and stack profiles; add maximum-layer and multi-pot hot paths; establish/report thresholds under accepted CI policy. | Simulator/replay/fixture/coverage commands pass; benchmark report contains version, command, environment, sample data, and threshold rationale. |
| G15.1-W19 — Documentation and closeout | W01–W18 | Perform every §10 update, complete public-release checklist, reconcile catalog/smoke docs, flip statuses, and archive only after all three exit rows pass. | Doc checks, official-game checklist, final command transcript, `specs/README.md` `Done`, archived spec with outcome, Gate 16 admitted. |

## 6. Exit criteria

These rows map one-for-one and in order to the Gate 15.1 exit list in `docs/ROADMAP.md`. No row may be merged away or satisfied by prose alone.

| ROADMAP §15.1 exit row | Gate 15.1 completion condition | Required evidence |
|---|---|---|
| 1. All-in contribution caps, side-pot construction, eligibility, remainders, split winners, and terminal explanations are covered by named rules, tests, golden traces, replay/hash checks, no-leak checks, and benchmarks. | Stable `RL-STACK-*`, `RL-ALLIN-*`, `RL-POT-*`, `RL-OUTCOME-*`, `RL-VIS-*`, `RL-BOT-*`, and `RL-REPLAY-*` rules map to game-local Rust implementation and complete evidence. The algorithm and every edge class in §§3 and 7 pass for 3–6 seats. | Rule-coverage matrix; unit/rule/property/serialization/replay/visibility/bot/WASM/web tests; minimum golden set; pairwise matrix; simulator; benchmark report; migration note. |
| 2. The outcome surface explains every public allocation without revealing private cards still hidden to the viewer. | Every main/side pot and uncalled return has a Rust-authored amount, eligibility, winner/share/remainder explanation. Foldout and sole-eligible awards reveal no hand content. Seat and observer projections contain only authorized showdown facts. | Outcome/projection snapshots; public-observer and cross-seat traces; browser a11y/DOM/storage/log checks; replay-export checks; manual neutral-copy review. |
| 3. Accounting stays typed Rust and not static-data behavior. | Stack validation, action caps, reopen rights, pot construction, eligibility, settlement, and explanation are all in `games/river_ledger` Rust. Data contains only bounded typed parameters/presentation. WASM transports projections; TypeScript renders them. | Boundary script; focused code review; rejected behavior-key fixture tests; no `engine-core` nouns; completed pressure ledger; WASM/web tests proving no duplicate legality/allocation. |

Gate status changes from `In progress` to `Done` only when all three rows have current evidence and the §10 closeout surfaces are reconciled.

## 7. Acceptance evidence

### 7.1 Required command suite

The capstone packet MUST record commands, environment, and outcomes. The minimum suite, applying the repository's `CLAUDE.md` workflow to River Ledger, is:

```bash
cargo fmt --all --check
cargo clippy --workspace --all-targets -- -D warnings
cargo build --workspace
cargo test --workspace
cargo test -p river_ledger
cargo test -p wasm-api
cargo run -p simulate -- --game river_ledger --games 1000
cargo run -p replay-check -- --game river_ledger --all
cargo run -p fixture-check -- --game river_ledger
cargo run -p rule-coverage -- --game river_ledger
bash scripts/boundary-check.sh
node scripts/check-doc-links.mjs
node scripts/check-catalog-docs.mjs
node scripts/check-player-rules.mjs
node scripts/check-outcome-explanations.mjs
npm --prefix apps/web ci
npm --prefix apps/web run smoke:wasm
npm --prefix apps/web run build
npm --prefix apps/web run smoke:ui
npm --prefix apps/web run smoke:effects
npm --prefix apps/web run smoke:e2e
cargo bench -p river_ledger
```

The simulator count above is the per-game smoke required by the repository command suite, not a replacement for the accepted random-playout budget/CI lanes. Any higher calibrated lane remains governed by accepted ADRs 0001–0003 and `docs/TESTING-REPLAY-BENCHMARKING.md`. Proposed ADR 0005 is not treated as binding authority.

### 7.2 Named rule families and migration

At minimum, the rules and coverage documents MUST define and trace these stable families:

| Rule ID | Required meaning |
|---|---|
| `RL-STACK-SETUP-001` | Starting stacks are an ordered, validated, public N-seat setup input with equal 24-unit default. |
| `RL-STACK-CONSERVE-001` | Remaining stacks, commitments, returns, and awards conserve the initial total under checked arithmetic. |
| `RL-ALLIN-CAP-001` | No seat contributes more than its remaining stack; blinds and actions are capped. |
| `RL-ALLIN-CALL-001` | A seat unable to cover the call may fold or call all-in for its entire remaining stack. |
| `RL-ALLIN-BET-001` | A legal short/full bet or raise may consume the actor's stack and mark it all-in. |
| `RL-ALLIN-ACT-001` | Folded/all-in seats do not act; action-capable seats respond deterministically. |
| `RL-ALLIN-REOPEN-001` | Incomplete all-in increases do not individually reopen; cumulative pressure reopens only at a full street unit; full raises consume the cap. |
| `RL-ALLIN-RUNOUT-001` | The board runs out automatically when no legal betting decision remains and at least two non-folded seats remain. |
| `RL-POT-LAYER-001` | Ascending contribution caps create ordered, coalesced contestable pots. |
| `RL-POT-ELIG-001` | Folded contributions count in amounts but folded seats are never eligible. |
| `RL-POT-RETURN-001` | A singleton unmatched top layer is returned before allocation. |
| `RL-POT-ALLOC-001` | Every pot is evaluated and allocated independently among its eligible seats. |
| `RL-POT-REMAINDER-001` | Every pot's remainder is awarded independently in existing button order. |
| `RL-OUTCOME-POT-001` | Rust explains every public pot award and return. |
| `RL-VIS-POT-001` | Public stacks/eligibility are projected without leaking private cards or future deck state. |
| `RL-BOT-ALLIN-001` | Bots use only authorized views and stack-aware legal actions. |
| `RL-REPLAY-STACK-001` | Stack/reopen/pot state is deterministic, hashed, serialized, replayed, and viewer-filtered. |

Existing unaffected rule IDs remain stable. Rows that currently assert single-pot-only or all-in-out-of-scope behavior—including `RL-POT-ALLIN-001`, `RL-POT-SINGLE-*`, `RL-OOS-ALLIN-001`, and `RL-VAR-ALLIN-001` where present—MUST receive an explicit supersession/migration entry rather than silent deletion or semantic reuse.

### 7.3 Unit, rule, property, replay, and serialization classes

Required test classes include:

- setup vector length/order/default, invalid values, checked maxima, and 3–6-seat fixtures;
- blind posting with full, exact-exhaustion, and short stacks;
- legal action set and metadata for every stack-to-call relationship;
- per-seat response obligation versus raise right, including cumulative incomplete increases and cap behavior;
- actor selection and street completion with any mix of live, folded, and all-in seats;
- pure pot-layer construction under contribution permutations;
- identical-eligibility coalescing and no artificial remainder boundaries;
- folded-but-contributed money, sole eligibility, and singleton uncalled return;
- independent main/side-pot winner evaluation, ties, shares, and odd units;
- action-by-action and terminal conservation;
- deterministic pot/seat/share/effect/serialization order independent of container iteration;
- unchanged evaluator behavior and all existing Gate 15 regression cases;
- replay from seed and commands; import/export version handling; stable viewer hashes;
- bots always legal and deterministic; all-in/terminal seats produce no action;
- WASM setup parsing, projected JSON, API surface snapshots, and diagnostics;
- browser render/a11y/effect/no-leak behavior.

Property generators SHOULD cover bounded 3–6-seat contribution vectors directly as pure accounting inputs, but generated vectors MUST be filtered or classified by validity. At minimum, the properties assert:

```text
0 <= contribution[seat] <= starting_stack[seat]
remaining[seat] + contribution[seat] == starting_stack[seat] before settlement
sum(pots) + sum(returns) == sum(contributions)
sum(shares for pot) == pot.amount for every pot
winner_set(pot) is a non-empty subset of eligible_set(pot)
folded seats never appear in eligible_set
final pot list has no singleton contributor layer
sum(final_stacks) == sum(starting_stacks)
same canonical input => same ordered pots, returns, shares, effects, serialization, and hash
```

### 7.4 Minimum golden-trace set

The gate MUST add at least the following 22 named scenarios, using these filenames or equally explicit stable names:

1. `setup-equal-default-stacks-3p.trace.json`
2. `setup-asymmetric-stacks-6p.trace.json`
3. `small-blind-short-all-in.trace.json`
4. `call-all-in-below-price.trace.json`
5. `exact-call-exhausts-stack.trace.json`
6. `short-open-bet-all-in.trace.json`
7. `short-raise-all-in-no-reopen.trace.json`
8. `cumulative-short-raises-reopen-at-full-unit.trace.json`
9. `full-all-in-raise-reopens-and-counts-cap.trace.json`
10. `raise-cap-blocks-short-raise.trace.json`
11. `three-way-main-and-two-side-pots.trace.json`
12. `folded-contribution-retained-in-pots.trace.json`
13. `uncalled-top-excess-return.trace.json`
14. `different-main-and-side-pot-winners.trace.json`
15. `main-pot-tie-side-pot-sole-winner.trace.json`
16. `ties-across-multiple-pots.trace.json`
17. `per-pot-remainder-button-order.trace.json`
18. `all-remaining-all-in-auto-runout.trace.json`
19. `sole-live-foldout-no-showdown-leak.trace.json`
20. `public-observer-multi-pot-no-leak.trace.json`
21. `seat-private-replay-multi-pot-no-leak.trace.json`
22. `wasm-exported-side-pot-terminal.trace.json`

The trace set MUST include at least one 3-seat and one 6-seat hand, one hand with three different live contribution caps, one folded contributor, one returned excess, one pot with a sole eligible seat, one pot tie, different winners across pots, and an odd unit in more than one final pot. Existing Gate 15 traces remain unless an explicit River Ledger v2 migration note identifies why their public or internal state changed.

### 7.5 Pairwise N-seat no-leak matrix

For each `N ∈ {3,4,5,6}`, the acceptance harness covers:

- every source seat A projected to every different viewer seat B;
- each seat's self-view;
- the public observer;
- pre-action, post-action, all-in waiting, automatic runout, showdown, foldout, and replay-import states.

Surfaces:

| Surface | Intentionally public | Prohibited |
|---|---|---|
| Setup and live state | Starting/remaining stacks, contributions, all-in/folded status, public pot totals/tiers, public eligibility | Other seats' hole cards, deck tail, unrevealed board, internal evaluator data |
| Legal actions / previews | Exact legal amount, owed amount, stack after action, all-in/full-raise/reopen/cap metadata | Unavailable hidden-state branches, private opponent reasoning |
| Diagnostics / effects | Public accounting changes and neutral reasons | Hidden card identifiers, future deal order, private candidate values |
| Bots | Own authorized hand, public board/accounting, legal actions | Opponent hands, deck order, omniscient rankings, forbidden rollout data |
| Replay / hashes / JSON | Viewer-scoped public state and authorized self/private state | Internal full trace through a public export, terminal auto-reveal of unused private cards |
| Browser DOM / a11y / logs / storage / test ids / dev tools | The same viewer-authorized projection | Any serialized or text-hidden private/deck/evaluator payload |

Public stack and eligibility data are not treated as leaks. The harness must nevertheless prove that adding those fields does not accidentally carry adjacent private structures.

### 7.6 Replay, hash, and version evidence

- Global `docs/TRACE-SCHEMA-v1.md` remains unchanged.
- Gate 15.1 intentionally introduces behavior-critical state, so River Ledger's rules/data version MUST advance to v2 or the repository's next valid per-game version.
- All stack, reopen, ordered pot, return, award, and final-stack state that can affect future behavior or outcomes MUST participate in deterministic serialization/hash state.
- A migration note MUST list affected golden traces/snapshots and distinguish expected per-game drift from accidental global drift.
- Existing v1 River Ledger replay imports MUST either:
  1. fail with a stable deterministic version-mismatch diagnostic; or
  2. pass through an explicit, tested converter that supplies the exact historical semantics.

  Silent interpretation of a v1 replay under v2 stack rules is forbidden.
- Public replay export remains viewer-scoped under accepted ADR 0004. Terminal settlement does not authorize disclosure of cards that the viewer is not otherwise entitled to see.
- If implementation requires a global replay envelope, hash algorithm, Trace Schema, export-taxonomy, visibility-policy, or kernel-boundary change, work stops and an accepted ADR is required before proceeding.

### 7.7 Benchmark evidence

Native Rust is the primary performance lane. The benchmark plan covers every official seat count and both equal/default and asymmetric profiles. Required named hot paths, by these names or equivalents:

- `setup_3p_equal_stacks`
- `setup_6p_asymmetric_stacks`
- `legal_actions_short_stack`
- `apply_short_all_in_raise`
- `construct_side_pots_6p_max_layers`
- `allocate_side_pots_6p_split_winners`
- `resolve_all_in_showdown_6p`
- `project_view_6p_multi_pot`
- `serialize_replay_6p_multi_pot`
- `bot_policy_6p_short_stack`
- `full_game_6p_all_in_pressure`

The largest deterministic fixture MUST exercise six distinct positive contribution caps, folded money, at least three contestable final pots, a returned top layer, and a split pot. Reports include command, build/profile, CPU/environment, sample count, observed distribution, prior baseline where available, proposed threshold, and variance rationale. Thresholds are not invented in this spec; they are calibrated under accepted ADRs 0002–0003 and current testing law. WASM/browser timing is smoke evidence, not the primary threshold lane.

### 7.8 External research basis

External evidence sharpens the game-local rule, but does not by itself establish repository state.

- [Poker TDA Rules](https://www.pokertda.com/view-poker-tda-rules/), especially Rules 47–48, distinguishes incomplete all-in increases, cumulative reopening, and fixed-limit cap handling. It also exposes the fixed-limit half-bet convention that differs from this spec's deliberately documented full-unit River Ledger rule.
- [2025 WSOP Tournament Rules](https://www.wsop.com/2025/2025-WSOP-Tournament-Rules.pdf), especially Rules 73–74 and 95–96, supports awarding odd chips in button order, splitting each side pot separately, returning uncalled excess, and treating incomplete all-ins distinctly from full raises.
- [PokerKit 0.4 simulation documentation](https://pokerkit.readthedocs.io/en/0.4/simulation.html) models pots as amounts plus entitled players and resolves/pushes pots separately.
- [PokerKit 0.4 tagged source](https://raw.githubusercontent.com/uoftcprg/pokerkit/0.4/pokerkit/state.py) provides independent implementation prior art for ascending contribution-level construction and coalescing adjacent pots with the same eligibility set.
- The [PokerKit paper](https://arxiv.org/abs/2308.07327) supports using a deterministic, replayable engine as a validation reference rather than copying a product UI or trade dress.
- The [Poker Hand History specification paper](https://arxiv.org/abs/2312.11753) is additional prior art for recording starting stacks and action/accounting details explicitly in durable hand records.

No external rule set is copied wholesale. River Ledger uses original neutral wording, its existing button order, and the repository's own fixed-limit/cap model.

## 8. FOUNDATIONS & boundary alignment

### 8.1 Engaged principles

| Authority | Gate stance |
|---|---|
| `docs/FOUNDATIONS.md` §11 universal invariants | Rust owns behavior; deterministic replay/hash; no leaks; bots are fair; static data is not behavior; public/IP-safe; tests and official-game evidence are mandatory. |
| `FOUNDATIONS` §12 stop conditions | Stop for any required `engine-core` noun, behavior-bearing data, TypeScript authority, global replay/visibility change, unexplained nondeterminism, hidden-info leak, forbidden bot method, or need to weaken a valid test. |
| `FOUNDATIONS` §13 ADR triggers | A game-local rules/data version and trace refresh are planned. A global schema/hash/export taxonomy, kernel boundary, shared primitive, or data-policy change requires an accepted ADR first. |
| `docs/ARCHITECTURE.md` | All-in/stack/pot nouns and behavior live in `games/river_ledger`; `wasm-api` marshals projections; `apps/web` presents them. |
| `docs/ENGINE-GAME-DATA-BOUNDARY.md` | Contribution layering, eligibility, reopen rights, and allocation are typed game behavior. Data may hold bounded defaults/presets only. |
| `docs/MULTI-SEAT-AND-SURFACE-CONTRACT.md` | Ordered setup, seat-keyed public stacks, 3–6-seat projection, pairwise A→B no-leak, observer safety, and public outcome explanation all apply. |
| `docs/OFFICIAL-GAME-CONTRACT.md` | Every per-game rule/doc/test/bot/benchmark/WASM/UI/release artifact is updated and reconciled before closeout. |
| `docs/AI-BOTS.md` | Bots use legal actions and authorized views only; no public MCTS/ISMCTS/Monte Carlo/ML/RL. |
| `docs/TESTING-REPLAY-BENCHMARKING.md` | Named rule, golden, property, replay, serialization, no-leak, bot, UI, simulation, and benchmark evidence are required. |
| `docs/UI-INTERACTION.md` | Rust supplies legality and explanations; UI shows exact stack/pot/eligibility consequences, accessible states, and no hidden information. |
| `docs/WASM-CLIENT-BOUNDARY.md` | The bridge is a thin viewer-scoped JSON transport, not a second rules engine. |
| `docs/IP-POLICY.md` | Use neutral contribution/ledger/pot language, original copy, and no casino-room trade dress or money framing. |
| Accepted ADR 0004 | Preserve internal-full versus viewer-scoped export taxonomy; no terminal auto-reveal of unauthorized cards. |
| Accepted ADRs 0001–0003 | Preserve random-playout and calibrated CI benchmark disciplines. |
| Accepted ADR 0007 | Gate 15.1 is inside the admitted public scaling phase; this spec does not amend its scope or Gate P tail. |

### 8.2 Boundary decisions

- **`engine-core`: no change expected.** Generic command/replay/view contracts already suffice. Pot, stack, seat, card, and all-in nouns are forbidden there.
- **`game-stdlib`: no promotion planned.** Only River Ledger exerts the full layered-eligibility/allocation shape. Poker Lite's single-pot split is not the same abstraction pressure.
- **Static data:** default stack values and named neutral presets are allowed typed parameters. Any formula, branch, reopen threshold logic, eligibility rule, allocation rule, or winner rule in TOML/JSON/YAML is forbidden.
- **WASM/browser:** setup values cross into Rust for validation; only projected results return. TypeScript does not reconstruct caps, pot tiers, eligibility, winners, or remainders.
- **Trace/hash:** behavior-critical state must be hashed even when that changes River Ledger v1 traces. The change is explicit, per-game, versioned, and reviewable.
- **Visibility:** stacks, contributions, all-in state, pot amounts, and eligibility are public; cards/deck/evaluator internals remain governed by existing viewer authority.
- **Effects:** semantic effects are ordered public accounting facts. They never become a covert hidden-state log.

### 8.3 Required stop/escalation conditions

The implementing series stops before further behavior work if any of the following becomes necessary:

1. adding a poker/card/pot/stack noun to `engine-core`;
2. promoting a shared helper without the completed atlas decision and required prior-game conformance plan;
3. changing global Trace Schema v1, global hash algorithm, replay envelope, or hidden-info export taxonomy;
4. placing legality or allocation behavior in web code or static data;
5. exposing a viewer-unauthorized card, deck, evaluator, bot-ranking, or replay field;
6. using saturation to mask a behavior-critical overflow;
7. changing the fixed-limit structure, raise cap, or odd-unit order outside the explicit Gate 15.1 delta;
8. deleting or weakening a valid test to accept the implementation.

A genuine global trigger is routed to an ADR using `docs/adr/ADR-TEMPLATE.md`. No ADR is presumed necessary for the planned game-local implementation.

### 8.4 Primitive-pressure ledger entry required by this gate

The implementation MUST write an entry with the complete field set below. These are the planned contents; reviewers may tighten wording without changing the decision silently.

| Required field | Planned entry |
|---|---|
| Mechanic shape | Deterministic layered contribution-cap construction, folded-money inclusion, per-pot eligibility, uncalled-return extraction, independent split/remainder allocation, and viewer-safe allocation explanation. |
| Status | Evaluated at Gate 15.1; promotion rejected/deferred; keep game-local. |
| Games exerting pressure | River Ledger Gate 15.1. Poker Lite and River Ledger Gate 15 have single-pot split/remainder behavior, but not repeated nested eligibility layers. |
| Relevant files/docs | `games/river_ledger/src/{state,actions,betting,rules,pot,showdown,effects,visibility,ui,bots}.rs`; tests/bench/docs; ROADMAP §15.1; this spec. |
| What is repeated | Integer pot splitting and deterministic winner-order/remainder concepts already exist locally. |
| What differs | Side-pot construction depends on River Ledger contribution caps, fold/all-in statuses, fixed-limit action history, button order, showdown evaluator, and public explanation model. |
| Why local duplication is now risky or acceptable | Acceptable: there is one full implementation and no third repeated behavior shape. Premature extraction would freeze poker nouns or an under-proven eligibility API. |
| Decision: reuse / promote / defer-reject / ADR | Reuse River Ledger's local `winners_in_button_order` helper; implement the new layer constructor/allocation locally; defer/reject `game-stdlib` promotion; no ADR. |
| Why not engine-core | The mechanic contains game nouns and policy—seat, contribution, fold, all-in, pot, eligibility, button, showdown—prohibited by the noun-free kernel. |
| Why game-stdlib is or is not appropriate | Not appropriate yet: no narrow behavior-free cross-game contract has repeated. A future third use must re-open pressure with concrete conformance cases. |
| Data/Rust boundary impact | Stack defaults/presets may be typed data; all validation, caps, layers, eligibility, returns, and allocation remain Rust. |
| Replay/hash impact | Intentional River Ledger v2 state/hash/trace change; stable ordered pots/returns required; no global trace schema change. |
| Visibility impact | Pot amounts/eligibility/stacks public; private cards and evaluator details remain viewer-scoped; pairwise matrix required. |
| Bot impact | Legal-action and policy inputs gain public stack/pot fields; no hidden-state expansion; no forbidden search methods. |
| UI/effect impact | Rust-authored all-in, return, pot-tier, award, and remainder effects/explanations; TS presentation only. |
| Tests required | Pure algorithm unit/property tests; rule/state tests; replay/serialization; 3–6 pairwise no-leak; bots; WASM; UI/e2e; golden set in §7. |
| Benchmarks required | Six-seat maximum-layer construction, multi-pot split allocation, all-in showdown, projection, serialization/replay, bots, and full game. |
| Back-port/conformance plan | Not applicable: no shared promotion. Preserve Poker Lite and prior River Ledger behavior; no prior-game migration. |
| Affected prior games | None behaviorally. Poker Lite may be cited as comparison evidence only. |
| Exceptions, if any | Existing River Ledger button-order helper remains game-local and is reused. |
| Closure gate if debt is deferred | Not applicable. The decision closes with no promotion debt; §10A remains empty. |
| Examples | A pure `games/river_ledger/src/pot.rs` function from ordered contributions/statuses to ordered pots/returns; per-pot allocator using existing button order. |
| Anti-examples | `engine_core::SidePot`; a universal `game-stdlib` poker pot; TOML formulas for eligibility; TypeScript constructing pots or winners. |
| Agent misuse risks | Treating public eligibility as card-reveal authority; using hash maps without canonical ordering; creating one pot per folded cap without coalescing; saturation; duplicating allocator logic in UI/WASM. |
| Review owner/date | Rulepath maintainers / Gate 15.1 acceptance and closeout dates. |

If implementation research reveals a genuinely repeated, narrow, behavior-free helper, W02 must be revised and accepted before extraction. That possibility does not authorize speculative promotion during another packet.

## 9. Forbidden changes

In addition to §3.5, no Gate 15.1 packet may:

- replace or redesign the shipped evaluator, deck, hand-ranking ladder, seat labels, button order, or base renderer without a demonstrated all-in/side-pot dependency;
- introduce a sixth action family named “all in” that permits arbitrary sizing; all-in remains a stack-qualified fixed-limit call/bet/raise;
- change the standard small blind, big blind, small-bet unit, big-bet unit, or raise-cap count except through a separately accepted scope change;
- treat setup stack units as money, permit decimals, display currency symbols, or add buying/selling language;
- award unmatched excess as a pot win or include it in winner/tie evaluation;
- discard folded contributions from pot amounts or permit folded seats to win;
- combine independently eligible pots before splitting, or distribute one global remainder across all pots;
- award odd units according to vector/hash-map order rather than the existing button order;
- let an all-in seat remain in `actors_to_respond` or receive a no-op action loop;
- derive final stacks from UI-side arithmetic or from multiple competing terminal-result assemblies;
- expose all private showdown cards merely because the hand reached all-in runout;
- silently accept v1 replays under v2 behavior;
- generalize a poker pot abstraction into `game-stdlib` merely because single-pot splitting exists in another game;
- create, edit, or archive ticket files as part of this spec-authoring deliverable;
- perform the documentation edits enumerated in §10 while authoring this file.

## 10. Documentation updates required

These are future Gate 15.1 work items. This spec enumerates them and does not perform them.

| Target | Required delta |
|---|---|
| `games/river_ledger/docs/GAME-IMPLEMENTATION-ADMISSION.md` | Supersede the base exclusion for all-in/side pots; record option (b), equal default, game-local boundary, rules/data version, research divergence, and admission evidence. |
| `games/river_ledger/docs/RULES.md` | Add the stable `RL-STACK-*`, `RL-ALLIN-*`, `RL-POT-*`, outcome/visibility/bot/replay rules; include exact fixed-unit reopen, runout, layer, return, split, and odd-unit semantics; provide old-rule migration table. |
| `games/river_ledger/docs/RULE-COVERAGE.md` | Map every new/changed rule to Rust modules, unit/rule/property tests, golden traces, replay/serialization, no-leak, bots, UI/WASM, simulation, and benchmark evidence. |
| `games/river_ledger/docs/MECHANICS.md` | Extend the resource/accounting row with public stacks, all-in caps, layered pots, eligibility, returns, and atlas pressure; distinguish shipped base from delta. |
| `games/river_ledger/docs/HOW-TO-PLAY.md` | Explain neutral contribution units, equal/default and configured stacks, call all-in, short raise behavior, who can win each pot, uncalled returns, separate side-pot awards, and odd-unit order in original player-facing prose. |
| `games/river_ledger/docs/UI.md` | Specify setup controls, seat stack/all-in states, current ordered pot breakdown, eligibility, returns, terminal per-pot explanations, accessibility/live-region behavior, and Rust/TS ownership. |
| `games/river_ledger/docs/AI.md` | Update bot registry/status, authorized inputs, legal short-stack actions, all-in waiting, determinism, and explicit exclusion of omniscient/rollout/ML methods. |
| `games/river_ledger/docs/COMPETENT-PLAYER.md` | Add stack-aware fixed-limit decisions, main-versus-side-pot incentives, call price/effective cap, folded money, reopen/cap awareness, and uncertainty-safe heuristics. |
| `games/river_ledger/docs/BOT-STRATEGY-EVIDENCE-PACK.md` | Turn competent-player findings into bounded L2 features/tie-breaks/explanations; document information budget and adversarial no-leak cases. |
| `games/river_ledger/docs/BENCHMARKS.md` | Add maximum-layer construction, allocation, projection, replay, bot, and six-seat full-hand cases; record calibrated thresholds and environment. |
| `games/river_ledger/docs/SOURCES.md` | Cite the external sources in §7.8, identify the River Ledger-specific divergence, record original wording/no trade dress, and distinguish source use from copied behavior/UI. |
| `games/river_ledger/docs/PRIMITIVE-PRESSURE-LEDGER.md` | Write the complete §8.4 entry and closure decision; no silent omission. |
| `games/river_ledger/docs/PUBLIC-RELEASE-CHECKLIST.md` | Re-run all universal, N-seat, no-leak, rules, UI, bot, benchmark, source/IP, and browser closeout rows for the v2 delta. |
| `apps/web/public/rules/river_ledger.md` | Publish the updated player rules generated/copied through the repository's approved rules pipeline; include all-in/side-pot explanation without casino framing. |
| `apps/web/public/rules/manifest.json` | Reconcile the public rules artifact/version/hash as required by existing scripts. |
| `docs/MECHANIC-ATLAS.md` | Record the Gate 15.1 pressure decision and confirm the open-debt register remains empty. |
| `specs/README.md` | On spec acceptance, replace the unwritten seed with this path and change `Not started` → `Planned`. At execution start, change to `In progress`. Only after §6 passes, change to `Done` and admit Gate 16. |
| `apps/web/README.md` | At closeout, reconcile the intro catalog list, Shell Surface renderer list, and Smoke Layers `smoke:e2e` list as required by `specs/README.md` and `OFFICIAL-GAME-CONTRACT` §§10/12. |
| `archive/specs/` and this spec's Outcome/closeout record | When the gate is `Done`, follow `docs/archival-workflow.md`: record outcome/evidence, archive the completed spec, and leave the living index pointing to the archived path. |
| Future `tickets/` | After acceptance only, decompose §5 using `templates/AGENT-TASK.md`. This deliverable creates no tickets. |

`templates/README.md` governs universal completion, and `templates/AGENT-TASK.md` governs later packets; neither template is copied into a game document or edited merely for this gate.

## 11. Sequencing

1. **Predecessor state.** Infra A–D and Gate 15 are `Done`; the subsequent River Ledger presentation/correctness specs are also `Done`. Gate 15.1 consumes those shipped surfaces rather than superseding them.
2. **Spec admission.** Acceptance of this document changes the Gate 15.1 tracker row from `Not started` to `Planned`; it does not start implementation automatically.
3. **Before behavior code.** Execute W01 and W02 first. The rule/version contract and full primitive-pressure decision must exist before any shared extraction or trace migration.
4. **Core dependency chain.** W03–W07 establish stacks and legal betting state; W08 is a pure accounting branch that can proceed after stack types; W09 joins betting termination to allocation; W10–W14 project the one authoritative result through Rust, bots, WASM, and web.
5. **Evidence chain.** Local tests accompany every behavior packet. W15–W18 integrate the full property, no-leak, golden, simulation, replay, and benchmark proof.
6. **Closeout.** W19 runs only after the three §6 rows pass. It reconciles every per-game/public/index surface, records commands and reviews, changes Gate 15.1 to `Done`, and archives the completed spec.
7. **Successor admission.** Gate 16 Hearts remains `Not started` until Gate 15.1 is `Done` and the atlas open-debt register is empty. Gate 15.1 does not pre-design or promote trick-taking behavior.
8. **No hidden parallel scope.** Unrelated River Ledger polish or a generic poker framework must be split into separately admitted work, not smuggled into these packets.

## 12. Assumptions

Each line is independently correctable without re-opening the locked gate determination.

- `assumption:` Gate 15.1 is the next unit because the in-repo tracker, atlas, ROADMAP, predecessor, and ADR evidence agree.
- `assumption:` The shipped Gate 15 hand remains a single-hand match; stacks reset at new setup and do not carry between matches.
- `assumption:` Stack model option (b) is selected: configurable ordered per-seat stacks with an equal 24-unit default.
- `assumption:` Stack and contribution units are non-monetary positive integers; one unit is the smallest indivisible accounting unit.
- `assumption:` Existing fixed-limit street units and the existing capped-raise structure remain unchanged.
- `assumption:` An incomplete all-in increase below one full street unit does not individually reopen raising; cumulative incomplete increases reopen only upon reaching one full unit since the seat's last action.
- `assumption:` The existing button-ordered remainder rule remains the odd-unit rule and applies independently to every final pot.
- `assumption:` Starting/remaining stacks, committed contributions, all-in state, pot amounts, and pot eligibility are public; private cards/deck/evaluator internals retain existing viewer authority.
- `assumption:` The contribution-layer and allocation implementation stays in `games/river_ledger`; no `game-stdlib` promotion or `engine-core` change is warranted.
- `assumption:` River Ledger advances to an explicit per-game v2 rules/data/hash baseline; global Trace Schema v1 and accepted replay/export taxonomy remain unchanged.
- `assumption:` Existing v1 replay data is rejected deterministically unless a bounded explicit converter is separately justified and tested.
- `assumption:` No ADR is needed for the planned game-local delta; any global replay/hash/visibility/data-policy/kernel/shared-primitive need triggers the stop route in §8.
- `assumption:` External poker authorities are validation prior art, not repository law; where their fixed-limit reopening convention differs, the River Ledger rule is stated openly rather than misattributed.
- `assumption:` No real-money, tournament, casino-room, or product-mimicry feature is implied by finite stacks or side pots.

## Outcome

Completed 2026-06-20. Gate 15.1 shipped the River Ledger v2 finite-stack,
all-in, and side-pot delta in dependency order through archived tickets
`GAT151RIVLED-001` through `GAT151RIVLED-020`.

Delivered scope:

- typed configurable starting stacks with equal defaults and asymmetric
  acceptance profiles;
- stack-capped fixed-limit calls, bets, raises, all-in qualification, full-unit
  reopening state, all-in actor skipping, and cap interaction;
- contribution-layer construction, folded-money retention, uncalled returns,
  per-pot eligibility, independent split/remainder allocation, and one
  authoritative terminal allocation assembly;
- viewer-safe Rust projections, WASM bridge output, browser renderer, effect
  feedback, public replay/export, no-leak proof, bots, rule coverage, golden
  traces, simulation, benchmarks, public player rules, and release checklist
  reconciliation;
- mechanic-atlas decision: the side-pot/all-in shape remains local to
  `games/river_ledger`; no `game-stdlib` helper, `engine-core` noun, global
  replay/hash migration, or open promotion debt was introduced.

Status reconciliation:

- `specs/README.md` Order 6 / Gate 15.1 now points to this archived spec and
  reads `Done`.
- `docs/MECHANIC-ATLAS.md` §10A still reads `Current debt: _None_`.
- Gate 16 Hearts is admitted as the next not-started public scaling unit.

Verification:

- `cargo fmt --all --check` — passed.
- `cargo clippy --workspace --all-targets -- -D warnings` — passed after the
  mechanical iterator rewrite in River Ledger setup state construction.
- `cargo build --workspace` — passed.
- `cargo test --workspace` — passed.
- `cargo test -p river_ledger` — passed.
- `cargo test -p wasm-api` — passed.
- `cargo run -p simulate -- --game river_ledger --games 1000` — passed:
  1000 games, cycled 3-6 seats, cycled stack profiles, average length 15.96,
  throughput 935.52 games/sec.
- `cargo run -p replay-check -- --game river_ledger --all` — passed
  (`replay-check: all traces passed`).
- `cargo run -p fixture-check -- --game river_ledger` — passed
  (`fixture-check: all fixtures passed`).
- `cargo run -p rule-coverage -- --game river_ledger` — passed
  (`rule-coverage: river_ledger coverage matrix passed`).
- `bash scripts/boundary-check.sh` — passed
  (`engine-core boundary check passed`).
- `node scripts/check-doc-links.mjs` — passed (`Checked 27 markdown files`).
- `node scripts/check-catalog-docs.mjs` — passed
  (`catalog-docs check passed — 15 games reflected in intro, root, and smoke surfaces`).
- `node scripts/check-player-rules.mjs` — passed
  (`player-rules check passed — 15 catalog games validated`).
- `node scripts/check-outcome-explanations.mjs` — passed
  (`outcome-explanations check passed — 15 catalog games validated`).
- `npm --prefix apps/web ci` — passed with the pre-existing npm audit notice of
  one low-severity vulnerability.
- `npm --prefix apps/web run smoke:wasm` — passed.
- `npm --prefix apps/web run build` — passed.
- `npm --prefix apps/web run smoke:ui` — passed.
- `npm --prefix apps/web run smoke:effects` — passed.
- `npm --prefix apps/web run smoke:e2e` — passed, including
  `{"browser":"puppeteer","smoke":"river_ledger noleak legal controls terminal responsive"}`.
- `cargo bench -p river_ledger` — passed and emitted the
  `BEGIN_RIVER_LEDGER_BENCHMARK_JSON` / `END_RIVER_LEDGER_BENCHMARK_JSON`
  block.

Deviations:

- None from Gate 15.1 acceptance. The required Clippy lane exposed a
  `needless_range_loop` warning in `games/river_ledger/src/state.rs`; closeout
  replaced it with an equivalent iterator/enumerate loop so the full
  workspace hygiene command could pass.

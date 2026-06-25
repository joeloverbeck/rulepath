# Gate 18 implementation spec — **Blackglass Pact** / classic partnership Spades trick-taking

## 1. Header

| Field | Value |
| --- | --- |
| Spec ID | `GAT18-BLAPAC-PARTRI-001` |
| File | `specs/gate-18-blackglass-pact-spades-partnership-trick-taking.md` |
| Roadmap stage | Stage 18 / Public scaling phase |
| Roadmap build gate | Gate 18 |
| Status | `Planned` / `Not started` |
| Date | 2026-06-25 |
| Owner | Rulepath maintainers / implementation agents |
| Primary crate | `games/blackglass_pact` |
| Internal game id | `blackglass_pact` |
| Public display name | **Blackglass Pact** |
| Rules-family label | Classic four-player partnership Spades with individual nil, blind nil, and cumulative bags |
| Standard variant id | `blackglass_pact_standard` |
| Trace rules version | `blackglass-pact-rules-v1` |
| Data/manifest version | `blackglass-pact-data-v1` |
| Browser implementation required | Yes — Rust/WASM-backed fixed-four renderer, grouped partnership table, public observer, four seat-private viewers, viewer-scoped replay exports, Rust-authored team outcome explanation, and dedicated e2e smoke are gate requirements |
| Official seat declaration | Minimum `4`; maximum `4`; default `4`; supported set `{4}`; stable trace ids `seat_0` through `seat_3`; fallback public labels `North`, `East`, `South`, `West`; clockwise order is `seat_0 → seat_1 → seat_2 → seat_3 → seat_0`; setup rejects every other count with a stable Rust diagnostic |
| Partnership declaration | **Fixed** and public from setup. Stable `team_0` is North–South (`seat_0`, `seat_2`); stable `team_1` is East–West (`seat_1`, `seat_3`). Team membership does not replace seat identity and does not authorize partner-hand visibility |
| Public observer | Required; observer receives all public bids, partnership identities, played cards, trick results, scores, bags, and outcomes, but never any unplayed hand, future deal identity, private action tree, private preview, or private bot payload |
| Team-private viewer | `not applicable` for `blackglass_pact_standard`: the locked game has no hidden fact lawfully shared with a partner but withheld from opponents. Public team facts are visible to every viewer; each seat-private view adds only that seat's own hand and own controls |
| Bot floor | L0 random-legal required. A bounded L1 rule-informed partnership-aware bidding/nil/play policy is required for serious-demo quality. L2 is not admitted until competent-player and strategy-evidence gates are accepted. L3 is `not applicable` because deterministic search is perfect-information-only and Blackglass Pact is hidden-information |
| Trace schema | Existing Trace Schema v1; no schema migration authorized |
| Fixture/export taxonomy | Existing Evidence Fixture Contract plus ADR 0004 and ADR 0009 taxonomies; no blanket golden regeneration and no hash-taxonomy migration authorized |
| Authority order | `docs/README.md` → `docs/FOUNDATIONS.md` → `docs/ARCHITECTURE.md` → `docs/ENGINE-GAME-DATA-BOUNDARY.md` → governing area docs → accepted ADRs only where they explicitly supersede named sections → `docs/ROADMAP.md` → this spec → later candidate tasks/tickets |
| Kernel stance | No new kernel concept. Card, deck, hand, suit, rank, trick, trump, dealer, bid, contract, nil, blind nil, bag, partnership, team, and score remain outside `engine-core` |
| Promoted primitive stance | Reuse `game-stdlib::trick_taking::{follow_suit_indices, winning_play_index}` unchanged. Pass `Some(Suit::Spades)` as caller-projected public trump. No new third-use gate fires for these already-promoted helpers |
| Numeric-contract stance | Second close official use after Vow Tide. Reopen and compare as required, then keep all bid, nil, blind-nil, contract aggregation, and scoring behavior game-local; no helper promotion at second use |
| Partnership/team stance | First official competitive-team use. Record one new `local-only` primitive-pressure entry covering fixed pairing, combined ordinary contract, individual nil/blind-nil interaction, team-keyed scoring and bags, partnership visibility, and per-team outcome |
| Mechanical-scaffolding stance | First official game admitted under ADR 0008's `forward-v1` obligation. A reuse-first C-01…C-10 audit is a preimplementation admission gate; register-new and queue-or-dispose closeout are mandatory; `ci/scaffolding-audits.json` must carry a verified `forward-v1` receipt |
| Delivery posture | One authored implementation spec. It enumerates bounded candidate `AGENT-TASK` packets but creates no ticket files. `/reassess-spec` and `/spec-to-tickets` happen only after this spec is accepted |

This specification is subordinate to the foundation set. It does not redefine an upstream contract. If this spec conflicts with an earlier authority, the earlier authority wins. A product or architecture exception that changes a foundation principle requires an accepted ADR explicitly naming the affected section before implementation.

The public product name is **Blackglass Pact**. *Blackglass* evokes the polished black trump suit without reproducing a suit glyph, branded table, or third-party trade dress; *pact* evokes a fixed partnership and the public commitments made during bidding. “Spades” remains the rules-family label in source/IP notes, not the product identity. A bounded exact-title collision check found no prominent conflicting game title, but that check is not legal clearance; the human IP/public-release review required by `docs/IP-POLICY.md` remains mandatory.[^E10]

## 2. Objective

Gate 18 turns the `docs/ROADMAP.md` Gate 18 row — “Spades — partnership pairs, team scoring, contract evaluation, grouped UI” — into one concrete official-game plan. Blackglass Pact must prove that Rulepath can model a full fixed-four partnership trick-taking game in which Rust owns pre-deal blind commitments, public sequential bids, individual nil contracts, partnership aggregation, bags, team outcomes, hidden-hand projection, deterministic replay, and grouped browser presentation.[^R1]

### 2.1 Gate determination — confirmed and documented, not reopened

The next unit is settled by repository evidence:

1. `specs/README.md` records Unit 8F — forward scaffolding-reuse governance — as `Done` on 2026-06-25. It was the last predecessor interlock and explicitly names Gate 18 as the first `forward-v1` audit user.[^R2]
2. `docs/MECHANIC-ATLAS.md` §10A records no open promotion debt. Gate 17 promoted the narrow trick-taking helper, conformed Plain Tricks and Briar Circuit in-gate, and left no debt-closure unit to interpose.[^R3]
3. Gate 18 is the lowest non-`Done` row in the active-epoch tracker at Order 9. The listed predecessors — 8M, 8C, 8C-R1…R4, 8F, accepted ADRs 0008/0009, fixed trace profiles, canonical seat grammar, and the partnership/trick-taking atlas interlock — are closed.[^R2]
4. `docs/ROADMAP.md` admits Gate 18 as prescriptive ladder law and supplies the purpose, proof obligations, debt review, and exit criteria mapped in §6.[^R1]

This spec therefore does not re-decide what comes next, propose a maintenance detour, reopen Blackjack Lite, or rebuild Plain Tricks, Briar Circuit, Vow Tide, River Ledger, the promoted trick-taking helper, or the 8F governance. Those shipped artifacts are exemplars and comparison baselines.

### 2.2 Product proof

Blackglass Pact must demonstrate all of the following in one official game:

- exactly four stable seats arranged as two fixed opposite partnerships with stable team IDs;
- a full 52-card deterministic deal of 13 owner-private cards per seat;
- a pre-deal blind-nil decision surface that cannot observe or leak any hand or future-card identity;
- public sequential bidding left of dealer around the table, including ordinary numeric bids and individual nil bids;
- an ordinary partnership contract equal to the sum of the two ordinary numeric bids, with nil and blind nil evaluated separately;
- follow-suit card legality, spades-always-trump resolution, winner-leads sequencing, and a broken-spades lead restriction;
- exact-style partnership scoring, individual nil/blind-nil bonuses or penalties, cumulative overtrick bags, threshold penalties, rollover, and a 500-point terminal target;
- Rust-authored seat-keyed and team-keyed explanatory outcomes, including competition ranks and deterministic tie continuation;
- public-observer and four seat-private projections with pairwise no-leak proof across every relevant surface;
- L0 and bounded L1 opponents that reason only from public information, their own private hand, and lawful deductions;
- a grouped partnership UI that makes teams, bids, nil status, tricks, bags, scores, and outcomes legible without allowing TypeScript to decide rules.

### 2.3 Reuse posture for promoted trick-taking helpers

`game-stdlib::trick_taking::follow_suit_indices` returns stable indices matching the led suit, or all held indices when the caller is void. `game-stdlib::trick_taking::winning_play_index` returns the stable winning play index using caller-projected suit/rank values and an optional caller-projected trump suit.[^R16]

Blackglass Pact fits that existing behavior-free boundary without modification:

- use `follow_suit_indices` only after game-local phase, actor, ownership, and lead checks;
- use `winning_play_index(..., Some(Suit::Spades), ...)` after four game-local plays exist;
- keep broken-spades lead policy, first leader, winner-leads mutation, deal, bidding, nil, bags, scoring, visibility, effects, replay, and outcomes in `games/blackglass_pact`;
- add no partnership or team data to either helper;
- do not fire a new third-use hard gate and do not reopen the already-closed Gate 17 promotion.

The game-local primitive-pressure ledger must record conformance and confirm that the promoted signatures were sufficient unchanged. If implementation inspection at `/reassess-spec` discovers a real mismatch, Blackglass Pact keeps the affected behavior local and records the mismatch; it may not silently widen the shared helper.

### 2.4 Numeric bidding/contracts — mandatory second-use comparison, local result

Vow Tide is the first official numeric trick-contract use: every independent seat makes a numeric exact bid, the dealer has a last-bidder total hook, and scoring compares each seat's own bid with that seat's tricks. Its contract is per-seat, exact, and schedule-local.[^R17]

Blackglass Pact is structurally different:

- the ordinary contract is a **team aggregation** of two seats' positive numeric bids;
- each seat may instead hold an individual nil contract;
- blind nil is committed before the deal and before the actor can inspect a hand;
- tricks taken by a failed nil are deliberately excluded from the ordinary contract and become bags;
- bags persist across hands and trigger threshold penalties;
- terminal comparison and explanation are team-keyed.

This is a second-use review, not a promotion authorization. Bid leaves, commitment state, aggregation, legality, scoring, diagnostics, effects, visibility, and bot policy stay typed game-local Rust. No numeric-bid, contract, nil, bags, or scoring helper is promoted. The atlas and game ledger record the next review trigger as a **third close official numeric trick-contract game** whose shape can be compared against both Vow Tide and Blackglass Pact.

### 2.5 Partnership/team outcome — first official use

No shipped official game uses fixed competitive partnerships, team-aggregated trick contracts, persistent team bags, or per-team terminal outcomes. The existing atlas row for a shared-outcome cooperative terminal concerns all-win/all-lose cooperation and is not this competitive two-team shape.[^R3]

Under the atlas first-use rule, Gate 18 must implement and record one new `local-only` entry covering:

- fixed seat-to-team pairing;
- public stable team IDs;
- ordinary bid aggregation and ordinary-trick attribution;
- individual nil and blind-nil interaction with the team score;
- team bag accumulation, penalties, and rollover;
- team-keyed standings and outcome explanation;
- partnership visibility, including the explicit rule that a partner's hand is not visible merely because the partnership is fixed.

First use does not authorize generalization. Team membership must not be folded into generic seat identity, `engine-core`, the promoted trick-taking helper, or static data.

### 2.6 First `forward-v1` game admission

Gate 18 is the first game that must consume the completed 8F standing obligation. Before serious implementation, its `GAME-MECHANICS.md` and `GAME-IMPLEMENTATION-ADMISSION.md` must contain a reuse-first audit of every C-01…C-10 register entry and every lawful shared home. At closeout it must:

1. reuse matching promoted scaffolding or record an accepted exception;
2. register every newly invented behavior-free shape on first use as `candidate`, `local-only`, or `rejected`;
3. queue a bounded tracker unit for any matching prior-game refactor, or record an accepted local/deferred/rejected no-refactor disposition with rationale, owner, evidence, and next review trigger;
4. add a `coverage: "forward-v1"` receipt for `blackglass_pact` to `ci/scaffolding-audits.json` and pass the Gate 1 checker.[^R18][^R19]

The audit is an implementation-admission gate, not retrospective paperwork.

## 3. Scope

### 3.1 In scope — locked `blackglass_pact_standard` variant

The following parameters are normative. They must agree across `RULES.md`, `HOW-TO-PLAY.md`, `SOURCES.md`, typed Rust, rule coverage, traces, fixtures, bot docs, UI copy, simulator output, replay exports, and terminal explanations.

| Rule area | Locked Blackglass Pact rule | Research/source decision |
| --- | --- | --- |
| Seats and teams | Exactly four seats. `team_0 = {seat_0 North, seat_2 South}` and `team_1 = {seat_1 East, seat_3 West}`. Partnerships are fixed, opposite, stable, and public from setup | Classic partnership Spades is a four-player, opposite-partner game. Rulepath pins stable IDs and does not randomize partnerships.[^E1][^E2] |
| Deck and rank | Standard 52-card deck; four suits; ranks 2 through ace; ace high; no jokers; spades are always trump | Common baseline across Pagat, Bicycle, and Trickster.[^E1][^E2][^E3] |
| Initial dealer and rotation | Initial dealer is `seat_0`. Dealer rotates clockwise one seat after every completed non-terminal hand. If the match terminates, the completed-hand dealer remains in the terminal record | Sources rotate the deal clockwise; a fixed initial dealer makes seeded Rulepath replays self-contained.[^E2][^E3] |
| Blind-nil eligibility | At hand start, a seat is eligible only when its team trails the opposing team by at least 100 points. At a tied score or deficit below 100, blind nil is unavailable | A 100-point trailing threshold is a documented common option; Trickster exposes it as a standard configuration and Pagat documents trailing-only blind nil variants.[^E1][^E4] |
| Blind-nil timing | Eligible seats decide **before shuffle/deal and before any card identity exists in any viewer or bot payload**. Decision order is left of dealer clockwise. Each eligible seat receives exactly `blind_nil/declare` and `blind_nil/decline`; ineligible seats are deterministically skipped. Accepted decisions are public and immutable | Sources define blind nil as a nil declared before seeing cards. Rulepath deliberately commits before dealing, rather than merely hiding already-dealt cards, to make authorization and replay evidence unambiguous.[^E1][^E4][^E7] |
| Blind-nil interaction | A declaring seat's later ordinary bidding turn is skipped and its bid is fixed as blind nil/zero. Both partners may independently declare when eligible. There is no double-nil, double-blind-nil, or automatic-win bonus | Multiple nils can occur, but published special double-nil schedules vary widely. Blackglass Pact evaluates each seat independently and excludes special combined bonuses.[^E1] |
| Blind-nil passing | No card exchange, pass, or partner consultation follows nil or blind nil | Passing is a documented house option, not a universal rule. Excluding it preserves the fixed hidden-hand boundary and avoids a second private exchange protocol in this gate.[^E1][^E4] |
| Shuffle and deal | After blind decisions complete, derive the hand shuffle from match seed, hand index, and rules/data versions only. Blind choices must not alter RNG draw order. Deal the entire deck one card at a time clockwise beginning left of dealer; each seat receives 13 cards; no undealt tail remains | Full-deck, 13-card, one-at-a-time deal is the classic four-player baseline. RNG independence prevents a commitment from probing or steering future cards.[^E2][^E3] |
| Ordinary bidding order | Begin left of dealer and proceed clockwise, ending with dealer. Blind-nil declarers are skipped. Every other seat makes one public immutable bid | Common round-the-table order.[^E1][^E2][^E3] |
| Bid vocabulary | Legal leaves are `bid/nil` and `bid/1` through `bid/13`. There is no plain numeric zero, pass, minimum-board rule, second round, rebid, simultaneous bid, secret bid, or total-13 last-bidder hook | Nil replaces zero in the selected variant. Sources document many optional minimum/total hooks; Gate 18 excludes them, and Vow Tide's dealer hook is not imported.[^E1][^E4] |
| Ordinary team contract | Sum only the two partners' positive numeric bids. Nil and blind-nil bids contribute zero to the ordinary contract and are evaluated separately | Standard partnership aggregation, with individual nil scored separately.[^E1][^E5] |
| First leader | The seat left of dealer leads the first trick of every hand | Common baseline.[^E2][^E3] |
| Legal lead | Before spades are broken, the leader must play a non-spade if one is held. A leader holding only spades may lead a spade; that legal lead breaks spades. Once broken, any held card may be led | Common broken-spades rule with only-spades exception.[^E2][^E3][^E5] |
| Following suit | A follower holding at least one card of the led suit must play that suit. A void follower may play any held card, including a spade. The first legal off-suit spade breaks spades | Common baseline.[^E1][^E2][^E5] |
| Trick winner | Highest spade wins if any spade was played; otherwise highest rank of the led suit wins. Off-suit non-spades cannot win. The winning seat leads the next trick | Common baseline and exact fit for the promoted comparator with caller-projected spades trump.[^E2][^E3][^E5] |
| Ordinary contract scoring | Let `C` be the partnership's combined positive numeric bid and `O` be tricks won by the seats that made positive numeric bids. If `O >= C`, ordinary base is `+10 × C`; otherwise it is `−10 × C` | The ±10× contract style is the standard partnership form described by Pagat and many online implementations.[^E1][^E5] |
| Ordinary overtricks | Only when `O >= C`, ordinary overtricks are `O − C`. Each adds `+1` point and one bag. A set ordinary contract produces no ordinary overtrick points or ordinary bags | +1 overtrick and cumulative-bag scoring is a common sandbagging form.[^E1][^E3][^E4] |
| Nil scoring | A nil bidder taking zero tricks adds `+100`; taking one or more adds `−100` | Common nil schedule.[^E1][^E4] |
| Blind-nil scoring | A blind-nil bidder taking zero tricks adds `+200`; taking one or more adds `−200` | Common double-nil-value schedule.[^E1][^E4] |
| Failed nil trick attribution | Tricks taken by a failed nil or blind-nil bidder do **not** help the partner make the ordinary contract. Every such trick adds `+1` point and one bag, whether the ordinary contract was made or set | Pagat identifies this as the usual rule; Trickster exposes “takes bags” as an explicit option. UChicago's no-bag failed-nil rule is documented but deliberately not selected.[^E1][^E4][^E6] |
| Bag threshold and rollover | Bags persist across hands. For every 10 accumulated bags, subtract 100 points and remove 10 bags; apply repeatedly if a hand crosses multiple thresholds. The remainder carries. Bags do not reset on a set, a nil result, or reaching the target | Cumulative 10-bag/−100 with rollover is a common baseline.[^E1][^E4][^E6] |
| Score application order | Compute ordinary base, ordinary overtricks, each nil/blind-nil delta, new failed-nil bags, and bag penalties as one Rust-authored hand breakdown. Add bag-point increments before subtracting each threshold penalty; store score and bag remainder separately | Explicit ordering avoids decimal-score tricks and makes traces/outcomes stable. It is Rulepath's deterministic formalization of the selected scoring facts. |
| Target and terminal | After a complete hand, if at least one team has score `>= 500` and the two team scores differ, the higher-scoring team wins. If scores are exactly tied at or above 500, play another complete hand. Continue until a qualifying hand ends with a unique higher score | 500 is common. Pagat awards the higher team when both cross together; common rules use extra hands for exact ties. Blackglass Pact never uses bags or seat order as an arbitrary tiebreak.[^E1][^E2][^E3] |
| Public facts | Seat/team mapping, dealer, hand/phase, blind-nil eligibility and accepted decisions, accepted bids, active actor, spades-broken flag, public played cards, trick winners/counts, score components, team scores/bags/ranks, and terminal outcome | Rust projects these facts; TypeScript may only render them. |
| Private/internal facts | Before deal: shuffled future-card identities and RNG state. After deal: each unplayed hand is owner-private. Also private are unauthorized action trees/previews/candidate rankings, internal full trace, seeds, and any inference state not expressly projected | Required by the multi-seat, bot, testing, UI, and replay contracts.[^R6][^R7][^R8][^R9][^R12] |
| Variants | Only `blackglass_pact_standard` is official in Gate 18. Typed data may hold identity, text, presentation, and bounded parameters, but may not encode selectors, conditions, triggers, bid legality, score formulas, or bags behavior | No YAML, no DSL, and no behavior in static data.[^R5] |

### 3.2 Normative phase and action order

The game-local state machine has these externally relevant phases:

1. `BlindNilCommitment` — present only while one or more eligible seats still owe a public declare/decline decision. No hand or future-card identity may be present in any actor view, legal tree, preview, bot input, candidate explanation, effect, export, browser state, or log.
2. `Bidding` — begins only after deterministic deal completion. Blind-nil seats are skipped; other seats submit one immutable `Bid::Nil` or `Bid::Tricks(1..=13)` in clockwise order.
3. `PlayingTrick` — first leader is left of dealer; subsequent leaders are prior trick winners. Rust emits only legal owned cards after follow-suit and broken-spades checks.
4. `Terminal` — entered only after a complete hand produces a unique higher team score with at least one team at or above 500.

Hand scoring and next-hand setup are deterministic transitions/effect batches, not client-driven action phases.

The canonical action paths are:

```text
blind_nil/declare
blind_nil/decline
bid/nil
bid/1
...
bid/13
play/<stable-card-id>
```

No browser-only alias may create a second legality vocabulary.

### 3.3 Normative scoring model

For each team and completed hand:

```text
C = sum(positive numeric bids by the team's ordinary bidders)
O = sum(tricks won by those ordinary bidders)
ordinary_made = O >= C
ordinary_base = if ordinary_made { 10 * C } else { -10 * C }
ordinary_overtricks = if ordinary_made { O - C } else { 0 }
failed_nil_bags = sum(tricks won by failed nil and failed blind-nil bidders)
new_bags = ordinary_overtricks + failed_nil_bags
nil_delta = sum(+100 made nil, -100 failed nil,
                +200 made blind nil, -200 failed blind nil)
raw_bags = prior_bags + new_bags
bag_penalty_count = floor(raw_bags / 10)
next_bags = raw_bags mod 10
hand_delta = ordinary_base + new_bags + nil_delta
             - 100 * bag_penalty_count
next_score = prior_score + hand_delta
```

Normative consequences:

- a nil bidder's tricks never increase `O`;
- failed nil tricks are bags even when the ordinary contract is set;
- a team with two nil contracts has `C = 0` and `O = 0`; each nil is still evaluated independently;
- an ordinary `C = 0` contributes zero base points and cannot manufacture ordinary bags;
- crossing 20 bags in one hand applies two penalties and retains the remainder;
- score and bag remainder are separate fields; the unit digit of score is not authoritative bag storage;
- the result must be represented as a Rust-authored ordered breakdown, not recomputed by WASM JavaScript or React.

### 3.4 In-scope modes and surfaces

| Mode / surface | Gate 18 requirement |
| --- | --- |
| Human vs bot | Local browser match with one or more human-controlled seats and remaining seats using L0 or bounded L1. Team assignment remains fixed. |
| Hotseat | Viewer handoff removes the previous private subtree before requesting/rendering the next seat view. Opposite partners receive no shared private-hand surface. |
| Bot vs bot | Deterministic native simulation and replay generation at fixed four seats, with seat-keyed and team-keyed summaries. |
| Public observer | Public bids, plays, trick results, team scores/bags/outcomes, and fixed partnerships; no unplayed hand, future cards, private controls, private bot candidates, or seat-private export content. |
| Seat-private viewer | Own hand and own legal controls plus all public facts; never partner or opponent unplayed cards. |
| Team grouping | Two public team summary regions keyed by stable team IDs. Team grouping is presentation and public state, not authorization to reveal partner-private data. |
| Replay | Native internal full trace remains test authority. Browser/public and seat-private exports are viewer-scoped observation histories under ADR 0004/0009. |
| Outcome explanation | Rust-authored per-team standings and per-seat/per-team hand breakdowns rendered through the shared outcome surface. |
| Rules/help | Original `HOW-TO-PLAY.md` is copied/checked into the public rules surface; formal `RULES.md` remains maintainer authority and is not rendered directly. |

### 3.5 Out of scope

| Area | Gate 18 stance |
| --- | --- |
| Alternate player counts | Out of scope. No 2-, 3-, 5-, 6-, or 8-seat variant and no cutthroat mode. |
| Alternate partnership layouts | Out of scope. No inferred, temporary, chosen, asymmetric, rotating, or three-team grouping. |
| Jokers/deuces-high/deck variants | Out of scope. No big/little joker, deuce-high, stripped deck, second deck, or wild card. |
| Partnership bidding/table talk | Out of scope. Seats bid independently in public sequence; no communication channel, shared hand, or negotiated team bid. |
| Bid variants | Out of scope: minimum “board,” total-bids-not-13 hook, second bidding round, bid reduction, simultaneous/secret bids, blind numeric bids, 10-for-200, Boston/moon, Bemo, no-trump bid, Suicide, Whiz, Mirrors, or nil-in-the-dark. |
| Nil variants | Out of scope: nil passing/exchange, partner consultation, double-nil special scoring, double-blind auto-win, failed nil helping ordinary contract, failed nil immunity from bags, or nil score values other than the locked schedule. |
| Play variants | Out of scope: fixed-card first lead, lowest-club first trick, no-spade first trick beyond the normal broken rule, spades-lead-anytime, renege/call-renege, review-all-hands, or claim-the-rest. |
| Score/end variants | Out of scope: 200/300/1000 target, fixed hand count, mercy/backdoor loss, no-deduction sets, per-undertrick penalties, bags-lost/win tiebreak, bags immune on blind nil, no-bag mode, quicksand, 5-bag threshold, 10-for-200, or arbitrary seat tiebreak. |
| Networking/accounts/chat/tournaments/stakes | Out of scope under local-first public v1/v2. No casino, wagering, leaderboard, or real-money framing. |
| L2 bot | Not admitted in this gate. A later L2 requires accepted competent-player and strategy-evidence material first. |
| L3 deterministic search | `not applicable`: AI-BOTS permits L3 only for perfect-information games. |
| Search/sampling/learning AI | Forbidden for public v1/v2: no MCTS, ISMCTS, Monte Carlo, rollout, determinization, sampled worlds, ML, RL, or runtime LLM move selection. |
| Generic partnership/team framework | Out of scope. First use remains local and must not alter generic seat identity. |
| Generic bid/contract/nil/bags framework | Out of scope. Second use remains local; no promotion. |
| Trick-taking helper expansion | Out of scope. Reuse the two existing helper functions unchanged; no partnership, broken-spades, bidding, scoring, or orchestration is added. |
| `engine-core` nouns | Forbidden. No card/deck/hand/suit/rank/trick/trump/bid/contract/nil/bag/team/partnership noun enters the kernel. |
| Behavior in data | Forbidden. No score formula, bid rule, follow-suit rule, broken-spades condition, nil eligibility, team assignment, selector, trigger, or executable rule table in TOML/JSON/static content. |
| Foundation amendments | `not applicable` unless `/reassess-spec` discovers a genuine uncovered foundation gap. The expected posture is updates-only; no silent amendment. |
| Trace/hash migration | Out of scope absent an accepted ADR naming the changed surface. No blanket golden regeneration. |
| Ticket files | Out of scope. This is the authored spec; decomposition occurs later. |
| Rebuilding shipped games | Out of scope. Prior games are read-only baselines unless the forward-v1 audit truthfully exposes a bounded scaffolding retrofit, which must be separately queued or disposed rather than smuggled into Gate 18. |

### 3.6 ROADMAP public-scaling prohibition — carried forward

Gate 18 must not introduce private licensed content, copied rules prose or trade dress, YAML/DSL behavior, TypeScript legality, public MCTS/ISMCTS/Monte Carlo/ML/RL bots, kernel noun growth, hidden-state leakage, or private work that silently shapes public architecture.[^R1]

### 3.7 Per-gate debt review

| Debt lane | Gate 18 admission/closeout requirement |
| --- | --- |
| Mechanic-atlas pressure | Reuse promoted trick-taking helpers unchanged; complete numeric-contract second-use review and keep local; add partnership/team first-use local row; leave §10A empty unless an independently justified promotion is actually earned. No promotion is expected. |
| Mechanical-scaffolding debt | Complete C-01…C-10 reuse-first audit before admission; register-new every new behavior-free shape; queue-or-dispose every prior matching site; add passing `forward-v1` receipt. |
| Trace debt | Trace Schema v1 is sufficient. Add exact event/field coverage for blind commitment, public bids, team IDs, score components, and bag rollover. Any schema gap is a stop condition, not permission for ad hoc fields. |
| Fixture-profile debt | Instantiate all required evidence profiles and the N-seat hidden-information completion profile; use explicit `not applicable` rows. |
| Seat/viewer grammar debt | Reuse canonical seat grammar. Add game-local stable team IDs and public fixed grouping without changing seat identity. Exercise observer plus all four seat viewers. |
| Replay/hash debt | No migration expected. Use ADR 0009 bounded authorization for any discovered byte change; never regenerate unrelated goldens. |
| Evidence-receipt blockers | Game cannot close without official-game evidence, all no-leak/export receipts, benchmark receipts, catalog/doc checks, and the `forward-v1` scaffolding audit receipt. |

## 4. Deliverables

### 4.1 New game crate and official evidence tree

The implementation creates the following new crate. Exact file subdivision may be reassessed when a bounded task starts, but every responsibility and evidence surface below remains required.

```text
games/blackglass_pact/
├── Cargo.toml
├── benches/
│   ├── blackglass_pact.rs
│   └── thresholds.json
├── data/
│   ├── fixtures/
│   │   ├── blackglass_pact_standard.fixture.json
│   │   ├── blackglass_pact_blind_nil.fixture.json
│   │   ├── blackglass_pact_bags_rollover.fixture.json
│   │   ├── blackglass_pact_double_bag_penalty.fixture.json
│   │   └── blackglass_pact_target_tie.fixture.json
│   ├── manifest.toml
│   └── variants.toml
├── docs/
│   ├── AI.md
│   ├── BENCHMARKS.md
│   ├── BOT-STRATEGY-EVIDENCE-PACK.md
│   ├── COMPETENT-PLAYER.md
│   ├── GAME-EVIDENCE.md
│   ├── GAME-IMPLEMENTATION-ADMISSION.md
│   ├── HOW-TO-PLAY.md
│   ├── MECHANICS.md
│   ├── PRIMITIVE-PRESSURE-LEDGER.md
│   ├── PUBLIC-RELEASE-CHECKLIST.md
│   ├── RULE-COVERAGE.md
│   ├── RULES.md
│   ├── SOURCES.md
│   └── UI.md
├── src/
│   ├── actions.rs
│   ├── bidding.rs
│   ├── bots.rs
│   ├── cards.rs
│   ├── effects.rs
│   ├── ids.rs
│   ├── lib.rs
│   ├── partnerships.rs
│   ├── replay_support.rs
│   ├── rules.rs
│   ├── scoring.rs
│   ├── setup.rs
│   ├── state.rs
│   ├── ui.rs
│   ├── variants.rs
│   └── visibility.rs
└── tests/
    ├── bots.rs
    ├── golden_traces.rs
    ├── golden_traces/
    │   └── *.trace.json
    ├── property.rs
    ├── replay.rs
    ├── rules.rs
    ├── serialization.rs
    └── visibility.rs
```

The filled game documents instantiate the repository templates while following the existing per-game naming convention. `GAME-EVIDENCE.md` keeps that exact filename because the current template makes it the canonical status and artifact-link receipt.[^R20]

### 4.2 Requirements-first official-game documents

| Filled artifact | Required content for Blackglass Pact | Governing contract |
| --- | --- | --- |
| `docs/SOURCES.md` | Source bibliography, rule-variation reconciliation, selected-variant decisions, deliberate deviations, neutral-name rationale, asset/font plan, original-prose declaration, external implementation prior art, and human review status | OFFICIAL-GAME-CONTRACT §§3–5; IP-POLICY §§4–6, §11–§12; `docs/SOURCES.md` |
| `docs/RULES.md` | Original Rulepath prose keyed to stable `BP-*` rule IDs; exact seat/team grammar; blind-nil timing; bid/play/scoring/end law; viewer facts; diagnostics; no copied sequence or examples | OFFICIAL-GAME-CONTRACT §5; GAME-RULES template |
| `docs/RULE-COVERAGE.md` | Every rule ID mapped to Rust owner, named tests, properties, traces, replay/export evidence, UI smoke, bot evidence, and benchmark workload; no orphan implementation and no uncovered rule | OFFICIAL-GAME-CONTRACT §6; TESTING §1/§17 |
| `docs/MECHANICS.md` | Full mechanic inventory; promoted trick-helper reuse; Vow Tide↔Blackglass Pact second-use comparison; new partnership first-use entry; C-01…C-10 reuse-first audit; expected register and prior-game dispositions | MECHANIC-ATLAS §§4–5B/§9A/§10; MECHANICAL-SCAFFOLDING-REGISTER; ADR 0008 |
| `docs/GAME-IMPLEMENTATION-ADMISSION.md` | Signed admission receipt showing rules/source/coverage readiness, boundary checks, primitive decisions, completed pre-code scaffolding audit, required evidence profiles, and no active stop condition | OFFICIAL-GAME-CONTRACT §3/§12; FOUNDATIONS §12 |
| `docs/GAME-EVIDENCE.md` | Completion profile `hidden-information + n-seat + release-candidate`; artifact-link/status receipt; all named trace profiles; viewer/no-leak links; replay/hash status; primitive/scaffolding status; forward-v1 CI receipt; release blockers | EVIDENCE-FIXTURE-CONTRACT; GAME-EVIDENCE template |
| `docs/HOW-TO-PLAY.md` | Original player-facing rules and examples for the shared web Rules surface; distinct from formal requirements and strategy prose | OFFICIAL-GAME-CONTRACT §5/§10; UI-INTERACTION; IP-POLICY |
| `docs/COMPETENT-PLAYER.md` | Sourced, rules-checked analysis of bidding, nil risk, partner coverage through public play, contract management, bag pressure, setting, and score posture | AI-BOTS competent-player gate |
| `docs/BOT-STRATEGY-EVIDENCE-PACK.md` | Status `not admitted / intentionally deferred` for L2, with the complete evidence needed before any authored L2 coding; no empty placeholder | AI-BOTS §2/§4A; template |
| `docs/AI.md` | L0 and L1 policy IDs, authorized observations, deterministic priorities/ties, memory limits, explanation safety, test seeds, and explicit prohibition of hidden-world/search/learning methods | AI-BOTS §§2–4A/§12 |
| `docs/UI.md` | Grouped partnership table, phase controls, legal-only interaction, Rust-safe previews, effects, replay, public observer, hotseat erasure, no-leak/a11y matrix, responsive budgets, and e2e acceptance | OFFICIAL-GAME-CONTRACT §10; UI-INTERACTION; MULTI-SEAT |
| `docs/BENCHMARKS.md` | Workload IDs, seed/fixture manifest, native and browser measurements, variance/floors, by-seat/by-team output shape, regression decisions, and hardware/toolchain context | TESTING §§12–17; benchmark ADRs |
| `docs/PRIMITIVE-PRESSURE-LEDGER.md` | Reuse note for promoted trick helpers; numeric-contract second-use keep-local decision and next trigger; new partnership/team first-use local entry; no false promotion debt | MECHANIC-ATLAS; template |
| `docs/PUBLIC-RELEASE-CHECKLIST.md` | Public rules, IP/assets, catalog, UI, accessibility, no-leak, viewer exports, outcomes, bots, benchmarks, e2e, receipts, and human sign-off | OFFICIAL-GAME-CONTRACT §12; IP/UI/testing contracts |

No candidate task may treat these documents as optional end-of-gate prose. Sources, rules, coverage, mechanics, admission, and the initialized evidence receipt precede serious implementation.

### 4.3 Typed Rust behavior

The new crate must provide:

- typed `SeatId` and game-local `TeamId` mappings with canonical serialization order;
- typed `Card`, `Suit`, `Rank`, stable `CardId`, owner-private hands, played-card history, and deterministic deck construction;
- typed `Phase`, `Bid`, blind decision, current trick, completed trick, score state, bag state, and match outcome;
- deterministic setup, dealer rotation, pre-deal eligibility, blind decision order, shuffle/deal, bidding order, trick order, scoring, and terminal evaluation;
- Rust-authored legal action trees and exact validators for blind decisions, bids, and card play;
- use of the promoted follow-suit and winner-index helpers without changing their signatures or ownership boundary;
- semantic effects for phase entry, blind decision, deal completion, accepted bid, spades breaking, card play, trick capture, score breakdown, bag threshold, dealer advance, and match completion;
- public and four seat-private views, with no team-only viewer and no partner-hand privilege;
- Rust-authored previews only where safe and useful; a preview must not expose future cards, partner cards, or hidden bot state;
- deterministic stable-byte, serialization, trace, replay, export/import, and hash support under existing versions;
- Rust-authored L0 and bounded L1 bot decisions using only an authorized viewer state and legal leaves;
- public terminal outcome with stable per-team and per-seat arrays.

### 4.4 Fixture, trace, replay, and test corpus

Deliver at minimum:

- one standard setup/short-path fixture;
- one blind-nil eligible fixture with no pre-deal card identities in viewer evidence;
- one single-threshold bags rollover fixture;
- one multi-threshold bags fixture;
- one exact target tie/continuation fixture;
- the golden-trace floor in §7.6;
- named unit tests for every `BP-*` rule;
- property tests for card conservation, legal-tree/validator equivalence, follow-suit, comparator conformance, score arithmetic, bag rollover, team mapping, terminal conditions, and deterministic replay;
- serialization tests for stable seat/team order and map-free or canonically sorted byte output;
- visibility tests for observer, all four seat viewers, all 12 ordered seat-to-seat pairs, blind pre-deal surfaces, effects, diagnostics, bot explanations, and exports;
- WASM-exported public and seat-private terminal traces.

### 4.5 Repository, CI, tool, WASM, and web registration

| Surface | Required Gate 18 change |
| --- | --- |
| Workspace | Add `games/blackglass_pact` to the Rust workspace and lockfile through normal dependency resolution. |
| `ci/games.json` | Add the canonical game ID and all required official-game metadata in the established schema. |
| `ci/scaffolding-audits.json` | Add exactly one `blackglass_pact` record with `coverage: "forward-v1"`, evidence paths, all MSC reviews, truth-based disposition, known-signal dispositions, prior-match closeout, and no unauthorized compatibility migration. |
| `crates/wasm-api` | Add game dependency/import; ID/display/rules/variant constants; adapter module; catalog record; action/view/effect/replay/export/import/bot dispatch; fixed-four setup; public/seat viewer support; API snapshot updates limited to authorized new rows. |
| `tools/simulate` | Add `blackglass_pact` game dispatch, fixed-four validation, bot orchestration, deterministic seed handling, and seat/team summaries. |
| `tools/replay-check` | Register game-specific replay/export validators and all required viewer variants where the tool uses explicit dispatch. |
| `tools/fixture-check` | Not generic: `resolve_game` (`src/main.rs`) dispatches via a hard-coded `match game`. Add a new match arm, a `blackglass_pact` path-dependency to `tools/fixture-check/Cargo.toml`, and a `BLACKGLASS_PACT_ALLOWED_JSON_KEYS` constant mirroring the existing per-game key lists (e.g. `VOW_TIDE_ALLOWED_JSON_KEYS`). Reuse the shared validator body; do not fork a duplicate driver. |
| `tools/rule-coverage` | Not generic: `resolve_game` (`src/main.rs`) dispatches via a hard-coded `match game`. Add a new match arm wiring `games/blackglass_pact/docs/{RULES,RULE-COVERAGE,BENCHMARKS}.md`; no game-crate dependency is required (the tool reads doc files by path). |
| Web catalog | Add the game to Rust catalog/WASM data and the React picker; render the neutral public name and original catalog description. |
| Board renderer | Add `apps/web/src/components/BlackglassPactBoard.tsx` or a reassessed equivalent dedicated shell-surface renderer using shared presentation scaffolding where applicable. |
| Public rules | Add generated/checked `apps/web/public/rules/blackglass_pact.md` and manifest entry derived from `HOW-TO-PLAY.md`. |
| E2E | Add `apps/web/e2e/blackglass-pact.smoke.mjs` covering setup, blind phase, bidding, card play, team grouping, score/bags, observer/no-leak, replay, a11y, and terminal outcome. |
| Web scripts/docs | Add the smoke to `smoke:e2e`; update `apps/web/README.md` in the intro catalog list, Shell Surface renderer list, and Smoke Layers list; pass `scripts/check-catalog-docs.mjs`. |
| Documentation | Update the repo docs named in §10 and no others unless reassessment identifies a real linked obligation. |

### 4.6 `forward-v1` audit receipt deliverable

The final `ci/scaffolding-audits.json` record must be generated against the live checker schema, not copied blindly from this spec. It must nevertheless satisfy these normative facts:

| Receipt field/surface | Gate 18 requirement |
| --- | --- |
| `id` | `blackglass_pact` |
| `coverage` | `forward-v1`; Gate 18 may not claim `legacy-8c-covered` |
| `evidence_paths` | Existing in-repository paths that include at least `games/blackglass_pact/docs/MECHANICS.md`, `games/blackglass_pact/docs/GAME-EVIDENCE.md`, and `docs/MECHANICAL-SCAFFOLDING-REGISTER.md`; add admission/ledger paths if the checker and evidence model require them |
| `register_entries_reviewed` | All `MSC-8C-001` through `MSC-8C-010` exactly once in stable order |
| `register_decisions` | Empty only when the completed implementation introduced no new behavior-free shape and no register decision is required; otherwise list every new/changed MSC decision supported by the register |
| `disposition` | Truth-based allowed value such as `reuse-only`, `no-new-scaffolding`, `register-updated`, `accepted-local-only`, `accepted-deferred`, or `accepted-rejected`; do not preselect a convenient value before code review |
| `prior_matching_games` | Every earlier official game whose matching behavior-free shape was actually identified, otherwise `[]` |
| Follow-on/no-unit surface | A named bounded tracker unit when a prior match needs work, or the checker's accepted no-unit decision pointing to the governing legacy/register decision; never an unnamed TODO |
| Known signals | Resolve every signal the checker detects, including effect-envelope literals, local seat grammar, local action-tree framing, local stable-byte writer, and production support edges, using allowed decisions and real evidence paths |
| Compatibility | `hash_migration`, `visibility_migration`, and `determinism_migration` are expected `none`, with `migration_authority: "none"`. Any non-none value requires accepted ADR 0009 authority and bounded migration evidence |
| Text discipline | The machine receipt contains only schema-approved fields and concise evidence references. It must not smuggle behavior-bearing rules into CI metadata and must pass unknown-field and prohibited-language checks |

The expected design-time result is reuse of existing scaffolding plus game-local behavior, no new behavior-free shape, and no prior-game retrofit. That is an **expectation**, not a preapproved closeout claim. The post-build audit must change the register and/or queue/dispose prior work if implementation evidence disproves it.

### 4.7 Static-data boundary

`data/manifest.toml` and `data/variants.toml` may contain only typed identity, bounded setup metadata, version strings, public copy references, fixture references, and non-behavioral presentation parameters. The following remain compiled Rust:

- fixed team membership;
- blind-nil deficit eligibility and action order;
- legal bid vocabulary and sequencing;
- follow-suit and broken-spades legality;
- trump and trick resolution;
- ordinary/nil/blind-nil scoring;
- bag accumulation and threshold penalties;
- terminal target and tie continuation;
- visibility authorization;
- bot priorities.

No YAML file and no rule DSL is introduced.

### 4.8 Deliverable-to-contract crosswalk

| Deliverable cluster | OFFICIAL-GAME-CONTRACT | MULTI-SEAT / surface law | TESTING / EVIDENCE | AI / UI / boundary |
| --- | --- | --- | --- | --- |
| Sources, formal rules, public rules | §§3–5 | seat/team declaration included | rule IDs feed coverage | IP policy; original prose |
| Coverage, mechanics, admission | §3/§6/§12 | §14 spec minimums | taxonomy and stop-condition evidence | engine/game boundary; atlas/register |
| Rust crate | §7–§9 | §§2–3/§11/§13 | unit/property/serialization/replay | Rust authority; noun-free kernel |
| Views/exports | §10–§12 | §§5–7/§10–§11 | §8.1/§8.2; ADR 0004/0009 | no-leak UI/WASM boundary |
| Traces/fixtures | §11–§12 | every viewer/team outcome | Trace Schema v1; Evidence Fixture Contract | deterministic replay/hash |
| Bots | §12 | partnership-aware authorized information | seed/replay/explanation evidence | AI-BOTS L0/L1 gate |
| Web surface | §10/§12 | grouped partnership budgets | e2e/no-leak/a11y evidence | UI-INTERACTION; TS presentation only |
| Benchmarks/simulation | §12 | by-seat and by-team summaries | §§12–17 and benchmark ADRs | bot/renderer workloads |
| Primitive/scaffolding receipts | §3/§12 | no team leak/generalization | Gate 1 receipt and artifact links | atlas; ADR 0008; register |

## 5. Work breakdown

The rows below are **candidate** bounded `AGENT-TASK` packets, not ticket files. `/reassess-spec` validates the live code seams, then `/spec-to-tickets` may split or merge them while preserving dependency order and acceptance intent.

| Order | Candidate task ID | Bounded objective | Depends on | Required proof before completion |
| ---: | --- | --- | --- | --- |
| 1 | `GAT18-BLAPAC-001` | Finalize `SOURCES.md`, neutral-name/IP notes, exact variant reconciliation, stable `BP-*` rule IDs, original `RULES.md`, and initial `RULE-COVERAGE.md` | Accepted spec | Every selected parameter cites sources or is marked a deliberate Rulepath formalization; no copied prose; human IP review owner named |
| 2 | `GAT18-BLAPAC-002` | Complete the **preimplementation `forward-v1` reuse-first audit** in `MECHANICS.md`, all C-01…C-10 decisions, lawful-home review, expected register updates, prior-game disposition, `GAME-EVIDENCE.md` initialization, and implementation admission | 001 | Audit complete; every N/A has rationale; all stop conditions reviewed; serious code remains blocked until admission passes |
| 3 | `GAT18-BLAPAC-003` | Add crate skeleton, workspace/CI game identity, fixed-four setup, stable seats/teams, deck/card model, versions, typed state, diagnostics, and deterministic dealer/hand context | 002 | Unsupported seats rejected; team mapping stable; card conservation and serialization tests pass; no kernel noun growth |
| 4 | `GAT18-BLAPAC-004` | Implement pre-deal blind-nil eligibility, action tree, declare/decline validation, public effects, RNG independence, deterministic full deal, and owner-private hand projection | 003 | No card identity reaches any pre-deal blind surface; declare/decline does not alter deal bytes; blind traces and pairwise tests pass |
| 5 | `GAT18-BLAPAC-005` | Implement public sequential bidding, nil and numeric bid leaves, blind-seat skips, team contract derivation, immutable accepted bids, diagnostics, views, and effects | 004 | Exact bid order/tree/validator equivalence; no Vow Tide total hook; public bids stable in replay/export |
| 6 | `GAT18-BLAPAC-006` | Integrate promoted trick-taking helpers, implement broken-spades lead policy, ownership/follow-suit validation, trick mutation, winner-leads, and helper-conformance tests | 005 | Helper signatures unchanged; `Some(Spades)` comparator proof; no partnership policy shared; all play rule/property tests pass |
| 7 | `GAT18-BLAPAC-007` | Implement team scoring, ordinary-trick attribution, nil/blind-nil outcomes, bag points, repeated threshold penalties, rollover, dealer advance, 500 target, tie continuation, and Rust-authored outcome arrays | 006 | Worked score corpus and properties pass; per-team/per-seat explanations match state; exact tie cannot terminate |
| 8 | `GAT18-BLAPAC-008` | Complete public/seat visibility, safe previews/diagnostics/effects, pairwise no-leak harness, viewer-scoped replay exports/imports, and ADR 0004/0009 hash receipts | 004–007 | Observer + all four seats + all 12 ordered pairs; all four seat-private exports; blind pre-deal taxonomy; no migration without authority |
| 9 | `GAT18-BLAPAC-009` | Implement L0 and bounded L1 policies; complete `COMPETENT-PLAYER.md`, `AI.md`, and deferred L2 evidence pack | 005–008 | Authorized inputs only; deterministic priorities; legal-path validation; no sampled worlds/search/learning; viewer-safe explanations |
| 10 | `GAT18-BLAPAC-010` | Build rule tests, properties, serialization, replay, visibility, bots, fixtures, golden traces, rule coverage, and `GAME-EVIDENCE.md` profile links | 003–009 | §7 taxonomy and golden floor complete; no weakened/deleted test; every `BP-*` ID covered |
| 11 | `GAT18-BLAPAC-011` | Add simulator dispatch, deterministic all-L0 and mixed L1 runs, seat/team summaries, native benchmarks, thresholds, and benchmark report | 009–010 | Fixed seed manifests, action-cap diagnostics, no nontermination, by-seat/by-team output, provisional budgets assessed honestly |
| 12 | `GAT18-BLAPAC-012` | Add WASM adapter/catalog/constants/dispatch, replay/export support, tool registrations, API snapshot rows, public rules generation, and generic-driver conformance | 008–011 | Rust/WASM parity, API tests, fixture/replay/rule tools, no duplicated generic driver, no browser legality |
| 13 | `GAT18-BLAPAC-013` | Add grouped partnership browser renderer, legal blind/bid/card controls, effects/animation, score/bags/outcomes, hotseat/observer/replay surfaces, accessibility, and dedicated e2e smoke | 012 | §7 browser tests; no hidden nodes/state; color-independent team identity; all score facts sourced from Rust |
| 14 | `GAT18-BLAPAC-014` | Close primitive-pressure and **post-build forward-v1** governance: helper reuse note, numeric second-use atlas/ledger update, partnership first-use row, register freshness, prior-game queue-or-dispose, final CI receipt | 010–013 | Atlas/register/ledger/evidence agree; `forward-v1` checker passes; §10A truthfully empty unless a separately justified promotion exists |
| 15 | `GAT18-BLAPAC-015` | Complete public-release checklist, repo/web docs, catalog lists, source bibliography, command suite, evidence links, status flip, archival readiness, and final foundation/stop-condition review | 014 | All §6 exit rows and §7 commands pass; no open blocker; `specs/README.md` Gate 18 flips only after evidence is complete |

### 5.1 Candidate-task discipline

Every task packet produced later must:

- name exact in-scope and forbidden paths;
- restate the authority order and relevant rule IDs;
- declare whether it touches hash, RNG, visibility, replay, fixture, UI, bot, primitive, or scaffolding surfaces;
- determine whether any failing test remains valid before deciding whether the SUT or the test is wrong;
- never delete, skip, loosen, or mass-regenerate evidence to obtain green;
- include exact commands and artifacts in acceptance evidence;
- stop on a foundation §12 condition or unauthorized migration;
- avoid opportunistic cleanup of shipped games unless a named follow-on unit expressly owns it.

## 6. Exit criteria

### 6.1 ROADMAP Gate 18 mapping — row for row

| ROADMAP proof obligation | Gate 18 exit criterion | Required evidence |
| --- | --- | --- |
| Partnership pairs | Exactly four seats are accepted; fixed opposite pairs map to `team_0`/`team_1`; mapping is public, stable, serialized deterministically, and never substitutes for seat identity | Setup/rules/serialization traces; viewer matrix; UI grouping smoke |
| Team scoring | Rust computes every ordinary, nil, blind-nil, bag, penalty, hand-delta, cumulative-score, and rank field keyed by stable team ID | Named scoring tests/properties; hand-score traces; outcome JSON; WASM parity |
| Contract evaluation | Public sequential bids produce one ordinary team contract plus independent nil/blind-nil contracts; failed nil tricks do not help ordinary contract | Bid and score rule tests; contract traces; rule coverage |
| Grouped UI | Browser groups opposite partners and presents team score/bags/combined contract while preserving seat-level bid/nil/trick facts; TypeScript does not calculate them | UI component tests/e2e; Rust payload snapshots; accessibility checks |
| Team outcome pressure | Terminal output contains per-team score/rank/winner plus per-seat and per-team final breakdowns, in stable arrays suitable for traces/simulator/UI | Terminal traces; outcome explanation smoke; simulator JSON |
| Partnership visibility pressure | Partner hand stays private; every public/team fact is explicitly public; no team-only viewer is invented; blind commitment and all four seat projections satisfy no-leak law | 12-pair matrix, observer tests, all-seat export/import, DOM/storage/log checks |
| Mechanical-scaffolding closeout (ROADMAP Exit) | The `forward-v1` audit registers every new behavior-free shape, records all reused promoted scaffolding, and queues a named follow-on unit or accepted no-unit disposition for any prior-game match before the game is marked official | §6.2 / §7.10 forward-v1 evidence; `ci/scaffolding-audits.json` receipt; `node scripts/check-scaffolding-governance.mjs` pass |

### 6.2 Locked game-behavior exit criteria

The gate is not complete until all of these statements are true:

- [ ] setup accepts only four seats and creates exactly two fixed opposite partnerships;
- [ ] the first hand uses dealer `seat_0`, later non-terminal hands rotate dealer clockwise, and no terminal phantom advance occurs;
- [ ] blind nil is offered only to seats on a team trailing by at least 100 at hand start;
- [ ] blind decisions happen before deal, are public, immutable, ordered left of dealer, and cannot observe or influence future card identities;
- [ ] the full 52-card deck is dealt deterministically, one at a time, 13 cards to each seat, with no tail;
- [ ] non-blind seats bid publicly left of dealer through dealer using nil or 1–13; no pass, rebid, numeric zero, or total-13 hook exists;
- [ ] ordinary contract is the sum of positive numeric partner bids;
- [ ] first lead is left of dealer, players follow suit when able, spades are always trump, spades cannot be led before breaking except from an all-spade hand, and trick winner leads next;
- [ ] both promoted helper functions are reused unchanged and conformance-tested;
- [ ] ordinary contract, nil, blind nil, failed-nil attribution, bags, multi-threshold penalties, and rollover follow §3.3 exactly;
- [ ] the match ends only after a full hand with at least one team at 500+ and a unique higher score; exact target ties continue;
- [ ] terminal results include stable `standings_by_team` and `standings_by_seat`, winner team IDs, and complete explanatory score breakdowns;
- [ ] L0 and bounded L1 complete deterministic matches without hidden-state access or prohibited algorithms;
- [ ] public observer, all four seat viewers, and all four seat-private replay exports pass no-leak and round-trip checks;
- [ ] web setup, blind phase, bidding, trick play, team scoring, bags, replay, rules, observer, accessibility, and outcome smoke all pass;
- [ ] native simulations and benchmarks produce seat-keyed and team-keyed results with fixed seeds and no nontermination;
- [ ] the `forward-v1` audit, register freshness, prior-game disposition, and CI receipt are complete and passing;
- [ ] primitive-pressure records show trick-helper reuse, numeric-contract second-use keep-local, and partnership first-use local-only;
- [ ] every official-game document is complete or uses a reasoned `not applicable`; human IP/release review is recorded;
- [ ] all per-gate debt lanes in §3.7 are closed or explicitly blocked; no unnamed debt remains.

### 6.3 Official-game acceptance checklist

| Contract area | Exit requirement |
| --- | --- |
| Requirements-first workflow | Source notes, formal rules, coverage, mechanics, forward audit, admission, and initialized evidence receipt predate serious code. |
| Originality/IP | Rules/help/UI copy and assets are original; “Spades” is only a family label; no copied score sheet, icon set, screenshot, font, or trade dress; human review complete. |
| Rust authority | Legal actions, validation, previews, effects, visibility, score/outcome, replay, and bots originate in Rust. |
| Rule coverage | Every stable rule ID has implementation and evidence; no implementation behavior lacks a rule or diagnostic owner. |
| Trace set | Required success, edge, invalid, bot, viewer, export, WASM, and terminal traces exist under Trace Schema v1. |
| Viewer safety | Public/seat views and exports obey authorization on every named surface; partner-private leakage is impossible. |
| Replay/hash | Fixed seeds replay byte-for-byte under current versions; no unauthorized migration or broad golden rewrite. |
| Bots | L0/L1 policies are legal, deterministic, bounded, explainable, viewer-safe, and free of forbidden methods. |
| UI | Public browser renderer, public rules, replay, outcome explanation, accessibility, and dedicated smoke are complete. |
| Performance | Native/browser workload reports meet adopted floors or record an honest blocker; no budget is waived silently. |
| Primitive/scaffolding | Atlas, game ledger, register, evidence receipt, CI receipt, and any follow-on unit agree. |
| Release | Catalog/docs/checkers/status are synchronized; public-release checklist and human sign-off complete. |

## 7. Acceptance evidence

### 7.1 Required command suite

The exact CLI spelling must be revalidated against the live tree when tasks are authored. Equivalent updated commands are acceptable only when they preserve the stated evidence intent; removing an evidence lane is not.

```bash
cargo fmt --all --check
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo check --workspace

cargo test -p game-stdlib
cargo test -p blackglass_pact
cargo test -p blackglass_pact --test rules
cargo test -p blackglass_pact --test property
cargo test -p blackglass_pact --test replay
cargo test -p blackglass_pact --test serialization
cargo test -p blackglass_pact --test visibility
cargo test -p blackglass_pact --test bots
cargo test -p wasm-api
cargo test --workspace

cargo run -p fixture-check -- --game blackglass_pact
cargo run -p rule-coverage -- --game blackglass_pact
cargo run -p replay-check -- --game blackglass_pact
cargo run -p replay-check -- --game blackglass_pact --all
cargo run -p simulate -- \
  --game blackglass_pact \
  --seat-count 4 \
  --games 1000 \
  --start-seed 180400 \
  --action-cap 4096

cargo bench -p game-stdlib
cargo bench -p blackglass_pact

bash scripts/boundary-check.sh
node scripts/check-scaffolding-governance.mjs
node --test scripts/check-scaffolding-governance.test.mjs
node scripts/check-doc-links.mjs
node scripts/check-player-rules.mjs
node scripts/check-catalog-docs.mjs
node scripts/check-ci-games.mjs
node scripts/check-outcome-explanations.mjs
node scripts/check-presentation-copy.mjs

npm --prefix apps/web run build
npm --prefix apps/web run smoke:wasm
npm --prefix apps/web run smoke:ui
npm --prefix apps/web run smoke:effects
npm --prefix apps/web run smoke:preview
npm --prefix apps/web run smoke:e2e
```

The closeout evidence must also record:

- one deterministic all-L0 corpus and one deterministic L1-bearing corpus, even if the current simulator CLI exposes that distinction through compiled policy assignment rather than a command flag;
- the exact seed range, action cap, build profile, toolchain, hardware, and failure-report path;
- public-observer export plus all four seat-private exports;
- any intentionally scoped API snapshot additions;
- the final `forward-v1` checker summary and receipt path.

### 7.2 Test taxonomy

| Test class | Mandatory Blackglass Pact coverage | Minimum evidence owner |
| --- | --- | --- |
| Named rule tests | Setup, teams, blind eligibility/timing, deal, bid order/vocabulary, spades breaking, follow suit, trump resolution, scoring, bags, target, tie continuation | `tests/rules.rs` + rule coverage |
| Action-tree/validator equivalence | Every emitted leaf validates; every accepted action was emitted for the same fresh state/view; invalid paths return stable Rust diagnostics | Rules/actions tests and traces |
| Unit arithmetic | Team aggregation, ordinary-trick attribution, nil/blind deltas, repeated bag thresholds, negative scores, score/bag storage separation | `scoring.rs` unit tests |
| Property/invariant | Card conservation/uniqueness, exactly four plays per trick, 13 tricks per hand, legal follow-suit, helper/local comparator agreement, team partition, score equations, deterministic target logic | `tests/property.rs` |
| Setup/RNG | Same seed/version produces same deal; blind declare/decline choices do not alter deal; dealer/hand index separation; no pre-deal card projection | Setup tests and traces |
| Serialization/hash | Stable seat/team/card ordering; same state gives same bytes/hash; replay/export versions explicit; unordered maps excluded or canonically sorted | `tests/serialization.rs` |
| Replay determinism | Command replay reproduces terminal state/effects/hash; every named trace validates; invalid commands replay to stable diagnostics | `tests/replay.rs`, replay-check |
| Public/seat visibility | Observer and four seat views; 12 ordered pairwise checks; partner hand protected; blind phase no-card proof; effects/diagnostics/previews safe | `tests/visibility.rs` |
| Export/import | Public and every seat-private export round-trip; imported viewer history cannot grant another viewer's authorization | replay/WASM tests |
| Bot legality | L0/L1 blind, bid, and play actions always derive from legal leaves; deterministic priorities/ties; no hidden data in input/explanation/candidates | `tests/bots.rs` |
| Tool integration | Fixture, rule-coverage, replay, simulation, and benchmark tools recognize canonical ID and produce stable seat/team summaries | tool commands |
| WASM/API | Catalog, setup, actions, views, effects, replay, exports, bot dispatch, terminal outcome, and bounded snapshot additions | `wasm-api` tests |
| Browser/e2e | Rules, setup, blind phase, bidding, play, score/bags, observer, hotseat, replay, a11y, reduced motion, terminal outcome, no-leak DOM/storage/log inspection | dedicated smoke + shared smokes |
| Governance/docs | Boundary, game registry, catalog docs, rules copy, outcome, presentation copy, doc links, scaffolding audit, receipt/register consistency | repository scripts |
| Benchmarks | Action generation, views, exports, scoring, bots, full match; variance-aware thresholds and no unexamined regression | Criterion/browser report |

All failing tests follow `docs/AGENT-DISCIPLINE.md`: determine whether the expectation is still valid, determine whether the defect is in the system under test or the test, then fix the responsible side. Test deletion, weakening, blanket ignore, or unrelated golden regeneration is forbidden.

### 7.3 Viewer matrix

| Viewer class | Public facts | Authorized private facts | Legal actions/previews | Effects/diagnostics | Browser replay export |
| --- | --- | --- | --- | --- | --- |
| Internal full trace/test authority | Full state, RNG inputs as authorized by internal profile, every hand, command/effect state | Internal only; never browser payload | Full test harness only | Full internal evidence as profile permits | `not applicable`: internal trace is not a browser viewer export |
| Public observer | Seats/teams, dealer, phase, blind eligibility/decisions, accepted bids, played cards, trick results, scores, bags, outcome | None | No seat-private legal tree; observer-only controls such as replay navigation where supported | Public-safe semantic effects/diagnostics only | `public-export-v1` |
| `seat_0` / North | All public facts | North's unplayed hand and North's own private legal controls | North-only tree/previews when active | Public effects plus North-authorized private deal/control facts | `seat-private-export-v1` for `seat_0` |
| `seat_1` / East | All public facts | East's unplayed hand and controls | East-only | Same rule | `seat-private-export-v1` for `seat_1` |
| `seat_2` / South | All public facts | South's unplayed hand and controls | South-only | Same rule | `seat-private-export-v1` for `seat_2` |
| `seat_3` / West | All public facts | West's unplayed hand and controls | West-only | Same rule | `seat-private-export-v1` for `seat_3` |
| `team_0` private viewer | `not applicable`: no hidden team-shared fact exists | None beyond public facts; North and South remain separate private viewers | `not applicable` | `not applicable` | `not applicable` |
| `team_1` private viewer | `not applicable`: same rationale | None | `not applicable` | `not applicable` | `not applicable` |

A partnership badge, team panel, or team ID is public grouping, not authorization. There is no payload mode that merges partner hands.

### 7.4 Pairwise no-leak and export-coverage matrix

Every row below is mandatory. “Protected datum” includes the source seat's unplayed card IDs/ranks/suits, card order, private legal leaves, private previews, private candidate ranking, and any explanation derived from those facts.

| Source private seat | Unauthorized viewer seat | Relationship | Required assertion |
| --- | --- | --- | --- |
| `seat_0` | `seat_1` | Opponent | No `seat_0` protected datum occurs in `seat_1` view/action/effect/diagnostic/export/browser surfaces |
| `seat_0` | `seat_2` | **Partner** | Same assertion; partnership grants no hand visibility |
| `seat_0` | `seat_3` | Opponent | Same assertion |
| `seat_1` | `seat_0` | Opponent | Same assertion |
| `seat_1` | `seat_2` | Opponent | Same assertion |
| `seat_1` | `seat_3` | **Partner** | Same assertion |
| `seat_2` | `seat_0` | **Partner** | Same assertion |
| `seat_2` | `seat_1` | Opponent | Same assertion |
| `seat_2` | `seat_3` | Opponent | Same assertion |
| `seat_3` | `seat_0` | Opponent | Same assertion |
| `seat_3` | `seat_1` | **Partner** | Same assertion |
| `seat_3` | `seat_2` | Opponent | Same assertion |

Additional mandatory edges:

- every source seat → public observer;
- future shuffled deck → observer and all four seat viewers during blind commitment;
- every non-active seat's private legal tree/preview → active and non-active unauthorized viewers;
- internal bot candidate state → every browser viewer except explicitly safe explanation fields;
- one seat-private export → every other seat's import/viewer mode.

TESTING §8.2 is satisfied by **exhaustive**, not sampled, coverage: CI must exercise the public observer and all four seat-private export/import variants. Browser e2e may use a bounded visual subset, but Rust/WASM export tests exercise every seat viewer.

### 7.5 Mandatory no-leak datum and surface taxonomy

| Surface | Protected content and required proof |
| --- | --- |
| Pre-deal blind action tree | Contains only declare/decline and public score/eligibility context; no card, suit, rank, hand strength, deck index, RNG sample, or future-card-derived recommendation |
| Pre-deal bot input/candidates | Same restriction; L0 may randomly choose legal leaves from seeded bot RNG, and L1 may use public deficit/score only |
| Deal effects | Public effect gives counts/phase/dealer only; each private hand effect reaches only its owner if the architecture exposes private deal effects at all |
| Views/payloads | Owner sees own unplayed hand; partner/opponents/observer see only counts/backs/public plays |
| Action trees | A viewer never receives another seat's playable card IDs or private control state; inactive viewers receive no speculative actor tree |
| Previews | No preview reveals a future card, partner card, opponent card, or hidden candidate; safe preview fields are Rust-authored and viewer-scoped |
| Diagnostics/disabled reasons | Do not confirm another hand's suit contents. Public rejection text names the submitted rule violation without exposing hidden alternatives |
| Semantic effects/effect log | No private hand identity in public/team effects. Team score effects are public and keyed by stable team ID |
| Command/dev logs | No hidden cards, full internal state, seeds, private candidates, or imported private exports in public builds/logs |
| Bot explanations | Explain public/own-hand features only; never claim partner/opponent holdings, stock/deck contents, or sampled probabilities |
| Candidate rankings | Remain native/authorized; public UI receives only safe bounded fields if an accepted policy exposes them |
| Replay export/import | Viewer-scoped observation history only; no full-state serialization masquerading as public export; import cannot elevate viewer authorization |
| DOM/React props/state | Unauthorized cards never exist hidden by CSS, offscreen, in data attributes, keys, alt text, labels, comments, hydration payload, or stale component state |
| Test IDs/accessibility tree | No card identity or private inference in IDs, names, descriptions, live regions, labels, or hidden semantic nodes |
| Storage/clipboard/download | No private state in local/session storage, cache, clipboard helper, analytics, crash report, or default exported filename/body |
| Animation queues | A removed or face-down card animation cannot retain/reveal private identity to another viewer after handoff |
| Screenshots/video | Public-release capture uses observer or an expressly authorized seat and is reviewed for accidental private data |
| Source maps/dev inspector | Public build exposes no secret dev-panel command or full-state payload outside the existing whitelist/boundary |

### 7.6 Golden-trace minimum set

The following is a floor, not a cap. Trace names may be normalized to repository convention, but each scenario remains individually discoverable and mapped to rule IDs.

#### Setup, partnership, blind commitment, and deal

1. `setup-fixed-four-and-team-pairs.trace.json`
2. `invalid-seat-count-below-four.trace.json`
3. `invalid-seat-count-above-four.trace.json`
4. `stable-team-id-and-seat-order.trace.json`
5. `first-hand-no-blind-eligibility-at-tie.trace.json`
6. `blind-nil-eligibility-at-100-deficit.trace.json`
7. `blind-nil-ineligible-at-99-deficit.trace.json`
8. `blind-nil-declare-before-deal-no-card-surface.trace.json`
9. `blind-nil-decline-before-deal.trace.json`
10. `both-partners-blind-nil-independent.trace.json`
11. `blind-nil-decision-public-and-immutable.trace.json`
12. `deterministic-full-deal-thirteen-each.trace.json`
13. `deal-identical-after-blind-declare-vs-decline.trace.json`
14. `dealer-rotation-after-nonterminal-hand.trace.json`

#### Bidding

15. `bidding-left-of-dealer-through-dealer.trace.json`
16. `blind-nil-seat-skipped-in-bidding.trace.json`
17. `ordinary-nil-bid-accepted.trace.json`
18. `numeric-bid-one-accepted.trace.json`
19. `numeric-bid-thirteen-accepted.trace.json`
20. `numeric-zero-not-a-legal-leaf.trace.json`
21. `bid-out-of-range-diagnostic.trace.json`
22. `accepted-bid-immutable.trace.json`
23. `public-sequential-bid-projection.trace.json`
24. `team-contract-sums-only-positive-numeric-bids.trace.json`

#### Trick play and helper conformance

25. `first-lead-left-of-dealer.trace.json`
26. `spade-lead-blocked-before-broken.trace.json`
27. `only-spades-lead-exception-breaks-spades.trace.json`
28. `off-suit-spade-breaks-spades.trace.json`
29. `follow-suit-forced.trace.json`
30. `void-seat-may-discard-or-trump.trace.json`
31. `highest-spade-wins.trace.json`
32. `highest-led-suit-wins-without-spade.trace.json`
33. `off-suit-nonspade-never-wins.trace.json`
34. `trick-winner-leads-next.trace.json`
35. `promoted-follow-suit-helper-conformance.trace.json`
36. `promoted-winner-index-helper-conformance.trace.json`

#### Scoring, bags, and terminal outcome

37. `ordinary-contract-made-exact.trace.json`
38. `ordinary-contract-made-with-bags.trace.json`
39. `ordinary-contract-set-minus-ten-times-bid.trace.json`
40. `nil-success-plus-100.trace.json`
41. `nil-failure-minus-100-no-help-to-partner.trace.json`
42. `blind-nil-success-plus-200.trace.json`
43. `blind-nil-failure-minus-200.trace.json`
44. `failed-nil-tricks-bag-even-when-team-set.trace.json`
45. `ten-bag-penalty-and-rollover.trace.json`
46. `multiple-bag-thresholds-one-hand.trace.json`
47. `bags-carry-across-set-and-nil.trace.json`
48. `public-team-hand-score-breakdown.trace.json`
49. `one-team-crosses-500-unique-higher.trace.json`
50. `both-teams-cross-500-unique-higher.trace.json`
51. `exact-tie-at-or-above-500-continues.trace.json`
52. `terminal-team-and-seat-standings.trace.json`

#### Invalid, bot, viewer, export, WASM, and full-match evidence

53. `invalid-unowned-card-diagnostic.trace.json`
54. `invalid-must-follow-suit-diagnostic.trace.json`
55. `invalid-spade-lead-diagnostic.trace.json`
56. `invalid-wrong-seat-diagnostic.trace.json`
57. `invalid-wrong-phase-diagnostic.trace.json`
58. `invalid-stale-command-diagnostic.trace.json`
59. `l0-blind-bid-and-play.trace.json`
60. `l1-partnership-bid-nil-and-play.trace.json`
61. `public-observer-no-leak.trace.json`
62. `seat-private-pairwise-no-leak-all-four.trace.json`
63. `blind-phase-no-future-card-leak.trace.json`
64. `public-replay-export-import.trace.json`
65. `seat-private-replay-export-import-seat-0.trace.json`
66. `seat-private-replay-export-import-seat-1.trace.json`
67. `seat-private-replay-export-import-seat-2.trace.json`
68. `seat-private-replay-export-import-seat-3.trace.json`
69. `wasm-exported-partnership-terminal.trace.json`
70. `mixed-l0-l1-full-match.trace.json`

At least one trace must begin from a negative score, at least one must cross a bag threshold while the team is set on its ordinary contract, and at least one must prove that an exact target tie can later fall below 500 and still continue until the terminal predicate is met again.

### 7.7 Evidence Fixture Contract profiles and completion receipt

Blackglass Pact defines the gate-specific completion profile:

```text
n-seat-hidden-information-release-candidate
```

It is a narrower label permitted by the template and means all obligations of `n-seat`, `hidden-information`, and `release-candidate` apply simultaneously. It waives nothing. `GAME-EVIDENCE.md` must include these rows:

| Profile ID | Version | Visibility | Validator owner | Gate 18 status expectation |
| --- | --- | --- | --- | --- |
| `replay-command-v1` | `v1` | internal-dev/public as authorized | `fixture-check` / `replay-check` | Required; all named command traces and stable diagnostics |
| `public-export-v1` | `v1` | public | Rust/WASM export/import | Required; observer round-trip and no-leak |
| `seat-private-export-v1` | `v1` | seat-private | Rust/WASM plus pairwise harness | Required for `seat_0` through `seat_3`; no sampled viewer exemption |
| `setup-evidence-v1` | `v1` | public/viewer-scoped/seat-private/internal-dev | fixture/static-data validator | Required for fixed teams, blind phase, dealer, deterministic full deal, and invalid seat counts |
| `domain-evidence-v1` | `v1` | public/viewer-scoped/seat-private/internal-dev | game-local validator | Required for contracts, nils, play, bags, scoring, outcome, bots, UI-safe projections |

The receipt must also state:

- canonical byte authority for each profile;
- game/rules/data/profile versions;
- public versus seat-private artifact links;
- every `not applicable` reason, including team-private viewer and L3;
- replay/hash migration note `none` unless an accepted ADR authorizes otherwise;
- exact forward-audit register/CI links;
- human IP review status and any release blocker.

### 7.8 Surface and action-fanout budgets

| Surface | Normative/provisional budget |
| --- | --- |
| Seats / teams | Exactly 4 seats / 2 teams |
| Cards | 52 unique cards; 13 owner-private cards per seat after deal; no undealt tail after deal |
| Pre-deal hidden future state | Up to all 52 card identities internal; zero card identities projected to browser viewers/bots during blind commitment |
| Current trick | 0–4 ordered public plays |
| Completed hand history | 13 tricks / 52 public played cards; retain enough typed history for replay/outcome without duplicating private hands |
| Blind legal leaves | Maximum 2 for an eligible active seat; 0 for other viewers |
| Bid legal leaves | Maximum 14 (`nil` plus `1..=13`) for active non-blind seat |
| Play legal leaves | Maximum 13 at start of play, bounded by owner hand and follow-suit policy |
| Action-tree depth | Target maximum 2 below operation group; no deep DSL-shaped tree |
| Viewer variants | 1 observer + 4 seat viewers; 0 team-private viewers |
| Team summary regions | Exactly 2 stable team groups |
| Seat summary regions | Exactly 4 stable seat groups |
| Normal action effect count | Target `<= 8`; semantic grouping preferred over low-level mutation spam |
| Hand-resolution effect count | Target `<= 24`, or one bounded public score batch plus stable per-team/per-seat components; profiling decides exact representation without hiding meaning |
| Public bid history | 4 accepted bid records per hand, including blind-nil records |
| Score history | Potentially unbounded until a unique 500+ leader; UI uses bounded rendering/scrolling but replay retains complete deterministic history |
| Simulator action cap | Provisional 4096 commands per match; cap breach is a failure report, not a draw rule |
| Browser interaction | Target under 100 ms for blind/bid/card control response on supported hardware, excluding deliberate animation |
| Native action/view/scoring | Provisional p95 under 1 ms per operation in release benchmark workloads |
| Viewer export | Provisional p95 under 50 ms for the largest supported match export/import workload |

A long tie/negative-score match means match length is not mathematically fixed. Tests and simulator must use deterministic action caps and report cap breaches; the game rules must not silently introduce a hand cap.

### 7.9 Benchmark and simulation expectations

| Workload | Required variants/measurements |
| --- | --- |
| Setup/blind/deal | no-eligibility and eligible blind phases; declare/decline RNG independence; full 52-card deal; four seat projections |
| Blind legal tree | eligible versus ineligible; L0 and L1 choice; public effect filtering |
| Bid tree | 14-leaf maximum, blind-seat skip, public projection, contract aggregation |
| Card tree | opening 13-card unconstrained lead, broken-spades restriction, forced follow suit, void discard/trump, late-hand small tree |
| Promoted helpers | `follow_suit_indices` and `winning_play_index` direct/conformance workloads; no regression from Gate 17 baseline without investigation |
| Trick transition | four-play resolution, spades break, winner-leads, public effects |
| Hand scoring | made/set, nil/blind, single and multiple bag thresholds, negative scores, tie continuation |
| Views | observer and each of four seats at blind, bidding, mid-trick, score, and terminal states |
| Replay/export | internal command replay, observer export, all four seat-private exports/imports, terminal match history |
| Bots | L0 blind/bid/play; L1 bid/nil/play; explanation generation; deterministic tie-break cost |
| Full match | all-L0 and L1-bearing fixed-seed corpora; action count, hands, scores, bags, terminal rate, cap breaches, throughput |
| Browser | WASM action/view/export, grouped render, 13-card hand controls, score history, replay seek, reduced motion |

Provisional performance posture:

- native p95 action generation, validation, projection, trick resolution, and score calculation: `< 1 ms` per operation in release mode;
- largest viewer export/import: `< 50 ms` p95 on the benchmark host;
- browser control-to-safe-feedback: `< 100 ms` p95 excluding intentional animation;
- release simulation throughput target: at least 75 completed matches/second on the calibrated benchmark host, or a replacement floor derived through the accepted variance-aware benchmark process with the original target and reason preserved in `BENCHMARKS.md`;
- zero unexplained cap breaches in the required 1,000-match corpus.

These are provisional budgets under TESTING. They may be calibrated through the accepted benchmark ADR process; they may not be silently deleted because the full partnership game is slower than a sibling.

### 7.10 `forward-v1` acceptance evidence

Gate 1 acceptance requires all of the following to agree:

1. `games/blackglass_pact/docs/MECHANICS.md` pre-code C-01…C-10 audit;
2. `games/blackglass_pact/docs/GAME-IMPLEMENTATION-ADMISSION.md` admission state;
3. `games/blackglass_pact/docs/GAME-EVIDENCE.md` pre-code and post-build status links;
4. `games/blackglass_pact/docs/PRIMITIVE-PRESSURE-LEDGER.md` behavioral-only decisions;
5. `docs/MECHANICAL-SCAFFOLDING-REGISTER.md` reuse/new/disposition evidence;
6. any bounded follow-on row in `specs/README.md`, when a prior matching game actually requires work;
7. `ci/scaffolding-audits.json` `forward-v1` record;
8. `node scripts/check-scaffolding-governance.mjs` success;
9. checker test-suite success;
10. no unauthorized hash, visibility, or determinism migration.

A receipt that passes schema validation but disagrees with code, register, or evidence is a failure.

## 8. FOUNDATIONS & boundary alignment

### 8.1 Contract alignment

| Governing principle/contract | Gate 18 alignment |
| --- | --- |
| FOUNDATIONS product priority | Blackglass Pact is a public, complete, replayable official game, not speculative engine research. Foundation reuse is preferred, but public correctness and legibility remain the goal. |
| Rust owns behavior | Blind eligibility, bidding, card legality, scoring, views, previews, effects, bot choices, replay, and outcome are Rust-authored. TypeScript presents typed payloads only. |
| Generic noun-free kernel | No domain noun enters `engine-core`; all Spades/team/nil/bag types stay game-local. |
| Game/data boundary | Static files hold typed content, versions, fixture records, and presentation metadata only. All conditions and formulas remain Rust behavior. |
| Determinism/replay | Fixed versions, canonical seat/team order, RNG independence, stable serialization, Trace Schema v1, and ADR 0009 migration discipline apply. |
| Hidden-information safety | Owner-private hands, pre-deal future cards, legal trees, previews, bots, effects, exports, browser state, and logs follow the exhaustive viewer matrix. |
| Multi-seat/team law | Seat range is fixed 4; team declaration is fixed/public; team IDs are stable; partner-private facts remain private; outcomes contain team and seat arrays. |
| Official-game contract | Sources → rules → coverage → mechanics/audit → admission → evidence → implementation; original public rules, trace set, UI, tools, benchmarks, and release evidence are all scoped. |
| AI law | L0 and bounded L1 only; any L2 remains behind a strategy-evidence gate; L3 and sampled/search/learning methods are excluded. |
| UI law | Legal-only controls, Rust-safe previews, semantic effects, replay, keyboard/accessibility, reduced motion, and no hidden-state DOM are mandatory. |
| Mechanic atlas | Promoted trick helper reused; numeric contract reviewed at second use and kept local; partnership/team first use recorded local-only. |
| Scaffolding register / ADR 0008 | First `forward-v1` audit, register-new, queue-or-dispose, and Gate 1 receipt are mandatory. |
| IP policy | Neutral product identity, original prose/assets, no casino/trade-dress mimicry, and human review. |
| Agent discipline | Bounded tasks, explicit forbidden changes, valid-test/SUT diagnosis, and no drive-by refactors. |

### 8.2 Promoted trick-taking helper reuse confirmation

The exact promoted API at the target commit is:[^R16]

```rust
pub fn follow_suit_indices<T, S>(
    held: &[T],
    led_suit: S,
    suit_of: impl Fn(&T) -> S,
) -> Vec<usize>
where
    S: Copy + Eq;

pub fn winning_play_index<T, S, R>(
    plays: &[T],
    led_suit: S,
    trump: Option<S>,
    suit_of: impl Fn(&T) -> S,
    rank_of: impl Fn(&T) -> R,
) -> Option<usize>
where
    S: Copy + Eq,
    R: Copy + Ord;
```

Normative integration:

```rust
let legal_indices = game_stdlib::trick_taking::follow_suit_indices(
    hand,
    led_suit,
    |card| card.suit,
);

let winner = game_stdlib::trick_taking::winning_play_index(
    plays,
    led_suit,
    Some(Suit::Spades),
    |play| play.card.suit,
    |play| play.card.rank,
);
```

The snippets illustrate the type boundary, not a complete implementation. Game-local code still owns:

- whether the actor is active and owns the card;
- whether a lead is legal before spades are broken;
- stable card/action diagnostics;
- trick state and seat mapping;
- mutation after the winning index is known;
- semantic effects and projections;
- bidding, partnerships, nil, bags, scoring, dealer rotation, terminal state, bots, and UI.

Compatibility proof must compare the helper result with an independent test oracle over exhaustive or strongly sampled four-play suit/rank combinations, including no-spade, one-spade, multiple-spade, tied-impossible-by-unique-card, off-suit, and led-spade cases. The helper itself is not changed merely to make a Blackglass Pact test convenient.

**Primitive decision:** `promoted primitive — reuse unchanged`.

**Atlas/ledger effect:** add a Gate 18 reuse/conformance note to the existing follow-suit and led-suit comparator rows. No new §10A debt and no third-use hard gate.

### 8.3 Numeric trick-contract second-use comparison

| Comparison dimension | Vow Tide first use | Blackglass Pact second use | Shared-helper conclusion |
| --- | --- | --- | --- |
| Seat model | Independent 3–7 seats | Fixed 4 seats grouped into two teams | Different aggregation/identity pressure |
| Bid timing | Post-deal public sequence | Optional pre-deal blind commitment plus post-deal public sequence | Blind commitment is a distinct hidden-info phase |
| Bid leaves | `0..=hand_size` | `nil` plus `1..=13`; blind nil declared separately | Only superficial numeric overlap |
| Last-bidder policy | Dealer hook forbids one total | No total-13 hook | Hook cannot be generalized from one game |
| Contract subject | Each seat's own exact trick count | Team ordinary minimum plus individual zero-trick contracts | Different subject and success predicate |
| Trick attribution | Every seat's tricks evaluate that seat | Failed nil tricks excluded from ordinary contract and converted to bags | Game-specific attribution law |
| Scoring | Exact hit scores local fixed amount; miss scores zero | ±10× ordinary contract, +1 bags, nil ±100, blind ±200, threshold penalties | No stable narrow formula boundary |
| Persistence | Multi-hand cumulative seat score | Persistent team score and separate bag remainder | Different state lifetime and outcome |
| Terminal/outcome | Fixed schedule, seat-keyed standings/co-winners | Point target, tie continuation, team-keyed winner plus seat breakdown | Different terminal policy |
| Visibility/bots | No partner | Partner's public bid/play affects policy, but hand remains private | Partnership-safe reasoning is local behavior |

**Decision:** `repeated-shape candidate — keep local`. The shared shape is too small (“a public integer-like commitment exists”) and the behavior-bearing differences are the substance of both games. Promotion at second use is forbidden by this spec.

**Next review trigger:** the next official game that implements a close numeric trick contract must compare all three games before proposing any shared boundary. A proposal remains limited to behavior-free typing/transport only unless the atlas hard gate justifies a narrow behavioral primitive.

**Required repo record:** update the atlas row `numeric trick bid / contract-vs-result / last-bidder hook` with Gate 18's second-use review, differences, keep-local decision, and next trigger. Update both Vow Tide and Blackglass Pact ledgers only where the current ledger convention requires cross-reference; do not rewrite shipped behavior.

### 8.4 Partnership/team first-use local stance

The new atlas/ledger row must carry substantially this information:

| Field | Gate 18 entry |
| --- | --- |
| Shape | Fixed competitive partnership grouping with team-aggregated ordinary trick contract, individual nil/blind-nil contracts, team score/bags, partnership visibility, and team-keyed terminal outcome |
| First official use | `blackglass_pact` / Gate 18 |
| Classification | `local-only` first use |
| Why local | Team pairing, bid attribution, nil interaction, bags, visibility, score explanation, and terminal policy are rules-family behavior; one use provides no generalization evidence |
| Stable identity | Game-local `TeamId`; stable `team_0`/`team_1`; seats remain independent `SeatId`s |
| Visibility law | Team membership is public; partner hand and controls are not shared; no team-private viewer in the locked variant |
| Outcome law | Public `standings_by_team` plus `standings_by_seat`, stable arrays and explanatory fields |
| Shared-outcome comparison | Not the existing all-win/all-lose cooperative terminal shape; no promotion or row merger |
| Next review trigger | Another official competitive-team game with shared scoring/outcome or partnership-private information |
| Prohibited extraction | No generic team engine, no generic bid aggregator, no shared nil/bag scorer, no generic partner bot, no team identity in `engine-core` |

The game may use generic seat containers and transport primitives, but team behavior is local. A UI component that groups seats is not evidence that the scoring/visibility mechanic is generic.

### 8.5 `forward-v1` mechanical-scaffolding reuse-first audit

The preimplementation audit in `games/blackglass_pact/docs/MECHANICS.md` must reproduce or link a completed version of this table and refine it against the actual implementation plan.

| Register entry | Existing shape reviewed | Gate 18 design-time decision | Boundary rationale | Register-new / prior-game posture | Compatibility expectation |
| --- | --- | --- | --- | --- | --- |
| `MSC-8C-001` / C-01 | Shared semantic effect-envelope construction | **Reuse** the registered/shared constructors and established effect shape; do not hand-roll a parallel envelope literal | Envelope mechanics are behavior-free; effect meanings such as `BlindNilDeclared` and `BagPenaltyApplied` remain game-local | No new entry expected. If a local literal appears, replace it or record a checker-supported exception; no prior retrofit expected | Hash/visibility/determinism unchanged |
| `MSC-8C-002` / C-02 | Canonical seat grammar and import aliases | **Reuse** canonical stable seat grammar for `seat_0..seat_3`; add game-local stable team IDs without changing seat grammar | Seat serialization/presentation plumbing is reusable; partnership membership is behavior and stays local | No new entry expected. A generic team grammar is explicitly not inferred from first use | Unchanged |
| `MSC-8C-003` / C-03 | Seat-count validation and clockwise/ring arithmetic | **Reuse** shared fixed-seat/ring scaffolding where signatures fit; game-local constants pin `{4}` and opposite pairing | Ring arithmetic is mechanical; dealer, bid order, leader, and partner rules are local | No new entry expected; do not copy prior fixed-four arithmetic | Unchanged |
| `MSC-8C-004` / C-04 | Action-tree v1 framing and hashing | **Reuse** canonical operation-group/leaf framing for blind, bid, and play actions | Framing/hash is behavior-free; leaf meaning/legality stays Rust game-local | No new entry expected. Any new framing is a stop condition unless registered/migrated | No hash migration |
| `MSC-8C-005` / C-05 | Stable-byte writer v1 | **Reuse** canonical byte-writing scaffolding for versions, state, seats, teams, bids, cards, scores, and outcomes | Byte encoding mechanics are shared; field selection/order is game-owned and versioned | No local writer. Any required byte change routes through ADR 0009 | No migration expected |
| `MSC-8C-006` / C-06 | Dev-only `game-test-support` helpers | **Reuse** only from dev/test dependencies for no-leak/profile/property support; never add production dependency | Test scaffolding is lawful in dev only; runtime behavior remains in the game crate | No new entry expected; checker must show no production support edge | Runtime graph unchanged |
| `MSC-8C-007` / C-07 | Pairwise no-leak matrix geometry | **Reuse** the shared ordered-pair harness for 4 seats and extend only data extractors/assertions local to Blackglass Pact | Pair enumeration is mechanical; protected datum and viewer authorization are game behavior | No new entry expected. Do not copy 12 loops by hand | Visibility semantics unchanged; evidence expands |
| `MSC-8C-008` / C-08 | Evidence-profile drivers | **Reuse** canonical profile/fixture/replay drivers for all five required profile IDs and four viewer exports | Driver mechanics are reusable; profile contents and expected artifacts are game-local | No new entry expected; generic tool registration only as needed | Hash/export versions unchanged |
| `MSC-8C-009` / C-09 | Versioned bounded-index sampling | **Reuse when property/benchmark sampling needs it**; otherwise record `not applicable` because deterministic exhaustive four-seat viewer coverage is used | Sampling mechanics may be reusable, but mandatory viewer coverage is exhaustive and may not be replaced by sampling | No new sampler. N/A must state which tests are exhaustive and which use the registered sampler | Determinism unchanged |
| `MSC-8C-010` / C-10 | Non-promotion behavioral bundle / route-back guard | **Apply** as a negative boundary: team, bid, nil, bags, scoring, visibility, bot, effect meaning, and renderer policy stay in behavioral owners | Prevents mechanical-scaffolding work from becoming a hidden game framework | No generic team/scoring/helper entry. New behavior-free presentation plumbing, if actually created, must be registered separately | No migration |

#### Lawful shared-home review

| Potential reusable shape | Lawful-home decision |
| --- | --- |
| Trick legality/resolution | Existing `game-stdlib::trick_taking` promoted helpers; reuse unchanged |
| Stable seats/ring | Existing shared scaffolding per register; reuse |
| Pairwise no-leak/evidence profiles | `game-test-support` dev-only; reuse without production edge |
| WASM operation framing/catalog transport | Existing `wasm-api` shared transport; add one adapter, not a game rule in transport |
| Seat frame/action controls/effect/replay presentation | Existing `apps/web` shared presentation scaffolding where it fits; Blackglass-specific grouped layout remains local until evidence shows a behavior-free reusable shape |
| Team-group presentation frame | **Watch item, not preapproved extraction.** Prefer composition of existing `SeatFrame` and semantic markup. If implementation creates a genuinely reusable behavior-free `TeamFrame`/group scaffold, register it on first use as candidate/local/rejected and identify prior matches |
| Score/bag computation | Game-local behavior; no shared home selected |
| Blind/nil action protocol | Game-local behavior; no shared home selected |

#### Register-new rule

At post-build review, inspect every new non-domain utility, wrapper, serializer, test driver, WASM adapter helper, and presentation frame. A newly invented behavior-free shape cannot be omitted because it is “small.” Before gate close it must have:

- an MSC ID and register entry;
- owning lawful home or accepted local/rejected state;
- semantic responsibility and explicit behavior exclusions;
- hash/visibility/determinism impact;
- first-use evidence;
- next review trigger;
- prior matching sites.

First use never authorizes promotion.

#### Queue-or-dispose rule

When Gate 18 reveals a matching earlier site, closeout must choose exactly one:

1. queue a named bounded active-epoch tracker unit with scope, owner, dependency, evidence, and migration posture; or
2. record an accepted `local-only`, `deferred`, or `rejected` register disposition with rationale, owner, evidence, and next review trigger, and encode the checker's accepted no-unit decision in the receipt.

“Follow up later,” an issue comment, or a TODO is not a disposition. The design-time expectation is no prior refactor, but only the completed audit can establish that result.

### 8.6 Architecture and data-boundary allocation

| Concern | Owning layer | Forbidden alternative |
| --- | --- | --- |
| Generic action/replay/RNG interfaces | Existing `engine-core` abstractions only | Adding card/team/bid nouns or Spades policy to kernel |
| Pure promoted trick selection/comparison | `game-stdlib::trick_taking` | Duplicating locally or widening with partnership/broken-spades rules |
| All Blackglass game nouns and behavior | `games/blackglass_pact` | Static formula tables, WASM/TS rules, generic team framework |
| Dev/test geometry/profile support | `game-test-support` dev dependency | Production runtime dependency or hidden test-only behavior |
| Browser bridge/transport | `wasm-api` | Recomputing legality/score or retaining full hidden state |
| Presentation | `apps/web` shared scaffolding + Blackglass-specific renderer | Hidden cards in DOM, client score math, color-only team identity |
| Typed metadata/fixtures | game `data/**` | YAML/DSL or executable selectors/conditions/triggers |
| Evidence/bench/tools | existing generic tools plus minimal registration | Per-game duplicate tool framework |

### 8.7 Determinism, replay, and hash posture

- Match seed, rules version, data version, hand index, and canonical algorithm version determine shuffle/deal.
- Blind declare/decline commands cannot perturb game deal RNG. Bot decision RNG, if used by L0, is isolated from game RNG and replayed through accepted commands.
- Seat and team arrays are serialized in canonical ID order. Unordered maps may not define hashes, trace ordering, score order, or UI order.
- Score components use bounded integer types with overflow tests; no floating point.
- Every effect batch has stable semantic order: accepted action → public/private consequences → phase/trick/score transition → next actor/terminal.
- Public and seat-private exports carry explicit rules/data/profile versions and viewer authorization.
- Trace Schema v1 remains authoritative. A needed schema change is a stop condition and routes through an accepted ADR; local ad hoc fields cannot silently become schema.
- ADR 0009 forbids blanket regeneration. Only Blackglass Pact's new authorized goldens are added unless a named migration explicitly covers an existing artifact.

### 8.8 Foundation-amendment posture

**None expected; documentation updates only.** Existing foundations already govern:

- fixed teams/partnerships and viewer authorization;
- stable per-team outcomes;
- hidden commitments and replay exports;
- N-seat no-leak/export matrices;
- primitive pressure;
- forward mechanical-scaffolding governance;
- Rust/WASM/browser boundaries.

If `/reassess-spec` identifies a genuine gap that makes the locked gate impossible, implementation stops. The spec must be amended to name the gap and the required accepted ADR/foundation section before code proceeds. Editorially changing a foundation or silently redefining a term inside a game doc is forbidden.

### 8.9 FOUNDATIONS §12 stop conditions

Stop serious implementation or gate closeout when any of the following is true:

| Stop condition | Required response |
| --- | --- |
| A required exact rule/source decision is unresolved or contradictory | Resolve in `SOURCES.md`/spec; do not let code pick accidentally |
| A game noun/policy is proposed for `engine-core` | Reject/reroute; accepted ADR required for any kernel change |
| TypeScript/WASM transport would compute legal bids/cards, team score, bags, nil result, or terminal winner | Move behavior to game Rust before continuing |
| Static data would encode rule behavior or a YAML/DSL is proposed | Reject; compile typed behavior in Rust or obtain an accepted ADR |
| Pre-deal blind surface can observe or influence future card identities | Stop and repair RNG/view/bot/action boundary; add regression evidence |
| Any partner/opponent/observer surface leaks an unplayed hand or private derived datum | Stop; no release or golden acceptance until repaired |
| Replay/hash/fixture/export bytes would change outside authorized Blackglass additions | Name the surface and obtain ADR 0009 migration authority; no broad regeneration |
| The promoted trick helper does not fit unchanged | Keep the behavior local and record the mismatch, or open the proper atlas/ADR process; do not silently widen |
| Numeric-contract behavior is proposed for promotion at second use | Reject; keep local and record next review trigger |
| Partnership/team behavior is proposed for shared promotion at first use | Reject; keep local |
| Preimplementation C-01…C-10 audit is incomplete | Implementation admission remains blocked |
| New behavior-free scaffolding lacks a register entry | Gate closeout blocked until register-new is complete |
| A prior matching game is identified without a named unit or accepted no-unit disposition | Gate closeout blocked |
| `ci/scaffolding-audits.json` cannot honestly carry `forward-v1` or checker fails | Gate 1 blocked; do not claim completion |
| L2/search/sampling/learning bot work begins without accepted evidence and law | Remove/block the work; complete the required evidence process first |
| Tests are weakened/deleted or unrelated goldens regenerated to pass | Revert and follow failing-test protocol |
| Human IP/public-release review is unresolved for a public launch | Keep release blocked and record owner/status |

## 9. Forbidden changes

Gate 18 implementation and later candidate tasks must not:

1. add `card`, `deck`, `hand`, `suit`, `rank`, `trick`, `trump`, `dealer`, `bid`, `contract`, `nil`, `bag`, `partnership`, `team`, or equivalent domain concepts to `engine-core`;
2. add partnership, team, nil, bag, scoring, broken-spades, deal, turn-order, visibility, or winner-leads policy to `game-stdlib::trick_taking`;
3. duplicate the promoted trick helper locally without an accepted, documented mismatch decision;
4. promote numeric bid, contract evaluation, nil, blind nil, bags, or scoring at second use;
5. generalize fixed partnerships or team outcome at first official use;
6. represent team membership by replacing or overloading generic seat identity;
7. let a partner see the other partner's hand, legal cards, previews, bot candidates, or private export;
8. create a team-private viewer for a ruleset that has no team-private fact;
9. deal cards before the blind decision and then rely on UI concealment as proof that the blind actor did not see them;
10. let blind decisions alter shuffle seed, RNG draw count, deal order, or future cards;
11. let TypeScript filter bids/cards, enforce broken spades, sum contracts, score nils/bags, rank teams, or determine game end;
12. encode scoring, bid legality, blind eligibility, team grouping, play restrictions, or outcome behavior in TOML/JSON/static content;
13. introduce YAML or a rules DSL without an accepted ADR;
14. expose hidden state through views, action trees, previews, diagnostics, effects, logs, bot explanations, candidate rankings, replay exports, DOM, storage, accessibility text, test IDs, animations, source maps, or screenshots;
15. use MCTS, ISMCTS, Monte Carlo, rollouts, determinization, sampled hidden worlds, ML, RL, runtime LLM decisions, or actual-hidden-state peeking in public v1/v2 bots;
16. copy external rules prose, examples, score sheets, artwork, card faces, icons, fonts, screenshots, UI layout, or trade dress;
17. add casino, wagering, chip, cash, prize, or real-money framing;
18. add networked multiplayer, accounts, chat, ranking ladders, or tournament services;
19. change Trace Schema v1 or existing replay/hash semantics without an accepted ADR naming the surface;
20. mass-regenerate unrelated fixtures, snapshots, or golden traces;
21. delete, skip, loosen, or rewrite valid tests merely to obtain green;
22. claim `legacy-8c-covered` for Blackglass Pact or omit the `forward-v1` receipt;
23. omit a new behavior-free shape from the scaffolding register because it is local, small, or first use;
24. leave a prior-game scaffolding match as an unnamed TODO;
25. reopen or reimplement shipped Gate 16, Gate 17, River Ledger, Plain Tricks, 8F, or accepted ADR work as though missing;
26. amend a foundation silently or use this spec to supersede one;
27. create ticket files in this deliverable.

## 10. Documentation updates required

### 10.1 `specs/README.md`

At spec admission:

- add/link `specs/gate-18-blackglass-pact-spades-partnership-trick-taking.md` in the Gate 18 row or active work index using the repository's current convention;
- retain status `Not started`/`Planned` until implementation begins;
- retain the determination evidence that 8F is done and Gate 18 is the first `forward-v1` user.

At implementation closeout only:

- flip Gate 18 to `Done` with completion date and evidence link;
- record Blackglass Pact's neutral identity and fixed partnership scope;
- name any bounded scaffolding follow-on unit discovered by the audit, or ensure the accepted no-unit disposition is represented through the register/receipt instead;
- do not mark Gate 19 active until every Gate 18 exit/debt row is closed.

### 10.2 `docs/SOURCES.md` and game source/IP notes

- add a concise bibliography entry for the consulted Spades rules, strategy, implementation prior art, and accessibility sources;
- state the Rulepath lessons: variant diversity requires explicit pinning; failed-nil attribution is variant-sensitive; blind nil is modeled as a pre-deal commitment; partnership strategy may use public signals but not private partner state;
- distinguish external rules/strategy sources from in-repository evidence;
- keep detailed variant reconciliation, neutral-name rationale, prose originality, asset/font plan, and review status in `games/blackglass_pact/docs/SOURCES.md`;
- note that “Spades” is a permissible common public-domain family name but the product uses Blackglass Pact under the established neutral-catalog convention;
- preserve the requirement for human IP review.

### 10.3 `docs/MECHANIC-ATLAS.md`

Update the atlas without changing foundation doctrine:

1. **Follow-suit legality** — append Gate 18 as a reuse/conformance site for the existing promoted primitive. State that `follow_suit_indices` fits unchanged and broken-spades lead policy remains local.
2. **Trick resolution / led-suit comparator** — append Gate 18 as a reuse/conformance site using caller-projected `Some(Spades)`. State that partnership and winner-leads policy remain local.
3. **Numeric trick bid / contract-vs-result / last-bidder hook** — record the mandatory Vow Tide↔Blackglass Pact second-use comparison, the structural differences, the keep-local decision, no promotion, and the third close-game review trigger.
4. **New row: fixed competitive partnership contract/team outcome** — record first use in Blackglass Pact as `local-only`, including pairing, ordinary aggregation, nil/blind interaction, bags, partnership visibility, and stable team outcomes.
5. **Shared-outcome cooperative terminal** — if useful, add a cross-note that Blackglass Pact is competitive teams and does not satisfy or promote the fully cooperative shape; do not merge the rows.
6. **§9A interlock** — record Gate 18's compliance with the rule that partnership/team behavior stays local and is not folded into seat identity.
7. **§10A open promotion debt** — update only if a genuine promotion occurs under governing law. The planned result is still empty; do not create debt merely for the local team or second-use bid shapes.

### 10.4 `docs/MECHANICAL-SCAFFOLDING-REGISTER.md`

At pre-code admission, add or link the Gate 18 audit record as the register's forward maintenance cadence requires. At post-build closeout:

- record reuse evidence for applicable C-01…C-10 entries;
- register any newly invented behavior-free shape on first use with decision state, exclusions, evidence, compatibility impact, and next review trigger;
- record every actual earlier-game match;
- name a bounded tracker unit or an accepted no-unit disposition for each match;
- state the final Blackglass Pact audit disposition that agrees with `GAME-EVIDENCE.md` and `ci/scaffolding-audits.json`;
- do not place team/scoring/bot/UI behavior in the register merely to make the receipt look complete.

### 10.5 Game-local documents

Every document in §4.1 must be created and kept synchronized. In particular:

- `RULES.md`, `RULE-COVERAGE.md`, code, traces, and `HOW-TO-PLAY.md` agree on the exact failed-nil/bag rule;
- `MECHANICS.md`, `PRIMITIVE-PRESSURE-LEDGER.md`, atlas, and register agree on behavioral versus scaffolding classification;
- `GAME-EVIDENCE.md` links evidence rather than duplicating rules/strategy/UI prose;
- `AI.md`, `COMPETENT-PLAYER.md`, and the deferred evidence pack agree on L0/L1/L2 status;
- `UI.md` includes the complete viewer/no-leak and accessibility matrix;
- `BENCHMARKS.md` names calibrated workloads and variance posture;
- `PUBLIC-RELEASE-CHECKLIST.md` carries pending human review until signed.

### 10.6 `apps/web/README.md` and catalog-enforced surfaces

Update all three documentation surfaces enforced by the current catalog checker:

1. intro/catalog game list — add Blackglass Pact;
2. Shell Surface renderer list — add the dedicated grouped partnership renderer;
3. Smoke Layers `smoke:e2e` list — add the dedicated Blackglass Pact smoke.

Also update as required by live checks:

- public rules manifest/list;
- game picker/catalog description;
- supported renderer and smoke documentation;
- any root catalog list that the current checker proves is authoritative.

Passing the UI while leaving these enforced lists stale is not completion.

### 10.7 CI, tools, WASM, and public rules records

- add `blackglass_pact` to `ci/games.json`;
- add the `forward-v1` receipt to `ci/scaffolding-audits.json`;
- add Rust workspace and WASM catalog constants/dispatch/snapshots;
- add simulator/replay/fixture/rule-coverage registration only where live code requires explicit registration;
- add `apps/web/public/rules/blackglass_pact.md` and its manifest entry through the established copy/check process;
- add `blackglass-pact.smoke.mjs` to the e2e command;
- update outcome explanation and presentation-copy registries only where current checkers require explicit entries.

### 10.8 Foundation-amendment posture

**No foundation amendment is expected. Updates only.** The anticipated changes are game-local docs, atlas/register/source/index/catalog evidence, and ordinary registration surfaces.

If a real gap is discovered:

- add a dedicated “Foundation amendment required” subsection to this spec during `/reassess-spec`;
- identify the exact foundation section and contradiction;
- obtain an accepted ADR that names the affected section before implementation;
- never slip a new team, visibility, bot, trace, or scaffolding rule into an area doc or game doc as an implicit amendment.

### 10.9 Closeout and archival records

After all evidence passes:

- record the exact command/evidence summary in the spec outcome or linked closeout record under repository convention;
- update `specs/README.md` status and date;
- follow `docs/archival-workflow.md` for any later move from active `specs/` to `archive/specs/`; do not archive early and do not sever links.

## 11. Sequencing

### 11.1 Predecessor determination

Unit 8F is `Done` as of 2026-06-25 and Gate 18 is its first `forward-v1` consumer. The Gate 17 trick-helper promotion is complete and §10A debt is empty. No additional pre-Gate-18 maintenance unit is required before this spec.[^R2][^R3]

### 11.2 Admission sequence

The normative sequence is:

1. save this file under `specs/` and link it from the active index;
2. run `/reassess-spec` against the live target tree, preserving settled product decisions unless a genuine contradiction is proven;
3. complete source, rule, coverage, mechanic, and forward-audit documents;
4. pass `GAME-IMPLEMENTATION-ADMISSION.md`; serious code is blocked before this point;
5. run `/spec-to-tickets` to create bounded dependency-ordered tasks from §5;
6. implement Rust state/rules/visibility/replay before relying on browser presentation;
7. add bots, evidence, tools, WASM, and browser surfaces in dependency order;
8. complete primitive/scaffolding post-build closeout and all evidence receipts;
9. pass the full command suite and human release review;
10. flip Gate 18 to `Done` and only then admit the successor.

### 11.3 In-gate dependency order

```text
sources/rules/coverage
        ↓
forward-v1 reuse-first audit + implementation admission
        ↓
fixed seats/teams/state/setup
        ↓
blind commitment + deterministic deal
        ↓
public bidding/contracts
        ↓
promoted-helper trick play
        ↓
team scoring/bags/outcome
        ↓
visibility/replay/export
        ↓
L0/L1 + tests/traces/evidence
        ↓
simulation/benchmarks
        ↓
WASM/tools/catalog
        ↓
grouped UI/e2e/a11y
        ↓
atlas/register/forward receipt closeout
        ↓
release docs/status
```

A later task may begin early only when its inputs are stable and the bounded packet proves no authority inversion. Browser code cannot become the de facto rules prototype.

### 11.4 Successor

The roadmap successor is **Gate 19 — Five Hundred Rummy**. Gate 19 is not admitted until Blackglass Pact is complete, every required debt lane is closed, and no unnamed promotion/scaffolding/trace/evidence blocker remains.[^R1]

This spec does not design Gate 19 or use it as justification to over-generalize Gate 18 mechanics.

## 12. Assumptions

Each assumption is intentionally one-line-correctable. A correction updates source notes, rules, coverage, and affected evidence before implementation continues.

1. `assumption:` no foundation amendment is expected; Gate 18 consumes existing team, hidden-info, evidence, primitive, and scaffolding law.
2. `assumption:` the deliverable is this authored spec only; ticket decomposition follows `/reassess-spec` and `/spec-to-tickets`.
3. `assumption:` **Blackglass Pact** is the neutral public name; `blackglass_pact` is the module/game ID and `blackglass-pact` the filename slug, subject to human IP review.
4. `assumption:` initial dealer `seat_0` is preferable to a pregame random draw because the match seed already provides deterministic variation and the dealer rotates.
5. `assumption:` blind nil uses the common 100-point trailing threshold and is evaluated per seat, not as a special team-wide bid.
6. `assumption:` blind nil is committed before shuffle/deal, a deliberate deterministic/no-leak formalization of “before seeing cards.”
7. `assumption:` no nil or blind-nil card exchange is included.
8. `assumption:` failed nil tricks do not help the partner's ordinary contract and do count as bags, following the selected Pagat/Trickster option.
9. `assumption:` ordinary contract uses only positive numeric bidders' tricks; nil/blind bidders are excluded from both `C` and `O`.
10. `assumption:` ordinary overtricks count only when the ordinary contract is made; failed-nil tricks count as bags whether the ordinary contract is made or set.
11. `assumption:` ten bags subtract 100 with repeated rollover and separate bag storage.
12. `assumption:` a match ends after a complete hand only when at least one team is at 500+ and scores are unequal; exact ties continue without a bag tiebreak.
13. `assumption:` public team facts require no team-private viewer; a partner's hand remains ordinary seat-private information.
14. `assumption:` the existing promoted trick helpers fit spades-as-always-trump without modification, as direct signature inspection indicates.
15. `assumption:` numeric trick-contract behavior remains local after the Vow Tide comparison; no second-use promotion is justified.
16. `assumption:` fixed competitive partnership/team outcome is first official use and remains local-only.
17. `assumption:` the forward audit will primarily reuse C-01…C-10 and discover no new behavior-free shape or prior-game retrofit; closeout must replace this expectation with evidence.
18. `assumption:` a grouped partnership renderer can compose existing shared seat/action/effect/replay presentation scaffolding without a new generic team framework.
19. `assumption:` all four of replay-check, simulate, fixture-check, and rule-coverage require explicit per-game registration — there is no generic auto-discovery path. `tools/fixture-check/src/main.rs` and `tools/rule-coverage/src/main.rs` both resolve games through a hard-coded `match game { … }` in `resolve_game`; fixture-check additionally needs a game-crate dependency in `tools/fixture-check/Cargo.toml` and a per-game allowed-JSON-keys constant. Reassessment confirmed this against the live tree; the §4.5 rows record the minimum live edits.
20. `assumption:` L1 is required for serious-demo quality; L2 remains intentionally deferred.
21. `assumption:` provisional performance budgets in §7.8–§7.9 are calibrated, not silently waived, under accepted benchmark ADRs.
22. `assumption:` Trace Schema v1 and current replay/export/hash taxonomy are sufficient; no migration is expected.

# Appendix A — Stable rule-ID and coverage skeleton

The following IDs are normative anchors for `RULES.md` and `RULE-COVERAGE.md`. Implementation may split a row into narrower sub-IDs only with a migration note that preserves traceability.

## A.1 Identity, setup, seats, and partnerships

| Rule ID | Requirement | Minimum evidence |
| --- | --- | --- |
| `BP-ID-001` | Game ID is `blackglass_pact`; standard variant and rules/data versions match the header | Manifest/catalog/WASM/tool tests |
| `BP-ID-002` | Public name is Blackglass Pact; Spades appears only as the family/source label | Source/IP and presentation-copy checks |
| `BP-SETUP-001` | Exactly four seats are supported; every other count is rejected | Unit tests + invalid traces |
| `BP-SETUP-002` | Stable seats serialize in `seat_0..seat_3` order with North/East/South/West labels | Serialization/view traces |
| `BP-SETUP-003` | `team_0 = seat_0 + seat_2`; `team_1 = seat_1 + seat_3` | Setup/team mapping trace |
| `BP-SETUP-004` | Team IDs are stable/public and do not replace seat IDs | View/outcome snapshots |
| `BP-SETUP-005` | Initial dealer is `seat_0`; non-terminal hand dealer rotates clockwise | Setup/dealer traces |
| `BP-SETUP-006` | Match seed and version inputs are recorded under existing replay law | Replay/serialization tests |

## A.2 Blind-nil commitment

| Rule ID | Requirement | Minimum evidence |
| --- | --- | --- |
| `BP-BLIND-001` | Blind nil eligibility requires the actor's team to trail by at least 100 at hand start | Boundary tests at 99/100 and negative scores |
| `BP-BLIND-002` | Eligible decisions run left of dealer clockwise before shuffle/deal | Order trace |
| `BP-BLIND-003` | Legal leaves are exactly declare/decline for the active eligible seat | Tree/validator tests |
| `BP-BLIND-004` | Ineligible seats are deterministically skipped and receive no blind control | View/action tests |
| `BP-BLIND-005` | Declaration/decline is public, accepted once, and immutable | Public projection/replay trace |
| `BP-BLIND-006` | No hand, deck order, card-derived preview, or card-derived bot input exists before decision | Blind no-leak corpus |
| `BP-BLIND-007` | Blind decisions do not change shuffle/deal bytes | Paired-seed property/trace |
| `BP-BLIND-008` | Both partners may independently declare; no special combined bonus exists | Double-declaration score tests |
| `BP-BLIND-009` | No card pass/exchange follows a declaration | Action-tree and state tests |
| `BP-BLIND-010` | Declaring seat is skipped during ordinary bidding and has a zero blind-nil contract | Bid order/contract trace |

## A.3 Deal and hidden hands

| Rule ID | Requirement | Minimum evidence |
| --- | --- | --- |
| `BP-DEAL-001` | Deck contains exactly one of each standard 52-card rank/suit combination, ace high | Deck unit/property tests |
| `BP-DEAL-002` | Deal begins left of dealer and proceeds singly clockwise | Deterministic deal trace |
| `BP-DEAL-003` | Each seat receives 13 unique cards and no tail remains | Card conservation property |
| `BP-DEAL-004` | An unplayed card is visible only to its owning seat | Viewer/pairwise tests |
| `BP-DEAL-005` | Public deal evidence exposes counts and phase, not card identities | Observer/effect trace |
| `BP-DEAL-006` | Deal/redeal ordering is stable across replay and serialization | Replay/hash tests |

## A.4 Bidding and contracts

| Rule ID | Requirement | Minimum evidence |
| --- | --- | --- |
| `BP-BID-001` | Bidding starts left of dealer and proceeds clockwise through dealer | Bidding-order trace |
| `BP-BID-002` | Each non-blind seat bids exactly once | State/property tests |
| `BP-BID-003` | Legal bid leaves are nil and integers 1–13 | Tree boundary tests |
| `BP-BID-004` | Numeric zero, pass, rebid, and out-of-range values are illegal | Invalid traces |
| `BP-BID-005` | Accepted bids become public immediately and are immutable | Public view/replay tests |
| `BP-BID-006` | No total-13 or dealer last-bidder hook is applied | Regression test contrasting Vow Tide |
| `BP-BID-007` | Ordinary team contract is the sum of positive numeric partner bids | Contract unit/property tests |
| `BP-BID-008` | Nil and blind nil contribute zero to the ordinary team contract | Contract trace |
| `BP-BID-009` | Each nil/blind-nil contract remains attached to its bidding seat | State/serialization tests |
| `BP-BID-010` | Team and seat bid projections use stable IDs/order | View/WASM snapshots |

## A.5 Trick play

| Rule ID | Requirement | Minimum evidence |
| --- | --- | --- |
| `BP-PLAY-001` | Seat left of dealer leads the first trick | Trace |
| `BP-PLAY-002` | Before breaking, a leader with a non-spade cannot lead a spade | Rule/tree/diagnostic tests |
| `BP-PLAY-003` | A leader holding only spades may lead one, and that lead breaks spades | Exception trace |
| `BP-PLAY-004` | An off-suit spade legally played by a void follower breaks spades | Trace/property |
| `BP-PLAY-005` | A follower must follow led suit when holding it | Helper/tree/validator tests |
| `BP-PLAY-006` | A void follower may play any owned card | Property/trace |
| `BP-PLAY-007` | Highest spade wins if any spade is played | Comparator conformance tests |
| `BP-PLAY-008` | Otherwise highest led-suit rank wins; off-suit non-spades cannot win | Comparator tests |
| `BP-PLAY-009` | Trick winner leads next | Transition trace |
| `BP-PLAY-010` | Exactly four cards complete a trick and exactly 13 tricks complete a hand | State/property tests |
| `BP-PLAY-011` | `follow_suit_indices` is reused unchanged | Direct/helper conformance evidence |
| `BP-PLAY-012` | `winning_play_index` is reused with `Some(Spades)` unchanged | Direct/helper conformance evidence |

## A.6 Scoring and bags

| Rule ID | Requirement | Minimum evidence |
| --- | --- | --- |
| `BP-SCORE-001` | `C` sums positive numeric bids; `O` sums tricks won only by those ordinary bidders | Scoring unit/property |
| `BP-SCORE-002` | Ordinary contract is made iff `O >= C` | Boundary tests |
| `BP-SCORE-003` | Made ordinary base is `+10 × C` | Worked examples/traces |
| `BP-SCORE-004` | Set ordinary base is `−10 × C` | Worked examples/traces |
| `BP-SCORE-005` | Made ordinary overtricks add +1 point and one bag each | Trace/property |
| `BP-SCORE-006` | A set ordinary contract produces no ordinary overtrick points/bags | Regression test |
| `BP-SCORE-007` | Made ordinary nil is +100; failed nil is −100 | Nil traces |
| `BP-SCORE-008` | Made blind nil is +200; failed blind nil is −200 | Blind traces |
| `BP-SCORE-009` | Failed nil/blind tricks never help the ordinary contract | Attribution tests |
| `BP-SCORE-010` | Every failed nil/blind trick adds +1 point and one bag, even on an ordinary set | Cross-case trace |
| `BP-SCORE-011` | Bags persist across hands as a separate integer field | Serialization/history tests |
| `BP-SCORE-012` | Every 10 raw bags subtract 100 and remove 10; multiple thresholds apply repeatedly | Threshold property/tests |
| `BP-SCORE-013` | Bag remainder survives sets, nil outcomes, and target crossing | Multi-hand traces |
| `BP-SCORE-014` | Hand delta applies the exact §3.3 component order | Unit oracle/trace breakdown |
| `BP-SCORE-015` | Every hand exposes Rust-authored per-seat and per-team score components | View/effect/outcome tests |
| `BP-SCORE-016` | Integer arithmetic cannot overflow within supported evidence budgets | Boundary/property tests |

## A.7 Hand transition and terminal outcome

| Rule ID | Requirement | Minimum evidence |
| --- | --- | --- |
| `BP-END-001` | Terminal evaluation occurs only after all 13 tricks and hand scoring | Transition tests |
| `BP-END-002` | At least one team must be at 500+ and scores must differ | Target boundary tests |
| `BP-END-003` | Unique higher team wins when the terminal predicate is met | Terminal traces |
| `BP-END-004` | Exact tie at/above 500 continues to another full hand | Tie trace |
| `BP-END-005` | After tie continuation, falling below 500 does not create a terminal state | Multi-hand trace |
| `BP-END-006` | Bags, seat order, dealer, and team ID are not tiebreakers | Outcome tests |
| `BP-END-007` | Non-terminal hand advances dealer and starts a fresh blind-eligibility phase | Transition trace |
| `BP-END-008` | Terminal state retains the completed-hand dealer/context and does not start a phantom hand | Serialization/trace |
| `BP-END-009` | `standings_by_team` is stable team-ID order with scores/ranks/winner flags | Outcome snapshot |
| `BP-END-010` | `standings_by_seat` is stable seat-ID order with team, bid, tricks, nil result, and rank linkage | Outcome snapshot |

## A.8 Visibility, replay, bot, and UI requirements

| Rule ID | Requirement | Minimum evidence |
| --- | --- | --- |
| `BP-VIS-001` | Public observer receives no unplayed card or private control/candidate | Observer corpus |
| `BP-VIS-002` | Seat viewer receives own hand only | Four seat-view tests |
| `BP-VIS-003` | Partner relationship grants no private visibility | Partner rows in 12-pair matrix |
| `BP-VIS-004` | Blind phase exposes no future card-derived datum | Pre-deal corpus |
| `BP-VIS-005` | Action trees/previews are actor- and viewer-scoped | Tree/preview tests |
| `BP-VIS-006` | Diagnostics/effects reveal no unauthorized hand fact | Rejection/effect tests |
| `BP-VIS-007` | Public export and all four seat exports round-trip without privilege elevation | Export tests/traces |
| `BP-VIS-008` | DOM, storage, logs, test IDs, a11y tree, and animations contain no unauthorized datum | E2E no-leak checklist |
| `BP-REPLAY-001` | Same accepted command stream reproduces state/effects/hash under fixed versions | Replay tests |
| `BP-REPLAY-002` | Trace Schema v1 fields include phase, actor, team context, bids, score components, and migration notes as applicable | Golden validation |
| `BP-REPLAY-003` | No unrelated golden is regenerated for Gate 18 | Review evidence |
| `BP-BOT-001` | L0 selects uniformly/deterministically from legal leaves using isolated bot RNG | Bot tests |
| `BP-BOT-002` | L1 uses public facts, own hand, and lawful public-play deductions only | Authorized-input tests |
| `BP-BOT-003` | L1 explanations/candidates are viewer-safe and deterministic | Snapshot/no-leak tests |
| `BP-BOT-004` | L2 is unadmitted; L3 and prohibited algorithms are absent | AI docs/dependency/code review |
| `BP-UI-001` | Grouped table renders fixed partners and stable team IDs without color-only meaning | A11y/e2e |
| `BP-UI-002` | Blind, bid, and card controls come only from Rust legal leaves | WASM/UI tests |
| `BP-UI-003` | Team scores, bags, contracts, nil state, ranks, and explanations come from Rust | Payload/UI assertions |
| `BP-UI-004` | Hotseat handoff removes prior private subtree before next seat render | E2E DOM inspection |
| `BP-UI-005` | Observer/replay/rules/outcome surfaces are complete and accessible | Shared/dedicated smokes |
| `BP-UI-006` | Reduced motion and logical focus/status behavior preserve semantic results | Accessibility smoke |

# Appendix B — Implementation model and protocol details

This appendix is design guidance subordinate to the rule IDs. Exact Rust names may change during bounded implementation, but ownership, fields, and invariants may not disappear silently.

## B.1 Core game-local types

```rust
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum SeatId {
    North, // seat_0
    East,  // seat_1
    South, // seat_2
    West,  // seat_3
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum TeamId {
    NorthSouth, // team_0
    EastWest,   // team_1
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum BlindNilChoice {
    Declared,
    Declined,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum Bid {
    Tricks(u8), // validated 1..=13
    Nil,
    BlindNil,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum Suit {
    Clubs,
    Diamonds,
    Hearts,
    Spades,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Serialize, Deserialize)]
pub enum Rank {
    Two, Three, Four, Five, Six, Seven, Eight,
    Nine, Ten, Jack, Queen, King, Ace,
}
```

`TeamId` is deliberately local. Generic engine/WASM APIs may transport an opaque stable string, but they do not gain team semantics.

## B.2 Phase model

A representative typed phase model:

```rust
pub enum Phase {
    BlindNilCommitment {
        pending: Vec<SeatId>,
        next_index: usize,
    },
    Bidding {
        next: SeatId,
        accepted: [Option<Bid>; 4],
    },
    PlayingTrick {
        leader: SeatId,
        next: SeatId,
        plays: Vec<PlayedCard>,
        trick_index: u8,
    },
    Terminal {
        outcome: MatchOutcome,
    },
}
```

Normative constraints:

- before leaving `BlindNilCommitment`, no `hands` or shuffled `deck` field is populated with card identities;
- implementation may retain a deterministic hand number and public score context during blind commitment;
- deal occurs exactly once after the final pending blind decision;
- blind declarers are represented as accepted `Bid::BlindNil` before the bidding cursor is derived;
- scoring is a deterministic transition after the thirteenth trick, not a user action;
- a terminal state cannot also contain an active action actor.

## B.3 State ownership

A representative state inventory:

```rust
pub struct GameState {
    pub rules_version: RulesVersion,
    pub data_version: DataVersion,
    pub match_seed: MatchSeed,
    pub hand_index: u32,
    pub dealer: SeatId,
    pub phase: Phase,
    pub spades_broken: bool,
    pub bids: [Option<Bid>; 4],
    pub tricks_won: [u8; 4],
    pub current_trick: Vec<PlayedCard>,
    pub completed_tricks: Vec<CompletedTrick>,
    pub team_scores: [i32; 2],
    pub team_bags: [u8; 2],
    hands: [Vec<Card>; 4],
    history: Vec<HandResult>,
}
```

Private fields remain serializable for internal replay authority but are filtered by Rust projection. Public/browser payloads do not serialize `GameState` wholesale.

## B.4 Action generation and validation order

For every command, generation and validation apply the same ordered checks:

1. parse operation group/path under current rules version;
2. reject terminal state or wrong phase;
3. map actor to a declared seat;
4. verify actor is the active seat for the phase;
5. parse leaf to typed blind choice, bid, or card ID;
6. enforce freshness/state token;
7. enforce phase-specific range/eligibility/ownership;
8. enforce follow-suit and broken-spades policy for card play;
9. confirm the typed action exists in the Rust-authored legal leaf set;
10. apply through one validated transition path;
11. emit deterministic semantic effects;
12. project the next viewer-safe state.

There is no alternate “trusted bot” or “trusted UI” action path.

## B.5 Stable diagnostic floor

Exact messages may be original product copy, but stable codes must distinguish at least:

| Code | Meaning | Safe-detail rule |
| --- | --- | --- |
| `BP_UNSUPPORTED_SEAT_COUNT` | Setup count is not 4 | May state supported set `{4}` |
| `BP_WRONG_PHASE` | Action family not accepted now | Name public current phase only |
| `BP_WRONG_SEAT` | Actor is not active | May name active public seat |
| `BP_STALE_COMMAND` | Freshness token/state version mismatch | No hidden state diff |
| `BP_BLIND_INELIGIBLE` | Deficit rule not met | Public scores/threshold may be stated |
| `BP_BLIND_ALREADY_RESOLVED` | Seat already declared/declined | Public resolution may be stated |
| `BP_BLIND_LEAF_INVALID` | Invalid blind path | List only legal public blind choices |
| `BP_BID_OUT_OF_RANGE` | Numeric bid outside 1–13 | State range and nil option |
| `BP_BID_LOCKED` | Seat already bid | Public accepted bid may be stated |
| `BP_CARD_NOT_OWNED` | Submitted card not in actor hand | Do not reveal owner/other hand |
| `BP_MUST_FOLLOW_SUIT` | Actor held led suit | May name led suit and rule, but not enumerate hidden hand in public diagnostic |
| `BP_SPADES_NOT_BROKEN` | Illegal spade lead while a non-spade is held | Do not list actor's other cards to unauthorized viewers |
| `BP_TRICK_ALREADY_COMPLETE` | Extra play after four cards | Public trick state only |
| `BP_MATCH_TERMINAL` | Action after terminal | Public outcome only |

Diagnostics are part of replay and no-leak evidence; they are not free-form debug strings.

## B.6 Semantic effects

Minimum effect vocabulary, subject to existing shared envelope conventions:

| Effect | Visibility | Required semantic content |
| --- | --- | --- |
| `BlindNilWindowOpened` | Public | Eligible stable seat IDs, threshold context if projected by rules |
| `BlindNilDeclared` | Public | Seat/team ID, blind contract type |
| `BlindNilDeclined` | Public | Seat ID; no reason inference |
| `DealCompleted` | Public | Dealer, hand index, 13-card counts, next bidder; no card identities |
| `PrivateHandReceived` | Seat-private, if used | Owning seat's 13 cards only |
| `BidAccepted` | Public | Seat, team, bid type/value |
| `SpadesBroken` | Public | Play/seat that broke spades |
| `CardPlayed` | Public | Seat/card/trick position |
| `TrickCaptured` | Public | Winning seat/team, ordered trick cards, updated trick counts |
| `HandScored` | Public | Ordered per-team breakdown and per-seat nil/ordinary results |
| `BagPenaltyApplied` | Public | Team, threshold count, points deducted, bag remainder |
| `DealerAdvanced` | Public | Previous/new dealer and next hand index |
| `MatchCompleted` | Public | Winner team ID, stable standings arrays, final breakdown |

No team-private effect is needed. That row is explicitly `not applicable` because the locked rules have no team-private fact.

## B.7 Rust-authored views

A public view must include enough to render without re-deriving rules:

- stable seat descriptors and stable team descriptors;
- team membership for every seat;
- dealer, phase, active seat, leader, hand/trick index;
- blind eligibility/accepted decisions when public;
- accepted bids and Rust-derived ordinary team contracts;
- spades-broken state and current/completed public plays;
- per-seat trick counts;
- Rust-derived team score/bag/hand-result fields;
- legal operation-group availability appropriate to the viewer;
- terminal team and seat standings.

A seat view adds:

- only that seat's unplayed hand;
- only that seat's legal leaves and safe previews when active;
- any owner-private receipt allowed by the current interface.

It does not add partner cards or a combined team hand.

## B.8 Outcome model

Representative public output:

```rust
pub struct MatchOutcome {
    pub winning_team_ids: Vec<TeamId>, // exactly one in the locked terminal rule
    pub standings_by_team: [TeamStanding; 2],
    pub standings_by_seat: [SeatStanding; 4],
    pub final_hand: HandScoreBreakdown,
}

pub struct TeamStanding {
    pub team_id: TeamId,
    pub member_seat_ids: [SeatId; 2],
    pub score: i32,
    pub bags: u8,
    pub competition_rank: u8,
    pub is_winner: bool,
}

pub struct SeatStanding {
    pub seat_id: SeatId,
    pub team_id: TeamId,
    pub final_bid: Bid,
    pub final_hand_tricks: u8,
    pub nil_result: Option<NilResult>,
    pub team_rank: u8,
}
```

Even though the terminal rule yields one winning team, the schema uses a collection-compatible winner field if that is the established shared outcome convention. TypeScript may not infer a winner from score rows.

# Appendix C — Worked scoring and terminal examples

These examples are normative test vectors. All values are per team for one completed hand unless stated otherwise.

## C.1 Made ordinary contract crossing one bag threshold

Starting state:

```text
score = 240
bags = 8
numeric bids = 4 + 3, so C = 7
ordinary-bidder tricks = 9, so O = 9
no nil contracts
```

Calculation:

```text
ordinary_made = true
ordinary_base = +70
ordinary_overtricks = 2
failed_nil_bags = 0
new_bags = 2
raw_bags = 8 + 2 = 10
bag_penalty_count = 1
next_bags = 0
nil_delta = 0
hand_delta = 70 + 2 - 100 = -28
next_score = 212
```

Required explanation: “North–South made 7 with 9 ordinary tricks for 70 contract points and 2 bags. Reaching 10 bags applied a 100-point penalty; 0 bags carry.”

## C.2 Set ordinary contract plus failed nil

Starting state:

```text
score = 100
bags = 1
North bids 5 and wins 4 ordinary tricks
South bids nil and wins 2 tricks
C = 5
O = 4
```

Calculation:

```text
ordinary_made = false
ordinary_base = -50
ordinary_overtricks = 0
nil_delta = -100
failed_nil_bags = 2
new_bags = 2
raw_bags = 3
bag_penalty_count = 0
next_bags = 3
hand_delta = -50 + 2 - 100 = -148
next_score = -48
```

The nil bidder's two tricks do not make `O = 6`. They are worth two bag points and two carried bags even though the ordinary contract was set.

## C.3 Successful nil beside a made ordinary contract

Starting state:

```text
score = 30
bags = 4
North bids 4 and wins 5 ordinary tricks
South bids nil and wins 0
C = 4
O = 5
```

Calculation:

```text
ordinary_base = +40
ordinary_overtricks = 1
nil_delta = +100
new_bags = 1
raw_bags = 5
bag_penalty_count = 0
next_bags = 5
hand_delta = 40 + 1 + 100 = +141
next_score = 171
```

## C.4 Failed blind nil triggers a bag penalty

Starting state:

```text
score = 410
bags = 9
North bids 3 and wins 3 ordinary tricks
South declared blind nil and wins 1 trick
C = 3
O = 3
```

Calculation:

```text
ordinary_base = +30
ordinary_overtricks = 0
nil_delta = -200
failed_nil_bags = 1
new_bags = 1
raw_bags = 10
bag_penalty_count = 1
next_bags = 0
hand_delta = 30 + 1 - 200 - 100 = -269
next_score = 141
```

The blind-nil failure is both a −200 contract result and one bag point/bag; the threshold penalty is additional.

## C.5 Two bag thresholds in one hand

Starting state:

```text
score = 600
bags = 9
partners bid 1 and 1, so C = 2
ordinary bidders win all 13 tricks, so O = 13
no nil contracts
```

Calculation:

```text
ordinary_base = +20
ordinary_overtricks = 11
new_bags = 11
raw_bags = 20
bag_penalty_count = 2
next_bags = 0
hand_delta = 20 + 11 - 200 = -169
next_score = 431
```

The implementation may not apply only one penalty because a normal hand often crosses at most one threshold. The repeated rule is authoritative.

## C.6 Two failed nils with no ordinary bid

Starting state:

```text
score = -20
bags = 7
North bids nil and wins 1 trick
South declared blind nil and wins 2 tricks
C = 0
O = 0
```

Calculation:

```text
ordinary_base = 0
ordinary_overtricks = 0
nil_delta = -100 - 200 = -300
failed_nil_bags = 3
new_bags = 3
raw_bags = 10
bag_penalty_count = 1
next_bags = 0
hand_delta = 0 + 3 - 300 - 100 = -397
next_score = -417
```

There is no synthetic ordinary contract and no special double-nil penalty or automatic loss.

## C.7 Target crossing and exact tie continuation

Hand N ends:

```text
team_0 score = 500
team_1 score = 500
```

Result: non-terminal exact tie; advance dealer and begin the next blind-eligibility phase.

Hand N+1 ends:

```text
team_0 score = 431
team_1 score = 431
```

Result: still non-terminal. Neither is at 500+, and the previous threshold crossing does not permanently arm a terminal flag.

Hand N+2 ends:

```text
team_0 score = 501
team_1 score = 491
```

Result: terminal; `team_0` is the unique higher team and at least one team is at 500+.

## C.8 Both teams cross with a unique higher score

```text
team_0 score = 523
team_1 score = 517
```

Result: terminal `team_0`. The system does not require the opponent to remain below 500.

# Appendix D — Bot policy and strategy evidence

## D.1 AI level posture

| Level | Gate 18 status | Requirement |
| --- | --- | --- |
| L0 random legal | Required | Supports blind decision, bid, and play phases using only current legal leaves and isolated deterministic bot RNG |
| L1 rule informed | Required | Deterministic authored priorities using public state, own hand after deal, and lawful deductions from public play |
| L2 authored competent | Not admitted / intentionally deferred | Requires accepted competent-player analysis, evidence pack, scenario corpus, calibration, explanation/no-leak proof, and benchmark receipts before code |
| L3 deterministic search | `not applicable` | Hidden-information game; AI-BOTS permits L3 only for perfect-information |
| MCTS/ISMCTS/Monte Carlo/ML/RL/runtime LLM | Forbidden | No public v1/v2 implementation or hidden experimental hook in shipped paths |

## D.2 Authorized information

An L1 seat may use:

- own current unplayed hand after deal;
- own/team/opponent accepted public bids and blind declarations;
- public seat/team scores, bags, target posture, dealer, lead order, spades-broken state;
- public played cards, completed tricks, winners, and trick counts;
- lawful deductions from public play, such as a seat being void in a suit after failing to follow it;
- the Rust-authored legal leaf set;
- its own deterministic bounded memory derived from those observations.

It may not use:

- partner or opponent unplayed cards;
- future deck/deal identities;
- internal full state or authoritative hidden-card counts unavailable from public history;
- sampled/determinized worlds;
- aggregate statistics produced by peeking at hidden cards;
- another bot's private candidate list or memory;
- a runtime service/LLM;
- hidden information in explanations, logs, or benchmarks.

Before the deal, a blind-nil policy has **no own hand**. It may use only public scores, bags, dealer/order, prior public history if the policy contract allows it, and whether its partner has already declared.

## D.3 L0 policy

L0 behavior:

1. obtain the current viewer-authorized legal leaves from the normal game API;
2. sort leaves by canonical action-path order before applying seeded selection;
3. select one leaf using isolated bot RNG recorded by the normal bot/replay contract;
4. submit through the same validator as a human action;
5. produce a minimal safe explanation naming only the phase and chosen legal action category.

L0 never constructs a bid range or playable-card set itself.

## D.4 Bounded L1 blind-nil policy

The initial L1 blind policy is intentionally conservative and deterministic:

1. if not eligible, no action exists;
2. if the team deficit is below 200 points, choose `blind_nil/decline`;
3. if the deficit is at least 200 and the partner has not already declared blind nil this hand, choose `blind_nil/declare`;
4. if the partner has already declared, decline unless the opponent is at or above 450 and the team deficit is at least 300;
5. break any remaining equality by stable seat ID.

This policy uses no cards and does not claim statistical optimality. `COMPETENT-PLAYER.md` and `AI.md` may refine the exact thresholds before implementation admission, but any refinement must remain public-information-only, source/evidence-backed, and fixed before golden bot traces are accepted.

## D.5 Bounded L1 bidding policy

The bidding policy separates three steps:

### D.5.1 Own-hand trick estimate

Use a documented deterministic feature vector, for example:

- top spade controls and spade length;
- off-suit aces;
- kings discounted by suit length and lower-card exposure in own hand;
- queens only in tightly bounded supported combinations;
- voids/singletons paired with sufficient spades for potential ruffs;
- lead position and whether the bidder is dealer;
- no hidden-card probability and no sampled deal.

Coefficients, rounding, caps, and stable tie-breaks must be explicit in `AI.md`, covered by scenario tests, and expressed as integer/fixed-point arithmetic if scoring is needed.

### D.5.2 Nil risk screen

A regular nil is considered only when a deterministic danger score stays below a pinned threshold. Danger features may include:

- number and height of spades;
- aces and unsupported kings/queens;
- long suits likely to force a late winner;
- absence of sufficiently low escape cards;
- bidder position relative to partner/opponents;
- partner's already-public bid, without assuming partner holdings.

The policy never bids nil merely because an opponent's hidden cards are assumed favorable.

### D.5.3 Team/score adjustment

After the raw own bid:

- respect a partner's public bid as a team-risk signal, not as card knowledge;
- reduce reckless overbidding when the team is near the target and already carries many bags;
- allow bounded set-seeking aggression when materially behind;
- never add Vow Tide's total-bid hook;
- clamp numeric result to 1–13 or choose nil through the separate risk screen;
- document every priority and stable tie-break.

## D.6 Bounded L1 play policy

The play policy tracks public goals:

```text
ordinary_needed = max(0, team_contract - ordinary_tricks_so_far)
team_bag_pressure = current_bags and projected public overtricks
own_nil_state = made-so-far / failed
partner_nil_state = public bid plus public tricks
opponent_nil_state = public bid plus public tricks
```

Priority families:

1. **Legality first.** Select only Rust-emitted legal cards.
2. **Protect own nil.** Prefer a currently losing legal card; shed dangerous high cards when safely covered; never rely on unseen future cover.
3. **Cover partner nil.** When partner is currently winning unintentionally and the bot can legally overtake without jeopardizing a critical team contract, prefer the cheapest public-card-derived cover. Do not infer partner's remaining hand.
4. **Set opponent nil.** When an opponent nil bidder is currently positioned to take a trick, avoid covering that opponent unless team contract survival has higher pinned priority.
5. **Make ordinary contract.** If tricks are still needed, prefer the cheapest card that is currently winning or creates a documented own-hand control line.
6. **Avoid bags after contract.** Prefer a currently losing card; when forced to win, use the cheapest winner and preserve flexibility.
7. **Lead selection.** Before spades are broken, prefer low cards from supported long suits or safe own-hand patterns; after breaking, lead spades only under a documented contract/nil/setting rationale.
8. **Stable ties.** Canonical suit/rank/card ID order, or another documented stable order; no iteration-order dependence.

“Currently winning” uses public current-trick data and the promoted comparator. It is not a prediction about hidden future plays.

## D.7 Partner signals and inference boundary

The game has no chat or explicit signaling channel. A bot may react to lawful public evidence:

- partner's public bid/nil declaration;
- partner's public leads and played cards;
- publicly established void suits;
- public trick and score context.

It may not encode conventions that require private coordination outside the rules, exchange hidden messages through timing/UI metadata, or claim certainty about partner holdings. A strategy explanation should say “Partner bid nil and has not taken a trick” or “Partner is publicly void in hearts,” never “Partner has the ace of spades.”

## D.8 Competent-player analysis requirements

`COMPETENT-PLAYER.md` must cover, with sources and original Rulepath analysis:

- counting likely tricks without treating every ace/king as certain;
- spade length/control and void/short-suit ruff potential;
- positional effects of bidding/leading order;
- individual nil risk and how a partner can lawfully cover through play;
- why failed nil tricks do not help the ordinary contract in this variant;
- ordinary team contract management versus individual nil objectives;
- setting opponents versus making one's own contract;
- bag avoidance and deliberate bag pressure;
- score-target posture, including exact ties and negative scores;
- public void inference and card-count memory;
- common novice errors: overbidding face cards, unsafe nil with high spades, breaking spades carelessly, helping a failed nil into the ordinary contract, forgetting bag rollover, and assuming a partner's unseen cards.

The 2019/2020 Spades bidding paper supports treating bidding and play as distinct decision problems and using domain-specific features, but its learned utility correction is outside Rulepath v1/v2 and is not adopted.[^E8]

## D.9 Future L2 evidence gate

Before any L2 code, the evidence pack must contain:

- accepted competent-player taxonomy;
- exact policy ID/version and authorized input/memory schema;
- deterministic priority vector and tie-break rules;
- fixed scenario corpus spanning ordinary, nil, blind, bags, target, and partner/opponent states;
- evaluation against L0 and L1 with fixed seed manifests and team/seat balance;
- calibration separated by bidding, nil selection, play, and score posture;
- no-leak proof for inputs, explanations, candidates, replays, and browser surfaces;
- legality/replay/hash/benchmark evidence;
- explicit code/dependency review proving absence of MCTS, ISMCTS, Monte Carlo, determinization, sampled worlds, ML, RL, runtime LLM, and hidden-state peeking;
- accepted implementation-admission update.

Until all rows pass, `BOT-STRATEGY-EVIDENCE-PACK.md` remains informative but `not admitted`.

# Appendix E — Browser interaction, grouped partnership layout, and accessibility

## E.1 Information architecture

The renderer must communicate two structures simultaneously without conflating them:

- **seat ring:** North, East, South, West in stable clockwise order;
- **team grouping:** North–South versus East–West.

Recommended hierarchy:

1. match header: game name, hand number, dealer, phase, observer/viewer mode;
2. two public team summary regions keyed by `team_0` and `team_1`;
3. four positional seat frames around the current trick;
4. current trick/play order and spades-broken state;
5. active viewer's own hand and Rust-emitted controls;
6. public hand score/history and replay controls;
7. outcome explanation when terminal.

Opposite partners may share a visual bracket, repeated team label, pattern, or bordered region, but seat focus order remains logical and each seat retains its own name, bid, trick count, status, and controls.

## E.2 Team summary content

Each Rust-backed team summary presents:

- stable public team label and both member seat labels;
- cumulative score;
- carried bags and next threshold context when supplied by Rust;
- current ordinary combined contract;
- current ordinary-bidder trick count;
- each member's bid type/value;
- nil/blind-nil status and made/failed status only when lawfully determined;
- current hand delta and component breakdown after scoring;
- terminal competition rank and winner state.

The client may format numbers and labels but may not sum bids, attribute tricks, predict made/set status, calculate bag thresholds, or rank teams. A non-terminal “needed tricks” indicator must be a Rust field, not client subtraction.

## E.3 Seat frame content

Each seat frame shows public facts:

- seat label and team label;
- dealer, active bidder/player, leader, and last-trick-winner state;
- public bid or “not yet bid”;
- nil/blind-nil badge;
- tricks won;
- public played card in the current trick;
- hand count or card backs for non-viewers.

Only the authorized local seat frame/hand region receives card identities and actionable controls. Partner cards are never face-up merely because the frames share a team region.

## E.4 Blind-nil interaction

- Enter a distinct public “blind commitment” phase before any hand is rendered.
- The eligible active seat receives exactly two native button controls from Rust leaves: declare and decline.
- The prompt states the public risk/reward (`+200 / −200`) and 100-point trailing eligibility, without displaying a hand placeholder that could imply cards are already loaded.
- Ineligible viewers see public turn/progress only.
- Accepted decision becomes public immediately and receives a semantic status update.
- No keyboard/pointer interaction can reveal, peek, fan, focus, or inspect a future card.
- Browser network/dev-panel evidence must show that no card identities were sent during this phase.
- Focus moves to a stable phase-status region after acceptance, not to a soon-to-be-mounted hidden hand.

## E.5 Bidding interaction

- Once the deal is projected, the active non-blind seat sees Rust-emitted `nil` and numeric bid controls.
- Controls use native buttons or a standards-compliant single-selection group; no drag, wheel-only, timed gesture, or free-text parsing is required.
- The group has a programmatic name such as “Choose North's bid for this 13-trick hand.”
- A blind-nil seat's skipped turn is announced as public status and has no hidden disabled controls.
- Accepted bids move to the public seat/team summaries and cannot be edited.
- The browser does not derive the combined contract; it renders the Rust field after each accepted bid.
- Candidate/strategy hints, when present in a non-public learning mode, must be viewer-safe Rust output and cannot expose private partner reasoning.

## E.6 Card interaction

- Own cards are semantic buttons/list items with accessible names containing rank and suit in text; suit is never glyph/color-only.
- Only legal cards are actionable in the default surface. The UI does not render every card as actionable and wait for a Rust rejection.
- A learning mode may render a Rust-provided safe disabled reason, but cannot infer “must follow clubs” or “spades not broken” itself.
- Card order is a presentation preference that does not change stable card IDs or action paths.
- Current trick is an ordered semantic list naming seat, card, and play order.
- Played-card animation and trick collection are effect-driven. Reduced-motion mode presents the same result immediately.
- After a card leaves the own hand, focus returns to a stable turn/trick status target rather than a removed hidden node.

## E.7 Scoring and bags presentation

After each hand, expose a public semantic table or structured list with:

- both team IDs/labels and member seats;
- ordinary numeric bids and combined contract;
- ordinary-bidder tricks and made/set result;
- ordinary base points;
- each nil/blind-nil seat, tricks, success/failure, and delta;
- ordinary overtricks, failed-nil bags, total new bags;
- starting bags, threshold count/penalty, carried remainder;
- hand delta and cumulative score;
- next dealer/hand state or terminal state.

Do not collapse the entire result to “+52” because nil and bag interactions are the gate's explanatory proof. The summary may progressively disclose detail, but all components remain keyboard and screen-reader reachable.

## E.8 Replay and outcome

- Replay seeks use viewer-scoped observation history and do not elevate to internal full state.
- Seat-private replay selection requires an explicit authorized viewer context and never caches two private histories together in the DOM.
- The terminal outcome surface reads Rust-authored team standings and per-seat breakdowns.
- It states “North–South wins” or the original team labels, not an inferred seat winner.
- An exact 500+ tie displays “Match continues” rather than a winner; this state is a normal non-terminal hand summary.
- Outcome explanations use stable IDs for tests and human labels for presentation, without exposing hidden final hands unless a separately authorized postgame reveal rule exists. The locked variant has no such reveal.

## E.9 Hotseat and observer safety

Hotseat transition order:

1. disable prior controls;
2. remove prior private hand, previews, candidates, and derived React state;
3. settle/cancel private animations;
4. render neutral handoff screen;
5. request the next authorized Rust viewer payload;
6. mount the next private hand only after confirmation.

Observer mode never mounts a seat-private payload and cannot switch to one through a client-only flag. A dev panel follows the existing whitelist and public-build boundary.

## E.10 Accessibility acceptance floor

The grouped table must satisfy at least:

- team identity conveyed by text/structure/pattern as well as color;[^E12]
- text and essential graphical contrast meeting the applicable WCAG 2.2 criteria;[^E11]
- logical focus order matching the meaningful phase/seat/control sequence;[^E13]
- visible focus and keyboard operation for every action/replay/rules control;
- correct accessible name, role, state, and value for buttons, groups, score regions, replay controls, and modal/handoff surfaces;[^E15]
- status messages for accepted blind decisions, bids, tricks, score changes, and match completion without forced focus jumps;[^E14]
- restrained live regions: one scoped phase/status channel and one urgent error channel rather than announcing every decorative update;
- text alternatives for card/suit/team visuals and decorative images hidden from assistive technology;
- responsive reflow without two-dimensional scrolling for core controls at the supported narrow viewport, except a deliberately scrollable score table with headers preserved;
- target size and spacing consistent with the repository's adopted WCAG posture;
- reduced-motion support for card play, trick collection, score changes, and replay seek;
- no private card identity in accessible names, descriptions, hidden labels, or live-region history for unauthorized viewers.

WCAG conformance is assessed against the repository's public-release standard; this appendix identifies the load-bearing criteria rather than replacing a full audit.

## E.11 Dedicated e2e smoke floor

`blackglass-pact.smoke.mjs` must cover at least:

1. catalog selection and exact fixed-four setup;
2. public team grouping/seat labels;
3. blind-nil ineligible first-hand state;
4. a fixture/replay path with blind-nil eligible declare/decline;
5. absence of card identities during blind commitment in payload/DOM/log/storage assertions;
6. post-deal owner hand and hidden partner/opponents;
7. sequential bid controls and public combined contract;
8. blocked spade lead before breaking and a legal only-spades/break path through Rust-provided controls;
9. follow-suit legal-only card controls;
10. trick winner/effect-driven next leader;
11. score breakdown with nil and bag rollover;
12. observer mode;
13. hotseat private subtree erasure;
14. public replay and at least two browser seat-private replay samples, while Rust/WASM covers all four;
15. exact target tie displays continuation;
16. terminal per-team outcome explanation;
17. keyboard traversal, accessible names/status, no color-only team identity, and reduced motion;
18. no console errors or leaked private values.

## E.12 Presentation anti-patterns

Reject:

- computing the team contract with `northBid + southBid` in React;
- deriving bags from score string digits;
- deciding nil success from rendered trick counts;
- marking cards disabled by a TypeScript suit check;
- using only black versus gray color to distinguish teams;
- laying out DOM in a visual order that contradicts keyboard/screen-reader order;
- leaving partner cards mounted face-up under `display:none`;
- using an aria-live region for every card animation frame;
- embedding private card IDs in React keys/test IDs visible to unauthorized viewers;
- presenting a “blind nil recommendation” from card data before the deal;
- copying a third-party green-felt table, score sheet, or branded iconography.

# Appendix F — Research reconciliation and source notes

## F.1 Rules-source reconciliation

| Topic | Source landscape | Blackglass Pact decision |
| --- | --- | --- |
| Four seats/partnerships | Core partnership descriptions use four players with opposite partners | Fixed four, fixed opposite public teams |
| Deal | Full 52-card one-at-a-time deal and 13 tricks is common | Adopt exactly; fixed deterministic initial dealer |
| First lead | Common baseline is left of dealer; some variants use lowest club or dealer/high bidder | Left of dealer; exclude fixed-card/high-bidder variants |
| Spades lead | Broken-spades restriction is common, but some allow anytime or only ban first trick | Require broken spades with all-spades exception |
| Bidding | Round-table left of dealer is common; optional minimums, team talk, total-13 hooks, and second rounds vary | Public immutable individual sequence; nil or 1–13; no hook/minimum/rebid/table talk |
| Ordinary score | ±10× contract with +1 overtricks is a common partnership baseline; some sources use zero rather than negative on set | Select ±10× to match the brief's exact-style target and Pagat baseline |
| Nil | ±100 is common | Adopt |
| Blind nil | Often double nil value; eligibility and passing vary | ±200, team trailing 100, pre-deal commitment, no passing |
| Failed nil tricks | Common variants: help partner, count as bags, or neither | Do not help partner; count as bags, following Pagat's usual rule and Trickster's explicit option |
| Bags | 10 cumulative bags → −100 with rollover is common; variants alter threshold/value or remove penalty | Adopt 10/−100 repeated rollover |
| End/tie | 500 common; higher score wins if both cross; ties may continue | 500, unique higher after full hand; exact ties continue; no bag tiebreak |
| Double nil/special bids | Highly variable | Exclude every combined/special bonus and score each nil seat independently |

The rules family has no single universal tournament standard. The purpose of `SOURCES.md` is therefore not to claim canonicity; it is to show that every selected parameter is either a documented common form or a deliberate deterministic Rulepath choice.

## F.2 Strategy research disposition

- Pagat's strategy notes and common teaching material support assessing high-card controls, spade strength, void/short-suit potential, nil danger, partner coverage, setting, and bags.[^E9]
- The Spades bidding paper separates bidding from play and uses domain-specific hand features. Blackglass Pact adopts the decomposition and the idea of explicit features, not its machine-learned correction or expected-utility search.[^E8]
- Public discussion/teaching sources are useful for identifying novice heuristics but are not authority for hidden-state access or exact bot coefficients.
- Any L1 heuristic must be rewritten as deterministic, inspectable Rulepath policy and checked against the selected rules.

## F.3 External implementation prior art

- `lukiffer/SpadesBot` exposes a blind decision endpoint before the deal endpoint that contains the hand. This is external architectural prior art for separating blind commitment from hand-bearing bid input; no code or API shape is imported.[^E7]
- `CardsWithCats` includes a game-specific Spades implementation with explicit teams, score target, bags, and lead policy. It is useful evidence that partnership aggregation belongs naturally in the game model. Its architecture and any search/simulation bot techniques are not imported.[^E16]
- External repositories are external evidence only. They do not establish anything about Rulepath's own state.

## F.4 Naming/IP disposition

“Spades” is a common rules-family name and may be used in source notes. Rulepath's public catalog convention favors original neutral names, so the product is **Blackglass Pact**. The name avoids proprietary character/story references and direct suit-icon branding. The catalog description, rules prose, examples, visual system, card art, table layout, sounds, and animation must be original or separately licensed and documented.

A bounded exact-phrase collision search is a screening step only. Human review under `docs/IP-POLICY.md` decides whether the name and presentation are acceptable for public release.[^E10]

## F.5 External source list

[^E1]: John McLeod, Pagat, “Spades — Card Game Rules,” <https://www.pagat.com/auctionwhist/spades.html>, consulted 2026-06-25. Used for fixed partnerships, round-table bidding, ±10× contract, usual failed-nil attribution, nil ±100, blind nil ±200, 10-bag/−100 rollover, 500 target, and documented variants.
[^E2]: Bicycle Cards, “Spades,” <https://bicyclecards.com/how-to-play/spades/>, consulted 2026-06-25. Used for standard deck/rank, one-at-a-time deal from dealer's left, bidding/lead order, follow suit, spades trump, broken-spades exception, 13 tricks, common 500 target, and extra-hand tie handling. Bicycle's zero-on-set scoring is contrast evidence and is not adopted.
[^E3]: Trickster Cards, “Spades Basics,” <https://www.trickstercards.com/help/spades/>, consulted 2026-06-25. Used as a second common-rules reference for team bids, nil ±100, play flow, and public score concepts.
[^E4]: Trickster Cards, “Spades Rules,” <https://www.trickstercards.com/help/spades-rules/>, consulted 2026-06-25. Used for explicit variant options: blind nil before cards are revealed, 100-point trailing eligibility, no-pass choices, broken-spades lead option, 10-bag/−100 scoring, nil values, and failed-nil “takes bags” versus “helps team” alternatives.
[^E5]: CardGames.io, “How to Play Spades,” <https://cardgames.io/blog/how-to-play-spades/>, consulted 2026-06-25. Used as accessible secondary confirmation of the partnership/bidding/play/scoring family; no prose or UI is copied.
[^E6]: University of Chicago Athletics, “IM Rules: Spades,” <https://athletics.uchicago.edu/sports/2023/6/12/intramurals-im-rules-spades.aspx>, consulted 2026-06-25. Used for explicit broken-spades, ±10× contract, 10-bag rollover, 500 target, and as contrast for a failed-nil rule that does **not** count bags; Blackglass Pact deliberately selects the Pagat/Trickster alternative.
[^E7]: `lukiffer/SpadesBot`, <https://github.com/lukiffer/SpadesBot>, consulted 2026-06-25. External implementation/API prior art only: separate blind call without a hand followed by a deal/bid call containing the hand. No code or architecture is imported.
[^E8]: Gal Cohensius, Reshef Meir, Nadav Oved, and Roni Stern, “Bidding in Spades,” arXiv:1912.11323, <https://arxiv.org/abs/1912.11323>, consulted 2026-06-25. Used for the distinction between bidding and playing and for domain-feature motivation. Its machine-learning/expected-utility method is not adopted and remains outside public v1/v2.
[^E9]: John McLeod, Pagat, “Spades Strategy and Tips,” <https://www.pagat.com/auctionwhist/spadetip.html>, consulted 2026-06-25. Used as strategy background for bidding controls, nil, partnership play, setting, and bag pressure; all Rulepath prose/policy is original and rules-checked.
[^E10]: Bounded exact-title and near-title web screening for “Blackglass Pact,” performed 2026-06-25. No prominent exact game-title conflict was identified in the screening results. This is not trademark/legal clearance and does not replace human review.
[^E11]: W3C, “Web Content Accessibility Guidelines (WCAG) 2.2,” <https://www.w3.org/TR/WCAG22/>, consulted 2026-06-25. Used for the overall accessibility acceptance posture.
[^E12]: W3C WAI, “Understanding Success Criterion 1.4.1: Use of Color,” <https://www.w3.org/WAI/WCAG21/Understanding/use-of-color.html>, consulted 2026-06-25. Used for non-color-only partnership identity.
[^E13]: W3C WAI, “Understanding Success Criterion 2.4.3: Focus Order,” <https://w3c.github.io/wcag/understanding/focus-order.html>, consulted 2026-06-25. Used for seat/team/control reading and focus order.
[^E14]: W3C WAI, “Understanding Success Criterion 4.1.3: Status Messages,” <https://www.w3.org/WAI/WCAG22/Understanding/status-messages.html>, consulted 2026-06-25. Used for phase/bid/trick/score announcements without focus theft.
[^E15]: W3C WAI, “Understanding Success Criterion 4.1.2: Name, Role, Value,” <https://www.w3.org/WAI/WCAG21/Understanding/name-role-value.html>, consulted 2026-06-25. Used for controls, groups, replay, and score semantics.
[^E16]: `dozingcat/CardsWithCats`, game-specific Spades implementation, <https://github.com/dozingcat/CardsWithCats/blob/master/lib/spades/spades.dart>, consulted 2026-06-25. External implementation prior art only for explicit teams, bags, target score, and local game modeling; no code, architecture, or prohibited bot method is imported.

# Appendix G — Repository reference paths

Each footnote below maps a spec claim to the in-repository file that grounds it.

[^R1]: `docs/ROADMAP.md` — Gate 18 ladder row, public-scaling prohibitions, per-gate debt review, successor, and forward interlock.
[^R2]: `specs/README.md` — active-epoch determination, 8F completion, first `forward-v1` user, 12-section format, workflow, and new-game audit requirements.
[^R3]: `docs/MECHANIC-ATLAS.md` — first/second/third-use law, hard-gate options, Gate 18 interlocks, promoted trick rows, numeric-contract reopen note, shared-outcome comparison, and empty §10A debt register.
[^R4]: `docs/FOUNDATIONS.md` — constitution, universal invariants, stop conditions, and ADR triggers.
[^R5]: `docs/ENGINE-GAME-DATA-BOUNDARY.md` — noun-free kernel, game-local typed behavior, static-data limits, no YAML/DSL default, and promotion process.
[^R6]: `docs/MULTI-SEAT-AND-SURFACE-CONTRACT.md` — fixed teams/partnerships, viewer matrices, no-leak, public observer, grouped UI/effects, per-team outcomes, simulator summaries, and Gate 15+ minimums.
[^R7]: `docs/TESTING-REPLAY-BENCHMARKING.md` — test taxonomy, trace/replay/determinism, N-seat no-leak/export coverage, primitive tests, budgets, and CI expectations.
[^R8]: `docs/AI-BOTS.md` — L0–L3 law, hidden-information authorized inputs, forbidden methods, competent-player/evidence gates, and bot-policy fields.
[^R9]: `docs/UI-INTERACTION.md` — legal-only interaction, Rust previews, semantic effects, replay, accessibility, and hidden-state safety.
[^R10]: `docs/IP-POLICY.md` — common versus neutral names, original prose/assets, source notes, and public-release review.
[^R11]: `docs/AGENT-DISCIPLINE.md` — bounded tasks, forbidden changes, failing-test protocol, and review law.
[^R12]: `docs/adr/0004-hidden-info-replay-export-taxonomy.md` — viewer-scoped replay/export taxonomy.
[^R13]: `docs/TRACE-SCHEMA-v1.md` — trace fields, versions, actions/effects, visibility, and outcome evidence.
[^R14]: `docs/EVIDENCE-FIXTURE-CONTRACT.md` — named profiles, canonical byte authority, artifact placement, and completion evidence.
[^R15]: `docs/WASM-CLIENT-BOUNDARY.md` — Rust/WASM/browser operation groups, replay safety, and dev-panel boundary.
[^R16]: `crates/game-stdlib/src/trick_taking.rs` — exact promoted helper signatures and behavior-free ownership boundary.
[^R17]: `archive/specs/gate-17-vow-tide-oh-hell-bidding-trick-taking.md`, `games/vow_tide/src/{actions,rules,state,setup,scoring,visibility,bots}.rs`, and `games/vow_tide/docs/PRIMITIVE-PRESSURE-LEDGER.md` — first-use numeric-contract and promoted-helper comparison baseline.
[^R18]: `docs/adr/0008-mechanical-scaffolding-governance.md` — accepted lane and 2026-06-25 forward per-game extension.
[^R19]: `docs/MECHANICAL-SCAFFOLDING-REGISTER.md`, `archive/specs/pre-gate-18-forward-scaffolding-reuse-governance.md`, `ci/scaffolding-audits.json`, and `scripts/check-scaffolding-governance.{mjs,test.mjs}` — C-01…C-10 catalog, audit lifecycle, receipt schema, and Gate 1 enforcement.
[^R20]: `templates/README.md`, `templates/GAME-EVIDENCE.md`, `templates/GAME-MECHANICS.md`, `templates/GAME-IMPLEMENTATION-ADMISSION.md`, and the remaining `templates/**` set — official-game form/lifecycle and status receipt requirements.
[^R21]: `docs/OFFICIAL-GAME-CONTRACT.md` — requirements-first workflow, rules/source/coverage, UI, trace set, and official acceptance checklist.
[^R22]: `docs/ARCHITECTURE.md` — workspace/dependency direction and action/view/effect/replay/determinism model.
[^R23]: `docs/adr/0009-replay-fixture-hash-taxonomy.md` — bounded migration authority and no blanket regeneration.
[^R24]: `docs/SOURCES.md` — researched bibliography and Rulepath lesson convention.
[^R25]: `archive/specs/gate-16-briar-circuit-trick-taking.md`, `archive/specs/gate-10-1-plain-tricks-trick-taking-proof.md`, `archive/specs/gate-15-river-ledger-texas-holdem-base.md`, and their relevant game source trees — fixed-four trick-taking, first-use trick-taking, N-seat no-leak, and outcome exemplars.

# Appendix H — Author self-check

- [x] Gate 18 is confirmed from 8F `Done`/first `forward-v1` user, empty §10A debt, lowest non-`Done` Order 9, and ROADMAP admission; no alternate next unit or maintenance detour is proposed.
- [x] The full partnership-Spades family is locked: fixed 4/2 teams, 52/13 deal, left-of-dealer bid/lead, nil, blind nil, broken spades, ±10× contract, bags, 500 target, and exact tie continuation.
- [x] Exact parameters are research-pinned and deliberate variants/deviations are documented.
- [x] The original neutral identity **Blackglass Pact** determines `blackglass_pact` and the spec filename; human IP review remains mandatory.
- [x] The promoted `follow_suit_indices` and `winning_play_index` helpers are reused unchanged with spades as caller-projected trump; no new third-use gate fires and no partnership policy enters the helper.
- [x] No card/suit/rank/hand/trick/trump/bid/contract/team/partnership/nil/bag noun enters `engine-core`.
- [x] Vow Tide↔Blackglass Pact numeric-contract second-use comparison is explicit; the behavior stays local, no helper is promoted, and a third close game is the next trigger.
- [x] Fixed competitive partnership/team outcome is recorded as a new first-use `local-only` shape, including visibility and stable team outcomes.
- [x] The first `forward-v1` audit reviews C-01…C-10 and lawful homes before admission, requires register-new, queue-or-dispose, and a Gate 1 CI receipt.
- [x] Fixed-four no-leak covers observer, all four seat viewers, every ordered seat pair, all four seat-private exports, blind pre-deal future state, and every named browser/bot/replay surface.
- [x] Stable seats/teams and seat-keyed/team-keyed simulator/outcome arrays are defined.
- [x] L0 and bounded L1 are required; L2 is gated; L3 and MCTS/ISMCTS/Monte Carlo/ML/RL/runtime LLM methods are forbidden.
- [x] Every deliverable maps to official-game, multi-seat, testing/evidence, bot, UI, replay, IP, atlas, register, WASM, tools, and web obligations.
- [x] The 12 required sections are present and explicit `not applicable` rows replace silence.
- [x] Documentation updates name `specs/README.md`, `docs/SOURCES.md`, `docs/MECHANIC-ATLAS.md`, `docs/MECHANICAL-SCAFFOLDING-REGISTER.md`, game docs, CI receipts, and all enforced `apps/web/README.md` surfaces.
- [x] Foundation posture is updates-only; any genuine gap is a stop condition requiring an explicit ADR, never silent redefinition.
- [x] Accepted ADRs 0004, 0007, 0008, and 0009 are consumed rather than weakened or reopened.
- [x] The deliverable is one authored spec and contains no ticket files.

# Outcome

**Specification status:** `Planned` / `Not started`.

This file is the decompose-ready Gate 18 plan. After it is saved to
`specs/gate-18-blackglass-pact-spades-partnership-trick-taking.md`, the repository
workflow is `/reassess-spec` in place, then `/spec-to-tickets`. No implementation,
status flip, foundation amendment, or ticket decomposition is claimed by this
deliverable.

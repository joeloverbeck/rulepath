# Gate 17 implementation spec — **Vow Tide** / classic Oh Hell bidding trick-taking

## 1. Header

| Field | Value |
| --- | --- |
| Spec ID | `GAT17-VOWTID-BIDTRI-001` |
| File | `specs/gate-17-vow-tide-oh-hell-bidding-trick-taking.md` |
| Roadmap stage | Stage 17 / Public scaling phase |
| Roadmap build gate | Gate 17 |
| Status | `Done` |
| Date | 2026-06-21 |
| Owner | Rulepath maintainers / implementation agents |
| Primary crate | `games/vow_tide` |
| Internal game id | `vow_tide` |
| Public display name | **Vow Tide** |
| Rules-family label | Classic Oh Hell / exact-bid changing-hand-size trick-taking |
| Standard variant id | `vow_tide_standard` |
| Trace rules version | `vow-tide-rules-v1` |
| Data/manifest version | `vow-tide-data-v1` |
| Browser implementation required | Yes — Rust/WASM-backed public renderer, public observer, seat-private views and replay exports, outcome explanation, and e2e smoke are gate requirements |
| Official seat declaration | Minimum `3`; maximum `7`; default `4`; supported set `{3,4,5,6,7}`; stable trace ids `seat_0` through `seat_6`; fallback public labels `Tide 1` through `Tide 7`; setup rejects every other count with a stable Rust diagnostic |
| Public observer | Required; observer receives public table facts only and never any private hand or undealt-stock identity |
| Bot floor | L0 random-legal required; a bounded L1 rule-informed bidding/play policy is in scope; L2 is not admitted; L3 is `not applicable` because deterministic search is perfect-information-only and Vow Tide is hidden-information |
| Trace schema | Existing Trace Schema v1; no schema migration authorized |
| Authority order | `docs/README.md` → `docs/FOUNDATIONS.md` → `docs/ARCHITECTURE.md` → `docs/ENGINE-GAME-DATA-BOUNDARY.md` → area contracts → accepted ADRs only where they explicitly supersede named sections → `docs/ROADMAP.md` → this spec → later candidate tasks/tickets |
| Kernel stance | No new kernel concept. Card, deck, hand, suit, rank, trump, trick, bid, contract, dealer, hand schedule, trick count, and score remain outside `engine-core` |
| Primitive stance | Gate 17 is the third close trick-taking use. Select option 2, **promote a narrow typed trick-selection/comparison helper to `game-stdlib`**, and back-port both `plain_tricks` and `briar_circuit` in this same gate so no promotion debt opens. Winner-leads orchestration, dealing, dealer rotation, hand scheduling, and scoring remain game-local. Numeric bidding/contracts are first official use and stay `local-only` in `games/vow_tide` |
| Delivery posture | One authored implementation spec. It names bounded candidate `AGENT-TASK` packets but creates no ticket files; `/reassess-spec` and `/spec-to-tickets` follow only after spec acceptance |
| Repository baseline | Authored against `joeloverbeck/rulepath` repository state as of 2026-06-21; all references resolve against the current tree |

This specification is subordinate to the foundation set. It does not redefine an upstream contract. A conflict is resolved in favor of the earlier authority, and an architecture-changing exception requires an accepted ADR before implementation.

Although `docs/IP-POLICY.md` permits the common public-domain family name “Oh Hell,” Rulepath's shipped catalog consistently favors original neutral identities. **Vow Tide** is the selected product name: *vow* describes each public exact-trick commitment, while *tide* describes the hand-size sequence flowing down to one card and back up. “Oh Hell” remains a rules-family label in source/IP notes, not the product identity. The name, copy, icon, and presentation still require the human IP/public-release review required by `docs/IP-POLICY.md`; this spec is not legal clearance.[^R10]

## 2. Objective

Gate 17 turns `docs/ROADMAP.md` §15 “Gate 17: Oh Hell” into one concrete official-game plan. Vow Tide must prove that Rulepath can generalize shipped trick-taking from fixed two- and four-seat games to a **variable 3–7-seat**, multi-hand, exact-bid game with rotating dealer, changing hand size, public sequential bidding, a last-bidder hook, trump, private hands, deterministic replay, seat-keyed outcomes, and browser-quality presentation.[^R1]

### 2.1 Gate determination — confirmed, not reopened

The next unit is settled by exact-target repository evidence:

1. `specs/README.md` records Gate 16 — Briar Circuit / Hearts — as `Done`, completed 2026-06-21.[^R2]
2. `docs/MECHANIC-ATLAS.md` §10A records `Current debt: _None_`; no promotion-debt closure unit interposes before the next mechanic-ladder gate.[^R3]
3. Gate 17 — Oh Hell is the lowest non-`Done` active-epoch row, Order 8, and its predecessor is complete.[^R2]
4. `docs/ROADMAP.md` §15 admits Gate 17 and supplies the purpose and exit obligations mapped in §6.[^R1]

This spec therefore does not reconsider a different game, reopen Briar Circuit, extend River Ledger, or insert a maintenance detour. Plain Tricks, Briar Circuit, and River Ledger are shipped evidence and comparison baselines.

### 2.2 Product and architecture objective

Vow Tide must demonstrate that Rulepath can support:

- an official variable seat range of 3, 4, 5, 6, and 7 seats with stable clockwise order and diagnostics;
- a seat-count-derived maximum hand size and a fixed, finite down-and-up schedule;
- deterministic per-hand shuffle, deal, public turn-up trump, dealer rotation, and hidden undealt stock;
- Rust-authored sequential bids from zero through hand size, with the dealer-last hook enforced in the legal action tree and validator;
- follow-suit legality, trump-aware trick resolution, and trick-winner-leads sequencing;
- exact-bid scoring, a fixed schedule terminal, co-winner-safe standings, and per-seat explanatory breakdowns;
- public observer plus every authorized seat projection, viewer-scoped replay export, and exhaustive N-seat no-leak proof;
- L0 and bounded L1 opponents that use only authorized views and legal Rust actions;
- a responsive 3–7-seat browser table where TypeScript renders, but never computes, bid or card legality.

### 2.3 Third-use trick-taking posture

`plain_tricks` is the first official close use and `briar_circuit` is the second. The atlas explicitly names Gate 17 as the third-use hard-gate trigger for follow-suit legality, led-suit/trick comparison, winner-leads sequencing, and deterministic deal/redeal rotation.[^R3][^R13][^R14] Vow Tide may not copy a third implementation and decide later.

This spec chooses hard-gate option 2: **promote a narrow, pure, typed `game-stdlib` trick-selection/comparison helper**. The promoted boundary is limited to:

- selecting the stable indices of held items matching a required led suit, or all indices when none match; and
- selecting the stable winning play index from caller-projected suit/rank values using “highest trump if any, otherwise highest led suit.”

The helper does not own cards, seats, actions, diagnostics, visibility, effects, turn mutation, dealer rotation, dealing, hand schedules, scoring, contracts, or terminal policy. Both prior matching games are back-ported in this gate with behavior, command ordering, diagnostics, effects, views, replay, hashes, and traces preserved. No §10A debt is permitted. The full decision and conformance plan are normative in §8.2.

### 2.4 First-use bidding/contracts posture

No shipped official game implements a numeric bid whose success is judged against tricks actually taken. `secret_draft`, `masked_claims`, and `high_card_duel` provide commitment/reveal comparisons, but not a numeric public contract, dealer-last sum constraint, or exact-contract scoring.[^R3][^R15]

Bidding is therefore a first official use under the atlas rule. Its bid order, hook legality, trick-count comparison, scoring, action paths, diagnostics, effects, views, bots, and UI remain typed game-local Rust in `games/vow_tide`. The gate adds a new `local-only` primitive-pressure ledger entry; it does not generalize bidding and does not encode bid legality or scoring formulas in static data.

## 3. Scope

### 3.1 In scope — locked `vow_tide_standard` variant

The following parameters are normative. They must agree across `RULES.md`, `HOW-TO-PLAY.md`, `SOURCES.md`, typed Rust, rule coverage, traces, fixtures, bot docs, UI copy, simulator output, and terminal explanations.

| Rule area | Locked Vow Tide rule | Research/source decision |
| --- | --- | --- |
| Seats | 3–7 independent seats; no partnerships or teams. Default public setup is 4 seats. Stable internal order is the supplied seat vector; trace ids are `seat_0`…`seat_6`, with fallback public labels `Tide 1`…`Tide 7`; clockwise means next index with wraparound | Pagat gives 3–7 players and says 4–6 is best; the roadmap fixes 3–7 as the official range.[^E1][^R1] |
| Deck and ranks | Standard 52-card deck; four suits; ranks 2 through ace; ace high; no jokers | Common baseline across consulted rules.[^E1][^E2] |
| Maximum hand size | `K(N) = min(10, floor((52 - 1) / N))`. One card is reserved after every deal to determine trump. Therefore `K=10` for 3–5 seats, `K=8` for 6, and `K=7` for 7 | This reproduces Pagat's 10/8/7 maxima while stating the deck-capacity derivation explicitly.[^E1] |
| Deal schedule | Descend from `K` to `1`, then ascend from `2` to `K`; the one-card hand occurs once. Total hands are `2K-1`: 19 hands for 3–5 seats, 15 for 6, 13 for 7 | Pagat's baseline runs from the seat-count maximum down to one and back up; Trickster documents the same `10..1..10` pattern for four seats. Other sequences are excluded variants.[^E1][^E2] |
| Initial dealer | Standard deterministic setup uses `seat_0` as first dealer. Dealer rotates one seat clockwise after each completed hand | Physical rules rotate the dealer; fixing the first dealer makes setup and traces reproducible.[^E1] |
| Shuffle and deal | Each hand uses deterministic Rust RNG derived from match seed plus hand index under a documented versioned rule. Deal one card at a time clockwise, beginning left of dealer, until each seat has the scheduled hand size | Physical deal is single-card rotation; seed derivation is Rulepath's deterministic digital specification.[^E1][^R4] |
| Trump | After the deal, turn the next undealt card face up. Its suit is trump for the hand. The indicator is public; it is not in any player's hand and cannot be played. The remaining undealt stock stays face down and identity-private | Baseline turn-up trump is common. The hand-size cap guarantees an indicator on every hand, so v1 needs no no-trump fallback.[^E1][^E2] |
| Bid order | Bidding starts with the seat left of dealer and proceeds clockwise; dealer bids last. Every seat submits exactly one integer bid from `0` through current hand size. Bids are public immediately and immutable after acceptance | Baseline order and range follow Pagat/Trickster. Pagat permits a bid change before the next bidder; Vow Tide deliberately uses immutable digital commands for deterministic action/replay clarity.[^E1][^E2] |
| Dealer hook | Let `H` be the current hand size and `S` the sum of all earlier bids. The dealer may not bid `H-S` when that value lies in `[0,H]`, because that would make total bids equal available tricks. If `H-S` is outside `[0,H]`, no otherwise-valid dealer bid is removed. The Rust legal tree omits the forbidden value and validation returns `VT_BID_HOOK_FORBIDDEN` if submitted | This pins the usual “hook” precisely and handles over-bid prefixes without an impossible legal set.[^E1][^E2] |
| First leader | The seat left of dealer leads the first trick of every hand | Baseline rule.[^E1][^E2] |
| Legal lead | Any held card may be led, including trump. There is no “trump must be broken” rule and no first-trick restriction | Pagat explicitly permits any suit, including trump.[^E1] |
| Following suit | A follower holding at least one card of the led suit must play that suit. A void follower may play any held card, including trump | Baseline rule and the repeated hard-gate shape.[^E1][^E2] |
| Trick winner | Highest trump wins if at least one trump was played; otherwise highest rank of the led suit wins. Off-suit non-trumps cannot win. The winning seat leads the next trick | Baseline rule.[^E1][^E2] |
| Per-hand scoring | Exact contract: score `10 + bid` when `tricks_taken == bid`; score `0` when under or over. A successful zero therefore scores 10. No consolation trick points and no negative penalty | Pagat identifies this as the simplest Blackout/Blob scoring form. The more widespread “one per trick plus ten exact” variant is deliberately excluded because it rewards tricks after a failed contract and weakens the exact-vow product identity.[^E1] |
| Hand transition | After all `H` tricks, Rust records each seat's bid, tricks taken, exact/miss result, hand addition, and cumulative score; dealer and schedule advance; a new deterministic hand is dealt unless the schedule is complete | Fixed schedule, no point target.[^E1][^E2] |
| Game end | The match ends after the final scheduled `K`-card hand. There is no target score and no extra sudden-death hand | Consulted rules end after the selected deal pattern; extra maximum-card tie hands are a known option but excluded.[^E1][^E2] |
| Tie and standings | Highest cumulative score wins. Equal top scores are co-winners and share rank 1. All standings use competition ranking (`1,1,3…`) with stable seat order only for display serialization, never as a tiebreak | Avoids arbitrary seat privilege and preserves the fixed terminal schedule. Pagat documents extra tie hands as an optional variation, not a necessity.[^E1] |
| Public facts | Seat order/labels, dealer, hand index and schedule, hand size, active bidder/player, public trump indicator, bids already placed, current/completed played cards, trick winners/counts, hand results, cumulative scores, and terminal standings | Rust projects these facts; TypeScript may only present them. |
| Private/internal facts | Each seat's unplayed hand is visible only to that seat; the undealt stock order and identities remain hidden from all browser viewers; unsubmitted future choices, unauthorized legal trees/previews/candidates, internal full trace, and seed-derived future cards remain private | Required by the multi-seat, testing, bot, UI, and replay contracts.[^R6][^R7][^R8][^R9][^R12] |
| Variants | Only `vow_tide_standard` is official in Gate 17. Variant files may hold typed identity/presentation parameters but may not select alternate rules behavior | No YAML, no DSL, and no behavior in data.[^R5] |

#### Seat-count schedule table

| Seats `N` | `floor(51/N)` | `K(N)` | Hand sequence | Total hands | Maximum cards in private hands | Minimum hidden stock after turn-up |
| ---: | ---: | ---: | --- | ---: | ---: | ---: |
| 3 | 17 | 10 | `10,9,8,7,6,5,4,3,2,1,2,3,4,5,6,7,8,9,10` | 19 | 30 | 21 |
| 4 | 12 | 10 | same 19-hand sequence | 19 | 40 | 11 |
| 5 | 10 | 10 | same 19-hand sequence | 19 | 50 | 1 |
| 6 | 8 | 8 | `8,7,6,5,4,3,2,1,2,3,4,5,6,7,8` | 15 | 48 | 3 |
| 7 | 7 | 7 | `7,6,5,4,3,2,1,2,3,4,5,6,7` | 13 | 49 | 2 |

“Minimum hidden stock” excludes the public turn-up indicator. Even at the maximum deal, at least one face-down stock card remains under the chosen formula; at smaller hands the hidden tail is larger.

#### Exact legality order

Rust action generation and validation must apply the same ordered checks so diagnostics remain stable:

1. trace/rules/data version and freshness token;
2. match is non-terminal and phase accepts the action family;
3. actor maps to a declared seat and is the active bidder/player;
4. path parses to a typed `Bid(u8)` or `Play(CardId)`;
5. bid is in `0..=H`, or played card is owned by the actor;
6. if dealer is bidding, apply the exact hook exclusion;
7. if following a lead and the actor holds led suit, require led suit;
8. requested action appears in the Rust-authored legal leaf set;
9. apply through the normal validated transition path and emit deterministic semantic effects.

The browser must not reconstruct any step.

### 3.2 In-scope modes and public surface

| Mode / surface | Gate 17 requirement |
| --- | --- |
| Human vs bot | Local browser match with 3–7 seats and at least one human-controlled seat; remaining seats may use L0 or bounded L1. |
| Hotseat | Viewer handoff hides the previous private hand before exposing the next seat; no private hand is pre-rendered behind CSS. |
| Bot vs bot | Deterministic native simulation and replay generation at every seat count. |
| Public observer | Public table facts, bids, plays, scores, and outcomes; no private hand, stock identity/order, private legal tree, or private bot candidate payload. |
| Seat-private viewer | Own hand and own legal controls plus public table facts; never another seat's unplayed hand or hidden stock. |
| Replay | Internal full trace remains native/test authority; browser exports are viewer-scoped observation histories under ADR 0004. |
| Outcome explanation | Rust-authored seat-keyed standings and hand-by-hand exact/miss breakdown rendered through the shared outcome surface. |
| Rules/help | Original `HOW-TO-PLAY.md` copied/checked into the public rules surface; formal `RULES.md` remains maintainer authority and is not rendered directly. |

### 3.3 Out of scope

| Area | Gate 17 stance |
| --- | --- |
| Alternate deal schedules | Out of scope: ascending-first, one-way, duplicate one/max hands, fixed hand size, full-deck 13-card, arbitrary user-configured schedules. |
| Alternate trump methods | Out of scope: fixed trump, rotating suits, no-trump hands, permanent spades, jokers, second decks, duplicated cards. |
| Alternate scoring | Out of scope: consolation trick points, negative distance penalties, squared bids, zero-bid bonuses, target-score/bounce scoring. |
| Bid changes or simultaneous/secret bids | Out of scope. Accepted bids are public and immutable; bidding is sequential. |
| Special one-card forehead rule | Out of scope; every seat sees only its own card. |
| Partnerships/teams | Out of scope and reserved for Gate 18 Spades. |
| Elimination, tournaments, ranking ladders, stakes | Out of scope. No casino/real-money presentation. |
| Networked multiplayer/accounts | Out of scope under local-first v1/v2. |
| L2 bot | Not admitted. A later L2 requires accepted competent-player and strategy-evidence material first. |
| L3 deterministic search | `not applicable`: AI-BOTS permits L3 only for perfect-information games, while Vow Tide has private hands and hidden stock. |
| Search/research AI | Forbidden for public v1/v2: no MCTS, ISMCTS, Monte Carlo/rollout sampling, ML, RL, or runtime LLM move selection. |
| Generic bidding/contract framework | Out of scope; this is first official use and remains local. |
| Broad card/deck/trick framework | Out of scope. Only the two pure functions in §8.2 are promoted; no shared card model, phase machine, deal engine, or scoring framework. |
| Trace Schema v2 / changed hash semantics | Out of scope absent an accepted ADR. |
| YAML / rules DSL / behavior tables | Forbidden. |
| Ticket files | Out of scope. Ticket decomposition happens after acceptance via `/reassess-spec` then `/spec-to-tickets`. |
| Rebuilding shipped games | Out of scope except the narrowly required helper conformance back-port to Plain Tricks and Briar Circuit. |

### 3.4 ROADMAP public-scaling prohibition — carried verbatim

> Private licensed content, copied rules prose or trade dress, YAML/DSL behavior, TypeScript legality, public MCTS/ISMCTS/Monte Carlo/ML/RL bots, kernel noun growth, hidden-state leakage, or private work shaping public architecture.

## 4. Deliverables

### 4.1 New game crate and evidence tree

```text
games/vow_tide/
├── Cargo.toml
├── benches/
│   ├── thresholds.json
│   └── vow_tide.rs
├── data/
│   ├── fixtures/
│   │   ├── vow_tide_3p_standard.fixture.json
│   │   ├── vow_tide_4p_standard.fixture.json
│   │   ├── vow_tide_6p_standard.fixture.json
│   │   ├── vow_tide_7p_standard.fixture.json
│   │   ├── vow_tide_hook.fixture.json
│   │   └── vow_tide_terminal_tie.fixture.json
│   ├── manifest.toml
│   └── variants.toml
├── docs/
│   ├── AI.md
│   ├── BENCHMARKS.md
│   ├── BOT-STRATEGY-EVIDENCE-PACK.md
│   ├── COMPETENT-PLAYER.md
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
│   ├── bots.rs
│   ├── cards.rs
│   ├── effects.rs
│   ├── ids.rs
│   ├── lib.rs
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
    ├── property.rs
    ├── replay.rs
    ├── rules.rs
    ├── serialization.rs
    ├── visibility.rs
    └── golden_traces/
        └── *.trace.json
```

The exact module split may be corrected during `/reassess-spec` if current crate convention makes one file unnecessary, but every behavior and evidence category named here remains required. A layout correction is not permission to broaden scope.

### 4.2 Promoted `game-stdlib` primitive and same-gate conformance

The gate adds one earned module and a dedicated microbenchmark harness:

```text
crates/game-stdlib/src/trick_taking.rs
crates/game-stdlib/benches/trick_taking.rs
```

Its locked semantic API is equivalent to the following; reassessment may improve Rust lifetimes/names without broadening behavior:

```rust
pub fn follow_suit_indices<T, S: Copy + Eq>(
    held: &[T],
    led_suit: S,
    suit_of: impl Fn(&T) -> S,
) -> Vec<usize>;

pub fn winning_play_index<T, S: Copy + Eq, R: Copy + Ord>(
    plays: &[T],
    led_suit: S,
    trump: Option<S>,
    suit_of: impl Fn(&T) -> S,
    rank_of: impl Fn(&T) -> R,
) -> Option<usize>;
```

Normative behavior:

- `follow_suit_indices` preserves input order; it returns every led-suit index when at least one exists, otherwise every index; empty input returns empty.
- `winning_play_index` preserves stable first occurrence on equal projected rank; if any trump is present, only trump can win; otherwise only led suit can win; off-suit non-trumps are ignored; empty/no-eligible input returns `None`.
- callers own all types, legality exceptions, IDs, state mutation, and diagnostics.
- no allocation or genericity shortcut may reorder action leaves or effect output in a back-ported game.

Same-gate conformance is mandatory:

- `games/plain_tricks` uses both functions with `trump = None` while retaining its two-seat round/deal/scoring behavior;
- `games/briar_circuit` uses both functions with `trump = None` inside its existing opening/point/hearts restrictions while retaining all Hearts policy;
- `games/vow_tide` uses both functions with a public per-hand trump;
- `crates/game-stdlib/src/lib.rs` exports the new module;
- helper unit/property tests, examples, anti-examples, and benchmarks land before Vow Tide trick behavior;
- both prior game ledgers/inventories, central atlas rows, and rule coverage notes are updated;
- all pre-existing Plain Tricks and Briar Circuit tests, traces, replay/hash results, visibility checks, bot tests, benchmarks, WASM behavior, and browser smoke remain valid.

No promotion debt may be recorded as a convenience. If a prior game cannot be back-ported without a behavior/hash migration, implementation stops for reassessment; the gate does not silently half-promote.

### 4.3 Typed Rust game behavior deliverables

| Deliverable | Required behavior |
| --- | --- |
| Setup | Validate 3–7 seats and stable seat order; derive `K` and full schedule; initialize match seed, dealer, hand index, cumulative scores, and first hand. |
| Card model | Game-local typed `Suit`, `Rank`, `Card`, and stable `CardId`; canonical deterministic ordering for serialization, actions, tests, and explanations. |
| Deterministic deal | Per-hand RNG derivation/version; shuffle; single-card clockwise deal from left of dealer; public turn-up trump; hidden stock; conservation checks. |
| Phase/state model | Explicit `Bidding`, `PlayingTrick`, hand-resolution transition, and `Terminal` state; active seat, dealer, schedule position, hand size, trump, public bids, private hands, current/completed tricks, per-seat trick counts, cumulative score/history. |
| Bid actions | Rust-generated `0..=H` leaves; exact dealer hook omission; immutable accepted bids; stable wrong-seat/range/hook/stale diagnostics; public bid effects. |
| Play actions | Rust-generated card leaves using local exceptions plus promoted follow-suit selection; stable ownership/follow/stale diagnostics. |
| Trick resolution | Promoted comparator supplies winning index; local Rust maps index to seat, captures public trick, increments count, emits effect, and assigns next leader. |
| Scoring | Exact bid adds `10 + bid`; every miss adds zero; hand breakdown and cumulative totals; no formula in static data. |
| Terminal outcome | Fixed schedule end; Rust-authored competition-ranked standings, co-winners, decisive score facts, per-seat hand/exact/miss totals, and final explanation. |
| Effects | Deterministically ordered semantic public/private effects; never animation instructions and never private stock/hand leakage. |
| Views | Observer and seat-private projections in Rust; browser never receives raw internal state. |
| Replay | Internal full deterministic trace plus viewer-scoped observation exports under ADR 0004. |
| Bots | L0 random legal and bounded L1 rule-informed bidding/play using authorized view, public history, legal leaves, and declared bot RNG only. |

### 4.4 Official-game documentation deliverables

Every template-backed document is filled. Explicit `not applicable` or `intentionally deferred` rows replace silent omissions.

| File | Gate 17 requirement |
| --- | --- |
| `docs/SOURCES.md` | Source identity/access date/facts used; variant comparison; deliberate deviations; original-prose and no-copy statement; Vow Tide naming rationale; asset/license status; external-prior-art boundary; unresolved questions must be none at release. |
| `docs/RULES.md` | Original normative rules with stable `VT-*` IDs for setup, schedule, trump, bid order/hook, play, scoring, terminal/tie, visibility, diagnostics, determinism. |
| `docs/HOW-TO-PLAY.md` | Original player-facing goal/setup/bid/play/score/finish explanation; no maintainer details or internal seat IDs. |
| `docs/RULE-COVERAGE.md` | Every `VT-*` rule mapped to unit/rule/property/trace/simulation/replay/serialization/visibility/UI evidence. |
| `docs/MECHANICS.md` | Full mechanic inventory, variable-seat surface pressure, bidding first use, promoted helper use, hidden stock/hands, outcome and benchmark pressure. |
| `docs/PRIMITIVE-PRESSURE-LEDGER.md` | Full Plain Tricks ↔ Briar Circuit ↔ Vow Tide comparison; option 2 promotion decision and back-port receipt; separate bidding first-use `local-only` entry. |
| `docs/GAME-IMPLEMENTATION-ADMISSION.md` | Requirements-first admission receipt completed before behavior code, then final implementation receipt. |
| `docs/AI.md` | L0/L1 contract; exact authorized fields, memory, deterministic tie-breaks, explanation schema, exclusions, simulations. |
| `docs/COMPETENT-PLAYER.md` | Sourced strategy landscape, novice traps, bidding/play priorities, contract-relative phase changes, lawful inference boundary, future L2 competence criteria. |
| `docs/BOT-STRATEGY-EVIDENCE-PACK.md` | Status `L2 not admitted`; fields completed as deferred/not applicable and evidence required for later admission stated. |
| `docs/UI.md` | Rust/React boundary, 3–7-seat layouts, viewer matrix, legal bid/card controls, outcome/replay/accessibility/reduced-motion/no-leak details. |
| `docs/BENCHMARKS.md` | Operations and fixtures by every seat count, helper before/after evidence, environment, provisional targets, calibrated floors, native/WASM distinction. |
| `docs/PUBLIC-RELEASE-CHECKLIST.md` | Official contract, IP, no-leak, catalog, public rules, renderer, replay, bot, benchmark, smoke, helper conformance, and closeout receipt. |
| `templates/AGENT-TASK.md` | `not applicable` to spec authorship; consumed later by `/spec-to-tickets`. |
| `templates/README.md` | Process/index guidance only; not copied into the game crate. |

### 4.5 Repository registration and public surfaces

| Surface | Required work |
| --- | --- |
| Workspace | Add `games/vow_tide` to root workspace and normal lockfile resolution; add `game-stdlib` dependency to the three matching game crates. |
| CI game catalog | Add a `vow_tide` row to `ci/games.json` (entry shape `{ id, sim_flags, e2e }`), e.g. `{ "id": "vow_tide", "sim_flags": "--seat-count 4 --action-cap 2048", "e2e": "vow-tide.smoke.mjs" }`; keep `node scripts/check-ci-games.mjs` green. |
| Simulator | Add game dependency/id/dispatch; accept `--seat-count 3..=7` with default 4; L0/L1 bot dispatch; seat-keyed wins/ties/exact-bid rates, average actions/hands, hook exclusions, and completion data by seat count. |
| Replay checker | `tools/replay-check/src/main.rs` uses a hard-coded `resolve_game()` registry (not generic discovery). Add a `vow_tide` arm supplying `game_id`, `rules_version` (`vow-tide-rules-v1`), and `trace_dir` (`games/vow_tide/tests/golden_traces`); add any bot-match trace handling if Vow Tide traces follow the per-seed format. |
| Fixture checker | `tools/fixture-check/src/main.rs` uses a hard-coded `resolve_game()` registry and calls per-game `<game>::load_manifest()` / `load_variants()`. Add a `vow_tide` arm supplying `game_id`, `rules_version`, `trace_dir`, `fixture_dir`, `manifest_path`, `variants_path`, and `variant_id` (`vow_tide_standard`); the crate must export `vow_tide::load_manifest()` and `vow_tide::load_variants()`. Validate typed manifest/fixtures, unknown fields, version consistency, and behavior-key rejection. |
| Rule coverage | `tools/rule-coverage/src/main.rs` uses a hard-coded `resolve_game()` registry. Add a `vow_tide` arm supplying `rules_path`, `coverage_path`, `benchmarks_path`, and `benchmarks_required`; recognize `VT-*` coverage and fail on undocumented/unproved rules. |
| WASM constants | Add `GAME_VOW_TIDE`, display name, variant, trace rules version, and associated metadata constants. |
| WASM game bridge | Add current-convention game module/dispatch (expected `crates/wasm-api/src/games/vow.rs`), setup/action/view/effect/replay/bot/outcome operation groups, 3–7 seat metadata, observer/every-seat viewer modes, and bridge pairwise no-leak dispatch. No public API schema expansion is expected. |
| WASM catalog | Catalog tags include hidden information, viewer filtering, public replay export, trick-taking, bidding/contracts, and multi-seat; declare min/max/default/supported set and labels. |
| Web catalog | Add Vow Tide metadata, neutral original icon, rules link, seat range, modes, and renderer mapping. |
| Web renderer | Add `apps/web/src/components/VowTideBoard.tsx`; reuse `SeatFrame`, shared action controls, replay, outcome, rules, mode, and effect scheduler surfaces. Register the board at every `game_id` dispatch site (the live board selector and `apps/web/src/components/ReplayViewer.tsx`, which imports each board and branches on `view.game_id`) so the renderer mounts. |
| Public rules | Add `apps/web/public/rules/vow_tide.md` and manifest row, generated from or mechanically checked against game-local original `HOW-TO-PLAY.md`. |
| Presentation/effects | Register only semantic presenters that add value: hand/deal/trump reveal, bid placed/hook warning, card play, trick capture, exact/missed contract, dealer/schedule advance, terminal standings. |
| E2E | Add `apps/web/e2e/vow-tide.smoke.mjs`; include 3- and 7-seat setup, legal-only bid/card controls, dealer hook, keyboard, hotseat handoff, observer/private no-leak, reduced motion, replay, and outcome explanation. |
| Catalog documentation | Update `apps/web/README.md` intro catalog list, Shell Surface renderer list, and Smoke Layers `smoke:e2e` list; pass `scripts/check-catalog-docs.mjs`. |

### 4.6 Static-data boundary

`data/manifest.toml` and `data/variants.toml` may contain identity, version, supported-seat metadata, labels, and presentation-safe parameters only. Fixtures may contain authoritative test setup content. They must not encode:

- the hand-size formula or schedule procedure;
- bid order or hook condition;
- follow-suit or trump comparison;
- scoring or tie formulas;
- visibility/reveal rules;
- bot priorities or explanations;
- conditions, triggers, selectors, or executable behavior.

Unknown fields and behavior-looking keys must be rejected. No YAML and no DSL are authorized.

## 5. Work breakdown

The following are candidate `AGENT-TASK` packets for later decomposition. Labels are illustrative spec-internal references; canonical ticket IDs are assigned only by `/spec-to-tickets`. Every packet follows `docs/AGENT-DISCIPLINE.md`: first determine whether failing tests remain valid, then locate the fault in the system under test or test suite, then fix and add regression evidence without weakening valid tests.[^R11]

| Candidate | Depends on | Bounded packet | Required outputs and evidence |
| --- | --- | --- | --- |
| `G17-VT-001` | Spec acceptance | **Identity, sources, variant, and requirements admission.** Lock Vow Tide name, exact rules, rule IDs, seat declaration, visibility categories, source divergences, budgets, and original-prose posture before gameplay code. | Initial `SOURCES.md`, `RULES.md`, `HOW-TO-PLAY.md`, `RULE-COVERAGE.md`, `MECHANICS.md`, `GAME-IMPLEMENTATION-ADMISSION.md`; source-reconciliation table; human IP review remains open until closeout. |
| `G17-VT-002` | `001` | **Third-use hard-gate resolution and helper conformance — gating prerequisite.** Complete the three-way ledger, add the narrow helper, tests/examples/anti-examples/benchmarks, and back-port Plain Tricks/Briar Circuit before Vow Tide trick behavior. | `game-stdlib::trick_taking`; all prior game regression suites and traces preserved; both prior ledgers/inventories updated; atlas rows updated; no §10A debt. Failure to complete blocks `005`. |
| `G17-VT-003` | `001` | **Crate skeleton, card types, variable-seat setup, schedule, deterministic deal, trump, and phase state.** | New crate/evidence tree; 3–7 acceptance and below/above diagnostics; `K`/schedule tests; card conservation; per-seat deterministic fixtures; serialization/property evidence. |
| `G17-VT-004` | `001`, `003` | **Bidding/contracts first-use ledger and implementation.** Record local-only pressure before coding; implement sequential public bids, exact hook, legal trees, validation, effects, and viewer fields. | New bidding ledger/atlas row; bid order/range/hook tests and traces; no shared bidding helper; no behavior in data. |
| `G17-VT-005` | `002`, `003`, `004` | **Trick-play legality and resolution using promoted helper.** | Lead/follow/void/trump comparator/winner-leads Rust behavior; rule/property tests; stable diagnostics/action order/effects; no TypeScript legality. |
| `G17-VT-006` | `003`–`005` | **Per-hand exact-contract scoring, schedule progression, terminal standings, and outcome explanation.** | Exact/under/over scorer; cumulative history; dealer/schedule transition; fixed terminal; competition-ranked ties; Rust seat-keyed explanation; scoring/outcome traces. |
| `G17-VT-007` | `003`–`006` | **Variable-N visibility and semantic-effect boundary.** | Observer + every seat projection for N=3..7; hand/stock redaction; action tree/preview/effect/diagnostic/bot fields filtered; exhaustive pairwise harness and viewer hashes. |
| `G17-VT-008` | `006`, `007` | **Replay, serialization, viewer-scoped exports, and golden traces.** | Trace Schema v1 pack; internal full trace; observer/every-seat export/import; all view hashes; deterministic replays; no retroactive private reveal. |
| `G17-VT-009` | `004`–`008` | **L0/L1 bots and simulations by seat count.** | Random legal L0; bounded rule-informed L1; authorized input-field audit; viewer-safe explanations; many-seed legality/no-leak; 3/4/5/6/7 seat-keyed summaries. |
| `G17-VT-010` | `003`–`009` | **Fixtures, coverage, workspace, CI, and native tool registration.** | Root/CI registration; explicit `resolve_game()` / `GAME_*` registration in `simulate`, `replay-check`, `fixture-check`, and `rule-coverage` (all hard-coded registries) plus `vow_tide::load_manifest()` / `load_variants()` exports for `fixture-check`; fixture/replay/rule-coverage/simulate evidence. |
| `G17-VT-011` | `007`–`010` | **WASM catalog and operation groups.** | Constants/catalog/module/dispatch; seat setup; all viewer modes; replay/bot/outcome operations; API snapshots; exhaustive bridge no-leak dispatch; no rule reimplementation. |
| `G17-VT-012` | `011` | **Public renderer, rules surface, interaction, accessibility, and e2e.** | `VowTideBoard.tsx`; catalog/icon/rules; phase/bid/trick/score UI; legal-only controls; hotseat/observer; keyboard/live regions/reduced motion; DOM/storage/log no-leak. |
| `G17-VT-013` | `002`, `008`–`012` | **Native benchmarks, helper before/after evidence, and calibrated CI floors by seat count.** | Operations/fixtures in §7.8; thresholds and report; all 3–7 seat counts; native source of truth; WASM smoke; no hidden-filter bypass. |
| `G17-VT-014` | All prior | **Documentation and public-release closeout.** | Complete every game doc/checklist; central sources/atlas/web docs; helper conformance receipt; exact command/trace/benchmark evidence; status `Done`; archival workflow. |

No candidate authorizes opportunistic engine, shell, or sibling-game cleanup. `G17-VT-002` is a real gate: Vow Tide follow-suit/comparator code must not land first and be “cleaned up later.”

## 6. Exit criteria

### 6.1 ROADMAP Gate 17 mapping — row for row

| ROADMAP Gate 17 exit obligation | Vow Tide completion criterion | Required evidence |
| --- | --- | --- |
| Official seat range | Rust declares and enforces minimum 3, maximum 7, default 4, supported `{3,4,5,6,7}`, stable trace ids/fallback public labels/order, and deterministic below/above diagnostics; WASM/catalog/setup/UI agree. | Setup tests/traces for every accepted count and both boundaries; catalog snapshot; simulator and e2e setup evidence. |
| Dealer rotation | `seat_0` starts; dealer advances clockwise exactly once after each completed hand; bid/first-lead order derives from dealer without TypeScript inference. | Unit/property tests, dealer-cycle trace, full-schedule traces, Rust view/effect assertions. |
| Changing hand size | `K(N)` and the exact down-to-one/up schedule in §3.1 are Rust-owned, deterministic, and conserved with the deck/trump/tail at every seat count. | Formula/schedule properties, per-seat-count fixtures, schedule-boundary traces, simulation completion. |
| Bidding order | Sequential clockwise bidding begins left of dealer and ends with dealer; all bids are public, immutable, and in range. | Rule tests, action-tree tests, bid-order trace, wrong-seat/range/stale diagnostics, UI legal-only smoke. |
| Last-bidder constraint | Dealer cannot make total bids equal hand size under the precise `H-S` rule; no other bid is removed; every state retains at least one legal dealer bid. | Exhaustive property over `H` and reachable prefix sums, hook traces, validator/legal-tree equivalence, L0/L1 legality. |
| Trick play | Any legal lead including trump; must follow if able; void may play any; highest trump else highest led wins; winner leads; all use the promoted selection/comparison helper within local phase policy. | Helper tests, three-game conformance suites, Vow Tide rule/property/traces, no TypeScript rule code. |
| Scoring | Exact bid scores `10+bid`; every over/under scores zero; cumulative scores and per-hand breakdown are Rust-authored. | Unit/property tests, zero/exact/under/over traces, rule coverage, outcome explanation. |
| Terminal standings | Match ends only after fixed schedule; highest total wins; equal leaders co-win; competition rank and decisive breakdown are viewer-safe and deterministic. | Terminal unique/tie traces, serialization/replay, outcome-panel e2e, seat-keyed simulator summaries. |
| Simulations and benchmarks by seat count | 3-, 4-, 5-, 6-, and 7-seat runs report seat-keyed wins/co-wins, exact-bid rates, average length/hands, completion/cap failures, and stable version/seed fields; benchmarks name every count and largest surfaces. | §7.1 simulation commands; `BENCHMARKS.md`; machine-readable reports and thresholds; deterministic rerun checks. |
| Trick-taking and bidding helper decisions through primitive pressure | The third-use ledger selects option 2, helper/backports complete in-gate with no debt; bidding receives a new first-use local-only entry and no helper. | Vow Tide/Plain Tricks/Briar Circuit ledgers and inventories; `docs/MECHANIC-ATLAS.md` rows; helper tests/benchmarks; boundary check; §10A remains `_None_`. |

### 6.2 Official-game completion criteria

Vow Tide is not `Done` until:

- every locked rule has a stable `VT-*` identifier and complete coverage row;
- every official document is complete, original, consistent, and has no `open` release row;
- human-vs-bot, hotseat, bot replay, observer, and every seat-private viewer work for all supported seat counts;
- L0 and L1 finish legal seeded matches at every count without hidden-state access;
- every required rule, property, trace, replay, serialization, visibility, bot, tool, benchmark, WASM, and UI test passes;
- exhaustive Rust/WASM viewer coverage and required browser samples find no hidden token on any unauthorized surface;
- Rust produces all bid legality, card legality, winner, score, standings, and explanation facts;
- the promoted helper has full same-gate conformance and no open debt;
- native benchmark floors are measured/calibrated without sacrificing filtering, explanation fidelity, or deterministic ordering;
- the 3–7-seat UI is readable, keyboard-operable, responsive, reduced-motion safe, and not debug-first;
- catalog, rules, source, atlas, web README, and release-checklist closeout is complete.

## 7. Acceptance evidence

### 7.1 Required command suite

Closeout records exact tool versions, environment, commands, and outcomes. Reassessment may correct CLI syntax if the implementation target has changed, but it may not weaken the coverage intent.

```text
cargo fmt --all --check
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo check --workspace
cargo test -p game-stdlib
cargo test -p plain_tricks
cargo test -p briar_circuit
cargo test -p vow_tide
cargo test -p vow_tide --test rules
cargo test -p vow_tide --test property
cargo test -p vow_tide --test replay
cargo test -p vow_tide --test serialization
cargo test -p vow_tide --test visibility
cargo test -p vow_tide --test bots
cargo test -p wasm-api
cargo test --workspace
cargo run -p fixture-check -- --game vow_tide
cargo run -p rule-coverage -- --game vow_tide
cargo run -p replay-check -- --game vow_tide
cargo run -p replay-check -- --game vow_tide --all
cargo run -p simulate -- --game vow_tide --seat-count 3 --games 1000 --start-seed 170300 --action-cap 2048
cargo run -p simulate -- --game vow_tide --seat-count 4 --games 1000 --start-seed 170400 --action-cap 2048
cargo run -p simulate -- --game vow_tide --seat-count 5 --games 1000 --start-seed 170500 --action-cap 2048
cargo run -p simulate -- --game vow_tide --seat-count 6 --games 1000 --start-seed 170600 --action-cap 2048
cargo run -p simulate -- --game vow_tide --seat-count 7 --games 1000 --start-seed 170700 --action-cap 2048
cargo bench -p game-stdlib
cargo bench -p plain_tricks
cargo bench -p briar_circuit
cargo bench -p vow_tide
bash scripts/boundary-check.sh
node scripts/check-doc-links.mjs
node scripts/check-player-rules.mjs
node scripts/check-catalog-docs.mjs
node scripts/check-ci-games.mjs
node scripts/check-outcome-explanations.mjs
node scripts/check-presentation-copy.mjs
npm --prefix apps/web run build
npm --prefix apps/web run smoke:wasm
npm --prefix apps/web run smoke:ui
npm --prefix apps/web run smoke:e2e
```

`--action-cap 2048` is a simulation safety guard, not a game rule. A cap hit is a reproducible failure with seed, seat count, command stream, phase, hashes, and reason; it is never counted as a draw or successful completion. For each seat count, the closeout receipt must include at least one all-L0 run and one all-L1 or documented L0/L1 mixed run; `/reassess-spec` pins the current simulator flags without reducing that matrix.

The helper back-port is not accepted merely because Vow Tide passes. The command receipt must show the unchanged Plain Tricks and Briar Circuit suites, traces, WASM checks, and relevant browser smokes passing after conformance.

### 7.2 Test taxonomy

| Test class | Required Gate 17 coverage |
| --- | --- |
| Helper unit tests | Empty/single/multi-item inputs; stable order; led-suit present/absent; no trump/trump present; off-suit non-trump exclusion; stable tie; no eligible play; caller-owned projections. |
| Helper property tests | Returned indices are valid/ordered; follow set is complete and exclusive when present; comparator result is maximal in winning class; caller input is not mutated; equivalent projections produce equivalent result. |
| Back-port regression | Existing Plain Tricks and Briar Circuit legality, action order, diagnostics, effects, state/view/effect/action-tree hashes, traces, replay, visibility, bots, benchmarks, and web behavior remain unchanged unless a separately reviewed defect is proved. |
| Game unit tests | Seat/schedule math, dealer/order wraparound, card/rank ordering, per-hand seed derivation, bid range/hook, trick counts, scoring, competition ranking, diagnostics. |
| Rule tests | Every `VT-*` path, including valid and invalid bids, legal lead, forced follow, void discard, trump/led winner, winner-leads, score exact/miss, fixed terminal/tie. |
| Property/invariant tests | Card conservation; no duplicate ownership; schedule length/range/symmetry; legal action non-emptiness; dealer hook never permits total equal to `H`; sum of trick counts equals `H`; score additions obey exact formula; dealer rotates once; deterministic state under seed+commands. |
| Golden traces | Minimum inventory in §7.6 plus distinct implementation-discovered failures. |
| Simulation tests | Seeded complete L0/L1 playouts for N=3..7; no cap hits; legal normal validation path; seat-keyed summaries; reproducible failures. |
| Replay/checkpoint/hash | Same setup/commands reproduce internal state, effects, action trees, observer and every-seat views; viewer-scoped export/import preserves only authorized history; no schema change. |
| Serialization | Stable JSON ordering and round trip for state, public/private views, effects, actions, bot outputs, fixtures, internal traces, and viewer exports; versions present; unknown/newer behavior explicit. |
| Visibility/no-leak | §7.3–§7.5 exhaustive source/target matrix over views, actions, previews, diagnostics, effects, bots, exports, WASM, DOM, storage, logs, a11y text, IDs, snapshots. |
| Bot legality | L0/L1 request normal legal tree, choose a leaf, validate normally, are deterministic under declared seed/view, and never receive raw state or unauthorized candidates. |
| Tool/manifest | Workspace/CI catalog consistency; fixtures typed and behavior-key-safe; rule coverage complete; replay checker stable; per-game registration entries in `simulate`/`replay-check`/`fixture-check`/`rule-coverage` present and evidenced. |
| Benchmark | Helper before/after; setup/deal, bid/card actions, projection/filtering, replay/export, bots, and full matches by every seat count; thresholds visible. |
| UI/e2e | Catalog/setup 3–7, phase presentation, legal-only bid/card interaction, hook, trump, score/history, hotseat/observer, replay, outcome, keyboard, focus, live regions, reduced motion, responsive table, no hidden DOM/storage/log data. |
| IP/public artifact | Rules/assets/icon/copy/fixtures/traces are original, neutral, and free of copied prose, commercial trade dress, proprietary IDs, or private content. |

### 7.3 Viewer matrix

| Viewer | Public table facts | Own unplayed hand | Other hands | Hidden stock identities/order | Own active legal tree/preview | Other seat legal tree | Viewer-scoped replay |
| --- | --- | --- | --- | --- | --- | --- | --- |
| Public observer | Yes | No | No | No | No; public pending metadata only | No | Public observation history only |
| Active seat `seat_i` | Yes | Yes | No | No | Yes | No | `seat_i` facts as known at each time plus public history |
| Non-active seat `seat_i` | Yes | Yes | No | No | No private choice alternatives until active | No | `seat_i` facts as known at each time plus public history |
| Team/partnership viewer | `not applicable` — Vow Tide has independent seats and no team-authorized private facts | `not applicable` | `not applicable` | No | No | No | `not applicable` |
| Internal native full-trace authority | Yes | Yes, all seats | Yes | Yes | Yes | Yes | Omniscient internal/test artifact only; never default browser export |

Bids are public after acceptance. The game has no hidden bid commitment. Before submission, a prospective bid exists only as a local UI focus/selection state or Rust preview for the active viewer; it is not public game state.

### 7.4 Pairwise no-leak and export-coverage matrix

The gate chooses **exhaustive CI viewer coverage**, not a sampled private-export matrix. For every supported N, every source seat receives a distinct canary hand token and the hidden stock receives separate canaries. Native game tests and the reusable WASM bridge harness scan every unauthorized viewer artifact.

| Seats `N` | Ordered seat-to-seat pairs `N(N-1)` | Source-seat → observer edges | Seat-private export classes required | CI viewer rule |
| ---: | ---: | ---: | ---: | --- |
| 3 | 6 | 3 | 3 | Observer plus every seat viewer/export |
| 4 | 12 | 4 | 4 | Observer plus every seat viewer/export |
| 5 | 20 | 5 | 5 | Observer plus every seat viewer/export |
| 6 | 30 | 6 | 6 | Observer plus every seat viewer/export |
| 7 | 42 | 7 | 7 | Observer plus every seat viewer/export |
| **Total** | **110** | **25** | **25** | **135 source→unauthorized-viewer edges before stock-specific checks** |

The browser e2e suite need not replay all 110 seat-pair combinations through Playwright because the exhaustive Rust/WASM bridge proof is the authoritative pairwise matrix. Browser e2e must nevertheless sample every seat count and all distinct UI roles across the suite: first bidder, middle bidder, dealer/last bidder, first leader, current winner, non-active owner, and observer. The 7-seat smoke must cycle viewer selection through all seven seats at least once so §8.2's 4+ seat-viewer expectation is exercised in CI.

For every ordered source `A` and unauthorized target `B`, assert that no `A` private datum appears in:

- target view or serialized target view;
- target action tree or preview;
- diagnostics returned to target;
- target/public effect stream and effect log;
- target bot input, memory, candidate ranking, or explanation;
- target/public replay export or imported timeline;
- WASM JSON, dev-panel whitelist output, or API snapshot;
- DOM text/attributes/ARIA/offscreen nodes/keys/classes/test IDs/comments;
- local/session storage, IndexedDB, URL, clipboard, console, error text, screenshots, or public fixtures.

### 7.5 Mandatory no-leak datum taxonomy

| Private/internal datum | Owner may see | Other seat | Observer | Public replay | Owner-private replay | Bot/explanation rule | Browser rule |
| --- | --- | --- | --- | --- | --- | --- | --- |
| Owner's unplayed cards | Yes | No | No | No | Yes, only as known then | Owner bot may consume; explanation mentions only viewer-authorized own facts | Only owner viewer DOM; removed before handoff |
| Another seat's unplayed cards | No | Owner only | No | No | No | Never | Never |
| Publicly played card | Yes | Yes | Yes | Yes | Yes | May reference as public history | May render after Rust play effect |
| Turn-up trump indicator | Yes | Yes | Yes | Yes | Yes | Public input | Public accessible label required |
| Hidden stock identity/order | No browser viewer | No | No | No | No | Never; no future-card or sampled-real-world feature | Never in payload, DOM, logs, seed reconstruction, or test IDs |
| Submitted bid | Public | Public | Public | Public | Public | May use as public history | Public bid rail/table |
| Unsubmitted local bid focus | Active viewer UI only; not authoritative game state | No | No | No | No | Not a bot-visible game fact until submitted | Must not leak through shared state/storage |
| Active seat legal bid/card leaves | Active seat only | No | No | Applied command only | Active seat's historical command if authorized | Acting bot only | Controls exist only for authorized current viewer |
| Dealer forbidden hook value | Dealer may receive Rust-safe legal omission/diagnostic/preview; once all prior bids are public, it is derivable public information, but no UI may compute it | May display Rust-projected public bidding context, not a client calculation | Same | Applied public history; rejected private path not exported | Same | Bot uses legal leaves, not recomputation from raw hidden state | TypeScript never filters the value itself |
| Internal RNG seed/per-hand seed | Native authority/test only | No | No | No | No | Bot receives only its separate declared bot RNG where allowed | Never persisted or exposed as replay shortcut |
| Internal full trace | Native/test only | No | No | No | No | Never public bot input | Never browser default |
| Private candidate rankings | Authorized bot/test surface only | No | No | No | No | Viewer-safe chosen rationale only; dev candidates still viewer-safe | No hidden-derived fields in dev output |

### 7.6 Golden-trace minimum set

All traces use Trace Schema v1 and record `game_id = "vow_tide"`, `rules_version = "vow-tide-rules-v1"`, exact variant/data/engine versions, ordered seat array, actor/freshness, commands, checkpoints, state/effect/action-tree/public-view hashes, private-view hashes for every declared seat, diagnostics where applicable, terminal expectation, and non-empty migration/update notes.[^R7][^R16]

Minimum inventory:

1. `setup-3p-schedule-and-deal.trace.json`
2. `setup-4p-schedule-and-deal.trace.json`
3. `setup-5p-schedule-and-deal.trace.json`
4. `setup-6p-schedule-and-deal.trace.json`
5. `setup-7p-schedule-and-deal.trace.json`
6. `invalid-seat-count-below.trace.json`
7. `invalid-seat-count-above.trace.json`
8. `deterministic-turn-up-trump-and-hidden-tail.trace.json`
9. `dealer-rotation-full-cycle.trace.json`
10. `schedule-ten-down-one-up-ten.trace.json`
11. `schedule-eight-down-one-up-eight.trace.json`
12. `schedule-seven-down-one-up-seven.trace.json`
13. `bidding-left-of-dealer-through-dealer.trace.json`
14. `bid-zero-accepted.trace.json`
15. `bid-upper-bound-accepted.trace.json`
16. `dealer-hook-forbidden-total.trace.json`
17. `dealer-hook-out-of-range-no-removal.trace.json`
18. `invalid-bid-range-diagnostic.trace.json`
19. `invalid-bid-wrong-seat-diagnostic.trace.json`
20. `bid-public-after-submit.trace.json`
21. `trump-may-lead.trace.json`
22. `follow-suit-forced.trace.json`
23. `void-may-play-any.trace.json`
24. `highest-trump-wins.trace.json`
25. `highest-led-wins-without-trump.trace.json`
26. `off-suit-nontrump-never-wins.trace.json`
27. `trick-winner-leads-next.trace.json`
28. `one-card-hand-completes.trace.json`
29. `exact-zero-scores-ten.trace.json`
30. `exact-positive-scores-ten-plus-bid.trace.json`
31. `underbid-scores-zero.trace.json`
32. `overbid-scores-zero.trace.json`
33. `hand-score-dealer-and-size-advance.trace.json`
34. `terminal-unique-high-score.trace.json`
35. `terminal-co-winners-competition-rank.trace.json`
36. `invalid-card-not-owned-diagnostic.trace.json`
37. `invalid-follow-suit-diagnostic.trace.json`
38. `invalid-stale-diagnostic.trace.json`
39. `l0-bid-and-play.trace.json`
40. `l1-contract-relative-bid-and-play.trace.json`
41. `public-observer-no-leak-3p.trace.json`
42. `public-observer-no-leak-7p.trace.json`
43. `seat-private-pairwise-no-leak-3p.trace.json` with all three private-view hashes
44. `seat-private-pairwise-no-leak-7p.trace.json` with all seven private-view hashes
45. `public-replay-export-import.trace.json`
46. `seat-private-replay-export-import-all-viewers.trace.json`
47. `bot-vs-bot-full-match-3p.trace.json`
48. `bot-vs-bot-full-match-7p.trace.json`
49. `wasm-exported-hook-to-terminal.trace.json`
50. `promoted-helper-backport-preservation.trace.json` or a closeout manifest pointing to the unchanged Plain Tricks/Briar Circuit trace sets and hashes

The minimum is not a cap. Any distinct bug or rule ambiguity discovered during implementation gets a regression test and, when it affects observable behavior or determinism, a trace.

### 7.7 Surface and action-fanout budgets

| Budget | Maximum / required fixture |
| --- | --- |
| Official seats | 7 |
| Maximum private cards per seat | 10 |
| Card identities | 52 |
| Maximum public turn-up cards at once | 1 current hand; history may show prior public indicators |
| Hidden face-down stock after indicator | Up to 48 cards on a 3-seat one-card hand |
| Current trick cards | 7 |
| Completed tricks in one hand | 10 |
| Hands in one match | 19 |
| Public bids in one hand | 7 |
| Legal bid leaves | At most 11 before hook; at most 10 for dealer when forbidden value is in range |
| Legal card leaves | At most 10 |
| Progressive action depth | At most 2 semantic segments for `bid/<n>` or `play/<card_id>` under the current envelope; no combinatorial construction |
| Viewer modes at max seats | Observer plus 7 seat viewers |
| Normal public effect batch | Target ≤8 envelopes per accepted bid/card/trick transition |
| Hand-resolution effect batch | Target ≤24 deterministic envelopes, preferably grouped into one public summary plus per-seat breakdown entries |
| Largest native fixture | 7 seats, 7-card hands, all bids placed, full 7-card trick, hidden stock canaries, terminal tie history, all viewer projections |
| Largest browser fixture | 7 seat rails, owner hand, bid rail, trump, 7-card current trick, score/history table, viewer selector, outcome explanation, replay controls |

A budget overrun requires reviewed spec/document update and benchmark/no-leak/UI evidence. It may not be hidden by collapsing Rust facts or moving legality into the client.

### 7.8 Benchmark expectations by seat count

Native Rust is the performance source of truth; WASM/browser is product-latency smoke.[^R7]

Required stable operation names include seat suffixes (`_3p` … `_7p`) where the dimension matters:

- setup, schedule derivation, shuffle/deal/trump, and canonical serialization;
- bid legal-action generation for first, middle, and dealer/hook decisions;
- card legal-action generation in unconstrained, forced-follow, and void states;
- validation/apply for bid and card actions;
- promoted helper follow selection and comparator, including before/after back-port microbenchmarks;
- trick resolution and hand scoring;
- observer and every seat projection at maximum private/public surfaces;
- effect filtering for observer and every seat;
- internal replay and viewer-scoped public/private export/import;
- L0 and L1 bid/play decisions;
- full seeded match for 3, 4, 5, 6, and 7 seats.

Provisional targets, replaced only by documented measured calibration rather than omission:

- p95 native helper, legal-tree generation, validation/apply, trick resolution, hand scoring, and any one viewer projection: `< 1 ms` in the largest relevant fixture;
- p95 native full-match observer or one-seat export generation: `< 50 ms` for the benchmark fixture;
- browser-visible Rust/WASM setup, action tree, preview, apply, and view refresh remain inside the existing complex-game `< 100 ms` interaction budget;
- release-mode full-match throughput floor: at least `75 completed matches/second` for **each** supported seat count on the documented reference machine, or a reviewed calibrated replacement with the original target retained in `BENCHMARKS.md` and the reason stated;
- helper conformance must show no material regression in Plain Tricks or Briar Circuit stable operations; any regression above measurement noise is investigated rather than accepted as abstraction tax.

Machine-readable summaries must remain seat-keyed and include: game/rules/data/bot-policy versions, seat count/order, seed range, matches run/completed, cap failures, wins and co-wins by seat, exact/miss counts/rates by seat, average actions/hands, elapsed time, throughput, and reproducible failure location.

## 8. FOUNDATIONS & boundary alignment

### 8.1 Contract alignment

| Principle / contract | Gate 17 stance | Rationale / evidence |
| --- | --- | --- |
| Public playable product first | Required. | Gate 17 ships one complete browser-playable official game, not a research-only engine project. |
| Rust behavior authority | Non-negotiable. | Rust owns schedule, deal, bids, hook, legal cards, winner, scoring, views, effects, replay, outcomes, and bots. |
| TypeScript presentation-only | Non-negotiable. | Web maps Rust action IDs and fields to controls/presentation; it never sums bids to filter the dealer or examines suits to filter cards. |
| `engine-core` contract kernel | Preserved. | No card/deck/hand/suit/rank/trick/trump/bid/contract/dealer/schedule noun or rule enters the kernel. Existing generic actor/seat/action/effect/view/replay/RNG contracts suffice. |
| Earned `game-stdlib` | Narrow option-2 promotion. | Three shipped/planned close uses establish a stable pure selection/comparison kernel; all policy-bearing lifecycle remains local and both prior games conform in-gate. |
| Static typed content | Preserved. | TOML carries metadata/presentation/setup identity only; no schedule, hook, legality, scoring, visibility, or bot behavior. |
| Official-game evidence | Fully engaged. | Source notes, original rules/how-to, rule coverage, mechanics/ledger, competent-player work, bot docs, traces, replay, serialization, simulation, benchmark, UI, release checklist. |
| Determinism | Required. | Per-hand RNG derivation, deck/deal, public ordering, legal leaves, effects, serialization, bot tie-breaks, replay, hashes, standings all stable. |
| N-seat contract | Fully engaged. | 3–7 declaration, clockwise order, viewer matrix, exhaustive pairwise proof, public observer, seat-keyed simulator/outcome, surface budgets, every-viewer hashes. |
| Hidden-information safety | Required across every surface. | Hands and stock stay out of unauthorized views/actions/previews/diagnostics/effects/bots/exports/WASM/DOM/storage/logs/a11y/IDs. |
| Replay taxonomy | ADR 0004 reused. | Internal full traces may be omniscient native evidence; browser/public and seat-private exports are viewer-scoped observation histories. |
| Bot law | L0 + bounded L1; L2 deferred. | Authorized view/legal API only; no hidden-world sampling or forbidden search/learning. |
| UI interaction | Legal-only/effect-driven. | Rust supplies actions/previews/status/effects/outcome; React/SVG presents responsive 3–7-seat table, replay, a11y, reduced motion. |
| IP conservatism | Preserved. | Neutral Vow Tide identity, original prose/assets, no copied rules/trade dress/casino language, human release review. |
| Local-first | Preserved. | Human/bot/hotseat/replay/observer in static/local browser; no accounts/network service. |
| Agent discipline | Preserved. | Bounded tasks; no broad cleanup; failing-test validity/SUT/test analysis precedes fixes; valid tests never weakened. |

### 8.2 Third-use trick-taking primitive-pressure analysis and decision

#### Three-way comparison

| Mechanic shape | Plain Tricks — first use | Briar Circuit — second use | Vow Tide — third use | Stable repeated core | Material divergence |
| --- | --- | --- | --- | --- | --- |
| Follow-suit legal subset | Two-seat local hand; leader any card; follower must use led suit if held | Four-seat hand; same core nested under forced 2♣, first-trick point restriction, hearts-broken lead restriction | 3–7 seats; same core; any lead including trump; no Hearts exceptions | Given a held sequence and led suit, choose matching items if any, else all, preserving order | Lead policy, phases, diagnostics, ownership, card IDs, hidden-info surface, hand size |
| Trick winner comparison | Two plays; no trump; higher led-suit rank wins | Four plays; no trump; maximum led-suit rank wins | 3–7 plays; highest trump if present, else highest led rank | A pure winner-class/rank comparison over caller-projected suit/rank values | Trump presence, play count, captured meaning, scoring, effects, duplicate-card/tie variants |
| Winner leads next | Local state sets winner as next active seat unless round closes | Local state creates next trick led/active by winner unless hand closes | Local state does same unless hand closes | Winner seat is next leader | Phase/state types, close conditions, score/deal transitions, effects and replay fields |
| Deal/redeal/dealer rotation | Two short rounds, continuing RNG, fixed alternating round leader | Full 52-card four-seat hands, per-hand seed, rotating dealer, pass cycle, 2♣ holder opens | Variable N, variable H, public turn-up trump, hidden tail, rotating dealer, left-of-dealer opens/bids | Deterministic shuffle/allocation and seat-relative order are recurring concepts | RNG partitioning, deck use, leader source, pass/bid phases, stock/trump, schedule, terminal lifecycle |
| Private-hand projection | Two owner hands plus tail redaction | Four owner hands plus private pass staging | 3–7 owner hands plus large hidden stock | Generic existing viewer/effect/replay infrastructure | No need for a card-hand primitive; game-local categories differ |
| Scoring | Positive trick totals across two rounds | Penalty points, moon transform, threshold/tie continuation | Public numeric contract compared to trick result; exact bonus or zero | Public seat-keyed accounting after completed trick/hand | Meanings/formulas/terminal policy are incompatible |

#### Hard-gate option review

| Atlas option | Decision | Reason |
| --- | --- | --- |
| 1. Reuse existing promoted primitive | Rejected | No promoted trick-taking primitive exists in the current codebase (`crates/game-stdlib/src/lib.rs` exports only `board_space`). |
| 2. Promote narrow typed helper | **Selected** | Three close uses and Gate 18's admitted fourth pressure make a small pure selection/comparison boundary more valuable than a third/fourth local fork. The helper can be fully described without game policy and back-ported with preservation evidence. |
| 3. Explicitly defer/reject | Not selected for the core | This remained correct at Gate 16, but after a third use with trump and variable N, local comparator/follow implementations now risk semantic drift and inconsistent edge tests. Deferral would intentionally create a third duplicate immediately before Spades. |
| 4. Escalate to ADR | Not required | The helper stays in the already-authorized `game-stdlib` promotion lane and does not change kernel vocabulary, data policy, visibility taxonomy, Trace Schema, or public API semantics. Any discovered need to change those areas triggers a stop/ADR. |

**Decision:** choose option 2 and promote one narrow `game_stdlib::trick_taking` selection/comparison module. This is the single hard-gate outcome. The module's explicit non-goals resolve the wider trick-taking rows without creating a second policy framework: winner-leads mutation and deal/redeal remain caller behavior because they cannot be included without phase, seat, RNG, schedule, or scoring policy. They are documented anti-examples inside the promotion decision, not separate generic primitives.

#### Locked boundary

The helper may:

- inspect only caller-projected, copyable/equatable/ordered suit/rank keys;
- return stable input indices;
- express led-suit availability and trump/led winning class;
- be deterministic, side-effect free, allocation-bounded, and game-state agnostic.

The helper must not:

- define or export a shared card/deck/hand/seat/dealer/trump-source/trick-state type;
- inspect ownership, actor, phase, visibility, effect, replay, score, bid, contract, or terminal state;
- decide legal leads, first-trick exceptions, hearts-broken, trump-breaking, forced openings, or bid-dependent play;
- mutate active seat, capture cards, increment counts, deal, shuffle, rotate dealer, advance schedule, emit diagnostics/effects, or serialize views;
- accept policy flags that encode game variants;
- move any noun or behavior into `engine-core`;
- be called from TypeScript.

Examples:

- Plain Tricks follower hand contains led suit: helper returns only those stable indices.
- Briar Circuit is void in led suit: helper returns all indices, after which local first-trick point policy may further restrict them.
- Vow Tide trick contains a low trump and a high led-suit ace: comparator returns the trump index.

Anti-examples:

- “Must lead two of clubs,” “hearts cannot be led,” “dealer cannot make bids sum to H,” “winner becomes active,” “deal H cards starting left of dealer,” and “score 10+bid” are local behavior and must never enter the module.
- Duplicate-deck variants with equal ranked identical cards are outside the helper's promoted proof; stable first occurrence is deterministic but does not claim to implement second-played-wins house rules.

#### Back-port and preservation plan

1. Capture pre-change Plain Tricks and Briar Circuit command/action/effect/view/hash/trace/benchmark baselines.
2. Add helper tests, examples, anti-examples, and benchmark operations.
3. Replace only each game's local “matching led suit or all” and pure winner-index computation.
4. Keep local card types, ordering, diagnostics, legal-action construction, phase checks, effects, state mutation, scoring, visibility, and bots.
5. Run every prior crate/tool/WASM/UI contract.
6. Compare golden traces and hashes byte-for-byte where the repository currently treats them as stable.
7. If observable output changes, first determine whether an existing test is valid and whether the defect is in the system or test. A trace refresh is not a shortcut. Any intentional migration requires explicit reviewed scope; otherwise revert/refine the helper.
8. Update both game ledgers/inventories and central atlas. §10A remains `_None_` because all matching prior games migrate in-gate.

Required helper evidence:

- unit/property tests, examples, anti-examples, docs and misuse warnings;
- before/after native benchmarks;
- tests from all three games;
- unchanged prior traces/hashes or a stop for explicit migration authority;
- boundary check proving no `engine-core` noun growth;
- source review proving no behavior/static-data or TypeScript path.

### 8.3 Bidding/contracts first-use ledger stance

| Ledger field | Gate 17 entry |
| --- | --- |
| Mechanic shape | Sequential public numeric trick contract; dealer-last total-not-equal hook; exact contract-vs-result scoring |
| Status | `local-only` / first official use |
| Games exerting pressure | `vow_tide` only |
| Nearest non-uses | `secret_draft` hidden choices, `masked_claims` claim/reveal, `high_card_duel` face-down commitment; none compares a numeric bid to trick outcome |
| Relevant local files | `actions.rs`, `state.rs`, `rules.rs`, `scoring.rs`, `effects.rs`, `visibility.rs`, `bots.rs`, docs/tests/traces |
| What is new | Numeric range depends on current H; ordered public bids; dealer-only dynamic exclusion; exact result comparison; contract-relative bot/UI state |
| Decision | Implement locally and record first-use evidence; do not generalize |
| Why not `engine-core` | Bid, contract, trick, dealer, and score are mechanic/game nouns and policy |
| Why not `game-stdlib` yet | One implementation cannot establish a stable boundary; Gate 18 may compare Spades only after Vow Tide ships |
| Data/Rust impact | All formulas/order/legality in typed Rust; data carries identity/presentation only |
| Replay/hash impact | New game-local commands/effects/views under existing schema; no cross-game migration |
| Visibility | Submitted bids public; unsubmitted choice/private legal tree authorized only to actor; no hand/stock leak in diagnostics/explanations |
| Bot/UI impact | Bot uses own hand plus public facts/legal leaves; UI renders Rust legal bid buttons and hook explanation, never recomputes |
| Tests/benchmarks | Exhaustive H/prefix-sum hook properties, action-tree/validator equivalence, traces, bot legality, bid fanout/latency |
| Next review trigger | Gate 18 Spades or another official numeric trick-contract game; that second use compares semantics but normally remains local under atlas law |

No “contract” static-data object may carry scoring or legality expressions. No generic bid phase, scoring table, or hook callback is authorized.

### 8.4 FOUNDATIONS §12 stop conditions

Implementation stops and returns to reassessment if:

- any card/suit/rank/hand/trick/trump/bid/contract/dealer/schedule noun or policy is proposed for `engine-core`;
- the promoted helper grows beyond the two pure operations or acquires game-policy flags/state mutation;
- a prior matching game cannot be back-ported in-gate and someone proposes shipping the helper with silent debt;
- a back-port changes valid gameplay, action order, diagnostics, effects, visibility, replay, hashes, traces, bot behavior, or UI without explicit migration authority;
- bid legality/scoring, follow-suit, comparator, schedule, or visibility moves into static data, YAML, a DSL, or TypeScript;
- normal UI shows a clickable illegal bid/card and relies on later rejection;
- animation or replay causality is guessed from state diffs instead of Rust effects;
- any hand/stock/private candidate canary reaches an unauthorized viewer/export/WASM/DOM/storage/log/a11y/test artifact;
- a bot reads raw state, actual hidden worlds, deck tail, other hands, unredacted replay, or hidden-derived rankings;
- L2, search, Monte Carlo/ISMCTS/MCTS, ML, RL, or runtime LLM work begins;
- trace fields, hash meaning, replay taxonomy, or public API must change without accepted ADR authority;
- required 3–7 evidence, public UI, docs, or helper conformance is cut to preserve schedule;
- valid failing tests are deleted, weakened, skipped, renamed away, or reclassified before validity/SUT/test analysis.

## 9. Forbidden changes

This gate must not:

- modify `engine-core` vocabulary or responsibilities;
- add card, deck, hand, suit, rank, trick, trump, bid, contract, dealer, schedule, or game-scoring types to `engine-core`;
- add a shared card model, deck/shuffle/deal engine, trick phase machine, winner-leads turn policy, bid framework, contract scorer, or game outcome framework to `game-stdlib`;
- expand the promoted helper with Hearts, Oh Hell, Spades, seat-count, dealer, visibility, effect, scoring, or variant flags;
- leave Plain Tricks or Briar Circuit on matching local code after promotion, or open debt merely to reduce Gate 17 scope;
- alter shipped Plain Tricks/Briar Circuit behavior, traces, hashes, action paths, diagnostics, effects, visibility, bot policy, or public presentation except the behavior-preserving helper conformance;
- revisit River Ledger, Gate 15.1, Blackjack Lite/ADR 0006, or unrelated game work;
- change Trace Schema v1, replay/hash semantics, or ADR 0004's hidden-info export taxonomy;
- export an internal full trace, raw state, seed, hidden stock, or “all viewers” payload to the browser;
- add a WASM get-all-state/debug escape hatch or rely on CSS/client hiding;
- encode schedule, hook, follow-suit, trump comparison, scoring, visibility, or bot strategy in TOML, YAML, JSON behavior tables, selectors, callbacks, or DSLs;
- let TypeScript sum bids, remove the hook value, determine follow suit/trick winner, calculate scores/standings, or infer active/dealer/leader facts missing from Rust;
- add alternate schedules, trump methods, scoring systems, secret/simultaneous bids, bid changes, special one-card reveal rules, jokers, duplicate decks, teams, partnerships, or configurable house-rule matrices;
- introduce hosted multiplayer, accounts, databases, matchmaking, chat, ranking, tournaments, stakes, currency, chips, rake, or casino presentation;
- introduce MCTS, ISMCTS, Monte Carlo/rollout sampling, ML, RL, external solvers, determinization, actual-hidden-state sampling, or omniscient candidate scoring;
- claim L2 competence from L0/L1 or from a strategy document without an accepted evidence pack;
- copy rules prose, card art, app screenshots, commercial score sheets, branded table appearance, icons, fonts, or trade dress;
- replace React/SVG with Canvas/PixiJS absent the required profiling evidence and ADR path;
- perform broad workspace, engine, tool, WASM, shell, or sibling-game refactors;
- create ticket files as part of this spec-authorship deliverable;
- weaken, delete, skip, quarantine, or rewrite valid tests just to obtain green CI.

## 10. Documentation updates required

### 10.1 `specs/README.md`

When this spec is accepted and added:

- replace Gate 17's seed/unwritten link with this file;
- flip Gate 17 from `Not started` to `Planned`;
- retain Gate 16 `Done` and Gate 18 `Not started`;
- summarize the selected Vow Tide identity, 3–7 range, option-2 helper promotion/back-port, and bidding first-use local stance.

At implementation closeout:

- flip Gate 17 to `Done` only after every §6 criterion and §7 evidence class passes;
- record completion date and concise evidence;
- archive the completed spec/tickets under the normal workflow without erasing history.

### 10.2 `docs/SOURCES.md` and source/IP records

Add a Vow Tide / Oh Hell rules-family entry with:

- Pagat and secondary rule sources, consulted date, facts used, and variants excluded;
- selected 10/8/7 maximum derivation and down/up schedule;
- turn-up trump, bid order/hook, exact `10+bid`/zero miss scoring, fixed schedule, and co-winner tie decision;
- deliberate immutable-bid and no-extra-tie-hand deviations;
- Vow Tide neutral-name rationale;
- original prose/assets/no-copy statement;
- external implementation/strategy sources classified as prior art only;
- human IP/release review receipt at closeout.

### 10.3 `docs/MECHANIC-ATLAS.md`

Update the §10 rows for:

- `follow-suit legality` → third use resolved by promoted narrow helper; list three games and same-gate conformance;
- `trick resolution / led-suit comparator` → promoted helper with optional caller-provided trump and stable index output;
- `trick-winner-leads turn order` → third-use reviewed as an explicit anti-example/non-goal inside the selected narrow promotion; local phase orchestration remains required;
- `deal rotation / trick-round redeal` → third-use reviewed as an explicit anti-example/non-goal; local deal/schedule/RNG policy remains required;
- new `numeric trick bid / contract-vs-result / last-bidder hook` row → `vow_tide`, first-use `local-only`, next review Gate 18 or another close use.

Update §9A armed interlock text to record Gate 17's resolution and arm Gate 18 only for remaining partnership/team and second-use bidding pressure.

§10A must remain `Current debt: _None_` after closeout. Because this spec requires same-gate back-port, no promotion-debt row should be added. If implementation discovers a genuine conformance impossibility, stop and reassess rather than quietly editing §10A.

### 10.4 Game-local and prior-game mechanic docs

Complete every file in §4.4 and update:

- `games/plain_tricks/docs/PRIMITIVE-PRESSURE-LEDGER.md` and `MECHANICS.md` with helper conformance and preserved evidence;
- `games/briar_circuit/docs/PRIMITIVE-PRESSURE-LEDGER.md` and `MECHANICS.md` with Gate 17 decision/back-port receipt;
- prior `RULE-COVERAGE.md`, `BENCHMARKS.md`, or implementation-admission records only where the conformance process requires explicit evidence; do not rewrite unrelated prose.

### 10.5 `apps/web/README.md` and catalog-enforced surfaces

At web exposure, update all three required closeout surfaces:

1. intro catalog list — add Vow Tide;
2. Shell Surface renderer list — add `VowTideBoard`/current renderer mapping;
3. Smoke Layers `smoke:e2e` list — add the Vow Tide smoke chained by the package script.

`node scripts/check-catalog-docs.mjs` must pass. This reconciliation is part of Gate 17, not an aftermath cleanup.

### 10.6 Public rules, manifests, registration docs, and checks

Update as required:

- root workspace/Cargo lock and `ci/games.json`;
- WASM constants/catalog/game modules/API snapshots;
- simulator help/available-game output and seat-range diagnostics;
- native tool registration entries and manifests;
- web catalog/renderer/icon/rules manifest/e2e/package smoke chain;
- public rules copy from original `HOW-TO-PLAY.md`;
- outcome explanation registry/checks;
- player-rules/catalog/CI/presentation/doc-link checks.

### 10.7 Closeout evidence

Record:

- exact command log and environment;
- trace inventory with version/hash notes;
- exhaustive viewer matrix result and browser sample plan result;
- helper before/after/back-port receipt;
- simulation summaries by every seat count;
- benchmark report and calibrated floors;
- source/IP/no-copy review;
- public-release checklist;
- no open promotion debt;
- status/archival receipt.

## 11. Sequencing

### 11.1 Admission evidence

Gate 17 is admitted because:

- Gate 16 is `Done`;
- the atlas debt register is empty;
- Gate 17 is the lowest non-`Done` active-epoch unit;
- ROADMAP §15 names it next.

### 11.2 Requirements-first order

Implementation order is mandatory:

```text
spec acceptance
  -> /reassess-spec
  -> sources + original rules + variant + rule IDs + admission receipt
  -> third-use ledger decision, helper, tests, benchmarks, prior-game back-ports
  -> Vow Tide setup/schedule/deal state
  -> bidding first-use ledger + bid behavior
  -> trick behavior through promoted helper
  -> scoring/outcome
  -> visibility/effects
  -> replay/serialization/traces
  -> bots/simulation
  -> tools/WASM
  -> web/public rules/accessibility/e2e
  -> benchmark calibration
  -> docs/release/status closeout
```

`/spec-to-tickets` follows accepted reassessment. The spec itself does not instantiate tickets.

### 11.3 Successor rule

Gate 18 — Spades — remains the successor. It may be admitted only after:

- Gate 17 is `Done`;
- the promoted trick helper is used by every matching prior game with no open debt or accepted exception;
- bidding's first-use local evidence is complete for Gate 18's second-use comparison;
- all 3–7-seat no-leak, simulation, benchmark, UI, and official-game obligations close.

Gate 18 must treat partnerships/team outcomes as new pressure and must not fold team identity into generic seat identity or assume Vow Tide's individual scoring is reusable.

## 12. Assumptions

| Assumption | One-line-correctable statement |
| --- | --- |
| Repository baseline | Authored against repository state as of 2026-06-21; references resolve against the current tree. |
| Identity | Public name is Vow Tide and internal id is `vow_tide`, subject to required human IP review rather than automatic clearance. |
| Default seats | Default public setup is 4 while every count 3–7 is equally official. |
| Schedule | V1 uses maximum-down-to-one-then-up, with one one-card hand and `K=min(10,floor(51/N))`. |
| Trump | Every hand uses the next undealt face-up card as trump indicator; no no-trump hand exists in v1. |
| Bids | Bids are sequential, public on acceptance, immutable, and dealer-last with the exact hook. |
| Scoring | Exact bid scores `10+bid`; every miss scores zero; no consolation/negative points. |
| Terminal/tie | Fixed schedule ends the match; equal high scores are co-winners with competition ranks. |
| Primitive promotion | The narrow two-operation helper can back-port without observable change; if not, implementation stops for reassessment. |
| Bots | L0 and bounded L1 ship; L2 remains unimplemented and not admitted. |
| Tool registration | Resolved at reassessment: `simulate`, `replay-check`, `fixture-check`, and `rule-coverage` are all hard-coded per-game registries (`resolve_game()` / `GAME_*` dispatch), not generic-discovery tools. Each requires an explicit `vow_tide` registration entry, and `fixture-check` additionally requires the crate to export `vow_tide::load_manifest()` / `vow_tide::load_variants()`. No `not applicable` receipt applies. |
| Schema/API | Trace Schema v1 and current WASM operation groups are sufficient; a real gap triggers the ADR path. |

# Appendix A — Normative rule and coverage skeleton

The game-local `RULES.md` may split or add subordinate rules, but it must preserve these stable top-level identifiers and meanings. Renaming an identifier after traces exist is a rules/coverage migration, not editorial cleanup.

| Rule ID | Normative rule | Minimum direct evidence |
| --- | --- | --- |
| `VT-IDENTITY-001` | The only Gate 17 official variant is `vow_tide_standard`, rules version `vow-tide-rules-v1`, for independent seats. | Manifest/variant validation, setup snapshot, rules-version diagnostic. |
| `VT-SEATS-001` | Supported seat counts are exactly 3, 4, 5, 6, and 7; default 4; supplied order is clockwise order; trace ids are `seat_0`…`seat_6` and fallback public labels are `Tide 1`…`Tide 7`; all other counts are rejected. | Unit tests for every accepted count and below/above/irregular rejects; WASM catalog snapshot; setup e2e. |
| `VT-CARDS-001` | One standard 52-card, four-suit, ace-high deck is constructed without duplicates or jokers. | Deck-conservation unit/property tests; serialization canonical-order test. |
| `VT-SCHEDULE-001` | `K=min(10,floor(51/N))`; hand sizes are `K..1..K` with the one-card hand once; total hands `2K-1`. | Exhaustive properties for N=3..7; schedule traces at descending floor and ascending restart. |
| `VT-DEALER-001` | `seat_0` is initial dealer; dealer advances one seat clockwise after every resolved hand and never during a hand. | Dealer-cycle property and full-schedule trace. |
| `VT-DEAL-001` | Each hand is independently and deterministically shuffled under the documented seed partition; cards are dealt singly clockwise beginning left of dealer. | Same-seed/same-state property, different-seed coverage, per-seat-count fixtures, conservation. |
| `VT-TRUMP-001` | The next undealt card is a public, non-playable trump indicator; its suit is trump; every remaining undealt card is hidden stock. | Trump fixture/trace, card-conservation test, observer/seat no-leak checks. |
| `VT-BID-ORDER-001` | Bidding is sequential and public, begins left of dealer, proceeds clockwise, and ends with dealer; each seat bids once. | Bid-order rule test/trace, wrong-seat and duplicate/wrong-phase diagnostics. |
| `VT-BID-RANGE-001` | A normal legal bid is every integer from zero through current hand size inclusive. | Property across H=1..10; action-tree/validator equivalence; malformed/out-of-range diagnostics. |
| `VT-HOOK-001` | On the dealer's bid only, `H-S` is excluded when in `0..=H`; all other range-valid values remain legal. | Exhaustive H/prefix-sum property; under-bid and over-bid traces; hook UI smoke. |
| `VT-BID-PUBLIC-001` | An accepted bid is immediately public and immutable. An unsubmitted choice has no authoritative state and cannot leak through preview, storage, logs, or replay. | View/effect tests, replay export, DOM/storage/log canaries; bid-change diagnostic is `not applicable` because no change action exists. |
| `VT-FIRST-LEAD-001` | The seat left of dealer leads the first trick of each hand and may lead any held card, including trump. | Rule tests and first-lead trace. |
| `VT-FOLLOW-001` | A follower with led suit must play it; a void follower may play any held card. | Promoted-helper unit/property tests, game rule tests, forced/void traces. |
| `VT-TRICK-WIN-001` | Highest trump wins when any trump is played; otherwise highest led-suit card wins; off-suit non-trumps cannot win. | Promoted-helper tests, exhaustive projected-play properties, trump/led/off-suit traces. |
| `VT-NEXT-LEAD-001` | The trick winner leads the next trick unless the hand has ended. | State-transition test and winner-leads trace. |
| `VT-HAND-END-001` | A hand ends after exactly H tricks; every seat then has zero cards and all H×N dealt cards are in completed tricks. | Conservation property and shortest/maximum-hand traces. |
| `VT-SCORE-001` | Exact bid adds `10+bid`; underbid and overbid each add zero; cumulative scores never decrease. | Score unit/property tests and exact-zero/exact-positive/under/over traces. |
| `VT-HAND-ADVANCE-001` | Hand result/history is recorded atomically before dealer/schedule advances; next hand starts at bidding. | Transition ordering/effect tests; descending-to-one and one-to-ascending traces. |
| `VT-TERMINAL-001` | The match ends after the final scheduled hand, never by a score target or action cap. | Full-match replay at each seat count and terminal schedule property. |
| `VT-STANDINGS-001` | Highest cumulative score wins; equal totals co-win; ranks use competition ranking with stable seat order only for serialization/display. | Unique/two-way/multi-way tie tests and terminal outcome traces. |
| `VT-VIEW-001` | Public observer sees only public facts; seat viewer sees those facts plus that seat's hand/legal controls; no viewer sees hidden stock identity/order or another hand. | Exhaustive N-seat pairwise matrix, public observer checks, browser canaries. |
| `VT-EFFECT-001` | Semantic effects are viewer-filtered independently from state views and never contain hidden identities for an unauthorized viewer. | Effect-filter unit tests, trace hashes, WASM/browser payload inspection. |
| `VT-REPLAY-001` | Internal replay is deterministic; browser import/export is viewer-scoped observation history under ADR 0004 with no retroactive reveal. | Full/internal replay, observer and every-seat export/import, historical-view tests. |
| `VT-BOT-001` | L0 and L1 submit only Rust legal leaves from authorized information; explanations are viewer-safe; forbidden search/sampling/learning is absent. | Bot input audit, many-seed legality/no-leak, explanation canaries, simulation. |
| `VT-OUTCOME-001` | Rust supplies seat-keyed hand history, exact/miss totals, cumulative score, rank, and co-winner state for the shared outcome surface. | Outcome serialization, explanation registry check, terminal UI smoke. |
| `VT-BOUNDARY-001` | Rules stay in typed Rust; static data is content/metadata; TypeScript presents only; `engine-core` remains mechanic-noun-free. | Boundary script, source review, static-data rejection tests, web source audit. |

The coverage matrix must include one row per rule above and may never substitute a broad full-match trace for the direct legality/property evidence a rule requires.

# Appendix B — Proposed Rust state, action, effect, view, and outcome contract

## B.1 State phases and transition ownership

A concrete implementation may choose equivalent Rust names, but the state machine must expose no ambiguous “miscellaneous” phase:

```rust
pub enum Phase {
    Bidding,
    PlayingTrick,
    Terminal,
}
```

Hand scoring and new-hand setup are deterministic internal transitions performed atomically after the last trick. They do not create an artificial player decision. Effects may expose the ordered consequences of that transition.

The game-local state owns, at minimum:

- versioned game/variant/rules/data identity and freshness token;
- ordered 3–7 seat vector and deterministic seat-index helpers;
- match seed plus documented per-hand RNG derivation inputs;
- complete immutable hand-size schedule, current schedule index, current hand size, and dealer;
- phase and active seat;
- game-local deck/card identities, hidden stock order, public trump indicator, and per-seat private hands;
- public accepted bids as a seat-keyed `Option<u8>` collection;
- current trick with leader, led suit, ordered public plays, and active player;
- completed public trick history for the current hand, per-seat tricks taken, and card-conservation counters;
- cumulative score, immutable completed-hand summaries, and optional terminal outcome;
- only those derived caches that are deterministic, reconstructible, and covered by serialization/hash tests.

The state must not retain a browser user's hovering bid/card, an unsubmitted bid, client animation progress, DOM identity, or hidden-derived bot ranking as authoritative game state.

### B.1.1 Transition order

The final play of a non-final trick performs this deterministic order:

1. validate and remove the card from the actor's hand;
2. append the public play;
3. resolve the winner through the promoted comparator;
4. record completed trick and increment the winner's trick count;
5. clear current-trick plays;
6. set winner as next leader/active seat;
7. increment freshness once according to the crate's documented command contract;
8. emit filtered semantic effects in stable order.

The final play of a hand performs steps 1–4, then:

1. calculate every seat's exact/miss result and hand addition from the pre-transition bids/counts;
2. append one immutable completed-hand summary;
3. update cumulative scores;
4. if schedule complete, construct terminal standings and enter `Terminal`;
5. otherwise rotate dealer, advance schedule, derive the next hand RNG stream, deal, expose trump, clear bids/counts/tricks, set active bidder left of dealer, and enter `Bidding`;
6. emit hand-result, score, dealer/schedule, deal/trump, and next-bidder effects in a documented stable order.

No TypeScript callback participates in either sequence.

## B.2 Action paths and legal trees

The initial stable path families are:

```text
bid/<decimal_u8>
play/<stable_card_id>
```

During `Bidding`, the active actor receives one root “Bid” branch whose leaves are Rust-generated legal values in ascending numeric order. For a non-dealer, the leaves are `0..=H`. For the dealer, the one hook-forbidden value is absent when applicable. Metadata may include public, Rust-authored `hand_size`, `current_bid_total`, `is_dealer`, and `hook_forbidden_bid`; metadata must not include cards or hidden stock.

During `PlayingTrick`, the active actor receives one “Play” branch whose leaves are legal owned cards in canonical hand order. A public observer, inactive seat, wrong viewer, terminal state, or wrong phase receives no private decision tree.

The validator independently checks the same rules and must not trust membership alone. Legal-tree/validator equivalence properties must prove:

- every emitted leaf validates in the same state;
- every typed action that validates appears as a leaf;
- no stale, wrong-seat, wrong-phase, unowned, hook-forbidden, or off-suit action validates;
- action ordering is deterministic and independent of map iteration.

## B.3 Diagnostic code floor

Exact wording remains game-local original prose, but the following stable codes are required unless reassessment demonstrates an established repository naming constraint:

| Code | Trigger | Viewer-safety requirement |
| --- | --- | --- |
| `VT_WRONG_RULES_VERSION` | Unsupported rules version | Names expected/public version only. |
| `VT_STALE_COMMAND` | Freshness mismatch | No hidden change detail. |
| `VT_INVALID_SEAT_COUNT` | Count outside 3–7 | States accepted range/default. |
| `VT_UNKNOWN_SEAT` | Actor not in declared seats | No private seat mapping. |
| `VT_WRONG_SEAT` | Known but inactive actor | May name active public seat; never its hand. |
| `VT_TERMINAL_STATE` | Action after terminal | Public. |
| `VT_WRONG_PHASE` | Bid during play or play during bidding | Public phase only. |
| `VT_MALFORMED_ACTION` | Path cannot parse | No echoed untrusted hidden payload. |
| `VT_BID_OUT_OF_RANGE` | Bid outside `0..=H` | Public H. |
| `VT_BID_HOOK_FORBIDDEN` | Dealer submits exact excluded value | Public bids/H only. |
| `VT_BID_ALREADY_SET` | Duplicate bid command | Public accepted-bid status only. |
| `VT_UNKNOWN_CARD` | Card id does not parse | Do not disclose deck/stock membership. |
| `VT_CARD_NOT_OWNED` | Parsed card absent from actor hand | Names submitted id only if actor-authorized; observer export redacts private diagnostic details. |
| `VT_MUST_FOLLOW_SUIT` | Actor breaks led suit while holding it | May name public led suit; must not list hidden hand. |
| `VT_ACTION_UNAVAILABLE` | Typed action not currently legal | Generic, viewer-safe fallback. |

Invalid-action effects/logs must carry the filtered diagnostic rather than the full internal error object.

## B.4 Semantic effects

Public effects may include:

- match initialized with declared seat count and schedule summary;
- dealer assigned/rotated;
- hand started with hand index/size;
- trump indicator revealed;
- bidder became active;
- bid accepted with seat/value and updated public total;
- dealer hook constrained the legal set, without client-side calculation;
- play accepted with public card/seat;
- trick captured with winning seat/card and updated public trick count;
- hand result with each seat's bid, tricks, exact/miss, addition, and cumulative score;
- schedule advanced;
- match completed with competition-ranked standings/co-winners.

Seat-private effects may include only:

- cards dealt to that owner;
- own legal-control/preview detail authorized by the action-tree contract;
- own bot explanation/candidate rationale where that mode exposes it.

There is no effect for the identity/order of hidden stock, another seat's hand, an unsubmitted bid, a hidden bot feature, or a future shuffle result. Animation presenters consume effects; they may not infer a missing winner, hook, score, or schedule transition from state diffs.

## B.5 Viewer model

Rust produces exactly these viewer classes:

| Viewer | Receives | Must never receive |
| --- | --- | --- |
| `PublicObserver` | Public seat/dealer/schedule/trump/bids/trick/score/outcome facts and public effects/history | Any unplayed hand, hidden stock identity/order, private legal tree/preview, raw seed/state, seat-private bot detail. |
| `Seat(seat_i)` | Public facts plus `seat_i` hand, `seat_i` legal tree when active, and viewer-authorized private effects | Any `seat_j` hand for `j != i`, stock identity/order, private tree/preview for another seat, omniscient bot fields. |
| Internal native/test authority | Full deterministic state/commands/effects needed to validate and replay | Not exportable through normal WASM/browser APIs. |

Because bids are sequential-public in this variant, there is no hidden committed-bid object. “Bid privacy” applies only to a user's unsubmitted UI choice, which is not authoritative Rust state and must not be persisted or logged.

## B.6 Rust-authored outcome model

For every seat, the terminal outcome supplies:

- stable seat id/label;
- cumulative score and competition rank;
- winner/co-winner boolean;
- hands played;
- exact contracts made and missed;
- successful zero bids;
- total tricks bid and taken;
- per-hand immutable rows: hand index/size, dealer, trump suit, bid, tricks, exact/miss, addition, cumulative before/after;
- optional concise decisive facts chosen from public data, such as “made 14 of 19 contracts.”

At non-terminal hand boundaries, the same shape may be projected as current standings without declaring a winner. The shared outcome/explanation surface must render the supplied ranking and facts; it must not sort/re-rank by a TypeScript score calculation.

# Appendix C — Bot policy and strategy-evidence contract

## C.1 L0 random legal

L0 is mandatory at every supported seat count:

- request the current legal leaves for the bot's authorized seat viewer;
- sample uniformly with declared deterministic bot RNG;
- submit the normal command envelope through validation;
- return a viewer-safe explanation such as “Random legal bid from 6 choices” or “Random legal card from 3 choices”;
- never read internal state, another hand, hidden stock, future RNG, an unfiltered action tree, or an internal full replay.

An empty legal set for an active non-terminal bot is a test failure, not an invitation to invent an action.

## C.2 Bounded L1 rule-informed policy

L1 is included because a bidding game presented only with arbitrary opponents would undersell the gate. It remains a small authored policy, not an L2 competence claim. The Haskell prior-art implementation demonstrates two useful non-search ideas—estimate a bid from own high/trump cards and switch play posture according to whether the current trick count is below or at the bid—but its probability wording is not adopted, and no sampled hidden world is permitted.[^E3]

### C.2.1 Authorized inputs

L1 may read only:

- its seat-private projected hand;
- the public trump indicator/suit;
- current hand index/size, dealer, active seat, public accepted bids, public trick, completed public plays/trick winners, per-seat public trick counts, and scores;
- its own accepted bid and current tricks taken;
- its own Rust-authored legal leaves and viewer-safe metadata;
- public observation/effect history already authorized to the seat;
- declared bot RNG solely for documented final ties.

It may maintain only deterministic memory derivable from those authorized observations. It may not infer by reading hidden fields, inspect cards merely because they are present in native state, enumerate or sample possible deals, determinize the stock, call an external solver, or emit hidden-derived candidate scores.

### C.2.2 Bidding policy

The initial L1 bid is a deterministic, documented estimate over its own hand only. A recommended bounded feature set is:

1. count top trump controls, with ace strongest and successively discounted lower trump ranks;
2. count off-trump aces and backed high cards using only cards in the bot's hand;
3. reduce unsupported kings/queens as seat count grows or when the suit is long in the bot's own hand;
4. clamp and round the resulting control estimate to an integer in `0..=H`;
5. choose the legal bid nearest that estimate;
6. when the hook removes the nearest value, choose the nearest remaining value, preferring the lower contract on an equal-distance tie;
7. break any remaining exact tie by stable numeric order, not hidden-state RNG.

The exact weights are evidence-bearing authored parameters in `AI.md`, not static game rules and not TOML behavior. They require scenario tests at H=1, H=K, every seat count, obvious zero/one/high-control hands, and both directions of hook adjustment. Public earlier bids may be named in the explanation, but the policy must not claim they reveal exact opponent cards.

Example safe explanations:

- “Bid 2 from two own-hand controls; 2 was legal.”
- “Estimated 3; dealer hook removed 3, so chose the nearest lower legal bid, 2.”

Unsafe explanations include “Seat 4 has no clubs,” “the ace is still in the stock,” or any probability derived from sampled hidden deals.

### C.2.3 Play policy

Let `needed = max(0, own_bid - own_tricks_taken)`.

When `needed > 0`:

- if following, choose the lowest-ranked legal card that is currently winning the public trick, when one exists;
- otherwise choose the lowest legal card under canonical suit/rank/id order;
- when leading, prefer a documented own-hand control that is most likely *by static rank/trump class only* to secure a trick, while preserving stronger controls when multiple choices are equivalent;
- never call a rollout or estimate from hidden worlds.

When `needed == 0`:

- if following, prefer a legal card that is currently losing; among losing cards, discard the highest future-risk card under a documented own-hand-only ordering;
- if every legal card currently wins, play the lowest winning card;
- when leading, prefer the lowest non-trump card, then the lowest trump if forced by hand contents.

If the bot has already exceeded its contract, the hand score is irrecoverably zero under the locked scoring rule. The v1 L1 then follows the same avoid-taking policy rather than introducing kingmaking logic. Opponent-denial strategy is deferred to any future L2 evidence pack because it is score-table dependent and harder to explain without hidden-state overreach.

Every selected card still passes through the normal validator. “Currently winning” is calculated against the public trick using the same promoted pure comparator; it is not a prediction that the trick will remain won after unseen players act.

## C.3 Strategy evidence and future L2 gate

`COMPETENT-PLAYER.md` must cover, with sources and original analysis:

- bid calibration by trump strength, top cards, suit length, void/short-suit potential, hand size, and seat count;
- how the dealer hook changes contract selection without changing hand strength;
- the difference between securing needed tricks and shedding winners after making a contract;
- lead/follow position, current winner, cheapest-winning play, and conserving controls;
- zero-bid risk, especially at small H;
- public bid/trick/score awareness that does not claim hidden cards;
- late-schedule score posture and the distinction between maximizing own exact probability and deliberately setting an opponent;
- novice traps: bidding raw aces/trumps without discounting, forgetting the hook, taking an unnecessary trick after making the bid, and assuming undealt cards are known.

`BOT-STRATEGY-EVIDENCE-PACK.md` ships with status **not admitted / intentionally deferred**. L3 remains `not applicable` because the game is imperfect-information. A later L2 requires all of:

- accepted competent-player taxonomy and a fixed scenario corpus;
- exact authorized input and memory schema;
- deterministic priority vector/tie-break contract;
- 3–7-seat evaluation against L0 and L1 with fixed seed manifests;
- calibration evidence separated by hand size and seat count;
- legality, no-leak, explanation, replay, and benchmark receipts;
- explicit evidence that no MCTS, ISMCTS, Monte Carlo/rollout, determinization, ML, RL, runtime LLM, or actual-hidden-state sampling is used.

The external Scala implementation is useful as variant prior art—ascending then descending hand sizes, random turned trump, a total-bid constraint, and contract-relative scoring—but its special one-card visibility, negative miss score, networking, and architecture are deliberately not imported.[^E4]

# Appendix D — Browser interaction, layout, and accessibility acceptance details

## D.1 Variable-seat table and information hierarchy

The renderer must support 3–7 seats without a separate rules implementation per count:

- use the shared `SeatFrame`/seat-label contract and Rust-projected clockwise order;
- keep the local viewer's hand and legal controls in the primary focus region;
- arrange opponents around a responsive ring/rail, collapsing decorative detail before seat identity, dealer/active/leader status, bid, tricks, or score;
- show public trump indicator, current trick, current hand size, hand index/total, direction on the down/up schedule, and next scheduled size;
- provide a compact public bid rail in clockwise order and a score/history table keyed by the same stable seat labels;
- allow score history to scroll independently on narrow screens while preserving row/column headers and the current hand summary;
- never infer dealer, active bidder/player, leader, schedule direction, exact/miss, rank, or co-winner in TypeScript.

At seven seats and a ten-card hand is impossible by rule; the renderer must nevertheless size from Rust-projected current hand and seat count rather than hard-coded four-seat/ten-card assumptions.

## D.2 Bidding interaction

- The active seat receives only Rust-emitted legal bid buttons/leaves; the hook-forbidden value is not an actionable control.
- The bid group has a programmatic label containing current hand size, for example “Choose your contract for a 6-trick hand.”
- Keyboard users can tab to the group and activate any legal value with standard button controls; no pointer drag, wheel-only picker, or timing gesture is required.
- The current public bid total and dealer-last rule are explanatory status, not a client legality calculation.
- A Rust-authored `hook_forbidden_bid` may be displayed as contextual explanation when public and relevant; the client must not derive it by summing bids.
- Accepted bids are announced and immediately moved to the public bid rail. An unsubmitted focused value is not persisted to local storage, analytics, logs, replay, or another viewer.
- There is no “edit bid” control in the locked variant.

## D.3 Card play and trick feedback

- Own cards are semantic buttons/list items with accessible names including rank, suit, and trump status where relevant; suit is not communicated by color or glyph alone.
- Only Rust-legal cards are actionable. Learning mode may display a Rust-supplied safe disabled reason, but the default legal-only surface must not offer a knowingly illegal card and depend on rejection.
- The current trick is an ordered semantic list that names play order and seat; the winner announcement is driven by the trick-captured effect.
- Animation may move a played card and collect a trick, but reduced-motion mode presents the same semantic result immediately.
- Focus after a play moves to a stable public status/current-trick region or the next viewer handoff, never to a removed hidden node.

## D.4 Score and schedule presentation

The public score sheet must expose, in text as well as visual styling:

- each seat's cumulative score;
- current-hand bid and tricks taken;
- completed-hand exact/miss status and point addition;
- current competition rank when available;
- all schedule rows or an accessible progressive history with hand size and dealer;
- terminal co-winners without inventing a single tiebreak winner.

The 19-hand maximum is a real larger-surface test. Virtualization is allowed only if semantic headers, keyboard navigation, replay selection, and historical access remain correct; a simple scrollable semantic table is preferred until profiling proves otherwise.

## D.5 Viewer handoff, observer, replay, and no-leak

- Hotseat viewer change first renders a neutral handoff screen, removes the prior private subtree, then requests/renders the next authorized Rust view.
- Private hands may not coexist in DOM, React state, serialized props, accessibility trees, test ids, console output, local/session storage, animation queues, or screenshots hidden by CSS.
- Observer mode renders public card backs/counts only where a count is public; card IDs must not appear in `key`, `data-*`, alt text, labels, styles, or test fixtures.
- Replay scrubbing requests the selected viewer's historical observation at that checkpoint. It must not project final-state knowledge backward or reveal a hand after it later becomes empty.
- Import validation rejects an internal full trace at the public viewer-scoped import lane unless the existing dev-only whitelist explicitly authorizes the operation outside normal UI.

## D.6 Accessibility floor

The implementation follows the repository UI contract plus W3C guidance for native semantics, accessible names, keyboard behavior, focus visibility, status announcements, and non-text alternatives.[^E5][^E6]

Acceptance includes:

- complete keyboard play for setup, bid, card play, score/history, replay, rules, viewer handoff, and outcome;
- visible focus and logical order that does not traverse hidden opponent cards;
- text labels for suits/trump/exact/miss/dealer/active/leader states; no color-only communication;
- accessible names for icons and cards, or empty/hidden alternatives for purely decorative art;
- polite live announcements for bid accepted, card played, trick winner, hand exact/miss, score addition, dealer/schedule advance, and terminal standings;
- no live announcement of another seat's hidden hand or any unsubmitted choice;
- target sizes and spacing consistent with WCAG 2.2 guidance, with larger card/bid targets preferred for the primary game controls;[^E7]
- `prefers-reduced-motion` support for all nonessential movement;
- zoom/reflow testing at the largest seat/history surface without loss of controls or overlap;
- an e2e no-leak/a11y checklist entry for 3-seat, 7-seat, observer, hotseat, and replay modes.

# Appendix E — Research and source notes

## E.1 Rules-source reconciliation

| Topic | Source landscape | Vow Tide resolution |
| --- | --- | --- |
| Player count/deck | Pagat gives 3–7 with a standard 52-card ace-high deck; Trickster's public implementation describes four seats but the same base deck | Adopt Pagat's 3–7 range, reinforced by the roadmap; one 52-card deck, no jokers. |
| Maximum hand size | Pagat gives 10 cards for 3–5 seats, 8 for 6, 7 for 7 because of deck capacity | Adopt exactly and make the derivation explicit as `min(10,floor(51/N))`, reserving the turn-up card. |
| Schedule | Pagat's baseline descends to one then ascends; it lists ascending-first, one-way, duplicate endpoint, and larger-max variants. Trickster offers four patterns and documents `10..1..10` | Adopt maximum-down-to-one-then-up, one one-card hand. Exclude all configurable patterns in v1. |
| Trump | Baseline turns the next undealt card face up; fixed rotation and no-trump variants are common | Adopt turn-up trump every hand. The hand-size cap guarantees an indicator exists. |
| Bid order/range | Both core sources start left of dealer, proceed to dealer, and allow zero through H | Adopt. |
| Bid changes | Pagat permits changing a bid only before the next player bids; many digital rules treat acceptance as final | Deliberate Rulepath simplification: accepted bids are immutable, which improves deterministic command/replay semantics and removes a timing-sensitive edit window. Document as a deviation. |
| Hook | Pagat and Trickster define the dealer-last prohibition against making total bids equal H; hookless and simultaneous variants exist | Hook is mandatory; exact formula is `H-S`; no simultaneous bidding. |
| First lead/play | Both sources put first lead left of dealer, require following suit, permit any discard when void, and use highest trump else highest led; Pagat permits leading trump | Adopt without trump-breaking or first-trick exceptions. |
| Scoring | Pagat identifies `10+bid` exact / zero on miss as the simplest Blackout/Blob form and `tricks+10 exact` as perhaps most widespread; many penalty variants exist. Trickster uses trick points plus exact bonus | Adopt the simpler exact-or-zero form because it makes contract success the sole hand objective and gives clean outcome explanations. This is a deliberate choice among documented common variants, not a claim of universal canon. |
| End/tie | Sources end after the selected schedule; Pagat notes optional extra maximum hands on a tie | End after the fixed schedule; tied leaders co-win. Exclude extra hands to keep replay length fixed and avoid dealer/seat advantage as an arbitrary tiebreak. |
| One-card visibility | Some house variants show opponents' cards while hiding one's own | Excluded. Normal owner-only hand visibility applies at every H. |

All game-local rules prose must be newly authored from the reconciled facts. No source's sequence, examples, score sheet, images, UI, or trade dress may be copied.

## E.2 Strategy and implementation prior art

- The Haskell bot README is external evidence that simple authored policies often separate bidding from contract-relative play and adjust the dealer's bid for the hook.[^E3] Vow Tide borrows no code and replaces “probability of winning” with deterministic current-trick/own-hand categories to stay inside the no-sampled-world law.
- The Scala/Electron project is external variant/UI prior art: it describes a changing up/down schedule, random public trump, total-bid constraint, exact-contract scoring, score history, and a special one-card UI.[^E4] Vow Tide deliberately rejects its negative miss score, special one-card visibility, networking, chat, and architecture.
- The Leiden bachelor thesis treats bidding and card play as separate decision engines and describes a rule-based player whose posture changes according to tricks already taken versus the bid.[^E8] That supports the spec's contract-relative L1 decomposition, but its Monte Carlo and tree-search agents are negative prior art for Rulepath public v1/v2 and are explicitly excluded.
- These external repositories and papers are not target-repository evidence. Their source identity has no bearing on exact-commit fetch provenance for Rulepath.
- No external implementation authorizes a generic Rulepath trick engine, bidding framework, private-state access, browser legality, or prohibited search method.

## E.3 External source list

[^E1]: John McLeod, Pagat, “Oh Hell! — Card Game Rules,” <https://www.pagat.com/exact/ohhell.html>, consulted 2026-06-21. Used for the 3–7 range, 52-card/ace-high deck, 10/8/7 maxima, down-and-up baseline, rotating dealer, single-card deal, turn-up trump, bid order/range/hook, first leader, follow-suit/trump winner, simple exact-or-zero scoring, and documented house variants.
[^E2]: Trickster Cards, “Oh Hell Basics,” <https://www.trickstercards.com/help/oh-hell/>, consulted 2026-06-21. Used as a second rules-family reference for changing hand patterns, turned trump, dealer-last hook, first leader, follow-suit/trump winner, trick-points-plus-bonus scoring contrast, and fixed-schedule termination.
[^E3]: Harsil S. Patel, `harsilspatel/oh-hell` README, <https://github.com/harsilspatel/oh-hell>, consulted 2026-06-21. External open-source bot prior art only: own high/trump-card bidding, hook adjustment, and contract-relative play posture. No code, probability model, hidden-state access, or architecture is imported.
[^E4]: Sherpal, `sherpal/oh-hell-card-game` README, <https://github.com/sherpal/oh-hell-card-game>, consulted 2026-06-21. External open-source rules/UI prior art only; its ascending-first schedule, special one-card visibility, negative miss penalty, networked server, and Electron/Scala.js design are not adopted.
[^E5]: W3C WAI, “ARIA Authoring Practices Guide,” <https://www.w3.org/WAI/ARIA/apg/>, consulted 2026-06-21. Used for native/ARIA widget semantics, accessible names, keyboard interfaces, and focus conventions.
[^E6]: W3C WAI, “Understanding Success Criterion 1.1.1: Non-text Content,” <https://www.w3.org/WAI/WCAG22/Understanding/non-text-content>, consulted 2026-06-21. Used for text alternatives and decorative-image treatment.
[^E7]: W3C WAI, “How to Meet WCAG 2.2 (Quick Reference),” <https://www.w3.org/WAI/WCAG22/quickref/>, consulted 2026-06-21. Used for keyboard, focus, reflow, target-size, status-message, and reduced-motion acceptance planning.
[^E8]: Thijs Snelleman, “Strategic Gambling in DiminishingBridge,” Leiden Institute of Advanced Computer Science bachelor thesis, 2020, <https://theses.liacs.nl/pdf/2019-2020-SnellemanT.pdf>, consulted 2026-06-21. Used only for the separation of bidding/play decision engines, rule-based contract-relative play, and incomplete-information cautions. Its Monte Carlo and Monte Carlo tree-search methods are not adopted and remain forbidden for Rulepath public v1/v2.

## E.4 Repository reference paths

Each reference below names a repository-relative path. The spec was authored against repository state as of 2026-06-21; resolve every path against the current tree.

[^R1]: `docs/ROADMAP.md`.
[^R2]: `specs/README.md`.
[^R3]: `docs/MECHANIC-ATLAS.md`.
[^R4]: `docs/FOUNDATIONS.md`.
[^R5]: `docs/ENGINE-GAME-DATA-BOUNDARY.md`.
[^R6]: `docs/MULTI-SEAT-AND-SURFACE-CONTRACT.md`.
[^R7]: `docs/TESTING-REPLAY-BENCHMARKING.md`.
[^R8]: `docs/AI-BOTS.md`.
[^R9]: `docs/UI-INTERACTION.md`.
[^R10]: `docs/IP-POLICY.md`.
[^R11]: `docs/AGENT-DISCIPLINE.md`.
[^R12]: `docs/adr/0004-hidden-info-replay-export-taxonomy.md`.
[^R13]: `archive/specs/gate-10-1-plain-tricks-trick-taking-proof.md` and `games/plain_tricks/docs/PRIMITIVE-PRESSURE-LEDGER.md`.
[^R14]: `archive/specs/gate-16-briar-circuit-trick-taking.md` and `games/briar_circuit/docs/PRIMITIVE-PRESSURE-LEDGER.md`.
[^R15]: Inspection of `games/secret_draft/src/{actions,rules,state,setup,visibility,bots}.rs`, `games/masked_claims/src/{actions,rules,state,setup,visibility,bots}.rs`, and `games/high_card_duel/src/{actions,rules,state,setup,visibility,bots}.rs`. These are commitment/claim/hidden-choice precedents, not numeric trick-contract implementations.
[^R16]: `docs/TRACE-SCHEMA-v1.md`.

# Appendix G — Author self-check

- [x] Gate 17 is confirmed from Gate 16 `Done`, empty §10A debt, lowest non-`Done` Order 8, and ROADMAP §15; no alternate next gate is proposed.
- [x] The classic Oh Hell family is locked and exact parameters are research-pinned: 3–7 seats, deck-derived 10/8/7 maximum, down-and-up schedule, turn-up trump, dealer-last hook, first leader, play, exact-or-zero scoring, fixed terminal, and co-winner tie handling.
- [x] An original neutral identity, **Vow Tide** / `vow_tide`, determines the crate and spec filename; human IP review remains mandatory.
- [x] The third-use hard gate is fired and resolved with exactly one atlas option: promote a narrow pure follow-suit/winner-index helper, back-port both matching prior games in-gate, and create no §10A debt.
- [x] Winner-leads and deal/redeal are explicitly reviewed anti-examples/non-goals inside the selected promotion rather than silently ignored.
- [x] No card/suit/rank/hand/trick/trump/bid/contract noun or policy is added to `engine-core`.
- [x] Numeric bidding/contracts receives a new first-use `local-only` ledger/atlas entry; legality and scoring remain typed local Rust, never static-data behavior.
- [x] Variable 3–7-seat no-leak covers every ordered seat pair, source-to-observer edge, observer/every-seat replay export, Rust/WASM exhaustive viewers, and required browser samples across all named surfaces.
- [x] Seat declaration includes minimum, maximum, default, supported set, labels/order, and deterministic diagnostics; simulations/outcomes are seat-keyed and benchmarks cover every count.
- [x] L0 is required; bounded L1 is specified from authorized information; L2 remains unadmitted behind competent-player/evidence gates; all forbidden search/sampling/learning methods are excluded.
- [x] Deliverables map to the official-game templates, Rust/WASM/tools/web registrations, official contract, testing, multi-seat, UI, bot, replay, source/IP, and release obligations.
- [x] The 12 required sections are present; explicit `not applicable`/deferred requirements replace silent omission.
- [x] Documentation closeout names `specs/README.md`, `docs/SOURCES.md`, every relevant mechanic-atlas row and §10A, prior ledgers, game-local docs, and all three enforced `apps/web/README.md` surfaces.
- [x] The spec is authored only and creates no ticket files.

# Outcome

**Specification status:** `Done`.

Completed: 2026-06-21.

Gate 17 shipped **Vow Tide**, a variable 3-7-seat exact-bid trick-taking
official game. The implementation locks the `vow_tide_standard` variant,
registers the Rust game, tools, WASM bridge, public rules, web renderer, e2e
smoke, rule coverage, replay/fixture evidence, simulations, bot docs, benchmark
evidence, source/IP notes, and public-release checklist.

The third-use trick-taking hard gate closed through the narrow
`game-stdlib::trick_taking` promotion. Plain Tricks and Briar Circuit were
conformed in the same gate, Vow Tide uses the helper, and `docs/MECHANIC-ATLAS.md`
§10A records `Current debt: _None_`.

Verification recorded for closeout includes `cargo test --workspace`,
`fixture-check`, `replay-check`, `rule-coverage`, the 3-7-seat simulation matrix,
Plain Tricks and Briar Circuit replay checks, `cargo bench -p vow_tide`, web
build/e2e smoke, catalog/doc/outcome checkers, and the central spec/source/atlas
status reconciliation.

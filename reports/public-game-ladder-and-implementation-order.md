# Public Game Ladder and Implementation Order

**Advisory input only.** This document does not overwrite any repository file. Final roadmap, spec-index, and ticket wording remains the user's.

**Target repository:** `joeloverbeck/rulepath`  
**Target commit used:** `e3b1729cfeb38431835636f7df6d7e518420b2ee`  
**Freshness claim:** user-supplied target commit only; this document does **not** independently verify latest `main`.

> Commit-selection note: the uploaded research brief names commit `a97625c43028d425d400bc8a4a112b9b6ffba899`, but the explicit Exact-Commit Git Discipline Guardrail supplied with the task names `e3b1729cfeb38431835636f7df6d7e518420b2ee`. This document follows the guardrail and treats the uploaded manifest as a path inventory only.

## 1. Source discipline and evidence basis

Repository metadata used: **no**. Default-branch lookup used: **no**. Branch-name file fetch used: **no**. GitHub code search used: **no**. Clone used: **no**. Connector namespace trusted as evidence: **no**. Repository files were fetched only from exact `raw.githubusercontent.com/joeloverbeck/rulepath/e3b1729cfeb38431835636f7df6d7e518420b2ee/<path>` URLs whose paths appeared in the uploaded manifest.

The recommendations below rest on three evidence groups:

1. **Foundation and process law:** `docs/README.md`, `FOUNDATIONS.md`, `ARCHITECTURE.md`, `ENGINE-GAME-DATA-BOUNDARY.md`, `OFFICIAL-GAME-CONTRACT.md`, `MECHANIC-ATLAS.md`, `AI-BOTS.md`, `UI-INTERACTION.md`, `TESTING-REPLAY-BENCHMARKING.md`, `TRACE-SCHEMA-v1.md`, `ROADMAP.md`, `IP-POLICY.md`, `AGENT-DISCIPLINE.md`, `WASM-CLIENT-BOUNDARY.md`, `SOURCES.md`, `archival-workflow.md`, and ADRs `0001` through `0006` plus `ADR-TEMPLATE.md`.
2. **Code seams:** `engine-core`, `wasm-api`, representative two-seat game crates, `event_frontier`, `frontier_control`, `poker_lite`, `tools/simulate`, and web-shell seat/view/outcome components.
3. **External rule/prior-art research:** public-domain card/board-game references, Texas Hold'Em references, and game-engine/prior-art sources for N-player and imperfect-information representation.

## 2. Settled decisions confirmed

These are not reopened here.

1. **The public mechanic ladder should continue past Gate 14.** The current completed ladder proves many mechanics in small two-seat games, but it has not proven official 3+ seat play or substantially larger surfaces. The kernel is already generic over seats, so the next phase should press the game crates, tooling, app shell, templates, tests, and official-game contract rather than moving behavior into TypeScript or into the kernel.
2. **Texas Hold'Em proper is the first committed public game of the new phase.** It should be a new official game that coexists with `poker_lite` / Crest Ledger. It is not a rename and not a replacement.
3. **Gate P remains last.** It stays private, isolated, optional, and non-public. It must not drive public architecture before the public ladder has earned the needed capability.
4. **This plan is advisory input.** It is designed so the user can archive the current `specs/README.md` date-suffixed, then write a new implementation order using this document.

## 3. Repository evidence: what must be scaled

### 3.1 The kernel is already seat-generic

`engine-core` already exposes generic seat machinery: games receive `setup(seats: &[SeatId])`, actors and viewers are modeled separately, and visibility includes seat-scoped private data. The replay trace also stores a `seats` array rather than hardcoding two slots. That means the next phase should **not** add poker, map, deck, or N-seat nouns to `engine-core`. Keep the kernel noun-free and make the official games and tooling catch up.

**Implication:** do not design a generic `Player1/Player2` replacement in the kernel. Each game should own its seat/faction/domain model, and repeated support shapes should promote to `game-stdlib` only through `MECHANIC-ATLAS.md` discipline.

### 3.2 The two-seat ceiling is in games, tools, and presentation assumptions

Representative game crates define two concrete seat variants and reject any seat count other than two in setup. `event_frontier` and `frontier_control`, the current largest surfaces, also reject non-two-seat setups. The simulator aggregates `seat_0_wins` and `seat_1_wins`, which is a tooling assumption that will become false immediately at the Hold'Em rung.

The web shell has a mixed picture. Some surfaces already represent standings and breakdowns as arrays, which is good. Other components still assume a singular active seat, two local-control seats, or a first/second player mode model. That is presentation debt, not behavior authority.

**Implication:** the first implementation phase needs a narrow N-seat interlock: not “implement multiplayer,” but “teach official-game metadata, setup, simulator reporting, WASM view projection, and UI panels to accept game-declared seat ranges.”

### 3.3 The current largest surfaces are still tiny

`event_frontier` has a small graph and event deck; `frontier_control` has a small graph and two asymmetric factions. They are valuable proofs, but they are not enough evidence for COIN-like maps, Imperium-like card economies, Arkham-like scenario surfaces, or even multi-seat classic card games with large visibility matrices.

**Implication:** the ladder should not jump straight from Gate 14 to a monster game. It should add scale in deliberately separable dimensions: N seats, hidden information, partnerships, meld/tableau surfaces, 100+ board nodes, interrupt/reaction priority, then medium-large asymmetric event maps.

### 3.4 `poker_lite` is a useful delta source, not Texas Hold'Em

`poker_lite` / Crest Ledger is a two-seat abstract betting/showdown proof with a tiny custom card set and intentionally excludes casino/product presentation. It does not prove 52-card dealing, 3+ player betting order, blinds, community cards, five-card-from-seven evaluation, split pots among multiple winners, or no-leak behavior across multiple private hands.

**Implication:** Texas Hold'Em should reuse the lessons of `poker_lite`—betting-state clarity, hidden-info visibility, showdown explanation patterns—but should be its own crate and spec.

## 4. Recommended ambition ceiling

### Default recommendation

The public ladder should climb to a **medium-heavy original public asymmetric event-map game** before Gate P:

- **Players:** 3–4 official seats.
- **Map/surface:** about 24–36 named sites, with typed adjacency/regions/zones validated in Rust.
- **Cards/events:** about 60–100 public-domain/original event, role, or policy cards.
- **State scale:** several public resource/status tracks, site control/presence, per-faction capabilities, periodic scoring, and visible event queues.
- **Hidden information:** limited and purposeful; enough to prove N-player no-leak, but not a licensed-style scenario book or content ocean.
- **Architecture:** no DSL, no YAML, no TypeScript legality, no noun creep into `engine-core`, no MCTS/ISMCTS/Monte Carlo/ML/RL public bots.
- **Public release posture:** original Rulepath presentation, source-backed public-domain mechanics where inherited, and no private target IP in public assets.

Call this ceiling **“medium-heavy public capstone,” not “monster game.”** Gate P then remains the private red-team after the capstone.

### Why this ceiling is the right default

Going straight to a COIN/Imperium/Arkham-sized private target would couple public architecture to private aspirations too early. The current repo has not yet proven 3+ seats, dynamic player counts, large view payloads, or N-player no-leak. A medium-heavy capstone is high enough to force the architecture to handle the real pressure points—large topology, repeated control/resource shapes, multiple factions, event timing, outcome explanation, bigger benchmarks—without using licensed presentation or hiding hard problems inside a private throwaway.

The capstone should be **original** rather than a public-domain clone because, by that point, the platform’s remaining question is not “can it encode a known old game?” but “can it carry a large, explainable, replayable rules system while preserving Rulepath law?”

## 5. Texas Hold'Em proper: first committed public rung

### 5.1 Name and IP posture

Recommended public/product name: **River Ledger**.  
Recommended rules-family label inside docs: **Texas Hold'Em rules family**.

Use neutral table/card language. Avoid casino trade dress, tournament branding, chip-culture copy, and borrowed product prose. The game system—standard 52-card poker with Texas Hold'Em dealing and betting structure—is safe as a public-domain rules family, but the presentation must remain original and sober.

### 5.2 Scope: recommended v1

**Official player range:** **3–6 seats**.

- Three seats proves the feature that the phase exists to prove: more than two players.
- Six seats is enough to exercise multi-opponent betting order and showdown ties without immediately taking on 9–10 seat payload/UI clutter.
- Do not make heads-up the primary official path. A two-seat variant may exist later for completeness, but the first official proof should not let the project pass while still mostly operating in a two-seat mental model.

**Deck and deal:** standard 52-card deck, deterministic shuffle from engine RNG, two private hole cards per seat, five community cards, and Hold'Em street structure. Include burn cards as internal deterministic deck advancement if the rules spec wants canonical casino dealing semantics, but never leak burn-card identities to public views, DOM, logs, or public replay exports.

**Betting model:** **fixed-limit, capped-raise Hold'Em** for v1.

- Button, small blind, and big blind rotate in seat order.
- Streets: preflop, flop, turn, river, showdown.
- Legal actions: fold, check, call, bet, raise, and street advancement where appropriate.
- Use abstract contribution units, not casino-branded chip language.
- Fixed limit: small bet size on preflop/flop, big bet size on turn/river.
- Cap raises per street to keep action trees bounded and tests comprehensible.

**All-in / side pots:** **defer from v1**.

This is the strongest default. Side pots are important eventually, but they combine contribution caps, eligibility, partial pots, fold/no-showdown visibility, odd remainders, and multi-way explanation into one knot. The first public rung already proves N seats, hidden private hands, community cards, multi-street betting, showdown evaluation, and split winners. V1 should use fixed contribution capacity high enough that legal play cannot require all-in handling. All-in/side pots should become a follow-up rung after the base no-leak and showdown surfaces are green.

**Split pots:** **in scope**.

Tied best hands among showdown-eligible players must split the pot. If integer contribution units create an odd remainder, allocate the remainder deterministically by stable button-order among tied winners and explain it in the outcome rationale. This is not optional; without it, the engine has not really proven multi-winner N-player outcome accounting.

### 5.3 Required showdown explanation

Showdown explanation is non-negotiable and must be Rust-authored.

For each seat, the final outcome explanation should include:

- Whether the seat folded before showdown or reached showdown.
- The seat’s private cards only if the final view is authorized to reveal them under the game’s visibility rules.
- The best five-card hand selected from the seven available cards.
- The hand category: high card, pair, two pair, trips, straight, flush, full house, quads, straight flush.
- The ordered tie-break vector used by the evaluator.
- The decisive comparison, for example “pair rank lost to higher pair,” “same two pair; kicker lost,” or “identical best five; pot split.”
- Pot allocation including ties and any deterministic remainder rule.

A player who wins because everyone else folded should receive a different rationale: “last remaining live hand after folds,” with no unnecessary reveal of folded players’ private cards.

### 5.4 Hand evaluator recommendation

Implement a straightforward, deterministic five-card evaluator plus seven-card best-hand search:

1. Enumerate the 21 possible five-card subsets from seven cards.
2. Evaluate each five-card subset into a comparable tuple: category + category-specific rank vector.
3. Select the maximum tuple by deterministic ordering.
4. Return both the tuple and the exact cards used for explanation.

This is plenty fast for 3–6 players and vastly easier to audit than a lookup-table evaluator. Avoid optimizing prematurely. The official admission bar is correctness, explainability, replayability, and no-leak safety, not casino-scale throughput.

### 5.5 Bot scope

Use only policy/belief heuristics permitted by `AI-BOTS.md`.

V1 bots:

- Level 0: legal random bot using only its legal view.
- Level 1: conservative policy bot using private hole strength, board texture, current call price, number of live opponents, and street.
- Level 2: limited opponent-count-aware heuristic; still no MCTS, ISMCTS, Monte Carlo rollouts, ML, RL, or hidden-card peeking.

The bot evidence pack must explicitly prove that each bot decision is derived only from the bot’s authorized seat view. Multi-opponent “belief” should be rule-of-thumb categories and public pot odds, not sampled hidden worlds.

## 6. Capability gaps the next phase must close

### 6.1 Game-local N-seat modeling

Current games can keep their two-seat enums; do not churn them unless needed. New games should establish the pattern for dynamic seat ranges:

- A game-local `SeatIndex` / `SeatRole` representation derived from the `seats: &[SeatId]` supplied by `engine-core`.
- Setup validation against `min_seats` and `max_seats` metadata.
- Seat order, dealer/button order, active/pending seat sets, and eliminated/folded statuses stored deterministically.
- No use of `Player1`/`Player2` semantic names in generic code or tools.

### 6.2 Official-game metadata and setup UX

Add or standardize game metadata that the WASM layer and React shell can present without deciding legality:

- `min_seats`, `max_seats`, supported seat counts, default seat count.
- Seat labels supplied by Rust/game data.
- Whether the game has teams/partnerships, hidden hands, simultaneous/pending decisions, or showdown reveal.
- Whether a public observer view is supported.

The app shell should render this metadata but never infer legal setup beyond selecting a supported count and passing it to Rust/WASM.

### 6.3 Simulator and benchmark summaries

Generalize `tools/simulate` from `seat_0_wins` / `seat_1_wins` to indexed maps:

- `wins_by_seat: BTreeMap<SeatId, u64>`
- `draws`, `ties_by_winner_set`, or equivalent multi-winner accounting
- per-seat average score/contribution where applicable
- stable serialized output order

Benchmarks must record seat count and surface scale. A 3-seat Hold'Em benchmark should not be compared blindly to a 2-seat tiny board benchmark.

### 6.4 Trace schema and replay

The v1 trace schema already has a `seats` array, so do not migrate the schema solely to “support N seats.” Instead, document stricter semantics:

- The trace’s `seats` array is ordered and can contain any supported count.
- View hashes must be recorded for every participating seat plus public observer where relevant.
- Hidden-info replay exports must be tested for each viewer class.
- Golden traces for N-player games must include wrong-seat diagnostics involving at least three seats, not just “other player.”

A trace-schema migration is needed only if the replay payload cannot express simultaneous/pending multiple actors cleanly. If that happens, outline an ADR before changing schema semantics.

### 6.5 N-player hidden-information no-leak

N-player no-leak is harder than two-player no-leak because seat A must not see seat B’s private cards, seat C’s private cards, burn cards, deck order, private bot rationale, or unrevealed committed choices. The test matrix must include:

- Public observer view.
- Each seat’s private view.
- Pairwise negative assertions: seat A cannot read seat B’s private zones for all A ≠ B.
- DOM/storage/log/effect-log assertions.
- Public replay export/import no-leak.
- Bot explanation no-leak.
- Final reveal timing: cards revealed only when rules say they are revealed.

For Hold'Em specifically, public betting patterns are allowed to reveal strategic information, but payloads must not reveal hidden cards or deck state. Tests should distinguish legitimate inference from payload leakage.

### 6.6 WASM and active/pending seat projection

The current shell can survive a singular `active_seat` for many sequential games, but the next ladder will need the docs and types to anticipate:

- one active seat;
- multiple pending seats for simultaneous choices or response windows;
- no active seat during deterministic resolution;
- observer views independent from player views;
- per-seat legal action lists generated by Rust.

Do not let React infer whose turn it is. Rust/WASM should expose enough view metadata for rendering turn order, pending seats, and action availability.

### 6.7 Multi-seat UI

The UI needs a public multi-seat frame before Hold'Em becomes a game polish problem:

- Seat rail with 3–6+ seats.
- Active/pending/folded/eliminated markers.
- Turn-order display.
- Viewer selector in dev/local mode that scales beyond two seats.
- Showdown table with one row per seat, hand class, chosen cards, comparison reason, and allocation.
- Accessibility labels that do not depend on color or left/right player positions.

The UI remains presentation-only. It can display legal actions and rationale; it cannot compute them.

### 6.8 Mechanic-atlas pressure

Scaling will trip the third-use hard gate. Plan for it instead of pretending every game is isolated.

Likely promotion-pressure points:

- deterministic shuffle / hidden hands / private zones;
- multi-seat turn order and pending-seat windows;
- fixed graph/topology validation;
- site control and presence accounting;
- public resource/status tracks;
- per-seat and per-team outcome explanation;
- deterministic card-tableau/meld surfaces;
- interrupt/reaction priority.

The ladder below intentionally spaces these pressures so each can be resolved before the next rung depends on it.

## 7. Researched public game ladder

The ladder is designed as a bridge, not a nostalgia list. Each rung earns a new capability while staying public-domain or original and IP-safe.

### Gate 15 — River Ledger / Texas Hold'Em rules family

**Seats:** 3–6.  
**Proves:** first official N-seat hidden-information betting game; 52-card deck; multi-street betting; showdown explanation; split winners.  
**Atlas pressure:** deterministic shuffle/private hand, N-player hidden-info views, outcome explanation, contribution accounting.  
**IP posture:** public-domain card-game rules family with original Rulepath presentation.

### Gate 15.1 — River Ledger all-in and side-pot extension

**Seats:** 3–6.  
**Proves:** partial eligibility, nested pots, all-in contribution caps, multi-way allocation explanation.  
**Atlas pressure:** public resource accounting, per-seat allocation rationale, edge-case golden traces.  
**Why separate:** side pots are too easy to get subtly wrong and too broad to bundle into first Hold'Em admission.

### Gate 16 — Hearts

**Seats:** 4.  
**Proves:** fixed four-seat trick-taking with pass direction, follow-suit obligation, negative scoring, round-to-match accumulation, and large hidden-hand no-leak after a full deal.  
**Atlas pressure:** trick-taking shape second/third use, N-seat private hands, per-round score explanation.  
**IP posture:** traditional public-domain card game; use original prose and neutral presentation.

### Gate 17 — Oh Hell

**Seats:** 3–7.  
**Proves:** variable seat count, dealer rotation, changing hand size, bidding/contract surface, and “last bidder constrained” validation.  
**Atlas pressure:** seat-count metadata, N-seat bidding order, trick-taking promotion decision.  
**Why after Hearts:** it generalizes trick-taking from fixed four seats to variable N.

### Gate 18 — Spades

**Seats:** 4, partnership pairs.  
**Proves:** teams/partnerships, team scoring, contract evaluation, nil-bid style risk if included, and UI grouping that is neither per-seat-only nor two-player.  
**Atlas pressure:** team outcome explanation, partnership visibility and scoring.  
**Recommended v1 scope:** standard partnership Spades; defer Blind Nil or exotic variants.

### Gate 19 — Five Hundred Rummy / Rummy 500 family

**Seats:** 3–5 recommended official range.  
**Proves:** draw/discard piles, public meld tableau, private hands, multi-round score target, and larger action affordance sets without map topology.  
**Atlas pressure:** public/private card zones, meld validation, open-table presentation, N-player no-leak beyond trick-taking.

### Gate 20 — Star Halma / Chinese Checkers family

**Seats:** 3, 4, or 6 official variants; optionally 2 as non-primary.  
**Surface:** 121-hole star board.  
**Proves:** first large board surface, long jump chains, multi-seat spatial race, blocked-path tactics, and scalable board rendering.  
**Atlas pressure:** topology helper, board-space presentation, path/jump validation, larger-surface benchmarks.  
**Why here:** it scales surface size without hidden cards or event timing, isolating board/UI/benchmark pressure.

### Gate 21 — Pachisi-family race game

**Seats:** 4 recommended.  
**Proves:** track topology, dice/chance, safe/capture spaces, multiple pawns per seat, home-entry rules, and possible partnerships depending on variant.  
**Atlas pressure:** deterministic chance events, track/path topology, capture/safety semantics.  
**IP posture:** historic public-domain family; final spec should cite a stable public-domain rules source and use original presentation.

### Gate 22 — Four Winds Melds / Mahjong-family scoped original

**Seats:** 4.  
**Proves:** wall draw/discard rhythm, exposed vs concealed sets, discard-claim priority, reaction windows among multiple opponents, and larger hidden-information state.  
**Atlas pressure:** reaction-window hard gate, private/public meld zones, draw-wall no-leak, scoring explanation.  
**Recommended posture:** not a full regional Mahjong clone. Build a small, explicitly scoped public-domain-inspired original that proves the priority/reaction machinery without importing a sprawling scoring tradition.

### Gate 23 — Commonwealth Frontier / original medium-heavy public capstone

**Seats:** 3–4 asymmetric factions.  
**Surface:** 24–36 sites, regional adjacency, several public tracks, 60–100 original event cards.  
**Proves:** the recommended ambition ceiling: large topology, asymmetry, public resource accounting, event queues, periodic scoring, faction-specific actions, large view payloads, and explainable outcomes.  
**Atlas pressure:** graph-map topology, site control, faction asymmetry, resource accounting, reaction/timing windows, event-card presentation.  
**IP posture:** fully original Rulepath setting/content; mechanics documented by public-domain/general board-game references where relevant, but no private target expression.

### Gate P — private monster-game red-team

**Seats/surface:** private by design.  
**Placement:** after the whole public ladder.  
**Purpose:** optional isolated red-team against private aspirations once public architecture has earned the right to be stressed.  
**Constraint:** cannot drive public architecture retroactively unless a public, foundation-consistent ADR and follow-up ladder task justify the change.

## 8. Recommended phased implementation order

This table is intentionally shaped so it can be adapted into a replacement `specs/README.md`. It is **not** the finished index.

| Order | Proposed gate/spec seed | Initial status | Purpose | Primary capability gap closed | Atlas / ADR interlock | Exit evidence |
|---:|---|---|---|---|---|---|
| 0 | Foundation realignment package | Planned | Update docs/templates before new execution | Admits 3+ seats and larger surfaces into authority docs | Proposed no ADR by default; add ADR only for DSL/schema/bot-law changes | Docs updated; templates have seat-range/no-leak/surface fields |
| 1 | N-seat setup/catalog metadata | Not started | Let official games declare min/max/default seats | App and WASM setup stop assuming two seats | No kernel change; no TS legality | Metadata tests; setup UI smoke with 3 seats |
| 2 | N-seat simulator summaries | Not started | Replace `seat_0_wins` / `seat_1_wins` summaries | Tooling can report arbitrary seats and tied winner sets | Deterministic serialized order | Simulate smoke for synthetic 3-seat game or Hold'Em harness |
| 3 | Multi-seat shell frame | Not started | Render 3+ seat rails, turn order, viewer selector | Presentation shell no longer left/right player only | TypeScript presentation only | A11y smoke with 3/4/6 seats and no legal inference |
| 4 | N-player no-leak test harness | Not started | Add reusable test expectations for public observer and pairwise private views | Hidden-info proof obligation scales past two seats | Must conform to ADR 0004 | Pairwise no-leak golden trace fixture |
| 5 | Gate 15: River Ledger base Hold'Em | Not started | First official 3–6 player public game | N-seat hidden-info betting + showdown explanation | Private-hand/shuffle and outcome-rationale pressure | Rules, coverage, bots, UI, traces, no-leak, benchmarks green |
| 6 | Gate 15.1: side pots/all-in | Not started | Extend Hold'Em after base proof | Nested pot eligibility and allocation rationale | Public resource/accounting pressure | Golden traces for all-in fold, multi-way side pot, split side pot |
| 7 | Gate 16: Hearts | Not started | Fixed four-seat trick-taking | Full-deal N-seat private hands, pass direction, scoring | Trick-taking promotion evaluation | Four-seat traces, no-leak, follow-suit diagnostics |
| 8 | Gate 17: Oh Hell | Not started | Variable-N trick-taking and bidding | Seat-count range 3–7, bidding constraints | Trick-taking helper promotion likely | 3-, 5-, and 7-seat fixture traces |
| 9 | Gate 18: Spades | Not started | Partnerships and team scoring | Team outcome model and UI grouping | Team/partnership contract addition | Partnership trace and team score explanation |
| 10 | Gate 19: Five Hundred Rummy | Not started | Meld/tableau card surface | Public melds + private hands + discard interaction | Meld/tableau primitive pressure | Multi-round traces; no-leak across open/closed zones |
| 11 | Gate 20: Star Halma | Not started | Large board surface without hidden cards | 121-space topology and jump-chain UI/perf | Topology helper hard gate likely | Large-board smoke, replay, benchmark floor |
| 12 | Gate 21: Pachisi-family race | Not started | Track topology and deterministic chance | Multiple pawns/seat, safe/capture spaces | Chance + track topology pressure | Dice trace determinism and capture/safety coverage |
| 13 | Gate 22: Four Winds Melds | Not started | Scoped Mahjong-family priority/reaction proof | Multi-opponent discard claims and concealed/exposed sets | Reaction-window hard gate likely | Priority traces; no-leak for wall/hand/concealed sets |
| 14 | Gate 23: Commonwealth Frontier capstone | Not started | Medium-heavy original public asymmetric map game | 3–4 factions, 24–36 sites, 60–100 events | Graph/site/faction/resource/event promotions resolved first | Full official-game contract plus benchmark gates |
| 15 | Gate P: private monster-game red-team | Deferred / last | Isolated non-public stress test | Private aspiration validation only | Must not drive public architecture silently | Private red-team report, not public game admission |

## 9. Sequencing rules for the user’s new `specs/README.md`

When turning this into a real index, keep these rules:

1. Put foundation/template realignment before any implementation gate.
2. Keep each implementation task bounded. Do not create a spec called “implement multiplayer.”
3. Put Texas Hold'Em before Hearts/Oh Hell/etc. The first public game rung is locked.
4. Keep side pots/all-in separate from base Hold'Em unless the user explicitly decides to accept higher risk.
5. Resolve mechanic-atlas third-use gates before later games depend on the repeated shape.
6. Gate P is last and remains private/deferred.
7. Every gate must end with official-game evidence: rules, coverage, mechanic inventory, bots, UI, outcome explanation, traces, no-leak tests where relevant, and benchmarks.

## 10. Recommended additions to `docs/SOURCES.md`

Add a new section such as **“Next-phase scaling sources”** with entries for:

- Pagat, Texas Hold'em rules: `https://www.pagat.com/poker/variants/texas_holdem.html` — player range, blinds, betting streets, flop/turn/river/showdown rule reference.
- Pagat, poker hand ranking: `https://www.pagat.com/poker/rules/ranking.html` — hand categories and comparison baseline.
- Pagat, Hearts: `https://www.pagat.com/reverse/hearts.html` — four-seat trick-taking, passing, scoring reference.
- Pagat, Oh Hell: `https://www.pagat.com/exact/ohhell.html` — variable player count and bidding/trick-taking reference.
- Pagat, Spades: `https://www.pagat.com/boston/spades.html` — partnership trick-taking and bidding reference.
- Pagat, Chinese Checkers / Bead Game: `https://www.pagat.com/race/bead_game.html` — 121-hole board and 2–6 player race reference.
- Boardgame.io: `https://boardgame.io/` — prior-art reference for turn-based multiplayer engine concepts, phases, events, logs, and multiplayer support.
- OpenSpiel paper: `https://arxiv.org/abs/1908.09453` — prior-art reference for a framework that explicitly spans N-player, sequential/simultaneous, perfect/imperfect-information games.
- A stable Rummy 500 rules source selected by the user during final doc writing.
- A stable Pachisi rules source selected by the user during final doc writing.
- A stable Mahjong-family rules source selected by the user during final doc writing, if Gate 22 remains in scope.

## 11. Risk register

| Risk | Why it matters | Mitigation |
|---|---|---|
| Hold'Em becomes too large if side pots ship in base v1 | Side pots multiply edge cases before the N-seat/no-leak surface is proven | Defer all-in/side pots to Gate 15.1 |
| UI accidentally decides legality for turn order or betting | Violates Rust authority | Rust/WASM provides legal actions, active/pending seats, and rationale; React renders only |
| Third-use promotion debt blocks later games | Scaling repeats card zones, topology, resource, reaction, and outcome shapes | Add primitive-pressure ledger checks at each rung |
| Hidden-info leak through logs or replay exports | N-player poker and meld games make leaks harder to notice | Pairwise no-leak matrix and public export/import tests |
| Ambition ceiling drifts into licensed/private target clone | IP and architecture contamination | Capstone is original public Rulepath content; Gate P remains private last |
| Bot quality pressure tempts Monte Carlo/ISMCTS | Public bot law forbids it | Use policy/belief heuristics and flag any future relaxation as proposed ADR only |
| Trace schema evolves informally | Replay determinism is foundation law | Use existing `seats` array unless an explicit ADR/migration is necessary |

## 12. Bottom-line recommendation

Proceed with the next public phase, but do it with discipline: update the docs/templates, add N-seat shell/tooling/no-leak foundations, ship **River Ledger / Texas Hold'Em** as the first real public proof, then climb through traditional public-domain multiplayer card and board games before attempting an original medium-heavy asymmetric capstone. Only after that should Gate P run as the private monster-game red-team.

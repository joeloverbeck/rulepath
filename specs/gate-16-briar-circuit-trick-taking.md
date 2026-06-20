# Gate 16 implementation spec — **Briar Circuit** / classic Hearts trick-taking

## 1. Header

| Field | Value |
| --- | --- |
| Spec ID | `GAT16-BRICIR-TRICK-001` |
| File | `specs/gate-16-briar-circuit-trick-taking.md` |
| Roadmap stage | Stage 16 / Public scaling phase |
| Roadmap build gate | Gate 16 |
| Status | `Planned` |
| Date | 2026-06-20 |
| Owner | Rulepath maintainers / implementation agents |
| Primary crate | `games/briar_circuit` |
| Internal game id | `briar_circuit` |
| Public display name | **Briar Circuit** |
| Rules-family label | Classic four-player Hearts |
| Standard variant id | `briar_circuit_standard` |
| Trace rules version | `briar-circuit-rules-v1` |
| Data/manifest version | `briar-circuit-data-v1` |
| Browser implementation required | Yes — Rust/WASM-backed public renderer, observer and seat-private replay, and e2e smoke are gate requirements |
| Official seat declaration | Minimum `4`; maximum `4`; default `4`; supported set `{4}`; stable labels `seat_0` through `seat_3` |
| Public observer | Required |
| Bot floor | L0 random-legal required; a bounded L1 rule-informed policy is in scope; L2 is not admitted by this spec |
| Trace schema | Existing Trace Schema v1; no schema migration authorized |
| Authority order | `docs/README.md` → `docs/FOUNDATIONS.md` → `docs/ARCHITECTURE.md` → `docs/ENGINE-GAME-DATA-BOUNDARY.md` → area contracts → accepted ADRs only where they explicitly supersede named sections → `docs/ROADMAP.md` → this spec → later candidate tasks/tickets |
| Kernel stance | No new kernel concept. Card, deck, hand, suit, rank, pass, trick, led-suit comparison, hearts-broken state, hand scoring, moon resolution, and match accumulation remain typed game-local Rust |
| Primitive stance | Gate 16 is the second close trick-taking use. Compare `plain_tricks` and Briar Circuit, then keep local/defer. No trick-taking promotion to `game-stdlib`; no card/suit/rank/hand/trick noun in `engine-core` |
| Delivery posture | This is one authored implementation spec. It names bounded candidate `AGENT-TASK` packets but creates no ticket files |

This specification is subordinate to the foundation set. It does not redefine an upstream contract. A conflict is resolved in favor of the earlier authority, and an architecture-changing exception requires an accepted ADR before implementation.

Although `docs/IP-POLICY.md` permits common public-domain game names such as Hearts, the established Rulepath catalog convention favors an original neutral identity. **Briar Circuit** is the chosen coinage: *briar* evokes the penalty-bearing suit without copying the source game's name or imagery, while *circuit* evokes the rotating pass and repeated cycle of tricks and hands. “Hearts” remains only the rules-family label in source notes and explanatory copy. The naming decision does not replace the human IP review required by `docs/IP-POLICY.md`.[^R17]

## 2. Objective

Gate 16 turns the roadmap's fixed-four-seat Hearts entry into a concrete official-game plan. The game must prove a complete four-seat, full-hand, multi-hand trick-taking match with private hands, private pass commitments, deterministic deal/replay, legal-only interaction, viewer-safe explanations, and a polished browser surface. The implementation must remain recognizably classic Hearts while using original Rulepath prose, naming, visual language, and assets.[^R1][^R5]

### 2.1 Gate determination — confirmed, not reopened

The next unit is settled by the current repository evidence:

1. `specs/README.md` records Gate 15.1 — River Ledger all-in / side pots — as `Done`, completed 2026-06-20.[^R2]
2. `docs/MECHANIC-ATLAS.md` §10A records `Current debt: _None_` at Gate 15.1 closeout; no debt-closure unit blocks the mechanic ladder.[^R3]
3. Gate 16 — Hearts is the lowest non-`Done` active-epoch row, Order 7, and its predecessor is complete.[^R2]
4. `docs/ROADMAP.md` §15 admits Gate 16 after Gate 15.1 and defines the purpose and three exit obligations this spec maps in §6.[^R1]

This spec therefore does not reconsider a different game, a River Ledger continuation, a maintenance detour, or a helper-promotion gate.

### 2.2 Product and architecture objective

Briar Circuit must demonstrate that Rulepath can support:

- exactly four official seats with stable clockwise order;
- a deterministic 52-card deal of 13 private cards per seat;
- a private three-card pass phase rotating left → right → across → hold;
- the complete lead/follow/trick-resolution loop for 13 tricks per hand;
- negative hand scoring, fixed shoot-the-moon adjustment, cumulative match scoring, threshold detection, and an unambiguous winner;
- pairwise no-leak proof for every ordered seat pair and the public observer;
- Rust-authored legal action trees, previews, diagnostics, semantic effects, views, outcomes, replays, and bot decisions;
- a pleasant four-seat React/SVG table that never reconstructs legality or hidden facts in TypeScript.

### 2.3 Second-use trick-taking posture

`plain_tricks` is the first official use of follow-suit legality, led-suit trick comparison, trick-winner-leads sequencing, and deterministic trick-round redeal, and the atlas records those rows as `local-only` first use.[^R3][^R13][^R14] Briar Circuit is the second close use. Under the first/second/third-use law, Gate 16 must implement honestly in its own crate, compare both implementations, update both game inventories and the repository atlas, and record a keep-local/defer decision.[^R3][^R4]

The differences are not decorative: Plain Tricks is a two-seat, reduced-deck, positive trick-count microgame, while Briar Circuit is a fixed-four-seat 52-card penalty game with simultaneous-hidden passing, lead restrictions, point-card exceptions, moon transformation, and cumulative match scoring. The expected decision is therefore **keep local / defer extraction**, with Gate 17 — Oh Hell — designated as the third-use hard gate. Gate 16 does not create a trick helper and does not pre-design Gate 17's answer.

## 3. Scope

### 3.1 In scope — locked standard variant

The official v1 variant is `briar_circuit_standard`. These parameters are normative and must appear consistently in `RULES.md`, `HOW-TO-PLAY.md`, `SOURCES.md`, typed Rust constants/types, rule coverage, traces, fixtures, bot docs, UI copy, and outcome explanations.

| Rule area | Locked Briar Circuit rule | Research/source decision |
| --- | --- | --- |
| Seats | Exactly four independent seats; no teams or partnerships | Gate 16 roadmap contract fixes four seats.[^R1] |
| Deck | Standard 52-card deck, four suits, ranks 2 through ace; ace high; no trump; every card dealt | Core four-player Hearts sources agree.[^E1][^E2] |
| Dealer and deal | Initial dealer is `seat_0` for the standard deterministic setup. Deal clockwise one card at a time beginning left of dealer until each seat has 13. Dealer rotates one seat clockwise after each hand | Physical rules commonly rotate the deal; fixing the initial dealer makes the digital fixture reproducible. The holder of 2♣, not the dealer, starts play.[^E1] |
| Pass cycle | Hand index modulo four: left, right, across, hold/no-pass; repeat. On pass hands each seat selects exactly three distinct cards before receiving any incoming cards | The left/right/across/hold cycle is the common four-player cycle.[^E1][^E2] |
| Pass privacy and resolution | Selections and committed card identities remain private. The engine may serialize commitments in deterministic seat order, but the exchange is atomic only after all four commitments. Public viewers see direction and completion/pending counts, never identities | Rulepath's commitment/no-leak law controls the digital form.[^R6][^R12] |
| Opening lead | The seat holding 2♣ must lead 2♣ to the first trick | Standard rule.[^E1][^E2] |
| Following suit | A player holding any card of the led suit must play a card of that suit. Only a void player may discard another suit | Standard rule and the direct repeated shape from Plain Tricks.[^E1][^R3] |
| Trick winner | Highest rank of the led suit wins. Off-suit cards never win. The winner captures all four cards and leads the next trick | Standard no-trump trick resolution.[^E1] |
| Hearts broken | Hearts become broken when a heart is played to any trick. Q♠ does **not** break hearts. Hearts may not be led while unbroken unless the leader's hand contains only hearts; a heart legally led under that exception breaks hearts immediately | Sources differ on whether Q♠ may break hearts. Briar Circuit adopts the hearts-only definition described by Pagat and treats the broader Bicycle phrasing as a house-rule divergence.[^E1][^E2] |
| First trick point restriction | On the first trick, a void-in-clubs player may not discard a heart or Q♠ while any non-point card is available. If every card otherwise available to that player is a point card, all held cards are legal. Follow-suit always has priority | “No points on the first trick” is common but not universal; the no-alternative exception avoids an impossible action set and is explicitly pinned here.[^E1][^E2] |
| Point values | Each heart is 1 point; Q♠ is 13 points; all other cards are 0 | Standard rule.[^E1][^E2] |
| Hand score | After 13 tricks, sum captured point cards by seat before moon transformation. A normal hand contributes 26 total raw points | Standard rule.[^E1] |
| Shoot the moon | A seat capturing all 13 hearts and Q♠ — all 26 raw points — adds 0 for the hand; each of the other three seats adds 26. The shooter does not choose an alternative subtraction | Sources describe multiple moon conventions. Fixed add-26-to-opponents is deterministic, widely used, and selected for Rulepath v1.[^E1][^E2] |
| Match threshold | After each completed hand, if any cumulative score is at least 100, evaluate match end. The lowest cumulative score wins | 100 is the usual target in the consulted classic rules.[^E1][^E2] |
| Low-score tie | A tied lowest cumulative score never resolves by seat order. Continue dealing complete hands, preserving dealer and pass-cycle rotation, until there is a unique lowest score; then terminate | Pagat documents playing additional hands when the low score is tied. This is the explicit Rulepath tie rule.[^E1] |
| Public information | Played cards, current trick, completed trick winners, captured point totals, pass direction/status, dealer, active/pending seats, hand number, cumulative scores, moon event, and terminal breakdown are public | Rust projection decides exact public shape.[^R6] |
| Private information | Unplayed cards in each hand; the association between a seat and its staged/committed pass selection; incoming cards before exchange; pass provenance after exchange; unrevealed shuffle/deck order; private legal tree and preview; private bot candidates | A selected card's identity becomes public when it is later played, but who passed it is not thereby revealed. All still-private facts must be absent from every unauthorized surface.[^R6][^R12] |

#### Exact legality order

Rust legality generation and validation must use this order; UI code must not restate it:

1. If the action is the first play of the hand, only 2♣ is legal for its holder.
2. Otherwise, if the acting seat holds the led suit, only cards of that suit are legal.
3. Otherwise, on the first trick, exclude hearts and Q♠ if at least one non-point card remains; if no non-point card remains, allow every held card.
4. Otherwise, when leading a later trick and hearts are unbroken, exclude hearts if at least one non-heart remains; if only hearts remain, allow every held card.
5. In every other play state, every held card is legal.

The apply path must revalidate the submitted card against the same Rust rule and emit stable diagnostics for wrong seat, stale token, unowned card, opening-lead violation, must-follow-suit violation, first-trick-point violation, and hearts-not-broken lead violation.

### 3.2 In-scope game modes and product surface

- Human plus one to three Rust bots in the local shell.
- Four-seat hotseat with an explicit private handoff/cover step between seat viewers.
- Bot-versus-bot seeded simulation and replay.
- Public-observer live/replay view.
- Explicit seat-private viewer selection where the shell already supports it.
- Viewer-scoped browser replay export/import; public observer is the default export class.
- Internal full traces for native replay/hash authority.
- Original card, table, score, pass-direction, broken-heart, trick, and outcome presentation.

### 3.3 Out of scope

| Area | Gate 16 stance |
| --- | --- |
| Variable seats | Out of scope. Gate 17 owns variable-N trick-taking pressure. |
| Partnerships or team information | Out of scope. Gate 18 owns partnership proof. |
| Alternative deck sizes / three- or five-player dealing | Out of scope. |
| Omnibus/J♦ bonus, spot hearts, cancellation hearts, turbo thresholds, partnership Hearts | Out of scope. |
| Q♠-breaks-hearts house rule | Out of scope for `briar_circuit_standard`. |
| Moon choice / subtract-26 option | Out of scope. Fixed opponent addition is the only v1 resolution. |
| Shoot the sun / taking every trick | Out of scope; no separate bonus or transformation. |
| Mid-hand claims, undo, takeback, hints that expose hidden state | Out of scope. |
| Hosted multiplayer, accounts, persistence, matchmaking, chat, ranking | Out of scope under local-first v1/v2. |
| L2 authored policy | Not admitted by this spec. A later L2 requires a completed strategy-evidence pack and competent-player acceptance evidence first. |
| Search/research AI | Out of scope and forbidden for public v1/v2: no MCTS, ISMCTS, Monte Carlo, rollout sampling, ML, or RL. |
| Generic card/trick library | Out of scope. No Gate 16 `game-stdlib` promotion. |
| Trace Schema v2 or changed hash semantics | Out of scope absent an accepted ADR. |
| New DSL, YAML rules, behavior tables, selector expressions | Out of scope and forbidden. |
| Ticket files | Out of scope. Ticket decomposition follows `/reassess-spec` and `/spec-to-tickets`. |
| River Ledger work | Out of scope. Gate 15/15.1 are shipped baselines, not Gate 16 implementation targets. |

### 3.4 ROADMAP §15 public-scaling prohibition — carried verbatim

> Private licensed content, copied rules prose or trade dress, YAML/DSL behavior, TypeScript legality, public MCTS/ISMCTS/Monte Carlo/ML/RL bots, kernel noun growth, hidden-state leakage, or private work shaping public architecture.

## 4. Deliverables

### 4.1 New game crate and evidence tree

The gate creates a new coexisting official game. It does not rename or fork `plain_tricks`.

```text
games/briar_circuit/
├── Cargo.toml
├── benches/
│   ├── briar_circuit.rs
│   └── thresholds.json
├── data/
│   ├── fixtures/
│   │   ├── briar_circuit_standard.fixture.json
│   │   ├── briar_circuit_first_trick_exception.fixture.json
│   │   ├── briar_circuit_moon.fixture.json
│   │   └── briar_circuit_threshold_tie.fixture.json
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

The exact module split may be corrected during reassessment if the current crate convention makes one file unnecessary, but all behavior and evidence named here remain required. A file-layout correction is not permission to broaden scope.

### 4.2 Typed Rust behavior deliverables

| Deliverable | Required behavior |
| --- | --- |
| Setup | Reject every seat count except four with a stable Rust diagnostic; establish ordered seats, deterministic RNG, initial dealer, hand index, pass direction, scores, and first deal. |
| Card model | Game-local `Suit`, `Rank`, `Card`, and stable `CardId`; canonical deterministic ordering for serialization, action trees, tests, and explanations. |
| Phase/state model | Explicit `Passing`, `PlayingTrick`, `ScoringHand`, and `Terminal` states, plus deterministic substate for pending pass seats, current trick, captured cards/points, dealer, hand number, hearts-broken, cumulative scores, and active seat. |
| Pass actions | Rust-owned select, unselect, confirm, and hold-hand transition. Exactly three distinct owned cards are required before confirm. Public commitment status is identity-free. Exchange is atomic after all seats confirm. |
| Play actions | Rust-generated legal card actions and stable diagnostics under the exact legality order in §3.1. |
| Trick resolution | Led suit and highest led-suit rank decide winner; captured cards transfer to the winner's hand record; winner leads next. |
| Scoring | Per-card raw point ledger, moon detection/transformation, per-hand additions, cumulative totals, threshold/tie continuation, terminal winner. |
| Effects | Deterministically ordered public and private semantic effects; never animation instructions. |
| Views | Public observer and four seat-private projections derived in Rust. Internal state is never serialized as a browser view. |
| Outcome | Rust-authored per-seat final standings and decisive breakdown, including raw hand points, moon adjustment, cumulative before/after, threshold trigger, tie continuation or unique-low winner. |
| Replay | Internal full deterministic trace plus viewer-scoped observation exports under ADR 0004. |
| Bots | L0 random legal and bounded L1 rule-informed policies using only authorized seat view, public history, legal actions, and declared bot RNG. |

### 4.3 Official-game documentation deliverables

Every template-backed document is filled; explicit `not applicable` rows replace silent omissions.

| File | Gate 16 requirement |
| --- | --- |
| `docs/SOURCES.md` | Source identities, access dates, facts used, variant divergences, original-prose statement, naming rationale, asset/license review, external-prior-art boundary. |
| `docs/RULES.md` | Normative original-prose rules, terminology, setup, pass, legal play, trick resolution, scoring, moon, terminal/tie behavior, diagnostics, deterministic ordering, rule IDs. |
| `docs/HOW-TO-PLAY.md` | Player-facing original summary matching Rust behavior and public rules asset. |
| `docs/RULE-COVERAGE.md` | Every `BC-*` rule mapped to unit/rule/property/trace/simulation/replay/serialization/visibility/UI evidence. |
| `docs/MECHANICS.md` | Full mechanic inventory using atlas categories, including private commitment, trick flow, negative scoring, effect shape, surface budgets, and benchmark pressure. |
| `docs/PRIMITIVE-PRESSURE-LEDGER.md` | Plain Tricks ↔ Briar Circuit comparison and one decision: `defer-reject / keep local`; Gate 17 named as next review trigger. |
| `docs/GAME-IMPLEMENTATION-ADMISSION.md` | Requirements-first receipt completed before behavior implementation begins, then final implementation receipt at closeout. |
| `docs/AI.md` | L0/L1 policy contract, authorized fields, deterministic tie-breaks, explanation shape, hard exclusions, simulation evidence. |
| `docs/COMPETENT-PLAYER.md` | Strategy landscape and measurable future L2 competence criteria; it does not falsely claim L1 is a competent human proxy. |
| `docs/BOT-STRATEGY-EVIDENCE-PACK.md` | Explicit status `L2 not admitted`. Required fields are completed as not applicable/deferred with the evidence still needed before a later L2. |
| `docs/UI.md` | Rust/React boundary, viewer matrix, pass handoff, legal-only card controls, score/trick presentation, replay, accessibility, reduced motion, no-leak DOM/storage rules. |
| `docs/BENCHMARKS.md` | Operations, fixtures, environment, provisional floors, calibration, variance-aware threshold method, native/WASM distinction. |
| `docs/PUBLIC-RELEASE-CHECKLIST.md` | Complete official, IP, no-leak, catalog, rules-copy, renderer, replay, bot, benchmark, smoke, and closeout receipt. |
| `templates/AGENT-TASK.md` | `not applicable` to this authored-spec deliverable; it is consumed later by `/spec-to-tickets`, not instantiated here. |
| `templates/README.md` | Process/index guidance only; `not applicable` as a file copied into the game crate. Its template-use rules still govern every document above. |

### 4.4 Repository registration and public surfaces

| Surface | Required work |
| --- | --- |
| Workspace | Add `games/briar_circuit` to the root workspace and lockfile through normal Cargo resolution. |
| CI catalog | Add `briar_circuit` to `ci/games.json`; keep discovery checks green. |
| Simulator | Add the crate dependency/dispatch, require `--seat-count 4`, preserve seat-keyed deterministic summaries, and report threshold/tie/moon terminal reasons. |
| Replay checker | Register `briar_circuit`; check internal traces, all view hashes, diagnostics, and viewer-scoped export stability. |
| Fixture checker | Register manifests, typed fixture validation, unknown-field rejection, version consistency, and behavior-key rejection. |
| Rule coverage | Register `briar_circuit` and the `BC-*` rule prefix; fail on undocumented or unproved rules. |
| WASM crate | Add dependency, catalog entry (`crates/wasm-api/src/constants.rs` `GAME_BRIAR_CIRCUIT`/`…_DISPLAY_NAME` consts plus the `catalog.rs` registration and `games.rs` module/dispatch), fixed-four-seat metadata, game dispatch, setup/action/view/effect/replay/bot operation groups, viewer modes, outcome explanation, and pairwise no-leak harness dispatch. No new exported API schema is expected. |
| WASM game adapter | Add the current-convention adapter, expected as `crates/wasm-api/src/games/briar.rs` or a reassessment-approved equivalent. It translates opaque envelopes only; it does not reimplement game rules. |
| Web catalog | Add **Briar Circuit** metadata, neutral icon, rules link, supported seats, modes, and renderer mapping. |
| Web renderer | Add `apps/web/src/components/BriarCircuitBoard.tsx`; reuse `SeatFrame`, shared action controls, outcome panel, replay surfaces, and effect scheduler rather than rebuilding the shell. |
| Public rules | Add `apps/web/public/rules/briar_circuit.md` and its manifest row, generated from or consistency-checked against game-local original prose. |
| Effects/presentation | Register semantic presenters only where useful: deal counts, pass completion, card play, trick capture, hearts broken, hand score, moon, dealer/pass rotation, and terminal outcome. Generic feedback remains acceptable where custom motion adds no product value. |
| E2E | Add `apps/web/e2e/briar-circuit.smoke.mjs`, pairwise/observer DOM assertions, keyboard play, hotseat handoff, reduced-motion path, replay import/export, moon and terminal explanation checks. |
| Catalog documentation | Update all catalog-enforced surfaces and pass `scripts/check-catalog-docs.mjs`: the `apps/web/README.md` lists, the root `README.md` "current official games are" list, and the `apps/web/package.json` `smoke:e2e` bullet. |

### 4.5 Static data boundary

`manifest.toml` and `variants.toml` may contain typed identity, display metadata, version anchors, official seat metadata, and presentation labels. Fixtures and traces may contain evidence. They must not encode follow-suit conditions, pass routing procedures, heart-breaking exceptions, trick comparators, moon formulas, terminal/tie logic, bot priorities, or visibility selectors. The standard rules are typed Rust, not an interpreted table or mini-language.[^R4][^R10]

## 5. Work breakdown

The following are candidate `AGENT-TASK` packets for later decomposition. Each packet must remain bounded, preserve the failing-test protocol in `docs/AGENT-DISCIPLINE.md`, and deliver complete files or coherent complete sections rather than broad cleanup.[^R18] The `G16-BC-NNN` labels below are illustrative spec-internal references; `/spec-to-tickets` assigns the canonical ticket IDs (`GAT16BRICIR-NNN`, per the established `tickets/<PREFIX>-NNN.md` convention) at decomposition time.

| Candidate | Depends on | Bounded packet | Required outputs and evidence |
| --- | --- | --- | --- |
| `G16-BC-001` | Spec acceptance | **Source, identity, and requirements admission.** Lock Briar Circuit naming, source notes, normative rule decisions, rule IDs, seat declaration, surface budgets, and original-prose stance before gameplay code. | Initial `SOURCES.md`, `RULES.md`, `HOW-TO-PLAY.md`, `RULE-COVERAGE.md`, `GAME-IMPLEMENTATION-ADMISSION.md`; source-divergence table; no copied prose. |
| `G16-BC-002` | `001` | **Reopen primitive pressure before rule implementation.** Compare Plain Tricks and Briar Circuit trick/private-hand shapes using the template fields; update both inventories and the atlas. | `games/briar_circuit/docs/PRIMITIVE-PRESSURE-LEDGER.md`; updates to `games/plain_tricks/docs/MECHANICS.md` and ledger; atlas rows marked second use/repeated-shape candidate; decision `keep local/defer`; no code promotion; debt remains none. |
| `G16-BC-003` | `001`, `002` | **Crate skeleton, typed cards, setup, deterministic deal, and phase state.** | New crate/data/docs/test skeleton; fixed-four-seat diagnostic; canonical 52-card model; deterministic shuffle/deal; dealer/pass cycle; setup/property/serialization tests. |
| `G16-BC-004` | `003` | **Private pass phase.** Implement select/unselect/confirm, pending seats, atomic exchange, hold hand, filtered effects/views, and stable diagnostics. | Rule/unit/property tests for all directions and malformed choices; pass-in-flight no-leak tests; deterministic action/effect/view hashes. |
| `G16-BC-005` | `003` | **Trick play legality and resolution.** Implement 2♣ opening, follow-suit, first-trick restriction/exception, hearts-broken lead rules, comparator, capture, and winner-leads. | Rule and property tests; illegal/stale diagnostics; semantic effect ordering; legal-tree tests; no TypeScript rule code. |
| `G16-BC-006` | `004`, `005` | **Hand scoring, moon, match accumulation, threshold/tie, and outcome breakdown.** | Typed scorer; 26-point conservation property; moon transform; 100 threshold; unique-low terminal; low-tie continuation; public per-seat explanations; scoring traces. |
| `G16-BC-007` | `004`, `005`, `006` | **Four-seat visibility and semantic-effect boundary.** | Observer + four seat projections; effect filters; private action trees/previews; stable viewer matrix; 12 ordered pairwise tests plus observer checks over every required surface. |
| `G16-BC-008` | `006`, `007` | **Replay, serialization, and golden-trace pack.** | Internal full traces; deterministic hashes; viewer-scoped observation exports/imports; seat-private export tests; trace-schema fields and migration notes; minimum trace set in §7.6. |
| `G16-BC-009` | `005`, `006`, `007` | **Bots and simulation.** | L0 random legal; bounded L1 rule-informed policy; viewer-safe explanations; bot legality/no-leak tests; seeded 1,000-match smoke; seat-keyed summaries; no forbidden algorithms. |
| `G16-BC-010` | `003`–`009` | **Fixtures, rule-coverage tooling, and native registrations.** | Fixture/replay/rule-coverage/simulate dispatch; `BC-*` validation; CI game row; root workspace; command evidence. |
| `G16-BC-011` | `007`–`010` | **WASM catalog and operation groups.** | Catalog metadata, fixed seat support, game adapter/dispatch, setup/action/view/effect/replay/bot/outcome operations, snapshots, pairwise no-leak test dispatch; unchanged public API schema unless stopped for ADR. |
| `G16-BC-012` | `011` | **Public web renderer and accessibility.** | `BriarCircuitBoard.tsx`, catalog/icon/rules registration, legal-only card/pass controls, hotseat handoff, score/trick/outcome surface, keyboard and screen-reader semantics, reduced motion, e2e/DOM/storage no-leak proof. |
| `G16-BC-013` | `008`–`012` | **Benchmarking and calibrated CI floors.** | Native benches and largest fixtures, provisional floors, benchmark report, variance-aware thresholds, WASM smoke comparison, no weakened checks. |
| `G16-BC-014` | All prior | **Documentation and public-release closeout.** | Complete all game docs/checklists, central source/atlas/web docs, exact command log, trace inventory, benchmark receipt, catalog checks, `specs/README.md` status `Done`, archival workflow receipt. |

No candidate authorizes opportunistic refactors. A failing test is first evaluated for continued validity, then located in the system under test or test, then fixed without deleting or weakening valid coverage.

## 6. Exit criteria

### 6.1 ROADMAP §15 Gate 16 mapping — row for row

| ROADMAP Gate 16 exit row | Briar Circuit completion criterion | Required evidence |
| --- | --- | --- |
| Passing, lead/follow obligations, trick capture, round scoring, match accumulation, and shoot-the-moon or scoped equivalent are covered for the chosen variant | All locked rules in §3.1 have named `BC-*` IDs, typed Rust implementation, positive/negative unit tests, property tests, golden traces, simulation reachability, replay/hash checks, serialization checks, and original player documentation. Fixed shoot-the-moon is implemented, not replaced by a stripped equivalent. | `RULES.md`, `RULE-COVERAGE.md`, tests, §7.6 traces, 1,000-match simulation, rule-coverage command, replay-check command. |
| Four-seat private-hand no-leak holds across views, UI, replay exports, logs, storage, and bot explanations | Every ordered source-seat → unauthorized-viewer pair passes on Rust projections, legal trees, previews, diagnostics, effects, bot candidates/explanations, public and seat-private exports, WASM payloads, DOM/CSS/test IDs, console/dev logs, screenshots/fixtures, and storage. Pass choices and deck order receive the same protection as hands. | §7.3–§7.5 matrices; native visibility tests; WASM no-leak harness; browser e2e; export token scans; observer and four seat view hashes. |
| Trick-taking helper pressure is recorded and resolved or deferred through the mechanic atlas before later gates depend on it | Plain Tricks ↔ Briar Circuit comparison is completed before gameplay implementation; follow-suit, comparator, winner-leads, deal rotation, and private-hand rows become second-use/repeated-shape evidence; one decision is recorded: keep local/defer; no helper is promoted; open debt remains `_None_`; Gate 17 is the next hard-gate trigger. | Game-local ledger, Plain Tricks inventory update, `docs/MECHANIC-ATLAS.md` update, boundary check, source review showing no new `game-stdlib` trick helper and no kernel nouns. |

### 6.2 Official-game completion criteria

Briar Circuit is not `Done` until all of the following are true:

- the fixed-four-seat setup path accepts four and rejects every other count in Rust;
- all official game documents are complete, internally consistent, and free of copied prose;
- human-vs-bot, hotseat, bot-vs-bot replay, public observer, and seat-private viewer flows work in the static/local browser;
- L0 and L1 bots finish legal seeded matches using authorized views only;
- every required trace, rule, property, replay, serialization, visibility, bot, tool, benchmark, WASM, and UI smoke check passes;
- no hidden token is found on an unauthorized surface, including terminal exports;
- Rust produces the final per-seat result and explanation; TypeScript only renders it;
- native benchmark floors are calibrated and checked without sacrificing explanation fidelity or visibility filtering;
- the public surface is readable, keyboard-operable, responsive, reduced-motion safe, and not debug-first;
- the public release checklist and central catalog/source/atlas documentation are closed.

## 7. Acceptance evidence

### 7.1 Required command suite

Implementation closeout records the exact commands, versions, environment, and outcomes. If a CLI has changed by implementation time, reassessment may correct syntax while preserving or strengthening coverage.

```text
cargo fmt --all -- --check
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo check --workspace
cargo test -p briar_circuit
cargo test -p briar_circuit --test rules
cargo test -p briar_circuit --test property
cargo test -p briar_circuit --test replay
cargo test -p briar_circuit --test serialization
cargo test -p briar_circuit --test visibility
cargo test -p briar_circuit --test bots
cargo test -p wasm-api
cargo test --workspace
cargo run -p fixture-check -- --game briar_circuit
cargo run -p rule-coverage -- --game briar_circuit
cargo run -p replay-check -- --game briar_circuit
cargo run -p replay-check -- --game briar_circuit --all
cargo run -p simulate -- --game briar_circuit --seat-count 4 --games 1000 --start-seed 1600 --action-cap 4096
cargo bench -p briar_circuit
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

`--action-cap 4096` is an operational simulation guard, not a rules terminal condition. A cap hit must emit a reproducible failure seed/command record and must not be silently counted as a draw.

### 7.2 Test taxonomy

| Test class | Required Briar Circuit coverage |
| --- | --- |
| Unit | Card ordering, suit/rank identities, pass direction mapping, across-seat mapping, score values, moon transform, comparator, deterministic ordering, diagnostic formatting. |
| Rule | Every `BC-*` legality, transition, scoring, terminal, and tie rule with positive and negative cases. |
| Property | 52 unique cards; 13 per seat after deal and exchange; no duplicate/lost cards; legal set never empty in a non-terminal acting state; follow-suit closure; off-suit never wins; captured cards partition played cards; raw hand points total 26; moon predicate iff one seat owns all point cards; scores monotonic; deterministic setup/replay. |
| Simulation | At least 1,000 seeded four-seat bot matches; zero illegal actions, panics, impossible phases, duplicate cards, stale internal choices, or unauthorized bot inputs; moon and threshold/tie counters reported. |
| Golden trace | Minimum set in §7.6 with Trace Schema v1 fields, hashes, diagnostics, and update notes. |
| Replay | Internal seed+command replay reproduces state/effect/action/view hashes; public/seat-private observation exports are byte-stable and import to the same authorized timeline. |
| Serialization | Canonical map/list/card/seat ordering; strict unknown-field rejection; version anchors; round-trip state/evidence as applicable; no hidden fields in viewer exports. |
| Visibility | Viewer matrix and all 12 ordered pairwise seat checks across every §7.5 surface; public observer checks; terminal non-reveal checks. |
| Bot | L0/L1 choose only legal actions; same declared inputs and seed yield same choice; no hidden fields consumed; explanations contain authorized facts only. |
| WASM/API | Catalog, fixed seat metadata, all operation groups, stale/invalid diagnostics, observer/seat projections, replay classes, outcome, snapshots, pairwise harness. |
| UI/e2e | Legal-only card/pass controls, 2♣ opening, handoff privacy, pending seats, trick winner/next leader, score/moon/terminal explanation, keyboard flow, accessible names, reduced motion, observer/seat DOM and storage scans. |
| Benchmark | Setup/deal, pass action generation/apply/exchange, play legal tree/apply, trick resolution, hand/moon scoring, observer/four-seat projection, effect filtering, replay export/import, bot action, full hand, full match. |

### 7.3 Viewer matrix

| Viewer | View payload | Action tree / preview | Effects and explanations | Replay/export | Browser obligations |
| --- | --- | --- | --- | --- | --- |
| Public observer | Public phase, dealer/pass direction, active/pending seats, hand counts, current/completed public plays, broken status, trick winners, scores, public outcomes; no private card identities | No seat-private tree or preview; only public pending metadata | Public effects and public score/outcome reasons only | Default viewer-scoped observation timeline; no seed/private command paths/deck order | Safe for screenshots, DOM, dev panel, logs, and storage |
| `seat_0` viewer | Public facts plus `seat_0` unplayed hand, own pass selection/commit receipt, own authorized preview | Only `seat_0`'s legal tree when acting | Public effects plus private effects addressed to `seat_0`; own bot explanation only when applicable | Explicitly labelled seat-private timeline containing only facts `seat_0` knew at each checkpoint | No private facts of seats 1–3 in any browser surface |
| `seat_1` viewer | Same rule for `seat_1` | Same | Same | Same | No private facts of seats 0, 2, 3 |
| `seat_2` viewer | Same rule for `seat_2` | Same | Same | Same | No private facts of seats 0, 1, 3 |
| `seat_3` viewer | Same rule for `seat_3` | Same | Same | Same | No private facts of seats 0–2 |
| Internal native test/dev authority | May inspect full deterministic state and internal full trace | May inspect all for tests | May inspect unfiltered evidence | Internal full trace only; never default browser export | Must be clearly separated from public/WASM payloads |
| Team viewer | `not applicable` — no teams | `not applicable` | `not applicable` | `not applicable` | Gate 18 owns partnership semantics |

### 7.4 Pairwise no-leak matrix — every ordered seat pair

For each row, the unauthorized viewer must not receive the source seat's unplayed cards, staged/committed pass cards, incoming cards before exchange, pass provenance, private action tree/preview, private diagnostic detail, private effect payload, bot candidates/rankings/explanation inputs, or any seed/deck fact that reconstructs them. When a card is legally played, its identity becomes public; that does not make the earlier pass association public.

| Private source | Unauthorized viewer | Required result |
| --- | --- | --- |
| `seat_0` | `seat_1` | No source-private datum on any tested surface |
| `seat_0` | `seat_2` | No source-private datum on any tested surface |
| `seat_0` | `seat_3` | No source-private datum on any tested surface |
| `seat_1` | `seat_0` | No source-private datum on any tested surface |
| `seat_1` | `seat_2` | No source-private datum on any tested surface |
| `seat_1` | `seat_3` | No source-private datum on any tested surface |
| `seat_2` | `seat_0` | No source-private datum on any tested surface |
| `seat_2` | `seat_1` | No source-private datum on any tested surface |
| `seat_2` | `seat_3` | No source-private datum on any tested surface |
| `seat_3` | `seat_0` | No source-private datum on any tested surface |
| `seat_3` | `seat_1` | No source-private datum on any tested surface |
| `seat_3` | `seat_2` | No source-private datum on any tested surface |
| `seat_0` | Public observer | No source-private datum on any tested surface |
| `seat_1` | Public observer | No source-private datum on any tested surface |
| `seat_2` | Public observer | No source-private datum on any tested surface |
| `seat_3` | Public observer | No source-private datum on any tested surface |

The test harness must generate canary identifiers for every seat and hidden category, then search structured values and serialized strings. A pass is not merely “the component did not render it”; the unauthorized datum must be absent before React receives the payload.

### 7.5 Mandatory no-leak surfaces and datum taxonomy

| Surface | Assertions |
| --- | --- |
| Rust public/private views | Only owner view contains owner hand/selection; counts are public, identities are not; projection is generated before serialization. |
| Legal action trees | Only the authorized acting viewer receives card-bearing action paths. Non-actors and observer receive no private alternatives. |
| Previews | A preview may mention the actor's selected/played card to that actor, but no opponent hand, pass origin, future play, or deck order. |
| Diagnostics | Stable public reason without echoing hidden alternatives or opponent contents. Owner-specific unowned-card diagnostics must not expose the actual owner. |
| Semantic effects / effect log | Private deal/pass effects are correctly scoped. Public commitment effects carry counts/status only. Card identity becomes public only through a legal public play effect. |
| Bot inputs | Own projected hand, public history/state, legal actions, declared RNG only. No raw state, opponent hand, committed pass map, deck tail, omniscient trick forecast, or hidden-state-derived feature. |
| Bot explanation/candidates | Viewer-safe chosen-action rationale; L0 exposes only random-legal metadata; L1 exposes public/own-hand facts. Candidate rankings remain private to authorized bot/test surfaces and never enter observer export. |
| Internal traces | May be omniscient, but are test/dev artifacts clearly classified and never used as browser exports. |
| Public replay export/import | Observation timeline only; no seed that reconstructs deal, raw private action paths, commitments, private hands, deck order, or terminal auto-reveal. |
| Seat-private replay export/import | Includes only that seat's facts as known at each point; never retroactively reveals other initial hands or pass choices. |
| WASM JSON | Same viewer filtering as Rust; no convenience “all state” property, debug field, or hidden fallback. |
| DOM and accessibility tree | No hidden values in text, attributes, ARIA labels/descriptions, offscreen nodes, CSS class names, `data-*`, `data-testid`, keys, or comments. Face-down card nodes use count/position only. |
| Storage / clipboard / URL | No hidden match state, seed, private command path, or card IDs in local/session storage, IndexedDB, URLs, replay defaults, clipboard payloads, or crash recovery. |
| Logs / dev panel / errors | No hidden cards or pass choices in console, telemetry-like output, panic/error strings, snapshots, screenshots, or dev panel data. |
| Tests and fixtures claiming public scope | No omniscient fixture embedded in a public/e2e asset. Internal full fixtures remain native-only and labelled. |

### 7.6 Golden-trace minimum set

All traces use Trace Schema v1 and record `game_id = "briar_circuit"`, `rules_version = "briar-circuit-rules-v1"`, exact variant/data/engine versions, ordered four-seat array, explicit actor seat, freshness token, commands, checkpoints, state/effect/action-tree/public-view hashes, private-view hashes for all four seats, diagnostics where applicable, terminal expectation, and non-empty migration/update notes.[^R11]

Minimum trace inventory:

1. `setup-four-seat-deterministic-deal.trace.json`
2. `invalid-seat-count-below.trace.json`
3. `invalid-seat-count-above.trace.json`
4. `deal-private-no-leak.trace.json`
5. `pass-left-atomic-exchange.trace.json`
6. `pass-right-atomic-exchange.trace.json`
7. `pass-across-atomic-exchange.trace.json`
8. `hold-hand-no-pass.trace.json`
9. `pass-choice-in-flight-no-leak.trace.json`
10. `invalid-pass-not-three.trace.json`
11. `invalid-pass-unowned-or-duplicate.trace.json`
12. `two-clubs-forced-opening.trace.json`
13. `follow-suit-forced.trace.json`
14. `void-free-discard.trace.json`
15. `first-trick-points-suppressed.trace.json`
16. `first-trick-all-points-exception.trace.json`
17. `hearts-not-broken-lead-diagnostic.trace.json`
18. `only-hearts-lead-exception.trace.json`
19. `heart-discard-breaks-hearts.trace.json`
20. `queen-spades-does-not-break-hearts.trace.json`
21. `off-suit-never-wins.trace.json`
22. `trick-winner-leads-next.trace.json`
23. `normal-hand-scoring.trace.json`
24. `shoot-the-moon-fixed-addition.trace.json`
25. `dealer-and-pass-cycle-rotation.trace.json`
26. `threshold-unique-low-winner.trace.json`
27. `threshold-low-tie-continues.trace.json`
28. `invalid-wrong-seat-diagnostic.trace.json`
29. `invalid-stale-diagnostic.trace.json`
30. `l0-bot-action.trace.json`
31. `l1-bot-pass-and-play.trace.json`
32. `public-observer-no-leak.trace.json`
33. `seat-private-pairwise-no-leak.trace.json` with all four viewer hashes and canaries
34. `public-replay-export-import.trace.json`
35. `seat-private-replay-export-import.trace.json` with all four export classes
36. `bot-vs-bot-full-match.trace.json`
37. `wasm-exported-moon-terminal.trace.json`

Additional traces are required whenever implementation reveals a distinct failure mode. The minimum is not a cap.

### 7.7 Surface and action-fanout budgets

| Budget | Maximum / required fixture |
| --- | --- |
| Official seats | 4 |
| Private cards per seat | 13 |
| Internal deck/card identities | 52 |
| Public card identities in one hand timeline | Up to 52 after all cards have legally been played; never future/unplayed identities |
| Current trick cards | 4 |
| Completed tricks per hand | 13 |
| Pass selection | Exactly 3 per seat on pass hands |
| Simultaneously pending pass seats | 4 logically; deterministic command serialization is allowed, but all remain unresolved until atomic exchange |
| Flat legal card choices | At most 13 |
| Pass selection controls | At most 13 selectable/unselectable cards plus one Rust-authorized confirm action |
| Progressive action depth | At most 3 path segments for a single card/select/confirm operation under the current action envelope; no 286-button combination surface |
| Normal public effect batch | Target ≤8 envelopes; private recipient filtering may produce additional per-seat envelopes internally |
| Pass-exchange internal effect batch | Target ≤20 deterministic envelopes, with public summary plus private send/receive facts |
| Hand/terminal scoring effect batch | Target ≤20, preferably grouped into Rust-authored per-seat breakdown effects |
| Largest native fixture | Four seats, 13-card hands, all pending pass commitments, point-heavy final trick, moon transformation, cumulative scores near threshold |
| Largest browser fixture | Four seat rails, 13-card owner hand, current trick, 13-trick history/replay timeline, score table, outcome explanation, observer/seat switcher |

If implementation exceeds a budget, update this spec through review and rerun the related benchmark/UI/no-leak proof; do not conceal the larger surface.

### 7.8 Benchmark expectations

Native Rust is the performance source of truth; WASM/browser checks are secondary product latency evidence.[^R7]

Required operations:

- setup, shuffle, deal, and canonical serialization;
- pass legal-action generation, selection apply, commitment, and atomic exchange;
- play legal-action generation in maximum-hand and constrained-follow states;
- validation/apply and trick resolution;
- normal and moon hand scoring;
- match-threshold/tie evaluation and outcome construction;
- public observer plus each of four seat projections;
- effect filtering for public and each seat;
- full internal trace replay;
- full viewer-scoped public and seat-private export/import;
- L0 and L1 action selection;
- full seeded 13-trick hand;
- full seeded match to unique-low terminal.

Provisional targets, subject to documented calibration on the CI reference environment:

- p95 native legal-action generation, validation/apply, trick resolution, and any one viewer projection: `< 1 ms` in the largest relevant fixture;
- p95 native full-hand viewer-scoped export generation: `< 10 ms`;
- p95 native full-match viewer-scoped export/import: `< 50 ms` for the benchmark fixture;
- release-mode random/legal full-match throughput: at least `100 completed matches/second` on the reference machine, or a reviewed replacement floor justified by measured baseline evidence;
- browser-visible Rust/WASM setup, action, preview, and view-refresh operations remain inside the existing provisional `< 100 ms` public interaction budget.

`thresholds.json` must record environment and variance-aware floors. Calibration may replace an unrealistic provisional number, but it may not remove the operation, hide a regression, bypass visibility filtering, weaken explanation detail, or introduce a lookup/search shortcut.

## 8. FOUNDATIONS & boundary alignment

### 8.1 Contract alignment

| Principle / contract | Briar Circuit stance | Required proof |
| --- | --- | --- |
| Public playable product first | Full classic match, polished fixed-four-seat table, legal-only interaction, readable score/trick state, replay, and accessible handoff are core gate work, not optional polish. | Web smoke, release checklist, human review. |
| Rust behavior authority | Setup, deal, pass routing, legality, validation, trick resolution, scoring, moon, terminal/tie logic, effects, views, replay, serialization, and bots are Rust-owned. | Boundary review and tests. |
| TypeScript presentation only | React maps Rust/WASM action IDs and views to controls/visuals. It does not infer legal cards, broken state, winner, score, pass destination, threshold, or private facts. | Code review, boundary script, e2e legal-only assertions. |
| `engine-core` generic | No card, deck, hand, suit, rank, trick, heart, pass, dealer, score, moon, or match noun is added. Existing generic actor/viewer/action/effect/replay contracts are sufficient. | Diff review and boundary check. |
| `game-stdlib` earned | No Gate 16 trick/card/private-hand promotion. Local implementation plus comparison is required for this second close use. | Ledger and atlas update; no new helper module. |
| Static data is not behavior | TOML/JSON holds typed identity, metadata, parameters/evidence only. No conditions, selectors, pass cycle procedures, scoring formulas, bot rules, or visibility rules. | Fixture checker and manual audit. |
| Determinism | Seeded shuffle, seat/dealer/pass ordering, card/action/effect ordering, hashes, serialization, replay, bot tie-breaks, and exports are deterministic. | Property, replay, serialization, trace checks. |
| Hidden-information safety | Pairwise redaction applies to hands, pass choices, deck order, legal trees, previews, effects, bots, replay, browser, logs, storage, a11y text, and tests. | §7.3–§7.5 evidence. |
| Replay taxonomy | Internal full trace remains native authority; browser default is observer-scoped observation timeline; seat-private export is explicitly labelled. | ADR 0004 conformance tests.[^R12] |
| Bot law | L0 and bounded L1 use legal APIs and authorized views. L2 is deferred; no MCTS/ISMCTS/Monte Carlo/ML/RL. | AI docs, input-field audit, bot tests/simulations. |
| Semantic effects | Deal/pass/card/trick/score/moon/terminal animation and logs originate in Rust effects; renderer diffs are diagnostic only. | Effect tests and animation smoke. |
| Local-first | Human/bot, hotseat, bot replay, observer, replay import/export only. No hosted service scope. | Catalog/mode metadata and code review. |
| IP conservatism | Neutral name, original prose and assets, no casino/card-brand trade dress, source-use notes and human release review. | Sources and release checklist. |
| Evidence-heavy official game | Docs, rule coverage, traces, replay, serialization, simulation, bots, benchmarks, UI smoke, and source notes are all gate blockers. | Official-game checklist. |

### 8.2 Primitive-pressure comparison and decision

| Mechanic shape | Plain Tricks first use | Briar Circuit second use | Repeated core | Material divergence | Gate 16 decision |
| --- | --- | --- | --- | --- | --- |
| Card/suit/rank/hand representation | Reduced local deck and two private hands | Full 52-card deck and four private 13-card hands plus pass staging | Stable typed identity, owner-only projection, deterministic ordering | Deck composition, seat count, pass ownership changes, point semantics, surface size | Keep game-local; no shared card noun/helper |
| Follow-suit legality | Follower must use led suit if held; otherwise any card | Same obligation, plus 2♣ opening, first-trick point restriction/exception, hearts lead restriction | “Available led suit constrains legal set” | Briar Circuit has phase/point/lead exceptions that would force policy flags | Keep local; record repeated-shape candidate |
| Trick resolution / led-suit comparator | Highest rank of led suit wins; off-suit cannot win | Same no-trump comparator | Led suit and rank determine winner | Briar Circuit attaches penalty capture, broken state, 13-trick hand, four-way ordering | Keep local; Gate 17 hard-gates a possible narrow comparator |
| Trick-winner-leads sequencing | Winner leads next unless round ends | Winner leads next unless 13th trick ends hand | Winner-derived next actor | Four seats, pass/dealer cycle, hand scoring, threshold continuation | Keep local; no generic turn policy |
| Deal rotation / redeal | Two short rounds with continuing RNG and alternating starter | Full redeal every hand, rotating dealer and independent pass cycle; 2♣ holder leads | Deterministic repeated deals and seat-relative allocation | Different deck, seat count, leader rule, lifecycle, pass phase | Keep local; defer |
| Private-hand projection | Two owner-only hands/tail redaction | Four hands, pairwise matrix, pass commitments, viewer-scoped exports | Rust owner projection and generic visibility envelopes | N-seat pairwise proof and additional hidden categories | Reuse existing generic viewer/effect/replay infrastructure only; no card-hand primitive |
| Trick/hand scoring | Positive trick counts and a small fixed match | Penalty cards, moon transformation, cumulative threshold and tie continuation | Public post-trick/hand accounting | Scoring meaning and terminal policy are incompatible | Game-local only; not a shared scoring helper |
| UI interaction | Small hand, lead/follow card controls | 13-card hand, private multi-select pass, four-seat table, score history | Rust legal actions and effect-driven play | Fanout, handoff privacy, pass commitment, four-seat layout | Reuse shell/components, not rule helpers |
| Bot policy | Small local trick heuristic | Penalty avoidance, pass heuristic, moon awareness only at bounded L1 | Legal API and authorized view | Strategy priorities and information surface differ | Policies remain game-local |

**Decision:** `defer-reject / keep local` for Gate 16. Local duplication is acceptable and safer because the shared portion is small, behavior-bearing exceptions differ, and the third close use has not yet landed. No back-port code work is required. Documentation updates to Plain Tricks are required so Gate 17 can compare real evidence rather than chat memory. No promotion debt is created.

**Next review trigger:** before Gate 17 Oh Hell implements a third close follow-suit/comparator/winner-leads/deal shape, its primitive-pressure ledger must choose reuse, narrow promotion, explicit defer/reject, or ADR. Gate 17 may not simply copy a third implementation.

Suggested atlas row updates:

```markdown
| follow-suit legality | `plain_tricks`, `briar_circuit` | `repeated-shape candidate` / second close use | Gate 16 compares the shared must-follow-suit shape but keeps both implementations local because Briar Circuit adds first-lead, first-trick point, hearts-broken, four-seat, and pass-phase policy. No promotion. | Hard-gate before Gate 17 Oh Hell implements a third close use. |
| trick resolution / led-suit comparator | `plain_tricks`, `briar_circuit` | `repeated-shape candidate` / second close use | Both use highest led-suit rank with no trump, but Briar Circuit couples capture to penalty cards, broken state, four-seat flow, moon, and cumulative scoring. Keep local. | Hard-gate at Gate 17. |
| trick-winner-leads turn order | `plain_tricks`, `briar_circuit` | `repeated-shape candidate` / second close use | Winner-derived next leader repeats; hand/round/deal lifecycle differs. Keep local. | Hard-gate at Gate 17. |
| deal rotation / trick-round redeal | `plain_tricks`, `briar_circuit` | `repeated-shape candidate` / second close use | Both redeal deterministically, but dealer, pass cycle, first leader, deck, seats, and terminal lifecycle differ. Keep local. | Revisit at Gate 17. |
```

The existing Gate 15/15.1 card/private-hand bundle remains defer/reject evidence. Briar Circuit consumes generic N-seat viewer/replay/tooling seams but adds no cross-game card evaluator, deck, hand, or trick API.

### 8.3 Stop conditions

Implementation stops and returns to reassessment if any of the following occurs:

- a required game noun or policy is proposed for `engine-core`;
- a trick/card helper is proposed for `game-stdlib` without replacing this spec through the atlas process;
- data files begin expressing legality, scoring, pass routing, visibility, or bot behavior;
- TypeScript computes legal cards, trick winner, broken state, points, moon, threshold, or pass destination;
- normal mode renders a clickable illegal card and relies on later validation;
- animation causality is guessed from state diffs instead of Rust effects;
- any seat-private canary reaches an unauthorized seat, observer, export, DOM, accessibility tree, storage, log, test ID, bot explanation, or candidate ranking;
- a bot reads raw state or bypasses the legal-action/validation path;
- an L2 implementation begins without an accepted strategy-evidence pack;
- trace fields, public/private visibility semantics, or hash meaning must change without an accepted ADR;
- the second-use comparison is skipped, the atlas is not updated, or Gate 17 is allowed to proceed without the third-use decision;
- official-game evidence or polished public UI is being cut to preserve schedule;
- valid failing tests are deleted, weakened, or reclassified without first establishing that the test is obsolete.

## 9. Forbidden changes

This gate must not:

- modify `engine-core` vocabulary or responsibilities;
- add card, deck, hand, suit, rank, trick, pass, hearts, moon, dealer, or game-scoring types to `engine-core`;
- add a trick-taking/card/private-hand helper to `game-stdlib`;
- alter `game-stdlib::board_space` or unrelated primitives;
- change Plain Tricks gameplay, traces, hashes, action paths, or renderer behavior; only the required inventory/ledger comparison documentation may change unless a separately proven bug receives explicit scope;
- revisit, rebuild, or extend River Ledger or its side-pot work;
- change Trace Schema v1, replay/hash meaning, or the accepted hidden-info export taxonomy;
- export an internal full hidden-information trace to the browser as the default replay;
- add a WASM “get all state” escape hatch or rely on client-side hiding;
- add YAML, a DSL, behavior-looking configuration, scoring formulas, conditional selectors, or bot strategy to static data;
- introduce hosted multiplayer, accounts, databases, matchmaking, chat, ranked play, server persistence, or tournament systems;
- introduce MCTS, ISMCTS, Monte Carlo/rollout sampling, ML, RL, external solvers, hidden-state sampling, belief search, or omniscient candidate scoring;
- claim L2 competence from L0/L1 evidence;
- copy rules prose, card art, branded table appearance, screenshots, fonts, icons, or commercial trade dress;
- use casino/real-money/chip/rake/betting presentation;
- add house variants, shoot-the-sun, Omnibus/J♦, partnerships, variable seats, or a configurable rules matrix;
- replace React/SVG with Canvas/PixiJS absent the required profiling evidence/ADR;
- perform broad shell, tool, engine, replay, or game-crate cleanup;
- decompose this spec into ticket files as part of the spec-authorship deliverable;
- weaken, delete, skip, or quarantine valid tests to obtain green CI.

## 10. Documentation updates required

### 10.1 `specs/README.md`

When this spec lands:

- replace Gate 16's seed/unwritten link with `gate-16-briar-circuit-trick-taking.md`;
- change status from `Not started` to `Planned`;
- note fixed four-seat classic Hearts under the neutral name Briar Circuit, full pass/trick/penalty/moon/match scope, and second-use keep-local comparison;
- leave Gate 17 `Not started` and explicitly pending Gate 16.

At implementation closeout, change Gate 16 to `Done` only after every §6/§7 receipt exists, include the completion date/evidence summary, and follow `docs/archival-workflow.md` for any later archive move. Do not mark `Done` when the browser merely appears playable.

### 10.2 Central source and IP documentation

Update `docs/SOURCES.md` with a Briar Circuit/Hearts research entry that records:

- Pagat and Bicycle as rules references, facts used, access date, and their relevant divergence;
- OpenSpiel Hearts only as external implementation prior art for phase/observation modeling, with no code or architecture imported;
- W3C accessibility references used for target size, keyboard/semantics, and reduced motion;
- original Rulepath name/prose/assets statement;
- the deliberate rules choices: strict first-trick point rule with no-alternative exception, Q♠ does not break hearts, fixed add-26 moon, 100 threshold, low-tie continuation, no shoot-the-sun.

### 10.3 Mechanic atlas and Plain Tricks comparison docs

Update:

- `docs/MECHANIC-ATLAS.md` §10 rows for follow-suit, led-suit comparator, winner-leads, and deal/redeal from first use to second-use/repeated-shape candidate;
- §9A evidence note to show Hearts comparison completed while Oh Hell remains the third-use trigger;
- §10A debt register remains `_None_` unless a different accepted decision explicitly creates named debt — this spec creates none;
- `games/plain_tricks/docs/MECHANICS.md` and `games/plain_tricks/docs/PRIMITIVE-PRESSURE-LEDGER.md` with the second-use comparison and no behavior change;
- all Briar Circuit game-local docs in §4.3.

### 10.4 `apps/web/README.md` and catalog-enforced surfaces

Per the official-game contract and catalog checker, update all applicable lists, not merely the intro paragraph:

- **Introduction/catalog game list:** add Briar Circuit.
- **Shell Surface renderer list:** add `BriarCircuitBoard`.
- **Action presentation audit:** record Briar Circuit as `board-native`/game-native because its renderer maps Rust legal actions to card/pass controls without computing legality.
- **Effect animation audit:** honestly record `adopt` for custom semantic presenters that ship, or `generic-only` for effects left to shared feedback.
- **Smoke Layers `smoke:e2e` list:** add `briar-circuit.smoke.mjs`.
- **No-leak/a11y checklist:** add public observer, all four seat-private DOM/storage checks, pass handoff, keyboard card/pass controls, and reduced-motion verification.

`scripts/check-catalog-docs.mjs` enforces catalog parity beyond `apps/web/README.md`; the same update must also reach:

- the **root `README.md`** "current official games are" list;
- the `apps/web/package.json` **`smoke:e2e`** bullet that names `briar-circuit.smoke.mjs`.

`node scripts/check-catalog-docs.mjs` must pass.

### 10.5 Public rules and registration documentation

Update or create:

- `apps/web/public/rules/briar_circuit.md`;
- `apps/web/public/rules/manifest.json`;
- player-rules copy/check expectations;
- root workspace registration;
- `ci/games.json`;
- WASM catalog, snapshots, game dispatch, and tests;
- `simulate`, `replay-check`, `fixture-check`, and `rule-coverage` registrations;
- outcome-explanation registry entries required by `scripts/check-outcome-explanations.mjs`: the viewer-safe rationale mirror in `apps/web/src/wasm/client.ts`, the static copy keys in `apps/web/src/components/outcomeExplanationTemplates.ts`, the "Outcome / victory explanation" section in `games/briar_circuit/docs/UI.md`, and the stable scoring/terminal rule IDs in `games/briar_circuit/docs/RULES.md`;
- presentation-copy expectations enforced by `scripts/check-presentation-copy.mjs` over `BriarCircuitBoard.tsx` and the outcome panel (no debug vocabulary or raw internal identifiers in display copy).

### 10.6 Closeout evidence

The final closeout note or PR description must include:

- exact command/evidence results from §7.1;
- trace inventory and any migration/update notes;
- benchmark environment, baseline, floor, and variance evidence;
- pairwise no-leak matrix result;
- source/IP review result;
- atlas decision and confirmation of no promotion debt;
- public catalog/rules/renderer/smoke receipts;
- status update and successor interlock.

## 11. Sequencing

### 11.1 Admission evidence

| Sequence fact | Repository status | Consequence |
| --- | --- | --- |
| Gate 15.1 River Ledger all-in / side pots | `Done` on 2026-06-20 | Predecessor interlock is clear; do not reopen its work.[^R2] |
| Atlas open promotion debt | `_None_` at Gate 15.1 closeout | No maintenance closure spec precedes Gate 16.[^R3] |
| Gate 16 Hearts | Lowest non-`Done` active-epoch row, Order 7 | This spec is the admitted next gate.[^R2] |
| ROADMAP Gate 16 | Prescriptive fixed-four-seat Hearts gate | Scope and exit mapping in §§3 and 6 are mandatory.[^R1] |
| Gate 17 Oh Hell | Pending Gate 16 | It is the next ladder unit and the third-use trick-taking hard gate.[^R1][^R2][^R3] |

### 11.2 Requirements-first order

1. Accept this spec and mark Gate 16 `Planned`.
2. Complete source/rules/coverage/admission documents and the primitive-pressure comparison.
3. Run `/reassess-spec` against the exact implementation target and current code seams.
4. Only then run `/spec-to-tickets` to create bounded tasks in dependency order.
5. Implement native game behavior and evidence before browser presentation claims.
6. Add tools/WASM/web registrations without moving behavior into adapters or TypeScript.
7. Complete no-leak, replay, benchmark, accessibility, catalog, and release receipts.
8. Mark Gate 16 `Done` only after §6/§7 closure.

### 11.3 Successor rule

Gate 17 must not begin trick-taking behavior implementation until:

- Gate 16 is `Done`;
- Briar Circuit and Plain Tricks inventories/ledgers contain the second-use comparison;
- the atlas has no open promotion debt;
- Gate 17's own ledger explicitly resolves the third close use before coding.

Gate 16 neither promotes a helper on Gate 17's behalf nor weakens the future hard gate.

## 12. Assumptions

Each assumption is one-line-correctable during reassessment without changing the locked product rules or foundation obligations.

| Assumption | Correction rule |
| --- | --- |
| The current root/tool/WASM registries can add one game without exported API/schema changes | If false, stop and determine whether an ADR is required; do not improvise a compatibility break. |
| The existing N-seat `SeatFrame`, viewer selection, pairwise harness, seat-keyed simulator summary, and replay observation APIs can support fixed four seats | If an implementation seam differs, adapt within its current generic contract rather than rebuilding N-seat infrastructure. |
| `crates/wasm-api/src/games/briar.rs` is the likely adapter filename | Reassessment may select the exact current naming convention; behavior remains in `games/briar_circuit`. |
| `seat_0` is the deterministic initial dealer for the standard fixture | A different fixed initial dealer may be selected only before traces ship and must be changed consistently across rules, fixtures, and source notes. |
| Existing card/table presentation components can be reused compositionally without a cross-game card-framework project | If not, create only the smallest presentation-only component needed; no legality or mechanic promotion follows. |
| Trace Schema v1 and a game-local viewer-export version are expressive enough | If not, stop for the replay/visibility ADR process before changing fields or semantics. |
| L0 plus bounded L1 is sufficient for Gate 16 public admission; L2 remains a later evidence-backed enhancement | A request to ship L2 first must supply and accept the strategy-evidence pack before implementation. |
| No genuine contradiction was found among the repository authorities and the settled brief | A later proven contradiction is recorded in reassessment and resolved by authority order, not silently. |

---

# Appendix A — Normative rule and coverage skeleton

The exact prose belongs in `games/briar_circuit/docs/RULES.md`; this table fixes the minimum rule identity set for implementation and coverage.

| Rule ID | Normative requirement | Minimum proof |
| --- | --- | --- |
| `BC-SETUP-001` | Exactly four seats are accepted; all other counts receive a stable diagnostic | unit, rule, trace, WASM |
| `BC-SETUP-002` | Standard deck has 52 unique cards and deterministic canonical IDs | unit, property, serialization |
| `BC-DEAL-001` | Seeded shuffle/deal produces 13 unique private cards per seat and no remainder | property, trace, replay |
| `BC-DEAL-002` | Dealer rotates clockwise after each hand; deal starts left of dealer | rule, trace |
| `BC-PASS-001` | Pass cycle is left, right, across, hold, repeating by hand index | unit, rule, trace |
| `BC-PASS-002` | Each seat selects exactly three distinct owned cards on a pass hand | rule, diagnostic, property |
| `BC-PASS-003` | No incoming cards are delivered until all four commitments; exchange is atomic | rule, trace, visibility |
| `BC-PASS-004` | Hold hand skips selection/exchange and proceeds to 2♣ opening | rule, trace |
| `BC-PLAY-001` | Holder of 2♣ leads it to trick one | rule, trace |
| `BC-PLAY-002` | A seat must follow led suit when able | rule, property, diagnostic, trace |
| `BC-PLAY-003` | A void seat may discard any otherwise legal card | rule, trace |
| `BC-PLAY-004` | Hearts and Q♠ are forbidden on trick one while a non-point discard exists; no-alternative exception allows all | rule, property, two traces |
| `BC-PLAY-005` | Hearts cannot be led unbroken while a non-heart remains | rule, diagnostic, trace |
| `BC-PLAY-006` | If the leader holds only hearts, a heart lead is legal and breaks hearts | rule, trace |
| `BC-PLAY-007` | A played heart breaks hearts; Q♠ alone does not | rule, traces |
| `BC-TRICK-001` | Highest card of led suit wins; off-suit never wins | unit, property, trace |
| `BC-TRICK-002` | Trick winner captures all four cards and leads next unless hand closes | rule, property, trace |
| `BC-SCORE-001` | Each captured heart is 1; captured Q♠ is 13; others 0 | unit, property |
| `BC-SCORE-002` | Raw point total for a complete hand is 26 | property, trace |
| `BC-SCORE-003` | Capturing all 26 triggers fixed moon: shooter +0, each opponent +26 | unit, rule, trace |
| `BC-MATCH-001` | Hand additions accumulate monotonically by seat | property, replay |
| `BC-MATCH-002` | Threshold check occurs after a completed hand when any score ≥100 | rule, trace |
| `BC-MATCH-003` | Unique lowest score wins; tied low continues complete hands without seat-order tie-break | rule, simulation, traces |
| `BC-VIS-001` | Unplayed cards are visible only to their owner | pairwise visibility, trace, e2e |
| `BC-VIS-002` | Pass selection and pass provenance remain owner-only; a card identity may separately become public only when legally played | pairwise visibility, export, e2e |
| `BC-VIS-003` | Deck order and seed-reconstructable material never enter viewer-scoped export | export scan, replay test |
| `BC-VIS-004` | Unauthorized viewers receive no private action tree, preview, diagnostics, effects, bot candidates, or explanation facts | pairwise harness, e2e |
| `BC-REPLAY-001` | Internal replay reproduces deterministic state/effect/action/view hashes | replay, trace |
| `BC-REPLAY-002` | Public and seat-private exports reproduce only authorized observation timelines | replay/export, visibility |
| `BC-BOT-001` | L0 samples uniformly from the Rust legal leaf set using declared bot RNG | bot unit, trace, simulation |
| `BC-BOT-002` | L1 uses only own projected hand, public state/history, legal actions, and deterministic tie-breaks | input audit, bot tests, trace |
| `BC-UI-001` | Browser controls expose legal actions only and do not derive legality | e2e, boundary review |
| `BC-OUTCOME-001` | Rust supplies per-seat hand and cumulative breakdown, moon adjustment, threshold/tie reason, and terminal winner | rule, trace, outcome check |

# Appendix B — Proposed state, action, effect, and view model

This appendix is implementation guidance subordinate to the required behavior. Reassessment may rename types without changing their semantics.

## B.1 State phases

```text
MatchState
  seats: [SeatId; 4]
  dealer: SeatId
  hand_index: u32
  cumulative_scores: SeatMap<u16>
  phase:
    Passing {
      direction: Left | Right | Across,
      selections: private SeatMap<PassSelection>,
      committed: SeatSet,
      next_actor/pending actors: viewer-safe generic seat facts
    }
    Playing {
      hearts_broken: bool,
      trick_index: 0..12,
      leader: SeatId,
      active_seat: SeatId,
      current_trick: ordered public plays,
      hands: private SeatMap<Hand>,
      captured: SeatMap<CapturedCards>
    }
    ScoringHand { typed intermediate only }
    Terminal { OutcomeBreakdown }
```

On the hold hand, setup/deal emits a public pass-skipped effect and enters `Playing` directly. The state transition may retain completed-hand summaries for UI/replay, but it must not retain or expose unnecessary prior private hands or pass-selection provenance.

## B.2 Action shape

Recommended stable action families:

- `pass/select/<card-id>`
- `pass/unselect/<card-id>`
- `pass/confirm`
- `play/<card-id>`

Selection order is stateful and private, avoiding a flat 286-combination UI. Rust supplies the legal set at each state. Confirm exists only at exactly three distinct selections. The public shell receives only pending/committed status for other seats.

## B.3 Diagnostic codes

At minimum:

- `BC_UNSUPPORTED_SEAT_COUNT`
- `BC_WRONG_PHASE`
- `BC_WRONG_SEAT`
- `BC_STALE_COMMAND`
- `BC_CARD_NOT_OWNED`
- `BC_PASS_DUPLICATE_CARD`
- `BC_PASS_REQUIRES_THREE`
- `BC_PASS_ALREADY_COMMITTED`
- `BC_TWO_CLUBS_MUST_OPEN`
- `BC_MUST_FOLLOW_SUIT`
- `BC_FIRST_TRICK_POINT_FORBIDDEN`
- `BC_HEARTS_NOT_BROKEN`
- `BC_TERMINAL_NO_ACTIONS`

Diagnostics identify the violated public rule without exposing another seat's hand or naming hidden legal alternatives.

## B.4 Semantic effects

Public effects may include:

- match/hand started;
- dealer and pass direction established;
- hand count dealt per seat;
- seat committed pass (count/status only);
- pass exchange completed;
- card played;
- hearts broken;
- trick captured and next leader;
- hand raw score recorded;
- moon resolved;
- cumulative scores updated;
- threshold reached but low tie continues;
- match completed with per-seat standings.

Private effects may include:

- cards dealt to owner;
- own pass selection changed;
- own pass commitment receipt;
- own sent cards and own received cards after exchange;
- own legal preview details.

Effect filters must be tested independently from view filters. An effect hidden from rendering but present in JSON is a leak.

## B.5 Outcome model

For every seat, Rust supplies:

- raw captured hearts count;
- whether Q♠ was captured;
- raw hand points;
- moon status (`shooter`, `opponent_adjusted`, or `none`);
- adjusted hand addition;
- cumulative score before and after;
- current rank/standing;
- threshold reached flag;
- terminal status or tied-low continuation reason.

At terminal, supply exactly one winner seat and three loser seats. A tied-low threshold hand is non-terminal, not a draw. Display sorting may use seat order for stability, but seat order never breaks a score tie.

# Appendix C — Bot policy contract

## C.1 L0 random legal

L0 is required and simple:

- obtain the current authorized Rust legal leaf set;
- sample uniformly using declared bot RNG;
- submit the normal command through validation;
- explanation: “Random legal choice from N legal actions,” plus public phase and bot seed metadata safe for the viewer;
- never inspect raw state, opponent hands, pass map, deck order, or future cards.

## C.2 Bounded L1 rule-informed policy

L1 exists to avoid presenting a serious four-seat game with only arbitrary opponents. It is deterministic after declared tie-break RNG and remains intentionally below “competent player” admission.

Allowed input fields:

- own seat-private projected hand and own staged pass selection;
- public current trick, led suit, broken status, trick/hand index, dealer/pass direction, captured public point totals, cumulative scores, active/pending seats;
- legal action leaves supplied by Rust;
- public command/effect history already authorized to that seat.

Suggested priorities, all implemented locally and documented exactly. The broad risk-avoidance ideas are consistent with the non-search strategy guidance summarized by Pagat, but Rulepath's ordering is its own bounded authored policy rather than copied strategy prose.[^E1]:

- **Pass:** prefer shedding Q♠ when held; then dangerous high spades; then high hearts; then high cards from short suits to create a void; deterministic card-ID tie-break.
- **Void discard:** prefer Q♠, then highest heart, then highest remaining legal card.
- **Follow suit:** when possible, play the highest legal card that remains below the current winning led-suit card; otherwise play the lowest legal card, with deterministic tie-break.
- **Lead:** prefer a low card from a short non-heart suit while hearts are unbroken; when hearts are legal, prefer low-risk legal cards under the documented ordering.
- **Moon posture:** no hidden-state moon planning, belief model, or search. A minimal public score-aware guard may prioritize dumping a point card on a publicly apparent single collector only if the exact heuristic is documented and benchmarked; otherwise mark it not applicable.

The L1 explanation names only the chosen public/own-hand feature and priority, for example “Discarded the queen of spades while void in clubs.” It must never say or imply what an opponent holds.

## C.3 L2 gate

`BOT-STRATEGY-EVIDENCE-PACK.md` must state `not admitted` at Gate 16. A future L2 requires:

- competent-player strategy taxonomy and source notes;
- exact authorized input schema;
- deterministic priority vector and tie-breaks;
- scenario corpus covering passing, queen management, void creation, safe losing, point dumping, moon attempt/defense, score-aware endgame;
- evidence against L0 and L1 across fixed seeds without hidden-state access;
- viewer-safe explanation audit;
- no forbidden search/sampling/learning method.

# Appendix D — UI and accessibility acceptance details

- The owner hand is a semantic list/group of cards with explicit accessible names such as “queen of spades, 13-point penalty card”; suit is never conveyed by color or symbol alone.
- Legal cards are native buttons or equivalent keyboard-operable controls. Illegal cards are absent/inert or receive only Rust-supplied viewer-safe disabled reasons in learning mode.
- Pass selection has selected-state semantics, a visible “0/3 … 3/3” count, and a Rust-authorized confirm button. No drag-only interaction.
- Pointer targets meet at least the WCAG 2.2 24-by-24 CSS-pixel minimum or its spacing exception; larger card targets are preferred.[^E4]
- Focus order follows the current viewer's hand, current trick, legal controls, score table, and replay controls without traversing hidden/disabled duplicate nodes.
- The four seat rails expose active/pending/dealer/leader facts supplied by Rust. TypeScript does not infer them from seat index.
- A hotseat handoff overlay obscures the prior seat's hand before switching viewer; it never stores or pre-renders the next viewer's hand behind CSS.
- Card play, trick collection, and moon feedback use semantic effects. `prefers-reduced-motion` suppresses nonessential movement while preserving immediate state and textual consequence.[^E5]
- Screen-reader announcements cover card played, trick winner, hearts broken, pass completed, hand score, moon, tied-threshold continuation, and terminal winner without announcing hidden identities.
- Observer mode contains no face-down card identity in alt text, ARIA, DOM keys, test IDs, or CSS.
- Replay scrub/navigation shows the selected viewer's authorized historical observation, not the final-state hand projected backward.
- Responsive layouts must keep all four seat identities, current trick, score state, and the owner's legal hand usable without horizontal precision traps; a compact layout may collapse history but not remove decisive status.

# Appendix E — Research and source notes

## E.1 Rules-source reconciliation

| Topic | Source landscape | Briar Circuit resolution |
| --- | --- | --- |
| Core deck, 2♣ opening, follow suit, led-suit winner, points, target | Broad agreement between Pagat and Bicycle | Adopted |
| Pass cycle | Pagat and Bicycle describe left/right/across/hold | Adopted exactly |
| First-trick point cards | Common modern rule; sources vary in whether it is baseline and how the impossible-hand exception is phrased | Strict restriction with “no non-point alternative” exception |
| What breaks hearts | Pagat's common rule uses a heart; Bicycle wording can be read to include Q♠ in some variants | Only a heart breaks hearts; Q♠ does not |
| Moon | Common alternatives include adding 26 to opponents or subtracting 26 from shooter | Fixed add 26 to each opponent, no choice |
| Match tie | Rules commonly target 100; Pagat describes additional hands for a tied low score | Continue complete hands until unique low |
| Shoot the sun | House/extended variant | Excluded |

The game-local `SOURCES.md` must paraphrase these facts in original prose and explicitly label deliberate choices. It must not reproduce a source's rules text.

## E.2 External implementation prior art

OpenSpiel's Hearts implementation is useful only as external prior art for explicit deal/pass/play phases, delayed pass exchange, follow-suit legal-action filtering, trick winner state, point transformation, and player-specific observations.[^E3] Briar Circuit does not import OpenSpiel code, public API, single-hand utility model, bot/search approach, or architecture. Rulepath retains cumulative 100-point match play, its own action/effect/view/replay contracts, and Rust/WASM browser authority.

External Hearts projects using determinization, PIMC, MCTS, POMCP, Monte Carlo, ML, or RL are negative examples for this gate. Their existence does not authorize those approaches under Rulepath's public bot law.

## E.3 Source list

[^E1]: Pagat, “Hearts — Card Game Rules,” <https://www.pagat.com/reverse/hearts.html>, consulted 2026-06-20. Used for four-player deck/deal, pass cycle, 2♣ opening, follow suit, trick winner, hearts-only breaking convention, points, 100 target, moon variants, and tied-low continuation.
[^E2]: Bicycle Cards, “Hearts,” <https://bicyclecards.com/how-to-play/hearts/>, consulted 2026-06-20. Used as a second classic-rules reference for 52 cards, first-trick point restriction, point values, 100 target, and fixed add-26 moon convention; divergences are resolved explicitly above.
[^E3]: Google DeepMind OpenSpiel, `open_spiel/games/hearts/hearts.cc`, <https://github.com/google-deepmind/open_spiel/blob/master/open_spiel/games/hearts/hearts.cc>, consulted 2026-06-20. External implementation prior art only; no target-repository claim and no code import.
[^E4]: W3C WAI, “Understanding Success Criterion 2.5.8: Target Size (Minimum),” <https://www.w3.org/WAI/WCAG22/Understanding/target-size-minimum.html>, consulted 2026-06-20.
[^E5]: W3C WAI, “Technique SCR40: Using the CSS `prefers-reduced-motion` query in JavaScript to prevent motion,” <https://www.w3.org/WAI/WCAG22/Techniques/client-side-script/SCR40>, consulted 2026-06-20.
[^E6]: W3C WAI, “ARIA Authoring Practices Guide,” <https://www.w3.org/WAI/ARIA/apg/>, consulted 2026-06-20. Used for semantic widget, accessible-name, landmark, and keyboard-interface guidance.

## E.4 Repository authority references

The references below are repo-relative paths into this same repository. Consult each at the implementation target when reassessing or decomposing this spec.

[^R1]: `docs/ROADMAP.md` (repo-relative).
[^R2]: `specs/README.md` (repo-relative).
[^R3]: `docs/MECHANIC-ATLAS.md` (repo-relative).
[^R4]: `docs/FOUNDATIONS.md` (repo-relative).
[^R5]: `docs/OFFICIAL-GAME-CONTRACT.md` (repo-relative).
[^R6]: `docs/MULTI-SEAT-AND-SURFACE-CONTRACT.md` (repo-relative).
[^R7]: `docs/TESTING-REPLAY-BENCHMARKING.md` (repo-relative).
[^R8]: `docs/AI-BOTS.md` (repo-relative).
[^R9]: `docs/UI-INTERACTION.md` (repo-relative).
[^R10]: `docs/ENGINE-GAME-DATA-BOUNDARY.md` (repo-relative).
[^R11]: `docs/TRACE-SCHEMA-v1.md` (repo-relative).
[^R12]: `docs/adr/0004-hidden-info-replay-export-taxonomy.md` (repo-relative).
[^R13]: `archive/specs/gate-10-1-plain-tricks-trick-taking-proof.md` (repo-relative).
[^R14]: `games/plain_tricks/docs/MECHANICS.md` (repo-relative).
[^R15]: `archive/specs/gate-15-river-ledger-texas-holdem-base.md` (repo-relative).
[^R16]: `games/river_ledger/docs/PRIMITIVE-PRESSURE-LEDGER.md` (repo-relative).
[^R17]: `docs/IP-POLICY.md` (repo-relative).
[^R18]: `docs/AGENT-DISCIPLINE.md` (repo-relative).
[^R19]: `docs/WASM-CLIENT-BOUNDARY.md` (repo-relative).
[^R20]: `docs/README.md` (repo-relative).

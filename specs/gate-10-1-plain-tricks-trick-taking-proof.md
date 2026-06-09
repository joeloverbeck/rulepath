# Gate 10.1 implementation spec — `plain_tricks` trick / follow-suit proof

## 1. Header

| Field | Value |
| --- | --- |
| Spec ID | `GATE101-PLNTRICKS-TRICKFOLLOW-001` |
| Roadmap stage | Stage 10 |
| Build gate | Gate 10.1 (proposed sub-gate of ROADMAP Gate 10, mirroring the accepted Gate 9 / Gate 9.1 precedent; `docs/ROADMAP.md` keeps both candidates under a single Gate 10) |
| Status | Planned |
| Date | 2026-06-09 |
| Owner | Rulepath maintainers / implementation agents |
| Primary crate | `games/plain_tricks` |
| Internal game id | `plain_tricks` |
| Chosen public display name | **Plain Tricks** |
| Standard variant id | `plain_tricks_standard` |
| Trace rules version | `plain-tricks-rules-v1` |
| Browser implementation required | Yes — Rust/WASM-backed shell renderer and e2e smoke are in scope |
| Authority order | `docs/README.md` → `docs/FOUNDATIONS.md` → `docs/ARCHITECTURE.md` → `docs/ENGINE-GAME-DATA-BOUNDARY.md` → area docs / accepted ADRs → `docs/ROADMAP.md` → this spec → later AGENT-TASKs/tickets |
| Public presentation posture | Original, neutral, board-game-table framing; no commercial card-brand trade dress, no copied rules prose, no claim to be a Whist/Hearts product |
| Kernel stance | No new kernel concept; cards, suits, hands, tricks, lead/follow legality, trick resolution, round scoring, and deal rotation are game-local `plain_tricks` behavior expressed through existing action-tree, command-envelope, semantic-effect, visibility, replay, and WASM envelopes |

## 2. Objective

Gate 10.1 implements `plain_tricks` as the closing half of ROADMAP Gate 10: a
small, deterministic, two-seat, browser-playable hidden-hand trick-taking
microgame proving lead/follow constraints, must-follow-suit legality, trick
resolution, round scoring, deal rotation, viewer-safe hidden-hand behavior, and
a Level 2 authored-policy bot that does not cheat.

The locked public display name is **Plain Tricks**. The internal id remains
`plain_tricks` because the roadmap uses that id; the display name matches the
`token_bazaar` → **Token Bazaar** precedent of presenting the neutral id
directly. Public copy presents an original Rulepath microgame and must not
brand itself as Whist, Hearts, or any commercial card product.

The next-gate determination is grounded in the progress index, not reopened
here:

- `specs/README.md` records Gates 0 through 9.1 as `Done` and Gate 10 as
  `In progress`: the `poker_lite` / Crest Ledger betting-showdown half is
  complete, and the `plain_tricks` trick/follow-suit half is the only
  not-yet-specced unit at the lowest open stage.
- The archived Gate 10 spec
  ([`gate-10-poker-lite-betting-showdown.md`](../archive/specs/gate-10-poker-lite-betting-showdown.md)
  §11) names `plain_tricks` as its successor and designates it Gate 10.1, a
  proposed sub-gate split of ROADMAP's single Gate 10.
- `docs/MECHANIC-ATLAS.md` §10A is empty — no promotion-debt interlock blocks
  a new mechanic-ladder gate.
- `progress.md` records the Gate 10 trick/follow-suit half as deferred scope
  owned by `plain_tricks`.

**Third-use hard gate.** This spec triggers the FOUNDATIONS §4 third-use rule.
`docs/MECHANIC-ATLAS.md` §10B records the deterministic shuffle / private hand
/ staged reveal shape with `high_card_duel` (first use) and `poker_lite`
(second use), and names `plain_tricks` as the likely third use: *"Third
card/private-hand use, likely `plain_tricks` if it repeats the shape, must
record a ledger decision before extraction."* `plain_tricks` does repeat the
shape (seeded shuffle of an opaque-id deck, per-seat private hand, viewer-
filtered deal effects, hidden tail). Therefore the game MUST NOT proceed past
crate skeleton work until a primitive-pressure ledger entry decides exactly one
of: reuse existing promoted primitive / promote narrow typed helper / explicit
defer-reject with rationale / ADR escalation. Work item 2 in §5 is that
decision, and §8 carries the corresponding §12 stop condition.

The resulting gate proves Rulepath can support:

1. multi-card private hands with deterministic deal and a never-revealed tail;
2. state-dependent legality where the legal set depends on hidden private
   state (must-follow-suit) without leaking that state to non-actors;
3. trick resolution, trick-winner-leads turn order, and round scoring without
   engine-core nouns;
4. multi-round play with deterministic deal rotation under one seeded RNG
   stream;
5. a competent, fair, beatable Level 2 bot using only its own hand plus public
   trick history;
6. web presentation, viewer-scoped replay export/import, and e2e no-leak
   behavior for a classic public-domain card-game shape under neutral IP rules.

Completing this spec closes the remaining trick/follow-suit rows of ROADMAP
§12's Gate 10 exit list; with the `poker_lite` half already complete, Gate 10
as a whole becomes eligible for `Done` when this spec's exit criteria pass.

## 3. Scope

### In scope

- New Rust crate `games/plain_tricks` with the same practical layout as
  `poker_lite`: `src/*`, `data/*`, `tests/*`, `benches/*`, and `docs/*`.
- Concrete original rules for **Plain Tricks**, fully specified before
  implementation (appendix A):
  - two seats, `seat_0` and `seat_1`;
  - an original 18-card deck: three neutral suits, ranks 1–6, one copy each;
  - per round: deterministic shuffle, 6 cards dealt to each seat, 6-card tail
    that is never used and never revealed;
  - 6 tricks per round; leader plays any card, follower must follow the led
    suit if able, otherwise may play any card;
  - highest rank of the led suit wins the trick; off-suit cards never win;
  - trick winner leads the next trick;
  - round scoring: 1 point per trick won;
  - two rounds with deal rotation: `seat_0` leads trick 1 of round 1,
    `seat_1` leads trick 1 of round 2, each round freshly shuffled from the
    same seeded RNG stream;
  - terminal: most total points across both rounds wins; exact tie is a
    `Split` outcome;
  - fixed maximum action count by construction: exactly 24 card plays.
- The third-use primitive-pressure ledger decision for the deterministic
  shuffle / private hand / staged reveal shape (§5 item 2), including the
  conditional back-port/debt work it may create.
- Full Rust-owned legality, validation, transitions, effects, view projection,
  replay serialization, public export/import, bot decisions, and diagnostics.
- Level 0 random-legal bot plus Level 2 authored-policy bot with
  `COMPETENT-PLAYER.md` and `BOT-STRATEGY-EVIDENCE-PACK.md` authored before
  the Level 2 bot is coded.
- Native unit/rule/property/replay/serialization/visibility-no-leak/bot tests,
  golden traces, simulation support, benchmark support, fixture checking, rule
  coverage, replay checking.
- WASM registration and browser shell support: a new `PlainTricksBoard.tsx`
  renderer, effect-feedback copy, action controls, typed client view/effect
  definitions, replay import/export, no-leak dev-panel behavior, and e2e
  smoke.
- In-gate documentation/catalog reconciliation: `specs/README.md`,
  `docs/MECHANIC-ATLAS.md`, per-game docs, root `README.md`, `progress.md`,
  and `apps/web/README.md` after implementation evidence lands.
- Source notes for classic trick-taking research (e.g. the Pagat trick-taking
  overview already cited in `docs/SOURCES.md`), routed to
  `games/plain_tricks/docs/SOURCES.md` and cross-linked from `docs/SOURCES.md`
  if repository practice keeps the central bibliography current.

### Out of scope

- A general trick-taking engine or card-game framework.
- Trump suits, bidding, contracts, partnerships, 3+ seats, point-cards with
  unequal values, penalty cards, shooting-the-moon mechanics, passing phases,
  exposed-card (dummy) play, configurable deck sizes, or variant families.
- New engine-core types or terminology for card, deck, hand, suit, rank,
  trick, lead, follow, void, discard, or deal.
- `game-stdlib` promotion of card/deck/hand/shuffle/trick helpers **except**
  as the §5 item 2 ledger decision explicitly authorizes; an unauthorized or
  silent promotion is a stop condition.
- TypeScript legality, TypeScript follow-suit checks, TypeScript trick-winner
  or scoring logic, TypeScript hidden-hand timing, or TypeScript bot decision
  logic.
- Any MCTS, ISMCTS, Monte Carlo simulation, opponent-hand enumeration or
  sampling, ML, RL, or hidden-state inference beyond public play history.
- Any copied rules prose, scanned card faces, commercial card-brand suit/court
  imagery, or "Whist"/"Hearts" branding.
- A separate aftermath spec. Web catalog closeout is folded into this gate.

### Not allowed

The `docs/ROADMAP.md` §12 prohibitions are binding and repeated as gate law:

- no real-money or casino features;
- no unbounded variants;
- no hidden-state cheating;
- no ML/RL;
- no copied rules prose.

Gate-local additions:

- no public MCTS/ISMCTS/Monte Carlo bot, even as an "analysis-only" helper;
- no proceeding past crate-skeleton work before the §5 item 2 third-use ledger
  decision is recorded (FOUNDATIONS §12: "a third repeated mechanic proceeds
  without ledger decision" is a stop condition);
- no hidden-hand or deck-tail identifiers in public/opponent views,
  action-tree metadata delivered to non-actors, diagnostics, effect logs, test
  IDs, storage, dev panel dumps, replay exports, bot explanations, or
  candidate rankings — a card identity becomes public only when that card is
  played;
- no reveal of the undealt tail at any point, including terminal;
- no accidental trace/hash migration of existing games, including any
  migration a "promote" ledger decision would cause, unless explicitly
  designed, documented, and covered by tests.

## 4. Deliverables

### New crate and source tree

```text
games/plain_tricks/Cargo.toml
games/plain_tricks/src/actions.rs
games/plain_tricks/src/bots.rs
games/plain_tricks/src/effects.rs
games/plain_tricks/src/ids.rs
games/plain_tricks/src/lib.rs
games/plain_tricks/src/replay_support.rs
games/plain_tricks/src/rules.rs
games/plain_tricks/src/setup.rs
games/plain_tricks/src/state.rs
games/plain_tricks/src/ui.rs
games/plain_tricks/src/variants.rs
games/plain_tricks/src/visibility.rs
```

The crate mirrors the established game shape:

- `ids.rs`: `PlainTricksSeat`, `TrickCardId`, `TrickSuit`, `TrickRank`, action
  segment constants, variant id constants, stable label helpers.
- `state.rs`: internal phase, seats, per-round shuffled deck, private hands,
  hidden tail, current trick (led card/suit, plays), trick counts, round
  index, round leader, completed-trick history, terminal outcome, effect
  history, freshness token. Hidden hands and tail are internal only.
- `setup.rs`: deterministic 18-card deck construction, per-round seeded
  shuffle and deal using the existing seeded-RNG discipline, deal rotation; no
  new kernel RNG concept. Subject to the §5 item 2 ledger decision, the
  shuffle either stays local or uses the helper that decision authorizes.
- `actions.rs`: legal action tree (one `play` family; one leaf per legal card
  in the actor's hand under the follow-suit rule), safe action metadata, path
  parsing, command validation, stale/wrong-seat/terminal/malformed/
  must-follow-suit diagnostics.
- `rules.rs`: transition engine for card play, trick resolution, trick-winner
  leads, round close, deal rotation, round scoring, terminal outcome,
  freshness increments.
- `effects.rs`: typed semantic effects with public/private visibility
  envelopes; private deal effects, public play/trick/score effects.
- `visibility.rs`: `PublicView`, `SeatPrivateView`, observer and seat
  projection, stable summaries, no-leak helpers.
- `bots.rs`: Level 0 random-legal and Level 2 authored-policy bot using only
  its own hand, the legal tree, and public trick/score history.
- `replay_support.rs`: golden trace command replay, internal full trace for
  tests, public observer/seat export and import per ADR 0004.
- `ui.rs`: neutral display labels, rules summaries, accessibility copy.
- `variants.rs`: strict typed parsing of data manifests; reject
  behavior-looking keys.
- `lib.rs`: public crate surface matching established games.

### Data, fixtures, benchmarks

```text
games/plain_tricks/data/manifest.toml
games/plain_tricks/data/variants.toml
games/plain_tricks/data/fixtures/plain_tricks_standard.fixture.json
games/plain_tricks/benches/plain_tricks.rs
games/plain_tricks/benches/thresholds.json
```

Data files contain typed content, labels, variant metadata, fixtures, and
version declarations only. They must not contain selectors, conditions,
formulas, scripts, YAML, DSL fragments, follow-suit expressions, trick-winner
formulas, scoring formulas, bot policy rules, or deal-routing logic. The
standard variant constants are Rust-owned typed behavior documented in
`RULES.md`; data may mirror display metadata but does not own behavior.

### Tests and golden traces

```text
games/plain_tricks/tests/rules.rs
games/plain_tricks/tests/property.rs
games/plain_tricks/tests/replay.rs
games/plain_tricks/tests/serialization.rs
games/plain_tricks/tests/visibility.rs
games/plain_tricks/tests/bots.rs
games/plain_tricks/tests/golden_traces/deal-private-no-leak.trace.json
games/plain_tricks/tests/golden_traces/follow-suit-forced.trace.json
games/plain_tricks/tests/golden_traces/void-free-discard.trace.json
games/plain_tricks/tests/golden_traces/off-suit-never-wins.trace.json
games/plain_tricks/tests/golden_traces/trick-winner-leads-next.trace.json
games/plain_tricks/tests/golden_traces/round-close-deal-rotation.trace.json
games/plain_tricks/tests/golden_traces/terminal-most-points-win.trace.json
games/plain_tricks/tests/golden_traces/tie-split.trace.json
games/plain_tricks/tests/golden_traces/no-leak-public-observer.trace.json
games/plain_tricks/tests/golden_traces/seat-private-view.trace.json
games/plain_tricks/tests/golden_traces/invalid-wrong-seat-diagnostic.trace.json
games/plain_tricks/tests/golden_traces/invalid-stale-diagnostic.trace.json
games/plain_tricks/tests/golden_traces/invalid-must-follow-diagnostic.trace.json
games/plain_tricks/tests/golden_traces/bot-action.trace.json
games/plain_tricks/tests/golden_traces/public-replay-export-import.trace.json
games/plain_tricks/tests/golden_traces/wasm-exported.trace.json
```

Golden traces should be added only after Rust rules are implemented enough to
generate stable evidence. Trace names may be adjusted, but coverage categories
may not be deleted or weakened.

### Per-game documentation

Instantiate the official documentation set from templates:

```text
games/plain_tricks/docs/RULES.md
games/plain_tricks/docs/HOW-TO-PLAY.md
games/plain_tricks/docs/SOURCES.md
games/plain_tricks/docs/RULE-COVERAGE.md
games/plain_tricks/docs/MECHANICS.md
games/plain_tricks/docs/AI.md
games/plain_tricks/docs/BOT-STRATEGY-EVIDENCE-PACK.md
games/plain_tricks/docs/COMPETENT-PLAYER.md
games/plain_tricks/docs/UI.md
games/plain_tricks/docs/BENCHMARKS.md
games/plain_tricks/docs/GAME-IMPLEMENTATION-ADMISSION.md
games/plain_tricks/docs/PUBLIC-RELEASE-CHECKLIST.md
games/plain_tricks/docs/PRIMITIVE-PRESSURE-LEDGER.md
```

`COMPETENT-PLAYER.md` and `BOT-STRATEGY-EVIDENCE-PACK.md` are mandatory
because this gate requires a Level 2 authored-policy bot, and per
`templates/README.md` they precede Level 2 bot coding.
`PRIMITIVE-PRESSURE-LEDGER.md` is mandatory and load-bearing for this gate: it
records the third-use hard-gate decision (§5 item 2) and must cross-reference,
not duplicate, the repo-level `docs/MECHANIC-ATLAS.md` entries and the
`games/poker_lite/docs/PRIMITIVE-PRESSURE-LEDGER.md` second-use comparison.
`RULES.md` must carry stable rule IDs for lead/follow legality, trick
resolution, round scoring, deal rotation, terminal/tie rules, and the
visibility rules, so rule tests, `RULE-COVERAGE.md`, and the outcome
explanation surface can cite them. `UI.md` must include the
"Outcome / victory explanation" section required by
`docs/OFFICIAL-GAME-CONTRACT.md` §5/§10.

### WASM, tools, CI, and web shell

- `crates/wasm-api/src/lib.rs`: register `plain_tricks`, display name
  **Plain Tricks**, `plain_tricks_standard` variant, hidden-information tags
  (`hidden_info`, `viewer_filtered`, `public_replay_export`, `trick_taking`),
  viewer modes, match record, new-match setup, get-view, get-action-tree (with
  the existing actor-only authorization: non-actor viewers get an empty tree),
  apply-action, run-bot-turn, effects, export/import, replay-step/reset, JSON
  serializers, redaction tests.
- `Cargo.toml`: add the workspace member and dependencies consistently with
  other games.
- Tool registration (verified against current registries): `simulate`,
  `replay-check`, `fixture-check`, and `rule-coverage` register all games and
  require a `plain_tricks` arm. `bench-report` registers games with threshold
  files; add `plain_tricks` there for the gate-2 benchmark lane.
  `seed-reducer` and `trace-viewer` register only `race_to_n` +
  `directional_flip`; `plain_tricks` is **not expected** to need an entry in
  either, matching the `poker_lite` precedent. Add only if a concrete need
  arises.
- Native benchmark lane: `games/plain_tricks/benches/plain_tricks.rs`,
  thresholds per ADR 0002/0003 discipline, and `gate-2-benchmarks.yml`
  registration (non-gating PR smoke; gating scheduled/manual/main lane).
- Native smoke lane: `gate-1-game-smoke.yml` simulate / replay-check /
  fixture-check / rule-coverage steps for `plain_tricks`.
- Web shell: add `apps/web/src/components/PlainTricksBoard.tsx`; add the
  `PlainTricksPublicView` type to `apps/web/src/wasm/client.ts` (and into its
  `PublicView` union); wire renderer selection in `apps/web/src/main.tsx` (a
  `PlainTricksBoard` import, an `isPlainTricksView()` guard, a render clause,
  and `ActionControls` handling alongside the existing hidden-info games); add
  neutral effect feedback in `effectFeedback.ts` and no-leak-safe action test
  IDs in `ActionControls.tsx`.
- E2E: add `apps/web/e2e/plain-tricks.smoke.mjs` and append it to the
  hardcoded `smoke:e2e` `&&`-chain in `apps/web/package.json`; update existing
  no-leak/a11y smoke coverage where the harness expects hidden-info games.
- Catalog docs/scripts: update `apps/web/README.md` (intro catalog, Shell
  Surface renderer list, Smoke Layers list) and satisfy
  `scripts/check-catalog-docs.mjs` in-gate.

## 5. Work breakdown

Do not create tickets in this spec. The following are bounded candidate
AGENT-TASKs in dependency order.

1. **Admission and source packet.** Add `games/plain_tricks/docs/SOURCES.md`,
   `GAME-IMPLEMENTATION-ADMISSION.md`, and the initial `RULES.md` skeleton
   from this spec. Record classic trick-taking research (Pagat overview and
   any public-domain Whist-family references consulted), the chosen variant,
   deliberate simplifications (no trump, two seats, 18-card deck), neutral
   naming, and IP constraints. Confirm `docs/MECHANIC-ATLAS.md` §10A is still
   empty before coding.
2. **Third-use primitive-pressure ledger decision (hard gate).** Before any
   shuffle/hand/deal code is written, author
   `games/plain_tricks/docs/PRIMITIVE-PRESSURE-LEDGER.md` and update
   `docs/MECHANIC-ATLAS.md`: compare `high_card_duel`, `poker_lite`, and the
   planned `plain_tricks` deterministic shuffle / private hand / viewer-
   filtered deal shape using the §6 ledger fields, and decide exactly one of
   reuse / promote narrow typed helper / explicit defer-reject / ADR. The
   spec's provisional analysis (appendix A6) leans defer-reject or, at most, a
   narrow behavior-free seeded-shuffle helper; the ledger decision at
   implementation time is authoritative. If the decision is **promote**, the
   same gate must either back-port `high_card_duel` and `poker_lite` to the
   helper with trace/hash preservation evidence, or record named promotion
   debt with a closure gate in atlas §10A — and the back-port/debt work
   becomes additional bounded AGENT-TASKs inside this gate. The ledger entry
   must also record the explicit stance that `plain_tricks` round scoring is
   trick-count scoring, not the `token_bazaar`/`poker_lite` public
   resource-accounting shape (or, if the comparison contradicts that stance,
   treat it as a third accounting use and run the same four-option decision).
3. **Crate skeleton and typed ids.** Add `games/plain_tricks` to the workspace
   with ids, variants, display labels, strict data parsing, fixture manifest,
   and compile-only exports. No engine-core or game-stdlib change.
4. **Deterministic setup and internal state.** Implement 18-card deck
   construction, per-round seeded shuffle (honoring the item 2 decision), 6+6
   deal with hidden 6-card tail, deal rotation, round/trick state, freshness
   token, and stable internal serialization helpers.
5. **Legal action tree and validation.** Implement the Rust-owned `play`
   action family: one leaf per hand card legal under must-follow-suit.
   Metadata may expose public trick state only (led suit/card, trick index,
   round index); the actor's own tree necessarily names the actor's own
   playable cards, and non-actor viewers receive an empty tree. Add stale,
   wrong-seat, terminal, malformed-path, not-in-hand, and must-follow-suit
   diagnostics; diagnostics must not name unplayed cards other than echoing
   the actor's own attempted card.
6. **Rules and effects.** Implement card-play transitions, trick resolution
   (highest led-suit rank wins; off-suit never wins), trick-winner-leads turn
   order, round close and scoring, deal rotation into round 2, terminal
   outcome (win/Split), and viewer-scoped semantic effects.
7. **Visibility and no-leak tests.** Implement observer/seat views: own hand
   visible to owner only; opponent hand as count only; tail never visible;
   played cards public from the moment of play; trick history and scores
   public. Document and test that off-suit play publicly revealing a void is
   rule-implied public information. Add exhaustive string-search tests for
   hidden card ids/suit/rank labels across views, action trees delivered to
   non-actors, effects, diagnostics, replay exports, and browser-facing
   surfaces.
8. **Replay and golden traces.** Add internal trace replay, public replay
   export/import per ADR 0004 taxonomy, deterministic hash checkpoints, and
   the golden trace set. Public exports must not include seed material capable
   of reconstructing hands or tail; at terminal, played cards are public but
   the tail remains unreconstructable.
9. **Bots.** Author `COMPETENT-PLAYER.md`, then
   `BOT-STRATEGY-EVIDENCE-PACK.md`, then the Level 0 random-legal bot and
   Level 2 authored-policy bot. The Level 2 bot consumes only its own hand,
   its legal tree, and public trick/score/void history; no opponent-hand or
   tail enumeration or sampling. Complete `AI.md`.
10. **Native tools and benchmarks.** Register `plain_tricks` in `simulate`,
    `replay-check`, `fixture-check`, `rule-coverage`, and `bench-report`. Add
    benchmarks and provisional thresholds under ADR 0002/0003 lanes with a
    named calibration follow-up. If ADR 0005 has been accepted by then, apply
    its variance-aware floor discipline; otherwise do not claim it as
    accepted.
11. **WASM registration.** Add WASM game constants, match record variant,
    serializers, viewer-safe action tree access, bot turn, effects,
    export/import, replay stepping, and no-leak unit tests.
12. **Browser renderer and smoke.** Add `PlainTricksBoard.tsx`: own hand in
    seat view, opponent hand as face-down count, current trick surface,
    trick/score ledger, legal-only play controls from the Rust tree,
    deal/trick/score effect feedback, outcome explanation surface, viewer
    modes, dev-panel whitelist behavior, replay import/export controls,
    reduced-motion behavior, and `plain-tricks.smoke.mjs`.
13. **Official-doc closeout.** Complete per-game docs, rule coverage matrix,
    mechanics inventory, public release checklist, and benchmark docs. Update
    `specs/README.md` (flip Gate 10.1 per §10 and assess the Gate 10 parent
    row), `docs/MECHANIC-ATLAS.md`, `progress.md`, root `README.md`, and
    `apps/web/README.md`; run doc-link and catalog checks. Do not edit
    `docs/ROADMAP.md` merely to record progress.

## 6. Exit criteria

ROADMAP §12 Gate 10 exit rows, with the betting rows already closed by the
`poker_lite` half (see the archived Gate 10 spec §6):

| ROADMAP §12 Gate 10 exit row | Gate 10.1 `plain_tricks` acceptance mapping |
| --- | --- |
| Betting/trick rules correct for chosen variants | Betting was proven by `poker_lite`. The chosen trick variant is **Plain Tricks** / `plain_tricks_standard`. Rules tests and golden traces prove deal, must-follow-suit legality, void free-play, trick resolution (highest led-suit rank; off-suit never wins), trick-winner-leads, round close, deal rotation, terminal win, and Split tie. |
| Pot/accounting and follow-suit edge cases covered | Pot/accounting was proven by `poker_lite`. Follow-suit edge cases covered here: forced follow with exactly one legal card; forced follow with several; void in led suit (all cards legal); off-suit play cannot win; leader unconstrained; last-trick exhaustion; rotation reset of trick state; tie at 6–6 splits. |
| Bots finish games without hidden-state cheating | Level 0 and Level 2 bots run simulations to terminal under the action cap. Level 2 consumes only its own hand, legal tree, and public history; tests assert no opponent card, tail card, or hidden-state-derived ranking enters bot input or explanations. |
| No public MCTS/ISMCTS | `plain_tricks` exposes no MCTS, ISMCTS, Monte Carlo dealing/sampling, ML/RL, or opponent-hand enumeration. The Level 2 policy is authored, deterministic, explainable, and beatable. |
| UI remains understandable | The browser presents **Plain Tricks** with neutral language, legal-only Rust-supplied controls, a clear current-trick surface, public trick/score ledger, observer/seat viewer modes, outcome explanation, reduced-motion behavior, and no commercial card trade dress. Human+bot, hotseat, bot-vs-bot, replay, and no-leak e2e smokes pass. |
| Native benchmarks exist | `games/plain_tricks/benches/plain_tricks.rs` and `thresholds.json` exist; CI lanes run per ADR 0002/0003. Provisional native floor target: at least 2,000 completed matches/sec for random-legal playout unless calibration evidence sets a different accepted floor. |

Third-use hard-gate exit criteria:

- A completed primitive-pressure ledger entry for the card/private-hand shape
  exists in `games/plain_tricks/docs/PRIMITIVE-PRESSURE-LEDGER.md` and
  `docs/MECHANIC-ATLAS.md` (§10/§10B updated) with exactly one recorded
  decision, made before rules implementation.
- If the decision was promote: the helper has tests, docs, examples,
  anti-examples, and benchmarks; `high_card_duel` and `poker_lite` are
  migrated with preserved traces/hashes, or atlas §10A records named promotion
  debt with a closure gate.
- If the decision was defer-reject or reuse: atlas §10B reflects it with
  rationale and next review trigger.
- The accounting-shape stance (trick scoring vs. resource accounting) is
  recorded.

Universal hidden-info no-leak exit criteria:

- At no point does an observer/opponent/browser payload contain an unplayed
  card's id, suit, rank, or label from either hand or the tail; the owning
  seat's private view contains only that seat's own hand.
- The tail is never revealed, including at terminal and in seat-scoped
  exports.
- A card identity becomes public exactly when it is played, through a public
  effect and the public view, and not earlier via metadata, previews,
  diagnostics, or test IDs.
- Non-actor viewers receive an empty action tree; the actor's tree is
  delivered only to the actor's viewer.
- Hidden identifiers are absent from action-tree metadata delivered to
  non-actors, previews, diagnostics, effect logs, DOM text, `data-testid`,
  local storage, dev panel data, replay export, bot explanations, and
  candidate rankings.

## 7. Acceptance evidence

### Required command/evidence set

Implementation closeout must record the exact command outputs in the gate PR
or closeout note:

```text
cargo fmt --all --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
cargo test -p plain_tricks
cargo test -p plain_tricks --test rules
cargo test -p plain_tricks --test property
cargo test -p plain_tricks --test replay
cargo test -p plain_tricks --test serialization
cargo test -p plain_tricks --test visibility
cargo test -p plain_tricks --test bots
cargo run -p simulate -- --game plain_tricks --games 1000 --start-seed 0 --action-cap 32
cargo run -p replay-check -- --game plain_tricks
cargo run -p fixture-check -- --game plain_tricks
cargo run -p rule-coverage -- --game plain_tricks
cargo bench -p plain_tricks
bash scripts/boundary-check.sh
node scripts/check-doc-links.mjs
node scripts/check-catalog-docs.mjs
npm --prefix apps/web run smoke:wasm
npm --prefix apps/web run smoke:ui
npm --prefix apps/web run smoke:e2e
```

`smoke:e2e` in `apps/web/package.json` is a hardcoded `node e2e/*.smoke.mjs`
`&&`-chain; `plain-tricks.smoke.mjs` must be appended to that chain.

The `--action-cap 32` value is deliberate headroom, not a value to "correct"
down to the 24-play maximum: `simulate` checks for the terminal outcome at the
top of the playout iteration *after* the final play is applied
(`tools/simulate/src/main.rs`), so a 24-play match requires `--action-cap` of at
least 25; a cap of exactly 24 exhausts the loop one iteration early and reports
a false `SIMULATION FAILURE: action cap reached before terminal outcome`. The
gate-1 native-smoke step (§4) must likewise use a cap above 24, not 24 (the
`poker_lite` precedent sets its cap to its own maximum-plus-one, `16`).

The final commands must match the actual tool CLIs at implementation time; if a
tool interface differs, update this evidence section in the implementation
closeout without weakening coverage.

### Test taxonomy

- **Unit/rule tests:** deck construction; deal and rotation; legal action
  generation under follow-suit; path parsing; validation; trick resolution
  including off-suit-never-wins; trick-winner-leads; round close and scoring;
  terminal and Split; malformed/stale/wrong-seat/must-follow diagnostics. Rule
  tests cite `RULES.md` rule IDs.
- **Property tests:** deterministic replay from seed and command stream; the
  match always terminates in exactly 24 plays; the legal tree never offers an
  off-suit card while the actor holds the led suit; the legal tree always
  offers the full hand when leading or void; trick winner is always one of the
  two played cards' seats; total points always equal 12 at terminal; no hidden
  id appears in public-facing projections.
- **Golden traces:** named trace set from §4, including deal no-leak, forced
  follow, void discard, off-suit-never-wins, trick-winner-leads, rotation,
  terminal win, tie split, diagnostics, bot action, public export/import, and
  WASM-exported trace.
- **Replay tests:** internal full replay reproduces hashes; public replay
  export/import supports observer and seat viewers per ADR 0004; public
  export omits seed material; the tail is unreconstructable from any export.
- **Serialization tests:** stable field order, strict unknown-field rejection,
  stable IDs, no behavior-looking trace/data fields, no accidental schema
  migration.
- **Visibility-no-leak tests:** explicit string search for every unplayed card
  id and suit/rank label across view JSON, non-actor action-tree JSON, effect
  JSON, diagnostic JSON, replay export, web smoke DOM, dev panel, local
  storage, `data-testid`, bot explanation, and candidate ranking.
- **AI tests:** Level 0 uses legal actions only; Level 2 action is legal,
  deterministic under tie-breaks, uses only `PlainTricksBotInput` (own hand +
  public state), never receives opponent hand or tail, produces viewer-safe
  explanation effects, and completes simulations under cap.
- **Browser e2e:** human-vs-bot full match; forced-follow UI state; observer
  no-leak (no opponent hand identities); seat-private own-hand view; outcome
  explanation surface; public replay export/import; reduced motion;
  keyboard/a11y smoke; stale diagnostic smoke; dev panel whitelist smoke.
- **Benchmarks:** setup+shuffle+deal, legal tree generation per phase,
  validate/apply, trick resolution, observer/seat projection, public
  export/import, full random-legal playout, Level 2 playout. Thresholds
  follow ADR 0002 lanes and ADR 0003 CI-floor calibration, with a named
  calibration follow-up; apply ADR 0005 only if it is accepted by then.

### Official-game contract evidence

The gate is not done until the official-game deliverable set exists and is
internally consistent: original `RULES.md` with rule IDs; `HOW-TO-PLAY.md`
player prose wired to the shared How to Play surface; `SOURCES.md` with
research notes and IP posture; `RULE-COVERAGE.md` with no silent gaps;
`MECHANICS.md` and `PRIMITIVE-PRESSURE-LEDGER.md` with the third-use decision;
`AI.md`, `BOT-STRATEGY-EVIDENCE-PACK.md`, `COMPETENT-PLAYER.md`; `UI.md` with
the outcome-explanation section; `BENCHMARKS.md`;
`GAME-IMPLEMENTATION-ADMISSION.md`; `PUBLIC-RELEASE-CHECKLIST.md`; native
tests/traces/replay/serialization/visibility evidence; simulation and
benchmarks; WASM/browser registration and e2e smoke; docs/catalog checks
green.

## 8. FOUNDATIONS & boundary alignment

| Principle / source | Gate stance | Rationale |
| --- | --- | --- |
| §2 behavior authority | Rust owns deck/deal, follow-suit legality, validation, trick resolution, scoring, rotation, terminal detection, hidden-hand projection, replay/export, and bot decisions. | `plain_tricks` extends the existing Rust game-crate model; TypeScript renders only WASM-provided view/action/effect data. Follow-suit legality is exactly the kind of state-dependent rule TypeScript must never compute. |
| §3 noun-free `engine-core` | No kernel noun is added for card, deck, hand, suit, rank, trick, lead, follow, void, or deal. `trick` is explicitly on the FOUNDATIONS §3 forbidden-noun list. | Existing generic `ActionTree`, `CommandEnvelope`, `EffectEnvelope`, `VisibilityScope`, deterministic RNG, replay, and view contracts suffice. Typed nouns live in `games/plain_tricks`. |
| §4 earned `game-stdlib` — **third-use hard gate** | `plain_tricks` is the third official use of deterministic shuffle / private hand / viewer-filtered deal (`high_card_duel`, `poker_lite`, then this game). The game MUST NOT proceed past skeleton work until the §5 item 2 ledger decision is recorded. No silent promotion; a promote decision carries in-gate back-port or named debt. | This clears the §12 stop condition "a third repeated mechanic proceeds without ledger decision" by construction: the decision is a blocking early work item with its own exit criteria. |
| Accounting-shape stance | Trick-count round scoring is treated as scoring/outcome, not the `token_bazaar`/`poker_lite` public resource-accounting shape; the ledger entry must confirm or refute this explicitly. | Prevents a silent third accounting use; if the comparison contradicts the stance, the same four-option decision applies before proceeding. |
| §5 static data | Data contains labels, manifests, fixtures, and version metadata only. Follow-suit rules, trick comparator, scoring, rotation, and deal counts are typed Rust behavior. | No formulas, selectors, triggers, or behavior-like data. |
| §8 public bots | Level 0 random legal required; Level 2 authored-policy bot required, preceded by `COMPETENT-PLAYER.md` and `BOT-STRATEGY-EVIDENCE-PACK.md`. No MCTS/ISMCTS/Monte Carlo/ML/RL/hidden-state sampling; public played-card memory is allowed because it is public information. | The proof is a fair hidden-hand bot for state-dependent legality, not solver strength. |
| §11 universal invariants | Determinism, Rust behavior authority, replay/hash stability, legal-only controls, hidden-info no-leak, original IP, tests-before-done, and bounded termination (24 plays) are explicit exit criteria. | Every hidden-information surface is named in scope/tests, including DOM/test IDs/storage/dev panel/replay/bot explanations. |
| §12 stop conditions | Stop if implementation needs kernel nouns, unauthorized helper extraction, behavior-in-data, hidden leak, TS legality, solver bot, copied prose/trade dress, unbounded variants, test weakening, or proceeds past the third-use gate without a ledger decision. | Promoted to forbidden changes and failing-test protocol requirements. |
| §13 ADR verdict | **No ADR expected.** Trick/suit/hand nouns are game-local; existing action/effect/view/replay/RNG contracts cover the implementation; ADR 0004 supplies the hidden-info export taxonomy. An ADR becomes prerequisite only if the §5 item 2 ledger decision escalates (option 4) or implementation truly needs a new kernel semantic. | Matches the `poker_lite` precedent. |

## 9. Forbidden changes

- Do not modify `engine-core` to add card, deck, hand, suit, rank, trick,
  lead, follow, void, deal, or bot-policy nouns.
- Do not promote card/deck/hand/shuffle/trick or bot-policy helpers into
  `game-stdlib` except as the recorded §5 item 2 ledger decision authorizes.
- Do not proceed past crate-skeleton work before the §5 item 2 ledger decision
  is recorded.
- Do not encode legality, follow-suit checks, trick-winner logic, scoring,
  rotation, thresholds, selectors, triggers, branch logic, or bot policy in
  static data.
- Do not let TypeScript decide legality, follow-suit, trick winner, scoring,
  rotation, terminal outcome, bot decisions, or replay redaction.
- Do not use MCTS, ISMCTS, Monte Carlo dealing or rollouts, opponent-hand
  enumeration or sampling, hidden-state inference beyond public history, ML,
  RL, or LLM move selection for public bots.
- Do not leak unplayed hand cards or the tail through any browser-facing or
  export surface; do not reveal the tail at terminal.
- Do not copy rules prose, card imagery, or commercial trick-game trade
  dress; do not brand the game as Whist/Hearts.
- Do not edit `docs/ROADMAP.md` to record progress.
- Do not silently alter accepted ADR semantics or claim ADR 0005 is accepted
  unless the repository has accepted it.
- Do not migrate existing games' traces/hashes as a side effect of any
  promote decision without explicit design, documentation, and test coverage.
- Do not delete, weaken, rename away, or skip failing tests to get green.
  Follow the failing-test protocol: confirm validity, locate SUT vs test
  issue, fix the correct side, add/keep regression coverage, and report the
  outcome.

## 10. Documentation updates required

- `specs/README.md`: this spec's index row is added at status `Planned` when
  the spec lands. After implementation evidence passes, flip the Gate 10.1
  row to `Done`; because this spec closes the remaining trick/follow-suit
  half, also flip the parent Gate 10 row from `In progress` to `Done` at that
  point (and not before).
- `docs/MECHANIC-ATLAS.md`:
  - record the §5 item 2 third-use ledger decision in §10/§10B (and §10A only
    if promotion debt is created);
  - update the deterministic shuffle / private hand / staged reveal row from
    "third use pending" to the recorded decision;
  - record the trick-scoring-vs-accounting stance on the public
    resource-accounting row;
  - add first-use notes for trick-specific shapes (follow-suit legality,
    trick resolution, trick-winner-leads, deal rotation) as `local-only`.
- `games/plain_tricks/docs/*`: complete all per-game docs listed in §4.
- `docs/SOURCES.md`: cross-link the per-game source notes if repository
  practice keeps the central bibliography current (the Pagat trick-taking
  overview is already present).
- `progress.md`: after implementation, record Gate 10.1 evidence and flip the
  Gate 10 narrative from "trick half deferred" to complete.
- Root `README.md`: after implementation, add **Plain Tricks** /
  `plain_tricks` to the implemented-game catalog list.
- `apps/web/README.md`: reconcile in-gate, not aftermath: intro/catalog list
  includes **Plain Tricks**; Shell Surface renderer list includes
  `PlainTricksBoard`; Smoke Layers `smoke:e2e` list includes
  `plain-tricks.smoke.mjs`.
- `scripts/check-catalog-docs.mjs`: no script edit is needed — it is
  parametric, deriving the expected game set from the `GAME_*` consts in
  `crates/wasm-api/src/lib.rs` and checking that each appears in the
  `apps/web/README.md` intro catalog list, the root `README.md` "current
  official games are" list, and the `apps/web/README.md` Smoke Layers
  `smoke:e2e` bullet. Registering `plain_tricks` in the wasm-api consts and
  those three surfaces satisfies it. Note that the script intentionally does
  **not** check the `apps/web/README.md` Shell Surface renderer bullet, so the
  `PlainTricksBoard` renderer-list addition is not script-enforced and must be
  made by hand.
- `docs/ROADMAP.md`: no progress edit.

## 11. Sequencing

- **Predecessor:** Gate 10 `poker_lite` / Crest Ledger betting-showdown half,
  status complete per `progress.md` and the archived Gate 10 spec.
- **Admission condition:** `poker_lite` evidence has landed,
  `docs/MECHANIC-ATLAS.md` §10A is empty, no aftermath spec is owed, and the
  third-use hard gate is built into this gate's own work breakdown rather
  than deferred past it.
- **Current gate:** Gate 10.1 `plain_tricks` / **Plain Tricks**, Stage 10,
  status `Planned`.
- **Successor:** Gate 11 `masked_claims` (Stage 11, bluffing/reaction-window
  proof). It must not be admitted while this gate leaves promotion debt open
  in atlas §10A.
- **Closeout order:** admission/sources → third-use ledger decision → crate
  and rules tests → replay/visibility evidence → bot evidence pack and bots →
  tools/benchmarks → WASM/browser/e2e → docs/catalog updates → progress/index
  update.

## 12. Assumptions

- The default and only initial player count is two seats, `seat_0` and
  `seat_1`, matching every existing official game and the current shell.
- The display name **Plain Tricks** is acceptable; a maintainer may replace it
  with another original, neutral name before implementation without changing
  the internal id.
- The standard variant uses 18 cards (three suits × ranks 1–6, one copy
  each), 6 cards per seat per round, a 6-card never-revealed tail, no trump,
  2 rounds with deal/lead rotation, 1 point per trick, and Split on a 6–6
  tie. Each constant is one-line-correctable before implementation.
- Suggested neutral suit display labels are `Gale`, `River`, `Ember`; ranks
  display as numerals 1–6. Labels are presentation metadata and
  maintainer-replaceable.
- The §5 item 2 ledger decision is genuinely open: this spec's provisional
  lean is defer-reject (or at most a narrow behavior-free seeded-shuffle
  helper), but the implementation-time ledger comparison is authoritative and
  may decide otherwise, including ADR escalation.
- A trick won by the leader because the follower played off-suit is a normal
  win, not a special outcome; void revelation through off-suit play is
  rule-implied public information and is documented as such.
- The Level 2 bot is an authored heuristic, not a solver; it may remember
  publicly played cards but must never enumerate or sample hidden hands or
  the tail.
- ADR 0005 may be accepted before implementation; until then, implementation
  must not report ADR 0005 as accepted even if it follows variance-aware
  benchmark practice.
- The resolved tool-scope set (§4) is load-bearing: `simulate` /
  `replay-check` / `fixture-check` / `rule-coverage` / `bench-report` require
  a `plain_tricks` arm; `seed-reducer` / `trace-viewer` are not expected to
  need one. Re-confirm at implementation time only if a tool registry has
  changed.
- "Gate 10.1" remains this spec's sub-gate label for ROADMAP's single Gate 10,
  consistent with the archived Gate 10 spec, unless maintainers change
  sequencing in a future spec/ADR.

---

# Implementation reference — proposed **Plain Tricks** rules and seams

This section is non-canonical appendix material in the style of the Gate 9 and
Gate 10 sibling specs. It is intentionally concrete so that variant scope is
written before coding (ROADMAP §12 mandate).

## A. Core rules

### A1. Components

- **Seats:** `seat_0`, `seat_1`.
- **Deck:** eighteen `TrickCardId`s: three suits × ranks 1–6, one copy each.
  Suggested neutral display labels:
  - suit 1: `Gale`
  - suit 2: `River`
  - suit 3: `Ember`
  - ranks: numerals `1`–`6`; rank 6 is highest.
- **Hands:** 6 private cards per seat per round.
- **Tail:** the 6 undealt cards each round; internal only, never revealed,
  never used.
- **Rounds:** 2, each freshly shuffled and dealt from the same seeded RNG
  stream.
- **Tricks:** 6 per round; 12 per match; exactly 24 card plays per match.
- **Terminal outcomes:** `TrickWin { winner, points }` (most total points) or
  `Split { each }` on a 6–6 tie. The terminal rationale cites the per-round
  and total trick counts.

### A2. Setup and deal rotation

1. Construct the 18-card deck in stable id order.
2. Shuffle with the seeded deterministic RNG discipline already used by
   `high_card_duel` and `poker_lite` local shuffles (or the helper the §5
   item 2 ledger decision authorizes).
3. Deal 6 cards to `seat_0`, 6 to `seat_1`, leave 6 as the internal tail.
   (Deal order is a stable implementation detail covered by golden traces.)
4. Round 1: `seat_0` leads trick 1. Round 2: reshuffle the full deck from the
   continuing RNG stream, redeal, and `seat_1` leads trick 1.
5. Emit private `hand_dealt` effects per seat (own cards only) and a public
   `deal_completed` effect carrying counts and round index only.

### A3. Play and legality

One action family, `play`, one leaf per legal card in the actor's hand:

| Situation | Legal cards |
| --- | --- |
| Actor leads the trick | Every card in hand. |
| Actor follows and holds ≥1 card of the led suit | Exactly the held cards of the led suit (must follow). |
| Actor follows and is void in the led suit | Every card in hand (free discard). |

Action metadata may carry only public trick state (`round_index`,
`trick_index`, `led_suit`/`led_card` once led, `actor_seat`) plus, in the
actor's own tree only, the actor's own card id/labels. Non-actor viewers
receive an empty tree (existing hidden-info pattern). Diagnostics for an
illegal off-suit attempt say "a card of the led suit must be played" without
naming any other held card.

### A4. Trick resolution and round close

1. Leader plays; the led suit is that card's suit.
2. Follower plays under A3 legality.
3. Winner: if the follower followed suit, the higher rank of the led suit
   wins; if the follower played off-suit, the leader wins. Off-suit cards
   never win a trick. No within-trick tie is possible (single-copy deck).
4. Emit public `trick_resolved` effect (both played cards, winner, running
   trick counts). The trick winner leads the next trick.
5. After trick 6, emit public `round_scored` (per-seat trick counts and
   running totals). If round 1, rotate the deal per A2 and continue. If
   round 2, resolve terminal: higher total wins (`TrickWin`); 6–6 is `Split`.

### A5. Visibility model

- Own hand: visible to owner only, always.
- Opponent hand: count only, never identities, until cards are played.
- Played cards: public from the moment of play, permanently (current trick
  and trick history).
- Tail: never visible to anyone, including at terminal and in seat exports.
- Voids: revealed only implicitly and publicly by off-suit play; the views
  carry no explicit void flags for the opponent.
- Scores, trick counts, round index, leader, and turn state: public.
- Public observer view: counts, public trick surface, history, scores; no
  hand identities.
- Seat view: observer fields plus own hand.

### A6. Third-use ledger framing (provisional, non-binding)

What repeats across `high_card_duel`, `poker_lite`, `plain_tricks`: seeded
deterministic shuffle of a small opaque-id deck; per-seat private holdings;
viewer-filtered deal effects; hidden residue (tail/deck) that never goes
public. What differs: holding size (1 vs 1 vs 6), reveal model (commitment
reveal vs staged center + showdown vs play-by-play reveal), legality coupling
(none vs none vs follow-suit depends on hand contents), and terminal reveal
(showdown vs none). The plausible narrow promotable piece is a behavior-free
seeded shuffle/deal of opaque ids; reveal timing, legality, and zone semantics
are game policy and must stay local. The ledger entry (§5 item 2) makes the
binding decision.

### A7. Effects

- `deal_started` — public, round index, counts only.
- `hand_dealt` — `PrivateToSeat`, own cards only.
- `deal_completed` — public, counts and leader seat.
- `card_played` — public, seat, card id/labels, led-suit flag.
- `trick_resolved` — public, both cards, winner, trick counts.
- `round_scored` — public, per-seat trick counts, running totals.
- `deal_rotated` — public, round 2 leader.
- `match_resolved` — public, totals and decisive cause (terminal rationale).
- `terminal` — public, terminal outcome.
- `bot_chose_action` — public version carries policy id and chosen action
  family only; a private actor explanation may reference the bot's own hand
  reasoning but never opponent inferences beyond public history.

## B. Bot policy

### B1. Level 0 random-legal

Reuse the established pattern: collect legal leaf paths from the Rust action
tree, select deterministically from the bot seed, validate/apply normally. No
direct state mutation, no illegal fallback.

### B2. Level 2 authored policy

Policy id: `plain-tricks-level2-v1`. Authored after `COMPETENT-PLAYER.md` and
`BOT-STRATEGY-EVIDENCE-PACK.md`.

Allowed input: own seat, own hand, legal action tree, current trick state,
public play history (including publicly revealed voids), trick counts and
totals, round/trick index, terminal flag.

Forbidden input: opponent hand, tail, seed/reconstructed shuffle, full
internal trace, hidden payloads from replay, opponent bot private
explanations, sampled or enumerated hidden holdings.

Heuristic sketch (final priorities belong in the evidence pack):

1. never choose outside the Rust legal tree;
2. when following and able to win: win with the cheapest winning card if the
   trick or position justifies it; otherwise discard the lowest legal card;
3. when following and unable to win: discard the lowest legal card;
4. when leading: lead established winners (highest remaining rank of a suit
   by public history) before speculative leads; otherwise lead low from the
   longest suit;
5. stable deterministic tie-break by suit order then rank then stable card id.

Explanations are viewer-safe: they may cite own-hand reasoning and public
history ("led the highest remaining River card by public play"), never
opponent-hand claims.

## C. Replay/export model

- Internal full trace may include seed, both hands, and the tail for tests
  and deterministic replay.
- Browser public export follows the ADR 0004 viewer-scoped taxonomy: public
  timeline, redacted commands where needed, public effects, public terminal
  outcome; no seed material capable of reconstructing hands or tail.
- Seat-scoped export may include that seat's own hand observations at the
  times they were visible to that seat, never the opponent's unplayed cards
  and never the tail.
- Import of a public export replays the public timeline, not the hidden
  internal game.
- Replay-check golden traces test both internal deterministic replay and
  public export/import redaction.

## D. WASM/browser specifics

WASM additions: constants `GAME_PLAIN_TRICKS`, display name **Plain Tricks**,
`VARIANT_PLAIN_TRICKS_STANDARD`, trace rules version `plain-tricks-rules-v1`;
`RegisteredGame::PlainTricks`; `MatchRecord::PlainTricks`; list-games entry
with hidden-information tags; two-seat new-match setup; view/action/effect
serializers; actor-only `get_action_tree_for_viewer` authorization; bot-turn
branch with Level 0/Level 2 selection; replay export/import branch with public
timeline redaction.

Browser additions: `PlainTricksBoard.tsx` renders a neutral table surface with
own hand (seat view), opponent face-down count, current-trick surface, trick
history/score ledger, legal-only play controls from the Rust tree, outcome
explanation surface, viewer modes, replay controls, and reduced-motion
behavior. `ActionControls.tsx` uses no-leak-safe test ids (e.g.
`choice-plain-tricks-trick-${trick}-${index}`), never raw card ids in
non-actor contexts. `effectFeedback.ts` adds neutral copy for deal, play,
trick, score, rotation, and terminal effects. Dev panel stays whitelisted:
never raw state, hands, tail, or bot private rankings. E2E no-leak tests check
DOM text, accessibility names, `data-testid`, local storage, replay export
text, and dev panel content for unplayed-card identifiers.

## E. Benchmark operations

- setup + shuffle + deal (per round);
- legal action tree generation (leading, forced-follow, void cases);
- validate/apply for play, trick resolution, round close, rotation;
- observer projection and seat projection;
- public export/import;
- full random-legal match to terminal;
- Level 2 bot match to terminal.

Provisional native floor: at least 2,000 completed random-legal matches/sec
(24 plays each) on the established native benchmark machine, recorded in
`BENCHMARKS.md` as the native target with CI floors set separately per ADR
0002/0003 lanes and a named calibration follow-up. Failures against the
provisional floor are implementation-quality issues unless calibrated CI
evidence proves the floor inappropriate.

## F. Sources note to copy into game docs

The game docs should state that **Plain Tricks** is an original Rulepath
microgame in the public-domain trick-taking family. Classic trick-game
references (e.g. the Pagat trick-taking overview already recorded in
`docs/SOURCES.md`) were consulted for the mechanic family — leading,
following suit, trick capture, trick scoring — which is centuries-old public
domain. No rules prose, card imagery, product naming, or trade dress is
copied; the deck, suits, labels, and rules text are original.

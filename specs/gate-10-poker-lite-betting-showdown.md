# Gate 10 implementation spec — `poker_lite` betting / showdown proof

## 1. Header

| Field | Value |
| --- | --- |
| Spec ID | `GATE10-POKLITE-BETSHOW-001` |
| Roadmap stage | Stage 9 |
| Build gate | Gate 10 |
| Status | Planned |
| Date | 2026-06-08 |
| Owner | Rulepath maintainers / implementation agents |
| Primary crate | `games/poker_lite` |
| Internal game id | `poker_lite` |
| Chosen public display name | **Crest Ledger** |
| Standard variant id | `poker_lite_standard` |
| Browser implementation required | Yes — Rust/WASM-backed shell renderer and e2e smoke are in scope |
| Authority order | `docs/README.md` → `docs/FOUNDATIONS.md` → `docs/ARCHITECTURE.md` → `docs/ENGINE-GAME-DATA-BOUNDARY.md` → area docs / accepted ADRs → `docs/ROADMAP.md` → this spec → later AGENT-TASKs/tickets |
| Public presentation posture | Original, neutral, board-game-table framing; no casino trade dress, no real-money framing, no public “poker engine” claim |
| Kernel stance | No new kernel concept; betting, shared-pool accounting, cards, reveal, and showdown are game-local `poker_lite` behavior expressed through existing action-tree, command-envelope, semantic-effect, visibility, replay, and WASM envelopes |

## 2. Objective

Gate 10 implements `poker_lite` as the next mechanic-ladder proof: a small, deterministic, two-seat, browser-playable imperfect-information microgame with private cards, one public card, bounded betting-style pledge rounds, exact shared-pool accounting, deterministic showdown, viewer-safe reveal/export behavior, and a Level 2 authored-policy bot that does not cheat.

The locked public display name for this spec is **Crest Ledger**. The internal id remains `poker_lite` because the roadmap and prompt use that id, but the user-facing shell, docs prose, effect copy, and UI should present **Crest Ledger** and neutral terms such as *crest*, *marker*, *pledge*, *shared pool*, *hold*, *press*, *lift*, *match*, and *yield*. Public copy must not present a casino product, a money game, a poker clone, or casino trade dress.

The next-gate determination is not reopened here. Gate 10 `poker_lite` is the required target because:

- `specs/README.md` at the target commit records Gates 0 through 9.1 as `Done` and leaves Gate 10 not started.
- Gate 9.1’s own sequencing names Gate 10 `poker_lite` / `plain_tricks` as successor and says it should wait for no-leak waiting/reveal evidence and no open promotion debt.
- `docs/MECHANIC-ATLAS.md` §10A is empty at the target commit, so no promotion-debt interlock blocks the next mechanic-ladder gate.
- `docs/ROADMAP.md` (§12 "Gate 10: betting and tricks", and the Stage/gate crosswalk rows) places **both** `poker_lite` (Stage 9) and `plain_tricks` (Stage 10) under a **single Gate 10** as two build candidates. This spec builds the lower-stage `poker_lite` (betting/showdown) candidate first; `plain_tricks` (lead/follow/trick scoring) is sequenced after it. Because ROADMAP's Gate 10 exit list also names trick/follow-suit rules, **completing this spec closes only the betting/showdown half of Gate 10** — the trick-taking exit rows remain open for the `plain_tricks` successor (see §6 and §11). The "Gate 10.1" label used for `plain_tricks` in §11/§12 is this spec's *proposed* sub-gate split mirroring the accepted Gate 9 / Gate 9.1 precedent; it is not stated in `docs/ROADMAP.md`, which keeps both candidates under Gate 10.
- Gate 9.1 already folded web-catalog reconciliation into its gate. No separate aftermath spec is owed before this one.

Design altitude: **Crest Ledger** is deliberately closer to research-minimal Kuhn/Leduc poker than to commercial poker. Kuhn’s simplified game demonstrates that a two-player private-card game with tiny deck, ante, bounded pass/bet decisions, and deterministic terminal comparison can preserve the essential imperfect-information pressure while remaining small enough for proof work.[^kuhn] Leduc-style benchmark games add a tiny paired deck, one private card, one public card, two bounded betting rounds, limited raises, and pair-before-high-card comparison; that structure informs this spec’s proof shape without copying public rules text or trade dress.[^leduc] OpenSpiel’s information-state / observation framing supports the design target: player decisions must be made from what that player is allowed to observe, not from hidden histories.[^openSpielPaper][^openSpielDocs]

The resulting gate proves Rulepath can support:

1. deterministic hidden-card setup and staged public reveal;
2. bounded pledge/betting semantics without engine-core nouns;
3. public shared-pool accounting and split/tie resolution;
4. showdown reveal that remains viewer-safe until the reveal point;
5. a competent, fair, beatable Level 2 bot using its own allowed private view only;
6. web presentation, replay export/import, and e2e no-leak behavior for a casino-adjacent mechanic under neutral IP rules.

## 3. Scope

### In scope

- New Rust crate `games/poker_lite` with the same practical layout as `token_bazaar` and `secret_draft`: `src/*`, `data/*`, `tests/*`, `benches/*`, and `docs/*`.
- Concrete original rules for **Crest Ledger**, fully specified before implementation:
  - two seats, `seat_0` and `seat_1`;
  - one private card per seat;
  - one initially hidden center card that becomes public after round 1 if no one yields;
  - fixed six-card deck: three ranks, two copies per rank, original neutral labels;
  - automatic one-marker opening pledge from each seat;
  - two pledge rounds with fixed round units `[1, 2]` and at most one lift per round;
  - immediate terminal resolution on yield;
  - deterministic showdown after round 2 if both seats remain active;
  - pair-before-high-card comparison and exact split on true tie;
  - fixed maximum action count by construction.
- Full Rust-owned legality, validation, transitions, effects, view projection, replay serialization, public export/import, bot decisions, and diagnostics.
- Level 0 random-legal bot plus Level 2 authored-policy bot with `BOT-STRATEGY-EVIDENCE-PACK.md` and `COMPETENT-PLAYER.md`.
- Native unit/rule/property/replay/serialization/visibility-no-leak/bot tests, golden traces, simulation support, benchmark support, fixture checking, rule coverage, replay checking, trace viewing as applicable.
- WASM registration and browser shell support, including a new neutral `PokerLiteBoard.tsx` renderer for **Crest Ledger**, effect-feedback copy, action controls, typed client view/effect definitions, replay import/export, no-leak dev-panel behavior, and e2e smoke.
- In-gate documentation/catalog reconciliation: `specs/README.md`, `docs/MECHANIC-ATLAS.md`, per-game docs, root `README.md`, `progress.md`, and `apps/web/README.md` after implementation evidence lands.
- Source notes for Kuhn, Leduc, and OpenSpiel-style information-state references, routed to `games/poker_lite/docs/SOURCES.md` and, if the repository-level bibliography remains the consolidated index, `docs/SOURCES.md`.

### Out of scope

- A general poker engine.
- Texas Hold’em, Omaha, draw poker, commercial casino poker variants, side pots, all-in logic, rake, betting stacks, cash values, payouts, tournaments, blind structures, insurance, real-money balances, multiplayer tables, or configurable poker families.
- New engine-core types or engine-core terminology for cards, decks, hands, betting, pots, showdown, suits, ranks, chips, folds, calls, raises, or solver policy.
- `game-stdlib` promotion of card, deck, hand, betting, shared-pool, showdown, or bot-policy helpers.
- TypeScript legality, TypeScript showdown/tie-break logic, TypeScript shared-pool accounting, TypeScript hidden-card timing, or TypeScript bot decision logic.
- Any MCTS, ISMCTS, Monte Carlo equity simulation, opponent-card enumeration, ML, RL, or hidden-state sampling.
- Any copied rules prose, copied hand-ranking table, casino layout, poker table/felt/chip/payout art direction, proprietary naming, or public “poker” branding.
- A separate aftermath spec. Web catalog closeout is folded into this gate.

### Not allowed

The `docs/ROADMAP.md` §12 prohibitions are binding and are repeated here as gate law:

- no real-money or casino features;
- no unbounded variants;
- no hidden-state cheating;
- no ML/RL;
- no copied rules prose.

Gate-local additions:

- no public MCTS/ISMCTS/Monte Carlo bot, even as an “analysis-only” helper;
- no hidden-card identifiers in public action-tree metadata, diagnostics, effect logs, test IDs, storage, dev panel dumps, replay exports, bot explanations, or candidate rankings before showdown;
- no automatic reveal of a yielded seat’s hidden card after a yield terminal;
- no accidental trace/hash migration unless explicitly designed, documented, and covered by tests.

## 4. Deliverables

### New crate and source tree

Create the new crate and register it in the workspace:

```text
games/poker_lite/Cargo.toml
games/poker_lite/src/actions.rs
games/poker_lite/src/bots.rs
games/poker_lite/src/effects.rs
games/poker_lite/src/ids.rs
games/poker_lite/src/lib.rs
games/poker_lite/src/replay_support.rs
games/poker_lite/src/rules.rs
games/poker_lite/src/setup.rs
games/poker_lite/src/state.rs
games/poker_lite/src/ui.rs
games/poker_lite/src/variants.rs
games/poker_lite/src/visibility.rs
```

The crate mirrors the current game shape:

- `ids.rs`: `PokerLiteSeat`, `CrestCardId`, `CrestRank`, action segment constants, variant id constants, stable label helpers.
- `state.rs`: internal phase, seats, shuffled deck, private cards, center card, center visibility, active seat, pledge-round state, per-seat contributions, shared-pool total, terminal outcome, effect history, freshness token. Hidden cards and deck tail are internal only.
- `setup.rs`: deterministic six-card deck construction and shuffle using existing seeded-RNG discipline; no new kernel RNG concept.
- `actions.rs`: legal action tree, safe action metadata, path parsing, command validation, stale/wrong-seat/terminal/malformed/insufficient-state diagnostics.
- `rules.rs`: transition engine for hold/press/lift/match/yield, round close, center reveal, showdown reveal, exact accounting, terminal outcome, freshness increments.
- `effects.rs`: typed semantic effects with public/private visibility envelopes; grouped reveal effects for center and showdown.
- `visibility.rs`: `PublicView`, `SeatPrivateView`, observer and seat projection, stable summaries, no-leak helpers.
- `bots.rs`: Level 0 random legal and Level 2 authored-policy bot using only its own legal/private view.
- `replay_support.rs`: golden trace command replay, internal full trace for tests, public observer/seat export and import, redacted command summaries.
- `ui.rs`: neutral display labels, rules summaries, accessibility copy, no casino language.
- `variants.rs`: strict typed parsing of data manifests; reject behavior-looking keys.
- `lib.rs`: public crate surface matching established games.

### Data, fixtures, benchmarks

```text
games/poker_lite/data/manifest.toml
games/poker_lite/data/variants.toml
games/poker_lite/data/fixtures/poker_lite_standard.fixture.json
games/poker_lite/benches/poker_lite.rs
games/poker_lite/benches/thresholds.json
```

Data files contain typed content, labels, variant metadata, fixtures, and version declarations only. They must not contain selectors, conditions, formulas, scripts, YAML, DSL fragments, action legality expressions, showdown formulas, betting formulas, bot policy rules, or hidden-card routing logic. The standard variant constants are written in Rust first and documented in `RULES.md`; data may mirror display metadata but does not own behavior.

### Tests and golden traces

```text
games/poker_lite/tests/rules.rs
games/poker_lite/tests/property.rs
games/poker_lite/tests/replay.rs
games/poker_lite/tests/serialization.rs
games/poker_lite/tests/visibility.rs
games/poker_lite/tests/bots.rs
games/poker_lite/tests/golden_traces/deal-private-no-leak.trace.json
games/poker_lite/tests/golden_traces/hold-hold-center-reveal.trace.json
games/poker_lite/tests/golden_traces/press-match-showdown-reveal.trace.json
games/poker_lite/tests/golden_traces/lift-match-showdown.trace.json
games/poker_lite/tests/golden_traces/yield-terminal-no-showdown.trace.json
games/poker_lite/tests/golden_traces/pair-beats-high-card.trace.json
games/poker_lite/tests/golden_traces/high-card-showdown.trace.json
games/poker_lite/tests/golden_traces/tie-split.trace.json
games/poker_lite/tests/golden_traces/no-leak-public-observer.trace.json
games/poker_lite/tests/golden_traces/seat-private-view.trace.json
games/poker_lite/tests/golden_traces/invalid-wrong-seat-diagnostic.trace.json
games/poker_lite/tests/golden_traces/invalid-stale-diagnostic.trace.json
games/poker_lite/tests/golden_traces/invalid-lift-cap-diagnostic.trace.json
games/poker_lite/tests/golden_traces/invalid-private-card-redacted.trace.json
games/poker_lite/tests/golden_traces/bot-action.trace.json
games/poker_lite/tests/golden_traces/public-replay-export-import.trace.json
games/poker_lite/tests/golden_traces/wasm-exported.trace.json
```

Golden traces should be added only after Rust rules are implemented enough to generate stable evidence. Trace names may be adjusted, but coverage categories may not be deleted or weakened.

### Per-game documentation

Instantiate the official documentation set from templates:

```text
games/poker_lite/docs/RULES.md
games/poker_lite/docs/SOURCES.md
games/poker_lite/docs/RULE-COVERAGE.md
games/poker_lite/docs/MECHANICS.md
games/poker_lite/docs/AI.md
games/poker_lite/docs/BOT-STRATEGY-EVIDENCE-PACK.md
games/poker_lite/docs/COMPETENT-PLAYER.md
games/poker_lite/docs/UI.md
games/poker_lite/docs/BENCHMARKS.md
games/poker_lite/docs/GAME-IMPLEMENTATION-ADMISSION.md
games/poker_lite/docs/PUBLIC-RELEASE-CHECKLIST.md
games/poker_lite/docs/PRIMITIVE-PRESSURE-LEDGER.md
```

`BOT-STRATEGY-EVIDENCE-PACK.md` and `COMPETENT-PLAYER.md` are mandatory because this gate requires a Level 2 authored-policy bot. `PRIMITIVE-PRESSURE-LEDGER.md` is mandatory for this gate because `poker_lite` is explicitly a second-use comparison point for hidden cards and accounting, and first use for bounded pledge/shared-pool mechanics. Note: no existing game (`high_card_duel`, `token_bazaar`, `secret_draft`) instantiates a per-game `PRIMITIVE-PRESSURE-LEDGER.md`, so this is the first per-game instance — it must **cross-reference, not duplicate**, the repo-level `docs/MECHANIC-ATLAS.md` §10B entries for card/private-hand (first `high_card_duel`) and resource accounting (first `token_bazaar`).

### WASM, tools, CI, and web shell

- `crates/wasm-api/src/lib.rs`: register `poker_lite`, display name **Crest Ledger**, standard variant, hidden-information tags, viewer modes, match record, new-match setup, get-view, get-action-tree, apply-action, run-bot-turn, effects, export/import, replay-step/reset, JSON serializers, redaction tests.
- `Cargo.toml`: add the workspace member and dependencies consistently with other games.
- Tools that enumerate game ids must register `poker_lite`. Resolved tool-scope set (validated against the current tool registries): the four tools that register **all** games — `simulate`, `replay-check`, `fixture-check`, `rule-coverage` — require a `poker_lite` arm. `bench-report` currently registers only games with threshold files (`race_to_n`, `column_four`, `directional_flip`, `draughts_lite`, `high_card_duel`; `token_bazaar`/`secret_draft` are absent), so a `poker_lite` entry there is **optional** and follows the benchmark-lane decision. `seed-reducer` and `trace-viewer` currently register only `race_to_n` + `directional_flip`; `poker_lite` is **not expected** to need an entry in either, matching the `token_bazaar`/`secret_draft`/`high_card_duel` precedent. Add to these last three only if a concrete need arises.
- Native benchmark lane: add `games/poker_lite/benches/poker_lite.rs`, benchmark thresholds, and `gate-2-benchmarks.yml` registration.
- Native smoke lane: add `gate-1-game-smoke.yml` game smoke / simulate / replay hooks.
- Web shell: add `apps/web/src/components/PokerLiteBoard.tsx`; add the `PokerLitePublicView` type to `apps/web/src/wasm/client.ts` (and into its `PublicView` union); wire renderer selection in `apps/web/src/main.tsx` — the per-game dispatch site that holds the board imports, the `is<Game>View()` type guards, and the render conditional (add a `PokerLiteBoard` import, an `isPokerLiteView()` guard, a render clause alongside `isSecretDraftView`/`isHighCardDuelView`, and the corresponding `ActionControls` handling); add neutral effect feedback in `effectFeedback.ts`, no-leak-safe action test IDs in `ActionControls.tsx`, and any shell reducer/client additions required by the WASM API.
- E2E: add `apps/web/e2e/poker-lite.smoke.mjs` and append it to the hardcoded `smoke:e2e` `&&`-chain in `apps/web/package.json`; update existing no-leak/a11y smoke coverage where the test harness expects hidden-info games.
- Catalog docs/scripts: update `apps/web/README.md` and satisfy `scripts/check-catalog-docs.mjs` in-gate.

## 5. Work breakdown

Do not create tickets in this spec. The following are bounded candidate AGENT-TASKs in dependency order.

1. **Admission and source packet.** Add `games/poker_lite/docs/SOURCES.md`, `GAME-IMPLEMENTATION-ADMISSION.md`, and the initial `RULES.md` skeleton from this spec. Record Kuhn/Leduc/OpenSpiel influence, neutral naming, and IP constraints. Confirm no open atlas §10A debt before coding.
2. **Crate skeleton and typed ids.** Add `games/poker_lite` to the workspace with ids, variants, display labels, strict data parsing, fixture manifest, and compile-only exports. No engine-core or game-stdlib change.
3. **Deterministic setup and internal state.** Implement six-card deck, seeded shuffle, private seat cards, hidden center card, opening contributions, active seat/round state, freshness token, and stable internal serialization helpers.
4. **Legal action tree and validation.** Implement Rust-owned action families `hold`, `press`, `lift`, `match`, and `yield`, with bounded round units and one-lift cap. Metadata may expose public round/accounting deltas only. Add stale, wrong-seat, terminal, malformed-path, unavailable-action, and lift-cap diagnostics that reveal no hidden cards.
5. **Rules and effects.** Implement transitions, round-close logic, center reveal, immediate yield terminal, showdown reveal, pair/high-card/tie comparator, exact shared-pool accounting, terminal outcome, and viewer-scoped semantic effects.
6. **Visibility and no-leak tests.** Implement observer/seat views, private card visibility for owner only, public center visibility after reveal only, showdown grouped reveal, folded-hand non-reveal, public export redaction, and exhaustive string-search tests for hidden card ids/ranks/labels across browser-facing surfaces.
7. **Replay and golden traces.** Add internal trace replay, public replay export/import per ADR 0004 taxonomy, deterministic hash checkpoints, and the golden trace set. Public exports must not include seed material capable of reconstructing private cards.
8. **Bots.** Add Level 0 random-legal bot and Level 2 authored-policy bot. The Level 2 bot consumes only its own allowed private view plus public betting/center/pool state, emits viewer-safe explanation effects, and has no opponent hidden-hand sampling. Complete `AI.md`, `BOT-STRATEGY-EVIDENCE-PACK.md`, and `COMPETENT-PLAYER.md`.
9. **Native tools and benchmarks.** Register `poker_lite` in simulate, replay-check, fixture-check, rule-coverage, seed-reducer/trace-viewer/bench-report as needed. Add benchmarks and provisional thresholds, then run calibration follow-up under accepted benchmark ADRs. If ADR 0005 has become accepted, apply its variance-aware floor discipline; otherwise do not claim it as accepted.
10. **WASM registration.** Add WASM game constants, match record variant, serializers, viewer-safe action tree access, bot turn, effects, export/import, replay stepping, and no-leak unit tests. Keep action-tree access authorized: a non-actor viewer gets an empty tree for the actor, matching the existing hidden-info pattern.
11. **Browser renderer and smoke.** Add `PokerLiteBoard.tsx` for **Crest Ledger**, effect feedback, neutral legal-only controls, viewer-mode UI, dev-panel whitelist behavior, public observer no-leak display, replay import/export controls, reduced-motion behavior, and `poker-lite.smoke.mjs`.
12. **Official-doc closeout.** Complete per-game docs, rule coverage matrix, mechanics ledger, public release checklist, benchmark docs, and documentation updates. Update `specs/README.md`, `docs/MECHANIC-ATLAS.md`, `progress.md`, root `README.md`, and `apps/web/README.md`; run doc-link and catalog checks. Do not edit `docs/ROADMAP.md` merely to record progress.

## 6. Exit criteria

| ROADMAP §12 Gate 10 exit row | Gate 10 `poker_lite` acceptance mapping |
| --- | --- |
| Betting/trick rules correct for chosen variants | The chosen variant is **Crest Ledger** / `poker_lite_standard`. Rules tests and golden traces prove two bounded pledge rounds, one-lift cap, center reveal timing, yield terminal, showdown reveal, pair/high-card/tie comparator, action legality, stale validation, and deterministic terminal outcomes. Trick-taking is not part of this chosen candidate; `plain_tricks` is the successor. |
| Pot/accounting and follow-suit edge cases covered | Shared-pool accounting tests cover opening contributions, press, lift, match, yield, no-overrun cap, equalized showdown contributions, split ties, and terminal payout summaries. Follow-suit is out of scope because this spec selects `poker_lite`; the successor `plain_tricks` must cover lead/follow/trick scoring. |
| Bots finish games without hidden-state cheating | Level 0 and Level 2 bots run through simulation to terminal under an action cap. Level 2 consumes only its own allowed private card plus public center/pool/history and legal action tree; tests assert no opponent card, deck tail, hidden center before reveal, or hidden-state-derived candidate ranking enters bot input or explanations. |
| No public MCTS/ISMCTS | `poker_lite` exposes no MCTS, no ISMCTS, no Monte Carlo equity simulation, no ML/RL, and no hidden-state sampling. The Level 2 policy is authored, deterministic, explainable, and beatable. |
| UI remains understandable | The browser presents **Crest Ledger** with neutral language, legal-only Rust-supplied controls, public pool/contribution summaries, clear center/showdown reveal states, observer/seat viewer modes, reduced-motion behavior, and no casino imagery/language. Human+bot, hotseat, bot-vs-bot, replay, and no-leak e2e smokes pass. |
| Native benchmarks exist | `games/poker_lite/benches/poker_lite.rs` and thresholds exist; CI smoke lanes run. Provisional floor target: at least 2,000 completed hands/sec on the established native benchmark machine unless calibration evidence sets a stricter accepted floor. Benchmark reports include variance/calibration notes and avoid flaky hard-gate claims. |

Universal hidden-info no-leak exit criteria:

- Before center reveal, no observer/opponent/browser payload contains the center card id, rank, label, or deck-tail identity.
- Before showdown, no observer/opponent/browser payload contains either hidden private card except the owning seat’s private view contains only that seat’s own card.
- On yield terminal, the yielded seat’s private card remains unrevealed in public and opponent views forever; public export/import cannot reconstruct it.
- At showdown, reveal occurs as one grouped public reveal with both private cards and the center card already public; no partial reveal effect leaks order-dependent private state.
- Hidden identifiers are absent from action-tree metadata, previews, diagnostics, effect logs, DOM text, `data-testid`, local storage, dev panel data, replay export, bot explanations, and candidate rankings until the rule-defined reveal point.

## 7. Acceptance evidence

### Required command/evidence set

Implementation closeout must record the exact command outputs in the gate PR or closeout note:

```text
cargo fmt --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
cargo test -p poker_lite
cargo test -p poker_lite --test rules
cargo test -p poker_lite --test property
cargo test -p poker_lite --test replay
cargo test -p poker_lite --test serialization
cargo test -p poker_lite --test visibility
cargo test -p poker_lite --test bots
cargo run -p simulate -- --game poker_lite --games 1000 --start-seed 0 --action-cap 16
cargo run -p replay-check -- --game poker_lite   # --all is the default mode; scans every poker_lite golden trace
cargo run -p fixture-check -- --game poker_lite
cargo run -p rule-coverage -- --game poker_lite
cargo bench -p poker_lite
./scripts/boundary-check.sh
node scripts/check-doc-links.mjs
node scripts/check-catalog-docs.mjs
npm --prefix apps/web run smoke:wasm
npm --prefix apps/web run smoke:ui
npm --prefix apps/web run smoke:e2e   # poker-lite.smoke.mjs and a11y-noleak.smoke.mjs run inside this hardcoded chain; smoke:e2e takes no positional arg
```

The `smoke:e2e` script in `apps/web/package.json` is a hardcoded `node e2e/*.smoke.mjs` `&&`-chain, not a positional-arg runner; `poker-lite.smoke.mjs` must be appended to that chain (a11y-noleak already runs in it). The final commands must match the actual tool CLIs at implementation time. If the current tool interface differs, update this evidence section in the implementation closeout without weakening coverage.

### Test taxonomy

- **Unit/rule tests:** legal action generation; path parsing; validation; state transitions; round-close; center reveal; showdown; yield terminal; split/tie accounting; terminal outcome; malformed/stale/wrong-seat diagnostics.
- **Property tests:** deterministic replay from seed and command stream; action cap terminal bound; contributions never negative; shared pool equals contribution sum; legal action tree never offers illegal lift after cap; no hidden id in public-facing projections before reveal.
- **Golden traces:** named trace set from §4, including deal, press/match, lift, yield, showdown reveal, tie split, no-leak public observer, diagnostics, bot action, public export/import, and WASM-exported trace.
- **Replay tests:** internal full replay reproduces hashes; public replay export/import supports observer and seat viewers; public export omits private seed material per ADR 0004; folded private cards remain unreconstructable.
- **Serialization tests:** stable field order, strict unknown-field rejection, stable IDs, no behavior-looking trace/data fields, no accidental schema migration.
- **Visibility-no-leak tests:** explicit string search for every hidden card id, rank label, sigil label, deck-tail id, and folded-hand id across view JSON, action-tree JSON, effect JSON, diagnostic JSON, replay export, web smoke DOM, dev panel, local storage, `data-testid`, bot explanation, and candidate ranking.
- **AI tests:** Level 0 uses legal actions; Level 2 action is legal, deterministic under tie-breaks, uses only `PokerLiteBotInput`, never receives opponent private card or deck tail, produces viewer-safe explanation effects, and completes simulations under cap.
- **Browser e2e:** human-vs-bot path with press/match/showdown; yield path; observer no-leak before showdown; seat-private own-card view; public replay export/import; reduced motion; keyboard/a11y smoke; stale diagnostic smoke; dev panel whitelist smoke.
- **Benchmarks:** setup+playout throughput, legal-action generation, apply-action, projection for observer and seat, public export/import. Benchmarks start with smoke floors and a named calibration follow-up under ADR 0002/0003. If ADR 0005 is accepted in the repo by then, use its variance-aware floor policy; do not claim that status from this spec alone.

### Official-game contract evidence

The gate is not done until the official-game deliverable set exists and is internally consistent:

- original `RULES.md` with rule IDs;
- `SOURCES.md` with external research notes and non-copying/IP posture;
- `RULE-COVERAGE.md` mapping rules to tests/traces/docs/UI;
- `MECHANICS.md` and `PRIMITIVE-PRESSURE-LEDGER.md` with second-use / first-use stances;
- `AI.md`, `BOT-STRATEGY-EVIDENCE-PACK.md`, and `COMPETENT-PLAYER.md` for Level 2 bot;
- `UI.md`, `BENCHMARKS.md`, `GAME-IMPLEMENTATION-ADMISSION.md`, `PUBLIC-RELEASE-CHECKLIST.md`;
- native tests/traces/replay/serialization/visibility tests;
- simulation and benchmarks;
- WASM/browser registration and e2e smoke;
- docs/catalog checks green.

## 8. FOUNDATIONS & boundary alignment

| Principle / source | Gate stance | Rationale |
| --- | --- | --- |
| §2 behavior authority | Rust owns setup, legal actions, validation, transitions, effects, scoring/showdown, hidden-info projection, replay/export, and bot decisions. | `poker_lite` extends the existing Rust game-crate model; TypeScript renders only WASM-provided view/action/effect data. |
| §3 noun-free `engine-core` | No kernel noun is added for card, deck, hand, rank, suit, betting, bet, raise, call, fold, pot, pool, showdown, chip, or pledge. | The fetched engine-core has generic `ActionTree`, `CommandEnvelope`, `EffectEnvelope`, `VisibilityScope`, deterministic RNG, replay, and view contracts sufficient for this gate. Typed nouns live in `games/poker_lite`. |
| §4 earned `game-stdlib` | No promotion. Cards/private hands are second use after `high_card_duel`; resource/accounting is second use after `token_bazaar`; bounded pledge/shared-pool is first use. | Foundation rule: first use local, second use local + compare, third use hard gate. This gate records comparisons and keeps implementation local. If a helper feels unavoidable, stop and write/update a primitive-pressure ledger entry first. |
| Second-use card/private-hand stance | Keep local. Compare `high_card_duel` deterministic shuffle/private hand/viewer-filtered effects with `poker_lite` staged center card/showdown reveal. | `poker_lite` adds betting and staged reveal pressure but still fits the same local hidden-info envelope. Third card game, likely `plain_tricks`, must revisit extraction pressure. |
| Second-use resource/accounting stance | Keep local. Compare `token_bazaar` public ledgers/tie-break rigor with `poker_lite` contributions/shared-pool/split handling. | Accounting is public and deterministic, but the domain shape differs enough that generic extraction would be premature. Third economy/accounting game is the review gate. |
| First-use betting/pot stance | Keep local. Add first-use atlas note for bounded pledge rounds and shared-pool terminal allocation. | Betting/pot vocabulary is explicitly forbidden in engine-core; one official use cannot justify `game-stdlib` extraction. |
| §5 static data | Data contains labels, manifests, fixtures, and version metadata only. No formulas, selectors, triggers, scripts, branch conditions, or behavior-like YAML/DSL. | Round units, lift cap, comparator, reveal timing, and legal-action rules are typed Rust behavior. |
| §8 public bots | Level 0 random legal required; Level 2 authored-policy bot required. No MCTS/ISMCTS/Monte Carlo/ML/RL/hidden-state sampling. | The proof is an imperfect-information fair-bot proof, not solver strength. The bot acts from its own information state and public state only. |
| §11 universal invariants | Determinism, Rust behavior, replay/hash stability, legal-only controls, hidden-info no-leak, original IP, tests-before-Done, and bounded termination are all explicit exit criteria. | Every hidden-information surface is named in scope/tests, including DOM/test IDs/storage/dev panel/replay/bot explanations. |
| §12 stop conditions | Stop if implementation needs kernel nouns, shared helper extraction, behavior-in-data, hidden leak, TS legality, solver bot, casino/IP trade dress, unbounded variants, or test weakening. | These are promoted to forbidden changes and failing-test protocol requirements. |
| §13 ADR verdict | **No ADR required expected.** | Betting/showdown/accounting are game-local nouns; existing generic action/effect/view/replay/RNG contracts cover the implementation. ADR 0004 already supplies the hidden-info public export taxonomy. An ADR becomes prerequisite only if implementation truly needs a new kernel visibility/replay semantic or shared primitive before third use. |

## 9. Forbidden changes

- Do not modify `engine-core` to add card, deck, hand, rank, suit, betting, bet, raise, call, fold, pot, pool, pledge, chip, showdown, or bot-policy nouns.
- Do not promote card/deck/hand, betting/shared-pool/showdown, or bot-policy helpers into `game-stdlib` in this gate.
- Do not encode legal actions, betting formulas, showdown formulas, thresholds, selectors, trigger conditions, branch logic, or bot policy in static data.
- Do not let TypeScript decide legality, turn order, round-close, reveal timing, shared-pool accounting, winner/tie-break, bot decisions, or replay redaction.
- Do not use MCTS, ISMCTS, Monte Carlo equity simulation, opponent-hand enumeration, hidden-state sampling, ML, RL, or LLM move selection for public bots.
- Do not leak hidden cards through public/opponent view, action tree, preview, diagnostic, effect, replay export, DOM text, `data-testid`, local storage, dev panel, bot explanation, candidate ranking, benchmark output, trace summary, or failure report before reveal.
- Do not reveal a yielded private card merely because the match is terminal; yield terminal is not showdown.
- Do not add real-money, casino, chip, payout, insurance, blind, rake, tournament, table/felt, or copied poker trade-dress features.
- Do not resurrect or redirect this gate to blackjack or blackjack-like draw/stand mechanics.
- Do not edit `docs/ROADMAP.md` to record progress.
- Do not silently alter accepted ADR semantics or claim ADR 0005 is accepted unless the repository has accepted it.
- Do not delete, weaken, rename away, or skip failing tests to get green. Follow the failing-test protocol: confirm validity, locate SUT vs test issue, fix the correct side, add/keep regression coverage, and report the outcome.

## 10. Documentation updates required

- `specs/README.md`: a Gate 10 row already exists (`| Gate 10 | poker_lite / plain_tricks — not yet specced | Not started |`). **Update** that row to link `specs/gate-10-poker-lite-betting-showdown.md` for the `poker_lite` portion and flip Status `Not started → Planned` while this spec is pending; leave the `plain_tricks` portion noted as not-yet-specced. After implementation evidence lands, update status under the normal closeout process. Because this spec closes only the betting/showdown half of Gate 10, do not mark the whole Gate 10 row `Done` on `poker_lite` landing alone.
- `docs/MECHANIC-ATLAS.md`:
  - keep §10A open promotion debt empty unless implementation creates actual debt;
  - add §10B first-use note: bounded pledge rounds / shared-pool accounting / showdown allocation, first official use in `poker_lite`, local only;
  - add second-use comparison note for deterministic shuffle/private hand/staged reveal: first `high_card_duel`, second `poker_lite`, keep local, third-use review before/at `plain_tricks` if it uses the same card/private-hand primitive;
  - add second-use comparison note for public resource/accounting ledgers: first `token_bazaar`, second `poker_lite`, keep local, third-use review before another economy/accounting game.
- `games/poker_lite/docs/*`: complete all per-game docs listed in §4.
- `docs/SOURCES.md`: add or cross-link the new source-research entries if repository practice keeps the central bibliography current; otherwise ensure `games/poker_lite/docs/SOURCES.md` is complete and follows repository source-use rules.
- `progress.md`: after implementation, record Gate 10 evidence and status. Do not mark done before tests, docs, benchmarks, wasm, browser, and catalog evidence exist.
- Root `README.md`: after implementation, add **Crest Ledger** / `poker_lite` to the implemented game/catalog list using neutral presentation.
- `apps/web/README.md`: reconcile in-gate, not aftermath:
  - intro/catalog list includes **Crest Ledger**;
  - Shell Surface renderer list includes `PokerLiteBoard`;
  - Smoke Layers `smoke:e2e` list includes `poker-lite.smoke.mjs` and any no-leak/a11y coverage additions.
- `scripts/check-catalog-docs.mjs`: update expectations if hardcoded catalog lists require `poker_lite`.
- `docs/ROADMAP.md`: no progress edit. Only edit if a future accepted roadmap-change spec/ADR requires it.

## 11. Sequencing

- **Predecessor:** Gate 9.1 `secret_draft` / Veiled Draft, status `Done` in the target-commit progress index.
- **Admission condition:** Gate 9.1 no-leak waiting/reveal evidence has landed, `docs/MECHANIC-ATLAS.md` §10A is empty, this gate creates no skipped promotion debt, and no aftermath spec is owed.
- **Current gate:** Gate 10 `poker_lite` / **Crest Ledger**, Stage 9, status `Planned`.
- **Successor:** `plain_tricks`, expected to cover lead/follow/trick scoring and likely to revisit card-helper pressure if it becomes the third official card-game use. `docs/ROADMAP.md` keeps `plain_tricks` under the same **Gate 10** (Stage 10); this spec designates it **Gate 10.1** as a *proposed* sub-gate split — mirroring the accepted Gate 9 / Gate 9.1 precedent — rather than a label ROADMAP already uses. The trick/follow-suit rows of ROADMAP's Gate 10 exit list stay open until this successor lands.
- **Closeout order:** Rust crate and tests → replay/visibility evidence → bot evidence pack → tools/benchmarks → WASM/browser/e2e → docs/catalog updates → progress/index update.

## 12. Assumptions

- The default and only initial player count is two seats, `seat_0` and `seat_1`.
- The display name **Crest Ledger** is acceptable; a maintainer may replace it with another original, casino-neutral name before implementation without changing the internal id.
- The standard variant uses six cards, three ranks, two copies per rank, one private card per seat, one center card, two pledge rounds, round units `[1, 2]`, and one lift cap per round.
- A yield terminal does not reveal private cards; only showdown reveals both private cards.
- The Level 2 bot is an authored heuristic, not a solver, and it must never enumerate or sample hidden opponent states.
- Variant constants are Rust-owned typed behavior; data files may document/mirror labels but cannot own formulas.
- The repository may accept ADR 0005 before implementation; until then, implementation must not report ADR 0005 as accepted even if it follows variance-aware benchmark practice.
- `plain_tricks` is treated as Gate 10.1 — a proposed sub-gate split of ROADMAP's single Gate 10, not a ROADMAP-stated label — unless maintainers explicitly change sequencing in a future spec/ADR.
- The resolved tool-scope set (§4) is load-bearing: `simulate`/`replay-check`/`fixture-check`/`rule-coverage` require a `poker_lite` arm; `bench-report` is optional; `seed-reducer`/`trace-viewer` are not expected to need one. Re-confirm at implementation time only if a tool registry has changed.

---

# Implementation reference — proposed **Crest Ledger** rules and seams

This section is non-canonical appendix material in the style of the Gate 9 sibling spec. It is intentionally concrete so that variant scope is written before coding.

## A. Core rules

### A1. Components

- **Seats:** `seat_0`, `seat_1`.
- **Deck:** six `CrestCardId`s: ranks `low`, `middle`, `high`, two copies each. Suggested neutral display labels:
  - rank 1: `Sprout`
  - rank 2: `Current`
  - rank 3: `Crown`
  - copies: `dawn`, `dusk`
- **Private crests:** one card dealt to each seat at setup.
- **Center crest:** one card dealt face down at setup; revealed to all only after pledge round 1 closes without yield.
- **Deck tail:** remaining three cards, never public and never needed for legal decisions after setup.
- **Opening contribution:** each seat contributes 1 marker to the shared pool at setup.
- **Round units:** round 1 unit = 1 marker; round 2 unit = 2 markers.
- **Lift cap:** at most one lift per round.
- **Maximum contribution:** deterministic bound of 7 markers per seat: opening 1 + round 1 max 2 + round 2 max 4.
- **Terminal outcomes:** `YieldWin { winner, loser, shared_pool, contributions }`, `ShowdownWin { winner, shared_pool, contributions, reveal }`, or `Split { shared_pool, each, contributions, reveal }`.

### A2. Setup

1. Construct the six-card deck in stable id order.
2. Shuffle with the same seeded deterministic RNG discipline already used by `high_card_duel`’s local shuffle helper.
3. Deal top card to `seat_0`, next to `seat_1`, next to `center_card`, and leave the deck tail internal.
4. Set `phase = PledgeRound { round_index: 0 }`.
5. Set `active_seat = seat_0`.
6. Set `contributions = [1, 1]`, `shared_pool = 2`, and `round_state` to no outstanding pledge.
7. Emit private effects for each seat’s private card and a public effect saying private crests were dealt and the shared pool opened with two markers. Public effect payload contains counts only, not ids/ranks.

### A3. Action families

Use neutral public labels; internal paths may stay neutral too:

| Segment | Public label | Legal when | Accounting |
| --- | --- | --- | --- |
| `hold` | Hold | No outstanding pledge in current round | Adds 0. If both seats hold in sequence, close round. |
| `press` | Press | No outstanding pledge and actor has not already pressed in the closed state | Actor adds current round unit; opponent faces outstanding pledge. |
| `lift` | Lift | Actor faces outstanding pledge, round lift cap unused | Actor adds amount needed to match plus one current round unit; opponent faces outstanding pledge; lift cap consumed. |
| `match` | Match | Actor faces outstanding pledge | Actor adds amount needed to equalize current-round contribution; close round. |
| `yield` | Yield | Actor faces outstanding pledge | Terminal; opponent wins current shared pool; no showdown reveal. |

All legal action choices must carry only public metadata: `action_family`, `round_index`, `round_unit`, `actor_seat`, `required_to_match`, `adds_to_pool`, `shared_pool_after`, `lift_cap_remaining`, and safe accessibility copy. No metadata includes card id, rank, hidden center status beyond `center_visible: true/false`, opponent strength, bot ranking, or inferred hidden state.

### A4. Round close

A pledge round closes when:

- both seats hold with no outstanding pledge; or
- a facing seat matches the outstanding pledge after a press/lift; or
- yield ends the match immediately.

On round 1 close without yield:

1. Reveal the center crest in a public grouped effect.
2. Set `center_visible = true`.
3. Advance to round 2.
4. Set `active_seat = seat_1` so the second round lead alternates deterministically.
5. Reset per-round pledge state and lift cap.

On round 2 close without yield:

1. Emit `showdown_reveal_started` public effect.
2. Reveal both private crests in one public `showdown_revealed` effect.
3. Resolve comparator and terminal allocation.
4. Emit `ledger_resolved` and `terminal` effects.

### A5. Showdown comparator

For each seat:

```text
pair_flag = private_card.rank == center_card.rank
strength = (pair_flag ? 1 : 0, private_card.rank_value)
```

Compare `strength` lexicographically:

1. pair beats no pair;
2. among pair hands, higher private rank wins;
3. among no-pair hands, higher private rank wins;
4. equal strength splits the shared pool exactly.

Split determinism:

- Showdown contributions are equal by rule, so the shared pool is even.
- `each = shared_pool / 2`.
- Terminal outcome is `Split`, not a hidden seat-priority win.

Yield determinism:

- Non-yielding seat wins the current shared pool.
- No hidden card reveal occurs.
- Terminal effect may say `seat_0 yielded to seat_1` or equivalent neutral copy, but not why, not inferred strength, and not folded-card identity.

## B. State and visibility model

### B1. Internal state

Suggested internal shape:

```rust
pub struct PokerLiteState {
    pub seats: [SeatId; 2],
    pub phase: Phase,
    pub active_seat: Option<PokerLiteSeat>,
    pub private_cards: [CrestCardId; 2],
    pub center_card: CrestCardId,
    pub center_visible: bool,
    pub deck_tail: Vec<CrestCardId>,
    pub contributions: [u8; 2],
    pub shared_pool: u8,
    pub round: PledgeRoundState,
    pub terminal_outcome: Option<TerminalOutcome>,
    pub freshness_token: FreshnessToken,
}
```

The exact Rust type can differ, but the internal/public split cannot. `private_cards`, hidden `center_card`, and `deck_tail` never appear in public projection before their reveal points.

### B2. Public/seat view

Public observer before center reveal:

- display name, game id, variant id;
- phase and active seat;
- shared pool and per-seat contributions;
- round index, round unit, outstanding amount, lift-cap public state;
- private card counts per seat: `1` / `1`;
- center card status: `hidden`;
- terminal status: nonterminal;
- safe UI copy.

Seat view before showdown:

- all observer fields;
- own private card id/rank/label;
- own safe strength bucket from own view only, if provided (`low_private`, `middle_private`, `high_private`, or after center reveal `paired_high`, etc.); this field is private to owner and must never appear in observer/opponent views.

Public observer after center reveal but before showdown:

- center card id/rank/label;
- no private card identities;
- deck tail remains hidden.

Showdown public view:

- center card;
- both private cards;
- comparator result;
- shared-pool allocation;
- terminal outcome.

Yield terminal public view:

- winner/loser and shared-pool allocation;
- center shown only if it had already been revealed;
- private cards remain hidden except owner’s own private view may still show its own card in seat-local view; public export remains redacted.

### B3. Effects

Suggested effects:

- `private_crest_dealt` — `PrivateToSeat`, includes own card.
- `crest_deal_started` — public, counts only.
- `opening_pool_set` — public, contributions and shared pool.
- `pledge_held` — public.
- `pledge_pressed` — public, actor, amount, pool after.
- `pledge_lifted` — public, actor, amount, pool after, lift cap consumed.
- `pledge_matched` — public, actor, amount, pool after.
- `seat_yielded` — public, actor yielded, winner, pool, no private card.
- `center_reveal_started` — public grouped reveal marker.
- `center_revealed` — public, center card.
- `showdown_reveal_started` — public grouped reveal marker.
- `showdown_revealed` — public, both private cards and center card.
- `ledger_resolved` — public, allocation summary.
- `terminal` — public, terminal outcome.
- `bot_chose_action` — public or private-to-actor depending on explanation content; public version must include policy id and selected action family only, never private card. A private actor explanation may include the bot’s own private strength bucket but not the opponent card.

## C. Bot policy

### C1. Level 0 random-legal

- Reuse the current random-legal pattern: collect legal leaf paths from the Rust action tree, select deterministically from bot seed, validate/apply normally.
- No direct state mutation, no illegal fallback. If no legal action exists, return diagnostic.

### C2. Level 2 authored policy

Policy id: `poker-lite-crest-ledger-level2-v1`.

Allowed input:

```text
own seat
legal action tree for own seat
own private card
center card only if public
round index / round unit
shared pool
own contribution
opponent public contribution
outstanding amount to match
lift cap remaining
public action history families and amounts
terminal flag
```

Forbidden input:

```text
opponent private card
hidden center card before reveal
deck tail
seed/reconstructed shuffle
full internal trace
public replay hidden payloads
opponent bot private explanation
sampled or enumerated opponent holdings
```

Heuristic priorities, evaluated against legal actions only:

1. **Survive legality:** never choose outside the Rust legal tree.
2. **Protect strong made pair:** after center reveal, if own card pairs center and a press/lift is legal, prefer one controlled pressure action unless already facing a large outstanding amount; otherwise match.
3. **Respect public price:** facing an outstanding amount, match when required amount is cheap relative to shared pool and own strength bucket is medium-or-better by own view; yield low/no-pair hands when the price is high and no reveal obligation has already made showdown attractive.
4. **Use high private rank before center:** before center reveal, press high rank more often, hold middle/low rank unless matching is cheap; lift only high rank and only if cap unused.
5. **Avoid reckless lift:** never lift with low private rank before center; never lift no-pair low after center.
6. **Close when uncertain:** prefer match over lift when a legal match closes the round and strength is medium.
7. **Stable tie-break:** if multiple actions share priority, choose by stable action family order `match`, `hold`, `press`, `lift`, `yield` adjusted by context, then stable path string. Optional bot seed may break only documented equal-priority ties without hidden sampling.

Viewer-safe explanation examples:

- Public effect: `Level 2 selected Match from the legal pledge options.`
- Private actor panel: `Matched because your visible strength bucket was paired and the required marker count was within policy threshold.`
- Never: `Opponent likely has Sprout-Dawn` or `deck enumeration says...`.

## D. Replay/export model

- Internal full trace may include seed, private deal, hidden center, and deck tail for tests and deterministic replay.
- Browser public export must follow ADR 0004 viewer-scoped replay taxonomy: it includes public timeline, redacted commands, public effects, public terminal outcome, and enough public summaries to step the replay without reconstructing hidden cards.
- Public export for pre-showdown or yield terminal must not include original seed if that seed plus public commands would reconstruct private cards.
- Seat-scoped export may include that seat’s own private observations at the times they were visible to that seat, but never opponent hidden cards before showdown and never deck tail.
- Import of public export replays the public timeline, not the hidden internal game.
- Replay-check golden traces should test both internal deterministic replay and public export/import redaction.

## E. WASM/browser specifics

WASM additions:

- constants: `GAME_POKER_LITE`, display name **Crest Ledger**, `VARIANT_POKER_LITE_STANDARD`, trace rules version `poker-lite-rules-v1`;
- `RegisteredGame::PokerLite`;
- `MatchRecord::PokerLite { game_id, seed, state, effects, commands }`;
- list-games entry with hidden-information tags: `hidden_info`, `viewer_filtered`, `public_replay_export`, `public_accounting`, `bounded_pledge`;
- new-match setup with two seats;
- view/action/effect serializers;
- `get_action_tree_for_viewer` authorization: actor’s tree only for actor’s viewer; observer/opponent get empty tree;
- bot turn branch with Level 0/Level 2 policy selection according to existing shell mode conventions;
- replay export/import branch with public timeline redaction.

Browser additions:

- `PokerLiteBoard.tsx` renders:
  - neutral board-game surface, not casino felt/table;
  - shared pool and contribution ledger;
  - center crest hidden/revealed state;
  - own private crest only in seat view;
  - opponent/private unknown as count/status only;
  - showdown grouped reveal;
  - yield terminal without private reveal;
  - legal action buttons from Rust action tree only.
- `ActionControls.tsx`: for `poker_lite`, use no-leak-safe test ids such as `choice-poker-lite-round-${round}-${index}`, not raw segments if future segments ever include private ids.
- `effectFeedback.ts`: add neutral copy for pledge and reveal effects. Avoid words such as casino, chip, payout, table, poker, ante, blind, rake, or insurance in public UI copy. The internal docs may use “betting/pot” only as research/mechanic descriptors.
- Dev panel: display public API version, selected game, viewer mode, public view summaries, action count, effect cursor; never raw state, seed-derived hidden deck, private cards, bot private rankings, or raw replay hidden fields.
- E2E no-leak tests must check DOM text, accessibility names, `data-testid`, local storage, replay export text, and dev panel content for hidden ids before reveal.

## F. Benchmark operations

Native benchmark operations:

- setup + shuffle + deal;
- legal action tree generation for each phase;
- validate/apply for hold, press, lift, match, yield, center reveal, showdown;
- observer projection;
- seat projection;
- public export/import;
- full random-legal simulation to terminal;
- Level 2 bot simulation to terminal.

Initial performance floor: `poker_lite` should meet or exceed the testing guide’s provisional `2,000+ hands/sec` native playout target. Since the game is small, failures against this floor should be treated as implementation-quality issues unless calibrated CI evidence proves the floor is inappropriate.

## G. Sources note to copy into game docs

The game docs should state that **Crest Ledger** is an original Rulepath microgame. Kuhn poker and Leduc poker were consulted only as public research-minimal structures for small imperfect-information betting games, while OpenSpiel informed vocabulary around information states and observations. No public rules prose, hand-ranking table, casino imagery, product naming, or trade dress is copied.

[^kuhn]: H. W. Kuhn, “A Simplified Two-Person Poker,” 1951. Consulted from the Rutgers-hosted scan for the minimal two-player/private-card/ante/bounded-bet shape. https://sites.math.rutgers.edu/~zeilberg/akherim/PokerPapers/Kuhn1951.pdf
[^openSpielGames]: OpenSpiel, “Available games.” Consulted for the classification of Kuhn poker and Leduc poker as simplified imperfect-information benchmark games. https://openspiel.readthedocs.io/en/latest/games.html
[^openSpielPaper]: Lanctot et al., “OpenSpiel: A Framework for Reinforcement Learning in Games,” 2019. Consulted for the information-state / observation framing and the distinction between perfect and imperfect information games. https://arxiv.org/pdf/1908.09453
[^openSpielDocs]: OpenSpiel documentation PDF. Consulted for the observation/information-state model and sequential/simultaneous game support. https://openspiel.readthedocs.io/_/downloads/en/latest/pdf/
[^leduc]: Kroer and Sandholm, “An Algorithm for Constructing and Solving Imperfect Recall Abstractions of Large Extensive-Form Games,” IJCAI-17, §5. Consulted only for the compact Leduc-style pattern: three ranks/two copies, one private card, one public card, two bounded betting rounds, limited raises, pair-before-high-card comparison. https://www.ijcai.org/proceedings/2017/0130.pdf

# Gate 9 Token Bazaar Browser Proof

| Field | Value |
|---|---|
| Spec ID | `gate-9-token-bazaar-browser-proof` |
| Roadmap stage | 7 |
| Roadmap build gate | Gate 9 — resources / economy. This spec implements the `token_bazaar` (public resource/economy) slice; the `secret_draft` simultaneous-commitment / reveal slice is deferred to a dedicated successor gate (see Sequencing). |
| Status | Done |
| Date | 2026-06-08 |
| Owner | Rulepath maintainers |
| Primary crate / internal game id | `token_bazaar` |
| Public display name | `Token Bazaar` unless implementation finds a clearly better neutral name |
| Browser implementation | required |
| Authority order | [`docs/FOUNDATIONS.md`](../docs/FOUNDATIONS.md) → [`docs/ARCHITECTURE.md`](../docs/ARCHITECTURE.md) → [`docs/ENGINE-GAME-DATA-BOUNDARY.md`](../docs/ENGINE-GAME-DATA-BOUNDARY.md) → [`docs/OFFICIAL-GAME-CONTRACT.md`](../docs/OFFICIAL-GAME-CONTRACT.md) → [`docs/ROADMAP.md`](../docs/ROADMAP.md) → this spec. Where this spec and a foundation document disagree, the foundation document wins. |

> Reader orientation: this spec carries the canonical Rulepath section set
> (Objective, Scope, Deliverables, Work breakdown, Exit criteria, Acceptance
> evidence, FOUNDATIONS & boundary alignment, Forbidden changes, Documentation
> updates required, Sequencing, Assumptions). The detailed design material —
> product intent, full proposed rules, effect shapes, action-tree metadata, bot
> policy, WASM/browser wiring, fixtures, and benchmarks — is preserved verbatim
> below the canonical sections under **Implementation reference**.

## Objective

Implement `token_bazaar` as the Gate 9 browser proof: a small, original,
deterministic, public-resource economy game for two players (ROADMAP §11, stage
7). It proves resource effects, payments, gains, scoring economy, visible market
state, deterministic contract fulfillment/refill, supply exhaustion, a fixed turn
cap, Rust-owned action-tree payloads, auditable replay/effect accounting, a
readable browser economy UI, and a competent Level 1 heuristic bot.

Gate 8 (High Card Duel) already proved hidden information, deterministic chance
setup, private views, public redaction, bot-view discipline, and browser no-leak
patterns. Gate 9 broadens Rulepath toward public resource accounting so it is
less likely to become accidentally card-game-shaped. The `secret_draft`
simultaneous-commitment / reveal half of ROADMAP §11 is **deliberately deferred**
to a dedicated successor gate (see Sequencing and Exit criteria); bundling it
here would turn one implementation session into two gates and muddle the
acceptance evidence. This is not a research-platform gate, an economy engine, or
a general resource DSL. The full product intent and proof goals are preserved
under **Implementation reference → Product intent / What the gate proves**.

## Scope

### In scope

- `games/token_bazaar`: typed Rust rules, state, setup, actions, effects,
  visibility, variants, replay support, bots, UI projection, and IDs.
- `token_bazaar_standard` variant: public supply, per-player inventories, a
  three-slot market over a deterministic ten-contract queue, collect / exchange /
  fulfill / forced-pass actions, an 8-turn-per-seat cap, terminal + tie-break rules.
- Game-local resource nouns (`amber`, `jade`, `iron`) confined to
  `games/token_bazaar` and its docs/tests/UI.
- Level 0 random-legal bot and a default Level 1 `TokenBazaarLevel1Bot`.
- The full official-game evidence set (rules/property/replay/serialization/
  visibility/bot tests, golden traces, fixtures, rule coverage, benchmarks, docs).
- WASM bridge + React presentation wiring through the existing shell seams.
- Tool and CI registration by game id.

### Out of scope

- `secret_draft` / simultaneous commitment, hidden choices, reveal phases, and
  pending-seat waiting UX — deferred to the successor commitment/reveal gate.
- Random setup, random refill, or random contract order (see RNG decision).
- Any generic `game-stdlib` resource/market/contract/economy primitive, and any
  `engine-core` economy noun.

### Not allowed

Carried from ROADMAP §11 "Not allowed", plus gate-local prohibitions (full list
under **Forbidden changes**):

- static data formulas for payments/costs (no behavior-in-data);
- hidden choices in DOM / local storage (N/A here — all state is public — but the
  no-leak harness still asserts no debug/candidate data leaks);
- actual hidden-state sampling by bots; no MCTS/ISMCTS/Monte Carlo/ML/RL.

## Deliverables

Concrete artifact tree (full enumeration preserved under **Implementation
reference → Rust crate and data areas likely affected**, **Required game docs**,
and **WASM and browser integration requirements**):

- `games/token_bazaar/{Cargo.toml, src/*.rs, data/*, benches/*, tests/*, docs/*}`
  following the `games/high_card_duel` layout (verified to match file-for-file).
- Twelve golden traces under `games/token_bazaar/tests/golden_traces/`.
- Eleven game docs under `games/token_bazaar/docs/` from `templates/*`.
- WASM/browser: `crates/wasm-api`, `apps/web/src/...`, a `TokenBazaarBoard.tsx`,
  and `apps/web/e2e/token-bazaar.smoke.mjs`.
- Workspace + tooling + CI registration: root `Cargo.toml`, `tools/*`,
  `.github/workflows/gate-1-game-smoke.yml`,
  `.github/workflows/gate-2-benchmarks.yml`, `apps/web/package.json`.

## Work breakdown

Bounded candidate AGENT-TASKs, in dependency order (decompose from
`templates/AGENT-TASK.md`):

| # | Item | Depends on |
|---|---|---|
| 1 | Crate skeleton + workspace registration (`Cargo.toml`, `ids.rs`, `state.rs`, `setup.rs`) | — |
| 2 | Actions, rules, legality, effects, terminal + tie-breaks (`actions.rs`, `rules.rs`, `effects.rs`) | 1 |
| 3 | Visibility / public-view projection + variants + replay support | 2 |
| 4 | Static data (`data/manifest.toml`, `data/variants.toml`, standard fixture) | 1 |
| 5 | Level 0 + Level 1 bots (`bots.rs`) with public rationale | 2 |
| 6 | Rust test suite + golden traces + rule coverage | 2,3,4,5 |
| 7 | Benchmarks (`benches/*`, `thresholds.json`) | 2 |
| 8 | Game docs from templates | 6 |
| 9 | WASM bridge + tool/CI registration by game id | 6 |
| 10 | React board + effect log + e2e smoke + a11y/no-leak | 9 |
| 11 | MECHANIC-ATLAS first-use note + index/ROADMAP-status reconciliation | 6 |

## Exit criteria

Mapped row-for-row to ROADMAP §11 (Gate 9). Rows the `secret_draft` deferral
descopes are carried explicitly to the successor gate rather than dropped.

| ROADMAP §11 line | Disposition in this gate |
|---|---|
| resource accounting is effect-visible | **Met** — every gain/payment/exchange/refill/score change is a structured replayable effect. |
| costs/previews come from Rust | **Met** — action-tree metadata (cost/gain/points/slot) is Rust-owned; TS computes no legality/affordability. |
| simultaneous choices remain hidden until reveal | **Deferred** — no simultaneous choice in Token Bazaar; carried to the successor commitment/reveal gate (`secret_draft`). |
| UI shows pending seats without leaking choices | **Deferred** — no pending/waiting state here; carried to the successor gate. |
| bots use allowed views | **Met** — bots route through the normal legal-action API; all state is public; rationale exposes no debug/candidate tables. |
| invariant/no-leak tests and benchmarks pass | **Met** — property/visibility/replay/serialization tests, no-leak/a11y e2e, and benchmark floors. |
| Not allowed: static data formulas for payments | **Honored** — costs/points are typed setup/scoring constants; no formulas/selectors in data. |
| Not allowed: hidden choices in DOM/local storage | **N/A (asserted)** — fully public game; no-leak test still asserts no debug/candidate leakage. |
| Not allowed: hidden-state sampling by bots | **Honored** — no sampling/search bot; Level 1 is a deterministic heuristic. |

## Acceptance evidence

The detailed acceptance checklist is preserved under **Implementation reference →
Acceptance criteria**; it must cover the `docs/OFFICIAL-GAME-CONTRACT.md`
deliverable set:

- rule/property/replay/serialization/visibility/bot tests pass (`cargo test -p token_bazaar`);
- golden traces for normal / terminal / refill / exchange / exhaustion /
  invalid-diagnostic / stale / bot / WASM-export cases;
- replay reproduces final state, effects, action-tree + public-view hashes, and outcome;
- `simulate`, `replay-check`, `fixture-check`, `rule-coverage` pass for `token_bazaar`;
- benchmarks run with documented smoke floors + a named calibration follow-up;
- browser e2e smoke (human + bot action, replay step / export-import, dev panel,
  reduced-motion, a11y/no-leak checklist);
- `bash scripts/boundary-check.sh` and `node scripts/check-doc-links.mjs` pass.

## FOUNDATIONS & boundary alignment

| Principle | Stance | Rationale |
|---|---|---|
| §2 Behavior authority | aligned | Rust owns setup, legality, effects, refill, terminal, tie-breaks, bot decisions; TS presents Rust view/effect payloads only. |
| §3 `engine-core` kernel | aligned | Resource/market/contract/supply nouns stay in `games/token_bazaar`; Forbidden changes bars them from `engine-core`. |
| §4 `game-stdlib` earned | aligned | First official resource/accounting use → local; `docs/MECHANIC-ATLAS.md` records it as a later candidate, with no open promotion debt. |
| §5 Static data is typed content | aligned | Contract queue, costs, points are typed setup/scoring constants; no formulas/selectors/triggers; unknown fields rejected. |
| §8 Public bots | aligned | Level 1 heuristic with public rationale; no MCTS/ISMCTS/Monte Carlo/ML/RL; legal-action API only. |
| §11 Acceptance invariants | aligned | Deterministic replay/hash/serialization; viewer-safe public views; no-leak asserted even though public; full evidence set. |
| §13 ADR triggers | clear | No replay/hash-semantics, visibility-contract, kernel-vocabulary, or new-bot-class change; the RNG decision avoids touching `engine-core::DeterministicRng`. |
| §12 Stop conditions | clear | No kernel nouns, no procedural data, no YAML/DSL, TS decides nothing, no leak path, no bot bypass, no open promotion debt. |

## Forbidden changes

Do not, in this gate (detailed list preserved under **Implementation reference →
Non-goals**):

- add economy nouns (`resource`, `market`, `token`, `contract`, `supply`, `card`,
  `deck`, `hand`, `board`, `grid`, `auction`, `betting`, `pot`) to `engine-core`;
- create generic `game-stdlib` resource/market/contract/economy/bot-policy
  primitives (no `ResourcePool`, `Inventory`, `Market`, `Contract`, `Cost`,
  `Payment`, `Economy`);
- introduce behavior-in-data formulas, a DSL, YAML, selectors, or conditional
  effects;
- let TypeScript compute legality, affordability, refill, winner, terminal
  outcome, or bot policy;
- add MCTS/ISMCTS/Monte Carlo/ML/RL/LLM bots or hidden-state sampling;
- add random setup/refill/contract order, or touch `engine-core::DeterministicRng`;
- implement `secret_draft`, simultaneous hidden commitments, online multiplayer,
  or server authority;
- copy commercial board/card rules, names, prose, assets, icons, or trade dress.

## Documentation updates required

- Flip this spec's row in [`specs/README.md`](README.md) from `Not started` to
  `Planned` (this spec now exists) and, on completion, to `Done`.
- Do **not** edit `docs/ROADMAP.md` to record progress (the index tracks status).
- Add the Token Bazaar first-use note to `docs/MECHANIC-ATLAS.md` (resource/
  accounting remains local first-use pressure).
- Update `progress.md` and root `README.md` after implementation.
- Author all eleven `games/token_bazaar/docs/*` from `templates/*`.

## Sequencing

- **Predecessor:** Gate 8 (High Card Duel) — `Done` in the `specs/README.md`
  index; its hidden-info / chance / no-leak evidence is the baseline this gate
  builds on. Admission rule satisfied: no open promotion debt blocks Gate 9
  (`docs/MECHANIC-ATLAS.md` records none).
- **Successor — `secret_draft` commitment/reveal gate.** The two deferred
  ROADMAP §11 exit lines (simultaneous choices hidden until reveal; UI shows
  pending seats without leaking choices) are carried to a dedicated focused
  simultaneous-commitment / waiting / reveal proof. **Recommended placement:
  Gate 9.1, sequenced immediately after this gate and before Gate 10 (betting /
  `poker_lite`)** — poker_lite's imperfect-information and waiting UX depend on
  the simultaneous-choice / private-view proof landing first, so the
  commitment/reveal gate should not slip past it. Authoring that Gate 9.1 spec
  and adding its index row is a follow-up, out of scope for this spec.

## Assumptions

- A1: `games/token_bazaar` follows the `games/high_card_duel` file layout —
  verified to match file-for-file at reassessment time (src set, `data/`,
  `benches/`, `tests/`, `docs/`, `golden_traces/`).
- A2: Tools enumerate games by a hardcoded game-id const + dispatch branch
  (e.g. `tools/simulate/src/main.rs` `GAME_HIGH_CARD_DUEL` + `run_*` branch);
  `token_bazaar` registration is real code in each tool, not config.
- A3: `docs/MECHANIC-ATLAS.md` shows no open promotion debt; Gate 9 may proceed
  without a back-port interlock.
- A4: Public display name `Token Bazaar` and contract labels are original
  placeholders; `SOURCES.md` must record the IP/originality review.
- A5: The `secret_draft` deferral is confirmed; its exit criteria move to the
  successor gate per Sequencing.

---

# Implementation reference

The sections below are the detailed design, preserved from the original spec.

## Product intent

`Token Bazaar` is a small, original, deterministic, public-resource economy game for two players. It proves that Rulepath can handle:

- public resource pools;
- per-player inventories;
- resource costs and payments;
- visible market slots;
- deterministic contract fulfillment and slot refill;
- supply exhaustion and legality changes;
- score pressure with a fixed turn cap;
- Rust-owned action-tree payloads carrying resource and slot choices;
- auditable replay/effect logs for accounting transitions;
- a pleasant browser UI that makes resources, costs, and market state obvious;
- a Level 1 heuristic bot that is competent enough to demonstrate the game and is not merely random legal.

This gate should make Rulepath less likely to become accidentally card-game-shaped. It is not a research-platform gate, an economy engine, or a general resource DSL.

## What the gate proves

Gate 9 acceptance should demonstrate these new pressures beyond prior games:

1. **Public accounting correctness.** Every resource gain, payment, exchange, supply return, and score change is deterministic, effect-visible, and replayable.
2. **Market-slot state.** The game maintains a visible market row, a deterministic queue, empty slots after exhaustion, and contract refill effects.
3. **Action-tree payload discipline.** Legal choices and previews come from Rust, including costs, bundle choices, exchange choices, and market-slot choices. TypeScript never computes legality.
4. **Browser economy readability.** The shell can render inventory chips, shared supply, contract cards, costs, rewards, score, turn cap, legal options, and recent accounting effects without hidden rule logic.
5. **Level 1 bot quality.** The bot prefers good public moves with stable tie-breakers and a public rationale. Random legal may remain as a Level 0 fallback/helper, but it must not be the only bot evidence.
6. **No generic primitive promotion.** Resource logic stays in `games/token_bazaar` unless a separate accepted primitive-pressure ledger later proves repeated behavior-free pressure.

## Non-goals

Do not implement any of the following in Gate 9:

- poker;
- blackjack;
- betting, pots, raises, real-money language, or casino framing;
- a general card engine;
- a general resource/accounting engine;
- generic `game-stdlib` resource, market, contract, economy, or bot-policy primitives;
- `engine-core` nouns such as resource, market, token, contract, supply, card, deck, hand, board, grid, auction, betting, or pot;
- a DSL or behavior-in-data system;
- MCTS, ISMCTS, Monte Carlo playout policy, ML, RL, LLM-assisted bots, or hidden-state sampling;
- simultaneous hidden commitments;
- online multiplayer;
- server authority;
- animations or art polish that delay rule/replay/UI correctness;
- copying commercial board/card game rules, component names, prose, assets, icons, traces, or trade dress.

## Proposed game rules

These rules are intentionally small and original. Implement them exactly unless implementation finds a direct contradiction in the current repository.

### Players and visibility

- Two seats: `seat_0` and `seat_1`.
- All game state is public.
- A viewer may be an observer or a seat viewer; all see the same resources, market, scores, turn count, and public effects.
- The implementation still uses the repository's normal viewer/effect/replay boundaries so future hidden-state work is not regressed.

### Resources

Use three game-local resource types:

- `amber`
- `jade`
- `iron`

These names are game nouns. They belong only inside `games/token_bazaar`, its docs, its tests, and its UI projection. They must not enter `engine-core` or `game-stdlib`.

Initial public supply:

| Resource | Supply |
|---|---:|
| amber | 14 |
| jade | 14 |
| iron | 14 |

Initial inventory per player:

| Resource | Inventory |
|---|---:|
| amber | 1 |
| jade | 1 |
| iron | 1 |

Initial score per player: `0`.

### Market contracts

The market has three visible slots: `slot_0`, `slot_1`, and `slot_2`.

A deterministic queue of ten contracts exists at setup. The first three contracts fill the visible slots in order. When a contract is fulfilled, the empty slot refills immediately from the front of the queue. If the queue is empty, the slot remains empty.

Use this standard contract queue for the first implementation:

| Contract id | Display label | Cost | Points |
|---|---|---|---:|
| `balanced-wares` | Balanced Wares | 1 amber, 1 jade, 1 iron | 3 |
| `amber-guild` | Amber Guild | 2 amber, 1 jade | 3 |
| `iron-guild` | Iron Guild | 2 iron, 1 amber | 3 |
| `jade-guild` | Jade Guild | 2 jade, 1 iron | 3 |
| `amber-focus` | Amber Focus | 3 amber | 4 |
| `jade-focus` | Jade Focus | 3 jade | 4 |
| `iron-focus` | Iron Focus | 3 iron | 4 |
| `sun-route` | Sun Route | 2 amber, 2 jade | 5 |
| `stone-route` | Stone Route | 2 jade, 2 iron | 5 |
| `crown-route` | Crown Route | 2 iron, 2 amber | 5 |

The labels are original placeholder text. They may be renamed for clarity, but do not import proprietary names or rules.

### Turn structure

- `seat_0` starts.
- Active seat alternates after every applied action.
- Each seat may take at most 8 turns.
- The game ends after both seats have taken 8 turns, or immediately after the last market contract is fulfilled and no visible slots remain.
- A terminal state exposes no normal gameplay actions.

### Legal actions

On a non-terminal active turn, exactly one of these action families is chosen.

#### 1. Collect

Collect takes tokens from the public supply into the active player's inventory. A collect action is legal only if the supply can satisfy the entire bundle.

Stable path shape:

```text
collect/<bundle-id>
```

Implementation may represent it as one slash-bearing segment or as a nested action path, but the exported action tree must be stable across replay and WASM. Prefer nested paths if that is cleaner for the existing action-tree renderer:

```text
["collect", "amber-jade"]
```

Allowed bundles:

| Bundle id | Gain |
|---|---|
| `amber` | 2 amber |
| `jade` | 2 jade |
| `iron` | 2 iron |
| `amber-jade` | 1 amber, 1 jade |
| `jade-iron` | 1 jade, 1 iron |
| `iron-amber` | 1 iron, 1 amber |

If a resource supply is exhausted, any bundle requiring that resource becomes illegal until payments/exchanges return that resource to supply.

#### 2. Exchange

Exchange converts two matching resources from the active player's inventory into one different resource from supply.

Stable path shape:

```text
exchange/<pay-resource>/<take-resource>
```

Legality:

- `pay-resource != take-resource`;
- active player has at least 2 of `pay-resource`;
- public supply has at least 1 of `take-resource`.

Effect:

- active player pays 2 `pay-resource` back to supply;
- active player takes 1 `take-resource` from supply.

This action is intentionally inefficient. It exists to prove conversion legality, supply return, and bot valuation pressure, not to be a dominant strategy.

#### 3. Fulfill contract

Fulfill pays the exact cost of a visible contract and scores its points.

Stable path shape:

```text
fulfill/<slot-id>
```

Legality:

- slot is visible and occupied;
- active player has all required resources.

Effect:

- active player pays contract resources to public supply;
- active player's score increases by the contract's points;
- contract id is added to the active player's fulfilled-contract list;
- market slot refills from the queue if possible;
- terminal condition is checked.

#### 4. Forced pass

A `pass` action is legal only if no collect, exchange, or fulfill action is legal. It should almost never appear in normal play, but it prevents pathological no-action states after supply exhaustion. It must be tested if present.

### Winner and tie-breaks

At terminal:

1. Higher score wins.
2. If tied, more fulfilled contracts wins.
3. If still tied, higher total remaining inventory wins.
4. If still tied, the game is a draw.

Tie-breaks are public and deterministic. They must be documented in `RULES.md` and covered by rule tests or golden traces.

### Effects

All effects are public. Effect names may follow repository style, but they must carry enough accounting data for replay and UI audit.

Required semantic effect shapes:

- resource collection: seat, bundle, resource deltas, inventory after, supply after;
- resource exchange: seat, paid resources, taken resource, inventory after, supply after;
- contract fulfilled: seat, slot, contract id, cost, points, score after;
- market slot refilled or exhausted: slot, new contract id or empty state, remaining queue length;
- turn advanced: next active seat, turn count per seat;
- terminal outcome: winner/draw, scores, tie-break data.

Do not reduce accounting effects to prose-only strings. UI prose can be derived from Rust-provided effect payloads.

### Action-tree previews and metadata

Legal action choices must include stable labels, accessibility labels, and Rust-provided metadata sufficient for the browser to present costs and consequences without computing rules.

Recommended metadata keys:

- `family`: `collect`, `exchange`, `fulfill`, or `pass`;
- `resource_delta`: compact stable string or structured JSON string owned by the game projection;
- `cost`: compact stable string for fulfill/exchange;
- `gain`: compact stable string for collect/exchange;
- `slot_id` and `contract_id` for fulfill;
- `points` for fulfill;
- `blocked_reason` only if the existing action-tree pattern supports unavailable preview nodes. Otherwise omit illegal choices entirely.

Do not expose hidden or debug-only valuation data through action metadata. This game is public, but it must not teach the web shell to rely on debug payloads.

## RNG decision

Gate 9 should be deterministic and public-resource-only. Do **not** add random setup, random market refill, or random contract order in this gate.

The repository currently has deterministic RNG contracts in `engine-core`, and High Card Duel carries a game-local unbiased bounded-index helper for its shuffle proof. That is not enough pressure to promote a new `engine-core` RNG helper during Token Bazaar.

Decision:

- Keep Token Bazaar setup deterministic.
- Do not add a noun-free unbiased bounded-index helper to `engine-core` in Gate 9.
- Do not touch `engine-core::DeterministicRng::next_index` unless a separate accepted RNG-cleanup spec or ADR explicitly scopes the replay/hash/compatibility impact.
- If a future gate needs random public market setup, handle it in that future game/spec with replay-safe seed evidence and unbiased sampling tests before considering promotion.

## Generic-promotion decision

No generic `game-stdlib` resource primitive is authorized.

Resource/accounting logic is first official public-economy pressure. The atlas already says resource accounting becomes a repeated-shape candidate only after later economy/betting pressure. Therefore:

- implement resource types, supply, inventory, payment, exchange, contracts, and market refill locally in `games/token_bazaar`;
- update the game `MECHANICS.md` inventory and repository mechanic atlas with the first-use evidence;
- do not create `ResourcePool`, `Inventory`, `Market`, `Contract`, `Cost`, `Payment`, `Economy`, or similar shared helpers in `game-stdlib`;
- do not add economy nouns to `engine-core`;
- do not create behavior-in-data formulas for costs or payments.

If the implementer believes a tiny helper is unavoidable, stop and write a primitive-pressure ledger entry first. The expected answer for Gate 9 is still local implementation.

## Level 1 bot requirements

Ship both of these if the repository pattern makes it cheap:

- Level 0 random legal bot for baseline compatibility and simulation coverage;
- Level 1 `TokenBazaarLevel1Bot` as the default browser bot.

Level 1 policy must be deterministic for a given public state and seed. It may use seed only as a final tie-breaker if repository precedent requires seeded bot constructors; prefer lexicographic tie-breaks for reproducibility.

Policy priorities:

1. Fulfill the highest-point affordable visible contract. Tie-break by leftmost slot, then by lowest remaining inventory waste after payment, then by stable contract id.
2. If no contract is affordable, evaluate visible contracts by near-term value: points, total deficit, rarest required resource in public supply, and whether a collect bundle can reduce the deficit immediately.
3. Prefer collect actions that reduce the best target contract's deficit the most.
4. If exchange is the only way to reach a high-value contract and it improves the deficit vector, choose the best exchange.
5. If no meaningful preference exists, choose the first legal collect action in stable bundle order.
6. Use random legal only as a safety fallback after the policy has failed to choose from a non-empty legal tree; such fallback must be visible in tests and should not be reached in normal fixtures.

Bot evidence must show:

- every selected action validates through normal Rust command validation;
- bot decisions are deterministic for fixed state/seed;
- rationale is present and public-safe;
- rationale does not expose internal candidate tables, debug scores, hidden simulation, or omniscient information;
- at least one fixture shows the bot fulfilling a contract rather than merely collecting;
- at least one fixture shows the bot collecting toward a visible contract it cannot yet afford.

## Rust crate and data areas likely affected

Create or update these areas. This list is implementation guidance, not a ticket breakdown.

- `Cargo.toml`: add `games/token_bazaar` to the workspace.
- `games/token_bazaar/Cargo.toml`
- `games/token_bazaar/src/lib.rs`
- `games/token_bazaar/src/ids.rs`
- `games/token_bazaar/src/state.rs`
- `games/token_bazaar/src/setup.rs`
- `games/token_bazaar/src/actions.rs`
- `games/token_bazaar/src/rules.rs`
- `games/token_bazaar/src/effects.rs`
- `games/token_bazaar/src/visibility.rs`
- `games/token_bazaar/src/variants.rs`
- `games/token_bazaar/src/replay_support.rs`
- `games/token_bazaar/src/bots.rs`
- `games/token_bazaar/src/ui.rs`
- `games/token_bazaar/data/manifest.toml`
- `games/token_bazaar/data/variants.toml`
- `games/token_bazaar/data/fixtures/token_bazaar_standard.fixture.json`
- `games/token_bazaar/benches/token_bazaar.rs`
- `games/token_bazaar/benches/thresholds.json`
- `games/token_bazaar/tests/rules.rs`
- `games/token_bazaar/tests/property.rs`
- `games/token_bazaar/tests/replay.rs`
- `games/token_bazaar/tests/serialization.rs`
- `games/token_bazaar/tests/visibility.rs`
- `games/token_bazaar/tests/bots.rs`
- `games/token_bazaar/tests/golden_traces/*.trace.json`

Expected golden traces:

- `shortest-normal.trace.json`
- `terminal-turn-cap.trace.json`
- `contract-fulfill-refill.trace.json`
- `market-exhaustion.trace.json`
- `exchange.trace.json`
- `supply-exhaustion-diagnostic.trace.json`
- `insufficient-resources-diagnostic.trace.json`
- `empty-slot-diagnostic.trace.json`
- `stale-diagnostic.trace.json`
- `non-active-seat-diagnostic.trace.json`
- `bot-action.trace.json`
- `wasm-exported.trace.json`

## Required game docs

Create docs from the existing templates and keep them implementation-specific:

- `games/token_bazaar/docs/RULES.md`
- `games/token_bazaar/docs/MECHANICS.md`
- `games/token_bazaar/docs/RULE-COVERAGE.md`
- `games/token_bazaar/docs/SOURCES.md`
- `games/token_bazaar/docs/AI.md`
- `games/token_bazaar/docs/UI.md`
- `games/token_bazaar/docs/BENCHMARKS.md`
- `games/token_bazaar/docs/GAME-IMPLEMENTATION-ADMISSION.md`
- `games/token_bazaar/docs/PUBLIC-RELEASE-CHECKLIST.md`
- `games/token_bazaar/docs/BOT-STRATEGY-EVIDENCE-PACK.md`
- `games/token_bazaar/docs/COMPETENT-PLAYER.md`

Docs must include:

- the complete rules above;
- resource conservation and supply-return notes;
- market refill semantics;
- terminal and tie-break rules;
- bot level, rationale, and evidence;
- no-leak notes even though all state is public;
- benchmark operations and thresholds;
- source/IP note stating the game is original and only uses public generic mechanism vocabulary;
- mechanic atlas update note showing resource/accounting remains local first-use pressure.

## WASM and browser integration requirements

Wire Token Bazaar through the existing Rust/WASM and React shell seams.

Likely affected areas:

- `crates/wasm-api/Cargo.toml`
- `crates/wasm-api/src/lib.rs`
- `apps/web/src/wasm/client.ts`
- `apps/web/src/components/GamePicker.tsx`
- `apps/web/src/components/AppShell.tsx`
- `apps/web/src/components/ActionControls.tsx`, only if the existing generic action renderer needs metadata presentation support
- `apps/web/src/components/TokenBazaarBoard.tsx`
- `apps/web/src/components/EffectLog.tsx`, only for new public effect labels/icons/text mapping
- `apps/web/src/components/effectFeedback.ts`
- `apps/web/src/state/shellReducer.ts`, only if existing game-specific reducers need a new branch
- `apps/web/src/styles.css`
- `apps/web/e2e/token-bazaar.smoke.mjs`
- `apps/web/package.json`
- `.github/workflows/gate-1-game-smoke.yml`
- `.github/workflows/gate-2-benchmarks.yml`

Browser requirements:

- Game picker lists `Token Bazaar` with `game_id = token_bazaar` and `variant_id = token_bazaar_standard`.
- Match setup can start hotseat, human-vs-bot, and bot-vs-bot modes.
- Main board shows, at minimum:
  - active seat and turn count;
  - both player scores;
  - both public inventories;
  - central public supply counts;
  - visible market slots with contract label, cost chips, point value, and empty-slot state;
  - legal collect/exchange/fulfill controls sourced from Rust action tree;
  - recent resource/market effects;
  - terminal outcome and tie-break explanation.
- Resource state must not rely on color alone. Use text labels, counts, shapes, or icons plus text.
- Dense market/inventory controls must be keyboard reachable. A simple list of buttons is acceptable; an ARIA grid is optional only if implemented correctly.
- TypeScript may map Rust view/effect payloads to presentational components, but it must not compute legality, affordability, terminal outcome, winner, tie-breaks, refill, or bot policy.
- Dev panel and replay import/export must not expose hidden or debug-only data. Since Token Bazaar is public, no redaction is expected, but the test should assert that no internal candidate/rationale debug tables leak.

## Replay, export, and no-leak requirements

Even though Token Bazaar is public, it must satisfy the same determinism discipline as prior games.

Required coverage:

- seed + variant + command stream reproduces final state, effects, action-tree hashes, public view hashes, outcome, and terminal state;
- every accounting effect is stable-serialized and replay-visible;
- invalid commands produce stable diagnostics without mutating state;
- stale freshness tokens reject without mutation;
- public replay export/import works through WASM;
- UI smoke can export a replay, import it, step at least one command, and display the same public resource/market state;
- no test fixture, DOM attribute, local storage value, replay export, dev panel, or bot rationale contains internal-only valuation/debug fields.

The visibility tests can be simpler than High Card Duel's because there is no hidden information, but they must exist to preserve the no-leak harness and public-view discipline.

## Fixtures, properties, rule coverage, and benchmarks

Property/invariant tests should cover:

- resources are conserved across inventories plus public supply plus paid/held contracts according to the intended accounting model;
- a fulfilled contract cannot be fulfilled again;
- no resource count goes negative;
- legal collect/exchange/fulfill actions never panic and never create invalid state;
- illegal insufficient-cost, empty-slot, exhausted-supply, non-active-seat, terminal, and stale-token commands reject without mutation;
- terminal states expose no normal gameplay actions;
- action-tree choice IDs are stable and duplicate-free;
- bot actions always validate through normal command validation.

Rule coverage must map every rule section to at least one test, fixture, or explicit not-applicable note.

Benchmark operations should include at least:

- setup;
- legal action tree;
- validate/apply collect;
- validate/apply exchange;
- validate/apply fulfill/refill;
- public view projection;
- effect serialization/filtering;
- replay command stream;
- random legal playout;
- Level 1 bot decision;
- WASM operation smoke.

Start with smoke thresholds if CI baselines are unknown, but `BENCHMARKS.md` must explain that they are smoke floors and must name the follow-up calibration expectation. Do not claim performance without measured numbers.

## Tooling and workflow registration

Register `token_bazaar` wherever existing official games are enumerated, including tools that operate by game id:

- `tools/simulate`
- `tools/replay-check`
- `tools/fixture-check`
- `tools/rule-coverage`
- `tools/bench-report`, if it keeps game-specific assumptions
- `tools/seed-reducer`, if game id or trace schema output is enumerated
- `tools/trace-viewer`, if game-specific projection/labels are enumerated
- `.github/workflows/gate-1-game-smoke.yml`
- `.github/workflows/gate-2-benchmarks.yml`
- `apps/web/package.json`

The implementation summary must mention every registration point that was updated or deliberately not applicable.

## Validation commands

Run from the repository root.

```bash
cargo fmt --all --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
cargo test -p token_bazaar
cargo run -p simulate -- --game token_bazaar --games 1000 --start-seed 1
cargo run -p replay-check -- --game token_bazaar --all
cargo run -p fixture-check -- --game token_bazaar
cargo run -p rule-coverage -- --game token_bazaar
cargo bench -p token_bazaar -- legal_actions
cargo bench -p token_bazaar
bash scripts/boundary-check.sh
node scripts/check-doc-links.mjs
npm --prefix apps/web ci
npm --prefix apps/web run build
npm --prefix apps/web run smoke:wasm
npm --prefix apps/web run smoke:ui
npm --prefix apps/web run smoke:e2e
node apps/web/e2e/token-bazaar.smoke.mjs
```

If benchmark thresholds are only smoke floors initially, still run the bench and commit a clear `thresholds.json` with documented floor semantics.

## Acceptance criteria

Gate 9 is accepted only when all criteria below are true.

### Rules and state

- `token_bazaar_standard` implements the specified public supply, inventories, market row, deterministic contract queue, collect/exchange/fulfill actions, forced pass if needed, turn cap, terminal checks, and tie-breaks.
- Rust owns all legality, effects, replay, terminal outcome, tie-breaks, and bot decisions.
- TypeScript never computes affordability, refill, winner, terminal outcome, or bot policy.
- All invalid commands reject with stable diagnostics and no mutation.
- All resource transitions are auditable through effects and replay.

### Docs

- All required game docs exist and are filled out from templates.
- `docs/MECHANIC-ATLAS.md` records Token Bazaar as first official public resource/accounting pressure and keeps it local.
- `docs/ROADMAP.md`, `specs/README.md`, and `progress.md` are updated consistently after implementation.
- `games/token_bazaar/docs/SOURCES.md` states the game is original and does not copy commercial rules/assets/prose.

### Tests and replay

- Unit/rule/property/replay/serialization/visibility/bot tests pass.
- Golden traces cover normal play, terminal play, contract refill, exchange, market exhaustion, invalid diagnostics, stale command rejection, bot action, and WASM export.
- Replay checks reproduce hashes and terminal outcomes.
- Fixture and rule-coverage tools pass for `token_bazaar`.

### Bot

- Level 1 bot is the default browser bot.
- Bot rationale is public-safe and present for non-random decisions.
- Bot action tests prove legal validation over multiple states/seeds.
- Evidence pack shows at least one contract fulfillment and one collect/exchange move aimed at a visible target.

### Browser

- Game picker can start Token Bazaar.
- Browser UI shows inventories, public supply, market row, costs, scores, turn count, legal actions, effects, replay, bot turn, and terminal outcome.
- E2E smoke covers human action, bot action, replay/export/import or replay stepping, dev panel, reduced-motion path, and no-leak/a11y checklist items.
- Resource information is not conveyed by color alone.

### Boundary and CI

- `bash scripts/boundary-check.sh` passes.
- `node scripts/check-doc-links.mjs` passes.
- Workflows include Token Bazaar in the appropriate smoke and benchmark lanes.
- No economy/card/market/contract/resource noun is added to `engine-core`.
- No generic resource primitive is added to `game-stdlib`.

## Candidate placement after Gate 9

| Candidate | Decision |
|---|---|
| `token_bazaar` | Build now as Gate 9 public resource/accounting browser proof. |
| `resource_race` | Treat as an alternate/alias label for the same economy slot; do not build separately unless a future accepted spec replaces Token Bazaar. |
| `secret_draft` | Build next as a dedicated focused simultaneous commitment / waiting / reveal proof, inheriting the two deferred ROADMAP §11 exit lines. Recommended placement: **Gate 9.1, immediately after Gate 9 and before Gate 10 (`poker_lite` betting)**, since poker_lite's imperfect-info/waiting UX depends on the commitment/reveal proof landing first. Not bundled into this implementation. |
| `blackjack_lite` | Keep deferred under ADR 0006. Reconsider only after resource/accounting and simultaneous-choice pressure have landed, and only with a scoped non-casino naming/IP plan. |
| `poker_lite` | Gate 10+ candidate after hidden info, resources/accounting, and bot/no-leak discipline are in place. Do not start now. |
| `plain_tricks` | Gate 10+ classic trick-taking candidate after current breadth proofs. Do not start now. |
| `masked_claims` | Later bluffing/reaction-window proof. No reaction primitive now. |
| `flood_watch`, `frontier_control`, `event_frontier` | Later high-complexity public/cooperative/asymmetric gates. Do not prepare monster-game scaffolding now. |

## Risks and rollback/defer decisions

| Risk | Decision / mitigation |
|---|---|
| Scope creep into an economy engine. | Keep all accounting game-local; add atlas note only. |
| Bot looks random or embarrassing. | Require Level 1 target-contract heuristics, public rationale, and evidence fixtures. |
| UI becomes cluttered with chips and cards. | Prefer simple labelled sections and Rust-provided metadata; no fancy art required. |
| Supply exhaustion creates no-action deadlock. | Include forced pass only when no other legal action exists, and test it. |
| Market queue balance is imperfect. | Accept rough balance. This is a proof game, not a polished commercial design. |
| Contract names accidentally echo a commercial game. | Keep labels generic/original and document IP review. |
| Random setup is tempting. | Defer it. Deterministic contract queue is enough for Gate 9. |
| Generic helper feels convenient. | Stop unless a primitive-pressure ledger justifies it. Expected outcome is no helper. |
| Benchmark floors are unknown. | Use smoke floors with explicit calibration notes; do not claim performance without measured baselines. |
| Existing tools are missing game registration seams. | Register Token Bazaar or document a deliberate not-applicable reason in the implementation summary. |

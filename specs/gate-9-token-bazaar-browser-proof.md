# Gate 9 Token Bazaar Browser Proof

Status: proposed implementation spec  
Target repository: `joeloverbeck/rulepath`  
Target commit: `5a489b1d54d1b419db439893f628c4c7e6b410fc`  
Primary crate/internal game id: `token_bazaar`  
Public display name: `Token Bazaar` unless the implementation session finds a clearly better neutral name  
Browser implementation: required

## Exact-commit discipline

This spec is based only on the uploaded manifest as path inventory and exact raw URLs under:

```text
https://raw.githubusercontent.com/joeloverbeck/rulepath/5a489b1d54d1b419db439893f628c4c7e6b410fc/<path>
```

This workflow did **not** independently verify the latest `main`. It analyzes only the user-supplied target commit `5a489b1d54d1b419db439893f628c4c7e6b410fc`.

No fetched repository source pointed to `joeloverbeck/one-more-branch` or any repository other than `joeloverbeck/rulepath`. If an implementation session cannot fetch the same exact URLs, it must stop rather than falling back to a branch name, repository search, connector namespace, or default-branch metadata.

## Strategic decision

Implement `token_bazaar` as the Gate 9 browser proof.

Do not replace it with `secret_draft` now. Gate 8 already proved hidden information, deterministic chance setup, private views, public redaction, bot view discipline, and browser no-leak patterns through High Card Duel. Gate 9 should now broaden Rulepath toward public resource accounting, payments, visible market state, deterministic refills, scoring pressure, and browser UI affordances for economy state.

`secret_draft` is still valuable, but it should come later as a focused simultaneous commitment / waiting / reveal gate. Bundling it into Token Bazaar would turn one implementation session into two gates and would muddle the acceptance evidence.

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

These rules are intentionally small and original. Implement them exactly unless the implementation session documents a direct contradiction in exact-commit repository evidence.

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
| `secret_draft` | Preserve as a later focused simultaneous commitment / waiting / reveal proof. Recommended placement: Gate 9.1 or the next hidden/waiting UX gate after Token Bazaar, not bundled into this implementation. |
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

## Evidence appendix

### Uploaded manifest

- `manifest_2026-06-08(10).txt` was used only as path inventory. Repository facts came from exact raw URLs below.

### Repository URLs fetched

- `https://raw.githubusercontent.com/joeloverbeck/rulepath/5a489b1d54d1b419db439893f628c4c7e6b410fc/README.md`
- `https://raw.githubusercontent.com/joeloverbeck/rulepath/5a489b1d54d1b419db439893f628c4c7e6b410fc/progress.md`
- `https://raw.githubusercontent.com/joeloverbeck/rulepath/5a489b1d54d1b419db439893f628c4c7e6b410fc/AGENTS.md`
- `https://raw.githubusercontent.com/joeloverbeck/rulepath/5a489b1d54d1b419db439893f628c4c7e6b410fc/CLAUDE.md`
- `https://raw.githubusercontent.com/joeloverbeck/rulepath/5a489b1d54d1b419db439893f628c4c7e6b410fc/Cargo.toml`
- `https://raw.githubusercontent.com/joeloverbeck/rulepath/5a489b1d54d1b419db439893f628c4c7e6b410fc/benches/README.md`
- `https://raw.githubusercontent.com/joeloverbeck/rulepath/5a489b1d54d1b419db439893f628c4c7e6b410fc/specs/README.md`
- `https://raw.githubusercontent.com/joeloverbeck/rulepath/5a489b1d54d1b419db439893f628c4c7e6b410fc/docs/AGENT-DISCIPLINE.md`
- `https://raw.githubusercontent.com/joeloverbeck/rulepath/5a489b1d54d1b419db439893f628c4c7e6b410fc/docs/AI-BOTS.md`
- `https://raw.githubusercontent.com/joeloverbeck/rulepath/5a489b1d54d1b419db439893f628c4c7e6b410fc/docs/ARCHITECTURE.md`
- `https://raw.githubusercontent.com/joeloverbeck/rulepath/5a489b1d54d1b419db439893f628c4c7e6b410fc/docs/ENGINE-GAME-DATA-BOUNDARY.md`
- `https://raw.githubusercontent.com/joeloverbeck/rulepath/5a489b1d54d1b419db439893f628c4c7e6b410fc/docs/FOUNDATIONS.md`
- `https://raw.githubusercontent.com/joeloverbeck/rulepath/5a489b1d54d1b419db439893f628c4c7e6b410fc/docs/IP-POLICY.md`
- `https://raw.githubusercontent.com/joeloverbeck/rulepath/5a489b1d54d1b419db439893f628c4c7e6b410fc/docs/MECHANIC-ATLAS.md`
- `https://raw.githubusercontent.com/joeloverbeck/rulepath/5a489b1d54d1b419db439893f628c4c7e6b410fc/docs/OFFICIAL-GAME-CONTRACT.md`
- `https://raw.githubusercontent.com/joeloverbeck/rulepath/5a489b1d54d1b419db439893f628c4c7e6b410fc/docs/README.md`
- `https://raw.githubusercontent.com/joeloverbeck/rulepath/5a489b1d54d1b419db439893f628c4c7e6b410fc/docs/ROADMAP.md`
- `https://raw.githubusercontent.com/joeloverbeck/rulepath/5a489b1d54d1b419db439893f628c4c7e6b410fc/docs/SOURCES.md`
- `https://raw.githubusercontent.com/joeloverbeck/rulepath/5a489b1d54d1b419db439893f628c4c7e6b410fc/docs/TESTING-REPLAY-BENCHMARKING.md`
- `https://raw.githubusercontent.com/joeloverbeck/rulepath/5a489b1d54d1b419db439893f628c4c7e6b410fc/docs/TRACE-SCHEMA-v1.md`
- `https://raw.githubusercontent.com/joeloverbeck/rulepath/5a489b1d54d1b419db439893f628c4c7e6b410fc/docs/UI-INTERACTION.md`
- `https://raw.githubusercontent.com/joeloverbeck/rulepath/5a489b1d54d1b419db439893f628c4c7e6b410fc/docs/WASM-CLIENT-BOUNDARY.md`
- `https://raw.githubusercontent.com/joeloverbeck/rulepath/5a489b1d54d1b419db439893f628c4c7e6b410fc/docs/archival-workflow.md`
- `https://raw.githubusercontent.com/joeloverbeck/rulepath/5a489b1d54d1b419db439893f628c4c7e6b410fc/docs/adr/0001-stage-1-random-playout-budget.md`
- `https://raw.githubusercontent.com/joeloverbeck/rulepath/5a489b1d54d1b419db439893f628c4c7e6b410fc/docs/adr/0002-ci-benchmark-gating-lanes.md`
- `https://raw.githubusercontent.com/joeloverbeck/rulepath/5a489b1d54d1b419db439893f628c4c7e6b410fc/docs/adr/0003-ci-calibrated-benchmark-thresholds.md`
- `https://raw.githubusercontent.com/joeloverbeck/rulepath/5a489b1d54d1b419db439893f628c4c7e6b410fc/docs/adr/0004-hidden-info-replay-export-taxonomy.md`
- `https://raw.githubusercontent.com/joeloverbeck/rulepath/5a489b1d54d1b419db439893f628c4c7e6b410fc/docs/adr/0005-variance-aware-ci-benchmark-floors.md`
- `https://raw.githubusercontent.com/joeloverbeck/rulepath/5a489b1d54d1b419db439893f628c4c7e6b410fc/docs/adr/0006-blackjack-lite-roadmap-placement.md`
- `https://raw.githubusercontent.com/joeloverbeck/rulepath/5a489b1d54d1b419db439893f628c4c7e6b410fc/docs/adr/ADR-TEMPLATE.md`
- `https://raw.githubusercontent.com/joeloverbeck/rulepath/5a489b1d54d1b419db439893f628c4c7e6b410fc/templates/README.md`
- `https://raw.githubusercontent.com/joeloverbeck/rulepath/5a489b1d54d1b419db439893f628c4c7e6b410fc/templates/AGENT-TASK.md`
- `https://raw.githubusercontent.com/joeloverbeck/rulepath/5a489b1d54d1b419db439893f628c4c7e6b410fc/templates/BOT-STRATEGY-EVIDENCE-PACK.md`
- `https://raw.githubusercontent.com/joeloverbeck/rulepath/5a489b1d54d1b419db439893f628c4c7e6b410fc/templates/COMPETENT-PLAYER.md`
- `https://raw.githubusercontent.com/joeloverbeck/rulepath/5a489b1d54d1b419db439893f628c4c7e6b410fc/templates/GAME-AI.md`
- `https://raw.githubusercontent.com/joeloverbeck/rulepath/5a489b1d54d1b419db439893f628c4c7e6b410fc/templates/GAME-BENCHMARKS.md`
- `https://raw.githubusercontent.com/joeloverbeck/rulepath/5a489b1d54d1b419db439893f628c4c7e6b410fc/templates/GAME-IMPLEMENTATION-ADMISSION.md`
- `https://raw.githubusercontent.com/joeloverbeck/rulepath/5a489b1d54d1b419db439893f628c4c7e6b410fc/templates/GAME-MECHANICS.md`
- `https://raw.githubusercontent.com/joeloverbeck/rulepath/5a489b1d54d1b419db439893f628c4c7e6b410fc/templates/GAME-RULE-COVERAGE.md`
- `https://raw.githubusercontent.com/joeloverbeck/rulepath/5a489b1d54d1b419db439893f628c4c7e6b410fc/templates/GAME-RULES.md`
- `https://raw.githubusercontent.com/joeloverbeck/rulepath/5a489b1d54d1b419db439893f628c4c7e6b410fc/templates/GAME-SOURCES.md`
- `https://raw.githubusercontent.com/joeloverbeck/rulepath/5a489b1d54d1b419db439893f628c4c7e6b410fc/templates/GAME-UI.md`
- `https://raw.githubusercontent.com/joeloverbeck/rulepath/5a489b1d54d1b419db439893f628c4c7e6b410fc/templates/PRIMITIVE-PRESSURE-LEDGER.md`
- `https://raw.githubusercontent.com/joeloverbeck/rulepath/5a489b1d54d1b419db439893f628c4c7e6b410fc/templates/PUBLIC-RELEASE-CHECKLIST.md`
- `https://raw.githubusercontent.com/joeloverbeck/rulepath/5a489b1d54d1b419db439893f628c4c7e6b410fc/archive/specs/gate-6-directional-flip.md`
- `https://raw.githubusercontent.com/joeloverbeck/rulepath/5a489b1d54d1b419db439893f628c4c7e6b410fc/archive/specs/gate-7-draughts-lite-compound-action-tree.md`
- `https://raw.githubusercontent.com/joeloverbeck/rulepath/5a489b1d54d1b419db439893f628c4c7e6b410fc/archive/specs/gate-7-1-board-space-primitive-back-port.md`
- `https://raw.githubusercontent.com/joeloverbeck/rulepath/5a489b1d54d1b419db439893f628c4c7e6b410fc/archive/specs/gate-7-2-and-gate-8-high-card-duel-hidden-info-chance-proof.md`
- `https://raw.githubusercontent.com/joeloverbeck/rulepath/5a489b1d54d1b419db439893f628c4c7e6b410fc/crates/engine-core/Cargo.toml`
- `https://raw.githubusercontent.com/joeloverbeck/rulepath/5a489b1d54d1b419db439893f628c4c7e6b410fc/crates/engine-core/src/action.rs`
- `https://raw.githubusercontent.com/joeloverbeck/rulepath/5a489b1d54d1b419db439893f628c4c7e6b410fc/crates/engine-core/src/game.rs`
- `https://raw.githubusercontent.com/joeloverbeck/rulepath/5a489b1d54d1b419db439893f628c4c7e6b410fc/crates/engine-core/src/lib.rs`
- `https://raw.githubusercontent.com/joeloverbeck/rulepath/5a489b1d54d1b419db439893f628c4c7e6b410fc/crates/engine-core/src/replay.rs`
- `https://raw.githubusercontent.com/joeloverbeck/rulepath/5a489b1d54d1b419db439893f628c4c7e6b410fc/crates/engine-core/src/rng.rs`
- `https://raw.githubusercontent.com/joeloverbeck/rulepath/5a489b1d54d1b419db439893f628c4c7e6b410fc/crates/game-stdlib/Cargo.toml`
- `https://raw.githubusercontent.com/joeloverbeck/rulepath/5a489b1d54d1b419db439893f628c4c7e6b410fc/crates/game-stdlib/src/board_space.rs`
- `https://raw.githubusercontent.com/joeloverbeck/rulepath/5a489b1d54d1b419db439893f628c4c7e6b410fc/crates/game-stdlib/src/lib.rs`
- `https://raw.githubusercontent.com/joeloverbeck/rulepath/5a489b1d54d1b419db439893f628c4c7e6b410fc/crates/ai-core/Cargo.toml`
- `https://raw.githubusercontent.com/joeloverbeck/rulepath/5a489b1d54d1b419db439893f628c4c7e6b410fc/crates/ai-core/src/lib.rs`
- `https://raw.githubusercontent.com/joeloverbeck/rulepath/5a489b1d54d1b419db439893f628c4c7e6b410fc/crates/ai-core/src/random_legal.rs`
- `https://raw.githubusercontent.com/joeloverbeck/rulepath/5a489b1d54d1b419db439893f628c4c7e6b410fc/crates/wasm-api/Cargo.toml`
- `https://raw.githubusercontent.com/joeloverbeck/rulepath/5a489b1d54d1b419db439893f628c4c7e6b410fc/crates/wasm-api/src/lib.rs`
- `https://raw.githubusercontent.com/joeloverbeck/rulepath/5a489b1d54d1b419db439893f628c4c7e6b410fc/games/high_card_duel/Cargo.toml`
- `https://raw.githubusercontent.com/joeloverbeck/rulepath/5a489b1d54d1b419db439893f628c4c7e6b410fc/games/high_card_duel/benches/high_card_duel.rs`
- `https://raw.githubusercontent.com/joeloverbeck/rulepath/5a489b1d54d1b419db439893f628c4c7e6b410fc/games/high_card_duel/benches/thresholds.json`
- `https://raw.githubusercontent.com/joeloverbeck/rulepath/5a489b1d54d1b419db439893f628c4c7e6b410fc/games/high_card_duel/data/fixtures/.gitkeep`
- `https://raw.githubusercontent.com/joeloverbeck/rulepath/5a489b1d54d1b419db439893f628c4c7e6b410fc/games/high_card_duel/data/fixtures/high_card_duel_standard.fixture.json`
- `https://raw.githubusercontent.com/joeloverbeck/rulepath/5a489b1d54d1b419db439893f628c4c7e6b410fc/games/high_card_duel/data/manifest.toml`
- `https://raw.githubusercontent.com/joeloverbeck/rulepath/5a489b1d54d1b419db439893f628c4c7e6b410fc/games/high_card_duel/data/variants.toml`
- `https://raw.githubusercontent.com/joeloverbeck/rulepath/5a489b1d54d1b419db439893f628c4c7e6b410fc/games/high_card_duel/docs/AI.md`
- `https://raw.githubusercontent.com/joeloverbeck/rulepath/5a489b1d54d1b419db439893f628c4c7e6b410fc/games/high_card_duel/docs/BENCHMARKS.md`
- `https://raw.githubusercontent.com/joeloverbeck/rulepath/5a489b1d54d1b419db439893f628c4c7e6b410fc/games/high_card_duel/docs/BOT-STRATEGY-EVIDENCE-PACK.md`
- `https://raw.githubusercontent.com/joeloverbeck/rulepath/5a489b1d54d1b419db439893f628c4c7e6b410fc/games/high_card_duel/docs/COMPETENT-PLAYER.md`
- `https://raw.githubusercontent.com/joeloverbeck/rulepath/5a489b1d54d1b419db439893f628c4c7e6b410fc/games/high_card_duel/docs/GAME-IMPLEMENTATION-ADMISSION.md`
- `https://raw.githubusercontent.com/joeloverbeck/rulepath/5a489b1d54d1b419db439893f628c4c7e6b410fc/games/high_card_duel/docs/MECHANICS.md`
- `https://raw.githubusercontent.com/joeloverbeck/rulepath/5a489b1d54d1b419db439893f628c4c7e6b410fc/games/high_card_duel/docs/PUBLIC-RELEASE-CHECKLIST.md`
- `https://raw.githubusercontent.com/joeloverbeck/rulepath/5a489b1d54d1b419db439893f628c4c7e6b410fc/games/high_card_duel/docs/RULE-COVERAGE.md`
- `https://raw.githubusercontent.com/joeloverbeck/rulepath/5a489b1d54d1b419db439893f628c4c7e6b410fc/games/high_card_duel/docs/RULES.md`
- `https://raw.githubusercontent.com/joeloverbeck/rulepath/5a489b1d54d1b419db439893f628c4c7e6b410fc/games/high_card_duel/docs/SOURCES.md`
- `https://raw.githubusercontent.com/joeloverbeck/rulepath/5a489b1d54d1b419db439893f628c4c7e6b410fc/games/high_card_duel/docs/UI.md`
- `https://raw.githubusercontent.com/joeloverbeck/rulepath/5a489b1d54d1b419db439893f628c4c7e6b410fc/games/high_card_duel/src/actions.rs`
- `https://raw.githubusercontent.com/joeloverbeck/rulepath/5a489b1d54d1b419db439893f628c4c7e6b410fc/games/high_card_duel/src/bots.rs`
- `https://raw.githubusercontent.com/joeloverbeck/rulepath/5a489b1d54d1b419db439893f628c4c7e6b410fc/games/high_card_duel/src/effects.rs`
- `https://raw.githubusercontent.com/joeloverbeck/rulepath/5a489b1d54d1b419db439893f628c4c7e6b410fc/games/high_card_duel/src/ids.rs`
- `https://raw.githubusercontent.com/joeloverbeck/rulepath/5a489b1d54d1b419db439893f628c4c7e6b410fc/games/high_card_duel/src/lib.rs`
- `https://raw.githubusercontent.com/joeloverbeck/rulepath/5a489b1d54d1b419db439893f628c4c7e6b410fc/games/high_card_duel/src/replay_support.rs`
- `https://raw.githubusercontent.com/joeloverbeck/rulepath/5a489b1d54d1b419db439893f628c4c7e6b410fc/games/high_card_duel/src/rules.rs`
- `https://raw.githubusercontent.com/joeloverbeck/rulepath/5a489b1d54d1b419db439893f628c4c7e6b410fc/games/high_card_duel/src/setup.rs`
- `https://raw.githubusercontent.com/joeloverbeck/rulepath/5a489b1d54d1b419db439893f628c4c7e6b410fc/games/high_card_duel/src/state.rs`
- `https://raw.githubusercontent.com/joeloverbeck/rulepath/5a489b1d54d1b419db439893f628c4c7e6b410fc/games/high_card_duel/src/ui.rs`
- `https://raw.githubusercontent.com/joeloverbeck/rulepath/5a489b1d54d1b419db439893f628c4c7e6b410fc/games/high_card_duel/src/variants.rs`
- `https://raw.githubusercontent.com/joeloverbeck/rulepath/5a489b1d54d1b419db439893f628c4c7e6b410fc/games/high_card_duel/src/visibility.rs`
- `https://raw.githubusercontent.com/joeloverbeck/rulepath/5a489b1d54d1b419db439893f628c4c7e6b410fc/games/high_card_duel/tests/bots.rs`
- `https://raw.githubusercontent.com/joeloverbeck/rulepath/5a489b1d54d1b419db439893f628c4c7e6b410fc/games/high_card_duel/tests/property.rs`
- `https://raw.githubusercontent.com/joeloverbeck/rulepath/5a489b1d54d1b419db439893f628c4c7e6b410fc/games/high_card_duel/tests/replay.rs`
- `https://raw.githubusercontent.com/joeloverbeck/rulepath/5a489b1d54d1b419db439893f628c4c7e6b410fc/games/high_card_duel/tests/rules.rs`
- `https://raw.githubusercontent.com/joeloverbeck/rulepath/5a489b1d54d1b419db439893f628c4c7e6b410fc/games/high_card_duel/tests/serialization.rs`
- `https://raw.githubusercontent.com/joeloverbeck/rulepath/5a489b1d54d1b419db439893f628c4c7e6b410fc/games/high_card_duel/tests/visibility.rs`
- `https://raw.githubusercontent.com/joeloverbeck/rulepath/5a489b1d54d1b419db439893f628c4c7e6b410fc/games/high_card_duel/tests/golden_traces/bot-action.trace.json`
- `https://raw.githubusercontent.com/joeloverbeck/rulepath/5a489b1d54d1b419db439893f628c4c7e6b410fc/games/high_card_duel/tests/golden_traces/hidden-info-public-observer.trace.json`
- `https://raw.githubusercontent.com/joeloverbeck/rulepath/5a489b1d54d1b419db439893f628c4c7e6b410fc/games/high_card_duel/tests/golden_traces/invalid-private-card-redacted.trace.json`
- `https://raw.githubusercontent.com/joeloverbeck/rulepath/5a489b1d54d1b419db439893f628c4c7e6b410fc/games/high_card_duel/tests/golden_traces/invalid-wrong-seat-diagnostic.trace.json`
- `https://raw.githubusercontent.com/joeloverbeck/rulepath/5a489b1d54d1b419db439893f628c4c7e6b410fc/games/high_card_duel/tests/golden_traces/public-replay-export-import.trace.json`
- `https://raw.githubusercontent.com/joeloverbeck/rulepath/5a489b1d54d1b419db439893f628c4c7e6b410fc/games/high_card_duel/tests/golden_traces/seat-private-view.trace.json`
- `https://raw.githubusercontent.com/joeloverbeck/rulepath/5a489b1d54d1b419db439893f628c4c7e6b410fc/games/high_card_duel/tests/golden_traces/shortest-normal.trace.json`
- `https://raw.githubusercontent.com/joeloverbeck/rulepath/5a489b1d54d1b419db439893f628c4c7e6b410fc/games/high_card_duel/tests/golden_traces/stale-diagnostic.trace.json`
- `https://raw.githubusercontent.com/joeloverbeck/rulepath/5a489b1d54d1b419db439893f628c4c7e6b410fc/games/high_card_duel/tests/golden_traces/terminal.trace.json`
- `https://raw.githubusercontent.com/joeloverbeck/rulepath/5a489b1d54d1b419db439893f628c4c7e6b410fc/games/high_card_duel/tests/golden_traces/tie-round.trace.json`
- `https://raw.githubusercontent.com/joeloverbeck/rulepath/5a489b1d54d1b419db439893f628c4c7e6b410fc/games/column_four/docs/AI.md`
- `https://raw.githubusercontent.com/joeloverbeck/rulepath/5a489b1d54d1b419db439893f628c4c7e6b410fc/games/column_four/docs/BENCHMARKS.md`
- `https://raw.githubusercontent.com/joeloverbeck/rulepath/5a489b1d54d1b419db439893f628c4c7e6b410fc/games/column_four/docs/BOT-STRATEGY-EVIDENCE-PACK.md`
- `https://raw.githubusercontent.com/joeloverbeck/rulepath/5a489b1d54d1b419db439893f628c4c7e6b410fc/games/column_four/docs/COMPETENT-PLAYER.md`
- `https://raw.githubusercontent.com/joeloverbeck/rulepath/5a489b1d54d1b419db439893f628c4c7e6b410fc/games/column_four/docs/GAME-IMPLEMENTATION-ADMISSION.md`
- `https://raw.githubusercontent.com/joeloverbeck/rulepath/5a489b1d54d1b419db439893f628c4c7e6b410fc/games/column_four/docs/MECHANICS.md`
- `https://raw.githubusercontent.com/joeloverbeck/rulepath/5a489b1d54d1b419db439893f628c4c7e6b410fc/games/column_four/docs/PUBLIC-RELEASE-CHECKLIST.md`
- `https://raw.githubusercontent.com/joeloverbeck/rulepath/5a489b1d54d1b419db439893f628c4c7e6b410fc/games/column_four/docs/RULE-COVERAGE.md`
- `https://raw.githubusercontent.com/joeloverbeck/rulepath/5a489b1d54d1b419db439893f628c4c7e6b410fc/games/column_four/docs/RULES.md`
- `https://raw.githubusercontent.com/joeloverbeck/rulepath/5a489b1d54d1b419db439893f628c4c7e6b410fc/games/column_four/docs/SOURCES.md`
- `https://raw.githubusercontent.com/joeloverbeck/rulepath/5a489b1d54d1b419db439893f628c4c7e6b410fc/games/column_four/docs/UI.md`
- `https://raw.githubusercontent.com/joeloverbeck/rulepath/5a489b1d54d1b419db439893f628c4c7e6b410fc/games/directional_flip/docs/AI.md`
- `https://raw.githubusercontent.com/joeloverbeck/rulepath/5a489b1d54d1b419db439893f628c4c7e6b410fc/games/directional_flip/docs/BENCHMARKS.md`
- `https://raw.githubusercontent.com/joeloverbeck/rulepath/5a489b1d54d1b419db439893f628c4c7e6b410fc/games/directional_flip/docs/BOT-STRATEGY-EVIDENCE-PACK.md`
- `https://raw.githubusercontent.com/joeloverbeck/rulepath/5a489b1d54d1b419db439893f628c4c7e6b410fc/games/directional_flip/docs/COMPETENT-PLAYER.md`
- `https://raw.githubusercontent.com/joeloverbeck/rulepath/5a489b1d54d1b419db439893f628c4c7e6b410fc/games/directional_flip/docs/GAME-IMPLEMENTATION-ADMISSION.md`
- `https://raw.githubusercontent.com/joeloverbeck/rulepath/5a489b1d54d1b419db439893f628c4c7e6b410fc/games/directional_flip/docs/MECHANICS.md`
- `https://raw.githubusercontent.com/joeloverbeck/rulepath/5a489b1d54d1b419db439893f628c4c7e6b410fc/games/directional_flip/docs/PRIMITIVE-PRESSURE-LEDGER.md`
- `https://raw.githubusercontent.com/joeloverbeck/rulepath/5a489b1d54d1b419db439893f628c4c7e6b410fc/games/directional_flip/docs/PUBLIC-RELEASE-CHECKLIST.md`
- `https://raw.githubusercontent.com/joeloverbeck/rulepath/5a489b1d54d1b419db439893f628c4c7e6b410fc/games/directional_flip/docs/RULE-COVERAGE.md`
- `https://raw.githubusercontent.com/joeloverbeck/rulepath/5a489b1d54d1b419db439893f628c4c7e6b410fc/games/directional_flip/docs/RULES.md`
- `https://raw.githubusercontent.com/joeloverbeck/rulepath/5a489b1d54d1b419db439893f628c4c7e6b410fc/games/directional_flip/docs/SOURCES.md`
- `https://raw.githubusercontent.com/joeloverbeck/rulepath/5a489b1d54d1b419db439893f628c4c7e6b410fc/games/directional_flip/docs/UI.md`
- `https://raw.githubusercontent.com/joeloverbeck/rulepath/5a489b1d54d1b419db439893f628c4c7e6b410fc/games/draughts_lite/docs/AI.md`
- `https://raw.githubusercontent.com/joeloverbeck/rulepath/5a489b1d54d1b419db439893f628c4c7e6b410fc/games/draughts_lite/docs/BENCHMARKS.md`
- `https://raw.githubusercontent.com/joeloverbeck/rulepath/5a489b1d54d1b419db439893f628c4c7e6b410fc/games/draughts_lite/docs/BOT-STRATEGY-EVIDENCE-PACK.md`
- `https://raw.githubusercontent.com/joeloverbeck/rulepath/5a489b1d54d1b419db439893f628c4c7e6b410fc/games/draughts_lite/docs/COMPETENT-PLAYER.md`
- `https://raw.githubusercontent.com/joeloverbeck/rulepath/5a489b1d54d1b419db439893f628c4c7e6b410fc/games/draughts_lite/docs/GAME-IMPLEMENTATION-ADMISSION.md`
- `https://raw.githubusercontent.com/joeloverbeck/rulepath/5a489b1d54d1b419db439893f628c4c7e6b410fc/games/draughts_lite/docs/MECHANICS.md`
- `https://raw.githubusercontent.com/joeloverbeck/rulepath/5a489b1d54d1b419db439893f628c4c7e6b410fc/games/draughts_lite/docs/PRIMITIVE-PRESSURE-LEDGER.md`
- `https://raw.githubusercontent.com/joeloverbeck/rulepath/5a489b1d54d1b419db439893f628c4c7e6b410fc/games/draughts_lite/docs/PUBLIC-RELEASE-CHECKLIST.md`
- `https://raw.githubusercontent.com/joeloverbeck/rulepath/5a489b1d54d1b419db439893f628c4c7e6b410fc/games/draughts_lite/docs/RULE-COVERAGE.md`
- `https://raw.githubusercontent.com/joeloverbeck/rulepath/5a489b1d54d1b419db439893f628c4c7e6b410fc/games/draughts_lite/docs/RULES.md`
- `https://raw.githubusercontent.com/joeloverbeck/rulepath/5a489b1d54d1b419db439893f628c4c7e6b410fc/games/draughts_lite/docs/SOURCES.md`
- `https://raw.githubusercontent.com/joeloverbeck/rulepath/5a489b1d54d1b419db439893f628c4c7e6b410fc/games/draughts_lite/docs/UI.md`
- `https://raw.githubusercontent.com/joeloverbeck/rulepath/5a489b1d54d1b419db439893f628c4c7e6b410fc/games/race_to_n/docs/AI.md`
- `https://raw.githubusercontent.com/joeloverbeck/rulepath/5a489b1d54d1b419db439893f628c4c7e6b410fc/games/race_to_n/docs/BENCHMARKS.md`
- `https://raw.githubusercontent.com/joeloverbeck/rulepath/5a489b1d54d1b419db439893f628c4c7e6b410fc/games/race_to_n/docs/GAME-IMPLEMENTATION-ADMISSION.md`
- `https://raw.githubusercontent.com/joeloverbeck/rulepath/5a489b1d54d1b419db439893f628c4c7e6b410fc/games/race_to_n/docs/MECHANICS.md`
- `https://raw.githubusercontent.com/joeloverbeck/rulepath/5a489b1d54d1b419db439893f628c4c7e6b410fc/games/race_to_n/docs/RULE-COVERAGE.md`
- `https://raw.githubusercontent.com/joeloverbeck/rulepath/5a489b1d54d1b419db439893f628c4c7e6b410fc/games/race_to_n/docs/RULES.md`
- `https://raw.githubusercontent.com/joeloverbeck/rulepath/5a489b1d54d1b419db439893f628c4c7e6b410fc/games/race_to_n/docs/SOURCES.md`
- `https://raw.githubusercontent.com/joeloverbeck/rulepath/5a489b1d54d1b419db439893f628c4c7e6b410fc/games/race_to_n/docs/UI.md`
- `https://raw.githubusercontent.com/joeloverbeck/rulepath/5a489b1d54d1b419db439893f628c4c7e6b410fc/games/three_marks/docs/AI.md`
- `https://raw.githubusercontent.com/joeloverbeck/rulepath/5a489b1d54d1b419db439893f628c4c7e6b410fc/games/three_marks/docs/BENCHMARKS.md`
- `https://raw.githubusercontent.com/joeloverbeck/rulepath/5a489b1d54d1b419db439893f628c4c7e6b410fc/games/three_marks/docs/GAME-IMPLEMENTATION-ADMISSION.md`
- `https://raw.githubusercontent.com/joeloverbeck/rulepath/5a489b1d54d1b419db439893f628c4c7e6b410fc/games/three_marks/docs/MECHANICS.md`
- `https://raw.githubusercontent.com/joeloverbeck/rulepath/5a489b1d54d1b419db439893f628c4c7e6b410fc/games/three_marks/docs/RULE-COVERAGE.md`
- `https://raw.githubusercontent.com/joeloverbeck/rulepath/5a489b1d54d1b419db439893f628c4c7e6b410fc/games/three_marks/docs/RULES.md`
- `https://raw.githubusercontent.com/joeloverbeck/rulepath/5a489b1d54d1b419db439893f628c4c7e6b410fc/games/three_marks/docs/SOURCES.md`
- `https://raw.githubusercontent.com/joeloverbeck/rulepath/5a489b1d54d1b419db439893f628c4c7e6b410fc/games/three_marks/docs/UI.md`
- `https://raw.githubusercontent.com/joeloverbeck/rulepath/5a489b1d54d1b419db439893f628c4c7e6b410fc/games/column_four/tests/bots.rs`
- `https://raw.githubusercontent.com/joeloverbeck/rulepath/5a489b1d54d1b419db439893f628c4c7e6b410fc/games/column_four/tests/replay.rs`
- `https://raw.githubusercontent.com/joeloverbeck/rulepath/5a489b1d54d1b419db439893f628c4c7e6b410fc/games/column_four/tests/rules.rs`
- `https://raw.githubusercontent.com/joeloverbeck/rulepath/5a489b1d54d1b419db439893f628c4c7e6b410fc/games/column_four/tests/visibility.rs`
- `https://raw.githubusercontent.com/joeloverbeck/rulepath/5a489b1d54d1b419db439893f628c4c7e6b410fc/games/draughts_lite/tests/bots.rs`
- `https://raw.githubusercontent.com/joeloverbeck/rulepath/5a489b1d54d1b419db439893f628c4c7e6b410fc/games/draughts_lite/tests/replay.rs`
- `https://raw.githubusercontent.com/joeloverbeck/rulepath/5a489b1d54d1b419db439893f628c4c7e6b410fc/games/draughts_lite/tests/rules.rs`
- `https://raw.githubusercontent.com/joeloverbeck/rulepath/5a489b1d54d1b419db439893f628c4c7e6b410fc/games/draughts_lite/tests/visibility.rs`
- `https://raw.githubusercontent.com/joeloverbeck/rulepath/5a489b1d54d1b419db439893f628c4c7e6b410fc/games/directional_flip/tests/bots.rs`
- `https://raw.githubusercontent.com/joeloverbeck/rulepath/5a489b1d54d1b419db439893f628c4c7e6b410fc/games/directional_flip/tests/replay.rs`
- `https://raw.githubusercontent.com/joeloverbeck/rulepath/5a489b1d54d1b419db439893f628c4c7e6b410fc/apps/web/README.md`
- `https://raw.githubusercontent.com/joeloverbeck/rulepath/5a489b1d54d1b419db439893f628c4c7e6b410fc/apps/web/e2e/NO-LEAK-A11Y-CHECKLIST.md`
- `https://raw.githubusercontent.com/joeloverbeck/rulepath/5a489b1d54d1b419db439893f628c4c7e6b410fc/apps/web/e2e/a11y-noleak.smoke.mjs`
- `https://raw.githubusercontent.com/joeloverbeck/rulepath/5a489b1d54d1b419db439893f628c4c7e6b410fc/apps/web/e2e/column-four.smoke.mjs`
- `https://raw.githubusercontent.com/joeloverbeck/rulepath/5a489b1d54d1b419db439893f628c4c7e6b410fc/apps/web/e2e/directional-flip.smoke.mjs`
- `https://raw.githubusercontent.com/joeloverbeck/rulepath/5a489b1d54d1b419db439893f628c4c7e6b410fc/apps/web/e2e/draughts-lite.smoke.mjs`
- `https://raw.githubusercontent.com/joeloverbeck/rulepath/5a489b1d54d1b419db439893f628c4c7e6b410fc/apps/web/e2e/high-card-duel.smoke.mjs`
- `https://raw.githubusercontent.com/joeloverbeck/rulepath/5a489b1d54d1b419db439893f628c4c7e6b410fc/apps/web/e2e/shell.smoke.mjs`
- `https://raw.githubusercontent.com/joeloverbeck/rulepath/5a489b1d54d1b419db439893f628c4c7e6b410fc/apps/web/e2e/three-marks.smoke.mjs`
- `https://raw.githubusercontent.com/joeloverbeck/rulepath/5a489b1d54d1b419db439893f628c4c7e6b410fc/apps/web/index.html`
- `https://raw.githubusercontent.com/joeloverbeck/rulepath/5a489b1d54d1b419db439893f628c4c7e6b410fc/apps/web/package.json`
- `https://raw.githubusercontent.com/joeloverbeck/rulepath/5a489b1d54d1b419db439893f628c4c7e6b410fc/apps/web/scripts/smoke-load-wasm.mjs`
- `https://raw.githubusercontent.com/joeloverbeck/rulepath/5a489b1d54d1b419db439893f628c4c7e6b410fc/apps/web/scripts/smoke-preview.mjs`
- `https://raw.githubusercontent.com/joeloverbeck/rulepath/5a489b1d54d1b419db439893f628c4c7e6b410fc/apps/web/scripts/smoke-ui.mjs`
- `https://raw.githubusercontent.com/joeloverbeck/rulepath/5a489b1d54d1b419db439893f628c4c7e6b410fc/apps/web/src/components/ActionControls.tsx`
- `https://raw.githubusercontent.com/joeloverbeck/rulepath/5a489b1d54d1b419db439893f628c4c7e6b410fc/apps/web/src/components/AppShell.tsx`
- `https://raw.githubusercontent.com/joeloverbeck/rulepath/5a489b1d54d1b419db439893f628c4c7e6b410fc/apps/web/src/components/ColumnFourBoard.tsx`
- `https://raw.githubusercontent.com/joeloverbeck/rulepath/5a489b1d54d1b419db439893f628c4c7e6b410fc/apps/web/src/components/DevPanel.tsx`
- `https://raw.githubusercontent.com/joeloverbeck/rulepath/5a489b1d54d1b419db439893f628c4c7e6b410fc/apps/web/src/components/DirectionalFlipBoard.tsx`
- `https://raw.githubusercontent.com/joeloverbeck/rulepath/5a489b1d54d1b419db439893f628c4c7e6b410fc/apps/web/src/components/DraughtsLiteBoard.tsx`
- `https://raw.githubusercontent.com/joeloverbeck/rulepath/5a489b1d54d1b419db439893f628c4c7e6b410fc/apps/web/src/components/EffectLog.tsx`
- `https://raw.githubusercontent.com/joeloverbeck/rulepath/5a489b1d54d1b419db439893f628c4c7e6b410fc/apps/web/src/components/GamePicker.tsx`
- `https://raw.githubusercontent.com/joeloverbeck/rulepath/5a489b1d54d1b419db439893f628c4c7e6b410fc/apps/web/src/components/HighCardDuelBoard.tsx`
- `https://raw.githubusercontent.com/joeloverbeck/rulepath/5a489b1d54d1b419db439893f628c4c7e6b410fc/apps/web/src/components/MatchSetup.tsx`
- `https://raw.githubusercontent.com/joeloverbeck/rulepath/5a489b1d54d1b419db439893f628c4c7e6b410fc/apps/web/src/components/ModeControls.tsx`
- `https://raw.githubusercontent.com/joeloverbeck/rulepath/5a489b1d54d1b419db439893f628c4c7e6b410fc/apps/web/src/components/RaceBoard.tsx`
- `https://raw.githubusercontent.com/joeloverbeck/rulepath/5a489b1d54d1b419db439893f628c4c7e6b410fc/apps/web/src/components/ReplayImportExport.tsx`
- `https://raw.githubusercontent.com/joeloverbeck/rulepath/5a489b1d54d1b419db439893f628c4c7e6b410fc/apps/web/src/components/ReplayViewer.tsx`
- `https://raw.githubusercontent.com/joeloverbeck/rulepath/5a489b1d54d1b419db439893f628c4c7e6b410fc/apps/web/src/components/ThreeMarksBoard.tsx`
- `https://raw.githubusercontent.com/joeloverbeck/rulepath/5a489b1d54d1b419db439893f628c4c7e6b410fc/apps/web/src/components/effectFeedback.ts`
- `https://raw.githubusercontent.com/joeloverbeck/rulepath/5a489b1d54d1b419db439893f628c4c7e6b410fc/apps/web/src/main.tsx`
- `https://raw.githubusercontent.com/joeloverbeck/rulepath/5a489b1d54d1b419db439893f628c4c7e6b410fc/apps/web/src/state/shellReducer.ts`
- `https://raw.githubusercontent.com/joeloverbeck/rulepath/5a489b1d54d1b419db439893f628c4c7e6b410fc/apps/web/src/styles.css`
- `https://raw.githubusercontent.com/joeloverbeck/rulepath/5a489b1d54d1b419db439893f628c4c7e6b410fc/apps/web/src/vite-env.d.ts`
- `https://raw.githubusercontent.com/joeloverbeck/rulepath/5a489b1d54d1b419db439893f628c4c7e6b410fc/apps/web/src/wasm/client.ts`
- `https://raw.githubusercontent.com/joeloverbeck/rulepath/5a489b1d54d1b419db439893f628c4c7e6b410fc/apps/web/tsconfig.json`
- `https://raw.githubusercontent.com/joeloverbeck/rulepath/5a489b1d54d1b419db439893f628c4c7e6b410fc/apps/web/vite.config.ts`
- `https://raw.githubusercontent.com/joeloverbeck/rulepath/5a489b1d54d1b419db439893f628c4c7e6b410fc/.github/workflows/gate-0-hygiene.yml`
- `https://raw.githubusercontent.com/joeloverbeck/rulepath/5a489b1d54d1b419db439893f628c4c7e6b410fc/.github/workflows/gate-1-game-smoke.yml`
- `https://raw.githubusercontent.com/joeloverbeck/rulepath/5a489b1d54d1b419db439893f628c4c7e6b410fc/.github/workflows/gate-2-benchmarks.yml`
- `https://raw.githubusercontent.com/joeloverbeck/rulepath/5a489b1d54d1b419db439893f628c4c7e6b410fc/scripts/boundary-check.sh`
- `https://raw.githubusercontent.com/joeloverbeck/rulepath/5a489b1d54d1b419db439893f628c4c7e6b410fc/scripts/check-doc-links.mjs`
- `https://raw.githubusercontent.com/joeloverbeck/rulepath/5a489b1d54d1b419db439893f628c4c7e6b410fc/tools/bench-report/Cargo.toml`
- `https://raw.githubusercontent.com/joeloverbeck/rulepath/5a489b1d54d1b419db439893f628c4c7e6b410fc/tools/bench-report/src/main.rs`
- `https://raw.githubusercontent.com/joeloverbeck/rulepath/5a489b1d54d1b419db439893f628c4c7e6b410fc/tools/fixture-check/Cargo.toml`
- `https://raw.githubusercontent.com/joeloverbeck/rulepath/5a489b1d54d1b419db439893f628c4c7e6b410fc/tools/fixture-check/src/main.rs`
- `https://raw.githubusercontent.com/joeloverbeck/rulepath/5a489b1d54d1b419db439893f628c4c7e6b410fc/tools/replay-check/Cargo.toml`
- `https://raw.githubusercontent.com/joeloverbeck/rulepath/5a489b1d54d1b419db439893f628c4c7e6b410fc/tools/replay-check/src/main.rs`
- `https://raw.githubusercontent.com/joeloverbeck/rulepath/5a489b1d54d1b419db439893f628c4c7e6b410fc/tools/rule-coverage/Cargo.toml`
- `https://raw.githubusercontent.com/joeloverbeck/rulepath/5a489b1d54d1b419db439893f628c4c7e6b410fc/tools/rule-coverage/src/main.rs`
- `https://raw.githubusercontent.com/joeloverbeck/rulepath/5a489b1d54d1b419db439893f628c4c7e6b410fc/tools/seed-reducer/Cargo.toml`
- `https://raw.githubusercontent.com/joeloverbeck/rulepath/5a489b1d54d1b419db439893f628c4c7e6b410fc/tools/seed-reducer/src/main.rs`
- `https://raw.githubusercontent.com/joeloverbeck/rulepath/5a489b1d54d1b419db439893f628c4c7e6b410fc/tools/simulate/Cargo.toml`
- `https://raw.githubusercontent.com/joeloverbeck/rulepath/5a489b1d54d1b419db439893f628c4c7e6b410fc/tools/simulate/src/main.rs`
- `https://raw.githubusercontent.com/joeloverbeck/rulepath/5a489b1d54d1b419db439893f628c4c7e6b410fc/tools/trace-viewer/Cargo.toml`
- `https://raw.githubusercontent.com/joeloverbeck/rulepath/5a489b1d54d1b419db439893f628c4c7e6b410fc/tools/trace-viewer/src/main.rs`

### External references consulted

- OpenSpiel documentation describes a broad research framework for sequential, simultaneous, stochastic, perfect-information, and imperfect-information games; this is useful vocabulary, not an implementation model for Rulepath.
  - https://openspiel.readthedocs.io/en/latest/intro.html
  - https://arxiv.org/abs/1908.09453
- BoardGameGeek mechanic vocabulary: "Market" and "Contracts" are useful labels for buy/sell rows, prices/quantities, and goal fulfillment rewards. Use the vocabulary only; do not copy commercial game rules.
  - https://boardgamegeek.com/boardgamemechanic/2900/market
  - https://boardgamegeek.com/boardgamemechanic/2912/contracts
- W3C/WAI guidance supports the browser spec's requirement that resource state not be encoded by color alone and that dense interactive grids/rows remain keyboard navigable.
  - https://www.w3.org/WAI/WCAG22/Understanding/use-of-color.html
  - https://www.w3.org/WAI/ARIA/apg/patterns/grid/

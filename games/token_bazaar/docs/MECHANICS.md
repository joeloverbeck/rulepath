# Token Bazaar Mechanics Inventory

Game ID: `token_bazaar`

Roadmap stage/gate: Gate 9 public resource/economy proof

Rules version: `token-bazaar-rules-v1`

Last updated: 2026-06-08

## Purpose

This inventory records Token Bazaar's game-local mechanic shapes and primitive-pressure posture. It is evidence for [../../../docs/MECHANIC-ATLAS.md](../../../docs/MECHANIC-ATLAS.md).

Token Bazaar is a deterministic two-seat, fully public resource-accounting game. Rust owns setup, legal actions, payments, gains, market refill, terminal checks, tie-breaks, effects, replay, and bot choice. TypeScript presents the Rust/WASM projection only.

## Mechanic Inventory

| Category | Game-local description | Evidence | Current status | Notes |
|---|---|---|---|---|
| topology/spatial model | No board topology. The only positional model is three visible market slots: `slot_0`, `slot_1`, `slot_2`. | [RULES.md](RULES.md), `state.rs` | `local-only` | Slot identity stays game-local. |
| component/zone model | Public supply, two public inventories, three market slots, deterministic contract queue, fulfilled-contract lists, scores, and turn counters. | [RULES.md](RULES.md), `setup.rs`, `state.rs` | `local-only` | The queue is deterministic; no shuffle or hidden zone exists. |
| action shape | Flat Rust action paths: `collect/<bundle>`, `exchange/<pay>/<take>`, `fulfill/<slot>`, or forced `pass`. | `actions.rs`, [RULE-COVERAGE.md](RULE-COVERAGE.md) | `local-only` | TypeScript never builds or filters legal actions. |
| turn/phase model | Alternating single active seat with an eight-turn-per-seat cap. | `rules.rs`, golden traces | `local-only` | Terminal may also happen when the market is exhausted. |
| randomness/chance | None in rules or setup. Bot seeds affect bot choice only and resolved commands are replayed. | [SOURCES.md](SOURCES.md), replay traces | `local-only` | No RNG primitive pressure. |
| visibility/hidden information | Fully public: supply, inventories, market, scores, turns, actions, effects, and terminal outcome are public. | `visibility.rs`, e2e no-leak smoke | `local-only` | No private or hidden state exists, but no-leak assertions still run. |
| resource/accounting | Three local resources (`amber`, `jade`, `iron`), exact payments, supply returns, collection, exchange, scoring, and inventory tie-breaks. | [RULES.md](RULES.md), `effects.rs` | `local-only first use` | First official public-economy use. Keep local; atlas records later repeated pressure. |
| movement/capture/placement | None. | [RULES.md](RULES.md) | not applicable | Board-space primitive is not involved. |
| pattern/line/directional scanning | None. | [RULES.md](RULES.md) | not applicable | No pattern helper pressure. |
| commitment/reveal | None. | [SOURCES.md](SOURCES.md) | deferred elsewhere | Gate 9.1 `secret_draft` carries commitment/reveal pressure. |
| reaction/window/pending response | None. | [RULES.md](RULES.md) | not applicable | No pending response window. |
| scoring/outcome | Contract points decide score; tie-breaks are fulfilled-contract count, total inventory, then draw. | `rules.rs`, [RULE-COVERAGE.md](RULE-COVERAGE.md) | `local-only` | All tie-break facts are public. |
| semantic effect shape | Resource collected/exchanged, contract fulfilled, slot refilled/emptied, pass accepted, turn advanced, terminal. | `effects.rs`, golden traces | `local-only` | Effects drive logs, replay, and UI feedback. |
| UI interaction pattern | Board shows inventories, public supply, market slots, Rust legal buttons, recent accounting effects, replay, and reduced-motion path. | [UI.md](UI.md), `TokenBazaarBoard.tsx` | `local-only` | Resource chips include code, name, and count. |
| bot policy pattern | Level 0 random legal plus Level 1 public heuristic: fulfill, collect/exchange toward visible target, deterministic fallback. | [AI.md](AI.md), [BOT-STRATEGY-EVIDENCE-PACK.md](BOT-STRATEGY-EVIDENCE-PACK.md) | `local-only` | No MCTS/ISMCTS/Monte Carlo/ML/RL. |
| benchmark/performance pressure | Legal action generation, apply action, projection, and replay export/import. | [BENCHMARKS.md](BENCHMARKS.md) | `local-only` | Smoke floors are documented; no generic economy benchmark helper. |

## Primitive Pressure Decision

Resource/accounting remains game-local for Gate 9. This is the first official public-economy implementation, so it is not evidence for a `game-stdlib` primitive by itself.

The repeated-shape pressure to watch later is public accounting over costs, gains, supply/pot/budget movement, and effect-visible scoring. A later betting or economy gate may compare Token Bazaar against another implementation, but a third official repeated shape must go through the mechanic atlas ledger before any helper is promoted.

## Conservation And Supply-Return Notes

- Collect moves exact resources from public supply to the active inventory.
- Exchange returns exactly two matching resources to public supply and removes one different resource from supply.
- Fulfill returns the exact contract cost to public supply, awards points, records the fulfilled contract, then refills or empties the slot.
- All accounting transitions are represented by semantic effects and replay hashes.

## Review Checklist

- `engine-core` remains noun-free.
- `game-stdlib` receives no resource, market, supply, contract, payment, or economy primitive in Gate 9.
- Resource/accounting is recorded as first-use-kept-local.
- Browser controls and effect rows present Rust payloads only.

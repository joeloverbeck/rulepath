# Crest Ledger Mechanics Inventory

Game ID: `poker_lite`

Roadmap stage/gate: Gate 10 betting/showdown proof

Rules version: `poker-lite-rules-v1`

Last updated: 2026-06-09

## Purpose

This inventory records Crest Ledger's game-local mechanic shapes and
primitive-pressure posture. It is evidence for
[../../../docs/MECHANIC-ATLAS.md](../../../docs/MECHANIC-ATLAS.md).

Crest Ledger is a deterministic two-seat hidden-information pledge game. Rust
owns setup, legal actions, validation, private crest storage, center reveal
timing, shared-pool accounting, terminal allocation, showdown comparison,
effects, replay, visibility projection, and bot decisions. TypeScript presents
the Rust/WASM projection only.

## Mechanic Inventory

| Category | Game-local description | Evidence | Current status | Notes |
|---|---|---|---|---|
| topology/spatial model | No board topology. The public layout is two seat ledgers around one center crest and a shared contribution track. | [RULES.md](RULES.md), `PokerLiteBoard.tsx` | `local-only` | No board-space primitive pressure. |
| component/zone model | Six local crests, two private crest slots, one hidden/revealed center slot, an internal deck tail, public contributions, and a shared pool. | [RULES.md](RULES.md), `state.rs`, `visibility.rs` | `repeated-shape candidate` | Card/private-hand shape is a second official use after `high_card_duel`; it stays game-local pending the third-use review. |
| action shape | Flat Rust action paths: `hold`, `press`, `lift`, `match`, and `yield`, generated only for the active seated actor. | `actions.rs`, [RULE-COVERAGE.md](RULE-COVERAGE.md) | `local-only` | TypeScript never builds or filters legal choices. |
| turn/phase model | Two pledge rounds; round 1 reveals the center if it closes without yield, round 2 resolves showdown if it closes without yield. | `rules.rs`, golden traces | `local-only` | The round lead changes after center reveal. |
| randomness/chance | Seeded setup shuffle only; no later random draw or stochastic resolution. | `setup.rs`, `replay_support.rs`, golden traces | `repeated-shape candidate` | Similar deterministic shuffle/private-card pressure to `high_card_duel`, still local. |
| visibility/hidden information | Owner seat sees own private crest; observer/opponent do not. Center is hidden until round 1 closes; private crests reveal only at showdown, never on yield. | `visibility.rs`, [RULE-COVERAGE.md](RULE-COVERAGE.md), `poker-lite.smoke.mjs` | `repeated-shape candidate` | The no-leak surface spans Rust payloads, traces, WASM, DOM, storage, replay export, and dev panel. |
| resource/accounting | Opening contributions, exact marker additions, public shared pool, one-lift cap per round, yield award, showdown win, and exact split. | `rules.rs`, [RULES.md](RULES.md) | `repeated-shape candidate` | Public accounting is a second official use after `token_bazaar`; bounded pledge/shared-pool pressure is first use. |
| movement/capture/placement | Not applicable; no component movement or capture exists. | [RULES.md](RULES.md) | `local-only` | Crests are dealt/revealed, not moved by players. |
| pattern/line/directional scanning | Not applicable; no line, adjacency, trick, route, or directional scan exists. | [RULES.md](RULES.md) | `local-only` | Showdown comparison is a direct rank/pair comparison. |
| commitment/reveal | Hidden center reveal after round 1 and grouped showdown reveal after round 2. Yield terminal reveals no private crests. | `effects.rs`, `rules.rs`, golden traces | `repeated-shape candidate` | Reveal timing stays local; grouped public reveal is critical no-leak evidence. |
| reaction/window/pending response | Facing an outstanding pledge creates a required response choice: match, yield, or lift if the cap remains. | `actions.rs`, `rules.rs` | `local-only first use` | This is bounded pledge pressure, not a generic reaction primitive. |
| scoring/outcome | Yield win, showdown win by pair flag then private rank, or exact split on equal strength. | `rules.rs`, [RULE-COVERAGE.md](RULE-COVERAGE.md) | `local-only` | Copy identity never breaks ties. |
| semantic effect shape | Private deal, public setup, pledge held/pressed/lifted/matched, yield, center reveal, showdown reveal, ledger resolution, terminal, and bot choice. | `effects.rs`, golden traces | `local-only` | Effects drive logs, replay, and UI feedback. |
| UI interaction pattern | Board shows public ledger, center status, private owner view, legal Rust action buttons, grouped showdown, replay, and reduced-motion path. | [UI.md](UI.md), `PokerLiteBoard.tsx` | `local-only` | `data-testid` anchors use round/index, not hidden crest ids. |
| bot policy pattern | Level 0 random legal plus Level 2 authored priorities over own private rank, public center rank, price, cap, and survival. | [AI.md](AI.md), [BOT-STRATEGY-EVIDENCE-PACK.md](BOT-STRATEGY-EVIDENCE-PACK.md) | `local-only` | No MCTS, ISMCTS, Monte Carlo, ML, RL, or opponent hidden-card sampling. |
| benchmark/performance pressure | Legal action generation, validate/apply, projection, public export, replay, simulation, and Level 2 choice. | [BENCHMARKS.md](BENCHMARKS.md) | `local-only` | Native benchmark evidence exists; no shared optimization primitive is promoted. |

## Primitive Pressure Decision

Card/private-hand visibility remains local for Gate 10. It is the second
official similar shape after `high_card_duel`, but Crest Ledger adds public
shared-pool pressure and staged center reveal. The next card/private-hand game,
especially `plain_tricks` if it uses the same primitive shape, must revisit the
third-use hard gate before extracting anything.

Public accounting remains local for Gate 10. It is the second official similar
shape after `token_bazaar`, but Crest Ledger's contribution ledger, yield
terminal, and split allocation differ from Token Bazaar's economy rules.

Bounded pledge rounds and shared-pool allocation are first-use local mechanics.
They require atlas and per-game primitive-pressure ledger notes, but they do not
justify a `game-stdlib` or `engine-core` primitive by themselves.

## Repeated-Shape Comparison

| Mechanic shape | Already appears in | Same shape? | Similarities | Differences | Required next step |
|---|---|---:|---|---|---|
| deterministic card/private-hand projection | `high_card_duel` | no | Seeded card setup, owner/private/public projection, redacted replay/export evidence. | Crest Ledger has a hidden center, two pledge rounds, grouped showdown, and yield without reveal. | Record second-use pressure; review before third use. |
| public resource/accounting ledger | `token_bazaar` | no | Public counts, deterministic accounting, exact terminal allocation. | Crest Ledger uses per-seat contributions, a shared pool, and pledge responses rather than token-market exchange rules. | Record second-use pressure; review before third economy/accounting use. |
| hidden reveal timing | `secret_draft`, `high_card_duel` | similar | Hidden facts become public only at rule-defined reveal points. | Crest Ledger has owner-private knowledge and a staged center reveal before optional grouped showdown. | Keep local; no generic reveal helper. |

## Review Checklist

- `engine-core` remains noun-free.
- `game-stdlib` receives no card, deck, private-hand, pledge, pool, lift, yield,
  or showdown primitive in Gate 10.
- Static data carries labels, metadata, fixtures, traces, and thresholds only;
  all legality and accounting stay in Rust.
- Browser controls and effect rows present Rust payloads only.
- Pre-reveal hidden crest ids/ranks/labels stay out of observer and opponent
  browser surfaces.
- `docs/MECHANIC-ATLAS.md` and `PRIMITIVE-PRESSURE-LEDGER.md` closeout is
  tracked by GAT10POKLITBET-018.

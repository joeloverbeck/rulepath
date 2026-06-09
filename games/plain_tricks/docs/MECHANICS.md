# Plain Tricks Mechanics Inventory

Game ID: `plain_tricks`

Roadmap stage/gate: Gate 10.1 trick-taking proof

Rules version: `plain-tricks-rules-v1`

Last updated: 2026-06-09

## Purpose

This inventory records Plain Tricks' game-local mechanic shapes and primitive
pressure posture. It is evidence for
[../../../docs/MECHANIC-ATLAS.md](../../../docs/MECHANIC-ATLAS.md).

Plain Tricks is a deterministic two-seat hidden-hand trick-taking microgame.
Rust owns setup, legal actions, validation, hidden hands and tail, follow-suit
legality, trick resolution, round scoring, deal rotation, terminal outcome,
semantic effects, replay, visibility projection, and bot decisions. TypeScript
presents the Rust/WASM projection only.

## Mechanic Inventory

| Category | Game-local description | Evidence | Current status | Notes |
|---|---|---|---|---|
| topology/spatial model | No board topology. The public layout is two seat zones around one current-trick surface and a trick-history ledger. | [RULES.md](RULES.md), [UI.md](UI.md) | `local-only` | No grid, path, adjacency, or movement primitive. |
| component/zone model | Eighteen local trick cards, two private hands, one internal tail, one current trick, and public trick history. | [RULES.md](RULES.md), `state.rs`, `visibility.rs` | `third-use pressure` | Card/private-hand visibility now appears across High Card Duel, Crest Ledger, and Plain Tricks; capstone reconciliation belongs to GAT101PLATRI-020. |
| action shape | Nested Rust action paths: `play/<card-id>`, generated only for the active seated actor. | `actions.rs`, [RULE-COVERAGE.md](RULE-COVERAGE.md) | `local-only` | TypeScript maps Rust leaves to buttons; it never decides legal cards. |
| turn/phase model | Six tricks per round, two rounds, trick winner leads next trick, round 2 starts with `seat_1`. | `rules.rs`, golden traces | `local-only` | No extra bidding, pass window, or reaction stack. |
| randomness/chance | Seeded setup shuffle for each round; round 2 continues the same RNG stream. | `setup.rs`, `replay_support.rs`, golden traces | `repeated-shape candidate` | The shuffle/private deal remains game-local. |
| visibility/hidden information | Owner sees own unplayed hand; opponent and observer see counts only; tail is internal only; played cards are public. | `visibility.rs`, [RULE-COVERAGE.md](RULE-COVERAGE.md), `plain-tricks.smoke.mjs` | `third-use pressure` | No-leak scope includes Rust payloads, WASM, DOM, storage, replay export, and dev panel. |
| follow-suit legality | Leader may play any held card; follower must play led suit if holding it, otherwise any held card. | `actions.rs`, `rules.rs` | `local-only first use` | This is trick-taking pressure, not an engine-core primitive. |
| trick resolution | Led-suit higher rank wins; off-suit follower loses to leader; winner leads next trick. | `rules.rs`, [RULES.md](RULES.md) | `local-only first use` | No trump, partnerships, equal-card tie, or point cards. |
| scoring/outcome | One point per trick, totals across two rounds, higher total wins, 6-6 split. | `rules.rs`, [RULE-COVERAGE.md](RULE-COVERAGE.md) | `local-only` | Outcome rationale is public and count-based. |
| semantic effect shape | Deal counts/private hand-dealt effects, card played, trick resolved, round scored, deal rotated, match resolved, terminal, and public bot choice. | `effects.rs`, golden traces | `local-only` | Private hand-dealt effects are viewer-filtered. |
| UI interaction pattern | Own hand, opponent face-down count, current trick, history ledger, Rust-legal card buttons, replay, reduced motion. | [UI.md](UI.md), `PlainTricksBoard.tsx` | `local-only` | `data-testid` anchors use trick/index, not card ids. |
| bot policy pattern | Level 0 random legal plus Level 2 follow-suit/trick-aware authored priorities. | [AI.md](AI.md), [BOT-STRATEGY-EVIDENCE-PACK.md](BOT-STRATEGY-EVIDENCE-PACK.md) | `local-only` | No MCTS, ISMCTS, Monte Carlo, ML, RL, or opponent-hand sampling. |
| benchmark/performance pressure | Legal action generation, validate/apply, projection, public export, replay, simulation, and Level 2 choice. | [BENCHMARKS.md](BENCHMARKS.md) | `local-only` | Native benchmark lane exists. |

## Primitive Pressure Decision

Follow-suit legality, trick-winner-led turn order, and trick scoring remain
Plain Tricks-local in Gate 10.1. They are first official uses and do not justify
`engine-core` or `game-stdlib` promotion.

Card/private-hand visibility has now reached third-use pressure across
`high_card_duel`, `poker_lite`, and `plain_tricks`. This ticket records the
pressure; GAT101PLATRI-020 owns the capstone atlas/status reconciliation and
must decide whether the repeated shape remains documented local pressure or
earns a shared helper.

## Review Checklist

- `engine-core` remains noun-free.
- `game-stdlib` receives no card, hand, suit, rank, trick, follow-suit, tail, or
  deal primitive in this ticket.
- Static data carries labels, fixtures, traces, and version declarations only;
  all legality and scoring stay in Rust.
- Browser controls and effect rows present Rust payloads only.
- Unplayed opponent hand and tail identities stay out of browser-facing
  surfaces.

# High Card Duel Mechanics Inventory

Game ID: `high_card_duel`

Roadmap stage/gate: Gate 8 chance and hidden-information proof

Rules version: `high-card-duel-rules-v1`

Last updated: 2026-06-08

## Purpose

This inventory records High Card Duel's game-local mechanic shapes and primitive-pressure posture. It is evidence for [../../../docs/MECHANIC-ATLAS.md](../../../docs/MECHANIC-ATLAS.md).

High Card Duel is a deterministic two-seat hidden-information card game. Rust owns shuffle, deal, private hands, face-down commitments, reveal, scoring, effects, replay, bot choice, and viewer filtering.

## Mechanic Inventory

| Category | Game-local description | Evidence | Current status | Notes |
|---|---|---|---|---|
| card/deck identity | Local 24-card deck, ranks 1-12, two neutral sigils per rank, stable IDs `hcd:rNN:a/b`. | [RULES.md](RULES.md), `ids.rs` | `local-only` | First official local card/deck use; no promotion authorized. |
| deterministic chance | Fisher-Yates shuffle from Rust `Seed`, unbiased bounded indices, canonical deck before shuffle. | [RULES.md](RULES.md), setup tests, replay traces | `local-only` | Public surfaces expose deck count, not order. |
| private hands | Each seat has an owner-private hand; observer sees counts only. | `visibility.rs`, no-leak tests | `local-only` | TypeScript renders viewer-safe projection only. |
| hidden commitment | Active seat commits one own card face-down; opponent and observer see occupancy, not identity. | `actions.rs`, `rules.rs`, WASM no-leak test | `local-only` | Action paths are private actor data. |
| simultaneous reveal | When both commitments exist, Rust reveals both cards together and scores the round. | `rules.rs`, golden traces | `local-only` | Reveal effects drive browser animation. |
| scoring/outcome | Higher rank scores one point; ties score no point; six rounds then high score wins or draw. | [RULE-COVERAGE.md](RULE-COVERAGE.md) | `local-only` | No betting, blackjack, poker, or casino semantics. |
| visibility/effects | Public effects exclude private card IDs; private effects are addressed to one seat. | `effects.rs`, `EffectLog`, browser smoke | `local-only` | No hidden IDs in DOM, logs, storage, replay export, or dev panel. |
| replay export | Internal full trace is native evidence; WASM default is public observer projection. | [../../../docs/adr/0004-hidden-info-replay-export-taxonomy.md](../../../docs/adr/0004-hidden-info-replay-export-taxonomy.md) | `local-only` | Public export is no-leak by default and not seed-reconstructable. |
| bot policy | Level 0 random legal bot chooses from its own legal tree and private hand only. | [AI.md](AI.md), bot tests | `local-only` | No candidate rankings or explanations are exposed. |
| UI pattern | Viewer selector, own-hand-only display, backs/counts for others, reduced-motion reveal. | [UI.md](UI.md), `HighCardDuelBoard.tsx`, e2e smoke | `local-only` | Legal actions come only from Rust action tree. |

## Primitive Pressure Decision

High Card Duel creates first official pressure for card/deck/hand/commitment primitives, but this is not enough for `game-stdlib` promotion. The local design is intentionally narrow: one deck, no discard choice, no piles, no suits, no trick rules, no betting, no reaction windows, and no public reconstructable hidden trace.

Future pressure points named in the atlas are poker-lite, trick-taking, later hidden draw/discard or commitment/reaction games, and a deferred Blackjack comparison case only after resource/accounting and casino-adjacent scope are intentionally admitted. Promotion remains blocked until repeated games prove a behavior-free helper that preserves hidden-information safety and replay determinism.

## Review Checklist

- `engine-core` remains noun-free.
- `game-stdlib` receives no card/deck helper in Gate 8.
- Public projections never reveal private hands, deck order, or face-down commitments.
- Browser-visible labels, test IDs, dev panels, console logs, and replay exports stay no-leak.

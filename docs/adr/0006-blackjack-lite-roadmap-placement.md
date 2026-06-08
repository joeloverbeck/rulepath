# ADR: Blackjack Lite Roadmap Placement

Status: Accepted

Date: 2026-06-08

Decision owner: Rulepath maintainers

Related documents:

- `docs/FOUNDATIONS.md`
- `docs/ARCHITECTURE.md`
- `docs/ENGINE-GAME-DATA-BOUNDARY.md`
- `docs/OFFICIAL-GAME-CONTRACT.md`
- `docs/MECHANIC-ATLAS.md`
- `docs/AI-BOTS.md`
- `docs/UI-INTERACTION.md`
- `docs/TESTING-REPLAY-BENCHMARKING.md`
- `docs/ROADMAP.md`
- `docs/IP-POLICY.md`
- `docs/AGENT-DISCIPLINE.md`
- `specs/README.md`
- `archive/specs/gate-7-2-and-gate-8-high-card-duel-hidden-info-chance-proof.md`
- `games/high_card_duel/docs/MECHANICS.md`

## Context

Gate 8 has been implemented with `high_card_duel`. The archived Gate 7.2/Gate 8 spec intentionally used `high_card_duel` first because it isolates deterministic shuffle, hidden private hands, face-down commitments, viewer-scoped legal actions, filtered effects, no-leak public replay export, bot fairness, and public card UI without Blackjack's rule baggage. The same spec required the Blackjack continuation point to be explicitly resolved before Gate 9 proceeded.

The target commit still contained several roadmap-facing references that could let future agents treat `blackjack_lite` as an immediate Gate 8.1 implementation target: `docs/ROADMAP.md` paired it with `high_card_duel`, `docs/MECHANIC-ATLAS.md` named it as near pressure for card/deck helper comparison, `docs/TESTING-REPLAY-BENCHMARKING.md` assigned it a Gate 8 benchmark target, and `docs/IP-POLICY.md` used it as the safer name for simple draw/stand scoring. `specs/README.md` marked the post-Gate-8 checkpoint `Done`, but still described a Gate 8.1 reconsideration before Gate 9.

External rules research confirms that Blackjack is not merely a draw/stand card comparison. Standard and common variants include dealer automation, dealer hole-card or no-hole-card policy, soft-hand valuation, naturals, busts, pushes, dealer busts, settlement, insurance, even money, double, split, surrender, multiple player hands, multiple players against a dealer, and payout/betting/casino framing. Even stripped academic environments model Blackjack as a stochastic decision process with hit/stick actions, dealer reveal/draw policy, usable ace observations, and reward/terminal rules.

An ADR is required because the roadmap says a stage or gate may be skipped or reordered only by accepted ADR, and because reclassifying `blackjack_lite` changes roadmap semantics rather than merely progress tracking.

## Decision

`blackjack_lite` MUST NOT be treated as a Gate 8 or Gate 8.1 implementation target. Gate 8 is satisfied by `high_card_duel`. No Blackjack implementation interlock blocks Gate 9.

`blackjack_lite` is retained only as a deferred comparison label for a later Gate 10-or-later audit, where it may be useful to compare card/deck/hidden-information/resource/accounting pressure against `poker_lite`, trick-taking, or other official card games.

A future public implementation SHOULD NOT use `blackjack_lite` as its public product name by default. If Rulepath wants a draw/stand threshold proof before any casino-adjacent game, it SHOULD propose an original non-casino microgame with original naming, neutral presentation, no betting/chips/payouts, and a narrow Rust-owned rule contract. That proposal requires a spec and MAY require a new ADR if it changes gate order.

Any future Blackjack-family admission MUST state explicitly:

- whether betting, chips, payouts, bankrolls, pots, or side bets are excluded;
- whether split, double, surrender, insurance, and even money are excluded;
- whether the dealer has a hole card or uses a no-hole-card protocol;
- whether the dealer hits or stands on soft 17;
- whether naturals exist and how they settle;
- how aces and soft hands are valued;
- how bust, push, dealer bust, and terminal settlement work;
- whether multiple players or multiple hands exist;
- how public/private/replay exports handle dealer hidden information;
- what public name and UI avoid casino/trade-dress direction;
- which mechanics are proved that `high_card_duel` did not already prove.

## Alternatives considered

| Alternative | Why considered | Why accepted/rejected |
|---|---|---|
| Keep `blackjack_lite` immediately after Gate 8 | It could provide a second card/deck game and draw/stand pressure. | Rejected. Even a narrow version forces dealer policy, valuation, terminal settlement, and variant decisions before Gate 9's intended resource/simultaneous ladder. It also risks casino-adjacent product drift. |
| Move `blackjack_lite` later as a normal named candidate | It preserves a useful pressure test without blocking Gate 9. | Partially accepted. It remains a deferred comparison label, not a public product name or committed implementation target. |
| Keep `blackjack_lite` as a deferred comparison case only | It records source-grounded pressure while avoiding immediate scope creep. | Accepted. This closes the checkpoint and keeps the public-product ladder moving. |
| Replace it now with an original draw/stand microgame | A neutral original game could isolate the useful threshold mechanic. | Rejected as the immediate next target. It is a good future option if the ladder later needs draw/stand pressure, but Gate 9 should proceed first. |
| Add simultaneous bid/auction or draft candidates now | Goofspiel-like and secret-draft games separate simultaneous commitment from dealer/accounting pressure. | Deferred to Gate 9 planning. The existing roadmap already names resource and simultaneous-choice candidates. |

## Consequences

Positive consequences:

- Gate 8 stays focused on the already-implemented hidden-information/chance proof.
- Gate 9 is unblocked without pretending Blackjack was implemented.
- Future agents cannot treat `blackjack_lite` as mandatory Gate 8.1 work.
- Resource/accounting and simultaneous-choice pressure remain in the intended Gate 9 band.
- Public naming and UI avoid unnecessary casino vibes.
- `engine-core` and `game-stdlib` stay protected from premature card/deck/accounting abstractions.

Negative or risky consequences:

- Rulepath does not get an immediate second card/deck implementation for primitive comparison.
- Draw/stand threshold valuation remains unproved until a later original game or deferred Blackjack audit.
- A future agent may still be tempted to reintroduce Blackjack as a product candidate; the corrected docs must be explicit.

Operational requirements:

- Update `docs/ROADMAP.md` to make Gate 8 `high_card_duel` only and defer `blackjack_lite`.
- Update `specs/README.md` so the checkpoint is closed by this ADR and does not block Gate 9.
- Update `docs/MECHANIC-ATLAS.md` so Blackjack is not a Gate 8.1 promotion trigger.
- Update `docs/SOURCES.md` with the Blackjack-placement research references.
- Update `docs/TESTING-REPLAY-BENCHMARKING.md` so Blackjack has no Gate 8 benchmark target.
- Update `docs/IP-POLICY.md` so draw/stand threshold scoring uses an original non-casino name by default.
- Update `games/high_card_duel/docs/MECHANICS.md` so its primitive-pressure note matches the deferred comparison decision.

## Determinism impact

This ADR changes documentation only. It does not affect RNG, iteration order, clocks, floating point, parallelism, serialization order, replay, or hashes.

A future draw/stand or Blackjack-family implementation must define deterministic shuffle/draw/dealer policy in Rust and must test same-seed replay, setup, legal action generation, view generation, effect filtering, and public/private export behavior.

## Replay/hash impact

This ADR changes no command streams, state hashes, effect hashes, action-tree hashes, public/private view hashes, trace format, or migration rules. Existing golden traces are preserved.

A future Blackjack-family implementation must not export seed plus command stream as a public replay if that would reconstruct private cards, dealer hole cards, hidden deck order, or hidden unused deck tails.

## Visibility impact

This ADR reduces visibility risk by not introducing a dealer hole card, private player hands beyond `high_card_duel`, hidden deck order, betting state, side-bet state, bot belief output, or terminal hidden-state reveal.

A future Blackjack-family implementation must prove that browser payloads, DOM, local storage, logs, diagnostics, previews, replay exports, bot explanations, candidate rankings, and dev tools do not leak hidden information.

## Data/Rust boundary impact

No static data field, hand-authored behavior format, expression, selector, or schema is introduced. Behavior remains in typed Rust.

A future Blackjack-family implementation must not put hit/stand policy, dealer policy, ace valuation, settlement, payout, split/double/surrender/insurance legality, or bot strategy in TOML/JSON/YAML/data. YAML and DSL work remain out of scope unless a later accepted ADR changes that law.

## `engine-core` contamination risk

This ADR explicitly avoids adding `card`, `deck`, `hand`, `pile`, `bet`, `bankroll`, `dealer`, `hole_card`, `soft_17`, `insurance`, `payout`, `pot`, `rank`, `suit`, or equivalent mechanic nouns to `engine-core`.

The Blackjack question belongs in game docs, roadmap docs, and later gate specs. It is not a kernel concern.

## `game-stdlib` / primitive-pressure impact

`high_card_duel` remains the first official card/deck/hidden-hand/commitment use. That is not enough for `game-stdlib` promotion.

`blackjack_lite` is not counted as a second official implementation because it is not implemented. It is a deferred comparison case only. Card/deck/hand/commitment helpers remain game-local until repeated implemented official games prove a behavior-free helper and the atlas/ledger admits it.

## UI impact

No UI changes are required by this ADR. The public UI remains presentation-only.

Future draw/stand or Blackjack-family UI must avoid casino table layouts, felt/chip/payout motifs, copied card art, copied rulebook presentation, and hidden-state leakage through labels, `data-testid`, CSS classes, dev panels, animations, or logs.

## Bot impact

No bot changes are required by this ADR. Public v1/v2 bots still exclude MCTS, ISMCTS, Monte Carlo rollout/search bots, ML, RL, and runtime LLM move selection.

A future draw/stand or Blackjack-family bot must choose only from Rust-owned legal action APIs and viewer-authorized inputs. It must not serialize belief state, candidate rankings, hidden deck inference, or private diagnostics into public outputs unless separately proven no-leak and accepted by the bot-policy docs.

## IP impact

This ADR lowers IP/product risk. `blackjack_lite` is a source-research comparison label, not a default public product name. Public draw/stand threshold work should use original naming, original prose, and neutral presentation unless human/legal review and a future ADR justify an exception.

Do not copy Blackjack rules prose, casino table layouts, screenshots, chip/felt visuals, card art, fonts, icons, or trade dress.

## Benchmark impact

No benchmark changes are required by this ADR beyond removing `blackjack_lite` from Gate 8's provisional benchmark target.

A future draw/stand/dealer/accounting game must define native benchmarks for setup/shuffle, legal action generation, validation, action application, dealer automation, valuation/settlement, view generation, effect filtering, replay/export, bot decision latency, and browser smoke performance as applicable.

## Migration notes

Existing docs to update:

- `docs/ROADMAP.md`
- `specs/README.md`
- `docs/MECHANIC-ATLAS.md`
- `docs/SOURCES.md`
- `docs/TESTING-REPLAY-BENCHMARKING.md`
- `docs/IP-POLICY.md`
- `games/high_card_duel/docs/MECHANICS.md`

Existing games to back-port:

- None. This is documentation-only.

Existing traces to preserve or update:

- Preserve all traces. No behavior or format migration is authorized.

Existing data/schema versions to bump:

- None.

Existing public UI behavior to migrate:

- None.

## Review checklist

Before accepting this ADR, verify:

- the decision supports public playable Rulepath before engine research;
- Rust remains behavior authority;
- TypeScript does not decide legality;
- `engine-core` remains noun-free;
- `game-stdlib` remains earned and narrow;
- static data remains content/parameters, not behavior;
- replay determinism is preserved or migration is explicit;
- visibility boundaries remain safe;
- bots remain fair and explainable;
- benchmarks exist for hot paths when a future game is admitted;
- IP/public-private boundaries are preserved;
- affected foundation docs and per-game docs are updated.

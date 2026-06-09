# Rulepath Official Game Contract

Status: requirements and evidence law for official game modules.

A Rulepath game is official only when it is correct, documented, replayable, test-covered, benchmarked, bot-supported, IP-safe, and ready for the public product role assigned by the roadmap. It is not official merely because it can be clicked in the browser.

## 1. Official game definition of done

Every official game MUST have:

| Area | Required evidence |
|---|---|
| Rules | Typed Rust setup, legal action generation, validation, transitions, scoring, terminal detection, deterministic randomness, semantic effects. |
| Documentation | Original Rulepath rules summary, player-facing how-to-play prose, source notes, variant decision, rule coverage matrix, mechanic inventory, UI notes, bot notes, benchmark notes. |
| Tests | Unit, named rule, golden trace, property/invariant, simulation, replay/hash, serialization, bot legality, and UI smoke tests once web-exposed. |
| Replay | Seed/options/command stream can reproduce state/effect/action-tree/view hashes. |
| Simulation | Native CLI random legal simulation with seed failure output. |
| Benchmarks | Native benchmarks for legal actions, apply, view/effect filtering, replay/serialization, simulation throughput, and bot decisions where relevant. |
| Bots | Level 0 random legal bot at minimum; higher levels when public role requires them. |
| UI metadata | Rust/static typed metadata sufficient for legal controls, labels, accessibility, previews, effects, and replay. |
| IP | Public-domain/classic, original, or permissioned game; original prose/assets; source notes; neutral naming when needed. |

Hidden-information games additionally require no-leak tests for public/private views, action trees, previews, diagnostics, effect logs, serialized payloads, UI fixtures, DOM-safe attributes, local storage, replay exports, bot views, bot explanations, and dev candidate rankings.

## 2. Readiness labels

Use these labels honestly:

| Label | Meaning | Public exposure |
|---|---|---|
| `experimental` | Local exploration; may not satisfy official contract. | Not a public game. |
| `foundation-smoke` | Official tiny/scaffolding game proving architecture plumbing. | May be public if labeled modestly and still fully supported. |
| `official` | Satisfies this contract. | May be in public catalog. |
| `showcase` | Official plus polished UI, public-friendly bot, replay UI, accessibility, visual quality, and benchmark confidence. | Portfolio-facing. |
| `private-red-team` | Private licensed/commercial stress test, isolated and non-public. | Never public. |

A `foundation-smoke` game may be visually modest. It still needs full official evidence.

## 3. Requirements-first implementation workflow

A public game SHOULD move through this workflow:

```text
rules research
  -> source notes
  -> original Rulepath rules summary
  -> variant scope and naming decision
  -> rule coverage matrix
  -> mechanic inventory
  -> primitive-pressure comparison
  -> competent-player analysis if useful
  -> Level 2 strategy evidence pack if Level 2 bot planned
  -> typed Rust rules and tests
  -> semantic effects and visibility tests
  -> replay/golden traces/serialization
  -> random legal simulation and benchmarks
  -> bot implementation and bot tests
  -> UI metadata and UI smoke
  -> public polish review
```

Do not start with UI and backfill rules later. UI affordances are downstream of Rust legal actions, previews, effects, and viewer-safe views.

## 4. Rules research and source notes

Before public exposure, every game MUST document sources consulted.

Source notes SHOULD record:

```text
Source: name + URL or bibliographic identifier
Consulted: YYYY-MM-DD
Used for: rule verification / variant comparison / historical note / terminology check
Copied prose/assets: none unless explicitly permissioned and reviewed
Variant choice: selected variant and excluded variants
Rulepath deviations: deliberate simplifications or changes
Public name rationale: common/neutral/permissioned
Asset status: original / project-owned / compatible license / generated-reviewed
Open questions: unresolved ambiguities
```

Source notes do not grant permission to copy prose, assets, component text, screenshots, scans, or trade dress.

## 5. Original Rulepath rules summary

`RULES.md` for each official game MUST be original prose written for Rulepath. It MUST NOT paste rulebook text.

It SHOULD include:

- player count and setup;
- components using safe names;
- objective;
- turn/phase structure;
- legal actions;
- mandatory actions;
- scoring and terminal conditions;
- variant scope;
- hidden-information rules if any;
- deliberate simplifications;
- glossary of game-local terms.

Rules sections SHOULD have stable IDs or headings used by rule tests and coverage rows.

### Player-facing rules document

Every official catalog game MUST include `games/<game_id>/docs/HOW-TO-PLAY.md`.

`HOW-TO-PLAY.md` is original Rulepath prose for players. It teaches the goal,
setup summary, turn flow, action meanings, scoring/winning, and
hidden-information/reveal timing. It is the only per-game rules prose intended
for the shared web How to Play / Rules surface.

`RULES.md` remains the formal rule contract and Rust validation authority. The
web app MUST NOT render `RULES.md` directly as player help.

`HOW-TO-PLAY.md` MUST NOT duplicate `COMPETENT-PLAYER.md` strategy guidance.

Hidden-information games MUST describe visibility from the player's own
perspective and public perspective without exposing opponent secrets, deck
tails, unrevealed commitments, or seed-derived hidden data.

## 6. Rule coverage matrix

Every rule requirement MUST be classified.

| Status | Meaning |
|---|---|
| `covered` | Implemented and tested. |
| `covered-by-trace` | Covered by a golden trace plus supporting tests. |
| `not-applicable` | Source rule does not apply to the chosen variant. |
| `intentionally-deferred` | Not in current scope; reason and gate recorded. |
| `unsupported` | Not supported and not promised. |
| `open` | Ambiguity remains; game is not showcase-ready. |

No silent gaps. A public showcase game SHOULD have no `open` rows.

## 7. Mechanic inventory

Every official game MUST maintain `MECHANICS.md` and update the repo-level [MECHANIC-ATLAS.md](MECHANIC-ATLAS.md) when repeated shapes appear.

When the atlas records a promoted `game-stdlib` primitive whose scope matches an official game, the game is not contract-clean until it either uses that primitive or has an accepted atlas exception. This applies retroactively to earlier official games when the primitive is promoted later. Public behavior, traces, replay hashes, action order, diagnostics, semantic effects, visibility, bot legality, and UI surfaces remain stable by default during conformance work.

The inventory MUST cover topology/spatial model, components/zones, action shape, turn/phase model, randomness, visibility, resources/accounting, movement/capture/placement, pattern/directional scanning, commitment/reveal, reaction windows, scoring/outcome, semantic effects, UI interaction pattern, bot policy pattern, and benchmark pressure.

Game-specific nouns are correct inside game inventories. Shared atlas entries should describe mechanic shapes, not commercial product identities.

## 8. Competent-player document workflow

For public games with clear strategic texture, the workflow SHOULD include a human/LLM-produced “what makes a competent player at this game?” document before a Level 2 bot is coded.

This document is not behavior authority. It is strategy research input.

It SHOULD include:

- sources and play observations used;
- novice traps;
- immediate tactics;
- medium-term priorities;
- phase changes;
- visible threats and opportunities;
- risk posture options;
- hidden-information inference boundaries if any;
- examples of competent choices;
- examples of intentionally weak but plausible choices;
- forbidden shortcuts and hidden information.

The bot author then converts this into a formal Level 2 strategy evidence pack as defined in [AI-BOTS.md](AI-BOTS.md). No evidence pack, no Level 2 bot.

## 9. Bot requirements by public role

| Game role | Minimum bot |
|---|---|
| Official game | Level 0 random legal bot. |
| Serious public demo | Level 1 rule-informed bot. |
| Showcase/polished public game | Level 2 authored policy bot preferred. |
| Small perfect-information game | Level 3 shallow deterministic search MAY be allowed with benchmarks and explicit limits. |

Public v1/v2 MUST NOT use MCTS, ISMCTS, Monte Carlo bots, ML, or RL.

## 10. UI exposure requirements

A game may be web-exposed only when:

- Rust supplies viewer-safe public/private views;
- Rust supplies legal action trees and diagnostics;
- Rust supplies safe previews for partial/compound actions;
- Rust emits semantic effects sufficient for normal animation/replay;
- UI metadata is present;
- `games/<game_id>/docs/HOW-TO-PLAY.md` exists and is rendered by the shared web How to Play / Rules surface rather than rendering formal `RULES.md`;
- UI smoke tests cover start, legal action display, one human action, one bot action where applicable, effects, replay stepping, safe dev toggle, and reduced-motion behavior once animation exists;
- hidden-information games prove no leak through browser-facing payloads and DOM-safe fixtures;
- the web-shell catalog README ([`../apps/web/README.md`](../apps/web/README.md)) names the newly web-exposed game in its intro catalog list, its Shell Surface renderer list (when the game ships a board renderer), and its Smoke Layers `smoke:e2e` list (when the game's smoke is chained by `smoke:e2e`). This reconciliation is part of web-exposure done, not a later cleanup pass — `scripts/check-catalog-docs.mjs` enforces the intro and smoke lists mechanically against the `crates/wasm-api` catalog and `apps/web/package.json`.

Public UI polish is not optional for showcase games.

## 11. Required trace set

Each official game SHOULD include at least:

- shortest normal/representative trace;
- terminal trace;
- bot-action trace;
- invalid/stale diagnostic trace when applicable;
- stochastic trace when randomness exists;
- redacted hidden-information trace when hidden information exists;
- stage-specific mechanic trace from the roadmap gate.

Trace updates require notes explaining whether behavior changed, effect contracts changed, view projection changed, hash format changed, or only formatting/tooling changed.

## 12. Official game acceptance check

Before marking a game official, verify:

- the game satisfies [FOUNDATIONS.md](FOUNDATIONS.md) universal invariants;
- rule docs and implementation match;
- player-facing `HOW-TO-PLAY.md` exists, is original prose, and matches the formal rules version it cites;
- source notes are complete and IP-safe;
- rule coverage has no silent gaps;
- mechanic inventory is complete;
- atlas/ledger pressure is updated;
- any matching promoted `game-stdlib` primitive is used, or a named atlas exception explains why not;
- random legal bot exists;
- non-random bots have evidence, tests, explanations, and benchmarks;
- replay determinism passes;
- serialization tests pass;
- visibility/no-leak tests pass where relevant;
- CLI simulation can reproduce failing seeds;
- native benchmarks exist;
- UI smoke tests pass if web-exposed;
- if web-exposed, the web-shell catalog README ([`../apps/web/README.md`](../apps/web/README.md)) names the game across its intro, renderer, and smoke lists, and `scripts/check-catalog-docs.mjs` passes;
- public presentation is neutral/original and does not imitate trade dress.

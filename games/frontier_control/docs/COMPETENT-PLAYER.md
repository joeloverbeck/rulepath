# Frontier Control Competent Player Analysis

Game ID: `frontier_control`

Implemented variant: `frontier_control_standard` and `frontier_control_highlands`

Rules version checked: `frontier-control-rules-v1`

Prepared by: `Codex`

Date: 2026-06-11

## Purpose and authority

This document records original Rulepath strategy analysis for the implemented
Frontier Control variants. It is not rule authority. If this document conflicts
with `RULES.md`, `RULES.md` and the Rust implementation win.

This gate ships Level 0 random legal bots and Level 1 rule-informed bots only.
The strategy notes below explain the public-information priorities those bots
approximate; they do not claim Level 2 authored-policy strength.

## Sources and consulted strategy references

| Source/reference | URL/reference | Date consulted | Source quality | Used for | Copied prose status | Notes |
|---|---|---|---|---|---|---|
| Frontier Control formal rules | `games/frontier_control/docs/RULES.md` | 2026-06-11 | rules authority | strategy, tactics, terminology | none | Stable rule IDs are the cross-check authority. |
| Gate 13 proof spec | `specs/gate-13-frontier-control-asymmetric-area-control-proof.md` | 2026-06-11 | project spec | balance target, bot posture | none | Assumption A5 defines the 35-65% Level-1-vs-Level-1 balance band. |
| Implemented bot policies | `games/frontier_control/src/bots.rs` | 2026-06-11 | implementation | Level 1 policy fidelity | none | The docs describe the current public-view policies, not an idealized future policy. |
| Bot and property tests | `games/frontier_control/tests/bots.rs`, `games/frontier_control/tests/property.rs` | 2026-06-11 | test evidence | legality, determinism, terminal smoke | none | Used as provisional evidence until the native `simulate` registration lands. |

## Rules cross-check

| Strategy area | Rule IDs checked | Any uncertainty? | Notes |
|---|---|---:|---|
| Faction-specific legal actions | `FC-ACT-001` through `FC-ACT-009` | no | Competent choices must come from the Rust legal tree. |
| Graph movement and clashes | `FC-CTRL-001` through `FC-CTRL-005` | no | Crew-entry trades remove both entering crew and one guard; guard-entry removes one crew and keeps the guard. |
| Round scoring | `FC-SCORE-GARRISON-FORT`, `FC-SCORE-PROSPECTOR-SUPPLY`, `FC-SCORE-STAKE-VALUE` | no | Garrison wants occupied, crew-free forts; Prospectors want guard-free supply to staked sites. |
| Terminal and tiebreak | `FC-TERM-SCORE-COMPARE`, `FC-TERM-GARRISON-TIEBREAK` | no | Tied final scores favor the Garrison. |
| Bot boundary | `FC-BOT-001` through `FC-BOT-003`, `FC-VIS-001` through `FC-VIS-004` | no | Frontier Control is perfect-information, but bots still use public view plus legal tree only. |

## Competent-player summary

Competent Frontier Control play is about using the two-action turn budget to
change the next scoring round, not merely to move pieces.

- The Garrison wins by keeping forts crew-free while using guard pressure to cut
  or dismantle high-value stakes.
- The Prospectors win by converting crew position into supplied stakes and by
  making Garrison patrols choose between fort points and supply cuts.
- Both factions must notice that clashes are asymmetric: crews can trade
  themselves into guards, while guards survive when entering crews.
- The public graph matters because supply is path-based. A single guard on a
  connector can make a valuable stake score zero for the round.
- End turn is sometimes correct when remaining legal actions would move units
  away from the scoring plan.

## Phases and situations

| Phase/situation | What competent players notice | Important rule IDs | Notes |
|---|---|---|---|
| Prospector opening | Base Camp crews need to spread toward value sites before the Garrison can occupy connectors. | `FC-ACT-002`, `FC-ACT-003`, `FC-SCORE-PROSPECTOR-SUPPLY` | Highest stake value is attractive only if supply can remain guard-free. |
| Garrison response | A guard can defend a fort, cut a path, or dismantle a stake, but often not all three in one turn. | `FC-ACT-006`, `FC-ACT-008`, `FC-SCORE-GARRISON-FORT` | The best response is usually the action that changes the next score most. |
| Clash opportunity | Entering a site with opponents can remove material and alter scoring eligibility immediately. | `FC-CTRL-002`, `FC-CTRL-003` | The faction entering the clash changes the result. |
| Late rounds | Total score and tiebreak posture matter more than board coverage. | `FC-TERM-SCORE-COMPARE`, `FC-TERM-GARRISON-TIEBREAK` | Prospectors must beat the Garrison outright; equal totals are not enough. |

## Immediate tactics

| Tactic | Situation | Why it matters | Rule IDs | Bot feature candidate? |
|---|---|---|---|---:|
| Stake the richest legal site | A crew stands on an unguarded, stakeable high-value site. | Converts position into repeat scoring potential. | `FC-ACT-003`, `FC-SCORE-STAKE-VALUE` | yes |
| Dismantle an occupied stake | A guard stands on a staked site. | Removes future Prospector scoring without moving the guard. | `FC-ACT-008`, `FC-CTRL-005` | yes |
| Patrol onto crew pressure | A guard can enter a crewed site or connector. | Removes a crew and can cut supply while the guard survives. | `FC-ACT-006`, `FC-CTRL-003` | yes |
| Reinforce held forts | A held fort is below cap and still crew-free. | Protects recurring Garrison income and makes crew trades more costly. | `FC-ACT-007`, `FC-SCORE-GARRISON-FORT` | yes |
| Muster when expansion stalls | Base Camp is safe and below cap. | Restores Prospector material for future staking and crew trades. | `FC-ACT-004` | yes |

## Threats to block

| Threat | How a player detects it from visible information | Blocking response candidates | Rule IDs | Hidden-info risk |
|---|---|---|---|---|
| Supplied high-value stake | Public stake with a guard-free path to Base Camp. | Patrol onto a connector or the staked site; dismantle if already occupied by a guard. | `FC-SCORE-PROSPECTOR-SUPPLY`, `FC-ACT-006`, `FC-ACT-008` | none |
| Crew contesting a fort | Crew adjacent to or on a fort that could remove or block guards. | Reinforce, patrol into the crew, or move a guard to restore fort control. | `FC-SCORE-GARRISON-FORT`, `FC-CTRL-002`, `FC-CTRL-003` | none |
| Garrison tiebreak lead | Final-round totals are equal or Garrison is ahead. | Prospectors must prefer actions that create supplied points before terminal scoring. | `FC-TERM-GARRISON-TIEBREAK` | none |
| Overextended Garrison guard | Guard leaves a fort or connector exposed. | Prospectors march into the opening or stake the newly supplied value site. | `FC-ACT-002`, `FC-SCORE-PROSPECTOR-SUPPLY` | none |

## Positional, resource, and tempo principles

| Principle type | Principle | Visible evidence | Rule IDs | Notes |
|---|---|---|---|---|
| positional | Connector sites are often more important than remote value sites. | Public graph edges and guard-free supply status. | `FC-COMP-004`, `FC-SCORE-PROSPECTOR-SUPPLY` | Control of Ford, Quarry, Signal Hill, or Timberline can decide several stakes. |
| resource/accounting | Each faction has only two action points per turn. | Public budget in the view. | `FC-SCORE-ACTION-BUDGET`, `FC-TURN-004` | A flashy move that does not affect scoring may be worse than ending turn. |
| card/hand/deck | not applicable | Frontier Control has no cards, hands, decks, or hidden draw order. | `FC-RNG-001`, `FC-VIS-004` | No belief model is needed. |
| tempo/initiative | Prospectors act before Garrison scores each round. | Public active faction and round sequence. | `FC-TURN-001`, `FC-TURN-006` | Garrison gets the last word before each score. |
| risk/control | Crews can trade into guards; guards entering crews survive. | Public units and legal movement paths. | `FC-CTRL-002`, `FC-CTRL-003` | The same occupied destination has different value for each faction. |

## Common beginner mistakes

| Mistake | Why it is bad | How competent play avoids it | Rule IDs | Bot test implied? |
|---|---|---|---|---:|
| Moving without a scoring purpose | Wastes one of two budget points. | Prefer actions that change fort control, stake status, or supply. | `FC-SCORE-ACTION-BUDGET` | yes |
| Staking without supply awareness | A cut stake scores zero. | Check the Rust-projected supplied/cut status and nearby guards. | `FC-SCORE-PROSPECTOR-SUPPLY` | yes |
| Leaving forts crew-contested | Garrison loses fort points when any crew is present. | Patrol into crews or reinforce held forts before scoring. | `FC-SCORE-GARRISON-FORT` | yes |
| Playing for a draw as Prospectors | Ties are Garrison wins. | Prospectors must create a strict final score lead. | `FC-TERM-GARRISON-TIEBREAK` | yes |

## Risk posture

| Situation | Cautious posture | Aggressive posture | Recommended default | Notes |
|---|---|---|---|---|
| Garrison ahead | Hold forts and dismantle stakes already reached by guards. | Patrol farther from forts to cut richer future supply. | cautious | The Garrison tiebreak rewards preserving a lead. |
| Garrison behind | Reinforce only forts that are still scoring. | Patrol into supplied routes and crew clusters. | balanced | A purely defensive Garrison can lose to repeat stake scoring. |
| Prospectors ahead | Preserve supplied stakes and avoid unnecessary crew trades. | Add new stakes only when supply can survive the Garrison reply. | balanced | Garrison scores after its own turn, so exposed stakes are fragile. |
| Prospectors behind | Muster and march to high-value sites even through trades. | Trade crews into guards to open a route before final scoring. | aggressive | Equal final score still loses. |

## Visible signals

| Signal | Visible to whom | Strategic meaning | Bot feature candidate | Notes |
|---|---|---|---:|---|
| `supplied == true` on a stake | all | Prospector points will score if unchanged until scoring. | yes | Rust computes this; TypeScript must not. |
| Crew count on or adjacent to a fort | all | Fort income is threatened or already blocked. | yes | Current Level 1 Garrison policy reacts through patrol/reinforce choices. |
| Guard on a staked site | all | Dismantle is likely available. | yes | Current Level 1 Garrison policy prioritizes it first. |
| Public score totals and round | all | Determines whether Garrison tiebreak or Prospector urgency matters. | yes | Current Level 1 policies do not yet use score-race urgency. |

## Hidden/private information boundary

| Information | Human seat sees? | Competent inference allowed? | Bot may use? | Forbidden peeking risk | Notes |
|---|---:|---:|---:|---|---|
| Sites, edges, units, stakes, scores, supplied/cut status | yes | yes | yes | none | These are public facts. |
| Bot seed | no gameplay fact | no strategic inference | only for deterministic tie-break | low | Current Level 1 policies are deterministic even where the seed is not materially used. |
| Hidden pieces, cards, deck order, secret objectives | no | no | no | none | Not applicable; the game has none. |
| Internal mutable state outside public view/legal tree | no | no | no | low | Public bots validate through the normal command path. |

## Inference allowed vs forbidden peeking

| Scenario | Allowed inference | Forbidden shortcut | Test implied |
|---|---|---|---|
| Choosing a supply cut | Use public graph, public guard/crew locations, and Rust-projected supplied status. | Let TypeScript or a bot-only hidden helper decide scoring connectivity from private state. | visibility and bot legality tests |
| Choosing a stake | Use legal `stake/<site>` leaves and public stake values. | Construct a stake action not present in the legal tree. | bot legality tests |
| Explaining a bot decision | Cite public faction, site, stake, unit, score, or legal-action facts. | Mention inaccessible internal scoring probes or future outcomes. | bot explanation smoke |

## Strategy examples

| Example ID | Situation | Candidate choices | Competent choice | Explanation | Rule IDs |
|---|---|---|---|---|---|
| `FC-S-EX-001` | Prospectors have a crew on Goldfield and no guard occupies it. | `stake/site_goldfield`, march elsewhere, muster, end turn | `stake/site_goldfield` | Goldfield is the highest-value stake site, so it creates the largest recurring upside if supply remains open. | `FC-ACT-003`, `FC-SCORE-STAKE-VALUE` |
| `FC-S-EX-002` | A Garrison guard stands on a staked site. | `dismantle/<site>`, patrol away, reinforce, end turn | `dismantle/<site>` | Removing a public stake denies all future scoring from that marker. | `FC-ACT-008`, `FC-CTRL-005` |
| `FC-S-EX-003` | Final scores are tied entering the last scoring window. | Prospectors preserve status quo, or create one more supplied point | Create one more supplied point | The Garrison wins tied final totals. | `FC-TERM-GARRISON-TIEBREAK` |

## Anti-examples

| Anti-example ID | Bad choice | Why it is bad | Rule IDs | Bot guard/test implied |
|---|---|---|---|---|
| `FC-S-BAD-001` | Garrison patrols away from the only guarded fort while ahead. | It may lose recurring fort income and already owns the tiebreak. | `FC-SCORE-GARRISON-FORT`, `FC-TERM-GARRISON-TIEBREAK` | future balance/policy tests |
| `FC-S-BAD-002` | Prospectors repeatedly end turn with crews at Base Camp and no stakes. | It never creates scoring pressure. | `FC-ACT-002`, `FC-ACT-003`, `FC-SCORE-PROSPECTOR-SUPPLY` | bot terminal smoke |
| `FC-S-BAD-003` | A public explanation says a future path will be safe after the Garrison turn. | Future opponent choices are not known. | `FC-VIS-003`, `FC-RNG-001` | explanation review |

## Known hard problems

| Problem | Why hard | Out-of-scope for current bot? | Notes |
|---|---|---:|---|
| Multi-turn supply race | Best play can require valuing a route two or more turns ahead. | yes | Current Level 1 bots use immediate public priorities, not search. |
| Score-race urgency | The best action changes when one faction can coast or must catch up. | yes | Current policies expose this as a known weakness. |
| Balance tuning | Small changes in map constants or policy priority can swing deterministic bot-vs-bot results. | no | The A5 band is a public-polish gate; misses require retune evidence. |

## Out-of-scope advanced strategy

| Strategy idea | Why out of scope | Future trigger |
|---|---|---|
| Level 2 authored policy with phase nodes and bounded scoring | Gate 13 only claims Level 1 baseline bots. | A future polish ticket that completes the full evidence workflow. |
| Shallow deterministic search | Not required for this graph/asymmetry proof and would need limits/benchmarks. | Separate ADR or Level 3 review if product value justifies it. |
| MCTS, ISMCTS, Monte Carlo, ML, RL, or runtime LLM move selection | Forbidden for public v1/v2 bots. | Future ADR only. |

## Level 1 balance evidence

Assumption A5 sets a target that each faction wins roughly 35-65% of Level 1 vs
Level 1 games on the standard map. The registered native `simulate` arm now
records current standard-map evidence alongside the bot-vs-bot harness.

| Evidence source | Scope | Result | A5 status | Required follow-up |
|---|---|---|---|---|
| `cargo test -p frontier_control bots` | bot legality and explanation smoke | both faction bots validate through the normal command path | does not measure band | keep as policy fidelity evidence |
| `cargo test -p frontier_control level1_bot_sequence_reaches_terminal_without_illegal_actions` | deterministic Level-1-vs-Level-1 terminal smoke | completes a full standard-map game without illegal actions | does not measure band | keep as provisional terminal evidence |
| 2026-06-11 registered simulation | `cargo run -p simulate -- --game frontier_control --games 1000` | Garrison wins 1000-0; average score 16-0; average rounds 8; average length 32 | outside A5 band | retune constants or policy before claiming balanced public play |
| temporary 2026-06-11 Level-1-vs-Level-1 probe | one deterministic highlands playout through current public Level 1 policies | Prospectors win 15-3 | informational, not A5 standard-map band | use as cross-map tuning signal |

The current docs therefore do not claim balance is achieved. They record a
retune trigger: the standard-map Level-1-vs-Level-1 result is outside 35-65%,
so constants or Level 1 priorities must be retuned before public polish.

## Translation to candidate future Level 2 features

These are future candidates only. They are not implemented policy promises.

| Candidate feature | Derived from | Visible to bot? | Used for | Hidden-info risk | Test implied |
|---|---|---:|---|---|---|
| Score-race urgency | late-round strategy and A5 balance evidence | yes | phase priority | none | simulation and decision examples |
| Connector value estimation | supply threats and positional principles | yes | bounded tie-break | none | supply-cut examples |
| Fort danger estimate | threat and immediate tactic sections | yes | Garrison priority ordering | none | Garrison decision examples |
| Crew material threshold | muster and clash sections | yes | Prospector bounded tie-break | none | Prospector decision examples |

## Tests implied by strategy claims

| Strategy claim | Rule IDs | Test type | Test name placeholder | Notes |
|---|---|---|---|---|
| Bots choose only legal action paths. | `FC-ACT-001` through `FC-ACT-009` | bot decision | `bots_select_legal_paths_for_both_factions` | Existing unit coverage. |
| Level 1 decisions are deterministic under declared inputs. | `FC-RNG-001`, `FC-BOT-002`, `FC-BOT-003` | bot decision | `bots_are_deterministic_under_declared_inputs` | Existing unit coverage. |
| Bot-vs-bot play can reach terminal without illegal actions. | `FC-TURN-008`, `FC-TERM-NO-ACTIONS` | property/invariant | `level1_bot_sequence_reaches_terminal_without_illegal_actions` | Existing integration coverage. |
| Standard-map Level 1 balance falls in A5 band or has a retune note. | Assumption A5 | simulation | `cargo run -p simulate -- --game frontier_control --games 1000` | Current registered simulation records a retune note. |

## Review checklist

- All strategy prose is original.
- Sources are recorded and not copied.
- Strategy claims are checked against `RULES.md`.
- Hidden-information boundaries are explicit.
- Allowed inference and forbidden peeking are separated.
- Examples and anti-examples are concrete enough to test.
- Candidate Level 2 features are evidence, not current policy claims.
- Level 1 balance is recorded as a measured target with an explicit retune trigger.

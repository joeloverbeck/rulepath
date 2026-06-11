# Masked Claims Competent Player Analysis

Game ID: `masked_claims`

Implemented variant: `masked_claims_standard`

Rules version checked: `masked-claims-rules-v1`

Date: 2026-06-11

## Purpose and Authority

This is strategy analysis for the implemented Masked Claims variant. It
documents competent human play and the Level 1 bot posture. It does not define
rules. If this document conflicts with [RULES.md](RULES.md), the rules win.

## Sources and References

| Source/reference | Date consulted | Source quality | Used for | Copied prose status | Notes |
|---|---|---|---|---|---|
| [RULES.md](RULES.md) | 2026-06-11 | rules authority | claim, reaction, reveal, scoring, visibility, bot boundaries | none | Local Rulepath prose. |
| [SOURCES.md](SOURCES.md) | 2026-06-11 | source and IP record | original-design boundaries and non-authoritative bluffing context | none | Strategy examples are Rulepath-authored. |
| [bots.rs](../src/bots.rs) | 2026-06-11 | implementation evidence | Level 0 and Level 1 policy behavior | none | Rust owns bot choices. |
| [bots.rs tests](../tests/bots.rs) | 2026-06-11 | executable evidence | legality, determinism, repeated completions, rationale no-leak | none | Native bot suite. |
| [BENCHMARKS.md](BENCHMARKS.md) | 2026-06-11 | benchmark posture | calibration status | none | Balance calibration is a named follow-up. |

## Competent-Player Summary

Competent Masked Claims play is short bluff discipline under strict visibility:

- claim only through the legal Rust action tree;
- use own hand grades to decide when honest claims, underclaims, and bounded
  bluffs are plausible;
- track public exposed masks and own hand masks when judging whether a claim is
  impossible;
- accept plausible low-value claims when challenging is not clearly profitable;
- challenge claims that public counting proves impossible;
- remember that accepted masks never reveal, so accepted-gallery identities are
  not information a player or bot may use later.

## Phases and Situations

| Phase/situation | What competent players notice | Important rules | Notes |
|---|---|---|---|
| Claim phase | Own hand, legal claim paths, current score, public exposed rows, and turn count. | `MC-ACT-001`, `MC-ACT-002`, `MC-SCORE-001` | Honest high claims score well if accepted; bounded bluffs punish always-accept opponents. |
| Reaction window | Public declared grade, own hand, exposed rows, scores, and legal accept/challenge choices. | `MC-ACT-003`, `MC-SCORE-002`, `MC-VIS-003` | The responder never sees the pedestal identity unless challenge resolves. |
| After accept | Declared grade and score delta are public; tile identity is gone from browser-facing surfaces. | `MC-SCORE-001`, `MC-VIS-005` | Do not treat veiled gallery as a memory of actual grade. |
| After challenge | The challenged tile is revealed exactly once and becomes public exposed-row information. | `MC-SCORE-002`, `MC-VIS-004` | Public counting may use exposed actual grades after reveal. |
| Terminal | Score and tie-break rationale are public; accepted masks remain redacted. | `MC-END-001` through `MC-END-005` | Terminal is not permission to inspect veiled-gallery identities. |

## Immediate Tactics

| Tactic | Situation | Why it matters | Bot feature candidate? |
|---|---|---|---:|
| Highest honest claim | Claim phase with a high held grade | Scores well if accepted and is safe if challenged. | yes |
| Bounded one-step bluff | Mid-grade held mask and declared grade not impossible by counting | Keeps the bluff game alive without wild overclaiming. | yes |
| Underclaim trap | High held mask can be declared lower | A challenge still reveals an honest claim and awards actual grade plus bonus. | yes |
| Certain-lie challenge | All copies of the declared grade are visible through own hand plus exposed rows | Challenge is justified from legal information only. | yes |
| Accept plausible low claim | Declared grade remains possible and low value | Avoids giving honest claims challenge bonuses. | yes |

## Hidden/Private Information Boundary

| Information | Human seat sees? | Competent inference allowed? | Bot may use? | Forbidden peeking risk | Notes |
|---|---:|---:|---:|---|---|
| own hand | yes | yes | yes | low | Seat-private view only. |
| legal action tree | yes | yes | yes | none | Rust supplies legal actions. |
| public declared grade on pedestal | yes | yes | yes | none | Safe public fact. |
| exposed rows and actual revealed grades | yes | yes | yes | none | Revealed only after challenge. |
| veiled-gallery declared grades/counts | yes | yes | yes | medium | Actual accepted tile identities remain forbidden. |
| opponent hand | no | no | no | high | Must not appear in bot input or rationale. |
| reserve identities | no | no | no | high | Internal setup material only. |
| pedestal tile identity before reveal | no | no | no | high | Challenge decision must not depend on it. |
| accepted mask identity | no | no | no | high | Hidden forever, including terminal. |

## Strategy Examples

| Example ID | Situation | Candidate choices | Competent choice | Explanation |
|---|---|---|---|---|
| `MC-EX-001` | Claimant holds a Master and can claim any grade | declare Master with that held mask | Master claim | Honest high claims are safe against challenge and valuable if accepted. |
| `MC-EX-002` | Claimant holds Gilded and public counting does not exhaust Jeweled | declare Jeweled as bounded bluff | Jeweled claim | A one-step bluff is explainable and punishable if overused. |
| `MC-EX-003` | Responder sees all three Master masks in own hand/exposed rows and a Master claim is pending | accept or challenge | challenge | The claim is impossible from visible information. |
| `MC-EX-004` | Responder sees a plausible Trimmed claim | accept or challenge | accept | A low plausible claim is often not worth gifting a challenge bonus. |

## Common Beginner Mistakes

| Mistake | Why it is bad | How competent play avoids it | Bot test implied? |
|---|---|---|---:|
| Always bluffing maximum grade | Public counting and challenges expose large gaps. | Use bounded bluffs and honest defaults. | yes |
| Always challenging | Honest and underclaimed masks score actual grade plus bonus. | Challenge only impossible or low-plausibility claims. | yes |
| Remembering accepted mask identities | Violates the no-leak model. | Treat veiled masks as declared-grade slots only. | yes |
| Acting from opponent hand guesses as facts | Hidden-info leak. | Use only own hand, public exposed rows, and declared grades. | yes |

## Balance Evidence and Calibration

Current executable evidence proves Level 0 and Level 1 legality, determinism,
hidden-state independence, and many completed Level 1 games in
[bots.rs](../tests/bots.rs). Statistical mirrored Level 1 vs Level 1 balance is
not treated as calibrated yet. A material asymmetry outside the rough 40-60
seat-win band should trigger the Assumption A4 scoring-constant retune described
by the Gate 11 spec and the calibration follow-up named in
[BENCHMARKS.md](BENCHMARKS.md).

## Review Checklist

- Strategy prose is original.
- Rules authority is separate from strategy.
- Hidden-information boundaries are explicit.
- No strategy claim requires MCTS, Monte Carlo, ML, RL, hidden-state sampling,
  opponent-hand access, reserve access, or pedestal peeking.

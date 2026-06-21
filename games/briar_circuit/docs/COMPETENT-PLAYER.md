# Briar Circuit Competent Player Analysis

Game ID: `briar_circuit`

Implemented variant: `briar_circuit_standard`

Rules version checked: `briar-circuit-rules-v1`

Date: 2026-06-21

## Purpose And Authority

This document is strategy analysis for future bot work. It is not rule
authority. [RULES.md](RULES.md) wins over this document whenever they differ.

All prose is original Rulepath prose. Sources are recorded in [SOURCES.md](SOURCES.md).

## Sources And Observations

| Source/reference | Date consulted | Used for | Copied prose status | Notes |
|---|---:|---|---|---|
| [RULES.md](RULES.md) | 2026-06-21 | implemented rule IDs and legal boundaries | none | Rule authority. |
| [SOURCES.md](SOURCES.md) | 2026-06-21 | rules-family facts and variant choices | none | Consulted-not-copied source notes. |
| self-play/code review | 2026-06-21 | strategy implications from implemented pass/play/scoring | none | No external strategy prose copied. |

## Rules Cross-Check

| Strategy area | Rule IDs checked | Any uncertainty? | Notes |
|---|---|---:|---|
| pass pressure | `BC-PASS-001` through `BC-PASS-004` | no | Pass direction and private selection control future hand shape. |
| trick legality | `BC-PLAY-001` through `BC-PLAY-007`, `BC-TRICK-*` | no | Follow suit and first-trick restrictions dominate immediate tactics. |
| scoring/moon | `BC-SCORE-*`, `BC-MATCH-*` | no | Low score wins; moon transform is fixed. |
| hidden information | `BC-VIS-*`, `BC-BOT-*` | no | Competent inference must use legal public/own information only. |

## Competent-Player Summary

A competent Briar Circuit player tries to avoid taking penalty cards while
preserving legal exits for later tricks. They track public played cards, current
score pressure, whether hearts are broken, who is leading, and their own hand
shape. They may infer risk from public history, but they may not know opponent
hands, pass provenance, deck order, or future random facts.

The shipped Level 1 bot is not a competent-player proxy. It is a safe,
deterministic baseline.

## Seat And Opponent Model

| Field | Analysis | Rule IDs | Notes |
|---|---|---|---|
| supported seat range | exactly four | `BC-SETUP-001` | Everyone is an independent opponent. |
| number of opponents | three table competitors | `BC-MATCH-003` | Low cumulative score wins. |
| partnership/team roles | none | `BC-OOS-001` | No teammate sharing. |
| turn-order pressure | trick winner leads next; pass direction changes by hand | `BC-PASS-001`, `BC-TRICK-002` | Leadership can be good or bad depending on hearts state and hand shape. |

## Phases And Situations

| Phase/situation | What competent players notice | Rule IDs | Notes |
|---|---|---|---|
| pass left/right/across | Which cards are dangerous to keep and whether short-suiting a suit is useful. | `BC-PASS-001`, `BC-PASS-002` | Cannot know incoming cards until exchange. |
| hold hand | No pass relief; manage original hand. | `BC-PASS-004` | Higher risk when holding many points. |
| opening trick | 2 clubs forced, no points unless no alternative. | `BC-PLAY-001`, `BC-PLAY-004` | First trick is low point risk but establishes public void clues. |
| follow-suit trick | Whether to win cheaply, duck, or discard a safe/dangerous card when void. | `BC-PLAY-002`, `BC-PLAY-003` | Must follow suit when able. |
| hearts unbroken lead | Avoid illegal heart leads unless only hearts remain. | `BC-PLAY-005`, `BC-PLAY-006` | Hearts broken changes leading choices. |
| score threshold | Table standings matter more near 100. | `BC-MATCH-002`, `BC-MATCH-003` | Lowest unique score wins after threshold. |

## Immediate Tactics

| Tactic | Situation | Why it matters | Rule IDs | Bot feature candidate? |
|---|---|---|---|---:|
| dump a point card when void after trick one | Cannot follow suit and point card is legal. | Reduces own future penalty burden, but may feed another player's moon. | `BC-PLAY-003`, `BC-SCORE-001` | yes |
| avoid winning a point-heavy trick | Holding led suit with multiple legal cards. | Taking hearts or queen of spades adds penalties. | `BC-TRICK-001`, `BC-SCORE-001` | yes |
| shed queen of spades safely | Void in led suit after first trick or unable to be captured by self. | Queen is high penalty. | `BC-SCORE-001` | yes |
| preserve low exits | Multiple low cards in a suit. | Cheap followers help avoid taking later tricks. | `BC-PLAY-002`, `BC-TRICK-001` | yes |
| disrupt visible moon attempt | One seat has captured many/all public points. | Fixed moon transform punishes every opponent. | `BC-SCORE-003` | future |

## Visible Signals

| Signal | Visible to whom | Strategic meaning | Bot feature candidate | Notes |
|---|---|---|---:|---|
| current trick cards | all viewers | determines led suit and current high card | yes | Public after play. |
| captured tricks | all viewers | identifies who has captured public point cards | yes | Can support moon-risk logic. |
| hand counts | all viewers | tracks remaining cards only | yes | Counts do not reveal identities. |
| own hand shape | owning seat only | suit length, point burden, exits | yes | Bot may use only its own hand. |
| cumulative scores | all viewers | threshold and table-leader pressure | yes | Public scoring. |
| pass direction/counts | all viewers | public pass phase progress | yes | Selected identities stay private. |

## Hidden/Private Information Boundary

| Information | Human seat sees? | Competent inference allowed? | Bot may use? | Forbidden peeking risk | Notes |
|---|---:|---:|---:|---|---|
| own private hand | yes | yes | yes | none | Authorized seat-private information. |
| opponent private hand | no | no actual identities | no | high | May infer only from public play history. |
| pass selections/provenance | own selection only | no provenance for others | no unauthorized provenance | high | Passed-card origin remains private. |
| deck order/future deals | no | no | no | high | Seed reconstruction is forbidden. |
| public played cards/history | yes | yes | yes | none | Legal public memory. |
| bot candidate rankings | no public default | not strategy input | no public hidden facts | medium | Dev-only and redacted if ever added. |

## Private Inference Forbidden

| Tempting shortcut | Why forbidden | Required bot guard/test | Notes |
|---|---|---|---|
| reading opponent hand to decide whether queen is safe | unavailable to acting seat | opponent-hand mutation invariance test | Covered for L1. |
| reconstructing deck from seed | future hidden setup fact | replay/export no-leak tests | Seed material must not be bot input. |
| remembering pass provenance after exchange for all seats | provenance is private | export/e2e canary scans | Card identity may become public later, origin does not. |
| sampling possible deals from actual hidden state | hidden-state shortcut and Monte Carlo-like path | public v1/v2 exclusion | Level 2 must not use this. |

## Kingmaking And Coalition Risk

| Risk | Visible trigger | Competent response principle | Bot feature candidate? | Rule IDs | Notes |
|---|---|---|---:|---|---|
| table-leader protection or kingmaking | Multiple opponents near threshold with one low-score leader. | Use only public scores/history; avoid giving a clear leader easy low-risk tricks when a safer legal play exists. | future | `BC-MATCH-003` | No private coordination or intent model. |
| moon prevention | One seat publicly captured most points in a hand. | Consider taking a small point to break the moon if legally possible and strategically justified. | future | `BC-SCORE-003` | Requires careful hidden-info-safe tests. |

## Strategy Examples

| Example ID | Situation | Candidate choices | Competent choice | Explanation | Rule IDs |
|---|---|---|---|---|---|
| `S-EX-001` | Following led clubs with low and high clubs, no points in trick. | low club / high club | low club | Avoid taking control unless winning helps later. | `BC-PLAY-002`, `BC-TRICK-001` |
| `S-EX-002` | Void after first trick with queen of spades and low non-point card. | queen / low non-point | depends on current winner and moon risk; often shed queen if another seat will take it | Queen is 13 points, but feeding a moon is dangerous. | `BC-SCORE-001`, `BC-SCORE-003` |
| `S-EX-003` | Holding only hearts before hearts are broken while leading. | any heart | legal lowest heart | Rule permits heart lead when only hearts remain. | `BC-PLAY-006` |

## Anti-Examples

| Anti-example ID | Bad choice | Why it is bad | Rule IDs | Bot guard/test implied |
|---|---|---|---|---|
| `S-BAD-001` | Claiming an off-suit play is legal while holding led suit. | Illegal; Rust must reject. | `BC-PLAY-002` | legal action API only |
| `S-BAD-002` | Planning from an opponent's hidden queen location. | Hidden information. | `BC-VIS-001`, `BC-BOT-002` | no-leak/invariance tests |
| `S-BAD-003` | Treating Level 1 high-point passing as competent moon defense. | Too shallow and not evidence-backed. | `BC-BOT-002` | Level 2 evidence gate |

## Known Hard Problems

| Problem | Why hard | Out-of-scope for current bot? | Notes |
|---|---|---:|---|
| moon prevention/planning | Needs public point tracking, risk posture, and no hidden hand peeking. | yes | Future Level 2 only. |
| endgame leader targeting | Multi-opponent public-score pressure can create kingmaking risk. | yes | Requires tests and simulations. |
| suit-memory inference | Legal inference from public history is subtle but must not become actual hidden-state access. | yes | Future evidence pack item. |

## Candidate Level 2 Features

| Candidate feature | Derived from | Visible to bot? | Used for | Hidden-info risk | Test implied |
|---|---|---:|---|---|---|
| own-hand danger score | own points, suit lengths | yes | pass/play priorities | low | own-view tests |
| public moon-risk tracker | captured public point cards by seat | yes | prevention priority | medium | no hidden-state tests |
| public standing pressure | cumulative scores | yes | endgame posture | low | terminal examples |
| suit-memory from public history | played cards only | yes | legal inference | medium | forbidden-peeking tests |

## Review Checklist

- This document does not authorize Level 2.
- Strategy claims are checked against [RULES.md](RULES.md).
- Hidden-information boundaries are explicit.
- Level 1 is not represented as competent-human play.

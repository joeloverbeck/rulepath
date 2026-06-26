# Meldfall Ledger Mechanics Inventory

Game ID: `meldfall_ledger`

Roadmap stage/gate: Public scaling phase / Gate 19 Five Hundred Rummy proof

Rules version: `meldfall-ledger-rules-v1`

Prepared by: `Codex`

Last updated: 2026-06-26

Evidence receipt: [GAME-EVIDENCE.md](GAME-EVIDENCE.md)

Post-build status: behavior, WASM, tools, web renderer, e2e smoke,
primitive-pressure ledger, repo atlas/register, and machine `forward-v1`
receipt are complete as of 2026-06-26.

## Purpose

This inventory records Meldfall Ledger's game-local mechanic shapes,
primitive-pressure posture, and mechanical-scaffolding audit closeout. It is
evidence for [docs/MECHANIC-ATLAS.md](../../../docs/MECHANIC-ATLAS.md),
[docs/MECHANICAL-SCAFFOLDING-REGISTER.md](../../../docs/MECHANICAL-SCAFFOLDING-REGISTER.md),
and [GAME-EVIDENCE.md](GAME-EVIDENCE.md), not permission to generalize.

Meldfall Ledger is a variable 2-6 seat hidden-hand public-meld game. Rust owns
setup, shuffle/deal, draw-source legality, discard-pile pickup commitment,
meld validation, lay-off legality, scoring, terminal outcomes, visibility,
effects, replay, bots, and benchmark evidence. TypeScript presents Rust/WASM
output only.

## Mechanic Inventory

| Category | Game-local description | Evidence in rules | Current status | Notes |
|---|---|---|---|---|
| N-seat model | Variable seats 2 through 6, default 4, public observer, and one seat-private viewer per seat. | `ML-SETUP-001`, `ML-VIS-*` | `local-only` | Reuses canonical seat grammar/scaffolding; setup admission and deal sizes stay game-local. |
| turn-order policy | Dealer/start-seat state, active seat, draw/table/discard phases, round settlement, and next-round transition. | `ML-SETUP-005`, `ML-TURN-*`, `ML-MATCH-*` | `local-only` | No generic phase machine. |
| topology/spatial model | No board topology; public layout is private hands, stock/discard zones, public meld tableau, and score ledger. | `ML-UI-*` | `local-only` | No graph, grid, route, or movement primitive. |
| component/zone model | Standard 52-card deck, hidden stock, private hands, public discard pile, public meld groups, scores, and standings. | `ML-SETUP-*`, `ML-VIS-*`, `ML-MELD-*` | `local-only` | Card/deck/hand behavior stays game-local. |
| hidden-hand/deck model | Own hand is seat-private; opponent hands and stock order are hidden; public observer sees counts and public zones only. | `ML-VIS-*`, `ML-REPLAY-*` | `rejected/deferred with rationale` | Existing private-hand pressure remains deferred; Meldfall adds no helper boundary. |
| action shape | Phase-specific legal leaves for stock/discard draw, meld, lay-off, finish/go-out, and discard. | `ML-TURN-*`, `ML-MELD-*`, `ML-LAYOFF-*` | `local-only` | Action-tree framing may be reused; leaves and legality stay local. |
| randomness/chance | Seeded one-deck shuffle and deal; stock draws are deterministic internally and viewer-redacted externally. | `ML-SETUP-003`, `ML-REPLAY-001` | `local-only` | No RNG/hash migration authorized. |
| visibility/hidden information | Public discard/tableau/scores/counts; owner-private hands; stock order hidden from every viewer. | `ML-VIS-*` | `local-only` | Pairwise proof covers public observer and all six seat viewers. |
| meld validation | Same-rank sets and same-suit runs with ace low/high/no-wrap policy. | `ML-MELD-001` through `ML-MELD-004` | `local-only` first use | Recorded as `ML-PP-001`; no rummy helper. |
| public meld tableau | Stable public meld ids, origin seats, ordered cards, and played-by score-credit owners. | `ML-MELD-005`, `ML-SCORE-006` | `local-only` first use | Recorded as `ML-PP-002`; public transport is not scaffolding. |
| discard-pile pickup | Selecting a discard takes that card plus newer cards and requires immediate use of the selected card. | `ML-TURN-003`, `ML-TURN-004` | `local-only` first use | Recorded as `ML-PP-003`; commitment policy is behavior. |
| lay-off | A seat may extend any public meld when the resulting meld is legal; the laying-off seat receives score credit. | `ML-LAYOFF-*`, `ML-SCORE-006` | `local-only` first use | Recorded as `ML-PP-004`. |
| scoring/outcome | Tabled positives, in-hand penalties, negative scores allowed, target 500, unique highest wins, target ties continue. | `ML-SCORE-*`, `ML-MATCH-*` | `local-only` first use | Recorded as `ML-PP-005`. |
| semantic effect shape | Setup, draw, discard pickup, meld, lay-off, discard, round-score, terminal, and setup/turn diagnostics. | `ML-UI-*`, `ML-REPLAY-*` | `local-only` | Effect-envelope constructors are scaffolding only; effect meaning stays local. |
| UI interaction pattern | Large private hands, public meld tableau, discard tail, score ledger, replay import/export, no-drag action controls. | `ML-UI-*` | `local-only` | Browser renders Rust legal choices and Rust projections. |
| bot policy pattern | L0 random legal only; L1/L2 not admitted; no public search/ML/RL/Monte Carlo. | `ML-BOT-*` | `local-only` | Bot input is authorized Rust projection only. |
| benchmark/performance pressure | 2/4/6 setup and playout, large action surfaces, discard tail, tableau projection, replay export/import, L0 bot. | `ML-REPLAY-*`, `ML-BOT-*` | `local-only` | Benchmarks are evidence, not extraction authority. |

## Mechanical Scaffolding Reuse-First Audit

Pre-implementation admission is recorded in
[GAME-IMPLEMENTATION-ADMISSION.md](GAME-IMPLEMENTATION-ADMISSION.md). The
post-build machine receipt is recorded in
`ci/scaffolding-audits.json` with `coverage: "forward-v1"` and
`disposition: "no-new-scaffolding"`.

| Surface | Existing MSC entry/shared symbol reviewed | Final decision | Hash/visibility/determinism expectation |
|---|---|---|---|
| semantic effect envelopes | `MSC-8C-001` | reused only as generic envelope plumbing | effect meaning/order remains Meldfall-owned |
| seat identity strings and WASM import boundary | `MSC-8C-002` | reused for canonical `seat_<n>` grammar | no hidden-info impact |
| seat-count/ring helpers | `MSC-8C-003` | reused structurally where applicable | setup policy and deal bytes remain game-local |
| action-tree encoding/hash v1 | `MSC-8C-004` | reused as framing/evidence only | no legality or replay-authority migration |
| stable-byte writer v1 | `MSC-8C-005` | not present as a new production authority | no broad state/effect/view hash migration |
| dev-only test-support crate | `MSC-8C-006` | dev/test use only where applicable | no production edge |
| pairwise no-leak matrix | `MSC-8C-007` | reused as proof geometry | projection policy remains game-owned |
| evidence-profile drivers | `MSC-8C-008` | reused where validators apply | metadata checks only |
| bounded-index sampling | `MSC-8C-009` | no migration authority | shuffle/deal bytes remain versioned by game evidence |
| behavior-policy bundle | `MSC-8C-010` | rejected/local-only | meld, pickup, lay-off, scoring, bots, visibility, and UI stay local |

No new behavior-free scaffolding shape was invented. No prior official game
requires a refactor unit. The accepted no-unit disposition is `MSC-8C-010`.

## Primitive Pressure Closeout

| Pressure | Disposition | Evidence |
|---|---|---|
| meld validation | first-use `local-only` | [PRIMITIVE-PRESSURE-LEDGER.md](PRIMITIVE-PRESSURE-LEDGER.md#ml-pp-001---meld-validation-sets-and-runs) |
| public meld tableau | first-use `local-only` | [PRIMITIVE-PRESSURE-LEDGER.md](PRIMITIVE-PRESSURE-LEDGER.md#ml-pp-002---public-meld-tableau-and-zone-model) |
| draw/discard pickup | first-use `local-only` | [PRIMITIVE-PRESSURE-LEDGER.md](PRIMITIVE-PRESSURE-LEDGER.md#ml-pp-003---drawdiscard-piles-with-multi-card-pickup) |
| lay-off onto any meld | first-use `local-only` | [PRIMITIVE-PRESSURE-LEDGER.md](PRIMITIVE-PRESSURE-LEDGER.md#ml-pp-004---lay-off-onto-any-public-meld) |
| cumulative scoring | first-use `local-only` | [PRIMITIVE-PRESSURE-LEDGER.md](PRIMITIVE-PRESSURE-LEDGER.md#ml-pp-005---multi-round-cumulative-scoring-to-500) |
| deterministic shuffle/private hands | reviewed against §10B; no new hard gate | [PRIMITIVE-PRESSURE-LEDGER.md](PRIMITIVE-PRESSURE-LEDGER.md#ml-pp-006---deterministic-shuffleprivate-hand-redacted-export-review) |

## Review Checklist

- `engine-core` remains noun-free.
- No `game-stdlib` rummy helper is created.
- Static data remains typed content, parameters, metadata, fixtures, traces, and reports only.
- TypeScript presents Rust/WASM payloads only.
- Hidden hands and stock order stay redacted outside authorized seat-private views.
- L0 bots choose only from Rust legal actions and do not access hidden stock or opponent hands.
- The forward-v1 C-01 through C-10 audit and post-build receipt are complete.

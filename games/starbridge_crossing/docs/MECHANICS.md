# Starbridge Crossing Mechanics Inventory

Game ID: `starbridge_crossing`

Roadmap stage/gate: Public scaling phase / Gate 20 Star Halma topology proof

Rules version: `starbridge-crossing-rules-v1`

Prepared by: `Codex`

Last updated: 2026-06-27

Evidence receipt: `GAME-EVIDENCE.md` in a later ticket

Pre-code status: source/IP notes, rules contract, topology primitive-pressure
ledger, and this forward-v1 scaffolding audit are complete for implementation
admission. Production code, fixtures, WASM, web, benchmarks, and post-build
machine receipts land in later tickets.

## Purpose

This inventory records Starbridge Crossing's game-local mechanic shapes,
primitive-pressure posture, and mechanical-scaffolding reuse-first audit. It is
evidence for [docs/MECHANIC-ATLAS.md](../../../docs/MECHANIC-ATLAS.md),
[docs/MECHANICAL-SCAFFOLDING-REGISTER.md](../../../docs/MECHANICAL-SCAFFOLDING-REGISTER.md),
and future [GAME-EVIDENCE.md](GAME-EVIDENCE.md), not permission to generalize.

Starbridge Crossing is a 2/3/4/6-seat perfect-information race on a public
121-space six-pointed star. Rust owns setup, topology interpretation, legal
steps, hop-chain enumeration, blocked passes, finish ranks, turn-limit
standings, effects, replay, bots, and benchmark evidence. TypeScript presents
Rust/WASM output only.

## Mechanic Inventory

| Category | Game-local description | Evidence in rules | Current status | Notes |
|---|---|---|---|---|
| topology/spatial model | 121 stable spaces in a six-pointed-star topology, with game-local coordinates, neighbor metadata, home/target zones, and UI anchors. | `SC-SETUP-004`, `SC-MOVE-*` | `rejected/deferred with rationale` | [PRIMITIVE-PRESSURE-LEDGER.md](PRIMITIVE-PRESSURE-LEDGER.md) keeps topology/path legality local. |
| N-seat model | Supported seat counts are exactly 2, 3, 4, and 6; default 2; active homes are deterministic subsets of six point labels. | `SC-SETUP-001` through `SC-SETUP-003` | `local-only` with scaffolding reuse | Seat-count/ring helpers may support structural validation; supported-count set and home/target assignment stay local. |
| component/zone model | 10 public pegs per active seat; public spaces; home, target, and neutral zone labels. | `SC-SETUP-005`, `SC-VIS-*` | `local-only` | Peg ownership and zones are game behavior/content. |
| action shape | Compound legal tree: choose peg, choose step or hop path, continue hops, stop after any hop, or forced `pass_blocked`. | `SC-TURN-*`, `SC-MOVE-*` | `local-only` | Action-tree framing may be reused; leaves and legality stay local. |
| turn/phase model | One active seat at a time in clockwise order, skipping finished seats. | `SC-TURN-*`, `SC-FINISH-*` | `local-only` | No generic phase machine. |
| randomness/chance | No game-rule randomness after deterministic setup; L0 bot tie-breaking uses bot infrastructure. | `SC-SETUP-006`, `SC-REPLAY-*`, `SC-BOT-*` | `local-only` | No chance-node or shuffle helper. |
| visibility/hidden information | Every board fact is public; seat views may only add viewer-local presentation labels. | `SC-VIS-*` | all-public audit required | Pairwise hidden-info taxonomy is not applicable, but all-public parity still gets evidence. |
| movement/capture/placement | Adjacent steps, occupied-midpoint hops, empty landings, no capture, no repeated hop landing, no mixed step/hop. | `SC-MOVE-*` | `local-only` | Movement/path/jump behavior is Non-Promotion List behavior. |
| scoring/outcome | Finish ranks, all-but-one terminal, final rank assignment, turn-limit progress-vector standings. | `SC-FINISH-*` | `local-only` | Ranking/tiebreak/progress policy stays game-local. |
| semantic effect shape | Setup, step, hop-chain substeps, blocked pass, finish rank, terminal, and turn-limit effects. | `SC-UI-*`, `SC-REPLAY-*` | `local-only` | Effect-envelope constructors are scaffolding only; effect meaning stays local. |
| UI interaction pattern | 121-space renderer, peg selection, legal step targets, progressive hop-chain path builder, stop controls, keyboard operation, replay. | `SC-UI-*` | `local-only` | Browser renders Rust legal choices and Rust projections. |
| bot policy pattern | L0 random legal required; higher bots need evidence; forbidden algorithms remain forbidden. | `SC-BOT-*` | `local-only` | Bot heuristics and explanations remain game-local. |
| benchmark/performance pressure | Large topology setup, dense action generation, jump-chain enumeration, playout, serialization/replay, renderer smoke budget. | `SC-REPLAY-*`, `SC-BOT-*`, `SC-UI-*` | `local-only` | Benchmarks are evidence, not extraction authority. |

## Mechanical Scaffolding Reuse-First Audit

Gate 20 is the third `forward-v1` user after Blackglass Pact and Meldfall
Ledger. This audit reviews the accepted scaffolding baseline before serious
implementation. It does not authorize behavior extraction.

| Surface | Register target | Disposition | Behavior exclusion | Prior-game follow-on? | Compatibility expectation |
|---|---|---|---|---|---|
| C-01 semantic effect envelopes | `MSC-8C-001` | reuse expected for envelope construction only | Move, jump, blocked-pass, finish, terminal, animation, and explanation meaning stay Starbridge-owned. | none expected | Effect order, payload, scope, and hashes remain game-owned and stable once pinned. |
| C-02 canonical seat grammar and import boundary | `MSC-8C-002` | reuse expected for canonical `seat_<n>` contracts where the generic API is used | Seat helpers do not decide point labels, supported seat counts, home/target assignment, turn order, finish skipping, or visibility. | none expected | Canonical seat strings only; no alias-output migration without ADR 0009 authority. |
| C-03 seat-count validation and ring-index arithmetic | `MSC-8C-003` | reuse expected with local policy | Helpers may validate nonzero counts and clockwise indices; the discontinuous `{2,3,4,6}` set, six-point mapping, opposite targets, and finished-seat skipping stay local. | none expected | Setup diagnostics and replay bytes remain game-local evidence. |
| C-04 action-tree encoding/hash v1 | `MSC-8C-004` | reuse expected as framing/evidence only | The helper may transport Rust-owned action nodes; it must not generate step legality, hop continuation, stop leaves, repeated-landing policy, blocked pass, or finish outcomes. | none expected | Action-tree bytes are parallel evidence unless a later ticket names migration authority. |
| C-05 stable-byte writer v1 | `MSC-8C-005` | not present unless an authorized evidence surface needs it | Stable byte writing frames caller-supplied bytes only; it may not decide topology order, state meaning, serialization order beyond explicit fields, visibility, or hash authority. | none expected | No broad state/effect/view/replay authority flip in this gate. |
| C-06 dev-only game test-support crate | `MSC-8C-006` | reuse expected as dev/test support only | Test helpers may assert invariants and profile shapes; production crates, WASM, tools, and browser bundles must not gain normal/build dependencies on dev-only support. | none expected | No runtime hash/visibility impact. |
| C-07 pairwise no-leak assertion geometry | `MSC-8C-007` | reuse expected as all-public proof geometry | Matrix geometry may enumerate public observer and seat viewers; it must not decide which Starbridge facts are public. | none expected | Public/seat-view parity is deterministic; no private class is invented. |
| C-08 evidence-profile drivers | `MSC-8C-008` | reuse expected where profiles apply | Drivers may validate profile metadata and shape; setup, commands, topology domain checks, projection, import/export, and outcome facts remain game/tool code. | none expected | Fixture/export/profile authority named per artifact under ADR 0009. |
| C-09 bounded-index sampling | `MSC-8C-009` | no production migration authority expected | Bounded sampling may support bot/test RNG where accepted; it does not become setup, topology, movement, or jump-chain policy. | none expected | No RNG/hash migration without explicit evidence and authority. |
| C-10 behavioral-policy bundle on the Non-Promotion List | `MSC-8C-010` | apply as rejected/local-only | Graph/topology/adjacency/movement/reachability/connectivity, step/hop legality, finish ranking, progress vectors, bot policy, visibility classification, and UI policy are behavior, not scaffolding. | accepted no-unit disposition unless implementation invents a pure scaffolding match | No shared behavior extraction; revisit behavior via mechanic atlas, not the scaffolding register. |

Admission disposition: `no-new-scaffolding` expected.

Prior-game retrofit disposition: no follow-on unit expected at admission,
because Gate 20 does not introduce a new behavior-free scaffolding shape that
earlier games must characterize or migrate to. Frontier Control and Event
Frontier are behavioral topology comparison points, not scaffolding adopters.
If implementation later invents a pure large-surface metadata transport shape
or another behavior-free exact duplicate, the post-build closeout must register
it and either queue a bounded prior-game unit or record an accepted no-unit
disposition with evidence.

## Lawful Shared Homes Review

| Home | Gate 20 admission result |
|---|---|
| `engine-core` | Allowed only for existing generic contracts: game id, seat id, viewer, action tree, command envelope, visibility scope, effects, replay, hash, and serialization boundary. No board, space, peg, topology, path, jump, home, target, graph, or track nouns enter. |
| `game-stdlib::seat` | Allowed for behavior-free seat-count and ring helpers when the implementation proves they fit; supported-count set, point mapping, turn skipping, and home/target policy stay game-local. |
| `game-stdlib::board_space` | Audited not applicable by [PRIMITIVE-PRESSURE-LEDGER.md](PRIMITIVE-PRESSURE-LEDGER.md); do not broaden it for masked star topology. |
| `game-test-support` | Allowed as dev/test-only proof support for no-leak, topology invariants, and evidence profiles. Production code must not depend on it. |
| `wasm-api` | Allowed as a safe bridge for Rust-owned catalog/setup/view/action/export payloads. It must not decide legality, topology, path, finish, or visibility. |
| static data | Allowed for typed space metadata, IDs, fixtures, presentation labels, and docs. It must not encode selectors, formulas, conditions, triggers, path scripts, or rule behavior. |

## Primitive Pressure Posture

| Primitive-pressure shape | Admission decision | Rationale | Evidence owner |
|---|---|---|---|
| `game-stdlib::board_space` reuse | not applicable | Existing helper is rectangular row/column scaffolding; Starbridge is a masked star. | [PRIMITIVE-PRESSURE-LEDGER.md](PRIMITIVE-PRESSURE-LEDGER.md) |
| graph/topology/path-jump legality | defer/reject promotion | Prior graph pressure is related but not a narrow helper shape; path/jump legality is behavior. | [PRIMITIVE-PRESSURE-LEDGER.md](PRIMITIVE-PRESSURE-LEDGER.md), [docs/MECHANIC-ATLAS.md](../../../docs/MECHANIC-ATLAS.md) |
| finish ranking and progress-vector terminal fallback | local-only | Scoring/outcome/tiebreak behavior remains game-owned. | `RULES.md`, later `RULE-COVERAGE.md` |
| all-public visibility classification | local proof required | Perfect information removes private facts but does not remove visibility evidence. | later `visibility` tests and `GAME-EVIDENCE.md` |

No `game-stdlib` topology/path helper is admitted. No `engine-core` topology
noun is admitted. No foundation amendment is expected.

## Review Checklist

- `engine-core` remains noun-free.
- No `game-stdlib` topology/path/jump helper is created.
- Static data remains typed content, parameters, metadata, fixtures, traces, and reports only.
- TypeScript presents Rust/WASM payloads only.
- The all-public visibility stance is explicit and still tested.
- L0 bots choose only from Rust legal actions and do not compute legality.
- The forward-v1 C-01 through C-10 audit is complete.

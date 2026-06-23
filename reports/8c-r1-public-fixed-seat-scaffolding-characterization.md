# 8C-R1 Public Fixed-Seat Scaffolding Characterization

Status: admission baseline for `8CR1PUBFIXSEA-001`
Date: 2026-06-23
Scope: `race_to_n`, `draughts_lite`, `three_marks`, `column_four`,
`directional_flip`, and `token_bazaar`

This report is the append-only baseline required by
`specs/8c-r1-public-fixed-seat-scaffolding.md` before any R1 migration begins.
It records current repository state only. Ticket `8CR1PUBFIXSEA-001` changes no
Rust code, schema, fixture, trace, hash, replay, seat spelling, WASM output, or
visibility surface.

## Determination

Unit `8C-R1` is locked as the next unit:

- `specs/README.md` records Unit `8C` as `Done` and `8C-R1` as the lowest
  non-`Done` row.
- `docs/MECHANIC-ATLAS.md` is cited by the spec as `Current debt: None`; no
  behavior-primitive promotion debt blocks this scaffolding retrofit.
- `archive/specs/unit-8c-mechanical-scaffolding-code-extraction.md` seeded
  exactly four C-11 waves; this report covers only the first fixed/public
  six-game wave.
- Successors remain `8C-R2`, `8C-R3`, `8C-R4`, then Gate 18. No successor work
  is admitted here.

## Primary Verdict Matrix

| Game | C-01 envelope constructor | C-02 seat grammar/output | C-03 seat count/ring | C-04 action-tree v1 | C-05 writer v1 | C-08 profile drivers |
|---|---|---|---|---|---|---|
| `race_to_n` | already discharged by Unit 8C pilot | migrate output; parser already discharged; native/non-WASM seat spellings excepted | already discharged by Unit 8C pilot; ring N/A | already discharged by Unit 8C pilot | already discharged by Unit 8C pilot | already discharged by Unit 8C pilot |
| `draughts_lite` | migrate | migrate parser and output; native/non-WASM seat spellings excepted | migrate exact count; ring N/A | already discharged by Unit 8C pilot | already discharged by Unit 8C pilot | migrate replay and setup profiles |
| `three_marks` | migrate | migrate parser and output; native/non-WASM seat spellings excepted | migrate exact count; ring N/A | migrate parallel v1 wrappers; legacy hash excepted | migrate only action-tree v1 writer surface; adjacent stable-byte surfaces excepted | migrate replay profile |
| `column_four` | migrate | migrate parser and output; native/non-WASM seat spellings excepted | migrate exact count; ring N/A | migrate parallel v1 wrappers; legacy hash excepted | migrate only action-tree v1 writer surface; adjacent stable-byte surfaces excepted | migrate replay profile |
| `directional_flip` | migrate | migrate parser and output; native/non-WASM seat spellings excepted | migrate exact count; ring N/A | migrate parallel v1 wrappers; legacy hash excepted | migrate only action-tree v1 writer surface; adjacent stable-byte surfaces excepted | migrate replay and setup profiles |
| `token_bazaar` | migrate | migrate parser and output; native/non-WASM seat spellings excepted | migrate exact count; ring N/A | migrate parallel v1 wrappers; legacy hash excepted | migrate only action-tree v1 writer surface; adjacent stable-byte surfaces excepted | migrate replay, setup, and public-export profiles |

## Current C-01 Constructor Shapes

| Game | Current `public_effect` shape | R1 verdict |
|---|---|---|
| `race_to_n` | `EffectEnvelope::public(payload)` in `games/race_to_n/src/effects.rs` | already discharged by Unit 8C pilot |
| `draughts_lite` | `EffectEnvelope { visibility: VisibilityScope::Public, payload }` literal | migrate unchanged |
| `three_marks` | `EffectEnvelope { visibility: VisibilityScope::Public, payload }` literal | migrate unchanged |
| `column_four` | `EffectEnvelope { visibility: VisibilityScope::Public, payload }` literal | migrate unchanged |
| `directional_flip` | `EffectEnvelope { visibility: VisibilityScope::Public, payload }` literal | migrate unchanged |
| `token_bazaar` | `EffectEnvelope { visibility: VisibilityScope::Public, payload }` literal | migrate unchanged |

Required post-migration invariant: payload, effect order, visibility, and effect
hashes stay byte-identical. Token Bazaar must also preserve public-export effect
bytes.

## Current C-02 Seat Spellings

Canonical Rust identity is `seat_<zero-based>`. Legacy hyphen aliases are
accepted only by the Rust WASM import adapter.

| Surface | Current state |
|---|---|
| Kernel strict parser | `SeatId::parse_canonical` accepts canonical underscore spelling and rejects non-canonical labels. |
| WASM import adapter | `crates/wasm-api/src/seats.rs::parse_seat_import` accepts bounded canonical underscore, bounded legacy hyphen, and bounded symbolic aliases. |
| Race typed parser | `RaceSeat::parse` already delegates through `SeatId::parse_canonical`; `as_str()` returns `seat_0` / `seat_1`. |
| Other five typed parsers | Hand-written matches for `seat_0` / `seat_1`; `as_str()` returns `seat_0` / `seat_1`. |
| WASM roster output | Current six `wasm-exported.trace.json` files emit roster `seat-0` / `seat-1`. |
| WASM command actor output | Race, Draughts, and Three Marks currently emit legacy hyphen actor seats in the selected exported document. Column Four, Directional Flip, and Token Bazaar currently emit underscore actor seats while their rosters still use hyphen spellings. |
| Native default seats and non-WASM traces | Accepted exception in R1; unchanged until a separately admitted native replay/trace seat-surface migration. |

Selected `wasm-exported.trace.json` seat-bearing baseline:

| Game | Roster spelling | Command actor spelling | Terminal/outcome seat spelling |
|---|---|---|---|
| `race_to_n` | `seat-0`, `seat-1` | `seat-0` | winner `null` in selected document |
| `draughts_lite` | `seat-0`, `seat-1` | `seat-0` | winner `null` in selected document |
| `three_marks` | `seat-0`, `seat-1` | `seat-0` | winner `null` in selected document |
| `column_four` | `seat-0`, `seat-1` | `seat_0`, `seat_1`, `seat_0` | winner `null` in selected document |
| `directional_flip` | `seat-0`, `seat-1` | `seat_0`, `seat_1` | winner `null` in selected document |
| `token_bazaar` | `seat-0`, `seat-1` | `seat_0`, `seat_1`, `seat_0` | winner `null` in selected document |

## Current C-03 Seat-Count Surfaces

| Game | Exact-count state | Ring/index geometry |
|---|---|---|
| `race_to_n` | already uses Unit 8C `SeatCountRange` pilot | not applicable; typed `other()` mapping remains game-local |
| `draughts_lite` | hand-written exact-two predicate in `setup_match` | not applicable; typed `other()` mapping remains game-local |
| `three_marks` | hand-written exact-two predicate in `setup_match` | not applicable; typed `other()` mapping remains game-local |
| `column_four` | hand-written exact-two predicate in `setup_match` | not applicable; typed `other()` mapping remains game-local |
| `directional_flip` | hand-written exact-two predicate in `setup_match` | not applicable; typed `other()` mapping remains game-local |
| `token_bazaar` | hand-written exact-two predicate in `setup_match`; no current normal `game-stdlib` dependency | not applicable; typed `other()` mapping remains game-local |

Post-migration invariant: `invalid_seat_count` diagnostic code/message,
accepted count, ordering, setup state, replay hashes, and visibility remain
unchanged.

## Current C-04/C-05 Action-Tree Surfaces

Legacy `action_tree_hash` remains the authoritative field for current Trace
Schema v1 replay checks. R1 may add parallel v1 wrappers only for the four
non-pilot games.

| Game / representative trace | Legacy action-tree hash sentinel | Current v1 surface |
|---|---:|---|
| `race_to_n` / `shortest-normal.trace.json` | `8451402319224114161` | present: `action_tree_v1_bytes` / `action_tree_v1_hash` |
| `draughts_lite` / `shortest-quiet.trace.json` | `8126650368011512904` | present: `action_tree_v1_bytes` / `action_tree_v1_hash` |
| `three_marks` / `shortest-normal.trace.json` | `14695981039346656037` | absent; add parallel wrappers later |
| `column_four` / `shortest-normal-win.trace.json` | `14695981039346656037` | absent; add parallel wrappers later |
| `directional_flip` / `opening-legal-move.trace.json` | `16457061400249558986` | absent; add parallel wrappers later |
| `token_bazaar` / `shortest-normal.trace.json` | `6002416109879922099` | absent; add parallel wrappers later |

Adjacent C-05 stable-byte/hash surfaces are accepted exceptions in R1:

| Surface | Games | R1 verdict | Owner / next trigger |
|---|---|---|---|
| State snapshot bytes/hash | all six | exception | game-local replay/serialization owner; reopen under a dedicated state-surface migration |
| Effect bytes/hash | all six | exception | game-local effect serialization; C-01 must prove unchanged bytes |
| Public-view bytes/hash | all six | exception | game-local projection/serialization; reopen under a dedicated public-view migration |
| Replay/export bytes/hash | all six | exception | native replay or Rust/WASM export owner; C-02 owns selected WASM exported documents |
| Dedicated diagnostic hash/bytes | `race_to_n`, `draughts_lite`, `three_marks` | exception | game-local diagnostics; reopen under a diagnostic-surface migration |
| Dedicated diagnostic hash/bytes | `column_four`, `directional_flip`, `token_bazaar` | not applicable | no distinct dedicated diagnostic hash field found in the inspected replay-support surface |

## Selected WASM Exported Trace Digests

These are the before-digests for the only default golden files authorized to
change in R1. Later C-02 output tickets must record after-digests and exact
seat-bearing field deltas one game at a time.

| Game | File | Before SHA-256 |
|---|---|---|
| `race_to_n` | `games/race_to_n/tests/golden_traces/wasm-exported.trace.json` | `cad1053c0d77115d92b14cfb224f4561c9ad69e8973b47df2062cbd742755314` |
| `draughts_lite` | `games/draughts_lite/tests/golden_traces/wasm-exported.trace.json` | `f000a314e7f5295c34bb3da19830792bfaa1d7e2c8d16053e8742903c3bf9c98` |
| `three_marks` | `games/three_marks/tests/golden_traces/wasm-exported.trace.json` | `d9d68e109118de313dec56f264a4525a874246ef382a69ab94f8dd096b5074c6` |
| `column_four` | `games/column_four/tests/golden_traces/wasm-exported.trace.json` | `fa0d4d29471d6ddd7b003f824865383847324a647afba3696fba1d8971307ad3` |
| `directional_flip` | `games/directional_flip/tests/golden_traces/wasm-exported.trace.json` | `d9f7339044c2e4a6e78d433666fddacfda364a6260dd13444510cc67f11fdc36` |
| `token_bazaar` | `games/token_bazaar/tests/golden_traces/wasm-exported.trace.json` | `bad018e566306a9e80fabc6446b1897401378b9c1c64062a46949c044d1d0bd0` |

### 8CR1PUBFIXSEA-013 Race to N after receipt

- File: `games/race_to_n/tests/golden_traces/wasm-exported.trace.json`
- Before SHA-256: `cad1053c0d77115d92b14cfb224f4561c9ad69e8973b47df2062cbd742755314`
- After SHA-256: `f6d764e610436e3f29ac0dbb902ef6f05da10fb46fc762c9fd5b7c2b26c5bdce`
- Seat-bearing delta: roster `seat-0` / `seat-1` -> `seat_0` / `seat_1`;
  command actor `seat-0` -> `seat_0`; winner remains `null`.
- Verification: `cargo test -p wasm-api`; `cargo run -p replay-check -- --game race_to_n --all`;
  `git diff --name-only -- games/*/tests/golden_traces/wasm-exported.trace.json`
  reported only `games/race_to_n/tests/golden_traces/wasm-exported.trace.json`.

Baseline command for full trace digest inventory:

```text
find games/race_to_n/tests/golden_traces games/draughts_lite/tests/golden_traces games/three_marks/tests/golden_traces games/column_four/tests/golden_traces games/directional_flip/tests/golden_traces games/token_bazaar/tests/golden_traces -type f -name '*.json' -print | sort | xargs sha256sum
```

The command was run on 2026-06-23 before any R1 migration. Its output is the
before inventory for proving non-selected traces remain byte-identical during
later migration tickets. No after comparison exists yet because `-001` is
report-only.

## Current C-08 Profile Classification

| Game | `replay-command-v1` | `setup-evidence-v1` | `public-export-v1` | `seat-private-export-v1` | `domain-evidence-v1` |
|---|---|---|---|---|---|
| `race_to_n` | already discharged by Unit 8C pilot; `game-test-support` is already a dev dependency | not applicable: no setup fixture | not applicable in R1 | not applicable: perfect information/no seat-private export | not applicable |
| `draughts_lite` | migrate; validator owner `replay-check`; visibility internal-dev command evidence | migrate standard setup fixture; validator owner `fixture-check`; byte authority none | not applicable | not applicable | not applicable |
| `three_marks` | migrate; validator owner `replay-check`; visibility internal-dev command evidence | not applicable: no setup fixture | not applicable | not applicable | not applicable |
| `column_four` | migrate; validator owner `replay-check`; visibility internal-dev command evidence | not applicable: current standard fixture is command/terminal/diagnostic/bot evidence, not setup evidence | not applicable | not applicable | not applicable |
| `directional_flip` | migrate; validator owner `replay-check`; visibility internal-dev command evidence | migrate standard setup fixture; validator owner `fixture-check`; byte authority none | not applicable | not applicable | not applicable |
| `token_bazaar` | migrate; validator owner `replay-check`; visibility internal-dev command evidence | migrate standard setup fixture; validator owner `fixture-check`; byte authority none | migrate actual `PublicReplayExport`; validator owner Rust/WASM export/import; visibility public | not applicable: observer and seat views are equivalent | not applicable |

Current dependency observation: only `games/race_to_n/Cargo.toml` contains
`game-test-support`. Later C-08 migration tickets may add it only as a
`[dev-dependencies]` edge.

## C-06/C-07/C-09/C-10 Checkpoints

| Checkpoint | Baseline conclusion |
|---|---|
| C-06 dev-only test-support crate | Infrastructure exists. Current R1 wave production crates do not depend on `game-test-support`; only Race has the pilot dev dependency. Final proof belongs to consolidation via `cargo tree --workspace -e normal --invert game-test-support`. |
| C-07 no-leak geometry | Not applicable for this six-game public/perfect-information wave. Existing public visibility tests remain regression evidence; no hidden source datum or unauthorized seat-private viewer pair is introduced. |
| C-09 unbiased RNG | Not applicable for R1. No selected game-rule bounded-index sampling surface is admitted for migration; bot RNG and existing RNG consumption remain unchanged. |
| C-10 non-promotion | Every planned change is behavior-free scaffolding adoption or evidence classification. No legality, setup policy, reveal, projection, scoring, outcome, bot, diagnostic policy, TypeScript authority, YAML, DSL, or static behavior moves into shared code. |

## Accepted Exceptions And Review Triggers

| Exception | Owner | Risk | Compatibility window | Rollback | Next trigger |
|---|---|---|---|---|---|
| Native `default_seats` and non-WASM legacy trace spellings | native replay/trace owners per game | mixed seat spellings persist outside selected WASM exports | remain readable until separately admitted native replay/trace migration | no change in R1; rollback N/A | named native replay/trace seat-surface migration |
| Legacy `action_tree_hash` remains authoritative in Trace Schema v1 | game replay-support owners | parallel identity may confuse callers if undocumented | R1 keeps legacy fields unchanged and adds v1 only as parallel surface | remove new v1 wrappers/tests per game | future authority flip with ADR-0009 surface packet |
| Adjacent state/effect/view/replay/export stable bytes | owning game replay/serialization/export modules | broad byte drift if swept together | R1 excludes these surfaces except selected WASM export seat spelling | no change in R1; rollback N/A | dedicated per-surface migration |
| Public perfect-information C-07 geometry | owning game visibility tests | false hidden-info claim if pairwise no-leak added without hidden datum | not applicable for R1 | no C-07 artifact added | future hidden-info/private-view surface |
| C-09 unbiased RNG | owning game rule/setup surfaces | seed/vector drift if adopted without a characterized sampler | not applicable for R1 | no RNG change | separately admitted RNG migration packet |

## Command Evidence

Commands were run from repository root on 2026-06-23.

| Command | Result | Notes |
|---|---|---|
| `cargo test --workspace --all-targets` | passed, exit 0 | This workspace command executes benchmark harness binaries as all-target test artifacts. Some local benchmark rows printed `pass:false` for benchmark thresholds while the command itself exited 0; this is baseline test evidence, not benchmark-gate evidence. |
| `cargo run -p replay-check -- --game race_to_n --all` | passed, exit 0 | All Race traces passed; `not-applicable` trace accepted. |
| `cargo run -p replay-check -- --game draughts_lite --all` | passed, exit 0 | All Draughts traces passed. Parallel run waited briefly for Cargo artifact lock. |
| `cargo run -p replay-check -- --game three_marks --all` | passed, exit 0 | All Three Marks traces passed; `not-applicable` trace accepted. Parallel run waited briefly for Cargo artifact lock. |
| `cargo run -p replay-check -- --game column_four --all` | passed, exit 0 | All Column Four traces passed. |
| `cargo run -p replay-check -- --game directional_flip --all` | passed, exit 0 | All Directional Flip traces passed. Parallel run waited briefly for Cargo artifact lock. |
| `cargo run -p replay-check -- --game token_bazaar --all` | passed, exit 0 | All Token Bazaar traces passed. |
| `sha256sum .../wasm-exported.trace.json` | passed, exit 0 | Selected before-digests recorded above. |
| `find ... golden_traces ... | sort | xargs sha256sum` | passed, exit 0 | Full before inventory captured for later non-selected trace byte comparisons. |

Ticket `8CR1PUBFIXSEA-001` did not run the full §7 final command set because
the unit has not migrated code yet. Those commands remain required at the
consolidated verification ticket and final spec closeout.

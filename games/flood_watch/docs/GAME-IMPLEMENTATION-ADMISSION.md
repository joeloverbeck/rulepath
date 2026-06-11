# flood_watch Implementation Admission

Game ID: `flood_watch`

Public display name: `Flood Watch`

Implemented variants: `flood_watch_standard`, `flood_watch_deluge`

Roadmap stage/gate: Gate 12 - cooperative event-pressure proof

Public role: original portfolio game / hidden-info proof

Prepared by: Codex

Date: 2026-06-11

## Purpose

This is the Gate 12 admission and completion receipt for Flood Watch. It records that the cooperative shared-outcome, event-deck pressure, role-power, scenario, multi-action budget, bot, WASM, browser, and no-leak evidence is complete enough to admit the game as the Rulepath cooperative event-pressure proof.

Admission does not waive later maintenance: rule IDs, replay hashes, no-leak surfaces, benchmark thresholds, and documentation must remain stable or be intentionally migrated.

## Prerequisite documents

| Prerequisite | Path | Complete? | Notes/blockers |
|---|---|---:|---|
| source/IP notes | `games/flood_watch/docs/SOURCES.md` | yes | Original rules, naming, and prior-art boundaries recorded. |
| original rules with stable rule IDs | `games/flood_watch/docs/RULES.md` | yes | Stable `FW-*` IDs cover setup, scenarios, actions, environment, visibility, bot, and terminal rules. |
| rule coverage matrix | `games/flood_watch/docs/RULE-COVERAGE.md` | yes | `cargo run -p rule-coverage -- --game flood_watch` passes. |
| mechanic inventory | `games/flood_watch/docs/MECHANICS.md` | yes | Cooperative, event-deck, budget, role, scenario, and hidden-order mechanics recorded as game-local. |
| primitive-pressure ledger | `games/flood_watch/docs/PRIMITIVE-PRESSURE-LEDGER.md` | yes | Reaction-window and deterministic-shuffle reviews completed; no promotion debt opened. |
| competent-player analysis | `games/flood_watch/docs/COMPETENT-PLAYER.md` | yes | Cooperative priorities and bot expectations recorded. |
| ADR, if boundary-changing | not applicable | n/a | No `engine-core`, `game-stdlib`, schema, or policy ADR was required. |

## Source and IP readiness

| Check | Status | Evidence/notes |
|---|---|---|
| consulted sources recorded with dates | ready | `SOURCES.md` records references and use limits. |
| sources used only for verification/context | ready | Names, labels, rules prose, and UI copy are original Rulepath expression. |
| Rulepath rules prose is original | ready | `RULES.md` and `HOW-TO-PLAY.md` are original prose. |
| no copied card/component text | ready | District, role, and event labels are original. |
| no copied art/icons/screenshots/scans/fonts/trade dress | ready | No external assets ship; the web shell uses existing system styling. |
| public naming rationale recorded | ready | `SOURCES.md` records the neutral original name rationale. |
| private licensed content excluded from public files | not applicable | No private licensed content used. |
| human/legal review triggers cleared or recorded | ready | No proprietary text, art, or trademark-forward expression identified. |

## Rule-ID and coverage readiness

| Check | Status | Evidence/notes |
|---|---|---|
| every implemented rule has a stable rule ID | ready | `RULES.md` uses stable `FW-*` rows. |
| shared terminal IDs are explainable | ready | `FW-END-001` and `FW-END-002` are mirrored by browser outcome templates and UI docs. |
| coverage matrix has one row per rule ID | ready | `RULE-COVERAGE.md`; `rule-coverage` passes. |
| primary Rust test strategy is implemented | ready | Unit, rule, property, replay, serialization, visibility, and bot tests pass in `cargo test --workspace`. |
| golden traces are implemented | ready | Flood Watch traces under `games/flood_watch/tests/golden_traces/`; `replay-check --all` passes. |
| visibility/no-leak requirements are implemented | ready | Native visibility tests, WASM tests, and `flood-watch.smoke.mjs` pass. |

## Mechanic inventory readiness

| Check | Status | Evidence/notes |
|---|---|---|
| all mechanic atlas categories are inventoried | ready | `MECHANICS.md` and `PRIMITIVE-PRESSURE-LEDGER.md`. |
| local mechanics are named and scoped | ready | Districts, flood levels, levees, event cards, roles, scenarios, budgets, and shared outcome remain game-local. |
| repeated-shape comparison is complete | ready | Reaction-window review says not reaction-capable; deterministic shuffle review says not a full private-holding fifth use. |
| repo atlas update required? | no open debt | `docs/MECHANIC-ATLAS.md` §10B records Gate 12 first-use rows and §10A remains `_None_`. |

## Primitive-pressure status

| Mechanic shape | Status | Decision/evidence | Blocks implementation? |
|---|---|---|---:|
| deterministic shuffle / private hand / staged reveal | not a fifth full use | Flood Watch shuffles a hidden event deck and stages public forecast/draw reveal, but has no per-seat private holdings. | no |
| reaction window / pending response | not applicable | Environment automation is not a responder choice window. | no |
| shared-outcome cooperative terminal | local-only first official use | Both seats win on deck exhaustion or lose on district inundation; no generic helper authorized. | no |
| event-deck environment automation | local-only first official use | Environment resolution is a Rust consequence of turn-ending/final-budget commands. | no |
| role-modified action effects | local-only first official use | Pumpwright and Levee Warden modifiers stay in game-local Rust validation/application. | no |
| multi-action turn budgets | local-only first official use | Budget accounting and legal-tree regeneration stay game-local. | no |

## Engine-core and static-data review

| Boundary check | Result | Notes |
|---|---|---|
| no game noun enters `engine-core` | pass | `engine-core` remains generic and noun-free. |
| no generic cooperative/event/budget/role helper added | pass | All Gate 12 mechanic shapes stay in `games/flood_watch`. |
| static data limited to typed metadata/fixtures | pass | Variant and fixture data contain counts/constants only, not behavior scripts. |
| no YAML/DSL introduced | pass | Existing validated TOML/JSON patterns only. |

## Hidden-information risk review

| Surface | Risk level | Required safeguard/test | Admission status |
|---|---|---|---|
| public/browser payload | high | native visibility tests, WASM redaction tests, browser no-leak smoke | ready |
| action tree | medium | active-seat legal tree and teammate/observer empty-tree tests | ready |
| diagnostics/effect log | medium | viewer-safe diagnostics/effects and effect-feedback smoke | ready |
| DOM/test IDs/local storage/replay export | high | `apps/web/e2e/flood-watch.smoke.mjs` | ready |
| bot explanations/candidate rankings | medium | bot tests and public policy/effect smoke | ready |
| dev inspector | medium | browser no-leak smoke and viewer-safe dev panel boundary | ready |

## Bot level

| Level | Required before public release? | Evidence |
|---:|---:|---|
| 0 random legal | yes | Legal decisions validate across seeds and simulations. |
| 1 baseline | yes | `FloodWatchLevel1Bot` covers either role/seat with public-view-only priorities and explanations. |
| 2 authored policy | no | Not shipped. |
| 3 shallow deterministic search | no | Not allowed/needed for this cooperative hidden-order gate. |

## UI exposure

| Check | Status | Notes |
|---|---|---|
| web-exposed in this stage? | yes | `FloodWatchBoard.tsx` is registered in the React shell. |
| legal-action tree maps to UI controls | ready | District and turn controls render only from Rust action-tree segments. |
| TypeScript presentation-only boundary understood | ready | TS displays Rust/WASM views, effects, action trees, and replay data only. |
| effect-driven animation expectations identified | ready | Storm/event effects drive browser feedback; reduced motion is covered. |
| accessibility/reduced-motion/responsive expectations identified | ready | Covered by `flood-watch.smoke.mjs` and full `smoke:e2e`. |

## Verification transcript

Commands below passed during Gate 12 closeout on 2026-06-11:

- `cargo test --workspace`
- `bash scripts/boundary-check.sh`
- `cargo run -p simulate -- --game flood_watch --games 1000`
- `cargo run -p replay-check -- --game flood_watch --all`
- `cargo run -p fixture-check -- --game flood_watch`
- `cargo run -p rule-coverage -- --game flood_watch`
- `npm --prefix apps/web run smoke:wasm`
- `npm --prefix apps/web run smoke:ui`
- `npm --prefix apps/web run smoke:effects`
- `npm --prefix apps/web run smoke:e2e`
- `node scripts/check-catalog-docs.mjs`
- `node scripts/check-player-rules.mjs`
- `node scripts/check-outcome-explanations.mjs`

`node scripts/check-doc-links.mjs` is run after this document and `PUBLIC-RELEASE-CHECKLIST.md` are added.

## Admission decision

Decision: admitted

Decision rationale:

- Flood Watch satisfies Gate 12 with Rust-owned cooperative legality, shared terminal outcomes, deterministic event-deck pressure, local role powers, scenario setup, multi-action budgets, public-view-only bots, native tools, WASM bridge, browser board, generated player rules, outcome explanations, E2E no-leak coverage, and catalog documentation.

Explicit constraints:

- No generic shared-outcome, event-deck, role-power, scenario, or action-budget helper is admitted.
- The undrawn event-deck order remains redacted from every public/browser/replay/bot surface forever.
- Future repeated cooperative/event/budget/role mechanics must reopen the mechanic-atlas rows before promotion.

## Blocking issues

| Issue | Required fix | Owner | Blocks coding? |
|---|---|---|---:|
| None | Not applicable | Rulepath maintainers | no |

## Sign-off

Prepared by: Codex

Reviewed by: Rulepath maintainers

Date: 2026-06-11

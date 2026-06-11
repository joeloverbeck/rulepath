# masked_claims Implementation Admission

Game ID: `masked_claims`

Public display name: `Masked Claims`

Implemented variant: `masked_claims_standard`

Roadmap stage/gate: Gate 11 - bluffing / reaction-window proof

Public role: hidden-info proof

Prepared by: Codex

Date: 2026-06-11

## Purpose

This is the Gate 11 admission and completion receipt for Masked Claims. It records that implementation, evidence, and web exposure are complete enough to admit the game as the Rulepath reaction-window proof.

Admission does not waive later maintenance: rule IDs, replay hashes, no-leak surfaces, benchmarks, and documentation must remain stable or be intentionally migrated.

## Prerequisite documents

| Prerequisite | Path | Complete? | Notes/blockers |
|---|---|---:|---|
| source/IP notes | `games/masked_claims/docs/SOURCES.md` | yes | Original rules, naming, and consulted prior-art boundaries recorded. |
| original rules with stable rule IDs | `games/masked_claims/docs/RULES.md` | yes | Stable `MC-*` IDs cover setup, actions, scoring, visibility, terminal outcomes, and no-leak rules. |
| rule coverage matrix | `games/masked_claims/docs/RULE-COVERAGE.md` | yes | `cargo run -p rule-coverage -- --game masked_claims` passes. |
| mechanic inventory | `games/masked_claims/docs/MECHANICS.md` | yes | Game-local nouns and repeated-shape comparisons recorded. |
| primitive-pressure ledger | `games/masked_claims/docs/PRIMITIVE-PRESSURE-LEDGER.md` | yes | Fourth-use shuffle/private-hand review defer-rejects extraction; reaction window is first official local use. |
| competent-player analysis | `games/masked_claims/docs/COMPETENT-PLAYER.md` | yes | Baseline bluff/response strategy and bot expectations recorded. |
| ADR, if boundary-changing | not applicable | n/a | No `engine-core`, `game-stdlib`, schema, or policy ADR was required. |

## Source and IP readiness

| Check | Status | Evidence/notes |
|---|---|---|
| consulted sources recorded with dates | ready | `SOURCES.md` records references and use limits. |
| sources used only for verification/context | ready | Rules, names, labels, and prose are original Rulepath expression. |
| Rulepath rules prose is original | ready | `RULES.md` and `HOW-TO-PLAY.md` are original prose. |
| no copied card/component text | ready | Mask grades and labels are original; no role/card roster exists. |
| no copied art/icons/screenshots/scans/fonts/trade dress | ready | No external assets ship. Web uses existing system presentation. |
| public naming rationale recorded | ready | `SOURCES.md` records the neutral original name rationale. |
| private licensed content excluded from public files | not applicable | No private licensed content used. |
| human/legal review triggers cleared or recorded | ready | No proprietary text, art, or trademark-forward expression identified. |

## Rule-ID and coverage readiness

| Check | Status | Evidence/notes |
|---|---|---|
| every implemented rule has a stable rule ID | ready | `RULES.md` uses stable `MC-*` rows. |
| scoring and terminal IDs are explainable | ready | `MC-SCORE-*` and `MC-END-*` are mirrored by outcome docs/templates. |
| coverage matrix has one row per rule ID | ready | `RULE-COVERAGE.md`; `rule-coverage` passes. |
| primary Rust test strategy is implemented | ready | Unit, integration, property, replay, serialization, visibility, and bot tests pass in `cargo test --workspace`. |
| golden traces are implemented | ready | Seventeen traces under `games/masked_claims/tests/golden_traces/`; `replay-check --all` passes. |
| visibility/no-leak requirements are implemented | ready | Native visibility tests, WASM tests, and `masked-claims.smoke.mjs` pass. |

## Mechanic inventory readiness

| Check | Status | Evidence/notes |
|---|---|---|
| all mechanic atlas categories are inventoried | ready | `MECHANICS.md` and `PRIMITIVE-PRESSURE-LEDGER.md`. |
| local mechanics are named and scoped | ready | Mask, grade, claim, pedestal, reaction, gallery, exposed row, and counters remain game-local. |
| repeated-shape comparison is complete | ready | Deterministic shuffle/private-hand/staged reveal fourth-use review completed. |
| third/fourth-use hard gate is cleared | ready | Extraction deferred/rejected with rationale; no promotion debt opened. |
| repo atlas update required? | no | `docs/MECHANIC-ATLAS.md` already records `masked_claims` and reaction-window first use. |

## Primitive-pressure status

| Mechanic shape | Status | Decision/evidence | Blocks implementation? |
|---|---|---|---:|
| deterministic shuffle / private hand / staged reveal | rejected/deferred with rationale | Local because mask hands, reserve, accepted-never-revealed masks, challenged one-mask reveal, claim-path redaction, and bot inputs are game-specific. | no |
| simultaneous commitment/reveal + visible draft-pool removal | not a second use | Masked Claims uses sequential claim plus response, not synchronized commitment or draft-pool removal. | no |
| reaction window / pending response | local-only first official use | One accept/challenge window with responder-only choices and claimant waiting metadata; broad generalization remains ADR-required. | no |

## Engine-core and static-data review

| Boundary check | Result | Notes |
|---|---|---|
| no game noun enters `engine-core` | pass | `engine-core` remains generic and noun-free. |
| no generic reaction helper added | pass | Reaction-window behavior stays in `games/masked_claims`. |
| static data limited to typed metadata/fixtures | pass | Manifest, variant, fixture, and benchmark data contain no behavior scripts. |
| no YAML/DSL introduced | pass | Existing validated TOML/JSON patterns only. |

## Hidden-information risk review

| Surface | Risk level | Required safeguard/test | Admission status |
|---|---|---|---|
| public/browser payload | high | native visibility tests, WASM redaction tests, browser no-leak smoke | ready |
| action tree | high | actor/viewer-scoped action-tree tests and browser response-control smoke | ready |
| diagnostics/effect log | medium | viewer-safe diagnostics/effects tests and effect feedback smoke | ready |
| DOM/test IDs/local storage/replay export | high | `apps/web/e2e/masked-claims.smoke.mjs` | ready |
| bot explanations/candidate rankings | medium | bot tests and public effect redaction | ready |
| dev inspector | medium | viewer-filtered dev panel smoke | ready |

## Bot level

| Level | Required before public release? | Evidence |
|---:|---:|---|
| 0 random legal | yes | Legal decisions validate across seeds and simulations. |
| 1 baseline | yes | `MaskedClaimsLevel1Bot` covers claim and response roles with deterministic, viewer-safe explanations. |
| 2 authored policy | no | Not shipped. |
| 3 shallow deterministic search | no | Not allowed/needed for this hidden-information gate. |

## UI exposure

| Check | Status | Notes |
|---|---|---|
| web-exposed in this stage? | yes | `MaskedClaimsBoard.tsx` is registered in the React shell. |
| legal-action tree maps to UI controls | ready | Claim and response controls render only from Rust action trees. |
| TypeScript presentation-only boundary understood | ready | TS displays Rust/WASM views, effects, action trees, and replay data only. |
| effect-driven animation expectations identified | ready | Reveal/claim effects drive browser feedback; reduced motion is covered. |
| accessibility/reduced-motion/responsive expectations identified | ready | Covered by `masked-claims.smoke.mjs` and full `smoke:e2e`. |

## Verification transcript

All commands below passed on 2026-06-11:

- `cargo test --workspace`
- `bash scripts/boundary-check.sh`
- `cargo run -p simulate -- --game masked_claims --games 1000`
- `cargo run -p replay-check -- --game masked_claims --all`
- `cargo run -p fixture-check -- --game masked_claims`
- `cargo run -p rule-coverage -- --game masked_claims`
- `npm --prefix apps/web run smoke:wasm`
- `npm --prefix apps/web run smoke:ui`
- `npm --prefix apps/web run smoke:e2e`
- `node scripts/check-catalog-docs.mjs`
- `node scripts/check-player-rules.mjs`
- `node scripts/check-outcome-explanations.mjs`

`node scripts/check-doc-links.mjs` is run after this document and `PUBLIC-RELEASE-CHECKLIST.md` are added.

## Admission decision

Decision: admitted

Decision rationale:

- Masked Claims satisfies the Gate 11 claim/challenge/reaction-window proof with Rust-owned legality, conditional resolution, deterministic replay/visibility, bots, benchmarks, official-game docs, browser UI, generated player rules, outcome explanations, CI/tool registration, and no-leak evidence.

Explicit constraints:

- No generic reaction-window, claim, bluff, mask, or reveal helper is admitted.
- Accepted masks, unplayed hand masks, and the reserve remain redacted forever.
- Future repeated reaction-window mechanics must reopen the atlas row before promotion.

## Blocking issues

| Issue | Required fix | Owner | Blocks coding? |
|---|---|---|---:|
| None | Not applicable | Rulepath maintainers | no |

## Sign-off

Prepared by: Codex

Reviewed by: Rulepath maintainers

Date: 2026-06-11

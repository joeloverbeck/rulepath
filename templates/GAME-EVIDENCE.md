# <game_id> Game Evidence Receipt

Game ID: `<game_id>`

Rules version: `<rules_version>`

Data/manifest version: `<data_or_manifest_version>`

Trace/profile version set: `<profile_ids_and_versions>`

Engine version: `<engine_version>`

Prepared by: `<name/agent>`

Last updated: YYYY-MM-DD

Template realignment mapping: report `B-13 -> GAME-EVIDENCE.md`

## Purpose

This receipt is a status and artifact-link index for official-game conformance.
It does not replace the domain templates and MUST NOT duplicate rules prose,
strategy prose, UI prose, behavior tables, rule data, or hidden information.

Use explicit `not applicable: <rationale>` entries when an evidence surface does
not apply. Silent omissions are not allowed.

## Completion Profile

| Field | Value |
|---|---|
| Completion profile | full / minimal-perfect-information / hidden-information / n-seat / release-candidate / intentionally-deferred / private-milestone-1-rule-complete / private-release-candidate / public-release-candidate |
| Profile rationale | `<why this profile applies>` |
| Not applicable summary | `<links to explicit rows below; no silent omissions>` |
| Deferred checker surface | future `GAME-EVIDENCE` checker / not applicable |
| Foundation invariants status | pass/fail/blocker |
| Stop-condition review | no stop condition / blocker: `<FOUNDATIONS §12 item>` |

Completion profile selection never waives a
[FOUNDATIONS.md](../docs/FOUNDATIONS.md) §11 invariant or §12 stop condition.

## Supported Seats and Variants

| Surface | Status | Artifact link | Notes |
|---|---|---|---|
| Supported seat counts | complete/partial/not applicable | `<GAME-RULES.md#seat-model>` | `<notes>` |
| Implemented variants | complete/partial/not applicable | `<GAME-SOURCES.md variant section>` | `<notes>` |
| Seat roles/labels | complete/partial/not applicable | `<GAME-RULES.md or GAME-UI.md link>` | `<notes>` |
| N-seat obligations | complete/partial/not applicable: `<rationale>` | `<coverage or UI link>` | `<notes>` |

## Source and IP Receipt

| Check | Status | Artifact link | Notes |
|---|---|---|---|
| Source notes complete | pass/fail/blocker | `<GAME-SOURCES.md>` | `<notes>` |
| Original rules prose complete | pass/fail/blocker | `<GAME-RULES.md>` | `<notes>` |
| Public name/trade-dress review | pass/fail/not applicable: `<rationale>` | `<PUBLIC-RELEASE-CHECKLIST.md link>` | `<notes>` |
| Assets/fonts/license review | pass/fail/not applicable: `<rationale>` | `<PUBLIC-RELEASE-CHECKLIST.md link>` | `<notes>` |
| Private-source exclusion | pass/fail/blocker | `<PUBLIC-RELEASE-CHECKLIST.md link>` | `<notes>` |
| Private build/source receipt | pass/fail/not applicable: `<rationale>` | `<PRIVATE-RELEASE-CHECKLIST.md or private receipt>` | `<notes>` |
| No publisher flowchart/priority chart copied into bot docs/tests/policy | pass/fail/not applicable: `<rationale>` | `<source/bot review link>` | `<notes>` |

## Rule-Coverage Summary

| Evidence surface | Status | Artifact link | Notes |
|---|---|---|---|
| Rule coverage matrix | complete/partial/blocker | `<GAME-RULE-COVERAGE.md>` | `<notes>` |
| Unit and named rule tests | pass/fail/not applicable: `<rationale>` | `<test link>` | `<notes>` |
| Property/invariant tests | pass/fail/not applicable: `<rationale>` | `<test link>` | `<notes>` |
| Simulation/fuzz runs | pass/fail/not applicable: `<rationale>` | `<command or report link>` | `<notes>` |
| Serialization coverage | pass/fail/not applicable: `<rationale>` | `<test link>` | `<notes>` |

## Named Trace Profiles

Use profile names from
[EVIDENCE-FIXTURE-CONTRACT.md](../docs/EVIDENCE-FIXTURE-CONTRACT.md).

| Profile ID | Profile version | Visibility class | Validator owner | Artifact link | Status | Notes |
|---|---|---|---|---|---|---|
| `replay-command-v1` | `v1` | internal-dev/public | fixture-check / replay-check | `<trace link>` | pass/fail/not applicable: `<rationale>` | `<notes>` |
| `public-export-v1` | `v1` | public | Rust/WASM export / import smoke | `<export link>` | pass/fail/not applicable: `<rationale>` | `<notes>` |
| `seat-private-export-v1` | `v1` | seat-private | Rust/WASM export / pairwise no-leak harness | `<export link>` | pass/fail/not applicable: `<rationale>` | `<notes>` |
| `setup-evidence-v1` | `v1` | public/viewer-scoped/seat-private/internal-dev | fixture/static-data validator | `<fixture link>` | pass/fail/not applicable: `<rationale>` | `<notes>` |
| `domain-evidence-v1` | `v1` | public/viewer-scoped/seat-private/internal-dev/private-source | game-local validator | `<fixture link>` | pass/fail/not applicable: `<rationale>` | `<notes>` |

## Viewer Matrix

| Viewer class | Public view evidence | Seat-private view evidence | Action/effect/diagnostic evidence | Replay/export evidence | Status |
|---|---|---|---|---|---|
| public observer | `<artifact link>` | not applicable: `<rationale>` | `<artifact link>` | `<artifact link>` | pass/fail/not applicable |
| seat `<seat_id>` | `<artifact link>` | `<artifact link>` | `<artifact link>` | `<artifact link>` | pass/fail/not applicable |
| additional viewer class | `<artifact link or not applicable: rationale>` | `<artifact link or not applicable: rationale>` | `<artifact link or not applicable: rationale>` | `<artifact link or not applicable: rationale>` | pass/fail/not applicable |

Hidden-information, asymmetric-view, partnership, or 3+ seat games must link the
pairwise no-leak matrix and per-seat outcome evidence.

## Hidden-Information No-Leak Matrix

Fill every row for hidden-information, asymmetric-view, partnership, or 3+
seat games. Perfect-information games may mark rows `not applicable` with a
rationale. This matrix owns the detailed no-leak proof formerly repeated in the
public release checklist.

| Surface | Status | Evidence/test | Notes |
|---|---|---|---|
| public/browser payload | pass/fail/not applicable: `<rationale>` | `<test/report link>` | `<notes>` |
| public view | pass/fail/not applicable: `<rationale>` | `<test/report link>` | `<notes>` |
| action tree | pass/fail/not applicable: `<rationale>` | `<test/report link>` | `<notes>` |
| previews | pass/fail/not applicable: `<rationale>` | `<test/report link>` | `<notes>` |
| diagnostics/disabled reasons | pass/fail/not applicable: `<rationale>` | `<test/report link>` | `<notes>` |
| effect logs | pass/fail/not applicable: `<rationale>` | `<test/report link>` | `<notes>` |
| command logs | pass/fail/not applicable: `<rationale>` | `<test/report link>` | `<notes>` |
| DOM attributes | pass/fail/not applicable: `<rationale>` | `<test/report link>` | `<notes>` |
| test IDs | pass/fail/not applicable: `<rationale>` | `<test/report link>` | `<notes>` |
| browser console/logs | pass/fail/not applicable: `<rationale>` | `<test/report link>` | `<notes>` |
| local storage/session storage | pass/fail/not applicable: `<rationale>` | `<test/report link>` | `<notes>` |
| replay export/import | pass/fail/not applicable: `<rationale>` | `<test/report link>` | `<notes>` |
| bot explanations | pass/fail/not applicable: `<rationale>` | `<test/report link>` | `<notes>` |
| candidate rankings | pass/fail/not applicable: `<rationale>` | `<test/report link>` | `<notes>` |
| dev inspector/public build boundary | pass/fail/not applicable: `<rationale>` | `<test/report link>` | `<notes>` |

## Replay and Hash Compatibility

| Surface | Version/status | Artifact link | Notes |
|---|---|---|---|
| Replay import/export compatibility | pass/fail/not applicable: `<rationale>` | `<replay-check/export smoke link>` | `<notes>` |
| Hash surface version | `<hash_surface_version or not applicable: rationale>` | `<test/report link>` | `<notes>` |
| Canonical byte authority | Rust replay/fixture validator / Rust/WASM export / none | `<contract or validator link>` | `<notes>` |
| Migration/update note | none / `<migration note link>` | `<artifact link>` | `<notes>` |

## Benchmarks and Bot Policy

| Surface | Status | Artifact link | Notes |
|---|---|---|---|
| Benchmark workload IDs | pass/fail/not applicable: `<rationale>` | `<GAME-BENCHMARKS.md or bench link>` | `<ids>` |
| Benchmark floor/variance note | pass/fail/not applicable: `<rationale>` | `<report link>` | `<notes>` |
| Bot levels shipped | Level 0 / Level 1 / Level 2 / not applicable: `<rationale>` | `<GAME-AI.md>` | `<notes>` |
| Bot policy IDs | pass/fail/not applicable: `<rationale>` | `<GAME-AI.md or BOT-STRATEGY-EVIDENCE-PACK.md>` | `<ids>` |
| Bot explanation safety | pass/fail/not applicable: `<rationale>` | `<test/report link>` | `<notes>` |
| Level 0 deferral status | not deferred / deferred only for private-milestone-1-rule-complete / blocker | `<GAME-AI.md / ticket/spec gate>` | `<closure gate>` |
| Asymmetric role bot coverage | pass/fail/not applicable: `<rationale>` | `<GAME-AI.md role matrix>` | `<roles/factions covered>` |

## Mechanic and Scaffolding Decisions

| Decision surface | Status | Artifact link | Notes |
|---|---|---|---|
| Mechanic inventory | complete/partial/blocker | `<GAME-MECHANICS.md>` | `<notes>` |
| Primitive-pressure ledger | complete/not applicable: `<rationale>`/blocker | `<PRIMITIVE-PRESSURE-LEDGER.md or atlas link>` | `<notes>` |
| Pre-implementation mechanical-scaffolding reuse-first audit | complete/blocker | `<GAME-MECHANICS.md audit section>` | `<reused MSC ids, exceptions, anticipated new shapes>` |
| Existing registered/promoted scaffolding adoption | complete/not applicable: `<rationale>`/blocker | `<MSC entries and code/test evidence>` | `<notes>` |
| Post-implementation new-scaffolding/register-freshness receipt | no new scaffolding / register updated / blocker | `<MECHANICAL-SCAFFOLDING-REGISTER.md entry ids>` | `<new sites, decision states, next review triggers>` |
| Prior-game duplication/refactor disposition | no prior match / follow-on unit queued / accepted local-only / accepted deferred / accepted rejected / blocker | `<specs/README.md unit or register decision>` | `<migration set, owner, next review trigger>` |
| CI scaffolding-audit record | pass/fail/blocker | `<ci/scaffolding-audits.json row>` | `<known signal dispositions>` |
| Open behavioral promotion/scaffolding debt | none / blocker / deferred by accepted exception | `<artifact link>` | `<notes>` |

## Release State and Blockers

| Surface | Status | Artifact link | Notes |
|---|---|---|---|
| Implementation admission | pass/fail/blocker | `<GAME-IMPLEMENTATION-ADMISSION.md>` | `<notes>` |
| UI evidence | pass/fail/not applicable: `<rationale>` | `<GAME-UI.md or smoke link>` | `<notes>` |
| Public release checklist | pass/fail/not applicable: `<rationale>` | `<PUBLIC-RELEASE-CHECKLIST.md>` | `<notes>` |
| Private release checklist | pass/fail/not applicable: `<rationale>` | `<PRIVATE-RELEASE-CHECKLIST.md>` | `<notes>` |
| Known blockers | none / `<blocker list>` | `<ticket/spec link>` | `<notes>` |
| Human/legal review | complete/pending/not applicable: `<rationale>` | `<review link>` | `<notes>` |

## Artifact Links

| Artifact | Required? | Link | Status |
|---|---:|---|---|
| `GAME-SOURCES.md` | yes | `<path>` | complete/partial/blocker |
| `GAME-RULES.md` | yes | `<path>` | complete/partial/blocker |
| `GAME-RULE-COVERAGE.md` | yes | `<path>` | complete/partial/blocker |
| `GAME-MECHANICS.md` | yes | `<path>` | complete/partial/blocker |
| `GAME-IMPLEMENTATION-ADMISSION.md` | yes | `<path>` | complete/partial/blocker |
| `GAME-HOW-TO-PLAY.md` | yes | `<path>` | complete/partial/blocker |
| `COMPETENT-PLAYER.md` | profile-dependent | `<path or not applicable: rationale>` | complete/partial/not applicable/blocker |
| `BOT-STRATEGY-EVIDENCE-PACK.md` | profile-dependent | `<path or not applicable: rationale>` | complete/partial/not applicable/blocker |
| `GAME-AI.md` | yes | `<path>` | complete/partial/blocker |
| `GAME-UI.md` | web-exposed games | `<path or not applicable: rationale>` | complete/partial/not applicable/blocker |
| `GAME-BENCHMARKS.md` | yes | `<path>` | complete/partial/blocker |
| `PRIMITIVE-PRESSURE-LEDGER.md` | when pressure exists | `<path or not applicable: rationale>` | complete/partial/not applicable/blocker |
| `PUBLIC-RELEASE-CHECKLIST.md` | before public release | `<path or not applicable: rationale>` | complete/partial/not applicable/blocker |
| `PRIVATE-RELEASE-CHECKLIST.md` | before private release | `<path or not applicable: rationale>` | complete/partial/not applicable/blocker |

## Receipt Review Checklist

- This receipt contains status, rationale, and artifact links only.
- No rule data, procedural configuration, hidden state, copied source prose, or
  duplicated domain prose appears here.
- Every `not applicable` entry has a reason.
- Every referenced artifact link resolves.
- Completion profile selection does not waive any §11 invariant or §12 stop
  condition in [FOUNDATIONS.md](../docs/FOUNDATIONS.md).
- The pre-implementation audit and post-implementation register receipt are distinct and both current.
- Every new behavior-free shape has a register decision and next review trigger.
- Every prior-game match has a tracker unit or accepted no-unit disposition.
- The CI audit record agrees with this receipt and the register.
- Any byte/hash/fixture/export/RNG/visibility change cites ADR 0009 migration authority.

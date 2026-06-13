# <game_id> Public Release Checklist

Game ID: `<game_id>`

Public display name: `<display_name>`

Implemented variant: `<variant>`

Release target: local preview / public web build / portfolio demo / tagged release / other: `<target>`

URL/build artifact if applicable: `<url_or_artifact>`

Rules version: `<rules_version>`

Data/manifest version: `<data_or_manifest_version>`

Engine version: `<engine_version>`

Prepared by: `<name/agent>`

Date: YYYY-MM-DD

## Public shipment rule

If content ships to an unauthorized browser, it has shipped.

Public release includes any linked route, hosted artifact, downloadable build, exported replay containing unsafe content, public source path, public package, screenshot, demo video, or browser payload that exposes content to users who are not authorized to see it.

## Official-game contract status

| Requirement | Status | Evidence/notes |
|---|---|---|
| `GAME-SOURCES.md` complete | pass/fail/not applicable | `<path/notes>` |
| `GAME-RULES.md` complete with stable rule IDs | pass/fail | `<path/notes>` |
| `GAME-RULE-COVERAGE.md` complete | pass/fail | `<path/notes>` |
| `GAME-MECHANICS.md` complete | pass/fail | `<path/notes>` |
| `GAME-IMPLEMENTATION-ADMISSION.md` complete | pass/fail | `<path/notes>` |
| `COMPETENT-PLAYER.md` complete if strategy matters | pass/fail/not applicable | `<path/notes>` |
| `BOT-STRATEGY-EVIDENCE-PACK.md` complete if Level 2 ships | pass/fail/not applicable | `<path/notes>` |
| `GAME-AI.md` complete | pass/fail | `<path/notes>` |
| `GAME-UI.md` complete | pass/fail | `<path/notes>` |
| `GAME-BENCHMARKS.md` complete | pass/fail | `<path/notes>` |
| primitive-pressure ledger complete if needed | pass/fail/not applicable | `<path/notes>` |
| supported player-count smoke complete | pass/fail/not applicable | `<seat counts and evidence>` |
| all seat labels/roles are safe and stable | pass/fail/not applicable | `<evidence>` |
| pairwise no-leak matrix complete | pass/fail/not applicable | `<path/notes>` |
| per-seat outcome explanation complete | pass/fail/not applicable | `<path/notes>` |

## Rule, source, and IP status

| Check | Status | Evidence/notes |
|---|---|---|
| public rules prose is original Rulepath prose | pass/fail | `<notes>` |
| sources are recorded with dates and quality | pass/fail | `<notes>` |
| variant and deviations are clear | pass/fail | `<notes>` |
| no copied rulebook prose | pass/fail | `<notes>` |
| no copied card/component text | pass/fail/not applicable | `<notes>` |
| no copied icons, board art, screenshots, scans, or trade dress | pass/fail | `<notes>` |
| public name/trademark/trade-dress risk reviewed | pass/fail | `<notes>` |
| generated assets reviewed | pass/fail/not applicable | `<notes>` |
| fonts are system-only or license-reviewed | pass/fail | `<notes>` |
| human/legal review triggers resolved | pass/fail/not applicable | `<notes>` |

## Original prose, assets, and font status

| Content group | Status | Public artifact path | Reviewer/notes |
|---|---|---|---|
| rules/help prose | original / blocked | `<path>` | `<notes>` |
| UI copy | original / blocked | `<path>` | `<notes>` |
| component/card text | original / public-domain verified / license-reviewed / none / blocked | `<path>` | `<notes>` |
| icons/SVG/assets | original / license-reviewed / generated-reviewed / blocked | `<path>` | `<notes>` |
| screenshots/scans | none / blocked / license-reviewed | `<path>` | `<notes>` |
| fonts | system only / license-reviewed / blocked | `<path>` | `<notes>` |

## Private licensed content exclusion

| Check | Status | Evidence/notes |
|---|---|---|
| no private licensed rules/content in public files | pass/fail | `<notes>` |
| no private game names in public source/build | pass/fail | `<notes>` |
| no private assets in bundle | pass/fail | `<notes>` |
| no private content in traces/fixtures/replay exports | pass/fail | `<notes>` |
| private stress-test work did not shape `engine-core` | pass/fail/not applicable | `<notes>` |

## Bundle/public artifact inspection

| Surface/artifact | Inspected? | Unsafe content found? | Notes/action |
|---|---:|---:|---|
| production JS/WASM bundle | yes/no | yes/no | `<notes>` |
| static assets | yes/no | yes/no | `<notes>` |
| source maps if shipped | yes/no/not shipped | yes/no | `<notes>` |
| public routes | yes/no | yes/no | `<notes>` |
| local storage defaults | yes/no | yes/no | `<notes>` |
| replay examples/export samples | yes/no | yes/no | `<notes>` |
| console logs/diagnostics | yes/no | yes/no | `<notes>` |
| test IDs/DOM attributes | yes/no | yes/no | `<notes>` |
| dev inspector disabled/redacted in public build | yes/no | yes/no | `<notes>` |

## Hidden-information no-leak surfaces

Fill every row. Perfect-information games may mark `not applicable` with rationale.

| Surface | Status | Evidence/test | Notes |
|---|---|---|---|
| public/browser payload | pass/fail/not applicable | `<test>` | `<notes>` |
| public view | pass/fail/not applicable | `<test>` | `<notes>` |
| action tree | pass/fail/not applicable | `<test>` | `<notes>` |
| previews | pass/fail/not applicable | `<test>` | `<notes>` |
| diagnostics/disabled reasons | pass/fail/not applicable | `<test>` | `<notes>` |
| effect logs | pass/fail/not applicable | `<test>` | `<notes>` |
| command logs | pass/fail/not applicable | `<test>` | `<notes>` |
| DOM attributes | pass/fail/not applicable | `<test>` | `<notes>` |
| test IDs | pass/fail/not applicable | `<test>` | `<notes>` |
| browser console/logs | pass/fail/not applicable | `<test>` | `<notes>` |
| local storage/session storage | pass/fail/not applicable | `<test>` | `<notes>` |
| replay export/import | pass/fail/not applicable | `<test>` | `<notes>` |
| bot explanations | pass/fail/not applicable | `<test>` | `<notes>` |
| candidate rankings | pass/fail/not applicable | `<test>` | `<notes>` |
| dev inspector/public build boundary | pass/fail/not applicable | `<test>` | `<notes>` |

## Pairwise no-leak and per-seat outcome gate

Required for hidden-information, asymmetric-view, team, partnership, or 3+ seat games. Perfect-information two-seat games may use explicit `not applicable` rows.

| Check | Status | Evidence/test | Notes |
|---|---|---|---|
| pairwise no-leak matrix covers source seat private datum by viewer by surface | pass/fail/not applicable | `<GAME-RULE-COVERAGE.md path/test>` | `<notes>` |
| per-seat outcome explanation complete for every terminal result | pass/fail/not applicable | `<GAME-UI.md path/test>` | `<notes>` |
| per-team/partnership/coalition outcome explanation complete | pass/fail/not applicable | `<path/test>` | `<notes>` |
| no-reveal terminal outcomes verified | pass/fail/not applicable | `<test>` | `<notes>` |

## Replay/export safety

| Check | Status | Evidence/notes |
|---|---|---|
| replay reproduces deterministic hashes | pass/fail | `<notes>` |
| exported replay contains only intended public or authorized data | pass/fail | `<notes>` |
| hidden-info redaction verified for exports | pass/fail/not applicable | `<notes>` |
| multi-seat replay export/import verified for every supported player count | pass/fail/not applicable | `<notes>` |
| replay import validates versions and schema | pass/fail | `<notes>` |
| replay UI is viewer-safe | pass/fail | `<notes>` |
| golden traces are not silently updated | pass/fail | `<notes>` |

## UI polish and visual target

| Check | Status | Evidence/notes |
|---|---|---|
| default public screen is play-first, not debug-first | pass/fail | `<notes>` |
| visual target is polished and neutral | pass/fail | `<notes>` |
| no proprietary mimicry/trade dress | pass/fail | `<notes>` |
| legal moves are obvious | pass/fail | `<notes>` |
| player feedback is clear after every action | pass/fail | `<notes>` |
| semantic effects drive animations | pass/fail | `<notes>` |
| animations settle to Rust public view | pass/fail | `<notes>` |
| help/onboarding adequate for target | pass/fail | `<notes>` |
| React + SVG default preserved or justified | pass/fail | `<notes>` |

## Legal-only UI

| Check | Status | Evidence/notes |
|---|---|---|
| TypeScript does not decide legality | pass/fail | `<notes>` |
| UI controls derive from Rust action tree | pass/fail | `<notes>` |
| compound actions use Rust next choices at every stage | pass/fail/not applicable | `<notes>` |
| Rust previews are viewer-safe | pass/fail/not applicable | `<notes>` |
| stale/invalid submissions return safe diagnostics | pass/fail | `<notes>` |
| no raw command editing in public mode | pass/fail | `<notes>` |

## Accessibility, reduced-motion, and responsive checks

| Check | Status | Evidence/notes |
|---|---|---|
| keyboard path for core actions | pass/fail | `<notes>` |
| visible focus indicators | pass/fail | `<notes>` |
| accessible names/labels for controls | pass/fail | `<notes>` |
| screen-reader summaries where practical | pass/fail/not applicable | `<notes>` |
| contrast reviewed | pass/fail | `<notes>` |
| color is not sole information channel | pass/fail | `<notes>` |
| reduced-motion behavior implemented | pass/fail | `<notes>` |
| responsive layout smoke-tested | pass/fail | `<notes>` |
| small-screen seat-rail accessibility smoke-tested | pass/fail/not applicable | `<notes>` |
| accessibility scan where practical | pass/fail/not applicable | `<notes>` |

## Bot explanation safety

| Check | Status | Evidence/notes |
|---|---|---|
| public bot uses legal action API | pass/fail/not applicable | `<notes>` |
| bot does not access forbidden hidden information | pass/fail/not applicable | `<notes>` |
| Level 2 evidence pack complete if Level 2 ships | pass/fail/not applicable | `<notes>` |
| no public v1/v2 MCTS/ISMCTS/Monte Carlo/ML/RL | pass/fail | `<notes>` |
| public explanations are viewer-safe | pass/fail/not applicable | `<notes>` |
| candidate rankings are dev-only and redacted | pass/fail/not applicable | `<notes>` |
| bot latency acceptable | pass/fail/not applicable | `<notes>` |
| public default bot suitability recorded | pass/fail/not applicable | `<notes>` |

## Dev inspector/public build boundary

| Check | Status | Evidence/notes |
|---|---|---|
| public build hides or disables unsafe dev inspector | pass/fail | `<notes>` |
| dev inspector receives viewer-safe payloads only unless local test harness | pass/fail | `<notes>` |
| full internal state never ships to unauthorized browser | pass/fail | `<notes>` |
| debug candidate rankings redacted | pass/fail/not applicable | `<notes>` |
| console/log output safe | pass/fail | `<notes>` |

## Tests, traces, simulations, replay, serialization, and benchmarks

| Evidence | Status | Notes |
|---|---|---|
| unit tests | pass/fail | `<notes>` |
| named rule tests | pass/fail | `<notes>` |
| golden traces | pass/fail | `<notes>` |
| invalid/stale diagnostic traces | pass/fail | `<notes>` |
| property/invariant tests | pass/fail/not applicable | `<notes>` |
| simulation/fuzz runs | pass/fail | `<notes>` |
| replay/hash tests | pass/fail | `<notes>` |
| serialization/deserialization tests | pass/fail | `<notes>` |
| visibility/no-leak tests | pass/fail/not applicable | `<notes>` |
| bot legality/determinism/explanation tests | pass/fail/not applicable | `<notes>` |
| UI smoke tests | pass/fail | `<notes>` |
| accessibility/reduced-motion/responsive smoke | pass/fail | `<notes>` |
| native benchmarks | pass/fail | `<notes>` |
| WASM/browser smoke benchmarks | pass/fail/not applicable | `<notes>` |
| large-surface performance benchmarks | pass/fail/not applicable | `<seat count / max-surface fixture / notes>` |

## Public release decision

Decision: release / release with explicit constraints / blocked

Decision rationale:

- `<rationale>`

Release constraints, if any:

- `<constraint>`

## Blocking issues

| Issue | Required fix | Owner | Blocks release? |
|---|---|---|---:|
| `<issue>` | `<fix>` | `<owner>` | yes/no |

## Human review notes

| Reviewer | Area | Notes | Date |
|---|---|---|---|
| `<name>` | IP / UI / accessibility / rules / bot / release | `<notes>` | YYYY-MM-DD |

## Final checklist

- Official-game contract is satisfied.
- Rule/source/IP evidence is complete.
- No private licensed content ships publicly.
- Production bundle/artifact inspection is clean.
- Hidden-information no-leak surfaces are verified.
- Replay/export safety is verified.
- UI is polished, legal-only, accessible, reduced-motion-aware, and responsive.
- Bot explanations are safe.
- Dev inspector/public build boundary is safe.
- Tests, traces, simulations, replay, serialization, and benchmarks are green or explicitly accepted.
- Blocking issues are resolved or release is blocked.

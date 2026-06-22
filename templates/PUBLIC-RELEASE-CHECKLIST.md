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

Template realignment mapping: report `B-16 -> PUBLIC-RELEASE-CHECKLIST.md`

Evidence receipt: [`GAME-EVIDENCE.md`](GAME-EVIDENCE.md)

## Public Shipment Rule

If content ships to an unauthorized browser, it has shipped.

Public release includes any linked route, hosted artifact, downloadable build,
exported replay containing unsafe content, public source path, public package,
screenshot, demo video, or browser payload that exposes content to users who
are not authorized to see it.

## Release Evidence Snapshot

Missing or failing linked evidence is an automatic release blocker.

| Evidence surface | Status | Evidence ID/link | Notes |
|---|---|---|---|
| completion profile and foundation stop-condition review | pass/fail/blocker | `<GAME-EVIDENCE.md Completion Profile>` | `<notes>` |
| source/IP receipt | pass/fail/blocker | `<GAME-EVIDENCE.md Source and IP Receipt>` | `<notes>` |
| rule coverage and named tests | pass/fail/blocker | `<GAME-EVIDENCE.md Rule-Coverage Summary>` | `<notes>` |
| trace/replay/hash/export compatibility | pass/fail/blocker | `<GAME-EVIDENCE.md Named Trace Profiles / Replay and Hash Compatibility>` | `<notes>` |
| hidden-information no-leak surfaces | pass/fail/not applicable: `<rationale>` | `<GAME-EVIDENCE.md Hidden-Information No-Leak Matrix>` | `<notes>` |
| pairwise no-leak and per-seat outcome evidence | pass/fail/not applicable: `<rationale>` | `<GAME-EVIDENCE.md Viewer Matrix>` | `<notes>` |
| UI evidence | pass/fail/not applicable: `<rationale>` | `<GAME-UI.md / GAME-EVIDENCE.md Release State>` | `<notes>` |
| bot evidence | pass/fail/not applicable: `<rationale>` | `<GAME-AI.md / GAME-EVIDENCE.md Benchmarks and Bot Policy>` | `<notes>` |
| benchmark evidence | pass/fail/not applicable: `<rationale>` | `<GAME-BENCHMARKS.md / GAME-EVIDENCE.md Benchmarks and Bot Policy>` | `<notes>` |
| public-build artifact inspection | pass/fail/not applicable: `<rationale>` | `<build log/report link>` | `<notes>` |

## Non-Delegable Human Checks

These checks remain in the release checklist because they require release-time
judgment over the linked evidence and final artifact.

| Check | Status | Reviewer/evidence | Notes |
|---|---|---|---|
| public experience is play-first and not debug-first | pass/fail | `<reviewer/build link>` | `<notes>` |
| IP, trademark, trade-dress, asset, and font risks are acceptable | pass/fail/not applicable: `<rationale>` | `<reviewer/source evidence link>` | `<notes>` |
| hidden-info surface spot check matches the linked no-leak evidence | pass/fail/not applicable: `<rationale>` | `<reviewer/test/build link>` | `<notes>` |
| accessibility, reduced-motion, and responsive experience are acceptable | pass/fail/not applicable: `<rationale>` | `<reviewer/smoke link>` | `<notes>` |
| public build hides or disables unsafe dev inspector and debug rankings | pass/fail/not applicable: `<rationale>` | `<reviewer/build link>` | `<notes>` |

## Build and Version Receipt

| Item | Value |
|---|---|
| release artifact or URL | `<url_or_artifact>` |
| commit/hash | `<commit>` |
| rules version | `<rules_version>` |
| data/manifest version | `<data_or_manifest_version>` |
| engine version | `<engine_version>` |
| smoke commands | `<commands/reports>` |

## Public Release Decision

Decision: release / release with explicit constraints / blocked

Decision rationale:

- `<rationale>`

Release constraints, if any:

- `<constraint>`

## Blocking Issues

| Issue | Required fix | Owner | Blocks release? |
|---|---|---|---:|
| `<issue>` | `<fix>` | `<owner>` | yes/no |

## Human Review Notes

| Reviewer | Area | Notes | Date |
|---|---|---|---|
| `<name>` | IP / UI / accessibility / rules / bot / release | `<notes>` | YYYY-MM-DD |

## Final Sign-off

- `GAME-EVIDENCE.md` is complete for the selected completion profile.
- Required domain evidence links resolve.
- Missing linked evidence is recorded as a blocker or accepted release constraint.
- Non-delegable human checks are complete.
- Public release decision is explicit.


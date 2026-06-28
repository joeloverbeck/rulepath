# <private_lane_id> Private Release Checklist

Private lane ID: `<opaque_private_lane_id>`

Pinned public Rulepath commit: `<commit>`

Private repository/build artifact: `<private_reference>`

Release target: private preview / private playtest / private tagged build / other: `<target>`

Prepared by: `<name/agent>`

Date: YYYY-MM-DD

Template realignment mapping: report `B-16 -> PRIVATE-RELEASE-CHECKLIST.md`

## Purpose

This checklist is for sanctioned private licensed games built outside the public
Rulepath repository. It may be copied into the private repository. The public
template intentionally uses opaque placeholders only; do not replace them with
private titles, ids, catalog strings, source references, fixture names, e2e
names, screenshots, card/event text, or licensed prose in public files.

## Authority Gates

| Gate | Status | Evidence |
|---|---|---|
| ADR 0010 sanctioned lane applies | pass/fail/blocker | `<evidence>` |
| ADR 0011 typed Rust event-card boundary applies if needed | pass/fail/not applicable | `<evidence>` |
| ADR 0012 private repository/catalog overlay applies | pass/fail/blocker | `<evidence>` |
| private repository pins a public Rulepath commit | pass/fail/blocker | `<evidence>` |
| public repository contains no private names, ids, catalog strings, source text, fixtures, traces, or artifacts | pass/fail/blocker | `<evidence>` |

## Private Build Separation

| Surface | Status | Evidence | Notes |
|---|---|---|---|
| private crates/docs/fixtures/traces/e2e live only in the private repository | pass/fail/blocker | `<evidence>` | `<notes>` |
| private WASM/web build is separate from public artifacts | pass/fail/blocker | `<evidence>` | `<notes>` |
| private catalog entries appear only in private builds | pass/fail/blocker | `<evidence>` | `<notes>` |
| private renderer mappings/assets appear only in private builds | pass/fail/blocker | `<evidence>` | `<notes>` |
| private CI does not publish private artifacts to public logs or public release surfaces | pass/fail/blocker | `<evidence>` | `<notes>` |

## Public Back-Leak Sweep

Run this sweep against the public checkout and public build artifacts. Keep the
actual private search terms in the private repository.

| Sweep | Status | Evidence | Notes |
|---|---|---|---|
| private title/id/catalog-string search over public files | pass/fail/blocker | `<private evidence>` | `<notes>` |
| private source-expression search over public files | pass/fail/blocker | `<private evidence>` | `<notes>` |
| private fixture/e2e/trace/artifact search over public files | pass/fail/blocker | `<private evidence>` | `<notes>` |
| public catalog docs and public `rulepath_list_games` remain public-only | pass/fail/blocker | `<evidence>` | `<notes>` |
| public JS/WASM/static build contains no private strings or assets | pass/fail/blocker | `<evidence>` | `<notes>` |

## Private Viewer Safety

| Surface | Status | Evidence | Notes |
|---|---|---|---|
| public observer projection hides private-only facts | pass/fail/not applicable | `<evidence>` | `<notes>` |
| every authorized seat/viewer receives only its own private payload | pass/fail/not applicable | `<evidence>` | `<notes>` |
| action trees/previews/diagnostics are viewer-safe | pass/fail/not applicable | `<evidence>` | `<notes>` |
| effects, replay exports, dev panels, logs, storage, DOM, accessibility labels, and test IDs are no-leak checked | pass/fail/not applicable | `<evidence>` | `<notes>` |
| bot explanations and candidate diagnostics do not copy private flowchart/source expression | pass/fail/not applicable | `<evidence>` | `<notes>` |

## Release Decision

Decision: private release / private release with constraints / blocked

Decision rationale:

- `<rationale>`

Constraints:

- `<constraint>`

## Final Sign-off

- Private release is authorized for the named private audience only.
- Public Rulepath remains private-free and catalog-public-only.
- Private build artifacts are not published through public release channels.
- Human/legal review questions are resolved or recorded as blockers.

---
name: specs-readme-status-set
description: Rulepath specs/README.md index schema and the only valid Status values
metadata:
  type: project
---

`specs/README.md` is the living spec-progress index. Its table schema is `| Stage | Gate | Spec | Status |`, and the ONLY valid Status values are `Not started → Planned → In progress → Done` (defined at specs/README.md:50). There is no `Proposed` status — externally-generated specs (ChatGPT-Pro deep-research output) tend to invent one; map it to `Planned` (spec written, not yet executing).

Non-gate specs (e.g. cross-game UI infrastructure like [[rules-display-shared-surface]]) don't fit the gate-keyed Stage/Gate columns — record them with `—` stage/gate or under a separate non-gate note, still using a valid Status value.

specs/README.md also documents the canonical spec FORMAT (section set: Header / Objective / Scope / Deliverables / Work breakdown / Exit criteria / Acceptance evidence / FOUNDATIONS alignment / Forbidden changes / Documentation updates / Sequencing / Assumptions); exemplar is specs/gate-0-repository-skeleton.md.

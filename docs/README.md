# Rulepath Foundation Documents

Status: replacement foundation set for the Rulepath repository.

Rulepath is a Rust-first, public playable, portfolio-quality web app for card and board games. It is also a disciplined route toward later private stress tests and long-term engine research. When those goals compete, the polished public playable product wins.

Read these documents in this order.

| Order | Document | Purpose |
|---:|---|---|
| 1 | [FOUNDATIONS.md](FOUNDATIONS.md) | Constitution: product priority, hard authority rules, universal invariants, stop conditions. |
| 2 | [ARCHITECTURE.md](ARCHITECTURE.md) | Workspace shape, dependency direction, Rust/WASM boundary, action/view/effect/replay/determinism model. |
| 3 | [ENGINE-GAME-DATA-BOUNDARY.md](ENGINE-GAME-DATA-BOUNDARY.md) | The exact boundary between `engine-core`, `game-stdlib`, `games/*`, static data, formats, and future DSL pressure. |
| 4 | [OFFICIAL-GAME-CONTRACT.md](OFFICIAL-GAME-CONTRACT.md) | What it means for a game to be official; requirements-first workflow from rules research through bots and UI. |
| 5 | [MECHANIC-ATLAS.md](MECHANIC-ATLAS.md) | Mechanic inventory, primitive-pressure ledger, second-use comparison, third-use hard gate. |
| 6 | [MECHANICAL-SCAFFOLDING-REGISTER.md](MECHANICAL-SCAFFOLDING-REGISTER.md) | Mechanical-scaffolding decision register governed by ADR 0008; subordinate to the behavioral mechanic atlas and foundation boundary law. |
| 7 | [AI-BOTS.md](AI-BOTS.md) | Bot law, levels, hidden-information safety, Level 2 strategy evidence workflow, explanations, personalities. |
| 8 | [UI-INTERACTION.md](UI-INTERACTION.md) | Public visual target, legal-only interaction, previews, effect-driven animation, replay UI, accessibility. |
| 9 | [TESTING-REPLAY-BENCHMARKING.md](TESTING-REPLAY-BENCHMARKING.md) | Test taxonomy, golden traces, deterministic replay/hash discipline, no-leak tests, benchmarks, CI. |
| 10 | [TRACE-SCHEMA-v1.md](TRACE-SCHEMA-v1.md) | Trace and replay-fixture schema law; subordinate to the constitution, architecture, data boundary, and accepted visibility/hash ADRs. |
| 11 | [EVIDENCE-FIXTURE-CONTRACT.md](EVIDENCE-FIXTURE-CONTRACT.md) | Evidence fixture profile contract governed by ADR 0009; separates command traces, setup/domain fixtures, and viewer-scoped exports. |
| 12 | [MULTI-SEAT-AND-SURFACE-CONTRACT.md](MULTI-SEAT-AND-SURFACE-CONTRACT.md) | N-seat and larger-surface obligations; subordinate to the constitution, architecture, data boundary, hidden-info ADRs, and bot law. |
| 13 | [ROADMAP.md](ROADMAP.md) | Prescriptive staged ladder and build gates. |
| 14 | [IP-POLICY.md](IP-POLICY.md) | Public/private content policy, naming, original prose/assets, generated asset and font review. |
| 15 | [AGENT-DISCIPLINE.md](AGENT-DISCIPLINE.md) | Coding-agent law: bounded tasks, forbidden changes, failing-test protocol, handoff expectations. |
| 16 | [SOURCES.md](SOURCES.md) | Researched bibliography and Rulepath-specific lessons. |
| 17 | [WASM-CLIENT-BOUNDARY.md](WASM-CLIENT-BOUNDARY.md) | Gate 3 Rust/WASM-to-browser client contract, operation groups, replay safety, and dev-panel data whitelist. |
| 18 | [adr/ADR-TEMPLATE.md](adr/ADR-TEMPLATE.md) | Foundation-level ADR template for architecture-changing decisions. |

## ADR status index

Proposed ADRs are informative only. They must not be cited as accepted law or
used to supersede foundation documents unless maintainers later mark them
`Accepted` and the named foundation updates land.

| ADR | Status | Subject |
|---|---|---|
| [0001](adr/0001-stage-1-random-playout-budget.md) | Accepted | Stage 1 random playout benchmark budget |
| [0002](adr/0002-ci-benchmark-gating-lanes.md) | Accepted | CI benchmark gating lanes |
| [0003](adr/0003-ci-calibrated-benchmark-thresholds.md) | Accepted | CI-calibrated benchmark thresholds |
| [0004](adr/0004-hidden-info-replay-export-taxonomy.md) | Accepted | Hidden-information replay/export visibility taxonomy |
| [0005](adr/0005-variance-aware-ci-benchmark-floors.md) | Accepted | Variance-aware CI benchmark floors |
| [0006](adr/0006-blackjack-lite-roadmap-placement.md) | Accepted | Blackjack Lite roadmap placement |
| [0007](adr/0007-next-public-scaling-phase-and-gate-p-tail.md) | Accepted | Next public scaling phase and Gate P tail placement |
| [0008](adr/0008-mechanical-scaffolding-governance.md) | Accepted | Mechanical scaffolding governance |
| [0009](adr/0009-replay-fixture-hash-taxonomy.md) | Accepted | Replay, fixture, export, and hash taxonomy v2 |
| [0010](adr/0010-sanctioned-parallel-private-game-lane.md) | Accepted | Sanctioned parallel private-game lane |
| [0011](adr/0011-constrained-typed-rust-event-card-mechanism.md) | Accepted | Constrained typed Rust event-card mechanism |
| [0012](adr/0012-private-repository-ci-catalog-overlay.md) | Accepted | Private repository, CI federation, and catalog overlay |

Private-lane doctrine remains subordinate to [FOUNDATIONS.md](FOUNDATIONS.md).
It may amend roadmap timing, IP policy, or public/private build doctrine only
through accepted ADRs and matching foundation/area-document updates.

## What belongs in this set

This set contains foundation-level repository law only. It intentionally does **not** include per-game templates, per-task templates, implementation tickets, code, asset files, proprietary rules text, screenshots, fonts, or private-game material.

## Decision hierarchy

1. Accepted ADRs may supersede foundation documents only when the ADR explicitly names the affected sections and updates the documents.
2. [FOUNDATIONS.md](FOUNDATIONS.md) is the constitution.
3. Area documents define operational law for their domain.
4. Per-game documents must conform to this set.
5. Agent instructions and temporary task notes do not override foundation law.

## Implementation specs

Implementation planning lives **outside** this foundation set, in
[`../specs/`](../specs/). Each roadmap gate gets one spec, and
[`../specs/README.md`](../specs/README.md) is the living index and progress
tracker. Specs conform to this foundation set and never override it; this set
remains foundation-level repository law only.

## Default posture

When in doubt:

- keep `engine-core` generic and noun-free;
- keep rules in typed Rust game modules;
- keep TypeScript presentation-only;
- keep public games polished and fully supported;
- keep private licensed experiments late, isolated, optional, and non-architectural;
- stop and reassess rather than generalize casually.

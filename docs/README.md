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
| 6 | [AI-BOTS.md](AI-BOTS.md) | Bot law, levels, hidden-information safety, Level 2 strategy evidence workflow, explanations, personalities. |
| 7 | [UI-INTERACTION.md](UI-INTERACTION.md) | Public visual target, legal-only interaction, previews, effect-driven animation, replay UI, accessibility. |
| 8 | [TESTING-REPLAY-BENCHMARKING.md](TESTING-REPLAY-BENCHMARKING.md) | Test taxonomy, golden traces, deterministic replay/hash discipline, no-leak tests, benchmarks, CI. |
| 9 | [ROADMAP.md](ROADMAP.md) | Prescriptive staged ladder and build gates. |
| 10 | [IP-POLICY.md](IP-POLICY.md) | Public/private content policy, naming, original prose/assets, generated asset and font review. |
| 11 | [AGENT-DISCIPLINE.md](AGENT-DISCIPLINE.md) | Coding-agent law: bounded tasks, forbidden changes, failing-test protocol, handoff expectations. |
| 12 | [SOURCES.md](SOURCES.md) | Researched bibliography and Rulepath-specific lessons. |
| 13 | [WASM-CLIENT-BOUNDARY.md](WASM-CLIENT-BOUNDARY.md) | Gate 3 Rust/WASM-to-browser client contract, operation groups, replay safety, and dev-panel data whitelist. |
| 14 | [adr/ADR-TEMPLATE.md](adr/ADR-TEMPLATE.md) | Foundation-level ADR template for architecture-changing decisions. |

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

# Rulepath Documentation

This is the index for the Rulepath foundation document set. Start with the constitution, then read the area docs relevant to your work. Shared protocols and checklist items live once in `INVARIANTS.md`; other docs link to it rather than restate it.

## Constitution

- [FOUNDATIONS.md](FOUNDATIONS.md) — repository constitution. The priority order and §1–§12 principles, including the §12 stop conditions. Supersede only by accepted ADR.

## Shared invariants

- [INVARIANTS.md](INVARIANTS.md) — single source of truth for the failing-test protocol, the kernel-change protocol, and the universal acceptance invariants referenced throughout the set.

## Architecture & boundaries

- [ARCHITECTURE.md](ARCHITECTURE.md) — architectural law: repository shape, dependency direction, ownership table, `engine-core` contract, WASM API, view/effect/replay/determinism models.
- [DATA-RUST-BOUNDARY.md](DATA-RUST-BOUNDARY.md) — what static data may and may not contain; typed-content pipeline, format table, behavior-looking-field detection.

## Authoring, testing, bots, UI

- [AUTHORING-MODEL.md](AUTHORING-MODEL.md) — authoring law for game modules, per-game docs, static content, source notes, rule coverage, and naming.
- [TESTING-AND-BENCHMARKING.md](TESTING-AND-BENCHMARKING.md) — correctness and performance law: test categories, golden traces, no-leak tests, benchmark doctrine and budgets.
- [AI-BOTS.md](AI-BOTS.md) — bot law: levels, fairness, candidate model, policy nodes, lexicographic priorities, explanations, no-leak requirements.
- [UI-INTERACTION.md](UI-INTERACTION.md) — public web app and interaction law: visual direction, action lifecycle, previews, effect-log animation, dev inspector boundary, accessibility.

## Process & roadmap

- [ROADMAP.md](ROADMAP.md) — the staged mechanic ladder and build-gate order (merged), with a stage↔gate crosswalk.
- [MECHANIC-ATLAS.md](MECHANIC-ATLAS.md) — primitive-pressure law: when a repeated mechanic may be promoted to `game-stdlib`, and the third-use hard gate.
- [AGENT-DISCIPLINE.md](AGENT-DISCIPLINE.md) — operational law for coding agents: roles, task structure, and per-area protocols.

## Policy & reference

- [IP-POLICY.md](IP-POLICY.md) — public repository and website IP law.
- [SOURCES.md](SOURCES.md) — supporting bibliography for the foundation set.

## Decisions

- [adr/](adr/) — Architecture Decision Records. Use [adr/ADR-TEMPLATE.md](adr/ADR-TEMPLATE.md) for new decisions.

## Templates

Per-game and per-task document templates live in [`../templates/`](../templates/).

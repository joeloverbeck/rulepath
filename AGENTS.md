# AGENTS.md

Rulepath is a Rust-first, rule-enforcing, replayable, testable card/board-game
platform. Rust owns behavior; TypeScript/React presents Rust/WASM output only.
This is Codex's every-query orientation card; keep detailed law in the docs.

## Read First

- [docs/FOUNDATIONS.md](docs/FOUNDATIONS.md) is the constitution: authority,
  invariants, stop conditions, and ADR triggers.
- [docs/README.md](docs/README.md) is the ordered foundation-doc index.
- [docs/AGENT-DISCIPLINE.md](docs/AGENT-DISCIPLINE.md) is coding-agent law:
  bounded tasks, forbidden changes, failing-test protocol, and handoff.
- [specs/README.md](specs/README.md) and [tickets/README.md](tickets/README.md)
  track active gate specs and bounded tickets.

## Non-Negotiables

These hold on every change; the full law lives in `docs/FOUNDATIONS.md`.

- `engine-core` stays generic and noun-free. Typed mechanic nouns belong in
  `games/*` first.
- TypeScript never decides legality. Legal actions, validation, effects, views,
  and bot decisions come from Rust/WASM.
- Do not add YAML, a DSL, or procedural static data without an accepted ADR.
- Replay, hashes, RNG, serialization order, and traces must stay deterministic
  unless explicitly migrated.
- Do not leak hidden information into payloads, DOM, storage, logs, bot
  explanations, replay exports, traces, or tests.
- Do not add MCTS, ISMCTS, Monte Carlo, ML, or RL bots for public v1/v2.
- Never delete or weaken tests to get green output.
- When uncertain, keep the boundary, keep determinism, and reassess before
  generalizing.

## Verification Commands

Use the narrowest truthful verification for the change.

Rust hygiene:

```bash
cargo fmt --all --check
cargo clippy --workspace --all-targets -- -D warnings
cargo build --workspace
cargo test --workspace
```

Per-game verification (replace the game id with the game under change; check
`specs/README.md` for the active gate):

```bash
cargo run -p simulate -- --game race_to_n --games 1000
cargo run -p replay-check -- --game race_to_n --all
cargo run -p fixture-check -- --game race_to_n
cargo run -p rule-coverage -- --game race_to_n
bash scripts/boundary-check.sh
node scripts/check-doc-links.mjs
```

Web, when Rust/WASM or UI changes:

```bash
npm --prefix apps/web ci
npm --prefix apps/web run smoke:wasm
npm --prefix apps/web run build
npm --prefix apps/web run smoke:ui
```

Benchmarks, when performance or benchmark gates change:

```bash
cargo bench -p <game-crate>
```

## Workspace Map

- `crates/engine-core`: generic contract kernel, noun-free.
- `crates/game-stdlib`: earned shared helpers via the mechanic atlas.
- `crates/ai-core`: bot infrastructure.
- `crates/wasm-api`: Rust-to-browser JSON bridge.
- `games/*`: typed Rust game modules.
- `tools/*`: simulation, replay, fixture, coverage, benchmark, reducer, viewer.
- `apps/web`: TypeScript/React presentation shell.

## Workflow

Follow the repo sequence: roadmap in [docs/ROADMAP.md](docs/ROADMAP.md), gate
specs in `specs/`, bounded tickets in `tickets/`, task/game templates in
`templates/`, and architecture decisions in `docs/adr/`.

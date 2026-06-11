# CLAUDE.md

Rulepath — a Rust-first, rule-enforcing, replayable, testable card/board-game
platform. **Rust owns all behavior; TypeScript/React present only.**

This file is the every-query orientation card. It does not restate the law —
read the docs below before any product-behavior change.

## Read first

- **[docs/FOUNDATIONS.md](docs/FOUNDATIONS.md)** — the constitution: priority
  order, behavior authority, §11 universal invariants, §12 stop conditions,
  §13 ADR triggers. Authoritative; supersede only by an accepted ADR.
- **[docs/README.md](docs/README.md)** — ordered index of the foundation
  doc set (architecture, boundaries, official-game contract, bots, UI,
  testing, roadmap, IP, agent discipline).
- **[docs/AGENT-DISCIPLINE.md](docs/AGENT-DISCIPLINE.md)** — operational law for
  coding agents: bounded tasks, forbidden changes, failing-test protocol.

## Non-negotiables (the full law lives in FOUNDATIONS.md)

These are the highest-cost mistakes; they hold on every change:

- `engine-core` stays generic and **noun-free** — no `board`, `card`, `deck`,
  `grid`, `hand`, etc. Typed mechanic nouns belong in `games/*` first.
- **TypeScript never decides legality.** Legal actions, validation, effects,
  views, and bot decisions all come from Rust/WASM.
- **No YAML and no DSL without an accepted ADR.** Static data is typed
  content/parameters/metadata only — never selectors, conditions, or triggers.
- **Replay, hashes, RNG, serialization order, and traces stay deterministic**
  (or are explicitly migrated).
- **No hidden-information leaks** into payloads, DOM, storage, logs, effect
  logs, bot explanations, or replay exports.
- **No MCTS / ISMCTS / Monte Carlo / ML / RL bots** in public v1/v2.
- **Never delete or weaken tests to get green.** Follow the failing-test
  protocol (AGENT-DISCIPLINE §4).
- **Deliver complete files or coherent complete sections, not diffs.**

When in doubt: keep the boundary, keep it deterministic, and **stop and
reassess rather than generalize** (FOUNDATIONS §12).

## Commands

Rust hygiene (CI gate 0):

```bash
cargo fmt --all --check
cargo clippy --workspace --all-targets -- -D warnings
cargo build --workspace
cargo test --workspace
```

Per-game verification (CI gate 1; replace the game id with the game under
change and check `specs/README.md` for the active gate):

```bash
cargo run -p simulate      -- --game race_to_n --games 1000
cargo run -p replay-check  -- --game race_to_n --all
cargo run -p fixture-check -- --game race_to_n
cargo run -p rule-coverage -- --game race_to_n
bash scripts/boundary-check.sh        # engine-core stays noun-free
node scripts/check-doc-links.mjs      # doc link integrity
node scripts/check-catalog-docs.mjs   # web-shell catalog docs name every game
```

Web (CI gate 1; needs `rustup target add wasm32-unknown-unknown`):

```bash
npm --prefix apps/web ci
npm --prefix apps/web run smoke:wasm
npm --prefix apps/web run build
npm --prefix apps/web run smoke:ui
npm --prefix apps/web run smoke:effects
```

Benchmarks (CI gate 2):

```bash
cargo bench -p <game-crate>           # full; use the crate's bench filters for a smoke run
```

## Workspace map

- `crates/engine-core` — generic contract kernel (noun-free).
- `crates/game-stdlib` — earned shared helpers (only via the mechanic atlas).
- `crates/ai-core` — bot infrastructure.
- `crates/wasm-api` — Rust↔browser JSON bridge.
- `games/*` — typed Rust game modules (rules, traces, bots, docs).
- `tools/*` — `simulate`, `replay-check`, `fixture-check`, `rule-coverage`,
  `bench-report`, `seed-reducer`, `trace-viewer`.
- `apps/web` — TypeScript/React presentation shell.

## Workflow

Roadmap (`docs/ROADMAP.md`) → spec per gate (`specs/`, tracked in
**[specs/README.md](specs/README.md)** — the living progress index) →
bounded tickets (`tickets/`, from `tickets/_TEMPLATE.md`) → per-game/per-task
templates (`templates/`). Architecture decisions: `docs/adr/`.

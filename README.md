# Rulepath

> A Rust-first, rule-enforcing, replayable, testable card/board-game platform.
> **Rust owns all behavior; TypeScript/React only presents it.**

[![License: GPL v3](https://img.shields.io/badge/License-GPLv3-blue.svg)](LICENSE)

Rulepath builds polished, public, browser-playable card and board games on a
deterministic Rust engine. Every legal action, validation, effect, view, bot
decision, and replay comes from Rust compiled to WebAssembly — the web shell
renders what Rust says and never decides game legality itself. The result is a
platform where games are replayable from a deterministic command log, testable
end to end, and safe against hidden-information leaks by construction.

## Status

**Gates 0-9 complete; successor commitment/reveal gate next** — Rulepath now ships seven local-playable
official games: **Race to 21** (`race_to_n`), **Three Marks** (`three_marks`),
**Column Four** (`column_four`), **Directional Flip** (`directional_flip`),
**Draughts Lite** (`draughts_lite`), **High Card Duel**
(`high_card_duel`), and **Token Bazaar** (`token_bazaar`). Gate 9 is complete
with Token Bazaar as the accepted public resource/economy proof. `blackjack_lite`
is deferred by [ADR 0006](docs/adr/0006-blackjack-lite-roadmap-placement.md) and
does not block later gates; the next roadmap target is the successor
commitment/reveal proof (`secret_draft`, to be specced separately). See
[`specs/README.md`](specs/README.md) for the live gate-by-gate progress tracker
and [`docs/ROADMAP.md`](docs/ROADMAP.md) for the full staged ladder.

## Play it locally

**Prerequisites**

- A stable [Rust toolchain](https://rustup.rs/) with the WebAssembly target:
  ```bash
  rustup target add wasm32-unknown-unknown
  ```
- Node.js 20.19+ (or 22.12+) and npm.

**Run the web shell**

```bash
npm --prefix apps/web install
npm --prefix apps/web run build
npm --prefix apps/web run preview
```

Then open **http://127.0.0.1:4173** and play. There is no backend, account, or
database — match state lives entirely in the in-memory Rust/WASM store.

> `build` first compiles `crates/wasm-api` to `wasm32-unknown-unknown`, copies
> the artifact into `apps/web/public/`, typechecks, and emits the static Vite
> bundle. `preview` serves that bundle. (There is no dev-server script — the app
> always runs against a real compiled WASM artifact.)

More detail on the shell, its smoke layers, and static serving lives in
[`apps/web/README.md`](apps/web/README.md).

## How it works

- **Rust owns behavior.** `engine-core` is a generic, noun-free contract kernel;
  typed game rules live in `games/*`; bots live in `ai-core`.
- **WASM is the boundary.** `crates/wasm-api` is a JSON bridge that hands the
  browser legal actions, views, effects, diagnostics, bot turns, and replay
  projections.
- **TypeScript/React only presents.** The shell renders Rust output and submits
  actions; it never validates moves or computes legality.
- **Determinism is law.** Replays, hashes, RNG, serialization order, and traces
  are deterministic unless explicitly migrated.

The authoritative rules are in [`docs/FOUNDATIONS.md`](docs/FOUNDATIONS.md) (the
constitution) and the ordered foundation set indexed by
[`docs/README.md`](docs/README.md).

## Develop & verify

**Rust hygiene**

```bash
cargo fmt --all --check
cargo clippy --workspace --all-targets -- -D warnings
cargo build --workspace
cargo test --workspace
```

**Per-game checks** (replace the game id as needed; current official games are
`race_to_n`, `three_marks`, `column_four`, `directional_flip`,
`draughts_lite`, `high_card_duel`, and `token_bazaar`)

```bash
cargo run -p simulate      -- --game race_to_n --games 1000
cargo run -p replay-check  -- --game race_to_n --all
cargo run -p fixture-check -- --game race_to_n
cargo run -p rule-coverage -- --game race_to_n
bash scripts/boundary-check.sh        # engine-core stays noun-free
node scripts/check-doc-links.mjs      # doc link integrity
```

**Web checks**

```bash
npm --prefix apps/web run smoke:wasm     # raw WASM ABI coverage
npm --prefix apps/web run smoke:ui       # Node/WASM shell-state smoke
npm --prefix apps/web run smoke:e2e      # built-bundle browser + no-leak smoke
```

The Puppeteer E2E smoke uses system Chrome at `/usr/bin/google-chrome`; set
`PUPPETEER_EXECUTABLE_PATH` to override.

## Workspace map

| Path | What it is |
|---|---|
| `crates/engine-core` | Generic contract kernel (noun-free). |
| `crates/game-stdlib` | Earned shared helpers (via the mechanic atlas). |
| `crates/ai-core` | Bot infrastructure. |
| `crates/wasm-api` | Rust ↔ browser JSON bridge. |
| `games/*` | Typed Rust game modules (rules, traces, bots, docs). |
| `tools/*` | `simulate`, `replay-check`, `fixture-check`, `rule-coverage`, and more. |
| `apps/web` | TypeScript/React presentation shell. |

## Documentation

| Document | Purpose |
|---|---|
| [`docs/README.md`](docs/README.md) | Ordered index of the foundation doc set. |
| [`docs/FOUNDATIONS.md`](docs/FOUNDATIONS.md) | The constitution: priority, invariants, stop conditions. |
| [`docs/ARCHITECTURE.md`](docs/ARCHITECTURE.md) | Workspace shape and the Rust/WASM boundary. |
| [`docs/ROADMAP.md`](docs/ROADMAP.md) | Prescriptive staged ladder and build gates. |
| [`specs/README.md`](specs/README.md) | Living per-gate progress tracker. |
| [`apps/web/README.md`](apps/web/README.md) | Web shell commands, smoke layers, static serving. |
| [`CLAUDE.md`](CLAUDE.md) / [`AGENTS.md`](AGENTS.md) | Coding-agent orientation and workflow. |

## License

[GNU General Public License v3.0](LICENSE).

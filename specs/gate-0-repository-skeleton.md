# Spec: Gate 0 — Repository Skeleton

- Spec ID: `gate-0-repository-skeleton`
- Roadmap stage: 0
- Roadmap build gate: Gate 0
- Status: Planned
- Date: 2026-06-05
- Owner: joeloverbeck

This spec is an implementation plan. It is subordinate to the foundation set in
[`../docs/README.md`](../docs/README.md) and MUST NOT redefine any foundation
contract. Where this spec and a foundation document disagree, the foundation
document wins. Authority order for this spec:
[`../docs/FOUNDATIONS.md`](../docs/FOUNDATIONS.md),
[`../docs/ARCHITECTURE.md`](../docs/ARCHITECTURE.md),
[`../docs/ENGINE-GAME-DATA-BOUNDARY.md`](../docs/ENGINE-GAME-DATA-BOUNDARY.md),
[`../docs/TESTING-REPLAY-BENCHMARKING.md`](../docs/TESTING-REPLAY-BENCHMARKING.md),
[`../docs/ROADMAP.md`](../docs/ROADMAP.md),
[`../docs/AGENT-DISCIPLINE.md`](../docs/AGENT-DISCIPLINE.md).

## 1. Objective

Stand up the empty Rulepath workspace skeleton so that later gates have a place
to land. This gate produces **no gameplay**. It proves the workspace compiles,
the web shell builds, a placeholder WASM artifact loads in the browser shell,
and `engine-core` exists as a contract-only crate with no mechanic vocabulary.

Source of truth for this gate: ROADMAP.md §4 (Gate 0) and ARCHITECTURE.md §1–2.

## 2. Scope

### In scope

- A Rust workspace (`Cargo.toml` workspace manifest at repo root).
- Placeholder crates: `crates/engine-core`, `crates/game-stdlib`,
  `crates/ai-core`, `crates/wasm-api`.
- A React/TypeScript web shell under `apps/web/` that builds and loads the
  placeholder WASM artifact.
- Placeholder tool crates under `tools/`: `simulate`, `replay-check`,
  `trace-viewer`, `rule-coverage`, `bench-report`, `seed-reducer`,
  `fixture-check`.
- A `benches/` placeholder.
- An empty `games/` directory (a `.gitkeep` or a `README.md` placeholder; the
  first game crate arrives in Gate 1).
- A CI smoke pipeline (formatting, lint, build, workspace tests, WASM build
  smoke).
- The documentation updates listed in §9.

### Out of scope (deferred to later gates or forbidden here)

- Any real game, mechanic, rule, action, or effect (Gate 1+: `race_to_n`).
- Any populated `game-stdlib` helper (earned only via the mechanic atlas).
- Trace serialization, replay checker, stable-hash machinery, benchmark harness
  (Gate 2).
- Batched WASM gameplay API, game picker, view/action/effect stores, replay
  controls (Gate 3).
- Any networking, accounts, database, or hosted service.

### Not allowed (ROADMAP §4)

- Real mechanics in `engine-core`.
- YAML behavior.
- DSL work.
- Hosted services.
- Private-game names anywhere in the tree.

## 3. Deliverables

Target tree after this gate (concrete file names MAY vary; responsibilities MUST
NOT), per ARCHITECTURE.md §1:

```text
/
  Cargo.toml                # workspace manifest
  crates/
    engine-core/            # generic contracts only; noun-free
    game-stdlib/            # placeholder; no helpers yet
    ai-core/                # placeholder; bot-trait stub only
    wasm-api/               # placeholder; builds to a loadable WASM artifact
  games/                    # empty placeholder (first game in Gate 1)
  apps/
    web/                    # React/TS shell; builds; loads placeholder WASM
  tools/
    simulate/ replay-check/ trace-viewer/
    rule-coverage/ bench-report/ seed-reducer/ fixture-check/
  benches/                  # placeholder
  .github/workflows/        # CI smoke (or equivalent CI config)
  docs/  docs/adr/          # already present
  specs/                    # already present (this spec + index)
```

`engine-core` placeholder MAY declare only generic contract vocabulary from
ENGINE-GAME-DATA-BOUNDARY.md §3 (e.g. id/version/seed/viewer/actor/action-tree/
command-envelope/diagnostic/effect-envelope/visibility/replay/hash/serialization
contracts). It MUST NOT name any mechanic noun (`board`, `card`, `deck`, `grid`,
`suit`, `resource`, `capture`, etc.). At Gate 0 it is acceptable for these
contracts to be minimal or empty marker traits/types — emptiness is preferred
over speculative contract surface.

The four crates MUST wire dependency direction per ARCHITECTURE.md §2:
`engine-core` depends on no other Rulepath crate; `game-stdlib`, `ai-core`, and
`wasm-api` may depend on `engine-core` only. The dependency skeleton is set here
at Gate 0 — `engine-core` MUST NOT depend on `game-stdlib`, `ai-core`,
`wasm-api`, `games/*`, or `apps/web`. This is the dependency half of the kernel
boundary (FOUNDATIONS §3 / ARCHITECTURE.md §2); compilation alone does not prove
it, so it is an exit criterion below.

## 4. Work breakdown

Each item below is a candidate `templates/AGENT-TASK.md` packet. They are ordered
by dependency. WB1 must precede the rest; WB4 depends on WB1–WB3.

| ID | Work item | Depends on | Becomes AGENT-TASK |
|---|---|---|---|
| WB1 | Workspace manifest + four placeholder crates (`engine-core`, `game-stdlib`, `ai-core`, `wasm-api`) that compile; each has a trivial smoke test. | — | yes |
| WB2 | `wasm-api` builds to a WASM artifact; `apps/web` React/TS shell builds and loads it (a placeholder call that returns a version/string is sufficient). | WB1 | yes |
| WB3 | Seven `tools/*` placeholder crates that compile and run a no-op `--help`/version; `benches/` placeholder; empty `games/` placeholder. | WB1 | yes |
| WB4 | CI smoke pipeline: format check, lint, `cargo build`/`test` for the workspace, WASM build smoke, web shell build, and (where practical) an internal docs/anchor link check over `docs/` and `specs/` (TESTING §17). | WB1–WB3 | yes |
| WB5 | Documentation updates per §9 (this spec's index status flips to Done; ROADMAP/README pointers already added when this spec was authored). | WB1–WB4 | optional |

Implementation does not begin from this spec alone. Each WB item is decomposed
into an AGENT-TASK with bounded scope and forbidden-changes before coding, per
AGENT-DISCIPLINE.md.

## 5. Exit criteria

Mapped directly to ROADMAP.md §4 "Exit":

| ROADMAP §4 exit criterion | Met when |
|---|---|
| workspace smoke tests run | `cargo test` over the workspace passes with the placeholder smoke tests. |
| web shell builds | `apps/web` production build completes without error. |
| placeholder WASM loads | The web shell loads the `wasm-api` artifact and a placeholder call succeeds in a browser/headless smoke. |
| foundation docs are present | `docs/` foundation set present (already true). |
| `engine-core` contains only generic contracts | Boundary review confirms no mechanic nouns in `engine-core`. |
| dependency direction is correct (ARCHITECTURE.md §2) | `cargo tree -p engine-core` shows no dependency on `game-stdlib`, `ai-core`, `wasm-api`, `games/*`, or `apps/web`; boundary review confirms the dependency edges. |

## 6. Acceptance evidence

Gate 0 has no gameplay, so most game-level evidence categories are
`not applicable` and will be exercised from Gate 1 onward. Required here:

| Evidence | Required? | Expected |
|---|---:|---|
| build evidence | yes | `cargo build` (workspace) and `apps/web` build both succeed in CI. |
| smoke tests | yes | Per-crate trivial unit tests pass; web shell load-WASM smoke passes. |
| WASM build smoke | yes | `wasm-api` compiles to a WASM artifact in CI. |
| boundary review | yes | Written confirmation `engine-core` is noun-free (FOUNDATIONS §3 / BOUNDARY §3) AND its dependency edges follow ARCHITECTURE.md §2 (`cargo tree -p engine-core` carries no Rulepath-crate dependency). |
| lint/format | yes | Format check and lint pass in CI (TESTING §17). |
| docs link check | optional | Internal doc/anchor links across `docs/` and `specs/` resolve in CI (TESTING §17 "docs link checks where practical"); mark deferred if not wired at Gate 0. |
| rule/golden/replay/visibility/bot tests | no (n/a) | No game exists; first required at Gate 1. |
| benchmarks | no (n/a) | First required at Gate 1 (TESTING §15). |

## 7. FOUNDATIONS & boundary alignment

| Principle | Stance | Rationale |
|---|---|---|
| §3 `engine-core` is a contract kernel | aligns | Placeholder `engine-core` declares only generic contract vocabulary AND depends on no other Rulepath crate (ARCHITECTURE.md §2); both halves of the boundary are exit criteria. |
| §4 `game-stdlib` is earned | aligns | `game-stdlib` ships empty; no helper is promoted without atlas evidence. |
| §5 Static data is not behavior | aligns | No YAML, no DSL, no static rule data introduced at Gate 0 (also a §12 stop condition kept clear). |
| §9 Local-first v1/v2 | aligns | No accounts, database, hosted service, or server; static web shell only. |
| §11 Universal acceptance invariants | aligns | Rust-authority/TS-presentation split is set up structurally; no legality lives in TypeScript. |
| §12 Stop conditions | clear | Skeleton introduces no mechanic nouns in `engine-core`, no procedural static data, no YAML/DSL, no hosted services, no private-game names. |

This is a dev-process/tooling gate; it engages the principles above because it
establishes the kernel boundary and dependency direction, but it changes no
runtime product behavior (there is none yet).

## 8. Forbidden changes

- Do not add game or mechanic nouns to `engine-core`.
- Do not make `engine-core` depend on `game-stdlib`, `ai-core`, `wasm-api`,
  `games/*`, or `apps/web` (ARCHITECTURE.md §2 dependency direction).
- Do not populate `game-stdlib` with any helper.
- Do not introduce YAML or any DSL.
- Do not add networking, accounts, persistence, or any hosted service.
- Do not let TypeScript hold or decide any rule/legality state.
- Do not use private licensed or private-monster-game names anywhere.
- Do not build any real game mechanic; Gate 1 owns the first game.

## 9. Documentation updates required

These are authored alongside this spec so the next brainstorm does not retread
Gate 0:

| Document | Update |
|---|---|
| `specs/README.md` | Spec index lists this gate; flip its Status to `Done` only after exit criteria pass. |
| `docs/ROADMAP.md` | One-line pointer to `specs/README.md` as the progress tracker (ROADMAP stays law). |
| `docs/README.md` | Short "Implementation specs" pointer noting `specs/` lives outside foundation law. |

## 10. Sequencing

Gate 0 → Gate 1 (`race_to_n`). The next spec, `gate-1-race-to-n.md`, MUST NOT be
admitted until this gate's exit criteria pass and its index status reads `Done`.

## 11. Assumptions

Recorded so they can be corrected in one line:

1. Spec granularity is one spec per gate — assuming the ROADMAP gate is the right
   unit (confirmed during brainstorm).
2. Progress is tracked in `specs/README.md` + a ROADMAP pointer — assuming
   ROADMAP stays immutable law (confirmed during brainstorm).
3. CI provider is GitHub Actions (`.github/workflows/`) — assuming the repo's
   eventual host; swap the CI config location if it changes. No foundation doc
   pins a provider.
4. `wasm-api` builds via the standard `wasm-bindgen`/`wasm-pack`-style toolchain
   — assuming the conventional Rust→WASM path; the exact tool is an
   implementation choice for the WB2 AGENT-TASK, not fixed by this spec.

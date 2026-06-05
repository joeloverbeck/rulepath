# Spec: Gate 1 — `race_to_n`

- Spec ID: `gate-1-race-to-n`
- Roadmap stage: 1
- Roadmap build gate: Gate 1
- Status: Done
- Date: 2026-06-05
- Owner: joeloverbeck

This spec is an implementation plan. It is subordinate to the foundation set in
[`../docs/README.md`](../docs/README.md) and MUST NOT redefine any foundation
contract. Where this spec and a foundation document disagree, the foundation
document wins. Authority order for this spec:
[`../docs/FOUNDATIONS.md`](../docs/FOUNDATIONS.md),
[`../docs/ARCHITECTURE.md`](../docs/ARCHITECTURE.md),
[`../docs/ENGINE-GAME-DATA-BOUNDARY.md`](../docs/ENGINE-GAME-DATA-BOUNDARY.md),
[`../docs/OFFICIAL-GAME-CONTRACT.md`](../docs/OFFICIAL-GAME-CONTRACT.md),
[`../docs/MECHANIC-ATLAS.md`](../docs/MECHANIC-ATLAS.md),
[`../docs/AI-BOTS.md`](../docs/AI-BOTS.md),
[`../docs/UI-INTERACTION.md`](../docs/UI-INTERACTION.md),
[`../docs/TESTING-REPLAY-BENCHMARKING.md`](../docs/TESTING-REPLAY-BENCHMARKING.md),
[`../docs/ROADMAP.md`](../docs/ROADMAP.md),
[`../docs/IP-POLICY.md`](../docs/IP-POLICY.md),
[`../docs/AGENT-DISCIPLINE.md`](../docs/AGENT-DISCIPLINE.md).

## 1. Objective

Implement `race_to_n`, the first official Rulepath game: a tiny two-seat,
perfect-information, deterministic numeric game (Nim / subtraction family). Its
public role is **plumbing proof** — it proves setup, turn order, flat legal
action generation, validation, command application, terminal detection, semantic
effects, deterministic replay with reproducible hashes, a random legal bot, and
the end-to-end WASM path, **without hiding architecture mistakes** behind polish.

`race_to_n` is a `foundation-smoke` game (OFFICIAL-GAME-CONTRACT §2): visually
modest but held to the **full** official evidence contract. Browser playability
without rule coverage, traces, replay, bot legality, benchmarks, and docs is a
demo shell, not an official game (FOUNDATIONS §6).

Source of truth for this gate: ROADMAP.md §3 (Stage 1 row) and §5 (Gate 1),
plus ROADMAP §2 per-stage requirements.

## 2. Scope

### In scope

- A game crate `games/race_to_n/` (ARCHITECTURE §11 module shape): typed Rust
  setup, flat legal action generation, validation, transitions, terminal/outcome
  detection, deterministic randomness, semantic effects, public-view projection,
  serialization, and a single declared variant.
- Minimal **generic** contract growth in `engine-core`, driven by what
  `race_to_n` actually exercises: action tree / action path, deterministic RNG
  contract, effect cursor/log, a generic `Game`-style entry contract over an
  opaque game-defined payload, a freshness-token contract (a generic version
  marker for graceful stale-submission rejection, ARCHITECTURE §5),
  state/effect/action-tree/view hash contract, and
  the replay/command-stream + serialization boundary. All noun-free
  (FOUNDATIONS §3); each addition clears the AGENT-DISCIPLINE §5 kernel-change
  protocol.
- A **generic random legal bot** in `ai-core` (Level 0) that picks uniformly
  among Rust-supplied legal action paths via the deterministic RNG contract;
  `race_to_n` wires it. (ARCHITECTURE §3: random legal bot is an `ai-core`
  responsibility.)
- Native CLI random legal simulation through `tools/simulate` for `race_to_n`:
  100,000+ seeds, per-action invariant checks, and failure seed/command output
  (OFFICIAL-GAME-CONTRACT §1; TESTING §7).
- Native benchmark coverage for `race_to_n` (legal-action generation, apply,
  view projection / effect filtering, serialization + replay throughput,
  random-playout throughput, and random-bot decision latency) against the
  Stage-1 budget — the OFFICIAL-GAME-CONTRACT §1 / TESTING §14 "measure at
  least" floor (TESTING §15: 500,000+ games/sec native target).
- The full per-game evidence set: unit, named rule, golden trace, property/
  invariant, simulation, replay/hash, and serialization tests (TESTING §1).
- A **minimal WASM vertical slice**: a thin batched `wasm-api` surface
  (`new_match`, `get_view`, `get_action_tree`, `apply_action`, `run_bot_turn`,
  `get_effects`) and a **bare** `apps/web` harness page that plays human vs
  random bot. No picker, no stores, no polish.
- Per-game docs under `games/race_to_n/docs/` (ARCHITECTURE §11 naming):
  `SOURCES.md`, `RULES.md`, `RULE-COVERAGE.md`, `MECHANICS.md`, `AI.md`,
  `UI.md`, `BENCHMARKS.md`, plus a `GAME-IMPLEMENTATION-ADMISSION` receipt and a
  mechanic-atlas confirmation.
- The documentation updates listed in §10.

### Out of scope (deferred to later gates or forbidden here)

- The **polished WASM/static web shell**: game picker, public-view store, action
  tree store, effect queue, replay controls, replay viewer, dev toggle, and
  local replay import/export UI (**Gate 3**, ROADMAP §5).
- **Trace/replay/benchmark tooling hardening**: standalone `replay-check`,
  `trace-viewer`, `seed-reducer`, `fixture-check`, and `bench-report`
  generalization; golden-trace drift-loud CI tooling; the general benchmark
  harness (**Gate 2**, ROADMAP §5). Gate 1 proves replay/hash **in-crate** with a
  basic replay test and at least one golden trace; Gate 2 hardens the tools.
- Any second game, any `game-stdlib` helper (first use is local-only,
  FOUNDATIONS §4), Level 1+ bots, hidden information, and visual polish.
- Any networking, accounts, database, or hosted service.

### Not allowed (ROADMAP §5 Gate 1)

- Generalized piles, decks, boards, tracks, or resources.
- Multiplayer.
- A polished-renderer detour.
- (Carried from the constitution) mechanic nouns in `engine-core`; YAML or DSL;
  data-driven rule behavior; TypeScript legality; private-game names.

## 3. Deliverables

Target additions after this gate (concrete file names MAY vary; responsibilities
MUST NOT), per ARCHITECTURE §11:

```text
games/
  race_to_n/
    Cargo.toml            # depends on engine-core (+ ai-core traits); NOT engine-core internals
    src/
      lib.rs ids.rs state.rs setup.rs actions.rs rules.rs
      visibility.rs effects.rs variants.rs bots.rs ui.rs
    data/
      manifest.toml       # typed metadata only (no rule behavior)
      variants.toml       # typed variant selection only
      fixtures/
    docs/
      RULES.md SOURCES.md RULE-COVERAGE.md MECHANICS.md AI.md UI.md BENCHMARKS.md
    tests/
      golden_traces/ rule_tests.rs property_tests.rs
      simulation_tests.rs serialization_tests.rs replay_tests.rs bot_tests.rs
crates/
  engine-core/            # gains generic, noun-free contracts driven by race_to_n
  ai-core/                # gains the generic random legal bot (Level 0)
  wasm-api/               # gains the minimal batched gameplay surface
apps/
  web/                    # bare race_to_n harness page (human vs random bot)
tools/
  simulate/               # wired to run race_to_n simulations with failure output
benches/                  # native race_to_n benchmark
```

`race_to_n` is registered so `wasm-api` can resolve it through the games registry
(ARCHITECTURE §2: `wasm-api -> games registry + engine-core contracts`). The
dependency direction MUST hold: `games/race_to_n -> engine-core + ai-core traits`
only; `engine-core` depends on no Rulepath crate. `apps/web` reaches Rust only
through the `wasm-api` package boundary (ARCHITECTURE §2).

Static data (`manifest.toml`, `variants.toml`, `fixtures/`) is typed content,
parameters, and fixtures only — no selectors, conditions, or rule branches
(FOUNDATIONS §5). Unknown fields in hand-authored data are rejected by default
(FOUNDATIONS §11).

## 4. Work breakdown

Each item is a candidate [`../templates/AGENT-TASK.md`](../templates/AGENT-TASK.md)
packet, ordered by dependency. Each is decomposed into a bounded AGENT-TASK with
forbidden-changes before coding (AGENT-DISCIPLINE §2).

| ID | Work item | Depends on | Becomes AGENT-TASK |
|---|---|---|---|
| WB1 | **Rules research + docs scaffold (requirements-first).** `SOURCES.md` (sources consulted, IP/naming rationale), original `RULES.md` with stable rule IDs, the variant + win-condition decision, `MECHANICS.md` inventory, a `RULE-COVERAGE.md` skeleton, and the `GAME-IMPLEMENTATION-ADMISSION` receipt. No code. Pins the variant (resolves Assumption 1). | — | yes |
| WB2 | **`engine-core` generic contract extension**, driven by WB1's mechanic needs: action tree/path, deterministic RNG contract, effect cursor/log, generic game-entry contract over an opaque payload, hash contract (state/effect/action-tree/view), replay/command-stream + serialization boundary. Noun-free; each addition answers the AGENT-DISCIPLINE §5 kernel-change protocol. | WB1 | yes |
| WB3 | **`games/race_to_n` core rules.** Setup, ids, state, flat legal action generation, validation (emitting viewer-safe `Diagnostic`s + freshness-token rejection of stale submissions), transitions, terminal/outcome detection, semantic effects, public-view projection, serialization, variant. Forbidden: any mechanic noun in `engine-core`. | WB2 | yes |
| WB4 | **Level 0 random legal bot.** Generic random-legal-bot in `ai-core` over the deterministic RNG contract + Rust-supplied legal paths; `race_to_n` wires it; bot legality + determinism tests (AI-BOTS; TESTING §10). | WB3 | yes |
| WB5 | **Replay, hashing, golden traces, serialization tests.** Seed + options + command stream reproduce state/effect/action-tree/view hashes; ≥1 shortest-normal and ≥1 terminal golden trace; one bot-action trace; one invalid/stale diagnostic trace; replay + serialization round-trip tests; property/invariant tests. | WB3, WB4 | yes |
| WB6 | **Native simulation + benchmarks.** Wire `tools/simulate` to run 100,000+ random `race_to_n` games with per-action invariant checks and failure seed/command output; native benchmarks covering the OFFICIAL-GAME-CONTRACT §1 / TESTING §14 floor (legal actions, apply, view/effect filtering, serialization + replay throughput, random-playout throughput, random-bot decision latency) hitting the Stage-1 budget; fill `BENCHMARKS.md` (TESTING §15–16). Pin the benchmark home (a top-level `benches/` workspace crate per ARCHITECTURE §1, currently a non-member placeholder, or an in-crate `games/race_to_n/benches/` criterion target) and wire it into the workspace. | WB3, WB4 | yes |
| WB7 | **Minimal WASM vertical slice + bare web harness + UI smoke.** Batched `wasm-api` surface; `apps/web` bare harness (human vs random bot, no polish); `UI.md` (minimal); UI smoke covering load, start, display legal actions, one human action, one bot turn, semantic effects, and the stale-submission diagnostic path. Forbidden: TypeScript legality; any Gate-3 shell scope. | WB3, WB4 | yes |
| WB8 | **Docs finalize + CI wiring + index flip.** Close `RULE-COVERAGE.md` (no `open` rows), finalize `AI.md`, confirm the `race_to_n` mechanic-atlas row stays `local-only`; extend CI with race_to_n rule/golden/replay/serialization/quick-sim/UI-smoke/bench-smoke; flip the `specs/README.md` index status to `Done`. | WB1–WB7 | yes |

Implementation does not begin from this spec alone. Each WB item is decomposed
into an AGENT-TASK with bounded scope and forbidden-changes before coding, per
AGENT-DISCIPLINE.

## 5. Exit criteria

Mapped row-for-row to ROADMAP.md §5 (Gate 1) "Exit":

| ROADMAP §5 Gate 1 exit criterion | Met when |
|---|---|
| human vs random bot works in CLI and web | A native path (CLI sim / hotseat harness) and the bare WASM harness both play a human seat vs the Level 0 random legal bot to a terminal outcome. |
| 100,000 native random games complete without crash | `tools/simulate` runs ≥100,000 seeded `race_to_n` games; per-action invariants hold; no panic; failing seeds are reproducible. |
| replay reproduces hashes | Replay tests prove seed + options + command stream reproduce identical state, effect, action-tree, and public-view hashes (ARCHITECTURE §8; TESTING §4). |
| invalid/stale diagnostics are tested | Validation emits viewer-safe diagnostics for invalid paths; a freshness token rejects stale submissions; both are covered by tests and one diagnostic golden trace. |
| per-game docs and mechanic inventory exist | `games/race_to_n/docs/*` are filled with no silent gaps; `MECHANICS.md` + the mechanic-atlas `race_to_n` row exist. |

## 5.1 Closeout Evidence

Recorded 2026-06-05 before flipping `specs/README.md` Gate 1 to `Done`.

| Criterion/evidence | Command or review | Result |
|---|---|---|
| workspace tests, including rule/golden/replay/serialization/bot/sim/wasm tests | `cargo test --workspace` | passed |
| 100,000 native random games | `cargo run -p simulate -- --game race_to_n --games 100000` | passed; `games_run=100000`, `seat_0_wins=49743`, `seat_1_wins=50257`, `average_length=10.92` |
| web human-vs-bot/stale diagnostic path | `npm --prefix apps/web run smoke:ui` | passed; output recorded `counter=2`, `effects=8`, `diagnostic=stale_action` |
| web build | `npm --prefix apps/web run build` | passed |
| engine boundary | `bash scripts/boundary-check.sh` | passed; `engine-core` boundary check passed |
| docs links | `node scripts/check-doc-links.mjs` | passed; checked 17 markdown files |
| no open coverage rows | `grep -nE '\bopen\b' games/race_to_n/docs/RULE-COVERAGE.md` | no matches |
| no TypeScript legality terms in app source | `grep -rniE 'legal\|isValid\|canPlay' apps/web/src` | no matches |
| mechanic atlas confirmation | `grep -n 'race_to_n' docs/MECHANIC-ATLAS.md` | row reads `local-only`; no `game-stdlib` promotion |
| benchmark evidence | `cargo bench -p race_to_n` and `games/race_to_n/docs/BENCHMARKS.md` | benchmark coverage exists; Stage-1 random-playout budget miss is recorded, not silently claimed |

## 6. Acceptance evidence

| Evidence | Required? | Expected |
|---|---:|---|
| build evidence | yes | `cargo build --workspace` and `apps/web` build succeed in CI. |
| unit + rule tests | yes | Named rule tests reference stable `RULES.md` IDs and coverage rows (TESTING §2). |
| golden trace tests | yes | Shortest-normal, terminal, bot-action, and invalid/stale diagnostic traces, with notes (OFFICIAL-GAME-CONTRACT §11). |
| property/invariant tests | yes | Legal-action gen never panics/produces invalid states; conservation/turn-order/terminal invariants hold (TESTING §6). |
| simulation tests | yes | 100,000+ seeds via `tools/simulate`; invariant checks; failure seed/command output (TESTING §7). |
| replay/hash tests | yes | Seed + commands reproduce state/effect/action-tree/view hashes (TESTING §4). |
| serialization tests | yes | Snapshot + public-view + replay JSON round trips; version-field presence; unknown-field rejection (TESTING §9). |
| bot legality tests | yes | Random legal bot chooses legal paths through normal validation; deterministic for fixed seed/view/limits (TESTING §10). |
| benchmarks | yes | Native benchmarks for legal-action gen, apply, view/effect filtering, serialization + replay throughput, random-playout throughput, and random-bot decision latency vs Stage-1 budget (OFFICIAL-GAME-CONTRACT §1; TESTING §14–15). |
| UI smoke | yes (web-exposed) | Bare harness: load, start, legal actions, one human action, one bot turn, effects, stale-submission path (TESTING §11). |
| boundary review | yes | Written confirmation: Rust holds all behavior; no legality in TypeScript; `engine-core` stays noun-free; dependency edges per ARCHITECTURE §2. |
| visibility/no-leak tests | no (n/a) | `race_to_n` is perfect-information; no hidden state exists to leak. View projection is wholly public; recorded as `not-applicable` with rationale. |
| Level 1+ bot evidence | no (n/a) | Only a Level 0 random legal bot at Gate 1; `COMPETENT-PLAYER` / `BOT-STRATEGY-EVIDENCE-PACK` are `not-applicable`. |
| primitive-pressure ledger | no (n/a) | First use; mechanic stays `local-only` (MECHANIC-ATLAS §10). |
| public release checklist | no (deferred) | `race_to_n` is not publicly released at Gate 1; `PUBLIC-RELEASE-CHECKLIST` is exercised when the Gate-3 static site ships it. |

## 7. FOUNDATIONS & boundary alignment

| Principle | Stance | Rationale |
|---|---|---|
| §2 Behavior authority | aligns | Setup, legal actions, validation, transitions, scoring, RNG, effects, replay, and bot decisions all live in Rust; TypeScript presents only. |
| §3 `engine-core` is a contract kernel | aligns (watch) | `engine-core` grows only generic, noun-free contracts (action tree, RNG, effect, hash, replay, serialization) earned by a real game; mechanic vocabulary stays in `games/race_to_n`. WB2 is the highest-risk surface — each addition clears the AGENT-DISCIPLINE §5 kernel-change protocol and a §12 stop-condition check. |
| §4 `game-stdlib` is earned | aligns | `game-stdlib` stays empty; the random legal bot lives in `ai-core`; the `race_to_n` mechanic stays `local-only` (first use). |
| §5 Static data is not behavior | aligns | `manifest.toml`/`variants.toml` carry typed metadata and variant selection only; no selectors/conditions/branches; unknown fields rejected. |
| §6 Official games are evidence-heavy | aligns | Full evidence set (rules, docs, tests, traces, replay, simulation, benchmarks, bot, UI smoke) is required by §5–§6 here. |
| §7/§10 Legal-only UI, no TS legality | aligns | The bare harness builds controls from Rust legal action trees and submits action paths with freshness tokens; TypeScript invents no legality. |
| §8 Bots are product opponents | aligns | Level 0 random legal bot is fair, deterministic under declared inputs, uses the normal legal action API, and accesses no hidden state. No MCTS/ISMCTS/Monte Carlo/ML/RL. |
| §9 Local-first v1/v2 | aligns | Native + static local WASM only; no accounts, database, or hosted service. |
| §11/§12 Invariants & stop conditions | clear | Replay/hash/serialization deterministic; no YAML/DSL; no data-driven rules; no mechanic nouns in `engine-core`; no private-game names. The one live stop-condition watch is `engine-core` noun creep via WB2, explicitly gated. |

This is a planning artifact (dev-process/docs), but the work it governs is
product-behavior; the table above aligns that behavior to the constitution.

## 8. Forbidden changes

- Do not add any mechanic/domain noun (`pile`, `track`, `deck`, `board`,
  `resource`, etc.) to `engine-core`; keep WB2 additions generic and noun-free.
- Do not generalize `race_to_n` into piles/decks/boards/tracks/resources
  (ROADMAP §5 Not allowed).
- Do not populate `game-stdlib` with any helper (first use is local-only).
- Do not build the Gate-3 shell scope: picker, view/action/effect stores, effect
  queue, replay controls, replay viewer, dev toggle, or local import/export UI.
- Do not build the Gate-2 tooling hardening (`replay-check`, `trace-viewer`,
  `seed-reducer`, `fixture-check`, `bench-report` generalization).
- Do not let TypeScript hold or decide any rule/legality state.
- Do not introduce YAML, a DSL, data-driven rules, or procedural static data.
- Do not let the bot bypass the legal action API or read unauthorized state.
- Do not add networking, accounts, persistence, or any hosted service.
- Do not use private licensed or private-monster-game names anywhere.
- Do not copy rulebook prose; `RULES.md` MUST be original Rulepath prose
  (IP-POLICY; OFFICIAL-GAME-CONTRACT §5).

## 9. Documentation updates required

| Document | Update |
|---|---|
| `specs/README.md` | Flip the Gate 1 index row to `Planned` on spec authoring (done with this spec) and to `Done` only after §5 exit criteria pass with evidence. |
| `games/race_to_n/docs/*` | Author `SOURCES.md`, `RULES.md`, `RULE-COVERAGE.md`, `MECHANICS.md`, `AI.md`, `UI.md`, `BENCHMARKS.md` from verified rules and implemented behavior. |
| `docs/MECHANIC-ATLAS.md` | Confirm/keep the `tiny numeric turn race` → `race_to_n` row as `local-only` (no extraction; first use). |
| `docs/ROADMAP.md` | Not edited — the roadmap is immutable law; progress lives in `specs/README.md`. |

## 10. Sequencing

Gate 0 (`Done`) → **Gate 1 (`race_to_n`)** → Gate 2 (trace, replay, and benchmark
hardening). This spec is admitted now because Gate 0's exit criteria pass and its
index status reads `Done`. The Gate 2 spec MUST NOT be admitted until this gate's
exit criteria pass and its index status reads `Done`.

## 11. Assumptions

Recorded so they can be corrected in one line:

1. The exact variant + win condition (e.g. take-the-last vs misère subtraction,
   or race-to-N counting) is **pinned by WB1 rules research**, not by this spec —
   assuming the requirements-first workflow (OFFICIAL-GAME-CONTRACT §3) owns that
   choice. The spec only fixes the mechanic family (tiny two-seat,
   perfect-information, deterministic numeric game).
2. "Web" at Gate 1 is a **minimal WASM vertical slice** (bare harness, human vs
   random bot) — confirmed during brainstorm; the polished shell is Gate 3.
3. Replay/hash is proven **in-crate** at Gate 1 (basic replay test + ≥1 golden
   trace); the standalone replay/trace/seed/fixture/bench **tool hardening** is
   Gate 2 — assuming the roadmap's gate split maps cleanly to tool granularity.
4. The Level 0 random legal bot lives in `ai-core` as a generic helper over
   Rust-supplied legal paths — assuming ARCHITECTURE §3's placement of the random
   legal bot; `race_to_n` only wires it.
5. `race_to_n` carries the `foundation-smoke` readiness label
   (OFFICIAL-GAME-CONTRACT §2) — full evidence, modest visuals.

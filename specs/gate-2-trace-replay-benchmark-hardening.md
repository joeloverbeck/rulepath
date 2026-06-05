# Spec: Gate 2 — Trace, Replay, Fixture, and Benchmark Hardening

## 1. Header

- Spec ID: `gate-2-trace-replay-benchmark-hardening`
- Roadmap stage: 1 hardening / repository discipline after Gate 1
- Roadmap build gate: Gate 2
- Status: Planned
- Suggested landing path: `specs/gate-2-trace-replay-benchmark-hardening.md`
- Prepared: 2026-06-05
- Owner: `joeloverbeck`
- Repository requested: `joeloverbeck/rulepath`
- Target commit reviewed: `b2038fa72d1695b493887b404c92c915a1ff2e6c`
- Uploaded manifest commit: `b2038fa72d1695b493887b404c92c915a1ff2e6c`

This spec is an implementation plan. It is subordinate to the foundation set in `docs/README.md` and MUST NOT redefine any foundation contract. Where this spec and a foundation document disagree, the foundation document wins. Authority order for this spec:

1. `docs/FOUNDATIONS.md`
2. `docs/ARCHITECTURE.md`
3. `docs/ENGINE-GAME-DATA-BOUNDARY.md`
4. `docs/OFFICIAL-GAME-CONTRACT.md`
5. `docs/MECHANIC-ATLAS.md`
6. `docs/AI-BOTS.md`
7. `docs/UI-INTERACTION.md`
8. `docs/TESTING-REPLAY-BENCHMARKING.md`
9. `docs/ROADMAP.md`
10. `docs/IP-POLICY.md`
11. `docs/AGENT-DISCIPLINE.md`
12. `specs/README.md`

### Evidence ledger

| Evidence item | Result |
|---|---|
| Requested repository | `joeloverbeck/rulepath` |
| Repository returned by stale `get_repo` connector action | `joeloverbeck/one-more-branch`; rejected as stale and not used as repository identity. |
| Repository identity verified through installed-repository listing | yes: `joeloverbeck/rulepath`, repository id `1260178999`, default branch `main`, public repository. |
| Default branch | `main` |
| Current default/main branch SHA | verified at reassessment: local `git rev-parse HEAD` on `main` returns `b2038fa72d1695b493887b404c92c915a1ff2e6c`. |
| Uploaded manifest commit SHA | `b2038fa72d1695b493887b404c92c915a1ff2e6c` |
| Exact-commit targeted file fetches | yes; files were fetched from explicit `joeloverbeck/rulepath` blob URLs at `b2038fa72d1695b493887b404c92c915a1ff2e6c`. |
| Repository identity verified | yes, after bypassing stale connector identity output. |
| SHA match verified | yes; at reassessment, local `main` HEAD equals the reviewed/manifest commit `b2038fa72d1695b493887b404c92c915a1ff2e6c`. |
| Manifest use | used only as inventory after repository identity verification and exact-commit file fetches. |
| GitHub code search/snippet search | not used. |
| Clone | not used. |

This identity uncertainty has since been resolved locally: at reassessment (2026-06-05) `git rev-parse HEAD` on `main` returns the reviewed/manifest commit `b2038fa72d1695b493887b404c92c915a1ff2e6c`, so the spec was prepared against current `main`. If the spec is landed from a different checkout, re-verify `main` HEAD before deriving tickets.

### Current gate finding

Gate 0 and Gate 1 are archived as `Done`. `specs/README.md` lists Gate 2 as `Not started` and not yet specced. Gate 2 is therefore the lowest non-done gate. The correct next spec is:

`Spec: Gate 2 — Trace, Replay, Fixture, and Benchmark Hardening`

## 2. Objective

Convert Gate 1's embedded `race_to_n` proof into repository-grade, CI-grade, CLI-grade hardening discipline.

Gate 1 already proved that `race_to_n` can run through Rust rules, validation, replay-ish tests, stable hash surfaces, simulation, benchmarks, WASM, and a bare web harness. That proof is currently too local: the golden traces are legacy key-value files, replay checking lives inside game tests, benchmark output is CSV-like stdout with no threshold gate, and most Gate 2 tool crates are no-op placeholders.

Gate 2 makes that evidence loud, structured, reproducible, and future-game-ready without pretending Rulepath is a universal tabletop engine. It must harden trace serialization, replay checking, fixture validation, stable hash drift detection, benchmark reports, CI hard-fail thresholds, failure seed/command output, minimal seed reproduction, rule-coverage drift checks, and human-readable trace inspection.

Source of truth: `docs/ROADMAP.md` Gate 2; `docs/TESTING-REPLAY-BENCHMARKING.md`; `docs/OFFICIAL-GAME-CONTRACT.md`; `docs/FOUNDATIONS.md`; `docs/ENGINE-GAME-DATA-BOUNDARY.md`; `docs/AGENT-DISCIPLINE.md`.

## 3. Scope

### In scope

| Area | Required Gate 2 stance |
|---|---|
| Trace format | Define `Trace Schema v1` as structured JSON for golden traces and replay fixtures. Migrate current `race_to_n` legacy `.trace` files. Keep a temporary legacy parser/import path only as a migration aid. |
| Replay checker | Implement real `tools/replay-check` for `--game race_to_n`, one trace or all known `race_to_n` traces, with loud hash/diagnostic/outcome drift failures. |
| Fixture checker | Implement real `tools/fixture-check` for schema strictness, required notes, duplicate IDs, behavior-looking fields, legacy migration status, `manifest.toml`, and `variants.toml`. |
| Benchmarks | Keep the current native custom benchmark harness unless implementation evidence proves it untenable. Emit structured JSON plus human summary. Implement `tools/bench-report` threshold checking. CI must hard-fail benchmark threshold failures. |
| Stage-1 playout miss | Triage the recorded `random_playout` miss. Gate 2 is not done until the 500,000 games/sec target is met or an explicit accepted benchmark-doctrine/ADR/spec adjustment recalibrates it. |
| Seed reducer | Replace the no-op placeholder with honest `seed-reducer` v0: normalize a failing seed/command stream into a replay command or trace; optionally do bounded prefix minimization only when a failure predicate is available. |
| Rule coverage | Implement lightweight structural `rule-coverage` for `race_to_n` docs. It checks coverage integrity; it does not prove rule correctness. |
| Trace viewer | Implement minimal `trace-viewer` CLI summary for Trace Schema v1 traces. Browser replay viewer remains Gate 3. |
| CI | Wire hard-failing checks: replay-check, fixture-check, rule-coverage, quick simulation, benchmark smoke, bench-report threshold gate, boundary check, WASM smoke, web build, UI smoke, docs link check. |
| Docs | Update `specs/README.md`, `games/race_to_n/docs/RULE-COVERAGE.md`, `games/race_to_n/docs/BENCHMARKS.md`, and trace/benchmark doctrine docs as needed. |

### Out of scope

| Area | Gate 2 stance |
|---|---|
| Gate 3 web shell | out of scope. No replay browser UI, no game picker, no public-view store, no action/effect store refactor unless required for current smoke stability. |
| New games | out of scope. Do not start `three_marks` or any other game. |
| Generic tabletop engine | forbidden. Do not create board/card/resource abstractions or move game nouns into `engine-core`. |
| `game-stdlib` helpers | forbidden in Gate 2. First-use `race_to_n` mechanics stay local. |
| Universal replay trait | not the default. Add only if actual code pressure proves necessary, and then keep it contract-only, noun-free, and narrow. |
| Semantic rule coverage analyzer | out of scope. `rule-coverage` is structural only. |
| True fuzzing framework | out of scope. `seed-reducer` v0 is a reproducer/minimal reducer only. |
| Criterion or Iai-Callgrind migration | not in scope by default. Record as future option if wall-clock benchmark noise makes the current harness unusable. |
| Multiplayer/accounts/persistence | forbidden. |
| Level 1+ search bots | forbidden. No MCTS, ISMCTS, Monte Carlo, ML, or RL bots. |
| Private/licensed content | forbidden. |

### Explicit `not applicable` rows

| Surface | Status | Rationale |
|---|---|---|
| Hidden-information trace redaction for `race_to_n` | not applicable, but must be recorded in each trace set | `race_to_n` is perfect-information; the trace schema must still require an explicit rationale so future hidden-information games do not silently omit this surface. |
| Stochastic game-rule event trace for `race_to_n` | not applicable, but must be recorded in each trace set | `race_to_n` rules use no randomness; bot RNG exists outside game rules and is covered separately. |
| Private-view hash for `race_to_n` | not applicable unless a private-view API is added | All state is public; public-view hash remains required. |
| Preview hash for `race_to_n` | not applicable unless Rust preview surface is added | Gate 1 omitted preview with rationale. Gate 2 must not invent preview behavior. |
| Browser replay viewer | not applicable to Gate 2 | Gate 3 owns replay controls/viewer. |
| `game-stdlib` promotion | not applicable and forbidden | First-use local mechanic; no repeated-shape pressure. |

## 4. Deliverables

### D1 — Trace Schema v1

Gate 2 must define a formal JSON schema for traces and replay fixtures. The current key-value `.trace` format is useful proof-of-life, not durable evidence.

Decision: **migrate to structured JSON**.

Current legacy `.trace` files may be parsed only during migration. After Gate 2, golden traces used by CI must be Trace Schema v1 JSON. Legacy traces may remain only if they are explicitly marked as migration fixtures and `fixture-check` verifies their migration status.

Trace files are evidence. They are not rule behavior. They must not become a DSL.

Minimum Trace Schema v1 root fields:

| Field | Required? | Notes |
|---|---:|---|
| `schema_version` | yes | Exact integer/string version for Trace Schema v1. |
| `trace_id` | yes | Stable unique trace ID; duplicate IDs are a fixture-check failure. |
| `fixture_kind` | yes | Example values: `commands`, `terminal`, `bot`, `invalid`, `diagnostic`, `not_applicable`. Values are classification only, not behavior. |
| `purpose` | yes | Short machine-readable purpose. |
| `note` | yes | Human explanation for why the trace exists. Empty notes fail fixture-check. |
| `migration_update_note` | yes | Initial migration note for converted traces; required non-empty whenever expected hashes or schema fields are updated. |
| `game_id` | yes | `race_to_n` for Gate 2. |
| `rules_version` | yes | Canonical representation is the string form `race_to_n-rules-v1` (as in `RULES.md` / `RULE-COVERAGE.md`), not the bare integer. `data/manifest.toml` and `BENCHMARKS.md` currently use the integer `1`; reconcile them to the string form (or carry a documented integer↔string mapping) so the `fixture-check` consistency check in D4 has one unambiguous anchor. |
| `engine_version` | yes | Must identify the engine contract version used for hashes. |
| `data_version` | yes | Must match manifest/data version. |
| `seed` | yes | Setup seed. For `race_to_n`, game rules ignore seed, but replay evidence still records it. |
| `variant` | yes | Selected variant ID, e.g. `race_to_21`. |
| `options` | yes | Structured setup/options object, even if empty/default. |
| `seats` | yes | Ordered seat/player mapping where relevant. |
| `commands` | yes unless `fixture_kind=not_applicable` | Ordered stream of explicit command records. Each command has index, actor seat, action path segments, freshness token used or expected diagnostic path, and optional producer metadata. |
| `checkpoints` | yes | Ordered checkpoints, including at least final checkpoint for normal traces. |
| `expected_state_hashes` | yes | Hashes by checkpoint or final hash. |
| `expected_effect_hashes` | yes | Hashes by checkpoint or final effect surface. |
| `expected_action_tree_hashes` | yes where action tree exists | Required for normal and invalid/stale traces; terminal empty-tree hash required for terminal traces. |
| `expected_public_view_hashes` | yes | Include selected viewers; for `race_to_n`, public/all-viewer hash is required. |
| `expected_private_view_hashes` | conditional | Either provide selected private-view hashes or explicit `not_applicable` rationale. |
| `expected_diagnostics` | conditional | Required for invalid/stale/diagnostic traces. |
| `expected_outcome` | yes | Terminal/winner status or explicit non-terminal expectation. |
| `expected_terminal_state` | yes | Boolean plus winner/rationale where terminal. |
| `not_applicable` | yes | Explicit hidden-information and stochastic-game-event rationale for `race_to_n`. |

Command stream minimum fields:

| Field | Required? | Notes |
|---|---:|---|
| `index` | yes | Zero-based command index. |
| `actor_seat` | yes | Explicit actor seat; no implicit active-seat behavior in fixture data. |
| `action_path` | yes | Ordered action path segments as data. |
| `freshness_token` | yes | Token submitted by the command. This records evidence; Rust still validates it. |
| `expect` | yes | `applied` or `diagnostic`. |
| `expected_diagnostic_code` | conditional | Required when `expect=diagnostic`. |
| `producer` | conditional | For bot traces, record bot policy/version/seed and the resulting explicit command. Replay-check may verify the bot chose the same command, but the trace still contains the command stream. |

Forbidden in trace JSON: `when`, `if`, `then`, `selector`, `condition`, `trigger`, `script`, `loop`, `foreach`, `priority_expression`, `ai_condition`, `effect_script`, `rule`, `requires`, `valid_if`, `on_play`, `on_reveal`, or any field that turns the fixture into behavior.

### D2 — `race_to_n` replay/hash support extraction

Move canonical `race_to_n` trace/replay/hash evaluation out of test-only helper code into a game-local support module or narrow public game API that tests and tools can both call.

Recommended target: `games/race_to_n/src/replay_support.rs` or equivalent game-local module.

Rules:

- Do not put game-specific trace logic in `engine-core`.
- Do not promote anything to `game-stdlib`.
- Prefer game-local functions over a generic trait.
- Add a narrow contract-only trait only if duplication becomes worse than the abstraction, and only after applying the kernel-change protocol.

### D3 — Real `tools/replay-check`

Minimum CLI:

```text
cargo run -p replay-check -- --game race_to_n --trace <path>
cargo run -p replay-check -- --game race_to_n --all
cargo run -p replay-check -- --game race_to_n --directory games/race_to_n/tests/golden_traces
```

Required behavior:

- Parse Trace Schema v1 JSON.
- Optionally import legacy `.trace` only under explicit migration mode.
- Replay setup/options/commands through Rust behavior.
- Compare expected vs actual state hash, effect hash, action-tree hash, public-view hash, private-view hash where applicable, diagnostics, terminal status, and outcome.
- Fail non-zero on any drift, malformed trace, unsupported game, unsupported schema version, duplicate trace ID, missing expected surface, or invalid migration note.

Required failure output:

- trace path
- trace ID
- game ID
- schema/rules/engine/data versions
- command index and checkpoint
- expected vs actual hash/surface
- diagnostic code mismatch where relevant
- replay command that reproduces the failure
- reminder that intentional updates require a migration/update note

### D4 — Real `tools/fixture-check`

Minimum CLI:

```text
cargo run -p fixture-check -- --game race_to_n
cargo run -p fixture-check -- --game race_to_n --trace <path>
```

Required checks:

- Trace Schema v1 version and required fields.
- Unknown fields rejected.
- Duplicate IDs rejected.
- Empty `note` rejected.
- Missing `migration_update_note` rejected where required.
- Game/rules/data/engine version fields present and consistent. The `rules_version` consistency anchor is the canonical string form `race_to_n-rules-v1` (see D1); flag any trace, `manifest.toml`, or doc whose `rules_version` does not reduce to that canonical form.
- Behavior-looking fields rejected recursively.
- Legacy trace migration status checked if legacy files remain.
- No YAML files under trace/fixture/report paths.
- `manifest.toml` and `variants.toml` strictness checked where practical.
- Behavior-in-data keys rejected in fixture/static data contexts.

### D5 — Structured benchmark output and `tools/bench-report`

Keep the existing native custom benchmark harness for Gate 2 unless implementation evidence proves it cannot support reliable gating. The current harness already measures the right operations; the missing piece is structured output and threshold enforcement.

Required benchmark output:

- JSON machine report.
- Human-readable summary.
- Stable operation names.
- Required metadata:
  - hardware/environment notes when available
  - OS
  - Rust version
  - command
  - build profile
  - game ID
  - rules version
  - data version
  - engine version
  - operation name
  - iterations
  - unit
  - current value
  - threshold
  - pass/fail
  - known caveats

Required `bench-report` behavior:

- Parse benchmark JSON.
- Reject malformed output.
- Reject missing required metadata.
- Reject missing required benchmark operations.
- Compare every required operation to explicit thresholds.
- Hard-fail accepted floors and unapproved regressions.
- Print a concise failure report with operation, current value, threshold, rationale, and environment caveat.

Threshold policy:

- Every threshold must have a rationale: foundation target, measured baseline, conservative CI floor, or accepted ADR/spec adjustment.
- CI thresholds must be conservative enough for hosted/noisy runners but not meaningless.
- Wall-clock benchmark gates are allowed in Gate 2 because the repo already has a custom harness and the user explicitly wants hard-fail gates.
- If wall-clock timing proves too noisy, record a future migration candidate to Iai-Callgrind or a similar instruction-count benchmark. Do not add that dependency in Gate 2 unless the current harness cannot meet hard-fail discipline.

### D6 — Stage-1 random playout budget resolution

The current `race_to_n` benchmark report records `random_playout` at roughly 134,277 games/sec against a Stage-1 budget of 500,000 games/sec. That miss is not accepted. Gate 2 must resolve it.

Required triage questions:

- Was the correct build/profile used?
- Is accidental setup cost included?
- Are unnecessary allocations dominating?
- Is validation overhead intentionally included?
- Is the WSL/noisy-host caveat hiding a real miss?
- Is the benchmark harness measuring what the foundation target means?
- Is the target unrealistic for the current correctness scope?

Gate 2 exit stance:

- Do not silently waive the target.
- If the benchmark is valid and low-risk optimizations exist, implement bounded optimization.
- If the target is unrealistic or wrong, create an accepted benchmark-doctrine adjustment or ADR candidate before changing it.
- Gate 2 is not `Done` until either `random_playout >= 500,000 games/sec` under the accepted benchmark command/environment or the target is formally recalibrated by the accepted decision mechanism.

### D7 — Minimal `tools/seed-reducer` v0

Minimum CLI:

```text
cargo run -p seed-reducer -- --game race_to_n --seed <n> --commands <stream>
cargo run -p seed-reducer -- --game race_to_n --failure-report <path>
```

Required behavior:

- Parse a simulation failure report or explicit seed/command stream. `tools/simulate` currently prints failure data only as human text (`print!`/`eprint!`); this deliverable depends on `simulate` first emitting a machine-readable failure report (file or structured stdout) that defines the `--failure-report <path>` input contract. The report fields are the ones `simulate` already computes — `seed`, `turn_index`, `actor`, `command_stream`, `state_hash`, `effect_hash`, `view_hash`, `failure_reason` — plus game/rules/data/engine versions; specify that schema before `seed-reducer` consumes it.
- Emit a normalized replay command and/or Trace Schema v1 reproducer.
- Replay the failure if enough context exists.
- Attempt simple prefix/command-stream minimization only when there is a clear failure predicate.
- If true minimization is not available, say so explicitly and preserve the exact reproducer.
- Document a future reducer plan.

No fake delta-debugging. No fuzzing framework.

### D8 — Lightweight `tools/rule-coverage`

Minimum CLI:

```text
cargo run -p rule-coverage -- --game race_to_n
```

Required checks:

- Every stable rule ID in `games/race_to_n/docs/RULES.md` appears exactly once in `RULE-COVERAGE.md`.
- No coverage row references an unknown rule ID.
- No `open`, `not started`, or empty evidence rows remain for a done official game.
- Required evidence categories are not silently blank.
- `not-applicable`, `unsupported`, and `intentionally-deferred` rows have rationale.
- Performance-deferral rows are consistent with `BENCHMARKS.md` and Gate 2 target resolution.

Non-goal: semantic proof of rule correctness.

### D9 — Minimal `tools/trace-viewer`

Minimum CLI:

```text
cargo run -p trace-viewer -- --game race_to_n --trace <path>
```

Required output:

- trace metadata
- fixture kind/purpose
- migration/update note
- command stream
- checkpoints
- expected hashes
- diagnostics
- not-applicable rationales
- expected outcome/terminal state

Optional only if safely shared with `replay-check`: replay and annotate actual state/effect/view summaries.

Non-goal: polished replay UI. Browser replay viewer belongs to Gate 3.

### D10 — CI hard-fail wiring

CI must run, at minimum:

- `cargo fmt --all --check`
- `cargo clippy --workspace --all-targets -- -D warnings`
- workspace tests
- `race_to_n` tests
- `replay-check` against all `race_to_n` golden traces
- `fixture-check` against `race_to_n` manifest, variants, traces, and fixtures
- `rule-coverage` against `race_to_n` docs
- quick simulation
- benchmark smoke
- `bench-report` threshold check
- engine boundary check
- WASM smoke
- web build
- current UI smoke
- docs link check

Benchmark thresholds must hard-fail. If full benchmark runs are too slow for every push, split smoke vs full workflows, but the Gate 2 acceptance path must include a hard-failing benchmark threshold check somewhere explicit and required.

## 5. Work breakdown

Each item below is suitable for later conversion into `templates/AGENT-TASK.md` packets. Do not start coding from this spec directly; decompose into bounded task packets first.

### WB1 — Gate 2 evidence audit and trace schema decision

- Dependencies: none.
- Target paths:
  - `specs/gate-2-trace-replay-benchmark-hardening.md`
  - `docs/TESTING-REPLAY-BENCHMARKING.md`
  - optional `docs/TRACE-SCHEMA-v1.md` if maintainers prefer a separate canonical schema doc
  - `games/race_to_n/docs/BENCHMARKS.md`
  - `games/race_to_n/docs/RULE-COVERAGE.md`
- Required work:
  - Reconfirm Gate 0 and Gate 1 are done.
  - Reconfirm Gate 2 is lowest non-done gate.
  - Record Trace Schema v1 as JSON.
  - Reconcile Trace Schema v1 field names with the golden-trace field list in `docs/TESTING-REPLAY-BENCHMARKING.md §3`. That list currently reads `action_stream`, `expected_legal_action_hashes`, and `expected_public_view_hashes for selected viewers`, whereas Trace Schema v1 uses `commands`, `expected_action_tree_hashes`, and `expected_public_view_hashes`. Update §3 (or the canonical `docs/TRACE-SCHEMA-v1.md`) so a single set of field names is authoritative; do not leave two doctrine documents naming the same surfaces differently.
  - Record legacy `.trace` migration policy.
  - Record hard-failing benchmark policy.
  - Record Stage-1 playout miss handling.
- Non-goals:
  - implementation code
  - ticket decomposition
  - engine trait design
- Forbidden changes:
  - changing foundation docs casually
  - weakening the Stage-1 performance miss into a report-only caveat
  - declaring Gate 2 done
- Acceptance evidence:
  - planned spec lands with status `Planned`
  - `specs/README.md` can list Gate 2 as `Planned` only after the spec lands
  - no foundation contradiction remains unresolved

### WB2 — `race_to_n` replay/hash support extraction or consolidation

- Dependencies: WB1.
- Target paths:
  - `games/race_to_n/src/replay_support.rs` or equivalent game-local module
  - `games/race_to_n/src/lib.rs`
  - `games/race_to_n/tests/replay_tests.rs`
  - `games/race_to_n/benches/race_to_n.rs`
  - `tools/replay-check/src/main.rs`
  - `tools/trace-viewer/src/main.rs`
- Required work:
  - Move test-only replay/hash helpers into reusable game-local support.
  - Provide canonical `race_to_n` replay evaluation for state/effect/action-tree/public-view/diagnostic/outcome surfaces.
  - Keep bot trace verification game-local.
  - Keep hash semantics stable and documented.
- Non-goals:
  - generic tabletop replay framework
  - `engine-core` game-specific trace logic
  - `game-stdlib` promotion
- Forbidden changes:
  - adding counter/target/seat/race vocabulary to `engine-core`
  - adding generic board/card/resource abstractions
  - silently changing existing hash semantics
- Acceptance evidence:
  - existing `race_to_n` replay tests pass using shared game-local support
  - `replay-check` and tests call the same canonical replay/hash path
  - no `engine-core` noun-boundary violation

### WB3 — Trace Schema v1 migration and fixture catalog

- Dependencies: WB1, WB2.
- Target paths:
  - `games/race_to_n/tests/golden_traces/*.trace.json`
  - `games/race_to_n/data/fixtures/*.trace.json`
  - legacy `*.trace` files only if retained as explicit migration fixtures
  - `docs/TESTING-REPLAY-BENCHMARKING.md` or `docs/TRACE-SCHEMA-v1.md`
  - `games/race_to_n/docs/RULE-COVERAGE.md`
- Required work:
  - Convert existing key-value traces to Trace Schema v1 JSON.
  - Resolve the duplicate trace sets. Today `games/race_to_n/tests/golden_traces/*.trace` is the only set consumed by code (`tests/replay_tests.rs` `include_str!`s it), and `games/race_to_n/data/fixtures/replay-*.trace` is an orphaned parallel copy with no source consumer. Decide one canonical trace location and either migrate the orphaned set into it or remove it; do not silently migrate both into two competing JSON sets. Whichever location survives must be the one `fixture-check` (D4/WB5) and `replay-check` (D3/WB4) target.
  - Reconcile the field-name vocabulary with `docs/TESTING-REPLAY-BENCHMARKING.md §3` (see WB1) so the migrated JSON and the doctrine list use the same canonical field names.
  - Include normal, terminal, bot-action, invalid/stale diagnostic, hidden-info not-applicable, and stochastic-game-event not-applicable coverage.
  - Add fixture catalog IDs and purposes.
  - Require migration/update notes.
  - Ensure trace files remain evidence, not behavior.
- Non-goals:
  - trace DSL
  - rule execution from trace data
  - hidden/private-info machinery for `race_to_n`
- Forbidden changes:
  - YAML
  - behavior-looking keys
  - silently changing expected hashes
- Acceptance evidence:
  - old trace coverage is preserved or expanded
  - every migrated trace includes schema/rules/engine/data versions, notes, expected hashes, and not-applicable rationales
  - legacy parser/import is explicitly marked temporary
  - malformed/legacy-unapproved traces fail fixture-check

### WB4 — Real `replay-check`

- Dependencies: WB2, WB3.
- Target paths:
  - `tools/replay-check/src/main.rs`
  - `tools/replay-check/Cargo.toml`
  - `games/race_to_n/src/replay_support.rs` or equivalent
  - `games/race_to_n/tests/replay_tests.rs`
- Required work:
  - Implement CLI parsing.
  - Support `--game race_to_n` only.
  - Support one trace, directory, and all known traces.
  - Replay through Rust behavior.
  - Compare all required expected surfaces.
  - Print loud, actionable failure output.
  - Exit non-zero on drift or malformed input.
- Non-goals:
  - multi-game registry abstraction beyond what the CLI needs
  - browser viewer
  - universal trace trait unless unavoidable
- Forbidden changes:
  - moving game-specific replay logic into `engine-core`
  - updating golden traces silently to make replay-check pass
- Acceptance evidence:
  - `cargo run -p replay-check -- --game race_to_n --all` passes on valid traces
  - an intentionally corrupted expected hash fails with non-zero exit and useful output
  - malformed trace fails with non-zero exit
  - CI uses replay-check, not only in-crate replay tests

### WB5 — Real `fixture-check`

- Dependencies: WB1, WB3.
- Target paths:
  - `tools/fixture-check/src/main.rs`
  - `tools/fixture-check/Cargo.toml`
  - `games/race_to_n/data/manifest.toml`
  - `games/race_to_n/data/variants.toml`
  - `games/race_to_n/tests/golden_traces/`
  - `games/race_to_n/data/fixtures/`
- Required work:
  - Enforce Trace Schema v1 required fields and unknown-field rejection.
  - Reject behavior-looking keys recursively.
  - Reject duplicate trace IDs.
  - Check notes and migration/update notes.
  - Check manifest/variants strictness where practical.
  - Reject YAML under fixture/report/trace contexts.
  - Report all failures with path and field context.
- Non-goals:
  - executing replay behavior
  - proving rule correctness
  - replacing Rust static-data parser tests
- Forbidden changes:
  - allowing behavior in data because it is “just a test fixture”
  - allowing silent missing notes
- Acceptance evidence:
  - valid `race_to_n` fixtures pass
  - fixtures with unknown fields fail
  - fixtures with behavior-looking fields fail
  - duplicate IDs fail
  - missing notes/migration notes fail
  - CI runs fixture-check

### WB6 — Structured benchmark output and `bench-report`

- Dependencies: WB1.
- Target paths:
  - `games/race_to_n/benches/race_to_n.rs`
  - `games/race_to_n/benches/thresholds.json` or equivalent no-YAML threshold file
  - `tools/bench-report/src/main.rs`
  - `tools/bench-report/Cargo.toml`
  - `games/race_to_n/docs/BENCHMARKS.md`
- Required work:
  - Emit benchmark JSON plus human summary from the custom harness.
  - Include required metadata and operation rows.
  - Add threshold file with rationale per threshold.
  - Implement bench-report parser and hard-fail comparison.
  - Reject missing operations and malformed reports.
- Non-goals:
  - Criterion migration by default
  - Iai-Callgrind dependency by default
  - publishing benchmark dashboards
- Forbidden changes:
  - benchmark report-only mode for required gates
  - hidden thresholds without rationale
  - silently waiving benchmark failures
- Acceptance evidence:
  - `cargo bench -p race_to_n` or documented equivalent emits valid JSON
  - `cargo run -p bench-report -- --input <report> --thresholds <thresholds>` passes valid report
  - missing operation fails
  - malformed metadata fails
  - threshold failure exits non-zero

### WB7 — Stage-1 playout budget triage and threshold decision

- Dependencies: WB6.
- Target paths:
  - `games/race_to_n/benches/race_to_n.rs`
  - `games/race_to_n/docs/BENCHMARKS.md`
  - optional `docs/adr/*` if target recalibration or benchmark doctrine changes
  - optional `docs/TESTING-REPLAY-BENCHMARKING.md` if doctrine changes
- Required work:
  - Profile or inspect `random_playout` cost.
  - Determine whether setup, allocation, validation, bot construction, or harness overhead explains the miss.
  - Apply bounded low-risk optimization if obvious.
  - Otherwise produce a benchmark target recalibration proposal through the accepted decision path.
- Non-goals:
  - broad performance rewrite
  - weakening validation correctness
  - replacing correctness with speed
- Forbidden changes:
  - hiding the miss
  - changing the target without accepted documentation
  - lowering thresholds only to make CI green
- Acceptance evidence:
  - either `random_playout` meets the accepted Stage-1 threshold or an accepted ADR/spec/doctrine adjustment records the new target and rationale
  - `BENCHMARKS.md` records the decision, environment, command, and caveats
  - bench-report hard-fails the accepted threshold

### WB8 — Minimal `seed-reducer` v0 or honest seed-reduction plan

- Dependencies: WB2, WB3, WB4.
- Target paths:
  - `tools/seed-reducer/src/main.rs`
  - `tools/seed-reducer/Cargo.toml`
  - `tools/simulate/src/main.rs`
  - `docs/TESTING-REPLAY-BENCHMARKING.md`
  - `games/race_to_n/docs/BENCHMARKS.md` only if failure reporting changes benchmark evidence
- Required work:
  - Add a machine-readable failure-report emission to `tools/simulate` (file or structured stdout) and define its schema; this is the `seed-reducer --failure-report <path>` input contract. `simulate` currently prints only human text, so this is net-new work, not a parse-only task.
  - Accept that failure report or an explicit seed/command stream.
  - Emit normalized replay-check command and/or Trace Schema v1 reproducer.
  - Replay failure when enough context is present.
  - Implement prefix minimization only when a predicate is available.
  - Explicitly state when minimization is unavailable.
  - Document future reducer plan.
- Non-goals:
  - fuzzing framework
  - randomized shrinking
  - fake delta-debugging
- Forbidden changes:
  - claiming minimization when only normalization happened
  - swallowing failure context
- Acceptance evidence:
  - injected simulation failure produces a replayable normalized reproducer
  - exact reproducer can be passed to replay-check or converted to a trace
  - no-op placeholder behavior is gone

### WB9 — Lightweight `rule-coverage`

- Dependencies: WB1.
- Target paths:
  - `tools/rule-coverage/src/main.rs`
  - `tools/rule-coverage/Cargo.toml`
  - `games/race_to_n/docs/RULES.md`
  - `games/race_to_n/docs/RULE-COVERAGE.md`
  - `games/race_to_n/docs/BENCHMARKS.md`
- Required work:
  - Parse stable rule IDs from `RULES.md`.
  - Parse coverage matrix rows from `RULE-COVERAGE.md`.
  - Check exactly-one coverage row per rule ID.
  - Reject unknown IDs and open/blank evidence rows.
  - Require rationale for `not-applicable`, `unsupported`, and `intentionally-deferred`.
  - Check Stage-1 performance deferral is resolved by WB7 before Gate 2 is done.
- Non-goals:
  - semantic analyzer
  - test coverage percentages
  - proof of implementation correctness
- Forbidden changes:
  - treating UI smoke as rule authority
  - accepting silent doc gaps
- Acceptance evidence:
  - valid `race_to_n` docs pass
  - deleted coverage row fails
  - unknown rule ID fails
  - open/blank row fails
  - unresolved Stage-1 perf deferral fails Gate 2 readiness

### WB10 — Minimal `trace-viewer`

- Dependencies: WB3; optional WB4 for replay annotation.
- Target paths:
  - `tools/trace-viewer/src/main.rs`
  - `tools/trace-viewer/Cargo.toml`
  - `games/race_to_n/src/replay_support.rs` only if annotation shares logic
- Required work:
  - Print readable trace summary.
  - Include metadata, commands, checkpoints, expected hashes, diagnostics, outcome, and migration notes.
  - Optionally replay and annotate actual hashes if it reuses replay-check safely.
- Non-goals:
  - browser replay viewer
  - polished UI
  - editing traces
- Forbidden changes:
  - making trace-viewer an authority for rule behavior
  - duplicating incompatible replay logic
- Acceptance evidence:
  - `cargo run -p trace-viewer -- --game race_to_n --trace <valid trace>` prints complete summary
  - malformed trace exits non-zero with useful error
  - output is useful enough for CI failure triage

### WB11 — CI hard-fail wiring

- Dependencies: WB4, WB5, WB6, WB7, WB8, WB9, WB10.
- Target paths:
  - `.github/workflows/ci.yml`
  - `scripts/boundary-check.sh`
  - `scripts/check-doc-links.mjs`
  - `apps/web/package.json`
  - tool crates above
- Required work:
  - Add replay-check, fixture-check, rule-coverage, benchmark report/threshold gate.
  - Keep existing fmt, clippy, tests, simulation, boundary, WASM, web, UI, docs checks.
  - Decide whether benchmark threshold gate runs in the main CI job or a required benchmark workflow.
  - Ensure benchmark threshold failures hard-fail.
- Non-goals:
  - hosted service setup
  - benchmark dashboard
  - browser replay UI
- Forbidden changes:
  - report-only threshold mode for required checks
  - weakening tests to get green CI
  - removing existing Gate 1 smoke checks
- Acceptance evidence:
  - CI fails on corrupted trace hash
  - CI fails on malformed fixture
  - CI fails on unresolved rule coverage drift
  - CI fails on benchmark threshold miss
  - CI passes on valid Gate 2 evidence set

### WB12 — Docs/index finalization and Gate 2 exit evidence

- Dependencies: WB1-WB11.
- Target paths:
  - `specs/README.md`
  - `specs/gate-2-trace-replay-benchmark-hardening.md`
  - `games/race_to_n/docs/RULE-COVERAGE.md`
  - `games/race_to_n/docs/BENCHMARKS.md`
  - `docs/TESTING-REPLAY-BENCHMARKING.md`
  - optional `docs/adr/*`
  - optional `progress.md`
- Required work:
  - Record final Gate 2 evidence commands and results.
  - Update benchmark decision and trace schema doctrine docs.
  - Flip spec status to `Done` only after exit evidence passes.
  - Keep Gate 3 admitted only after Gate 2 is done.
- Non-goals:
  - implementation tickets
  - code changes after evidence unless a failure is found
- Forbidden changes:
  - marking Gate 2 done before benchmark target resolution
  - leaving unresolved legacy trace ambiguity
  - changing foundation law casually
- Acceptance evidence:
  - closeout table with exact commands/results
  - spec index reflects `Done` only after evidence
  - next gate remains blocked until evidence passes

## 6. Exit criteria

| Exit criterion | Source alignment | Required proof |
|---|---|---|
| Trace Schema v1 is documented and used by `race_to_n` golden traces | ROADMAP Gate 2; TESTING golden traces | All active golden traces are JSON Trace Schema v1 or explicitly validated migration fixtures. |
| Legacy trace migration is explicit | TESTING drift discipline | Current key-value `.trace` files are migrated or retained only with migration status and rationale. |
| `replay-check` replays all `race_to_n` golden traces | ROADMAP Gate 2 replay checker | `cargo run -p replay-check -- --game race_to_n --all` passes. |
| Golden trace drift cannot pass silently | TESTING golden trace drift failure | Corrupted expected hash fails replay-check and CI. |
| `fixture-check` rejects malformed traces and behavior-looking fields | ENGINE-GAME-DATA-BOUNDARY; AGENT-DISCIPLINE | Negative fixture tests or scripted checks prove rejection. |
| Stable state/effect/action-tree/view hash checks remain deterministic | TESTING replay/hash | Repeated replay produces identical hashes; drift fails. |
| Diagnostics are checked where applicable | OFFICIAL-GAME-CONTRACT; TESTING | Invalid/stale trace includes expected diagnostic code/hash and replay-check verifies it. |
| Simulation failures print enough seed/command/hash/context data | ROADMAP Gate 2 | Injected failure output can be normalized into replay command/trace. |
| `seed-reducer` is no longer a no-op | ROADMAP Gate 2 seed-reduction plan | v0 emits reproducible normalized command/trace and honestly reports minimization status. |
| Benchmark JSON output exists | TESTING benchmarks | Native benchmark emits structured report with required metadata. |
| `bench-report` validates and hard-fails thresholds | User decision; ROADMAP benchmark thresholds | Malformed/missing/regressed report exits non-zero; CI uses it. |
| Stage-1 random playout budget miss is resolved | TESTING Stage-1 budget; BENCHMARKS evidence | Either threshold is met or accepted target recalibration exists. No silent waiver. |
| `rule-coverage` catches doc drift | FOUNDATIONS rule coverage requirement | Missing/unknown/open coverage rows fail. |
| `trace-viewer` gives useful summary | ROADMAP trace/replay hardening | CLI prints metadata, commands, checkpoints, hashes, diagnostics, outcome, notes. |
| CI runs hardening checks | ROADMAP Gate 2 exit | CI includes replay-check, fixture-check, rule-coverage, bench-report threshold gate, simulation, wasm/web/UI, boundary, docs. |
| No `engine-core` game/mechanic nouns introduced | FOUNDATIONS; BOUNDARY | Boundary check and code review pass. |
| No behavior moves into static data | FOUNDATIONS; BOUNDARY | Fixture/static-data checks reject behavior-looking fields. |
| No YAML or DSL appears | FOUNDATIONS; BOUNDARY | Fixture-check and repo review pass. |
| No TypeScript legality appears | FOUNDATIONS; UI-INTERACTION | UI smoke remains presentation-only; TS does not decide legality. |
| No private/licensed content appears | IP-POLICY | Trace/docs/content review passes. |
| Specs index admits next gate only after evidence | specs workflow | `specs/README.md` remains truthful. |

## 7. Acceptance evidence

Gate 2 closeout must include exact commands and results. Minimum expected command set:

| Evidence | Command or review | Required result |
|---|---|---|
| formatting | `cargo fmt --all --check` | pass |
| lint | `cargo clippy --workspace --all-targets -- -D warnings` | pass |
| workspace tests | `cargo test --workspace` | pass |
| game tests | `cargo test -p race_to_n` | pass |
| replay check | `cargo run -p replay-check -- --game race_to_n --all` | pass on valid traces; fail on intentionally corrupted hash |
| fixture check | `cargo run -p fixture-check -- --game race_to_n` | pass on valid fixtures; fail on malformed/unknown/behavior-looking fixture |
| rule coverage | `cargo run -p rule-coverage -- --game race_to_n` | pass on valid docs; fail on missing/unknown/open coverage row |
| quick simulation | `cargo run -p simulate -- --game race_to_n --games 1000` | pass; failure output remains reproducible |
| injected failure reproducer | documented `simulate` failure injection plus `seed-reducer` command | normalized replay command/trace emitted |
| benchmark smoke | documented `cargo bench -p race_to_n` command | structured JSON report produced |
| benchmark threshold gate | `cargo run -p bench-report -- --input <report> --thresholds <thresholds>` | pass valid report; hard-fail missing/malformed/regressed report |
| Stage-1 playout decision | benchmark evidence or ADR/spec adjustment | target met or formally recalibrated; no silent waiver |
| engine boundary | `bash scripts/boundary-check.sh` | pass |
| WASM smoke | `npm --prefix apps/web run smoke:wasm` | pass |
| web build | `npm --prefix apps/web run build` | pass |
| UI smoke | `npm --prefix apps/web run smoke:ui` | pass |
| docs links | `node scripts/check-doc-links.mjs` | pass |
| no YAML | repo/fixture-check review | pass |
| no private/licensed content | docs/fixture review | pass |
| not applicable evidence | trace/docs review | hidden-info/stochastic/private-view N/A rationales present |

## 8. FOUNDATIONS and boundary alignment

| Foundation principle | Gate 2 alignment |
|---|---|
| Rust owns rules, legality, validation, transitions, effects, replay, serialization, visibility, and bots | `replay-check`, `seed-reducer`, benchmarks, and traces all execute Rust behavior. TypeScript remains smoke presentation only. |
| `engine-core` is a generic contract kernel | Gate 2 may reuse `ReplayRecord`, stable hashes, command envelopes, action trees, and generic IDs; it must not add game/mechanic nouns. |
| `game-stdlib` is earned | No promotion. `race_to_n` remains first-use local-only. |
| Static data is not behavior | Trace Schema v1 records evidence only. Fixture-check rejects behavior-looking keys. |
| No YAML/DSL | JSON for traces/reports; existing TOML manifest/variant files stay typed metadata/parameters only. |
| Official games require evidence | Gate 2 turns in-crate evidence into CLIs and CI gates. |
| Bots are Rust-owned and bounded | Bot trace verification remains Level 0 random legal; no strategy/search detours. |
| UI is presentation-only | Existing web smoke remains; no new TypeScript legality. |
| Local-first | No hosted service, account, persistence, or multiplayer work. |
| Stop conditions | Any attempt to generalize engine mechanics, create a DSL, hide benchmark failures, or weaken tests stops Gate 2 work. |

### Research grounding

This section is non-authoritative; project foundation docs win. It explains the external practice behind the Gate 2 decisions.

| Topic | Research signal | Gate 2 decision |
|---|---|---|
| JSON trace format | RFC 8259 defines JSON as a lightweight, text-based, language-independent data interchange format; JSON Schema practice supports required properties and type validation. | Use structured JSON for Trace Schema v1, with required fields and strict unknown-field rejection. |
| Snapshot/approval testing | Rust `insta` documents snapshot testing, serializers, snapshot files, redactions, and CLI testing; approval testing practice captures complex outputs and confirms they have not changed. | Treat golden traces as committed evidence. Drift must be intentional, reviewed, and explained with migration/update notes. |
| Command logs and replay | Command pattern literature explicitly supports logging serializable command streams and replaying them through normal simulation. Deterministic lockstep literature emphasizes sending inputs rather than state when deterministic replay is reliable. | Trace files record command streams and expected hash surfaces; replay-check executes commands through Rust behavior rather than interpreting trace data as rules. |
| Wall-clock benchmarks | Criterion.rs is statistics-driven and documents warmup, measurement, comparison, outlier/noise handling, and the need for quiet machines where possible. | Keep wall-clock harness for Gate 2 but require conservative thresholds, metadata, and caveats. |
| CI noise | GitHub-hosted runners are fresh hosted VMs/containers with specified resources; shared hosted environments are not publication-grade benchmark labs. | CI benchmark gates must hard-fail, but thresholds must be conservative and explicitly justified. |
| Instruction-count alternatives | Iai-Callgrind uses Valgrind Callgrind and is designed for accurate, consistent measurements in CI-like virtualized environments. | Record as future option if wall-clock thresholds prove unstable; do not add the dependency in Gate 2 by default. |

References:

- RFC 8259, “The JavaScript Object Notation (JSON) Data Interchange Format”: https://www.rfc-editor.org/rfc/rfc8259
- JSON Schema getting-started documentation: https://json-schema.org/learn/getting-started-step-by-step
- Insta snapshot testing docs: https://insta.rs/docs/
- ApprovalTests overview: https://approvaltests.com/
- Game Programming Patterns, “Command”: https://gameprogrammingpatterns.com/command.html
- Gaffer on Games, “Deterministic Lockstep”: https://gafferongames.com/post/deterministic_lockstep/
- Criterion.rs documentation: https://bheisler.github.io/criterion.rs/book/
- Criterion.rs analysis process: https://bheisler.github.io/criterion.rs/book/analysis.html
- Iai-Callgrind docs.rs: https://docs.rs/iai-callgrind/latest/iai_callgrind/
- GitHub-hosted runners reference: https://docs.github.com/en/actions/reference/runners/github-hosted-runners

## 9. Forbidden changes

The following are explicitly forbidden in Gate 2:

- adding game/mechanic nouns to `engine-core`
- adding generic board/card/resource abstractions
- promoting anything to `game-stdlib`
- creating a DSL
- using YAML
- moving behavior into static data
- adding TypeScript legality
- broadening Gate 2 into the Gate 3 web shell
- starting `three_marks` or any other next game
- adding hosted multiplayer/accounts/persistence
- adding MCTS, ISMCTS, Monte Carlo, ML, or RL bots
- weakening tests to get green CI
- silently updating golden traces
- silently waiving benchmark failures
- hiding benchmark target misses
- copying proprietary rules, assets, fonts, or trade dress
- using GitHub code search or snippets as evidence
- cloning the repository for this work
- fetching implementation files from a branch name after the exact commit SHA is known

## 10. Documentation updates required

| Document | Required update |
|---|---|
| `specs/README.md` | Add Gate 2 spec row as `Planned` when this spec lands. Flip to `Done` only after all exit evidence passes. |
| `specs/gate-2-trace-replay-benchmark-hardening.md` | Add closeout evidence when implemented; do not mark done in this planned version. |
| `docs/TESTING-REPLAY-BENCHMARKING.md` | Add Trace Schema v1 doctrine or link to a single canonical Trace Schema v1 doc. Add benchmark hard-fail doctrine if not already clear enough. |
| `docs/TRACE-SCHEMA-v1.md` | Optional. Use only if maintainers prefer a separate canonical schema doc. If created, link it from TESTING; do not create two conflicting schema authorities. |
| `games/race_to_n/docs/RULE-COVERAGE.md` | Update evidence rows for replay-check, fixture-check, rule-coverage, trace-viewer, bench-report, seed-reducer, and Stage-1 budget resolution. |
| `games/race_to_n/docs/BENCHMARKS.md` | Record structured benchmark report format, thresholds, CI caveats, current values, Stage-1 triage decision, and accepted benchmark command. |
| `games/race_to_n/docs/UI.md` | Update only if existing web smoke/replay surfaces change. Otherwise mark not applicable. |
| `docs/adr/*` | Required only if benchmark target recalibration, trace schema policy that changes foundation doctrine, hash semantics, or engine contract changes require ADR. Otherwise not applicable. |
| `progress.md` | Update only if repository practice expects implementation progress notes. Current file records Gate 1 ticket progress; Gate 2 may instead use spec closeout. |
| `docs/MECHANIC-ATLAS.md` | not applicable unless Gate 2 accidentally creates mechanic extraction pressure; it should not. |
| `templates/*` | not applicable unless the implementation discovers a template gap. Do not alter templates casually. |

## 11. Sequencing

1. Land this spec as `Planned`.
2. Decompose WB1-WB12 into AGENT-TASK packets.
3. Complete WB1 before trace/tool implementation so the schema decision is fixed.
4. Complete WB2 before replay-check, trace-viewer, and seed-reducer to avoid duplicated replay logic.
5. Complete WB3 before fixture-check and replay-check CI wiring.
6. Complete WB4-WB5 before CI hard-fail wiring.
7. Complete WB6 before WB7 so the Stage-1 miss is triaged against structured benchmark output.
8. Complete WB8-WB10 before WB11 so CI has real tools to run.
9. Complete WB11 before WB12.
10. WB12 flips Gate 2 to `Done` only after all exit criteria and acceptance evidence pass.
11. Gate 3 remains blocked until Gate 2 is `Done`.

## 12. Assumptions

| Assumption | Status | Correction path |
|---|---|---|
| `b2038fa72d1695b493887b404c92c915a1ff2e6c` is the intended commit for this spec | verified at reassessment: local `main` HEAD equals this commit | Resolved. If landing from a different checkout, re-verify `main` HEAD; if it differs, record the mismatch and decide whether to rebase the spec. |
| `race_to_n` is the only Gate 2 game | accepted | If another game exists by implementation time, Gate 2 still hardens `race_to_n` first; expansion requires explicit scope update. |
| Trace Schema v1 should be JSON | accepted | Change only through explicit spec/ADR-level decision; do not revert to ad hoc key-value traces. |
| Legacy `.trace` parser is temporary | accepted | Remove or quarantine after migration unless a documented migration fixture reason remains. |
| Custom benchmark harness remains for Gate 2 | accepted | Switch to Criterion/Iai-Callgrind only if implementation evidence proves the current harness cannot support hard-failing gates. |
| CI benchmark gates hard-fail | accepted by user decision | No report-only substitute for required thresholds. |
| Stage-1 random playout target remains binding until formally changed | accepted | Either hit 500,000 games/sec or land accepted recalibration. |
| `seed-reducer` v0 may be a normalized reproducer rather than true minimizer | accepted | It must say when minimization is unavailable and document the future plan. |
| `rule-coverage` is structural only | accepted | Semantic proof remains tests/traces/replay, not doc parsing. |
| `trace-viewer` is CLI-only in Gate 2 | accepted | Browser replay viewer belongs to Gate 3. |

## Outcome placeholder

Status remains `Planned`.

Do not add a `Done` outcome section until Gate 2 implementation passes all exit criteria and acceptance evidence.

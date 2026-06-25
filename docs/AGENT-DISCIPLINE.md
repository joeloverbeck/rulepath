# Rulepath Agent Discipline

Status: operational law for coding agents and AI-assisted documentation work.

Claude Code, Codex, and similar agents are accelerators. They are not unattended architects. Agent work MUST be bounded, testable, source-grounded, explicit about forbidden changes, and delivered as complete files or coherent complete sections, not diffs.

## 1. Agent role

Agents MAY:

- implement bounded game-module behavior in typed Rust;
- add tests, traces, simulations, and benchmarks;
- write or update per-game documentation from verified rules and implemented behavior;
- build UI components against stable Rust/WASM contracts;
- improve diagnostics, replay tooling, benchmark reports, or fixture validation;
- refactor narrow code after the mechanic atlas permits it.

Agents MUST NOT:

- invent major architecture without ADR;
- generalize from one game;
- add game/mechanic nouns to `engine-core`;
- make static data procedural;
- add YAML by convenience;
- create a DSL;
- put legality in TypeScript;
- let bots inspect hidden information unavailable to their seat;
- add private licensed content to public files;
- delete or weaken tests to get green output;
- optimize without benchmark evidence;
- produce sprawling unreviewable changes.

## 2. Required task packet

Every agent task SHOULD state:

```text
Context: relevant foundation docs, ADRs, game docs, source notes
Target: exact crate/game/doc/tool being changed
Stage/gate: roadmap stage and mechanics being proved
Goal: observable behavior or document outcome
Non-goals: areas not to touch
Forbidden changes: kernel, data format, DSL, UI, bot, IP, primitive boundaries
Sources/docs: rule sources, source notes, coverage rows, ADRs
Tests: required unit/rule/golden/property/simulation/replay/visibility/bot/UI tests
Benchmarks: baseline or non-regression evidence
Docs: required updates
Output format: complete files or coherent complete sections, not diffs
Handoff: commands run, results, unresolved questions
```

A task without forbidden changes invites architecture drift.

## 3. Good and bad tasks

Good tasks are bounded:

```text
Implement Stage 3 `column_four` legal action generation, validation, semantic effects, golden traces, Level 1 bot legality tests, and per-game docs. Do not modify `engine-core`. Update the mechanic inventory and atlas notes. Output complete files or coherent sections, not diffs.
```

```text
Add viewer-filtered effect-log tests for `high_card_duel`. First decide whether failing tests are still valid, then whether the issue is the SUT or tests. Add regression coverage for any bug found.
```

```text
Generalize the simulator summary map from fixed `seat_0`/`seat_1` winner
fields to a deterministic winner-id map. Do not change game rules, replay
hashes, or browser presentation. Add machine-output tests for 2-seat and
3-seat fixtures.
```

```text
Add a 3-seat setup fixture for one named game and verify Rust-owned wrong-seat
diagnostics. Do not add a generic multiplayer setup system or TypeScript
legality.
```

```text
Add a pairwise no-leak matrix harness for one hidden-information game. Name the
source seat, viewer seat, and checked surfaces. Do not expose internal full
traces to browser exports.
```

```text
Add Rust-owned showdown rationale for a Hold'Em-family game: evaluated result,
comparison vector, decisive reason, and split/tie explanation. TypeScript may
render the payload but must not compute winners or hand strength.
```

```text
Add a multi-seat seat-rail UI fed by the Rust view. It may display active seats
and pending responders, but it must not infer turn order or legal actors.
```

Bad tasks are vague or architecture-seeking:

- “make the engine support any board game”;
- “generalize this”;
- “add YAML support for convenience”;
- “finish the bot”;
- “make the UI feel good”;
- “fix all tests” without validating test intent;
- “add MCTS”;
- “implement multiplayer”;
- “clean up the architecture”;
- “port a private licensed game into the public repo.”

## 4. Failing-test protocol

When tests fail, agents and humans MUST:

1. determine whether the failing tests are still valid;
2. determine whether the issue is in the system under test or in the test suite;
3. fix the issue;
4. add or update regression coverage;
5. report changes.

Tests MUST NOT be deleted, weakened, renamed away, or rewritten merely to get green output.

## 5. Kernel-change protocol

Any `engine-core` change MUST answer:

1. Which already implemented official games require this?
2. Why can the change not live inside `games/*`?
3. Why can the change not live inside `game-stdlib` after earned pressure?
4. Does it introduce any game noun, mechanic noun, strategy, renderer concern, network concern, or storage concern?
5. Does it preserve deterministic replay, visibility boundaries, serialization compatibility, and hashes?
6. Does it require ADR?

Default answer: do not change `engine-core`.

## 6. Data protocol

Agents MUST use approved v1/v2 formats only:

- TOML for manifests and simple configuration;
- JSON for traces, replay interchange, fixtures, and machine reports;
- RON for Rust-shaped fixtures and enum-heavy typed content;
- CSV for tables;
- Postcard or equivalent compact Serde formats only for approved non-hand-authored internal artifacts.

Agents MUST reject unknown fields by default and flag behavior-looking fields such as `when`, `if`, `then`, `else`, `selector`, `condition`, `trigger`, `script`, `loop`, `foreach`, `priority_expression`, `ai_condition`, `effect_script`, `rule`, and `requires`.

Agents MUST NOT use “data-driven rules” approvingly.

## 7. DSL protocol

Agents MUST NOT create a DSL.

A future DSL requires ADR and must address typed semantics, lowering/compilation, source spans, formatting, linting, versioning, tests, benchmarks, examples, anti-examples, determinism, replay/hash implications, visibility, hidden defaults, agent safety, and migration.

A DSL MUST NOT be introduced to rescue one complex game or private experiment.

## 8. Primitive-pressure protocol

Agents MUST update per-game mechanic inventories and the repo-level atlas when mechanics repeat.

Hard gate: a third official game with the same mechanic shape MUST NOT proceed until the primitive-pressure ledger says one of:

- reuse an existing primitive;
- promote a narrow typed helper;
- explicitly defer/reject with rationale;
- require ADR.

Agents MUST NOT “clean up” repeated mechanics into `engine-core`. Earned helpers belong in `game-stdlib`, and only after the atlas process permits them.

After a helper is promoted, agents MUST treat matching local implementations in earlier official games as conformance work, not optional cleanup. A spec that advances the mechanic ladder while promotion debt is open is invalid unless the atlas records an accepted exception or ADR. Same-gate deferral must name the games, primitive, evidence, risk, and closure gate. Agents must preserve behavior by default during conformance repair and must not update traces, hashes, diagnostics, effect order, UI surfaces, or bot behavior unless the spec explicitly authorizes the migration and explains why.

## 8A. Scaffold-refactor protocol

Shared-scaffolding and hash-sensitive refactors MUST be bounded tasks, not
open-ended cleanup. Before starting, the task packet MUST name the accepted
authority for the lane, usually
[ADR 0008](adr/0008-mechanical-scaffolding-governance.md) and the
[MECHANICAL-SCAFFOLDING-REGISTER.md](MECHANICAL-SCAFFOLDING-REGISTER.md), plus
the exact duplicate sites, affected hashes, visibility impact, migration set,
and forbidden behavior changes.

For a scaffold-refactor task, agents MUST:

1. validate whether any failing tests are still valid;
2. classify failures as system-under-test, test-suite, environment, or stale
   expectation before editing;
3. inventory exact adopters, current trace/hash surfaces, fixture profiles, and
   no-leak matrices;
4. freeze public behavior by adding or naming characterization tests before the
   helper migration;
5. implement the narrow helper without adding mechanic nouns, rule policy,
   reveal policy, scoring, strategy, renderer policy, YAML, or a DSL;
6. migrate one reference game or tool first and compare traces, hashes,
   serialization, diagnostics, no-leak evidence, and public/browser payloads;
7. migrate remaining matching sites only when the comparison proves semantic
   identity, or record an accepted exception/defer/reject decision in the
   governing register;
8. record any intentional trace/hash/schema migration under the accepted ADR or
   ticket that authorized it.

Agents MUST NOT use "update all goldens", broad formatting churn, or "tests
pass" alone as scaffold-refactor acceptance. If the refactor changes legality,
visibility authorization, scoring, hidden-state meaning, effect semantics,
replay bytes, or public UI behavior without explicit migration authority, stop
and reassess.

## 8B. New-game scaffolding reuse-and-track protocol

This protocol applies to every bounded task that creates or extends an official
game's Rust-owned production, bridge, test, replay, serialization, or evidence
plumbing. It applies even when `Task profile` is not `scaffold-refactor`.

Before implementing such plumbing, agents MUST:

1. read the relevant entries in `MECHANICAL-SCAFFOLDING-REGISTER.md` and inspect
   the accepted shared home named by those entries;
2. complete the task packet's reuse-first audit fields;
3. reuse a matching promoted helper, or link the accepted register exception
   before writing a parallel local shape; and
4. identify any genuinely new behavior-free scaffolding and any prior official
   games likely to contain the same shape.

During implementation, agents MUST keep adapters narrow. An adapter may translate
game-local types into a generic accepted API; it MUST NOT recreate generic seat
syntax, effect-envelope construction, action-tree framing, stable-byte framing,
visibility geometry, or evidence-profile driving behind a different name.

Before closeout, agents MUST:

1. update `GAME-EVIDENCE.md` with the reuse and new-scaffolding receipt;
2. add or update the governing register entry for every new shape;
3. update the machine scaffolding-audit record;
4. name the prior-game migration set; and
5. add the required follow-on unit to `specs/README.md`, or link the accepted
   `local-only`, `deferred`, or `rejected` disposition.

Agents MUST stop and reassess if the proposed scaffolding decides legality,
scoring, reveal, turn, trick, team, graph, accounting, reaction, outcome,
strategy, effect meaning, renderer policy, or hidden-state policy. Such work is
behavioral and does not belong in this lane.

Agents MUST NOT use a local wrapper, renamed copy, blanket `allow` list, skipped
CI job, or broad golden update to evade this protocol. Any byte, hash, fixture,
RNG, export, or visibility migration requires the authority and evidence
required by ADR 0009.

## 9. Bot protocol

Agents implementing bots MUST:

- use the same legal action API as humans;
- choose legal action paths through normal validation;
- use only the allowed view for that seat;
- implement deterministic tie-breaking;
- avoid omniscient state, hidden-state shortcuts, and weight soup;
- add legality tests over many seeds;
- add determinism tests, explanation examples, and latency benchmarks;
- add no-leak tests for hidden-information games;
- provide/update a strategy evidence pack before Level 2 authored policy bots.

Public v1/v2 agents MUST NOT add MCTS, ISMCTS, Monte Carlo bots, ML, or RL.

## 10. UI protocol

Agents implementing UI MUST:

- build controls from Rust legal action trees;
- request Rust previews for partial/compound actions;
- submit action paths with freshness tokens;
- animate from viewer-filtered semantic effects;
- settle visual state to the latest public view;
- keep debug panels behind a dev toggle and viewer-safe data source;
- protect hidden information in DOM, logs, local storage, payloads, test IDs, and replay exports;
- support reduced motion when animations are added;
- avoid proprietary mimicry and debug-console-first layouts.

TypeScript MUST NOT decide legality.

## 11. IP protocol

Agents MUST:

- write original rules summaries;
- cite consulted rules sources in source notes;
- avoid copied rulebook prose, proprietary card text, proprietary assets, screenshots, scans, font files without verified redistribution rights, and trade dress;
- use neutral names when commercial trademark or presentation risk exists;
- keep private licensed experiments out of public files, public CI, public docs, public traces, public bundles, and public WASM/JS;
- leave a human/legal review note when unsure.

If content ships to an unauthorized browser, it has shipped.

## 12. Output protocol

Agents SHOULD output complete files or coherent complete sections. Diffs are not the primary deliverable.

For substantial work, report:

- files changed;
- tests added or updated;
- traces added or intentionally changed;
- simulations and benchmarks run;
- docs updated;
- boundary decisions;
- unresolved questions;
- commands a human should run.

## 13. Review check

Before accepting agent work, verify:

- the task stayed inside scope;
- forbidden changes were not made;
- open promotion debt was either closed, explicitly out of scope by accepted exception, or not relevant;
- failing-test protocol was followed;
- Rust remains behavior authority;
- TypeScript remains presentation-only;
- `engine-core` remains noun-free;
- static data remains typed content/parameters only;
- replay/hash determinism is preserved;
- hidden information is safe;
- bots are fair and explainable;
- tests/traces/simulations/benchmarks/docs were updated;
- IP boundaries are preserved;
- output is reviewable without reconstructing patches.
- every new-game task completed the scaffolding reuse-first audit;
- known promoted scaffolding was reused or covered by an accepted register exception;
- every new behavior-free scaffolding shape and prior-game migration set is registered;
- every required follow-on refactor unit is queued or explicitly disposed by an accepted register decision.

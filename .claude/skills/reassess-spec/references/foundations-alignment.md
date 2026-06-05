# FOUNDATIONS.md Alignment Check (Step 4)

`docs/FOUNDATIONS.md` is the Rulepath constitution. Its principles are §1 Priority order, §2 Behavior authority, §3 `engine-core` is a contract kernel, §4 `game-stdlib` is earned, §5 Static data is typed content not behavior, §6 Official games are evidence-heavy, §7 Public UI is central product work, §8 Public bots are product opponents, §9 Local-first v1/v2, §10 IP conservatism, §11 Universal acceptance invariants, §12 Stop conditions, §13 ADR triggers.

## 4.0 Internal Contradictions

Before checking FOUNDATIONS, scan for contradictions between the spec's Objective, Scope (in scope / out of scope / not allowed), Deliverables, Work breakdown, FOUNDATIONS & boundary alignment section, Exit criteria, and Forbidden changes. If the spec includes a table that classifies state (e.g. "in scope vs. out of scope", "required vs. not-applicable evidence", "allowed vs. forbidden change"), verify consistency across sections. A deliverable that contradicts the spec's own Out-of-scope, Not-allowed, or Forbidden-changes entry is a CRITICAL Issue.

## 4.1 Alignment Section Verification

Rulepath specs carry a **FOUNDATIONS & boundary alignment** section (see `gate-0-repository-skeleton.md` §7) as a table of `Principle | Stance | Rationale` rows. Verify each entry:

- Principle names must match `docs/FOUNDATIONS.md` headings (e.g. `§3 engine-core is a contract kernel`, `§11 Universal acceptance invariants`, `§12 Stop conditions`).
- Each rationale must be specific — a bare `aligns` without a named mechanism (which deliverable, which boundary, which invariant) is a MEDIUM Improvement finding.
- For deliverables touching behavior authority, the kernel boundary, validation/acceptance invariants, visibility, or replay/hash, the rationale must name the mechanism (e.g. "boundary review confirms `engine-core` declares only generic contracts; no mechanic noun enters the kernel").
- The §12 stop-condition row should be present and read `clear` (or name the specific stop conditions kept clear). A stop condition the spec actually crosses is a CRITICAL Issue, not a `clear` row.

**No alignment section at all**: do not treat absence as CRITICAL by default. Surface adding one as an Addition (MEDIUM) for specs whose deliverables don't touch behavior-authority / kernel-boundary / acceptance-invariant semantics. Escalate to a HIGH Issue only when a deliverable that *does* touch those semantics ships with no alignment statement anywhere — there the omission is a grounding gap.

## 4.2 Missing Principles

Identify FOUNDATIONS principles the spec should address but doesn't. Pay particular attention to:

- **§2 Behavior authority** — specs whose deliverables touch setup, legal-action generation, validation, state transitions, scoring/terminal detection, RNG, semantic effects, view projection, replay/hash, serialization, or bot decisions must keep all of these in Rust and keep TypeScript presentation-only. A deliverable letting TypeScript decide legality or invent rule state is a CRITICAL Issue (§2, §12 "TypeScript decides legality").
- **§3 `engine-core` is a contract kernel** — specs touching `engine-core` must keep it free of mechanic/domain nouns (`board`, `card`, `deck`, `grid`, `suit`, `capture`, `auction`, etc.); a mechanic noun in the kernel is a boundary-failure Issue (§3, §12 "`engine-core` gains game/mechanic nouns"). Typed mechanic nouns belong in `games/*` first.
- **§4 `game-stdlib` is earned** — specs promoting a helper into `game-stdlib` must cite mechanic-atlas / primitive-pressure evidence; the third official use is a hard gate (`docs/MECHANIC-ATLAS.md`). Unearned or speculative promotion is a HIGH Issue.
- **§5 Static data is not behavior** — specs introducing static data must keep it typed content/parameters/metadata/fixtures/traces only. Behavior-looking fields (selectors, rule branches, loops, triggers, conditional effects, arbitrary expressions) are blocked or escalated; YAML or DSL without an ADR is a §12 stop condition (CRITICAL).
- **§11 Universal acceptance invariants** — the master checklist every substantial change/game/ADR must satisfy. See 4.3.

## 4.3 §11 Acceptance-Invariant Check

`docs/FOUNDATIONS.md §11` lists the universal acceptance invariants. For each invariant the spec's deliverables engage, verify the spec upholds it. The load-bearing ones for spec reassessment:

- **Rust remains behavior authority; TypeScript does not decide legality** — no deliverable moves legality, validation, or rule state into TS. Violation: CRITICAL.
- **`engine-core` generic-only; `game-stdlib` earned** — see 4.2 §3/§4. Violation: CRITICAL / HIGH.
- **Unknown fields rejected by default; behavior-looking fields blocked or escalated** — specs proposing hand-authored data schemas must reject unknown fields and refuse behavior-looking ones. Missing fail-closed handling is a HIGH Issue.
- **Validation is fail-closed and blocking; warnings are distinguished from blockers** — specs proposing validation must name what failing means (block? warn?) and keep hard failures non-overridable in v1/v2. Conflating warnings with blockers, or a user-overridable hard failure, is a §11/§12 violation. Unaddressed second-order effects are Improvement findings at minimum.
- **Replay, hashes, serialization order, RNG, and traces remain deterministic** — see the determinism sub-check below. A change to replay/hash semantics also trips a §13 ADR trigger.
- **Public/private views are viewer-safe; hidden information does not leak** — for deliverables touching view projection, previews, effect logs, bot explanations, candidate rankings, UI test IDs, or replay exports, verify no path lets hidden information reach a viewer the deterministic views forbid. Leakage is CRITICAL (§11 no-leak firewall, §12 "hidden information reaches browser payloads…").
- **Bots use the normal legal action API and allowed views only; v1/v2 exclude MCTS/ISMCTS/Monte Carlo/ML/RL** — bot-touching deliverables must route through the legal action API, mutate no state directly, and use no hidden state; introducing a search/learning bot class without an ADR is a §13 trigger and §8 violation. CRITICAL.
- **Semantic effects drive animation; renderer diffs are diagnostics only** — UI/animation deliverables must drive animation from Rust-emitted semantic effects. Animation depending on guessed state diffs is a §12 stop condition.
- **Evidence coverage** — substantial changes/games must carry tests, traces, simulations, benchmarks, docs, and source notes (§6, §11). A game spec missing these is an Issue (see 3.11 in codebase-validation.md and `docs/OFFICIAL-GAME-CONTRACT.md`).

Record each violation with the specific principle and invariant. Cite the section heading exactly (`§11 Universal acceptance invariants`, `§3 engine-core is a contract kernel`). Bare citations (`FOUNDATIONS violation`) without principle names force Step 7's pre-apply verification to disambiguate.

**Determinism sub-check**: if the spec's deliverables or verification steps claim determinism, byte-identical replay, or stable hashing, verify against §2 (deterministic randomness, replay/hash behavior) and §11:

- Deterministic RNG only — the engine's declared deterministic RNG contract, never wall-clock seeding or `std::time` in canonical/replayed forms.
- Stable serialization/iteration order — rely on the serialization-boundary contract and sorted/insertion-ordered collections, not incidental hash-map iteration order.
- No nondeterministic inputs in canonical forms (wall-clock timestamps, thread-scheduling-dependent ordering) unless the spec explicitly separates a captured-at value (allowed) from a canonical-form input (forbidden).

Flag determinism violations as HIGH Issues citing §11 / §2; if the deliverable *changes* replay/hash semantics, also flag the §13 ADR trigger.

## 4.4 §12 Stop-Condition Check

`docs/FOUNDATIONS.md §12` lists stop conditions — "stop and reassess before continuing." For a spec being prepared for decomposition, a deliverable that *would cross* a stop condition is a CRITICAL Issue (continuing through a stop condition is architectural debt). Check each engaged condition against the spec:

- `engine-core` gains game/mechanic nouns;
- static files start acting procedural; "data-driven rules" approved for v1/v2; YAML or DSL appears without ADR;
- TypeScript decides legality; normal-mode illegal moves become clickable; animation depends on guessed state diffs instead of Rust effects;
- hidden information reaches browser payloads, DOM, local storage, logs, previews, bot explanations, candidate rankings, or replay exports;
- a bot bypasses legal action APIs or uses unauthorized hidden state;
- a third repeated mechanic proceeds without a ledger decision;
- official games lack docs, traces, simulations, benchmarks, rule coverage, replay, or serialization tests;
- public UI becomes debug-first;
- private licensed content enters public files/CI/docs/traces/bundles/WASM/JS; private monster-game work shapes public architecture;
- agents are asked to "generalize," "clean up the engine," or "fix all tests" without bounded scope and forbidden changes.

Present in Step 6 as a `### §12 Stop-Condition Check` section: one line per engaged condition, each `clear | N/A | flag (reason)`. Omit the section when no stop condition is engaged.

## 4.5 §13 ADR-Trigger Check

`docs/FOUNDATIONS.md §13` lists architecture-changing decisions that REQUIRE an ADR. When a spec's deliverable makes one of these decisions without citing an accepted ADR (under `docs/adr/`), flag it as a HIGH Issue (the spec needs the ADR first, or must drop the decision). Triggers include: changing the priority order or v1/v2 local-first scope; adding hosted multiplayer/accounts/server persistence; changing `engine-core` vocabulary; promoting mechanics outside the primitive-pressure path; introducing YAML / a new data format / selectors / expressions / DSL; changing replay/hash semantics; changing public/private visibility contracts; introducing public MCTS/ISMCTS/Monte Carlo/ML/RL bots; replacing React + SVG without profiling evidence; allowing private licensed experiments to influence public architecture.

# Cuba Libre Private Game Foundation-Readiness Change Plan

**Advisory deliverable:** one change-plan document. It does not replace repository law and does not implement any game code, specification, template rewrite, or finished ADR.

**Target repository:** `joeloverbeck/rulepath`  
**Target commit:** `142ddfae2be3ae2d7c861ab65f2c786a49de54ac` (`142ddfa`)  
**Freshness claim:** user-supplied target commit only. This plan does **not** independently verify that the commit is current `main`.  
**Target private game:** *Cuba Libre*, a private licensed GMT COIN-series game, handled as private licensed IP.  
**Private-IP rule for this plan:** the uploaded rules/playbook PDFs were used only to identify mechanical pressure. This plan does not reproduce their prose, diagrams, art, flowchart text, card text, or trade dress.

## Provenance notice

Requested repository: `joeloverbeck/rulepath`  
Target commit: `142ddfae2be3ae2d7c861ab65f2c786a49de54ac`  
Freshness claim: user-supplied target commit only; not independently verified as latest `main`  
Manifest role: path inventory only  
Repository metadata used: no  
Default-branch lookup used: no  
Branch-name file fetch used: no  
Target-repository code search used: no  
Clone used: no  
URL fetch method: exact raw GitHub URL fetches through the web tool  
Requested file count: 67  
Successfully verified file count: 67  
Fetch-provenance contamination observed: no  
Foreign-repository references inside fetched file contents: permitted; not a provenance check  
Connector/tool namespace trusted as evidence: no  
External research lane: separate from repository evidence

The complete exact URL ledger is in [Appendix A](#appendix-a--exact-commit-repository-fetch-ledger).

---

## 1. Executive summary

Rulepath can support a first private licensed monster game now, but only if the repository stops treating “private” as merely a late public-ladder tail and instead defines a sanctioned, quarantined private lane. The private lane must be allowed to start in parallel with the unfinished public ladder, yet remain unable to leak licensed content into public source, public bundles, public CI artifacts, public docs, public traces, or `engine-core`.

The highest-leverage moves are:

1. **Accept a sanctioned-private-lane ADR before any Cuba Libre spec work.** This ADR limits the Gate-P-tail timing rule in [ADR 0007](https://raw.githubusercontent.com/joeloverbeck/rulepath/142ddfae2be3ae2d7c861ab65f2c786a49de54ac/docs/adr/0007-next-public-scaling-phase-and-gate-p-tail.md) and amends [FOUNDATIONS.md](https://raw.githubusercontent.com/joeloverbeck/rulepath/142ddfae2be3ae2d7c861ab65f2c786a49de54ac/docs/FOUNDATIONS.md), [ROADMAP.md](https://raw.githubusercontent.com/joeloverbeck/rulepath/142ddfae2be3ae2d7c861ab65f2c786a49de54ac/docs/ROADMAP.md), and [IP-POLICY.md](https://raw.githubusercontent.com/joeloverbeck/rulepath/142ddfae2be3ae2d7c861ab65f2c786a49de54ac/docs/IP-POLICY.md). It authorizes private licensed work now, but only in a private repository/build lane whose outputs never become public architecture or public assets.
2. **Adopt a constrained typed Rust event-card mechanism under a new ADR.** The uploaded PDFs show an event deck whose resolution includes faction-order gating, dual-use event branches, persistent effects, temporary effects, free operations, rule-overriding card effects, and propaganda-round interactions. The right boundary is not “static data becomes smart.” It is: static typed content may name card identity, ordering, inert presentation metadata, and non-behavioral parameters; every selector, condition, trigger, rule override, target choice, legality hook, and state transition remains Rust behavior. This is a typed Rust registry and effect trait/match pattern, not YAML, not a scripting language, and not an untyped DSL.
3. **Default to a separate private repository with a pinned public Rulepath dependency/checkout and a private web/WASM overlay.** A public submodule or optional dependency that names a private game in the public tree is too leaky. The normal web app may show private games only when built from the private repository’s private WASM/catalog/renderer overlay. The public build must not contain even dormant private IDs, docs, rules snippets, e2e names, or private cargo dependencies.
4. **Add a private milestone profile instead of pretending milestone 1 is a public official release.** Milestone 1 should require full four-faction hotseat, all operations and special activities, propaganda rounds, victory/terminal detection, all event cards resolved, replay/fixtures/no-leak evidence, and private CI. It should not require competent AI. A Level-0 random-legal bot may be deferred until the private game seeks “private release candidate” or “official/private done” status.
5. **Scale the templates without copying licensed material.** The current templates have the right bones, but they need private-source fields, per-faction sections, event-card coverage, private-release readiness, 4-faction no-leak matrices, action-tree/surface budgets, and explicit AI deferral/future-AI source rules.

This plan preserves the non-negotiables: Rust owns behavior; `engine-core` stays noun-free; no untyped DSL/YAML; hidden information is viewer-filtered; determinism and trace/hash discipline remain; no private licensed IP enters public surfaces; v1/v2 bots still exclude MCTS/ISMCTS/Monte Carlo/ML/RL.

---

## 2. Method & evidence base

### 2.1 Repository baseline and acquisition

The repository baseline is commit `142ddfae2be3ae2d7c861ab65f2c786a49de54ac`. The uploaded `manifest_2026-06-28_142ddfa.txt` was used only as the authoritative file inventory. Every repository file used for a repository-state claim was fetched from the exact raw URL form:

```text
https://raw.githubusercontent.com/joeloverbeck/rulepath/142ddfae2be3ae2d7c861ab65f2c786a49de54ac/<manifest path>
```

No branch fetch, clone, repository search, default-branch lookup, or connector repository metadata was used. All selected paths were present in the manifest. No read-list path failed acquisition.

### 2.2 Repository files read

Read-in-full or load-bearing files included the authority-ordered foundation documents in `docs/**`, all ADRs 0001-0009 plus the ADR template, every file in `templates/**`, `specs/README.md`, the prior change-plan report and prior brief in `reports/`, and the VCS/CI/catalog seams named by the prompt. Appendix A lists every exact URL.

The most load-bearing repository findings were:

- [FOUNDATIONS.md](https://raw.githubusercontent.com/joeloverbeck/rulepath/142ddfae2be3ae2d7c861ab65f2c786a49de54ac/docs/FOUNDATIONS.md) currently prioritizes public, IP-safe games over private stress tests; says Rust owns all behavior; forbids selectors, branches, triggers, conditional effects, and DSL behavior in static data without ADR; and treats private licensed work as late, isolated, optional, non-public, and unable to shape public architecture.
- [ROADMAP.md](https://raw.githubusercontent.com/joeloverbeck/rulepath/142ddfae2be3ae2d7c861ab65f2c786a49de54ac/docs/ROADMAP.md), [ADR 0007](https://raw.githubusercontent.com/joeloverbeck/rulepath/142ddfae2be3ae2d7c861ab65f2c786a49de54ac/docs/adr/0007-next-public-scaling-phase-and-gate-p-tail.md), and [specs/README.md](https://raw.githubusercontent.com/joeloverbeck/rulepath/142ddfae2be3ae2d7c861ab65f2c786a49de54ac/specs/README.md) place Gate P after the public scaling ladder and describe it as private, optional, isolated, and non-architectural.
- [IP-POLICY.md](https://raw.githubusercontent.com/joeloverbeck/rulepath/142ddfae2be3ae2d7c861ab65f2c786a49de54ac/docs/IP-POLICY.md) already states the core public/private split and the shipment rule: if protected content reaches an unauthorized browser, it has shipped.
- [ENGINE-GAME-DATA-BOUNDARY.md](https://raw.githubusercontent.com/joeloverbeck/rulepath/142ddfae2be3ae2d7c861ab65f2c786a49de54ac/docs/ENGINE-GAME-DATA-BOUNDARY.md) and [MECHANICAL-SCAFFOLDING-REGISTER.md](https://raw.githubusercontent.com/joeloverbeck/rulepath/142ddfae2be3ae2d7c861ab65f2c786a49de54ac/docs/MECHANICAL-SCAFFOLDING-REGISTER.md) keep behavior in typed Rust and prevent behavior-free scaffolding from becoming rule logic.
- [crates/wasm-api/src/constants.rs](https://raw.githubusercontent.com/joeloverbeck/rulepath/142ddfae2be3ae2d7c861ab65f2c786a49de54ac/crates/wasm-api/src/constants.rs), [crates/wasm-api/src/catalog.rs](https://raw.githubusercontent.com/joeloverbeck/rulepath/142ddfae2be3ae2d7c861ab65f2c786a49de54ac/crates/wasm-api/src/catalog.rs), [crates/wasm-api/src/lib.rs](https://raw.githubusercontent.com/joeloverbeck/rulepath/142ddfae2be3ae2d7c861ab65f2c786a49de54ac/crates/wasm-api/src/lib.rs), and [crates/wasm-api/Cargo.toml](https://raw.githubusercontent.com/joeloverbeck/rulepath/142ddfae2be3ae2d7c861ab65f2c786a49de54ac/crates/wasm-api/Cargo.toml) show a compile-time public game registry, game-specific enum, and direct dependencies on every public game crate. That shape is hostile to private optional inclusion unless a private overlay or generic extension seam is introduced.
- [ci/games.json](https://raw.githubusercontent.com/joeloverbeck/rulepath/142ddfae2be3ae2d7c861ab65f2c786a49de54ac/ci/games.json), [scripts/check-ci-games.mjs](https://raw.githubusercontent.com/joeloverbeck/rulepath/142ddfae2be3ae2d7c861ab65f2c786a49de54ac/scripts/check-ci-games.mjs), [scripts/check-catalog-docs.mjs](https://raw.githubusercontent.com/joeloverbeck/rulepath/142ddfae2be3ae2d7c861ab65f2c786a49de54ac/scripts/check-catalog-docs.mjs), and the three GitHub workflows show public set-equality assumptions and hardcoded public surfaces that must not be forced to mention private games.

### 2.3 Cuba Libre PDFs read

The uploaded living rules and playbook PDFs were read as private licensed source material. The relevant functional requirements, summarized without copying protected prose, are:

- Four asymmetric factions with fixed roles, different action menus, different victory formulas, and open negotiation/kingmaking pressure.
- A card-driven sequence of play with eligibility/ineligibility state, faction order printed on event cards, pass behavior, first/second eligible option constraints, limited operations, and a one-card lookahead model.
- A map/state model with spaces, control, support/opposition, resources, forces, markers, and faction-specific component states.
- Government operations, insurgent operations, and faction-specific special activities with different cost, targeting, adjacency, terrain, and state-change rules.
- Event cards with dual-use branches, card-specific exceptions, free operations/special activities, persistent insurgent effects, temporary government effects, and interactions with eligibility and later rounds.
- Propaganda rounds with ordered phases for victory checks, resources, support/opposition, redeploy/reset, and final-game handling.
- Non-player faction flowcharts and examples, which are evidence of complexity but are **not** a sanctioned source for future Rulepath bot policy.

### 2.4 External prior art researched

External research was used to pressure-test design choices, not to assert repository state.

- **Rally the Troops / GMT Digital Editions.** GMT publicly lists authorized free-to-play online implementations including a COIN-series title on Rally the Troops, and Rally the Troops states that its framework is open source while game modules/assets remain under copyright-holder license. The Rulepath lesson is “separate open framework from licensed modules,” not “put licensed game content in public source.”[^gmt-digital][^rtt]
- **VASSAL.** VASSAL is a free, open-source module engine for board games and supports live, PBEM, hotseat, and solitaire play, but its module-editor model is closer to virtual tabletop/module play than strict Rust-owned enforcement. The Rulepath lesson is that module isolation is useful; it does not justify weakening rule authority.[^vassal]
- **boardgame.io.** boardgame.io promotes functions that describe how game state changes, with networking/storage handled by the framework. The Rulepath lesson is that procedural behavior belongs in code; Rulepath still rejects TypeScript as rule authority.[^boardgameio]
- **OpenSpiel.** OpenSpiel models games procedurally and supports multiplayer, imperfect information, chance, sequential/simultaneous moves, and general-sum games. The Rulepath lesson is that rich game state/observation boundaries are normal; Rulepath does not adopt its search/RL stack for v1/v2 bots.[^openspiel]
- **GitHub Actions and Cargo.** GitHub supports reusable workflows referenced by `uses`, safest pinned by commit SHA; `actions/checkout` can check out multiple repositories but private secondary repositories require an explicit token; Cargo workspaces have explicit members and auto-member behavior for path dependencies under a workspace; Cargo git dependencies can be pinned by `rev`; Cargo alternate registries exist but are heavier than needed for milestone 1.[^gh-reuse][^checkout][^cargo-workspaces][^cargo-git][^cargo-registries]

## 3. Implemented baseline acknowledgment

The prior doc/template overhaul has already shipped the items the prompt said not to re-recommend. I verified them at `142ddfa` and build on them rather than re-adding them:

- [MULTI-SEAT-AND-SURFACE-CONTRACT.md](https://raw.githubusercontent.com/joeloverbeck/rulepath/142ddfae2be3ae2d7c861ab65f2c786a49de54ac/docs/MULTI-SEAT-AND-SURFACE-CONTRACT.md) exists and covers seat ranges, roles/factions, pairwise no-leak matrices, larger surfaces, and per-seat outcome obligations.
- [ROADMAP.md](https://raw.githubusercontent.com/joeloverbeck/rulepath/142ddfae2be3ae2d7c861ab65f2c786a49de54ac/docs/ROADMAP.md), [ADR 0007](https://raw.githubusercontent.com/joeloverbeck/rulepath/142ddfae2be3ae2d7c861ab65f2c786a49de54ac/docs/adr/0007-next-public-scaling-phase-and-gate-p-tail.md), and [specs/README.md](https://raw.githubusercontent.com/joeloverbeck/rulepath/142ddfae2be3ae2d7c861ab65f2c786a49de54ac/specs/README.md) already contain the public scaling phase and Gate P tail. This plan amends that accepted order; it does not fill a missing roadmap.
- [ADR 0008](https://raw.githubusercontent.com/joeloverbeck/rulepath/142ddfae2be3ae2d7c861ab65f2c786a49de54ac/docs/adr/0008-mechanical-scaffolding-governance.md), [MECHANICAL-SCAFFOLDING-REGISTER.md](https://raw.githubusercontent.com/joeloverbeck/rulepath/142ddfae2be3ae2d7c861ab65f2c786a49de54ac/docs/MECHANICAL-SCAFFOLDING-REGISTER.md), [ADR 0009](https://raw.githubusercontent.com/joeloverbeck/rulepath/142ddfae2be3ae2d7c861ab65f2c786a49de54ac/docs/adr/0009-replay-fixture-hash-taxonomy.md), [EVIDENCE-FIXTURE-CONTRACT.md](https://raw.githubusercontent.com/joeloverbeck/rulepath/142ddfae2be3ae2d7c861ab65f2c786a49de54ac/docs/EVIDENCE-FIXTURE-CONTRACT.md), and [ADR 0004](https://raw.githubusercontent.com/joeloverbeck/rulepath/142ddfae2be3ae2d7c861ab65f2c786a49de54ac/docs/adr/0004-hidden-info-replay-export-taxonomy.md) exist and should be reused.
- N-seat template fields and completion-profile machinery exist across `templates/**`; this plan extends them for private COIN scale rather than re-creating them.

---

## 4. The change plan

### Part A — Foundation docs (`docs/**`)

#### A-01 — Sanction a parallel private-game lane without making private pressure public architecture

**ID:** A-01  
**Target file(s):** [docs/FOUNDATIONS.md](https://raw.githubusercontent.com/joeloverbeck/rulepath/142ddfae2be3ae2d7c861ab65f2c786a49de54ac/docs/FOUNDATIONS.md)  
**Type:** amend + new-ADR  
**Evidence:** `FOUNDATIONS.md` currently ranks later private stress tests below public IP-safe games and says private licensed experiments must be late, isolated, optional, non-public, and unable to shape public architecture. Cuba Libre milestone 1 is explicitly authorized to start now.  
**Proposed change:** Add a narrow constitutional carve-out: “sanctioned private-game lane” work may begin before the public ladder completes only after an accepted ADR names the private-lane scope, repository isolation, CI expectations, catalog/build boundary, and public-architecture non-contamination rule. State that the priority order changes only for timing; private content still may not enter public files, public bundles, public docs, public CI artifacts, public traces, or `engine-core`.  
**Rationale:** Without this amendment, Cuba Libre violates the current priority order and the Gate-P tail. With it, the repo can learn from a monster game while refusing licensed-IP leakage or kernel capture.  
**Doctrine-check:** Touches priority order, private IP, stop conditions, and ADR triggers. Preserves Rust authority, noun-free `engine-core`, no hidden-info leaks, no DSL/YAML, and no private licensed content in public outputs.  
**Priority:** Critical  
**Depends-on:** D-01.

#### A-02 — Amend the roadmap’s Gate P tail into a governed private lane

**ID:** A-02  
**Target file(s):** [docs/ROADMAP.md](https://raw.githubusercontent.com/joeloverbeck/rulepath/142ddfae2be3ae2d7c861ab65f2c786a49de54ac/docs/ROADMAP.md)  
**Type:** amend  
**Evidence:** The roadmap currently keeps private monster-game work at the final Gate P tail after public Gates 21-23. Cuba Libre must begin now, parallel to that unfinished ladder.  
**Proposed change:** Add a “Private Lane P1 — sanctioned private licensed red-team” section that sits beside, not inside, the public gate sequence. It should state that public Gates 21-23 remain the public roadmap order; P1 may progress only in private repositories/builds; P1 cannot introduce public release blockers except when it reveals a public invariant violation in already-public seams; and P1 has its own private milestone statuses.  
**Rationale:** A parallel lane avoids pretending Cuba Libre is a public ladder gate while removing the doctrinal block on starting now.  
**Doctrine-check:** Limits ADR 0007’s timing but preserves its isolation/non-public/non-architectural intent.  
**Priority:** Critical  
**Depends-on:** A-01, D-01.

#### A-03 — Rewrite private-IP policy from “late experiments only” to “authorized now, isolated always”

**ID:** A-03  
**Target file(s):** [docs/IP-POLICY.md](https://raw.githubusercontent.com/joeloverbeck/rulepath/142ddfae2be3ae2d7c861ab65f2c786a49de54ac/docs/IP-POLICY.md)  
**Type:** amend  
**Evidence:** `IP-POLICY.md` already bans proprietary rules/card text/art/assets in public files and states that private experiments live in private repos/submodules/local paths, excluded from public CI/builds/docs. It currently frames them as late red-team work.  
**Proposed change:** Retain every public/private exclusion, but add a sanctioned-private-lane section: private work may start early after ADR approval; the private repository owns licensed rules references, private docs, private e2e names, private trace fixtures, private card metadata, and private renderers; public docs may contain only generic doctrine and opaque private-lane placeholders. Add a “no-name/no-ID public leak” checklist: no private game ID, card ID, e2e filename, fixture filename, rules-page copy, screenshot, or catalog string in public files unless separately license-reviewed and explicitly approved.  
**Rationale:** Catalog and CI integration are the dangerous points. The policy needs to say exactly how private content remains absent even when the private build is real.  
**Doctrine-check:** Strengthens the shipment rule and does not relax copyright/trademark/trade-dress constraints.  
**Priority:** Critical  
**Depends-on:** A-01, D-01, D-03.

#### A-04 — Add a typed Rust event-card mechanism boundary

**ID:** A-04  
**Target file(s):** [docs/ENGINE-GAME-DATA-BOUNDARY.md](https://raw.githubusercontent.com/joeloverbeck/rulepath/142ddfae2be3ae2d7c861ab65f2c786a49de54ac/docs/ENGINE-GAME-DATA-BOUNDARY.md), [docs/FOUNDATIONS.md](https://raw.githubusercontent.com/joeloverbeck/rulepath/142ddfae2be3ae2d7c861ab65f2c786a49de54ac/docs/FOUNDATIONS.md)  
**Type:** amend + new-ADR  
**Evidence:** Cuba Libre’s event deck requires conditional effects, rule overrides, dual-use branches, persistent effects, temporary effects, free operations, and interactions with eligibility/round state. The current boundary forbids selectors, conditions, triggers, conditional effects, and DSL behavior in static data.  
**Proposed change:** Add a section named “typed Rust card-effect registries.” It should allow game-local Rust code to define a `CardId`/`EventEffect`-style registry under an ADR. Static data may contain only card identity, sequence/deck metadata, inert display metadata, license-local source references, and non-behavioral parameters. Rust functions/match arms own all target selection, conditions, rule overrides, triggers, legality hooks, state transitions, visibility filtering, diagnostics, and effects. Ban YAML, JSON, TOML, RON, or table rows that encode selectors/conditions/effect formulas.  
**Rationale:** Typed Rust per card is safe but scales poorly. A typed registry keeps behavior reviewable and testable without creating a data-driven rules engine.  
**Doctrine-check:** Preserves “Rust owns behavior” and no DSL. The ADR must say this mechanism is game-local/private until public evidence justifies any public helper.  
**Priority:** Critical  
**Depends-on:** D-02.

#### A-05 — Define a private milestone completion profile and bot deferral rule

**ID:** A-05  
**Target file(s):** [docs/OFFICIAL-GAME-CONTRACT.md](https://raw.githubusercontent.com/joeloverbeck/rulepath/142ddfae2be3ae2d7c861ab65f2c786a49de54ac/docs/OFFICIAL-GAME-CONTRACT.md)  
**Type:** amend  
**Evidence:** The contract currently defines “official/done” with Level-0 random-legal bot as the floor. Cuba Libre milestone 1 is a private hotseat/rule-complete target with no complex AI.  
**Proposed change:** Add completion profiles for `private-milestone-1-rule-complete`, `private-release-candidate`, and `public-release-candidate`. For `private-milestone-1-rule-complete`, require full hotseat rules, all event cards, propaganda rounds, terminal/victory detection, replay/fixtures/no-leak/private CI, and private build/cat proof. Permit Level-0 random-legal bot to be explicitly deferred in `GAME-EVIDENCE.md` until `private-release-candidate` or official/private “done.” Keep Level-0 mandatory for public release and for any private build that exposes bot modes.  
**Rationale:** For a COIN-scale game, random-legal bot work before the full action tree settles is busywork and can distort milestone 1. It should not block hotseat legality.  
**Doctrine-check:** Does not change bot law for public releases. Deferral is explicit, bounded, and recorded as a profile decision, not a silent waiver.  
**Priority:** High  
**Depends-on:** A-01, D-01.

#### A-06 — Add COIN-scale mechanic categories and private-pressure accounting

**ID:** A-06  
**Target file(s):** [docs/MECHANIC-ATLAS.md](https://raw.githubusercontent.com/joeloverbeck/rulepath/142ddfae2be3ae2d7c861ab65f2c786a49de54ac/docs/MECHANIC-ATLAS.md)  
**Type:** amend  
**Evidence:** Current categories cover many public-game shapes but not the combined COIN pressure of faction-order eligibility, asymmetric operations/special activities, propaganda/upkeep rounds, conditional event cards, persistent card effects, support/control/resource tracks, and 4-way victory/kingmaking.  
**Proposed change:** Add mechanic-atlas categories for: card-driven eligibility/initiative; asymmetric faction menus; operation + special-activity coupling; periodic reset/upkeep rounds; conditional event-effect branches; persistent capability/momentum effects; faction-specific victory tracks; negotiated resource/cash transfer; and four-faction kingmaking/leader-pressure. Add a rule that private games exert “private stress evidence” but do not count as public third-use promotion pressure unless an ADR explicitly admits sanitized evidence.  
**Rationale:** The atlas needs vocabulary to review Cuba Libre without pushing private COIN nouns into public primitives.  
**Doctrine-check:** Keeps behavior local and prevents private licensed evidence from silently forcing public `game-stdlib` promotion.  
**Priority:** High  
**Depends-on:** A-01, A-04.

#### A-07 — Clarify that event payload routing and faction routing are behavior, not scaffolding

**ID:** A-07  
**Target file(s):** [docs/MECHANICAL-SCAFFOLDING-REGISTER.md](https://raw.githubusercontent.com/joeloverbeck/rulepath/142ddfae2be3ae2d7c861ab65f2c786a49de54ac/docs/MECHANICAL-SCAFFOLDING-REGISTER.md)  
**Type:** amend  
**Evidence:** Cuba Libre will tempt authors to call card dispatch, faction eligibility routing, event-effect payloads, propaganda sequencing, and target filtering “scaffolding.” ADR 0008 allows only behavior-free scaffolding.  
**Proposed change:** Add COIN anti-examples: event-card dispatch that selects targets, faction eligibility/initiative policy, operation/special activity coupling, propaganda phase transitions, persistent event expiry, and faction bot priorities are behavioral. Generic test harness geometry, typed stable-byte framing, and private fixture classification may be scaffolding.  
**Rationale:** Prevents a private monster game from smuggling rule logic into a behavior-free lane.  
**Doctrine-check:** Reinforces ADR 0008 and `engine-core` noun-free constraints.  
**Priority:** High  
**Depends-on:** A-04.

#### A-08 — Add private overlay architecture and large-action-tree guidance

**ID:** A-08  
**Target file(s):** [docs/ARCHITECTURE.md](https://raw.githubusercontent.com/joeloverbeck/rulepath/142ddfae2be3ae2d7c861ab65f2c786a49de54ac/docs/ARCHITECTURE.md)  
**Type:** amend  
**Evidence:** The current architecture has one public workspace and a public `wasm-api` depending directly on every catalog game. Cuba Libre needs private crates, private renderer/catalog registration, and large progressive action trees for multi-space operations and event resolution.  
**Proposed change:** Add a “private overlay build” lane to the repo shape: the public repo exposes generic extension seams, while private repositories own private game crates, private `wasm-api`/registry overlay or wrapper, private web renderer registry, private e2e, and private docs. Add action-tree guidance for large games: staged operations must be generated in Rust, allow progressive selection/confirmation, cap fanout via grouping/pagination metadata, and benchmark legal tree generation.  
**Rationale:** This documents how private work integrates without adding private members to the public workspace.  
**Doctrine-check:** Public architecture can add generic seams only; private nouns stay out of `engine-core` and public bundles.  
**Priority:** High  
**Depends-on:** D-03.

#### A-09 — Extend multi-seat law for asymmetric factions and coalition/kingmaking pressure

**ID:** A-09  
**Target file(s):** [docs/MULTI-SEAT-AND-SURFACE-CONTRACT.md](https://raw.githubusercontent.com/joeloverbeck/rulepath/142ddfae2be3ae2d7c861ab65f2c786a49de54ac/docs/MULTI-SEAT-AND-SURFACE-CONTRACT.md)  
**Type:** amend  
**Evidence:** Cuba Libre has four fixed asymmetric factions, open negotiation, faction-specific victory, eligibility ordering, and no simple team partition.  
**Proposed change:** Extend the seat-range declaration with “fixed asymmetric faction seats.” Require per-faction role/visibility rows, faction-order/eligibility state ownership, public negotiation limitations, per-faction terminal explanation payloads, coalition/kingmaking risk notes, and a 5-viewer no-leak matrix: public observer plus each of the four faction seats.  
**Rationale:** The current N-seat contract is present and correct; this is the COIN-scale specialization it needs.  
**Doctrine-check:** Rust remains the source for roles, active/pending actors, view authorization, and outcome causes.  
**Priority:** High  
**Depends-on:** A-01.

#### A-10 — Add future 4-faction AI sourcing limits; ban flowchart transcription as bot source

**ID:** A-10  
**Target file(s):** [docs/AI-BOTS.md](https://raw.githubusercontent.com/joeloverbeck/rulepath/142ddfae2be3ae2d7c861ab65f2c786a49de54ac/docs/AI-BOTS.md)  
**Type:** amend  
**Evidence:** The playbook includes non-player flowcharts and examples, but locked decision D6 says future Rulepath AI must be researched online for competent play, not transcribed from those flowcharts. `AI-BOTS.md` already bans MCTS/ISMCTS/Monte Carlo/ML/RL in v1/v2 and requires viewer-safe bot inputs/explanations.  
**Proposed change:** Add a private licensed game AI note: publisher non-player aids, flowcharts, and bot priority text are private source material and may not be copied into bot policy, strategy docs, explanations, or tests. Future four-faction AI must have four separate policy sections, legal-view-only inputs, coalition/kingmaking treatment, leader-targeting explanation policy, and no-flowchart provenance receipts.  
**Rationale:** The most attractive shortcut is also the most legally and doctrinally dangerous.  
**Doctrine-check:** Preserves v1/v2 AI bans and no hidden-info peeking.  
**Priority:** Medium  
**Depends-on:** A-03.

#### A-11 — Define private catalog semantics at the WASM/browser boundary

**ID:** A-11  
**Target file(s):** [docs/WASM-CLIENT-BOUNDARY.md](https://raw.githubusercontent.com/joeloverbeck/rulepath/142ddfae2be3ae2d7c861ab65f2c786a49de54ac/docs/WASM-CLIENT-BOUNDARY.md)  
**Type:** amend  
**Evidence:** `rulepath_list_games` is the normal game catalog endpoint. The current public registry compiles public games into one public WASM crate. Cuba Libre must appear in normal lists only when the private repo/build is present.  
**Proposed change:** Add a private-catalog rule: `list_games` may include private games only in a private build artifact produced from a private repository or private overlay. Public `list_games` must not emit private placeholders, disabled rows, private IDs, private display names, or private docs paths. Private builds must mark the artifact private, run private no-leak tests, and never be uploaded as public CI/web artifacts.  
**Rationale:** “Present-only” catalog inclusion is acceptable only as compile/build artifact selection, not a runtime flag in the public bundle.  
**Doctrine-check:** Protects the unauthorized-browser shipment rule and keeps TypeScript presentation-only.  
**Priority:** High  
**Depends-on:** D-03.

#### A-12 — Add private web-shell and large asymmetric UI guidance

**ID:** A-12  
**Target file(s):** [docs/UI-INTERACTION.md](https://raw.githubusercontent.com/joeloverbeck/rulepath/142ddfae2be3ae2d7c861ab65f2c786a49de54ac/docs/UI-INTERACTION.md)  
**Type:** amend  
**Evidence:** Cuba Libre needs a large map, many markers, four faction panels, eligibility/sequence display, event/propaganda context, action construction, and terminal/victory explanations. Public UI doctrine currently assumes public original visual targets.  
**Proposed change:** Add a private-web overlay section: private renderers live in private source; public shell may expose only generic renderer-extension seams. Require COIN-scale UI documents to budget map objects, per-faction panels, sequence/eligibility, event/round display, action-tree grouping, effect batching, reduced-motion summaries, and per-faction outcome explanations. Ban copied maps, icons, card layouts, screenshots, color/trade dress, and rulebook/player-aid diagrams.  
**Rationale:** The browser is the highest-risk IP surface. UI doctrine must say “functional remake, original presentation,” not “digital scan.”  
**Doctrine-check:** TS remains presenter-only; no private assets enter public app.  
**Priority:** High  
**Depends-on:** A-03, A-11.

#### A-13 — Scale testing/replay/benchmarking for private COIN games

**ID:** A-13  
**Target file(s):** [docs/TESTING-REPLAY-BENCHMARKING.md](https://raw.githubusercontent.com/joeloverbeck/rulepath/142ddfae2be3ae2d7c861ab65f2c786a49de54ac/docs/TESTING-REPLAY-BENCHMARKING.md)  
**Type:** amend  
**Evidence:** Cuba Libre milestone 1 needs four faction seats, all operation/special activity paths, all event-card branches, propaganda rounds, terminal detection, viewer-scoped replay/export, and private CI.  
**Proposed change:** Add a “private large-game coverage” subsection requiring: per-faction named rule tests; event-card/effect-branch coverage; propaganda-round golden traces; four-seat pairwise no-leak tests across view/action/effect/diagnostic/export surfaces; large action-tree fanout benchmarks; long-game replay import/export tests; private artifact bundle inspection; and benchmark thresholds tuned in private CI.  
**Rationale:** Existing testing doctrine is sound but too small-scale for a COIN implementation.  
**Doctrine-check:** No private fixtures/traces in public CI. Determinism and viewer-scoped exports remain.  
**Priority:** High  
**Depends-on:** A-03, A-04, D-03.

#### A-14 — Add private-source evidence profiles for licensed game artifacts

**ID:** A-14  
**Target file(s):** [docs/EVIDENCE-FIXTURE-CONTRACT.md](https://raw.githubusercontent.com/joeloverbeck/rulepath/142ddfae2be3ae2d7c861ab65f2c786a49de54ac/docs/EVIDENCE-FIXTURE-CONTRACT.md)  
**Type:** amend  
**Evidence:** `EVIDENCE-FIXTURE-CONTRACT.md` already has `private-source` visibility. Cuba Libre needs private card/source coverage, private fixtures, private trace exports, and public-safe aggregate evidence.  
**Proposed change:** Add `private-rule-source-v1`, `private-event-coverage-v1`, and `private-build-inspection-v1` profile guidance. These profiles may store private source IDs, private event IDs, and private source-version receipts only in the private repo. Public docs may record only aggregate status: pass/fail/blocker and opaque private evidence IDs.  
**Rationale:** Maintainers need proof without copying proof material into public source.  
**Doctrine-check:** Extends ADR 0009 profile taxonomy without weakening ADR 0004 no-leak rules.  
**Priority:** High  
**Depends-on:** A-03, A-13.

#### A-15 — Keep trace schema v1; add large-event vocabulary guidance without migration

**ID:** A-15  
**Target file(s):** [docs/TRACE-SCHEMA-v1.md](https://raw.githubusercontent.com/joeloverbeck/rulepath/142ddfae2be3ae2d7c861ab65f2c786a49de54ac/docs/TRACE-SCHEMA-v1.md)  
**Type:** clarify  
**Evidence:** The current trace schema can hold command/action/effect/replay metadata. Cuba Libre does not inherently require a public trace schema change; it requires larger command/effect vocabularies and private-source profiles.  
**Proposed change:** Add a note that large private games should not change trace schema merely because they have many rule IDs/effect names. They may add game-local effect variants and private profile artifacts, but schema/hash migration still requires ADR 0009-style authority.  
**Rationale:** Prevents “COIN is large” from becoming a silent trace migration.  
**Doctrine-check:** Determinism and hash compatibility preserved.  
**Priority:** Medium  
**Depends-on:** A-14.

#### A-16 — Record external private-IP and event-engine lessons in sources doctrine

**ID:** A-16  
**Target file(s):** [docs/SOURCES.md](https://raw.githubusercontent.com/joeloverbeck/rulepath/142ddfae2be3ae2d7c861ab65f2c786a49de54ac/docs/SOURCES.md)  
**Type:** amend  
**Evidence:** External prior art shows useful separations: open framework vs licensed modules, code-owned state transitions, procedural game-state models, and reusable CI/workspace patterns.  
**Proposed change:** Add a “private licensed games / event-driven engines” bibliography note covering Rally the Troops/GMT Digital Editions, VASSAL, boardgame.io, OpenSpiel, GitHub reusable workflows, GitHub checkout private-repo behavior, Cargo workspaces/git dependencies/registries. For each, record the Rulepath lesson and non-adoption.  
**Rationale:** Future maintainers should know why the plan recommends private overlay + typed Rust effects, not a public submodule or DSL.  
**Doctrine-check:** External sources inform doctrine; they do not become rule authority.  
**Priority:** Medium  
**Depends-on:** None.

#### A-17 — Add private-monster task discipline and decomposition law

**ID:** A-17  
**Target file(s):** [docs/AGENT-DISCIPLINE.md](https://raw.githubusercontent.com/joeloverbeck/rulepath/142ddfae2be3ae2d7c861ab65f2c786a49de54ac/docs/AGENT-DISCIPLINE.md)  
**Type:** amend  
**Evidence:** Cuba Libre is too large for a single implementation task. The agent law already bans unbounded kernel changes, DSLs, and private IP in public files.  
**Proposed change:** Add a private-monster protocol: first author/private-lane ADRs and spec placement; then private implementation specs split by setup/state, sequence/eligibility, operations, special activities, propaganda, events, visibility/replay, UI, CI; each task must name source/IP handling and public-boundary impact; no task may “implement Cuba Libre” as a single packet.  
**Rationale:** Decomposition is the only way to keep the private game reviewable.  
**Doctrine-check:** Reinforces bounded outputs and stop conditions.  
**Priority:** High  
**Depends-on:** A-01.

#### A-18 — Update the documentation map and ADR status index

**ID:** A-18  
**Target file(s):** [docs/README.md](https://raw.githubusercontent.com/joeloverbeck/rulepath/142ddfae2be3ae2d7c861ab65f2c786a49de54ac/docs/README.md)  
**Type:** amend  
**Evidence:** New private-lane, event-card, and private VCS/CI/catalog ADRs will become authority-bearing. Docs map currently lists ADR 0001-0009.  
**Proposed change:** Add the new ADRs to the status index after acceptance and cross-link the amended docs. Add a short map note that private-lane doctrine sits below `FOUNDATIONS.md` but can amend roadmap/IP policy only through accepted ADRs.  
**Rationale:** Future sessions need a single authority map.  
**Doctrine-check:** Maintains existing hierarchy.  
**Priority:** High  
**Depends-on:** D-01, D-02, D-03.

#### A-19 — Add archival handling for superseded private-lane roadmap text

**ID:** A-19  
**Target file(s):** [docs/archival-workflow.md](https://raw.githubusercontent.com/joeloverbeck/rulepath/142ddfae2be3ae2d7c861ab65f2c786a49de54ac/docs/archival-workflow.md)  
**Type:** clarify  
**Evidence:** ADR 0007 is accepted but will be limited/superseded in part. Specs index/roadmap entries may need status notes without erasing history.  
**Proposed change:** Add a short “ADR-limited roadmap text” note: when a new ADR supersedes part of a prior accepted roadmap ADR, the older text remains historical, the current docs carry the active law, and archive notes must name the superseding ADR rather than deleting the old rationale.  
**Rationale:** Avoids ambiguity over whether Gate P was “wrong” or merely amended for a new private lane.  
**Doctrine-check:** Process-only.  
**Priority:** Low  
**Depends-on:** D-01.

### Part B — Templates (`templates/**`)

#### B-01 — Add private-lane index guidance to the template README

**ID:** B-01  
**Target file(s):** [templates/README.md](https://raw.githubusercontent.com/joeloverbeck/rulepath/142ddfae2be3ae2d7c861ab65f2c786a49de54ac/templates/README.md)  
**Type:** amend  
**Evidence:** Templates currently govern official public game docs and completion profiles; there is no private-release analogue to the public checklist.  
**Proposed change:** Add a private-game template bundle: `PRIVATE-RELEASE-CHECKLIST.md`, event-card coverage, private-source evidence rows, and per-faction AI/how-to splits. State that private templates live in the private repo for private game docs; public template files may define structure but must not include private examples.  
**Rationale:** Maintainers need a clean checklist without copying Cuba Libre content into public templates.  
**Doctrine-check:** Template structure only; no private data.  
**Priority:** High  
**Depends-on:** A-03.

#### B-02 — Extend agent task packets for private-source tasks

**ID:** B-02  
**Target file(s):** [templates/AGENT-TASK.md](https://raw.githubusercontent.com/joeloverbeck/rulepath/142ddfae2be3ae2d7c861ab65f2c786a49de54ac/templates/AGENT-TASK.md)  
**Type:** amend  
**Evidence:** Private implementation will require many bounded tasks and IP-sensitive source handling.  
**Proposed change:** Add fields: `private-source touched?`, `public artifact touched?`, `private build only?`, `source material may not be quoted`, `private evidence artifact IDs`, and `no public leak acceptance command`. Require an explicit “public tree diff contains no private licensed content” review for any task touching public repo seams.  
**Rationale:** Converts IP discipline into task-level acceptance.  
**Doctrine-check:** Reinforces public/private stop conditions.  
**Priority:** High  
**Depends-on:** A-17.

#### B-03 — Add private-source and event-deck sections to rules template

**ID:** B-03  
**Target file(s):** [templates/GAME-RULES.md](https://raw.githubusercontent.com/joeloverbeck/rulepath/142ddfae2be3ae2d7c861ab65f2c786a49de54ac/templates/GAME-RULES.md)  
**Type:** amend  
**Evidence:** Cuba Libre requires stable rule IDs for operations, special activities, card-driven sequence, propaganda rounds, and event effects, but public prose cannot copy licensed rules.  
**Proposed change:** Add a private-source mode: rule IDs may reference private source IDs and section numbers in the private repo, but public-safe rule summaries must be original and functional. Add sections for faction seat model, event-card effect authority, persistent/temporary card effects, and periodic round phases.  
**Rationale:** Keeps the formal rules contract useful at COIN scale while remaining IP-clean.  
**Doctrine-check:** No copied rules prose; no behavior tables in static data.  
**Priority:** High  
**Depends-on:** A-03, A-04.

#### B-04 — Expand mechanic inventory for COIN-scale categories

**ID:** B-04  
**Target file(s):** [templates/GAME-MECHANICS.md](https://raw.githubusercontent.com/joeloverbeck/rulepath/142ddfae2be3ae2d7c861ab65f2c786a49de54ac/templates/GAME-MECHANICS.md)  
**Type:** amend  
**Evidence:** Current categories include N-seat, turn-order, topology, resources, action shape, and related rows, but not card-driven eligibility, operations/special-activity coupling, propaganda phases, or private stress accounting.  
**Proposed change:** Add optional rows for card-driven initiative/eligibility, asymmetric faction menus, operations + special activity, periodic upkeep/propaganda, conditional event branches, persistent/temporary effects, faction-specific victory tracks, private stress evidence, and non-flowchart bot pressure.  
**Rationale:** A future Cuba Libre spec should not have to invent these categories ad hoc.  
**Doctrine-check:** Inventory remains evidence, not permission to generalize.  
**Priority:** High  
**Depends-on:** A-06.

#### B-05 — Split rule coverage into scalable rule and event-effect matrices

**ID:** B-05  
**Target file(s):** [templates/GAME-RULE-COVERAGE.md](https://raw.githubusercontent.com/joeloverbeck/rulepath/142ddfae2be3ae2d7c861ab65f2c786a49de54ac/templates/GAME-RULE-COVERAGE.md); new template `templates/GAME-EVENT-COVERAGE.md`  
**Type:** amend + new  
**Evidence:** A single rule-coverage matrix will be unwieldy for roughly ninety event-effect branches plus operations, special activities, propaganda, setup, visibility, and terminal rules.  
**Proposed change:** Keep `GAME-RULE-COVERAGE.md` as the rule-ID-to-proof index, but add a linked event-effect coverage template with rows for private event ID, branch kind, behavior owner, source section/private card reference, named tests, golden traces, visibility effects, replay/hash impact, and status. The public template must use placeholder IDs only.  
**Rationale:** Card effects need coverage without copying card text.  
**Doctrine-check:** Event behavior remains Rust; coverage rows are evidence, not executable data.  
**Priority:** Critical  
**Depends-on:** A-04, D-02.

#### B-06 — Extend sources template for private licensed source receipts

**ID:** B-06  
**Target file(s):** [templates/GAME-SOURCES.md](https://raw.githubusercontent.com/joeloverbeck/rulepath/142ddfae2be3ae2d7c861ab65f2c786a49de54ac/templates/GAME-SOURCES.md)  
**Type:** amend  
**Evidence:** The current source template bans proprietary text/assets and records source quality. Cuba Libre needs private official PDFs and possibly future strategy sources.  
**Proposed change:** Add `private-source` rows with `public-safe summary`, `private repo path or source ID`, `copied prose/assets: none`, `license review`, `may appear in public docs? no`, and `used for rules / ambiguity / strategy / not bot source`. Add an explicit row type for “publisher non-player aid: consulted for complexity only, not copied or translated into bot policy.”  
**Rationale:** This separates “we had access” from “we may publish.”  
**Doctrine-check:** Strengthens IP policy and future AI boundaries.  
**Priority:** High  
**Depends-on:** A-03, A-10.

#### B-07 — Make how-to-play per-faction and private-safe

**ID:** B-07  
**Target file(s):** [templates/GAME-HOW-TO-PLAY.md](https://raw.githubusercontent.com/joeloverbeck/rulepath/142ddfae2be3ae2d7c861ab65f2c786a49de54ac/templates/GAME-HOW-TO-PLAY.md)  
**Type:** amend  
**Evidence:** Current how-to template has a single “players/seats/roles” and action-list path. Cuba Libre needs four faction-specific player guides, but public/private docs may not copy tutorial examples or player-aid prose.  
**Proposed change:** Add a “fixed asymmetric faction game” mode: one neutral overview plus one short original section per faction, each covering objective, normal action menu, special-action relationship, round pressure, and visibility/negotiation notes. Require private game how-to pages to be stored in the private repo and to avoid copied examples, historical flavor, card text, diagrams, and trade dress.  
**Rationale:** One generic action list is not teachable for COIN-scale asymmetry.  
**Doctrine-check:** Player-facing prose is original; no hidden/private facts.  
**Priority:** Medium  
**Depends-on:** A-09.

#### B-08 — Extend UI template for private overlay, large map, and four-faction dashboards

**ID:** B-08  
**Target file(s):** [templates/GAME-UI.md](https://raw.githubusercontent.com/joeloverbeck/rulepath/142ddfae2be3ae2d7c861ab65f2c786a49de54ac/templates/GAME-UI.md)  
**Type:** amend  
**Evidence:** Current UI template already covers object counts, multi-seat layout, legal action mapping, previews, effects, outcome, accessibility, and no-leak. Cuba Libre needs much larger surfaces and private renderer/build proofs.  
**Proposed change:** Add fields for private/public build target, private renderer registry, public artifact exclusion, map-object budget, marker/piece count budget, event/round panel, faction dashboard rows, eligibility sequence display, action-tree fanout, and private screenshot prohibition.  
**Rationale:** The private build can use normal shell semantics, but its renderer and assets cannot be public.  
**Doctrine-check:** TypeScript remains presentation-only; renderer does not decide legality.  
**Priority:** High  
**Depends-on:** A-11, A-12.

#### B-09 — Extend AI registry for four asymmetric policies and milestone deferral

**ID:** B-09  
**Target file(s):** [templates/GAME-AI.md](https://raw.githubusercontent.com/joeloverbeck/rulepath/142ddfae2be3ae2d7c861ab65f2c786a49de54ac/templates/GAME-AI.md)  
**Type:** amend  
**Evidence:** Milestone 1 may defer Level-0; future AI must be researched and four-faction-specific.  
**Proposed change:** Add rows for `not implemented / deferred by private-milestone-1 profile`, one row per faction policy, allowed-input summary, public-default status, and `flowchart-derived? must be no`. Add a field that says whether bot modes are compiled into the private catalog.  
**Rationale:** AI absence should be explicit and non-blocking for M1, then rigorous later.  
**Doctrine-check:** No v1/v2 excluded AI methods; no hidden-info shortcut.  
**Priority:** Medium  
**Depends-on:** A-05, A-10.

#### B-10 — Make competent-player analysis faction-specific and non-flowchart

**ID:** B-10  
**Target file(s):** [templates/COMPETENT-PLAYER.md](https://raw.githubusercontent.com/joeloverbeck/rulepath/142ddfae2be3ae2d7c861ab65f2c786a49de54ac/templates/COMPETENT-PLAYER.md)  
**Type:** amend  
**Evidence:** Current template covers seat/opponent model, phases, threats, resources, inference, and kingmaking. Cuba Libre needs four asymmetric analyses and coalition/leader pressure.  
**Proposed change:** Add a required per-faction structure for 3+ asymmetric games: faction objective, opening/mid/end pressure, public signals, threats to block, negotiation/kingmaking risk, leader-targeting rules, and legal-view-only inference. Add a source-provenance checkbox that no publisher flowchart text or priority structure was copied.  
**Rationale:** A single competent-player summary will blur faction incentives.  
**Doctrine-check:** Strategy remains evidence, not rule authority or hidden implementation plan.  
**Priority:** Medium  
**Depends-on:** A-10.

#### B-11 — Extend bot evidence pack for multi-opponent authored policies

**ID:** B-11  
**Target file(s):** [templates/BOT-STRATEGY-EVIDENCE-PACK.md](https://raw.githubusercontent.com/joeloverbeck/rulepath/142ddfae2be3ae2d7c861ab65f2c786a49de54ac/templates/BOT-STRATEGY-EVIDENCE-PACK.md)  
**Type:** amend  
**Evidence:** Future Cuba Libre AI needs four asymmetric policies, legal-action API use, coalition/kingmaking, and no flowchart-derived priorities.  
**Proposed change:** Add rows for faction-specific candidate extraction, opponent/leader evaluation, table-wide standings, coalition/kingmaking guardrails, visible-only deck/event knowledge, explanation redaction, and no-flowchart provenance. Keep lexicographic priority vectors; forbid giant weight soups and copied priority charts.  
**Rationale:** COIN AI is where hidden peeking and IP copying are most likely.  
**Doctrine-check:** Preserves deterministic authored policy law and v1/v2 exclusions.  
**Priority:** Medium  
**Depends-on:** A-10.

#### B-12 — Add COIN-scale benchmark workloads

**ID:** B-12  
**Target file(s):** [templates/GAME-BENCHMARKS.md](https://raw.githubusercontent.com/joeloverbeck/rulepath/142ddfae2be3ae2d7c861ab65f2c786a49de54ac/templates/GAME-BENCHMARKS.md)  
**Type:** amend  
**Evidence:** Cuba Libre’s action trees can be large because one operation can involve many spaces and a later event branch can touch many state subsystems.  
**Proposed change:** Add workloads for largest map view projection, largest legal tree, event resolution, propaganda round, long-game replay import/export, private WASM build load, per-viewer no-leak projection, and private renderer smoke. Add “bot-turn” as `deferred/not applicable` for M1 unless bot is compiled.  
**Rationale:** Benchmark pressure should be measured in the private repo before the public shell adopts any generic performance changes.  
**Doctrine-check:** No premature optimization without evidence; no private artifacts in public CI.  
**Priority:** High  
**Depends-on:** A-13.

#### B-13 — Extend evidence receipt for private build and private-source proof

**ID:** B-13  
**Target file(s):** [templates/GAME-EVIDENCE.md](https://raw.githubusercontent.com/joeloverbeck/rulepath/142ddfae2be3ae2d7c861ab65f2c786a49de54ac/templates/GAME-EVIDENCE.md)  
**Type:** amend  
**Evidence:** Current receipt includes completion profile, source/IP, trace profiles, viewer matrix, hidden-info matrix. Cuba Libre needs private CI and private build proof.  
**Proposed change:** Add completion profiles from A-05; private source profiles from A-14; private build artifact inspection; public tree no-leak receipt; private catalog inclusion receipt; and a row for “Level-0 bot deferred by profile.” Add opaque private evidence IDs rather than public links when the artifact is private.  
**Rationale:** One evidence receipt should tell reviewers whether the private milestone is legally and technically safe.  
**Doctrine-check:** Does not waive foundation invariants.  
**Priority:** High  
**Depends-on:** A-05, A-14, D-03.

#### B-14 — Extend implementation admission for private-lane ADR gates

**ID:** B-14  
**Target file(s):** [templates/GAME-IMPLEMENTATION-ADMISSION.md](https://raw.githubusercontent.com/joeloverbeck/rulepath/142ddfae2be3ae2d7c861ab65f2c786a49de54ac/templates/GAME-IMPLEMENTATION-ADMISSION.md)  
**Type:** amend  
**Evidence:** The admission template asks whether implementation may begin under current foundations. Cuba Libre cannot begin until private-lane and event-card ADRs are proposed/accepted as required.  
**Proposed change:** Add private-game admission rows: sanctioned-private-lane ADR accepted, event-card ADR accepted, private VCS/CI/catalog ADR accepted, private repo present, public tree leak review passed, private-source plan ready, event coverage strategy ready, bot profile decided.  
**Rationale:** Admission is the correct gate for “doctrine no longer blocks this.”  
**Doctrine-check:** No implementation starts under unaccepted constitutional exceptions.  
**Priority:** Critical  
**Depends-on:** D-01, D-02, D-03.

#### B-15 — Add private stress evidence to primitive pressure ledger without promoting it

**ID:** B-15  
**Target file(s):** [templates/PRIMITIVE-PRESSURE-LEDGER.md](https://raw.githubusercontent.com/joeloverbeck/rulepath/142ddfae2be3ae2d7c861ab65f2c786a49de54ac/templates/PRIMITIVE-PRESSURE-LEDGER.md)  
**Type:** amend  
**Evidence:** Cuba Libre will exert pressure on graph/topology, event cards, resource accounting, action construction, and multi-faction victory. Private evidence should not silently trigger public third-use hard gates.  
**Proposed change:** Add a `private-stress-only` pressure type. Require a sanitized rationale before private evidence may be counted toward public promotion, and an ADR if the only third-use evidence is private licensed work.  
**Rationale:** Private games are valuable stress tests but cannot be the hidden reason for public architecture.  
**Doctrine-check:** Keeps public mechanic promotion reviewable and IP-clean.  
**Priority:** High  
**Depends-on:** A-06.

#### B-16 — Add a private-release checklist and cross-link the public one

**ID:** B-16  
**Target file(s):** new `templates/PRIVATE-RELEASE-CHECKLIST.md`; [templates/PUBLIC-RELEASE-CHECKLIST.md](https://raw.githubusercontent.com/joeloverbeck/rulepath/142ddfae2be3ae2d7c861ab65f2c786a49de54ac/templates/PUBLIC-RELEASE-CHECKLIST.md)  
**Type:** new + amend  
**Evidence:** Public checklist has a shipment rule and public artifact proof, but no private-release analogue. Cuba Libre needs private build readiness without public release.  
**Proposed change:** Create a private checklist with: authorized audience, private repo/build commit, private source license review, public tree leak scan, private WASM/catalog inspection, private CI gates 0/1/2, private e2e, private fixture/export classification, no public upload, local/authorized hosting constraints, and human signoff. Amend public checklist to say it is not a private-release checklist and link to the private analogue.  
**Rationale:** Private shipment still ships to someone. It needs a release decision surface.  
**Doctrine-check:** Reinforces “if it ships to an unauthorized browser, it has shipped.”  
**Priority:** Critical  
**Depends-on:** A-03, D-03.

### Part C — VCS / CI / catalog & code-seam doctrine

#### C-01 — Choose a separate private repository with pinned public Rulepath checkout as the default

**ID:** C-01  
**Target file(s):** [docs/IP-POLICY.md](https://raw.githubusercontent.com/joeloverbeck/rulepath/142ddfae2be3ae2d7c861ab65f2c786a49de54ac/docs/IP-POLICY.md), [docs/ARCHITECTURE.md](https://raw.githubusercontent.com/joeloverbeck/rulepath/142ddfae2be3ae2d7c861ab65f2c786a49de54ac/docs/ARCHITECTURE.md), [Cargo.toml](https://raw.githubusercontent.com/joeloverbeck/rulepath/142ddfae2be3ae2d7c861ab65f2c786a49de54ac/Cargo.toml)  
**Type:** process doctrine + code-seam identification  
**Evidence:** The public workspace explicitly lists every member in `Cargo.toml`; Cargo workspaces run root commands over members by default and path dependencies under a workspace can become members. Public `Cargo.toml` should not name private paths. Cargo supports exact git revisions for dependencies; private repos can pin public Rulepath by `rev` or checkout/submodule.[^cargo-workspaces][^cargo-git]  
**Proposed change:** Make the default architecture a private repository that pins the public Rulepath commit and owns a private workspace. The private repo may vendor public Rulepath as a submodule/checkout under `vendor/rulepath` or use exact `rev` git dependencies; it owns `games_private/<opaque-private-game>`, private docs, private fixtures, private e2e, private CI manifests, private web overlay, and private WASM artifact. Do **not** add the private game to public `Cargo.toml` members.  
**Rationale:** This is the cleanest isolation/reproducibility balance: the private repo can run full CI and build the game while the public repo remains cloneable, buildable, and IP-clean.  
**Doctrine-check:** Private game nouns and licensed content never enter public workspace metadata.  
**Priority:** Critical  
**Depends-on:** D-03.

#### C-02 — Reject public submodule/default optional dependency as default; allow only private-local overlays

**ID:** C-02  
**Target file(s):** [docs/IP-POLICY.md](https://raw.githubusercontent.com/joeloverbeck/rulepath/142ddfae2be3ae2d7c861ab65f2c786a49de54ac/docs/IP-POLICY.md), [Cargo.toml](https://raw.githubusercontent.com/joeloverbeck/rulepath/142ddfae2be3ae2d7c861ab65f2c786a49de54ac/Cargo.toml)  
**Type:** process doctrine  
**Evidence:** A public `.gitmodules`, optional dependency, feature, e2e name, or workspace member can leak the existence, path, or ID of a licensed game even if contents are absent. GitHub checkout can fetch submodules/private repos with tokens, but that creates CI/auth/log complexity.[^checkout]  
**Proposed change:** State that a public submodule or optional dependency naming a private game is not the default. Developers may use local-only overlays ignored by git, but public source may include only generic extension seam names. A private repo can include the public repo as a submodule; the public repo should not include the private repo.  
**Rationale:** Direction matters. Public → private reference leaks. Private → public reference does not.  
**Doctrine-check:** Preserves no private IP and no private names in public surfaces.  
**Priority:** High  
**Depends-on:** C-01.

#### C-03 — Refactor catalog registration into public registry plus private overlay registry

**ID:** C-03  
**Target file(s):** [crates/wasm-api/src/constants.rs](https://raw.githubusercontent.com/joeloverbeck/rulepath/142ddfae2be3ae2d7c861ab65f2c786a49de54ac/crates/wasm-api/src/constants.rs), [crates/wasm-api/src/catalog.rs](https://raw.githubusercontent.com/joeloverbeck/rulepath/142ddfae2be3ae2d7c861ab65f2c786a49de54ac/crates/wasm-api/src/catalog.rs), [crates/wasm-api/src/lib.rs](https://raw.githubusercontent.com/joeloverbeck/rulepath/142ddfae2be3ae2d7c861ab65f2c786a49de54ac/crates/wasm-api/src/lib.rs), [crates/wasm-api/Cargo.toml](https://raw.githubusercontent.com/joeloverbeck/rulepath/142ddfae2be3ae2d7c861ab65f2c786a49de54ac/crates/wasm-api/Cargo.toml), [docs/WASM-CLIENT-BOUNDARY.md](https://raw.githubusercontent.com/joeloverbeck/rulepath/142ddfae2be3ae2d7c861ab65f2c786a49de54ac/docs/WASM-CLIENT-BOUNDARY.md)  
**Type:** code-seam doctrine  
**Evidence:** Current `wasm-api` hardcodes public `GAME_*` constants, a `RegisteredGame` enum, direct public game dependencies, `MatchRecord` variants, and `list_games` rows. A private game cannot be “present when repo exists” without modifying these compile-time surfaces or creating a private wrapper.  
**Proposed change:** Plan a later implementation seam: extract a public `RegistryEntry`/`GameAdapter` contract and keep the public registry as the default provider. The private repo builds either (a) a private `wasm-api-private` crate with the same external ABI and both public + private adapters, or (b) a compile-time overlay provider compiled only in the private workspace. Public source must not contain private IDs, display names, private docs paths, or private dependencies.  
**Rationale:** This validates the maintainer hypothesis with a stricter rule: private games appear in normal lists only in the private build artifact, never by dormant public runtime flag.  
**Doctrine-check:** Rust/WASM still owns catalog, views, legal actions, and validation. Public bundle remains private-free.  
**Priority:** Critical  
**Depends-on:** D-03.

#### C-04 — Add a private web renderer overlay seam instead of importing private React from public app

**ID:** C-04  
**Target file(s):** [apps/web/README.md](https://raw.githubusercontent.com/joeloverbeck/rulepath/142ddfae2be3ae2d7c861ab65f2c786a49de54ac/apps/web/README.md), [docs/UI-INTERACTION.md](https://raw.githubusercontent.com/joeloverbeck/rulepath/142ddfae2be3ae2d7c861ab65f2c786a49de54ac/docs/UI-INTERACTION.md), [docs/WASM-CLIENT-BOUNDARY.md](https://raw.githubusercontent.com/joeloverbeck/rulepath/142ddfae2be3ae2d7c861ab65f2c786a49de54ac/docs/WASM-CLIENT-BOUNDARY.md)  
**Type:** code-seam doctrine  
**Evidence:** The public web shell lists public board renderers and imports public components. A private map renderer and any private help/rules assets cannot be imported from public source.  
**Proposed change:** Plan a renderer-extension seam: the public app exports shell components and accepts a renderer registry. The public build supplies only public renderers. The private repo supplies a private app package or build entry that imports the public shell plus private renderers and private WASM. The private build can show the private game in the normal picker because the private WASM catalog includes it and the private renderer registry can render it.  
**Rationale:** The “normal catalog” user experience is preserved without shipping private code to public users.  
**Doctrine-check:** No private React/assets in public bundle; TS remains presenter-only.  
**Priority:** High  
**Depends-on:** C-03.

#### C-05 — Federate CI through reusable workflows and private manifests

**ID:** C-05  
**Target file(s):** [.github/workflows/gate-0-hygiene.yml](https://raw.githubusercontent.com/joeloverbeck/rulepath/142ddfae2be3ae2d7c861ab65f2c786a49de54ac/.github/workflows/gate-0-hygiene.yml), [.github/workflows/gate-1-game-smoke.yml](https://raw.githubusercontent.com/joeloverbeck/rulepath/142ddfae2be3ae2d7c861ab65f2c786a49de54ac/.github/workflows/gate-1-game-smoke.yml), [.github/workflows/gate-2-benchmarks.yml](https://raw.githubusercontent.com/joeloverbeck/rulepath/142ddfae2be3ae2d7c861ab65f2c786a49de54ac/.github/workflows/gate-2-benchmarks.yml), [ci/games.json](https://raw.githubusercontent.com/joeloverbeck/rulepath/142ddfae2be3ae2d7c861ab65f2c786a49de54ac/ci/games.json), [scripts/check-ci-games.mjs](https://raw.githubusercontent.com/joeloverbeck/rulepath/142ddfae2be3ae2d7c861ab65f2c786a49de54ac/scripts/check-ci-games.mjs)  
**Type:** code-seam doctrine  
**Evidence:** Gate 0 runs workspace fmt/lint/build/test; Gate 1 emits a per-game matrix from `ci/games.json`; Gate 2 has hardcoded public benchmark lanes. GitHub reusable workflows can be called by `uses`, safest pinned by SHA; private repositories can call shared workflows with appropriate access and token constraints.[^gh-reuse][^gh-private-workflows]  
**Proposed change:** Refactor public workflows into callable reusable workflows with inputs for workspace root, game matrix path/JSON, web build command, private/public artifact mode, and benchmark package list. Public runs use `ci/games.json` and public hardcoded defaults. Private repo calls the reusable workflows at a pinned SHA with `ci/private-games.json` and private benchmark lists, or initially vendors the workflows until reusable inputs exist. Gate 2 must become manifest-driven before it can cover private games fully.  
**Rationale:** Private games should meet the same bar without uploading private artifacts to public CI.  
**Doctrine-check:** Private secrets/tokens are private-repo scoped; public CI remains private-free.  
**Priority:** Critical  
**Depends-on:** C-01, D-03.

#### C-06 — Split catalog/docs drift checks into public and private surfaces

**ID:** C-06  
**Target file(s):** [scripts/check-catalog-docs.mjs](https://raw.githubusercontent.com/joeloverbeck/rulepath/142ddfae2be3ae2d7c861ab65f2c786a49de54ac/scripts/check-catalog-docs.mjs), [apps/web/README.md](https://raw.githubusercontent.com/joeloverbeck/rulepath/142ddfae2be3ae2d7c861ab65f2c786a49de54ac/apps/web/README.md), root `README.md`  
**Type:** code-seam doctrine  
**Evidence:** `check-catalog-docs.mjs` parses public `GAME_*` constants and requires every cataloged game to appear in public app/root README/smoke surfaces. That is correct for public games and wrong for private games.  
**Proposed change:** Keep the public check public-only. Add support for an explicit `--catalog-source public|private` or a private sibling script in the private repo. Public checks must fail if private IDs appear in public constants/readmes. Private checks should assert that private catalog entries have private docs/e2e surfaces in the private repo and no public docs paths.  
**Rationale:** The current set-equality check would either force private names into public docs or block private cataloging.  
**Doctrine-check:** Public docs remain public-only; private docs are checked privately.  
**Priority:** High  
**Depends-on:** C-03.

#### C-07 — Keep boundary and scaffolding checks public-safe, but parameterize private audit roots

**ID:** C-07  
**Target file(s):** [scripts/boundary-check.sh](https://raw.githubusercontent.com/joeloverbeck/rulepath/142ddfae2be3ae2d7c861ab65f2c786a49de54ac/scripts/boundary-check.sh), [scripts/check-scaffolding-governance.mjs](https://raw.githubusercontent.com/joeloverbeck/rulepath/142ddfae2be3ae2d7c861ab65f2c786a49de54ac/scripts/check-scaffolding-governance.mjs), [ci/scaffolding-audits.json](https://raw.githubusercontent.com/joeloverbeck/rulepath/142ddfae2be3ae2d7c861ab65f2c786a49de54ac/ci/scaffolding-audits.json)  
**Type:** code-seam doctrine  
**Evidence:** `boundary-check.sh` protects `engine-core` from mechanic/domain nouns, including `faction`, `card`, `deck`, `initiative`, and `eligibility`; private COIN nouns belong in private game crates, not engine-core. `check-scaffolding-governance.mjs` assumes public `games/` and public CI manifests.  
**Proposed change:** Private CI must run `boundary-check.sh` against the public engine checkout unchanged. Add a private-root mode or private sibling governance check so private games can record scaffolding audits without adding private game IDs to public `ci/scaffolding-audits.json`.  
**Rationale:** Boundary law is exactly what protects the public kernel from private COIN pressure. Governance checks need a private input lane, not relaxed rules.  
**Doctrine-check:** `engine-core` remains noun-free; private audit data stays private.  
**Priority:** High  
**Depends-on:** C-01, C-05.

#### C-08 — Evaluate realistic VCS/CI/catalog options and default decision

**ID:** C-08  
**Target file(s):** new doctrine note, likely in [docs/IP-POLICY.md](https://raw.githubusercontent.com/joeloverbeck/rulepath/142ddfae2be3ae2d7c861ab65f2c786a49de54ac/docs/IP-POLICY.md) and [docs/ARCHITECTURE.md](https://raw.githubusercontent.com/joeloverbeck/rulepath/142ddfae2be3ae2d7c861ab65f2c786a49de54ac/docs/ARCHITECTURE.md)  
**Type:** comparison + default recommendation  
**Evidence:** The task explicitly delegates this architecture choice. External Cargo/GitHub docs and current Rulepath seams constrain the options.  
**Proposed change:** Add the following comparison and default:

| Option | Isolation | CI coverage | Catalog integration | Reproducibility | Decision |
|---|---|---|---|---|---|
| Public repo contains private submodule | Weak; leaks repo/path/name | Possible with tokens | Easy locally, risky publicly | Fragile absent auth | Reject as default |
| Separate private repo, pins public Rulepath checkout/dependency | Strong | Full private Gate 0/1/2 | Via private WASM/web overlay | Strong with pinned SHA/rev | **Default** |
| Workspace overlay/private members file | Medium; easy local accidents | Good locally | Good locally | Depends on generated state | Allowed only inside private repo |
| Public optional feature/private dependency | Weak; leaks names and can bundle | Hard to prove absence | Easy but dangerous | Feature mistakes costly | Reject |
| Private Cargo registry | Strong | Good | Good if crates are packaged | Strong | Defer until API/package discipline matures |

**Rationale:** The separate private repo is boring and safe. That is the point.  
**Doctrine-check:** No public private-IP leaks; no private pressure in engine-core.  
**Priority:** Critical  
**Depends-on:** D-03.

### Part D — Doctrine & ADR stubs

#### D-01 — ADR stub: Sanctioned Parallel Private-Game Lane

**ID:** D-01  
**Target file(s):** new `docs/adr/0010-sanctioned-parallel-private-game-lane.md`  
**Type:** new-ADR  
**Status:** Proposed  
**Context:** `FOUNDATIONS.md` and ADR 0007 make private monster-game work a late tail after the public ladder. Cuba Libre is approved to start now as Rulepath’s first private licensed game.  
**Decision:** Create a sanctioned private-game lane that may run in parallel with the public roadmap after explicit ADR approval. The lane permits private licensed implementation work now, but only in private repositories/build artifacts. It does not authorize private content in public source, public docs, public CI artifacts, public traces, public app bundles, or `engine-core`. Public architecture may gain only generic, private-free extension seams.  
**FOUNDATIONS/ROADMAP/IP-POLICY sections amended:** `FOUNDATIONS.md` priority order, private-IP invariants/stop conditions, ADR triggers; `ROADMAP.md` Gate P ordering; `IP-POLICY.md` private experiment timing.  
**Consequences:** Cuba Libre milestone 1 can be admitted without waiting for Gate 23. ADR 0007 remains valid for the public ladder and for the original Gate P tail except where this ADR limits timing. Any private lane leak is a stop condition.

#### D-02 — ADR stub: Constrained Typed Rust Event-Card Mechanism

**ID:** D-02  
**Target file(s):** new `docs/adr/0011-constrained-typed-rust-event-card-mechanism.md`  
**Type:** new-ADR  
**Status:** Proposed  
**Context:** Cuba Libre’s event-card branches need conditional effects, target selection, persistent/temporary effects, and rule overrides. Existing doctrine forbids conditions/triggers/selectors/effects in static data and forbids DSLs without ADR.  
**Decision:** Authorize a game-local typed Rust event-card mechanism. Card identity, deck order, inert display metadata, and non-behavioral parameters may be typed static content in the private crate. Every condition, selector, trigger, rule override, target choice, legality check, state transition, visibility decision, diagnostic, and semantic effect is implemented as Rust behavior through explicit functions/match arms/traits. No YAML, script, untyped JSON/TOML effect rows, or declarative behavior language is allowed.  
**FOUNDATIONS/ROADMAP/IP-POLICY sections amended:** `FOUNDATIONS.md` static-data/no-DSL section; `ENGINE-GAME-DATA-BOUNDARY.md` typed content/behavior line; `MECHANIC-ATLAS.md` private event pressure notes.  
**Consequences:** This avoids ninety ad hoc unstructured effect blobs while preserving Rust authority. Promotion outside the private game requires later public-safe evidence and a separate decision.

#### D-03 — ADR stub: Private Repository, CI Federation, and Catalog Overlay Architecture

**ID:** D-03  
**Target file(s):** new `docs/adr/0012-private-repository-ci-catalog-overlay.md`  
**Type:** new-ADR  
**Status:** Proposed  
**Context:** Private games must run the full testing bar and appear in the normal web catalog when the private repo/build is present, but public bundles/docs/CI cannot contain private licensed IP or even unnecessary private identifiers.  
**Decision:** Default private games to a separate private repository that pins the public Rulepath commit and owns private game crates, docs, fixtures, renderer overlay, e2e, private CI manifests, and private WASM/web build. Public repo changes may add only generic extension seams and reusable workflow inputs. Public catalog contains only public games. Private catalog entries appear only in private build artifacts. Public submodule/feature/optional dependency that names private games is rejected as the default.  
**FOUNDATIONS/ROADMAP/IP-POLICY sections amended:** `FOUNDATIONS.md` private architecture trigger; `IP-POLICY.md` private build/repo rules; `WASM-CLIENT-BOUNDARY.md` catalog boundary; `ARCHITECTURE.md` overlay shape.  
**Consequences:** Private CI can run Gate 0/1/2 in the private repo. The normal web app experience is preserved by a private artifact, not by public dormant code. Public reproducibility remains intact.

### Part E — Roadmap & specs placement

#### E-01 — Add Private Lane P1 to roadmap

**ID:** E-01  
**Target file(s):** [docs/ROADMAP.md](https://raw.githubusercontent.com/joeloverbeck/rulepath/142ddfae2be3ae2d7c861ab65f2c786a49de54ac/docs/ROADMAP.md)  
**Type:** amend  
**Evidence:** D5 requires a Cuba Libre milestone-1 lane/gate.  
**Proposed change:** Add `Private Lane P1 — Private COIN milestone 1` after the public scaling overview but outside the public gate order. Public text should use an opaque private-lane ID if the maintainers choose not to publish the licensed title. The private repo may name the game directly. Exit criteria: accepted ADRs D-01/D-02/D-03; private repo/workspace admitted; full four-faction hotseat; all operations/special activities; propaganda rounds; all event cards; victory/terminal detection; replay/fixture/no-leak evidence; private CI Gate 0/1/2; private-release checklist not yet necessarily satisfied.  
**Rationale:** Separates roadmap permission from game implementation spec.  
**Doctrine-check:** Private lane does not reorder public Gates 21-23.  
**Priority:** Critical  
**Depends-on:** D-01.

#### E-02 — Add a public specs-index placeholder and private spec fields

**ID:** E-02  
**Target file(s):** [specs/README.md](https://raw.githubusercontent.com/joeloverbeck/rulepath/142ddfae2be3ae2d7c861ab65f2c786a49de54ac/specs/README.md) plus private repo `specs/private-coin-m1.md` later  
**Type:** amend  
**Evidence:** `specs/README.md` currently tracks public gates and Gate P as not started. D5 requires private Cuba Libre milestone placement and private spec fields, but public specs must not leak licensed content.  
**Proposed change:** Add a “Private lane tracker” table with an opaque row such as `P1-M1 — private COIN licensed game milestone 1` and status `Doctrine pending`. The public row should link only to accepted ADRs and say the executable spec lives in the private repo. The private spec must contain: mechanical-scaffolding reuse audit; no-leak matrix scope; fixed four-faction seat declaration; event-card mechanism compliance; private-source receipt; surface budgets; VCS/CI/catalog plan; private-release readiness; bot deferral or Level-0 requirement decision; and a public-tree no-leak closeout.  
**Rationale:** Public index acknowledges the lane without publishing private content.  
**Doctrine-check:** Specs remain subordinate to foundations; private source remains private.  
**Priority:** Critical  
**Depends-on:** E-01, D-03.

#### E-03 — Define milestone-1 capability target and explicit non-goals

**ID:** E-03  
**Target file(s):** private spec later; [docs/OFFICIAL-GAME-CONTRACT.md](https://raw.githubusercontent.com/joeloverbeck/rulepath/142ddfae2be3ae2d7c861ab65f2c786a49de54ac/docs/OFFICIAL-GAME-CONTRACT.md) profile note  
**Type:** amend/spec-field requirement  
**Evidence:** Locked decision D7 defines milestone 1. D6 says AI is milestone 2+.  
**Proposed change:** The private spec must state milestone 1 in scope: full four-faction hotseat; all operations and special activities; propaganda rounds; terminal/victory detection; all event cards resolved; viewer-safe replay/export; private CI. Out of scope: complex AI, flowchart bots, hosted multiplayer, public release, public assets, public rule docs, engine-core nouns, DSL/YAML, and generic COIN engine abstraction. Level-0 random-legal bot is deferred unless private release candidate status is requested.  
**Rationale:** Keeps the first private milestone large but finite.  
**Doctrine-check:** No unbounded “implement everything” mandate.  
**Priority:** Critical  
**Depends-on:** A-05, E-02.

---

## 5. Prioritized execution order

1. **D-01 / A-01 / A-02 / A-03 / A-18 / A-19** — Accept the sanctioned-private-lane ADR and update constitutional, roadmap, IP, index, and archival doctrine. No private spec begins before this.
2. **D-02 / A-04 / A-06 / A-07 / B-04 / B-05 / B-15** — Accept the typed Rust event-card mechanism ADR and make mechanic/scaffolding/template boundaries safe for event coverage.
3. **D-03 / C-01 through C-08 / A-08 / A-11 / A-12 / B-08 / B-16** — Accept the private VCS/CI/catalog ADR, choose the separate private repo default, and document private catalog/web/CI seams.
4. **A-05 / B-09 / B-10 / B-11 / B-13 / B-14** — Add private milestone profiles, AI deferral, and private evidence/admission fields.
5. **A-09 / A-13 / A-14 / A-15 / B-03 / B-06 / B-07 / B-12** — Scale multiseat, testing, fixtures, trace guidance, rules/source/how-to/bench templates.
6. **A-16** — Add external prior-art source notes.
7. **E-01 / E-02 / E-03** — Place P1-M1 in the roadmap/spec index and define private spec fields.
8. **Later implementation session** — Only after the above: create the private repository/workspace and author the private implementation spec. This plan does not implement those steps.

---

## 6. Risks & rejected ideas

- **Untyped event-card DSL/YAML/static behavior rows — rejected.** It directly violates Rulepath’s no-conditional-static-data/no-DSL boundary. Typed Rust registries are enough.
- **Typed Rust per card with no shared registry discipline — rejected as default.** It is legally safe but likely produces unreviewable sprawl at COIN scale. The plan still allows explicit per-card Rust functions inside the typed registry.
- **Runtime flag hiding private content in the public WASM/JS bundle — rejected.** If the bytes reach an unauthorized browser, the private game has shipped.
- **Public repo submodule pointing to a private game repo — rejected as default.** It leaks private repository/path identity and creates public CI auth/log risk.
- **Public optional Cargo feature/dependency for the private game — rejected.** Cargo features are compile-time knobs, not IP isolation, and public `Cargo.toml` would still name private code.
- **Private game as a reason to add `faction`, `deck`, `card`, `operation`, or `eligibility` to `engine-core` — rejected.** COIN nouns stay in the private game crate or later in `game-stdlib` only through public-safe evidence and accepted doctrine.
- **Copying publisher non-player flowcharts into AI policy — rejected.** Future AI must be researched and written in original Rulepath terms, with legal-view-only inputs.
- **Making complex AI part of milestone 1 — rejected.** Milestone 1 is hotseat/rule-complete. Bot work starts after action/state surfaces stabilize.
- **Publishing private specs/rule coverage/card coverage in the public repo — rejected.** Public docs may include opaque lane status and doctrine only.
- **VASSAL-style non-enforcing module as success criterion — rejected.** Rulepath’s value is Rust-enforced legality, replayability, testing, and no-leak evidence.
- **Private Cargo registry as milestone-1 default — deferred.** It is viable later but heavier than a pinned private repository checkout while Rulepath APIs are still moving.

---

## 7. Self-check

- **Rust owns behavior:** Preserved. Event-card behavior, operation legality, special activities, propaganda, victory, views, effects, replay, and bots remain Rust-owned.
- **`engine-core` noun-free:** Preserved. No recommendation adds COIN nouns to `engine-core`; boundary checks remain important.
- **No untyped DSL/YAML:** Preserved. The recommended event mechanism is typed Rust only; static content remains inert.
- **Hidden-info/no-leak:** Preserved. Four faction seats plus public observer require pairwise no-leak across views, actions, effects, diagnostics, exports, DOM/log/storage, and future bot explanations.
- **Determinism:** Preserved. Replay/hash/schema changes are avoided unless ADR 0009-style migration is explicitly invoked.
- **No private licensed IP in public surfaces:** Preserved. Public docs may carry only doctrine and opaque private-lane placeholders; private names, cards, rules prose, art, docs, traces, fixtures, and e2e live in the private repo/build only.
- **No MCTS/ISMCTS/Monte Carlo/ML/RL bots in v1/v2:** Preserved. Complex AI is future work and remains under `AI-BOTS.md` law.
- **Implemented baseline not re-recommended:** Confirmed. This plan extends existing multi-seat, ADR 0008/0009, evidence fixture, and N-seat templates; it does not re-add them as missing.
- **Read-list correspondence:** All selected read-list paths resolved against the uploaded manifest and were fetched from the exact commit URL form. No manifest-listed required path was missing.
- **PDF IP discipline:** The plan references mechanics functionally and generically. It does not copy or closely paraphrase rulebook/playbook prose, card text, flowchart text, examples, diagrams, art, or trade dress.

---

## Appendix A — Exact-commit repository fetch ledger

Requested repository: `joeloverbeck/rulepath`  
Target commit: `142ddfae2be3ae2d7c861ab65f2c786a49de54ac`  
Fetches used for repository-state claims:

- https://raw.githubusercontent.com/joeloverbeck/rulepath/142ddfae2be3ae2d7c861ab65f2c786a49de54ac/docs/FOUNDATIONS.md
- https://raw.githubusercontent.com/joeloverbeck/rulepath/142ddfae2be3ae2d7c861ab65f2c786a49de54ac/docs/README.md
- https://raw.githubusercontent.com/joeloverbeck/rulepath/142ddfae2be3ae2d7c861ab65f2c786a49de54ac/docs/ARCHITECTURE.md
- https://raw.githubusercontent.com/joeloverbeck/rulepath/142ddfae2be3ae2d7c861ab65f2c786a49de54ac/docs/ENGINE-GAME-DATA-BOUNDARY.md
- https://raw.githubusercontent.com/joeloverbeck/rulepath/142ddfae2be3ae2d7c861ab65f2c786a49de54ac/docs/OFFICIAL-GAME-CONTRACT.md
- https://raw.githubusercontent.com/joeloverbeck/rulepath/142ddfae2be3ae2d7c861ab65f2c786a49de54ac/docs/MECHANIC-ATLAS.md
- https://raw.githubusercontent.com/joeloverbeck/rulepath/142ddfae2be3ae2d7c861ab65f2c786a49de54ac/docs/MECHANICAL-SCAFFOLDING-REGISTER.md
- https://raw.githubusercontent.com/joeloverbeck/rulepath/142ddfae2be3ae2d7c861ab65f2c786a49de54ac/docs/ROADMAP.md
- https://raw.githubusercontent.com/joeloverbeck/rulepath/142ddfae2be3ae2d7c861ab65f2c786a49de54ac/docs/IP-POLICY.md
- https://raw.githubusercontent.com/joeloverbeck/rulepath/142ddfae2be3ae2d7c861ab65f2c786a49de54ac/docs/AI-BOTS.md
- https://raw.githubusercontent.com/joeloverbeck/rulepath/142ddfae2be3ae2d7c861ab65f2c786a49de54ac/docs/MULTI-SEAT-AND-SURFACE-CONTRACT.md
- https://raw.githubusercontent.com/joeloverbeck/rulepath/142ddfae2be3ae2d7c861ab65f2c786a49de54ac/docs/AGENT-DISCIPLINE.md
- https://raw.githubusercontent.com/joeloverbeck/rulepath/142ddfae2be3ae2d7c861ab65f2c786a49de54ac/docs/WASM-CLIENT-BOUNDARY.md
- https://raw.githubusercontent.com/joeloverbeck/rulepath/142ddfae2be3ae2d7c861ab65f2c786a49de54ac/docs/UI-INTERACTION.md
- https://raw.githubusercontent.com/joeloverbeck/rulepath/142ddfae2be3ae2d7c861ab65f2c786a49de54ac/docs/TESTING-REPLAY-BENCHMARKING.md
- https://raw.githubusercontent.com/joeloverbeck/rulepath/142ddfae2be3ae2d7c861ab65f2c786a49de54ac/docs/EVIDENCE-FIXTURE-CONTRACT.md
- https://raw.githubusercontent.com/joeloverbeck/rulepath/142ddfae2be3ae2d7c861ab65f2c786a49de54ac/docs/TRACE-SCHEMA-v1.md
- https://raw.githubusercontent.com/joeloverbeck/rulepath/142ddfae2be3ae2d7c861ab65f2c786a49de54ac/docs/SOURCES.md
- https://raw.githubusercontent.com/joeloverbeck/rulepath/142ddfae2be3ae2d7c861ab65f2c786a49de54ac/docs/archival-workflow.md
- https://raw.githubusercontent.com/joeloverbeck/rulepath/142ddfae2be3ae2d7c861ab65f2c786a49de54ac/docs/adr/ADR-TEMPLATE.md
- https://raw.githubusercontent.com/joeloverbeck/rulepath/142ddfae2be3ae2d7c861ab65f2c786a49de54ac/docs/adr/0001-stage-1-random-playout-budget.md
- https://raw.githubusercontent.com/joeloverbeck/rulepath/142ddfae2be3ae2d7c861ab65f2c786a49de54ac/docs/adr/0002-ci-benchmark-gating-lanes.md
- https://raw.githubusercontent.com/joeloverbeck/rulepath/142ddfae2be3ae2d7c861ab65f2c786a49de54ac/docs/adr/0003-ci-calibrated-benchmark-thresholds.md
- https://raw.githubusercontent.com/joeloverbeck/rulepath/142ddfae2be3ae2d7c861ab65f2c786a49de54ac/docs/adr/0004-hidden-info-replay-export-taxonomy.md
- https://raw.githubusercontent.com/joeloverbeck/rulepath/142ddfae2be3ae2d7c861ab65f2c786a49de54ac/docs/adr/0005-variance-aware-ci-benchmark-floors.md
- https://raw.githubusercontent.com/joeloverbeck/rulepath/142ddfae2be3ae2d7c861ab65f2c786a49de54ac/docs/adr/0006-blackjack-lite-roadmap-placement.md
- https://raw.githubusercontent.com/joeloverbeck/rulepath/142ddfae2be3ae2d7c861ab65f2c786a49de54ac/docs/adr/0007-next-public-scaling-phase-and-gate-p-tail.md
- https://raw.githubusercontent.com/joeloverbeck/rulepath/142ddfae2be3ae2d7c861ab65f2c786a49de54ac/docs/adr/0008-mechanical-scaffolding-governance.md
- https://raw.githubusercontent.com/joeloverbeck/rulepath/142ddfae2be3ae2d7c861ab65f2c786a49de54ac/docs/adr/0009-replay-fixture-hash-taxonomy.md
- https://raw.githubusercontent.com/joeloverbeck/rulepath/142ddfae2be3ae2d7c861ab65f2c786a49de54ac/templates/README.md
- https://raw.githubusercontent.com/joeloverbeck/rulepath/142ddfae2be3ae2d7c861ab65f2c786a49de54ac/templates/AGENT-TASK.md
- https://raw.githubusercontent.com/joeloverbeck/rulepath/142ddfae2be3ae2d7c861ab65f2c786a49de54ac/templates/GAME-RULES.md
- https://raw.githubusercontent.com/joeloverbeck/rulepath/142ddfae2be3ae2d7c861ab65f2c786a49de54ac/templates/GAME-MECHANICS.md
- https://raw.githubusercontent.com/joeloverbeck/rulepath/142ddfae2be3ae2d7c861ab65f2c786a49de54ac/templates/GAME-RULE-COVERAGE.md
- https://raw.githubusercontent.com/joeloverbeck/rulepath/142ddfae2be3ae2d7c861ab65f2c786a49de54ac/templates/GAME-SOURCES.md
- https://raw.githubusercontent.com/joeloverbeck/rulepath/142ddfae2be3ae2d7c861ab65f2c786a49de54ac/templates/GAME-HOW-TO-PLAY.md
- https://raw.githubusercontent.com/joeloverbeck/rulepath/142ddfae2be3ae2d7c861ab65f2c786a49de54ac/templates/GAME-UI.md
- https://raw.githubusercontent.com/joeloverbeck/rulepath/142ddfae2be3ae2d7c861ab65f2c786a49de54ac/templates/GAME-AI.md
- https://raw.githubusercontent.com/joeloverbeck/rulepath/142ddfae2be3ae2d7c861ab65f2c786a49de54ac/templates/COMPETENT-PLAYER.md
- https://raw.githubusercontent.com/joeloverbeck/rulepath/142ddfae2be3ae2d7c861ab65f2c786a49de54ac/templates/BOT-STRATEGY-EVIDENCE-PACK.md
- https://raw.githubusercontent.com/joeloverbeck/rulepath/142ddfae2be3ae2d7c861ab65f2c786a49de54ac/templates/GAME-BENCHMARKS.md
- https://raw.githubusercontent.com/joeloverbeck/rulepath/142ddfae2be3ae2d7c861ab65f2c786a49de54ac/templates/GAME-EVIDENCE.md
- https://raw.githubusercontent.com/joeloverbeck/rulepath/142ddfae2be3ae2d7c861ab65f2c786a49de54ac/templates/GAME-IMPLEMENTATION-ADMISSION.md
- https://raw.githubusercontent.com/joeloverbeck/rulepath/142ddfae2be3ae2d7c861ab65f2c786a49de54ac/templates/PRIMITIVE-PRESSURE-LEDGER.md
- https://raw.githubusercontent.com/joeloverbeck/rulepath/142ddfae2be3ae2d7c861ab65f2c786a49de54ac/templates/PUBLIC-RELEASE-CHECKLIST.md
- https://raw.githubusercontent.com/joeloverbeck/rulepath/142ddfae2be3ae2d7c861ab65f2c786a49de54ac/specs/README.md
- https://raw.githubusercontent.com/joeloverbeck/rulepath/142ddfae2be3ae2d7c861ab65f2c786a49de54ac/reports/RULEPATH-DOC-AND-TEMPLATE-CHANGE-PLAN.md
- https://raw.githubusercontent.com/joeloverbeck/rulepath/142ddfae2be3ae2d7c861ab65f2c786a49de54ac/reports/doc-and-template-overhaul-from-game-evidence-research-brief.md
- https://raw.githubusercontent.com/joeloverbeck/rulepath/142ddfae2be3ae2d7c861ab65f2c786a49de54ac/Cargo.toml
- https://raw.githubusercontent.com/joeloverbeck/rulepath/142ddfae2be3ae2d7c861ab65f2c786a49de54ac/crates/wasm-api/Cargo.toml
- https://raw.githubusercontent.com/joeloverbeck/rulepath/142ddfae2be3ae2d7c861ab65f2c786a49de54ac/crates/wasm-api/src/constants.rs
- https://raw.githubusercontent.com/joeloverbeck/rulepath/142ddfae2be3ae2d7c861ab65f2c786a49de54ac/crates/wasm-api/src/catalog.rs
- https://raw.githubusercontent.com/joeloverbeck/rulepath/142ddfae2be3ae2d7c861ab65f2c786a49de54ac/crates/wasm-api/src/games.rs
- https://raw.githubusercontent.com/joeloverbeck/rulepath/142ddfae2be3ae2d7c861ab65f2c786a49de54ac/crates/wasm-api/src/lib.rs
- https://raw.githubusercontent.com/joeloverbeck/rulepath/142ddfae2be3ae2d7c861ab65f2c786a49de54ac/crates/wasm-api/src/store.rs
- https://raw.githubusercontent.com/joeloverbeck/rulepath/142ddfae2be3ae2d7c861ab65f2c786a49de54ac/ci/games.json
- https://raw.githubusercontent.com/joeloverbeck/rulepath/142ddfae2be3ae2d7c861ab65f2c786a49de54ac/ci/scaffolding-audits.json
- https://raw.githubusercontent.com/joeloverbeck/rulepath/142ddfae2be3ae2d7c861ab65f2c786a49de54ac/.github/workflows/gate-0-hygiene.yml
- https://raw.githubusercontent.com/joeloverbeck/rulepath/142ddfae2be3ae2d7c861ab65f2c786a49de54ac/.github/workflows/gate-1-game-smoke.yml
- https://raw.githubusercontent.com/joeloverbeck/rulepath/142ddfae2be3ae2d7c861ab65f2c786a49de54ac/.github/workflows/gate-2-benchmarks.yml
- https://raw.githubusercontent.com/joeloverbeck/rulepath/142ddfae2be3ae2d7c861ab65f2c786a49de54ac/scripts/boundary-check.sh
- https://raw.githubusercontent.com/joeloverbeck/rulepath/142ddfae2be3ae2d7c861ab65f2c786a49de54ac/scripts/check-catalog-docs.mjs
- https://raw.githubusercontent.com/joeloverbeck/rulepath/142ddfae2be3ae2d7c861ab65f2c786a49de54ac/scripts/check-ci-games.mjs
- https://raw.githubusercontent.com/joeloverbeck/rulepath/142ddfae2be3ae2d7c861ab65f2c786a49de54ac/scripts/check-scaffolding-governance.mjs
- https://raw.githubusercontent.com/joeloverbeck/rulepath/142ddfae2be3ae2d7c861ab65f2c786a49de54ac/apps/web/README.md
- https://raw.githubusercontent.com/joeloverbeck/rulepath/142ddfae2be3ae2d7c861ab65f2c786a49de54ac/apps/web/package.json
- https://raw.githubusercontent.com/joeloverbeck/rulepath/142ddfae2be3ae2d7c861ab65f2c786a49de54ac/apps/web/public/rules/manifest.json

---

## Appendix B — External research sources

[^gmt-digital]: GMT Games, “Digital Editions,” listing authorized online implementations and noting free-to-play online platforms. https://www.gmtgames.com/t-digitaleditions.aspx
[^rtt]: Rally the Troops GitHub organization README, stating framework source is open while game modules/assets remain licensed by copyright holders. https://github.com/rally-the-troops
[^vassal]: VASSAL Designer’s Guide 3.7.5, overview of VASSAL as a free/open-source board-game module engine supporting live, PBEM, hotseat, and solitaire modes. https://vassalengine.org/doc/3.7.5/designerguide/designerguide.pdf
[^boardgameio]: boardgame.io homepage, describing code functions for state changes plus networking/storage features. https://boardgame.io/
[^openspiel]: OpenSpiel documentation, “What is OpenSpiel?”, listing support for single/multiplayer, imperfect information, stochasticity, sequential/simultaneous, and general-sum/cooperative games. https://openspiel.readthedocs.io/en/latest/intro.html
[^gh-reuse]: GitHub Docs, “Reuse workflows,” including `jobs.<job_id>.uses` syntax and SHA-pinning guidance. https://docs.github.com/en/actions/how-tos/reuse-automations/reuse-workflows
[^checkout]: `actions/checkout` documentation, including multiple-repo checkout, submodule options, and private secondary repository token requirements. https://github.com/actions/checkout
[^gh-private-workflows]: GitHub Docs, “Sharing actions and workflows from your private repository,” including private reusable workflow access and log/token warnings. https://docs.github.com/en/actions/how-tos/reuse-automations/share-across-private-repositories
[^cargo-workspaces]: Cargo Book, “Workspaces,” `members`, `exclude`, and package selection behavior. https://doc.rust-lang.org/cargo/reference/workspaces.html
[^cargo-git]: Cargo Book, “Specifying dependencies,” git dependency `rev`/`tag`/`branch` behavior and `Cargo.lock` pinning. https://doc.rust-lang.org/cargo/reference/specifying-dependencies.html
[^cargo-registries]: Cargo Book, “Registries,” alternate registry support. https://doc.rust-lang.org/cargo/reference/registries.html

# Deep-research brief — author the **Gate 20 (Star Halma / Chinese Checkers) implementation spec** for Rulepath

> **You are ChatGPT-Pro, the deep researcher (Session 2).** This brief is final and
> self-contained. The requirements below were settled in a prior session with full repository
> access. **Do not interview, do not ask clarifying questions, do not re-decide what comes
> next.** Produce the deliverable directly as a downloadable markdown document. If a genuine
> contradiction makes a requirement impossible, state it in the deliverable and proceed with
> the most faithful interpretation.

---

## 1. Context

The uploaded file `manifest_2026-06-27_b3e7efd.txt` is the exact path inventory of the
`joeloverbeck/rulepath` repository — a Rust-first, rule-enforcing, replayable, testable
card/board-game platform where **Rust owns all behavior and TypeScript/React present only**.
The foundation docs are an ordered, layered authority indexed by `docs/README.md`:
`FOUNDATIONS.md` (the constitution) → `ARCHITECTURE.md` → `ENGINE-GAME-DATA-BOUNDARY.md` →
the area docs → `ROADMAP.md`; earlier documents govern later ones, and accepted ADRs supersede
them only by explicitly naming the affected sections.

**Fetch every file you read from commit `b3e7efd` (verified `HEAD`); the uploaded manifest is
that exact tree (`git ls-tree -r --name-only HEAD`).** If any source you consult cites a
different "commit of record," note the divergence and use `b3e7efd`, not the cited string.

**What just shipped (the delta baseline).** The entire trick-taking lane and the first
non-trick card game are now `Done`. Per `specs/README.md`'s active-epoch tracker:
**Gate 19 — Five Hundred Rummy (`games/meldfall_ledger`)** flipped to `Done` (2026-06-26), and
its two follow-ons **Gate 19.1** (multi-round completion) and **Gate 19.2** (settlement-detail
projection) flipped `Done` (2026-06-26 / 2026-06-27). Earlier, **Gate 18 — Spades
(`games/blackglass_pact`)** shipped (2026-06-25) as the first `forward-v1` reuse-first
scaffolding audit user under the 8F forward-governance extension, and **Gate 19** was the
**second** `forward-v1` user. The mechanic-atlas open-promotion-debt register
(`docs/MECHANIC-ATLAS.md` §10A) is **empty** (last reviewed at the Gate 19 closeout,
2026-06-26). The whole shipped corpus — the trick-taking games (`plain_tricks`, `briar_circuit`,
`vow_tide`, `blackglass_pact`) and the two promoted `game-stdlib::trick_taking` helpers, the
N-seat hidden-information betting exemplar `games/river_ledger` (Gates 15 / 15.1), the rummy
proof `games/meldfall_ledger` (Gates 19 / 19.1 / 19.2), and the 8F/forward-v1 governance — are
all implemented. This brief commissions the **next** spec on the ladder — **Gate 20 — Star Halma
/ Chinese Checkers** — which is the **first large-topology, perfect-information (no-hidden-cards)
N-seat board game** of the public scaling phase. It is **not** a revisit of any shipped game or
helper, and **must not re-recommend any already-shipped work — the trick-taking games and helpers,
River Ledger, Meldfall Ledger, or the 8F/forward-v1 governance — as if it were missing.** Build on
them as exemplars and comparison baselines; the trick-taking helpers and the rummy/meld mechanics
are explicitly **not** reused (Star Halma has no tricks, no cards, no melds, no hidden hands).

---

## 2. Read in full (authority order)

Read these in this order before producing. The repository tree in the manifest is ground truth;
read the **entire `docs/**` tree** (the explicit floor for this task), plus `templates/**`,
`specs/README.md`, and the planning and delta-baseline artifacts below. Each line states why the
file is load-bearing *for the Gate 20 spec*.

**Tier 1 — constitution & authority**
- `docs/README.md` — the authority order and the layering rule every section of the spec must respect.
- `docs/FOUNDATIONS.md` — the constitution: product priority, §11 universal invariants, §12 stop conditions, §13 ADR triggers; every spec decision must satisfy these.

**Tier 2 — area law the gate engages (read the whole `docs/**`; these are the load-bearing ones)**
- `docs/ARCHITECTURE.md` — workspace shape, dependency direction, action/view/effect/replay/determinism model the new game crate must fit; the renderer/effect path the large board surface uses.
- `docs/ENGINE-GAME-DATA-BOUNDARY.md` — the exact `engine-core` / `game-stdlib` / `games/*` / static-data boundary; **all board-topology nouns (board, space, peg, marble, hole, cell, coordinate, adjacency, jump, path, home, target, track, graph, node, edge) stay in the game crate, never `engine-core`.** Topology may be typed content/parameters, but path legality and capture/blocking behavior are Rust.
- `docs/OFFICIAL-GAME-CONTRACT.md` — what makes a game official: §3 requirements-first workflow, §4 rules-research/source-notes format, §5 original RULES.md prose, §6 rule-coverage matrix, §10 UI-exposure obligations, §11 required trace set, §12 acceptance checklist the spec must map deliverables to.
- `docs/MECHANIC-ATLAS.md` — primitive-pressure law and **the single most load-bearing document for this gate's hard-gate decision**: §4 first/second/**third-use rule**, §5 hard-gate decision options, §5A promotion-conformance lifecycle, §5B parallel mechanical-scaffolding check, **§9A "Next-phase armed interlocks" — the `Star Halma and Pachisi-family race` row: "graph/track topology, route networks, jump/path validation, capture/safety semantics … Compare against prior board-space and graph-map decisions. Topology may be typed content; path legality and capture/safety behavior remain Rust."**, **§10 atlas table — specifically the `fixed 2D occupancy / board-space identity` row (the promoted `game-stdlib::board_space` primitive, with `frontier_control`/`event_frontier` already audited not-applicable) and the `graph-map topology / adjacency legality / connectivity scoring` row (`repeated-shape candidate` for `frontier_control` + `event_frontier`, "hard-gate before a third close graph helper shape")**, §10A **empty** debt register, and §10B deferred/candidate registers. **This gate is the structural third use of graph/topology/adjacency pressure — the third-use hard gate fires and MUST be resolved in-spec (see §3.7).**
- `docs/MULTI-SEAT-AND-SURFACE-CONTRACT.md` — **the variable-N seat law for this gate**: §2 seat-range declaration (Star Halma is **variable, research-pinned — expect supported {2,3,4,6}, default to be pinned**); §3 "Roles, Teams, and Partnerships" (declare **absent** — the base game is an individual competitive race); §5 viewer matrix; §6 pairwise no-leak matrix (**all board state is public — record that no per-seat private datum exists**); §7 public-observer rules (the entire board is public); §8 surface budgets (**the largest official board fixture — the 121-space star — is the budget driver**); §10 effect grouping (move/jump-chain effects); §11 "Outcomes and Final Breakdowns" (per-seat finish order/rank, terminal trace summaries); §13 seat-keyed simulator summaries; §14 Gate 15+ spec/ticket minimums.
- `docs/AI-BOTS.md` — bot law: §2 levels (**L0 random-legal required; L3 deterministic search is PERMITTED for perfect-information games** — Star Halma is perfect-information, so a bounded search bot is allowed); §3 v1/v2 exclusions (**no MCTS/ISMCTS/Monte Carlo/ML/RL — these remain forbidden even though the game is perfect-information**); the competent-player → strategy-evidence-pack gate for any L2 authored heuristic or L3 search. **Jump-chain / race / blocking bot policy is first-use bot territory** — no prior official game reasons about long hop chains toward a home target on a public board.
- `docs/UI-INTERACTION.md` — legal-only interaction, Rust-safe previews, effect-driven animation, replay UI, accessibility — the GAME-UI deliverable's law; the **large 121-space star board with multi-hop jump-path construction** is the new presentation surface, and the ROADMAP exit criterion "renderer performance and accessibility are proven for the largest official board fixture" lives here.
- `docs/TESTING-REPLAY-BENCHMARKING.md` — §1 test taxonomy, §3 golden-trace Trace Schema v1 obligations, §4 replay determinism, §5 determinism hazards, **§8 visibility/no-leak and §8.1 N-seat no-leak taxonomy / §8.2 export-coverage — which here resolve to an explicit "all surfaces public, no hidden-information class exists" audit (declared, not silent), not a private-hand matrix**, **§12 mechanic-primitive third-use/back-port tests (load-bearing — the graph-topology third-use hard-gate decision needs the §12 evidence)**, §15 provisional budgets (the **large-board branching-factor / pathfinding / playout-throughput** pressure), §17 CI expectations.
- `docs/EVIDENCE-FIXTURE-CONTRACT.md` — the canonical evidence/fixture contract (consolidated by 8M) the spec's acceptance-evidence section instantiates; fixture-profile and completion-profile obligations.
- `docs/TRACE-SCHEMA-v1.md` — the golden-trace schema fields Gate 20 traces must populate (load-bearing because Star Halma adds **single-step move, multi-hop jump-chain, blocked/no-move, reach-home, and terminal finish-order** traces; large board-state snapshots).
- `docs/ROADMAP.md` — §1 crosswalk, the **Gate 20 ladder row ("Star Halma / Chinese Checkers family — 121-space star board, long jump chains, multi-seat spatial race; larger board/topology proof; topology/path/jump helper hard gate")** and the **Gate 20 exit block** ("official seat variants, 121-space topology, move/jump chains, blocked-path behavior, win conditions, replay, serialization, and benchmarks are covered; renderer performance and accessibility are proven for the largest official board fixture; topology/path helper pressure is resolved before the next topology-dependent gate"), §2 per-stage requirements and the **per-gate debt-review obligation** (mechanic-atlas pressure, mechanical-scaffolding debt, trace debt, fixture-profile debt, seat/viewer grammar debt, replay/hash debt, evidence-receipt blockers — each named or `not applicable`). The prescriptive ladder law this spec realizes; note Gate 20 is the **purpose stated as "topology/path pressure without hidden cards."**
- `docs/IP-POLICY.md` — naming/original-presentation rules for a public-domain game: §5 common-vs-neutral names (whether "Chinese Checkers"/"Star Halma"/"Halma" may be used or must be coined-neutral), §4/§4A/§11 source-notes + IP-evidence-receipt checklist, §6 original prose/assets (no proprietary board/peg trade dress), §12 release checklist.
- `docs/AGENT-DISCIPLINE.md` — coding-agent law: bounded tasks, forbidden changes, failing-test protocol the spec's work breakdown must respect.
- `docs/SOURCES.md` — researched bibliography + Rulepath lessons; the spec's GAME-SOURCES deliverable extends this convention.
- `docs/WASM-CLIENT-BOUNDARY.md` — Rust/WASM-to-browser contract, operation groups, replay safety, dev-panel whitelist the web-exposed game must obey.
- `docs/MECHANICAL-SCAFFOLDING-REGISTER.md` — the scaffolding register and **the reuse-first audit shape Gate 20 must run as the THIRD `forward-v1` user**: the lawful shared homes, the C-01…C-10 scaffolding catalog, the **Non-Promotion List (note "Graph, topology, adjacency, movement, reachability, connectivity … stays behavioral unless the mechanic atlas records a narrow promoted primitive")**, the "Forward Per-Game Maintenance Cadence" (pre/post-implementation checkpoints), the "Automatic Prior-Game Refactor Trigger", and the per-new-game audit record fields.
- `docs/adr/0008-mechanical-scaffolding-governance.md` — the accepted mechanical-scaffolding governance **including the 2026-06-25 forward-obligation extension (Unit 8F)**: the standing per-new-game reuse-first audit, first-use registration, queue-or-dispose prior-game refactor, and Gate 1 CI receipt (`ci/scaffolding-audits.json`, `forward-v1` required for future games). Gate 18 was the first game admitted under this rule, Gate 19 the second; **Gate 20 is the third.**
- `docs/adr/0009-replay-fixture-hash-taxonomy.md` — the accepted replay/fixture/export/hash taxonomy v2 governing any trace/fixture/hash surface the new game introduces (no blanket golden regeneration; bounded authorized exports only).
- `docs/adr/0007-next-public-scaling-phase-and-gate-p-tail.md` — the accepted ADR that admits the public scaling phase (Gates 15–23) and pins Gate 20's place in it.
- (Read the remaining `docs/**` — `archival-workflow.md`, ADRs `0001`/`0002`/`0003`/`0005`/`0006`, `adr/0004-hidden-info-replay-export-taxonomy.md`, `adr/ADR-TEMPLATE.md` — as the explicit `docs/**` floor; they shape benchmark gating, archival, and ADR conventions the spec touches at closeout. **`adr/0004-hidden-info-replay-export-taxonomy.md` is likely `not applicable` to a perfect-information game — state that explicitly rather than silently omitting it. `adr/0006-blackjack-lite-roadmap-placement.md`: do not accidentally re-open it.**)

**Tier 3 — planning artifacts**
- `specs/README.md` — the living spec index and progress tracker; carries the **determination evidence** (active-epoch table: Gates 19 / 19.1 / 19.2 `Done`; Gate 20 the lowest non-`Done` row at Order 11; §10A debt empty; all predecessors closed), the 12-section **spec format**, the new-game **mechanical-scaffolding reuse-first audit / register-update / prior-game-retrofit** requirement (a spec is incomplete when those fields are silent), and the author workflow (`/reassess-spec` → `/spec-to-tickets` happens *after* the spec, not in it).
- `templates/**` — the per-game template set the spec's deliverables instantiate: `GAME-SOURCES.md`, `GAME-RULES.md`, `GAME-RULE-COVERAGE.md`, `GAME-MECHANICS.md`, `GAME-IMPLEMENTATION-ADMISSION.md`, `GAME-HOW-TO-PLAY.md`, `COMPETENT-PLAYER.md`, `BOT-STRATEGY-EVIDENCE-PACK.md`, `GAME-AI.md`, `GAME-UI.md`, `GAME-BENCHMARKS.md`, `GAME-EVIDENCE.md`, `PRIMITIVE-PRESSURE-LEDGER.md`, `PUBLIC-RELEASE-CHECKLIST.md`, `AGENT-TASK.md`, `README.md`. The spec's Deliverables section names which of these Star Halma fills.

**Tier 4 — delta baselines (sibling specs to mirror, not rebuild)**

*Mechanic baselines (the board / topology family — this gate's true ancestry):*
- `archive/specs/gate-7-draughts-lite-compound-action-tree.md` — the **movement / capture / forced-continuation / compound-action-tree** exemplar and a `game-stdlib::board_space` consumer; the closest prior game for **multi-step path/jump construction as an action tree** and for board-coordinate handling. Mirror its action-tree path construction; Star Halma's hop chains are the analogue of draughts capture chains. (Movement/capture there is game-local — do not assume a shared helper exists.)
- `archive/specs/gate-7-1-board-space-primitive-back-port.md` — **the structural-fork precedent**: how a promoted board primitive's conformance/back-port was handled (there, as a *separate* interlock gate). Read it to understand the board-space primitive's scope and why this gate audits `board_space` **not-applicable** for a non-rectangular star rather than reusing it, and what a promotion-with-back-port would have looked like had one been earned.
- `archive/specs/gate-13-frontier-control-asymmetric-area-control-proof.md` and `archive/specs/gate-14-event-frontier-event-complexity-capstone.md` — **the two prior graph-topology uses** (uses 1 and 2 of the `graph-map topology / adjacency legality / connectivity scoring` shape) that Star Halma's third-use hard gate must compare against. Read their `PRIMITIVE-PRESSURE-LEDGER.md` topology entries: both use **named site/edge graphs** (irregular maps), not a regular geometric star board with hex-adjacency and jump chains — the differences are the crux of the hard-gate decision. Both also already audited `board_space` **not-applicable**; reuse that reasoning shape.

*Section-anatomy / governance baselines (mirror structure; family-specific mechanics do NOT carry over):*
- `archive/specs/gate-17-vow-tide-oh-hell-bidding-trick-taking.md` — the **variable-N seat** spec (3–7 seats) and the structural template for declaring a supported seat set, setup diagnostics, and by-seat simulator summaries. Mirror its variable-N seat handling; its trick-taking mechanics do **not** carry over.
- `archive/specs/gate-18-blackglass-pact-spades-partnership-trick-taking.md` — the **first `forward-v1` audit user**: read it for the `forward-v1` reuse-first audit pattern, the register/CI-receipt closeout, and the 12-section spec anatomy. Mirror its forward-v1 audit + closeout sections; its partnerships/teams do **not** apply (Star Halma declares teams **absent**).
- `archive/specs/gate-19-meldfall-ledger-five-hundred-rummy.md` — the **most recent sibling** (plus `gate-19-1-*` and `gate-19-2-*` follow-ons): mirror its 12-section anatomy and its position as a `forward-v1` user. **Family pivot: it is a rummy/card game — its meld/tableau/draw-discard/cards/hidden-hand mechanics do NOT carry over to a perfect-information board game.** Use it for *shape*, not for *mechanics*.
- `archive/specs/gate-0-repository-skeleton.md` — the canonical 12-section spec example referenced by `specs/README.md`.

### Code seams to inspect directly (*inspect in the repo, not read-fully; not pasted here*)
- `games/draughts_lite/src/**` — origin/destination movement, capture, **same-piece forced continuation**, compound action-tree construction, and `board_space` use. The closest model for Star Halma's single-step move + **multi-hop jump-chain** legal-action tree and board-coordinate handling.
- `games/frontier_control/src/**` and `games/event_frontier/src/**` (+ their `docs/PRIMITIVE-PRESSURE-LEDGER.md`) — the two prior graph-topology implementations: how typed sites/edges constrain movement legality, how adjacency/traversal is computed in Rust, and the recorded `board_space` not-applicable + graph-topology `repeated-shape candidate` reasoning the hard gate extends.
- `games/vow_tide/src/**` — the **variable-N seat-range** implementation: how a multi-seat game declares its supported seat set, validates seat count at setup, and emits by-seat summaries. Star Halma mirrors the variable-N plumbing (not the trick-taking behavior).
- `games/meldfall_ledger/src/**` and `games/meldfall_ledger/docs/{PRIMITIVE-PRESSURE-LEDGER.md,GAME-EVIDENCE.md}` — the **most recent new-game crate** and the second forward-v1 audit evidence: the crate module layout, the ledger format for first-use entries, and the GAME-EVIDENCE completion profile to mirror (mechanics do not carry).
- `crates/game-stdlib/src/**` — confirm the lawful shared homes; **inspect `board_space`** (the promoted rectangular-coordinate primitive — confirm it does **not** fit a non-rectangular star, supporting the not-applicable audit) and the `seat` helpers (seat-count/range/ring arithmetic that variable-N setup may reuse). Confirm **no graph/topology/path/adjacency helper exists** (the hard gate decides whether to create one). The `trick_taking` module is **not** reused.
- New-crate registration seams (same pattern Vow Tide / Blackglass Pact / Meldfall Ledger followed): `crates/wasm-api/src/lib.rs` (game import + dispatch) and `crates/wasm-api/src/constants.rs` (game id + display-name constants); `tools/simulate/src/main.rs` (game const + match arm + bot dispatch); `tools/{replay-check,fixture-check,rule-coverage}/src/main.rs` (confirm generic vs game-specific registration); `apps/web` catalog + `apps/web/README.md` (intro catalog list, Shell Surface renderer list, Smoke Layers `smoke:e2e` list, enforced by `scripts/check-catalog-docs.mjs`).
- **Forward-v1 governance receipt seams:** `ci/scaffolding-audits.json` (the audit-receipt file — already carries the frozen 17-game legacy set plus the Gate 18 and Gate 19 `forward-v1` entries; Gate 20 must add its **own `forward-v1`** entry) and `scripts/check-scaffolding-governance.mjs` (the checker that enforces it at Gate 1). Inspect both to author the audit deliverable correctly.

---

## 3. Settled intentions (final — do not re-open)

These decisions were locked in Session 1. They pre-empt every clarifying question.

1. **The next spec is Gate 20 — Star Halma / Chinese Checkers.** This determination is **settled**;
   your job is to *confirm-and-document* it (citing the evidence below) and then **write the spec** —
   not to re-decide what comes next, and not to propose a maintenance detour or a different gate. The
   evidence that fixed it, which the spec's Objective/Sequencing sections must cite:
   - **Gate 19 — Five Hundred Rummy** (`games/meldfall_ledger`) and its follow-ons **19.1 / 19.2**
     are `Done` (2026-06-26 / 2026-06-27) per `specs/README.md` — the **last** predecessors.
   - `docs/MECHANIC-ATLAS.md` §10A open-promotion-debt register is **empty** (reviewed 2026-06-26) —
     so the "close open promotion debt before the next mechanic-ladder gate" interlock does **not**
     block, and **no separate debt-closure spec is needed first**.
   - Gate 20 — Star Halma / Chinese Checkers is the **lowest non-`Done`** unit on the
     `specs/README.md` active-epoch tracker (Order 11), and **every** listed predecessor (8M, 8C,
     8C-R1…R4, 8F, Gates 15–19.2, accepted ADRs 0007/0008/0009) is closed.
   - `docs/ROADMAP.md` admits Gate 20 as ladder law.

2. **Do not re-emit shipped work; this is a mechanic-family pivot.**
   The trick-taking lane (`plain_tricks`/`briar_circuit`/`vow_tide`/`blackglass_pact`), the
   `game-stdlib::trick_taking` helpers, the betting exemplar `river_ledger`, the rummy proof
   `meldfall_ledger`, and the 8F/forward-v1 governance are **implemented exemplars and comparison
   baselines**, not gaps to fill. **Star Halma has no cards, no tricks, no follow-suit, no trump, no
   bidding, no melds, no draw/discard, and no hidden hands** — none of those helpers or mechanics are
   reused. The reusable inheritance is the **non-card plumbing and the board/topology lineage**:
   variable-N seat handling (Vow Tide), compound action-tree path construction + board-coordinate
   handling (Draughts Lite), graph-topology adjacency/traversal patterns to *compare against*
   (Frontier Control / Event Frontier), and the new-crate registration + forward-v1 audit pattern
   (Blackglass Pact / Meldfall Ledger).

3. **PERFECT-INFORMATION game — no hidden cards, no private seat state (a defining family
   characteristic).** Per the ROADMAP Gate 20 purpose ("topology/path pressure **without hidden
   cards**"), the entire board, every peg position, the active seat, and the move history are
   **public**. The spec must therefore:
   - declare in `docs/MULTI-SEAT-AND-SURFACE-CONTRACT.md` §6 terms that **no per-seat private datum
     exists** and the §8.1/§8.2 no-leak obligation resolves to an **explicit "all surfaces public,
     no hidden-information class"** audit — *declared with rationale, never silently omitted*;
   - mark `docs/adr/0004-hidden-info-replay-export-taxonomy.md` **`not applicable`** (no redacted
     export class) with a one-line rationale;
   - still run the standard no-leak harness as a **confirming** pass (proving nothing seat-private
     leaks because nothing seat-private exists), exactly as a public game records the §8.1 result.

4. **Variant locked: classic six-pointed-star Chinese Checkers / Star Halma — a single peg-set race.**
   The spec locks the **121-space six-pointed star board**, one set of pegs/marbles per seat starting
   in one point of the star, **single-step moves to adjacent empty spaces plus chains of hops over a
   single adjacent occupied space into the empty space beyond** (the characteristic jump chain, which
   may mix directions and which the player may stop at any legal point), **no capture** (hopped pegs
   are not removed), and a **win on getting all of a seat's pegs into the opposite (target) home
   point**. You must **deep-research the canonical Chinese Checkers / Star Halma ruleset and its
   common variants and research-PIN the exact parameters inside the spec, with sources**, choosing
   one canonical Rulepath variant and documenting any deliberate deviation in the GAME-SOURCES-style
   notes the spec calls for. At minimum pin:
   - the **board topology / coordinate model** — the 121-hole star geometry, adjacency degree (six
     directions), and how a coordinate/space identity is represented as **typed content** (not in
     `engine-core`);
   - **pegs per seat** (classic 10 per home point) and **starting/target home assignment** by seat
     count;
   - the **move rules** — single-step adjacency moves, hop-over-adjacent-occupied jump definition,
     **multi-hop chaining** (whether a chain may change direction; whether the player may stop
     mid-chain), and whether a turn is exactly one move-or-chain;
   - **blocking / no-legal-move** handling and any "must vacate own/target home" or
     "swap"/"no-permanent-block" rules some variants use;
   - the **win condition** (all pegs in the opposite home) and any **tie / finish-order** rule for
     3+ seats (the race continues for remaining places, or terminates at first-home — pin one), plus
     any **turn/ply limit or draw** safeguard for replay/benchmark termination;
   - any **hop-the-corner / out-of-turn** edge rules you adopt or exclude.

5. **Seat range locked: variable, exact set research-pinned.** Classic Chinese Checkers supports
   **2, 3, 4, and 6** seats (the star geometry excludes 5; some variants extend to other counts) —
   research-pin and declare the supported set and a default per `docs/MULTI-SEAT-AND-SURFACE-CONTRACT.md`
   §2, validating seat count at setup with diagnostics (mirror Vow Tide's variable-N declaration).
   Seat-keyed simulator summaries (§13) and benchmarks run across the supported set. (If you judge a
   `{2,3,4,6}` set canonical, lock it and justify; if you include other counts, source the variant.)

6. **Teams declared ABSENT — individual competitive race.** Per
   `docs/MULTI-SEAT-AND-SURFACE-CONTRACT.md` §3, declare roles/teams/partnerships **absent**: the base
   Gate 20 game is an individual spatial race, each seat races its own pegs, with a per-seat
   finish-order outcome. **Do not** carry over any partnership/team machinery from Blackglass Pact.
   The well-known **partnership variant** (opposite seats teaming in 4/6-seat play) is recorded **only
   as a sourced note / explicitly out-of-scope future option** in the GAME-SOURCES/RULES notes — not
   implemented, not in the seat declaration, not in the exit criteria. §11 outcomes are **per-seat**
   finish order / rank.

7. **Graph-topology / path-jump pressure is the THIRD official use → the third-use hard gate fires and
   is resolved INLINE in this spec.** `frontier_control` (Gate 13) and `event_frontier` (Gate 14) are
   uses 1 and 2 of the `graph-map topology / adjacency legality / connectivity scoring` shape
   (`docs/MECHANIC-ATLAS.md` §10, `repeated-shape candidate`, "hard-gate before a third close graph
   helper shape"); Star Halma's regular star-board adjacency + jump-path legality is the third use.
   **Resolve the hard gate in-spec** (the standard §4/§5 ledger process performed in the game's
   `PRIMITIVE-PRESSURE-LEDGER.md`, as Gates 13/14/17 did) — **do NOT author a separate preceding
   interlock spec, and do NOT hand the decision back open-endedly.** The decision is yours to make and
   document via research + the ledger fields, not to defer to the user. The in-spec resolution MUST:
   - **audit the promoted `game-stdlib::board_space` primitive `not applicable`** for the
     non-rectangular six-pointed star (no rectangular dimensions / row-major `rNcM` identity), mirroring
     the `frontier_control`/`event_frontier` not-applicable reasoning, with rationale and evidence link;
   - **write the third-use primitive-pressure ledger entry** for the graph/topology/adjacency/path-jump
     shape, comparing Star Halma against the two named-site graphs and against board-space, and **decide
     exactly one** of reuse / promote-narrow-helper / defer-reject / ADR. The **default and expected
     posture** per §9A is **topology = typed content + path/jump legality + blocking + win detection
     stay game-local Rust** (i.e. defer/reject promotion, or at most promote a *narrow behavior-free*
     coordinate/adjacency-iteration helper that owns no legality) — but you must justify the chosen
     option with the §6/§8 ledger fields and §12 evidence, not assert it;
   - if (and only if) you promote a narrow behavior-free helper, either **conform same-gate** the prior
     games the atlas identifies as matching, **or** queue a **named follow-on tracker unit** in
     `specs/README.md` per the forward-governance queue-or-dispose rule, and record promotion debt per
     §5A — never leave a silent promotion;
   - introduce **no `engine-core` board/space/peg/graph/node/edge/adjacency/path/topology noun**, and
     encode **no path legality, jump rule, blocking rule, or win condition in static data** (topology
     parameters are typed content only).

8. **Gate 20 is the THIRD `forward-v1` reuse-first scaffolding audit user (8F governance).** The spec
   must run the forward reuse-first audit **inline, before implementation admission**, per
   `docs/adr/0008-*.md` (the 2026-06-25 forward extension), `docs/MECHANICAL-SCAFFOLDING-REGISTER.md`,
   and the 8F spec: (a) audit the scaffolding register (C-01…C-10) and lawful shared homes for reusable
   behavior-free scaffolding (effect-envelope constructors, canonical seat-ID grammar, seat-count/range
   validation + ring arithmetic, action-tree encoding/hash v1, stable-byte writer, etc.) — a `not
   applicable` result requires a rationale, never silence; (b) **register-new** any newly invented
   behavior-free scaffolding shape on first use (`candidate` / `local-only` / `rejected`; first use does
   **not** authorize promotion; **graph/topology/adjacency/movement is on the Non-Promotion List —
   board-topology *behavior* is not scaffolding and stays in the mechanic-atlas lane / game-local**, not
   the register); (c) **queue-or-dispose** any prior-game refactor the new scaffolding exposes — a named
   bounded tracker unit in `specs/README.md` **or** an accepted `local-only`/`deferred`/`rejected`
   register disposition with rationale, owner, evidence, and next review trigger; and (d) add the
   **`forward-v1`** audit receipt to `ci/scaffolding-audits.json`, enforced by
   `scripts/check-scaffolding-governance.mjs` at Gate 1. The spec's Scope/Deliverables/Acceptance-Evidence
   sections are **incomplete** if the reuse-first audit, expected register updates, prior-game retrofit
   disposition, and the `forward-v1` receipt are silent. (Keep the §3.7 mechanic-atlas hard-gate decision
   distinct from this §3.8 scaffolding audit — they are parallel obligations, not the same audit.)

9. **Bot policy: perfect-information → L0 required, bounded L3 permitted, no Monte Carlo.** Require an
   **L0 random-legal bot** floor. Because the game is perfect-information, a **deterministic bounded
   search bot (L3)** is *permissible* per `docs/AI-BOTS.md` §2 — but **MCTS / ISMCTS / Monte Carlo / ML
   / RL remain forbidden** (§3), so any search must be bounded/authored (e.g. depth-limited heuristic
   minimax-style or an authored race/jump-chain heuristic). Any **L2 authored** or **L3 search** policy
   needs a competent-player analysis + strategy-evidence pack first (`COMPETENT-PLAYER.md` →
   `BOT-STRATEGY-EVIDENCE-PACK.md`). Jump-chain/race/blocking bot reasoning is first-use; research-pin
   competent-player strategy. The spec must state the **bot floor** (L0 required) and the bot ceiling
   it scopes, with the evidence-pack gate for anything above L0.

10. **Foundation-amendment posture: Gate 20 is a *user* of the foundation set, not an amender.** The
    8F/forward-v1 governance, ADRs 0008/0009/0007, the multi-seat contract, and the trace/evidence
    taxonomy are already in place; Gate 20 **consumes** them. The spec therefore performs
    **documentation UPDATES** — `docs/MECHANIC-ATLAS.md` (the third-use graph-topology hard-gate
    resolution row + the `board_space` not-applicable audit; §10A stays empty unless the hard gate
    earns a promotion, in which case record it correctly), `docs/MECHANICAL-SCAFFOLDING-REGISTER.md`
    (forward-v1 audit + any register-new), `docs/SOURCES.md`, the `specs/README.md` status flip, and
    `apps/web/README.md` catalog/smoke surfaces — **not foundation amendments**. Amend a foundation doc
    **only if a genuine gap forces it** (e.g. if the 121-space public board exposes a §8 surface-budget
    or large-board-renderer gap not covered by the existing contract); if so, flag the amendment
    explicitly in a dedicated spec subsection, justify it, and honor the supersession rule (a divergence
    from a foundation principle requires an accepted ADR naming the affected section — never silent
    redefinition). `assumption:` no foundation amendment is expected.

11. `assumption:` the deliverable is the **spec only** — it is *not* decomposed into `tickets/`
    AGENT-TASK packets. Ticket decomposition happens separately afterward via the repo's
    `/reassess-spec` then `/spec-to-tickets` workflow (`specs/README.md` "Workflow"). The spec's work
    breakdown should enumerate **candidate** AGENT-TASK items with dependency order, as the sibling
    specs do, without writing the tickets themselves.

12. `assumption:` **Neutral game name — bounded delegation to you.** Rulepath ships original,
    evocative neutral names rather than the source game's name (River Ledger ← Texas Hold'Em, Crest
    Ledger ← poker, Briar Circuit ← Hearts, Vow Tide ← Oh Hell, Blackglass Pact ← Spades, **Meldfall
    Ledger ← Five Hundred Rummy**). **Coin an original, IP-safe neutral name for this Star Halma /
    Chinese Checkers implementation** consistent with that catalog convention, keep "Chinese Checkers"
    / "Star Halma" / "Halma" only as the rules-family label in source/IP notes, and **derive the game
    module id (snake_case) and the spec filename slug from it.** Per `docs/IP-POLICY.md` §5, keeping a
    common name may be *permissible*, but the established convention is to coin a neutral name, so coin
    one and use it, documenting the naming rationale in the spec's source/IP notes (and noting, as the
    siblings did, that the coinage does not replace the human IP/legal review IP-POLICY requires). The
    spec filename is `specs/gate-20-<neutral-slug>-star-halma.md`; if you judge a neutral coinage
    genuinely unjustified, fall back to `specs/gate-20-star-halma.md`.

---

## 4. The task

Produce the **Gate 20 (Star Halma / Chinese Checkers) implementation spec** — a **new** roadmap-gate
spec — that turns `docs/ROADMAP.md`'s Gate 20 row ("Star Halma / Chinese Checkers family — 121-space
star board, long jump chains, multi-seat spatial race; topology/path/jump helper hard gate") into one
concrete, reviewable, foundation-aligned plan. This is a **new-spec** deliverable. The spec must
(a) confirm-and-document the Gate 20 determination with the evidence in §3.1; (b) lock the classic
six-pointed-star Chinese Checkers / Star Halma single-peg-set-race variant (single-step + multi-hop
jump-chain moves, no capture, reach-opposite-home win) and research-pin its exact parameters with
sources; (c) define the new `games/<neutral_id>` crate, its filled official-game documents, and its
registration across tools/WASM/web-catalog surfaces; (d) scope the **variable-seat, teams-absent,
perfect-information** model to the MULTI-SEAT §2/§6/§11 contract and the TESTING §8.1/§8.2 taxonomy
**as an explicit all-public, no-hidden-information-class audit** (the entire board is public);
(e) **resolve the graph-topology/path-jump third-use hard gate INLINE** — audit `board_space`
not-applicable, write the third-use primitive-pressure ledger entry comparing against
`frontier_control`/`event_frontier`, decide reuse/promote/defer-reject/ADR (default: topology = typed
content, legality stays Rust), and queue-or-conform any promotion (§3.7); (f) run the **third
`forward-v1` reuse-first scaffolding audit** with register-new, queue-or-dispose, and the CI receipt
(§3.8); (g) scope the **L0 floor + bounded-L3-permitted, no-Monte-Carlo** bot policy with the
evidence-pack gate (§3.9); and (h) map every deliverable to the OFFICIAL-GAME-CONTRACT,
MULTI-SEAT-AND-SURFACE-CONTRACT, TESTING/EVIDENCE, UI-INTERACTION (large-board renderer + accessibility),
and AI-BOTS obligations, following the `specs/README.md` 12-section spec format and mirroring
`archive/specs/gate-7-draughts-lite-compound-action-tree.md` (compound move/jump action trees +
board coordinates), `archive/specs/gate-13-frontier-control-asymmetric-area-control-proof.md` /
`archive/specs/gate-14-event-frontier-event-complexity-capstone.md` (graph-topology hard-gate
comparison), `archive/specs/gate-17-vow-tide-oh-hell-bidding-trick-taking.md` (variable-N seat
plumbing), and `archive/specs/gate-18-blackglass-pact-spades-partnership-trick-taking.md` /
`archive/specs/gate-19-meldfall-ledger-five-hundred-rummy.md` (forward-v1 audit + 12-section closeout
anatomy).

---

## 5. Exploration & online-research mandate

Explore the repository as deeply as needed beyond the files listed above. **Research online as deeply
as needed** — the canonical Chinese Checkers / Star Halma ruleset and its common variants (the 121-hole
six-pointed-star geometry and adjacency; pegs per seat and home assignment by seat count of 2/3/4/6;
the single-step move and the hop-over-adjacent-occupied jump definition; multi-hop chaining rules,
direction changes, and stop-anywhere; no-capture; blocking / no-legal-move handling and any
swap/no-permanent-block rule; the reach-opposite-home win condition and 3+-seat finish-order/tie
handling; the ancestral square-board **Halma** ruleset and how Chinese Checkers descends from it),
public-domain rules sources suitable for an original prose summary, prior open-source **Chinese Checkers
/ Halma / peg-board engine implementations** and how they model **board topology / coordinate systems
for a star or hex board, adjacency, jump-chain enumeration, and move generation**, research or write-ups
on **Chinese Checkers / Halma strategy** for a competent-player analysis and any L1/L2/bounded-L3 bot
policy (**without proposing any MCTS/ISMCTS/Monte Carlo/ML/RL approach — those are forbidden in public
v1/v2 even though the game is perfect-information**; favor authored race/jump-chain heuristics or
depth-bounded search), and accessibility/UX prior art for presenting a **large 121-space star board with
multi-hop jump-path construction and previews** for a variable-2-to-6-seat game (the ROADMAP "renderer
performance and accessibility are proven for the largest official board fixture" criterion). Also
research the **graph/topology representation question** that the third-use hard gate turns on — how
comparable engines separate behavior-free coordinate/adjacency iteration from path-legality behavior —
to ground the reuse/promote/defer decision. Cite sources for any external claim that shapes a decision
in the spec (variant parameters, board geometry, win/finish rules, naming/IP rationale, strategy
posture, the topology hard-gate rationale). The deep research is **your** job; do it thoroughly.

---

## 6. Doctrine & constraints (honor these in every spec decision)

- `docs/FOUNDATIONS.md` is the constitution — every product-behavior decision must satisfy its §11
  universal invariants and clear its §12 stop conditions; a genuine divergence requires an accepted ADR
  superseding the affected principle first ("supersede only by accepted ADR"), never designing against it
  silently. The spec is **subordinate** to the foundation set and must not redefine or override any
  foundation contract (§10 posture: updates, not amendments).
- Authority order: foundation docs govern area docs govern specs govern tickets. Where the spec and a
  foundation document disagree, the foundation document wins.
- `engine-core` stays generic and **noun-free** — no `board`, `space`, `peg`, `marble`, `hole`, `cell`,
  `coordinate`, `adjacency`, `jump`, `path`, `home`, `target`, `track`, `graph`, `node`, `edge`, etc.;
  typed mechanic nouns belong in `games/*` first, shared helpers in `game-stdlib` only via the mechanic
  atlas. **All board-topology / path / jump / win behavior stays game-local** (the third-use hard gate
  default); the `board_space` primitive is audited not-applicable; the `trick_taking` helpers are not
  reused.
- **TypeScript never decides legality.** Legal actions (legal single-step moves, legal jump targets,
  legal multi-hop chains, stop-chain, terminal/win detection), validation, effects, views, previews
  (move/jump-path highlighting), and bot decisions all come from Rust/WASM. The browser must not compute
  adjacency, enumerate jumps, validate paths, or decide a win.
- **No YAML and no DSL without an accepted ADR.** Static data is typed content / parameters / metadata
  only — never selectors, conditions, or triggers; **the board topology may be typed content, but no
  path legality, jump rule, blocking rule, or win condition is encoded in data.**
- **Determinism**: deterministic setup/seat assignment, deterministic move generation order, replay,
  hashes, serialization order, and traces stay deterministic (or are explicitly migrated under ADR 0009
  with trace notes — no blanket golden regeneration). Provide a turn/ply-limit or equivalent safeguard so
  simulations/benchmarks terminate.
- **Perfect-information game — no hidden-information class** (§3.3): the entire board, peg positions,
  active seat, and history are public. There is **no per-seat private datum**; the no-leak obligation is
  an explicit "all surfaces public" audit, and ADR 0004 is `not applicable`. The no-leak harness still
  runs as a confirming pass.
- **No MCTS / ISMCTS / Monte Carlo / ML / RL bots** in public v1/v2 — even though Star Halma is
  perfect-information. L0 random-legal is required; bounded L3 deterministic search is *permitted*; any
  L2/L3 policy needs a competent-player analysis + strategy-evidence pack first.
- **No proprietary board/peg trade dress; original prose and assets only** (`docs/IP-POLICY.md`); coin a
  neutral name (§3.12) and keep "Chinese Checkers"/"Star Halma"/"Halma" only as the rules-family label.
- **Never delete or weaken tests to get green** — follow the failing-test protocol
  (`docs/AGENT-DISCIPLINE.md` §4).
- **The forward-v1 scaffolding governance is mandatory** (§3.8): the reuse-first audit, register-new,
  queue-or-dispose, and the `ci/scaffolding-audits.json` `forward-v1` receipt are gate obligations, not
  optional. Gate 20 is the third forward-v1 user.
- **The graph-topology third-use hard gate must be resolved in-spec** (§3.7), distinct from the
  scaffolding audit: audit `board_space` not-applicable, write the ledger entry, decide one option, and
  conform-or-queue any promotion — never advance with an unresolved third use.
- The spec is **authored, not decomposed** (§3.11): enumerate candidate AGENT-TASKs; do not write
  tickets.

---

## 7. Deliverable specification

Produce exactly **one downloadable markdown document**:

- **`specs/gate-20-<neutral-slug>-star-halma.md`** — a **new** file (not a replacement).
  `<neutral-slug>` is derived from the neutral name you coin (§3.12); fall back to
  `specs/gate-20-star-halma.md` only if you keep a common name. This is the `new-spec` pipeline
  deliverable: after download, the user saves it to that `specs/` path, `/reassess-spec` reassesses it
  in place, then `/spec-to-tickets` decomposes it — so author it as a complete, decompose-ready spec,
  **not** pre-decomposed into tickets.

It MUST follow the **12-section spec format** defined in `specs/README.md` ("Spec format"), mirroring
`archive/specs/gate-7-draughts-lite-compound-action-tree.md`,
`archive/specs/gate-17-vow-tide-oh-hell-bidding-trick-taking.md`,
`archive/specs/gate-18-blackglass-pact-spades-partnership-trick-taking.md`, and
`archive/specs/gate-19-meldfall-ledger-five-hundred-rummy.md`:
1. Header (Spec ID, stage, gate, status `Planned`/`Not started`, date, owner, authority order; plus the
   game-identity fields the sibling specs carry — internal game id, public display name, rules-family
   label, variant id, trace rules version, data/manifest version, browser-impl-required flag, **official
   seat declaration: variable seats** (research-pinned supported set, e.g. {2,3,4,6}; min/max/default;
   seat labels; **teams/partnerships = absent**), public-observer stance, **information model =
   perfect-information / no hidden class**, bot floor, kernel stance, primitive stance, delivery posture);
2. Objective (sourced from the ROADMAP Gate 20 row; cite the §3.1 determination evidence; state the
   **perfect-information** posture, the **topology third-use hard-gate** posture, and that the
   **trick-taking/rummy helpers are not reused**);
3. Scope (in / out / **not allowed** — carry the ROADMAP Gate 20 prohibitions and the
   public-scaling-phase "not allowed" list; topology/path/jump/win stays game-local, no data legality;
   partnership variant out-of-scope);
4. Deliverables (the new `games/<neutral_id>` crate tree; the filled official-game documents from
   `templates/**` including `PRIMITIVE-PRESSURE-LEDGER.md` and `GAME-EVIDENCE.md`; registration across
   `simulate`/`replay-check`/`fixture-check`/`rule-coverage`, WASM, and the `apps/web` catalog; **the
   `forward-v1` entry in `ci/scaffolding-audits.json`**);
5. Work breakdown (bounded **candidate** AGENT-TASK items with dependency order — **including the
   topology third-use hard-gate ledger decision as a gating prerequisite item**, the **`forward-v1`
   reuse-first scaffolding audit as a gating prerequisite item**, a board-topology/coordinate-content
   item, a move-generation (single-step) item, a jump-chain enumeration item, a win/finish-order
   detection item, a large-board renderer + accessibility item, and the new first-use/topology
   primitive-pressure ledger items);
6. Exit criteria (mapped row-for-row to the ROADMAP Gate 20 exit block — official seat variants,
   121-space topology, move/jump chains, blocked-path behavior, win conditions, replay, serialization,
   benchmarks; **renderer performance and accessibility proven for the largest official board fixture**;
   and **topology/path helper pressure recorded and resolved/deferred** — plus by-seat
   simulation/benchmarks and the scaffolding-audit receipt);
7. Acceptance evidence (command suite; the test-taxonomy table; the **all-surfaces-public no-leak audit**
   (no hidden-information class; ADR 0004 N/A) standing in for the §8.1 pairwise matrix; the golden-trace
   minimum set per OFFICIAL-GAME-CONTRACT §11 / TESTING §6 — including **single-step move, multi-hop
   jump-chain, blocked/no-move, reach-home, and terminal finish-order** traces; **large-board benchmark
   expectations (branching factor, jump-chain enumeration, playout throughput, serialization/replay
   overhead, renderer perf for the 121-space fixture)**; the `forward-v1` scaffolding-audit CI receipt;
   the EVIDENCE-FIXTURE-CONTRACT completion profile; **the §12 third-use mechanic-primitive evidence for
   the topology hard-gate decision**);
8. FOUNDATIONS & boundary alignment (principles engaged; the **perfect-information no-hidden-class
   stance** (§3.3), the **topology third-use hard-gate in-spec resolution + `board_space` N/A** (§3.7),
   the explicit **no-reuse of trick-taking/rummy helpers**, and the **forward-v1 audit** (§3.8); §12 stop
   conditions);
9. Forbidden changes (gate-specific prohibitions — no `engine-core` board/space/peg/graph/node/edge/path
   noun, no path/jump/win/blocking helper promotion unless the hard gate earns a narrow behavior-free one
   with conformance, no static-data legality, no partnership/team machinery, no TypeScript legality, no
   MCTS/ISMCTS/Monte Carlo/ML/RL bot);
10. Documentation updates required (the `specs/README.md` status flip; `docs/SOURCES.md`;
    **`docs/MECHANIC-ATLAS.md`** — the third-use graph-topology hard-gate resolution row + the
    `board_space` not-applicable audit, §10A only if a promotion is earned;
    **`docs/MECHANICAL-SCAFFOLDING-REGISTER.md`** — the forward-v1 audit record + any register-new entry +
    prior-game disposition; game-local docs; **`apps/web/README.md`** intro catalog list + Shell Surface
    renderer list + Smoke Layers `smoke:e2e` list, per OFFICIAL-GAME-CONTRACT §10/§12 and
    `scripts/check-catalog-docs.mjs`; **the foundation-amendment posture statement** per §3.10 — "none
    expected; updates only" unless a genuine large-board surface gap is flagged);
11. Sequencing (predecessor Gates 19 / 19.1 / 19.2 `Done`; this is the third `forward-v1` audit user;
    successor Gate 21 Pachisi-family race; admission rule; note Gate 20 resolves the topology pressure the
    successor track-topology gate depends on);
12. Assumptions (one-line-correctable — including the carried `assumption:` lines from §3.10/§3.11/§3.12).

Use explicit `not applicable` rows over silent omissions (notably the hidden-information / ADR 0004 N/A
rows). You may include implementation appendices (core rules, board-topology/coordinate model, move +
jump-chain model, win/finish model, bot policy, replay model, WASM specifics, benchmark operations,
sources) as the sibling specs do, but keep them inside the single spec file.

> Produce the deliverable directly as a downloadable markdown document. Do not interview, do not ask
> clarifying questions — the requirements above are final. If a genuine contradiction makes a
> requirement impossible, state it in the deliverable and proceed with the most faithful interpretation.

---

## 8. Self-check (run against your own output before returning)

- [ ] The spec **confirms-and-documents** Gate 20 as next with the §3.1 evidence (Gates 19/19.1/19.2
      `Done`; §10A empty; lowest non-`Done` Order 11; ROADMAP admits it); it does **not** re-open
      "what's next" or propose a different gate / maintenance detour / preceding interlock spec.
- [ ] It locks the **classic six-pointed-star Chinese Checkers / Star Halma single-peg-set race**
      (single-step + multi-hop jump-chain moves, no capture, reach-opposite-home win) and
      **research-pins** exact parameters with sources (board geometry/adjacency, pegs per seat + home
      assignment, move + jump-chain + stop rules, blocking / no-legal-move, win + finish-order, turn/ply
      safeguard).
- [ ] It coins an original, IP-safe **neutral name** (or justifies a common-name fallback) and derives
      the module id and spec filename slug from it.
- [ ] It declares **variable seats** (research-pinned supported set, e.g. {2,3,4,6}; min/max/default)
      with **teams/partnerships absent** per MULTI-SEAT §2/§3, records the partnership variant as
      out-of-scope sourced note, and uses **seat-keyed** simulator summaries (§13) and benchmarks.
- [ ] It treats the game as **perfect-information**: declares **no per-seat private datum / no
      hidden-information class**, marks **ADR 0004 `not applicable`**, and runs the no-leak harness as an
      explicit all-public confirming audit (never a silent omission).
- [ ] It **resolves the graph-topology/path-jump third-use hard gate IN-SPEC**: audits
      `game-stdlib::board_space` **not-applicable** for the non-rectangular star, writes the third-use
      primitive-pressure ledger entry comparing against `frontier_control`/`event_frontier`, decides
      exactly one of reuse/promote/defer-reject/ADR (default: topology = typed content, legality stays
      Rust), and conforms-or-queues any promotion — with **no** `engine-core`
      board/space/peg/graph/path noun and **no** legality/jump/win rule in static data.
- [ ] It does **not** reuse the `game-stdlib::trick_taking` helpers or any card/meld/hidden-hand
      mechanic, and does not re-emit any shipped trick-taking/rummy work.
- [ ] It runs the **third `forward-v1` reuse-first scaffolding audit** (register C-01…C-10 + lawful
      homes), **register-new** on first use (keeping graph/topology behavior OUT of the register per the
      Non-Promotion List), **queue-or-dispose** any prior-game refactor, and adds the **`forward-v1`
      receipt** to `ci/scaffolding-audits.json` enforced by `scripts/check-scaffolding-governance.mjs`;
      the Scope/Deliverables/Acceptance-Evidence sections are not silent on the audit; the §3.7 hard-gate
      decision and the §3.8 scaffolding audit are kept distinct.
- [ ] It requires an **L0 random-legal bot** floor, permits a **bounded L3 deterministic search** for
      this perfect-information game while **forbidding MCTS/ISMCTS/Monte Carlo/ML/RL**, and gates any
      L2/L3 policy behind a competent-player analysis + strategy-evidence pack.
- [ ] It proves **renderer performance and accessibility for the largest official board fixture** (the
      121-space star) per the ROADMAP exit and UI-INTERACTION, and scopes large-board benchmarks.
- [ ] Every deliverable maps to OFFICIAL-GAME-CONTRACT §3/§5/§6/§10/§11/§12, the MULTI-SEAT/TESTING/
      EVIDENCE/UI-INTERACTION/AI-BOTS obligations, and the `templates/**` set; the **12-section spec
      format** is fully present with `not applicable` used over silent omission.
- [ ] The **foundation-amendment posture** is documentation-updates-only (§3.10); any foundation
      amendment is explicitly flagged and ADR-justified, never silent; `assumption:` none is expected.
- [ ] No spec decision weakens an upstream foundation doc or silently amends an accepted ADR
      (0004/0007/0008/0009).
- [ ] The deliverable is **one spec file**, authored (not decomposed into tickets).
- [ ] Commit `b3e7efd` contains every file named in §2 (the manifest is that tree).

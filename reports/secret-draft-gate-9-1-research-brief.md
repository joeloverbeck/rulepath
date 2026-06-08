# Research brief — Gate 9.1 `secret_draft` spec (+ small Gate 9 aftermath cleanup spec)

> **You are ChatGPT-Pro Session 2.** This prompt is final and self-contained. Produce
> the deliverables directly as downloadable markdown documents. **Do not interview, do not
> ask clarifying questions** — the requirements below were already settled with the repo
> owner in a prior session that had full repository access. If a genuine contradiction
> makes a requirement impossible, state it inside the deliverable and proceed with the most
> faithful interpretation.

---

## 1. Context

The uploaded manifest (`manifest_2026-06-08.txt`) is the path inventory of the
`joeloverbeck/rulepath` repository — a Rust-first, rule-enforcing, replayable, testable
card/board-game platform where **Rust owns all behavior and TypeScript/React present only**.
The foundation docs are an ordered, layered authority indexed by `docs/README.md`:
`FOUNDATIONS.md` (the constitution) → `ARCHITECTURE.md` → `ENGINE-GAME-DATA-BOUNDARY.md`
→ the area docs → `ROADMAP.md`; earlier documents govern later ones, and accepted ADRs
supersede them only by explicitly naming the affected sections.

**Fetch every file from commit `65ec79d403e8481b439b1908332c263c73e1d002` (branch `main`).**
The uploaded manifest is exactly that commit's tree (`git ls-tree -r --name-only HEAD`). If
any referenced report cites a different "commit of record," note the divergence and use this
verified HEAD, not the report's string. (The repo working tree at authoring time had a few
uncommitted presentation-only token_bazaar UI fixes and skill edits that are **not** part of
this task; the manifest deliberately reflects the committed HEAD, not the dirty tree.)

### Where the repository is right now

- **Gates 0 through 9 are complete.** `specs/README.md` is the living progress index;
  every gate row through **Gate 9 (`token_bazaar`)** reads `Done`.
- **`docs/MECHANIC-ATLAS.md` shows no open promotion debt** (§10A register is empty), so the
  next mechanic-ladder advancement is *not* blocked by a back-port interlock.
- The just-completed **Gate 9 `token_bazaar`** spec
  (`archive/specs/gate-9-token-bazaar-browser-proof.md`) implemented the *public
  resource/economy* half of ROADMAP §11 and **deliberately deferred** the *simultaneous
  commitment / reveal / drafting* half (`secret_draft`) to "a dedicated successor gate,"
  with an explicit recommendation: **"Gate 9.1, immediately after Gate 9 and before
  Gate 10 (`poker_lite` betting)"** — because poker_lite's imperfect-information and waiting
  UX depend on the commitment/reveal proof landing first. That successor spec does not yet
  exist. **Authoring it is the primary task of this brief.**

---

## 2. Read in full (authority order)

Read every file below **in full, in this order**, before producing anything. Each line says
why it is load-bearing for *this* task.

**Foundation set (authority flows downward):**

- `docs/README.md` — the authority order and the layering rule the deliverables must respect.
- `docs/FOUNDATIONS.md` — the constitution: product-priority order, **§11 universal
  invariants**, **§12 stop conditions**, **§13 ADR triggers**. Every decision in both
  deliverables must satisfy these.
- `docs/ARCHITECTURE.md` — the action / public-private-view / semantic-effect / replay /
  determinism model. Note the effect taxonomy that already lists **"commitments/reveals,
  pending responses, grouped batches"** and **"simultaneous/reveal batches"** — `secret_draft`
  extends this envelope; it does not invent a new kernel concept.
- `docs/ENGINE-GAME-DATA-BOUNDARY.md` — the exact `engine-core` / `game-stdlib` / `games/*`
  / static-data boundary, the noun-free kernel rule, and the forbidden behavior-in-data
  vocabulary. Commitment/draft/reveal nouns live in `games/secret_draft`, never the kernel.
- `docs/OFFICIAL-GAME-CONTRACT.md` — what makes a game "official" and the requirements-first
  deliverable set (rules → sources → coverage → mechanics → tests → bots → UI). The
  `secret_draft` spec's Deliverables and Acceptance sections must cover this set.
- `docs/MECHANIC-ATLAS.md` — the primitive-pressure ledger. §10B lists **"simultaneous
  commitment/reveal"** as a *candidate after second use* and **"reaction window/pending
  response"** as `ADR-required if generalized broadly`. The spec must keep `secret_draft`
  logic **local** and add a first-use atlas note — **no `game-stdlib` promotion**.
- `docs/AI-BOTS.md` — bot law, levels, and hidden-information safety. The `secret_draft` bot
  must choose its commitment from its **own allowed private view only**, never sampling the
  opponent's hidden pick; rationale must be viewer-safe.
- `docs/UI-INTERACTION.md` — public visual target, **legal-only** controls, Rust-owned
  previews, **§10 effect-driven animation including "simultaneous/reveal batches"**, **§12
  hidden-information safety** (no hidden state in DOM / `data-testid` / storage / replay), and
  the **pending-seat / waiting-state** UX the gate must prove.
- `docs/TESTING-REPLAY-BENCHMARKING.md` — test taxonomy (unit / rule / golden / property /
  simulation / replay / serialization / **visibility/no-leak**), deterministic replay/hash
  discipline, and benchmark/CI expectations.
- `docs/ROADMAP.md` — the prescriptive ladder. **§11 (Gate 9)** carries the two exit lines
  `secret_draft` inherits ("simultaneous choices remain hidden until reveal"; "UI shows
  pending seats without leaking choices"); the stage-8 ladder row names `secret_draft`. This
  is law; do **not** edit it to record progress.
- `docs/IP-POLICY.md` — original prose/assets and non-IP naming; the designed ruleset, names,
  and labels must be original and free of proprietary drafting-game text/trade-dress.
- `docs/AGENT-DISCIPLINE.md` — coding-agent law: bounded tasks, **forbidden changes**, the
  **failing-test protocol** ("never delete or weaken tests to get green"). The spec's
  Work breakdown and Forbidden-changes sections inherit this.
- `docs/SOURCES.md` — the researched bibliography and Rulepath lessons; route any new
  drafting / simultaneous-selection prior-art citations here (and into the game's
  `SOURCES.md`).
- `docs/WASM-CLIENT-BOUNDARY.md` — the Rust/WASM→browser client contract, operation groups,
  replay-export safety, and the dev-panel data whitelist. `secret_draft`'s hidden commitment
  must stay redacted across every operation group.
- `docs/TRACE-SCHEMA-v1.md` — the trace schema. Commitments and reveals must serialize
  deterministically and reproduce hashes **without leaking the hidden choice** before reveal.
- `docs/archival-workflow.md` — how completed specs/tickets are archived; context for the
  spec lifecycle and where finished specs eventually move.
- `docs/adr/0001-stage-1-random-playout-budget.md`,
  `docs/adr/0002-ci-benchmark-gating-lanes.md`,
  `docs/adr/0003-ci-calibrated-benchmark-thresholds.md`,
  `docs/adr/0004-hidden-info-replay-export-taxonomy.md`,
  `docs/adr/0005-variance-aware-ci-benchmark-floors.md`,
  `docs/adr/0006-blackjack-lite-roadmap-placement.md`,
  `docs/adr/ADR-TEMPLATE.md` — accepted decisions the spec must honor and **must not silently
  amend**. **ADR 0004 (hidden-info replay-export taxonomy)** is directly load-bearing for a
  commitment/reveal game; the benchmark ADRs (0002/0003/0005) govern the benchmark/CI lanes
  the spec must register into; **ADR 0006** explains why Blackjack stays deferred (do not
  resurrect it).

**Progress index, templates, and sibling specs:**

- `specs/README.md` — the living spec index and progress tracker; the canonical 12-section
  spec format; the workflow ("pick the lowest non-`Done` gate; close promotion debt first").
  Both deliverables add a row here (via their own "Documentation updates required" section —
  you describe the row, you do not need to regenerate the index file).
- `templates/` — **read all of them.** `templates/AGENT-TASK.md`, `GAME-RULES.md`,
  `GAME-MECHANICS.md`, `GAME-RULE-COVERAGE.md`, `GAME-SOURCES.md`, `GAME-AI.md`,
  `GAME-UI.md`, `GAME-BENCHMARKS.md`, `GAME-IMPLEMENTATION-ADMISSION.md`,
  `BOT-STRATEGY-EVIDENCE-PACK.md`, `COMPETENT-PLAYER.md`, `PUBLIC-RELEASE-CHECKLIST.md`,
  `PRIMITIVE-PRESSURE-LEDGER.md`, `README.md`. The `secret_draft` spec's Deliverables must
  enumerate the per-game `docs/*` set instantiated from these templates (Gate 9 produced
  eleven game docs; mirror that).
- `archive/specs/gate-9-token-bazaar-browser-proof.md` — **the sibling spec.** Its
  "Sequencing" and "Candidate placement after Gate 9" sections contain the explicit Gate 9.1
  `secret_draft` recommendation and the deferred exit lines; its overall shape, depth, and
  section set are the **template to match** for the `secret_draft` spec.
- `archive/specs/gate-7-2-and-gate-8-high-card-duel-hidden-info-chance-proof.md` — the
  hidden-information / deterministic-chance / private-view / no-leak proof that
  `secret_draft` **extends**. Its commitment/reveal machinery is the closest existing pattern.
- `archive/specs/gate-8-aftermath-roadmap-realignment.md` — **the precedent for the second
  deliverable** (the small cleanup spec): a non-feature truthfulness/realignment pass. Match
  its section set, its "validated already-correct — do not touch" discipline, and its tight
  scope.

**Code seams to inspect directly (read in the repo; not pasted here):**

- `games/high_card_duel/src/*` — the commitment/reveal/private-view/visibility model to
  extend: `state.rs` (`Phase`, `commitments`), `rules.rs` (commit legality), `effects.rs`
  (`commit_face_down_effect`, `cards_revealed_effect`), `visibility.rs` (viewer redaction),
  `bots.rs` (private-view-only bot). This is the most relevant existing game.
- `games/token_bazaar/src/*` and `games/token_bazaar/{data,tests,docs,benches}/*` — the
  **newest** official game; its file-for-file layout is what a new `games/secret_draft`
  crate should mirror.
- `crates/engine-core/src/*` — confirm the noun-free actor/turn/effect/replay envelopes and
  exactly what simultaneous/grouped-effect support already exists; the spec must state the
  kernel stance (no new noun, no behavior smuggled in).
- `crates/wasm-api/src/lib.rs` — per-game WASM registration surface and the redaction path.
- `tools/simulate`, `tools/replay-check`, `tools/fixture-check`, `tools/rule-coverage`
  (and `bench-report`, `seed-reducer`, `trace-viewer` if they enumerate game ids) — the
  hardcoded game-id registration seams the spec must list.
- `apps/web/src/components/*` (`GamePicker.tsx`, `AppShell.tsx`, `ActionControls.tsx`,
  `EffectLog.tsx`, `effectFeedback.ts`, and the per-game board components, e.g.
  `TokenBazaarBoard.tsx`), `apps/web/src/state/shellReducer.ts`, `apps/web/src/wasm/client.ts`,
  and `apps/web/e2e/*.smoke.mjs` — the presentation + pending-seat/no-leak smoke wiring a new
  `SecretDraftBoard` and `secret-draft.smoke.mjs` would extend.
- `.github/workflows/gate-1-game-smoke.yml` and `.github/workflows/gate-2-benchmarks.yml` —
  the native-smoke and benchmark lanes the spec must register `secret_draft` into.
- `apps/web/README.md` — **the cleanup target** for the second deliverable (see §3.4 / §7).

---

## 3. Settled intentions (final — these make this brief locked)

These decisions were resolved with the repo owner. Treat them as committed, not optional.

### 3.1 The primary deliverable is the Gate 9.1 `secret_draft` spec

Author the implementation spec that turns the deferred ROADMAP §11 commitment/reveal/drafting
half into a concrete, reviewable plan — placed as **Gate 9.1**, after Gate 9 and before
Gate 10. This is the next progress spec; it is **not** a ticket decomposition (see §3.5).

### 3.2 You design the concrete original ruleset

The Gate 9 spec itself proposed a full original ruleset for `token_bazaar`; do the same here.
**Design `secret_draft`'s concrete, original rules yourself**, grounded in researched
drafting / simultaneous-selection prior art, within these **hard constraints**:

- **Two seats** by default (`seat_0`, `seat_1`), matching the entire current portfolio. You
  *may* propose a small fixed N>2 **only** if simultaneous drafting genuinely needs it and
  you justify the determinism/UI/bot cost; otherwise keep it 2-seat.
- **Fully deterministic** — deterministic setup and (if any) deterministic shuffle/order via
  the existing seeded RNG discipline; **no** random-during-play surprise that cannot be
  replayed from seed + command log. Reuse the repository's existing RNG contracts; do **not**
  add an `engine-core` RNG noun (follow the Gate 9 RNG decision precedent).
- **Original / non-IP** — original game name (working id `secret_draft`; pick a clear neutral
  public display name), original card/token/label prose, no proprietary drafting-game rules,
  names, icons, or trade dress (`docs/IP-POLICY.md`).
- **Simultaneous hidden commitment + synchronized reveal** — each seat privately commits a
  choice; the choice is redacted from the opponent (and from every browser payload, DOM,
  storage, replay export, and bot view) until **all seats have committed**, then a single
  grouped **reveal** batch resolves them together. This is the core proof.
- **Drafting with removal** — picks come from a shared, visible pool; a taken item leaves the
  pool; the draft proceeds over a fixed number of rounds.
- **Fixed round/turn cap**, **public deterministic scoring**, and **public deterministic
  tie-breaks** (mirror the rigor of Gate 9's tie-break ladder).
- **Pending-seat waiting UX** — the UI must show that a seat *has committed* without leaking
  *what* it committed, and must present the synchronized reveal as an effect-driven batch.

Keep the design **small and proof-shaped**, not an economy/draft engine. The point is to
prove the architecture handles simultaneous commitment, synchronized reveal, waiting states,
and drafting — not to ship a deep commercial design.

### 3.3 `secret_draft` is a delta on proven machinery — no new kernel concept, no promotion

- It **extends** `high_card_duel`'s commitment/reveal/private-view/redaction envelope and
  `ARCHITECTURE.md`'s existing "commitments/reveals, pending responses, grouped batches /
  simultaneous-reveal batches" effect shapes.
- **No `engine-core` noun** (`draft`, `pick`, `commit`, `reveal`, `pool`, `hand`, `card`,
  `deck`, etc.) — these stay in `games/secret_draft`.
- **No `game-stdlib` promotion.** The atlas marks simultaneous-commitment/reveal a *candidate
  after second use*; this is the **first** official simultaneous-commitment game. Implement
  locally and add a first-use atlas note. If a helper feels unavoidable, the spec instructs
  the implementer to **stop and write a primitive-pressure ledger entry first** — the expected
  Gate 9.1 answer is still local implementation.
- If you conclude the commitment/reveal/simultaneous-resolution shape genuinely requires a
  kernel/visibility/replay-semantics change (a real **§13 ADR trigger**), do **not** design
  against the foundation silently: state it explicitly in the spec, identify the affected
  sections, and recommend an ADR as a prerequisite — but first prove it is not already
  covered by `high_card_duel`'s pattern.

### 3.4 The second deliverable is a separate small "Gate 9 aftermath" cleanup spec

Repo exploration confirmed the post-Gate-9 living docs are **almost entirely truthful** — root
`README.md`, `progress.md`, `docs/SOURCES.md`, `docs/ROADMAP.md`, `docs/MECHANIC-ATLAS.md`,
`specs/README.md`, both CI workflows, all four game tools, and `crates/wasm-api` already name
`token_bazaar` / record Gate 9 `Done`. **The one stale file is `apps/web/README.md`**, which
omits `token_bazaar` in three places:

1. the intro browser-games list (ends at `high_card_duel`);
2. the **Shell Surface** board-renderer list (names only five renderers, omitting the Token
   Bazaar board);
3. the **Smoke Layers** `smoke:e2e` description (lists only four games; omits `token_bazaar`,
   though `apps/web/e2e/token-bazaar.smoke.mjs` exists and CI already chains it).

Author a **small, separate** `gate-9-aftermath` cleanup spec mirroring
`gate-8-aftermath-roadmap-realignment.md`: a non-feature truthfulness pass scoped to
`apps/web/README.md`. **Verify these findings yourself during exploration** and, if you find
additional genuinely-stale living docs, fold them in — but apply the aftermath precedent's
discipline of an explicit **"validated already-correct — do not touch"** list so the pass
does not re-litigate already-correct docs or grow into feature work. (You may also note, as
an optional in-scope hygiene item, the standing `apps/web/README.md` remark that the
`directional_flip` E2E file exists but is "not chained by `smoke:e2e`" — only if your reading
confirms it is genuinely inconsistent; do not invent CI changes.)

### 3.5 Specs only — no ticket decomposition

Both deliverables are **specs**, not tickets. Do **not** produce `AGENT-TASK` packets,
`tickets/*`, or a per-diff decomposition. The Work-breakdown section should list **bounded
candidate AGENT-TASKs in dependency order** (as the Gate 9 spec does), but decomposing them
into ticket files is a later, separate step.

---

## 4. The task

Produce two **new-spec** markdown documents for the Rulepath repository. The primary one is
the **Gate 9.1 `secret_draft` implementation spec**: a concrete, reviewable plan — grounded in
`docs/ROADMAP.md` §11 and the full foundation set — for an original, deterministic,
two-player, browser-playable simultaneous-commitment / synchronized-reveal / drafting game
that proves hidden simultaneous choice, waiting states, and grouped reveal while keeping all
behavior in Rust, the kernel noun-free, and no hidden information leaking anywhere. The
secondary one is a **small, separate Gate 9 aftermath cleanup spec** that makes
`apps/web/README.md` tell the truth about `token_bazaar`. Both follow the canonical Rulepath
spec format and satisfy every governing foundation document.

---

## 5. Exploration + online-research mandate

Explore the repository as deeply as needed beyond the files listed in §2 — read the actual
`games/high_card_duel` and `games/token_bazaar` sources, the engine-core envelopes, the WASM
surface, the web components, the CI lanes, and `apps/web/README.md` — to ground every claim in
the real tree rather than assumption. **Research online as deeply as needed** — drafting and
simultaneous-selection game designs, simultaneous-move / commitment-reveal handling in game
frameworks (e.g. OpenSpiel's treatment of simultaneous-move and imperfect-information games),
and any prior art that sharpens an *original* ruleset and a clean determinism/reveal model.
**Cite every external source** that shapes a design decision, and route the citations into the
spec's sources note and `docs/SOURCES.md` guidance. Keep all designed content original — use
prior art for vocabulary and structural ideas, never for copied rules, names, or prose.

---

## 6. Doctrine & constraints (honor all that the task engages)

- `docs/FOUNDATIONS.md` is the constitution — every product-behavior decision must satisfy its
  **§11 universal invariants** and clear its **§12 stop conditions**; a genuine divergence
  requires an accepted ADR superseding the affected principle first ("supersede only by
  accepted ADR"), never designing against it silently.
- **Authority order**: foundation docs govern area docs govern specs govern tickets. If a
  design choice conflicts with architecture or foundation, the design is wrong.
- `engine-core` stays generic and **noun-free** — no `draft`, `pick`, `commit`, `reveal`,
  `pool`, `card`, `deck`, `hand`, `board`, `grid` nouns; typed mechanic nouns live in
  `games/secret_draft` first, shared helpers in `game-stdlib` only via the mechanic atlas.
- **TypeScript never decides legality.** Legal actions, validation, previews, effects, views,
  reveal timing, scoring, tie-breaks, and bot decisions all come from Rust/WASM.
- **No YAML and no DSL without an accepted ADR.** Static data is typed content / parameters /
  metadata only — never selectors, conditions, triggers, or payment/scoring formulas.
- **Determinism**: replay, hashes, RNG, serialization order, and traces stay deterministic
  (or are explicitly migrated with a trace note). Commitments/reveals reproduce from seed +
  command log.
- **No hidden-information leaks** into views, action trees, previews, effect logs,
  diagnostics, UI metadata, DOM attributes, `data-testid` values, local storage, replay
  exports, dev panels, bot explanations, or candidate rankings — **before reveal**, the
  committed choice is invisible to the opponent seat and every payload (`ADR 0004`,
  `UI-INTERACTION.md` §12).
- **No MCTS / ISMCTS / Monte Carlo / ML / RL bots**, and **no hidden-state sampling** by bots
  — the `secret_draft` bot commits from its own allowed private view with a viewer-safe
  rationale.
- **Never delete or weaken tests to get green** — the spec carries the failing-test protocol
  (`AGENT-DISCIPLINE.md` §4).
- **No open promotion debt may be created and skipped**, and **no primitive is promoted** in
  this gate (first official simultaneous-commitment use → local, with a first-use atlas note).

---

## 7. Deliverable specification

Produce exactly **two** downloadable markdown documents. Both are **new files**; neither
replaces an existing file.

### Deliverable A (primary) — `specs/gate-9-1-secret-draft-commitment-reveal.md`

A complete Rulepath implementation spec carrying the **canonical 12-section set** (per
`specs/README.md` "Spec format" and matching the depth of
`archive/specs/gate-9-token-bazaar-browser-proof.md`):

1. **Header** table — Spec ID, roadmap stage (8), build gate (Gate 9.1), status (`Planned`),
   date, owner, primary crate / internal game id (`secret_draft`), public display name,
   browser implementation required, authority order.
2. **Objective** — sourced from ROADMAP §11 and the Gate 9 successor recommendation.
3. **Scope** — in scope / out of scope / **not allowed** (carry ROADMAP §11 "Not allowed"
   plus gate-local prohibitions).
4. **Deliverables** — the `games/secret_draft/{Cargo.toml, src/*, data/*, benches/*, tests/*,
   docs/*}` tree (mirroring the `token_bazaar` layout), the golden-trace set (including
   commit / simultaneous-reveal / pending-seat / no-leak / diagnostic / bot / WASM-export
   cases), the eleven `docs/*` instantiated from `templates/*`, the WASM + React board +
   `secret-draft.smoke.mjs` wiring, and the tool/CI registration points.
5. **Work breakdown** — bounded candidate AGENT-TASKs in dependency order (do **not** decompose
   into tickets).
6. **Exit criteria** — mapped **row-for-row** to ROADMAP §11, explicitly **claiming the two
   deferred lines** ("simultaneous choices remain hidden until reveal"; "UI shows pending
   seats without leaking choices") that Gate 9 carried forward.
7. **Acceptance evidence** — the full `OFFICIAL-GAME-CONTRACT.md` deliverable set: rule /
   property / replay / serialization / **visibility-no-leak** tests, golden traces, the
   `simulate`/`replay-check`/`fixture-check`/`rule-coverage` runs, benchmarks (smoke floors
   with a named calibration follow-up, per the benchmark ADRs), browser e2e smoke
   (human + bot + reveal + pending-seat + replay/export-import + reduced-motion + a11y/no-leak),
   and `boundary-check.sh` + `check-doc-links.mjs`.
8. **FOUNDATIONS & boundary alignment** — a principle/stance/rationale table covering §2
   behavior authority, §3 kernel, §4 `game-stdlib` earned (no promotion), §5 static data, §8
   public bots, §11 invariants (incl. hidden-info no-leak before reveal), §12 stop conditions,
   and a clear **§13 ADR-trigger** verdict (state whether the simultaneous-reveal model needs
   an ADR or is already covered by the `high_card_duel` pattern).
9. **Forbidden changes** — gate-specific prohibitions (no kernel nouns; no generic
   draft/commit/reveal/pool/bot-policy `game-stdlib` helper; no behavior-in-data; no TS
   legality/reveal-timing/scoring; no MCTS/ISMCTS/ML/RL/hidden-state sampling; no proprietary
   drafting content; no Blackjack resurrection; no accidental trace/hash migration).
10. **Documentation updates required** — incl. the `specs/README.md` Gate 9.1 row, the
    `docs/MECHANIC-ATLAS.md` first-use note (and §10B candidate update), `progress.md` and
    root `README.md` updates after implementation, and the eleven game docs; **do not** edit
    `docs/ROADMAP.md` to record progress.
11. **Sequencing** — predecessor Gate 9 (`Done`); successor Gate 10 (`poker_lite` /
    `plain_tricks`); admission rule (no open promotion debt).
12. **Assumptions** — one-line-correctable, including the 2-seat default and any design choice
    you want the owner to be able to override.

You **may** append an "Implementation reference" section below the canonical set (as the
Gate 9 spec does) carrying the full proposed rules, effect shapes, action-tree/commit-path
metadata, reveal/redaction model, bot policy, WASM/browser wiring, fixtures, golden-trace
list, and benchmark operations.

### Deliverable B (secondary, small) — `specs/gate-9-aftermath-roadmap-realignment.md`

A **small, non-feature** cleanup spec mirroring
`archive/specs/gate-8-aftermath-roadmap-realignment.md`, carrying the same canonical section
set at proportionate (smaller) depth. Scope: make `apps/web/README.md` truthful about
`token_bazaar` (the three omissions in §3.4), with an explicit **"validated already-correct —
do not touch"** list naming the post-Gate-9 docs/CI/tooling that exploration confirmed are
already correct, and a `specs/README.md` maintenance row for the pass. No gameplay code
changes; no primitive promotion; no archived-spec rewrites; no ticket creation.

### Locked / no-questions instruction

> Produce the deliverables directly as downloadable markdown documents. Do not interview, do
> not ask clarifying questions — the requirements above are final. If a genuine contradiction
> makes a requirement impossible, state it in the deliverable and proceed with the most
> faithful interpretation.

---

## 8. Self-check (run against your own output before returning)

- The deliverable set is **exactly** the two new files named in §7 — no tickets, no extra docs.
- The `secret_draft` spec carries all **12 canonical sections** and matches the depth/shape of
  the Gate 9 sibling spec; the aftermath spec mirrors the Gate 8 aftermath spec at smaller
  scale.
- The designed ruleset is **original, deterministic, 2-seat (unless justified), drafting +
  simultaneous hidden commitment + synchronized reveal**, with a fixed cap and public
  deterministic scoring/tie-breaks.
- **No hidden choice leaks before reveal** in any payload, DOM, `data-testid`, storage, replay
  export, dev panel, or bot rationale; the no-leak / visibility test obligations are explicit.
- **No `engine-core` noun**, **no `game-stdlib` promotion**; a first-use atlas note is required
  and any helper temptation is gated behind a primitive-pressure ledger entry.
- Determinism/replay/hash/serialization discipline is preserved; commitment/reveal reproduce
  from seed + command log; no trace/hash migration is introduced by accident.
- No deliverable weakens an upstream foundation doc or silently amends an accepted ADR; any
  genuine §13 ADR trigger is named, not designed around.
- Exit criteria are mapped row-for-row to ROADMAP §11 and explicitly claim the two deferred
  commitment/reveal lines.
- Every external claim that shaped a design decision is **cited**.
- The aftermath spec's findings were **verified against the real tree** and carry an explicit
  "do-not-touch" already-correct list.
- The §1 fetch-baseline commit `65ec79d403e8481b439b1908332c263c73e1d002` contains every file
  named in the §2 read-in-full list (it does).

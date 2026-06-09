# Research brief — Gate 10 `poker_lite` spec (betting / showdown / imperfect-information proof)

> **You are ChatGPT-Pro Session 2.** This prompt is final and self-contained. Produce the
> deliverable directly as a downloadable markdown document. **Do not interview, do not ask
> clarifying questions** — the requirements below were already settled with the repo owner in
> a prior session that had full repository access. If a genuine contradiction makes a
> requirement impossible, state it inside the deliverable and proceed with the most faithful
> interpretation.

---

## 1. Context

The uploaded manifest (`manifest_2026-06-08.txt`) is the path inventory of the
`joeloverbeck/rulepath` repository — a Rust-first, rule-enforcing, replayable, testable
card/board-game platform where **Rust owns all behavior and TypeScript/React present only**.
The foundation docs are an ordered, layered authority indexed by `docs/README.md`:
`FOUNDATIONS.md` (the constitution) → `ARCHITECTURE.md` → `ENGINE-GAME-DATA-BOUNDARY.md` →
the area docs → `ROADMAP.md`; earlier documents govern later ones, and accepted ADRs
supersede them only by explicitly naming the affected sections.

**Fetch every file from commit `85713751653411667f885fa13d7267a3d20280a8` (branch `main`).**
The uploaded manifest is exactly that commit's tree (`git ls-tree -r --name-only HEAD`); the
working tree at authoring time was clean. If any referenced report cites a different "commit
of record," note the divergence and use this verified HEAD, not the report's string.

### Where the repository is right now

- **Gates 0 through 9.1 are complete.** `specs/README.md` is the living progress index; every
  gate row through **Gate 9.1 (`secret_draft` / Veiled Draft)** reads `Done`.
- **`docs/MECHANIC-ATLAS.md` shows no open promotion debt** (§10A register is empty), so the
  next mechanic-ladder advancement is *not* blocked by a back-port interlock.
- **No "aftermath" cleanup spec is owed.** Unlike Gate 8 and Gate 9 (each of which spawned a
  separate `gate-N-aftermath-roadmap-realignment` truthfulness pass), Gate 9.1 *folded* the
  web-shell catalog reconciliation into the gate itself per the closeout rule now codified in
  `specs/README.md` §10. Repo verification at the baseline commit confirms `apps/web/README.md`
  (intro catalog list, Shell Surface renderer list, Smoke Layers `smoke:e2e` list), root
  `README.md`, `progress.md`, `specs/README.md`, and `docs/MECHANIC-ATLAS.md` **already name
  `secret_draft` / Veiled Draft and record Gate 9.1 `Done`**. Do **not** author an aftermath
  spec; if your own exploration nonetheless surfaces a genuinely stale living doc, note it
  inside the spec's "Documentation updates required" section rather than spinning up a second
  deliverable.
- **The next gate is Gate 10.** `archive/specs/gate-9-1-secret-draft-commitment-reveal.md`
  ("Sequencing") names the successor as **Gate 10 (`poker_lite` / `plain_tricks`)** and states
  it "should not start until Gate 9.1 evidence proves no-leak waiting/reveal behavior and no
  open promotion debt remains" — **both conditions are now met.** `docs/ROADMAP.md` §12
  (Gate 10) lists two build candidates: `poker_lite` (scoped betting/showdown) and
  `plain_tricks` (scoped lead/follow/trick scoring); the §1 stage/gate crosswalk places
  `poker_lite` at the lower ladder stage (stage 9) and `plain_tricks` at stage 10.
  **Authoring the Gate 10 `poker_lite` spec is the task of this brief.**

---

## 2. Read in full (authority order)

Read every file below **in full, in this order**, before producing anything. Each line says
why it is load-bearing for *this* task.

**Foundation set (authority flows downward):**

- `docs/README.md` — the authority order and the layering rule the deliverable must respect.
- `docs/FOUNDATIONS.md` — the constitution: product-priority order, **§2 behavior authority**,
  **§3 noun-free `engine-core`** (its forbidden-noun list explicitly includes `card`, `deck`,
  `hand`, `suit`, `trick`, `pot`, `betting`, `drafting`), **§8 public bots** (no
  MCTS/ISMCTS/Monte Carlo/ML/RL), **§11 universal invariants**, **§12 stop conditions**, and
  **§13 ADR triggers**. Every decision in the spec must satisfy these.
- `docs/ARCHITECTURE.md` — the action / public-private-view / semantic-effect / replay /
  determinism model. `poker_lite` betting rounds, pot accounting, and showdown are new
  *game-local* mechanics expressed through the existing action-tree / command-envelope /
  semantic-effect / viewer-safe-view envelopes; the spec must state the kernel stance (no new
  kernel concept).
- `docs/ENGINE-GAME-DATA-BOUNDARY.md` — the exact `engine-core` / `game-stdlib` / `games/*` /
  static-data boundary, the noun-free kernel rule, and the forbidden behavior-in-data
  vocabulary. `bet`, `raise`, `call`, `fold`, `pot`, `card`, `deck`, `hand`, `showdown` nouns
  live in `games/poker_lite`, never the kernel; betting amounts/showdown thresholds are typed
  content, never data-encoded formulas/selectors.
- `docs/OFFICIAL-GAME-CONTRACT.md` — what makes a game "official" and the requirements-first
  deliverable set (rules → sources → coverage → mechanics → tests → bots → UI). The spec's
  Deliverables and Acceptance sections must cover this set; note its §10/§12 web-shell catalog
  closeout obligations (enforced by `scripts/check-catalog-docs.mjs`).
- `docs/MECHANIC-ATLAS.md` — the primitive-pressure ledger. §10A is **empty** (no open
  debt); §10B records `deterministic shuffle / private hand / hidden commitment / reveal` as
  first-use `high_card_duel` with the note that card/deck/hand helpers "remain game-local until
  repeated pressure from later official card games **such as poker-lite**…" and `resource
  accounting` as first-use `token_bazaar` ("compare ledgers before third use"). `poker_lite` is
  the **second** official use of the card/private-hand shape and of resource/accounting — a
  *second-use compare*, **not** a third-use hard gate. The spec must add a first-use
  betting/pot atlas note and a second-use comparison for cards and accounting, and authorize
  **no `game-stdlib` promotion**.
- `docs/AI-BOTS.md` — bot law, levels, hidden-information safety, and the Level 2 strategy
  evidence workflow. The `poker_lite` bot must choose its action from its **own allowed private
  view only** (its hole card(s) + public board/pot), never sampling the opponent's hidden
  hand; explanations must be viewer-safe.
- `docs/UI-INTERACTION.md` — public visual target, **legal-only** controls, Rust-owned
  previews, effect-driven animation, hidden-information safety (no hidden state in DOM /
  `data-testid` / storage / replay), and the warm board-game-table aesthetic that explicitly
  **avoids casino vibes** — directly relevant to presenting a betting/showdown game without
  casino trade dress.
- `docs/TESTING-REPLAY-BENCHMARKING.md` — test taxonomy (unit / rule / golden / property /
  simulation / replay / serialization / **visibility-no-leak**), deterministic replay/hash
  discipline, and benchmark/CI expectations the spec must register into.
- `docs/ROADMAP.md` — the prescriptive ladder. **§12 (Gate 10)** carries `poker_lite`'s
  purpose ("imperfect-information accounting/bot proof"), its exit list ("betting/trick rules
  correct for chosen variants"; "pot/accounting … tests cover edge cases"; "bots finish games
  without hidden-state cheating"; "no public MCTS/ISMCTS"; "UI remains understandable";
  "native benchmarks exist") and its **"Variant scope MUST be written before coding"** mandate
  and "Not allowed" list (real-money/casino features, unbounded variants, hidden-state
  cheating, ML/RL, copied rules prose). This is law; do **not** edit it to record progress.
- `docs/IP-POLICY.md` — original prose/assets and non-IP naming. It flags `poker_lite` as a
  source-research/internal label and lists "blackjack, poker, … casino, betting, chip, table,
  payout, insurance" as **research descriptors, not default product presentation**. The
  designed name, prose, and UI must be original and casino-neutral.
- `docs/adr/0006-blackjack-lite-roadmap-placement.md` — the precedent: a casino-adjacent
  candidate (blackjack) was deferred and, if ever built, "SHOULD propose an original
  non-casino microgame with original naming, neutral presentation, no betting/chips/payouts."
  `poker_lite` is the first casino-adjacent game actually admitted to the ladder; the spec must
  honor this precedent's spirit (original neutral framing) **without** resurrecting blackjack.
- `docs/AGENT-DISCIPLINE.md` — coding-agent law: bounded tasks, **forbidden changes**, the
  **failing-test protocol** ("never delete or weaken tests to get green"). The spec's
  Work-breakdown and Forbidden-changes sections inherit this.
- `docs/SOURCES.md` — the researched bibliography and Rulepath lessons; route any new
  poker / betting / imperfect-information prior-art citations here (and into the game's
  `SOURCES.md`).
- `docs/WASM-CLIENT-BOUNDARY.md` — the Rust/WASM→browser client contract, operation groups,
  replay-export safety, and the dev-panel data whitelist. `poker_lite`'s hidden hole card(s)
  must stay redacted across every operation group until showdown.
- `docs/TRACE-SCHEMA-v1.md` — the trace schema. Deals, bets, and showdown must serialize
  deterministically and reproduce hashes **without leaking the hidden hand** before showdown.
- `docs/archival-workflow.md` — how completed specs/tickets are archived; context for the spec
  lifecycle and where finished specs eventually move.
- `docs/adr/0001-stage-1-random-playout-budget.md`,
  `docs/adr/0002-ci-benchmark-gating-lanes.md`,
  `docs/adr/0003-ci-calibrated-benchmark-thresholds.md`,
  `docs/adr/0004-hidden-info-replay-export-taxonomy.md`,
  `docs/adr/0005-variance-aware-ci-benchmark-floors.md`,
  `docs/adr/ADR-TEMPLATE.md` — accepted decisions the spec must honor and **must not silently
  amend**. **ADR 0004 (hidden-info replay-export taxonomy)** is directly load-bearing for a
  private-hand betting game; the benchmark ADRs (0001/0002/0003/0005) govern the
  playout-budget and benchmark/CI lanes the spec must register into.

**Progress index, templates, and sibling specs:**

- `specs/README.md` — the living spec index and progress tracker; the canonical **12-section
  spec format**; the workflow ("pick the lowest non-`Done` gate; close promotion debt first;
  check the atlas for open promotion debt before a new mechanic-ladder spec"); and the §10
  web-shell catalog closeout obligation a web-exposed game gate must name. The deliverable adds
  a Gate 10 row here (via its own "Documentation updates required" section — describe the row;
  you need not regenerate the index file).
- `templates/` — **read all of them.** `templates/AGENT-TASK.md`, `GAME-RULES.md`,
  `GAME-MECHANICS.md`, `GAME-RULE-COVERAGE.md`, `GAME-SOURCES.md`, `GAME-AI.md`, `GAME-UI.md`,
  `GAME-BENCHMARKS.md`, `GAME-IMPLEMENTATION-ADMISSION.md`, `BOT-STRATEGY-EVIDENCE-PACK.md`,
  `COMPETENT-PLAYER.md`, `PUBLIC-RELEASE-CHECKLIST.md`, `PRIMITIVE-PRESSURE-LEDGER.md`,
  `README.md`. The spec's Deliverables must enumerate the per-game `docs/*` set instantiated
  from these templates (the recent games produced eleven-to-twelve game docs; mirror that, and
  include `BOT-STRATEGY-EVIDENCE-PACK.md` + `COMPETENT-PLAYER.md` for the required Level 2 bot).
- `archive/specs/gate-9-token-bazaar-browser-proof.md` — **the depth/shape template to match**
  and the resource/accounting sibling. Match its overall structure, section depth, its
  "design the concrete original ruleset inside the spec" approach, and its appended
  implementation-reference section. Its accounting/scoring/tie-break rigor is the model for
  `poker_lite`'s pot accounting and showdown tie-breaks.
- `archive/specs/gate-9-1-secret-draft-commitment-reveal.md` — the most recent sibling spec and
  the precedent for **folding the web-catalog closeout into the gate** (see its "Documentation
  updates required"); also the commitment/reveal pattern relevant to `poker_lite`'s
  deal-then-reveal flow.
- `archive/specs/gate-7-2-and-gate-8-high-card-duel-hidden-info-chance-proof.md` — the
  deterministic-shuffle / private-view / viewer-filtered-effect / no-leak-export proof that
  `poker_lite` **extends**. Its hidden-hand machinery is the closest existing pattern; the
  `poker_lite` spec is a delta on it, not a fresh invention.

**Code seams to inspect directly (read in the repo; not pasted here — *inspect, not
read-fully*):**

- `games/high_card_duel/src/*` — the deterministic-shuffle / private-hand / viewer-redaction /
  private-view-only-bot model to extend: `state.rs`, `rules.rs`, `effects.rs`, `visibility.rs`,
  `bots.rs`. This is the most relevant existing game for cards + hidden info.
- `games/token_bazaar/src/*` and `games/token_bazaar/{data,tests,docs,benches}/*` — the
  resource/accounting and the newest **file-for-file crate layout** a new `games/poker_lite`
  crate should mirror; its `BOT-STRATEGY-EVIDENCE-PACK`/`COMPETENT-PLAYER` docs model the
  Level 2 evidence the `poker_lite` bot must carry.
- `games/secret_draft/src/*` and its `{data,tests,docs,benches}` — the most recent crate; its
  golden-trace set (commit / reveal / pending-seat / no-leak / diagnostic / bot / WASM-export)
  models the trace coverage `poker_lite` needs.
- `crates/engine-core/src/*` — confirm the noun-free actor/turn/action-tree/effect/replay/
  visibility envelopes; the spec must state the kernel stance (no `bet`/`pot`/`card` noun, no
  behavior smuggled in).
- `crates/wasm-api/src/lib.rs` — the per-game WASM registration surface (game-id consts,
  variant consts, action dispatch, redaction path) a new game extends.
- `tools/simulate`, `tools/replay-check`, `tools/fixture-check`, `tools/rule-coverage` (and
  `bench-report`, `seed-reducer`, `trace-viewer` if they enumerate game ids) — the hardcoded
  game-id registration seams the spec must list.
- `apps/web/src/components/*` (`GamePicker.tsx`, `AppShell.tsx`, `ActionControls.tsx`,
  `EffectLog.tsx`, `effectFeedback.ts`, and the per-game board components such as
  `SecretDraftBoard.tsx` / `HighCardDuelBoard.tsx`), `apps/web/src/state/shellReducer.ts`,
  `apps/web/src/wasm/client.ts`, and `apps/web/e2e/*.smoke.mjs` — the presentation +
  no-leak/pending smoke wiring a new `PokerLiteBoard` and `poker-lite.smoke.mjs` would extend.
- `.github/workflows/gate-1-game-smoke.yml` and `.github/workflows/gate-2-benchmarks.yml` —
  the native-smoke and benchmark lanes the spec must register `poker_lite` into.
- `apps/web/README.md` + `scripts/check-catalog-docs.mjs` — the web-shell catalog the gate's
  capstone work must reconcile in-gate (intro list, Shell Surface renderer list, `smoke:e2e`
  list), as Gate 9.1 did.

---

## 3. Settled intentions (final — these make this brief locked)

These decisions were resolved with the repo owner. Treat them as committed, not optional.

### 3.1 The deliverable is a single Gate 10 `poker_lite` spec

Author the implementation spec that turns `docs/ROADMAP.md` §12 (Gate 10) `poker_lite` into a
concrete, reviewable plan — the next progress spec. **One deliverable only.** It is **not** a
ticket decomposition (see §3.7). No aftermath/cleanup spec is owed (see §1); do not produce a
second document.

### 3.2 The next-gate determination is Gate 10 `poker_lite` — confirm it, do not re-open it

`poker_lite` is the locked target: it is the lowest non-`Done` ladder stage (stage 9), Gate 9.1
named Gate 10 as successor, and exploration confirmed **no interlock blocks it** (atlas §10A
empty; no third-use hard gate; no aftermath owed). Session 2's job is to **confirm and document
this determination** inside the spec's Objective and Sequencing sections (citing the empty
promotion-debt register and the satisfied Gate 9.1 successor preconditions) — **not** to
re-derive "what is next" open-endedly. `plain_tricks` is the expected **Gate 10.1** successor;
name it as such in Sequencing.

### 3.3 You design the concrete original ruleset, and you write variant scope before "coding"

The Gate 9 `token_bazaar` spec proposed a full original ruleset inside the spec; do the same
here. **Design `poker_lite`'s concrete, original betting/showdown variant yourself**, grounded
in researched minimal-poker prior art, satisfying ROADMAP §12's **"Variant scope MUST be
written before coding"** mandate, within these **hard constraints**:

- **Two seats** by default (`seat_0`, `seat_1`), matching the entire current portfolio. Keep
  it heads-up unless a tiny fixed N>2 is genuinely required and justified against the
  determinism/UI/bot cost; default to 2-seat.
- **Small and proof-shaped, not a poker engine.** The point is to prove the architecture
  handles **betting, pots, public/private cards, a simple showdown, and an imperfect-information
  bot policy** — not to ship Texas Hold'em. A minimal research-style design (in the spirit of
  Kuhn poker / Leduc poker — see §5) is the right altitude: a tiny fixed deck, one or few
  private cards, an optional small public card, **one (or a small fixed number of) bounded
  betting round(s)** with a small fixed bet ladder, and a deterministic showdown.
- **Fully deterministic** — deterministic setup and deterministic shuffle/deal via the existing
  seeded-RNG discipline (reuse `high_card_duel`'s shuffle precedent; do **not** add an
  `engine-core` RNG noun, per the established RNG decision). Every deal, bet, and showdown
  replays from seed + command log.
- **Bounded by construction** — fixed deck, fixed bet ladder, fixed maximum raises, fixed round
  cap. No unbounded variants (ROADMAP §12 "Not allowed").
- **Original / non-IP and casino-neutral** — internal id `poker_lite`; choose a clear original
  neutral **public display name** (mirroring "High Card Duel" / "Token Bazaar" / "Veiled
  Draft"); original card/label prose; **no casino/chip/payout/table/insurance language in
  public UI**, no proprietary poker rules text or trade dress (`docs/IP-POLICY.md`, ADR 0006
  precedent). Stakes are abstract original units, not "chips/money."
- **Public deterministic pot accounting and public deterministic tie-breaks** — pot
  contributions, the showdown comparison, split/tie resolution, and the winner determination
  are exact and effect-visible (mirror Gate 9's accounting/tie-break rigor).
- **Imperfect-information no-leak** — each seat's hidden hole card(s) are redacted from the
  opponent and from every browser payload, DOM, `data-testid`, storage, replay export, dev
  panel, and bot view **until showdown** resolves them in a grouped reveal.

### 3.4 The bot is Level 2 authored-policy with an evidence pack — and heuristic by law

Gate 10 requires, beyond the mandatory **Level 0** random-legal bot, a **Level 2 authored-policy
bot** carrying a `BOT-STRATEGY-EVIDENCE-PACK` and a `COMPETENT-PLAYER` write-up (matching the
recent games). The bot's policy is **authored priorities** driven by its own hand strength,
pot odds, and the public betting state, choosing through the normal legal-action API:

- It uses **only its own allowed private view** — its hole card(s) plus public board/pot —
  and **never** samples, enumerates, or models the opponent's hidden hand.
- **Explicitly forbidden:** MCTS, ISMCTS, Monte Carlo equity simulation, ML, RL, or any
  hidden-state sampling (FOUNDATIONS §8/§11, ROADMAP §12). The "imperfect-information bot proof"
  is proven by a *competent, explainable, fair, beatable authored heuristic*, not a solver.
- Explanations and candidate rankings must be **viewer-safe** (no leak of the bot's own hidden
  card to the opponent's view, and no leak of the opponent's hidden card to anyone).

### 3.5 `poker_lite` is a delta on proven machinery — no new kernel concept, no promotion

- It **extends** `high_card_duel`'s deterministic-shuffle / private-view / viewer-filtered-effect
  / no-leak-export envelope and `ARCHITECTURE.md`'s existing action-tree / command-envelope /
  semantic-effect / grouped-reveal shapes.
- **No `engine-core` noun** (`bet`, `raise`, `call`, `fold`, `pot`, `card`, `deck`, `hand`,
  `suit`, `showdown`, `chip`, etc.) — these live in `games/poker_lite`.
- **No `game-stdlib` promotion.** `poker_lite` is the **second** official use of the
  card/private-hand shape (first: `high_card_duel`) and of resource/accounting (first:
  `token_bazaar`); betting/pot is a **first** official use. Per the atlas, second use is a
  *compare-and-record*, not an extraction. Implement locally; add a first-use betting/pot atlas
  note and second-use comparison notes for cards and accounting. If a helper feels unavoidable,
  the spec instructs the implementer to **stop and write a primitive-pressure ledger entry
  first** — the expected Gate 10 answer is still local implementation.
- If you conclude any part of the betting/showdown/imperfect-information model genuinely requires
  a kernel/visibility/replay-semantics change (a real **§13 ADR trigger**), do **not** design
  against the foundation silently: state it explicitly in the spec, identify the affected
  sections, and recommend an ADR as a prerequisite — but first prove it is not already covered
  by the `high_card_duel` hidden-info pattern. The **expected verdict is "no ADR required"**:
  betting/pot are game-local nouns, and the bot stays heuristic.

### 3.6 Give an explicit §13 ADR verdict and an explicit second-use ledger stance

The spec's "FOUNDATIONS & boundary alignment" section must carry (a) a clear **§13
ADR-trigger verdict** (expected: none), and (b) an explicit **second-use primitive-pressure
stance** for cards/private-hand and resource/accounting plus a **first-use** note for
betting/pot — each "keep local, record in atlas, no promotion," with the closure/review gate
named (e.g., third card/economy game).

### 3.7 Specs only — no ticket decomposition

The deliverable is a **spec**, not tickets. Do **not** produce `AGENT-TASK` packets,
`tickets/*`, or a per-diff decomposition. The Work-breakdown section lists **bounded candidate
AGENT-TASKs in dependency order** (as the Gate 9 spec does); decomposing them into ticket files
is a later, separate step.

---

## 4. The task

Produce one **new-spec** markdown document for the Rulepath repository: the **Gate 10
`poker_lite` implementation spec** — a concrete, reviewable plan, grounded in `docs/ROADMAP.md`
§12 and the full foundation set, for an original, deterministic, two-seat, browser-playable
betting-and-showdown microgame that proves **betting, pots, public/private cards, a simple
showdown, and an imperfect-information authored-policy bot** while keeping all behavior in Rust,
the kernel noun-free, no hidden card leaking anywhere before showdown, and the presentation
casino-neutral and original. The spec follows the canonical Rulepath 12-section format and
satisfies every governing foundation document.

---

## 5. Exploration + online-research mandate

Explore the repository as deeply as needed beyond the files listed in §2 — read the actual
`games/high_card_duel`, `games/token_bazaar`, and `games/secret_draft` sources, the
`engine-core` envelopes, the WASM surface, the web components and e2e smokes, and the CI lanes
— to ground every claim in the real tree rather than assumption.

**Research online as deeply as needed.** Recommended prior art for a minimal, deterministic,
imperfect-information betting design and a clean reveal/redaction model: **Kuhn poker** and
**Leduc poker** (the canonical research-minimal poker games), and **OpenSpiel's** treatment of
imperfect-information and simultaneous/sequential betting games and information-state APIs.
Use these for *structure, vocabulary, and a determinism/abstraction model* only — the designed
ruleset, name, prose, and units must be **original** and casino-neutral; never copy poker rules
text, hand-ranking tables verbatim as proprietary prose, or trade dress. **Cite every external
source** that shapes a design decision, and route the citations into the spec's sources note
and `docs/SOURCES.md` guidance.

---

## 6. Doctrine & constraints (honor all that the task engages)

- `docs/FOUNDATIONS.md` is the constitution — every product-behavior decision must satisfy its
  **§11 universal invariants** and clear its **§12 stop conditions**; a genuine divergence
  requires an accepted ADR superseding the affected principle first ("supersede only by accepted
  ADR"), never designing against it silently.
- **Authority order**: foundation docs govern area docs govern specs govern tickets. If a design
  choice conflicts with architecture or foundation, the design is wrong.
- `engine-core` stays generic and **noun-free** — no `bet`, `raise`, `call`, `fold`, `pot`,
  `card`, `deck`, `hand`, `suit`, `showdown`, `chip` nouns; typed mechanic nouns live in
  `games/poker_lite` first, shared helpers in `game-stdlib` only via the mechanic atlas.
- **TypeScript never decides legality.** Legal actions, validation, previews, effects, views,
  reveal/showdown timing, pot accounting, tie-breaks, and bot decisions all come from Rust/WASM.
- **No YAML and no DSL without an accepted ADR.** Static data is typed content / parameters /
  metadata only — never selectors, conditions, triggers, or betting/scoring formulas. Bet
  ladders, deck composition, and showdown tables are typed constants, not behavior.
- **Determinism**: replay, hashes, RNG, serialization order, and traces stay deterministic (or
  are explicitly migrated with a trace note). Deals, bets, and showdown reproduce from seed +
  command log.
- **No hidden-information leaks** into views, action trees, previews, effect logs, diagnostics,
  UI metadata, DOM attributes, `data-testid` values, local storage, replay exports, dev panels,
  bot explanations, or candidate rankings — **before showdown**, each seat's hole card(s) are
  invisible to the opponent and every payload (`ADR 0004`, `UI-INTERACTION.md` hidden-info
  safety).
- **No MCTS / ISMCTS / Monte Carlo / ML / RL bots**, and **no hidden-state sampling** — the
  `poker_lite` bot acts from its own allowed private view with a viewer-safe authored-heuristic
  rationale.
- **Casino-neutral, original IP** — no casino/chip/payout/insurance language or trade dress in
  public naming/UI; original prose; no proprietary poker rules text (`docs/IP-POLICY.md`,
  ADR 0006). Do not resurrect `blackjack_lite`.
- **Never delete or weaken tests to get green** — the spec carries the failing-test protocol
  (`AGENT-DISCIPLINE.md` §4).
- **No open promotion debt may be created and skipped**, and **no primitive is promoted** in
  this gate (second card/accounting use → compare-and-record; first betting/pot use → local,
  first-use atlas note).

---

## 7. Deliverable specification

Produce exactly **one** downloadable markdown document. It is a **new file**; it replaces
nothing.

### Deliverable — `specs/gate-10-poker-lite-betting-showdown.md`

A complete Rulepath implementation spec carrying the **canonical 12-section set** (per
`specs/README.md` "Spec format" and matching the depth of
`archive/specs/gate-9-token-bazaar-browser-proof.md`):

1. **Header** table — Spec ID, roadmap stage (9), build gate (Gate 10), status (`Planned`),
   date, owner, primary crate / internal game id (`poker_lite`), chosen original public display
   name, browser implementation required, authority order.
2. **Objective** — sourced from ROADMAP §12 and the Gate 9.1 successor recommendation; state
   and justify the **next-gate determination** (Gate 10 `poker_lite`; preconditions met; no
   interlock).
3. **Scope** — in scope / out of scope / **not allowed** (carry ROADMAP §12 "Not allowed":
   real-money/casino features, unbounded variants, hidden-state cheating, ML/RL, copied rules
   prose — plus gate-local prohibitions).
4. **Deliverables** — the `games/poker_lite/{Cargo.toml, src/*, data/*, benches/*, tests/*,
   docs/*}` tree (mirroring the `token_bazaar`/`secret_draft` layout), the golden-trace set
   (deal / bet-call / raise / fold / showdown-reveal / tie-split / no-leak public-observer /
   diagnostic / bot-action / WASM-export cases), the per-game `docs/*` instantiated from
   `templates/*` (**including `BOT-STRATEGY-EVIDENCE-PACK.md` and `COMPETENT-PLAYER.md` for the
   Level 2 bot**), the WASM + React `PokerLiteBoard` + `poker-lite.smoke.mjs` wiring, and the
   tool/CI/web-catalog registration points.
5. **Work breakdown** — bounded candidate AGENT-TASKs in dependency order (do **not** decompose
   into tickets).
6. **Exit criteria** — mapped **row-for-row** to ROADMAP §12 Gate 10 (betting/showdown rules
   correct for the chosen variant; pot/accounting tests cover edge cases; bots finish games
   without hidden-state cheating; no public MCTS/ISMCTS; UI understandable; native benchmarks
   exist), plus the universal hidden-info no-leak obligations.
7. **Acceptance evidence** — the full `OFFICIAL-GAME-CONTRACT.md` deliverable set: rule /
   property / replay / serialization / **visibility-no-leak** tests, golden traces, the
   `simulate` / `replay-check` / `fixture-check` / `rule-coverage` runs, benchmarks (smoke
   floors with a named calibration follow-up, per ADRs 0002/0003/0005 and the 0001 playout
   budget), browser e2e smoke (human + bot + bet/fold + showdown reveal + no-leak/observer +
   replay/export-import + reduced-motion + a11y), and `boundary-check.sh` + `check-doc-links.mjs`
   + `check-catalog-docs.mjs`.
8. **FOUNDATIONS & boundary alignment** — a principle/stance/rationale table covering §2
   behavior authority, §3 kernel (no betting/card noun), §4 `game-stdlib` earned (no promotion;
   explicit second-use card/accounting stance + first-use betting/pot note), §5 static data
   (no betting/scoring formulas in data), §8 public bots (Level 2 authored heuristic; no
   Monte-Carlo/ISMCTS; private-view only), §11 invariants (incl. hidden-hand no-leak before
   showdown), §12 stop conditions, and a clear **§13 ADR-trigger verdict** (expected: none —
   state why the betting/showdown/imperfect-info model is covered by the existing kernel and
   the `high_card_duel` hidden-info pattern).
9. **Forbidden changes** — gate-specific prohibitions (no kernel nouns; no generic
   bet/pot/card/deck/showdown/bot-policy `game-stdlib` helper; no behavior-in-data; no TS
   legality/showdown-timing/pot-accounting; no MCTS/ISMCTS/ML/RL/hidden-state sampling; no
   casino language or proprietary poker content; no Blackjack resurrection; no accidental
   trace/hash migration).
10. **Documentation updates required** — incl. the `specs/README.md` Gate 10 row, the
    `docs/MECHANIC-ATLAS.md` first-use betting/pot note + second-use card/accounting comparison
    (and §10B candidate updates), `progress.md` and root `README.md` updates after
    implementation, the in-gate `apps/web/README.md` web-catalog reconciliation (intro list,
    Shell Surface renderer list, `smoke:e2e` list — folded into the gate per the Gate 9.1
    precedent and `specs/README.md` §10), and the per-game docs; **do not** edit
    `docs/ROADMAP.md` to record progress.
11. **Sequencing** — predecessor Gate 9.1 (`Done`); successor **Gate 10.1 `plain_tricks`**;
    admission rule (no open promotion debt; Gate 9.1 no-leak evidence landed).
12. **Assumptions** — one-line-correctable, including the **2-seat default**, the
    minimal-variant altitude, the heuristic-bot stance, and any design choice you want the
    owner to be able to override.

You **may** append an "Implementation reference" section below the canonical set (as the Gate 9
spec does) carrying the full proposed rules, deck/bet-ladder constants, action-tree / bet-path
metadata, deal/showdown reveal-and-redaction model, pot-accounting and tie-break ladder, bot
policy and explanation templates, WASM/browser wiring, fixtures, golden-trace list, and
benchmark operations.

### Locked / no-questions instruction

> Produce the deliverable directly as a downloadable markdown document. Do not interview, do not
> ask clarifying questions — the requirements above are final. If a genuine contradiction makes
> a requirement impossible, state it in the deliverable and proceed with the most faithful
> interpretation.

---

## 8. Self-check (run against your own output before returning)

- The deliverable set is **exactly** the one new file named in §7 — no tickets, no aftermath
  spec, no extra docs.
- The spec carries all **12 canonical sections** and matches the depth/shape of the Gate 9
  `token_bazaar` sibling spec, with an optional appended implementation-reference section.
- The next-gate determination (**Gate 10 `poker_lite`**) is stated and justified (preconditions
  met; atlas §10A empty; no third-use hard gate; no aftermath owed), not re-opened.
- The designed ruleset is **original, casino-neutral, deterministic, 2-seat (unless justified),
  bounded** betting + public/private cards + simple showdown, with a fixed bet ladder, fixed
  round cap, and **public deterministic pot accounting and tie-breaks**; variant scope is fully
  written before any "coding."
- **No hidden hole card leaks before showdown** in any view, action tree, preview, effect log,
  diagnostic, UI metadata, DOM, `data-testid`, storage, replay export, dev panel, bot
  explanation, or candidate ranking; the no-leak / visibility test obligations are explicit.
- The bot is **Level 2 authored-heuristic from its own private view only** (plus the mandatory
  Level 0), with a `BOT-STRATEGY-EVIDENCE-PACK` and `COMPETENT-PLAYER` write-up, and **no**
  MCTS/ISMCTS/Monte-Carlo/ML/RL/hidden-state sampling.
- **No `engine-core` noun**, **no `game-stdlib` promotion**; a first-use betting/pot atlas note
  and a second-use card/accounting comparison are required, and any helper temptation is gated
  behind a primitive-pressure ledger entry.
- Determinism/replay/hash/serialization discipline is preserved; deals/bets/showdown reproduce
  from seed + command log; no trace/hash migration is introduced by accident.
- No deliverable weakens an upstream foundation doc or silently amends an accepted ADR; the §13
  ADR-trigger verdict is stated (expected: none, with reasons), and any genuine trigger is
  named, not designed around. `blackjack_lite` is not resurrected.
- Exit criteria are mapped row-for-row to ROADMAP §12 Gate 10, including "no public
  MCTS/ISMCTS" and "bots finish games without hidden-state cheating."
- Every external claim that shaped a design decision is **cited** (Kuhn/Leduc/OpenSpiel or
  others), with original prose only.
- The web-catalog closeout (`apps/web/README.md` + `check-catalog-docs.mjs`) is folded into the
  gate's documentation-updates section (Gate 9.1 precedent), not deferred to an aftermath pass.
- The §1 fetch-baseline commit `85713751653411667f885fa13d7267a3d20280a8` contains every file
  named in the §2 read-in-full list (it does).

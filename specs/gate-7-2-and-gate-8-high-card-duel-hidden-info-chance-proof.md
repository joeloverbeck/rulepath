# Gate 7.2 + Gate 8 Implementation Spec — High Card Duel Hidden-Information/Chance Proof

Spec ID: `gate-7-2-and-gate-8-high-card-duel-hidden-info-chance-proof`  
Roadmap stage: Stage 6 (chance / hidden-information proof)  
Roadmap build gate: Gate 7.2 (orientation interlock) + Gate 8 (`high_card_duel`)  
Status: Planned  
Date: 2026-06-07  
Owner: joeloverbeck  
Authority order: `docs/FOUNDATIONS.md` → `docs/ARCHITECTURE.md` → `docs/ENGINE-GAME-DATA-BOUNDARY.md` → `docs/OFFICIAL-GAME-CONTRACT.md` → `docs/ROADMAP.md`  
Output type: Implementation spec only; not tickets and not code.

---

## Source references

External research consulted for rationale (not repository evidence):

- boardgame.io `playerView`: https://github.com/boardgameio/boardgame.io/blob/main/docs/documentation/api/Game.md
- OpenSpiel introduction: https://openspiel.readthedocs.io/en/latest/intro.html
- OpenSpiel paper: https://arxiv.org/abs/1908.09453
- Ludii imperfect-information / nondeterminism universality paper: https://arxiv.org/abs/2205.00451
- Bicycle Cards War rules: https://bicyclecards.com/how-to-play/war
- Pagat War rules and notes: https://www.pagat.com/war/war.html
- Pagat Blackjack rules and notes: https://www.pagat.com/banking/blackjack.html
- Lemire, *Fast Random Integer Generation in an Interval*: https://arxiv.org/abs/1805.10941
- Fisher-Yates shuffle background: https://en.wikipedia.org/wiki/Fisher%E2%80%93Yates_shuffle

---

## Repository findings that drive this spec

`specs/README.md` is the active gate tracker. At the target commit it marks Gates 0 through 7.1 as `Done` and Gate 8, `high_card_duel` / `blackjack_lite`, as `Not started`. It also says open promotion debt interlocks advancement and defines the spec format used below.

`docs/ROADMAP.md` identifies Gate 8 as the first Stage 6 chance/hidden-information proof. It specifically names deterministic shuffle, private views, filtered logs/effects, no-leak serialization, and bots that act only from allowed private views. It recommends `high_card_duel` as the first candidate and says to add `blackjack_lite` only if useful pressure can be gained without derailing polish.

`docs/FOUNDATIONS.md`, `docs/ARCHITECTURE.md`, `docs/WASM-CLIENT-BOUNDARY.md`, and `docs/ENGINE-GAME-DATA-BOUNDARY.md` make the critical boundary non-negotiable: Rust owns setup, legality, transitions, deterministic randomness, public/private projection, effects, replay/hash behavior, serialization, and bots. TypeScript owns presentation only. Hidden state must not be sent to unauthorized browser surfaces and then hidden in React.

`docs/MECHANIC-ATLAS.md` says the Gate 7.1 board-space debt is closed and the open promotion-debt register is empty. It anticipates `high_card_duel` as the first local deterministic-shuffle / hidden-draw implementation and does not authorize generic card/deck promotion.

`README.md`, `progress.md`, `AGENTS.md`, and `CLAUDE.md` are stale orientation surfaces at the target commit. `README.md` still describes Gate 5 completion and an official-game set that omits later completed gates. `progress.md` records only Gate 3 and Gate 5-era progress. `AGENTS.md` and `CLAUDE.md` still hardcode Race-to-N as the current verification game. These are the only reason for Gate 7.2.

`crates/engine-core` already has generic viewer, action, replay, RNG, and effect visibility vocabulary. `EffectLog` supports public and private-to-seat visibility filtering. `Game::project_view` accepts a `Viewer`. `legal_action_tree` is actor-scoped. There are no card/deck/hand nouns in `engine-core`.

`crates/wasm-api/src/lib.rs` has a `get_view(match_id, viewer_seat)`-shaped export, but the fetched code ignores the viewer seat in its main view-return path and passes `Viewer { seat_id: None }` for the current public-equivalent games. `get_effects` already accepts a viewer seat and constructs a viewer before filtering. `apply_action` and `run_bot_turn` return a view/effects response assuming public projections. Gate 8 must harden this API boundary before hidden information is browser-visible.

`crates/engine-core/src/rng.rs` exposes `DeterministicRng::next_index`, currently implemented with modulo reduction. That is acceptable for existing uses only to the extent current docs/tests allow it, but Gate 8 must not silently use modulo reduction as proof of an unbiased shuffle. The Gate 8 shuffle must either use a documented rejection-sampling bounded helper or make a tightly scoped RNG fix with tests.

---

# 1. Header

## 1.1 Gate name

Combined next-work gate:

- Part A: Gate 7.2 — Lightweight Repository Orientation / Progress Hygiene Interlock
- Part B: Gate 8 — `high_card_duel` Hidden-Information/Chance Proof
- Part C: Post-Gate-8 `blackjack_lite` Continuation Checkpoint

## 1.2 Status

Planned. This spec defines the next implementation work. It does not itself mark the gate done.

## 1.3 Dependency posture

Gate 8 may not begin until Gate 7.2 passes its small exit criteria. Gate 7.2 must not expand into broad cleanup. It is an interlock to make future agents orient correctly before hidden-information work begins.

## 1.4 Deliverable posture

Implementation agents must decompose this spec into tickets or tasks. This file is not a ticket list and does not authorize code-by-diff delivery.

---

# 2. Objective

Deliver Rulepath’s first official chance and hidden-information proof through an original, local-first, public-facing card duel named `high_card_duel`.

The gate must prove:

- deterministic setup shuffle owned by Rust;
- private hands and private face-down commitments;
- viewer-safe public, seat-private, and observer projections;
- viewer-scoped legal action trees;
- hidden-info-safe previews, disabled reasons, diagnostics, effects, and command summaries;
- no-leak browser/WASM surfaces;
- no-leak public replay/export/import surfaces;
- bot decisions constrained to allowed view data only;
- a polished, accessible card-game UI without casino, poker, blackjack, or commercial-trade-dress vibes;
- local card/deck/hand/commitment implementation without premature `game-stdlib` or `engine-core` promotion.

The gate also repairs stale orientation/progress drift left after Gate 7.1 and encodes a mandatory `blackjack_lite` checkpoint so the deferred Blackjack proof cannot disappear.

---

# 3. Scope

## 3.1 In scope

| Area | Scope |
|---|---|
| Gate 7.2 orientation hygiene | Update only the stale repository-orientation documents proven stale by exact-commit analysis. |
| Gate tracker | Keep `specs/README.md` coherent for Gate 7.2, Gate 8, and the post-Gate-8 `blackjack_lite` checkpoint. |
| New game crate | Add official game `games/high_card_duel` with code, data, docs, tests, traces, fixtures, benches, and UI metadata matching existing crate conventions. |
| Rules | Implement a small original two-seat high-card commitment duel with deterministic shuffle, private hands, hidden commitments, reveal/compare, scoring, refill, and finite terminal conditions. |
| Chance | Use deterministic Rust-owned shuffle/deal from `Seed`, with explicit shuffle algorithm/version and no silent modulo-bias assumption. |
| Hidden information | Enforce private hand, hidden deck order, and hidden commitment secrecy at Rust/WASM boundary. |
| Viewer model | Harden `Viewer { seat_id: Option<SeatId> }` flow through Rust, WASM, TypeScript, replay, effects, action controls, and smoke tests. |
| Action trees | Treat action trees as private data. Return actor-private card choices only to the authorized acting seat. |
| Effects | Add public and private effect events with correct filtering. |
| Replay/export/import | Separate internal full traces from public/viewer-scoped exports. Public browser exports are no-leak by default. |
| Bots | Level 0 random legal bot is mandatory. A Level 1 simple baseline is allowed only if it remains hidden-info safe and documented; Level 2 is out of scope. |
| UI | Add polished, accessible, responsive high-card duel UI with viewer selector/hotseat/observer mode, safe card rendering, safe reveal animation, and no-leak checks. |
| Primitive pressure | Record card/deck/hand/commitment mechanic pressure locally and in atlas notes, without promotion. |
| Tests/benchmarks | Add native, WASM, replay, serialization, no-leak, UI smoke, and benchmark evidence. |

## 3.2 Out of scope

| Area | Explicitly out of scope |
|---|---|
| Blackjack implementation | `blackjack_lite` is not implemented in this gate unless a follow-up checkpoint requires Gate 8.1 / Gate 8B after Gate 8. |
| Generic card engine | No generic card/deck/hand/pile/suit/rank engine in `engine-core` or `game-stdlib`. |
| DSL/YAML behavior | No game behavior in YAML/static data and no new game-description DSL. |
| Hosted multiplayer | No accounts, server persistence, matchmaking, chat, rankings, or remote multiplayer. |
| Betting/casino features | No betting, chips, payout tables, insurance, surrender, doubles, splits, or casino-styled UI. |
| Advanced AI | No MCTS, ISMCTS, Monte Carlo search, reinforcement learning, ML, or LLM move choice. |
| Foundation rewrites | No broad foundation law rewrite. Only narrow boundary-doc updates required by Gate 8 behavior. |
| Trace schema churn | No trace schema change unless unavoidable; if unavoidable, it must be documented and justified. |

---

# 4. Deliverables

## 4.1 Part A — Gate 7.2 deliverables

1. `specs/README.md`
   - This single spec file is referenced by **two** index rows: a Gate 7.2 maintenance-interlock row (mirroring the existing Gate 7.1 "5M" interlock convention) and the existing Gate 8 row, both pointing at `specs/gate-7-2-and-gate-8-high-card-duel-hidden-info-chance-proof.md`.
   - Add or record the Gate 7.2 interlock row according to repo status conventions.
   - Keep Gates 0–7.1 as done.
   - Flip the Gate 8 row from `Not started` / `not yet specced` to `Planned`, pointing at this spec.
   - Add an explicit `blackjack_lite` continuation checkpoint before Gate 9 admission.

2. `README.md`
   - Replace Gate 5-era status language with current orientation:
     - official games through Gate 7 / Gate 7.1;
     - `board_space` back-port/progression status;
     - Gate 8 as next chance/hidden-information proof.
   - Do not rewrite project identity or foundation doctrine.

3. `progress.md`
   - Update stale progress summary so future agents do not infer the repo is still at Gate 5.
   - Include concise Gate 6, Gate 7, Gate 7.1 completion entries or a current-progress pointer pattern consistent with the repo.

4. `AGENTS.md` and `CLAUDE.md`
   - Remove stale “current game: race_to_n” assumptions.
   - Replace with current verification guidance that covers all official game crates or points to `specs/README.md` for the active gate.
   - Preserve discipline rules.

5. `docs/MECHANIC-ATLAS.md`
   - Verify the open promotion-debt register (§10A) remains empty.
   - Confirm Gate 7.1 `board_space` debt is closed.
   - Note: at the target commit the atlas already records a `high_card_duel` "deterministic shuffle and hidden draw → local-only" pressure row. Verify and, if needed, extend that existing row; do not author a duplicate.

6. Optional orientation file updates
   - Only update another orientation file if exact-commit content proves it is stale and future agents would be misled.
   - Do not touch foundation law just for style.

## 4.2 Part B — Gate 8 deliverables

### 4.2.1 New crate and workspace

Add `games/high_card_duel` and register it in the workspace `Cargo.toml`
members list (and as a bench target). Tool- and CI-level registration is a
separate concrete deliverable — see 4.2.13.

Required layout, adjusted only where existing repo conventions require:

```text
games/high_card_duel/
  Cargo.toml
  src/
    lib.rs
    ids.rs
    state.rs
    setup.rs
    actions.rs
    rules.rs
    effects.rs
    visibility.rs
    variants.rs
    bots.rs
    replay_support.rs
    ui.rs
  data/
    manifest.toml
    variants.toml
    fixtures/
      high_card_duel_standard.fixture.json
  docs/
    SOURCES.md
    RULES.md
    RULE-COVERAGE.md
    MECHANICS.md
    AI.md
    UI.md
    BENCHMARKS.md
    GAME-IMPLEMENTATION-ADMISSION.md
    PUBLIC-RELEASE-CHECKLIST.md
    COMPETENT-PLAYER.md
    BOT-STRATEGY-EVIDENCE-PACK.md
  tests/
    rules.rs
    replay.rs
    property.rs
    visibility.rs
    bots.rs
    serialization.rs
    golden_traces/
      shortest-normal.trace.json
      tie-round.trace.json
      invalid-wrong-seat-diagnostic.trace.json
      invalid-private-card-redacted.trace.json
      stale-diagnostic.trace.json
      bot-action.trace.json
      hidden-info-public-observer.trace.json
      seat-private-view.trace.json
      public-replay-export-import.trace.json
      terminal.trace.json
  benches/
    high_card_duel.rs
    thresholds.json
```

`COMPETENT-PLAYER.md` and `BOT-STRATEGY-EVIDENCE-PACK.md` must exist only if repo policy requires physical files for every official game. If present and Level 2 is not shipped, they must explicitly say `not applicable / deferred` rather than implying strategy proof completion.

Note: existing official games (`race_to_n` … `draughts_lite`) do not carry a dedicated `tests/serialization.rs`; they fold serialization coverage into `tests/visibility.rs` (stable-serialization assertions) and `tests/replay.rs` (`to_json` round-trips). The dedicated `serialization.rs` above consolidates the §4.2.8 serialization tests into one file — a deliberate, cleaner divergence, not a convention error. If implementation prefers parity with prior games, fold those tests into `visibility.rs`/`replay.rs` and drop the `--test serialization` evidence line accordingly.

### 4.2.2 High Card Duel rules

Implement an original Rulepath game, not a direct War clone.

#### Rule identity

Game ID: `high_card_duel`  
Default variant ID: `high_card_duel_standard`  
Seats: `seat_0`, `seat_1`  
Public theme: neutral “wayfarer duel table” / “trail badges” / “rune cards” aesthetic. No casino table, chips, betting, poker branding, blackjack terminology, or commercial trade dress.

#### Card model

Use a game-local card model.

Recommended default:

- 24-card local duel deck.
- 12 ranks: `1` through `12`.
- Two neutral sigils per rank, for identity uniqueness only. Sigils do not affect comparison.
- Stable internal card IDs, for example `hcd:r01:a`, `hcd:r01:b`, …, `hcd:r12:a`, `hcd:r12:b`.
- Public rank labels may be themed, but numeric rank must remain clear in docs/tests.
- No `Card`, `Deck`, `Hand`, `Suit`, `Rank`, `Pile`, or equivalent mechanic noun is added to `engine-core`.
- No generic `game-stdlib` card/deck helper is added in this gate.

#### Setup

Rule IDs:

- `HCD-SETUP-001` — Build the game-local 24-card deck in canonical sorted order.
- `HCD-SETUP-002` — Shuffle the deck using deterministic Rust-owned randomness from the match seed.
- `HCD-SETUP-003` — Deal three private cards to each seat, alternating `seat_0`, then `seat_1`, until both hands contain three cards.
- `HCD-SETUP-004` — Set round number to `1`, score to `0–0`, phase to `lead_commit`, and lead seat to `seat_0`.
- `HCD-SETUP-005` — Store remaining deck order internally only. Public and unauthorized projections may expose only deck count.

#### Round structure

Each game has exactly six rounds unless an explicit variant changes it.

Rule IDs:

- `HCD-ROUND-001` — At the start of each round, exactly one seat is lead and one seat is reply.
- `HCD-ROUND-002` — The lead seat chooses one card from its own private hand and commits it face-down.
- `HCD-ROUND-003` — The reply seat chooses one card from its own private hand without seeing the lead commitment identity.
- `HCD-ROUND-004` — After both commitments exist, both committed cards reveal simultaneously.
- `HCD-ROUND-005` — Higher rank wins the round and earns one point.
- `HCD-ROUND-006` — If ranks tie, no point is awarded and the round is recorded as a tie.
- `HCD-ROUND-007` — Revealed cards move to a revealed/discard history visible to all viewers.
- `HCD-ROUND-008` — After scoring, refill hands up to three cards each from the internal deck while cards remain.
- `HCD-ROUND-009` — Refill order starts with the next round’s lead seat, then alternates seats until both hands are full or the deck is empty.
- `HCD-ROUND-010` — Lead seat alternates by round: odd rounds `seat_0`, even rounds `seat_1`.
- `HCD-ROUND-011` — Advance to the next round after refill/cleanup.
- `HCD-ROUND-012` — After round six resolves, terminal state is reached.
- `HCD-ROUND-013` — Terminal winner is the higher score; equal score is a draw.

The fixed six-round terminal makes the game finite, quick to smoke-test, and still leaves unrevealed deck order in many seeds. The unrevealed tail is part of the hidden-information proof and must not leak in public exports or terminal browser payloads.

#### Legal actions

Rule IDs:

- `HCD-ACT-001` — Only the active lead seat may commit during `lead_commit`.
- `HCD-ACT-002` — Only the active reply seat may commit during `reply_commit`.
- `HCD-ACT-003` — A commit action must identify one card currently in the actor’s own private hand.
- `HCD-ACT-004` — A seat may not commit twice in the same round.
- `HCD-ACT-005` — Observer/no-seat viewers have no legal private commit actions.
- `HCD-ACT-006` — Terminal states have no legal gameplay actions.
- `HCD-ACT-007` — Existing engine patterns for not-applicable/no-op actions may be used only if already required by cross-game tooling; otherwise terminal action tree is empty.
- `HCD-ACT-008` — Action-tree labels returned to an authorized actor may show that actor’s own card identity. Public, observer, and opponent views must not receive those labels or paths.

#### Invalid and stale actions

Rule IDs:

- `HCD-DIAG-001` — Wrong-seat action returns a public-safe wrong-seat diagnostic.
- `HCD-DIAG-002` — Wrong-phase action returns a public-safe phase diagnostic.
- `HCD-DIAG-003` — Invalid private card identity returns a redacted diagnostic to unauthorized viewers and may include the card identity only in the acting seat’s private diagnostic if safe.
- `HCD-DIAG-004` — Stale action uses existing stale-action conventions and must not leak current hidden state.
- `HCD-DIAG-005` — Occupied/missing commitment diagnostics never reveal opponent card identity before reveal.
- `HCD-DIAG-006` — Diagnostics in browser-visible logs, dev panels, test IDs, DOM attributes, and replay command summaries must use redacted public tokens unless viewer authorization is explicit.

#### Views

Define three viewer modes:

- `observer`: `Viewer { seat_id: None }`
- `seat_0`: `Viewer { seat_id: Some(seat_0) }`
- `seat_1`: `Viewer { seat_id: Some(seat_1) }`

Observer/public projection includes only:

- game ID and variant ID;
- round number and finite-round limit;
- phase;
- active/lead/reply seat IDs;
- scores;
- hand counts by seat;
- deck count only, if included;
- commitment occupancy by seat as face-down/redacted until reveal;
- revealed cards after reveal;
- public effect cursor and public-safe status;
- terminal result after terminal.

Seat-private projection includes all observer fields plus:

- that seat’s private hand card identities;
- that seat’s own committed card identity after it commits;
- private legal action affordances for that seat when it is actor;
- private effects addressed to that seat.

Seat-private projection must not include:

- opponent private hand identities;
- opponent face-down committed card identity before reveal;
- unrevealed deck order;
- future draw identities;
- opponent bot candidates or explanations;
- any internal state that can reconstruct hidden cards.

#### Effects

Use existing `EffectLog` visibility semantics.

Required semantic effect families:

| Effect | Visibility | Purpose |
|---|---|---|
| `hcd_deal_private_card` | `PrivateToSeat(owner)` | Owner learns card identity. |
| `hcd_hand_count_changed` | `Public` | Everyone sees hand count changes. |
| `hcd_commit_face_down` | `Public` | Everyone sees that a seat committed. |
| `hcd_own_commit_confirmed` | `PrivateToSeat(owner)` | Owner may see own committed card. |
| `hcd_cards_revealed` | `Public` | Both committed cards become public simultaneously. |
| `hcd_round_scored` | `Public` | Score/tie update. |
| `hcd_refill_started` | `Public` | Optional, no hidden identities. |
| `hcd_terminal` | `Public` | Final result. |
| `hcd_private_diagnostic` | `PrivateToSeat(owner)` | Only if needed; must not expose opponent/deck facts. |
| `hcd_public_diagnostic` | `Public` | Redacted/public-safe diagnostics. |

Do not emit private card identities in public effect payloads, text, keys, CSS classes, DOM attributes, test IDs, console logs, storage, replay exports, or command summaries.

### 4.2.3 Deterministic shuffle and RNG

Gate 8 must document and test the shuffle algorithm.

Required behavior:

1. Shuffle algorithm is named and versioned in `games/high_card_duel/docs/RULES.md`, `docs/SOURCES.md`, and/or `docs/MECHANICS.md`.
2. Deterministic setup uses the existing `Seed` / deterministic RNG contract.
3. Same seed + same variant yields identical deck/deal/internal trace in internal tests.
4. Different seeds can yield different deals.
5. Public and observer projections never include the shuffled deck order.
6. Public browser replay export must not include material that lets unauthorized viewers reconstruct unrevealed cards.
7. If using Fisher-Yates or inside-out Fisher-Yates, bounded random index generation must be unbiased or explicitly tested/documented as a bounded local helper.
8. Do not silently use modulo reduction as the hidden-information/chance proof.

Implementation options:

- Preferred: implement a game-local `next_bounded_index_unbiased(upper_bound)` helper using rejection sampling over `DeterministicRng::next_u64`, with tests.
- Acceptable: make a narrow, generic `engine-core` RNG helper such as `next_index_unbiased`, only if it remains card-agnostic and includes tests/documentation. This is an RNG utility change, not a card primitive.
- Forbidden: promote card/deck vocabulary to `engine-core` to solve shuffle.

Research note: Fisher-Yates is the right conceptual algorithm for uniformly shuffling a finite deck, but correct uniformity depends on unbiased random integers in each shrinking interval. Practical ranged-integer generation literature calls out this exact interval-generation problem; Gate 8 must not wave it away.

### 4.2.4 Replay/export/import contract

Gate 8 must explicitly separate internal replay truth from public/viewer replay exports.

Required modes:

1. **Internal full trace mode**
   - Native tests, golden traces, replay-check tooling, and dev-only fixtures may contain seed and full command stream with private action choices.
   - This mode is not exposed as a public browser export for hidden-information games.
   - If a full internal trace is ever downloadable in a development build, it must be explicitly fenced as dev/test-only and fail public-build no-leak checks.

2. **Public observer replay export**
   - Default browser export for `observer`.
   - Contains public projections/effects and redacted command summaries.
   - Does not include unrevealed deck order, private hands, private card IDs before reveal, hidden commitments before reveal, full seed material if seed would reconstruct hidden cards, bot private candidates, or raw action paths containing private card IDs.
   - Import replays a public projection timeline, not an omniscient hidden state.

3. **Seat-private replay export**
   - Optional only if useful and safe.
   - May include that seat’s own observations at the times they were authorized.
   - Must not include opponent private cards, unrevealed deck order, or future draw identities.
   - Must be visibly labelled as seat-scoped, not public.

4. **Terminal behavior**
   - Terminal public exports do not automatically reveal all hidden information.
   - A postgame reveal mode is out of scope unless the implementation defines an explicit authorized viewer/policy and passes no-leak tests.
   - Unused deck tail remains hidden by default even after terminal.

WASM/API changes must preserve existing public-perfect-information replay behavior where appropriate, but hidden-info games must not reuse public command-stream export if that leaks action paths, seed, or hidden cards.

### 4.2.5 WASM/API changes

Gate 8 must make viewer-awareness real.

Required Rust/WASM changes:

| Surface | Gate 8 requirement |
|---|---|
| `new_match` | Supports `high_card_duel` default variant and stores hidden internal state Rust-side. |
| `get_view(match_id, viewer_seat)` | Honors `viewer_seat` for all games. Perfect-information games may return equivalent projections for all viewers. Hidden-info games must filter. |
| `get_action_tree(match_id, actor_seat)` | Treat as actor-private. Do not call from observer mode for hidden-info games. |
| `get_action_tree(match_id, actor_seat, viewer_seat)` or replacement | Preferred hardening: add viewer/authorization context so browser cannot accidentally request another seat’s private action tree. |
| `apply_action` | Validates action Rust-side and returns only a view/effects projection for the requested/authorized viewer, or returns a safe public response and requires follow-up `get_view`. |
| `run_bot_turn` | Bot acts from allowed bot input only. Response does not leak bot private candidates, opponent hidden data, deck order, or full post-action hidden state. |
| `get_effects(match_id, since_cursor, viewer_seat)` | Filters public/private effects using existing `EffectLog` semantics and new high-card effects. |
| `export_replay` | Default export for hidden-info games is viewer-scoped/public-safe, not internal full replay. |
| `import_replay` | Distinguishes public projection replay from full internal test/dev trace. |
| `catalog` | Registers `high_card_duel`, display metadata, supported viewer modes, and hidden-info/chance tags. |
| TypeScript bindings | Types must encode viewer mode and prevent accidental `any`/raw hidden-state pass-through. |

A hidden-info game must not ship hidden state to TypeScript/browser and rely on React to hide it.

Existing perfect-information games:

- They should flow through the same viewer-aware API where cheap and safe.
- They may treat all viewers equivalently.
- Their docs/tests should not be rewritten broadly just because the signature becomes viewer-aware.
- Smoke tests must confirm regression-free behavior for at least one existing public game.

### 4.2.6 Web client and UI

Add a polished `HighCardDuelBoard` or equivalent component and integrate it into the app shell.

Required UX:

- game picker includes `High Card Duel`;
- match setup supports the default variant;
- viewer selector supports `Seat 0`, `Seat 1`, and `Observer`;
- hotseat switching is explicit and safe;
- observer mode is fully playable as a watcher but has no private action controls;
- own hand is visible only to current seat viewer;
- opponent hand shows count/backs only;
- face-down commitments remain face-down until reveal effect;
- reveal animation is driven by Rust semantic effects;
- reduced-motion path disables or simplifies animation;
- score, round, phase, lead/reply roles, and terminal result are clear;
- legal actions come only from Rust action tree;
- pending action controls do not leak hidden IDs through labels, paths, data attributes, test IDs, CSS classes, or console logs;
- keyboard operation covers card selection, commit, viewer switch, replay controls, and export/import where present;
- focus management moves predictably after commit/reveal;
- accessible labels do not leak hidden facts;
- responsive layout works in narrow viewport.

Visual design:

- Use original neutral card-like presentation.
- Use CSS, simple geometric marks, generated text, or original local SVG only.
- Avoid casino green felt, chip stacks, blackjack/poker table motifs, commercial card art, copied icons, copied fonts, screenshots, or proprietary trade dress.
- Use “duel cards,” “trail badges,” “runes,” “waypoints,” or similarly neutral terms.

Developer panel and debug surfaces:

- Dev panel may show current viewer, phase, public score, effect cursor, and public-safe action availability.
- Dev panel must not show hidden state, raw private action paths, private card IDs, deck order, candidate rankings, bot explanations, or raw memory.
- Public builds must pass no-leak smoke checks with dev panel closed and, where relevant, open.

### 4.2.7 Bot requirements

Required:

- Level 0 random legal bot for `high_card_duel`.
- Bot action selection must use only the actor’s legal action tree and allowed actor-private view.
- Bot must not read opponent private hand, unrevealed deck order, hidden lead commitment before reply, future draw identities, or internal hidden state outside the actor’s authorized data.
- Same seed + same bot policy/version + same view yields deterministic bot choices.
- Many-seed simulation proves all bot actions legal and terminal completion reliable.
- Bot latency is benchmarked.
- Bot docs in `games/high_card_duel/docs/AI.md` explicitly state hidden-info boundaries.

Allowed but not required:

- Level 1 simple baseline using only own hand, public score, public round, and phase.
- Example Level 1 policy: conserve high cards while ahead, spend higher card while behind or in final rounds, tie-break deterministically by stable local action order.
- Level 1 may not claim competent play unless supported by docs and evidence.
- If Level 1 ships, add tests that compare its chosen inputs to a sanitized bot-input fixture and prove no forbidden hidden fields are available.

Forbidden:

- Level 2 in Gate 8.
- MCTS, ISMCTS, Monte Carlo rollout/search, ML/RL, LLM move choice.
- Bot explanations or candidate rankings in public browser output.
- Omniscient bot helpers, even for “simple” heuristics.

### 4.2.8 Tests

Native tests must include at least the following.

#### Unit/rule tests

- `setup_deals_private_hands_and_hides_deck`
- `same_seed_same_initial_deal_internal`
- `different_seeds_can_change_initial_deal`
- `shuffle_uses_unbiased_bounded_index_or_documented_helper`
- `lead_commit_removes_card_from_own_hand`
- `reply_commit_cannot_see_lead_identity`
- `both_commitments_reveal_together`
- `higher_rank_scores_one_point`
- `tie_round_scores_no_points`
- `refill_restores_hand_size_when_deck_available`
- `lead_alternates_by_round`
- `terminal_after_six_rounds`
- `terminal_winner_and_draw_policy`
- `wrong_seat_diagnostic_public_safe`
- `wrong_phase_diagnostic_public_safe`
- `invalid_private_card_diagnostic_redacted_for_unauthorized`
- `stale_action_diagnostic_no_hidden_leak`
- `observer_has_no_private_commit_actions`

#### Visibility tests

- observer view contains no private hand IDs;
- observer view contains no unrevealed deck order;
- observer view contains no face-down commitment identity;
- `seat_0` view contains `seat_0` hand only;
- `seat_1` view contains `seat_1` hand only;
- seat view after own commit shows own committed card but not opponent hidden commitment;
- reply actor view after lead commit does not contain lead card identity;
- revealed cards become public only after reveal;
- terminal public view still hides unused deck tail;
- disabled reasons/previews/action metadata do not leak private card IDs to unauthorized viewers;
- serialized JSON field names do not contain private identifiers in public projections;
- effect filtering returns the correct public/private event sets for observer, `seat_0`, and `seat_1`.

#### Property/invariant tests

- no duplicate cards across deck, hands, commitments, revealed/discard zones;
- card conservation across every transition;
- no card appears simultaneously in two zones;
- only current actor has private legal commit actions;
- committed hidden cards reveal exactly once;
- score increments only from reveal/compare;
- round count is monotonic;
- terminal state has no gameplay actions;
- public projection never grows hidden fields across random seeds/actions;
- replaying an internal full trace reproduces the same revealed sequence;
- public replay export never contains unrevealed internal card identities.

#### Serialization tests

- strict unknown fields where existing conventions require;
- stable public view schema;
- stable seat-private view schema;
- stable internal trace schema for tests;
- public replay export has no hidden fields;
- importing public replay produces the public projection timeline without reconstructing hidden state;
- importing internal full test trace reconstructs state only in test/dev tooling.

#### Bot tests

- Level 0 chooses only legal actions;
- Level 0 uses actor-private action tree only;
- bot cannot access opponent hand/deck/hidden commitment via exposed input type;
- same seed/policy/version deterministic;
- many-seed terminal simulation;
- bot action trace golden;
- no public bot explanation/candidate leak.

### 4.2.9 Golden traces

Required golden traces:

| Trace | Purpose |
|---|---|
| `shortest-normal.trace.json` | Completed six-round game with normal scoring. |
| `tie-round.trace.json` | At least one tied rank and no point awarded. |
| `invalid-wrong-seat-diagnostic.trace.json` | Wrong actor, no hidden leak. |
| `invalid-private-card-redacted.trace.json` | Invalid private card diagnostic redaction proof. |
| `stale-diagnostic.trace.json` | Stale action behavior. |
| `bot-action.trace.json` | Level 0 bot action proof. |
| `hidden-info-public-observer.trace.json` | Observer projection/effects never leak hidden identities. |
| `seat-private-view.trace.json` | Seat view sees own hand only. |
| `public-replay-export-import.trace.json` | Public replay projection export/import no-leak. |
| `terminal.trace.json` | Terminal winner/draw and no legal actions. |

If the existing trace tooling cannot represent both internal full traces and public projection traces cleanly, Gate 8 must add a minimal hidden-info-safe trace classification rather than forcing public browser exports to use full internal command streams.

### 4.2.10 UI/e2e smoke tests

Add high-card duel smoke coverage.

Required scenarios:

- load app and start `High Card Duel`;
- `Seat 0` sees only Seat 0 private hand;
- `Seat 1` sees only Seat 1 private hand;
- `Observer` sees hand counts and face-down cards only;
- lead commit hides committed card from reply/observer;
- reply commit does not reveal lead card before reveal;
- reveal flow exposes both cards simultaneously;
- score/round advance correctly;
- bot turn does not leak private candidates/explanations;
- replay export/import default is public-safe;
- dev panel open does not reveal hidden information;
- DOM text no-leak;
- DOM attributes no-leak;
- CSS class/test ID no-leak where value-derived;
- console no-leak;
- localStorage/sessionStorage no-leak;
- keyboard/focus path works;
- reduced-motion mode works;
- responsive narrow viewport works;
- a11y scan passes or records bounded, justified exceptions consistent with existing checklist practice.

No-leak search terms should include game-specific hidden tokens, examples:

```text
hidden_state
private_state
internal_state
deck_order
future_draw
card_id
hcd:r
lead_commit_card
opponent_hand
bot_candidate
candidate_ranking
bot_explanation
```

The exact denylist must avoid false positives from public-safe source-code labels in test files; smoke checks should target browser-visible DOM/storage/console/export payloads, not repository source text.

### 4.2.11 Benchmarks

Add native benchmark coverage for:

- setup + deterministic shuffle/deal;
- legal action generation for lead and reply;
- validation of legal/illegal commits;
- apply action for commit/reveal/refill;
- public/observer view projection;
- seat-private view projection;
- effect filtering by viewer;
- public replay export;
- internal replay/check reconstruction;
- serialization/deserialization;
- random legal playout;
- Level 0 bot decision latency;
- optional Level 1 latency if Level 1 ships.

Benchmark thresholds:

- Follow existing ADR lane and threshold conventions.
- Establish thresholds from measured baselines.
- Do not fabricate thresholds.
- If high variance appears, document the baseline sample and choose conservative thresholds.
- CI updates must not weaken existing gates.

### 4.2.12 Documentation

Per-game docs must follow templates and use explicit `not applicable` entries.

Required per-game docs:

- `games/high_card_duel/docs/SOURCES.md`
  - Source notes for high-card comparison inspiration, War not being copied, hidden-info precedents if needed, and original-rule statement.
- `games/high_card_duel/docs/RULES.md`
  - Complete rules with stable rule IDs.
- `games/high_card_duel/docs/RULE-COVERAGE.md`
  - Rule-to-test matrix.
- `games/high_card_duel/docs/MECHANICS.md`
  - Card/deck/hand/commitment/reveal/chance/hidden-info inventory and primitive-pressure notes.
- `games/high_card_duel/docs/AI.md`
  - Level 0 bot and optional Level 1 boundary.
- `games/high_card_duel/docs/UI.md`
  - UX, accessibility, viewer selector, reduced motion, no-leak UI requirements.
- `games/high_card_duel/docs/BENCHMARKS.md`
  - Native and browser benchmark evidence.
- `games/high_card_duel/docs/GAME-IMPLEMENTATION-ADMISSION.md`
  - Admission checklist.
- `games/high_card_duel/docs/PUBLIC-RELEASE-CHECKLIST.md`
  - Public polish and no-leak checklist.
- `games/high_card_duel/docs/COMPETENT-PLAYER.md`
  - `not applicable / deferred` unless a nontrivial Level 1 strategy claim requires a light competent-player note.
- `games/high_card_duel/docs/BOT-STRATEGY-EVIDENCE-PACK.md`
  - `not applicable / Level 2 not shipped`.

Repo-level docs:

- `docs/MECHANIC-ATLAS.md`
  - Extend the existing `high_card_duel` pressure row with local notes for deterministic shuffle, private hand, hidden commitment, reveal, effect filtering, and no-leak replay export (the row already exists at the target commit as "deterministic shuffle and hidden draw → local-only"; broaden it rather than adding a second row).
  - Explicitly state this is first official local card/deck implementation and does not authorize promotion.
  - Name likely future pressure points: `blackjack_lite`, poker-lite, trick-taking, hidden draw/discard games.
- `docs/SOURCES.md`
  - Add source/rationale references only if repo convention expects repo-level source rollup.
- `docs/WASM-CLIENT-BOUNDARY.md`
  - Update only if Gate 8 changes viewer-aware API expectations beyond existing Gate 3 boundary text.
- `docs/TRACE-SCHEMA-v1.md`
  - Update only if trace schema actually changes.
- ADR
  - Create no ADR unless a real architecture decision is made, such as changing replay export taxonomy or adding a generic unbiased RNG helper to `engine-core`.

### 4.2.13 Tool and CI registration

The native verification tools resolve games through hardcoded registries, not
by path. Registering `high_card_duel` is therefore a concrete deliverable, not
an implicit consequence of adding the crate:

| Site | Gate 8 requirement |
|---|---|
| `Cargo.toml` workspace members | Add `games/high_card_duel` (and its bench target) so the crate builds in `cargo build/test --workspace`. |
| `tools/simulate/src/main.rs` | Add `high_card_duel` to the game registry / dispatch and usage string so `simulate --game high_card_duel --games N --start-seed N` runs. |
| `tools/replay-check/src/main.rs` | Add a `RegisteredGame` entry (game id + `trace_dir = games/high_card_duel/tests/golden_traces`) so `replay-check --game high_card_duel` and `--all` cover it. |
| `tools/fixture-check/src/main.rs` | Add a `RegisteredGame` entry so `fixture-check --game high_card_duel` validates the fixture. |
| `tools/rule-coverage/src/main.rs` | Add a `RegisteredGame` entry so `rule-coverage --game high_card_duel` checks the rule-to-test matrix. |
| `.github/workflows/gate-1-game-smoke.yml` | Add `node apps/web/e2e/high-card-duel.smoke.mjs` alongside the existing per-game smoke lines. |
| `.github/workflows/gate-2-benchmarks.yml` | Add a `high_card_duel` bench smoke step matching the existing per-game bench lanes. |

`tools/seed-reducer` and `tools/trace-viewer` are selectively wired (the latter
supports only `race_to_n` / `directional_flip` at the target commit); extend
them only if a Gate 8 workflow actually needs them, and document the choice.

---

# 5. Work breakdown

This section is decomposition guidance, not tickets.

## 5.1 Part A — Gate 7.2 work breakdown

1. Confirm exact current status from `specs/README.md`.
2. Edit `README.md` only enough to remove Gate 5-era stale state.
3. Edit `progress.md` only enough to represent Gates 6, 7, and 7.1 completion and Gate 8 next status.
4. Edit `AGENTS.md` / `CLAUDE.md` to remove stale Race-to-N-only verification cues.
5. Verify `docs/MECHANIC-ATLAS.md` open promotion debt remains empty.
6. Add explicit `blackjack_lite` continuation row/hook in `specs/README.md`.
7. Run documentation/link checks required by current repo conventions.
8. Commit or report Gate 7.2 as done only after the orientation drift is demonstrably gone.

## 5.2 Part B — Gate 8 work breakdown

1. Create crate skeleton, workspace registration, data manifests, variant config, fixture placeholder, and per-game docs.
2. Implement local IDs/state/rules/setup/actions/effects/visibility modules.
3. Implement deterministic shuffle/deal with unbiased bounded index plan and tests.
4. Implement legal action tree and validation.
5. Implement apply/reveal/score/refill/terminal transitions.
6. Implement viewer projections and effect filtering.
7. Implement internal replay support and public/viewer-scoped replay export/import rules.
8. Implement Level 0 random legal bot and optional Level 1 only if safe.
9. Add unit, rule, visibility, property, replay, serialization, bot, and golden trace tests.
10. Register `high_card_duel` in WASM API and TypeScript catalog, in the four native tool registries (`simulate`, `replay-check`, `fixture-check`, `rule-coverage`), and in the CI smoke/bench workflows (Deliverable 4.2.13).
11. Harden viewer-aware API responses and action tree authorization.
12. Build web UI component and integrate it into shell/replay/export/dev panel patterns.
13. Add no-leak/a11y/e2e smoke coverage.
14. Add benchmarks and thresholds from baseline evidence.
15. Update per-game and repo-level docs.
16. Run full verification suite.

## 5.3 Part C — Blackjack continuation work breakdown

1. Before Gate 8 is marked Done, ensure `specs/README.md` contains a blocking continuation checkpoint before Gate 9.
2. After Gate 8 evidence lands, record one of:
   - create a follow-up Gate 8.1 / Gate 8B `blackjack_lite` spec; or
   - record a formal deferral with source-grounded and docs-grounded rationale.
3. The deferral, if chosen, must name the future closure gate and identify what evidence from `high_card_duel` satisfied Gate 8’s hidden-info/chance proof.
4. Gate 9 may not begin while this checkpoint is unresolved.

---

# 6. Exit criteria

## 6.1 Gate 7.2 exit criteria

Gate 7.2 is complete only when:

- future agents can infer from `specs/README.md` that Gates 0–7.1 are done and Gate 8 is next;
- `README.md` no longer claims Gate 5 is the latest completed state;
- `progress.md` no longer makes Gate 3/Gate 5 appear current;
- `AGENTS.md` and `CLAUDE.md` no longer hardcode Race-to-N as the current verification game;
- `docs/MECHANIC-ATLAS.md` still records no open `board_space` promotion debt, or records a concrete exception if one is found;
- `blackjack_lite` continuation is explicit before Gate 9;
- no foundation law was rewritten for style;
- no ADR was added without a real architecture decision;
- doc/link checks pass.

## 6.2 Gate 8 exit criteria

Gate 8 is complete only when:

- `high_card_duel` is a registered official game;
- deterministic shuffle/deal is reproducible from internal seed/trace and has bounded-index bias addressed;
- observer/public projections never expose private hands, hidden commitments, deck order, future draw identities, private bot data, or unused deck tail;
- seat projections expose only that seat’s own private hand/commitment plus public state;
- action trees are actor-private and viewer-authorized;
- reply seat cannot learn lead committed card identity before reveal;
- effects are correctly filtered for observer, `seat_0`, and `seat_1`;
- public browser replay export/import is no-leak by default;
- internal full traces exist only in test/dev-fenced surfaces;
- Level 0 bot can play terminal games legally without hidden-info access;
- optional Level 1, if shipped, passes hidden-info input tests and is documented honestly;
- high-card duel UI is polished, accessible, keyboard-operable, responsive, reduced-motion aware, and no-leak tested;
- dev panel, DOM, attributes, CSS/test IDs, console, storage, replay export, and command summaries pass no-leak checks;
- native tests, UI smoke tests, replay checks, fixture checks, rule coverage, simulations, seed reducer expectations, and benchmarks pass;
- per-game docs are complete or explicitly `not applicable`;
- `docs/MECHANIC-ATLAS.md` records card/deck pressure without promotion;
- `specs/README.md` records Gate 8 outcome and leaves the Blackjack checkpoint resolved or blocking before Gate 9.

## 6.3 Part C exit criteria

The `blackjack_lite` checkpoint is complete only when one of these is true:

1. A follow-up spec exists for `blackjack_lite` as Gate 8.1 / Gate 8B before Gate 9; or
2. `specs/README.md` records a formal deferral that:
   - names the future gate where Blackjack Lite will close or be reconsidered;
   - cites the Gate 8 `high_card_duel` evidence that satisfied deterministic shuffle/private-view/effect-filter/no-leak proof;
   - cites source/rule complexity reasons for not implementing Blackjack immediately;
   - is reflected in roadmap-facing documentation without mutating foundational law unless required.

Gate 9 is blocked until one of those conditions is true.

---

# 7. Acceptance evidence

Implementation must report exact commands and outcomes. Do not say “tests pass” without listing the command set.

Minimum acceptance evidence:

```text
cargo fmt --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
cargo test -p high_card_duel
cargo test -p high_card_duel --test rules
cargo test -p high_card_duel --test visibility
cargo test -p high_card_duel --test property
cargo test -p high_card_duel --test replay
cargo test -p high_card_duel --test serialization
cargo test -p high_card_duel --test bots
cargo run -p fixture-check -- --game high_card_duel
cargo run -p rule-coverage -- --game high_card_duel
cargo run -p replay-check  -- --game high_card_duel
cargo run -p simulate      -- --game high_card_duel --games <documented-count> --start-seed <documented-seed>
cargo run -p seed-reducer  -- <documented failing-seed workflow if applicable>
cargo bench -p high_card_duel
```

These tools are `--game`-keyed against a hardcoded registry; the commands above
run only after `high_card_duel` is registered in each tool (see Deliverable
4.2.13). `simulate` selects the random-legal bot by default and exposes no
`--bot` flag at the target commit; if a bot selector is added it must be a
separate, justified change.

Web evidence (e2e smokes run directly with `node`, matching
`.github/workflows/gate-1-game-smoke.yml`):

```text
npm --prefix apps/web ci
npm --prefix apps/web run build
node apps/web/e2e/high-card-duel.smoke.mjs
node apps/web/e2e/a11y-noleak.smoke.mjs
node apps/web/e2e/shell.smoke.mjs
```

Documentation evidence:

```text
node scripts/check-doc-links.mjs
bash scripts/boundary-check.sh
```

If exact scripts differ at implementation time, use current repo scripts and record the substitution.

Required evidence artifacts:

- Gate 7.2 orientation diff summary, not a raw diff.
- High Card Duel rule coverage matrix.
- Golden trace list and replay-check output.
- Visibility/no-leak test report for observer, `seat_0`, and `seat_1`.
- Public replay export sample with hidden fields absent.
- Internal full trace sample stored only under test/dev path.
- UI smoke report including DOM/storage/console/dev-panel no-leak checks.
- Benchmark baseline and threshold rationale.
- Mechanic atlas pressure note.
- Blackjack continuation checkpoint entry.

---

# 8. FOUNDATIONS & boundary alignment

Canonical alignment table (per `gate-0-repository-skeleton.md` §7). The prose
subsections 8.1–8.7 carry the detail beneath it.

| Principle | Stance | Rationale |
|---|---|---|
| §2 Behavior authority | aligns | Rust owns deck build, deterministic shuffle, deal, legality/action trees, validation, transitions, reveal timing, scoring/terminal, effects, visibility filtering, replay/export, and bot choice; TypeScript stays presentation-only (§8.1). |
| §3 `engine-core` is a contract kernel | aligns | No `card`/`deck`/`hand`/`suit`/`rank`/`pile` mechanic noun enters `engine-core`; all card vocabulary is local to `games/high_card_duel` (§9 Forbidden). |
| §4 `game-stdlib` is earned | aligns | First official card use stays local; the mechanic atlas records local-only pressure and does not promote (§4.2.13 note, §8.7). |
| §5 Static data is not behavior | aligns | Static data is limited to variants, manifests, labels, fixtures, and traces; no selectors/triggers/scoring/shuffle live in data (§8.6). |
| §11 Universal acceptance invariants | aligns | Viewer-safe public/private/observer projections, no-leak effects/exports/DOM, deterministic shuffle/replay, Level 0 legal-API bot, and full evidence set (tests/traces/sims/benches/docs) are required (§4.2.7–§4.2.11, §8.2–§8.5). |
| §12 Stop conditions | clear | No kernel mechanic noun, no TS legality, no hidden-info leak, no YAML/DSL, no legal-API bypass, no third-use mechanic — see the §12 stop-condition review in this spec's deliverables. |
| §13 ADR triggers | conditional | An ADR is required only if Gate 8 adds a generic `engine-core` unbiased-RNG helper or changes replay-export taxonomy/visibility semantics; otherwise none (§4.2.3, §10.2). |

## 8.1 Rust ownership

Rust must own:

- deck construction;
- deterministic shuffle;
- deal;
- private hand storage;
- commitment storage;
- legality/action trees;
- validation;
- state transitions;
- reveal timing;
- score/terminal logic;
- effect emission;
- visibility filtering;
- replay/export/import semantics;
- bot input construction and action selection.

TypeScript may not recompute legality, reveal hidden cards, infer private action availability, shuffle, score, validate, or decide bot moves.

## 8.2 Viewer-safe browser payloads

The browser receives only viewer-authorized JSON. This includes all direct and indirect surfaces:

- public payloads;
- private seat views;
- observer/no-seat views;
- action trees;
- previews;
- diagnostics/disabled reasons;
- effects;
- command logs when exposed;
- replay export/import;
- trace exports/golden-trace viewer;
- DOM text;
- DOM attributes;
- CSS class names if value-derived;
- test IDs;
- browser console logs;
- localStorage/sessionStorage;
- dev panel;
- bot explanations;
- candidate rankings;
- debug traces.

No implementation may send hidden state to React and rely on conditional rendering to hide it.

## 8.3 Action-tree privacy

Action trees are private data. For hidden-information games:

- actor-private action tree may include own card choices;
- opponent and observer action trees are empty, unavailable, or public-safe;
- action paths/labels must not leak hidden card identity through UI or logs;
- old perfect-information games may keep public-equivalent behavior but should use the same viewer-aware pathway if cheap.

## 8.4 Effect-log privacy

Effect visibility must be explicit and tested.

- Public effects contain only public facts.
- Private seat effects contain only facts that seat may know.
- No private event payload can be downgraded to public for convenience.
- Terminal does not automatically make all hidden state public.

## 8.5 Replay/hash behavior

Internal deterministic replay remains important, but public replay export for hidden information must be no-leak by default. If current architecture assumes public export is seed + command stream, Gate 8 must split that assumption for hidden-info games rather than leaking private choices.

## 8.6 Static data boundary

Static data may define variants, manifests, labels, and fixtures. Static data may not define hidden-info behavior, scoring, reveal, shuffle, legality, or bot policy.

## 8.7 Primitive promotion boundary

Cards/deck/hands/commitments are first official use in Gate 8. They remain local. If `blackjack_lite` later creates second pressure, that later gate compares local implementations and records pressure. Third official use is the hard point for reuse/promotion/deferral/ADR under current atlas doctrine.

---

# 9. Forbidden changes

Do not:

- add `card`, `deck`, `hand`, `pile`, `bet`, `suit`, `rank`, `trump`, `hole_card`, or equivalent mechanic nouns to `engine-core`;
- add generic card/deck/hand/pile primitives to `game-stdlib` in Gate 8;
- move shuffle, legality, reveal, scoring, visibility filtering, replay filtering, or bot choice to TypeScript;
- use YAML/static data as behavior;
- introduce a game-description DSL;
- fetch or expose hidden state to unauthorized browser payloads;
- hide private state in React after it has already crossed WASM;
- leak private state through DOM text, DOM attributes, CSS classes, test IDs, console logs, local/session storage, replay export, dev panel, bot explanations, candidate rankings, command logs, or traces exposed in public builds;
- expose full internal state in public builds;
- auto-reveal unused deck/order/hidden history at terminal;
- copy War, Blackjack, poker, or commercial rulebook prose;
- copy commercial card art, table layouts, fonts, screenshots, icons, or trade dress;
- add betting/chips/payouts/casino motifs;
- add hosted multiplayer/accounts/databases/chat/matchmaking/ranked play;
- add MCTS, ISMCTS, Monte Carlo rollout/search, ML, RL, or LLM move selection;
- weaken or delete tests to go green;
- silently update golden traces/hashes without a migration rationale;
- create an ADR for routine implementation details;
- broaden Gate 7.2 into a docs rewrite.

---

# 10. Documentation updates required

## 10.1 Gate 7.2 docs

- `specs/README.md`
- `README.md`
- `progress.md`
- `AGENTS.md`
- `CLAUDE.md`
- `docs/MECHANIC-ATLAS.md` if a minimal clarification is needed

## 10.2 Gate 8 repo docs

- `specs/README.md`
- `docs/MECHANIC-ATLAS.md`
- `docs/SOURCES.md` if repo-level source index is expected
- `docs/WASM-CLIENT-BOUNDARY.md` if viewer-aware API expectations change
- `docs/TRACE-SCHEMA-v1.md` only if trace schema changes
- ADR only if a real architecture decision is needed

## 10.3 Gate 8 game docs

- `games/high_card_duel/docs/SOURCES.md`
- `games/high_card_duel/docs/RULES.md`
- `games/high_card_duel/docs/RULE-COVERAGE.md`
- `games/high_card_duel/docs/MECHANICS.md`
- `games/high_card_duel/docs/AI.md`
- `games/high_card_duel/docs/UI.md`
- `games/high_card_duel/docs/BENCHMARKS.md`
- `games/high_card_duel/docs/GAME-IMPLEMENTATION-ADMISSION.md`
- `games/high_card_duel/docs/PUBLIC-RELEASE-CHECKLIST.md`
- `games/high_card_duel/docs/COMPETENT-PLAYER.md` as `not applicable / deferred` unless strategy evidence requires otherwise
- `games/high_card_duel/docs/BOT-STRATEGY-EVIDENCE-PACK.md` as `not applicable / Level 2 not shipped`

## 10.4 Blackjack continuation docs

At minimum:

- `specs/README.md` records the checkpoint.
- `docs/MECHANIC-ATLAS.md` records Blackjack as future card/hidden-info pressure if not implemented immediately.
- If deferral changes roadmap semantics rather than progress tracking, update the relevant roadmap-facing doc or create a justified ADR.

---

# 11. Sequencing

1. Perform Gate 7.2 first.
2. Verify Gate 7.2 exit criteria.
3. Begin Gate 8 crate/data/docs skeleton.
4. Implement hidden-info-safe Rust state/rules/projection before UI.
5. Implement shuffle/deal and visibility tests before browser integration.
6. Harden WASM viewer APIs before rendering private hands.
7. Add UI only after Rust projections prove no-leak behavior.
8. Add replay/export/import split before public replay UI is enabled.
9. Add e2e no-leak tests before public polish signoff.
10. Add benchmarks after behavior is stable.
11. Complete docs and admission checklist.
12. Resolve or explicitly block on the Blackjack continuation checkpoint.
13. Mark Gate 8 done only after evidence is attached.

Gate 9 must not start until Part C is resolved.

---

# 12. Assumptions

- The user-supplied commit is the target of record; this spec does not verify latest `main`.
- The uploaded manifest accurately lists paths available at the target commit.
- The existing engine viewer/effect vocabulary is intended to support hidden-information games, but WASM/client surfaces need hardening.
- `high_card_duel` is sufficient as the first proof because it isolates deterministic shuffle, hidden private hands, hidden commitments, viewer-scoped action trees, effect filtering, replay export safety, bot fairness, and public card UI without Blackjack’s rule baggage.
- Blackjack remains valuable as a later pressure test. Deferral must be explicit and recorded, not implicit.
- Existing perfect-information games can remain public-equivalent under viewer-aware APIs.
- Public replay exports for hidden-information games must prioritize no-leak safety over full hidden-state reconstructability.
- Internal full traces may continue to serve deterministic replay/hash/test evidence if fenced away from public browser exports.
- No card/deck primitive promotion is justified by one official card game.
- If implementation discovers a foundation contradiction, the implementation agent must stop and write a narrow findings note rather than improvising architecture changes.

---

# Appendix A — Why `high_card_duel` first, not `blackjack_lite`

`high_card_duel` directly targets the Gate 8 proof: deterministic chance, private hand, face-down commitment, viewer-scoped legal actions, filtered effects, and public no-leak export. It keeps rule complexity low enough that hidden-information surfaces can be tested exhaustively.

Plain War is not enough: common War rules are largely automatic high-card comparison with little or no strategy, and Pagat explicitly notes that there is normally no strategy involved. That makes War too weak to prove private action-tree choices and bot fairness.

Blackjack Lite is useful later, but it brings more baggage than Gate 8 needs: dealer policy, hole card handling, hit/stand loops, bust/push/natural logic, optional betting/payout concepts, soft-hand rules, and casino-adjacent UI risk. That extra complexity is better handled after the first hidden-info/chance proof lands.

External precedents support the direction without dictating architecture. boardgame.io’s `playerView` is a useful comparison point for player-specific state, but Rulepath must be stricter because hidden state should not cross WASM into unauthorized TypeScript/browser surfaces. OpenSpiel demonstrates that stochastic and imperfect-information games are a serious general-game concern; Rulepath should borrow the observation/information-state discipline without trying to become OpenSpiel. Ludii research reinforces that true universality requires formal language/tooling work, so Gate 8 should not invent a generic DSL or card engine prematurely.

---

# Appendix B — Suggested `specs/README.md` checkpoint wording

Use this exact wording or a semantically equivalent version:

> Gate 8 admits `high_card_duel` as the first chance/hidden-information proof. It does not erase `blackjack_lite` from the roadmap. Before advancing to Gate 9, the implementation summary or a follow-up spec must decide whether Blackjack Lite is required as Gate 8.1 / Gate 8B to finish Stage 6. If Blackjack Lite is deferred, the deferral must be explicit, source-grounded, docs-grounded, and recorded in `specs/README.md` with a named closure gate.

---

# Appendix C — Minimal Gate 7.2 edit policy

Gate 7.2 should be boring. If an edit is not needed for future-agent orientation, leave it alone.

Allowed:

- fix stale gate number/status;
- update official-game list;
- update active verification guidance;
- record next gate;
- record Blackjack checkpoint;
- confirm board-space promotion debt closure.

Not allowed:

- rewrite foundations;
- rename architecture concepts;
- reformat whole docs;
- add aspirational roadmap prose;
- create ADRs for ordinary cleanup;
- sneak in Gate 8 implementation changes before Gate 7.2 exit.


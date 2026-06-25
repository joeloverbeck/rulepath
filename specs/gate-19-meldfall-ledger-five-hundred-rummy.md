# Gate 19 Implementation Spec — Meldfall Ledger (Five Hundred Rummy / Rummy 500)

> **Spec path:** `specs/gate-19-meldfall-ledger-five-hundred-rummy.md`  
> **Deliverable type:** new roadmap-gate implementation spec, not ticket decomposition  

## 1. Header

| Field | Value |
|---|---|
| Spec ID | `GATE-19-MELDFALL-LEDGER-FIVE-HUNDRED-RUMMY` |
| Stage | Public scaling phase, Gate 19 |
| Gate | Gate 19 — Five Hundred Rummy / Rummy 500 family |
| Status | `Planned` / implementation `Not started` |
| Date | 2026-06-25 |
| Owner | Rulepath maintainers |
| Authority order | `docs/README.md` → `docs/FOUNDATIONS.md` → `docs/ARCHITECTURE.md` → `docs/ENGINE-GAME-DATA-BOUNDARY.md` → area docs → `docs/ROADMAP.md` → this spec → future tickets. Accepted ADRs supersede only the sections they name. |
| Internal game id | `meldfall_ledger` |
| Public display name | **Meldfall Ledger** |
| Rules-family label | Five Hundred Rummy / Rummy 500 / 500 Rum family |
| Neutral-name rationale | Rulepath catalog convention uses original evocative names while keeping public-domain/common names in source notes. “Meldfall Ledger” evokes public meld growth and cumulative score accounting without presenting itself as the source game name. Human IP/legal review remains required under `docs/IP-POLICY.md`. |
| Variant id | `classic_500_single_deck_v1` |
| Rules version | `meldfall-ledger-rules-v1` |
| Trace rules version | `trace-schema-v1+meldfall-ledger-rules-v1` |
| Data / manifest version | `meldfall-ledger-data-v1`; typed parameters and presentation metadata only. |
| Browser implementation required | Yes. Gate 19 exit requires a usable larger hand + public tableau action surface. |
| Official seat declaration | Variable seat game: min 2, max 6, default 4, supported set `{2, 3, 4, 5, 6}`. |
| Seat keys | `seat_0` through `seat_5`; display labels default to `Seat 1` through `Seat 6` unless the shared shell supplies player labels. |
| Roles | No persistent roles. Dealer, active seat, and round starter are public turn-order state only. |
| Teams / partnerships | **Absent.** Individual competitive game; each seat scores for itself. |
| Public-observer stance | Supported. Public observer sees public meld tableau, public discard pile, stock count only, hand counts, scores, active seat, dealer/turn order, diagnostics, and public effects. Public observer never sees private hands or unseen stock order. |
| Hidden-information stance | Each seat’s hand and the unseen stock order are hidden. A seat-private viewer sees only that seat’s hand. Opponents’ hands, stock order, bot private rankings, hidden action labels, effect logs, DOM, storage, replay exports, and diagnostics must not leak hidden card identities. |
| Bot floor | L0 random-legal required. L1 rule-informed allowed after docs/tests. L2 authored meld/lay-off/discard policy is deferred behind `COMPETENT-PLAYER.md` + `BOT-STRATEGY-EVIDENCE-PACK.md`. L3 is not admissible because this is N-seat imperfect information. |
| Kernel stance | Gate 19 consumes the foundation set. No foundation amendment is expected. |
| Primitive stance | Meld validation, public meld tableau, draw/discard zones including multi-card discard pickup, laying off onto any player’s meld, and multi-round cumulative scoring to a 500 target are **first official uses** and stay game-local. |
| Scaffolding stance | Gate 19 is the **second `forward-v1` reuse-first scaffolding audit user** after Gate 18. The audit, prior-game disposition, and `ci/scaffolding-audits.json` receipt are gate requirements. |
| Delivery posture | Author this spec only. Future `/reassess-spec` and `/spec-to-tickets` steps decompose it into bounded `AGENT-TASK` packets. |

---

## 2. Objective

Gate 19 implements **Meldfall Ledger**, Rulepath’s neutral presentation of the Five Hundred Rummy / Rummy 500 family, as the next roadmap unit after Gate 18. The determination is settled and this spec only confirms it:

- `specs/README.md` marks Gate 18 — Blackglass Pact / Spades — `Done` on 2026-06-25, and marks Gate 19 — Five Hundred Rummy — as the lowest not-done active-epoch unit.
- `docs/MECHANIC-ATLAS.md` §10A is empty at the Gate 18 closeout, so no open promotion debt blocks the next mechanic-ladder gate.
- `docs/ROADMAP.md` admits Gate 19 as the public-scaling row for draw/discard piles, public meld tableau, private hands, multi-round score target, larger card-zone/action-surface proof, and meld/tableau primitive pressure.
- Accepted ADRs 0004, 0007, 0008, and 0009 are already in force; Gate 19 consumes them rather than reopening them.

The gameplay objective is to ship a full classic single-deck Five Hundred Rummy variant with:

1. sets and runs as legal melds;
2. public meld tableau grouped by originating meld and score-credit ownership;
3. laying off onto any player’s existing melds;
4. stock draw and public discard-pile draw, including the signature multi-card discard-pile pickup with an immediate-use commitment for the deepest selected card;
5. private hands and an unseen stock order with viewer-scoped no-leak evidence;
6. going out, stock-exhaustion round settlement, per-card scoring, in-hand penalties, cumulative multi-round scores, and a 500-point match target with unique-winner tie continuation;
7. browser presentation proving that a large private hand plus public multi-seat meld tableau plus stock/discard zones remains usable.

Gate 19 is deliberately **not** another trick-taking gate. It does not reuse `game-stdlib::trick_taking`, and it must not re-emit the shipped Plain Tricks, Briar Circuit, Vow Tide, or Blackglass Pact work as if missing. Those games are comparison baselines only. River Ledger supplies the closest N-seat hidden-information/no-leak and Rust-owned outcome pattern; Vow Tide supplies variable-N setup plumbing; Blackglass Pact supplies the most recent new-game crate and first `forward-v1` audit anatomy.

---

## 3. Scope

### 3.1 In scope

| Area | Gate 19 scope |
|---|---|
| New game crate | Create `games/meldfall_ledger` as an official game crate with Rust-owned behavior, local card/meld/tableau/pile/scoring nouns, docs, fixtures, tests, benches, and public web surface. |
| Variant | Full classic Five Hundred Rummy / Rummy 500 family, pinned as `classic_500_single_deck_v1`. Use one standard 52-card deck for all supported seats, no jokers. |
| Seat count | Variable 2–6 seats; default 4; setup diagnostics for unsupported seat counts. Single deck across the whole supported range. |
| Deal | 2 seats: 13 cards each. 3–6 seats: 7 cards each. One face-up initial discard; remaining cards form face-down stock. Left of dealer acts first; clockwise play. |
| Turn flow | Draw from stock or from discard pile; optionally meld/lay off one or more times; discard unless the seat empties its hand through melds/layoffs. |
| Melds | Sets of 3–4 same-rank cards; runs of 3+ consecutive cards in the same suit. Aces may be low or high but not around-the-corner; Rulepath scoring values aces at 15 points in all contexts. |
| Laying off | A seat may lay off onto any existing public meld, regardless of who opened it, when the resulting meld remains legal. Score credit belongs to the seat that played the laid-off card. |
| Discard-pile pickup | The discard pile is public and ordered. A seat may choose any visible discard and take that card plus all newer cards above it. The chosen deepest card must be used immediately in a new meld or lay-off during the same turn. This immediate-use rule also applies to choosing the top discard in this variant. |
| Going out | A round ends when a seat has no cards after melding/laying off all cards, or after melding/laying off all but one card and discarding that last card. A final discard is not required. No floating. |
| Stock exhaustion | If stock is exhausted and the active seat cannot or will not legally draw from the discard pile, the round ends and scores. |
| Scoring | Melded/laid-off cards score positive to the seat that played them. Cards left in hand score negative to the holding seat. Values: ace = 15, K/Q/J/10 = 10, 2–9 = pip value. Scores may be negative. Cumulative match scores continue across rounds. |
| Match end | First seat to reach or exceed 500 after a round is eligible to win. If exactly one seat has the highest score at or above 500, that seat wins. If the highest at/above-500 score is tied, continue rounds until a unique highest winner exists. |
| Visibility | Public surfaces: meld tableau, discard pile, scores, hand counts, active seat, dealer, stock count. Private surfaces: each seat’s hand and stock order. |
| Bot floor | L0 random-legal bot. L1 rule-informed bot may use public/own-private features only. L2 is deferred behind competent-player evidence. |
| Forward governance | Run the second `forward-v1` scaffolding reuse-first audit before implementation admission; update register/docs/CI receipt. |
| Official docs | Fill every required game-local template document, including `PRIMITIVE-PRESSURE-LEDGER.md` and `GAME-EVIDENCE.md`. |
| Browser | Add `MeldfallLedgerBoard.tsx`, catalog/rules/renderer/smoke wiring, keyboard-safe action builder, and no-leak/a11y smoke checks. |

### 3.2 Out of scope

| Item | Disposition |
|---|---|
| Jokers / wild cards | Not implemented. Record as sourced house variant only. |
| Two-deck shoe for 5+ seats | Not implemented. Record as sourced convention only; Gate 19 intentionally uses one 52-card deck across 2–6 seats. |
| Opening minimum / “must have 30 before melding” | Not implemented. |
| Calling “Rummy” on discarded playable cards | Not implemented. |
| Frozen discard pile / canasta-style restrictions | Not implemented. |
| Floating / must discard to go out | Not implemented. Final discard not required. |
| Around-the-corner runs (`Q-K-A-2`) | Not implemented. |
| Moving/rearranging already tabled melds | Not implemented. Existing meld groups may be extended only if the resulting group remains legal. |
| Partnerships, teams, alliances | Not implemented; explicitly absent. |
| Any trick-taking behavior | Not applicable; no tricks, follow-suit, trump, bids, contracts, nils, bags, or partnership scoring. |
| Promoting rummy helpers to `game-stdlib` | Not allowed at first official use. |
| Adding card/meld/tableau/pile nouns to `engine-core` | Not allowed. |
| Static-data rule formulas | Not allowed. Static files may hold identifiers, version labels, presentation text, fixture profiles, and typed parameters only. |
| Ticket generation | Out of scope. This spec enumerates candidate tasks only. |

### 3.3 Not allowed

- TypeScript must not validate melds, infer legal lay-offs, compute scores, decide discard-pile pickup legality, filter action trees, or decide going out.
- Rust/WASM must be the only source for setup validation, action legality, previews, effects, scoring, terminal outcome, bot actions, and viewer-scoped exports.
- Do not add YAML or a DSL. Do not encode selectors, triggers, branches, loops, scoring formulas, meld legality, lay-off legality, or discard pickup rules in static data.
- Do not weaken, delete, or mark tests ignored to get green.
- Do not create a foundation amendment unless implementation finds a genuine contract gap; if that happens, stop and require an ADR or explicitly documented foundation change according to `docs/README.md` authority order.

---

## 4. Deliverables

### 4.1 New crate and source layout

Create the following crate skeleton and keep all rummy nouns local to it:

```text
games/meldfall_ledger/
  Cargo.toml
  benches/
    meldfall_ledger.rs
    thresholds.json
  data/
    manifest.toml
    variants.toml
    fixtures/
      meldfall_ledger_2p_standard.fixture.json
      meldfall_ledger_4p_standard.fixture.json
      meldfall_ledger_6p_standard.fixture.json
      meldfall_ledger_multi_discard_pickup.fixture.json
      meldfall_ledger_layoff_any_tableau.fixture.json
      meldfall_ledger_500_tie_continues.fixture.json
  docs/
    SOURCES.md
    RULES.md
    RULE-COVERAGE.md
    MECHANICS.md
    GAME-IMPLEMENTATION-ADMISSION.md
    HOW-TO-PLAY.md
    COMPETENT-PLAYER.md
    BOT-STRATEGY-EVIDENCE-PACK.md
    AI.md
    UI.md
    BENCHMARKS.md
    GAME-EVIDENCE.md
    PRIMITIVE-PRESSURE-LEDGER.md
    PUBLIC-RELEASE-CHECKLIST.md
  src/
    lib.rs
    ids.rs
    cards.rs
    setup.rs
    variants.rs
    state.rs
    actions.rs
    rules.rs
    scoring.rs
    effects.rs
    visibility.rs
    replay_support.rs
    bots.rs
    ui.rs
  tests/
    bots.rs
    golden_traces.rs
    golden_traces/*.trace.json
    property.rs
    replay.rs
    rules.rs
    serialization.rs
    visibility.rs
```

Module responsibilities:

| Module | Required responsibility |
|---|---|
| `ids.rs` | Game id, variant id, rules version, fixture profile ids, local card/meld/discard identifiers. |
| `cards.rs` | Local `Suit`, `Rank`, `CardId`, card values, deterministic deck construction, rank ordering and ace-run helpers. No shared card noun. |
| `setup.rs` | Variable 2–6 seat declaration, diagnostics, dealer/start-seat rotation, local deterministic shuffle/deal using engine RNG primitives only. |
| `variants.rs` | Typed `classic_500_single_deck_v1` parameters. Static data may mirror typed constants but must not drive behavior through formulas or selectors. |
| `state.rs` | Match/round state, private hands, stock, public discard pile, public meld tableau, score ledger, pending discard-pickup commitment, terminal summaries. |
| `actions.rs` | Rust-owned action tree and command payloads for draw-source choice, discard-pile index selection, meld creation, lay-off, discard, and turn finish. |
| `rules.rs` | Legality for setup, draw, meld, lay-off, discard, going out, stock exhaustion, stale/wrong-seat diagnostics, and action previews. |
| `scoring.rs` | Positive meld/lay-off scoring, in-hand penalties, round settlement, cumulative match scoring, 500 target, unique-winner tie continuation. |
| `effects.rs` | Viewer-safe semantic effects: draw group, meld group, lay-off group, discard group, round-score group, match-terminal group. |
| `visibility.rs` | Public observer and seat-private view projection, action-tree redaction, effect redaction, replay-export filtering, pairwise no-leak utilities. |
| `replay_support.rs` | Trace Schema v1 integration, export/import v2, fixture completion profile, hash labels under ADR 0009. |
| `bots.rs` | L0 random legal and optional L1 rule-informed policy using only public + own-private + allowed inference features. |
| `ui.rs` | Rust-owned presentation metadata: public zone labels, action affordance groups, a11y-safe labels, effect animation hints. No legality in TS. |

### 4.2 Official game documents

Fill the template-derived docs, with explicit `not applicable` rows where required:

| Document | Gate 19-specific content required |
|---|---|
| `docs/SOURCES.md` | Public-domain/common-name research, variant pinning, deliberate deviations, neutral-name/IP note, prior-art implementation notes, strategy references. |
| `docs/RULES.md` | Original prose rules for Meldfall Ledger; no copied rule text. |
| `docs/RULE-COVERAGE.md` | Matrix covering every source rule, variant decision, exclusion, diagnostic, and trace. |
| `docs/MECHANICS.md` | Local model for melds, public tableau, discard/stock zones, lay-off, cumulative scoring, no trick-taking reuse. |
| `docs/GAME-IMPLEMENTATION-ADMISSION.md` | Admission checklist, exact authority references, forward-v1 audit evidence, first-use primitive decisions. |
| `docs/HOW-TO-PLAY.md` | Player-facing original guide using the neutral name first, family label in source note. |
| `docs/COMPETENT-PLAYER.md` | Rummy-specific competence: hand-shaping, meld timing, discard risk, discard-pile pickup risk/reward, high-card penalty management. |
| `docs/BOT-STRATEGY-EVIDENCE-PACK.md` | Required before any L2 admission; for Gate 19 initial ship may mark L2 `not admitted`. |
| `docs/AI.md` | L0 required; L1 allowed if implemented; L2 deferred; hidden-info bot fields; no MCTS/ISMCTS/Monte Carlo/ML/RL. |
| `docs/UI.md` | Large-hand/tableau layout, keyboard-only operation, no-drag-required interaction, effect grouping, no-leak a11y labels. |
| `docs/BENCHMARKS.md` | Seat-count profiles, max 6-seat/action-surface budgets, bot-vs-bot simulation budgets, benchmark thresholds. |
| `docs/GAME-EVIDENCE.md` | Completion profile, fixture profile, command receipts, export coverage, no-leak matrix, forward-v1 receipt path. |
| `docs/PRIMITIVE-PRESSURE-LEDGER.md` | New first-use `local-only` entries for meld validation, public tableau/zone model, draw/discard piles with multi-card pickup, lay-off onto any tableau, multi-round cumulative scoring. |
| `docs/PUBLIC-RELEASE-CHECKLIST.md` | IP, source-note, no-leak, web, smoke, docs, benchmark, evidence receipts. |

### 4.3 Repository registrations

| Area | Required changes |
|---|---|
| Cargo/workspace | Add `games/meldfall_ledger` to the workspace and crate dependency graph without reversing dependency direction. |
| CI game catalog | Add a `meldfall_ledger` entry to `ci/games.json` — shape `{ "id": "meldfall_ledger", "sim_flags": "--seat-count 4 --action-cap 4096", "e2e": "meldfall-ledger.smoke.mjs" }` — plus any game-list checker inputs. `ci/games.json` is also the driver for `scripts/copy-player-rules.mjs` and is validated by `scripts/check-ci-games.mjs`. |
| WASM | Add constants/display name, game module wrapper, catalog metadata, setup/action/view/export dispatch in `crates/wasm-api/src/{constants.rs,games.rs,catalog.rs,lib.rs}` and a new `crates/wasm-api/src/games/meldfall.rs`. |
| Tools | Register `meldfall_ledger` in `tools/simulate` (game-id constant, validation chain, seat-count validation, bot dispatch, by-seat summaries). `replay-check`, `fixture-check`, and `rule-coverage` are **not** generic — `replay-check` carries a per-game `trace_dir` table and `fixture-check`/`rule-coverage` carry a `--game` allowlist — so add an explicit `meldfall_ledger` entry to all three. |
| Web catalog | Add the `GamePicker` entry, shell renderer dispatch (`apps/web/src/main.tsx` view type-guard + board switch), `MeldfallLedgerBoard.tsx`, effect presenter mapping if needed, and `apps/web/e2e/meldfall-ledger.smoke.mjs` — and register that smoke file in the `smoke:e2e` chain in `apps/web/package.json`. The player-rules asset `apps/web/public/rules/meldfall_ledger.md` and the `manifest.json` entry are **generated** by `node scripts/copy-player-rules.mjs` from `games/meldfall_ledger/docs/HOW-TO-PLAY.md`; author the source doc and regenerate — do not hand-author the generated files. |
| Web docs | Update `apps/web/README.md` intro catalog list, Shell Surface renderer list, and Smoke Layers `smoke:e2e` list so `scripts/check-catalog-docs.mjs` passes. |
| Scaffolding audit | Add a `meldfall_ledger` `forward-v1` entry to `ci/scaffolding-audits.json`; keep it behavior-free and checker-compliant. |
| Foundation docs | No foundation amendment expected. Update only `docs/MECHANIC-ATLAS.md`, `docs/MECHANICAL-SCAFFOLDING-REGISTER.md`, `docs/SOURCES.md`, and `specs/README.md` as described in §10. |

### 4.4 `forward-v1` audit receipt deliverable

Gate 19 must add its own `ci/scaffolding-audits.json` entry. The implementation ticket must validate the exact accepted schema from `scripts/check-scaffolding-governance.mjs`; the intended content is:

```json
{
  "id": "meldfall_ledger",
  "coverage": "forward-v1",
  "evidence_paths": [
    "games/meldfall_ledger/docs/MECHANICS.md",
    "games/meldfall_ledger/docs/PRIMITIVE-PRESSURE-LEDGER.md",
    "games/meldfall_ledger/docs/GAME-EVIDENCE.md",
    "docs/MECHANIC-ATLAS.md",
    "docs/MECHANICAL-SCAFFOLDING-REGISTER.md"
  ],
  "register_entries_reviewed": [
    "MSC-8C-001",
    "MSC-8C-002",
    "MSC-8C-003",
    "MSC-8C-004",
    "MSC-8C-005",
    "MSC-8C-006",
    "MSC-8C-007",
    "MSC-8C-008",
    "MSC-8C-009",
    "MSC-8C-010"
  ],
  "register_decisions": [],
  "disposition": "no-new-scaffolding",
  "prior_matching_games": [],
  "follow_on_unit": null,
  "no_follow_on_decision": "MSC-8C-010",
  "known_signal_dispositions": [
    {
      "signal": "MSC-8C-001.effect-envelope-literal",
      "decision": "reused",
      "evidence": "docs/MECHANICAL-SCAFFOLDING-REGISTER.md"
    },
    {
      "signal": "MSC-8C-002.local-seat-grammar",
      "decision": "reused",
      "evidence": "docs/MECHANICAL-SCAFFOLDING-REGISTER.md"
    },
    {
      "signal": "MSC-8C-004.local-action-tree-v1-framing",
      "decision": "reused",
      "evidence": "docs/MECHANICAL-SCAFFOLDING-REGISTER.md"
    },
    {
      "signal": "MSC-8C-005.local-stable-byte-writer",
      "decision": "not-present",
      "evidence": "docs/MECHANICAL-SCAFFOLDING-REGISTER.md"
    },
    {
      "signal": "MSC-8C-006.production-support-edge",
      "decision": "not-present",
      "evidence": "docs/MECHANICAL-SCAFFOLDING-REGISTER.md"
    }
  ],
  "compatibility": {
    "hash_migration": "none",
    "visibility_migration": "none",
    "determinism_migration": "none",
    "migration_authority": "none"
  }
}
```

If implementation invents a new behavior-free shape, the JSON must change from `no-new-scaffolding` to the checker-approved disposition that matches the register update. The receipt must not smuggle meld, lay-off, discard-pickup, scoring, bot, or visibility behavior into the scaffolding register.

---

## 5. Work breakdown

These are **candidate AGENT-TASK items**, not ticket files. `/spec-to-tickets` decomposes them later.

| Order | Candidate task | Depends on | Required output |
|---:|---|---|---|
| 1 | Forward-v1 reuse-first audit and implementation admission gate | This spec | Audit C-01…C-10, document reuse/not-applicable rows, decide no prior-game retrofit unless a pure scaffolding shape is discovered, draft `ci/scaffolding-audits.json` receipt, fill admission doc skeleton. This item blocks behavior implementation. |
| 2 | Crate skeleton, metadata, and source/IP docs | 1 | `games/meldfall_ledger` crate, local docs copied from templates, `data/manifest.toml`, `data/variants.toml`, neutral-name notes, original source notes, typed variant constants only. |
| 3 | Variable 2–6 setup and deterministic single-deck deal | 2 | Seat-count declaration/diagnostics, 52-card local deck, local shuffle/deal, 13-card 2p deal, 7-card 3–6 deal, initial discard, stock, dealer/start-seat rotation, setup traces for 2/4/6 and invalid counts. |
| 4 | State, action, effect, and replay skeleton | 3 | `MatchState`, `RoundState`, public/private zones, action enums, semantic effect groups, trace/export scaffolding under ADR 0009. |
| 5 | Meld validation: sets and runs | 4 | Local meld legality for sets, runs, ace-low/high/no-wrap, duplicate ownership diagnostics, test/property coverage, first-use ledger entry. |
| 6 | Public meld tableau model | 5 | Public tableau grouping, meld ownership and per-card score-credit ownership, stable ids, public projection, no helper promotion, first-use ledger entry. |
| 7 | Lay-off onto any tableau | 6 | Lay-off legality onto own or opponent melds, score-credit attribution to laying-off seat, public effects, invalid lay-off diagnostics, first-use ledger entry. |
| 8 | Draw/discard zones and multi-card discard-pile pickup | 4,5,6,7 | Stock draw, discard draw by visible index, “take selected card plus newer cards,” immediate-use commitment for deepest selected card, public/seat-private effects, cannot discard the just-drawn discard when top pickup remains unmelded, first-use ledger entry. |
| 9 | Turn lifecycle, going out, stock exhaustion | 7,8 | Draw/meld/lay-off/discard phase model, no-final-discard go-out path, discard go-out path, stock-exhaustion round settlement, stale/wrong-phase diagnostics. |
| 10 | Round and match scoring | 6,7,9 | Card-value table, positive meld/layoff scores, in-hand penalties, cumulative match scores, 500 target, unique-winner tie continuation, terminal rankings, first-use ledger entry. |
| 11 | Visibility, exports, no-leak harness | 4–10 | Public/seat-private views, action-tree redaction, preview redaction, effect redaction, replay export/import v2, public observer + all six seat-private export coverage, pairwise no-leak tests. |
| 12 | Bots and strategy docs | 10,11 | L0 random-legal bot. Optional L1 rule-informed bot based on public/own-private features. `AI.md`, `COMPETENT-PLAYER.md`, strategy evidence note. L2 remains blocked unless evidence pack is accepted. |
| 13 | Golden traces, fixtures, coverage matrix, properties | 5–12 | Trace Schema v1 golden traces, fixture profiles, rule-coverage matrix, serialization/replay/property tests, completion profile. |
| 14 | WASM and tools registration | 11–13 | WASM constants/catalog/dispatch; `simulate` game id, seat-count CLI validation, bot dispatch, by-seat summaries; explicit `meldfall_ledger` registration in `replay-check` (per-game `trace_dir` table), `fixture-check`, and `rule-coverage`. |
| 15 | Web renderer and large-surface UI proof | 11,14 | `MeldfallLedgerBoard.tsx`, card-zone presenter, keyboard-safe action builder, no-drag alternative, grouped effects, catalog/rules/smoke wiring, a11y/no-leak smoke. |
| 16 | Benchmarks and CI receipts | 13–15 | Bench thresholds for 2/4/6, action fanout, view/export all viewers, replay, bots, CI command receipts. |
| 17 | Documentation closeout | 1–16 | `specs/README.md` status update, `docs/MECHANIC-ATLAS.md` first-use rows and §10B note, `docs/MECHANICAL-SCAFFOLDING-REGISTER.md` audit/update, `docs/SOURCES.md`, `apps/web/README.md`, `GAME-EVIDENCE.md`, public release checklist. |

---

## 6. Exit criteria

| ROADMAP / contract criterion | Gate 19 exit requirement | Evidence required |
|---|---|---|
| Draw/discard piles covered | Stock draw and discard-pile draw, including multi-card pickup, are implemented in Rust and covered by tests/traces. | Rule tests, property tests, `draw-source-choice`, `multi-card-discard-pickup`, invalid pickup trace, no-leak trace. |
| Public melds covered | Sets and runs can be created and are public after play. | Meld legality tests, tableau view tests, public trace, rule-coverage rows. |
| Laying off covered | Lay-off onto any existing meld is in scope and implemented, including opponent melds. | `layoff-onto-opponent-tableau` trace, score-credit assertion, public effect assertion. |
| Private hands covered | Seat hands are private across 2–6 seats; unseen stock order is hidden. | Pairwise no-leak matrix, public observer exports, all six seat-private exports, DOM/a11y/storage/log checks. |
| Scoring covered | Card values, meld positive scoring, laid-off score credit, in-hand penalties, negative scores, cumulative scores are Rust-owned. | Scoring unit tests, round scoring trace, terminal summaries, rule coverage. |
| Round/match flow covered | Deal, turn flow, go-out, stock-exhaustion round end, next-round setup, cumulative target and tie continuation are implemented. | Setup traces, go-out traces, stock-exhausted trace, multi-round 500 trace, tie-continues trace. |
| Terminal results covered | Rust emits per-seat score/rank breakdowns and unique winner after tie continuation. | Terminal trace, WASM export trace, outcome panel smoke, simulator summary. |
| Larger hand/tableau action affordances usable | Browser supports large private hand, public tableau, discard tail, and stock/discard zones without relying on drag-only interaction or TS legality. | UI smoke, a11y/no-leak checklist, keyboard-only path, grouped action builder screenshot/evidence, preview tests. |
| Meld/tableau primitive pressure recorded and resolved/deferred | Five first-use local-only primitive-pressure entries are recorded; no third-use gate fires; no helper promotion. | `games/meldfall_ledger/docs/PRIMITIVE-PRESSURE-LEDGER.md`, `docs/MECHANIC-ATLAS.md` updates, `GAME-EVIDENCE.md` receipt. |
| Forward-v1 scaffolding audit receipt | Gate 19 has its own `forward-v1` CI receipt and a documented queue-or-dispose prior-game decision. | `ci/scaffolding-audits.json` entry, `scripts/check-scaffolding-governance.mjs` pass, `GAME-EVIDENCE.md` entry. |
| Variable-N setup and summaries | Supported seat counts `{2,3,4,5,6}` validate; default 4; by-seat simulator summaries work up to 6. | Setup tests, invalid-count trace, `simulate` 2/4/6 receipts, by-seat summary JSON. |
| Official-game contract | Required docs, source notes, rules prose, coverage, traces, web exposure, acceptance checklist are complete. | Game docs, command suite, docs checks, public release checklist. |
| Foundation alignment | No stop condition triggered; no foundation amendment expected. | Boundary check, no engine-core nouns, no TS legality, no static behavior data, no helper promotion. |

---

## 7. Acceptance evidence

### 7.1 Command suite

Implementation closeout must record command receipts in `games/meldfall_ledger/docs/GAME-EVIDENCE.md`.

```bash
cargo fmt --all -- --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test -p meldfall_ledger
cargo test -p wasm-api
cargo test --workspace

cargo run -p simulate -- --game meldfall_ledger --seat-count 2 --games 1000 --start-seed 1900 --action-cap 4096
cargo run -p simulate -- --game meldfall_ledger --seat-count 4 --games 1000 --start-seed 1901 --action-cap 4096
cargo run -p simulate -- --game meldfall_ledger --seat-count 6 --games 1000 --start-seed 1902 --action-cap 8192

cargo run -p replay-check -- --game meldfall_ledger --all
cargo run -p fixture-check -- --game meldfall_ledger
cargo run -p rule-coverage -- --game meldfall_ledger
cargo bench -p meldfall_ledger

bash scripts/boundary-check.sh
node scripts/check-doc-links.mjs
node scripts/check-ci-games.mjs
node scripts/check-catalog-docs.mjs
node scripts/check-player-rules.mjs
node scripts/check-scaffolding-governance.mjs

npm --prefix apps/web test
npm --prefix apps/web run smoke:e2e   # the smoke:e2e chain must include node e2e/meldfall-ledger.smoke.mjs
```

If the existing npm smoke command uses a different selector grammar, the ticket may adapt the command but must preserve a `meldfall_ledger` smoke receipt that exercises setup, stock draw, discard-pile pickup, meld creation, lay-off, discard, replay import/export, and no hidden card text in public observer DOM/a11y output.

### 7.2 Test taxonomy

| Test class | Required Gate 19 coverage |
|---|---|
| Unit/rule tests | Seat-count diagnostics; deal counts; meld sets/runs; ace low/high/no-wrap; lay-off legality; discard-pickup commitment; going out; stock exhaustion; scoring; tie continuation. |
| Property tests | Deck uniqueness; card ownership conservation; no card simultaneously in hand/tableau/discard/stock; public card counts add to 52; legal action apply never panics; score deltas equal card-value accounting; redacted views never expose hidden cards. |
| Replay tests | Trace Schema v1 fields; deterministic replay; viewer-scoped export/import round trips; no blanket regeneration; migration labels only if authorized under ADR 0009. |
| Serialization/hash tests | Stable export order for public tableau, discard pile, hands in seat-private exports, score ledger, effect groups; no hash drift without trace note. |
| Visibility/no-leak tests | Public observer, every seat-private viewer at 6 seats, pairwise source-seat vs viewer-seat matrix, action-tree/previews/effects/diagnostics/bot explanations/DOM/a11y/storage/log surfaces. |
| Bot tests | L0 random-legal always legal and deterministic by seed. L1, if admitted, never uses opponents’ hidden hands or stock order. No L2 unless evidence pack accepted. |
| WASM/API tests | Catalog, setup, legal actions, apply action, preview, export/import, bot dispatch, terminal outcome, action path grouping. |
| Web smoke | Catalog entry, rules display, setup for 2/4/6, large hand/tableau rendering, keyboard-only action builder, no-drag alternative, effect animation, no-leak/a11y smoke. |
| Benchmarks | Native apply/action/view/export/bot/simulate budgets at 2/4/6, large discard-tail fixture, large tableau fixture. |
| Primitive-pressure tests | First-use ledger entries exist; no game-stdlib rummy helper; no engine-core card/meld/tableau/pile noun; no static data formulas. |
| Scaffolding governance tests | `ci/scaffolding-audits.json` has `meldfall_ledger` `forward-v1`; checker passes; prior-game retrofit decision recorded. |

### 7.3 Pairwise no-leak matrix

Gate 19 will exercise **every seat-private viewer at six seats** in CI rather than sampling. This exceeds the minimum export-coverage rule for 4+ seat games and prevents ambiguity around the max supported seat count.

Authorized public facts:

- active seat, dealer, turn/round/match phase;
- each seat’s hand count, cumulative score, current-round public played score, and terminal rank;
- public meld tableau cards and their score-credit owner;
- public discard pile ordered from oldest to newest/top, including all visible card identities;
- stock count only, never stock order or next stock card;
- diagnostics that do not name hidden opponent cards;
- public effect summaries that identify only public cards.

Hidden facts:

- all cards in any non-viewer hand;
- unseen stock order and next stock card;
- internal shuffle/deal tail;
- private action labels involving non-viewer hand cards;
- bot candidate rankings and explanations involving opponents’ hands or stock order;
- seat-private replay payloads for other seats.

| Source hidden fact | Public observer | Viewer same seat | Viewer different seat | CI requirement |
|---|---|---|---|---|
| `seat_0` hand | Redacted to count | Full for `seat_0` only | Redacted for `seat_1`…`seat_5` | Assert six-seat matrix. |
| `seat_1` hand | Redacted to count | Full for `seat_1` only | Redacted for `seat_0`,`seat_2`…`seat_5` | Assert six-seat matrix. |
| `seat_2` hand | Redacted to count | Full for `seat_2` only | Redacted for all other seats | Assert six-seat matrix. |
| `seat_3` hand | Redacted to count | Full for `seat_3` only | Redacted for all other seats | Assert six-seat matrix. |
| `seat_4` hand | Redacted to count | Full for `seat_4` only | Redacted for all other seats | Assert six-seat matrix. |
| `seat_5` hand | Redacted to count | Full for `seat_5` only | Redacted for all other seats | Assert six-seat matrix. |
| Stock order | Count only | Count only | Count only | Assert public + all six seat exports. |
| Stock draw card | Public: “drew from stock” + stock count change | Drawn card visible only to acting seat after draw | Hidden from non-acting seats | Trace and effect log assertions. |
| Discard draw cards | Public because discard pile is public before draw | Same as public plus resulting own hand contents | Same as public, not final private hand contents | Effect grouping assertions. |
| Melded/laid-off cards | Public after tabled | Public after tabled | Public after tabled | Not hidden; use as ordinary public state. |
| In-hand penalty identities at round end | Public totals/counts only | Own remaining cards if seat-private export includes terminal own hand | Opponent identities redacted | Round-score public/private export assertions. |

No-leak surfaces to assert: view JSON, action tree JSON, preview JSON, diagnostics, semantic effects, bot explanations, candidate rankings, replay export, fixture export, browser DOM text, accessibility labels, `data-testid` values, local/session storage, console logs captured by smoke, and simulator summaries.

### 7.4 Golden trace minimum set

Trace filenames are suggestions; tickets may rename while preserving coverage.

| Trace | Required proof |
|---|---|
| `setup-2p-13-card-deal.trace.json` | 2 seats receive 13 cards each; initial discard and stock count correct; public observer sees counts only. |
| `setup-4p-default.trace.json` | Default 4 seats, 7-card deal, dealer/start-seat order. |
| `setup-6p-max-seat.trace.json` | 6 seats, single deck, 7-card deal, stock/discard sizes, all seat keys emitted. |
| `invalid-seat-count-below.trace.json` | Seat count 1 rejected with setup diagnostic. |
| `invalid-seat-count-above.trace.json` | Seat count 7 rejected with setup diagnostic. |
| `deterministic-stock-draw-no-leak.trace.json` | Stock draw deterministic internally; only acting seat sees drawn card. |
| `draw-source-choice-stock-vs-discard.trace.json` | Rust legal action tree exposes stock draw and valid discard choices only. |
| `multi-card-discard-pickup-melds-deepest.trace.json` | Seat takes selected discard plus newer cards and immediately melds/uses selected card. |
| `invalid-discard-pickup-without-use.trace.json` | Deep discard selection rejected if selected card is not used immediately. |
| `top-discard-pickup-also-requires-use.trace.json` | Rulepath strict variant: top discard must also be used immediately. |
| `meld-set-valid-and-invalid.trace.json` | 3+ same-rank set valid; too-small or mixed-rank set invalid. |
| `meld-run-valid-ace-low-high-no-wrap.trace.json` | A-2-3 and Q-K-A supported; K-A-2 / Q-K-A-2 rejected. |
| `layoff-onto-own-tableau.trace.json` | Seat extends own public meld legally. |
| `layoff-onto-opponent-tableau-score-credit.trace.json` | Seat extends opponent meld; card is public; score credit goes to laying-off seat. |
| `invalid-layoff-gap-or-wrong-rank.trace.json` | Bad layoff rejected with viewer-safe diagnostic. |
| `discard-after-draw-turn-end.trace.json` | Normal draw/meld/discard turn end. |
| `go-out-by-final-discard.trace.json` | Seat ends round by discarding last card after table plays. |
| `go-out-without-final-discard.trace.json` | Seat ends round by melding/laying off every card; no final discard required. |
| `stock-exhausted-round-settlement.trace.json` | Stock exhaustion and no legal/accepted discard draw settle round. |
| `round-scoring-positive-negative.trace.json` | Meld/layoff positive values and in-hand penalties applied. |
| `scores-can-go-negative.trace.json` | A seat may finish a round below zero. |
| `multi-round-first-to-500.trace.json` | Cumulative scores reach/exceed 500; unique highest wins. |
| `target-tie-continues.trace.json` | Multiple seats tied for highest at/above 500; match continues. |
| `public-observer-no-leak-6p.trace.json` | Max-seat public export hides all hands and stock order. |
| `seat-private-export-round-trip-all-viewers.trace.json` | Every six-seat viewer export imports without privilege elevation. |
| `viewer-export-no-privilege-elevation.trace.json` | A public export cannot become a seat-private export on import. |
| `l0-random-legal-full-match.trace.json` | L0 bots complete a deterministic full match. |
| `l1-rule-informed-smoke.trace.json` | Required only if L1 is admitted; otherwise mark not applicable with reason. |
| `wasm-large-tableau-exported.trace.json` | WASM-exported trace covers large public tableau + discard tail + terminal summary. |

### 7.5 Fixture and completion profile

Minimum fixtures:

| Fixture | Purpose |
|---|---|
| `meldfall_ledger_2p_standard.fixture.json` | 2-player 13-card deal, short deterministic round. |
| `meldfall_ledger_4p_standard.fixture.json` | Default 4-player setup and normal round/match segment. |
| `meldfall_ledger_6p_standard.fixture.json` | Max-seat export/no-leak and simulator setup. |
| `meldfall_ledger_multi_discard_pickup.fixture.json` | Deep discard pickup and immediate-use commitment. |
| `meldfall_ledger_layoff_any_tableau.fixture.json` | Opponent-tableau lay-off with score credit. |
| `meldfall_ledger_500_tie_continues.fixture.json` | Multi-round target and tie continuation. |

`GAME-EVIDENCE.md` must list fixture profile, completion profile, command receipts, benchmark profile, export coverage, hash/serialization receipts, and any authorized trace migration notes. Blanket golden regeneration is prohibited.

### 7.6 Benchmark expectations

Provisional budgets are intentionally conservative until first implementation data exists. The ticket may calibrate thresholds under the benchmark ADR process, but it must not remove the profiles.

| Benchmark profile | Required operation | Provisional target |
|---|---|---:|
| `native_2p_short_round` | Setup + 200 random-legal actions | p95 under 2 ms/action on CI reference lane after calibration. |
| `native_4p_default` | Setup + 500 random-legal actions | p95 under 3 ms/action. |
| `native_6p_large_surface` | Legal action tree + apply + view projection for all viewers | p95 under 8 ms/action. |
| `large_discard_tail` | Generate legal discard-pickup choices for a long visible discard pile | p95 under 5 ms/action-tree generation. |
| `large_public_tableau` | View export public + all six seat-private viewers | p95 under 12 ms total. |
| `replay_export_import` | Export/import public + all six seat-private views | p95 under 20 ms per fixture. |
| `l0_bot_decision` | Random legal selection from grouped tree | p95 under 1 ms. |
| `l1_bot_decision` | If admitted, feature scoring over legal groups | p95 under 10 ms. |

---

## 8. FOUNDATIONS & boundary alignment

### 8.1 Authority and boundary commitments

| Foundation / area law | Gate 19 alignment |
|---|---|
| Rust owns behavior | Every legal draw, meld, lay-off, discard, go-out, score, terminal, preview, export, and bot decision is Rust-owned. |
| TypeScript presents only | Browser reads Rust legal action trees and Rust previews/effects. It may group or lay out choices but cannot validate or invent them. |
| Engine-core generic | `engine-core` receives no card/deck/hand/suit/rank/meld/set/run/sequence/stock/discard/pile/tableau nouns. |
| Game-stdlib promotion law | First-use rummy shapes stay local. No `game-stdlib` rummy helper is created. Existing `seat` helpers may be reused; `trick_taking` is not. |
| Data boundary | `data/variants.toml` can carry typed identifiers and constants such as target score and display text. It cannot encode meld conditions, lay-off legality, discard-pickup rules, or scoring formulas. |
| Official game contract | Requirements-first, source notes, original rules prose, rule-coverage matrix, UI exposure, traces, acceptance checklist, and release checklist are mandatory. |
| Multi-seat contract | Variable 2–6 declaration, roles/teams absent declaration, viewer matrix, pairwise no-leak, public observer, surface budgets, effect grouping, per-seat outcome breakdowns, and seat-keyed simulator summaries are mandatory. |
| AI bots | L0 required. L1/L2 constrained by hidden-info boundaries. No MCTS/ISMCTS/Monte Carlo/ML/RL. L2 deferred without strategy evidence. |
| UI interaction | Legal-only affordances, Rust previews, effect-driven animation, replay UI, keyboard and accessibility proof for large zones. |
| Testing/replay | Trace Schema v1, deterministic replay, no-leak taxonomy, export coverage, fixture profiles, benchmark receipts, no blanket golden regeneration. |
| ADR 0004 | Viewer-scoped hidden-info replay exports; public observer and seat-private exports never elevate privileges. |
| ADR 0008 | Forward-v1 reuse-first scaffolding audit, register-new if needed, queue-or-dispose prior-game refactors, CI receipt. |
| ADR 0009 | Replay/fixture/hash taxonomy v2; bounded authorized exports only. |
| IP policy | Neutral name, original prose/assets, source notes, public-domain/common-name evidence, IP release checklist. |
| Agent discipline | Bounded tasks, no forbidden changes, failing-test protocol, no weakening tests. |

### 8.2 First-use primitive-pressure posture

| Shape | Gate 19 decision | Rationale | Next review trigger |
|---|---|---|---|
| Meld validation: sets + runs | New first official use, `local-only` in `games/meldfall_ledger`. | No manifest-verified existing official game has meld validation; mechanic atlas row says start local. | Before a third close meld/tableau/zone helper appears. |
| Public meld tableau / zone model | New first official use, `local-only`. | Public tableau is rummy-specific state and presentation pressure, not generic engine law. | Third close tableau/zone use. |
| Draw/discard piles including multi-card pickup | New first official use, `local-only`. | Existing card games have decks/hands/community cards, but not Rummy 500 discard-tail pickup with immediate-use commitment. | Third close draw/discard zone use. |
| Laying off onto any player’s meld | New first official use, `local-only`. | Behavior depends on meld-group legality and score-credit attribution; not scaffolding. | Third close lay-off/tableau-extension use. |
| Multi-round cumulative scoring to 500 | New first official use for this rummy scoring pattern, `local-only`. | Similar targets exist, but this combines positive tabled cards, in-hand penalties, laid-off credit, negative scores, and tie continuation. | Third close rummy-style cumulative scoring target. |
| Deterministic shuffle + private hand + redacted export row (§10B) | Reviewed; no new hard gate. | Gate 19 repeats deterministic shuffle/private hands but has no staged hidden reveal. Shuffle/deal remains game-local; no shared shuffle helper. | Reopen if the deferred-row trigger criteria are met by future games. |

`docs/MECHANIC-ATLAS.md` §10A remains empty because no helper promotion is earned and no open promotion debt is created. If implementation discovers a prior close use that makes any row a second use, the ledger may record second-use pressure; it still must not promote at Gate 19 without the hard-gate process.

### 8.3 No-reuse of trick-taking helpers

`crates/game-stdlib/src/trick_taking.rs` is excluded. Meldfall Ledger has no lead suit, follow-suit, trick winner, trump, bid, nil, bag, contract, or partnership team. Any import of `game_stdlib::trick_taking` from `games/meldfall_ledger` is a failing boundary condition.

Permitted shared reuse:

- generic `engine-core` game/action/replay/RNG contracts;
- `game-stdlib::seat` helpers for seat count/ring/labels if behavior-free and already accepted;
- existing generic action-tree/effect/replay/export scaffolding where lawful;
- game-test-support no-leak/profile utilities in tests only.

### 8.4 Foundation stop conditions

Implementation must stop before merge if any of these occur:

- `engine-core` gets rummy/card/meld/tableau/pile nouns;
- TypeScript validates legality or filters actions based on card/meld rules;
- static data starts encoding rule formulas or behavior selectors;
- public/seat-private export leaks hidden hands or stock order;
- bot policy uses opponents’ hidden cards or stock order;
- MCTS/ISMCTS/Monte Carlo/ML/RL is proposed for public v1/v2;
- rummy helpers are promoted without mechanic-atlas hard-gate authorization;
- `ci/scaffolding-audits.json` lacks the Gate 19 `forward-v1` receipt;
- a foundation amendment is needed but not handled through authority-order rules.

---

## 9. Forbidden changes

| Forbidden change | Reason |
|---|---|
| Add `Card`, `Deck`, `Hand`, `Suit`, `Rank`, `Meld`, `Set`, `Run`, `Sequence`, `Stock`, `Discard`, `Pile`, or `Tableau` nouns to `engine-core`. | Violates the engine-game-data boundary. |
| Add a rummy/meld/tableau/pile helper to `game-stdlib` in Gate 19. | First official use; atlas says start local and hard-gate before a third helper. |
| Reuse `game-stdlib::trick_taking`. | No tricks exist in Five Hundred Rummy. |
| Encode meld legality, lay-off legality, discard-pickup rules, scoring formulas, or go-out rules in TOML/YAML/JSON/static data. | Static data cannot own behavior. YAML/DSL is forbidden without accepted ADR. |
| Add teams, partnerships, team scores, or Blackglass-style partnership machinery. | Five Hundred Rummy is individual competitive; teams absent. |
| Let TypeScript determine legal melds, legal discards, legal pickups, score totals, terminal state, or bot choices. | Browser presentation only. |
| Display hidden opponent hand identities or stock order in views, actions, previews, effects, DOM, a11y text, logs, storage, exports, fixtures, simulator summaries, bot explanations, or candidate rankings. | Hidden-information no-leak law. |
| Treat public meld tableau/discard pile as hidden. | They are public game surfaces after cards are tabled/discarded. |
| Reveal unmelded opponent hand identities at round settlement in public exports unless a future accepted rule note explicitly makes them public. | Gate 19 public settlement exposes totals/counts only to minimize hidden-info leaks. |
| Implement jokers, two-deck shoe, Call Rummy, opening minimum, frozen pile, around-the-corner runs, floating, or discard-required going-out. | Out-of-scope variants. |
| Delete/ignore/weaken tests to pass. | Violates agent discipline. |
| Skip the `forward-v1` reuse-first audit or omit prior-game disposition. | Violates ADR 0008 forward governance. |
| Mark Gate 19 `Done` before docs, traces, evidence, web surface, no-leak, benchmarks, and CI receipts are complete. | Violates official-game and roadmap exit criteria. |

---

## 10. Documentation updates required

| Path | Required update |
|---|---|
| `specs/README.md` | Add/save this spec path as Gate 19 planned spec. At implementation closeout, flip Gate 19 to `Done` only after evidence passes; do not run `/spec-to-tickets` before `/reassess-spec`. |
| `docs/MECHANIC-ATLAS.md` | Add first-use local-only entries for meld validation, public meld tableau/zone model, draw/discard piles with multi-card pickup, laying off onto any tableau, and multi-round cumulative scoring. Add §10B note: Gate 19 is another deterministic-shuffle/private-hand game with redacted exports but no staged hidden reveal; no trigger fires. Keep §10A empty unless a promotion is legitimately earned, which this spec does not expect. |
| `docs/MECHANICAL-SCAFFOLDING-REGISTER.md` | Record the Gate 19 `forward-v1` reuse-first audit. If no new behavior-free scaffolding is invented, document `no-new-scaffolding` and no prior-game retrofit. If a pure scaffolding shape is invented, register it as candidate/local-only/rejected with rationale, owner, evidence, and next review. |
| `ci/scaffolding-audits.json` | Add `meldfall_ledger` `coverage: "forward-v1"` entry; ensure `scripts/check-scaffolding-governance.mjs` passes. |
| `docs/SOURCES.md` | Add source summary for Five Hundred Rummy/Rummy 500, house variants excluded, neutral-name note, and prior-art implementation/strategy/UX sources. |
| `games/meldfall_ledger/docs/*` | Fill all official game documents listed in §4.2 with explicit N/A rows where needed. |
| `apps/web/README.md` | Update intro catalog list, Shell Surface renderer list, and Smoke Layers `smoke:e2e` list for Meldfall Ledger. |
| `games/meldfall_ledger/docs/HOW-TO-PLAY.md` → generated `apps/web/public/rules/meldfall_ledger.md` + `apps/web/public/rules/manifest.json` | Author the source `HOW-TO-PLAY.md`; regenerate the web rules asset and manifest via `node scripts/copy-player-rules.mjs` (parity guarded by `scripts/check-player-rules.mjs`). Never hand-edit the generated files. |
| `ci/games.json` | Add `meldfall_ledger` so game-list checks and scaffolding governance align. |
| `Cargo.toml` | Add the new crate member. |
| `crates/wasm-api` docs/tests | Update API snapshot/catalog docs if required by existing tests. |
| Public release checklist | Complete IP/source/assets/no-leak/browser/replay/benchmark evidence before public release. |

### 10.1 Foundation-amendment posture

No foundation amendment is expected. Gate 19 is a user of existing foundation docs, ADR 0004, ADR 0007, ADR 0008, ADR 0009, multi-seat law, evidence fixture law, trace law, and UI law. If the larger hand/tableau surface exposes a genuine foundation gap, implementation must flag it explicitly, stop the affected work, and use the repository’s ADR/foundation update process. Silent redefinition is forbidden.

---

## 11. Sequencing

| Sequence fact | Gate 19 stance |
|---|---|
| Predecessor | Gate 18 — Blackglass Pact / Spades — is `Done` as of 2026-06-25 and was the first `forward-v1` audit user. |
| Promotion-debt interlock | Mechanic-atlas §10A is empty; no debt-closure spec blocks Gate 19. |
| Current gate | Gate 19 — Meldfall Ledger / Five Hundred Rummy — is the next active implementation spec. |
| Audit sequence | Gate 19 is the second `forward-v1` audit user and must add its own CI receipt. |
| Successor | Gate 20 — Star Halma / Chinese Checkers — remains successor. Do not start it until Gate 19 exits or is explicitly paused by roadmap authority. |
| Admission rule | Implementation admission requires this spec, the forward-v1 audit, variant source notes, first-use primitive-pressure decisions, and official-game doc skeletons before behavior tickets proceed. |
| Ticket flow | After saving this spec to `specs/`, run `/reassess-spec`; only then run `/spec-to-tickets`. |

---

## 12. Assumptions

- `assumption:` No foundation amendment is expected; documentation updates only.
- `assumption:` This deliverable is the spec only, not `tickets/` AGENT-TASK packets.
- `assumption:` The neutral public name **Meldfall Ledger** is acceptable. If maintainers reject it, rename public display, module id, constants, spec filename, and docs consistently before ticket decomposition.
- `assumption:` One standard 52-card deck is used for all supported 2–6 seat counts even though some external rules sources recommend or permit two decks for larger groups.
- `assumption:` Aces may be low or high in runs, but never wrap around; aces always score 15 points in this Rulepath variant.
- `assumption:` A final discard is not required to go out; a seat may go out by melding/laying off its entire hand.
- `assumption:` Top-discard pickup follows the same immediate-use rule as deeper discard pickup in this variant.
- `assumption:` Public round-settlement output lists in-hand penalty totals/counts, not opponents’ exact remaining card identities, unless a future accepted source note explicitly changes that visibility decision.
- `assumption:` CI can exercise all six seat-private viewers rather than a sampled matrix. If CI cost proves excessive, the spec must be amended with an explicit sampled matrix and rationale before closeout.

---

# Appendix A — Research-pinned rules and source notes

External research is source material for the rules-family/variant decision only. All Rulepath prose must be original.

## A.1 External sources consulted

| Source | Use in this spec |
|---|---|
| Pagat, “500 Rum” — `https://www.pagat.com/rummy/500rum.html` | Primary rules-family reference for players/cards, deal, melds, lay-off, discard-pile pickup, scoring, stock exhaustion, and match target. |
| Bicycle Cards, “500 Rum” — `https://bicyclecards.com/how-to-play/500-rum` | Secondary public rules reference for 52-card deck, target score, deal counts, meld/run definitions, and highest-score-at-500 winner wording. |
| Pagat, “Scoring Rummies” — `https://www.pagat.com/rummy/Scoring_Rummies.html` | Family context: scoring rummies, 500 Rum deal summary, two-deck convention for larger groups, strategy notes. |
| Rummy Rulebook, “Rummy 500” — `https://www.rummyrulebook.com/pages/rummy-500/` | Supplemental rule/variant reference for draw/discard/meld/layoff/go-out/scoring variants. |
| timpalpant/rummy — `https://github.com/timpalpant/rummy` | External implementation prior art only. It models a server state machine with stock/discard actions and a “must play picked discard” invariant. Not target-repository evidence. |
| RLCard Gin Rummy docs — `https://rlcard.org/rlcard.games.gin_rummy.html` | External implementation taxonomy/prior art for melding modules; also a negative boundary reminder that RL/ML tooling is not admissible for Rulepath public v1/v2 bots. |
| WAI-ARIA Authoring Practices, Grid Pattern — `https://www.w3.org/WAI/ARIA/apg/patterns/grid/` | UX/accessibility reference for keyboard-operable dense card/tableau grids. |
| WCAG 2.2 Success Criterion 2.5.7 Dragging Movements — `https://www.w3.org/WAI/WCAG22/Understanding/dragging-movements.html` | Requires single-pointer alternatives for drag operations; supports click/select action builder instead of drag-only card movement. |

## A.2 Variant decision matrix

| Parameter | Rulepath pinned decision | Source support | Deliberate deviation / note | Required tests |
|---|---|---|---|---|
| Deck | One standard 52-card deck, no jokers, all supported 2–6 seat counts. | Pagat and Bicycle describe standard 52-card play; Pagat variants mention jokers/two decks. | Two-deck/joker variants recorded but excluded. For 5–6 seats this intentionally preserves the prompt’s single-deck requirement. | Setup 2/4/6 deck-size and conservation tests. |
| Deal | 2 seats: 13 cards each. 3–6 seats: 7 cards each. One face-up initial discard, rest stock. | Bicycle: 13 each for two players; 7 each for three/four. Pagat scoring-rummies summary gives 13 for 2 and 7 for 3+. | Use 7 for 5–6 despite two-deck convention in some sources. | Setup traces for 2/4/6. |
| Turn order | Left of dealer starts; clockwise. | Pagat states player left of dealer begins and turn passes clockwise. | None. | Setup/order trace. |
| Draw options | Draw one stock card or take from discard pile. | Pagat and Rummy Rulebook. | None. | Draw-source trace. |
| Discard-pile draw | Seat may select any visible discard and take it plus all newer cards above it. Selected deepest card must be used immediately in a meld or lay-off in that turn. | Pagat and Bicycle both describe taking a discard and all cards above/after it, with the selected card immediately melded. | Rulepath applies immediate-use to top discard too, following Pagat’s stricter “most books” note. No frozen pile. | Multi-card pickup, top discard use, invalid pickup traces. |
| Meld kinds | Sets and runs. Sets are 3–4 same-rank cards; runs are 3+ consecutive same-suit cards. | Pagat/Bicycle/Rummy Rulebook. | No duplicate-rank multi-deck sets because single deck. No remelding/rearranging tabled groups. | Meld set/run traces. |
| Ace in runs | Ace can be low (`A-2-3`) or high (`Q-K-A`) but cannot wrap (`K-A-2`, `Q-K-A-2`). | Bicycle says ace may be high or low but not around the corner; Pagat lists ace high/low variants. | Pin one explicit rule to avoid ambiguity. | Ace run trace. |
| Card values | Ace = 15; K/Q/J/10 = 10; 2–9 = pip value. | Pagat primary values: picture cards 10, aces 15, number cards face value; Bicycle/Rummy Rulebook agree for common scoring. | Low ace still scores 15 in this Rulepath variant; no low-ace one-point/five-point house rule. | Scoring unit and trace. |
| Melding timing | After draw, seat may make any number of legal melds/lay-offs before discarding or going out. Melding is optional except to satisfy a discard-pickup commitment or empty hand. | Pagat/Rummy Rulebook describe melding/laying off during turn. | No opening minimum. | Turn lifecycle tests. |
| Lay-off | Seat may extend any existing public meld, including opponents’ melds, if resulting meld remains legal. Score credit goes to the seat that plays the card. | Pagat says players may add to combinations on table, including examples credited to the player who adds. | Preserve table ownership separately from score-credit ownership. | Opponent layoff trace. |
| Discard after pickup | A picked discard that is under immediate-use commitment cannot be discarded instead; if it is the top discard, it must be used. Other picked-up cards become ordinary hand cards after commitment is satisfied. | Pagat’s stricter book rule; external prior-art implementation tracks a “must play” card after discard pickup. | Rulepath validates this in Rust; no TS filters. | Invalid pickup/discard trace. |
| Going out | Round ends when a player has no cards after melding/laying off all cards or after discarding last card. Final discard not required. | Pagat and Rummy Rulebook allow going out by discarding or melding all remaining cards. | No floating or discard-required variant. | Two go-out traces. |
| Stock exhaustion | If stock is empty and no legal/accepted discard draw continues play, round ends and scores. | Pagat/Rummy Rulebook describe stock exhaustion end condition. | No reshuffle discard pile into stock in Gate 19. | Stock-exhaustion trace. |
| Round scoring | Public tabled cards score positive to playing seat; cards left in hand score negative. | Pagat and Bicycle. | Public output lists penalty totals/counts, not opponent card identities. | Round-score trace/no-leak. |
| Match target | First to reach/exceed 500 after a round is eligible; highest at/above 500 wins. If tied, continue. | Pagat: if more than one reaches 500, highest wins; further hands if tied. Bicycle: game ends when one player reaches 500 and highest score wins. | Gate requires exactly one winner, so equal-high ties continue. | 500 and tie-continuation traces. |

## A.3 Bot-policy research notes

The strategy evidence should emphasize low-risk, explainable rummy heuristics:

- Early tabled high cards reduce penalty exposure but may give opponents lay-off opportunities.
- Deep discard-pile pickup can score immediately but inflates hand size and penalty risk.
- Retaining flexible connectors near existing runs can be valuable; isolated high cards are dangerous when opponents are near going out.
- Opponent proximity may be inferred only from public hand counts, public tabled melds, public discard behavior, and public scores. Opponent hand identities and stock order are forbidden inputs.

L1 may use deterministic authored scoring over legal Rust actions. L2 requires `COMPETENT-PLAYER.md` and `BOT-STRATEGY-EVIDENCE-PACK.md` acceptance. Public v1/v2 must not use MCTS, ISMCTS, Monte Carlo rollouts, machine learning, reinforcement learning, or hidden-state search over opponent hands/stock.

## A.4 UX/accessibility research notes

The web surface must assume a dense but keyboard-operable card UI:

- A large private hand and public tableau should use roving-focus or equivalent keyboard navigation over grouped card grids.
- Any drag/drop card movement must have a single-pointer click/select alternative and a keyboard alternative.
- The preferred interaction is a Rust-owned progressive action builder: choose draw source, choose selected discard if applicable, choose meld/lay-off cards/target, and choose discard. Each step displays only Rust-provided legal continuations or Rust-safe previews.
- Public tableau groups need stable labels and card counts for screen readers: “Seat 3 run, hearts 7 through 10, score credit: Seat 3/Seat 5…” rather than raw hidden identifiers.
- Public observer DOM/a11y output must never include opponent private hand card text, stock order, or hidden action labels.

---

# Appendix B — Implementation model notes

## B.1 Local state model

Suggested local Rust shapes, to be refined by tickets:

```rust
pub struct MatchState {
    pub seats: Vec<SeatState>,
    pub cumulative_scores: Vec<i32>,
    pub dealer: SeatIndex,
    pub round: RoundState,
    pub terminal: Option<MatchOutcome>,
}

pub struct RoundState {
    pub active_seat: SeatIndex,
    pub phase: TurnPhase,
    pub stock: Vec<Card>,          // internal only, top at end or explicit top index
    pub discard: Vec<Card>,        // public, ordered oldest -> newest/top
    pub tableau: MeldTableau,      // public cards only
    pub pending_pickup: Option<DiscardPickupCommitment>,
    pub round_played_scores: Vec<i32>,
}

pub struct SeatState {
    pub hand: Vec<Card>,           // visible only to owning seat/internal
}

pub struct MeldGroup {
    pub id: MeldId,
    pub kind: MeldKind,
    pub origin_seat: SeatIndex,
    pub cards: Vec<TableCard>,
}

pub struct TableCard {
    pub card: Card,
    pub played_by: SeatIndex,      // score-credit owner
    pub play_turn: TurnOrdinal,
}
```

Do not copy this verbatim if the crate’s existing patterns prefer different names; preserve the ownership semantics and boundary stance.

## B.2 Action model

Rust may expose either coarse complete commands or a progressive action path tree. For browser usability, a progressive tree is strongly preferred:

1. `DrawFromStock` or `DrawFromDiscard { selected_discard_index }`.
2. If discard pickup selected, mark `pending_pickup.required_card` internally.
3. Zero or more `MeldNew { cards }` and `LayOff { card, target_meld_id, position }` continuations, each validated by Rust.
4. `Discard { card }` if the seat still has cards and no pending commitment remains.
5. `GoOutWithoutDiscard` only if hand is empty after melds/layoffs.

The action tree may group choices by action family and card id. It must not flatten every possible meld partition if that creates unusable fanout; instead, expose Rust-owned continuation builders and validate final submitted commands. Browser grouping does not become legality.

## B.3 Score-credit model

Each tabled card has a `played_by` score-credit owner. A meld’s `origin_seat` identifies who opened the group, but later lay-offs score to the laying-off seat. Round score for each seat is:

```text
sum(value(card) for public table cards where played_by == seat)
- sum(value(card) for private hand cards still held by seat at round end)
```

Then add round score to cumulative score. Scores may be negative. Terminal result checks cumulative scores only after round settlement.

## B.4 Public settlement visibility

Public round settlement may expose:

- positive tabled total by seat;
- in-hand penalty total by seat;
- remaining hand count by seat;
- round delta and cumulative score by seat;
- terminal rank/winner if any.

Public settlement must not expose opponents’ exact unmelded cards unless a future accepted rules note explicitly makes them public and the no-leak matrix is updated. Seat-private settlement may include the viewer’s own remaining cards.

---

# Appendix C — Forward-v1 reuse-first scaffolding audit matrix

Gate 19 must run this matrix before implementation admission and copy its outcome into game-local evidence.

| Register entry | Gate 19 review result | Required action |
|---|---|---|
| MSC-8C-001 — effect-envelope constructors | Reuse. Meldfall needs grouped public and seat-private effects; use accepted effect envelope shape. | Evidence in `effects.rs`, `GAME-EVIDENCE.md`, CI receipt known signal. |
| MSC-8C-002 — local seat grammar / aliases | Reuse. Variable 2–6 uses stable `seat_0`…`seat_5`. | Evidence in setup/view/simulator summaries and CI receipt known signal. |
| MSC-8C-003 | Review required. No new pure scaffolding expected. | Record `not applicable` if unused; register-new if a behavior-free shape is invented. |
| MSC-8C-004 — local action-tree v1 framing/hash | Reuse. Progressive action builder should use accepted action-tree framing and hash behavior. | Evidence in action tree tests and CI receipt known signal. |
| MSC-8C-005 — local stable-byte writer | Review. Use existing stable writers only if the action/replay/export APIs expose them; otherwise not present. | CI receipt known signal `not-present` unless reused. |
| MSC-8C-006 — production support edge | Review. Game-test support utilities remain test-only; no production support edge expected. | CI receipt known signal `not-present`. |
| MSC-8C-007 | Review required. No new pure scaffolding expected. | Record `not applicable` if unused. |
| MSC-8C-008 — evidence fixture/export profiles | Reuse. Gate 19 instantiates the consolidated evidence/fixture profiles. | Evidence in `GAME-EVIDENCE.md` and fixture-check receipts. |
| MSC-8C-009 — bounded index / RNG sampling shape | Reuse only the accepted RNG primitive where lawful. Shuffle/deal algorithm stays local; no shared shuffle helper. | Evidence in setup determinism tests; no register promotion. |
| MSC-8C-010 — behavior/policy bundle rejected/local-only | Apply. Meld validation, lay-off, discard-pickup legality, scoring, bot policy, and visibility remain game-owned behavior. | Prior-game refactor disposition: no follow-on unit unless a pure scaffolding match is found. |

Prior-game retrofit disposition expected at spec time: **dispose / no follow-on unit**, because Gate 19 exposes no new behavior-free scaffolding that earlier games must migrate to. River Ledger, Vow Tide, and Blackglass Pact are exemplars but do not share rummy meld/tableau behavior. If implementation invents behavior-free selected-card-zone metadata, the developer must either register it and queue a bounded follow-on unit, or record accepted local-only/deferred/rejected disposition with owner, evidence, and next review.

---

# Appendix D — Primitive-pressure ledger entries to create

`games/meldfall_ledger/docs/PRIMITIVE-PRESSURE-LEDGER.md` must include at least these entries.

| Entry id | Shape | Status | Evidence | Atlas update |
|---|---|---|---|---|
| `ML-PP-001` | Meld validation: sets + runs, ace low/high/no-wrap. | First official use, local-only. | `rules.rs`, meld traces, rule coverage. | Add local-only row / note to `docs/MECHANIC-ATLAS.md`. |
| `ML-PP-002` | Public meld tableau / tabled-card score-credit model. | First official use, local-only. | `state.rs`, `visibility.rs`, public tableau traces. | Add local-only row / next review trigger. |
| `ML-PP-003` | Draw/discard zones with public discard-tail pickup and stock hidden order. | First official use, local-only. | `actions.rs`, `rules.rs`, no-leak traces. | Add local-only row / next review trigger. |
| `ML-PP-004` | Laying off onto any player’s meld. | First official use, local-only. | Lay-off traces, score-credit tests. | Add local-only row / next review trigger. |
| `ML-PP-005` | Multi-round cumulative scoring to 500 with in-hand penalties and tie continuation. | First official use for rummy-style target model, local-only. | `scoring.rs`, multi-round traces. | Add local-only row / next review trigger. |
| `ML-PP-006` | Deterministic shuffle + private hand + redacted export comparison. | Reviewed against §10B; no new hard gate. | Setup/no-leak/export traces. | Add §10B note only. |

No §10A open-promotion-debt entry is expected.

---

# Appendix E — Web and WASM specifics

## E.1 Rust/WASM contract

WASM must expose:

- catalog metadata: id, display name, seat range, default seats, bot levels, public observer support, rules path;
- setup with seat-count validation and deterministic seed;
- viewer-scoped view export for public observer or `seat_0`…`seat_5`;
- legal action tree for the active viewer, redacted according to viewer;
- Rust-safe preview for partial/proposed actions;
- apply action with semantic effect groups;
- bot action for L0 and optional L1;
- replay export/import under viewer scope.

WASM must not expose internal stock order, all hands, or privileged debug state through dev-panel operations except in test-only internal harnesses that are never browser-public.

## E.2 Web renderer minimum

`MeldfallLedgerBoard.tsx` should include:

- private hand zone for the current seat-private viewer, or public hand-count badges for public/opponent views;
- stock zone showing only count and legal draw affordance if active viewer can draw;
- discard zone showing ordered public cards, including legal pickup affordances supplied by Rust;
- public meld tableau grouped by meld, origin seat, run/set label, and score-credit cards;
- score ledger panel with round played totals, in-hand penalty totals, cumulative scores, target progress, and terminal ranks;
- progressive action builder with keyboard navigation and click/select alternatives; drag/drop optional only with non-drag alternative;
- effect feedback groups for stock draw, discard-tail pickup, meld, lay-off, discard, round scoring, and match terminal;
- replay/import/export controls compatible with existing shell;
- no hidden card text in public observer DOM, a11y labels, or test ids.

---

# Appendix F — Source-code seams inspected for pattern transfer

Source-code seams referenced for pattern transfer, validated against the current `main` tree. Pattern transfer summary:

| Source seam | How Gate 19 uses it |
|---|---|
| `games/river_ledger/src/setup.rs` | Variable/N-seat setup diagnostics, deterministic shuffle/deal pattern, private hand distribution. |
| `games/river_ledger/src/visibility.rs` | Per-seat hidden-information filtering and public/seat-private export pattern. |
| `games/river_ledger/src/state.rs` / scoring/outcome docs | Rust-owned terminal outcome projection and seat-keyed summaries. |
| `games/vow_tide/src/setup.rs` | Variable seat-count declaration and setup diagnostics for a non-fixed seat set. Trick-taking details do not transfer. |
| `games/vow_tide/tests/golden_traces` patterns | Setup traces by seat count and all-viewer export coverage pattern. |
| `games/blackglass_pact` crate | New-game crate anatomy, docs completion style, recent WASM/web registration pattern. Partnership/team machinery does not transfer. |
| `games/blackglass_pact/docs/GAME-EVIDENCE.md` and primitive ledger | Forward-v1 audit closeout and evidence receipt anatomy. |
| `crates/game-stdlib/src/seat.rs` | Accepted seat-count/ring helper use is allowed. |
| `crates/game-stdlib/src/trick_taking.rs` | Confirmed not reused; no rummy/meld/tableau/draw-discard helper exists. |
| `ci/scaffolding-audits.json` and checker | Receipt schema and `forward-v1` enforcement pattern. |
| `apps/web` catalog and board components | Renderer/catalog/smoke registration shape; legality remains Rust/WASM. |


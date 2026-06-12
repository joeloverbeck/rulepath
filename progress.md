Current status: Gates 0-13 are complete in the worktree. Gate 10 is complete
with `poker_lite` / Crest Ledger as the accepted betting/showdown half and
`plain_tricks` / Plain Tricks as the accepted trick/follow-suit half. Gate 11 is
complete with `masked_claims` / Masked Claims as the accepted claim/challenge
reaction-window proof. Gate 12 is complete with `flood_watch` / Flood Watch as
the accepted cooperative event-pressure proof. Gate 13 is complete with
`frontier_control` / Frontier Control as the accepted asymmetric graph-map
area-control proof.
`blackjack_lite` is deferred by ADR 0006 and is not a blocker for the current
roadmap ladder. The mutable source of truth for gate progress is
`specs/README.md`.

## Gate 13 Frontier Control

- Completed on 2026-06-11 for `frontier_control` / Frontier Control, the
  asymmetric graph-map area-control proof for ROADMAP Gate 13.
- Added the accepted perfect-information asymmetric proof: typed site/edge graph
  maps, adjacency-constrained movement, deterministic clash resolution, site
  control, faction-disjoint action sets, faction-specific scoring formulas,
  comparable final score track with Garrison tiebreak, per-faction Level 0 and
  Level 1 bots, public semantic effects, golden traces, native tools,
  benchmarks, WASM/browser board, generated player rules, outcome
  explanations, catalog reconciliation, and no-leak browser smoke.
- Boundary notes: graph, site, edge, faction, unit, guard, crew, stake, fort,
  clash, supply, control, and scoring vocabulary stayed game-local. No
  `engine-core` noun, `game-stdlib` helper, or promotion debt was introduced;
  `docs/MECHANIC-ATLAS.md` §10A remains `_None_`. The multi-action turn-budget
  third-use hard gate is armed for Gate 14, and graph/control/asymmetry rows are
  local-only first uses pending future pressure.
- Acceptance evidence:
  - `cargo fmt --all --check`
  - `cargo clippy --workspace --all-targets -- -D warnings`
  - `cargo test --workspace`
  - `cargo run -p simulate -- --game frontier_control --games 1000`
  - `cargo run -p replay-check -- --game frontier_control --all`
  - `cargo run -p fixture-check -- --game frontier_control`
  - `cargo run -p rule-coverage -- --game frontier_control`
  - `cargo bench -p frontier_control`
  - `bash scripts/boundary-check.sh`
  - `npm --prefix apps/web run smoke:wasm`
  - `npm --prefix apps/web run smoke:ui`
  - `npm --prefix apps/web run smoke:effects`
  - `npm --prefix apps/web run smoke:e2e`
  - `node scripts/check-catalog-docs.mjs`
  - `node scripts/check-player-rules.mjs`
  - `node scripts/check-outcome-explanations.mjs`
  - `node scripts/check-doc-links.mjs`
- Release constraint: the registered Level 1 simulation currently reports a
  Garrison-dominant 1000-0 result on the standard map; docs record this as
  balance retune debt before any stronger public balance claim.

## Gate 12 Flood Watch

- Completed on 2026-06-11 for `flood_watch` / Flood Watch, the cooperative
  event-pressure proof for ROADMAP Gate 12.
- Added the accepted shared-outcome cooperative game proof: deterministic hidden
  event-deck setup, public forecast/draw reveal, Rust-owned environment
  automation, role-modified bail/reinforce powers, multi-action turn budgets,
  standard and Deluge scenario variants, shared win/loss terminal rationale,
  viewer-scoped public export/import, Level 0 and Level 1 cooperative bots,
  golden traces, benchmarks, native tool/CI registration, WASM/browser board,
  generated player rules, outcome explanations, catalog reconciliation, and
  no-leak browser smoke.
- Boundary notes: district, flood, levee, event, role, scenario, action-budget,
  environment, and shared-outcome vocabulary stayed game-local. Flood Watch was
  reviewed as not reaction-capable and not a fifth full deterministic
  shuffle/private-hand/staged-reveal use because it has no per-seat private
  holdings. No `engine-core` noun, `game-stdlib` helper, or promotion debt was
  introduced; `docs/MECHANIC-ATLAS.md` §10A remains `_None_`.
- Acceptance evidence:
  - `cargo test --workspace`
  - `bash scripts/boundary-check.sh`
  - `cargo run -p simulate -- --game flood_watch --games 1000`
  - `cargo run -p replay-check -- --game flood_watch --all`
  - `cargo run -p fixture-check -- --game flood_watch`
  - `cargo run -p rule-coverage -- --game flood_watch`
  - `npm --prefix apps/web run smoke:wasm`
  - `npm --prefix apps/web run smoke:ui`
  - `npm --prefix apps/web run smoke:effects`
  - `npm --prefix apps/web run smoke:e2e`
  - `node scripts/check-catalog-docs.mjs`
  - `node scripts/check-player-rules.mjs`
  - `node scripts/check-outcome-explanations.mjs`
  - `node scripts/check-doc-links.mjs`

## Gate 11 Masked Claims

- Completed on 2026-06-11 for `masked_claims` / Masked Claims, the
  claim/challenge reaction-window proof for ROADMAP Gate 11.
- Added the accepted hidden-information bluffing proof: deterministic mask
  setup, owner-private hands, hidden reserve, claim pedestal, responder-only
  accept/challenge window, accepted masks that never reveal, challenged one-mask
  reveal, deterministic scoring and tiebreaks, public rationale, viewer-scoped
  public export/import, Level 0 and Level 1 bots for claim and response roles,
  golden traces, benchmarks, tool/CI registration, WASM/browser board,
  player-rules and outcome-explanation surfaces, and no-leak browser smoke.
- Boundary notes: mask, grade, claim, challenge, reaction window, pedestal,
  gallery, and exposed-row vocabulary stayed game-local. The fourth-use
  deterministic shuffle/private-hand/staged-reveal hard gate was reopened and
  extraction was defer/rejected; no `engine-core` noun, `game-stdlib` helper, or
  promotion debt was introduced. Reaction window / pending response is recorded
  as first official local use and remains ADR-required for broad promotion.
- Acceptance evidence:
  - `cargo test --workspace`
  - `bash scripts/boundary-check.sh`
  - `cargo run -p simulate -- --game masked_claims --games 1000`
  - `cargo run -p replay-check -- --game masked_claims --all`
  - `cargo run -p fixture-check -- --game masked_claims`
  - `cargo run -p rule-coverage -- --game masked_claims`
  - `npm --prefix apps/web run smoke:wasm`
  - `npm --prefix apps/web run smoke:ui`
  - `npm --prefix apps/web run smoke:e2e`
  - `node scripts/check-doc-links.mjs`
  - `node scripts/check-catalog-docs.mjs`
  - `node scripts/check-player-rules.mjs`
  - `node scripts/check-outcome-explanations.mjs`

Original prompt: Implement the GAT1RACTON tickets one at a time, archiving and committing each ticket before moving on.

## Gate 9 candidate placement after Gate 8

| Candidate | Placement |
|---|---|
| `token_bazaar` | Primary Gate 9 implementation target; public resource economy and accounting proof. |
| `resource_race` | Alias or alternate design label for the economy proof; do not implement separately unless a future accepted spec replaces `token_bazaar`. |
| `secret_draft` | Later simultaneous commitment / waiting / reveal proof, preferably after Token Bazaar proves public resources and browser economy UI. |
| `blackjack_lite` | Deferred comparison case under ADR 0006; not a Gate 8.1 interlock and not a Gate 9 prerequisite. |
| `poker_lite` / `plain_tricks` | Gate 10 card-depth candidates. `poker_lite` / Crest Ledger is complete for the betting/showdown half; `plain_tricks` / Plain Tricks is complete for the trick/follow-suit half. |

## Gate 10.1 Plain Tricks

- Completed on 2026-06-09 for `plain_tricks` / Plain Tricks, the
  trick/follow-suit half of ROADMAP Gate 10.
- Added the accepted hidden-hand trick-taking proof: deterministic two-round
  deal, owner-private hands, internal-only tail, must-follow-suit legality,
  void free-discard, led-suit trick resolution, trick-winner-leads turn order,
  round scoring, deal rotation, terminal win/split rationale, public replay
  export/import, Level 0 and Level 2 bots, golden traces, benchmarks, tool
  registration, WASM/browser board, e2e no-leak/a11y smoke, and full
  official-game docs.
- Boundary notes: deterministic shuffle/private-hand pressure reached its
  third-use hard gate and was explicitly deferred/rejected for extraction in
  `games/plain_tricks/docs/PRIMITIVE-PRESSURE-LEDGER.md`; no `engine-core`
  noun, `game-stdlib` helper, or promotion debt was introduced. Follow-suit
  legality, trick resolution, trick-winner-led turn order, and deal rotation
  are first-use local-only rows in the mechanic atlas.
- Acceptance evidence:
  - `cargo fmt --all --check`
  - `cargo clippy --workspace --all-targets -- -D warnings`
  - `cargo test --workspace`
  - `cargo run -p simulate -- --game plain_tricks --games 1000 --start-seed 0 --action-cap 32`
  - `cargo run -p replay-check -- --game plain_tricks`
  - `cargo run -p fixture-check -- --game plain_tricks`
  - `cargo run -p rule-coverage -- --game plain_tricks`
  - `cargo bench -p plain_tricks`
  - `bash scripts/boundary-check.sh`
  - `node scripts/check-doc-links.mjs`
  - `node scripts/check-catalog-docs.mjs`
  - `npm --prefix apps/web run smoke:wasm`
  - `npm --prefix apps/web run smoke:ui`
  - `npm --prefix apps/web run smoke:e2e`
| private monster-game red-team | Not part of this pass; leave late, optional, and isolated. |

## Gate 10 Crest Ledger

- Completed on 2026-06-09 for `poker_lite` / Crest Ledger, the
  betting/showdown half of ROADMAP Gate 10.
- Added the accepted hidden-card accounting proof: deterministic six-crest
  setup, owner-private crests, hidden center reveal, two bounded pledge rounds,
  one-lift cap per round, exact shared-pool accounting, yield terminal without
  private reveal, grouped showdown reveal, pair/high-rank/split comparator,
  public replay export/import, Level 0 and Level 2 bots, golden traces,
  benchmarks, tool registration, WASM/browser board, e2e no-leak/a11y smoke,
  and full official-game docs.
- Boundary notes: card/private-hand pressure is a second use after
  `high_card_duel`; public accounting pressure is a second use after
  `token_bazaar`; bounded pledge/shared-pool allocation is first use. All stay
  game-local. No `engine-core` noun, `game-stdlib` helper, or promotion debt was
  introduced; the mechanic atlas §10A remains empty.
- Companion scope: `plain_tricks` / Plain Tricks now closes the Gate 10
  trick/follow-suit half.
- Acceptance evidence:
  - `cargo fmt --all --check`
  - `cargo clippy --workspace --all-targets -- -D warnings`
  - `cargo test --workspace`
  - `cargo bench -p poker_lite`
  - `cargo run -p simulate -- --game poker_lite --games 1000 --start-seed 0 --action-cap 16`
  - `cargo run -p replay-check -- --game poker_lite`
  - `cargo run -p fixture-check -- --game poker_lite`
  - `cargo run -p rule-coverage -- --game poker_lite`
  - `bash scripts/boundary-check.sh`
  - `node scripts/check-doc-links.mjs`
  - `node scripts/check-catalog-docs.mjs`
  - `npm --prefix apps/web run smoke:wasm`
  - `npm --prefix apps/web run smoke:ui`
  - `npm --prefix apps/web run smoke:e2e`

## Gate 9.1 Veiled Draft

- Completed on 2026-06-08 for `secret_draft` / Veiled Draft.
- Added the accepted simultaneous commitment/reveal proof: shared visible draft
  pool, hidden per-seat commitments, public pending-seat booleans, synchronized
  reveal batch, deterministic conflict fallback, exact visible-pool removal,
  public scoring/tie-breaks, Level 0 and Level 1 bots, golden traces,
  benchmarks, tool registration, WASM/browser board, e2e no-leak/a11y smoke,
  and the full official-game docs.
- Boundary notes: commitment, reveal, draft-pool, item, fallback, scoring, and
  pending-seat vocabulary stayed game-local. No `engine-core` noun or
  `game-stdlib` commitment/reveal primitive was introduced; the mechanic atlas
  records first-use pressure only and §10A remains empty.
- Acceptance evidence:
  - `cargo test -p secret_draft`
  - `cargo test --workspace`
  - `cargo run -p simulate -- --game secret_draft --games 1000`
  - `cargo run -p replay-check -- --game secret_draft --all`
  - `cargo run -p fixture-check -- --game secret_draft`
  - `cargo run -p rule-coverage -- --game secret_draft`
  - `cargo bench -p secret_draft -- legal_actions`
  - `cargo run -p bench-report -- --input /tmp/secret_draft-benchmark-report.txt --thresholds games/secret_draft/benches/thresholds.json`
  - `npm --prefix apps/web run smoke:wasm`
  - `npm --prefix apps/web run smoke:ui`
  - `npm --prefix apps/web run smoke:preview`
  - `npm --prefix apps/web run smoke:e2e`
  - `bash scripts/boundary-check.sh`
  - `node scripts/check-doc-links.mjs`
  - `node scripts/check-catalog-docs.mjs`

## Gate 9 Token Bazaar

- Completed on 2026-06-08 for `token_bazaar` / Token Bazaar.
- Added the accepted public resource/economy proof: public supply and
  inventories, exact payments, exchange supply returns, visible market slots,
  deterministic contract refill, fixed turn cap, terminal tie-breaks, semantic
  accounting effects, Level 0 and Level 1 bots, golden traces, benchmarks, tool
  registration, WASM/browser board, e2e no-leak/a11y smoke, and the full
  official-game docs.
- Boundary notes: resource/accounting, market, contract, supply, and payment
  vocabulary stayed game-local. No `engine-core` noun or `game-stdlib` economy
  primitive was introduced; the mechanic atlas records first-use pressure only.
- Deferred scope: simultaneous hidden choices / pending seats / reveal UX remain
  assigned to the successor `secret_draft` commitment/reveal gate.

## Gate 8 High Card Duel

- Completed on 2026-06-08 for `high_card_duel` / High Card Duel.
- Added the accepted chance / hidden-information proof: deterministic setup
  shuffle, private player views, viewer-filtered effects and logs, public
  replay/export redaction, bot view discipline, browser no-leak smoke, and
  benchmark smoke floors.
- Boundary notes: card, deck, hand, commitment, and zone semantics stayed
  game-local. No `engine-core` or `game-stdlib` promotion occurred.

## Gate 7.1 board-space primitive back-port

- Completed on 2026-06-07 for the `game-stdlib::board_space` promotion-debt
  closure.
- Back-ported the behavior-free board-space coordinate/dimension primitive to
  the earlier official board games where applicable and audited `race_to_n` as
  not applicable.
- Confirmed the mechanic atlas open promotion-debt register is empty, allowing
  the roadmap to proceed to the next mechanic-ladder gate.

## Gate 7 Draughts Lite compound action tree

- Completed on 2026-06-07 for `draughts_lite` / Draughts Lite.
- Added the first serious compound-action official game proof: movement,
  mandatory capture, forced continuation, promotion, terminal detection,
  action-tree legality, replay support, fixtures, rule coverage, bots, and UI
  presentation.
- Boundary notes: draughts movement/capture semantics stayed game-local; only
  the earned behavior-free board-space primitive remained promoted.

## Gate 6 Directional Flip

- Completed on 2026-06-06 for `directional_flip` / Directional Flip.
- Added directional scan, bracketed flip, pass/no-move, grouped effect,
  preview, replay, fixture, bot, benchmark, and UI coverage for the fourth
  official game.
- Boundary notes: directional rays, legal bracketing, previews, grouped flips,
  and pattern decisions stayed local after mechanic-atlas review.

## GAT1RACTON-012

- Using the web-game development loop because this ticket adds the browser harness.
- The previous `wasm-api` Rust-callable surface is not directly callable from raw WebAssembly JS; the web harness needs a small JSON bridge export.
- Added raw wasm JSON bridge exports in `crates/wasm-api` for the six batched operations.
- Added the React `race_to_n` harness, dependency-free UI smoke script, and `games/race_to_n/docs/UI.md`.
- Browser proof loaded `http://127.0.0.1:5173/`, clicked Start Match, `add-1`, and Submit Stale; `render_game_to_text` showed counter 2, eight effects, and `stale_action`. Desktop and mobile screenshots looked coherent; console had no messages.

## Gate 3 WASM/static web shell

- Completed on 2026-06-06 for `race_to_n` / Race to 21 only.
- Added the typed TypeScript WASM client, Rust feature/catalog/replay operations, reducer-backed React shell, game picker, match setup, Race to 21 board, Rust action controls, effect log, play modes, replay UI, developer panel, base-aware static WASM loading, and browser E2E smoke coverage.
- Verification evidence:
  - `npm --prefix apps/web run smoke:wasm`
  - `npm --prefix apps/web run smoke:preview`
  - `npm --prefix apps/web run smoke:e2e`
  - `npm --prefix apps/web run build`
- Boundary notes: TypeScript remains presentation-only; legal actions, validation, effects, bots, replay projection, diagnostics, and public views come from Rust/WASM.

## Gate 5 Column Four public polish

- Completed on 2026-06-06 for `column_four` / Column Four.
- Added a full official game crate with local typed grid/column/gravity/line rules, public view projection, semantic effects, replay support, golden traces, fixtures, Level 0 and Level 2 bots, native benchmarks, WASM registration, CLI tool registration, and a first-class React/SVG board.
- Added the browser proof surface: seven Rust-legal column controls, Rust landing previews, effect-log-driven landed-piece animation, terminal win/draw display, public bot rationale, replay projection, keyboard path, reduced-motion handling, and DOM/storage/console/replay no-leak checks.
- Updated CI gates to run Column Four simulation, replay drift, fixture validation, rule coverage, WASM smoke, browser E2E, and benchmark lanes.
- Acceptance evidence:
  - `cargo test --workspace`
  - `cargo run -p simulate -- --game column_four --games 1000`
  - `cargo run -p replay-check -- --game column_four --all`
  - `cargo run -p fixture-check -- --game column_four`
  - `cargo run -p rule-coverage -- --game column_four`
  - `npm --prefix apps/web run smoke:wasm`
  - `npm --prefix apps/web run smoke:e2e`
  - `bash scripts/boundary-check.sh`
  - `node scripts/check-doc-links.mjs`
- ROADMAP Gate 5 exit mapping:
  - public page feels polished: `ColumnFourBoard` plus `column-four.smoke.mjs`
  - legal columns only are clickable: Rust legal targets, full-column inertness smoke, fixture/replay coverage
  - previews are Rust-safe: hover/focus preview from Rust `landing_preview`
  - animations come from semantic effects: landed-piece class from Rust `piece_landed`; reduced-motion smoke
  - bot explanations are available: Level 2 public rationale in bot effects and browser smoke
  - replay viewer smoke passes: export/import/step renders `ColumnFourBoard`
  - benchmark and UI smoke coverage exists: `cargo bench -p column_four` plus `smoke:e2e`
  - mechanic atlas records repeated coordinate/line pressure: `docs/MECHANIC-ATLAS.md`
- Boundary notes: fixed-grid, coordinate/targeted placement, line detection, terminal-line highlighting, column actions, and gravity are recorded as local or repeated-shape pressure only. No `engine-core` or `game-stdlib` extraction occurred.

# GAT101PLATRI-001: Plain Tricks rules prose, source notes, and admission record

**Status**: COMPLETED
**Priority**: HIGH
**Effort**: Medium
**Engine Changes**: Yes — new per-game docs under `games/plain_tricks/docs/` (`RULES.md`, `SOURCES.md`, `GAME-IMPLEMENTATION-ADMISSION.md`); no Rust/engine code
**Deps**: None

## Problem

Gate 10.1 admits a new official game, **Plain Tricks** (`plain_tricks`), the trick/follow-suit half of ROADMAP Gate 10. `docs/OFFICIAL-GAME-CONTRACT.md` §3 requires original rules prose and source/IP notes to land before implementation, and `docs/ROADMAP.md` §12 mandates that variant scope be written before coding. This ticket authors the rules foundation — stable rule IDs, the original 18-card variant, neutral IP posture, and the admission record — so every later ticket can cite it.

## Assumption Reassessment (2026-06-09)

1. The per-game doc set and naming match the `poker_lite` precedent: `games/poker_lite/docs/{RULES.md,SOURCES.md,GAME-IMPLEMENTATION-ADMISSION.md}` all exist; `games/plain_tricks/docs/` does not yet exist and is created here.
2. Spec `specs/gate-10-1-plain-tricks-trick-taking-proof.md` §3/§4 and appendix A fix the variant: two seats, 18-card deck (3 suits × ranks 1–6), 6-card hands, 6-card never-revealed tail, 6 tricks/round, 2 rounds with deal rotation, 1 point/trick, Split on 6–6, exactly 24 plays. `docs/OFFICIAL-GAME-CONTRACT.md` §3 requires rules prose precede implementation.
3. Shared boundary under audit: `RULES.md` rule IDs are consumed downstream by rule tests, `RULE-COVERAGE.md` (GAT101PLATRI-014), and the outcome-explanation surface (`UI.md`, GAT101PLATRI-019). Rule IDs must be stable and cover lead/follow legality, trick resolution, round scoring, deal rotation, terminal/tie, and visibility.
4. FOUNDATIONS §10 IP conservatism: Plain Tricks must be an original Rulepath microgame in the public-domain trick-taking family — no copied rules prose, no Whist/Hearts branding, no commercial trade dress. The display-name-from-neutral-id posture mirrors `token_bazaar` → **Token Bazaar**.

## Architecture Check

1. Front-loading rules prose + rule IDs (vs. authoring docs alongside code) lets every implementation ticket cite stable IDs and prevents rule drift between code and docs; it is the `docs/OFFICIAL-GAME-CONTRACT.md` §3 contract.
2. No backwards-compatibility aliasing/shims — this is greenfield doc authoring for a new game.
3. No `engine-core` or `game-stdlib` change; docs only. `engine-core` stays free of card/deck/suit/trick nouns.

## Verification Layers

1. Original-IP posture (no copied prose, neutral naming) -> manual review (IP-conservatism audit) against `docs/SOURCES.md` Pagat reference.
2. Rule-ID completeness (every legality/scoring/rotation/terminal/visibility rule has an ID) -> manual review + forward-consumption by GAT101PLATRI-014 `RULE-COVERAGE.md`.
3. Doc-link integrity -> `node scripts/check-doc-links.mjs`.

## What to Change

### 1. `games/plain_tricks/docs/RULES.md`

Author original rules prose with stable rule IDs for: lead/follow legality (must-follow-suit, void free-discard, leader unconstrained), trick resolution (highest led-suit rank wins; off-suit never wins; no within-trick tie), trick-winner-leads turn order, round scoring (1 point/trick), deal rotation (seat_0 leads round 1, seat_1 leads round 2; fresh shuffle from the continuing RNG stream), terminal (most total points; Split on 6–6), and visibility rules (own hand owner-only; opponent count-only; tail never revealed; played cards public from play; void revealed only implicitly by off-suit play). Document the 18-card deck, suit labels (`Gale`/`River`/`Ember`), ranks 1–6, and the exactly-24-plays bound.

### 2. `games/plain_tricks/docs/SOURCES.md`

Record the classic trick-taking research consulted (the Pagat trick-taking overview already in `docs/SOURCES.md`) and the original-IP posture per spec §F. State that deck, suits, labels, and rules text are original and that no rules prose / card imagery / product naming / trade dress is copied.

### 3. `games/plain_tricks/docs/GAME-IMPLEMENTATION-ADMISSION.md`

Record admission: the chosen variant, deliberate simplifications (no trump, two seats, 18-card deck), neutral naming, IP constraints, and confirmation that `docs/MECHANIC-ATLAS.md` §10A is empty before coding. Note the third-use hard gate is owned by GAT101PLATRI-002.

## Files to Touch

- `games/plain_tricks/docs/RULES.md` (new)
- `games/plain_tricks/docs/SOURCES.md` (new)
- `games/plain_tricks/docs/GAME-IMPLEMENTATION-ADMISSION.md` (new)

## Out of Scope

- Any Rust code, crate skeleton, or `Cargo.toml` wiring (GAT101PLATRI-004).
- The primitive-pressure ledger decision and atlas update (GAT101PLATRI-002).
- `MECHANICS.md`, `UI.md`, `AI.md`, bot-strategy docs, `RULE-COVERAGE.md`, `BENCHMARKS.md`, `HOW-TO-PLAY.md`, `PUBLIC-RELEASE-CHECKLIST.md` (later tickets).

## Acceptance Criteria

### Tests That Must Pass

1. `node scripts/check-doc-links.mjs` passes with the three new docs present.
2. `RULES.md` contains a stable rule ID for each rule category listed in What to Change §1 (manual checklist against spec §4 line 303–308).
3. Manual IP audit confirms no copied prose / commercial branding / trade dress.

### Invariants

1. No card/deck/suit/trick mechanic noun appears in any `engine-core` doc or code (none touched here).
2. Rule IDs introduced here are stable and never renamed without a documented migration (consumed by rule tests and `RULE-COVERAGE.md`).

## Test Plan

### New/Modified Tests

1. `None — documentation-only ticket; verification is command-based and existing pipeline/test coverage is named in Assumption Reassessment.`

### Commands

1. `node scripts/check-doc-links.mjs`
2. `node scripts/check-catalog-docs.mjs` (confirms no catalog regression; catalog reconciliation lands in GAT101PLATRI-018)
3. Narrower command set is correct here: this is doc authoring with no code surface; rule-ID coverage is asserted later by `cargo run -p rule-coverage -- --game plain_tricks` (GAT101PLATRI-014).

## Outcome

Completed: 2026-06-09

What changed:

- Added `games/plain_tricks/docs/RULES.md` with original Plain Tricks rules prose, stable `PT-*` rule IDs, the 18-card `plain_tricks_standard` variant, lead/follow legality, trick resolution, round scoring, deal rotation, terminal/tie rules, visibility rules, replay notes, and bot/out-of-scope boundaries.
- Added `games/plain_tricks/docs/SOURCES.md` with consulted-not-copied trick-taking research notes, neutral public naming rationale, IP/trade-dress review, ambiguity log, adopted design facts, and rule-source-to-rule-ID cross-reference.
- Added `games/plain_tricks/docs/GAME-IMPLEMENTATION-ADMISSION.md` with admission constraints, evidence expectations, mechanic-atlas admission check, primitive-pressure blocker handoff to GAT101PLATRI-002, and source/rule-ID readiness.

Deviations from original plan:

- None. The ticket remained documentation-only and did not add Rust, workspace, WASM, web, or catalog registration code.

Verification results:

- `node scripts/check-doc-links.mjs` passed (`Checked 25 markdown files`).
- `node scripts/check-catalog-docs.mjs` passed (`catalog-docs check passed — 9 games reflected in intro, root, and smoke surfaces`).
- Manual rule-ID checklist passed for lead/follow legality, leader unconstrained play, forced follow, void free-discard, trick resolution, trick-winner-led order, scoring, deal rotation, terminal/tie, visibility/no-leak, replay, and bot/out-of-scope boundaries.
- Manual IP audit passed: new prose and labels are original; sources are consulted-not-copied; no Whist/Hearts/commercial branding, copied card imagery, copied rules prose, or trade dress was introduced.

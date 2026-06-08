# GAT9TOKBAZBRO-001: Token Bazaar RULES.md + SOURCES.md

**Status**: ✅ COMPLETED
**Priority**: HIGH
**Effort**: Small
**Engine Changes**: None — documentation only (`games/token_bazaar/docs/RULES.md`, `games/token_bazaar/docs/SOURCES.md`)
**Deps**: None

## Problem

`docs/OFFICIAL-GAME-CONTRACT.md` §3 requires original rules prose and IP source
notes to precede implementation. Gate 9 introduces a new public-economy game,
`token_bazaar`, whose rules (public supply, per-player inventories, a three-slot
market over a deterministic ten-contract queue, collect / exchange / fulfill /
forced-pass actions, an 8-turn-per-seat cap, terminal + tie-breaks) must be
written down as the authoritative human-readable contract before any Rust lands,
so later tickets implement against a fixed reference rather than re-deriving rules
from code. The IP note must record that the game is original and uses only
generic mechanism vocabulary (`market`, `contracts`) — not copied commercial
rules, names, prose, or trade dress.

## Assumption Reassessment (2026-06-08)

1. The sibling game `games/high_card_duel/docs/RULES.md` and `.../SOURCES.md`
   exist and define the house structure for these two docs (verified: both files
   present under `games/high_card_duel/docs/`). `games/token_bazaar/` does not yet
   exist (verified: `ls games/token_bazaar` → No such file or directory), so both
   files are `(new)`; this ticket creates `games/token_bazaar/docs/` as part of
   authoring them.
2. The rules content is fixed by `specs/gate-9-token-bazaar-browser-proof.md`
   → "Proposed game rules" (players/visibility, resources `amber`/`jade`/`iron`,
   initial supply 14 each, initial inventory 1 each, the ten-contract standard
   queue with costs/points, turn structure, the four action families, winner +
   tie-breaks). RULES.md transcribes that section verbatim-in-intent as the
   authoritative prose; no rule is invented here.
3. Cross-artifact boundary under audit: `tools/rule-coverage` consumes
   `RULES.md` by path (verified: `tools/rule-coverage/src/main.rs` references a
   per-game `/docs/RULES.md` path). This ticket authors the doc; the rule-coverage
   registration that *reads* it lands later (GAT9TOKBAZBRO-012). RULES.md here
   must therefore enumerate every rule section so the later RULE-COVERAGE.md can
   map each to a test.
4. FOUNDATIONS §10 (IP conservatism) motivates SOURCES.md: public games must use
   original rules prose and must not copy rulebook prose, proprietary names, or
   trade dress. The market/contracts vocabulary is generic mechanism naming
   (BoardGameGeek mechanic labels) used as vocabulary only; SOURCES.md must state
   the game is original and name no commercial source as a rules model.

## Architecture Check

1. Authoring rules + IP notes first (rather than reverse-engineering them from
   the eventual Rust) keeps the human contract authoritative and gives every
   downstream ticket a single fixed reference; it is the `docs/OFFICIAL-GAME-CONTRACT.md`
   §3 ordering and matches the Gate 8 `high_card_duel` decomposition (RULES/SOURCES
   front-loaded).
2. No backwards-compatibility aliasing/shims — these are brand-new files.
3. Docs-only ticket: introduces no `engine-core` nouns and no `game-stdlib`
   change. The resource nouns (`amber`/`jade`/`iron`) are documented as
   game-local to `games/token_bazaar` only.

## Verification Layers

1. Rules completeness (every spec rule section present) -> manual review against
   `specs/gate-9-token-bazaar-browser-proof.md` "Proposed game rules".
2. IP conservatism (original prose; no copied names/rules) -> manual review /
   IP-conservatism audit per FOUNDATIONS §10.
3. Doc-link integrity (new docs resolve in the doc graph) -> `node scripts/check-doc-links.mjs`.
4. Single-artifact pair, but two distinct invariants (rule fidelity vs IP) are
   mapped to two distinct surfaces above; no collapse.

## What to Change

### 1. `games/token_bazaar/docs/RULES.md`

Author from `templates/GAME-RULES.md`. Transcribe the spec's rules:

- Players & visibility: two seats `seat_0`/`seat_1`, `seat_0` starts, all state
  public, observer and seat viewers see identical state.
- Resources: `amber`, `jade`, `iron`; initial public supply 14 each; initial
  inventory 1 each; initial score 0.
- Market: three visible slots `slot_0`/`slot_1`/`slot_2`; deterministic
  ten-contract queue (the standard table: `balanced-wares` … `crown-route` with
  costs and points); first three fill the slots; fulfilled slot refills from the
  queue front; empty when queue exhausted.
- Turn structure: alternate after every applied action; max 8 turns per seat;
  game ends after both seats take 8 turns or immediately when the last contract
  is fulfilled and no slots remain; terminal exposes no gameplay actions.
- Actions: collect (six bundles), exchange (pay 2 → take 1), fulfill (exact
  cost, score points, refill), forced pass (only when nothing else is legal).
- Winner & tie-breaks: higher score → more fulfilled contracts → higher total
  remaining inventory → draw. State explicitly that tie-breaks are public and
  deterministic.

### 2. `games/token_bazaar/docs/SOURCES.md`

Author from `templates/GAME-SOURCES.md`. State the game is original; the
`market`/`contracts` framing uses generic mechanism vocabulary only; contract
labels are original placeholders; no commercial board/card game is a rules,
prose, name, asset, or trade-dress source. Record the W3C color/keyboard
accessibility guidance as a UI-affordance reference (not a rules source).

## Files to Touch

- `games/token_bazaar/docs/RULES.md` (new)
- `games/token_bazaar/docs/SOURCES.md` (new)

## Out of Scope

- Any Rust code, data manifest, fixture, or test (later tickets).
- The other game docs (MECHANICS/UI/AI/RULE-COVERAGE/BENCHMARKS/ADMISSION/
  PUBLIC-RELEASE-CHECKLIST/COMPETENT-PLAYER/BOT-STRATEGY-EVIDENCE-PACK) — split
  across GAT9TOKBAZBRO-011/012/017.
- Implementing `secret_draft` or any simultaneous-commitment content (deferred to
  the successor Gate 9.1).

## Acceptance Criteria

### Tests That Must Pass

1. `node scripts/check-doc-links.mjs` — new docs introduce no broken links.
2. Manual review: every rule section in the spec's "Proposed game rules" appears
   in RULES.md with matching costs/points/bundles.
3. Manual IP review: SOURCES.md asserts originality and cites no commercial
   rules source.

### Invariants

1. RULES.md is the authoritative human rules contract; code in later tickets
   conforms to it, not vice versa.
2. Resource nouns (`amber`/`jade`/`iron`) and economy nouns (`market`,
   `contract`, `supply`) appear only in game-local docs — never proposed for
   `engine-core` or `game-stdlib`.

## Test Plan

### New/Modified Tests

1. `None — documentation-only ticket; verification is command-based and existing pipeline/test coverage is named in Assumption Reassessment.`

### Commands

1. `node scripts/check-doc-links.mjs`
2. `bash scripts/boundary-check.sh` — confirms no economy noun leaked into a
   boundary-checked location via the new docs' paths.
3. A narrower command is correct here because the ticket ships only prose; rule
   fidelity and IP originality are manual-review invariants, not test-runnable.

## Outcome

Completed: 2026-06-08

What changed:

- Added `games/token_bazaar/docs/RULES.md` as the authoritative human-readable
  rules contract for `token_bazaar_standard`, including public seats/visibility,
  resources, initial supply/inventory, the deterministic ten-contract queue,
  collect/exchange/fulfill/forced-pass legality, terminal conditions, and
  tie-breaks.
- Added `games/token_bazaar/docs/SOURCES.md` documenting Token Bazaar as an
  original Rulepath game, using generic market/contract vocabulary only, with no
  commercial rules, prose, names, assets, or trade dress copied.

Deviations from original plan:

- None.

Verification results:

- `node scripts/check-doc-links.mjs` passed.
- `bash scripts/boundary-check.sh` passed.
- Manual review confirmed the rules doc includes the Gate 9 contract table,
  collect bundle table, exchange/fulfill constraints, turn cap, terminal
  conditions, and tie-break order.
- Manual IP review confirmed `SOURCES.md` asserts originality and cites no
  commercial rules source as a model.

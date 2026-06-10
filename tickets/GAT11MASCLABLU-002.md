# GAT11MASCLABLU-002: Resolve fourth-use primitive-pressure ledger and update atlas §10B

**Status**: PENDING
**Priority**: HIGH
**Effort**: Small
**Engine Changes**: Yes — new `games/masked_claims/docs/PRIMITIVE-PRESSURE-LEDGER.md`; modifies `docs/MECHANIC-ATLAS.md` §10B (documentation/decision artifacts; no Rust behavior)
**Deps**: None

## Problem

FOUNDATIONS §4 makes the **fourth** official use of the `deterministic shuffle / private hand / staged reveal` mechanic shape a hard gate: implementation MUST NOT proceed until the primitive-pressure ledger records reuse / narrow-promote / defer-reject / ADR-escalate. Masked Claims is that fourth game. The atlas must also convert the `reaction window/pending response` row from candidate to realized first official use, and record the Stage-11 review outcome of the `simultaneous commitment/reveal + visible draft-pool removal` row. This ticket **blocks all implementation tickets** (GAT11MASCLABLU-003 onward).

## Assumption Reassessment (2026-06-10)

1. `games/plain_tricks/docs/PRIMITIVE-PRESSURE-LEDGER.md` records the third-use defer/reject decision this gate reopens (confirmed: 7 occurrences of the third/fourth-use framing). `games/masked_claims/docs/` is created by GAT11MASCLABLU-001; this ticket adds the ledger doc there.
2. `docs/MECHANIC-ATLAS.md` validated: §10A reads `_None_` / `_No open promotion debt remains._` (confirmed); §10B `deterministic shuffle / private hand / staged reveal` row lists exactly `high_card_duel`, `poker_lite`, `plain_tricks` with a "Reopen before a fourth official game…" next-gate (confirmed); the `reaction window/pending response` row already names `masked_claims` as a candidate with `ADR-required if generalized broadly` (confirmed); the `simultaneous commitment/reveal…` row's next-gate is literally "Stage 11 review." (confirmed). §5A promotion-conformance lifecycle exists (atlas line 81).
3. Cross-artifact boundary under audit: the §10B candidate/deferred register and the per-game `PRIMITIVE-PRESSURE-LEDGER.md` are the shared contract; §10A (open promotion-debt register) must stay `_None_` unless the decision is promote-with-debt, in which case §10A records named games/primitive/evidence/risk/closure-gate per §4.
4. FOUNDATIONS §4 (and §11) motivates this ticket: third-and-later mechanic pressure is resolved before proceeding; a promoted primitive is adopted by all matching official games or carries an explicit accepted exception.
5. Third-use mechanic hard-gate enforcement surface: this ledger decision plus atlas §10B/§10A are the enforcement record. The change is a documentation/decision artifact only — it introduces no runtime path, so it cannot leak hidden information or perturb deterministic replay/hash; per Assumption A7 the expected outcome is defer/reject, or at most a narrow behavior-free shuffle helper authorized under §5A (whose extraction, if chosen, is a separate follow-up ticket — not in this base decomposition).

## Architecture Check

1. Resolving the §4 hard gate before any shuffle/deal/visibility code prevents building on an un-adjudicated primitive — continuing through this stop condition (FOUNDATIONS §12) would be architectural debt.
2. No backwards-compatibility aliasing or shims introduced.
3. `engine-core` stays noun-free; no `game-stdlib` promotion is authorized by this ticket unless the ledger explicitly records a narrow behavior-free helper with the full §5A back-port/debt process inside this gate.

## Verification Layers

1. §4 hard gate resolved before implementation -> FOUNDATIONS alignment check + grep-proof that `PRIMITIVE-PRESSURE-LEDGER.md` records a reuse/promote/defer-reject/ADR decision.
2. §10A stays `_None_` (or records promote-debt) -> grep `docs/MECHANIC-ATLAS.md` §10A.
3. `reaction window/pending response` row converted candidate -> realized first official use -> grep `docs/MECHANIC-ATLAS.md` §10B.
4. Cross-artifact (ledger ↔ atlas) consistency -> doc-link integrity (`node scripts/check-doc-links.mjs`) + manual review that the per-game ledger and §10B rows agree.

## What to Change

### 1. `games/masked_claims/docs/PRIMITIVE-PRESSURE-LEDGER.md`

Instantiate from `templates/PRIMITIVE-PRESSURE-LEDGER.md`. Record (a) the fourth-use hard-gate decision for `deterministic shuffle / private hand / staged reveal` (reuse / narrow-promote / defer-reject / ADR — defer/reject expected per A7), cross-referencing `games/plain_tricks/docs/PRIMITIVE-PRESSURE-LEDGER.md`; and (b) the first-use `reaction window / pending response` record. If the decision is promote, this same doc must carry the §5A back-port/debt plan.

### 2. `docs/MECHANIC-ATLAS.md` §10B (and §10A if promote-with-debt)

- `deterministic shuffle / private hand / staged reveal`: add `masked_claims` as the fourth game and record the reopen decision; update §10A only if the decision is promote-with-debt (otherwise confirm §10A stays `_None_`).
- `reaction window/pending response`: convert from candidate to realized first official local use; keep the `ADR-required if generalized broadly` posture; set the next-gate trigger to the next reaction-capable game (Gate 12+ event games).
- `simultaneous commitment/reveal + visible draft-pool removal`: record the Stage-11 review outcome (expected: the masked-claims pedestal is single-seat sequential hidden placement with conditional reveal, not a second simultaneous-commitment use; row stays first-use candidate unless implementation evidence contradicts).

## Files to Touch

- `games/masked_claims/docs/PRIMITIVE-PRESSURE-LEDGER.md` (new)
- `docs/MECHANIC-ATLAS.md` (modify)

## Out of Scope

- Any `game-stdlib` extraction. A narrow behavior-free shuffle helper is permitted only if this ledger decides *promote*; that extraction (with §5A back-ports) is a separate conditional follow-up ticket, not part of this base decomposition (Assumption A7 expects defer/reject).
- Crate skeleton and all implementation code (GAT11MASCLABLU-003 onward).

## Acceptance Criteria

### Tests That Must Pass

1. `docs/MECHANIC-ATLAS.md` §10A still reads `_None_`, or records named promote-debt with a closure gate.
2. `docs/MECHANIC-ATLAS.md` §10B `deterministic shuffle / private hand / staged reveal` row names `masked_claims`; the `reaction window/pending response` row reads realized first official use.
3. `node scripts/check-doc-links.mjs` passes.

### Invariants

1. The §4 fourth-use hard gate is resolved before any implementation ticket starts (FOUNDATIONS §4/§12).
2. No `engine-core` mechanic noun and no unauthorized `game-stdlib` promotion result from this decision.

## Test Plan

### New/Modified Tests

1. `None — documentation-only ticket; verification is command-based and existing pipeline/test coverage is named in Assumption Reassessment.`

### Commands

1. `grep -n "_None_" docs/MECHANIC-ATLAS.md` (confirm §10A debt status) and `grep -n "masked_claims" docs/MECHANIC-ATLAS.md` (confirm §10B rows updated).
2. `node scripts/check-doc-links.mjs`
3. A narrower command set is correct because the decision artifact has no Rust surface to compile or test; atlas/ledger consistency is proven by grep + doc-link integrity.

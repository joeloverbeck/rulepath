# GAT19MELLEDFIV-002: forward-v1 reuse-first scaffolding audit and implementation admission

**Status**: PENDING
**Priority**: HIGH
**Effort**: Medium
**Engine Changes**: None — game-local governance docs (`games/meldfall_ledger/docs/GAME-IMPLEMENTATION-ADMISSION.md`)
**Deps**: None

## Problem

FOUNDATIONS §11 makes a forward-v1 reuse-first scaffolding audit a universal acceptance invariant for every new official game, and §12 makes starting serious implementation without it a stop condition (ADR 0008). Gate 19 is the **second** `forward-v1` audit user after Gate 18. This ticket authors the reuse-first audit (register entries MSC-8C-001…010 reviewed) and the `GAME-IMPLEMENTATION-ADMISSION.md` admission record, which **blocks** the crate skeleton (GAT19MELLEDFIV-003 depends on it). The matching post-build `ci/scaffolding-audits.json` receipt + register/atlas reconciliation lands in the closeout (GAT19MELLEDFIV-022).

## Assumption Reassessment (2026-06-25)

1. Precedent `games/blackglass_pact/docs/GAME-IMPLEMENTATION-ADMISSION.md` (first forward-v1 user) and the governance unit `archive/tickets/PREGAT18FORSCA-*` define the admission/audit anatomy; `games/blackglass_pact/docs/PRIMITIVE-PRESSURE-LEDGER.md` shows the first-use ledger shape.
2. Spec Appendix C (forward-v1 reuse-first scaffolding audit matrix, MSC-8C-001…010 review results) and §4.4 (intended `ci/scaffolding-audits.json` receipt content) define this ticket's audit substance; spec §8.2/Appendix D define the first-use primitive-pressure posture.
3. Cross-artifact: register entries `MSC-8C-001`…`MSC-8C-010` in `docs/MECHANICAL-SCAFFOLDING-REGISTER.md` are the audit targets (verified present during reassessment: 001 effect-envelope constructors, 002 seat grammar, 003 seat-count/ring, 004 action-tree v1, 005 stable-byte writer, 006 dev test-support, 007 no-leak geometry, 008 evidence-profile drivers, 009 bounded-index sampling, 010 behavioral-policy bundle rejected/local-only).
4. FOUNDATIONS §11 (reuse-first audit is a universal acceptance invariant) and §12 (skipping it is a stop condition): this ticket discharges that invariant before code; restate the audit obligation rather than trusting spec narrative.
5. Forward-v1 / scaffolding-governance enforcement surface (`scripts/check-scaffolding-governance.mjs`): confirm the admission record smuggles **no** behavior (meld/lay-off/discard-pickup/scoring/bot/visibility) into the scaffolding lane; meld/tableau/pickup/scoring stay game-owned (MSC-8C-010 `apply`/local-only). The disposition is `no-new-scaffolding` to match the blackglass precedent.

## Architecture Check

1. Authoring the audit before the skeleton makes the §11/§12 gate a hard predecessor rather than a retrofit, and records the reuse-only decision where reviewers see it before any code lands.
2. No backwards-compatibility shims — new doc.
3. `engine-core` untouched; the audit confirms `game-stdlib` gets no new rummy helper (first official use stays local) and the scaffolding register gains no behavior.

## Verification Layers

1. Every MSC-8C-001…010 entry has a reviewed disposition -> codebase grep-proof against the admission doc and `docs/MECHANICAL-SCAFFOLDING-REGISTER.md`.
2. No behavior in the scaffolding lane -> FOUNDATIONS alignment check (§4/§11; MSC-8C-010 reject/local-only restated).
3. Admission doc links resolve -> `node scripts/check-doc-links.mjs`.

## What to Change

### 1. `GAME-IMPLEMENTATION-ADMISSION.md`

Author the admission checklist with: exact authority references (`docs/FOUNDATIONS.md`, ADR 0004/0007/0008/0009, `docs/MULTI-SEAT-AND-SURFACE-CONTRACT.md`); the forward-v1 reuse-first audit reviewing MSC-8C-001…010 with reuse/`not-applicable`/`not-present` dispositions (reuse effect envelopes, seat grammar/validators, action-tree v1 framing, evidence-profile drivers, bounded-index RNG; `not-present` for stable-byte writer and production support edge); the lawful-shared-home review (`engine-core` contracts + `game-stdlib::seat`, explicitly **not** `game_stdlib::trick_taking`); first-use primitive decisions for meld/tableau/draw-discard/lay-off/cumulative-scoring (local-only, no promotion); and the `no-new-scaffolding` disposition with no prior-game retrofit.

## Files to Touch

- `games/meldfall_ledger/docs/GAME-IMPLEMENTATION-ADMISSION.md` (new)

## Out of Scope

- The post-build `ci/scaffolding-audits.json` receipt and `MECHANIC-ATLAS.md` / `MECHANICAL-SCAFFOLDING-REGISTER.md` / `PRIMITIVE-PRESSURE-LEDGER.md` reconciliation (GAT19MELLEDFIV-022).
- Any code; this ticket is the pre-code governance gate.

## Acceptance Criteria

### Tests That Must Pass

1. `node scripts/check-doc-links.mjs` passes.
2. The admission doc names every MSC-8C-001…010 entry with a disposition (grep each ID).
3. The admission doc records the `no-new-scaffolding` disposition and the `game_stdlib::trick_taking` exclusion.

### Invariants

1. No meld/lay-off/discard-pickup/scoring/bot/visibility behavior is described as scaffolding (FOUNDATIONS §4/§11; MSC-8C-010).
2. The crate skeleton (GAT19MELLEDFIV-003) does not start until this admission record exists.

## Test Plan

### New/Modified Tests

1. `None — documentation-only ticket; verification is command-based (`check-doc-links`) plus the register-ID grep, with `check-scaffolding-governance` exercised at GAT19MELLEDFIV-022 when the JSON receipt lands.`

### Commands

1. `for id in 001 002 003 004 005 006 007 008 009 010; do grep -q "MSC-8C-$id" games/meldfall_ledger/docs/GAME-IMPLEMENTATION-ADMISSION.md || echo "MISSING MSC-8C-$id"; done`
2. `node scripts/check-doc-links.mjs`
3. `node scripts/check-scaffolding-governance.mjs` is deferred to GAT19MELLEDFIV-022 (it validates the `ci/scaffolding-audits.json` receipt, not this doc).

# GAT17VOWTIDOHHEL-001: Vow Tide rules, sources, and requirements-admission receipt

**Status**: PENDING
**Priority**: HIGH
**Effort**: Medium
**Engine Changes**: None — game-local rules/source docs only (`games/vow_tide/docs/*`); no Rust/code surface
**Deps**: None

## Problem

Gate 17 admits **Vow Tide** (`vow_tide`), a 3–7-seat exact-bid trick-taking game. `docs/OFFICIAL-GAME-CONTRACT.md` §3 and FOUNDATIONS §6 require original rules prose, source notes, and a requirements-first admission receipt to exist before gameplay code. This ticket front-loads the normative rules, the rules-family source reconciliation, and the implementation-admission requirements receipt that every later ticket implements against.

## Assumption Reassessment (2026-06-21)

1. No `games/vow_tide/` tree exists yet (`ls games/vow_tide` → absent, confirmed during reassessment); the sibling `games/briar_circuit/docs/` carries the canonical doc set (`RULES.md`, `SOURCES.md`, `GAME-IMPLEMENTATION-ADMISSION.md`, …) and game-crate docs strip the `GAME-` template prefix.
2. The spec's Appendix A fixes the stable `VT-*` rule IDs and Appendix E the source reconciliation; templates `templates/GAME-RULES.md`, `templates/GAME-SOURCES.md`, `templates/GAME-IMPLEMENTATION-ADMISSION.md` exist and are the structural source.
3. Cross-artifact boundary: the `VT-*` IDs authored here are consumed by `RULE-COVERAGE.md` (ticket 016) and the rule-coverage tool, by golden traces (`rules_version = "vow-tide-rules-v1"`), and by WASM constants (017); the ID set and rules-version string are the shared contract under audit.
4. FOUNDATIONS §10 IP conservatism motivates the original-prose/no-copy posture: rules prose is newly authored from reconciled facts; no source's sequence, examples, score sheet, art, or trade dress is copied. Human IP review remains open until closeout (022).

## Architecture Check

1. Authoring rules/sources/admission first is the requirements-first order the spec mandates; it gives every downstream ticket a single normative reference and prevents rule drift across Rust, traces, and UI.
2. No backwards-compatibility shims — these are new files.
3. `engine-core` untouched; no mechanic noun enters the kernel; no `game-stdlib` change. Docs carry no behavior (§5).

## Verification Layers

1. Every locked rule has a stable `VT-*` ID → grep `games/vow_tide/docs/RULES.md` for the Appendix A ID set (`VT-IDENTITY-001` … `VT-BOUNDARY-001`).
2. Rules prose is original / no copied source → manual IP-conservatism review against `docs/IP-POLICY.md`.
3. Source reconciliation records deliberate deviations (immutable bids, exact-or-zero scoring, no extra tie hand) → manual review against spec Appendix E.

## What to Change

### 1. `RULES.md`

Author the normative rules with the Appendix A `VT-*` identifiers: identity/variant, seats (3–7, default 4, `seat_0…seat_6`, `Tide 1…Tide 7`), deck, schedule (`K=min(10,floor(51/N))`, down-to-1-up), dealer rotation, deterministic deal, turn-up trump, bid order, the exact `H-S` hook, play/follow/winner, exact `10+bid`/zero scoring, fixed terminal, competition-ranked co-winner ties, visibility, diagnostics, determinism.

### 2. `SOURCES.md`

Author the rules-family reconciliation (Pagat/Trickster + prior-art repos/thesis classified as prior art only), the 10/8/7 derivation, deliberate deviations, the Vow Tide neutral-name rationale, and the original-prose/no-copy statement.

### 3. `GAME-IMPLEMENTATION-ADMISSION.md`

Fill the requirements-first admission receipt (rules locked, seat declaration, visibility categories, budgets) — the final implementation receipt is completed at closeout.

## Files to Touch

- `games/vow_tide/docs/RULES.md` (new)
- `games/vow_tide/docs/SOURCES.md` (new)
- `games/vow_tide/docs/GAME-IMPLEMENTATION-ADMISSION.md` (new)

## Out of Scope

- `HOW-TO-PLAY.md` (co-lands with the WASM/player-rules ticket 017), `RULE-COVERAGE.md` (co-lands with rule-coverage registration 016), `MECHANICS.md` (trailing docs 021).
- Any Rust/gameplay code; human IP sign-off (closeout 022).

## Acceptance Criteria

### Tests That Must Pass

1. `node scripts/check-doc-links.mjs` — new docs link-clean.
2. `grep -c "VT-" games/vow_tide/docs/RULES.md` — the full Appendix A ID set is present.
3. Manual review: no copied prose/trade dress; deviations recorded.

### Invariants

1. Every `VT-*` rule has a single normative statement; renaming an ID after traces exist is a migration, not editorial.
2. Docs encode no selectors/conditions/behavior (§5).

## Test Plan

### New/Modified Tests

1. `None — documentation-only ticket; verification is command-based (`check-doc-links`) and the `VT-*` coverage is proven by ticket 016's rule-coverage registration.`

### Commands

1. `node scripts/check-doc-links.mjs`
2. `grep -nE "VT-[A-Z]+-[0-9]+" games/vow_tide/docs/RULES.md`
3. Narrower command is correct: no crate exists yet, so doc-link + ID-presence are the only runnable surfaces; rule-coverage validates the IDs once the tool arm lands (016).

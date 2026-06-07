# GAT72GAT8HIG-002: High Card Duel RULES.md + SOURCES.md

**Status**: PENDING
**Priority**: HIGH
**Effort**: Medium
**Engine Changes**: None — per-game docs only (`games/high_card_duel/docs/RULES.md`, `games/high_card_duel/docs/SOURCES.md`)
**Deps**: GAT72GAT8HIG-001

## Problem

Per `docs/OFFICIAL-GAME-CONTRACT.md` §3, original rules prose and IP source
notes precede implementation. Gate 8 needs the complete, stable-rule-ID'd
ruleset for `high_card_duel` and a source/originality statement (high-card
comparison inspiration, War explicitly *not* copied) before any rules code is
written, so downstream tickets and `tools/rule-coverage` have an authoritative
contract to validate against.

## Assumption Reassessment (2026-06-07)

1. Verified the sibling convention: existing games carry `docs/RULES.md` and
   `docs/SOURCES.md` (e.g. `games/draughts_lite/docs/RULES.md`); rule IDs are
   stable prefixed tokens. The spec defines the `HCD-SETUP-*`, `HCD-ROUND-*`,
   `HCD-ACT-*`, `HCD-DIAG-*` families in §4.2.2.
2. Verified against the spec: rule identity (Game ID `high_card_duel`, default
   variant `high_card_duel_standard`, seats `seat_0`/`seat_1`, 24-card local
   deck, ranks 1–12, six rounds) is fixed in §4.2.2; §11 sequencing requires
   rules docs first.
3. Cross-artifact boundary under audit: the rule-ID set is the contract
   consumed by `RULE-COVERAGE.md` (GAT72GAT8HIG-013) and every rules/test
   ticket; rule IDs authored here must be the exact strings those tickets cite.
4. FOUNDATIONS principle under audit (§10 IP conservatism): rules prose must be
   original — no War/Blackjack/poker rulebook prose, no commercial card text or
   trade dress. The doc states the rules are original Rulepath rules with neutral
   themed labels and clear numeric ranks.

## Architecture Check

1. Authoring rules + IDs first (contract-before-code) is cleaner than inferring
   rule IDs from implementation later, and prevents `RULE-COVERAGE.md` drift.
2. No backwards-compatibility shims — greenfield game docs.
3. `engine-core`/`game-stdlib` untouched; all card vocabulary stays in the
   game's docs/crate.

## Verification Layers

1. Rule-ID completeness -> codebase grep-proof: every `HCD-SETUP/ROUND/ACT/DIAG` ID in spec §4.2.2 appears in `RULES.md`.
2. IP originality -> manual review (IP-conservatism audit): no copied rulebook prose; `SOURCES.md` states War-not-copied and original-rule rationale.
3. Numeric-rank clarity -> manual review: themed labels permitted but numeric rank 1–12 unambiguous in the doc.

## What to Change

### 1. `games/high_card_duel/docs/RULES.md`

Author the complete ruleset from `templates/GAME-RULES.md`: card model (24-card
local deck, 12 ranks × 2 neutral sigils, stable IDs `hcd:r01:a`…`hcd:r12:b`),
setup (`HCD-SETUP-001..005`), round structure (`HCD-ROUND-001..013`, six rounds,
lead/reply commit, simultaneous reveal, higher-rank-scores, tie, refill,
terminal), legal actions (`HCD-ACT-001..008`), invalid/stale diagnostics
(`HCD-DIAG-001..006`). Numeric ranks explicit; sigils are identity-only and do
not affect comparison.

### 2. `games/high_card_duel/docs/SOURCES.md`

From `templates/GAME-SOURCES.md`: high-card-comparison inspiration, explicit
statement that War is **not** copied (cite the original-rule rationale from spec
Appendix A), hidden-information precedents as rationale-only, original-rule
declaration. No copied prose.

## Files to Touch

- `games/high_card_duel/docs/RULES.md` (new)
- `games/high_card_duel/docs/SOURCES.md` (new)

## Out of Scope

- Any Rust code, data files, or other game docs (MECHANICS/AI/UI/etc. land later).
- `RULE-COVERAGE.md` (co-lands with the rule-coverage tool arm in GAT72GAT8HIG-013).

## Acceptance Criteria

### Tests That Must Pass

1. `grep -c "HCD-" games/high_card_duel/docs/RULES.md` — covers every rule ID in spec §4.2.2 (counts match the spec's enumerated families).
2. `node scripts/check-doc-links.mjs` — passes.

### Invariants

1. Rule IDs in `RULES.md` are the canonical strings all downstream tickets cite.
2. No copied rulebook/commercial prose (IP-conservatism, §10).

## Test Plan

### New/Modified Tests

1. `None — documentation-only ticket; verification is command-based and existing pipeline/test coverage is named in Assumption Reassessment.`

### Commands

1. `grep -oE "HCD-[A-Z]+-[0-9]+" games/high_card_duel/docs/RULES.md | sort -u`
2. `node scripts/check-doc-links.mjs`
3. Doc-grep is the correct boundary — rule prose has no compiled surface until the rules tickets (004–008) consume these IDs.

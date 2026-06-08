# GAT72GAT8HIG-022: ADR — hidden-info replay-export taxonomy + viewer-aware visibility contract

**Status**: COMPLETED
**Priority**: HIGH
**Effort**: Small
**Engine Changes**: None — `docs/adr/0004-hidden-info-replay-export-taxonomy.md` (new)
**Deps**: GAT72GAT8HIG-002

## Problem

Gate 8 introduces (a) a viewer-scoped/public-safe replay-export taxonomy for
hidden-information games distinct from the internal full trace, and (b) a
viewer-aware `get_view` visibility contract at the WASM boundary. FOUNDATIONS §13
requires an ADR for "changing replay/hash semantics" and "changing public/private
visibility contracts." This decision ticket records the architecture decision so
the replay (009) and WASM-registration (016) tickets implement a ratified
contract rather than crossing §13 silently.

## Assumption Reassessment (2026-06-07)

1. Verified the ADR convention: `docs/adr/` holds `0001-…`/`0002-…`/`0003-…`
   plus `ADR-TEMPLATE.md`; the next number is `0004`. `docs/FOUNDATIONS.md:212-227`
   (§13) lists the replay/hash-semantics and public/private-visibility-contract
   triggers.
2. Verified against the spec: §4.2.4/§8.5 (replay-export split), §4.2.5
   (viewer-aware API), and §10.2 ("ADR only if a real architecture decision is
   made, such as changing replay export taxonomy"). The reassessed §8 alignment
   table marks §13 conditional; the user dispositioned this Issue as
   ADR-worthy (option (a)) during decomposition.
3. Cross-artifact boundary under audit: the replay-export taxonomy
   (`docs/TRACE-SCHEMA-v1.md`, `docs/TESTING-REPLAY-BENCHMARKING.md`) and the
   public/private visibility contract (`docs/WASM-CLIENT-BOUNDARY.md`,
   `docs/ENGINE-GAME-DATA-BOUNDARY.md`). The ADR must state that existing public-
   perfect-information replay is unchanged and hidden-info games add a parallel
   viewer-scoped mode.
4. FOUNDATIONS principle under audit (§13 ADR triggers): this ticket *is* the
   ADR that satisfies the trigger before the implementing tickets land; it must
   address determinism, replay/hash implications, visibility, and migration per
   the ADR discipline.

## Architecture Check

1. Recording the decision in an ADR before implementation keeps the §13 trigger
   satisfied and gives 009/016 a ratified contract — cleaner than implementing a
   new taxonomy and back-filling justification.
2. No backwards-compatibility shims — the ADR explicitly preserves existing
   public-perfect-info replay and adds a parallel hidden-info mode.
3. `engine-core`/`game-stdlib` untouched; this is a decision record. It does not
   itself add the engine-core RNG helper (the game-local helper is the chosen
   path in 004), so the ADR's scope is replay-export taxonomy + viewer-aware
   visibility only.

## Verification Layers

1. ADR completeness -> manual review: addresses typed semantics, determinism, replay/hash implications, visibility, migration, and agent safety per `docs/adr/ADR-TEMPLATE.md`.
2. §13 satisfied -> FOUNDATIONS alignment check: the replay/hash-semantics + visibility-contract triggers are explicitly cited and resolved.
3. Downstream gating -> codebase grep-proof: tickets 009 and 016 reference this ADR (their `Deps` include this ticket).

## What to Change

### 1. `docs/adr/0004-hidden-info-replay-export-taxonomy.md`

From `docs/adr/ADR-TEMPLATE.md`: decide and record the hidden-info replay-export
taxonomy (internal full trace vs public observer export vs optional seat-private
export; terminal no-auto-reveal) and the viewer-aware `get_view` visibility
contract (viewer honored for all games; perfect-info games output-equivalent;
hidden-info games filtered). State scope, determinism/replay-hash implications,
no-leak guarantees, and that existing public-perfect-info replay is unchanged.

## Files to Touch

- `docs/adr/0004-hidden-info-replay-export-taxonomy.md` (new)

## Out of Scope

- Implementing the taxonomy (GAT72GAT8HIG-009) or the WASM viewer wiring (015/016) — they consume this decision.
- The unbiased-RNG helper decision (resolved game-local in 004; not this ADR).

## Acceptance Criteria

### Tests That Must Pass

1. `node scripts/check-doc-links.mjs` — passes (ADR links resolve).
2. `test -f docs/adr/0004-hidden-info-replay-export-taxonomy.md` — ADR exists and is `Accepted`.

### Invariants

1. The §13 replay/hash-semantics + visibility-contract triggers are satisfied before 009/016 implement them.
2. Existing public-perfect-information replay semantics are preserved (additive mode only).

## Test Plan

### New/Modified Tests

1. `None — decision-record ticket; verification is doc-link/existence based and the implementing tickets (009/016) carry the behavioral tests.`

### Commands

1. `node scripts/check-doc-links.mjs`
2. `test -f docs/adr/0004-hidden-info-replay-export-taxonomy.md`
3. Doc-link + existence are the correct boundary — an ADR has no compiled surface; its behavior is tested by 009/016.

## Outcome

Completed: 2026-06-07

What changed:

- Added `docs/adr/0004-hidden-info-replay-export-taxonomy.md` with `Status: Accepted`.
- Recorded the hidden-information replay/export taxonomy: internal full trace for deterministic test/golden evidence, viewer-scoped public export as the default browser export, optional explicitly labelled seat-private export, and terminal no-auto-reveal.
- Recorded the viewer-aware WASM visibility contract: `get_view(match_id, viewer_seat)` must honor the viewer; perfect-information games may remain output-equivalent; hidden-information games must filter before browser payloads are produced.
- Preserved existing public-perfect-information replay semantics as unchanged and additive-only.
- Addressed determinism, replay/hash implications, visibility, data/Rust boundary, engine-core contamination risk, primitive pressure, UI, bot, IP, benchmark, and migration notes.

Deviations from original plan:

- None.

Verification results:

- `node scripts/check-doc-links.mjs` passed.
- `test -f docs/adr/0004-hidden-info-replay-export-taxonomy.md` passed.
- `rg -n "Status: Accepted|FOUNDATIONS|replay|visibility|get_view|public-perfect" docs/adr/0004-hidden-info-replay-export-taxonomy.md` confirmed the required acceptance/status and contract terms are present.

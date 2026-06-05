# GAT1RACTON-001: Rules research + per-game docs scaffold (requirements-first)

**Status**: PENDING
**Priority**: HIGH
**Effort**: Medium
**Engine Changes**: Yes — new `games/race_to_n/docs/` (SOURCES.md, RULES.md, RULE-COVERAGE.md skeleton, MECHANICS.md) and a `GAME-IMPLEMENTATION-ADMISSION` receipt. No Rust code.
**Deps**: None

## Problem

`race_to_n` is a `foundation-smoke` official game held to the **full** evidence
contract (spec §1; OFFICIAL-GAME-CONTRACT §2). The requirements-first workflow
(OFFICIAL-GAME-CONTRACT §3) requires rules research, original prose, the variant
+ win-condition decision, a mechanic inventory, and a rule-coverage skeleton to
land **before** any Rust, so downstream tickets implement against pinned rules
rather than inventing them. This ticket pins the variant (spec Assumption 1) and
produces the docs scaffold every later ticket cites.

## Assumption Reassessment (2026-06-05)

1. The game crate does not exist yet — `games/` contains only `.gitkeep`
   (verified `ls -A games/`). This ticket creates `games/race_to_n/docs/` only;
   no `src/` is added here (that is GAT1RACTON-004+).
2. The per-game doc templates exist under `templates/`: `GAME-SOURCES.md`,
   `GAME-RULES.md`, `GAME-RULE-COVERAGE.md`, `GAME-MECHANICS.md`, and
   `GAME-IMPLEMENTATION-ADMISSION.md` (verified `ls templates/`). Author the
   game docs from these templates and the contracts in
   `docs/OFFICIAL-GAME-CONTRACT.md` §4 (source notes), §5 (original prose), §6
   (coverage matrix), §7 (mechanic inventory).
3. Cross-artifact boundary under audit: the rule IDs authored in `RULES.md` are
   the stable keys later rule tests (GAT1RACTON-005) and the coverage matrix
   reference; the variant decision here constrains setup/state in
   GAT1RACTON-004. These are forward contracts — no existing consumer yet.
4. FOUNDATIONS §6 (official games are evidence-heavy) and §10 (IP conservatism)
   motivate this ticket: `RULES.md` MUST be original Rulepath prose, not copied
   rulebook text (IP-POLICY; OFFICIAL-GAME-CONTRACT §5), and source notes MUST
   record consulted sources (OGC §4). The mechanic family is fixed by the spec
   (tiny two-seat, perfect-information, deterministic numeric game); only the
   exact variant + win condition is pinned here.
5. Mechanic-atlas pressure: `docs/MECHANIC-ATLAS.md:166` already carries the
   `tiny numeric turn race` → `race_to_n` → `local-only` row. This ticket's
   `MECHANICS.md` inventory MUST stay consistent with that row (first use,
   local-only, no extraction). Confirming/closing the atlas row is owned by
   GAT1RACTON-014; this ticket only authors the game-local inventory.

## Architecture Check

1. Authoring rules before code is the OGC §3 requirements-first order; the
   alternative (UI/code first, backfill rules) is explicitly forbidden by OGC §3
   and hides architecture mistakes (spec §1).
2. No backwards-compatibility shims — these are new documents.
3. No Rust touched, so `engine-core` noun-freeness (§3) and `game-stdlib`
   earned-ness (§4) are trivially preserved; `MECHANICS.md` uses game-specific
   nouns (correct inside a game inventory, MECHANIC-ATLAS §2).

## Verification Layers

1. Original-prose / IP safety -> manual review (IP-conservatism audit per
   IP-POLICY; OGC §5 — no pasted rulebook text).
2. Coverage completeness -> manual review (every authored rule ID appears as a
   `RULE-COVERAGE.md` row; no silent gaps, OGC §6).
3. Variant pinned -> grep-proof (`RULES.md` states one selected variant + win
   condition; `SOURCES.md` records excluded variants).
4. Single-artifact set, no cross-crate invariants — verification is review +
   grep against the authored docs, not a code proof surface.

## What to Change

### 1. Author `games/race_to_n/docs/SOURCES.md`

From `templates/GAME-SOURCES.md` + OGC §4: sources consulted (name + URL/identifier,
consulted date), variant choice (selected + excluded), Rulepath deviations,
public-name rationale (neutral/classic — Nim/subtraction family), asset status,
open questions.

### 2. Author `games/race_to_n/docs/RULES.md`

Original Rulepath prose (OGC §5): player count + setup, components with neutral
names, objective, turn structure, legal actions, terminal/win condition, the
**single pinned variant** (e.g. take-the-last vs misère subtraction, or
race-to-N counting — choose one and record why), deliberate simplifications,
glossary. Each rule carries a stable ID/heading used by rule tests and coverage.

### 3. Author `games/race_to_n/docs/MECHANICS.md`

From `templates/GAME-MECHANICS.md` + MECHANIC-ATLAS §2 categories: topology
(none / single counter), action shape (flat), turn model, randomness (RNG used
by bot only; game deterministic), visibility (fully public), scoring/outcome,
semantic effect shape, UI interaction pattern, bot policy pattern, benchmark
pressure. Consistent with the `tiny numeric turn race` atlas row.

### 4. Author `games/race_to_n/docs/RULE-COVERAGE.md` skeleton

From `templates/GAME-RULE-COVERAGE.md` + OGC §6: one row per `RULES.md` rule ID,
status column initialized (`open`/`intentionally-deferred` allowed at this stage;
closed by GAT1RACTON-014). No silent gaps.

### 5. Author the `GAME-IMPLEMENTATION-ADMISSION` receipt

From `templates/GAME-IMPLEMENTATION-ADMISSION.md`: record `foundation-smoke`
readiness label, the gate, and the admission rationale.

## Files to Touch

- `games/race_to_n/docs/SOURCES.md` (new)
- `games/race_to_n/docs/RULES.md` (new)
- `games/race_to_n/docs/MECHANICS.md` (new)
- `games/race_to_n/docs/RULE-COVERAGE.md` (new)
- `games/race_to_n/docs/GAME-IMPLEMENTATION-ADMISSION.md` (new)

## Out of Scope

- Any Rust code, `Cargo.toml`, or `src/` (GAT1RACTON-004+).
- `AI.md`, `UI.md`, `BENCHMARKS.md` — authored by the tickets that produce their
  evidence (007/012/010); finalized in 014.
- Flipping the `docs/MECHANIC-ATLAS.md` row or the `specs/README.md` index
  (GAT1RACTON-014 / 015).
- Copying any rulebook prose, proprietary names, or trade dress (Forbidden).

## Acceptance Criteria

### Tests That Must Pass

1. `test -f games/race_to_n/docs/RULES.md && test -f games/race_to_n/docs/SOURCES.md && test -f games/race_to_n/docs/MECHANICS.md && test -f games/race_to_n/docs/RULE-COVERAGE.md` — all scaffold docs exist.
2. `node scripts/check-doc-links.mjs` — any links added resolve.
3. Manual review confirms `RULES.md` states exactly one variant + win condition and contains stable rule IDs referenced by `RULE-COVERAGE.md`.

### Invariants

1. `RULES.md` is original prose — no pasted rulebook text (IP-POLICY; OGC §5).
2. Every `RULES.md` rule ID has a `RULE-COVERAGE.md` row (no silent gaps, OGC §6).

## Test Plan

### New/Modified Tests

1. `None — documentation-only ticket; verification is command-based and existing pipeline/test coverage is named in Assumption Reassessment.`

### Commands

1. `test -f games/race_to_n/docs/RULES.md && test -f games/race_to_n/docs/RULE-COVERAGE.md`
2. `node scripts/check-doc-links.mjs`
3. Narrower command boundary is correct: this ticket adds no code, so `cargo` build/test coverage is unchanged; verification is file-presence + doc-link + IP/coverage review.

# GAT1RACTON-015: Capstone — verify §5 exit criteria end-to-end + index flip to Done

**Status**: PENDING
**Priority**: HIGH
**Effort**: Medium
**Engine Changes**: Yes (verification + index) — no new production logic; runs the spec §5 exit-criteria gauntlet and flips the `specs/README.md` Gate 1 row to `Done`.
**Deps**: GAT1RACTON-013, GAT1RACTON-014

## Problem

Gate 1 is "Done" only when its §5 exit criteria pass with evidence (spec §10;
`specs/README.md` admission rule). This capstone exercises every prior ticket
end-to-end against the spec's five exit criteria, records the evidence, and flips
the index status — the single gate-completion surface. It introduces no new
production logic.

## Assumption Reassessment (2026-06-05)

1. All implementation, evidence, CI, and docs tickets (001–014) are complete:
   engine contracts (002/003), the game (004/005/006), bot (007), replay/traces
   (008), simulation (009), benchmarks (010), wasm-api (011), web harness +
   UI smoke (012), CI (013), docs finalize (014). This ticket runs their
   composition, not their internals.
2. `specs/README.md:27` currently reads
   `| 1 | Gate 1 | [`gate-1-race-to-n.md`](gate-1-race-to-n.md) | Planned |`
   (verified). The spec §9 Documentation-updates row directs flipping this to
   `Done` only after §5 exit criteria pass with evidence. The spec §10 Sequencing
   forbids admitting Gate 2 until this row reads `Done`.
3. Cross-artifact boundary under audit: this ticket aggregates the leaf set
   (013 CI + 014 docs), whose transitive `Deps` cover every prior ticket. It
   asserts the five §5 exit criteria, each mapped to a runnable command or a
   manual-runbook step (the web human-vs-bot criterion is a UI smoke; the 100k run
   is the full simulation command).
4. FOUNDATIONS §11 (tests/traces/simulations/benchmarks/docs cover the change) and
   §12 (no stop condition crossed) motivate this ticket; the boundary review
   (spec §6 acceptance evidence) is recorded here: Rust holds all behavior, no TS
   legality, `engine-core` noun-free, dependency edges per ARCHITECTURE §2.
5. Determinism + no-leak final check: the capstone re-runs the deterministic
   replay/hash tests (008) and confirms the boundary review — Rust behavior
   authority, no hidden-state leak (perfect-info; recorded n/a per spec §6),
   deterministic replay. This is the §11 acceptance-invariant enforcement surface
   for the gate as a whole; it changes no replay/hash semantics (no §13 trigger).
6. Schema/contract: this ticket edits `specs/README.md` (one status-cell flip) and
   adds no code/schema. The flip is the documented outcome of passing exit
   criteria; it must not precede green evidence.

## Architecture Check

1. A single trailing capstone that exercises the composed pipeline (vs re-testing
   each unit) is the cleanest gate-completion surface — it proves the parts
   integrate and gates the index flip on real evidence. It introduces no
   production logic (any new artifact is an e2e/runbook only).
2. No backwards-compatibility shims.
3. `engine-core`/`game-stdlib` untouched; the boundary review confirms the kernel
   stayed noun-free across the gate.

## Verification Layers

1. human vs random bot (CLI + web) -> simulation/CLI run + UI smoke (a native
   hotseat/sim path and the bare WASM harness both play human vs the Level 0 bot
   to terminal — spec §5 row 1).
2. 100,000 native games -> simulation/CLI run (`cargo run -p simulate -- --game
   race_to_n --games 100000` completes; no panic; failing seeds reproducible —
   spec §5 row 2).
3. replay reproduces hashes -> deterministic replay-hash check (008's replay tests
   pass: state/effect/action-tree/public-view hashes reproduce — spec §5 row 3).
4. invalid/stale diagnostics tested -> named rule test + golden trace check (005/008
   diagnostic tests + the invalid/stale diagnostic golden trace — spec §5 row 4).
5. per-game docs + mechanic inventory -> codebase grep-proof + manual review
   (`games/race_to_n/docs/*` complete, no `open` coverage rows, atlas row exists —
   spec §5 row 5).
6. boundary review -> FOUNDATIONS alignment check (Rust behavior authority, no TS
   legality, `engine-core` noun-free, ARCHITECTURE §2 dependency edges — spec §6).

## What to Change

### 1. Exit-criteria runbook + e2e (capstone)

Add an e2e/smoke harness or a documented runbook (`What to Change` runbook
section) that runs each §5 exit criterion and records pass/fail evidence:
CLI+web human-vs-bot, 100k simulation, replay-hash reproduction, invalid/stale
diagnostics, docs/mechanic-inventory completeness, plus the boundary review.
Re-enumerate expected counts from fixtures at run start (no hardcoding).

### 2. Index flip

After all criteria pass, flip `specs/README.md` Gate 1 row status from `Planned`
to `Done`.

## Files to Touch

- `specs/README.md` (modify) — Gate 1 row → `Done`
- `games/race_to_n/tests/e2e_gate1.rs` (new — optional; the CI-runnable portion of the gauntlet) OR `None — verification-only` if exercised purely by existing scripts + a manual runbook

## Out of Scope

- Any new production/game logic (capstone is verification-only).
- Admitting or specifying Gate 2 (spec §10 — only after this row reads `Done`).
- Editing `docs/ROADMAP.md` (immutable law; progress lives in `specs/README.md`).

## Acceptance Criteria

### Tests That Must Pass

1. All five spec §5 exit criteria pass with recorded evidence (each mapped to a command or runbook step above).
2. `cargo run -p simulate -- --game race_to_n --games 100000` completes without panic.
3. `cargo test --workspace && npm --prefix apps/web run smoke:ui && npm --prefix apps/web run build` — full evidence pipeline green.

### Invariants

1. The `specs/README.md` Gate 1 row reads `Done` only after all §5 exit criteria pass with evidence (admission rule).
2. The boundary review confirms Rust behavior authority, no TS legality, `engine-core` noun-free, ARCHITECTURE §2 edges (spec §6; FOUNDATIONS §11/§12).

## Test Plan

### New/Modified Tests

1. `games/race_to_n/tests/e2e_gate1.rs` — optional CI-runnable gauntlet (replay-hash + diagnostics + a small sim), OR `None — verification-only` runbook exercising existing scripts.
2. Runbook steps (in What to Change) for the web human-vs-bot smoke and the 100k run (manual/nightly per TESTING §17).

### Commands

1. `cargo test --workspace`
2. `cargo run -p simulate -- --game race_to_n --games 100000`
3. `grep -n 'Gate 1' specs/README.md` — confirm the row reads `Done` after the flip (the narrower verification boundary for the index change).

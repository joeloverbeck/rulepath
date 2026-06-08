# GAT8AFTROAREA-006: Aftermath capstone — specs/README.md maintenance row, exit verification, and status flip

**Status**: PENDING
**Priority**: HIGH
**Effort**: Small
**Engine Changes**: None — verification capstone plus a `specs/README.md` index edit.
**Deps**: GAT8AFTROAREA-001, GAT8AFTROAREA-002, GAT8AFTROAREA-003, GAT8AFTROAREA-004, GAT8AFTROAREA-005

## Problem

The Gate 8 aftermath pass is not done until the realignment edits land coherently, the spec's exit criteria pass with evidence, and the `specs/README.md` index records the pass. The index currently has no row for this maintenance interlock. This trailing capstone (1) adds a 6M maintenance row to `specs/README.md`, (2) runs the spec's §Acceptance-evidence commands against the integrated tree (the README/progress/web-README/CI/SOURCES edits from 001–005), and (3) flips that row's Status to `Done`. Keeping the row creation and `Done` flip in one trailing ticket makes the `Done` marker atomic with passing evidence (spec D6 + §Exit criteria + §Acceptance evidence / WB6).

## Assumption Reassessment (2026-06-08)

1. `grep -ci aftermath specs/README.md` returns `0` — the index has no row for this pass. The index uses maintenance-row conventions `5M` (Gate 7.1), `5M` (Gate 7.2), and `6C` (post-Gate-8 Blackjack ADR closure) at `specs/README.md:35-38`; a `6M` row for this aftermath pass fits that convention. Status vocabulary is `Not started → Planned → In progress → Done` (`specs/README.md:47-49`).
2. Spec §Exit criteria (11 items), §Acceptance evidence (the command list), and §Documentation-updates-required (add the 6M maintenance row; flip to `Done` when exit criteria pass). The verifiable surfaces are delivered by upstream tickets 001–005 (declared via `Deps`); this capstone exercises them and mutates only the `specs/README.md` row.
3. Cross-artifact tie-together: this ticket validates the root-orientation docs (001/002), the web README (003), the CI smoke wiring (004), and the conditional source notes (005) against the spec's exit-criteria contract; the only mutated artifact is the `specs/README.md` index row.
4. FOUNDATIONS §11 (universal acceptance invariants) and §12 (stop conditions) are the acceptance surface for this pass: the boundary check (`engine-core` noun-free, §3) and the no-leak/determinism web smokes must pass, and no resource/card/market/contract helper may be promoted (§4). Restate these before the index flip rather than trusting the upstream tickets' self-reports.
5. The boundary-check (`scripts/boundary-check.sh`), doc-link check, and web no-leak smokes (`smoke:wasm`/`smoke:ui`/`smoke:e2e`) are the §3/§11 enforcement surfaces this capstone invokes. This pass touches docs/CI only; the capstone confirms the edits introduce no kernel mechanic noun, no leak, and no nondeterminism — it runs the existing checks, it does not modify them.

## Architecture Check

1. A single trailing verification-and-flip ticket keeps the `Done` marker atomic with passing evidence — cleaner than flipping the index inside an early doc ticket before the CI/web evidence is run.
2. No backwards-compatibility shims; no production logic introduced — a status-line edit plus a verification runbook.
3. `engine-core` is untouched; no `game-stdlib` change; the capstone's boundary check actively confirms no mechanic noun entered the kernel (§3) and no helper was promoted (§4).

## Verification Layers

1. All §Exit-criteria items hold -> manual runbook + codebase grep-proof (per-item: README/progress/web-README truthfulness greps from 001–003, CI step-presence grep from 004, candidate-note presence grep from 002).
2. Boundary stays clean -> `bash scripts/boundary-check.sh` (engine-core noun-free, §3) + FOUNDATIONS alignment check (no §4 promotion).
3. No-leak / determinism unaffected -> no-leak visibility test + golden trace / deterministic replay-hash check (`npm --prefix apps/web run smoke:e2e`; `cargo test --workspace`).
4. Doc links resolve and the index records the pass -> `node scripts/check-doc-links.mjs` + codebase grep-proof (`specs/README.md` 6M row reads `Done`).

## What to Change

### 1. Add the aftermath maintenance row

Add a `6M` row to the `specs/README.md` spec index for this pass (`gate-8-aftermath-roadmap-realignment.md`), in the maintenance-row style of the `6C` / `7.1` / `7.2` rows, initially reflecting in-progress status.

### 2. Exit-criteria runbook (performed in this ticket)

Run the spec's §Acceptance-evidence commands against the integrated tree and confirm each §Exit-criteria item: `cargo fmt --all --check`, `cargo test --workspace`, `bash scripts/boundary-check.sh`, `node scripts/check-doc-links.mjs`, `npm --prefix apps/web ci`, `npm --prefix apps/web run build`, `npm --prefix apps/web run smoke:wasm`, `npm --prefix apps/web run smoke:ui`, `npm --prefix apps/web run smoke:e2e`, plus the four `high_card_duel` native commands wired by 004. Record results.

### 3. Index status flip

Flip the new 6M row's Status to `Done` once all exit criteria pass with evidence.

## Files to Touch

- `specs/README.md` (modify)

## Out of Scope

- Any change to `README.md`, `progress.md`, `apps/web/README.md`, the CI workflow, or `docs/SOURCES.md` (those are 001–005).
- Editing `docs/ROADMAP.md` (ladder law, not a progress tracker).
- Admitting / speccing Gate 9 (separate work; `specs/gate-9-token-bazaar-browser-proof.md` already drafted).

## Acceptance Criteria

### Tests That Must Pass

1. The spec's §Acceptance-evidence command set is green on the integrated tree: `cargo fmt --all --check`, `cargo test --workspace`, `bash scripts/boundary-check.sh`, `node scripts/check-doc-links.mjs`, and the `npm --prefix apps/web` build + `smoke:wasm`/`smoke:ui`/`smoke:e2e` steps.
2. Boundary review passes: `bash scripts/boundary-check.sh` shows `engine-core` noun-free, and no resource/card/market/contract helper was promoted to `engine-core` or `game-stdlib`.
3. `specs/README.md` carries a 6M aftermath row whose Status reads `Done` only after criteria 1–2 hold.

### Invariants

1. `specs/README.md` 6M Status reads `Done` only after the exit criteria pass with evidence (atomic marker).
2. `docs/ROADMAP.md` remains unedited for progress (ROADMAP is law; the index tracks progress — spec §Documentation-update-rules).

## Test Plan

### New/Modified Tests

1. `None — verification-only capstone; verification is command/runbook-based and exercises the doc/CI surfaces that tickets 001–005 composed.`

### Commands

1. `bash scripts/boundary-check.sh; grep -rniE "resource|market|contract|card|deck" crates/engine-core/src crates/game-stdlib/src` — boundary + no-promotion review.
2. `cargo fmt --all --check && cargo test --workspace && node scripts/check-doc-links.mjs && npm --prefix apps/web run build && npm --prefix apps/web run smoke:e2e` — exit-criteria mirror.
3. `grep -niE "aftermath|6M" specs/README.md` — narrow post-edit proof that the 6M row exists and reads `Done`.

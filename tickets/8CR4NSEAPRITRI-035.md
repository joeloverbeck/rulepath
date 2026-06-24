# 8CR4NSEAPRITRI-035: Vow Tide C-08 domain-evidence profile driver

**Status**: PENDING
**Priority**: MEDIUM
**Effort**: Small
**Engine Changes**: Yes (dev-only profile adapter) — `games/vow_tide/tests/`; bid legality/scoring owned by Vow
**Deps**: 8CR4NSEAPRITRI-001

## Problem

Vow's hook/dealer-bid constraint and terminal-tie evidence are not yet validated through the shipped `domain-evidence-v1` driver (MSC-8C, C-08). Add a `domain-evidence-v1` adapter over `vow_tide_hook.fixture.json` and `vow_tide_terminal_tie.fixture.json`, delegating bid legality and competition-ranking validation to Vow Rust (spec §3.9 Vow domain, §5.9, §3.9 minimum selections).

## Assumption Reassessment (2026-06-24)

1. `crates/game-test-support/src/profiles.rs::DomainEvidenceV1Driver` exists; `games/vow_tide/data/fixtures/{vow_tide_hook,vow_tide_terminal_tie}.fixture.json` exist. Confirmed during `/reassess-spec`.
2. Spec §3.9 classifies Vow `domain-evidence-v1` as `migrate`; minimum selections are the hook negative boundary and the terminal competition ranking, validated through Vow Rust.
3. Cross-artifact: the domain-evidence contract is owned by `game-test-support`; Vow owns bid legality/scoring. Baseline fixture bytes come from `-001`.
4. §5/§11 motivate this ticket: the profile metadata is typed evidence only — no selector/formula; hook/dealer-bid constraint and terminal-tie legality/scoring delegate to Vow.
5. Enforcement surface = `DomainEvidenceV1Driver` virtual metadata over the hook and terminal-tie fixtures; fixture bytes remain unchanged unless a separate ADR-0009 packet says otherwise.

## Architecture Check

1. Virtual domain metadata delegating to Vow bid/scoring is cleaner than re-implementing the hook/tie checks in the driver — the driver validates metadata, Vow owns semantics.
2. No backwards-compatibility shim is introduced; no fixture byte changes. Rollback removes only the metadata adapter.
3. `engine-core` stays noun-free (§3); `game-test-support` is dev-only and encodes no behavior (§4/§5).

## Verification Layers

1. Driver accepts correct metadata, rejects wrong profile/version/field set -> schema/serialization validation via `DomainEvidenceV1Driver`.
2. Hook negative boundary + terminal competition ranking validated through Vow -> rule/scoring test (game-owned).
3. Fixture bytes unchanged -> `fixture-check --game vow_tide` byte-identical + codebase grep-proof (no static-data formula).

## What to Change

### 1. Add the `domain-evidence-v1` profile adapter

In `games/vow_tide/tests/` (rule/scoring module), add a `DomainEvidenceV1Driver` virtual-metadata adapter over `vow_tide_hook.fixture.json` and `vow_tide_terminal_tie.fixture.json`, with game-owned bid-legality and competition-ranking assertions delegated to Vow Rust.

## Files to Touch

- `games/vow_tide/tests/serialization.rs` (modify; or the rule/scoring test module)

## Out of Scope

- The replay-command (`-033`) and setup (`-034`) profiles; the public/seat-private export profiles are pilot credit.
- Encoding any bid/scoring formula in fixture/profile metadata; rewriting any fixture without a separate ADR-0009 packet.
- Any bid/contract/hook or scoring policy change.

## Acceptance Criteria

### Tests That Must Pass

1. `cargo test -p vow_tide` is green, including the `domain-evidence-v1` driver + game-owned hook/terminal-tie assertions.
2. `cargo run -p fixture-check -- --game vow_tide` passes with fixture bytes unchanged.
3. `bash scripts/boundary-check.sh` passes.

### Invariants

1. The driver validates metadata only; bid legality/scoring stays in Vow; fixture bytes are unchanged.
2. No procedural metadata is inserted into any fixture.

## Test Plan

### New/Modified Tests

1. `games/vow_tide/tests/serialization.rs` — `domain-evidence-v1` virtual metadata + game-owned hook/terminal-tie assertions.

### Commands

1. `cargo test -p vow_tide`
2. `cargo run -p fixture-check -- --game vow_tide`
3. The per-game fixture/rule test is the correct boundary: domain evidence delegates to Vow bid/scoring.

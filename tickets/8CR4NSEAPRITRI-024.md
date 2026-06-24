# 8CR4NSEAPRITRI-024: Vow Tide C-07 bid/trick/export/bot pairwise no-leak matrix

**Status**: PENDING
**Priority**: HIGH
**Effort**: Medium
**Engine Changes**: Yes (no-leak test geometry) — `games/vow_tide/tests/`; public bid/trump/play stay public, private IDs absent
**Deps**: 8CR4NSEAPRITRI-001, 8CR4NSEAPRITRI-012

## Problem

Vow's no-leak matrix must extend across bidding/trick phases, public and seat-private export, bot input/explanation, and candidate rendering, proving public bids/trump/plays remain public while private hand/stock IDs stay absent (MSC-8C, C-07). Add this coverage for counts 3–7 × source seat × observer/every declared seat (spec §3.8 Vow, §5.8).

## Assumption Reassessment (2026-06-24)

1. `replay_support::export_for_viewer`, the Vow action-tree/explanation seams, and `games/vow_tide/tests/visibility.rs` exist; the shared harness `assert_pairwise_no_leak` exists. Confirmed during `/reassess-spec`.
2. Spec §3.8 classifies the bid/trick/export/bot matrix as `migrate`; this ticket `Deps` `-012` for the centralized 3–7 enumeration and is a mutually independent append to the Vow test files alongside `-023`.
3. Cross-artifact: Vow projection/export (ADR 0004) and bot input are game-owned; the shared harness owns enumeration/reporting. Baseline exports come from `-001`.
4. §11 no-leak firewall + §8 bot-input rules motivate this ticket: bid values, dealer, hand size, trump indicator, played cards, and captured tricks are public as current rules permit; raw hand/stock IDs never appear in bot explanations or candidate rendering except the seat's own legal input where allowed.
5. Enforcement surface = count 3–7 × source seat × observer/every declared seat over bidding/trick phases, public + seat-private export, bot input/explanation, candidate rendering; canaries in-memory only.

## Architecture Check

1. Extending the shared matrix across bid/trick/export/bot phases gives uniform structured no-leak coverage and proves the bot path leaks no raw private IDs.
2. No backwards-compatibility shim is introduced; existing focused tests remain; no bot strategy changes.
3. `engine-core` stays noun-free (§3); `game-test-support` is dev-only (§4 mechanical-scaffolding lane).

## Verification Layers

1. Public bids/trump/plays present as rules permit; private hand/stock IDs absent across 3–7 -> no-leak visibility test via `assert_pairwise_no_leak`.
2. No private ID in public/seat-private export -> no-leak export test over `export_for_viewer` (public and seat-private classes).
3. No raw disallowed ID in bot explanation/candidates -> bot legality check over the Vow bot input/explanation path.

## What to Change

### 1. Add the bid/trick/export/bot no-leak matrix

In `games/vow_tide/tests/visibility.rs` (and the replay/bot test modules as needed), enumerate counts 3–7 × source seat × observer/every declared seat over bidding/trick phases, public + seat-private export (`export_for_viewer`), bot input/explanation, and candidate rendering via `assert_pairwise_no_leak`. Assert public data public and private hand/stock IDs absent.

## Files to Touch

- `games/vow_tide/tests/visibility.rs` (modify)
- `games/vow_tide/tests/replay.rs` (modify; or a narrowly added bot/export no-leak module)

## Out of Scope

- The hand/stock matrix (`-023`).
- Any bid/contract/hook, trump, trick, scoring, or bot-strategy change (game-local).
- Writing any canary into a committed artifact.

## Acceptance Criteria

### Tests That Must Pass

1. `cargo test -p vow_tide` is green, including the bid/trick/export/bot no-leak matrix for 3–7.
2. `cargo run -p replay-check -- --game vow_tide --all` passes (no production behavior changed).
3. `bash scripts/boundary-check.sh` passes.

### Invariants

1. Public bids/trump/plays stay public; private hand/stock IDs are absent on every unauthorized surface including bot explanations/candidates.
2. No canary token appears in any committed trace, fixture, export, snapshot, log, or test identifier.

## Test Plan

### New/Modified Tests

1. `games/vow_tide/tests/visibility.rs` / `tests/replay.rs` — bid/trick/export/bot no-leak matrix across counts 3–7 × source seat × observer/every seat.

### Commands

1. `cargo test -p vow_tide`
2. `cargo run -p replay-check -- --game vow_tide --all`
3. The per-game visibility/bot test is the correct boundary: bid/trick/export/bot no-leak is a game-local projection property.

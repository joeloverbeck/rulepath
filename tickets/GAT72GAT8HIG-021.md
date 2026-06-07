# GAT72GAT8HIG-021: Capstone — acceptance evidence + Gate 8 Done-flip + blackjack continuation resolution

**Status**: PENDING
**Priority**: HIGH
**Effort**: Medium
**Engine Changes**: None — verification capstone; `specs/README.md` (modify), spec Status flip
**Deps**: GAT72GAT8HIG-001, GAT72GAT8HIG-013, GAT72GAT8HIG-014, GAT72GAT8HIG-019, GAT72GAT8HIG-020

## Problem

Gate 8 is done only when the full acceptance-evidence command set passes end-to-
end, the `specs/README.md` index and spec Status are flipped to `Done`, and the
Part C `blackjack_lite` continuation checkpoint is resolved (a follow-up Gate
8.1/8B spec OR a recorded formal deferral) before Gate 9 may begin.

## Assumption Reassessment (2026-06-07)

1. Verified the leaf surfaces exist after the upstream tickets: tools (013),
   benches (014), e2e (019), docs+atlas (020), Gate 7.2 hygiene (001). The
   transitive `Deps` of this leaf set cover tickets 002–018 and 022.
2. Verified against the spec: §6 (exit criteria for 7.2/8/Part C), §7
   (acceptance evidence command set), and Part C §4.3/§6.3 (resolution options:
   follow-up spec or formal deferral naming the closure gate and the
   `high_card_duel` evidence that satisfied the hidden-info/chance proof).
3. Cross-artifact boundary under audit: the `specs/README.md` gate-tracker
   index (Gate 8 row → `Done`; Gate 7.2 interlock; blackjack checkpoint) and the
   spec file's Status field — this is the aggregate-completion flip, gated on all
   exit evidence passing.
4. FOUNDATIONS principle under audit (§12 stop conditions): confirm none are
   crossed at close — no hidden-info leak, no engine-core mechanic noun, no
   un-ADR'd DSL/YAML, no bot bypass, no open promotion debt advancing a gate. The
   §13 replay/visibility decision is recorded in the ADR (022).

## Architecture Check

1. A single capstone that runs the exit-criteria command set end-to-end and
   performs the Done-flip keeps completion gated on real evidence — it introduces
   no new production logic, only exercises and records.
2. No backwards-compatibility shims.
3. `engine-core`/`game-stdlib` untouched; this is verification + index/status
   bookkeeping.

## Verification Layers

1. Native acceptance -> simulation/CLI run + deterministic replay-hash + benchmark check: the §7 `cargo` command set (fmt/clippy/test/fixture-check/rule-coverage/replay-check/simulate/bench) passes.
2. Web acceptance -> no-leak visibility test + a11y scan: the §7 web command set (build + `node apps/web/e2e/*.smoke.mjs`) passes.
3. Exit-criteria coverage -> manual review (runbook): each §6.1/§6.2 exit bullet is checked off against a passing command or artifact.
4. Part C resolution -> manual review: blackjack checkpoint resolved (follow-up spec OR formal deferral with named closure gate + cited evidence); Gate 9 stays blocked until resolved.

## What to Change

### Runbook (implementer-followed) + index/status flip

1. Run the §7 acceptance-evidence command set; capture outputs as the evidence
   summary (rule-coverage matrix, golden-trace replay-check, visibility/no-leak
   report, public-export sample with hidden fields absent, UI smoke no-leak
   report, benchmark baseline/threshold rationale).
2. Verify every §6.1 (Gate 7.2) and §6.2 (Gate 8) exit-criterion is satisfied.
3. Resolve Part C: either create a follow-up `gate-8-1`/`gate-8B` `blackjack_lite`
   spec, OR record a formal deferral in `specs/README.md` (per Appendix B
   wording) naming the closure gate and the `high_card_duel` evidence that
   satisfied the deterministic-shuffle/private-view/effect-filter/no-leak proof.
4. Flip the `specs/README.md` Gate 8 row to `Done` and the spec file Status to
   `Done`; leave the blackjack checkpoint recorded/resolved before Gate 9.

## Files to Touch

- `specs/README.md` (modify — Gate 8 → `Done`; blackjack resolution)
- `specs/gate-7-2-and-gate-8-high-card-duel-hidden-info-chance-proof.md` (modify — Status → `Done`)
- `None — verification-only` for the acceptance run itself (exercises upstream tickets; modifies none of their files)

## Out of Scope

- Any production logic (this ticket exercises, it does not implement).
- Implementing `blackjack_lite` (only the checkpoint resolution is in scope).

## Acceptance Criteria

### Tests That Must Pass

1. `cargo fmt --all --check && cargo clippy --workspace --all-targets -- -D warnings && cargo test --workspace` — pass.
2. `cargo run -p rule-coverage -- --game high_card_duel`, `replay-check --game high_card_duel`, `fixture-check --game high_card_duel`, `simulate --game high_card_duel --games 1000 --start-seed 1`, `cargo bench -p high_card_duel` — pass.
3. `npm --prefix apps/web run build && node apps/web/e2e/high-card-duel.smoke.mjs && node apps/web/e2e/a11y-noleak.smoke.mjs && node apps/web/e2e/shell.smoke.mjs` — pass.

### Invariants

1. No §12 stop condition is crossed at close; no open promotion debt advances the gate.
2. Gate 8 is `Done` in index + spec; the blackjack checkpoint is resolved before Gate 9.

## Test Plan

### New/Modified Tests

1. `None — verification/closeout ticket; it runs the existing acceptance pipeline named in §7 and flips index/status.`

### Commands

1. The full §7 native + web command set (above).
2. `node scripts/check-doc-links.mjs && bash scripts/boundary-check.sh`
3. The whole-gate command set is the correct boundary — the capstone's job is end-to-end exercise + Done-flip, not a narrower filter.

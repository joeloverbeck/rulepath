# GAT10POKLITBET-018: Capstone — mechanic atlas, primitive-pressure ledger, and status reconciliation

**Status**: DONE
**Priority**: HIGH
**Effort**: Medium
**Engine Changes**: None — documentation/status reconciliation plus a tool-fixture intake correction (`docs/MECHANIC-ATLAS.md`, new `games/poker_lite/docs/PRIMITIVE-PRESSURE-LEDGER.md`, `progress.md`, `specs/README.md`, the spec's own Status, `tools/replay-check`, `tools/fixture-check`). No Rust game/engine behavior changed.
**Deps**: GAT10POKLITBET-011, GAT10POKLITBET-013, GAT10POKLITBET-016, GAT10POKLITBET-017

## Problem

Once all implementation, bot, benchmark, web, and doc tickets land, the gate must be reconciled: record the second-use/first-use mechanic notes in the atlas, author the per-game primitive-pressure ledger, update progress and the specs index, and flip the spec's Status — while exercising the distributed exit-criteria evidence end-to-end. Critically, completing `poker_lite` closes only the betting/showdown half of ROADMAP Gate 10, so the whole Gate 10 index row must NOT be marked fully `Done`.

## Assumption Reassessment (2026-06-08)

1. The atlas state is verified this session: `docs/MECHANIC-ATLAS.md` §10A (open promotion-debt register) is explicitly empty ("No open promotion debt remains"), and §10B already carries the `high_card_duel` card/private-hand entry, the `token_bazaar` accounting entry, and the `secret_draft` commitment/reveal entry — `poker_lite` adds second-use comparison notes against the first two plus a first-use note for bounded pledge / shared-pool. `templates/PRIMITIVE-PRESSURE-LEDGER.md` exists; no existing game has instantiated a per-game one (this is the first).
2. The spec (`specs/gate-10-poker-lite-betting-showdown.md` §10 documentation updates, §11 Sequencing, §12 Assumptions) fixes: keep §10A empty unless real debt is created; add the §10B notes; complete `PRIMITIVE-PRESSURE-LEDGER.md`; update `progress.md`, root `README.md` (catalog — handled in 016), `apps/web/README.md` (016); **update the existing `specs/README.md` Gate 10 row (line ~43, currently "poker_lite / plain_tricks — not yet specced | Not started")** to link this spec for the `poker_lite` portion and flip Status `Not started → Planned`/`Done` for that portion only — **not** marking the whole Gate 10 row `Done` (the trick/follow-suit half awaits `plain_tricks`). Do not edit `docs/ROADMAP.md`.
3. Cross-artifact boundary under audit: the atlas §10B ledger ↔ the new per-game `PRIMITIVE-PRESSURE-LEDGER.md` (cross-reference, not duplicate — per the spec's M3/A1 reassessment note); the `specs/README.md` index row; and the distributed exit-criteria evidence produced by the leaf tickets (011 bot docs, 013 benchmarks, 016 web/catalog/e2e, 017 trailing docs) which this capstone exercises end-to-end without adding production logic.
4. FOUNDATIONS §4 (`game-stdlib` is earned) motivates the ledger: this gate is a **second use** of card/private-hand and accounting — the atlas/ledger must record the keep-local comparison, not a promotion. Restated: no third-use hard gate fires here; no `game-stdlib` extraction; no promotion debt created.
5. §4 third-use hard-gate enforcement surface under audit (§4/§12): the mechanic atlas IS the enforcement surface for primitive-pressure. Confirm `poker_lite` is recorded as second/first use kept local, §10A stays empty (no debt), and no §12 stop condition is crossed (no unearned promotion, no third-use proceeding without a ledger decision). The per-game ledger cross-references the atlas rather than asserting a separate decision.

## Architecture Check

1. A single capstone that performs atlas/ledger/index/status reconciliation (rather than scattering status edits across implementation tickets) keeps the `Done`-flip gated on all exit evidence passing and avoids a half-reconciled index. It introduces no production logic — it exercises the pipeline the prior tickets built.
2. No backwards-compatibility aliasing/shims — additive atlas notes + new ledger + status edits.
3. `engine-core` untouched (§3); the atlas confirms `game-stdlib` stays unchanged — second use kept local (§4).

## Verification Layers

1. Atlas/ledger fidelity (second/first-use notes recorded; §10A empty; per-game ledger cross-refs atlas) -> manual review against `docs/MECHANIC-ATLAS.md` + FOUNDATIONS §4.
2. Index reconciliation (Gate 10 row links this spec for poker_lite; whole row not marked Done) -> grep-proof in `specs/README.md`.
3. Distributed exit-criteria evidence (all leaf tickets' commands pass) -> re-run the full evidence set (cargo test/bench, simulate, replay-check, fixture-check, rule-coverage, boundary, doc-links, catalog, smoke:wasm/ui/e2e).
4. Doc-link + catalog integrity -> `node scripts/check-doc-links.mjs` + `node scripts/check-catalog-docs.mjs`.

## What to Change

### 1. `docs/MECHANIC-ATLAS.md` (modify)

Add §10B notes: bounded pledge / shared-pool / showdown allocation = first official use in `poker_lite` (local only); second-use comparison for deterministic shuffle / private hand / staged reveal (first `high_card_duel`, second `poker_lite`, keep local, third-use review at `plain_tricks`); second-use comparison for public resource/accounting ledgers (first `token_bazaar`, second `poker_lite`, keep local). Keep §10A empty.

### 2. `games/poker_lite/docs/PRIMITIVE-PRESSURE-LEDGER.md` (new)

Instantiate from `templates/PRIMITIVE-PRESSURE-LEDGER.md`; record the second-use/first-use stances, **cross-referencing** the atlas §10B entries rather than duplicating them.

### 3. `progress.md` + `specs/README.md` + spec Status (modify)

Record Gate 10 `poker_lite` evidence/status in `progress.md`; update the existing `specs/README.md` Gate 10 row to link this spec for the `poker_lite` portion (Status for that portion; whole-row Gate 10 NOT marked Done); flip the spec's own Status field. Do not edit `docs/ROADMAP.md`.

## Files to Touch

- `docs/MECHANIC-ATLAS.md` (modify)
- `games/poker_lite/docs/PRIMITIVE-PRESSURE-LEDGER.md` (new)
- `progress.md` (modify)
- `specs/README.md` (modify)
- `specs/gate-10-poker-lite-betting-showdown.md` (modify — Status field only)

## Out of Scope

- Any production logic or test infra — this capstone exercises prior tickets, it does not modify them.
- `apps/web/README.md` / root `README.md` catalog lists (GAT10POKLITBET-016).
- Editing `docs/ROADMAP.md` (spec §10 forbids progress edits) and marking the whole Gate 10 row `Done` (trick half awaits `plain_tricks`).

## Acceptance Criteria

### Tests That Must Pass

1. Full evidence set re-runs green: `cargo fmt --check`, `cargo clippy --workspace --all-targets -- -D warnings`, `cargo test --workspace`, `cargo bench -p poker_lite`, `cargo run -p simulate -- --game poker_lite --games 1000 --start-seed 0 --action-cap 16`, `cargo run -p replay-check -- --game poker_lite`, `cargo run -p fixture-check -- --game poker_lite`, `cargo run -p rule-coverage -- --game poker_lite`.
2. `bash scripts/boundary-check.sh`, `node scripts/check-doc-links.mjs`, `node scripts/check-catalog-docs.mjs` all green.
3. `npm --prefix apps/web run smoke:wasm`, `smoke:ui`, `smoke:e2e` all pass.

### Invariants

1. Atlas §10A stays empty; `poker_lite` recorded as second/first use kept local; no `game-stdlib` promotion, no promotion debt (§4/§12).
2. The `specs/README.md` Gate 10 row is not marked fully `Done` on `poker_lite` landing alone (the `plain_tricks` half remains).

## Test Plan

### New/Modified Tests

1. `None — capstone/documentation reconciliation; verification re-runs the distributed exit-criteria evidence (no new production logic or tests).`

### Commands

1. `cargo test --workspace && cargo run -p rule-coverage -- --game poker_lite && cargo run -p replay-check -- --game poker_lite`
2. `bash scripts/boundary-check.sh && node scripts/check-doc-links.mjs && node scripts/check-catalog-docs.mjs`
3. `npm --prefix apps/web run smoke:wasm && npm --prefix apps/web run smoke:ui && npm --prefix apps/web run smoke:e2e` — the distributed browser-acceptance evidence.

## Outcome

Completed on 2026-06-09.

- Added the Crest Ledger primitive-pressure ledger, recording card/private-hand
  and public-accounting pressure as second-use repeated-shape candidates kept
  local, and bounded pledge/shared-pool allocation as first-use local-only.
- Updated `docs/MECHANIC-ATLAS.md` §10B and the initial atlas table while
  keeping §10A empty; no `game-stdlib` promotion or promotion debt was created.
- Updated `progress.md`, `specs/README.md`, and the spec status: the
  `poker_lite` betting/showdown half is done, while broader Gate 10 remains in
  progress because `plain_tricks` still owns the trick/follow-suit half.
- Corrected `replay-check` and `fixture-check` to recognize viewer-scoped public
  export fixtures such as `poker_lite`'s `wasm-exported.trace.json` instead of
  treating them as internal command-replay traces.

Verification:

- `cargo fmt --all --check`
- `cargo clippy --workspace --all-targets -- -D warnings`
- `cargo test --workspace`
- `cargo bench -p poker_lite`
- `cargo run -p simulate -- --game poker_lite --games 1000 --start-seed 0 --action-cap 16`
- `cargo run -p replay-check -- --game poker_lite`
- `cargo run -p fixture-check -- --game poker_lite`
- `cargo run -p rule-coverage -- --game poker_lite`
- `bash scripts/boundary-check.sh`
- `node scripts/check-doc-links.mjs`
- `node scripts/check-catalog-docs.mjs`
- `npm --prefix apps/web run smoke:wasm`
- `npm --prefix apps/web run smoke:ui`
- `npm --prefix apps/web run smoke:e2e`

# River Ledger Rule Coverage

Game ID: `river_ledger`

Variant: `river_ledger_standard`

Created: 2026-06-14

Last updated: 2026-06-14

## Coverage status

This is the pre-coding planned coverage matrix for the stable rule IDs in
`RULES.md`. Code, tests, traces, and tool registration land in later
`GAT15RIVLEDTEX-*` tickets, so most rows are intentionally `open` until the
owning implementation ticket closes. The final reconciliation lands with the
tool registration and capstone tickets.

Status values follow `docs/OFFICIAL-GAME-CONTRACT.md`: `covered`,
`covered-by-trace`, `not-applicable`, `intentionally-deferred`, `unsupported`,
or `open`.

## Planned matrix

| Rule IDs | Requirement | Planned implementation owner | Planned proof | Current status |
|---|---|---|---|---|
| `RL-SETUP-SEATS-*` | Accept 3-6 seats and reject every other count deterministically. | `setup.rs`, `ids.rs`, `variants.rs` | rule tests, setup fixtures, invalid-seat trace, WASM setup smoke | open |
| `RL-SETUP-VARIANT-*` | Select typed standard variant; static data remains metadata. | `variants.rs`, `ui.rs` | strict static-data tests, boundary check | open |
| `RL-DEAL-DECK-*` | Stable game-local deck construction and internal deck-tail handling. | `cards.rs`, `setup.rs` | deterministic setup tests, serialization tests, no-leak tests | open |
| `RL-DEAL-SHUFFLE-*` | Seeded deterministic shuffle. | `cards.rs`, `setup.rs` | same-seed equality, different-seed variance, replay/hash tests | open |
| `RL-DEAL-HOLE-*` | Two owner-private hole cards per seat. | `setup.rs`, `visibility.rs` | seat-private projection tests, pairwise no-leak matrix, replay export no-leak | open |
| `RL-DEAL-BOARD-*` | Flop/turn/river board reveal and internal future-board/burn redaction. | `setup.rs`, `rules.rs`, `visibility.rs`, `effects.rs` | street traces, effect tests, no-leak tests | open |
| `RL-BET-BUTTON-*` | Deterministic button/blind roles and split-remainder order. | `setup.rs`, `betting.rs`, `pot.rs` | setup tests, split-remainder trace | open |
| `RL-BET-BLIND-*` | Forced small/big blind contributions. | `setup.rs`, `betting.rs` | setup fixtures, rule tests, simulator smoke | open |
| `RL-BET-ACTION-*` | Legal `fold`, `check`, `call`, `bet`, `raise`; reject wrong/stale/unavailable commands. | `actions.rs`, `rules.rs` | rule tests, wrong-seat/stale/cap traces, property tests | open |
| `RL-BET-LIMIT-*` | Small/big fixed-limit units by street. | `betting.rs` | rule tests and traces for preflop/flop/turn/river | open |
| `RL-BET-CAP-*` | One opening bet plus three raises per street. | `betting.rs`, `actions.rs` | cap diagnostics tests, raise-cap golden trace | open |
| `RL-BET-CALL-*` | Exact call amount and no mutation on invalid call. | `betting.rs`, `rules.rs` | rule tests, contribution property tests | open |
| `RL-BET-RAISE-*` | Exact raise amount and cap enforcement. | `betting.rs`, `rules.rs` | cap tests, property tests | open |
| `RL-BET-CHECK-*` | Check only when no amount is owed. | `actions.rs`, `rules.rs` | legal-action and validation tests | open |
| `RL-BET-AMB-*` | Base model cannot enter all-in-required states. | `betting.rs`, `pot.rs` | property tests and fixture review | open |
| `RL-STREET-PREFLOP-*` | Preflop action order after blinds. | `rules.rs`, `betting.rs` | golden trace and wraparound rule tests | open |
| `RL-STREET-FLOP-*` | Three-card flop reveal and small-unit round. | `rules.rs`, `effects.rs` | street reveal trace and effect tests | open |
| `RL-STREET-TURN-*` | One-card turn reveal and big-unit round. | `rules.rs`, `effects.rs` | trace and rule tests | open |
| `RL-STREET-RIVER-*` | One-card river reveal, big-unit round, showdown transition. | `rules.rs`, `effects.rs` | trace and rule tests | open |
| `RL-STREET-SHOWDOWN-*` | Terminal showdown after river closes. | `rules.rs`, `showdown.rs` | showdown traces, replay/hash tests | open |
| `RL-STREET-FOLDOUT-*` | Terminal last-live-hand outcome. | `rules.rs`, `showdown.rs` | foldout trace and no-leak tests | open |
| `RL-EVAL-*` | Five-card categories, ace-low straight, seven-card best hand, tie-break vector, used cards. | `evaluator.rs` | category unit tests, antisymmetry/determinism property tests, showdown traces | open |
| `RL-SHOW-ELIGIBLE-*` | Only live showdown seats are evaluated. | `showdown.rs` | rule tests and foldout/showdown traces | open |
| `RL-SHOW-WINNER-*` | Single strongest hand wins. | `showdown.rs`, `pot.rs` | winner traces, outcome explanation tests | open |
| `RL-SHOW-SPLIT-*` | Tied best hands split. | `showdown.rs`, `pot.rs` | split traces and allocation tests | open |
| `RL-SHOW-FOLDOUT-*` | Foldout explanation without folded-card reveal. | `showdown.rs`, `visibility.rs` | foldout no-leak tests and trace | open |
| `RL-POT-SINGLE-*` | Single-pot contribution allocation. | `pot.rs`, `betting.rs` | conservation/property tests, simulator checks | open |
| `RL-POT-ALLIN-*` | All-in/side-pot state absent. | `pot.rs`, `betting.rs` | property tests and explicit unsupported diagnostics if needed | open |
| `RL-POT-REMAINDER-*` | Stable button-order split remainder. | `pot.rs` | split-remainder trace | open |
| `RL-VIS-*` | Viewer-safe public/seat/private projections, diagnostics, foldout/showdown redaction, view hashes. | `visibility.rs`, `effects.rs`, `replay_support.rs` | ordered pair no-leak matrix for 3-6 seats, observer no-leak, browser payload smoke | open |
| `RL-REPLAY-*` | Deterministic replay, viewer-scoped export/import, stable serialization. | `replay_support.rs`, tests | replay tests, serialization tests, replay-check tool | open |
| `RL-BOT-*` | Legal-action-only L0/L1/L2 bots and viewer-safe explanations. | `bots.rs`, `AI.md`, evidence pack | bot legality tests, no-leak explanation tests, seeded simulations | open |
| `RL-UI-*` | Presentation-only web surface, legal-only controls, safe ledger/outcome/no-leak UI. | `ui.rs`, WASM bridge, `RiverLedgerBoard.tsx` | web build, smoke:wasm, smoke:ui, smoke:e2e, DOM/storage/log no-leak | open |
| `RL-SETUP-AMB-*`, `RL-DEAL-AMB-*`, `RL-EVAL-AMB-*`, `RL-POT-AMB-*`, `RL-VIS-AMB-*` | Chosen ambiguity resolutions. | corresponding modules/docs | targeted rule tests and trace evidence | open |
| `RL-VAR-*`, `RL-OOS-*` | Variant deviations and explicit non-goals. | docs, validators where applicable | docs review, static-data tests, no side-pot/all-in implementation | open |

## Current gaps and blockers

- No Rust crate exists yet; implementation starts after the admission spine is
  reviewed and archived.
- `tools/rule-coverage` does not yet know the `RL-` prefix; that is owned by
  the later tool-registration ticket.
- Golden traces, fixtures, simulation, replay-check, fixture-check, benchmarks,
  WASM, web smoke, and final public-release docs are planned but not yet
  implemented.

This partial status is intentional and is not a claim of official-game
completion.

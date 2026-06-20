# River Ledger AI

Game ID: `river_ledger`

Variant: `river_ledger_standard`

Rules version: `river-ledger-rules-v2`

Status: L0, L1, and L2 bot policies implemented for the Rust crate.

## Bot Registry

| Level | Policy id | Status | Summary |
|---:|---|---|---|
| 0 | `river-ledger-random-legal-v0` | implemented | Seeded random choice from the Rust legal-action tree. |
| 1 | `river-ledger-conservative-level1-v1` | implemented | Conservative legal heuristic: check free, call cheap, distinguish call all-in, fold poor price. |
| 2 | `river-ledger-level2-v1` | implemented | Authored stack-aware policy from [BOT-STRATEGY-EVIDENCE-PACK.md](BOT-STRATEGY-EVIDENCE-PACK.md). |

## Information Boundary

Bots use only:

- Rust legal-action trees;
- public projection fields from `PublicView`;
- the acting seat's `PrivateView::Seat.hole_cards`;
- public stack, call-price, all-in, pot-tier, and eligibility metadata exposed through legal actions and authorized views;
- deterministic seed only for tie-breaking.

Bots do not use opponent hole cards, future community cards, burn/deck-tail
facts, raw internal traces, private diagnostics, hidden-state sampling, MCTS,
ISMCTS, Monte Carlo, ML, RL, solvers, or TypeScript legality.

## Explanation Boundary

Bot decisions may expose policy id, action family, own authorized hand bucket,
public price, stack pressure, all-in action class, live-opponent count,
street/cap pressure, and deterministic legal-action rationale. They must not
expose card ids, opponent private facts, future cards, sampled hidden states,
or solver claims.

## Verification

- `cargo test -p river_ledger --test bots`
- `cargo test -p river_ledger`
- `node scripts/check-doc-links.mjs`

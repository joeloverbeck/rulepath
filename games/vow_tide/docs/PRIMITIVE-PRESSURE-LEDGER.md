# Primitive Pressure Ledger: Vow Tide numeric trick contracts

Candidate name: `vow-tide-numeric-trick-contracts`

Status: first official use recorded; keep local

Decision date: 2026-06-21

Prepared by: `Codex`

## First-Use Review

Vow Tide is the first official Rulepath game with a sequential public numeric
bid whose success is judged against tricks later taken. Prior official games
include commitments, claims, pledges, or trick counts, but not this
contract-vs-result mechanic with a dealer-last total-not-equal hook.

Decision: implement locally in `games/vow_tide`.

No helper is added to `engine-core` or `game-stdlib`. No promotion debt is
created.

## Mechanic Shape

```text
sequential public bid/<n> action;
current hand-size bounded range 0..=H;
dealer-last hook excluding H minus prior public bid total;
accepted bids public and immutable;
future exact contract-vs-tricks scoring.
```

## Boundary Decision

| Factor | Finding |
|---|---|
| use count | first official use |
| decision | `local-only` |
| why not `engine-core` | Bid, contract, dealer, hook, hand size, trick count, and score are game/mechanic nouns. |
| why not `game-stdlib` | One implementation cannot prove a reusable behavior-free boundary; a generic helper would need policy for order, hook, scoring, visibility, effects, bots, and UI. |
| data/Rust boundary | Static data names the variant only. Bid legality, hook exclusion, validation, effects, and scoring stay in typed Rust. |
| replay/hash impact | Accepted bids mutate deterministic Rust state and advance freshness; no unsubmitted UI choice is authoritative state. |
| visibility impact | Accepted bids are public. Unsubmitted choices, private hands, hidden stock, and hidden-derived bot features are not exposed by bid metadata or effects. |
| bot/UI impact | Bots and TypeScript consume Rust legal leaves; neither may sum bids or remove the hook value independently. |

## Local Implementation Notes

The local modules are:

- `src/actions.rs`: parses `bid/<decimal_u8>`, emits ascending legal leaves, and
  omits the dealer hook value when public state makes it applicable.
- `src/rules.rs`: independently validates freshness, seat/phase/order, range,
  duplicate immutable bids, and hook-forbidden dealer bids.
- `src/effects.rs`: records public bid acceptance and dealer-hook constrained
  effects without cards, stock, or hidden future facts.
- `src/state.rs`: stores accepted bids as seat-keyed public `Option<u8>` rows
  during bidding.

## Tests Required

| Test | Current status |
|---|---|
| bid order left of dealer through dealer | `games/vow_tide/tests/rules.rs` |
| `0..=H` legal range | `games/vow_tide/tests/rules.rs` |
| dealer hook excludes exactly `H-S` when in range | `games/vow_tide/tests/rules.rs` and `tests/property.rs` |
| out-of-range prefix removes no legal bid | `games/vow_tide/tests/rules.rs` and `tests/property.rs` |
| legal-tree and validator equivalence | `games/vow_tide/tests/property.rs` |
| immutable accepted bids | `games/vow_tide/tests/rules.rs` |
| no behavior in static data | deferred to Vow Tide fixture registration ticket; current data files contain identity/presentation fields only |

## Rejected Alternatives

| Alternative | Why rejected |
|---|---|
| Promote a bid helper now | First use only; no second implementation exists for comparison. |
| Encode hook/range in TOML | Would make static data behavior-bearing and violate Rulepath foundations. |
| Let UI compute the hook | TypeScript legality is forbidden; the browser may only present Rust leaves and metadata. |
| Add a generic contract/scoring framework | Scoring and outcome behavior are not implemented enough to prove a reusable boundary and would introduce game nouns. |

## Next Review Trigger

Reopen this ledger at Gate 18 or before any later official game implements a
close numeric trick bid, dealer hook, or contract-vs-result scoring mechanic.
The next review should compare Vow Tide's local implementation before proposing
reuse, promotion, defer/reject, or ADR.

# Token Bazaar Bot Strategy Evidence Pack

Game ID: `token_bazaar`

Last updated: 2026-06-08

## Status

Level 1 shipped for Gate 9.

Policy id: `token_bazaar_level1_v1`

Implementation: `TokenBazaarLevel1Bot` in `games/token_bazaar/src/bots.rs`

## Policy Summary

The Level 1 bot chooses from the active seat's Rust legal actions:

1. highest-value affordable visible contract;
2. collect toward the best visible target;
3. exchange toward the best visible target;
4. stable first legal fallback.

It emits a short public rationale. It does not expose candidate rankings, debug values, valuation tables, hidden state, internal state, or serialized decision structures.

## Evidence Fixtures

| Evidence | Test / trace | Expected behavior |
|---|---|---|
| random baseline validates | `random_bot_selection_validates_through_normal_command_path` | Level 0 decision validates through normal command path. |
| Level 1 validates across public states | `level1_selection_validates_across_public_states` | Every sampled public decision validates and rationale is public-safe. |
| deterministic fixed-state decision | `level1_decision_is_deterministic_for_fixed_state_and_seed` | Same state and seed produce identical decision. |
| fulfill priority | `level1_fulfills_affordable_contract` and `golden_traces/bot-action.trace.json` | Initial state chooses `fulfill/slot_0` with affordable visible-contract rationale. |
| collect toward target | `level1_collects_toward_unaffordable_visible_contract` | With only `Amber Focus` visible and unaffordable, chooses `collect/amber`. |
| forced pass fallback | `level1_fallback_reaches_forced_pass` | With no supply, inventory, visible contracts, or queue, chooses `pass`. |
| inactive seat guard | `inactive_bot_has_no_legal_actions` | Inactive bot request returns `no_legal_actions`. |
| playout validation | `level1_bot_actions_validate_during_playout` | Level 1 decisions validate during repeated play. |
| replay no-leak terms | replay no-leak assertions | Bot trace excludes `bot_debug`, `bot_candidate`, and hidden/private/internal terms. |

## Public Rationale Samples

Allowed rationale examples from the shipped policy:

- `Fulfilled the highest-value affordable visible contract.`
- `Collected resources toward the visible Amber Focus contract.`
- `Exchanged resources to reduce the public cost gap for Amber Focus.`
- `Chose the first legal action after equivalent public options were resolved deterministically.`

These examples name only public state. They must not grow candidate lists or debug math in public output.

## Verification Commands

- `cargo test -p token_bazaar --test bots`
- `cargo test -p token_bazaar --test property level1_bot_actions_validate_during_playout`
- `cargo run -p replay-check -- --game token_bazaar --all`
- `cargo run -p simulate -- --game token_bazaar --games 1000`

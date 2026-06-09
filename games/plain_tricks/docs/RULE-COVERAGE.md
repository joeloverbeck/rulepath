# Plain Tricks Rule Coverage

Game ID: `plain_tricks`

Rules version: `plain-tricks-rules-v1`

Last updated: 2026-06-09

## Rule Coverage Matrix

| Rule ID | Rule summary | Implementation | Evidence | Status | Notes |
|---|---|---|---|---|---|
| `PT-ACT-001` | Leaders may play any card from hand. | `games/plain_tricks/src/actions.rs` | `games/plain_tricks/tests/property.rs`; golden command traces | `covered` | |
| `PT-ACT-002` | Followers with led-suit cards must play led suit. | `games/plain_tricks/src/actions.rs` | `games/plain_tricks/tests/property.rs`; diagnostic traces | `covered` | |
| `PT-ACT-003` | Followers void in led suit may play any held card. | `games/plain_tricks/src/actions.rs` | `games/plain_tricks/tests/visibility.rs`; `games/plain_tricks/tests/property.rs` | `covered` | |
| `PT-ACT-004` | Terminal states expose no gameplay actions. | `games/plain_tricks/src/actions.rs` | terminal golden traces; `games/plain_tricks/tests/replay.rs` | `covered-by-trace` | |
| `PT-ACT-005` | Legal-action metadata is viewer safe. | `games/plain_tricks/src/actions.rs` | `games/plain_tricks/tests/visibility.rs`; no-leak golden traces | `covered` | |
| `PT-ACT-006` | Non-actor viewers receive empty card action trees. | `games/plain_tricks/src/actions.rs` | `games/plain_tricks/tests/visibility.rs`; no-leak golden traces | `covered` | |
| `PT-BOT-001` | Random bot selects legal leaves deterministically. | `games/plain_tricks/src/bots.rs` | `games/plain_tricks/tests/bots.rs` | `covered` | |
| `PT-BOT-002` | Level 2 bot uses own-hand/public facts only. | `games/plain_tricks/src/bots.rs` | `games/plain_tricks/tests/bots.rs`; `games/plain_tricks/docs/BOT-STRATEGY-EVIDENCE-PACK.md` | `covered` | |
| `PT-COMP-001` | Two public seats exist. | `games/plain_tricks/src/ids.rs`; `games/plain_tricks/src/setup.rs` | `games/plain_tricks/tests/serialization.rs`; golden traces | `covered` | |
| `PT-COMP-002` | Trick cards have stable ids, suits, ranks, and labels. | `games/plain_tricks/src/ids.rs`; static data | `games/plain_tricks/tests/serialization.rs`; `cargo run -p fixture-check -- --game plain_tricks` | `covered` | |
| `PT-COMP-003` | Suits are game-local and control follow rules. | `games/plain_tricks/src/ids.rs`; `games/plain_tricks/src/actions.rs` | `games/plain_tricks/tests/property.rs` | `covered` | |
| `PT-COMP-004` | Ranks resolve same-suit tricks. | `games/plain_tricks/src/ids.rs`; `games/plain_tricks/src/rules.rs` | `games/plain_tricks/tests/property.rs`; `games/plain_tricks/tests/replay.rs` | `covered` | |
| `PT-COMP-005` | Private hands are owner-only until played. | `games/plain_tricks/src/state.rs`; `games/plain_tricks/src/visibility.rs` | `games/plain_tricks/tests/visibility.rs`; no-leak traces | `covered` | |
| `PT-COMP-006` | Tail cards remain internal. | `games/plain_tricks/src/state.rs`; `games/plain_tricks/src/visibility.rs` | `games/plain_tricks/tests/replay.rs`; `games/plain_tricks/tests/visibility.rs` | `covered` | |
| `PT-COMP-007` | Current trick stores the active play sequence. | `games/plain_tricks/src/state.rs`; `games/plain_tricks/src/rules.rs` | `games/plain_tricks/tests/property.rs` | `covered` | |
| `PT-COMP-008` | Resolved trick history is public. | `games/plain_tricks/src/state.rs`; `games/plain_tricks/src/visibility.rs` | `games/plain_tricks/tests/visibility.rs`; golden traces | `covered` | |
| `PT-COMP-009` | Round score tracks tricks won this round. | `games/plain_tricks/src/state.rs`; `games/plain_tricks/src/rules.rs` | `games/plain_tricks/tests/property.rs` | `covered` | |
| `PT-COMP-010` | Match total determines terminal outcome. | `games/plain_tricks/src/state.rs`; `games/plain_tricks/src/rules.rs` | terminal golden traces; `games/plain_tricks/tests/replay.rs` | `covered` | |
| `PT-END-001` | Higher total after round 2 wins. | `games/plain_tricks/src/rules.rs` | terminal golden traces; `games/plain_tricks/tests/replay.rs` | `covered-by-trace` | |
| `PT-END-002` | 6-6 totals produce split. | `games/plain_tricks/src/rules.rs` | `games/plain_tricks/tests/replay.rs`; terminal golden traces | `covered` | |
| `PT-END-003` | Terminal outcome is final and tail stays hidden. | `games/plain_tricks/src/actions.rs`; `games/plain_tricks/src/visibility.rs` | terminal and no-leak golden traces | `covered-by-trace` | |
| `PT-OOS-001` | Trump, bidding, partnerships, penalties, passing, dummy play, and 3+ seats are out of scope. | no implementation path | `games/plain_tricks/docs/RULES.md`; `games/plain_tricks/docs/GAME-IMPLEMENTATION-ADMISSION.md` | `not-applicable` | Explicitly out of scope for this variant. |
| `PT-OOS-002` | No general trick-taking framework is added. | local `games/plain_tricks` modules only | `bash scripts/boundary-check.sh`; crate layout review | `covered` | |
| `PT-OOS-003` | Static data cannot define behavior. | static data loaders plus fixture checks | `cargo run -p fixture-check -- --game plain_tricks`; `games/plain_tricks/tests/serialization.rs` | `covered` | |
| `PT-OOS-004` | Solver and learning bots are forbidden. | `games/plain_tricks/src/bots.rs` | `games/plain_tricks/tests/bots.rs`; source scan | `covered` | |
| `PT-RESTRICT-001` | Unknown or non-seat actors are rejected. | `games/plain_tricks/src/actions.rs` | diagnostic golden traces; `games/plain_tricks/tests/property.rs` | `covered` | |
| `PT-RESTRICT-002` | Wrong active seat is rejected. | `games/plain_tricks/src/actions.rs` | diagnostic golden traces; `games/plain_tricks/tests/property.rs` | `covered` | |
| `PT-RESTRICT-003` | Malformed or unavailable paths are rejected. | `games/plain_tricks/src/actions.rs` | diagnostic golden traces; `games/plain_tricks/tests/replay.rs` | `covered` | |
| `PT-RESTRICT-004` | Stale commands are rejected without mutation. | `games/plain_tricks/src/actions.rs` | diagnostic golden traces; replay hash tests | `covered` | |
| `PT-RESTRICT-005` | Cards outside actor hand are rejected safely. | `games/plain_tricks/src/actions.rs` | diagnostic golden traces; `games/plain_tricks/tests/visibility.rs` | `covered` | |
| `PT-RESTRICT-006` | Off-suit follow while holding led suit is rejected safely. | `games/plain_tricks/src/actions.rs` | diagnostic golden traces; `games/plain_tricks/tests/property.rs` | `covered` | |
| `PT-RESTRICT-007` | Terminal gameplay submissions are rejected. | `games/plain_tricks/src/actions.rs` | terminal golden traces; replay checks | `covered-by-trace` | |
| `PT-RNG-001` | Deals use deterministic seeded shuffles. | `games/plain_tricks/src/setup.rs`; `games/plain_tricks/src/replay_support.rs` | `games/plain_tricks/tests/replay.rs`; golden hashes | `covered` | |
| `PT-RNG-002` | Public replay export is redacted. | `games/plain_tricks/src/replay_support.rs` | `games/plain_tricks/tests/replay.rs`; export traces | `covered` | |
| `PT-RNG-003` | Serialization order remains stable. | stable serializers and replay support | `games/plain_tricks/tests/serialization.rs`; `cargo run -p replay-check -- --game plain_tricks` | `covered` | |
| `PT-SCORE-001` | Won tricks add one round point. | `games/plain_tricks/src/rules.rs` | `games/plain_tricks/tests/property.rs` | `covered` | |
| `PT-SCORE-002` | Six tricks close a round into match totals. | `games/plain_tricks/src/rules.rs` | `games/plain_tricks/tests/property.rs`; terminal traces | `covered` | |
| `PT-SCORE-003` | Round 1 closes into a fresh rotated deal. | `games/plain_tricks/src/rules.rs`; `games/plain_tricks/src/setup.rs` | `games/plain_tricks/tests/property.rs`; `games/plain_tricks/tests/replay.rs` | `covered` | |
| `PT-SETUP-001` | Setup creates exactly two seats. | `games/plain_tricks/src/setup.rs` | `games/plain_tricks/tests/serialization.rs`; fixture check | `covered` | |
| `PT-SETUP-002` | Deck is stable then seeded-shuffled. | `games/plain_tricks/src/ids.rs`; `games/plain_tricks/src/setup.rs` | `games/plain_tricks/tests/replay.rs`; `games/plain_tricks/tests/property.rs` | `covered` | |
| `PT-SETUP-003` | Deal six cards to each seat and keep six tail cards. | `games/plain_tricks/src/setup.rs`; `games/plain_tricks/src/state.rs` | `games/plain_tricks/tests/property.rs`; visibility tests | `covered` | |
| `PT-SETUP-004` | Round 1 starts with seat_0. | `games/plain_tricks/src/setup.rs` | `games/plain_tricks/tests/property.rs`; golden traces | `covered` | |
| `PT-SETUP-005` | Round 2 redeals from continuing RNG and starts with seat_1. | `games/plain_tricks/src/rules.rs`; `games/plain_tricks/src/setup.rs` | `games/plain_tricks/tests/property.rs`; replay tests | `covered` | |
| `PT-SETUP-006` | Deal effects keep private card identities scoped. | `games/plain_tricks/src/effects.rs`; `games/plain_tricks/src/visibility.rs` | `games/plain_tricks/tests/visibility.rs`; no-leak traces | `covered` | |
| `PT-TRICK-001` | Leader card establishes led suit. | `games/plain_tricks/src/rules.rs`; `games/plain_tricks/src/actions.rs` | `games/plain_tricks/tests/property.rs` | `covered` | |
| `PT-TRICK-002` | Higher same-suit rank wins. | `games/plain_tricks/src/rules.rs` | `games/plain_tricks/tests/property.rs`; replay traces | `covered` | |
| `PT-TRICK-003` | Off-suit follower card loses to leader. | `games/plain_tricks/src/rules.rs` | `games/plain_tricks/tests/property.rs`; replay traces | `covered` | |
| `PT-TRICK-004` | Trick winner leads next trick unless round closes. | `games/plain_tricks/src/rules.rs` | `games/plain_tricks/tests/property.rs` | `covered` | |
| `PT-TURN-001` | Trick starts with current leader. | `games/plain_tricks/src/setup.rs`; `games/plain_tricks/src/rules.rs` | `games/plain_tricks/tests/property.rs`; golden traces | `covered` | |
| `PT-TURN-002` | Follower acts after lead. | `games/plain_tricks/src/rules.rs`; `games/plain_tricks/src/actions.rs` | `games/plain_tricks/tests/property.rs`; golden traces | `covered` | |
| `PT-TURN-003` | Two-card trick resolves in Rust. | `games/plain_tricks/src/rules.rs` | `games/plain_tricks/tests/property.rs`; replay traces | `covered` | |
| `PT-TURN-004` | Non-final trick winner becomes active leader. | `games/plain_tricks/src/rules.rs` | `games/plain_tricks/tests/property.rs` | `covered` | |
| `PT-TURN-005` | Round 1 final trick closes and rotates to round 2. | `games/plain_tricks/src/rules.rs` | `games/plain_tricks/tests/property.rs`; replay traces | `covered` | |
| `PT-TURN-006` | Round 2 final trick resolves terminal outcome. | `games/plain_tricks/src/rules.rs` | terminal golden traces; replay tests | `covered-by-trace` | |
| `PT-TURN-007` | Terminal state has no active gameplay turn. | `games/plain_tricks/src/state.rs`; `games/plain_tricks/src/actions.rs` | terminal golden traces; action-tree checks | `covered-by-trace` | |
| `PT-VAR-001` | Only `plain_tricks_standard` ships. | static data and setup defaults | `games/plain_tricks/tests/serialization.rs`; fixture check | `covered` | |
| `PT-VAR-002` | Public name is Plain Tricks. | docs and static metadata | `games/plain_tricks/tests/serialization.rs`; fixture check | `covered` | |
| `PT-VIS-001` | Public facts are visible to all viewers. | `games/plain_tricks/src/visibility.rs` | `games/plain_tricks/tests/visibility.rs`; public traces | `covered` | |
| `PT-VIS-002` | Unplayed hands are owner-only. | `games/plain_tricks/src/visibility.rs` | `games/plain_tricks/tests/visibility.rs`; no-leak traces | `covered` | |
| `PT-VIS-003` | Played cards become public. | `games/plain_tricks/src/visibility.rs`; `games/plain_tricks/src/effects.rs` | `games/plain_tricks/tests/visibility.rs`; golden traces | `covered` | |
| `PT-VIS-004` | Tail cards are never browser-facing. | `games/plain_tricks/src/visibility.rs`; `games/plain_tricks/src/replay_support.rs` | `games/plain_tricks/tests/replay.rs`; no-leak traces | `covered` | |
| `PT-VIS-005` | Void facts are implicit only. | `games/plain_tricks/src/visibility.rs` | `games/plain_tricks/tests/visibility.rs`; no-leak traces | `covered` | |
| `PT-VIS-006` | Legal card choices are actor-scoped. | `games/plain_tricks/src/actions.rs` | `games/plain_tricks/tests/visibility.rs`; action-tree traces | `covered` | |
| `PT-VIS-007` | Bot rationale and rankings are viewer safe. | `games/plain_tricks/src/bots.rs`; `games/plain_tricks/src/effects.rs` | `games/plain_tricks/tests/bots.rs`; no-leak traces | `covered` | |

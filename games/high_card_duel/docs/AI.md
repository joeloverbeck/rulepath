# High Card Duel AI

Game ID: `high_card_duel`

Rules version: `high-card-duel-rules-v1`

Last updated: 2026-06-07

## Shipped Bot Level

High Card Duel ships a Level 0 random legal bot only. The bot receives the same Rust-owned private input an acting seat may use: its seat, its legal action tree, its own hand, and its own commitment. It does not receive opponent hand identities, deck order, hidden commitments, unrevealed history, or future draws.

## Hidden-Information Boundary

The bot must not serialize private candidate lists, ranked options, belief state, deck order, opponent-card guesses, or hidden diagnostics. Browser-visible bot effects are limited to public-safe action application effects; High Card Duel does not emit a public candidate-ranking explanation.

## Deferred Levels

Level 1 and Level 2 are not shipped for Gate 8. A future authored policy must add strategy evidence, explanation examples, no-leak tests for bot memory/output, and latency benchmarks before public use.

## Verification

- `cargo test -p high_card_duel --test bots`
- `cargo run -p simulate -- --game high_card_duel --games 1000 --start-seed 1`
- Browser no-leak smoke verifies no candidate or hidden token reaches the UI.

# Three Marks AI Notes

Game ID: `three_marks`

Rules version: `three_marks-rules-v1`

Last updated: 2026-06-06

## Policies

| Level | Policy id | Implementation | Determinism | Evidence |
|---|---|---|---|---|
| 0 | `three_marks-random-legal-v1` | `ThreeMarksRandomBot` chooses from Rust legal action tree with deterministic seed. | Same seed and state choose the same legal action. | `level0_choices_validate_for_many_seeds_and_states`; `level0_fixed_seed_is_deterministic_and_terminal_reports_no_action`; bot trace. |
| 1 | `three_marks-priority-v1` | `ThreeMarksLevel1Bot` checks immediate win, block, fork/block-fork, center, opposite corner, corner, side. | Deterministic priority order, no randomness. | Level 1 bot tests; browser smoke bot explanation. |

## Authority boundary

Bots choose an action path; they do not apply state directly. The normal Rust validation/application path still supervises every bot turn. Bot explanations are public semantic effects (`BotChoseAction`) and must not expose candidate rankings, hidden search state, or private data.

## Limitations

- No MCTS, ISMCTS, Monte Carlo tree search, ML, RL, or search-heavy policy is implemented.
- Level 1 is a product-smoke policy, not a solved-game proof.
- Setup and rules remain deterministic; bot seed belongs to the bot layer, not game rules.

## Verification

- `cargo test -p three_marks --test bot_tests`
- `cargo bench -p three_marks -- level0_bot_decision`
- `cargo bench -p three_marks -- level1_bot_decision`
- `apps/web/e2e/three-marks.smoke.mjs` checks that a Rust bot response and public explanation render.

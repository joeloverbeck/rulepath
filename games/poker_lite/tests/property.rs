use engine_core::{
    ActionPath, Actor, CommandEnvelope, DeterministicRng, RulesVersion, SeatId, Seed, SeededRng,
    Viewer,
};
use poker_lite::{
    apply_action, legal_action_tree, project_view, setup_match, validate_command, Phase,
    PokerLiteSeat, PokerLiteState, SetupOptions, TerminalOutcome, ACTION_LIFT,
};

const SAMPLE_SEEDS: u64 = 128;
const ACTION_CAP: usize = 16;

#[derive(Clone, Debug, Eq, PartialEq)]
struct RecordedCommand {
    seat: PokerLiteSeat,
    segment: String,
}

fn setup(seed: u64) -> PokerLiteState {
    setup_match(
        Seed(seed),
        &[SeatId("seat_0".to_owned()), SeatId("seat_1".to_owned())],
        &SetupOptions::default(),
    )
    .expect("setup succeeds")
}

fn actor(state: &PokerLiteState, seat: PokerLiteSeat) -> Actor {
    Actor {
        seat_id: state.seats[seat.index()].clone(),
    }
}

fn command(state: &PokerLiteState, seat: PokerLiteSeat, segment: &str) -> CommandEnvelope {
    CommandEnvelope {
        actor: actor(state, seat),
        action_path: ActionPath {
            segments: vec![segment.to_owned()],
        },
        freshness_token: state.freshness_token,
        rules_version: RulesVersion(1),
    }
}

#[test]
fn seeded_legal_playouts_replay_deterministically() {
    for seed in 0..SAMPLE_SEEDS {
        let (first, stream) = random_legal_playout(seed);
        let second = replay(seed, &stream);

        assert_eq!(
            first.stable_internal_summary(),
            second.stable_internal_summary()
        );
        assert_eq!(first.terminal_outcome, second.terminal_outcome);
        assert_eq!(first.contributions, second.contributions);
        assert_eq!(first.shared_pool, second.shared_pool);
    }
}

#[test]
fn seeded_legal_playouts_terminate_within_action_cap() {
    for seed in 0..SAMPLE_SEEDS {
        let (state, stream) = random_legal_playout(seed);

        assert!(stream.len() <= ACTION_CAP, "seed {seed} exceeded cap");
        assert_eq!(
            state.phase,
            Phase::Terminal,
            "seed {seed} did not terminate"
        );
        assert!(
            state.terminal_outcome.is_some(),
            "seed {seed} missing terminal outcome"
        );
    }
}

#[test]
fn seeded_legal_playouts_preserve_accounting_invariants() {
    for seed in 0..SAMPLE_SEEDS {
        let _ = random_legal_playout(seed);
    }
}

#[test]
fn legal_tree_never_offers_lift_after_round_lift_cap() {
    for seed in 0..SAMPLE_SEEDS {
        let _ = random_legal_playout(seed);
    }
}

#[test]
fn observer_projection_never_leaks_hidden_crests_before_reveal() {
    for seed in 0..SAMPLE_SEEDS {
        let _ = random_legal_playout(seed);
    }
}

fn random_legal_playout(seed: u64) -> (PokerLiteState, Vec<RecordedCommand>) {
    let mut state = setup(seed);
    let mut rng = SeededRng::from_seed(Seed(seed.wrapping_add(10_000)));
    let mut stream = Vec::new();

    assert_invariants(&state);
    while state.phase != Phase::Terminal {
        assert!(
            stream.len() < ACTION_CAP,
            "playout did not terminate by cap"
        );

        let active = state
            .active_seat
            .expect("non-terminal state has active seat");
        let tree = legal_action_tree(&state, &actor(&state, active));
        assert_lift_cap(&state, &tree);
        let index = rng
            .next_index(tree.root.choices.len())
            .expect("non-terminal active seat has legal choices");
        let segment = tree.root.choices[index].segment.clone();
        let envelope = command(&state, active, &segment);
        let action = validate_command(&state, &envelope).expect("legal action validates");

        apply_action(&mut state, action).expect("legal action applies");
        stream.push(RecordedCommand {
            seat: active,
            segment,
        });
        assert_invariants(&state);
    }

    (state, stream)
}

fn replay(seed: u64, stream: &[RecordedCommand]) -> PokerLiteState {
    let mut state = setup(seed);

    assert_invariants(&state);
    for recorded in stream {
        assert_eq!(
            state.active_seat,
            Some(recorded.seat),
            "recorded stream must follow active seat"
        );
        let envelope = command(&state, recorded.seat, &recorded.segment);
        let action = validate_command(&state, &envelope).expect("recorded action validates");
        apply_action(&mut state, action).expect("recorded action applies");
        assert_invariants(&state);
    }

    state
}

fn assert_invariants(state: &PokerLiteState) {
    assert_accounting(state);
    assert_no_hidden_crests_in_observer_projection(state);

    if let Some(active) = state.active_seat {
        let tree = legal_action_tree(state, &actor(state, active));
        assert_lift_cap(state, &tree);
    }
}

fn assert_accounting(state: &PokerLiteState) {
    assert_non_negative(state.contributions[0]);
    assert_non_negative(state.contributions[1]);
    assert_eq!(state.shared_pool, state.contributions.iter().sum::<u8>());
    assert!(state.contributions.iter().all(|amount| *amount <= 7));
}

fn assert_non_negative(_amount: u8) {
    // Contributions are u8; this assertion documents the no-negative invariant.
}

fn assert_lift_cap(state: &PokerLiteState, tree: &engine_core::ActionTree) {
    if state.round.lift_used && state.round.outstanding_actor == state.active_seat {
        assert!(
            tree.root
                .choices
                .iter()
                .all(|choice| choice.segment != ACTION_LIFT),
            "lift offered after round lift cap: {tree:?}"
        );
    }
}

fn assert_no_hidden_crests_in_observer_projection(state: &PokerLiteState) {
    let view = project_view(state, &Viewer { seat_id: None });
    let text = format!("{view:?}");
    let showdown_revealed = matches!(
        state.terminal_outcome,
        Some(TerminalOutcome::ShowdownWin { .. } | TerminalOutcome::Split { .. })
    );

    if !showdown_revealed {
        for card in state.private_cards_internal() {
            assert!(!text.contains(card.as_str()), "{text}");
            if !state.center_visible {
                assert!(!text.contains(&card.label()), "{text}");
            }
        }
    }

    if !state.center_visible {
        let center = state.center_card_internal();
        assert!(!text.contains(center.as_str()), "{text}");
        assert!(!text.contains(&center.label()), "{text}");
    }

    for card in state.deck_tail_internal() {
        assert!(!text.contains(card.as_str()), "{text}");
    }
}

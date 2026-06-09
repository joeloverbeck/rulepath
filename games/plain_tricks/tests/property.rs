use engine_core::{
    ActionPath, Actor, CommandEnvelope, DeterministicRng, RulesVersion, SeatId, Seed, SeededRng,
    Viewer,
};
use plain_tricks::{
    apply_action, legal_action_tree, project_view, setup_match, trick_winner, validate_command,
    CardView, Phase, PlainTricksSeat, PlainTricksState, PrivateView, SetupOptions, TrickCardId,
    TrickPlay, ACTION_PLAY, STANDARD_MAX_PLAYS, STANDARD_TOTAL_TRICKS,
};

const SAMPLE_SEEDS: u64 = 128;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct RecordedCommand {
    seat: PlainTricksSeat,
    card: TrickCardId,
}

fn setup(seed: u64) -> PlainTricksState {
    setup_match(
        Seed(seed),
        &[SeatId("seat_0".to_owned()), SeatId("seat_1".to_owned())],
        &SetupOptions::default(),
    )
    .expect("setup succeeds")
}

fn actor(state: &PlainTricksState, seat: PlainTricksSeat) -> Actor {
    Actor {
        seat_id: state.seats[seat.index()].clone(),
    }
}

fn observer() -> Viewer {
    Viewer { seat_id: None }
}

fn command(state: &PlainTricksState, seat: PlainTricksSeat, card: TrickCardId) -> CommandEnvelope {
    CommandEnvelope {
        actor: actor(state, seat),
        action_path: ActionPath {
            segments: vec![ACTION_PLAY.to_owned(), card.as_str().to_owned()],
        },
        freshness_token: state.freshness_token,
        rules_version: RulesVersion(1),
    }
}

fn seat_viewer(state: &PlainTricksState, seat: PlainTricksSeat) -> Viewer {
    Viewer {
        seat_id: Some(state.seats[seat.index()].clone()),
    }
}

fn own_hand(state: &PlainTricksState, seat: PlainTricksSeat) -> Vec<TrickCardId> {
    let view = project_view(state, &seat_viewer(state, seat));
    let PrivateView::Seat(private) = view.private_view else {
        panic!("seat viewer should receive a seat-private view");
    };
    private.own_hand.iter().map(card_from_view).collect()
}

fn card_from_view(card: &CardView) -> TrickCardId {
    TrickCardId::parse(&card.card_id).expect("view contains a known card")
}

#[test]
fn seeded_legal_playouts_replay_deterministically() {
    for seed in 0..SAMPLE_SEEDS {
        let (first, stream) = random_legal_playout(seed);
        let second = replay(seed, &stream);

        assert_eq!(
            first.stable_internal_summary(),
            second.stable_internal_summary(),
            "seed {seed} replay diverged"
        );
        assert_eq!(
            project_view(&first, &observer()).stable_summary(),
            project_view(&second, &observer()).stable_summary(),
            "seed {seed} observer projection diverged"
        );
    }
}

#[test]
fn seeded_legal_playouts_terminate_after_exactly_twenty_four_plays() {
    for seed in 0..SAMPLE_SEEDS {
        let (state, stream) = random_legal_playout(seed);

        assert_eq!(
            stream.len(),
            STANDARD_MAX_PLAYS as usize,
            "seed {seed} did not use the full two-round command count"
        );
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
fn seeded_legal_playouts_preserve_trick_accounting() {
    for seed in 0..SAMPLE_SEEDS {
        let (state, _) = random_legal_playout(seed);

        assert_eq!(
            state.completed_tricks.len(),
            STANDARD_TOTAL_TRICKS as usize,
            "seed {seed} completed trick count mismatch"
        );
        assert_eq!(
            state.total_trick_counts.seat_0 + state.total_trick_counts.seat_1,
            STANDARD_TOTAL_TRICKS,
            "seed {seed} terminal totals do not conserve tricks"
        );
    }
}

#[test]
fn legal_tree_obeys_follow_suit_and_void_freedom() {
    for seed in 0..SAMPLE_SEEDS {
        let _ = random_legal_playout(seed);
    }
}

#[test]
fn observer_projection_never_leaks_hidden_card_ids() {
    for seed in 0..SAMPLE_SEEDS {
        let _ = random_legal_playout(seed);
    }
}

fn random_legal_playout(seed: u64) -> (PlainTricksState, Vec<RecordedCommand>) {
    let mut state = setup(seed);
    let mut rng = SeededRng::from_seed(Seed(seed.wrapping_add(10_000)));
    let mut stream = Vec::new();
    let mut publicly_played = Vec::new();

    assert_invariants(&state, &publicly_played, seed);
    while state.phase != Phase::Terminal {
        assert!(
            stream.len() < STANDARD_MAX_PLAYS as usize,
            "seed {seed} exceeded the maximum play count"
        );

        let active = state
            .active_seat
            .expect("non-terminal state has active seat");
        let legal_cards = legal_card_choices(&state, active);
        let index = rng
            .next_index(legal_cards.len())
            .expect("non-terminal active seat has legal choices");
        let card = legal_cards[index];
        let envelope = command(&state, active, card);
        let action = validate_command(&state, &envelope).expect("legal command validates");

        apply_action(&mut state, action).expect("legal action applies");
        stream.push(RecordedCommand { seat: active, card });
        publicly_played.push(card);
        assert_invariants(&state, &publicly_played, seed);
    }

    (state, stream)
}

fn replay(seed: u64, stream: &[RecordedCommand]) -> PlainTricksState {
    let mut state = setup(seed);

    for recorded in stream {
        assert_eq!(
            state.active_seat,
            Some(recorded.seat),
            "recorded stream must follow active seat"
        );
        let envelope = command(&state, recorded.seat, recorded.card);
        let action = validate_command(&state, &envelope).expect("recorded command validates");
        apply_action(&mut state, action).expect("recorded action applies");
    }

    state
}

fn assert_invariants(state: &PlainTricksState, publicly_played: &[TrickCardId], seed: u64) {
    assert_accounting(state, seed);
    assert_completed_trick_winners_are_played_seats(state, seed);
    assert_no_hidden_card_ids_in_observer_projection(state, publicly_played, seed);

    if let Some(active) = state.active_seat {
        assert_legal_tree_matches_follow_suit_rule(state, active, seed);
    }
}

fn assert_accounting(state: &PlainTricksState, seed: u64) {
    let round_total = state.round_trick_counts.seat_0 + state.round_trick_counts.seat_1;
    let terminal_total = state.total_trick_counts.seat_0 + state.total_trick_counts.seat_1;
    let completed_in_round = state
        .completed_tricks
        .iter()
        .filter(|trick| trick.round_index == state.round_index)
        .count() as u8;

    assert!(
        round_total <= STANDARD_TOTAL_TRICKS,
        "seed {seed} round total exceeded maximum"
    );
    assert!(
        terminal_total <= STANDARD_TOTAL_TRICKS,
        "seed {seed} terminal total exceeded maximum"
    );
    assert_eq!(
        round_total, completed_in_round,
        "seed {seed} round count does not match completed tricks"
    );
    if state.phase == Phase::Terminal {
        assert_eq!(
            terminal_total, STANDARD_TOTAL_TRICKS,
            "seed {seed} terminal trick total mismatch"
        );
    }
}

fn assert_completed_trick_winners_are_played_seats(state: &PlainTricksState, seed: u64) {
    for trick in &state.completed_tricks {
        let replayed_winner = trick_winner(
            TrickPlay {
                seat: trick.plays[0].seat,
                card: trick.plays[0].card,
            },
            TrickPlay {
                seat: trick.plays[1].seat,
                card: trick.plays[1].card,
            },
        );
        assert!(
            trick.plays.iter().any(|play| play.seat == trick.winner),
            "seed {seed} trick winner was not one of the played seats"
        );
        assert_eq!(
            trick.winner, replayed_winner,
            "seed {seed} trick winner did not match rule replay"
        );
    }
}

fn assert_legal_tree_matches_follow_suit_rule(
    state: &PlainTricksState,
    active: PlainTricksSeat,
    seed: u64,
) {
    let own_hand = own_hand(state, active);
    let offered = legal_card_choices(state, active);

    if state.current_trick.plays.is_empty() {
        assert_same_cards(
            &offered,
            &own_hand,
            seed,
            "leader should be offered the full hand",
        );
        return;
    }

    let led_suit = state
        .current_trick
        .led_suit
        .expect("started trick has led suit");
    let suited = own_hand
        .iter()
        .copied()
        .filter(|card| card.suit() == led_suit)
        .collect::<Vec<_>>();
    if suited.is_empty() {
        assert_same_cards(
            &offered,
            &own_hand,
            seed,
            "follower without led suit should be offered the full hand",
        );
    } else {
        assert_same_cards(
            &offered,
            &suited,
            seed,
            "follower holding led suit should only be offered that suit",
        );
    }
}

fn legal_card_choices(state: &PlainTricksState, seat: PlainTricksSeat) -> Vec<TrickCardId> {
    let tree = legal_action_tree(state, &actor(state, seat));
    let Some(play) = tree.root.choices.first() else {
        return Vec::new();
    };
    assert_eq!(play.segment, ACTION_PLAY);
    play.next
        .as_ref()
        .expect("play family has card choices")
        .choices
        .iter()
        .map(|choice| {
            TrickCardId::parse(&choice.segment).expect("legal card segment parses to known card")
        })
        .collect()
}

fn assert_same_cards(actual: &[TrickCardId], expected: &[TrickCardId], seed: u64, context: &str) {
    let mut actual = actual.to_vec();
    let mut expected = expected.to_vec();
    actual.sort();
    expected.sort();
    assert_eq!(actual, expected, "seed {seed}: {context}");
}

fn assert_no_hidden_card_ids_in_observer_projection(
    state: &PlainTricksState,
    publicly_played: &[TrickCardId],
    seed: u64,
) {
    let text = format!("{:?}", project_view(state, &observer()));

    for card in TrickCardId::ALL {
        if publicly_played.contains(&card) {
            continue;
        }
        assert!(
            !text.contains(card.as_str()),
            "seed {seed} leaked hidden card id {} in {text}",
            card.as_str()
        );
        assert!(
            !text.contains(&format!("{card:?}")),
            "seed {seed} leaked hidden card debug name {card:?} in {text}"
        );
    }
}

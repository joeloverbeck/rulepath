use engine_core::{
    ActionPath, Actor, CommandEnvelope, HashValue, RulesVersion, SeatId, Seed, StableSerialize,
    Viewer,
};
use event_frontier::{
    apply_command, export_public_replay, generate_internal_full_trace, import_public_export,
    import_public_export_json, legal_action_tree, project_view, public_replay_step,
    resolve_reckoning, setup_match, CardId, CardPhase, SetupOptions, ACTION_PASS,
    TRACE_HIDDEN_SURFACE, TRACE_STOCHASTIC_SURFACE,
};

fn seats() -> [SeatId; 2] {
    [SeatId("seat_0".to_owned()), SeatId("seat_1".to_owned())]
}

fn actor(seat: &str) -> Actor {
    Actor {
        seat_id: SeatId(seat.to_owned()),
    }
}

fn observer() -> Viewer {
    Viewer { seat_id: None }
}

fn pass_command(seat: &str, state: &event_frontier::EventFrontierState) -> CommandEnvelope {
    CommandEnvelope {
        actor: actor(seat),
        action_path: ActionPath {
            segments: vec![ACTION_PASS.to_owned()],
        },
        freshness_token: state.freshness_token,
        rules_version: RulesVersion(1),
    }
}

fn debug_hash<T: std::fmt::Debug>(value: &T) -> HashValue {
    HashValue::from_stable_bytes(format!("{value:?}").as_bytes())
}

fn scenario_options() -> [SetupOptions; 3] {
    [
        SetupOptions::standard(),
        SetupOptions::hard_winter(),
        SetupOptions::land_rush(),
    ]
}

#[test]
fn deterministic_setup_reproduces_deck_order_and_state_hash() {
    let seats = seats();

    for options in scenario_options() {
        let first = setup_match(Seed(99), &seats, &options).expect("first setup");
        let second = setup_match(Seed(99), &seats, &options).expect("second setup");

        assert_eq!(first.deck, second.deck);
        assert_eq!(first.stable_hash(), second.stable_hash());
        assert_eq!(first.stable_summary(), second.stable_summary());
    }
}

#[test]
fn reckoning_is_never_first_in_any_seeded_epoch() {
    let seats = seats();

    for options in scenario_options() {
        for seed in 0..150 {
            let state = setup_match(Seed(seed), &seats, &options).expect("setup");
            let mut deck = Vec::new();
            deck.extend(state.deck.current);
            deck.extend(state.deck.next_public);
            deck.extend(state.deck.undrawn);

            for epoch_start in [0, 7, 14] {
                assert!(!is_reckoning(deck[epoch_start]));
            }
        }
    }
}

fn is_reckoning(card: CardId) -> bool {
    matches!(
        card,
        CardId::ReckoningOne | CardId::ReckoningTwo | CardId::ReckoningThree
    )
}

#[test]
fn reckoning_breakdown_scores_and_terminal_reproduce_for_same_state() {
    let seats = seats();
    let mut first = setup_match(Seed(1), &seats, &SetupOptions::default()).expect("setup");
    let mut second = first.clone();
    for state in [&mut first, &mut second] {
        state.deck.current = Some(CardId::ReckoningOne);
        state.card_phase = CardPhase::Reckoning;
    }

    let first_result = resolve_reckoning(&mut first).expect("first reckoning");
    let second_result = resolve_reckoning(&mut second).expect("second reckoning");

    assert_eq!(first.scores, second.scores);
    assert_eq!(first.terminal_outcome, second.terminal_outcome);
    assert_eq!(
        format!("{:?}", first_result.effects),
        format!("{:?}", second_result.effects)
    );
    assert_eq!(first.stable_hash(), second.stable_hash());
}

#[test]
fn public_replay_export_import_reproduces_public_hashes_without_hidden_order() {
    let seats = seats();
    let mut state = setup_match(Seed(1), &seats, &SetupOptions::default()).expect("setup");
    let hidden = state.deck.undrawn[0].as_str().to_owned();
    let command = pass_command("seat_1", &state);

    let applied = apply_command(&mut state, &command).expect("pass command applies");
    let step = public_replay_step(0, &state, &command, &applied.effects, &observer());
    let export = export_public_replay(state.variant.id.clone(), &observer(), vec![step]);

    let imported_from_struct = import_public_export(&export);
    let imported_from_json = import_public_export_json(&export.to_json()).expect("public import");

    assert_eq!(
        imported_from_struct.stable_hash(),
        imported_from_json.stable_hash()
    );
    assert_eq!(imported_from_struct.raw_json, export.to_json());
    assert_eq!(imported_from_json.raw_json, export.to_json());
    assert!(!export.stable_summary().contains(&hidden));
    assert!(!export.to_json().contains(&hidden));
    assert!(!imported_from_json.stable_summary().contains(&hidden));
}

#[test]
fn replaying_same_seed_scenario_and_command_stream_reproduces_public_hashes() {
    let seats = seats();
    let mut first = setup_match(Seed(1), &seats, &SetupOptions::default()).expect("first setup");
    let mut second = setup_match(Seed(1), &seats, &SetupOptions::default()).expect("second setup");
    let first_command = pass_command("seat_1", &first);
    let second_command = pass_command("seat_1", &second);

    let first_applied = apply_command(&mut first, &first_command).expect("first command applies");
    let second_applied =
        apply_command(&mut second, &second_command).expect("second command applies");

    assert_eq!(first.stable_hash(), second.stable_hash());
    assert_eq!(
        debug_hash(&first_applied.effects),
        debug_hash(&second_applied.effects)
    );
    assert_eq!(
        debug_hash(&legal_action_tree(&first, &actor("seat_0"))),
        debug_hash(&legal_action_tree(&second, &actor("seat_0")))
    );
    assert_eq!(
        project_view(&first, &observer()).stable_hash(),
        project_view(&second, &observer()).stable_hash()
    );
}

#[test]
fn internal_trace_marks_hidden_and_stochastic_surfaces() {
    let seats = seats();
    let state = setup_match(Seed(1), &seats, &SetupOptions::default()).expect("setup");
    let hidden = state.deck.undrawn[0].as_str();

    let trace = generate_internal_full_trace(1, &state);

    assert_eq!(trace.hidden_surface, TRACE_HIDDEN_SURFACE);
    assert_eq!(trace.stochastic_surface, TRACE_STOCHASTIC_SURFACE);
    assert_eq!(trace.per_seat_hidden_surface, "not_applicable");
    assert!(trace.full_deck_order.iter().any(|card| card == hidden));
}

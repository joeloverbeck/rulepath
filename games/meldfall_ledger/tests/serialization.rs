use engine_core::{FreshnessToken, Seed, VisibilityScope};
use meldfall_ledger::{
    actions::{
        draw_action_tree, progressive_turn_tree, table_action_tree, LayoffPosition, MeldfallAction,
    },
    cards::{Card, Rank, Suit},
    effects::{
        effect_stable_string, public_effect, DrawSource, LayoffEffectPosition, MeldfallEffect,
    },
    replay_support::{replay_skeleton_record, TRACE_SCHEMA_VERSION},
    scoring::apply_round_deltas,
    setup::{default_seats, setup_match, SetupOptions},
    state::{
        LastSettlementSeatSnapshot, LastSettlementSnapshot, MatchOutcome, MatchState, MeldId,
        MeldKind, MeldTableau, SeatStanding, TableCard, TurnOrdinal,
    },
};

fn sample_state() -> MatchState {
    let seats = default_seats(4).expect("supported seats");
    let setup = setup_match(Seed(19), &seats, &SetupOptions::default()).expect("setup succeeds");
    MatchState::from_initial_setup(setup)
}

fn ace_clubs() -> meldfall_ledger::cards::CardId {
    Card::new(Rank::Ace, Suit::Clubs).id()
}

fn two_clubs() -> meldfall_ledger::cards::CardId {
    Card::new(Rank::Two, Suit::Clubs).id()
}

fn three_clubs() -> meldfall_ledger::cards::CardId {
    Card::new(Rank::Three, Suit::Clubs).id()
}

#[test]
fn match_state_summary_has_stable_field_order() {
    let mut state = sample_state();
    state.round.tableau = MeldTableau {
        groups: vec![meldfall_ledger::state::MeldGroup {
            id: MeldId(7),
            kind: MeldKind::Run { suit: Suit::Clubs },
            origin_seat: 1,
            cards: vec![TableCard {
                card: ace_clubs(),
                played_by: 1,
                score_credit_owner: 1,
                play_turn: TurnOrdinal(2),
            }],
        }],
    };
    state.terminal = Some(MatchOutcome {
        winner: Some(1),
        standings: vec![
            SeatStanding {
                seat_index: 1,
                cumulative_score: 505,
                latest_round_delta: 40,
                rank: 1,
                winner: true,
            },
            SeatStanding {
                seat_index: 0,
                cumulative_score: 410,
                latest_round_delta: -5,
                rank: 2,
                winner: false,
            },
        ],
    });

    let summary = state.stable_internal_summary();

    assert!(summary.starts_with(
        "match|variant=classic_500_single_deck_v1|seats=[seat_0,seat_1,seat_2,seat_3]|scores=[0,0,0,0]|dealer=0|round_index=0|round=round|active=1|phase=draw|"
    ));
    assert!(summary.contains("|round_index=0|"));
    assert!(
        summary.contains("meld_7:run:clubs:origin=1:cards=[ace_clubs:played_by=1:credit=1:turn=2]")
    );
    assert!(summary.ends_with("terminal=winner=1:standings=[1:505:40:1:true,0:410:-5:2:false]"));
}

#[test]
fn retained_last_settlement_is_excluded_from_stable_internal_summary() {
    let without_snapshot = sample_state();
    let mut with_snapshot = without_snapshot.clone();
    with_snapshot.last_settlement = Some(LastSettlementSnapshot {
        round_index: 0,
        round_end_reason: "go_out_by_final_discard:seat=2".to_owned(),
        seats: vec![LastSettlementSeatSnapshot {
            seat_index: 2,
            tabled_positive: 40,
            in_hand_penalty: 3,
            remaining_hand_count: 1,
            round_delta: 37,
            cumulative_score: 115,
            rank: 1,
            winner: false,
        }],
    });

    assert_eq!(
        without_snapshot.stable_internal_summary(),
        with_snapshot.stable_internal_summary()
    );
    assert_ne!(without_snapshot, with_snapshot);
}

#[test]
fn action_paths_and_action_trees_are_deterministic() {
    let meld = MeldfallAction::MeldNew {
        cards: vec![ace_clubs(), two_clubs(), three_clubs()],
    };
    let lay_off = MeldfallAction::LayOff {
        card: three_clubs(),
        target_meld: MeldId(2),
        position: LayoffPosition::Append,
    };
    let discard = MeldfallAction::Discard { card: ace_clubs() };

    assert_eq!(
        meld.action_path().segments,
        vec!["meld-new-ace_clubs_two_clubs_three_clubs"]
    );
    assert_eq!(
        lay_off.action_path().segments,
        vec!["lay-off-three_clubs-meld_2-append"]
    );
    assert_eq!(discard.action_path().segments, vec!["discard-ace_clubs"]);

    let draw_tree = draw_action_tree(FreshnessToken(5), true, &[0, 3]);
    assert_eq!(
        draw_tree
            .root
            .choices
            .iter()
            .map(|choice| choice.segment.as_str())
            .collect::<Vec<_>>(),
        vec!["draw-stock", "draw-discard-0", "draw-discard-3"]
    );
    assert_eq!(draw_tree.root.choices[1].metadata[2].value, "0");

    let table_tree = table_action_tree(
        FreshnessToken(6),
        vec![meld.clone()],
        vec![lay_off],
        vec![discard],
        true,
    );
    assert_eq!(
        table_tree
            .root
            .choices
            .iter()
            .map(|choice| choice.segment.as_str())
            .collect::<Vec<_>>(),
        vec![
            "meld-new-ace_clubs_two_clubs_three_clubs",
            "lay-off-three_clubs-meld_2-append",
            "discard-ace_clubs",
            "go-out-without-discard",
            "finish-turn"
        ]
    );

    let progressive = progressive_turn_tree(
        FreshnessToken(7),
        vec![MeldfallAction::DrawFromStock],
        vec![meld],
    );
    assert_eq!(progressive.root.choices[0].segment, "draw-stock");
    assert_eq!(
        progressive.root.choices[0]
            .next
            .as_ref()
            .expect("draw has child node")
            .choices[0]
            .segment,
        "meld-new-ace_clubs_two_clubs_three_clubs"
    );
}

#[test]
fn effect_groups_have_public_envelopes_and_stable_strings() {
    let table_card = TableCard {
        card: ace_clubs(),
        played_by: 0,
        score_credit_owner: 0,
        play_turn: TurnOrdinal(1),
    };
    let effects = [
        public_effect(MeldfallEffect::Draw {
            seat: 0,
            source: DrawSource::Stock,
            cards_moved: 1,
            stock_count_after: 28,
            discard_count_after: 1,
        }),
        public_effect(MeldfallEffect::Meld {
            seat: 0,
            meld_id: MeldId(1),
            cards: vec![table_card.clone()],
        }),
        public_effect(MeldfallEffect::LayOff {
            seat: 1,
            meld_id: MeldId(1),
            card: table_card,
            position: LayoffEffectPosition::Append,
        }),
        public_effect(MeldfallEffect::Discard {
            seat: 0,
            card: two_clubs(),
            discard_count_after: 2,
        }),
    ];

    assert!(effects
        .iter()
        .all(|effect| effect.visibility == VisibilityScope::Public));
    assert_eq!(
        effects.iter().map(effect_stable_string).collect::<Vec<_>>(),
        vec![
            "Draw:seat=0:source=stock:cards=1:stock_after=28:discard_after=1",
            "Meld:seat=0:meld=meld_1:cards=[ace_clubs:played_by=0:credit=0:turn=1]",
            "LayOff:seat=1:meld=meld_1:card=ace_clubs:played_by=0:credit=0:turn=1:position=append",
            "Discard:seat=0:card=two_clubs:discard_after=2",
        ]
    );
}

#[test]
fn replay_skeleton_record_uses_trace_schema_v1_and_stable_labels() {
    let state = sample_state();
    let action_tree = draw_action_tree(FreshnessToken(9), true, &[0]);
    let effects = vec![public_effect(MeldfallEffect::RoundScore {
        round_index: 1,
        deltas: vec![20, -5, 0, 10],
        cumulative_scores: vec![20, -5, 0, 10],
    })];

    let record = replay_skeleton_record(&state, &action_tree, &effects);

    assert_eq!(record.schema_version, TRACE_SCHEMA_VERSION);
    assert_eq!(
        record.stable_string(),
        replay_skeleton_record(&state, &action_tree, &effects).stable_string()
    );
    assert!(record.stable_string().contains(
        "replay|schema=1|export=2|game=meldfall_ledger|rules=meldfall-ledger-rules-v1|data=meldfall-ledger-data-v1|action_encoding=action_tree_v1|"
    ));
}

#[test]
fn viewer_export_surfaces_have_stable_discard_tableau_hand_score_and_effect_order() {
    let mut state = sample_state();
    state.round.discard = vec![ace_clubs(), two_clubs(), three_clubs()];
    state.round.tableau = MeldTableau {
        groups: vec![meldfall_ledger::state::MeldGroup {
            id: MeldId(3),
            kind: MeldKind::Run { suit: Suit::Clubs },
            origin_seat: 0,
            cards: vec![
                TableCard {
                    card: ace_clubs(),
                    played_by: 0,
                    score_credit_owner: 0,
                    play_turn: TurnOrdinal(1),
                },
                TableCard {
                    card: two_clubs(),
                    played_by: 1,
                    score_credit_owner: 1,
                    play_turn: TurnOrdinal(2),
                },
            ],
        }],
    };
    state.round.seats[0].hand = vec![three_clubs(), Card::new(Rank::Four, Suit::Clubs).id()];
    apply_round_deltas(&mut state, &[3, -2, 0, 10], &[3, 0, 0, 10], &[0, 2, 0, 0]);

    let view = meldfall_ledger::visibility::project_view(
        &state,
        &engine_core::Viewer {
            seat_id: Some(state.seats[0].clone()),
        },
    );
    let stable = view.stable_string();

    assert!(stable.contains("discard=[ace_clubs,two_clubs,three_clubs]"));
    assert!(stable.contains("meld_3:run:clubs:origin=0:cards=[ace_clubs:played_by=0:credit=0:turn=1,two_clubs:played_by=1:credit=1:turn=2]"));
    assert!(stable.contains("private=seat=0:hand=[three_clubs,four_clubs]"));
    assert!(stable.contains("scores=[3,-2,0,10]"));
    assert_eq!(
        stable,
        meldfall_ledger::visibility::project_view(
            &state,
            &engine_core::Viewer {
                seat_id: Some(state.seats[0].clone()),
            },
        )
        .stable_string()
    );
}

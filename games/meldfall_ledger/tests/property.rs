use std::collections::BTreeSet;

use engine_core::{FreshnessToken, Seed, Viewer};
use meldfall_ledger::{
    actions::draw_source_action_tree,
    cards::{canonical_deck, Card, CardId, Rank, Suit},
    rules::{
        discard_card, draw_from_stock, finish_turn_after_table_plays, take_new_meld_from_hand,
        validate_new_meld,
    },
    scoring::{card_score, settle_round},
    setup::{default_seats, setup_match, SetupOptions},
    state::{
        MatchState, MeldId, MeldKind, MeldTableau, SeatIndex, TableCard, TurnOrdinal, TurnPhase,
    },
    visibility::{project_action_tree_for_viewer, project_view},
};

#[test]
fn generated_same_rank_sets_always_validate_locally() {
    for rank in Rank::ALL {
        let cards = Suit::ALL
            .iter()
            .take(3)
            .copied()
            .map(|suit| Card::new(rank, suit).id())
            .collect::<Vec<_>>();

        assert_eq!(
            validate_new_meld(&cards).expect("generated set validates"),
            MeldKind::Set { rank }
        );
    }
}

#[test]
fn generated_same_suit_runs_always_validate_locally() {
    for suit in Suit::ALL {
        for ranks in Rank::ALL[1..].windows(3) {
            let cards = ranks
                .iter()
                .copied()
                .map(|rank| Card::new(rank, suit).id())
                .collect::<Vec<_>>();

            assert_eq!(
                validate_new_meld(&cards).expect("generated run validates"),
                MeldKind::Run { suit }
            );
        }

        let ace_low = cards(&[Rank::Ace, Rank::Two, Rank::Three], suit);
        let ace_high = cards(&[Rank::Queen, Rank::King, Rank::Ace], suit);
        assert_eq!(
            validate_new_meld(&ace_low).expect("ace-low run validates"),
            MeldKind::Run { suit }
        );
        assert_eq!(
            validate_new_meld(&ace_high).expect("ace-high run validates"),
            MeldKind::Run { suit }
        );
    }
}

#[test]
fn taking_generated_legal_melds_preserves_card_ownership_conservation() {
    for (index, meld_cards) in generated_legal_melds().into_iter().enumerate() {
        let spare = Card::new(Rank::King, Suit::Spades).id();
        if meld_cards.contains(&spare) {
            continue;
        }
        let before_len = meld_cards.len() + 1;
        let mut hand = meld_cards.clone();
        hand.push(spare);

        let group = take_new_meld_from_hand(
            &mut hand,
            &meld_cards,
            MeldId(index as u32),
            0,
            TurnOrdinal(index as u32),
        )
        .expect("generated legal meld is accepted from owned hand");

        assert_eq!(hand, vec![spare]);
        assert_eq!(hand.len() + group.cards.len(), before_len);
        assert_eq!(
            group
                .cards
                .iter()
                .map(|table| table.card)
                .collect::<Vec<_>>(),
            meld_cards
        );
    }
}

#[test]
fn setup_decks_are_unique_and_public_counts_sum_to_52() {
    for seat_count in 2..=6 {
        let state = setup_state(seed_for_seats(seat_count), seat_count);
        assert_conserved_unique_cards(&state);

        let public = project_view(&state, &Viewer { seat_id: None });
        assert_eq!(
            public.hand_counts.iter().sum::<usize>(),
            total_private_cards(&state)
        );
        assert_eq!(
            public.hand_counts.iter().sum::<usize>() + public.discard.len() + public.stock_count,
            52
        );
    }
}

#[test]
fn legal_apply_sequence_does_not_panic_and_preserves_ownership() {
    let mut state = setup_state(1930, 4);
    state.round.active_seat_index = 0;
    state.round.phase = TurnPhase::Draw;
    state.round.stock = vec![card(Rank::Nine, Suit::Hearts)];
    state.round.discard = vec![card(Rank::Four, Suit::Clubs)];
    state.round.seats[0].hand = vec![card(Rank::Two, Suit::Clubs), card(Rank::Three, Suit::Clubs)];
    state.round.seats[1].hand = vec![card(Rank::Five, Suit::Diamonds)];
    state.round.seats[2].hand = vec![card(Rank::Six, Suit::Hearts)];
    state.round.seats[3].hand = vec![card(Rank::Seven, Suit::Spades)];

    draw_from_stock(&mut state.round, 0).expect("legal stock draw applies");
    assert_conserved_unique_cards(&state);
    finish_turn_after_table_plays(&mut state.round, 0).expect("legal finish enters discard phase");
    assert_conserved_unique_cards(&state);
    discard_card(&mut state.round, 0, card(Rank::Nine, Suit::Hearts))
        .expect("legal discard applies");
    assert_conserved_unique_cards(&state);
}

#[test]
fn score_deltas_equal_card_value_accounting() {
    let mut state = setup_state(1931, 4);
    state.round.tableau = MeldTableau {
        groups: vec![meldfall_ledger::state::MeldGroup {
            id: MeldId(0),
            kind: MeldKind::Run { suit: Suit::Clubs },
            origin_seat: 0,
            cards: vec![
                table_card(card(Rank::Ace, Suit::Clubs), 0, 0, 1),
                table_card(card(Rank::Two, Suit::Clubs), 0, 0, 1),
                table_card(card(Rank::Three, Suit::Clubs), 0, 0, 1),
                table_card(card(Rank::Four, Suit::Clubs), 1, 1, 2),
            ],
        }],
    };
    state.round.seats[0].hand = vec![card(Rank::King, Suit::Spades)];
    state.round.seats[1].hand = vec![
        card(Rank::Five, Suit::Diamonds),
        card(Rank::Six, Suit::Diamonds),
    ];
    state.round.seats[2].hand.clear();
    state.round.seats[3].hand = vec![card(Rank::Ace, Suit::Hearts)];

    let settlement = settle_round(&mut state);

    for seat in &settlement.seats {
        let expected =
            tabled_total_for(&state, seat.seat_index) - hand_penalty_for(&state, seat.seat_index);
        assert_eq!(seat.round_delta, expected);
        assert_eq!(seat.cumulative_score, expected);
    }
}

#[test]
fn redacted_views_and_action_trees_never_expose_hidden_cards() {
    let state = setup_state(1932, 6);
    let tree = draw_source_action_tree(FreshnessToken(19), &state.round);

    for viewer_seat in [None, Some(0), Some(1), Some(2), Some(3), Some(4), Some(5)] {
        let viewer = Viewer {
            seat_id: viewer_seat.map(meldfall_ledger::seat_id_for_index),
        };
        let surface = format!(
            "{:?}|{:?}",
            project_view(&state, &viewer),
            project_action_tree_for_viewer(&tree, &state, &viewer)
        );
        for hidden in hidden_for_viewer(&state, viewer_seat) {
            assert!(
                !surface.contains(&hidden.as_str()),
                "viewer {viewer_seat:?} leaked hidden card {}",
                hidden.as_str()
            );
        }
    }
}

fn generated_legal_melds() -> Vec<Vec<CardId>> {
    let mut melds = Vec::new();
    for rank in Rank::ALL {
        melds.push(
            Suit::ALL
                .iter()
                .take(3)
                .copied()
                .map(|suit| Card::new(rank, suit).id())
                .collect(),
        );
    }
    for suit in Suit::ALL {
        for ranks in Rank::ALL[1..].windows(3) {
            melds.push(cards(ranks, suit));
        }
        melds.push(cards(&[Rank::Ace, Rank::Two, Rank::Three], suit));
        melds.push(cards(&[Rank::Queen, Rank::King, Rank::Ace], suit));
    }
    melds
}

fn cards(ranks: &[Rank], suit: Suit) -> Vec<CardId> {
    ranks
        .iter()
        .copied()
        .map(|rank| Card::new(rank, suit).id())
        .collect()
}

fn card(rank: Rank, suit: Suit) -> CardId {
    Card::new(rank, suit).id()
}

fn setup_state(seed: u64, seat_count: usize) -> MatchState {
    let seats = default_seats(seat_count).expect("seat count supported");
    let setup = setup_match(Seed(seed), &seats, &SetupOptions::default()).expect("setup succeeds");
    MatchState::from_initial_setup(setup)
}

fn seed_for_seats(seat_count: usize) -> u64 {
    1900 + seat_count as u64
}

fn total_private_cards(state: &MatchState) -> usize {
    state.round.seats.iter().map(|seat| seat.hand.len()).sum()
}

fn assert_conserved_unique_cards(state: &MatchState) {
    let all = all_cards_in_zones(state);
    let unique = all.iter().copied().collect::<BTreeSet<_>>();
    assert_eq!(unique.len(), all.len(), "no card may be in two zones");
    assert!(all.len() <= canonical_deck().len());
}

fn all_cards_in_zones(state: &MatchState) -> Vec<CardId> {
    let mut cards = Vec::new();
    cards.extend(state.round.stock.iter().copied());
    cards.extend(state.round.discard.iter().copied());
    for seat in &state.round.seats {
        cards.extend(seat.hand.iter().copied());
    }
    for group in &state.round.tableau.groups {
        cards.extend(group.cards.iter().map(|card| card.card));
    }
    cards
}

fn table_card(
    card: CardId,
    played_by: SeatIndex,
    score_credit_owner: SeatIndex,
    turn: u32,
) -> TableCard {
    TableCard {
        card,
        played_by,
        score_credit_owner,
        play_turn: TurnOrdinal(turn),
    }
}

fn tabled_total_for(state: &MatchState, seat_index: SeatIndex) -> i32 {
    state
        .round
        .tableau
        .groups
        .iter()
        .flat_map(|group| group.cards.iter())
        .filter(|table| table.score_credit_owner == seat_index)
        .map(|table| card_score(table.card))
        .sum()
}

fn hand_penalty_for(state: &MatchState, seat_index: SeatIndex) -> i32 {
    state.round.seats[seat_index]
        .hand
        .iter()
        .copied()
        .map(card_score)
        .sum()
}

fn hidden_for_viewer(state: &MatchState, viewer_seat: Option<SeatIndex>) -> Vec<CardId> {
    let mut hidden = state.round.stock.clone();
    for (seat_index, seat) in state.round.seats.iter().enumerate() {
        if viewer_seat != Some(seat_index) {
            hidden.extend(seat.hand.iter().copied());
        }
    }
    hidden
}

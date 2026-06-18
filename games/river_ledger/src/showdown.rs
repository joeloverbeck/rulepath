use std::{cmp::Ordering, collections::BTreeSet};

use crate::{
    cards::{Card, Rank},
    evaluator::{best_five_from_seven, compare_evaluations, HandCategory, HandEvaluation},
    ids::RiverLedgerSeat,
    pot::{allocate_single_pot, PotAllocation},
    state::{
        CategoryLadderPosition, RiverLedgerShowdownPresentationV2, RiverLedgerState, SeatStatus,
        ShowdownBoardCardPresentation, ShowdownCardUsageMark, ShowdownDecisiveReason,
        ShowdownDetailRow, ShowdownFoldedRowPresentation, ShowdownResultBanner, ShowdownReveal,
        ShowdownSeatExplanation, ShowdownStandingPresentation, TerminalOutcome,
    },
    ui::seat_public_label,
};

#[derive(Clone, Debug, Eq, PartialEq)]
struct SeatEvaluation {
    seat: RiverLedgerSeat,
    evaluation: HandEvaluation,
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct ResolvedShowdown {
    evaluations: Vec<SeatEvaluation>,
    canonical_winners: Vec<RiverLedgerSeat>,
    allocation: PotAllocation,
    headline: String,
    decisive_comparison: String,
    comparison_basis: String,
    explanations: Vec<ShowdownSeatExplanation>,
    presentation_v2: RiverLedgerShowdownPresentationV2,
}

impl ResolvedShowdown {
    fn into_terminal(self) -> TerminalOutcome {
        self.assert_invariants();
        TerminalOutcome::Showdown {
            winners: self.canonical_winners,
            pot_total: self.allocation.pot_total,
            allocations: self.allocation.shares,
            headline: self.headline,
            decisive_comparison: self.decisive_comparison,
            comparison_basis: self.comparison_basis,
            explanations: self.explanations,
            presentation_v2: Box::new(self.presentation_v2),
        }
    }

    fn assert_invariants(&self) {
        assert!(
            !self.canonical_winners.is_empty(),
            "resolved showdown requires at least one canonical winner"
        );
        let winner_set = self
            .canonical_winners
            .iter()
            .copied()
            .collect::<BTreeSet<_>>();
        assert_eq!(
            winner_set.len(),
            self.canonical_winners.len(),
            "resolved showdown winners must be unique"
        );
        assert_eq!(
            self.allocation.winners, self.canonical_winners,
            "allocation winner order must match canonical showdown winner order"
        );
        assert_eq!(
            self.allocation
                .remainder_order
                .iter()
                .copied()
                .collect::<BTreeSet<_>>(),
            winner_set,
            "remainder order must contain exactly the canonical winner set"
        );
        assert_eq!(
            self.allocation.remainder_order.len(),
            self.canonical_winners.len(),
            "remainder order must not duplicate winners"
        );
        assert_eq!(
            self.allocation
                .shares
                .iter()
                .map(|share| share.seat)
                .collect::<Vec<_>>(),
            self.canonical_winners,
            "allocation shares must serialize in canonical winner order"
        );
        assert_eq!(
            self.allocation
                .shares
                .iter()
                .map(|share| share.amount)
                .sum::<u16>(),
            self.allocation.pot_total,
            "showdown allocations must conserve the ledger"
        );
        assert!(
            self.allocation
                .shares
                .iter()
                .all(|share| winner_set.contains(&share.seat) && share.amount > 0),
            "only winners may receive positive showdown allocations"
        );
        assert!(
            self.evaluations
                .iter()
                .filter(|entry| winner_set.contains(&entry.seat))
                .count()
                == self.canonical_winners.len(),
            "each canonical winner must have one evaluation"
        );
        assert!(
            self.presentation_v2.standings.iter().all(|standing| {
                let winner = winner_set.contains(&standing.seat);
                standing.default_expanded == winner
                    && standing.result_label == result_label(standing.seat, &self.canonical_winners)
            }),
            "V2 standings winner flags must match canonical winners"
        );

        if self.canonical_winners.len() == 1 {
            let winner = self.canonical_winners[0];
            assert!(
                self.headline.contains(&seat_public_label(winner))
                    && self
                        .presentation_v2
                        .result_banner
                        .headline
                        .contains(&seat_public_label(winner)),
                "single-winner banner identity must match canonical winner"
            );
        } else {
            assert!(
                self.headline.contains("split the ledger")
                    && self
                        .presentation_v2
                        .result_banner
                        .headline
                        .contains("split the ledger"),
                "split showdown banner must not imply a sole winner"
            );
            for winner in &self.canonical_winners {
                assert!(
                    self.headline.contains(&seat_public_label(*winner)),
                    "split showdown headline must name every canonical winner"
                );
            }
            for loser in self
                .evaluations
                .iter()
                .map(|entry| entry.seat)
                .filter(|seat| !winner_set.contains(seat))
            {
                assert!(
                    !self.headline.contains(&seat_public_label(loser)),
                    "split showdown headline must not name a loser as a winner"
                );
            }
        }
    }
}

pub fn showdown_eligible_seats(state: &RiverLedgerState) -> Vec<RiverLedgerSeat> {
    state
        .ledger
        .seats
        .iter()
        .filter(|entry| {
            matches!(
                entry.status,
                SeatStatus::Live | SeatStatus::ShowdownEligible
            )
        })
        .map(|entry| entry.seat)
        .collect()
}

pub fn resolve_showdown(state: &RiverLedgerState) -> TerminalOutcome {
    resolve_showdown_internal(state).into_terminal()
}

fn resolve_showdown_internal(state: &RiverLedgerState) -> ResolvedShowdown {
    let evaluations = evaluate_showdown_seats(state);
    let canonical_winners = winning_seats(&evaluations);
    let allocation = allocate_single_pot(
        state.ledger.pot_total,
        &canonical_winners,
        state.button,
        state.seats.len() as u8,
    );
    let headline = showdown_headline(&evaluations, &canonical_winners);
    let decisive_comparison = decisive_comparison(&evaluations, &canonical_winners);
    let comparison_basis = comparison_basis(&evaluations, &canonical_winners);
    let explanations = explain_showdown(state, &evaluations, &canonical_winners, &allocation);
    let presentation_v2 = showdown_presentation_v2(
        state,
        &evaluations,
        &canonical_winners,
        &allocation,
        &headline,
        &decisive_comparison,
        &comparison_basis,
    );

    ResolvedShowdown {
        evaluations,
        canonical_winners,
        allocation,
        headline,
        decisive_comparison,
        comparison_basis,
        explanations,
        presentation_v2,
    }
}

fn evaluate_showdown_seats(state: &RiverLedgerState) -> Vec<SeatEvaluation> {
    assert_eq!(
        state.board.len(),
        5,
        "showdown evaluation requires a complete public board"
    );

    showdown_eligible_seats(state)
        .into_iter()
        .map(|seat| {
            let hand = state
                .private_hand_for_internal(seat)
                .expect("eligible seat has private hand");
            let seven = [
                hand[0],
                hand[1],
                state.board[0],
                state.board[1],
                state.board[2],
                state.board[3],
                state.board[4],
            ];
            SeatEvaluation {
                seat,
                evaluation: best_five_from_seven(seven),
            }
        })
        .collect()
}

fn winning_seats(evaluations: &[SeatEvaluation]) -> Vec<RiverLedgerSeat> {
    let best = evaluations
        .iter()
        .map(|entry| &entry.evaluation)
        .max_by(|left, right| compare_evaluations(left, right))
        .expect("showdown requires at least one eligible seat");

    evaluations
        .iter()
        .filter(|entry| compare_evaluations(&entry.evaluation, best) == Ordering::Equal)
        .map(|entry| entry.seat)
        .collect()
}

fn explain_showdown(
    state: &RiverLedgerState,
    evaluations: &[SeatEvaluation],
    winners: &[RiverLedgerSeat],
    allocation: &PotAllocation,
) -> Vec<ShowdownSeatExplanation> {
    let closest_challenger = closest_challenger(evaluations, winners);
    let single_winner = single_winner_evaluation(evaluations, winners);

    state
        .ledger
        .seats
        .iter()
        .map(|ledger| {
            if let Some(entry) = evaluations.iter().find(|entry| entry.seat == ledger.seat) {
                let share = allocation
                    .shares
                    .iter()
                    .find(|share| share.seat == ledger.seat)
                    .map(|share| share.amount)
                    .unwrap_or(0);
                ShowdownSeatExplanation {
                    seat: ledger.seat,
                    status: ledger.status,
                    revealed: Some(reveal_for(
                        state,
                        entry,
                        winners,
                        single_winner,
                        closest_challenger,
                    )),
                    summary: format!(
                        "{} reached showdown with {}; tie_break={:?}; allocated={}; total_contribution={}",
                        seat_public_label(ledger.seat),
                        entry.evaluation.category.as_str(),
                        entry.evaluation.tie_break_vector,
                        share,
                        ledger.total_contribution
                    ),
                }
            } else {
                ShowdownSeatExplanation {
                    seat: ledger.seat,
                    status: ledger.status,
                    revealed: None,
                    summary: format!(
                        "{} folded before showdown; allocated=0; total_contribution={}",
                        seat_public_label(ledger.seat),
                        ledger.total_contribution
                    ),
                }
            }
        })
        .collect()
}

fn reveal_for(
    state: &RiverLedgerState,
    entry: &SeatEvaluation,
    winners: &[RiverLedgerSeat],
    single_winner: Option<&SeatEvaluation>,
    closest_challenger: Option<&SeatEvaluation>,
) -> ShowdownReveal {
    let hand_name = hand_name(&entry.evaluation);
    ShowdownReveal {
        seat: entry.seat,
        hole_cards: state
            .private_hand_for_internal(entry.seat)
            .expect("showdown entry has a private hand"),
        best_five: entry.evaluation.used_cards,
        category: entry.evaluation.category.as_str().to_owned(),
        tie_break_vector: entry.evaluation.tie_break_vector.clone(),
        category_ladder_position: category_ladder_position(entry.evaluation.category),
        result_label: result_label(entry.seat, winners).to_owned(),
        rank_explanation: rank_explanation(&entry.evaluation),
        comparison_note: comparison_note(entry, winners, single_winner, closest_challenger),
        best_five_accessibility_label: best_five_accessibility_label(&entry.evaluation.used_cards),
        hand_name,
    }
}

fn showdown_presentation_v2(
    state: &RiverLedgerState,
    evaluations: &[SeatEvaluation],
    winners: &[RiverLedgerSeat],
    allocation: &PotAllocation,
    headline: &str,
    decisive_comparison: &str,
    comparison_basis: &str,
) -> RiverLedgerShowdownPresentationV2 {
    let closest = closest_challenger(evaluations, winners);
    let ranked = ranked_evaluations(evaluations);
    let folded_rows = folded_rows(state, evaluations);
    let standings = ranked
        .iter()
        .enumerate()
        .map(|(index, entry)| standing_presentation(state, entry, index + 1, allocation, winners))
        .collect::<Vec<_>>();

    RiverLedgerShowdownPresentationV2 {
        result_banner: ShowdownResultBanner {
            headline: headline.to_owned(),
            subheadline: decisive_comparison.to_owned(),
            accessibility_label: format!("{headline} {comparison_basis}"),
        },
        decisive_reason: ShowdownDecisiveReason {
            short_text: comparison_basis.to_owned(),
            contrast_seat: closest.map(|entry| entry.seat),
            contrast_seat_label: closest.map(|entry| seat_public_label(entry.seat)),
            rule_refs: rule_refs(winners),
        },
        board_cards: board_card_presentations(state, evaluations),
        standings,
        folded_rows,
    }
}

fn ranked_evaluations(evaluations: &[SeatEvaluation]) -> Vec<&SeatEvaluation> {
    let mut ranked = evaluations.iter().collect::<Vec<_>>();
    ranked.sort_by(|left, right| {
        compare_evaluations(&right.evaluation, &left.evaluation)
            .then_with(|| left.seat.index().cmp(&right.seat.index()))
    });
    ranked
}

fn standing_presentation(
    state: &RiverLedgerState,
    entry: &SeatEvaluation,
    rank: usize,
    allocation: &PotAllocation,
    winners: &[RiverLedgerSeat],
) -> ShowdownStandingPresentation {
    let share = allocation
        .shares
        .iter()
        .find(|share| share.seat == entry.seat)
        .map(|share| share.amount)
        .unwrap_or(0);
    let hand = state
        .private_hand_for_internal(entry.seat)
        .expect("showdown standing has a private hand");
    let best_five = entry.evaluation.used_cards;

    ShowdownStandingPresentation {
        seat: entry.seat,
        seat_label: seat_public_label(entry.seat),
        rank: rank as u8,
        result_label: result_label(entry.seat, winners).to_owned(),
        allocation_label: format!("{share} from the ledger"),
        hand_name: hand_name(&entry.evaluation),
        short_comparison_note: rank_explanation(&entry.evaluation),
        rank_ladder_label: category_ladder_position(entry.evaluation.category).description,
        hole_cards: hand
            .iter()
            .copied()
            .map(|card| card_usage_mark(card, &best_five))
            .collect(),
        board_cards: state
            .board
            .iter()
            .copied()
            .map(|card| card_usage_mark(card, &best_five))
            .collect(),
        best_five: best_five.to_vec(),
        best_five_accessibility_label: best_five_accessibility_label(&best_five),
        detail_rows: vec![
            ShowdownDetailRow {
                label: "Category".to_owned(),
                value: category_label(entry.evaluation.category).to_owned(),
            },
            ShowdownDetailRow {
                label: "Tie break".to_owned(),
                value: entry
                    .evaluation
                    .tie_break_vector
                    .iter()
                    .map(|rank| rank_plural(*rank))
                    .collect::<Vec<_>>()
                    .join(", "),
            },
        ],
        default_expanded: winners.contains(&entry.seat),
    }
}

fn card_usage_mark(card: Card, best_five: &[Card; 5]) -> ShowdownCardUsageMark {
    ShowdownCardUsageMark {
        card,
        public_label: card.public_label(),
        used_in_best_five: best_five.contains(&card),
    }
}

fn board_card_presentations(
    state: &RiverLedgerState,
    evaluations: &[SeatEvaluation],
) -> Vec<ShowdownBoardCardPresentation> {
    state
        .board
        .iter()
        .copied()
        .enumerate()
        .map(|(index, card)| ShowdownBoardCardPresentation {
            slot: board_slot(index).to_owned(),
            card,
            public_label: card.public_label(),
            used_by_selected: evaluations
                .iter()
                .filter(|entry| entry.evaluation.used_cards.contains(&card))
                .map(|entry| seat_public_label(entry.seat))
                .collect(),
        })
        .collect()
}

fn folded_rows(
    state: &RiverLedgerState,
    evaluations: &[SeatEvaluation],
) -> Vec<ShowdownFoldedRowPresentation> {
    state
        .ledger
        .seats
        .iter()
        .filter(|ledger| !evaluations.iter().any(|entry| entry.seat == ledger.seat))
        .map(|ledger| ShowdownFoldedRowPresentation {
            seat: ledger.seat,
            seat_label: seat_public_label(ledger.seat),
            redaction_label: "Folded before showdown; hand remains hidden.".to_owned(),
        })
        .collect()
}

fn board_slot(index: usize) -> &'static str {
    match index {
        0 => "flop_1",
        1 => "flop_2",
        2 => "flop_3",
        3 => "turn",
        _ => "river",
    }
}

fn rule_refs(winners: &[RiverLedgerSeat]) -> Vec<String> {
    if winners.len() > 1 {
        vec![
            "RL-SCORE-SHOWDOWN".to_owned(),
            "RL-SCORE-SPLIT".to_owned(),
            "RL-END-SHOWDOWN".to_owned(),
        ]
    } else {
        vec!["RL-SCORE-SHOWDOWN".to_owned(), "RL-END-SHOWDOWN".to_owned()]
    }
}

fn single_winner_evaluation<'a>(
    evaluations: &'a [SeatEvaluation],
    winners: &[RiverLedgerSeat],
) -> Option<&'a SeatEvaluation> {
    match winners {
        [winner] => evaluations.iter().find(|entry| entry.seat == *winner),
        _ => None,
    }
}

fn representative_winning_evaluation<'a>(
    evaluations: &'a [SeatEvaluation],
    winners: &[RiverLedgerSeat],
) -> Option<&'a SeatEvaluation> {
    winners
        .first()
        .and_then(|winner| evaluations.iter().find(|entry| entry.seat == *winner))
}

fn closest_challenger<'a>(
    evaluations: &'a [SeatEvaluation],
    winners: &[RiverLedgerSeat],
) -> Option<&'a SeatEvaluation> {
    evaluations
        .iter()
        .filter(|entry| !winners.contains(&entry.seat))
        .max_by(|left, right| compare_evaluations(&left.evaluation, &right.evaluation))
}

fn showdown_headline(evaluations: &[SeatEvaluation], winners: &[RiverLedgerSeat]) -> String {
    if winners.len() > 1 {
        let hand = representative_winning_evaluation(evaluations, winners)
            .map(|entry| hand_name(&entry.evaluation))
            .unwrap_or_else(|| "the best hand".to_owned());
        return format!("{} split the ledger with {hand}.", seat_list(winners));
    }

    let winner = winners
        .first()
        .expect("showdown headline requires at least one winner");
    let hand = single_winner_evaluation(evaluations, winners)
        .map(|entry| hand_name(&entry.evaluation))
        .unwrap_or_else(|| "the best hand".to_owned());
    format!("{} wins with {hand}.", seat_public_label(*winner))
}

fn decisive_comparison(evaluations: &[SeatEvaluation], winners: &[RiverLedgerSeat]) -> String {
    let winner = if winners.len() > 1 {
        representative_winning_evaluation(evaluations, winners)
    } else {
        single_winner_evaluation(evaluations, winners)
    };
    let Some(winner) = winner else {
        return "No showdown comparison is available.".to_owned();
    };
    if winners.len() > 1 {
        return format!(
            "{} all hold {}, so the ledger is split.",
            seat_list(winners),
            hand_name(&winner.evaluation)
        );
    }

    match closest_challenger(evaluations, winners) {
        Some(challenger) => format!(
            "{} beats {}.",
            hand_name(&winner.evaluation),
            hand_name(&challenger.evaluation)
        ),
        None => format!(
            "{} is the only revealed showdown hand.",
            hand_name(&winner.evaluation)
        ),
    }
}

fn comparison_basis(evaluations: &[SeatEvaluation], winners: &[RiverLedgerSeat]) -> String {
    let Some(winner) = single_winner_evaluation(evaluations, winners) else {
        if winners.len() > 1 {
            return "The best revealed hands have equal category and tie-break ranks.".to_owned();
        }
        return "Showdown requires at least one evaluated hand.".to_owned();
    };
    let Some(challenger) = closest_challenger(evaluations, winners) else {
        return "Only one seat reached showdown, so no tie-break comparison is needed.".to_owned();
    };

    if winner.evaluation.category != challenger.evaluation.category {
        return format!(
            "{} outranks {}.",
            category_label(winner.evaluation.category),
            category_label(challenger.evaluation.category)
        );
    }

    let category = category_label_lower(winner.evaluation.category);
    let vectors = winner
        .evaluation
        .tie_break_vector
        .iter()
        .zip(challenger.evaluation.tie_break_vector.iter())
        .enumerate()
        .find(|(_, (left, right))| left != right);

    match vectors {
        Some((index, (winner_rank, challenger_rank))) => format!(
            "Both hands are {category}, so the {} decides first: {} > {}.",
            tie_break_label(winner.evaluation.category, index),
            rank_plural(*winner_rank),
            rank_plural(*challenger_rank)
        ),
        None => format!("Both hands are {category} with equal tie-break ranks."),
    }
}

fn result_label(seat: RiverLedgerSeat, winners: &[RiverLedgerSeat]) -> &'static str {
    if !winners.contains(&seat) {
        "Showdown loss"
    } else if winners.len() > 1 {
        "Split win"
    } else {
        "Win"
    }
}

fn comparison_note(
    entry: &SeatEvaluation,
    winners: &[RiverLedgerSeat],
    single_winner: Option<&SeatEvaluation>,
    closest_challenger: Option<&SeatEvaluation>,
) -> String {
    if winners.contains(&entry.seat) {
        if winners.len() > 1 {
            return "Ties for the best hand and shares the ledger.".to_owned();
        }
        return closest_challenger.map_or_else(
            || "Only revealed showdown hand.".to_owned(),
            |challenger| {
                format!(
                    "{} beats {}.",
                    hand_name(&entry.evaluation),
                    hand_name(&challenger.evaluation)
                )
            },
        );
    }

    if winners.len() > 1 {
        return "Does not tie the best showdown hand.".to_owned();
    }

    single_winner.map_or_else(
        || "Does not hold the best showdown hand.".to_owned(),
        |winner| {
            format!(
                "{} loses to {}.",
                hand_name(&entry.evaluation),
                hand_name(&winner.evaluation)
            )
        },
    )
}

fn hand_name(evaluation: &HandEvaluation) -> String {
    let ranks = &evaluation.tie_break_vector;
    match evaluation.category {
        HandCategory::HighCard => format!("{}-high", rank_singular(ranks[0])),
        HandCategory::OnePair => format!("Pair of {}", rank_plural(ranks[0])),
        HandCategory::TwoPair => format!(
            "Two pair, {} and {}",
            rank_plural(ranks[0]),
            rank_plural(ranks[1])
        ),
        HandCategory::ThreeOfAKind => format!("Three of a kind, {}", rank_plural(ranks[0])),
        HandCategory::Straight => format!("{}-high straight", rank_singular(ranks[0])),
        HandCategory::Flush => format!("{}-high flush", rank_singular(ranks[0])),
        HandCategory::FullHouse => format!(
            "Full house, {} over {}",
            rank_plural(ranks[0]),
            rank_plural(ranks[1])
        ),
        HandCategory::FourOfAKind => format!("Four of a kind, {}", rank_plural(ranks[0])),
        HandCategory::StraightFlush => {
            format!("{}-high straight flush", rank_singular(ranks[0]))
        }
    }
}

fn category_ladder_position(category: HandCategory) -> CategoryLadderPosition {
    const TOTAL: u8 = 9;
    let position = match category {
        HandCategory::StraightFlush => 1,
        HandCategory::FourOfAKind => 2,
        HandCategory::FullHouse => 3,
        HandCategory::Flush => 4,
        HandCategory::Straight => 5,
        HandCategory::ThreeOfAKind => 6,
        HandCategory::TwoPair => 7,
        HandCategory::OnePair => 8,
        HandCategory::HighCard => 9,
    };
    CategoryLadderPosition {
        position,
        total: TOTAL,
        description: format!(
            "{} is category {position} of {TOTAL} from strongest to weakest.",
            category_label(category)
        ),
    }
}

fn rank_explanation(evaluation: &HandEvaluation) -> String {
    let ranks = &evaluation.tie_break_vector;
    match evaluation.category {
        HandCategory::HighCard => format!("high cards {}", rank_list(ranks)),
        HandCategory::OnePair => format!(
            "pair rank {}; kickers {}",
            rank_singular(ranks[0]),
            rank_list(&ranks[1..])
        ),
        HandCategory::TwoPair => format!(
            "pair ranks {} and {}; kicker {}",
            rank_plural(ranks[0]),
            rank_plural(ranks[1]),
            rank_singular(ranks[2])
        ),
        HandCategory::ThreeOfAKind => format!(
            "three-of-a-kind rank {}; kickers {}",
            rank_singular(ranks[0]),
            rank_list(&ranks[1..])
        ),
        HandCategory::Straight => format!("straight high card {}", rank_singular(ranks[0])),
        HandCategory::Flush => format!("flush ranks {}", rank_list(ranks)),
        HandCategory::FullHouse => format!(
            "three {} over pair {}",
            rank_plural(ranks[0]),
            rank_plural(ranks[1])
        ),
        HandCategory::FourOfAKind => format!(
            "four-of-a-kind rank {}; kicker {}",
            rank_singular(ranks[0]),
            rank_singular(ranks[1])
        ),
        HandCategory::StraightFlush => {
            format!("straight flush high card {}", rank_singular(ranks[0]))
        }
    }
}

fn best_five_accessibility_label(cards: &[Card; 5]) -> String {
    format!(
        "Best five cards: {}.",
        cards
            .iter()
            .map(|card| { format!("{} of {}", card.rank.as_str(), card.suit.as_str()) })
            .collect::<Vec<_>>()
            .join(", ")
    )
}

fn category_label(category: HandCategory) -> &'static str {
    match category {
        HandCategory::HighCard => "High card",
        HandCategory::OnePair => "One pair",
        HandCategory::TwoPair => "Two pair",
        HandCategory::ThreeOfAKind => "Three of a kind",
        HandCategory::Straight => "Straight",
        HandCategory::Flush => "Flush",
        HandCategory::FullHouse => "Full house",
        HandCategory::FourOfAKind => "Four of a kind",
        HandCategory::StraightFlush => "Straight flush",
    }
}

fn category_label_lower(category: HandCategory) -> &'static str {
    match category {
        HandCategory::HighCard => "high card",
        HandCategory::OnePair => "one pair",
        HandCategory::TwoPair => "two pair",
        HandCategory::ThreeOfAKind => "three of a kind",
        HandCategory::Straight => "a straight",
        HandCategory::Flush => "a flush",
        HandCategory::FullHouse => "a full house",
        HandCategory::FourOfAKind => "four of a kind",
        HandCategory::StraightFlush => "a straight flush",
    }
}

fn tie_break_label(category: HandCategory, index: usize) -> &'static str {
    match (category, index) {
        (HandCategory::HighCard | HandCategory::Flush, 0) => "highest card",
        (HandCategory::HighCard | HandCategory::Flush, _) => "next highest card",
        (HandCategory::OnePair, 0) => "pair rank",
        (HandCategory::OnePair, _) => "kicker",
        (HandCategory::TwoPair, 0) => "higher pair",
        (HandCategory::TwoPair, 1) => "lower pair",
        (HandCategory::TwoPair, _) => "kicker",
        (HandCategory::ThreeOfAKind, 0) => "three-of-a-kind rank",
        (HandCategory::ThreeOfAKind, _) => "kicker",
        (HandCategory::Straight | HandCategory::StraightFlush, _) => "straight high card",
        (HandCategory::FullHouse, 0) => "three-of-a-kind rank",
        (HandCategory::FullHouse, _) => "pair rank",
        (HandCategory::FourOfAKind, 0) => "four-of-a-kind rank",
        (HandCategory::FourOfAKind, _) => "kicker",
    }
}

fn rank_list(ranks: &[u8]) -> String {
    ranks
        .iter()
        .map(|rank| rank_singular(*rank))
        .collect::<Vec<_>>()
        .join(", ")
}

fn rank_singular(value: u8) -> &'static str {
    rank_from_value(value).map_or("Unknown", |rank| match rank {
        Rank::Two => "Two",
        Rank::Three => "Three",
        Rank::Four => "Four",
        Rank::Five => "Five",
        Rank::Six => "Six",
        Rank::Seven => "Seven",
        Rank::Eight => "Eight",
        Rank::Nine => "Nine",
        Rank::Ten => "Ten",
        Rank::Jack => "Jack",
        Rank::Queen => "Queen",
        Rank::King => "King",
        Rank::Ace => "Ace",
    })
}

fn rank_plural(value: u8) -> &'static str {
    rank_from_value(value).map_or("Unknowns", |rank| match rank {
        Rank::Two => "Twos",
        Rank::Three => "Threes",
        Rank::Four => "Fours",
        Rank::Five => "Fives",
        Rank::Six => "Sixes",
        Rank::Seven => "Sevens",
        Rank::Eight => "Eights",
        Rank::Nine => "Nines",
        Rank::Ten => "Tens",
        Rank::Jack => "Jacks",
        Rank::Queen => "Queens",
        Rank::King => "Kings",
        Rank::Ace => "Aces",
    })
}

fn rank_from_value(value: u8) -> Option<Rank> {
    Rank::ALL.iter().copied().find(|rank| rank.value() == value)
}

fn seat_list(seats: &[RiverLedgerSeat]) -> String {
    match seats {
        [] => "No seats".to_owned(),
        [seat] => seat_public_label(*seat),
        [first, second] => format!(
            "{} and {}",
            seat_public_label(*first),
            seat_public_label(*second)
        ),
        _ => {
            let mut parts = seats
                .iter()
                .copied()
                .map(seat_public_label)
                .collect::<Vec<_>>();
            let last = parts.pop().expect("non-empty list has a last seat");
            format!("{}, and {last}", parts.join(", "))
        }
    }
}

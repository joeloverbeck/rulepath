use crate::{
    ids::RiverLedgerSeat,
    state::{PotShare, SeatLedger, SeatStatus},
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ContributionLayerPot {
    pub id: String,
    pub lower_cap: u16,
    pub upper_cap: u16,
    pub amount: u16,
    pub contributors: Vec<RiverLedgerSeat>,
    pub eligible: Vec<RiverLedgerSeat>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UncalledReturn {
    pub seat: RiverLedgerSeat,
    pub amount: u16,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ContributionLayers {
    pub pots: Vec<ContributionLayerPot>,
    pub returns: Vec<UncalledReturn>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PotAllocation {
    pub pot_total: u16,
    pub winners: Vec<RiverLedgerSeat>,
    pub shares: Vec<PotShare>,
    pub remainder: u16,
    pub remainder_order: Vec<RiverLedgerSeat>,
}

pub fn allocate_single_pot(
    pot_total: u16,
    winners: &[RiverLedgerSeat],
    button: RiverLedgerSeat,
    seat_count: u8,
) -> PotAllocation {
    assert!(
        !winners.is_empty(),
        "single-pot allocation requires at least one winner"
    );

    let remainder_order = winners_in_button_order(winners, button, seat_count);
    let base_share = pot_total / winners.len() as u16;
    let remainder = pot_total % winners.len() as u16;
    let remainder_recipients = remainder_order
        .iter()
        .take(remainder as usize)
        .copied()
        .collect::<Vec<_>>();
    let shares = winners
        .iter()
        .map(|seat| PotShare {
            seat: *seat,
            amount: base_share + u16::from(remainder_recipients.contains(seat)),
        })
        .collect::<Vec<_>>();

    PotAllocation {
        pot_total,
        winners: winners.to_vec(),
        shares,
        remainder,
        remainder_order,
    }
}

pub fn construct_contribution_layers(seats: &[SeatLedger]) -> ContributionLayers {
    let mut levels = seats
        .iter()
        .map(|seat| seat.total_contribution)
        .filter(|amount| *amount > 0)
        .collect::<Vec<_>>();
    levels.sort_unstable();
    levels.dedup();

    let mut previous = 0u16;
    let mut segments = Vec::<ContributionLayerPot>::new();
    let mut returns = Vec::<UncalledReturn>::new();

    for level in levels {
        let contributors = seats_at_or_above(seats, level);
        let delta = level
            .checked_sub(previous)
            .expect("contribution levels are ascending");
        let amount = delta
            .checked_mul(contributors.len() as u16)
            .expect("contribution layer amount fits u16");

        if contributors.len() == 1 {
            returns.push(UncalledReturn {
                seat: contributors[0],
                amount,
            });
        } else {
            let eligible = contributors
                .iter()
                .copied()
                .filter(|seat| seats[seat.index()].status != SeatStatus::Folded)
                .collect::<Vec<_>>();
            assert!(
                !eligible.is_empty(),
                "valid River Ledger pot layer requires at least one eligible seat"
            );
            segments.push(ContributionLayerPot {
                id: String::new(),
                lower_cap: previous,
                upper_cap: level,
                amount,
                contributors,
                eligible,
            });
        }

        previous = level;
    }

    let mut pots = coalesced_segments(segments);
    for (index, pot) in pots.iter_mut().enumerate() {
        pot.id = if index == 0 {
            "main_pot".to_owned()
        } else {
            format!("side_pot_{index}")
        };
    }

    assert_contribution_layer_conservation(seats, &pots, &returns);
    ContributionLayers { pots, returns }
}

fn seats_at_or_above(seats: &[SeatLedger], level: u16) -> Vec<RiverLedgerSeat> {
    seats
        .iter()
        .filter(|seat| seat.total_contribution >= level)
        .map(|seat| seat.seat)
        .collect()
}

fn coalesced_segments(segments: Vec<ContributionLayerPot>) -> Vec<ContributionLayerPot> {
    let mut coalesced: Vec<ContributionLayerPot> = Vec::new();
    for segment in segments {
        if let Some(previous) = coalesced.last_mut() {
            if previous.eligible == segment.eligible {
                previous.upper_cap = segment.upper_cap;
                previous.amount = previous
                    .amount
                    .checked_add(segment.amount)
                    .expect("coalesced contribution layer amount fits u16");
                merge_canonical_seats(&mut previous.contributors, &segment.contributors);
                continue;
            }
        }
        coalesced.push(segment);
    }
    coalesced
}

fn merge_canonical_seats(target: &mut Vec<RiverLedgerSeat>, incoming: &[RiverLedgerSeat]) {
    for seat in incoming {
        if !target.contains(seat) {
            target.push(*seat);
        }
    }
    target.sort_by_key(|seat| seat.index());
}

fn assert_contribution_layer_conservation(
    seats: &[SeatLedger],
    pots: &[ContributionLayerPot],
    returns: &[UncalledReturn],
) {
    let input = seats
        .iter()
        .try_fold(0u16, |total, seat| {
            total.checked_add(seat.total_contribution)
        })
        .expect("input contributions fit u16");
    let pot_total = pots
        .iter()
        .try_fold(0u16, |total, pot| total.checked_add(pot.amount))
        .expect("pot layer contributions fit u16");
    let return_total = returns
        .iter()
        .try_fold(0u16, |total, returned| total.checked_add(returned.amount))
        .expect("uncalled returns fit u16");
    assert_eq!(input, pot_total + return_total);
    assert!(
        pots.iter().all(|pot| pot.contributors.len() > 1),
        "singleton top layers must be represented as uncalled returns"
    );
}

pub fn winners_in_button_order(
    winners: &[RiverLedgerSeat],
    button: RiverLedgerSeat,
    seat_count: u8,
) -> Vec<RiverLedgerSeat> {
    let mut ordered = Vec::with_capacity(winners.len());
    let mut current = button;
    for _ in 0..seat_count {
        if winners.contains(&current) {
            ordered.push(current);
        }
        current = current
            .next_in_count(seat_count)
            .expect("button order uses valid seat count");
    }
    ordered
}

#[cfg(test)]
mod tests {
    use super::*;

    fn seat(index: usize) -> RiverLedgerSeat {
        RiverLedgerSeat::from_index(index).unwrap()
    }

    fn ledger(index: usize, status: SeatStatus, total_contribution: u16) -> SeatLedger {
        SeatLedger {
            seat: seat(index),
            status,
            starting_stack: 24,
            remaining_stack: 24u16.saturating_sub(total_contribution),
            street_contribution: 0,
            total_contribution,
        }
    }

    #[test]
    fn contribution_layers_build_main_and_side_pots_in_cap_order() {
        let layers = construct_contribution_layers(&[
            ledger(0, SeatStatus::ShowdownEligible, 4),
            ledger(1, SeatStatus::ShowdownEligible, 8),
            ledger(2, SeatStatus::ShowdownEligible, 8),
        ]);

        assert!(layers.returns.is_empty());
        assert_eq!(layers.pots.len(), 2);
        assert_eq!(layers.pots[0].id, "main_pot");
        assert_eq!(layers.pots[0].lower_cap, 0);
        assert_eq!(layers.pots[0].upper_cap, 4);
        assert_eq!(layers.pots[0].amount, 12);
        assert_eq!(layers.pots[0].contributors, vec![seat(0), seat(1), seat(2)]);
        assert_eq!(layers.pots[0].eligible, vec![seat(0), seat(1), seat(2)]);
        assert_eq!(layers.pots[1].id, "side_pot_1");
        assert_eq!(layers.pots[1].lower_cap, 4);
        assert_eq!(layers.pots[1].upper_cap, 8);
        assert_eq!(layers.pots[1].amount, 8);
        assert_eq!(layers.pots[1].contributors, vec![seat(1), seat(2)]);
        assert_eq!(layers.pots[1].eligible, vec![seat(1), seat(2)]);
    }

    #[test]
    fn contribution_layers_keep_folded_money_but_exclude_folded_eligibility() {
        let layers = construct_contribution_layers(&[
            ledger(0, SeatStatus::Folded, 4),
            ledger(1, SeatStatus::ShowdownEligible, 4),
            ledger(2, SeatStatus::ShowdownEligible, 8),
        ]);

        assert_eq!(layers.pots.len(), 1);
        assert_eq!(layers.returns.len(), 1);
        assert_eq!(layers.pots[0].amount, 12);
        assert_eq!(layers.pots[0].contributors, vec![seat(0), seat(1), seat(2)]);
        assert_eq!(layers.pots[0].eligible, vec![seat(1), seat(2)]);
        assert_eq!(
            layers.returns[0],
            UncalledReturn {
                seat: seat(2),
                amount: 4
            }
        );
    }

    #[test]
    fn contribution_layers_coalesce_identical_eligibility_segments() {
        let layers = construct_contribution_layers(&[
            ledger(0, SeatStatus::ShowdownEligible, 10),
            ledger(1, SeatStatus::ShowdownEligible, 10),
            ledger(2, SeatStatus::Folded, 4),
        ]);

        assert!(layers.returns.is_empty());
        assert_eq!(layers.pots.len(), 1);
        assert_eq!(layers.pots[0].id, "main_pot");
        assert_eq!(layers.pots[0].lower_cap, 0);
        assert_eq!(layers.pots[0].upper_cap, 10);
        assert_eq!(layers.pots[0].amount, 24);
        assert_eq!(layers.pots[0].contributors, vec![seat(0), seat(1), seat(2)]);
        assert_eq!(layers.pots[0].eligible, vec![seat(0), seat(1)]);
    }

    #[test]
    fn single_pot_even_split_conserves_total() {
        let allocation = allocate_single_pot(12, &[seat(1), seat(3)], seat(0), 4);

        assert_eq!(allocation.winners, vec![seat(1), seat(3)]);
        assert_eq!(
            allocation.shares,
            vec![
                PotShare {
                    seat: seat(1),
                    amount: 6,
                },
                PotShare {
                    seat: seat(3),
                    amount: 6,
                },
            ]
        );
        assert_eq!(
            allocation
                .shares
                .iter()
                .map(|share| share.amount)
                .sum::<u16>(),
            allocation.pot_total
        );
    }

    #[test]
    fn remainder_is_assigned_by_button_order() {
        let allocation = allocate_single_pot(11, &[seat(0), seat(2), seat(3)], seat(2), 4);

        assert_eq!(allocation.winners, vec![seat(0), seat(2), seat(3)]);
        assert_eq!(allocation.remainder, 2);
        assert_eq!(allocation.remainder_order, vec![seat(2), seat(3), seat(0)]);
        assert_eq!(
            allocation.shares,
            vec![
                PotShare {
                    seat: seat(0),
                    amount: 3,
                },
                PotShare {
                    seat: seat(2),
                    amount: 4,
                },
                PotShare {
                    seat: seat(3),
                    amount: 4,
                },
            ]
        );
    }

    #[test]
    fn canonical_winner_order_survives_nontrivial_button_order() {
        let canonical_winners = vec![seat(1), seat(2), seat(3)];
        let allocation = allocate_single_pot(11, &canonical_winners, seat(2), 4);

        assert_eq!(allocation.winners, canonical_winners);
        assert_eq!(allocation.remainder_order, vec![seat(2), seat(3), seat(1)]);
        assert_eq!(
            allocation.shares,
            vec![
                PotShare {
                    seat: seat(1),
                    amount: 3,
                },
                PotShare {
                    seat: seat(2),
                    amount: 4,
                },
                PotShare {
                    seat: seat(3),
                    amount: 4,
                },
            ]
        );
        assert_eq!(
            allocation
                .shares
                .iter()
                .map(|share| share.amount)
                .sum::<u16>(),
            allocation.pot_total
        );
    }
}

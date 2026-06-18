use crate::{ids::RiverLedgerSeat, state::PotShare};

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

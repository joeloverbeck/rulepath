use engine_core::{Diagnostic, FreshnessToken, SeatId, Seed};

use crate::{
    ids::TokenBazaarSeat,
    state::{ResourceCounts, TokenBazaarState, STANDARD_CONTRACTS},
    variants::Variant,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SetupOptions {
    pub variant: Variant,
}

impl Default for SetupOptions {
    fn default() -> Self {
        Self {
            variant: Variant::token_bazaar_standard(),
        }
    }
}

pub fn setup_match(
    _seed: Seed,
    seats: &[SeatId],
    options: &SetupOptions,
) -> Result<TokenBazaarState, Diagnostic> {
    if options.variant.id != crate::ids::VARIANT_ID {
        return Err(Diagnostic {
            code: "unsupported_variant".to_owned(),
            message: "token_bazaar supports only token_bazaar_standard".to_owned(),
        });
    }

    if seats.len() != options.variant.seat_count as usize {
        return Err(Diagnostic {
            code: "invalid_seat_count".to_owned(),
            message: "token_bazaar requires exactly two seats".to_owned(),
        });
    }

    let mut queue = STANDARD_CONTRACTS
        .iter()
        .map(|contract| contract.id)
        .collect::<Vec<_>>();
    let slots = [
        Some(queue.remove(0)),
        Some(queue.remove(0)),
        Some(queue.remove(0)),
    ];

    Ok(TokenBazaarState {
        variant: options.variant.clone(),
        seats: [seats[0].clone(), seats[1].clone()],
        supply: ResourceCounts::new(
            options.variant.resource_supply,
            options.variant.resource_supply,
            options.variant.resource_supply,
        ),
        inventories: [
            ResourceCounts::new(
                options.variant.starting_resource_count,
                options.variant.starting_resource_count,
                options.variant.starting_resource_count,
            ),
            ResourceCounts::new(
                options.variant.starting_resource_count,
                options.variant.starting_resource_count,
                options.variant.starting_resource_count,
            ),
        ],
        scores: [0, 0],
        slots,
        queue,
        fulfilled: [Vec::new(), Vec::new()],
        turns_taken: [0, 0],
        active_seat: TokenBazaarSeat::Seat0,
        terminal_outcome: None,
        freshness_token: FreshnessToken(0),
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use engine_core::StableSerialize;

    use crate::{
        ids::{ContractId, TokenBazaarSlot},
        state::TokenBazaarSnapshot,
    };

    #[test]
    fn setup_is_deterministic_standard_public_state() {
        let seats = vec![SeatId("seat-0".to_owned()), SeatId("seat-1".to_owned())];
        let options = SetupOptions::default();

        let left = setup_match(Seed(1), &seats, &options).expect("setup succeeds");
        let right = setup_match(Seed(999), &seats, &options).expect("setup succeeds");

        assert_eq!(left, right);
        assert_eq!(left.supply, ResourceCounts::new(14, 14, 14));
        assert_eq!(left.inventories[0], ResourceCounts::new(1, 1, 1));
        assert_eq!(left.inventories[1], ResourceCounts::new(1, 1, 1));
        assert_eq!(left.scores, [0, 0]);
        assert_eq!(
            left.slot_contract(TokenBazaarSlot::Slot0),
            Some(ContractId::BalancedWares)
        );
        assert_eq!(
            left.slot_contract(TokenBazaarSlot::Slot1),
            Some(ContractId::AmberGuild)
        );
        assert_eq!(
            left.slot_contract(TokenBazaarSlot::Slot2),
            Some(ContractId::IronGuild)
        );
        assert_eq!(left.queue.len(), 7);
        assert_eq!(left.queue[0], ContractId::JadeGuild);
        assert_eq!(left.queue[6], ContractId::CrownRoute);
        assert_eq!(left.fulfilled, [Vec::new(), Vec::new()]);
        assert_eq!(left.turns_taken, [0, 0]);
        assert_eq!(left.active_seat, TokenBazaarSeat::Seat0);
        assert_eq!(left.terminal_outcome, None);
        assert_eq!(left.freshness_token, FreshnessToken(0));

        let left_snapshot = TokenBazaarSnapshot::from_state(&left);
        let right_snapshot = TokenBazaarSnapshot::from_state(&right);
        assert_eq!(left_snapshot.stable_bytes(), right_snapshot.stable_bytes());
    }

    #[test]
    fn setup_rejects_wrong_seat_count() {
        let seats = vec![SeatId("seat-0".to_owned())];

        let diagnostic = setup_match(Seed(1), &seats, &SetupOptions::default())
            .expect_err("setup rejects missing seat");

        assert_eq!(diagnostic.code, "invalid_seat_count");
    }
}

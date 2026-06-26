use engine_core::SeatId;
use game_stdlib::{SeatCount, SeatCountRange};
use std::sync::LazyLock;

pub const GAME_ID: &str = "meldfall_ledger";
pub const VARIANT_ID: &str = "classic_500_single_deck_v1";
pub const RULES_VERSION_LABEL: &str = "meldfall-ledger-rules-v1";
pub const DATA_VERSION_LABEL: &str = "meldfall-ledger-data-v1";
pub const STANDARD_MIN_SEATS: u8 = 2;
pub const STANDARD_DEFAULT_SEATS: u8 = 4;
pub const STANDARD_MAX_SEATS: u8 = 6;
pub const STANDARD_SUIT_COUNT: u8 = 4;
pub const STANDARD_RANK_COUNT: u8 = 13;
pub const STANDARD_CARD_COUNT: u8 = STANDARD_SUIT_COUNT * STANDARD_RANK_COUNT;
pub const STANDARD_TARGET_SCORE: i32 = 500;
pub const STANDARD_TWO_SEAT_HAND_SIZE: u8 = 13;
pub const STANDARD_MULTI_SEAT_HAND_SIZE: u8 = 7;

pub const FIXTURE_2P_STANDARD: &str = "meldfall_ledger_2p_standard";
pub const FIXTURE_4P_STANDARD: &str = "meldfall_ledger_4p_standard";
pub const FIXTURE_6P_STANDARD: &str = "meldfall_ledger_6p_standard";
pub const FIXTURE_MULTI_DISCARD_PICKUP: &str = "meldfall_ledger_multi_discard_pickup";
pub const FIXTURE_LAYOFF_ANY_TABLEAU: &str = "meldfall_ledger_layoff_any_tableau";
pub const FIXTURE_500_TIE_CONTINUES: &str = "meldfall_ledger_500_tie_continues";

static STANDARD_SEAT_COUNT_RANGE: LazyLock<SeatCountRange> = LazyLock::new(|| {
    SeatCountRange::inclusive(STANDARD_MIN_SEATS as usize, STANDARD_MAX_SEATS as usize)
        .expect("standard Meldfall Ledger seat count range is valid")
});

pub fn supported_seat_count(seat_count: usize) -> bool {
    (*STANDARD_SEAT_COUNT_RANGE).validate(seat_count).is_ok()
}

pub fn seat_id_for_index(index: usize) -> SeatId {
    SeatId::from_zero_based_index(index.try_into().expect("seat index must fit u32"))
}

pub fn canonical_seat_ids(seat_count: usize) -> Vec<SeatId> {
    (0..seat_count).map(seat_id_for_index).collect()
}

pub fn next_clockwise_index(index: usize, seat_count: usize) -> Option<usize> {
    if !supported_seat_count(seat_count) || index >= seat_count {
        return None;
    }
    let count = SeatCount::new(seat_count).expect("validated seat count is nonzero");
    count.next_ring_index(index).ok()
}

pub fn hand_size_for_seats(seat_count: usize) -> Option<u8> {
    match seat_count {
        2 => Some(STANDARD_TWO_SEAT_HAND_SIZE),
        3..=6 => Some(STANDARD_MULTI_SEAT_HAND_SIZE),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn seat_helpers_cover_supported_range_only() {
        for rejected in [0, 1, 7, 8] {
            assert!(!supported_seat_count(rejected));
            assert_eq!(hand_size_for_seats(rejected), None);
        }

        assert_eq!(hand_size_for_seats(2), Some(13));
        for seat_count in 3..=6 {
            assert!(supported_seat_count(seat_count));
            assert_eq!(hand_size_for_seats(seat_count), Some(7));
        }
    }

    #[test]
    fn canonical_seats_use_engine_core_grammar() {
        let seats = canonical_seat_ids(6);
        assert_eq!(
            seats,
            vec![
                SeatId("seat_0".to_owned()),
                SeatId("seat_1".to_owned()),
                SeatId("seat_2".to_owned()),
                SeatId("seat_3".to_owned()),
                SeatId("seat_4".to_owned()),
                SeatId("seat_5".to_owned()),
            ]
        );
        assert_eq!(next_clockwise_index(5, 6), Some(0));
        assert_eq!(next_clockwise_index(6, 6), None);
    }
}

use engine_core::{Actor, SeatId, Viewer};

pub const GAME_ID: &str = "river_ledger";
pub const VARIANT_ID: &str = "river_ledger_standard";
pub const RULES_VERSION_LABEL: &str = "river-ledger-rules-v1";
pub const RULE_ID_PREFIX: &str = "RL-";

pub const STANDARD_MIN_SEATS: u8 = 3;
pub const STANDARD_DEFAULT_SEATS: u8 = 6;
pub const STANDARD_MAX_SEATS: u8 = 6;
pub const STANDARD_STREET_COUNT: u8 = 4;
pub const STANDARD_SMALL_BLIND: u8 = 1;
pub const STANDARD_BIG_BLIND: u8 = 2;
pub const STANDARD_SMALL_BET_UNIT: u8 = 2;
pub const STANDARD_BIG_BET_UNIT: u8 = 4;
pub const MAX_RAISES_PER_STREET: u8 = 3;

pub const ACTION_FOLD: &str = "fold";
pub const ACTION_CHECK: &str = "check";
pub const ACTION_CALL: &str = "call";
pub const ACTION_BET: &str = "bet";
pub const ACTION_RAISE: &str = "raise";

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub struct RiverLedgerSeat {
    index: u8,
}

impl RiverLedgerSeat {
    pub fn from_index(index: usize) -> Option<Self> {
        if index < STANDARD_MAX_SEATS as usize {
            Some(Self { index: index as u8 })
        } else {
            None
        }
    }

    pub const fn index(self) -> usize {
        self.index as usize
    }

    pub fn as_str(self) -> String {
        format!("seat_{}", self.index)
    }

    pub fn parse(value: &str) -> Option<Self> {
        let raw_index = value.strip_prefix("seat_")?.parse::<usize>().ok()?;
        Self::from_index(raw_index)
    }

    pub fn seats_for_count(count: u8) -> Option<Vec<Self>> {
        if !(STANDARD_MIN_SEATS..=STANDARD_MAX_SEATS).contains(&count) {
            return None;
        }
        Some((0..count).map(|index| Self { index }).collect())
    }

    pub fn next_in_count(self, count: u8) -> Option<Self> {
        if count == 0 || self.index >= count {
            return None;
        }
        Some(Self {
            index: (self.index + 1) % count,
        })
    }
}

pub fn seat_id_for_index(index: usize) -> Option<SeatId> {
    RiverLedgerSeat::from_index(index).map(|seat| SeatId(seat.as_str()))
}

pub fn actor_for_seat(seat: RiverLedgerSeat) -> Actor {
    Actor {
        seat_id: SeatId(seat.as_str()),
    }
}

pub fn seat_viewer_for_index(index: usize) -> Option<Viewer> {
    seat_id_for_index(index).map(|seat_id| Viewer {
        seat_id: Some(seat_id),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn river_ledger_seats_are_stable_and_bounded() {
        let seats = RiverLedgerSeat::seats_for_count(3).expect("3 seats");

        assert_eq!(seats[0].as_str(), "seat_0");
        assert_eq!(seats[2].index(), 2);
        assert_eq!(RiverLedgerSeat::seats_for_count(2), None);
        assert_eq!(RiverLedgerSeat::seats_for_count(7), None);
        assert_eq!(
            RiverLedgerSeat::parse("seat_5"),
            RiverLedgerSeat::from_index(5)
        );
        assert_eq!(RiverLedgerSeat::parse("seat_6"), None);
        assert_eq!(seats[2].next_in_count(3).unwrap().as_str(), "seat_0");
        assert_eq!(actor_for_seat(seats[1]).seat_id.0, "seat_1");
        assert_eq!(
            seat_viewer_for_index(2).unwrap().seat_id.unwrap().0,
            "seat_2"
        );
    }
}

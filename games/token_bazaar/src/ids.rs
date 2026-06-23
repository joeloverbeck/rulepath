use engine_core::SeatId;

pub const GAME_ID: &str = "token_bazaar";
pub const VARIANT_ID: &str = "token_bazaar_standard";
pub const RULES_VERSION_LABEL: &str = "token-bazaar-rules-v1";
pub const STANDARD_SEAT_COUNT: u8 = 2;
pub const STANDARD_RESOURCE_SUPPLY: u8 = 14;
pub const STANDARD_STARTING_RESOURCE_COUNT: u8 = 1;
pub const STANDARD_MARKET_SLOT_COUNT: u8 = 3;
pub const STANDARD_CONTRACT_COUNT: u8 = 10;
pub const STANDARD_TURNS_PER_SEAT: u8 = 8;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub enum TokenBazaarSeat {
    Seat0,
    Seat1,
}

impl TokenBazaarSeat {
    pub const ALL: [Self; 2] = [Self::Seat0, Self::Seat1];

    pub fn from_index(index: usize) -> Option<Self> {
        match index {
            0 => Some(Self::Seat0),
            1 => Some(Self::Seat1),
            _ => None,
        }
    }

    pub const fn index(self) -> usize {
        match self {
            Self::Seat0 => 0,
            Self::Seat1 => 1,
        }
    }

    pub const fn other(self) -> Self {
        match self {
            Self::Seat0 => Self::Seat1,
            Self::Seat1 => Self::Seat0,
        }
    }

    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Seat0 => "seat_0",
            Self::Seat1 => "seat_1",
        }
    }

    pub fn parse(value: &str) -> Option<Self> {
        let index = SeatId::parse_canonical(value)
            .ok()?
            .canonical_zero_based_index()
            .ok()?;
        Self::from_index(index as usize)
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub enum ResourceId {
    Amber,
    Jade,
    Iron,
}

impl ResourceId {
    pub const ALL: [Self; 3] = [Self::Amber, Self::Jade, Self::Iron];

    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Amber => "amber",
            Self::Jade => "jade",
            Self::Iron => "iron",
        }
    }

    pub fn parse(value: &str) -> Option<Self> {
        match value {
            "amber" => Some(Self::Amber),
            "jade" => Some(Self::Jade),
            "iron" => Some(Self::Iron),
            _ => None,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub enum CollectBundleId {
    Amber,
    Jade,
    Iron,
    AmberJade,
    JadeIron,
    IronAmber,
}

impl CollectBundleId {
    pub const ALL: [Self; 6] = [
        Self::Amber,
        Self::Jade,
        Self::Iron,
        Self::AmberJade,
        Self::JadeIron,
        Self::IronAmber,
    ];

    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Amber => "amber",
            Self::Jade => "jade",
            Self::Iron => "iron",
            Self::AmberJade => "amber-jade",
            Self::JadeIron => "jade-iron",
            Self::IronAmber => "iron-amber",
        }
    }

    pub fn parse(value: &str) -> Option<Self> {
        match value {
            "amber" => Some(Self::Amber),
            "jade" => Some(Self::Jade),
            "iron" => Some(Self::Iron),
            "amber-jade" => Some(Self::AmberJade),
            "jade-iron" => Some(Self::JadeIron),
            "iron-amber" => Some(Self::IronAmber),
            _ => None,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub enum ContractId {
    BalancedWares,
    AmberGuild,
    IronGuild,
    JadeGuild,
    AmberFocus,
    JadeFocus,
    IronFocus,
    SunRoute,
    StoneRoute,
    CrownRoute,
}

impl ContractId {
    pub const ALL: [Self; 10] = [
        Self::BalancedWares,
        Self::AmberGuild,
        Self::IronGuild,
        Self::JadeGuild,
        Self::AmberFocus,
        Self::JadeFocus,
        Self::IronFocus,
        Self::SunRoute,
        Self::StoneRoute,
        Self::CrownRoute,
    ];

    pub const fn as_str(self) -> &'static str {
        match self {
            Self::BalancedWares => "balanced-wares",
            Self::AmberGuild => "amber-guild",
            Self::IronGuild => "iron-guild",
            Self::JadeGuild => "jade-guild",
            Self::AmberFocus => "amber-focus",
            Self::JadeFocus => "jade-focus",
            Self::IronFocus => "iron-focus",
            Self::SunRoute => "sun-route",
            Self::StoneRoute => "stone-route",
            Self::CrownRoute => "crown-route",
        }
    }

    pub fn parse(value: &str) -> Option<Self> {
        match value {
            "balanced-wares" => Some(Self::BalancedWares),
            "amber-guild" => Some(Self::AmberGuild),
            "iron-guild" => Some(Self::IronGuild),
            "jade-guild" => Some(Self::JadeGuild),
            "amber-focus" => Some(Self::AmberFocus),
            "jade-focus" => Some(Self::JadeFocus),
            "iron-focus" => Some(Self::IronFocus),
            "sun-route" => Some(Self::SunRoute),
            "stone-route" => Some(Self::StoneRoute),
            "crown-route" => Some(Self::CrownRoute),
            _ => None,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub enum TokenBazaarSlot {
    Slot0,
    Slot1,
    Slot2,
}

impl TokenBazaarSlot {
    pub const ALL: [Self; 3] = [Self::Slot0, Self::Slot1, Self::Slot2];

    pub fn from_index(index: usize) -> Option<Self> {
        match index {
            0 => Some(Self::Slot0),
            1 => Some(Self::Slot1),
            2 => Some(Self::Slot2),
            _ => None,
        }
    }

    pub const fn index(self) -> usize {
        match self {
            Self::Slot0 => 0,
            Self::Slot1 => 1,
            Self::Slot2 => 2,
        }
    }

    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Slot0 => "slot_0",
            Self::Slot1 => "slot_1",
            Self::Slot2 => "slot_2",
        }
    }

    pub fn parse(value: &str) -> Option<Self> {
        match value {
            "slot_0" => Some(Self::Slot0),
            "slot_1" => Some(Self::Slot1),
            "slot_2" => Some(Self::Slot2),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn assert_round_trips<T: Copy + Ord + std::fmt::Debug>(
        values: &[T],
        as_str: impl Fn(T) -> &'static str,
        parse: impl Fn(&str) -> Option<T>,
    ) {
        let mut sorted = values.to_vec();
        sorted.sort();
        assert_eq!(sorted, values);

        for value in values {
            let stable = as_str(*value);
            assert_eq!(parse(stable), Some(*value));
        }
    }

    #[test]
    fn stable_id_strings_round_trip_in_canonical_order() {
        assert_round_trips(
            &TokenBazaarSeat::ALL,
            TokenBazaarSeat::as_str,
            TokenBazaarSeat::parse,
        );
        assert_round_trips(&ResourceId::ALL, ResourceId::as_str, ResourceId::parse);
        assert_round_trips(
            &CollectBundleId::ALL,
            CollectBundleId::as_str,
            CollectBundleId::parse,
        );
        assert_round_trips(&ContractId::ALL, ContractId::as_str, ContractId::parse);
        assert_round_trips(
            &TokenBazaarSlot::ALL,
            TokenBazaarSlot::as_str,
            TokenBazaarSlot::parse,
        );
    }

    #[test]
    fn seats_and_slots_are_bounded() {
        assert_eq!(TokenBazaarSeat::from_index(0), Some(TokenBazaarSeat::Seat0));
        assert_eq!(TokenBazaarSeat::from_index(1), Some(TokenBazaarSeat::Seat1));
        assert_eq!(TokenBazaarSeat::from_index(2), None);
        assert_eq!(TokenBazaarSeat::Seat0.other(), TokenBazaarSeat::Seat1);

        assert_eq!(TokenBazaarSlot::from_index(0), Some(TokenBazaarSlot::Slot0));
        assert_eq!(TokenBazaarSlot::from_index(2), Some(TokenBazaarSlot::Slot2));
        assert_eq!(TokenBazaarSlot::from_index(3), None);
    }

    #[test]
    fn seat_parse_rejects_non_canonical_and_out_of_range_ids() {
        for value in [
            "seat_00", "seat_01", "seat_１", "seat-0", "seat-a", "seat_2",
        ] {
            assert_eq!(TokenBazaarSeat::parse(value), None, "{value}");
        }
    }
}

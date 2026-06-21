use crate::ids::{STANDARD_CARD_COUNT, STANDARD_RANK_COUNT, STANDARD_SUIT_COUNT};

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub enum Suit {
    Clubs,
    Diamonds,
    Hearts,
    Spades,
}

impl Suit {
    pub const ALL: [Self; 4] = [Self::Clubs, Self::Diamonds, Self::Hearts, Self::Spades];

    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Clubs => "clubs",
            Self::Diamonds => "diamonds",
            Self::Hearts => "hearts",
            Self::Spades => "spades",
        }
    }

    pub const fn short_label(self) -> &'static str {
        match self {
            Self::Clubs => "C",
            Self::Diamonds => "D",
            Self::Hearts => "H",
            Self::Spades => "S",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub enum Rank {
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace,
}

impl Rank {
    pub const ALL: [Self; 13] = [
        Self::Two,
        Self::Three,
        Self::Four,
        Self::Five,
        Self::Six,
        Self::Seven,
        Self::Eight,
        Self::Nine,
        Self::Ten,
        Self::Jack,
        Self::Queen,
        Self::King,
        Self::Ace,
    ];

    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Two => "two",
            Self::Three => "three",
            Self::Four => "four",
            Self::Five => "five",
            Self::Six => "six",
            Self::Seven => "seven",
            Self::Eight => "eight",
            Self::Nine => "nine",
            Self::Ten => "ten",
            Self::Jack => "jack",
            Self::Queen => "queen",
            Self::King => "king",
            Self::Ace => "ace",
        }
    }

    pub const fn short_label(self) -> &'static str {
        match self {
            Self::Two => "2",
            Self::Three => "3",
            Self::Four => "4",
            Self::Five => "5",
            Self::Six => "6",
            Self::Seven => "7",
            Self::Eight => "8",
            Self::Nine => "9",
            Self::Ten => "10",
            Self::Jack => "J",
            Self::Queen => "Q",
            Self::King => "K",
            Self::Ace => "A",
        }
    }

    pub const fn value(self) -> u8 {
        match self {
            Self::Two => 2,
            Self::Three => 3,
            Self::Four => 4,
            Self::Five => 5,
            Self::Six => 6,
            Self::Seven => 7,
            Self::Eight => 8,
            Self::Nine => 9,
            Self::Ten => 10,
            Self::Jack => 11,
            Self::Queen => 12,
            Self::King => 13,
            Self::Ace => 14,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub struct CardId(u8);

impl CardId {
    pub const fn new(index: u8) -> Option<Self> {
        if index < STANDARD_CARD_COUNT {
            Some(Self(index))
        } else {
            None
        }
    }

    pub const fn index(self) -> u8 {
        self.0
    }

    pub fn parse(value: &str) -> Option<Self> {
        let (rank, suit) = value.split_once('_')?;
        let card = Card::new(parse_rank(rank)?, parse_suit(suit)?);
        Some(card.id())
    }

    pub fn as_str(self) -> String {
        self.card().id_str()
    }

    pub const fn card(self) -> Card {
        let suit_index = self.0 / STANDARD_RANK_COUNT;
        let rank_index = self.0 % STANDARD_RANK_COUNT;
        Card::new(
            Rank::ALL[rank_index as usize],
            Suit::ALL[suit_index as usize],
        )
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub struct Card {
    pub rank: Rank,
    pub suit: Suit,
}

impl Card {
    pub const fn new(rank: Rank, suit: Suit) -> Self {
        Self { rank, suit }
    }

    pub fn id(self) -> CardId {
        let suit_index = match self.suit {
            Suit::Clubs => 0,
            Suit::Diamonds => 1,
            Suit::Hearts => 2,
            Suit::Spades => 3,
        };
        let rank_index = match self.rank {
            Rank::Two => 0,
            Rank::Three => 1,
            Rank::Four => 2,
            Rank::Five => 3,
            Rank::Six => 4,
            Rank::Seven => 5,
            Rank::Eight => 6,
            Rank::Nine => 7,
            Rank::Ten => 8,
            Rank::Jack => 9,
            Rank::Queen => 10,
            Rank::King => 11,
            Rank::Ace => 12,
        };
        CardId(suit_index * STANDARD_RANK_COUNT + rank_index)
    }

    pub fn id_str(self) -> String {
        format!("{}_{}", self.rank.as_str(), self.suit.as_str())
    }

    pub fn public_label(self) -> String {
        format!("{}{}", self.rank.short_label(), self.suit.short_label())
    }

    pub const fn point_value(self) -> u8 {
        match (self.rank, self.suit) {
            (_, Suit::Hearts) => 1,
            (Rank::Queen, Suit::Spades) => 13,
            _ => 0,
        }
    }

    pub const fn is_heart(self) -> bool {
        matches!(self.suit, Suit::Hearts)
    }

    pub const fn is_two_of_clubs(self) -> bool {
        matches!((self.rank, self.suit), (Rank::Two, Suit::Clubs))
    }
}

pub type Deck = Vec<CardId>;

pub fn canonical_deck() -> Deck {
    let mut deck = Vec::with_capacity(STANDARD_CARD_COUNT as usize);
    for suit in Suit::ALL {
        for rank in Rank::ALL {
            deck.push(Card::new(rank, suit).id());
        }
    }
    deck
}

fn parse_suit(value: &str) -> Option<Suit> {
    match value {
        "clubs" => Some(Suit::Clubs),
        "diamonds" => Some(Suit::Diamonds),
        "hearts" => Some(Suit::Hearts),
        "spades" => Some(Suit::Spades),
        _ => None,
    }
}

fn parse_rank(value: &str) -> Option<Rank> {
    match value {
        "two" => Some(Rank::Two),
        "three" => Some(Rank::Three),
        "four" => Some(Rank::Four),
        "five" => Some(Rank::Five),
        "six" => Some(Rank::Six),
        "seven" => Some(Rank::Seven),
        "eight" => Some(Rank::Eight),
        "nine" => Some(Rank::Nine),
        "ten" => Some(Rank::Ten),
        "jack" => Some(Rank::Jack),
        "queen" => Some(Rank::Queen),
        "king" => Some(Rank::King),
        "ace" => Some(Rank::Ace),
        _ => None,
    }
}

pub const fn suit_count() -> u8 {
    STANDARD_SUIT_COUNT
}

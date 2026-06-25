use crate::ids::{STANDARD_CARD_COUNT, STANDARD_RANK_COUNT};

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
    Ace,
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
}

impl Rank {
    pub const ALL: [Self; 13] = [
        Self::Ace,
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
    ];

    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Ace => "ace",
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
        }
    }

    pub const fn short_label(self) -> &'static str {
        match self {
            Self::Ace => "A",
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
        }
    }

    pub const fn score_value(self) -> u8 {
        match self {
            Self::Ace => 15,
            Self::Two => 2,
            Self::Three => 3,
            Self::Four => 4,
            Self::Five => 5,
            Self::Six => 6,
            Self::Seven => 7,
            Self::Eight => 8,
            Self::Nine => 9,
            Self::Ten | Self::Jack | Self::Queen | Self::King => 10,
        }
    }

    pub const fn low_run_value(self) -> u8 {
        match self {
            Self::Ace => 1,
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
        }
    }

    pub const fn high_run_value(self) -> u8 {
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
        Some(Card::new(parse_rank(rank)?, parse_suit(suit)?).id())
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
            Rank::Ace => 0,
            Rank::Two => 1,
            Rank::Three => 2,
            Rank::Four => 3,
            Rank::Five => 4,
            Rank::Six => 5,
            Rank::Seven => 6,
            Rank::Eight => 7,
            Rank::Nine => 8,
            Rank::Ten => 9,
            Rank::Jack => 10,
            Rank::Queen => 11,
            Rank::King => 12,
        };
        CardId(suit_index * STANDARD_RANK_COUNT + rank_index)
    }

    pub fn id_str(self) -> String {
        format!("{}_{}", self.rank.as_str(), self.suit.as_str())
    }

    pub fn public_label(self) -> String {
        format!("{}{}", self.rank.short_label(), self.suit.short_label())
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

pub fn ranks_are_consecutive_low_or_high(ranks: &[Rank]) -> bool {
    ranks_are_consecutive_by(ranks, Rank::low_run_value)
        || ranks_are_consecutive_by(ranks, Rank::high_run_value)
}

fn ranks_are_consecutive_by(ranks: &[Rank], value: impl Fn(Rank) -> u8) -> bool {
    if ranks.len() < 3 {
        return false;
    }
    let mut values = ranks.iter().copied().map(value).collect::<Vec<_>>();
    values.sort_unstable();
    values.dedup();
    if values.len() != ranks.len() {
        return false;
    }
    values.windows(2).all(|pair| pair[1] == pair[0] + 1)
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
        "ace" => Some(Rank::Ace),
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
        _ => None,
    }
}

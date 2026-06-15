pub const STANDARD_RANK_COUNT: usize = 13;
pub const STANDARD_SUIT_COUNT: usize = 4;
pub const STANDARD_CARD_COUNT: u8 = (STANDARD_RANK_COUNT * STANDARD_SUIT_COUNT) as u8;

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
pub struct Card {
    pub rank: Rank,
    pub suit: Suit,
}

impl Card {
    pub const fn new(rank: Rank, suit: Suit) -> Self {
        Self { rank, suit }
    }

    pub fn id(self) -> String {
        format!("{}_{}", self.rank.as_str(), self.suit.as_str())
    }

    pub fn public_label(self) -> String {
        format!("{}{}", self.rank.short_label(), self.suit.short_label())
    }
}

pub type Deck = Vec<Card>;

pub fn canonical_deck() -> Deck {
    let mut deck = Vec::with_capacity(STANDARD_CARD_COUNT as usize);
    for suit in Suit::ALL {
        for rank in Rank::ALL {
            deck.push(Card::new(rank, suit));
        }
    }
    deck
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn canonical_deck_is_stable_52_cards() {
        let deck = canonical_deck();

        assert_eq!(deck.len(), STANDARD_CARD_COUNT as usize);
        assert_eq!(deck[0], Card::new(Rank::Two, Suit::Clubs));
        assert_eq!(deck[51], Card::new(Rank::Ace, Suit::Spades));
        assert_eq!(deck[0].id(), "two_clubs");
        assert_eq!(deck[51].public_label(), "AS");
    }
}

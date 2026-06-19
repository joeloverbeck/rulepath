//! Seat-id parsing and trace-label helpers for every registered game.
//!
//! Translates browser-supplied seat strings (`"seat_0"`, ...) into each game's
//! typed seat enum (or an `unknown_seat` diagnostic), and renders the canonical
//! trace seat labels used in replay documents. Glob-imported at the crate root.

use engine_core::SeatId;

use crate::json::escape_json;

use column_four::ColumnFourSeat;
use directional_flip::DirectionalFlipSeat;
use draughts_lite::DraughtsLiteSeat;
use high_card_duel::HighCardDuelSeat;
use masked_claims::MaskedClaimsSeat;
use plain_tricks::PlainTricksSeat;
use poker_lite::PokerLiteSeat;
use race_to_n::RaceSeat;
use river_ledger::RiverLedgerSeat;
use secret_draft::SecretDraftSeat;
use three_marks::ThreeMarksSeat;
use token_bazaar::TokenBazaarSeat;

pub(crate) fn parse_race_seat(value: &str) -> Result<RaceSeat, String> {
    RaceSeat::parse(value).ok_or_else(|| {
        format!(
            "{{\"code\":\"unknown_seat\",\"message\":\"unknown seat: {}\"}}",
            escape_json(value)
        )
    })
}

pub(crate) fn parse_replay_race_seat(value: &str) -> Result<RaceSeat, String> {
    match value {
        "seat-0" => Ok(RaceSeat::Seat0),
        "seat-1" => Ok(RaceSeat::Seat1),
        _ => parse_race_seat(value),
    }
}

pub(crate) fn trace_race_seat(seat: RaceSeat) -> &'static str {
    match seat {
        RaceSeat::Seat0 => "seat-0",
        RaceSeat::Seat1 => "seat-1",
    }
}

pub(crate) fn parse_three_seat(value: &str) -> Result<ThreeMarksSeat, String> {
    match value {
        "seat-0" => Ok(ThreeMarksSeat::Seat0),
        "seat-1" => Ok(ThreeMarksSeat::Seat1),
        _ => ThreeMarksSeat::parse(value).ok_or_else(|| {
            format!(
                "{{\"code\":\"unknown_seat\",\"message\":\"unknown seat: {}\"}}",
                escape_json(value)
            )
        }),
    }
}

pub(crate) fn trace_three_seat(seat: ThreeMarksSeat) -> &'static str {
    match seat {
        ThreeMarksSeat::Seat0 => "seat-0",
        ThreeMarksSeat::Seat1 => "seat-1",
    }
}

pub(crate) fn parse_column_seat(value: &str) -> Result<ColumnFourSeat, String> {
    match value {
        "seat-0" => Ok(ColumnFourSeat::Seat0),
        "seat-1" => Ok(ColumnFourSeat::Seat1),
        _ => ColumnFourSeat::parse(value).ok_or_else(|| {
            format!(
                "{{\"code\":\"unknown_seat\",\"message\":\"unknown seat: {}\"}}",
                escape_json(value)
            )
        }),
    }
}

pub(crate) fn trace_column_seat(seat: ColumnFourSeat) -> &'static str {
    match seat {
        ColumnFourSeat::Seat0 => "seat-0",
        ColumnFourSeat::Seat1 => "seat-1",
    }
}

pub(crate) fn parse_directional_seat(value: &str) -> Result<DirectionalFlipSeat, String> {
    match value {
        "seat-0" => Ok(DirectionalFlipSeat::Seat0),
        "seat-1" => Ok(DirectionalFlipSeat::Seat1),
        _ => DirectionalFlipSeat::parse(value).ok_or_else(|| {
            format!(
                "{{\"code\":\"unknown_seat\",\"message\":\"unknown seat: {}\"}}",
                escape_json(value)
            )
        }),
    }
}

pub(crate) fn trace_directional_seat(seat: DirectionalFlipSeat) -> &'static str {
    match seat {
        DirectionalFlipSeat::Seat0 => "seat-0",
        DirectionalFlipSeat::Seat1 => "seat-1",
    }
}

pub(crate) fn parse_draughts_seat(value: &str) -> Result<DraughtsLiteSeat, String> {
    match value {
        "seat-0" => Ok(DraughtsLiteSeat::Seat0),
        "seat-1" => Ok(DraughtsLiteSeat::Seat1),
        _ => DraughtsLiteSeat::parse(value).ok_or_else(|| {
            format!(
                "{{\"code\":\"unknown_seat\",\"message\":\"unknown seat: {}\"}}",
                escape_json(value)
            )
        }),
    }
}

pub(crate) fn trace_draughts_seat(seat: DraughtsLiteSeat) -> &'static str {
    match seat {
        DraughtsLiteSeat::Seat0 => "seat-0",
        DraughtsLiteSeat::Seat1 => "seat-1",
    }
}

pub(crate) fn parse_high_card_seat(value: &str) -> Result<HighCardDuelSeat, String> {
    match value {
        "seat-0" => Ok(HighCardDuelSeat::Seat0),
        "seat-1" => Ok(HighCardDuelSeat::Seat1),
        _ => HighCardDuelSeat::parse(value).ok_or_else(|| {
            format!(
                "{{\"code\":\"unknown_seat\",\"message\":\"unknown seat: {}\"}}",
                escape_json(value)
            )
        }),
    }
}

pub(crate) fn trace_high_card_seat(seat: HighCardDuelSeat) -> &'static str {
    match seat {
        HighCardDuelSeat::Seat0 => "seat-0",
        HighCardDuelSeat::Seat1 => "seat-1",
    }
}

pub(crate) fn parse_masked_seat(value: &str) -> Result<MaskedClaimsSeat, String> {
    match value {
        "seat-0" => Ok(MaskedClaimsSeat::Seat0),
        "seat-1" => Ok(MaskedClaimsSeat::Seat1),
        _ => MaskedClaimsSeat::parse(value).ok_or_else(|| {
            format!(
                "{{\"code\":\"unknown_seat\",\"message\":\"unknown seat: {}\"}}",
                escape_json(value)
            )
        }),
    }
}

pub(crate) fn trace_masked_seat(seat: MaskedClaimsSeat) -> &'static str {
    seat.as_str()
}

pub(crate) fn parse_flood_seat(value: &str) -> Result<SeatId, String> {
    match value {
        "seat-0" | "seat_0" => Ok(SeatId("seat_0".to_owned())),
        "seat-1" | "seat_1" => Ok(SeatId("seat_1".to_owned())),
        _ => Err(format!(
            "{{\"code\":\"unknown_seat\",\"message\":\"unknown seat: {}\"}}",
            escape_json(value)
        )),
    }
}

pub(crate) fn parse_frontier_seat(value: &str) -> Result<SeatId, String> {
    match value {
        "seat-0" | "seat_0" => Ok(SeatId("seat_0".to_owned())),
        "seat-1" | "seat_1" => Ok(SeatId("seat_1".to_owned())),
        _ => Err(format!(
            "{{\"code\":\"unknown_seat\",\"message\":\"unknown seat: {}\"}}",
            escape_json(value)
        )),
    }
}

pub(crate) fn parse_event_frontier_seat(value: &str) -> Result<SeatId, String> {
    match value {
        "seat-0" | "seat_0" => Ok(SeatId("seat_0".to_owned())),
        "seat-1" | "seat_1" => Ok(SeatId("seat_1".to_owned())),
        _ => Err(format!(
            "{{\"code\":\"unknown_seat\",\"message\":\"unknown seat: {}\"}}",
            escape_json(value)
        )),
    }
}

pub(crate) fn parse_token_seat(value: &str) -> Result<TokenBazaarSeat, String> {
    match value {
        "seat-0" => Ok(TokenBazaarSeat::Seat0),
        "seat-1" => Ok(TokenBazaarSeat::Seat1),
        _ => TokenBazaarSeat::parse(value).ok_or_else(|| {
            format!(
                "{{\"code\":\"unknown_seat\",\"message\":\"unknown seat: {}\"}}",
                escape_json(value)
            )
        }),
    }
}

pub(crate) fn trace_token_seat(seat: TokenBazaarSeat) -> &'static str {
    match seat {
        TokenBazaarSeat::Seat0 => "seat_0",
        TokenBazaarSeat::Seat1 => "seat_1",
    }
}

pub(crate) fn parse_secret_seat(value: &str) -> Result<SecretDraftSeat, String> {
    match value {
        "seat-0" => Ok(SecretDraftSeat::Seat0),
        "seat-1" => Ok(SecretDraftSeat::Seat1),
        _ => SecretDraftSeat::parse(value).ok_or_else(|| {
            format!(
                "{{\"code\":\"unknown_seat\",\"message\":\"unknown seat: {}\"}}",
                escape_json(value)
            )
        }),
    }
}

pub(crate) fn trace_secret_seat(seat: SecretDraftSeat) -> &'static str {
    seat.as_str()
}

pub(crate) fn parse_poker_seat(value: &str) -> Result<PokerLiteSeat, String> {
    match value {
        "seat-0" => Ok(PokerLiteSeat::Seat0),
        "seat-1" => Ok(PokerLiteSeat::Seat1),
        _ => PokerLiteSeat::parse(value).ok_or_else(|| {
            format!(
                "{{\"code\":\"unknown_seat\",\"message\":\"unknown seat: {}\"}}",
                escape_json(value)
            )
        }),
    }
}

pub(crate) fn trace_poker_seat(seat: PokerLiteSeat) -> &'static str {
    seat.as_str()
}

pub(crate) fn parse_plain_seat(value: &str) -> Result<PlainTricksSeat, String> {
    match value {
        "seat-0" => Ok(PlainTricksSeat::Seat0),
        "seat-1" => Ok(PlainTricksSeat::Seat1),
        _ => PlainTricksSeat::parse(value).ok_or_else(|| {
            format!(
                "{{\"code\":\"unknown_seat\",\"message\":\"unknown seat: {}\"}}",
                escape_json(value)
            )
        }),
    }
}

pub(crate) fn trace_plain_seat(seat: PlainTricksSeat) -> &'static str {
    seat.as_str()
}

pub(crate) fn parse_river_seat(value: &str) -> Result<RiverLedgerSeat, String> {
    RiverLedgerSeat::parse(value).ok_or_else(|| {
        format!(
            "{{\"code\":\"unknown_seat\",\"message\":\"unknown seat: {}\"}}",
            escape_json(value)
        )
    })
}

pub(crate) fn trace_river_seat(seat: RiverLedgerSeat) -> String {
    seat.as_str()
}

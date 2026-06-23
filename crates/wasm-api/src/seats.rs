//! Seat-id parsing and trace-label helpers for every registered game.
//!
//! Translates browser-supplied seat strings (`"seat_0"`, ...) into each game's
//! typed seat enum (or an `unknown_seat` diagnostic), and renders the canonical
//! trace seat labels used in replay documents. Glob-imported at the crate root.

use engine_core::SeatId;

use crate::constants::DEFAULT_SEAT_COUNT;
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

const TWO_SEAT_SYMBOLIC_ALIASES: &[(&str, u32)] = &[("seat-a", 0), ("seat-b", 1)];
const SIX_SEAT_SYMBOLIC_ALIASES: &[(&str, u32)] = &[
    ("seat-a", 0),
    ("seat-b", 1),
    ("seat-c", 2),
    ("seat-d", 3),
    ("seat-e", 4),
    ("seat-f", 5),
];

fn unknown_seat(value: &str) -> String {
    format!(
        "{{\"code\":\"unknown_seat\",\"message\":\"unknown seat: {}\"}}",
        escape_json(value)
    )
}

fn parse_seat_import(
    value: &str,
    seat_count: u32,
    symbolic_aliases: &[(&str, u32)],
) -> Result<SeatId, String> {
    if let Ok(seat_id) = SeatId::parse_canonical(value) {
        return bounded_canonical_seat(value, seat_id, seat_count);
    }

    if let Some(suffix) = value.strip_prefix("seat-") {
        if let Ok(seat_id) = SeatId::parse_canonical(&format!("seat_{suffix}")) {
            return bounded_canonical_seat(value, seat_id, seat_count);
        }
    }

    let mut matches = symbolic_aliases
        .iter()
        .filter(|(alias, _)| *alias == value)
        .map(|(_, index)| *index);
    match (matches.next(), matches.next()) {
        (Some(index), None) if index < seat_count => Ok(SeatId::from_zero_based_index(index)),
        _ => Err(unknown_seat(value)),
    }
}

fn bounded_canonical_seat(value: &str, seat_id: SeatId, seat_count: u32) -> Result<SeatId, String> {
    let index = seat_id
        .canonical_zero_based_index()
        .map_err(|_| unknown_seat(value))?;
    if index < seat_count {
        Ok(seat_id)
    } else {
        Err(unknown_seat(value))
    }
}

fn parse_seat_enum<T>(
    value: &str,
    seat_count: u32,
    symbolic_aliases: &[(&str, u32)],
    parse: impl FnOnce(&str) -> Option<T>,
) -> Result<T, String> {
    let seat_id = parse_seat_import(value, seat_count, symbolic_aliases)?;
    parse(&seat_id.0).ok_or_else(|| unknown_seat(value))
}

pub(crate) fn parse_race_seat(value: &str) -> Result<RaceSeat, String> {
    parse_seat_enum(value, 2, TWO_SEAT_SYMBOLIC_ALIASES, RaceSeat::parse)
}

pub(crate) fn parse_replay_race_seat(value: &str) -> Result<RaceSeat, String> {
    parse_race_seat(value)
}

pub(crate) fn trace_race_seat(seat: RaceSeat) -> &'static str {
    match seat {
        RaceSeat::Seat0 => "seat-0",
        RaceSeat::Seat1 => "seat-1",
    }
}

pub(crate) fn parse_three_seat(value: &str) -> Result<ThreeMarksSeat, String> {
    parse_seat_enum(value, 2, TWO_SEAT_SYMBOLIC_ALIASES, ThreeMarksSeat::parse)
}

pub(crate) fn trace_three_seat(seat: ThreeMarksSeat) -> &'static str {
    match seat {
        ThreeMarksSeat::Seat0 => "seat-0",
        ThreeMarksSeat::Seat1 => "seat-1",
    }
}

pub(crate) fn parse_column_seat(value: &str) -> Result<ColumnFourSeat, String> {
    parse_seat_enum(value, 2, TWO_SEAT_SYMBOLIC_ALIASES, ColumnFourSeat::parse)
}

pub(crate) fn trace_column_seat(seat: ColumnFourSeat) -> &'static str {
    match seat {
        ColumnFourSeat::Seat0 => "seat-0",
        ColumnFourSeat::Seat1 => "seat-1",
    }
}

pub(crate) fn parse_directional_seat(value: &str) -> Result<DirectionalFlipSeat, String> {
    parse_seat_enum(
        value,
        2,
        TWO_SEAT_SYMBOLIC_ALIASES,
        DirectionalFlipSeat::parse,
    )
}

pub(crate) fn trace_directional_seat(seat: DirectionalFlipSeat) -> &'static str {
    match seat {
        DirectionalFlipSeat::Seat0 => "seat-0",
        DirectionalFlipSeat::Seat1 => "seat-1",
    }
}

pub(crate) fn parse_draughts_seat(value: &str) -> Result<DraughtsLiteSeat, String> {
    parse_seat_enum(value, 2, TWO_SEAT_SYMBOLIC_ALIASES, DraughtsLiteSeat::parse)
}

pub(crate) fn trace_draughts_seat(seat: DraughtsLiteSeat) -> &'static str {
    match seat {
        DraughtsLiteSeat::Seat0 => "seat-0",
        DraughtsLiteSeat::Seat1 => "seat-1",
    }
}

pub(crate) fn parse_high_card_seat(value: &str) -> Result<HighCardDuelSeat, String> {
    parse_seat_enum(value, 2, TWO_SEAT_SYMBOLIC_ALIASES, HighCardDuelSeat::parse)
}

pub(crate) fn trace_high_card_seat(seat: HighCardDuelSeat) -> &'static str {
    match seat {
        HighCardDuelSeat::Seat0 => "seat-0",
        HighCardDuelSeat::Seat1 => "seat-1",
    }
}

pub(crate) fn parse_masked_seat(value: &str) -> Result<MaskedClaimsSeat, String> {
    parse_seat_enum(value, 2, TWO_SEAT_SYMBOLIC_ALIASES, MaskedClaimsSeat::parse)
}

pub(crate) fn trace_masked_seat(seat: MaskedClaimsSeat) -> &'static str {
    seat.as_str()
}

pub(crate) fn parse_flood_seat(value: &str) -> Result<SeatId, String> {
    parse_seat_import(value, 2, TWO_SEAT_SYMBOLIC_ALIASES)
}

pub(crate) fn parse_frontier_seat(value: &str) -> Result<SeatId, String> {
    parse_seat_import(value, 2, TWO_SEAT_SYMBOLIC_ALIASES)
}

pub(crate) fn parse_event_frontier_seat(value: &str) -> Result<SeatId, String> {
    parse_seat_import(value, 2, TWO_SEAT_SYMBOLIC_ALIASES)
}

pub(crate) fn parse_token_seat(value: &str) -> Result<TokenBazaarSeat, String> {
    parse_seat_enum(value, 2, TWO_SEAT_SYMBOLIC_ALIASES, TokenBazaarSeat::parse)
}

pub(crate) fn trace_token_seat(seat: TokenBazaarSeat) -> &'static str {
    match seat {
        TokenBazaarSeat::Seat0 => "seat_0",
        TokenBazaarSeat::Seat1 => "seat_1",
    }
}

pub(crate) fn parse_secret_seat(value: &str) -> Result<SecretDraftSeat, String> {
    parse_seat_enum(value, 2, TWO_SEAT_SYMBOLIC_ALIASES, SecretDraftSeat::parse)
}

pub(crate) fn trace_secret_seat(seat: SecretDraftSeat) -> &'static str {
    seat.as_str()
}

pub(crate) fn parse_poker_seat(value: &str) -> Result<PokerLiteSeat, String> {
    parse_seat_enum(value, 2, TWO_SEAT_SYMBOLIC_ALIASES, PokerLiteSeat::parse)
}

pub(crate) fn trace_poker_seat(seat: PokerLiteSeat) -> &'static str {
    seat.as_str()
}

pub(crate) fn parse_plain_seat(value: &str) -> Result<PlainTricksSeat, String> {
    parse_seat_enum(value, 2, TWO_SEAT_SYMBOLIC_ALIASES, PlainTricksSeat::parse)
}

pub(crate) fn trace_plain_seat(seat: PlainTricksSeat) -> &'static str {
    seat.as_str()
}

pub(crate) fn parse_river_seat(value: &str) -> Result<RiverLedgerSeat, String> {
    parse_seat_enum(value, 6, SIX_SEAT_SYMBOLIC_ALIASES, RiverLedgerSeat::parse)
}

pub(crate) fn trace_river_seat(seat: RiverLedgerSeat) -> String {
    seat.as_str()
}

pub(crate) fn canonical_trace_seat_id(index: u32) -> String {
    SeatId::from_zero_based_index(index).0
}

// Seat-roster builders: construct the ordered SeatId list for a match of a
// given size. Hyphen vs. underscore seat-id spelling is per game.
pub(crate) fn seats() -> Vec<SeatId> {
    seats_for_count(DEFAULT_SEAT_COUNT)
}

pub(crate) fn seats_for_count(seat_count: usize) -> Vec<SeatId> {
    (0..seat_count)
        .map(|index| SeatId(format!("seat-{index}")))
        .collect()
}

pub(crate) fn canonical_seats_for_count(seat_count: usize) -> Vec<SeatId> {
    (0..seat_count)
        .map(|index| SeatId::from_zero_based_index(index as u32))
        .collect()
}

pub(crate) fn plain_seats() -> Vec<SeatId> {
    plain_seats_for_count(DEFAULT_SEAT_COUNT)
}

pub(crate) fn plain_seats_for_count(seat_count: usize) -> Vec<SeatId> {
    underscore_seats_for_count(seat_count)
}

pub(crate) fn river_seats_for_count(seat_count: usize) -> Vec<SeatId> {
    underscore_seats_for_count(seat_count)
}

pub(crate) fn masked_seats() -> Vec<SeatId> {
    masked_seats_for_count(DEFAULT_SEAT_COUNT)
}

pub(crate) fn masked_seats_for_count(seat_count: usize) -> Vec<SeatId> {
    underscore_seats_for_count(seat_count)
}

pub(crate) fn flood_seats() -> Vec<SeatId> {
    flood_seats_for_count(DEFAULT_SEAT_COUNT)
}

pub(crate) fn flood_seats_for_count(seat_count: usize) -> Vec<SeatId> {
    underscore_seats_for_count(seat_count)
}

pub(crate) fn frontier_seats() -> Vec<SeatId> {
    frontier_seats_for_count(DEFAULT_SEAT_COUNT)
}

pub(crate) fn frontier_seats_for_count(seat_count: usize) -> Vec<SeatId> {
    underscore_seats_for_count(seat_count)
}

pub(crate) fn event_frontier_seats() -> Vec<SeatId> {
    event_frontier_seats_for_count(DEFAULT_SEAT_COUNT)
}

pub(crate) fn event_frontier_seats_for_count(seat_count: usize) -> Vec<SeatId> {
    underscore_seats_for_count(seat_count)
}

fn underscore_seats_for_count(seat_count: usize) -> Vec<SeatId> {
    (0..seat_count)
        .map(|index| SeatId(format!("seat_{index}")))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn canonical_output_helpers_emit_underscore_seat_ids() {
        assert_eq!(canonical_trace_seat_id(0), "seat_0");
        assert_eq!(canonical_trace_seat_id(12), "seat_12");
        assert_eq!(
            canonical_seats_for_count(4),
            vec![
                SeatId("seat_0".to_owned()),
                SeatId("seat_1".to_owned()),
                SeatId("seat_2".to_owned()),
                SeatId("seat_3".to_owned()),
            ]
        );
    }

    #[test]
    fn existing_trace_and_roster_helpers_keep_legacy_outputs() {
        assert_eq!(trace_race_seat(RaceSeat::Seat0), "seat-0");
        assert_eq!(trace_three_seat(ThreeMarksSeat::Seat1), "seat-1");
        assert_eq!(trace_column_seat(ColumnFourSeat::Seat0), "seat-0");
        assert_eq!(trace_directional_seat(DirectionalFlipSeat::Seat1), "seat-1");
        assert_eq!(trace_draughts_seat(DraughtsLiteSeat::Seat0), "seat-0");
        assert_eq!(trace_token_seat(TokenBazaarSeat::Seat1), "seat_1");
        assert_eq!(
            seats_for_count(3),
            vec![
                SeatId("seat-0".to_owned()),
                SeatId("seat-1".to_owned()),
                SeatId("seat-2".to_owned()),
            ]
        );
        assert_eq!(
            plain_seats_for_count(2),
            vec![SeatId("seat_0".to_owned()), SeatId("seat_1".to_owned())]
        );
    }

    #[test]
    fn import_adapter_accepts_canonical_hyphen_and_symbolic_aliases() {
        assert_eq!(parse_race_seat("seat_0"), Ok(RaceSeat::Seat0));
        assert_eq!(parse_race_seat("seat-1"), Ok(RaceSeat::Seat1));
        assert_eq!(parse_race_seat("seat-a"), Ok(RaceSeat::Seat0));
        assert_eq!(parse_race_seat("seat-b"), Ok(RaceSeat::Seat1));

        assert_eq!(parse_draughts_seat("seat_0"), Ok(DraughtsLiteSeat::Seat0));
        assert_eq!(parse_draughts_seat("seat-1"), Ok(DraughtsLiteSeat::Seat1));
        assert_eq!(parse_draughts_seat("seat-a"), Ok(DraughtsLiteSeat::Seat0));

        assert_eq!(parse_high_card_seat("seat-0"), Ok(HighCardDuelSeat::Seat0));
        assert_eq!(parse_high_card_seat("seat-b"), Ok(HighCardDuelSeat::Seat1));

        assert_eq!(parse_flood_seat("seat-a"), Ok(SeatId("seat_0".to_owned())));
        assert_eq!(
            parse_frontier_seat("seat-1"),
            Ok(SeatId("seat_1".to_owned()))
        );
        assert_eq!(
            parse_event_frontier_seat("seat_1"),
            Ok(SeatId("seat_1".to_owned()))
        );

        assert_eq!(
            parse_river_seat("seat_5"),
            RiverLedgerSeat::from_index(5).ok_or_else(|| unknown_seat("seat_5"))
        );
        assert_eq!(
            parse_river_seat("seat-f"),
            RiverLedgerSeat::from_index(5).ok_or_else(|| unknown_seat("seat-f"))
        );
    }

    #[test]
    fn import_adapter_rejects_unknown_out_of_range_and_ambiguous_labels() {
        assert_eq!(parse_race_seat("seat_2"), Err(unknown_seat("seat_2")));
        assert_eq!(parse_race_seat("seat-2"), Err(unknown_seat("seat-2")));
        assert_eq!(parse_race_seat("seat-c"), Err(unknown_seat("seat-c")));
        assert_eq!(parse_race_seat("seat_01"), Err(unknown_seat("seat_01")));
        assert_eq!(parse_race_seat("seat_１"), Err(unknown_seat("seat_１")));
        assert_eq!(
            parse_seat_import("seat-a", 2, &[("seat-a", 0), ("seat-a", 1)]),
            Err(unknown_seat("seat-a"))
        );
        assert_eq!(
            parse_seat_import("seat-c", 2, &[("seat-c", 2)]),
            Err(unknown_seat("seat-c"))
        );
    }
}

//! Starbridge Crossing game crate.
//!
//! This crate keeps all Star Halma-family topology, peg, path, home/target,
//! and jump-chain nouns local to the game module. The shared engine sees only
//! generic Rulepath contracts.

pub mod ids;
pub mod topology;

pub use ids::{
    active_points_for_seat_count, canonical_seat_ids, seat_id_for_index, supported_seat_count,
    StarPoint, StarSpaceId, StarSpaceIdError, StarZone, DATA_VERSION_LABEL, GAME_ID,
    MAX_SPACE_INDEX, RULES_VERSION_LABEL, SPACE_COUNT, STANDARD_DEFAULT_SEATS, STANDARD_MAX_SEATS,
    STANDARD_MIN_SEATS, STANDARD_PEGS_PER_SEAT, SUPPORTED_SEAT_COUNTS, VARIANT_ID,
};
pub use topology::{
    coordinate_for_id, home_spaces, load_manifest, neighbor_in_direction, space_for_id, spaces,
    spaces_by_stable_order, Manifest, StarCoord, StarDirection, StarSpace, StarUiAnchor,
    TOPOLOGY_GENERATOR,
};

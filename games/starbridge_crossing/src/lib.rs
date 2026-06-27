//! Starbridge Crossing game crate.
//!
//! This crate keeps all Star Halma-family topology, peg, path, home/target,
//! and jump-chain nouns local to the game module. The shared engine sees only
//! generic Rulepath contracts.

pub mod actions;
pub mod bots;
pub mod effects;
pub mod ids;
pub mod replay_support;
pub mod rules;
pub mod setup;
pub mod state;
pub mod topology;
pub mod ui;
pub mod variants;
pub mod visibility;

pub use actions::{
    encode_jump_path, encode_step_path, legal_action_tree, parse_action_path, StarbridgeAction,
};
pub use bots::{
    legal_action_paths, parse_bot_action, StarbridgeCrossingL0Bot, StarbridgeL0Decision,
    L0_POLICY_ID,
};
pub use effects::{public_effect, JumpSubstep, StarbridgeEffect, StarbridgeEffectEnvelope};
pub use ids::{
    active_points_for_seat_count, canonical_seat_ids, seat_id_for_index, supported_seat_count,
    StarPoint, StarSpaceId, StarSpaceIdError, StarZone, DATA_VERSION_LABEL, GAME_ID,
    MAX_SPACE_INDEX, RULES_VERSION_LABEL, SPACE_COUNT, STANDARD_DEFAULT_SEATS, STANDARD_MAX_SEATS,
    STANDARD_MIN_SEATS, STANDARD_PEGS_PER_SEAT, SUPPORTED_SEAT_COUNTS, VARIANT_ID,
};
pub use replay_support::{replay_commands, ReplayHashes};
pub use rules::{
    apply_jump_command, apply_pass_blocked_command, apply_step_command, is_active_seat_blocked,
    legal_jump_landings, legal_step_moves, validate_jump_command, validate_pass_blocked_command,
    validate_step_command, JumpChain, JumpLanding, StepMove,
};
pub use setup::{setup_match, SetupOptions};
pub use state::{
    FinishRank, SeatAssignment, StarPeg, StarPegId, StarbridgeSnapshot, StarbridgeState,
    TerminalStatus,
};
pub use topology::{
    coordinate_for_id, home_spaces, load_manifest, neighbor_in_direction, space_for_id, spaces,
    spaces_by_stable_order, Manifest, StarCoord, StarDirection, StarSpace, StarUiAnchor,
    TOPOLOGY_GENERATOR,
};
pub use ui::{space_label, zone_label, SpaceUiMetadata};
pub use variants::{load_variants, Variant, VariantCatalog};
pub use visibility::{
    filter_effects_for_viewer, project_view, AllPublicAudit, PegView, SeatView, SpaceView,
    StarbridgePublicView,
};
